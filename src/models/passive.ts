import {
  ClassSkillLine,
  DamageType,
  EsoClass,
  WeaponSkillLineName,
} from './skill';

export type PassiveBonusType =
  | 'damage' // General damage %
  | 'damageType' // Specific type (flame, poison)
  | 'dot' // DoT damage %
  | 'direct' // Direct damage %
  | 'statusEffect' // Burning/Poisoned effect damage
  | 'skillLine'; // Affects all skills in the passive's skill line

export type StatusEffect = 'Burning' | 'Poisoned' | 'Chilled' | 'Concussed';

export interface PassiveBonus {
  type: PassiveBonusType;
  value: number; // Decimal (0.05 = 5%)
  damageTypes?: DamageType[]; // For damageType bonuses
  statusEffects?: StatusEffect[]; // For statusEffect bonuses
}

export interface ClassPassiveSkill {
  name: string;
  skillLine: ClassSkillLine;
  esoClass: EsoClass;
  bonuses: PassiveBonus[];
}

export interface WeaponPassiveSkill {
  name: string;
  skillLine: WeaponSkillLineName;
  bonuses: PassiveBonus[];
}

export type AnyPassiveSkill = ClassPassiveSkill | WeaponPassiveSkill;
