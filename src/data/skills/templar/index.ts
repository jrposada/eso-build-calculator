import { AEDRIC_SPEAR_SKILLS } from './aedric-spear';
import { DAWNS_WRATH_SKILLS } from './dawns-wrath';
import { RESTORING_LIGHT_SKILLS } from './restoring-light';
import type { TemplarSkill } from './types';

const TEMPLAR_SKILLS: TemplarSkill[] = [
  ...AEDRIC_SPEAR_SKILLS,
  ...DAWNS_WRATH_SKILLS,
  ...RESTORING_LIGHT_SKILLS,
];

export { TEMPLAR_SKILLS };
