import { ArcanistSkill } from './types';

const HERALD_OF_THE_TOME_SKILLS: ArcanistSkill<'HeraldOfTheTome'>[] = [
  // Ultimate abilities - The Unblinking Eye line
  {
    name: 'The Unblinking Eye',
    baseSkillName: 'The Unblinking Eye',
    className: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dots: [{ value: 1115, duration: 6, interval: 0.5 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: 'The Languid Eye',
    baseSkillName: 'The Unblinking Eye',
    className: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dots: [
        { value: 1115, duration: 6, interval: 0.5, increasePerTick: 0.07 },
      ], // damage increases by 7% every 0.5 seconds
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  {
    name: "The Tide King's Gaze",
    baseSkillName: 'The Unblinking Eye',
    className: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dots: [{ value: 1151, duration: 8, interval: 0.5 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'ultimate',
  },
  // Runeblades line
  {
    name: 'Runeblades',
    baseSkillName: 'Runeblades',
    className: 'Arcanist',
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
    baseSkillName: 'Runeblades',
    className: 'Arcanist',
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
    baseSkillName: 'Runeblades',
    className: 'Arcanist',
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
    baseSkillName: 'Fatecarver',
    className: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dots: [{ value: 879, duration: 4, interval: 0.3 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
    channelTime: 4,
  },
  {
    name: 'Exhausting Fatecarver',
    baseSkillName: 'Fatecarver',
    className: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dots: [{ value: 879, duration: 4, interval: 0.3 }], // +0.3s per Crux spent
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
    channelTime: 4,
  },
  {
    name: 'Pragmatic Fatecarver',
    baseSkillName: 'Fatecarver',
    className: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dots: [{ value: 879, duration: 4, interval: 0.3 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
    channelTime: 4,
  },
  // Abyssal Impact line
  {
    name: 'Abyssal Impact', // You deal 5% increased damage to enemies drenched in Abyssal Ink.
    baseSkillName: 'Abyssal Impact',
    className: 'Arcanist',
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
    baseSkillName: 'Abyssal Impact',
    className: 'Arcanist',
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
    baseSkillName: 'Abyssal Impact',
    className: 'Arcanist',
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
    baseSkillName: "Tome-Bearer's Inspiration",
    className: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dots: [{ value: 1161, duration: 20, interval: 5 }], // assumed duration for buff
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Inspired Scholarship',
    baseSkillName: "Tome-Bearer's Inspiration",
    className: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dots: [{ value: 935, duration: 20, interval: 3 }], // assumed duration for buff
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Recuperative Treatise',
    baseSkillName: "Tome-Bearer's Inspiration",
    className: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dots: [{ value: 1161, duration: 20, interval: 5 }], // assumed duration for buff
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // The Imperfect Ring line
  {
    name: 'The Imperfect Ring',
    baseSkillName: 'The Imperfect Ring',
    className: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dots: [{ value: 4631, duration: 20 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Fulminating Rune',
    baseSkillName: 'The Imperfect Ring',
    className: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      hits: [{ value: 1438, delay: 6 }], // detonation after 6s
      dots: [{ value: 4642, duration: 20 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Rune of Displacement',
    baseSkillName: 'The Imperfect Ring',
    className: 'Arcanist',
    skillLine: 'HeraldOfTheTome',
    damage: {
      dots: [{ value: 4780, duration: 18 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
];

export { HERALD_OF_THE_TOME_SKILLS };
