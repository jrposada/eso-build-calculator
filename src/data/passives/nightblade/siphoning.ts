import { ClassPassiveSkill } from '../../../models/passive';

export const SIPHONING_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Catalyst',
    skillLine: 'Siphoning',
    esoClass: 'Nightblade',
    bonuses: [], // Non relevant
  },
  {
    name: 'Magicka Flood',
    skillLine: 'Siphoning',
    esoClass: 'Nightblade',
    bonuses: [{ type: 'max-stamina', value: 0.06, multiplier: 'skillLine' }],
  },
  {
    name: 'Soul Siphoner',
    skillLine: 'Siphoning',
    esoClass: 'Nightblade',
    bonuses: [], // Non relevant
  },
  {
    name: 'Transfer',
    skillLine: 'Siphoning',
    esoClass: 'Nightblade',
    bonuses: [], // Non relevant
  },
];
