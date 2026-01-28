import { SorcererPassive } from './types';

export const DARK_MAGIC_PASSIVES: SorcererPassive<'DarkMagic'>[] = [
  {
    name: 'Unholy Knowledge',
    skillLine: 'DarkMagic',
    className: 'Sorcerer',
    bonuses: [], // Reduced cost - not relevant for damage
  },
  {
    name: 'Blood Magic',
    skillLine: 'DarkMagic',
    className: 'Sorcerer',
    bonuses: [], // Healing on hit - not relevant for damage
  },
  {
    name: 'Persistence',
    skillLine: 'DarkMagic',
    className: 'Sorcerer',
    bonuses: [], // Reduced cost - not relevant for damage
  },
  {
    name: 'Exploitation',
    skillLine: 'DarkMagic',
    className: 'Sorcerer',
    bonuses: [
      {
        type: 'critical-chance',
        value: 0.03,
        multiplier: 'abilitySlottedCount',
      },
    ],
  },
];
