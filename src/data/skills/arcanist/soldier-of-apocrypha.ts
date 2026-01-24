import { ClassSkill } from '../../../models/skill';

const SOLDIER_OF_APOCRYPHA_SKILLS: ClassSkill<'Arcanist', 'SoldierOfApocrypha'>[] = [
  // Ultimate abilities - Gibbering Shield line
  {
    name: 'Gibbering Shield',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {}, // Damage scales off absorbed damage
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Gibbering Shelter',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {}, // No longer deals damage
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Sanctum of the Abyssal Sea',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {}, // Damage scales off absorbed damage
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  // Runic Jolt line
  {
    name: 'Runic Jolt',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {
      hits: [{ value: 1161 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Runic Embrace',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {
      hits: [{ value: 1161 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Runic Sunder',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {
      hits: [{ value: 1161 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  // Runespite Ward line
  {
    name: 'Runespite Ward',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {}, // Retaliation damage scales off Armor
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Impervious Runeward',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {}, // Retaliation damage scales off Armor
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Spiteward of the Lucid Mind',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {}, // Retaliation damage scales off Armor
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Fatewoven Armor line
  {
    name: 'Fatewoven Armor',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Cruxweaver Armor',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Unbreakable Fate',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Runic Defense line
  {
    name: 'Runic Defense',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {},
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Runeguard of Freedom',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {},
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Runeguard of Still Waters',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {},
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Rune of Eldritch Horror line
  {
    name: 'Rune of Eldritch Horror',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Rune of Uncanny Adoration',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Rune of the Colorless Pool',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
];

export { SOLDIER_OF_APOCRYPHA_SKILLS };
