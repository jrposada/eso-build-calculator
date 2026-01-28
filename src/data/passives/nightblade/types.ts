import { PassiveData } from '../types';

export type NightbladePassiveSkillLineName =
  | 'Assassination'
  | 'Shadow'
  | 'Siphoning';

export type NightbladePassive<
  TSkillLineName extends NightbladePassiveSkillLineName =
    NightbladePassiveSkillLineName,
> = PassiveData<'Nightblade', TSkillLineName>;
