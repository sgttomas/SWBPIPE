//! RV97 round-1 probe (review only; never committed). A child of `retained_facade_tests`,
//! so it can call the candidate's private input builders for comparison. My own input
//! constructions, my own counters (`crate::zz_rv97_counters`, independent of grant 2's
//! tally), my own byte oracle, and a stage-by-stage private-driver replay that reads the
//! kernel's native outcome directly (no production print).
use super::*;
use crate::zz_rv97_counters as ctr;
use open_pipe_stress_result_export::retained_precision::{validate, AccuracyClass};

const PROV: &str = "invented_t3_p1_detection_input_no_library_data";
const N1_TEXT: &str = "Retained-precision recovery is unavailable for this load case. Its published rows keep their ordinary values, standing and diagnostics.";

fn emit(tag: &str, v: Value) {
    println!("RV97 {tag} {}", serde_json::to_string(&v).unwrap());
}
fn save(name: &str, bytes: &[u8]) {
    if let Ok(d) = std::env::var("RV97_OUT") {
        std::fs::write(std::path::Path::new(&d).join(name), bytes).unwrap();
    }
}

// ---- inputs, built my way --------------------------------------------------------------
fn m() -> Value {
    serde_json::from_str(MILESTONE).unwrap()
}
fn i_first_load_only() -> Value {
    let mut r = m();
    r["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().truncate(1);
    r
}
fn i_tiny_spring() -> Value {
    let mut r = m();
    let s = r["model"]["supports"].as_array_mut().unwrap().iter_mut().find(|s| s["id"] == "spring:N0:0").unwrap();
    s["stiffness"]["value"]["value"] = json!(1e-300);
    r
}
/// W6 (retained_memory_witness_tests.rs `w6_input`), retyped.
fn i_w6() -> Value {
    let p = "invented_t3_g5_witness_input_no_library_data";
    let tip = f64::from_bits(0x0031fa182c40c60d);
    json!({"model": {
        "schema_version": "0.1.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:section-oracle", "units": {"length": "m", "force": "N"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed", "professional_acceptance": "not_provided"},
        "nodes": [{"id": "node:section-a", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": p},
                  {"id": "node:section-b", "position": {"x": 1.0, "y": 0.0, "z": 0.0}, "provenance": p}],
        "pipe_segments": [{"id": "pipe:source-section", "from": "node:section-a", "to": "node:section-b", "material": "material:section",
            "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
            "section": {"outside_diameter": {"value": 4e-77, "unit": "m"}, "wall_thickness": {"value": 1e-77, "unit": "m"}}, "provenance": p}],
        "materials": [{"id": "material:section", "elastic_modulus": {"value": 1.0, "unit": "Pa"}, "shear_modulus": {"value": 0.4545, "unit": "Pa"}, "provenance": p}],
        "supports": [{"id": "support:section-a", "node": "node:section-a", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": p}],
        "load_cases": [{"id": "case:source-section", "provenance": p, "primitive_loads": [
            {"id": "load:tip-y", "category": "concentrated_force", "target": {"type": "node", "node": "node:section-b"}, "direction": "global_y",
                "dimension": "force", "magnitude": {"value": tip, "unit": "N"}, "provenance": p},
            {"id": "load:tip-torque", "category": "concentrated_moment", "target": {"type": "node", "node": "node:section-b"}, "direction": "rotation_x",
                "dimension": "moment", "magnitude": {"value": tip, "unit": "N*m"}, "provenance": p}]}],
        "combinations": []}, "materials": []})
}
/// The milestone body plus W6's body shifted by +5 m in x; case B carries W6's loads.
fn i_two_body(case_b: bool) -> Value {
    let mut r = m();
    let w6 = i_w6();
    let wm = &w6["model"];
    let model = &mut r["model"];
    for n in wm["nodes"].as_array().unwrap() {
        let mut n = n.clone();
        let x = n["position"]["x"].as_f64().unwrap();
        n["position"]["x"] = json!(x + 5.0);
        model["nodes"].as_array_mut().unwrap().push(n);
    }
    for key in ["pipe_segments", "materials", "supports"] {
        for item in wm[key].as_array().unwrap() {
            model[key].as_array_mut().unwrap().push(item.clone());
        }
    }
    if case_b {
        model["load_cases"][0]["primitive_loads"] = wm["load_cases"][0]["primitive_loads"].clone();
    }
    r
}
/// One body (W6's model), the tip force alone along `direction`.
fn i_one_body(direction: &str) -> Value {
    let mut r = i_w6();
    let mut force = r["model"]["load_cases"][0]["primitive_loads"][0].clone();
    force["direction"] = json!(direction);
    r["model"]["load_cases"][0]["primitive_loads"] = json!([force]);
    r
}
fn i_l0() -> Value {
    let mut r = m();
    r["model"]["nodes"].as_array_mut().unwrap().push(json!({"id": "N2", "position": {"x": 3.0, "y": 0.0, "z": 0.0}, "provenance": PROV}));
    r["model"]["supports"].as_array_mut().unwrap().push(json!({"id": "rigid:N2", "node": "N2", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV}));
    r
}

// ---- my byte oracle --------------------------------------------------------------------
/// The published bytes are the plain bytes with exactly one insertion, at the end of the
/// diagnostics array, consisting of "," and one JSON object equal (as a value) to N1's notice
/// for `case`; parsed, the diagnostics are the plain ones plus that notice, last.
fn oracle(plain: &[u8], published: &[u8], case: &str) -> Result<(), String> {
    if published.len() <= plain.len() {
        return Err(format!("not longer ({} vs {})", published.len(), plain.len()));
    }
    let p = plain.iter().zip(published.iter()).take_while(|(a, b)| a == b).count();
    let extra = published.len() - plain.len();
    if published[p + extra..] != plain[p..] {
        return Err(format!("more than one insertion (common prefix {p})"));
    }
    let ins = std::str::from_utf8(&published[p..p + extra]).map_err(|e| e.to_string())?;
    let obj = ins.strip_prefix(',').ok_or("the insertion does not start with a comma")?;
    let notice: Value = serde_json::from_str(obj).map_err(|e| format!("the insertion is not one JSON value: {e}"))?;
    let expected = json!({"id": format!("diagnostic:retained-precision:{case}:unavailable"), "code": "RETAINED_PRECISION_UNAVAILABLE",
        "severity": "info", "message": N1_TEXT, "source": "core/product_physics", "affected_refs": [case]});
    if notice != expected {
        return Err(format!("the insertion is not N1's notice: {notice}"));
    }
    if !plain[p..].starts_with(b"],\"professional_boundary\"") {
        return Err("the insertion is not at the end of the diagnostics array".into());
    }
    let (pv, qv): (Value, Value) = (serde_json::from_slice(published).unwrap(), serde_json::from_slice(plain).unwrap());
    let (pd, qd) = (pv["diagnostics"].as_array().unwrap(), qv["diagnostics"].as_array().unwrap());
    if pd.len() != qd.len() + 1 || pd[..qd.len()] != qd[..] || pd.last() != Some(&expected) {
        return Err("the diagnostics are not the plain diagnostics plus the notice, last".into());
    }
    Ok(())
}
fn n1_count(bytes: &[u8]) -> usize {
    let v: Value = serde_json::from_slice(bytes).unwrap();
    v["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == "RETAINED_PRECISION_UNAVAILABLE").count()
}
/// Every test seam on this thread is at rest.
fn at_rest() -> bool {
    hooks::armed_names().is_empty()
        && hooks::armed() == hooks::Armed::default()
        && crate::DENSE_SCRUTINY_CEILING_OVERRIDE.with(|c| c.get()).is_none()
        && crate::retained_memory::RESERVED_STACK_OVERRIDE.with(|c| c.get()).is_none()
}
fn reader(source: &Value, invocation: &Value) -> Value {
    match validate(source, Some(invocation)) {
        Ok(v) => {
            let mut counts = std::collections::BTreeMap::<&str, usize>::new();
            for c in &v.classifications {
                let k = match c.class {
                    AccuracyClass::RelativeVerified => "relative",
                    AccuracyClass::AbsoluteVerified { .. } => "absolute",
                    AccuracyClass::InputDerived => "input",
                    AccuracyClass::NonQuantity => "non_quantity",
                    AccuracyClass::NotCovered => "not_covered",
                };
                *counts.entry(k).or_default() += 1;
            }
            json!({"ok": true, "bound": v.invocation_bound, "eligible": v.numerical_eligible, "publication_sha256": v.publication_sha256,
                "rows": v.classifications.len(), "classes": counts})
        }
        Err(e) => json!({"ok": false, "gate": e.gate, "code": e.code, "detail": e.detail}),
    }
}
fn document(id: String, source: &Value, raw: &Value, mode: PreviewSolverMode) -> String {
    serde_json::to_string_pretty(&json!({"id": id, "source": source, "invocation": {"request": raw, "solver_mode": mode.as_str()}})).unwrap()
}

/// The Direct entry, with my counters and grant 2's tally side by side.
fn run_direct(label: &str, raw: &Value, mode: PreviewSolverMode) -> Option<Value> {
    let case = raw["model"]["load_cases"][0]["id"].as_str().unwrap().to_owned();
    let plain_bytes = plain(mode, raw);
    let before = at_rest();
    ctr::reset();
    let r = raw.clone();
    let (output, tally) = hooks::counted(move || run_linear_static_preview_value_with_retained_direct(r, mode).unwrap());
    let (runs, gcs, native) = ctr::read();
    let after = at_rest();
    let adm = output.admission().expect("G-A ran");
    let law = adm.law();
    let admission = json!({"profile": format!("{:?}", adm.profile), "allowance": format!("{:?}", adm.allowance), "refusal": format!("{:?}", law.refusal),
            "domain": format!("{:?}", law.domain), "required": law.required, "census_complete": adm.census_complete(), "typed": format!("{:?}", adm.typed)});
    let retained = match output.retained() {
        None => "none".to_string(),
        Some(Ok(_)) => "successor".to_string(),
        Some(Err(f)) => format!("{f:?}"),
    };
    let successor = output.successor().cloned();
    let b_prime = serde_json::to_vec(output.envelope()).unwrap() == plain_bytes;
    let plain_json: Value = serde_json::from_slice(&plain_bytes).unwrap();
    let published_bytes = published(output);
    let mut rec = json!({"label": label, "mode": mode.as_str(), "input_sha256": sha(&serde_json::to_vec(raw).unwrap()),
        "plain": {"sha256": sha(&plain_bytes), "len": plain_bytes.len(), "results": plain_json["results"].as_array().map(|a| a.len()), "status": plain_json["status"].clone(),
            "diagnostic_codes": plain_json["diagnostics"].as_array().unwrap().iter().map(|d| d["code"].clone()).collect::<Vec<_>>()},
        "admission": admission,
        "retained": retained, "successor": successor.is_some(),
        "tally": [tally.runs, tally.complete_gates], "rv97_counters": [runs, gcs], "native_direct": native,
        "at_rest_before": before, "at_rest_after": after, "b_prime": b_prime,
        "published": {"sha256": sha(&published_bytes), "len": published_bytes.len(), "n1": n1_count(&published_bytes),
            "equals_plain": published_bytes == plain_bytes,
            "oracle": match oracle(&plain_bytes, &published_bytes, &case) { Ok(()) => "plain + one N1 notice".to_string(), Err(e) => e }}});
    if let Some(s) = &successor {
        let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
        rec["successor_detail"] = json!({"receipt_sha256": s["retained_precision"]["receipt_sha256"], "published_is_successor": published_bytes == serde_json::to_vec(s).unwrap(),
            "rust_reader": reader(s, &invocation)});
    }
    emit("DIRECT", rec);
    successor
}

/// The private driver, stage by stage (retained_w1's order without the permit's gates),
/// reading the kernel outcome directly.
fn run_stages(label: &str, raw: &Value, mode: PreviewSolverMode, direct: Option<&Value>) -> Option<(Value, Value)> {
    let plain_bytes = plain(mode, raw);
    let (capture, observer, ordinary) = observed(mode, raw);
    let mut rec = json!({"label": label, "mode": mode.as_str(), "observed_is_plain": serde_json::to_vec(&ordinary).unwrap() == plain_bytes,
        "coexistence": ordinary.source_block_recovery.is_some()});
    let mut dump = None;
    match observer.prepare_case(ordinary) {
        Err(f) => {
            rec["stage"] = json!("Preparation");
            rec["detail"] = json!(format!("capture.error={:?} preparation_error={:?}", f.capture.error, f.preparation_error));
        }
        Ok(mut prepared) => {
            let native = prepared.solve_native().map(|_| ()).map_err(|e| format!("{e:?}"));
            ctr::reset();
            if let Some((_, case)) = prepared.capture().native.as_ref() {
                ctr::native(&case.outcome);
            }
            rec["native"] = json!(ctr::read().2);
            if let Err(e) = native {
                rec["stage"] = json!("Native");
                rec["detail"] = json!(e);
            } else {
                match prepared.freeze_candidate() {
                    Err(refusal) => {
                        rec["stage"] = json!("Candidate");
                        rec["detail"] = json!(format!("{:?}", refusal.error).chars().take(1200).collect::<String>());
                    }
                    Ok(frozen) => match frozen.staged_envelope() {
                        Err(fault) => {
                            rec["stage"] = json!("Staging");
                            rec["detail"] = json!(format!("{fault:?}"));
                        }
                        Ok(staged) => match crate::retained_wire::serialize_frozen(&frozen, &staged, &capture) {
                            Err(f) => {
                                rec["stage"] = json!("Serializer");
                                rec["detail"] = json!(format!("{f:?}"));
                            }
                            Ok(successor) => {
                                let invocation = json!({"request": capture.borrowed_raw(), "solver_mode": capture.mode().as_str()});
                                let verdict = reader(&successor, &invocation);
                                rec["stage"] = json!(if verdict["ok"] == true { "Successor".to_string() } else { format!("Precommit {} {}", verdict["gate"], verdict["code"]) });
                                rec["reader"] = verdict;
                                rec["equals_direct_successor"] = json!(direct.map(|d| d == &successor));
                                rec["receipt_sha256"] = successor["retained_precision"]["receipt_sha256"].clone();
                                dump = Some((successor, invocation));
                            }
                        },
                    },
                }
            }
        }
    }
    emit("STAGES", rec);
    dump
}

#[test]
fn zz_rv97_inputs_equal_the_candidates_builders() {
    let pairs = [
        ("first_load_only", i_first_load_only(), u8_first_load_only()),
        ("tiny_spring", i_tiny_spring(), u8_tiny_spring()),
        ("two_body_case_a", i_two_body(false), u8_two_body_case_a()),
        ("two_body_case_b", i_two_body(true), u8_two_body_case_b()),
        ("l0", i_l0(), u8_l0_isolated_node()),
    ];
    for (label, mine, theirs) in pairs {
        emit("INPUT", json!({"label": label, "equal": mine == theirs, "mine_sha256": sha(&serde_json::to_vec(&mine).unwrap()),
            "candidate_sha256": sha(&serde_json::to_vec(&theirs).unwrap())}));
    }
}

#[test]
fn zz_rv97_probe() {
    assert!(registered(), "RV97's probe runs in the registered build");
    let inputs: Vec<(&str, Value)> = vec![
        ("milestone", m()),
        ("first_load_only", i_first_load_only()),
        ("tiny_spring", i_tiny_spring()),
        ("w6", i_w6()),
        ("l0", i_l0()),
        ("two_body_a", i_two_body(false)),
        ("two_body_b", i_two_body(true)),
        ("one_body_a_axial", i_one_body("global_x")),
        ("one_body_b_transverse", i_one_body("global_y")),
    ];
    for (label, raw) in &inputs {
        for mode in MODES {
            let successor = run_direct(label, raw, mode);
            let dump = run_stages(label, raw, mode, successor.as_ref());
            let name = mode.as_str();
            if let Some(s) = &successor {
                match *label {
                    "milestone" => save(&format!("milestone_document_{name}.json"), document(format!("u1_milestone_{name}"), s, raw, mode).as_bytes()),
                    "l0" => save(&format!("l0_document_{name}.json"), document(format!("u8_l0_isolated_node_{name}"), s, raw, mode).as_bytes()),
                    _ => {}
                }
            }
            if let Some((source, invocation)) = dump {
                if matches!(*label, "l0" | "two_body_a" | "milestone") {
                    save(&format!("stages_{label}_{name}.json"), serde_json::to_string_pretty(&json!({"source": source, "invocation": invocation})).unwrap().as_bytes());
                }
            }
        }
    }
}
