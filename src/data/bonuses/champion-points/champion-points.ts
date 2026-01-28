import { ChampionPointBonus } from './types';

const CHAMPION_POINTS: ChampionPointBonus[] = [
  {
    className: 'passive',
    name: 'Backstabber',
    type: 'critical-damage',
    value: 0.02 * 5,
  },
  {
    className: 'passive',
    name: 'Biting Aura',
    type: 'aoe-damage',
    value: 0.03 * 2,
  },
  {
    className: 'passive',
    name: 'Deadly Aim',
    type: 'single-damage',
    value: 0.03 * 2,
  },
  {
    className: 'passive',
    name: 'Master-at-Arms',
    type: 'direct-damage',
    value: 0.03 * 2,
  },
  {
    className: 'passive',
    name: 'Exploiter',
    type: 'off-balance-damage',
    value: 0.02 * 5,
  },
  {
    className: 'passive',
    name: 'Fighting Finesse',
    type: 'critical-damage',
    value: 0.04 * 2,
  },
  {
    className: 'passive',
    name: 'Thaumaturge',
    type: 'dot-damage',
    value: 0.03 * 2,
  },
];

export { CHAMPION_POINTS };
