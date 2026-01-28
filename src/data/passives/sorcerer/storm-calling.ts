import { SorcererPassive } from './types';

export const STORM_CALLING_PASSIVES: SorcererPassive<'StormCalling'>[] = [
  {
    name: 'Capacitor',
    skillLine: 'StormCalling',
    className: 'Sorcerer',
    bonuses: [], // Magicka recovery - not relevant for damage
  },
  {
    name: 'Energized',
    skillLine: 'StormCalling',
    className: 'Sorcerer',
    bonuses: [], // Shock/Physical damage done - handled by base skill damage
  },
  {
    name: 'Amplitude',
    skillLine: 'StormCalling',
    className: 'Sorcerer',
    bonuses: [], // Damage based on missing Magicka - situational, not tracked
  },
  {
    name: 'Expert Mage',
    skillLine: 'StormCalling',
    className: 'Sorcerer',
    bonuses: [
      {
        type: 'critical-damage',
        value: 0.05,
        multiplier: 'abilitySlottedCount',
      },
    ],
  },
];
