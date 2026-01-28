import { CHAMPION_POINTS } from '../data/bonuses/champion-points/champion-points';
import { ClassSkillLineName, WeaponSkillLineName } from '../data/skills';
import { SkillData } from '../data/skills/types';
import { ClassName } from '../data/types';
import { logger, table } from '../infrastructure';
import {
  countGroupedCombinations,
  generateCombinations,
  generateGroupedCombinationsIterator,
} from '../infrastructure/combinatorics';
import { Build, BUILD_CONSTRAINTS } from '../models/build';
import { Skill } from '../models/skill';
import {
  CLASS_SKILL_LINES_NAMES,
  GetSkillsOptions,
  SkillsService,
  WEAPON_SKILL_LINE_NAMES,
} from './skills-service';

interface BuildOptimizerOptions {
  verbose?: boolean;
  className?: ClassName;
  skills?: SkillData[];
}

class BuildOptimizer {
  private readonly skillsService: SkillsService;
  private readonly className?: ClassName;
  private readonly verbose: boolean;

  constructor(options?: BuildOptimizerOptions) {
    this.skillsService = new SkillsService(options?.skills);
    this.className = options?.className;
    this.verbose = options?.verbose ?? false;
  }

  /**
   * Find the optimal build that maximizes total damage per cast
   * Uses exhaustive enumeration of skill line combinations
   */
  findOptimalBuild(): Build | null {
    const championPointCombinations = generateCombinations(
      CHAMPION_POINTS,
      BUILD_CONSTRAINTS.maxModifiers,
    );

    // Cross-product of skill combinations and champion point combinations and
    // keep track of the best build found
    let bestBuild: Build | null = null;
    let evaluatedCount = 0;
    for (const championPointCombination of championPointCombinations) {
      // Use generator to iterate lazily - only one skill combination in memory at a time
      for (const skillCombination of this.generateSkillCombinations()) {
        const build = new Build(skillCombination, championPointCombination);
        if (build.isBetterThan(bestBuild)) {
          bestBuild = build;
        }
        evaluatedCount++;
        if (this.verbose && evaluatedCount % 200000 === 0) {
          logger.progress(`Evaluated ${evaluatedCount.toLocaleString()} builds...`);
        }
      }
    }

    if (this.verbose) {
      logger.info(`Evaluated ${evaluatedCount} total build combinations.`);
    }

    return bestBuild;
  }

  private *generateSkillCombinations(): Generator<Skill[], void, unknown> {
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
        `Generated ${classSkillLineNameCombinations.length * weaponSkillLineNameCombinations.length} skill line combinations.`,
      );
    }

    // Cross-product of class and weapon skill line combinations and filter out invalid ones
    for (const classSkillLineCombination of classSkillLineNameCombinations) {
      if (this.className) {
        const hasRequiredClass = classSkillLineCombination.some(
          (line) => SkillsService.getClassName(line) === this.className,
        );

        if (!hasRequiredClass) continue;
      }

      for (const weaponSkillLineCombination of weaponSkillLineNameCombinations) {
        const skillOptions: GetSkillsOptions = {
          excludeBaseSkills: true,
          excludeUltimates: true,
          excludeNonDamaging: true,
        };
        const allCombinationPossibleSkills = [
          ...classSkillLineCombination.flatMap((line) =>
            this.skillsService
              .getSkillsBySkillLineName(line, skillOptions)
              .map(Skill.fromData),
          ),
          ...weaponSkillLineCombination.flatMap((line) =>
            this.skillsService
              .getSkillsBySkillLineName(line, skillOptions)
              .map(Skill.fromData),
          ),
        ];

        if (this.verbose) {
          logger.info(
            `Evaluating ${allCombinationPossibleSkills.length} skills from class lines [${classSkillLineCombination.join(', ')}] and weapon lines [${weaponSkillLineCombination.join(', ')}]`,
          );
          logger.info(
            table(
              allCombinationPossibleSkills.map((skill) => [
                skill.name,
                skill.skillLine,
              ]),
              {
                columns: [
                  { header: 'Skill', width: 40 },
                  { header: 'Source', width: 25 },
                ],
              },
            ),
          );
          logger.info(
            `Total skills combinations for current lines combination: ${countGroupedCombinations(
              allCombinationPossibleSkills,
              BUILD_CONSTRAINTS.maxSkills,
              (skill) => skill.baseSkillName,
            )}`,
          );
        }

        // Yield skill combinations lazily using iterator
        // Group by baseSkillName to avoid invalid combinations with multiple morphs of the same skill
        yield *
          generateGroupedCombinationsIterator(
            allCombinationPossibleSkills,
            BUILD_CONSTRAINTS.maxSkills,
            (skill) => skill.baseSkillName,
          );
      }
    }
  }
}

export { BuildOptimizer };
