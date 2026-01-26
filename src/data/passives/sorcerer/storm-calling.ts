import { ClassPassiveSkill } from '../../../models/passive';

export const STORM_CALLING_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Capacitor',
    skillLine: 'StormCalling',
    esoClass: 'Sorcerer',
    bonuses: [], // Magicka recovery - not relevant for damage
  },
  {
    name: 'Energized',
    skillLine: 'StormCalling',
    esoClass: 'Sorcerer',
    bonuses: [], // Shock/Physical damage done - handled by base skill damage
  },
  {
    name: 'Amplitude',
    skillLine: 'StormCalling',
    esoClass: 'Sorcerer',
    bonuses: [], // Damage based on missing Magicka - situational, not tracked
  },
  {
    name: 'Expert Mage',
    skillLine: 'StormCalling',
    esoClass: 'Sorcerer',
    bonuses: [
      {
        type: 'critical-damage',
        value: 0.05,
        multiplier: 'abilitySlottedCount',
      },
    ],
  },
];
