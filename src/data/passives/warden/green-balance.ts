import { ClassPassiveSkill } from '../../../models/passive';

export const GREEN_BALANCE_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Accelerated Growth',
    skillLine: 'GreenBalance',
    esoClass: 'Warden',
    bonuses: [], // Healing done - not relevant for damage
  },
  {
    name: "Nature's Gift",
    skillLine: 'GreenBalance',
    esoClass: 'Warden',
    bonuses: [], // Magicka/Stamina return - not relevant for damage
  },
  {
    name: 'Emerald Moss',
    skillLine: 'GreenBalance',
    esoClass: 'Warden',
    bonuses: [], // Healing received - not relevant for damage
  },
  {
    name: 'Maturation',
    skillLine: 'GreenBalance',
    esoClass: 'Warden',
    bonuses: [], // Minor Toughness - not relevant for damage
  },
];
