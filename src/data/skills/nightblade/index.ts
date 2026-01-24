import { Skill } from '../../../models/skill';
import { ASSASSINATION_SKILLS } from './assassination';
import { SHADOW_SKILLS } from './shadow';
import { SIPHONING_SKILLS } from './siphoning';

const NIGHTBLADE_SKILLS: Skill<'Nightblade'>[] = [
  ...ASSASSINATION_SKILLS,
  ...SHADOW_SKILLS,
  ...SIPHONING_SKILLS,
];

export { NIGHTBLADE_SKILLS };
