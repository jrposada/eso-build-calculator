export type {
  ClassPassiveSkillLineName,
  WeaponPassiveSkillLineName,
} from '../data/passives';
export type {
  PassiveBonus,
  PassiveBonusType,
  PassiveData,
  StatusEffect,
} from '../data/passives/types';

// Type aliases for backward compatibility
import type { PassiveData } from '../data/passives/types';
export type ClassPassiveSkill = PassiveData;
export type WeaponPassiveSkill = PassiveData;
export type AnyPassiveSkill = PassiveData;
