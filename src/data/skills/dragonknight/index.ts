import { ARDENT_FLAME_SKILLS } from './ardent-flame';
import { DRACONIC_POWER_SKILLS } from './draconic-power';
import { EARTHEN_HEART_SKILLS } from './earthen-heart';
import type { DragonknightSkill } from './types';

const DRAGONKNIGHT_SKILLS: DragonknightSkill[] = [
  ...ARDENT_FLAME_SKILLS,
  ...DRACONIC_POWER_SKILLS,
  ...EARTHEN_HEART_SKILLS,
];

export { DRAGONKNIGHT_SKILLS };
