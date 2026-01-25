import { ClassPassiveSkill } from '../../../models/passive';

export const WINTERS_EMBRACE_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Icy Aura',
    skillLine: 'WintersEmbrace',
    esoClass: 'Warden',
    bonuses: [
      {
        type: 'statusEffect',
        value: 0.2,
        statusEffects: ['Chilled'],
      },
    ],
  },
  {
    name: 'Piercing Cold',
    skillLine: 'WintersEmbrace',
    esoClass: 'Warden',
    bonuses: [
      {
        type: 'damageType',
        value: 0.05,
        damageTypes: ['frost'],
      },
    ],
  },
];
