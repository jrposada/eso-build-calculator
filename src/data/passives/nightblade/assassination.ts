import { MINOR_SAVAGERY } from '../../bonuses/buff/buff';
import { NightbladePassive } from './types';

export const ASSASSINATION_PASSIVES: NightbladePassive<'Assassination'>[] = [
  {
    name: 'Master Assassin',
    skillLine: 'Assassination',
    className: 'Nightblade',
    bonuses: [
      {
        className: 'skill-line',
        type: 'critical-chance',
        value: 0.066,
      },
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
        className: 'ability-slotted-count',
        type: 'critical-chance',
        value: 0.025,
      },
    ],
  },
  {
    name: 'Hemorrhage',
    skillLine: 'Assassination',
    className: 'Nightblade',
    bonuses: [
      {
        className: 'ability-slotted',
        type: 'critical-damage',
        value: 0.1,
      },
      MINOR_SAVAGERY, // REVIEW duration
    ],
  },
];
