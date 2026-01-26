import { logger } from '../infrastructure';
import { DamageModifier } from '../models/modifier';
import {
  AnyPassiveSkill,
  ClassPassiveSkill,
  PassiveBonus,
  WeaponPassiveSkill,
} from '../models/passive';
import { ClassSkill, WeaponSkill } from '../models/skill';

export type AnySkill = ClassSkill | WeaponSkill;

export type SkillMechanic = 'dot' | 'instant' | 'channeled';

export interface SkillLineCounts {
  [skillLine: string]: number;
}

export function getSkillSource(skill: AnySkill): string {
  if ('esoClass' in skill) {
    return skill.esoClass;
  }
  return 'Weapon';
}

export function getSkillMechanic(skill: AnySkill): SkillMechanic | 'unknown' {
  if (skill.channelTime) {
    return 'channeled';
  }

  if (skill.damage.dots?.length) {
    return 'dot';
  }

  if (
    !!skill.damage.hits?.length &&
    skill.damage.hits.some((hit) => Boolean(hit.value)) &&
    !skill.damage.dots?.length &&
    !skill.channelTime
  ) {
    return 'instant';
  }

  return 'unknown';
}

/**
 * Get the duration of a skill (max dot duration or channelTime)
 */
export function getSkillDuration(skill: AnySkill): number {
  if (skill.damage.dots?.length) {
    return Math.max(
      ...skill.damage.dots.map((dot) => dot.duration + (dot.delay ?? 0)),
    );
  }
  if (skill.channelTime) {
    return skill.channelTime;
  }
  return 0;
}

function applyDamageModifier(
  modifiers: DamageModifier[],
  value: number,
): number {
  const totalModifier = modifiers.reduce(
    (sum, modifier) => sum + modifier.value * modifier.maxLevel,
    0,
  );
  return value * (1 + totalModifier);
}

/**
 * Calculate total damage dealt by a skill over its full duration
 */
export function calculateDamagePerCast(
  skill: AnySkill,
  modifiers: DamageModifier[] = [],
): number {
  const debug = false;
  let totalDamage = 0;

  // Sum all direct hits
  if (skill.damage.hits) {
    const hitAffectedBy: DamageModifier['affects'][] = [
      'direct',
      skill.targetType,
    ];

    const hitModifiers = modifiers.filter((m) =>
      hitAffectedBy.includes(m.affects),
    );

    totalDamage += skill.damage.hits.reduce(
      (sum, hit) => sum + applyDamageModifier(hitModifiers, hit.value),
      0,
    );
  }

  // Add DoT damage over full duration
  if (skill.damage.dots) {
    const dotAffectedBy: DamageModifier['affects'][] = [
      'dot',
      skill.targetType,
    ];

    const dotModifiers = modifiers.filter((m) =>
      dotAffectedBy.includes(m.affects),
    );

    if (debug) {
      logger.info('Modifiers: ', dotModifiers);
    }

    for (const dot of skill.damage.dots) {
      if (debug) {
        logger.info('Dot: ', dot);
      }
      // If interval is not defined then we only know the total damage done over
      // the duration which is equivalent to interval = duration
      const interval = dot.interval ?? dot.duration;
      const ticks = Math.floor(dot.duration / interval);
      const increasePerTick = dot.increasePerTick ?? 0;
      const flatIncreasePerTick = dot.flatIncreasePerTick ?? 0;

      if (debug) {
        logger.info('Meta', {
          interval,
          ticks,
          increasePerTick,
          flatIncreasePerTick,
        });
      }

      for (let i = 0; i < ticks; i++) {
        const percentageMultiplier = 1 + i * increasePerTick;
        const flatIncrease = i * flatIncreasePerTick;
        const tickDamage = dot.value * percentageMultiplier + flatIncrease;
        if (debug) {
          logger.info('Tick: ', applyDamageModifier(dotModifiers, tickDamage));
        }
        totalDamage += dot.ignoresModifier
          ? tickDamage
          : applyDamageModifier(dotModifiers, tickDamage);
      }
    }
  }

  if (debug) {
    logger.log('Damage: ', totalDamage);
  }

  return totalDamage;
}

// Base crit stats (assumed from gear/CP) - could be made configurable
const BASE_CRIT_CHANCE = 0.15; // 15% base crit chance
const BASE_CRIT_DAMAGE = 0.5; // 50% base crit damage

/**
 * Check if a bonus applies to a skill and return the applicable bonus value
 */
function getApplicableBonus(
  _skill: AnySkill,
  bonus: PassiveBonus,
  passiveSkillLine: string,
  skillLineCounts: SkillLineCounts,
): number {
  // Handle buffId type
  if ('buffId' in bonus) {
    // Minor Savagery = +10% crit damage
    if (bonus.buffId === 'Minor Savagery') {
      const skillCount = skillLineCounts[passiveSkillLine] ?? 0;
      if (skillCount === 0) return 0;
      // Convert crit damage to expected damage: crit_chance * crit_damage_increase
      return BASE_CRIT_CHANCE * 0.1;
    }
    return 0;
  }

  // Get base value and apply multiplier
  const skillCount = skillLineCounts[passiveSkillLine] ?? 0;
  let multipliedValue = 0;

  switch (bonus.multiplier) {
    case 'skillLine':
    case 'abilitySlotted':
      multipliedValue = skillCount > 0 ? bonus.value : 0;
      break;
    case 'abilitySlottedCount':
      multipliedValue = bonus.value * skillCount;
      break;
    default:
      multipliedValue = skillCount > 0 ? bonus.value : 0;
  }

  if (multipliedValue === 0) return 0;

  // Convert stat types to expected damage bonus
  switch (bonus.type) {
    case 'critical-chance':
      // More crit chance = more expected damage: crit_chance_increase * crit_damage
      return multipliedValue * (1 + BASE_CRIT_DAMAGE);
    case 'critical-damage':
      // More crit damage = more expected damage: crit_chance * crit_damage_increase
      return BASE_CRIT_CHANCE * multipliedValue;
    case 'duration':
    case 'max-stamina':
    case 'max-magicka':
      // These don't directly affect damage (could be expanded later)
      return 0;
    default:
      return 0;
  }
}

/**
 * Calculate total passive bonus percentage for a skill
 */
export function calculatePassiveBonus(
  skill: AnySkill,
  passives: AnyPassiveSkill[],
  skillLineCounts: SkillLineCounts,
): number {
  let totalBonus = 0;

  for (const passive of passives) {
    for (const bonus of passive.bonuses) {
      totalBonus += getApplicableBonus(
        skill,
        bonus,
        passive.skillLine,
        skillLineCounts,
      );
    }
  }

  return totalBonus;
}

/**
 * Get passives that apply to a skill based on its skill line
 */
export function getApplicablePassives(
  skill: AnySkill,
  allClassPassives: ClassPassiveSkill[],
  allWeaponPassives: WeaponPassiveSkill[],
): AnyPassiveSkill[] {
  const isClassSkill = 'esoClass' in skill;

  if (isClassSkill) {
    return allClassPassives.filter((p) => p.skillLine === skill.skillLine);
  } else {
    return allWeaponPassives.filter((p) => p.skillLine === skill.skillLine);
  }
}
