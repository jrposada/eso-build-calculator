import { ClassPassiveSkill } from '../../../models/passive';

export const ANIMAL_COMPANIONS_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Bond With Nature',
    skillLine: 'AnimalCompanions',
    esoClass: 'Warden',
    bonuses: [], // Healing received - not relevant for damage
  },
  {
    name: 'Savage Beast',
    skillLine: 'AnimalCompanions',
    esoClass: 'Warden',
    bonuses: [], // Ultimate gain - not relevant for damage
  },
  {
    name: 'Flourish',
    skillLine: 'AnimalCompanions',
    esoClass: 'Warden',
    bonuses: [], // Magicka/Stamina recovery - not relevant for damage
  },
  {
    name: 'Advanced Species',
    skillLine: 'AnimalCompanions',
    esoClass: 'Warden',
    bonuses: [
      {
        type: 'critical-damage',
        value: 0.03,
        multiplier: 'abilitySlottedCount',
      },
    ],
  },
];
