import { ClassPassiveSkill } from '../../../models/passive';

export const HERALD_OF_THE_TOME_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Scion of Apocrypha',
    skillLine: 'HeraldOfTheTome',
    esoClass: 'Arcanist',
    bonuses: [], // Magicka cost reduction - not relevant for damage
  },
  {
    name: 'Meticulous Curation',
    skillLine: 'HeraldOfTheTome',
    esoClass: 'Arcanist',
    bonuses: [], // Crux effectiveness - complex, not tracked
  },
  {
    name: 'Sage-Sight Aura',
    skillLine: 'HeraldOfTheTome',
    esoClass: 'Arcanist',
    bonuses: [], // Minor Courage for group - not tracked
  },
  {
    name: "Tome-Bearer's Inspiration",
    skillLine: 'HeraldOfTheTome',
    esoClass: 'Arcanist',
    bonuses: [
      {
        type: 'critical-chance',
        value: 0.03,
        multiplier: 'abilitySlottedCount',
      },
    ],
  },
];
