import { logger } from '../infrastructure';
import { DamageModifier } from '../models/modifier';
import {
  AnyPassiveSkill,
  ClassPassiveSkill,
  PassiveBonus,
  WeaponPassiveSkill,
} from '../models/passive';
import { ClassSkill, DamageType, WeaponSkill } from '../models/skill';

export type AnySkill = ClassSkill | WeaponSkill;

export type SkillMechanic = 'dot' | 'instant' | 'channeled';

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

// Mapping from status effects to damage types that apply them
const STATUS_EFFECT_TO_DAMAGE_TYPE: Record<string, DamageType> = {
  Burning: 'flame',
  Poisoned: 'poison',
  Chilled: 'frost',
  Concussed: 'shock',
};

/**
 * Check if a bonus applies to a skill and return the applicable bonus value
 */
function getApplicableBonus(
  skill: AnySkill,
  bonus: PassiveBonus,
  passiveSkillLine: string,
): number {
  switch (bonus.type) {
    case 'damage':
      // General damage bonus applies to all skills
      return bonus.value;

    case 'damageType':
      // Only applies if skill's damage type matches
      return bonus.damageTypes?.includes(skill.damageType) ? bonus.value : 0;

    case 'dot':
      // Only applies if skill has DoTs
      return skill.damage.dots?.length ? bonus.value : 0;

    case 'direct':
      // Only applies if skill has direct hits
      return skill.damage.hits?.length ? bonus.value : 0;

    case 'skillLine':
      // Only applies if skill belongs to the same skill line as the passive
      return skill.skillLine === passiveSkillLine ? bonus.value : 0;

    case 'statusEffect':
      // Status effects apply to skills that can apply them (based on damage type)
      if (!bonus.statusEffects) return 0;
      return bonus.statusEffects.some(
        (se) => STATUS_EFFECT_TO_DAMAGE_TYPE[se] === skill.damageType,
      )
        ? bonus.value
        : 0;

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
): number {
  let totalBonus = 0;

  for (const passive of passives) {
    for (const bonus of passive.bonuses) {
      totalBonus += getApplicableBonus(skill, bonus, passive.skillLine);
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
