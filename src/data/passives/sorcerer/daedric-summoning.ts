import { ClassPassiveSkill } from '../../../models/passive';

export const DAEDRIC_SUMMONING_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Rebate',
    skillLine: 'DaedricSummoning',
    esoClass: 'Sorcerer',
    bonuses: [], // Magicka return on pet death - not relevant for damage
  },
  {
    name: 'Power Stone',
    skillLine: 'DaedricSummoning',
    esoClass: 'Sorcerer',
    bonuses: [], // Ultimate cost reduction - not relevant for damage
  },
  {
    name: 'Daedric Protection',
    skillLine: 'DaedricSummoning',
    esoClass: 'Sorcerer',
    bonuses: [], // Health/Magicka recovery - not relevant for damage
  },
  {
    name: 'Expert Summoner',
    skillLine: 'DaedricSummoning',
    esoClass: 'Sorcerer',
    bonuses: [{ type: 'max-magicka', value: 0.08, multiplier: 'skillLine' }],
  },
];
