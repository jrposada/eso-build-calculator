import { WeaponPassive } from './types';

export const DUAL_WIELD_PASSIVES: WeaponPassive<'DualWield'>[] = [
  {
    name: 'Slaughter',
    className: 'Weapon',
    skillLine: 'DualWield',
    bonuses: [], // Damage to low health - situational, not tracked
  },
  {
    name: 'Dual Wield Expert',
    className: 'Weapon',
    skillLine: 'DualWield',
    bonuses: [], // Off-hand weapon damage - handled by base damage
  },
  {
    name: 'Controlled Fury',
    className: 'Weapon',
    skillLine: 'DualWield',
    bonuses: [], // Cost reduction - not relevant for damage
  },
  {
    name: 'Twin Blade and Blunt',
    className: 'Weapon',
    skillLine: 'DualWield',
    bonuses: [
      { type: 'critical-damage', value: 0.05, multiplier: 'skillLine' },
    ],
  },
];
