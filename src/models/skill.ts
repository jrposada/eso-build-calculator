export type EsoClass =
  | 'Dragonknight'
  | 'Sorcerer'
  | 'Nightblade'
  | 'Warden'
  | 'Necromancer'
  | 'Templar'
  | 'Arcanist';

export type DragonknightSkillLineName =
  | 'ArdentFlame'
  | 'DraconicPower'
  | 'EarthenHeart';

export type SorcererSkillLineName =
  | 'DarkMagic'
  | 'DaedricSummoning'
  | 'StormCalling';

export type NightbladeSkillLineName = 'Assassination' | 'Shadow' | 'Siphoning';

export type WardenSkillLineName =
  | 'AnimalCompanions'
  | 'GreenBalance'
  | 'WintersEmbrace';

export type NecromancerSkillLineName =
  | 'GraveLord'
  | 'BoneTyrant'
  | 'LivingDeath';

export type TemplarSkillLineName =
  | 'AedricSpear'
  | 'DawnsWrath'
  | 'RestoringLight';

export type ArcanistSkillLineName =
  | 'CurativeRuneforms'
  | 'SoldierOfApocrypha'
  | 'HeraldOfTheTome';

export type WeaponSkillLineName =
  | 'Bow'
  | 'TwoHanded'
  | 'DestructionStaff'
  | 'DualWield';

export type ClassSkillLine =
  | DragonknightSkillLineName
  | SorcererSkillLineName
  | NightbladeSkillLineName
  | WardenSkillLineName
  | NecromancerSkillLineName
  | TemplarSkillLineName
  | ArcanistSkillLineName;

export type SkillLine = ClassSkillLine | WeaponSkillLineName;

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

interface BaseSkill {
  name: string;
  baseSkillName: string; // The base skill name for grouping (base, morph1, morph2)
  damage: {
    hits?: Array<{ value: number; delay?: number }>;
    dots?: DotDamage[];
  };
  damageType: DamageType;
  targetType: TargetType;
  resource: Resource;
  channelTime?: number;
}

export interface ClassSkill<
  TEsoClass extends EsoClass = EsoClass,
  TSkillLine extends ClassSkillLine = ClassSkillLine,
> extends BaseSkill {
  esoClass: TEsoClass;
  skillLine: TSkillLine;
}

export interface WeaponSkill<
  TSkillLine extends WeaponSkillLineName = WeaponSkillLineName,
> extends BaseSkill {
  skillLine: TSkillLine;
}

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
  readonly skillLine: ClassSkillLine | WeaponSkillLineName;
  readonly skillType: SkillType;
  readonly esoClass?: EsoClass;

  private constructor(data: ClassSkill | WeaponSkill) {
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

    if ('esoClass' in data) {
      this.skillType = 'class';
      this.esoClass = data.esoClass;
    } else {
      this.skillType = 'weapon';
    }

    Object.freeze(this);
  }

  static fromData(data: ClassSkill | WeaponSkill): Skill {
    return new Skill(data);
  }

  static fromDataArray(data: (ClassSkill | WeaponSkill)[]): Skill[] {
    return data.map((d) => Skill.fromData(d));
  }

  isClassSkill(): this is Skill & { esoClass: EsoClass } {
    return this.skillType === 'class';
  }

  isWeaponSkill(): boolean {
    return this.skillType === 'weapon';
  }

  get source(): string {
    return this.esoClass ?? 'Weapon';
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

  toData(): ClassSkill | WeaponSkill {
    const baseData = {
      name: this.name,
      baseSkillName: this.baseSkillName,
      damage: {
        hits: this.damage.hits ? [...this.damage.hits] : undefined,
        dots: this.damage.dots ? [...this.damage.dots] : undefined,
      },
      damageType: this.damageType,
      targetType: this.targetType,
      resource: this.resource,
      channelTime: this.channelTime,
      skillLine: this.skillLine,
    };

    if (this.isClassSkill()) {
      return {
        ...baseData,
        esoClass: this.esoClass,
        skillLine: this.skillLine as ClassSkillLine,
      } as ClassSkill;
    }

    return {
      ...baseData,
      skillLine: this.skillLine as WeaponSkillLineName,
    } as WeaponSkill;
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
