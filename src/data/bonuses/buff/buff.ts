import { BuffBonus } from './types';

const MINOR_PROPHECY: BuffBonus = {
  id: 'Minor Prophecy',
  className: 'duration',
  type: 'spell-critical-chance',
  value: 1314, // REVIEW value and duration
};

const MINOR_SAVAGERY: BuffBonus = {
  id: 'Minor Savagery',
  className: 'duration',
  type: 'weapon-critical-chance',
  value: 1314, // REVIEW value and duration
};

const BUFFS: BuffBonus[] = [MINOR_SAVAGERY, MINOR_PROPHECY];

export { BUFFS, MINOR_PROPHECY, MINOR_SAVAGERY };
