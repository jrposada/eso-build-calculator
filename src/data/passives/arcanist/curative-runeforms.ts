import { ClassPassiveSkill } from '../../../models/passive';

export const CURATIVE_RUNEFORMS_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Erudition',
    skillLine: 'CurativeRuneforms',
    esoClass: 'Arcanist',
    bonuses: [], // Magicka recovery - not relevant for damage
  },
  {
    name: 'Circumscribed Recovery',
    skillLine: 'CurativeRuneforms',
    esoClass: 'Arcanist',
    bonuses: [], // Healing done - not relevant for damage
  },
  {
    name: 'Healing Tides',
    skillLine: 'CurativeRuneforms',
    esoClass: 'Arcanist',
    bonuses: [], // Healing received - not relevant for damage
  },
  {
    name: "Curator's Focus",
    skillLine: 'CurativeRuneforms',
    esoClass: 'Arcanist',
    bonuses: [], // Crux gen on heal - not relevant for damage
  },
];
