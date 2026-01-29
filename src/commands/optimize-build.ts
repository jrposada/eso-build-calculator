import { Command, Option } from 'commander';

import { ClassName } from '../data/types';
import { logger } from '../infrastructure';
import { BUILD_CONSTRAINTS } from '../models/build';
import { BuildOptimizer } from '../services/build-optimizer';

interface OptimizeOptions {
  class?: ClassName;
  verbose: boolean;
  workers?: number;
  threads?: number;
}

async function action(options: OptimizeOptions) {
  logger.info('Finding optimal build...');

  if (options.verbose) {
    logger.info(`Constraints: ${JSON.stringify(BUILD_CONSTRAINTS)}`);
  }

  if (options.class) {
    logger.info(`Required class: ${options.class}`);
  }

  const optimizer = new BuildOptimizer({
    verbose: options.verbose,
    className: options.class,
    workers: options.workers,
    threads: options.threads,
  });
  const build = await optimizer.findOptimalBuild();

  if (!build) {
    logger.error('No valid build found with the given constraints.');
    process.exit(1);
  }

  logger.log(build.toString());
}

const classOption = new Option(
  '-c, --class <class>',
  'Require at least 1 skill line from this class',
);
classOption.choices([
  'Dragonknight',
  'Sorcerer',
  'Nightblade',
  'Warden',
  'Necromancer',
  'Templar',
  'Arcanist',
]);

const optimizeCommand = new Command('optimize')
  .description('Find the optimal build to maximize total damage per cast')
  .addOption(classOption)
  .option('-v, --verbose', 'Show optimization progress', false)
  .option(
    '-w, --workers <number>',
    'Number of worker batches (default: CPU cores - 1)',
    (value) => parseInt(value, 10),
  )
  .option(
    '-t, --threads <number>',
    'Max threads in worker pool (default: CPU cores - 1)',
    (value) => parseInt(value, 10),
  )
  .action(action);

export { optimizeCommand };
