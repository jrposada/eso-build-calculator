import { ClassSkill } from '../../../models/skill';

const DAWNS_WRATH_SKILLS: ClassSkill<'Templar', 'DawnsWrath'>[] = [
  // Ultimate abilities - Nova line
  {
    name: 'Nova',
    baseSkillName: 'Nova',
    esoClass: 'Templar',
    skillLine: 'DawnsWrath',
    damage: {
      dots: [{ value: 1161, duration: 8, interval: 1 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Solar Disturbance',
    baseSkillName: 'Nova',
    esoClass: 'Templar',
    skillLine: 'DawnsWrath',
    damage: {
      dots: [{ value: 1161, duration: 8, interval: 1 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Solar Prison',
    baseSkillName: 'Nova',
    esoClass: 'Templar',
    skillLine: 'DawnsWrath',
    damage: {
      dots: [{ value: 1199, duration: 8, interval: 1 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  // Sun Fire line
  {
    name: 'Sun Fire',
    baseSkillName: 'Sun Fire',
    esoClass: 'Templar',
    skillLine: 'DawnsWrath',
    damage: {
      hits: [{ value: 1161 }],
      dots: [{ value: 3470, duration: 20 }],
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Reflective Light',
    baseSkillName: 'Sun Fire',
    esoClass: 'Templar',
    skillLine: 'DawnsWrath',
    damage: {
      hits: [{ value: 1199 }],
      dots: [{ value: 3470, duration: 20 }],
    },
    damageType: 'flame',
    targetType: 'aoe', // hits up to 3 enemies
    resource: 'magicka',
  },
  {
    name: "Vampire's Bane",
    baseSkillName: 'Sun Fire',
    esoClass: 'Templar',
    skillLine: 'DawnsWrath',
    damage: {
      hits: [{ value: 1161 }],
      dots: [{ value: 5370, duration: 30 }],
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  // Solar Flare line
  {
    name: 'Solar Flare', // Increase class abilities damage by 5% for 10s
    baseSkillName: 'Solar Flare',
    esoClass: 'Templar',
    skillLine: 'DawnsWrath',
    damage: {
      hits: [{ value: 2404 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Dark Flare', // Increase class abilities damage by 5% for 10s
    baseSkillName: 'Solar Flare',
    esoClass: 'Templar',
    skillLine: 'DawnsWrath',
    damage: {
      hits: [{ value: 2483 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Solar Barrage', // Increase class abilities damage by 5% for 20s
    baseSkillName: 'Solar Flare',
    esoClass: 'Templar',
    skillLine: 'DawnsWrath',
    damage: {
      dots: [{ value: 435, duration: 20, interval: 2 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Backlash line
  {
    name: 'Backlash',
    baseSkillName: 'Backlash',
    esoClass: 'Templar',
    skillLine: 'DawnsWrath',
    damage: {
      hits: [
        { value: 1161 },
        { value: 1284, delay: 6 }, // burst damage after 6s, up to 200% more
      ],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Power of the Light',
    baseSkillName: 'Backlash',
    esoClass: 'Templar',
    skillLine: 'DawnsWrath',
    damage: {
      hits: [
        { value: 1161 },
        { value: 1285, delay: 6 }, // burst damage after 6s, up to 200% more
      ],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Purifying Light',
    baseSkillName: 'Backlash',
    esoClass: 'Templar',
    skillLine: 'DawnsWrath',
    damage: {
      hits: [
        { value: 1161 },
        { value: 1285, delay: 6 }, // burst damage after 6s, up to 200% more
      ],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Eclipse line
  {
    name: 'Eclipse',
    baseSkillName: 'Eclipse',
    esoClass: 'Templar',
    skillLine: 'DawnsWrath',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Living Dark',
    baseSkillName: 'Eclipse',
    esoClass: 'Templar',
    skillLine: 'DawnsWrath',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Unstable Core',
    baseSkillName: 'Eclipse',
    esoClass: 'Templar',
    skillLine: 'DawnsWrath',
    damage: {
      hits: [{ value: 449 + 898 + 1799 }], // max damage if all 3 triggers hit
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Radiant Destruction line
  {
    name: 'Radiant Destruction',
    baseSkillName: 'Radiant Destruction',
    esoClass: 'Templar',
    skillLine: 'DawnsWrath',
    damage: {
      hits: [{ value: 7248 }], // up to 500% more vs enemies below 33% HP
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
    channelTime: 3.8,
  },
  {
    name: 'Radiant Glory',
    baseSkillName: 'Radiant Destruction',
    esoClass: 'Templar',
    skillLine: 'DawnsWrath',
    damage: {
      hits: [{ value: 7482 }], // up to 500% more vs enemies below 33% HP
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
    channelTime: 3.8,
  },
  {
    name: 'Radiant Oppression',
    baseSkillName: 'Radiant Destruction',
    esoClass: 'Templar',
    skillLine: 'DawnsWrath',
    damage: {
      hits: [{ value: 7482 }], // up to 500% more vs enemies below 40% HP
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
    channelTime: 3.8,
  },
];

export { DAWNS_WRATH_SKILLS };
