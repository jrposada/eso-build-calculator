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

export { generateCombinations };
