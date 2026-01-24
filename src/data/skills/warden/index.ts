import { ClassSkill } from '../../../models/skill';
import { ANIMAL_COMPANIONS_SKILLS } from './animal-companions';
import { GREEN_BALANCE_SKILLS } from './green-balance';
import { WINTERS_EMBRACE_SKILLS } from './winters-embrace';

const WARDEN_SKILLS: ClassSkill<'Warden'>[] = [
  ...ANIMAL_COMPANIONS_SKILLS,
  ...GREEN_BALANCE_SKILLS,
  ...WINTERS_EMBRACE_SKILLS,
];

export { WARDEN_SKILLS };
