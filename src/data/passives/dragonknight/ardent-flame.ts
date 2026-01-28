import { DragonknightPassive } from './types';

export const ARDENT_FLAME_PASSIVES: DragonknightPassive<'ArdentFlame'>[] = [
  {
    name: 'Combustion',
    skillLine: 'ArdentFlame',
    className: 'Dragonknight',
    bonuses: [], // Ultimate generation on status effect application - not relevant for damage
  },
  {
    name: 'Warmth',
    skillLine: 'ArdentFlame',
    className: 'Dragonknight',
    bonuses: [], // Snare reduction - not relevant for damage
  },
  {
    name: 'Searing Heat',
    skillLine: 'ArdentFlame',
    className: 'Dragonknight',
    bonuses: [
      {
        className: 'skill-line',
        type: 'duration',
        value: 2,
      },
    ],
  },
  {
    name: 'World in Ruin',
    skillLine: 'ArdentFlame',
    className: 'Dragonknight',
    bonuses: [], // Damage done - handled by base skill damage
  },
];
