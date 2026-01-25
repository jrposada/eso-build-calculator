import { Command } from 'commander';

import { ALL_SKILLS } from '../data/skills';
import {
  AnySkill,
  calculateDamagePerCast,
  getSkillDuration,
} from '../services/skill-service';

interface SkillDamage {
  name: string;
  source: string;
  skillLine: string;
  damagePerCast: number;
  duration: number;
}

function getSkillSource(skill: AnySkill): string {
  if ('esoClass' in skill) {
    return skill.esoClass;
  }
  return 'Weapon';
}

interface RankOptions {
  limit: string;
  format: 'table' | 'json';
  excludeUltimates: boolean;
}

function formatTable(skills: SkillDamage[], limit: number): string {
  const divider = '─'.repeat(85);
  const lines: string[] = [
    '',
    'Skills Ranked by Damage Per Cast',
    divider,
    formatRow('#', 'Name', 'Source', 'Skill Line', 'Damage', 'Duration'),
    divider,
  ];

  const displaySkills = skills.slice(0, limit);

  displaySkills.forEach((skill, i) => {
    lines.push(
      formatRow(
        (i + 1).toString(),
        skill.name,
        skill.source,
        skill.skillLine,
        skill.damagePerCast.toFixed(0),
        skill.duration > 0 ? `${skill.duration}s` : 'instant',
      ),
    );
  });

  lines.push(divider);
  lines.push(`Showing ${displaySkills.length} of ${skills.length} skills`);
  lines.push('');

  return lines.join('\n');
}

function formatRow(
  rank: string,
  name: string,
  source: string,
  skillLine: string,
  damage: string,
  duration: string,
): string {
  const rankWidth = 4;
  const nameWidth = 25;
  const sourceWidth = 12;
  const skillLineWidth = 18;
  const damageWidth = 10;
  const durationWidth = 10;

  return `${rank.padStart(rankWidth)} ${name.padEnd(nameWidth)} ${source.padEnd(sourceWidth)} ${skillLine.padEnd(skillLineWidth)} ${damage.padStart(damageWidth)} ${duration.padStart(durationWidth)}`;
}

function formatJson(skills: SkillDamage[], limit: number): string {
  return JSON.stringify(skills.slice(0, limit), null, 2);
}

export const rankCommand = new Command('rank')
  .description('Rank skills by damage per cast')
  .option('-l, --limit <number>', 'Number of skills to show', '20')
  .option('-f, --format <format>', 'Output format (table/json)', 'table')
  .option('--exclude-ultimates', 'Exclude ultimate abilities', false)
  .action((options: RankOptions) => {
    const limit = parseInt(options.limit, 10);
    if (isNaN(limit) || limit <= 0) {
      console.error('Error: Limit must be a positive number.');
      process.exit(1);
    }

    let skills = ALL_SKILLS;

    // Exclude ultimates if specified
    if (options.excludeUltimates) {
      skills = skills.filter((skill) => skill.resource !== 'ultimate');
    }

    // Calculate damage and create ranking data
    const skillDamages: SkillDamage[] = skills
      .map((skill) => ({
        name: skill.name,
        source: getSkillSource(skill),
        skillLine: skill.skillLine,
        damagePerCast: calculateDamagePerCast(skill),
        duration: getSkillDuration(skill),
      }))
      .filter((s) => s.damagePerCast > 0) // Only show skills that deal damage
      .sort((a, b) => b.damagePerCast - a.damagePerCast);

    if (skillDamages.length === 0) {
      console.log('No damaging skills found.');
      return;
    }

    if (options.format === 'json') {
      console.log(formatJson(skillDamages, limit));
    } else {
      console.log(formatTable(skillDamages, limit));
    }
  });
