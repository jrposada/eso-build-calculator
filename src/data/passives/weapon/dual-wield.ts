import { WeaponPassiveSkill } from '../../../models/passive';

export const DUAL_WIELD_PASSIVES: WeaponPassiveSkill[] = [
  {
    name: 'Slaughter',
    skillLine: 'DualWield',
    bonuses: [], // Damage to low health - situational, not tracked
  },
  {
    name: 'Dual Wield Expert',
    skillLine: 'DualWield',
    bonuses: [], // Off-hand weapon damage - handled by base damage
  },
  {
    name: 'Controlled Fury',
    skillLine: 'DualWield',
    bonuses: [], // Cost reduction - not relevant for damage
  },
  {
    name: 'Twin Blade and Blunt',
    skillLine: 'DualWield',
    bonuses: [
      { type: 'critical-damage', value: 0.05, multiplier: 'skillLine' },
    ],
  },
];
