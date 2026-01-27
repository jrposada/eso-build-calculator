import { table } from '../infrastructure';
import {
  AnySkill,
  calculatePassiveBonus,
  SkillLineCounts,
} from '../services/skill-service';
import { DamageModifier } from './modifier';
import { AnyPassiveSkill } from './passive';
import { ClassSkillLine, EsoClass, Skill, WeaponSkillLineName } from './skill';

export const BUILD_CONSTRAINTS = {
  maxSkills: 10,
  maxModifiers: 4,
  maxClassSkillLines: 3,
  maxWeaponSkillLines: 2,
};

export class Build {
  readonly skills: readonly AnySkill[];
  readonly passives: readonly AnyPassiveSkill[];
  readonly modifiers: readonly DamageModifier[];
  readonly usedClassSkillLines: readonly ClassSkillLine[];
  readonly usedWeaponSkillLines: readonly WeaponSkillLineName[];
  readonly requiredClass?: EsoClass;

  private readonly _skillDamages: Map<string, number>;
  private readonly _totalDamage: number;

  constructor(
    skills: AnySkill[],
    passives: AnyPassiveSkill[],
    modifiers: DamageModifier[],
    usedClassSkillLines: ClassSkillLine[],
    usedWeaponSkillLines: WeaponSkillLineName[],
    requiredClass?: EsoClass,
  ) {
    this.skills = Object.freeze([...skills]);
    this.passives = Object.freeze([...passives]);
    this.modifiers = Object.freeze([...modifiers]);
    this.usedClassSkillLines = Object.freeze([...usedClassSkillLines]);
    this.usedWeaponSkillLines = Object.freeze([...usedWeaponSkillLines]);
    this.requiredClass = requiredClass;

    // Calculate and cache damages at construction time
    const { skillDamages, totalDamage } = this.calculateDamages();
    this._skillDamages = skillDamages;
    this._totalDamage = totalDamage;
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
      this.skills.length <= BUILD_CONSTRAINTS.maxSkills &&
      this.modifiers.length <= BUILD_CONSTRAINTS.maxModifiers &&
      this.usedClassSkillLines.length <= BUILD_CONSTRAINTS.maxClassSkillLines &&
      this.usedWeaponSkillLines.length <= BUILD_CONSTRAINTS.maxWeaponSkillLines
    );
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
      const source = 'esoClass' in skill ? skill.esoClass : 'Weapon';
      return [
        (i + 1).toString(),
        skill.name,
        source,
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
        const source = 'esoClass' in passive ? passive.esoClass : 'Weapon';
        return [passive.name, source, passive.skillLine];
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
  private getSkillLineCounts(): SkillLineCounts {
    const counts: SkillLineCounts = {};
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
      const skillInstance = Skill.fromData(skill);
      const baseDamage = skillInstance.calculateDamagePerCast(
        this.modifiers as DamageModifier[],
      );
      const passiveBonus = calculatePassiveBonus(
        skill,
        this.passives as AnyPassiveSkill[],
        skillLineCounts,
      );
      const damage = baseDamage * (1 + passiveBonus);

      skillDamages.set(skill.name, damage);
      totalDamage += damage;
    }

    return { skillDamages, totalDamage };
  }
}
