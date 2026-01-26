import { WeaponPassiveSkill } from '../../../models/passive';

export const TWO_HANDED_PASSIVES: WeaponPassiveSkill[] = [
  {
    name: 'Forceful',
    skillLine: 'TwoHanded',
    bonuses: [], // Splash damage - handled by base skill damage
  },
  {
    name: 'Heavy Weapons',
    skillLine: 'TwoHanded',
    bonuses: [
      { type: 'critical-damage', value: 0.12, multiplier: 'skillLine' },
    ],
  },
  {
    name: 'Balanced Blade',
    skillLine: 'TwoHanded',
    bonuses: [], // Cost reduction - not relevant for damage
  },
  {
    name: 'Follow Up',
    skillLine: 'TwoHanded',
    bonuses: [], // Damage buff after heavy attack - complex, not tracked
  },
];
