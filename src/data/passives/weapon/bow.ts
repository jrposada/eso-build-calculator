import { WeaponPassiveSkill } from '../../../models/passive';

export const BOW_PASSIVES: WeaponPassiveSkill[] = [
  {
    name: 'Long Shots',
    skillLine: 'Bow',
    bonuses: [], // Damage at range - situational, not tracked
  },
  {
    name: 'Accuracy',
    skillLine: 'Bow',
    bonuses: [
      { type: 'critical-chance', value: 0.08, multiplier: 'skillLine' },
    ],
  },
  {
    name: 'Ranger',
    skillLine: 'Bow',
    bonuses: [], // Cost reduction while moving - not relevant for damage
  },
  {
    name: 'Hawk Eye',
    skillLine: 'Bow',
    bonuses: [
      {
        type: 'critical-damage',
        value: 0.02,
        multiplier: 'abilitySlottedCount',
      },
    ],
  },
];
