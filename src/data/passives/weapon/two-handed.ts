import { WeaponPassive } from './types';

export const TWO_HANDED_PASSIVES: WeaponPassive<'TwoHanded'>[] = [
  {
    name: 'Forceful',
    className: 'Weapon',
    skillLine: 'TwoHanded',
    bonuses: [], // Splash damage - handled by base skill damage
  },
  {
    name: 'Heavy Weapons',
    className: 'Weapon',
    skillLine: 'TwoHanded',
    bonuses: [
      { type: 'critical-damage', value: 0.12, multiplier: 'skillLine' },
    ],
  },
  {
    name: 'Balanced Blade',
    className: 'Weapon',
    skillLine: 'TwoHanded',
    bonuses: [], // Cost reduction - not relevant for damage
  },
  {
    name: 'Follow Up',
    className: 'Weapon',
    skillLine: 'TwoHanded',
    bonuses: [], // Damage buff after heavy attack - complex, not tracked
  },
];
