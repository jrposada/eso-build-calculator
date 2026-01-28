import { ArcanistPassive } from './types';

export const CURATIVE_RUNEFORMS_PASSIVES: ArcanistPassive<'CurativeRuneforms'>[] =
  [
    {
      name: 'Erudition',
      skillLine: 'CurativeRuneforms',
      className: 'Arcanist',
      bonuses: [], // Magicka recovery - not relevant for damage
    },
    {
      name: 'Circumscribed Recovery',
      skillLine: 'CurativeRuneforms',
      className: 'Arcanist',
      bonuses: [], // Healing done - not relevant for damage
    },
    {
      name: 'Healing Tides',
      skillLine: 'CurativeRuneforms',
      className: 'Arcanist',
      bonuses: [], // Healing received - not relevant for damage
    },
    {
      name: "Curator's Focus",
      skillLine: 'CurativeRuneforms',
      className: 'Arcanist',
      bonuses: [], // Crux gen on heal - not relevant for damage
    },
  ];
