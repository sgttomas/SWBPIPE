//! KF2 tests (T3 I20, plan §8; ROOT's KF2 checkpoint-0 rulings). The O(n^2)
//! dense negative-pair witness against a verbatim reference copy of the O(n^4)
//! search it replaces. The equality is bitwise: the outcome's shape, the error
//! variant and message, and every bit of a witness's direction, energy and
//! allowance. Invented inputs only: every matrix, exponent, section and
//! corruption below is stated here.
use super::*;
use crate::{assemble_global_stiffness, FrameElement, FrameNode, FrameSection};
use std::panic::{catch_unwind, AssertUnwindSafe};

type Outcome = Result<Option<StructuralError>, StructuralError>;

// ------------------------------------------------------------------ T1: reference

/// Verbatim copy of `verify_negative_direction` at main `78f55f927`
/// (`FK/structural.rs:2150-2193`), renamed. Test-only reference.
fn reference_verify_negative_direction(
    prepared: &PreparedSystem<'_>,
    direction: &[f64],
) -> Result<Option<StructuralError>, StructuralError> {
    validate_prepared(prepared)?;
    let system = prepared.source;
    if direction.len() != prepared.matrix.len() || direction.iter().any(|v| !v.is_finite()) {
        return Err(StructuralError::InvalidInput("negative direction"));
    }
    let mut energy = 0.0;
    let mut magnitude = 0.0;
    let mut terms = 0;
    for i in 0..direction.len() {
        for j in 0..direction.len() {
            if prepared.matrix[i][j] == 0.0 || direction[i] == 0.0 || direction[j] == 0.0 {
                continue;
            }
            let original = radix_scale(
                system.stiffness[system.free_dofs[i]][system.free_dofs[j]],
                prepared.scale_exponents[i] + prepared.scale_exponents[j],
            )?;
            let term = checked_product(checked_product(direction[i], original)?, direction[j])?;
            energy = checked_value(energy + term)?;
            magnitude = checked_value(magnitude + term.abs())?;
            terms += 1;
        }
    }
    let allowance = 64.0 * gamma(3 * terms + 2) * magnitude;
    if energy < -allowance {
        let mut mapped = vec![0.0; system.force.len()];
        for (i, &global) in system.free_dofs.iter().enumerate() {
            mapped[global] = radix_scale(direction[i], prepared.scale_exponents[i])?;
        }
        Ok(Some(StructuralError::NegativeEnergy {
            direction: mapped,
            energy,
            allowance,
        }))
    } else {
        Ok(None)
    }
}
/// Verbatim copy of `negative_pair_witness` at main `78f55f927`
/// (`FK/structural.rs:2194-2215`), renamed, calling the reference verifier.
fn reference_negative_pair_witness(
    prepared: &PreparedSystem<'_>,
) -> Result<Option<StructuralError>, StructuralError> {
    validate_prepared(prepared)?;
    let n = prepared.matrix.len();
    for i in 0..n {
        for j in 0..i {
            let mut v = vec![0.0; n];
            v[i] = 1.0;
            v[j] = if prepared.matrix[i][j] >= 0.0 {
                -1.0
            } else {
                1.0
            };
            if let Some(witness) = reference_verify_negative_direction(prepared, &v)? {
                return Ok(Some(witness));
            }
        }
    }
    Ok(None)
}

// ------------------------------------------------------------------ T2: comparison

/// Bitwise equality of two outcomes: the shape, the variant and message, and
/// for `NegativeEnergy` every bit of the direction, energy and allowance; the
/// `Debug` bytes as a second check.
fn same(a: &Outcome, b: &Outcome) -> bool {
    fn error_bits(x: &StructuralError, y: &StructuralError) -> bool {
        match (x, y) {
            (
                StructuralError::NegativeEnergy {
                    direction: d1,
                    energy: e1,
                    allowance: a1,
                },
                StructuralError::NegativeEnergy {
                    direction: d2,
                    energy: e2,
                    allowance: a2,
                },
            ) => {
                e1.to_bits() == e2.to_bits()
                    && a1.to_bits() == a2.to_bits()
                    && d1.len() == d2.len()
                    && d1.iter().zip(d2).all(|(p, q)| p.to_bits() == q.to_bits())
            }
            (StructuralError::NegativeEnergy { .. }, _)
            | (_, StructuralError::NegativeEnergy { .. }) => false,
            _ => format!("{x:?}") == format!("{y:?}"),
        }
    }
    let shape = match (a, b) {
        (Ok(None), Ok(None)) => true,
        (Ok(Some(x)), Ok(Some(y))) | (Err(x), Err(y)) => error_bits(x, y),
        _ => false,
    };
    shape && format!("{a:?}") == format!("{b:?}")
}

/// The cells the new search evaluates over its first `visited` pairs, counted
/// independently: each pair's nonzero entries among (j,j), (j,i), (i,j), (i,i).
fn nonzero_cells(prepared: &PreparedSystem<'_>, visited: usize) -> usize {
    let m = &prepared.matrix;
    let mut seen = 0;
    let mut count = 0;
    for i in 0..m.len() {
        for j in 0..i {
            if seen == visited {
                return count;
            }
            seen += 1;
            count += [(j, j), (j, i), (i, j), (i, i)]
                .iter()
                .filter(|&&(a, b)| m[a][b] != 0.0)
                .count();
        }
    }
    count
}

/// T3 + T4: runs the reference, the public search and the counted search on
/// `prepared`; asserts bitwise-equal outcomes and, on an `Ok` outcome, the
/// count invariants. Returns the outcome.
fn check(prepared: &PreparedSystem<'_>, context: &str) -> Outcome {
    let reference = reference_negative_pair_witness(prepared);
    let counted = catch_unwind(AssertUnwindSafe(|| negative_pair_witness_counted(prepared)))
        .unwrap_or_else(|_| panic!("{context}: the new search panicked"));
    let public = catch_unwind(AssertUnwindSafe(|| negative_pair_witness(prepared)))
        .unwrap_or_else(|_| panic!("{context}: the public search panicked"));
    let (new, counts) = match counted {
        Ok((witness, visited, evaluated, verified)) => {
            (Ok(witness), Some((visited, evaluated, verified)))
        }
        Err(error) => (Err(error), None),
    };
    assert!(
        same(&reference, &new),
        "{context}: reference {reference:?}, new {new:?}"
    );
    assert!(same(&public, &new), "{context}: public {public:?}");
    if let (Ok(witness), Some((visited, evaluated, verified))) = (&new, counts) {
        let n = prepared.matrix.len();
        assert_eq!(
            verified,
            usize::from(witness.is_some()),
            "{context}: verifications"
        );
        if witness.is_none() {
            assert_eq!(visited, n * (n - 1) / 2, "{context}: pairs visited");
        } else {
            assert!(visited >= 1 && visited <= n * (n - 1) / 2, "{context}");
        }
        assert_eq!(
            evaluated,
            nonzero_cells(prepared, visited),
            "{context}: cells evaluated"
        );
        assert!(evaluated <= 4 * visited, "{context}: O(1) cells per pair");
    }
    new
}

// ------------------------------------------------------------------ systems

/// splitmix64 (fixed seeds; FK has no dev-dependencies).
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// Uniform in [0, 1).
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1_u64 << 53) as f64
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
    fn chance(&mut self, p: f64) -> bool {
        self.unit() < p
    }
}

fn pow2(k: i32) -> f64 {
    2.0_f64.powi(k)
}

/// An owned structural system (the source a `PreparedSystem` borrows).
struct Case {
    k: Vec<Vec<f64>>,
    f: Vec<f64>,
    free: Vec<usize>,
    bc: Vec<(usize, f64)>,
    evidence: Option<(Vec<Vec<f64>>, Vec<Vec<usize>>)>,
}
impl Case {
    fn system(&self) -> StructuralSystem<'_> {
        StructuralSystem {
            stiffness: &self.k,
            force: &self.f,
            free_dofs: &self.free,
            prescribed: &self.bc,
            contributions: None,
            symmetry: self
                .evidence
                .as_ref()
                .map(|(roundoff, counts)| SymmetryEvidence {
                    absolute_roundoff: roundoff,
                    operation_counts: counts,
                    basis: "KF2 test: invented roundoff allowance",
                }),
        }
    }
    /// All DOFs free, in order.
    fn plain(k: Vec<Vec<f64>>) -> Self {
        let n = k.len();
        Case {
            k,
            f: vec![1.0; n],
            free: (0..n).collect(),
            bc: Vec::new(),
            evidence: None,
        }
    }
}

/// A system specified in the prepared (scaled) space. `a` is the n×n scaled
/// matrix (diagonal in [1, 4), so `prepare_bound`'s exponents are exactly `e`),
/// written into an N×N source through a free map; `prescribed` extra DOFs are
/// restrained. With `skew`, each stored coupling's upper source entry is one
/// ulp away from the lower one, allowed by symmetry evidence, so the prepared
/// matrix is the projection and the two source cells differ.
struct Spec {
    a: Vec<Vec<f64>>,
    e: Vec<i32>,
    prescribed: usize,
    shuffle: bool,
    skew: bool,
}
impl Spec {
    fn build(&self, rng: &mut Rng) -> Case {
        let n = self.a.len();
        let total = n + self.prescribed;
        let mut dofs: Vec<usize> = (0..total).collect();
        if self.shuffle {
            for k in (1..total).rev() {
                dofs.swap(k, rng.below(k + 1));
            }
        }
        let free: Vec<usize> = dofs[..n].to_vec();
        let restrained: Vec<usize> = dofs[n..].to_vec();
        let mut k = vec![vec![0.0; total]; total];
        let mut roundoff = vec![vec![0.0; total]; total];
        for r in 0..n {
            for c in 0..n {
                k[free[r]][free[c]] = self.a[r][c] * pow2(-self.e[r] - self.e[c]);
            }
        }
        if self.skew {
            for r in 0..n {
                for c in 0..r {
                    let lower = k[free[r]][free[c]];
                    if lower != 0.0 {
                        let upper = lower.next_up();
                        k[free[c]][free[r]] = upper;
                        let d = (upper - lower).abs();
                        roundoff[free[r]][free[c]] = d;
                        roundoff[free[c]][free[r]] = d;
                    }
                }
            }
        }
        for (q, &p) in restrained.iter().enumerate() {
            k[p][p] = 3.0 + q as f64;
            for &g in &free {
                if rng.chance(0.3) {
                    let v = 0.25
                        * (rng.unit() - 0.5)
                        * pow2(-self.e[free.iter().position(|&x| x == g).unwrap()]);
                    k[p][g] = v;
                    k[g][p] = v;
                }
            }
        }
        let bc: Vec<(usize, f64)> = restrained
            .iter()
            .map(|&p| {
                (
                    p,
                    if rng.chance(0.5) {
                        0.0
                    } else {
                        rng.unit() - 0.5
                    },
                )
            })
            .collect();
        let f: Vec<f64> = (0..total).map(|_| rng.unit() - 0.5).collect();
        let evidence = self
            .skew
            .then(|| (roundoff, vec![vec![0_usize; total]; total]));
        Case {
            k,
            f,
            free,
            bc,
            evidence,
        }
    }
}

/// A scaled diagonal in [1, 4) with a random mantissa.
fn diagonal(rng: &mut Rng) -> f64 {
    1.0 + 3.0 * rng.unit()
}

/// Kinds of coupling for `random_spec`.
#[derive(Clone, Copy, Debug)]
enum Kind {
    /// |a_ij| < 0.45 min(a_ii, a_jj): no pair is a witness.
    NoWitness,
    /// As `NoWitness`, with witnesses planted at the given pairs.
    Planted,
    /// Couplings up to 1.2 (a_ii + a_jj)/2 in magnitude: witnesses anywhere.
    Wild,
}

fn random_spec(
    rng: &mut Rng,
    n: usize,
    kind: Kind,
    plant: &[(usize, usize, f64)],
    density: f64,
) -> Vec<Vec<f64>> {
    let s: Vec<f64> = (0..n).map(|_| diagonal(rng)).collect();
    let mut a = vec![vec![0.0; n]; n];
    for i in 0..n {
        a[i][i] = s[i];
        for j in 0..i {
            let value = if !rng.chance(density) {
                if rng.chance(0.2) {
                    -0.0
                } else {
                    0.0
                }
            } else {
                let sign = if rng.chance(0.5) { 1.0 } else { -1.0 };
                let bound = match kind {
                    Kind::NoWitness | Kind::Planted => 0.45 * s[i].min(s[j]),
                    Kind::Wild => 1.2 * (s[i] + s[j]) / 2.0,
                };
                sign * bound * (0.05 + 0.95 * rng.unit())
            };
            a[i][j] = value;
            a[j][i] = value;
        }
    }
    for &(i, j, sign) in plant {
        // A witness: |a_ij| beyond (a_ii + a_jj)/2 by a clear margin.
        let value = sign * (s[i] + s[j]) / 2.0 * (1.0 + 0.01 + 0.5 * rng.unit());
        a[i][j] = value;
        a[j][i] = value;
    }
    a
}

fn exponents(rng: &mut Rng, n: usize) -> Vec<i32> {
    (0..n).map(|_| rng.below(41) as i32 - 20).collect()
}

fn run_spec(rng: &mut Rng, spec: &Spec, context: &str) -> Outcome {
    let case = spec.build(rng);
    let s = case.system();
    let prepared = prepare_structural(&s).unwrap_or_else(|e| panic!("{context}: prepare {e:?}"));
    assert_eq!(prepared.scale_exponents, spec.e, "{context}: exponents");
    check(&prepared, context)
}

/// The global DOFs a witness's direction names (its nonzero entries).
fn witness_dofs(outcome: &Outcome) -> Vec<usize> {
    match outcome {
        Ok(Some(StructuralError::NegativeEnergy { direction, .. })) => direction
            .iter()
            .enumerate()
            .filter(|(_, v)| **v != 0.0)
            .map(|(k, _)| k)
            .collect(),
        _ => Vec::new(),
    }
}

// ------------------------------------------------------------------ T3: differential

#[test]
fn kf2_reference_verifier_is_todays_verifier() {
    // The reference copy must be today's verifier: compare it with the
    // product's `verify_negative_direction` on arbitrary directions.
    let mut rng = Rng(0x4B46_3201);
    for n in [2, 3, 5, 8, 13] {
        for kind in [Kind::NoWitness, Kind::Wild] {
            let a = random_spec(&mut rng, n, kind, &[], 0.6);
            let spec = Spec {
                a,
                e: exponents(&mut rng, n),
                prescribed: rng.below(3),
                shuffle: true,
                skew: rng.chance(0.5),
            };
            let case = spec.build(&mut rng);
            let s = case.system();
            let prepared = prepare_structural(&s).unwrap();
            for _ in 0..8 {
                let direction: Vec<f64> = (0..n)
                    .map(|_| {
                        if rng.chance(0.3) {
                            0.0
                        } else {
                            rng.unit() - 0.5
                        }
                    })
                    .collect();
                let reference = reference_verify_negative_direction(&prepared, &direction);
                let product = verify_negative_direction(&prepared, &direction);
                assert!(
                    same(&reference, &product),
                    "n={n}: {reference:?} {product:?}"
                );
            }
        }
    }
}

#[test]
fn kf2_witness_position_first_last_middle_none_and_several() {
    let mut rng = Rng(0x4B46_3202);
    for n in [2, 3, 4, 5, 8, 13, 21, 34, 48, 64] {
        let last = (n - 1, n - 2);
        let middle = (n / 2 + usize::from(n > 2), n / 4);
        let mut plans: Vec<(&str, Vec<(usize, usize, f64)>)> = vec![
            ("none", vec![]),
            ("first", vec![(1, 0, 1.0)]),
            ("first-negative", vec![(1, 0, -1.0)]),
            ("last", vec![(last.0, last.1, 1.0)]),
            ("last-negative", vec![(last.0, last.1, -1.0)]),
        ];
        if n >= 4 {
            plans.push(("middle", vec![(middle.0, middle.1, -1.0)]));
            // Several witnesses: the first in row-major order wins.
            plans.push((
                "several",
                vec![
                    (n - 1, 0, 1.0),
                    (middle.0, middle.1, 1.0),
                    (last.0, last.1, -1.0),
                ],
            ));
        }
        for (name, plant) in plans {
            for density in [0.0, 0.3, 1.0] {
                let kind = if plant.is_empty() {
                    Kind::NoWitness
                } else {
                    Kind::Planted
                };
                let a = random_spec(&mut rng, n, kind, &plant, density);
                let spec = Spec {
                    a,
                    e: exponents(&mut rng, n),
                    prescribed: rng.below(3),
                    shuffle: rng.chance(0.5),
                    skew: rng.chance(0.3),
                };
                let context = format!("n={n} {name} density={density}");
                let outcome = run_spec(&mut rng, &spec, &context);
                if plant.is_empty() {
                    assert!(matches!(outcome, Ok(None)), "{context}");
                } else {
                    assert!(matches!(outcome, Ok(Some(_))), "{context}");
                }
            }
        }
    }
    // No witness at the largest sizes (the reference visits every pair).
    for n in [80, 100] {
        let a = random_spec(&mut rng, n, Kind::NoWitness, &[], 0.2);
        let spec = Spec {
            a,
            e: exponents(&mut rng, n),
            prescribed: 2,
            shuffle: true,
            skew: false,
        };
        let outcome = run_spec(&mut rng, &spec, &format!("n={n} none"));
        assert!(matches!(outcome, Ok(None)));
    }
}

#[test]
fn kf2_witness_across_zero_couplings_and_both_signs() {
    // The witness pair's row and column hold +0.0 and -0.0 couplings before
    // and after it; the witness coupling takes both signs.
    let mut rng = Rng(0x4B46_3203);
    for n in [4, 6, 9] {
        for &sign in &[1.0, -1.0] {
            for &zero in &[0.0, -0.0] {
                let s: Vec<f64> = (0..n).map(|_| diagonal(&mut rng)).collect();
                let mut a = vec![vec![zero; n]; n];
                for i in 0..n {
                    a[i][i] = s[i];
                }
                let (wi, wj) = (n - 2, 1);
                let value = sign * (s[wi] + s[wj]) * 0.6;
                a[wi][wj] = value;
                a[wj][wi] = value;
                let spec = Spec {
                    a,
                    e: exponents(&mut rng, n),
                    prescribed: 1,
                    shuffle: true,
                    skew: false,
                };
                let context = format!("n={n} sign={sign} zero={zero:?}");
                let outcome = run_spec(&mut rng, &spec, &context);
                assert_eq!(witness_dofs(&outcome).len(), 2, "{context}");
            }
        }
    }
}

#[test]
fn kf2_randomized_systems_match_the_reference() {
    let mut rng = Rng(0x4B46_3204);
    let mut witnesses = 0;
    let mut nones = 0;
    for round in 0..240 {
        let n = 2 + rng.below(if round % 8 == 0 { 63 } else { 20 });
        let kind = match round % 3 {
            0 => Kind::NoWitness,
            1 => Kind::Wild,
            _ => Kind::Planted,
        };
        let plant: Vec<(usize, usize, f64)> = if matches!(kind, Kind::Planted) {
            (0..1 + rng.below(3))
                .map(|_| {
                    let i = 1 + rng.below(n - 1);
                    let j = rng.below(i);
                    (i, j, if rng.chance(0.5) { 1.0 } else { -1.0 })
                })
                .collect()
        } else {
            Vec::new()
        };
        let density = [0.1, 0.4, 0.9][rng.below(3)];
        let a = random_spec(&mut rng, n, kind, &plant, density);
        let spec = Spec {
            a,
            e: exponents(&mut rng, n),
            prescribed: rng.below(4),
            shuffle: rng.chance(0.7),
            skew: rng.chance(0.3),
        };
        match run_spec(&mut rng, &spec, &format!("round {round} n={n} {kind:?}")) {
            Ok(Some(_)) => witnesses += 1,
            Ok(None) => nones += 1,
            Err(e) => panic!("round {round}: unexpected error {e:?}"),
        }
    }
    eprintln!("KF2 randomized: {witnesses} witnesses, {nones} none");
    assert!(
        witnesses > 60 && nones > 60,
        "{witnesses} witnesses, {nones} none"
    );
}

// ------------------------------------------------------------------ edge ties

/// V's energy sums of a 2×2 pair's four terms, in the given order (the terms
/// are exact: each is ±1 × a cell × ±1). Independent of the product code.
fn local_verdict(terms: &[f64]) -> (f64, f64, bool) {
    let mut energy = 0.0;
    let mut magnitude = 0.0;
    for &t in terms {
        energy = energy + t;
        magnitude = magnitude + t.abs();
    }
    let allowance = 64.0 * gamma(3 * terms.len() + 2) * magnitude;
    (energy, allowance, energy < -allowance)
}

/// Exact ties energy == -allowance on K = [[1, t], [t, b]] (plan §8):
/// 64·gamma(14) = 2^-96 (7·2^50 + 12), so fl(64·gamma(14)·M) lands on the
/// energy's 2^-52 grid for M = mu·2^-50 with q = round(...)·2^-42 ≡ 0 mod 8;
/// t = (4mu + q) 2^-54 and b = (4mu - q) 2^-53 - 1 keep every sum exact.
fn exact_ties() -> Vec<(f64, f64)> {
    assert_eq!(
        64.0 * gamma(14),
        (7.0 * pow2(50) + 12.0) * pow2(-96),
        "64·gamma(14)"
    );
    let g: u128 = 7 * (1 << 50) + 12;
    let mut ties = Vec::new();
    for q in (1792_u128..2048).step_by(8) {
        let mu0 = (q << 94) / g;
        for mu in mu0 - 3..=mu0 + 3 {
            if mu % 2 != 0 || mu < (1 << 52) || mu >= (1 << 53) {
                continue;
            }
            let t = (4 * mu + q) as f64 * pow2(-54);
            let b = (4 * mu - q) as f64 * pow2(-53) - 1.0;
            assert_eq!((t * pow2(54)) as u128, 4 * mu + q, "t exact");
            if !(1.0..4.0).contains(&b) {
                continue;
            }
            let (energy, allowance, _) = local_verdict(&[1.0, -t, -t, b]);
            if energy == -allowance {
                ties.push((t, b));
            }
        }
    }
    ties
}

#[test]
fn kf2_ties_at_the_allowance_edge() {
    let ties = exact_ties();
    eprintln!("KF2 exact ties: {}", ties.len());
    assert!(!ties.is_empty(), "no exact tie found");
    for &(t, b) in &ties {
        // The tie itself is not a witness; b one ulp below is; one ulp above is not.
        for (bb, witness) in [(b.next_down(), true), (b, false), (b.next_up(), false)] {
            let case = Case::plain(vec![vec![1.0, t], vec![t, bb]]);
            let s = case.system();
            let prepared = prepare_structural(&s).unwrap();
            assert_eq!(prepared.scale_exponents, vec![0, 0]);
            let outcome = check(&prepared, &format!("tie t={t:e} b={bb:e}"));
            assert_eq!(matches!(outcome, Ok(Some(_))), witness, "b={bb:e}");
        }
    }
    // A tie at the first pair, then its one-below witness later, across zero
    // couplings: 4×4 blocks [[1, t], [t, b]] and [[1, t], [t, b - ulp]].
    let (t, b) = ties[0];
    let mut k = vec![vec![0.0; 4]; 4];
    k[0][0] = 1.0;
    k[1][1] = b;
    k[1][0] = t;
    k[0][1] = t;
    k[2][2] = 1.0;
    k[3][3] = b.next_down();
    k[3][2] = t;
    k[2][3] = t;
    let case = Case::plain(k);
    let s = case.system();
    let prepared = prepare_structural(&s).unwrap();
    let outcome = check(&prepared, "tie then witness");
    assert_eq!(witness_dofs(&outcome), vec![2, 3]);
}

#[test]
fn kf2_rounding_family_discriminates_the_cell_order() {
    // Pairs at the verdict's edge whose sums round: a in [1, 2), b in [2, 4)
    // with random mantissas, and the coupling scanned over 128 ulps around
    // the crossing. Self-check: the family must contain pairs whose verdict in
    // the reversed cell order differs from the verifier's order.
    let mut rng = Rng(0x4B46_3205);
    let g = 64.0 * gamma(14);
    let mut differing = 0;
    for _ in 0..96 {
        let a = 1.0 + rng.unit();
        let b = 2.0 + 2.0 * rng.unit();
        let mut k = (a + b) * (1.0 + g) / (2.0 * (1.0 - g));
        for _ in 0..64 {
            k = k.next_down();
        }
        for _ in 0..128 {
            let (_, _, forward) = local_verdict(&[a, -k, -k, b]);
            let (_, _, reversed) = local_verdict(&[b, -k, -k, a]);
            differing += usize::from(forward != reversed);
            let case = Case::plain(vec![vec![a, k], vec![k, b]]);
            let s = case.system();
            let prepared = prepare_structural(&s).unwrap();
            assert_eq!(prepared.scale_exponents, vec![0, 0]);
            let outcome = check(&prepared, &format!("edge a={a:e} b={b:e} k={k:e}"));
            assert_eq!(matches!(outcome, Ok(Some(_))), forward);
            k = k.next_up();
        }
    }
    eprintln!(
        "KF2 rounding family: {differing} of 12288 pairs change verdict in the reversed order"
    );
    assert!(
        differing > 0,
        "the family does not discriminate the cell order"
    );
}

// ------------------------------------------------------------------ errors

#[test]
fn kf2_errors_in_a_pair_before_and_after_a_witness() {
    // Energy overflow: [[1, 1e308], [1e308, 1]] (the energy passes -1e308 -
    // 1e308 = -inf at cell (i, j)); product underflow: a subnormal coupling at
    // exponent sum 0.
    for (name, bad) in [("overflow", 1e308), ("underflow", 5e-324)] {
        for witness_first in [true, false] {
            let mut k = vec![vec![0.0; 4]; 4];
            for (d, row) in k.iter_mut().enumerate() {
                row[d] = 1.0;
            }
            let (w, e) = if witness_first {
                ((1, 0), (3, 2))
            } else {
                ((3, 2), (1, 0))
            };
            k[w.0][w.1] = 3.0;
            k[w.1][w.0] = 3.0;
            k[e.0][e.1] = bad;
            k[e.1][e.0] = bad;
            let case = Case::plain(k);
            let s = case.system();
            let prepared = prepare_structural(&s).unwrap();
            let context = format!("{name} witness_first={witness_first}");
            let outcome = check(&prepared, &context);
            if witness_first {
                assert_eq!(witness_dofs(&outcome), vec![0, 1], "{context}");
            } else {
                let message = if bad > 1.0 {
                    "arithmetic outside normal range"
                } else {
                    "product overflow or underflow"
                };
                assert_eq!(outcome, Err(StructuralError::Range(message)), "{context}");
            }
        }
    }
}

// ------------------------------------------------------------------ corrupted

fn diagonal_case(values: &[f64]) -> Case {
    let n = values.len();
    let mut k = vec![vec![0.0; n]; n];
    for (i, &v) in values.iter().enumerate() {
        k[i][i] = v;
    }
    Case::plain(k)
}

#[test]
fn kf2_corrupted_shapes_and_sources_match_the_reference() {
    let base = Case::plain(vec![vec![1.0, 2.0], vec![2.0, 1.0]]);
    let s = base.system();
    // Shape corruptions: validation refuses before any pair, without a panic.
    let corruptions: [fn(&mut PreparedSystem<'_>); 4] = [
        |p| p.matrix[0].clear(),
        |p| p.scale_exponents.clear(),
        |p| p.matrix[1][0] = 7.0,
        |p| p.rhs.clear(),
    ];
    for (k, corrupt) in corruptions.iter().enumerate() {
        let mut prepared = prepare_structural(&s).unwrap();
        corrupt(&mut prepared);
        let outcome = check(&prepared, &format!("shape corruption {k}"));
        assert!(matches!(outcome, Err(StructuralError::InvalidInput(_))));
    }
    let mut prepared = prepare_structural(&s).unwrap();
    let wrong = Case {
        free: vec![0],
        ..Case::plain(vec![vec![1.0, 2.0], vec![2.0, 1.0]])
    };
    let ws = wrong.system();
    prepared.source = &ws;
    assert!(check(&prepared, "wrong free map").is_err());
}

#[test]
fn kf2_zero_coupling_pairs_are_visited() {
    // A source with a negative diagonal behind zero couplings (+0.0 and
    // -0.0): the witness is a zero-coupling pair, with the sign rule's -1.
    for zero in [0.0, -0.0] {
        let mut clean = diagonal_case(&[2.0, 3.0, 5.0]);
        clean.k[1][0] = zero;
        clean.k[0][1] = zero;
        let s = clean.system();
        let mut prepared = prepare_structural(&s).unwrap();
        let bad = diagonal_case(&[2.0, -3.0, 5.0]);
        let bs = bad.system();
        prepared.source = &bs;
        let outcome = check(
            &prepared,
            &format!("negative source diagonal, zero {zero:?}"),
        );
        match outcome {
            Ok(Some(StructuralError::NegativeEnergy { direction, .. })) => {
                assert_eq!(direction, vec![-1.0, 1.0, 0.0]);
            }
            other => panic!("expected the zero-coupling witness, got {other:?}"),
        }
    }
    // An exponent edit that makes a zero-coupling pair's diagonal overflow.
    let clean = diagonal_case(&[2.0, 3.0, 5.0]);
    let s = clean.system();
    let mut prepared = prepare_structural(&s).unwrap();
    prepared.scale_exponents[2] = 600;
    let outcome = check(&prepared, "zero-coupling radix overflow");
    assert_eq!(
        outcome,
        Err(StructuralError::Range("radix scaling loses normal range"))
    );
}

#[test]
fn kf2_first_error_depends_on_the_cell_order() {
    // Pair (1, 0) with a zero coupling: cell (0,0) fails in radix_scale (an
    // exponent edit) and cell (1,1) in checked_product (a subnormal source
    // diagonal at exponent 0). The verifier's order reaches (0,0) first.
    let clean = diagonal_case(&[2.0, 5e-324]);
    let s = clean.system();
    let mut prepared = prepare_structural(&s).unwrap();
    assert_eq!(prepared.scale_exponents, vec![0, 537]);
    prepared.scale_exponents = vec![600, 0];
    let outcome = check(&prepared, "two failing cells");
    assert_eq!(
        outcome,
        Err(StructuralError::Range("radix scaling loses normal range"))
    );
}

#[test]
fn kf2_mapping_error_after_the_verdict() {
    // Zeroed prepared diagonals and balanced exponents (+1100, -1100): the
    // couplings evaluate at exponent 0 and make a witness; the verifier's
    // mapping of the direction then leaves the normal range.
    let clean = Case::plain(vec![vec![1.0, 3.0], vec![3.0, 1.0]]);
    let s = clean.system();
    let mut prepared = prepare_structural(&s).unwrap();
    prepared.matrix[0][0] = 0.0;
    prepared.matrix[1][1] = 0.0;
    prepared.scale_exponents = vec![1100, -1100];
    let outcome = check(&prepared, "mapping error");
    assert_eq!(
        outcome,
        Err(StructuralError::Range("radix scaling loses normal range"))
    );
}

#[test]
fn kf2_cells_follow_the_prepared_matrix_not_the_source() {
    // A source coupling of 7 behind a zero prepared coupling is never
    // evaluated; a source of zeros behind nonzero prepared diagonals gives a
    // pair of magnitude 0 (energy 0 against allowance 0: no witness).
    let clean = diagonal_case(&[2.0, 3.0]);
    let s = clean.system();
    let mut prepared = prepare_structural(&s).unwrap();
    let coupled = Case::plain(vec![vec![2.0, 7.0], vec![7.0, 3.0]]);
    let cs = coupled.system();
    prepared.source = &cs;
    assert_eq!(check(&prepared, "hidden source coupling"), Ok(None));
    let zeros = Case::plain(vec![vec![0.0, 0.0], vec![0.0, 0.0]]);
    let zs = zeros.system();
    prepared.source = &zs;
    assert_eq!(check(&prepared, "magnitude 0"), Ok(None));
}

// ------------------------------------------------------------------ T5, T6: cost

/// R1's SEC_N properties (E 200 GPa, G 80 GPa, OD 0.2 m, ID 0.18 m; J = 2I).
fn sec_n() -> FrameSection {
    let (od, id) = (0.2_f64, 0.18_f64);
    let area = std::f64::consts::PI * (od * od - id * id) / 4.0;
    let inertia = std::f64::consts::PI * (od.powi(4) - id.powi(4)) / 64.0;
    FrameSection::new(200e9, 80e9, area, inertia, inertia, 2.0 * inertia).unwrap()
}

/// A cantilever chain of `members` 3 m frames along x (N0 fixed), as
/// RF-LARGE-CHAIN-AX. Axis-aligned, so the assembled matrix is exactly
/// symmetric (no symmetry evidence is needed). Positive definite: no witness.
fn chain(members: usize) -> Case {
    let point = |i: usize| [3.0 * i as f64, 0.0, 0.0];
    let frames: Vec<FrameElement> = (0..members)
        .map(|i| {
            FrameElement::new(
                FrameNode::new(i, point(i)).unwrap(),
                FrameNode::new(i + 1, point(i + 1)).unwrap(),
                sec_n(),
                [0.0, 1.0, 0.0],
            )
            .unwrap()
        })
        .collect();
    let k = assemble_global_stiffness(members + 1, &frames).unwrap();
    let total = 6 * (members + 1);
    let mut f = vec![0.0; total];
    f[total - 5] = -1000.0;
    Case {
        k,
        f,
        free: (6..total).collect(),
        bc: (0..6).map(|d| (d, 0.0)).collect(),
        evidence: None,
    }
}

/// The counted search on a chain: every pair visited, two cells per pair
/// plus two more per stored coupling, no verification.
fn chain_counts(prepared: &PreparedSystem<'_>) -> (usize, usize, usize, usize) {
    let (witness, visited, evaluated, verified) = negative_pair_witness_counted(prepared).unwrap();
    assert!(witness.is_none());
    let n = prepared.matrix.len();
    let stored: usize = (0..n)
        .map(|i| (0..i).filter(|&j| prepared.matrix[i][j] != 0.0).count())
        .sum();
    assert_eq!(visited, n * (n - 1) / 2);
    assert_eq!(evaluated, 2 * visited + 2 * stored);
    assert_eq!(verified, 0);
    (n, visited, evaluated, stored)
}

#[test]
fn kf2_cost_counts_on_a_100_member_chain() {
    // T5 (debug): RF-LARGE-CHAIN-n00100-AX's matrix, n = 600, no factor.
    let case = chain(100);
    let s = case.system();
    let prepared = prepare_structural(&s).unwrap();
    let (n, visited, _, _) = chain_counts(&prepared);
    assert_eq!((n, visited), (600, 179_700));
    assert!(same(&negative_pair_witness(&prepared), &Ok(None)));
}

#[test]
#[ignore = "KF2 T6: release-only cost test (n = 6,006); run with --release -- --ignored"]
fn kf2_cost_bound_at_6006_dofs() {
    // T6: a 1,001-member chain (N = 6,012, n = 6,006), prepared, no
    // factor. Bound: the witness ends within 30 s in release.
    let case = chain(1001);
    let s = case.system();
    let prepared = prepare_structural(&s).unwrap();
    let start = std::time::Instant::now();
    let (n, visited, evaluated, stored) = chain_counts(&prepared);
    let seconds = start.elapsed().as_secs_f64();
    eprintln!(
        "KF2 T6: n={n} visited={visited} evaluated={evaluated} stored_couplings={stored} witness_seconds={seconds:.3}"
    );
    assert_eq!(n, 6006);
    assert!(seconds < 30.0, "witness took {seconds} s");
}
