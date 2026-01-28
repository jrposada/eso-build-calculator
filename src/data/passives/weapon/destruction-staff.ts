import { WeaponPassive } from './types';

export const DESTRUCTION_STAFF_PASSIVES: WeaponPassive<'DestructionStaff'>[] = [
  {
    name: 'Tri Focus',
    className: 'Weapon',
    skillLine: 'DestructionStaff',
    bonuses: [], // Heavy attack effects - not tracked
  },
  {
    name: 'Penetrating Magic',
    className: 'Weapon',
    skillLine: 'DestructionStaff',
    bonuses: [], // Penetration - handled by base skill damage
  },
  {
    name: 'Elemental Force',
    className: 'Weapon',
    skillLine: 'DestructionStaff',
    bonuses: [], // Status effect chance - not tracked in stat-based system
  },
  {
    name: 'Ancient Knowledge',
    className: 'Weapon',
    skillLine: 'DestructionStaff',
    bonuses: [
      {
        className: 'ability-slotted-count',
        type: 'critical-chance',
        value: 0.04,
      },
    ],
  },
];
