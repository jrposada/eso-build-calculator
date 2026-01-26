import { ClassPassiveSkill } from '../../../models/passive';

export const DRACONIC_POWER_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Iron Skin',
    skillLine: 'DraconicPower',
    esoClass: 'Dragonknight',
    bonuses: [], // Block mitigation - not relevant for damage
  },
  {
    name: 'Burning Heart',
    skillLine: 'DraconicPower',
    esoClass: 'Dragonknight',
    bonuses: [], // Healing received - not relevant for damage
  },
  {
    name: 'Elder Dragon',
    skillLine: 'DraconicPower',
    esoClass: 'Dragonknight',
    bonuses: [], // Health/Stamina/Magicka recovery - not relevant for damage
  },
  {
    name: 'Scaled Armor',
    skillLine: 'DraconicPower',
    esoClass: 'Dragonknight',
    bonuses: [], // Spell resistance - not relevant for damage
  },
];
