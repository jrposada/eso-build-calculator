import { ClassPassiveSkill } from '../../../models/passive';

export const AEDRIC_SPEAR_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Piercing Spear',
    skillLine: 'AedricSpear',
    esoClass: 'Templar',
    bonuses: [{ type: 'skillLine', value: 0.1 }],
  },
  {
    name: 'Burning Light',
    skillLine: 'AedricSpear',
    esoClass: 'Templar',
    bonuses: [{ type: 'skillLine', value: 0.2 }],
  },
];
