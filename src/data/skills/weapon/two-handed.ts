import { WeaponSkill } from '../../../models/skill';

const TWO_HANDED_SKILLS: WeaponSkill<'TwoHanded'>[] = [
  // Ultimate abilities - Berserker Strike line
  {
    name: 'Berserker Strike',
    skillLine: 'TwoHanded',
    damage: {
      hits: [{ value: 3486 }],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Berserker Rage',
    skillLine: 'TwoHanded',
    damage: {
      hits: [{ value: 3600 }],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Onslaught',
    skillLine: 'TwoHanded',
    damage: {
      hits: [{ value: 3485 }],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  // Uppercut line
  {
    name: 'Uppercut',
    skillLine: 'TwoHanded',
    damage: {
      hits: [{ value: 2672 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Dizzying Swing',
    skillLine: 'TwoHanded',
    damage: {
      hits: [{ value: 2760 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Wrecking Blow',
    skillLine: 'TwoHanded',
    damage: {
      hits: [{ value: 2760 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  // Critical Charge line
  {
    name: 'Critical Charge',
    skillLine: 'TwoHanded',
    damage: {
      hits: [{ value: 1392 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Critical Rush',
    skillLine: 'TwoHanded',
    damage: {
      hits: [{ value: 1393 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Stampede',
    skillLine: 'TwoHanded',
    damage: {
      hits: [{ value: 1393 }],
      dot: 319,
      dotDuration: 15,
      dotInterval: 1,
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  // Cleave line
  {
    name: 'Cleave',
    skillLine: 'TwoHanded',
    damage: {
      hits: [{ value: 1742 }],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: 'Brawler',
    skillLine: 'TwoHanded',
    damage: {
      hits: [{ value: 1742 }],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: 'Carve',
    skillLine: 'TwoHanded',
    damage: {
      hits: [{ value: 1742 }],
      dot: 2868,
      dotDuration: 12,
    },
    damageType: 'bleed',
    targetType: 'aoe',
    resource: 'stamina',
  },
  // Reverse Slash line
  {
    name: 'Reverse Slash',
    skillLine: 'TwoHanded',
    damage: {
      hits: [{ value: 1161 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Executioner',
    skillLine: 'TwoHanded',
    damage: {
      hits: [{ value: 1161 }],
    },
    damageType: 'bleed',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Reverse Slice',
    skillLine: 'TwoHanded',
    damage: {
      hits: [{ value: 1199 }],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  // Momentum line
  {
    name: 'Momentum',
    skillLine: 'TwoHanded',
    damage: {},
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Forward Momentum',
    skillLine: 'TwoHanded',
    damage: {},
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Rally',
    skillLine: 'TwoHanded',
    damage: {},
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
];

export { TWO_HANDED_SKILLS };
