import {
  AnySkill,
  calculateDamagePerCast,
  calculatePassiveBonus,
  SkillLineCounts,
} from '../services/skill-service';
import { DamageModifier } from './modifier';
import { AnyPassiveSkill } from './passive';
import { ClassSkillLine, EsoClass, WeaponSkillLineName } from './skill';

export interface BuildConstraints {
  maxSkills: number;
  maxModifiers: number;
  maxClassSkillLines: number;
  maxWeaponSkillLines: number;
}

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
  isValid(constraints: BuildConstraints): boolean {
    return (
      this.skills.length <= constraints.maxSkills &&
      this.modifiers.length <= constraints.maxModifiers &&
      this.usedClassSkillLines.length <= constraints.maxClassSkillLines &&
      this.usedWeaponSkillLines.length <= constraints.maxWeaponSkillLines
    );
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
      const baseDamage = calculateDamagePerCast(
        skill,
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
