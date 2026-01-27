import { SkillData } from '../types';

export type WeaponSkillLineName =
  | 'Bow'
  | 'TwoHanded'
  | 'DestructionStaff'
  | 'DualWield';

export type WeaponSkillData<
  TSkillLineName extends WeaponSkillLineName = WeaponSkillLineName,
> = SkillData<'Weapon', TSkillLineName>;
