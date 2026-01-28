import { PassiveData } from '../types';

export type ArcanistPassiveSkillLineName =
  | 'CurativeRuneforms'
  | 'HeraldOfTheTome'
  | 'SoldierOfApocrypha';

export type ArcanistPassive<
  TSkillLineName extends ArcanistPassiveSkillLineName =
    ArcanistPassiveSkillLineName,
> = PassiveData<'Arcanist', TSkillLineName>;
