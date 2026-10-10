//! T4-U1: the objective curved element against T4-I6's frozen references
//! (`validation/references/t4_i6/u1_reference_cases.json`; U1_REFERENCE B2,
//! B3 and B5.4). The element is the product's `CurvedBendMacroElement`
//! formed from (x_i, x_j, R, y_reference) and the section constants.
//!
//! - P4: |ΔK_ij| ≤ 1e-9·max|K_ref| on every case at every X, and RV2 S-4's
//!   diagonal-scaled |ΔK_ij| ≤ 1e-9·√(K_ii K_jj) (normative here).
//! - P1: for each rigid motion of the actual nodes and each row,
//!   |(K r)_r| ≤ 1e-11·Σ_c |K_rc r_c|, both sides formed exactly.
//! - P2: the exact rotation (x, y, z) → (z, x, y) gives P K Pᵀ to 1e-12 of
//!   max|K_ref|; each generic-rotation copy meets P4 against its own K.
//! - P3: K is bit-identical at every X.
//! - P5: no refusal on any case, control or seeded elbow.
use open_pipe_stress_curved_bend::CurvedBendMacroElement;
use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
use open_pipe_stress_frame_kernel::{FrameNode, Matrix12};
use serde_json::Value;

const REFERENCES: &str =
    include_str!("../../../validation/references/t4_i6/u1_reference_cases.json");

const P4_CRITERION: f64 = 1e-9;
const P1_CRITERION: f64 = 1e-11;
const P2_CRITERION: f64 = 1e-12;

fn references() -> Value {
    serde_json::from_str(REFERENCES).expect("frozen reference JSON")
}

fn f(value: &Value) -> f64 {
    match value {
        Value::Number(n) => n.as_f64().unwrap(),
        Value::String(s) => s.parse::<f64>().unwrap(),
        other => panic!("not a number: {other}"),
    }
}

fn v3(value: &Value) -> [f64; 3] {
    let a = value.as_array().unwrap();
    [f(&a[0]), f(&a[1]), f(&a[2])]
}

fn matrix(value: &Value) -> [[f64; 12]; 12] {
    let mut out = [[0.0; 12]; 12];
    for (r, row) in value.as_array().unwrap().iter().enumerate() {
        for (c, entry) in row.as_array().unwrap().iter().enumerate() {
            out[r][c] = f(entry);
        }
    }
    out
}

struct Inputs {
    radius: f64,
    y: [f64; 3],
    e: f64,
    g: f64,
    a: f64,
    i: f64,
    j: f64,
    k_in: f64,
    k_out: f64,
}

fn element(xi: [f64; 3], xj: [f64; 3], p: &Inputs) -> CurvedBendMacroElement {
    CurvedBendMacroElement::new(
        FrameNode::new(0, xi).unwrap(),
        FrameNode::new(1, xj).unwrap(),
        p.radius,
        p.y,
        p.e,
        p.g,
        p.a,
        p.i,
        p.j,
        p.k_in,
        p.k_out,
    )
    .expect("P5: no refusal")
}

fn case_inputs(case: &Value) -> Inputs {
    let inputs = &case["inputs"];
    Inputs {
        radius: f(&inputs["R"]),
        y: v3(&inputs["y_reference"]),
        e: f(&inputs["E"]),
        g: f(&inputs["G"]),
        a: f(&inputs["A"]),
        i: f(&inputs["I"]),
        j: f(&inputs["J"]),
        k_in: f(&case["k_in"]),
        k_out: f(&case["k_out"]),
    }
}

fn max_abs(k: &[[f64; 12]; 12]) -> f64 {
    k.iter().flatten().fold(0.0_f64, |m, v| m.max(v.abs()))
}

/// (P4 matrix-scale error, diagonal-scaled error), both against the reference.
fn agreement(k: &Matrix12, reference: &[[f64; 12]; 12]) -> (f64, f64) {
    let scale = max_abs(reference);
    let (mut p4, mut diagonal) = (0.0_f64, 0.0_f64);
    for r in 0..12 {
        for c in 0..12 {
            let difference = (k[r][c] - reference[r][c]).abs();
            p4 = p4.max(difference / scale);
            let pair = (reference[r][r] * reference[c][c]).abs().sqrt();
            if pair > 0.0 {
                diagonal = diagonal.max(difference / pair);
            } else if difference > 0.0 {
                diagonal = f64::INFINITY;
            }
        }
    }
    (p4, diagonal)
}

/// The six rigid motions of the actual nodes: three translations, and
/// rotations about node i (node j swings by ω × d, formed exactly from d).
fn rigid_modes(d: [f64; 3]) -> Vec<[f64; 12]> {
    let mut modes = Vec::new();
    for axis in 0..3 {
        let mut m = [0.0; 12];
        m[axis] = 1.0;
        m[6 + axis] = 1.0;
        modes.push(m);
    }
    for axis in 0..3 {
        let mut omega = [0.0; 3];
        omega[axis] = 1.0;
        let swing = [
            omega[1] * d[2] - omega[2] * d[1],
            omega[2] * d[0] - omega[0] * d[2],
            omega[0] * d[1] - omega[1] * d[0],
        ];
        let mut m = [0.0; 12];
        m[3 + axis] = 1.0;
        m[9 + axis] = 1.0;
        m[6..9].copy_from_slice(&swing);
        modes.push(m);
    }
    modes
}

/// Worst row-relative rigid-null residual (P1); `None` when a zero-scale row
/// has a nonzero residual.
fn null_residual(k: &Matrix12, d: [f64; 3]) -> Option<f64> {
    let mut worst = 0.0_f64;
    for mode in rigid_modes(d) {
        for row in k {
            let mut residual = ExactAccumulator::new();
            let mut magnitude = ExactAccumulator::new();
            for (kv, &u) in row.iter().zip(&mode) {
                residual.add_product(*kv, u).unwrap();
                magnitude.add_product(kv.abs(), u.abs()).unwrap();
            }
            let (r, m) = (residual.round().unwrap().abs(), magnitude.round().unwrap());
            if m == 0.0 {
                if r != 0.0 {
                    return None;
                }
            } else {
                worst = worst.max(r / m);
            }
        }
    }
    Some(worst)
}

fn subtract(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[test]
fn t4_u1_frozen_cases_meet_p1_p3_p4_and_the_diagonal_scaled_criterion_at_every_x() {
    let data = references();
    let cases = data["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 49);
    let mut bands: Vec<(String, f64, f64, f64)> = Vec::new();
    let mut count = 0;
    for case in cases {
        let id = case["id"].as_str().unwrap();
        let band = case["angle_label"].as_str().unwrap().to_string();
        let inputs = case_inputs(case);
        let reference = matrix(&case["K_global"]);
        let mut first: Option<Matrix12> = None;
        let nodes = case["nodes_by_X"].as_object().unwrap();
        assert!(nodes.len() >= 2, "{id}");
        for (x, pair) in nodes {
            let (xi, xj) = (v3(&pair["x_i"]), v3(&pair["x_j"]));
            let d = subtract(xj, xi);
            assert_eq!(d, v3(&case["inputs"]["d"]), "{id} X {x}: grid d");
            let k = element(xi, xj, &inputs).global_stiffness().unwrap();
            // P3: bit-identical at every X.
            match &first {
                None => first = Some(k),
                Some(k0) => {
                    for r in 0..12 {
                        for c in 0..12 {
                            assert_eq!(k[r][c].to_bits(), k0[r][c].to_bits(), "{id} X {x}: P3");
                        }
                    }
                }
            }
            let (p4, diagonal) = agreement(&k, &reference);
            assert!(p4 <= P4_CRITERION, "{id} X {x}: P4 {p4:e}");
            assert!(diagonal <= P4_CRITERION, "{id} X {x}: S-4 {diagonal:e}");
            let p1 = null_residual(&k, d).unwrap_or(f64::INFINITY);
            assert!(p1 <= P1_CRITERION, "{id} X {x}: P1 {p1:e}");
            match bands.iter_mut().find(|b| b.0 == band) {
                Some(b) => {
                    b.1 = b.1.max(p4);
                    b.2 = b.2.max(diagonal);
                    b.3 = b.3.max(p1);
                }
                None => bands.push((band.clone(), p4, diagonal, p1)),
            }
            count += 1;
        }
    }
    for (band, p4, diagonal, p1) in &bands {
        eprintln!("T4-U1 band {band}: worst P4 {p4:.2e}, diagonal-scaled {diagonal:.2e}, P1 {p1:.2e}");
    }
    assert_eq!(count, 48 * 5 + 2);
}

#[test]
fn t4_u1_exact_rotation_gives_the_permuted_matrix() {
    // P2: P maps (x, y, z) to (z, x, y); K(Px, Py) = P K Pᵀ to 1e-12.
    let permute = |v: [f64; 3]| [v[2], v[0], v[1]];
    let data = references();
    for case in data["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let mut inputs = case_inputs(case);
        let reference = matrix(&case["K_global"]);
        let pair = case["nodes_by_X"].as_object().unwrap().values().next().unwrap();
        let (xi, xj) = (v3(&pair["x_i"]), v3(&pair["x_j"]));
        let base = element(xi, xj, &inputs).global_stiffness().unwrap();
        inputs.y = permute(inputs.y);
        let rotated = element(permute(xi), permute(xj), &inputs)
            .global_stiffness()
            .unwrap();
        // Global DOF index of (block b, component c) after P: c → (c + 1) mod 3.
        let map = |index: usize| 3 * (index / 3) + (index % 3 + 1) % 3;
        let scale = max_abs(&reference);
        let mut worst = 0.0_f64;
        for r in 0..12 {
            for c in 0..12 {
                worst = worst.max((rotated[map(r)][map(c)] - base[r][c]).abs() / scale);
            }
        }
        assert!(worst <= P2_CRITERION, "{id}: P2 {worst:e}");
    }
}

#[test]
fn t4_u1_large_coordinate_controls_are_formed_and_meet_p1_and_p4() {
    // B5.4: eight elbows at X ≈ 5e6 and 7.3e6 that the centre-based element
    // refused (radius mismatch); each carries its own frozen K.
    let data = references();
    let controls = data["utm_controls_refused_today"]["cases"].as_array().unwrap();
    assert_eq!(controls.len(), 8);
    for control in controls {
        let section = &control["section"];
        let k_factor = f(&control["k"]);
        let inputs = Inputs {
            radius: f(&control["R"]),
            y: v3(&control["y_reference"]),
            e: f(&section["e"]),
            g: f(&section["g"]),
            a: f(&section["a"]),
            i: f(&section["i"]),
            j: f(&section["j"]),
            k_in: k_factor,
            k_out: k_factor,
        };
        let (xi, xj) = (v3(&control["x_i"]), v3(&control["x_j"]));
        let k = element(xi, xj, &inputs).global_stiffness().unwrap();
        let (p4, diagonal) = agreement(&k, &matrix(&control["K_global"]));
        let p1 = null_residual(&k, subtract(xj, xi)).unwrap_or(f64::INFINITY);
        eprintln!(
            "T4-U1 control X {}: P4 {p4:.2e}, diagonal-scaled {diagonal:.2e}, P1 {p1:.2e}",
            f(&control["X"])
        );
        assert!(p4 <= P4_CRITERION && diagonal <= P4_CRITERION && p1 <= P1_CRITERION);
    }
}

/// A seeded linear congruential generator (no dependency): uniform in [0, 1).
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}

#[test]
fn t4_u1_seeded_elbows_at_every_x_are_formed_and_annihilate_rigid_motions() {
    // P5 (and P1) on N = 2,000 seeded elbows per X ∈ {0, 5e5, 2e6, 5e6,
    // 7.3e6} (T4-I2 §1.4's column). Angles U(1e-4, π − 1e-4) rad (RV2 N-8:
    // inside [1e-9, π − 1e-9]); R U(0.05, 3) m; node i on a 1 mm grid near
    // (X, 0.7X, 0); random plane; bow toward y.
    let mut rng = Lcg(20261010);
    let mut worst = 0.0_f64;
    let (mut formed, mut plane_refused) = (0usize, 0usize);
    for x in [0.0, 5.0e5, 2.0e6, 5.0e6, 7.3e6] {
        for _ in 0..2000 {
            let phi = 1.0e-4 + rng.next() * (std::f64::consts::PI - 2.0e-4);
            let radius = 0.05 + rng.next() * 2.95;
            let chord_length = 2.0 * radius * (0.5 * phi).sin();
            // A random unit chord direction and a random reference vector.
            let z = 2.0 * rng.next() - 1.0;
            let t = 2.0 * std::f64::consts::PI * rng.next();
            let w = (1.0 - z * z).sqrt();
            let direction = [w * t.cos(), w * t.sin(), z];
            let y = [
                2.0 * rng.next() - 1.0,
                2.0 * rng.next() - 1.0,
                2.0 * rng.next() - 1.0,
            ];
            let mm = |v: f64| (v * 1000.0).round() / 1000.0;
            let xi = [
                mm(x + 10.0 * rng.next()),
                mm(0.7 * x + 10.0 * rng.next()),
                mm(10.0 * rng.next()),
            ];
            let xj = [
                xi[0] + chord_length * direction[0],
                xi[1] + chord_length * direction[1],
                xi[2] + chord_length * direction[2],
            ];
            // The represented chord may be a little longer than 2R for
            // nearly semicircular arcs; use a radius that spans it.
            let d = subtract(xj, xi);
            let length = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
            let radius = radius.max(0.5 * length * (1.0 + 1.0e-12));
            let inputs = Inputs {
                radius,
                y,
                e: 2.0e11,
                g: 8.0e10,
                a: 0.005969026041820614,
                i: 2.700984283923829e-05,
                j: 5.401968567847658e-05,
                k_in: 1.0,
                k_out: 1.0,
            };
            let element = CurvedBendMacroElement::new(
                FrameNode::new(0, xi).unwrap(),
                FrameNode::new(1, xj).unwrap(),
                inputs.radius,
                inputs.y,
                inputs.e,
                inputs.g,
                inputs.a,
                inputs.i,
                inputs.j,
                1.0,
                1.0,
            );
            let element = match element {
                Ok(element) => element,
                // A random y nearly parallel to the chord is a legitimate
                // plane refusal, not a radius refusal.
                Err(open_pipe_stress_curved_bend::CurvedBendError::DegenerateArc { .. }) => {
                    plane_refused += 1;
                    continue;
                }
                Err(error) => panic!("P5: X {x} phi {phi}: {error}"),
            };
            let k = element.global_stiffness().unwrap();
            let p1 = null_residual(&k, d).unwrap_or(f64::INFINITY);
            assert!(p1 <= P1_CRITERION, "X {x} phi {phi}: P1 {p1:e}");
            worst = worst.max(p1);
            formed += 1;
        }
    }
    eprintln!(
        "T4-U1 seeded elbows: {formed} formed, {plane_refused} plane refusals, worst P1 {worst:.2e}"
    );
    assert_eq!(formed + plane_refused, 10_000);
    assert!(plane_refused <= 10, "{plane_refused}");
}
