import { ClassSkillLineName, WeaponSkillLineName } from '../data/skills';
import { DamageType, SkillClassName } from '../data/skills/types';

export type PassiveBonusType =
  | 'critical-chance'
  | 'critical-damage'
  | 'duration'
  | 'max-stamina'
  | 'max-magicka';

export type StatusEffect = 'Burning' | 'Poisoned' | 'Chilled' | 'Concussed';

export type PassiveBonus = {
  /** How many times does the passive apply */
  multiplier:
    | 'skillLine' // Skill line is part of build
    | 'abilitySlotted' // At least 1 skill from skill line is equipped on build
    | 'abilitySlottedCount'; // Applied once per skill of related skill line equipped on build
} & (
  | {
      type: PassiveBonusType;
      //** Decimal (0.05 = 5%) or seconds */
      value: number;
      damageTypes?: DamageType[];
      statusEffects?: StatusEffect[];
    }
  | {
      buffId: 'Minor Savagery';
    }
);

export interface ClassPassiveSkill {
  name: string;
  skillLine: ClassSkillLineName;
  esoClass: SkillClassName;
  bonuses: PassiveBonus[];
}

export interface WeaponPassiveSkill {
  name: string;
  skillLine: WeaponSkillLineName;
  bonuses: PassiveBonus[];
}

export type AnyPassiveSkill = ClassPassiveSkill | WeaponPassiveSkill;
