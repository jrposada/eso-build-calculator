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

  if (skill.damage.dot) {
    return 'dot';
  }

  if (
    !!skill.damage.hits?.length &&
    skill.damage.hits.some((hit) => Boolean(hit.value)) &&
    !skill.damage.dot &&
    !skill.channelTime
  ) {
    return 'instant';
  }

  return 'unknown';
}

/**
 * Get the duration of a skill (dotDuration or channelTime)
 */
export function getSkillDuration(skill: AnySkill): number {
  if (skill.damage.dotDuration) {
    return skill.damage.dotDuration;
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
  if (skill.damage.dot && skill.damage.dotDuration) {
    // If interval is not defined then we only know the total damage done over
    // the duration which is equivalent to interval = duration
    const interval = skill.damage.dotInterval ?? skill.damage.dotDuration;
    const ticks = Math.floor(skill.damage.dotDuration / interval);
    const increasePerTick = skill.damage.dotIncreasePerTick ?? 0;

    if (increasePerTick > 0) {
      for (let i = 0; i < ticks; i++) {
        totalDamage += skill.damage.dot * (1 + i * increasePerTick);
      }
    } else {
      totalDamage += skill.damage.dot * ticks;
    }
  }

  return totalDamage;
}
