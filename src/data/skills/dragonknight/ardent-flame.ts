import { Skill } from '../../../models/skill';

const ARDENT_FLAME_SKILLS: Skill<'Dragonknight', 'ArdentFlame'>[] = [
  // Ultimate abilities
  {
    name: 'Dragonknight Standard',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      dot: 870,
      dotDuration: 16,
      dotInterval: 1,
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Shifting Standard',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      dot: 898,
      dotDuration: 25,
      dotInterval: 1,
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Standard of Might',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      dot: 870,
      dotDuration: 16,
      dotInterval: 1,
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  // Lava Whip line
  {
    name: 'Lava Whip',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      initial: 2323,
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Flame Lash',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      initial: 2323,
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Molten Whip',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      initial: 2323,
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  // Searing Strike line
  {
    name: 'Searing Strike',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      initial: 1161,
      dot: 3470,
      dotDuration: 20,
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Burning Embers',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      initial: 1161,
      dot: 3470,
      dotDuration: 20,
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Venomous Claw',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      initial: 1161,
      dot: 3470, // FIXME: The poison seeps into the target and deals increased damage the longer it lasts, dealing 12% more damage every 2 seconds
      dotDuration: 20,
    },
    damageType: 'poison',
    targetType: 'single',
    resource: 'stamina',
  },
  // Fiery Breath line
  {
    name: 'Fiery Breath',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      initial: 1742,
      dot: 2900,
      dotDuration: 20,
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Engulfing Flames',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      initial: 1799,
      dot: 2980,
      dotDuration: 20,
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Noxious Breath',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      initial: 1799,
      dot: 2980,
      dotDuration: 20,
    },
    damageType: 'poison',
    targetType: 'aoe',
    resource: 'stamina',
  },
  // Fiery Grip line
  {
    name: 'Fiery Grip',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      initial: 1392,
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Chains of Devastation',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      initial: 1438,
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Unrelenting Grip',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      initial: 1438,
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  // Inferno line
  {
    name: 'Inferno',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      dot: 1742,
      dotDuration: 15,
      dotInterval: 5,
    },
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Cauterize',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {},
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Flames of Oblivion',
    esoClass: 'Dragonknight',
    skillLine: 'ArdentFlame',
    damage: {
      dot: 1799,
      dotDuration: 15,
      dotInterval: 5,
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
  },
];

export { ARDENT_FLAME_SKILLS };
