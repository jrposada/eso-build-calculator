import { ClassPassiveSkill } from '../../../models/passive';

export const ARDENT_FLAME_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Combustion',
    skillLine: 'ArdentFlame',
    esoClass: 'Dragonknight',
    bonuses: [
      {
        type: 'statusEffect',
        value: 0.33,
        statusEffects: ['Burning', 'Poisoned'],
      },
    ],
  },
  {
    name: 'Warmth',
    skillLine: 'ArdentFlame',
    esoClass: 'Dragonknight',
    bonuses: [{ type: 'dot', value: 0.06 }],
  },
  {
    name: 'Searing Heat',
    skillLine: 'ArdentFlame',
    esoClass: 'Dragonknight',
    bonuses: [{ type: 'skillLine', value: 0.25 }],
  },
  {
    name: 'World in Ruin',
    skillLine: 'ArdentFlame',
    esoClass: 'Dragonknight',
    bonuses: [
      {
        type: 'damageType',
        value: 0.05,
        damageTypes: ['flame', 'poison'],
      },
    ],
  },
];
