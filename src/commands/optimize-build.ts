import { Command, Option } from 'commander';

import { ClassName } from '../data/types';
import { logger } from '../infrastructure';
import { BUILD_CONSTRAINTS } from '../models/build';
import { BuildOptimizer } from '../services/build-optimizer';

interface OptimizeOptions {
  class?: ClassName;
  verbose: boolean;
}

function action(options: OptimizeOptions) {
  if (options.verbose) {
    logger.info('Finding optimal build...');
    logger.info(`Constraints: ${JSON.stringify(BUILD_CONSTRAINTS)}`);
    if (options.class) {
      logger.info(`Required class: ${options.class}`);
    }
  }

  const optimizer = new BuildOptimizer({
    verbose: options.verbose,
    className: options.class,
  });
  const build = optimizer.findOptimalBuild().build;

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
  .action(action);

export { optimizeCommand };
