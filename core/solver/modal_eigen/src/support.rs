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
//! slice or `Vec` element primitive (`resize`'s fill, `extend_from_slice`,
//! `copy_from_slice`, `reverse`, `clear`) is one step per element, and
//! allocator calls (here `try_reserve_exact`) are charged in no term. Each
//! function states its count as a function of its actual sizes; P0
//! charges those at their bounds.
//!
//! **Reservations.** A *structural vector* (a complement, a counting
//! order and its starts, the reverse Cuthill–McKee lists and traversal
//! state, and the consumers' key, slot, sorted, off-pattern, scaling and
//! row-sum vectors in `reformation` and `sturm`) is requested once, by
//! [`reserve`] or [`filled`], at an element count its caller computes with
//! checked arithmetic. [`reserve`] checks `len × size_of::<T>()` against
//! `isize::MAX` (a [`SupportResourceError::CapacityOverflow`] refusal, made
//! before any request) and then requests exactly that capacity with
//! `try_reserve_exact`, so an allocator refusal returns
//! [`SupportResourceError::AllocationFailed`] instead of aborting. A
//! reservation is **one support step**: `reserve`'s body (the checked
//! size, the request and, on a refusal, the typed error: at most 20
//! operations) with the caller's `?` or `map_err` on its result. A zero
//! length requests nothing and cannot refuse. A refusal ends its function
//! there with no partial result; each enclosing function's `?` on it is
//! within that function's call step. So a refused call does at most the
//! steps its count states.
//!
//! **Capacity.** Each structural vector then holds at most its requested
//! element count, so it makes no further request (no explicit growth).
//! Stated capacities are *requested* sizes, `len × size_of::<T>()` bytes;
//! the allocator may round a request up, and resident memory is neither
//! stated nor bounded here. Other allocations on these paths keep their
//! existing behaviour and are not structural vectors: the `ExactSum` limbs
//! (growth bounded by the limb allotment, `T7.limbs`), the reads' scratch
//! and term lists, the re-formed pencil's element lists and its retained
//! stage workspace (`crate::workspace`) grow by std's infallible
//! allocation, as before, and an allocation failure there still aborts.
//! The whole-invocation memory ledger remains open.

use std::fmt;

/// A structural vector's reservation refused (module docs): typed, before
/// any partial result. Its refusal names are the kernel's `MODAL_RESOURCE_*`
/// names ([`SupportResourceError::code`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportResourceError {
    /// The element count of `structure`, or its byte size, overflows
    /// `usize` or exceeds `isize::MAX` bytes
    /// (`MODAL_RESOURCE_ARITHMETIC_OVERFLOW`). Nothing was requested.
    CapacityOverflow { structure: &'static str },
    /// The allocator refused the request of `bytes` for `structure`
    /// (`MODAL_RESOURCE_ALLOCATION_REFUSED`). Nothing was allocated.
    AllocationFailed { structure: &'static str, bytes: usize },
}

impl SupportResourceError {
    /// The refusal name: an overflow is a resource count's overflow, and an
    /// allocator refusal has its own name; neither is a numerical refusal or
    /// a budget comparison.
    pub fn code(&self) -> &'static str {
        match self {
            Self::CapacityOverflow { .. } => crate::dense::RESOURCE_ARITHMETIC_OVERFLOW,
            Self::AllocationFailed { .. } => crate::dense::RESOURCE_ALLOCATION_REFUSED,
        }
    }
}

impl fmt::Display for SupportResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CapacityOverflow { structure } => {
                write!(f, "{}: the capacity of `{structure}` overflows the addressable size", self.code())
            }
            Self::AllocationFailed { structure, bytes } => {
                write!(f, "{}: the allocator refused {bytes} B for `{structure}`", self.code())
            }
        }
    }
}

impl std::error::Error for SupportResourceError {}

/// `len × size_of::<T>()` when it is at most `isize::MAX` (the largest
/// allocation std admits), else `None`.
pub(crate) fn requested_bytes<T>(len: usize) -> Option<usize> {
    len.checked_mul(std::mem::size_of::<T>()).filter(|&bytes| bytes <= isize::MAX as usize)
}

/// An empty vector with capacity for exactly `len` elements (module docs:
/// one support step). Refuses `CapacityOverflow` before any request, and
/// `AllocationFailed` when the allocator refuses.
pub(crate) fn reserve<T>(len: usize, structure: &'static str) -> Result<Vec<T>, SupportResourceError> {
    let bytes = requested_bytes::<T>(len).ok_or(SupportResourceError::CapacityOverflow { structure })?;
    let mut out = Vec::new();
    #[cfg(test)]
    if bytes > 0 && failpoint::refuses() {
        return Err(SupportResourceError::AllocationFailed { structure, bytes });
    }
    out.try_reserve_exact(len).map_err(|_| SupportResourceError::AllocationFailed { structure, bytes })?;
    Ok(out)
}

/// `len` copies of `value` in a [`reserve`]d vector: one step and `len`
/// fill steps.
pub(crate) fn filled<T: Clone>(len: usize, value: T, structure: &'static str) -> Result<Vec<T>, SupportResourceError> {
    let mut out = reserve(len, structure)?;
    out.resize(len, value);
    Ok(out)
}

/// TEST-ONLY allocation failpoint: armed, the `k`-th nonzero reservation
/// on this thread (0-based) refuses as an allocator refusal would, before
/// any request. Never compiled into a product build.
#[cfg(test)]
pub(crate) mod failpoint {
    use std::cell::Cell;

    std::thread_local! {
        static ARMED: Cell<Option<usize>> = const { Cell::new(None) };
        static SEEN: Cell<usize> = const { Cell::new(0) };
    }

    /// Arms the failpoint at the `k`-th nonzero reservation from now
    /// (`None`: count only) and resets the count.
    pub(crate) fn arm(k: Option<usize>) {
        ARMED.with(|armed| armed.set(k));
        SEEN.with(|seen| seen.set(0));
    }

    /// Disarms it and returns the nonzero reservations seen since `arm`.
    pub(crate) fn disarm() -> usize {
        ARMED.with(|armed| armed.set(None));
        SEEN.with(Cell::get)
    }

    /// Arms the failpoint at each nonzero reservation of `run` in turn
    /// (`run` must be deterministic): each refuses, `refused` maps the
    /// consumer's error to that typed allocation refusal, and the run stops
    /// there (no later reservation is made). Past the last reservation,
    /// and disarmed again, `run` gives its unarmed result. Returns the
    /// number of nonzero reservations.
    pub(crate) fn each_reservation_refuses<T: PartialEq + std::fmt::Debug, E: std::fmt::Debug>(
        run: impl Fn() -> Result<T, E>,
        refused: impl Fn(&E) -> Option<super::SupportResourceError>,
    ) -> usize {
        arm(None);
        let free = run().unwrap_or_else(|e| panic!("the unarmed run refused: {e:?}"));
        let total = disarm();
        for k in 0..total {
            arm(Some(k));
            let found = run();
            let seen = disarm();
            match found {
                Err(e) => assert!(
                    matches!(refused(&e), Some(super::SupportResourceError::AllocationFailed { .. })),
                    "reservation {k} of {total}: {e:?}"
                ),
                Ok(_) => panic!("reservation {k} of {total} did not refuse"),
            }
            assert_eq!(seen, k + 1, "the refusal at reservation {k} stops the run");
        }
        arm(Some(total));
        assert_eq!(run().ok(), Some(free), "past the last reservation");
        assert_eq!(disarm(), total);
        total
    }

    pub(super) fn refuses() -> bool {
        let index = SEEN.with(|seen| {
            let index = seen.get();
            seen.set(index + 1);
            index
        });
        ARMED.with(Cell::get) == Some(index)
    }
}

/// The ascending complement of `excluded` in `0..n`: each `i < n` not in
/// `excluded`, ascending. `excluded` must be strictly ascending with every
/// element below `n`, as `dense::structurally_massless_dofs` returns it.
///
/// It is the former `(0..n).filter(|i| excluded.binary_search(i).is_err())`:
/// at index `i` the cursor has passed exactly the elements below `i`, so
/// `excluded[cursor]` is the least element `≥ i`, and `i` is excluded
/// exactly when it equals `i`.
///
/// Count: `n + 2` support steps (one per index, one for the call, one for
/// the reservation; the body is at most 9 operations). Capacity: exactly
/// `n − |excluded|` (8 B each), requested once.
pub(crate) fn ascending_complement(excluded: &[usize], n: usize) -> Result<Vec<usize>, SupportResourceError> {
    debug_assert!(excluded.windows(2).all(|w| w[0] < w[1]) && excluded.last().is_none_or(|&last| last < n));
    // `excluded` is at most n long (strictly ascending below n).
    let mut out = reserve(n - excluded.len(), "massive-DOF complement")?;
    let mut cursor = 0;
    for i in 0..n {
        if cursor < excluded.len() && excluded[cursor] == i {
            cursor += 1;
        } else {
            out.push(i);
        }
    }
    Ok(out)
}

/// A stable counting sort: the items of `sequence` (`len` of them) ordered
/// by `key(item)`, ascending, equal keys keeping their order in `sequence`.
/// Every key must be below `buckets`. With `len = 0` nothing is requested
/// and the result is empty, whatever `buckets` is.
///
/// Count: at `len = 0`, 1; otherwise 1 + `buckets` (the starts' reservation
/// and fill) + `len` (counting) + `buckets` (the exclusive prefix) + 1 +
/// `len` (the output's reservation and fill) + `len` (placing) + 1, so
/// `3·len + 2·buckets + 3` support steps; every body is at most 12
/// operations. Capacity: `buckets` starts (8 B each), freed on return, and
/// the result, exactly `len` (8 B each). No sum overflows: the running
/// total is at most `len`.
pub(crate) fn stable_counting_order(
    sequence: impl Iterator<Item = usize> + Clone,
    len: usize,
    buckets: usize,
    key: impl Fn(usize) -> usize,
) -> Result<Vec<usize>, SupportResourceError> {
    if len == 0 {
        return Ok(Vec::new());
    }
    let mut starts = filled(buckets, 0usize, "counting starts")?;
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
    let mut out = filled(len, 0usize, "counting order")?;
    for item in sequence {
        let bucket = key(item);
        out[starts[bucket]] = item;
        starts[bucket] += 1;
    }
    Ok(out)
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
/// - lists: `degree`'s reservation and fill 1 + n, the degrees q (the
///   half-edge count checked), `start`'s reservation, fill and prefix
///   1 + 2n, the ascending lists' reservation and fill 1 + 2q, the
///   cursor's reservation and copy 1 + n, placing q, the node order
///   (`stable_counting_order`, len and buckets n) 5n + 3, the lists'
///   reservation and fill 1 + 2q, the cursor's copy n, and placing n + 2q;
///   `mark`'s, `buffer`'s, `visited`'s and `order`'s reservations 4 and
///   `mark`'s and `visited`'s fills 2n: so `13n + 8q + 12`;
/// - traversals: per component of m nodes and h half-edges, at most m + 2
///   pseudo-peripheral traversals and the final one, each at most
///   `3m + h + 2` steps; the `least_by_degree` scans, at most
///   (m + 1)(m + 1); and the candidate loop's bodies, at most m. Summed
///   over the components: at most `(n + 3)(3n + H + 2C)` ≤
///   `(n + 3)(5n + 2q)` and `Σ (m + 1)(m + 2)` ≤ `(n + 1)(n + 2C)` ≤
///   `3n(n + 1)`;
/// - the seed loop n, the reversal n and the call 2.
///
/// So at most `15n + 8q + 14 + (n + 3)(5n + 2q) + 3n(n + 1)` steps.
///
/// **Capacity** (requested once each, module docs). `degree`, `start`, the
/// cursor, the node order and `mark`: n each, 8 B an element; `visited`:
/// n, 1 B each; the ascending lists and `neighbours`: H each, 8 B (the
/// ascending lists, the cursor and the node order and its starts are freed
/// before the traversals); `buffer` and the result: n each, 8 B, never
/// exceeded since a traversal discovers each node at most once. H = 2q is
/// counted with checked arithmetic. Stamps count traversals, at most 3n, below
/// `usize::MAX`.
pub(crate) fn reverse_cuthill_mckee_lower(
    n: usize,
    lower: impl Iterator<Item = (usize, usize)> + Clone,
) -> Result<Vec<usize>, SupportResourceError> {
    let mut degree = filled(n, 0usize, "RCM degrees")?;
    let mut half_edges = 0usize;
    for (row, col) in lower.clone() {
        debug_assert!(col < row && row < n);
        degree[row] += 1;
        degree[col] += 1;
        half_edges = half_edges.checked_add(2).ok_or(SupportResourceError::CapacityOverflow { structure: "RCM half-edges" })?;
    }
    let mut start = filled(n, 0usize, "RCM list starts")?;
    let mut total = 0;
    for (offset, &count) in start.iter_mut().zip(&degree) {
        *offset = total;
        total += count;
    }
    debug_assert_eq!(total, half_edges);
    let mut ascending = filled(half_edges, 0usize, "RCM ascending lists")?;
    let mut cursor = reserve(n, "RCM cursor")?;
    cursor.extend_from_slice(&start);
    for (row, col) in lower {
        ascending[cursor[row]] = col;
        cursor[row] += 1;
        ascending[cursor[col]] = row;
        cursor[col] += 1;
    }
    debug_assert!((0..n).all(|v| ascending[start[v]..start[v] + degree[v]].windows(2).all(|w| w[0] < w[1])));
    // Nodes by (degree, index): a node has at most n − 1 distinct
    // neighbours, so every degree is a bucket below n.
    let by_degree = stable_counting_order(0..n, n, n, |v| degree[v])?;
    let mut neighbours = filled(half_edges, 0usize, "RCM neighbour lists")?;
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
    let mark = filled(n, 0usize, "RCM marks")?;
    let buffer = reserve(n, "RCM traversal buffer")?;
    let mut traversal = Traversal { degree, start, neighbours, mark, stamp: 0, buffer };
    let mut visited = filled(n, false, "RCM visited")?;
    let mut order = reserve(n, "RCM order")?;
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
    Ok(order)
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
            let found = ascending_complement(excluded, *n).unwrap();
            assert_eq!(found, oracle(excluded, *n), "{excluded:?} in 0..{n}");
            assert_eq!(found.capacity(), Vec::<usize>::with_capacity(n - excluded.len()).capacity(), "the reserved capacity, never grown");
        }
        let mut mix = Mix(7);
        for _ in 0..200 {
            let n = mix.below(40);
            let excluded: Vec<usize> = (0..n).filter(|_| mix.below(3) == 0).collect();
            assert_eq!(ascending_complement(&excluded, n).unwrap(), oracle(&excluded, n));
        }
    }

    #[test]
    fn the_counting_order_is_a_stable_sort_by_key() {
        // Equal keys keep their order: the stable std sort is the oracle.
        let mut mix = Mix(11);
        for (len, buckets) in [(0, 0), (0, 3), (1, 1), (7, 1), (9, 2), (50, 7), (300, 40), (40, 300)] {
            let keys: Vec<usize> = (0..len).map(|_| mix.below(buckets.max(1))).collect();
            let found = stable_counting_order(0..len, len, buckets.max(usize::from(len > 0)), |i| keys[i]).unwrap();
            let mut oracle: Vec<usize> = (0..len).collect();
            oracle.sort_by_key(|&i| keys[i]);
            assert_eq!(found, oracle, "len {len}, buckets {buckets}");
            assert_eq!(found.capacity(), Vec::<usize>::with_capacity(len).capacity());
        }
        // Reversed and constant keys, over a permuted input sequence.
        let order: Vec<usize> = (0..20).rev().collect();
        let reversed = stable_counting_order(order.iter().copied(), 20, 20, |i| i).unwrap();
        assert_eq!(reversed, (0..20).collect::<Vec<_>>());
        let constant = stable_counting_order(order.iter().copied(), 20, 1, |_| 0).unwrap();
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
        let found = reverse_cuthill_mckee_lower(n, sorted.iter().copied()).unwrap();
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

    /// Review witness (1091d885): an empty order with a bucket count whose
    /// starts would overflow panicked. With nothing to order nothing is
    /// requested, so it is empty, as the former std sorts gave.
    #[test]
    fn an_empty_counting_order_requests_nothing_whatever_its_buckets() {
        failpoint::arm(Some(0));
        assert_eq!(stable_counting_order(0..0, 0, 1_152_921_504_606_846_976, |_| 0), Ok(vec![]));
        assert_eq!(stable_counting_order(0..0, 0, usize::MAX, |_| 0), Ok(vec![]));
        assert_eq!(ascending_complement(&[0, 1], 2), Ok(vec![]));
        assert_eq!(reverse_cuthill_mckee_lower(0, std::iter::empty()), Ok(vec![]));
        assert_eq!(failpoint::disarm(), 0, "nothing was requested");
    }

    /// The byte-size check at its inclusive boundary, and the refusals
    /// above and at it, with no oversized request made: above it nothing is
    /// requested; at it the size is legal and the failpoint stands for the
    /// allocator's refusal before the request.
    #[test]
    fn reservations_check_the_byte_size_at_the_inclusive_boundary() {
        let limit = isize::MAX as usize;
        let largest = limit / 8;
        assert_eq!(requested_bytes::<usize>(largest), Some(largest * 8));
        assert_eq!(requested_bytes::<usize>(largest + 1), None);
        assert_eq!(requested_bytes::<u8>(limit), Some(limit));
        assert_eq!(requested_bytes::<u8>(limit + 1), None);
        assert_eq!(requested_bytes::<usize>(usize::MAX), None, "the product itself overflows usize");
        assert_eq!(requested_bytes::<usize>(0), Some(0));
        assert_eq!(requested_bytes::<()>(usize::MAX), Some(0));

        let overflow = |structure| Err(SupportResourceError::CapacityOverflow { structure });
        failpoint::arm(None);
        assert_eq!(stable_counting_order(0..1, 1, 1 << 60, |_| 0), overflow("counting starts"));
        assert_eq!(stable_counting_order(0..1, 1, largest + 1, |_| 0), overflow("counting starts"));
        assert_eq!(stable_counting_order(0..1, 1, usize::MAX, |_| 0), overflow("counting starts"));
        assert_eq!(ascending_complement(&[], 1 << 61), overflow("massive-DOF complement"));
        assert_eq!(reverse_cuthill_mckee_lower(1 << 61, std::iter::empty()), overflow("RCM degrees"));
        assert_eq!(filled(largest + 1, 0usize, "x"), overflow("x"));
        assert_eq!(failpoint::disarm(), 0, "an overflow refuses before any request");

        failpoint::arm(Some(0));
        let found = stable_counting_order(0..1, 1, largest, |_| 0);
        assert_eq!(found, Err(SupportResourceError::AllocationFailed { structure: "counting starts", bytes: largest * 8 }));
        assert_eq!(failpoint::disarm(), 1);
        failpoint::arm(Some(0));
        assert_eq!(reserve::<u8>(limit, "y"), Err(SupportResourceError::AllocationFailed { structure: "y", bytes: limit }));
        assert_eq!(failpoint::disarm(), 1);

        // The codes and texts name the refusal, not a numerical failure.
        let overflowed = SupportResourceError::CapacityOverflow { structure: "counting starts" };
        let refused = SupportResourceError::AllocationFailed { structure: "counting starts", bytes: 16 };
        assert_eq!(overflowed.code(), "MODAL_RESOURCE_ARITHMETIC_OVERFLOW");
        assert_eq!(refused.code(), "MODAL_RESOURCE_ALLOCATION_REFUSED");
        assert!(overflowed.to_string().starts_with("MODAL_RESOURCE_ARITHMETIC_OVERFLOW: "));
        assert!(refused.to_string().starts_with("MODAL_RESOURCE_ALLOCATION_REFUSED: the allocator refused 16 B"));
    }

    /// Every nonzero reservation refuses cleanly and stops its function,
    /// and the number of reservations is each count's: one for the
    /// complement, two for a counting order, eleven for the order (nine
    /// vectors and the node order's two). A refused run leaves nothing
    /// behind: the next run is the unarmed one.
    #[test]
    fn each_reservation_refuses_cleanly_and_the_result_is_unchanged_after() {
        let typed = |e: &SupportResourceError| Some(*e);
        assert_eq!(failpoint::each_reservation_refuses(|| ascending_complement(&[1, 3], 6), typed), 1);
        let keys = [2, 0, 1, 2, 0];
        assert_eq!(failpoint::each_reservation_refuses(|| stable_counting_order(0..5, 5, 3, |i| keys[i]), typed), 2);
        let lower = [(1, 0), (2, 1), (4, 0), (4, 3)];
        assert_eq!(failpoint::each_reservation_refuses(|| reverse_cuthill_mckee_lower(5, lower.iter().copied()), typed), 11);
        // A run with no positions makes no zero-length request count.
        assert_eq!(failpoint::each_reservation_refuses(|| reverse_cuthill_mckee_lower(3, std::iter::empty()), typed), 9);
    }

    /// Growth controls: every structural result has exactly its requested
    /// capacity after a full run (no push past it).
    #[test]
    fn the_results_keep_their_requested_capacities() {
        let found = stable_counting_order(0..7, 7, 2, |i| i % 2).unwrap();
        assert_eq!((found.len(), found.capacity()), (7, 7));
        let found = ascending_complement(&[0], 4).unwrap();
        assert_eq!((found.len(), found.capacity()), (3, 3));
        let found = reverse_cuthill_mckee_lower(4, [(1, 0), (3, 2)].into_iter()).unwrap();
        assert_eq!((found.len(), found.capacity()), (4, 4));
    }
}
