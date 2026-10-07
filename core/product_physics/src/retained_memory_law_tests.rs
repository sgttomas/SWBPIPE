//! U4 G5: the admission law's tests, through pure functions and the actual G-A
//! census. The tests construct no profile (decision 7): the one registered entry
//! is the production one, a permit comes only from `admit` in the registered build,
//! and a check that needs another matched build passes its inputs to the pure function.
use super::*;
use crate::source_receipt::CapturedInvocation;
use crate::PreviewSolverMode;
use serde_json::json;

const MILESTONE: &str = include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const M: u64 = 4_026_531_840;

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
    assess(&capture, &request, Entry::Direct)
}
/// G-A over a typed request changed after the parse (capacity rows).
fn admitted_typed(raw: Value, change: impl FnOnce(&mut LinearStaticPreviewRequest)) -> RetainedAdmissionReport {
    let (mut request, capture) = CapturedInvocation::parse(raw, PreviewSolverMode::SparseInteractive).unwrap();
    change(&mut request);
    assess(&capture, &request, Entry::Direct)
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

/// A cap-maximal D1 request (STACK_INVENTORY.md §3 W2's counts at `l ≤ 128`):
/// 32 nodes, a 32-member ring, 32 supports with 6 restraints each (r = 192) and a
/// scalar spring on each (s = 32), 128 nodal loads, 4 + 4 materials with 16
/// temperature points each, and one 128-byte identifier.
pub(super) fn cap_maximal() -> Value {
    let p = "invented_t3_g5_cap_maximal_input_no_library_data";
    let nodes: Vec<Value> = (0..32)
        .map(|i| {
            let t = 2.0 * std::f64::consts::PI * i as f64 / 32.0;
            json!({"id": format!("N{i}"), "position": {"x": 10.0 * t.cos(), "y": 10.0 * t.sin(), "z": 0.0}, "provenance": p})
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
            "schema_version": "0.1.0",
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
    let mut moved = layouts;
    moved[2].size += 8;
    assert!(!bindings_hold(true, Some("v1;x"), "v1;x", &layouts, &moved), "a changed reader layout");
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

#[test]
fn reviewed_inputs_bind_the_lock_and_the_reader_statics() {
    use sha2::{Digest, Sha256};
    // G4 NOTES §3 and ORIGINS.json `statics`: the reviewed hashes at NUM b1f80234dc.
    const REVIEWED: [&str; 14] = [
        "4f494db6d8a6eca87e7a16d8561197f20b1951a033bd3a6c424acfff5613475b",
        "3bb969555d5616af6eefdb68788ee4a74a8a3681c42fe9aae577a5d25a51ac5c",
        "07951edacfedd410c153929ee75bb5bada15dbd222369ec63240c678b233b61c",
        "3e0779a45a74cf0bb3a4ed08ed3a6b44347aea8a3c33b59e9dd92130426ee296",
        "c74742ce6a936384e00986006e6a0b2e6bb11f190451e876eed9ffa11903c6a8",
        "4d6886d19e304db897e5e9f8f0054cbee91ba7795868f9698e2bbe070bde94da",
        "d75aacee175e178dbdeb256d89a65f4b375265f7da077725ee635af33df51d7e",
        "9a2cf6268b57bd5265a1a115497c07450819dd4d03cd5ab618097bd9d19da8cc",
        "44bc41c06f589fab6ce931ac0eaa5344765ff64fd5f880cc2dd69ecb839c4f4d",
        "d1628194a7730f427843b00228dd233cf92b8e7d26f3bc31c660a3ea59e28337",
        "ae55503d44a4750714a35c423623e38cf4132099134097193024d1635bfbc88a",
        "ba13f2aefd7a38bd725e5f111e6ec30144bc8776aa957c6278ee7b1178298ba1",
        "5f299065f15a157bbedf9467a598994ae684c4ecb3f851bbcb291981ec550a9f",
        "544e196d2f7bef27276acc160aa19ab738a4f7949e846d2e8871328d2208129c",
    ];
    // The same files, read here at compile time (test-only), by the sha2 dependency.
    let files: [&[u8]; 14] = [
        include_bytes!("../Cargo.lock"),
        include_bytes!("../../../schemas/physics_source_recovery.schema.json"),
        include_bytes!("../../../schemas/retained_precision_mp_v2.schema.json"),
        include_bytes!("../../../fixtures/results/retained_precision_prepared_ordinary_v1.json"),
        include_bytes!("../../../fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json"),
        include_bytes!("../../../fixtures/results/semantic_contract_v0_2.json"),
        include_bytes!("../../../fixtures/results/semantic_contract_v0_3_precision_1.json"),
        include_bytes!("../../../fixtures/results/semantic_contract_v0_3_physics_1.json"),
        include_bytes!("../../../fixtures/results/semantic_contract_v0_3_load_reference_1.json"),
        include_bytes!("../../../fixtures/results/semantic_contract_v0_3_load_reference_source_1.json"),
        include_bytes!("../../../fixtures/results/semantic_contract_v0_3_preview_physics_1.json"),
        include_bytes!("../../../fixtures/results/semantic_contract_v0_3_physics_source_1.json"),
        include_bytes!("../../../fixtures/results/semantic_contract_v0_3_source_blocks_1.json"),
        include_bytes!("../../../schemas/source_block_recovery.schema.json"),
    ];
    let compiled = COMPILED_REVIEWED_INPUTS.expect("build.rs sets the reviewed inputs");
    assert_eq!(option_env!("OPS_RETAINED_REVIEWED_INPUTS"), Some(compiled));
    let mut tokens = compiled.split(';');
    assert_eq!(tokens.next(), Some("v1"));
    for (i, token) in tokens.by_ref().take(14).enumerate() {
        let (path, hex) = token.split_once('=').unwrap();
        assert_eq!(path, build_identity::REVIEWED_INPUTS[i]);
        assert_eq!(hex, format!("{:x}", Sha256::digest(files[i])), "{path}: build.rs's digest");
        assert_eq!(hex, REVIEWED[i], "{path}: G4's reviewed record");
    }
    assert_eq!(tokens.next(), None);
    println!("I65_G5_REVIEWED_INPUTS {compiled}");
    // Each reviewed static is the reader's own input: result_export names it.
    let reader = [
        include_str!("../../reporting/result_export/src/retained_precision.rs"),
        include_str!("../../reporting/result_export/src/physics_source.rs"),
        include_str!("../../reporting/result_export/src/semantic_contract.rs"),
        include_str!("../../reporting/result_export/src/source_blocks.rs"),
    ]
    .concat();
    for path in &build_identity::REVIEWED_INPUTS[1..] {
        let name = path.rsplit('/').next().unwrap();
        assert!(reader.contains(&format!("{name}\"")), "{name} is an include_str! input of the reader");
    }
}

#[test]
fn the_registered_profile_is_the_only_permit_source() {
    // G6: exactly one production profile, registered by reviewed change (decision 7: no test permit).
    assert_eq!(REGISTERED_PROFILES.len(), 1);
    assert_eq!(REGISTERED_PROFILES[0].identity, super::law_tests::PINNED_RECORD_IDENTITY, "the qualified build is the pinned record's");
    assert_eq!(REGISTERED_PROFILES[0].threshold_bytes, 4_026_531_840);
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
    // This build is the registered one, or Stale: never Missing now.
    match COMPILED_IDENTITY {
        Some(id) if id == REGISTERED_PROFILES[0].identity => assert_eq!(build_status(), Ok(0)),
        _ => assert_eq!(build_status(), Err(ProfileStatus::Stale)),
    }
}

/// G6 (brief step 5): under the registered build, `admit` grants a permit for the in-domain
/// milestone Direct invocation in both modes; Headless and out-of-domain requests stay refused.
#[test]
fn admit_grants_a_permit_for_the_milestone_in_the_registered_build() {
    let registered = COMPILED_IDENTITY == Some(REGISTERED_PROFILES[0].identity);
    for mode in MODES {
        let (request, capture) = CapturedInvocation::parse(milestone(), mode).unwrap();
        let admitted = admit(&capture, &request, Entry::Direct);
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
        assert!(required <= 3_623_878_656, "the 0.9 M margin holds at admission");
        // Headless is refused at D1.0 even in the registered build.
        let raw = milestone();
        let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
        let id = String::from("g6");
        let headless = admit(&capture, &request, Entry::Headless(RetainedHeadlessContext::from_borrowed_roots(&raw, &invocation, &id)));
        assert_eq!(headless.err().unwrap().law().refusal, Some(AdmissionRefusal::Caller(RetainedCaller::Headless)));
        // An out-of-domain request (B1, PLAN_v2 §2.3 and RV107 SF-2: C + 1 load cases) is
        // refused at its D1 clause, D1.4. (G6: the blocked examples and the failed attempt
        // are in `registered_g_c_declines_only_unattempted_solves`.)
        let (request, capture) = CapturedInvocation::parse(milestone_cases(caps::LOAD_CASES + 1), mode).unwrap();
        assert_eq!(admit(&capture, &request, Entry::Direct).err().unwrap().law().refusal,
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
            let report = assess(&capture, &request, Entry::Direct);
            assert_eq!(report.law().domain, None, "inside D1");
            assert_eq!(report.law().refusal, d1_1_refusal(), "D1.1: the qualified build admits; any other is Stale");
            assert_eq!(report.profile, build_profile());
            assert_eq!(report.law().raw_text.control_bytes, 0);
            let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
            let id = String::from("g5");
            let headless = assess(&capture, &request, Entry::Headless(RetainedHeadlessContext::from_borrowed_roots(&raw, &invocation, &id)));
            assert_eq!(headless.law().refusal, Some(AdmissionRefusal::Caller(RetainedCaller::Headless)), "D1.0 first: Headless is refused");
            assert_eq!(headless.law().refusal.unwrap().precondition().as_str(), "caller");
        }
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
    assert_eq!(with(&|r| r["model"]["schema_version"] = json!("0.3.0")), family(C::Namespace, F::SchemaVersion));
    assert_eq!(with(&|r| r["model"]["schema_version"] = json!("0.2.0")), None, "0.2.0 is in the namespace");
    assert_eq!(with(&|r| r["model"]["pressure_contract"] = json!({})), family(C::Namespace, F::PressureContract));
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
    assert_eq!(with(&|r| r["model"]["combinations"] = json!([{"id": "c", "basis": "mechanics", "terms": [{"load_case": "case", "factor": 1.0}]}])),
        family(C::Invocation, F::Combinations));
    assert_eq!(with(&|r| r["model"]["components"] = json!([{"id": "k", "kind": "elbow", "node": "N0"}])), family(C::Invocation, F::Components));
    let case = |field: &'static str, value: Value| move |r: &mut Value| r["model"]["load_cases"][0][field] = value.clone();
    assert_eq!(with(&case("pressure_regions", json!([]))), family(C::Case, F::PressureRegions));
    assert_eq!(with(&case("equivalent_static", json!({}))), family(C::Case, F::EquivalentStatic));
    assert_eq!(with(&case("modulus_basis_ref", json!("T0"))), family(C::Case, F::ModulusBasisRef));
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
    assert_eq!(with(&load("target", json!({"type": "element", "pipe": "M1"}))), family(C::Loads, F::LoadTarget));
    assert_eq!(with(&load("dimension", json!("pressure"))), family(C::Loads, F::LoadDimension));
    assert_eq!(with(&load("dimension", json!("force"))), None);
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
        ("modulus_basis_ref", json!("T0"), F::ModulusBasisRef),
        ("modulus_basis_temperature", json!({"value": 20.0, "unit": "degC"}), F::ModulusBasisTemperature),
        ("analysis_state", Value::Null, F::AnalysisState),
    ] {
        assert_eq!(with(&on_last(field, value)), family(C::Case, fact), "D1.5 on case {last}: {field}");
    }
    let load_on_last = |field: &'static str, value: Value| move |r: &mut Value| {
        *r = with_cases(r.clone(), caps::LOAD_CASES);
        r["model"]["load_cases"][last]["primitive_loads"][2][field] = value.clone();
    };
    assert_eq!(with(&load_on_last("target", json!({"type": "element", "pipe": "M1"}))), family(C::Loads, F::LoadTarget), "D1.7 on case {last}");
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
    let facts = DomainFacts { raw: &report.raw, raw_text: &law.raw_text, typed: &report.typed, nested: &law.nested, headless: None, digest: &report.captured_digest };
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
    // A raw-only string (an unknown field) is bounded by the raw text row.
    assert!(cap(CapFact::RawTextBytes)(over(&|r| model(r).insert("unknown".into(), json!(text("u", 129))).map(drop).unwrap_or(()))));
    assert!(cap(CapFact::RawKeyTextBytes)(over(&|r| model(r).insert(text("k", 129), json!(1)).map(drop).unwrap_or(()))));
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
    // `model.deep` sits at depth 2, so `levels` arrays put its scalar at depth levels + 2.
    assert_eq!(over(&|r| model(r).insert("deep".into(), nested(14)).map(drop).unwrap_or(())), None, "depth 16");
    assert!(cap(CapFact::RawDepth)(over(&|r| model(r).insert("deep".into(), nested(15)).map(drop).unwrap_or(()))));
    // Raw totals: string bytes and key bytes.
    assert!(cap(CapFact::RawStringBytes)(over(&|r| {
        model(r).insert("bulk".into(), Value::Array((0..520).map(|i| json!(text(&format!("b{i}"), 128))).collect())).map(drop).unwrap_or(())
    })));
    assert!(cap(CapFact::RawKeyBytes)(over(&|r| {
        let keys: serde_json::Map<String, Value> = (0..520).map(|i| (text(&format!("k{i}"), 128), json!(0))).collect();
        model(r).insert("keys".into(), Value::Object(keys)).map(drop).unwrap_or(())
    })));
    // Raw value count: past 16,384 the census itself stops (D1.2).
    let report = admitted({
        let mut raw = cap_maximal();
        model(&mut raw).insert("many".into(), Value::Array(vec![json!(0); VALUE_LIMIT]));
        raw
    });
    assert_eq!(report.law().domain, Some(AdmissionRefusal::Census(CensusPart::Raw, CensusStatus::ValueLimit)));
    // Raw capacities: owners with spare capacity in an unknown field.
    let spare = |value: Value| {
        let mut raw = cap_maximal();
        model(&mut raw).insert("spare".into(), value);
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
        (CapFact::CombinationsCapacity, Box::new(|r| r.model.combinations.reserve_exact(1))),
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
            &DomainFacts { raw: &raw, raw_text: &text, typed: &typed, nested: &nested, headless: headless.as_ref(), digest: &report.captured_digest },
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
            let output = crate::run_linear_static_preview_value_with_retained_direct(raw.clone(), mode).unwrap();
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
        assert!(maximum + r <= 3_623_878_656, "{mode:?}: the 0.9 M margin holds at admission");
    }
    // The R override sizes only the witness thread; the bound ignores it.
    RESERVED_STACK_OVERRIDE.with(|c| c.set(Some(1 << 20)));
    assert_eq!(admission_bound(Ok(M - r), M), Ok(M));
    RESERVED_STACK_OVERRIDE.with(|c| c.set(None));
    let source = include_str!("retained_memory.rs");
    let admit_src = &source[source.find("pub(super) fn admit(").unwrap()..];
    let admit_src = &admit_src[..admit_src.find("\n}\n").unwrap()];
    assert_eq!(admit_src.matches("admission_bound(cap_priced_maximum(mode), REGISTERED_PROFILES[index].threshold_bytes)").count(), 1);
    assert_eq!(admit_src.matches("required = bound_required(&bound);").count(), 1);
    assert_eq!(admit_src.matches("report.law.required = required;").count(), 1);
    assert!(!admit_src.contains("bound_admits("), "admit prices its bound only through admission_bound");
    // In a build that refuses before the bound, nothing is recorded.
    if build_status().is_err() {
        for mode in MODES {
            let (request, capture) = CapturedInvocation::parse(milestone(), mode).unwrap();
            assert_eq!(assess(&capture, &request, Entry::Direct).law().required, None, "{mode:?}");
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
    assert_eq!(P_FINAL, 2_115);
    // B1 SA (option S3, C = 3): the bounds that read the regenerated profile are asserted as
    // expressions (SF-4's back edge: SQ fixes their values; RV-Q round 1 reviews these).
    let c = caps::LOAD_CASES as u64;
    assert_eq!(caps.complete[2], 2 * c * P_FINAL * text_atoms::ROW, "2·C·P_final·Text(row)");
    assert_eq!(caps.complete[5], 2 * profile::TEXT_TEXT_DIAG_ENV, "2·Text(diag_env) at l ≤ 128 (API_G4.md, S-6(d))");
    assert_eq!(profile::TEXT_TEXT_DIAG_ENV, 68_720_236, "the part-2 text closure (1e323058f3, R-4 graph, RV87 S-2)");
    assert_eq!((caps.complete[0], caps.complete[1]), (3 * 2_115, 8_192), "C·P_final, PushCap(C·P_final)");
    assert_eq!(caps.complete[4], push_capacity(text_atoms::D_ENV), "PushCap(D_env)");
    assert!(caps.late[LATE_FACTS - 1] > 0 && caps.late[LATE_FACTS - 1] < caps.complete[15], "T11 without its late capture < T11");
    assert!(caps.complete[16] > 0 && caps.complete[16] < caps.complete[15], "T11.4 < T11");
    assert_eq!((caps.complete[6], caps.complete[7]), (text_atoms::L_PUB, text_atoms::L_DIAGID), "L_PUB (RV84 C-N1) and L_DIAGID (RV87 N-3)");
    assert_eq!(caps.complete[17], c * (3 * 32 + 1) * text_atoms::ERR, "C·(3m + 1)·Text(err)");
    // The atoms' values in the profile as registered at G6 (c = 1). SQ re-pins them with the
    // regenerated profile; until then they are unchanged by SA.
    assert_eq!((text_atoms::L_PUB, text_atoms::L_DIAGID, text_atoms::ERR, push_capacity(text_atoms::D_ENV)), (2_599_962, 2_330, 16_384, 16_384),
        "L_PUB, L_DIAGID, Text(err) and PushCap(D_env) as registered");
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
            (bytes + r) as f64 / 4_026_531_840.0, PHASE_NAMES[phase]);
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
    assert_eq!(PHASE_NAMES.map(|n| n.split(' ').next().unwrap()), ["W1", "W2", "W3", "W4", "W5", "X1", "X2"]);
    // The pinned record G6 qualifies (R/I65/u4_g6_01/QUALIFICATION.md §3): each phase's requested
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
    assert_eq!((SPARSE, DENSE), (Some((PINNED_RECORD[0][2], 2)), Some((PINNED_RECORD[1][2], 2))), "W3 is the maximum in both modes");
}
/// The build identity of the pinned profile record (the qualified dev/test build of G6).
pub(super) const PINNED_RECORD_IDENTITY: &str = "v1;rustc.release=1.97.1;rustc.commit=8bab26f4f68e0e26f0bb7960be334d5b520ea452;rustc.host=aarch64-apple-darwin;rustc.llvm=22.1.6;target=aarch64-apple-darwin;target.arch=aarch64;target.pointer_width=64;target.endian=little;target.os=macos;target.env=;panic=unwind;profile=debug;opt_level=0;debug_assertions=true;rustflags=;pkg=open_pipe_stress_product_physics@0.2.0";
/// The pinned record (W1, W2, W3, W4, W5, X1, X2), sparse then dense, in that build.
pub(super) const PINNED_RECORD: [[u64; 7]; 2] = [
    [1_856_156_348, 1_963_966_754, 3_508_669_422, 3_482_311_587, 2_136_222_836, 3_256_308_814, 1_777_046_728],
    [1_875_866_796, 1_983_677_202, 3_528_379_870, 3_502_022_035, 2_155_933_284, 3_276_019_262, 1_796_757_176],
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
    assert_eq!(caps.complete[3], 9_361);
    assert_eq!(text_atoms::ROW, 11_474);
    // Every atom is bound; the source-derived and estimate atoms are the recorded ones.
    let count = |b| profile::ATOM_BINDINGS.iter().filter(|x| **x == b).count();
    assert_eq!((count(profile::Binding::SourceUpper), count(profile::Binding::Text), count(profile::Binding::Estimate)), (16, 6, 0));
    assert_eq!(profile::ESTIMATES, 0, "G6 closed every Estimate");
    assert_eq!(cap_priced_maximum(PreviewSolverMode::SparseInteractive), Ok(profile::SPARSE.unwrap().0), "priced in-build");
    assert_eq!(cap_priced_maximum(PreviewSolverMode::DenseScrutiny), Ok(profile::DENSE.unwrap().0), "priced in-build");
    // The in-build maximum is within the 0.9 M margin rule in both modes (RR "U4 G3 verified").
    let r = RESERVED_STACK_BYTES as u64;
    for (bytes, _) in [profile::SPARSE.unwrap(), profile::DENSE.unwrap()] {
        assert!(bytes + r <= 3_623_878_656, "{} above 0.9 M", bytes + r);
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
    assert_eq!(assess(&capture, &request, Entry::Direct).law().domain,
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
    let read = build_identity::encode_reviewed_inputs(&[Some([7; 32]); 14]);
    assert!(!read.contains("unavailable"));
    assert!(bindings_hold(true, Some(&read), &read, &layouts, &layouts), "every input read: the record binds");
    for i in 0..14 {
        let mut digests = [Some([7u8; 32]); 14];
        digests[i] = None;
        let unread = build_identity::encode_reviewed_inputs(&digests);
        assert!(unread.contains("=unavailable"));
        assert!(!bindings_hold(true, Some(&unread), &unread, &layouts, &layouts), "input {i} unreadable");
    }
    let none = build_identity::encode_reviewed_inputs(&[None; 14]);
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
        ("rejected_stress_range sparse", serde_json::from_str(include_str!(
            "../../../fixtures/product_preview/source_blocks/rejected_stress_range/sparse_interactive.request.json")).unwrap()),
        ("rejected_stress_range dense", serde_json::from_str(include_str!(
            "../../../fixtures/product_preview/source_blocks/rejected_stress_range/dense_scrutiny.request.json")).unwrap()),
    ]
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
            "schema_version": "0.1.0",
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
                assert_eq!(assess(&capture, &request, Entry::Direct).law().domain, None, "{label}: inside D1");
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
            let direct = crate::run_linear_static_preview_value_with_retained_direct(raw, mode).unwrap();
            assert_eq!(direct.admission().unwrap().law().refusal, None, "{label}: admitted");
            match direct.retained() {
                Some(Err(crate::W1Fallback::CompleteGate(refusal))) => assert_eq!(refusal.fact, PhaseFact::OrdinarySolveNotAttempted, "{label} {mode:?}"),
                other => panic!("{label} {mode:?}: expected G-C's decline, got {other:?}"),
            }
            assert_eq!(serde_json::to_vec(direct.envelope()).unwrap(), plain, "{label} {mode:?}: the value route's bytes exactly, no notice");
            assert!(direct.successor().is_none());
        }
        let milestone = crate::run_linear_static_preview_value_with_retained_direct(milestone(), mode).unwrap();
        assert!(milestone.successor().is_some(), "{mode:?}: the milestone publishes");
        for (label, raw) in attempted_examples().into_iter().skip(1) {
            let direct = crate::run_linear_static_preview_value_with_retained_direct(raw, mode).unwrap();
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
            let admitted = admit(&capture, &request, Entry::Direct);
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

/// PLAN_v2 §2.3 and RV107 SF-2: the runner's out-of-domain oracle
/// (`core/runner/headless/tests/retained_precision_admission.rs`,
/// `explicit_headless_refusal_preserves_output_and_completion_fields_both_modes`) cannot read
/// `pub(crate)` `caps::LOAD_CASES`, and no D1 item's visibility changes for a test, so it builds
/// a literal 4 load cases. This test ties that literal to the producer: 4 is `LOAD_CASES + 1`,
/// and `admit` refuses `LOAD_CASES + 1` cases at D1.4 with `(Invocation, LoadCases)`. If C
/// changes, this test fails first and names the runner test to re-base.
#[test]
fn b1_sa_runner_oracle_literal_is_load_cases_plus_one() {
    const RUNNER_LITERAL: usize = 4;
    assert_eq!(caps::LOAD_CASES + 1, RUNNER_LITERAL, "re-base the runner's literal (explicit_headless_refusal_preserves_output_and_completion_fields_both_modes)");
    for mode in MODES {
        let (request, capture) = CapturedInvocation::parse(milestone_cases(caps::LOAD_CASES + 1), mode).unwrap();
        let report = admit(&capture, &request, Entry::Direct).err().expect("C + 1 cases are refused in every build");
        assert_eq!(report.typed.load_cases.length, RUNNER_LITERAL);
        assert_eq!(report.law().domain, family(D1Clause::Invocation, FamilyFact::LoadCases), "{mode:?}");
        assert_eq!(report.law().refusal, d1_1_refusal().or(family(D1Clause::Invocation, FamilyFact::LoadCases)), "{mode:?}");
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
        let facts = DomainFacts { raw: &report.raw, raw_text: &law.raw_text, typed: &report.typed, nested: &nested, headless: None, digest: &report.captured_digest };
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
            let (permit, _report) = admit(&capture, &request, Entry::Direct).unwrap_or_else(|r| panic!("{mode:?}: {:?}", r.law().refusal));
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
            assert_eq!(assess(&capture, &request, Entry::Direct).law().domain, None, "{label}: inside D1");
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
            let direct = crate::run_linear_static_preview_value_with_retained_direct(raw.clone(), mode).unwrap();
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
    let late = [n, m, m, g, 128, 384, 192, 192, 8, profile_bytes(F_T11).saturating_sub(profile_bytes(F_T11_LATE_CAPTURE))];
    assert_eq!(p.late, late, "G-B: n, m, m, g, l, L, min(6n, Σr), s, 4 + 4, T11 − T11_late_capture");
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
        (F::ContractEvidenceEntries, c * 553),
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
