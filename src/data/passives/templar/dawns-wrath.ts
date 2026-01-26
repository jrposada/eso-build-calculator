import { ClassPassiveSkill } from '../../../models/passive';

export const DAWNS_WRATH_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Illuminate',
    skillLine: 'DawnsWrath',
    esoClass: 'Templar',
    bonuses: [], // Minor Sorcery for group - not tracked
  },
  {
    name: 'Restoring Spirit',
    skillLine: 'DawnsWrath',
    esoClass: 'Templar',
    bonuses: [], // Cost reduction - not relevant for damage
  },
  {
    name: 'Enduring Rays',
    skillLine: 'DawnsWrath',
    esoClass: 'Templar',
    bonuses: [{ type: 'duration', value: 3, multiplier: 'skillLine' }],
  },
  {
    name: 'Prism',
    skillLine: 'DawnsWrath',
    esoClass: 'Templar',
    bonuses: [], // Ultimate gain - not relevant for damage
  },
];
