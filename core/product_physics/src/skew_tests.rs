//! §10.5 at the product (DEL-04-03 Work `skew-restraints.md` §10.5, §15 step 7): on the
//! references' guided cantilever, every skewed record's certified K-D5 record and every
//! axis record at its node are available (no recovery failure), the skewed record
//! carries its along-direction group, and the certified rows enclose the references'
//! exact values. Model inputs restate `tests/skew_restraints_references.rs`.
use crate::tests::KD5_RECOVERY_PROBE;
use crate::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const PROV: &str = "skew_restraints invented benchmark input; not library, component, material-catalog or code data";

fn qv(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

/// `(id, family, token, direction)` records at B (a zero direction: none); a spring
/// has k = 2^20 N/m.
fn document(records: &[(&str, &str, &str, [f64; 3])]) -> Value {
    let mut supports = vec![
        json!({"id": "anchor_A", "node": "A", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV}),
        json!({"id": "rot_B", "node": "B", "family": "anchor", "restraints": ["RX", "RY", "RZ"], "provenance": PROV}),
    ];
    for &(id, family, token, d) in records {
        let mut s = json!({"id": id, "node": "B", "family": family, "restraints": [token], "provenance": PROV});
        if d != [0.0; 3] {
            s["direction"] = json!({"x": d[0], "y": d[1], "z": d[2]});
        }
        if family == "spring" {
            s["stiffness"] = json!({"dof": token, "value": qv(1_048_576.0, "N/m")});
        }
        supports.push(s);
    }
    json!({"model": {"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:skew", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": [{"id": "A", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": PROV}, {"id": "B", "position": {"x": 4.0, "y": 0.0, "z": 0.0}, "provenance": PROV}],
        "pipe_segments": [{"id": "pipe:AB", "from": "A", "to": "B", "section": {"outside_diameter": qv(0.206, "m"), "wall_thickness": qv(0.011, "m")},
            "material": "material:skew", "y_reference": {"x": 0.0, "y": 0.0, "z": 1.0}, "provenance": PROV}],
        "supports": supports, "components": [],
        "materials": [{"id": "material:skew", "elastic_modulus": qv(171_798_691_840.0, "Pa"), "shear_modulus": qv(68_719_476_736.0, "Pa"), "provenance": PROV}],
        "load_cases": [{"id": "case:skew", "primitive_loads": [{"id": "load:y", "category": "concentrated_force", "target": {"type": "node", "node": "B"},
            "direction": "global_y", "magnitude": qv(-1024.0, "N"), "dimension": "force", "provenance": PROV}], "provenance": PROV}], "combinations": []},
        "materials": []})
}

#[test]
fn skewed_records_publish_available_certified_records() {
    let tiny = 2.0_f64.powi(-600);
    let cases: [(&str, Vec<(&str, &str, &str, [f64; 3])>); 5] = [
        ("P_R1_stop", vec![("stop_B", "line_stop", "UY", [3.0, 4.0, 0.0])]),
        ("P_S1_spring", vec![("spring_B", "spring", "UY", [3.0, 4.0, 0.0])]),
        ("P_RS_stop_and_spring", vec![("stop_B", "line_stop", "UY", [3.0, 4.0, 0.0]), ("spring_B", "spring", "UY", [0.0, 3.0, 4.0])]),
        ("P_R4_guide_plus_stop", vec![("guide_uz_B", "line_stop", "UZ", [0.0; 3]), ("stop_B", "line_stop", "UY", [3.0, 4.0, 5.0])]),
        ("P_R4_tiny_restricted_direction", vec![("ux_B", "line_stop", "UX", [0.0; 3]), ("uz_B", "line_stop", "UZ", [0.0; 3]), ("stop_B", "line_stop", "UY", [1.0, tiny, 0.0])]),
    ];
    for (name, records) in cases {
        let wire = document(&records);
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            KD5_RECOVERY_PROBE.with(|probe| *probe.borrow_mut() = Some(Vec::new()));
            let output = run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap();
            let probe = KD5_RECOVERY_PROBE.with(|probe| probe.borrow_mut().take().unwrap());
            assert_eq!(output.status.mechanics, "MECHANICS_SOLVED", "{name} {mode:?}: {:?}", output.diagnostics);
            for id in records.iter().map(|r| r.0).chain(["rot_B"]) {
                let record = probe.iter().find(|r| r.member == id).unwrap_or_else(|| panic!("{name} {mode:?}: no certified record for {id}"));
                for group in &record.groups {
                    assert!(group.bound.is_ok(), "{name} {mode:?}: {id} {}: {:?}", group.kind, group.bound);
                }
                let skewed = records.iter().any(|r| r.0 == id && r.3 != [0.0; 3]);
                let along = record.groups.iter().filter(|g| g.kind == "along_direction").count();
                assert_eq!(along, usize::from(skewed), "{name} {mode:?}: {id} along-direction groups");
                if skewed {
                    // The published along row is the certified value, within its bound.
                    let group = record.groups.iter().find(|g| g.kind == "along_direction").unwrap();
                    let row = output.results.iter()
                        .find(|r| r.entity_ref == id && r.kind == "support_reaction_along_direction_v1").unwrap();
                    assert_eq!(row.value.abs().to_bits(), group.q.to_bits(), "{name} {mode:?}: {id} published = certified");
                }
            }
        }
    }
}

/// RT-A3 condition 1 (§6.2): a coordinate-transform product below the split range (a
/// UZ guide and a stop (1, 2^-600, 0): k·q·q < 2^-969) is refused on the existing
/// `NumericallyUnresolved` arm with its exact reason, never the pivot-screen special
/// case (`append_short_bend_refusal`, keyed on `PIVOT_SCREEN_REASON`) and never the
/// `InvalidInput` backstop.
#[test]
fn a_split_range_transform_is_refused_on_the_numerically_unresolved_arm() {
    const REASON: &str = "restraint_basis: split range in the coordinate transform";
    assert_ne!(REASON, crate::PIVOT_SCREEN_REASON);
    let error = open_pipe_stress_frame_kernel::structural::StructuralError::NumericallyUnresolved { reason: REASON, global_dof: None };
    assert_eq!(crate::integrity_failure_code(&error), "NUMERICAL_INTEGRITY_UNRESOLVED");
    let wire = document(&[("guide_uz_B", "line_stop", "UZ", [0.0; 3]), ("stop_B", "line_stop", "UY", [1.0, 2.0_f64.powi(-600), 0.0])]);
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let output = run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap();
        let blocking: Vec<_> = output.diagnostics.iter().filter(|d| d.severity == "blocking").collect();
        assert_eq!(blocking.len(), 1, "{mode:?}: {blocking:?}");
        assert_eq!(blocking[0].code, "NUMERICAL_INTEGRITY_UNRESOLVED", "{mode:?}");
        assert!(blocking[0].message.contains(&format!("{error:?}")), "{mode:?}: {}", blocking[0].message);
        assert!(!output.diagnostics.iter().any(|d| d.code == crate::CURVED_BEND_SHORT), "{mode:?}");
        assert!(!blocking[0].message.contains("InvalidInput") && !blocking[0].message.contains(crate::PIVOT_SCREEN_REASON), "{mode:?}");
        assert!(output.results.is_empty(), "{mode:?}: nothing published");
    }
}

/// A skew-only restraint set (no axis restraint) with six ground rows that still
/// leaves the rigid rotation about X free: nodes A, M, B on the X axis, each with a
/// stop (3, 4, 0) and a spring (0, 3, 4).
pub(crate) fn skew_only_mechanism() -> Value {
    let mut wire = document(&[]);
    let model = &mut wire["model"];
    model["nodes"] = json!([{"id": "A", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": PROV},
        {"id": "M", "position": {"x": 2.0, "y": 0.0, "z": 0.0}, "provenance": PROV},
        {"id": "B", "position": {"x": 4.0, "y": 0.0, "z": 0.0}, "provenance": PROV}]);
    let pipe = |id: &str, from: &str, to: &str| json!({"id": id, "from": from, "to": to, "section": {"outside_diameter": qv(0.206, "m"), "wall_thickness": qv(0.011, "m")},
        "material": "material:skew", "y_reference": {"x": 0.0, "y": 0.0, "z": 1.0}, "provenance": PROV});
    model["pipe_segments"] = json!([pipe("pipe:AM", "A", "M"), pipe("pipe:MB", "M", "B")]);
    let mut supports = Vec::new();
    for node in ["A", "M", "B"] {
        supports.push(json!({"id": format!("stop_{node}"), "node": node, "family": "line_stop", "restraints": ["UY"], "direction": {"x": 3.0, "y": 4.0, "z": 0.0}, "provenance": PROV}));
        supports.push(json!({"id": format!("spring_{node}"), "node": node, "family": "spring", "restraints": ["UY"], "direction": {"x": 0.0, "y": 3.0, "z": 4.0},
            "stiffness": {"dof": "UY", "value": qv(1_048_576.0, "N/m")}, "provenance": PROV}));
    }
    model["supports"] = json!(supports);
    wire
}

/// RT-A3 condition 4: with `SUPPORT_INPUT_MISSING` no longer raised for a skew-only
/// restraint set, a skew-only model that is still a mechanism passes the six-row count
/// and is refused by name by the rigid-body ground screen (F-3: every axis and
/// direction row counts), witnessing the rigid rotation about X.
#[test]
fn a_skew_only_mechanism_is_refused_by_the_ground_screen() {
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        let output = run_linear_static_preview_value_with_mode(skew_only_mechanism(), mode).unwrap();
        let blocking: Vec<_> = output.diagnostics.iter().filter(|d| d.severity == "blocking").collect();
        assert_eq!(blocking.len(), 1, "{mode:?}: {blocking:?}");
        assert_eq!(blocking[0].code, "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM", "{mode:?}");
        let rx = "Mechanism { direction: [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0] }";
        assert!(blocking[0].message.contains(rx), "{mode:?}: {}", blocking[0].message);
        assert!(output.results.is_empty());
    }
}

// ------------------------------------------------------------------ Astra N5

const REFERENCES: &str = include_str!("../../../validation/references/skew_restraints/references.json");

/// The decimal integer `text` (no sign) as base-2^32 limbs, least significant first.
fn limbs(text: &str) -> Vec<u64> {
    let mut out: Vec<u64> = vec![0];
    for digit in text.bytes() {
        assert!(digit.is_ascii_digit(), "{text}");
        let mut carry = u64::from(digit - b'0');
        for limb in out.iter_mut() {
            let v = *limb * 10 + carry;
            *limb = v & 0xffff_ffff;
            carry = v >> 32;
        }
        if carry != 0 {
            out.push(carry);
        }
    }
    out
}

/// The decimal integer `text` as exact binary64 pairs (a, c) with a·c = limb·2^(32i): c = 1,
/// or 2^960 for a limb beyond binary64's exponent range.
fn big_terms(text: &str) -> Vec<(f64, f64)> {
    limbs(text)
        .iter()
        .enumerate()
        .map(|(i, &l)| {
            let shift = 32 * i as i32;
            if shift <= 960 { (l as f64 * 2f64.powi(shift), 1.0) } else { (l as f64 * 2f64.powi(shift - 960), 2f64.powi(960)) }
        })
        .collect()
}

/// Exactly whether |p − n/d| ≤ b for binary64 p, b ≥ 0 and the references' rational
/// `n/d` (d > 0, below 2^960): p·d − n − b·d ≤ 0 and n − p·d − b·d ≤ 0, each one exact sum
/// (`ExactAccumulator`).
fn encloses(p: f64, b: f64, exact: &str) -> bool {
    use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
    let (n, d) = exact.split_once('/').unwrap();
    let (negative, n) = n.strip_prefix('-').map_or((false, n), |rest| (true, rest));
    let (n, d) = (big_terms(n), big_terms(d));
    assert!(d.iter().all(|&(_, c)| c == 1.0), "denominator {exact}");
    let side = |sign: f64| {
        // sign·(p·d − n) − b·d
        let mut sum = ExactAccumulator::new();
        for &(di, _) in &d {
            sum.add_product(sign * p, di).unwrap();
            sum.add_product(-b, di).unwrap();
        }
        for &(a, c) in &n {
            sum.add_product(if negative { sign * a } else { -sign * a }, c).unwrap();
        }
        sum.signum() <= 0
    };
    b >= 0.0 && side(1.0) && side(-1.0)
}

/// Exactly whether |p − s| ≤ b for an irrational reference s = sign·√(n/d) (the references'
/// `sign` and `square`): with x = sign·p, (x − b)²·d ≤ n when x > b, and n ≤ (x + b)²·d.
/// Squares are expanded with error-free products (TwoProduct by `mul_add`) into one exact
/// sum. A common power-of-two shift 2^-k keeps x² and b² in binary64's range: x, b scale by
/// 2^-k and n by 2^-2k, applied as the two representable factors 2^-k·2^-k.
fn encloses_root(p: f64, b: f64, sign: i64, square: &str) -> bool {
    use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
    let (n, d) = square.split_once('/').unwrap();
    let (n, d) = (big_terms(n), big_terms(d));
    assert!(d.iter().all(|&(_, c)| c == 1.0), "denominator {square}");
    let x = if sign < 0 { -p } else { p };
    if b < 0.0 || x + b <= 0.0 {
        return false;
    }
    let k = (x.abs().log2().floor() as i32 - 400).max(0);
    let (x, b, scale) = (x * 2f64.powi(-k), b * 2f64.powi(-k), 2f64.powi(-k));
    let two_product = |a: f64, c: f64| {
        let h = a * c;
        let l = a.mul_add(c, -h);
        assert!(h.is_finite() && (h == 0.0 || h.abs() >= 2f64.powi(-960)), "two_product range");
        (h, l)
    };
    // sign_b·(x ± b)²·d − n, as one exact sum; true when it is ≤ 0 (lower) or ≥ 0 (upper).
    let side = |plus: bool| {
        let mut sum = ExactAccumulator::new();
        let cross = if plus { 2.0 } else { -2.0 };
        let (xx, xb, bb) = (two_product(x, x), two_product(x, b), two_product(b, b));
        for &(di, _) in &d {
            for (h, l, w) in [(xx.0, xx.1, 1.0), (xb.0, xb.1, cross), (bb.0, bb.1, 1.0)] {
                sum.add_product(w * h, di).unwrap();
                sum.add_product(w * l, di).unwrap();
            }
        }
        for &(a, c) in &n {
            sum.add_product(-(a * scale), c * scale).unwrap();
        }
        sum.signum()
    };
    (x <= b || side(false) <= 0) && side(true) >= 0
}

/// Astra N5: the K-D5 bounds the product itself forms contain the references' exact
/// values, on every S-K1 product case in both static modes:
/// - the lifted displacement at B: each published `certify_displacement` row (the value
///   `published_nodal_displacements` publishes) is within its own bound of the exact u_B;
/// - the attributed reactions: each published global component of a skewed record is
///   within its `support_force` group bound, and its along-direction value within the
///   `along_direction` group bound (the groups the recovery guard judges), of the exact
///   values.
/// The check is exact (rational containment); no tolerance is supplied.
#[test]
fn the_products_k_d5_bounds_contain_the_exact_references() {
    use crate::{PreviewSolverMode, LinearStaticPreviewRequest};
    use open_pipe_stress_frame_kernel::structural::certify_displacement;
    let refs: Value = serde_json::from_str(REFERENCES).unwrap();
    let case_of = |name: &str| refs["product"]["cases"].as_array().unwrap().iter().find(|c| c["name"] == name).unwrap().clone();
    let tiny = 2.0_f64.powi(-600);
    let cases: [(&str, Vec<(&str, &str, &str, [f64; 3])>); 5] = [
        ("P_R1_stop", vec![("stop_B", "line_stop", "UY", [3.0, 4.0, 0.0])]),
        ("P_S1_spring", vec![("spring_B", "spring", "UY", [3.0, 4.0, 0.0])]),
        ("P_RS_stop_and_spring", vec![("stop_B", "line_stop", "UY", [3.0, 4.0, 0.0]), ("spring_B", "spring", "UY", [0.0, 3.0, 4.0])]),
        ("P_R4_guide_plus_stop", vec![("guide_uz_B", "line_stop", "UZ", [0.0; 3]), ("stop_B", "line_stop", "UY", [3.0, 4.0, 5.0])]),
        ("P_R4_tiny_restricted_direction", vec![("ux_B", "line_stop", "UX", [0.0; 3]), ("uz_B", "line_stop", "UZ", [0.0; 3]), ("stop_B", "line_stop", "UY", [1.0, tiny, 0.0])]),
    ];
    let mut checked = 0;
    for (name, records) in cases {
        let reference = case_of(name);
        let expect = &reference["expect"];
        let mut wire = document(&records);
        // The reference case's own load at B (the document's is −1024 along Y).
        let load: Vec<f64> = reference["load_at_B"].as_array().unwrap().iter().map(|v| hex_value(v["binary64"].as_str().unwrap())).collect();
        wire["model"]["load_cases"][0]["primitive_loads"] = json!(["global_x", "global_y", "global_z"].iter().zip(&load).filter(|(_, v)| **v != 0.0)
            .map(|(direction, v)| json!({"id": format!("load:{direction}"), "category": "concentrated_force", "target": {"type": "node", "node": "B"},
                "direction": direction, "magnitude": qv(*v, "N"), "dimension": "force", "provenance": PROV})).collect::<Vec<_>>());
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            KD5_RECOVERY_PROBE.with(|probe| *probe.borrow_mut() = Some(Vec::new()));
            crate::skew::LIFT_PROBE.with(|probe| *probe.borrow_mut() = Some(Vec::new()));
            let request: LinearStaticPreviewRequest = serde_json::from_value(wire.clone()).unwrap();
            let output = crate::run_linear_static_preview_with_mode(request, mode);
            let probe = KD5_RECOVERY_PROBE.with(|probe| probe.borrow_mut().take().unwrap());
            let lifts = crate::skew::LIFT_PROBE.with(|probe| probe.borrow_mut().take().unwrap());
            let what = format!("{name} {mode:?}");
            assert_eq!(output.status.mechanics, "MECHANICS_SOLVED", "{what}: {:?}", output.diagnostics);
            // The lifted displacement at B (node 1: DOFs 6, 7, 8).
            assert_eq!(lifts.len(), 1, "{what}: one skew solve");
            let (u, bound) = &lifts[0];
            for (k, exact) in expect["u_B_translations"].as_array().unwrap().iter().enumerate() {
                let row = certify_displacement(6 + k, u, bound).unwrap_or_else(|e| panic!("{what}: u_B[{k}]: {e:?}"));
                let exact = exact["exact"].as_str().unwrap();
                assert!(encloses(row.published, row.bound(), exact), "{what}: u_B[{k}] {} ± {} does not contain {exact}", row.published, row.bound());
                checked += 1;
            }
            // The attributed reactions of each skewed record.
            for (id, record) in expect["records"].as_object().unwrap() {
                if !matches!(record["kind"].as_str(), Some("skew_stop" | "skew_spring")) {
                    continue;
                }
                let groups = &probe.iter().find(|r| &r.member == id).unwrap_or_else(|| panic!("{what}: {id}")).groups;
                let bound_of = |kind: &str| groups.iter().find(|g| g.kind == kind).unwrap().bound.clone().unwrap_or_else(|e| panic!("{what}: {id} {kind}: {e}"));
                let force_bound = bound_of("support_force");
                for (i, c) in ["Fx", "Fy", "Fz"].iter().enumerate() {
                    let row = output.results.iter().find(|r| &r.entity_ref == id && r.kind == "support_reaction_component_v2"
                        && r.metadata.as_ref().is_some_and(|m| m.component == *c)).unwrap();
                    let exact = record["global_force"][i]["exact"].as_str().unwrap();
                    assert!(encloses(row.value, force_bound, exact), "{what}: {id} {c} {} ± {force_bound} does not contain {exact}", row.value);
                    checked += 1;
                }
                let along = output.results.iter().find(|r| &r.entity_ref == id && r.kind == "support_reaction_along_direction_v1").unwrap();
                let along_bound = bound_of("along_direction");
                let reference = &record["along_direction"];
                match reference["exact"].as_str() {
                    Some(exact) => assert!(encloses(along.value, along_bound, exact), "{what}: {id} along {} ± {along_bound} does not contain {exact}", along.value),
                    // An irrational s = sign·√square, exactly.
                    None => {
                        let (sign, square) = (reference["sign"].as_i64().unwrap(), reference["square"].as_str().unwrap());
                        assert!(encloses_root(along.value, along_bound, sign, square), "{what}: {id} along {} ± {along_bound} does not contain √{square}", along.value);
                    }
                }
                checked += 1;
            }
        }
    }
    assert!(checked >= 60, "{checked}");
    // The containment check itself: exact at the boundary, refused just outside it.
    assert!(encloses(0.5, 0.0, "1/2") && encloses(0.25, 0.25, "1/2") && !encloses(0.25, 0.25f64.next_down(), "1/2"));
    assert!(encloses(-0.5, 0.0, "-1/2") && !encloses(0.5, 0.5f64.next_down(), "-1/2"));
    // √2 ∈ [1.4142135623730951 − 2^-52, +2^-52], not within 2^-60 of 1.5.
    assert!(encloses_root(std::f64::consts::SQRT_2, 2f64.powi(-52), 1, "2/1") && !encloses_root(1.5, 2f64.powi(-60), 1, "2/1"));
    assert!(encloses_root(-std::f64::consts::SQRT_2, 2f64.powi(-52), -1, "2/1") && !encloses_root(std::f64::consts::SQRT_2, 2f64.powi(-52), -1, "2/1"));
    // At 2^610 scale: √(2^1220) = 2^610 exactly.
    let big = format!("{}/1", (0..1).map(|_| "".to_string()).collect::<String>() + &pow2_decimal(1220));
    assert!(encloses_root(2f64.powi(610), 0.0, 1, &big) && !encloses_root(2f64.powi(610).next_up(), 0.0, 1, &big));
}

/// 2^e in decimal.
fn pow2_decimal(e: u32) -> String {
    let mut digits = vec![1u32];
    for _ in 0..e {
        let mut carry = 0;
        for d in digits.iter_mut() {
            let v = *d * 2 + carry;
            *d = v % 10;
            carry = v / 10;
        }
        if carry > 0 {
            digits.push(carry);
        }
    }
    digits.iter().rev().map(|d| char::from(b'0' + *d as u8)).collect()
}

fn hex_value(text: &str) -> f64 {
    let (negative, body) = text.strip_prefix('-').map_or((false, text), |rest| (true, rest));
    let (mantissa, exponent) = body.strip_prefix("0x").unwrap().split_once('p').unwrap();
    let (int, frac) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let mut m = u64::from_str_radix(int, 16).unwrap();
    for ch in frac.chars() {
        m = m * 16 + u64::from(ch.to_digit(16).unwrap());
    }
    let mut value = m as f64;
    let mut k: i32 = exponent.parse::<i32>().unwrap() - 4 * frac.len() as i32;
    while k != 0 {
        let step = k.clamp(-1000, 1000);
        value *= 2.0_f64.powi(step);
        k -= step;
    }
    if negative { -value } else { value }
}
