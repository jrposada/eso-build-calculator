import { ANIMAL_COMPANIONS_SKILLS } from './animal-companions';
import { GREEN_BALANCE_SKILLS } from './green-balance';
import type { WardenSkill } from './types';
import { WINTERS_EMBRACE_SKILLS } from './winters-embrace';

const WARDEN_SKILLS: WardenSkill[] = [
  ...ANIMAL_COMPANIONS_SKILLS,
  ...GREEN_BALANCE_SKILLS,
  ...WINTERS_EMBRACE_SKILLS,
];

export { WARDEN_SKILLS };
