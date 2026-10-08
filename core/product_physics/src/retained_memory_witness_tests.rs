//! U4 G5 part 2: the S1 stack witnesses W1–W7 (STACK_PLAN.md §4; STACK_INVENTORY.md §3).
//!
//! Each witness runs, on one thread with R/k = 4 MiB of reserved stack (the
//! `on_reserved_stack` the permitted dispatch uses), the work the permitted path
//! runs there: the single observed ordinary run with capture installed, then the
//! W1 phases (`retained_w1`). The witnesses mint no permit (decision 7): the private driver
//! enters `retained_w1` directly, as the facade tests do; G-B and G-C are not
//! consulted. The witness is measured evidence for this build and these inputs,
//! not a proof (D-3 = S1). A stack overflow aborts the test process, so the
//! witnesses are `#[ignore]` and run explicitly, one invocation each; G6 records them per
//! qualified build identity.
//!
//! B1 SQ (PLAN_v2 §3.4, RV107 SF-5): each witness has one entry point per mode, run as its own
//! process: `<lib test binary> <witness>::sparse --exact --ignored --test-threads=1 --nocapture`
//! (and `::dense`). W7's fault controls run in one mode, as before. The B1 inputs (W2b's
//! replacement, the c = 1 publishing input, the three-case input) are shared with the challenge
//! through `tests/common/b1_sq_inputs.rs`, pinned by I86's input hashes. The `control_*` entries
//! are RSS_TIME.md's controls in this build: the ordinary value route alone, and the process floor.
use super::*;
use crate::source_receipt::CapturedInvocation;
use crate::{PreviewSolverMode, SourceRecoveryBudget};
use serde_json::{json, Value};

#[path = "../tests/common/b1_sq_inputs.rs"]
mod b1_sq_inputs;
use b1_sq_inputs as inputs;

const WITNESS_STACK: usize = RESERVED_STACK_BYTES / STACK_WITNESS_DIVISOR;

fn milestone() -> Value {
    serde_json::from_str(include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json")).unwrap()
}
/// One witness entry point per mode: `$name::sparse` and `$name::dense`, each `#[ignore]`.
macro_rules! per_mode {
    ($(#[$doc:meta])* $name:ident, |$mode:ident| $body:block) => {
        $(#[$doc])*
        mod $name {
            use super::*;
            fn run($mode: PreviewSolverMode) $body
            #[test]
            #[ignore]
            fn sparse() {
                run(PreviewSolverMode::SparseInteractive)
            }
            #[test]
            #[ignore]
            fn dense() {
                run(PreviewSolverMode::DenseScrutiny)
            }
        }
    };
}
fn sha256_of(raw: &Value) -> String {
    use sha2::Digest;
    format!("{:x}", sha2::Sha256::digest(serde_json::to_vec(raw).unwrap()))
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
    let start = std::time::Instant::now();
    let (request, capture) = CapturedInvocation::parse(raw, mode).expect("a valid request");
    let mut observer = crate::retained_product::ProductCapture::prepared_probe();
    let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut SourceRecoveryBudget::default(), Some(&mut observer));
    let ordinary_ms = start.elapsed().as_secs_f64() * 1e3;
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
    let w1 = std::time::Instant::now();
    let retained = crate::retained_w1(observer, ordinary, &capture).1;
    // B1 SQ (PLAN_v2 §3.6): the ordinary observed run's and W1's wall times, in this build.
    println!("I104_SQ_WITNESS_TIME {mode:?} parse_and_ordinary_ms={ordinary_ms:.1} w1_ms={:.1}", w1.elapsed().as_secs_f64() * 1e3);
    match retained {
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

per_mode! {
    /// W1 (and W5's dense half): the milestone: native, proof, serializer and the precommit
    /// reader on the selected W1 path.
    witness_w1_milestone, |mode| {
        let ran = witness(&format!("W1 {mode:?}"), WITNESS_STACK, move || permitted_work(milestone(), mode, |_| {}));
        assert_eq!(ran, Ran::Successor);
    }
}

per_mode! {
    /// W2 (and W5's dense half): the cap-maximal D1 input (law_tests::cap_maximal): the largest
    /// counts, 128-byte identifiers, a quote and a backslash in every provenance, and a raw value
    /// of depth 16. Its support shapes stop it at preparation.
    witness_w2_cap_maximal, |mode| {
        let raw = inputs::w2();
        let ran = witness(&format!("W2 {mode:?}"), WITNESS_STACK, move || permitted_work(raw, mode, |_| {}));
        assert_eq!(ran, Ran::Fallback("Preparation".into()), "W2 {mode:?}");
    }
}

per_mode! {
    /// W2's publishing half (RV89 G5 part 2 S-2): the milestone with a quote and a backslash in
    /// every provenance and a raw value of depth 16, inside D1. It publishes a successor at R/16
    /// and at R/64 = 1 MiB, so the deepest raw Value and the maximal escaping run through the
    /// serializer and the precommit reader on the witness stack.
    witness_w2_deep_milestone_publishes, |mode| {
        let mut raw = milestone();
        escape_every_provenance(&mut raw);
        raw["model"]["deep_input_witness"] = depth_16_value();
        let (request, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
        let report = super::super::assess(&capture, &request, super::super::Entry::Direct);
        assert_eq!(report.law().domain, None, "inside D1");
        assert_eq!(report.raw.maximum_depth, 16, "raw depth 16");
        for stack in [WITNESS_STACK, RESERVED_STACK_BYTES / 64] {
            let raw = raw.clone();
            let ran = witness(&format!("W2-deep {mode:?}"), stack, move || permitted_work(raw, mode, |_| {}));
            assert_eq!(ran, Ran::Successor, "W2-deep {mode:?} at {stack} B");
        }
    }
}

per_mode! {
    /// W2b's input: the cap-maximal shape made solvable (one rigid support, 31 scalar springs). Its
    /// ordinary report passes (`checks_passed`, no W2; R/I81/b1_probe_01 PROBE §2.1), so under T-4
    /// (B1) A is empty and it pins `NoTriggeredCase` at the largest counts: the exact ordinary bytes
    /// and no W1 work (RR "I81's B1-0 probe verified…", ruling 4). Its former role, the only full
    /// native run at the cap-maximal counts, moves to W2b's replacement below.
    witness_w2b_cap_maximal_passed_report_no_triggered_case, |mode| {
        no_triggered_case_witness(&format!("W2b {mode:?}"), w2b_input(), mode, W2B_INPUT_SHA256);
    }
}

per_mode! {
    /// B1 SQ: W2b's replacement (RR "I86's SW probe accepted; …", ruling 1): W2b's input with S0 an
    /// anchor restraining UX, UY, UZ, RY, RZ and S31 an RX spring of 1000 N·m/rad at N0. Sensitive
    /// by its report, so W1 runs at the cap-maximal counts; the candidate fails (`Candidate`).
    witness_w2b_replacement_b2_k1e3, |mode| {
        let raw = inputs::b2_k1e3();
        assert_eq!(sha256_of(&raw), inputs::B2_K1E3_SHA256, "I86's b2_k1e3");
        let ran = witness(&format!("W2b-replacement b2_k1e3 {mode:?}"), WITNESS_STACK, move || permitted_work(raw, mode, |_| {}));
        assert_eq!(ran, Ran::Fallback("Candidate".into()), "b2_k1e3 {mode:?}");
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

per_mode! {
    /// W3: an in-domain request whose legacy exact recovery selects (the X branch: T25's
    /// requested() calls, the commitment, the publication and body hashes).
    witness_w3_exact_selected, |mode| {
        let file = match mode {
            PreviewSolverMode::SparseInteractive => include_str!("../../../fixtures/product_preview/source_blocks/n05-sparse_interactive.request.json"),
            _ => include_str!("../../../fixtures/product_preview/source_blocks/n05-dense_scrutiny.request.json"),
        };
        let raw: Value = serde_json::from_str(file).unwrap();
        let ran = witness(&format!("W3 {mode:?}"), WITNESS_STACK, move || permitted_work(raw, mode, |_| {}));
        assert_eq!(ran, Ran::ExactSelected, "the exact-block selection ran");
    }
}

per_mode! {
    /// W4: the preparation refusal and its fallback chain (the closed annulus helper).
    witness_w4_preparation_refusal, |mode| {
        let ran = witness(&format!("W4 {mode:?}"), WITNESS_STACK, move || permitted_work(milestone(), mode, |o| o.facts[0].diameter = 0.0));
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

per_mode! {
    /// W6: an in-domain force-scaled (W2) case on the native stack path. Since B1 (T-4; RR "I81's
    /// B1-0 probe verified…", ruling 3) its input is W-C2's case C alone (R/I81/b1_probe_01 PROBE
    /// §4–§5): Sensitive and W2-published (b = 518), so W1 runs, and its native run climbs the full
    /// ladder to Unresolved(Ceiling): `Fallback("Native")`.
    witness_w6_force_scaled, |mode| {
        let raw = crate::retained_facade_tests::w_c2_case_c();
        assert_eq!(sha256_of(&raw), crate::retained_facade_tests::W_C2_CASE_C_INPUT_SHA256, "PROBE §4's case C");
        let plain = crate::run_linear_static_preview_value_with_mode(raw.clone(), PreviewSolverMode::SparseInteractive).unwrap();
        let scaled = plain.diagnostics.iter().any(|d| d.message.contains("range_scaling: force_scale_exponent=") && !d.message.contains("force_scale_exponent=none"));
        println!("I65_G5_WITNESS_INPUT W6 force_scaled={scaled}");
        assert!(scaled, "W6: the ordinary run is force-scaled");
        let ran = witness(&format!("W6 {mode:?}"), WITNESS_STACK, move || permitted_work(raw, mode, |_| {}));
        println!("I65_G5_WITNESS_OUTCOME W6 {mode:?} {ran:?}");
        assert_eq!(ran, Ran::Fallback("Native".into()), "W6 {mode:?}: case C's full native ladder, then Native");
    }
}

per_mode! {
    /// B1 ST (T-4; RR "I81's B1-0 probe verified…", ruling 4): PHYS-R4's cantilever (`w6_input()`),
    /// W6's input before B1, pins `NoTriggeredCase` on the witness stack.
    witness_w6_phys_r4_input_no_triggered_case, |mode| {
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
/// fallback (native, serializer, staging, precommit binding and corruption). One mode.
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

per_mode! {
    /// Headroom (carry item 8): W1 again at R/64 = 1 MiB. Its reader validates the whole
    /// successor, so passing here bounds the reader's schema-walk chain on the W1 input at a
    /// quarter of the witness stack. The 36-level `$ref` chain is bounded by source
    /// (schema_depth.py); whether W1's successor drives the deepest chain is not observable
    /// without instrumenting the reader, which is outside the fence.
    witness_headroom_w1_at_one_mebibyte, |mode| {
        let ran = witness(&format!("W1@1MiB {mode:?}"), RESERVED_STACK_BYTES / 64, move || permitted_work(milestone(), mode, |_| {}));
        assert_eq!(ran, Ran::Successor);
    }
}

// ---- B1 SQ (PLAN_v2 §3.4): the multi-case and cap-maximal witnesses --------------------------

per_mode! {
    /// W-C2 (R/I81/b1_probe_01 PROBE §4; the facade tests' `w_c2()`): three cases, A and C in A. A
    /// successor at R/16, and again at R/64 = 1 MiB as headroom.
    witness_w_c2_publishes, |mode| {
        for stack in [WITNESS_STACK, RESERVED_STACK_BYTES / 64] {
            let raw = crate::retained_facade_tests::w_c2();
            let ran = witness(&format!("W-C2 {mode:?}"), stack, move || permitted_work(raw, mode, |_| {}));
            assert_eq!(ran, Ran::Successor, "W-C2 {mode:?} at {stack} B");
        }
    }
}

/// W-C2's (A, C) (RV109 round 2 N-3: it publishes through the registered Direct entry in sparse):
/// a successor at R/16. Sparse only.
#[test]
#[ignore]
fn witness_w_c2_ac_publishes_sparse() {
    let mode = PreviewSolverMode::SparseInteractive;
    let ran = witness(&format!("W-C2 (A, C) {mode:?}"), WITNESS_STACK, move || permitted_work(inputs::w_c2_ac(), mode, |_| {}));
    assert_eq!(ran, Ran::Successor, "W-C2 (A, C) {mode:?}");
}

per_mode! {
    /// SW item 2's publishing cap-maximal input (I86 `c1`; RR "I86's SW probe accepted; …", ruling 2):
    /// 7 milestone copies and 4 filler bodies at D1's count caps, 128 moments, one case. A successor.
    witness_c1_cap_maximal_publishes, |mode| {
        let raw = inputs::c1();
        assert_eq!(sha256_of(&raw), inputs::C1_SHA256, "I86's c1");
        let ran = witness(&format!("c1 {mode:?}"), WITNESS_STACK, move || permitted_work(raw, mode, |_| {}));
        assert_eq!(ran, Ran::Successor, "c1 {mode:?}");
    }
}

per_mode! {
    /// The cap-maximal three-case input (I86 `i3_c1_three_case`; |A| = 3): c1's model with cases A,
    /// B and C of 128 moments each (Σ l_i = 384 = L), every provenance escaped, a raw value of
    /// depth 16. The private driver skips admission, so `assess` asserts it is inside D1 first
    /// (RV107 N-7). Its outcome is asserted.
    witness_i3_three_case, |mode| {
        let raw = inputs::i3_three_case();
        assert_eq!(sha256_of(&raw), inputs::I3_THREE_CASE_SHA256, "I86's i3_c1_three_case");
        let (request, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
        let report = super::super::assess(&capture, &request, super::super::Entry::Direct);
        assert_eq!(report.law().domain, None, "inside D1");
        assert_eq!(report.raw.maximum_depth, 16, "raw depth 16");
        assert_eq!((report.typed.load_cases.length, report.law().nested.total_loads), (3, 384), "C cases, Σ l_i = L");
        let ran = witness(&format!("i3 three-case {mode:?}"), WITNESS_STACK, move || permitted_work(raw, mode, |_| {}));
        assert_eq!(ran, Ran::Successor, "i3 three-case {mode:?}");
    }
}

/// B1 SQ: the shared inputs are I86's (their `input_sha` pins) and equal the committed helpers
/// they transcribe. Not a witness; no solve.
#[test]
fn b1_sq_inputs_are_i86s_and_the_committed_helpers() {
    assert_eq!(inputs::milestone(), milestone());
    assert_eq!(inputs::law_cap_maximal(), super::law_tests::cap_maximal(), "law_tests::cap_maximal");
    assert_eq!(inputs::w2b(), w2b_input(), "W2b's input");
    assert_eq!(inputs::W2B_SHA256, W2B_INPUT_SHA256);
    assert_eq!(sha256_of(&inputs::w2b()), W2B_INPUT_SHA256, "W2b's PROBE pin");
    let mut w2 = super::law_tests::cap_maximal();
    escape_every_provenance(&mut w2);
    w2["model"]["unknown_depth_witness"] = depth_16_value();
    assert_eq!(inputs::w2(), w2, "W2's input");
    assert_eq!(inputs::w_c2(), crate::retained_facade_tests::w_c2(), "W-C2");
    let mut ac = crate::retained_facade_tests::w_c2();
    ac["model"]["load_cases"].as_array_mut().unwrap().remove(1);
    assert_eq!(inputs::w_c2_ac(), ac, "W-C2's (A, C)");
    assert_eq!(sha256_of(&inputs::b2_k1e3()), inputs::B2_K1E3_SHA256, "b2_k1e3");
    assert_eq!(sha256_of(&inputs::c1()), inputs::C1_SHA256, "c1");
    for (id, pin) in inputs::I3_CASE_SHA256 {
        assert_eq!(sha256_of(&inputs::i3_case(id)), pin, "i3 {id}");
    }
    assert_eq!(sha256_of(&inputs::i3_three_case()), inputs::I3_THREE_CASE_SHA256, "i3 three-case");
    assert_eq!(inputs::depth_16_value(), depth_16_value());
}

// ---- B1 SQ (PLAN_v2 §3.6): RSS_TIME.md's controls in this build ------------------------------

/// The ordinary value route alone on the witness stack (W1's increment is the witness's peak
/// over this one), for each RSS_TIME input.
fn ordinary_only(label: &str, raw: Value, mode: PreviewSolverMode) {
    let work = move || {
        let start = std::time::Instant::now();
        let envelope = crate::run_linear_static_preview_value_with_mode(raw, mode).unwrap();
        (start.elapsed().as_secs_f64() * 1e3, envelope.results.len())
    };
    let (ms, rows) = crate::on_reserved_stack(WITNESS_STACK, work).unwrap_or_else(|| panic!("{label}: the reserved thread did not spawn"));
    println!("I104_SQ_CONTROL ordinary {label} {mode:?} rows={rows} ordinary_ms={ms:.1}");
}
macro_rules! control_ordinary {
    ($($name:ident: $input:expr;)+) => {
        $(per_mode! { $name, |mode| { ordinary_only(stringify!($name), $input, mode); } })+
    };
}
control_ordinary! {
    control_ordinary_milestone: milestone();
    control_ordinary_w_c2: crate::retained_facade_tests::w_c2();
    control_ordinary_w2: inputs::w2();
    control_ordinary_b2_k1e3: inputs::b2_k1e3();
    control_ordinary_c1: inputs::c1();
    control_ordinary_i3_three_case: inputs::i3_three_case();
}
/// The process floor in this build: no product call.
#[test]
#[ignore]
fn control_process_floor() {
    println!("I104_SQ_CONTROL process_floor");
}

// ---- B1 SQ (RR "RV115 confirms S-4 (a)'s soundness …", NC-1): DEF-O's availability, report only --

/// The DEF-O row classes RV115 names: `mm_to_si`, the rows DEF-O projects from mm to SI by a
/// second rounding (`ProductUnit::Millimetre`: the three displacement components and the
/// displacement magnitude); `support_magnitude`, the support force and moment magnitudes (the
/// nested hypot); `other`, every other row.
fn def_o_class(kind: &str) -> &'static str {
    match kind {
        "global_nodal_displacement_x" | "global_nodal_displacement_y" | "global_nodal_displacement_z" | "displacement_magnitude" => "mm_to_si",
        "support_reaction_force_magnitude_v2" | "support_reaction_moment_magnitude_v2" => "support_magnitude",
        _ => "other",
    }
}
/// The private transaction to T-9 (custody, preparation, the batch call, the freezes) on the
/// witness stack, as `w1_transaction` runs it. Then, per case in A: its end; its verdicts' failing
/// rows by predicate and class; and, for a Candidate fallback, its typed cause and the class of its
/// cause row (`certify_final` reports the first failing verdict). Prints only: no assertion on the
/// shares, which ROOT reads (NC-1).
fn def_o_report(label: &'static str, raw: Value, mode: PreviewSolverMode) {
    use crate::retained_product::{AttemptEnd, PreparedCandidateError};
    use std::collections::BTreeMap;
    let work = move || {
        let (request, capture) = CapturedInvocation::parse(raw, mode).expect("a valid request");
        let mut observer = crate::retained_product::ProductCapture::prepared_probe();
        let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut SourceRecoveryBudget::default(), Some(&mut observer));
        if ordinary.source_block_recovery.is_some() {
            return println!("I104_SQ_DEF_O {label} {mode:?} end=exact_selected");
        }
        let cases = crate::w1_case_ids(&capture).expect("inside D1.4");
        let ids: Vec<String> = cases.ids().iter().map(|id| id.to_string()).collect();
        let attempted = cases.attempted(crate::case_triggers(&ordinary.numerical_quality, &observer.ordinary, cases.ids()));
        if attempted.requests().is_empty() {
            return println!("I104_SQ_DEF_O {label} {mode:?} end=no_triggered_case");
        }
        let mut prepared = match observer.prepare_cases(ordinary, ids.len(), attempted.requests()) {
            Ok(prepared) => prepared,
            Err(failure) => return println!("I104_SQ_DEF_O {label} {mode:?} end=custody error={:?}", failure.error),
        };
        prepared.native();
        prepared.freeze();
        for i in 0..prepared.attempts.len() {
            let request = prepared.attempts[i].request;
            let case_id = &ids[request];
            let verdicts = prepared.capture.with_case(request, |c| c.verdicts.clone());
            let rows: Vec<&crate::ResultItem> = prepared.ordinary.results.iter()
                .filter(|r| r.basis_ref.as_ref().is_some_and(|b| b.ref_type == "load_case" && &b.ref_id == case_id))
                .collect();
            let kind = |row: usize| rows.get(row).map_or("?", |r| r.kind.as_str());
            let mut tally: BTreeMap<(String, &'static str), usize> = BTreeMap::new();
            for v in verdicts.iter().filter(|v| !v.passed) {
                *tally.entry((format!("{:?}", v.failed), def_o_class(kind(v.row)))).or_default() += 1;
            }
            let classes: BTreeMap<&'static str, usize> = verdicts.iter().fold(BTreeMap::new(), |mut m, v| {
                *m.entry(def_o_class(kind(v.row))).or_default() += 1;
                m
            });
            let (end, cause) = match &prepared.attempts[i].end {
                AttemptEnd::Frozen(_) => ("frozen", String::new()),
                AttemptEnd::Native => ("native", String::new()),
                AttemptEnd::Preparation => ("preparation", String::new()),
                AttemptEnd::Candidate(refused) => ("candidate", match &refused.error {
                    PreparedCandidateError::Proof(failure) => format!("Proof({:?})", failure.failure().typed_cause(&mut open_pipe_stress_frame_kernel::structural::retained_api::TraceCopyWork::default())),
                    PreparedCandidateError::Values { .. } => "Values".to_owned(),
                    PreparedCandidateError::Abandoned { cause, .. } => format!("Abandoned({cause:?})"),
                    PreparedCandidateError::Capture(e) => format!("Capture({e:?})"),
                    PreparedCandidateError::Numeric => "Numeric".to_owned(),
                    PreparedCandidateError::Observable => "Observable".to_owned(),
                    PreparedCandidateError::G5a => "G5a".to_owned(),
                }),
                _ => ("other", String::new()),
            };
            let first = verdicts.iter().find(|v| !v.passed)
                .map(|v| format!("row={} kind={} class={} failed={:?} predicates={:?}", v.row, kind(v.row), def_o_class(kind(v.row)), v.failed, v.predicates));
            println!("I104_SQ_DEF_O {label} {mode:?} case={case_id} end={end} rows={} verdicts={} by_class={classes:?} failing={} tally={tally:?} first_failing={first:?} cause={cause}",
                rows.len(), verdicts.len(), verdicts.iter().filter(|v| !v.passed).count());
        }
    };
    crate::on_reserved_stack(WITNESS_STACK, work).unwrap_or_else(|| panic!("{label}: the reserved thread did not spawn"));
}
macro_rules! def_o {
    ($($name:ident: $input:expr;)+) => {
        $(per_mode! { $name, |mode| { def_o_report(stringify!($name), $input, mode); } })+
    };
}
def_o! {
    def_o_b2_k1e3: inputs::b2_k1e3();
    def_o_c1: inputs::c1();
    def_o_i3_three_case: inputs::i3_three_case();
    def_o_w2: inputs::w2();
    def_o_w_c2: crate::retained_facade_tests::w_c2();
}
