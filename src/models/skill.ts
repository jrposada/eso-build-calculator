import type {
  DamageType,
  DotDamage,
  Resource,
  SkillClassName,
  SkillData,
  TargetType,
} from '../data/skills/types';
import { DamageModifier } from './modifier';

export type SkillType = 'class' | 'weapon';
export type SkillMechanic = 'dot' | 'instant' | 'channeled' | 'unknown';

interface SkillDamage {
  readonly hits?: ReadonlyArray<{
    readonly value: number;
    readonly delay?: number;
  }>;
  readonly dots?: ReadonlyArray<DotDamage>;
}

export class Skill {
  readonly name: string;
  readonly baseSkillName: string;
  readonly damage: SkillDamage;
  readonly damageType: DamageType;
  readonly targetType: TargetType;
  readonly resource: Resource;
  readonly channelTime?: number;
  readonly skillLine: string;
  readonly skillType: SkillType;
  readonly className: SkillClassName;

  private constructor(data: SkillData) {
    this.name = data.name;
    this.baseSkillName = data.baseSkillName;
    this.damage = Object.freeze({
      hits: data.damage.hits
        ? Object.freeze(data.damage.hits.map((h) => Object.freeze({ ...h })))
        : undefined,
      dots: data.damage.dots
        ? Object.freeze(data.damage.dots.map((d) => Object.freeze({ ...d })))
        : undefined,
    });
    this.damageType = data.damageType;
    this.targetType = data.targetType;
    this.resource = data.resource;
    this.channelTime = data.channelTime;
    this.skillLine = data.skillLine;
    this.className = data.className;
    this.skillType = data.className === 'Weapon' ? 'weapon' : 'class';

    Object.freeze(this);
  }

  static fromData(data: SkillData): Skill {
    return new Skill(data);
  }

  static fromDataArray(data: SkillData[]): Skill[] {
    return data.map((d) => Skill.fromData(d));
  }

  get source(): string {
    return this.className;
  }

  get mechanic(): SkillMechanic {
    if (this.channelTime) {
      return 'channeled';
    }

    if (this.damage.dots?.length) {
      return 'dot';
    }

    if (
      !!this.damage.hits?.length &&
      this.damage.hits.some((hit) => Boolean(hit.value)) &&
      !this.damage.dots?.length &&
      !this.channelTime
    ) {
      return 'instant';
    }

    return 'unknown';
  }

  get duration(): number {
    if (this.damage.dots?.length) {
      return Math.max(
        ...this.damage.dots.map((dot) => dot.duration + (dot.delay ?? 0)),
      );
    }
    if (this.channelTime) {
      return this.channelTime;
    }
    return 0;
  }

  get isUltimate(): boolean {
    return this.resource === 'ultimate';
  }

  calculateDamagePerCast(modifiers: DamageModifier[] = []): number {
    let totalDamage = 0;

    // Sum all direct hits
    if (this.damage.hits) {
      const hitAffectedBy: DamageModifier['affects'][] = [
        'direct',
        this.targetType,
      ];

      const hitModifiers = modifiers.filter((m) =>
        hitAffectedBy.includes(m.affects),
      );

      totalDamage += this.damage.hits.reduce(
        (sum, hit) => sum + this.applyDamageModifier(hitModifiers, hit.value),
        0,
      );
    }

    // Add DoT damage over full duration
    if (this.damage.dots) {
      const dotAffectedBy: DamageModifier['affects'][] = [
        'dot',
        this.targetType,
      ];

      const dotModifiers = modifiers.filter((m) =>
        dotAffectedBy.includes(m.affects),
      );

      for (const dot of this.damage.dots) {
        // If interval is not defined then we only know the total damage done over
        // the duration which is equivalent to interval = duration
        const interval = dot.interval ?? dot.duration;
        const ticks = Math.floor(dot.duration / interval);
        const increasePerTick = dot.increasePerTick ?? 0;
        const flatIncreasePerTick = dot.flatIncreasePerTick ?? 0;

        for (let i = 0; i < ticks; i++) {
          const percentageMultiplier = 1 + i * increasePerTick;
          const flatIncrease = i * flatIncreasePerTick;
          const tickDamage = dot.value * percentageMultiplier + flatIncrease;
          totalDamage += dot.ignoresModifier
            ? tickDamage
            : this.applyDamageModifier(dotModifiers, tickDamage);
        }
      }
    }

    return totalDamage;
  }

  toString(): string {
    return `${this.name} (${this.source}/${this.skillLine})`;
  }

  private applyDamageModifier(
    modifiers: DamageModifier[],
    value: number,
  ): number {
    const totalModifier = modifiers.reduce(
      (sum, modifier) => sum + modifier.value * modifier.maxLevel,
      0,
    );
    return value * (1 + totalModifier);
  }
}
