//! RV82 review probe (disposable copy only; never committed). Dumps the
//! serializer's milestone output and native facts for an independent Python
//! derivation, and sweeps fixture requests for the F1 predicate and the G-l
//! capture against the ordinary envelope's own diagnostics.
use super::retained_product as rp;
use super::retained_wire as wire;
use super::*;
use open_pipe_stress_frame_kernel::structural::retained_api as k;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const MILESTONE: &str = include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn out_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(std::env::var("RV82_OUT").expect("RV82_OUT"))
}

#[test]
fn rv82_dump_milestone() {
    let dir = out_dir();
    for mode in MODES {
        let raw: Value = serde_json::from_str(MILESTONE).unwrap();
        let plain = serde_json::to_vec(&run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap();
        let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
        let mut prepared = rp::PreparedCase::prepare_observed(request, mode, &capture).unwrap_or_else(|e| panic!("{:?}", e.capture.error));
        let ordinary = serde_json::to_vec(prepared.ordinary()).unwrap();
        prepared.solve_native().unwrap();
        let candidate = match prepared.project_candidate() { Ok(c) => c, Err(e) => panic!("{:?}", e.error) };
        // B' analogue: the envelope the serializer starts from is the plain run's bytes.
        let candidate_env = serde_json::to_vec(candidate.envelope()).unwrap();
        let successor = wire::serialize_selected(&candidate, &capture).unwrap_or_else(|f| panic!("{f:?}"));
        let pc = candidate.capture();
        let (inv, case) = pc.native.as_ref().unwrap();
        let k::ExecutionOutcome::Selected(owner) = &case.outcome else { panic!("selected") };
        let ev = owner.evidence();
        let rows = pc.bind_rows(candidate.envelope(), owner).unwrap();
        let verdicts: Vec<Value> = candidate.certificate().verdicts().iter().map(|v| json!({
            "row": v.row, "row_id": rows[v.row].id, "class": format!("{:?}", v.class), "recipe": format!("{:?}", rows[v.row].recipe), "passed": v.passed,
        })).collect();
        let constraints: Vec<Value> = owner.source().constraints().iter().map(|c| json!({"node": c.dof.node, "component": c.dof.component.index()})).collect();
        let seed = &pc.ordinary[0];
        let facts = json!({
            "mode": mode.as_str(),
            "plain_len": plain.len(), "plain_sha256": sha(&plain),
            "captured_ordinary_equals_plain": ordinary == plain,
            "candidate_envelope_equals_plain": candidate_env == plain,
            "k4rst_prefix_hex": ev.retained_state_encoding.iter().take(6).map(|b| format!("{b:02x}")).collect::<String>(),
            "k4rst_len": ev.retained_state_encoding.len(),
            "k4rst_sha256": sha(&ev.retained_state_encoding),
            "ledger_sha256": sha(&ev.ledger_encoding),
            "verdicts": verdicts,
            "kernel_constraints": constraints,
            "runs": inv.runs().len(), "calls": inv.calls().len(), "sources": inv.sources().len(), "groups": inv.groups().len(), "builds": inv.builds().len(),
            "case_run": case.run,
            "seed_debug": format!("{seed:?}"),
            "meter_limit": inv.meter().limit(),
        });
        let name = mode.as_str();
        std::fs::write(dir.join(format!("successor_{name}.json")), serde_json::to_string_pretty(&successor).unwrap()).unwrap();
        std::fs::write(dir.join(format!("plain_{name}.json")), &plain).unwrap();
        std::fs::write(dir.join(format!("facts_{name}.json")), serde_json::to_string_pretty(&facts).unwrap()).unwrap();
        // The committed-test file shape, to compare with experiment 03 and the pins.
        let invocation = json!({"request": raw, "solver_mode": name});
        let text = serde_json::to_string_pretty(&json!({"id": format!("u1_milestone_{name}"), "source": successor, "invocation": invocation})).unwrap();
        std::fs::write(dir.join(format!("u1_milestone_{name}.json")), &text).unwrap();
    }
}

fn sweep_requests() -> Vec<(String, Value)> {
    let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut out = Vec::new();
    for rel in [
        "../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json",
        "../../fixtures/product_preview/invented_preview_model.json",
        "../../fixtures/product_preview/numerical_sensitive_torsion_model.json",
        "../../fixtures/product_preview/source_blocks/n05-sparse_interactive.request.json",
        "../../fixtures/product_preview/source_blocks/n06-sparse_interactive.request.json",
        "../../fixtures/product_preview/source_blocks/multicase-sparse_interactive.request.json",
        "../../fixtures/product_preview/physics_source/mixed.request.json",
        "../../fixtures/product_preview/physics_source/n05_units.request.json",
        "../../fixtures/product_preview/physics_source/n06.request.json",
        "../../fixtures/product_preview/physics_source/fields.request.json",
        "../../fixtures/product_preview/invented_dec092_temperature_g_request.json",
        "tests/fixtures/preview_physics_invented_model.json",
    ] {
        if let Ok(text) = std::fs::read_to_string(base.join(rel)) {
            if let Ok(v) = serde_json::from_str::<Value>(&text) {
                out.push((rel.to_string(), v));
            }
        }
    }
    for rel in ["tests/fixtures/s11f/rf_cancel_cases.json", "tests/fixtures/s11g/rb_controls.json"] {
        let v: Value = serde_json::from_str(&std::fs::read_to_string(base.join(rel)).unwrap()).unwrap();
        for c in v["cases"].as_array().unwrap() {
            if c.get("request").is_some() {
                out.push((format!("{rel}#{}", c["id"].as_str().unwrap_or("?")), c["request"].clone()));
            }
        }
    }
    out
}

/// F1 predicate and G-l capture, swept over fixture requests (oracle: the
/// envelope's own diagnostics; test-only text reads).
#[test]
fn rv82_sweep_f1_and_gl() {
    let mut lines = Vec::new();
    let (mut checked, mut d5_present, mut d5_absent) = (0, 0, 0);
    let mut dispositions: std::collections::BTreeMap<String, usize> = Default::default();
    for (name, raw) in sweep_requests() {
        for mode in MODES {
            let Ok((typed, capture)) = source_receipt::CapturedInvocation::parse(raw.clone(), mode) else {
                lines.push(format!("{name} {} parse-refused", mode.as_str()));
                continue;
            };
            let Ok(plain) = run_linear_static_preview_value_with_mode(raw.clone(), mode) else {
                lines.push(format!("{name} {} plain-error", mode.as_str()));
                continue;
            };
            let mut observer = rp::ProductCapture::prepared_probe();
            let (typed_bare, _) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
            let bare = run_linear_static_preview_observed(typed_bare, mode, Some(&capture), &mut SourceRecoveryBudget::default(), None);
            let observed = run_linear_static_preview_observed(typed, mode, Some(&capture), &mut SourceRecoveryBudget::default(), Some(&mut observer));
            // Capture installed vs absent on the same observed route: identical bytes.
            assert_eq!(serde_json::to_vec(&observed).unwrap(), serde_json::to_vec(&bare).unwrap(), "{name} {mode:?}: ordinary bytes under capture");
            let plain_equal = serde_json::to_vec(&observed).unwrap() == serde_json::to_vec(&plain).unwrap();
            lines.push(format!("{name} {} observed_route_equals_public_dispatch={plain_equal}", mode.as_str()));
            for seed in &observer.ordinary {
                checked += 1;
                let integrity_id = format!("diagnostic:numerical-integrity:{}", seed.case);
                let integrity = observed.diagnostics.iter().find(|d| d.id == integrity_id);
                let line_present = integrity.is_some_and(|d| d.message.contains(" formation_check: reason="));
                // F1: the capture predicate equals "K-D5's line is in that diagnostic".
                assert_eq!(seed.d5_diagnostic_ref.is_some(), line_present, "{name} {mode:?} {}: F1 predicate", seed.case);
                if let Some(r) = &seed.d5_diagnostic_ref {
                    assert_eq!(r, &integrity_id);
                    d5_present += 1;
                } else {
                    d5_absent += 1;
                }
                // G-l: the typed route against the envelope's legacy disclosure.
                let legacy_id = format!("diagnostic:source-recovery:{}", seed.case);
                let legacy = observed.diagnostics.iter().find(|d| d.id == legacy_id);
                let selected = observed.diagnostics.iter().any(|d| d.id == format!("{legacy_id}:selected"));
                let label = match &seed.legacy {
                    Some(rp::LegacySeed::NotEligible) => { assert!(legacy.is_none() && !selected, "{name}"); "not_eligible".to_string() }
                    Some(rp::LegacySeed::NotRequired) => { assert!(legacy.is_none() && !selected, "{name}"); "not_required".to_string() }
                    Some(rp::LegacySeed::ExactSelected) => { assert!(legacy.is_none() && selected, "{name}"); "exact_selected".to_string() }
                    Some(rp::LegacySeed::DeclinedWithoutAttempt { work, diagnostic_ref }) | Some(rp::LegacySeed::Unavailable { work, diagnostic_ref }) => {
                        let d = legacy.expect("legacy disclosure");
                        assert_eq!(diagnostic_ref, &d.id);
                        assert_eq!(d.code, "SOURCE_BLOCK_RECOVERY_UNAVAILABLE");
                        // Oracle only: the Debug text the ordinary route already publishes.
                        for needle in [format!("stage: \"{}\"", work.stage), format!("charged: {}", work.charged), format!("rejected: {}", work.rejected), format!("limit: {}", work.limit)] {
                            assert!(d.message.contains(&needle), "{name} {mode:?}: {needle} in {}", d.message);
                        }
                        let declined = matches!(seed.legacy, Some(rp::LegacySeed::DeclinedWithoutAttempt { .. }));
                        if declined { assert_eq!((work.charged, work.rejected, work.limit), (0, 0, 0), "{name}"); }
                        format!("{}:{}", if declined { "declined_without_attempt" } else { "unavailable" }, work.stage)
                    }
                    None => "none".to_string(),
                };
                *dispositions.entry(label.clone()).or_default() += 1;
                lines.push(format!("{name} {} case={} d5={} legacy={label} initial={:?} w2={:?} finding_ref={:?} demoted={}",
                    mode.as_str(), seed.case, seed.d5_diagnostic_ref.is_some(),
                    seed.initial.as_ref().map(|i| match i { rp::InitialSeed::Report { code, .. } => code.clone(), rp::InitialSeed::StructuralFailure { .. } => "structural_failure".into(), rp::InitialSeed::FormationFailure { .. } => "formation_failure".into() }),
                    match &seed.w2 { rp::W2Seed::NotTriggered => "not_triggered", rp::W2Seed::Published { .. } => "published", rp::W2Seed::Failed { .. } => "failed" },
                    seed.load_row_finding.as_ref().map(|f| f.diagnostic_ref.is_some()), seed.recovery_demoted));
            }
        }
    }
    lines.push(format!("SUMMARY seeds={checked} d5_present={d5_present} d5_absent={d5_absent} dispositions={dispositions:?}"));
    std::fs::write(out_dir().join("sweep_f1_gl.txt"), lines.join("\n") + "\n").unwrap();
    assert!(d5_present > 0 && d5_absent > 0, "both directions exercised");
}

/// Item 4: the accepted Rust reader (dev-dependency bytes = NUM's) on U1's files.
#[test]
fn rv82_rust_reader() {
    use open_pipe_stress_result_export::retained_precision as reader;
    let dir = std::path::PathBuf::from(std::env::var("RV82_READ_DIR").expect("RV82_READ_DIR"));
    let mut lines = Vec::new();
    let mut names: Vec<_> = std::fs::read_dir(&dir).unwrap().filter_map(|e| e.ok().map(|e| e.file_name().into_string().unwrap()))
        .filter(|n| n.starts_with("u1_milestone_") && n.ends_with(".json")).collect();
    names.sort();
    for n in names {
        let c: Value = serde_json::from_str(&std::fs::read_to_string(dir.join(&n)).unwrap()).unwrap();
        match reader::validate(&c["source"], Some(&c["invocation"])) {
            Ok(v) => {
                lines.push(format!("RS {n} PASS invocation_bound={} eligible={} classes={}", v.invocation_bound, v.numerical_eligible, v.classifications.len()));
                let text: String = v.classifications.iter().map(|x| {
                    let class = match &x.class {
                        reader::AccuracyClass::RelativeVerified => "relative_verified",
                        reader::AccuracyClass::AbsoluteVerified { .. } => "absolute_verified",
                        reader::AccuracyClass::InputDerived => "input_derived",
                        reader::AccuracyClass::NonQuantity => "non_quantity",
                        reader::AccuracyClass::NotCovered => "not_covered",
                    };
                    format!("{}|{:016x}|{}|{class}\n", x.result_id, x.normalized_bits, x.scale_bits.map_or("null".to_string(), |b| format!("{b:016x}")))
                }).collect();
                std::fs::write(dir.join(format!("{n}.rs_classes.txt")), text).unwrap();
            }
            Err(e) => lines.push(format!("RS {n} FIRST {} {} {:?}", e.gate, e.code, e.detail)),
        }
    }
    std::fs::write(dir.join("rs_results.txt"), lines.join("\n") + "\n").unwrap();
}

/// Association probe: the serializer binds the supplied CapturedInvocation to the
/// candidate by mode and case id only. Supply a different same-mode one-case
/// invocation (a label change; a load-magnitude change) and record what happens,
/// and what the accepted Rust reader says with each invocation.
#[test]
fn rv82_foreign_invocation() {
    use open_pipe_stress_result_export::retained_precision as reader;
    let mode = PreviewSolverMode::SparseInteractive;
    let raw: Value = serde_json::from_str(MILESTONE).unwrap();
    let (request, own) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
    let mut prepared = rp::PreparedCase::prepare_observed(request, mode, &own).unwrap_or_else(|e| panic!("{:?}", e.capture.error));
    prepared.solve_native().unwrap();
    let candidate = match prepared.project_candidate() { Ok(c) => c, Err(e) => panic!("{:?}", e.error) };
    let own_inv = json!({"request": raw, "solver_mode": mode.as_str()});
    let mut lines = Vec::new();
    let mut variants: Vec<(&str, Value)> = Vec::new();
    let mut label = raw.clone();
    label["model"]["load_cases"][0]["label"] = json!("RV82 foreign label");
    variants.push(("label", label));
    let mut project = raw.clone();
    project["model"]["project"]["id"] = json!("invented:rv82:foreign");
    variants.push(("project_id", project));
    let mut load = raw.clone();
    load["model"]["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"] = json!(0.0049);
    variants.push(("load_magnitude", load));
    for (name, other_raw) in variants {
        let (_, other) = source_receipt::CapturedInvocation::parse(other_raw.clone(), mode).unwrap();
        let other_inv = json!({"request": other_raw, "solver_mode": mode.as_str()});
        match wire::serialize_selected(&candidate, &other) {
            Err(f) => lines.push(format!("{name}: serializer refused {f:?}")),
            Ok(s) => {
                let with_other = reader::validate(&s, Some(&other_inv)).map(|v| format!("PASS bound={}", v.invocation_bound)).unwrap_or_else(|e| format!("FIRST {} {}", e.gate, e.code));
                let with_own = reader::validate(&s, Some(&own_inv)).map(|v| format!("PASS bound={}", v.invocation_bound)).unwrap_or_else(|e| format!("FIRST {} {}", e.gate, e.code));
                lines.push(format!("{name}: serializer EMITTED; reader(with the foreign invocation)={with_other}; reader(with the candidate's own invocation)={with_own}"));
            }
        }
    }
    std::fs::write(out_dir().join("foreign_invocation.txt"), lines.join("\n") + "\n").unwrap();
}
