import { ARCANIST_SKILLS } from './arcanist';
import { DRAGONKNIGHT_SKILLS } from './dragonknight';
import { NIGHTBLADE_SKILLS } from './nightblade';
import { SORCERER_SKILLS } from './sorcerer';
import { TEMPLAR_SKILLS } from './templar';
import { WARDEN_SKILLS } from './warden';
import { WEAPON_SKILLS } from './weapon';

const ALL_CLASS_SKILLS = [
  ...ARCANIST_SKILLS,
  ...DRAGONKNIGHT_SKILLS,
  ...NIGHTBLADE_SKILLS,
  ...SORCERER_SKILLS,
  ...TEMPLAR_SKILLS,
  ...WARDEN_SKILLS,
];
type ClassSkillLineName = (typeof ALL_CLASS_SKILLS)[number]['skillLine'];
type ClassClassName = (typeof ALL_CLASS_SKILLS)[number]['className'];

const ALL_WEAPON_SKILLS = [...WEAPON_SKILLS];
type WeaponSkillLineName = (typeof ALL_WEAPON_SKILLS)[number]['skillLine'];

const ALL_SKILLS = [...ALL_CLASS_SKILLS, ...ALL_WEAPON_SKILLS];
type SkillLineName = (typeof ALL_SKILLS)[number]['skillLine'];

export type {
  ClassClassName,
  ClassSkillLineName,
  SkillLineName,
  WeaponSkillLineName,
};
export { ALL_CLASS_SKILLS, ALL_SKILLS, ALL_WEAPON_SKILLS };
