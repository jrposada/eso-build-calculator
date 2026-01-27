import { CURATIVE_RUNEFORMS_SKILLS } from './curative-runeforms';
import { HERALD_OF_THE_TOME_SKILLS } from './herald-of-the-tome';
import { SOLDIER_OF_APOCRYPHA_SKILLS } from './soldier-of-apocrypha';
import type { ArcanistSkill } from './types';

const ARCANIST_SKILLS: ArcanistSkill[] = [
  ...CURATIVE_RUNEFORMS_SKILLS,
  ...HERALD_OF_THE_TOME_SKILLS,
  ...SOLDIER_OF_APOCRYPHA_SKILLS,
];

export { ARCANIST_SKILLS };
