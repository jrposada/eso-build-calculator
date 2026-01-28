import { NightbladePassive } from './types';

export const ASSASSINATION_PASSIVES: NightbladePassive<'Assassination'>[] = [
  {
    name: 'Master Assassin',
    skillLine: 'Assassination',
    className: 'Nightblade',
    bonuses: [
      { type: 'critical-chance', value: 0.066, multiplier: 'skillLine' },
    ],
  },
  {
    name: 'Executioner',
    skillLine: 'Assassination',
    className: 'Nightblade',
    bonuses: [], // Non relevant
  },
  {
    name: 'Pressure Point',
    skillLine: 'Assassination',
    className: 'Nightblade',
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
    className: 'Nightblade',
    bonuses: [
      { type: 'critical-damage', value: 0.1, multiplier: 'abilitySlotted' },
      { buffId: 'Minor Savagery', multiplier: 'abilitySlotted' },
    ],
  },
];
