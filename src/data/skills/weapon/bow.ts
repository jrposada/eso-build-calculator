import { WeaponSkill } from '../../../models/skill';

const BOW_SKILLS: WeaponSkill<'Bow'>[] = [
  // Ultimate abilities - Rapid Fire line
  {
    name: 'Rapid Fire',
    skillLine: 'Bow',
    damage: {
      dot: 17415,
      dotDuration: 4,
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'ultimate',
    channelTime: 4,
  },
  {
    name: 'Ballista',
    skillLine: 'Bow',
    damage: {
      dot: 15587,
      dotDuration: 5,
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'ultimate',
  },
  {
    name: 'Toxic Barrage',
    skillLine: 'Bow',
    damage: {
      dot: 17415,
      dotDuration: 4,
    },
    damageType: 'poison',
    targetType: 'single',
    resource: 'ultimate',
    channelTime: 4,
  },
  // Snipe line
  {
    name: 'Snipe',
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
    skillLine: 'Bow',
    damage: {
      dot: 342,
      dotDuration: 8,
      dotInterval: 1,
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: 'Arrow Barrage',
    skillLine: 'Bow',
    damage: {
      dot: 460,
      dotDuration: 8,
      dotInterval: 1,
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: 'Endless Hail',
    skillLine: 'Bow',
    damage: {
      dot: 343,
      dotDuration: 13,
      dotInterval: 1,
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  // Scatter Shot line
  {
    name: 'Scatter Shot',
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
    skillLine: 'Bow',
    damage: {
      hits: [{ value: 1742 }],
      dot: 1635,
      dotDuration: 5,
    },
    damageType: 'poison',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: 'Bombard',
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
    skillLine: 'Bow',
    damage: {
      hits: [{ value: 1161 }],
      dot: 3470,
      dotDuration: 20,
    },
    damageType: 'poison',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Poison Injection',
    skillLine: 'Bow',
    damage: {
      hits: [{ value: 1161 }],
      dot: 3470,
      dotDuration: 20,
    },
    damageType: 'poison',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Venom Arrow',
    skillLine: 'Bow',
    damage: {
      hits: [{ value: 1161 }],
      dot: 3470,
      dotDuration: 20,
    },
    damageType: 'poison',
    targetType: 'single',
    resource: 'stamina',
  },
];

export { BOW_SKILLS };
