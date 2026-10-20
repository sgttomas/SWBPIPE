//! The W1 coverage census probe (DEL-04-08 Design `w1-coverage-repair.md` §3.2
//! and `w1e-sharper-exact-coverage.md` §6.2). Compiled only under the opt-in
//! `census-probe` feature, which no product or ordinary test build enables.
//!
//! - **Switches** are thread-local: the corner-1 driven search, and the lane
//!   counterfactuals (b) "the same section" and (c) "the same readout", which
//!   change the annular lane's problem and so are judged by re-running the
//!   invocation with the switch on.
//! - **Records** are thread-local and drained by the census targets. Search
//!   records come from every loaded-maximum search; gate records from
//!   `check_intervals`, after the actual verdicts.
//! - **Counterfactual judging** without a re-solve runs in separate work owners
//!   that no receipt reads: switch (a), the admitted Z set to the single Ẑ, and
//!   the projection and judging variants of w1e §6.2. Each recomputes the
//!   projected values, the support magnitudes, the coupled and floored scales
//!   and every class and predicate with the gate's own arithmetic. The probe
//!   checks itself on every case: its unaltered variant must reproduce the
//!   published values, the final scales and every actual verdict bit for bit.
//!
//! Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
use super::*;
use std::cell::{Cell, RefCell};

/// The census switches. All off by default, and always off without the feature.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Switches {
    /// The corner-1 repair (§1.2): drive an interval represented Z at z_lo.
    pub corner1: bool,
    /// Counterfactual (b): the annular lane takes the admitted section leaves.
    pub same_section: bool,
    /// Counterfactual (c): the annular lane takes the admitted readout whole.
    pub same_readout: bool,
    /// The null lane override: the annular lane's coefficients re-formed from
    /// its own section leaves by the (b) code path. Must reproduce today's run.
    pub lane_null: bool,
    /// Record search and gate records, with the counterfactual judging.
    pub record: bool,
}
impl Switches {
    pub(crate) fn shares_section(&self) -> bool {
        self.same_section || self.same_readout
    }
}

/// One recorded value.
#[derive(Clone, Debug, PartialEq)]
pub enum Field {
    F(f64),
    U(u64),
    I(i64),
    B(bool),
    S(String),
    L(Vec<(String, Field)>),
}
/// One probe record: a kind and its named fields.
#[derive(Clone, Debug)]
pub struct Record {
    pub kind: &'static str,
    pub fields: Vec<(String, Field)>,
}
impl Record {
    fn new(kind: &'static str) -> Self {
        Self { kind, fields: Vec::new() }
    }
    fn put(&mut self, k: &str, v: Field) -> &mut Self {
        self.fields.push((k.to_owned(), v));
        self
    }
    pub fn get(&self, k: &str) -> Option<&Field> {
        self.fields.iter().find(|(n, _)| n == k).map(|(_, v)| v)
    }
}

thread_local! {
    static SWITCHES: Cell<Switches> = const { Cell::new(Switches {
        corner1: false, same_section: false, same_readout: false, lane_null: false, record: false }) };
    static RECORDS: RefCell<Vec<Record>> = const { RefCell::new(Vec::new()) };
    static CONTEXT: RefCell<(String, &'static str)> = const { RefCell::new((String::new(), "")) };
    static LANE: Cell<bool> = const { Cell::new(false) };
}
/// Names the lane of the next loaded-maximum search: `true` for the represented
/// (admitted) lane, `false` for the annular lane.
pub(super) fn lane(represented: bool) {
    LANE.with(|l| l.set(represented));
}
/// The proof owner's counted work, component by component (wide and exact-sum LME,
/// the seven entry counts, binary64 arithmetic and visits), for a search's own
/// work as the difference of two snapshots. Reporting only; `u64::MAX` marks a
/// component that is not exact.
pub(super) fn work_counts(spent: &ProductCertificateSpent<'_>) -> [u64; 11] {
    let n = &spent.numeric;
    let x = |t: WorkTotal| t.exact().unwrap_or(u64::MAX);
    let mut out = [0u64; 11];
    out[0] = x(n.wide.checked_lme());
    out[1] = x(n.sums.checked_lme());
    for (i, e) in n.entries.iter().enumerate() {
        out[2 + i] = x(*e);
    }
    out[9] = x(n.f64_arithmetic);
    out[10] = x(spent.visits);
    out
}
/// The current thread's switches.
pub fn switches() -> Switches {
    SWITCHES.with(Cell::get)
}
/// Runs `f` with `s` on this thread, restoring the previous switches after.
pub fn with<T>(s: Switches, f: impl FnOnce() -> T) -> T {
    struct Restore(Switches);
    impl Drop for Restore {
        fn drop(&mut self) {
            SWITCHES.with(|c| c.set(self.0));
        }
    }
    let old = SWITCHES.with(|c| c.replace(s));
    let _restore = Restore(old);
    f()
}
/// Takes every record made on this thread since the last drain.
pub fn drain() -> Vec<Record> {
    RECORDS.with(|r| std::mem::take(&mut *r.borrow_mut()))
}
fn push(mut r: Record) {
    if !switches().record {
        return;
    }
    let (case, phase) = CONTEXT.with(|c| c.borrow().clone());
    r.put("case_id", Field::S(case)).put("phase", Field::S(phase.to_owned()));
    RECORDS.with(|v| v.borrow_mut().push(r));
}
/// Names the case and phase that the following records belong to.
pub(super) fn context(case_id: &str, phase: &'static str) {
    CONTEXT.with(|c| *c.borrow_mut() = (case_id.to_owned(), phase));
}

/// An endpoint's nearest binary64 (NaN where it has none), for reporting only.
fn approx(e: &Endpoint) -> f64 {
    e.to_binary64().value().unwrap_or(f64::NAN)
}
/// hi − lo, rounded up, as binary64 (reporting only, in a scratch owner).
fn width(e: Enclosure) -> f64 {
    let mut w = NumericWork::new();
    w.scalar(Entry::Sub, &e.hi, &e.lo, Toward::Up).map(|x| approx(&x)).unwrap_or(f64::NAN)
}
fn ulp(x: f64) -> f64 {
    let a = x.abs();
    if a == 0.0 || !a.is_finite() {
        return f64::from_bits(1);
    }
    f64::from_bits(a.to_bits() + 1) - a
}

// ------------------------------------------------------------------ the search

/// One loaded-maximum search's census record (C1).
pub(super) struct SearchProbe {
    row: usize,
    member: u32,
    driven: bool,
    z: Enclosure,
    best: [[Enclosure; 3]; 3],
    best_lower: Endpoint,
    max_live: usize,
    max_depth: u8,
    represented: bool,
    work_start: [u64; 11],
}
impl SearchProbe {
    pub(super) fn start(row: usize, member: u32, driven: bool, z: Enclosure, controls: [[Enclosure; 3]; 3], lower: Endpoint,
        work_start: [u64; 11]) -> Self {
        let represented = LANE.with(Cell::get);
        Self { row, member, driven, z, best: controls, best_lower: lower, max_live: 1, max_depth: 0, represented, work_start }
    }
    pub(super) fn point(&mut self, controls: [[Enclosure; 3]; 3], value: Endpoint, _lower: Endpoint, depth: u8) {
        if value.cmp_value(&self.best_lower) == Ordering::Greater {
            self.best_lower = value;
            self.best = controls;
        }
        self.max_depth = self.max_depth.max(depth);
    }
    pub(super) fn live(&mut self, n: usize) {
        self.max_live = self.max_live.max(n);
    }
    #[allow(clippy::too_many_arguments)]
    pub(super) fn finish(&self, outcome: &str, splits: usize, lower: Endpoint, lower_hull: Endpoint, upper: Endpoint, a: Enclosure,
        work_end: [u64; 11]) {
        let point_z = if self.driven { Enclosure::point(self.z.lo) } else { self.z };
        let (axial, bending) = loaded_maximum::probe_parts(self.best, a, point_z)
            .map(|(x, y)| (approx(&x), approx(&y))).unwrap_or((f64::NAN, f64::NAN));
        let zl = approx(&self.z.lo);
        let mut r = Record::new("search");
        r.put("row", Field::U(self.row as u64)).put("member", Field::U(u64::from(self.member)))
            .put("driven", Field::B(self.driven)).put("outcome", Field::S(outcome.to_owned()))
            .put("splits", Field::U(splits as u64)).put("max_live", Field::U(self.max_live as u64))
            .put("max_depth", Field::U(u64::from(self.max_depth)))
            .put("z_lo", Field::F(zl)).put("z_hi", Field::F(approx(&self.z.hi)))
            .put("z_width_2m64", Field::F(width(self.z) / zl * 2f64.powi(64)))
            .put("lower", Field::F(approx(&lower))).put("lower_hull", Field::F(approx(&lower_hull)))
            .put("upper", Field::F(approx(&upper)))
            .put("best_axial", Field::F(axial)).put("best_bending", Field::F(bending))
            .put("lane", Field::S(if self.represented { "represented" } else { "annular" }.to_owned()))
            // The search's exact result and its own counted work (W1 coverage repair §5,
            // `w1_annular_lane_search_is_unchanged`).
            .put("lower_exact", Field::S(format!("{:?}", (if self.driven { lower_hull } else { lower }).parts())))
            .put("upper_exact", Field::S(format!("{:?}", upper.parts())))
            .put("work", Field::L(work_end.iter().zip(self.work_start).enumerate()
                .map(|(i, (e, s))| (i.to_string(), Field::U(e.wrapping_sub(s)))).collect()));
        push(r);
    }
}

// ------------------------------------------------------------------ lane switches (b), (c)

/// Switches (b) and (c) on the annular lane's member coefficients, after its
/// own `build_member` and shear rigidity: (b) pairs the admitted section leaves
/// Â, Î, Ĵ (and Â_s) with the lane's own exact moduli; (c) takes the admitted
/// readout whole, so its coefficients are the admitted products. The
/// coefficient differences are re-formed as `build_member` forms them.
/// Its arithmetic runs in a scratch owner, so a null override reproduces
/// today's run byte for byte, counted work included.
pub(in crate::structural::retained) fn lane_override(input: &MemberOperands, enc: &mut MemberEnclosures, shear_area: Option<f64>)
    -> Result<(), NumericError> {
    let mut scratch = NumericWork::new();
    let work = &mut scratch;
    let s = switches();
    if s.same_readout {
        for i in 0..4 {
            enc.coefficients[i] = Enclosure::point(enc.admitted_products[i]);
            enc.coefficient_differences[i] = Endpoint::ZERO;
        }
        if let Some(shear) = enc.shear.as_mut() {
            shear.annular = Enclosure::point(shear.admitted);
        }
    } else if s.same_section || s.lane_null {
        let k = &input.admitted;
        let (a, i, j) = if s.lane_null { (enc.a, enc.i, enc.j) }
            else { (Enclosure::point(pos_lift(k.a)?), Enclosure::point(pos_lift(k.iz)?), Enclosure::point(pos_lift(k.j)?)) };
        enc.coefficients = [work.mul(enc.e, a)?, work.mul(enc.g, j)?, work.mul(enc.e, i)?, work.mul(enc.e, i)?];
        for index in 0..4 {
            let lo = work.absdiff(&enc.coefficients[index].lo, &enc.admitted_products[index])?;
            let hi = work.absdiff(&enc.coefficients[index].hi, &enc.admitted_products[index])?;
            enc.coefficient_differences[index] = max(lo, hi);
        }
        if let (Some(shear), Some(a_s)) = (enc.shear.as_mut(), shear_area) {
            if !s.lane_null {
                shear.annular = work.mul(enc.g, Enclosure::point(pos_lift(a_s)?))?;
            }
        }
    }
    Ok(())
}

// ------------------------------------------------------------------ the gate

/// One judged row as `check_intervals` saw it.
pub(super) struct GateRow {
    pub(super) row: usize,
    pub(super) k: Enclosure,
    pub(super) s: Enclosure,
    pub(super) scale: f64,
    pub(super) input: bool,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Sel { Both, K, S }
#[derive(Clone, Copy, PartialEq, Debug)]
enum ValueRule { Fixed(Sel), SiMid(Sel), UnitSearch, Any(Sel) }
#[derive(Clone, Copy, PartialEq, Debug)]
enum Judge { Gate(Sel), SharperOn(Sel), RawSharper }
struct Variant { name: &'static str, zpoint: bool, value: ValueRule, judge: Judge }
const fn v(name: &'static str, zpoint: bool, value: ValueRule, judge: Judge) -> Variant {
    Variant { name, zpoint, value, judge }
}
/// The judging variants (w1e §3.2's table and §6.2's counterfactuals, each
/// alone and with (a)). "base" is the unaltered gate (the self-check).
const VARIANTS: &[Variant] = &[
    v("base", false, ValueRule::Fixed(Sel::Both), Judge::Gate(Sel::Both)),
    v("a", true, ValueRule::Fixed(Sel::Both), Judge::Gate(Sel::Both)),
    v("only_k", false, ValueRule::Fixed(Sel::K), Judge::Gate(Sel::K)),
    v("only_s", false, ValueRule::Fixed(Sel::S), Judge::Gate(Sel::S)),
    v("a+only_k", true, ValueRule::Fixed(Sel::K), Judge::Gate(Sel::K)),
    v("a+only_s", true, ValueRule::Fixed(Sel::S), Judge::Gate(Sel::S)),
    v("any_both", false, ValueRule::Any(Sel::Both), Judge::Gate(Sel::Both)),
    v("any_k", false, ValueRule::Any(Sel::K), Judge::Gate(Sel::K)),
    v("any_s", false, ValueRule::Any(Sel::S), Judge::Gate(Sel::S)),
    v("a+any_both", true, ValueRule::Any(Sel::Both), Judge::Gate(Sel::Both)),
    v("a+any_k", true, ValueRule::Any(Sel::K), Judge::Gate(Sel::K)),
    v("a+any_s", true, ValueRule::Any(Sel::S), Judge::Gate(Sel::S)),
    // (d) U1 (iii): rows published in SI, n = RN64 of the SI midpoint.
    v("d", false, ValueRule::SiMid(Sel::Both), Judge::Gate(Sel::Both)),
    v("a+d", true, ValueRule::SiMid(Sel::Both), Judge::Gate(Sel::Both)),
    // (d') U1 (i): the published unit value searched within ±4 binary64 steps.
    v("dp", false, ValueRule::UnitSearch, Judge::Gate(Sel::Both)),
    v("a+dp", true, ValueRule::UnitSearch, Judge::Gate(Sel::Both)),
    // (d'') U1 (ii): both sharper predicates judged on the raw unit value.
    v("dpp", false, ValueRule::Fixed(Sel::Both), Judge::RawSharper),
    v("a+dpp", true, ValueRule::Fixed(Sel::Both), Judge::RawSharper),
    // (e) R1: the sharper predicates against one lane; dual-hull projection.
    v("e_k", false, ValueRule::Fixed(Sel::Both), Judge::SharperOn(Sel::K)),
    v("e_s", false, ValueRule::Fixed(Sel::Both), Judge::SharperOn(Sel::S)),
    v("a+e_k", true, ValueRule::Fixed(Sel::Both), Judge::SharperOn(Sel::K)),
    v("a+e_s", true, ValueRule::Fixed(Sel::Both), Judge::SharperOn(Sel::S)),
    // (e') R1': R1 with the selected lane projected alone.
    v("ep_k", false, ValueRule::Fixed(Sel::K), Judge::SharperOn(Sel::K)),
    v("ep_s", false, ValueRule::Fixed(Sel::S), Judge::SharperOn(Sel::S)),
    v("a+ep_k", true, ValueRule::Fixed(Sel::K), Judge::SharperOn(Sel::K)),
    v("a+ep_s", true, ValueRule::Fixed(Sel::S), Judge::SharperOn(Sel::S)),
];

/// One row's judgement under a variant.
#[derive(Clone, Copy, PartialEq, Debug)]
struct Verdict { passed: bool, class: u8, failed: Option<ProductPredicate>, predicates: [Option<bool>; 4] }
fn class_code(c: &Option<adaptive::RowClass>) -> u8 {
    match c {
        None => 0,
        Some(adaptive::RowClass::InputDerived) => 1,
        Some(adaptive::RowClass::RelativeVerified) => 2,
        Some(adaptive::RowClass::AbsoluteVerified { .. }) => 3,
        Some(_) => 4,
    }
}
fn class_name(c: u8) -> &'static str {
    ["none", "input_derived", "relative_verified", "absolute_verified", "other"][c as usize]
}
fn predicate_name(p: Option<ProductPredicate>) -> String {
    p.map_or("none".into(), |p| format!("{p:?}"))
}

/// The gate's judgement (`gate`) with its intervals and value parameterized:
/// `all` judges the class, the absolute bound and the two decimal predicates,
/// `sharper` the two sharper predicates; `raw_sharper` judges those on the
/// raw unit value and its raw interval (S in raw units). With `all` = `sharper`
/// and `raw_sharper` off, it is `gate` operation for operation.
#[allow(clippy::too_many_arguments)]
fn judge(w: &mut ProductCertificateSpent<'_>, y: f64, u: ProductUnit, all: Enclosure, sharper: Enclosure,
    scale: f64, input: bool, raw_sharper: bool) -> Result<Verdict, ProductFailure> {
    let n = w.normalize(u, y)?;
    let h = distance(&mut w.numeric, n, all)?;
    let small = f64::from_bits(0x0230_0000_0000_0000);
    let relative = if input || scale < small { false } else {
        let threshold = w.f64_op(Scalar64::Mul, f64::from_bits(adaptive::FLOOR_RATIO_BITS), scale)?;
        !(n.abs() < threshold)
    };
    let mut predicates = [None; 4];
    let (class, fail) = if input {
        predicates[0] = Some(h.is_zero());
        (1, if h.is_zero() { None } else { Some(ProductPredicate::InputDerived) })
    } else if !relative {
        let b = if scale > 0.0 && scale < small { w.small_bound(n, scale)? } else { w.absolute(scale)? };
        let ok = h.cmp_value(&lift(b)?) != Ordering::Greater;
        predicates[0] = Some(ok);
        (3, if ok { None } else { Some(ProductPredicate::Absolute) })
    } else {
        let (hs, xs, ss) = if raw_sharper {
            let raw = raw_interval(&mut w.numeric, sharper, u)?;
            let s_raw = match u {
                ProductUnit::Millimetre => scale * 1000.0,
                ProductUnit::Megapascal => scale / 1_000_000.0,
                _ => scale,
            };
            (distance(&mut w.numeric, y, raw)?, y, s_raw)
        } else {
            (distance(&mut w.numeric, n, sharper)?, n, scale)
        };
        let a0 = w.f64_op(Scalar64::Mul, f64::from_bits((1023u64 - 64) << 52), xs.abs().max(ss))?;
        let a1 = w.f64_op(Scalar64::Mul, a0, f64::from_bits(0x3ff0_0000_8000_0000))?;
        let u0 = w.f64_op(Scalar64::Mul, f64::from_bits((1023u64 - 53) << 52), xs.abs())?;
        let u1 = w.f64_op(Scalar64::Add, u0, f64::from_bits(1))?;
        let allowance = w.f64_op(Scalar64::Add, a1, u1)?;
        if !allowance.is_finite() {
            return Err(bad("allowance range"));
        }
        let raw = raw_interval(&mut w.numeric, all, u)?;
        let hu = distance(&mut w.numeric, y, raw)?;
        let tests = [
            (ProductPredicate::SharperExact, exact_test(w, hs, xs, ss, true)?),
            (ProductPredicate::SharperBinary64, hs.cmp_value(&lift(allowance)?) != Ordering::Greater),
            (ProductPredicate::DecimalSi, exact_test(w, h, n, scale, false)?),
            (ProductPredicate::DecimalRaw, exact_test(w, hu, y, scale, false)?),
        ];
        for (i, (_, ok)) in tests.iter().enumerate() {
            predicates[i] = Some(*ok);
        }
        (2, tests.into_iter().find(|(_, ok)| !*ok).map(|(p, _)| p))
    };
    Ok(Verdict { passed: fail.is_none(), class, failed: fail, predicates })
}

/// RN64, ties to even, of an enclosure's exact midpoint (SI).
fn si_midpoint(e: Enclosure) -> Option<f64> {
    let mut ctx = WideContext::<16>::new(1024).ok()?;
    let sum = ctx.add(&e.lo, &e.hi).ok()?;
    let m = shift(&sum, -1).ok()?;
    match m.to_binary64() {
        super::super::super::wide::multi::Binary64Outcome::Normal(y)
        | super::super::super::wide::multi::Binary64Outcome::Subnormal { value: y, .. } => Some(if y == 0.0 { 0.0 } else { y }),
        super::super::super::wide::multi::Binary64Outcome::Underflow { .. } => Some(0.0),
        _ => None,
    }
}
fn step(y: f64, k: i32) -> f64 {
    let mut x = y;
    for _ in 0..k.unsigned_abs() {
        x = if k > 0 { adaptive::next_up(x) } else { adaptive::next_down(x) };
    }
    x
}

/// Everything the counterfactual judging reads, from `check_intervals`.
pub(super) struct GateInputs<'x, 'r> {
    pub(super) owner: &'x adaptive::RetainedSolve,
    pub(super) facts: &'x [ProductMemberFacts],
    pub(super) reduced: &'x ReducedSectionSet,
    pub(super) reduced_map: &'x [u32],
    pub(super) k: &'x [Enclosure],
    pub(super) s: &'x [Enclosure],
    pub(super) k_curvatures: &'x [(u32, [Enclosure; 2])],
    pub(super) s_curvatures: &'x [(u32, [Enclosure; 2])],
    pub(super) rows: &'r [ProductFinalRow<'r>],
    pub(super) verdicts: &'x [ProductRowVerdict],
    pub(super) scales: &'x [[f64; 4]],
    pub(super) gate_rows: &'x [GateRow],
}

/// Which rows the kernel projects (`ProductProofDraft::project`); the others
/// keep their published value, or (magnitudes) are re-formed from components.
#[derive(Clone, Copy, PartialEq)]
enum Source { Projected, Published, SupportMagnitude(u32, usize), DisplacementNorm(u32) }
fn source_of(owner: &adaptive::RetainedSolve, r: ProductRecipe) -> Source {
    let combination = !owner.prep.factors.is_empty();
    match r {
        ProductRecipe::NonQuantity | ProductRecipe::ModulusBasisRecord | ProductRecipe::NotCovered { .. } => Source::Published,
        ProductRecipe::CircularMaximum { member } if !owner.member_is_loaded(member) => Source::Published,
        ProductRecipe::Native(QuantityId::SupportForceMagnitude(s)) => Source::SupportMagnitude(s, 0),
        ProductRecipe::Native(QuantityId::SupportMomentMagnitude(s)) => Source::SupportMagnitude(s, 3),
        ProductRecipe::Native(QuantityId::DisplacementMagnitude(n)) if combination => Source::DisplacementNorm(n),
        _ => Source::Projected,
    }
}

/// Records the gate's rows with every counterfactual judgement, for one case.
pub(super) fn gate_records(g: &GateInputs<'_, '_>) {
    let case = g.rows.first().map_or("", |r| r.case_id).to_owned();
    context(&case, "counterfactual");
    let n = g.rows.len();
    // Per-row lane enclosures, actual and under (a).
    let mut by_row: Vec<Option<usize>> = vec![None; n];
    for (j, gr) in g.gate_rows.iter().enumerate() {
        by_row[gr.row] = Some(j);
    }
    let mut zpoint: Vec<Option<Result<(Enclosure, Enclosure), String>>> = vec![None; n];
    let shared = switches().shares_section();
    for gr in g.gate_rows {
        let r = &g.rows[gr.row];
        let bending = matches!(r.recipe, ProductRecipe::Stress { stress: ProductStress::BendingY | ProductStress::BendingZ, .. }
            | ProductRecipe::CircularMaximum { .. });
        if !bending {
            continue;
        }
        let mut w = ProductCertificateSpent::new(g.rows);
        let result = (|| -> Result<(Enclosure, Enclosure), ProductFailure> {
            let member = match r.recipe {
                ProductRecipe::Stress { member, .. } | ProductRecipe::CircularMaximum { member } => member,
                _ => unreachable!(),
            };
            let mi = g.owner.source().member_index(member).ok_or_else(|| bad("member"))?;
            let row_reduced = reduced_of(g.reduced, g.reduced_map, gr.row);
            let (m, f) = (&g.owner.source().members()[mi], &g.facts[mi]);
            let mut section = build_member(&section_operands(m, f, row_reduced), &mut w.numeric)?;
            let zhat = row_reduced.map_or(f.section_modulus, |rs| rs.section_modulus);
            section.represented_z = Some(Enclosure::point(pos_lift(zhat)?));
            let k = recipe_reduced(g.owner, &mut w, g.k, r.recipe, Some(&section), g.facts, row_reduced, true, g.k_curvatures, gr.row)?;
            let s = if shared {
                recipe_reduced(g.owner, &mut w, g.s, r.recipe, Some(&section), g.facts, row_reduced, false, g.s_curvatures, gr.row)?
            } else {
                gr.s
            };
            Ok((k, s))
        })();
        zpoint[gr.row] = Some(result.map_err(|e| format!("{e:?}")));
    }
    let lanes = |row: usize, z: bool| -> Option<(Enclosure, Enclosure)> {
        let gr = &g.gate_rows[by_row[row]?];
        if z {
            match &zpoint[row] {
                Some(Ok(p)) => Some(*p),
                Some(Err(_)) => None,
                None => Some((gr.k, gr.s)),
            }
        } else {
            Some((gr.k, gr.s))
        }
    };
    let pick = |p: (Enclosure, Enclosure), sel: Sel| match sel {
        Sel::Both => hull(p.0, p.1),
        Sel::K => p.0,
        Sel::S => p.1,
    };
    let source = g.owner.source();
    let nb = source.body_count() as usize;
    // Values of one variant (unit or SI); None where a row cannot be formed.
    let values_of = |zp: bool, rule: ValueRule, w: &mut ProductCertificateSpent<'_>| -> Vec<Option<f64>> {
        let si = matches!(rule, ValueRule::SiMid(_));
        let sel = match rule { ValueRule::Fixed(s) | ValueRule::SiMid(s) | ValueRule::Any(s) => s, ValueRule::UnitSearch => Sel::Both };
        let mut out: Vec<Option<f64>> = vec![None; n];
        for (i, r) in g.rows.iter().enumerate() {
            out[i] = match source_of(g.owner, r.recipe) {
                Source::Published => Some(if si { r.unit.normalize(*r.value) } else { *r.value }),
                Source::Projected => lanes(i, zp).and_then(|p| {
                    let e = pick(p, sel);
                    if si { si_midpoint(e) } else { project_hull(e, r.unit, i, w).ok() }
                }),
                _ => None,
            };
        }
        for (i, r) in g.rows.iter().enumerate() {
            out[i] = match source_of(g.owner, r.recipe) {
                Source::SupportMagnitude(support, first) => {
                    let mut c = [0.0; 3];
                    let mut ok = true;
                    for (j, slot) in c.iter_mut().enumerate() {
                        let want = ProductRecipe::SupportComponent { support, component: Component::ALL[first + j] };
                        match g.rows.iter().position(|x| x.recipe == want).and_then(|p| out[p]) {
                            Some(x) => *slot = x,
                            None => ok = false,
                        }
                    }
                    ok.then(|| { let v = open_pipe_stress_correct_norm::norm3(c[0], c[1], c[2]); if v == 0.0 { 0.0 } else { v } })
                }
                Source::DisplacementNorm(node) => {
                    let mut c = [0.0; 3];
                    let mut ok = true;
                    for (j, slot) in c.iter_mut().enumerate() {
                        let want = ProductRecipe::Native(QuantityId::Displacement(Dof { node, component: Component::ALL[j] }));
                        match g.rows.iter().position(|x| x.recipe == want).and_then(|p| out[p]) {
                            Some(x) => *slot = x,
                            None => ok = false,
                        }
                    }
                    if ok { rn64_norm3(c).result.ok() } else { None }
                }
                _ => out[i],
            };
        }
        out
    };
    // The coupled, floored scales of a variant's values (`row_scales_reduced`'s
    // scale law, `couple`, and the p = 512 floor).
    let scales_of = |vals: &[Option<f64>], si: bool, w: &mut ProductCertificateSpent<'_>| -> Option<Vec<[f64; 4]>> {
        let mut sc = vec![[0.0f64; 4]; nb];
        for (i, r) in g.rows.iter().enumerate() {
            let slot = match r.recipe {
                ProductRecipe::SupportComponent { component, .. } => Some(if component.index() < 3 { 2 } else { 3 }),
                ProductRecipe::Native(id) => {
                    let index = native_index(g.owner, id).ok()?;
                    let nr = &g.owner.publish().rows[index];
                    (nr.class != adaptive::RowClass::InputDerived).then_some(match nr.kind {
                        Kind::Translation => 0, Kind::Rotation => 1, Kind::Force => 2, Kind::Moment => 3 })
                }
                _ => None,
            };
            if let Some(slot) = slot {
                let x = vals[i]?;
                let nx = if si { x } else { w.normalize(r.unit, x).ok()? };
                sc[r.body as usize][slot] = sc[r.body as usize][slot].max(nx.abs());
            }
        }
        for (b, s) in sc.iter_mut().enumerate() {
            let extent = body_extent(source, b as u32, w).ok()?;
            *s = couple(*s, extent, w).ok()?;
            if g.owner.selected_precision() == 512 {
                let floor = g.owner.evidence().floor.as_ref()?.iter().find(|f| f.0 == b as u32)?;
                s[2] = s[2].max(f64::from_bits(floor.1));
                s[3] = s[3].max(f64::from_bits(floor.2));
            }
        }
        Some(sc)
    };
    // One row's judging scale from body scales (`check_intervals`).
    let row_scale = |i: usize, sc: &[[f64; 4]], w: &mut ProductCertificateSpent<'_>| -> Option<f64> {
        let r = &g.rows[i];
        Some(match r.recipe {
            ProductRecipe::Native(id) => {
                let nr = &g.owner.publish().rows[native_index(g.owner, id).ok()?];
                sc[r.body as usize][match nr.kind { Kind::Translation => 0, Kind::Rotation => 1, Kind::Force => 2, Kind::Moment => 3 }]
            }
            ProductRecipe::SupportComponent { component, .. } => sc[r.body as usize][if component.index() < 3 { 2 } else { 3 }],
            ProductRecipe::Stress { member, .. } | ProductRecipe::CircularMaximum { member } => {
                let f = &g.facts[source.member_index(member)?];
                let k = if matches!(r.recipe, ProductRecipe::CircularMaximum { .. }) { f64::from_bits(adaptive::K_TWO_SQRT2_BITS) } else { 1.0 };
                let (area, modulus) = match reduced_of(g.reduced, g.reduced_map, i) {
                    Some(rs) => (rs.area, rs.section_modulus),
                    None => (f.area, f.section_modulus),
                };
                let s = sc[r.body as usize];
                let axial = w.f64_op(Scalar64::Div, s[2], area).ok()?;
                let bending = w.f64_op(Scalar64::Div, s[3], modulus).ok()?;
                let bending = w.f64_op(Scalar64::Mul, k, bending).ok()?;
                w.f64_op(Scalar64::Add, axial, bending).ok()?
            }
            _ => return None,
        })
    };
    let judged = |i: usize| by_row[i].is_some();

    let mut results: Vec<Vec<(String, Field)>> = vec![Vec::new(); n];
    let mut self_check = Vec::new();
    for var in VARIANTS {
        let mut w = ProductCertificateSpent::new(g.rows);
        w.projection_outcomes = Vec::with_capacity(4 * n + 8);
        let si_values = matches!(var.value, ValueRule::SiMid(_));
        let (vals, scales) = match var.value {
            ValueRule::Fixed(_) | ValueRule::SiMid(_) => {
                let vals = values_of(var.zpoint, var.value, &mut w);
                let sc = scales_of(&vals, si_values, &mut w);
                (vals, sc)
            }
            // The search and any-value rules judge candidates at the scales of
            // their fixed-projection counterpart; each row's candidates are judged
            // on their own, so these are fixed-scale feasibility diagnostics
            // (w1-coverage-repair.md §3.2, revision 10).
            ValueRule::UnitSearch => {
                let vals = values_of(var.zpoint, ValueRule::Fixed(Sel::Both), &mut w);
                let sc = scales_of(&vals, false, &mut w);
                (vals, sc)
            }
            ValueRule::Any(sel) => {
                let vals = values_of(var.zpoint, ValueRule::Fixed(sel), &mut w);
                let sc = scales_of(&vals, false, &mut w);
                (vals, sc)
            }
        };
        if var.name == "base" {
            let values_ok = g.rows.iter().zip(&vals).all(|(r, x)| x.is_some_and(|x| x.to_bits() == r.value.to_bits()));
            let scales_ok = scales.as_ref().is_some_and(|s| s.len() == g.scales.len()
                && s.iter().zip(g.scales).all(|(a, b)| a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits())));
            self_check.push(("values".to_owned(), Field::B(values_ok)));
            self_check.push(("scales".to_owned(), Field::B(scales_ok)));
        }
        let mut verdicts_ok = true;
        for i in 0..n {
            if !judged(i) {
                continue;
            }
            let gr = &g.gate_rows[by_row[i].unwrap()];
            let r = &g.rows[i];
            let outcome = (|| -> Option<bool> {
                let p = lanes(i, var.zpoint)?;
                let sc = scales.as_ref()?;
                let scale = row_scale(i, sc, &mut w)?;
                let (all, sharper, raw) = match var.judge {
                    Judge::Gate(sel) => (pick(p, sel), pick(p, sel), false),
                    Judge::SharperOn(sel) => (hull(p.0, p.1), pick(p, sel), false),
                    Judge::RawSharper => (hull(p.0, p.1), hull(p.0, p.1), true),
                };
                match var.value {
                    ValueRule::Fixed(_) => {
                        let y = vals[i]?;
                        let v = judge(&mut w, y, r.unit, all, sharper, scale, gr.input, raw).ok()?;
                        if var.name == "base" {
                            let actual = &g.verdicts[i];
                            let same = v.passed == actual.passed && v.class == class_code(&actual.class)
                                && v.failed == actual.failed && v.predicates == actual.predicates
                                && scale.to_bits() == actual.scale_bits;
                            verdicts_ok &= same;
                        }
                        Some(v.passed)
                    }
                    ValueRule::SiMid(_) => {
                        let y = vals[i]?;
                        Some(judge(&mut w, y, ProductUnit::Pascal, all, sharper, scale, gr.input, raw).ok()?.passed)
                    }
                    ValueRule::UnitSearch => {
                        let y0 = vals[i]?;
                        let steps: &[i32] = if source_of(g.owner, r.recipe) == Source::Projected { &[0, 1, -1, 2, -2, 3, -3, 4, -4] } else { &[0] };
                        for &k in steps {
                            let y = step(y0, k);
                            if judge(&mut w, y, r.unit, all, sharper, scale, gr.input, raw).ok()?.passed {
                                return Some(true);
                            }
                        }
                        Some(false)
                    }
                    ValueRule::Any(sel) => {
                        let m = si_midpoint(pick(p, sel))?;
                        let mut candidates = vec![m];
                        for k in 1..=3 {
                            candidates.push(step(m, k));
                            candidates.push(step(m, -k));
                        }
                        let small = f64::from_bits(0x0230_0000_0000_0000);
                        if !gr.input && scale >= small {
                            let b = f64::from_bits(adaptive::FLOOR_RATIO_BITS) * scale;
                            candidates.push(if m < 0.0 { -b } else { b });
                        }
                        for y in candidates {
                            if judge(&mut w, y, ProductUnit::Pascal, all, sharper, scale, gr.input, raw).ok()?.passed {
                                return Some(true);
                            }
                        }
                        Some(false)
                    }
                }
            })();
            results[i].push((var.name.to_owned(), match outcome { Some(b) => Field::B(b), None => Field::S("unformed".into()) }));
        }
        if var.name == "base" {
            self_check.push(("verdicts".to_owned(), Field::B(verdicts_ok)));
        }
    }

    // The case record: final scales, native scales and the self-check.
    let mut c = Record::new("gate_case");
    c.put("rows", Field::U(n as u64)).put("precision", Field::U(u64::from(g.owner.selected_precision())))
        .put("self_check", Field::L(self_check))
        .put("final_scales", Field::L(g.scales.iter().enumerate().map(|(b, s)|
            (b.to_string(), Field::L(["translation", "rotation", "force", "moment"].iter().zip(s)
                .map(|(k, x)| ((*k).to_owned(), Field::U(x.to_bits()))).collect()))).collect()))
        .put("native_scales", Field::L(g.owner.evidence().body_scales.iter()
            .map(|(b, kind, bits)| (format!("{b}:{kind:?}"), Field::U(*bits))).collect()));
    push(c);
    // The row records.
    for gr in g.gate_rows {
        let i = gr.row;
        let r = &g.rows[i];
        let actual = &g.verdicts[i];
        let nval = f64::from_bits(actual.normalized_bits);
        let h = hull(gr.k, gr.s);
        let u = ulp(nval);
        let (recipe, member) = match r.recipe {
            ProductRecipe::Native(id) => (format!("native:{:?}", g.owner.publish().rows.get(native_index(g.owner, id).unwrap_or(usize::MAX)).map(|x| x.kind)), None),
            ProductRecipe::SupportComponent { component, .. } => (format!("support:{}", if component.index() < 3 { "force" } else { "moment" }), None),
            ProductRecipe::Stress { member, stress, .. } => (format!("stress:{stress:?}"), Some(member)),
            ProductRecipe::CircularMaximum { member } => (if g.owner.member_is_loaded(member) { "maximum:loaded".into() } else { "maximum:unloaded".into() }, Some(member)),
            other => (format!("{other:?}"), None),
        };
        let mut rec = Record::new("gate_row");
        rec.put("row", Field::U(i as u64)).put("id", Field::S(r.id.to_owned())).put("recipe", Field::S(recipe))
            .put("member", member.map_or(Field::I(-1), |m| Field::I(i64::from(m))))
            .put("unit", Field::S(format!("{:?}", r.unit))).put("body", Field::U(u64::from(r.body)))
            .put("value_bits", Field::U(r.value.to_bits())).put("normalized_bits", Field::U(actual.normalized_bits))
            .put("scale_bits", Field::U(actual.scale_bits)).put("input", Field::B(gr.input))
            .put("class", Field::S(class_name(class_code(&actual.class)).to_owned()))
            .put("passed", Field::B(actual.passed)).put("failed", Field::S(predicate_name(actual.failed)))
            .put("k_lo", Field::F(approx(&gr.k.lo))).put("k_hi", Field::F(approx(&gr.k.hi)))
            .put("s_lo", Field::F(approx(&gr.s.lo))).put("s_hi", Field::F(approx(&gr.s.hi)))
            .put("k_width_ulp", Field::F(width(gr.k) / u)).put("s_width_ulp", Field::F(width(gr.s) / u))
            .put("hull_width_2m64", Field::F(if nval == 0.0 { f64::NAN } else { width(h) / nval.abs() * 2f64.powi(64) }))
            .put("zpoint_error", Field::S(match &zpoint[i] { Some(Err(e)) => e.clone(), _ => String::new() }))
            // w1e §6.2's per-row diagnostics: h (rounded up, and in units of n's ulp),
            // the binary64 sharper allowance as `gate` forms it, the absolute bound
            // where the class is absolute, and all four predicate verdicts.
            .put("h", Field::F(distance(&mut NumericWork::new(), nval, h).map(|x| approx(&x)).unwrap_or(f64::NAN)))
            .put("h_ulp", Field::F(distance(&mut NumericWork::new(), nval, h).map(|x| approx(&x) / u).unwrap_or(f64::NAN)))
            .put("allowance", Field::F({
                let s = f64::from_bits(actual.scale_bits);
                let a1 = f64::from_bits((1023u64 - 64) << 52) * nval.abs().max(s) * f64::from_bits(0x3ff0_0000_8000_0000);
                a1 + (f64::from_bits((1023u64 - 53) << 52) * nval.abs() + f64::from_bits(1))
            }))
            .put("absolute_bound_bits", match actual.class { Some(adaptive::RowClass::AbsoluteVerified { bound_bits }) => Field::U(bound_bits), _ => Field::I(-1) })
            .put("predicates", Field::L(["sharper_exact", "sharper_binary64", "decimal_si", "decimal_raw"].iter().zip(actual.predicates)
                .map(|(k, p)| ((*k).to_owned(), p.map_or(Field::S("not_judged".into()), Field::B))).collect()))
            .put("variants", Field::L(std::mem::take(&mut results[i])));
        push(rec);
    }
}
