import { parentPort } from 'worker_threads';

import { BonusData } from '../data/bonuses/types';
import { SkillData } from '../data/skills/types';
import { generateGroupedCombinationsIterator } from '../infrastructure/combinatorics';
import { Build, BUILD_CONSTRAINTS } from '../models/build';
import { Skill } from '../models/skill';

/**
 * Payload sent to worker for evaluation
 */
export interface WorkerPayload {
  /** Worker identifier for progress reporting */
  workerId: number;
  /** Batch of champion point combinations to evaluate */
  championPointBatches: BonusData[][];
  /** Pre-filtered skills to generate combinations from */
  allowedSkills: SkillData[];
  /** How often to report progress (number of evaluations) */
  progressInterval?: number;
}

/**
 * Progress message sent from worker to main thread
 */
export interface WorkerProgress {
  workerId: number;
  evaluated: number;
  currentBestDamage: number | null;
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

/**
 * Generate skill combinations from pre-filtered allowed skills
 */
function* generateSkillCombinations(
  allowedSkills: SkillData[],
): Generator<Skill[], void, unknown> {
  const skills = allowedSkills.map(Skill.fromData);

  // Yield skill combinations lazily using iterator
  // Group by baseSkillName to avoid invalid combinations with multiple morphs of the same skill
  yield* generateGroupedCombinationsIterator(
    skills,
    BUILD_CONSTRAINTS.maxSkills,
    (skill) => skill.baseSkillName,
  );
}

/**
 * Worker function that evaluates a batch of champion point combinations
 * and returns the best build found.
 */
export default function evaluateBatch(
  payload: WorkerPayload,
): WorkerResult | null {
  const {
    workerId,
    championPointBatches,
    allowedSkills,
    progressInterval = 200_000,
  } = payload;

  let bestBuild: Build | null = null;
  let evaluatedCount = 0;

  for (const championPointCombination of championPointBatches) {
    for (const skillCombination of generateSkillCombinations(allowedSkills)) {
      const build = new Build(skillCombination, championPointCombination);
      if (build.isBetterThan(bestBuild)) {
        bestBuild = build;
      }
      evaluatedCount++;

      // Report progress periodically
      if (evaluatedCount % progressInterval === 0) {
        parentPort?.postMessage({
          workerId,
          evaluated: evaluatedCount,
          currentBestDamage: bestBuild?.totalDamagePerCast ?? null,
        } satisfies WorkerProgress);
      }
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
