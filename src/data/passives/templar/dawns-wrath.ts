import { TemplarPassive } from './types';

export const DAWNS_WRATH_PASSIVES: TemplarPassive<'DawnsWrath'>[] = [
  {
    name: 'Illuminate',
    skillLine: 'DawnsWrath',
    className: 'Templar',
    bonuses: [], // Minor Sorcery for group - not tracked
  },
  {
    name: 'Restoring Spirit',
    skillLine: 'DawnsWrath',
    className: 'Templar',
    bonuses: [], // Cost reduction - not relevant for damage
  },
  {
    name: 'Enduring Rays',
    skillLine: 'DawnsWrath',
    className: 'Templar',
    bonuses: [
      {
        className: 'skill-line',
        type: 'duration',
        value: 3,
      },
    ],
  },
  {
    name: 'Prism',
    skillLine: 'DawnsWrath',
    className: 'Templar',
    bonuses: [], // Ultimate gain - not relevant for damage
  },
];
