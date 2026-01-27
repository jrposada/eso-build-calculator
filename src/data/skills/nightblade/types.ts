import { SkillData } from '../types';

export type NightbladeSkillLineName = 'Assassination' | 'Shadow' | 'Siphoning';

export type NightbladeSkill<
  TSkillLineName extends NightbladeSkillLineName = NightbladeSkillLineName,
> = SkillData<'Nightblade', TSkillLineName>;
