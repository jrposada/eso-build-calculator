import { ClassSkill } from '../../../models/skill';

const EARTHEN_HEART_SKILLS: ClassSkill<'Dragonknight', 'EarthenHeart'>[] = [
  // Ultimate abilities - Magma Armor line
  {
    name: 'Magma Armor',
    baseSkillName: 'Magma Armor',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {
      dots: [{ value: 336, duration: 10, interval: 1 }],
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Corrosive Armor',
    baseSkillName: 'Magma Armor',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {
      dots: [{ value: 347, duration: 10, interval: 1 }],
    },
    damageType: 'poison',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Magma Shell',
    baseSkillName: 'Magma Armor',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {
      dots: [{ value: 347, duration: 10, interval: 1 }],
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  // Stonefist line
  {
    name: 'Stonefist',
    baseSkillName: 'Stonefist',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {
      hits: [{ value: 2323 }], // initial AoE + 3x2323 projectile damage
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: 'Obsidian Shard',
    baseSkillName: 'Stonefist',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {
      hits: [{ value: 448 }],
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Stone Giant',
    baseSkillName: 'Stonefist',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {
      hits: [{ value: 2323 }], // initial AoE + 3x2323 projectile damage
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  // Molten Weapons line
  {
    name: 'Molten Weapons',
    baseSkillName: 'Molten Weapons',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {},
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Igneous Weapons',
    baseSkillName: 'Molten Weapons',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {},
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Molten Armaments',
    baseSkillName: 'Molten Weapons',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {},
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Obsidian Shield line
  {
    name: 'Obsidian Shield',
    baseSkillName: 'Obsidian Shield',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {},
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Fragmented Shield',
    baseSkillName: 'Obsidian Shield',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {},
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Igneous Shield',
    baseSkillName: 'Obsidian Shield',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {},
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Petrify line
  {
    name: 'Petrify',
    baseSkillName: 'Petrify',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {
      hits: [{ value: 1161, delay: 2.5 }], // damage when stun ends
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Fossilize',
    baseSkillName: 'Petrify',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {
      hits: [{ value: 1199, delay: 2.5 }], // damage when stun ends
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Shattering Rocks',
    baseSkillName: 'Petrify',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {
      hits: [{ value: 1199, delay: 2.5 }], // damage when stun ends
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  // Ash Cloud line
  {
    name: 'Ash Cloud',
    baseSkillName: 'Ash Cloud',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {},
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Cinder Storm',
    baseSkillName: 'Ash Cloud',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {},
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Eruption',
    baseSkillName: 'Ash Cloud',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {
      hits: [{ value: 1799 }],
      dots: [{ value: 319, duration: 15, interval: 1 }],
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
  },
];

export { EARTHEN_HEART_SKILLS };
