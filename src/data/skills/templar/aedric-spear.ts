import { ClassSkill } from '../../../models/skill';

const AEDRIC_SPEAR_SKILLS: ClassSkill<'Templar', 'AedricSpear'>[] = [
  // Ultimate abilities - Radial Sweep line
  {
    name: 'Radial Sweep',
    esoClass: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 2323 }],
      dot: 1161,
      dotDuration: 6,
      dotInterval: 2,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Crescent Sweep',
    esoClass: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 2399 }], // +60% to enemies in front
      dot: 1161,
      dotDuration: 6,
      dotInterval: 2,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Everlasting Sweep',
    esoClass: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 2399 }],
      dot: 1161,
      dotDuration: 10, // +2s per enemy hit
      dotInterval: 2,
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  // Puncturing Strikes line
  {
    name: 'Puncturing Strikes',
    esoClass: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 889 * 3 }], // 3 strikes
    },
    channelTime: 0.8,
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Biting Jabs',
    esoClass: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 919 * 3 }], // 3 strikes
    },
    channelTime: 0.8,
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: 'Puncturing Sweep',
    esoClass: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 919 * 3 }], // 3 strikes
    },
    channelTime: 0.8,
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Piercing Javelin line
  {
    name: 'Piercing Javelin',
    esoClass: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 1392 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Aurora Javelin',
    esoClass: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 1438 }], // +2% per meter, up to 40%
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Binding Javelin',
    esoClass: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 1393 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  // Focused Charge line
  {
    name: 'Focused Charge',
    esoClass: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 1392 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Explosive Charge',
    esoClass: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 1799 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Toppling Charge',
    esoClass: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 1393 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Spear Shards line
  {
    name: 'Spear Shards',
    esoClass: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 1742 }],
      dot: 166,
      dotDuration: 10,
      dotInterval: 1,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Blazing Spear',
    esoClass: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 1742 }],
      dot: 276,
      dotDuration: 10,
      dotInterval: 1,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Luminous Shards',
    esoClass: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 1742 }],
      dot: 165,
      dotDuration: 10,
      dotInterval: 1,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Sun Shield line
  {
    name: 'Sun Shield',
    esoClass: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 1742 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Blazing Shield',
    esoClass: 'Templar',
    skillLine: 'AedricSpear',
    damage: {}, // No initial damage, explodes when shield expires
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Radiant Ward',
    esoClass: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 1742 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
];

export { AEDRIC_SPEAR_SKILLS };
