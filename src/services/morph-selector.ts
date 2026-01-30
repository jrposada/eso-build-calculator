import { SkillData } from '../data/skills/types';
import { logger } from '../infrastructure/logger';
import { Skill } from '../models/skill';

interface MorphSelectorOptions {
  forcedMorphs?: string[];
}

class MorphSelector {
  private readonly forcedMorphs: Set<string>;

  constructor(options?: MorphSelectorOptions) {
    this.forcedMorphs = new Set(options?.forcedMorphs ?? []);
  }

  /**
   * Select one morph per base skill using greedy strategy (highest damage)
   * or forced selections from CLI options.
   */
  selectMorphs(skills: SkillData[]): SkillData[] {
    const invalidMorphs = this.validateForcedMorphs(skills);
    if (invalidMorphs.length > 0) {
      logger.warn(
        `Warning: The following morph names are invalid and will be ignored: ${invalidMorphs.sort().join(', ')}`,
      );
    }

    // Group skills by base skill name
    const skillsByBase = new Map<string, SkillData[]>();
    for (const skill of skills) {
      const existing = skillsByBase.get(skill.baseSkillName) ?? [];
      existing.push(skill);
      skillsByBase.set(skill.baseSkillName, existing);
    }

    const selectedSkills: SkillData[] = [];

    for (const [, morphs] of skillsByBase) {
      // Check if any morph is forced
      const forcedMorph = morphs.find((m) => this.forcedMorphs.has(m.name));

      if (forcedMorph) {
        selectedSkills.push(forcedMorph);
      } else {
        // Use greedy strategy: select morph with highest base damage
        const bestMorph = this.selectHighestDamageMorph(morphs);
        if (bestMorph) {
          selectedSkills.push(bestMorph);
        }
      }
    }

    return selectedSkills;
  }

  /**
   * Returns list of invalid morph names that don't exist in the skill list
   */
  private validateForcedMorphs(skills: SkillData[]): string[] {
    const validMorphNames = new Set(skills.map((s) => s.name));
    const invalidMorphs: string[] = [];

    for (const morphName of this.forcedMorphs) {
      if (!validMorphNames.has(morphName)) {
        invalidMorphs.push(morphName);
      }
    }

    return invalidMorphs;
  }

  private selectHighestDamageMorph(morphs: SkillData[]): SkillData | undefined {
    if (morphs.length === 0) return undefined;
    if (morphs.length === 1) return morphs[0];

    let bestMorph: SkillData | undefined;
    let bestDamage = -Infinity;

    for (const morphData of morphs) {
      const skill = Skill.fromData(morphData);
      const damage = skill.calculateDamagePerCast();

      if (damage > bestDamage) {
        bestDamage = damage;
        bestMorph = morphData;
      }
    }

    return bestMorph;
  }
}

export { MorphSelector };
export type { MorphSelectorOptions };
