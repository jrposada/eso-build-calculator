import { ClassPassiveSkill } from '../../../models/passive';

export const HERALD_OF_THE_TOME_PASSIVES: ClassPassiveSkill[] = [
  {
    name: "Tome-Bearer's Inspiration",
    skillLine: 'HeraldOfTheTome',
    esoClass: 'Arcanist',
    bonuses: [{ type: 'skillLine', value: 0.1 }],
  },
];
