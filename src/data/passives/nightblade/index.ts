import { ClassPassiveSkill } from '../../../models/passive';
import { ASSASSINATION_PASSIVES } from './assassination';
import { SHADOW_PASSIVES } from './shadow';
import { SIPHONING_PASSIVES } from './siphoning';

export const NIGHTBLADE_PASSIVES: ClassPassiveSkill[] = [
  ...ASSASSINATION_PASSIVES,
  ...SHADOW_PASSIVES,
  ...SIPHONING_PASSIVES,
];

export { ASSASSINATION_PASSIVES, SHADOW_PASSIVES, SIPHONING_PASSIVES };
