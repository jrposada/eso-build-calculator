/**
 * Calculate the number of combinations C(n, k).
 */
function countCombinations<T>(items: T[], k: number): number {
  if (items.length === 0) return k === 0 ? 1 : 0;

  if (k < 0 || k > items.length) return 0;
  if (k === 0 || k === items.length) return 1;

  // Use symmetry: C(n, k) = C(n, n-k)
  if (k > items.length - k) {
    k = items.length - k;
  }

  let result = 1;
  for (let i = 0; i < k; i++) {
    result = (result * (items.length - i)) / (i + 1);
  }
  return Math.round(result);
}

/**
 * Generate all C(n,k) combinations of items
 */
function generateCombinations<T>(items: T[], k: number): T[][] {
  const result: T[][] = [];

  function backtrack(start: number, current: T[]): void {
    if (current.length === k) {
      result.push([...current]);
      return;
    }

    for (let i = start; i < items.length; i++) {
      const item = items[i];
      if (item !== undefined) {
        current.push(item);
        backtrack(i + 1, current);
        current.pop();
      }
    }
  }

  backtrack(0, []);
  return result;
}

/**
 * Generator version of generateCombinations for memory-efficient iteration.
 * Yields combinations one at a time instead of collecting them all.
 */
function* generateCombinationsIterator<T>(
  items: T[],
  k: number,
): Generator<T[], void, unknown> {
  function* backtrack(start: number, current: T[]): Generator<T[]> {
    if (current.length === k) {
      yield [...current];
      return;
    }

    for (let i = start; i < items.length; i++) {
      const item = items[i];
      if (item !== undefined) {
        current.push(item);
        yield* backtrack(i + 1, current);
        current.pop();
      }
    }
  }

  yield* backtrack(0, []);
}

/**
 * Compute the Cartesian product of two arrays of arrays.
 * Each result is the concatenation of one element from the first array with one from the second.
 */
function cartesianProduct<T, U>(first: T[][], second: U[][]): (T | U)[][] {
  return first.flatMap((a) => second.map((b) => [...a, ...b]));
}

export {
  cartesianProduct,
  countCombinations,
  generateCombinations,
  generateCombinationsIterator,
};
