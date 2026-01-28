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
    bonuses: [{ type: 'max-stamina', value: 0.06, multiplier: 'skillLine' }],
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
