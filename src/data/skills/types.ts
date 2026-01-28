import { ClassName } from '../types';

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

export interface SkillDamage {
  readonly hits?: ReadonlyArray<{
    readonly value: number;
    readonly delay?: number;
  }>;
  readonly dots?: ReadonlyArray<DotDamage>;
}

export interface SkillData<
  TClassName extends ClassName = ClassName,
  TSkillLine extends string = string,
> {
  name: string;
  baseSkillName: string;

  className: TClassName;
  skillLine: TSkillLine;

  damage: SkillDamage;
  damageType: DamageType;
  targetType: TargetType;
  resource: Resource;
  channelTime?: number;
}
