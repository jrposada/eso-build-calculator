import chalk from 'chalk';

import { Logger } from './logger.types';

let lastWasProgress = false;

const clearProgress = (): void => {
  if (lastWasProgress) {
    process.stdout.write('\r\x1b[K');
    lastWasProgress = false;
  }
};

export const logger: Logger = {
  log: (message: string, ...args: unknown[]): void => {
    clearProgress();
    console.log(message, ...args);
  },

  info: (message: string, ...args: unknown[]): void => {
    clearProgress();
    console.log(chalk.blue(message), ...args);
  },

  warn: (message: string, ...args: unknown[]): void => {
    clearProgress();
    console.warn(chalk.yellow(message), ...args);
  },

  error: (message: string, ...args: unknown[]): void => {
    clearProgress();
    console.error(chalk.red(message), ...args);
  },

  success: (message: string, ...args: unknown[]): void => {
    clearProgress();
    console.log(chalk.green(message), ...args);
  },

  dim: (message: string, ...args: unknown[]): void => {
    clearProgress();
    console.log(chalk.dim(message), ...args);
  },

  progress: (message: string): void => {
    if (lastWasProgress) {
      process.stdout.write('\r\x1b[K');
    }
    process.stdout.write(chalk.dim(message));
    lastWasProgress = true;
  },
};
