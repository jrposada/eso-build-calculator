export type EsoClass =
  | 'Dragonknight'
  | 'Sorcerer'
  | 'Nightblade'
  | 'Warden'
  | 'Necromancer'
  | 'Templar'
  | 'Arcanist';

export type DragonknightSkillLineName =
  | 'ArdentFlame'
  | 'DraconicPower'
  | 'EarthenHeart';

export type SorcererSkillLineName =
  | 'DarkMagic'
  | 'DaedricSummoning'
  | 'StormCalling';

export type NightbladeSkillLineName = 'Assassination' | 'Shadow' | 'Siphoning';

export type WardenSkillLineName =
  | 'AnimalCompanions'
  | 'GreenBalance'
  | 'WintersEmbrace';

export type NecromancerSkillLineName =
  | 'GraveLord'
  | 'BoneTyrant'
  | 'LivingDeath';

export type TemplarSkillLineName =
  | 'AedricSpear'
  | 'DawnsWrath'
  | 'RestoringLight';

export type ArcanistSkillLineName =
  | 'CurativeRuneforms'
  | 'SoldierOfApocrypha'
  | 'HeraldOfTheTome';

export type WeaponSkillLineName =
  | 'Bow'
  | 'TwoHanded'
  | 'DestructionStaff'
  | 'DualWield';

export type ClassSkillLine =
  | DragonknightSkillLineName
  | SorcererSkillLineName
  | NightbladeSkillLineName
  | WardenSkillLineName
  | NecromancerSkillLineName
  | TemplarSkillLineName
  | ArcanistSkillLineName;

export type SkillLine = ClassSkillLine | WeaponSkillLineName;

export type Resource = 'magicka' | 'stamina' | 'health' | 'ultimate';

export type DamageType =
  | 'magic'
  | 'physical'
  | 'disease'
  | 'flame'
  | 'poison'
  | 'bleed'
  | 'frost'
  | 'shock';

export type TargetType = 'single' | 'aoe';

export interface ClassSkill<
  TEsoClass extends EsoClass = EsoClass,
  TSkillLine extends ClassSkillLine = ClassSkillLine,
> {
  name: string;
  baseSkillName: string; // The base skill name for grouping (base, morph1, morph2)
  esoClass: TEsoClass;
  skillLine: TSkillLine;
  damage: {
    hits?: Array<{ value: number; delay?: number }>;
    dot?: number;
    dotDuration?: number;
    dotInterval?: number;
    dotIncreasePerTick?: number; // Percentage increase per tick (e.g., 0.12 for 12%)
  };
  damageType: DamageType;
  targetType: TargetType;
  resource: Resource;
  channelTime?: number;
}

export interface WeaponSkill<
  TSkillLine extends WeaponSkillLineName = WeaponSkillLineName,
> {
  name: string;
  baseSkillName: string; // The base skill name for grouping (base, morph1, morph2)
  skillLine: TSkillLine;
  damage: {
    hits?: Array<{ value: number; delay?: number }>;
    dot?: number;
    dotDuration?: number;
    dotInterval?: number;
    dotIncreasePerTick?: number; // Percentage increase per tick (e.g., 0.12 for 12%)
  };
  damageType: DamageType;
  targetType: TargetType;
  resource: Resource;
  channelTime?: number;
}
