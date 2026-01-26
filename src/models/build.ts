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

export interface Build {
  skills: BuildSkill[];
  passives: BuildPassive[];
  modifiers: string[];
  totalDamagePerCast: number;
  usedClassSkillLines: string[];
  usedWeaponSkillLines: string[];
  requiredClass?: string;
}

export interface BuildConstraints {
  maxSkills: number;
  maxModifiers: number;
  maxClassSkillLines: number;
  maxWeaponSkillLines: number;
}
