import { WeaponSkillData } from './types';

const DESTRUCTION_STAFF_SKILLS: WeaponSkillData<'DestructionStaff'>[] = [
  // Ultimate abilities - Elemental Storm line
  {
    name: 'Elemental Storm',
    baseSkillName: 'Elemental Storm',
    className: 'Weapon',
    skillLine: 'DestructionStaff',
    damage: {
      dots: [{ value: 1742, duration: 7, interval: 1 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Elemental Rage',
    baseSkillName: 'Elemental Storm',
    className: 'Weapon',
    skillLine: 'DestructionStaff',
    damage: {
      dots: [{ value: 2249, duration: 7, interval: 1 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Eye of the Storm',
    baseSkillName: 'Elemental Storm',
    className: 'Weapon',
    skillLine: 'DestructionStaff',
    damage: {
      dots: [{ value: 1799, duration: 7, interval: 1 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  // Force Shock line
  {
    name: 'Force Shock',
    baseSkillName: 'Force Shock',
    className: 'Weapon',
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
    baseSkillName: 'Force Shock',
    className: 'Weapon',
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
    baseSkillName: 'Force Shock',
    className: 'Weapon',
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
    baseSkillName: 'Wall of Elements',
    className: 'Weapon',
    skillLine: 'DestructionStaff',
    damage: {
      dots: [{ value: 280, duration: 8, interval: 1 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Elemental Blockade',
    baseSkillName: 'Wall of Elements',
    className: 'Weapon',
    skillLine: 'DestructionStaff',
    damage: {
      dots: [{ value: 281, duration: 10, interval: 1 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Unstable Wall of Elements',
    baseSkillName: 'Wall of Elements',
    className: 'Weapon',
    skillLine: 'DestructionStaff',
    damage: {
      dots: [{ value: 281, duration: 8, interval: 1 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Destructive Touch line
  {
    name: 'Destructive Touch',
    baseSkillName: 'Destructive Touch',
    className: 'Weapon',
    skillLine: 'DestructionStaff',
    damage: {
      hits: [{ value: 1161 }],
      dots: [{ value: 3470, duration: 20 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Destructive Clench',
    baseSkillName: 'Destructive Touch',
    className: 'Weapon',
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
    baseSkillName: 'Destructive Touch',
    className: 'Weapon',
    skillLine: 'DestructionStaff',
    damage: {
      hits: [{ value: 1161 }],
      dots: [{ value: 3470, duration: 20 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Weakness to Elements line
  {
    name: 'Weakness to Elements',
    baseSkillName: 'Weakness to Elements',
    className: 'Weapon',
    skillLine: 'DestructionStaff',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Elemental Drain',
    baseSkillName: 'Weakness to Elements',
    className: 'Weapon',
    skillLine: 'DestructionStaff',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Elemental Susceptibility',
    baseSkillName: 'Weakness to Elements',
    className: 'Weapon',
    skillLine: 'DestructionStaff',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Impulse line
  {
    name: 'Impulse',
    baseSkillName: 'Impulse',
    className: 'Weapon',
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
    baseSkillName: 'Impulse',
    className: 'Weapon',
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
    baseSkillName: 'Impulse',
    className: 'Weapon',
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
