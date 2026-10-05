//! RV89 part 2 (scratch only; never in maintained code): independent probes of the
//! in-build profile, the gate caps and the permitted path's heap. Mounted as a child
//! of retained_memory in RV89's archive copy. It constructs no profile and no permit.
use super::*;
use crate::source_receipt::CapturedInvocation;
use crate::PreviewSolverMode;
use serde_json::{json, Value};
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};

// A counting allocator: per-thread allocation events (allocation-free checks) and the
// process's live requested bytes with their peak (the challenge pattern).
struct Counting;
thread_local! { static EVENTS: Cell<Option<u64>> = const { Cell::new(None) }; }
static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
fn note(grow: usize) {
    let _ = EVENTS.try_with(|c| {
        if let Some(n) = c.get() {
            c.set(Some(n + 1));
        }
    });
    let now = LIVE.fetch_add(grow, Ordering::SeqCst) + grow;
    PEAK.fetch_max(now, Ordering::SeqCst);
}
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        note(l.size());
        System.alloc(l)
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        note(l.size());
        System.alloc_zeroed(l)
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        // Old and new coexist during a moving reallocation: count both, then release the old.
        note(n);
        let q = System.realloc(p, l, n);
        LIVE.fetch_sub(l.size(), Ordering::SeqCst);
        q
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        LIVE.fetch_sub(l.size(), Ordering::SeqCst);
        System.dealloc(p, l)
    }
}
#[global_allocator]
static RV89_P2_ALLOCATOR: Counting = Counting;
fn events<R>(f: impl FnOnce() -> R) -> (u64, R) {
    EVENTS.with(|c| c.set(Some(0)));
    let r = f();
    (EVENTS.with(|c| c.replace(None)).unwrap(), r)
}

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
fn milestone() -> Value {
    serde_json::from_str(include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json")).unwrap()
}
/// RV89's own solvable large D1 input: 32 nodes on a circle, a 32-member ring, one anchor
/// and 31 scalar springs (milestone shapes), 128 nodal loads, 4 + 4 materials x 16 points.
fn large() -> Value {
    let p = "rv89_invented_large_d1_no_library_data";
    let nodes: Vec<Value> = (0..32).map(|i| { let t = i as f64 * std::f64::consts::TAU / 32.0;
        json!({"id": format!("n{i:02}"), "position": {"x": 8.0 * t.cos(), "y": 8.0 * t.sin(), "z": 0.25 * (i % 4) as f64}, "provenance": p}) }).collect();
    let pipes: Vec<Value> = (0..32).map(|i| json!({"id": format!("p{i:02}"), "from": format!("n{i:02}"), "to": format!("n{:02}", (i + 1) % 32), "material": "m0",
        "y_reference": {"x": 0.0, "y": 0.0, "z": 1.0},
        "section": {"outside_diameter": {"value": 0.1683, "unit": "m"}, "wall_thickness": {"value": 0.00711, "unit": "m"}}, "provenance": p})).collect();
    let supports: Vec<Value> = (0..32).map(|i| if i == 0 {
        json!({"id": "s00", "node": "n00", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": p})
    } else {
        json!({"id": format!("s{i:02}"), "node": format!("n{i:02}"), "family": "spring", "restraints": ["UZ"],
            "stiffness": {"dof": "UZ", "value": {"value": 3.0e6, "unit": "N/m"}}, "provenance": p})
    }).collect();
    let loads: Vec<Value> = (0..128).map(|i| json!({"id": format!("l{i:03}"), "category": "concentrated_force", "target": {"type": "node", "node": format!("n{:02}", i % 32)},
        "direction": if i % 2 == 0 { "global_z" } else { "rotation_x" }, "magnitude": {"value": 5.0 + i as f64, "unit": if i % 2 == 0 { "N" } else { "N*m" }},
        "dimension": if i % 2 == 0 { "force" } else { "moment" }, "provenance": p})).collect();
    let pts: Vec<Value> = (0..16).map(|i| json!({"id": format!("t{i:02}"), "provenance": p})).collect();
    let mats: Vec<Value> = (0..4).map(|i| json!({"id": format!("m{i}"), "elastic_modulus": {"value": 2.0e11, "unit": "Pa"}, "shear_modulus": {"value": 7.9e10, "unit": "Pa"},
        "temperature_points": pts, "provenance": p})).collect();
    json!({"model": {"schema_version": "0.1.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "r".repeat(128), "units": {"length": "m", "force": "N"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "materials": mats,
        "load_cases": [{"id": "c0", "primitive_loads": loads, "provenance": p}], "combinations": []}, "materials": mats})
}
fn atom(name: &str) -> u64 {
    profile::ATOM_VALUES[profile::ATOM_NAMES.iter().position(|n| *n == name).unwrap()]
}
fn form(i: usize) -> u64 {
    profile::form(&profile::FORMS[i], &profile::ATOM_VALUES).unwrap()
}

// ---- 1. Still no permit -----------------------------------------------------------
#[test]
fn rv89p2_no_permit_and_unpriced() {
    assert!(REGISTERED_PROFILES.is_empty());
    let estimates = profile::ATOM_BINDINGS.iter().filter(|b| matches!(b, profile::Binding::Estimate)).count();
    assert_eq!((profile::ESTIMATES, estimates), (42, 42));
    for mode in MODES {
        assert_eq!(cap_priced_maximum(mode), Err(BoundRefusal::Unpriced));
        for e in [1usize, 41, 42, 1000] {
            assert_eq!(priced_maximum(e, mode), Err(BoundRefusal::Unpriced));
        }
    }
    assert_eq!(build_status(), Err(ProfileStatus::Missing));
    let counts = [profile::Binding::InBuild, profile::Binding::SourceUpper, profile::Binding::Text, profile::Binding::Estimate]
        .map(|b| profile::ATOM_BINDINGS.iter().filter(|x| **x == b).count());
    assert_eq!(counts, [190, 6, 6, 42]);
}

// ---- 2. In-build atoms: print them, and recompute a sample independently ------------
fn node_up(sk: usize, ak: usize, sv: usize, av: usize) -> u64 {
    // RV89's own reading of BUILD.md §4 (std 1.97.1 btree/node.rs LeafNode / InternalNode).
    let a = 8usize.max(ak).max(av);
    let up = |x: usize| x.div_ceil(a) * a;
    let leaf = up(8) + up(2) + up(2) + up(11 * sk) + up(11 * sv);
    let internal = (leaf.div_ceil(8) * 8 + 12 * 8).div_ceil(a) * a;
    leaf.max(internal) as u64
}
#[test]
fn rv89p2_in_build_atoms() {
    use std::mem::{align_of, size_of};
    for (n, v) in profile::ATOM_NAMES.iter().zip(profile::ATOM_VALUES) {
        println!("RV89_ATOM\t{n}\t{v}");
    }
    let s = |x: usize| x as u64;
    let checks: Vec<(&str, u64)> = vec![
        ("s(Value)", s(size_of::<Value>())),
        ("s(String)", s(size_of::<String>())),
        ("s(ResultItem)", s(size_of::<crate::ResultItem>())),
        ("s(Diagnostic)", s(size_of::<crate::Diagnostic>())),
        ("s(MechanicsEnvelope)", s(size_of::<crate::MechanicsEnvelope>())),
        ("s(MaterialInput)", s(size_of::<crate::MaterialInput>())),
        ("s(PreviewSupport)", s(size_of::<crate::PreviewSupport>())),
        ("s(PreviewLoadCase)", s(size_of::<crate::PreviewLoadCase>())),
        ("s(PrimitiveLoadInput)", s(size_of::<crate::PreviewPrimitiveLoad>())),
        ("s(TemperaturePoint)", s(size_of::<crate::MaterialTemperaturePointInput>())),
        ("s(StationResultants)", s(size_of::<crate::StationResultants>())),
        ("s(OrdinarySeed)", s(size_of::<crate::retained_product::OrdinarySeed>())),
        ("s(CaptureError)", s(size_of::<crate::retained_product::CaptureError>())),
        ("s(RowClassification)", s(size_of::<open_pipe_stress_result_export::retained_precision::RowClassification>())),
        ("s(Validation)", s(size_of::<open_pipe_stress_result_export::retained_precision::Validation>())),
        ("s(Vec<usize>)", s(size_of::<Vec<usize>>())),
        ("s((String,String))", s(size_of::<(String, String)>())),
        ("s(Option<f64>)", s(size_of::<Option<f64>>())),
        ("s(FrameNode)", s(size_of::<open_pipe_stress_frame_kernel::FrameNode>())),
        ("s(FrameElement)", s(size_of::<open_pipe_stress_frame_kernel::FrameElement>())),
        ("s(SpringEntry)", s(size_of::<open_pipe_stress_linear_supports::SpringEntry>())),
        ("s(ThreadPacketOutput)", s(size_of::<Option<std::thread::Result<Option<Result<crate::RetainedPreviewOutput, String>>>>>())),
        ("Node(String,Value)", node_up(size_of::<String>(), align_of::<String>(), size_of::<Value>(), align_of::<Value>())),
        ("Node(String,ResultItem)", node_up(size_of::<String>(), 8, size_of::<crate::ResultItem>(), align_of::<crate::ResultItem>())),
        ("Node(usize,())", node_up(8, 8, 0, 1)),
        ("Node(&str,())", node_up(16, 8, 0, 1)),
        ("Node(usize,String)", node_up(8, 8, 24, 8)),
        ("Node((String,String),())", node_up(48, 8, 0, 1)),
    ];
    for (n, v) in &checks {
        assert_eq!(atom(n), *v, "{n}");
    }
    // The six SourceUpper atoms against RV89's own field sums (source_receipt.rs:582, :597; rows.rs:324).
    let (st, sr, os, of) = (size_of::<String>(), size_of::<&'static str>(), size_of::<Option<String>>(), size_of::<f64>());
    assert_eq!(atom("s(Projection)"), (4 * st + 3 * sr + 4 * of + 2 * of) as u64);
    assert_eq!(atom("s(RowTreatment)"), (st + sr + os + size_of::<Option<&'static str>>() + size_of::<Vec<String>>()) as u64);
    assert_eq!(atom("s(Derived)"), (size_of::<crate::ResultItem>() + sr + size_of::<Vec<String>>()) as u64);
    assert!(atom("s((Content,Content))") >= 2 * 32);
    println!("RV89_SAMPLE {} atoms recomputed independently", checks.len() + 3);
}

// ---- 3. Gate caps from the profile; the RawVec push law -------------------------------
#[test]
fn rv89p2_gate_caps_and_push_law() {
    let caps = phase_caps();
    assert_eq!(caps.late[8], form(profile::F_T11) - form(profile::F_T11_LATE_CAPTURE));
    assert_eq!(caps.complete[15], form(profile::F_T11));
    assert_eq!(caps.complete[16], form(profile::F_T11_ORDINARY_SEED));
    // push_capacity against the actual Vec growth, for the D1 counts and beyond.
    let mut v: Vec<crate::Diagnostic> = Vec::new();
    let mut r: Vec<u64> = Vec::new();
    for h in 1..=20_000u64 {
        r.push(h);
        if h <= 12_000 {
            v.push(crate::Diagnostic { id: String::new(), code: String::new(), severity: String::new(), message: String::new(), source: None, affected_refs: Vec::new() });
            assert_eq!(push_capacity(h), v.capacity() as u64, "Diagnostic h={h}");
        }
        assert_eq!(push_capacity(h), r.capacity() as u64, "u64 h={h}");
    }
    println!("RV89_GATE_CAPS late={:?} complete={:?}", caps.late, caps.complete);
}

// ---- 4. The priced gates on actual in-domain owners (as if a permit existed) ---------
fn observed(raw: Value, mode: PreviewSolverMode) -> (crate::MechanicsEnvelope, crate::retained_product::ProductCapture, Option<AdmissionRefusal>) {
    let (request, capture) = CapturedInvocation::parse(raw, mode).unwrap();
    let domain = assess(&capture, &request, Entry::Direct).law().domain;
    let mut observer = crate::retained_product::ProductCapture::prepared_probe();
    let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
    (ordinary, observer, domain)
}
#[test]
fn rv89p2_priced_gates_admit_in_domain_owners_without_allocating() {
    for (label, raw) in [("milestone", milestone()), ("large", large())] {
        for mode in MODES {
            let (ordinary, observer, domain) = observed(raw.clone(), mode);
            assert_eq!(domain, None, "{label} is inside D1");
            let (n, verdict) = events(|| check_phase(PhaseGate::Complete, &complete_observations(&CompleteFacts { ordinary: &ordinary, capture: &observer }), &phase_caps().complete));
            assert_eq!(n, 0, "G-C allocated");
            println!("RV89_GC {label} {mode:?} rows={} diags={} verdict={verdict:?}", ordinary.results.len(), ordinary.diagnostics.len());
            assert_eq!(verdict, Ok(()), "{label} {mode:?}: the priced G-C admits an in-domain run");
        }
    }
}

// ---- 5. The witness inputs: inside D1? ---------------------------------------------
#[test]
fn rv89p2_witness_inputs_and_domain() {
    let n05s: Value = serde_json::from_str(include_str!("../../../fixtures/product_preview/source_blocks/n05-sparse_interactive.request.json")).unwrap();
    let n05d: Value = serde_json::from_str(include_str!("../../../fixtures/product_preview/source_blocks/n05-dense_scrutiny.request.json")).unwrap();
    for (label, raw, mode) in [("W3 n05 sparse", n05s, PreviewSolverMode::SparseInteractive), ("W3 n05 dense", n05d, PreviewSolverMode::DenseScrutiny),
        ("W2 cap_maximal", super::law_tests::cap_maximal(), PreviewSolverMode::SparseInteractive)] {
        let (request, capture) = CapturedInvocation::parse(raw, mode).unwrap();
        let r = assess(&capture, &request, Entry::Direct);
        println!("RV89_WITNESS_DOMAIN {label} domain={:?} loads={} cases={}", r.law().domain, r.law().nested.primitive_loads.length, request.model.load_cases.len());
    }
}

// ---- 6. A heap challenge on the permitted W1 path (private driver) -------------------
#[test]
fn rv89p2_permitted_path_peak_against_the_profile() {
    let w3 = |mode| match mode { PreviewSolverMode::SparseInteractive => profile::SPARSE.unwrap().0, PreviewSolverMode::DenseScrutiny => profile::DENSE.unwrap().0 };
    for (label, raw) in [("milestone", milestone()), ("large", large())] {
        for mode in MODES {
            let raw = raw.clone();
            let base = LIVE.load(Ordering::SeqCst);
            PEAK.store(base, Ordering::SeqCst);
            let outcome = crate::on_reserved_stack(RESERVED_STACK_BYTES, move || {
                let (request, capture) = CapturedInvocation::parse(raw, mode).unwrap();
                let mut observer = crate::retained_product::ProductCapture::prepared_probe();
                let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
                let rows = ordinary.results.len();
                let (_, retained) = crate::retained_w1(observer, ordinary, &capture);
                (rows, retained.map(|_| ()).map_err(|f| format!("{f:?}")))
            }).unwrap();
            let peak = PEAK.load(Ordering::SeqCst) - base;
            println!("RV89_PEAK {label} {mode:?} rows={} outcome={:?} peak={peak} max_phase={} ratio={:.6}", outcome.0, outcome.1, w3(mode), peak as f64 / w3(mode) as f64);
            assert!((peak as u64) < w3(mode));
        }
    }
}

// ---- 7. A deep-input publication witness (STACK_INVENTORY W2's depth-16 raw Value and
// escaping, carried through the serializer and the precommit reader on a publishing input).
fn deep_milestone() -> Value {
    let mut raw = milestone();
    fn tag(v: &mut Value) {
        match v {
            Value::Object(m) => {
                for (k, x) in m.iter_mut() {
                    if k == "provenance" {
                        if let Value::String(s) = x {
                            s.push_str(" q\"b\\");
                        }
                    } else {
                        tag(x);
                    }
                }
            }
            Value::Array(a) => a.iter_mut().for_each(tag),
            _ => {}
        }
    }
    tag(&mut raw);
    let mut deep = json!(1);
    for _ in 0..14 {
        deep = json!([deep]);
    }
    raw["model"]["rv89_depth_witness"] = deep;
    raw
}
#[test]
fn rv89p2_deep_input_publishes_on_the_witness_stack() {
    for mode in MODES {
        let raw = deep_milestone();
        let (request, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
        let r = assess(&capture, &request, Entry::Direct);
        assert_eq!(r.law().domain, None, "inside D1");
        assert_eq!(r.raw.maximum_depth, 16, "raw depth 16");
        for stack in [RESERVED_STACK_BYTES / STACK_WITNESS_DIVISOR, RESERVED_STACK_BYTES / 64] {
            let raw = raw.clone();
            let ran = crate::on_reserved_stack(stack, move || {
                let (request, capture) = CapturedInvocation::parse(raw, mode).unwrap();
                let mut observer = crate::retained_product::ProductCapture::prepared_probe();
                let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
                crate::retained_w1(observer, ordinary, &capture).1.map(|s| serde_json::to_vec(s.value()).unwrap().len()).map_err(|f| format!("{f:?}"))
            }).unwrap();
            println!("RV89_DEEP_WITNESS {mode:?} stack={stack} ran={ran:?}");
            assert!(ran.is_ok(), "the deep input publishes a successor at {stack} B of stack");
        }
    }
}

#[test]
fn rv89p2_shared_4_instantiations() {
    use open_pipe_stress_frame_kernel::structural::retained_resource as fkr;
    println!("RV89_SHARED4 shared_4_4={} shared_4_8={} atom={}", fkr::SHARED_4_4, fkr::SHARED_4_8, atom("s(Shared<4>)"));
}
