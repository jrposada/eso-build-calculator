import {
  ALL_CLASS_PASSIVES,
  ALL_WEAPON_PASSIVES,
} from '../data/passives';
import { PassiveData } from '../data/passives/types';

/**
 * Get all class passives for a specific skill line
 */
export function getClassPassivesBySkillLine(skillLine: string): PassiveData[] {
  return ALL_CLASS_PASSIVES.filter((p) => p.skillLine === skillLine);
}

/**
 * Get all weapon passives for a specific skill line
 */
export function getWeaponPassivesBySkillLine(skillLine: string): PassiveData[] {
  return ALL_WEAPON_PASSIVES.filter((p) => p.skillLine === skillLine);
}
