//! U4 G5 part 2: the S1 stack witnesses W1–W7 (STACK_PLAN.md §4; STACK_INVENTORY.md §3).
//!
//! Each witness runs, on one thread with R/k = 4 MiB of reserved stack (the
//! `on_reserved_stack` the permitted dispatch uses), the work the permitted path
//! runs there: the single observed ordinary run with capture installed, then the
//! W1 phases (`retained_w1`). The witnesses mint no permit (decision 7): the private driver
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
/// B1 ST (T-4; RR "I81's B1-0 probe verified…", ruling 4): a `NoTriggeredCase` pin on the witness
/// stack. The private driver's work, as `permitted_work` runs it, also hands back the ordinary
/// owner `retained_w1` returned; the pin asserts the cause and that owner's bytes against the
/// plain route's (exact bytes: no notice, no W1 work). `input_sha256` is the input's PROBE pin.
fn no_triggered_case_witness(label: &str, raw: Value, mode: PreviewSolverMode, input_sha256: &str) {
    use sha2::Digest;
    assert_eq!(format!("{:x}", sha2::Sha256::digest(serde_json::to_vec(&raw).unwrap())), input_sha256, "{label}: PROBE's input");
    let plain = serde_json::to_vec(&crate::run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap();
    let work = move || {
        let (request, capture) = CapturedInvocation::parse(raw, mode).expect("a valid request");
        let mut observer = crate::retained_product::ProductCapture::prepared_probe();
        let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut SourceRecoveryBudget::default(), Some(&mut observer));
        assert!(ordinary.source_block_recovery.is_none(), "not an exact-selected input");
        let (envelope, retained) = crate::retained_w1(observer, ordinary, &capture);
        (retained.err().map(|fallback| format!("{fallback:?}")), envelope)
    };
    let (cause, envelope) = crate::on_reserved_stack(WITNESS_STACK, crate::carry_test_hooks(work)).unwrap_or_else(|| panic!("{label}: the reserved thread did not spawn"));
    let ran = cause.map_or(Ran::Successor, Ran::Fallback);
    println!("I65_G5_WITNESS {label} stack={WITNESS_STACK} ran={ran:?}");
    assert_eq!(ran, Ran::Fallback("NoTriggeredCase".into()), "{label}");
    assert!(serde_json::to_vec(&envelope).unwrap() == plain, "{label}: the exact ordinary bytes");
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

/// W2b's input: the cap-maximal shape made solvable (one rigid support, 31 scalar springs). Its
/// ordinary report passes (`checks_passed`, no W2; R/I81/b1_probe_01 PROBE §2.1), so under T-4 (B1)
/// A is empty and it pins `NoTriggeredCase` at the largest counts: the exact ordinary bytes and no
/// W1 work (RR "I81's B1-0 probe verified…", ruling 4). Its former role, the only full native run at
/// the cap-maximal counts, moves to B1's SQ (W2b's replacement, from the SW probe).
#[test]
#[ignore]
fn witness_w2b_cap_maximal_passed_report_no_triggered_case() {
    for mode in MODES {
        no_triggered_case_witness(&format!("W2b {mode:?}"), w2b_input(), mode, W2B_INPUT_SHA256);
    }
}
/// W2b's input: `law_tests::cap_maximal` with the milestone's support shapes.
fn w2b_input() -> Value {
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
    raw
}
/// The PROBE pins (R/I81/b1_probe_01, `input_sha` lines) of W2b's and W6's PHYS-R4 inputs.
const W2B_INPUT_SHA256: &str = "d74d01ce1bc33244796877890ad134dd8476beb059bd82f26f2e004f8af619cb";
const W6_PHYS_R4_INPUT_SHA256: &str = "19a424c5ff064fa0010bcbb95b437e87092605e322f405e8252066995795e8f5";

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

/// W6's input before B1: PHYS-R4's cantilever (OD 4e-77 m, tip load about 1e-307) in the
/// legacy namespace, so the ordinary run scales by an exact power of two (f1b_w2_runtime.rs
/// `phys_r4(false)`, without its pressure contract). It is W2-published with the published
/// verdict `checks_passed` (R/I81/b1_probe_01 PROBE §2.1), so since B1 it is a `NoTriggeredCase`
/// pin, and W6's stack witness runs on W-C2's case C.
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

/// W6: an in-domain force-scaled (W2) case on the native stack path. Since B1 (T-4; RR "I81's
/// B1-0 probe verified…", ruling 3) its input is W-C2's case C alone (R/I81/b1_probe_01 PROBE
/// §4–§5): Sensitive and W2-published (b = 518), so W1 runs, and its native run climbs the full
/// ladder to Unresolved(Ceiling): `Fallback("Native")` in both modes.
#[test]
#[ignore]
fn witness_w6_force_scaled() {
    let raw = crate::retained_facade_tests::w_c2_case_c();
    {
        use sha2::Digest;
        assert_eq!(format!("{:x}", sha2::Sha256::digest(serde_json::to_vec(&raw).unwrap())), crate::retained_facade_tests::W_C2_CASE_C_INPUT_SHA256, "PROBE §4's case C");
    }
    let plain = crate::run_linear_static_preview_value_with_mode(raw.clone(), PreviewSolverMode::SparseInteractive).unwrap();
    let scaled = plain.diagnostics.iter().any(|d| d.message.contains("range_scaling: force_scale_exponent=") && !d.message.contains("force_scale_exponent=none"));
    println!("I65_G5_WITNESS_INPUT W6 force_scaled={scaled}");
    assert!(scaled, "W6: the ordinary run is force-scaled");
    for mode in MODES {
        let raw = raw.clone();
        let ran = witness(&format!("W6 {mode:?}"), WITNESS_STACK, move || permitted_work(raw, mode, |_| {}));
        println!("I65_G5_WITNESS_OUTCOME W6 {mode:?} {ran:?}");
        assert_eq!(ran, Ran::Fallback("Native".into()), "W6 {mode:?}: case C's full native ladder, then Native");
    }
}

/// B1 ST (T-4; RR "I81's B1-0 probe verified…", ruling 4): PHYS-R4's cantilever (`w6_input()`),
/// W6's input before B1, pins `NoTriggeredCase` on the witness stack in both modes.
#[test]
#[ignore]
fn witness_w6_phys_r4_input_no_triggered_case() {
    for mode in MODES {
        no_triggered_case_witness(&format!("W6-PHYS-R4 {mode:?}"), w6_input(), mode, W6_PHYS_R4_INPUT_SHA256);
    }
}

/// B1 ST repair 1 (RV109 N-4): not a witness, and not `#[ignore]`. W2b's input (an ordinary
/// Passed report at the cap-maximal counts) and W6's PHYS-R4 input (one-body, W2-published
/// Passed) are `NoTriggeredCase` pins in the default suite, by the facade tests' pin: exact plain
/// bytes, no notice, no reservation and no W1 work, on the private driver and the actual Direct
/// entry, registered or Stale. They do no W1 work, so they carry no stack risk; the witnesses
/// above keep them on the witness stack for G6.
#[test]
fn b1_t4_w2b_and_w6_phys_r4_inputs_are_no_triggered_case_pins() {
    crate::retained_facade_tests::assert_no_triggered_case_pin("W2b's input", &w2b_input(), W2B_INPUT_SHA256);
    crate::retained_facade_tests::assert_no_triggered_case_pin("W6's PHYS-R4 input", &w6_input(), W6_PHYS_R4_INPUT_SHA256);
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
