import { ClassSkill, WeaponSkill } from '../../models/skill';
import { ARCANIST_SKILLS } from './arcanist';
import { DRAGONKNIGHT_SKILLS } from './dragonknight';
import { NIGHTBLADE_SKILLS } from './nightblade';
import { SORCERER_SKILLS } from './sorcerer';
import { TEMPLAR_SKILLS } from './templar';
import { WARDEN_SKILLS } from './warden';
import { WEAPON_SKILLS } from './weapon';

const ALL_CLASS_SKILLS: ClassSkill[] = [
  ...ARCANIST_SKILLS,
  ...DRAGONKNIGHT_SKILLS,
  ...NIGHTBLADE_SKILLS,
  ...SORCERER_SKILLS,
  ...TEMPLAR_SKILLS,
  ...WARDEN_SKILLS,
];

const ALL_WEAPON_SKILLS: WeaponSkill[] = [...WEAPON_SKILLS];

const ALL_SKILLS: (ClassSkill | WeaponSkill)[] = [
  ...ALL_CLASS_SKILLS,
  ...ALL_WEAPON_SKILLS,
];

export { ALL_CLASS_SKILLS, ALL_SKILLS, ALL_WEAPON_SKILLS };
