//! U4 G5: the admission law's tests, through pure functions and the actual G-A
//! census. The tests construct no profile (decision 7): the one registered entry
//! is the production one, a permit comes only from `admit` in the registered build,
//! and a check that needs another matched build passes its inputs to the pure function.
use super::*;
use crate::source_receipt::CapturedInvocation;
use crate::PreviewSolverMode;
use serde_json::json;

#[path = "../tests/common/w1_member_load_inputs.rs"]
mod w1_member_load_inputs;

const MILESTONE: &str = include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
/// The owner-selected current product budget: 12 GiB.
const M: u64 = 12_884_901_888;
/// The caller limit the tests pass: the registered profile's qualified M.
pub(super) const RETAINED_LIMIT: RetainedByteLimit = RetainedByteLimit::new(M);
/// The 0.9 M margin rule's limit, ⌊0.9 M⌋ (`profile_laws_hold_in_this_build` pins its value).
const MARGIN: u64 = 9 * M / 10;
/// B2-A: the combination facts of a request with no combination.
const NO_COMBINATIONS: CombinationFacts =
    CombinationFacts { terms: CapacityFact { length: 0, capacity: 0 }, range_operands: CapacityFact { length: 0, capacity: 0 } };

fn milestone() -> Value {
    serde_json::from_str(MILESTONE).unwrap()
}
/// G6 registration: this build's D1.1 status (Registered in the qualified build, Stale in any
/// other), and the refusal D1.1 then gives (none in the qualified build).
fn build_profile() -> ProfileStatus {
    build_status().map_or_else(|status| status, |_| ProfileStatus::Registered)
}
fn d1_1_refusal() -> Option<AdmissionRefusal> {
    build_status().err().map(AdmissionRefusal::Profile)
}
/// The actual G-A over a raw request: parse once, as the dispatch does, then admit.
fn admitted(raw: Value) -> RetainedAdmissionReport {
    let (request, capture) = CapturedInvocation::parse(raw, PreviewSolverMode::SparseInteractive).unwrap();
    assess(&capture, &request, Entry::Direct, RETAINED_LIMIT)
}
/// G-A over a typed request changed after the parse (capacity rows).
fn admitted_typed(raw: Value, change: impl FnOnce(&mut LinearStaticPreviewRequest)) -> RetainedAdmissionReport {
    let (mut request, capture) = CapturedInvocation::parse(raw, PreviewSolverMode::SparseInteractive).unwrap();
    change(&mut request);
    assess(&capture, &request, Entry::Direct, RETAINED_LIMIT)
}
fn domain(raw: Value) -> Option<AdmissionRefusal> {
    admitted(raw).law().domain
}
fn cap(fact: CapFact) -> impl Fn(Option<AdmissionRefusal>) -> bool {
    move |r| matches!(r, Some(AdmissionRefusal::Cap { fact: f, .. }) if f == fact)
}
fn model(r: &mut Value) -> &mut serde_json::Map<String, Value> {
    r["model"].as_object_mut().unwrap()
}
/// The raw-census rows' raw-only value: every model record is closed (an unknown key refuses
/// before admission), so the value rides on a declared member the solve never reads, node 3's
/// `notes` (model document 0.7.0), at the depth of the node's own members (4).
fn carry(r: &mut Value, value: Value) {
    r["model"]["nodes"][3]["notes"] = value;
}
fn family(clause: D1Clause, fact: FamilyFact) -> Option<AdmissionRefusal> {
    Some(AdmissionRefusal::Family(clause, fact))
}
fn text(prefix: &str, len: usize) -> String {
    let mut s = String::from(prefix);
    while s.len() < len {
        s.push('x');
    }
    s.truncate(len);
    s
}

/// The cap-maximal ring's node positions as binary64 bits, (x, y) = (10·cos t, 10·sin t) at
/// t = 2π·i/32, exactly as computed when the inputs were pinned (I81's PROBE and SQ). They are
/// spelled as bits because `f64::cos` and `f64::sin` are not correctly rounded: the hosted CI's
/// libm differs from the Mac's in the last bit for some `t`, which changed this input and its
/// pinned hashes there. `cap_maximal_ring_is_the_trigonometric_ring` checks each entry against
/// the platform's `cos` and `sin` to within one ulp.
const CAP_MAXIMAL_RING: [(u64, u64); 32] = [
    (0x4024000000000000, 0x0000000000000000), (0x40239d9ee1fa99ee, 0x3fff36e64b840f8c),
    (0x40227a43617f984c, 0x400e9d5b505a53bc), (0x4020a11fd9a92506, 0x4016390a081a02fa),
    (0x401c48c6001f0ac0, 0x401c48c6001f0abf), (0x4016390a081a02fb, 0x4020a11fd9a92506),
    (0x400e9d5b505a53bd, 0x40227a43617f984c), (0x3fff36e64b840f90, 0x40239d9ee1fa99ee),
    (0x3cc60fafbfd97309, 0x4024000000000000), (0xbfff36e64b840f8a, 0x40239d9ee1fa99ee),
    (0xc00e9d5b505a53ba, 0x40227a43617f984c), (0xc016390a081a02f8, 0x4020a11fd9a92506),
    (0xc01c48c6001f0abf, 0x401c48c6001f0ac0), (0xc020a11fd9a92506, 0x4016390a081a02fa),
    (0xc0227a43617f984c, 0x400e9d5b505a53be), (0xc0239d9ee1fa99ee, 0x3fff36e64b840f9d),
    (0xc024000000000000, 0x3cd60fafbfd97309), (0xc0239d9ee1fa99ee, 0xbfff36e64b840f92),
    (0xc0227a43617f984c, 0xc00e9d5b505a53b9), (0xc020a11fd9a92507, 0xc016390a081a02f8),
    (0xc01c48c6001f0ac2, 0xc01c48c6001f0abf), (0xc016390a081a02fa, 0xc020a11fd9a92506),
    (0xc00e9d5b505a53c8, 0xc0227a43617f984a), (0xbfff36e64b840f9f, 0xc0239d9ee1fa99ed),
    (0xbce08bc3cfe31646, 0xc024000000000000), (0x3fff36e64b840f8f, 0xc0239d9ee1fa99ee),
    (0x400e9d5b505a53c1, 0xc0227a43617f984b), (0x4016390a081a02f6, 0xc020a11fd9a92507),
    (0x401c48c6001f0abe, 0xc01c48c6001f0ac2), (0x4020a11fd9a92506, 0xc016390a081a02fa),
    (0x40227a43617f984a, 0xc00e9d5b505a53ca), (0x40239d9ee1fa99ed, 0xbfff36e64b840fa2),
];

/// A cap-maximal D1 request (STACK_INVENTORY.md §3 W2's counts at `l ≤ 128`):
/// 32 nodes, a 32-member ring, 32 supports with 6 restraints each (r = 192) and a
/// scalar spring on each (s = 32), 128 nodal loads, 4 + 4 materials with 16
/// temperature points each, and one 128-byte identifier.
pub(super) fn cap_maximal() -> Value {
    let p = "invented_t3_g5_cap_maximal_input_no_library_data";
    let nodes: Vec<Value> = CAP_MAXIMAL_RING
        .iter()
        .enumerate()
        .map(|(i, &(x, y))| {
            json!({"id": format!("N{i}"), "position": {"x": f64::from_bits(x), "y": f64::from_bits(y), "z": 0.0}, "provenance": p})
        })
        .collect();
    let pipes: Vec<Value> = (0..32)
        .map(|i| json!({"id": format!("M{i}"), "from": format!("N{i}"), "to": format!("N{}", (i + 1) % 32), "material": "mat:0", "y_reference": {"x": 0.0, "y": 0.0, "z": 1.0},
            "section": {"outside_diameter": {"value": 0.2, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"}}, "provenance": p}))
        .collect();
    let supports: Vec<Value> = (0..32)
        .map(|i| json!({"id": format!("S{i}"), "node": format!("N{i}"), "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"],
            "stiffness": {"dof": "UY", "value": {"value": 1.0e6, "unit": "N/m"}}, "provenance": p}))
        .collect();
    let loads: Vec<Value> = (0..128)
        .map(|i| json!({"id": format!("L{i}"), "category": "concentrated_force", "target": {"type": "node", "node": format!("N{}", i % 32)},
            "direction": if i % 2 == 0 { "global_y" } else { "rotation_x" }, "magnitude": {"value": 1.0, "unit": if i % 2 == 0 { "N" } else { "N*m" }},
            "dimension": if i % 2 == 0 { "force" } else { "moment" }, "provenance": p}))
        .collect();
    let points: Vec<Value> = (0..16).map(|i| json!({"id": format!("T{i}"), "provenance": p})).collect();
    let materials: Vec<Value> = (0..4)
        .map(|i| json!({"id": format!("mat:{i}"), "elastic_modulus": {"value": 2.0e11, "unit": "Pa"}, "shear_modulus": {"value": 8.0e10, "unit": "Pa"},
            "temperature_points": points, "provenance": p}))
        .collect();
    json!({
        "model": {
            "schema_version": "0.8.0",
            "document_kind": "openpipestress.product_preview.model",
            "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
            "project": {"id": text("project:", 128), "units": {"length": "m", "force": "N"}},
            "nodes": nodes,
            "pipe_segments": pipes,
            "materials": materials,
            "supports": supports,
            "load_cases": [{"id": "case", "primitive_loads": loads, "provenance": p}],
            "combinations": []
        },
        "materials": materials
    })
}

/// `CAP_MAXIMAL_RING` is the ring `cap_maximal` was pinned on: each coordinate is within one ulp of
/// 10·cos t or 10·sin t as this platform's libm computes it, except the near-zero residues.
#[test]
fn cap_maximal_ring_is_the_trigonometric_ring() {
    let ulps = |a: f64, b: f64| (a.to_bits() as i64 - b.to_bits() as i64).unsigned_abs();
    let mut residues = 0;
    for (i, &(x, y)) in CAP_MAXIMAL_RING.iter().enumerate() {
        let t = 2.0 * std::f64::consts::PI * i as f64 / 32.0;
        let (cx, cy) = (10.0 * t.cos(), 10.0 * t.sin());
        for (pinned, computed) in [(f64::from_bits(x), cx), (f64::from_bits(y), cy)] {
            // Near zero (i = 8, 16, 24) the coordinate is a rounding residue of π, compared
            // absolutely (RV125 A1-N1); every other coordinate, N0's exact 0 included, is
            // within one ulp.
            if pinned != 0.0 && pinned.abs() < 1e-14 {
                residues += 1;
                assert!((pinned - computed).abs() <= 1e-14, "N{i}: {pinned:e} against {computed:e}");
            } else {
                assert!(ulps(pinned, computed) <= 1, "N{i}: {pinned:e} against {computed:e}");
            }
        }
    }
    assert_eq!(residues, 3, "the residues are N8's x, N16's y and N24's x");
}

/// B1 SA: `raw` with `cases` load cases, each a copy of its first case, with the ids
/// `case-1`, `case-2`, … and its loads' ids suffixed `~1`, `~2`, … (the ordinary route
/// requires primitive-load ids unique across cases). `cases` = 0 leaves no case.
fn with_cases(mut raw: Value, cases: usize) -> Value {
    let first = raw["model"]["load_cases"][0].clone();
    let list = raw["model"]["load_cases"].as_array_mut().unwrap();
    list.clear();
    for ordinal in 1..=cases {
        let mut case = first.clone();
        case["id"] = json!(format!("case-{ordinal}"));
        for load in case["primitive_loads"].as_array_mut().unwrap() {
            load["id"] = json!(format!("{}~{ordinal}", load["id"].as_str().unwrap()));
        }
        list.push(case);
    }
    raw
}
/// The milestone with `cases` load cases (B1 SA's c = 0 to C + 1 oracles).
fn milestone_cases(cases: usize) -> Value {
    with_cases(milestone(), cases)
}
/// The cap-maximal request with `cases` load cases of 128 loads each: at C = 3 it is the
/// cap-maximal three-case shape (every case at l, Σ l_i = L).
pub(super) fn cap_maximal_cases(cases: usize) -> Value {
    with_cases(cap_maximal(), cases)
}
/// A cap-maximal request whose ordinary solve publishes (W2b's construction in
/// `witness_tests::w2b_input`: `cap_maximal` with the milestone's support shapes), with
/// `cases` load cases of 128 loads each.
fn solvable_cap_maximal_cases(cases: usize) -> Value {
    let mut raw = cap_maximal();
    for (i, support) in raw["model"]["supports"].as_array_mut().unwrap().iter_mut().enumerate() {
        if i == 0 {
            support.as_object_mut().unwrap().remove("stiffness");
        } else {
            support["family"] = json!("spring");
            support["restraints"] = json!(["UY"]);
        }
    }
    with_cases(raw, cases)
}

// ---- D-6 -------------------------------------------------------------------

#[test]
fn encoder_roundtrip_covers_every_byte() {
    use build_identity::*;
    for byte in 0..=255u8 {
        let mut encoded = String::new();
        encode_identity_value(&[byte], &mut encoded);
        assert!(encoded.bytes().all(|b| identity_byte_is_plain(b) || b == b'%'), "{byte:#04x}: {encoded}");
        assert!(!encoded.contains([';', '=', ' ']) && encoded.bytes().all(|b| (0x21..0x7F).contains(&b)), "{byte:#04x}");
        assert_eq!(decode_identity_value(&encoded), Some(vec![byte]), "{byte:#04x}");
        assert_eq!(encoded.len(), if identity_byte_is_plain(byte) { 1 } else { 3 });
    }
    let all: Vec<u8> = (0..=255).collect();
    let mut encoded = String::new();
    encode_identity_value(&all, &mut encoded);
    assert_eq!(decode_identity_value(&encoded), Some(all));
    // Outside the encoder's image: lowercase, short, non-hex, escaped plain bytes, raw space.
    for bad in ["%3b", "%3", "%G0", "%41", "a b", "a;b", "a=b"] {
        assert_eq!(decode_identity_value(bad), None, "{bad}");
    }
    // An empty value is a value.
    let empty: [&[u8]; 16] = [b""; 16];
    let text = encode_identity(&empty);
    assert_eq!(decode_identity(&text), Some(vec![Vec::new(); 16]));
    assert_eq!(text.matches(';').count(), 16);
}

#[test]
fn identity_carries_every_key_in_order() {
    use build_identity::*;
    let compiled = COMPILED_IDENTITY.expect("build.rs sets the identity");
    assert_eq!(option_env!("OPS_RETAINED_BUILD_IDENTITY"), Some(compiled));
    assert_ne!(compiled, IDENTITY_UNAVAILABLE, "every key is readable on this host");
    assert!(!compiled.contains(['\n', ' ']), "one line");
    let tokens: Vec<&str> = compiled.split(';').collect();
    assert_eq!(tokens[0], IDENTITY_VERSION);
    let keys: Vec<&str> = tokens[1..].iter().map(|t| t.split_once('=').unwrap().0).collect();
    assert_eq!(keys, IDENTITY_KEYS, "every key, in order, no extras");
    let values = decode_identity(compiled).expect("decodes");
    let borrowed: [&[u8]; 16] = std::array::from_fn(|i| values[i].as_slice());
    assert_eq!(encode_identity(&borrowed), compiled, "re-encoding reproduces the text");
    let value = |key: &str| String::from_utf8(values[IDENTITY_KEYS.iter().position(|k| *k == key).unwrap()].clone()).unwrap();
    assert_eq!(value("debug_assertions"), cfg!(debug_assertions).to_string());
    assert_eq!(value("target.pointer_width"), (usize::BITS).to_string());
    assert_eq!(value("pkg"), format!("{}@{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
    assert_eq!(value("target.endian"), if cfg!(target_endian = "little") { "little" } else { "big" });
    assert_eq!(value("target.os"), std::env::consts::OS);
    assert_eq!(value("target.arch"), std::env::consts::ARCH);
    assert_eq!(value("panic"), if cfg!(panic = "unwind") { "unwind" } else { "abort" });
    assert_eq!(value("profile"), if cfg!(debug_assertions) { "debug" } else { "release" });
    // G6 registers the qualified build's text, printed by the same encoder.
    println!("I65_G5_IDENTITY {compiled}");
}

#[test]
fn identity_matching_is_missing_stale_or_exact() {
    use build_identity::IDENTITY_UNAVAILABLE;
    let none: [&str; 0] = [];
    assert_eq!(identity_match(Some("v1;a=b"), none.into_iter()), Err(ProfileStatus::Missing), "nothing registered");
    assert_eq!(identity_match(None, none.into_iter()), Err(ProfileStatus::Missing));
    let registered = ["v1;a=b", "v1;a=c"];
    assert_eq!(identity_match(None, registered.into_iter()), Err(ProfileStatus::Stale), "absent is Stale");
    assert_eq!(identity_match(Some(IDENTITY_UNAVAILABLE), registered.into_iter()), Err(ProfileStatus::Stale));
    assert_eq!(identity_match(Some("v1;a=d"), registered.into_iter()), Err(ProfileStatus::Stale));
    assert_eq!(identity_match(Some("v1;a=b;"), registered.into_iter()), Err(ProfileStatus::Stale), "byte-exact, no prefix");
    assert_eq!(identity_match(Some("v1;a=c"), registered.into_iter()), Ok(1));
    let unavailable = [IDENTITY_UNAVAILABLE];
    assert_eq!(identity_match(Some(IDENTITY_UNAVAILABLE), unavailable.into_iter()), Err(ProfileStatus::Stale), "never matches");
}

#[test]
fn bindings_need_witnesses_inputs_and_reader_layouts() {
    let layouts = READER_LAYOUTS;
    assert!(bindings_hold(true, Some("v1;x"), "v1;x", &layouts, &layouts));
    assert!(!bindings_hold(false, Some("v1;x"), "v1;x", &layouts, &layouts), "a false layout witness");
    assert!(!bindings_hold(true, None, "v1;x", &layouts, &layouts), "no reviewed-input record");
    assert!(!bindings_hold(true, Some("v1;y"), "v1;x", &layouts, &layouts), "a changed lock or static");
    for index in 0..layouts.len() {
        let mut moved = layouts;
        moved[index].size += 8;
        assert!(!bindings_hold(true, Some("v1;x"), "v1;x", &layouts, &moved), "changed reader layout {}",READER_LAYOUT_NAMES[index]);
    }
    assert!(LAYOUT_WITNESSES, "this build's std and serde_json layouts are the ones the formulas assume");
    for (name, layout) in READER_LAYOUT_NAMES.iter().zip(READER_LAYOUTS) {
        println!("I65_G5_READER_LAYOUT {name} size={} align={}", layout.size, layout.align);
    }
}

#[test]
fn sha256_matches_the_sha2_dependency() {
    use sha2::{Digest, Sha256};
    let mut data = Vec::new();
    for len in 0..=300usize {
        let expected: [u8; 32] = Sha256::digest(&data).into();
        assert_eq!(build_identity::sha256(&data), expected, "length {len}");
        data.push((len * 31 % 251) as u8);
    }
    assert_eq!(build_identity::sha256(b"abc")[..4], [0xba, 0x78, 0x16, 0xbf]);
}

const PRESSURE_SOURCE_BINDING_INPUT: &str =
    "../../fixtures/results/semantic_contract_v0_3_pressure_source_1.json";
const PRESSURE_SCHEMA_BINDING_INPUT: &str =
    "../../schemas/results_pressure_2.v0.3.schema.json";
const PRESSURE_TABLE_INPUT: &str =
    "../../fixtures/results/semantic_contract_v0_3_pressure_2.json";
// B7b (O-10, b7-plan "Join"): the pressure-source table is resident
// (`pressure_source_contract()`), so its binding is an ordinary include
// obligation again; the pressure result schema is the one nonresident binding.
const NONRESIDENT_BINDING_INPUTS: [(usize, &str); 1] = [
    (10, PRESSURE_SCHEMA_BINDING_INPUT),
];
const SEMANTIC_E14_TEST_ITEM: &str = "#[cfg(test)]\nmod e14_tests {";
const SEMANTIC_B7B_TEST_ITEM: &str = "#[cfg(test)]\nmod b7b_current_table_tests {";

fn reader_include_required(path: &str) -> bool {
    path != PRESSURE_SCHEMA_BINDING_INPUT
}
#[derive(Clone, Copy)]
struct AuditedTestRegion {
    first_item: &'static str,
    complete_suffix_sha256: &'static str,
    production_resume: Option<&'static str>,
    /// A closed test item after `production_resume` that runs to the end of the
    /// file. Its bytes are inside the hashed complete suffix.
    trailing_test: Option<&'static str>,
}

fn audited_reader_sources() -> [(&'static str, Option<AuditedTestRegion>); 5] {
    [
        (include_str!("../../reporting/result_export/src/retained_precision.rs"), Some(AuditedTestRegion {
            first_item: "#[cfg(test)]\nmod b3b_table_binding_tests {",
            // Re-audited on the K-D5 merge: the suffix gained only K-D5's closed
            // `c7_prescribed_key_controls` test module after the trailing
            // production functions; it has no include and stays inspected.
            // Re-audited at B7b D3: only the closed `b7a_preview_namespace_tests`
            // module changed (a 0.5.0 null-contract expectation and the exact v3
            // namespace controls); the production after the resume is byte-identical.
            // Re-audited at T4-U6/U8 D3 (addbdc030e): the suffix only gains the closed
            // `t4_u6_g8_tests` module at the end of the file (G8's mill-tolerance
            // decline; its one include is a request fixture, not a reviewed input), and
            // it stays inspected with the production after the resume, which is
            // byte-identical.
            // W1 independent re-audit: the complete RF suffix is byte-identical,
            // followed only by the closed `w1_member_reader_gate_tests` module
            // (no includes, path modules or statics). It stays inspected after
            // the unchanged production resume; no exclusion boundary changes.
            // The G8 scope repair adds controls only inside that closed module.
            // Re-audited for model 0.6.0 (S1): only the closed
            // `b7a_preview_namespace_tests` module (the current version and 0.5.0
            // among the retired versions), the closed member-load test's invocation
            // version and the doc comment of a trailing production function (the
            // -2 identity) changed; no include, path module or excluded production.
            // Re-audited at O4 S3 (W1-E slice): only the closed `t4_u6_g8_tests` module's
            // one test changed, renamed `g8_namespace_admits_either_shear_model_and_nonzero_allowances`
            // (the retired mill-tolerance decline becomes namespace controls; the same
            // request-fixture include); no include, path module, static or production entered
            // the suffix, and the production after the resume is byte-identical.
            // Re-audited for model 0.7.0 (W-13 S1b): only the closed
            // `b7a_preview_namespace_tests` module (the current version 0.7.0, 0.6.0 among the
            // retired versions, and one doc comment) and, after the resume, the closed
            // `w1_member_reader_gate_tests` module's invocation version changed; no include, path
            // module, static or production entered the suffix, and the two production functions
            // after the resume are byte-identical.
            // Re-audited for model 0.8.0 (W-15 slice c): the same three places changed their
            // version token only (0.7.0 -> 0.8.0, same length): the closed
            // `b7a_preview_namespace_tests` module's current version and one doc comment, and,
            // after the resume, the closed `w1_member_reader_gate_tests` module's invocation
            // version; no include, path module, static or production entered the suffix, and
            // the two production functions after the resume are byte-identical.
            complete_suffix_sha256: "cc5e359afd4f725b62a1540cb749b40f3e3e385dea2a100aa2c9b2637e98f805",
            // Only the preceding closed test region is excluded. These two
            // trailing public functions are production and remain inspected.
            production_resume: Some("/// The reader's outcome as JSON, for callers outside Rust (the desktop's wasm"),
            trailing_test: None,
        })),
        (include_str!("../../reporting/result_export/src/physics_source.rs"), Some(AuditedTestRegion {
            first_item: "#[cfg(test)]\nmod review_repair_tests {",
            // Re-audited at B7b D3 (033626030c): the closed test module's four
            // include paths moved under fixtures/product_preview/pressure_source/;
            // nothing else in the suffix changed and it holds no production.
            // Re-audited at T4-U6/U8 D3 (addbdc030e): the closed module's
            // torsion-constant control writes both D-5 section bases, and it gains
            // the test `actual_materials_bind_each_section_beam_theory_to_the_euler_bernoulli_contract`
            // (its include is a request fixture); the suffix still holds no production.
            // Re-audited at O4 S1 (source slice): only that closed test changed, renamed
            // `actual_materials_bind_each_section_beam_theory_to_the_contract_selection`
            // (both admitted contracts and an unknown one; the same request-fixture
            // include); no include, path module, static or production entered the suffix.
            complete_suffix_sha256: "79dd258fcedcac126713dc385f12e2020908064f64b7435ae8b0fede5d9b82d0",
            production_resume: None,
            trailing_test: None,
        })),
        (include_str!("../../reporting/result_export/src/semantic_contract.rs"), Some(AuditedTestRegion {
            // B7b D3 audit: the closed `b7b_current_table_tests` module (its own
            // test-only includes of the two current tables), then production from
            // `for_source_metadata` on, which stays inspected, then the closed E14
            // module to the end of the file (its table list changed, test-only).
            // Re-audited at ccb19ba0bf: only the inspected production between the
            // regions changed (current_header's load-state carrier guard).
            // Re-audited at T4-U6/U8 D3 (addbdc030e): only the closed
            // `b7b_current_table_tests` module changed (the D-5 tables' 106 and 88
            // rows, and the per-basis signature test); the inspected production and
            // E14 are byte-identical.
            // Re-audited for model 0.6.0 (S1): the closed `b7b_current_table_tests`
            // module's test-only include names the renamed pressure-2 table; the
            // inspected production between the regions gains current_header's
            // pressure-1/preview-physics-1 refusals and the 0.6.0 re-author text;
            // E14 is byte-identical. The S1 repair restores two comments in the
            // inspected production to their historical pressure-1 wording; no code changed.
            // Re-audited for T4-U5b (S3), on the merged bytes: only the closed
            // `b7b_current_table_tests` module's pressure-2 row count changed
            // (106 -> 107, the tie-rod force signature); the inspected production
            // and E14 are byte-identical to the S1 repair's.
            // Re-audited for T4-U4b (S2), on the bytes merged with S3: only the closed
            // `b7b_current_table_tests` module changed (pressure-2's 107 -> 129 rows,
            // with its comment); the inspected production after the resume and E14
            // are byte-identical to S3's.
            // Re-audited for T4-U4b S4: only the closed `b7b_current_table_tests` module
            // changed (pressure-source-1's 88 -> 94 rows and 51 -> 53 kinds, with its
            // comment); the inspected production after the resume and E14 are
            // byte-identical to S2's.
            // Re-audited for model 0.7.0 (W-13 S1b): only the inspected production's
            // current_header v2 re-author text changed its model version token (0.6.0 -> 0.7.0,
            // same length); the closed `b7b_current_table_tests` module and E14 are byte-identical.
            // Re-audited for W-7 (DEL-04-01 Design global-end-actions.md): only the closed
            // `b7b_current_table_tests` module changed (pressure-2's 129 -> 135 rows, with its
            // comment); the inspected production after the resume and E14 are byte-identical.
            // Re-audited on W-7 merged with main 2653f87392: the closed module is W-7's, the inspected
            // production after the resume is main's (model 0.7.0's token) and E14 is unchanged, each
            // byte-identical to its parent's; no production entered the suffix.
            // Re-audited for W-15 slice d (DEL-03-05): only the inspected production's
            // `rule_binding_refusal` gains its first check (a row carrying `component_body` is
            // refused with COMPONENT_BODY_REFERENCE_SECTION, two crate constants, seven lines);
            // no include, path module or static entered, and the closed `b7b_current_table_tests`
            // module and E14 are byte-identical.
            first_item: SEMANTIC_B7B_TEST_ITEM,
            // Re-audited on W-15 merged with main c9517615af (W-7's closed module): against main's suffix
            // only the inspected production's `rule_binding_refusal` gains W-15's first check (seven
            // lines); the closed `b7b_current_table_tests` module and E14 are main's, byte for byte.
            // Re-audited for model 0.8.0 (W-15 slice c): only the inspected production's
            // current_header v2 re-author text changed its model version token (0.7.0 -> 0.8.0,
            // same length); the closed `b7b_current_table_tests` module and E14 are byte-identical.
            complete_suffix_sha256: "6dc09d339738a7ac30ef674a4d017d266433b3be8d95f7596d1b2fb23a420428",
            production_resume: Some("/// Header dispatch only; canonical envelopes have no raw rows. This does not"),
            trailing_test: Some(SEMANTIC_E14_TEST_ITEM),
        })),
        (include_str!("../../reporting/result_export/src/source_blocks.rs"), Some(AuditedTestRegion {
            first_item: "#[cfg(test)]\nmod norm_tests {",
            // B4/D1 (b) and E15: the closed test suffix uses the shared
            // composite helpers and a test-only preview_magnitude_controls include.
            // The audited suffix contains no ungated production or resident include.
            // RF D3 re-audit: the old suffix is byte-identical and gains only the
            // closed `rf16_failure_block_order_tests` module. Its sole include is
            // the nonresident receipt schema; no production is excluded.
            complete_suffix_sha256: "d78d6a148e963c3dc3f8662ec702093ec8aafd348b3437557968e14d079d0b1c",
            production_resume: None,
            trailing_test: None,
        })),
        (include_str!("../../reporting/result_export/src/reader_verdict.rs"), None),
    ]
}

/// Fail closed on any changed or unknown test region. The exact current
/// boundaries and complete suffix bytes have been audited independently; this
/// is not a parser or a general rule for discarding cfg(test) text.
fn audited_reader_production(source: &str, region: Option<AuditedTestRegion>) -> Result<String, &'static str> {
    use sha2::{Digest, Sha256};
    let Some(region) = region else {
        return if source.contains("#[cfg(test)]") { Err("UNKNOWN_TEST_MARKER") } else { Ok(source.into()) };
    };
    if source.matches(region.first_item).count() != 1 {
        return Err("MISSING_OR_AMBIGUOUS_TEST_BOUNDARY");
    }
    let start = source.find(region.first_item).unwrap();
    if start != 0 && source.as_bytes()[start-1] != b'\n' {
        return Err("TEST_BOUNDARY_NOT_ITEM_LINE");
    }
    if source.find("#[cfg(test)]") != Some(start) {
        return Err("EARLIER_TEST_MARKER");
    }
    let suffix = &source[start..];
    // Hash the entire suffix before excluding anything, including production
    // after the retained reader's test region. Appended production cannot hide.
    if format!("{:x}", Sha256::digest(suffix.as_bytes())) != region.complete_suffix_sha256 {
        return Err("CHANGED_AUDITED_SUFFIX");
    }
    let resume = match region.production_resume {
        Some(anchor) => {
            if suffix.matches(anchor).count() != 1 { return Err("MISSING_OR_AMBIGUOUS_PRODUCTION_RESUME"); }
            start + suffix.find(anchor).unwrap()
        }
        None => source.len(),
    };
    // A trailing closed test item ends the resumed production; it was hashed above.
    let end = match (region.production_resume, region.trailing_test) {
        (_, None) => source.len(),
        (None, Some(_)) => return Err("TRAILING_TEST_WITHOUT_RESUME"),
        (Some(_), Some(item)) => {
            let resumed = &source[resume..];
            if resumed.matches(item).count() != 1 { return Err("MISSING_OR_AMBIGUOUS_TRAILING_TEST"); }
            let at = resume + resumed.find(item).unwrap();
            if source.as_bytes()[at - 1] != b'\n' { return Err("TRAILING_TEST_NOT_ITEM_LINE"); }
            at
        }
    };
    Ok(format!("{}{}", &source[..start], &source[resume..end]))
}
fn actual_reader_include(reader: &str, input: &str) -> bool {
    let target = format!("../../{input}");
    let compact: String = reader.chars().filter(|c| !c.is_ascii_whitespace()).collect();
    ["include_str!", "include_bytes!"].iter()
        .any(|kind| compact.contains(&format!("{kind}(\"{target}\"")))
}

/// The full registration law and the focused pre-registration check share this
/// guard. Only the exact pressure result schema path is exempt from resident includes.
fn assert_reviewed_reader_input_bindings() {
    const CURRENT: [&str; 12] = [
        "Cargo.lock",
        "../../schemas/physics_source_recovery.schema.json",
        "../../schemas/retained_precision_mp_v2.schema.json",
        "../../fixtures/results/retained_precision_prepared_ordinary_v1.json",
        "../../fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json",
        "../../fixtures/results/semantic_contract_v0_3_preview_physics_2.json",
        "../../fixtures/results/retained_precision_prepared_combination_v1.json",
        "../../fixtures/results/retained_precision_prepared_exact_v1.json",
        "../../fixtures/results/semantic_contract_v0_3_physics_retained_1.json",
        PRESSURE_SOURCE_BINDING_INPUT,
        PRESSURE_SCHEMA_BINDING_INPUT,
        PRESSURE_TABLE_INPUT,
    ];
    // B7a E2: the retired v0_2 table left; B1: the precision-1 table left;
    // B4/D1 (b): the source-blocks-1 table and its receipt schema left; B7b
    // (A4/A5): the physics-1, load-reference-1, load-reference-source-1 and
    // physics-source-1 tables left with v2, and the resident ordinary pressure
    // table was appended (O-10). Every other input keeps its order.
    assert_eq!(build_identity::REVIEWED_INPUTS, CURRENT);
    let audited = audited_reader_sources();
    let sources: Vec<_> = audited[..4].iter().map(|(source, region)|
        audited_reader_production(source, *region).expect("reader test region changed: re-audit before exclusion")
    ).collect();
    let reader = sources.concat();
    let mut exemptions = 0;
    for (index, path) in build_identity::REVIEWED_INPUTS.iter().enumerate().skip(1) {
        let name = path.rsplit('/').next().unwrap();
        if reader_include_required(path) {
            // Preserve every earlier include obligation and require its actual
            // source-qualified production macro, rather than a filename mention.
            assert!(reader.contains(&format!("{name}\"")), "{name} is an included input of the reader");
            assert!(actual_reader_include(&reader, path), "{path} is a production reader include");
        } else {
            exemptions += 1;
            let expected = NONRESIDENT_BINDING_INPUTS.iter()
                .find(|(_, binding)| *binding == *path).expect("only the exact closed binding paths are exempt");
            assert_eq!(index, expected.0);
            assert!(!actual_reader_include(&reader, path), "the binding is nonresident");
        }
    }
    assert_eq!(exemptions, 1);
    let semantic = &sources[2];
    // B7b (O-10): the supplied-byte verifier checks the reviewed digest and the
    // table's identity and creates nothing resident; the one resident current
    // source table is that verifier over the reviewed bytes, held once.
    let verifier = semantic.split("pub fn verify_pressure_source_table(").nth(1).unwrap()
        .split("pub fn pressure_source_contract(").next().unwrap();
    for required in ["Sha256::digest(bytes)", "PRESSURE_SOURCE_TABLE_SHA256",
        "serde_json::from_slice(bytes)", "table[\"semantic_contract_id\"] != PRESSURE_SOURCE_ID",
        "table[\"source_signature_count\"] != PRESSURE_SOURCE_SIGNATURE_COUNT",
        "table[\"source_kind_count\"] != PRESSURE_SOURCE_KIND_COUNT"] {
        assert!(verifier.contains(required), "binding verifier must retain {required}");
    }
    // T4-U6 (D-5): the verifier's counts are named constants; the D-5 table had 88 signatures
    // over the same 51 kinds (B7b: 69 and 51); T4-U4b §4.3 adds the straight derived signatures
    // (94 over 53), and its digest is reviewed input #10.
    {
        use open_pipe_stress_result_export::semantic_contract as sc;
        assert_eq!((sc::PRESSURE_SOURCE_SIGNATURE_COUNT, sc::PRESSURE_SOURCE_KIND_COUNT), (94, 53));
        assert_eq!(sc::PRESSURE_SOURCE_TABLE_SHA256, COMPILED_REVIEWED_INPUTS.unwrap().split(';').nth(10).unwrap()
            .strip_prefix("../../fixtures/results/semantic_contract_v0_3_pressure_source_1.json=").unwrap());
    }
    for forbidden in ["include_str!", "include_bytes!", "OnceLock", "get_or_init"] {
        assert!(!verifier.contains(forbidden), "binding verifier creates no resident table: {forbidden}");
    }
    let body = |text: &str, item: &str| text.split(item).nth(1).unwrap().split("\n}\n").next().unwrap().to_string();
    for (item, verify, path) in [("pub fn pressure_source_contract(", "verify_pressure_source_table(include_bytes!(", PRESSURE_SOURCE_BINDING_INPUT),
        ("pub fn pressure_rows_contract(", "verify_pressure_table(include_bytes!(", PRESSURE_TABLE_INPUT)] {
        let resident = body(semantic, item);
        assert!(resident.contains("static CONTRACT: OnceLock<Value> = OnceLock::new();") && resident.contains(verify), "{item}");
        assert!(actual_reader_include(&resident, path), "{item} holds the reviewed {path}");
    }
    let metadata = body(semantic, "fn current_header(");
    assert!(metadata.contains("Some(PRESSURE_SOURCE_ID) => pressure_source_contract(),"));
    assert!(metadata.contains("Some(PRESSURE_ID) => pressure_rows_contract(),"));
    let signature = sources[3].split("fn signature(").nth(1).unwrap().split("fn ").next().unwrap();
    assert!(signature.contains("crate::semantic_contract::pressure_source_contract()"));
    let verdict = audited_reader_production(audited[4].0, audited[4].1).expect("verdict has no test region");
    for (_, path) in NONRESIDENT_BINDING_INPUTS {
        assert!(!actual_reader_include(&verdict, path), "the verdict has no resident binding include");
    }
    let findings = verdict.split("pub fn source_findings(").nth(1).unwrap().split("pub fn ").next().unwrap();
    assert!(findings.contains("let table = sc::pressure_source_contract();"));
}

#[test]
fn pressure_source_reviewed_guard_refuses_hidden_or_changed_production() {
    fn refuses(source: &str, region: Option<AuditedTestRegion>, expected: &str) {
        match audited_reader_production(source, region) {
            Err(error) => assert_eq!(error, expected),
            Ok(_) => panic!("guard unexpectedly accepted control expecting {expected}"),
        }
    }
    // B7b: the late include names the one nonresident binding (the pressure
    // result schema); the pressure-source table is resident since O-10.
    const LATE_PRODUCTION: &str = "\nconst LATE_REVIEWED_TABLE: &str = include_str!(\"../../../../schemas/results_pressure_2.v0.3.schema.json\");\n";
    let mut unchanged = 0;
    let mut refused = 0;
    let mut visible = 0;
    for (source, region) in audited_reader_sources() {
        let production = audited_reader_production(source, region).unwrap();
        unchanged += 1;
        // Preserve the historical 5/29/2 controls. The semantic marker-free
        // fixture is now the production projection, only after the full actual
        // suffix (B7b's table tests, production, E14) passed the exact audit
        // above; the file itself is not marker-free. Its regions' own controls
        // are `semantic_e14_reviewed_guard_refuses_changed_or_hidden_production`.
        if let Some(region) = region.filter(|r| r.trailing_test.is_none()) {
            let start = source.find(region.first_item).unwrap();
            let (prefix, suffix) = source.split_at(start);
            assert!(production.starts_with(prefix));
            for inserted in ["// #[cfg(test)] in a comment\n",
                "const EARLY_MARKER: &str = \"#[cfg(test)]\";\n",
                "#[cfg(test)]\nmod early_test_module {}\n"] {
                refuses(&format!("{prefix}{inserted}{suffix}"), Some(region), "EARLIER_TEST_MARKER");
                refused += 1;
            }
            // Commenting the known attribute makes its module ungated. A text
            // occurrence at the same boundary must not pass the item-line check.
            refuses(&format!("{prefix}// {suffix}"), Some(region), "TEST_BOUNDARY_NOT_ITEM_LINE");
            refused += 1;
            let missing = source.replacen(region.first_item, "#[cfg(not(test))]\nmod changed_test_module {", 1);
            refuses(&missing, Some(region), "MISSING_OR_AMBIGUOUS_TEST_BOUNDARY"); refused += 1;
            let ambiguous = format!("{prefix}{}\n}}\n{suffix}", region.first_item);
            refuses(&ambiguous, Some(region), "MISSING_OR_AMBIGUOUS_TEST_BOUNDARY"); refused += 1;
            refuses(&format!("{source}\n// altered audited suffix\n"), Some(region), "CHANGED_AUDITED_SUFFIX"); refused += 1;
            refuses(&format!("{source}{LATE_PRODUCTION}"), Some(region), "CHANGED_AUDITED_SUFFIX"); refused += 1;
            if let Some(anchor) = region.production_resume {
                // These are actual ungated production callers, following the
                // audited tests in the current retained reader; neither is hidden.
                assert!(production.contains("pub fn outcome_json("));
                assert!(production.contains("pub fn validate_preview_physics_transport_metadata("));
                let after_tests = source.replacen(anchor, &format!("{LATE_PRODUCTION}{anchor}"), 1);
                refuses(&after_tests, Some(region), "CHANGED_AUDITED_SUFFIX"); refused += 1;
            } else {
                assert_eq!(production.len(), prefix.len());
            }
        } else {
            let marker_free = if region.is_some() { production.as_str() } else { source };
            assert!(production == marker_free, "the concrete marker-free fixture remains complete production");
            for inserted in ["// #[cfg(test)] in a comment\n",
                "const EARLY_MARKER: &str = \"#[cfg(test)]\";\n"] {
                refuses(&format!("{marker_free}{inserted}"), None, "UNKNOWN_TEST_MARKER"); refused += 1;
            }
            let appended = audited_reader_production(&format!("{marker_free}{LATE_PRODUCTION}"), None).unwrap();
            assert!(actual_reader_include(&appended, PRESSURE_SCHEMA_BINDING_INPUT), "late production remains visible to the real include guard");
            visible += 1;
        }
    }
    assert_eq!((unchanged, refused, visible), (5, 29, 2));
    println!("reviewed_reader_guard_controls=5unchanged,29refused,2visible_late_production");
}

/// The semantic reader's two closed test regions (B7b D3): `b7b_current_table_tests`, then
/// production from `for_source_metadata` on, then E14 to the end of the file. Only the two
/// test regions are excluded, after the complete suffix from the first passed the exact audit.
#[test]
fn semantic_e14_reviewed_guard_refuses_changed_or_hidden_production() {
    let (source, region) = audited_reader_sources()[2];
    let region = region.expect("the exact semantic suffix is audited");
    assert_eq!(region.first_item, SEMANTIC_B7B_TEST_ITEM);
    assert_eq!(region.trailing_test, Some(SEMANTIC_E14_TEST_ITEM));
    let anchor = region.production_resume.expect("production resumes between the test regions");
    let production = audited_reader_production(source, Some(region)).unwrap();
    let start = source.find(region.first_item).unwrap();
    let resume = source.find(anchor).unwrap();
    let trailing = source.find(SEMANTIC_E14_TEST_ITEM).unwrap();
    assert!(start < resume && resume < trailing);
    let (prefix, suffix) = source.split_at(start);
    assert_eq!(production, format!("{prefix}{}", &source[resume..trailing]));
    assert!(!production.contains("#[cfg(test)]"));
    assert!(source[start..resume].contains("fn b7b_current_tables_are_independent_and_bind_their_own_bytes()"));
    assert!(source[trailing..].contains("fn e14_no_table_carries_the_retired_parity_kind()"));
    for item in ["pub fn for_source_metadata(", "fn current_header(", "pub fn pressure_source_contract(", "pub fn pressure_rows_contract("] {
        assert!(production.contains(item), "{item} stays inspected");
    }
    let refuses = |candidate: String, region: AuditedTestRegion, expected| {
        assert_eq!(audited_reader_production(&candidate, Some(region)).unwrap_err(), expected);
    };
    let mut refused = 0;
    for inserted in ["// #[cfg(test)] in a comment\n",
        "const EARLY_MARKER: &str = \"#[cfg(test)]\";\n",
        "#[cfg(test)]\nmod early_test_module {}\n"] {
        refuses(format!("{prefix}{inserted}{suffix}"), region, "EARLIER_TEST_MARKER"); refused += 1;
    }
    refuses(format!("{prefix}// {suffix}"), region, "TEST_BOUNDARY_NOT_ITEM_LINE"); refused += 1;
    refuses(source.replacen(region.first_item, "#[cfg(not(test))]\nmod changed_test_module {", 1), region, "MISSING_OR_AMBIGUOUS_TEST_BOUNDARY"); refused += 1;
    refuses(format!("{prefix}{}\n}}\n{suffix}", region.first_item), region, "MISSING_OR_AMBIGUOUS_TEST_BOUNDARY"); refused += 1;
    refuses(format!("{source}\n// altered audited suffix\n"), region, "CHANGED_AUDITED_SUFFIX"); refused += 1;
    for path in [PRESSURE_SOURCE_BINDING_INPUT, PRESSURE_SCHEMA_BINDING_INPUT] {
        let late = format!("\nconst LATE_REVIEWED_TABLE: &str = include_str!(\"../../{path}\");\n");
        // Inside the first excluded region, before the trailing one, or after it.
        refuses(source.replacen(anchor, &format!("{late}{anchor}"), 1), region, "CHANGED_AUDITED_SUFFIX"); refused += 1;
        refuses(source.replacen(SEMANTIC_E14_TEST_ITEM, &format!("{late}{SEMANTIC_E14_TEST_ITEM}"), 1), region, "CHANGED_AUDITED_SUFFIX"); refused += 1;
        refuses(format!("{source}{late}"), region, "CHANGED_AUDITED_SUFFIX"); refused += 1;
        let visible = audited_reader_production(&format!("{production}{late}"), None).unwrap();
        assert!(actual_reader_include(&visible, path), "actual late production stays visible");
    }
    // The trailing boundary is exact: absent, not an item line, or without a resume.
    refuses(source.into(), AuditedTestRegion { trailing_test: Some("#[cfg(test)]\nmod absent_tests {"), ..region }, "MISSING_OR_AMBIGUOUS_TRAILING_TEST"); refused += 1;
    refuses(source.into(), AuditedTestRegion { trailing_test: Some("e14_tests {"), ..region }, "TRAILING_TEST_NOT_ITEM_LINE"); refused += 1;
    refuses(source.into(), AuditedTestRegion { production_resume: None, ..region }, "TRAILING_TEST_WITHOUT_RESUME"); refused += 1;
    assert_eq!(refused, 16);
    println!("semantic_test_region_guard_controls=1unchanged,{refused}refused,2visible_late_production");
}

/// B7b (O-10): the current pressure-source table is resident, so the pressure result schema is
/// the one nonresident binding. The source table, the schema and the resident ordinary pressure
/// table are each bound by their reviewed digest; a missing, removed or changed token refuses.
#[test]
fn pressure_schema_is_the_only_nonresident_binding() {
    use open_pipe_stress_result_export::{reader_verdict, semantic_contract as sc};
    use sha2::{Digest, Sha256};
    assert_reviewed_reader_input_bindings();
    // A same-basename or similar family path does not acquire this exemption, and the
    // now-resident pressure-source table has none.
    for path in [PRESSURE_SOURCE_BINDING_INPUT, PRESSURE_TABLE_INPUT,
        "../../schemas/semantic_contract_v0_3_pressure_source_1.json",
        "fixtures/results/semantic_contract_v0_3_pressure_source_1.json",
        "../../fixtures/results/results_pressure_2.v0.3.schema.json",
        "schemas/results_pressure_2.v0.3.schema.json",
        "../../schemas/results_pressure_2.v0.3.schema.yaml"] {
        assert!(reader_include_required(path), "{path}");
    }
    let bytes = include_bytes!("../../../fixtures/results/semantic_contract_v0_3_pressure_source_1.json");
    assert_eq!(format!("{:x}", Sha256::digest(bytes)), sc::PRESSURE_SOURCE_TABLE_SHA256);
    let table = sc::verify_pressure_source_table(bytes).unwrap();
    assert_eq!(&table, sc::pressure_source_contract(), "the resident table is the reviewed bytes");
    assert_eq!(reader_verdict::binding("pressure_source"), json!({"id":sc::PRESSURE_SOURCE_ID,"sha256":sc::PRESSURE_SOURCE_TABLE_SHA256}));
    let mut edited = bytes.to_vec(); edited.push(b'\n');
    assert_eq!(sc::verify_pressure_source_table(&edited).unwrap_err(), "SOURCE_PRESSURE_SOURCE_TABLE_HASH");
    let ordinary = include_bytes!("../../../fixtures/results/semantic_contract_v0_3_pressure_2.json");
    assert_eq!(format!("{:x}", Sha256::digest(ordinary)), sc::PRESSURE_TABLE_SHA256);
    assert_eq!(&sc::verify_pressure_table(ordinary).unwrap(), sc::pressure_rows_contract());
    let schema_bytes = include_bytes!("../../../schemas/results_pressure_2.v0.3.schema.json");
    let schema_hash = format!("{:x}", Sha256::digest(schema_bytes));
    // B7b A5: the result schema takes v3 only. T4-U6 (462b777b22): it carries the D-5
    // geometry, per-basis records and limitations (reviewed input #10, re-bound at D3).
    assert_eq!(schema_hash, "ce085117a3501ecc87e08afa478e187c770cc11057b9d9c637022c8ec3fea9b9");
    let compiled = COMPILED_REVIEWED_INPUTS.expect("build.rs sets all current inputs");
    let tokens: Vec<_> = compiled.split(';').collect();
    assert_eq!(tokens.len(), build_identity::REVIEWED_INPUT_COUNT + 1);
    assert_eq!(tokens[0], "v1");
    let bound = [(9, PRESSURE_SOURCE_BINDING_INPUT, sc::PRESSURE_SOURCE_TABLE_SHA256),
        (10, PRESSURE_SCHEMA_BINDING_INPUT, schema_hash.as_str()),
        (11, PRESSURE_TABLE_INPUT, sc::PRESSURE_TABLE_SHA256)];
    for (index, path, hash) in bound {
        assert_eq!(tokens[index + 1], format!("{path}={hash}"));
    }
    let digests = [Some([7; 32]); build_identity::REVIEWED_INPUT_COUNT];
    let read = build_identity::encode_reviewed_inputs(&digests);
    assert!(bindings_hold(true, Some(&read), &read, &READER_LAYOUTS, &READER_LAYOUTS));
    for (index, path, _) in bound {
        let token = read.split(';').nth(index + 1).unwrap();
        assert_eq!(token, format!("{path}={}", "07".repeat(32)));
        // Retain the pressure-source 134/81 token-byte proof and each other
        // path's same wire-byte accounting.
        let available_bytes = token.len() + 1;
        assert_eq!(available_bytes, path.len() + 66);
        let mut unavailable = digests; unavailable[index] = None;
        let unavailable = build_identity::encode_reviewed_inputs(&unavailable);
        let unavailable_bytes = unavailable.split(';').nth(index + 1).unwrap().len() + 1;
        assert_eq!(unavailable_bytes, path.len() + 13);
        if path == PRESSURE_SOURCE_BINDING_INPUT { assert_eq!((available_bytes, unavailable_bytes), (134, 81)); }
        assert!(!bindings_hold(true, Some(&unavailable), &read, &READER_LAYOUTS, &READER_LAYOUTS));
        let removed = read.split(';').enumerate().filter(|(i, _)| *i != index + 1)
            .map(|(_, token)| token).collect::<Vec<_>>().join(";");
        assert!(!bindings_hold(true, Some(&removed), &read, &READER_LAYOUTS, &READER_LAYOUTS));
        let mut changed = digests; changed[index] = Some([8; 32]);
        let changed = build_identity::encode_reviewed_inputs(&changed);
        assert!(!bindings_hold(true, Some(&changed), &read, &READER_LAYOUTS, &READER_LAYOUTS));
        let resident = NONRESIDENT_BINDING_INPUTS.iter().all(|(_, p)| *p != path);
        println!("pressure_binding_index={index},resident={resident},available_bytes={available_bytes},unavailable_bytes={unavailable_bytes}");
    }
}
#[test]
fn reviewed_inputs_bind_the_lock_and_the_reader_statics() {
    use sha2::{Digest, Sha256};
    // Current assembled inputs (T4-U6/U8 D3 registration at addbdc030e): the
    // lock and resident obligations in their preserved order, then the resident
    // pressure-source table, the nonresident pressure result schema and the
    // resident ordinary pressure table. #1 and #7-#11 carry D-5's section bases
    // (DEL-05-03 Design §9). Every digest below checks actual bytes. Model 0.7.0
    // (DEL-04-07 Design `u4b-u5b-implementation.md` §1.3) moved #3-#11
    // (the -2 tables, the renamed schema, the re-inherited tables, the
    // definitions and the version sentence of the pressure-source table); T4-S7's D3
    // re-registration at b4c9f35f8e records these bytes. T4-U4b S4 (§1.3, §4.1, §4.3)
    // moves #1 (the `derived_not_qualified` treatment and its two recipes), #3 DEF-O, #4
    // (the preview-retained table's U4b rows and texts), #6 DEF-C, #7 DEF-E, #8 (DEF-E's
    // hash in the physics-retained table) and #9 (the pressure-source table's signatures
    // and disclosure). O4 (DEL-04-08 Design `o4-source-extension.md` §4.6-§4.9; retained-pricing-rules.md
    // §8.9) moves #2 (both section shapes of the retained-precision schema), #7 DEF-E and #8 (DEF-E's
    // hash in the physics-retained table), counted from 0; O4's D3 registration at 7823a79f45 records
    // these bytes. #0, #1 and #9-#11 keep theirs.
    // W-7 (DEL-04-01 Design `global-end-actions.md`; retained-pricing-rules.md §8.20) moves #4, #5,
    // #8, #10 and #11 (the tables' six global end signatures, carried to the inheriting tables and
    // the pressure-2 result schema's closed basis enum), counted from 0; W-7's D3 registration
    // records these bytes.
    const REVIEWED: [&str; build_identity::REVIEWED_INPUT_COUNT] = [
        "e27b28f710ea6b9cc26bb14d0de53c64c2504c5dde17cbf0acc50dfac9220a11",
        "ae45b97742fd463866d8f231004f2053834d5bd413a1430e766f24516d3569ac",
        "73bebc5fb562b7f519c7d59be8e7830f98b21ce29fa0b7c017e43426f50cfcaa",
        "a056c395e27f964c842a1347db689eb24377d0b5a79a3b954435105a2c422d08",
        "b001070e5fedeeb101522b22ce0d61bb7cf85f82e86b8d851713f8d9fc8c4c46",
        "1ccc9561cb4ce59d047f3dab7927a927c44a707efb3acb90e7b07a21b3bad0a2",
        "fe371e07cd727154235f3725eeeef393ad0a433f0e8c98656f98468940d9aea2",
        "c7bcf0ceb82372fbf5f9958446ffe3f9c1ca3eb2384c47663514d21f89a885bb",
        "33cc0c4dc1d7dea07e14c16e07f4ee3f93cc2f84e4dd0494e232397d4e3861c1",
        "609a4e92ac19de0f5a1dd067dab64b4abeb7b3b70511e2095455f1d000d28e2c",
        "ce085117a3501ecc87e08afa478e187c770cc11057b9d9c637022c8ec3fea9b9",
        "be61c6ea828797ad5664d06a2c835b28deffaff07cc33fec6bb4d48a8fcb0919",
    ];
    // The same files, read here at compile time (test-only), by the sha2 dependency.
    let files: [&[u8]; build_identity::REVIEWED_INPUT_COUNT] = [
        include_bytes!("../Cargo.lock"),
        include_bytes!("../../../schemas/physics_source_recovery.schema.json"),
        include_bytes!("../../../schemas/retained_precision_mp_v2.schema.json"),
        include_bytes!("../../../fixtures/results/retained_precision_prepared_ordinary_v1.json"),
        include_bytes!("../../../fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json"),
        include_bytes!("../../../fixtures/results/semantic_contract_v0_3_preview_physics_2.json"),
        include_bytes!("../../../fixtures/results/retained_precision_prepared_combination_v1.json"),
        include_bytes!("../../../fixtures/results/retained_precision_prepared_exact_v1.json"),
        include_bytes!("../../../fixtures/results/semantic_contract_v0_3_physics_retained_1.json"),
        include_bytes!("../../../fixtures/results/semantic_contract_v0_3_pressure_source_1.json"),
        include_bytes!("../../../schemas/results_pressure_2.v0.3.schema.json"),
        include_bytes!("../../../fixtures/results/semantic_contract_v0_3_pressure_2.json"),
    ];
    let compiled = COMPILED_REVIEWED_INPUTS.expect("build.rs sets the reviewed inputs");
    assert_eq!(option_env!("OPS_RETAINED_REVIEWED_INPUTS"), Some(compiled));
    let mut tokens = compiled.split(';');
    assert_eq!(tokens.next(), Some("v1"));
    for (i, token) in tokens.by_ref().take(build_identity::REVIEWED_INPUT_COUNT).enumerate() {
        let (path, hex) = token.split_once('=').unwrap();
        assert_eq!(path, build_identity::REVIEWED_INPUTS[i]);
        assert_eq!(hex, format!("{:x}", Sha256::digest(files[i])), "{path}: build.rs's digest");
        assert_eq!(hex, REVIEWED[i], "{path}: G4's reviewed record");
    }
    assert_eq!(tokens.next(), None);
    println!("I65_G5_REVIEWED_INPUTS {compiled}");
    assert_reviewed_reader_input_bindings();
}

#[test]
fn the_registered_profile_is_the_only_permit_source() {
    // G6: exactly one production profile, registered by reviewed change (decision 7: no test permit).
    assert_eq!(REGISTERED_PROFILES.len(), 1);
    assert_eq!(REGISTERED_PROFILES[0].identity, super::law_tests::PINNED_RECORD_IDENTITY, "the qualified build is the pinned record's");
    assert_eq!(REGISTERED_PROFILES[0].threshold_bytes, M, "current product M = 12 GiB");
    let source = include_str!("retained_memory.rs");
    let production = &source[..source.find("#[cfg(test)]\n#[path = \"retained_memory_law_tests.rs\"]").unwrap()];
    assert_eq!(production.matches("RegisteredProfile {").count(), 2, "the definition and the one registered entry: no other literal");
    assert_eq!(production.matches("CapturePermit { _profile").count(), 1, "admission's one construction");
    assert!(production.contains("(None, Some(profile)) => Ok(CapturePermit { _profile: profile }),"));
    // A forged index past the list, or a refusal, never mints a permit.
    let mut report = admitted(milestone());
    report.law.refusal = None;
    report.law.registered = Some(1);
    assert!(admission(report).is_err());
    let mut refused = admitted(milestone());
    refused.law.refusal = Some(AdmissionRefusal::Bound(BoundRefusal::Unpriced));
    refused.law.registered = Some(0);
    assert!(admission(refused).is_err());
    // Toolchain identity alone does not bind changed reviewed inputs or reader layouts.
    // This build is registered only when that complete binding holds, else Stale.
    let profile = &REGISTERED_PROFILES[0];
    let registered = COMPILED_IDENTITY == Some(profile.identity)
        && bindings_hold(LAYOUT_WITNESSES, COMPILED_REVIEWED_INPUTS, profile.reviewed_inputs,
            &READER_LAYOUTS, &profile.reader_layouts);
    if registered {
        assert_eq!(build_status(), Ok(0));
    } else {
        assert_eq!(build_status(), Err(ProfileStatus::Stale));
    }
}

/// G6 (brief step 5): under the registered build, `admit` grants a permit for the in-domain
/// milestone Direct invocation in both modes; Headless and out-of-domain requests stay refused.
#[test]
fn admit_grants_a_permit_for_the_milestone_in_the_registered_build() {
    let profile = &REGISTERED_PROFILES[0];
    let registered = COMPILED_IDENTITY == Some(profile.identity)
        && bindings_hold(LAYOUT_WITNESSES, COMPILED_REVIEWED_INPUTS, profile.reviewed_inputs,
            &READER_LAYOUTS, &profile.reader_layouts);
    for mode in MODES {
        let (request, capture) = CapturedInvocation::parse(milestone(), mode).unwrap();
        let admitted = admit(&capture, &request, Entry::Direct, RETAINED_LIMIT);
        if !registered {
            let report = admitted.err().expect("an unregistered build is refused");
            assert_eq!(report.law().refusal, Some(AdmissionRefusal::Profile(ProfileStatus::Stale)));
            assert_eq!(report.law().required, None, "refused before the bound: nothing computed");
            continue;
        }
        let (_permit, report) = admitted.unwrap_or_else(|r| panic!("{mode:?}: refused {:?}", r.law().refusal));
        assert_eq!(report.law().refusal, None);
        assert_eq!(report.law().registered, Some(0));
        let required = cap_priced_maximum(mode).unwrap() + RESERVED_STACK_BYTES as u64;
        // RV89 G6 S-3: admission's own bound is the mode's maximum plus R (64 MiB).
        assert_eq!(report.law().required, Some(required), "{mode:?}: admit adds R before comparing with M");
        assert!(required <= MARGIN, "the 0.9 M margin holds at admission");
        // Headless is refused at D1.0 even in the registered build.
        let raw = milestone();
        let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
        let id = String::from("g6");
        let headless = admit(&capture, &request, Entry::Headless(RetainedHeadlessContext::from_borrowed_roots(&raw, &invocation, &id)), RETAINED_LIMIT);
        assert_eq!(headless.err().unwrap().law().refusal, Some(AdmissionRefusal::Caller(RetainedCaller::Headless)));
        // An out-of-domain request (B1, PLAN_v2 §2.3 and RV107 SF-2: C + 1 load cases) is
        // refused at its D1 clause, D1.4. (G6: the blocked examples and the failed attempt
        // are in `registered_g_c_declines_only_unattempted_solves`.)
        let (request, capture) = CapturedInvocation::parse(milestone_cases(caps::LOAD_CASES + 1), mode).unwrap();
        assert_eq!(admit(&capture, &request, Entry::Direct, RETAINED_LIMIT).err().unwrap().law().refusal,
            family(D1Clause::Invocation, FamilyFact::LoadCases), "C + 1 cases");
    }
}

// ---- The census --------------------------------------------------------------

#[test]
fn raw_text_census_reads_lengths_and_control_bytes() {
    let mut map = serde_json::Map::new();
    map.insert("k\u{1f}ey".into(), json!(["a", "b\u{7f}c", "\u{0}", " ~"]));
    map.insert(text("long", 40), json!(text("v", 77)));
    let f = raw_text_census(&Value::Object(map));
    assert_eq!(f.status, CensusStatus::Complete);
    assert_eq!((f.max_key_bytes, f.max_string_bytes, f.control_bytes), (40, 77, 3));
    let mut deep = Value::Null;
    for _ in 0..DEPTH_LIMIT + 2 {
        deep = Value::Array(vec![deep]);
    }
    assert_eq!(raw_text_census(&deep).status, CensusStatus::DepthLimit);
    assert_eq!(raw_text_census(&Value::Array(vec![Value::Null; VALUE_LIMIT])).status, CensusStatus::ValueLimit);
    assert_eq!(raw_text_census(&Value::Array(vec![Value::Null; VALUE_LIMIT - 1])).status, CensusStatus::Complete);
}

#[test]
fn nested_typed_census_reads_the_roster() {
    let raw = cap_maximal();
    let request: LinearStaticPreviewRequest = serde_json::from_value(raw).unwrap();
    let f = nested_typed_census(&request);
    assert_eq!(f.status, CensusStatus::Complete);
    assert_eq!((f.restraints, f.springs, f.primitive_loads.length, f.total_loads), (192, 32, 128, 128));
    assert_eq!(f.max_temperature_points.length, 16);
    assert_eq!(f.max_string_bytes, 128, "the project id");
    assert!(f.max_string_capacity >= 128 && f.string_capacity_bytes >= f.strings);
    assert_eq!(f.units.values, 3);
    // Strings are counted once each: 32 nodes × 2 + 32 pipes × 7 + 32 supports × 11 + …
    let mut fewer = request.clone();
    fewer.model.nodes[0].provenance = None;
    assert_eq!(nested_typed_census(&fewer).strings + 1, f.strings);
    let milestone: LinearStaticPreviewRequest = serde_json::from_value(milestone()).unwrap();
    let m = nested_typed_census(&milestone);
    assert_eq!((m.status, m.primitive_loads.length, m.restraints), (CensusStatus::Complete, 3, 6));
    // B1 SA: the per-case load facts are the maxima over every case, and the total is their sum.
    let three: LinearStaticPreviewRequest = serde_json::from_value(cap_maximal_cases(3)).unwrap();
    let t = nested_typed_census(&three);
    assert_eq!((t.status, t.primitive_loads.length, t.primitive_loads.capacity, t.total_loads), (CensusStatus::Complete, 128, 128, 384));
    let mut uneven = milestone_cases(3);
    let extra = uneven["model"]["load_cases"][0]["primitive_loads"][0].clone();
    uneven["model"]["load_cases"][1]["primitive_loads"].as_array_mut().unwrap().push(extra);
    let mut uneven: LinearStaticPreviewRequest = serde_json::from_value(uneven).unwrap();
    uneven.model.load_cases[2].primitive_loads.reserve_exact(40);
    let u = nested_typed_census(&uneven);
    assert_eq!((u.primitive_loads.length, u.total_loads), (4, 3 + 4 + 3), "the second case is the longest");
    assert_eq!(u.primitive_loads.capacity, uneven.model.load_cases[2].primitive_loads.capacity(), "the third case has the largest capacity");
    assert!(u.primitive_loads.capacity >= 43);
}

// ---- D1 --------------------------------------------------------------------

#[test]
fn milestone_and_cap_maximal_inputs_are_inside_d1() {
    for raw in [milestone(), cap_maximal()] {
        for mode in MODES {
            let (request, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
            let report = assess(&capture, &request, Entry::Direct, RETAINED_LIMIT);
            assert_eq!(report.law().domain, None, "inside D1");
            assert_eq!(report.law().refusal, d1_1_refusal(), "D1.1: the qualified build admits; any other is Stale");
            assert_eq!(report.profile, build_profile());
            assert_eq!(report.law().raw_text.control_bytes, 0);
            let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
            let id = String::from("g5");
            let headless = assess(&capture, &request, Entry::Headless(RetainedHeadlessContext::from_borrowed_roots(&raw, &invocation, &id)), RETAINED_LIMIT);
            assert_eq!(headless.law().refusal, Some(AdmissionRefusal::Caller(RetainedCaller::Headless)), "D1.0 first: Headless is refused");
            assert_eq!(headless.law().refusal.unwrap().precondition().as_str(), "caller");
        }
    }
}

/// Stage 2's widened count boundary: preserve the nodal cap helper and replace
/// only this additive input's leaves. This is an admission-law control, not a
/// claim that the fully restrained ring publishes a retained successor.
#[test]
fn w1_three_basis_member_cap_input_is_inside_d1_and_refuses_129th_term() {
    use w1_member_load_inputs as member;
    let raw = member::three_basis_member_cases(cap_maximal_cases(3));
    for mode in MODES {
        let (request, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
        let report = assess(&capture, &request, Entry::Direct, RETAINED_LIMIT);
        let law = report.law();
        assert_eq!(law.domain, None, "{mode:?}: widened input is inside D1");
        assert_eq!(law.refusal, d1_1_refusal(), "registration remains build-specific");
        assert_eq!(report.profile, build_profile());
        assert_eq!((request.model.nodes.len(), request.model.pipe_segments.len(), request.model.supports.len()), (32, 32, 32));
        assert_eq!((request.model.materials.len(), request.materials.len()), (4, 4));
        assert_eq!((law.nested.restraints, law.nested.springs, law.nested.max_temperature_points.length), (192, 32, 16));
        assert_eq!((report.typed.load_cases.length, law.nested.primitive_loads.length,
            law.nested.primitive_loads.capacity, law.nested.total_loads), (3, 128, 128, 384));
        for (case, authored) in request.model.load_cases.iter().enumerate() {
            assert_eq!(authored.modulus_basis_ref, Some(member::basis_id(case)));
            assert_eq!(authored.primitive_loads.len(), 128);
            assert!(authored.primitive_loads.iter().all(|load| matches!(load.target, crate::LoadTargetInput::Element { .. })));
            for material in &request.model.materials {
                let point = material.temperature_points.iter().find(|point| point.id == member::basis_id(case)).unwrap();
                assert_eq!(point.elastic_modulus.as_ref().unwrap().value,
                    material.elastic_modulus.value * member::BASIS_SCALES[case]);
                assert_eq!(point.shear_modulus.as_ref().unwrap().value,
                    material.shear_modulus.as_ref().unwrap().value * member::BASIS_SCALES[case]);
            }
        }
        let mut over = raw.clone();
        let mut extra = over["model"]["load_cases"][2]["primitive_loads"][127].clone();
        extra["id"] = json!("w1-member:2:128");
        over["model"]["load_cases"][2]["primitive_loads"].as_array_mut().unwrap().push(extra);
        let (request, capture) = CapturedInvocation::parse(over, mode).unwrap();
        let over = assess(&capture, &request, Entry::Direct, RETAINED_LIMIT);
        assert_eq!(over.law().nested.total_loads, 385);
        let expected = Some(AdmissionRefusal::Cap { fact: CapFact::Loads, observed: 129, cap: 128 });
        assert_eq!(over.law().domain, expected, "{mode:?}: member and nodal terms share l=128");
        assert_eq!(over.law().refusal, d1_1_refusal().or(expected));
        println!("W1_MEMBER_CAP_LAW {mode:?} profile={:?} nodes=32 members=32 supports=32 bases=3 cases=3 terms=384 member_129_refused=true", report.profile);
    }
}

#[test]
fn every_family_clause_refuses_with_its_fact() {
    use D1Clause as C;
    use FamilyFact as F;
    let with = |change: &dyn Fn(&mut Value)| {
        let mut raw = milestone();
        change(&mut raw);
        domain(raw)
    };
    // B3-D §4.1, after B7b (A4/A5, D4 (a)): branch E is 0.8.0 with the exact v3 contract, so
    // 0.3.0 is retired with v2 and, like every schema in no branch, refuses with
    // `SchemaVersion`; any other contract on 0.8.0 with `PressureContract`.
    assert_eq!(with(&|r| r["model"]["schema_version"] = json!("0.3.0")), family(C::Namespace, F::SchemaVersion), "0.3.0 is retired");
    assert_eq!(with(&|r| r["model"]["schema_version"] = json!("0.4.0")), family(C::Namespace, F::SchemaVersion));
    assert_eq!(with(&|r| r["model"]["schema_version"] = json!("0.8.0")), None, "0.8.0 without a contract is in the namespace");
    assert_eq!(with(&|r| r["model"]["schema_version"] = json!("0.2.0")), family(C::Namespace, F::SchemaVersion), "0.2.0 is retired");
    assert_eq!(with(&|r| r["model"]["pressure_contract"] = json!({})), family(C::Namespace, F::PressureContract));
    assert_eq!(with(&|r| r["model"]["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3"})),
        family(C::Namespace, F::PressureContract), "the exact v3 contract names its shear model");
    assert_eq!(with(&|r| r["model"]["reference_configurations"] = Value::Null), family(C::Namespace, F::ReferenceConfigurations));
    assert_eq!(with(&|r| r["model"]["materials"][0]["expansion_laws"] = Value::Null), family(C::Namespace, F::MaterialExpansionLaw));
    assert_eq!(with(&|r| r["materials"] = json!([{"id": "m", "elastic_modulus": {"value": 1.0, "unit": "Pa"}, "expansion_laws": null}])),
        family(C::Namespace, F::RequestExpansionLaws));
    assert_eq!(with(&|r| r["model"]["sections"] = json!([{"id": "s", "name": "s", "section_type": "pipe", "properties": {}, "provenance": null}])),
        family(C::Namespace, F::Sections));
    assert_eq!(with(&|r| r["model"]["pipe_segments"][0]["section_ref"] = json!("s")), family(C::Namespace, F::SectionRef));
    // D1.4 (B1 SA, PLAN_v2 §2.3 and RV107 SF-2): C + 1 cases and 0 cases refuse.
    assert_eq!(with(&|r| *r = with_cases(r.clone(), caps::LOAD_CASES + 1)), family(C::Invocation, F::LoadCases), "C + 1 cases");
    assert_eq!(with(&|r| r["model"]["load_cases"] = json!([])), family(C::Invocation, F::LoadCases), "0 cases");
    // B2-A (B2-C §9): a combination is inside D1.4; one whose id is a load case's is not (C-9).
    assert_eq!(with(&|r| r["model"]["combinations"] = json!([{"id": "c", "basis": "mechanics", "terms": [{"load_case": "case", "factor": 1.0}]}])),
        None);
    assert_eq!(with(&|r| r["model"]["combinations"] = json!([{"id": "case", "basis": "mechanics", "terms": [{"load_case": "case", "factor": 1.0}]}])),
        family(C::Invocation, F::CombinationIds));
    assert_eq!(with(&|r| r["model"]["components"] = json!([{"id": "k", "kind": "elbow", "node": "N0"}])), family(C::Invocation, F::Components));
    let case = |field: &'static str, value: Value| move |r: &mut Value| r["model"]["load_cases"][0][field] = value.clone();
    assert_eq!(with(&case("pressure_regions", json!([]))), family(C::Case, F::PressureRegions));
    assert_eq!(with(&case("equivalent_static", json!({}))), family(C::Case, F::EquivalentStatic));
    // B7a (b) Widen domain, stage 1: a named-point basis is inside D1.5 on branch L; a solve
    // temperature still refuses.
    assert_eq!(with(&case("modulus_basis_ref", json!("T0"))), None);
    assert_eq!(with(&case("modulus_basis_temperature", json!({"value": 20.0, "unit": "degC"}))), family(C::Case, F::ModulusBasisTemperature));
    assert_eq!(with(&case("analysis_state", Value::Null)), family(C::Case, F::AnalysisState));
    let support = |field: &'static str, value: Value| move |r: &mut Value| r["model"]["supports"][1][field] = value.clone();
    assert_eq!(with(&support("hanger", json!({}))), family(C::Supports, F::Hanger));
    assert_eq!(with(&support("nonlinear", json!({"behavior": "gap", "dof": "UY"}))), family(C::Supports, F::Nonlinear));
    assert_eq!(with(&support("family", json!(" anchor"))), family(C::Supports, F::SupportFamily), "exact strings, no trim");
    for family_name in ["anchor", "guide", "line_stop", "vertical_support", "spring"] {
        assert_eq!(with(&support("family", json!(family_name))), None, "{family_name}");
    }
    let load = |field: &'static str, value: Value| move |r: &mut Value| r["model"]["load_cases"][0]["primitive_loads"][2][field] = value.clone();
    // B7a (b) Widen domain, stage 1: on branch L a full-span uniform member load, an element
    // `distributed_force` in `force_per_length` along a global axis, is inside D1.7; any other
    // element form refuses by its fact, and so does the deterministic self-weight id.
    let element = |direction: &'static str| move |r: &mut Value| {
        r["model"]["load_cases"][0]["primitive_loads"][2] = json!({"id": "udl", "category": "distributed_force",
            "target": {"type": "element", "pipe": "M1"}, "direction": direction,
            "magnitude": {"value": 10.0, "unit": "N/m"}, "dimension": "force_per_length"});
    };
    for direction in ["global_x", "global_y", "global_z"] {
        assert_eq!(with(&element(direction)), None, "{direction}");
    }
    for direction in ["RX", "UY", "local_y", "global_y "] {
        assert_eq!(with(&element(direction)), family(C::Loads, F::LoadDirection), "{direction}");
    }
    assert_eq!(with(&load("target", json!({"type": "element", "pipe": "M1"}))), family(C::Loads, F::LoadDimension), "a concentrated element load");
    // A global_y member load with one field changed.
    let element_with = |field: &'static str, value: Value| move |r: &mut Value| {
        element("global_y")(r);
        r["model"]["load_cases"][0]["primitive_loads"][2][field] = value.clone();
    };
    assert_eq!(with(&element_with("dimension", json!("force"))), family(C::Loads, F::LoadDimension));
    assert_eq!(with(&element_with("category", json!("weight"))), family(C::Loads, F::LoadDimension));
    let case_id = milestone()["model"]["load_cases"][0]["id"].as_str().unwrap().to_owned();
    let self_weight = format!("load:self-weight:{}:{case_id}:2:M1", case_id.len());
    assert_eq!(with(&element_with("id", json!(self_weight))), family(C::Loads, F::GeneratedLoadIdentity));
    for near in [format!("{self_weight}x"), format!("load:self-weight:{}:{case_id}:3:M1", case_id.len()), "load:self-weight:".to_owned()] {
        assert_eq!(with(&element_with("id", json!(near))), None, "{near}");
    }
    assert_eq!(with(&load("dimension", json!("pressure"))), family(C::Loads, F::LoadDimension));
    assert_eq!(with(&load("dimension", json!("force"))), None);
    // W-15 slice b: a node load in the generated self-weight namespace (the component-weight
    // lineage `inspect_applied_self_weight` claims for any node id with this prefix) refuses by
    // its fact; a near id, and a weight-category nodal load under its own id, stay inside D1.7.
    for id in ["load:self-weight:", "load:self-weight:4:case:C-1:force:y", "load:self-weight:x"] {
        assert_eq!(with(&load("id", json!(id))), family(C::Loads, F::GeneratedLoadIdentity), "{id}");
    }
    for id in ["load:self-weight", "Load:self-weight:x", " load:self-weight:x", "x:load:self-weight:"] {
        assert_eq!(with(&load("id", json!(id))), None, "{id}");
    }
    assert_eq!(with(&load("category", json!("weight"))), None);
    // D1.10, after D1.9: an object provenance, with or without leading ASCII whitespace.
    for provenance in ["{\"method\":\"pipe_mass_per_length_times_explicit_axis_acceleration\"}", " \t{", "{"] {
        assert_eq!(with(&load("provenance", json!(provenance))), family(C::Provenance, F::ObjectProvenance), "{provenance:?}");
    }
    for provenance in ["x{", "[{}]", "\"{\"", ""] {
        assert_eq!(with(&load("provenance", json!(provenance))), None, "{provenance:?}");
    }
    // B1 SA: D1.5 and D1.7 apply to every case. Each fact on the last of C cases refuses
    // with its own fact, and the same request with the fact removed is inside D1.
    let last = caps::LOAD_CASES - 1;
    let on_last = |field: &'static str, value: Value| move |r: &mut Value| {
        *r = with_cases(r.clone(), caps::LOAD_CASES);
        r["model"]["load_cases"][last][field] = value.clone();
    };
    assert_eq!(with(&|r| *r = with_cases(r.clone(), caps::LOAD_CASES)), None, "C cases: inside D1");
    for (field, value, fact) in [
        ("pressure_regions", json!([]), F::PressureRegions),
        ("equivalent_static", json!({}), F::EquivalentStatic),
        ("modulus_basis_temperature", json!({"value": 20.0, "unit": "degC"}), F::ModulusBasisTemperature),
        ("analysis_state", Value::Null, F::AnalysisState),
    ] {
        assert_eq!(with(&on_last(field, value)), family(C::Case, fact), "D1.5 on case {last}: {field}");
    }
    assert_eq!(with(&on_last("modulus_basis_ref", json!("T0"))), None, "B7a (b): a named point on case {last}");
    let load_on_last = |field: &'static str, value: Value| move |r: &mut Value| {
        *r = with_cases(r.clone(), caps::LOAD_CASES);
        r["model"]["load_cases"][last]["primitive_loads"][2][field] = value.clone();
    };
    assert_eq!(with(&load_on_last("target", json!({"type": "element", "pipe": "M1"}))), family(C::Loads, F::LoadDimension), "D1.7 on case {last}");
    assert_eq!(with(&load_on_last("dimension", json!("pressure"))), family(C::Loads, F::LoadDimension), "D1.7 on case {last}");
    assert_eq!(with(&load_on_last("provenance", json!("{"))), family(C::Provenance, F::ObjectProvenance), "D1.10 on case {last}");
    // D1.8 restates D1.4 (straight members only when no components exist).
    assert!(include_str!("retained_memory.rs").contains("return refuse(C::Members, F::Components);"));
    for clause in [C::Namespace, C::Invocation, C::Case, C::Supports, C::Loads, C::Members, C::Provenance] {
        assert_eq!(AdmissionRefusal::Family(clause, F::Components).precondition().as_str(), "source_family");
    }
}

#[test]
fn every_cap_row_admits_its_cap_and_refuses_cap_plus_one() {
    let report = admitted(cap_maximal());
    let law = report.law();
    let facts = DomainFacts {
        raw: &report.raw,
        raw_text: &law.raw_text,
        typed: &report.typed,
        nested: &law.nested,
        headless: None,
        digest: &report.captured_digest,
        combinations: &NO_COMBINATIONS,
    };
    let rows = cap_rows(&facts);
    assert_eq!(first_cap_violation(&rows), Ok(()));
    for i in 0..CAP_ROWS {
        assert_eq!(rows.iter().filter(|r| r.fact == rows[i].fact).count(), 1, "{:?} once", rows[i].fact);
        let mut at = rows;
        at[i].observed = at[i].cap;
        assert_eq!(first_cap_violation(&at), Ok(()), "{:?} at its cap", rows[i].fact);
        let mut over = rows;
        over[i].observed = over[i].cap + 1;
        assert_eq!(first_cap_violation(&over), Err(AdmissionRefusal::Cap { fact: rows[i].fact, observed: rows[i].cap + 1, cap: rows[i].cap }));
        let refusal = AdmissionRefusal::Cap { fact: rows[i].fact, observed: 1, cap: 0 };
        let clause = if rows[i].fact == CapFact::ControlBytes { D1Clause::ControlBytes } else { D1Clause::Caps };
        assert_eq!((refusal.clause(), refusal.precondition().as_str()), (Some(clause), "resource_admission"));
    }
    // The cap-maximal input sits at the count caps.
    let at = |fact| rows.iter().find(|r| r.fact == fact).map(|r| (r.observed, r.cap)).unwrap();
    for fact in [CapFact::Nodes, CapFact::Members, CapFact::Supports, CapFact::Restraints, CapFact::Loads, CapFact::ModelMaterials,
        CapFact::RequestMaterials, CapFact::TemperaturePoints, CapFact::TypedTextBytes] {
        let (observed, cap) = at(fact);
        assert_eq!(observed, cap, "{fact:?}");
    }
    assert_eq!(at(CapFact::LoadsCapacity).1, 128, "l ≤ 128 (RR \"U4 G4\")");
    // B1 SA (option S3): the cap-maximal C-case input is inside D1, with C cases of l loads
    // each, so `LoadCasesCapacity` and `TotalLoads` sit at their caps too.
    assert_eq!((caps::LOAD_CASES, caps::TOTAL_LOADS), (3, 384), "C = 3 and L = 384 (RR \"I82's addendum…\")");
    assert_eq!(caps::TOTAL_LOADS, caps::LOAD_CASES * caps::LOADS, "L = C·l: stated, not binding (ADDENDUM_01 §2)");
    let report = admitted(cap_maximal_cases(caps::LOAD_CASES));
    assert_eq!(report.law().domain, None, "the cap-maximal C-case input is inside D1");
    let law = report.law();
    let facts = DomainFacts { raw: &report.raw, raw_text: &law.raw_text, typed: &report.typed, nested: &law.nested, headless: None, digest: &report.captured_digest, combinations: &NO_COMBINATIONS };
    let rows = cap_rows(&facts);
    let at = |fact| rows.iter().find(|r| r.fact == fact).map(|r| (r.observed, r.cap)).unwrap();
    for (fact, cap) in [(CapFact::LoadCasesCapacity, 3), (CapFact::Loads, 128), (CapFact::LoadsCapacity, 128), (CapFact::TotalLoads, 384)] {
        assert_eq!(at(fact), (cap, cap), "{fact:?} at its cap");
    }
    let order: Vec<CapFact> = rows.iter().map(|r| r.fact).collect();
    let loads = order.iter().position(|f| *f == CapFact::Loads).unwrap();
    assert_eq!(order[loads..loads + 3], [CapFact::Loads, CapFact::LoadsCapacity, CapFact::TotalLoads], "D1.9's load rows together");
    assert_eq!(order[CAP_ROWS - 1], CapFact::ControlBytes, "D1.11 stays the last row");
}

#[test]
fn actual_inputs_map_each_cap_fact() {
    let over = |change: &dyn Fn(&mut Value)| {
        let mut raw = cap_maximal();
        change(&mut raw);
        domain(raw)
    };
    assert!(cap(CapFact::Nodes)(over(&|r| { let n = r["model"]["nodes"][0].clone(); model(r)["nodes"].as_array_mut().unwrap().push(n) })));
    assert!(cap(CapFact::Members)(over(&|r| { let p = r["model"]["pipe_segments"][0].clone(); model(r)["pipe_segments"].as_array_mut().unwrap().push(p) })));
    assert!(cap(CapFact::Supports)(over(&|r| {
        let mut s = r["model"]["supports"][0].clone();
        s["restraints"] = json!([]);
        model(r)["supports"].as_array_mut().unwrap().push(s)
    })));
    assert!(cap(CapFact::Restraints)(over(&|r| r["model"]["supports"][0]["restraints"].as_array_mut().unwrap().push(json!("UX")))));
    assert!(cap(CapFact::Loads)(over(&|r| {
        let l = r["model"]["load_cases"][0]["primitive_loads"][0].clone();
        r["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().push(l)
    })));
    // B1 SA: every case is bounded by l, not only the first. Case 0 keeps the milestone's 3
    // loads while a later case holds l or l + 1 (Σ stays far below L).
    let later = |loads: usize| {
        let mut raw = milestone_cases(caps::LOAD_CASES);
        let load = raw["model"]["load_cases"][0]["primitive_loads"][0].clone();
        let last = &mut raw["model"]["load_cases"][caps::LOAD_CASES - 1]["primitive_loads"];
        *last = Value::Array((0..loads).map(|i| { let mut l = load.clone(); l["id"] = json!(format!("load:later:{i}")); l }).collect());
        admitted(raw)
    };
    assert_eq!(later(caps::LOADS).law().domain, None, "a later case at l");
    let report = later(caps::LOADS + 1);
    assert_eq!(report.law().domain, Some(AdmissionRefusal::Cap { fact: CapFact::Loads, observed: 129, cap: 128 }), "a later case at l + 1");
    assert_eq!(report.law().nested.total_loads as usize, (caps::LOAD_CASES - 1) * 3 + 129);
    assert!(cap(CapFact::ModelMaterials)(over(&|r| { let m = r["model"]["materials"][0].clone(); model(r)["materials"].as_array_mut().unwrap().push(m) })));
    assert!(cap(CapFact::RequestMaterials)(over(&|r| { let m = r["materials"][0].clone(); r["materials"].as_array_mut().unwrap().push(m) })));
    assert!(cap(CapFact::TemperaturePoints)(over(&|r| {
        let p = r["model"]["materials"][1]["temperature_points"][0].clone();
        r["model"]["materials"][1]["temperature_points"].as_array_mut().unwrap().push(p)
    })));
    assert!(cap(CapFact::TypedTextBytes)(over(&|r| r["model"]["nodes"][3]["provenance"] = json!(text("p", 129)))));
    // A raw-only string (a declared member the solve never reads) is bounded by the raw text row.
    assert!(cap(CapFact::RawTextBytes)(over(&|r| carry(r, json!(text("u", 129))))));
    assert!(cap(CapFact::RawKeyTextBytes)(over(&|r| carry(r, Value::Object([(text("k", 129), json!(1))].into_iter().collect())))));
    assert!(cap(CapFact::ControlBytes)(over(&|r| r["model"]["nodes"][3]["provenance"] = json!("tab\there"))));
    assert_eq!(over(&|r| r["model"]["nodes"][3]["provenance"] = json!("quote\" and \\backslash")), None, "escapes are not control bytes");
    // Raw depth 16 is admitted, 17 is not (the census counts the root as 0).
    let nested = |levels: usize| {
        let mut v = json!(1);
        for _ in 0..levels {
            v = json!([v]);
        }
        v
    };
    // `model.nodes[3].notes` sits at depth 4, so `levels` arrays put its scalar at depth levels + 4.
    assert_eq!(over(&|r| carry(r, nested(12))), None, "depth 16");
    assert!(cap(CapFact::RawDepth)(over(&|r| carry(r, nested(13)))));
    // Raw totals: string bytes and key bytes.
    assert!(cap(CapFact::RawStringBytes)(over(&|r| {
        carry(r, Value::Array((0..520).map(|i| json!(text(&format!("b{i}"), 128))).collect()))
    })));
    assert!(cap(CapFact::RawKeyBytes)(over(&|r| {
        let keys: serde_json::Map<String, Value> = (0..520).map(|i| (text(&format!("k{i}"), 128), json!(0))).collect();
        carry(r, Value::Object(keys))
    })));
    // Raw value count: past 16,384 the census itself stops (D1.2).
    let report = admitted({
        let mut raw = cap_maximal();
        carry(&mut raw, Value::Array(vec![json!(0); VALUE_LIMIT]));
        raw
    });
    assert_eq!(report.law().domain, Some(AdmissionRefusal::Census(CensusPart::Raw, CensusStatus::ValueLimit)));
    // Raw capacities: owners with spare capacity in a declared, solve-ignored member.
    let spare = |value: Value| {
        let mut raw = cap_maximal();
        carry(&mut raw, value);
        domain(raw)
    };
    let mut big = String::with_capacity(131_073);
    big.push('s');
    assert!(cap(CapFact::RawStringCapacity)(spare(Value::String(big))));
    let mut array = Vec::with_capacity(32_769);
    array.push(Value::Null);
    assert!(cap(CapFact::RawArrayCapacity)(spare(Value::Array(array))));
    let mut key = String::with_capacity(131_073);
    key.push('k');
    let mut map = serde_json::Map::new();
    map.insert(key, Value::Null);
    assert!(cap(CapFact::RawKeyCapacity)(spare(Value::Object(map))));
}

#[test]
fn typed_capacity_and_units_rows_read_the_actual_owners() {
    type Change = Box<dyn Fn(&mut LinearStaticPreviewRequest)>;
    let rows: Vec<(CapFact, Change)> = vec![
        (CapFact::NodesCapacity, Box::new(|r| r.model.nodes.reserve_exact(1))),
        (CapFact::MembersCapacity, Box::new(|r| r.model.pipe_segments.reserve_exact(1))),
        (CapFact::SupportsCapacity, Box::new(|r| r.model.supports.reserve_exact(1))),
        (CapFact::RestraintCapacity, Box::new(|r| {
            r.model.supports[0].restraints.reserve_exact(200);
        })),
        (CapFact::LoadsCapacity, Box::new(|r| r.model.load_cases[0].primitive_loads.reserve_exact(1))),
        (CapFact::ModelMaterialsCapacity, Box::new(|r| r.model.materials.reserve_exact(1))),
        (CapFact::RequestMaterialsCapacity, Box::new(|r| r.materials.reserve_exact(1))),
        (CapFact::TemperaturePointsCapacity, Box::new(|r| r.model.materials[0].temperature_points.reserve_exact(1))),
        // B1 SA: the typed capacity is capped by C, so one case with room for C more refuses.
        (CapFact::LoadCasesCapacity, Box::new(|r| r.model.load_cases.reserve_exact(caps::LOAD_CASES))),
        (CapFact::SectionsCapacity, Box::new(|r| r.model.sections.reserve_exact(1))),
        (CapFact::ComponentsCapacity, Box::new(|r| r.model.components.reserve_exact(1))),
        // B2-A: the typed capacity is capped by z's cap, so room for one more refuses.
        (CapFact::CombinationsCapacity, Box::new(|r| r.model.combinations.reserve_exact(caps::COMBINATIONS + 1))),
        (CapFact::RequestExpansionLawsCapacity, Box::new(|r| r.model.request_material_expansion_laws.reserve_exact(1))),
        (CapFact::MaterialExpansionLawsCapacity, Box::new(|r| r.model.material_expansion_laws.reserve_exact(1))),
        (CapFact::TypedTextCapacity, Box::new(|r| r.model.nodes[0].id.reserve_exact(200))),
        (CapFact::UnitsDepth, Box::new(|r| {
            let mut v = Value::Null;
            for _ in 0..17 {
                v = Value::Array(vec![v]);
            }
            r.model.project.units = v;
        })),
        (CapFact::UnitsTextBytes, Box::new(|r| r.model.project.units = json!(text("u", 129)))),
        (CapFact::UnitsKeyTextBytes, Box::new(|r| {
            let mut map = serde_json::Map::new();
            map.insert(text("k", 129), json!(1));
            r.model.project.units = Value::Object(map);
        })),
        (CapFact::UnitsStringCapacity, Box::new(|r| {
            let mut big = String::with_capacity(131_073);
            big.push('u');
            r.model.project.units = Value::String(big);
        })),
    ];
    for (fact, change) in rows {
        let report = admitted_typed(cap_maximal(), |r| change(r));
        assert!(cap(fact)(report.law().domain), "{fact:?}: {:?}", report.law().domain);
    }
    // B1 SA: C cases with exactly C slots are inside D1; one more slot refuses. A later
    // case's spare load capacity is read as its own (every case ≤ l).
    let report = admitted_typed(cap_maximal_cases(caps::LOAD_CASES), |r| r.model.load_cases.shrink_to_fit());
    assert_eq!(report.law().domain, None);
    let report = admitted_typed(cap_maximal_cases(caps::LOAD_CASES), |r| r.model.load_cases.reserve_exact(1));
    assert_eq!(report.law().domain, Some(AdmissionRefusal::Cap { fact: CapFact::LoadCasesCapacity, observed: report.typed.load_cases.capacity, cap: 3 }));
    let report = admitted_typed(milestone_cases(caps::LOAD_CASES), |r| r.model.load_cases[caps::LOAD_CASES - 1].primitive_loads.reserve_exact(126));
    assert_eq!(report.law().domain, Some(AdmissionRefusal::Cap { fact: CapFact::LoadsCapacity, observed: report.law().nested.primitive_loads.capacity, cap: 128 }),
        "the last case's capacity");
    let report = admitted_typed(milestone_cases(caps::LOAD_CASES), |r| r.model.load_cases[caps::LOAD_CASES - 1].primitive_loads.reserve_exact(125));
    assert_eq!(report.law().domain, None, "the last case's capacity at 128");
    // A units Value past the census limits is D1.2, before its cap rows.
    let report = admitted_typed(cap_maximal(), |r| r.model.project.units = Value::Array(vec![Value::Null; caps::RAW_VALUES]));
    assert_eq!(report.law().domain, Some(AdmissionRefusal::Census(CensusPart::TypedNested, CensusStatus::ValueLimit)));
    let report = admitted_typed(cap_maximal(), |r| {
        let mut v = Value::Null;
        for _ in 0..DEPTH_LIMIT + 1 {
            v = Value::Array(vec![v]);
        }
        r.model.project.units = v;
    });
    assert_eq!(report.law().domain, Some(AdmissionRefusal::Census(CensusPart::TypedNested, CensusStatus::DepthLimit)));
}

#[test]
fn unknown_stale_overflow_and_partial_refusals_keep_every_fact() {
    let report = admitted(milestone());
    let law = report.law();
    let with_text = |raw: BorrowedValueFacts, text: RawTextFacts, typed: BorrowedRequestFacts, nested: NestedTypedFacts, headless: Option<HeadlessRootFacts>| {
        let request: LinearStaticPreviewRequest = serde_json::from_value(milestone()).unwrap();
        domain_clauses(
            &DomainFacts { raw: &raw, raw_text: &text, typed: &typed, nested: &nested, headless: headless.as_ref(), digest: &report.captured_digest, combinations: &NO_COMBINATIONS },
            &request,
        )
    };
    let base = |raw, typed, nested, headless| with_text(raw, law.raw_text, typed, nested, headless);
    for status in [CensusStatus::DepthLimit, CensusStatus::ValueLimit, CensusStatus::ArithmeticOverflow] {
        let mut raw = report.raw;
        raw.status = status;
        assert_eq!(base(raw, report.typed, law.nested, None), Err(AdmissionRefusal::Census(CensusPart::Raw, status)));
        // The raw-text walk shares the raw census's cursor and limits, so in practice it is
        // incomplete only together with it; D1.2 still checks it on its own.
        let mut text = law.raw_text;
        text.status = status;
        assert_eq!(with_text(report.raw, text, report.typed, law.nested, None), Err(AdmissionRefusal::Census(CensusPart::RawText, status)));
        let mut typed = report.typed;
        typed.dof_upper = Err(status);
        assert_eq!(base(report.raw, typed, law.nested, None), Err(AdmissionRefusal::Census(CensusPart::TypedDof, status)));
        let mut nested = law.nested;
        nested.status = status;
        assert_eq!(base(report.raw, report.typed, nested, None), Err(AdmissionRefusal::Census(CensusPart::TypedNested, status)));
        let partial = HeadlessRootFacts { payload: report.raw, invocation: BorrowedValueFacts { status, ..report.raw }, request_id: report.captured_digest, payload_and_invocation_alias: false };
        assert_eq!(base(report.raw, report.typed, law.nested, Some(partial)), Err(AdmissionRefusal::Census(CensusPart::Headless, status)));
    }
    // The ordinary route is unchanged by every refusal, and the report keeps its facts.
    let mut deep = milestone();
    let mut child = Value::Null;
    for _ in 0..70 {
        child = Value::Array(vec![child]);
    }
    deep["unknown_admission_depth"] = child;
    for (raw, expected) in [
        (deep, Some(AdmissionRefusal::Census(CensusPart::Raw, CensusStatus::DepthLimit))),
        ({ let mut r = milestone(); r["model"]["supports"][1]["family"] = json!(" anchor"); r }, family(D1Clause::Supports, FamilyFact::SupportFamily)),
        ({ let mut r = milestone(); r["model"]["nodes"][0]["provenance"] = json!(text("p", 129)); r }, Some(AdmissionRefusal::Cap { fact: CapFact::TypedTextBytes, observed: 129, cap: 128 })),
    ] {
        for mode in MODES {
            let plain = serde_json::to_vec(&crate::run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap();
            let output = crate::run_linear_static_preview_value_with_retained_direct(raw.clone(), mode, RETAINED_LIMIT).unwrap();
            assert_eq!(serde_json::to_vec(output.envelope()).unwrap(), plain, "ordinary bytes");
            let report = output.admission().unwrap();
            assert_eq!(report.law().domain, expected);
            assert_eq!(report.law().refusal, d1_1_refusal().or(expected), "D1.1 first, then the domain refusal");
            assert!(output.successor().is_none());
        }
    }
}

#[test]
fn law_order_reports_the_first_failing_clause() {
    use std::cell::Cell;
    let called = Cell::new(false);
    let bound = |result: Result<u64, BoundRefusal>| {
        let called = &called;
        move |_index: usize| {
            called.set(true);
            result
        }
    };
    let family = Err(AdmissionRefusal::Family(D1Clause::Loads, FamilyFact::LoadTarget));
    assert_eq!(law_order(RetainedCaller::Headless, Ok(0), Ok(()), bound(Ok(1))), Err(AdmissionRefusal::Caller(RetainedCaller::Headless)));
    assert_eq!(law_order(RetainedCaller::Direct, Err(ProfileStatus::Missing), family, bound(Ok(1))), Err(AdmissionRefusal::Profile(ProfileStatus::Missing)));
    assert_eq!(law_order(RetainedCaller::Direct, Err(ProfileStatus::Stale), Ok(()), bound(Ok(1))), Err(AdmissionRefusal::Profile(ProfileStatus::Stale)));
    assert!(!called.get(), "the bound is not evaluated before D1 holds");
    assert_eq!(law_order(RetainedCaller::Direct, Ok(0), family, bound(Ok(1))), family.map(|_| 0));
    assert!(!called.get());
    assert_eq!(law_order(RetainedCaller::Direct, Ok(2), Ok(()), bound(Err(BoundRefusal::Unpriced))), Err(AdmissionRefusal::Bound(BoundRefusal::Unpriced)));
    assert!(called.get());
    assert_eq!(law_order(RetainedCaller::Direct, Ok(2), Ok(()), bound(Ok(7))), Ok(2));
    assert_eq!(AdmissionRefusal::Bound(BoundRefusal::Overflow).precondition().as_str(), "resource_admission");
    assert_eq!(AdmissionRefusal::Profile(ProfileStatus::Stale).clause(), Some(D1Clause::Build));
    // G6: every atom is priced, so the bound is the mode's in-build maximum; still no profile is
    // registered, so D1.1 refuses first in every build (no_profile_or_permit_is_constructible).
    assert_eq!(cap_priced_maximum(PreviewSolverMode::SparseInteractive).map(|b| b > 0), Ok(true));
}

#[test]
fn bound_admits_at_m_and_refuses_above() {
    let r = RESERVED_STACK_BYTES as u64;
    assert_eq!(r, 67_108_864);
    assert_eq!(bound_admits(M - r - 1, r, M), Ok(M - 1));
    assert_eq!(bound_admits(M - r, r, M), Ok(M), "at M");
    assert_eq!(bound_admits(M - r + 1, r, M), Err(BoundRefusal::Exceeds { required: M + 1, threshold: M }));
    assert_eq!(bound_admits(u64::MAX, 1, M), Err(BoundRefusal::Overflow));
    assert_eq!(bound_admits(u64::MAX - r, r, u64::MAX), Ok(u64::MAX));
}

/// RV89 G6 S-3: admission adds R (the constant 64 MiB, never the `cfg(test)` stack override)
/// to the mode's maximum before comparing with the threshold. Tested at the edges M − R − 1,
/// M − R and M − R + 1, on this build's own maxima, and in `admit`'s source: its bound is
/// priced through `admission_bound` alone, and `law.required` records what it computed.
#[test]
fn admission_bound_adds_r_before_comparing_with_m() {
    let r = RESERVED_STACK_BYTES as u64;
    assert_eq!(r, 64 << 20);
    assert_eq!(admission_bound(Ok(M - r - 1), M), Ok(M - 1));
    assert_eq!(admission_bound(Ok(M - r), M), Ok(M), "at M");
    assert_eq!(admission_bound(Ok(M - r + 1), M), Err(BoundRefusal::Exceeds { required: M + 1, threshold: M }), "one byte over M once R is added");
    assert_eq!(admission_bound(Ok(M), M), Err(BoundRefusal::Exceeds { required: M + r, threshold: M }), "the maximum alone at M is refused");
    assert_eq!(admission_bound(Err(BoundRefusal::Unpriced), M), Err(BoundRefusal::Unpriced));
    assert_eq!(admission_bound(Ok(u64::MAX - r + 1), u64::MAX), Err(BoundRefusal::Overflow));
    assert_eq!(bound_required(&admission_bound(Ok(M - r), M)), Some(M));
    assert_eq!(bound_required(&admission_bound(Ok(M - r + 1), M)), Some(M + 1));
    assert_eq!(bound_required(&Err(BoundRefusal::Unpriced)), None);
    assert_eq!(bound_required(&Err(BoundRefusal::Overflow)), None);
    for mode in MODES {
        let maximum = cap_priced_maximum(mode).unwrap();
        assert_eq!(admission_bound(Ok(maximum), M), Ok(maximum + r), "{mode:?}");
        assert!(maximum + r <= MARGIN, "{mode:?}: the 0.9 M margin holds at admission");
    }
    // The R override sizes only the witness thread; the bound ignores it.
    RESERVED_STACK_OVERRIDE.with(|c| c.set(Some(1 << 20)));
    assert_eq!(admission_bound(Ok(M - r), M), Ok(M));
    RESERVED_STACK_OVERRIDE.with(|c| c.set(None));
    let source = include_str!("retained_memory.rs");
    let admit_src = &source[source.find("pub(super) fn admit(").unwrap()..];
    let admit_src = &admit_src[..admit_src.find("\n}\n").unwrap()];
    assert_eq!(admit_src.matches("limited_bound(cap_priced_maximum(mode), limit, REGISTERED_PROFILES[index].threshold_bytes)").count(), 1);
    assert_eq!(admit_src.matches("required = bound_required(&bound);").count(), 1);
    assert_eq!(admit_src.matches("report.law.required = required;").count(), 1);
    assert!(!admit_src.contains("bound_admits(") && !admit_src.contains("admission_bound("), "admit prices its bound only through limited_bound");
    // In a build that refuses before the bound, nothing is recorded.
    if build_status().is_err() {
        for mode in MODES {
            let (request, capture) = CapturedInvocation::parse(milestone(), mode).unwrap();
            assert_eq!(assess(&capture, &request, Entry::Direct, RETAINED_LIMIT).law().required, None, "{mode:?}");
        }
    }
}

#[test]
fn refusal_kinds_are_the_schema_preconditions() {
    let schema = include_str!("../../../schemas/retained_precision_mp_v2.schema.json");
    let start = schema.find("\"const\": \"unavailable_precondition\"").unwrap();
    let block = &schema[start..start + 400];
    for kind in [UnavailablePrecondition::Caller, UnavailablePrecondition::SourceFamily, UnavailablePrecondition::ResourceAdmission] {
        assert!(block.contains(&format!("\"{}\"", kind.as_str())), "{kind:?}");
    }
    assert_eq!(STACK_RESERVATION_PRECONDITION.as_str(), "resource_admission");
    assert_eq!(AdmissionRefusal::Census(CensusPart::Raw, CensusStatus::ValueLimit).precondition().as_str(), "resource_admission");
    let ids: Vec<&str> = [D1Clause::Caller, D1Clause::Build, D1Clause::Census, D1Clause::Namespace, D1Clause::Invocation, D1Clause::Case,
        D1Clause::Supports, D1Clause::Loads, D1Clause::Members, D1Clause::Caps, D1Clause::Provenance, D1Clause::ControlBytes]
        .map(D1Clause::id)
        .to_vec();
    assert_eq!(ids, ["D1.0", "D1.1", "D1.2", "D1.3", "D1.4", "D1.5", "D1.6", "D1.7", "D1.8", "D1.9", "D1.10", "D1.11"]);
}

// ---- G-B and G-C -------------------------------------------------------------

#[test]
fn every_phase_fact_admits_its_cap_and_refuses_cap_plus_one() {
    let caps = phase_caps();
    let late: [PhaseFact; LATE_FACTS] = [PhaseFact::BuiltNodes, PhaseFact::BuiltMembers, PhaseFact::BuiltFrameElements, PhaseFact::BuiltSupports,
        PhaseFact::CaseLoads, PhaseFact::CaseLoadsTotal, PhaseFact::Restrained, PhaseFact::Springs, PhaseFact::Materials, PhaseFact::LateObservationBytes];
    let observed = |facts: &[PhaseFact], values: &[u64]| facts.iter().zip(values).map(|(f, v)| PhaseObservation { fact: *f, observed: *v }).collect::<Vec<_>>();
    let at: [PhaseObservation; LATE_FACTS] = observed(&late, &caps.late).try_into().unwrap();
    assert_eq!(check_phase(PhaseGate::Late, &at, &caps.late), Ok(()));
    for i in 0..LATE_FACTS {
        let mut over = at;
        over[i].observed += 1;
        assert_eq!(check_phase(PhaseGate::Late, &over, &caps.late),
            Err(PhaseRefusal { gate: PhaseGate::Late, fact: late[i], observed: caps.late[i] + 1, cap: caps.late[i] }));
    }
    assert_eq!(caps.late[..9], [32, 32, 32, 32, 128, 384, 192, 192, 8], "D1's counts: n, m, m, g, l, L (B1 SA), k = min(6n, r), s, 4 + 4");
    // W-7 (§8.20, W7-RL): route L's members also publish their twelve global end rows, 78 per member.
    assert_eq!(P_FINAL, 2_979);
    // T4-U6 (D-5, U6-RP), T4-U4b (U4B-RP), W-7 (W7-RP): route E publishes 114 rows per exact member; its G-C
    // rows and their capacity are C·P_final at its own P_final, the chain's pushcap(C·P_final) results slots
    // (U6-RB-V); the rows' text bound stays route L's (12,393 <= 17,874 at the caps).
    assert_eq!(P_FINAL_EXACT, 4_131);
    let exact = phase_caps_on(crate::retained_product::W1Route::Exact).complete;
    assert_eq!((exact[0], exact[1], exact[2]), (3 * 4_131, 16_384, caps.complete[2]), "route E: C·P_final, PushCap, route L's text");
    // B1 SA (option S3, C = 3): the bounds that read the regenerated profile are asserted as
    // expressions (SF-4's back edge: SQ fixes their values; RV-Q round 1 reviews these).
    let c = caps::LOAD_CASES as u64;
    assert_eq!(caps.complete[2], 2 * c * P_FINAL * text_atoms::ROW, "2·C·P_final·Text(row)");
    assert_eq!(caps.complete[5], 2 * profile::TEXT_TEXT_DIAG_ENV, "2·Text(diag_env) at l ≤ 128 (API_G4.md, S-6(d))");
    // Absolute node-proximity tolerance at f985fe297a (§8.18): +10,398 B, validate_model_settings'
    // blocking refusal (NP-MS, 10,392 B) and the near-duplicate message's new text (+3 B in each of the
    // two topology diagnostics' message windows), on the silent-zero guard's 269,518,418.
    // Silent-zero guard on W-8 (§8.15): +7,194 B, the per-case LOAD_CASE_APPLIES_NO_ACTION warning,
    // 3 × 2,398 B (SZ-ID), on W-8's 269,511,224.
    // W-8 topology warnings' source 34e96d6b17, C_eq = 3, maximum
    // over the L/E routes (retained-pricing-rules.md §8.16): +16,378 B from the pre-0.4 failure
    // unification's 269,494,846 (64144ab20b, §8.14), the three topology warnings in the envelope. That
    // was +429 B from the T4-U4b/U5b epoch's 269,494,417 (b4c9f35f8e; unchanged through O4's 7823a79f45
    // and R-A's 7d946b7e94), the three S3 finalization diagnostics whose error text may now be the S2
    // cause (PU-S2). T4-U6/U8's at 588de796a4: 262,735,773; B7b's at 7e63847: 264,366,429.
    // W-7 at 8afaa282b3 (§8.20): +1,028,736 B, the case solve's per-member global end diagnostic (W7-ID: its id,
    // message and refs, 10,716 B) for each of 32 members in each of 3 cases, on the silent-drop guard's 269,528,816.
    // W-15 at c7318c32f9 (§8.21): +84,896 B with D_env unchanged: the chain bounds a formatted diagnostic message by
    // the largest format within 40 lines (text_budget message_size), and the rigid-scale refusal's 786 B message (zero
    // on D1, W15-NZ) now lies in the window of the exact-section shear-factor diagnostic (407 B before): +379 B for
    // each of its 224 envelope instances, on W-7's 270,557,552.
    // W-16 slice a at 6d4e0c1506 (§8.23): +3,762,176 B, route L(3,0)'s envelope (now the maximum over routes) gains the
    // node/element imposed_displacement refusal for each of its 384 primitive loads (W16-ID) and loses the 128 deleted
    // pre-parse refusals (its D_env 25,470 -> 25,726), on W-15's 270,642,448.
    // Model document 0.8.0's shear-model member at 642e9737e4 (§8.26): +10,392 B, validate_model_settings'
    // second blocking refusal (SM-MS: 2,048 + 8,192 + 152 B per validation call, as NP-MS's), on 274,404,624.
    assert_eq!(profile::TEXT_TEXT_DIAG_ENV, 274_415_016, "current text closure at C_eq = 3, maximum over routes");
    // W-7 (§8.20): 3 · 2,979 = 8,937 rows exceed the former 8,192 slots; their push capacity is 16,384.
    assert_eq!((caps.complete[0], caps.complete[1]), (3 * 2_979, 16_384), "C·P_final, PushCap(C·P_final)");
    assert_eq!(caps.complete[4], push_capacity(text_atoms::D_ENV), "PushCap(D_env)");
    assert!(caps.late[LATE_FACTS - 1] > 0 && caps.late[LATE_FACTS - 1] < caps.complete[15], "T11 without its late capture < T11");
    assert!(caps.complete[16] > 0 && caps.complete[16] < caps.complete[15], "T11.4 < T11");
    assert_eq!((caps.complete[6], caps.complete[7]), (text_atoms::L_PUB, text_atoms::L_DIAGID), "L_PUB (RV84 C-N1) and L_DIAGID (RV87 N-3)");
    assert_eq!(caps.complete[17], c * (3 * 32 + 1) * text_atoms::ERR, "C·(3m + 1)·Text(err)");
    // Current generated TEXT atoms; their backing and visit counts stay positive.
    assert_eq!((text_atoms::L_PUB, text_atoms::L_DIAGID, text_atoms::ERR, text_atoms::D_ENV, push_capacity(text_atoms::D_ENV)),
        (6_139_118, 2_330, 16_384, 28_775, 32_768), "current L_PUB, L_DIAGID, Text(err), D_env and PushCap(D_env) at C_eq = 3");
    assert_eq!(caps.complete[17], 3 * (3 * 32 + 1) * 16_384, "C·(3m + 1)·Text(err) at C = 3");
    let complete_facts: Vec<PhaseFact> = complete_observations_of_milestone().iter().map(|o| o.fact).collect();
    let at: [PhaseObservation; COMPLETE_FACTS] = observed(&complete_facts, &caps.complete).try_into().unwrap();
    assert_eq!(check_phase(PhaseGate::Complete, &at, &caps.complete), Ok(()));
    for i in 0..COMPLETE_FACTS {
        let mut over = at;
        over[i].observed += 1;
        assert_eq!(check_phase(PhaseGate::Complete, &over, &caps.complete).unwrap_err().fact, complete_facts[i]);
    }
}
fn complete_observations_of_milestone() -> [PhaseObservation; COMPLETE_FACTS] {
    let raw = milestone();
    let mode = PreviewSolverMode::SparseInteractive;
    let (request, capture) = CapturedInvocation::parse(raw, mode).unwrap();
    let mut observer = crate::retained_product::ProductCapture::prepared_probe();
    let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
    complete_observations(&CompleteFacts { ordinary: &ordinary, capture: &observer, requested_cases: 1 })
}

#[test]
fn complete_facts_read_the_actual_owners() {
    let raw = milestone();
    for mode in MODES {
        let (request, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
        let mut observer = crate::retained_product::ProductCapture::prepared_probe();
        let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
        let o = complete_observations(&CompleteFacts { ordinary: &ordinary, capture: &observer, requested_cases: 1 });
        let get = |fact| o.iter().find(|x| x.fact == fact).unwrap().observed;
        assert_eq!(get(PhaseFact::EnvelopeResults), ordinary.results.len() as u64);
        assert_eq!(get(PhaseFact::EnvelopeDiagnostics), ordinary.diagnostics.len() as u64);
        // Independent sums of every String the rows and diagnostics own.
        let cap = |s: &String| s.capacity();
        let vec_text = |v: &Vec<String>| v.capacity() * std::mem::size_of::<String>() + v.iter().map(cap).sum::<usize>();
        let rows: usize = ordinary.results.iter().map(|r| {
            [&r.id, &r.kind, &r.unit, &r.entity_ref].map(cap).iter().sum::<usize>() + vec_text(&r.source_result_refs)
                + r.basis_ref.as_ref().map_or(0, |b| cap(&b.ref_type) + cap(&b.ref_id))
                + r.metadata.as_ref().map_or(0, |m| [&m.component, &m.coordinate_system, &m.location, &m.basis, &m.sign_convention].map(cap).iter().sum())
        }).sum();
        assert_eq!(get(PhaseFact::EnvelopeResultTextBytes), rows as u64);
        let diagnostics: usize = ordinary.diagnostics.iter().map(|d| {
            [&d.id, &d.code, &d.severity, &d.message].map(cap).iter().sum::<usize>() + d.source.as_ref().map_or(0, cap) + vec_text(&d.affected_refs)
        }).sum();
        assert_eq!(get(PhaseFact::EnvelopeDiagnosticTextBytes), diagnostics as u64);
        let longest = ordinary.results.iter().flat_map(|r| [&r.id, &r.kind, &r.unit, &r.entity_ref]).chain(ordinary.diagnostics.iter().flat_map(|d| [&d.id, &d.code, &d.severity, &d.message]))
            .map(String::len).max().unwrap();
        assert!(get(PhaseFact::EnvelopeMaxStringBytes) >= longest as u64);
        let message = ordinary.diagnostics.iter().map(|d| d.message.len()).max().unwrap();
        assert!(get(PhaseFact::EnvelopeMaxStringBytes) >= message as u64 && message > 0);
        assert_eq!(get(PhaseFact::DiagnosticIdMaxBytes), ordinary.diagnostics.iter().map(|d| d.id.len()).max().unwrap() as u64);
        assert_eq!(get(PhaseFact::EnvelopeResultCapacity), ordinary.results.capacity() as u64);
        assert_eq!(get(PhaseFact::EnvelopeDiagnosticCapacity), ordinary.diagnostics.capacity() as u64);
        let census = borrowed_value_census(ordinary.contract_evidence.as_ref().unwrap());
        assert_eq!((get(PhaseFact::ContractEvidenceArrayElements), get(PhaseFact::ContractEvidenceObjects), get(PhaseFact::ContractEvidenceEntries),
            get(PhaseFact::ContractEvidenceStringBytes), get(PhaseFact::ContractEvidenceKeyBytes)),
            (census.array_capacity_elements as u64, census.objects as u64, census.object_entries as u64, census.string_capacity_bytes as u64, census.key_capacity_bytes as u64));
        let seeds = observer.ordinary.capacity() * std::mem::size_of::<crate::retained_product::OrdinarySeed>()
            + observer.ordinary.iter().map(|s| s.case.capacity() + s.d5_diagnostic_ref.as_ref().map_or(0, cap)).sum::<usize>();
        assert_eq!(get(PhaseFact::OrdinarySeedBytes), seeds as u64);
        // A source-block recovery present at G-C is observed (permitted_run checks it first).
        let mut selected = ordinary.clone();
        selected.source_block_recovery = Some(json!({}));
        let s = complete_observations(&CompleteFacts { ordinary: &selected, capture: &observer, requested_cases: 1 });
        assert_eq!(s.iter().find(|x| x.fact == PhaseFact::SourceBlockRecovery).unwrap().observed, 1);
        // An incomplete preview-tree census is observed as such.
        let mut deep = ordinary.clone();
        let mut v = Value::Null;
        for _ in 0..DEPTH_LIMIT + 1 {
            v = Value::Array(vec![v]);
        }
        deep.contract_evidence = Some(v);
        let d = complete_observations(&CompleteFacts { ordinary: &deep, capture: &observer, requested_cases: 1 });
        assert_eq!(d.iter().find(|x| x.fact == PhaseFact::ContractEvidenceStatus).unwrap().observed, 1);
        // An association error's text is retained error text (C-N4).
        let mut failed = crate::retained_product::ProductCapture::prepared_probe();
        failed.error = Some(crate::retained_product::CaptureError::Association(String::with_capacity(40)));
        failed.observable_error = Some(crate::retained_product::CaptureError::Association(String::with_capacity(9)));
        let e = complete_observations(&CompleteFacts { ordinary: &ordinary, capture: &failed, requested_cases: 1 });
        assert_eq!(e.iter().find(|x| x.fact == PhaseFact::RetainedErrorTextBytes).unwrap().observed, 49);
        assert_eq!(get(PhaseFact::SourceBlockRecovery), 0);
        assert_eq!(get(PhaseFact::ContractEvidenceStatus), 0);
        assert!(get(PhaseFact::ContractEvidenceEntries) > 0, "the preview tree");
        assert!(get(PhaseFact::ObservationBytes) > 0, "the capture's own tally");
        assert!(get(PhaseFact::OrdinarySeedBytes) >= std::mem::size_of::<crate::retained_product::OrdinarySeed>() as u64);
        assert_eq!(get(PhaseFact::RetainedErrorTextBytes), 0);
        // The count and text facts sit well inside their bounds for the milestone.
        let caps = phase_caps().complete;
        for (x, cap) in o.iter().zip(caps) {
            assert!(x.observed <= cap, "{:?}: {} > {}", x.fact, x.observed, cap);
        }
    }
}

// ---- S1 and the budgets --------------------------------------------------------

#[test]
fn reserved_stack_is_r_with_a_test_override() {
    assert_eq!((RESERVED_STACK_BYTES, STACK_WITNESS_DIVISOR), (64 << 20, 16));
    assert_eq!(reserved_stack(), RESERVED_STACK_BYTES);
    RESERVED_STACK_OVERRIDE.with(|c| c.set(Some(RESERVED_STACK_BYTES / STACK_WITNESS_DIVISOR)));
    assert_eq!(reserved_stack(), 4 << 20);
    RESERVED_STACK_OVERRIDE.with(|c| c.set(None));
    assert_eq!(reserved_stack(), RESERVED_STACK_BYTES);
}

#[test]
fn structural_budgets_are_u3s() {
    let b = PHASE_BUDGETS;
    assert_eq!((b.ordinary_runs, b.parses_per_invocation, b.fallible_allocations_after_first_mutation, b.fallback_new_allocations), (1, 1, 0, 0));
    assert_eq!(b.reserved_stack_bytes, RESERVED_STACK_BYTES as u64);
    assert_eq!(b.thread_heap_bytes, 8192 + std::mem::size_of::<crate::RetainedPreviewOutput>() as u64);
    let diagnostic = std::mem::size_of::<crate::Diagnostic>() as u64;
    // B1 SA (B-6; I82 STUDY §4.3): one notice reserve per case in A, |A| ≤ C.
    assert_eq!(b.notice_reserve_bytes, NOTICE_RESERVE_BYTES * caps::LOAD_CASES as u64, "B-6 = NOTICE_RESERVE_BYTES × C");
    assert!(NOTICE_RESERVE_BYTES >= diagnostic + 196, "the slot and the 196-byte message reservation");
    assert!(NOTICE_RESERVE_BYTES <= 2_048, "G4's push-growth figure tightened to grant 1b's exact reservation");
    // Part 2: every byte budget is a priced in-build form.
    for (name, bytes) in [("staged", b.staged_copy_bytes), ("successor", b.successor_bytes), ("invocation", b.precommit_invocation_bytes),
        ("reader", b.precommit_reader_bytes), ("statics", b.reader_statics_bytes)] {
        assert!(bytes > 1_000_000, "{name}: {bytes}");
        println!("I65_G5_BUDGET {name} {bytes}");
    }
    assert!(b.precommit_reader_bytes > b.successor_bytes && b.successor_bytes > b.staged_copy_bytes);
}

#[test]
fn late_facts_read_the_actual_owners() {
    let raw = milestone();
    let mode = PreviewSolverMode::SparseInteractive;
    let (request, capture) = CapturedInvocation::parse(raw, mode).unwrap();
    let mut diagnostics = Vec::new();
    // The dispatch builds with the model's materials when the request supplies none.
    let built = crate::build_model(&request.model, &request.model.materials, &mut diagnostics).expect("the milestone builds");
    let mut observer = crate::retained_product::ProductCapture::prepared_probe();
    let model = request.model.clone();
    let _ = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
    let restrained = [0usize, 1, 2, 3, 4, 5, 6];
    let springs: [crate::SpringEntry; 0] = [];
    let materials = &model.materials;
    let o = late_observations(&LateFacts {
        model: &model,
        built: &built,
        materials,
        case: &model.load_cases[0],
        restrained: &restrained,
        springs: &springs,
        capture: &observer,
    });
    let get = |fact| o.iter().find(|x| x.fact == fact).unwrap().observed;
    assert_eq!(get(PhaseFact::BuiltNodes), built.nodes.len() as u64);
    assert_eq!(get(PhaseFact::BuiltMembers), built.pipes.len() as u64);
    assert_eq!(get(PhaseFact::BuiltFrameElements), built.frame_elements.len() as u64);
    assert_eq!(get(PhaseFact::BuiltSupports), built.supports.len() as u64);
    assert_eq!(get(PhaseFact::CaseLoads), 3);
    // B1 SA: the running total is the capture's own (`late_loads_total`); this observer has no
    // permit, so its late hook never reached G-B and the total stayed 0.
    assert_eq!((get(PhaseFact::CaseLoadsTotal), observer.late_loads_total), (0, 0));
    assert_eq!(get(PhaseFact::Restrained), 7);
    assert_eq!(get(PhaseFact::Springs), 0);
    assert_eq!(get(PhaseFact::Materials), materials.len() as u64);
    assert!(get(PhaseFact::LateObservationBytes) > 0);
    assert_eq!(get(PhaseFact::LateObservationBytes),
        observer.adapter.counts.get()[crate::retained_product::AdapterEvent::RustCapacityBytes as usize]);
    // The milestone's distinct counts tell the readers apart.
    assert_eq!((built.nodes.len(), built.pipes.len(), built.frame_elements.len(), built.supports.len()), (2, 1, 1, 4), "pipes and frame elements coincide in D1 (straight members)");
    assert_eq!(check_phase(PhaseGate::Late, &o, &{ let mut c = phase_caps().late; c[LATE_FACTS - 1] = u64::MAX; c }), Ok(()));
}

// ---- G5 part 2: the generated profile ---------------------------------------------

#[test]
fn profile_transcribes_the_python_chain_exactly() {
    // Under the chain's own illustrative values, the generated expressions reproduce its
    // maximum and phase in both modes: the transcription is exact.
    for (i, phases) in [profile::phases_sparse(&profile::ATOM_ASSUMED), profile::phases_dense(&profile::ATOM_ASSUMED)].iter().enumerate() {
        let (bytes, phase) = profile::maximum(phases).unwrap();
        assert_eq!((bytes, profile::PHASE_NAMES[phase]), profile::PYTHON_CHECK[i], "mode {i}");
    }
}

#[test]
fn profile_in_build_record() {
    use profile::*;
    let r = RESERVED_STACK_BYTES as u64;
    let mut estimate_weight = [0u64; 2];
    for (m, (name, phases)) in [("sparse", phases_sparse(&ATOM_VALUES)), ("dense", phases_dense(&ATOM_VALUES))].iter().enumerate() {
        let (bytes, phase) = maximum(phases).unwrap();
        println!("I65_G5_PROFILE mode={name} max_without_R={bytes} E_mov_plus_R={} fraction_of_M={:.4} phase={}", bytes + r,
            (bytes + r) as f64 / M as f64, PHASE_NAMES[phase]);
        for (i, (req, mov)) in phases.iter().enumerate() {
            println!("I65_G5_PHASE mode={name} phase={} requested={} moving={} E_mov_plus_R={}", PHASE_NAMES[i].split(' ').next().unwrap(),
                req.unwrap(), mov.unwrap(), req.unwrap() + mov.unwrap() + r);
        }
        // The Estimate atoms' weight in the maximum: the maximum with every Estimate at zero.
        let mut zeroed = ATOM_VALUES;
        for i in 0..ATOMS {
            if ATOM_BINDINGS[i] == Binding::Estimate {
                zeroed[i] = 0;
            }
        }
        let without = maximum(&if m == 0 { phases_sparse(&zeroed) } else { phases_dense(&zeroed) }).unwrap().0;
        estimate_weight[m] = bytes - without;
        println!("I65_G5_ESTIMATE_WEIGHT mode={name} bytes={}", estimate_weight[m]);
    }
    for i in 0..ATOMS {
        println!("I65_G5_ATOM {:?}\t{}\t{}\t{}", ATOM_BINDINGS[i], ATOM_NAMES[i], ATOM_VALUES[i], ATOM_ASSUMED[i]);
    }
    assert!(SPARSE.is_some() && DENSE.is_some());
    // Pre-0.4 failure unification D3 (retained-pricing-rules.md §8.14, PU-X3): X3, the republication's second ordinary
    // span after a selected source could not finalize, is its own phase.
    assert_eq!(PHASE_NAMES.map(|n| n.split(' ').next().unwrap()), ["W1", "W2", "W3", "W4", "W5", "X1", "X2", "X3"]);
    // The pinned record G6 qualifies (R/I65/u4_g6_01/QUALIFICATION.md §3; B1: R/I104/b1_sq_01/QUAL_B1.md): each phase's requested
    // + moving bytes, without R, in the build whose identity is PINNED_RECORD_IDENTITY. It is
    // asserted only in that build (ROOT's ruling on part 2's decision 1); another identity has
    // its own layouts and prints a skip. Any change to a binding, form, combination or phase
    // changes it: regenerate the record with the profile, never one without the other.
    if super::COMPILED_IDENTITY != Some(PINNED_RECORD_IDENTITY) {
        println!("I65_G6_RECORD_SKIP the compiled identity is not the pinned record's: {:?}", super::COMPILED_IDENTITY);
        return;
    }
    for (m, phases) in [phases_sparse(&ATOM_VALUES), phases_dense(&ATOM_VALUES)].iter().enumerate() {
        assert_eq!(phases.map(|(req, mov)| req.unwrap() + mov.unwrap()), PINNED_RECORD[m], "mode {m}");
    }
    // T4-U4b/U5b with T4-S6's streamed hashes: W3 (the T16 publication with the staged copy) is
    // the maximum in both modes; from T4-U6/U8 (D-5) to W1 it was W4, at B7b W3.
    assert_eq!((SPARSE, DENSE), (Some((PINNED_RECORD[0][2], 2)), Some((PINNED_RECORD[1][2], 2))), "W3 is the maximum in both modes");
}
/// The build identity of the pinned profile record (the qualified dev/test build of G6).
pub(super) const PINNED_RECORD_IDENTITY: &str = "v1;rustc.release=1.97.1;rustc.commit=8bab26f4f68e0e26f0bb7960be334d5b520ea452;rustc.host=aarch64-apple-darwin;rustc.llvm=22.1.6;target=aarch64-apple-darwin;target.arch=aarch64;target.pointer_width=64;target.endian=little;target.os=macos;target.env=;panic=unwind;profile=debug;opt_level=0;debug_assertions=true;rustflags=;pkg=open_pipe_stress_product_physics@0.2.0";
/// W-16 slices b and c at 3493838a8b (retained-pricing-rules.md §8.27): from the shear-model member's
/// record below, the certified producer's two generated-id prefix formats, now behind the case's
/// generation (W16C-ES), leave TAV_W and TAV_X (-223,488 B) and the shorter identity text and longer
/// load/reference-state limitation add +690 B (-222,798 B each); the pressure-2 table and the
/// source-recovery schema raise STATICS by 6,332 B + 18 Node(String,Value) + 294 s(Value) (+28,988 B
/// here), so every phase falls by 193,810 B in both modes; W3 is still the maximum.
/// Model document 0.8.0's shear-model member at 642e9737e4 (retained-pricing-rules.md §8.26, "The
/// shear-model member"): from the unit factor-sum gate's record below (unchanged by §8.25 and §8.26's
/// first registration), validate_model_settings' second blocking diagnostic and its three texts (SM-MS,
/// SM-PT) raise TAV_W by 42,724 B and TAV_X by 21,362 B, Text(diag_env) by 10,392 B and D_env by 1, so
/// W1 and W2 rise by 54,124 B, W3 by 98,070 B, W4 by 92,536 B, W5 by 73,330 B, X1 by
/// 44,882 B, X2 by 39,848 B and X3 by 32,762 B in both modes; W3 is still the maximum.
/// Unit factor-sum gate at 0f0f28106e (retained-pricing-rules.md §8.24): from W-16 slice a's record
/// below, the restated preview-table condition in its three resident copies (UF-ST, +333 B of
/// STATICS) and `FactorSumError`'s Display texts (UF-TX, +180 B of TAV_W and TAV_X) raise every
/// phase by +513 B in both modes; D, D_env and the native atoms are unchanged, and W3 is still the
/// maximum.
/// W-16 slice a imposed displacement on the pressure-free route at 6d4e0c1506
/// (retained-pricing-rules.md §8.23): from W-15's record below (unchanged by W-15 slice b), D1.7
/// refuses every support target, so the tree adds case_prescribed's node index (W16-NI), validate's
/// findings vector (W16-VF) and, on route L, the node/element `imposed_displacement` refusal's texts
/// (W16-ID: L(3,0)'s Text(diag_env) +3,762,176 B), and loses the deleted pre-parse refusal (route E's
/// D and D_env -128); LoadTargetInput's support arm raises s(PrimitiveLoadInput) by 16 B
/// (+6,144 B per primitive-load copy). The phases move by +2,507,429 (W1, W2), +2,883,013 (W3),
/// -608,059 (W4), +957,381 (W5), +1,863,113 (X1), +1,600,697 (X2) and +2,507,705 B (X3) in both
/// modes; of W3's, +2,876,869 B is the tree and +6,144 B the native layouts (main's tree evaluated
/// at this build's atoms). W3 is still the maximum.
/// W-15 rigid-component scaling and reference-section identification at c7318c32f9
/// (retained-pricing-rules.md §8.21): from W-7's record below, D1 holds no component body (P2) and
/// D_env is unchanged; the readers' component-body refusal text (W15-ID), the longer pressure-2
/// identity literal (U6-J4), the predicate's member maps (W15-PM) and the readers' working set
/// (W15-RB) raise the tree, and ResultItem's `component_body` member raises its native layouts
/// (s(ResultItem) 296 -> 344 B), so the phases rise by +16,786,724 (W1, W2), +17,551,380 (W3),
/// +16,988,356 (W4), +16,871,620 (W5), +4,518,384 (X1) and +3,838,624 B (X2, X3) in both modes;
/// W3 is still the maximum.
/// W-7 member end actions in global coordinates at 8afaa282b3 (retained-pricing-rules.md §8.20): from
/// the silent-drop guard's record below, the rows rise to 78 per member on route L and 114 on route E
/// (W7-RL, W7-RP) with their texts and copies, the case solve's per-member global end diagnostic
/// (D_env +96, Text(diag_env) +1,028,736 B), the end-action operand table (W7-NC), the source
/// qualification's filtered references (W7-QF) and the readers' coverage maps (W7-RD, four new node
/// atoms), so the phases rise by +327,563,426 (W1), +334,104,866 (W2), +391,657,186 (W3),
/// +389,387,832 (W4), +360,296,930 (W5), +319,511,866 (X1), +215,863,434 (X2) and +210,018,826 B (X3)
/// in both modes; W3 is still the maximum.
/// Silent-drop guard at bc99d4d09f (retained-pricing-rules.md §8.19): from the node-proximity
/// tolerance's record below, TAV_W and TAV_X rise by 1,245,594 B (the closed-record scan's names and
/// paths at E's and L(3,0)'s caps, SD-ID) and O_base by 49 B and s(&str) by 4 (validation's
/// duplicate-term set, SD-HS, and the scan's probe error, SD-PB), so every phase rises by 1,245,707 B
/// in both modes. D, D_env and Text(diag_env) are unchanged.
/// Absolute node-proximity tolerance at f985fe297a (retained-pricing-rules.md §8.18): from the
/// silent-zero guard's record below, TAV_W rises by 63,748 B and TAV_X by 35,648 B (the tolerance
/// refusal's diagnostic and the accessor's five texts, NP-MS/NP-PT, the units lookup's extra calls and
/// the near-duplicate message's new text), Text(diag_env) by 10,398 B and D_env by 1, so W1 and W2
/// rise by 75,154 B, W3 by 119,112 B, W4 by 114,170 B, W5 by 94,366 B, X1 by 59,180 B, X2 by 54,140 B
/// and X3 by 47,054 B in both modes.
/// Silent-zero guard on W-8 (retained-pricing-rules.md §8.15, the merge of 1cb6b275f6 with
/// f0b6ae40a0): from W-8's record below, the per-case LOAD_CASE_APPLIES_NO_ACTION warning (SZ-ID)
/// raises TAV_W by 32,028 B and TAV_X by 16,014 B, Text(diag_env) by 7,194 B and D_env by 3, so W1
/// and W2 rise by 42,246 B, W3 by 126,856 B, W4 by 109,518 B, W5 by 75,882 B, X1 by 39,642 B, X2 by
/// 47,490 B and X3 by 26,232 B in both modes.
/// W-8 topology warnings actual-source record at 34e96d6b17 (retained-pricing-rules.md §8.16): from
/// the pre-0.4 unification's record below, TAV_W and TAV_X rise by 59,540 B (the warnings' texts),
/// O_base by W8-TW and W8-ND's owners (37,216 B) and the envelope's text owners by the three warnings
/// (Text(diag_env) +16,378 B, D_env +3), so W1, W2 and X3 rise by 116,158 B, W3 by 219,136 B, W4 by
/// 203,862 B, W5 by 159,714 B, X1 by 137,720 B and X2 by 137,416 B in both modes.
/// Pre-0.4 failure unification actual-source record at 64144ab20b (retained-pricing-rules.md
/// §8.14): requested + moving bytes, without R or the separately upward-rounded TEXT reserve, from
/// the canonical 100-form union, now over eight phases. From R-A's record (7d946b7e94, §8.13) TAV_W
/// and TAV_X rise by 6,277,658 B (the cause texts reachable on route E: PU-ID, PU-S2, PU-C, PU-CL)
/// and FAILED_MANIFEST_COPY, STAGED, SUCC, T16, T17 and T25 by 429 B each (Text(diag_env)'s three S3
/// finalization diagnostics, PU-S2), so W1, W2 and X2 rise by 6,278,087 B, W5 and X1 by
/// 6,278,516 B and W3 and W4 by 6,278,945 B in both modes. X3 is new (PU-X3): the republication's
/// second ordinary span. The build identity, atoms and native values are unchanged; reviewed-input
/// registration is separate below.
pub(super) const PINNED_RECORD: [[u64; 8]; 2] = [
    [6_559_512_350, 6_901_506_911, 8_855_689_571, 8_643_849_998, 7_721_878_225, 8_607_115_058, 7_338_515_720, 7_011_805_596],
    [6_618_643_694, 6_960_638_255, 8_914_820_915, 8_702_981_342, 7_781_009_569, 8_666_246_402, 7_397_647_064, 7_070_936_940],
];

#[test]
fn challenge_bounds_are_the_profile() {
    if COMPILED_IDENTITY != Some(PINNED_RECORD_IDENTITY) {
        println!("I65_G6_RECORD_SKIP the challenge bounds are pinned to {PINNED_RECORD_IDENTITY}");
        return;
    }
    // The isolated challenge binary (tests/retained_memory_challenge.rs) holds the profile's
    // W1 phase as literals; they must equal the in-build evaluation.
    let text = include_str!("../tests/retained_memory_challenge.rs");
    let line = text.lines().find(|l| l.starts_with("const W1_PHASE_BYTES: [u64; 2] = [")).unwrap();
    let digits: Vec<u64> = line.split(['[', ']', ',']).filter_map(|t| t.trim().replace('_', "").parse().ok()).collect();
    let w1 = profile::PHASE_NAMES.iter().position(|n| n.starts_with("W1 ")).unwrap();
    let sparse = profile::phases_sparse(&profile::ATOM_VALUES)[w1];
    let dense = profile::phases_dense(&profile::ATOM_VALUES)[w1];
    assert_eq!(digits, [sparse.0.unwrap() + sparse.1.unwrap(), dense.0.unwrap() + dense.1.unwrap()]);
    let line = text.lines().find(|l| l.starts_with("const MAX_PHASE_BYTES: [u64; 2] = [")).unwrap();
    let digits: Vec<u64> = line.split(['[', ']', ',']).filter_map(|t| t.trim().replace('_', "").parse().ok()).collect();
    assert_eq!(digits, [profile::SPARSE.unwrap().0, profile::DENSE.unwrap().0]);
    // B1 SQ (PLAN_v2 §3.5): the abort cap is 16 GiB, above E_mov,max + R and above M; and the
    // challenge reads W1 work from the public surface by the N1 notice's own text (A1-S-1).
    assert_eq!(text.lines().filter(|l| l.starts_with("const CAP_BYTES: usize = ")).collect::<Vec<_>>(), ["const CAP_BYTES: usize = 16 << 30;"]);
    let r = RESERVED_STACK_BYTES as u64;
    assert!(profile::DENSE.unwrap().0 + r < M && M < 16 << 30, "the cap is above M, and M above E_mov,max + R");
    let line = text.lines().find(|l| l.starts_with("const N1_NOTICE: &str = ")).unwrap();
    let notice = line.split('"').nth(1).unwrap();
    assert!(notice.len() > 40 && crate::RETAINED_UNAVAILABLE_NOTICE.starts_with(notice), "the challenge's N1 text is the notice's");
}

/// RV89 G5 part 2 S-3: `profile::maximum` takes every phase, X1 and X2 included. Each phase
/// in turn is the largest; ties keep the earliest; an overflowed phase is no maximum at all.
#[test]
fn maximum_takes_every_phase() {
    use profile::{maximum, PHASES};
    for winner in 0..PHASES {
        let mut phases = [(Some(100u64), Some(5u64)); PHASES];
        phases[winner] = (Some(100), Some(6));
        assert_eq!(maximum(&phases), Some((106, winner)), "phase {winner} largest");
        // The moving part alone can decide it.
        let mut phases = [(Some(100u64), Some(0u64)); PHASES];
        phases[winner] = (Some(99), Some(2));
        assert_eq!(maximum(&phases), Some((101, winner)), "phase {winner} by its moving part");
    }
    assert_eq!(maximum(&[(Some(7u64), Some(0u64)); PHASES]), Some((7, 0)), "a tie keeps the earliest phase");
    for broken in 0..PHASES {
        let mut phases = [(Some(1u64), Some(1u64)); PHASES];
        phases[broken] = (None, Some(1));
        assert_eq!(maximum(&phases), None, "phase {broken} overflowed in its requested part");
        phases[broken] = (Some(1), None);
        assert_eq!(maximum(&phases), None, "phase {broken} overflowed in its moving part");
        phases[broken] = (Some(u64::MAX), Some(1));
        assert_eq!(maximum(&phases), None, "phase {broken}'s sum overflowed");
    }
}

#[test]
fn profile_batch5_carriers_bind_actual_layouts_and_all_case_owners() {
    use profile::{ATOM_BINDINGS, ATOM_NAMES, ATOM_VALUES, Binding, FORMS};
    use std::mem::size_of;
    use open_pipe_stress_frame_kernel::structural::LoadFormula;
    let actual = [
        ("s(ExactPressureCaseOption)", size_of::<Option<crate::pressure_runtime::ExactPressureCase>>()),
        ("s(PressureRegionInput)", size_of::<crate::PressureRegionInput>()),
        ("s(LoadFormula)", size_of::<LoadFormula>()),
        ("s(PreparedCaseResult)", size_of::<Result<crate::LoadCaseSolve, open_pipe_stress_frame_kernel::FrameKernelError>>()),
        ("s(BTreeMapHeader)", size_of::<std::collections::BTreeMap<usize, f64>>()),
    ];
    for (name, size) in actual {
        let index = ATOM_NAMES.iter().position(|entry| *entry == name).expect("positive owned carrier");
        assert_eq!(ATOM_BINDINGS[index], Binding::InBuild, "{name} is the actual build's type");
        assert_eq!(ATOM_VALUES[index], size as u64, "{name} retains its complete owner layout");
        assert!(size > 0);
        for form in [profile::F_O_BASE_SPARSE, profile::F_O_BASE_DENSE] {
            let coefficient = FORMS[form].terms.iter().find(|(atom, _)| *atom == index).expect("carrier cannot disappear from an ordinary case").1;
            assert!(coefficient >= caps::LOAD_CASES as u64, "{name} retains every overlapping case owner");
        }
    }
    let index = ATOM_NAMES.iter().position(|entry| *entry == "s(PreparedCaseWorkspace)").unwrap();
    assert_eq!(ATOM_BINDINGS[index], Binding::SourceUpper);
    assert!(ATOM_VALUES[index] > 0);
    for (name, layout) in READER_LAYOUT_NAMES.iter().zip(READER_LAYOUTS) {
        println!("I65_G5_READER_LAYOUT {name} size={} align={}", layout.size, layout.align);
    }
}

#[test]
fn profile_laws_hold_in_this_build() {
    use profile::btree_node_upper;
    use std::mem::{align_of, size_of};
    // BUILD.md §4's check: (String, Value) is Leaf_up 640 and Internal_up 736 on a 64-bit build.
    assert_eq!(btree_node_upper(size_of::<String>(), align_of::<String>(), size_of::<Value>(), align_of::<Value>()), 736);
    assert_eq!(btree_node_upper(8, 8, 0, 1), 208, "a usize-keyed set node: leaf 112, internal 208");
    assert_eq!(btree_node_upper(4, 4, 0, 1), 168, "A is at least 8: leaf 72, internal 168");
    // The combinators are checked: an overflowed sum is None (the bound then refuses), and max is the larger.
    assert_eq!((profile::add(Some(u64::MAX), Some(1)), profile::add(Some(2), Some(3)), profile::add(None, Some(3))), (None, Some(5), None));
    assert_eq!((profile::max(Some(2), Some(9)), profile::max(Some(9), Some(2)), profile::max(Some(1), None)), (Some(9), Some(9), None));
    // The bound is the mode's own maximum once no Estimate remains; Unpriced before.
    assert_eq!(priced_maximum(0, PreviewSolverMode::SparseInteractive), Ok(profile::SPARSE.unwrap().0));
    assert_eq!(priced_maximum(0, PreviewSolverMode::DenseScrutiny), Ok(profile::DENSE.unwrap().0));
    assert_eq!(priced_maximum(1, PreviewSolverMode::DenseScrutiny), Err(BoundRefusal::Unpriced));
    assert_ne!(profile::SPARSE.unwrap().0, profile::DENSE.unwrap().0, "the modes differ, so a swap is visible");
    // The kernel exports agree with the kernel types PP can also name.
    {
        use open_pipe_stress_frame_kernel::load_ledger::Formation;
        use open_pipe_stress_frame_kernel::structural::retained_api as k;
        use open_pipe_stress_frame_kernel::structural::retained_resource as fkr;
        assert_eq!((fkr::FORMATION, fkr::FORMATION_ALIGN), (size_of::<Formation>(), align_of::<Formation>()));
        assert_eq!((fkr::PRODUCT_FINAL_ROW, fkr::PRODUCT_RECIPE, fkr::RETAINED_SOLVE),
            (size_of::<k::ProductFinalRow<'static>>(), size_of::<k::ProductRecipe>(), size_of::<k::RetainedSolve>()));
        assert!(fkr::MAX_ALIGN >= align_of::<Formation>() && fkr::MAX_ALIGN <= 16, "BUILD.md §4's premise align(K) <= 16");
    }
    // RawVec's doubling from 4.
    assert_eq!([0, 1, 4, 5, 4096, 4097].map(push_capacity), [0, 4, 4, 8, 4096, 8192]);
    // An overflowed profile value is a 0 cap (every positive fact refuses), never u64::MAX.
    assert_eq!((checked_or_zero(None), checked_or_zero(Some(5))), (0, 5));
    // The gate's D_env and text bounds are the profile's text closure.
    let caps = phase_caps();
    assert_eq!(caps.complete[3], 28_775, "current canonical D_env at C_eq = 3 (model document 0.8.0's shear-model refusal, §8.26: +1; W-16 slice a, §8.23: -128, route E's deleted pre-parse imposed-displacement refusal; W-7, §8.20: +96, the global end diagnostic per member and case; the node-proximity tolerance's refusal, §8.18: +1; silent-zero guard on W-8, §8.15: +3, one warning per case; W-8's 34e96d6b17: the three topology warnings on 28,799 from T4-U4b/U5b's b4c9f35f8e)");
    assert_eq!(text_atoms::ROW, 11_474);
    // Every atom is bound; the source-derived and estimate atoms are the recorded ones.
    let count = |b| profile::ATOM_BINDINGS.iter().filter(|x| **x == b).count();
    // O4 (retained-pricing-rules.md §8.9) adds three SourceUppers: PreparedSectionPatch, CompositeMaximum, O4KernelStaging.
    assert_eq!((count(profile::Binding::SourceUpper), count(profile::Binding::Text), count(profile::Binding::Estimate)), (26, 6, 0));
    assert_eq!(profile::ESTIMATES, 0, "G6 closed every Estimate");
    assert_eq!(cap_priced_maximum(PreviewSolverMode::SparseInteractive), Ok(profile::SPARSE.unwrap().0), "priced in-build");
    assert_eq!(cap_priced_maximum(PreviewSolverMode::DenseScrutiny), Ok(profile::DENSE.unwrap().0), "priced in-build");
    // The in-build maximum is within the 0.9 M margin rule in both modes (RR "U4 G3 verified").
    let r = RESERVED_STACK_BYTES as u64;
    assert_eq!((M, MARGIN), (12_884_901_888, 11_596_411_699), "M = 12 GiB and ⌊0.9 M⌋");
    for (bytes, _) in [profile::SPARSE.unwrap(), profile::DENSE.unwrap()] {
        assert!(bytes + r <= MARGIN, "{} above 0.9 M", bytes + r);
    }
    // Qualification keeps the existing 5% reserve in every phase, including X1/X2.
    // Round the required reserve upward to whole bytes; never round down a requirement.
    for phases in [profile::phases_sparse(&profile::ATOM_VALUES), profile::phases_dense(&profile::ATOM_VALUES)] {
        for (phase, (requested, moving)) in phases.iter().enumerate() {
            let text_form = if phase < 5 { profile::F_TAV_W } else { profile::F_TAV_X };
            let text = profile::form(&profile::FORMS[text_form], &profile::ATOM_VALUES).unwrap();
            let reserve = text.checked_add(19).unwrap() / 20;
            let required = requested.unwrap().checked_add(moving.unwrap()).unwrap().checked_add(r).unwrap();
            assert!(required.checked_add(reserve).unwrap() <= MARGIN, "{} exceeds the 5% TEXT reserve", profile::PHASE_NAMES[phase]);
        }
    }
    assert!(profile::DENSE.unwrap().0 >= profile::SPARSE.unwrap().0, "the dense parity tail");
}

// ---- RV89 on part 1 (S-1, N-2, N-4) ----------------------------------------------

/// S-1 (V03): D1.3 refuses one authored expansion law among several materials,
/// wherever it sits, in the model list and in the request list.
#[test]
fn d1_3_refuses_one_authored_law_among_several_materials() {
    let two = || {
        let mut raw = milestone();
        let mut second = raw["model"]["materials"][0].clone();
        second["id"] = json!("mat:second");
        raw["model"]["materials"].as_array_mut().unwrap().push(second);
        raw
    };
    assert_eq!(domain(two()), None, "two model materials, no law: inside D1");
    for authored in 0..2 {
        let mut raw = two();
        raw["model"]["materials"][authored]["expansion_laws"] = Value::Null;
        assert_eq!(domain(raw), family(D1Clause::Namespace, FamilyFact::MaterialExpansionLaw), "model material {authored}");
    }
    let request_material = |id: &str| json!({"id": id, "elastic_modulus": {"value": 1.0, "unit": "Pa"}});
    let mut raw = milestone();
    raw["materials"] = json!([request_material("r0"), request_material("r1")]);
    assert_eq!(domain(raw.clone()), None, "two request materials, no law: inside D1");
    for authored in 0..2 {
        let mut with = raw.clone();
        with["materials"][authored]["expansion_laws"] = Value::Null;
        assert_eq!(domain(with), family(D1Clause::Namespace, FamilyFact::RequestExpansionLaws), "request material {authored}");
    }
}

/// S-1 (V06): the Σ restraint-capacity row is the sum over supports, so a total
/// above 192 refuses even when every single capacity is within 192.
#[test]
fn restraint_capacity_total_is_the_sum_over_supports() {
    let (mut request, capture) = CapturedInvocation::parse(cap_maximal(), PreviewSolverMode::SparseInteractive).unwrap();
    request.model.supports[0].restraints.reserve_exact(100);
    let facts = nested_typed_census(&request);
    assert!(facts.max_restraint_capacity <= caps::RESTRAINTS, "every single capacity is within the cap");
    assert!(facts.restraint_capacity > caps::RESTRAINTS, "the total is above it: {}", facts.restraint_capacity);
    assert_eq!(assess(&capture, &request, Entry::Direct, RETAINED_LIMIT).law().domain,
        Some(AdmissionRefusal::Cap { fact: CapFact::RestraintCapacityTotal, observed: facts.restraint_capacity, cap: caps::RESTRAINTS }));
}

/// S-1 (V07, V08): the typed walk reads the request-level materials and every
/// temperature-point id, in both material lists, for length and for capacity.
#[test]
fn typed_walk_reads_request_materials_and_temperature_point_ids() {
    type Change = Box<dyn Fn(&mut LinearStaticPreviewRequest)>;
    let over = caps::TEXT_BYTES + 1;
    let rows: Vec<(&str, CapFact, Change)> = vec![
        ("request material id", CapFact::TypedTextBytes, Box::new(move |r| r.materials[3].id = text("m", over))),
        ("request material id capacity", CapFact::TypedTextCapacity, Box::new(|r| r.materials[3].id.reserve_exact(200))),
        ("request temperature-point id", CapFact::TypedTextBytes, Box::new(move |r| r.materials[2].temperature_points[15].id = text("t", over))),
        ("request temperature-point id capacity", CapFact::TypedTextCapacity,
            Box::new(|r| r.materials[2].temperature_points[15].id.reserve_exact(200))),
        ("model temperature-point id", CapFact::TypedTextBytes, Box::new(move |r| r.model.materials[1].temperature_points[7].id = text("t", over))),
        ("model temperature-point id capacity", CapFact::TypedTextCapacity,
            Box::new(|r| r.model.materials[1].temperature_points[7].id.reserve_exact(200))),
    ];
    for (label, fact, change) in rows {
        let report = admitted_typed(cap_maximal(), |r| change(r));
        assert!(cap(fact)(report.law().domain), "{label}: {:?}", report.law().domain);
        if fact == CapFact::TypedTextBytes {
            assert!(matches!(report.law().domain, Some(AdmissionRefusal::Cap { observed, .. }) if observed == over), "{label}");
        }
    }
}

/// N-2 (V17, V24): a gate sum that overflows saturates at u64::MAX (above every
/// bound), never 0; G-C's longest string reads a diagnostic's `source` and every
/// `affected_refs` entry.
#[test]
fn gate_sums_saturate_and_the_longest_string_reads_every_diagnostic_field() {
    assert_eq!(Bytes::ZERO.add(7).get(), 7);
    assert_eq!(Bytes::ZERO.add(usize::MAX).add(1).get(), u64::MAX);
    assert_eq!(Bytes::times(usize::MAX, 2).get(), u64::MAX);
    assert_eq!(Bytes::ZERO.add(3).plus(Bytes::times(usize::MAX, 3)).get(), u64::MAX);
    let mode = PreviewSolverMode::SparseInteractive;
    let (request, capture) = CapturedInvocation::parse(milestone(), mode).unwrap();
    let mut observer = crate::retained_product::ProductCapture::prepared_probe();
    let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
    let long = 200_000;
    assert!(longest_string(&ordinary) < long);
    type Change = Box<dyn Fn(&mut crate::Diagnostic)>;
    let changes: [(&str, Change); 3] = [
        ("source", Box::new(move |d| d.source = Some(text("s", long)))),
        ("affected_refs first", Box::new(move |d| d.affected_refs.insert(0, text("a", long)))),
        ("affected_refs last", Box::new(move |d| d.affected_refs.push(text("a", long)))),
    ];
    for (label, change) in changes {
        let mut e = ordinary.clone();
        change(e.diagnostics.last_mut().unwrap());
        assert_eq!(longest_string(&e), long, "{label}");
        let o = complete_observations(&CompleteFacts { ordinary: &e, capture: &observer, requested_cases: 1 });
        assert_eq!(o.iter().find(|x| x.fact == PhaseFact::EnvelopeMaxStringBytes).unwrap().observed, long as u64, "{label}");
    }
}

/// N-4: a reviewed-input record with any unreadable input (`<path>=unavailable`)
/// never binds, even when the registered text is identical: an unreadable
/// reviewed input is Stale by construction, as an unavailable identity is.
#[test]
fn an_unreadable_reviewed_input_never_binds() {
    let layouts = READER_LAYOUTS;
    let read = build_identity::encode_reviewed_inputs(&[Some([7; 32]); build_identity::REVIEWED_INPUT_COUNT]);
    assert!(!read.contains("unavailable"));
    assert!(bindings_hold(true, Some(&read), &read, &layouts, &layouts), "every input read: the record binds");
    for i in 0..build_identity::REVIEWED_INPUT_COUNT {
        let mut digests = [Some([7u8; 32]); build_identity::REVIEWED_INPUT_COUNT];
        digests[i] = None;
        let unread = build_identity::encode_reviewed_inputs(&digests);
        assert!(unread.contains("=unavailable"));
        assert!(!bindings_hold(true, Some(&unread), &unread, &layouts, &layouts), "input {i} unreadable");
    }
    let none = build_identity::encode_reviewed_inputs(&[None; build_identity::REVIEWED_INPUT_COUNT]);
    assert!(!bindings_hold(true, Some(&none), &none, &layouts, &layouts), "no input read");
    assert_eq!(identity_match(Some(build_identity::IDENTITY_UNAVAILABLE), [build_identity::IDENTITY_UNAVAILABLE].into_iter()),
        Err(ProfileStatus::Stale), "the identity's own unavailable value is Stale");
    assert!(COMPILED_REVIEWED_INPUTS.is_some_and(|text| !text.contains("unavailable")), "this build read every reviewed input");
}

// ---- G6: G-C declines W1 when the ordinary solve was not attempted ---------------

/// D1 inputs whose ordinary route returns before attempting the case's solve (ROOT's G6
/// ruling): an invalid document kind (validation), an invalid load category (the
/// load-application findings, lib.rs `solve_load_case_observed`), no supports (validation) and a lone spring
/// support (the mechanism refusal before the attempt). Every one is inside D1.
pub(super) fn not_attempted_examples() -> Vec<(&'static str, Value)> {
    let mut doc = milestone();
    doc["model"]["document_kind"] = json!("invalid-kind");
    let mut category = milestone();
    category["model"]["load_cases"][0]["primitive_loads"][0]["category"] = json!("not_a_category");
    let mut unsupported = milestone();
    unsupported["model"]["supports"] = json!([]);
    let mut mechanism = milestone();
    let spring = mechanism["model"]["supports"][1].clone();
    mechanism["model"]["supports"] = json!([spring]);
    vec![("document kind", doc), ("load category", category), ("no supports", unsupported), ("lone spring", mechanism)]
}
/// D1 inputs whose ordinary solve ran: the milestone (Sensitive), a 1e-300 spring whose
/// attempt fails NumericallyUnresolved (a blocked envelope after the attempt), K2a's
/// partial-underflow product reach (F1b's deferred formation: the attempt's seed is
/// `FormationFailure`; RV89 G6 S-2), and the rejected_stress_range pair (solved Sensitive,
/// then blocked by the legacy source-block finalization).
pub(super) fn attempted_examples() -> Vec<(&'static str, Value)> {
    let mut failed = milestone();
    failed["model"]["supports"][1]["stiffness"]["value"]["value"] = json!(1e-300);
    vec![
        ("milestone", milestone()),
        ("failed attempt", failed),
        (DEFERRED_FORMATION, k2a_partial_underflow()),
        ("rejected_stress_range sparse", current_rejected_stress_range(include_str!(
            "../../../fixtures/product_preview/w1_controls/rejected_stress_range/sparse_interactive.request.json"))),
        ("rejected_stress_range dense", current_rejected_stress_range(include_str!(
            "../../../fixtures/product_preview/w1_controls/rejected_stress_range/dense_scrutiny.request.json"))),
    ]
}
/// The rejected_stress_range pair is a pinned historical capture (its CAPTURE/ORACLE records bind
/// the request and raw bytes), so its request stays the 0.1.0 document it was captured with. B7a
/// retired 0.1.0: the live examples re-author it to the current version, changing nothing else.
pub(super) fn current_rejected_stress_range(captured: &str) -> Value {
    let mut request: Value = serde_json::from_str(captured).unwrap();
    assert_eq!(request["model"]["schema_version"], json!("0.1.0"), "the historical capture");
    request["model"]["schema_version"] = json!(crate::MODEL_SCHEMA_VERSION);
    request
}
/// The attempted example whose seed is F1b's deferred formation (`FormationFailure`).
pub(super) const DEFERRED_FORMATION: &str = "deferred formation (K2a partial underflow)";
/// K2a's `product-reach-partial-underflow` shape (tests/k2a_formation_range_runtime.rs,
/// `PARTIAL_UNDERFLOW`): one 2^-20 m member N0 -> N1, OD 3e-8 m, wall 3e-9 m,
/// E = 1.3e-292 Pa, G = 1e-200 Pa, N1 free in UY only, a 1e-307 N load. Its formation is
/// deferred (F1b `RangeDeferred`); it is inside D1 (RV89 G6 S-2).
pub(super) fn k2a_partial_underflow() -> Value {
    const PROV: &str = "invented_t3_k2a_product_reach_input_no_library_data";
    let all = ["UX", "UY", "UZ", "RX", "RY", "RZ"];
    let anchored: Vec<&str> = all.iter().copied().filter(|d| *d != "UY").collect();
    json!({
        "model": {
            "schema_version": "0.8.0",
            "document_kind": "openpipestress.product_preview.model",
            "analysis_status": {
                "mechanics": "ready_for_preview_diagnostics",
                "rule_check": "not_performed_user_rule_inputs_missing",
                "professional_acceptance": "not_provided"
            },
            "project": {
                "id": "invented:t3-k2a:product-reach-partial-underflow",
                "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "temperature": "degC", "stress": "Pa"}
            },
            "nodes": [
                {"id": "N0", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": PROV},
                {"id": "N1", "position": {"x": 9.5367431640625e-07, "y": 0.0, "z": 0.0}, "provenance": PROV}
            ],
            "pipe_segments": [{
                "id": "M1", "from": "N0", "to": "N1", "material": "mat:K2A",
                "y_reference": {"x": 0, "y": 1, "z": 0},
                "section": {"outside_diameter": {"value": 3.0e-8, "unit": "m"}, "wall_thickness": {"value": 3.0e-9, "unit": "m"}},
                "provenance": PROV
            }],
            "materials": [{
                "id": "mat:K2A",
                "elastic_modulus": {"value": 1.3e-292, "unit": "Pa"},
                "shear_modulus": {"value": 1.0e-200, "unit": "Pa"},
                "provenance": PROV
            }],
            "supports": [
                {"id": "rigid:N0", "node": "N0", "restraints": all, "family": "anchor", "provenance": PROV},
                {"id": "rigid:N1", "node": "N1", "restraints": anchored, "family": "anchor", "provenance": PROV}
            ],
            "load_cases": [{
                "id": "case", "label": "product-reach-partial-underflow", "kind": "primitive_user_load",
                "primitive_loads": [{"id": "load:0", "category": "concentrated_force", "target": {"type": "node", "node": "N1"},
                    "direction": "global_y", "magnitude": {"value": 1.0e-307, "unit": "N"}, "dimension": "force",
                    "provenance": PROV}],
                "provenance": PROV
            }],
            "combinations": []
        },
        "materials": []
    })
}
#[test]
fn g_c_declines_w1_when_the_ordinary_solve_was_not_attempted() {
    let caps = phase_caps().complete;
    assert_eq!(caps[COMPLETE_FACTS - 1], 0, "the bound is 0");
    for (attempted, examples) in [(false, not_attempted_examples()), (true, attempted_examples())] {
        for (label, raw) in examples {
            for mode in MODES {
                let (request, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
                assert_eq!(assess(&capture, &request, Entry::Direct, RETAINED_LIMIT).law().domain, None, "{label}: inside D1");
                let mut observer = crate::retained_product::ProductCapture::prepared_probe();
                let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
                assert_eq!(ordinary_solve_attempted(&observer, 1), attempted, "{label} {mode:?}");
                if label == DEFERRED_FORMATION {
                    // RV89 G6 S-2: the deferred-formation arm is the one this example reaches.
                    assert_eq!(observer.ordinary.len(), 1, "{label} {mode:?}");
                    assert!(matches!(observer.ordinary[0].initial, Some(crate::retained_product::InitialSeed::FormationFailure { .. })),
                        "{label} {mode:?}: the seed is F1b's FormationFailure, got {:?}", observer.ordinary[0].initial);
                }
                let o = complete_observations(&CompleteFacts { ordinary: &ordinary, capture: &observer, requested_cases: 1 });
                let fact = o.iter().find(|x| x.fact == PhaseFact::OrdinarySolveNotAttempted).unwrap();
                assert_eq!(fact.observed, u64::from(!attempted), "{label} {mode:?}");
                let checked = check_phase(PhaseGate::Complete, &o, &caps);
                if attempted {
                    assert_eq!(checked, Ok(()), "{label} {mode:?}: a solve that ran, at any quality, proceeds to W1");
                } else {
                    assert_eq!(checked, Err(PhaseRefusal { gate: PhaseGate::Complete, fact: PhaseFact::OrdinarySolveNotAttempted, observed: 1, cap: 0 }), "{label} {mode:?}");
                    assert_eq!(ordinary.status.mechanics, "MODEL_INCOMPLETE", "{label}: a blocked envelope");
                }
            }
        }
    }
    // The predicate reads the observer, not the envelope: no seed, or a seed whose
    // attempt outcome is unset, is not attempted.
    let mut empty = crate::retained_product::ProductCapture::prepared_probe();
    assert!(!ordinary_solve_attempted(&empty, 1));
    let raw = milestone();
    let (request, capture) = CapturedInvocation::parse(raw, PreviewSolverMode::SparseInteractive).unwrap();
    let _ = crate::run_linear_static_preview_observed(request, PreviewSolverMode::SparseInteractive, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut empty));
    assert!(ordinary_solve_attempted(&empty, 1));
    empty.ordinary[0].initial = None;
    assert!(!ordinary_solve_attempted(&empty, 1), "a seed without an attempt outcome");
}

/// G6 (ROOT's ruling 2(a)), in the registered build: a D1 request whose ordinary route
/// returns without attempting the case's solve takes G-C's CompleteGate fallback, and
/// Direct publishes the value route's bytes exactly, with no notice. The milestone still
/// publishes, and a solve that ran but failed still reaches W1.
#[test]
fn registered_g_c_declines_only_unattempted_solves() {
    if COMPILED_IDENTITY != Some(REGISTERED_PROFILES[0].identity) {
        return;
    }
    for mode in MODES {
        for (label, raw) in not_attempted_examples() {
            let plain = serde_json::to_vec(&crate::run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap();
            let direct = crate::run_linear_static_preview_value_with_retained_direct(raw, mode, RETAINED_LIMIT).unwrap();
            assert_eq!(direct.admission().unwrap().law().refusal, None, "{label}: admitted");
            match direct.retained() {
                Some(Err(crate::W1Fallback::CompleteGate(refusal))) => assert_eq!(refusal.fact, PhaseFact::OrdinarySolveNotAttempted, "{label} {mode:?}"),
                other => panic!("{label} {mode:?}: expected G-C's decline, got {other:?}"),
            }
            assert_eq!(serde_json::to_vec(direct.envelope()).unwrap(), plain, "{label} {mode:?}: the value route's bytes exactly, no notice");
            assert!(direct.successor().is_none());
        }
        let milestone = crate::run_linear_static_preview_value_with_retained_direct(milestone(), mode, RETAINED_LIMIT).unwrap();
        assert!(milestone.successor().is_some(), "{mode:?}: the milestone publishes");
        for (label, raw) in attempted_examples().into_iter().skip(1) {
            let direct = crate::run_linear_static_preview_value_with_retained_direct(raw, mode, RETAINED_LIMIT).unwrap();
            match direct.retained() {
                Some(Err(crate::W1Fallback::CompleteGate(refusal))) => panic!("{label} {mode:?}: G-C declined a solve that ran ({refusal:?})"),
                Some(_) => {}
                None => panic!("{label} {mode:?}: no W1 result"),
            }
        }
    }
}

// ---- B1 SA: admission at option S3 (PLAN_v2 §2.3) ----------------------------------

/// D1.4 admits 1 ≤ c ≤ C load cases. c = 0 and c = C + 1 refuse with `(Invocation,
/// LoadCases)`; c = 1, 2 and 3 are inside D1, and the registered build grants each a permit,
/// in both modes. (A permit is not a solve: nothing runs here.)
#[test]
fn b1_sa_d1_4_admits_one_to_c_load_cases() {
    assert_eq!(caps::LOAD_CASES, 3, "C = 3 (option S3)");
    let registered = COMPILED_IDENTITY == Some(REGISTERED_PROFILES[0].identity);
    for c in 0..=caps::LOAD_CASES + 1 {
        let expected = if (1..=caps::LOAD_CASES).contains(&c) { None } else { family(D1Clause::Invocation, FamilyFact::LoadCases) };
        for mode in MODES {
            let (request, capture) = CapturedInvocation::parse(milestone_cases(c), mode).unwrap();
            let admitted = admit(&capture, &request, Entry::Direct, RETAINED_LIMIT);
            let report = match &admitted {
                Ok((_, report)) | Err(report) => *report,
            };
            assert_eq!((report.typed.load_cases.length, report.typed.load_cases.capacity), (c, c), "c = {c}");
            assert_eq!(report.law().domain, expected, "c = {c} {mode:?}");
            assert_eq!(report.law().refusal, d1_1_refusal().or(expected), "c = {c} {mode:?}: D1.1 first, then D1.4");
            assert_eq!(admitted.is_ok(), registered && expected.is_none(), "c = {c} {mode:?}: a permit only inside D1, in the registered build");
        }
    }
}

/// The runner's out-of-domain oracle (`core/runner/headless/tests/retained_precision_admission.rs`,
/// `explicit_headless_refusal_preserves_output_and_completion_fields_both_modes`) cannot read
/// `pub(crate)` `caps`, and no D1 item's visibility changes for a test, so it builds a literal
/// C_eq + 1 = 4 case-equivalents: the milestone's case three times (C) and one mechanics
/// combination. B2-A re-bases it from B1's C + 1 load cases (PLAN_v2 SF-2) to C_eq + 1 (I93
/// PLAN §1.2.4; B2-C §9). This test builds the same input and ties the literal to the producer:
/// 4 is `CASE_EQUIVALENTS + 1`, and `admit` refuses it at D1.9's `CaseEquivalents` row. If C_eq
/// or C changes, this test fails first and names the runner test to re-base.
fn runner_oracle() -> Value {
    let mut raw = milestone();
    let case = raw["model"]["load_cases"][0].clone();
    for _ in 1..caps::LOAD_CASES {
        raw["model"]["load_cases"].as_array_mut().unwrap().push(case.clone());
    }
    raw["model"]["combinations"] = json!([{"id": "combination:c-eq", "basis": "mechanics", "terms": [{"load_case": "case", "factor": 1.0}]}]);
    raw
}
#[test]
fn b2_a_runner_oracle_literal_is_case_equivalents_plus_one() {
    const RUNNER_LITERAL: usize = 4;
    assert_eq!(caps::CASE_EQUIVALENTS + 1, RUNNER_LITERAL, "re-base the runner's literal (explicit_headless_refusal_preserves_output_and_completion_fields_both_modes)");
    assert_eq!((caps::LOAD_CASES, RUNNER_LITERAL - caps::LOAD_CASES), (3, 1), "the runner builds C cases and one combination");
    // B1 SQ (RV112 N-1): the tie runs both ways. The runner's workspace is Stale, so its own test
    // passes with any literal; this test reads the runner's source and holds its one literal
    // line to the literal here, so a change of C_eq or of the runner's literal alone fails here.
    // B2-A re-based the runner's C + 1 load cases to C_eq + 1 case-equivalents (C cases and one
    // combination: `C_EQ_PLUS_ONE`, `COMBINATIONS`); J0a carries B1's tie over to those lines.
    let runner = include_str!("../../runner/headless/tests/retained_precision_admission.rs");
    let lines: Vec<&str> = runner.lines().filter(|l| l.contains("const C_EQ_PLUS_ONE")).collect();
    assert_eq!(lines.len(), 1, "the runner holds one C_EQ_PLUS_ONE literal");
    assert_eq!(lines[0].trim(), format!("const C_EQ_PLUS_ONE: usize = {};", caps::CASE_EQUIVALENTS + 1), "the runner's literal is C_eq + 1");
    assert!(runner.lines().any(|l| l.trim() == format!("const COMBINATIONS: usize = {};", RUNNER_LITERAL - caps::LOAD_CASES)), "the runner's one combination");
    assert!(runner.contains("fn explicit_headless_refusal_preserves_output_and_completion_fields_both_modes()"));
    let expected = Some(AdmissionRefusal::Cap { fact: CapFact::CaseEquivalents, observed: RUNNER_LITERAL, cap: caps::CASE_EQUIVALENTS });
    for mode in MODES {
        let (request, capture) = CapturedInvocation::parse(runner_oracle(), mode).unwrap();
        let report = admit(&capture, &request, Entry::Direct, RETAINED_LIMIT).err().expect("C_eq + 1 is refused in every build");
        assert_eq!(report.typed.load_cases.length + report.typed.combinations.length, RUNNER_LITERAL);
        assert_eq!(report.law().domain, expected, "{mode:?}");
        assert_eq!(report.law().refusal, d1_1_refusal().or(expected), "{mode:?}");
    }
}

/// D1.9's new `TotalLoads` row (Σ l_i ≤ L) at its cap and at cap + 1. Every case ≤ l and
/// c ≤ C give Σ ≤ C·l = L, so no request reaches L + 1 (the row is stated, not binding);
/// cap + 1 is shown on the actual census of the cap-maximal C-case input with its total raised.
#[test]
fn b1_sa_total_loads_row_admits_l_and_refuses_l_plus_one() {
    let raw = cap_maximal_cases(caps::LOAD_CASES);
    let request: LinearStaticPreviewRequest = serde_json::from_value(raw.clone()).unwrap();
    let report = admitted(raw);
    let law = report.law();
    assert_eq!((law.nested.total_loads as usize, law.nested.primitive_loads.length), (caps::TOTAL_LOADS, caps::LOADS));
    let mut nested = law.nested;
    for (total, expected) in [
        (caps::TOTAL_LOADS, Ok(())),
        (caps::TOTAL_LOADS + 1, Err(AdmissionRefusal::Cap { fact: CapFact::TotalLoads, observed: 385, cap: 384 })),
    ] {
        nested.total_loads = total as u32;
        let facts = DomainFacts { raw: &report.raw, raw_text: &law.raw_text, typed: &report.typed, nested: &nested, headless: None, digest: &report.captured_digest, combinations: &NO_COMBINATIONS };
        assert_eq!(domain_clauses(&facts, &request), expected, "Σ l_i = {total}");
    }
}

/// G-B (PLAN_v2 §2.3; RV107 SF-4; RV109 N-3): `CaseLoads` reads the case at hand (≤ l) and the
/// new `CaseLoadsTotal` the capture's running total `late_loads_total` (ST's seam; ≤ L), so G-B
/// at case k bounds Σ_{i≤k} l_i. Each refuses one above its cap whatever the other reads, in
/// table order. A saturated total (`usize::MAX`, the reading a saturating seam would give on
/// overflow) is above every bound, so G-B refuses typed. In the registered build, the actual
/// seam has added the one case's loads by G-B: Σ = l_0 at c = 1.
#[test]
fn b1_sa_g_b_reads_each_case_and_the_running_total() {
    let late = phase_caps().late;
    assert_eq!((late[4], late[5]), (caps::LOADS as u64, caps::TOTAL_LOADS as u64), "CaseLoads ≤ l, CaseLoadsTotal ≤ L");
    let mode = PreviewSolverMode::SparseInteractive;
    let (request, _) = CapturedInvocation::parse(milestone(), mode).unwrap();
    let mut diagnostics = Vec::new();
    let built = crate::build_model(&request.model, &request.model.materials, &mut diagnostics).expect("the milestone builds");
    let model = request.model;
    let restrained = [0usize, 1, 2, 3, 4, 5, 6];
    let springs: [crate::SpringEntry; 0] = [];
    let load = model.load_cases[0].primitive_loads[0].clone();
    // LateObservationBytes is not under test here.
    let mut bounds = late;
    bounds[LATE_FACTS - 1] = u64::MAX;
    let gate = |case_loads: usize, total: usize| {
        let mut case = model.load_cases[0].clone();
        case.primitive_loads = vec![load.clone(); case_loads];
        let mut capture = crate::retained_product::ProductCapture::prepared_probe();
        capture.late_loads_total = total;
        let o = late_observations(&LateFacts { model: &model, built: &built, materials: &model.materials, case: &case, restrained: &restrained, springs: &springs, capture: &capture });
        assert_eq!(o.map(|x| x.fact), [PhaseFact::BuiltNodes, PhaseFact::BuiltMembers, PhaseFact::BuiltFrameElements, PhaseFact::BuiltSupports,
            PhaseFact::CaseLoads, PhaseFact::CaseLoadsTotal, PhaseFact::Restrained, PhaseFact::Springs, PhaseFact::Materials,
            PhaseFact::LateObservationBytes], "G-B's facts, in table order");
        let get = |fact| o.iter().find(|x| x.fact == fact).unwrap().observed;
        assert_eq!((get(PhaseFact::CaseLoads), get(PhaseFact::CaseLoadsTotal)), (case_loads as u64, total as u64));
        check_phase(PhaseGate::Late, &o, &bounds).map_err(|r| (r.fact, r.observed, r.cap))
    };
    // The case at hand, and the running total over the cases so far.
    assert_eq!(gate(3, 3), Ok(()), "c = 1: the milestone's own case");
    assert_eq!(gate(3, 3 + 3 + 3), Ok(()), "case 2 of the milestone's three cases");
    assert_eq!(gate(128, 384), Ok(()), "case 2 of the cap-maximal three: l and L");
    assert_eq!(gate(3, 384), Ok(()));
    assert_eq!(gate(3, 385), Err((PhaseFact::CaseLoadsTotal, 385, 384)), "the total over L although the case at hand is small");
    assert_eq!(gate(129, 129), Err((PhaseFact::CaseLoads, 129, 128)), "the case at hand over l although the total is small");
    assert_eq!(gate(129, 385), Err((PhaseFact::CaseLoads, 129, 128)), "table order: the case before the total");
    assert_eq!(gate(3, usize::MAX), Err((PhaseFact::CaseLoadsTotal, u64::MAX, 384)), "a saturated total refuses at G-B");
    // The actual seam, in the registered build: by G-B the capture holds the case's loads.
    if COMPILED_IDENTITY == Some(REGISTERED_PROFILES[0].identity) {
        for mode in MODES {
            let (request, capture) = CapturedInvocation::parse(milestone(), mode).unwrap();
            let (permit, _report) = admit(&capture, &request, Entry::Direct, RETAINED_LIMIT).unwrap_or_else(|r| panic!("{mode:?}: {:?}", r.law().refusal));
            let mut observer = crate::retained_product::ProductCapture::permitted_probe(permit);
            let _ = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
            assert!(observer.late_refusal().is_none(), "{mode:?}: G-B admitted the milestone");
            assert_eq!(observer.late_loads_total, 3, "{mode:?}: Σ l_i = l_0 at G-B");
        }
    }
}

/// G-C (RV107 N-13): the result rows, their capacity and their text are bounded per invocation
/// by C·P_final, PushCap(C·P_final) and 2·C·P_final·Text(row). The gate admits C·P_final rows
/// and refuses one more. A real C-case run of a solvable cap-maximal model (W2b's shape) has
/// more rows than one case's P_final allows, so a P_final bound would decline an input inside D1.
#[test]
fn b1_sa_envelope_results_bound_is_c_times_p_final() {
    let caps = phase_caps().complete;
    let c = caps::LOAD_CASES as u64;
    assert_eq!((caps[0], caps[1], caps[2]), (c * P_FINAL, push_capacity(c * P_FINAL), 2 * c * P_FINAL * text_atoms::ROW));
    let base = complete_observations_of_milestone();
    assert_eq!(base[0].fact, PhaseFact::EnvelopeResults);
    for (rows, expected) in [(c * P_FINAL, Ok(())), (c * P_FINAL + 1, Err((PhaseFact::EnvelopeResults, c * P_FINAL + 1, c * P_FINAL)))] {
        let mut o = base;
        o[0].observed = rows;
        assert_eq!(check_phase(PhaseGate::Complete, &o, &caps).map_err(|r| (r.fact, r.observed, r.cap)), expected, "{rows} rows");
    }
    for mode in MODES {
        let rows = |cases: usize| {
            let (request, capture) = CapturedInvocation::parse(solvable_cap_maximal_cases(cases), mode).unwrap();
            let mut observer = crate::retained_product::ProductCapture::prepared_probe();
            let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
            assert_eq!(ordinary.status.mechanics, "MECHANICS_SOLVED", "{mode:?}, {cases} cases");
            let o = complete_observations(&CompleteFacts { ordinary: &ordinary, capture: &observer, requested_cases: cases });
            let get = |fact| o.iter().find(|x| x.fact == fact).unwrap().observed;
            (get(PhaseFact::EnvelopeResults), get(PhaseFact::EnvelopeResultCapacity), get(PhaseFact::EnvelopeResultTextBytes))
        };
        let (one, three) = (rows(1), rows(caps::LOAD_CASES));
        println!("I89_B1_SA_ENVELOPE_RESULTS mode={} one={:?} three={:?} p_final={P_FINAL}", mode.as_str(), one, three);
        assert!(one.0 <= P_FINAL && one.1 <= caps[1] && one.2 <= caps[2], "{mode:?}: one case within one case's rows");
        assert!(three.0 > P_FINAL, "{mode:?}: C cases exceed one case's P_final ({} rows)", three.0);
        assert!(three.0 <= caps[0] && three.1 <= caps[1] && three.2 <= caps[2], "{mode:?}: C cases within C·P_final");
    }
}

/// T-3 (e) (DESIGN_v2 §1.2; RV105 N-1): G-C's attempt fact counts the requested cases. It holds
/// exactly when c ≥ 1, the capture has one seed per requested case, and every seed's `initial`
/// is set. A run that blocks at case k < c − 1 leaves the later cases unseeded although every
/// seed it left is attempted: G-C declines it, and Direct publishes the exact ordinary bytes
/// with no notice.
#[test]
fn b1_sa_t3_e_counts_the_requested_cases() {
    use crate::retained_product::ProductCapture;
    let mode = PreviewSolverMode::SparseInteractive;
    let (request, capture) = CapturedInvocation::parse(milestone(), mode).unwrap();
    let mut one = ProductCapture::prepared_probe();
    let _ = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut one));
    let attempted = one.ordinary[0].clone();
    assert!(attempted.initial.is_some());
    let mut unattempted = attempted.clone();
    unattempted.initial = None;
    let seeds = |seeds: Vec<crate::retained_product::OrdinarySeed>| {
        let mut capture = ProductCapture::prepared_probe();
        capture.ordinary = seeds;
        capture
    };
    assert!(!ordinary_solve_attempted(&seeds(vec![]), 0), "no requested case");
    assert!(!ordinary_solve_attempted(&seeds(vec![]), 1), "no seed");
    for requested in 1..=caps::LOAD_CASES {
        assert!(ordinary_solve_attempted(&seeds(vec![attempted.clone(); requested]), requested), "{requested}: one attempted seed per case");
        assert!(!ordinary_solve_attempted(&seeds(vec![attempted.clone(); requested - 1]), requested), "{requested}: a requested case without a seed");
        assert!(!ordinary_solve_attempted(&seeds(vec![attempted.clone(); requested + 1]), requested), "{requested}: more seeds than requested cases");
        let mut last_unattempted = vec![attempted.clone(); requested];
        last_unattempted[requested - 1] = unattempted.clone();
        assert!(!ordinary_solve_attempted(&seeds(last_unattempted), requested), "{requested}: a seed without an attempt outcome");
    }
    // Actual runs of C cases that block at case k < c − 1 (the private route: no admission).
    let mut failed = milestone();
    failed["model"]["supports"][1]["stiffness"]["value"]["value"] = json!(1e-300);
    let failed_first = with_cases(failed, caps::LOAD_CASES);
    let mut invalid_second = milestone_cases(caps::LOAD_CASES);
    invalid_second["model"]["load_cases"][1]["primitive_loads"][0]["category"] = json!("not_a_category");
    let blocked = [("case 0's attempt fails and blocks", failed_first, 1), ("case 1 blocks before its attempt", invalid_second, 1)];
    let complete_caps = phase_caps().complete;
    for (label, raw, left) in &blocked {
        for mode in MODES {
            let (request, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
            assert_eq!(assess(&capture, &request, Entry::Direct, RETAINED_LIMIT).law().domain, None, "{label}: inside D1");
            let mut observer = ProductCapture::prepared_probe();
            let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
            assert_eq!(ordinary.status.mechanics, "MODEL_INCOMPLETE", "{label} {mode:?}: a blocked envelope");
            assert_eq!(observer.ordinary.len(), *left, "{label} {mode:?}: the later cases are unseeded");
            assert!(observer.ordinary.iter().all(|seed| seed.initial.is_some()), "{label} {mode:?}: every seed it left is attempted");
            assert!(!ordinary_solve_attempted(&observer, caps::LOAD_CASES), "{label} {mode:?}");
            let o = complete_observations(&CompleteFacts { ordinary: &ordinary, capture: &observer, requested_cases: caps::LOAD_CASES });
            assert_eq!(check_phase(PhaseGate::Complete, &o, &complete_caps),
                Err(PhaseRefusal { gate: PhaseGate::Complete, fact: PhaseFact::OrdinarySolveNotAttempted, observed: 1, cap: 0 }), "{label} {mode:?}");
        }
    }
    // Every case attempted: the milestone with C cases passes the fact.
    for mode in MODES {
        let (request, capture) = CapturedInvocation::parse(milestone_cases(caps::LOAD_CASES), mode).unwrap();
        let mut observer = ProductCapture::prepared_probe();
        let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
        assert_eq!(observer.ordinary.len(), caps::LOAD_CASES, "{mode:?}");
        assert!(ordinary_solve_attempted(&observer, caps::LOAD_CASES), "{mode:?}");
        let o = complete_observations(&CompleteFacts { ordinary: &ordinary, capture: &observer, requested_cases: caps::LOAD_CASES });
        assert_eq!(o[COMPLETE_FACTS - 1], PhaseObservation { fact: PhaseFact::OrdinarySolveNotAttempted, observed: 0 }, "{mode:?}");
    }
    // Direct, in the registered build: admitted (c = C is inside D1), G-C declines, and the
    // published bytes are exactly the value route's, with no notice and no successor.
    if COMPILED_IDENTITY != Some(REGISTERED_PROFILES[0].identity) {
        return;
    }
    for (label, raw, _) in blocked {
        for mode in MODES {
            let plain = serde_json::to_vec(&crate::run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap();
            let direct = crate::run_linear_static_preview_value_with_retained_direct(raw.clone(), mode, RETAINED_LIMIT).unwrap();
            assert_eq!(direct.admission().unwrap().law().refusal, None, "{label} {mode:?}: admitted");
            match direct.retained() {
                Some(Err(crate::W1Fallback::CompleteGate(r))) => assert_eq!((r.gate, r.fact, r.observed, r.cap),
                    (PhaseGate::Complete, PhaseFact::OrdinarySolveNotAttempted, 1, 0), "{label} {mode:?}"),
                other => panic!("{label} {mode:?}: expected G-C's decline, got {other:?}"),
            }
            assert_eq!(serde_json::to_vec(direct.envelope()).unwrap(), plain, "{label} {mode:?}: exact bytes, no notice");
            assert!(direct.successor().is_none(), "{label} {mode:?}");
        }
    }
}

/// SF-4's back edge (PLAN_v2 §2.3): every gate bound at C = 3, by its fact. The bounds that read
/// the regenerated profile (`profile::F_T11`, `F_T11_LATE_CAPTURE`, `F_T11_ORDINARY_SEED`,
/// `text_atoms::*`) are asserted as expressions: the named form, or the atom times its count.
/// SQ fixes their values; RV-Q round 1 reviews these expressions. The bounds that read only D1's
/// caps are asserted by value.
#[test]
fn b1_sa_gate_bounds_at_c_are_the_stated_expressions() {
    use profile::{F_T11, F_T11_LATE_CAPTURE, F_T11_ORDINARY_SEED};
    use text_atoms as t;
    let p = phase_caps();
    let (n, m, g, c) = (32u64, 32u64, 32u64, caps::LOAD_CASES as u64);
    // G-B's facts are in `late_observations`' order (b1_sa_g_b_reads_each_case_and_the_running_total).
    let late = [n, m, m, g, 128, 384, 192, 192, 8, profile_bytes(F_T11).saturating_sub(profile_bytes(F_T11_LATE_CAPTURE) / c)];
    assert_eq!(p.late, late, "G-B: n, m, m, g, l, L, min(6n, Σr), s, 4 + 4, T11 − T11_late_capture / C (RV112 SF-1)");
    let observed: Vec<PhaseFact> = complete_observations_of_milestone().iter().map(|o| o.fact).collect();
    use PhaseFact as F;
    let complete: [(PhaseFact, u64); COMPLETE_FACTS] = [
        (F::EnvelopeResults, c * P_FINAL),
        (F::EnvelopeResultCapacity, push_capacity(c * P_FINAL)),
        (F::EnvelopeResultTextBytes, 2 * c * P_FINAL * t::ROW),
        (F::EnvelopeDiagnostics, t::D_ENV),
        (F::EnvelopeDiagnosticCapacity, push_capacity(t::D_ENV)),
        (F::EnvelopeDiagnosticTextBytes, 2 * t::DIAG_ENV),
        (F::EnvelopeMaxStringBytes, t::L_PUB),
        (F::DiagnosticIdMaxBytes, t::L_DIAGID),
        (F::ContractEvidenceStatus, 0),
        (F::ContractEvidenceArrayElements, c * 160),
        (F::ContractEvidenceObjects, c * 67),
        // T3 F1: cut from C·553 = 1,659 so that objects + ⌊entries / 5⌋ ≤ C·(67 + ⌊553 / 5⌋) = 531,
        // the BTree nodes the profile prices for the preview tree.
        (F::ContractEvidenceEntries, 5 * (c * (67 + 553 / 5) - c * 67) + 4),
        (F::ContractEvidenceStringBytes, c * 66_816),
        (F::ContractEvidenceKeyBytes, c * 22_120),
        (F::SourceBlockRecovery, 0),
        (F::ObservationBytes, profile_bytes(F_T11)),
        (F::OrdinarySeedBytes, profile_bytes(F_T11_ORDINARY_SEED)),
        (F::RetainedErrorTextBytes, c * (3 * m + 1) * t::ERR),
        (F::OrdinarySolveNotAttempted, 0),
    ];
    assert_eq!(observed, complete.map(|(fact, _)| fact), "G-C's facts, in table order");
    assert_eq!(p.complete, complete.map(|(_, bound)| bound), "G-C at C = 3");
    // The per-case contract-evidence facts, at D1's caps (ordinary_caps.py's PREVIEW facts).
    assert_eq!((3 * m + 2 * g, 3 + m + g, 9 + 15 * m + 2 * g), (160, 67, 553));
    assert_eq!((m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64), (9 + 15 * m + 2 * g) * 40), (66_816, 22_120));
    // The budgets keep the profile's forms; B-6 is one notice reserve per case in A (|A| ≤ C).
    let b = PHASE_BUDGETS;
    assert_eq!((b.staged_copy_bytes, b.successor_bytes, b.precommit_invocation_bytes, b.reader_statics_bytes),
        (profile_bytes(profile::F_STAGED), profile_bytes(profile::F_SUCC), profile_bytes(profile::F_INVOC), profile_bytes(profile::F_STATICS)));
    assert_eq!(b.precommit_reader_bytes, checked_or_zero(profile::t17(&profile::ATOM_VALUES)));
    assert_eq!(b.notice_reserve_bytes, c * NOTICE_RESERVE_BYTES);
}

/// G-C's RetainedErrorTextBytes (C-N4) over every requested case's slot (B1 SA after SP's T-2;
/// ROOT's note after R3′). A two-case run parks case 0's slot when case 1's early hook runs, so
/// the text of both slots' `error` and `observable_error` is observed: the parked slot's as
/// well as the capture's own fields'. (The bound's check against SP's producer is phase 4's.)
#[test]
fn b1_sa_retained_error_text_reads_every_case_slot() {
    use crate::retained_product::{CaptureError, ProductCapture};
    for mode in MODES {
        let (request, capture) = CapturedInvocation::parse(milestone_cases(2), mode).unwrap();
        let mut observer = ProductCapture::prepared_probe();
        let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
        assert_eq!((observer.cases_seen(), observer.parked_cases().len()), (2, 1), "{mode:?}: case 0 is parked, case 1 is in the capture's fields");
        observer.with_case(0, |case| {
            case.error = Some(CaptureError::Association(String::with_capacity(40)));
            case.observable_error = Some(CaptureError::Association(String::with_capacity(9)));
        });
        observer.error = Some(CaptureError::Association(String::with_capacity(7)));
        observer.observable_error = Some(CaptureError::Association(String::with_capacity(3)));
        let o = complete_observations(&CompleteFacts { ordinary: &ordinary, capture: &observer, requested_cases: 2 });
        assert_eq!(o.iter().find(|x| x.fact == PhaseFact::RetainedErrorTextBytes).unwrap().observed, 40 + 9 + 7 + 3, "{mode:?}: both slots");
    }
}

// ---- B3a dropped: D1.3 refuses 0.3.0 `legacy_pressure_v1` (RR "Owner decisions: the legacy ----
// ---- pressure contract is retired product-wide; …"; formerly B3a-A, I93 PLAN §1.3; B3-D §4.1) ----

/// `raw` carrying the retired `legacy_pressure_v1` label with zero pressure: I99's `legacy3`
/// (B3-W's `m3l` when `raw` is the milestone): the contract changes, nothing else. B3a is
/// dropped, so this is a refusal witness. B7b retired 0.3.0 with v2, so the label is authored on
/// the current 0.8.0 document, the one document it can still appear on (`m3l` at 0.3.0 is
/// refused by its version first; `b3a_dropped_d1_3_refuses_the_legacy_pressure_contract`).
fn legacy3(mut raw: Value) -> Value {
    raw["model"]["schema_version"] = json!(crate::MODEL_SCHEMA_VERSION);
    raw["model"]["pressure_contract"] = json!({"version": "1.0.0", "mode": "legacy_pressure_v1"});
    raw
}
/// B7b (D4 (a)): branch E's one contract, `3.0.0/exact_pressure_v3` with an Euler-Bernoulli shear model.
fn exact_contract() -> Value {
    json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"})
}

/// B7a (b7-plan "Model format, pressure-free half"): branch L keys on the pressure contract's
/// absence, not on the version string. A current-version (0.8.0) document is on branch L only
/// while it carries no contract; B7b: with branch E's exact v3 contract it is on branch E, with
/// any other contract it is refused by that contract (`PressureContract`), and every retired
/// version (0.1.0–0.4.0) is on no branch (`SchemaVersion`), with or without a contract. An
/// explicit null contract is not absence: the presence-keyed parse refuses it. Keying branch L
/// on the version string alone (`schema == 0.8.0`) or on the retired versions fails this test.
#[test]
fn b7a_branch_l_keys_on_the_contracts_absence_not_the_version() {
    use FamilyFact as F;
    let authored = |schema: &str, contract: Option<Value>| {
        let mut raw = milestone();
        raw["model"]["schema_version"] = json!(schema);
        match contract {
            Some(contract) => raw["model"]["pressure_contract"] = contract,
            None => { raw["model"].as_object_mut().unwrap().remove("pressure_contract"); }
        }
        raw
    };
    let branch = |schema: &str, contract: Option<Value>| {
        let (request, _) = CapturedInvocation::parse(authored(schema, contract), PreviewSolverMode::SparseInteractive).unwrap();
        namespace_branch(&request.model)
    };
    assert_eq!(milestone()["model"]["schema_version"], json!(crate::MODEL_SCHEMA_VERSION), "the milestone is current");
    assert!(milestone()["model"].get("pressure_contract").is_none(), "the milestone carries no contract");
    assert_eq!(branch("0.8.0", None), Ok(NamespaceBranch::Preview), "pressure-free 0.8.0");
    assert_eq!(branch("0.8.0", Some(exact_contract())), Ok(NamespaceBranch::Exact), "0.8.0 with the exact v3 contract");
    for contract in [json!({"version": "2.0.0", "mode": "exact_straight_pressure_v2"}), json!({"version": "3.0.0", "mode": "exact_pressure_v3"}),
        json!({"version": "1.0.0", "mode": "legacy_pressure_v1"})] {
        assert_eq!(branch("0.8.0", Some(contract.clone())), Err(F::PressureContract), "0.8.0 with {contract}: the contract's presence leaves branch L");
    }
    for retired in crate::RETIRED_MODEL_SCHEMA_VERSIONS {
        assert_eq!(branch(retired, None), Err(F::SchemaVersion), "retired {retired} without a contract");
        assert_eq!(branch(retired, Some(exact_contract())), Err(F::SchemaVersion), "retired {retired} with the exact contract");
    }
    assert!(CapturedInvocation::parse(authored("0.8.0", Some(Value::Null)), PreviewSolverMode::SparseInteractive).is_err(),
        "an explicit null contract is present, and refused at parse");
}

/// B3a's drop: exactly `{1.0.0, legacy_pressure_v1}` is on no D1.3 branch, so it refuses with
/// `PressureContract`, alone and with C cases, in both modes, and no build grants it a permit.
/// Every contract that does not match its schema's branch refuses with `PressureContract`; a
/// schema in no branch with `SchemaVersion` (B3-D §4.1's refusal map, L and E; B7b: 0.3.0, the
/// historical `m3l` document, is retired, so the label is witnessed on 0.8.0). D1.3 refuses the
/// label before D1.5's and D1.7's clauses are read.
#[test]
fn b3a_dropped_d1_3_refuses_the_legacy_pressure_contract() {
    use D1Clause as C;
    use FamilyFact as F;
    for raw in [legacy3(milestone()), legacy3(milestone_cases(caps::LOAD_CASES)), legacy3(cap_maximal_cases(caps::LOAD_CASES))] {
        for mode in MODES {
            let (request, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
            assert_eq!(namespace_branch(&request.model), Err(F::PressureContract));
            let admitted = admit(&capture, &request, Entry::Direct, RETAINED_LIMIT);
            let report = match &admitted {
                Ok((_, report)) | Err(report) => *report,
            };
            assert_eq!(report.law().domain, family(C::Namespace, F::PressureContract), "{mode:?}: the label is outside D1");
            assert!(admitted.is_err(), "{mode:?}: no permit in any build");
        }
    }
    // `Value::Null` here means the contract is absent (an explicit null is a parse refusal).
    let contract = |schema: &str, contract: Value| {
        let mut raw = milestone();
        raw["model"]["schema_version"] = json!(schema);
        if contract.is_null() {
            raw["model"].as_object_mut().unwrap().remove("pressure_contract");
        } else {
            raw["model"]["pressure_contract"] = contract;
        }
        domain(raw)
    };
    let legacy = json!({"version": "1.0.0", "mode": "legacy_pressure_v1"});
    assert_eq!(contract("0.8.0", legacy.clone()), family(C::Namespace, F::PressureContract), "0.8.0 with the legacy contract");
    // B3b, B7b D4 (a): the exact contract is branch E, whose D1.5 needs explicitly empty regions.
    assert_eq!(contract("0.8.0", exact_contract()), family(C::Case, F::PressureRegions));
    for (label, schema, value, fact) in [
        ("0.8.0 with no contract", "0.8.0", Value::Null, None),
        ("0.8.0, version 1.0.1", "0.8.0", json!({"version": "1.0.1", "mode": "legacy_pressure_v1"}), Some(F::PressureContract)),
        ("0.8.0, version absent", "0.8.0", json!({"mode": "legacy_pressure_v1"}), Some(F::PressureContract)),
        ("0.8.0, mode absent", "0.8.0", json!({"version": "1.0.0"}), Some(F::PressureContract)),
        ("0.8.0, empty contract", "0.8.0", json!({}), Some(F::PressureContract)),
        ("0.8.0, another mode", "0.8.0", json!({"version": "1.0.0", "mode": "exact_straight_pressure_v2"}), Some(F::PressureContract)),
        ("0.8.0, the mode's case", "0.8.0", json!({"version": "1.0.0", "mode": "Legacy_pressure_v1"}), Some(F::PressureContract)),
        ("0.8.0 with the retired v2 contract", "0.8.0", json!({"version": "2.0.0", "mode": "exact_straight_pressure_v2"}), Some(F::PressureContract)),
        ("0.3.0 with the legacy contract (retired: m3l)", "0.3.0", legacy.clone(), Some(F::SchemaVersion)),
        ("0.3.0 with the v2 contract (retired)", "0.3.0", json!({"version": "2.0.0", "mode": "exact_straight_pressure_v2"}), Some(F::SchemaVersion)),
        ("0.3.0 with no contract (retired)", "0.3.0", Value::Null, Some(F::SchemaVersion)),
        ("0.2.0 with the legacy contract (retired)", "0.2.0", legacy.clone(), Some(F::SchemaVersion)),
        ("0.1.0 with the legacy contract (retired)", "0.1.0", legacy.clone(), Some(F::SchemaVersion)),
        ("0.4.0 with the legacy contract (retired)", "0.4.0", legacy.clone(), Some(F::SchemaVersion)),
        ("0.3 with the legacy contract", "0.3", legacy.clone(), Some(F::SchemaVersion)),
    ] {
        assert_eq!(contract(schema, value), fact.and_then(|fact| family(C::Namespace, fact)), "{label}");
    }
    // D1.3 first: the label with an empty region list (D1.5), a zero-magnitude element pressure
    // load (D1.7, formerly N-11's reading on L3) or a nodal force whose free-text category is
    // `pressure` refuses at D1.3.
    let mut regions = legacy3(milestone());
    regions["model"]["load_cases"][0]["pressure_regions"] = json!([]);
    assert_eq!(domain(regions), family(C::Namespace, F::PressureContract), "regions");
    let mut element = legacy3(milestone());
    element["model"]["load_cases"][0]["primitive_loads"][2] = json!({"id": "load:p", "category": "pressure",
        "target": {"type": "element", "pipe": "M1"}, "direction": "internal", "magnitude": {"value": 0.0, "unit": "Pa"}, "dimension": "pressure"});
    assert_eq!(domain(element), family(C::Namespace, F::PressureContract), "element pressure load");
    let mut nodal = legacy3(milestone());
    nodal["model"]["load_cases"][0]["primitive_loads"][2]["category"] = json!("pressure");
    assert_eq!(domain(nodal), family(C::Namespace, F::PressureContract), "nodal pressure category");
    // The census reads a contract's typed strings whatever its branch (D1.9's typed text rows;
    // B7b: the exact v3 contract's shear model is the third). Their capacity is priced on branch
    // E, the one branch that admits a contract.
    let base = nested_typed_census(&serde_json::from_value(milestone()).unwrap());
    let l3 = nested_typed_census(&serde_json::from_value(legacy3(milestone())).unwrap());
    assert_eq!(l3.strings, base.strings + 2, "version and mode");
    let mut e_free = exact3(milestone());
    e_free["model"].as_object_mut().unwrap().remove("pressure_contract");
    let e_free = nested_typed_census(&serde_json::from_value(e_free).unwrap());
    let e = nested_typed_census(&serde_json::from_value(exact3(milestone())).unwrap());
    assert_eq!(e.strings, e_free.strings + 3, "version, mode and shear model");
    for field in ["version", "mode", "shear_model"] {
        let report = admitted_typed(exact3(milestone()), |r| {
            let c = r.model.pressure_contract.as_mut().unwrap();
            let s = match field { "version" => c.version.as_mut(), "mode" => c.mode.as_mut(), _ => c.shear_model.as_mut() };
            s.unwrap().reserve_exact(200);
        });
        assert!(cap(CapFact::TypedTextCapacity)(report.law().domain), "{field}: {:?}", report.law().domain);
    }
}

/// B3a's drop on the Direct entry: `m3l`, and the label with another version, are refused at
/// G-A with `PressureContract` and publish the exact ordinary bytes, with no W1 and no
/// successor, in every build and both modes. B7b: the label is on 0.8.0; the historical 0.3.0
/// `m3l` is refused by its retired version (`SchemaVersion`) with the same exact ordinary bytes.
#[test]
fn b3a_dropped_direct_entry_refuses_m3l() {
    for mode in MODES {
        let mut outside = legacy3(milestone());
        outside["model"]["pressure_contract"]["version"] = json!("1.0.1");
        let mut retired = legacy3(milestone());
        retired["model"]["schema_version"] = json!("0.3.0");
        for (label, raw, fact) in [("version 1.0.1", outside, FamilyFact::PressureContract), ("m3l", legacy3(milestone()), FamilyFact::PressureContract),
            ("m3l at 0.3.0 (retired)", retired, FamilyFact::SchemaVersion)] {
            let plain = serde_json::to_vec(&crate::run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap();
            let direct = crate::run_linear_static_preview_value_with_retained_direct(raw, mode, RETAINED_LIMIT).unwrap();
            assert_eq!(direct.admission().unwrap().law().domain, family(D1Clause::Namespace, fact), "{label} {mode:?}");
            assert!(direct.admission().unwrap().law().refusal.is_some(), "{label} {mode:?}: refused");
            assert!(direct.retained().is_none() && direct.successor().is_none(), "{label} {mode:?}: no permit, no W1");
            assert_eq!(serde_json::to_vec(direct.envelope()).unwrap(), plain, "{label} {mode:?}: the exact ordinary bytes");
        }
    }
}

// ---- B2-A: D1.4 with combinations, C_eq and terms (B2-C §9; I93 PLAN §1.2.4) -------------

/// `raw` with `z` mechanics combinations `combination:k` (k = 1, …, z), each 1·(the first case),
/// with the provenance the ordinary validation requires.
fn with_combinations(mut raw: Value, z: usize) -> Value {
    let case = raw["model"]["load_cases"][0]["id"].clone();
    raw["model"]["combinations"] = Value::Array((1..=z)
        .map(|k| json!({"id": format!("combination:{k}"), "basis": "mechanics", "terms": [{"load_case": case.clone(), "factor": 1.0}],
            "provenance": "invented_i103_b2_a_combination"}))
        .collect());
    raw
}

/// D1.4 and D1.9 (B2-C §9): 1 ≤ c ≤ C, z ≤ 2 and C_eq = c + z ≤ 3. Over c = 1..C and z = 0..3,
/// a request is inside D1 exactly when z ≤ 2 and c + z ≤ 3; otherwise `Combinations` refuses
/// first (z = 3), then `CaseEquivalents`. Every basis counts toward z and C_eq.
#[test]
fn b2_a_d1_admits_case_equivalents_up_to_three() {
    assert_eq!((caps::COMBINATIONS, caps::CASE_EQUIVALENTS, caps::COMBINATION_TERMS, caps::RANGE_OPERANDS), (2, 3, 3, 3));
    let registered = COMPILED_IDENTITY == Some(REGISTERED_PROFILES[0].identity);
    for c in 1..=caps::LOAD_CASES {
        for z in 0..=caps::CASE_EQUIVALENTS {
            let expected = if z > caps::COMBINATIONS {
                Some(AdmissionRefusal::Cap { fact: CapFact::Combinations, observed: z, cap: caps::COMBINATIONS })
            } else if c + z > caps::CASE_EQUIVALENTS {
                Some(AdmissionRefusal::Cap { fact: CapFact::CaseEquivalents, observed: c + z, cap: caps::CASE_EQUIVALENTS })
            } else {
                None
            };
            for mode in MODES {
                let (request, capture) = CapturedInvocation::parse(with_combinations(milestone_cases(c), z), mode).unwrap();
                let admitted = admit(&capture, &request, Entry::Direct, RETAINED_LIMIT);
                let report = match &admitted {
                    Ok((_, report)) | Err(report) => *report,
                };
                assert_eq!((report.typed.combinations.length, report.typed.combinations.capacity), (z, z), "c = {c}, z = {z}");
                assert_eq!(report.law().domain, expected, "c = {c}, z = {z} {mode:?}");
                assert_eq!(report.law().refusal, d1_1_refusal().or(expected), "c = {c}, z = {z} {mode:?}");
                assert_eq!(admitted.is_ok(), registered && expected.is_none(), "c = {c}, z = {z} {mode:?}: a permit only inside D1");
            }
        }
    }
    // Subtraction and range combinations are case-equivalents too.
    let mut two = milestone_cases(2);
    two["model"]["combinations"] = json!([{"id": "range", "basis": "range_envelope", "operand_ids": ["case-1", "case-2"], "mode": "max"}]);
    assert_eq!(domain(two.clone()), None, "c = 2 with one range: C_eq = 3");
    two["model"]["combinations"].as_array_mut().unwrap().push(json!({"id": "difference", "basis": "result_state_subtraction",
        "minuend_id": "case-1", "subtrahend_id": "case-2"}));
    assert_eq!(domain(two), Some(AdmissionRefusal::Cap { fact: CapFact::CaseEquivalents, observed: 4, cap: 3 }), "and a subtraction: C_eq = 4");
    let mut one = milestone();
    one["model"]["combinations"] = json!([{"id": "difference", "basis": "result_state_subtraction", "minuend_id": "case", "subtrahend_id": "case"},
        {"id": "range", "basis": "range_envelope", "operand_ids": ["case"], "mode": "min_abs"},
        {"id": "twice", "basis": "mechanics", "terms": [{"load_case": "case", "factor": 2.0}]}]);
    assert_eq!(domain(one), Some(AdmissionRefusal::Cap { fact: CapFact::Combinations, observed: 3, cap: 2 }), "three combinations of three bases");
    // B1's D1.4 is unchanged: no load case, and C + 1 load cases, refuse at D1.4 whatever z is.
    assert_eq!(domain(with_combinations(milestone_cases(caps::LOAD_CASES + 1), 1)), family(D1Clause::Invocation, FamilyFact::LoadCases));
    let mut none = with_combinations(milestone(), 1);
    none["model"]["load_cases"] = json!([]);
    assert_eq!(domain(none), family(D1Clause::Invocation, FamilyFact::LoadCases));
}

/// D1.4 (C-9): no combination id equals a load-case id, whichever combination and case carry
/// it. It is a family fact, so it refuses before D1.9's rows.
#[test]
fn b2_a_combination_ids_are_disjoint_from_load_case_ids() {
    let refused = family(D1Clause::Invocation, FamilyFact::CombinationIds);
    let renamed = |c: usize, z: usize, combination: usize, id: &str| {
        let mut raw = with_combinations(milestone_cases(c), z);
        raw["model"]["combinations"][combination]["id"] = json!(id);
        domain(raw)
    };
    assert_eq!(renamed(1, 1, 0, "case-1"), refused, "the first case's id");
    assert_eq!(renamed(2, 1, 0, "case-2"), refused, "a later case's id");
    assert_eq!(renamed(1, 2, 1, "case-1"), refused, "the second combination");
    assert_eq!(renamed(1, 2, 1, "case-2"), None, "an id no case has");
    assert_eq!(renamed(1, 2, 1, "combination:1"), None, "combination ids repeated among themselves are not D1's (validation's)");
    assert_eq!(renamed(1, 3, 2, "case-1"), refused, "before D1.9's Combinations row");
    assert_eq!(AdmissionRefusal::Family(D1Clause::Invocation, FamilyFact::CombinationIds).precondition().as_str(), "source_family");
}

/// D1.9 (B2-C §9): each combination's terms (repeats counted) and operand ids, ≤ 3, and their
/// typed capacities, over every combination of any basis.
#[test]
fn b2_a_terms_and_range_operands_are_at_most_three() {
    let terms = |h: usize| Value::Array((0..h).map(|_| json!({"load_case": "case", "factor": 1.0})).collect());
    let ids = |k: usize| Value::Array((0..k).map(|_| json!("case")).collect());
    let with = |second: bool, field: &str, value: Value, basis: &str| {
        let mut raw = with_combinations(milestone(), 2);
        let z = &mut raw["model"]["combinations"][usize::from(second)];
        z["basis"] = json!(basis);
        z[field] = value;
        if basis == "range_envelope" {
            z["mode"] = json!("max");
        }
        domain(raw)
    };
    for second in [false, true] {
        assert_eq!(with(second, "terms", terms(3), "mechanics"), None, "three terms (repeats counted)");
        assert_eq!(with(second, "terms", terms(4), "mechanics"), Some(AdmissionRefusal::Cap { fact: CapFact::CombinationTerms, observed: 4, cap: 3 }), "second: {second}");
        assert_eq!(with(second, "operand_ids", ids(3), "range_envelope"), None, "three operands");
        assert_eq!(with(second, "operand_ids", ids(4), "range_envelope"), Some(AdmissionRefusal::Cap { fact: CapFact::RangeOperands, observed: 4, cap: 3 }), "second: {second}");
    }
    // Before validation, a basis that does not use an array still owns it, and it is bounded.
    assert_eq!(with(true, "terms", terms(4), "range_envelope"), Some(AdmissionRefusal::Cap { fact: CapFact::CombinationTerms, observed: 4, cap: 3 }));
    assert_eq!(with(true, "operand_ids", ids(4), "mechanics"), Some(AdmissionRefusal::Cap { fact: CapFact::RangeOperands, observed: 4, cap: 3 }));
    // The typed capacities, read from the actual owners (the second combination's).
    let report = admitted_typed(with_combinations(milestone(), 2), |r| r.model.combinations[1].terms.reserve_exact(3));
    assert!(matches!(report.law().domain, Some(AdmissionRefusal::Cap { fact: CapFact::CombinationTermsCapacity, observed, cap: 3 }) if observed >= 4),
        "{:?}", report.law().domain);
    let report = admitted_typed(with_combinations(milestone(), 2), |r| r.model.combinations[1].terms.reserve_exact(2));
    assert_eq!(report.law().domain, None, "capacity 3");
    let report = admitted_typed(with_combinations(milestone(), 2), |r| {
        let mut ids = Vec::with_capacity(4);
        ids.push(String::from("case"));
        r.model.combinations[1].operand_ids = Some(ids);
    });
    assert_eq!(report.law().domain, Some(AdmissionRefusal::Cap { fact: CapFact::RangeOperandsCapacity, observed: 4, cap: 3 }));
    let census = combination_census(&serde_json::from_value(with_combinations(milestone(), 2)).unwrap());
    assert_eq!(census, CombinationFacts { terms: CapacityFact { length: 1, capacity: 1 }, range_operands: CapacityFact { length: 0, capacity: 0 } });
}

/// B2-C REVISION_01 N-10: the typed census reads every combination string as a case's, so
/// D1.9's typed text rows bound them; each string's spare capacity refuses on its own.
#[test]
fn b2_a_typed_census_reads_every_combination_string() {
    let full = || {
        let mut raw = milestone();
        raw["model"]["combinations"] = json!([{"id": "combination:all", "label": "every string", "basis": "mechanics",
            "terms": [{"load_case": "case", "factor": 1.0}], "minuend_id": "case", "subtrahend_id": "case", "operand_ids": ["case"],
            "mode": "max", "provenance": "invented"}]);
        raw
    };
    assert_eq!(domain(full()), None);
    let base = nested_typed_census(&serde_json::from_value(milestone()).unwrap());
    let with = nested_typed_census(&serde_json::from_value(full()).unwrap());
    assert_eq!(with.strings, base.strings + 9, "id, label, basis, the term's case, minuend, subtrahend, the operand, mode, provenance");
    type Pick = fn(&mut crate::PreviewCombination) -> &mut String;
    let picks: [(&str, Pick); 9] = [
        ("id", |z| &mut z.id),
        ("label", |z| z.label.as_mut().unwrap()),
        ("basis", |z| &mut z.basis),
        ("term case", |z| &mut z.terms[0].load_case),
        ("minuend", |z| z.minuend_id.as_mut().unwrap()),
        ("subtrahend", |z| z.subtrahend_id.as_mut().unwrap()),
        ("operand", |z| &mut z.operand_ids.as_mut().unwrap()[0]),
        ("mode", |z| z.mode.as_mut().unwrap()),
        ("provenance", |z| z.provenance.as_mut().unwrap()),
    ];
    for (label, pick) in picks {
        let report = admitted_typed(full(), |r| pick(&mut r.model.combinations[0]).reserve_exact(200));
        assert!(cap(CapFact::TypedTextCapacity)(report.law().domain), "{label}: {:?}", report.law().domain);
    }
    let mut long = full();
    long["model"]["combinations"][0]["provenance"] = json!(text("p", 129));
    assert_eq!(domain(long), Some(AdmissionRefusal::Cap { fact: CapFact::TypedTextBytes, observed: 129, cap: 128 }), "the typed row first");
}

/// G-C (I93 PLAN §1.2.4): `EnvelopeResults ≤ C_eq·P_final` (with its capacity and text) and
/// `RetainedErrorTextBytes ≤ C_eq·(3m + 1)·Text(err)`, numerically B1's because C_eq's cap
/// equals C's; the contract-evidence facts stay per case. An actual c = 1, z = 2 run of the
/// solvable cap-maximal model publishes more rows than one case's P_final, within C_eq·P_final.
/// G-B and T-3 (e) are unchanged: the attempt fact counts the requested cases, not z.
#[test]
fn b2_a_g_c_bounds_are_per_case_equivalent() {
    let p = phase_caps();
    let (m, g, ceq, c) = (32u64, 32u64, caps::CASE_EQUIVALENTS as u64, caps::LOAD_CASES as u64);
    assert_eq!(caps::CASE_EQUIVALENTS, caps::LOAD_CASES, "C_eq's cap equals C's: G-C's values are B1's");
    assert_eq!((p.complete[0], p.complete[1], p.complete[2]), (ceq * P_FINAL, push_capacity(ceq * P_FINAL), 2 * ceq * P_FINAL * text_atoms::ROW));
    assert_eq!(p.complete[COMPLETE_FACTS - 2], ceq * (3 * m + 1) * text_atoms::ERR);
    assert_eq!(p.complete[9], c * (3 * m + 2 * g), "contract evidence per case");
    assert_eq!(p.late[4..6], [caps::LOADS as u64, caps::TOTAL_LOADS as u64], "G-B unchanged");
    for mode in MODES {
        let (request, capture) = CapturedInvocation::parse(with_combinations(solvable_cap_maximal_cases(1), 2), mode).unwrap();
        assert_eq!(assess(&capture, &request, Entry::Direct, RETAINED_LIMIT).law().domain, None, "{mode:?}: c = 1, z = 2 is inside D1");
        let mut observer = crate::retained_product::ProductCapture::prepared_probe();
        let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
        assert_eq!(ordinary.status.mechanics, "MECHANICS_SOLVED", "{mode:?}");
        let o = complete_observations(&CompleteFacts { ordinary: &ordinary, capture: &observer, requested_cases: 1 });
        let get = |fact| o.iter().find(|x| x.fact == fact).unwrap().observed;
        let (rows, capacity, text) = (get(PhaseFact::EnvelopeResults), get(PhaseFact::EnvelopeResultCapacity), get(PhaseFact::EnvelopeResultTextBytes));
        println!("I103_B2_A_ENVELOPE_RESULTS mode={} rows={rows} capacity={capacity} text={text} p_final={P_FINAL}", mode.as_str());
        assert!(rows > P_FINAL, "{mode:?}: one case and two combinations exceed one case's P_final ({rows} rows)");
        assert!(rows <= p.complete[0] && capacity <= p.complete[1] && text <= p.complete[2], "{mode:?}: within C_eq·P_final");
        assert_eq!(get(PhaseFact::OrdinarySolveNotAttempted), 0, "{mode:?}: T-3 (e) counts the one requested case");
        assert!(ordinary_solve_attempted(&observer, 1) && !ordinary_solve_attempted(&observer, 3), "{mode:?}: combinations are not requested cases");
    }
}

/// B2-A's admitted combinations on the Direct entry, after B2-P (lane P, I105) and B2's RS
/// reader: W1 runs on a combination-bearing invocation inside D1.4. In the registered build, in
/// both modes, precommit validates the successor, so W1's result is that successor and the
/// ordinary owner is untouched:
/// - c = 1, z = 2 (two mechanics combinations of the one case, each retained by T-10a, so each
///   has a combination attempt);
/// - c = 2, z = 1 (a range combination, `ordinary`).
///
/// Stale (unregistered) builds refuse at D1.1 and keep the plain bytes.
#[test]
fn b2_a_admitted_combinations_run_w1_and_publish_the_validated_successor() {
    let registered = COMPILED_IDENTITY == Some(REGISTERED_PROFILES[0].identity);
    let mut mixed = milestone_cases(2);
    mixed["model"]["combinations"] = json!([{"id": "range", "basis": "range_envelope", "operand_ids": ["case-1", "case-2"], "mode": "max_abs",
        "provenance": "invented_i103_b2_a_range"}]);
    for (label, raw) in [("c = 1, z = 2", with_combinations(milestone(), 2)), ("c = 2, z = 1 (range)", mixed)] {
        for mode in MODES {
            let plain = serde_json::to_vec(&crate::run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap();
            let direct = crate::run_linear_static_preview_value_with_retained_direct(raw.clone(), mode, RETAINED_LIMIT).unwrap();
            let report = direct.admission().unwrap();
            assert_eq!(report.law().domain, None, "{label} {mode:?}: inside D1");
            let published = serde_json::to_vec(direct.envelope()).unwrap();
            if !registered {
                assert_eq!(report.law().refusal, d1_1_refusal(), "{label} {mode:?}");
                assert!(direct.successor().is_none(), "{label} {mode:?}");
                assert_eq!(published, plain, "{label} {mode:?}: the exact ordinary bytes");
                continue;
            }
            assert_eq!(report.law().refusal, None, "{label} {mode:?}: admitted");
            match direct.retained() {
                Some(Ok(_)) => {
                    assert!(direct.successor().is_some(), "{label} {mode:?}: the validated successor (B2's RS reader)");
                    assert_eq!(published, plain, "{label} {mode:?}: the ordinary owner is untouched");
                }
                other => panic!("{label} {mode:?}: B2's RS reader validates the successor: {other:?}"),
            }
        }
    }
}

// ---- B3b-A: D1.3, D1.4 and D1.5 for the exact route (B3-D §4.2; provisional on B1's M) ---

/// `raw` authored as exact: I99's `exact` (B3-W's `m3x` when `raw` is the milestone): the exact
/// contract, the common E/ν basis with E = 2e11 Pa and ν = 0.25 and no shear modulus, and
/// explicitly empty pressure regions on every case. B7b re-authored it once, as
/// `tests/common/b1_sq_inputs.rs`'s `exact3`: model 0.8.0 with `3.0.0/exact_pressure_v3`,
/// shear model Euler-Bernoulli and an explicit zero corrosion allowance on every pipe section.
fn exact3(mut raw: Value) -> Value {
    let m = &mut raw["model"];
    m["schema_version"] = json!(crate::MODEL_SCHEMA_VERSION);
    m["pressure_contract"] = exact_contract();
    for material in m["materials"].as_array_mut().unwrap() {
        material.as_object_mut().unwrap().remove("shear_modulus");
        material["constitutive_basis"] = json!("homogeneous_isotropic_E_nu_v1");
        material["poisson_ratio"] = json!({"value": 0.25, "unit": "1"});
    }
    for pipe in m["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"]["corrosion_allowance"] = json!({"value": 0.0, "unit": "m"});
    }
    for case in m["load_cases"].as_array_mut().unwrap() {
        case["pressure_regions"] = json!([]);
    }
    raw
}
/// The committed physics-source requests n05 and n06 (B3b's coexistence pins).
// B7b PP compile fix only: the v2 originals were deleted; their current twins.
const N05: &str = include_str!("../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json");
const N06: &str = include_str!("../../../fixtures/product_preview/pressure_source/physics_source/n06.request.json");

/// B3-D §4.2's law tests: exact (B7b: 0.8.0 with the v3 contract) with `[]` on every case is
/// inside D1 (alone, with C cases, and as B3-W's mixed base `m3x_mix_anchor`), with a permit in
/// the registered build; refused: regions absent or with one region (`PressureRegions`), on any
/// case; one combination (`Combinations`); a retired version (`SchemaVersion`); any other
/// contract (`PressureContract`); a point basis (`ModulusBasisRef`).
#[test]
fn b3b_d1_admits_the_exact_route_with_empty_regions() {
    use D1Clause as C;
    use FamilyFact as F;
    let registered = COMPILED_IDENTITY == Some(REGISTERED_PROFILES[0].identity);
    let mut mix = milestone();
    mix["model"]["load_cases"].as_array_mut().unwrap().push(json!({"id": "case:b", "label": "I99 B3-W second case (anchor)",
        "kind": "primitive_user_load", "primitive_loads": [{"id": "load:b:0", "category": "concentrated_force", "target": {"type": "node", "node": "N0"},
        "direction": "global_x", "magnitude": {"value": 1.0, "unit": "N"}, "dimension": "force"}]}));
    for (label, raw) in [("m3x", exact3(milestone())), ("C cases", exact3(milestone_cases(caps::LOAD_CASES))), ("m3x_mix_anchor", exact3(mix))] {
        for mode in MODES {
            let (request, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
            assert_eq!(namespace_branch(&request.model), Ok(NamespaceBranch::Exact), "{label}");
            let admitted = admit(&capture, &request, Entry::Direct, RETAINED_LIMIT);
            let report = match &admitted {
                Ok((_, report)) | Err(report) => *report,
            };
            assert_eq!(report.law().domain, None, "{label} {mode:?}: branch E is inside D1");
            assert_eq!(admitted.is_ok(), registered, "{label} {mode:?}");
        }
    }
    let with = |change: &dyn Fn(&mut Value)| {
        let mut raw = exact3(milestone_cases(caps::LOAD_CASES));
        change(&mut raw);
        domain(raw)
    };
    let last = caps::LOAD_CASES - 1;
    for case in [0, last] {
        assert_eq!(with(&|r| { r["model"]["load_cases"][case].as_object_mut().unwrap().remove("pressure_regions"); }),
            family(C::Case, F::PressureRegions), "case {case}: regions absent");
        assert_eq!(with(&|r| r["model"]["load_cases"][case]["pressure_regions"] = json!([{"id": "region", "member_pipe_ids": ["M1"],
            "pressure_basis": "gauge", "pressure": {"value": 0.0, "unit": "Pa"}}])), family(C::Case, F::PressureRegions), "case {case}: one region");
        assert_eq!(with(&|r| r["model"]["load_cases"][case]["pressure_regions"] = Value::Null), family(C::Case, F::PressureRegions), "case {case}: null");
        assert_eq!(with(&|r| r["model"]["load_cases"][case]["modulus_basis_ref"] = json!("T0")), family(C::Case, F::ModulusBasisRef), "case {case}: a point basis");
        // B7a (b) Widen domain, stage 1, is branch L only: a member load stays outside branch E.
        assert_eq!(with(&|r| r["model"]["load_cases"][case]["primitive_loads"][0] = json!({"id": "udl", "category": "distributed_force",
            "target": {"type": "element", "pipe": "M1"}, "direction": "global_y", "magnitude": {"value": 10.0, "unit": "N/m"},
            "dimension": "force_per_length"})), family(C::Loads, F::LoadTarget), "case {case}: a member load");
    }
    assert_eq!(with(&|r| r["model"]["combinations"] = json!([{"id": "combination", "basis": "mechanics", "terms": [{"load_case": "case-1", "factor": 1.0}]}])),
        family(C::Invocation, F::Combinations), "ruling 4: no combination on the exact route");
    assert_eq!(with(&|r| r["model"]["combinations"] = json!([{"id": "case-1", "basis": "mechanics", "terms": [{"load_case": "case-1", "factor": 1.0}]}])),
        family(C::Invocation, F::Combinations), "the exact clause first");
    assert_eq!(with(&|r| r["model"]["schema_version"] = json!("0.4.0")), family(C::Namespace, F::SchemaVersion), "0.4.0 stays out");
    assert_eq!(with(&|r| r["model"]["schema_version"] = json!("0.3.0")), family(C::Namespace, F::SchemaVersion), "0.3.0 is retired with v2");
    assert_eq!(with(&|r| { r["model"].as_object_mut().unwrap().remove("pressure_contract"); }), family(C::Case, F::PressureRegions),
        "without the contract it is branch L, where a region list refuses");
    assert_eq!(with(&|r| r["model"]["pressure_contract"]["version"] = json!("3.0.1")), family(C::Namespace, F::PressureContract));
    assert_eq!(with(&|r| r["model"]["pressure_contract"]["mode"] = json!("legacy_pressure_v1")), family(C::Namespace, F::PressureContract),
        "3.0.0 with the legacy mode");
    assert_eq!(with(&|r| r["model"]["pressure_contract"]["shear_model"] = json!("timoshenko")), family(C::Namespace, F::PressureContract),
        "another shear model");
    assert_eq!(with(&|r| { r["model"]["pressure_contract"].as_object_mut().unwrap().remove("shear_model"); }), family(C::Namespace, F::PressureContract),
        "no shear model");
    assert_eq!(with(&|r| r["model"]["pressure_contract"] = json!({"version": "2.0.0", "mode": "exact_straight_pressure_v2"})),
        family(C::Namespace, F::PressureContract), "the retired v2 contract on 0.8.0");
    assert_eq!(with(&|r| r["model"]["schema_version"] = json!("0.2.0")), family(C::Namespace, F::SchemaVersion), "0.2.0 is retired");
    // A combination on L is still inside D1 (B2-C's clauses apply there only; B3a's L3 is dropped).
    let mut l = milestone();
    l["model"]["combinations"] = json!([{"id": "combination", "basis": "mechanics", "terms": [{"load_case": "case", "factor": 1.0}]}]);
    assert_eq!(domain(l), None);
    // The committed physics-source requests n05 and n06 (B3b's coexistence pins) are on branch E.
    for (name, text) in [("n05", N05), ("n06", N06)] {
        assert_eq!(domain(serde_json::from_str(text).unwrap()), None, "{name}");
    }
}

/// B7a (b), stage 1, is branch L only (review F3a). On the exact route, on the first and the
/// last of C cases, a full-span global-axis member load (inside D1.7 on L) refuses by D1.7's
/// target fact, and a named-point basis (inside D1.5 on L) by D1.5's. D1 alone, so this holds
/// in any build, registered or not.
#[test]
fn b7a_d1_exact_route_keeps_member_loads_and_selectors_out() {
    use D1Clause as C;
    use FamilyFact as F;
    let with = |case: usize, change: &dyn Fn(&mut Value)| {
        let mut raw = exact3(milestone_cases(caps::LOAD_CASES));
        change(&mut raw["model"]["load_cases"][case]);
        domain(raw)
    };
    for case in [0, caps::LOAD_CASES - 1] {
        assert_eq!(with(case, &|_| {}), None, "case {case}: the exact request is inside D1");
        for direction in ["global_x", "global_y", "global_z"] {
            let udl = json!({"id": "udl", "category": "distributed_force", "target": {"type": "element", "pipe": "M1"},
                "direction": direction, "magnitude": {"value": 10.0, "unit": "N/m"}, "dimension": "force_per_length"});
            assert_eq!(with(case, &|c| c["primitive_loads"][0] = udl.clone()), family(C::Loads, F::LoadTarget),
                "case {case}: a {direction} member load");
        }
        assert_eq!(with(case, &|c| c["modulus_basis_ref"] = json!("T0")), family(C::Case, F::ModulusBasisRef),
            "case {case}: a named-point basis");
    }
}

/// W-16 slice a (DEL-04-03 Design `pf-imposed-displacement.md` §3): the census reads a
/// support target's two strings and D1.7 refuses it by its target fact on both branches, as
/// the first or last primitive of the first or last of C cases. D1 alone, so this holds in
/// any build, registered or not.
#[test]
fn w16_d1_refuses_support_targeted_imposed_displacement_on_both_branches() {
    use D1Clause as C;
    use FamilyFact as F;
    let motion = json!({"id": "motion", "category": "imposed_displacement", "target": {"type": "support", "support": "rigid:N0", "dof": "UY"},
        "direction": "UY", "magnitude": {"value": 0.001, "unit": "m"}, "dimension": "displacement"});
    for (branch, base) in [("L", milestone_cases(caps::LOAD_CASES)), ("E", exact3(milestone_cases(caps::LOAD_CASES)))] {
        assert_eq!(domain(base.clone()), None, "branch {branch} inside D1");
        for case in [0, caps::LOAD_CASES - 1] {
            for first in [true, false] {
                let mut raw = base.clone();
                let loads = raw["model"]["load_cases"][case]["primitive_loads"].as_array_mut().unwrap();
                if first { loads.insert(0, motion.clone()) } else { loads.push(motion.clone()) }
                let report = admitted(raw);
                assert!(report.census_complete(), "branch {branch} case {case}");
                assert_eq!(report.law().domain, family(C::Loads, F::LoadTarget), "branch {branch} case {case} first {first}");
            }
        }
    }
}

/// B3b-A's Direct-entry oracle, after B3b-P (lane P, I105; ROOT's one-off edit on b2, refined
/// by RV123's interim review). `permitted_run` sends a permitted exact model to W1's exact route
/// (B3-D P-1, P-2). In the registered build, in both modes:
/// - the coexistence pins n05 and n06 fall back with `Coexistence` (T-3 (c), under the route's
///   8,000,000 budget) and publish exactly the ordinary value route's bytes;
/// - m3x runs W1, and RS's precommit (B3's reader) validates its `physics-retained-1`
///   successor, so W1's result is that successor and the ordinary owner is untouched.
///
/// Stale (unregistered) builds refuse at D1.1 and keep the plain bytes.
#[test]
fn b3b_direct_entry_coexistence_keeps_the_exact_bytes_and_m3x_publishes_the_successor() {
    let registered = COMPILED_IDENTITY == Some(REGISTERED_PROFILES[0].identity);
    let read = |text: &str| -> Value { serde_json::from_str(text).unwrap() };
    for (label, raw) in [("m3x", exact3(milestone())), ("n05", read(N05)), ("n06", read(N06))] {
        for mode in MODES {
            let plain = serde_json::to_vec(&crate::run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap()).unwrap();
            let direct = crate::run_linear_static_preview_value_with_retained_direct(raw.clone(), mode, RETAINED_LIMIT).unwrap();
            let report = direct.admission().unwrap();
            assert_eq!(report.law().domain, None, "{label} {mode:?}");
            let published = serde_json::to_vec(direct.envelope()).unwrap();
            if !registered {
                assert_eq!(report.law().refusal, d1_1_refusal(), "{label} {mode:?}");
                assert!(direct.successor().is_none(), "{label} {mode:?}");
                assert_eq!(published, plain, "{label} {mode:?}: the exact ordinary bytes");
                continue;
            }
            assert_eq!(report.law().refusal, None, "{label} {mode:?}: admitted");
            if label != "m3x" {
                assert!(matches!(direct.retained(), Some(Err(crate::W1Fallback::Coexistence))), "{label} {mode:?}: {:?}", direct.retained());
                assert!(direct.successor().is_none(), "{label} {mode:?}");
                assert_eq!(published, plain, "{label} {mode:?}: the exact ordinary bytes");
                continue;
            }
            match direct.retained() {
                Some(Ok(_)) => {
                    assert!(direct.successor().is_some(), "{label} {mode:?}: the validated successor (B3's readers)");
                    assert_eq!(published, plain, "{label} {mode:?}: the ordinary owner is untouched");
                }
                other => panic!("{label} {mode:?}: {other:?}"),
            }
        }
    }
}

/// RV122 SF-2 (its probe B as the witness): on branch E the explicitly empty `pressure_regions`
/// list is a typed owner, and its capacity must be 0, on any case. With spare capacity it
/// refuses with `PressureRegionsCapacity` (resource admission), as the control, an empty
/// `sections` owner with spare capacity, refuses with `SectionsCapacity` at D1.9. Length 0 with
/// capacity 0, which every parsed request has, is admitted.
#[test]
fn b3b_exact_regions_capacity_is_read() {
    let spare = |case: usize, capacity: usize| {
        admitted_typed(exact3(milestone_cases(caps::LOAD_CASES)), |r| r.model.load_cases[case].pressure_regions = Some(Vec::with_capacity(capacity)))
            .law()
            .domain
    };
    for case in [0, caps::LOAD_CASES - 1] {
        assert_eq!(spare(case, 65_536), Some(AdmissionRefusal::Cap { fact: CapFact::PressureRegionsCapacity, observed: 65_536, cap: 0 }), "case {case}");
        assert_eq!(spare(case, 1), Some(AdmissionRefusal::Cap { fact: CapFact::PressureRegionsCapacity, observed: 1, cap: 0 }), "case {case}");
        assert_eq!(spare(case, 0), None, "case {case}: length 0, capacity 0");
    }
    let parsed: LinearStaticPreviewRequest = serde_json::from_value(exact3(milestone_cases(caps::LOAD_CASES))).unwrap();
    assert!(parsed.model.load_cases.iter().all(|case| case.pressure_regions.as_ref().is_some_and(|r| r.is_empty() && r.capacity() == 0)),
        "a parsed request's empty list has capacity 0");
    assert_eq!(domain(exact3(milestone())), None);
    let refusal = AdmissionRefusal::Cap { fact: CapFact::PressureRegionsCapacity, observed: 1, cap: 0 };
    assert_eq!(refusal.precondition().as_str(), "resource_admission");
    let control = admitted_typed(exact3(milestone()), |r| r.model.sections.reserve_exact(1));
    assert!(matches!(control.law().domain, Some(AdmissionRefusal::Cap { fact: CapFact::SectionsCapacity, cap: 0, .. })), "{:?}", control.law().domain);
}

/// B1 SQ (RV112 N-2): at C = 3 there are two parked slots, so "every parked slot" differs from
/// "the first": text in the last parked slot alone is counted (RV112's mutant G04, which folds
/// `parked[..1]`, reads 0), and then every slot's text with the capture's own.
#[test]
fn b1_sq_retained_error_text_reads_every_parked_slot_at_c() {
    use crate::retained_product::{CaptureError, ProductCapture};
    let fact = |o: &[PhaseObservation]| o.iter().find(|x| x.fact == PhaseFact::RetainedErrorTextBytes).unwrap().observed;
    for mode in MODES {
        let (request, capture) = CapturedInvocation::parse(milestone_cases(caps::LOAD_CASES), mode).unwrap();
        let mut observer = ProductCapture::prepared_probe();
        let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
        assert_eq!((observer.cases_seen(), observer.parked_cases().len()), (3, 2), "{mode:?}: cases 0 and 1 are parked, case 2 is in the capture's fields");
        let facts = |observer: &ProductCapture| complete_observations(&CompleteFacts { ordinary: &ordinary, capture: observer, requested_cases: 3 });
        assert_eq!(fact(&facts(&observer)), 0, "{mode:?}: no error text");
        observer.with_case(1, |case| case.observable_error = Some(CaptureError::Association(String::with_capacity(101))));
        assert_eq!(fact(&facts(&observer)), 101, "{mode:?}: the last parked slot alone");
        observer.with_case(0, |case| {
            case.error = Some(CaptureError::Association(String::with_capacity(40)));
            case.observable_error = Some(CaptureError::Association(String::with_capacity(9)));
        });
        observer.with_case(1, |case| case.error = Some(CaptureError::Association(String::with_capacity(5))));
        observer.error = Some(CaptureError::Association(String::with_capacity(7)));
        observer.observable_error = Some(CaptureError::Association(String::with_capacity(3)));
        assert_eq!(fact(&facts(&observer)), 40 + 9 + 101 + 5 + 7 + 3, "{mode:?}: every slot and the capture's own");
    }
}

/// B1 SQ (RV112 SF-1): G-B's byte bound subtracts one case's late capture, `F_T11_LATE_CAPTURE / C`.
/// That is exact because the generator prices the late capture per requested case: every
/// coefficient of the form (and its constant) is a multiple of C, so the form is C times a form
/// with integer coefficients (G5's chain checks it equals the c = 1 late form), and its in-build
/// value is C times that form's value. At C = 1 the bound is T11 − T11_late_capture as before.
#[test]
fn b1_sq_late_capture_form_is_c_times_one_case() {
    use profile::{ATOM_VALUES, FORMS, F_T11, F_T11_LATE_CAPTURE};
    let c = caps::LOAD_CASES as u64;
    let late = &FORMS[F_T11_LATE_CAPTURE];
    assert_eq!(late.name, "T11_late_capture");
    assert_eq!(late.constant % c, 0, "the constant is C times one case's");
    for (atom, coefficient) in late.terms {
        assert_eq!(coefficient % c, 0, "atom {}: the coefficient is C times one case's", profile::ATOM_NAMES[*atom]);
    }
    let one_case = late.constant / c + late.terms.iter().map(|(atom, coefficient)| ATOM_VALUES[*atom] * (coefficient / c)).sum::<u64>();
    assert_eq!(profile_bytes(F_T11_LATE_CAPTURE), c * one_case, "the in-build value is C times one case's");
    assert_eq!(phase_caps().late[LATE_FACTS - 1], profile_bytes(F_T11) - one_case, "G-B: T11 less one case's late capture");
    // The late form is part of T11: each of its atoms appears in T11 with at least its coefficient.
    for (atom, coefficient) in late.terms {
        let in_t11 = FORMS[F_T11].terms.iter().find(|(a, _)| a == atom).map_or(0, |(_, x)| *x);
        assert!(in_t11 >= *coefficient, "atom {}: T11 holds the late capture", profile::ATOM_NAMES[*atom]);
    }
    println!("I104_SQ_G_B_BOUND T11={} late_capture={} one_case={} g_b_bound={}", profile_bytes(F_T11), profile_bytes(F_T11_LATE_CAPTURE), one_case,
        phase_caps().late[LATE_FACTS - 1]);
}

// ---- The caller's limit L (memory-budget-decision.md) -------------------------

/// L above the registered profile's qualified M refuses `Unqualified`, before pricing and
/// never clamped; the actual admission then mints no permit in any build.
#[test]
fn a_limit_above_the_qualified_m_is_refused_never_clamped() {
    let r = RESERVED_STACK_BYTES as u64;
    assert_eq!(RetainedByteLimit::new(M + 1).bytes(), M + 1);
    for limit in [M + 1, 2 * M, u64::MAX] {
        let unqualified = Err(BoundRefusal::Unqualified { limit, qualified: M });
        assert_eq!(limited_bound(Ok(M - r), RetainedByteLimit::new(limit), M), unqualified, "even a request that fits M");
        assert_eq!(limited_bound(Err(BoundRefusal::Unpriced), RetainedByteLimit::new(limit), M), unqualified);
        assert_eq!(bound_required(&unqualified), None, "nothing computed above M");
        assert_eq!(AdmissionRefusal::Bound(BoundRefusal::Unqualified { limit, qualified: M }).precondition().as_str(), "resource_admission");
        let law = law_order(RetainedCaller::Direct, Ok(0), Ok(()), |index| {
            limited_bound(cap_priced_maximum(MODES[0]), RetainedByteLimit::new(limit), REGISTERED_PROFILES[index].threshold_bytes)
        });
        assert_eq!(law, Err(AdmissionRefusal::Bound(BoundRefusal::Unqualified { limit, qualified: M })));
    }
    for mode in MODES {
        let (request, capture) = CapturedInvocation::parse(milestone(), mode).unwrap();
        let report = admit(&capture, &request, Entry::Direct, RetainedByteLimit::new(M + 1)).err().expect("no permit above M");
        let expected = d1_1_refusal().or(Some(AdmissionRefusal::Bound(BoundRefusal::Unqualified { limit: M + 1, qualified: M })));
        assert_eq!(report.law().refusal, expected, "{mode:?}");
        assert_eq!((report.law().registered, report.law().required), (None, None), "{mode:?}");
    }
}

/// S-1 at L: a limit one byte below `E_mov,max(mode) + R` refuses `Exceeds` with threshold L;
/// a limit exactly equal to it admits.
#[test]
fn a_limit_is_the_threshold_and_admits_exactly_at_required() {
    let r = RESERVED_STACK_BYTES as u64;
    for mode in MODES {
        let required = cap_priced_maximum(mode).unwrap() + r;
        let below = RetainedByteLimit::new(required - 1);
        assert_eq!(limited_bound(cap_priced_maximum(mode), below, M), Err(BoundRefusal::Exceeds { required, threshold: required - 1 }), "{mode:?}");
        assert_eq!(limited_bound(cap_priced_maximum(mode), RetainedByteLimit::new(required), M), Ok(required), "{mode:?}: at L");
        assert_eq!(limited_bound(cap_priced_maximum(mode), RETAINED_LIMIT, M), Ok(required), "{mode:?}: at M");
        assert_eq!(limited_bound(cap_priced_maximum(mode), RetainedByteLimit::new(0), M), Err(BoundRefusal::Exceeds { required, threshold: 0 }));
        let (request, capture) = CapturedInvocation::parse(milestone(), mode).unwrap();
        let low = admit(&capture, &request, Entry::Direct, below).err().expect("no permit below required");
        let exact = admit(&capture, &request, Entry::Direct, RetainedByteLimit::new(required));
        match d1_1_refusal() {
            Some(refusal) => {
                assert_eq!(low.law().refusal, Some(refusal), "{mode:?}: D1.1 first");
                assert_eq!(exact.err().expect("an unregistered build is refused").law().refusal, Some(refusal));
            }
            None => {
                assert_eq!(low.law().refusal, Some(AdmissionRefusal::Bound(BoundRefusal::Exceeds { required, threshold: required - 1 })), "{mode:?}");
                assert_eq!(low.law().required, Some(required));
                let (_permit, report) = exact.unwrap_or_else(|r| panic!("{mode:?}: refused at L = required: {:?}", r.law().refusal));
                assert_eq!(report.law().required, Some(required));
            }
        }
    }
}

/// The public facts: the matched profile's identity and qualified M in the registered build,
/// its D-6 status otherwise; the required bytes are the mode's maximum plus R or an error.
#[test]
fn retained_profile_states_the_qualified_facts() {
    let r = RESERVED_STACK_BYTES as u64;
    match (build_status(), retained_profile()) {
        (Ok(index), Ok(facts)) => {
            assert_eq!(facts.identity(), REGISTERED_PROFILES[index].identity);
            assert_eq!(facts.qualified_limit(), RETAINED_LIMIT);
            for mode in MODES {
                assert_eq!(facts.required_bytes(mode), Ok(cap_priced_maximum(mode).unwrap() + r), "{mode:?}");
            }
        }
        (Err(status), Err(facts_status)) => assert_eq!(facts_status, status),
        (status, facts) => panic!("retained_profile disagrees with build_status: {status:?} {facts:?}"),
    }
    for mode in MODES {
        let maximum = cap_priced_maximum(mode);
        assert_eq!(qualified_required(maximum), Ok(maximum.unwrap() + r), "{mode:?}");
    }
    assert_eq!(qualified_required(Err(BoundRefusal::Unpriced)), Err(RetainedPricingError::Unpriced));
    assert_eq!(qualified_required(Err(BoundRefusal::Overflow)), Err(RetainedPricingError::Overflow));
    assert_eq!(qualified_required(Ok(u64::MAX - r + 1)), Err(RetainedPricingError::Overflow));
    assert_eq!(qualified_required(Ok(u64::MAX - r)), Ok(u64::MAX));
}
