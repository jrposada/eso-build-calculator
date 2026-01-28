import { Command } from 'commander';

import { CHAMPION_POINTS } from '../data/bonuses/champion-points/champion-points';
import { ChampionPointBonus } from '../data/bonuses/champion-points/types';
import { ALL_SKILLS } from '../data/skills';
import { logger, table } from '../infrastructure';
import { Skill } from '../models/skill';

interface RankOptions {
  limit: string;
  excludeUltimates: boolean;
  source?: string;
  mechanic?: string;
  championBonus?: string;
}

function formatTable(
  skills: Skill[],
  damageMap: Map<Skill, number>,
  limit: number,
): string {
  const displaySkills = skills.slice(0, limit);

  const data = displaySkills.map((skill, i) => [
    (i + 1).toString(),
    skill.name,
    skill.className,
    skill.skillLine,
    (damageMap.get(skill) ?? 0).toFixed(0),
    skill.duration > 0 ? `${skill.duration}s` : 'instant',
  ]);

  return table(data, {
    title: 'Skills Ranked by Damage Per Cast',
    columns: [
      { header: '#', width: 4, align: 'right' },
      { header: 'Name', width: 25 },
      { header: 'Source', width: 12 },
      { header: 'Skill Line', width: 18 },
      { header: 'Damage', width: 10, align: 'right' },
      { header: 'Duration', width: 10, align: 'right' },
    ],
    footer: `Showing ${displaySkills.length} of ${skills.length} skills`,
  });
}

function action(options: RankOptions) {
  logger.warn(
    'Rank command is experimental and may produce inaccurate results.',
  );

  const limit = parseInt(options.limit, 10);
  if (isNaN(limit) || limit <= 0) {
    logger.error('Error: Limit must be a positive number.');
    process.exit(1);
  }

  let skills = Skill.fromDataArray(ALL_SKILLS);

  if (options.excludeUltimates) {
    skills = skills.filter((skill) => !skill.isUltimate);
  }

  if (options.source) {
    const allowedSources = options.source
      .split(',')
      .map((s) => s.trim().toLowerCase());
    skills = skills.filter((skill) =>
      allowedSources.includes(skill.className.toLowerCase()),
    );
  }

  if (options.mechanic) {
    const allowedMechanics = options.mechanic
      .split(',')
      .map((s) => s.trim().toLowerCase());
    skills = skills.filter((skill) =>
      allowedMechanics.includes(skill.mechanic),
    );
  }

  let championBonuses: ChampionPointBonus[] = [];
  if (options.championBonus) {
    const allowedChampionPoints = options.championBonus
      .split(',')
      .map((s) => s.trim().toLowerCase());
    championBonuses = CHAMPION_POINTS.filter((bonus) =>
      allowedChampionPoints.includes(bonus.name.toLowerCase()),
    );
  }

  // Calculate damage for each skill and cache in map
  const damageMap = new Map<Skill, number>();
  for (const skill of skills) {
    damageMap.set(skill, skill.calculateDamagePerCast(championBonuses));
  }

  // Filter out skills with no damage
  const damagingSkills = skills.filter(
    (skill) => (damageMap.get(skill) ?? 0) > 0,
  );

  // Group by baseSkillName and pick the highest damage version from each group
  const skillsByBase = new Map<string, Skill>();
  for (const skill of damagingSkills) {
    const key = `${skill.className}-${skill.baseSkillName}`;
    const existing = skillsByBase.get(key);
    const skillDamage = damageMap.get(skill) ?? 0;
    const existingDamage = existing ? (damageMap.get(existing) ?? 0) : 0;
    if (!existing || skillDamage > existingDamage) {
      skillsByBase.set(key, skill);
    }
  }

  const rankedSkills = Array.from(skillsByBase.values()).sort(
    (a, b) => (damageMap.get(b) ?? 0) - (damageMap.get(a) ?? 0),
  );

  if (rankedSkills.length === 0) {
    logger.warn('No damaging skills found.');
    return;
  }

  logger.log(formatTable(rankedSkills, damageMap, limit));
}

const rankCommand = new Command('rank')
  .description('Rank skills by damage per cast')
  .option('-l, --limit <number>', 'Number of skills to show', '20')
  .option('--exclude-ultimates', 'Exclude ultimate abilities', false)
  .option(
    '-s, --source <sources>',
    'Only include skills from specified sources (comma-separated)',
  )
  .option(
    '-m, --mechanic <mechanics>',
    'Only include skills of specified mechanic (comma-separated)',
  )
  .option(
    '--champion-bonus <championBonuses>',
    'Apply list of champion bonuses (comma-separated)',
  )
  .action(action);

export { rankCommand };
