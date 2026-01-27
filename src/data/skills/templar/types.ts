import { SkillData } from '../types';

export type TemplarSkillLineName =
  | 'AedricSpear'
  | 'DawnsWrath'
  | 'RestoringLight';

export type TemplarSkill<
  TSkillLineName extends TemplarSkillLineName = TemplarSkillLineName,
> = SkillData<'Templar', TSkillLineName>;
