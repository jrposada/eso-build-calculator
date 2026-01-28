import { WeaponPassive } from './types';

export const BOW_PASSIVES: WeaponPassive<'Bow'>[] = [
  {
    name: 'Long Shots',
    className: 'Weapon',
    skillLine: 'Bow',
    bonuses: [], // Damage at range - situational, not tracked
  },
  {
    name: 'Accuracy',
    className: 'Weapon',
    skillLine: 'Bow',
    bonuses: [
      {
        className: 'skill-line',
        type: 'critical-chance',
        value: 0.08,
      },
    ],
  },
  {
    name: 'Ranger',
    className: 'Weapon',
    skillLine: 'Bow',
    bonuses: [], // Cost reduction while moving - not relevant for damage
  },
  {
    name: 'Hawk Eye',
    className: 'Weapon',
    skillLine: 'Bow',
    bonuses: [
      {
        className: 'ability-slotted-count',
        type: 'critical-damage',
        value: 0.02,
      },
    ],
  },
];
