import { AEDRIC_SPEAR_PASSIVES } from './aedric-spear';
import { DAWNS_WRATH_PASSIVES } from './dawns-wrath';
import { RESTORING_LIGHT_PASSIVES } from './restoring-light';
import type { TemplarPassive } from './types';

const TEMPLAR_PASSIVES: TemplarPassive[] = [
  ...AEDRIC_SPEAR_PASSIVES,
  ...DAWNS_WRATH_PASSIVES,
  ...RESTORING_LIGHT_PASSIVES,
];

export { TEMPLAR_PASSIVES };
