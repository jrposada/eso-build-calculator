import { BOW_PASSIVES } from './bow';
import { DESTRUCTION_STAFF_PASSIVES } from './destruction-staff';
import { DUAL_WIELD_PASSIVES } from './dual-wield';
import { TWO_HANDED_PASSIVES } from './two-handed';
import type { WeaponPassive } from './types';

const WEAPON_PASSIVES: WeaponPassive[] = [
  ...BOW_PASSIVES,
  ...DUAL_WIELD_PASSIVES,
  ...TWO_HANDED_PASSIVES,
  ...DESTRUCTION_STAFF_PASSIVES,
];

export { WEAPON_PASSIVES };
