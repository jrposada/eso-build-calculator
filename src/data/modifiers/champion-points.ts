import { DamageModifier } from '../../models/modifier';

const CHAMPION_POINTS: DamageModifier[] = [
  {
    name: 'Backstabber',
    value: 0.02,
    maxLevel: 5,
    affects: 'critical',
  },
  {
    name: 'Biting Aura',
    value: 0.03,
    maxLevel: 2,
    affects: 'aoe',
  },
  {
    name: 'Deadly Aim',
    value: 0.03,
    maxLevel: 2,
    affects: 'single',
  },
  {
    name: 'Master-at-Arms',
    value: 0.03,
    maxLevel: 2,
    affects: 'direct',
  },
  {
    name: 'Exploiter',
    value: 0.02,
    maxLevel: 5,
    affects: 'off-balance',
  },
  {
    name: 'Fighting Finesse',
    value: 0.04,
    maxLevel: 2,
    affects: 'critical',
  },
  {
    name: 'Thaumaturge',
    value: 0.03,
    maxLevel: 2,
    affects: 'dot',
  },
];

export { CHAMPION_POINTS };
