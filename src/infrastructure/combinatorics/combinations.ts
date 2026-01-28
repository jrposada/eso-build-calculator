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
 * Generator for Cartesian product of arrays.
 * Yields all possible combinations taking one element from each array.
 */
function* cartesianProduct<T>(arrays: T[][]): Generator<T[], void, unknown> {
  if (arrays.length === 0) {
    yield [];
    return;
  }

  const [first, ...rest] = arrays;
  if (first === undefined) {
    yield [];
    return;
  }

  for (const item of first) {
    for (const restCombination of cartesianProduct(rest)) {
      yield [item, ...restCombination];
    }
  }
}

/**
 * Calculate the number of grouped combinations.
 * This is the count of combinations where at most one item from each group is selected.
 *
 * Formula: Sum over all C(g,k) group combinations of the product of group sizes.
 *
 * @param items - Array of items to combine
 * @param k - Number of items to select
 * @param getGroupKey - Function to extract the group key from an item
 */
function countGroupedCombinations<T>(
  items: T[],
  k: number,
  getGroupKey: (item: T) => string,
): number {
  // Group items by key
  const groupedByKey = new Map<string, number>();
  for (const item of items) {
    const key = getGroupKey(item);
    groupedByKey.set(key, (groupedByKey.get(key) ?? 0) + 1);
  }

  const groupSizes = Array.from(groupedByKey.values());
  const g = groupSizes.length;

  if (k < 0 || k > g) return 0;
  if (k === 0) return 1;

  // Sum over all C(g,k) group combinations of the product of group sizes
  let total = 0;

  function sumProducts(start: number, remaining: number, product: number): void {
    if (remaining === 0) {
      total += product;
      return;
    }

    for (let i = start; i <= g - remaining; i++) {
      sumProducts(i + 1, remaining - 1, product * groupSizes[i]!);
    }
  }

  sumProducts(0, k, 1);
  return total;
}

/**
 * Generate valid combinations where at most one item from each group is selected.
 * Groups items using the provided key function, then generates combinations
 * of k groups and the Cartesian product of items within selected groups.
 *
 * This is more efficient than generating all combinations and filtering,
 * as it only generates valid combinations.
 *
 * @param items - Array of items to combine
 * @param k - Number of items to select
 * @param getGroupKey - Function to extract the group key from an item
 */
function* generateGroupedCombinationsIterator<T>(
  items: T[],
  k: number,
  getGroupKey: (item: T) => string,
): Generator<T[], void, unknown> {
  // Group items by key
  const groupedByKey = new Map<string, T[]>();
  for (const item of items) {
    const key = getGroupKey(item);
    if (!groupedByKey.has(key)) {
      groupedByKey.set(key, []);
    }
    groupedByKey.get(key)!.push(item);
  }

  const groups = Array.from(groupedByKey.values());

  // Generate all combinations of k groups
  for (const groupCombination of generateCombinationsIterator(groups, k)) {
    // Generate Cartesian product of item choices within selected groups
    yield* cartesianProduct(groupCombination);
  }
}

export {
  cartesianProduct,
  countCombinations,
  countGroupedCombinations,
  generateCombinations,
  generateCombinationsIterator,
  generateGroupedCombinationsIterator,
};
