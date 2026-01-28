import { WardenPassive } from './types';

export const GREEN_BALANCE_PASSIVES: WardenPassive<'GreenBalance'>[] = [
  {
    name: 'Accelerated Growth',
    skillLine: 'GreenBalance',
    className: 'Warden',
    bonuses: [], // Healing done - not relevant for damage
  },
  {
    name: "Nature's Gift",
    skillLine: 'GreenBalance',
    className: 'Warden',
    bonuses: [], // Magicka/Stamina return - not relevant for damage
  },
  {
    name: 'Emerald Moss',
    skillLine: 'GreenBalance',
    className: 'Warden',
    bonuses: [], // Healing received - not relevant for damage
  },
  {
    name: 'Maturation',
    skillLine: 'GreenBalance',
    className: 'Warden',
    bonuses: [], // Minor Toughness - not relevant for damage
  },
];
