import { ClassPassiveSkill } from '../../../models/passive';

export const WINTERS_EMBRACE_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Glacial Presence',
    skillLine: 'WintersEmbrace',
    esoClass: 'Warden',
    bonuses: [], // Chilled proc chance - not tracked in stat-based system
  },
  {
    name: 'Frozen Armor',
    skillLine: 'WintersEmbrace',
    esoClass: 'Warden',
    bonuses: [], // Physical/Spell resistance - not relevant for damage
  },
  {
    name: 'Icy Aura',
    skillLine: 'WintersEmbrace',
    esoClass: 'Warden',
    bonuses: [], // Block cost reduction + Minor Protection - not relevant for damage
  },
  {
    name: 'Piercing Cold',
    skillLine: 'WintersEmbrace',
    esoClass: 'Warden',
    bonuses: [], // Frost damage penetration - handled by base skill damage
  },
];
