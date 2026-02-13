/// Calculate the number of combinations C(n, k).
pub fn count_combinations(n: usize, k: usize) -> u64 {
    if n == 0 {
        return if k == 0 { 1 } else { 0 };
    }

    if k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }

    // Use symmetry: C(n, k) = C(n, n-k)
    let k = if k > n - k { n - k } else { k };

    let mut result: u64 = 1;
    for i in 0..k {
        result = result * (n - i) as u64 / (i + 1) as u64;
    }
    result
}

/// Generate all C(n,k) combinations of items
pub fn generate_combinations<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    let mut result = Vec::new();

    fn backtrack<T: Clone>(
        items: &[T],
        k: usize,
        start: usize,
        current: &mut Vec<T>,
        result: &mut Vec<Vec<T>>,
    ) {
        if current.len() == k {
            result.push(current.clone());
            return;
        }

        for i in start..items.len() {
            current.push(items[i].clone());
            backtrack(items, k, i + 1, current, result);
            current.pop();
        }
    }

    backtrack(items, k, 0, &mut Vec::new(), &mut result);
    result
}

use smallvec::SmallVec;

/// Iterator for generating combinations lazily.
/// Tracks which positions changed between consecutive outputs via `first_changed()`.
pub struct CombinationIterator<'a, T> {
    items: &'a [T],
    k: usize,
    indices: Vec<usize>,
    done: bool,
    /// Leftmost position that differs between previous and most-recently-returned combo.
    first_changed: usize,
    /// Staged value: leftmost position that will change in the NEXT combo.
    pending_first_changed: usize,
}

impl<'a, T> CombinationIterator<'a, T> {
    pub fn new(items: &'a [T], k: usize) -> Self {
        let done = k > items.len();
        let indices = if done { Vec::new() } else { (0..k).collect() };
        Self {
            items,
            k,
            indices,
            done,
            first_changed: 0,
            pending_first_changed: 0,
        }
    }

    /// Leftmost position that changed between the previous and most-recently-returned
    /// combination. For the very first combination, returns 0.
    #[inline]
    pub fn first_changed(&self) -> usize {
        self.first_changed
    }
}

impl<'a, T: Clone> Iterator for CombinationIterator<'a, T> {
    type Item = SmallVec<[T; 10]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // Promote the staged first_changed computed during the previous advance.
        self.first_changed = self.pending_first_changed;

        if self.k == 0 {
            self.done = true;
            return Some(SmallVec::new());
        }

        // Generate current combination
        let result: SmallVec<[T; 10]> = self
            .indices
            .iter()
            .map(|&i| self.items[i].clone())
            .collect();

        // Find rightmost index that can be incremented
        let n = self.items.len();
        let mut i = self.k;
        while i > 0 {
            i -= 1;
            if self.indices[i] < n - self.k + i {
                // Increment this index and reset all following indices
                self.indices[i] += 1;
                for j in (i + 1)..self.k {
                    self.indices[j] = self.indices[j - 1] + 1;
                }
                self.pending_first_changed = i;
                return Some(result);
            }
        }

        // No more combinations
        self.done = true;
        Some(result)
    }
}

/// Compute the Cartesian product of two arrays of arrays.
/// Each result is the concatenation of one element from the first array with one from the second.
pub fn cartesian_product<T: Clone>(first: &[Vec<T>], second: &[Vec<T>]) -> Vec<Vec<T>> {
    first
        .iter()
        .flat_map(|a| {
            second.iter().map(move |b| {
                let mut combined = a.clone();
                combined.extend(b.iter().cloned());
                combined
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_combinations() {
        assert_eq!(count_combinations(5, 2), 10);
        assert_eq!(count_combinations(5, 3), 10);
        assert_eq!(count_combinations(3, 0), 1);
        assert_eq!(count_combinations(3, 3), 1);
        assert_eq!(count_combinations(0, 0), 1);
        assert_eq!(count_combinations(0, 1), 0);
        assert_eq!(count_combinations(3, 5), 0);
    }

    #[test]
    fn test_generate_combinations() {
        let items = vec![1, 2, 3, 4];
        let combos = generate_combinations(&items, 2);
        assert_eq!(combos.len(), 6);
        assert!(combos.contains(&vec![1, 2]));
        assert!(combos.contains(&vec![1, 3]));
        assert!(combos.contains(&vec![1, 4]));
        assert!(combos.contains(&vec![2, 3]));
        assert!(combos.contains(&vec![2, 4]));
        assert!(combos.contains(&vec![3, 4]));
    }

    #[test]
    fn test_combination_iterator_first_changed() {
        let items = vec![0, 1, 2, 3, 4];
        let mut iter = CombinationIterator::new(&items, 3);

        // First combo [0,1,2]: first_changed = 0 (all new)
        let c0 = iter.next().unwrap();
        assert_eq!(&c0[..], &[0, 1, 2]);
        assert_eq!(iter.first_changed(), 0);

        // [0,1,3]: only position 2 changed
        let c1 = iter.next().unwrap();
        assert_eq!(&c1[..], &[0, 1, 3]);
        assert_eq!(iter.first_changed(), 2);

        // [0,1,4]: only position 2 changed
        let c2 = iter.next().unwrap();
        assert_eq!(&c2[..], &[0, 1, 4]);
        assert_eq!(iter.first_changed(), 2);

        // [0,2,3]: positions 1 and 2 changed (carry)
        let c3 = iter.next().unwrap();
        assert_eq!(&c3[..], &[0, 2, 3]);
        assert_eq!(iter.first_changed(), 1);

        // [0,2,4]: only position 2 changed
        let c4 = iter.next().unwrap();
        assert_eq!(&c4[..], &[0, 2, 4]);
        assert_eq!(iter.first_changed(), 2);

        // [0,3,4]: positions 1 and 2 changed (carry)
        let c5 = iter.next().unwrap();
        assert_eq!(&c5[..], &[0, 3, 4]);
        assert_eq!(iter.first_changed(), 1);

        // [1,2,3]: positions 0, 1, 2 all changed (carry)
        let c6 = iter.next().unwrap();
        assert_eq!(&c6[..], &[1, 2, 3]);
        assert_eq!(iter.first_changed(), 0);
    }

    #[test]
    fn test_cartesian_product() {
        let first = vec![vec![1, 2], vec![3, 4]];
        let second = vec![vec![5], vec![6]];
        let result = cartesian_product(&first, &second);
        assert_eq!(result.len(), 4);
        assert!(result.contains(&vec![1, 2, 5]));
        assert!(result.contains(&vec![1, 2, 6]));
        assert!(result.contains(&vec![3, 4, 5]));
        assert!(result.contains(&vec![3, 4, 6]));
    }
}
