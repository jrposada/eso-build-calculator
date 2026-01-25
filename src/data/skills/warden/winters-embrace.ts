import { ClassSkill } from '../../../models/skill';

const WINTERS_EMBRACE_SKILLS: ClassSkill<'Warden', 'WintersEmbrace'>[] = [
  // Ultimate abilities - Sleet Storm line
  {
    name: 'Sleet Storm',
    baseSkillName: 'Sleet Storm',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {
      dots: [{ value: 1161, duration: 8, interval: 1 }],
    },
    damageType: 'frost',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Northern Storm',
    baseSkillName: 'Sleet Storm',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {
      dots: [{ value: 1199, duration: 8, interval: 1 }],
    },
    damageType: 'frost',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Permafrost',
    baseSkillName: 'Sleet Storm',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {
      dots: [{ value: 158, duration: 13, interval: 1 }],
    },
    damageType: 'frost',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  // Frost Cloak line
  {
    name: 'Frost Cloak',
    baseSkillName: 'Frost Cloak',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {},
    damageType: 'frost',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Expansive Frost Cloak',
    baseSkillName: 'Frost Cloak',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {},
    damageType: 'frost',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Ice Fortress',
    baseSkillName: 'Frost Cloak',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {},
    damageType: 'frost',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Impaling Shards line
  {
    name: 'Impaling Shards',
    baseSkillName: 'Impaling Shards',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {
      dots: [{ value: 405, duration: 12, interval: 1 }], // Based on your Max Health
    },
    damageType: 'frost',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Gripping Shards',
    baseSkillName: 'Impaling Shards',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {
      dots: [{ value: 419, duration: 12, interval: 1 }], // Based on your Max Health
    },
    damageType: 'frost',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: "Winter's Revenge",
    baseSkillName: 'Impaling Shards',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {
      dots: [{ value: 294, duration: 12, interval: 1 }], // Based on your Max Health
    },
    damageType: 'frost',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Arctic Wind line
  {
    name: 'Arctic Wind',
    baseSkillName: 'Arctic Wind',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {},
    damageType: 'frost',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Arctic Blast',
    baseSkillName: 'Arctic Wind',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {
      hits: [{ value: 1799 }],
      dots: [{ value: 298, duration: 20, interval: 2 }],
    },
    damageType: 'frost',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Polar Wind',
    baseSkillName: 'Arctic Wind',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {},
    damageType: 'frost',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Crystallized Shield line
  {
    name: 'Crystallized Shield',
    baseSkillName: 'Crystallized Shield',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {},
    damageType: 'frost',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Crystallized Slab',
    baseSkillName: 'Crystallized Shield',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {}, // 1199 on projectile absorb - conditional damage
    damageType: 'frost',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Shimmering Shield',
    baseSkillName: 'Crystallized Shield',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {},
    damageType: 'frost',
    targetType: 'single',
    resource: 'magicka',
  },
  // Frozen Gate line
  {
    name: 'Frozen Gate',
    baseSkillName: 'Frozen Gate',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {
      hits: [{ value: 1742, delay: 1.5 }], // arms after 1.5 seconds
    },
    damageType: 'frost',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Frozen Device',
    baseSkillName: 'Frozen Gate',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {
      hits: [{ value: 1799, delay: 1.5 }], // arms after 1.5 seconds
    },
    damageType: 'frost',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Frozen Retreat',
    baseSkillName: 'Frozen Gate',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {
      hits: [{ value: 1799, delay: 1.5 }], // arms after 1.5 seconds
    },
    damageType: 'frost',
    targetType: 'single',
    resource: 'magicka',
  },
];

export { WINTERS_EMBRACE_SKILLS };
