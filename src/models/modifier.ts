export interface DamageModifier {
  name: string;
  value: number;
  maxLevel: number;
  affects: 'critical' | 'aoe' | 'single' | 'direct' | 'dot' | 'off-balance';
}
