import { BonusData } from '../data/bonuses/types';
import { PassiveData } from '../data/passives/types';
import {
  ALL_SKILLS,
  ClassSkillLineName,
  SkillLineName,
  WeaponSkillLineName,
} from '../data/skills';
import { SkillData } from '../data/skills/types';
import { ClassName } from '../data/types';

// Base crit stats (assumed from gear/CP) - could be made configurable
const BASE_CRIT_CHANCE = 0.15; // 15% base crit chance
const BASE_CRIT_DAMAGE = 0.5; // 50% base crit damage

/**
 * Check if a bonus applies to a skill and return the applicable bonus value
 */
function getApplicableBonus(bonus: BonusData, skillLineCount: number): number {
  // Get base value and apply multiplier based on className
  const skillCount = skillLineCount ?? 0;
  let multipliedValue = 0;

  switch (bonus.className) {
    case 'skill-line':
    case 'ability-slotted':
      multipliedValue = skillCount > 0 ? bonus.value : 0;
      break;
    case 'ability-slotted-count':
      multipliedValue = bonus.value * skillCount;
      break;
    case 'passive':
      // Always applied bonuses
      multipliedValue = bonus.value;
      break;
    case 'duration':
      // TODO: Duration buffs not yet implemented
      return 0;
    default:
      multipliedValue = skillCount > 0 ? bonus.value : 0;
  }

  if (multipliedValue === 0) return 0;

  // Convert stat types to expected damage bonus
  switch (bonus.type) {
    case 'critical-chance':
    case 'spell-critical-chance':
    case 'weapon-critical-chance':
      // More crit chance = more expected damage: crit_chance_increase * crit_damage
      return multipliedValue * (1 + BASE_CRIT_DAMAGE);
    case 'critical-damage':
      // More crit damage = more expected damage: crit_chance * crit_damage_increase
      return BASE_CRIT_CHANCE * multipliedValue;
    case 'duration':
    case 'max-stamina':
    case 'max-magicka':
    case 'spell-damage':
      // TODO: These don't directly affect damage yet (could be expanded later)
      return 0;
    default:
      return 0;
  }
}

/**
 * Calculate total passive bonus percentage for a skill
 */
export function calculatePassiveBonus(
  passives: Readonly<PassiveData[]>,
  skillLineCount: number,
): number {
  let totalBonus = 0;

  for (const passive of passives) {
    for (const bonus of passive.bonuses) {
      totalBonus += getApplicableBonus(bonus, skillLineCount);
    }
  }

  return totalBonus;
}

const WEAPON_SKILL_LINE_NAMES: WeaponSkillLineName[] = [
  'Bow',
  'TwoHanded',
  'DestructionStaff',
  'DualWield',
];

const CLASS_SKILL_LINE_NAME_TO_CLASS_NAME: Record<
  ClassSkillLineName,
  ClassName
> = {
  CurativeRuneforms: 'Arcanist',
  SoldierOfApocrypha: 'Arcanist',
  HeraldOfTheTome: 'Arcanist',
  ArdentFlame: 'Dragonknight',
  DraconicPower: 'Dragonknight',
  EarthenHeart: 'Dragonknight',
  Assassination: 'Nightblade',
  Shadow: 'Nightblade',
  Siphoning: 'Nightblade',
  DarkMagic: 'Sorcerer',
  DaedricSummoning: 'Sorcerer',
  StormCalling: 'Sorcerer',
  AedricSpear: 'Templar',
  DawnsWrath: 'Templar',
  RestoringLight: 'Templar',
  AnimalCompanions: 'Warden',
  GreenBalance: 'Warden',
  WintersEmbrace: 'Warden',
};

const CLASS_SKILL_LINES_NAMES = Object.keys(
  CLASS_SKILL_LINE_NAME_TO_CLASS_NAME,
) as ClassSkillLineName[];

interface GetSkillsOptions {
  excludeBaseSkills?: boolean;
  excludeUltimates?: boolean;
  excludeNonDamaging?: boolean;
}

class SkillsService {
  private readonly skills: SkillData[];

  private readonly skillsBySkillLineName: Map<SkillLineName, SkillData[]> =
    new Map();

  constructor(skills?: SkillData[]) {
    this.skills = skills ?? ALL_SKILLS;

    for (const skill of this.skills) {
      const existingSkills =
        this.skillsBySkillLineName.get(skill.skillLine as SkillLineName) ?? [];

      this.skillsBySkillLineName.set(skill.skillLine as SkillLineName, [
        ...existingSkills,
        skill,
      ]);
    }
  }

  static getClassName(classSkillLine: ClassSkillLineName): ClassName {
    return CLASS_SKILL_LINE_NAME_TO_CLASS_NAME[classSkillLine];
  }

  getSkillsBySkillLineName(
    skillLineName: SkillLineName,
    options?: GetSkillsOptions,
  ): SkillData[] {
    const skills = this.skillsBySkillLineName.get(skillLineName) ?? [];
    return skills.filter((skill) => {
      if (options?.excludeBaseSkills && skill.name === skill.baseSkillName) {
        return false;
      }
      if (options?.excludeUltimates && skill.resource === 'ultimate') {
        return false;
      }
      if (options?.excludeNonDamaging) {
        const hasDamage =
          (skill.damage.hits?.length ?? 0) > 0 ||
          (skill.damage.dots?.length ?? 0) > 0;
        if (!hasDamage) {
          return false;
        }
      }
      return true;
    });
  }
}

export { CLASS_SKILL_LINES_NAMES, SkillsService, WEAPON_SKILL_LINE_NAMES };
export type { GetSkillsOptions };
