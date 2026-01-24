#!/usr/bin/env node

import { Command } from 'commander';

import { compareCommand } from './commands/compare-skills';

const program = new Command();

program
  .name('eso-calc')
  .description('ESO Build Calculator CLI')
  .version('1.0.0');

program.addCommand(compareCommand);

program.parse();
