import { ArcanistPassive } from './types';

export const HERALD_OF_THE_TOME_PASSIVES: ArcanistPassive<'HeraldOfTheTome'>[] =
  [
    {
      name: 'Scion of Apocrypha',
      skillLine: 'HeraldOfTheTome',
      className: 'Arcanist',
      bonuses: [], // Magicka cost reduction - not relevant for damage
    },
    {
      name: 'Meticulous Curation',
      skillLine: 'HeraldOfTheTome',
      className: 'Arcanist',
      bonuses: [], // Crux effectiveness - complex, not tracked
    },
    {
      name: 'Sage-Sight Aura',
      skillLine: 'HeraldOfTheTome',
      className: 'Arcanist',
      bonuses: [], // Minor Courage for group - not tracked
    },
    {
      name: "Tome-Bearer's Inspiration",
      skillLine: 'HeraldOfTheTome',
      className: 'Arcanist',
      bonuses: [
        {
          type: 'critical-chance',
          value: 0.03,
          multiplier: 'abilitySlottedCount',
        },
      ],
    },
  ];
