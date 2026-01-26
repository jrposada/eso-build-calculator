import { ClassPassiveSkill } from '../../../models/passive';

export const DARK_MAGIC_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Unholy Knowledge',
    skillLine: 'DarkMagic',
    esoClass: 'Sorcerer',
    bonuses: [], // Reduced cost - not relevant for damage
  },
  {
    name: 'Blood Magic',
    skillLine: 'DarkMagic',
    esoClass: 'Sorcerer',
    bonuses: [], // Healing on hit - not relevant for damage
  },
  {
    name: 'Persistence',
    skillLine: 'DarkMagic',
    esoClass: 'Sorcerer',
    bonuses: [], // Reduced cost - not relevant for damage
  },
  {
    name: 'Exploitation',
    skillLine: 'DarkMagic',
    esoClass: 'Sorcerer',
    bonuses: [
      {
        type: 'critical-chance',
        value: 0.03,
        multiplier: 'abilitySlottedCount',
      },
    ],
  },
];
