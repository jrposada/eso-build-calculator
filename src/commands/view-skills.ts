import { Command } from 'commander';

import { ALL_SKILLS } from '../data/skills';
import { logger } from '../infrastructure';
import { DotDamage } from '../models/skill';
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
  dots: DotDamage[];
  duration: number;
  damagePerCast: number;
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
    if (skill.dots.length > 0) {
      lines.push(`  DoTs:`);
      skill.dots.forEach((dot, j) => {
        const interval =
          dot.interval !== undefined ? ` every ${dot.interval}s` : '';
        const increase = dot.increasePerTick
          ? ` (+${(dot.increasePerTick * 100).toFixed(0)}%/tick)`
          : '';
        const flatIncrease = dot.flatIncreasePerTick
          ? ` (+${dot.flatIncreasePerTick}/tick)`
          : '';
        lines.push(
          `    ${j + 1}. ${dot.value}${interval} for ${dot.duration}s${increase}${flatIncrease}`,
        );
      });
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
    dots: skill.damage.dots ?? [],
    duration: getSkillDuration(skill),
    damagePerCast: calculateDamagePerCast(skill),
  };
}

function action(name: string) {
  const skill = ALL_SKILLS.find(
    (skill) => name.trim().toLowerCase() === skill.name.toLowerCase(),
  );

  if (!skill) {
    logger.warn('Skill not found.');
    return;
  }

  const skillsData: SkillData[] = [mapSkillToData(skill)];

  logger.log(formatTable(skillsData));
}

const viewCommand = new Command('view')
  .description('View skill data')
  .argument('<name>', 'Skill names (comma-separated)')
  .action(action);

export { viewCommand };
