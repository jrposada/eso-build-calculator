import { Skill } from '../../../models/skill';
import { DAEDRIC_SUMMONING_SKILLS } from './daedric-summoning';
import { DARK_MAGIC_SKILLS } from './dark-magic';
import { STORM_CALLING_SKILLS } from './storm-calling';

const SORCERER_SKILLS: Skill<'Sorcerer'>[] = [
  ...DARK_MAGIC_SKILLS,
  ...DAEDRIC_SUMMONING_SKILLS,
  ...STORM_CALLING_SKILLS,
];

export { SORCERER_SKILLS };
