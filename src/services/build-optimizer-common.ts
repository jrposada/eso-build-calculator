import { ClassSkillLineName, WeaponSkillLineName } from '../data/skills';
import { SkillData } from '../data/skills/types';
import { ClassName } from '../data/types';
import { logger } from '../infrastructure';
import { countGroupedCombinations } from '../infrastructure/combinatorics';
import { BUILD_CONSTRAINTS } from '../models/build';
import { GetSkillsOptions, SkillsService } from './skills-service';

const SKILL_OPTIONS: GetSkillsOptions = {
  excludeBaseSkills: true,
  excludeUltimates: true,
  excludeNonDamaging: true,
};

/**
 * Iterate through all valid skill line combinations and yield the skills for each.
 * Applies class filtering when classNames is provided - at least one skill line must
 * belong to one of the specified classes.
 */
function* iterateSkillLineCombinations(
  skillsService: SkillsService,
  classSkillLineNameCombinations: ClassSkillLineName[][],
  weaponSkillLineNameCombinations: WeaponSkillLineName[][],
  classNames?: ClassName[],
): Generator<SkillData[], void, unknown> {
  for (const classSkillLineCombination of classSkillLineNameCombinations) {
    if (classNames && classNames.length > 0) {
      const hasRequiredClass = classNames.every((className) =>
        classSkillLineCombination.some(
          (line) => SkillsService.getClassName(line) === className,
        ),
      );
      if (!hasRequiredClass) continue;
    }

    for (const weaponSkillLineCombination of weaponSkillLineNameCombinations) {
      const allCombinationPossibleSkills = [
        ...classSkillLineCombination.flatMap((line) =>
          skillsService.getSkillsBySkillLineName(line, SKILL_OPTIONS),
        ),
        ...weaponSkillLineCombination.flatMap((line) =>
          skillsService.getSkillsBySkillLineName(line, SKILL_OPTIONS),
        ),
      ];

      yield allCombinationPossibleSkills;
    }
  }
}

/**
 * Count total skill combinations across all valid skill line combinations.
 * This is an instant calculation without iteration.
 */
function countTotalSkillCombinations(
  skillsService: SkillsService,
  classSkillLineNameCombinations: ClassSkillLineName[][],
  weaponSkillLineNameCombinations: WeaponSkillLineName[][],
  classNames?: ClassName[],
): number {
  let total = 0;

  for (const classSkillLineCombination of classSkillLineNameCombinations) {
    logger.progress(`Counting combinations: ${total.toLocaleString()}`);

    if (classNames && classNames.length > 0) {
      const hasRequiredClass = classSkillLineCombination.some((line) =>
        classNames.includes(SkillsService.getClassName(line)),
      );
      if (!hasRequiredClass) continue;
    }

    for (const weaponSkillLineCombination of weaponSkillLineNameCombinations) {
      const allCombinationPossibleSkills = [
        ...classSkillLineCombination.flatMap((line) =>
          skillsService.getSkillsBySkillLineName(line, SKILL_OPTIONS),
        ),
        ...weaponSkillLineCombination.flatMap((line) =>
          skillsService.getSkillsBySkillLineName(line, SKILL_OPTIONS),
        ),
      ];

      total += countGroupedCombinations(
        allCombinationPossibleSkills,
        BUILD_CONSTRAINTS.maxSkills,
        (skill) => skill.baseSkillName,
      );
    }
  }

  return total;
}

export {
  countTotalSkillCombinations,
  iterateSkillLineCombinations,
  SKILL_OPTIONS,
};
