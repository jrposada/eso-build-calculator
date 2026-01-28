import { CHAMPION_POINTS } from '../data/bonuses/champion-points/champion-points';
import { BonusData } from '../data/bonuses/types';
import { ClassSkillLineName, WeaponSkillLineName } from '../data/skills';
import { SkillData } from '../data/skills/types';
import { ClassName } from '../data/types';
import { logger } from '../infrastructure';
import { generateCombinations } from '../infrastructure/combinatorics';
import { Build, BUILD_CONSTRAINTS } from '../models/build';
import {
  BuildService,
  ProcessedSkill,
  SkillLineCombination,
} from './build-service';
import { SkillLineCounts } from './skill-service';

interface BuildOptimizerOptions {
  verbose?: boolean;
  className?: ClassName;
  skills?: SkillData[];
}

// Mapping from class skill lines to their ESO class
const CLASS_SKILL_LINE_TO_CLASS: Record<ClassSkillLineName, ClassName> = {
  // Dragonknight
  ArdentFlame: 'Dragonknight',
  DraconicPower: 'Dragonknight',
  EarthenHeart: 'Dragonknight',
  // Sorcerer
  DarkMagic: 'Sorcerer',
  DaedricSummoning: 'Sorcerer',
  StormCalling: 'Sorcerer',
  // Nightblade
  Assassination: 'Nightblade',
  Shadow: 'Nightblade',
  Siphoning: 'Nightblade',
  // Warden
  AnimalCompanions: 'Warden',
  GreenBalance: 'Warden',
  WintersEmbrace: 'Warden',
  // Necromancer
  // GraveLord: 'Necromancer',
  // BoneTyrant: 'Necromancer',
  // LivingDeath: 'Necromancer',
  // Templar
  AedricSpear: 'Templar',
  DawnsWrath: 'Templar',
  RestoringLight: 'Templar',
  // Arcanist
  CurativeRuneforms: 'Arcanist',
  SoldierOfApocrypha: 'Arcanist',
  HeraldOfTheTome: 'Arcanist',
};

const ALL_CLASS_SKILL_LINES = Object.keys(
  CLASS_SKILL_LINE_TO_CLASS,
) as ClassSkillLineName[];

const WEAPON_SKILL_LINES: WeaponSkillLineName[] = [
  'Bow',
  'TwoHanded',
  'DestructionStaff',
  'DualWield',
];

interface OptimizationResult {
  build: Build | null;
  combinationsTested: number;
  totalCombinations: number;
}

class BuildOptimizer {
  private readonly buildService: BuildService;
  private readonly className?: ClassName;
  private readonly verbose: boolean;

  constructor(options?: BuildOptimizerOptions) {
    this.buildService = new BuildService(options?.skills);
    this.className = options?.className;
    this.verbose = options?.verbose ?? false;
  }

  /**
   * Generate all possible builds for a given set of modifiers
   * - Generates skill line combinations (0-3 class lines, 0-2 weapon lines)
   * - Filters by requiredClass if specified
   * - Creates Build objects with optimal skill selection for each combination
   */
  generatePossibleBuilds(
    modifiers: BonusData[],
    processedSkills: ProcessedSkill[],
    skillCountByLine: Map<string, number>,
  ): Build[] {
    const builds: Build[] = [];

    // Generate all class skill line combinations (0 to maxClassSkillLines)
    const classLineCombos: ClassSkillLineName[][] = [[]];
    for (let k = 1; k <= BUILD_CONSTRAINTS.maxClassSkillLines; k++) {
      classLineCombos.push(...generateCombinations(ALL_CLASS_SKILL_LINES, k));
    }

    // Generate all weapon skill line combinations (0 to maxWeaponSkillLines)
    const weaponLineCombos: WeaponSkillLineName[][] = [[]];
    for (let k = 1; k <= BUILD_CONSTRAINTS.maxWeaponSkillLines; k++) {
      weaponLineCombos.push(...generateCombinations(WEAPON_SKILL_LINES, k));
    }

    // Calculate skill damages with modifiers
    const skillsWithDamage = processedSkills.map((ps) => ({
      ...ps,
      baseDamage: this.buildService.calculateSkillDamage(
        ps.skill,
        modifiers,
        [],
        {},
      ),
    }));

    // Cross-product and filter
    for (const classLines of classLineCombos) {
      // If className is set, at least one class line must be from that class
      if (this.className) {
        const hasRequiredClass = classLines.some(
          (line) => CLASS_SKILL_LINE_TO_CLASS[line] === this.className,
        );
        if (!hasRequiredClass) continue;
      }

      for (const weaponLines of weaponLineCombos) {
        // Count total available skills from these skill lines
        const totalAvailableSkills =
          classLines.reduce(
            (sum, line) => sum + (skillCountByLine.get(line) ?? 0),
            0,
          ) +
          weaponLines.reduce(
            (sum, line) => sum + (skillCountByLine.get(line) ?? 0),
            0,
          );

        // Filter: combination must have enough skills to fill maxSkills slots
        if (totalAvailableSkills < BUILD_CONSTRAINTS.maxSkills) {
          continue;
        }

        const skillLineCombo: SkillLineCombination = { classLines, weaponLines };

        // Get all passives for these skill lines
        const classLineSet = new Set(classLines);
        const weaponLineSet = new Set(weaponLines);

        const passives = this.buildService.getPassivesForSkillLines(
          classLines,
          weaponLines,
        );

        // Filter skills to only those from the selected skill lines
        const availableSkills = skillsWithDamage.filter((s) => {
          const isClassSkill = s.skill.className !== 'Weapon';
          if (isClassSkill) {
            return classLineSet.has(s.skillLine as ClassSkillLineName);
          } else {
            return weaponLineSet.has(s.skillLine as WeaponSkillLineName);
          }
        });

        // Calculate damage for each skill with passives applied
        const preliminarySkillLineCounts: SkillLineCounts = {};
        for (const line of classLines) preliminarySkillLineCounts[line] = 1;
        for (const line of weaponLines) preliminarySkillLineCounts[line] = 1;

        const skillsWithPassiveDamage = availableSkills.map((s) => ({
          ...s,
          damageWithPassives: this.buildService.calculateSkillDamage(
            s.skill,
            modifiers,
            passives,
            preliminarySkillLineCounts,
          ),
        }));

        // Sort by damage and take top maxSkills
        skillsWithPassiveDamage.sort(
          (a, b) => b.damageWithPassives - a.damageWithPassives,
        );
        const selectedSkills = skillsWithPassiveDamage.slice(
          0,
          BUILD_CONSTRAINTS.maxSkills,
        );

        // Skip if we couldn't fill all skill slots
        if (selectedSkills.length < BUILD_CONSTRAINTS.maxSkills) {
          continue;
        }

        // Create build
        const build = this.buildService.createBuild(
          selectedSkills.map((s) => s.skill),
          modifiers,
          passives,
          skillLineCombo,
          this.className,
        );

        builds.push(build);
      }
    }

    return builds;
  }

  /**
   * Find the optimal build that maximizes total damage per cast
   * Uses exhaustive enumeration of skill line combinations instead of greedy selection
   */
  findOptimalBuild(): OptimizationResult {
    const modifierCombinations = generateCombinations(
      CHAMPION_POINTS,
      BUILD_CONSTRAINTS.maxModifiers,
    );

    // Preprocess skills once (without passives - they depend on skill line selection)
    // Use a dummy modifier set for deduplication (base damage comparison)
    const processedSkills = this.buildService.preprocessSkills([]);

    // Count available skills per skill line (for filtering valid combinations)
    const skillCountByLine =
      this.buildService.getSkillCountByLine(processedSkills);

    // Count builds per modifier combination for progress tracking
    const sampleBuilds = this.generatePossibleBuilds([], processedSkills, skillCountByLine);
    const buildsPerModifierCombo = sampleBuilds.length;

    if (this.verbose) {
      logger.dim(
        `Testing ${buildsPerModifierCombo} skill line combinations × ${modifierCombinations.length} modifier combinations...`,
      );
    }

    let bestBuild: Build | null = null;
    let combinationsTested = 0;
    const totalCombinations = buildsPerModifierCombo * modifierCombinations.length;

    for (const modifiers of modifierCombinations) {
      const builds = this.generatePossibleBuilds(modifiers, processedSkills, skillCountByLine);

      for (const build of builds) {
        combinationsTested++;

        if (this.verbose && combinationsTested % 1000 === 0) {
          logger.dim(
            `Progress: ${combinationsTested}/${totalCombinations} combinations tested. Best damage so far: ${bestBuild?.totalDamagePerCast.toFixed(0) ?? 'N/A'}`,
          );
        }

        if (!bestBuild || build.totalDamagePerCast > bestBuild.totalDamagePerCast) {
          bestBuild = build;

          if (this.verbose) {
            logger.dim(
              `New best build found: ${build.totalDamagePerCast.toFixed(0)} damage`,
            );
          }
        }
      }
    }

    return {
      build: bestBuild,
      combinationsTested,
      totalCombinations,
    };
  }
}

export { BuildOptimizer };
