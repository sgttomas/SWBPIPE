//! KF1 tests (mounted in `adaptive.rs`, so they reach the tracker's
//! internals): the bounded tracker against a test-only copy of K4's unbounded
//! `ExtremeTracker` on randomized and adversarial streams, with the memory
//! bound asserted after every row; its work; the shared cap G; and the
//! model-level differential over every control at T = ∞, 1 and 512 (the
//! production T), with collapse work pinned at T = 64 through the test hook.
use super::super::combine::{CombinationOutcome, RetainedCombination};
use super::super::wide::multi::{limb_multiply_cost, OpKind};
use super::*;
use std::collections::BTreeMap;

#[allow(dead_code)]
#[path = "models.rs"]
mod models;

// ---------------------------------------------------------------- the reference

/// K4's `ExtremeTracker`, verbatim from `retained/adaptive.rs:542-596` at
/// `8cca91701` (main after K4's merge), renamed. The reference for KF1.
///
/// The exact directed extreme of a stream of ratios num/den (both exact): the
/// rows whose 64-bit approximation lies within `WINDOW_ULPS` of the running
/// approximate extreme are kept (compared as bit patterns, integers only) and
/// evaluated exactly at the end, so the result is the directed rounding of the
/// true extreme.
struct ReferenceTracker {
    direction: Direction,
    best: Option<u64>,
    kept: Vec<(ExactWideSum, ExactWideSum, u64)>,
}

impl ReferenceTracker {
    fn new(direction: Direction) -> Self {
        Self {
            direction,
            best: None,
            kept: Vec::new(),
        }
    }

    fn offer(
        &mut self,
        ctx64: &mut WideContext<4>,
        num: ExactWideSum,
        den: ExactWideSum,
    ) -> Result<(), AttemptStop> {
        let approx = approximate_ratio(ctx64, &num, &den)?.to_bits();
        let better = match (self.best, self.direction) {
            (None, _) => true,
            (Some(b), Direction::Up) => approx > b,
            (Some(b), Direction::Down) => approx < b,
        };
        if better {
            self.best = Some(approx);
            self.kept.retain(|k| k.2.abs_diff(approx) <= WINDOW_ULPS);
        }
        if self.best.is_some_and(|b| approx.abs_diff(b) <= WINDOW_ULPS) {
            self.kept.push((num, den, approx));
        }
        Ok(())
    }

    fn finish(self, ctx16: &mut WideContext<16>) -> Result<Option<f64>, AttemptStop> {
        let mut best: Option<f64> = None;
        for (num, den, _) in &self.kept {
            let exact = directed_ratio(ctx16, num, den, self.direction)?;
            best = Some(match (best, self.direction) {
                (None, _) => exact,
                (Some(b), Direction::Up) => b.max(exact),
                (Some(b), Direction::Down) => b.min(exact),
            });
        }
        Ok(best)
    }
}

// ---------------------------------------------------------------- streams

type Pair = (ExactWideSum, ExactWideSum);

const W: u64 = WINDOW_ULPS;

fn ctx64() -> WideContext<4> {
    WideContext::<4>::new(64).unwrap()
}

fn ctx16() -> WideContext<16> {
    WideContext::<16>::new(1024).unwrap()
}

fn sum_of(terms: &[f64]) -> ExactWideSum {
    let mut s = ExactWideSum::new();
    for &t in terms {
        s.add_binary64(t, false).unwrap();
    }
    s
}

/// num/den = v exactly, num the exact product v·d as two binary64 terms.
fn exact_ratio(v: f64, d: f64) -> Pair {
    let hi = v * d;
    let lo = v.mul_add(d, -hi);
    (sum_of(&[hi, lo]), sum_of(&[d]))
}

/// v·(1 ± 2^-100): v's key, and a directed value one step from v's.
fn nudged(v: f64, up: bool) -> Pair {
    let mut num = sum_of(&[v]);
    num.add_binary64(v * f64::from_bits(0x39B0_0000_0000_0000), !up) // 2^-100
        .unwrap();
    (num, sum_of(&[1.0]))
}

/// num = c (an odd significand), den = 1 + 2^-8100: the key is c's bits, and
/// `directed_ratio` refuses (c·den spans more than the 8,128-bit limit).
fn refusing(c: f64) -> Pair {
    let mut den = sum_of(&[1.0]);
    den.add_integer(false, &[1], -8100).unwrap();
    (sum_of(&[c]), den)
}

/// Whether a row was built by `refusing` (den − 1 is nonzero and below the
/// binary64 range).
fn is_refusing(den: &ExactWideSum) -> bool {
    let mut t = den.clone();
    t.add_binary64(1.0, true).unwrap();
    if t.is_zero() {
        return false;
    }
    let v = t.round(&mut ctx64()).unwrap();
    matches!(v.to_binary64(), Binary64Outcome::Underflow { .. })
}

fn zero() -> Pair {
    (ExactWideSum::new(), sum_of(&[3.0]))
}

/// A ratio above the binary64 range (its key is +∞'s bits).
fn overflowing() -> Pair {
    (
        sum_of(&[f64::from_bits(0x7E70_0000_0000_0000)]), // 2^1000
        sum_of(&[f64::from_bits(0x39B0_0000_0000_0000)]), // 2^-100
    )
}

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
    fn chance(&mut self, percent: u64) -> bool {
        self.below(100) < percent
    }
}

/// A value with the key `bits`, in one of four forms: exact (den 1), nudged
/// up or down (same key, directed value one step off), or an exact v·d/d.
fn at_key(bits: u64, rng: &mut Rng) -> Pair {
    let v = f64::from_bits(bits);
    match rng.below(4) {
        0 => (sum_of(&[v]), sum_of(&[1.0])),
        1 if v.is_normal() && v > 1e-200 && v < 1e200 => nudged(v, true),
        2 if v.is_normal() && v > 1e-200 && v < 1e200 => nudged(v, false),
        _ if v.is_normal() && v > 1e-200 && v < 1e200 => {
            exact_ratio(v, [1.5, 3.0, 5.0, 7.0, 11.0][rng.below(5) as usize])
        }
        _ => (sum_of(&[v]), sum_of(&[1.0])),
    }
}

/// The key one step toward the extreme (Up: larger; Down: smaller), s steps.
fn toward(bits: u64, s: u64, direction: Direction) -> u64 {
    match direction {
        Direction::Up => bits + s,
        Direction::Down => bits - s,
    }
}

fn away(bits: u64, s: u64, direction: Direction) -> u64 {
    match direction {
        Direction::Up => bits - s,
        Direction::Down => bits + s,
    }
}

/// An odd key near `bits` (a refusing row needs an odd significand).
fn odd(bits: u64) -> u64 {
    bits | 1
}

const CENTRE: u64 = 0x3FF8_0000_0000_0001; // 1.5 + 1 ulp (odd)

/// The families of streams (`family` modulo 9), with T = `t` for sizes.
fn stream(rng: &mut Rng, family: u64, t: usize, direction: Direction) -> Vec<Pair> {
    let t = t as u64;
    let mut out = Vec::new();
    match family % 9 {
        // Random ratios over a wide range, a few zeros and refusals.
        0 => {
            for _ in 0..1 + rng.below(120) {
                out.push(if rng.chance(5) {
                    zero()
                } else if rng.chance(2) {
                    refusing(f64::from_bits(odd(rng.next() >> 2)))
                } else {
                    let e = rng.below(121) as i32 - 60;
                    let v =
                        (1.0 + (rng.below(1 << 20) as f64) / (1u64 << 20) as f64) * 2f64.powi(e);
                    exact_ratio(v, 1.0 + rng.below(15) as f64)
                });
            }
        }
        // Clustered within ±2^k ulps of a centre, k ∈ {0, 3, 12, 13, 14, 20}.
        1 => {
            let k = [0u32, 3, 12, 13, 14, 20][rng.below(6) as usize];
            let spread = 1u64 << k;
            for _ in 0..1 + rng.below(4 * t + 40) {
                let bits = CENTRE - spread + rng.below(2 * spread + 1);
                out.push(if rng.chance(3) {
                    refusing(f64::from_bits(odd(bits)))
                } else {
                    at_key(bits, rng)
                });
            }
        }
        // All ties: one ratio repeated 1, T−1, T, T+1 or 3T+2 times, in one
        // form or in several (the same exact value).
        2 => {
            let n = [1, t.saturating_sub(1).max(1), t, t + 1, 3 * t + 2][rng.below(5) as usize];
            let v = f64::from_bits(CENTRE + rng.below(1000));
            let mixed = rng.chance(50);
            for _ in 0..n {
                out.push(if mixed {
                    exact_ratio(v, [1.0, 3.0, 7.0][rng.below(3) as usize])
                } else {
                    (sum_of(&[v]), sum_of(&[1.0]))
                });
            }
        }
        // The window edge: rows exactly W and W ± 1 from the best, refusals
        // among them, fillers that force collapses, then the best moving by 1
        // or 2 so the edge passes over them.
        3 => {
            let b = CENTRE;
            out.push(at_key(b, rng));
            for _ in 0..2 * t + 3 {
                let offset = [W, W + 1, W - 1, rng.below(W + 1)][rng.below(4) as usize];
                let bits = away(b, offset, direction);
                out.push(if rng.chance(20) && bits % 2 == 1 {
                    refusing(f64::from_bits(bits))
                } else {
                    at_key(bits, rng)
                });
            }
            out.push(at_key(toward(b, 1 + rng.below(2), direction), rng));
            for _ in 0..rng.below(t + 2) {
                let bits = away(b, W - 2 + rng.below(5), direction);
                out.push(at_key(bits, rng));
            }
            out.push(at_key(toward(b, 2 + rng.below(2), direction), rng));
        }
        // Many ties, then a new extreme exactly W, W + 1 or far beyond.
        4 => {
            let v = CENTRE;
            for i in 0..3 * t + 2 {
                out.push(if i == t && rng.chance(50) {
                    refusing(f64::from_bits(v))
                } else if rng.chance(30) {
                    nudged(f64::from_bits(v), rng.chance(50))
                } else {
                    (sum_of(&[f64::from_bits(v)]), sum_of(&[1.0]))
                });
            }
            let jump = [W, W + 1, 1 << 30][rng.below(3) as usize];
            out.push(at_key(toward(v, jump, direction), rng));
            for _ in 0..rng.below(t + 1) {
                out.push(at_key(v, rng));
            }
        }
        // Zeros: all zero, or zeros among subnormal ratios near 0.
        5 => {
            let all_zero = rng.chance(40);
            for _ in 0..1 + rng.below(3 * t + 5) {
                out.push(if all_zero || rng.chance(40) {
                    zero()
                } else {
                    let bits = 1 + rng.below(3 * W);
                    (sum_of(&[f64::from_bits(bits)]), sum_of(&[1.0]))
                });
            }
        }
        // The top of the range: overflowing ratios, f64::MAX and below.
        6 => {
            for _ in 0..1 + rng.below(3 * t + 5) {
                out.push(match rng.below(4) {
                    0 => overflowing(),
                    1 => (sum_of(&[f64::MAX]), sum_of(&[1.0])),
                    2 => {
                        let bits = f64::MAX.to_bits() - rng.below(2 * W);
                        (sum_of(&[f64::from_bits(bits)]), sum_of(&[1.0]))
                    }
                    _ => nudged(f64::from_bits(0x7FE0_0000_0000_0000), false),
                });
            }
        }
        // Refusals: early and then dropped by a far best, surviving, or two.
        7 => {
            let c = CENTRE;
            for _ in 0..rng.below(2 * t + 2) {
                out.push(at_key(away(c, rng.below(W / 2), direction), rng));
            }
            for _ in 0..1 + rng.below(2) {
                out.push(refusing(f64::from_bits(odd(away(
                    c,
                    rng.below(W),
                    direction,
                )))));
            }
            for _ in 0..rng.below(2 * t + 2) {
                out.push(at_key(away(c, rng.below(W), direction), rng));
            }
            if rng.chance(50) {
                out.push(at_key(toward(c, W + 1 + rng.below(4 * W), direction), rng));
            }
            for _ in 0..rng.below(t + 2) {
                out.push(at_key(c, rng));
            }
        }
        // A monotone run toward the extreme (the best improves on most rows)
        // with ties and stragglers.
        _ => {
            let mut bits = CENTRE;
            for _ in 0..1 + rng.below(4 * t + 20) {
                bits = toward(bits, rng.below(W / 4), direction);
                out.push(at_key(bits, rng));
                if rng.chance(30) {
                    out.push(at_key(away(bits, rng.below(2 * W), direction), rng));
                }
            }
        }
    }
    out
}

// ---------------------------------------------------------------- the differential

/// One exact evaluation's work at 16 limbs: two roundings and one division.
fn evaluation_cost() -> u64 {
    2 * limb_multiply_cost(OpKind::Round, 16) + limb_multiply_cost(OpKind::Div, 16)
}

#[derive(Default, Debug)]
struct Coverage {
    runs: u64,
    collapses: u64,
    collapsed_then_dropped: u64,
    refused_both: u64,
    refusal_collapsed_then_dropped: u64,
    ties_over_t: u64,
    max_table: usize,
}

/// The key-only replay (independent of the tracker): the number of nonzero
/// rows the bounded tracker evaluates and the number K4's evaluates.
fn replay(keys: &[(u64, bool)], direction: Direction, limit: usize) -> (u64, u64, u64) {
    let mut best: Option<u64> = None;
    let mut lazy: Vec<(u64, bool)> = Vec::new();
    let (mut collapsed, mut collapses) = (0u64, 0u64);
    for &(k, nonzero) in keys {
        let better = match (best, direction) {
            (None, _) => true,
            (Some(b), Direction::Up) => k > b,
            (Some(b), Direction::Down) => k < b,
        };
        if better {
            best = Some(k);
            lazy.retain(|r| r.0.abs_diff(k) <= W);
        }
        if best.is_some_and(|b| k.abs_diff(b) <= W) {
            if lazy.len() >= limit {
                collapsed += lazy.iter().filter(|r| r.1).count() as u64;
                collapses += 1;
                lazy.clear();
            }
            lazy.push((k, nonzero));
        }
    }
    let bounded = collapsed + lazy.iter().filter(|r| r.1).count() as u64;
    let b = best.unwrap_or(0);
    let reference = keys.iter().filter(|r| r.1 && r.0.abs_diff(b) <= W).count() as u64;
    (bounded, reference, collapses)
}

fn differential(stream: &[Pair], direction: Direction, limit: usize, cov: &mut Coverage) {
    let mut r = ReferenceTracker::new(direction);
    let mut b = BoundedExtremeTracker::with_limit(direction, limit);
    let (mut c64r, mut c64b, mut c64k) = (ctx64(), ctx64(), ctx64());
    let (mut c16r, mut c16b) = (ctx16(), ctx16());
    let mut keys = Vec::with_capacity(stream.len());
    let mut refused_seen = false;
    for (num, den) in stream {
        let key = approximate_ratio(&mut c64k, num, den).unwrap().to_bits();
        keys.push((key, !num.clone().is_zero()));
        r.offer(&mut c64r, num.clone(), den.clone()).unwrap();
        b.offer(&mut c64b, &mut c16b, num.clone(), den.clone())
            .unwrap();
        // The memory bound, after every row: at most T unevaluated rows, at
        // most W + 1 entries, every key in the window.
        let best = b.best.unwrap();
        assert_eq!(b.best, r.best);
        assert!(b.lazy.len() <= limit, "{} > {limit}", b.lazy.len());
        assert!(b.table.len() as u64 <= W + 1);
        assert!(b.lazy.iter().all(|e| in_window(e.2, best)));
        assert!(b.table.iter().all(|e| in_window(e.0, best)));
        // J1: the unevaluated rows are the last of K4's kept rows, in order;
        // and never more rows than K4 keeps.
        let kept: Vec<u64> = r.kept.iter().map(|k| k.2).collect();
        let lazy: Vec<u64> = b.lazy.iter().map(|e| e.2).collect();
        assert!(kept.ends_with(&lazy));
        assert!(b.lazy.len() + b.table.len() <= r.kept.len());
        // With the approximations' accuracy, one ratio entry at most.
        let ratios = b
            .table
            .iter()
            .filter(|e| matches!(e.1, Evaluated::Ratio(_)))
            .count();
        assert!(ratios <= 1, "{:?}", b.table);
        refused_seen |= b
            .table
            .iter()
            .any(|e| matches!(e.1, Evaluated::Refused { .. }));
        cov.max_table = cov.max_table.max(b.table.len());
    }
    let refusals = stream.iter().any(|(_, den)| is_refusing(den));
    let bits = |o: Result<Option<f64>, AttemptStop>| o.map(|v| v.map(f64::to_bits));
    let reference = bits(r.finish(&mut c16r));
    let bounded = bits(b.finish(&mut c16b));
    assert_eq!(
        bounded,
        reference,
        "{direction:?} T = {limit}, {} rows",
        stream.len()
    );
    let (evals, reference_evals, collapses) = replay(&keys, direction, limit);
    cov.runs += 1;
    cov.collapses += collapses;
    if reference.is_err() {
        cov.refused_both += 1;
    } else if refused_seen {
        cov.refusal_collapsed_then_dropped += 1;
    }
    if !refusals {
        // Work: K4's is one evaluation per nonzero kept row; the bounded
        // tracker's adds one per nonzero row it evaluated at a collapse and
        // K4 does not keep.
        let cost = evaluation_cost();
        assert_eq!(lme(&c16r), cost * reference_evals);
        assert_eq!(lme(&c16b), cost * evals, "{direction:?} T = {limit}");
        assert!(evals >= reference_evals);
        let nonzero = keys.iter().filter(|k| k.1).count() as u64;
        assert!(lme(&c16b) <= cost * nonzero);
        if evals > reference_evals {
            cov.collapsed_then_dropped += 1;
        }
    }
}

#[test]
fn kf1_one_evaluation_costs_17506() {
    assert_eq!(evaluation_cost(), 17_506);
    // The constructions: a refusing row passes the approximation with c's key
    // and refuses exactly, in both directions.
    for c in [
        f64::from_bits(CENTRE),
        f64::from_bits(odd(0x4123_4567_89AB_CDEF)),
    ] {
        let (num, den) = refusing(c);
        let key = approximate_ratio(&mut ctx64(), &num, &den).unwrap();
        assert_eq!(key.to_bits(), c.to_bits());
        for direction in [Direction::Up, Direction::Down] {
            assert_eq!(
                directed_ratio(&mut ctx16(), &num, &den, direction),
                Err(AttemptStop::Span)
            );
        }
    }
    // A nudged row keeps v's key; its directed values step off v.
    let v = f64::from_bits(CENTRE);
    let (num, den) = nudged(v, true);
    assert_eq!(
        approximate_ratio(&mut ctx64(), &num, &den)
            .unwrap()
            .to_bits(),
        CENTRE
    );
    assert_eq!(
        directed_ratio(&mut ctx16(), &num, &den, Direction::Up).unwrap(),
        next_up(v)
    );
    assert_eq!(
        directed_ratio(&mut ctx16(), &num, &den, Direction::Down).unwrap(),
        v
    );
    let (num, den) = overflowing();
    assert_eq!(
        approximate_ratio(&mut ctx64(), &num, &den).unwrap(),
        f64::INFINITY
    );
}

#[test]
fn kf1_bounded_tracker_equals_k4s_on_every_stream() {
    let mut cov = Coverage::default();
    let mut rng = Rng(0x4B46_3120_5452_4143); // "KF1 TRAC"
    for family in 0..9u64 {
        for direction in [Direction::Up, Direction::Down] {
            for limit in [1usize, 2, 3, 5, 64, 512] {
                let seeds = match limit {
                    512 => 3,
                    64 => 6,
                    _ => 10,
                };
                for _ in 0..seeds {
                    let s = stream(&mut rng, family, limit, direction);
                    if s.len() > limit {
                        cov.ties_over_t += u64::from(family == 2);
                    }
                    differential(&s, direction, limit, &mut cov);
                }
            }
        }
    }
    println!("KF1 differential coverage: {cov:?}");
    assert!(cov.runs >= 800, "{cov:?}");
    assert!(cov.collapses >= 1_000, "{cov:?}");
    assert!(cov.collapsed_then_dropped >= 50, "{cov:?}");
    assert!(cov.refused_both >= 20, "{cov:?}");
    assert!(cov.refusal_collapsed_then_dropped >= 5, "{cov:?}");
    assert!(cov.ties_over_t >= 20, "{cov:?}");
}

#[test]
fn kf1_the_shared_cap_bounds_a_calls_trackers_together() {
    // Many trackers (bodies), few rows each: G bounds their unevaluated rows'
    // allocated capacity together, and each tracker's result is K4's.
    struct Hook;
    impl Drop for Hook {
        fn drop(&mut self) {
            tracker_hook::set(None);
        }
    }
    let _hook = Hook;
    let mut rng = Rng(0x4B46_3120_4341_5053); // "KF1 CAPS"
    let mut collapses_all = 0u64;
    for round in 0..60u64 {
        let direction = if round % 2 == 0 {
            Direction::Up
        } else {
            Direction::Down
        };
        let t = [1usize, 2, 3, 5, 64][(round / 2 % 5) as usize];
        tracker_hook::set(Some(t));
        let g = [4usize, 8, 24, 512][rng.below(4) as usize];
        let mut set: TrackerSet<u32> = TrackerSet::with_limit(g);
        let mut references: BTreeMap<u32, ReferenceTracker> = BTreeMap::new();
        let (mut c64s, mut c64r) = (ctx64(), ctx64());
        let (mut c16s, mut c16r) = (ctx16(), ctx16());
        let trackers = 1 + rng.below(60) as u32;
        let mut rows: Vec<(u32, Pair)> = Vec::new();
        for key in 0..trackers {
            let family = rng.below(9);
            for p in stream(&mut rng, family, t, direction) {
                rows.push((key, p));
            }
        }
        // Interleave the trackers' rows.
        for i in (1..rows.len()).rev() {
            rows.swap(i, rng.below(i as u64 + 1) as usize);
        }
        let mut last_held = 0;
        for (key, (num, den)) in rows {
            set.offer(
                key,
                direction,
                &mut c64s,
                &mut c16s,
                num.clone(),
                den.clone(),
            )
            .unwrap();
            references
                .entry(key)
                .or_insert_with(|| ReferenceTracker::new(direction))
                .offer(&mut c64r, num, den)
                .unwrap();
            let (held, limit) = set.held();
            assert_eq!(limit, g);
            assert!(held <= g, "{held} > {g}");
            let capacity: usize = set.trackers().map(|(_, t)| t.held().1).sum();
            let rows_held: usize = set.trackers().map(|(_, t)| t.held().0).sum();
            assert_eq!(held, capacity);
            assert!(rows_held <= g);
            assert!(set.trackers().all(|(_, tr)| tr.held().0 <= t));
            if held < last_held && held == 0 {
                collapses_all += 1;
            }
            last_held = held;
        }
        for (key, tracker) in set.into_trackers() {
            let bits = |o: Result<Option<f64>, AttemptStop>| o.map(|v| v.map(f64::to_bits));
            assert_eq!(
                bits(tracker.finish(&mut c16s)),
                bits(references.remove(&key).unwrap().finish(&mut c16r)),
                "round {round}, tracker {key}"
            );
        }
        assert!(references.is_empty());
        // RV20-1: the set's collapses are charged to the call's context. Every
        // row K4's finish evaluates (up to its first refusal) is evaluated once
        // by the bounded trackers, at a collapse or at finish, at the same cost.
        assert!(
            lme(&c16s) >= lme(&c16r),
            "round {round}: the set's work {} < K4's {}",
            lme(&c16s),
            lme(&c16r)
        );
    }
    println!("KF1 shared cap: {collapses_all} collapses of every tracker");
    assert!(collapses_all >= 20, "{collapses_all}");
}

#[test]
fn kf1_the_stop_rules_trackers_finish_in_k4s_map_order() {
    // RV20-N1 (RV20's order test): the set's key order, (test, body, kind),
    // equals K4's three maps (a), (b), (d), each by (body, kind), concatenated.
    // The first `Err` among the finishes and each summary's order follow it.
    assert!(RuleTest::Disagreement < RuleTest::Estimate);
    assert!(RuleTest::Estimate < RuleTest::Charge);
    let mut set: TrackerSet<(RuleTest, u32, Kind)> = TrackerSet::with_limit(usize::MAX);
    let (mut c64, mut c16) = (ctx64(), ctx16());
    for test in [RuleTest::Charge, RuleTest::Disagreement, RuleTest::Estimate] {
        for body in [3u32, 0, 1] {
            for kind in [Kind::Moment, Kind::Force] {
                let (num, den) = exact_ratio(1.5, 3.0);
                set.offer(
                    (test, body, kind),
                    Direction::Up,
                    &mut c64,
                    &mut c16,
                    num,
                    den,
                )
                .unwrap();
            }
        }
    }
    let order: Vec<(RuleTest, u32, Kind)> = set.into_trackers().map(|(k, _)| k).collect();
    let mut expected = Vec::new();
    for test in [RuleTest::Disagreement, RuleTest::Estimate, RuleTest::Charge] {
        let mut map: BTreeMap<(u32, Kind), ()> = BTreeMap::new();
        for body in [3u32, 0, 1] {
            for kind in [Kind::Moment, Kind::Force] {
                map.insert((body, kind), ());
            }
        }
        expected.extend(map.keys().map(|&(body, kind)| (test, body, kind)));
    }
    assert_eq!(order, expected);
}

// ---------------------------------------------------------------- the model-level differential

const MODELS_5A3: &str = include_str!("models5a3.txt");

struct Setting;

impl Setting {
    /// T for the trackers built on this thread (None: `TRACKER_ROWS`), and
    /// the model's seeds, both cleared on drop.
    fn new(rows: Option<usize>, seeds: &[(usize, f64)]) -> Self {
        tracker_hook::set(rows);
        seed::set(seeds.to_vec());
        Setting
    }
}

impl Drop for Setting {
    fn drop(&mut self) {
        tracker_hook::set(None);
        seed::set(Vec::new());
    }
}

const SETTINGS: [(&str, Option<usize>); 3] =
    [("inf", Some(usize::MAX)), ("T=1", Some(1)), ("T=512", None)];

/// The per-attempt work that moves, as (stop rule, refinement, bounded gate,
/// condition) deltas against T = ∞.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct Moved {
    stop_rule: u64,
    refinement: u64,
    bounded_gate: u64,
    condition: u64,
}

impl Moved {
    fn any(&self) -> bool {
        *self != Moved::default()
    }
}

/// Everything but the work, and the work's deltas checked exactly: only the
/// 16-limb rounds and divisions move, 2:1, each division one evaluation, and
/// the LME delta lands in the stop rule, refinement, bounded gate (own) and
/// condition (shared) stages only.
fn same_attempts(name: &str, x: &[AttemptRecord], y: &[AttemptRecord]) -> Vec<Moved> {
    assert_eq!(x.len(), y.len(), "{name}: attempts");
    let cost = evaluation_cost();
    let mut moved = Vec::new();
    for (a, b) in x.iter().zip(y) {
        let strip = |r: &AttemptRecord| {
            let mut r = r.clone();
            r.work = AttemptWork::default();
            r.stages = StageWork::default();
            r.shared_work = 0;
            r.shared_stages = StageWork::default();
            r.stop_rule_work = 0;
            format!("{r:?}")
        };
        assert_eq!(strip(a), strip(b), "{name}: the record outside its work");
        assert_eq!(a.k4_work, b.k4_work, "{name}: K4 sum work");
        assert_eq!(a.work.width::<4>(), b.work.width::<4>(), "{name}");
        assert_eq!(a.work.width::<8>(), b.work.width::<8>(), "{name}");
        let (w, v) = (a.work.width::<16>(), b.work.width::<16>());
        assert!(v.div >= w.div, "{name}");
        let n = v.div - w.div;
        assert_eq!(v.round - w.round, 2 * n, "{name}");
        let mut v_rest = v;
        v_rest.div = w.div;
        v_rest.round = w.round;
        assert_eq!(v_rest, w, "{name}: other 16-limb operations");
        let d = |x: u64, y: u64| {
            assert!(y >= x, "{name}: work fell");
            assert_eq!((y - x) % cost, 0, "{name}: not whole evaluations");
            y - x
        };
        let m = Moved {
            stop_rule: d(a.stages.stop_rule, b.stages.stop_rule),
            refinement: d(a.stages.refinement, b.stages.refinement),
            bounded_gate: d(a.stages.bounded_gate, b.stages.bounded_gate),
            condition: d(a.shared_stages.condition, b.shared_stages.condition),
        };
        assert_eq!(
            b.work.limb_multiply_equivalents() - a.work.limb_multiply_equivalents(),
            cost * n,
            "{name}"
        );
        assert_eq!(
            m.stop_rule + m.refinement + m.bounded_gate,
            cost * n,
            "{name}"
        );
        assert_eq!(b.stop_rule_work - a.stop_rule_work, m.stop_rule, "{name}");
        assert_eq!(b.shared_work - a.shared_work, m.condition, "{name}");
        let mut s = b.stages.clone();
        s.stop_rule = a.stages.stop_rule;
        s.refinement = a.stages.refinement;
        s.bounded_gate = a.stages.bounded_gate;
        assert_eq!(s, a.stages, "{name}: other own stages");
        let mut s = b.shared_stages.clone();
        s.condition = a.shared_stages.condition;
        assert_eq!(s, a.shared_stages, "{name}: other shared stages");
        moved.push(m);
    }
    moved
}

fn same_solve(name: &str, a: &RetainedSolve, b: &RetainedSolve) -> Vec<Moved> {
    assert_eq!(a.selected_precision(), b.selected_precision(), "{name}");
    assert_eq!(
        format!("{:?}", a.publish()),
        format!("{:?}", b.publish()),
        "{name}: publication"
    );
    assert_eq!(
        format!("{:?}", a.states),
        format!("{:?}", b.states),
        "{name}: states"
    );
    let (mut ea, mut eb) = (a.evidence().clone(), b.evidence().clone());
    let moved = same_attempts(name, &ea.attempts, &eb.attempts);
    ea.attempts.clear();
    eb.attempts.clear();
    assert_eq!(format!("{ea:?}"), format!("{eb:?}"), "{name}: evidence");
    moved
}

fn same_outcome(name: &str, x: &CaseOutcome, y: &CaseOutcome) -> Vec<Moved> {
    match (x, y) {
        (CaseOutcome::Selected(a), CaseOutcome::Selected(b)) => same_solve(name, a, b),
        (
            CaseOutcome::Unresolved {
                reason: ra,
                attempts: aa,
                geometry: ga,
            },
            CaseOutcome::Unresolved {
                reason: rb,
                attempts: ab,
                geometry: gb,
            },
        ) => {
            assert_eq!(format!("{ra:?}{ga:?}"), format!("{rb:?}{gb:?}"), "{name}");
            same_attempts(name, aa, ab)
        }
        (CaseOutcome::Refused { .. }, CaseOutcome::Refused { .. }) => {
            assert_eq!(format!("{x:?}"), format!("{y:?}"), "{name}");
            Vec::new()
        }
        _ => panic!("{name}: {x:?} against {y:?}"),
    }
}

fn solve_under(m: &models::Model, rows: Option<usize>) -> CaseOutcome {
    let _setting = Setting::new(rows, &m.seeds);
    let mut meter = InvocationMeter::new(u64::MAX);
    solve_case(m.source(), CaseLimit::new(u64::MAX), &mut meter)
}

fn all_models() -> Vec<models::Model> {
    let mut out = models::models();
    out.extend(models::parse_models(MODELS_5A3));
    out
}

fn report(name: &str, setting: &str, moved: &[Moved]) {
    if moved.iter().any(Moved::any) {
        let cost = evaluation_cost();
        let rows: Vec<String> = moved
            .iter()
            .map(|m| {
                format!(
                    "[stop rule +{}, refinement +{}, bounded gate +{}, condition +{}]",
                    m.stop_rule / cost,
                    m.refinement / cost,
                    m.bounded_gate / cost,
                    m.condition / cost
                )
            })
            .collect();
        println!("KF1 moved {name} {setting}: {}", rows.join(" "));
    }
}

/// Every model at T = ∞ (K4's behaviour), T = 1 and T = 512 (`TRACKER_ROWS`),
/// compared.
fn models_differential(filter: impl Fn(&str) -> bool) -> usize {
    let mut compared = 0;
    for m in all_models().iter().filter(|m| filter(&m.name)) {
        let base = solve_under(m, SETTINGS[0].1);
        for (label, rows) in &SETTINGS[1..] {
            let other = solve_under(m, *rows);
            let moved = same_outcome(&format!("{} {label}", m.name), &base, &other);
            report(&m.name, label, &moved);
        }
        compared += 1;
    }
    compared
}

#[test]
fn kf1_every_control_is_unchanged_but_its_work_at_t_1_and_512() {
    let compared = models_differential(|n| !n.contains("n00100"));
    assert!(compared >= 125, "{compared}");
    // GEN's 5a.3 combinations, operands and combination under each setting.
    let all = all_models();
    let mut combos = 0;
    for c in models::parse_combos(MODELS_5A3) {
        let run = |rows: Option<usize>| {
            let operands: Vec<(f64, Box<RetainedSolve>)> = c
                .operands
                .iter()
                .map(|(f, n)| {
                    let m = all.iter().find(|m| m.name == *n).unwrap();
                    match solve_under(m, rows) {
                        CaseOutcome::Selected(s) => (*f, s),
                        other => panic!("{}: operand {n}: {other:?}", c.name),
                    }
                })
                .collect();
            let refs: Vec<(f64, &RetainedSolve)> =
                operands.iter().map(|(f, s)| (*f, s.as_ref())).collect();
            let _setting = Setting::new(rows, &[]);
            let mut meter = InvocationMeter::new(u64::MAX);
            RetainedCombination::solve(&refs, CaseLimit::new(u64::MAX), &mut meter)
        };
        let base = run(SETTINGS[0].1);
        for (label, rows) in &SETTINGS[1..] {
            let other = run(*rows);
            let name = format!("{} {label}", c.name);
            let moved = match (&base, &other) {
                (CombinationOutcome::Selected(a), CombinationOutcome::Selected(b)) => {
                    same_solve(&name, a, b)
                }
                (
                    CombinationOutcome::Unresolved {
                        reason: ra,
                        attempts: aa,
                    },
                    CombinationOutcome::Unresolved {
                        reason: rb,
                        attempts: ab,
                    },
                ) => {
                    assert_eq!(format!("{ra:?}"), format!("{rb:?}"), "{name}");
                    same_attempts(&name, aa, ab)
                }
                _ => panic!("{name}: the outcome changed"),
            };
            report(&c.name, label, &moved);
        }
        combos += 1;
    }
    assert_eq!(combos, 4);
}

#[test]
fn kf1_rf_large_at_100_members_is_unchanged_but_its_work_at_t_1_and_512() {
    assert_eq!(models_differential(|n| n.contains("n00100")), 6);
}

#[test]
fn kf1_golden_stop_rule_work_where_collapses_occur() {
    use super::publication_tests::{publication_bits, replay_certificate_components};
    // K4's golden work (`golden_work_counts`, `A3B_WORK`) does not move: no
    // tracker of the four golden models reaches 64 rows. At T = 64 only the
    // RF-LARGE frames at 100 members move (KF1's checkpoint A), and at the
    // production T = 512 none moves (addendum 1). Pinned here on two of them,
    // per attempt (p, stop-rule work, refinement, bounded gate, shared
    // condition): at T = ∞ (K4's figures, also measured on main `8cca91701`'s
    // code); at T = 512, unchanged; and at T = 64 through the test hook, so
    // collapse work stays exercised at model level, where the stop rule adds
    // 768 and 640 exact evaluations (collapsed rows the window later drops)
    // at 17,506 LME each: 13,444,608 and 11,203,840.
    let cost = evaluation_cost();
    type Row = (u32, u64, u64, u64, u64);
    let golden: [(&str, u64, [Row; 2]); 2] = [
        (
            "RF-LARGE-CHAIN-n00100-AX",
            768,
            [
                (128, 8_625_285, 1_182_130, 0, 8_435_467),
                (256, 0, 1_573_254, 0, 8_472_419),
            ],
        ),
        (
            "RF-LARGE-CONT-n00100-AX",
            640,
            [
                (128, 13_830_960, 851_913, 0, 4_776_664),
                (256, 0, 988_343, 0, 4_804_814),
            ],
        ),
    ];
    let all = all_models();
    for (name, evaluations, k4) in golden {
        let m = all.iter().find(|m| m.name == name).unwrap();
        let rows = |rows: Option<usize>| {
            // Keep the SAME tracker/seed overrides alive through paired replay.
            let _setting = Setting::new(rows, &m.seeds);
            let mut meter = InvocationMeter::new(u64::MAX);
            let CaseOutcome::Selected(s) =
                solve_case(m.source(), CaseLimit::new(u64::MAX), &mut meter)
            else {
                panic!("{name}")
            };
            let components = replay_certificate_components(&s, meter.charged());
            let historical: Vec<Row> = s
                .evidence()
                .attempts
                .iter()
                .zip(&components)
                .map(|(a, component)| {
                    (
                        a.precision,
                        component.r7,
                        a.stages.refinement,
                        a.stages.bounded_gate,
                        a.shared_stages.condition,
                    )
                })
                .collect();
            let certificate: Vec<_> = components
                .iter()
                .map(|c| (c.context, c.sums, c.total))
                .collect();
            (historical, certificate, s)
        };
        let (infinite, cert_infinite, solve_infinite) = rows(Some(usize::MAX));
        assert_eq!(infinite, k4, "{name} at T = ∞");
        assert_eq!(TRACKER_ROWS, 512);
        let (production, cert_production, solve_production) = rows(None);
        assert_eq!(production, k4, "{name} at T = 512");
        assert_eq!(
            cert_production, cert_infinite,
            "{name}: certificate components"
        );
        same_solve(name, &solve_infinite, &solve_production);
        let mut at_64 = k4;
        at_64[0].1 += evaluations * cost;
        let (limited, cert_limited, solve_limited) = rows(Some(64));
        assert_eq!(limited, at_64, "{name} at T = 64");
        assert_eq!(
            cert_limited, cert_infinite,
            "{name}: certificate components"
        );
        same_solve(name, &solve_infinite, &solve_limited);
        for s in [&solve_production, &solve_limited] {
            assert_eq!(
                publication_bits(s.publish()),
                publication_bits(solve_infinite.publish())
            );
            assert_eq!(
                s.publication_radius_bits,
                solve_infinite.publication_radius_bits
            );
        }
    }
    assert_eq!(768 * cost, 13_444_608);
    assert_eq!(640 * cost, 11_203_840);
}

#[test]
fn kf1_the_bound_in_bytes() {
    // The figures behind the stated bound (KF1 RETURN): one unevaluated row,
    // one evaluated entry, one tracker without its rows.
    let row = std::mem::size_of::<(ExactWideSum, ExactWideSum, u64, u64)>();
    let entry = std::mem::size_of::<(u64, Evaluated)>();
    let tracker = std::mem::size_of::<BoundedExtremeTracker>();
    println!(
        "KF1 sizes: ExactWideSum {} B, row {row} B, entry {entry} B, tracker {tracker} B; \
         T = {TRACKER_ROWS} rows {} B; G = {TRACKER_SET_ROWS} rows {} B; W + 1 entries {} B",
        std::mem::size_of::<ExactWideSum>(),
        TRACKER_ROWS * row,
        TRACKER_SET_ROWS * row,
        (W as usize + 1) * entry
    );
    assert!(row <= 4_400 && entry <= 64 && tracker <= 128);
    assert_eq!(TRACKER_SET_ROWS, 8 * TRACKER_ROWS);
}
