import { WeaponPassiveSkill } from '../../../models/passive';
import { BOW_PASSIVES } from './bow';
import { DESTRUCTION_STAFF_PASSIVES } from './destruction-staff';
import { DUAL_WIELD_PASSIVES } from './dual-wield';
import { TWO_HANDED_PASSIVES } from './two-handed';

export const WEAPON_PASSIVES: WeaponPassiveSkill[] = [
  ...BOW_PASSIVES,
  ...DUAL_WIELD_PASSIVES,
  ...TWO_HANDED_PASSIVES,
  ...DESTRUCTION_STAFF_PASSIVES,
];

export {
  BOW_PASSIVES,
  DESTRUCTION_STAFF_PASSIVES,
  DUAL_WIELD_PASSIVES,
  TWO_HANDED_PASSIVES,
};
