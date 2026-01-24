import { WeaponSkill } from '../../../models/skill';

const DESTRUCTION_STAFF_SKILLS: WeaponSkill<'DestructionStaff'>[] = [
  // Ultimate abilities - Elemental Storm line
  {
    name: 'Elemental Storm',
    skillLine: 'DestructionStaff',
    damage: {
      dot: 1742,
      dotDuration: 7,
      dotInterval: 1,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Elemental Rage',
    skillLine: 'DestructionStaff',
    damage: {
      dot: 2249,
      dotDuration: 7,
      dotInterval: 1,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Eye of the Storm',
    skillLine: 'DestructionStaff',
    damage: {
      dot: 1799,
      dotDuration: 7,
      dotInterval: 1,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  // Force Shock line
  {
    name: 'Force Shock',
    skillLine: 'DestructionStaff',
    damage: {
      hits: [{ value: 2085 }], // 695 + 695 + 695
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Crushing Shock',
    skillLine: 'DestructionStaff',
    damage: {
      hits: [{ value: 2088 }], // 696 + 696 + 696
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Force Pulse',
    skillLine: 'DestructionStaff',
    damage: {
      hits: [{ value: 2088 }], // 696 + 696 + 696
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Wall of Elements line
  {
    name: 'Wall of Elements',
    skillLine: 'DestructionStaff',
    damage: {
      dot: 280,
      dotDuration: 8,
      dotInterval: 1,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Elemental Blockade',
    skillLine: 'DestructionStaff',
    damage: {
      dot: 281,
      dotDuration: 10,
      dotInterval: 1,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Unstable Wall of Elements',
    skillLine: 'DestructionStaff',
    damage: {
      dot: 281,
      dotDuration: 8,
      dotInterval: 1,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Destructive Touch line
  {
    name: 'Destructive Touch',
    skillLine: 'DestructionStaff',
    damage: {
      hits: [{ value: 1161 }],
      dot: 3470,
      dotDuration: 20,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Destructive Clench',
    skillLine: 'DestructionStaff',
    damage: {
      hits: [{ value: 1161 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Destructive Reach',
    skillLine: 'DestructionStaff',
    damage: {
      hits: [{ value: 1161 }],
      dot: 3470,
      dotDuration: 20,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Weakness to Elements line
  {
    name: 'Weakness to Elements',
    skillLine: 'DestructionStaff',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Elemental Drain',
    skillLine: 'DestructionStaff',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Elemental Susceptibility',
    skillLine: 'DestructionStaff',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Impulse line
  {
    name: 'Impulse',
    skillLine: 'DestructionStaff',
    damage: {
      hits: [{ value: 1742 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Elemental Ring',
    skillLine: 'DestructionStaff',
    damage: {
      hits: [{ value: 1799 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Pulsar',
    skillLine: 'DestructionStaff',
    damage: {
      hits: [{ value: 1742 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
];

export { DESTRUCTION_STAFF_SKILLS };
