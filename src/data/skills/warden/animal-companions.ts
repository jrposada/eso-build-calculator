import { Skill } from '../../../models/skill';

const ANIMAL_COMPANIONS_SKILLS: Skill<'Warden', 'AnimalCompanions'>[] = [
  // Ultimate abilities - Feral Guardian line
  {
    name: 'Feral Guardian',
    esoClass: 'Warden',
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
    esoClass: 'Warden',
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
    esoClass: 'Warden',
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
    esoClass: 'Warden',
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
    esoClass: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {
      hits: [{ value: 2091 }],
      dot: 2140,
      dotDuration: 10,
    },
    damageType: 'bleed',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Screaming Cliff Racer',
    esoClass: 'Warden',
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
    esoClass: 'Warden',
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
    esoClass: 'Warden',
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
    esoClass: 'Warden',
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
    esoClass: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {
      dot: 4631,
      dotDuration: 20,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Fetcher Infection',
    esoClass: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {
      dot: 4785,
      dotDuration: 20,
    },
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Growing Swarm',
    esoClass: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {
      dot: 4785,
      dotDuration: 20,
    },
    damageType: 'bleed',
    targetType: 'single',
    resource: 'stamina',
  },
  // Betty Netch line
  {
    name: 'Betty Netch',
    esoClass: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Blue Betty',
    esoClass: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'magicka',
  },
  {
    name: 'Bull Netch',
    esoClass: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'stamina',
  },
  // Falcon's Swiftness line
  {
    name: "Falcon's Swiftness",
    esoClass: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Bird of Prey',
    esoClass: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'stamina',
  },
  {
    name: 'Deceptive Predator',
    esoClass: 'Warden',
    skillLine: 'AnimalCompanions',
    damage: {},
    damageType: 'magic',
    targetType: 'single',
    resource: 'stamina',
  },
];

export { ANIMAL_COMPANIONS_SKILLS };
