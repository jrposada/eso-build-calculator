import { ClassPassiveSkill } from '../../../models/passive';

export const ANIMAL_COMPANIONS_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Bond With Nature',
    skillLine: 'AnimalCompanions',
    esoClass: 'Warden',
    bonuses: [{ type: 'skillLine', value: 0.1 }],
  },
  {
    name: 'Savage Beast',
    skillLine: 'AnimalCompanions',
    esoClass: 'Warden',
    bonuses: [{ type: 'skillLine', value: 0.12 }],
  },
];
