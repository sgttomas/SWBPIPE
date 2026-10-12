//! B7a (b) Widen domain, stage 1: per-case basis custody (RV125 N-2) and member-load cases,
//! stepped through the actual W1 stages (prepare, native, freeze), below the held G0 table pins.
//! Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
use super::*;

/// n05's sensitive torque case and a renamed copy (`case-b`), each optionally naming a
/// temperature point whose E and G equal the base material's (the numerics are the base's).
fn two_torque_cases(selectors: [Option<&str>; 2]) -> Value {
    let mut b = n05_torques();
    b["id"] = json!("case-b");
    for load in b["primitive_loads"].as_array_mut().unwrap() {
        load["id"] = json!(format!("b:{}", load["id"].as_str().unwrap()));
    }
    let mut raw = n05_request(vec![n05_torques(), b]);
    let material = &mut raw["model"]["materials"][0];
    let (e, g) = (material["elastic_modulus"]["value"].clone(), material["shear_modulus"]["value"].clone());
    let mut points = Vec::new();
    for id in selectors.iter().flatten() {
        if !points.iter().any(|p: &Value| p["id"] == json!(id)) {
            points.push(json!({"id": id, "temperature": {"value": 20.0, "unit": "degC"},
                "elastic_modulus": {"value": e, "unit": "Pa"}, "shear_modulus": {"value": g, "unit": "Pa"}, "provenance": INVENTED}));
        }
    }
    material["temperature_points"] = json!(points);
    for (case, selector) in selectors.iter().enumerate() {
        if let Some(id) = selector {
            raw["model"]["load_cases"][case]["modulus_basis_ref"] = json!(id);
        }
    }
    raw
}

/// The actual W1 stages through freeze, with `before_freeze` applied to the prepared cases.
fn stepped(raw: &Value, mode: PreviewSolverMode, before_freeze: &dyn Fn(&mut rp::PreparedCases)) -> Vec<String> {
    let (capture, observer, mut ordinary) = early_observed(raw, mode, true);
    ordinary.diagnostics.shrink_to_fit();
    let cases = w1_case_ids(&capture).unwrap();
    let triggers: Vec<CaseTrigger> = case_triggers(&ordinary.numerical_quality, &observer.ordinary, cases.ids()).collect();
    let attempted = cases.attempted(triggers.iter().copied());
    let mut prepared = match observer.prepare_cases(ordinary, cases.ids().len(), attempted.requests()) {
        Ok(prepared) => prepared,
        Err(failure) => panic!("custody: {:?}", failure.error),
    };
    prepared.native();
    before_freeze(&mut prepared);
    prepared.freeze();
    prepared.attempts.iter().map(|attempt| match &attempt.end {
        rp::AttemptEnd::Frozen(_) => "Frozen".to_owned(),
        rp::AttemptEnd::Candidate(refused) => format!("Candidate({:?})", refused.error),
        rp::AttemptEnd::Preparation => format!("Preparation({})", prepared.capture.with_case(attempt.request, |c| format!("{:?}", c.error))),
        rp::AttemptEnd::Prepared => "Prepared".to_owned(),
        rp::AttemptEnd::Native => format!("Native({:?})", attempt.trace.native_error),
        rp::AttemptEnd::Selected => "Selected".to_owned(),
    }).collect()
}

/// RV125 N-2: at c = 2, a base case and a named case, a named case first, one point reused by
/// both cases, and two distinct points each bind their own basis custody, so both attempted cases
/// freeze. Binding a case to another case's custody, or a base case to any, refuses that case's
/// freeze by its custody cause, and leaves the other case frozen.
#[test]
fn b7a_selector_cases_bind_their_own_basis_custody() {
    let shapes: [(&str, [Option<&str>; 2], usize, [Option<usize>; 2]); 4] = [
        ("base, named", [None, Some("P1")], 1, [None, Some(0)]),
        ("named, base", [Some("P1"), None], 1, [Some(0), None]),
        ("reused", [Some("P1"), Some("P1")], 1, [Some(0), Some(0)]),
        ("distinct", [Some("P1"), Some("P2")], 2, [Some(0), Some(1)]),
    ];
    for (label, selectors, bases, bound) in shapes {
        let raw = two_torque_cases(selectors);
        for mode in MODES {
            let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
            assert_eq!(retained_memory::assess(&capture, &request, retained_memory::Entry::Direct).law().domain, None, "{label}");
            let (_, observer, ordinary) = early_observed(&raw, mode, true);
            assert_eq!(ordinary.status.mechanics, "MECHANICS_SOLVED", "{label}");
            assert_eq!(ordinary.results.iter().filter(|r| r.kind == "modulus_basis_record").count(),
                selectors.iter().flatten().count(), "{label}: one modulus row per named case");
            assert_eq!(observer.bases.len(), bases, "{label} {mode:?}: one custody per distinct basis");
            assert!(observer.bases.iter().all(|b| b.record_calls == 1 && b.record.is_some()), "{label}");
            for case in 0..2 {
                assert_eq!(observer.member_loads_basis_at(case), (false, selectors[case].is_some(), bound[case]), "{label} case {case}");
            }
            assert_eq!(stepped(&raw, mode, &|_| {}), ["Frozen", "Frozen"], "{label} {mode:?}");
            // Wrong custody: a base case bound to a basis, a named case to none, or a named case to
            // another basis's custody (whose record text differs).
            let wrong = match label {
                "base, named" => (0, Some(0)),
                "named, base" | "reused" => (0, None),
                _ => (1, Some(0)),
            };
            let ends = stepped(&raw, mode, &|prepared| {
                prepared.capture.with_case(wrong.0, |c| c.basis = wrong.1);
            });
            assert!(ends[wrong.0].starts_with("Candidate(") && (ends[wrong.0].contains("basis record capture presence")
                || ends[wrong.0].contains("modulus record association")), "{label} {mode:?}: {ends:?}");
            assert_eq!(ends[1 - wrong.0], "Frozen", "{label} {mode:?}: the other case keeps its own custody");
        }
    }
}

/// B7a (b), stage 1: a case with a full-span uniform member load (T18's shape) is admitted; its
/// capture builds no source, so it never refuses the invocation, and its attempt ends at
/// preparation with the named cause while the nodal case freezes. With the member load on case 0
/// the order is reversed. A member load on the exact route, a rotational or non-force_per_length
/// element load, and a self-weight id stay outside D1 (the law tests pin each fact).
#[test]
fn b7a_member_load_case_is_unavailable_by_its_named_cause() {
    let udl = |case: &str| json!({"id": format!("{case}:udl"), "category": "distributed_force", "target": {"type": "element", "pipe": "pipe"},
        "direction": "global_y", "magnitude": {"value": 10.0, "unit": "N/m"}, "dimension": "force_per_length", "provenance": INVENTED});
    for loaded in [1usize, 0] {
        let mut raw = two_torque_cases([None, None]);
        let id = raw["model"]["load_cases"][loaded]["id"].as_str().unwrap().to_owned();
        raw["model"]["load_cases"][loaded]["primitive_loads"].as_array_mut().unwrap().push(udl(&id));
        for mode in MODES {
            let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
            assert_eq!(retained_memory::assess(&capture, &request, retained_memory::Entry::Direct).law().domain, None);
            let (_, observer, _) = early_observed(&raw, mode, true);
            assert!(observer.error.is_none() && observer.basis_error.is_none());
            for case in 0..2 {
                assert_eq!(observer.member_loads_basis_at(case).0, case == loaded, "case {case}");
                assert_eq!(observer.source_terminal_at(case).0, case != loaded, "case {case}: only the nodal case has a source");
            }
            let ends = stepped(&raw, mode, &|_| {});
            assert_eq!(ends[1 - loaded], "Frozen", "loaded {loaded} {mode:?}");
            assert_eq!(ends[loaded], format!("Preparation(Some(Association({:?})))", rp::MEMBER_LOAD_CAUSE), "loaded {loaded} {mode:?}");
        }
    }
    // The early hook of a selector case is required: a named case whose basis-state capture never
    // ran keeps its pending slot, and installing it refuses that case.
    let raw = two_torque_cases([None, Some("P1")]);
    let (request, _) = source_receipt::CapturedInvocation::parse(raw, PreviewSolverMode::SparseInteractive).unwrap();
    let mut observer = rp::ProductCapture::prepared_probe_on(rp::W1Route::Preview);
    let refused = observer.test_install_pending(&request.model.load_cases[1]);
    assert_eq!(refused.map_err(|e| e.to_string()), Err("early basis source pending".to_owned()));
}
