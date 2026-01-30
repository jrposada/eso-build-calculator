import os from 'os';
import path from 'path';
import Piscina from 'piscina';

import { CHAMPION_POINTS } from '../data/bonuses/champion-points/champion-points';
import { ChampionPointBonus } from '../data/bonuses/champion-points/types';
import {
  ALL_SKILLS,
  ClassClassName,
  WeaponSkillLineName,
} from '../data/skills';
import { SkillData } from '../data/skills/types';
import { batch, logger, table } from '../infrastructure';
import {
  cartesianProduct,
  countCombinations,
  generateCombinations,
} from '../infrastructure/combinatorics';
import { Build, BUILD_CONSTRAINTS } from '../models/build';
import { Skill } from '../models/skill';
import type {
  WorkerPayload,
  WorkerProgress,
  WorkerResult,
} from './build-optimizer-worker';
import { MorphSelector } from './morph-selector';
import {
  CLASS_NAMES,
  SKILL_LINES_NAMES,
  SkillsService,
} from './skills-service';

function getDefaultParallelism(): number {
  return Math.max(1, Math.floor(os.cpus().length / 2));
}

function formatDuration(ms: number): string {
  const seconds = Math.floor(ms / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);

  if (hours > 0) {
    return `${hours}h ${minutes % 60}m`;
  }
  if (minutes > 0) {
    return `${minutes}m ${seconds % 60}s`;
  }
  return `${seconds}s`;
}

interface BuildOptimizerOptions {
  verbose?: boolean;
  requiredClassNames?: ClassClassName[];
  requiredWeapons?: WeaponSkillLineName[];
  forcedMorphs?: string[];
  skills?: SkillData[];
  workers?: number;
}

class BuildOptimizer {
  private readonly skillsService: SkillsService;
  private readonly morphSelector: MorphSelector;
  private readonly verbose: boolean;
  private readonly workers: number;

  private readonly requiredClassNames: ClassClassName[];
  private readonly requiredWeapons: WeaponSkillLineName[];
  private readonly classNames: Set<ClassClassName>;
  private readonly weaponSkillLinesNames: Set<WeaponSkillLineName>;
  private readonly skillNames: Set<string>;

  private readonly championPointCombinations: ChampionPointBonus[][];
  private readonly skillsCombinations;
  private readonly skillCombinationsCount: number;

  // Track progress for each worker
  private readonly workerProgress = new Map<
    number,
    { progressPercent: number; bestDamage: number | null }
  >();

  // Track when optimization started for ETA calculation
  private optimizationStartTime: number | null = null;

  constructor(options?: BuildOptimizerOptions) {
    this.morphSelector = new MorphSelector({
      forcedMorphs: options?.forcedMorphs,
    });

    if (options?.verbose) {
      logger.dim(
        `Total skills before greedy morph selection: ${(options?.skills ?? ALL_SKILLS).length.toLocaleString()}`,
      );
    }

    const skills = this.morphSelector.selectMorphs(
      options?.skills ?? ALL_SKILLS,
    );

    if (options?.verbose) {
      logger.dim(
        `Total skills after greedy morph selection: ${skills.length.toLocaleString()}`,
      );
    }

    this.skillsService = new SkillsService(skills);
    this.requiredClassNames = options?.requiredClassNames ?? [];
    this.requiredWeapons = options?.requiredWeapons ?? [];
    this.verbose = options?.verbose ?? false;
    this.workers = options?.workers ?? getDefaultParallelism();

    this.championPointCombinations = generateCombinations(
      CHAMPION_POINTS,
      BUILD_CONSTRAINTS.championPointCount,
    );

    if (this.verbose) {
      logger.dim(
        `Generated ${this.championPointCombinations.length.toLocaleString()} champion point combinations`,
      );
    }

    this.classNames = new Set<ClassClassName>();
    const classClassNameCombinations = generateCombinations(
      CLASS_NAMES.filter(
        (className): className is ClassClassName => className !== 'Weapon',
      ),
      BUILD_CONSTRAINTS.classSkillLineCount,
    ).filter((combination) => {
      let hasRequiredClass = this.requiredClassNames.length === 0;

      if (this.requiredClassNames.length > 0) {
        hasRequiredClass = this.requiredClassNames.every((requiredClassName) =>
          combination.some((className) => className === requiredClassName),
        );
      }

      if (hasRequiredClass) {
        combination.forEach((className) => this.classNames.add(className));
      }

      return hasRequiredClass;
    });

    if (this.verbose) {
      logger.dim(
        [
          `Generated ${classClassNameCombinations.length.toLocaleString()}`,
          `class combinations using ${this.classNames.size} classes`,
          `(required: ${this.requiredClassNames.sort().join(', ') || 'none'})`,
        ].join(' '),
      );
    }

    const classSkillLineCombinations = classClassNameCombinations.map(
      (classCombination) =>
        classCombination.flatMap((className) =>
          SKILL_LINES_NAMES.filter((skillLine) =>
            SkillsService.isSkillLineFromClass(className, skillLine),
          ),
        ),
    );

    this.weaponSkillLinesNames = new Set<WeaponSkillLineName>();
    const weaponSkillLinesCombinations = generateCombinations(
      SKILL_LINES_NAMES.filter(
        (skillLineName): skillLineName is WeaponSkillLineName =>
          SkillsService.getClass(skillLineName) === 'Weapon',
      ),
      BUILD_CONSTRAINTS.weaponSkillLineCount,
    ).filter((combination) => {
      let hasRequiredWeapon = this.requiredWeapons.length === 0;

      if (this.requiredWeapons.length > 0) {
        hasRequiredWeapon = this.requiredWeapons.every(
          (requiredWeaponSkillLineName) =>
            combination.some(
              (weaponSkillLineName) =>
                weaponSkillLineName === requiredWeaponSkillLineName,
            ),
        );
      }

      if (hasRequiredWeapon) {
        combination.forEach((weaponSkillLineName) =>
          this.weaponSkillLinesNames.add(weaponSkillLineName),
        );
      }

      return hasRequiredWeapon;
    });

    if (this.verbose) {
      logger.dim(
        [
          `Generated ${weaponSkillLinesCombinations.length.toLocaleString()}`,
          `weapon skill line combinations using ${this.weaponSkillLinesNames.size} weapons`,
          `(required: ${this.requiredWeapons.sort().join(', ') || 'none'})`,
        ].join(' '),
      );
    }

    const skillLineCombinations = cartesianProduct(
      classSkillLineCombinations,
      weaponSkillLinesCombinations,
    );

    if (this.verbose) {
      logger.dim(
        [
          `Generated ${skillLineCombinations.length.toLocaleString()}`,
          'total skill line combinations (class + weapon)',
        ].join(' '),
      );
    }

    this.skillNames = new Set<string>();
    this.skillsCombinations = skillLineCombinations.map(
      (skillLineCombination) =>
        skillLineCombination.flatMap((skillLine) => {
          const skillLineSkills = this.skillsService.getSkillsBySkillLine(
            skillLine,
            {
              excludeBaseSkills: true,
              excludeUltimates: true,
              excludeNonDamaging: true,
            },
          );

          skillLineSkills.forEach((skill) => this.skillNames.add(skill.name));

          return skillLineSkills;
        }),
    );

    // Calculate total skill combinations across all skill line combinations
    this.skillCombinationsCount = this.skillsCombinations.reduce(
      (sum, skills) =>
        sum + countCombinations(skills, BUILD_CONSTRAINTS.skillCount),
      0,
    );

    logger.info(this.toString());
  }

  /**
   * Find the optimal build that maximizes total damage per cast
   * Uses parallel workers to evaluate champion point combinations
   */
  async findOptimalBuild(): Promise<Build | null> {
    if (this.workerProgress.size > 0) {
      throw new Error('Optimization already in progress');
    }

    this.optimizationStartTime = Date.now();

    // Determine worker script path based on whether we're in dist or src
    const workerPath = __filename.endsWith('.ts')
      ? path.resolve(__dirname, 'build-optimizer-worker.ts')
      : path.resolve(__dirname, 'build-optimizer-worker.js');

    // Create worker pool
    const pool = new Piscina({
      filename: workerPath,
      maxThreads: this.workers,
    });

    this.registerWorkerProgressHandlers(pool);

    // Split champion point combinations into batches for workers
    const batchSize = Math.ceil(
      this.championPointCombinations.length / this.workers,
    );
    const batches: WorkerPayload[] = batch(
      this.championPointCombinations,
      batchSize,
    ).map((cpBatch, i) => ({
      workerId: i + 1,
      championPointBatches: cpBatch,
      skillsCombinations: this.skillsCombinations,
      totalIterations: cpBatch.length * this.skillCombinationsCount,
    }));

    logger.info(`Starting ${batches.length} worker(s)...`);

    // Submit all batches to worker pool
    const results = await Promise.all(
      batches.map(
        (payload) => pool.run(payload) as Promise<WorkerResult | null>,
      ),
    );

    // Destroy the pool
    await pool.destroy();
    this.workerProgress.clear();
    this.optimizationStartTime = null;

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

    // Clear progress line and show completion summary
    logger.info(
      `Completed: ${totalEvaluated.toLocaleString()} builds evaluated`,
    );

    if (!bestResult) {
      return null;
    }

    // Reconstruct the best Build from the worker result
    const skills = bestResult.skills.map(Skill.fromData);
    return new Build(skills, bestResult.championPoints);
  }

  toString(): string {
    const lines: string[] = [];

    lines.push(
      table(
        [
          ['Skills', BUILD_CONSTRAINTS.skillCount.toString()],
          ['Champion Points', BUILD_CONSTRAINTS.championPointCount.toString()],
          [
            'Class Skill Lines',
            BUILD_CONSTRAINTS.classSkillLineCount.toString(),
          ],
          [
            'Weapon Skill Lines',
            BUILD_CONSTRAINTS.weaponSkillLineCount.toString(),
          ],
          ['Workers', this.workers.toString()],
        ],
        {
          title: 'Configuration',
          columns: [
            { header: 'Constraint', width: 25 },
            { header: 'Value', width: 10, align: 'right' },
          ],
        },
      ),
    );

    const usedClassesStr = [...this.classNames].sort().join(', ') || 'None';
    const requiredClassesStr =
      this.requiredClassNames.sort().join(', ') || 'None';
    const usedWeaponsStr =
      [...this.weaponSkillLinesNames].sort().join(', ') || 'None';
    const requiredWeaponsStr = this.requiredWeapons.sort().join(', ') || 'None';
    const classesColumnWidth = Math.max(
      'Classes'.length,
      usedClassesStr.length,
      requiredClassesStr.length,
    );
    lines.push(
      table(
        [
          ['Used Classes', usedClassesStr],
          ['Required Classes', requiredClassesStr],
          ['Used Weapons', usedWeaponsStr],
          ['Required Weapons', requiredWeaponsStr],
          ['Used Champion Points', CHAMPION_POINTS.length.toString()],
          ['Required Champion Points', 'N/A'],
        ],
        {
          title: 'Skill Line Configuration',
          columns: [
            { header: 'Setting', width: 25 },
            { header: 'Name', width: classesColumnWidth },
          ],
        },
      ),
    );

    const totalCombinations =
      this.championPointCombinations.length * this.skillCombinationsCount;
    lines.push(
      `Estimated total combinations to evaluate: ${totalCombinations.toLocaleString()}`,
    );

    return lines.join('\n');
  }

  private registerWorkerProgressHandlers(pool: Piscina) {
    pool.on('message', (message: WorkerProgress) => {
      this.workerProgress.set(message.workerId, {
        progressPercent: message.progressPercent,
        bestDamage: message.currentBestDamage,
      });

      // Build table data from all workers
      const tableData: string[][] = [];
      let totalPercent = 0;
      let overallBestDamage: number | null = null;

      for (let i = 1; i <= this.workers; i++) {
        const progress = this.workerProgress.get(i);
        const progressPercent = progress?.progressPercent ?? 0;
        const bestDamage = progress?.bestDamage;

        totalPercent += progressPercent;
        if (bestDamage !== null && bestDamage !== undefined) {
          if (overallBestDamage === null || bestDamage > overallBestDamage) {
            overallBestDamage = bestDamage;
          }
        }

        tableData.push([
          `Worker ${i}`,
          `${progressPercent.toFixed(1)}%`,
          bestDamage !== null && bestDamage !== undefined
            ? bestDamage.toFixed(2)
            : '-',
        ]);
      }

      const averagePercent = totalPercent / this.workers;

      // Calculate ETA
      let etaStr = '';
      if (averagePercent > 1 && this.optimizationStartTime !== null) {
        const elapsedMs = Date.now() - this.optimizationStartTime;
        const etaMs = (elapsedMs * (100 - averagePercent)) / averagePercent;
        etaStr = ` | ETA: ${formatDuration(etaMs)}`;
      }

      const progressTable = table(tableData, {
        title: 'Worker Progress',
        columns: [
          { header: 'Worker', width: 10 },
          { header: 'Progress', width: 15, align: 'right' },
          { header: 'Best Damage', width: 15, align: 'right' },
        ],
        footer: `Overall: ${averagePercent.toFixed(1)}%${overallBestDamage !== null ? ` | Best: ${overallBestDamage.toFixed(2)}` : ''}${etaStr}`,
      });

      logger.progressMultiline(progressTable);
    });
  }
}

export { BuildOptimizer };
