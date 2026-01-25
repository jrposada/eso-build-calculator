import { ClassPassiveSkill } from '../../../models/passive';
import { ARDENT_FLAME_PASSIVES } from './ardent-flame';
import { DRACONIC_POWER_PASSIVES } from './draconic-power';
import { EARTHEN_HEART_PASSIVES } from './earthen-heart';

export const DRAGONKNIGHT_PASSIVES: ClassPassiveSkill[] = [
  ...ARDENT_FLAME_PASSIVES,
  ...DRACONIC_POWER_PASSIVES,
  ...EARTHEN_HEART_PASSIVES,
];

export {
  ARDENT_FLAME_PASSIVES,
  DRACONIC_POWER_PASSIVES,
  EARTHEN_HEART_PASSIVES,
};
