import { Skill } from '../../../models/skill';

const DRACONIC_POWER_SKILLS: Skill<'Dragonknight', 'DraconicPower'>[] = [
  // Ultimate abilities - Dragon Leap line
  {
    name: 'Dragon Leap',
    esoClass: 'Dragonknight',
    skillLine: 'DraconicPower',
    damage: {
      initial: 4241,
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Ferocious Leap',
    esoClass: 'Dragonknight',
    skillLine: 'DraconicPower',
    damage: {
      initial: 4241,
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'Take Flight',
    esoClass: 'Dragonknight',
    skillLine: 'DraconicPower',
    damage: {
      initial: 5037,
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  // Spiked Armor line
  {
    name: 'Spiked Armor',
    esoClass: 'Dragonknight',
    skillLine: 'DraconicPower',
    damage: {},
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Hardened Armor',
    esoClass: 'Dragonknight',
    skillLine: 'DraconicPower',
    damage: {},
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Volatile Armor',
    esoClass: 'Dragonknight',
    skillLine: 'DraconicPower',
    damage: {
      dot: 11,
      dotDuration: 20,
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Dark Talons line
  {
    name: 'Dark Talons',
    esoClass: 'Dragonknight',
    skillLine: 'DraconicPower',
    damage: {
      initial: 1742,
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Burning Talons',
    esoClass: 'Dragonknight',
    skillLine: 'DraconicPower',
    damage: {
      initial: 1799,
      dot: 1635,
      dotDuration: 5,
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Choking Talons',
    esoClass: 'Dragonknight',
    skillLine: 'DraconicPower',
    damage: {
      initial: 1742,
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Dragon Blood line
  {
    name: 'Dragon Blood',
    esoClass: 'Dragonknight',
    skillLine: 'DraconicPower',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Coagulating Blood',
    esoClass: 'Dragonknight',
    skillLine: 'DraconicPower',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Green Dragon Blood',
    esoClass: 'Dragonknight',
    skillLine: 'DraconicPower',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Protective Scale line
  {
    name: 'Protective Scale',
    esoClass: 'Dragonknight',
    skillLine: 'DraconicPower',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Dragon Fire Scale',
    esoClass: 'Dragonknight',
    skillLine: 'DraconicPower',
    damage: {},
    damageType: 'flame',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Protective Plate',
    esoClass: 'Dragonknight',
    skillLine: 'DraconicPower',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Inhale line
  {
    name: 'Inhale',
    esoClass: 'Dragonknight',
    skillLine: 'DraconicPower',
    damage: {
      initial: 870 + 1742, // 870 initial + 1742 exhale
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
    channelTime: 2.5,
  },
  {
    name: 'Deep Breath',
    esoClass: 'Dragonknight',
    skillLine: 'DraconicPower',
    damage: {
      initial: 870 + 2249, // 870 initial + 2249 exhale
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
    channelTime: 2.5,
  },
  {
    name: 'Draw Essence',
    esoClass: 'Dragonknight',
    skillLine: 'DraconicPower',
    damage: {
      initial: 870 + 1742, // 870 initial + 1742 exhale
    },
    damageType: 'flame',
    targetType: 'aoe',
    resource: 'magicka',
    channelTime: 2.5,
  },
];

export { DRACONIC_POWER_SKILLS };
