import { ClassPassiveSkill } from '../../../models/passive';

export const ASSASSINATION_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Master Assassin',
    skillLine: 'Assassination',
    esoClass: 'Nightblade',
    bonuses: [{ type: 'direct', value: 0.1 }],
  },
  {
    name: 'Executioner',
    skillLine: 'Assassination',
    esoClass: 'Nightblade',
    bonuses: [{ type: 'skillLine', value: 0.2 }],
  },
];
