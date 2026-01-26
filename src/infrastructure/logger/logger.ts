import chalk from 'chalk';

import { Logger } from './logger.types';

export const logger: Logger = {
  log: (message: string, ...args: unknown[]): void => {
    console.log(message, ...args);
  },

  info: (message: string, ...args: unknown[]): void => {
    console.log(chalk.blue(message), ...args);
  },

  warn: (message: string, ...args: unknown[]): void => {
    console.warn(chalk.yellow(message), ...args);
  },

  error: (message: string, ...args: unknown[]): void => {
    console.error(chalk.red(message), ...args);
  },

  success: (message: string, ...args: unknown[]): void => {
    console.log(chalk.green(message), ...args);
  },

  dim: (message: string, ...args: unknown[]): void => {
    console.log(chalk.dim(message), ...args);
  },
};
