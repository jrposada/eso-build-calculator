import { TemplarPassive } from './types';

export const AEDRIC_SPEAR_PASSIVES: TemplarPassive<'AedricSpear'>[] = [
  {
    name: 'Piercing Spear',
    skillLine: 'AedricSpear',
    className: 'Templar',
    bonuses: [
      {
        className: 'skill-line',
        type: 'critical-damage',
        value: 0.1,
      },
    ],
  },
  {
    name: 'Spear Wall',
    skillLine: 'AedricSpear',
    className: 'Templar',
    bonuses: [], // Block cost reduction - not relevant for damage
  },
  {
    name: 'Burning Light',
    skillLine: 'AedricSpear',
    className: 'Templar',
    bonuses: [], // Proc damage on hit - complex to model, not tracked
  },
  {
    name: 'Balanced Warrior',
    skillLine: 'AedricSpear',
    className: 'Templar',
    bonuses: [], // Weapon/Spell damage + armor - not tracked in stat-based system
  },
];
