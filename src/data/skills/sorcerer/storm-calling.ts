import { Skill } from '../../../models/skill';

const STORM_CALLING_SKILLS: Skill<'Sorcerer', 'StormCalling'>[] = [
  // Ultimate abilities - Overload line
  {
    name: 'Overload',
    esoClass: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {
      hits: [{ value: 2323 }],
    },
    damageType: 'shock',
    targetType: 'single',
    resource: 'ultimate',
  },
  {
    name: 'Energy Overload',
    esoClass: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {
      hits: [{ value: 2399 }],
    },
    damageType: 'shock',
    targetType: 'single',
    resource: 'ultimate',
  },
  {
    name: 'Power Overload',
    esoClass: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {
      hits: [{ value: 2640 }],
    },
    damageType: 'shock',
    targetType: 'single',
    resource: 'ultimate',
  },
  // Mages' Fury line
  {
    name: "Mages' Fury",
    esoClass: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {
      hits: [{ value: 870 }],
    },
    damageType: 'shock',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Endless Fury',
    esoClass: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {
      hits: [{ value: 871 }],
    },
    damageType: 'shock',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: "Mages' Wrath",
    esoClass: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {
      hits: [{ value: 871 }],
    },
    damageType: 'shock',
    targetType: 'single',
    resource: 'magicka',
  },
  // Lightning Form line
  {
    name: 'Lightning Form',
    esoClass: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {
      dot: 462,
      dotDuration: 20,
      dotInterval: 2,
    },
    damageType: 'shock',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Boundless Storm',
    esoClass: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {
      dot: 463,
      dotDuration: 30,
      dotInterval: 2,
    },
    damageType: 'shock',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Hurricane',
    esoClass: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {
      dot: 478,
      dotDuration: 20,
      dotInterval: 2,
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  // Lightning Splash line
  {
    name: 'Lightning Splash',
    esoClass: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {
      dot: 308,
      dotDuration: 10,
      dotInterval: 1,
    },
    damageType: 'shock',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Lightning Flood',
    esoClass: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {
      dot: 415,
      dotDuration: 10,
      dotInterval: 1,
    },
    damageType: 'shock',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Liquid Lightning',
    esoClass: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {
      dot: 309,
      dotDuration: 15,
      dotInterval: 1,
    },
    damageType: 'shock',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Surge line
  {
    name: 'Surge',
    esoClass: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {},
    damageType: 'shock',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Critical Surge',
    esoClass: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {},
    damageType: 'shock',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Power Surge',
    esoClass: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {},
    damageType: 'shock',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Bolt Escape line
  {
    name: 'Bolt Escape',
    esoClass: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {},
    damageType: 'shock',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Ball of Lightning',
    esoClass: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {},
    damageType: 'shock',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Streak',
    esoClass: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {
      hits: [{ value: 1438 }],
    },
    damageType: 'shock',
    targetType: 'aoe',
    resource: 'magicka',
  },
];

export { STORM_CALLING_SKILLS };
