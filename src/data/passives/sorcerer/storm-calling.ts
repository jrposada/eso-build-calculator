import { ClassPassiveSkill } from '../../../models/passive';

export const STORM_CALLING_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Amplitude',
    skillLine: 'StormCalling',
    esoClass: 'Sorcerer',
    bonuses: [
      {
        type: 'damageType',
        value: 0.05,
        damageTypes: ['shock'],
      },
    ],
  },
  {
    name: 'Implosion',
    skillLine: 'StormCalling',
    esoClass: 'Sorcerer',
    bonuses: [{ type: 'skillLine', value: 0.1 }],
  },
];
