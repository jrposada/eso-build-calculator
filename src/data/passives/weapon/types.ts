import { PassiveData } from '../types';

export type WeaponPassiveSkillLineName =
  | 'Bow'
  | 'DestructionStaff'
  | 'DualWield'
  | 'TwoHanded';

export type WeaponPassive<
  TSkillLineName extends WeaponPassiveSkillLineName =
    WeaponPassiveSkillLineName,
> = PassiveData<'Weapon', TSkillLineName>;
