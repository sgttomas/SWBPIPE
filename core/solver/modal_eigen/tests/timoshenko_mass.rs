//! The Timoshenko straight mass (DEL-04-09 REQ-001, AC-012) against the
//! exact element references of `validation/references/modal_timoshenko_mass/`.
//!
//! Source: `tests/fixtures/timoshenko_mass_cases.txt`, written by
//! `timoshenko_mass_cases.py` beside it from the references' `references.json`.
//! The element is formed in double-double from the cases' binary64 operands
//! and compared normwise with the references' global 12×12, which covers both
//! bending planes, the axial and torsional terms and the turn to global axes.

use std::collections::BTreeMap;

use open_pipe_stress_modal_eigen::dd::Dd;
use open_pipe_stress_modal_eigen::reformation::mass::{
    straight_mass, straight_mass_timoshenko, GeometryError, TimoshenkoShear,
};

/// The same normwise threshold as `tests/reformation.rs` for the
/// Euler–Bernoulli straight: an empirical fixture threshold, not a bound.
const NORMWISE: f64 = 1e-29;

struct Case {
    id: String,
    points: BTreeMap<String, [f64; 3]>,
    ops: BTreeMap<String, f64>,
    phi: BTreeMap<String, Dd>,
    global: BTreeMap<(usize, usize), Dd>,
}

fn bits(s: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(s, 16).unwrap())
}

fn dd(hi: &str, lo: &str) -> Dd {
    Dd::from_f64(bits(hi)).add_f64(bits(lo))
}

fn cases() -> Vec<Case> {
    let text = include_str!("fixtures/timoshenko_mass_cases.txt");
    let mut out = Vec::new();
    let mut current: Option<Case> = None;
    for line in text.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        match f[0] {
            "source" => {}
            "case" => {
                current = Some(Case {
                    id: f[1].to_string(),
                    points: BTreeMap::new(),
                    ops: BTreeMap::new(),
                    phi: BTreeMap::new(),
                    global: BTreeMap::new(),
                })
            }
            "point" => {
                let c = current.as_mut().unwrap();
                c.points.insert(f[1].to_string(), [bits(f[2]), bits(f[3]), bits(f[4])]);
            }
            "op" => {
                current.as_mut().unwrap().ops.insert(f[1].to_string(), bits(f[2]));
            }
            "phi" => {
                current.as_mut().unwrap().phi.insert(f[1].to_string(), dd(f[2], f[3]));
            }
            "g" => {
                let (r, c) = (f[1].parse().unwrap(), f[2].parse().unwrap());
                current.as_mut().unwrap().global.insert((r, c), dd(f[3], f[4]));
            }
            "end" => out.push(current.take().unwrap()),
            other => panic!("unknown record {other}"),
        }
    }
    assert!(current.is_none());
    out
}

fn shear(c: &Case) -> TimoshenkoShear {
    TimoshenkoShear {
        elastic_modulus: Dd::from_f64(c.ops["E"]),
        shear_modulus: Dd::from_f64(c.ops["G"]),
        shear_area: Dd::from_f64(c.ops["A_s"]),
        second_moment_y: Dd::from_f64(c.ops["I_y"]),
        second_moment_z: Dd::from_f64(c.ops["I_z"]),
    }
}

fn form(c: &Case, s: &TimoshenkoShear) -> Result<[[Dd; 12]; 12], GeometryError> {
    straight_mass_timoshenko(
        c.points["xi"],
        c.points["xj"],
        c.points["y_reference"],
        Dd::from_f64(c.ops["mbar"]),
        Dd::from_f64(c.ops["ibar"]),
        Dd::from_f64(c.ops["jbar"]),
        s,
    )
}

fn normwise_upper(m: &[[Dd; 12]; 12], expected: &BTreeMap<(usize, usize), Dd>) -> f64 {
    let scale = expected.values().fold(0.0f64, |a, v| a.max(v.abs().to_f64()));
    let mut worst = 0.0f64;
    for r in 0..12 {
        for c in r..12 {
            let e = expected.get(&(r, c)).copied().unwrap_or(Dd::ZERO);
            worst = worst.max(m[r][c].sub(e).abs().to_f64() / scale);
        }
    }
    worst
}

#[test]
fn the_element_matches_its_exact_references_in_global_axes() {
    let all = cases();
    assert_eq!(all.len(), 4, "the reference's four cases");
    let mut worst = 0.0f64;
    for c in &all {
        let m = form(c, &shear(c)).unwrap();
        // The reference is exactly symmetric (its generator checks it). The
        // turn to global axes sums (r, c) and (c, r) in different orders, as
        // for the Euler–Bernoulli straight, so the lower triangle is held to
        // the same normwise tolerance rather than to bitwise equality.
        let scale = c.global.values().fold(0.0f64, |a, v| a.max(v.abs().to_f64()));
        for r in 0..12 {
            for k in 0..r {
                let d = m[r][k].sub(m[k][r]).abs().to_f64() / scale;
                assert!(d <= NORMWISE, "{}: asymmetry {d:e} at ({r}, {k})", c.id);
            }
        }
        let e = normwise_upper(&m, &c.global);
        assert!(e <= NORMWISE, "{}: normwise difference {e:e}", c.id);
        worst = worst.max(e);
    }
    eprintln!("Timoshenko straight mass, largest normwise difference: {worst:e}");
}

/// The unequal-second-moment case separates the planes: forming it with I_y
/// and I_z exchanged must miss the reference by far more than the tolerance.
#[test]
fn exchanging_the_planes_second_moments_is_detected() {
    let c = cases().into_iter().find(|c| c.id == "T-SKEW-UNEQUAL").unwrap();
    let mut s = shear(&c);
    std::mem::swap(&mut s.second_moment_y, &mut s.second_moment_z);
    let e = normwise_upper(&form(&c, &s).unwrap(), &c.global);
    assert!(e > 1e-6, "a plane swap moved the element by only {e:e}");
}

/// Near Φ = 0 the element approaches the Euler–Bernoulli element: on the
/// near-EB case (Φ about 1.8e-13) every entry is within 10Φ of the
/// Euler–Bernoulli straight, normwise.
#[test]
fn the_element_approaches_the_euler_bernoulli_element() {
    let c = cases().into_iter().find(|c| c.id == "T-AXIS-NEAR-EB").unwrap();
    let phi = c.phi["y"].to_f64().max(c.phi["z"].to_f64());
    assert!(phi > 0.0 && phi < 1e-12);
    let t = form(&c, &shear(&c)).unwrap();
    let eb = straight_mass(
        c.points["xi"],
        c.points["xj"],
        c.points["y_reference"],
        Dd::from_f64(c.ops["mbar"]),
        Dd::from_f64(c.ops["ibar"]),
        Dd::from_f64(c.ops["jbar"]),
    )
    .unwrap();
    let mut expected = BTreeMap::new();
    for r in 0..12 {
        for k in r..12 {
            if !eb[r][k].is_zero() {
                expected.insert((r, k), eb[r][k]);
            }
        }
    }
    let e = normwise_upper(&t, &expected);
    assert!(e <= 10.0 * phi, "Timoshenko against Euler-Bernoulli at phi {phi:e}: {e:e}");
}

/// The PR 335 reader's witness: every operand normal, Φ = 2⁴⁰⁰ in both
/// planes, m̄ = ī = j̄ = 2⁻⁷⁰⁰. A prefactor formed in double-double before its
/// numerator underflowed to zero and lost the whole bending plane; the plane's
/// displacement diagonal is normal. Expected: the exact integral of the
/// static shapes, rounded to hi and lo (the reader's independent calculation).
#[test]
fn a_representable_plane_survives_tiny_prefactors() {
    let small = Dd::from_f64(2.0f64.powi(-700));
    let one = Dd::from_f64(1.0);
    let shear = TimoshenkoShear {
        elastic_modulus: one,
        shear_modulus: one,
        shear_area: Dd::from_f64(12.0 * 2.0f64.powi(-400)),
        second_moment_y: one,
        second_moment_z: one,
    };
    let m = straight_mass_timoshenko(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        small,
        small,
        small,
        &shear,
    )
    .unwrap();
    let expected = Dd {
        hi: f64::from_bits(0x1415_5555_5555_5555),
        lo: f64::from_bits(0x10b5_5555_5555_5555),
    };
    for r in [1usize, 2usize] {
        let relative = m[r][r].sub(expected).abs().to_f64() / expected.abs().to_f64();
        assert!(relative <= NORMWISE, "({r}, {r}): {relative:e}");
    }
}

/// An entry that cannot be held at double-double precision is refused by
/// name, never returned as zero: m̄ = ī = 2⁻¹⁰⁰⁰ on the same member puts every
/// bending entry far below 2⁻⁹⁶⁹.
#[test]
fn an_unrepresentable_plane_is_refused_by_name() {
    let tiny = Dd::from_f64(2.0f64.powi(-1000));
    let one = Dd::from_f64(1.0);
    let shear = TimoshenkoShear {
        elastic_modulus: one,
        shear_modulus: one,
        shear_area: Dd::from_f64(12.0 * 2.0f64.powi(-400)),
        second_moment_y: one,
        second_moment_z: one,
    };
    let r = straight_mass_timoshenko(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        tiny,
        tiny,
        one,
        &shear,
    );
    assert_eq!(r, Err(GeometryError::TimoshenkoMassRange));
}

#[test]
fn shear_operands_that_are_not_positive_and_finite_are_refused() {
    let c = cases().into_iter().next().unwrap();
    for bad in [0.0, -1.0, f64::INFINITY, f64::NAN] {
        let mut s = shear(&c);
        s.shear_area = Dd::from_f64(bad);
        assert_eq!(form(&c, &s), Err(GeometryError::ShearOperand), "A_s = {bad}");
        let mut s = shear(&c);
        s.second_moment_y = Dd::from_f64(bad);
        assert_eq!(form(&c, &s), Err(GeometryError::ShearOperand), "I_y = {bad}");
    }
}
