import { BonusData } from '../data/bonuses/types';
import { PassiveData } from '../data/passives/types';
import { ClassSkillLineName, WeaponSkillLineName } from '../data/skills';
import { ClassName } from '../data/types';
import { table } from '../infrastructure';
import {
  getClassPassivesBySkillLine,
  getWeaponPassivesBySkillLine,
} from '../services/passive-service';
import {
  calculatePassiveBonus,
  SkillsService,
} from '../services/skills-service';
import { Skill } from './skill';

const BUILD_CONSTRAINTS = {
  skillCount: 10,
  championPointCount: 4,
  classSkillLineCount: 3,
  weaponSkillLineCount: 2,
};

class Build {
  readonly skills: readonly Skill[];
  readonly championPoints: readonly BonusData[];

  readonly modifiers: readonly BonusData[];
  readonly usedClassSkillLines: readonly ClassSkillLineName[];
  readonly usedWeaponSkillLines: readonly WeaponSkillLineName[];
  readonly requiredClass?: ClassName;
  readonly passives: readonly PassiveData[];

  private readonly _skillDamages: Map<string, number>;
  private readonly _totalDamage: number;

  constructor(skills: Skill[], championPoints: BonusData[]) {
    this.skills = Object.freeze([...skills]);
    this.championPoints = Object.freeze([...championPoints]);

    // Derive skill lines from skills
    this.usedClassSkillLines = Object.freeze([
      ...new Set(
        skills
          .filter((s) => s.skillType === 'class')
          .map((s) => s.skillLine as ClassSkillLineName),
      ),
    ]);
    this.usedWeaponSkillLines = Object.freeze([
      ...new Set(
        skills
          .filter((s) => s.skillType === 'weapon')
          .map((s) => s.skillLine as WeaponSkillLineName),
      ),
    ]);

    // championPoints ARE the modifiers
    this.modifiers = this.championPoints;

    // Derive required class from class skill lines
    this.requiredClass = this.deriveRequiredClass();

    // Get passives for skill lines
    this.passives = Object.freeze(this.getPassivesForSkillLines());

    // Calculate and cache damages at construction time
    const { skillDamages, totalDamage } = this.calculateDamages();
    this._skillDamages = skillDamages;
    this._totalDamage = totalDamage;
  }

  private deriveRequiredClass(): ClassName | undefined {
    // Get unique class names from class skill lines
    const classNames = new Set<ClassName>();
    for (const line of this.usedClassSkillLines) {
      classNames.add(SkillsService.getClass(line));
    }
    // If all class skill lines belong to the same class, that's the required class
    if (classNames.size === 1) {
      return [...classNames][0];
    }
    return undefined;
  }

  private getPassivesForSkillLines(): PassiveData[] {
    const passives: PassiveData[] = [];
    for (const line of this.usedClassSkillLines) {
      passives.push(...getClassPassivesBySkillLine(line));
    }
    for (const line of this.usedWeaponSkillLines) {
      passives.push(...getWeaponPassivesBySkillLine(line));
    }
    return passives;
  }

  /** Sum of all skill damages */
  get totalDamagePerCast(): number {
    return this._totalDamage;
  }

  /** Get damage for a specific skill by name */
  getSkillDamage(skillName: string): number {
    return this._skillDamages.get(skillName) ?? 0;
  }

  /** Validate build against constraints */
  isValid(): boolean {
    return (
      this.skills.length <= BUILD_CONSTRAINTS.skillCount &&
      this.modifiers.length <= BUILD_CONSTRAINTS.championPointCount &&
      this.usedClassSkillLines.length <=
        BUILD_CONSTRAINTS.classSkillLineCount &&
      this.usedWeaponSkillLines.length <= BUILD_CONSTRAINTS.weaponSkillLineCount
    );
  }

  isBetterThan(other: Build | null): boolean {
    if (other === null) {
      return true;
    }
    return this.totalDamagePerCast > other.totalDamagePerCast;
  }

  toString(): string {
    const lines: string[] = [];

    // Header section
    const divider = '─'.repeat(73);
    lines.push('');
    lines.push('Optimal Build - Maximum Damage Per Cast');
    lines.push(divider);
    lines.push(
      `Total Damage: ${this.totalDamagePerCast.toLocaleString('en-US', { maximumFractionDigits: 0 })}`,
    );
    lines.push('');
    lines.push(`Modifiers: ${this.modifiers.map((m) => m.name).join(', ')}`);

    // Skills table
    const skillsData = this.skills.map((skill, i) => {
      return [
        (i + 1).toString(),
        skill.name,
        skill.className,
        skill.skillLine,
        this.getSkillDamage(skill.name).toFixed(0),
      ];
    });

    const classLines = this.usedClassSkillLines.join(', ');
    const weaponLines = this.usedWeaponSkillLines.join(', ');
    const classCount = this.usedClassSkillLines.length;
    const weaponCount = this.usedWeaponSkillLines.length;

    let skillsFooter = `Skill Lines: ${classLines} (${classCount}/3 class), ${weaponLines} (${weaponCount}/2 weapon)`;
    if (this.requiredClass) {
      skillsFooter += `\nRequired Class: ${this.requiredClass}`;
    }

    lines.push(
      table(skillsData, {
        title: 'Skills',
        columns: [
          { header: '#', width: 4, align: 'right' },
          { header: 'Name', width: 25 },
          { header: 'Source', width: 12 },
          { header: 'Skill Line', width: 18 },
          { header: 'Damage', width: 10, align: 'right' },
        ],
        footer: skillsFooter,
      }),
    );

    // Passives table
    if (this.passives.length > 0) {
      const passivesData = this.passives.map((passive) => {
        return [passive.name, passive.className, passive.skillLine];
      });

      lines.push(
        table(passivesData, {
          title: 'Passives',
          columns: [
            { header: 'Name', width: 30 },
            { header: 'Source', width: 15 },
            { header: 'Skill Line', width: 20 },
          ],
        }),
      );
    }

    return lines.join('\n');
  }

  /** Count skills per skill line */
  private getSkillLineCounts(): Record<string, number> {
    const counts: Record<string, number> = {};
    for (const skill of this.skills) {
      counts[skill.skillLine] = (counts[skill.skillLine] ?? 0) + 1;
    }
    return counts;
  }

  /** Calculate damage for all skills with passives applied */
  private calculateDamages(): {
    skillDamages: Map<string, number>;
    totalDamage: number;
  } {
    const skillDamages = new Map<string, number>();
    const skillLineCounts = this.getSkillLineCounts();
    let totalDamage = 0;

    for (const skill of this.skills) {
      const baseDamage = skill.calculateDamagePerCast(this.modifiers);
      const skillLineCount = skillLineCounts[skill.skillLine] ?? 0;
      const passiveBonus = calculatePassiveBonus(this.passives, skillLineCount);
      const damage = baseDamage * (1 + passiveBonus);

      skillDamages.set(skill.name, damage);
      totalDamage += damage;
    }

    return { skillDamages, totalDamage };
  }
}

export { Build, BUILD_CONSTRAINTS };
