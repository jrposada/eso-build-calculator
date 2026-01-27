import { SkillData } from '../types';

export type SorcererSkillLineName =
  | 'DarkMagic'
  | 'DaedricSummoning'
  | 'StormCalling';

export type SorcererSkill<
  TSkillLineName extends SorcererSkillLineName = SorcererSkillLineName,
> = SkillData<'Sorcerer', TSkillLineName>;
