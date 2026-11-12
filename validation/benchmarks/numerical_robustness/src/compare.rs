//! The comparison and report engine (T3 D1 §4.10's table; plan §6.3).
//!
//! Each R1 row gets exactly one verdict: `pass`, `fail`, `not_covered` (with
//! whether the predicate held, never a pass), `pass_absolute_range` (the
//! expected value has no binary64 value; separately counted), or
//! `structural_zero` (no published row by construction; R1's value must be
//! exactly 0), or `expected_unresolved` (the row of a case on the committed
//! expected-unresolved list, which ended honestly unresolved with no rows:
//! ROOT's ruling on THIN-A and THIN-B; never a pass). The report shows
//! passes, absolute-range passes and not-covered comparisons as three
//! separate numbers, and the six tallies must add up to the rows compared.
use crate::exact::{self, Exact};
use std::fmt;

/// What the kernel published for a row.
#[derive(Clone, Debug, PartialEq)]
pub enum Observed {
    /// A binary64 value (an `Underflow` row is observed as its rounding, ±0).
    Value(f64),
    /// The two components of a bending magnitude.
    Magnitude(f64, f64),
    /// No row by construction (plan §4.2).
    StructuralZero,
    /// Nothing comparable was published (missing, or overflowed).
    Unavailable(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Verdict {
    Pass,
    Fail(String),
    NotCovered { predicate_held: bool },
    PassAbsoluteRange,
    StructuralZero,
    ExpectedUnresolved,
}

fn holds(obs: &Observed, exp: &Exact, scale: &Exact) -> Option<bool> {
    match obs {
        Observed::Value(v) => Some(exact::predicate(&Exact::from_f64(*v), exp, scale)),
        Observed::Magnitude(y, z) => Some(exact::magnitude_predicate(*y, *z, exp, scale)),
        _ => None,
    }
}

/// The verdict of one row: `covered` is the floor check's decision.
pub fn judge(obs: &Observed, exp: &Exact, scale: &Exact, covered: bool) -> Verdict {
    if *obs == Observed::StructuralZero {
        return if exp.is_zero() {
            Verdict::StructuralZero
        } else {
            Verdict::Fail("structural zero with a nonzero reference".into())
        };
    }
    if !covered {
        return Verdict::NotCovered {
            predicate_held: holds(obs, exp, scale).unwrap_or(false),
        };
    }
    match holds(obs, exp, scale) {
        None => Verdict::Fail(match obs {
            Observed::Unavailable(why) => why.clone(),
            _ => unreachable!(),
        }),
        Some(true) if exact::outside_binary64(exp) => Verdict::PassAbsoluteRange,
        Some(true) => Verdict::Pass,
        Some(false) => Verdict::Fail("predicate".into()),
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Tally {
    pub rows: usize,
    pub pass: usize,
    pub pass_absolute_range: usize,
    pub not_covered: usize,
    pub structural_zero: usize,
    pub expected_unresolved: usize,
    pub fail: usize,
}

impl Tally {
    pub fn add(&mut self, v: &Verdict) {
        self.rows += 1;
        match v {
            Verdict::Pass => self.pass += 1,
            Verdict::PassAbsoluteRange => self.pass_absolute_range += 1,
            Verdict::NotCovered { .. } => self.not_covered += 1,
            Verdict::StructuralZero => self.structural_zero += 1,
            Verdict::ExpectedUnresolved => self.expected_unresolved += 1,
            Verdict::Fail(_) => self.fail += 1,
        }
    }

    pub fn merge(&mut self, o: &Tally) {
        self.rows += o.rows;
        self.pass += o.pass;
        self.pass_absolute_range += o.pass_absolute_range;
        self.not_covered += o.not_covered;
        self.structural_zero += o.structural_zero;
        self.expected_unresolved += o.expected_unresolved;
        self.fail += o.fail;
    }

    /// Every row has exactly one verdict.
    pub fn accounted(&self) -> bool {
        self.pass
            + self.pass_absolute_range
            + self.not_covered
            + self.structural_zero
            + self.expected_unresolved
            + self.fail
            == self.rows
    }
}

impl fmt::Display for Tally {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rows {}: passes {}, absolute-range passes {}, not covered {} | structural zeros {}, \
             expected unresolved {}, failures {}",
            self.rows,
            self.pass,
            self.pass_absolute_range,
            self.not_covered,
            self.structural_zero,
            self.expected_unresolved,
            self.fail
        )
    }
}

/// The discrimination tally of a set of cases (plan §8).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ControlTally {
    /// Discriminating controls that fail the predicate (or, for outcome
    /// controls, whose defect the kernel's outcome excludes).
    pub discriminated: usize,
    /// R1's non-discriminating controls (reported, never dropped).
    pub non_discriminating: usize,
    /// Discriminating controls that pass: each blocks the gate.
    pub undiscriminated: Vec<String>,
    /// Non-discriminating controls that V-K finds failing anyway (a
    /// difference from R1's own analysis; listed).
    pub unexpectedly_failing: Vec<String>,
}

impl ControlTally {
    pub fn merge(&mut self, o: &ControlTally) {
        self.discriminated += o.discriminated;
        self.non_discriminating += o.non_discriminating;
        self.undiscriminated
            .extend(o.undiscriminated.iter().cloned());
        self.unexpectedly_failing
            .extend(o.unexpectedly_failing.iter().cloned());
    }
}
