import { ALL_MODIFIERS } from '../data/modifiers';
import {
  getClassPassivesBySkillLine,
  getWeaponPassivesBySkillLine,
} from '../data/passives';
import { ALL_SKILLS } from '../data/skills';
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

interface SkillLineUsage {
  classSkillLines: Set<string>;
  weaponSkillLines: Set<string>;
  hasRequiredClassSkillLine: boolean;
}

/**
 * Check if a skill can be added given current constraints
 */
function canAddSkill(
  skill: AnySkill,
  usage: SkillLineUsage,
  constraints: BuildConstraints,
  requiredClass?: string,
): {
  canAdd: boolean;
  newUsage: SkillLineUsage;
  satisfiesRequiredClass: boolean;
} {
  const skillLine = skill.skillLine;
  const isClassSkill = 'esoClass' in skill;
  const isWeaponSkill = !isClassSkill;

  // Check if this skill line would satisfy the required class constraint
  let satisfiesRequiredClass = usage.hasRequiredClassSkillLine;
  if (requiredClass && isClassSkill) {
    const skillClass = CLASS_SKILL_LINE_TO_CLASS[skillLine as ClassSkillLine];
    if (skillClass === requiredClass) {
      satisfiesRequiredClass = true;
    }
  }

  // If skill line already used, we can add it without using more slots
  if (isClassSkill && usage.classSkillLines.has(skillLine)) {
    return {
      canAdd: true,
      newUsage: { ...usage, hasRequiredClassSkillLine: satisfiesRequiredClass },
      satisfiesRequiredClass,
    };
  }
  if (isWeaponSkill && usage.weaponSkillLines.has(skillLine)) {
    return {
      canAdd: true,
      newUsage: usage,
      satisfiesRequiredClass,
    };
  }

  // Check if we have room for a new skill line
  if (isClassSkill) {
    if (usage.classSkillLines.size >= constraints.maxClassSkillLines) {
      return { canAdd: false, newUsage: usage, satisfiesRequiredClass: false };
    }

    // Reserve a class skill line slot for required class if we haven't satisfied it yet
    const needsRequiredClassSlot =
      requiredClass &&
      !usage.hasRequiredClassSkillLine &&
      !satisfiesRequiredClass;
    const wouldFillLastSlot =
      usage.classSkillLines.size === constraints.maxClassSkillLines - 1;

    if (needsRequiredClassSlot && wouldFillLastSlot) {
      // Don't fill the last class skill line slot with a non-required class skill
      return { canAdd: false, newUsage: usage, satisfiesRequiredClass: false };
    }
    const newClassSkillLines = new Set(usage.classSkillLines);
    newClassSkillLines.add(skillLine);
    return {
      canAdd: true,
      newUsage: {
        classSkillLines: newClassSkillLines,
        weaponSkillLines: usage.weaponSkillLines,
        hasRequiredClassSkillLine: satisfiesRequiredClass,
      },
      satisfiesRequiredClass,
    };
  } else {
    if (usage.weaponSkillLines.size >= constraints.maxWeaponSkillLines) {
      return { canAdd: false, newUsage: usage, satisfiesRequiredClass: false };
    }
    const newWeaponSkillLines = new Set(usage.weaponSkillLines);
    newWeaponSkillLines.add(skillLine);
    return {
      canAdd: true,
      newUsage: {
        classSkillLines: usage.classSkillLines,
        weaponSkillLines: newWeaponSkillLines,
        hasRequiredClassSkillLine: usage.hasRequiredClassSkillLine,
      },
      satisfiesRequiredClass,
    };
  }
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
  classSkillLines: Set<string>,
  weaponSkillLines: Set<string>,
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
function calculateDamageWithPassives(
  skill: AnySkill,
  modifiers: DamageModifier[],
  passives: AnyPassiveSkill[],
): number {
  const baseDamage = calculateDamagePerCast(skill, modifiers);
  const passiveBonus = calculatePassiveBonus(skill, passives);
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
): number {
  return skills.reduce((total, skill) => {
    return total + calculateDamageWithPassives(skill, modifiers, passives);
  }, 0);
}

/**
 * Find the optimal build that maximizes total damage per cast
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

  if (verbose) {
    console.log(
      `Testing ${modifierCombinations.length} modifier combinations...`,
    );
  }

  let bestBuild: Build | null = null;
  let combinationsTested = 0;

  for (const modifiers of modifierCombinations) {
    combinationsTested++;
    if (verbose && combinationsTested % 10 === 0) {
      console.log(
        `Progress: ${combinationsTested}/${modifierCombinations.length}`,
      );
    }

    // Preprocess skills with current modifiers (no passives - they depend on selection)
    const processedSkills = preprocessSkills(ALL_SKILLS, modifiers);

    // Greedy selection: at each step, pick the skill that maximizes total build damage
    // considering which passives would be active with that skill added
    const selectedSkills: Array<(typeof processedSkills)[0]> = [];
    let usage: SkillLineUsage = {
      classSkillLines: new Set(),
      weaponSkillLines: new Set(),
      hasRequiredClassSkillLine: !requiredClass,
    };

    // Track which skills have been selected (by baseSkillName key)
    const selectedKeys = new Set<string>();

    while (selectedSkills.length < constraints.maxSkills) {
      let bestCandidate: {
        skill: (typeof processedSkills)[0];
        newUsage: SkillLineUsage;
        totalDamageIfAdded: number;
        satisfiesRequiredClass: boolean;
      } | null = null;

      // Evaluate each candidate skill
      for (const candidate of processedSkills) {
        const key = `${candidate.source}-${candidate.baseSkillName}`;

        // Skip already selected skills
        if (selectedKeys.has(key)) {
          continue;
        }

        const { canAdd, newUsage, satisfiesRequiredClass } = canAddSkill(
          candidate.skill,
          usage,
          constraints,
          requiredClass,
        );

        if (!canAdd) {
          continue;
        }

        // Check if we need to reserve slots for required class
        const remainingSlots = constraints.maxSkills - selectedSkills.length;
        const needsRequiredClass =
          requiredClass && !usage.hasRequiredClassSkillLine;

        if (needsRequiredClass && !satisfiesRequiredClass) {
          if (remainingSlots <= 1) {
            continue; // Skip - need to reserve slot for required class
          }
        }

        // Calculate total damage if we add this skill
        const potentialPassives = getPassivesForSkillLines(
          newUsage.classSkillLines,
          newUsage.weaponSkillLines,
        );
        const potentialSkills = [
          ...selectedSkills.map((s) => s.skill),
          candidate.skill,
        ];
        const totalDamageIfAdded = calculateTotalDamage(
          potentialSkills,
          modifiers,
          potentialPassives,
        );

        // Keep track of best candidate
        if (
          !bestCandidate ||
          totalDamageIfAdded > bestCandidate.totalDamageIfAdded
        ) {
          bestCandidate = {
            skill: candidate,
            newUsage,
            totalDamageIfAdded,
            satisfiesRequiredClass,
          };
        }
      }

      // If no valid candidate found, stop
      if (!bestCandidate) {
        break;
      }

      // Add the best candidate
      selectedSkills.push(bestCandidate.skill);
      selectedKeys.add(
        `${bestCandidate.skill.source}-${bestCandidate.skill.baseSkillName}`,
      );
      usage = bestCandidate.newUsage;
      if (bestCandidate.satisfiesRequiredClass) {
        usage.hasRequiredClassSkillLine = true;
      }
    }

    // Skip builds that don't satisfy the required class constraint
    if (requiredClass && !usage.hasRequiredClassSkillLine) {
      continue;
    }

    // Get final passives for the selected skill lines
    const finalPassives = getPassivesForSkillLines(
      usage.classSkillLines,
      usage.weaponSkillLines,
    );

    // Calculate final damages with the actual passive set
    const buildSkills: BuildSkill[] = selectedSkills.map((processedSkill) => {
      const finalDamage = calculateDamageWithPassives(
        processedSkill.skill,
        modifiers,
        finalPassives,
      );
      return {
        name: processedSkill.name,
        skillLine: processedSkill.skillLine,
        source: processedSkill.source,
        damagePerCast: finalDamage,
      };
    });

    // Calculate total damage
    const totalDamage = buildSkills.reduce(
      (sum, s) => sum + s.damagePerCast,
      0,
    );

    // Check if this is the best build
    if (!bestBuild || totalDamage > bestBuild.totalDamagePerCast) {
      bestBuild = {
        skills: buildSkills,
        passives: convertToBuildPassives(finalPassives),
        modifiers: modifiers.map((m) => m.name),
        totalDamagePerCast: totalDamage,
        usedClassSkillLines: Array.from(usage.classSkillLines),
        usedWeaponSkillLines: Array.from(usage.weaponSkillLines),
        requiredClass,
      };
    }
  }

  return bestBuild;
}

export { CLASS_SKILL_LINE_TO_CLASS, findOptimalBuild, WEAPON_SKILL_LINES };
