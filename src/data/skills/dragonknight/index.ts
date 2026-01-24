import { Skill } from '../../../models/skill';
import { ARDENT_FLAME_SKILLS } from './ardent-flame';
import { DRACONIC_POWER_SKILLS } from './draconic-power';
import { EARTHEN_HEART_SKILLS } from './earthen-heart';

const DRAGONKNIGHT_SKILLS: Skill<'Dragonknight'>[] = [
  ...ARDENT_FLAME_SKILLS,
  ...DRACONIC_POWER_SKILLS,
  ...EARTHEN_HEART_SKILLS,
];

export { DRAGONKNIGHT_SKILLS };
