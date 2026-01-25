import { ClassSkill } from '../../../models/skill';

const SOLDIER_OF_APOCRYPHA_SKILLS: ClassSkill<'Arcanist', 'SoldierOfApocrypha'>[] = [
  // Ultimate abilities - Gibbering Shield line
  {
    name: 'Gibbering Shield',
    baseSkillName: 'Gibbering Shield',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {}, // Damage scales off absorbed damage
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Gibbering Shelter',
    baseSkillName: 'Gibbering Shield',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {}, // No longer deals damage
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Sanctum of the Abyssal Sea',
    baseSkillName: 'Gibbering Shield',
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
    baseSkillName: 'Runic Jolt',
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
    baseSkillName: 'Runic Jolt',
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
    baseSkillName: 'Runic Jolt',
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
    baseSkillName: 'Runespite Ward',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {}, // Retaliation damage scales off Armor
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Impervious Runeward',
    baseSkillName: 'Runespite Ward',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {}, // Retaliation damage scales off Armor
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Spiteward of the Lucid Mind',
    baseSkillName: 'Runespite Ward',
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
    baseSkillName: 'Fatewoven Armor',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Cruxweaver Armor',
    baseSkillName: 'Fatewoven Armor',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Unbreakable Fate',
    baseSkillName: 'Fatewoven Armor',
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
    baseSkillName: 'Runic Defense',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {},
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Runeguard of Freedom',
    baseSkillName: 'Runic Defense',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {},
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Runeguard of Still Waters',
    baseSkillName: 'Runic Defense',
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
    baseSkillName: 'Rune of Eldritch Horror',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Rune of Uncanny Adoration',
    baseSkillName: 'Rune of Eldritch Horror',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Rune of the Colorless Pool',
    baseSkillName: 'Rune of Eldritch Horror',
    esoClass: 'Arcanist',
    skillLine: 'SoldierOfApocrypha',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
];

export { SOLDIER_OF_APOCRYPHA_SKILLS };
