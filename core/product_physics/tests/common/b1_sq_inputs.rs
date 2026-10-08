//! B1 SQ (PLAN_v2 §3.4–§3.6): the cap-maximal qualification inputs, shared by the S1 witnesses
//! (`src/retained_memory_witness_tests.rs`, through `#[path]`) and the allocation challenge
//! (`tests/retained_memory_challenge.rs`). Test-only; the product never reads this file.
//!
//! Each builder transcribes I86's input generator (R/I86/b1_w_probe_01/_run_records/gen_inputs.py)
//! or a committed test helper, and its result is pinned by the sha256 of `serde_json::to_vec` of
//! the Value, the `input_sha` I86's probe logs record (RR "I86's SW probe accepted; …", rulings 1
//! and 2):
//! - `b2_k1e3()`: W2b's replacement, W2b's input with S0 an anchor restraining UX, UY, UZ, RY, RZ
//!   and S31 an RX spring of 1000 N·m/rad at N0 (Sensitive by its report; `Fallback("Candidate")`);
//! - `c1()`: the c = 1 cap-maximal input that publishes (7 milestone copies and 4 filler bodies);
//! - `i3_case(…)` and `i3_three_case()`: the three case components on c1's model, each stressed
//!   (every provenance escaped, a raw value of depth 16), and their three-case assembly (|A| = 3);
//! - `w_c2()`: W-C2 (R/I81/b1_probe_01 PROBE §4), the facade tests' `w_c2()`.
#![allow(dead_code)]
use serde_json::{json, Value};

pub const MILESTONE: &str = include_str!("../../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json");
const PROV: &str = "invented_t3_b1_sw_probe_input_no_library_data";

/// I86's `input_sha` pins (sha256 of `serde_json::to_vec(&value)`).
pub const W2B_SHA256: &str = "d74d01ce1bc33244796877890ad134dd8476beb059bd82f26f2e004f8af619cb";
pub const B2_K1E3_SHA256: &str = "1ea4a168b40fd0c5a35c056c93c29b8b562eb739b7a5a09c1f888271f39d67a0";
pub const C1_SHA256: &str = "c5d2e93240b421092550097f8d1926d75be864df997a37a1901fe25a106388ed";
pub const I3_CASE_SHA256: [(&str, &str); 3] = [
    ("case:a", "8756006a612b6ed66c1dd01b2af228b339b6b51cd43a6fd416638c6f87c215b0"),
    ("case:b", "c533f5674753a880f6a7b83839b4d1957d32834570133ea27fd5821ddfa65e24"),
    ("case:c", "b4a22b37f452dfd12449fac41ef86fb4d13ad0dbb23cac0d4e321dd9a38092fb"),
];
pub const I3_THREE_CASE_SHA256: &str = "2ca29f02fc1bc96d0b30133951516bbe59abd5495b4a4e7cebde479bcbc4c508";

pub fn milestone() -> Value {
    serde_json::from_str(MILESTONE).unwrap()
}
fn text(prefix: &str, len: usize) -> String {
    let mut s = String::from(prefix);
    while s.len() < len {
        s.push('x');
    }
    s.truncate(len);
    s
}

/// `law_tests::CAP_MAXIMAL_RING`, transcribed: the ring's (x, y) as binary64 bits, because `cos`
/// and `sin` differ in the last bit between platform libms (see there).
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

/// `law_tests::cap_maximal`, transcribed (a lib test asserts the two are equal): 32 nodes, a
/// 32-member ring, 32 supports with 6 restraints and a scalar spring each, 128 nodal loads,
/// 4 + 4 materials with 16 temperature points, a 128-byte project id.
pub fn law_cap_maximal() -> Value {
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
/// W2b's input (`witness_tests::w2b_input`): `law_cap_maximal` with the milestone's support shapes.
pub fn w2b() -> Value {
    let mut raw = law_cap_maximal();
    for (i, support) in raw["model"]["supports"].as_array_mut().unwrap().iter_mut().enumerate() {
        if i == 0 {
            support.as_object_mut().unwrap().remove("stiffness");
        } else {
            support["family"] = json!("spring");
            support["restraints"] = json!(["UY"]);
        }
    }
    raw
}
/// W2b's replacement (gen_inputs.py `variant_b(w2b, 1000.0, anchor=True)`).
pub fn b2_k1e3() -> Value {
    let mut raw = w2b();
    let supports = raw["model"]["supports"].as_array_mut().unwrap();
    assert_eq!(supports[0]["node"], "N0");
    supports[0]["restraints"] = json!(["UX", "UY", "UZ", "RY", "RZ"]);
    supports[0]["family"] = json!("anchor");
    assert_eq!((&supports[31]["node"], &supports[31]["family"]), (&json!("N31"), &json!("spring")));
    let provenance = supports[31]["provenance"].clone();
    supports[31] = json!({"id": "S31", "node": "N0", "family": "spring", "restraints": ["RX"],
        "stiffness": {"dof": "RX", "value": {"value": 1000.0, "unit": "N*m/rad"}}, "provenance": provenance});
    raw
}

/// gen_inputs.py `split_exact`: `v` split into parts proportional to `weights`, each an integer
/// multiple of 2^E (v = M·2^E, |M| < 2^53), whose integer sum is M, so every partial sum is exact.
fn split_exact(v: f64, weights: &[u64]) -> Vec<f64> {
    let bits = v.to_bits();
    let exponent = ((bits >> 52) & 0x7ff) as i32;
    assert!(exponent != 0 && exponent != 0x7ff, "a normal value");
    let m = (bits & ((1u64 << 52) - 1)) | (1u64 << 52);
    let e = exponent - 1075;
    let sign = if bits >> 63 == 1 { -1.0 } else { 1.0 };
    let total: u64 = weights.iter().sum();
    let mut ints: Vec<u64> = weights.iter().map(|w| ((m as u128 * *w as u128) / total as u128) as u64).collect();
    let sum: u64 = ints.iter().sum();
    *ints.last_mut().unwrap() += m - sum;
    assert!(ints.iter().all(|i| *i > 0));
    let scale = 2f64.powi(e);
    let parts: Vec<f64> = ints.iter().map(|i| sign * (*i as f64 * scale)).collect();
    assert_eq!(parts.iter().fold(0.0, |a, p| a + p), v, "exact in order");
    assert_eq!(parts.iter().rev().fold(0.0, |a, p| a + p), v, "exact in reverse");
    parts
}
fn perpendicular_reference(d: [f64; 3]) -> Value {
    for r in [[0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] {
        let c = [d[1] * r[2] - d[2] * r[1], d[2] * r[0] - d[0] * r[2], d[0] * r[1] - d[1] * r[0]];
        let n = (c[0] * c[0] + c[1] * c[1] + c[2] * c[2]).sqrt();
        if n > 0.5 * (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() {
            return json!({"x": r[0], "y": r[1], "z": r[2]});
        }
    }
    unreachable!("a member direction")
}
/// gen_inputs.py `filler_bodies`: four anchored, connected, unloaded bodies (18 nodes, 25 members,
/// 4 supports), each first node fully restrained with no family.
fn filler_bodies() -> (Vec<Value>, Vec<Value>, Vec<Value>) {
    let shapes: [&[[f64; 3]]; 4] = [
        &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 1.0, 1.0]],
        &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.5, 0.5, 1.0]],
        &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 1.0]],
    ];
    let complete = |n: usize| (0..n).flat_map(|i| (i + 1..n).map(move |j| (i, j))).collect::<Vec<_>>();
    let edges = [complete(5), vec![(0, 1), (1, 2), (2, 3), (3, 0), (0, 4), (2, 4)], complete(4), vec![(0, 1), (1, 2), (2, 3)]];
    let (mut nodes, mut pipes, mut supports) = (Vec::new(), Vec::new(), Vec::new());
    for (b, (shape, es)) in shapes.iter().zip(edges.iter()).enumerate() {
        let origin = [100.0 + 10.0 * b as f64, 50.0, 0.0];
        let ids: Vec<String> = (0..shape.len()).map(|i| format!("F{b}:N{i}")).collect();
        for (i, p) in shape.iter().enumerate() {
            nodes.push(json!({"id": ids[i], "position": {"x": origin[0] + p[0], "y": origin[1] + p[1], "z": origin[2] + p[2]}, "provenance": PROV}));
        }
        for &(i, j) in es {
            let d = [shape[j][0] - shape[i][0], shape[j][1] - shape[i][1], shape[j][2] - shape[i][2]];
            pipes.push(json!({"id": format!("F{b}:M{i}{j}"), "from": ids[i], "to": ids[j], "material": "mat:1", "y_reference": perpendicular_reference(d),
                "section": {"outside_diameter": {"value": 0.2, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"}}, "provenance": PROV}));
        }
        supports.push(json!({"id": format!("F{b}:anchor"), "node": ids[0], "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV}));
    }
    assert_eq!((nodes.len(), pipes.len(), supports.len()), (18, 25, 4));
    (nodes, pipes, supports)
}
fn materials(points: usize) -> Vec<Value> {
    let pts: Vec<Value> = (0..points).map(|i| json!({"id": format!("T{i}"), "provenance": PROV})).collect();
    (0..4)
        .map(|i| {
            let mut m = json!({"id": format!("mat:{i}"), "elastic_modulus": {"value": 200000000000.0, "unit": "Pa"},
                "shear_modulus": {"value": 80000000000.0, "unit": "Pa"}, "provenance": PROV});
            if points > 0 {
                m["temperature_points"] = json!(pts);
            }
            m
        })
        .collect()
}
/// gen_inputs.py `copies_loads`: `total` moments over `copies` milestone copies (three rotational
/// DOFs at each copy's N1); copy k's net on each DOF is exactly `scale(k)` times the milestone's.
fn copies_loads(ms: &Value, prefix: &str, copies: usize, total: usize, scale: impl Fn(usize) -> f64, weights: impl Fn(usize) -> Vec<u64>) -> Vec<Value> {
    let base = |d: &str| {
        ms["model"]["load_cases"][0]["primitive_loads"].as_array().unwrap().iter()
            .find(|l| l["direction"] == d).unwrap()["magnitude"]["value"].as_f64().unwrap()
    };
    let slots: Vec<(usize, &str)> = (0..copies).flat_map(|k| ["RX", "RY", "RZ"].map(|d| (k, d))).collect();
    let mut per = vec![total / slots.len(); slots.len()];
    for p in per.iter_mut().take(total - (total / slots.len()) * slots.len()) {
        *p += 1;
    }
    let mut out = Vec::new();
    for (&(k, d), &n) in slots.iter().zip(&per) {
        let net = base(d) * scale(k);
        for (j, part) in split_exact(net, &weights(n)).into_iter().enumerate() {
            out.push(json!({"id": format!("{prefix}C{k}:{d}:{j}"), "category": "concentrated_moment", "target": {"type": "node", "node": format!("C{k}:N1")},
                "direction": d, "magnitude": {"value": part, "unit": "N*m"}, "dimension": "moment", "provenance": PROV}));
        }
    }
    assert_eq!(out.len(), total);
    out
}
/// gen_inputs.py `build_model`: the milestone body replicated `copies` times along X, plus the
/// filler, at D1's count caps (32 nodes, members and supports).
fn build_model(ms: &Value, copies: usize, points: usize, cases: Vec<Value>) -> Value {
    let m = &ms["model"];
    let (mut nodes, mut pipes, mut supports) = (Vec::new(), Vec::new(), Vec::new());
    for k in 0..copies {
        let dx = 10.0 * k as f64;
        for n in m["nodes"].as_array().unwrap() {
            let p = &n["position"];
            nodes.push(json!({"id": format!("C{k}:{}", n["id"].as_str().unwrap()),
                "position": {"x": p["x"].as_f64().unwrap() + dx, "y": p["y"], "z": p["z"]}, "provenance": n["provenance"]}));
        }
        for pipe in m["pipe_segments"].as_array().unwrap() {
            let mut q = pipe.clone();
            q["id"] = json!(format!("C{k}:{}", pipe["id"].as_str().unwrap()));
            q["from"] = json!(format!("C{k}:{}", pipe["from"].as_str().unwrap()));
            q["to"] = json!(format!("C{k}:{}", pipe["to"].as_str().unwrap()));
            q["material"] = json!("mat:0");
            pipes.push(q);
        }
        for s in m["supports"].as_array().unwrap() {
            let mut q = s.clone();
            q["id"] = json!(format!("C{k}:{}", s["id"].as_str().unwrap()));
            q["node"] = json!(format!("C{k}:{}", s["node"].as_str().unwrap()));
            supports.push(q);
        }
    }
    let (fnodes, fpipes, fsupports) = filler_bodies();
    nodes.extend(fnodes);
    pipes.extend(fpipes);
    supports.extend(fsupports);
    assert_eq!((nodes.len(), pipes.len(), supports.len()), (32, 32, 32));
    json!({"model": {
        "schema_version": m["schema_version"], "document_kind": m["document_kind"], "analysis_status": m["analysis_status"],
        "project": {"id": text("invented:t3-b1-sw:", 128), "units": m["project"]["units"]},
        "nodes": nodes, "pipe_segments": pipes, "materials": materials(points), "supports": supports,
        "load_cases": cases, "combinations": []}, "materials": materials(points)})
}
fn one_case(id: &str, loads: Vec<Value>) -> Value {
    json!({"id": id, "label": "I86 B1-SW probe case", "kind": "primitive_user_load", "primitive_loads": loads, "provenance": PROV})
}
/// The c = 1 cap-maximal input that publishes (gen_inputs.py `build_c("c1")`).
pub fn c1() -> Value {
    let ms = milestone();
    let loads = copies_loads(&ms, "", 7, 128, |_| 1.0, |n| vec![1; n]);
    build_model(&ms, 7, 16, vec![one_case("case", loads)])
}
/// A quote and a backslash appended to every provenance (maximal escaping under D1.11).
pub fn escape_every_provenance(v: &mut Value) {
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
/// A raw value of depth 16 at the root (D1.2's cap).
pub fn depth_16_value() -> Value {
    let mut deep = json!(1);
    for _ in 0..14 {
        deep = json!([deep]);
    }
    deep
}
fn stressed(mut raw: Value) -> Value {
    escape_every_provenance(&mut raw);
    raw["model"]["unknown_depth_witness"] = depth_16_value();
    raw
}
/// gen_inputs.py `i3_sets`: three 128-moment sets on c1's model. A: each copy's net the
/// milestone's, equal parts; B: minus it, parts weighted 1..n; C: copy k's net 2^(k−3) times it,
/// parts weighted n+1..2.
fn i3_sets(ms: &Value) -> [(&'static str, Vec<Value>); 3] {
    [
        ("case:a", copies_loads(ms, "a:", 7, 128, |_| 1.0, |n| vec![1; n])),
        ("case:b", copies_loads(ms, "b:", 7, 128, |_| -1.0, |n| (1..=n as u64).collect())),
        ("case:c", copies_loads(ms, "c:", 7, 128, |k| 2f64.powi(k as i32 - 3), |n| (0..n as u64).map(|i| n as u64 - i + 1).collect())),
    ]
}
/// One stressed component, alone (`i3_c1_case_{a,b,c}`).
pub fn i3_case(id: &str) -> Value {
    let ms = milestone();
    let (cid, loads) = i3_sets(&ms).into_iter().find(|(cid, _)| *cid == id).unwrap();
    stressed(build_model(&ms, 7, 16, vec![one_case(cid, loads)]))
}
/// The stressed three-case assembly (`i3_c1_three_case`): A, B and C, every case in A.
pub fn i3_three_case() -> Value {
    let ms = milestone();
    let cases = i3_sets(&ms).into_iter().map(|(cid, loads)| one_case(cid, loads)).collect();
    stressed(build_model(&ms, 7, 16, cases))
}

/// W-C2 (the facade tests' `w_c2()`, transcribed; a lib test asserts the two are equal): the
/// milestone plus PHYS-R4's cantilever as body 1, with case A (the milestone's moments), case B
/// (the tip force and torque) and case C (A's loads then B's, ids suffixed `:c`).
pub fn w_c2() -> Value {
    let mut raw = milestone();
    let p = "invented_t3_g5_witness_input_no_library_data";
    let model = &mut raw["model"];
    model["nodes"].as_array_mut().unwrap().extend([
        json!({"id": "node:section-a", "position": {"x": 5.0, "y": 0.0, "z": 0.0}, "provenance": p}),
        json!({"id": "node:section-b", "position": {"x": 6.0, "y": 0.0, "z": 0.0}, "provenance": p}),
    ]);
    model["pipe_segments"].as_array_mut().unwrap().push(json!({"id": "pipe:source-section", "from": "node:section-a", "to": "node:section-b",
        "material": "material:section", "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
        "section": {"outside_diameter": {"value": 4e-77, "unit": "m"}, "wall_thickness": {"value": 1e-77, "unit": "m"}}, "provenance": p}));
    model["materials"].as_array_mut().unwrap().push(json!({"id": "material:section", "elastic_modulus": {"value": 1.0, "unit": "Pa"},
        "shear_modulus": {"value": 0.4545, "unit": "Pa"}, "provenance": p}));
    model["supports"].as_array_mut().unwrap().push(json!({"id": "support:section-a", "node": "node:section-a", "family": "anchor",
        "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": p}));
    let template = model["load_cases"][0].clone();
    let loads_a = template["primitive_loads"].clone();
    let tip = f64::from_bits(0x0031fa182c40c60d);
    let loads_b = json!([
        {"id": "load:tip-y", "category": "concentrated_force", "target": {"type": "node", "node": "node:section-b"}, "direction": "global_y",
            "dimension": "force", "magnitude": {"value": tip, "unit": "N"}, "provenance": p},
        {"id": "load:tip-torque", "category": "concentrated_moment", "target": {"type": "node", "node": "node:section-b"}, "direction": "rotation_x",
            "dimension": "moment", "magnitude": {"value": tip, "unit": "N*m"}, "provenance": p}]);
    let mut loads_c: Vec<Value> = loads_a.as_array().unwrap().iter().chain(loads_b.as_array().unwrap()).cloned().collect();
    for load in &mut loads_c {
        load["id"] = json!(format!("{}:c", load["id"].as_str().unwrap()));
    }
    let case = |id: &str, loads: Value| {
        let mut case = template.clone();
        case["id"] = json!(id);
        case["primitive_loads"] = loads;
        case
    };
    model["load_cases"] = json!([case("case-a", loads_a), case("case-b", loads_b), case("case-c", json!(loads_c))]);
    raw
}
/// W-C2's (A, C): W-C2 without case B (RV109 round 2 N-3: it publishes in sparse).
pub fn w_c2_ac() -> Value {
    let mut raw = w_c2();
    let cases = raw["model"]["load_cases"].as_array_mut().unwrap();
    assert_eq!(cases[1]["id"], "case-b");
    cases.remove(1);
    raw
}
/// W2's input (`witness_w2_cap_maximal`): `law_cap_maximal`, every provenance escaped, with a raw
/// value of depth 16; its support shapes stop it at preparation.
pub fn w2() -> Value {
    stressed(law_cap_maximal())
}
