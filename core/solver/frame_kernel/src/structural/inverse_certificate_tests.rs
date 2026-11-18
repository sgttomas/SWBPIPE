//! Tests of the inverse-norm certificate arithmetic against the independent
//! reference `validation/references/inverse_norm_certificate/` (`cases.txt`:
//! truth, availability and direction lines; `directed.txt`: directed
//! operations against exact arithmetic), and of its arithmetic outcomes and
//! input refusals.
use super::*;

fn blocks_of_view(view: &dyn RowView) -> Blocks {
    blocks(view)
}

const CASES: &str =
    include_str!("../../../../../validation/references/inverse_norm_certificate/cases.txt");
const DIRECTED: &str =
    include_str!("../../../../../validation/references/inverse_norm_certificate/directed.txt");

// ------------------------------------------------------------------ parsing

fn hex(s: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(s, 16).expect("hex binary64"))
}

fn rational(s: &str) -> (u128, u128) {
    let (p, q) = s.split_once('/').expect("p/q");
    (p.parse().unwrap(), q.parse().unwrap())
}

/// x ≥ p/q exactly, for finite x ≥ 0 and p, q > 0 small enough for u128.
fn at_least(x: f64, (p, q): (u128, u128)) -> bool {
    assert!(x.is_finite() && x >= 0.0);
    if x == 0.0 {
        return p == 0;
    }
    let bits = x.to_bits();
    let exp = ((bits >> 52) & 0x7ff) as i32;
    let (mant, e) = if exp == 0 {
        ((bits & ((1 << 52) - 1)) as u128, -1074)
    } else {
        (((bits & ((1 << 52) - 1)) | (1 << 52)) as u128, exp - 1075)
    };
    // x = mant·2^e; compare mant·q·2^e with p.
    if e >= 0 {
        mant.checked_mul(q).unwrap().checked_shl(e as u32).unwrap() >= p
    } else {
        let shift = (-e) as u32;
        assert!(
            shift < 127,
            "exact comparison out of the test helper's range"
        );
        mant.checked_mul(q).unwrap()
            >= p.checked_mul(1u128 << shift)
                .expect("exact comparison in range")
    }
}

#[derive(Debug, Default, Clone)]
struct Case {
    name: String,
    role: String,
    n: usize,
    order: Vec<usize>,
    first: Vec<usize>,
    q: f64,
    entries: Vec<(usize, usize, f64)>,
    norm: (u128, u128),
    block_norms: Vec<(usize, usize, (u128, u128))>,
    requires_not_verified: bool,
    blocks: Vec<(usize, usize, usize)>,
    attempts: Vec<(usize, String, String, Option<f64>)>,
    figures: Vec<(usize, String, String, f64)>,
    directions: Vec<(usize, String, String, String, f64)>,
    verdict: (String, Option<f64>),
}

fn cases() -> Vec<Case> {
    let mut out = Vec::new();
    let mut c = Case::default();
    for line in CASES.lines().filter(|l| !l.starts_with('#')) {
        let w: Vec<&str> = line.split_whitespace().collect();
        let opt = |s: &str| (s != "-").then(|| hex(s));
        match (w[0], w.get(1).copied()) {
            ("case", _) => {
                c = Case {
                    name: w[1].into(),
                    ..Case::default()
                }
            }
            ("role", _) => c.role = w[1].into(),
            ("about", _) => {}
            ("n", _) => c.n = w[1].parse().unwrap(),
            ("order", _) => c.order = w[1..].iter().map(|x| x.parse().unwrap()).collect(),
            ("first", _) => c.first = w[1..].iter().map(|x| x.parse().unwrap()).collect(),
            ("q", _) => c.q = hex(w[1]),
            ("entry", _) => {
                c.entries
                    .push((w[1].parse().unwrap(), w[2].parse().unwrap(), hex(w[3])))
            }
            ("truth", Some("norm")) => c.norm = rational(w[2]),
            ("truth", Some("block")) => {
                c.block_norms
                    .push((w[2].parse().unwrap(), w[3].parse().unwrap(), rational(w[4])))
            }
            ("truth", Some("requires")) => c.requires_not_verified = w[2] == "not_verified",
            ("avail", Some("block")) => c.blocks.push((
                w[2].parse().unwrap(),
                w[3].parse().unwrap(),
                w[4].parse().unwrap(),
            )),
            ("avail", Some("attempt")) => {
                c.attempts
                    .push((w[2].parse().unwrap(), w[3].into(), w[4].into(), opt(w[5])))
            }
            ("avail", Some("figure")) => {
                c.figures
                    .push((w[2].parse().unwrap(), w[3].into(), w[4].into(), hex(w[5])))
            }
            ("avail", Some("verdict")) => c.verdict = (w[2].into(), opt(w[3])),
            ("direction" | "operand", _) => c.directions.push((
                w[1].parse().unwrap(),
                w[2].into(),
                w[3].into(),
                w[4].into(),
                hex(w[5]),
            )),
            ("end", _) => out.push(std::mem::take(&mut c)),
            other => panic!("unknown line {other:?}"),
        }
    }
    assert_eq!(
        out.iter().map(|c| c.name.as_str()).collect::<Vec<_>>(),
        ["C1", "C2", "C3", "C4", "C5", "C6", "C7", "C8"]
    );
    out
}

fn case(name: &str) -> Case {
    cases().into_iter().find(|c| c.name == name).unwrap()
}

// --------------------------------------------------- the representations

/// The lower triangle as stored entries (explicit zeros kept).
struct Lower {
    n: usize,
    values: Vec<Vec<f64>>,
}

impl LowerTriangle for Lower {
    fn dimension(&self) -> usize {
        self.n
    }
    fn value(&self, row: usize, column: usize) -> f64 {
        self.values[row][column]
    }
}

struct Forms {
    lower: Lower,
    dense: Vec<Vec<f64>>,
    row_starts: Vec<usize>,
    columns: Vec<usize>,
    values: Vec<f64>,
    bits: Vec<u64>,
}

/// The same matrix as a lower-triangle reader, a dense prepared matrix and
/// compressed rows (both triangles, columns ascending, explicit zeros kept).
fn forms(n: usize, entries: &[(usize, usize, f64)]) -> Forms {
    let mut dense = vec![vec![0.0; n]; n];
    let mut stored = vec![Vec::new(); n];
    for &(r, c, v) in entries {
        assert!(c <= r);
        dense[r][c] = v;
        dense[c][r] = v;
        stored[r].push((c, v));
        if c != r {
            stored[c].push((r, v));
        }
    }
    let (mut row_starts, mut columns, mut values) = (vec![0], Vec::new(), Vec::new());
    for row in &mut stored {
        row.sort_by_key(|&(c, _)| c);
        for &(c, v) in row.iter() {
            columns.push(c);
            values.push(v);
        }
        row_starts.push(columns.len());
    }
    let bits = values.iter().map(|v: &f64| v.to_bits()).collect();
    let lower = Lower {
        n,
        values: dense.clone(),
    };
    Forms {
        lower,
        dense,
        row_starts,
        columns,
        values,
        bits,
    }
}

impl Forms {
    /// Runs through every representation with the supplied elimination.
    fn runs(&self, order: &[usize], first: &[usize], q: f64) -> Vec<Run> {
        let e = Elimination::Supplied { order, first };
        vec![
            dense_run(&self.dense, e, q),
            compressed_run(
                &self.row_starts,
                &self.columns,
                &self.values,
                order,
                first,
                q,
            ),
            compressed_run(&self.row_starts, &self.columns, &self.bits, order, first, q),
            lower_run(&self.lower, e, q),
        ]
    }
}

fn case_forms(c: &Case) -> Forms {
    forms(c.n, &c.entries)
}

// ---------------------------------------------------------- invariants

/// What every result must satisfy, whatever the input.
fn invariants(result: &InverseNormCertificate, q: f64) {
    match result {
        InverseNormCertificate::Verified(v) => {
            assert!(v.bound <= q && v.bound >= 0.0, "verified B within [0, Q]");
            assert_eq!(
                v.by_method.lemma_e_sigma1 + v.by_method.lemma_e_sigma2 + v.by_method.lemma_d,
                v.block_count
            );
        }
        InverseNormCertificate::NotVerified(nv) => {
            if nv.input.is_some() {
                assert!(nv.attempts.is_empty() && nv.block_count == 0 && nv.candidates.is_empty());
            } else {
                assert!(
                    nv.blocks_verified < nv.block_count,
                    "some block is not verified"
                );
            }
            for cand in &nv.candidates {
                assert!(cand.bound > q, "a candidate exceeds Q");
            }
        }
    }
}

fn attempt_name(a: &AttemptRecord) -> &'static str {
    match (a.method, a.shift) {
        (BoundMethod::LemmaEShiftedFactorization, Some(ShiftStep::Sigma1)) => "e1",
        (BoundMethod::LemmaEShiftedFactorization, Some(ShiftStep::Sigma2)) => "e2",
        (BoundMethod::LemmaDComparisonMatrix, None) => "d",
        other => panic!("unexpected attempt {other:?}"),
    }
}

fn reason_name(r: NotVerifiedReason) -> &'static str {
    match r {
        NotVerifiedReason::LemmaENonpositivePivot => "lemma_e_nonpositive_pivot",
        NotVerifiedReason::LemmaESigmaNonpositive => "lemma_e_sigma_nonpositive",
        NotVerifiedReason::LemmaDNonpositivePivot => "lemma_d_nonpositive_pivot",
        NotVerifiedReason::LemmaDContractionNotBelowOne => "lemma_d_contraction_not_below_one",
        NotVerifiedReason::ArithmeticNonfinite => "arithmetic_nonfinite",
        NotVerifiedReason::BoundExceedsExistingQuantity => "bound_exceeds_existing_quantity",
        NotVerifiedReason::InputUnusable => "input_unusable",
    }
}

fn figure_bits(figures: &[(&'static str, f64)]) -> Vec<(&'static str, u64)> {
    figures.iter().map(|&(n, v)| (n, v.to_bits())).collect()
}

/// Two runs agree bit for bit: verdict, B, blocks, attempts and figures.
fn same_run(a: &Run, b: &Run) {
    assert_eq!(a.result, b.result);
    if let (InverseNormCertificate::Verified(x), InverseNormCertificate::Verified(y)) =
        (&a.result, &b.result)
    {
        assert_eq!(x.bound.to_bits(), y.bound.to_bits());
    }
    assert_eq!(a.blocks.len(), b.blocks.len());
    for (x, y) in a.blocks.iter().zip(&b.blocks) {
        assert_eq!((x.least, x.size, x.k), (y.least, y.size, y.k));
        assert_eq!(x.attempts.len(), y.attempts.len());
        for (s, t) in x.attempts.iter().zip(&y.attempts) {
            assert_eq!(
                (s.method, s.shift, s.outcome),
                (t.method, t.shift, t.outcome)
            );
            assert_eq!(figure_bits(&s.figures), figure_bits(&t.figures));
        }
    }
}

// ------------------------------------------------------ the reference cases

/// Every case through every representation: identical runs, the package's
/// availability expectations bit for bit, every directed figure on its side of
/// the exact quantity, and the truth's consequences.
#[test]
fn reference_cases_verdicts_methods_figures_and_truth() {
    for c in cases() {
        let forms = case_forms(&c);
        let runs = forms.runs(&c.order, &c.first, c.q);
        for other in &runs[1..] {
            same_run(&runs[0], other);
        }
        let run = &runs[0];
        invariants(&run.result, c.q);
        // Availability: blocks, attempts, figures, verdict.
        let blocks: Vec<_> = run.blocks.iter().map(|b| (b.least, b.size, b.k)).collect();
        assert_eq!(blocks, c.blocks, "{}: blocks", c.name);
        let mut attempts = Vec::new();
        let mut figures = Vec::new();
        for b in &run.blocks {
            for a in &b.attempts {
                let (outcome, shown) = match a.outcome {
                    AttemptOutcome::Verified(x) => ("verified", Some(x)),
                    AttemptOutcome::NotVerified(r, cand) => (reason_name(r), cand),
                };
                attempts.push((
                    b.least,
                    attempt_name(a).to_string(),
                    outcome.to_string(),
                    shown.map(f64::to_bits),
                ));
                for &(name, v) in &a.figures {
                    figures.push((
                        b.least,
                        attempt_name(a).to_string(),
                        name.to_string(),
                        v.to_bits(),
                    ));
                }
            }
        }
        let expected: Vec<_> = c
            .attempts
            .iter()
            .map(|(l, k, o, v)| (*l, k.clone(), o.clone(), v.map(f64::to_bits)))
            .collect();
        assert_eq!(attempts, expected, "{}: attempts", c.name);
        let expected: Vec<_> = c
            .figures
            .iter()
            .map(|(l, k, n, v)| (*l, k.clone(), n.clone(), v.to_bits()))
            .collect();
        assert_eq!(figures, expected, "{}: figures", c.name);
        match (&run.result, c.verdict.0.as_str()) {
            (InverseNormCertificate::Verified(v), "verified") => {
                assert_eq!(
                    v.bound.to_bits(),
                    c.verdict.1.unwrap().to_bits(),
                    "{}: B",
                    c.name
                )
            }
            (InverseNormCertificate::NotVerified(_), "not_verified") => {}
            (r, e) => panic!("{}: {r:?} against {e}", c.name),
        }
        // Direction: each directed figure on its side of the exact quantity it
        // stands for (`direction`) and of the exact result of its own last
        // operation on that operation's operands (`operand`).
        assert!(!c.directions.is_empty());
        for (least, kind, name, side, edge) in &c.directions {
            let block = run.blocks.iter().find(|b| b.least == *least).unwrap();
            let attempt = block
                .attempts
                .iter()
                .find(|a| attempt_name(a) == kind)
                .unwrap();
            let &(_, v) = attempt.figures.iter().find(|(n, _)| n == name).unwrap();
            match side.as_str() {
                "ge" => assert!(
                    v >= *edge,
                    "{} {least} {kind} {name}: {v:e} below {edge:e}",
                    c.name
                ),
                "le" => assert!(
                    v <= *edge,
                    "{} {least} {kind} {name}: {v:e} above {edge:e}",
                    c.name
                ),
                _ => panic!("side"),
            }
        }
        // Truth: what any certificate must satisfy, independent of the emulation.
        match &run.result {
            InverseNormCertificate::Verified(v) => {
                assert!(
                    !c.requires_not_verified,
                    "{}: exact norm exceeds Q, yet verified",
                    c.name
                );
                assert!(
                    at_least(v.bound, c.norm),
                    "{}: B below the exact norm",
                    c.name
                );
            }
            InverseNormCertificate::NotVerified(nv) => {
                for cand in &nv.candidates {
                    let &(_, size, norm) =
                        c.block_norms.iter().find(|b| b.0 == cand.block).unwrap();
                    assert_eq!(size, cand.block_size);
                    assert!(
                        at_least(cand.bound, norm),
                        "{}: candidate below its block's exact norm",
                        c.name
                    );
                }
            }
        }
        for b in &run.blocks {
            let &(_, _, norm) = c.block_norms.iter().find(|x| x.0 == b.least).unwrap();
            for a in &b.attempts {
                if let AttemptOutcome::Verified(x) | AttemptOutcome::NotVerified(_, Some(x)) =
                    a.outcome
                {
                    assert!(
                        at_least(x, norm),
                        "{}: block bound below its exact norm",
                        c.name
                    );
                }
            }
        }
    }
}

fn verified(r: &InverseNormCertificate) -> &VerifiedBound {
    match r {
        InverseNormCertificate::Verified(v) => v,
        other => panic!("expected verified, got {other:?}"),
    }
}

fn not_verified(r: &InverseNormCertificate) -> &NotVerified {
    match r {
        InverseNormCertificate::NotVerified(v) => v,
        other => panic!("expected not verified, got {other:?}"),
    }
}

/// The success control (C1): truth ≤ B ≤ Q, by Lemma E at σ₁.
#[test]
fn control_success_truth_at_most_b_at_most_q() {
    let c = case("C1");
    assert_eq!(c.role, "control_success");
    let r = certify_dense(&case_forms(&c).dense, Elimination::Natural, c.q);
    let v = verified(&r);
    assert!(at_least(v.bound, c.norm) && v.bound <= c.q);
    assert_eq!(
        v.by_method,
        BlocksByMethod {
            lemma_e_sigma1: 1,
            lemma_e_sigma2: 0,
            lemma_d: 0
        }
    );
    assert_eq!((v.block_count, v.largest_block), (1, 12));
}

/// The maintained underestimate (C5, the witness's scaled free matrix with
/// its Q): exact norm 262144 > Q ≈ 1024, not verified through every
/// representation; D's candidate covers the truth and exceeds Q.
#[test]
fn control_underestimate_witness_not_verified() {
    let c = case("C5");
    assert_eq!(c.role, "control_underestimate");
    assert_eq!(c.norm, (262_144, 1));
    assert!(c.requires_not_verified && c.q == 1024.0_f64.next_up());
    for run in case_forms(&c).runs(&c.order, &c.first, c.q) {
        let nv = not_verified(&run.result);
        assert_eq!(
            (nv.block_count, nv.largest_block, nv.blocks_verified),
            (3, 4, 2)
        );
        assert_eq!(nv.candidates.len(), 1);
        assert_eq!(nv.candidates[0].method, BoundMethod::LemmaDComparisonMatrix);
        assert!(at_least(nv.candidates[0].bound, (262_144, 1)));
    }
}

/// C2: E verifies at σ₂ and not at σ₁; C3: E fails at both, D verifies;
/// C4: the premise holds exactly (norm 1024 ≤ Q) and the procedure does not
/// verify it.
#[test]
fn witness_family_methods() {
    let c2 = case("C2");
    let v = verified(&certify_dense(
        &case_forms(&c2).dense,
        Elimination::Natural,
        c2.q,
    ))
    .clone();
    assert_eq!(
        v.by_method,
        BlocksByMethod {
            lemma_e_sigma1: 2,
            lemma_e_sigma2: 1,
            lemma_d: 0
        }
    );
    let c3 = case("C3");
    let v = verified(&certify_dense(
        &case_forms(&c3).dense,
        Elimination::Natural,
        c3.q,
    ))
    .clone();
    assert_eq!(
        v.by_method,
        BlocksByMethod {
            lemma_e_sigma1: 2,
            lemma_e_sigma2: 0,
            lemma_d: 1
        }
    );
    assert!(at_least(v.bound, (512, 1)));
    let c4 = case("C4");
    assert!(!c4.requires_not_verified && c4.norm == (1024, 1));
    let r = certify_dense(&case_forms(&c4).dense, Elimination::Natural, c4.q);
    let nv = not_verified(&r);
    use NotVerifiedReason::*;
    assert_eq!(
        nv.attempts,
        vec![
            AttemptReasons {
                method: BoundMethod::LemmaEShiftedFactorization,
                shift: Some(ShiftStep::Sigma1),
                reasons: vec![(LemmaENonpositivePivot, 1)]
            },
            AttemptReasons {
                method: BoundMethod::LemmaEShiftedFactorization,
                shift: Some(ShiftStep::Sigma2),
                reasons: vec![(LemmaENonpositivePivot, 1)]
            },
            AttemptReasons {
                method: BoundMethod::LemmaDComparisonMatrix,
                shift: None,
                reasons: vec![(BoundExceedsExistingQuantity, 1)]
            },
        ]
    );
    assert_eq!(
        nv.reasons(),
        vec![
            LemmaENonpositivePivot,
            LemmaENonpositivePivot,
            BoundExceedsExistingQuantity
        ]
    );
}

/// The natural envelope a dense entry derives equals the one the package
/// states for C1 to C5 and C7's base, and gives the same run.
#[test]
fn natural_elimination_matches_supplied() {
    for c in cases()
        .into_iter()
        .filter(|c| ["C1", "C2", "C3", "C4", "C5"].contains(&c.name.as_str()))
    {
        let f = case_forms(&c);
        let env = envelope(&DenseView(&f.dense), Elimination::Natural).unwrap();
        assert_eq!(
            (env.order.clone(), env.first.clone()),
            (c.order.clone(), c.first.clone())
        );
        same_run(
            &dense_run(&f.dense, Elimination::Natural, c.q),
            &f.runs(&c.order, &c.first, c.q)[0],
        );
        same_run(
            &lower_run(&f.lower, Elimination::Natural, c.q),
            &f.runs(&c.order, &c.first, c.q)[0],
        );
    }
}

/// C6: interleaved blocks under a non-identity order and a full envelope:
/// blocks from values, per-block shifts, and every computed cross-block entry
/// an exact zero at each shift used.
#[test]
fn interleaved_blocks_factor_block_diagonally() {
    let c = case("C6");
    let f = case_forms(&c);
    assert_ne!(c.order, (0..c.n).collect::<Vec<_>>());
    let view = DenseView(&f.dense);
    let env = envelope(
        &view,
        Elimination::Supplied {
            order: &c.order,
            first: &c.first,
        },
    )
    .unwrap();
    let blocks = blocks_of_view(&view);
    assert_eq!(blocks.members, vec![vec![0, 2, 4, 6, 8], vec![1, 3, 5, 7]]);
    let run = dense_run(
        &f.dense,
        Elimination::Supplied {
            order: &c.order,
            first: &c.first,
        },
        c.q,
    );
    let sigmas: Vec<f64> = run
        .blocks
        .iter()
        .map(|b| b.attempts[0].figures[0].1)
        .collect();
    assert_ne!(
        sigmas[0], sigmas[1],
        "per-block shifts differ (k = 3 and 2)"
    );
    let mut cross = 0;
    for shifts in [sigmas.clone(), vec![0.0; 2]] {
        let fac = factor(&view, &env, &blocks, &shifts);
        assert_eq!(fac.failure, vec![None, None]);
        for i in 0..c.n {
            for j in env.first[i]..i {
                if blocks.of[env.order[i]] != blocks.of[env.order[j]] {
                    cross += 1;
                    assert_eq!(fac.get(i, j), 0.0, "cross-block entry ({i}, {j})");
                }
            }
        }
    }
    assert!(cross > 20);
    // Each block factored alone, in the same relative order and at the same
    // shift, gives the same pivots and passes bit for bit.
    let full = factor(&view, &env, &blocks, &sigmas);
    for (b, mem) in blocks.members.iter().enumerate() {
        let local: Vec<usize> = c
            .order
            .iter()
            .filter(|p| mem.contains(p))
            .map(|p| mem.binary_search(p).unwrap())
            .collect();
        let sub: Vec<Vec<f64>> = mem
            .iter()
            .map(|&r| mem.iter().map(|&s| f.dense[r][s]).collect())
            .collect();
        let first = vec![0; mem.len()];
        let sub_view = DenseView(&sub);
        let sub_env = envelope(
            &sub_view,
            Elimination::Supplied {
                order: &local,
                first: &first,
            },
        )
        .unwrap();
        let sub_blocks = blocks_of_view(&sub_view);
        let alone = factor(&sub_view, &sub_env, &sub_blocks, &[sigmas[b]]);
        let pivots_full: Vec<u64> = (0..c.n)
            .filter(|&i| blocks.of[env.order[i]] == b)
            .map(|i| full.get(i, i).to_bits())
            .collect();
        let pivots_alone: Vec<u64> = (0..mem.len()).map(|i| alone.get(i, i).to_bits()).collect();
        assert_eq!(pivots_full, pivots_alone, "block {b} pivots");
        let (p, q) = (
            passes(&full, &env, &blocks, b),
            passes(&alone, &sub_env, &sub_blocks, 0),
        );
        assert_eq!(
            [p.n_prime, p.nu, p.u, p.shifted_max].map(f64::to_bits),
            [q.n_prime, q.nu, q.u, q.shifted_max].map(f64::to_bits),
            "block {b} passes"
        );
    }
}

/// C7: an explicit stored zero (+0.0 and −0.0) joins nothing.
#[test]
fn explicit_stored_zero_is_not_an_edge() {
    let c = case("C7");
    assert!(c
        .entries
        .iter()
        .any(|&(r, c_, v)| (r, c_) == (3, 0) && v.to_bits() == (-0.0f64).to_bits()));
    assert!(c
        .entries
        .iter()
        .any(|&(r, c_, v)| (r, c_) == (2, 1) && v.to_bits() == 0));
    let f = case_forms(&c);
    for run in f.runs(&c.order, &c.first, c.q) {
        let v = verified(&run.result);
        assert_eq!((v.block_count, v.largest_block), (2, 2));
    }
    let b = blocks(&CompressedView {
        row_starts: &f.row_starts,
        columns: &f.columns,
        values: &f.values,
    });
    assert_eq!(b.members, vec![vec![0, 1], vec![2, 3]]);
}

/// The same matrix in all three site representations gives identical
/// results, bit for bit in B and in block structure (C1, C3, C5 and C6 under
/// its non-identity order; every case runs this in the reference test).
#[test]
fn three_representations_agree() {
    for name in ["C1", "C3", "C5", "C6"] {
        let c = case(name);
        let f = case_forms(&c);
        let runs = f.runs(&c.order, &c.first, c.q);
        for other in &runs[1..3] {
            same_run(&runs[0], other);
        }
        let r = &runs[0].result;
        assert_eq!(
            certify_dense(
                &f.dense,
                Elimination::Supplied {
                    order: &c.order,
                    first: &c.first
                },
                c.q
            ),
            *r
        );
        assert_eq!(
            certify_compressed(
                &f.row_starts,
                &f.columns,
                &f.values,
                &c.order,
                &c.first,
                c.q
            ),
            *r
        );
        assert_eq!(
            certify_compressed_bits(&f.row_starts, &f.columns, &f.bits, &c.order, &c.first, c.q),
            *r
        );
    }
}

// ------------------------------------------------- directions of rounding

/// Each directed operation against the exact result, including
/// RN(1/3) < 1/3 and RN(h/2) = 0 < h/2.
#[test]
fn directed_operations_against_exact_arithmetic() {
    let mut count = 0;
    for line in DIRECTED.lines().filter(|l| l.starts_with("op ")) {
        let w: Vec<&str> = line.split_whitespace().collect();
        let (a, b, got, edge) = (hex(w[2]), hex(w[3]), hex(w[4]), hex(w[6]));
        let r = match w[1] {
            "add_up" => add_up(a, b),
            "mul_up" => mul_up(a, b),
            "div_up" => div_up(a, b),
            "sub_dn" => sub_dn(a, b),
            op => panic!("{op}"),
        };
        assert_eq!(r.to_bits(), got.to_bits(), "{line}");
        match w[5] {
            "ge" => assert!(r >= edge, "{line}"),
            "le" => assert!(r <= edge, "{line}"),
            _ => panic!(),
        }
        count += 1;
    }
    assert_eq!(count, 18);
    assert!(1.0 / 3.0 < div_up(1.0, 3.0));
    assert_eq!(LEAST_SUBNORMAL * 0.5, 0.0);
    assert_eq!(mul_up(LEAST_SUBNORMAL, 0.5), LEAST_SUBNORMAL);
}

/// [[2, h], [h, 2]], h = 2⁻¹⁰⁷⁴: the loop erases the off-diagonal (l21 =
/// RN(h/2) = 0), so the exact residual at (2,1) is h ≠ 0 while the relative
/// term γ·|l21|·d1 is 0; only the absolute allowance 2⁻¹⁰⁷⁴(n + ν) covers it,
/// and the certificate's β does.
#[test]
fn erased_off_diagonal_needs_beta() {
    let line = DIRECTED.lines().find(|l| l.starts_with("erased ")).unwrap();
    let w: Vec<&str> = line.split_whitespace().collect();
    let h = hex(w[1]);
    assert_eq!(h, LEAST_SUBNORMAL);
    let matrix = vec![vec![2.0, h], vec![h, 2.0]];
    let view = DenseView(&matrix);
    let env = envelope(&view, Elimination::Natural).unwrap();
    let b = blocks(&view);
    assert_eq!(b.members.len(), 1, "h is nonzero: one block");
    let f = factor(&view, &env, &b, &[0.0]);
    let (l21, d1, d2) = (f.get(1, 0), f.get(0, 0), f.get(1, 1));
    assert_eq!(
        (l21.to_bits(), d2.to_bits()),
        (hex(w[3]).to_bits(), hex(w[5]).to_bits())
    );
    assert_eq!((l21, d1, d2), (0.0, 2.0, 2.0));
    // Exact: (L D Lᵀ)_21 = l21·d1 = 0 (a product with an exact zero), so the
    // residual a21 − 0 is h exactly.
    let residual = h - l21 * d1;
    assert_eq!(residual.to_bits(), hex(w[7]).to_bits());
    assert!(residual != 0.0);
    let relative = l21.abs() * d1;
    assert_eq!(relative, hex(w[9]));
    assert_eq!(relative, 0.0);
    let p = passes(&f, &env, &b, 0);
    let allowance = mul_up(LEAST_SUBNORMAL, add_up(2.0, p.nu));
    assert!(hex(w[11]) >= residual && allowance >= residual);
    let run = dense_run(&matrix, Elimination::Natural, 1024.0_f64.next_up());
    verified(&run.result);
    for a in &run.blocks[0].attempts {
        let beta = a.figures.iter().find(|(n, _)| *n == "beta").unwrap().1;
        assert!(beta >= residual, "beta covers the erased entry");
    }
}

// ------------------------------------------------- arithmetic outcomes (6.2)

fn run_dense(m: &[Vec<f64>], q: f64) -> Run {
    let run = dense_run(m, Elimination::Natural, q);
    invariants(&run.result, q);
    run
}

fn reasons_of(
    r: &InverseNormCertificate,
) -> Vec<(
    Option<ShiftStep>,
    BoundMethod,
    Vec<(NotVerifiedReason, usize)>,
)> {
    not_verified(r)
        .attempts
        .iter()
        .map(|a| (a.shift, a.method, a.reasons.clone()))
        .collect()
}

/// Entries near the top of the range: the shifted factorization's product
/// overflows, which is refused, not verified, at σ₁ (no σ₂ retry) and in D.
#[test]
fn overflow_is_arithmetic_nonfinite() {
    let (a, b, c) = (2f64.powi(-20), 2f64.powi(1000), 2f64.powi(1020));
    let m = vec![vec![a, b], vec![b, c]];
    let run = run_dense(&m, 2f64.powi(100));
    use NotVerifiedReason::*;
    assert_eq!(
        reasons_of(&run.result),
        vec![
            (
                Some(ShiftStep::Sigma1),
                BoundMethod::LemmaEShiftedFactorization,
                vec![(ArithmeticNonfinite, 1)]
            ),
            (
                None,
                BoundMethod::LemmaDComparisonMatrix,
                vec![(ArithmeticNonfinite, 1)]
            ),
        ]
    );
}

/// A shift smaller than the backward-error loss: pivots positive, σ′ ≤ 0, not
/// retried at σ₂; D then verifies.
#[test]
fn loss_exceeding_the_shift() {
    let m = vec![vec![1.0, 0.5], vec![0.5, 1.0]];
    let q = 2f64.powi(60);
    let run = run_dense(&m, q);
    let e1 = &run.blocks[0].attempts[0];
    assert_eq!(
        e1.outcome,
        AttemptOutcome::NotVerified(NotVerifiedReason::LemmaESigmaNonpositive, None)
    );
    let fig = |n: &str| e1.figures.iter().find(|(x, _)| *x == n).unwrap().1;
    assert!(fig("loss") > fig("sigma") && fig("sigma_prime") <= 0.0);
    assert_eq!(run.blocks[0].attempts.len(), 2, "no σ₂ retry");
    let v = verified(&run.result);
    assert_eq!(v.by_method.lemma_d, 1);
    // The exact inverse norm is 2 (inverse [[4, −2], [−2, 4]]/3).
    assert!(at_least(v.bound, (2, 1)));
}

/// A nearly singular pair, exact inverse norm 2⁵⁰ > Q: E's pivots fail at both
/// shifts and D's contraction t is not below one.
#[test]
fn lemma_d_contraction_not_below_one() {
    let e = 1.0 - 2f64.powi(-50);
    let m = vec![vec![1.0, e], vec![e, 1.0]];
    let run = run_dense(&m, 1024.0_f64.next_up());
    use NotVerifiedReason::*;
    assert_eq!(
        reasons_of(&run.result),
        vec![
            (
                Some(ShiftStep::Sigma1),
                BoundMethod::LemmaEShiftedFactorization,
                vec![(LemmaENonpositivePivot, 1)]
            ),
            (
                Some(ShiftStep::Sigma2),
                BoundMethod::LemmaEShiftedFactorization,
                vec![(LemmaENonpositivePivot, 1)]
            ),
            (
                None,
                BoundMethod::LemmaDComparisonMatrix,
                vec![(LemmaDContractionNotBelowOne, 1)]
            ),
        ]
    );
    let d = run.blocks[0].attempts.last().unwrap();
    assert!(d.figures.iter().find(|(n, _)| *n == "t").unwrap().1 >= 1.0);
}

/// A product that underflows to zero inside the loop: β > 0 enters σ′'s loss
/// and D's t (each sum is visibly changed by it).
#[test]
fn underflowing_product_beta_enters_loss_and_t() {
    let a = 2f64.powi(-600);
    let m = vec![vec![1.0, a], vec![a, 1.0]];
    let view = DenseView(&m);
    let env = envelope(&view, Elimination::Natural).unwrap();
    let b = blocks(&view);
    let f = factor(&view, &env, &b, &[0.0]);
    assert_eq!(f.get(1, 0), a);
    assert_eq!(a * a, 0.0, "the product w·l underflows to zero");
    assert_eq!(f.get(1, 1), 1.0);
    // Force both lemmas: Q large enough for E to fail on loss, D then runs.
    let run = run_dense(&m, 2f64.powi(60));
    let fig = |a: &AttemptRecord, n: &str| a.figures.iter().find(|(x, _)| *x == n).unwrap().1;
    let e1 = &run.blocks[0].attempts[0];
    let beta = fig(e1, "beta");
    assert!(beta > 0.0);
    let without = add_up(
        mul_up(fig(e1, "gamma"), fig(e1, "n_prime")),
        fig(e1, "delta"),
    );
    assert_eq!(fig(e1, "loss"), add_up(without, beta));
    assert!(fig(e1, "loss") > without);
    let d = &run.blocks[0].attempts[1];
    assert_eq!(d.method, BoundMethod::LemmaDComparisonMatrix);
    let inner = mul_up(fig(d, "gamma"), fig(d, "n_l"));
    assert!(fig(d, "beta") > 0.0 && add_up(inner, fig(d, "beta")) > inner);
    assert_eq!(
        fig(d, "t"),
        mul_up(fig(d, "u"), add_up(inner, fig(d, "beta")))
    );
    verified(&run.result);
}

// ------------------------------------------------------------ input refusal

fn unusable_with(r: &InverseNormCertificate) -> InputDefect {
    let nv = not_verified(r);
    assert_eq!(nv.reasons(), vec![NotVerifiedReason::InputUnusable]);
    assert!(nv.attempts.is_empty() && nv.candidates.is_empty());
    nv.input.unwrap()
}

#[test]
fn envelope_omitting_a_stored_nonzero_is_not_verified() {
    let c = case("C1");
    let f = case_forms(&c);
    let mut first = c.first.clone();
    first[5] = 5; // a_54 = −1 lies left of it
    for r in [
        certify_dense(
            &f.dense,
            Elimination::Supplied {
                order: &c.order,
                first: &first,
            },
            c.q,
        ),
        certify_compressed(&f.row_starts, &f.columns, &f.values, &c.order, &first, c.q),
        certify_compressed_bits(&f.row_starts, &f.columns, &f.bits, &c.order, &first, c.q),
        certify_lower(
            &f.lower,
            Elimination::Supplied {
                order: &c.order,
                first: &first,
            },
            c.q,
        ),
    ] {
        assert_eq!(unusable_with(&r), InputDefect::EnvelopeOmitsNonzero);
    }
    // An explicit zero outside the envelope is not an omission (C7's (3, 0)
    // with first[3] = 1 instead of 0 still holds every nonzero).
    let c7 = case("C7");
    let g = case_forms(&c7);
    let first = vec![0, 0, 1, 2];
    verified(&certify_compressed(
        &g.row_starts,
        &g.columns,
        &g.values,
        &c7.order,
        &first,
        c7.q,
    ));
}

/// A zero stored as +0.0 on one side and −0.0 on its transpose is a symmetric
/// matrix (the kernel's input validation compares values and can keep both
/// signs through scaling and storage): it is certified, with the result of
/// the same matrix stored with equal bits.
#[test]
fn mirrored_zeros_of_opposite_sign_are_symmetric() {
    let c7 = case("C7");
    let g = case_forms(&c7);
    let same = certify_compressed(
        &g.row_starts,
        &g.columns,
        &g.values,
        &c7.order,
        &c7.first,
        c7.q,
    );
    assert!(matches!(same, InverseNormCertificate::Verified(_)));
    // C7 stores -0.0 at (3, 0) and (0, 3). Flip each side alone to +0.0: (0, 3),
    // which the identity-order factor never loads, and (3, 0), the lower entry
    // its workspace does load (row 3's envelope starts at column 0). Each
    // mirrored pair of opposite-signed zeros gives the result of equal bits.
    assert_eq!(c7.order, [0, 1, 2, 3]);
    assert_eq!(c7.first[3], 0, "(3, 0) is inside the envelope the factor fills");
    for (row, column) in [(0, 3), (3, 0)] {
        let at = (g.row_starts[row]..g.row_starts[row + 1])
            .find(|&i| g.columns[i] == column)
            .unwrap();
        assert_eq!(g.values[at].to_bits(), (-0.0f64).to_bits(), "({row}, {column})");
        let mut values = g.values.clone();
        values[at] = 0.0;
        let bits: Vec<u64> = values.iter().map(|v| v.to_bits()).collect();
        assert_eq!(
            certify_compressed(&g.row_starts, &g.columns, &values, &c7.order, &c7.first, c7.q),
            same,
            "compressed, ({row}, {column})"
        );
        assert_eq!(
            certify_compressed_bits(&g.row_starts, &g.columns, &bits, &c7.order, &c7.first, c7.q),
            same,
            "bits, ({row}, {column})"
        );
        let mut dense = g.dense.clone();
        assert_eq!(dense[row][column].to_bits(), (-0.0f64).to_bits(), "({row}, {column})");
        dense[row][column] = 0.0;
        let supplied = Elimination::Supplied { order: &c7.order, first: &c7.first };
        assert_eq!(certify_dense(&dense, supplied, c7.q), same, "dense supplied, ({row}, {column})");
        assert_eq!(certify_dense(&dense, Elimination::Natural, c7.q), certify_dense(&g.dense, Elimination::Natural, c7.q), "dense natural, ({row}, {column})");
    }
}

#[test]
fn asymmetric_inputs_are_not_verified() {
    let c = case("C1");
    let f = case_forms(&c);
    // Compressed: drop the stored (0, 1) but keep (1, 0).
    let mut row_starts = f.row_starts.clone();
    let mut columns = f.columns.clone();
    let mut values = f.values.clone();
    let drop = (f.row_starts[0]..f.row_starts[1])
        .find(|&i| f.columns[i] == 1)
        .unwrap();
    columns.remove(drop);
    values.remove(drop);
    for s in row_starts.iter_mut().skip(1) {
        *s -= 1;
    }
    let bits: Vec<u64> = values.iter().map(|v| v.to_bits()).collect();
    assert_eq!(
        unusable_with(&certify_compressed(
            &row_starts,
            &columns,
            &values,
            &c.order,
            &c.first,
            c.q
        )),
        InputDefect::NotSymmetric
    );
    assert_eq!(
        unusable_with(&certify_compressed_bits(
            &row_starts,
            &columns,
            &bits,
            &c.order,
            &c.first,
            c.q
        )),
        InputDefect::NotSymmetric
    );
    // Compressed: different nonzero values.
    let mut values = f.values.clone();
    values[drop] = -1.0f64.next_up();
    assert_eq!(
        unusable_with(&certify_compressed(
            &f.row_starts,
            &f.columns,
            &values,
            &c.order,
            &c.first,
            c.q
        )),
        InputDefect::NotSymmetric
    );
    // Dense.
    let mut dense = f.dense.clone();
    dense[0][1] = -0.5;
    assert_eq!(
        unusable_with(&certify_dense(&dense, Elimination::Natural, c.q)),
        InputDefect::NotSymmetric
    );
}

#[test]
fn malformed_inputs_are_not_verified() {
    let c = case("C1");
    let f = case_forms(&c);
    let q = c.q;
    let nat = Elimination::Natural;
    // Order and envelope.
    let bad_order: Vec<usize> = (0..12).map(|i| i.min(10)).collect();
    let short: Vec<usize> = (0..11).collect();
    let mut beyond = c.first.clone();
    beyond[3] = 4;
    for (order, first, defect) in [
        (&bad_order, &c.first, InputDefect::OrderNotPermutation),
        (&short, &c.first, InputDefect::OrderNotPermutation),
        (&c.order, &beyond, InputDefect::FirstBeyondRow),
    ] {
        let e = Elimination::Supplied { order, first };
        assert_eq!(unusable_with(&certify_dense(&f.dense, e, q)), defect);
        assert_eq!(
            unusable_with(&certify_compressed(
                &f.row_starts,
                &f.columns,
                &f.values,
                order,
                first,
                q
            )),
            defect
        );
    }
    // Shapes.
    let mut ragged = f.dense.clone();
    ragged[2].pop();
    assert_eq!(
        unusable_with(&certify_dense(&ragged, nat, q)),
        InputDefect::Shape
    );
    assert_eq!(
        unusable_with(&certify_compressed(&[], &[], &[], &[], &[], q)),
        InputDefect::Shape
    );
    let mut unsorted = f.columns.clone();
    unsorted.swap(0, 1);
    assert_eq!(
        unusable_with(&certify_compressed(
            &f.row_starts,
            &unsorted,
            &f.values,
            &c.order,
            &c.first,
            q
        )),
        InputDefect::Shape
    );
    assert_eq!(
        unusable_with(&certify_compressed(
            &f.row_starts,
            &f.columns,
            &f.values[1..],
            &c.order,
            &c.first,
            q
        )),
        InputDefect::Shape
    );
    // Non-finite entries.
    let mut nan = f.dense.clone();
    nan[4][4] = f64::NAN;
    assert_eq!(
        unusable_with(&certify_dense(&nan, nat, q)),
        InputDefect::EntryNotFinite
    );
    let mut inf = f.bits.clone();
    inf[0] = f64::INFINITY.to_bits();
    assert_eq!(
        unusable_with(&certify_compressed_bits(
            &f.row_starts,
            &f.columns,
            &inf,
            &c.order,
            &c.first,
            q
        )),
        InputDefect::EntryNotFinite
    );
    // Q.
    for bad in [f64::NAN, f64::INFINITY, 0.0, -1.0, -0.0, f64::NEG_INFINITY] {
        assert_eq!(
            unusable_with(&certify_dense(&f.dense, nat, bad)),
            InputDefect::QuantityNotFinitePositive
        );
    }
}

/// n = 0 is verified with B = 0 when 0 ≤ Q, and never with B above Q.
#[test]
fn empty_system() {
    let empty: Vec<Vec<f64>> = Vec::new();
    for q in [0.0, 1.0, f64::INFINITY] {
        let v = verified(&certify_dense(&empty, Elimination::Natural, q)).clone();
        assert_eq!((v.bound, v.block_count, v.largest_block), (0.0, 0, 0));
        verified(&certify_compressed(&[0], &[], &[], &[], &[], q));
    }
    for q in [f64::NAN, -1.0] {
        assert_eq!(
            unusable_with(&certify_dense(&empty, Elimination::Natural, q)),
            InputDefect::QuantityNotFinitePositive
        );
    }
}

/// The hypothesis's integer check: the largest admissible n gives m = 2⁵².
#[test]
fn hypothesis_limit_is_two_to_the_52() {
    assert_eq!(HYPOTHESIS_LIMIT, 1u64 << 52);
    assert_eq!(UNIT_ROUNDOFF, 2f64.powi(-53));
    assert!(mul_up(HYPOTHESIS_LIMIT as f64, UNIT_ROUNDOFF) <= 0.5f64.next_up());
    assert_eq!(
        (
            ceil_sqrt(1),
            ceil_sqrt(4),
            ceil_sqrt(5),
            ceil_sqrt(12),
            ceil_sqrt(16),
            ceil_sqrt(17)
        ),
        (1, 2, 3, 4, 4, 5)
    );
}

/// The premise of the retained-memory pricing's certificate workspace owner
/// (IN-WS): each private type the workspace holds is no larger than the upper
/// the product's profile binds it to, which it states in standard types it
/// can name. A layout change that outgrows an upper fails here, before the
/// priced bound could fall short of the run's heap.
#[test]
fn the_workspace_layouts_are_within_the_pricing_uppers() {
    use std::mem::size_of;
    let vec = size_of::<Vec<u8>>();
    let word = size_of::<usize>();
    assert!(size_of::<BlockRecord>() <= 3 * word + vec);
    assert!(size_of::<AttemptRecord>() <= 2 * vec + size_of::<Option<f64>>());
    assert!(size_of::<Option<FactorFailure>>() <= word);
    assert!(size_of::<AttemptReasons>() <= vec + 2 * word);
    assert!(size_of::<(NotVerifiedReason, usize)>() <= 2 * word);
    assert!(size_of::<CandidateBound>() <= size_of::<f64>() + 4 * word);
    // The figures' element is named as is by the profile.
    assert_eq!(size_of::<(&'static str, f64)>(), 3 * word);
}

/// The record vectors' lengths and capacities the pricing's certificate
/// workspace (IN-WS) charges, on every reference case in every representation
/// and at a Q low enough that every block runs all three attempts: at most n
/// blocks; per block at most three attempts (capacity 4); per attempt at most
/// nine figures (capacity 14); on not verified, at most three attempt kinds,
/// at most four distinct reasons in each (capacity 4), and at most two
/// candidates per block (capacity pushcap(2 * blocks)). Block records are
/// exact (capacity = length). Each bound is a property of `run`'s and
/// `assemble`'s straight-line structure; this test fails if a change outgrows
/// one.
#[test]
fn the_record_vectors_stay_within_the_priced_lengths() {
    let mut cases_seen = 0;
    let mut three = 0;
    for c in cases() {
        let forms = case_forms(&c);
        for q in [c.q, 1.0e-3, f64::MIN_POSITIVE] {
            for run in forms.runs(&c.order, &c.first, q) {
                cases_seen += 1;
                assert!(run.blocks.len() <= c.n && run.blocks.capacity() == run.blocks.len(), "{}: blocks", c.name);
                let mut attempts = 0;
                for b in &run.blocks {
                    assert!(b.attempts.len() <= 3 && b.attempts.capacity() <= 4, "{}: attempts", c.name);
                    three += usize::from(b.attempts.len() == 3);
                    attempts += b.attempts.len();
                    for a in &b.attempts {
                        assert!(a.figures.len() <= 9 && a.figures.capacity() <= 14, "{}: figures", c.name);
                    }
                }
                if let InverseNormCertificate::NotVerified(nv) = &run.result {
                    assert!(nv.attempts.len() <= 3 && nv.attempts.capacity() <= 4, "{}: kinds", c.name);
                    for kind in &nv.attempts {
                        assert!(kind.reasons.len() <= 4 && kind.reasons.capacity() <= 4, "{}: reasons", c.name);
                    }
                    let pushcap = |h: usize| if h == 0 { 0 } else { h.next_power_of_two().max(4) };
                    assert!(nv.candidates.len() <= attempts, "{}: candidates", c.name);
                    assert!(nv.candidates.len() <= 2 * run.blocks.len(), "{}: candidates per block", c.name);
                    assert!(nv.candidates.capacity() <= pushcap(2 * run.blocks.len()), "{}: candidates capacity", c.name);
                }
            }
        }
    }
    assert!(cases_seen >= 8 * 3 * 3);
    assert!(three > 0, "some block runs all three attempts");
}
