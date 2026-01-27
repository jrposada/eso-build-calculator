import { BOW_SKILLS } from './bow';
import { DESTRUCTION_STAFF_SKILLS } from './destruction-staff';
import { DUAL_WIELD_SKILLS } from './dual-wield';
import { TWO_HANDED_SKILLS } from './two-handed';
import type { WeaponSkillData } from './types';

const WEAPON_SKILLS: WeaponSkillData[] = [
  ...BOW_SKILLS,
  ...DESTRUCTION_STAFF_SKILLS,
  ...DUAL_WIELD_SKILLS,
  ...TWO_HANDED_SKILLS,
];

export { WEAPON_SKILLS };
