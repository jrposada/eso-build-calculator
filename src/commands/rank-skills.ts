import { Command } from 'commander';

import { ALL_SKILLS } from '../data/skills';
import {
  calculateDamagePerCast,
  getSkillDuration,
  getSkillMechanic,
  getSkillSource,
} from '../services/skill-service';

interface SkillDamage {
  name: string;
  baseSkillName: string;
  source: string;
  skillLine: string;
  damagePerCast: number;
  duration: number;
}

interface RankOptions {
  limit: string;
  format: 'table' | 'json';
  excludeUltimates: boolean;
  source?: string;
  mechanic?: string;
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

function action(options: RankOptions) {
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

  // Filter by source if specified
  if (options.source) {
    const allowedSources = options.source
      .split(',')
      .map((s) => s.trim().toLowerCase());
    skills = skills.filter((skill) =>
      allowedSources.includes(getSkillSource(skill).toLowerCase()),
    );
  }

  // Filter by mechanic if specified
  if (options.mechanic) {
    const allowedMechanics = options.mechanic
      .split(',')
      .map((s) => s.trim().toLocaleLowerCase());
    skills = skills.filter((skill) =>
      allowedMechanics.includes(getSkillMechanic(skill)),
    );
  }

  // Calculate damage and create ranking data
  const allSkillDamages: SkillDamage[] = skills
    .map((skill) => ({
      name: skill.name,
      baseSkillName: skill.baseSkillName,
      source: getSkillSource(skill),
      skillLine: skill.skillLine,
      damagePerCast: calculateDamagePerCast(skill),
      duration: getSkillDuration(skill),
    }))
    .filter((s) => s.damagePerCast > 0); // Only show skills that deal damage

  // Group by baseSkillName and pick the highest damage version from each group
  const skillsByBase = new Map<string, SkillDamage>();
  for (const skill of allSkillDamages) {
    const key = `${skill.source}-${skill.baseSkillName}`;
    const existing = skillsByBase.get(key);
    if (!existing || skill.damagePerCast > existing.damagePerCast) {
      skillsByBase.set(key, skill);
    }
  }

  const skillDamages = Array.from(skillsByBase.values()).sort(
    (a, b) => b.damagePerCast - a.damagePerCast,
  );

  if (skillDamages.length === 0) {
    console.log('No damaging skills found.');
    return;
  }

  if (options.format === 'json') {
    console.log(formatJson(skillDamages, limit));
  } else {
    console.log(formatTable(skillDamages, limit));
  }
}

export const rankCommand = new Command('rank')
  .description('Rank skills by damage per cast')
  .option('-l, --limit <number>', 'Number of skills to show', '20')
  .option('-f, --format <format>', 'Output format (table/json)', 'table')
  .option('--exclude-ultimates', 'Exclude ultimate abilities', false)
  .option(
    '-s, --source <sources>',
    'Only include skills from specified sources (comma-separated)',
  )
  .option(
    '-m, --mechanic <mechanics>',
    'Only include skills of specified mechanic (comma-separated)',
  )
  .action(action);
