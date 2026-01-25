import { WeaponPassiveSkill } from '../../../models/passive';

export const TWO_HANDED_PASSIVES: WeaponPassiveSkill[] = [
  {
    name: 'Heavy Weapons',
    skillLine: 'TwoHanded',
    bonuses: [{ type: 'skillLine', value: 0.12 }],
  },
  {
    name: 'Balanced Blade',
    skillLine: 'TwoHanded',
    bonuses: [{ type: 'skillLine', value: 0.1 }],
  },
  {
    name: 'Follow Up',
    skillLine: 'TwoHanded',
    bonuses: [{ type: 'direct', value: 0.1 }],
  },
];
