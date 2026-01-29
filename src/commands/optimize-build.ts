import { Command, Option } from 'commander';

import { CLASS_CLASS_NAMES, ClassClassName } from '../data/skills';
import { logger } from '../infrastructure';
import { BuildOptimizer } from '../services/build-optimizer';

interface OptimizeOptions {
  classes?: ClassClassName[];
  verbose: boolean;
  workers?: number;
  threads?: number;
}

async function action(options: OptimizeOptions) {
  logger.info('Finding optimal build...');

  const optimizer = new BuildOptimizer({
    verbose: options.verbose,
    classNames: options.classes,
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
  '-c, --classes <classes>',
  'Require at least 1 skill line from these classes (comma-separated, max 3)',
);
classOption.argParser((value: string): ClassClassName[] => {
  const classes = value.split(',').map((c) => c.trim()) as ClassClassName[];

  if (classes.length > 3) {
    throw new Error('Maximum 3 classes allowed');
  }

  for (const c of classes) {
    if (!CLASS_CLASS_NAMES.includes(c)) {
      throw new Error(
        `Invalid class "${c}". Valid options: ${CLASS_CLASS_NAMES.join(', ')}`,
      );
    }
  }

  return classes;
});

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
