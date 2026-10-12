//! B7a (b) Widen domain, stage 1: per-case basis custody (RV125 N-2) and member-load cases,
//! stepped through the actual W1 stages (prepare, native, freeze) and, at c = 3, published and
//! read by the full Rust reader (b7-plan ruling (6) moved the G0 table pins).
//! Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
use super::*;

/// n05's sensitive torque case and a renamed copy (`case-b`), each optionally naming a
/// temperature point whose E and G equal the base material's (the numerics are the base's).
fn two_torque_cases(selectors: [Option<&str>; 2]) -> Value {
    torque_cases(&selectors, &|_| 1.0)
}

/// n05's sensitive torque case and renamed copies (`case-b`, `case-c`, ...), one per selector,
/// each optionally naming a temperature point whose E and G are the base material's (Pa)
/// times `scale(point)`.
fn torque_cases(selectors: &[Option<&str>], scale: &dyn Fn(&str) -> f64) -> Value {
    let cases = (0..selectors.len()).map(|case| {
        let mut copy = n05_torques();
        if case > 0 {
            let tag = char::from(b'a' + u8::try_from(case).unwrap());
            copy["id"] = json!(format!("case-{tag}"));
            for load in copy["primitive_loads"].as_array_mut().unwrap() {
                load["id"] = json!(format!("{tag}:{}", load["id"].as_str().unwrap()));
            }
        }
        copy
    }).collect();
    let mut raw = n05_request(cases);
    let material = &mut raw["model"]["materials"][0];
    let (e, g) = (material["elastic_modulus"]["value"].as_f64().unwrap(), material["shear_modulus"]["value"].as_f64().unwrap());
    let mut points = Vec::new();
    for id in selectors.iter().flatten() {
        if !points.iter().any(|p: &Value| p["id"] == json!(id)) {
            points.push(json!({"id": id, "temperature": {"value": 20.0, "unit": "degC"},
                "elastic_modulus": {"value": e * scale(id), "unit": "Pa"}, "shear_modulus": {"value": g * scale(id), "unit": "Pa"},
                "provenance": INVENTED}));
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

/// B7a (b), stage 1 (review F3 b, c, e): at c = 3, with numerically distinct points (P1 is 1.25
/// times the base E and G, P2 0.8 times), each case's early source carries its own basis's
/// moduli (a selector case is captured at its basis state, not the base model's), every case
/// freezes, and the serialized receipt carries one basis per selector in first-use order, with
/// its moduli, and each case's own `material_basis_ref` on its ordinary attempt, product
/// attempt and source, also when a point is reused around a base case. W1 publishes that
/// successor (precommit's copy), and the full Rust reader, G0 first, finds it bound and
/// numerically eligible with the invocation.
#[test]
fn b7a_three_cases_capture_and_serialize_their_own_bases() {
    use crate::retained_tests_hooks as hooks;
    let scale = |id: &str| match id {
        "P1" => 1.25,
        "P2" => 0.8,
        other => panic!("point {other}"),
    };
    let (e, g) = (2.0e11, 8.0e10);
    for selectors in [[None, Some("P2"), Some("P1")], [Some("P1"), None, Some("P1")], [Some("P2"), Some("P2"), None]] {
        // The receipt's bases, one per selector in first-use order, and each case's index into
        // them; the capture's custody holds the named ones in the same order.
        let mut order: Vec<Option<&str>> = Vec::new();
        for selector in selectors {
            if !order.contains(&selector) {
                order.push(selector);
            }
        }
        let refs: Vec<usize> = selectors.iter().map(|s| order.iter().position(|o| o == s).unwrap()).collect();
        let named: Vec<&str> = order.iter().flatten().copied().collect();
        let raw = torque_cases(&selectors, &scale);
        for mode in MODES {
            let label = format!("{selectors:?} {mode:?}");
            let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
            assert_eq!(retained_memory::assess(&capture, &request, retained_memory::Entry::Direct).law().domain, None, "{label}");
            let (_, observer, _) = early_observed(&raw, mode, true);
            assert_eq!(observer.bases.len(), named.len(), "{label}: one custody per named basis");
            for (case, selector) in selectors.iter().enumerate() {
                let custody = selector.map(|id| named.iter().position(|n| *n == id).unwrap());
                assert_eq!(observer.member_loads_basis_at(case), (false, selector.is_some(), custody), "{label} case {case}");
                let s = selector.map_or(1.0, scale);
                assert_eq!(observer.source_moduli_at(case), Some(vec![(e * s, g * s)]), "{label} case {case}: the source at its basis state");
            }
            assert_eq!(stepped(&raw, mode, &|_| {}), ["Frozen", "Frozen", "Frozen"], "{label}");
            let (capture, observer, mut ordinary) = early_observed(&raw, mode, true);
            ordinary.diagnostics.shrink_to_fit();
            let ((_, retained), _, precommit) = hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
            let successor = match retained {
                Ok(successor) => successor.value().clone(),
                Err(fallback) => panic!("{label}: W1 fell back: {fallback:?}"),
            };
            assert_eq!(precommit.as_ref(), Some(&successor), "{label}: the published successor is precommit's copy");
            let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
            let verdict = reader::validate(&successor, Some(&invocation)).unwrap_or_else(|e| panic!("{label}: reader: {e:?}"));
            assert!(verdict.invocation_bound && verdict.numerical_eligible, "{label}: bound and eligible");
            let body = &successor["retained_precision"]["body"];
            let bases = body["material_bases"].as_array().unwrap();
            assert_eq!(bases.len(), order.len(), "{label}");
            for (index, (basis, selector)) in bases.iter().zip(&order).enumerate() {
                let (want, selection, s) = match selector {
                    None => (json!({"kind": "base"}), json!({"kind": "base"}), 1.0),
                    Some(id) => (json!({"kind": "named", "id": id}), json!({"kind": "named_point", "point_id": id}), scale(id)),
                };
                let cases: Vec<usize> = (0..3).filter(|&case| refs[case] == index).collect();
                assert_eq!((&basis["index"], &basis["selector"], &basis["case_indices"]), (&json!(index), &want, &json!(cases)), "{label}");
                let material = &basis["materials"][0];
                assert_eq!(material["selection"], selection, "{label}");
                assert_eq!(material["elastic_modulus"], json!(format!("{:016x}", (e * s).to_bits())), "{label} basis {index}");
                assert_eq!(material["shear_modulus"], json!(format!("{:016x}", (g * s).to_bits())), "{label} basis {index}");
            }
            for case in 0..3 {
                assert_eq!(body["cases"][case]["status"], "selected", "{label} case {case}");
                assert_eq!(body["ordinary_attempts"][case]["material_basis_ref"], json!(refs[case]), "{label} case {case}");
            }
            let attempts = body["product_attempts"].as_array().unwrap();
            let sources = body["sources"].as_array().unwrap();
            assert_eq!((attempts.len(), sources.len()), (3, 3), "{label}");
            for attempt in attempts {
                let case = usize::try_from(attempt["owner_ref"]["index"].as_u64().unwrap()).unwrap();
                assert_eq!(attempt["material_basis_ref"], json!(refs[case]), "{label}: product attempt of case {case}");
            }
            for source in sources {
                let case = usize::try_from(source["owner"]["case_index"].as_u64().unwrap()).unwrap();
                assert_eq!(source["material_basis_ref"], json!(refs[case]), "{label}: source of case {case}");
            }
        }
    }
}

/// B7a (b), stage 1 (review F3 d): a member-load case's late hook re-observes only that the
/// actual application still carries its member loads, and that the case holds no source. The
/// case's own application passes; one without the member load, or the case holding a source,
/// refuses by the member-load coverage cause.
#[test]
fn b7a_member_load_case_late_hook_re_observes_its_member_loads() {
    let mut raw = two_torque_cases([None, None]);
    raw["model"]["load_cases"][1]["primitive_loads"].as_array_mut().unwrap().push(json!({"id": "case-b:udl", "category": "distributed_force",
        "target": {"type": "element", "pipe": "pipe"}, "direction": "global_y", "magnitude": {"value": 10.0, "unit": "N/m"},
        "dimension": "force_per_length", "provenance": INVENTED}));
    let mut without = raw.clone();
    without["model"]["load_cases"][1]["primitive_loads"].as_array_mut().unwrap().pop();
    let coverage = Err("early member-load case coverage".to_owned());
    for mode in MODES {
        let (_, mut observer, _) = early_observed(&raw, mode, true);
        assert_eq!(observer.member_loads_basis_at(1).0, true, "{mode:?}: case 1 is the capture's current case");
        assert_eq!(observer.test_recheck_occurrences(&raw, 1).map_err(|e| e.to_string()), Ok(()), "{mode:?}");
        assert_eq!(observer.test_recheck_occurrences(&without, 1).map_err(|e| e.to_string()), coverage, "{mode:?}: member load gone");
        observer.source = observer.with_case(0, |c| c.source.take());
        assert!(observer.source.is_some(), "{mode:?}: case 0's source");
        assert_eq!(observer.test_recheck_occurrences(&raw, 1).map_err(|e| e.to_string()), coverage, "{mode:?}: a source");
    }
}
