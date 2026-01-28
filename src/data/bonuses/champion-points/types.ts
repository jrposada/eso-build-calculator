import { BonusData } from '../types';

export type ChampionPointBonus = BonusData<'passive'> & {
  name: NonNullable<BonusData<'passive'>['name']>;
};
