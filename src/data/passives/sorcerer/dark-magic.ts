import { ClassPassiveSkill } from '../../../models/passive';

export const DARK_MAGIC_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Exploitation',
    skillLine: 'DarkMagic',
    esoClass: 'Sorcerer',
    bonuses: [{ type: 'skillLine', value: 0.1 }],
  },
];
