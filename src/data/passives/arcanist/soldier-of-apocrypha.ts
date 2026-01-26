import { ClassPassiveSkill } from '../../../models/passive';

export const SOLDIER_OF_APOCRYPHA_PASSIVES: ClassPassiveSkill[] = [
  {
    name: "Seeker's Will",
    skillLine: 'SoldierOfApocrypha',
    esoClass: 'Arcanist',
    bonuses: [], // Cost reduction - not relevant for damage
  },
  {
    name: 'Resonating Glyphs',
    skillLine: 'SoldierOfApocrypha',
    esoClass: 'Arcanist',
    bonuses: [], // AoE damage - handled by base skill damage
  },
  {
    name: 'Hidden Knowledge',
    skillLine: 'SoldierOfApocrypha',
    esoClass: 'Arcanist',
    bonuses: [], // Penetration - handled by base skill damage
  },
  {
    name: 'Cruxweaver Armor',
    skillLine: 'SoldierOfApocrypha',
    esoClass: 'Arcanist',
    bonuses: [], // Armor on Crux - defensive, not relevant for damage
  },
];
