import { WeaponPassiveSkill } from '../../../models/passive';

export const BOW_PASSIVES: WeaponPassiveSkill[] = [
  {
    name: 'Long Shots',
    skillLine: 'Bow',
    bonuses: [{ type: 'skillLine', value: 0.12 }],
  },
  {
    name: 'Accuracy',
    skillLine: 'Bow',
    bonuses: [{ type: 'skillLine', value: 0.08 }],
  },
  {
    name: 'Hawk Eye',
    skillLine: 'Bow',
    bonuses: [{ type: 'skillLine', value: 0.05 }],
  },
];
