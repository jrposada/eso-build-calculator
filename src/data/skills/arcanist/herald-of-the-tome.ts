import { ClassSkill } from '../../../models/skill';

const HERALD_OF_THE_TOME_SKILLS: ClassSkill<'Arcanist', 'HeraldOfTheTome'>[] = [
  // Ultimate abilities - The Unblinking Eye line
  {
    name: 'The Unblinking Eye',
    esoClass: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dot: 1115,
      dotDuration: 6,
      dotInterval: 0.5,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'The Languid Eye',
    esoClass: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dot: 1115,
      dotDuration: 6,
      dotInterval: 0.5,
      dotIncreasePerTick: 0.07, // damage increases by 7% every 0.5 seconds
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: "The Tide King's Gaze",
    esoClass: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dot: 1151,
      dotDuration: 8,
      dotInterval: 0.5,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  // Runeblades line
  {
    name: 'Runeblades',
    esoClass: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      hits: [{ value: 695 * 3 }], // 3 hits
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Escalating Runeblades',
    esoClass: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      hits: [{ value: 696 + 766 + 917 }], // escalating 3 hits, last one is AoE
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Writhing Runeblades',
    esoClass: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      hits: [{ value: 718 * 3 }], // 3 hits
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Fatecarver line
  {
    name: 'Fatecarver',
    esoClass: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dot: 879,
      dotDuration: 4,
      dotInterval: 0.3,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
    channelTime: 4,
  },
  {
    name: 'Exhausting Fatecarver',
    esoClass: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dot: 879,
      dotDuration: 4, // +0.3s per Crux spent
      dotInterval: 0.3,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
    channelTime: 4,
  },
  {
    name: 'Pragmatic Fatecarver',
    esoClass: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dot: 879,
      dotDuration: 4,
      dotInterval: 0.3,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
    channelTime: 4,
  },
  // Abyssal Impact line
  {
    name: 'Abyssal Impact', // You deal 5% increased damage to enemies drenched in Abyssal Ink.
    esoClass: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      hits: [{ value: 1939 }],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: "Cephaliarch's Flail", // You deal 5% increased damage to enemies drenched in Abyssal Ink.
    esoClass: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      hits: [{ value: 1939 }],
    },
    damageType: 'physical',
    targetType: 'aoe',
    resource: 'stamina',
  },
  {
    name: 'Tentacular Dread', // You deal 5% increased damage to enemies drenched in Abyssal Ink.
    esoClass: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      hits: [{ value: 2002 }],
    },
    damageType: 'frost',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Tome-Bearer's Inspiration line
  {
    name: "Tome-Bearer's Inspiration",
    esoClass: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dot: 1161,
      dotDuration: 20, // assumed duration for buff
      dotInterval: 5,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Inspired Scholarship',
    esoClass: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dot: 935,
      dotDuration: 20, // assumed duration for buff
      dotInterval: 3,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Recuperative Treatise',
    esoClass: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dot: 1161,
      dotDuration: 20, // assumed duration for buff
      dotInterval: 5,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // The Imperfect Ring line
  {
    name: 'The Imperfect Ring',
    esoClass: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dot: 4631,
      dotDuration: 20,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Fulminating Rune',
    esoClass: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      hits: [{ value: 1438, delay: 6 }], // detonation after 6s
      dot: 4642,
      dotDuration: 20,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Rune of Displacement',
    esoClass: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dot: 4780,
      dotDuration: 18,
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
];

export { HERALD_OF_THE_TOME_SKILLS };
