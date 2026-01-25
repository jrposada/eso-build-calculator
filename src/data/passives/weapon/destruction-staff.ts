import { WeaponPassiveSkill } from '../../../models/passive';

export const DESTRUCTION_STAFF_PASSIVES: WeaponPassiveSkill[] = [
  {
    name: 'Tri Focus',
    skillLine: 'DestructionStaff',
    bonuses: [
      {
        type: 'damageType',
        value: 0.1,
        damageTypes: ['flame', 'frost', 'shock'],
      },
    ],
  },
  {
    name: 'Penetrating Magic',
    skillLine: 'DestructionStaff',
    bonuses: [{ type: 'skillLine', value: 0.08 }],
  },
  {
    name: 'Elemental Force',
    skillLine: 'DestructionStaff',
    bonuses: [
      {
        type: 'statusEffect',
        value: 0.3,
        statusEffects: ['Burning', 'Chilled', 'Concussed'],
      },
    ],
  },
  {
    name: 'Ancient Knowledge',
    skillLine: 'DestructionStaff',
    bonuses: [{ type: 'dot', value: 0.08 }],
  },
];
