import { ClassPassiveSkill } from '../../../models/passive';
import { ANIMAL_COMPANIONS_PASSIVES } from './animal-companions';
import { GREEN_BALANCE_PASSIVES } from './green-balance';
import { WINTERS_EMBRACE_PASSIVES } from './winters-embrace';

export const WARDEN_PASSIVES: ClassPassiveSkill[] = [
  ...ANIMAL_COMPANIONS_PASSIVES,
  ...GREEN_BALANCE_PASSIVES,
  ...WINTERS_EMBRACE_PASSIVES,
];

export {
  ANIMAL_COMPANIONS_PASSIVES,
  GREEN_BALANCE_PASSIVES,
  WINTERS_EMBRACE_PASSIVES,
};
