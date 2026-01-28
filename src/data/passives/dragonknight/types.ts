import { PassiveData } from '../types';

export type DragonknightPassiveSkillLineName =
  | 'ArdentFlame'
  | 'DraconicPower'
  | 'EarthenHeart';

export type DragonknightPassive<
  TSkillLineName extends DragonknightPassiveSkillLineName =
    DragonknightPassiveSkillLineName,
> = PassiveData<'Dragonknight', TSkillLineName>;
