import { SkillData } from '../types';

export type ArcanistSkillLineName =
  | 'CurativeRuneforms'
  | 'SoldierOfApocrypha'
  | 'HeraldOfTheTome';

export type ArcanistSkill<
  TSkillLineName extends ArcanistSkillLineName = ArcanistSkillLineName,
> = SkillData<'Arcanist', TSkillLineName>;
