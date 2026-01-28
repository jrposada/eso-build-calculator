import { NightbladePassive } from './types';

export const SHADOW_PASSIVES: NightbladePassive<'Shadow'>[] = [
  {
    name: 'Refreshing Shadows',
    skillLine: 'Shadow',
    className: 'Nightblade',
    bonuses: [], // Non relevant
  },
  {
    name: 'Shadow Barrier',
    skillLine: 'Shadow',
    className: 'Nightblade',
    bonuses: [], // Non relevant
  },
  {
    name: 'Dark Vigor',
    skillLine: 'Shadow',
    className: 'Nightblade',
    bonuses: [], // Non relevant
  },
  {
    name: 'Dark Veil',
    skillLine: 'Shadow',
    className: 'Nightblade',
    bonuses: [{ type: 'duration', value: 2, multiplier: 'skillLine' }],
  },
];
