import { Command } from 'commander';

import { ALL_SKILLS } from '../data/skills';
import {
  AnySkill,
  calculateDamagePerCast,
  getSkillDuration,
  getSkillMechanic,
  getSkillSource,
} from '../services/skill-service';

interface SkillData {
  name: string;
  baseSkillName: string;
  source: string;
  skillLine: string;
  resource: string;
  damageType: string;
  targetType: string;
  mechanic: string;
  channelTime: number | null;
  hits: Array<{ value: number; delay?: number }>;
  dot: number | null;
  dotDuration: number | null;
  dotInterval: number | null;
  dotIncreasePerTick: number | null;
  duration: number;
  damagePerCast: number;
}

interface ViewOptions {
  format: 'table' | 'json';
}

function formatTable(skills: SkillData[]): string {
  const lines: string[] = [];

  skills.forEach((skill, i) => {
    if (i > 0) lines.push('');
    lines.push('═'.repeat(60));
    lines.push(`  ${skill.name}`);
    lines.push('═'.repeat(60));
    lines.push('');
    lines.push('  Basic Info');
    lines.push('  ' + '─'.repeat(56));
    lines.push(`  Base Skill:      ${skill.baseSkillName}`);
    lines.push(`  Source:          ${skill.source}`);
    lines.push(`  Skill Line:      ${skill.skillLine}`);
    lines.push(`  Resource:        ${skill.resource}`);
    lines.push(`  Damage Type:     ${skill.damageType}`);
    lines.push(`  Target Type:     ${skill.targetType}`);
    lines.push(`  Mechanic:        ${skill.mechanic}`);
    if (skill.channelTime !== null) {
      lines.push(`  Channel Time:    ${skill.channelTime}s`);
    }
    lines.push('');
    lines.push('  Damage');
    lines.push('  ' + '─'.repeat(56));
    if (skill.hits.length > 0) {
      lines.push(`  Hits:`);
      skill.hits.forEach((hit, j) => {
        const delay = hit.delay !== undefined ? ` (delay: ${hit.delay}s)` : '';
        lines.push(`    ${j + 1}. ${hit.value}${delay}`);
      });
    }
    if (skill.dot !== null) {
      lines.push(`  DoT:             ${skill.dot} per tick`);
    }
    if (skill.dotDuration !== null) {
      lines.push(`  DoT Duration:    ${skill.dotDuration}s`);
    }
    if (skill.dotInterval !== null) {
      lines.push(`  DoT Interval:    ${skill.dotInterval}s`);
    }
    if (skill.dotIncreasePerTick !== null) {
      lines.push(
        `  DoT Increase:    ${(skill.dotIncreasePerTick * 100).toFixed(0)}% per tick`,
      );
    }
    lines.push('');
    lines.push('  Calculated');
    lines.push('  ' + '─'.repeat(56));
    lines.push(
      `  Duration:        ${skill.duration > 0 ? `${skill.duration}s` : 'instant'}`,
    );
    lines.push(`  Damage/Cast:     ${skill.damagePerCast.toFixed(0)}`);
  });

  lines.push('');
  lines.push('═'.repeat(60));
  lines.push(`Showing ${skills.length} skill(s)`);
  lines.push('');

  return lines.join('\n');
}

function formatJson(skills: SkillData[]): string {
  return JSON.stringify(skills, null, 2);
}

function mapSkillToData(skill: AnySkill): SkillData {
  return {
    name: skill.name,
    baseSkillName: skill.baseSkillName,
    source: getSkillSource(skill),
    skillLine: skill.skillLine,
    resource: skill.resource,
    damageType: skill.damageType,
    targetType: skill.targetType,
    mechanic: getSkillMechanic(skill),
    channelTime: skill.channelTime ?? null,
    hits: skill.damage.hits ?? [],
    dot: skill.damage.dot ?? null,
    dotDuration: skill.damage.dotDuration ?? null,
    dotInterval: skill.damage.dotInterval ?? null,
    dotIncreasePerTick: skill.damage.dotIncreasePerTick ?? null,
    duration: getSkillDuration(skill),
    damagePerCast: calculateDamagePerCast(skill),
  };
}

function action(name: string, options: ViewOptions) {
  const skill = ALL_SKILLS.find(
    (skill) => name.trim().toLowerCase() === skill.name.toLowerCase(),
  );

  if (!skill) {
    console.log('Skill not found.');
    return;
  }

  const skillsData: SkillData[] = [mapSkillToData(skill)];

  if (options.format === 'json') {
    console.log(formatJson(skillsData));
  } else {
    console.log(formatTable(skillsData));
  }
}

export const viewCommand = new Command('view')
  .description('View skill data')
  .argument('<name>', 'Skill names (comma-separated)')
  .action(action);
