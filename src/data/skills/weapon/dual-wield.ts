import { WeaponSkill } from '../../../models/skill';

const DUAL_WIELD_SKILLS: WeaponSkill<'DualWield'>[] = [
  // Ultimate abilities - Lacerate line
  {
    name: 'Lacerate',
    skillLine: 'DualWield',
    damage: {
      dot: 6960,
      dotDuration: 8,
    },
    damageType: 'bleed',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Rend',
    skillLine: 'DualWield',
    damage: {
      dot: 12942,
      dotDuration: 16,
    },
    damageType: 'bleed',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Thrive in Chaos',
    skillLine: 'DualWield',
    damage: {
      dot: 6965,
      dotDuration: 8,
    },
    damageType: 'bleed',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  // Flurry line
  {
    name: 'Flurry',
    skillLine: 'DualWield',
    damage: {
      hits: [
        { value: 667 },
        { value: 667 },
        { value: 667 },
        { value: 667 },
      ],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Bloodthirst',
    skillLine: 'DualWield',
    damage: {
      hits: [
        { value: 689 },
        { value: 689 },
        { value: 689 },
        { value: 689 },
      ],
    },
    damageType: 'bleed',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Rapid Strikes',
    skillLine: 'DualWield',
    damage: {
      hits: [
        { value: 689 },
        { value: 689 },
        { value: 689 },
        { value: 689 },
      ],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  // Twin Slashes line
  {
    name: 'Twin Slashes',
    skillLine: 'DualWield',
    damage: {
      hits: [{ value: 580 }, { value: 580 }],
      dot: 3470,
      dotDuration: 20,
    },
    damageType: 'bleed',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Blood Craze',
    skillLine: 'DualWield',
    damage: {
      hits: [{ value: 580 }, { value: 580 }],
      dot: 3470,
      dotDuration: 20,
    },
    damageType: 'bleed',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Rending Slashes',
    skillLine: 'DualWield',
    damage: {
      hits: [{ value: 718 }, { value: 718 }],
      dot: 3470,
      dotDuration: 20,
    },
    damageType: 'bleed',
    targetType: 'single',
    resource: 'stamina',
  },
  // Whirlwind line
  {
    name: 'Whirlwind',
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
    skillLine: 'DualWield',
    damage: {
      dot: 421,
      dotDuration: 20,
      dotInterval: 2,
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: 'Deadly Cloak',
    skillLine: 'DualWield',
    damage: {
      dot: 567,
      dotDuration: 20,
      dotInterval: 2,
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: 'Quick Cloak',
    skillLine: 'DualWield',
    damage: {
      dot: 422,
      dotDuration: 30,
      dotInterval: 2,
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  // Hidden Blade line
  {
    name: 'Hidden Blade',
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
