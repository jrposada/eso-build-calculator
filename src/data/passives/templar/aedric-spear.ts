import { ClassPassiveSkill } from '../../../models/passive';

export const AEDRIC_SPEAR_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Piercing Spear',
    skillLine: 'AedricSpear',
    esoClass: 'Templar',
    bonuses: [{ type: 'critical-damage', value: 0.1, multiplier: 'skillLine' }],
  },
  {
    name: 'Spear Wall',
    skillLine: 'AedricSpear',
    esoClass: 'Templar',
    bonuses: [], // Block cost reduction - not relevant for damage
  },
  {
    name: 'Burning Light',
    skillLine: 'AedricSpear',
    esoClass: 'Templar',
    bonuses: [], // Proc damage on hit - complex to model, not tracked
  },
  {
    name: 'Balanced Warrior',
    skillLine: 'AedricSpear',
    esoClass: 'Templar',
    bonuses: [], // Weapon/Spell damage + armor - not tracked in stat-based system
  },
];
