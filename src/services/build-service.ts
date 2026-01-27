import {
  getClassPassivesBySkillLine,
  getWeaponPassivesBySkillLine,
} from '../data/passives';
import { ALL_SKILLS } from '../data/skills';
import { Build } from '../models/build';
import { DamageModifier } from '../models/modifier';
import { AnyPassiveSkill } from '../models/passive';
import {
  ClassSkillLine,
  EsoClass,
  Skill,
  WeaponSkillLineName,
} from '../models/skill';
import {
  AnySkill,
  calculatePassiveBonus,
  SkillLineCounts,
} from './skill-service';

export interface ProcessedSkill {
  skill: AnySkill;
  name: string;
  baseSkillName: string;
  skillLine: string;
  source: string;
  baseDamage: number;
}

export interface SkillLineCombination {
  classLines: ClassSkillLine[];
  weaponLines: WeaponSkillLineName[];
}

export class BuildService {
  private readonly skills: AnySkill[];

  constructor(skills?: AnySkill[]) {
    this.skills = skills ?? ALL_SKILLS;
  }

  /**
   * Preprocess skills: filter ultimates and deduplicate by baseSkillName (keep best morph for given modifiers)
   * Note: We use base damage (without passives) for deduplication since passive availability
   * depends on which skill lines end up being selected
   */
  preprocessSkills(modifiers: DamageModifier[]): ProcessedSkill[] {
    // Filter out ultimates
    const nonUltimates = this.skills.filter(
      (skill) => skill.resource !== 'ultimate',
    );

    // Calculate base damage (without passives) and create processed skill data
    const processed = nonUltimates.map((skill) => {
      const skillInstance = Skill.fromData(skill);
      const baseDamage = skillInstance.calculateDamagePerCast(modifiers);
      return {
        skill,
        name: skill.name,
        baseSkillName: skill.baseSkillName,
        skillLine: skill.skillLine,
        source: skillInstance.source,
        baseDamage,
      };
    });

    // Filter to only damaging skills
    const damagingSkills = processed.filter((s) => s.baseDamage > 0);

    // Group by baseSkillName and pick highest base damage morph
    const skillsByBase = new Map<string, ProcessedSkill>();
    for (const skill of damagingSkills) {
      const key = `${skill.source}-${skill.baseSkillName}`;
      const existing = skillsByBase.get(key);
      if (!existing || skill.baseDamage > existing.baseDamage) {
        skillsByBase.set(key, skill);
      }
    }

    return Array.from(skillsByBase.values());
  }

  /**
   * Count skills per skill line from processed skills
   */
  getSkillCountByLine(skills: ProcessedSkill[]): Map<string, number> {
    const skillCountByLine = new Map<string, number>();
    for (const skill of skills) {
      skillCountByLine.set(
        skill.skillLine,
        (skillCountByLine.get(skill.skillLine) ?? 0) + 1,
      );
    }
    return skillCountByLine;
  }

  /**
   * Get all passives that apply based on selected skill lines
   */
  getPassivesForSkillLines(
    classSkillLines: ClassSkillLine[],
    weaponSkillLines: WeaponSkillLineName[],
  ): AnyPassiveSkill[] {
    const passives: AnyPassiveSkill[] = [];

    for (const skillLine of classSkillLines) {
      passives.push(...getClassPassivesBySkillLine(skillLine));
    }
    for (const skillLine of weaponSkillLines) {
      passives.push(...getWeaponPassivesBySkillLine(skillLine));
    }

    return passives;
  }

  /**
   * Calculate damage for a skill with applicable passives
   */
  calculateSkillDamage(
    skill: AnySkill,
    modifiers: DamageModifier[],
    passives: AnyPassiveSkill[],
    skillLineCounts: SkillLineCounts,
  ): number {
    const skillInstance = Skill.fromData(skill);
    const baseDamage = skillInstance.calculateDamagePerCast(modifiers);
    const passiveBonus = calculatePassiveBonus(
      skill,
      passives,
      skillLineCounts,
    );
    return baseDamage * (1 + passiveBonus);
  }

  /**
   * Calculate total damage for a set of skills with given passives
   */
  calculateTotalDamage(
    skills: AnySkill[],
    modifiers: DamageModifier[],
    passives: AnyPassiveSkill[],
    skillLineCounts: SkillLineCounts,
  ): number {
    return skills.reduce((total, skill) => {
      return (
        total +
        this.calculateSkillDamage(skill, modifiers, passives, skillLineCounts)
      );
    }, 0);
  }

  /**
   * Count skills per skill line from a set of skills
   */
  countSkillsPerLine(skills: AnySkill[]): SkillLineCounts {
    const counts: SkillLineCounts = {};
    for (const skill of skills) {
      counts[skill.skillLine] = (counts[skill.skillLine] ?? 0) + 1;
    }
    return counts;
  }

  /**
   * Creates a Build from selected skills and configuration
   */
  createBuild(
    selectedSkills: AnySkill[],
    modifiers: DamageModifier[],
    passives: AnyPassiveSkill[],
    skillLineCombination: SkillLineCombination,
    requiredClass?: EsoClass,
  ): Build {
    return new Build(
      selectedSkills,
      passives,
      modifiers,
      skillLineCombination.classLines,
      skillLineCombination.weaponLines,
      requiredClass,
    );
  }

  /**
   * Compare two builds and return true if candidate is better than current
   */
  isBetterBuild(candidate: Build, current: Build | null): boolean {
    if (!current) return true;
    return candidate.totalDamagePerCast > current.totalDamagePerCast;
  }
}
