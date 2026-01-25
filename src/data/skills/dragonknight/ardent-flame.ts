import { ClassSkill } from '../../../models/skill';

const ARDENT_FLAME_SKILLS: ClassSkill<'Dragonknight', 'ArdentFlame'>[] = [
  // Ultimate abilities
  {
    name: 'Dragonknight Standard',
    baseSkillName: 'Dragonknight Standard',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      dots: [{ value: 870, duration: 16, interval: 1 }],
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Shifting Standard',
    baseSkillName: 'Dragonknight Standard',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      dots: [{ value: 898, duration: 25, interval: 1 }],
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Standard of Might',
    baseSkillName: 'Dragonknight Standard',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      dots: [{ value: 870, duration: 16, interval: 1 }],
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  // Lava Whip line
  {
    name: 'Lava Whip',
    baseSkillName: 'Lava Whip',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      hits: [{ value: 2323 }],
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Flame Lash',
    baseSkillName: 'Lava Whip',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      hits: [{ value: 2323 }],
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Molten Whip',
    baseSkillName: 'Lava Whip',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      hits: [{ value: 2323 }],
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  // Searing Strike line
  {
    name: 'Searing Strike',
    baseSkillName: 'Searing Strike',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      hits: [{ value: 1161 }],
      dots: [{ value: 3470, duration: 20 }],
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Burning Embers',
    baseSkillName: 'Searing Strike',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      hits: [{ value: 1161 }],
      dots: [{ value: 3470, duration: 20 }],
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Venomous Claw',
    baseSkillName: 'Searing Strike',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      hits: [{ value: 1161 }],
      dots: [{ value: 347, duration: 20, interval: 2, increasePerTick: 0.12 }],
    },
    damageType: 'poison',
    targetType: 'single',
    resource: 'stamina',
  },
  // Fiery Breath line
  {
    name: 'Fiery Breath',
    baseSkillName: 'Fiery Breath',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      hits: [{ value: 1742 }],
      dots: [{ value: 2900, duration: 20 }],
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Engulfing Flames',
    baseSkillName: 'Fiery Breath',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      hits: [{ value: 1799 }],
      dots: [{ value: 2980, duration: 20 }],
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Noxious Breath',
    baseSkillName: 'Fiery Breath',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      hits: [{ value: 1799 }],
      dots: [{ value: 2980, duration: 20 }],
    },
    damageType: 'poison',
    targetType: 'aoe',
    resource: 'stamina',
  },
  // Fiery Grip line
  {
    name: 'Fiery Grip',
    baseSkillName: 'Fiery Grip',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      hits: [{ value: 1392 }],
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Chains of Devastation',
    baseSkillName: 'Fiery Grip',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      hits: [{ value: 1438 }],
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Unrelenting Grip',
    baseSkillName: 'Fiery Grip',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      hits: [{ value: 1438 }],
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  // Inferno line
  {
    name: 'Inferno',
    baseSkillName: 'Inferno',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      dots: [{ value: 1742, duration: 15, interval: 5 }],
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Cauterize',
    baseSkillName: 'Inferno',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {},
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Flames of Oblivion',
    baseSkillName: 'Inferno',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      dots: [{ value: 1799, duration: 15, interval: 5 }],
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
  },
];

export { ARDENT_FLAME_SKILLS };
