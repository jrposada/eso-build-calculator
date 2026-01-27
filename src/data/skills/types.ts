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
  ignoresModifier?: boolean;
}

export type SkillClassName =
  | 'Dragonknight'
  | 'Sorcerer'
  | 'Nightblade'
  | 'Warden'
  | 'Necromancer'
  | 'Templar'
  | 'Arcanist'
  | 'Weapon';

export interface SkillData<
  TSkillClassName extends SkillClassName = SkillClassName,
  TSkillLine extends string = string,
> {
  name: string;
  baseSkillName: string;

  className: TSkillClassName;
  skillLine: TSkillLine;

  damage: {
    hits?: Array<{ value: number; delay?: number }>;
    dots?: DotDamage[];
  };
  damageType: DamageType;
  targetType: TargetType;
  resource: Resource;
  channelTime?: number;
}
