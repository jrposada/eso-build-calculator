import { TemplarPassive } from './types';

export const RESTORING_LIGHT_PASSIVES: TemplarPassive<'RestoringLight'>[] = [
  {
    name: 'Mending',
    skillLine: 'RestoringLight',
    className: 'Templar',
    bonuses: [], // Healing done - not relevant for damage
  },
  {
    name: 'Sacred Ground',
    skillLine: 'RestoringLight',
    className: 'Templar',
    bonuses: [], // Ground effect bonuses - not relevant for damage
  },
  {
    name: 'Light Weaver',
    skillLine: 'RestoringLight',
    className: 'Templar',
    bonuses: [], // Ultimate gain on heal - not relevant for damage
  },
  {
    name: 'Master Ritualist',
    skillLine: 'RestoringLight',
    className: 'Templar',
    bonuses: [], // Resurrection speed - not relevant for damage
  },
];
