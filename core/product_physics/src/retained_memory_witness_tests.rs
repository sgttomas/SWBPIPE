//! U4 G5 part 2: the S1 stack witnesses W1–W7 (STACK_PLAN.md §4; STACK_INVENTORY.md §3).
//!
//! Each witness runs, on one thread with R/k = 4 MiB of reserved stack (the
//! `on_reserved_stack` the permitted dispatch uses), the work the permitted path
//! runs there: the single observed ordinary run with capture installed, then the
//! W1 phases (`retained_w1`). No permit exists (decision 7), so the private driver
//! enters `retained_w1` directly, as the facade tests do; G-B and G-C are not
//! consulted. The witness is measured evidence for this build and these inputs,
//! not a proof (D-3 = S1). A stack overflow aborts the test process, so the
//! witnesses are `#[ignore]` and run explicitly, one invocation each
//! (`cargo test --lib witness_ -- --ignored --test-threads=1`); G6 records them per
//! qualified build identity.
use super::*;
use crate::source_receipt::CapturedInvocation;
use crate::{PreviewSolverMode, SourceRecoveryBudget};
use serde_json::{json, Value};

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const WITNESS_STACK: usize = RESERVED_STACK_BYTES / STACK_WITNESS_DIVISOR;

fn milestone() -> Value {
    serde_json::from_str(include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json")).unwrap()
}
/// The outcome a witness records: what the reserved thread ran to.
#[derive(Debug, PartialEq)]
enum Ran {
    Successor,
    Fallback(String),
    ExactSelected,
}
/// The permitted path's work on the reserved thread, through the private driver.
fn permitted_work(raw: Value, mode: PreviewSolverMode, before_w1: impl FnOnce(&mut crate::retained_product::ProductCapture) + Send) -> Ran {
    let (request, capture) = CapturedInvocation::parse(raw, mode).expect("a valid request");
    let mut observer = crate::retained_product::ProductCapture::prepared_probe();
    let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut SourceRecoveryBudget::default(), Some(&mut observer));
    if ordinary.source_block_recovery.is_some() {
        let (_, retained) = crate::retained_w1(observer, ordinary, &capture);
        assert_eq!(retained.err(), Some(crate::W1Fallback::Coexistence));
        return Ran::ExactSelected;
    }
    let mut observer = observer;
    before_w1(&mut observer);
    if let Some(error) = &observer.error {
        println!("I65_G5_WITNESS_CAPTURE_ERROR {error:?}");
    }
    match crate::retained_w1(observer, ordinary, &capture).1 {
        Ok(_) => Ran::Successor,
        Err(fallback) => Ran::Fallback(format!("{fallback:?}")),
    }
}
/// Append a quote and a backslash to every provenance string (maximal escaping under D1.11).
fn escape_every_provenance(v: &mut Value) {
    match v {
        Value::Object(m) => {
            for (k, x) in m.iter_mut() {
                match x {
                    Value::String(s) if k == "provenance" => s.push_str(" q\"b\\"),
                    _ => escape_every_provenance(x),
                }
            }
        }
        Value::Array(a) => a.iter_mut().for_each(escape_every_provenance),
        _ => {}
    }
}
/// A raw value of depth 16 at the root (D1.2's cap: root at 0, 17 levels).
fn depth_16_value() -> Value {
    let mut deep = json!(1);
    for _ in 0..14 {
        deep = json!([deep]);
    }
    deep
}
fn witness(label: &str, stack: usize, work: impl FnOnce() -> Ran + Send) -> Ran {
    let ran = crate::on_reserved_stack(stack, crate::carry_test_hooks(work)).unwrap_or_else(|| panic!("{label}: the reserved thread did not spawn"));
    println!("I65_G5_WITNESS {label} stack={stack} ran={ran:?}");
    ran
}

/// W1 (and W5's dense half): the milestone, both modes: native, proof, serializer and
/// the precommit reader on the selected W1 path.
#[test]
#[ignore]
fn witness_w1_milestone() {
    for mode in MODES {
        let ran = witness(&format!("W1 {mode:?}"), WITNESS_STACK, || permitted_work(milestone(), mode, |_| {}));
        assert_eq!(ran, Ran::Successor);
    }
}

/// W2 (and W5's dense half): the cap-maximal D1 input (law_tests::cap_maximal), both
/// modes: the largest counts, 128-byte identifiers, a quote and a backslash in every
/// provenance, and a raw value of depth 16. Its support shapes stop it at preparation.
#[test]
#[ignore]
fn witness_w2_cap_maximal() {
    for mode in MODES {
        let mut raw = super::law_tests::cap_maximal();
        escape_every_provenance(&mut raw);
        raw["model"]["unknown_depth_witness"] = depth_16_value();
        let ran = witness(&format!("W2 {mode:?}"), WITNESS_STACK, move || permitted_work(raw, mode, |_| {}));
        assert_eq!(ran, Ran::Fallback("Preparation".into()), "W2 {mode:?}");
    }
}

/// W2's publishing half (RV89 G5 part 2 S-2): the milestone with a quote and a backslash
/// in every provenance and a raw value of depth 16, inside D1. It publishes a successor
/// at R/16 and at R/64 = 1 MiB in both modes, so the deepest raw Value and the maximal
/// escaping run through the serializer and the precommit reader on the witness stack.
#[test]
#[ignore]
fn witness_w2_deep_milestone_publishes() {
    for mode in MODES {
        let mut raw = milestone();
        escape_every_provenance(&mut raw);
        raw["model"]["deep_input_witness"] = depth_16_value();
        let (request, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
        let report = super::assess(&capture, &request, super::Entry::Direct);
        assert_eq!(report.law().domain, None, "inside D1");
        assert_eq!(report.raw.maximum_depth, 16, "raw depth 16");
        for stack in [WITNESS_STACK, RESERVED_STACK_BYTES / 64] {
            let raw = raw.clone();
            let ran = witness(&format!("W2-deep {mode:?}"), stack, move || permitted_work(raw, mode, |_| {}));
            assert_eq!(ran, Ran::Successor, "W2-deep {mode:?} at {stack} B");
        }
    }
}

/// W2b: the cap-maximal shape made solvable (one rigid support, 31 scalar springs), so the W1
/// phases run past preparation at the largest counts.
#[test]
#[ignore]
fn witness_w2b_cap_maximal_solvable() {
    for mode in MODES {
        let mut raw = super::law_tests::cap_maximal();
        let supports = raw["model"]["supports"].as_array_mut().unwrap();
        for (i, support) in supports.iter_mut().enumerate() {
            // The milestone's support shapes: a rigid support without a family, and
            // spring supports restraining exactly their stiffness DOF.
            if i == 0 {
                support.as_object_mut().unwrap().remove("stiffness");
            } else {
                support["family"] = json!("spring");
                support["restraints"] = json!(["UY"]);
            }
        }
        let ran = witness(&format!("W2b {mode:?}"), WITNESS_STACK, move || permitted_work(raw, mode, |_| {}));
        assert_eq!(ran, Ran::Fallback("Candidate".into()), "W2b {mode:?}: the full native run, then the candidate refuses");
    }
}

/// W3: an in-domain request whose legacy exact recovery selects (the X branch: T25's
/// requested() calls, the commitment, the publication and body hashes).
#[test]
#[ignore]
fn witness_w3_exact_selected() {
    for (mode, file) in [
        (PreviewSolverMode::SparseInteractive, include_str!("../../../fixtures/product_preview/source_blocks/n05-sparse_interactive.request.json")),
        (PreviewSolverMode::DenseScrutiny, include_str!("../../../fixtures/product_preview/source_blocks/n05-dense_scrutiny.request.json")),
    ] {
        let raw: Value = serde_json::from_str(file).unwrap();
        let ran = witness(&format!("W3 {mode:?}"), WITNESS_STACK, move || permitted_work(raw, mode, |_| {}));
        assert_eq!(ran, Ran::ExactSelected, "the exact-block selection ran");
    }
}

/// W4: the preparation refusal and its fallback chain (the closed annulus helper).
#[test]
#[ignore]
fn witness_w4_preparation_refusal() {
    for mode in MODES {
        let ran = witness(&format!("W4 {mode:?}"), WITNESS_STACK, || permitted_work(milestone(), mode, |o| o.facts[0].diameter = 0.0));
        assert_eq!(ran, Ran::Fallback("Preparation".into()));
    }
}

/// W6: an in-domain force-scaled (W2) case: PHYS-R4's cantilever (OD 4e-77 m, tip
/// load about 1e-307) in the legacy namespace, so the ordinary run scales by an exact
/// power of two (f1b_w2_runtime.rs `phys_r4(false)`, without its pressure contract).
/// W6's input: PHYS-R4's cantilever in the legacy namespace (force-scaled).
pub(super) fn w6_input() -> Value {
    let (a, b) = ("node:section-a", "node:section-b");
    let tip = f64::from_bits(0x0031fa182c40c60d);
    let p = "invented_t3_g5_witness_input_no_library_data";
    let raw = json!({"model": {
        "schema_version": "0.1.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:section-oracle", "units": {"length": "m", "force": "N"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed", "professional_acceptance": "not_provided"},
        "nodes": [{"id": a, "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": p},
                  {"id": b, "position": {"x": 1.0, "y": 0.0, "z": 0.0}, "provenance": p}],
        "pipe_segments": [{"id": "pipe:source-section", "from": a, "to": b, "material": "material:section", "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
            "section": {"outside_diameter": {"value": 4e-77, "unit": "m"}, "wall_thickness": {"value": 1e-77, "unit": "m"}}, "provenance": p}],
        "materials": [{"id": "material:section", "elastic_modulus": {"value": 1.0, "unit": "Pa"}, "shear_modulus": {"value": 0.4545, "unit": "Pa"}, "provenance": p}],
        "supports": [{"id": "support:section-a", "node": a, "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": p}],
        "load_cases": [{"id": "case:source-section", "provenance": p, "primitive_loads": [
            {"id": "load:tip-y", "category": "concentrated_force", "target": {"type": "node", "node": b}, "direction": "global_y", "dimension": "force", "magnitude": {"value": tip, "unit": "N"}, "provenance": p},
            {"id": "load:tip-torque", "category": "concentrated_moment", "target": {"type": "node", "node": b}, "direction": "rotation_x", "dimension": "moment", "magnitude": {"value": tip, "unit": "N*m"}, "provenance": p}]}],
        "combinations": []}, "materials": []});
    raw
}

#[test]
#[ignore]
fn witness_w6_force_scaled() {
    let raw = w6_input();
    let plain = crate::run_linear_static_preview_value_with_mode(raw.clone(), PreviewSolverMode::SparseInteractive).unwrap();
    let scaled = plain.diagnostics.iter().any(|d| d.message.contains("range_scaling: force_scale_exponent=") && !d.message.contains("force_scale_exponent=none"));
    println!("I65_G5_WITNESS_INPUT W6 force_scaled={scaled}");
    for mode in MODES {
        let raw = raw.clone();
        let ran = witness(&format!("W6 {mode:?}"), WITNESS_STACK, move || permitted_work(raw, mode, |_| {}));
        println!("I65_G5_WITNESS_OUTCOME W6 {mode:?} {ran:?}");
    }
}

/// W7: U3's fault-injection controls, carried onto the reserved thread: each stage's
/// fallback (native, serializer, staging, precommit binding and corruption).
#[test]
#[ignore]
fn witness_w7_fault_fallbacks() {
    use crate::retained_tests_hooks as hooks;
    let mode = PreviewSolverMode::SparseInteractive;
    hooks::withdraw_next_native_source();
    assert_eq!(witness("W7 native", WITNESS_STACK, || permitted_work(milestone(), mode, |_| {})), Ran::Fallback("Native".into()));
    hooks::fail_next_serializer(crate::retained_wire::ReceiptCheck::Encoding);
    assert!(matches!(witness("W7 serializer", WITNESS_STACK, || permitted_work(milestone(), mode, |_| {})), Ran::Fallback(f) if f.starts_with("Serializer")));
    hooks::break_next_staging();
    assert!(matches!(witness("W7 staging", WITNESS_STACK, || permitted_work(milestone(), mode, |_| {})), Ran::Fallback(f) if f.starts_with("Staging")));
    hooks::rebind_next_precommit_invocation();
    assert!(matches!(witness("W7 precommit binding", WITNESS_STACK, || permitted_work(milestone(), mode, |_| {})), Ran::Fallback(f) if f.starts_with("Precommit")));
    hooks::corrupt_next_precommit();
    assert!(matches!(witness("W7 precommit corruption", WITNESS_STACK, || permitted_work(milestone(), mode, |_| {})), Ran::Fallback(f) if f.starts_with("Precommit")));
}

/// Headroom (carry item 8): W1 again at R/64 = 1 MiB. Its reader validates the whole
/// successor, so passing here bounds the reader's schema-walk chain on the W1
/// input at a quarter of the witness stack. The 36-level `$ref` chain is bounded by
/// source (schema_depth.py); whether W1's successor drives the deepest chain is not
/// observable without instrumenting the reader, which is outside the fence.
#[test]
#[ignore]
fn witness_headroom_w1_at_one_mebibyte() {
    for mode in MODES {
        let ran = witness(&format!("W1@1MiB {mode:?}"), RESERVED_STACK_BYTES / 64, || permitted_work(milestone(), mode, |_| {}));
        assert_eq!(ran, Ran::Successor);
    }
}
