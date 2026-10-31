//! The inertia-risk suite (design §3.3, "the unpivoted-LDLᵀ inertia risk is
//! a first-class test target"; §3.4 check 4) and the finite-mode count
//! (§3.2 item 5).
//!
//! Sources:
//! - `tests/fixtures/sturm_pencils.txt`, written by `sturm_pencils.py` beside
//!   it from two frozen references, read-only: the inertia witnesses
//!   (`validation/references/f1_modal/continuum/inertia_witnesses.json`) and
//!   formed pencils of nine class (a) cases with their published exact Sturm
//!   counts (`validation/references/f1_modal/mass_discrete/`).
//! - The design's review witnesses of rounds 3, 4 and 6, built here from
//!   their stated construction.
//!
//! The rule asserted everywhere: a count taken under the premises equals the
//! exact count. Shifts that binary64 miscounts must be refused, never
//! counted. On well-separated gaps of the class (a) cases no shift may be
//! refused with the test-local constants below.
//!
//! The neighbouring eigenvalues are the crate's own dense eigenpairs, with a
//! test-local stand-in for check 2's error estimate e_λ, evaluated on the
//! formed pencil (the re-formed pencil of check 1 is not built yet):
//! e_λ = ‖D r‖₂·‖φ̃‖₂ / (λ·φᵀMφ), r = Kφ − λMφ in double-double, which is
//! η·κ_λ of check 2.

use open_pipe_stress_modal_eigen::clusters::cluster_ids;
use open_pipe_stress_modal_eigen::dd::{Dd, DdAccumulator};
use open_pipe_stress_modal_eigen::dense::{
    established_finite_mode_count, solve_dense_pencil, DenseEigenSolution, DenseOptions,
};
use open_pipe_stress_modal_eigen::sturm::{
    count_above_spectrum, count_at_shift, count_in_gap, Gap, GapLower, Neighbour, ShiftDecision,
    ShiftPencil, SturmError, SturmOutcome, SturmParameters, SturmRefusal,
};
use open_pipe_stress_modal_eigen::SymmetricMatrix;

/// Test-local algorithm parameters, measured by this suite (README, "Sturm
/// counts"). The production values are recorded under DEC-026 (design §9,
/// O-6), not here.
///
/// Measured with κ_c = κ_s = 1 and no growth limit, over every witness and
/// class (a) shift and every placement: the 34 factored shifts whose
/// negative-pivot count was wrong had e_b·ν ≥ 35.8 and separation ratio
/// ≤ 6.2·10⁻³; the 352 well-separated shifts had e_b·ν ≤ 2.13·10⁻⁴,
/// separation ratio ≥ 5.65·10³ and g ≤ 3.98·10¹³, every count right. So
/// κ_c = 10³ and κ_s = 10² sit between the two populations, and g_max = 10¹⁵
/// lies above every well-separated factorization: on this suite the growth
/// factor does not separate miscounts (they had g ≤ 7.7·10⁸).
fn parameters() -> SturmParameters {
    SturmParameters {
        conditioning_multiple: 1e3,
        separation_multiple: 1e2,
        growth_limit: 1e15,
        max_candidates: 7,
        gap_log_positions: vec![0.25, 0.75, 0.125, 0.375, 0.625, 0.875],
        whole_spectrum_offsets: vec![1.0, 0.5, 0.25],
    }
}

/// Test-local resolution-floor parameters c_μ and κ_μ (design §3.2 item 5);
/// the production values are recorded under DEC-026, not here. With
/// c_μ = 1, the review-round-3 artifact lies about 2× below δ_μ and the
/// smallest retained μ of every admitted fixture case at least 7.4·10² above
/// it (E-RIGID-SCALE-1e6), so κ_μ = 10 sits between them (README).
const RESOLUTION_FLOOR_CONSTANT: f64 = 1.0;
const RESOLUTION_FLOOR_MULTIPLE: f64 = 10.0;

/// The smallest retained μ over κ_μ·δ_μ: how far the resolution floor is
/// below the retained spectrum (+∞ with a zero floor or no mode).
fn floor_margin(solution: &DenseEigenSolution) -> f64 {
    let smallest = solution.mu.iter().copied().fold(f64::INFINITY, f64::min);
    smallest / (RESOLUTION_FLOOR_MULTIPLE * solution.resolution_floor)
}

/// Test-local dense options and thresholds.
fn dense_options() -> DenseOptions {
    DenseOptions {
        infinite_mu_relative_threshold: 1e-12,
        mass_pivot_relative_threshold: MASS_PIVOT_RELATIVE_THRESHOLD,
        resolution_floor_constant: RESOLUTION_FLOOR_CONSTANT,
        resolution_floor_multiple: RESOLUTION_FLOOR_MULTIPLE,
        max_ql_iterations_per_eigenvalue: 30,
    }
}
const MASS_PIVOT_RELATIVE_THRESHOLD: f64 = 1e-12;
const CLUSTER_DELTA: f64 = 1e-8;

type Rows = Vec<Vec<f64>>;

#[derive(Debug, Clone)]
struct FixtureShift {
    kind: String,
    sigma: f64,
    below: usize,
    equal: usize,
}

#[derive(Debug, Clone)]
struct Pencil {
    id: String,
    is_witness: bool,
    n_finite: usize,
    k: Rows,
    m: Rows,
    shifts: Vec<FixtureShift>,
}

fn fixture() -> Vec<Pencil> {
    let text = include_str!("fixtures/sturm_pencils.txt");
    let mut pencils = Vec::new();
    let mut current: Option<Pencil> = None;
    for line in text.lines() {
        let fields: Vec<&str> = line.split(' ').collect();
        let hex = |s: &str| parse_hex(s);
        match fields[0] {
            "source" => {}
            "witness" | "case" => {
                let n: usize = fields[2].parse().unwrap();
                current = Some(Pencil {
                    id: fields[1].to_string(),
                    is_witness: fields[0] == "witness",
                    n_finite: fields.get(3).map_or(n, |s| s.parse().unwrap()),
                    k: vec![vec![0.0; n]; n],
                    m: vec![vec![0.0; n]; n],
                    shifts: Vec::new(),
                });
            }
            "k" | "m" => {
                let p = current.as_mut().unwrap();
                let (i, j): (usize, usize) =
                    (fields[1].parse().unwrap(), fields[2].parse().unwrap());
                let target = if fields[0] == "k" { &mut p.k } else { &mut p.m };
                target[i][j] = hex(fields[3]);
                target[j][i] = hex(fields[3]);
            }
            "shift" => current.as_mut().unwrap().shifts.push(FixtureShift {
                kind: fields[1].to_string(),
                sigma: hex(fields[2]),
                below: fields[3].parse().unwrap(),
                equal: fields[4].parse().unwrap(),
            }),
            "end" => pencils.push(current.take().unwrap()),
            other => panic!("unknown fixture record {other}"),
        }
    }
    pencils
}

/// Parses `[-]0x1.<hex>p<exp>` (Python's `float.hex`), exactly.
fn parse_hex(s: &str) -> f64 {
    let (negative, s) = match s.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, s),
    };
    let s = s.strip_prefix("0x").expect("hex float");
    let (mantissa, exponent) = s.split_once('p').expect("hex exponent");
    let exponent: i32 = exponent.parse().unwrap();
    let (lead, frac) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let mut bits: u64 = u64::from_str_radix(lead, 16).unwrap();
    for c in frac.chars() {
        bits = (bits << 4) | c.to_digit(16).unwrap() as u64;
    }
    let shift = exponent - 4 * frac.len() as i32;
    // bits < 2^57 and the value is a binary64, so these products are exact.
    let mut value = bits as f64;
    let mut e = shift;
    while e > 0 {
        let step = e.min(1000);
        value *= 2f64.powi(step);
        e -= step;
    }
    while e < 0 {
        let step = (-e).min(1000);
        value /= 2f64.powi(step);
        e += step;
    }
    if negative {
        -value
    } else {
        value
    }
}

fn sym(rows: &Rows) -> SymmetricMatrix {
    SymmetricMatrix::from_rows(rows).unwrap()
}

/// The computed spectrum: eigenvalues ascending, vectors and the test-local
/// e_λ of each mode.
struct Spectrum {
    eigenvalues: Vec<f64>,
    vectors: Vec<Vec<f64>>,
    errors: Vec<f64>,
    clusters: Vec<usize>,
}

impl Spectrum {
    fn of(k: &Rows, m: &Rows) -> (Self, DenseEigenSolution) {
        let solution = solve_dense_pencil(&sym(k), &sym(m), &dense_options()).unwrap();
        let vectors: Vec<Vec<f64>> = (0..solution.mode_count())
            .map(|i| solution.vector(i).to_vec())
            .collect();
        let errors = solution
            .eigenvalues
            .iter()
            .zip(&vectors)
            .map(|(&lambda, phi)| error_estimate(k, m, lambda, phi))
            .collect();
        let clusters = cluster_ids(&solution.eigenvalues, CLUSTER_DELTA).unwrap();
        (
            Self {
                eigenvalues: solution.eigenvalues.clone(),
                vectors,
                errors,
                clusters,
            },
            solution,
        )
    }

    fn neighbour(&self, i: usize) -> Neighbour<'_> {
        Neighbour {
            eigenvalue: self.eigenvalues[i],
            relative_error_estimate: self.errors[i],
            vector: &self.vectors[i],
        }
    }

    /// The nearest computed eigenvalue below σ and the nearest at or above.
    fn neighbours_of(&self, sigma: f64) -> Vec<Neighbour<'_>> {
        let split = self.eigenvalues.partition_point(|&l| l < sigma);
        let mut neighbours = Vec::new();
        if split > 0 {
            neighbours.push(self.neighbour(split - 1));
        }
        if split < self.eigenvalues.len() {
            neighbours.push(self.neighbour(split));
        }
        neighbours
    }

    /// (index of the top member of the lower cluster, index of the bottom
    /// member of the upper cluster) for each gap between computed clusters.
    fn gaps(&self) -> Vec<(usize, usize)> {
        (1..self.eigenvalues.len())
            .filter(|&i| self.clusters[i] != self.clusters[i - 1])
            .map(|i| (i - 1, i))
            .collect()
    }
}

/// e_λ = ‖D r‖₂·‖φ̃‖₂ / (λ·φᵀMφ) on the formed pencil, r = Kφ − λMφ in
/// double-double, D = diag(k_ii^-½), φ̃ = D⁻¹φ.
fn error_estimate(k: &Rows, m: &Rows, lambda: f64, phi: &[f64]) -> f64 {
    let n = phi.len();
    let mut residual_sq = DdAccumulator::new();
    let mut scaled_norm_sq = DdAccumulator::new();
    let mut quadratic = DdAccumulator::new();
    for i in 0..n {
        let mut kr = DdAccumulator::new();
        let mut mr = DdAccumulator::new();
        for j in 0..n {
            kr.add_product(k[i][j], phi[j]);
            mr.add_product(m[i][j], phi[j]);
        }
        let r = kr.value().sub(mr.value().mul_f64(lambda)).to_f64();
        let scaled = r / k[i][i].sqrt();
        residual_sq.add_product(scaled, scaled);
        scaled_norm_sq.add_dd(Dd::product(phi[i], phi[i]).mul_f64(k[i][i]));
        quadratic.add_dd(mr.value().mul_f64(phi[i]));
    }
    let numerator = residual_sq
        .value()
        .sqrt()
        .mul(scaled_norm_sq.value().sqrt());
    numerator.div(quadratic.value().mul_f64(lambda)).to_f64()
}

fn pencil_of(k: &Rows, m: &Rows) -> ShiftPencil {
    ShiftPencil::from_dense(&sym(k), &sym(m)).unwrap()
}

fn describe(outcome: &SturmOutcome) -> String {
    let evidence = outcome.evidence();
    let last = evidence.candidates.last().unwrap();
    let head = match outcome {
        SturmOutcome::Counted { count, .. } => format!("counted {count}"),
        SturmOutcome::Refused { reason, .. } => format!("refused {reason:?}"),
    };
    match &last.factored {
        Some(f) => format!(
            "{head}: sigma {:e}, decision {:?}, negatives {}, e_b {:.3e} (factor {:.3e} + formation {:.3e}), \
             nu {:.3e}, product {:.3e} (limit {:.1e}), separation ratio {}, g {:.3e}",
            last.sigma,
            last.decision,
            f.negative_pivot_count,
            f.backward_error,
            f.factor_term,
            last.formation_term.unwrap(),
            f.inverse_norm_estimate,
            f.conditioning_product,
            evidence.conditioning_limit,
            f.separation_ratio
                .map_or("none".to_string(), |r| format!("{r:.3e}")),
            f.growth
        ),
        None => format!(
            "{head}: sigma {:e}, decision {:?}",
            last.sigma, last.decision
        ),
    }
}

/// Every count taken equals the exact count; returns the conditioning
/// products of the counted shifts.
fn assert_counts_agree(pencil: &Pencil) -> (usize, usize, Vec<f64>) {
    let (spectrum, _) = Spectrum::of(&pencil.k, &pencil.m);
    let shifted = pencil_of(&pencil.k, &pencil.m);
    let mut products = Vec::new();
    let (mut counted, mut refused) = (0, 0);
    for shift in &pencil.shifts {
        let neighbours = spectrum.neighbours_of(shift.sigma);
        let outcome = count_at_shift(&shifted, shift.sigma, &neighbours, &parameters()).unwrap();
        eprintln!(
            "{} {} {}: {}",
            pencil.id,
            shift.kind,
            shift.below,
            describe(&outcome)
        );
        match &outcome {
            SturmOutcome::Counted { count, evidence } => {
                assert_eq!(
                    (*count, shift.equal),
                    (shift.below, 0),
                    "{} at {} ({:e}): a count was taken and is wrong",
                    pencil.id,
                    shift.kind,
                    shift.sigma
                );
                counted += 1;
                products.push(
                    evidence
                        .accepted()
                        .unwrap()
                        .factored
                        .as_ref()
                        .unwrap()
                        .conditioning_product,
                );
            }
            SturmOutcome::Refused { evidence, .. } => {
                refused += 1;
                // The negative-pivot count is evidence only; when it is wrong
                // the refusal is what protects the result.
                let _ = evidence;
            }
        }
    }
    (counted, refused, products)
}

fn witness(id: &str) -> Pencil {
    fixture().into_iter().find(|p| p.id == id).unwrap()
}

#[test]
fn fixture_parses_and_matches_its_sources() {
    let pencils = fixture();
    assert_eq!(pencils.iter().filter(|p| p.is_witness).count(), 8);
    assert_eq!(pencils.iter().filter(|p| !p.is_witness).count(), 9);
    assert_eq!(parse_hex("0x1.2aa413b36221fp-2"), 0.29164152891510925);
    assert_eq!(parse_hex("-0x1.cccccccd64ccdp-2"), -0.4500000000345608);
    assert_eq!(parse_hex("0x1.0000000000000p-1022"), f64::MIN_POSITIVE);
    for p in &pencils {
        for i in 0..p.k.len() {
            assert!(p.k[i][i] > 0.0, "{} K diagonal {i}", p.id);
        }
    }
}

#[test]
fn every_witness_count_taken_is_exact_and_miscounts_are_refused() {
    let mut raw_miscounts_refused = 0;
    for pencil in fixture().iter().filter(|p| p.is_witness) {
        let (spectrum, _) = Spectrum::of(&pencil.k, &pencil.m);
        let shifted = pencil_of(&pencil.k, &pencil.m);
        for shift in &pencil.shifts {
            let neighbours = spectrum.neighbours_of(shift.sigma);
            let outcome =
                count_at_shift(&shifted, shift.sigma, &neighbours, &parameters()).unwrap();
            eprintln!(
                "{} {} (exact {}): {}",
                pencil.id,
                shift.kind,
                shift.below,
                describe(&outcome)
            );
            let candidate = outcome.evidence().candidates.last().unwrap();
            let raw = candidate.factored.as_ref().map(|f| f.negative_pivot_count);
            if let SturmOutcome::Counted { count, .. } = outcome {
                assert_eq!(
                    (count, shift.equal),
                    (shift.below, 0),
                    "{} at {:e}: a wrong count was taken",
                    pencil.id,
                    shift.sigma
                );
            } else if raw.is_some_and(|c| c != shift.below) {
                raw_miscounts_refused += 1;
            }
        }
    }
    eprintln!("binary64 miscounts met and refused: {raw_miscounts_refused}");
    assert!(
        raw_miscounts_refused > 0,
        "the suite must meet binary64 miscounts"
    );
}

#[test]
fn missed_eigenvalue_witness() {
    // K = I, M = diag(1, 2, 3): λ = 1/3, 1/2, 1.
    let pencil = witness("WIT-MISSED");
    let (spectrum, solution) = Spectrum::of(&pencil.k, &pencil.m);
    assert_eq!(solution.eigenvalues, vec![1.0 / 3.0, 0.5, 1.0]);
    let shifted = pencil_of(&pencil.k, &pencil.m);
    // A solver reporting {1/2, 1} has missed 1/3: the whole-spectrum count
    // is 3, not 2, which is how the miss is detected.
    let top = spectrum.neighbour(2);
    let above = count_above_spectrum(&shifted, &top, 3, &parameters()).unwrap();
    eprintln!("WIT-MISSED above the spectrum: {}", describe(&above));
    assert_eq!(above.count(), Some(3));
    // A solver reporting {1/3, 1/2} places its final shift in (1/2, 1).
    let gap = Gap {
        lower: GapLower::Mode(spectrum.neighbour(1)),
        upper: spectrum.neighbour(2),
    };
    let in_gap = count_in_gap(&shifted, &gap, &parameters()).unwrap();
    eprintln!("WIT-MISSED gap (1/2, 1): {}", describe(&in_gap));
    assert_eq!(in_gap.count(), Some(2));
    // The shift 1/2 lies on an eigenvalue: the factorization meets an exact
    // zero pivot, and the formation term alone shows the shift unseparated.
    let on = count_at_shift(&shifted, 0.5, &spectrum.neighbours_of(0.5), &parameters()).unwrap();
    eprintln!("WIT-MISSED at 1/2: {}", describe(&on));
    assert!(matches!(
        on,
        SturmOutcome::Refused {
            reason: SturmRefusal::Unseparated,
            ..
        }
    ));
    let candidate = &on.evidence().candidates[0];
    assert!(candidate.factored.is_none());
    assert!(candidate.formation_separation_ratio.unwrap() <= 0.0);
}

#[test]
fn zero_pivot_witness_in_the_product_order() {
    // K = chain(1, 1, 1), M = I. The witness's exact zero pivot at σ = 2 is
    // met in the natural order; the reverse Cuthill–McKee order of this path
    // is [2, 1, 0], in which σ = 2 factors without it and counts exactly,
    // and σ = 1 meets an exact zero first pivot (k₂₂ − σ·m₂₂ = 0) though 1
    // is not an eigenvalue.
    let pencil = witness("WIT-ZERO-PIVOT");
    let (spectrum, _) = Spectrum::of(&pencil.k, &pencil.m);
    let shifted = pencil_of(&pencil.k, &pencil.m);
    assert_eq!(shifted.order(), &[2, 1, 0]);
    let at_two =
        count_at_shift(&shifted, 2.0, &spectrum.neighbours_of(2.0), &parameters()).unwrap();
    eprintln!("WIT-ZERO-PIVOT at 2: {}", describe(&at_two));
    assert_eq!(at_two.count(), Some(2));
    let at_one =
        count_at_shift(&shifted, 1.0, &spectrum.neighbours_of(1.0), &parameters()).unwrap();
    eprintln!("WIT-ZERO-PIVOT at 1: {}", describe(&at_one));
    assert!(matches!(
        at_one,
        SturmOutcome::Refused {
            reason: SturmRefusal::GrowthLimit,
            ..
        }
    ));
    assert!(matches!(
        at_one.evidence().candidates[0].decision,
        ShiftDecision::FactorizationRejected(_)
    ));
    // Placed in its gap instead, the count is taken and is exact.
    let (lo, hi) = spectrum.gaps()[0];
    let gap = Gap {
        lower: GapLower::Mode(spectrum.neighbour(lo)),
        upper: spectrum.neighbour(hi),
    };
    let placed = count_in_gap(&shifted, &gap, &parameters()).unwrap();
    eprintln!("WIT-ZERO-PIVOT gap 1: {}", describe(&placed));
    assert_eq!(placed.count(), Some(1));
}

#[test]
fn round_two_witness_shift_is_refused_and_its_gap_counts() {
    let pencil = witness("WIT-ROUND2");
    let (spectrum, _) = Spectrum::of(&pencil.k, &pencil.m);
    let shifted = pencil_of(&pencil.k, &pencil.m);
    let sigma = parse_hex("0x1.2aa413b36221fp-2");
    let outcome = count_at_shift(
        &shifted,
        sigma,
        &spectrum.neighbours_of(sigma),
        &parameters(),
    )
    .unwrap();
    eprintln!("WIT-ROUND2 sigma: {}", describe(&outcome));
    assert!(
        matches!(
            outcome,
            SturmOutcome::Refused {
                reason: SturmRefusal::Unseparated,
                ..
            }
        ),
        "the separation rule must refuse the round-2 shift"
    );
    // Every gap of the 32×32 pencil counts exactly by placement.
    for (lo, hi) in spectrum.gaps() {
        let gap = Gap {
            lower: GapLower::Mode(spectrum.neighbour(lo)),
            upper: spectrum.neighbour(hi),
        };
        let placed = count_in_gap(&shifted, &gap, &parameters()).unwrap();
        assert_eq!(
            placed.count(),
            Some(hi),
            "gap {lo}-{hi}: {}",
            describe(&placed)
        );
    }
}

/// K = diag(1, 1, B), M = diag(1/32, 1/256, 8B), B = HHᵀ, H the 40×40 unit
/// lower bidiagonal with subdiagonal 2: forty eigenvalues at 1/8, then 32
/// and 256; the exact count at σ = √(32·256) is 41. `reversed` numbers B's
/// rows last to first (the same pencil up to a permutation).
fn round_three_witness(reversed: bool) -> (Rows, Rows) {
    let size = 40;
    let n = size + 2;
    let mut k = vec![vec![0.0; n]; n];
    let mut m = vec![vec![0.0; n]; n];
    k[0][0] = 1.0;
    k[1][1] = 1.0;
    m[0][0] = 1.0 / 32.0;
    m[1][1] = 1.0 / 256.0;
    let at = |i: usize| if reversed { 2 + size - 1 - i } else { 2 + i };
    // B = HHᵀ: diagonal 1, 5, 5, …; off-diagonal 2 (exact integers).
    for i in 0..size {
        let b = if i == 0 { 1.0 } else { 5.0 };
        k[at(i)][at(i)] = b;
        m[at(i)][at(i)] = 8.0 * b;
        if i + 1 < size {
            for (a, scale) in [(&mut k, 1.0), (&mut m, 8.0)] {
                a[at(i + 1)][at(i)] = 2.0 * scale;
                a[at(i)][at(i + 1)] = 2.0 * scale;
            }
        }
    }
    (k, m)
}

/// The review's round-3 witness: binary64 miscounts 40 for 41 with a large
/// separation and modest growth, and the conditioning premise must refuse it.
///
/// The miscount depends on the elimination order. In the reverse
/// Cuthill–McKee order of the witness as numbered, B's path is eliminated
/// from its far end, the near-singular direction lands on the last pivot,
/// and the pivot guard refuses the factorization before any premise is
/// evaluated. Numbered the other way, the product's order eliminates B from
/// H's leading row, as the review did: then the guard, the separation and
/// the growth all pass, the negative-pivot count is the review's wrong 40,
/// and the conditioning premise refuses it.
#[test]
fn round_three_witness_is_refused_ill_conditioned() {
    let sigma = (32.0f64 * 256.0).sqrt();
    let n = 42;
    let mut e0 = vec![0.0; n];
    e0[0] = 1.0;
    let mut e1 = vec![0.0; n];
    e1[1] = 1.0;
    // The exact neighbours: λ = 32 with φ = e₀, λ = 256 with φ = e₁.
    let neighbours = [
        Neighbour {
            eigenvalue: 32.0,
            relative_error_estimate: 0.0,
            vector: &e0,
        },
        Neighbour {
            eigenvalue: 256.0,
            relative_error_estimate: 0.0,
            vector: &e1,
        },
    ];
    let gap = Gap {
        lower: GapLower::Mode(neighbours[0]),
        upper: neighbours[1],
    };

    // As numbered: refused by the pivot guard.
    let (k, m) = round_three_witness(false);
    let shifted = pencil_of(&k, &m);
    let outcome = count_at_shift(&shifted, sigma, &neighbours, &parameters()).unwrap();
    eprintln!("round-3 witness as numbered: {}", describe(&outcome));
    assert!(outcome.count().is_none());
    assert!(matches!(
        outcome.evidence().candidates[0].decision,
        ShiftDecision::FactorizationRejected(_)
    ));

    // In the review's elimination order: refused IllConditioned.
    let (k, m) = round_three_witness(true);
    let shifted = pencil_of(&k, &m);
    let outcome = count_at_shift(&shifted, sigma, &neighbours, &parameters()).unwrap();
    eprintln!("round-3 witness, review order: {}", describe(&outcome));
    let SturmOutcome::Refused {
        reason: SturmRefusal::IllConditioned,
        evidence,
    } = &outcome
    else {
        panic!(
            "round-3 witness must be refused IllConditioned: {}",
            describe(&outcome)
        );
    };
    let f = evidence.candidates[0].factored.as_ref().unwrap();
    assert!(f.separated && f.within_growth_limit && !f.conditioned);
    assert!(f.separation_ratio.unwrap() > 1e9);
    assert_eq!(f.negative_pivot_count, 40, "the review's miscount");

    // Placement in the gap cannot rescue either numbering: B's
    // conditioning is shared by every shift in (32, 256).
    for reversed in [false, true] {
        let (k, m) = round_three_witness(reversed);
        let placed = count_in_gap(&pencil_of(&k, &m), &gap, &parameters()).unwrap();
        eprintln!(
            "round-3 witness placed (reversed {reversed}): {}",
            describe(&placed)
        );
        assert!(placed.count().is_none());
    }
}

/// K = I₂, σ = 1.125 and a binary64 M whose exact K − σM is positive
/// definite with determinant ≈ 1.1·10⁻²³. The factor-only estimate admitted
/// a miscount; with the formation term the shift must be refused.
#[test]
fn round_four_witness_is_refused() {
    let k = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    let off = -1.7777777777777775e-9;
    let m = vec![vec![0.88888888888, off], vec![off, 0.8888885333343496]];
    let (spectrum, _) = Spectrum::of(&k, &m);
    let shifted = pencil_of(&k, &m);
    let sigma = 1.125;
    let outcome = count_at_shift(
        &shifted,
        sigma,
        &spectrum.neighbours_of(sigma),
        &parameters(),
    )
    .unwrap();
    eprintln!(
        "round-4 witness (computed eigenvalues {:?}): {}",
        spectrum.eigenvalues,
        describe(&outcome)
    );
    assert!(outcome.count().is_none(), "round-4 witness must be refused");
}

/// K = 2⁻¹⁰²²·I₂, M = 2⁻⁶⁰⁰·[[2, 1], [1, 2]], σ = 2⁻⁶⁰⁰: σ·m₁₂ = 2⁻¹²⁰⁰
/// rounds to zero while the exact scaled entry is −2⁻¹⁷⁸.
#[test]
fn round_six_underflow_witness_is_out_of_range() {
    let tiny = f64::MIN_POSITIVE;
    let s = 2f64.powi(-600);
    let k = vec![vec![tiny, 0.0], vec![0.0, tiny]];
    let m = vec![vec![2.0 * s, s], vec![s, 2.0 * s]];
    // Since review round 3 the witness's entries are outside the admission
    // range and the pencil is refused at entry.
    assert!(matches!(
        ShiftPencil::from_dense(&sym(&k), &sym(&m)),
        Err(SturmError::PencilRangeUnsupported(_))
    ));
    // Within the range the formation's own range check still applies:
    // K = I₂, M = 2⁻³⁰⁰·[[2, 1], [1, 2]] and σ = 2⁻⁸⁰⁰, so σ·m₁₂ = 2⁻¹¹⁰⁰
    // rounds to zero.
    let s = 2f64.powi(-300);
    let k = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    let m = vec![vec![2.0 * s, s], vec![s, 2.0 * s]];
    let shifted = pencil_of(&k, &m);
    let outcome = count_at_shift(&shifted, 2f64.powi(-800), &[], &parameters()).unwrap();
    eprintln!("round-6 underflow, in range: {}", describe(&outcome));
    let SturmOutcome::Refused {
        reason: SturmRefusal::OutOfRange,
        evidence,
    } = &outcome
    else {
        panic!("the in-range underflow must be OutOfRange");
    };
    assert!(matches!(
        evidence.candidates[0].decision,
        ShiftDecision::OutOfRange(_)
    ));
}

/// K = I₂ and an SPD M with determinant 2⁻¹⁰⁴: the second finite eigenvalue
/// (≈ 1.3·10³¹) comes back as μ ≈ 0. The finite-mode count must be
/// unresolved on both halves.
#[test]
fn round_six_finite_count_witness_is_unresolved() {
    let scale = 2f64.powi(-52);
    let (a, b, c) = (
        806515533049393.0 * scale,
        1304969544928657.0 * scale,
        2111485077978050.0 * scale,
    );
    let k = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    let m = vec![vec![a, b], vec![b, c]];
    assert_eq!(
        established_finite_mode_count(&sym(&m), MASS_PIVOT_RELATIVE_THRESHOLD),
        None
    );
    let solution = solve_dense_pencil(&sym(&k), &sym(&m), &dense_options()).unwrap();
    eprintln!(
        "round-6 finite-count witness: mu {:?}, nonpositive {}, below threshold {}",
        solution.mu, solution.nonpositive_mu_count, solution.below_threshold_mu_count
    );
    assert_eq!(solution.structurally_massless_count, 0);
    assert!(solution.finite_count_unresolved);
}

#[test]
fn finite_count_is_established_with_structural_zeros() {
    // T₁₇ with mass on the odd DOFs only: 9 structurally massless DOFs, a
    // diagonal remaining block, 8 finite modes.
    let n = 17;
    let mut k = vec![vec![0.0; n]; n];
    let mut m = vec![vec![0.0; n]; n];
    for i in 0..n {
        k[i][i] = 2.0;
        if i + 1 < n {
            k[i][i + 1] = -1.0;
            k[i + 1][i] = -1.0;
        }
        if i % 2 == 1 {
            m[i][i] = 1.0;
        }
    }
    assert_eq!(
        established_finite_mode_count(&sym(&m), MASS_PIVOT_RELATIVE_THRESHOLD),
        Some(8)
    );
    let solution = solve_dense_pencil(&sym(&k), &sym(&m), &dense_options()).unwrap();
    eprintln!("T17: resolution-floor margin {:.2e}", floor_margin(&solution));
    assert_eq!(solution.structurally_massless_count, 9);
    assert!(!solution.finite_count_unresolved, "mu {:?}", solution.mu);
    // A coupling that is not structural leaves the count unestablished.
    m[0][1] = 0.5;
    m[1][0] = 0.5;
    assert_eq!(
        established_finite_mode_count(&sym(&m), MASS_PIVOT_RELATIVE_THRESHOLD),
        None
    );
    // An invalid threshold establishes nothing.
    assert_eq!(established_finite_mode_count(&sym(&k), 1.0), None);
}

/// Independent review of PR #33, finding 1: K = I₂, M = diag(1, ¼), the
/// gap (1, 4) with e_λ = 2 below and 0 above. At σ = 2 the lower
/// neighbour's separation is (|1 − 2| − 2·1)·1 = −1, so σ = 2 is
/// unseparated whatever the vector's scale. The vector (10⁻²⁰⁰, 0) squared
/// to zero, its mass quotient became 0/0 and `f64::min` dropped the NaN,
/// accepting σ = 2 with count 1. The quotient is now formed from the vector
/// scaled by a power of two.
#[test]
fn a_rescaled_neighbour_vector_keeps_its_separation_constraint() {
    let k = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    let m = vec![vec![1.0, 0.0], vec![0.0, 0.25]];
    let pencil = pencil_of(&k, &m);
    let upper_vector = [0.0, 1.0];
    let upper = Neighbour {
        eigenvalue: 4.0,
        relative_error_estimate: 0.0,
        vector: &upper_vector,
    };
    // The smallest subnormal and the largest finite number included.
    for scale in [1.0, 1e-200, 1e200, f64::from_bits(1), f64::MAX] {
        let lower_vector = [scale, 0.0];
        let lower = Neighbour {
            eigenvalue: 1.0,
            relative_error_estimate: 2.0,
            vector: &lower_vector,
        };
        let outcome = count_at_shift(&pencil, 2.0, &[lower, upper], &parameters()).unwrap();
        assert!(
            matches!(
                outcome,
                SturmOutcome::Refused {
                    reason: SturmRefusal::Unseparated,
                    ..
                }
            ),
            "scale {scale:e}: {}",
            describe(&outcome)
        );
        // The geometric mean of the gap is σ = 2, and it is refused there.
        let gap = Gap {
            lower: GapLower::Mode(lower),
            upper,
        };
        let placed = count_in_gap(&pencil, &gap, &parameters()).unwrap();
        let first = &placed.evidence().candidates[0];
        assert_eq!(first.sigma, 2.0);
        assert_eq!(first.decision, ShiftDecision::Unseparated, "scale {scale:e}");
    }
}

/// Review round 2, item 2: K = [[2⁻²⁴⁰, 2³⁷⁹], [2³⁷⁹, 2¹⁰⁰⁰]],
/// M = [[2⁻²⁴⁰ + 2⁻²²², 2³⁷⁸], [2³⁷⁸, (1 + 2⁻²²)·2¹⁰⁰⁰]], φ = (1, 2⁻⁶⁰⁰),
/// λ = 1, e_λ = 1 − 2⁻³⁰, σ = 2. φ₂² = 2⁻¹²⁰⁰ underflowed before it met
/// k₂₂ and m₂₂, the quotient came out as 786433 instead of ≈ 1.00000095,
/// and σ was accepted. The true separation, 2⁻³⁰·q ≈ 9.31·10⁻¹⁰, is below
/// κ_s·e_b ≈ 4.07·10⁻⁸: σ was then shown unseparated with the
/// extended-exponent quotient; since review round 3 the witness is refused
/// at admission.
#[test]
fn a_weighted_square_below_the_binary64_range_still_meets_its_weight() {
    let p = |e: i32| 2f64.powi(e);
    let k = vec![vec![p(-240), p(379)], vec![p(379), p(1000)]];
    let m = vec![
        vec![p(-240) + p(-222), p(378)],
        vec![p(378), (1.0 + p(-22)) * p(1000)],
    ];
    // Since review round 3 the witness's entries (2³⁷⁹, 2¹⁰⁰⁰) are outside
    // the admission range and the pencil is refused at entry.
    assert!(matches!(
        ShiftPencil::from_dense(&sym(&k), &sym(&m)),
        Err(SturmError::PencilRangeUnsupported(_))
    ));
    // Within the range: φ = 2⁻⁶⁰⁰·(1, 1), whose squares are below binary64,
    // on K = I₂ and M = diag(1, 3). The quotient is 4/2 = 2, so at σ = ½
    // beside λ = 1 (e_λ = 0) the separation is ½·2 = 1.
    let pencil = pencil_of(
        &vec![vec![1.0, 0.0], vec![0.0, 1.0]],
        &vec![vec![1.0, 0.0], vec![0.0, 3.0]],
    );
    let vector = [p(-600), p(-600)];
    let neighbour = Neighbour {
        eigenvalue: 1.0,
        relative_error_estimate: 0.0,
        vector: &vector,
    };
    let outcome = count_at_shift(&pencil, 0.5, &[neighbour], &parameters()).unwrap();
    let candidate = &outcome.evidence().candidates[0];
    let separation = candidate.formation_separation_ratio.unwrap() * candidate.formation_term.unwrap();
    assert!((separation - 1.0).abs() <= 4.0 * f64::EPSILON, "{separation}");
}

/// Finding 1, the typed refusals: a neighbour whose mass quotient is not
/// positive and normal is refused before any shift, and a separation that
/// is not finite refuses the candidate instead of entering the minimum.
#[test]
fn neighbour_quotients_and_separations_out_of_range_are_refused() {
    let identity = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    // A vector in the null space of a singular M: φᵀMφ = 0.
    let singular = pencil_of(&identity, &vec![vec![1.0, 0.0], vec![0.0, 0.0]]);
    let null = [0.0, 1.0];
    let neighbour = |eigenvalue, relative_error_estimate, vector| Neighbour {
        eigenvalue,
        relative_error_estimate,
        vector,
    };
    assert_eq!(
        count_at_shift(&singular, 0.5, &[neighbour(1.0, 0.0, &null)], &parameters()),
        Err(SturmError::NeighbourMassQuotient {
            index: 0,
            value: 0.0
        })
    );
    // A subnormal mass, which gave a subnormal quotient, is outside the
    // admission range since review round 3 and refused at entry.
    assert!(matches!(
        ShiftPencil::from_dense(&sym(&identity), &sym(&vec![vec![1e-310, 0.0], vec![0.0, 1e-310]])),
        Err(SturmError::PencilRangeUnsupported(_))
    ));
    // s = (|λ − σ| − e_λ·λ)·q with e_λ·λ overflowing (−∞), and with the
    // product by q overflowing (+∞).
    let pencil = pencil_of(&identity, &vec![vec![1e10, 0.0], vec![0.0, 1.0]]);
    let e1 = [1.0, 0.0];
    for nb in [neighbour(1e300, 1e10, &e1), neighbour(1e300, 0.0, &e1)] {
        let outcome = count_at_shift(&pencil, 0.5, &[nb], &parameters()).unwrap();
        let SturmOutcome::Refused {
            reason: SturmRefusal::OutOfRange,
            evidence,
        } = &outcome
        else {
            panic!("{}", describe(&outcome));
        };
        assert!(matches!(
            evidence.candidates[0].decision,
            ShiftDecision::SeparationOutOfRange { neighbour: 0, value } if value.is_infinite()
        ));
    }
}

/// Finding 3, under the condensation rule (design §3.2 item 5, revised
/// after review round 2): K = [[2, 1], [1, 1]], M = diag(1, 0). DOF 1 is
/// structurally massless; uncondensed, C's zero eigenvalue came back as
/// μ ≈ −1.1·10⁻¹⁶. Condensed, K̂ = 2 − 1·1⁻¹·1 = 1 and M̂ = 1: one μ = 1,
/// none classified infinite, resolved, and the shape recovered on the
/// massless DOF as φ₁ = −K₁₁⁻¹K₁₀φ₀ = −φ₀.
#[test]
fn a_structural_zero_is_condensed_out() {
    let k = vec![vec![2.0, 1.0], vec![1.0, 1.0]];
    let m = vec![vec![1.0, 0.0], vec![0.0, 0.0]];
    let solution = solve_dense_pencil(&sym(&k), &sym(&m), &dense_options()).unwrap();
    assert_eq!(solution.structurally_massless_count, 1);
    assert_eq!(solution.condensed_dimension, 1);
    assert_eq!(solution.nonpositive_mu_count, 0);
    assert_eq!(solution.below_threshold_mu_count, 0);
    assert_eq!(solution.eigenvalues, vec![1.0]);
    let phi = solution.vector(0);
    assert_eq!(phi[1], -phi[0]);
    assert_eq!(phi[0].abs(), 1.0);
    assert_eq!(solution.established_finite_mode_count, Some(1));
    assert!(!solution.finite_count_unresolved);
}

/// Review round 2, item 1: with τ_μ = 10⁻¹⁸, K = [[1, 1, 0], [1, 3, 0],
/// [0, 0, 10²⁰]] and M = diag(1, 0, 1). Uncondensed, the 10²⁰ mode came
/// back as λ = 2⁵² (μ = 2⁻⁵², the structural zero's rounding) and its own
/// μ = 10⁻²⁰ was counted infinite in its place: one infinite μ against one
/// structural zero, resolved, with the mode lost. Condensed, K̂ =
/// diag(2/3, 10²⁰) and M̂ = I: μ = (3/2, 10⁻²⁰). At τ_μ = 10⁻¹⁸ the 10²⁰
/// mode is classified infinite and the count is unresolved; at τ_μ = 0 the
/// resolution floor (review round 3) still classifies it infinite.
#[test]
fn a_finite_mode_traded_for_a_structural_zero_is_not_concealed() {
    let k = vec![
        vec![1.0, 1.0, 0.0],
        vec![1.0, 3.0, 0.0],
        vec![0.0, 0.0, 1e20],
    ];
    let m = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, 1.0],
    ];
    let options = |threshold| DenseOptions {
        infinite_mu_relative_threshold: threshold,
        ..dense_options()
    };
    let solution = solve_dense_pencil(&sym(&k), &sym(&m), &options(1e-18)).unwrap();
    eprintln!(
        "round-2 witness at τ_μ = 1e-18: λ {:?}, μ_max {:e}, below threshold {}",
        solution.eigenvalues, solution.mu_max, solution.below_threshold_mu_count
    );
    assert_eq!(solution.structurally_massless_count, 1);
    assert_eq!(solution.below_threshold_mu_count, 1);
    assert_eq!(solution.mode_count(), 1);
    assert!((solution.eigenvalues[0] - 2.0 / 3.0).abs() <= 4.0 * f64::EPSILON);
    assert!(solution.finite_count_unresolved);
    // Since review round 3 the resolution floor applies as well: with
    // τ_μ = 0, μ = 10⁻²⁰ is still below κ_μ·δ_μ (δ_μ = c_μ·n·u·ν(K̂)·‖M_mm‖
    // ≈ 5·10⁻¹⁶ here), so the 10²⁰ mode is not relied on and the count
    // stays unresolved.
    let solution = solve_dense_pencil(&sym(&k), &sym(&m), &options(0.0)).unwrap();
    assert!(solution.resolution_floor > 1e-20);
    assert_eq!(solution.mode_count(), 1);
    assert!(solution.finite_count_unresolved);
}

/// Review round 3, finding 5 (design §3.2 item 5, "Resolution floor"):
/// K = [[1, 1], [1, 3]], M = diag(1, 2⁻¹⁰⁰), τ_μ = 10⁻¹⁸. The true second
/// μ is about 2⁻¹⁰⁰/2, far below the reduction's resolution; it came back as
/// a rounding artifact near 2⁻⁵² (λ ≈ 2⁵²) above τ_μ·μ_max, retained, with
/// the count resolved. A retained μ must now exceed κ_μ·δ_μ too, so the
/// artifact is classified infinite and the count is unresolved.
#[test]
fn a_rounding_artifact_below_the_resolution_floor_is_not_retained() {
    let k = vec![vec![1.0, 1.0], vec![1.0, 3.0]];
    let m = vec![vec![1.0, 0.0], vec![0.0, 2f64.powi(-100)]];
    let options = DenseOptions {
        infinite_mu_relative_threshold: 1e-18,
        ..dense_options()
    };
    let solution = solve_dense_pencil(&sym(&k), &sym(&m), &options).unwrap();
    eprintln!(
        "resolution-floor witness: μ_max {:e}, ν(K̂) {:e}, ‖M‖ {:e}, δ_μ {:e}, threshold {:e}, \
         retained {:?}, below threshold {}, nonpositive {}",
        solution.mu_max,
        solution.condensed_inverse_norm_estimate,
        solution.condensed_mass_norm,
        solution.resolution_floor,
        solution.classification_threshold,
        solution.mu,
        solution.below_threshold_mu_count,
        solution.nonpositive_mu_count
    );
    assert_eq!(solution.mode_count(), 1);
    assert_eq!(
        solution.below_threshold_mu_count + solution.nonpositive_mu_count,
        1
    );
    assert!(solution.finite_count_unresolved);
    // The artifact itself, retained when κ_μ is set negligible: below δ_μ.
    let observe = DenseOptions {
        resolution_floor_multiple: 1e-30,
        ..options
    };
    let artifact = solve_dense_pencil(&sym(&k), &sym(&m), &observe).unwrap();
    let mu = artifact.mu[1];
    eprintln!(
        "resolution-floor witness: artifact μ {mu:e} (λ {:e}), δ_μ/μ {:.2e}",
        artifact.eigenvalues[1],
        artifact.resolution_floor / mu
    );
    assert!(mu > 0.0 && mu <= artifact.resolution_floor);
}

/// A genuine finite μ classified infinite leaves the count unresolved
/// whether or not a structural zero is beside it.
/// - The round-6 block (a finite mode whose μ rounds to about 0) with a
///   third, structurally massless DOF.
/// - K = diag(1, 10¹³, 1), M = diag(1, 1, 0): condensed μ = (1, 10⁻¹³),
///   the finite 10⁻¹³ at or below τ_μ·μ_max = 10⁻¹². The mass block is
///   diagonal and established, so the classification alone decides.
#[test]
fn a_finite_mu_classified_infinite_beside_a_structural_zero_is_unresolved() {
    let scale = 2f64.powi(-52);
    let (a, b, c) = (
        806515533049393.0 * scale,
        1304969544928657.0 * scale,
        2111485077978050.0 * scale,
    );
    let identity = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
    ];
    let m = vec![vec![a, b, 0.0], vec![b, c, 0.0], vec![0.0, 0.0, 0.0]];
    let solution = solve_dense_pencil(&sym(&identity), &sym(&m), &dense_options()).unwrap();
    assert_eq!(solution.structurally_massless_count, 1);
    assert_eq!(
        solution.nonpositive_mu_count + solution.below_threshold_mu_count,
        1
    );
    assert!(solution.finite_count_unresolved);

    let k = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1e13, 0.0],
        vec![0.0, 0.0, 1.0],
    ];
    let m = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 0.0, 0.0],
    ];
    let solution = solve_dense_pencil(&sym(&k), &sym(&m), &dense_options()).unwrap();
    assert_eq!(solution.structurally_massless_count, 1);
    assert_eq!(solution.below_threshold_mu_count, 1);
    assert_eq!(solution.nonpositive_mu_count, 0);
    assert_eq!(solution.established_finite_mode_count, Some(2));
    assert!(solution.finite_count_unresolved);
}

/// Finding 3: the pivots are those of the equilibrated block D_M·M·D_M and
/// are compared with τ_m inclusively ("at least τ_m"). M = [[1, ½], [½, ½]]
/// equilibrates to [[1, 1/√2], [1/√2, 1]], whose second pivot is ½.
#[test]
fn equilibrated_mass_pivots_meet_the_threshold_inclusively() {
    let m = vec![vec![1.0, 0.5], vec![0.5, 0.5]];
    assert_eq!(established_finite_mode_count(&sym(&m), 0.5), Some(2));
    assert_eq!(established_finite_mode_count(&sym(&m), 0.5000001), None);
    // Scaling a DOF (a change of its unit) leaves the pivots unchanged.
    let scaled = vec![vec![1.0, 0.5e-3], vec![0.5e-3, 0.5e-6]];
    assert_eq!(established_finite_mode_count(&sym(&scaled), 0.4999), Some(2));
    // A remaining diagonal that is not positive establishes nothing.
    let indefinite = vec![vec![1.0, 0.5], vec![0.5, 0.0]];
    assert_eq!(established_finite_mode_count(&sym(&indefinite), 0.0), None);
}

#[test]
fn published_counts_agree_and_well_separated_shifts_are_never_refused() {
    let mut products = Vec::new();
    let (mut counted, mut refused) = (0, 0);
    for pencil in fixture().iter().filter(|p| !p.is_witness) {
        let (c, r, p) = assert_counts_agree(pencil);
        counted += c;
        refused += r;
        products.extend(p);
    }
    eprintln!("published shifts: {counted} counted (all exact), {refused} refused");
    // The well-separated kinds must all be counted.
    for pencil in fixture().iter().filter(|p| !p.is_witness) {
        let (spectrum, _) = Spectrum::of(&pencil.k, &pencil.m);
        let shifted = pencil_of(&pencil.k, &pencil.m);
        for shift in pencil.shifts.iter().filter(|s| {
            matches!(
                s.kind.as_str(),
                "below_lowest" | "gap_geometric_mean" | "above_all_finite"
            )
        }) {
            let neighbours = spectrum.neighbours_of(shift.sigma);
            let outcome =
                count_at_shift(&shifted, shift.sigma, &neighbours, &parameters()).unwrap();
            assert_eq!(
                outcome.count(),
                Some(shift.below),
                "{} {}: {}",
                pencil.id,
                shift.kind,
                describe(&outcome)
            );
        }
    }
    let (lo, hi) = range(&products);
    eprintln!("conditioning products of the counted published shifts: {lo:.3e} to {hi:.3e}");
}

#[test]
fn placed_gap_shifts_count_exactly_with_no_refusal() {
    let mut products = Vec::new();
    let mut ratios = Vec::new();
    let mut growths = Vec::new();
    let mut placements = 0;
    for pencil in fixture().iter().filter(|p| !p.is_witness) {
        let (spectrum, solution) = Spectrum::of(&pencil.k, &pencil.m);
        let shifted = pencil_of(&pencil.k, &pencil.m);
        assert_eq!(solution.mode_count(), pencil.n_finite, "{}", pencil.id);
        let mut record = |outcome: &SturmOutcome, expected: usize, what: &str| {
            assert_eq!(
                outcome.count(),
                Some(expected),
                "{} {what}: {}",
                pencil.id,
                describe(outcome)
            );
            let accepted = outcome.evidence().accepted().unwrap();
            let f = accepted.factored.as_ref().unwrap();
            products.push(f.conditioning_product);
            if let Some(r) = f.separation_ratio {
                ratios.push(r);
            }
            growths.push(f.growth);
            placements += 1;
        };
        // Below the lowest cluster, from a cut-off bound at λ₁/4.
        let lowest = spectrum.neighbour(0);
        let below = Gap {
            lower: GapLower::Bound(lowest.eigenvalue / 4.0),
            upper: lowest,
        };
        record(
            &count_in_gap(&shifted, &below, &parameters()).unwrap(),
            0,
            "below",
        );
        for (lo, hi) in spectrum.gaps() {
            let gap = Gap {
                lower: GapLower::Mode(spectrum.neighbour(lo)),
                upper: spectrum.neighbour(hi),
            };
            let outcome = count_in_gap(&shifted, &gap, &parameters()).unwrap();
            record(&outcome, hi, &format!("gap {lo}-{hi}"));
        }
        // The whole-spectrum shift, only with an established count.
        let established =
            established_finite_mode_count(&sym(&pencil.m), MASS_PIVOT_RELATIVE_THRESHOLD);
        let resolved = !solution.finite_count_unresolved;
        eprintln!(
            "{}: n {}, finite {}, established {:?}, dense resolved {resolved}, \
             resolution-floor margin {:.2e}",
            pencil.id,
            pencil.k.len(),
            pencil.n_finite,
            established,
            floor_margin(&solution)
        );
        if let (Some(count), true) = (established, resolved) {
            assert_eq!(count, solution.mode_count());
            let top = spectrum.neighbour(count - 1);
            let outcome = count_above_spectrum(&shifted, &top, count, &parameters()).unwrap();
            record(&outcome, count, "above the spectrum");
        }
    }
    let (plo, phi) = range(&products);
    let (rlo, rhi) = range(&ratios);
    let (glo, ghi) = range(&growths);
    eprintln!(
        "{placements} placements, none refused: conditioning product {plo:.3e} to {phi:.3e}, \
         separation ratio {rlo:.3e} to {rhi:.3e}, growth {glo:.3e} to {ghi:.3e}"
    );
}

#[test]
fn repeated_runs_are_bit_identical() {
    let pencil = witness("WIT-SCALE-1E6");
    let run = || {
        let (spectrum, _) = Spectrum::of(&pencil.k, &pencil.m);
        let shifted = pencil_of(&pencil.k, &pencil.m);
        let mut outcomes = Vec::new();
        for shift in &pencil.shifts {
            outcomes.push(
                count_at_shift(
                    &shifted,
                    shift.sigma,
                    &spectrum.neighbours_of(shift.sigma),
                    &parameters(),
                )
                .unwrap(),
            );
        }
        for (lo, hi) in spectrum.gaps() {
            let gap = Gap {
                lower: GapLower::Mode(spectrum.neighbour(lo)),
                upper: spectrum.neighbour(hi),
            };
            outcomes.push(count_in_gap(&shifted, &gap, &parameters()).unwrap());
        }
        outcomes
    };
    let (first, second) = (run(), run());
    assert_eq!(format!("{first:?}"), format!("{second:?}"));
    let bits = |outcomes: &[SturmOutcome]| -> Vec<u64> {
        outcomes
            .iter()
            .flat_map(|o| o.evidence().candidates.iter())
            .flat_map(|c| {
                let f = c.factored.as_ref();
                [
                    c.sigma.to_bits(),
                    c.formation_term.unwrap_or(0.0).to_bits(),
                    f.map_or(0, |f| f.backward_error.to_bits()),
                    f.map_or(0, |f| f.inverse_norm_estimate.to_bits()),
                    f.map_or(0, |f| f.growth.to_bits()),
                ]
            })
            .collect()
    };
    assert_eq!(bits(&first), bits(&second));
}

fn range(values: &[f64]) -> (f64, f64) {
    values
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| {
            (lo.min(v), hi.max(v))
        })
}
