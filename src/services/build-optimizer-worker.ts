import { BonusData } from '../data/bonuses/types';
import { ClassSkillLineName, WeaponSkillLineName } from '../data/skills';
import { SkillData } from '../data/skills/types';
import { ClassName } from '../data/types';
import { generateGroupedCombinationsIterator } from '../infrastructure/combinatorics';
import { Build, BUILD_CONSTRAINTS } from '../models/build';
import { Skill } from '../models/skill';
import { SkillsService } from './skills-service';

/**
 * Payload sent to worker for evaluation
 */
export interface WorkerPayload {
  /** Batch of champion point combinations to evaluate */
  championPointBatches: BonusData[][];
  /** All skill data needed for combination generation */
  skillData: SkillData[];
  /** Class skill line combinations to iterate */
  classSkillLineNameCombinations: ClassSkillLineName[][];
  /** Weapon skill line combinations to iterate */
  weaponSkillLineNameCombinations: WeaponSkillLineName[][];
  /** Optional class filter */
  className?: ClassName;
}

/**
 * Result returned from worker
 */
export interface WorkerResult {
  /** Serialized skill data of the best build */
  skills: SkillData[];
  /** Champion points of the best build */
  championPoints: BonusData[];
  /** Total damage of the best build */
  totalDamage: number;
  /** Number of builds evaluated in this batch */
  evaluatedCount: number;
}

const SKILL_OPTIONS = {
  excludeBaseSkills: true,
  excludeUltimates: true,
  excludeNonDamaging: true,
};

/**
 * Generate skill combinations from skill data and skill line combinations
 */
function* generateSkillCombinations(
  skillsService: SkillsService,
  classSkillLineNameCombinations: ClassSkillLineName[][],
  weaponSkillLineNameCombinations: WeaponSkillLineName[][],
  className?: ClassName,
): Generator<Skill[], void, unknown> {
  // Cross-product of class and weapon skill line combinations
  for (const classSkillLineCombination of classSkillLineNameCombinations) {
    if (className) {
      const hasRequiredClass = classSkillLineCombination.some(
        (line) => SkillsService.getClassName(line) === className,
      );
      if (!hasRequiredClass) continue;
    }

    for (const weaponSkillLineCombination of weaponSkillLineNameCombinations) {
      const allCombinationPossibleSkills = [
        ...classSkillLineCombination.flatMap((line) =>
          skillsService
            .getSkillsBySkillLineName(line, SKILL_OPTIONS)
            .map(Skill.fromData),
        ),
        ...weaponSkillLineCombination.flatMap((line) =>
          skillsService
            .getSkillsBySkillLineName(line, SKILL_OPTIONS)
            .map(Skill.fromData),
        ),
      ];

      // Yield skill combinations lazily using iterator
      // Group by baseSkillName to avoid invalid combinations with multiple morphs of the same skill
      yield* generateGroupedCombinationsIterator(
        allCombinationPossibleSkills,
        BUILD_CONSTRAINTS.maxSkills,
        (skill) => skill.baseSkillName,
      );
    }
  }
}

/**
 * Worker function that evaluates a batch of champion point combinations
 * and returns the best build found.
 */
export default function evaluateBatch(
  payload: WorkerPayload,
): WorkerResult | null {
  const {
    championPointBatches,
    skillData,
    classSkillLineNameCombinations,
    weaponSkillLineNameCombinations,
    className,
  } = payload;

  // Create SkillsService instance with the provided skill data
  const skillsService = new SkillsService(skillData);

  let bestBuild: Build | null = null;
  let evaluatedCount = 0;

  for (const championPointCombination of championPointBatches) {
    for (const skillCombination of generateSkillCombinations(
      skillsService,
      classSkillLineNameCombinations,
      weaponSkillLineNameCombinations,
      className,
    )) {
      const build = new Build(skillCombination, championPointCombination);
      if (build.isBetterThan(bestBuild)) {
        bestBuild = build;
      }
      evaluatedCount++;
    }
  }

  if (!bestBuild) {
    return null;
  }

  // Convert skills back to SkillData for serialization
  const skillsAsData: SkillData[] = bestBuild.skills.map((skill) => ({
    name: skill.name,
    baseSkillName: skill.baseSkillName,
    className: skill.className,
    skillLine: skill.skillLine,
    damage: skill.damage,
    damageType: skill.damageType,
    targetType: skill.targetType,
    resource: skill.resource,
    channelTime: skill.channelTime,
  }));

  return {
    skills: skillsAsData,
    championPoints: [...bestBuild.championPoints],
    totalDamage: bestBuild.totalDamagePerCast,
    evaluatedCount,
  };
}
