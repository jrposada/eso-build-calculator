import { WeaponSkill } from '../../../models/skill';

const BOW_SKILLS: WeaponSkill<'Bow'>[] = [
  // Ultimate abilities - Rapid Fire line
  {
    name: 'Rapid Fire',
    baseSkillName: 'Rapid Fire',
    skillLine: 'Bow',
    damage: {
      dots: [{ value: 17415, duration: 4 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'ultimate',
    channelTime: 4,
  },
  {
    name: 'Ballista',
    baseSkillName: 'Rapid Fire',
    skillLine: 'Bow',
    damage: {
      dots: [{ value: 15587, duration: 5 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'ultimate',
  },
  {
    name: 'Toxic Barrage',
    baseSkillName: 'Rapid Fire',
    skillLine: 'Bow',
    damage: {
      dots: [{ value: 17415, duration: 4 }],
    },
    damageType: 'poison',
    targetType: 'single',
    resource: 'ultimate',
    channelTime: 4,
  },
  // Snipe line
  {
    name: 'Snipe',
    baseSkillName: 'Snipe',
    skillLine: 'Bow',
    damage: {
      hits: [{ value: 2404 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Focused Aim',
    baseSkillName: 'Snipe',
    skillLine: 'Bow',
    damage: {
      hits: [{ value: 2404 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Lethal Arrow',
    baseSkillName: 'Snipe',
    skillLine: 'Bow',
    damage: {
      hits: [{ value: 2483 }],
    },
    damageType: 'poison',
    targetType: 'single',
    resource: 'stamina',
  },
  // Volley line
  {
    name: 'Volley',
    baseSkillName: 'Volley',
    skillLine: 'Bow',
    damage: {
      dots: [{ value: 342, duration: 8, delay: 2, interval: 1 }],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: 'Arrow Barrage',
    baseSkillName: 'Volley',
    skillLine: 'Bow',
    damage: {
      dots: [{ value: 460, duration: 8, delay: 2, interval: 1 }],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: 'Endless Hail',
    baseSkillName: 'Volley',
    skillLine: 'Bow',
    damage: {
      dots: [{ value: 343, duration: 13, delay: 2, interval: 1 }],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: 'Endless Hail*',
    baseSkillName: 'Thunderous Volley',
    skillLine: 'Bow',
    damage: {
      dots: [
        { value: 343, duration: 13, delay: 2, interval: 1 },
        {
          value: 526,
          duration: 13,
          delay: 2,
          interval: 1,
          flatIncreasePerTick: 191,
          ignoresModifier: true,
        },
      ],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  // Scatter Shot line
  {
    name: 'Scatter Shot',
    baseSkillName: 'Scatter Shot',
    skillLine: 'Bow',
    damage: {
      hits: [{ value: 1392 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Draining Shot',
    baseSkillName: 'Scatter Shot',
    skillLine: 'Bow',
    damage: {
      hits: [{ value: 1393 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Magnum Shot',
    baseSkillName: 'Scatter Shot',
    skillLine: 'Bow',
    damage: {
      hits: [{ value: 1727 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  // Arrow Spray line
  {
    name: 'Arrow Spray',
    baseSkillName: 'Arrow Spray',
    skillLine: 'Bow',
    damage: {
      hits: [{ value: 1742 }],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: 'Acid Spray',
    baseSkillName: 'Arrow Spray',
    skillLine: 'Bow',
    damage: {
      hits: [{ value: 1742 }],
      dots: [{ value: 1635, duration: 5 }],
    },
    damageType: 'poison',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: 'Bombard',
    baseSkillName: 'Arrow Spray',
    skillLine: 'Bow',
    damage: {
      hits: [{ value: 1742 }],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  // Poison Arrow line
  {
    name: 'Poison Arrow',
    baseSkillName: 'Poison Arrow',
    skillLine: 'Bow',
    damage: {
      hits: [{ value: 1161 }],
      dots: [{ value: 3470, duration: 20 }],
    },
    damageType: 'poison',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Poison Injection',
    baseSkillName: 'Poison Arrow',
    skillLine: 'Bow',
    damage: {
      hits: [{ value: 1161 }],
      dots: [{ value: 3470, duration: 20 }],
    },
    damageType: 'poison',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Venom Arrow',
    baseSkillName: 'Poison Arrow',
    skillLine: 'Bow',
    damage: {
      hits: [{ value: 1161 }],
      dots: [{ value: 3470, duration: 20 }],
    },
    damageType: 'poison',
    targetType: 'single',
    resource: 'stamina',
  },
];

export { BOW_SKILLS };
