import { ClassPassiveSkill } from '../../../models/passive';

export const RESTORING_LIGHT_PASSIVES: ClassPassiveSkill[] = [
  {
    name: 'Mending',
    skillLine: 'RestoringLight',
    esoClass: 'Templar',
    bonuses: [], // Healing done - not relevant for damage
  },
  {
    name: 'Sacred Ground',
    skillLine: 'RestoringLight',
    esoClass: 'Templar',
    bonuses: [], // Ground effect bonuses - not relevant for damage
  },
  {
    name: 'Light Weaver',
    skillLine: 'RestoringLight',
    esoClass: 'Templar',
    bonuses: [], // Ultimate gain on heal - not relevant for damage
  },
  {
    name: 'Master Ritualist',
    skillLine: 'RestoringLight',
    esoClass: 'Templar',
    bonuses: [], // Resurrection speed - not relevant for damage
  },
];
