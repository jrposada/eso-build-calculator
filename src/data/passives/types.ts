import { DamageType } from '../skills/types';
import { ClassName } from '../types';

export type PassiveBonusType =
  | 'critical-chance'
  | 'critical-damage'
  | 'duration'
  | 'max-stamina'
  | 'max-magicka';

export type StatusEffect = 'Burning' | 'Poisoned' | 'Chilled' | 'Concussed';

export type PassiveBonus = {
  /** Base on what do we apply the modifier */
  multiplier:
    | 'skillLine' // Once if skill line is part of build
    | 'abilitySlotted' // Once if at least 1 skill from skill line is equipped on build
    | 'abilitySlottedCount'; // Once per skill of related skill line equipped on build
} & (
  | {
      type: PassiveBonusType;
      //** Decimal (0.05 = 5%) or seconds */
      value: number;
      damageTypes?: DamageType[];
      statusEffects?: StatusEffect[];
    }
  | {
      buffId: 'Minor Savagery';
    }
);

export interface PassiveData<
  TClassName extends ClassName = ClassName,
  TSkillLine extends string = string,
> {
  name: string;
  className: TClassName;
  skillLine: TSkillLine;
  bonuses: PassiveBonus[];
}
