//! The kernel lane at CI scale (plan §5–§8, §12): every R1 case of a family
//! with at most 100 members (and RF-MECH-LINE-IN-CHAIN1000, refused by
//! geometry before any factor) through `solve_case`, judged by the unchanged
//! predicate, with the report's counts pinned, zero failures, the not-covered
//! set equal to the committed list, the class correspondence, the
//! discrimination check, and the committed per-case records.
//!
//! The pins are plan §6.3's projection from R1's data, not from a run.
use piping_numerical_robustness::cases::{crate_dir, load_family, FAMILY_FILES};
use piping_numerical_robustness::compare::{ControlTally, Tally};
use piping_numerical_robustness::lane::run_case;
use piping_numerical_robustness::records::family_file;
use std::collections::BTreeMap;

struct Pin {
    cases: usize,
    tally: Tally,
    /// (selected, refused, unresolved)
    outcomes: (usize, usize, usize),
    discriminated: usize,
    non_discriminating: usize,
    input_derived_not_covered: &'static [&'static str],
}

fn tally(rows: usize, pass: usize, not_covered: usize, structural_zero: usize) -> Tally {
    Tally {
        rows,
        pass,
        pass_absolute_range: 0,
        not_covered,
        structural_zero,
        expected_unresolved: 0,
        fail: 0,
    }
}

fn lane(family: &str, pin: Pin) {
    let cases: Vec<_> = load_family(family)
        .into_iter()
        .filter(|c| !c.is_large())
        .collect();
    assert_eq!(cases.len(), pin.cases, "{family}: cases");
    let mut total = Tally::default();
    let mut controls = ControlTally::default();
    let mut failures = Vec::new();
    let mut outcomes: BTreeMap<String, usize> = BTreeMap::new();
    let mut input_derived = Vec::new();
    let mut records = Vec::new();
    for case in &cases {
        let run = run_case(case);
        assert!(run.tally.accounted(), "{}: row accounting", case.id);
        total.merge(&run.tally);
        controls.merge(&run.controls);
        failures.extend(run.failures.iter().cloned());
        failures.extend(run.class_mismatches.iter().map(|m| format!("CLASS {m}")));
        if run.not_covered != case.not_covered {
            failures.push(format!(
                "FLOOR {}: not covered {:?}, committed {:?}",
                case.id, run.not_covered, case.not_covered
            ));
        }
        input_derived.extend(run.input_derived_not_covered.iter().cloned());
        *outcomes.entry(run.outcome.clone()).or_default() += 1;
        records.push(run.record);
    }
    println!("{family}: {total}");
    println!("{family}: outcomes {outcomes:?}");
    println!(
        "{family}: controls discriminated {}, non-discriminating {}",
        controls.discriminated, controls.non_discriminating
    );
    for f in &failures {
        println!("{family}: FAILURE {f}");
    }
    assert!(failures.is_empty(), "{family}: {} failures", failures.len());
    assert_eq!(total, pin.tally, "{family}: the report's counts");
    let count = |k: &str| outcomes.get(k).copied().unwrap_or(0);
    assert_eq!(
        (count("selected"), count("refused"), count("unresolved")),
        pin.outcomes,
        "{family}: outcomes"
    );
    assert!(
        controls.undiscriminated.is_empty(),
        "{family}: discriminating controls that pass: {:?}",
        controls.undiscriminated
    );
    assert!(
        controls.unexpectedly_failing.is_empty(),
        "{family}: {:?}",
        controls.unexpectedly_failing
    );
    assert_eq!(
        (controls.discriminated, controls.non_discriminating),
        (pin.discriminated, pin.non_discriminating),
        "{family}: controls"
    );
    assert_eq!(
        input_derived, pin.input_derived_not_covered,
        "{family}: not-covered rows published as input-derived"
    );
    let (_, file) = FAMILY_FILES.iter().find(|(f, _)| *f == family).unwrap();
    let path = crate_dir()
        .join("observations/kernel_lane")
        .join(file.replace(".jsonl", ".json"));
    let committed =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    assert!(
        committed == family_file(&records),
        "{family}: the per-case records differ from {} (regenerate only by decision: \
         cargo run --release --example vk_records -- --write {family})",
        path.display()
    );
}

#[test]
fn rf_chain() {
    lane(
        "RF-CHAIN",
        Pin {
            cases: 30,
            tally: tally(2760, 2700, 0, 60),
            outcomes: (30, 0, 0),
            discriminated: 82,
            non_discriminating: 38,
            input_derived_not_covered: &[],
        },
    );
}

#[test]
fn rf_skew() {
    lane(
        "RF-SKEW",
        Pin {
            cases: 36,
            tally: tally(1080, 982, 2, 96),
            outcomes: (36, 0, 0),
            discriminated: 105,
            non_discriminating: 33,
            input_derived_not_covered: &[],
        },
    );
}

#[test]
fn rf_weak() {
    lane(
        "RF-WEAK",
        Pin {
            cases: 9,
            tally: tally(618, 566, 46, 6),
            outcomes: (9, 0, 0),
            discriminated: 23,
            non_discriminating: 13,
            input_derived_not_covered: &[
                "RF-WEAK-W-AX-rho1e-12:u.N5.UX",
                "RF-WEAK-W-AX-rho1e-12:u.N5.UY",
                "RF-WEAK-W-AX-rho1e-12:u.N5.UZ",
            ],
        },
    );
}

#[test]
fn rf_large_at_10_and_100_members() {
    lane(
        "RF-LARGE",
        Pin {
            cases: 12,
            tally: tally(9054, 9054, 0, 0),
            outcomes: (12, 0, 0),
            discriminated: 38,
            non_discriminating: 0,
            input_derived_not_covered: &[],
        },
    );
}

#[test]
fn rf_invariance() {
    lane(
        "RF-INVARIANCE",
        Pin {
            cases: 25,
            tally: tally(7096, 7056, 0, 40),
            outcomes: (25, 0, 0),
            discriminated: 47,
            non_discriminating: 7,
            input_derived_not_covered: &[],
        },
    );
}

#[test]
fn rf_range() {
    // ROOT's ruling on I17's A1 stop: THIN-A and THIN-B (25 rows each) are on
    // the expected-unresolved list; their rows are counted apart, never as
    // passes.
    let mut t = tally(2720, 2590, 0, 80);
    t.expected_unresolved = 50;
    lane(
        "RF-RANGE",
        Pin {
            cases: 32,
            tally: t,
            outcomes: (30, 0, 2),
            discriminated: 80,
            non_discriminating: 12,
            input_derived_not_covered: &[],
        },
    );
}

#[test]
fn rf_zero() {
    lane(
        "RF-ZERO",
        Pin {
            cases: 4,
            tally: tally(158, 158, 0, 0),
            outcomes: (4, 0, 0),
            discriminated: 9,
            non_discriminating: 0,
            input_derived_not_covered: &[],
        },
    );
}

#[test]
fn rf_finite() {
    lane(
        "RF-FINITE",
        Pin {
            cases: 6,
            tally: tally(748, 748, 0, 0),
            outcomes: (6, 0, 0),
            discriminated: 6,
            non_discriminating: 6,
            input_derived_not_covered: &[],
        },
    );
}

#[test]
fn rf_mech() {
    lane(
        "RF-MECH",
        Pin {
            cases: 9,
            tally: tally(26, 26, 0, 0),
            outcomes: (1, 8, 0),
            discriminated: 26,
            non_discriminating: 0,
            input_derived_not_covered: &[],
        },
    );
}

#[test]
fn rf_cancel() {
    lane(
        "RF-CANCEL",
        Pin {
            cases: 38,
            tally: tally(1444, 1441, 3, 0),
            outcomes: (38, 0, 0),
            discriminated: 90,
            non_discriminating: 62,
            input_derived_not_covered: &[],
        },
    );
}
