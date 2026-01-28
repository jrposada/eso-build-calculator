import { Command } from 'commander';

import { ALL_SKILLS } from '../data/skills';
import { logger } from '../infrastructure';
import { Skill } from '../models/skill';

function action(name: string) {
  const skillData = ALL_SKILLS.find(
    (s) => name.trim().toLowerCase() === s.name.toLowerCase(),
  );

  if (!skillData) {
    logger.warn('Skill not found.');
    return;
  }

  const skill = Skill.fromData(skillData);

  logger.log(skill.toString());
  logger.log('');
}

const viewCommand = new Command('view')
  .description('View skill data')
  .argument('<name>', 'Skill names (comma-separated)')
  .action(action);

export { viewCommand };
