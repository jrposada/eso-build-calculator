import { ASSASSINATION_SKILLS } from './assassination';
import { SHADOW_SKILLS } from './shadow';
import { SIPHONING_SKILLS } from './siphoning';
import type { NightbladeSkill } from './types';

const NIGHTBLADE_SKILLS: NightbladeSkill[] = [
  ...ASSASSINATION_SKILLS,
  ...SHADOW_SKILLS,
  ...SIPHONING_SKILLS,
];

export { NIGHTBLADE_SKILLS };
