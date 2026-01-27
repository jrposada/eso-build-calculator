import { Command } from 'commander';

import { logger } from '../infrastructure';
import { Build, BuildConstraints } from '../models/build';
import { EsoClass } from '../models/skill';
import { findOptimalBuild } from '../services/build-optimizer';

interface OptimizeOptions {
  format: 'table' | 'json';
  class?: string;
  verbose: boolean;
}

const VALID_CLASSES: EsoClass[] = [
  'Dragonknight',
  'Sorcerer',
  'Nightblade',
  'Warden',
  'Necromancer',
  'Templar',
  'Arcanist',
];

const DEFAULT_CONSTRAINTS: BuildConstraints = {
  maxSkills: 10,
  maxModifiers: 4,
  maxClassSkillLines: 3,
  maxWeaponSkillLines: 2,
};

function formatTable(build: Build): string {
  const divider = '─'.repeat(85);
  const lines: string[] = [
    '',
    'Optimal Build - Maximum Damage Per Cast',
    divider,
    `Total Damage: ${build.totalDamagePerCast.toLocaleString('en-US', { maximumFractionDigits: 0 })}`,
    '',
    `Modifiers: ${build.modifiers.join(', ')}`,
    '',
    'Skills',
    divider,
    formatRow('#', 'Name', 'Source', 'Skill Line', 'Damage'),
    divider,
  ];

  build.skills.forEach((skill, i) => {
    lines.push(
      formatRow(
        (i + 1).toString(),
        skill.name,
        skill.source,
        skill.skillLine,
        skill.damagePerCast.toFixed(0),
      ),
    );
  });

  lines.push(divider);

  // Format skill lines summary
  const classLines = build.usedClassSkillLines.join(', ');
  const weaponLines = build.usedWeaponSkillLines.join(', ');
  const classCount = build.usedClassSkillLines.length;
  const weaponCount = build.usedWeaponSkillLines.length;

  lines.push('');
  lines.push(
    `Skill Lines: ${classLines} (${classCount}/3 class), ${weaponLines} (${weaponCount}/2 weapon)`,
  );

  if (build.requiredClass) {
    lines.push(`Required Class: ${build.requiredClass}`);
  }

  // Format passives section
  if (build.passives.length > 0) {
    lines.push('');
    lines.push('Passives');
    lines.push(divider);
    lines.push(formatPassiveRow('Name', 'Source', 'Skill Line'));
    lines.push(divider);

    build.passives.forEach((passive) => {
      lines.push(
        formatPassiveRow(passive.name, passive.source, passive.skillLine),
      );
    });

    lines.push(divider);
  }

  lines.push('');

  return lines.join('\n');
}

function formatPassiveRow(
  name: string,
  source: string,
  skillLine: string,
): string {
  const nameWidth = 30;
  const sourceWidth = 15;
  const skillLineWidth = 20;

  return `${name.padEnd(nameWidth)} ${source.padEnd(sourceWidth)} ${skillLine.padEnd(skillLineWidth)}`;
}

function formatRow(
  rank: string,
  name: string,
  source: string,
  skillLine: string,
  damage: string,
): string {
  const rankWidth = 4;
  const nameWidth = 25;
  const sourceWidth = 12;
  const skillLineWidth = 18;
  const damageWidth = 10;

  return `${rank.padStart(rankWidth)} ${name.padEnd(nameWidth)} ${source.padEnd(sourceWidth)} ${skillLine.padEnd(skillLineWidth)} ${damage.padStart(damageWidth)}`;
}

function formatJson(build: Build): string {
  return JSON.stringify(build.toJSON(), null, 2);
}

function action(options: OptimizeOptions) {
  // Validate class option if provided
  if (options.class) {
    const normalizedClass = options.class.trim().toLowerCase();
    const matchedClass = VALID_CLASSES.find(
      (c) => c.toLowerCase() === normalizedClass,
    );
    if (!matchedClass) {
      logger.error(
        `Error: Invalid class "${options.class}". Valid classes: ${VALID_CLASSES.join(', ')}`,
      );
      process.exit(1);
    }
    options.class = matchedClass;
  }

  if (options.verbose) {
    logger.info('Finding optimal build...');
    logger.info(`Constraints: ${JSON.stringify(DEFAULT_CONSTRAINTS)}`);
    if (options.class) {
      logger.info(`Required class: ${options.class}`);
    }
  }

  const build = findOptimalBuild(
    DEFAULT_CONSTRAINTS,
    options.class,
    options.verbose,
  );

  if (!build) {
    logger.error('No valid build found with the given constraints.');
    process.exit(1);
  }

  if (options.format === 'json') {
    logger.log(formatJson(build));
  } else {
    logger.log(formatTable(build));
  }
}

export const optimizeCommand = new Command('optimize')
  .description('Find the optimal build to maximize total damage per cast')
  .option('-f, --format <format>', 'Output format (table/json)', 'table')
  .option(
    '-c, --class <class>',
    'Require at least 1 skill line from this class',
  )
  .option('-v, --verbose', 'Show optimization progress', false)
  .action(action);
