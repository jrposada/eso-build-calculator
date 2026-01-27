import { TemplarSkill } from './types';

const AEDRIC_SPEAR_SKILLS: TemplarSkill<'AedricSpear'>[] = [
  // Ultimate abilities - Radial Sweep line
  {
    name: 'Radial Sweep',
    baseSkillName: 'Radial Sweep',
    className: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 2323 }],
      dots: [{ value: 1161, duration: 6, interval: 2 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Crescent Sweep',
    baseSkillName: 'Radial Sweep',
    className: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 2399 }], // +60% to enemies in front
      dots: [{ value: 1161, duration: 6, interval: 2 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Everlasting Sweep',
    baseSkillName: 'Radial Sweep',
    className: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 2399 }],
      dots: [{ value: 1161, duration: 10, interval: 2 }], // +2s per enemy hit
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  // Puncturing Strikes line
  {
    name: 'Puncturing Strikes',
    baseSkillName: 'Puncturing Strikes',
    className: 'Templar',
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
    baseSkillName: 'Puncturing Strikes',
    className: 'Templar',
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
    baseSkillName: 'Puncturing Strikes',
    className: 'Templar',
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
    baseSkillName: 'Piercing Javelin',
    className: 'Templar',
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
    baseSkillName: 'Piercing Javelin',
    className: 'Templar',
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
    baseSkillName: 'Piercing Javelin',
    className: 'Templar',
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
    baseSkillName: 'Focused Charge',
    className: 'Templar',
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
    baseSkillName: 'Focused Charge',
    className: 'Templar',
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
    baseSkillName: 'Focused Charge',
    className: 'Templar',
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
    baseSkillName: 'Spear Shards',
    className: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 1742 }],
      dots: [{ value: 166, duration: 10, interval: 1 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Blazing Spear',
    baseSkillName: 'Spear Shards',
    className: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 1742 }],
      dots: [{ value: 276, duration: 10, interval: 1 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Luminous Shards',
    baseSkillName: 'Spear Shards',
    className: 'Templar',
    skillLine: 'AedricSpear',
    damage: {
      hits: [{ value: 1742 }],
      dots: [{ value: 165, duration: 10, interval: 1 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Sun Shield line
  {
    name: 'Sun Shield',
    baseSkillName: 'Sun Shield',
    className: 'Templar',
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
    baseSkillName: 'Sun Shield',
    className: 'Templar',
    skillLine: 'AedricSpear',
    damage: {}, // No initial damage, explodes when shield expires
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Radiant Ward',
    baseSkillName: 'Sun Shield',
    className: 'Templar',
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
