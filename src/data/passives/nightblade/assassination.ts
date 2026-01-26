import { ClassPassiveSkill } from '../../../models/passive';

export const ASSASSINATION_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Master Assassin',
    skillLine: 'Assassination',
    esoClass: 'Nightblade',
    bonuses: [
      { type: 'critical-chance', value: 0.066, multiplier: 'skillLine' },
    ],
  },
  {
    name: 'Executioner',
    skillLine: 'Assassination',
    esoClass: 'Nightblade',
    bonuses: [], // Non relevant
  },
  {
    name: 'Pressure Point',
    skillLine: 'Assassination',
    esoClass: 'Nightblade',
    bonuses: [
      {
        type: 'critical-chance',
        value: 0.025,
        multiplier: 'abilitySlottedCount',
      },
    ],
  },
  {
    name: 'Hemorrhage',
    skillLine: 'Assassination',
    esoClass: 'Nightblade',
    bonuses: [
      { type: 'critical-damage', value: 0.1, multiplier: 'abilitySlotted' },
      { buffId: 'Minor Savagery', multiplier: 'abilitySlotted' },
    ],
  },
];
