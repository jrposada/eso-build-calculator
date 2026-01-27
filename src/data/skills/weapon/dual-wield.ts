import { WeaponSkillData } from './types';

const DUAL_WIELD_SKILLS: WeaponSkillData<'DualWield'>[] = [
  // Ultimate abilities - Lacerate line
  {
    name: 'Lacerate',
    baseSkillName: 'Lacerate',
    className: 'Weapon',
    skillLine: 'DualWield',
    damage: {
      dots: [{ value: 6960, duration: 8 }],
    },
    damageType: 'bleed',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Rend',
    baseSkillName: 'Lacerate',
    className: 'Weapon',
    skillLine: 'DualWield',
    damage: {
      dots: [{ value: 12942, duration: 16 }],
    },
    damageType: 'bleed',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Thrive in Chaos',
    baseSkillName: 'Lacerate',
    className: 'Weapon',
    skillLine: 'DualWield',
    damage: {
      dots: [{ value: 6965, duration: 8 }],
    },
    damageType: 'bleed',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  // Flurry line
  {
    name: 'Flurry',
    baseSkillName: 'Flurry',
    className: 'Weapon',
    skillLine: 'DualWield',
    damage: {
      hits: [{ value: 667 }, { value: 667 }, { value: 667 }, { value: 667 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Bloodthirst',
    baseSkillName: 'Flurry',
    className: 'Weapon',
    skillLine: 'DualWield',
    damage: {
      hits: [{ value: 689 }, { value: 689 }, { value: 689 }, { value: 689 }],
    },
    damageType: 'bleed',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Rapid Strikes',
    baseSkillName: 'Flurry',
    className: 'Weapon',
    skillLine: 'DualWield',
    damage: {
      hits: [{ value: 689 }, { value: 689 }, { value: 689 }, { value: 689 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  // Twin Slashes line
  {
    name: 'Twin Slashes',
    baseSkillName: 'Twin Slashes',
    className: 'Weapon',
    skillLine: 'DualWield',
    damage: {
      hits: [{ value: 580 }, { value: 580 }],
      dots: [{ value: 3470, duration: 20 }],
    },
    damageType: 'bleed',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Blood Craze',
    baseSkillName: 'Twin Slashes',
    className: 'Weapon',
    skillLine: 'DualWield',
    damage: {
      hits: [{ value: 580 }, { value: 580 }],
      dots: [{ value: 3470, duration: 20 }],
    },
    damageType: 'bleed',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Rending Slashes',
    baseSkillName: 'Twin Slashes',
    className: 'Weapon',
    skillLine: 'DualWield',
    damage: {
      hits: [{ value: 718 }, { value: 718 }],
      dots: [{ value: 3470, duration: 20 }],
    },
    damageType: 'bleed',
    targetType: 'single',
    resource: 'stamina',
  },
  // Whirlwind line
  {
    name: 'Whirlwind',
    baseSkillName: 'Whirlwind',
    className: 'Weapon',
    skillLine: 'DualWield',
    damage: {
      hits: [{ value: 1742 }],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: 'Steel Tornado',
    baseSkillName: 'Whirlwind',
    className: 'Weapon',
    skillLine: 'DualWield',
    damage: {
      hits: [{ value: 1742 }],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: 'Whirling Blades',
    baseSkillName: 'Whirlwind',
    className: 'Weapon',
    skillLine: 'DualWield',
    damage: {
      hits: [{ value: 1799 }],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  // Blade Cloak line
  {
    name: 'Blade Cloak',
    baseSkillName: 'Blade Cloak',
    className: 'Weapon',
    skillLine: 'DualWield',
    damage: {
      dots: [{ value: 421, duration: 20, interval: 2 }],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: 'Deadly Cloak',
    baseSkillName: 'Blade Cloak',
    className: 'Weapon',
    skillLine: 'DualWield',
    damage: {
      dots: [{ value: 567, duration: 20, interval: 2 }],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: 'Quick Cloak',
    baseSkillName: 'Blade Cloak',
    className: 'Weapon',
    skillLine: 'DualWield',
    damage: {
      dots: [{ value: 422, duration: 30, interval: 2 }],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  // Hidden Blade line
  {
    name: 'Hidden Blade',
    baseSkillName: 'Hidden Blade',
    className: 'Weapon',
    skillLine: 'DualWield',
    damage: {
      hits: [{ value: 1392 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Flying Blade',
    baseSkillName: 'Hidden Blade',
    className: 'Weapon',
    skillLine: 'DualWield',
    damage: {
      hits: [{ value: 1438 }, { value: 2160 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Shrouded Daggers',
    baseSkillName: 'Hidden Blade',
    className: 'Weapon',
    skillLine: 'DualWield',
    damage: {
      hits: [{ value: 1799 }, { value: 1799 }, { value: 1799 }],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
];

export { DUAL_WIELD_SKILLS };
