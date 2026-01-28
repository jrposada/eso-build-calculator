import { SorcererPassive } from './types';

export const DAEDRIC_SUMMONING_PASSIVES: SorcererPassive<'DaedricSummoning'>[] =
  [
    {
      name: 'Rebate',
      skillLine: 'DaedricSummoning',
      className: 'Sorcerer',
      bonuses: [], // Magicka return on pet death - not relevant for damage
    },
    {
      name: 'Power Stone',
      skillLine: 'DaedricSummoning',
      className: 'Sorcerer',
      bonuses: [], // Ultimate cost reduction - not relevant for damage
    },
    {
      name: 'Daedric Protection',
      skillLine: 'DaedricSummoning',
      className: 'Sorcerer',
      bonuses: [], // Health/Magicka recovery - not relevant for damage
    },
    {
      name: 'Expert Summoner',
      skillLine: 'DaedricSummoning',
      className: 'Sorcerer',
      bonuses: [
        {
          className: 'skill-line',
          type: 'max-magicka',
          value: 0.08,
        },
      ],
    },
  ];
