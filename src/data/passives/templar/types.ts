import { PassiveData } from '../types';

export type TemplarPassiveSkillLineName =
  | 'AedricSpear'
  | 'DawnsWrath'
  | 'RestoringLight';

export type TemplarPassive<
  TSkillLineName extends TemplarPassiveSkillLineName =
    TemplarPassiveSkillLineName,
> = PassiveData<'Templar', TSkillLineName>;
