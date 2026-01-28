import { PassiveData } from '../types';

export type SorcererPassiveSkillLineName =
  | 'DaedricSummoning'
  | 'DarkMagic'
  | 'StormCalling';

export type SorcererPassive<
  TSkillLineName extends SorcererPassiveSkillLineName =
    SorcererPassiveSkillLineName,
> = PassiveData<'Sorcerer', TSkillLineName>;
