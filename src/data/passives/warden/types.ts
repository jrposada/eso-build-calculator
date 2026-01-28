import { PassiveData } from '../types';

export type WardenPassiveSkillLineName =
  | 'AnimalCompanions'
  | 'GreenBalance'
  | 'WintersEmbrace';

export type WardenPassive<
  TSkillLineName extends WardenPassiveSkillLineName =
    WardenPassiveSkillLineName,
> = PassiveData<'Warden', TSkillLineName>;
