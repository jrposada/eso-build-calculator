import { WeaponPassiveSkill } from '../../../models/passive';

export const DESTRUCTION_STAFF_PASSIVES: WeaponPassiveSkill[] = [
  {
    name: 'Tri Focus',
    skillLine: 'DestructionStaff',
    bonuses: [], // Heavy attack effects - not tracked
  },
  {
    name: 'Penetrating Magic',
    skillLine: 'DestructionStaff',
    bonuses: [], // Penetration - handled by base skill damage
  },
  {
    name: 'Elemental Force',
    skillLine: 'DestructionStaff',
    bonuses: [], // Status effect chance - not tracked in stat-based system
  },
  {
    name: 'Ancient Knowledge',
    skillLine: 'DestructionStaff',
    bonuses: [
      {
        type: 'critical-chance',
        value: 0.04,
        multiplier: 'abilitySlottedCount',
      },
    ],
  },
];
