//! RF source-method design §2 and §8 (RF-1 to RF-8): exact free blocks of
//! order 3 and 4 on the functional entry and its replay, the legacy entries'
//! unchanged order-2 scope, and the retained witness.
//!
//! The exact solutions are checked against an independent oracle: Cramer's
//! rule over arbitrary-precision integers, with Leibniz determinants of the
//! whole free system, formed from the declared contributions' binary64 bits.
//! It calls none of the producer's expansion arithmetic.
use super::*;
use oracle::Int;

const ID: &str = "block order source";

/// A source built from its declared contributions: each listed entry (row,
/// column) with row <= column, mirrored below the diagonal in the same order.
struct Case {
    entries: Vec<Vec<Vec<f64>>>,
    stiffness: Vec<Vec<f64>>,
    contributions: Vec<StiffnessContribution>,
    force: Vec<f64>,
    free: Vec<usize>,
    fixed: Vec<(usize, f64)>,
}
impl Case {
    fn new(
        n: usize,
        upper: &[((usize, usize), &[f64])],
        force: &[f64],
        free: &[usize],
        fixed: &[(usize, f64)],
    ) -> Self {
        let mut entries = vec![vec![Vec::new(); n]; n];
        let mut stiffness = vec![vec![0.0; n]; n];
        let mut contributions = Vec::new();
        for &((row, col), values) in upper {
            assert!(row <= col);
            for (r, c) in [(row, col), (col, row)].into_iter().take(if row == col { 1 } else { 2 }) {
                for &value in values {
                    entries[r][c].push(value);
                    stiffness[r][c] += value;
                    contributions.push(StiffnessContribution { row: r, col: c, value });
                }
            }
        }
        Self {
            entries,
            stiffness,
            contributions,
            force: force.to_vec(),
            free: free.to_vec(),
            fixed: fixed.to_vec(),
        }
    }
    /// tridiag(b, a, b), every DOF free.
    fn tridiagonal(a: &[f64], b: &[f64], force: &[f64]) -> Self {
        let mut upper: Vec<((usize, usize), &[f64])> = Vec::new();
        for i in 0..a.len() {
            upper.push(((i, i), std::slice::from_ref(&a[i])));
            if i + 1 < a.len() {
                upper.push(((i, i + 1), std::slice::from_ref(&b[i])));
            }
        }
        let free: Vec<usize> = (0..a.len()).collect();
        Self::new(a.len(), &upper, force, &free, &[])
    }
    fn system(&self) -> StructuralSystem<'_> {
        StructuralSystem {
            stiffness: &self.stiffness,
            force: &self.force,
            free_dofs: &self.free,
            prescribed: &self.fixed,
            contributions: Some(&self.contributions),
            contribution_groups: None,
            symmetry: None,
        }
    }
    fn dofs(&self) -> usize {
        self.force.len()
    }
}

/// A 5-DOF source: DOF 0 prescribed at 0.5, DOFs 1 to 3 one block of order 3
/// and DOF 4 one of order 1, both coupled to DOF 0.
fn case_order_three() -> Case {
    Case::new(
        5,
        &[
            ((0, 0), &[10.0]),
            ((0, 1), &[-2.0]),
            ((0, 2), &[1.0]),
            ((0, 4), &[3.0]),
            ((1, 1), &[7.0]),
            ((1, 2), &[3.0]),
            ((1, 3), &[-1.0]),
            ((2, 2), &[9.0]),
            ((2, 3), &[2.0]),
            ((3, 3), &[6.0]),
            ((4, 4), &[5.0]),
        ],
        &[1.5, 2.0, -1.0, 0.25, 4.0],
        &[1, 2, 3, 4],
        &[(0, 0.5)],
    )
}
/// An 8-DOF source with distinct dyadic entries: DOFs 0 to 3 one block of
/// order 4, found in the permuted order [2, 0, 3, 1]; DOFs 6 and 7 one of
/// order 2; DOFs 4 and 5 prescribed at nonzero values, coupled to both.
fn case_order_four() -> Case {
    Case::new(
        8,
        &[
            ((0, 0), &[12.0]),
            ((0, 1), &[2.0]),
            ((0, 2), &[-3.0]),
            ((0, 3), &[1.0]),
            ((0, 4), &[1.5]),
            ((1, 1), &[10.0]),
            ((1, 2), &[4.0]),
            ((1, 3), &[-2.0]),
            ((2, 2), &[11.0]),
            ((2, 3), &[3.0]),
            ((2, 4), &[-2.0]),
            ((3, 3), &[9.0]),
            ((3, 5), &[0.5]),
            ((4, 4), &[8.0]),
            ((4, 5), &[1.0]),
            ((5, 5), &[6.0]),
            ((5, 6), &[2.5]),
            ((6, 6), &[5.0]),
            ((6, 7), &[-1.5]),
            ((7, 7), &[4.0]),
        ],
        &[1.0, -2.0, 0.5, 3.0, 0.0, 0.0, -1.0, 2.0],
        &[2, 7, 0, 3, 1, 6],
        &[(4, -0.25), (5, 0.75)],
    )
}
/// Order 3 with multi-term exact entries (each a binary64 value plus a tail
/// that does not fit beside it), DOF 3 prescribed at 0.375.
fn case_order_three_tails() -> Case {
    let p2 = |k: i32| 2.0_f64.powi(k);
    Case::new(
        4,
        &[
            ((0, 0), &[96.0, p2(-50)]),
            ((0, 1), &[-20.0, p2(-52)]),
            ((0, 2), &[7.5, -p2(-49)]),
            ((0, 3), &[-9.0, p2(-54)]),
            ((1, 1), &[80.0, -3.0 * p2(-51)]),
            ((1, 2), &[-12.25, p2(-53)]),
            ((2, 2), &[64.0, p2(-48)]),
            ((2, 3), &[5.0]),
            ((3, 3), &[40.0]),
        ],
        &[1.0, -2.5, 0.75 + p2(-40), 3.0],
        &[0, 1, 2],
        &[(3, 0.375)],
    )
}
/// Order 4 with multi-term exact entries, DOF 4 prescribed at -0.125.
fn case_order_four_tails() -> Case {
    let p2 = |k: i32| 2.0_f64.powi(k);
    Case::new(
        5,
        &[
            ((0, 0), &[150.0, p2(-46)]),
            ((0, 1), &[-30.0, p2(-50)]),
            ((0, 2), &[12.5, -p2(-52)]),
            ((0, 3), &[-7.0, p2(-51)]),
            ((0, 4), &[6.0]),
            ((1, 1), &[140.0, -p2(-47)]),
            ((1, 2), &[-25.75, p2(-49)]),
            ((1, 3), &[9.5, p2(-55)]),
            ((2, 2), &[130.0, 3.0 * p2(-50)]),
            ((2, 3), &[-40.25, -p2(-53)]),
            ((3, 3), &[120.0, p2(-48)]),
            ((3, 4), &[-11.0, p2(-56)]),
            ((4, 4), &[50.0]),
        ],
        &[2.0, -1.5, 0.25, 4.0, -3.0],
        &[0, 1, 2, 3],
        &[(4, -0.125)],
    )
}

fn budget() -> AttemptBudget {
    AttemptBudget::new(Limits {
        operations: 200_000_000,
        ..Limits::default()
    })
}
fn functional(case: &Case, budget: &mut AttemptBudget) -> Result<Context, Error> {
    Context::prepare_with_budget(&case.system(), ID, ForceBasis::DeclaredVector, &[], budget).result
}
fn legacy(case: &Case) -> Result<Context, Error> {
    Context::new(&case.system(), ID, ForceBasis::DeclaredVector, Limits::default())
}
fn orders(c: &Context) -> Vec<usize> {
    c.blocks.iter().map(Vec::len).collect()
}
fn declared(label: &str, terms: &[(usize, f64)]) -> FunctionalDescriptor {
    FunctionalDescriptor {
        key: FunctionalKey {
            case_id: "block order".into(),
            quantity: FunctionalQuantity::DeclaredAffine {
                label: label.into(),
            },
            unit: FunctionalUnit::Dimensionless,
            convention: FunctionalConvention::DeclaredAffine,
        },
        offset: vec![],
        terms: terms
            .iter()
            .map(|&(dof, value)| AffineTerm {
                dof,
                products: vec![vec![value]],
            })
            .collect(),
    }
}
/// Weight 1 on every free DOF, and a weighted form over the same DOFs.
fn plan(case: &Case) -> Vec<FunctionalDescriptor> {
    let all: Vec<(usize, f64)> = case.free.iter().map(|&i| (i, 1.0)).collect();
    let weighted: Vec<(usize, f64)> = case
        .free
        .iter()
        .enumerate()
        .map(|(k, &i)| (i, k as f64 - 1.5))
        .collect();
    vec![declared("sum", &all), declared("weighted", &weighted)]
}
/// Preparation through retention on one ledger, every DOF leaf projected.
fn retain(case: &Case, ds: &[FunctionalDescriptor], budget: &mut AttemptBudget) -> RetainedFunctionalSet {
    let c = functional(case, budget).unwrap();
    let r = c.solve_with_budget(budget).result.unwrap();
    let p = FunctionalPlan::new(&c, ds, budget).result.unwrap();
    let set = r.evaluate_functionals(&p, budget).result.unwrap();
    let mut leaves = Vec::new();
    for i in 0..case.dofs() {
        for quantity in [Quantity::Displacement, Quantity::Reaction] {
            leaves.push(r.project_with_budget(quantity, i, 1e-9, budget).result.unwrap());
        }
    }
    let projections: Vec<_> = (0..set.len())
        .map(|i| set.project(i, 1e-9, budget).result.unwrap())
        .collect();
    set.retain(&[], &leaves, &projections, budget).result.unwrap()
}
fn replay(
    set: &RetainedFunctionalSet,
    case: &Case,
    ds: &[FunctionalDescriptor],
    budget: &mut AttemptBudget,
) -> Result<FunctionalReplayCheck, Error> {
    set.replay_against(&case.system(), ID, ForceBasis::DeclaredVector, &[], ds, None, &[], 1e-9, budget)
        .result
}

/// The independent oracle: arbitrary-precision integers and exact rationals.
/// Shared with `congruence_tests` (RF design §3.3).
pub(crate) mod oracle {
    use std::cmp::Ordering;

    /// A signed integer, little-endian base-2^32 limbs, normalized: no high
    /// zero limb, and zero is non-negative.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct Int {
        negative: bool,
        limbs: Vec<u32>,
    }
    fn trim(mut v: Vec<u32>) -> Vec<u32> {
        while v.last() == Some(&0) {
            v.pop();
        }
        v
    }
    fn compare(a: &[u32], b: &[u32]) -> Ordering {
        a.len().cmp(&b.len()).then_with(|| a.iter().rev().cmp(b.iter().rev()))
    }
    fn add(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut out = Vec::with_capacity(a.len().max(b.len()) + 1);
        let mut carry = 0u64;
        for i in 0..a.len().max(b.len()) {
            let s = carry + u64::from(*a.get(i).unwrap_or(&0)) + u64::from(*b.get(i).unwrap_or(&0));
            out.push(s as u32);
            carry = s >> 32;
        }
        out.push(carry as u32);
        trim(out)
    }
    /// a - b for |a| >= |b|.
    fn subtract(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut out = Vec::with_capacity(a.len());
        let mut borrow = 0i64;
        for (i, &x) in a.iter().enumerate() {
            let mut d = i64::from(x) - borrow - i64::from(*b.get(i).unwrap_or(&0));
            borrow = i64::from(d < 0);
            if d < 0 {
                d += 1 << 32;
            }
            out.push(d as u32);
        }
        trim(out)
    }
    fn multiply(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut out = vec![0u32; a.len() + b.len() + 1];
        for (i, &x) in a.iter().enumerate() {
            let mut carry = 0u64;
            for (j, &y) in b.iter().enumerate() {
                let t = u64::from(out[i + j]) + u64::from(x) * u64::from(y) + carry;
                out[i + j] = t as u32;
                carry = t >> 32;
            }
            let mut k = i + b.len();
            while carry > 0 {
                let t = u64::from(out[k]) + carry;
                out[k] = t as u32;
                carry = t >> 32;
                k += 1;
            }
        }
        trim(out)
    }
    impl Int {
        fn signed(negative: bool, limbs: Vec<u32>) -> Self {
            let limbs = trim(limbs);
            Self {
                negative: negative && !limbs.is_empty(),
                limbs,
            }
        }
        pub(crate) fn zero() -> Self {
            Self::signed(false, Vec::new())
        }
        pub(crate) fn from_i64(v: i64) -> Self {
            let m = v.unsigned_abs();
            Self::signed(v < 0, vec![m as u32, (m >> 32) as u32])
        }
        pub(crate) fn is_zero(&self) -> bool {
            self.limbs.is_empty()
        }
        pub(crate) fn neg(&self) -> Self {
            Self::signed(!self.negative, self.limbs.clone())
        }
        pub(crate) fn add(&self, o: &Self) -> Self {
            if self.negative == o.negative {
                return Self::signed(self.negative, add(&self.limbs, &o.limbs));
            }
            match compare(&self.limbs, &o.limbs) {
                Ordering::Equal => Self::zero(),
                Ordering::Greater => Self::signed(self.negative, subtract(&self.limbs, &o.limbs)),
                Ordering::Less => Self::signed(o.negative, subtract(&o.limbs, &self.limbs)),
            }
        }
        pub(crate) fn sub(&self, o: &Self) -> Self {
            self.add(&o.neg())
        }
        pub(crate) fn mul(&self, o: &Self) -> Self {
            Self::signed(self.negative != o.negative, multiply(&self.limbs, &o.limbs))
        }
        /// self·2^bits.
        pub(crate) fn shl(&self, bits: u32) -> Self {
            let (words, rest) = ((bits / 32) as usize, bits % 32);
            let mut out = vec![0u32; words];
            let mut carry = 0u32;
            for &l in &self.limbs {
                if rest == 0 {
                    out.push(l);
                } else {
                    out.push((l << rest) | carry);
                    carry = l >> (32 - rest);
                }
            }
            out.push(carry);
            Self::signed(self.negative, out)
        }
    }

    /// x = m·2^e exactly, with m odd (or x = 0, m = 0).
    fn decompose(x: f64) -> (i64, i32) {
        assert!(x.is_finite());
        let bits = x.to_bits();
        let raw = ((bits >> 52) & 0x7ff) as i32;
        let fraction = (bits & ((1 << 52) - 1)) as i64;
        let (m, e) = if raw == 0 {
            (fraction, -1074)
        } else {
            (fraction | (1 << 52), raw - 1075)
        };
        if m == 0 {
            return (0, 0);
        }
        let tz = m.trailing_zeros();
        let m = m >> tz;
        (if bits >> 63 == 1 { -m } else { m }, e + tz as i32)
    }
    /// The smallest s >= 0 with x·2^s an integer.
    pub(crate) fn shift(x: f64) -> u32 {
        (-decompose(x).1).max(0) as u32
    }
    /// x·2^s, which must be an integer.
    pub(crate) fn scaled(x: f64, s: u32) -> Int {
        let (m, e) = decompose(x);
        if m == 0 {
            return Int::zero();
        }
        let up = e + s as i32;
        assert!(up >= 0, "{x:e}·2^{s} is not an integer");
        Int::from_i64(m).shl(up as u32)
    }
    pub(crate) fn sum_scaled(terms: &[f64], s: u32) -> Int {
        terms.iter().fold(Int::zero(), |acc, &t| acc.add(&scaled(t, s)))
    }
    /// The determinant by the Leibniz formula: the signed sum over every
    /// permutation σ of Π m[i][σ(i)] (1 for order 0).
    pub(crate) fn det(m: &[Vec<Int>]) -> Int {
        fn permutations(n: usize) -> Vec<Vec<usize>> {
            if n == 0 {
                return vec![Vec::new()];
            }
            let mut out = Vec::new();
            for p in permutations(n - 1) {
                for at in 0..n {
                    let mut q = p.clone();
                    q.insert(at, n - 1);
                    out.push(q);
                }
            }
            out
        }
        let mut total = Int::zero();
        for p in permutations(m.len()) {
            let inversions = (0..p.len())
                .flat_map(|i| (i + 1..p.len()).map(move |j| (i, j)))
                .filter(|&(i, j)| p[i] > p[j])
                .count();
            let product = (0..p.len()).fold(Int::from_i64(1), |acc, i| acc.mul(&m[i][p[i]]));
            total = if inversions % 2 == 0 {
                total.add(&product)
            } else {
                total.sub(&product)
            };
        }
        total
    }
}

/// The exact displacements and reactions, as (numerator, denominator) pairs,
/// from the declared contributions, loads and prescribed values. With S the
/// common scale making every operand x·2^S an integer (K', f', v'), the free
/// equations K_ff u = f - K_fp v become K'_ff u = g / 2^S with
/// g = 2^S f' - K'_fp v'; Cramer's rule over the whole free system gives
/// u_a = det(K'_ff with column a replaced by g) / (det K'_ff · 2^S).
struct Exact {
    u: Vec<(Int, Int)>,
    r: Vec<(Int, Int)>,
}
fn exact(case: &Case) -> Exact {
    let n = case.dofs();
    let operands = case
        .entries
        .iter()
        .flatten()
        .flatten()
        .chain(&case.force)
        .chain(case.fixed.iter().map(|(_, v)| v));
    let s = operands.map(|&x| oracle::shift(x)).max().unwrap_or(0);
    let k: Vec<Vec<Int>> = case
        .entries
        .iter()
        .map(|row| row.iter().map(|e| oracle::sum_scaled(e, s)).collect())
        .collect();
    let f: Vec<Int> = case.force.iter().map(|&x| oracle::scaled(x, s)).collect();
    let mut v: Vec<Option<Int>> = vec![None; n];
    for &(j, value) in &case.fixed {
        v[j] = Some(oracle::scaled(value, s));
    }
    let free = &case.free;
    let g: Vec<Int> = free
        .iter()
        .map(|&i| {
            let mut gi = f[i].shl(s);
            for j in 0..n {
                if let Some(vj) = &v[j] {
                    gi = gi.sub(&k[i][j].mul(vj));
                }
            }
            gi
        })
        .collect();
    let kff: Vec<Vec<Int>> = free
        .iter()
        .map(|&i| free.iter().map(|&j| k[i][j].clone()).collect())
        .collect();
    let d0 = oracle::det(&kff);
    assert!(!d0.is_zero());
    let numerators: Vec<Int> = (0..free.len())
        .map(|a| {
            let mut m = kff.clone();
            for (row, gi) in m.iter_mut().zip(&g) {
                row[a] = gi.clone();
            }
            oracle::det(&m)
        })
        .collect();
    let two_s = Int::from_i64(1).shl(s);
    let mut u = vec![(Int::zero(), Int::from_i64(1)); n];
    for (a, &i) in free.iter().enumerate() {
        u[i] = (numerators[a].clone(), d0.mul(&two_s));
    }
    for (j, vj) in v.iter().enumerate() {
        if let Some(vj) = vj {
            u[j] = (vj.clone(), two_s.clone());
        }
    }
    // r_i = Σ_j k_ij u_j - f_i over the common denominator 2^(2S)·det K'_ff.
    let r = (0..n)
        .map(|i| {
            let mut num = f[i].shl(s).mul(&d0).neg();
            for (a, &j) in free.iter().enumerate() {
                num = num.add(&k[i][j].mul(&numerators[a]));
            }
            for (j, vj) in v.iter().enumerate() {
                if let Some(vj) = vj {
                    num = num.add(&k[i][j].mul(vj).mul(&d0));
                }
            }
            (num, d0.shl(2 * s))
        })
        .collect();
    Exact { u, r }
}
/// The producer's ratio equals the exact (numerator, denominator) pair.
pub(super) fn equals(r: &Ratio, (num, den): &(Int, Int)) -> bool {
    let t = r
        .numerator_terms()
        .iter()
        .chain(r.denominator_terms())
        .map(|&x| oracle::shift(x))
        .max()
        .unwrap_or(0);
    let n = oracle::sum_scaled(r.numerator_terms(), t);
    let d = oracle::sum_scaled(r.denominator_terms(), t);
    assert!(!d.is_zero());
    n.mul(den) == num.mul(&d)
}

#[test]
fn order_four_blocks_select_and_order_five_declines() {
    // K = tridiag(-1, 2, -1): K⁻¹e₁ = (4,3,2,1)/5 at order 4, (3,2,1)/4 at 3.
    for (n, det) in [(3usize, 4), (4, 5)] {
        let mut f = vec![0.0; n];
        f[0] = 1.0;
        let case = Case::tridiagonal(&vec![2.0; n], &vec![-1.0; n - 1], &f);
        let mut budget = budget();
        let c = functional(&case, &mut budget).unwrap();
        assert_eq!(orders(&c), [n]);
        let r = c.solve_with_budget(&mut budget).result.unwrap();
        for i in 0..n {
            let u = (Int::from_i64((n - i) as i64), Int::from_i64(det));
            assert!(equals(r.displacement(i).unwrap(), &u), "order {n}: u{i}");
            assert_eq!(r.reaction(i).unwrap().sign(), Sign::Zero);
            let p = r
                .project_with_budget(Quantity::Displacement, i, 1e-9, &mut budget)
                .result
                .unwrap();
            assert_eq!(p.value(), (n - i) as f64 / det as f64);
        }
        // Selected through the functional entry: evaluated, projected,
        // retained and replayed on the attempt's one ledger.
        let ds = vec![declared("sum", &(0..n).map(|i| (i, 1.0)).collect::<Vec<_>>())];
        let retained = retain(&case, &ds, &mut budget);
        let total = (n * (n + 1) / 2) as f64 / det as f64;
        assert_eq!(retained.projections()[0].value(), total);
        assert_eq!(replay(&retained, &case, &ds, &mut budget).unwrap().blocks(), 1);
        // The legacy entry keeps order 2: it refuses both.
        assert_eq!(legacy(&case).unwrap_err(), Error::UnsupportedBlock { order: n });
    }
    let order5 = Case::tridiagonal(&[2.0; 5], &[-1.0; 4], &[1.0, 0.0, 0.0, 0.0, 0.0]);
    assert_eq!(
        functional(&order5, &mut budget()).unwrap_err(),
        Error::UnsupportedBlock { order: 5 }
    );
    // Components [4, 5]: the order-4 block is prepared, the first one above 4
    // is refused; and [5, 4] refuses at once.
    for first_four in [true, false] {
        let (a, b) = if first_four { (0, 4) } else { (5, 0) };
        let mut upper: Vec<((usize, usize), &[f64])> = Vec::new();
        for i in 0..9 {
            upper.push(((i, i), &[2.0]));
        }
        for i in (a..a + 3).chain(b..b + 4) {
            upper.push(((i, i + 1), &[-1.0]));
        }
        let case = Case::new(9, &upper, &[1.0; 9], &(0..9).collect::<Vec<_>>(), &[]);
        assert_eq!(
            functional(&case, &mut budget()).unwrap_err(),
            Error::UnsupportedBlock { order: 5 }
        );
    }
    // Orders 1 and 2 are unchanged on the legacy entry.
    let order2 = Case::tridiagonal(&[2.0, 2.0], &[-1.0], &[1.0, 0.0]);
    assert_eq!(orders(&legacy(&order2).unwrap()), [2]);
}

#[test]
fn order_four_warrant_checks_every_leading_minor() {
    // Tridiagonal leading minors: D_k = a_k·D_(k-1) - b_(k-1)²·D_(k-2).
    let refused: [(&[f64], &[f64], &str); 7] = [
        // Δ1, Δ2, Δ4 > 0 and Δ3 < 0: (2, 3, -8, 13).
        (&[2.0, 2.0, -2.0, -2.0][..], &[1.0, 1.0, 1.0][..], "Δ3 < 0 < Δ4"),
        // Δ2, Δ3 < 0 < Δ4: (1, -3, -4, 7).
        (&[1.0, 1.0, 1.0, -1.0][..], &[2.0, 1.0, 1.0][..], "Δ2 < 0 < Δ4"),
        // Δ4 < 0: (2, 3, 4, -1); Δ4 = 0: (2, 3, 4, 0).
        (&[2.0, 2.0, 2.0, 0.5][..], &[1.0, 1.0, 1.0][..], "Δ4 < 0"),
        (&[2.0, 2.0, 2.0, 0.75][..], &[1.0, 1.0, 1.0][..], "Δ4 = 0"),
        // Order 3: Δ3 < 0 (2, 3, -8); Δ3 = 0 (1, 1, 0).
        (&[2.0, 2.0, -2.0][..], &[1.0, 1.0][..], "order 3, Δ3 < 0"),
        (&[1.0, 2.0, 1.0][..], &[1.0, 1.0][..], "order 3, Δ3 = 0"),
        // Δ1 < 0 (the existing first check).
        (&[-2.0, 2.0, 2.0, 2.0][..], &[1.0, 1.0, 1.0][..], "Δ1 < 0"),
    ];
    for (a, b, what) in refused {
        let case = Case::tridiagonal(a, b, &vec![1.0; a.len()]);
        assert_eq!(functional(&case, &mut budget()).unwrap_err(), Error::NotPositiveDefinite, "{what}");
    }
    // Order 3, Δ2 < 0 < Δ3: [[1,2,0],[2,1,1],[0,1,-1]] has (1, -3, 2).
    let case = Case::new(
        3,
        &[((0, 0), &[1.0]), ((0, 1), &[2.0]), ((1, 1), &[1.0]), ((1, 2), &[1.0]), ((2, 2), &[-1.0])],
        &[1.0; 3],
        &[0, 1, 2],
        &[],
    );
    assert_eq!(functional(&case, &mut budget()).unwrap_err(), Error::NotPositiveDefinite);
    // Positive controls: every leading minor positive.
    for (a, b) in [(&[2.0, 2.0, 2.0, 1.0][..], &[1.0, 1.0, 1.0][..]), (&[2.0, 2.0, 1.0][..], &[1.0, 1.0][..])] {
        let case = Case::tridiagonal(a, b, &vec![1.0; a.len()]);
        let mut budget = budget();
        let c = functional(&case, &mut budget).unwrap();
        assert_eq!(orders(&c), [a.len()]);
        assert!(c.solve_with_budget(&mut budget).result.is_ok());
    }
}

#[test]
fn order_three_and_four_solutions_are_exact() {
    for (case, expected) in [
        (case_order_three(), vec![3, 1]),
        (case_order_four(), vec![4, 2]),
        (case_order_three_tails(), vec![3]),
        (case_order_four_tails(), vec![4]),
    ] {
        let mut budget = budget();
        let c = functional(&case, &mut budget).unwrap();
        assert_eq!(orders(&c), expected);
        let r = c.solve_with_budget(&mut budget).result.unwrap();
        let reference = exact(&case);
        for i in 0..case.dofs() {
            assert!(equals(r.displacement(i).unwrap(), &reference.u[i]), "{expected:?}: u{i}");
            assert!(equals(r.reaction(i).unwrap(), &reference.r[i]), "{expected:?}: r{i}");
        }
        // Non-vacuous: the prescribed rows carry nonzero reactions.
        assert!(case.fixed.iter().all(|&(i, _)| r.reaction(i).unwrap().sign() != Sign::Zero));
        // The legacy entry refuses the first block above order 2.
        let first = expected.iter().copied().find(|&o| o > 2).unwrap();
        assert_eq!(legacy(&case).unwrap_err(), Error::UnsupportedBlock { order: first });
    }
    // The tail cases carry multi-term exact entries into the block arithmetic.
    let case = case_order_four_tails();
    let c = functional(&case, &mut budget()).unwrap();
    assert!(c.witnesses[0].upper.as_ref().unwrap().iter().all(|e| e.len() == 2));
}

#[test]
fn order_four_block_dofs_share_one_denominator() {
    for case in [
        case_order_three(),
        case_order_four(),
        case_order_three_tails(),
        case_order_four_tails(),
    ] {
        let mut budget = budget();
        let c = functional(&case, &mut budget).unwrap();
        let r = c.solve_with_budget(&mut budget).result.unwrap();
        for (block, witness) in c.blocks.iter().zip(&c.witnesses) {
            let order = block.len();
            assert_eq!(witness.dofs(), &block[..]);
            if order <= 2 {
                assert!(witness.upper_terms().is_none() && witness.minor_terms().is_none());
                assert_eq!(witness.b_terms().is_some(), order == 2);
                continue;
            }
            assert!(witness.b_terms().is_none() && witness.c_terms().is_none());
            assert!(witness.determinant_terms().is_none());
            let upper = witness.upper_terms().unwrap();
            let minors = witness.minor_terms().unwrap();
            assert_eq!(upper.len(), order * (order + 1) / 2);
            assert_eq!(minors.len(), order - 1);
            let mut e = 0;
            for p in 0..order {
                for q in p..order {
                    assert!(same_terms(&upper[e], &c.k[block[p]][block[q]].terms));
                    e += 1;
                }
            }
            assert!(same_terms(witness.a_terms(), &upper[0]));
            // Every DOF of the block: one Δo, bit for bit.
            let delta = minors.last().unwrap();
            for &i in block {
                assert!(same_terms(r.displacement(i).unwrap().denominator_terms(), delta));
            }
        }
    }
}

#[test]
fn evaluate_refuses_a_block_with_mixed_denominators() {
    for case in [case_order_four(), case_order_three_tails()] {
        let mut budget = budget();
        let c = functional(&case, &mut budget).unwrap();
        let mut r = c.solve_with_budget(&mut budget).result.unwrap();
        let block = c.blocks.iter().find(|b| b.len() > 2).unwrap().clone();
        let weights: Vec<(usize, f64)> = block.iter().enumerate().map(|(k, &i)| (i, k as f64 + 1.0)).collect();
        let ds = [declared("block", &weights)];
        let plan = FunctionalPlan::new(&c, &ds, &mut budget).result.unwrap();
        // Positive control against the independent reference.
        let reference = exact(&case);
        let (mut num, den) = (Int::zero(), reference.u[block[0]].1.clone());
        for &(i, weight) in &weights {
            assert_eq!(reference.u[i].1, den);
            num = num.add(&reference.u[i].0.mul(&Int::from_i64(weight as i64)));
        }
        {
            let set = r.evaluate_functionals(&plan, &mut budget).result.unwrap();
            assert!(equals(set.ratio(0).unwrap(), &(num.clone(), den.clone())));
        }
        // The same value with other bits: one DOF's numerator and
        // denominator doubled. Evaluate refuses before combining.
        let i = block[1];
        let double = |x: &mut Ratio, by: f64| {
            x.numerator.terms.iter_mut().chain(&mut x.denominator.terms).for_each(|t| *t *= by)
        };
        double(&mut r.displacement[i], 2.0);
        assert!(equals(&r.displacement[i], &reference.u[i]));
        assert_eq!(
            r.evaluate_functionals(&plan, &mut budget).result.unwrap_err(),
            Error::Invalid("functional block denominators")
        );
        double(&mut r.displacement[i], 0.5);
        assert!(r.evaluate_functionals(&plan, &mut budget).result.is_ok());
    }
}

#[test]
fn order_four_replay_refuses_a_mutated_minor() {
    let mismatch = Error::Invalid("functional retained block/minor mismatch");
    for case in [case_order_four_tails(), case_order_three_tails(), case_order_four()] {
        let ds = plan(&case);
        let mut budget = budget();
        let retained = retain(&case, &ds, &mut budget);
        assert!(replay(&retained, &case, &ds, &mut budget).is_ok());
        let higher: Vec<usize> = (0..retained.response.witnesses.len())
            .filter(|&b| retained.response.witnesses[b].dofs.len() > 2)
            .collect();
        assert_eq!(higher.len(), 1);
        for b in higher {
            let order = retained.response.witnesses[b].dofs.len();
            for m in 0..order - 1 {
                let mut x = retained.clone();
                x.response.witnesses[b].minors.as_mut().unwrap()[m][0] *= 2.0;
                assert_eq!(replay(&x, &case, &ds, &mut budget).unwrap_err(), mismatch, "minor Δ{}", m + 2);
            }
            for e in 0..order * (order + 1) / 2 {
                let mut x = retained.clone();
                let terms = &mut x.response.witnesses[b].upper.as_mut().unwrap()[e];
                let last = terms.len() - 1;
                terms[last] = -terms[last];
                assert_eq!(replay(&x, &case, &ds, &mut budget).unwrap_err(), mismatch, "entry {e}");
            }
            let mut x = retained.clone();
            x.response.witnesses[b].minors = None;
            assert_eq!(replay(&x, &case, &ds, &mut budget).unwrap_err(), mismatch);
            let mut x = retained.clone();
            x.response.witnesses[b].upper = None;
            assert_eq!(replay(&x, &case, &ds, &mut budget).unwrap_err(), mismatch);
        }
        // The legacy replay keeps its order-2 witness scope.
        let legacy = retained.response().replay_against(
            &case.system(),
            ID,
            ForceBasis::DeclaredVector,
            &[],
            1e-9,
            retained.response().limits(),
            usize::MAX,
        );
        assert_eq!(legacy.result.unwrap_err(), Error::Invalid("retained minor size"));
    }
}

#[test]
fn retained_order_four_witness_size_is_bounded() {
    let dimensions = Error::Invalid("functional retained witness dimensions");
    let mismatch = Error::Invalid("functional retained block/minor mismatch");
    let e = Limits::default().expansion_terms;
    for case in [case_order_four_tails(), case_order_three_tails(), case_order_four()] {
        let ds = plan(&case);
        let mut budget = budget();
        let retained = retain(&case, &ds, &mut budget);
        for (b, witness) in retained.response.witnesses.iter().enumerate() {
            let order = witness.dofs.len();
            let bound = if order <= 2 {
                4 * e + 2
            } else {
                (order * (order + 1) / 2 + order) * e + order
            };
            // Pad one stored expansion to the given total scalar count.
            let padded = |total: usize| {
                let mut x = retained.clone();
                let w = &mut x.response.witnesses[b];
                let extra = total - w.scalar_count();
                let slot = match order {
                    1 => &mut w.a,
                    2 => w.determinant.as_mut().unwrap(),
                    _ => &mut w.upper.as_mut().unwrap()[0],
                };
                slot.extend(std::iter::repeat(1.0).take(extra));
                assert_eq!(w.scalar_count(), total);
                x
            };
            // At the bound: admitted by size, refused by the rebuilt witness.
            assert_eq!(replay(&padded(bound), &case, &ds, &mut budget).unwrap_err(), mismatch, "order {order}");
            // One above: refused before any rebuild.
            assert_eq!(replay(&padded(bound + 1), &case, &ds, &mut budget).unwrap_err(), dimensions, "order {order}");
        }
        // At most 4 DOFs per retained block.
        if let Some(b) = retained.response.witnesses.iter().position(|w| w.dofs.len() == 4) {
            let mut x = retained.clone();
            x.response.witnesses[b].dofs.push(0);
            assert_eq!(replay(&x, &case, &ds, &mut budget).unwrap_err(), dimensions);
        }
    }
}
