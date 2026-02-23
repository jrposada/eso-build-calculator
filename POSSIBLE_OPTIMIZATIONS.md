# Possible Optimizations

Remaining optimization opportunities for `find_optimal_build`. Items already implemented are listed in the completed section for reference.

---

## Pending

### B3. Branch and bound with upper bound estimation
- **Problem:** Flat `CombinationIterator` cannot skip subtrees. A weak partial combination may lead to billions of evaluations that can never beat the current best.
- **Fix:** Replace lexicographic iterator with recursive DFS. Pre-sort skills by standalone damage (descending). At each node, compute upper bound: `partial_damage + sum_of_top_(10-k)_remaining_skills`. If upper bound < current best, prune.
- **Impact:** MEDIUM-HIGH (data-dependent; potentially 10-100x for diverse pools) | **Complexity:** HIGH
- **Trade-offs:** No loss of optimality. Requires restructuring from flat iterator to tree search. Upper bound must account for bonus modifiers for accuracy.

### B5. Genetic algorithm / simulated annealing (alternative solver)
- **Problem:** For unconstrained searches with billions of combinations, exact search may take hours.
- **Fix:** Implement a metaheuristic solver as an alternative mode (`--strategy ga`). Uses neighborhood operators: swap one skill, swap one CP. Keep brute-force for small search spaces.
- **Impact:** HIGH for huge search spaces | **Complexity:** HIGH
- **Trade-offs:** No optimality guarantee. Best as a complement, not a replacement.

---

## Attempted (no improvement)

### A5. Use `&'static str` for `BonusData.name` and `BonusValue.name`
- **Result:** Benchmarked with no improvement; slight regression on single-CP path (+5.6%). Reverted.

---

## Completed

| ID | Change | Commit |
|----|--------|--------|
| A1 | SmallVec for resolved bonuses | `99029a9` |
| A4 | Fuse Step 1 + Step 3 bonus iteration | `99029a9` |
| A2 | par_bridge() → par_iter() over work units | `99029a9` |
| B1 | Sort + cap skill pool (--max-pool-size) | `99029a9` |
| A3 | Pre-resolve simple bonuses (three_way_split) | `99029a9` |
| C1 | Reorder iteration (skills outermost, CP inner) | `99029a9` |
| C2 | Cache per-skill passive modifier sums across CP combos | `601df9f` |
| C3 | Bit-indexed ModifierLookup for O(1) damage flag queries | `e773eae` |
| B4 | Incremental evaluation for consecutive combinations | `0929982` |
| B2 | Dominated skill pruning (per skill line) | *pending commit* |
