import { ARDENT_FLAME_PASSIVES } from './ardent-flame';
import { DRACONIC_POWER_PASSIVES } from './draconic-power';
import { EARTHEN_HEART_PASSIVES } from './earthen-heart';
import type { DragonknightPassive } from './types';

const DRAGONKNIGHT_PASSIVES: DragonknightPassive[] = [
  ...ARDENT_FLAME_PASSIVES,
  ...DRACONIC_POWER_PASSIVES,
  ...EARTHEN_HEART_PASSIVES,
];

export { DRAGONKNIGHT_PASSIVES };
