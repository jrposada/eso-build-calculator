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
    bonuses: [
      {
        className: 'skill-line',
        type: 'duration',
        value: 2,
      },
    ],
  },
];
