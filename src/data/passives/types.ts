import { BonusData } from '../bonuses/types';
import { ClassName } from '../types';

export interface PassiveData<
  TClassName extends ClassName = ClassName,
  TSkillLine extends string = string,
> {
  name: string;
  className: TClassName;
  skillLine: TSkillLine;
  bonuses: BonusData[];
}
