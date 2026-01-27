import { ALL_MODIFIERS } from '../data/modifiers';
import {
  getClassPassivesBySkillLine,
  getWeaponPassivesBySkillLine,
} from '../data/passives';
import { ALL_SKILLS } from '../data/skills';
import { logger } from '../infrastructure';
import {
  Build,
  BuildConstraints,
  BuildPassive,
  BuildSkill,
} from '../models/build';
import { DamageModifier } from '../models/modifier';
import { AnyPassiveSkill } from '../models/passive';
import { ClassSkillLine, EsoClass, WeaponSkillLineName } from '../models/skill';
import {
  AnySkill,
  calculateDamagePerCast,
  calculatePassiveBonus,
  getSkillSource,
  SkillLineCounts,
} from './skill-service';

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

interface SkillLineCombination {
  classLines: ClassSkillLine[];
  weaponLines: WeaponSkillLineName[];
}

/**
 * Count skills per skill line from a set of skills
 */
function countSkillsPerLine(skills: AnySkill[]): SkillLineCounts {
  const counts: SkillLineCounts = {};
  for (const skill of skills) {
    counts[skill.skillLine] = (counts[skill.skillLine] ?? 0) + 1;
  }
  return counts;
}

/**
 * Generate all valid skill line combinations
 * - 0 to maxClassSkillLines class skill lines from 21 available
 * - 0 to maxWeaponSkillLines weapon skill lines from 4 available
 * - Filter by requiredClass if specified
 */
function generateSkillLineCombinations(
  constraints: BuildConstraints,
  skillCountByLine: Map<string, number>,
  requiredClass?: string,
): SkillLineCombination[] {
  const combinations: SkillLineCombination[] = [];

  // Generate all class skill line combinations (0 to maxClassSkillLines)
  const classLineCombos: ClassSkillLine[][] = [[]];
  for (let k = 1; k <= constraints.maxClassSkillLines; k++) {
    classLineCombos.push(...generateCombinations(ALL_CLASS_SKILL_LINES, k));
  }

  // Generate all weapon skill line combinations (0 to maxWeaponSkillLines)
  const weaponLineCombos: WeaponSkillLineName[][] = [[]];
  for (let k = 1; k <= constraints.maxWeaponSkillLines; k++) {
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
      if (totalAvailableSkills >= constraints.maxSkills) {
        combinations.push({ classLines, weaponLines });
      }
    }
  }

  return combinations;
}

interface ProcessedSkill {
  skill: AnySkill;
  name: string;
  baseSkillName: string;
  skillLine: string;
  source: string;
}

/**
 * Get all passives that apply based on selected skill lines
 */
function getPassivesForSkillLines(
  classSkillLineCounts: Map<string, number>,
  weaponSkillLineCounts: Map<string, number>,
): AnyPassiveSkill[] {
  const passives: AnyPassiveSkill[] = [];

  for (const skillLine of classSkillLineCounts.keys()) {
    passives.push(...getClassPassivesBySkillLine(skillLine));
  }
  for (const skillLine of weaponSkillLineCounts.keys()) {
    passives.push(...getWeaponPassivesBySkillLine(skillLine));
  }

  return passives;
}

/**
 * Calculate damage for a skill with applicable passives
 */
function calculateDamageWithPassives(
  skill: AnySkill,
  modifiers: DamageModifier[],
  passives: AnyPassiveSkill[],
  skillLineCounts: SkillLineCounts,
): number {
  const baseDamage = calculateDamagePerCast(skill, modifiers);
  const passiveBonus = calculatePassiveBonus(skill, passives, skillLineCounts);
  return baseDamage * (1 + passiveBonus);
}

/**
 * Convert passives to BuildPassive format for output
 */
function convertToBuildPassives(passives: AnyPassiveSkill[]): BuildPassive[] {
  return passives.map((p) => ({
    name: p.name,
    skillLine: p.skillLine,
    source: 'esoClass' in p ? p.esoClass : 'Weapon',
  }));
}

/**
 * Preprocess skills: filter ultimates and deduplicate by baseSkillName (keep best morph for given modifiers)
 * Note: We use base damage (without passives) for deduplication since passive availability
 * depends on which skill lines end up being selected
 */
function preprocessSkills(
  skills: AnySkill[],
  modifiers: DamageModifier[],
): Array<ProcessedSkill & { baseDamage: number }> {
  // Filter out ultimates
  const nonUltimates = skills.filter((skill) => skill.resource !== 'ultimate');

  // Calculate base damage (without passives) and create processed skill data
  const processed = nonUltimates.map((skill) => {
    const baseDamage = calculateDamagePerCast(skill, modifiers);
    return {
      skill,
      name: skill.name,
      baseSkillName: skill.baseSkillName,
      skillLine: skill.skillLine,
      source: getSkillSource(skill),
      baseDamage,
    };
  });

  // Filter to only damaging skills
  const damagingSkills = processed.filter((s) => s.baseDamage > 0);

  // Group by baseSkillName and pick highest base damage morph
  const skillsByBase = new Map<string, (typeof damagingSkills)[0]>();
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
 * Calculate total damage for a set of skills with given passives
 */
function calculateTotalDamage(
  skills: AnySkill[],
  modifiers: DamageModifier[],
  passives: AnyPassiveSkill[],
  skillLineCounts: SkillLineCounts,
): number {
  return skills.reduce((total, skill) => {
    return (
      total +
      calculateDamageWithPassives(skill, modifiers, passives, skillLineCounts)
    );
  }, 0);
}

/**
 * Find the optimal build that maximizes total damage per cast
 * Uses exhaustive enumeration of skill line combinations instead of greedy selection
 */
function findOptimalBuild(
  constraints: BuildConstraints,
  requiredClass?: string,
  verbose = false,
): Build | null {
  const modifierCombinations = generateCombinations(
    ALL_MODIFIERS,
    constraints.maxModifiers,
  );

  // Preprocess skills once (without passives - they depend on skill line selection)
  // Use a dummy modifier set for deduplication (base damage comparison)
  const processedSkills = preprocessSkills(ALL_SKILLS, []);

  // Count available skills per skill line (for filtering valid combinations)
  const skillCountByLine = new Map<string, number>();
  for (const skill of processedSkills) {
    skillCountByLine.set(
      skill.skillLine,
      (skillCountByLine.get(skill.skillLine) ?? 0) + 1,
    );
  }

  // Generate all valid skill line combinations
  const skillLineCombinations = generateSkillLineCombinations(
    constraints,
    skillCountByLine,
    requiredClass,
  );

  if (verbose) {
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
      baseDamage: calculateDamagePerCast(ps.skill, modifiers),
    }));

    for (const skillLineCombo of skillLineCombinations) {
      combinationsTested++;

      if (verbose && combinationsTested % 1000 === 0) {
        logger.dim(
          `Progress: ${combinationsTested}/${totalCombinations} combinations tested. Best damage so far: ${bestBuild?.totalDamagePerCast.toFixed(0) ?? 'N/A'}`,
        );
      }

      // Get all passives for these skill lines
      const classLineSet = new Set(skillLineCombo.classLines);
      const weaponLineSet = new Set(skillLineCombo.weaponLines);

      const classLineMap = new Map<string, number>();
      const weaponLineMap = new Map<string, number>();
      for (const line of skillLineCombo.classLines) classLineMap.set(line, 0);
      for (const line of skillLineCombo.weaponLines) weaponLineMap.set(line, 0);

      const passives = getPassivesForSkillLines(classLineMap, weaponLineMap);

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
        damageWithPassives: calculateDamageWithPassives(
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
        constraints.maxSkills,
      );

      // Skip if we couldn't fill all skill slots
      if (selectedSkills.length < constraints.maxSkills) {
        continue;
      }

      // Calculate actual skill line counts from selected skills
      const actualSkillLineCounts = countSkillsPerLine(
        selectedSkills.map((s) => s.skill),
      );

      // Recalculate total damage with actual skill line counts
      const totalDamage = calculateTotalDamage(
        selectedSkills.map((s) => s.skill),
        modifiers,
        passives,
        actualSkillLineCounts,
      );

      // Check if this is the best build
      if (!bestBuild || totalDamage > bestBuild.totalDamagePerCast) {
        const buildSkills: BuildSkill[] = selectedSkills.map((s) => ({
          name: s.name,
          skillLine: s.skillLine,
          source: s.source,
          damagePerCast: calculateDamageWithPassives(
            s.skill,
            modifiers,
            passives,
            actualSkillLineCounts,
          ),
        }));

        bestBuild = {
          skills: buildSkills,
          passives: convertToBuildPassives(passives),
          modifiers: modifiers.map((m) => m.name),
          totalDamagePerCast: totalDamage,
          usedClassSkillLines: skillLineCombo.classLines,
          usedWeaponSkillLines: skillLineCombo.weaponLines,
          requiredClass,
        };

        if (verbose) {
          logger.dim(`New best build found: ${totalDamage.toFixed(0)} damage`);
        }
      }
    }
  }

  return bestBuild;
}

export { CLASS_SKILL_LINE_TO_CLASS, findOptimalBuild, WEAPON_SKILL_LINES };
