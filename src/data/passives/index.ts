import { ClassPassiveSkill, WeaponPassiveSkill } from '../../models/passive';
import { ARCANIST_PASSIVES } from './arcanist';
import { DRAGONKNIGHT_PASSIVES } from './dragonknight';
import { NIGHTBLADE_PASSIVES } from './nightblade';
import { SORCERER_PASSIVES } from './sorcerer';
import { TEMPLAR_PASSIVES } from './templar';
import { WARDEN_PASSIVES } from './warden';
import { WEAPON_PASSIVES } from './weapon';

export const ALL_CLASS_PASSIVES: ClassPassiveSkill[] = [
  ...ARCANIST_PASSIVES,
  ...DRAGONKNIGHT_PASSIVES,
  ...NIGHTBLADE_PASSIVES,
  ...SORCERER_PASSIVES,
  ...TEMPLAR_PASSIVES,
  ...WARDEN_PASSIVES,
];

export const ALL_WEAPON_PASSIVES: WeaponPassiveSkill[] = [...WEAPON_PASSIVES];

export const ALL_PASSIVES: (ClassPassiveSkill | WeaponPassiveSkill)[] = [
  ...ALL_CLASS_PASSIVES,
  ...ALL_WEAPON_PASSIVES,
];

/**
 * Get all class passives for a specific skill line
 */
export function getClassPassivesBySkillLine(
  skillLine: string,
): ClassPassiveSkill[] {
  return ALL_CLASS_PASSIVES.filter((p) => p.skillLine === skillLine);
}

/**
 * Get all weapon passives for a specific skill line
 */
export function getWeaponPassivesBySkillLine(
  skillLine: string,
): WeaponPassiveSkill[] {
  return ALL_WEAPON_PASSIVES.filter((p) => p.skillLine === skillLine);
}

export {
  ARCANIST_PASSIVES,
  DRAGONKNIGHT_PASSIVES,
  NIGHTBLADE_PASSIVES,
  SORCERER_PASSIVES,
  TEMPLAR_PASSIVES,
  WARDEN_PASSIVES,
  WEAPON_PASSIVES,
};
