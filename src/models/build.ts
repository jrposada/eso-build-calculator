export interface BuildSkill {
  name: string;
  skillLine: string;
  source: string;
  damagePerCast: number;
}

export interface BuildPassive {
  name: string;
  skillLine: string;
  source: string; // Class name or 'Weapon'
}

export interface BuildConstraints {
  maxSkills: number;
  maxModifiers: number;
  maxClassSkillLines: number;
  maxWeaponSkillLines: number;
}

export class Build {
  readonly skills: readonly BuildSkill[];
  readonly passives: readonly BuildPassive[];
  readonly modifiers: readonly string[];
  readonly usedClassSkillLines: readonly string[];
  readonly usedWeaponSkillLines: readonly string[];
  readonly requiredClass?: string;

  constructor(
    skills: BuildSkill[],
    passives: BuildPassive[],
    modifiers: string[],
    usedClassSkillLines: string[],
    usedWeaponSkillLines: string[],
    requiredClass?: string,
  ) {
    this.skills = Object.freeze([...skills]);
    this.passives = Object.freeze([...passives]);
    this.modifiers = Object.freeze([...modifiers]);
    this.usedClassSkillLines = Object.freeze([...usedClassSkillLines]);
    this.usedWeaponSkillLines = Object.freeze([...usedWeaponSkillLines]);
    this.requiredClass = requiredClass;
  }

  /** Sum of all skill damages */
  get totalDamagePerCast(): number {
    return this.skills.reduce((sum, skill) => sum + skill.damagePerCast, 0);
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

  /** Serialize to plain object for JSON output */
  toJSON(): {
    skills: BuildSkill[];
    passives: BuildPassive[];
    modifiers: string[];
    totalDamagePerCast: number;
    usedClassSkillLines: string[];
    usedWeaponSkillLines: string[];
    requiredClass?: string;
  } {
    return {
      skills: [...this.skills],
      passives: [...this.passives],
      modifiers: [...this.modifiers],
      totalDamagePerCast: this.totalDamagePerCast,
      usedClassSkillLines: [...this.usedClassSkillLines],
      usedWeaponSkillLines: [...this.usedWeaponSkillLines],
      requiredClass: this.requiredClass,
    };
  }
}
