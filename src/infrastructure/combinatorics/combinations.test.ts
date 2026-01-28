import { describe, expect, it } from 'vitest';

import { generateCombinations } from './combinations';

describe('generateCombinations', () => {
  describe('basic functionality', () => {
    it('returns single empty array when k=0', () => {
      const result = generateCombinations([1, 2, 3], 0);
      expect(result).toEqual([[]]);
    });

    it('returns each element as single-item array when k=1', () => {
      const result = generateCombinations([1, 2, 3], 1);
      expect(result).toEqual([[1], [2], [3]]);
    });

    it('returns all pairs when k=2', () => {
      const result = generateCombinations([1, 2, 3], 2);
      expect(result).toEqual([
        [1, 2],
        [1, 3],
        [2, 3],
      ]);
    });

    it('returns single array with all elements when k=n', () => {
      const result = generateCombinations([1, 2, 3], 3);
      expect(result).toEqual([[1, 2, 3]]);
    });
  });

  describe('edge cases', () => {
    it('returns empty array when k > n', () => {
      const result = generateCombinations([1, 2], 3);
      expect(result).toEqual([]);
    });

    it('returns single empty array for empty input with k=0', () => {
      const result = generateCombinations([], 0);
      expect(result).toEqual([[]]);
    });

    it('returns empty array for empty input with k>0', () => {
      const result = generateCombinations([], 1);
      expect(result).toEqual([]);
    });

    it('returns empty array for negative k', () => {
      const result = generateCombinations([1, 2, 3], -1);
      expect(result).toEqual([]);
    });
  });

  describe('combination count verification', () => {
    it('generates C(4,2)=6 combinations', () => {
      const result = generateCombinations([1, 2, 3, 4], 2);
      expect(result).toHaveLength(6);
      expect(result).toEqual([
        [1, 2],
        [1, 3],
        [1, 4],
        [2, 3],
        [2, 4],
        [3, 4],
      ]);
    });

    it('generates C(5,3)=10 combinations', () => {
      const result = generateCombinations([1, 2, 3, 4, 5], 3);
      expect(result).toHaveLength(10);
    });
  });

  describe('generic type support', () => {
    it('works with strings', () => {
      const result = generateCombinations(['a', 'b', 'c'], 2);
      expect(result).toEqual([
        ['a', 'b'],
        ['a', 'c'],
        ['b', 'c'],
      ]);
    });

    it('works with objects', () => {
      const obj1 = { id: 1 };
      const obj2 = { id: 2 };
      const obj3 = { id: 3 };
      const result = generateCombinations([obj1, obj2, obj3], 2);
      expect(result).toEqual([
        [obj1, obj2],
        [obj1, obj3],
        [obj2, obj3],
      ]);
    });
  });

  describe('immutability', () => {
    it('does not modify the original array', () => {
      const original = [1, 2, 3];
      const copy = [...original];
      generateCombinations(original, 2);
      expect(original).toEqual(copy);
    });

    it('returns independent result arrays', () => {
      const result = generateCombinations([1, 2, 3], 2);
      result[0]![0] = 999;
      expect(result[1]![0]).toBe(1);
    });
  });

  describe('order preservation', () => {
    it('maintains original element order within combinations', () => {
      const result = generateCombinations(['first', 'second', 'third'], 2);
      for (const combo of result) {
        const [a, b] = combo;
        const indexA = ['first', 'second', 'third'].indexOf(a!);
        const indexB = ['first', 'second', 'third'].indexOf(b!);
        expect(indexA).toBeLessThan(indexB);
      }
    });
  });
});
