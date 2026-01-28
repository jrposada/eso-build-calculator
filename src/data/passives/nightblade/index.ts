import { ASSASSINATION_PASSIVES } from './assassination';
import { SHADOW_PASSIVES } from './shadow';
import { SIPHONING_PASSIVES } from './siphoning';
import type { NightbladePassive } from './types';

const NIGHTBLADE_PASSIVES: NightbladePassive[] = [
  ...ASSASSINATION_PASSIVES,
  ...SHADOW_PASSIVES,
  ...SIPHONING_PASSIVES,
];

export { NIGHTBLADE_PASSIVES };
