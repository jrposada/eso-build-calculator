import { SorcererSkill } from './types';

const STORM_CALLING_SKILLS: SorcererSkill<'StormCalling'>[] = [
  // Ultimate abilities - Overload line
  {
    name: 'Overload',
    baseSkillName: 'Overload',
    className: 'Sorcerer',
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
    baseSkillName: 'Overload',
    className: 'Sorcerer',
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
    baseSkillName: 'Overload',
    className: 'Sorcerer',
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
    baseSkillName: "Mages' Fury",
    className: 'Sorcerer',
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
    baseSkillName: "Mages' Fury",
    className: 'Sorcerer',
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
    baseSkillName: "Mages' Fury",
    className: 'Sorcerer',
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
    baseSkillName: 'Lightning Form',
    className: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {
      dots: [{ value: 462, duration: 20, interval: 2 }],
    },
    damageType: 'shock',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Boundless Storm',
    baseSkillName: 'Lightning Form',
    className: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {
      dots: [{ value: 463, duration: 30, interval: 2 }],
    },
    damageType: 'shock',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Hurricane',
    baseSkillName: 'Lightning Form',
    className: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {
      dots: [{ value: 478, duration: 20, interval: 2, increasePerTick: 0.12 }],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  // Lightning Splash line
  {
    name: 'Lightning Splash',
    baseSkillName: 'Lightning Splash',
    className: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {
      dots: [{ value: 308, duration: 10, interval: 1 }],
    },
    damageType: 'shock',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Lightning Flood',
    baseSkillName: 'Lightning Splash',
    className: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {
      dots: [{ value: 415, duration: 10, interval: 1 }],
    },
    damageType: 'shock',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Liquid Lightning',
    baseSkillName: 'Lightning Splash',
    className: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {
      dots: [{ value: 309, duration: 15, interval: 1 }],
    },
    damageType: 'shock',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Surge line
  {
    name: 'Surge',
    baseSkillName: 'Surge',
    className: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {},
    damageType: 'shock',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Critical Surge',
    baseSkillName: 'Surge',
    className: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {},
    damageType: 'shock',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Power Surge',
    baseSkillName: 'Surge',
    className: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {},
    damageType: 'shock',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Bolt Escape line
  {
    name: 'Bolt Escape',
    baseSkillName: 'Bolt Escape',
    className: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {},
    damageType: 'shock',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Ball of Lightning',
    baseSkillName: 'Bolt Escape',
    className: 'Sorcerer',
    skillLine: 'StormCalling',
    damage: {},
    damageType: 'shock',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Streak',
    baseSkillName: 'Bolt Escape',
    className: 'Sorcerer',
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
