import { Command, Option } from 'commander';

import { ClassClassName, WeaponSkillLineName } from '../data/skills';
import { logger } from '../infrastructure';
import { BUILD_CONSTRAINTS } from '../models/build';
import { BuildOptimizer } from '../services/build-optimizer';
import {
  CLASS_NAMES,
  SKILL_LINES_NAMES,
  SkillsService,
} from '../services/skills-service';

interface OptimizeOptions {
  classes?: ClassClassName[];
  weapons?: WeaponSkillLineName[];
  morphs?: string[];
  verbose: boolean;
  parallelism?: number;
}

async function action(options: OptimizeOptions) {
  logger.info('Finding optimal build...');

  const optimizer = new BuildOptimizer({
    verbose: options.verbose,
    requiredClassNames: options.classes,
    requiredWeapons: options.weapons,
    forcedMorphs: options.morphs,
    workers: options.parallelism,
  });
  const build = await optimizer.findOptimalBuild();

  if (!build) {
    logger.error('No valid build found with the given constraints.');
    process.exit(1);
  }

  logger.log(build.toString());
}

const classOption = new Option(
  '-c, --classes <classes>',
  `Require at least 1 skill line from these classes (comma-separated, max ${BUILD_CONSTRAINTS.classSkillLineCount})`,
);
classOption.argParser((value: string): ClassClassName[] => {
  const classes = value.split(',').map((c) => c.trim()) as ClassClassName[];

  if (classes.length > BUILD_CONSTRAINTS.classSkillLineCount) {
    throw new Error(
      `Maximum ${BUILD_CONSTRAINTS.classSkillLineCount} classes allowed`,
    );
  }

  const validClassNames = CLASS_NAMES.filter(
    (className) => className !== 'Weapon',
  );
  for (const c of classes) {
    if (!validClassNames.includes(c)) {
      throw new Error(
        `Invalid class "${c}". Valid options: ${validClassNames.join(', ')}`,
      );
    }
  }

  return classes;
});

const weaponOption = new Option(
  '-w, --weapons <weapons>',
  `Require at least 1 skill line from these weapons (comma-separated, max ${BUILD_CONSTRAINTS.weaponSkillLineCount})`,
);
weaponOption.argParser((value: string): WeaponSkillLineName[] => {
  const weapons = value
    .split(',')
    .map((w) => w.trim()) as WeaponSkillLineName[];

  if (weapons.length > BUILD_CONSTRAINTS.weaponSkillLineCount) {
    throw new Error(
      `Maximum ${BUILD_CONSTRAINTS.weaponSkillLineCount} weapons allowed`,
    );
  }

  const validWeaponSkillLineNames = SKILL_LINES_NAMES.filter(
    (skillLineName): skillLineName is WeaponSkillLineName =>
      SkillsService.getClass(skillLineName) === 'Weapon',
  );
  for (const w of weapons) {
    if (!validWeaponSkillLineNames.includes(w)) {
      throw new Error(
        `Invalid weapon "${w}". Valid options: ${validWeaponSkillLineNames.join(', ')}`,
      );
    }
  }

  return weapons;
});

const morphsOption = new Option(
  '-m, --morphs <morphs>',
  'Force specific morph selections (comma-separated morph names)',
);
morphsOption.argParser((value: string): string[] => {
  return value.split(',').map((m) => m.trim());
});

const optimizeCommand = new Command('optimize')
  .description('Find the optimal build to maximize total damage per cast')
  .addOption(classOption)
  .addOption(weaponOption)
  .addOption(morphsOption)
  .option('-v, --verbose', 'Show optimization progress', false)
  .option(
    '-p, --parallelism <number>',
    'Number of worker batches (default: CPU cores / 2)',
    (value) => parseInt(value, 10),
  )
  .action(action);

export { optimizeCommand };
