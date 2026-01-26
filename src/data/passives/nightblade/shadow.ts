import { ClassPassiveSkill } from '../../../models/passive';

export const SHADOW_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Refreshing Shadows',
    skillLine: 'Shadow',
    esoClass: 'Nightblade',
    bonuses: [], // Non relevant
  },
  {
    name: 'Shadow Barrier',
    skillLine: 'Shadow',
    esoClass: 'Nightblade',
    bonuses: [], // Non relevant
  },
  {
    name: 'Dark Vigor',
    skillLine: 'Shadow',
    esoClass: 'Nightblade',
    bonuses: [], // Non relevant
  },
  {
    name: 'Dark Veil',
    skillLine: 'Shadow',
    esoClass: 'Nightblade',
    bonuses: [{ type: 'duration', value: 2, multiplier: 'skillLine' }],
  },
];
