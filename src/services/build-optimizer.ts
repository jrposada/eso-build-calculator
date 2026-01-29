import os from 'os';
import path from 'path';
import Piscina from 'piscina';

import { CHAMPION_POINTS } from '../data/bonuses/champion-points/champion-points';
import { ChampionPointBonus } from '../data/bonuses/champion-points/types';
import { CLASS_CLASS_NAMES, ClassClassName } from '../data/skills';
import { SkillData } from '../data/skills/types';
import { ClassName } from '../data/types';
import { batch, logger, table } from '../infrastructure';
import {
  countCombinations,
  generateCombinations,
} from '../infrastructure/combinatorics';
import { MorphSelector } from './morph-selector';
import { Build, BUILD_CONSTRAINTS } from '../models/build';
import { Skill } from '../models/skill';
import type {
  WorkerPayload,
  WorkerProgress,
  WorkerResult,
} from './build-optimizer-worker';
import { SkillsService } from './skills-service';

function getDefaultParallelism(): number {
  return Math.max(1, Math.floor(os.cpus().length / 2));
}

interface BuildOptimizerOptions {
  verbose?: boolean;
  classNames?: ClassClassName[];
  forcedMorphs?: string[];
  skills?: SkillData[];
  workers?: number;
  threads?: number;
}

class BuildOptimizer {
  private readonly skillsService: SkillsService;
  private readonly verbose: boolean;
  private readonly workers: number;
  private readonly threads: number;

  private readonly requiredClassNames: ClassClassName[];
  private readonly classNames: Set<ClassName>;

  private readonly championPointCombinations: ChampionPointBonus[][];
  private readonly classClassNameCombinations: ClassClassName[][];
  private readonly allowedSkills: SkillData[] = [];

  private readonly allowedSkillsCombinationsCount: number;

  // Track progress for each worker
  private readonly workerProgress = new Map<
    number,
    { evaluated: number; bestDamage: number | null }
  >();

  constructor(options?: BuildOptimizerOptions) {
    this.skillsService = new SkillsService(options?.skills);
    this.requiredClassNames = options?.classNames ?? [];
    this.verbose = options?.verbose ?? false;
    this.workers = options?.workers ?? getDefaultParallelism();
    this.threads = options?.threads ?? getDefaultParallelism();

    this.championPointCombinations = generateCombinations(
      CHAMPION_POINTS,
      BUILD_CONSTRAINTS.maxModifiers,
    );

    if (this.verbose) {
      logger.dim(
        `Generated ${this.championPointCombinations.length.toLocaleString()} champion point combinations`,
      );
    }

    this.classNames = new Set<ClassName>(); // TODO: Add Weapons in the future. They work slightly different than classes.
    this.classClassNameCombinations = generateCombinations(
      CLASS_CLASS_NAMES,
      BUILD_CONSTRAINTS.maxClassSkillLines,
    ).filter((combination) => {
      if (this.requiredClassNames.length > 0) {
        const hasRequiredClass = this.requiredClassNames.every((className) =>
          combination.includes(className),
        );

        if (hasRequiredClass) {
          combination.forEach((className) => this.classNames.add(className));
        }

        return hasRequiredClass;
      }

      combination.forEach((className) => this.classNames.add(className));

      return true;
    });

    if (this.verbose) {
      logger.dim(
        `Generated ${this.classClassNameCombinations.length.toLocaleString()} class skill line combinations using ${this.classNames.size} classes`,
      );
    }

    // Gather all skills from allowed classes
    const allSkills = [...this.classNames].flatMap((className) => {
      const skills = this.skillsService.getSkillsByClassName(className, {
        excludeBaseSkills: true,
        excludeUltimates: true,
        excludeNonDamaging: true,
      });

      if (this.verbose) {
        logger.dim(`Class: ${className}, Skills found: ${skills.length}`);
      }

      return skills;
    });

    if (this.verbose) {
      logger.dim(
        `Total skills before morph selection: ${allSkills.length.toLocaleString()}`,
      );
    }

    // Use MorphSelector to pre-select one morph per base skill
    const morphSelector = new MorphSelector({
      forcedMorphs: options?.forcedMorphs,
    });

    // Validate forced morphs and warn about invalid names
    const invalidMorphs = morphSelector.validateForcedMorphs(allSkills);
    if (invalidMorphs.length > 0) {
      logger.warn(
        `Warning: The following morph names are invalid and will be ignored: ${invalidMorphs.join(', ')}`,
      );
    }

    // Select one morph per base skill (greedy or forced)
    this.allowedSkills = morphSelector.selectMorphs(allSkills);

    if (this.verbose) {
      logger.dim(
        `After morph selection: ${this.allowedSkills.length.toLocaleString()} skills (one per base skill)`,
      );
    }

    // Simple combination count since morphs are already pre-selected
    this.allowedSkillsCombinationsCount = countCombinations(
      this.allowedSkills,
      BUILD_CONSTRAINTS.maxSkills,
    );

    if (this.verbose) {
      logger.dim(
        `Total skill combinations: ${this.allowedSkillsCombinationsCount.toLocaleString()}`,
      );
    }

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

    // Determine worker script path based on whether we're in dist or src
    const workerPath = __filename.endsWith('.ts')
      ? path.resolve(__dirname, 'build-optimizer-worker.ts')
      : path.resolve(__dirname, 'build-optimizer-worker.js');

    // Create worker pool
    const pool = new Piscina({
      filename: workerPath,
      maxThreads: this.threads,
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
      allowedSkills: this.allowedSkills,
    }));

    if (this.verbose) {
      const tableData: string[][] = batches.map((batch, i) => [
        `Worker ${i + 1}`,
        (
          batch.championPointBatches.length *
          this.allowedSkillsCombinationsCount
        ).toLocaleString(),
      ]);

      logger.info(
        table(tableData, {
          title: 'Worker Distribution',
          columns: [
            { header: 'Worker', width: 10 },
            { header: 'Iterations', width: 25, align: 'right' },
          ],
        }),
      );
    }

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

    // Configuration table
    const configData: string[][] = [
      ['Max Skills', BUILD_CONSTRAINTS.maxSkills.toString()],
      ['Max Modifiers', BUILD_CONSTRAINTS.maxModifiers.toString()],
      [
        'Max Class Skill Lines',
        BUILD_CONSTRAINTS.maxClassSkillLines.toString(),
      ],
      [
        'Max Weapon Skill Lines',
        BUILD_CONSTRAINTS.maxWeaponSkillLines.toString(),
      ],
    ];

    lines.push(
      table(configData, {
        title: 'Build Constraints',
        columns: [
          { header: 'Constraint', width: 25 },
          { header: 'Value', width: 10, align: 'right' },
        ],
      }),
    );

    // Classes table
    const usedClassesStr = [...this.classNames].join(', ') || 'None';
    const requiredClassesStr = this.requiredClassNames.join(', ') || 'None';
    const classesColumnWidth = Math.max(
      'Classes'.length,
      usedClassesStr.length,
      requiredClassesStr.length,
    );

    const classesData: string[][] = [
      ['Used Classes', usedClassesStr],
      ['Required Classes', requiredClassesStr],
    ];

    lines.push(
      table(classesData, {
        title: 'Class Configuration',
        columns: [
          { header: 'Setting', width: 20 },
          { header: 'Classes', width: classesColumnWidth },
        ],
      }),
    );

    const tableData: string[][] = this.classClassNameCombinations.map(
      (combination, i) => [String(i + 1), combination.join(', ')],
    );

    lines.push(
      table(tableData, {
        title: 'Class Skill Line Combinations',
        columns: [
          { header: '#', width: 5, align: 'right' },
          { header: 'Classes', width: 50 },
        ],
      }),
    );

    // Combinations table
    const combinationsData: string[][] = [
      [
        'Champion Points',
        this.championPointCombinations.length.toLocaleString(),
      ],
      [
        'Class Skill Lines',
        this.classClassNameCombinations.length.toLocaleString(),
      ],
      ['Allowed Skills', this.allowedSkills.length.toLocaleString()],
      [
        'Skill Combinations',
        this.allowedSkillsCombinationsCount.toLocaleString(),
      ],
    ];

    lines.push(
      table(combinationsData, {
        title: 'Combination Counts',
        columns: [
          { header: 'Type', width: 20 },
          { header: 'Count', width: 20, align: 'right' },
        ],
      }),
    );

    // Worker configuration table
    const workerData: string[][] = [
      ['Workers', this.workers.toString()],
      ['Threads per Worker', this.threads.toString()],
    ];

    lines.push(
      table(workerData, {
        title: 'Parallelism',
        columns: [
          { header: 'Setting', width: 20 },
          { header: 'Value', width: 10, align: 'right' },
        ],
      }),
    );

    return lines.join('\n');
  }

  private registerWorkerProgressHandlers(pool: Piscina) {
    pool.on('message', (message: WorkerProgress) => {
      this.workerProgress.set(message.workerId, {
        evaluated: message.evaluated,
        bestDamage: message.currentBestDamage,
      });

      // Build table data from all workers
      const tableData: string[][] = [];
      let totalEvaluated = 0;
      let overallBestDamage: number | null = null;

      for (let i = 1; i <= this.workers; i++) {
        const progress = this.workerProgress.get(i);
        const evaluated = progress?.evaluated ?? 0;
        const bestDamage = progress?.bestDamage;

        totalEvaluated += evaluated;
        if (bestDamage !== null && bestDamage !== undefined) {
          if (overallBestDamage === null || bestDamage > overallBestDamage) {
            overallBestDamage = bestDamage;
          }
        }

        tableData.push([
          `Worker ${i}`,
          evaluated.toLocaleString(),
          bestDamage !== null && bestDamage !== undefined
            ? bestDamage.toFixed(2)
            : '-',
        ]);
      }

      const progressTable = table(tableData, {
        title: 'Worker Progress',
        columns: [
          { header: 'Worker', width: 10 },
          { header: 'Evaluated', width: 25, align: 'right' },
          { header: 'Best Damage', width: 15, align: 'right' },
        ],
        footer: `Total: ${totalEvaluated.toLocaleString()} evaluated${overallBestDamage !== null ? ` | Best: ${overallBestDamage.toFixed(2)}` : ''}`,
      });

      logger.progressMultiline(progressTable);
    });
  }
}

export { BuildOptimizer };
