import { Skill } from '../../../models/skill';

const ASSASSINATION_SKILLS: Skill<'Nightblade', 'Assassination'>[] = [
  // Ultimate abilities
  {
    name: 'Death Stroke',
    esoClass: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      initial: 3716,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'ultimate',
  },
  {
    name: 'Incapacitating Strike',
    esoClass: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      initial: 3840,
    },
    damageType: 'disease',
    targetType: 'single',
    resource: 'ultimate',
  },
  {
    name: 'Soul Harvest',
    esoClass: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      initial: 3718,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'ultimate',
  },
  // Veiled Strike line
  {
    name: 'Veiled Strike',
    esoClass: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      initial: 2323,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Concealed Weapon',
    esoClass: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      initial: 2556,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Surprise Attack',
    esoClass: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      initial: 2399,
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  // Teleport Strike line
  {
    name: 'Teleport Strike',
    esoClass: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      initial: 1602,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Ambush',
    esoClass: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      initial: 1655,
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Lotus Fan',
    esoClass: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      initial: 1603,
      dot: 2050,
      dotDuration: 5,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Assassin's Blade line
  {
    name: "Assassin's Blade",
    esoClass: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      initial: 1161,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Impale',
    esoClass: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      initial: 1161,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: "Killer's Blade",
    esoClass: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      initial: 1161,
    },
    damageType: 'disease',
    targetType: 'single',
    resource: 'stamina',
  },
  // Mark Target line (utility/debuff skills)
  {
    name: 'Mark Target',
    esoClass: 'Nightblade',
    skillLine: 'Assassination',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Piercing Mark',
    esoClass: 'Nightblade',
    skillLine: 'Assassination',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: "Reaper's Mark",
    esoClass: 'Nightblade',
    skillLine: 'Assassination',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Grim Focus line
  {
    name: 'Grim Focus',
    esoClass: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      initial: 4182,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Merciless Resolve',
    esoClass: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      initial: 4752,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Relentless Focus',
    esoClass: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      initial: 4183,
    },
    damageType: 'disease',
    targetType: 'single',
    resource: 'stamina',
  },
];

export { ASSASSINATION_SKILLS };
