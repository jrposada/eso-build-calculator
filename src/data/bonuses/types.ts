export type BonusClassName =
  | 'passive' // Always applied
  | 'duration' // Always applied for a duration
  | 'skill-line' // Applied once if skill line is part of build
  | 'ability-slotted' // Applied once if at least 1 skill from skill line is equipped on build
  | 'ability-slotted-count'; // Applied once per skill of related skill line equipped on build
export type BonusId = 'Minor Savagery' | 'Minor Prophecy';

export type BonusType =
  | 'aoe-damage'
  | 'critical-chance'
  | 'critical-damage'
  | 'critical-damage'
  | 'direct-damage'
  | 'dot-damage'
  | 'duration'
  | 'max-magicka'
  | 'max-stamina'
  | 'off-balance-damage'
  | 'single-damage'
  | 'spell-critical-chance'
  | 'spell-damage'
  | 'weapon-critical-chance';

export type BonusData<TClassName extends BonusClassName = BonusClassName> = {
  id?: BonusId;

  name?: string;

  className: TClassName;

  type: BonusType;
  value: number;

  // /** Base on what do we apply the modifier */
  // type: PassiveBonusType;
  // damageTypes?: DamageType[];
};
