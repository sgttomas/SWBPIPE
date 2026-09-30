//! V-K's seeded faults (T3 D1 §4.10 and §7.3; the V-K brief's Scope 9; ROOT's
//! rulings on I17's plan, Q4 and Q8).
//!
//! This module and every fault site are compiled only under
//! `cfg(any(test, feature = "mutation-controls"))`. No product manifest
//! enables the feature: `numerical_robustness`'s `seeded-faults` feature does,
//! for its mutation run only, and its CI test guards the manifests. With the
//! feature off and outside `cfg(test)` the code is absent.
//!
//! The active fault is read once per process from the environment variable
//! `FK_SEEDED_FAULT`. Unset, empty or `NONE` means no fault, so FK's own test
//! build is unchanged in effect. An unknown id panics, so a misspelt fault
//! cannot run as NONE. The mutation run starts one process per fault.
use super::source::PrimitiveSource;
use super::wide::{Wide2, WideArith};
use std::sync::OnceLock;

/// The seeded faults of V-K's plan §10.4 (VK-F05 added by ROOT's ruling on
/// I17's A1 stop).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Fault {
    /// §7.3-1: the assembled K promoted from binary64.
    F01,
    /// §7.3-2: u rounded to binary64 before recovery.
    F02,
    /// §7.3-4: the smallest contribution to each diagonal entry dropped.
    F03,
    /// §7.3-4: e_x's first two components swapped in the member frame.
    F04,
    /// §7.3-5: no escalation after a rejected candidate.
    F05,
    /// §7.3-6: the candidate accepted without the verification's verdict.
    F06,
    /// §7.3-7: the geometric mechanism check disabled.
    F07,
    /// §7.3-8: member 1's i–j coupling block left out, sparse mode only.
    F08,
    /// §7.3-10 (list-order form): the canonical sort skipped.
    F10,
    /// §7.3-13: the loads folded at p = 128 in listed order.
    F13,
    /// §7.3-17: every scaled row classified `relative_verified`.
    F17,
    /// R7-M2: ê uncoupled.
    R02,
    /// R7-M28: no shift (Uc alone).
    R28,
    /// A directional spring's k·n·nᵀ without the division by nᵀn.
    S1,
    /// Reactions with the opposite sign.
    S2,
}

pub(crate) const IDS: [(&str, Fault); 15] = [
    ("VK-F01", Fault::F01),
    ("VK-F02", Fault::F02),
    ("VK-F03", Fault::F03),
    ("VK-F04", Fault::F04),
    ("VK-F05", Fault::F05),
    ("VK-F06", Fault::F06),
    ("VK-F07", Fault::F07),
    ("VK-F08", Fault::F08),
    ("VK-F10", Fault::F10),
    ("VK-F13", Fault::F13),
    ("VK-F17", Fault::F17),
    ("VK-R02", Fault::R02),
    ("VK-R28", Fault::R28),
    ("VK-S1", Fault::S1),
    ("VK-S2", Fault::S2),
];

/// The fault an id names; None for no fault; a panic for an unknown id.
/// (Tested by V-K's mutation run, which starts the test binaries with NONE,
/// every id and an unknown id; FK's own suite is left unchanged.)
fn parse(id: &str) -> Option<Fault> {
    if id.is_empty() || id == "NONE" {
        return None;
    }
    match IDS.iter().find(|(name, _)| *name == id) {
        Some((_, fault)) => Some(*fault),
        None => panic!("FK_SEEDED_FAULT: unknown fault id {id:?}"),
    }
}

/// Whether `fault` is the process's seeded fault.
pub(crate) fn active(fault: Fault) -> bool {
    static SELECTED: OnceLock<Option<Fault>> = OnceLock::new();
    *SELECTED.get_or_init(|| match std::env::var("FK_SEEDED_FAULT") {
        Ok(id) => parse(&id),
        Err(_) => None,
    }) == Some(fault)
}

/// VK-F13: each loaded DOF's contributions folded one by one at p = 128 in
/// the source's canonical (listed) order, each partial sum rounded to 128
/// bits, then split exactly into binary64 terms for the ledger's accumulator.
/// (DOF, terms, whether any contribution is nonzero), DOFs ascending.
pub(crate) fn folded_loads(source: &PrimitiveSource) -> Vec<(usize, Vec<f64>, bool)> {
    let mut arith = WideArith::new(128).expect("p = 128");
    let mut out: Vec<(usize, Vec<f64>, bool)> = Vec::new();
    let mut current: Option<(usize, Wide2, bool)> = None;
    let lift = |v: f64| Wide2::from_f64(v).expect("finite load");
    let close = |entry: (usize, Wide2, bool), out: &mut Vec<(usize, Vec<f64>, bool)>| {
        let split = entry.1.split_binary64().expect("in range");
        out.push((entry.0, split.terms().to_vec(), entry.2));
    };
    for load in source.loads() {
        let dof = load.dof.global();
        current = Some(match current.take() {
            Some((d, partial, nonzero)) if d == dof => (
                d,
                arith.add(&partial, &lift(load.value)).expect("fold at 128"),
                nonzero || load.value != 0.0,
            ),
            Some(done) => {
                close(done, &mut out);
                (dof, lift(load.value), load.value != 0.0)
            }
            None => (dof, lift(load.value), load.value != 0.0),
        });
    }
    if let Some(done) = current {
        close(done, &mut out);
    }
    out
}
