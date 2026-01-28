import { CURATIVE_RUNEFORMS_PASSIVES } from './curative-runeforms';
import { HERALD_OF_THE_TOME_PASSIVES } from './herald-of-the-tome';
import { SOLDIER_OF_APOCRYPHA_PASSIVES } from './soldier-of-apocrypha';
import type { ArcanistPassive } from './types';

const ARCANIST_PASSIVES: ArcanistPassive[] = [
  ...HERALD_OF_THE_TOME_PASSIVES,
  ...SOLDIER_OF_APOCRYPHA_PASSIVES,
  ...CURATIVE_RUNEFORMS_PASSIVES,
];

export { ARCANIST_PASSIVES };
