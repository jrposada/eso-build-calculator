import type {
  DamageType,
  Resource,
  SkillDamage,
  SkillData,
  TargetType,
} from '../data/skills/types';
import { ClassName } from '../data/types';
import { DamageModifier } from './modifier';

export type SkillType = 'class' | 'weapon';
export type SkillMechanic = 'dot' | 'instant' | 'channeled' | 'unknown';

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
  readonly className: ClassName;

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
    const lines: string[] = [];

    lines.push('═'.repeat(60));
    lines.push(`  ${this.name}`);
    lines.push('═'.repeat(60));
    lines.push('');
    lines.push('  Basic Info');
    lines.push('  ' + '─'.repeat(56));
    lines.push(`  Base Skill:      ${this.baseSkillName}`);
    lines.push(`  Source:          ${this.className}`);
    lines.push(`  Skill Line:      ${this.skillLine}`);
    lines.push(`  Resource:        ${this.resource}`);
    lines.push(`  Damage Type:     ${this.damageType}`);
    lines.push(`  Target Type:     ${this.targetType}`);
    lines.push(`  Mechanic:        ${this.mechanic}`);
    if (this.channelTime !== undefined) {
      lines.push(`  Channel Time:    ${this.channelTime}s`);
    }
    lines.push('');
    lines.push('  Damage');
    lines.push('  ' + '─'.repeat(56));
    if (this.damage.hits && this.damage.hits.length > 0) {
      lines.push(`  Hits:`);
      this.damage.hits.forEach((hit, j) => {
        const delay = hit.delay !== undefined ? ` (delay: ${hit.delay}s)` : '';
        lines.push(`    ${j + 1}. ${hit.value}${delay}`);
      });
    }
    if (this.damage.dots && this.damage.dots.length > 0) {
      lines.push(`  DoTs:`);
      this.damage.dots.forEach((dot, j) => {
        const interval =
          dot.interval !== undefined ? ` every ${dot.interval}s` : '';
        const increase = dot.increasePerTick
          ? ` (+${(dot.increasePerTick * 100).toFixed(0)}%/tick)`
          : '';
        const flatIncrease = dot.flatIncreasePerTick
          ? ` (+${dot.flatIncreasePerTick}/tick)`
          : '';
        lines.push(
          `    ${j + 1}. ${dot.value}${interval} for ${dot.duration}s${increase}${flatIncrease}`,
        );
      });
    }
    lines.push('');
    lines.push('  Calculated');
    lines.push('  ' + '─'.repeat(56));
    lines.push(
      `  Duration:        ${this.duration > 0 ? `${this.duration}s` : 'instant'}`,
    );
    lines.push(
      `  Damage/Cast:     ${this.calculateDamagePerCast().toFixed(0)}`,
    );

    return lines.join('\n');
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
