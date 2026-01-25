import { ClassPassiveSkill } from '../../../models/passive';
import { DAEDRIC_SUMMONING_PASSIVES } from './daedric-summoning';
import { DARK_MAGIC_PASSIVES } from './dark-magic';
import { STORM_CALLING_PASSIVES } from './storm-calling';

export const SORCERER_PASSIVES: ClassPassiveSkill[] = [
  ...DARK_MAGIC_PASSIVES,
  ...DAEDRIC_SUMMONING_PASSIVES,
  ...STORM_CALLING_PASSIVES,
];

export {
  DAEDRIC_SUMMONING_PASSIVES,
  DARK_MAGIC_PASSIVES,
  STORM_CALLING_PASSIVES,
};
