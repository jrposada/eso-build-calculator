import { Skill } from '../../../models/skill';

const WINTERS_EMBRACE_SKILLS: Skill<'Warden', 'WintersEmbrace'>[] = [
  // Ultimate abilities - Sleet Storm line
  {
    name: 'Sleet Storm',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {
      dot: 1161,
      dotDuration: 8,
      dotInterval: 1,
    },
    damageType: 'frost',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Northern Storm',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {
      dot: 1199,
      dotDuration: 8,
      dotInterval: 1,
    },
    damageType: 'frost',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Permafrost',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {
      dot: 158,
      dotDuration: 13,
      dotInterval: 1,
    },
    damageType: 'frost',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  // Frost Cloak line
  {
    name: 'Frost Cloak',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {},
    damageType: 'frost',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Expansive Frost Cloak',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {},
    damageType: 'frost',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Ice Fortress',
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
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {
      dot: 405, // Based on your Max Health
      dotDuration: 12,
      dotInterval: 1,
    },
    damageType: 'frost',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Gripping Shards',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {
      dot: 419, // Based on your Max Health
      dotDuration: 12,
      dotInterval: 1,
    },
    damageType: 'frost',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: "Winter's Revenge",
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {
      dot: 294, // Based on your Max Health
      dotDuration: 12,
      dotInterval: 1,
    },
    damageType: 'frost',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Arctic Wind line
  {
    name: 'Arctic Wind',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {},
    damageType: 'frost',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Arctic Blast',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {
      hits: [{ value: 1799 }],
      dot: 298,
      dotDuration: 20,
      dotInterval: 2,
    },
    damageType: 'frost',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Polar Wind',
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
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {},
    damageType: 'frost',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Crystallized Slab',
    esoClass: 'Warden',
    skillLine: 'WintersEmbrace',
    damage: {}, // 1199 on projectile absorb - conditional damage
    damageType: 'frost',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Shimmering Shield',
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
