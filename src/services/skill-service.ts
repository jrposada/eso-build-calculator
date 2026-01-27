import { AnyPassiveSkill, PassiveBonus } from '../models/passive';

export interface SkillLineCounts {
  [skillLine: string]: number;
}

// Base crit stats (assumed from gear/CP) - could be made configurable
const BASE_CRIT_CHANCE = 0.15; // 15% base crit chance
const BASE_CRIT_DAMAGE = 0.5; // 50% base crit damage

/**
 * Check if a bonus applies to a skill and return the applicable bonus value
 */
function getApplicableBonus(
  bonus: PassiveBonus,
  passiveSkillLine: string,
  skillLineCounts: SkillLineCounts,
): number {
  // Handle buffId type
  if ('buffId' in bonus) {
    // Minor Savagery = +10% crit damage
    if (bonus.buffId === 'Minor Savagery') {
      const skillCount = skillLineCounts[passiveSkillLine] ?? 0;
      if (skillCount === 0) return 0;
      // Convert crit damage to expected damage: crit_chance * crit_damage_increase
      return BASE_CRIT_CHANCE * 0.1;
    }
    return 0;
  }

  // Get base value and apply multiplier
  const skillCount = skillLineCounts[passiveSkillLine] ?? 0;
  let multipliedValue = 0;

  switch (bonus.multiplier) {
    case 'skillLine':
    case 'abilitySlotted':
      multipliedValue = skillCount > 0 ? bonus.value : 0;
      break;
    case 'abilitySlottedCount':
      multipliedValue = bonus.value * skillCount;
      break;
    default:
      multipliedValue = skillCount > 0 ? bonus.value : 0;
  }

  if (multipliedValue === 0) return 0;

  // Convert stat types to expected damage bonus
  switch (bonus.type) {
    case 'critical-chance':
      // More crit chance = more expected damage: crit_chance_increase * crit_damage
      return multipliedValue * (1 + BASE_CRIT_DAMAGE);
    case 'critical-damage':
      // More crit damage = more expected damage: crit_chance * crit_damage_increase
      return BASE_CRIT_CHANCE * multipliedValue;
    case 'duration':
    case 'max-stamina':
    case 'max-magicka':
      // These don't directly affect damage (could be expanded later)
      return 0;
    default:
      return 0;
  }
}

/**
 * Calculate total passive bonus percentage for a skill
 */
export function calculatePassiveBonus(
  passives: AnyPassiveSkill[],
  skillLineCounts: SkillLineCounts,
): number {
  let totalBonus = 0;

  for (const passive of passives) {
    for (const bonus of passive.bonuses) {
      totalBonus += getApplicableBonus(
        bonus,
        passive.skillLine,
        skillLineCounts,
      );
    }
  }

  return totalBonus;
}
