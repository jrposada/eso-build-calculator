import { ClassPassiveSkill } from '../../../models/passive';
import { AEDRIC_SPEAR_PASSIVES } from './aedric-spear';
import { DAWNS_WRATH_PASSIVES } from './dawns-wrath';
import { RESTORING_LIGHT_PASSIVES } from './restoring-light';

export const TEMPLAR_PASSIVES: ClassPassiveSkill[] = [
  ...AEDRIC_SPEAR_PASSIVES,
  ...DAWNS_WRATH_PASSIVES,
  ...RESTORING_LIGHT_PASSIVES,
];

export {
  AEDRIC_SPEAR_PASSIVES,
  DAWNS_WRATH_PASSIVES,
  RESTORING_LIGHT_PASSIVES,
};
