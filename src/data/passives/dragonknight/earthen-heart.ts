import { DragonknightPassive } from './types';

export const EARTHEN_HEART_PASSIVES: DragonknightPassive<'EarthenHeart'>[] = [
  {
    name: 'Eternal Mountain',
    skillLine: 'EarthenHeart',
    className: 'Dragonknight',
    bonuses: [], // Buff duration - not relevant for damage
  },
  {
    name: 'Battle Roar',
    skillLine: 'EarthenHeart',
    className: 'Dragonknight',
    bonuses: [], // Resource return on ultimate - not relevant for damage
  },
  {
    name: "Mountain's Blessing",
    skillLine: 'EarthenHeart',
    className: 'Dragonknight',
    bonuses: [], // Ultimate generation - not relevant for damage
  },
  {
    name: 'Helping Hands',
    skillLine: 'EarthenHeart',
    className: 'Dragonknight',
    bonuses: [], // Stamina return - not relevant for damage
  },
];
