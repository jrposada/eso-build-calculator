import { DamageModifier } from '../models/modifier';
import { ClassSkill, WeaponSkill } from '../models/skill';

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
      console.log('Modifiers: ', dotModifiers);
    }

    for (const dot of skill.damage.dots) {
      if (debug) {
        console.log('Dot: ', dot);
      }
      // If interval is not defined then we only know the total damage done over
      // the duration which is equivalent to interval = duration
      const interval = dot.interval ?? dot.duration;
      const ticks = Math.floor(dot.duration / interval);
      const increasePerTick = dot.increasePerTick ?? 0;
      const flatIncreasePerTick = dot.flatIncreasePerTick ?? 0;

      if (debug) {
        console.log('Meta', {
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
          console.log('Tick: ', applyDamageModifier(dotModifiers, tickDamage));
        }
        totalDamage += dot.ignoresModifier
          ? tickDamage
          : applyDamageModifier(dotModifiers, tickDamage);
      }
    }
  }

  if (debug) {
    console.log('Damage: ', totalDamage);
  }

  return totalDamage;
}
