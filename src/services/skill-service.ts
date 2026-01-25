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

/**
 * Calculate total damage dealt by a skill over its full duration
 */
export function calculateDamagePerCast(skill: AnySkill): number {
  let totalDamage = 0;

  // Sum all direct hits
  if (skill.damage.hits) {
    totalDamage += skill.damage.hits.reduce((sum, hit) => sum + hit.value, 0);
  }

  // Add DoT damage over full duration
  if (skill.damage.dots) {
    for (const dot of skill.damage.dots) {
      // If interval is not defined then we only know the total damage done over
      // the duration which is equivalent to interval = duration
      const interval = dot.interval ?? dot.duration;
      const ticks = Math.floor(dot.duration / interval);
      const increasePerTick = dot.increasePerTick ?? 0;
      const flatIncreasePerTick = dot.flatIncreasePerTick ?? 0;

      for (let i = 0; i < ticks; i++) {
        const percentageMultiplier = 1 + i * increasePerTick;
        const flatIncrease = i * flatIncreasePerTick;
        totalDamage += dot.value * percentageMultiplier + flatIncrease;
      }
    }
  }

  return totalDamage;
}
