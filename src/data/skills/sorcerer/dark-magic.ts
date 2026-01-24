import { ClassSkill } from '../../../models/skill';

const DARK_MAGIC_SKILLS: ClassSkill<'Sorcerer', 'DarkMagic'>[] = [
  // Ultimate abilities - Negate Magic line
  {
    name: 'Negate Magic',
    esoClass: 'Sorcerer',
    skillLine: 'DarkMagic',
    damage: {},
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Absorption Field',
    esoClass: 'Sorcerer',
    skillLine: 'DarkMagic',
    damage: {},
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Suppression Field',
    esoClass: 'Sorcerer',
    skillLine: 'DarkMagic',
    damage: {
      dot: 1038,
      dotDuration: 12,
      dotInterval: 1,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  // Crystal Shard line
  {
    name: 'Crystal Shard',
    esoClass: 'Sorcerer',
    skillLine: 'DarkMagic',
    damage: {
      hits: [{ value: 2404 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Crystal Fragments',
    esoClass: 'Sorcerer',
    skillLine: 'DarkMagic',
    damage: {
      hits: [{ value: 2483 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Crystal Weapon',
    esoClass: 'Sorcerer',
    skillLine: 'DarkMagic',
    damage: {
      hits: [{ value: 2091 }, { value: 836 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  // Encase line
  {
    name: 'Encase',
    esoClass: 'Sorcerer',
    skillLine: 'DarkMagic',
    damage: {},
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Shattering Spines',
    esoClass: 'Sorcerer',
    skillLine: 'DarkMagic',
    damage: {
      hits: [{ value: 1979, delay: 4 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Vibrant Shroud',
    esoClass: 'Sorcerer',
    skillLine: 'DarkMagic',
    damage: {},
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Rune Prison line
  {
    name: 'Rune Prison',
    esoClass: 'Sorcerer',
    skillLine: 'DarkMagic',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Defensive Rune',
    esoClass: 'Sorcerer',
    skillLine: 'DarkMagic',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Rune Cage',
    esoClass: 'Sorcerer',
    skillLine: 'DarkMagic',
    damage: {
      hits: [{ value: 1799, delay: 3 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Dark Exchange line
  {
    name: 'Dark Exchange',
    esoClass: 'Sorcerer',
    skillLine: 'DarkMagic',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Dark Conversion',
    esoClass: 'Sorcerer',
    skillLine: 'DarkMagic',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Dark Deal',
    esoClass: 'Sorcerer',
    skillLine: 'DarkMagic',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Daedric Mines line
  {
    name: 'Daedric Mines',
    esoClass: 'Sorcerer',
    skillLine: 'DarkMagic',
    damage: {
      hits: [{ value: 2613, delay: 3 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Daedric Refuge',
    esoClass: 'Sorcerer',
    skillLine: 'DarkMagic',
    damage: {},
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Daedric Tomb',
    esoClass: 'Sorcerer',
    skillLine: 'DarkMagic',
    damage: {
      hits: [{ value: 2700 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
];

export { DARK_MAGIC_SKILLS };
