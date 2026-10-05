//! RV89 G6 (scratch only; never in maintained code): independent probes of the
//! registration change and G-C's solve-attempt fact. Mounted as a child of
//! retained_memory in RV89's registered archive copy (`registration.diff` applied).
//! The solve-attempt oracle is a test-only counter RV89 added in its copy of
//! lib.rs, after the one `attempted_linear` statement in `solve_load_case_observed`.
use super::*;
use crate::source_receipt::CapturedInvocation;
use crate::PreviewSolverMode;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const M: u64 = 4_026_531_840;
fn milestone() -> Value {
    serde_json::from_str(include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json")).unwrap()
}
fn with(f: impl FnOnce(&mut Value)) -> Value {
    let mut r = milestone();
    f(&mut r);
    r
}
fn registered() -> bool {
    COMPILED_IDENTITY == Some(REGISTERED_PROFILES[0].identity)
}

// ---- The registered entry binds exactly this build ---------------------------------
#[test]
fn rv89g6_identity_and_bound() {
    println!("RV89_G6_BUILD identity={:?}", COMPILED_IDENTITY);
    println!("RV89_G6_BUILD status={:?}", build_status());
    let p = &REGISTERED_PROFILES[0];
    assert_eq!(REGISTERED_PROFILES.len(), 1);
    assert_eq!(p.threshold_bytes, M);
    if !registered() {
        assert_eq!(build_status(), Err(ProfileStatus::Stale), "any other build is Stale");
        for mode in MODES {
            let (request, capture) = CapturedInvocation::parse(milestone(), mode).unwrap();
            let r = admit(&capture, &request, Entry::Direct).err().expect("refused");
            assert_eq!(r.law().refusal, Some(AdmissionRefusal::Profile(ProfileStatus::Stale)));
            assert_eq!(r.profile, ProfileStatus::Stale);
            let out = crate::run_linear_static_preview_value_with_retained_direct(milestone(), mode).unwrap();
            let plain = crate::run_linear_static_preview_value_with_mode(milestone(), mode).unwrap();
            assert_eq!(serde_json::to_vec(out.envelope()).unwrap(), serde_json::to_vec(&plain).unwrap());
            assert!(out.successor().is_none());
        }
        println!("RV89_G6_STALE ok");
        return;
    }
    assert_eq!(COMPILED_REVIEWED_INPUTS, Some(p.reviewed_inputs));
    assert_eq!(READER_LAYOUTS, p.reader_layouts);
    assert!(LAYOUT_WITNESSES);
    assert_eq!(build_status(), Ok(0));
    assert_eq!(profile::ESTIMATES, 0);
    for mode in MODES {
        let e = cap_priced_maximum(mode).unwrap();
        let required = bound_admits(e, RESERVED_STACK_BYTES as u64, M).unwrap();
        println!("RV89_G6_BOUND {mode:?} E_mov={e} +R={required} frac={:.6} under_0.9M={}", required as f64 / M as f64, 3_623_878_656u64 as i64 - required as i64);
        assert!(required <= 3_623_878_656);
    }
}

// ---- admit: grants the milestone; refuses Headless and every D1 violation ----------
#[test]
fn rv89g6_admit_matrix() {
    if !registered() {
        return;
    }
    let refusals: Vec<(&str, Value)> = vec![
        ("schema 0.3.0", with(|r| r["model"]["schema_version"] = json!("0.3.0"))),
        ("pressure contract", with(|r| r["model"]["pressure_contract"] = json!({}))),
        ("reference configurations", with(|r| r["model"]["reference_configurations"] = Value::Null)),
        ("material expansion law", with(|r| r["model"]["materials"][0]["expansion_laws"] = Value::Null)),
        ("sections", with(|r| r["model"]["sections"] = json!([{"id": "s", "name": "s", "section_type": "pipe", "properties": {}, "provenance": null}]))),
        ("section_ref", with(|r| r["model"]["pipe_segments"][0]["section_ref"] = json!("s"))),
        ("two cases", with(|r| { let c = r["model"]["load_cases"][0].clone(); r["model"]["load_cases"].as_array_mut().unwrap().push(c) })),
        ("combination", with(|r| r["model"]["combinations"] = json!([{"id": "c", "basis": "mechanics", "terms": [{"load_case": "x", "factor": 1.0}]}]))),
        ("component", with(|r| r["model"]["components"] = json!([{"id": "k", "kind": "elbow", "node": "N0"}]))),
        ("pressure regions", with(|r| r["model"]["load_cases"][0]["pressure_regions"] = json!([]))),
        ("equivalent static", with(|r| r["model"]["load_cases"][0]["equivalent_static"] = json!({}))),
        ("modulus basis", with(|r| r["model"]["load_cases"][0]["modulus_basis_ref"] = json!("t"))),
        ("hanger", with(|r| r["model"]["supports"][1]["hanger"] = json!({}))),
        ("family ' anchor'", with(|r| r["model"]["supports"][1]["family"] = json!(" anchor"))),
        ("element target", with(|r| r["model"]["load_cases"][0]["primitive_loads"][0]["target"] = json!({"type": "element", "pipe": "x"}))),
        ("pressure dimension", with(|r| r["model"]["load_cases"][0]["primitive_loads"][0]["dimension"] = json!("pressure"))),
        ("object provenance", with(|r| r["model"]["load_cases"][0]["primitive_loads"][0]["provenance"] = json!(" {"))),
        ("control byte", with(|r| r["model"]["nodes"][0]["provenance"] = json!("a\u{1}"))),
        ("DEL key", with(|r| { r["model"].as_object_mut().unwrap().insert("x\u{7f}".into(), json!(1)); })),
        ("text 129", with(|r| r["model"]["nodes"][0]["provenance"] = json!("p".repeat(129)))),
        ("raw depth 17", with(|r| { let mut v = json!(0); for _ in 0..15 { v = json!([v]); } r["model"]["deep"] = v; })),
    ];
    for mode in MODES {
        let (request, capture) = CapturedInvocation::parse(milestone(), mode).unwrap();
        let (_permit, report) = admit(&capture, &request, Entry::Direct).unwrap_or_else(|r| panic!("milestone {mode:?}: {:?}", r.law().refusal));
        assert_eq!((report.law().refusal, report.law().domain, report.profile), (None, None, ProfileStatus::Registered));
        let raw = milestone();
        let inv = json!({"request": raw, "solver_mode": mode.as_str()});
        let id = String::from("rv89");
        let h = admit(&capture, &request, Entry::Headless(RetainedHeadlessContext::from_borrowed_roots(&raw, &inv, &id))).err().expect("Headless refused");
        assert_eq!(h.law().refusal, Some(AdmissionRefusal::Caller(RetainedCaller::Headless)));
        let mut n = 0;
        for (label, raw) in &refusals {
            let (request, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
            let r = admit(&capture, &request, Entry::Direct).err().unwrap_or_else(|| panic!("{label}: admitted"));
            assert!(r.law().refusal.is_some() && r.law().refusal == r.law().domain, "{label}: {:?}", r.law().refusal);
            let out = crate::run_linear_static_preview_value_with_retained_direct(raw.clone(), mode).unwrap();
            let plain = crate::run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap();
            assert_eq!(serde_json::to_vec(out.envelope()).unwrap(), serde_json::to_vec(&plain).unwrap(), "{label}: refused, so ordinary bytes");
            assert!(out.successor().is_none() && out.retained().is_none(), "{label}");
            n += 1;
        }
        println!("RV89_G6_ADMIT {mode:?} milestone=permit headless=refused d1_violations_refused={n}");
    }
}

// ---- The milestone publishes U1's pinned successor bytes through the facade -------
#[test]
fn rv89g6_milestone_publishes_the_pinned_successor() {
    if !registered() {
        return;
    }
    let pinned = [
        ("sparse_interactive", "ac6986b0680e0df9d88c33a5bf4635372fc3b83cbb9080da44e6d32dbdca59dc", "efc1a39bbe83840df6bd0761c932b8020285d3b8c005ba8fd0b45ba10d667494"),
        ("dense_scrutiny", "6cd1d249e5352aaffbd2b7d7349c74a1d0e0572df35500f66be49c3cad95c9b5", "3e26499f17caff8f5fc0d46406bbe54acf43e8cb16e761784aa5074413b0ac4a"),
    ];
    for (mode, (name, file_sha, receipt_sha)) in MODES.into_iter().zip(pinned) {
        let out = crate::run_linear_static_preview_value_with_retained_direct(milestone(), mode).unwrap();
        let plain = crate::run_linear_static_preview_value_with_mode(milestone(), mode).unwrap();
        assert_eq!(serde_json::to_vec(out.envelope()).unwrap(), serde_json::to_vec(&plain).unwrap(), "the ordinary base is unchanged");
        let succ = out.successor().expect("a successor").clone();
        let text = serde_json::to_string_pretty(&json!({"id": format!("u1_milestone_{name}"), "source": succ, "invocation": {"request": milestone(), "solver_mode": mode.as_str()}})).unwrap();
        let got = format!("{:x}", Sha256::digest(text.as_bytes()));
        let receipt = succ["retained_precision"]["receipt_sha256"].as_str().unwrap().to_string();
        println!("RV89_G6_SUCCESSOR {name} file_sha256={got} receipt_sha256={receipt}");
        assert_eq!((got.as_str(), receipt.as_str()), (file_sha, receipt_sha), "{name}: U1's pinned bytes");
        match out.into_publication() {
            crate::RetainedPublication::Successor(v) => assert_eq!(v, succ),
            _ => panic!("{name}: the publication is the successor"),
        }
    }
}

// ---- G-C's solve-attempt fact against RV89's attempt counter -------------------------
fn extra_examples() -> Vec<(String, Value)> {
    let p = "rv89";
    let mut v: Vec<(&str, Value)> = vec![
        ("milestone", milestone()),
        ("invalid document_kind", with(|r| r["model"]["document_kind"] = json!("invalid-kind"))),
        ("invalid load category", with(|r| r["model"]["load_cases"][0]["primitive_loads"][0]["category"] = json!("nonsense_category"))),
        ("no supports", with(|r| r["model"]["supports"] = json!([]))),
        ("lone spring", with(|r| { let s = r["model"]["supports"].as_array().unwrap().iter().find(|s| s.get("stiffness").is_some()).unwrap().clone(); r["model"]["supports"] = json!([s]); })),
        ("1e-300 springs", with(|r| { for s in r["model"]["supports"].as_array_mut().unwrap() { if s.get("stiffness").is_some() { s["stiffness"]["value"]["value"] = json!(1e-300); } } })),
        ("unknown restraint token", with(|r| r["model"]["supports"][0]["restraints"] = json!(["QQ"]))),
        ("negative spring", with(|r| { for s in r["model"]["supports"].as_array_mut().unwrap() { if s.get("stiffness").is_some() { s["stiffness"]["value"]["value"] = json!(-5.0); } } })),
        ("coincident nodes", with(|r| { let p0 = r["model"]["nodes"][0]["position"].clone(); r["model"]["nodes"][1]["position"] = p0; })),
        ("huge load", with(|r| r["model"]["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"] = json!(1e300))),
        ("tiny OD", with(|r| r["model"]["pipe_segments"][0]["section"]["outside_diameter"]["value"] = json!(1e-300))),
        ("zero modulus", with(|r| r["model"]["materials"][0]["elastic_modulus"]["value"] = json!(0.0))),
        ("missing material", with(|r| r["model"]["pipe_segments"][0]["material"] = json!("no-such-material"))),
        ("load on unknown node", with(|r| r["model"]["load_cases"][0]["primitive_loads"][0]["target"]["node"] = json!("no-such-node"))),
        ("duplicate node ids", with(|r| { let id = r["model"]["nodes"][0]["id"].clone(); r["model"]["nodes"][1]["id"] = id; })),
        ("bad direction", with(|r| r["model"]["load_cases"][0]["primitive_loads"][0]["direction"] = json!("sideways"))),
        ("bad units", with(|r| r["model"]["project"]["units"] = json!({"length": "furlong", "force": "N"}))),
        ("wall > radius", with(|r| r["model"]["pipe_segments"][0]["section"]["wall_thickness"]["value"] = json!(10.0))),
        ("zero loads", with(|r| r["model"]["load_cases"][0]["primitive_loads"] = json!([]))),
        ("unit mismatch", with(|r| r["model"]["load_cases"][0]["primitive_loads"][0]["magnitude"]["unit"] = json!("Pa"))),
    ];
    let w6 = json!({"model": {
        "schema_version": "0.1.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:section-oracle", "units": {"length": "m", "force": "N"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed", "professional_acceptance": "not_provided"},
        "nodes": [{"id": "a", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": p}, {"id": "b", "position": {"x": 1.0, "y": 0.0, "z": 0.0}, "provenance": p}],
        "pipe_segments": [{"id": "pipe", "from": "a", "to": "b", "material": "m", "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
            "section": {"outside_diameter": {"value": 4e-77, "unit": "m"}, "wall_thickness": {"value": 1e-77, "unit": "m"}}, "provenance": p}],
        "materials": [{"id": "m", "elastic_modulus": {"value": 1.0, "unit": "Pa"}, "shear_modulus": {"value": 0.4545, "unit": "Pa"}, "provenance": p}],
        "supports": [{"id": "s", "node": "a", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": p}],
        "load_cases": [{"id": "case", "provenance": p, "primitive_loads": [
            {"id": "l", "category": "concentrated_force", "target": {"type": "node", "node": "b"}, "direction": "global_y", "dimension": "force", "magnitude": {"value": f64::from_bits(0x0031fa182c40c60d), "unit": "N"}, "provenance": p}]}],
        "combinations": []}, "materials": []});
    v.push(("W6 force-scaled", w6));
    let mut out: Vec<(String, Value)> = v.into_iter().map(|(a, b)| (a.to_string(), b)).collect();
    for f in ["dense_scrutiny", "sparse_interactive"] {
        let path = format!("{}/../../fixtures/product_preview/source_blocks/rejected_stress_range/{f}.request.json", env!("CARGO_MANIFEST_DIR"));
        out.push((format!("rejected_stress_range {f}"), serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()));
    }
    if let Ok(dir) = std::env::var("RV89_SWEEP_INPUTS") {
        let mut names: Vec<_> = std::fs::read_dir(&dir).unwrap().map(|e| e.unwrap().file_name().into_string().unwrap()).collect();
        names.sort();
        for n in names {
            let raw: Value = serde_json::from_str(&std::fs::read_to_string(format!("{dir}/{n}")).unwrap()).unwrap();
            out.push((format!("sweep {n}"), raw));
        }
    }
    out
}
#[test]
fn rv89g6_solve_attempt_fact_against_the_oracle() {
    let mut rows = Vec::new();
    let (mut checked, mut mismatches, mut in_d1) = (0, 0, 0);
    for (label, raw) in extra_examples() {
        for mode in MODES {
            let Ok((request, capture)) = CapturedInvocation::parse(raw.clone(), mode) else { continue };
            let domain = assess(&capture, &request, Entry::Direct).law().domain;
            let mut observer = crate::retained_product::ProductCapture::prepared_probe();
            crate::RV89_ATTEMPTS.with(|c| c.set(0));
            let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
            let attempts = crate::RV89_ATTEMPTS.with(|c| c.get());
            let fact = ordinary_solve_attempted(&observer);
            let agree = fact == (attempts > 0);
            checked += 1;
            if !agree && (domain.is_none() || observer.ordinary.len() <= 1) {
                mismatches += 1;
            }
            let blocking = ordinary.diagnostics.iter().find(|d| d.severity == "blocking").map(|d| d.code.clone());
            // Under registration, the Direct entry for an in-domain request.
            let mut direct_note = String::from("-");
            if registered() && domain.is_none() {
                in_d1 += 1;
                let plain = serde_json::to_vec(&crate::run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap();
                let out = crate::run_linear_static_preview_value_with_retained_direct(raw.clone(), mode).unwrap();
                let same = serde_json::to_vec(out.envelope()).unwrap() == plain;
                let notice = out.envelope().diagnostics.iter().any(|d| d.code == "RETAINED_PRECISION_UNAVAILABLE");
                let w1 = match out.retained() {
                    None => "none".to_string(),
                    Some(Ok(_)) => "successor".to_string(),
                    Some(Err(crate::W1Fallback::CompleteGate(r))) => format!("CompleteGate({:?})", r.fact),
                    Some(Err(f)) => format!("{f:?}").chars().take(60).collect(),
                };
                if !fact {
                    assert!(same && !notice && out.successor().is_none(), "{label} {mode:?}: not attempted must give exact bytes ({w1})");
                    assert!(w1.starts_with("CompleteGate") || w1.starts_with("LateGate"), "{label} {mode:?}: {w1}");
                } else {
                    assert!(!w1.starts_with("CompleteGate(OrdinarySolveNotAttempted"), "{label} {mode:?}: an attempted solve declined");
                }
                direct_note = format!("w1={w1} exact_bytes={same} notice={notice}");
            }
            rows.push(format!("RV89_G6_ATTEMPT {label} | {mode:?} | domain={:?} | attempts={attempts} fact={fact} agree={agree} seeds={} | status={} blocking={blocking:?} | {direct_note}",
                domain.map(|d| format!("{d:?}")).unwrap_or_else(|| "D1".into()), observer.ordinary.len(), ordinary.status.mechanics));
        }
    }
    for r in &rows {
        println!("{r}");
    }
    println!("RV89_G6_ATTEMPT_SUMMARY checked={checked} mismatches={mismatches} in_d1={in_d1}");
    assert_eq!(mismatches, 0);
}

#[test]
fn rv89g6_seed_variants() {
    for (label, raw) in extra_examples().into_iter().filter(|(l, _)| !l.starts_with("sweep ")) {
        for mode in MODES {
            let Ok((request, capture)) = CapturedInvocation::parse(raw.clone(), mode) else { continue };
            let mut observer = crate::retained_product::ProductCapture::prepared_probe();
            let _ = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
            let v: Vec<String> = observer.ordinary.iter().map(|s| format!("{:?}", s.initial).chars().take(40).collect()).collect();
            println!("RV89_G6_SEED {label} {mode:?} {v:?}");
        }
    }
}

#[test]
fn rv89g6_deferred_formation_reachability() {
    let variants: Vec<(&str, Value)> = vec![
        ("length 1e-60", with(|r| { let mut p = r["model"]["nodes"][0]["position"].clone(); p["x"] = json!(p["x"].as_f64().unwrap() + 1e-60); r["model"]["nodes"][1]["position"] = p; })),
        ("length 1e-100", with(|r| { let mut p = r["model"]["nodes"][0]["position"].clone(); p["x"] = json!(p["x"].as_f64().unwrap() + 1e-100); r["model"]["nodes"][1]["position"] = p; })),
        ("length 1e-30 E 1e15", with(|r| { let mut p = r["model"]["nodes"][0]["position"].clone(); p["x"] = json!(p["x"].as_f64().unwrap() + 1e-30); r["model"]["nodes"][1]["position"] = p; for m in r["model"]["materials"].as_array_mut().unwrap() { m["elastic_modulus"]["value"] = json!(1e15); } })),
        ("length 1e100", with(|r| { let mut p = r["model"]["nodes"][0]["position"].clone(); p["x"] = json!(p["x"].as_f64().unwrap() + 1e100); r["model"]["nodes"][1]["position"] = p; })),
        ("OD 1e-40", with(|r| { r["model"]["pipe_segments"][0]["section"]["outside_diameter"]["value"] = json!(1e-40); r["model"]["pipe_segments"][0]["section"]["wall_thickness"]["value"] = json!(1e-41); })),
        ("OD 1e-100", with(|r| { r["model"]["pipe_segments"][0]["section"]["outside_diameter"]["value"] = json!(1e-100); r["model"]["pipe_segments"][0]["section"]["wall_thickness"]["value"] = json!(1e-101); })),
        ("E 1e-200", with(|r| { for m in r["model"]["materials"].as_array_mut().unwrap() { m["elastic_modulus"]["value"] = json!(1e-200); if m.get("shear_modulus").is_some() { m["shear_modulus"]["value"] = json!(1e-200); } } })),
        ("E 1e300", with(|r| { for m in r["model"]["materials"].as_array_mut().unwrap() { m["elastic_modulus"]["value"] = json!(1e300); } })),
        ("E 1e305 G 1e305", with(|r| { for m in r["model"]["materials"].as_array_mut().unwrap() { m["elastic_modulus"]["value"] = json!(1e305); if m.get("shear_modulus").is_some() { m["shear_modulus"]["value"] = json!(1e305); } } })),
        ("OD 1e150 E 1e300", with(|r| { r["model"]["pipe_segments"][0]["section"]["outside_diameter"]["value"] = json!(1e150); r["model"]["pipe_segments"][0]["section"]["wall_thickness"]["value"] = json!(1e149); for m in r["model"]["materials"].as_array_mut().unwrap() { m["elastic_modulus"]["value"] = json!(1e300); } })),
        ("spring 1e300", with(|r| { for s in r["model"]["supports"].as_array_mut().unwrap() { if s.get("stiffness").is_some() { s["stiffness"]["value"]["value"] = json!(1e300); } } })),
        ("short member E 1e300", with(|r| { let mut p = r["model"]["nodes"][0]["position"].clone(); p["x"] = json!(p["x"].as_f64().unwrap() + 1e-200); r["model"]["nodes"][1]["position"] = p; for m in r["model"]["materials"].as_array_mut().unwrap() { m["elastic_modulus"]["value"] = json!(1e300); } })),
    ];
    for (label, raw) in variants {
        for mode in MODES {
            let Ok((request, capture)) = CapturedInvocation::parse(raw.clone(), mode) else { println!("RV89_G6_DEFER {label} {mode:?} parse refused"); continue };
            let domain = assess(&capture, &request, Entry::Direct).law().domain;
            let mut observer = crate::retained_product::ProductCapture::prepared_probe();
            crate::RV89_ATTEMPTS.with(|c| c.set(0));
            let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
            let attempts = crate::RV89_ATTEMPTS.with(|c| c.get());
            let v: Vec<String> = observer.ordinary.iter().map(|s| format!("{:?}", s.initial).chars().take(48).collect()).collect();
            let blocking = ordinary.diagnostics.iter().find(|d| d.severity == "blocking").map(|d| d.code.clone());
            println!("RV89_G6_DEFER {label} {mode:?} domain={domain:?} attempts={attempts} fact={} seeds={v:?} status={} blocking={blocking:?}", ordinary_solve_attempted(&observer), ordinary.status.mechanics);
        }
    }
}

/// K2a's product-reach shapes (tests/k2a_formation_range_runtime.rs `request`), rebuilt here.
fn k2a(id: &str, length: f64, od: f64, wall: f64, e: f64, g: f64, free: &str, spring: Option<f64>, load: f64) -> Value {
    let all = ["UX", "UY", "UZ", "RX", "RY", "RZ"];
    let anchored: Vec<&str> = all.iter().copied().filter(|d| *d != free).collect();
    let p = "rv89_k2a_shape";
    let mut supports = vec![json!({"id": "rigid:N0", "node": "N0", "restraints": all, "family": "anchor", "provenance": p}),
        json!({"id": "rigid:N1", "node": "N1", "restraints": anchored, "family": "anchor", "provenance": p})];
    let rot = free.starts_with('R');
    if let Some(k) = spring {
        supports.push(json!({"id": "spring:N1:0", "node": "N1", "family": "spring", "restraints": [free], "stiffness": {"dof": free, "value": {"value": k, "unit": if rot { "N*m/rad" } else { "N/m" }}}, "provenance": p}));
    }
    let load = if rot {
        json!({"id": "load:0", "category": "concentrated_moment", "target": {"type": "node", "node": "N1"}, "direction": free, "magnitude": {"value": load, "unit": "N*m"}, "dimension": "moment", "provenance": p})
    } else {
        json!({"id": "load:0", "category": "concentrated_force", "target": {"type": "node", "node": "N1"}, "direction": format!("global_{}", free[1..].to_lowercase()), "magnitude": {"value": load, "unit": "N"}, "dimension": "force", "provenance": p})
    };
    json!({"model": {"schema_version": "0.1.0", "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "project": {"id": format!("invented:rv89:{id}"), "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "temperature": "degC", "stress": "Pa"}},
        "nodes": [{"id": "N0", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": p}, {"id": "N1", "position": {"x": length, "y": 0.0, "z": 0.0}, "provenance": p}],
        "pipe_segments": [{"id": "M1", "from": "N0", "to": "N1", "material": "mat", "y_reference": {"x": 0, "y": 1, "z": 0},
            "section": {"outside_diameter": {"value": od, "unit": "m"}, "wall_thickness": {"value": wall, "unit": "m"}}, "provenance": p}],
        "materials": [{"id": "mat", "elastic_modulus": {"value": e, "unit": "Pa"}, "shear_modulus": {"value": g, "unit": "Pa"}, "provenance": p}],
        "supports": supports, "load_cases": [{"id": "case", "primitive_loads": [load], "provenance": p}], "combinations": []}, "materials": []})
}
#[test]
fn rv89g6_k2a_shapes() {
    let shapes = vec![
        ("spring carried", k2a("a", 2.0, 1.0e-6, 1.0e-7, 2.0e11, 1.0e-300, "RX", Some(1.0), 1.0)),
        ("partial underflow", k2a("b", 9.5367431640625e-07, 3.0e-8, 3.0e-9, 1.3e-292, 1.0e-200, "UY", None, 1.0e-307)),
        ("exact zero", k2a("c", 1.8189894035458565e-12, 1.0e-11, 1.0e-12, 6.4e-280, 1.0e-100, "UY", Some(3.7e-289), 9.25e-290)),
        ("least subnormal", k2a("d", 1.8189894035458565e-12, 1.0e-11, 1.0e-12, 9.6e-280, 1.0e-100, "UY", None, 2.05e-289)),
    ];
    for (label, raw) in shapes {
        for mode in MODES {
            let Ok((request, capture)) = CapturedInvocation::parse(raw.clone(), mode) else { println!("RV89_G6_K2A {label} {mode:?} parse refused"); continue };
            let domain = assess(&capture, &request, Entry::Direct).law().domain;
            let mut observer = crate::retained_product::ProductCapture::prepared_probe();
            crate::RV89_ATTEMPTS.with(|c| c.set(0));
            let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
            let attempts = crate::RV89_ATTEMPTS.with(|c| c.get());
            let fact = ordinary_solve_attempted(&observer);
            let v: Vec<String> = observer.ordinary.iter().map(|s| format!("{:?}", s.initial).chars().take(36).collect()).collect();
            let mut note = String::new();
            if registered() && domain.is_none() {
                let plain = serde_json::to_vec(&crate::run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap();
                let out = crate::run_linear_static_preview_value_with_retained_direct(raw.clone(), mode).unwrap();
                let w1 = match out.retained() { None => "none".to_string(), Some(Ok(_)) => "successor".into(), Some(Err(f)) => format!("{f:?}").chars().take(50).collect() };
                note = format!("w1={w1} exact={}", serde_json::to_vec(out.envelope()).unwrap() == plain);
            }
            println!("RV89_G6_K2A {label} {mode:?} domain={domain:?} attempts={attempts} fact={fact} seeds={v:?} status={} {note}", ordinary.status.mechanics);
            assert_eq!(fact, attempts > 0, "{label}");
        }
    }
}

// ---- G6 repair (RV89 S-3): admission's bound adds R, and the law record keeps it ----
#[test]
fn rv89g6r_required_is_maximum_plus_r() {
    const M: u64 = 4_026_531_840;
    let r = RESERVED_STACK_BYTES as u64;
    // The named bound at the edges, my own expectations.
    for (e, expect) in [(M - r - 1, Ok(M - 1)), (M - r, Ok(M)), (M - r + 1, Err(BoundRefusal::Exceeds { required: M + 1, threshold: M }))] {
        assert_eq!(admission_bound(Ok(e), M), expect);
        assert_eq!(bound_required(&admission_bound(Ok(e), M)), Some(e + r));
    }
    assert_eq!(admission_bound(Err(BoundRefusal::Unpriced), M), Err(BoundRefusal::Unpriced));
    assert_eq!(admission_bound(Ok(u64::MAX - r + 1), M), Err(BoundRefusal::Overflow));
    assert_eq!(bound_required(&Err(BoundRefusal::Overflow)), None);
    // R is the constant, not the witness override.
    RESERVED_STACK_OVERRIDE.with(|c| c.set(Some(1 << 20)));
    assert_eq!(admission_bound(Ok(M - r), M), Ok(M));
    RESERVED_STACK_OVERRIDE.with(|c| c.set(None));
    for mode in MODES {
        let (request, capture) = CapturedInvocation::parse(milestone(), mode).unwrap();
        match admit(&capture, &request, Entry::Direct) {
            Ok((_p, report)) => {
                assert!(registered());
                let e = cap_priced_maximum(mode).unwrap();
                assert_eq!(report.law().required, Some(e + r));
                println!("RV89_G6R_REQUIRED {mode:?} required={} = E {e} + R {r}; frac={:.6}", e + r, (e + r) as f64 / M as f64);
            }
            Err(report) => {
                assert!(!registered());
                assert_eq!(report.law().required, None, "refused at D1.1: the bound is never evaluated");
            }
        }
        // A refused request never evaluates the bound.
        let (request, capture) = CapturedInvocation::parse(with(|r| r["model"]["schema_version"] = json!("0.3.0")), mode).unwrap();
        assert_eq!(admit(&capture, &request, Entry::Direct).err().unwrap().law().required, None);
    }
}
