// Unit tests of the sizing arithmetic. The frozen independent references are
// compared in tests/r3_references.rs; these pin the refusals, the test order and
// the worked values of the DEL-04-11 review (round 2 of PR #34).

use super::*;

fn method(design: LoadDesign, reference: VariationReference, limit: f64, a: f64, rule: SelectionRule) -> Method {
    Method { load_design: design, variation_reference: reference, variation_limit: limit, travel_allowance_m: a, selection_rule: rule }
}

fn variable(id: &str, k: f64, range: (f64, f64), travel: f64) -> Record {
    Record {
        hanger_id: id.to_string(),
        hanger_type: HangerType::VariableSpring,
        stiffness_n_per_m: Some(k),
        stiffness_dof: Some(Dof::Uz),
        working_load_range_n: Some(range),
        travel_range_m: Some(travel),
    }
}

fn constant(id: &str, range: (f64, f64), travel: f64) -> Record {
    Record {
        hanger_id: id.to_string(),
        hanger_type: HangerType::ConstantEffort,
        stiffness_n_per_m: None,
        stiffness_dof: None,
        working_load_range_n: Some(range),
        travel_range_m: Some(travel),
    }
}

fn candidate(id: &str, policy: FamilyPolicy, w: f64, u_c: f64, u_ops: &[f64]) -> Candidate {
    Candidate {
        id: id.to_string(),
        dof: Dof::Uz,
        family_policy: policy,
        w_n: w,
        positions: Positions { u_cold_m: u_c, u_operating_m: u_ops.to_vec() },
        design_index: 0,
    }
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-9 * a.abs().max(b.abs()).max(1.0)
}

#[test]
fn one_origin_hot_and_cold_design_reproduce_the_reviewed_values() {
    // W = 1000 N, k = 10 N/mm, u_C = 2 mm, u_D = 12 mm, u_2 = −8 mm, a = 1 mm.
    let c = candidate("c", FamilyPolicy::Variable, 1000.0, 0.002, &[0.012, -0.008]);
    let r = variable("v", 10_000.0, (500.0, 2000.0), 0.1);
    for (design, f_cold, f_c, f_d, f_2, envelope) in [
        (LoadDesign::Hot, 1120.0, 1100.0, 1000.0, 1200.0, (990.0, 1210.0)),
        (LoadDesign::Cold, 1020.0, 1000.0, 900.0, 1100.0, (890.0, 1110.0)),
    ] {
        let m = method(design, VariationReference::OperatingLoad, 1.0, 0.001, SelectionRule::SmallestCapacity);
        let Evaluation::Variable(Ok(loads)) = evaluate(&c, &r, &m) else { panic!("fits") };
        assert!(close(loads.f_cold_n, f_cold));
        assert!(close(loads.f_cold_condition_n, f_c));
        assert!(close(loads.f_operating_n[0], f_d));
        assert!(close(loads.f_operating_n[1], f_2));
        assert!(close(loads.envelope_n.0, envelope.0) && close(loads.envelope_n.1, envelope.1));
        assert!(close(Span::of(&c.positions).required_travel_m(0.001), 0.022));
    }
}

#[test]
fn downward_travel_includes_the_installed_position_in_the_span() {
    let c = candidate("c", FamilyPolicy::Variable, 1000.0, -0.002, &[-0.012]);
    let r = variable("v", 10_000.0, (500.0, 2000.0), 0.1);
    let m = method(LoadDesign::Hot, VariationReference::DesignLoad, 1.0, 0.0, SelectionRule::SmallestCapacity);
    let Evaluation::Variable(Ok(loads)) = evaluate(&c, &r, &m) else { panic!("fits") };
    assert!(close(loads.f_cold_n, 880.0) && close(loads.f_cold_condition_n, 900.0) && close(loads.f_operating_n[0], 1000.0));
    assert!(close(Span::of(&c.positions).required_travel_m(0.0), 0.012));
    let span = Span::of(&c.positions);
    let (lo, hi) = span.envelope_n(loads.f_cold_n, 10_000.0, 0.001);
    assert!(close(lo, 870.0) && close(hi, 1010.0));
    assert!(close(span.required_travel_m(0.001), 0.014));
}

#[test]
fn tests_apply_in_order_and_the_first_failure_is_the_reason() {
    let m = method(LoadDesign::Hot, VariationReference::OperatingLoad, 0.25, 0.0, SelectionRule::SmallestCapacity);
    let c = candidate("c", FamilyPolicy::Variable, 1000.0, 0.0, &[0.01]);
    let mut no_range = variable("a", 10_000.0, (500.0, 2000.0), 0.1);
    no_range.working_load_range_n = None;
    assert_eq!(evaluate(&c, &no_range, &m).failed(), Some(Test::E1));
    let mut inverted = variable("a", 10_000.0, (2000.0, 500.0), 0.1);
    assert_eq!(evaluate(&c, &inverted, &m).failed(), Some(Test::E1));
    inverted.working_load_range_n = Some((500.0, 2000.0));
    inverted.stiffness_dof = Some(Dof::Uy);
    assert_eq!(evaluate(&c, &inverted, &m).failed(), Some(Test::E2));
    // Cold design with large upward travel: F_i = W − k·u_i ≤ 0.
    let cold = method(LoadDesign::Cold, VariationReference::OperatingLoad, 10.0, 0.0, SelectionRule::SmallestCapacity);
    let up = candidate("c", FamilyPolicy::Variable, 1000.0, 0.0, &[0.2]);
    assert_eq!(evaluate(&up, &variable("a", 10_000.0, (1.0, 5000.0), 1.0), &cold).failed(), Some(Test::V0));
    // F_cold = 1100, span [0, 0.01]: envelope [1000, 1100].
    assert_eq!(evaluate(&c, &variable("a", 10_000.0, (1000.5, 2000.0), 0.1), &m).failed(), Some(Test::L));
    assert_eq!(evaluate(&c, &variable("a", 10_000.0, (500.0, 1099.5), 0.1), &m).failed(), Some(Test::L));
    assert_eq!(evaluate(&c, &variable("a", 10_000.0, (500.0, 2000.0), 0.005), &m).failed(), Some(Test::T));
    // v = 100/1000 = 10 % against a 5 % limit.
    let tight = method(LoadDesign::Hot, VariationReference::OperatingLoad, 0.05, 0.0, SelectionRule::SmallestCapacity);
    assert_eq!(evaluate(&c, &variable("a", 10_000.0, (500.0, 2000.0), 0.1), &tight).failed(), Some(Test::V));
}

#[test]
fn constant_records_test_range_and_travel_only() {
    let m = method(LoadDesign::Hot, VariationReference::OperatingLoad, 0.01, 0.002, SelectionRule::LeastVariation);
    let c = candidate("c", FamilyPolicy::Constant, 1000.0, 0.0, &[0.03]);
    // W at the range minimum passes; required travel is 0.03 + 2·0.002.
    assert_eq!(evaluate(&c, &constant("k", (1000.0, 1000.5), 0.035), &m), Evaluation::Constant(Ok(())));
    assert_eq!(evaluate(&c, &constant("k", (1000.5, 2000.0), 0.1), &m).failed(), Some(Test::L));
    assert_eq!(evaluate(&c, &constant("k", (500.0, 2000.0), 0.0339), &m).failed(), Some(Test::T));
}

#[test]
fn selection_rules_break_ties_in_their_stated_order() {
    let c = candidate("c", FamilyPolicy::Variable, 1000.0, 0.0, &[0.01]);
    // Variation k·0.01/1000: 5 % at k = 5000, 10 % at k = 10000.
    let records = vec![
        variable("b-large-stiff", 10_000.0, (500.0, 3000.0), 0.1),
        variable("a-small-stiff", 10_000.0, (500.0, 2000.0), 0.1),
        variable("c-small-soft", 5_000.0, (500.0, 2000.0), 0.1),
        variable("d-large-soft", 5_000.0, (500.0, 3000.0), 0.1),
        variable("0-small-soft", 5_000.0, (500.0, 2000.0), 0.1),
    ];
    let capacity = method(LoadDesign::Hot, VariationReference::OperatingLoad, 0.25, 0.0, SelectionRule::SmallestCapacity);
    let variation = method(LoadDesign::Hot, VariationReference::OperatingLoad, 0.25, 0.0, SelectionRule::LeastVariation);
    let pick = |m: &Method| {
        let result = size_candidate(&c, &records, m);
        records[result.selected.unwrap().record_index()].hanger_id.clone()
    };
    // Smallest F_max (2000) ties three; least variation (5 %) ties two; byte order.
    assert_eq!(pick(&capacity), "0-small-soft");
    // Least variation ties three soft records; smallest F_max ties two; byte order.
    assert_eq!(pick(&variation), "0-small-soft");
    // A softer, larger record (k = 2000: 2 %) wins least variation but not capacity.
    let mut more = records.clone();
    more.push(variable("e-large-softest", 2_000.0, (500.0, 3000.0), 0.1));
    assert_eq!(more[size_candidate(&c, &more, &variation).selected.unwrap().record_index()].hanger_id, "e-large-softest");
    assert_eq!(more[size_candidate(&c, &more, &capacity).selected.unwrap().record_index()].hanger_id, "0-small-soft");
}

#[test]
fn fallback_counts_and_no_fit() {
    let m = method(LoadDesign::Hot, VariationReference::OperatingLoad, 0.05, 0.0, SelectionRule::SmallestCapacity);
    let c = candidate("c", FamilyPolicy::VariableElseConstant, 1000.0, 0.0, &[0.01]);
    let records = vec![variable("v", 10_000.0, (500.0, 2000.0), 0.1), constant("k", (500.0, 2000.0), 0.1)];
    let result = size_candidate(&c, &records, &m);
    assert_eq!(result.outcome, Outcome::SelectedByFallback);
    assert_eq!(result.variable_failures.as_ref().unwrap()[&Test::V], 1);
    assert_eq!(result.constant_failures.as_ref().unwrap().values().sum::<usize>(), 0);
    assert!(matches!(result.selected, Some(Selected::Constant { load_n, .. }) if load_n == 1000.0));
    let only_variable = candidate("c", FamilyPolicy::Variable, 1000.0, 0.0, &[0.01]);
    let none = size_candidate(&only_variable, &records, &m);
    assert_eq!(none.outcome, Outcome::NoRecordFits);
    assert!(none.constant_failures.is_none());
    assert_eq!(verification_standing(&[result, none]), VerificationStanding::Conditional);
}

#[test]
fn every_request_refusal_is_listed_together() {
    let bad = method(LoadDesign::Hot, VariationReference::OperatingLoad, 0.0, -0.001, SelectionRule::SmallestCapacity);
    let mut nan = candidate("n", FamilyPolicy::Variable, 100.0, f64::NAN, &[0.0]);
    nan.design_index = 0;
    let mut missing = candidate("m", FamilyPolicy::Variable, 100.0, 0.0, &[]);
    missing.design_index = 0;
    let candidates = vec![
        candidate("a", FamilyPolicy::Variable, 0.0, 0.0, &[0.0]),
        candidate("b", FamilyPolicy::Variable, 10.0, 0.0, &[0.0]),
        candidate("c", FamilyPolicy::Variable, -5.0, 0.0, &[0.0]),
        candidate("b", FamilyPolicy::Variable, 10.0, 0.0, &[0.0]),
        nan,
        missing,
    ];
    let records = vec![constant("x", (1.0, 2.0), 1.0), constant("x", (1.0, 2.0), 1.0)];
    let refusals = size(&candidates, &records, &bad).unwrap_err();
    let codes: Vec<&str> = refusals.iter().map(|r| r.code).collect();
    assert_eq!(codes, [
        "HANGER_SIZING_VARIATION_LIMIT_INVALID",
        "HANGER_SIZING_TRAVEL_ALLOWANCE_INVALID",
        "HANGER_SIZING_CANDIDATE_DUPLICATE",
        "HANGER_SIZING_LIBRARY_RECORD_DUPLICATE",
        "HANGER_SIZING_DESIGN_OPERATING_CASE_REQUIRED",
        "HANGER_SIZING_DERIVED_SOLVE_FAILED",
        "HANGER_SIZING_LOAD_NOT_POSITIVE",
    ]);
    assert_eq!(refusals.last().unwrap().names, ["a", "c"]);
    assert_eq!(size(&[], &[], &method(LoadDesign::Hot, VariationReference::DesignLoad, 0.1, 0.0, SelectionRule::LeastVariation))
        .unwrap_err()[0].code, "HANGER_SIZING_CANDIDATES_REQUIRED");
}

#[test]
fn mixed_verification_rechecks_constant_travel_and_uses_both_verified_loads() {
    // Review round 2's witness, in SI: A variable (k = 2 N/mm, hot setting 120 N),
    // B constant (W = 100 N); verified u_C = (20/3, 20/3) mm, u_D = (10, 0) mm, a = 1 mm.
    let m = method(LoadDesign::Hot, VariationReference::OperatingLoad, 0.25, 0.001, SelectionRule::SmallestCapacity);
    let a = candidate("A", FamilyPolicy::Variable, 100.0, 0.0, &[0.010]);
    let b = candidate("B", FamilyPolicy::Constant, 100.0, 0.0, &[0.0]);
    let records = vec![variable("va", 2000.0, (50.0, 200.0), 0.05), constant("kb", (50.0, 200.0), 0.005)];
    let results = size(&[a.clone(), b.clone()], &records, &m).unwrap();
    assert_eq!(results[1].outcome, Outcome::Selected); // 2 mm required against 5 mm.
    let third = 0.02 / 3.0;
    let va = verify(&a, &results[0], &records, &m, &Positions { u_cold_m: third, u_operating_m: vec![0.010] }).unwrap().unwrap();
    let vv = va.variable.as_ref().unwrap();
    assert!(close(vv.f_cold_condition_n, 320.0 / 3.0) && close(vv.f_operating_n[0], 100.0));
    assert!(close(vv.variation[0].unwrap(), 2.0 / 30.0));
    assert!(close(vv.departure_n, 0.0));
    assert!(va.failed_tests.is_empty());
    let vb = verify(&b, &results[1], &records, &m, &Positions { u_cold_m: third, u_operating_m: vec![0.0] }).unwrap().unwrap();
    assert!(close(vb.required_travel_m, 0.026 / 3.0));
    assert!(!vb.t_pass);
    assert_eq!(vb.failed_tests, [Test::T]);
    assert!(vb.variable.is_none());
}

#[test]
fn verification_of_no_fit_is_absent_and_bad_positions_refuse() {
    let m = method(LoadDesign::Hot, VariationReference::OperatingLoad, 0.25, 0.0, SelectionRule::SmallestCapacity);
    let c = candidate("c", FamilyPolicy::Variable, 1000.0, 0.0, &[0.01]);
    let none = size_candidate(&c, &[], &m);
    assert_eq!(verify(&c, &none, &[], &m, &c.positions).unwrap(), None);
    let records = vec![variable("v", 10_000.0, (500.0, 2000.0), 0.1)];
    let some = size_candidate(&c, &records, &m);
    let refused = verify(&c, &some, &records, &m, &Positions { u_cold_m: 0.0, u_operating_m: vec![] }).unwrap_err();
    assert_eq!(refused.code, "HANGER_SIZING_DERIVED_SOLVE_FAILED");
}

#[test]
fn the_design_case_index_selects_the_setting_and_the_departure() {
    // Hot design with the second case as the design case: F_cold = W + k·u_2.
    let m = method(LoadDesign::Hot, VariationReference::DesignLoad, 1.0, 0.0, SelectionRule::SmallestCapacity);
    let mut c = candidate("c", FamilyPolicy::Variable, 1000.0, 0.0, &[0.01, 0.02]);
    c.design_index = 1;
    let records = vec![variable("v", 10_000.0, (500.0, 2000.0), 0.1)];
    let result = size_candidate(&c, &records, &m);
    let Some(Selected::Variable { loads, .. }) = &result.selected else { panic!("fits") };
    assert!(close(loads.f_cold_n, 1200.0));
    assert!(close(loads.f_operating_n[0], 1100.0) && close(loads.f_operating_n[1], 1000.0));
    // Verified positions differ per case: the departure reads the design case, F_2^v − W.
    let verified = Positions { u_cold_m: 0.0, u_operating_m: vec![0.005, 0.025] };
    let v = verify(&c, &result, &records, &m, &verified).unwrap().unwrap();
    assert!(close(v.variable.unwrap().departure_n, -50.0));
}

#[test]
fn a_non_positive_verified_reference_load_forms_no_ratio() {
    // F_cold = 1100; verified u_D = 0.12 gives F_D^v = −100: no ratio, V0 reports it.
    let m = method(LoadDesign::Hot, VariationReference::OperatingLoad, 0.25, 0.0, SelectionRule::SmallestCapacity);
    let c = candidate("c", FamilyPolicy::Variable, 1000.0, 0.0, &[0.01]);
    let records = vec![variable("v", 10_000.0, (500.0, 2000.0), 1.0)];
    let result = size_candidate(&c, &records, &m);
    let v = verify(&c, &result, &records, &m, &Positions { u_cold_m: 0.0, u_operating_m: vec![0.12] }).unwrap().unwrap();
    assert_eq!(v.variable.as_ref().unwrap().variation, [None]);
    assert!(v.failed_tests.contains(&Test::V0) && !v.failed_tests.contains(&Test::V));
}
