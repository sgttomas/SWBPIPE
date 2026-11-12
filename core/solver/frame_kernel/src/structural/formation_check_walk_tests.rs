//! epsilon_E's pattern walk (`StoredPattern`) against the full walk it
//! replaces, kept verbatim below as `epsilon_e_reference`: the same f64 bits
//! and the same failure on hand-built systems; and the term grouping that
//! `load_closure` returns and `evaluate` reads, in source order.
use super::super::sparse::SparsePattern;
use super::super::StiffnessContribution;
use super::*;
use crate::{FrameNode, FrameSection};
use std::collections::{BTreeMap, BTreeSet};

/// `epsilon_e` before the pattern walk, verbatim but for its name: the
/// reference both walks are compared with, bit for bit.
fn epsilon_e_reference<A, P>(
    system: &StructuralSystem<'_>,
    source: &FormationSource,
    index: &[Option<usize>],
    exponents: &[i32],
    assembled: &A,
    projected: &P,
) -> Result<f64, Failure>
where
    A: Fn(usize, usize) -> f64,
    P: Fn(usize, usize) -> f64,
{
    let contributions = system
        .contributions
        .ok_or(Failure::Shape("stiffness contributions absent"))?;
    let n = system.free_dofs.len();
    let mut row_sums = vec![0.0; n];
    let mut col_sums = vec![0.0; n];
    let mut cursor = 0;
    let mut charge = |r: usize, c: usize, value: f64| {
        row_sums[r] = add_up(row_sums[r], value);
        col_sums[c] = add_up(col_sums[c], value);
    };
    intended_stiffness_pass(source, |ni, nj, k| {
        let map = element_dof_map(ni, nj);
        for r in 0..12 {
            for c in 0..12 {
                let entry = contributions
                    .get(cursor)
                    .ok_or(Failure::Shape("element contribution block missing"))?;
                cursor += 1;
                if entry.row != map[r] || entry.col != map[c] {
                    return Err(Failure::Shape("element contribution block identity"));
                }
                let (Some(fr), Some(fc)) = (index[map[r]], index[map[c]]) else {
                    continue;
                };
                let mut diff = ExactAccumulator::new();
                let split = k[r][c].split_binary64()?;
                for &x in split.terms() {
                    diff.add(x)?;
                }
                diff.add(-entry.value)?;
                let mut bound = exact_abs_scaled_up(&diff, exponents[fr] + exponents[fc])?;
                if split.truncated_below_min_subnormal() {
                    bound = add_up(
                        bound,
                        scale_up(f64::from_bits(1), exponents[fr] + exponents[fc]),
                    );
                }
                charge(fr, fc, bound);
            }
        }
        Ok(())
    })?;
    for &(dof, k) in &source.springs {
        let entry = contributions
            .get(cursor)
            .ok_or(Failure::Shape("spring contribution missing"))?;
        cursor += 1;
        if entry.row != dof || entry.col != dof || entry.value.to_bits() != k.to_bits() {
            return Err(Failure::Shape("spring contribution binding"));
        }
    }
    if cursor != contributions.len() {
        return Err(Failure::Shape("extra stiffness contribution"));
    }
    // Exact assembly error, one reusable row of accumulators rather than a matrix.
    let mut sums = vec![ExactAccumulator::new(); n];
    for (r, &i) in system.free_dofs.iter().enumerate() {
        for sum in &mut sums {
            *sum = ExactAccumulator::new();
        }
        for e in contributions {
            if e.row == i {
                if let Some(c) = index[e.col] {
                    sums[c].add(e.value)?;
                }
            }
        }
        for (c, &j) in system.free_dofs.iter().enumerate() {
            let stored = assembled(i, j);
            sums[c].add(-stored)?;
            let assembly = exact_abs_scaled_up(&sums[c], exponents[r] + exponents[c])?;
            let mut projection = ExactAccumulator::new();
            let scaled = Wide2::from_f64(stored)?
                .mul_pow2(i64::from(exponents[r]) + i64::from(exponents[c]))?;
            let split = scaled.split_binary64()?;
            for &x in split.terms() {
                projection.add(x)?;
            }
            projection.add(-projected(r, c))?;
            let mut change = exact_abs_scaled_up(&projection, 0)?;
            if split.truncated_below_min_subnormal() {
                change = add_up(change, f64::from_bits(1));
            }
            charge(r, c, add_up(assembly, change));
        }
    }
    Ok(row_sums.into_iter().chain(col_sums).fold(0.0, f64::max))
}

/// A hand-built system: frames along a chain, then springs (the contribution
/// order `epsilon_e` checks), free DOFs, scale exponents, the stored
/// assembled entries and the stored prepared entries. Anything absent reads
/// +0.0, as the sparse callbacks do.
#[derive(Clone)]
struct Case {
    nodes: usize,
    frames: Vec<FrameElement>,
    springs: Vec<(usize, f64)>,
    contributions: Vec<StiffnessContribution>,
    free: Vec<usize>,
    exponents: Vec<i32>,
    stored: BTreeMap<(usize, usize), f64>,
    prepared: BTreeMap<(usize, usize), f64>,
}

fn frame(i: usize, j: usize) -> FrameElement {
    FrameElement::new(
        FrameNode::new(i, [i as f64 * 4.0, 0.5 * (i % 2) as f64, 0.0]).unwrap(),
        FrameNode::new(j, [j as f64 * 4.0, 0.5 * (j % 2) as f64, 0.0]).unwrap(),
        FrameSection::new(8.0, 4.0, 2.0, 1.0, 1.0, 2.0).unwrap(),
        [0.0, 0.0, 1.0],
    )
    .unwrap()
}

impl Case {
    /// `nodes` nodes, a chain of frames over the first `chained`, node 0
    /// restrained, a spring on each listed DOF. Contributions are the frames'
    /// binary64 matrices, stored entries their coalesced sums and prepared
    /// entries the stored ones scaled exactly.
    fn chain(nodes: usize, chained: usize, springs: &[(usize, f64)]) -> Self {
        let frames: Vec<_> = (1..chained).map(|j| frame(j - 1, j)).collect();
        let mut contributions = Vec::new();
        for f in &frames {
            let k = f.global_stiffness().unwrap();
            let map = element_dof_map(f.node_i.index, f.node_j.index);
            for r in 0..12 {
                for c in 0..12 {
                    contributions.push(StiffnessContribution {
                        row: map[r],
                        col: map[c],
                        value: k[r][c],
                    });
                }
            }
        }
        for &(dof, k) in springs {
            contributions.push(StiffnessContribution {
                row: dof,
                col: dof,
                value: k,
            });
        }
        let free: Vec<usize> = (DOF_PER_NODE..nodes * DOF_PER_NODE).collect();
        // Exponents that differ by row (and stay well inside binary64).
        let exponents = (0..free.len()).map(|r| (r % 5) as i32 - 2).collect();
        let mut case = Self {
            nodes,
            frames,
            springs: springs.to_vec(),
            contributions,
            free,
            exponents,
            stored: BTreeMap::new(),
            prepared: BTreeMap::new(),
        };
        case.coalesce();
        case
    }
    /// Stored entries as the assembly sums them (list order, from +0.0), and
    /// the prepared entries from them.
    fn coalesce(&mut self) {
        self.stored.clear();
        for e in &self.contributions {
            *self.stored.entry((e.row, e.col)).or_insert(0.0) += e.value;
        }
        self.reproject();
    }
    /// Prepared entries: each free stored entry scaled exactly.
    fn reproject(&mut self) {
        self.prepared.clear();
        let position = self.positions();
        for (&(i, j), &v) in &self.stored {
            if let (Some(r), Some(c)) = (position[i], position[j]) {
                let scaled = v * 2.0_f64.powi(self.exponents[r] + self.exponents[c]);
                self.prepared.insert((r, c), scaled);
            }
        }
    }
    fn positions(&self) -> Vec<Option<usize>> {
        let mut index = vec![None; self.nodes * DOF_PER_NODE];
        for (r, &i) in self.free.iter().enumerate() {
            index[i] = Some(r);
        }
        index
    }
    fn dof(&self, node: usize, local: usize) -> usize {
        node * DOF_PER_NODE + local
    }
    fn source(&self) -> FormationSource<'static> {
        FormationSource {
            node_count: self.nodes,
            frames: self.frames.clone(),
            springs: self.springs.clone(),
            ..FormationSource::default()
        }
    }
    /// The symmetric closure of the stored positions (a mirror that is not
    /// stored reads +0.0: visiting it charges nothing).
    fn assembled_pattern(&self) -> SparsePattern {
        SparsePattern::from_positions(self.nodes * DOF_PER_NODE, self.stored.keys().copied())
            .unwrap()
    }
    fn prepared_rows(&self) -> (Vec<usize>, Vec<usize>) {
        let mut starts = vec![0];
        let mut columns = Vec::new();
        for r in 0..self.free.len() {
            columns.extend(self.prepared.range((r, 0)..(r + 1, 0)).map(|(&(_, c), _)| c));
            starts.push(columns.len());
        }
        (starts, columns)
    }
    /// (reference, dense walk, pattern walk), each as bits or failure.
    fn outcomes(&self) -> [String; 3] {
        let n = self.nodes * DOF_PER_NODE;
        let force = vec![0.0; n];
        let system = StructuralSystem {
            stiffness: &[],
            force: &force,
            free_dofs: &self.free,
            prescribed: &[],
            contributions: Some(&self.contributions),
            contribution_groups: None,
            symmetry: None,
        };
        let source = self.source();
        let index = self.positions();
        let assembled = |i: usize, j: usize| self.stored.get(&(i, j)).copied().unwrap_or(0.0);
        let projected = |r: usize, c: usize| self.prepared.get(&(r, c)).copied().unwrap_or(0.0);
        let pattern = self.assembled_pattern();
        let (starts, columns) = self.prepared_rows();
        let stored = StoredPattern {
            assembled: &pattern,
            prepared_starts: &starts,
            prepared_columns: &columns,
        };
        let e = &self.exponents;
        [
            shown(epsilon_e_reference(&system, &source, &index, e, &assembled, &projected)),
            shown(epsilon_e(&system, &source, &index, e, &assembled, &projected, None)),
            shown(epsilon_e(&system, &source, &index, e, &assembled, &projected, Some(stored))),
        ]
    }
    /// The reference outcome, once both walks are shown to reproduce it.
    #[track_caller]
    fn identical(&self) -> String {
        let [reference, dense, walk] = self.outcomes();
        assert_eq!(dense, reference, "dense walk");
        assert_eq!(walk, reference, "pattern walk");
        reference
    }
}

fn shown(result: Result<f64, Failure>) -> String {
    match result {
        Ok(v) => format!("ok {:#018x}", v.to_bits()),
        Err(e) => format!("err {e:?}"),
    }
}

#[test]
fn walk_is_the_full_walk_on_the_coalesced_chain() {
    let case = Case::chain(5, 5, &[(7, 3.0), (20, 0.5)]);
    let reference = case.identical();
    assert!(reference.starts_with("ok "), "{reference}");
}

/// A stored entry that is exactly +0.0 although its contributions' exact sum
/// is 1 (2^53, then 1, then -2^53, coalesced in binary64), a stored -0.0, and
/// contributions that cancel exactly on a stored entry.
#[test]
fn walk_matches_on_stored_zeros_and_exact_cancellation() {
    let mut case = Case::chain(4, 4, &[]);
    let shared = case.dof(1, 0);
    let at = |case: &Case, row: usize, col: usize| -> Vec<usize> {
        (0..case.contributions.len())
            .filter(|&k| case.contributions[k].row == row && case.contributions[k].col == col)
            .collect()
    };
    // Frames 0-1 and 1-2 both contribute at (shared, shared); a spring follows.
    let diagonal = at(&case, shared, shared);
    assert_eq!(diagonal.len(), 2);
    case.contributions[diagonal[0]].value = 2.0_f64.powi(53);
    case.contributions[diagonal[1]].value = 1.0;
    case.springs.push((shared, -(2.0_f64.powi(53))));
    case.contributions.push(StiffnessContribution {
        row: shared,
        col: shared,
        value: -(2.0_f64.powi(53)),
    });
    // Exact cancellation: v and -v at (shared, shared + 1).
    let next = shared + 1;
    let pair = at(&case, shared, next);
    assert_eq!(pair.len(), 2);
    case.contributions[pair[0]].value = 0.375;
    case.contributions[pair[1]].value = -0.375;
    case.coalesce();
    assert_eq!(case.stored[&(shared, shared)].to_bits(), 0.0_f64.to_bits());
    assert_eq!(case.stored[&(shared, next)].to_bits(), 0.0_f64.to_bits());
    let zero_free = case.identical();
    assert!(zero_free.starts_with("ok "), "{zero_free}");
    // A stored -0.0 where the contributions sum to something else.
    case.stored.insert((case.dof(2, 1), case.dof(2, 5)), -0.0);
    case.reproject();
    assert!(case.identical().starts_with("ok "));
}

/// Contributions at a pair that neither stored pattern holds. The product
/// refuses this when it prepares (`sparse_contribution_sums`: "contribution
/// outside pattern"); the walk takes the pair anyway, and its charge moves
/// the bound.
#[test]
fn walk_visits_contributions_off_the_stored_patterns() {
    let mut case = Case::chain(4, 4, &[]);
    let base = case.identical();
    let (i, j) = (case.dof(2, 0), case.dof(3, 0));
    for key in [(i, j), (j, i)] {
        assert!(case.contributions.iter().any(|e| (e.row, e.col) == key));
        case.stored.remove(&key);
    }
    case.reproject();
    let pattern = case.assembled_pattern();
    assert!(pattern.find(i, j).is_none() && pattern.find(j, i).is_none());
    let reference = case.identical();
    assert_ne!(reference, base, "the off-pattern pair must move the bound");
}

/// A prepared entry with no stored assembled entry and no contribution. The
/// product cannot hold one (prepared row r is stored row free[r] restricted
/// to free columns: `prepare_sparse_bound`); the walk takes it anyway.
#[test]
fn walk_visits_prepared_entries_without_an_assembled_entry() {
    let mut case = Case::chain(5, 3, &[]);
    let base = case.identical();
    let position = case.positions();
    // Nodes 3 and 4 have no element: no contribution and no stored entry.
    let (i, j) = (case.dof(3, 2), case.dof(4, 4));
    assert!(!case.stored.contains_key(&(i, j)));
    case.prepared.insert((position[i].unwrap(), position[j].unwrap()), 1.0e3);
    let reference = case.identical();
    assert_ne!(reference, base, "the prepared-only entry must move the bound");
}

/// Upward-rounded sums depend on the order of their charges: one row charged
/// 1, 2^-53 and 2^-53 in ascending columns sums to 1 + 3·2^-52, in
/// descending columns to 1 + 2·2^-52. The bound is that row's sum.
#[test]
fn walk_keeps_ascending_columns_where_the_rounded_sums_depend_on_order() {
    let mut case = Case::chain(5, 2, &[]);
    case.exponents.iter_mut().for_each(|e| *e = 0);
    let i = case.dof(3, 0);
    let (c1, c2, c3) = (case.dof(3, 1), case.dof(4, 2), case.dof(4, 3));
    let tiny = 2.0_f64.powi(-53);
    // assembly = next_up(|stored|): exactly 1, 2^-53 and 2^-53; no change.
    case.stored.insert((i, c1), 1.0_f64.next_down());
    case.stored.insert((i, c2), tiny.next_down());
    case.stored.insert((i, c3), tiny.next_down());
    case.reproject();
    let up = |values: &[f64]| values.iter().fold(0.0, |s, &v| add_up(s, v));
    let ascending = up(&[1.0, tiny, tiny]);
    let descending = up(&[tiny, tiny, 1.0]);
    assert_eq!(ascending, 1.0 + 3.0 * f64::EPSILON);
    assert_eq!(descending, 1.0 + 2.0 * f64::EPSILON);
    let reference = case.identical();
    assert_eq!(reference, format!("ok {:#018x}", ascending.to_bits()));
}

/// Non-finite stored, prepared and contributed values: the same failure from
/// the same pair. Two different failures in one row fix the column order (the
/// NaN pair's column is the larger); a later row's failure does not displace
/// an earlier row's; a non-finite contribution at a restrained DOF is skipped.
#[test]
fn walk_fails_where_and_as_the_full_walk_does() {
    let mut base = Case::chain(5, 3, &[]);
    base.exponents.iter_mut().for_each(|e| *e = 1);
    base.reproject();
    let i = base.dof(3, 0);
    let (low, high) = (base.dof(3, 1), base.dof(4, 0));
    let mut nan = base.clone();
    nan.stored.insert((i, high), f64::NAN);
    let nan_failure = nan.identical();
    let mut huge = base.clone();
    huge.stored.insert((i, low), f64::MAX);
    let huge_failure = huge.identical();
    assert!(nan_failure.starts_with("err ") && huge_failure.starts_with("err "));
    assert_ne!(nan_failure, huge_failure);
    let mut both = huge.clone();
    both.stored.insert((i, high), f64::NAN);
    assert_eq!(both.identical(), huge_failure);
    let mut rows = huge.clone();
    rows.stored.insert((base.dof(4, 1), base.dof(3, 0)), f64::NAN);
    assert_eq!(rows.identical(), huge_failure);
    let mut prepared = base.clone();
    let position = base.positions();
    prepared.prepared.insert(
        (position[base.dof(4, 3)].unwrap(), position[i].unwrap()),
        f64::INFINITY,
    );
    assert!(prepared.identical().starts_with("err "));
    // In the free block the formation pass fails first; at the restrained
    // node 0 both passes skip the contribution.
    let mut free = base.clone();
    let k = free
        .contributions
        .iter()
        .position(|e| e.row == base.dof(1, 2) && e.col == base.dof(2, 2))
        .unwrap();
    free.contributions[k].value = f64::NAN;
    assert!(free.identical().starts_with("err "));
    let mut restrained = base.clone();
    let k = restrained
        .contributions
        .iter()
        .position(|e| e.row == 0 && e.col == 0)
        .unwrap();
    restrained.contributions[k].value = f64::INFINITY;
    assert!(restrained.identical().starts_with("ok "));
}

/// Forty nodes in a chain, with springs and off-chain entries: fewer than one
/// free pair in ten is visited, and the bound is the full walk's.
#[test]
fn walk_matches_on_a_system_where_most_pairs_are_unvisited() {
    let springs: Vec<(usize, f64)> = (0..40).map(|k| (6 + (7 * k) % 234, 0.25 + k as f64)).collect();
    let mut case = Case::chain(40, 40, &springs);
    let position = case.positions();
    case.stored.insert((case.dof(10, 0), case.dof(30, 0)), 2.5);
    case.prepared.insert(
        (position[case.dof(5, 1)].unwrap(), position[case.dof(25, 4)].unwrap()),
        0.5,
    );
    let reference = case.identical();
    assert!(reference.starts_with("ok "), "{reference}");
    let n = case.free.len();
    let pattern = case.assembled_pattern();
    let mut visited = 0;
    for (r, &i) in case.free.iter().enumerate() {
        let mut row = BTreeSet::new();
        row.extend(case.contributions.iter().filter(|e| e.row == i).filter_map(|e| position[e.col]));
        row.extend(pattern.row(i).iter().filter_map(|&j| position[j]));
        row.extend(case.prepared.range((r, 0)..(r + 1, 0)).map(|(&(_, c), _)| c));
        visited += row.len();
    }
    assert!(visited * 10 < n * n, "{visited} of {}", n * n);
}

/// A pattern whose shape does not fit the system walks every pair.
#[test]
fn walk_with_a_misfit_pattern_is_the_full_walk() {
    let mut case = Case::chain(4, 3, &[]);
    let position = case.positions();
    case.prepared.insert((position[case.dof(3, 0)].unwrap(), position[case.dof(3, 1)].unwrap()), 7.0);
    let n = case.nodes * DOF_PER_NODE;
    let force = vec![0.0; n];
    let system = StructuralSystem {
        stiffness: &[],
        force: &force,
        free_dofs: &case.free,
        prescribed: &[],
        contributions: Some(&case.contributions),
        contribution_groups: None,
        symmetry: None,
    };
    let source = case.source();
    let assembled = |i: usize, j: usize| case.stored.get(&(i, j)).copied().unwrap_or(0.0);
    let projected = |r: usize, c: usize| case.prepared.get(&(r, c)).copied().unwrap_or(0.0);
    let e = &case.exponents;
    let reference = shown(epsilon_e_reference(&system, &source, &position, e, &assembled, &projected));
    let empty = SparsePattern::from_positions(n, []).unwrap();
    let wrong = SparsePattern::from_positions(n - 1, []).unwrap();
    let starts = vec![0; case.free.len() + 1];
    // One prepared row short, and an assembled pattern of the wrong order.
    for (assembled_pattern, prepared_starts) in [(&empty, &starts[1..]), (&wrong, &starts[..])] {
        let misfit = StoredPattern {
            assembled: assembled_pattern,
            prepared_starts,
            prepared_columns: &[],
        };
        let walked = epsilon_e(&system, &source, &position, e, &assembled, &projected, Some(misfit));
        assert_eq!(shown(walked), reference);
    }
}

// ------------------------------------------------------------ term grouping

fn nodal(term: usize, dof: usize, ordinal: usize, magnitude: f64, factor: f64) -> LoadPrimitive {
    LoadPrimitive {
        term_index: term,
        identity: LoadSummandIdentity {
            family: "nodal".into(),
            source: format!("term {term}"),
            ordinal,
        },
        local_row: dof % 6,
        formula: LoadFormula::NodalProduct {
            global_dof: dof,
            magnitude,
            factor,
        },
        force_scale_exponent: 0,
    }
}

/// One formed term per (dof, primitives), its manifest listing them.
fn ledger_of(terms: &[(usize, Vec<LoadPrimitive>)]) -> crate::load_ledger::AssembledForce {
    let mut ledger = crate::load_ledger::LoadLedger::new();
    for (t, (dof, primitives)) in terms.iter().enumerate() {
        ledger.push_formed(
            format!("term {t}"),
            *dof,
            1.0,
            crate::load_ledger::Formation::CannotBound,
            0.0,
            false,
        );
        let manifest = primitives.iter().map(LoadPrimitive::closure_entry).collect();
        assert!(ledger.set_closure_manifest(t, manifest));
    }
    ledger.finish(DOF_PER_NODE).unwrap()
}

#[test]
fn load_closure_groups_each_terms_primitives_in_source_order() {
    let t0: Vec<_> = (0..3).map(|o| nodal(0, 0, o, 1.0 + o as f64, 2.0)).collect();
    let t1: Vec<_> = (0..2).map(|o| nodal(1, 1, o, 3.0, 1.0 + o as f64)).collect();
    let t2: Vec<_> = (0..3).map(|o| nodal(2, 2, o, 0.5, 0.25 + o as f64)).collect();
    let force = ledger_of(&[(0, t0.clone()), (1, t1.clone()), (2, t2.clone())]);
    let orders = [
        [&t1[1], &t0[2], &t2[0], &t0[0], &t2[2], &t1[0], &t0[1], &t2[1]],
        [&t2[1], &t0[1], &t1[0], &t2[2], &t0[0], &t2[0], &t0[2], &t1[1]],
    ];
    for order in orders {
        let loads: Vec<LoadPrimitive> = order.into_iter().cloned().collect();
        let source = FormationSource {
            node_count: 1,
            loads: loads.clone().into(),
            ..FormationSource::default()
        };
        let by_term = load_closure(&source, &force).unwrap();
        for term in 0..3 {
            let expected: Vec<usize> =
                (0..loads.len()).filter(|&k| loads[k].term_index == term).collect();
            let grouped = group_of(&by_term, |k| loads[k].term_index, term);
            assert_eq!(grouped, &expected[..], "term {term}");
        }
    }
    // The first loop's failures come first and unchanged.
    let mut stray = t0.clone();
    stray[1].term_index = 7;
    let source = FormationSource {
        node_count: 1,
        loads: stray.into(),
        ..FormationSource::default()
    };
    assert!(matches!(
        load_closure(&source, &force),
        Err(Failure::Closure(d)) if d == "extra or mismatched-term primitive"
    ));
}

/// `evaluate` re-forms a term's primitives in source order: of two that fail
/// differently, the first in the source is the one reported.
#[test]
fn evaluate_reforms_a_terms_primitives_in_source_order() {
    let nan = nodal(0, 0, 0, f64::NAN, 1.0);
    let overflow = nodal(0, 0, 1, f64::MAX, f64::MAX);
    let other = nodal(1, 1, 0, 1.0, 1.0);
    let failure = |p: &LoadPrimitive| {
        format!("load closure: term 0: reformation {:?}", load_reformation(p).unwrap_err())
    };
    assert_ne!(failure(&nan), failure(&overflow));
    let force = ledger_of(&[(0, vec![nan.clone(), overflow.clone()]), (1, vec![other.clone()])]);
    let values = vec![0.0; DOF_PER_NODE];
    let free: Vec<usize> = (0..DOF_PER_NODE).collect();
    let system = StructuralSystem {
        stiffness: &[],
        force: &values,
        free_dofs: &free,
        prescribed: &[],
        contributions: None,
        contribution_groups: None,
        symmetry: None,
    };
    for (loads, first) in [
        (vec![nan.clone(), other.clone(), overflow.clone()], &nan),
        (vec![overflow.clone(), other.clone(), nan.clone()], &overflow),
    ] {
        let source = FormationSource {
            node_count: 1,
            loads: loads.into(),
            ..FormationSource::default()
        };
        let (finding, bound) = check(
            &system,
            &source,
            Some(&force),
            &[0; DOF_PER_NODE],
            &[0.0; DOF_PER_NODE],
            (1.0, 1.0, 1.0),
            &|_, _| 0.0,
            &|_, _| 0.0,
            &|rhs: &[f64]| Ok(rhs.to_vec()),
        );
        assert!(bound.is_none());
        assert_eq!(
            finding.unwrap().reason,
            FormationCheckReason::FormationCheckUnavailable { detail: failure(first) }
        );
    }
}
