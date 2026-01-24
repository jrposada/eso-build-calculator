import { ClassSkill } from '../../../models/skill';

const SIPHONING_SKILLS: ClassSkill<'Nightblade', 'Siphoning'>[] = [
  // Ultimate abilities
  {
    name: 'Soul Shred',
    esoClass: 'Nightblade',
    skillLine: 'Siphoning',
    damage: {
      hits: [{ value: 3486 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Soul Siphon',
    esoClass: 'Nightblade',
    skillLine: 'Siphoning',
    damage: {},
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Soul Tether',
    esoClass: 'Nightblade',
    skillLine: 'Siphoning',
    damage: {
      hits: [{ value: 3600 }],
      dot: 627,
      dotDuration: 8,
      dotInterval: 1,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  // Strife line
  {
    name: 'Strife',
    esoClass: 'Nightblade',
    skillLine: 'Siphoning',
    damage: {
      dot: 1548,
      dotDuration: 10,
      dotInterval: 2,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Funnel Health',
    esoClass: 'Nightblade',
    skillLine: 'Siphoning',
    damage: {
      dot: 1600,
      dotDuration: 10,
      dotInterval: 2,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Swallow Soul',
    esoClass: 'Nightblade',
    skillLine: 'Siphoning',
    damage: {
      dot: 2160,
      dotDuration: 10,
      dotInterval: 2,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Malevolent Offering line
  {
    name: 'Malevolent Offering',
    esoClass: 'Nightblade',
    skillLine: 'Siphoning',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Healthy Offering',
    esoClass: 'Nightblade',
    skillLine: 'Siphoning',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Shrewd Offering',
    esoClass: 'Nightblade',
    skillLine: 'Siphoning',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Cripple line
  {
    name: 'Cripple',
    esoClass: 'Nightblade',
    skillLine: 'Siphoning',
    damage: {
      dot: 4631,
      dotDuration: 20,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Crippling Grasp',
    esoClass: 'Nightblade',
    skillLine: 'Siphoning',
    damage: {
      hits: [{ value: 1199 }],
      dot: 4350,
      dotDuration: 20,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Debilitate',
    esoClass: 'Nightblade',
    skillLine: 'Siphoning',
    damage: {
      dot: 4785,
      dotDuration: 20,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Siphoning Strikes line
  {
    name: 'Siphoning Strikes',
    esoClass: 'Nightblade',
    skillLine: 'Siphoning',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Leeching Strikes',
    esoClass: 'Nightblade',
    skillLine: 'Siphoning',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Siphoning Attacks',
    esoClass: 'Nightblade',
    skillLine: 'Siphoning',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Drain Power line
  {
    name: 'Drain Power',
    esoClass: 'Nightblade',
    skillLine: 'Siphoning',
    damage: {
      hits: [{ value: 1742 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Power Extraction',
    esoClass: 'Nightblade',
    skillLine: 'Siphoning',
    damage: {
      hits: [{ value: 1742 }],
    },
    damageType: 'disease',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: 'Sap Essence',
    esoClass: 'Nightblade',
    skillLine: 'Siphoning',
    damage: {
      hits: [{ value: 1742 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
];

export { SIPHONING_SKILLS };
