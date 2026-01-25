import { ClassPassiveSkill } from '../../../models/passive';

export const SIPHONING_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Magicka Flood',
    skillLine: 'Siphoning',
    esoClass: 'Nightblade',
    bonuses: [{ type: 'skillLine', value: 0.1 }],
  },
];
