import { SkillData } from '../types';

export type DragonknightSkillLineName =
  | 'ArdentFlame'
  | 'DraconicPower'
  | 'EarthenHeart';

export type DragonknightSkill<
  TSkillLineName extends DragonknightSkillLineName = DragonknightSkillLineName,
> = SkillData<'Dragonknight', TSkillLineName>;
