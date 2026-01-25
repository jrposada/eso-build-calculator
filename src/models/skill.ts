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

export interface DotDamage {
  value: number;
  duration: number;
  delay?: number;
  interval?: number; // Defaults to duration if not specified
  increasePerTick?: number; // Percentage increase per tick (e.g., 0.12 for 12%)
  flatIncreasePerTick?: number; // Flat increase per tick
}

interface BaseSkill {
  name: string;
  baseSkillName: string; // The base skill name for grouping (base, morph1, morph2)
  damage: {
    hits?: Array<{ value: number; delay?: number }>;
    dots?: DotDamage[];
  };
  damageType: DamageType;
  targetType: TargetType;
  resource: Resource;
  channelTime?: number;
}

export interface ClassSkill<
  TEsoClass extends EsoClass = EsoClass,
  TSkillLine extends ClassSkillLine = ClassSkillLine,
> extends BaseSkill {
  esoClass: TEsoClass;
  skillLine: TSkillLine;
}

export interface WeaponSkill<
  TSkillLine extends WeaponSkillLineName = WeaponSkillLineName,
> extends BaseSkill {
  skillLine: TSkillLine;
}
