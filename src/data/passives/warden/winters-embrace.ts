import { WardenPassive } from './types';

export const WINTERS_EMBRACE_PASSIVES: WardenPassive<'WintersEmbrace'>[] = [
  {
    name: 'Glacial Presence',
    skillLine: 'WintersEmbrace',
    className: 'Warden',
    bonuses: [], // Chilled proc chance - not tracked in stat-based system
  },
  {
    name: 'Frozen Armor',
    skillLine: 'WintersEmbrace',
    className: 'Warden',
    bonuses: [], // Physical/Spell resistance - not relevant for damage
  },
  {
    name: 'Icy Aura',
    skillLine: 'WintersEmbrace',
    className: 'Warden',
    bonuses: [], // Block cost reduction + Minor Protection - not relevant for damage
  },
  {
    name: 'Piercing Cold',
    skillLine: 'WintersEmbrace',
    className: 'Warden',
    bonuses: [], // Frost damage penetration - handled by base skill damage
  },
];
