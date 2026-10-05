//! RV89 (scratch only; never in maintained code): independent probes of U4 G5 part 1.
//! Mounted as a child of retained_memory in RV89's archive copy, so it sees the
//! module's private items. It constructs no profile and no permit.
use super::*;
use crate::source_receipt::CapturedInvocation;
use crate::PreviewSolverMode;
use serde_json::json;
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

// ---- A per-thread counting allocator (the PP/tests counting pattern) ---------
struct Counting;
thread_local! { static COUNT: Cell<Option<u64>> = const { Cell::new(None) }; }
fn note() {
    let _ = COUNT.try_with(|c| {
        if let Some(n) = c.get() {
            c.set(Some(n + 1));
        }
    });
}
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        note();
        System.alloc(l)
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        note();
        System.alloc_zeroed(l)
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        note();
        System.realloc(p, l, n)
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        System.dealloc(p, l)
    }
}
#[global_allocator]
static RV89_ALLOCATOR: Counting = Counting;
fn allocations<R>(f: impl FnOnce() -> R) -> (u64, R) {
    COUNT.with(|c| c.set(Some(0)));
    let r = f();
    let n = COUNT.with(|c| c.replace(None)).unwrap();
    (n, r)
}

const MILESTONE: &str = include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
fn milestone() -> Value {
    serde_json::from_str(MILESTONE).unwrap()
}
/// RV89's own cap-maximal D1 request (not I65's helper).
fn capmax() -> Value {
    let p = "rv89_invented_cap_maximal_no_library_data";
    let nodes: Vec<Value> = (0..32).map(|i| json!({"id": format!("n{i:02}"), "position": {"x": i as f64, "y": 0.5 * (i % 3) as f64, "z": 0.0}, "provenance": p})).collect();
    let pipes: Vec<Value> = (0..32).map(|i| json!({"id": format!("p{i:02}"), "from": format!("n{i:02}"), "to": format!("n{:02}", (i + 1) % 32), "material": "m0",
        "section": {"outside_diameter": {"value": 0.1683, "unit": "m"}, "wall_thickness": {"value": 0.00711, "unit": "m"}}})).collect();
    let supports: Vec<Value> = (0..32).map(|i| json!({"id": format!("s{i:02}"), "node": format!("n{i:02}"), "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"],
        "family": "anchor", "stiffness": {"dof": "UZ", "value": {"value": 2.5e6, "unit": "N/m"}}})).collect();
    let dirs = ["FX", "FY", "FZ", "MX", "MY", "MZ"];
    let loads: Vec<Value> = (0..128).map(|i| json!({"id": format!("l{i:03}"), "category": "weight", "target": {"type": "node", "node": format!("n{:02}", i % 32)},
        "direction": dirs[i % 6], "magnitude": {"value": 10.0 + i as f64, "unit": if i % 6 < 3 { "N" } else { "N*m" }},
        "dimension": if i % 6 < 3 { "force" } else { "moment" }, "provenance": "rv89 plain text"})).collect();
    let pts: Vec<Value> = (0..16).map(|i| json!({"id": format!("t{i:02}"), "temperature": {"value": 20.0 + i as f64, "unit": "degC"}})).collect();
    let mats: Vec<Value> = (0..4).map(|i| json!({"id": format!("m{i}"), "elastic_modulus": {"value": 2.0e11, "unit": "Pa"}, "poisson_ratio": {"value": 0.3, "unit": "1"}, "temperature_points": pts})).collect();
    json!({"model": {"schema_version": "0.1.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "q".repeat(128), "units": {"length": "m", "force": "N"}},
        "analysis_status": {"mechanics": "preview", "rule_check": "not_performed", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "materials": mats,
        "load_cases": [{"id": "c0", "primitive_loads": loads}], "combinations": [], "components": [], "sections": []},
        "materials": mats})
}
fn parse(raw: Value, mode: PreviewSolverMode) -> (LinearStaticPreviewRequest, CapturedInvocation) {
    CapturedInvocation::parse(raw, mode).unwrap()
}
fn report_of(raw: Value) -> RetainedAdmissionReport {
    let (request, capture) = parse(raw, PreviewSolverMode::SparseInteractive);
    assess(&capture, &request, Entry::Direct)
}
fn dom(raw: Value) -> Option<AdmissionRefusal> {
    report_of(raw).law().domain
}
fn dom_typed(raw: Value, change: impl FnOnce(&mut LinearStaticPreviewRequest)) -> Option<AdmissionRefusal> {
    let (mut request, capture) = parse(raw, PreviewSolverMode::SparseInteractive);
    change(&mut request);
    assess(&capture, &request, Entry::Direct).law().domain
}
fn is_cap(r: Option<AdmissionRefusal>, fact: CapFact, observed: usize, cap: usize) -> bool {
    r == Some(AdmissionRefusal::Cap { fact, observed, cap })
}
fn with(base: fn() -> Value, f: impl FnOnce(&mut Value)) -> Value {
    let mut r = base();
    f(&mut r);
    r
}
fn mdl(r: &mut Value) -> &mut serde_json::Map<String, Value> {
    r["model"].as_object_mut().unwrap()
}

// ---- SHA-256 against sha2 ------------------------------------------------------
fn hex(d: [u8; 32]) -> String {
    d.iter().map(|b| format!("{b:02x}")).collect()
}
fn sha2_of(data: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    Sha256::digest(data).into()
}
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn bytes(&mut self, n: usize) -> Vec<u8> {
        (0..n).map(|_| (self.next() >> 24) as u8).collect()
    }
}
#[test]
fn rv89_sha256_nist_vectors_padding_edges_and_large_random() {
    use build_identity::sha256;
    let nist: [(&[u8], &str); 4] = [
        (b"", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
        (b"abc", "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"),
        (b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq", "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"),
        (b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
            "cf5b16a778af8380036ce59e7b0492370b249b11e8f07a51afac45037afee9d1"),
    ];
    for (m, d) in nist {
        assert_eq!(hex(sha256(m)), d);
        assert_eq!(sha256(m), sha2_of(m));
    }
    let million = vec![b'a'; 1_000_000];
    assert_eq!(hex(sha256(&million)), "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0");
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    let mut checked = 0;
    // Every length 0..=1100 (covers 55/56/63/64/65 and their block multiples), random content.
    for len in 0..=1100usize {
        let data = rng.bytes(len);
        assert_eq!(sha256(&data), sha2_of(&data), "len {len}");
        checked += 1;
    }
    // The padding edges explicitly, with all-zero, all-0xFF and 0x80 content.
    for base in [0usize, 64, 128, 4096, 65_536] {
        for edge in [55usize, 56, 57, 63, 64, 65, 119, 120, 127, 128, 129] {
            for fill in [0x00u8, 0xFF, 0x80] {
                let data = vec![fill; base + edge];
                assert_eq!(sha256(&data), sha2_of(&data), "len {} fill {fill:#x}", base + edge);
                checked += 1;
            }
        }
    }
    // Large random inputs.
    for len in [1usize << 20, (3 << 20) + 7, 5_000_003, 16_777_216 + 55] {
        let data = rng.bytes(len);
        assert_eq!(sha256(&data), sha2_of(&data), "len {len}");
        checked += 1;
    }
    // Random lengths.
    for _ in 0..2000 {
        let len = (rng.next() % 20_000) as usize;
        let data = rng.bytes(len);
        assert_eq!(sha256(&data), sha2_of(&data), "len {len}");
        checked += 1;
    }
    println!("RV89_SHA256 checked={checked}");
}

// ---- The identity encoder ------------------------------------------------------
#[test]
fn rv89_escaping_round_trips_and_cannot_forge_a_key() {
    use build_identity::*;
    // Every single byte and every ordered pair of bytes.
    for a in 0..=255u8 {
        for b in 0..=255u8 {
            let mut s = String::new();
            encode_identity_value(&[a, b], &mut s);
            assert!(s.bytes().all(|x| (0x21..0x7F).contains(&x) && x != b';' && x != b'='), "{a:#x} {b:#x}: {s}");
            assert_eq!(decode_identity_value(&s), Some(vec![a, b]));
        }
    }
    // Forgery attempts: values that look like keys, separators, newlines and escapes.
    let hostile: [&[u8]; 9] = [b";target=evil", b"\ncargo:rustc-env=X=1", b"=", b";", b"%3B", b"%", b"a b", b"\x00\x7f\xff", "é;π=".as_bytes()];
    let mut rng = Rng(7);
    for round in 0..400 {
        let values: Vec<Vec<u8>> = (0..16)
            .map(|i| if round < 9 { hostile[(i + round) % 9].to_vec() } else { let n = (rng.next() % 40) as usize; rng.bytes(n) })
            .collect();
        let borrowed: [&[u8]; 16] = std::array::from_fn(|i| values[i].as_slice());
        let text = encode_identity(&borrowed);
        assert!(!text.contains(['\n', '\r', ' ']), "one line, no space");
        assert_eq!(text.matches(';').count(), 16, "exactly 16 separators");
        assert_eq!(text.matches('=').count(), 16, "exactly 16 key=value splits");
        assert_eq!(decode_identity(&text), Some(values.clone()), "round {round}");
        // The cargo directive keeps the whole text (cargo splits at the first '=' only).
        let directive = format!("cargo:rustc-env={IDENTITY_VARIABLE}={text}");
        let (_, rest) = directive.split_once("cargo:rustc-env=").unwrap();
        assert_eq!(rest.split_once('=').unwrap().1, text);
    }
    // A decoded identity with a smuggled extra key, a reordered key or a lowercase escape is refused.
    let empty: [&[u8]; 16] = [b""; 16];
    let text = encode_identity(&empty);
    assert_eq!(decode_identity(&format!("{text};extra=1")), None);
    assert_eq!(decode_identity(&text.replacen("rustc.release", "rustc.commit", 1)), None);
    assert_eq!(decode_identity_value("%0a"), None);
    assert_eq!(decode_identity("v2;rustc.release="), None);
    // The reviewed-input encoding: 14 entries, lowercase hex, `unavailable` per unreadable input.
    let mut digests: [Option<[u8; 32]>; 14] = [Some([0xAB; 32]); 14];
    digests[5] = None;
    let r = encode_reviewed_inputs(&digests);
    assert_eq!(r.matches(';').count(), 14);
    assert!(r.contains(&format!("{}=unavailable", REVIEWED_INPUTS[5])));
    assert!(r.contains(&format!("{}={}", REVIEWED_INPUTS[0], "ab".repeat(32))));
}

#[test]
fn rv89_identity_status_and_bindings() {
    use build_identity::IDENTITY_UNAVAILABLE;
    let none: [&str; 0] = [];
    for compiled in [None, Some(IDENTITY_UNAVAILABLE), Some("v1;x=y"), Some("")] {
        assert_eq!(identity_match(compiled, none.into_iter()), Err(ProfileStatus::Missing));
    }
    let reg = ["v1;x=y", "", "v1;x=z"];
    assert_eq!(identity_match(None, reg.into_iter()), Err(ProfileStatus::Stale), "option_env! absence is Stale");
    assert_eq!(identity_match(Some(IDENTITY_UNAVAILABLE), reg.into_iter()), Err(ProfileStatus::Stale));
    assert_eq!(identity_match(Some("v1;x=y "), reg.into_iter()), Err(ProfileStatus::Stale));
    assert_eq!(identity_match(Some("v1;x="), reg.into_iter()), Err(ProfileStatus::Stale), "no prefix match");
    assert_eq!(identity_match(Some("v1;x=z"), reg.into_iter()), Ok(2));
    assert_eq!(identity_match(Some(""), reg.into_iter()), Ok(1), "an empty registered text matches only an empty compiled text");
    // This build: nothing registered, so Missing, whatever the compiled text.
    assert_eq!(build_status(), Err(ProfileStatus::Missing));
    assert!(COMPILED_IDENTITY.is_some() && COMPILED_REVIEWED_INPUTS.is_some());
    assert!(LAYOUT_WITNESSES);
    let l = READER_LAYOUTS;
    assert!(!bindings_hold(true, Some("a"), "a", &l, &l[..3]), "a shorter recorded layout list is a mismatch");
    println!("RV89_IDENTITY {}", COMPILED_IDENTITY.unwrap());
    println!("RV89_REVIEWED {}", COMPILED_REVIEWED_INPUTS.unwrap());
}

// ---- D1: cap and cap+1, through the actual G-A -----------------------------------
#[test]
fn rv89_capmax_is_inside_d1_and_every_count_sits_at_its_cap() {
    for mode in MODES {
        let (request, capture) = parse(capmax(), mode);
        let r = assess(&capture, &request, Entry::Direct);
        assert_eq!(r.law().domain, None, "{mode:?}");
        assert_eq!(r.law().refusal, Some(AdmissionRefusal::Profile(ProfileStatus::Missing)));
        let n = &r.law().nested;
        assert_eq!((r.typed.nodes.length, r.typed.members.length, r.typed.supports.length), (32, 32, 32));
        assert_eq!((n.restraints, n.springs, n.primitive_loads.length, n.max_temperature_points.length), (192, 32, 128, 16));
        assert_eq!((r.typed.model_materials.length, r.typed.request_materials.length), (4, 4));
        assert_eq!(n.max_string_bytes, 128);
    }
}

#[test]
fn rv89_cap_and_cap_plus_one_for_each_fact() {
    use CapFact as K;
    // nodes 33
    let r = dom(with(capmax, |r| { let x = json!({"id": "n32", "position": {"x": 0.0, "y": 9.0, "z": 0.0}}); r["model"]["nodes"].as_array_mut().unwrap().push(x) }));
    assert!(is_cap(r, K::Nodes, 33, 32), "{r:?}");
    // members 33
    let r = dom(with(capmax, |r| { let mut x = r["model"]["pipe_segments"][0].clone(); x["id"] = json!("p32"); r["model"]["pipe_segments"].as_array_mut().unwrap().push(x) }));
    assert!(is_cap(r, K::Members, 33, 32), "{r:?}");
    // supports 33 (no restraints, so r stays 192)
    let r = dom(with(capmax, |r| r["model"]["supports"].as_array_mut().unwrap().push(json!({"id": "s32", "node": "n00", "restraints": []}))));
    assert!(is_cap(r, K::Supports, 33, 32), "{r:?}");
    // restraints Σ 193; and a single support at 192 restraints is admitted when the rest have none.
    let r = dom(with(capmax, |r| r["model"]["supports"][7]["restraints"].as_array_mut().unwrap().push(json!("UX"))));
    assert!(is_cap(r, K::Restraints, 193, 192), "{r:?}");
    let one: Vec<Value> = (0..192).map(|i| json!(format!("R{i}"))).collect();
    let r = dom(with(capmax, |r| { for s in r["model"]["supports"].as_array_mut().unwrap() { s["restraints"] = json!([]); } r["model"]["supports"][0]["restraints"] = json!(one.clone()) }));
    assert_eq!(r, None, "one support with 192 restraints");
    // loads 129
    let r = dom(with(capmax, |r| { let mut x = r["model"]["load_cases"][0]["primitive_loads"][0].clone(); x["id"] = json!("l128"); r["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().push(x) }));
    assert!(is_cap(r, K::Loads, 129, 128), "{r:?}");
    // materials 5 (model) and 5 (request)
    let r = dom(with(capmax, |r| { let mut x = r["model"]["materials"][0].clone(); x["id"] = json!("m4"); r["model"]["materials"].as_array_mut().unwrap().push(x) }));
    assert!(is_cap(r, K::ModelMaterials, 5, 4), "{r:?}");
    let r = dom(with(capmax, |r| { let mut x = r["materials"][0].clone(); x["id"] = json!("m4"); r["materials"].as_array_mut().unwrap().push(x) }));
    assert!(is_cap(r, K::RequestMaterials, 5, 4), "{r:?}");
    // temperature points 17, in the request list only
    let r = dom(with(capmax, |r| r["materials"][3]["temperature_points"].as_array_mut().unwrap().push(json!({"id": "t16"}))));
    assert!(is_cap(r, K::TemperaturePoints, 17, 16), "{r:?}");
    // typed text: 128 bytes admitted in each of several typed owners, 129 refused (bytes, not chars)
    for path in [&["model", "nodes", "3", "provenance"][..], &["model", "load_cases", "0", "primitive_loads", "5", "category"], &["model", "supports", "2", "stiffness", "value", "unit"],
        &["materials", "1", "temperature_points", "4", "id"], &["model", "analysis_status", "rule_check"]] {
        let set = |r: &mut Value, s: String| {
            let mut v = &mut *r;
            for k in path { v = if let Ok(i) = k.parse::<usize>() { &mut v[i] } else { &mut v[*k] }; }
            *v = json!(s);
        };
        assert_eq!(dom(with(capmax, |r| set(r, "a".repeat(128)))), None, "{path:?} at 128");
        let r = dom(with(capmax, |r| set(r, "a".repeat(129))));
        // A typed owner is also a raw string: D1.9's typed row precedes the raw row.
        assert!(is_cap(r, K::TypedTextBytes, 129, 128), "{path:?}: {r:?}");
        assert_eq!(dom(with(capmax, |r| set(r, "é".repeat(64)))), None, "{path:?}: 128 bytes of 2-byte chars");
        assert!(is_cap(dom(with(capmax, |r| set(r, "é".repeat(65)))), K::TypedTextBytes, 130, 128));
    }
    // raw-only string and key (unknown fields)
    assert_eq!(dom(with(capmax, |r| { mdl(r).insert("unknown".into(), json!("u".repeat(128))); })), None);
    assert!(is_cap(dom(with(capmax, |r| { mdl(r).insert("unknown".into(), json!("u".repeat(129))); })), K::RawTextBytes, 129, 128));
    assert_eq!(dom(with(capmax, |r| { mdl(r).insert("k".repeat(128), json!(0)); })), None);
    assert!(is_cap(dom(with(capmax, |r| { mdl(r).insert("k".repeat(129), json!(0)); })), K::RawKeyTextBytes, 129, 128));
    // raw depth 16 admitted, 17 refused (root at 0; `model.deep` at 2)
    let nest = |levels: usize| { let mut v = json!(0); for _ in 0..levels { v = json!([v]); } v };
    assert_eq!(dom(with(capmax, |r| { mdl(r).insert("deep".into(), nest(14)); })), None);
    assert!(is_cap(dom(with(capmax, |r| { mdl(r).insert("deep".into(), nest(15)); })), K::RawDepth, 17, 16));
    // raw string bytes total: fill an unknown array to exactly 65,536 and 65,537 bytes.
    let total = report_of(capmax()).raw.string_bytes;
    let fill = |extra: usize| move |r: &mut Value| {
        let mut left = extra;
        let mut a = Vec::new();
        while left > 0 { let k = left.min(128); a.push(json!("s".repeat(k))); left -= k; }
        mdl(r).insert("bulk".into(), Value::Array(a));
    };
    assert_eq!(dom(with(capmax, fill(65_536 - total))), None, "raw string bytes at 65,536");
    assert!(is_cap(dom(with(capmax, fill(65_537 - total))), K::RawStringBytes, 65_537, 65_536));
    // raw key bytes total
    let keys = report_of(capmax()).raw.key_bytes + 4; // + the "keys" key itself
    let kfill = |extra: usize| move |r: &mut Value| {
        let mut m = serde_json::Map::new();
        let mut left = extra;
        let mut i = 0;
        while left > 0 {
            let k = if left > 134 { 128 } else if left > 128 { left - 6 } else { left };
            assert!(k >= 6);
            let mut s = format!("k{i:05}");
            while s.len() < k { s.push('x'); }
            left -= s.len();
            m.insert(s, json!(0));
            i += 1;
        }
        mdl(r).insert("keys".into(), Value::Object(m));
    };
    let at = report_of(with(capmax, kfill(65_536 - keys))).raw.key_bytes;
    assert_eq!(at, 65_536, "constructed exactly at the cap");
    assert_eq!(dom(with(capmax, kfill(65_536 - keys))), None);
    assert!(is_cap(dom(with(capmax, kfill(65_537 - keys))), K::RawKeyBytes, 65_537, 65_536));
    // typed capacities (after the parse): load cases 2, primitive loads 129, restraints 193, a String 129.
    assert!(is_cap(dom_typed(capmax(), |q| q.model.load_cases.reserve_exact(1)), K::LoadCasesCapacity, 2, 1));
    let r = dom_typed(capmax(), |q| q.model.load_cases[0].primitive_loads.reserve_exact(1));
    assert!(is_cap(r, K::LoadsCapacity, 129, 128), "{r:?}");
    let r = dom_typed(capmax(), |q| q.model.supports[3].restraints.reserve_exact(100));
    assert!(is_cap(r, K::RestraintCapacityTotal, 292, 192), "{r:?}");
    let r = dom_typed(capmax(), |q| { for s in q.model.supports.iter_mut() { s.restraints = Vec::new(); } q.model.supports[0].restraints = Vec::with_capacity(193); });
    assert!(is_cap(r, K::RestraintCapacity, 193, 192), "{r:?}");
    let r = dom_typed(capmax(), |q| { let mut s = String::with_capacity(129); s.push_str("q"); q.model.project.id = s; });
    assert!(is_cap(r, K::TypedTextCapacity, 129, 128), "{r:?}");
    let r = dom_typed(capmax(), |q| q.model.sections = Vec::with_capacity(1));
    assert!(is_cap(r, K::SectionsCapacity, 1, 0), "{r:?}");
    // units Value: depth and text inside it.
    assert!(is_cap(dom_typed(capmax(), |q| q.model.project.units = { let mut v = Value::Null; for _ in 0..17 { v = Value::Array(vec![v]); } v }), K::UnitsDepth, 17, 16));
    assert_eq!(dom_typed(capmax(), |q| q.model.project.units = { let mut v = Value::Null; for _ in 0..16 { v = Value::Array(vec![v]); } v }), None);
}

#[test]
fn rv89_d1_10_and_d1_11_edges() {
    use CapFact as K;
    let prov = |s: &'static str| move |r: &mut Value| r["model"]["load_cases"][0]["primitive_loads"][127]["provenance"] = json!(s);
    let obj = Some(AdmissionRefusal::Family(D1Clause::Provenance, FamilyFact::ObjectProvenance));
    for s in ["{", "{}", " {\"method\":\"x\"}", "   {"] {
        assert_eq!(dom(with(capmax, prov(s))), obj, "{s:?}");
    }
    // ASCII whitespace that is also a control byte: D1.10 is checked before D1.11.
    for s in ["\t{", "\n{", "\r{", "\x0c{"] {
        assert_eq!(dom(with(capmax, prov(s))), obj, "{s:?}");
    }
    // Vertical tab is not ASCII whitespace in Rust (nor JSON whitespace): D1.10 passes, D1.11 refuses.
    assert!(is_cap(dom(with(capmax, prov("\x0b{"))), K::ControlBytes, 1, 0));
    // Not an object start: inside D1.
    for s in ["x{", "\u{a0}{", "[{}]", "\"{\"", "", " ", "}{"] {
        assert_eq!(dom(with(capmax, prov(s))), None, "{s:?}");
    }
    // D1.11: a control byte in a key, 0x7F in a value and in a key, 0x1F, but not 0x20 or 0x80+.
    assert!(is_cap(dom(with(capmax, |r| { mdl(r).insert("a\u{1}b".into(), json!(0)); })), K::ControlBytes, 1, 0));
    assert!(is_cap(dom(with(capmax, |r| { mdl(r).insert("a\u{7f}".into(), json!(0)); })), K::ControlBytes, 1, 0));
    assert!(is_cap(dom(with(capmax, |r| r["model"]["nodes"][0]["provenance"] = json!("\u{7f}\u{1f}\u{0}"))), K::ControlBytes, 3, 0));
    assert!(is_cap(dom(with(capmax, |r| r["model"]["project"]["units"]["length"] = json!("m\u{1f}"))), K::ControlBytes, 1, 0));
    assert_eq!(dom(with(capmax, |r| r["model"]["nodes"][0]["provenance"] = json!(" \u{80}\u{85}\u{9f}é ~"))), None, "C1 code points are not D1.11 bytes");
    // Order: D1.3 before D1.9; D1.9 before D1.10; D1.10 before D1.11; D1.2 before D1.3.
    assert_eq!(dom(with(capmax, |r| { r["model"]["schema_version"] = json!("0.3.0"); r["model"]["nodes"].as_array_mut().unwrap().push(json!({"id": "x", "position": {"x": 0, "y": 0, "z": 0}})) })),
        Some(AdmissionRefusal::Family(D1Clause::Namespace, FamilyFact::SchemaVersion)));
    assert!(is_cap(dom(with(capmax, |r| { prov("{")(r); r["model"]["nodes"].as_array_mut().unwrap().push(json!({"id": "x", "position": {"x": 0, "y": 0, "z": 0}})) })), K::Nodes, 33, 32));
    assert_eq!(dom(with(capmax, |r| { prov("{")(r); mdl(r).insert("c\u{2}".into(), json!(0)); })), obj);
    let deep = |r: &mut Value| { let mut v = json!(0); for _ in 0..80 { v = json!([v]); } mdl(r).insert("deep".into(), v); };
    assert_eq!(dom(with(capmax, |r| { deep(r); r["model"]["schema_version"] = json!("9") })), Some(AdmissionRefusal::Census(CensusPart::Raw, CensusStatus::DepthLimit)));
    // Exact family strings and the namespace edges.
    assert_eq!(dom(with(capmax, |r| r["model"]["supports"][0]["family"] = json!("spring"))), None);
    assert_eq!(dom(with(capmax, |r| r["model"]["supports"][0]["family"] = json!("Spring"))), Some(AdmissionRefusal::Family(D1Clause::Supports, FamilyFact::SupportFamily)));
    assert_eq!(dom(with(capmax, |r| r["model"]["schema_version"] = json!("0.2.0"))), None);
    assert_eq!(dom(with(capmax, |r| r["model"]["schema_version"] = json!("0.1.0 "))), Some(AdmissionRefusal::Family(D1Clause::Namespace, FamilyFact::SchemaVersion)));
    // Headless: D1.0 first, whatever else fails.
    let raw = with(capmax, |r| { prov("{")(r); r["model"]["schema_version"] = json!("9") });
    let (request, capture) = parse(raw.clone(), PreviewSolverMode::DenseScrutiny);
    let inv = json!({"request": raw, "solver_mode": "dense_scrutiny"});
    let id = String::from("rv89");
    let h = assess(&capture, &request, Entry::Headless(RetainedHeadlessContext::from_borrowed_roots(&raw, &inv, &id)));
    assert_eq!(h.law().refusal, Some(AdmissionRefusal::Caller(RetainedCaller::Headless)));
    assert_eq!(h.law().refusal.unwrap().precondition(), UnavailablePrecondition::Caller);
    assert_eq!(h.law().domain, Some(AdmissionRefusal::Family(D1Clause::Namespace, FamilyFact::SchemaVersion)));
}

// ---- The bound -------------------------------------------------------------------
#[test]
fn rv89_bound_m_minus_one_m_m_plus_one() {
    const M: u64 = 4_026_531_840;
    let r = RESERVED_STACK_BYTES as u64;
    for (e, expect) in [(M - r - 1, Ok(M - 1)), (M - r, Ok(M)), (M - r + 1, Err(BoundRefusal::Exceeds { required: M + 1, threshold: M }))] {
        assert_eq!(bound_admits(e, r, M), expect);
    }
    assert_eq!(bound_admits(u64::MAX - r + 1, r, u64::MAX), Err(BoundRefusal::Overflow));
    assert_eq!(bound_admits(0, 0, 0), Ok(0));
    assert_eq!(bound_admits(1, 0, 0), Err(BoundRefusal::Exceeds { required: 1, threshold: 0 }));
    for mode in MODES {
        assert_eq!(cap_priced_maximum(mode), Err(BoundRefusal::Unpriced));
    }
    // law_order with a registered index and an in-domain request still refuses: Unpriced.
    let v = law_order(RetainedCaller::Direct, Ok(0), Ok(()), |_| bound_admits(cap_priced_maximum(PreviewSolverMode::DenseScrutiny)?, r, M));
    assert_eq!(v, Err(AdmissionRefusal::Bound(BoundRefusal::Unpriced)));
}

// ---- Allocation-free G-A and gates ---------------------------------------------------
#[test]
fn rv89_admission_and_gate_reads_allocate_nothing() {
    let inputs = [milestone(), capmax(), with(capmax, |r| r["model"]["schema_version"] = json!("0.3.0")),
        with(capmax, |r| r["model"]["load_cases"][0]["primitive_loads"][3]["provenance"] = json!(" {")),
        with(capmax, |r| { mdl(r).insert("x\u{7f}".into(), json!(1)); }),
        with(milestone, |r| { let mut v = json!(0); for _ in 0..80 { v = json!([v]); } mdl(r).insert("deep".into(), v); })];
    for raw in inputs {
        for mode in MODES {
            let (request, capture) = parse(raw.clone(), mode);
            let inv = json!({"request": raw.clone(), "solver_mode": mode.as_str()});
            let id = String::from("rv89");
            let (n, r) = allocations(|| assess(&capture, &request, Entry::Direct));
            assert_eq!(n, 0, "Direct G-A allocated");
            assert!(r.law().refusal.is_some());
            let (n, _) = allocations(|| assess(&capture, &request, Entry::Headless(RetainedHeadlessContext::from_borrowed_roots(&raw, &inv, &id))));
            assert_eq!(n, 0, "Headless G-A allocated");
            let (n, _) = allocations(|| (raw_text_census(capture.borrowed_raw()), nested_typed_census(&request), borrowed_request_census(&request)));
            assert_eq!(n, 0, "census extensions allocated");
        }
    }
    // The gate readers on the milestone's actual owners, in both modes.
    for mode in MODES {
        let (request, capture) = parse(milestone(), mode);
        let mut diagnostics = Vec::new();
        let built = crate::build_model(&request.model, &request.model.materials, &mut diagnostics).unwrap();
        let model = request.model.clone();
        let mut observer = crate::retained_product::ProductCapture::prepared_probe();
        let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
        let restrained = vec![0usize, 1, 2];
        let springs: Vec<crate::SpringEntry> = Vec::new();
        let late = LateFacts { model: &model, built: &built, materials: &model.materials, case: &model.load_cases[0], restrained: &restrained, springs: &springs, capture: &observer };
        let (n, l) = allocations(|| check_phase(PhaseGate::Late, &late_observations(&late), &phase_caps().late));
        assert_eq!(n, 0, "G-B allocated");
        let complete = CompleteFacts { ordinary: &ordinary, capture: &observer };
        let (n, c) = allocations(|| check_phase(PhaseGate::Complete, &complete_observations(&complete), &phase_caps().complete));
        assert_eq!(n, 0, "G-C allocated");
        // Part 1: the unpriced bounds are zero, so both gates refuse, with the gate, fact, value and cap.
        let l = l.unwrap_err();
        assert_eq!((l.gate, l.fact, l.cap), (PhaseGate::Late, PhaseFact::LateObservationBytes, 0));
        assert!(l.observed > 0);
        let c = c.unwrap_err();
        assert_eq!((c.gate, c.fact, c.cap), (PhaseGate::Complete, PhaseFact::EnvelopeResultCapacity, 0));
        assert_eq!(c.observed, ordinary.results.capacity() as u64);
        println!("RV89_GATES {mode:?} late={l:?} complete={c:?}");
    }
}

#[test]
fn rv89_g_c_longest_string_and_diagnostic_id_bounds() {
    let (request, capture) = parse(milestone(), PreviewSolverMode::SparseInteractive);
    let mut observer = crate::retained_product::ProductCapture::prepared_probe();
    let ordinary = crate::run_linear_static_preview_observed(request, PreviewSolverMode::SparseInteractive, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
    let mut caps = phase_caps().complete;
    for c in caps.iter_mut() {
        if *c == UNPRICED { *c = u64::MAX; }
    }
    caps[5] = u64::MAX; // diagnostic text bytes: not under test here
    let idx = |f: PhaseFact| complete_observations(&CompleteFacts { ordinary: &ordinary, capture: &observer }).iter().position(|o| o.fact == f).unwrap();
    let (s, d) = (idx(PhaseFact::EnvelopeMaxStringBytes), idx(PhaseFact::DiagnosticIdMaxBytes));
    assert_eq!((caps[s], caps[d]), (2_599_962, 2_330));
    let check = |e: &crate::MechanicsEnvelope| check_phase(PhaseGate::Complete, &complete_observations(&CompleteFacts { ordinary: e, capture: &observer }), &caps);
    assert_eq!(check(&ordinary), Ok(()));
    for (len, ok) in [(2_599_962usize, true), (2_599_963, false)] {
        let mut e = ordinary.clone();
        e.diagnostics[0].message = "m".repeat(len);
        let r = check(&e);
        assert_eq!(r.is_ok(), ok, "message {len}: {r:?}");
        if !ok { assert_eq!(r.unwrap_err(), PhaseRefusal { gate: PhaseGate::Complete, fact: PhaseFact::EnvelopeMaxStringBytes, observed: 2_599_963, cap: 2_599_962 }); }
        // The same length in a result row's entity_ref, and in the preview tree.
        let mut e = ordinary.clone();
        e.results[0].entity_ref = "r".repeat(len);
        assert_eq!(check(&e).is_ok(), ok, "row {len}");
    }
    for (len, ok) in [(2_330usize, true), (2_331, false)] {
        let mut e = ordinary.clone();
        e.diagnostics[0].id = "d".repeat(len);
        let r = check(&e);
        assert_eq!(r.is_ok(), ok, "diagnostic id {len}: {r:?}");
        if !ok { assert_eq!(r.unwrap_err(), PhaseRefusal { gate: PhaseGate::Complete, fact: PhaseFact::DiagnosticIdMaxBytes, observed: 2_331, cap: 2_330 }); }
    }
}

#[test]
fn rv89_counter_positive_control() {
    let (n, v) = allocations(|| Vec::<u8>::with_capacity(8));
    assert_eq!(n, 1, "the counting allocator sees an allocation");
    drop(v);
    let (n, s) = allocations(|| serde_json::to_string(&json!({"a": 1})).unwrap());
    assert!(n >= 1 && !s.is_empty());
}

#[test]
fn rv89_survivor_witnesses_d1_3_any_material_saturation_and_affected_refs() {
    // D1.3: one authored expansion law among several materials refuses (any, not all).
    let r = dom(with(capmax, |r| r["model"]["materials"][2]["expansion_laws"] = Value::Null));
    assert_eq!(r, Some(AdmissionRefusal::Family(D1Clause::Namespace, FamilyFact::MaterialExpansionLaw)));
    let r = dom(with(capmax, |r| r["materials"][1]["expansion_laws"] = json!([])));
    assert_eq!(r, Some(AdmissionRefusal::Family(D1Clause::Namespace, FamilyFact::RequestExpansionLaws)));
    // Gate sums saturate above every bound instead of wrapping or reading zero.
    assert_eq!(Bytes::times(usize::MAX, 2).get(), u64::MAX);
    assert_eq!(Bytes(Some(usize::MAX)).add(1).get(), u64::MAX);
    assert_eq!(Bytes::ZERO.plus(Bytes::times(usize::MAX, 3)).get(), u64::MAX);
    // G-C's longest string includes a diagnostic's affected refs and source.
    let (request, capture) = parse(milestone(), PreviewSolverMode::SparseInteractive);
    let mut observer = crate::retained_product::ProductCapture::prepared_probe();
    let mut e = crate::run_linear_static_preview_observed(request, PreviewSolverMode::SparseInteractive, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
    e.diagnostics[0].affected_refs.push("a".repeat(20_000));
    let o = complete_observations(&CompleteFacts { ordinary: &e, capture: &observer });
    assert_eq!(o.iter().find(|x| x.fact == PhaseFact::EnvelopeMaxStringBytes).unwrap().observed, 20_000);
    e.diagnostics[0].source = Some("s".repeat(30_000));
    let o = complete_observations(&CompleteFacts { ordinary: &e, capture: &observer });
    assert_eq!(o.iter().find(|x| x.fact == PhaseFact::EnvelopeMaxStringBytes).unwrap().observed, 30_000);
}
