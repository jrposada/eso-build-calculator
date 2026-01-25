import { ClassPassiveSkill } from '../../../models/passive';

export const DAEDRIC_SUMMONING_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Daedric Protection',
    skillLine: 'DaedricSummoning',
    esoClass: 'Sorcerer',
    bonuses: [{ type: 'skillLine', value: 0.1 }],
  },
];
