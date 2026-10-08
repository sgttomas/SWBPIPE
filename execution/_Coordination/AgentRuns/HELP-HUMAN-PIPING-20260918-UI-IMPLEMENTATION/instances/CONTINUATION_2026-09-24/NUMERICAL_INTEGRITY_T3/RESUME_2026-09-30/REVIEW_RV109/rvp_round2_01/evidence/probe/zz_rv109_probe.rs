//! RV109 (RV-P, B1 round 1): a reviewer's probe, in the reviewer's own archive copies only.
//! Declared as a child of `witness_tests` so it uses the committed witness inputs and driver.
//! Prints one `RV109 {json}` line per (input, mode); writes nothing else. Identical source in
//! the base and candidate copies, apart from `zz_rv109_rev.rs` (the seam reader).
#![allow(dead_code, unused_imports)]
use super::*;
use crate::retained_tests_hooks as hooks;
use crate::{PreviewSolverMode, RetainedPublication};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

#[path = "zz_rv109_rev.rs"]
mod rev;

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
const P: &str = "invented_t3_g5_witness_input_no_library_data";

// ---- inputs: committed builders where reachable, otherwise verbatim copies of the committed ones ----
fn two_body_a() -> Value {
    let mut raw = milestone();
    let model = &mut raw["model"];
    model["nodes"].as_array_mut().unwrap().extend([
        json!({"id": "node:section-a", "position": {"x": 5.0, "y": 0.0, "z": 0.0}, "provenance": P}),
        json!({"id": "node:section-b", "position": {"x": 6.0, "y": 0.0, "z": 0.0}, "provenance": P}),
    ]);
    model["pipe_segments"].as_array_mut().unwrap().push(json!({"id": "pipe:source-section", "from": "node:section-a", "to": "node:section-b",
        "material": "material:section", "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
        "section": {"outside_diameter": {"value": 4e-77, "unit": "m"}, "wall_thickness": {"value": 1e-77, "unit": "m"}}, "provenance": P}));
    model["materials"].as_array_mut().unwrap().push(json!({"id": "material:section", "elastic_modulus": {"value": 1.0, "unit": "Pa"},
        "shear_modulus": {"value": 0.4545, "unit": "Pa"}, "provenance": P}));
    model["supports"].as_array_mut().unwrap().push(json!({"id": "support:section-a", "node": "node:section-a", "family": "anchor",
        "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": P}));
    raw
}
fn tip_loads() -> Value {
    let tip = f64::from_bits(0x0031fa182c40c60d);
    json!([
        {"id": "load:tip-y", "category": "concentrated_force", "target": {"type": "node", "node": "node:section-b"}, "direction": "global_y",
            "dimension": "force", "magnitude": {"value": tip, "unit": "N"}, "provenance": P},
        {"id": "load:tip-torque", "category": "concentrated_moment", "target": {"type": "node", "node": "node:section-b"}, "direction": "rotation_x",
            "dimension": "moment", "magnitude": {"value": tip, "unit": "N*m"}, "provenance": P}])
}
fn two_body_b() -> Value {
    let mut raw = two_body_a();
    raw["model"]["load_cases"][0]["primitive_loads"] = tip_loads();
    raw
}
/// W-C2's case C, built here from DESIGN_v2 §1.4's words: case A's loads, then case B's, as one case.
fn case_c() -> Value {
    let mut raw = two_body_a();
    let tips = tip_loads();
    raw["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().extend(tips.as_array().unwrap().iter().cloned());
    raw
}
fn first_load_only() -> Value {
    let mut raw = milestone();
    let first = raw["model"]["load_cases"][0]["primitive_loads"][0].clone();
    raw["model"]["load_cases"][0]["primitive_loads"] = json!([first]);
    raw
}
fn tiny_spring() -> Value {
    let mut raw = milestone();
    raw["model"]["supports"][1]["stiffness"]["value"]["value"] = json!(1e-300);
    raw
}
fn l0() -> Value {
    let mut raw = milestone();
    let p = "invented_t3_p1_detection_input_no_library_data";
    raw["model"]["nodes"].as_array_mut().unwrap().push(json!({"id": "N2", "position": {"x": 3.0, "y": 0.0, "z": 0.0}, "provenance": p}));
    raw["model"]["supports"].as_array_mut().unwrap().push(json!({"id": "rigid:N2", "node": "N2", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": p}));
    raw
}
fn w2() -> Value {
    let mut raw = super::super::law_tests::cap_maximal();
    escape_every_provenance(&mut raw);
    raw["model"]["unknown_depth_witness"] = depth_16_value();
    raw
}
fn w2_deep() -> Value {
    let mut raw = milestone();
    escape_every_provenance(&mut raw);
    raw["model"]["deep_input_witness"] = depth_16_value();
    raw
}
fn w2b() -> Value {
    let mut raw = super::super::law_tests::cap_maximal();
    let supports = raw["model"]["supports"].as_array_mut().unwrap();
    for (i, support) in supports.iter_mut().enumerate() {
        if i == 0 {
            support.as_object_mut().unwrap().remove("stiffness");
        } else {
            support["family"] = json!("spring");
            support["restraints"] = json!(["UY"]);
        }
    }
    raw
}

fn inputs() -> Vec<(String, Value)> {
    let mut v: Vec<(String, Value)> = vec![
        ("milestone".into(), milestone()),
        ("l0_isolated_node".into(), l0()),
        ("two_body_a".into(), two_body_a()),
        ("two_body_b".into(), two_body_b()),
        ("case_c".into(), case_c()),
        ("first_load_only".into(), first_load_only()),
        ("tiny_spring".into(), tiny_spring()),
        ("w2_cap_maximal".into(), w2()),
        ("w2_deep".into(), w2_deep()),
        ("w2b_input".into(), w2b()),
        ("w6_phys_r4".into(), w6_input()),
    ];
    for (label, raw) in super::super::law_tests::attempted_examples() {
        v.push((format!("attempted:{label}"), raw));
    }
    for (label, raw) in super::super::law_tests::not_attempted_examples() {
        v.push((format!("not_attempted:{label}"), raw));
    }
    // Committed fixture requests, and the invocations inside the committed successor fixtures.
    if let Ok(root) = std::env::var("RV109_FIX") {
        for rel in std::env::var("RV109_FIX_FILES").unwrap_or_default().split(',').filter(|s| !s.is_empty()) {
            let text = std::fs::read_to_string(format!("{root}/{rel}")).unwrap_or_else(|e| panic!("{rel}: {e}"));
            let value: Value = serde_json::from_str(&text).unwrap();
            if value.get("invocation").is_some() {
                v.push((format!("fixture_invocation:{rel}"), value["invocation"]["request"].clone()));
            } else {
                v.push((format!("fixture:{rel}"), value));
            }
        }
    }
    // Files named by path (I86's inputs).
    for path in std::env::var("RV109_FILES").unwrap_or_default().split(',').filter(|s| !s.is_empty()) {
        let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
        let name = path.rsplit('/').next().unwrap().to_owned();
        v.push((format!("file:{name}"), serde_json::from_str(&text).unwrap()));
    }
    if let Ok(only) = std::env::var("RV109_ONLY") {
        let wanted: Vec<&str> = only.split(',').collect();
        v.retain(|(name, _)| wanted.iter().any(|w| name == w));
    }
    v
}

// ---- the independent oracles ----
const NOTICE_TEXT: &str = "Retained-precision recovery is unavailable for this load case. Its published rows keep their ordinary values, standing and diagnostics.";
/// The plain bytes with one plain N1 notice for `case` appended as the last diagnostic.
fn plain_plus_notice(plain: &[u8], case: &str) -> Option<Vec<u8>> {
    let text = std::str::from_utf8(plain).ok()?;
    let marker = "],\"professional_boundary\"";
    if text.matches(marker).count() != 1 {
        return None;
    }
    let at = text.find(marker)?;
    let empty = text[..at].ends_with("\"diagnostics\":[");
    let notice = json!({"id": format!("diagnostic:retained-precision:{case}:unavailable"), "code": "RETAINED_PRECISION_UNAVAILABLE",
        "severity": "info", "message": NOTICE_TEXT, "source": "core/product_physics", "affected_refs": [case]});
    // Field order as Diagnostic serializes: id, code, severity, message, source, affected_refs.
    let notice = format!(r#"{{"id":{},"code":{},"severity":{},"message":{},"source":{},"affected_refs":{}}}"#,
        notice["id"], notice["code"], notice["severity"], notice["message"], notice["source"], notice["affected_refs"]);
    Some(format!("{}{}{}{}", &text[..at], if empty { "" } else { "," }, notice, &text[at..]).into_bytes())
}
fn notices(bytes: &[u8]) -> Option<usize> {
    let value: Value = serde_json::from_slice(bytes).ok()?;
    Some(value["diagnostics"].as_array()?.iter().filter(|d| d["code"] == "RETAINED_PRECISION_UNAVAILABLE").count())
}
fn seed_summary(seed: &crate::retained_product::OrdinarySeed) -> Value {
    use crate::retained_product::{InitialSeed, W2Seed};
    let initial = match &seed.initial {
        None => "none".to_owned(),
        Some(InitialSeed::Report { code, .. }) => format!("report/{code}"),
        Some(InitialSeed::StructuralFailure { error, .. }) => {
            let tag = format!("{error:?}");
            format!("structural_failure/{}", tag.split(|c: char| !c.is_alphanumeric()).next().unwrap_or(""))
        }
        Some(InitialSeed::FormationFailure { .. }) => "formation_failure".to_owned(),
    };
    let w2 = match &seed.w2 {
        W2Seed::NotTriggered => "not_triggered".to_owned(),
        W2Seed::Published { force_scale_exponent, .. } => format!("published/b={force_scale_exponent}"),
        W2Seed::Failed { .. } => "failed".to_owned(),
    };
    json!({"case": seed.case, "initial": initial, "w2": w2, "recovery_demoted": seed.recovery_demoted, "d5": seed.d5_diagnostic_ref.is_some()})
}
fn cause_of<T>(r: &Result<T, crate::W1Fallback>) -> String {
    match r {
        Ok(_) => "Successor".into(),
        Err(f) => format!("{f:?}"),
    }
}
/// The furthest W1 phase a cause implies (PLAN_v2 §3.5's phase names, coarse).
fn phase_of(cause: &str) -> &'static str {
    match cause {
        "Successor" => "transfer (successor)",
        "NoTriggeredCase" => "T-4: no W1 (before the reservation)",
        "Domain" => "Domain: no W1",
        "Coexistence" => "coexistence: no W1",
        "NoticeReservation" => "reservation failed: no W1",
        "Preparation" => "preparation",
        "Native" => "native",
        "Candidate" => "proof (candidate)",
        c if c.starts_with("CompleteGate") => "G-C: no W1",
        c if c.starts_with("LateGate") => "G-B: no W1",
        c if c.starts_with("Staging") => "staging",
        c if c.starts_with("Serializer") => "serializer",
        c if c.starts_with("Precommit") => "precommit",
        "None" => "no permit: no W1",
        _ => "other",
    }
}

/// The private driver on the witness stack: (cause, envelope bytes, diagnostics reserved?,
/// successor bytes, seeds, verdicts). With `sentinel`, the native-stage fault is armed first and
/// the result says whether it was consumed (W1 reached native).
fn driver(raw: Value, mode: PreviewSolverMode, sentinel: bool) -> Value {
    hooks::disarm();
    if sentinel {
        hooks::withdraw_next_native_source();
    }
    let work = move || {
        let (request, capture) = match crate::source_receipt::CapturedInvocation::parse(raw, mode) {
            Ok(x) => x,
            Err(e) => return json!({"parse_error": format!("{e:?}")}),
        };
        let mut observer = crate::retained_product::ProductCapture::prepared_probe();
        let mut ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
        ordinary.diagnostics.shrink_to_fit();
        let seeds: Vec<Value> = observer.ordinary.iter().map(seed_summary).collect();
        let verdicts: Vec<Value> = ordinary.numerical_quality.cases.iter().map(|c| json!([c.basis_ref.ref_id, format!("{:?}", c.solve_quality)])).collect();
        let late = rev::late_total(&observer);
        let (envelope, retained) = crate::retained_w1(observer, ordinary, &capture);
        let reserved = envelope.diagnostics.capacity() > envelope.diagnostics.len();
        let env = serde_json::to_vec(&envelope).unwrap();
        let successor = retained.as_ref().ok().map(|s| sha(&serde_json::to_vec(s.value()).unwrap()));
        json!({"cause": cause_of(&retained), "env_sha": sha(&env), "env_len": env.len(), "reserved_slot": reserved,
            "successor_sha": successor, "seeds": seeds, "verdicts": verdicts, "late_loads_total": late, "notices": notices(&env)})
    };
    let mut out = crate::on_reserved_stack(WITNESS_STACK, crate::carry_test_hooks(work)).expect("the reserved thread spawned");
    hooks::reclaim_handed_back();
    if sentinel {
        out["native_reached"] = json!(!hooks::armed_names().contains(&"native"));
    }
    out["armed_after"] = json!(hooks::armed_names());
    hooks::disarm();
    out
}

/// A real permit through `admit` (Direct, the registered build), then the observed ordinary run with
/// `permitted_probe`: the seam's `late_loads_total` as G-B left it. Nothing else runs.
fn permitted_seam(raw: &Value, mode: PreviewSolverMode) -> Value {
    let Ok((request, capture)) = crate::source_receipt::CapturedInvocation::parse(raw.clone(), mode) else { return Value::Null };
    match super::super::admit(&capture, &request, super::super::Entry::Direct) {
        Ok((permit, _)) => {
            let mut observer = crate::retained_product::ProductCapture::permitted_probe(permit);
            let _ = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
            json!({"admitted": true, "late_loads_total": rev::late_total(&observer), "late_refused": observer.late_refusal().is_some(),
                "loads": raw["model"]["load_cases"].as_array().map(|cases| cases.iter().map(|c| c["primitive_loads"].as_array().map_or(0, Vec::len)).sum::<usize>())})
        }
        Err(_) => json!({"admitted": false}),
    }
}

fn probe_one(name: &str, raw: &Value, mode: PreviewSolverMode, sentinel: bool, out: &mut Option<std::fs::File>) {
    let input_sha = sha(&serde_json::to_vec(raw).unwrap());
    let plain = crate::run_linear_static_preview_value_with_mode(raw.clone(), mode).map(|e| serde_json::to_vec(&e).unwrap());
    let mut line = json!({"input": name, "mode": mode.as_str(), "input_sha": input_sha});
    match &plain {
        Ok(p) => { line["plain_sha"] = json!(sha(p)); line["plain_len"] = json!(p.len()); }
        Err(e) => { line["plain_err"] = json!(e); }
    }
    let case = raw["model"]["load_cases"][0]["id"].as_str().unwrap_or("").to_owned();
    // The actual Direct entry, counted.
    hooks::disarm();
    let raw_d = raw.clone();
    let armed_before = hooks::armed_names();
    let (direct, counts) = hooks::counted(move || crate::run_linear_static_preview_value_with_retained_direct(raw_d, mode));
    let armed_after = hooks::armed_names();
    match direct {
        Err(e) => { line["direct"] = json!({"err": e}); }
        Ok(output) => {
            let admission = output.admission().map(|a| json!({"profile": format!("{:?}", a.profile), "refusal": format!("{:?}", a.law().refusal),
                "domain": format!("{:?}", a.law().domain)}));
            let cause = output.retained().map_or("None".to_owned(), cause_of);
            let env_bytes = serde_json::to_vec(output.envelope()).unwrap();
            let successor = output.successor().is_some();
            let receipt = output.successor().and_then(|s| s["retained_precision"]["receipt_sha256"].as_str().map(str::to_owned));
            let bytes = match output.into_publication() {
                RetainedPublication::Successor(value) => serde_json::to_vec(&value).unwrap(),
                RetainedPublication::Ordinary(envelope) => serde_json::to_vec(&envelope).unwrap(),
            };
            let (eq_plain, eq_notice, env_eq_plain) = match &plain {
                Ok(p) => (bytes == *p, plain_plus_notice(p, &case).is_some_and(|n| n == bytes), env_bytes == *p),
                Err(_) => (false, false, false),
            };
            line["direct"] = json!({"admission": admission, "cause": cause, "phase": phase_of(&cause), "runs": counts.runs, "complete_gates": counts.complete_gates,
                "armed_before": armed_before, "armed_after": armed_after, "notices": notices(&bytes), "pub_sha": sha(&bytes), "pub_len": bytes.len(),
                "pub_eq_plain": eq_plain, "pub_eq_plain_plus_notice": eq_notice, "envelope_eq_plain": env_eq_plain, "successor": successor, "receipt_sha": receipt});
        }
    }
    // The private driver (the witnesses' path), at R/16.
    let mut d = driver(raw.clone(), mode, false);
    let env_sha = d["env_sha"].as_str().map(str::to_owned);
    if let (Ok(p), Some(env_sha)) = (&plain, env_sha) {
        let notice = plain_plus_notice(p, &case);
        d["env_eq_plain"] = json!(env_sha == sha(p));
        d["env_eq_plain_plus_notice"] = json!(notice.is_some_and(|n| sha(&n) == env_sha));
    }
    let cause = d["cause"].as_str().map(str::to_owned);
    if let Some(c) = cause {
        d["phase"] = json!(phase_of(&c));
    }
    line["driver"] = d;
    if sentinel {
        line["driver_sentinel"] = driver(raw.clone(), mode, true);
    }
    line["permitted_seam"] = permitted_seam(raw, mode);
    let text = line.to_string();
    println!("RV109 {text}");
    if let Some(f) = out {
        use std::io::Write;
        writeln!(f, "{text}").unwrap();
    }
}

#[test]
#[ignore]
fn zz_rv109_probe_all() {
    let mut out = std::env::var("RV109_OUT").ok().map(|p| std::fs::OpenOptions::new().create(true).append(true).open(p).unwrap());
    let sentinel = std::env::var("RV109_SENTINEL").map_or(true, |v| v != "0");
    for (name, raw) in inputs() {
        for mode in MODES {
            probe_one(&name, &raw, mode, sentinel, &mut out);
        }
    }
}

/// R10 discriminator (RV109's surviving mutant): on the private driver, with the observed
/// ordinary's diagnostics shrunk (no spare slot), a `NoTriggeredCase` input must leave no
/// reserved slot, and a base that already carries the notice's id must still give
/// `NoTriggeredCase` (T-4 before R-2's reservation), not `NoticeReservation`. Prints only.
#[test]
#[ignore]
fn zz_rv109_r10_discriminator() {
    for (name, raw) in [("two_body_b", two_body_b()), ("w6_phys_r4", w6_input())] {
        for mode in MODES {
            let case = raw["model"]["load_cases"][0]["id"].as_str().unwrap().to_owned();
            for collide in [false, true] {
                let (request, capture) = crate::source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
                let mut observer = crate::retained_product::ProductCapture::prepared_probe();
                let mut ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
                if collide {
                    ordinary.diagnostics.push(crate::Diagnostic { id: format!("diagnostic:retained-precision:{case}:unavailable"), code: "X".into(),
                        severity: "info".into(), message: "m".into(), source: None, affected_refs: Vec::new() });
                }
                ordinary.diagnostics.shrink_to_fit();
                let before = serde_json::to_vec(&ordinary).unwrap();
                let (envelope, retained) = crate::retained_w1(observer, ordinary, &capture);
                let line = json!({"input": name, "mode": mode.as_str(), "collide": collide, "cause": cause_of(&retained),
                    "spare_slot": envelope.diagnostics.capacity() > envelope.diagnostics.len(),
                    "bytes_unchanged": serde_json::to_vec(&envelope).unwrap() == before});
                println!("RV109_R10 {line}");
            }
        }
    }
}
/// Round 2 (RV109): the c = 1 adapter counts at each W1 stage boundary, through each revision's
/// production transaction (`rev::stages`: I1's `prepare_case`, `solve_native`, `freeze_candidate`,
/// `staged_envelope`, `serialize_frozen`; SP's `prepare_cases(…, 1, &[0])`, `native`, `freeze`,
/// `staged_envelope`, `serialize_cases`), then the accepted reader with the invocation. Prints only;
/// compared across revisions.
#[test]
#[ignore]
fn zz_rv109_stage_counts() {
    let mut out = std::env::var("RV109_STAGE_OUT").ok().map(|p| std::fs::OpenOptions::new().create(true).append(true).open(p).unwrap());
    for (name, raw) in inputs() {
        // c = 1 rows only (the one-case transaction on both revisions).
        if raw["model"]["load_cases"].as_array().map_or(0, Vec::len) != 1 {
            continue;
        }
        for mode in MODES {
            let Ok((request, capture)) = crate::source_receipt::CapturedInvocation::parse(raw.clone(), mode) else { continue };
            let mut observer = crate::retained_product::ProductCapture::prepared_probe();
            let mut ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
            ordinary.diagnostics.shrink_to_fit();
            let after_run = observer.adapter.counts.get();
            let seen = rev::cases_seen(&observer);
            let mut line = rev::stages(observer, ordinary, &capture);
            line["input"] = json!(name);
            line["mode"] = json!(mode.as_str());
            line["after_run"] = json!(after_run);
            line["cases_seen"] = json!(seen);
            if let Some(successor) = line.get("successor_value").cloned() {
                let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
                line["reader"] = json!(match open_pipe_stress_result_export::retained_precision::validate(&successor, Some(&invocation)) {
                    Ok(_) => "PASS".to_owned(),
                    Err(e) => format!("{} {}", e.gate, e.code),
                });
                line["successor_sha"] = json!(sha(&serde_json::to_vec(&successor).unwrap()));
                line.as_object_mut().unwrap().remove("successor_value");
            }
            let text = line.to_string();
            println!("RV109_STAGE {text}");
            if let Some(f) = &mut out {
                use std::io::Write;
                writeln!(f, "{text}").unwrap();
            }
        }
    }
}
