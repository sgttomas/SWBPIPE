//! B7a (b) Widen domain, stage 1: per-case basis custody (RV125 N-2) and member-load cases,
//! stepped through the actual W1 stages (prepare, native, freeze) and, at c = 3, published and
//! read by the full Rust reader (b7-plan ruling (6) moved the G0 table pins).
//! Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
use super::*;

/// These tests read only the D1 domain, which never reads the caller's limit L.
const DOMAIN_ONLY: RetainedByteLimit = RetainedByteLimit::new(0);

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
            assert_eq!(retained_memory::assess(&capture, &request, retained_memory::Entry::Direct, DOMAIN_ONLY).law().domain, None, "{label}");
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

/// Stage 2: authored full-span uniform loads keep per-case custody and use
/// member-aware native formation while the adjacent nodal case stays unchanged.
#[test]
fn w1_member_load_cases_form_and_freeze_in_both_case_orders() {
    let udl = |case: &str| json!({"id": format!("{case}:udl"), "category": "distributed_force", "target": {"type": "element", "pipe": "pipe"},
        "direction": "global_x", "magnitude": {"value": 10.0, "unit": "N/m"}, "dimension": "force_per_length", "provenance": INVENTED});
    for loaded in [1usize, 0] {
        let mut raw = two_torque_cases([None, None]);
        let id = raw["model"]["load_cases"][loaded]["id"].as_str().unwrap().to_owned();
        raw["model"]["load_cases"][loaded]["primitive_loads"].as_array_mut().unwrap().push(udl(&id));
        for mode in MODES {
            let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
            assert_eq!(retained_memory::assess(&capture, &request, retained_memory::Entry::Direct, DOMAIN_ONLY).law().domain, None);
            let (_, observer, _) = early_observed(&raw, mode, true);
            assert!(observer.error.is_none() && observer.basis_error.is_none(), "{:?}", observer.error);
            for case in 0..2 {
                assert_eq!(observer.member_loads_basis_at(case).0, case == loaded, "case {case}");
                assert!(observer.source_terminal_at(case).0, "case {case}: nodal and member cases have a source");
                assert_eq!(observer.member_source_at(case).is_some(), case==loaded);
            }
            let ends = stepped(&raw, mode, &|_| {});
            assert_eq!(ends, ["Frozen", "Frozen"], "loaded {loaded} {mode:?}");
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
            assert_eq!(retained_memory::assess(&capture, &request, retained_memory::Entry::Direct, DOMAIN_ONLY).law().domain, None, "{label}");
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

/// Late re-observation binds the member sidecar's actual authored occurrence,
/// including bits, target and count, before any preparation or publication.
#[test]
fn w1_member_load_late_hook_reobserves_exact_occurrences() {
    let mut raw = two_torque_cases([None, None]);
    raw["model"]["load_cases"][1]["primitive_loads"].as_array_mut().unwrap().push(json!({"id": "case-b:udl", "category": "distributed_force",
        "target": {"type": "element", "pipe": "pipe"}, "direction": "global_y", "magnitude": {"value": 10.0, "unit": "N/m"},
        "dimension": "force_per_length", "provenance": INVENTED}));
    let mut without = raw.clone();
    without["model"]["load_cases"][1]["primitive_loads"].as_array_mut().unwrap().pop();
    let mut changed=raw.clone();
    changed["model"]["load_cases"][1]["primitive_loads"].as_array_mut().unwrap().last_mut().unwrap()["magnitude"]["value"]=json!(11.0);
    for mode in MODES {
        let (_, mut observer, _) = early_observed(&raw, mode, true);
        assert!(observer.member_loads_basis_at(1).0);
        assert!(observer.test_recheck_occurrences(&raw,1).is_ok());
        assert!(observer.test_recheck_occurrences(&without,1).is_err());
        assert_eq!(observer.test_recheck_occurrences(&changed,1).map_err(|e|e.to_string()),Err("early member occurrence identity/bits".to_owned()));
        observer.member_sources.as_mut().unwrap().clear();
        assert_eq!(observer.test_recheck_occurrences(&raw,1).map_err(|e|e.to_string()),Err("early member-load case coverage".to_owned()));
    }
}

/// Maintained real product writer: capture -> preparation -> native solve ->
/// corrected recovery/certificate -> serialized successor -> full native reader.
/// Optional output is evidence for the Python and desktop wasm consumers.
#[test]
fn w1_member_load_product_consuming_successor() {
    use crate::retained_tests_hooks as hooks;
    let mut raw=two_torque_cases([None,Some("P1")]);
    for (case,axis,value) in [(0,"global_x",10.0),(1,"global_x",-7.0)] {
        let id=raw["model"]["load_cases"][case]["id"].as_str().unwrap().to_owned();
        raw["model"]["load_cases"][case]["primitive_loads"].as_array_mut().unwrap().push(json!({
            "id":format!("{id}:udl"),"category":"distributed_force","target":{"type":"element","pipe":"pipe"},
            "direction":axis,"magnitude":{"value":value,"unit":"N/m"},"dimension":"force_per_length","provenance":INVENTED}));
    }
    for mode in MODES {
        let (capture,observer,mut ordinary)=early_observed(&raw,mode,true);
        ordinary.diagnostics.shrink_to_fit();
        let ((_,retained),_,precommit)=hooks::counted_with_successor(||retained_w1(observer,ordinary,&capture));
        if let (Some(dir),Some(candidate))=(std::env::var_os("I61_U1_OUT"),precommit.as_ref()) {
            let name=format!("w1_member_load_{}",mode.as_str());
            let document=json!({"id":name,"source":candidate,"invocation":{"request":raw,"solver_mode":mode.as_str()}});
            std::fs::write(std::path::Path::new(&dir).join(format!("{name}.json")),serde_json::to_string_pretty(&document).unwrap()+"\n").unwrap();
        }
        let successor=retained.unwrap_or_else(|fallback|panic!("{mode:?}: {fallback:?}")).value().clone();
        assert_eq!(precommit.as_ref(),Some(&successor));
        let invocation=json!({"request":raw,"solver_mode":mode.as_str()});
        let verdict=reader::validate(&successor,Some(&invocation)).unwrap_or_else(|e|panic!("{mode:?}: {e:?}"));
        assert!(verdict.invocation_bound && verdict.numerical_eligible);
        let b=&successor["retained_precision"]["body"];
        for case in 0..2 {
            assert_eq!(b["cases"][case]["status"],"selected");
            let source=b["sources"].as_array().unwrap().iter().find(|s|s["owner"]["case_index"]==case).unwrap();
            assert_eq!(source["member_load_terms"].as_array().unwrap().len(),1);
        }
        assert_eq!(successor["diagnostics"].as_array().unwrap().iter().filter(|d|d["code"]==super::retained_wire::SELECTED_CODE
            && d["message"]==super::retained_wire::SELECTED_MEMBER_LOAD_MESSAGE).count(),2);
        if let Some(dir)=std::env::var_os("I61_U1_OUT") {
            let name=format!("w1_member_load_{}",mode.as_str());
            let document=json!({"id":name,"source":successor,"invocation":invocation});
            std::fs::write(std::path::Path::new(&dir).join(format!("{name}.json")),serde_json::to_string_pretty(&document).unwrap()+"\n").unwrap();
        }
    }
}

/// A real transverse case whose readout uncertainty prevents the search's
/// tighter stopping enclosure. Exhaustion publishes no retained rows for that
/// case; its exact diagnostic and all spent arithmetic survive beside a
/// successfully retained nodal case.
#[test]
fn w1_member_load_exhaustion_retains_typed_interval_work() {
    use crate::retained_tests_hooks as hooks;
    let mut raw=two_torque_cases([None,None]);
    raw["model"]["load_cases"][1]["primitive_loads"].as_array_mut().unwrap().push(json!({
        "id":"case-b:udl","category":"distributed_force","target":{"type":"element","pipe":"pipe"},
        "direction":"global_y","magnitude":{"value":10.0,"unit":"N/m"},"dimension":"force_per_length","provenance":INVENTED}));
    for mode in MODES {
        let (capture,observer,mut ordinary)=early_observed(&raw,mode,true);
        ordinary.diagnostics.shrink_to_fit();
        let ((_,retained),_,precommit)=hooks::counted_with_successor(||retained_w1(observer,ordinary,&capture));
        if let (Some(dir),Some(candidate))=(std::env::var_os("I61_U1_OUT"),precommit.as_ref()) {
            let name=format!("w1_member_exhaustion_{}",mode.as_str());
            let document=json!({"id":name,"source":candidate,"invocation":{"request":raw,"solver_mode":mode.as_str()}});
            std::fs::write(std::path::Path::new(&dir).join(format!("{name}.json")),serde_json::to_string_pretty(&document).unwrap()+"\n").unwrap();
        }
        let successor=retained.unwrap_or_else(|fallback|panic!("{mode:?}: {fallback:?}")).value().clone();
        let invocation=json!({"request":raw,"solver_mode":mode.as_str()});
        let verdict=reader::validate(&successor,Some(&invocation)).unwrap_or_else(|e|panic!("{mode:?}: {e:?}"));
        assert!(verdict.invocation_bound);
        assert!(!verdict.numerical_eligible,"the unavailable case keeps invocation-wide eligibility withheld");
        let b=&successor["retained_precision"]["body"];
        assert_eq!(b["cases"][0]["status"],"selected");
        assert_eq!(b["cases"][1]["status"],"unavailable");
        let attempt=&b["product_attempts"][1];
        assert_eq!(attempt["stages"]["projection"],"failed");
        for stage in ["maxima","values","aliases","certificate","observables","g5a"] {
            assert_eq!(attempt["stages"][stage],"not_entered");
        }
        assert!(attempt["proof"]["loaded_maximum_diagnostic"].is_object());
        assert_eq!(attempt["proof"]["numeric"]["wide_lme"]["kind"],"exact");
        assert!(attempt["proof"]["numeric"]["wide_lme"]["value"].as_u64().unwrap()>0);
        let unavailable_rows:Vec<_>=successor["results"].as_array().unwrap().iter()
            .filter(|r|r["basis_ref"]["ref_type"]=="load_case" && r["basis_ref"]["ref_id"]=="case-b").collect();
        assert!(!unavailable_rows.is_empty());
        assert!(unavailable_rows.iter().all(|r|r.get("recovery_method").is_none()));
    }
}
