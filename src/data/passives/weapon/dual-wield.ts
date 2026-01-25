import { WeaponPassiveSkill } from '../../../models/passive';

export const DUAL_WIELD_PASSIVES: WeaponPassiveSkill[] = [
  {
    name: 'Dual Wield Expert',
    skillLine: 'DualWield',
    bonuses: [{ type: 'skillLine', value: 0.06 }],
  },
  {
    name: 'Ruffian',
    skillLine: 'DualWield',
    bonuses: [{ type: 'skillLine', value: 0.1 }],
  },
  {
    name: 'Twin Blade and Blunt',
    skillLine: 'DualWield',
    bonuses: [
      {
        type: 'damageType',
        value: 0.05,
        damageTypes: ['bleed'],
      },
    ],
  },
];
