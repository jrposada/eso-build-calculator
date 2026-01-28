import { DragonknightPassive } from './types';

export const DRACONIC_POWER_PASSIVES: DragonknightPassive<'DraconicPower'>[] = [
  {
    name: 'Iron Skin',
    skillLine: 'DraconicPower',
    className: 'Dragonknight',
    bonuses: [], // Block mitigation - not relevant for damage
  },
  {
    name: 'Burning Heart',
    skillLine: 'DraconicPower',
    className: 'Dragonknight',
    bonuses: [], // Healing received - not relevant for damage
  },
  {
    name: 'Elder Dragon',
    skillLine: 'DraconicPower',
    className: 'Dragonknight',
    bonuses: [], // Health/Stamina/Magicka recovery - not relevant for damage
  },
  {
    name: 'Scaled Armor',
    skillLine: 'DraconicPower',
    className: 'Dragonknight',
    bonuses: [], // Spell resistance - not relevant for damage
  },
];
