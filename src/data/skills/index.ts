import { Skill } from '../../models/skill';
import { ARCANIST_SKILLS } from './arcanist';
import { DRAGONKNIGHT_SKILLS } from './dragonknight';
import { NIGHTBLADE_SKILLS } from './nightblade';
import { SORCERER_SKILLS } from './sorcerer';
import { TEMPLAR_SKILLS } from './templar';
import { WARDEN_SKILLS } from './warden';

const ALL_SKILLS: Skill[] = [
  ...ARCANIST_SKILLS,
  ...DRAGONKNIGHT_SKILLS,
  ...NIGHTBLADE_SKILLS,
  ...SORCERER_SKILLS,
  ...TEMPLAR_SKILLS,
  ...WARDEN_SKILLS,
];

export { ALL_SKILLS };
