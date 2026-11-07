//! Repository-owned support operations for the modal path (DEL-04-09 Work
//! `modal-analysis.md` §5.3.8, the resource terms `T3.filters`,
//! `T7.assembly` and `T9.construction`). They replace std's
//! `slice::binary_search`, `sort_*`, `BTreeMap` and `VecDeque` there, whose
//! step constants are not repository source, and reproduce exactly the
//! results those operations gave where the callers observe them: the same
//! membership, the same order, the same ties and the same refusals.
//!
//! **Unit.** Counts here are *support steps*: a loop iteration or call
//! body of at most 64 operations, counted literally under the `T3.kernel`
//! convention (PP `modal/resource.rs`): every read of a named value, every
//! arithmetic operation, comparison or predicate, and every binding,
//! assignment, store or push; loop control, index arithmetic, bounds checks
//! and `debug_assert`s excluded. The premises are the adopted ones: a core
//! slice or `Vec` element primitive (`vec![x; k]`'s fill, `clone`,
//! `copy_from_slice`, `reverse`, `clear`) is one step per element, and
//! allocator calls are charged in no term. Each function states its count
//! as a function of its actual sizes; P0 charges those at their bounds.
//!
//! **Capacity.** Every vector here is allocated once at its final length or
//! capacity, which is stated, and never grows: each `push` is within a
//! capacity reserved before it. Nothing is retained after a function
//! returns except its result.

/// The ascending complement of `excluded` in `0..n`: each `i < n` not in
/// `excluded`, ascending. `excluded` must be strictly ascending with every
/// element below `n`, as `dense::structurally_massless_dofs` returns it.
///
/// It is the former `(0..n).filter(|i| excluded.binary_search(i).is_err())`:
/// at index `i` the cursor has passed exactly the elements below `i`, so
/// `excluded[cursor]` is the least element `≥ i`, and `i` is excluded
/// exactly when it equals `i`.
///
/// Count: `n + 1` support steps (one per index, one for the call; the body
/// is at most 9 operations). Capacity: exactly `n − |excluded|`.
pub(crate) fn ascending_complement(excluded: &[usize], n: usize) -> Vec<usize> {
    debug_assert!(excluded.windows(2).all(|w| w[0] < w[1]) && excluded.last().is_none_or(|&last| last < n));
    let mut out = Vec::with_capacity(n.saturating_sub(excluded.len()));
    let mut cursor = 0;
    for i in 0..n {
        if cursor < excluded.len() && excluded[cursor] == i {
            cursor += 1;
        } else {
            out.push(i);
        }
    }
    out
}

/// A stable counting sort: the items of `sequence` (`len` of them) ordered
/// by `key(item)`, ascending, equal keys keeping their order in `sequence`.
/// Every key must be below `buckets`.
///
/// Count: `buckets` (the starts' fill) + `len` (counting) + `buckets` (the
/// exclusive prefix) + `len` (the output's fill) + `len` (placing) + 1, so
/// `3·len + 2·buckets + 1` support steps; every body is at most 12
/// operations. Capacity: `buckets` starts, freed on return, and the result,
/// exactly `len`. No sum overflows: the running total is at most `len`.
pub(crate) fn stable_counting_order(
    sequence: impl Iterator<Item = usize> + Clone,
    len: usize,
    buckets: usize,
    key: impl Fn(usize) -> usize,
) -> Vec<usize> {
    let mut starts = vec![0usize; buckets];
    for item in sequence.clone() {
        starts[key(item)] += 1;
    }
    let mut total = 0;
    for start in starts.iter_mut() {
        let count = *start;
        *start = total;
        total += count;
    }
    debug_assert_eq!(total, len);
    let mut out = vec![0usize; len];
    for item in sequence {
        let bucket = key(item);
        out[starts[bucket]] = item;
        starts[bucket] += 1;
    }
    out
}

/// The node of `nodes` least by (degree, index), the first on a tie (a tie
/// cannot occur: indices differ). It is the former
/// `nodes.iter().copied().min_by_key(|&v| (degrees[v], v))`.
///
/// Count: `|nodes| + 1` support steps (each body at most 14 operations).
fn least_by_degree(nodes: &[usize], degree: &[usize]) -> Option<usize> {
    let mut best: Option<usize> = None;
    for &v in nodes {
        if best.is_none_or(|b| (degree[v], v) < (degree[b], b)) {
            best = Some(v);
        }
    }
    best
}

/// The neighbour lists of the reverse Cuthill–McKee order, in compressed
/// form, and the traversal state reused by every traversal.
struct Traversal {
    /// `degree[v]`, the distinct neighbours of v.
    degree: Vec<usize>,
    /// `start[v]`, the offset of v's list in `neighbours`.
    start: Vec<usize>,
    /// Each node's neighbours by (degree, index), ascending.
    neighbours: Vec<usize>,
    /// `mark[v] == stamp` exactly when v is marked in the current traversal.
    mark: Vec<usize>,
    stamp: usize,
    /// The current traversal's nodes in discovery order (capacity n).
    buffer: Vec<usize>,
}

impl Traversal {
    fn list(&self, v: usize) -> &[usize] {
        &self.neighbours[self.start[v]..self.start[v] + self.degree[v]]
    }

    /// Breadth-first from `start`: `buffer` holds the discovered nodes level
    /// by level. Returns the eccentricity (the levels after the first) and
    /// the last nonempty level's range in `buffer` (the former
    /// `bfs_eccentricity`; its `buffer` is also the former `bfs_reachable`'s
    /// result, the same nodes in the same order).
    ///
    /// Count, over a component of m nodes and h half-edges: 1 (the call) +
    /// m (each node's discovery is within its parent's neighbour step, and
    /// each pop) + h (one step per neighbour checked) + (levels + 1) level
    /// closings ≤ m + 1, so at most `3m + h + 2` support steps (each body
    /// at most 20 operations).
    fn breadth_first(&mut self, start: usize) -> (usize, usize, usize) {
        self.stamp += 1;
        let stamp = self.stamp;
        self.buffer.clear();
        self.buffer.push(start);
        self.mark[start] = stamp;
        let (mut low, mut high) = (0, 1);
        let mut eccentricity = 0;
        let mut last = (0, 1);
        loop {
            for index in low..high {
                let node = self.buffer[index];
                let (from, to) = (self.start[node], self.start[node] + self.degree[node]);
                for k in from..to {
                    let next = self.neighbours[k];
                    if self.mark[next] != stamp {
                        self.mark[next] = stamp;
                        self.buffer.push(next);
                    }
                }
            }
            if self.buffer.len() == high {
                break;
            }
            eccentricity += 1;
            last = (high, self.buffer.len());
            low = high;
            high = self.buffer.len();
        }
        (eccentricity, last.0, last.1)
    }

    /// The pseudo-peripheral start of `seed`'s component (the former
    /// `pseudo_peripheral_start`): the component's least node by (degree,
    /// index), then, while the least node of the last level has a strictly
    /// larger eccentricity, that node.
    ///
    /// Traversals over a component of m nodes: one for reachability, one
    /// for the first candidate, at most m − 1 accepted (the eccentricity
    /// strictly grows and is at most m − 1) and one refused: at most m + 2.
    /// Scans: `least_by_degree` over the component and over each last
    /// level, at most (m + 2)(m + 1) steps.
    fn pseudo_peripheral(&mut self, seed: usize) -> usize {
        self.breadth_first(seed);
        let mut candidate = least_by_degree(&self.buffer, &self.degree).unwrap_or(seed);
        let (mut eccentricity, mut low, mut high) = self.breadth_first(candidate);
        while let Some(next) = least_by_degree(&self.buffer[low..high], &self.degree) {
            // The candidate's last level is no longer needed once `next` is
            // chosen: an accepted `next` replaces it, a refused one stops.
            let (next_eccentricity, next_low, next_high) = self.breadth_first(next);
            if next_eccentricity > eccentricity {
                candidate = next;
                eccentricity = next_eccentricity;
                (low, high) = (next_low, next_high);
            } else {
                break;
            }
        }
        candidate
    }
}

/// The reverse Cuthill–McKee order (`order[k]` is the node of ordered
/// position k) of the symmetric pattern on `n` nodes whose strictly lower
/// positions `lower` yields: ascending by (row, col), each once, with
/// `row > col` and `row < n`. It is exactly `sparse_direct`'s
/// `reverse_cuthill_mckee` on the adjacency lists `ShiftPencil` formerly
/// built from the same positions (both orientations of each): the same
/// neighbour sets, sorted by (degree, index); the same component seeds in
/// ascending order; the same pseudo-peripheral rule; the same
/// breadth-first order; reversed.
///
/// **No sort.** For node v the positions (v, c) (c < v, ascending c) all
/// precede the positions (r, v) (r > v, ascending r), so placing each
/// position's two half-edges in `lower`'s order leaves every list
/// ascending. Ordering the lists by (degree, index) is one stable counting
/// sort of the nodes by degree (degrees are below n), then, for each node
/// w in that order, appending w to the list of each of w's neighbours.
///
/// **Count** with q = |lower| positions, H = 2q half-edges and C ≤ n
/// components, in support steps (each body at most 20 operations):
/// - lists: `degree`'s fill n, the degrees q, `start`'s fill and prefix
///   2n, the ascending lists' fill 2q, the cursor's copy n, placing q, the
///   node order `stable_counting_order` 5n + 1, the lists' fill 2q, the
///   cursor's copy n, and placing n + 2q; `mark`'s and `visited`'s fills
///   2n: so `13n + 8q + 1`;
/// - traversals: per component of m nodes and h half-edges, at most m + 2
///   pseudo-peripheral traversals and the final one, each at most
///   `3m + h + 2` steps; the `least_by_degree` scans, at most
///   (m + 1)(m + 1); and the candidate loop's bodies, at most m. Summed
///   over the components: at most `(n + 3)(3n + H + 2C)` ≤
///   `(n + 3)(5n + 2q)` and `Σ (m + 1)(m + 2)` ≤ `(n + 1)(n + 2C)` ≤
///   `3n(n + 1)`;
/// - the seed loop n, the reversal n and the call 2.
///
/// So at most `15n + 8q + 3 + (n + 3)(5n + 2q) + 3n(n + 1)` steps.
///
/// **Capacity.** `degree`, `start`, the cursor, the node order, `mark` and
/// `visited`: n each; the ascending lists and `neighbours`: H each (the
/// ascending lists are freed before the traversals); `buffer` and the
/// result: capacity n each, never exceeded since a traversal discovers each
/// node at most once. Stamps count traversals, at most 3n, below
/// `usize::MAX`.
pub(crate) fn reverse_cuthill_mckee_lower(
    n: usize,
    lower: impl Iterator<Item = (usize, usize)> + Clone,
) -> Vec<usize> {
    let mut degree = vec![0usize; n];
    let mut half_edges = 0;
    for (row, col) in lower.clone() {
        debug_assert!(col < row && row < n);
        degree[row] += 1;
        degree[col] += 1;
        half_edges += 2;
    }
    let mut start = vec![0usize; n];
    let mut total = 0;
    for (offset, &count) in start.iter_mut().zip(&degree) {
        *offset = total;
        total += count;
    }
    debug_assert_eq!(total, half_edges);
    let mut ascending = vec![0usize; half_edges];
    let mut cursor = start.clone();
    for (row, col) in lower {
        ascending[cursor[row]] = col;
        cursor[row] += 1;
        ascending[cursor[col]] = row;
        cursor[col] += 1;
    }
    debug_assert!((0..n).all(|v| ascending[start[v]..start[v] + degree[v]].windows(2).all(|w| w[0] < w[1])));
    // Nodes by (degree, index): a node has at most n − 1 distinct
    // neighbours, so every degree is a bucket below n.
    let by_degree = stable_counting_order(0..n, n, n, |v| degree[v]);
    let mut neighbours = vec![0usize; half_edges];
    cursor.copy_from_slice(&start);
    for &w in &by_degree {
        for &v in &ascending[start[w]..start[w] + degree[w]] {
            neighbours[cursor[v]] = w;
            cursor[v] += 1;
        }
    }
    drop(ascending);
    drop(cursor);
    drop(by_degree);
    let mut traversal = Traversal { degree, start, neighbours, mark: vec![0; n], stamp: 0, buffer: Vec::with_capacity(n) };
    let mut visited = vec![false; n];
    let mut order = Vec::with_capacity(n);
    for seed in 0..n {
        if visited[seed] {
            continue;
        }
        let start = traversal.pseudo_peripheral(seed);
        visited[start] = true;
        let mut head = order.len();
        order.push(start);
        while head < order.len() {
            let node = order[head];
            head += 1;
            for &next in traversal.list(node) {
                if !visited[next] {
                    visited[next] = true;
                    order.push(next);
                }
            }
        }
    }
    order.reverse();
    order
}

#[cfg(test)]
mod tests {
    //! Old against new: the former std operations are the test-only oracles
    //! (`binary_search`, `sort_by_key`, and `sparse_direct`'s
    //! `reverse_cuthill_mckee` itself), never a production dependency.
    use super::*;
    use open_pipe_stress_sparse_direct::reverse_cuthill_mckee;

    /// A deterministic generator (SplitMix64), test-only.
    struct Mix(u64);
    impl Mix {
        fn next(&mut self) -> u64 {
            self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = self.0;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        }
        fn below(&mut self, n: usize) -> usize {
            (self.next() % n as u64) as usize
        }
    }

    #[test]
    fn the_complement_is_the_binary_search_filter() {
        let oracle = |excluded: &[usize], n: usize| -> Vec<usize> { (0..n).filter(|i| excluded.binary_search(i).is_err()).collect() };
        let cases: Vec<(Vec<usize>, usize)> = vec![
            (vec![], 0),
            (vec![], 1),
            (vec![0], 1),
            (vec![], 5),
            (vec![0, 1, 2, 3, 4], 5),
            (vec![0], 5),
            (vec![4], 5),
            (vec![1, 3], 5),
            (vec![0, 2, 4], 5),
        ];
        for (excluded, n) in &cases {
            let found = ascending_complement(excluded, *n);
            assert_eq!(found, oracle(excluded, *n), "{excluded:?} in 0..{n}");
            assert_eq!(found.capacity(), Vec::<usize>::with_capacity(n - excluded.len()).capacity(), "the reserved capacity, never grown");
        }
        let mut mix = Mix(7);
        for _ in 0..200 {
            let n = mix.below(40);
            let excluded: Vec<usize> = (0..n).filter(|_| mix.below(3) == 0).collect();
            assert_eq!(ascending_complement(&excluded, n), oracle(&excluded, n));
        }
    }

    #[test]
    fn the_counting_order_is_a_stable_sort_by_key() {
        // Equal keys keep their order: the stable std sort is the oracle.
        let mut mix = Mix(11);
        for (len, buckets) in [(0, 0), (0, 3), (1, 1), (7, 1), (9, 2), (50, 7), (300, 40), (40, 300)] {
            let keys: Vec<usize> = (0..len).map(|_| mix.below(buckets.max(1))).collect();
            let found = stable_counting_order(0..len, len, buckets.max(usize::from(len > 0)), |i| keys[i]);
            let mut oracle: Vec<usize> = (0..len).collect();
            oracle.sort_by_key(|&i| keys[i]);
            assert_eq!(found, oracle, "len {len}, buckets {buckets}");
            assert_eq!(found.capacity(), Vec::<usize>::with_capacity(len).capacity());
        }
        // Reversed and constant keys, over a permuted input sequence.
        let order: Vec<usize> = (0..20).rev().collect();
        let reversed = stable_counting_order(order.iter().copied(), 20, 20, |i| i);
        assert_eq!(reversed, (0..20).collect::<Vec<_>>());
        let constant = stable_counting_order(order.iter().copied(), 20, 1, |_| 0);
        assert_eq!(constant, order, "all ties keep the input order");
    }

    /// The adjacency `ShiftPencil` formerly passed to `sparse_direct`.
    fn former_adjacency(n: usize, lower: &[(usize, usize)]) -> Vec<Vec<usize>> {
        let mut adjacency = vec![Vec::new(); n];
        for &(r, c) in lower {
            adjacency[r].push(c);
            adjacency[c].push(r);
        }
        adjacency
    }

    fn check(n: usize, lower: &[(usize, usize)]) {
        let mut sorted = lower.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        let found = reverse_cuthill_mckee_lower(n, sorted.iter().copied());
        let oracle = reverse_cuthill_mckee(&former_adjacency(n, &sorted)).unwrap();
        assert_eq!(found, oracle, "n = {n}, lower = {sorted:?}");
        assert_eq!(found.capacity(), Vec::<usize>::with_capacity(n).capacity(), "the reserved capacity, never grown");
    }

    #[test]
    fn rcm_is_sparse_directs_order_on_structured_patterns() {
        check(0, &[]);
        check(1, &[]);
        check(4, &[]);
        // A path in index order, and in reverse index order.
        check(6, &[(1, 0), (2, 1), (3, 2), (4, 3), (5, 4)]);
        check(6, &[(5, 4), (4, 3), (3, 2), (2, 1), (1, 0)]);
        // A star (degree ties among the leaves), a cycle, a complete graph.
        check(7, &[(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)]);
        check(7, &[(6, 3), (6, 1), (6, 0), (5, 2), (4, 2), (3, 2)]);
        check(5, &[(1, 0), (2, 1), (3, 2), (4, 3), (4, 0)]);
        let complete: Vec<(usize, usize)> = (0..6).flat_map(|r| (0..r).map(move |c| (r, c))).collect();
        check(6, &complete);
        // Several components, singletons between them.
        check(9, &[(1, 0), (2, 1), (5, 4), (8, 6), (7, 6)]);
        // A 3 × 4 grid: equal eccentricities and degree ties.
        let mut grid = Vec::new();
        for i in 0..3 {
            for j in 0..4 {
                let v = 4 * i + j;
                if j > 0 {
                    grid.push((v, v - 1));
                }
                if i > 0 {
                    grid.push((v, v - 4));
                }
            }
        }
        check(12, &grid);
    }

    #[test]
    fn rcm_is_sparse_directs_order_on_random_patterns() {
        let mut mix = Mix(2026);
        for case in 0..400 {
            let n = 1 + mix.below(48);
            let density = 1 + mix.below(6);
            let mut lower = Vec::new();
            for r in 0..n {
                for c in 0..r {
                    if mix.below(n.max(2) * density / 3 + 1) == 0 || (case % 5 == 0 && r == c + 1) {
                        lower.push((r, c));
                    }
                }
            }
            check(n, &lower);
        }
    }
}
