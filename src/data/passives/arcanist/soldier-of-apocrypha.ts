import { ArcanistPassive } from './types';

export const SOLDIER_OF_APOCRYPHA_PASSIVES: ArcanistPassive<'SoldierOfApocrypha'>[] =
  [
    {
      name: "Seeker's Will",
      skillLine: 'SoldierOfApocrypha',
      className: 'Arcanist',
      bonuses: [], // Cost reduction - not relevant for damage
    },
    {
      name: 'Resonating Glyphs',
      skillLine: 'SoldierOfApocrypha',
      className: 'Arcanist',
      bonuses: [], // AoE damage - handled by base skill damage
    },
    {
      name: 'Hidden Knowledge',
      skillLine: 'SoldierOfApocrypha',
      className: 'Arcanist',
      bonuses: [], // Penetration - handled by base skill damage
    },
    {
      name: 'Cruxweaver Armor',
      skillLine: 'SoldierOfApocrypha',
      className: 'Arcanist',
      bonuses: [], // Armor on Crux - defensive, not relevant for damage
    },
  ];
