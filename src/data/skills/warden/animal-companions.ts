import { WardenSkill } from './types';

const ANIMAL_COMPANIONS_SKILLS: WardenSkill<'AnimalCompanions'>[] = [
  // Ultimate abilities - Feral Guardian line
  {
    name: 'Feral Guardian',
    baseSkillName: 'Feral Guardian',
    className: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {
      hits: [{ value: 580 }], // swipe damage, also has 2323 AoE swipe and 3253 maul
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'ultimate',
  },
  {
    name: 'Eternal Guardian',
    baseSkillName: 'Feral Guardian',
    className: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {
      hits: [{ value: 599 }], // swipe damage, also has 2399 AoE swipe and 3360 maul
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'ultimate',
  },
  {
    name: 'Wild Guardian',
    baseSkillName: 'Feral Guardian',
    className: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {
      hits: [{ value: 659 }], // swipe damage, also has 2640 AoE swipe and 3697 maul
    },
    damageType: 'bleed',
    targetType: 'single',
    resource: 'ultimate',
  },
  // Dive line
  {
    name: 'Dive',
    baseSkillName: 'Dive',
    className: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {
      hits: [{ value: 2090 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Cutting Dive',
    baseSkillName: 'Dive',
    className: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {
      hits: [{ value: 2091 }],
      dots: [{ value: 2140, duration: 10 }],
    },
    damageType: 'bleed',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Screaming Cliff Racer',
    baseSkillName: 'Dive',
    className: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {
      hits: [{ value: 2160 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  // Scorch line
  {
    name: 'Scorch',
    baseSkillName: 'Scorch',
    className: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {
      hits: [
        { value: 2509, delay: 3 },
        { value: 3486, delay: 9 },
      ],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Deep Fissure',
    baseSkillName: 'Scorch',
    className: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {
      hits: [
        { value: 2591, delay: 3 },
        { value: 3600, delay: 9 },
      ],
    },
    damageType: 'magic',
    targetType: 'aoe',
    resource: 'magicka',
  },
  {
    name: 'Subterranean Assault',
    baseSkillName: 'Scorch',
    className: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {
      hits: [
        { value: 2591, delay: 3 },
        { value: 2591, delay: 6 },
      ],
    },
    damageType: 'poison',
    targetType: 'aoe',
    resource: 'stamina',
  },
  // Swarm line
  {
    name: 'Swarm',
    baseSkillName: 'Swarm',
    className: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {
      dots: [{ value: 4631, duration: 20 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Fetcher Infection',
    baseSkillName: 'Swarm',
    className: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {
      dots: [{ value: 4785, duration: 20 }],
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Growing Swarm',
    baseSkillName: 'Swarm',
    className: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {
      dots: [{ value: 4785, duration: 20 }],
    },
    damageType: 'bleed',
    targetType: 'single',
    resource: 'stamina',
  },
  // Betty Netch line
  {
    name: 'Betty Netch',
    baseSkillName: 'Betty Netch',
    className: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Blue Betty',
    baseSkillName: 'Betty Netch',
    className: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Bull Netch',
    baseSkillName: 'Betty Netch',
    className: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'stamina',
  },
  // Falcon's Swiftness line
  {
    name: "Falcon's Swiftness",
    baseSkillName: "Falcon's Swiftness",
    className: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Bird of Prey',
    baseSkillName: "Falcon's Swiftness",
    className: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Deceptive Predator',
    baseSkillName: "Falcon's Swiftness",
    className: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'stamina',
  },
];

export { ANIMAL_COMPANIONS_SKILLS };
