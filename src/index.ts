#!/usr/bin/env node

import { Command } from 'commander';

import { rankCommand } from './commands/rank-skills';

const program = new Command();

program.name('eso').description('ESO Build Calculator CLI').version('1.0.0');

program.addCommand(rankCommand);

program.parse();
