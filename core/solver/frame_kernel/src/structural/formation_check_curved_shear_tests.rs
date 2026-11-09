//! Curved-bend transverse shear in the K-D5 re-formation and the R-1 load
//! certificate (DEL-04-01 Work `curved-bend-transverse-shear.md` §5 Step A,
//! §8). The references are the independent free-body/virtual-work oracle in
//! `validation/references/curved_bend_shear`, which reads no product code.
use super::super::arc_certificate::certify_curved_uniform_load;
use super::super::element_reformation::{reform_bend, BendOperands, WideOperand};
use super::*;
use crate::exact_sum::ExactAccumulator;

type Matrix6 = [[f64; 6]; 6];
type Matrix12 = [[f64; 12]; 12];

#[allow(dead_code)]
struct OracleCase {
    name: &'static str,
    xi: [f64; 3],
    xj: [f64; 3],
    reference: [f64; 3],
    radius: f64,
    kin: f64,
    kout: f64,
    load: [f64; 3],
    f_eb: Matrix6,
    f_v2: Matrix6,
    k_v2: Matrix12,
    p_v2: [f64; 12],
    d_v2: [f64; 6],
}

#[allow(dead_code)]
struct SweepCase {
    name: &'static str,
    xi: [f64; 3],
    xj: [f64; 3],
    reference: [f64; 3],
    radius: f64,
    f: [[f64; 3]; 3],
    d: [[f64; 3]; 3],
}

// The angle sweeps are the kernel's (curved_bend transverse_shear_tests);
// this module checks the element cases.
#[allow(dead_code)]
mod reference {
    include!("../../../../../validation/references/curved_bend_shear/kernel_cases.rs");
}

/// The oracle's section (`element_cases.json`).
const SHEAR_AREA: f64 = 0.00390625;

fn arc(c: &OracleCase, shear_area: Option<f64>) -> CurvedFormation {
    CurvedFormation {
        node_i: 0,
        node_j: 1,
        coordinates_i: c.xi,
        coordinates_j: c.xj,
        radius: c.radius,
        y_reference: c.reference,
        elastic_modulus: 200_000_000_000.0,
        shear_modulus: 80_000_000_000.0,
        area: 0.0078125,
        second_moment: 0.000030517578125,
        torsion_constant: 0.00006103515625,
        in_plane_flexibility_factor: c.kin,
        out_of_plane_flexibility_factor: c.kout,
        shear_area,
    }
}

fn round(w: &Wide2) -> f64 {
    let mut acc = ExactAccumulator::new();
    for &t in w.split_binary64().unwrap().terms() {
        acc.add(t).unwrap();
    }
    acc.round().unwrap()
}

/// |Σ terms − x| rounded once (the exact residual of a split against x).
fn residual(terms: &[f64], x: f64) -> f64 {
    let mut acc = ExactAccumulator::new();
    for &t in terms {
        acc.add(t).unwrap();
    }
    acc.add(-x).unwrap();
    acc.round().unwrap().abs()
}

fn matrix(e: &CurvedFormation) -> Element {
    let mut a = WideArith::new(FORMATION_PRECISION).unwrap();
    curved_matrix(&mut a, e).unwrap()
}

fn row_scale(row: &[f64; 12]) -> f64 {
    row.iter().fold(0.0_f64, |m, x| m.max(x.abs()))
}

fn primitive(formula: LoadFormula) -> LoadPrimitive {
    LoadPrimitive {
        term_index: 0,
        identity: LoadSummandIdentity {
            family: "family".into(),
            source: "member".into(),
            ordinal: 0,
        },
        local_row: 0,
        formula,
        force_scale_exponent: 0,
    }
}

#[test]
fn curved_shear_reformation_matches_the_independent_oracle() {
    for c in reference::CASES {
        let v2 = matrix(&arc(c, Some(SHEAR_AREA)));
        let eb = matrix(&arc(c, None));
        let mut shear_moves_k = false;
        for i in 0..12 {
            // The oracle (34 digits, correctly rounded) and the p = 128
            // re-formation (correctly rounded here) agree to a few ulp of the
            // row; an entry far below the row scale is held to that scale.
            let scale = row_scale(&c.k_v2[i]);
            for j in 0..12 {
                let actual = round(&v2[i][j]);
                assert!(
                    (actual - c.k_v2[i][j]).abs() <= 4.0 * f64::EPSILON * scale,
                    "{} K[{i}][{j}]: {actual:e} against {:e}",
                    c.name,
                    c.k_v2[i][j]
                );
                shear_moves_k |= (round(&eb[i][j]) - actual).abs() > 1e-6 * scale;
            }
        }
        assert!(shear_moves_k, "{}: shear must change K", c.name);
    }
}

#[test]
fn curved_shear_reform_bend_reads_the_shear_leaf_not_the_geometry_carrier() {
    for c in reference::CASES {
        let e = arc(c, Some(SHEAR_AREA));
        let lifted = |x: f64| WideOperand::from_f64(x);
        let operands = BendOperands {
            coordinates_i: e.coordinates_i,
            coordinates_j: e.coordinates_j,
            radius: e.radius,
            y_reference: e.y_reference,
            elastic_modulus: lifted(e.elastic_modulus),
            shear_modulus: lifted(e.shear_modulus),
            area: lifted(e.area),
            second_moment: lifted(e.second_moment),
            torsion_constant: lifted(e.torsion_constant),
            in_plane_flexibility_factor: lifted(e.in_plane_flexibility_factor),
            out_of_plane_flexibility_factor: lifted(e.out_of_plane_flexibility_factor),
            shear_area: Some(lifted(SHEAR_AREA)),
        };
        let seam = reform_bend(&operands).unwrap();
        let direct = matrix(&e);
        for i in 0..12 {
            for j in 0..12 {
                let mut expected = [0.0; 3];
                let split = direct[i][j].split_binary64().unwrap();
                expected[..split.terms().len()].copy_from_slice(split.terms());
                assert_eq!(seam.terms(i, j), expected, "{} [{i}][{j}]", c.name);
            }
        }
        let without = reform_bend(&BendOperands {
            shear_area: None,
            ..operands
        })
        .unwrap();
        assert_ne!(without, seam, "{}", c.name);
    }
}

#[test]
fn curved_shear_load_certificate_encloses_the_independent_oracle() {
    for c in reference::CASES {
        let v2 = certify_curved_uniform_load(&arc(c, Some(SHEAR_AREA)), c.load).unwrap();
        let eb = certify_curved_uniform_load(&arc(c, None), c.load).unwrap();
        let mut separated = false;
        for (k, term) in v2.terms.iter().enumerate() {
            // The ball holds the exact value; the oracle is that value
            // rounded once to binary64.
            let p = c.p_v2[k];
            let bound = term.radius + f64::EPSILON * p.abs() + f64::MIN_POSITIVE;
            assert!(
                residual(&term.intended, p) <= bound,
                "{} p[{k}]: ball misses {p:e}",
                c.name
            );
            let e = &eb.terms[k];
            separated |= residual(&e.intended, p) > e.radius + f64::EPSILON * p.abs();
        }
        assert!(separated, "{}: shear must change the load vector", c.name);
    }
}

#[test]
fn curved_shear_free_strain_loads_are_the_shear_dependent_homothety() {
    let strain = EigenstrainPrimitive {
        thermal: ThermalPrimitive::ExplicitIntervalStrain { strain: 0.25 },
        fit: FitPrimitive::None,
    };
    for c in reference::CASES {
        let d: Vec<f64> = (0..3).map(|k| 0.25 * (c.xj[k] - c.xi[k])).collect();
        let formula = |shear_area| LoadFormula::FreeStrainCurved {
            arc: arc(c, shear_area),
            strain: strain.clone(),
            column: None,
        };
        let (value, radius) = load_reformation(&primitive(formula(Some(SHEAR_AREA)))).unwrap();
        let (eb, _) = load_reformation(&primitive(formula(None))).unwrap();
        let mut moved = false;
        for r in 0..12 {
            // p = K_v2 u_free with u_free = ε(x_j − x_i) at node j only; the
            // oracle K is rounded once per entry.
            let terms: Vec<f64> = (0..3).map(|k| c.k_v2[r][6 + k] * d[k]).collect();
            let magnitude: f64 = terms.iter().map(|t| t.abs()).sum();
            let expected: f64 = terms.iter().sum();
            let actual = round(&value[r]);
            assert!(
                (actual - expected).abs() <= radius[r] + 8.0 * f64::EPSILON * magnitude,
                "{} p[{r}]: {actual:e} against {expected:e}",
                c.name
            );
            moved |= (round(&eb[r]) - actual).abs() > 1e-6 * magnitude;
        }
        assert!(moved, "{}: the equivalent load must read the v2 K", c.name);
        // Self-equilibrated: the resultant force of K·u vanishes.
        for k in 0..3 {
            let resultant = round(&value[k]) + round(&value[6 + k]);
            let scale = round(&value[k]).abs() + round(&value[6 + k]).abs();
            assert!(resultant.abs() <= 1e-12 * scale.max(1.0), "{}", c.name);
        }
    }
}

#[test]
fn curved_shear_pressure_free_strain_reads_the_shear_dependent_k() {
    for c in reference::CASES {
        let formula = |shear_area| LoadFormula::PressureFreeStrainCurved {
            arc: arc(c, shear_area),
            pressure: 2_000_000.0,
            outer_diameter: 0.1143,
            wall_thickness: 0.00602,
            poisson_ratio: 0.3,
            column: None,
        };
        let (some, radius) = load_reformation(&primitive(formula(Some(SHEAR_AREA)))).unwrap();
        let (none, _) = load_reformation(&primitive(formula(None))).unwrap();
        // The same strain under both: the ratio of the node-j axial pushes
        // is the ratio of the two Ks applied to the same u_free.
        let k_some = matrix(&arc(c, Some(SHEAR_AREA)));
        let k_none = matrix(&arc(c, None));
        let d: Vec<f64> = (0..3).map(|k| c.xj[k] - c.xi[k]).collect();
        let push =
            |k: &Element, r: usize| -> f64 { (0..3).map(|n| round(&k[r][6 + n]) * d[n]).sum() };
        let mut moved = false;
        for r in 0..12 {
            let (ps, pn) = (push(&k_some, r), push(&k_none, r));
            let (vs, vn) = (round(&some[r]), round(&none[r]));
            // v = ε·push with one ε for both; cross-multiplication avoids
            // dividing by a structural zero.
            let tolerance =
                1e-12 * (vs.abs() * pn.abs() + vn.abs() * ps.abs()) + radius[r] * pn.abs();
            assert!(
                (vs * pn - vn * ps).abs() <= tolerance.max(1e-300),
                "{} [{r}]",
                c.name
            );
            moved |= (vs - vn).abs() > 1e-6 * vs.abs().max(vn.abs());
        }
        assert!(
            moved,
            "{}: pressure equivalent loads must read the v2 K",
            c.name
        );
    }
}

#[test]
fn curved_shear_binding_extends_only_the_opt_in() {
    let c = &reference::CASES[0];
    let mut none = Vec::new();
    curved_binding(&mut none, &arc(c, None));
    // Nodes (2), three coordinate triples (9) and eight scalars: the
    // Euler–Bernoulli layout is unchanged.
    assert_eq!(none.len(), 19);
    let mut some = Vec::new();
    curved_binding(&mut some, &arc(c, Some(SHEAR_AREA)));
    assert_eq!(some[..19], none[..]);
    assert_eq!(some[19..], [TIMOSHENKO_BINDING_TAG, SHEAR_AREA.to_bits()]);
    let mut other = Vec::new();
    curved_binding(&mut other, &arc(c, Some(2.0 * SHEAR_AREA)));
    assert_ne!(other, some);
    let entry = |shear_area| {
        primitive(LoadFormula::UniformCurved {
            arc: arc(c, shear_area),
            intensity: IntensityPrimitive::Authored(c.load),
        })
        .closure_entry()
    };
    assert_ne!(entry(None), entry(Some(SHEAR_AREA)));
}
