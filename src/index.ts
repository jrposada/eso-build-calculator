#!/usr/bin/env node

import { Command } from 'commander';

import { optimizeCommand } from './commands/optimize-build';
import { rankCommand } from './commands/rank-skills';
import { viewCommand } from './commands/view-skills';

const program = new Command();

program.name('eso').description('ESO Build Calculator CLI').version('1.0.0');

program.addCommand(optimizeCommand);
program.addCommand(rankCommand);
program.addCommand(viewCommand);

program.parse();
