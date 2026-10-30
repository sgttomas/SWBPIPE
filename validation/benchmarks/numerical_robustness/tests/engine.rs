//! The engine and the controls on constructed inputs (plan §14.1's `engine.rs`
//! and `controls.rs` items; checkpoint C's harness mutants VK-H6, VK-H9 and
//! VK-H11, which R1's committed rows cannot discriminate).
//! - The predicate's comparison scale is max(|exp|, scale), never |obs|: a
//!   decimal exp just below 1 puts obs = 1 between the two tolerances.
//! - A row whose expected value has no binary64 value is its own verdict,
//!   `pass_absolute_range`, and the report counts it apart from the passes
//!   (R1's five rows; CI's lane has none, so the lane pins cannot see it).
//! - A discriminating control that passes is listed, never dropped, and a
//!   failing one is counted (a constructed case: every committed
//!   discriminating control fails).
//! - RV21-2 (RV21's drafted tests; plan §5.2 and §6.4): a wrong observation of
//!   a sub-range row fails, never an absolute-range pass; an `Overflow` row
//!   fails through the lane's own observation (RV21's mutants H6 and H4).
use open_pipe_stress_frame_kernel::structural::retained_api::{
    Binary64Outcome, Kind, PublishedRow, QuantityId, RowClass,
};
use piping_numerical_robustness::cases::{cases_dir, load_family, parse_case, Target};
use piping_numerical_robustness::compare::{judge, Observed, Tally, Verdict};
use piping_numerical_robustness::exact::{self, Exact};
use piping_numerical_robustness::lane::{observe, value_controls};
use serde_json::Value;
use std::collections::BTreeMap;

#[test]
fn the_comparison_scale_is_exps_magnitude_never_obss() {
    // exp = 0.999999999, scale 0: t = 0.999999999e-9 < |1 - exp| = 1e-9, so
    // obs = 1 fails; with |obs| in place of |exp| the tolerance would be 1e-9
    // and it would pass.
    let exp = Exact::parse_decimal("0.999999999").unwrap();
    let zero = Exact::zero();
    assert!(!exact::predicate(&Exact::from_f64(1.0), &exp, &zero));
    // The mirror: exp = 1.000000001 and obs = 1; |exp| is the larger, so
    // t = 1.000000001e-9 ≥ |Δ| = 1e-9, and it passes.
    let exp = Exact::parse_decimal("1.000000001").unwrap();
    assert!(exact::predicate(&Exact::from_f64(1.0), &exp, &zero));
    // Through the verdict: a covered row.
    let exp = Exact::parse_decimal("0.999999999").unwrap();
    assert_eq!(
        judge(&Observed::Value(1.0), &exp, &zero, true),
        Verdict::Fail("predicate".into())
    );
}

#[test]
fn r1s_five_absolute_range_rows_are_counted_apart_from_the_passes() {
    let rows: Value = serde_json::from_str(
        &std::fs::read_to_string(cases_dir().join("absolute_range_rows.json")).unwrap(),
    )
    .unwrap();
    let mut t = Tally::default();
    for r in rows.as_array().unwrap() {
        let exp = Exact::parse_decimal(r["expected"].as_str().unwrap()).unwrap();
        let scale = Exact::parse_decimal(r["scale"].as_str().unwrap()).unwrap();
        assert!(exact::outside_binary64(&exp), "{r}");
        // The kernel's binary64 rounding of such a value is ±0.
        let obs = if exp.is_negative() { -0.0 } else { 0.0 };
        let v = judge(&Observed::Value(obs), &exp, &scale, true);
        assert_eq!(v, Verdict::PassAbsoluteRange, "{r}");
        t.add(&v);
    }
    // A row inside binary64's range, for contrast.
    let one = Exact::from_f64(1.0);
    let v = judge(&Observed::Value(1.0), &one, &Exact::zero(), true);
    assert_eq!(v, Verdict::Pass);
    t.add(&v);
    assert!(t.accounted());
    assert_eq!(
        (t.rows, t.pass, t.pass_absolute_range, t.not_covered, t.fail),
        (6, 1, 5, 0, 0)
    );
}

#[test]
fn a_discriminating_control_that_passes_is_listed_never_dropped() {
    // One row, exp = 1 on a class scale of 1: t = 1e-9. NC-FAILS's value is
    // outside it; NC-PASSES's is inside; NC-NON is R1's non-discriminating kind.
    let line = r#"{"basis":"intended","controls":[["NC-FAILS",true,"value",{"u.N1.UX":"1.1"}],["NC-PASSES",true,"value",{"u.N1.UX":"1.0000000001"}],["NC-NON",false,"value",{"u.N1.UX":"1"}]],"family":"RF-TEST","id":"RF-TEST-CONTROLS","k4src_sha256":"","model":null,"needs_directional_spring":false,"not_covered":[],"refuse":false,"rows":[["u.N1.UX","1","translation",null]],"scales":{"translation":"1"},"units":"SI"}"#;
    let case = parse_case(line);
    let t = value_controls(&case);
    assert_eq!(t.discriminated, 1);
    assert_eq!(t.undiscriminated, ["RF-TEST-CONTROLS:NC-PASSES"]);
    assert_eq!(t.non_discriminating, 1);
    assert!(t.unexpectedly_failing.is_empty());
}

/// RV21-2 (plan §6.4): on R1's five sub-range rows, a normal value beyond
/// 1e-9·scale fails (it is never an absolute-range pass).
#[test]
fn a_wrong_value_on_a_sub_range_row_fails() {
    let rows: Value = serde_json::from_str(
        &std::fs::read_to_string(cases_dir().join("absolute_range_rows.json")).unwrap(),
    )
    .unwrap();
    for r in rows.as_array().unwrap() {
        let exp = Exact::parse_decimal(r["expected"].as_str().unwrap()).unwrap();
        let scale = Exact::parse_decimal(r["scale"].as_str().unwrap()).unwrap();
        let t = exact::tolerance(&exp, &scale).approx();
        for obs in [2.0 * t, -2.0 * t, 1.0] {
            let v = judge(&Observed::Value(obs), &exp, &scale, true);
            assert!(matches!(v, Verdict::Fail(_)), "{r} {obs:e}: {v:?}");
        }
    }
}

/// RV21-2 (plan §5.2): an `Overflow` row fails through the lane's own
/// observation.
#[test]
fn an_overflowed_published_row_fails() {
    let case = load_family("RF-CHAIN")
        .into_iter()
        .find(|c| c.id == "RF-CHAIN-T-n03-r1e-04")
        .unwrap();
    let model = case.model.as_ref().unwrap();
    let row = case.rows.iter().find(|r| r.key.starts_with("th.")).unwrap();
    let target = model.resolve(&row.key).unwrap();
    let Target::Displacement(d) = target else {
        panic!("{target:?}")
    };
    let id = QuantityId::Displacement(d);
    let mut rows = BTreeMap::new();
    for negative in [false, true] {
        rows.insert(
            id,
            PublishedRow {
                id,
                kind: Kind::Rotation,
                body: 0,
                value: Binary64Outcome::Overflow { negative },
                class: RowClass::Unpublishable,
            },
        );
        let obs = observe(target, &rows, model);
        assert!(matches!(obs, Observed::Unavailable(_)), "{obs:?}");
        let exp = Exact::parse_decimal(&row.expected).unwrap();
        let scale = Exact::parse_decimal(case.scale_of(row)).unwrap();
        assert!(matches!(judge(&obs, &exp, &scale, true), Verdict::Fail(_)));
    }
}
