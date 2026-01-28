import { ARCANIST_PASSIVES } from './arcanist';
import { DRAGONKNIGHT_PASSIVES } from './dragonknight';
import { NIGHTBLADE_PASSIVES } from './nightblade';
import { SORCERER_PASSIVES } from './sorcerer';
import { TEMPLAR_PASSIVES } from './templar';
import { WARDEN_PASSIVES } from './warden';
import { WEAPON_PASSIVES } from './weapon';

const ALL_CLASS_PASSIVES = [
  ...ARCANIST_PASSIVES,
  ...DRAGONKNIGHT_PASSIVES,
  ...NIGHTBLADE_PASSIVES,
  ...SORCERER_PASSIVES,
  ...TEMPLAR_PASSIVES,
  ...WARDEN_PASSIVES,
];
type ClassPassiveSkillLineName =
  (typeof ALL_CLASS_PASSIVES)[number]['skillLine'];

const ALL_WEAPON_PASSIVES = [...WEAPON_PASSIVES];
type WeaponPassiveSkillLineName =
  (typeof ALL_WEAPON_PASSIVES)[number]['skillLine'];

const ALL_PASSIVES = [...ALL_CLASS_PASSIVES, ...ALL_WEAPON_PASSIVES];

export { ALL_CLASS_PASSIVES, ALL_PASSIVES, ALL_WEAPON_PASSIVES };
export type { ClassPassiveSkillLineName, WeaponPassiveSkillLineName };
