import { DAEDRIC_SUMMONING_PASSIVES } from './daedric-summoning';
import { DARK_MAGIC_PASSIVES } from './dark-magic';
import { STORM_CALLING_PASSIVES } from './storm-calling';
import type { SorcererPassive } from './types';

const SORCERER_PASSIVES: SorcererPassive[] = [
  ...DARK_MAGIC_PASSIVES,
  ...DAEDRIC_SUMMONING_PASSIVES,
  ...STORM_CALLING_PASSIVES,
];

export { SORCERER_PASSIVES };
