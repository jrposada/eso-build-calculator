import { BonusData } from '../types';

export type BuffBonus = BonusData<'duration'> & {
  id: NonNullable<BonusData<'duration'>['id']>;
};
