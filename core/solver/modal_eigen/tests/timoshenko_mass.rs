//! The Timoshenko straight mass (DEL-04-09 REQ-001, AC-012) against the
//! exact element references of `validation/references/modal_timoshenko_mass/`.
//!
//! Source: `tests/fixtures/timoshenko_mass_cases.txt`, written by
//! `timoshenko_mass_cases.py` beside it from the package's two outputs, the
//! source-blind derivation of the frozen problem statement's five cases and of
//! its Addendum 1's three.
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
        c.points["x_i"],
        c.points["x_j"],
        c.points["v"],
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
    assert_eq!(all.len(), 8, "the statement's five cases and the addendum's three");
    let mut worst = 0.0f64;
    for c in &all {
        let m = form(c, &shear(c)).unwrap();
        // The turn forms the upper triangle exactly and mirrors it, so the
        // returned matrix is bitwise symmetric.
        for r in 0..12 {
            for k in 0..r {
                assert_eq!(m[r][k], m[k][r], "{}: asymmetry at ({r}, {k})", c.id);
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
        c.points["x_i"],
        c.points["x_j"],
        c.points["v"],
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

/// The first PR 335 reader's witness, now the package's case T-RANGE-TINY-MASS:
/// every operand normal, Φ = 2⁴⁰⁰ in both planes, m̄ = ī = j̄ = 2⁻⁷⁰⁰. A prefactor
/// formed in double-double before its numerator underflowed to zero and lost
/// the whole bending plane. The displacement diagonals (1, 1) and (2, 2) are
/// held entrywise to the package's values.
#[test]
fn a_representable_plane_survives_tiny_prefactors() {
    let c = cases().into_iter().find(|c| c.id == "T-RANGE-TINY-MASS").unwrap();
    let m = form(&c, &shear(&c)).unwrap();
    for r in [1usize, 2usize] {
        let expected = c.global[&(r, r)];
        let relative = m[r][r].sub(expected).abs().to_f64() / expected.abs().to_f64();
        assert!(relative <= NORMWISE, "({r}, {r}): {relative:e}");
    }
}

/// Addendum 1's three cases, each formed and held ENTRYWISE (not only
/// normwise) to the package: an exactly representable plane below 2⁻⁹⁶⁹, and
/// Φ = 2²⁰⁰⁰ and 2⁻²⁰⁰⁰, which no longer have to fit a double-double.
#[test]
fn the_addendum_cases_are_formed_and_match_entrywise() {
    for id in ["T-RANGE-SUBFLOOR", "T-RANGE-PHI-HUGE", "T-RANGE-PHI-TINY"] {
        let c = cases().into_iter().find(|c| c.id == id).unwrap();
        let m = form(&c, &shear(&c)).unwrap();
        for r in 0..12 {
            for k in r..12 {
                let e = c.global.get(&(r, k)).copied().unwrap_or(Dd::ZERO);
                if e.is_zero() {
                    assert!(m[r][k].is_zero(), "{id} ({r}, {k}): expected zero, got {:?}", m[r][k]);
                } else {
                    let relative = m[r][k].sub(e).abs().to_f64() / e.abs().to_f64();
                    assert!(relative <= NORMWISE, "{id} ({r}, {k}): {relative:e}");
                }
            }
        }
    }
}

/// A sparse-tailed shear parameter, Φ = (1 + 2⁻¹¹⁰)·2⁻⁹⁶⁸, which the earlier
/// repair refused because Φ itself did not narrow exactly. It is formed now.
/// It is an arithmetic test, not a member reference: at this Φ the element
/// equals the Euler–Bernoulli straight to far below the comparison threshold.
#[test]
fn a_sparse_tailed_shear_parameter_is_formed() {
    let one = Dd::from_f64(1.0);
    let s = TimoshenkoShear {
        elastic_modulus: Dd { hi: 1.0, lo: 2.0f64.powi(-110) },
        shear_modulus: one,
        shear_area: Dd::from_f64(12.0),
        second_moment_y: Dd::from_f64(2.0f64.powi(-968)),
        second_moment_z: Dd::from_f64(2.0f64.powi(-968)),
    };
    let (xi, xj, y) = ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let t = straight_mass_timoshenko(xi, xj, y, one, one, one, &s).unwrap();
    let eb = straight_mass(xi, xj, y, one, one, one).unwrap();
    let mut expected = BTreeMap::new();
    for r in 0..12 {
        for k in r..12 {
            if !eb[r][k].is_zero() {
                expected.insert((r, k), eb[r][k]);
            }
        }
    }
    let e = normwise_upper(&t, &expected);
    assert!(e <= NORMWISE, "Timoshenko against Euler-Bernoulli at a sparse-tailed tiny phi: {e:e}");
}

/// At m̄ = ī = 2^-1000 and Φ = 2^400, completed bending entries cannot
/// narrow within the conversion precision budget. Refuse by name.
#[test]
fn a_plane_outside_the_conversion_precision_budget_is_refused_by_name() {
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

fn unit_shear() -> TimoshenkoShear {
    let one = Dd::from_f64(1.0);
    TimoshenkoShear {
        elastic_modulus: one,
        shear_modulus: one,
        shear_area: Dd::from_f64(12.0),
        second_moment_y: one,
        second_moment_z: one,
    }
}

/// The round-2 reader's finding-2 witness 1: Φ = 1 in both planes and both
/// planes representable, but the exact torsional diagonal j̄L/3 = 2⁻¹¹⁰⁰/3 is
/// outside double-double range. It is refused, not returned as zero.
#[test]
fn an_unrepresentable_torsional_entry_is_refused_by_name() {
    let mut s = unit_shear();
    s.elastic_modulus = Dd::from_f64(2.0f64.powi(-200));
    let r = straight_mass_timoshenko(
        [0.0, 0.0, 0.0],
        [2.0f64.powi(-100), 0.0, 0.0],
        [0.0, 1.0, 0.0],
        Dd::from_f64(2.0f64.powi(200)),
        Dd::from_f64(1.0),
        Dd::from_f64(2.0f64.powi(-1000)),
        &s,
    );
    assert_eq!(r, Err(GeometryError::TimoshenkoMassRange));
}

/// The round-2 reader's finding-2 witness 2: a skew member at m̄ = ī = j̄ =
/// 2⁻⁹⁶⁰ whose global (0, 1) entry, turned in plain double-double, came back
/// with a normal high word and 3.85e-23 relative error. It must now be
/// returned within the conversion budget or refused, never silently degraded:
/// the reader's expected behaviour is refusal.
#[test]
fn a_degraded_global_entry_is_refused_by_name() {
    let tiny = Dd::from_f64(2.0f64.powi(-960));
    let r = straight_mass_timoshenko(
        [0.0, 0.0, 0.0],
        [1.0, 2.0f64.powi(-40), 0.0],
        [0.0, 0.0, 1.0],
        tiny,
        tiny,
        tiny,
        &unit_shear(),
    );
    assert_eq!(r, Err(GeometryError::TimoshenkoMassRange));
}

/// The round-2 reader's finding-2 witness 3: the exact global (0, 1) entry is
/// nonzero, of order 2⁻¹⁵⁰⁰, and every plain double-double term underflowed
/// to a finite zero. It is refused, not returned as zero.
#[test]
fn a_nonzero_global_entry_below_range_is_refused_by_name() {
    let tiny = Dd::from_f64(2.0f64.powi(-900));
    let r = straight_mass_timoshenko(
        [0.0, 0.0, 0.0],
        [1.0, 2.0f64.powi(-600), 0.0],
        [0.0, 0.0, 1.0],
        tiny,
        tiny,
        tiny,
        &unit_shear(),
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
