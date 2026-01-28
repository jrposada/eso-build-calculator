import { NightbladePassive } from './types';

export const SIPHONING_PASSIVES: NightbladePassive<'Siphoning'>[] = [
  {
    name: 'Catalyst',
    skillLine: 'Siphoning',
    className: 'Nightblade',
    bonuses: [], // Non relevant
  },
  {
    name: 'Magicka Flood',
    skillLine: 'Siphoning',
    className: 'Nightblade',
    bonuses: [
      {
        className: 'skill-line',
        type: 'max-stamina',
        value: 0.06,
      },
    ],
  },
  {
    name: 'Soul Siphoner',
    skillLine: 'Siphoning',
    className: 'Nightblade',
    bonuses: [], // Non relevant
  },
  {
    name: 'Transfer',
    skillLine: 'Siphoning',
    className: 'Nightblade',
    bonuses: [], // Non relevant
  },
];
