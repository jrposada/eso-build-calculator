import { ANIMAL_COMPANIONS_PASSIVES } from './animal-companions';
import { GREEN_BALANCE_PASSIVES } from './green-balance';
import type { WardenPassive } from './types';
import { WINTERS_EMBRACE_PASSIVES } from './winters-embrace';

const WARDEN_PASSIVES: WardenPassive[] = [
  ...ANIMAL_COMPANIONS_PASSIVES,
  ...GREEN_BALANCE_PASSIVES,
  ...WINTERS_EMBRACE_PASSIVES,
];

export { WARDEN_PASSIVES };
