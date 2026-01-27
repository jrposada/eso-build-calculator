import { ALL_MODIFIERS } from '../data/modifiers';
import { logger } from '../infrastructure';
import { Build, BUILD_CONSTRAINTS } from '../models/build';
import { ClassSkillLine, EsoClass, WeaponSkillLineName } from '../models/skill';
import { BuildService, SkillLineCombination } from './build-service';
import { AnySkill, SkillLineCounts } from './skill-service';

// Mapping from class skill lines to their ESO class
const CLASS_SKILL_LINE_TO_CLASS: Record<ClassSkillLine, EsoClass> = {
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
  GraveLord: 'Necromancer',
  BoneTyrant: 'Necromancer',
  LivingDeath: 'Necromancer',
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
) as ClassSkillLine[];

const WEAPON_SKILL_LINES: WeaponSkillLineName[] = [
  'Bow',
  'TwoHanded',
  'DestructionStaff',
  'DualWield',
];

/**
 * Generate all C(n,k) combinations of items
 */
function generateCombinations<T>(items: T[], k: number): T[][] {
  const result: T[][] = [];

  function backtrack(start: number, current: T[]): void {
    if (current.length === k) {
      result.push([...current]);
      return;
    }

    for (let i = start; i < items.length; i++) {
      const item = items[i];
      if (item !== undefined) {
        current.push(item);
        backtrack(i + 1, current);
        current.pop();
      }
    }
  }

  backtrack(0, []);
  return result;
}

export interface OptimizationResult {
  build: Build | null;
  combinationsTested: number;
  totalCombinations: number;
}

export class BuildOptimizer {
  private readonly buildService: BuildService;
  private readonly verbose: boolean;

  constructor(options?: {
    buildService?: BuildService;
    verbose?: boolean;
    skills?: AnySkill[];
  }) {
    this.buildService =
      options?.buildService ?? new BuildService(options?.skills);
    this.verbose = options?.verbose ?? false;
  }

  /**
   * Generate all valid skill line combinations
   * - 0 to maxClassSkillLines class skill lines from 21 available
   * - 0 to maxWeaponSkillLines weapon skill lines from 4 available
   * - Filter by requiredClass if specified
   */
  generateSkillLineCombinations(
    skillCountByLine: Map<string, number>,
    requiredClass?: string,
  ): SkillLineCombination[] {
    const combinations: SkillLineCombination[] = [];

    // Generate all class skill line combinations (0 to maxClassSkillLines)
    const classLineCombos: ClassSkillLine[][] = [[]];
    for (let k = 1; k <= BUILD_CONSTRAINTS.maxClassSkillLines; k++) {
      classLineCombos.push(...generateCombinations(ALL_CLASS_SKILL_LINES, k));
    }

    // Generate all weapon skill line combinations (0 to maxWeaponSkillLines)
    const weaponLineCombos: WeaponSkillLineName[][] = [[]];
    for (let k = 1; k <= BUILD_CONSTRAINTS.maxWeaponSkillLines; k++) {
      weaponLineCombos.push(...generateCombinations(WEAPON_SKILL_LINES, k));
    }

    // Cross-product and filter
    for (const classLines of classLineCombos) {
      // If requiredClass is set, at least one class line must be from that class
      if (requiredClass) {
        const hasRequiredClass = classLines.some(
          (line) => CLASS_SKILL_LINE_TO_CLASS[line] === requiredClass,
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
        if (totalAvailableSkills >= BUILD_CONSTRAINTS.maxSkills) {
          combinations.push({ classLines, weaponLines });
        }
      }
    }

    return combinations;
  }

  /**
   * Find the optimal build that maximizes total damage per cast
   * Uses exhaustive enumeration of skill line combinations instead of greedy selection
   */
  findOptimalBuild(requiredClass?: EsoClass): OptimizationResult {
    const modifierCombinations = generateCombinations(
      ALL_MODIFIERS,
      BUILD_CONSTRAINTS.maxModifiers,
    );

    // Preprocess skills once (without passives - they depend on skill line selection)
    // Use a dummy modifier set for deduplication (base damage comparison)
    const processedSkills = this.buildService.preprocessSkills([]);

    // Count available skills per skill line (for filtering valid combinations)
    const skillCountByLine =
      this.buildService.getSkillCountByLine(processedSkills);

    // Generate all valid skill line combinations
    const skillLineCombinations = this.generateSkillLineCombinations(
      skillCountByLine,
      requiredClass,
    );

    if (this.verbose) {
      logger.dim(
        `Testing ${skillLineCombinations.length} skill line combinations × ${modifierCombinations.length} modifier combinations...`,
      );
    }

    let bestBuild: Build | null = null;
    let combinationsTested = 0;
    const totalCombinations =
      skillLineCombinations.length * modifierCombinations.length;

    for (const modifiers of modifierCombinations) {
      // Recalculate skill damages with these modifiers
      const skillsWithDamage = processedSkills.map((ps) => ({
        ...ps,
        baseDamage: this.buildService.calculateSkillDamage(
          ps.skill,
          modifiers,
          [],
          {},
        ),
      }));

      for (const skillLineCombo of skillLineCombinations) {
        combinationsTested++;

        if (this.verbose && combinationsTested % 1000 === 0) {
          logger.dim(
            `Progress: ${combinationsTested}/${totalCombinations} combinations tested. Best damage so far: ${bestBuild?.totalDamagePerCast.toFixed(0) ?? 'N/A'}`,
          );
        }

        // Get all passives for these skill lines
        const classLineSet = new Set(skillLineCombo.classLines);
        const weaponLineSet = new Set(skillLineCombo.weaponLines);

        const passives = this.buildService.getPassivesForSkillLines(
          skillLineCombo.classLines,
          skillLineCombo.weaponLines,
        );

        // Filter skills to only those from the selected skill lines
        const availableSkills = skillsWithDamage.filter((s) => {
          const isClassSkill = 'esoClass' in s.skill;
          if (isClassSkill) {
            return classLineSet.has(s.skillLine as ClassSkillLine);
          } else {
            return weaponLineSet.has(s.skillLine as WeaponSkillLineName);
          }
        });

        // Calculate damage for each skill with passives applied
        // We need to estimate skill line counts for passive calculations
        // Since we're taking top N skills, we use a preliminary count based on available skills
        const preliminarySkillLineCounts: SkillLineCounts = {};
        for (const line of skillLineCombo.classLines)
          preliminarySkillLineCounts[line] = 1;
        for (const line of skillLineCombo.weaponLines)
          preliminarySkillLineCounts[line] = 1;

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

        // Calculate actual skill line counts from selected skills
        const actualSkillLineCounts = this.buildService.countSkillsPerLine(
          selectedSkills.map((s) => s.skill),
        );

        // Recalculate total damage with actual skill line counts
        const totalDamage = this.buildService.calculateTotalDamage(
          selectedSkills.map((s) => s.skill),
          modifiers,
          passives,
          actualSkillLineCounts,
        );

        // Check if this is the best build
        if (!bestBuild || totalDamage > bestBuild.totalDamagePerCast) {
          bestBuild = this.buildService.createBuild(
            selectedSkills.map((s) => s.skill),
            modifiers,
            passives,
            skillLineCombo,
            requiredClass,
          );

          if (this.verbose) {
            logger.dim(
              `New best build found: ${totalDamage.toFixed(0)} damage`,
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

export { CLASS_SKILL_LINE_TO_CLASS, WEAPON_SKILL_LINES };
