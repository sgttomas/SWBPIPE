//! Dev-only operation counter: the maintained control of the metered
//! work-charging rule (RF source-method design §5.4).
//!
//! Compiled only in the frame kernel's own test build or with the
//! `exact-operation-count` feature, which only test builds enable; the product
//! build has none of it. The state is thread-local, so concurrently running
//! tests do not mix their counts, and no type gains a field.
//!
//! It counts executed operations under §5.3's declared model: 6 per executed
//! two-sum step of `Expansion::add`, 2 for a two-product and its fused
//! residual, and 4 per `exact_radix` loop iteration (two `powi`, two
//! multiplications). Exponent reads, comparisons and integer arithmetic count
//! 0. Every `exact_boundary` `Work::mul` and `Work::add` call records the work
//! it charged (accepted reservations only) against the operations it executed.
//!
//! A census asserts the control through [`reset`] and [`totals`]: after
//! `reset()`, run the exact attempts and replays, then require
//! `totals().shortfalls() == 0` (and, to rule out a vacuous run,
//! `totals().mul.calls > 0`).
//!
//! It also carries R-A's row trace (source-work design §3.2): each functional
//! evaluation records the path each row took, B1's negation or the long
//! evaluation with the count of terms B2 skipped, for [`take_row_paths`]. The
//! gating tests compare those rows with `evaluate_functionals_long`, the
//! reference entry that evaluates without B1's hints or B2's skip.
use std::cell::Cell;

/// One call kind's record since the last [`reset`] on this thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CallTotals {
    /// Calls recorded.
    pub calls: u64,
    /// Work the calls charged (accepted reservations; refusals excluded).
    pub charged: u64,
    /// Operations the calls executed, under §5.3's model.
    pub counted: u64,
    /// Calls charged less than they executed. The control requires 0.
    pub shortfalls: u64,
    /// The smallest `charged - counted` of one call; `None` before any call.
    pub min_slack: Option<i64>,
}
impl CallTotals {
    const ZERO: Self = Self {
        calls: 0,
        charged: 0,
        counted: 0,
        shortfalls: 0,
        min_slack: None,
    };
    fn record(&mut self, charged: u64, counted: u64) -> i64 {
        let slack = charged as i64 - counted as i64;
        self.calls += 1;
        self.charged += charged;
        self.counted += counted;
        self.shortfalls += u64::from(slack < 0);
        self.min_slack = Some(self.min_slack.map_or(slack, |s| s.min(slack)));
        slack
    }
}

/// Everything recorded since the last [`reset`] on this thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Totals {
    /// `Work::mul` calls.
    pub mul: CallTotals,
    /// `Work::add` calls (including those `Work::sum` and `Work::scalar` make).
    pub add: CallTotals,
    /// Term pairs of `Work::mul` that took the per-pair constant.
    pub pairs: u64,
    /// The smallest `charged - counted` of one term pair; `None` before any.
    pub min_pair_slack: Option<i64>,
    /// The largest two-product count of one term pair: the product, its fused
    /// residual and the radix scalings, which the per-pair constant covers
    /// (at most 66 by §5.3).
    pub max_pair_two_product: u64,
}
impl Totals {
    const ZERO: Self = Self {
        mul: CallTotals::ZERO,
        add: CallTotals::ZERO,
        pairs: 0,
        min_pair_slack: None,
        max_pair_two_product: 0,
    };
    /// Calls of either kind charged less than they executed.
    pub fn shortfalls(&self) -> u64 {
        self.mul.shortfalls + self.add.shortfalls
    }
}

#[derive(Debug, Clone, Copy)]
struct Counters {
    two_sum: u64,
    two_product: u64,
    radix: u64,
}
impl Counters {
    const ZERO: Self = Self {
        two_sum: 0,
        two_product: 0,
        radix: 0,
    };
    fn operations(self) -> u64 {
        self.two_sum + self.two_product + self.radix
    }
}

thread_local! {
    static COUNTERS: Cell<Counters> = const { Cell::new(Counters::ZERO) };
    static TOTALS: Cell<Totals> = const { Cell::new(Totals::ZERO) };
}

/// Clears this thread's counter and totals.
pub fn reset() {
    COUNTERS.with(|c| c.set(Counters::ZERO));
    TOTALS.with(|t| t.set(Totals::ZERO));
}
/// This thread's totals since the last [`reset`].
pub fn totals() -> Totals {
    TOTALS.with(Cell::get)
}
/// Operations counted on this thread since the last [`reset`], inside or
/// outside a `Work` call.
pub fn operations() -> u64 {
    COUNTERS.with(Cell::get).operations()
}

fn bump(f: impl FnOnce(&mut Counters)) {
    COUNTERS.with(|c| {
        let mut counters = c.get();
        f(&mut counters);
        c.set(counters);
    });
}
/// One executed two-sum step of `Expansion::add`.
pub(crate) fn two_sum_step() {
    bump(|c| c.two_sum += 6);
}
/// One two-product and its fused residual.
pub(crate) fn two_product() {
    bump(|c| c.two_product += 2);
}
/// One `exact_radix` loop iteration.
pub(crate) fn radix_iteration() {
    bump(|c| c.radix += 4);
}

/// Opened at the start of a `Work` call or term pair, with the ledger's
/// charged total; closed with the charged total at its end.
pub(super) struct Probe {
    used: usize,
    counters: Counters,
}
impl Probe {
    pub(super) fn start(used: usize) -> Self {
        Self {
            used,
            counters: COUNTERS.with(Cell::get),
        }
    }
    fn deltas(&self, used: usize) -> (u64, Counters) {
        let now = COUNTERS.with(Cell::get);
        (
            (used - self.used) as u64,
            Counters {
                two_sum: now.two_sum - self.counters.two_sum,
                two_product: now.two_product - self.counters.two_product,
                radix: now.radix - self.counters.radix,
            },
        )
    }
    fn finish_call(self, used: usize, mul: bool) {
        let (charged, counted) = self.deltas(used);
        let counted = counted.operations();
        let slack = TOTALS.with(|t| {
            let mut totals = t.get();
            let call = if mul { &mut totals.mul } else { &mut totals.add };
            let slack = call.record(charged, counted);
            t.set(totals);
            slack
        });
        // The frame kernel's own tests: every test that exercises `Work`
        // asserts the control on every call it makes.
        #[cfg(test)]
        assert!(
            slack >= 0,
            "Work::{} charged {charged} < {counted} executed operations",
            if mul { "mul" } else { "add" }
        );
        #[cfg(not(test))]
        let _ = slack;
    }
    pub(super) fn finish_mul(self, used: usize) {
        self.finish_call(used, true);
    }
    pub(super) fn finish_add(self, used: usize) {
        self.finish_call(used, false);
    }
    pub(super) fn finish_pair(self, used: usize) {
        let (charged, counted) = self.deltas(used);
        let two_product = counted.two_product + counted.radix;
        let slack = charged as i64 - counted.operations() as i64;
        TOTALS.with(|t| {
            let mut totals = t.get();
            totals.pairs += 1;
            totals.min_pair_slack = Some(totals.min_pair_slack.map_or(slack, |s| s.min(slack)));
            totals.max_pair_two_product = totals.max_pair_two_product.max(two_product);
            t.set(totals);
        });
    }
}

/// R-A's row paths (DEL-04-08 Design `source-work-nondyadic-inputs.md` §3.2, the gating
/// test's coverage assertion): for each functional evaluation on this thread since the
/// last [`take_row_paths`], the path each row took. A thread records only after its
/// first [`take_row_paths`], which only the gating tests call. So the code that enables
/// this feature without taking (product_physics' other tests, whose build carries the
/// registered identity) records nothing and allocates nothing here (D3 registration, §8.13).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowPath {
    /// B1: the source row's Ratio, negated termwise.
    Negated,
    /// The long path, with the number of terms whose recipe B2 skipped.
    Long { b2_skipped: u32 },
}
thread_local! {
    static RECORDING: Cell<bool> = const { Cell::new(false) };
    static ROW_PATHS: std::cell::RefCell<Vec<Vec<RowPath>>> = const { std::cell::RefCell::new(Vec::new()) };
}
pub(super) fn begin_evaluation() {
    if RECORDING.with(Cell::get) {
        ROW_PATHS.with(|r| r.borrow_mut().push(Vec::new()));
    }
}
pub(super) fn row(path: RowPath) {
    if RECORDING.with(Cell::get) {
        ROW_PATHS.with(|r| {
            if let Some(last) = r.borrow_mut().last_mut() {
                last.push(path);
            }
        });
    }
}
/// Takes this thread's row paths, one list per evaluation, and starts (or continues)
/// this thread's recording.
pub fn take_row_paths() -> Vec<Vec<RowPath>> {
    RECORDING.with(|r| r.set(true));
    ROW_PATHS.with(|r| std::mem::take(&mut *r.borrow_mut()))
}
