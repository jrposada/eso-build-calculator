import { NightbladeSkill } from './types';

const ASSASSINATION_SKILLS: NightbladeSkill<'Assassination'>[] = [
  // Ultimate abilities
  {
    name: 'Death Stroke',
    baseSkillName: 'Death Stroke',
    className: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      hits: [{ value: 3716 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'ultimate',
  },
  {
    name: 'Incapacitating Strike',
    baseSkillName: 'Death Stroke',
    className: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      hits: [{ value: 3840 }],
    },
    damageType: 'disease',
    targetType: 'single',
    resource: 'ultimate',
  },
  {
    name: 'Soul Harvest',
    baseSkillName: 'Death Stroke',
    className: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      hits: [{ value: 3718 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'ultimate',
  },
  // Veiled Strike line
  {
    name: 'Veiled Strike',
    baseSkillName: 'Veiled Strike',
    className: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      hits: [{ value: 2323 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Concealed Weapon',
    baseSkillName: 'Veiled Strike',
    className: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      hits: [{ value: 2556 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Surprise Attack',
    baseSkillName: 'Veiled Strike',
    className: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      hits: [{ value: 2399 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  // Teleport Strike line
  {
    name: 'Teleport Strike',
    baseSkillName: 'Teleport Strike',
    className: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      hits: [{ value: 1602 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Ambush',
    baseSkillName: 'Teleport Strike',
    className: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      hits: [{ value: 1655 }],
    },
    damageType: 'physical',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Lotus Fan',
    baseSkillName: 'Teleport Strike',
    className: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      hits: [{ value: 1603 }],
      dots: [{ value: 2050, duration: 5 }],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  // Assassin's Blade line
  {
    name: "Assassin's Blade",
    baseSkillName: "Assassin's Blade",
    className: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      hits: [{ value: 1161 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Impale',
    baseSkillName: "Assassin's Blade",
    className: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      hits: [{ value: 1161 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: "Killer's Blade",
    baseSkillName: "Assassin's Blade",
    className: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      hits: [{ value: 1161 }],
    },
    damageType: 'disease',
    targetType: 'single',
    resource: 'stamina',
  },
  // Mark Target line (utility/debuff skills)
  {
    name: 'Mark Target',
    baseSkillName: 'Mark Target',
    className: 'Nightblade',
    skillLine: 'Assassination',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Piercing Mark',
    baseSkillName: 'Mark Target',
    className: 'Nightblade',
    skillLine: 'Assassination',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: "Reaper's Mark",
    baseSkillName: 'Mark Target',
    className: 'Nightblade',
    skillLine: 'Assassination',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Grim Focus line
  {
    name: 'Grim Focus',
    baseSkillName: 'Grim Focus',
    className: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      hits: [{ value: 4182 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Merciless Resolve',
    baseSkillName: 'Grim Focus',
    className: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      hits: [{ value: 4752 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Relentless Focus',
    baseSkillName: 'Grim Focus',
    className: 'Nightblade',
    skillLine: 'Assassination',
    damage: {
      hits: [{ value: 4183 }],
    },
    damageType: 'disease',
    targetType: 'single',
    resource: 'stamina',
  },
];

export { ASSASSINATION_SKILLS };
