import { describe, expect, it } from 'vitest';

import {
  cartesianProduct,
  countGroupedCombinations,
  generateCombinations,
  generateGroupedCombinationsIterator,
} from './combinations';

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

describe('cartesianProduct', () => {
  it('returns single empty array for empty input', () => {
    const result = [...cartesianProduct([])];
    expect(result).toEqual([[]]);
  });

  it('returns elements as single-item arrays for single array input', () => {
    const result = [...cartesianProduct([[1, 2, 3]])];
    expect(result).toEqual([[1], [2], [3]]);
  });

  it('returns all pairs for two arrays', () => {
    const result = [...cartesianProduct([[1, 2], ['a', 'b']])];
    expect(result).toEqual([
      [1, 'a'],
      [1, 'b'],
      [2, 'a'],
      [2, 'b'],
    ]);
  });

  it('returns all triples for three arrays', () => {
    const result = [...cartesianProduct([[1], ['a', 'b'], ['x', 'y']])];
    expect(result).toEqual([
      [1, 'a', 'x'],
      [1, 'a', 'y'],
      [1, 'b', 'x'],
      [1, 'b', 'y'],
    ]);
  });

  it('handles arrays of different lengths', () => {
    const result = [...cartesianProduct([[1, 2, 3], ['a']])];
    expect(result).toEqual([
      [1, 'a'],
      [2, 'a'],
      [3, 'a'],
    ]);
  });
});

describe('generateGroupedCombinationsIterator', () => {
  interface Skill {
    name: string;
    baseSkillName: string;
  }

  it('generates valid combinations without duplicate base skills', () => {
    const skills: Skill[] = [
      { name: 'A1', baseSkillName: 'A' },
      { name: 'A2', baseSkillName: 'A' },
      { name: 'B1', baseSkillName: 'B' },
      { name: 'B2', baseSkillName: 'B' },
    ];

    const result = [
      ...generateGroupedCombinationsIterator(
        skills,
        2,
        (s) => s.baseSkillName,
      ),
    ];

    // Should have C(2,2) * 2^2 = 1 * 4 = 4 combinations
    expect(result).toHaveLength(4);

    // Each combination should have skills from different base skills
    for (const combo of result) {
      const baseSkills = combo.map((s) => s.baseSkillName);
      const uniqueBaseSkills = new Set(baseSkills);
      expect(uniqueBaseSkills.size).toBe(combo.length);
    }

    // Verify all expected combinations
    const names = result.map((combo) =>
      combo
        .map((s) => s.name)
        .sort()
        .join(','),
    );
    expect(names.sort()).toEqual(['A1,B1', 'A1,B2', 'A2,B1', 'A2,B2'].sort());
  });

  it('handles groups with single items', () => {
    const skills: Skill[] = [
      { name: 'A1', baseSkillName: 'A' },
      { name: 'B1', baseSkillName: 'B' },
      { name: 'C1', baseSkillName: 'C' },
    ];

    const result = [
      ...generateGroupedCombinationsIterator(
        skills,
        2,
        (s) => s.baseSkillName,
      ),
    ];

    // C(3,2) * 1^2 = 3 combinations
    expect(result).toHaveLength(3);
  });

  it('returns empty when k exceeds number of groups', () => {
    const skills: Skill[] = [
      { name: 'A1', baseSkillName: 'A' },
      { name: 'A2', baseSkillName: 'A' },
    ];

    const result = [
      ...generateGroupedCombinationsIterator(
        skills,
        2,
        (s) => s.baseSkillName,
      ),
    ];

    // Only 1 group, can't pick 2 items from different groups
    expect(result).toHaveLength(0);
  });

  it('works with k=0', () => {
    const skills: Skill[] = [
      { name: 'A1', baseSkillName: 'A' },
      { name: 'B1', baseSkillName: 'B' },
    ];

    const result = [
      ...generateGroupedCombinationsIterator(
        skills,
        0,
        (s) => s.baseSkillName,
      ),
    ];

    expect(result).toEqual([[]]);
  });
});

describe('countGroupedCombinations', () => {
  interface Skill {
    name: string;
    baseSkillName: string;
  }

  it('counts combinations with equal group sizes', () => {
    const skills: Skill[] = [
      { name: 'A1', baseSkillName: 'A' },
      { name: 'A2', baseSkillName: 'A' },
      { name: 'B1', baseSkillName: 'B' },
      { name: 'B2', baseSkillName: 'B' },
    ];

    // C(2,2) * 2^2 = 1 * 4 = 4
    const count = countGroupedCombinations(skills, 2, (s) => s.baseSkillName);
    expect(count).toBe(4);
  });

  it('counts combinations with different group sizes', () => {
    const skills: Skill[] = [
      { name: 'A1', baseSkillName: 'A' },
      { name: 'A2', baseSkillName: 'A' },
      { name: 'A3', baseSkillName: 'A' },
      { name: 'B1', baseSkillName: 'B' },
      { name: 'B2', baseSkillName: 'B' },
      { name: 'C1', baseSkillName: 'C' },
    ];

    // Groups: A(3), B(2), C(1)
    // k=2: C(3,2) combinations of groups
    // AB: 3*2=6, AC: 3*1=3, BC: 2*1=2 = total 11
    const count = countGroupedCombinations(skills, 2, (s) => s.baseSkillName);
    expect(count).toBe(11);
  });

  it('matches actual generated count', () => {
    const skills: Skill[] = [
      { name: 'A1', baseSkillName: 'A' },
      { name: 'A2', baseSkillName: 'A' },
      { name: 'B1', baseSkillName: 'B' },
      { name: 'B2', baseSkillName: 'B' },
      { name: 'C1', baseSkillName: 'C' },
    ];

    const count = countGroupedCombinations(skills, 2, (s) => s.baseSkillName);
    const generated = [
      ...generateGroupedCombinationsIterator(skills, 2, (s) => s.baseSkillName),
    ];

    expect(count).toBe(generated.length);
  });

  it('returns 0 when k exceeds number of groups', () => {
    const skills: Skill[] = [
      { name: 'A1', baseSkillName: 'A' },
      { name: 'A2', baseSkillName: 'A' },
    ];

    const count = countGroupedCombinations(skills, 2, (s) => s.baseSkillName);
    expect(count).toBe(0);
  });

  it('returns 1 when k=0', () => {
    const skills: Skill[] = [
      { name: 'A1', baseSkillName: 'A' },
      { name: 'B1', baseSkillName: 'B' },
    ];

    const count = countGroupedCombinations(skills, 0, (s) => s.baseSkillName);
    expect(count).toBe(1);
  });

  it('returns 0 for negative k', () => {
    const skills: Skill[] = [{ name: 'A1', baseSkillName: 'A' }];

    const count = countGroupedCombinations(skills, -1, (s) => s.baseSkillName);
    expect(count).toBe(0);
  });
});
