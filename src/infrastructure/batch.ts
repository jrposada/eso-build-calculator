/**
 * Split an array into batches of a specified size
 */
function batch<T>(items: T[], size: number): T[][] {
  if (size <= 0) {
    throw new Error('Batch size must be greater than 0');
  }

  const batches: T[][] = [];
  for (let i = 0; i < items.length; i += size) {
    batches.push(items.slice(i, i + size));
  }
  return batches;
}

export { batch };
