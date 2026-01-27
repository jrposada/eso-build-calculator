import { SkillData } from '../types';

export type WardenSkillLineName =
  | 'AnimalCompanions'
  | 'GreenBalance'
  | 'WintersEmbrace';

export type WardenSkill<
  TSkillLineName extends WardenSkillLineName = WardenSkillLineName,
> = SkillData<'Warden', TSkillLineName>;
