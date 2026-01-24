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

export type SkillLine =
  | DragonknightSkillLineName
  | SorcererSkillLineName
  | NightbladeSkillLineName
  | WardenSkillLineName
  | NecromancerSkillLineName
  | TemplarSkillLineName
  | ArcanistSkillLineName;

export type Resource = 'magicka' | 'stamina' | 'health' | 'ultimate';

export type DamageType = 'magic' | 'physical' | 'disease' | 'flame' | 'poison';

export type TargetType = 'single' | 'aoe';

export interface Skill<
  TEsoClass extends EsoClass = EsoClass,
  TSkillLine extends SkillLine = SkillLine,
> {
  name: string;
  esoClass: TEsoClass;
  skillLine: TSkillLine;
  damage: {
    initial?: number;
    dot?: number;
    dotDuration?: number;
    dotInterval?: number;
  };
  damageType: DamageType;
  targetType: TargetType;
  resource: Resource;
}
