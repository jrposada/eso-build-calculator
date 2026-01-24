import { Skill } from '../../../models/skill';

const EARTHEN_HEART_SKILLS: Skill<'Dragonknight', 'EarthenHeart'>[] = [
  // Ultimate abilities - Magma Armor line
  {
    name: 'Magma Armor',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {
      dot: 336,
      dotDuration: 10,
      dotInterval: 1,
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Corrosive Armor',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {
      dot: 347,
      dotDuration: 10,
      dotInterval: 1,
    },
    damageType: 'poison',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Magma Shell',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {
      dot: 347,
      dotDuration: 10,
      dotInterval: 1,
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  // Stonefist line
  {
    name: 'Stonefist',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {
      initial: 2323, // initial AoE + 3x2323 projectile damage
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: 'Obsidian Shard',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {
      initial: 448,
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Stone Giant',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {
      initial: 2323, // initial AoE + 3x2323 projectile damage
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  // Molten Weapons line
  {
    name: 'Molten Weapons',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {},
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Igneous Weapons',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {},
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Molten Armaments',
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
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {},
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Fragmented Shield',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {},
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Igneous Shield',
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
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {
      initial: 1161,
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Fossilize',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {
      initial: 1199,
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Shattering Rocks',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {
      initial: 1199,
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  // Ash Cloud line
  {
    name: 'Ash Cloud',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {},
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Cinder Storm',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {},
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Eruption',
    esoClass: 'Dragonknight',
    skillLine: 'EarthenHeart',
    damage: {
      initial: 1799,
      dot: 319,
      dotDuration: 15,
      dotInterval: 1,
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
  },
];

export { EARTHEN_HEART_SKILLS };
