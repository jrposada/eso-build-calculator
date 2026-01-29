import os from 'os';
import path from 'path';
import Piscina from 'piscina';

import { CHAMPION_POINTS } from '../data/bonuses/champion-points/champion-points';
import { ClassSkillLineName, WeaponSkillLineName } from '../data/skills';
import { SkillData } from '../data/skills/types';
import { ClassName } from '../data/types';
import { logger, table } from '../infrastructure';
import { generateCombinations } from '../infrastructure/combinatorics';
import { Build, BUILD_CONSTRAINTS } from '../models/build';
import { Skill } from '../models/skill';
import { countTotalSkillCombinations } from './build-optimizer-common';
import type { WorkerPayload, WorkerResult } from './build-optimizer-worker';
import {
  CLASS_SKILL_LINES_NAMES,
  SkillsService,
  WEAPON_SKILL_LINE_NAMES,
} from './skills-service';

function getDefaultParallelism(): number {
  return Math.max(1, Math.floor(os.cpus().length / 2));
}

interface BuildOptimizerOptions {
  verbose?: boolean;
  className?: ClassName;
  skills?: SkillData[];
  workers?: number;
  threads?: number;
}

class BuildOptimizer {
  private readonly skillsService: SkillsService;
  private readonly className?: ClassName;
  private readonly verbose: boolean;
  private readonly workers: number;
  private readonly threads: number;

  constructor(options?: BuildOptimizerOptions) {
    this.skillsService = new SkillsService(options?.skills);
    this.className = options?.className;
    this.verbose = options?.verbose ?? false;
    this.workers = options?.workers ?? getDefaultParallelism();
    this.threads = options?.threads ?? getDefaultParallelism();
  }

  /**
   * Find the optimal build that maximizes total damage per cast
   * Uses exhaustive enumeration of skill line combinations with parallel workers
   */
  async findOptimalBuild(): Promise<Build | null> {
    const championPointCombinations = generateCombinations(
      CHAMPION_POINTS,
      BUILD_CONSTRAINTS.maxModifiers,
    );

    // Pre-compute skill line combinations for workers
    const classSkillLineNameCombinations: ClassSkillLineName[][] =
      generateCombinations(
        CLASS_SKILL_LINES_NAMES,
        BUILD_CONSTRAINTS.maxClassSkillLines,
      );

    const weaponSkillLineNameCombinations: WeaponSkillLineName[][] =
      generateCombinations(
        WEAPON_SKILL_LINE_NAMES,
        BUILD_CONSTRAINTS.maxWeaponSkillLines,
      );

    if (this.verbose) {
      logger.info(
        `Generated ${(classSkillLineNameCombinations.length * weaponSkillLineNameCombinations.length).toLocaleString()} skill line combinations.`,
      );
    }

    // Determine worker script path based on whether we're in dist or src
    const workerPath = __filename.endsWith('.ts')
      ? path.resolve(__dirname, 'build-optimizer-worker.ts')
      : path.resolve(__dirname, 'build-optimizer-worker.js');

    // Create worker pool
    const pool = new Piscina({
      filename: workerPath,
      maxThreads: this.threads,
    });

    // Split champion point combinations into batches for workers
    const batchSize = Math.ceil(
      championPointCombinations.length / this.workers,
    );
    const batches: WorkerPayload[] = [];

    for (let i = 0; i < championPointCombinations.length; i += batchSize) {
      const batch = championPointCombinations.slice(i, i + batchSize);
      batches.push({
        championPointBatches: batch,
        skillData: this.skillsService['skills'],
        classSkillLineNameCombinations,
        weaponSkillLineNameCombinations,
        className: this.className,
      });
    }

    logger.info(`Starting ${batches.length} worker(s)...`);

    if (this.verbose) {
      // Calculate total skill combinations across all skill line combinations
      const totalSkillCombinations = countTotalSkillCombinations(
        this.skillsService,
        classSkillLineNameCombinations,
        weaponSkillLineNameCombinations,
        this.className,
      );

      const tableData: string[][] = [];
      let totalCombinations = 0;

      for (let i = 0; i < batches.length; i++) {
        const batch = batches[i]!;
        const batchCombinations =
          batch.championPointBatches.length * totalSkillCombinations;
        totalCombinations += batchCombinations;
        tableData.push([
          `Worker ${i + 1}`,
          batch.championPointBatches.length.toLocaleString(),
          totalSkillCombinations.toLocaleString(),
          batchCombinations.toLocaleString(),
        ]);
      }

      logger.info(
        table(tableData, {
          title: 'Worker Distribution',
          columns: [
            { header: 'Worker', width: 10 },
            { header: 'CP Batches', width: 12, align: 'right' },
            { header: 'Skill Combos', width: 25, align: 'right' },
            { header: 'Total', width: 25, align: 'right' },
          ],
          footer: `Total combinations: ${totalCombinations.toLocaleString()}`,
        }),
      );
    }

    // Submit all batches to worker pool
    const results = await Promise.all(
      batches.map(
        (payload) => pool.run(payload) as Promise<WorkerResult | null>,
      ),
    );

    // Destroy the pool
    await pool.destroy();

    // Find the best build across all worker results
    let bestResult: WorkerResult | null = null;
    let totalEvaluated = 0;

    for (const result of results) {
      if (result) {
        totalEvaluated += result.evaluatedCount;
        if (!bestResult || result.totalDamage > bestResult.totalDamage) {
          bestResult = result;
        }
      }
    }

    if (this.verbose) {
      logger.info(`Evaluated ${totalEvaluated} total build combinations.`);
    }

    if (!bestResult) {
      return null;
    }

    // Reconstruct the best Build from the worker result
    const skills = bestResult.skills.map(Skill.fromData);
    return new Build(skills, bestResult.championPoints);
  }
}

export { BuildOptimizer };
