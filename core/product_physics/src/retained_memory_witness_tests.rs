//! U4 G5 part 2: the S1 stack witnesses W1–W7 (STACK_PLAN.md §4; STACK_INVENTORY.md §3).
//!
//! Each witness runs, on one thread with R/k = 4 MiB of reserved stack (the
//! `on_reserved_stack` the permitted dispatch uses), the work the permitted path
//! runs there: the single observed ordinary run with capture installed, then the
//! W1 phases (`retained_w1`). The private driver enters `retained_w1` directly, as the
//! facade tests do. W3 first asserts genuine G0 admission of its E counterpart; it does not
//! pass that permit into the private measured work. G-B and G-C are not
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

#[path = "../tests/common/w1_member_load_inputs.rs"]
mod w1_member_load_inputs;

/// The owner-adopted per-invocation default, 12 GiB: the registered profile's qualified M.
const RETAINED_LIMIT: RetainedByteLimit = RetainedByteLimit::new(12_884_901_888);
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
    // SQ2 (B3b-P, B3-D P-1 and P-2): the W1 route and its exact-block budget, as `permitted_run`
    // decides them (the preview route's are the default observer and budget, as before).
    let route = crate::w1_route(&request.model).expect("a model on a W1 route");
    let mut observer = crate::retained_product::ProductCapture::prepared_probe_on(route);
    let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::w1_budget(route), Some(&mut observer));
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
        // The depth-16 raw-only value rides on the declared, solve-ignored `diagnostics` member
        // (the closed model root refuses an unknown member; see `b1_sq_inputs::stressed`).
        raw["model"]["diagnostics"] = depth_16_value();
        let (request, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
        let report = super::super::assess(&capture, &request, super::super::Entry::Direct, RETAINED_LIMIT);
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
/// The PROBE pins (R/I81/b1_probe_01, `input_sha` lines) of W2b's and W6's PHYS-R4 inputs, each
/// moved with its version-only re-authoring (T4-S1: 0.6.0; W-13 S1b: 0.7.0; W-15 slice c: 0.8.0; `w6_input` changes nothing else).
const W2B_INPUT_SHA256: &str = "4bbbe484c3f97a9c067e86658147814888dba3426c2a6b29870bf03066d44a02";
const W6_PHYS_R4_INPUT_SHA256: &str = "04e737865bb3421ca081dd565b1cead79e553585f2449340b489823602cd8641";

per_mode! {
    /// W3: an in-domain request whose legacy exact recovery selects (the X branch: T25's
    /// requested() calls, the commitment, the publication and body hashes).
    witness_w3_exact_selected, |mode| {
        let file = match mode {
            PreviewSolverMode::SparseInteractive => include_str!("../../../fixtures/product_preview/w1_controls/n05-sparse_interactive.request.json"),
            _ => include_str!("../../../fixtures/product_preview/w1_controls/n05-dense_scrutiny.request.json"),
        };
        use sha2::Digest;
        // Selected source blocks require admitted E. Keep N05's physical
        // request frozen, and exercise the governed E counterpart.
        assert_eq!(format!("{:x}", sha2::Sha256::digest(file.as_bytes())),
            "c656d9a8d257fc968d0cda469a0277a7b2fba7463a52586eba977f3a2836dc3d");
        let original: Value = serde_json::from_str(file).unwrap();
        assert_eq!(original["model"]["schema_version"], json!("0.9.0"));
        let (original_request, _) = CapturedInvocation::parse(original.clone(), mode).unwrap();
        assert_eq!(namespace_branch(&original_request.model), Ok(NamespaceBranch::Preview));
        let original_control = original.clone();
        assert_eq!(witness(&format!("W3 original L {mode:?}"), WITNESS_STACK,
            move || permitted_work(original_control, mode, |_| {})), Ran::Successor);
        // B7b D3 re-authored the E counterpart once from the retired 0.3.0/v2 declaration to the
        // current one (D4 (a)): the same 0.9.0 document with `3.0.0/exact_pressure_v3`, shear
        // model Euler-Bernoulli and an explicit zero corrosion allowance on each pipe section.
        let mut raw = original.clone();
        raw["model"]["pressure_contract"] = json!({"version":"3.0.0","mode":"exact_pressure_v3","shear_model":"euler_bernoulli"});
        for pipe in raw["model"]["pipe_segments"].as_array_mut().unwrap() {
            assert!(pipe["section"].get("corrosion_allowance").is_none(), "N05 declares no allowance");
            pipe["section"]["corrosion_allowance"] = json!({"value":0.0,"unit":"m"});
        }
        for material in raw["model"]["materials"].as_array_mut().unwrap() {
            let e = material["elastic_modulus"]["value"].as_f64().unwrap();
            let g = material["shear_modulus"]["value"].as_f64().unwrap();
            let nu = e / (2.0 * g) - 1.0;
            assert_eq!(nu.to_bits(), 0.25f64.to_bits());
            material["constitutive_basis"] = json!("homogeneous_isotropic_E_nu_v1");
            material["poisson_ratio"] = json!({"value":nu,"unit":"1"});
        }
        for case in raw["model"]["load_cases"].as_array_mut().unwrap() {
            case["pressure_regions"] = json!([]);
        }
        let physical = |mut value: Value| {
            let model = value["model"].as_object_mut().unwrap();
            model.remove("schema_version");
            model.remove("pressure_contract");
            for pipe in model["pipe_segments"].as_array_mut().unwrap() {
                pipe["section"].as_object_mut().unwrap().remove("corrosion_allowance");
            }
            for material in model["materials"].as_array_mut().unwrap() {
                material.as_object_mut().unwrap().remove("constitutive_basis");
                material.as_object_mut().unwrap().remove("poisson_ratio");
            }
            for case in model["load_cases"].as_array_mut().unwrap() {
                case.as_object_mut().unwrap().remove("pressure_regions");
            }
            value
        };
        assert_eq!(raw["model"]["schema_version"], original["model"]["schema_version"], "the current document version stays");
        assert_eq!(physical(raw.clone()), physical(original.clone()),
            "only the exact declaration, the zero allowance and the analytically equivalent material basis move");
        let (request, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
        let mut normalized = request.model.materials.clone();
        let mut material_diagnostics = Vec::new();
        crate::pressure_material::resolve_base(&request.model, &mut normalized, &mut material_diagnostics);
        assert!(!material_diagnostics.iter().any(|d| d.severity == "blocking"));
        assert_eq!(normalized.len(), original_request.model.materials.len());
        for (actual, frozen) in normalized.iter().zip(&original_request.model.materials) {
            assert_eq!(actual.id, frozen.id);
            assert_eq!(actual.elastic_modulus.value.to_bits(), frozen.elastic_modulus.value.to_bits());
            assert_eq!(actual.shear_modulus.as_ref().unwrap().value.to_bits(),
                frozen.shear_modulus.as_ref().unwrap().value.to_bits());
        }
        assert_eq!(namespace_branch(&request.model), Ok(NamespaceBranch::Exact));
        assert_eq!(crate::w1_route(&request.model), Some(crate::retained_product::W1Route::Exact));
        let (_permit, report) = admit(&capture, &request, Entry::Direct, RETAINED_LIMIT)
            .unwrap_or_else(|r| panic!("E counterpart refused: {:?}", r.law().refusal));
        assert_eq!(report.law().domain, None);
        assert_eq!(report.law().registered, Some(0), "the genuine audited profile grants G0");
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
        "schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
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
        let report = super::super::assess(&capture, &request, super::super::Entry::Direct, RETAINED_LIMIT);
        assert_eq!(report.law().domain, None, "inside D1");
        assert_eq!(report.raw.maximum_depth, 16, "raw depth 16");
        assert_eq!((report.typed.load_cases.length, report.law().nested.total_loads), (3, 384), "C cases, Σ l_i = L");
        let ran = witness(&format!("i3 three-case {mode:?}"), WITNESS_STACK, move || permitted_work(raw, mode, |_| {}));
        assert_eq!(ran, Ran::Successor, "i3 three-case {mode:?}");
    }
}

/// The literal member-only cap, with distinct named E/G custody on every case.
/// These checks inspect the actual observed owners, not a relabeled nodal input.
fn member_cap_capture(capture: &crate::retained_product::ProductCapture, moduli: [(f64, f64); 3]) {
    use w1_member_load_inputs as member;
    assert!(capture.error.is_none() && capture.basis_error.is_none());
    assert_eq!(capture.bases.len(), member::CASES);
    assert_eq!(capture.member_sources.as_ref().unwrap().len(), member::CASES);
    for case in 0..member::CASES {
        assert_eq!(capture.member_loads_basis_at(case), (true, true, Some(case)));
        let source = capture.member_source_at(case).unwrap();
        assert_eq!((source.case_index, source.terms.len(), source.loads.loads().len()), (case, 128, 128));
        assert!(capture.source_terminal_at(case).0);
        assert_eq!(capture.source_moduli_at(case), Some(vec![moduli[case]]));
    }
}

per_mode! {
    /// W1 stage 2: three cases, three named bases and exactly 128 member terms
    /// per case, on the 4 MiB witness stack. Geometry is the narrow two-node,
    /// one-member control; the separate law input retains all geometry caps.
    /// Every case reaches the staged selected state. Its body-0 translation and
    /// rotation scales formerly differed by one ulp between the native candidate
    /// and the final rows, so G5b refused it; since the W1 coverage repair's
    /// corner 2 (Design `w1-coverage-repair.md` §2.2, §3.5) the receipt states the
    /// final scales the gate used, every row's gate verdict passes under them, and
    /// the successor publishes.
    witness_w1_member_load_three_case_three_basis_128, |mode| {
        let raw = w1_member_load_inputs::narrow_three_basis_member_cases();
        inside_d1("W1 member-only c=3/bases=3/l=128", &raw, mode, 3, 0, false);
        let (request, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
        let report = super::assess(&capture, &request, super::Entry::Direct, RETAINED_LIMIT);
        assert_eq!((report.typed.nodes.length, report.typed.members.length), (2, 1));
        assert_eq!((report.typed.load_cases.length, report.law().nested.primitive_loads.length,
            report.law().nested.total_loads), (3, 128, 384));
        let ran = witness(&format!("W1 member-only c=3/bases=3/l=128 {mode:?}"), WITNESS_STACK, move || {
            let (ran, _, staged) = crate::retained_tests_hooks::counted_with_successor(||
                permitted_work(raw.clone(), mode, |capture| member_cap_capture(capture,
                    std::array::from_fn(|case| (2.0e11 * w1_member_load_inputs::BASIS_SCALES[case],
                        8.0e10 * w1_member_load_inputs::BASIS_SCALES[case])))));
            let staged = staged.expect("all three member cases reach actual wire/precommit");
            let body = &staged["retained_precision"]["body"];
            assert_eq!(body["cases"].as_array().unwrap().len(), 3);
            assert_eq!(body["sources"].as_array().unwrap().len(), 3);
            assert_eq!(body["material_bases"].as_array().unwrap().len(), 3);
            assert_eq!(body["product_attempts"].as_array().unwrap().len(), 3);
            for case in 0..3 {
                assert_eq!(body["cases"][case]["status"], "selected", "case {case} reaches selected native/proof/wire");
                let source = body["sources"].as_array().unwrap().iter()
                    .find(|source| source["owner"]["case_index"] == case).unwrap();
                assert_eq!(source["member_load_terms"].as_array().unwrap().len(), 128);
                let basis = source["material_basis_ref"].as_u64().unwrap() as usize;
                assert_eq!(body["material_bases"][basis]["selector"],
                    json!({"kind":"named", "id":w1_member_load_inputs::basis_id(case)}));
                for term in 0..128 {
                    let id = &raw["model"]["load_cases"][case]["primitive_loads"][term]["id"];
                    assert!(source["member_load_terms"].as_array().unwrap().iter()
                        .any(|declared| &declared["source_id"] == id && declared["intensity"] == json!([
                            format!("{:016x}", raw["model"]["load_cases"][case]["primitive_loads"][term]["magnitude"]["value"].as_f64().unwrap().to_bits()),
                            "0000000000000000", "0000000000000000"])));
                }
            }
            // The expectation is the readers' verdict on the staged bytes, not a census
            // observation: G5b recomputes each body's scales from the final rows and the
            // gates re-check every row under the receipt's scales.
            let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
            let verdict = open_pipe_stress_result_export::retained_precision::validate(&staged, Some(&invocation))
                .expect("actual Rust G0-G8 consumer of all 384 member terms under the final scales");
            assert!(verdict.invocation_bound && verdict.numerical_eligible);
            println!("W1_MEMBER_CAP_STAGED {mode:?} selected=3 bases=3 terms=384 full_reader=true");
            ran
        });
        println!("W1_MEMBER_CAP_ACTUAL {mode:?} input_sha256={} ran={ran:?}", sha256_of(&w1_member_load_inputs::narrow_three_basis_member_cases()));
        assert_eq!(ran, Ran::Successor, "{mode:?}: the receipt states the gate's final scales and the unchanged reader admits it");
    }
}

per_mode! {
    /// The power-of-two axial variant exercises full G8 consumption of all
    /// three member-only cases, still on the reserved 4 MiB stack.
    witness_w1_member_load_three_case_three_basis_128_dyadic, |mode| {
        let raw = w1_member_load_inputs::narrow_dyadic_three_basis_member_cases();
        inside_d1("W1 dyadic member-only c=3/bases=3/l=128", &raw, mode, 3, 0, false);
        let ran = witness(&format!("W1 dyadic member-only c=3/bases=3/l=128 {mode:?}"), WITNESS_STACK, move || {
            let (ran, _, staged) = crate::retained_tests_hooks::counted_with_successor(||
                permitted_work(raw.clone(), mode, |capture| member_cap_capture(capture,
                    std::array::from_fn(|case| (2f64.powi(30 + case as i32), 2f64.powi(29 + case as i32))))));
            let staged = staged.expect("all dyadic member cases reach actual wire/precommit");
            let body = &staged["retained_precision"]["body"];
            assert_eq!(body["cases"].as_array().unwrap().len(), 3);
            assert_eq!(body["sources"].as_array().unwrap().len(), 3);
            assert_eq!(body["material_bases"].as_array().unwrap().len(), 3);
            assert_eq!(body["product_attempts"].as_array().unwrap().len(), 3);
            for case in 0..3 {
                assert_eq!(body["cases"][case]["status"], "selected");
                let source = body["sources"].as_array().unwrap().iter()
                    .find(|source| source["owner"]["case_index"] == case).unwrap();
                assert_eq!(source["member_load_terms"].as_array().unwrap().len(), 128);
                let basis = source["material_basis_ref"].as_u64().unwrap() as usize;
                assert_eq!(body["material_bases"][basis]["selector"],
                    json!({"kind":"named", "id":w1_member_load_inputs::basis_id(case)}));
                for term in 0..128 {
                    let authored = &raw["model"]["load_cases"][case]["primitive_loads"][term];
                    assert!(source["member_load_terms"].as_array().unwrap().iter().any(|declared|
                        declared["source_id"] == authored["id"] && declared["intensity"] == json!([
                            format!("{:016x}", authored["magnitude"]["value"].as_f64().unwrap().to_bits()),
                            "0000000000000000", "0000000000000000"])));
                }
            }
            println!("W1_MEMBER_CAP_DYADIC_INPUT {mode:?} input_sha256={}", sha256_of(&raw));
            let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
            let verdict = open_pipe_stress_result_export::retained_precision::validate(&staged, Some(&invocation))
                .expect("actual Rust G0-G8 consumer of all 384 member terms");
            assert!(verdict.invocation_bound && verdict.numerical_eligible);
            assert_eq!(ran, Ran::Successor, "only a successful private witness is exported");
            if let Some(root) = std::env::var_os("W1_MEMBER_CAP_OUT") {
                let root = std::path::PathBuf::from(root);
                std::fs::create_dir_all(&root).unwrap();
                let id = format!("w1_member_cap_{}", mode.as_str());
                let record = json!({"id":id, "source":staged, "invocation":invocation,
                    "provenance":"private_w1_stack_witness"});
                std::fs::write(root.join(format!("{id}.json")), serde_json::to_vec_pretty(&record).unwrap()).unwrap();
            }
            println!("W1_MEMBER_CAP_DYADIC {mode:?} selected=3 bases=3 terms=384 full_reader=true");
            ran
        });
        assert_eq!(ran, Ran::Successor, "{mode:?}: dyadic member-only cap witness");
    }
}

// ---- SQ2 (B2 combinations, B3b's exact route): the B2/B3 witnesses --------------------------

/// The input is inside D1 with `cases` load cases and `combinations` combinations (the private
/// driver skips admission; RV107 N-7), on the expected W1 route.
fn inside_d1(label: &str, raw: &Value, mode: PreviewSolverMode, cases: usize, combinations: usize, exact: bool) {
    let (request, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
    let report = super::assess(&capture, &request, super::Entry::Direct, RETAINED_LIMIT);
    assert_eq!(report.law().domain, None, "{label}: inside D1");
    assert_eq!((request.model.load_cases.len(), request.model.combinations.len()), (cases, combinations), "{label}: c and z");
    let branch = if exact { NamespaceBranch::Exact } else { NamespaceBranch::Preview };
    assert_eq!(namespace_branch(&request.model), Ok(branch), "{label}: the W1 route");
}

per_mode! {
    /// W-CB1 (I98 `r7_cb1_halfb`): the cap-maximal C_eq = 3 input, SW's cap-maximal cases A and B
    /// (128 moments each, every provenance escaped, a raw value of depth 16) and 1·A + 0.5·B:
    /// route L's priced worst shape but for z (c = 2, z = 1). A successor.
    witness_w_cb1, |mode| {
        let raw = inputs::w_cb1();
        inside_d1("W-CB1", &raw, mode, 2, 1, false);
        let ran = witness(&format!("W-CB1 {mode:?}"), WITNESS_STACK, move || permitted_work(raw, mode, |_| {}));
        assert_eq!(ran, Ran::Successor, "W-CB1 {mode:?}");
    }
}

per_mode! {
    /// W-CB2 (I98 `r7_cb2`): W-C2's two-body cases A and B, and A + B (its combination is
    /// `retained_unavailable`; case A publishes). A successor.
    witness_w_cb2, |mode| {
        let raw = inputs::w_cb2();
        inside_d1("W-CB2", &raw, mode, 2, 1, false);
        let ran = witness(&format!("W-CB2 {mode:?}"), WITNESS_STACK, move || permitted_work(raw, mode, |_| {}));
        assert_eq!(ran, Ran::Successor, "W-CB2 {mode:?}");
    }
}

per_mode! {
    /// `b2_c1_range_mechanics`: the milestone with `[range(case), 2·case]` (c = 1, z = 2). A successor.
    witness_b2_c1_range_mechanics, |mode| {
        let raw = inputs::b2_c1_range_mechanics();
        inside_d1("b2_c1_range_mechanics", &raw, mode, 1, 2, false);
        let ran = witness(&format!("b2_c1_range_mechanics {mode:?}"), WITNESS_STACK, move || permitted_work(raw, mode, |_| {}));
        assert_eq!(ran, Ran::Successor, "b2_c1_range_mechanics {mode:?}");
    }
}

per_mode! {
    /// The exact successor (B3-W's `m3x`): the milestone authored as exact (B7b: 0.9.0 with the
    /// v3 contract and a zero allowance). A successor.
    witness_m3x, |mode| {
        let raw = inputs::m3x();
        inside_d1("m3x", &raw, mode, 1, 0, true);
        let ran = witness(&format!("m3x {mode:?}"), WITNESS_STACK, move || permitted_work(raw, mode, |_| {}));
        assert_eq!(ran, Ran::Successor, "m3x {mode:?}");
    }
}

per_mode! {
    /// The cap-maximal exact input: the cap-maximal three-case input (`i3_three_case`) authored as
    /// exact (B7b: 0.9.0, v3), route E's priced shape (c = 3, z = 0) at the count caps.
    witness_exact_cap_maximal, |mode| {
        let raw = inputs::exact_cap_maximal();
        inside_d1("exact cap-maximal", &raw, mode, 3, 0, true);
        let ran = witness(&format!("exact cap-maximal {mode:?}"), WITNESS_STACK, move || permitted_work(raw, mode, |_| {}));
        assert_eq!(ran, Ran::Successor, "exact cap-maximal {mode:?}");
    }
}

/// SQ2: the B2/B3 shared inputs equal the facade tests' builders they transcribe. No solve.
#[test]
fn sq2_inputs_are_the_committed_helpers() {
    assert_eq!(inputs::w_cb1(), crate::retained_facade_tests::w_cb1(), "W-CB1");
    assert_eq!(inputs::w_cb2(), crate::retained_facade_tests::w_cb2(), "W-CB2");
    assert_eq!(inputs::b2_c1_range_mechanics(), crate::retained_facade_tests::b2_c1_range_mechanics(), "b2_c1_range_mechanics");
    assert_eq!(inputs::m3x(), crate::retained_facade_tests::m3x(), "m3x");
    let exact = inputs::exact_cap_maximal();
    assert_eq!(exact["model"], inputs::exact3(inputs::i3_three_case())["model"], "the cap-maximal exact model is i3's, authored as exact");
    assert_eq!(exact["materials"], exact["model"]["materials"], "the request-level materials take the same basis");
}

// ---- T3 F1: G-C's contract-evidence bounds per W1 route ----------------------------------------

/// The G-C observations of `raw`'s ordinary run, observed on its own W1 route as `permitted_run`
/// observes it, and the run's envelope.
fn complete_facts_on_route(raw: Value, mode: PreviewSolverMode) -> ([PhaseObservation; COMPLETE_FACTS], crate::MechanicsEnvelope, crate::retained_product::W1Route) {
    let (request, capture) = CapturedInvocation::parse(raw, mode).unwrap();
    let requested_cases = request.model.load_cases.len();
    let route = crate::w1_route(&request.model).unwrap();
    let mut observer = crate::retained_product::ProductCapture::prepared_probe_on(route);
    let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::w1_budget(route), Some(&mut observer));
    (complete_observations(&CompleteFacts { ordinary: &ordinary, capture: &observer, requested_cases }), ordinary, route)
}

/// T3 F1 (SQ2's G-C finding): route E's G-C bounds. Route L's bounds are `phase_caps()`, whose
/// entries row is cut from C·553 = 1,659 to 1,654 so that objects + ⌊entries / 5⌋ stays within the
/// 531 BTree nodes the profile prices. Route E's differ in the result rows and their capacity,
/// C_eq·P_final at route E's own P_final, now 7n + 114m + 8g + 3 (T4-U6, D-5: then 72m, 8,361 and
/// 16,384 at the caps, against route L's 6,345 and 8,192; W-7, §8.20: 12,393 and 16,384 against
/// route L's 8,937 and 16,384), and in the five contract-evidence rows, which are C
/// times route E's per-case facts (the componentwise maximum of the preview tree and the D-5
/// evidence census at the D1 caps, each source-selected extremum at the maximum over its three
/// `locations` shapes, U6-RL), the facts the registered profile prices for the evidence owner
/// (retained-pricing-rules.md, "Gate bounds (registration)"). At (n, m, g) = (32, 32, 32) and
/// C = 3, per case: arrays max(160, 3,118), objects max(67, 868), entries max(553, 4,093), string
/// bytes max(66,816, 414,656), key bytes max(22,120, 43,631); the entries bound is cut to 12,274 so
/// that objects + ⌊entries / 5⌋ ≤ 3·(868 + ⌊4,093 / 5⌋) = 5,058, the BTree nodes the profile prices.
#[test]
fn f1_g_c_contract_evidence_bounds_are_per_route() {
    use crate::retained_product::W1Route;
    let (l, e) = (phase_caps_on(W1Route::Preview), phase_caps_on(W1Route::Exact));
    assert_eq!((l.late, l.complete), (phase_caps().late, phase_caps().complete), "route L's bounds are phase_caps()");
    // W-7 (retained-pricing-rules.md §8.20): 78 and 114 rows per member (W7-RL, W7-RP).
    assert_eq!(l.complete[..2], [8_937, 16_384], "route L: C_eq times route L's P_final, and its push capacity");
    assert_eq!(e.complete[..2], [12_393, 16_384], "route E: C_eq times route E's P_final, and its push capacity");
    assert_eq!(l.complete[9..14], [480, 201, 1_654, 200_448, 66_360], "route L: C times the preview tree, entries cut to the priced nodes");
    assert_eq!(e.complete[9..14], [9_354, 2_604, 12_274, 1_243_968, 130_893], "route E: C times max(preview tree, D-5 exact census)");
    assert_eq!(e.late, l.late, "G-B is route-independent");
    for i in (0..COMPLETE_FACTS).filter(|i| !(0..2).contains(i) && !(9..14).contains(i)) {
        assert_eq!(e.complete[i], l.complete[i], "row {i} is route-independent");
    }
    // The node count the profile prices (`value_tree`: objects + ⌊entries / 5⌋ per case, times C),
    // reached and not exceeded on each route: 531 on route L, 5,058 on route E.
    for (caps, nodes) in [(l, 3 * (67 + 553 / 5)), (e, 3 * (868 + 4_093 / 5))] {
        assert!(caps.complete[10] + caps.complete[11] / 5 == nodes && caps.complete[10] + (caps.complete[11] + 1) / 5 > nodes, "{nodes}");
    }
    // The profile prices the evidence owner at these facts: route E's chain's "Final aggregation"
    // and "Preview tree final copy (envelope evidence)" terms are 5,058·Node(String,Value)
    // + 9,354·s(Value) + 1,374,861 B, and the arrays and bytes bounds sum to it.
    assert_eq!((e.complete[9], e.complete[12] + e.complete[13]), (9_354, 1_374_861));
    // Each fact admits its bound and refuses one more on route E.
    let (facts, _, _) = complete_facts_on_route(inputs::exact_cap_maximal(), PreviewSolverMode::SparseInteractive);
    let mut at = facts;
    for (o, cap) in at.iter_mut().zip(e.complete) {
        o.observed = cap;
    }
    assert_eq!(check_phase(PhaseGate::Complete, &at, &e.complete), Ok(()));
    for i in 0..COMPLETE_FACTS {
        let mut over = at;
        over[i].observed += 1;
        assert_eq!(check_phase(PhaseGate::Complete, &over, &e.complete).unwrap_err().fact, facts[i].fact);
    }
}

/// T3 F1: the cap-maximal exact input passes G-C on route E, while route L's bounds refuse it: at
/// its result rows first (D-5, U4b and W-7: route E publishes 114m rows per case against route L's 78m,
/// so its results exceed 8,937), and, with route E's result rows, at the evidence's array
/// elements. An exact owner one array element over route E's bound is still refused there, and
/// the milestone keeps route L's bounds.
#[test]
fn f1_g_c_admits_the_cap_maximal_exact_input_and_refuses_over_cap_evidence() {
    use crate::retained_product::W1Route;
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let (facts, ordinary, route) = complete_facts_on_route(inputs::exact_cap_maximal(), mode);
        assert_eq!(route, W1Route::Exact);
        let caps = phase_caps_on(route);
        assert_eq!(check_phase(PhaseGate::Complete, &facts, &caps.complete), Ok(()), "{mode:?}: inside route E's bounds");
        let refusal = check_phase(PhaseGate::Complete, &facts, &phase_caps().complete).unwrap_err();
        assert_eq!((refusal.fact, refusal.cap), (PhaseFact::EnvelopeResults, 8_937), "{mode:?}: route L's bounds refuse it");
        assert!(refusal.observed > 8_937 && refusal.observed <= caps.complete[0], "{mode:?}: {}", refusal.observed);
        let mut l_with_e_rows = phase_caps().complete;
        l_with_e_rows[..2].copy_from_slice(&caps.complete[..2]);
        let refusal = check_phase(PhaseGate::Complete, &facts, &l_with_e_rows).unwrap_err();
        assert_eq!((refusal.fact, refusal.cap), (PhaseFact::ContractEvidenceArrayElements, 480), "{mode:?}: route L's evidence bounds refuse it");
        let arrays = facts.iter().find(|o| o.fact == PhaseFact::ContractEvidenceArrayElements).unwrap().observed;
        assert!(arrays > 480 && arrays <= caps.complete[9], "{mode:?}: {arrays}");
        // Over the bound: the same owner with array slots added up to route E's bound plus one.
        let mut over = ordinary.clone();
        let base = borrowed_value_census(over.contract_evidence.as_ref().unwrap()).array_capacity_elements as u64;
        let extra = (caps.complete[9] + 1 - base) as usize;
        let mut pad = Vec::with_capacity(extra);
        pad.resize(extra, Value::Null);
        over.contract_evidence.as_mut().unwrap().as_object_mut().unwrap().insert("f1_over_cap".into(), Value::Array(pad));
        let census = borrowed_value_census(over.contract_evidence.as_ref().unwrap());
        assert_eq!(census.array_capacity_elements as u64, caps.complete[9] + 1, "{mode:?}");
        let (request, capture) = CapturedInvocation::parse(inputs::exact_cap_maximal(), mode).unwrap();
        let mut observer = crate::retained_product::ProductCapture::prepared_probe_on(route);
        let requested_cases = request.model.load_cases.len();
        let _ = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::w1_budget(route), Some(&mut observer));
        let o = complete_observations(&CompleteFacts { ordinary: &over, capture: &observer, requested_cases });
        let refusal = check_phase(PhaseGate::Complete, &o, &caps.complete).unwrap_err();
        assert_eq!((refusal.fact, refusal.observed, refusal.cap), (PhaseFact::ContractEvidenceArrayElements, caps.complete[9] + 1, caps.complete[9]), "{mode:?}");
    }
    let (facts, _, route) = complete_facts_on_route(milestone(), PreviewSolverMode::SparseInteractive);
    assert_eq!(route, W1Route::Preview);
    assert_eq!(phase_caps_on(route).complete, phase_caps().complete);
    assert_eq!(check_phase(PhaseGate::Complete, &facts, &phase_caps().complete), Ok(()));
}

/// T3 F1 (PR #1235 review R1): the permit's own G-C, `CapturePermit::check_complete`, selects the
/// bounds of the observer's W1 route. One admitted cap-maximal exact invocation, one ordinary owner:
/// with the Exact-route observer that `permitted_run` would build, G-C passes; with a Preview-route
/// observer over the same owner, G-C refuses at the result rows against route L's 7,785 (D-5: the
/// exact route's rows exceed route L's P_final). Ordinary runs only (no W1). In a Stale build there
/// is no permit and nothing to drive.
#[test]
fn f1_check_complete_reads_the_observer_route() {
    use crate::retained_product::{ProductCapture, W1Route};
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let raw = inputs::exact_cap_maximal();
        let (request, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
        let permit = match super::admit(&capture, &request, super::Entry::Direct, RETAINED_LIMIT) {
            Ok((permit, report)) => {
                assert_eq!(report.law().refusal, None, "{mode:?}: admitted");
                permit
            }
            Err(report) => {
                assert_ne!(COMPILED_IDENTITY, Some(REGISTERED_PROFILES[0].identity), "{mode:?}: the registered build admits it: {:?}", report.law());
                println!("F1_ROUTE_GATE_SKIP {mode:?}: Stale build, no permit");
                return;
            }
        };
        let route = crate::w1_route(&request.model).unwrap();
        assert_eq!(route, W1Route::Exact);
        let requested_cases = request.model.load_cases.len();
        let mut exact = ProductCapture::prepared_probe_on(route);
        let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::w1_budget(route), Some(&mut exact));
        assert_eq!(permit.check_complete(&CompleteFacts { ordinary: &ordinary, capture: &exact, requested_cases }), Ok(()), "{mode:?}: route E's bounds");
        let (request, capture) = CapturedInvocation::parse(raw, mode).unwrap();
        let mut preview = ProductCapture::prepared_probe_on(W1Route::Preview);
        let _ = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::w1_budget(W1Route::Preview), Some(&mut preview));
        let refusal = permit.check_complete(&CompleteFacts { ordinary: &ordinary, capture: &preview, requested_cases }).unwrap_err();
        assert_eq!((refusal.gate, refusal.fact, refusal.cap), (PhaseGate::Complete, PhaseFact::EnvelopeResults, 8_937), "{mode:?}: route L's bounds");
        assert!(refusal.observed > 8_937 && refusal.observed <= phase_caps_on(W1Route::Exact).complete[0], "{mode:?}: {}", refusal.observed);
    }
}

// ---- U6-RP (T4-U6/U8): route E's G-C bounds are the priced D-5 census premise -------------------

/// U6-RP and "Gate bounds (registration)" (retained-pricing-rules.md): `P_FINAL_EXACT` and
/// `evidence_caps` are written by hand outside the generated block, so they are tied here to the
/// committed census the pricing tool charges (`tools/retained_pricing/data/census_u4b_7c77bf8072.json`).
/// The tool's chain (`lib/census_patch.py`) charges route E's P_final = 7n + R·m + 8g + 3, with R the
/// census's `rows_per_member` in place of route L's 66, and each case's evidence owner at `_exmax`:
/// per fact, the maximum of the preview tree and rest + (per pipe + per unavailable pipe)·m
/// + (6·per DOF + per node)·n, priced for C cases as s(Value) array slots, objects + ⌊entries / 5⌋
/// Node(String,Value) nodes, and string and key bytes. A change to either side alone fails here;
/// the tool's own premise is tied to the same census by `tests/test_u6_rp_route_e_premise.py`.
/// No solve.
#[test]
fn u6_rp_route_e_gate_caps_are_the_priced_census() {
    use crate::retained_product::W1Route;
    let census: Value = serde_json::from_str(include_str!("../../../tools/retained_pricing/data/census_u4b_7c77bf8072.json")).unwrap();
    let (n, m, g) = (caps::NODES as u64, caps::MEMBERS as u64, caps::SUPPORTS as u64);
    let (c, ceq) = (caps::LOAD_CASES as u64, caps::CASE_EQUIVALENTS as u64);
    assert_eq!(census["caps"], json!({"n": n, "m": m, "g": g}), "the census is taken at D1's count caps");
    // P_final: route E's per-member coefficient is the census's rows per member; the other rows are route L's.
    let rpm = census["rows_per_member"].as_u64().unwrap();
    let rest = 7 * n + 8 * g + 3;
    assert_eq!(P_FINAL, rest + 78 * m, "route L's P_final (U4B-RL, W7-RL: 51 + 15 derived + 12 global end rows)");
    // W7-RP (§8.20): the census's pressure-source-1 fixtures omit the twelve global end rows per member
    // (`global_end_rows_published`) that route E's ordinary envelope carries.
    assert_eq!(P_FINAL_EXACT - rest, (rpm + 12) * m, "route E's per-member coefficient is the census's rows_per_member ({rpm}) + 12 global end rows");
    // The census's measurement: every empty-region exact case publishes 7n + R·m + 8g rows.
    let measured = census["rows"].as_array().unwrap();
    assert!(!measured.is_empty());
    for row in measured {
        let f = |k: &str| row[k].as_u64().unwrap();
        for case in row["empty_region_cases"].as_array().unwrap() {
            let rows = row["rows_by_basis_ref"][case.as_str().unwrap()].as_u64().unwrap();
            assert_eq!(rows, 7 * f("n") + rpm * f("m") + 8 * f("g"), "{} {}", row["fixture"], row["mode"]);
        }
    }
    // G-C's result rows and their capacity: C_eq·P_final at the push capacity the chain prices (U6-RB-V).
    let e = phase_caps_on(W1Route::Exact);
    assert_eq!(e.complete[..2], [ceq * P_FINAL_EXACT, push_capacity(ceq * P_FINAL_EXACT)]);
    // The chain's per-case evidence coefficients (census_patch.py `coef`), from the census's parts.
    const FACTS: [&str; 5] = ["arr", "obj", "ent", "strb", "keyb"];
    let parts = &census["exact_evidence_case_parts"];
    let coef = |k: &str| {
        let p = |part: &str| parts[part][k].as_u64().unwrap();
        (p("rest"), p("per_pipe") + p("per_unavailable_pipe"), 6 * p("per_dof") + p("per_node"))
    };
    // `evidence_caps` is the chain's `_exmax` at every (n, m, g) within D1's caps.
    for nn in 0..=n {
        for mm in 0..=m {
            for gg in 0..=g {
                let (preview, exact) = (evidence_caps(W1Route::Preview, nn, mm, gg), evidence_caps(W1Route::Exact, nn, mm, gg));
                for (i, k) in FACTS.iter().enumerate() {
                    let (r, per_member, per_node) = coef(k);
                    assert_eq!(exact[i], preview[i].max(r + per_member * mm + per_node * nn), "{k} at (n, m, g) = ({nn}, {mm}, {gg})");
                }
            }
        }
    }
    // At the caps: the census's preview-tree facts and its componentwise maximum.
    let (preview, exact) = (evidence_caps(W1Route::Preview, n, m, g), evidence_caps(W1Route::Exact, n, m, g));
    let x = |k: &str| census["componentwise_max"][k].as_u64().unwrap();
    for (i, k) in FACTS.iter().enumerate() {
        assert_eq!(preview[i], census["preview_tree_per_case_at_caps"][k].as_u64().unwrap(), "preview {k}");
        assert_eq!(exact[i], x(k), "exact {k}");
    }
    // G-C's five evidence rows are what the chain charges for C cases.
    assert_eq!([e.complete[9], e.complete[10], e.complete[12], e.complete[13]], [c * x("arr"), c * x("obj"), c * x("strb"), c * x("keyb")]);
    let nodes = c * (x("obj") + x("ent") / 5);
    assert_eq!(e.complete[10] + e.complete[11] / 5, nodes, "the Node(String,Value) count the chain prices");
    assert!(e.complete[10] + (e.complete[11] + 1) / 5 > nodes, "one more entry is a node more than the chain prices");
}

// ---- U6-RL (T4-U6/U8 D3 review R2): source-selected endpoint ties under route E's G-C ----------

/// An exact input whose three cases select the retained source (the separate body S's soft
/// torsional spring makes each case Sensitive) and whose line members tie at their endpoints: L1 is
/// free between the anchored L0 and L2, so a transverse force there bends M0 and M1
/// antisymmetrically, with equal end moments of opposite sign (`endpoint_candidates`). S carries a
/// constant moment (`whole_span_constant`). Every free block has order at most two, the
/// retained-source method's limit; dyadic section and material values keep its work small.
fn source_selected_ties() -> Value {
    let p = "invented_t4_u6u8_reg2_witness_input_no_library_data";
    let section = json!({"outside_diameter": {"value": 0.25, "unit": "m"}, "wall_thickness": {"value": 0.015625, "unit": "m"},
        "corrosion_allowance": {"value": 0.0, "unit": "m"}});
    let material = json!({"id": "mat", "elastic_modulus": {"value": 137_438_953_472.0, "unit": "Pa"},
        "constitutive_basis": "homogeneous_isotropic_E_nu_v1", "poisson_ratio": {"value": 0.25, "unit": "1"}, "provenance": p});
    let node = |id: &str, x: f64, y: f64| json!({"id": id, "position": {"x": x, "y": y, "z": 0.0}, "provenance": p});
    let pipe = |id: &str, from: &str, to: &str| json!({"id": id, "from": from, "to": to, "material": "mat",
        "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "section": section, "provenance": p});
    let support = |id: &str, node: &str, family: &str, restraints: &[&str]| json!({"id": id, "node": node, "family": family,
        "restraints": restraints, "provenance": p});
    let mut spring = support("spring:S0", "S0", "spring", &["RX"]);
    spring["stiffness"] = json!({"dof": "RX", "value": {"value": 1.0 / 16_384.0, "unit": "N*m/rad"}});
    let case = |id: &str, f: f64| json!({"id": id, "label": "U6-RL witness case", "kind": "primitive_user_load", "pressure_regions": [],
        "provenance": p, "primitive_loads": [
            {"id": format!("{id}:force"), "category": "concentrated_force", "target": {"type": "node", "node": "L1"},
                "direction": "UY", "magnitude": {"value": f, "unit": "N"}, "dimension": "force", "provenance": p},
            {"id": format!("{id}:moment"), "category": "concentrated_moment", "target": {"type": "node", "node": "S1"},
                "direction": "RZ", "magnitude": {"value": f, "unit": "N*m"}, "dimension": "moment", "provenance": p}]});
    json!({"materials": [material], "model": {
        "schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "professional_acceptance": "not_provided",
            "rule_check": "not_performed_user_rule_inputs_missing"},
        "project": {"id": "invented:t4-u6u8-reg2:source-selected-ties", "units": {"angle": "rad", "force": "N", "length": "m",
            "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"},
        "nodes": [node("L0", 0.0, 0.0), node("L1", 2.0, 0.0), node("L2", 4.0, 0.0), node("S0", 0.0, 10.0), node("S1", 2.0, 10.0)],
        "pipe_segments": [pipe("M0", "L0", "L1"), pipe("M1", "L1", "L2"), pipe("S", "S0", "S1")],
        "materials": [material],
        "supports": [support("anchor:L0", "L0", "anchor", &["UX", "UY", "UZ", "RX", "RY", "RZ"]),
            support("anchor:L2", "L2", "anchor", &["UX", "UY", "UZ", "RX", "RY", "RZ"]),
            support("anchor:S0", "S0", "anchor", &["UX", "UY", "UZ", "RY", "RZ"]), spring,
            json!({"id": "restraint:L1", "node": "L1", "restraints": ["UZ"], "provenance": p})],
        "load_cases": [case("case:a", 1.0), case("case:b", -2.0), case("case:c", 3.0)], "combinations": []}})
}

/// U6-RL (T4-U6/U8 D3 review R2): route E's contract-evidence bounds admit source-selected endpoint
/// ties. A selected case's maximum (`source_receipt/composite.rs` `maximum_inner`) publishes
/// `locations` in one of three shapes; `endpoint_candidates` has one entry and 31 key bytes more
/// than the `strict_endpoint` shape of the D-5 census fixture, so the census, the chain that prices
/// the evidence owner, and route E's bounds take each extremum at the maximum over the three shapes:
/// per member 127 entries and 1,348 key bytes (the strict shape's 125 and 1,286 plus 2 and 62).
/// A selected run finalizes its source-block receipt, so W1 falls back (`Coexistence`) and G-C's
/// `SourceBlockRecovery` row (cap 0) refuses it; the profile prices its evidence in the X-branch
/// phases at the census facts, with no runtime gate. The evidence rows (status and the five facts)
/// are checked.
/// (a) `source_selected_ties`: its three cases are selected, each line member ties on both bases, a
///     tied member's facts are route E's per-member coefficients, and the evidence is within route
///     E's bounds.
/// (b) The retained-source method's per-case work budget keeps selected inputs small (m = 3 here;
///     larger lines exhaust it), so the near-cap owner is composed from (a)'s own selected cases: in
///     each, M0's tied entries for m members and 32 nodes (node order and the three 6n RHS vectors),
///     C = 3 cases. At m = 23 it is within route E's bounds with a complete census; at m = 24 G-C's
///     borrowed census stops at its 16,384-value limit, which G-C refuses as an incomplete census.
#[test]
fn u6_rl_evidence_bounds_admit_source_selected_endpoint_ties() {
    use crate::retained_product::{ProductCapture, W1Route};
    const EVIDENCE: std::ops::Range<usize> = 8..14;
    let caps = phase_caps_on(W1Route::Exact);
    let within = |o: &[PhaseObservation; COMPLETE_FACTS], label: &str| {
        assert_eq!(o[8].fact, PhaseFact::ContractEvidenceStatus);
        for i in EVIDENCE {
            assert!(o[i].observed <= caps.complete[i], "{label}: {:?} {} above {}", o[i].fact, o[i].observed, caps.complete[i]);
        }
    };
    // Route E's per-member coefficients of the entries and key-byte bounds (one case, n = g = 0).
    let per_member = |i: usize| evidence_caps(W1Route::Exact, 0, 1, 0)[i] - evidence_caps(W1Route::Exact, 0, 0, 0)[i];
    assert_eq!((per_member(2), per_member(4)), (127, 1_348));
    let member = |case: &Value, id: &str| {
        let mut facts = (0u64, 0u64);
        for key in ["pipe_materials", "pipe_sections", "pipe_stress_extrema"] {
            for x in case[key].as_array().unwrap().iter().filter(|x| x["pipe_id"] == id) {
                let c = borrowed_value_census(x);
                facts = (facts.0 + c.object_entries as u64, facts.1 + c.key_bytes as u64);
            }
        }
        facts
    };
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let (request, capture) = CapturedInvocation::parse(source_selected_ties(), mode).unwrap();
        let requested_cases = request.model.load_cases.len();
        let route = crate::w1_route(&request.model).unwrap();
        assert_eq!(route, W1Route::Exact, "{mode:?}");
        let mut observer = ProductCapture::prepared_probe_on(route);
        let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::w1_budget(route), Some(&mut observer));
        assert!(ordinary.source_block_recovery.is_some(), "{mode:?}: the selected receipt finalizes (the X branch)");
        let evidence = ordinary.contract_evidence.clone().expect("exact evidence");
        let cases = evidence["exact_cases"].as_array().unwrap();
        assert_eq!(cases.len(), 3, "{mode:?}: three exact cases");
        for case in cases {
            let extrema = case["pipe_stress_extrema"].as_array().unwrap();
            assert_eq!(extrema.len(), 6, "{mode:?}: three members on both bases");
            assert!(extrema.iter().all(|x| x["basis"] == "retained_source_endpoint_normal_max_v1"), "{mode:?}: source-selected");
            for x in extrema {
                let tie = x["pipe_id"] != "S";
                let kind = if tie { "endpoint_candidates" } else { "whole_span_constant" };
                assert_eq!(x["locations"]["kind"], kind, "{mode:?}: {}", x["pipe_id"]);
                assert_eq!(x["locations"].as_object().unwrap().len(), if tie { 3 } else { 1 }, "{mode:?}");
            }
            // A tied member's facts are route E's per-member coefficients; the constant member's are below.
            assert_eq!(member(case, "M0"), (per_member(2), per_member(4)), "{mode:?}: a tied member");
            assert_eq!(member(case, "M1"), member(case, "M0"), "{mode:?}");
            assert_eq!(member(case, "S"), (123, 1_270), "{mode:?}: a whole-span-constant member");
        }
        // (a) The selected run itself, off W1 at G-C's SourceBlockRecovery row.
        let facts = complete_observations(&CompleteFacts { ordinary: &ordinary, capture: &observer, requested_cases });
        within(&facts, &format!("{mode:?} (a)"));
        let refusal = check_phase(PhaseGate::Complete, &facts, &caps.complete).unwrap_err();
        assert_eq!((refusal.fact, refusal.observed, refusal.cap), (PhaseFact::SourceBlockRecovery, 1, 0), "{mode:?}");
        // (b) The near-cap owner composed from (a)'s cases.
        let composed = |members: usize| {
            let near_cap = |case: &Value| {
                let mut case = case.clone();
                for key in ["pipe_materials", "pipe_sections", "pipe_stress_extrema"] {
                    let own: Vec<Value> = case[key].as_array().unwrap().iter().filter(|x| x["pipe_id"] == "M0").cloned().collect();
                    case[key] = Value::Array((0..members).flat_map(|_| own.iter().cloned()).collect());
                }
                let rhs = &mut case["pressure_rhs_assembly"];
                rhs["node_order"] = Value::Array((0..32).map(|i| json!(format!("node:{i}"))).collect());
                for key in ["assembled_pressure_rhs_global", "rounded_cap_rhs_global", "rounded_poisson_rhs_global"] {
                    let first = rhs[key][0].clone();
                    rhs[key] = Value::Array(vec![first; 6 * 32]);
                }
                case
            };
            let mut evidence = evidence.clone();
            evidence["exact_cases"] = Value::Array(cases.iter().map(near_cap).collect());
            let ties = evidence["exact_cases"].as_array().unwrap().iter()
                .flat_map(|c| c["pipe_stress_extrema"].as_array().unwrap().iter())
                .filter(|x| x["locations"]["kind"] == "endpoint_candidates").count();
            assert_eq!(ties, 3 * members * 2, "{mode:?}: every composed extremum ties");
            let mut near = ordinary.clone();
            near.contract_evidence = Some(evidence);
            complete_observations(&CompleteFacts { ordinary: &near, capture: &observer, requested_cases })
        };
        let o = composed(23);
        println!("U6_RL_NEAR_CAP {mode:?} m=23 {:?}", &o[8..14]);
        within(&o, &format!("{mode:?} (b)"));
        let over = composed(24);
        assert_eq!((over[8].fact, over[8].observed), (PhaseFact::ContractEvidenceStatus, 1), "{mode:?}: the census value limit");
    }
}

per_mode! {
    /// T3 F1 (a): the cap-maximal exact input through the public Direct entry in the registered
    /// build: admitted (branch E), through G-C on route E's bounds, W1 on the exact route, and the
    /// successor RS's precommit validated published. A Stale build refuses at D1.1 instead.
    f1_exact_cap_maximal_direct_publishes, |mode| {
        let raw = inputs::exact_cap_maximal();
        let output = crate::run_linear_static_preview_value_with_retained_direct(raw, mode, RETAINED_LIMIT).unwrap();
        let report = output.admission().unwrap();
        assert_eq!(report.law().domain, None, "{mode:?}: inside D1");
        if COMPILED_IDENTITY != Some(REGISTERED_PROFILES[0].identity) {
            assert!(output.successor().is_none(), "{mode:?}: Stale, no W1");
            return;
        }
        assert_eq!(report.law().refusal, None, "{mode:?}: admitted");
        match output.retained() {
            Some(Ok(_)) => {}
            other => panic!("{mode:?}: {:?}", other.map(|r| r.as_ref().err())),
        }
        let successor = output.successor().expect("the validated successor is published");
        println!("F1_EXACT_DIRECT {mode:?} successor_bytes={}", serde_json::to_vec(successor).unwrap().len());
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
    w2["model"]["diagnostics"] = depth_16_value();
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
    control_ordinary_w_cb1: inputs::w_cb1();
    control_ordinary_exact_cap_maximal: inputs::exact_cap_maximal();
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
