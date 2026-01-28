import { WardenPassive } from './types';

export const ANIMAL_COMPANIONS_PASSIVES: WardenPassive<'AnimalCompanions'>[] = [
  {
    name: 'Bond With Nature',
    skillLine: 'AnimalCompanions',
    className: 'Warden',
    bonuses: [], // Healing received - not relevant for damage
  },
  {
    name: 'Savage Beast',
    skillLine: 'AnimalCompanions',
    className: 'Warden',
    bonuses: [], // Ultimate gain - not relevant for damage
  },
  {
    name: 'Flourish',
    skillLine: 'AnimalCompanions',
    className: 'Warden',
    bonuses: [], // Magicka/Stamina recovery - not relevant for damage
  },
  {
    name: 'Advanced Species',
    skillLine: 'AnimalCompanions',
    className: 'Warden',
    bonuses: [
      {
        className: 'ability-slotted-count',
        type: 'critical-damage',
        value: 0.03,
      },
    ],
  },
];
