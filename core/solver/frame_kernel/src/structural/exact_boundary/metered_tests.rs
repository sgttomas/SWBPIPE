//! RF source-method design §5 and §8 (RF-W1 to RF-W4): the metered
//! `Work::mul` against an independent test-side oracle, run through every
//! entry, and the maintained operation-count control (§5.4).
//!
//! The oracle reimplements grow-expansion (two-sum with zero elimination) and
//! the two-product with radix scaling on binary64. It calls none of the
//! producer's `Expansion`, `add_product`, `add_product_charged` or
//! `exact_radix`, and states the rule itself: 96 per term pair, then
//! 6·len + 1 before each of the pair's two adds on the length at that moment.
//! Zero elimination depends on the values, so it replays the arithmetic.
//! `explicit` independently derives the non-arithmetic charge sequences for
//! each exercised entry; these are also checked on every refusal prefix.
//!
//! Charge equality uses no producer operation counters. Execution order needs
//! additional evidence: `metered_refusal_precedes_the_refused_step` and the
//! counted-operation comparisons below assert that a refused step did not run.
//! In particular, moving a charge after an add can preserve charge equality.
mod explicit;
use super::exact_operation_count as count;
use super::functionals::{
    AffineTerm, AttemptBudget, FunctionalConvention, FunctionalDescriptor, FunctionalKey,
    FunctionalPlan, FunctionalQuantity, FunctionalUnit, HintRule, MemberEnd,
};
use super::*;
use std::collections::BTreeSet;
use trace::{Event, Ledger, Outcome};

/// A test-only trace of one thread's `Work` events: the operands of every
/// `Work::mul` and `Work::add`, and every explicit `Work::charge` (validated
/// against `explicit`'s sequences).
pub(super) mod trace {
    use super::*;
    use std::cell::{Cell, RefCell};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) enum Outcome {
        Ok,
        Budget,
        Range,
    }
    pub(crate) fn outcome<T>(result: &Result<T, Error>) -> Outcome {
        match result {
            Ok(_) => Outcome::Ok,
            Err(Error::Budget) => Outcome::Budget,
            Err(Error::Arithmetic(_)) => Outcome::Range,
            Err(e) => panic!("Work arithmetic returned {e:?}"),
        }
    }
    /// The executing ledger at a call's start.
    #[derive(Debug, Clone, Copy)]
    pub(crate) struct Ledger {
        pub(crate) used: usize,
        pub(crate) limit: usize,
        pub(crate) terms: usize,
    }
    #[derive(Debug, Clone)]
    pub(crate) enum Event {
        /// An explicit `Work::charge`, made outside `Work::add` and `Work::mul`.
        Charge { amount: usize, accepted: bool },
        Add {
            ledger: Ledger,
            len: usize,
            charged: usize,
            rejected: u64,
            outcome: Outcome,
        },
        Mul {
            ledger: Ledger,
            a: Vec<f64>,
            b: Vec<f64>,
            charged: usize,
            rejected: u64,
            outcome: Outcome,
            product: Vec<f64>,
        },
    }
    thread_local! {
        static EVENTS: RefCell<Option<Vec<Event>>> = const { RefCell::new(None) };
        static DEPTH: Cell<usize> = const { Cell::new(0) };
    }
    pub(crate) fn start() {
        DEPTH.with(|d| d.set(0));
        EVENTS.with(|e| *e.borrow_mut() = Some(Vec::new()));
    }
    pub(crate) fn take() -> Vec<Event> {
        EVENTS
            .with(|e| e.borrow_mut().take())
            .expect("trace::start before trace::take")
    }
    fn recording() -> bool {
        EVENTS.with(|e| e.borrow().is_some())
    }
    fn push(event: Event) {
        EVENTS.with(|e| {
            if let Some(events) = e.borrow_mut().as_mut() {
                events.push(event);
            }
        });
    }
    pub(crate) struct Open {
        ledger: Ledger,
        rejected: u64,
        len: usize,
        operands: Option<(Vec<f64>, Vec<f64>)>,
    }
    fn open(w: &Work, len: usize, operands: Option<(Vec<f64>, Vec<f64>)>) -> Open {
        Open {
            ledger: Ledger {
                used: w.used,
                limit: w.limits.operations,
                terms: w.limits.expansion_terms,
            },
            rejected: w.rejected.exact().unwrap(),
            len,
            operands,
        }
    }
    fn close(open: &Open, w: &Work) -> (usize, u64) {
        (
            w.used - open.ledger.used,
            w.rejected.exact().unwrap() - open.rejected,
        )
    }
    pub(crate) fn charge(amount: usize, accepted: bool) {
        if DEPTH.with(Cell::get) == 0 {
            push(Event::Charge { amount, accepted });
        }
    }
    pub(crate) fn begin_add(w: &Work, a: &Expansion) -> Option<Open> {
        DEPTH.with(|d| d.set(d.get() + 1));
        recording().then(|| open(w, a.terms.len(), None))
    }
    pub(crate) fn end_add(open: Option<Open>, w: &Work, result: &Result<(), Error>) {
        DEPTH.with(|d| d.set(d.get() - 1));
        if let Some(open) = open {
            let (charged, rejected) = close(&open, w);
            push(Event::Add {
                ledger: open.ledger,
                len: open.len,
                charged,
                rejected,
                outcome: outcome(result),
            });
        }
    }
    pub(crate) fn begin_mul(w: &Work, a: &Expansion, b: &Expansion) -> Option<Open> {
        DEPTH.with(|d| d.set(d.get() + 1));
        recording().then(|| open(w, 0, Some((a.terms.clone(), b.terms.clone()))))
    }
    pub(crate) fn end_mul(open: Option<Open>, w: &Work, result: &Result<Expansion, Error>) {
        DEPTH.with(|d| d.set(d.get() - 1));
        if let Some(open) = open {
            let (charged, rejected) = close(&open, w);
            let (a, b) = open.operands.unwrap();
            push(Event::Mul {
                ledger: open.ledger,
                a,
                b,
                charged,
                rejected,
                outcome: outcome(result),
                product: result.as_ref().map_or_else(|_| Vec::new(), |e| e.terms.clone()),
            });
        }
    }
}

/// The independent oracle, on binary64 only.
mod oracle {
    use super::trace::{Ledger, Outcome};

    /// The rule, stated independently of the producer's constants.
    pub(super) const PAIR: usize = 96;
    const TWO_64: f64 = 18_446_744_073_709_551_616.0;

    /// floor(log2 |x|) for finite nonzero x.
    pub(super) fn exponent(x: f64) -> i32 {
        let x = x.abs();
        if x < f64::MIN_POSITIVE {
            return exponent(x * TWO_64) - 64;
        }
        (x.to_bits() >> 52) as i32 - 1023
    }
    /// The exponent of the lowest set bit of finite nonzero x.
    fn low_exponent(x: f64) -> i32 {
        let x = x.abs();
        if x < f64::MIN_POSITIVE {
            return low_exponent(x * TWO_64) - 64;
        }
        let significand = (x.to_bits() & ((1 << 52) - 1)) | (1 << 52);
        exponent(x) - 52 + significand.trailing_zeros() as i32
    }
    fn power_of_two(k: i32) -> f64 {
        assert!((-1022..=1023).contains(&k));
        f64::from_bits(((k + 1023) as u64) << 52)
    }
    /// x·2^k when that is exactly representable, which is exactly when the
    /// producer's radix scaling succeeds; `None` when it refuses.
    fn scale(x: f64, k: i32) -> Option<f64> {
        if x == 0.0 {
            return Some(x);
        }
        if exponent(x) + k > 1023 || low_exponent(x) + k < -1074 {
            return None;
        }
        let (mut y, mut rest) = (x, k);
        while rest != 0 {
            let step = rest.clamp(-1000, 1000);
            y *= power_of_two(step);
            rest -= step;
        }
        Some(y)
    }
    /// §5.3's model: one radix iteration per step of at most 512, through the
    /// step at which the scaling refuses.
    fn radix_iterations(x: f64, k: i32) -> u64 {
        if x == 0.0 {
            return 0;
        }
        let (top, low) = (exponent(x), low_exponent(x));
        let (mut done, mut n) = (0, 0);
        while done != k {
            done += (k - done).clamp(-512, 512);
            n += 1;
            if top + done > 1023 || low + done < -1074 {
                break;
            }
        }
        n
    }
    /// Dekker's exact product by Veltkamp splitting (no fused multiply-add):
    /// hi + lo = a·b exactly, for significands in [1, 2).
    fn two_product(a: f64, b: f64) -> (f64, f64) {
        const SPLITTER: f64 = 134_217_729.0; // 2^27 + 1
        let split = |x: f64| {
            let c = SPLITTER * x;
            let high = c - (c - x);
            (high, x - high)
        };
        let hi = a * b;
        let ((ah, al), (bh, bl)) = (split(a), split(b));
        (hi, al * bl - (((hi - ah * bh) - al * bh) - ah * bl))
    }
    /// Six-operation TwoSum, written independently of Expansion::add. Its
    /// virtual operand must also be finite: near MAX, it can overflow even
    /// when a+b is finite. Fast2Sum would incorrectly accept that producer
    /// Range case, so retaining the six-operation decomposition matters.
    fn two_sum(a: f64, b: f64) -> Option<(f64, f64)> {
        let rounded = a + b;
        let b_virtual = rounded - a;
        let a_virtual = rounded - b_virtual;
        let b_roundoff = b - b_virtual;
        let a_roundoff = a - a_virtual;
        let residual = a_roundoff + b_roundoff;
        (rounded.is_finite() && b_virtual.is_finite() && residual.is_finite())
            .then_some((rounded, residual))
    }
    /// Grow-expansion with zero elimination. `Err` carries the two-sum steps
    /// executed before an overflow.
    pub(super) fn grow(terms: &[f64], x: f64) -> Result<Vec<f64>, usize> {
        let mut q = x;
        let mut out = Vec::with_capacity(terms.len() + 1);
        for (i, &e) in terms.iter().enumerate() {
            let Some((s, err)) = two_sum(q, e) else {
                return Err(i + 1);
            };
            if err != 0.0 {
                out.push(err);
            }
            q = s;
        }
        if q != 0.0 {
            out.push(q);
        }
        Ok(out)
    }

    #[derive(Debug, Clone)]
    pub(super) struct Replay {
        pub(super) charged: usize,
        pub(super) rejected: u64,
        /// Operations executed under §5.3's model.
        pub(super) counted: u64,
        pub(super) outcome: Outcome,
        pub(super) product: Vec<f64>,
        /// The ledger total after each accepted charge, in order.
        pub(super) boundaries: Vec<usize>,
        pub(super) longest: usize,
        /// Adds whose output came out shorter than their input (cancellation).
        pub(super) shrinks: usize,
    }
    impl Replay {
        fn take(&mut self, ledger: &Ledger, n: usize) -> bool {
            match ledger
                .used
                .checked_add(self.charged)
                .and_then(|u| u.checked_add(n))
            {
                Some(total) if total <= ledger.limit => {
                    self.charged += n;
                    self.boundaries.push(total);
                    true
                }
                _ => {
                    self.rejected += n as u64;
                    false
                }
            }
        }
        fn stop(mut self, outcome: Outcome) -> Self {
            self.outcome = outcome;
            self
        }
    }
    /// One `Work::mul` of expansions `a` and `b` on `ledger`.
    pub(super) fn mul(a: &[f64], b: &[f64], ledger: &Ledger) -> Replay {
        let mut r = Replay {
            charged: 0,
            rejected: 0,
            counted: 0,
            outcome: Outcome::Ok,
            product: Vec::new(),
            boundaries: Vec::new(),
            longest: 0,
            shrinks: 0,
        };
        for &x in a {
            for &y in b {
                if r.product.len() + 2 > ledger.terms {
                    return r.stop(Outcome::Budget);
                }
                if !r.take(ledger, PAIR) {
                    return r.stop(Outcome::Budget);
                }
                if x == 0.0 || y == 0.0 {
                    continue;
                }
                let (ex, ey) = (exponent(x), exponent(y));
                r.counted += 4 * (radix_iterations(x, -ex) + radix_iterations(y, -ey)) + 2;
                let (hi, lo) = two_product(scale(x, -ex).unwrap(), scale(y, -ey).unwrap());
                for part in [lo, hi] {
                    r.counted += 4 * radix_iterations(part, ex + ey);
                    let Some(part) = scale(part, ex + ey) else {
                        return r.stop(Outcome::Range);
                    };
                    let len = r.product.len();
                    if !r.take(ledger, 6 * len + 1) {
                        return r.stop(Outcome::Budget);
                    }
                    match grow(&r.product, part) {
                        Ok(grown) => {
                            r.counted += 6 * len as u64;
                            r.shrinks += usize::from(grown.len() < len);
                            r.product = grown;
                            r.longest = r.longest.max(r.product.len());
                        }
                        Err(steps) => {
                            r.counted += 6 * steps as u64;
                            return r.stop(Outcome::Range);
                        }
                    }
                }
            }
        }
        r
    }
    /// One `Work::add` onto an expansion of `len` terms: (charged, rejected,
    /// whether the add runs).
    pub(super) fn add(len: usize, ledger: &Ledger) -> (usize, u64, bool) {
        if len >= ledger.terms {
            return (0, 0, false);
        }
        let n = 6 * len + 1;
        match ledger.used.checked_add(n) {
            Some(total) if total <= ledger.limit => (n, 0, true),
            _ => (0, n as u64, false),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Tally {
    charged: usize,
    rejected: u64,
    counted: u64,
    muls: usize,
    adds: usize,
    ranges: usize,
}
fn same_bits(a: &[f64], b: &[f64]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits())
}
/// Recomputes every arithmetic charge of a trace from its operands. Entry
/// schedules independently validate explicit charges before ledger equality
/// is considered evidence for the whole entry. Each mul equals the oracle.
fn verify(events: &[Event], entry: &str) -> Tally {
    if entry == "Work::mul" {
        explicit::verify(events, &[], entry);
    }
    let mut t = Tally::default();
    for (i, event) in events.iter().enumerate() {
        match event {
            Event::Charge { amount, accepted } => {
                if *accepted {
                    t.charged += amount;
                } else {
                    t.rejected += *amount as u64;
                }
            }
            Event::Add {
                ledger,
                len,
                charged,
                rejected,
                outcome,
            } => {
                let (c, r, runs) = oracle::add(*len, ledger);
                assert_eq!((c, r), (*charged, *rejected), "{entry}: event {i}, Work::add at length {len}");
                assert_eq!(runs, *outcome != Outcome::Budget, "{entry}: event {i}, Work::add outcome");
                if runs {
                    t.counted += 6 * *len as u64;
                }
                t.ranges += usize::from(*outcome == Outcome::Range);
                t.charged += c;
                t.rejected += r;
                t.adds += 1;
            }
            Event::Mul {
                ledger,
                a,
                b,
                charged,
                rejected,
                outcome,
                product,
            } => {
                let o = oracle::mul(a, b, ledger);
                let what = format!("{entry}: event {i}, Work::mul of {a:?} by {b:?} on {ledger:?}");
                assert_eq!(o.charged, *charged, "{what}: charged");
                assert_eq!(o.rejected, *rejected, "{what}: rejected");
                assert_eq!(o.outcome, *outcome, "{what}: outcome");
                if o.outcome == Outcome::Ok {
                    assert!(same_bits(&o.product, product), "{what}: product terms");
                }
                t.ranges += usize::from(o.outcome == Outcome::Range);
                t.charged += o.charged;
                t.rejected += o.rejected;
                t.counted += o.counted;
                t.muls += 1;
            }
        }
    }
    t
}
struct Traced {
    events: Vec<Event>,
    operations: u64,
}
fn traced<T>(f: impl FnOnce() -> T) -> (T, Traced) {
    let before = count::operations();
    trace::start();
    let value = f();
    let events = trace::take();
    (
        value,
        Traced {
            events,
            operations: count::operations() - before,
        },
    )
}
fn exact(total: WorkTotal) -> u64 {
    total.exact().unwrap()
}

/// Settles each traced entry against the oracle and its ledger.
#[derive(Default)]
struct Checker {
    tally: Tally,
    entries: BTreeSet<&'static str>,
    stages: Vec<(&'static str, Vec<Event>)>,
    schedules: Vec<explicit::Schedule>,
    /// Full, independently derived schedules for a refusal sweep. Successful
    /// fixtures derive theirs from their inputs/results, never their traces.
    expected: Option<Vec<explicit::Schedule>>,
}
impl Checker {
    fn settle(&mut self, entry: &'static str, traced: Traced, charged: usize, rejected: u64, multiplies: bool) {
        if let Some(expected) = &self.expected {
            let (expected_entry, charges) = &expected[self.stages.len()];
            assert_eq!(*expected_entry, entry);
            explicit::verify(&traced.events, charges, entry);
        }
        let t = verify(&traced.events, entry);
        assert_eq!(t.charged, charged, "{entry}: oracle-recomputed charges against the ledger");
        assert_eq!(t.rejected, rejected, "{entry}: refused amounts against the ledger");
        if multiplies {
            assert!(t.muls > 0, "{entry}: no Work::mul in its trace");
        }
        // Additional temporal/coverage evidence, using producer instrumentation:
        // charge equality alone cannot detect an add executed before refusal or
        // one multiplication bypassing Work among several traced calls. Range
        // adds may stop partway, so this aggregate comparison excludes them.
        if t.ranges == 0 {
            assert_eq!(t.counted, traced.operations, "{entry}: counted operations");
        }
        self.tally.charged += t.charged;
        self.tally.rejected += t.rejected;
        self.tally.counted += t.counted;
        self.tally.muls += t.muls;
        self.tally.adds += t.adds;
        self.tally.ranges += t.ranges;
        self.entries.insert(entry);
        self.stages.push((entry, traced.events));
    }
    fn attempt(&mut self, entry: &'static str, traced: Traced, work: &WorkReport, multiplies: bool) {
        self.settle(entry, traced, work.charged, exact(work.rejected), multiplies);
    }
    fn stage(&mut self, entry: &'static str, traced: Traced, last: &mut WorkReport, now: WorkReport, multiplies: bool) {
        self.settle(
            entry,
            traced,
            now.charged - last.charged,
            exact(now.rejected) - exact(last.rejected),
            multiplies,
        );
        *last = now;
    }
    fn expect_stages(&mut self, start: usize, expected: Vec<explicit::Schedule>) {
        assert_eq!(self.stages.len() - start, expected.len());
        assert_eq!(self.schedules.len(), start);
        for ((entry, events), (expected_entry, charges)) in self.stages[start..].iter().zip(&expected) {
            assert_eq!(entry, expected_entry);
            explicit::verify(events, charges, entry);
        }
        self.schedules.extend(expected);
    }
    fn schedule(&self, entry: &str) -> &[usize] {
        &self.schedules.iter().rev().find(|(e, _)| *e == entry).unwrap().1
    }
    /// Adds another checker's tallies and traced stages to this one's.
    fn absorb(&mut self, other: Checker) {
        self.tally.charged += other.tally.charged;
        self.tally.rejected += other.tally.rejected;
        self.tally.counted += other.tally.counted;
        self.tally.muls += other.tally.muls;
        self.tally.adds += other.tally.adds;
        self.tally.ranges += other.tally.ranges;
        self.stages.extend(other.stages);
        self.schedules.extend(other.schedules);
    }
    fn events(&self, entry: &str) -> &[Event] {
        &self.stages.iter().rev().find(|(e, _)| *e == entry).unwrap().1
    }
}

fn p2(k: i32) -> f64 {
    2.0_f64.powi(k)
}
fn expansion(terms: &[f64]) -> Expansion {
    Expansion {
        terms: terms.to_vec(),
        ..Expansion::default()
    }
}
fn work(limit: usize, terms: usize, used: usize) -> Work {
    Work {
        limits: Limits {
            operations: limit,
            expansion_terms: terms,
            ..Limits::default()
        },
        used,
        rejected: WorkTotal::zero(),
    }
}

const ID: &str = "metered source";
const GAP: f64 = 0.125;
/// A source whose exact entries and loads carry tails, so that products have
/// several terms.
struct Source {
    stiffness: Vec<Vec<f64>>,
    force: Vec<f64>,
    contributions: Vec<StiffnessContribution>,
    terms: Vec<ForceContribution>,
    free: Vec<usize>,
    fixed: Vec<(usize, f64)>,
    /// The gap proof the functional chain retains and replays.
    gap: (usize, f64),
    /// RF design §3.3: the functional entry's congruence members.
    congruence: Vec<CongruenceMember>,
}
impl Source {
    /// Three DOFs: DOF 0 prescribed (with a tail bit), DOFs 1 and 2 one
    /// coupled block of order 2.
    fn new() -> Self {
        let entries: [((usize, usize), Vec<f64>); 7] = [
            ((0, 0), vec![300.0, p2(-47)]),
            ((0, 1), vec![-100.0, -p2(-50)]),
            ((1, 0), vec![-100.0, -p2(-50)]),
            ((1, 1), vec![200.0, 3.0 * p2(-50)]),
            ((1, 2), vec![-100.0, p2(-55)]),
            ((2, 1), vec![-100.0, p2(-55)]),
            ((2, 2), vec![100.0, -p2(-53)]),
        ];
        let loads = [(0, 1.0), (1, -0.75), (2, 12.5), (2, p2(-58))];
        Self::build(3, &entries, &loads, vec![1, 2], (0, 0.0625 + p2(-40)), (2, GAP))
    }
    /// RF design §2 (orders 3 and 4) through the oracle: eight DOFs, DOF 0
    /// prescribed (with a tail bit) and coupled to DOFs 1 and 4, DOFs 1 to 3
    /// one block of order 3 and DOFs 4 to 7 one block of order 4, every
    /// entry with a tail. Strictly diagonally dominant, hence SPD.
    fn higher() -> Self {
        let upper: [((usize, usize), Vec<f64>); 18] = [
            ((0, 0), vec![500.0, p2(-45)]),
            ((0, 1), vec![-50.0, -p2(-48)]),
            ((0, 4), vec![-40.0, p2(-50)]),
            ((1, 1), vec![200.0, 3.0 * p2(-50)]),
            ((1, 2), vec![-60.0, p2(-52)]),
            ((1, 3), vec![25.0, -p2(-51)]),
            ((2, 2), vec![180.0, -p2(-49)]),
            ((2, 3), vec![-45.0, p2(-53)]),
            ((3, 3), vec![150.0, p2(-47)]),
            ((4, 4), vec![260.0, -p2(-46)]),
            ((4, 5), vec![-70.0, p2(-51)]),
            ((4, 6), vec![15.0, -p2(-55)]),
            ((5, 5), vec![210.0, p2(-48)]),
            ((5, 6), vec![-55.0, -p2(-50)]),
            ((5, 7), vec![30.0, p2(-54)]),
            ((6, 6), vec![190.0, 3.0 * p2(-51)]),
            ((6, 7), vec![-65.0, p2(-49)]),
            ((7, 7), vec![170.0, -p2(-47)]),
        ];
        let mut entries = Vec::new();
        for ((row, col), values) in upper {
            entries.push(((row, col), values.clone()));
            if row != col {
                entries.push(((col, row), values));
            }
        }
        let loads = [(0, 1.0), (1, -0.75), (2, 12.5), (2, p2(-58)), (5, 3.25), (7, -2.0), (7, p2(-60))];
        Self::build(8, &entries, &loads, (1..8).collect(), (0, 0.0625 + p2(-40)), (6, GAP))
    }
    /// RF design §3.3 through the oracle: the census's (3,4,0) member
    /// (RF-SKEW-T-PIN-OFF-345) as twelve DOFs, its 144 rounded contributions
    /// then the soft rotational spring, with the member's exact congruence on
    /// the functional entry; every translation prescribed (one at a tail
    /// value), identified loads with a tail; free blocks [4, 2].
    fn congruence() -> Self {
        use super::functionals::congruence_tests::{member, OFF_345};
        let (member, rounded) = member(OFF_345);
        let mut entries = Vec::new();
        for (p, row) in rounded.iter().enumerate() {
            for (q, &value) in row.iter().enumerate() {
                entries.push(((p, q), vec![value]));
            }
        }
        entries.push(((3, 3), vec![f64::from_bits(0x3f81_b1d9_2b7f_e08b)]));
        let loads = [
            (9, f64::from_bits(0x3ea1_6505_a7da_8610)),
            (10, f64::from_bits(0x3ea7_315c_dfce_0816)),
            (10, p2(-80)),
        ];
        let mut s = Self::build(12, &entries, &loads, vec![3, 4, 5, 9, 10, 11], (0, p2(-40)), (9, GAP));
        s.fixed.extend([(1, 0.0), (2, 0.0), (6, 0.0), (7, 0.0), (8, 0.0)]);
        s.congruence = vec![member];
        s
    }
    fn build(
        n: usize,
        entries: &[((usize, usize), Vec<f64>)],
        loads: &[(usize, f64)],
        free: Vec<usize>,
        fixed: (usize, f64),
        gap: (usize, f64),
    ) -> Self {
        let mut stiffness = vec![vec![0.0; n]; n];
        let mut contributions = Vec::new();
        for ((row, col), values) in entries {
            for &value in values {
                contributions.push(StiffnessContribution { row: *row, col: *col, value });
                stiffness[*row][*col] += value;
            }
        }
        let mut force = vec![0.0; n];
        let terms = loads
            .iter()
            .enumerate()
            .map(|(i, &(dof, value))| {
                force[dof] += value;
                ForceContribution {
                    source: format!("load {i}"),
                    dof,
                    value,
                }
            })
            .collect();
        Self {
            stiffness,
            force,
            contributions,
            terms,
            free,
            fixed: vec![fixed],
            gap,
            congruence: Vec::new(),
        }
    }
    fn dofs(&self) -> usize {
        self.force.len()
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
    fn basis(&self) -> ForceBasis<'_> {
        ForceBasis::IdentifiedContributions(&self.terms)
    }
}
/// `descriptors()`'s three recipes over the higher-order source: an affine
/// form over both blocks, a displacement in the order-4 block and DOF 0's
/// source reaction.
fn higher_descriptors() -> Vec<FunctionalDescriptor> {
    let mut ds = descriptors();
    ds[0].terms.extend([
        AffineTerm {
            dof: 3,
            products: vec![vec![7.5, 1.0 - p2(-50)]],
        },
        AffineTerm {
            dof: 5,
            products: vec![vec![-2.0], vec![p2(-48), 5.0]],
        },
        AffineTerm {
            dof: 7,
            products: vec![vec![0.25, 3.0 + p2(-49)]],
        },
    ]);
    ds[1].key.quantity = FunctionalQuantity::NodeDisplacement {
        node: "n6".into(),
        dof: 6,
    };
    ds[1].terms[0].dof = 6;
    ds[2].terms = vec![
        AffineTerm {
            dof: 0,
            products: vec![vec![500.0], vec![p2(-45)]],
        },
        AffineTerm {
            dof: 1,
            products: vec![vec![-50.0], vec![-p2(-48)]],
        },
        AffineTerm {
            dof: 4,
            products: vec![vec![-40.0], vec![p2(-50)]],
        },
    ];
    ds
}
/// `descriptors()`'s three recipes over the congruence source: an affine
/// form over both blocks, a displacement in the order-4 block and the
/// prescribed DOF 0's source reaction.
fn congruence_descriptors() -> Vec<FunctionalDescriptor> {
    let mut ds = descriptors();
    ds[0].terms[0].dof = 4;
    ds[0].terms[1].dof = 10;
    ds[0].terms.push(AffineTerm {
        dof: 5,
        products: vec![vec![0.25, 3.0 + p2(-49)]],
    });
    ds[1].key.quantity = FunctionalQuantity::NodeDisplacement {
        node: "n10".into(),
        dof: 10,
    };
    ds[1].terms[0].dof = 10;
    ds[2].terms[1].dof = 3;
    ds
}
fn descriptors() -> Vec<FunctionalDescriptor> {
    let key = |quantity, unit, convention| FunctionalKey {
        case_id: "metered".into(),
        quantity,
        unit,
        convention,
    };
    vec![
        FunctionalDescriptor {
            key: key(
                FunctionalQuantity::DeclaredAffine {
                    label: "affine".into(),
                },
                FunctionalUnit::Dimensionless,
                FunctionalConvention::DeclaredAffine,
            ),
            offset: vec![vec![0.5, 0.25 + p2(-30)]],
            terms: vec![
                AffineTerm {
                    dof: 1,
                    products: vec![vec![100.0, 1.0 + p2(-52)], vec![p2(-50)]],
                },
                AffineTerm {
                    dof: 2,
                    products: vec![vec![-100.0], vec![p2(-55), 3.0, -1.0]],
                },
            ],
        },
        FunctionalDescriptor {
            key: key(
                FunctionalQuantity::NodeDisplacement {
                    node: "n2".into(),
                    dof: 2,
                },
                FunctionalUnit::Metre,
                FunctionalConvention::DeclaredAffine,
            ),
            offset: vec![],
            terms: vec![AffineTerm {
                dof: 2,
                products: vec![vec![1.0]],
            }],
        },
        FunctionalDescriptor {
            key: key(
                FunctionalQuantity::SourceReaction { dof: 0 },
                FunctionalUnit::Newton,
                FunctionalConvention::SourceEquilibrium,
            ),
            offset: vec![vec![-1.0]],
            terms: vec![
                AffineTerm {
                    dof: 0,
                    products: vec![vec![300.0], vec![p2(-47)]],
                },
                AffineTerm {
                    dof: 1,
                    products: vec![vec![-100.0], vec![-p2(-50)]],
                },
            ],
        },
    ]
}

/// Member-shaped R-A fixtures. The free graph is unchanged from `higher`;
/// one extra prescribed -0 DOF exercises B2 alongside B1. Sources cover both
/// contributing blocks and the nonzero prescribed DOF independently.
fn hinted_fixture() -> (Source, Vec<FunctionalDescriptor>, Vec<Option<u32>>) {
    let mut source = Source::higher();
    for row in &mut source.stiffness {
        row.push(0.0);
    }
    source.stiffness.push(vec![0.0; 9]);
    source.force.push(0.0);
    source.fixed.push((8, -0.0));
    let term = |dof, products| AffineTerm { dof, products };
    let base = vec![
        term(1, vec![vec![2.5, 0.75]]),
        term(8, vec![vec![4.5, 0.5]]),
    ];
    let end = |member: &str, row: u8, terms, offset| FunctionalDescriptor {
        key: FunctionalKey {
            case_id: "metered".into(),
            quantity: FunctionalQuantity::MemberEnd {
                member: member.into(),
                end: MemberEnd::I,
                row,
            },
            unit: if row < 3 {
                FunctionalUnit::Newton
            } else {
                FunctionalUnit::NewtonMetre
            },
            convention: FunctionalConvention::NodeOnElement,
        },
        terms,
        offset,
    };
    let mut ds = vec![
        end("beam", 0, base.clone(), vec![]),
        end("beam", 1, vec![term(8, vec![vec![2.5, 3.5]])], vec![]),
        end("beam", 2, vec![term(1, vec![vec![2.5, 0.5, 0.25]])], vec![]),
        end("beam", 3, vec![term(1, vec![vec![1.0, 2.5]])], vec![]),
        end("beam", 4, vec![term(1, vec![vec![2.0, 0.5]])], vec![]),
        end(
            "beam",
            5,
            vec![term(0, vec![vec![2.0]]), term(1, vec![vec![2.5]])],
            vec![],
        ),
        end(
            "two blocks",
            0,
            vec![term(1, vec![vec![2.5]]), term(4, vec![vec![3.5]])],
            vec![],
        ),
        end("offset", 0, base.clone(), vec![vec![0.25]]),
    ];
    let mut wrong_case = end("beam", 0, base.clone(), vec![]);
    wrong_case.key.case_id = "other case".into();
    ds.push(wrong_case);
    let mut j_end = end("beam", 0, base, vec![]);
    if let FunctionalQuantity::MemberEnd { end, .. } = &mut j_end.key.quantity {
        *end = MemberEnd::J;
    }
    ds.push(j_end);
    let mut hints = vec![None; ds.len()];
    let section = |s: usize, station: f64| {
        let mut d = ds[s].clone();
        let FunctionalQuantity::MemberEnd { member, row, .. } = &d.key.quantity else {
            unreachable!()
        };
        d.key.quantity = FunctionalQuantity::MemberSection {
            member: member.clone(),
            station_bits: station.to_bits(),
            component: *row,
        };
        d.key.convention = FunctionalConvention::SectionLocal;
        for product in d
            .offset
            .iter_mut()
            .chain(d.terms.iter_mut().flat_map(|t| &mut t.products))
        {
            product.push(-1.0);
        }
        d
    };
    // Every static/dynamic source condition has an otherwise matching section.
    let mut sections: Vec<_> = (0..8).map(|i| (section(i, 0.0), Some(i as u32))).collect();
    // Extra zeroed products: scan stops at the first row in one case, and
    // after declining row 0 in the other. Atoms after the zero are unrestricted.
    for (station, r) in [(0.1, 0), (0.2, 1)] {
        let mut d = section(1, station);
        d.terms.extend(ds[r].terms.iter().cloned().map(|mut t| {
            for p in &mut t.products {
                p.extend([-1.0, -0.0, 7.5]);
            }
            t
        }));
        sections.push((d, Some(1)));
    }
    let mut atom = section(0, 0.3);
    atom.terms[0].products[0][0] = 2.75;
    sections.push((atom, Some(0)));
    let mut dof = section(0, 0.4);
    dof.terms[0].dof = 2;
    sections.push((dof, Some(0)));
    let mut missing = section(0, 0.5);
    missing.terms.pop();
    sections.push((missing, Some(0)));
    let mut offset = section(0, 0.6);
    offset.offset.push(vec![0.25]);
    sections.push((offset, Some(0)));
    let mut extra = section(1, 0.7);
    extra.terms.extend(ds[0].terms.clone()); // no zero: every search candidate declines
    sections.push((extra, Some(1)));
    let mut prefix = section(1, 0.8);
    prefix
        .terms
        .extend(ds[0].terms.iter().cloned().map(|mut t| {
            for p in &mut t.products {
                p.extend([2.0, 0.0]);
            }
            t
        }));
    sections.push((prefix, Some(1)));
    // Missing, component, member, case, J-end, later and out-of-range hints.
    for (i, hint) in [
        None,
        Some(1),
        Some(6),
        Some(8),
        Some(9),
        Some(32),
        Some(u32::MAX),
    ]
    .into_iter()
    .enumerate()
    {
        sections.push((section(0, 1.0 + i as f64), hint));
    }
    for (d, hint) in sections {
        ds.push(d);
        hints.push(hint);
    }
    (source, ds, hints)
}

/// The functional entry, preparation through its same-ledger replay, on one
/// ledger of `limit`; every stage settled. Stops at the first failing stage.
/// Returns the prepared Context's block orders.
fn functional_chain(
    s: &Source,
    ds: &[FunctionalDescriptor],
    hints: &[Option<u32>],
    limit: usize,
    check: &mut Checker,
) -> Result<Vec<usize>, Error> {
    let start = check.stages.len();
    let (gap_dof, gap) = s.gap;
    let system = s.system();
    let mut budget = AttemptBudget::new(Limits {
        operations: limit,
        ..Limits::default()
    });
    let mut last = budget.report();
    let (a, t) = traced(|| Context::prepare_with_budget(&system, ID, s.basis(), &s.congruence, &mut budget));
    check.stage("Context::prepare_with_budget", t, &mut last, budget.report(), a.result.is_ok());
    let c = a.result?;
    let (a, t) = traced(|| c.solve_with_budget(&mut budget));
    check.stage("Context::solve_with_budget", t, &mut last, budget.report(), a.result.is_ok());
    let r = a.result?;
    let entry = if hints.is_empty() { "FunctionalPlan::new" } else { "FunctionalPlan::new_with_hints" };
    let rule = |i: usize| hints[i];
    let rule: Option<HintRule<'_>> = (!hints.is_empty()).then_some(&rule);
    let (a, t) = traced(|| match rule {
        Some(rule) => FunctionalPlan::new_with_hints(&c, ds, rule, &mut budget),
        None => FunctionalPlan::new(&c, ds, &mut budget),
    });
    check.stage(entry, t, &mut last, budget.report(), false);
    let plan = a.result?;
    let (a, t) = traced(|| r.evaluate_functionals(&plan, &mut budget));
    check.stage("Response::evaluate_functionals", t, &mut last, budget.report(), a.result.is_ok());
    let set = a.result?;
    let (a, t) = traced(|| r.gap_proof_with_budget(gap_dof, 1, gap, &mut budget));
    check.stage("Response::gap_proof_with_budget", t, &mut last, budget.report(), a.result.is_ok());
    let proof = a.result?;
    let mut leaves = Vec::new();
    for dof in 0..s.dofs() {
        for quantity in [Quantity::Displacement, Quantity::Reaction] {
            let (a, t) = traced(|| r.project_with_budget(quantity, dof, 1e-9, &mut budget));
            // Every projection multiplies except an exact zero's (the free rows'
            // reactions here).
            let multiplies = a.result.as_ref().is_ok_and(|p| p.basis() != ProjectionBasis::ExactZero);
            check.stage("Response::project_with_budget", t, &mut last, budget.report(), multiplies);
            leaves.push(a.result?);
        }
    }
    let mut projections = Vec::new();
    for i in 0..set.len() {
        let (a, t) = traced(|| set.project(i, 1e-9, &mut budget));
        let multiplies = a.result.as_ref().is_ok_and(|p| p.basis() != ProjectionBasis::ExactZero);
        check.stage("FunctionalSet::project", t, &mut last, budget.report(), multiplies);
        projections.push(a.result?);
    }
    let (a, t) = traced(|| set.retain(&[proof], &leaves, &projections, &mut budget));
    check.stage("FunctionalSet::retain", t, &mut last, budget.report(), false);
    let retained = a.result?;
    let (a, t) = traced(|| retained.check_binding_with_budget(&system, ID, s.basis(), &s.congruence, ds, &mut budget));
    check.stage("RetainedFunctionalSet::check_binding_with_budget", t, &mut last, budget.report(), false);
    a.result?;
    let (a, t) = traced(|| {
        retained.replay_against(&system, ID, s.basis(), &s.congruence, ds, rule, &[(gap_dof, 1, gap)], 1e-9, &mut budget)
    });
    check.stage("RetainedFunctionalSet::replay_against", t, &mut last, budget.report(), a.result.is_ok());
    assert_eq!(a.result?.functionals(), ds.len());
    check.expect_stages(start, explicit::functional(s, &r, &retained, hints));
    Ok(c.blocks.iter().map(Vec::len).collect())
}

/// Every limit just below and at each accepted step of the traced
/// multiplications (every `stride`-th), on their absolute ledger totals.
fn step_limits(events: &[Event], stride: usize) -> Vec<usize> {
    let mut limits = BTreeSet::new();
    let muls = events.iter().filter_map(|e| match e {
        Event::Mul { ledger, a, b, .. } => Some((ledger, a, b)),
        _ => None,
    });
    for (ledger, a, b) in muls.step_by(stride) {
        let unlimited = Ledger {
            limit: usize::MAX,
            ..*ledger
        };
        for boundary in oracle::mul(a, b, &unlimited).boundaries {
            limits.insert(boundary - 1);
            limits.insert(boundary);
        }
    }
    limits.into_iter().collect()
}
/// Reruns a metered entry at every step limit of its full trace: the oracle
/// must account for each refusal, and a limit below the full charge refuses
/// with `Budget` and returns nothing.
fn sweep(
    check: &mut Checker,
    entry: &'static str,
    stride: usize,
    run: impl Fn(usize) -> (Outcome, WorkReport),
) {
    let ((outcome, full), t) = traced(|| run(usize::MAX));
    assert_eq!(outcome, Outcome::Ok, "{entry}");
    let limits = step_limits(&t.events, stride);
    let expected = check.schedule(entry).to_vec();
    explicit::verify(&t.events, &expected, entry);
    assert!(!limits.is_empty(), "{entry}: no multiplication to sweep");
    for limit in limits {
        let ((outcome, work), t) = traced(|| run(limit));
        assert_eq!(work.limit, limit.min(Limits::default().operations));
        assert_eq!(
            outcome == Outcome::Budget,
            limit < full.charged,
            "{entry} at limit {limit}: {outcome:?}"
        );
        let start = check.stages.len();
        check.attempt(entry, t, &work, false);
        check.expect_stages(start, vec![(entry, expected.clone())]);
    }
}

/// Every entry: the functional set, the legacy Context, every `Response`
/// method and the legacy `RetainedResponse` replay; with refusal sweeps when
/// `sweeps` is set.
fn every_entry(check: &mut Checker, sweeps: bool) {
    functional_chain(&Source::new(), &descriptors(), &[], Limits::default().operations, check).unwrap();
    // RF design §2: the order-3 and order-4 arithmetic (preparation, solve,
    // reactions, evaluation and replay) through the same oracle.
    let higher = Source::higher();
    let mut blocks = Checker::default();
    let orders = functional_chain(&higher, &higher_descriptors(), &[], Limits::default().operations, &mut blocks);
    assert_eq!(orders.unwrap(), [3, 4]);
    check.absorb(blocks);
    // RF design §3.3: a congruence member's exact formation, in the
    // functional entry's preparation and again in its replay, through the
    // same oracle.
    let mut members = Checker::default();
    let orders = functional_chain(&Source::congruence(), &congruence_descriptors(), &[], CONGRUENCE_LEDGER, &mut members);
    assert_eq!(orders.unwrap(), [4, 2]);
    check.absorb(members);
    let (source, ds, hints) = hinted_fixture();
    let mut hinted = Checker::default();
    functional_chain(&source, &ds, &hints, Limits::default().operations, &mut hinted).unwrap();
    if sweeps { sweep_hinted_explicit(check, &source, &ds, &hints, &hinted); }
    check.absorb(hinted);
    let start = check.stages.len();
    let s = Source::new();
    let system = s.system();
    let (c, t) = traced(|| Context::new(&system, ID, s.basis(), Limits::default()));
    let c = c.unwrap();
    check.settle("Context::new", t, c.preparation_operations, 0, true);
    let (r, t) = traced(|| c.solve());
    let r = r.unwrap();
    check.settle("Context::solve", t, r.operations() - c.preparation_operations, 0, true);
    let mut leaves = Vec::new();
    for dof in 0..3 {
        let (p, t) = traced(|| r.project_displacement(dof, 1e-9, usize::MAX));
        check.attempt("Response::project_displacement", t, &p.work, true);
        leaves.push(p.result.unwrap());
        let (p, t) = traced(|| r.project_reaction(dof, 1e-9, usize::MAX));
        check.attempt("Response::project_reaction", t, &p.work, dof == 0);
        leaves.push(p.result.unwrap());
    }
    let (a, t) = traced(|| r.gap_sign_with_work(2, 1, GAP, usize::MAX));
    check.attempt("Response::gap_sign_with_work", t, &a.work, true);
    a.result.unwrap();
    let (a, t) = traced(|| r.signed_reaction_with_work(0, 1, usize::MAX));
    check.attempt("Response::signed_reaction_with_work", t, &a.work, false);
    assert_ne!(a.result.unwrap(), Sign::Zero);
    let candidate: Vec<f64> = (0..3).map(|d| leaves[2 * d].value()).collect();
    let (a, t) = traced(|| r.verify_candidate_with_work(&candidate, usize::MAX));
    check.attempt("Response::verify_candidate_with_work", t, &a.work, true);
    a.result.unwrap();
    let (a, t) = traced(|| r.exact_representable_candidate_with_work(usize::MAX));
    check.attempt("Response::exact_representable_candidate_with_work", t, &a.work, true);
    a.result.unwrap();
    let (a, t) = traced(|| r.gap_proof_with_work(2, 1, GAP, usize::MAX));
    check.attempt("Response::gap_proof_with_work", t, &a.work, true);
    let proof = a.result.unwrap();
    let (a, t) = traced(|| r.retain_with_work(&[proof], &leaves, usize::MAX));
    check.attempt("Response::retain_with_work", t, &a.work, false);
    let retained = a.result.unwrap();
    let replay = |limit| {
        retained.replay_against(&system, ID, s.basis(), &[(2, 1, GAP)], 1e-9, Limits::default(), limit)
    };
    let (a, t) = traced(|| replay(usize::MAX));
    check.attempt("RetainedResponse::replay_against", t, &a.work, true);
    assert_eq!(a.result.unwrap().dofs(), 3);
    check.expect_stages(start, explicit::legacy(&s, &r, &retained));
    if !sweeps {
        return;
    }
    let report = |a: Attempt<_>| (trace::outcome(&a.result), a.work);
    sweep(check, "Response::gap_sign_with_work", 1, |l| {
        report(r.gap_sign_with_work(2, 1, GAP, l).map_unit())
    });
    sweep(check, "Response::project_displacement", 1, |l| {
        report(r.project_displacement(1, 1e-9, l).map_unit())
    });
    sweep(check, "Response::verify_candidate_with_work", 2, |l| {
        report(r.verify_candidate_with_work(&candidate, l).map_unit())
    });
    sweep(check, "Response::exact_representable_candidate_with_work", 3, |l| {
        report(r.exact_representable_candidate_with_work(l).map_unit())
    });
    sweep(check, "Response::gap_proof_with_work", 1, |l| {
        report(r.gap_proof_with_work(2, 1, GAP, l).map_unit())
    });
    sweep(check, "RetainedResponse::replay_against", 7, |l| report(replay(l).map_unit()));
    // The functional entry: refusals in preparation and in the replay, on the
    // attempt's one ledger (every earlier stage fits below these limits), for
    // the order-2 source and the order-3/4 source.
    for (source, ds, strides, ledger) in [
        (Source::new(), descriptors(), [1, 9], Limits::default().operations),
        (Source::higher(), higher_descriptors(), HIGHER_STRIDES, Limits::default().operations),
        (Source::congruence(), congruence_descriptors(), CONGRUENCE_STRIDES, CONGRUENCE_LEDGER),
    ] {
        let mut full = Checker::default();
        functional_chain(&source, &ds, &[], ledger, &mut full).unwrap();
        for (entry, stride) in [
            ("Context::prepare_with_budget", strides[0]),
            ("RetainedFunctionalSet::replay_against", strides[1]),
        ] {
            sweep_functional(check, &source, &ds, entry, &full, stride);
        }
    }
}
/// The order-3/4 source's sweep strides: every step of every multiplication
/// of its preparation (the block arithmetic), and of every 29th of its replay
/// (stride 1 there costs about a minute).
const HIGHER_STRIDES: [usize; 2] = [1, 29];
/// The congruence source's sweep strides: every 5th multiplication of its
/// preparation and every 61st of its replay, both of which form the member's
/// exact congruence (stride 1 there would rerun the chain about 52,000 times).
const CONGRUENCE_STRIDES: [usize; 2] = [5, 61];
/// The congruence source's ledger: its chain (preparation through replay,
/// with identified loads and a prescribed tail) needs more than the default
/// 2,000,000; this is the product's per-case source limit.
const CONGRUENCE_LEDGER: usize = 8_000_000;
/// Reruns the functional chain at every step limit of `entry`'s traced
/// multiplications: each refusal stops in `entry` with `Budget`.
fn sweep_functional(
    check: &mut Checker,
    source: &Source,
    ds: &[FunctionalDescriptor],
    entry: &'static str,
    full: &Checker,
    stride: usize,
) {
    let limits = step_limits(full.events(entry), stride);
    assert!(!limits.is_empty(), "{entry}: no multiplication to sweep");
    for limit in limits {
        let mut fresh = Checker { expected: Some(full.schedules.clone()), ..Checker::default() };
        let refused = functional_chain(source, ds, &[], limit, &mut fresh);
        let (last, events) = fresh.stages.last().unwrap();
        assert_eq!(*last, entry, "limit {limit} stopped in {last}");
        if refused.is_ok() {
            assert_eq!(*last, "RetainedFunctionalSet::replay_against");
        } else {
            assert_eq!(refused, Err(Error::Budget), "{entry} at limit {limit}");
            assert!(!events.is_empty());
        }
        check.tally.muls += fresh.tally.muls;
        check.tally.adds += fresh.tally.adds;
    }
}
/// Refuse immediately before and at every explicit charge in the hinted
/// plan, live evaluation and replay. Amounts here select ledger boundaries;
/// only `explicit`'s independently derived schedules supply expectations.
fn sweep_hinted_explicit(
    check: &mut Checker,
    source: &Source,
    ds: &[FunctionalDescriptor],
    hints: &[Option<u32>],
    full: &Checker,
) {
    let mut used = 0;
    let mut limits = BTreeSet::new();
    for (entry, events) in &full.stages {
        let selected = matches!(
            *entry,
            "FunctionalPlan::new_with_hints"
                | "Response::evaluate_functionals"
                | "RetainedFunctionalSet::replay_against"
        );
        for event in events {
            match event {
                Event::Charge {
                    amount,
                    accepted: true,
                } => {
                    used += amount;
                    if selected && *amount > 0 {
                        limits.extend([used - 1, used]);
                    }
                }
                Event::Add { charged, .. } | Event::Mul { charged, .. } => used += charged,
                _ => panic!("full fixture refused"),
            }
        }
    }
    for limit in limits {
        let mut fresh = Checker {
            expected: Some(full.schedules.clone()),
            ..Checker::default()
        };
        let result = functional_chain(source, ds, hints, limit, &mut fresh);
        if limit < used {
            assert_eq!(result, Err(Error::Budget), "hinted chain at {limit}");
        } else {
            assert!(
                result.is_ok(),
                "hinted chain at full charge {limit}: {result:?}"
            );
        }
        check.tally.muls += fresh.tally.muls;
        check.tally.adds += fresh.tally.adds;
    }
}

impl<T> Attempt<T> {
    fn map_unit(self) -> Attempt<()> {
        Attempt {
            result: self.result.map(|_| ()),
            work: self.work,
        }
    }
}

#[test]
fn metered_mul_charges_match_the_independent_oracle() {
    // Direct products: varied lengths, cancellations that shorten the output,
    // refusals (limit and expansion length) and range failures.
    let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    let mut random_value = |spread: u64| {
        let bits = next();
        let exponent = (bits % (2 * spread + 1)) as i64 - spread as i64;
        let mantissa = next() & ((1 << 52) - 1);
        let sign = (bits >> 63) << 63;
        f64::from_bits(sign | (((exponent + 1023) as u64) << 52) | mantissa)
    };
    let mut random_expansion = |values: usize, spread: u64| {
        let mut terms = Vec::new();
        for _ in 0..values {
            terms = oracle::grow(&terms, random_value(spread)).unwrap();
        }
        terms
    };
    let mut cases: Vec<(Vec<f64>, Vec<f64>)> = vec![
        (vec![1.5], vec![1.25]),
        (vec![-p2(-60), 1.0], vec![p2(-60), 1.0]),
        (vec![p2(-80), -p2(-30), 1.0 + p2(-52)], vec![3.0 * p2(-70), 1.0 + p2(-51)]),
        (vec![f64::MAX], vec![2.0]),
        // The third pair reaches a near-MAX grow whose virtual operand
        // overflows before the final (overflowing) pair could be processed.
        (vec![1.0, p2(60)], vec![-3.0 * p2(910), f64::MAX]),
        (vec![f64::from_bits(1)], vec![0.5]),
        (vec![f64::from_bits(5)], vec![f64::MAX]),
    ];
    for (la, lb, spread) in [(1, 1, 60), (1, 4, 60), (4, 1, 200), (3, 5, 30), (6, 6, 300), (9, 7, 40), (12, 12, 900)] {
        cases.push((random_expansion(la, spread), random_expansion(lb, spread)));
    }
    let mut shortened = 0;
    let mut refused_by_length = 0;
    let mut refused_by_limit = 0;
    let mut ranges = 0;
    for (a, b) in &cases {
        let unlimited = Ledger { used: 0, limit: usize::MAX, terms: 256 };
        let full = oracle::mul(a, b, &unlimited);
        shortened += usize::from(full.shrinks > 0);
        ranges += usize::from(full.outcome == Outcome::Range);
        let mut limits = vec![usize::MAX, 0];
        limits.extend(full.boundaries.iter().step_by(3).map(|b| b - 1));
        for limit in limits {
            for terms in [256, 4] {
                for used in [0, 1_000] {
                    let mut w = work(limit.saturating_add(used), terms, used);
                    let (result, t) = traced(|| w.mul(&expansion(a), &expansion(b)));
                    let tally = verify(&t.events, "Work::mul");
                    assert_eq!(tally.muls, 1);
                    assert_eq!(tally.charged, w.used - used);
                    assert_eq!(tally.rejected, exact(w.rejected));
                    let o = oracle::mul(a, b, &Ledger { used, limit: limit.saturating_add(used), terms });
                    assert_eq!(trace::outcome(&result), o.outcome);
                    if o.outcome != Outcome::Range {
                        assert_eq!(t.operations, o.counted);
                    }
                    refused_by_length += usize::from(o.outcome == Outcome::Budget && o.rejected == 0);
                    refused_by_limit += usize::from(o.outcome == Outcome::Budget && o.rejected > 0);
                }
            }
        }
    }
    assert!(shortened >= 1, "a cancellation must shorten some output");
    assert!(refused_by_length >= 1 && refused_by_limit >= 1 && ranges >= 2);

    // Through every entry, with refusal sweeps.
    let mut check = Checker::default();
    every_entry(&mut check, true);
    assert_eq!(check.schedules.len(), check.stages.len(), "every traced entry has an explicit-charge schedule");
    for entry in [
        "Context::prepare_with_budget",
        "Context::solve_with_budget",
        "Response::evaluate_functionals",
        "Response::gap_proof_with_budget",
        "Response::project_with_budget",
        "FunctionalSet::project",
        "FunctionalSet::retain",
        "RetainedFunctionalSet::replay_against",
        "Context::new",
        "Context::solve",
        "Response::project_displacement",
        "Response::project_reaction",
        "Response::gap_sign_with_work",
        "Response::signed_reaction_with_work",
        "Response::verify_candidate_with_work",
        "Response::exact_representable_candidate_with_work",
        "Response::gap_proof_with_work",
        "Response::retain_with_work",
        "RetainedResponse::replay_against",
    ] {
        assert!(check.entries.contains(entry), "{entry} not exercised");
    }
    eprintln!(
        "oracle: {} traced entry runs, {} Work::mul and {} Work::add calls recomputed",
        check.stages.len(),
        check.tally.muls,
        check.tally.adds
    );
    assert!(check.tally.muls > 100);
}

#[test]
fn metered_oracle_reproduces_near_overflow_two_sum_refusal() {
    for sign in [1.0, -1.0] {
        let (existing, incoming) = (sign * f64::MAX, sign * (-3.0 * p2(970)));
        let sum = existing + incoming;
        assert!(sum.is_finite());
        assert!((sum - incoming).is_infinite());
        assert_eq!(oracle::grow(&[existing], incoming), Err(1));
        let mut out = expansion(&[existing]);
        let mut w = work(usize::MAX, 256, 0);
        assert!(matches!(w.add(&mut out, incoming), Err(Error::Arithmetic(_))));
        assert_eq!(w.used, 7); // reserve the one six-operation TwoSum + final term
        assert_eq!(exact(w.rejected), 0);
        assert!(same_bits(&out.terms, &[existing]));
    }
}

#[test]
fn metered_refusal_precedes_the_refused_step() {
    let a = [p2(-70), -p2(-30), 1.5];
    let b = [3.0 * p2(-80), 1.25 + p2(-50)];
    let unlimited = Ledger { used: 0, limit: usize::MAX, terms: 256 };
    let full = oracle::mul(&a, &b, &unlimited);
    assert_eq!(full.outcome, Outcome::Ok);
    // Every limit set between two steps: each refused step is the next one.
    let mut previous = 0;
    let mut refused_adds = 0;
    for &boundary in &full.boundaries {
        let step = boundary - previous;
        let limit = boundary - 1;
        let mut w = work(limit, 256, 0);
        count::reset();
        let result = w.mul(&expansion(&a), &expansion(&b));
        let prefix = oracle::mul(&a, &b, &Ledger { limit, ..unlimited });
        assert_eq!(result.unwrap_err(), Error::Budget, "limit {limit}");
        assert_eq!(w.used, previous, "limit {limit}: only the earlier steps were charged");
        assert_eq!(exact(w.rejected), step as u64, "limit {limit}: the refused amount");
        assert_eq!((prefix.charged, prefix.rejected), (previous, step as u64));
        // Temporal refusal enforcement needs the executed-operation counter
        // as well as independent charge equality: an add run before its
        // refused charge would leave the same ledger but exceed this prefix.
        assert_eq!(count::operations(), prefix.counted, "limit {limit}: executed operations");
        assert_eq!(count::totals().shortfalls(), 0);
        refused_adds += usize::from(step != oracle::PAIR && step > 1);
        previous = boundary;
    }
    assert!(refused_adds >= 4, "adds refused on a nonempty output");

    // The same through an entry: the attempt returns `Budget` and nothing else.
    let s = Source::new();
    let c = Context::new(&s.system(), ID, s.basis(), Limits::default()).unwrap();
    let r = c.solve().unwrap();
    let (full, t) = traced(|| r.gap_sign_with_work(2, 1, GAP, usize::MAX));
    let (ledger, a, b) = t
        .events
        .iter()
        .find_map(|e| match e {
            Event::Mul { ledger, a, b, .. } => Some((*ledger, a.clone(), b.clone())),
            _ => None,
        })
        .unwrap();
    let steps = oracle::mul(&a, &b, &Ledger { limit: usize::MAX, ..ledger }).boundaries;
    let mut previous = ledger.used;
    for boundary in steps {
        count::reset();
        let (refused, t) = traced(|| r.gap_sign_with_work(2, 1, GAP, boundary - 1));
        assert_eq!(refused.result, Err(Error::Budget));
        assert_eq!(refused.work.charged, previous);
        assert_eq!(exact(refused.work.rejected), (boundary - previous) as u64);
        explicit::verify(&t.events, &[12], "Response::gap_sign_with_work");
        let tally = verify(&t.events, "Response::gap_sign_with_work");
        assert_eq!((tally.charged, tally.rejected), (previous, (boundary - previous) as u64));
        assert_eq!(count::operations(), tally.counted);
        previous = boundary;
    }
    assert!(full.result.is_ok() && full.work.charged > previous);
}

#[test]
fn metered_mul_keeps_the_expansion_term_refusal() {
    // Each pair adds two terms far from every other pair's.
    let t = 1.0 + p2(-52);
    let a = [t * p2(-400), t, t * p2(400)];
    let b = [t * p2(-130), t, t * p2(130)];
    let unlimited = Ledger { used: 0, limit: usize::MAX, terms: 256 };
    let whole = oracle::mul(&a, &b, &unlimited);
    assert_eq!(whole.outcome, Outcome::Ok);
    let limit_terms = 6;
    assert!(whole.product.len() > limit_terms);
    // Positive control: with room, the product completes with every term.
    let mut roomy = work(usize::MAX, 256, 0);
    let product = roomy.mul(&expansion(&a), &expansion(&b)).unwrap();
    assert!(same_bits(&product.terms, &whole.product));
    // At E = 6 the product would pass E terms: `Budget`, refused before any
    // charge of the pair that could pass E, and no reservation is recorded.
    let mut tight = work(usize::MAX, limit_terms, 0);
    count::reset();
    let (result, trace) = traced(|| tight.mul(&expansion(&a), &expansion(&b)));
    assert_eq!(result.unwrap_err(), Error::Budget);
    assert_eq!(exact(tight.rejected), 0);
    let o = oracle::mul(&a, &b, &Ledger { terms: limit_terms, ..unlimited });
    assert_eq!((o.outcome, o.rejected), (Outcome::Budget, 0));
    assert_eq!(tight.used, o.charged);
    assert!(o.charged < whole.charged);
    assert!(o.longest <= limit_terms);
    assert_eq!(verify(&trace.events, "Work::mul").charged, tight.used);
    assert_eq!(count::totals().shortfalls(), 0);
}

#[test]
fn metered_operation_count_control_has_no_shortfall() {
    // The maintained control over every entry (§5.4): every Work::mul and
    // Work::add call charged at least the operations it executed, and the
    // counter agrees with the oracle's count of the same arithmetic.
    count::reset();
    let mut check = Checker::default();
    every_entry(&mut check, false);
    let totals = count::totals();
    eprintln!("exact operation count over every entry: {totals:?}");
    assert_eq!(totals.shortfalls(), 0);
    assert!(totals.mul.calls > 0 && totals.add.calls > 0 && totals.pairs > 0);
    assert!(totals.mul.min_slack.unwrap() >= 0 && totals.add.min_slack.unwrap() >= 0);
    assert!(totals.min_pair_slack.unwrap() >= (oracle::PAIR - 66) as i64);
    assert!(totals.max_pair_two_product <= 66);
    assert_eq!(check.tally.ranges, 0);
    assert_eq!(totals.mul.counted + totals.add.counted, check.tally.counted);
    assert_eq!(totals.mul.calls as usize, check.tally.muls);
    assert_eq!(totals.add.calls as usize, check.tally.adds);
    assert_eq!(totals.mul.charged + totals.add.charged + explicit(&check), check.tally.charged as u64);
}
fn explicit(check: &Checker) -> u64 {
    check
        .stages
        .iter()
        .flat_map(|(_, events)| events)
        .map(|e| match e {
            Event::Charge { amount, accepted: true } => *amount as u64,
            _ => 0,
        })
        .sum()
}

#[test]
fn metered_pair_constant_covers_the_worst_case_exponents() {
    // Subnormal and extreme operands, both signs: one pair's two-product
    // (product, fused residual and radix scalings) counts at most 66 <= 96.
    let extremes = [
        f64::from_bits(1),
        f64::from_bits(3),
        f64::from_bits((1 << 52) - 1),
        f64::MIN_POSITIVE,
        1.5 * f64::MIN_POSITIVE,
        1.25 * p2(-600),
        1.0,
        1.0 + f64::EPSILON,
        1.75 * p2(600),
        p2(1023),
        f64::MAX,
    ];
    let mut worst = 0;
    let mut ranges = 0;
    for &x in &extremes {
        for &y in &extremes {
            for sign in [1.0, -1.0] {
                let (a, b) = ([x], [sign * y]);
                let mut w = work(usize::MAX, 256, 0);
                count::reset();
                let result = w.mul(&expansion(&a), &expansion(&b));
                let o = oracle::mul(&a, &b, &Ledger { used: 0, limit: usize::MAX, terms: 256 });
                assert_eq!(trace::outcome(&result), o.outcome);
                assert_eq!(w.used, o.charged);
                let totals = count::totals();
                assert_eq!(totals.pairs, 1);
                assert_eq!(totals.mul.counted, o.counted, "{x:e} x {y:e}");
                assert_eq!(totals.shortfalls(), 0);
                assert!(totals.max_pair_two_product <= 66, "{x:e} x {y:e}");
                worst = worst.max(totals.max_pair_two_product);
                ranges += usize::from(o.outcome == Outcome::Range);
            }
        }
    }
    assert!(ranges > 0);
    assert!(worst <= 66 && 66 <= oracle::PAIR as u64 && oracle::PAIR == PAIR_CHARGE);
    // The largest count reached here: both operands 2^-1074 (3 + 3 operand
    // iterations, the product, then 3 iterations before hi's refusal).
    assert_eq!(worst, 38);
}
