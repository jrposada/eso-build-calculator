import { ClassPassiveSkill } from '../../../models/passive';

export const ARDENT_FLAME_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Combustion',
    skillLine: 'ArdentFlame',
    esoClass: 'Dragonknight',
    bonuses: [], // Ultimate generation on status effect application - not relevant for damage
  },
  {
    name: 'Warmth',
    skillLine: 'ArdentFlame',
    esoClass: 'Dragonknight',
    bonuses: [], // Snare reduction - not relevant for damage
  },
  {
    name: 'Searing Heat',
    skillLine: 'ArdentFlame',
    esoClass: 'Dragonknight',
    bonuses: [{ type: 'duration', value: 2, multiplier: 'skillLine' }],
  },
  {
    name: 'World in Ruin',
    skillLine: 'ArdentFlame',
    esoClass: 'Dragonknight',
    bonuses: [], // Damage done - handled by base skill damage
  },
];
