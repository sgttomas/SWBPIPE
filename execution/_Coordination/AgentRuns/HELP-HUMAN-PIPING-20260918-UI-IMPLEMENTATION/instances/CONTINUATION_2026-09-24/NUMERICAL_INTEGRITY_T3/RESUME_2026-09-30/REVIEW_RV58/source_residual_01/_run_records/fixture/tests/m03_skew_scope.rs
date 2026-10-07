//! I9: the skew M03 pin (T3; tests only). RV7's review of K2a, finding B1
//! (`T3/REVIEW/K2A_REVIEW.md`), and K2a's `RETURN_ADDENDUM_1.md` §1 and §1.4
//! (`T3/IMPLEMENTATION/K2A/`), where `T3/` is the numerical-integrity folder
//! under `projects/chirality-piping/execution/_Coordination/`.
//!
//! **M03's element-entry floor.** `transform_roundoff` bounds each global
//! entry (i, j) of T^T (K T) by g·(inherited/(1 − g) + second)·(1 + 64ε),
//! with g = gamma(24), and refuses a subnormal bound (`checked_value`).
//! - On an axis-aligned member each global entry is one local coefficient c,
//!   so its bound is about 2g·|c|: M03 refuses every nonzero coefficient
//!   below the floor 2^-1022/(2g), about 2^-974.585.
//! - On a skew member the sums mix the coefficients of each 3×3 block. The
//!   rotational block mixes GJ/L with 4EI/L and 2EI/L, and the
//!   translation–rotation coupling block holds only 6EI/L² terms. So M03
//!   accepts subnormal-derived 4EI/L and 2EI/L below the floor, and in RV7's
//!   cases 6EI/L² is the limiting coefficient. (Below those cases a second
//!   limit appears, a product underflow in the bound itself; see
//!   `m03_skew_pin_6eil2_is_the_limiting_coefficient`.)
//!
//! K2a's `local_stiffness` now refuses all of these formations by name. The
//! local matrix here is therefore main's pre-K2a one, formed operation for
//! operation (FK `local_stiffness` at `134eefc24`, `src/lib.rs:716-741`),
//! with FK's own orientation, `transform_global_stiffness` and
//! `transform_roundoff`, which K2a and K1 left unchanged.
//!
//! **Inputs** (invented; RV7's confirmed cases):
//! - L = 2^-39 m; a pipe of OD 1e-11 m and wall 1e-12 m, its section formed
//!   as PP `derive_pipe_section` forms it; G = 1e-100 Pa.
//! - E = (1/(12·I))·2^-1000·2^(t+1000), so that (12E)·I rounds to 2^t, for
//!   t = -1030, -1040, -1045, -1050 and -1055.
//! - S6a: E = (2.5/(12·I))·2^-1000·2^-75. (12E)·I is intended as
//!   2.5·2^-1075; the exact product of the binary64 operands is
//!   2.5·2^-1075·(1 − 3.8e-17), which rounds to 2^-1074, as (6E)·I does,
//!   while (4E)·I and (2E)·I round to 0.
//! - Members from N0 at the origin, all of length 2^-39 m: axis x (y
//!   reference +y); skew (1,1,1) (y reference +z); skew (1,2,2) (y reference
//!   +x).
//!
//! The exact references are integer arithmetic on the binary64 inputs taken
//! exactly (L is a power of two here, so each kEI/L^n is an integer times a
//! power of two). No binary64 reference is trusted.
use open_pipe_stress_frame_kernel::structural::{
    assemble_sparse_stiffness, factor_sparse_structural_profile, factor_structural_profile,
    finish_sparse_structural, finish_structural, gamma, prepare_sparse_structural,
    prepare_structural, solve_structural_dense, transform_roundoff, SparseAssemblyOptions,
    SparseStructuralSystem, SparseSymmetryEvidence, StiffnessBlock, StiffnessContribution,
    StructuralError, StructuralSolution, StructuralSystem, SymmetryEvidence,
    TransformationRoundoff,
};
use open_pipe_stress_frame_kernel::{
    element_dof_map, local_stiffness, transform_global_stiffness, FrameElement, FrameKernelError,
    FrameNode, FrameProperties, FrameSection, Matrix12, DOF_PER_NODE, ELEMENT_DOF, RX, RY, RZ, UX,
    UY, UZ,
};
use std::f64::consts::PI;

// ------------------------------------------------------------------ inputs

/// M03's element-entry refusal (`checked_value` on a bound).
const RANGE: StructuralError = StructuralError::Range("arithmetic outside normal range");

/// 2^k exactly, including subnormal powers (`powi` would give 1/inf = 0).
fn p2(k: i32) -> f64 {
    if k >= -1022 {
        2f64.powi(k)
    } else {
        f64::from_bits(1u64 << (k + 1074))
    }
}

const LENGTH_EXPONENT: i32 = -39;

/// The section as PP `derive_pipe_section` forms it: (A, I, J).
fn section() -> (f64, f64, f64) {
    let (od, wall) = (1.0e-11_f64, 1.0e-12_f64);
    let id = od - 2.0 * wall;
    let area = PI * (od.powi(2) - id.powi(2)) / 4.0;
    let inertia = PI * (od.powi(4) - id.powi(4)) / 64.0;
    (area, inertia, 2.0 * inertia)
}

const G: f64 = 1.0e-100;

#[derive(Debug, Clone, Copy)]
struct Case {
    label: &'static str,
    e: f64,
    /// RV7's table: accepted on the skew members (never on the axis member).
    skew_accepted: bool,
    /// RV7's 2EI/L and 4EI/L relative errors, as stated (K2A_REVIEW.md B1;
    /// RETURN_ADDENDUM_1 §1.2), for the accepted rows.
    rv7_errors: Option<(f64, f64)>,
    /// The exact 2EI/L and 4EI/L relative errors to five digits, from the
    /// independent Fraction cross-check
    /// (`T3/IMPLEMENTATION/M03_SKEW_PIN/_run_records/exact_reference.py.txt`).
    exact_errors: Option<(f64, f64)>,
}

fn cases() -> Vec<Case> {
    let (_, i, _) = section();
    let e = |t: i32| (1.0 / (12.0 * i)) * p2(-1000) * p2(t + 1000);
    vec![
        Case {
            label: "(12E)I=2^-1030",
            e: e(-1030),
            skew_accepted: true,
            rv7_errors: Some((1.1e-13, 5.7e-14)),
            exact_errors: Some((1.1378e-13, 5.6750e-14)),
        },
        Case {
            label: "(12E)I=2^-1040",
            e: e(-1040),
            skew_accepted: true,
            rv7_errors: Some((1.16e-10, 5.8e-11)),
            exact_errors: Some((1.1642e-10, 5.8208e-11)),
        },
        Case {
            label: "(12E)I=2^-1045",
            e: e(-1045),
            skew_accepted: true,
            rv7_errors: Some((3.73e-9, 1.86e-9)),
            exact_errors: Some((3.7253e-9, 1.8626e-9)),
        },
        Case {
            label: "(12E)I=2^-1050",
            e: e(-1050),
            skew_accepted: true,
            rv7_errors: Some((1.19e-7, 5.96e-8)),
            exact_errors: Some((1.1921e-7, 5.9605e-8)),
        },
        Case {
            label: "(12E)I=2^-1055",
            e: e(-1055),
            skew_accepted: false,
            rv7_errors: None,
            exact_errors: None,
        },
        Case {
            label: "S6a (12E)I=2.5*2^-1075",
            e: (2.5 / (12.0 * i)) * p2(-1000) * p2(-75),
            skew_accepted: false,
            rv7_errors: None,
            exact_errors: None,
        },
    ]
}

#[derive(Debug, Clone, Copy)]
struct Orientation {
    label: &'static str,
    node_j: [f64; 3],
    y_reference: [f64; 3],
    skew: bool,
    /// RV7's least rotational-block bound (rows and columns RX..RZ of N0) and
    /// least positive coupling-block bound (rows UX..UZ, columns RX..RZ of
    /// N0), log2 to two decimals, down the accepted rows (RV7's
    /// `T3/REVIEW/_run_records/k2a/logs/fk_base3.log`).
    rv7_rotational_log2: f64,
    rv7_coupling_log2: [f64; 4],
}

fn orientations() -> [Orientation; 3] {
    let l = p2(LENGTH_EXPONENT);
    let s = l / 3f64.sqrt();
    [
        Orientation {
            label: "axis x, yref +y",
            node_j: [l, 0.0, 0.0],
            y_reference: [0.0, 1.0, 0.0],
            skew: false,
            rv7_rotational_log2: f64::NAN,
            rv7_coupling_log2: [f64::NAN; 4],
        },
        Orientation {
            label: "skew (1,1,1), yref +z",
            node_j: [s, s, s],
            y_reference: [0.0, 0.0, 1.0],
            skew: true,
            rv7_rotational_log2: -492.47,
            rv7_coupling_log2: [-1001.21, -1011.21, -1016.21, -1021.21],
        },
        Orientation {
            label: "skew (1,2,2), yref +x",
            node_j: [l / 3.0, 2.0 * l / 3.0, 2.0 * l / 3.0],
            y_reference: [1.0, 0.0, 0.0],
            skew: true,
            rv7_rotational_log2: -494.05,
            rv7_coupling_log2: [-1002.00, -1012.00, -1017.00, -1022.00],
        },
    ]
}

fn element(e: f64, g: f64, o: &Orientation) -> FrameElement {
    let (a, i, j) = section();
    FrameElement::new(
        FrameNode::new(0, [0.0; 3]).unwrap(),
        FrameNode::new(1, o.node_j).unwrap(),
        FrameSection::new(e, g, a, i, i, j).unwrap(),
        o.y_reference,
    )
    .unwrap()
}

// ------------------------------------------------ main's pre-K2a formation

const AXIAL: usize = 0;
const TORSION: usize = 1;
const BY6: usize = 3;
const BY4: usize = 4;
const BY2: usize = 5;
const BZ6: usize = 7;
const BZ4: usize = 8;
const BZ2: usize = 9;

/// Main's pre-K2a coefficients, operation for operation (FK
/// `local_stiffness` at 134eefc24, `lib.rs:716-728`): L², L³ = L²·L, then
/// (E·A)/L, (G·J)/L, ((k·E)·I)/Lⁿ for k = 12, 6, 4, 2, y before z. Nothing is
/// checked.
fn pre_k2a_coefficients(p: &FrameProperties) -> [f64; 10] {
    let s = p.section;
    let (e, g, area, iy, iz, j) = (
        s.elastic_modulus,
        s.shear_modulus,
        s.area,
        s.second_moment_y,
        s.second_moment_z,
        s.torsion_constant,
    );
    let length = p.length;
    let length2 = length * length;
    let length3 = length2 * length;
    [
        e * area / length,
        g * j / length,
        12.0 * e * iy / length3,
        6.0 * e * iy / length2,
        4.0 * e * iy / length,
        2.0 * e * iy / length,
        12.0 * e * iz / length3,
        6.0 * e * iz / length2,
        4.0 * e * iz / length,
        2.0 * e * iz / length,
    ]
}

/// Main's pre-K2a layout (FK `local_stiffness`, `set_symmetric`,
/// `add_bending_z`, `add_bending_y` and `add_terms` at 134eefc24,
/// `lib.rs:730-741`), from the given coefficients.
fn pre_k2a_local(c: &[f64; 10]) -> Matrix12 {
    let mut k = [[0.0; ELEMENT_DOF]; ELEMENT_DOF];
    let n = DOF_PER_NODE;
    k[UX][UX + n] = -c[AXIAL];
    k[UX + n][UX] = -c[AXIAL];
    k[UX][UX] = c[AXIAL];
    k[UX + n][UX + n] = c[AXIAL];
    k[RX][RX + n] = -c[TORSION];
    k[RX + n][RX] = -c[TORSION];
    k[RX][RX] = c[TORSION];
    k[RX + n][RX + n] = c[TORSION];
    let add = |k: &mut Matrix12, idx: [usize; 4], t: [[f64; 4]; 4]| {
        for r in 0..4 {
            for col in 0..4 {
                k[idx[r]][idx[col]] += t[r][col];
            }
        }
    };
    let (z12, z6, z4, z2) = (c[6], c[7], c[8], c[9]);
    add(
        &mut k,
        [UY, RZ, UY + n, RZ + n],
        [
            [z12, z6, -z12, z6],
            [z6, z4, -z6, z2],
            [-z12, -z6, z12, -z6],
            [z6, z2, -z6, z4],
        ],
    );
    let (y12, y6, y4, y2) = (c[2], c[3], c[4], c[5]);
    add(
        &mut k,
        [UZ, RY, UZ + n, RY + n],
        [
            [y12, -y6, -y12, -y6],
            [-y6, y4, y6, y2],
            [-y12, y6, y12, y6],
            [-y6, y2, y6, y4],
        ],
    );
    k
}

/// M03's element-entry floor: the least coefficient c whose axis-aligned
/// bound g·(c/(1 − g) + c)·(1 + 64ε) is normal.
fn floor() -> f64 {
    let g = gamma(24);
    f64::MIN_POSITIVE / (g * (1.0 / (1.0 - g) + 1.0) * (1.0 + 64.0 * f64::EPSILON))
}

// ------------------------------------------------------- exact references

/// (m, e) with x = m·2^e exactly, for a finite nonnegative binary64 x.
fn decompose(x: f64) -> (u128, i32) {
    assert!(x.is_finite() && x >= 0.0);
    let bits = x.to_bits();
    let exponent = ((bits >> 52) & 0x7ff) as i32;
    let fraction = (bits & ((1u64 << 52) - 1)) as u128;
    if exponent == 0 {
        (fraction, -1074)
    } else {
        (fraction | (1u128 << 52), exponent - 1075)
    }
}

/// |formed − k·E·I/L^n| / (k·E·I/L^n) with L = 2^LENGTH_EXPONENT. The
/// difference and the reference are exact integers (times one power of two);
/// only their quotient is formed in binary64, to about 3e-16 of the error
/// itself.
fn exact_relative_error(formed: f64, k: u128, e: f64, i: f64, n: i32) -> f64 {
    let (me, ee) = decompose(e);
    let (mi, ei) = decompose(i);
    let exact = k * me * mi;
    let exact_exponent = ee + ei - n * LENGTH_EXPONENT;
    if formed == 0.0 {
        return 1.0;
    }
    let (mf, ef) = decompose(formed);
    let base = exact_exponent.min(ef);
    let shift = |m: u128, by: i32| {
        let shifted = m << by;
        assert_eq!(shifted >> by, m, "exact reference overflow");
        shifted
    };
    let x = shift(exact, exact_exponent - base);
    let f = shift(mf, ef - base);
    x.abs_diff(f) as f64 / x as f64
}

// -------------------------------------------------------------- helpers

/// The prepared case: the element, main's pre-K2a local matrix, FK's
/// transform and FK's global matrix of it.
struct Member {
    properties: FrameProperties,
    coefficients: [f64; 10],
    transform: Matrix12,
    global: Matrix12,
}

fn member(case: &Case, o: &Orientation) -> Member {
    let element = element(case.e, G, o);
    let properties = element.properties().unwrap();
    let coefficients = pre_k2a_coefficients(&properties);
    let local = pre_k2a_local(&coefficients);
    let orientation = element.orientation().unwrap();
    Member {
        properties,
        coefficients,
        transform: orientation.transformation_matrix(),
        global: transform_global_stiffness(&local, &orientation),
    }
}

fn is_rotation(dof: usize) -> bool {
    dof % DOF_PER_NODE >= RX
}

fn least_positive(bounds: &Matrix12, keep: impl Fn(usize, usize) -> bool) -> f64 {
    let mut least = f64::INFINITY;
    for (p, row) in bounds.iter().enumerate() {
        for (q, &b) in row.iter().enumerate() {
            if keep(p, q) && b > 0.0 {
                least = least.min(b);
            }
        }
    }
    least
}

fn close_log2(value: f64, rv7: f64, ctx: &str) {
    assert!(
        (value.log2() - rv7).abs() <= 0.005,
        "{ctx}: log2 {} against RV7's {rv7}",
        value.log2()
    );
}

// ------------------------------------------------------------------ tests

/// Preconditions for everything below: the replica is main's formation (it
/// equals K2a's accepted formation bit for bit on normal inputs, which K2a
/// forms with main's operations), every member here has length exactly
/// 2^-39 m, and K2a refuses every case by name at formation.
#[test]
fn m03_skew_pin_preconditions_hold() {
    // A normal section on each orientation, at the same length.
    for o in orientations() {
        let normal = element(2.0e11, 8.0e10, &o);
        let p = normal.properties().unwrap();
        let replica = pre_k2a_local(&pre_k2a_coefficients(&p));
        let formed = normal.local_stiffness().unwrap();
        for (a, b) in replica.iter().flatten().zip(formed.iter().flatten()) {
            assert_eq!(a.to_bits(), b.to_bits(), "{}: replica", o.label);
        }
        assert_eq!(
            transform_global_stiffness(&replica, &normal.orientation().unwrap()),
            normal.global_stiffness().unwrap(),
            "{}",
            o.label
        );
    }
    for case in cases() {
        for o in orientations() {
            let m = member(&case, &o);
            assert_eq!(
                m.properties.length.to_bits(),
                p2(LENGTH_EXPONENT).to_bits(),
                "{} {}: length",
                case.label,
                o.label
            );
            // K2a refuses the formation by name (the first checked site).
            assert_eq!(
                local_stiffness(m.properties),
                Err(FrameKernelError::NumericalRange {
                    name: "12EIy/L^3: (12*E)*Iy"
                }),
                "{} {}",
                case.label,
                o.label
            );
        }
    }
    // The floor is about 2^-974.585, and it is the axis-aligned member's
    // floor behaviourally: one coefficient at 4EIz/L's diagonal position,
    // just above or just below it.
    let f = floor();
    assert!((f.log2() + 974.585).abs() < 1.0e-3, "{}", f.log2());
    let axis = element(2.0e11, 8.0e10, &orientations()[0]);
    let t = axis.orientation().unwrap().transformation_matrix();
    for (scale, accepted) in [(1.0 + p2(-30), true), (1.0 - p2(-30), false)] {
        let mut single = [[0.0; ELEMENT_DOF]; ELEMENT_DOF];
        single[RZ][RZ] = f * scale;
        let outcome = transform_roundoff(&single, &t).map(|_| ());
        assert_eq!(
            outcome.is_ok(),
            accepted,
            "floor scale {scale}: {outcome:?}"
        );
        if !accepted {
            assert_eq!(outcome, Err(RANGE));
        }
    }
}

/// The kernel pin on RV7's confirmed cases (K2A_REVIEW.md B1): the outcome
/// per case and orientation, and the figures that make it a pin.
#[test]
fn m03_skew_pin_rv7_cases_outcomes_and_figures() {
    let (_, i, _) = section();
    let f = floor();
    let mut accepted_rows = 0;
    for case in cases() {
        let ctx = case.label;
        let e = case.e;
        let c = pre_k2a_coefficients(&member(&case, &orientations()[0]).properties);
        // Intermediates, as main forms them.
        let products = [12.0 * e * i, 6.0 * e * i, 4.0 * e * i, 2.0 * e * i];
        eprintln!(
            "I9 {ctx}: E={e:e} bits={:#018x}; (12E)I={:e} (6E)I={:e} (4E)I={:e} (2E)I={:e}; \
             EA/L 2^{:.2} GJ/L 2^{:.2} 12EI/L3 2^{:.2} 6EI/L2 2^{:.2} 4EI/L 2^{:.2} 2EI/L 2^{:.2}",
            e.to_bits(),
            products[0],
            products[1],
            products[2],
            products[3],
            c[AXIAL].log2(),
            c[TORSION].log2(),
            c[2].log2(),
            c[BY6].log2(),
            c[BY4].log2(),
            c[BY2].log2()
        );
        if case.skew_accepted {
            accepted_rows += 1;
            // Every (kE)·I is subnormal and nonzero; 4EI/L and 2EI/L are
            // normal but below the floor; 6EI/L² is above it.
            assert!(products.iter().all(|v| v.is_subnormal()), "{ctx}");
            for k in [BY4, BY2, BZ4, BZ2] {
                assert!(c[k].is_normal() && c[k] < f, "{ctx}: coefficient {k}");
            }
            assert!(c[BY6] > f && c[BZ6] > f, "{ctx}");
            // The exact relative errors of 2EI/L and 4EI/L: RV7's figures
            // at their stated precision, and the Fraction cross-check's to
            // five digits.
            let e2 = exact_relative_error(c[BY2], 2, e, i, 1);
            let e4 = exact_relative_error(c[BY4], 4, e, i, 1);
            assert_eq!(e2, exact_relative_error(c[BZ2], 2, e, i, 1));
            assert_eq!(e4, exact_relative_error(c[BZ4], 4, e, i, 1));
            eprintln!("I9 {ctx}: exact relative error 2EI/L {e2:.6e}, 4EI/L {e4:.6e}");
            let (rv7_2, rv7_4) = case.rv7_errors.unwrap();
            let (x2, x4) = case.exact_errors.unwrap();
            for (value, stated, digits, what) in [
                (e2, rv7_2, significant(rv7_2), "2EI/L against RV7"),
                (e4, rv7_4, significant(rv7_4), "4EI/L against RV7"),
                (e2, x2, 5, "2EI/L against the Fraction reference"),
                (e4, x4, 5, "4EI/L against the Fraction reference"),
            ] {
                let half_unit = 0.5 * 10f64.powi(stated.log10().floor() as i32 - digits + 1);
                assert!(
                    (value - stated).abs() <= half_unit,
                    "{ctx}: {what}: {value:e} against {stated:e}"
                );
            }
            // 6EI/L² and 12EI/L³ are accurate here (E was chosen so that
            // (12E)·I and (6E)·I round to powers of two).
            assert!(exact_relative_error(c[BY6], 6, e, i, 2) < 1.0e-15, "{ctx}");
            assert!(exact_relative_error(c[2], 12, e, i, 3) < 1.0e-15, "{ctx}");
        } else {
            // 6EI/L² is below the floor.
            assert!(c[BY6] < f && c[BZ6] < f, "{ctx}");
        }
        for (index, o) in orientations().iter().enumerate() {
            let m = member(&case, o);
            let local = pre_k2a_local(&m.coefficients);
            let outcome = transform_roundoff(&local, &m.transform);
            let ctx = format!("{} {}", case.label, o.label);
            let expected = o.skew && case.skew_accepted;
            match (&outcome, expected) {
                (Ok(r), true) => {
                    let b = &r.absolute_roundoff;
                    let least = least_positive(b, |_, _| true);
                    let coupling = least_positive(b, |p, q| is_rotation(p) != is_rotation(q));
                    let rv7_coupling = least_positive(b, |p, q| p < RX && (RX..=RZ).contains(&q));
                    let rotational = b[RX..=RZ]
                        .iter()
                        .flat_map(|row| row[RX..=RZ].iter())
                        .copied()
                        .fold(f64::INFINITY, f64::min);
                    eprintln!(
                        "I9 {ctx}: ACCEPTED; least bound 2^{:.2}; least rotational-block \
                         bound 2^{:.2}; least positive coupling-block bound 2^{:.2}",
                        least.log2(),
                        rotational.log2(),
                        rv7_coupling.log2()
                    );
                    // 6EI/L² is the limiting coefficient: the least bound is
                    // in the coupling block, whose entries are formed only
                    // from ±6EI/L² terms, and it is RV7's figure. The
                    // rotational block's least bound is far above the floor,
                    // lifted by GJ/L.
                    assert_eq!(least.to_bits(), coupling.to_bits(), "{ctx}");
                    assert_eq!(least.to_bits(), rv7_coupling.to_bits(), "{ctx}");
                    let row = cases().iter().position(|k| k.label == case.label).unwrap();
                    close_log2(least, o.rv7_coupling_log2[row], &ctx);
                    close_log2(rotational, o.rv7_rotational_log2, &ctx);
                    assert!(least.is_normal() && least < p2(-1000), "{ctx}");
                }
                (Err(error), false) => {
                    eprintln!("I9 {ctx}: REFUSED {error:?}");
                    assert_eq!(error, &RANGE, "{ctx}");
                }
                (outcome, expected) => panic!(
                    "{ctx}: RV7's table says {}, got {outcome:?} (orientation {index})",
                    if expected { "accepted" } else { "refused" }
                ),
            }
        }
    }
    // Not vacuous: four rows are accepted on both skew members.
    assert_eq!(accepted_rows, 4);
}

/// The number of significant digits `value` is stated to (1.1e-13 has 2).
fn significant(value: f64) -> i32 {
    let text = format!("{value:e}");
    let mantissa = text.split('e').next().unwrap();
    mantissa.chars().filter(|c| c.is_ascii_digit()).count() as i32
}

/// 6EI/L² is what decides a skew member in RV7's range, and each coefficient
/// decides an axis-aligned one. Only the 6EI/L² coefficients are moved across
/// the floor; 4EI/L and 2EI/L are left as formed, below it.
/// - Lowering 6EI/L² refuses every accepted skew row with `Range`.
/// - Raising it makes the refused skew rows acceptable to the floor. Then a
///   second limit shows: `transform_roundoff` checks each product
///   |T_ki|·magnitude_kj of its inherited sum, and a term 2EI/L·|T|·|T| below
///   2^-1022 underflows. On (1,2,2), whose least nonzero |T| is 0.2357, the
///   2^-1055 row's 2EI/L (2^-1018.58) gives 2^-1022.75, so that member stays
///   refused, as `Range("product overflow or underflow")`. On (1,1,1) (least
///   |T| 0.4082) the term is 2^-1021.17 and the member is accepted. In RV7's
///   own rows the coupling block's bound refuses first, with `Range`.
/// - The axis member is accepted exactly when every nonzero coefficient is at
///   or above the floor.
#[test]
fn m03_skew_pin_6eil2_is_the_limiting_coefficient() {
    let f = floor();
    let product = StructuralError::Range("product overflow or underflow");
    let mut accepted_after_raise = 0;
    for case in cases() {
        for o in orientations() {
            let m = member(&case, &o);
            let mut c = m.coefficients;
            // Across the floor, by a factor of eight either way (the least
            // coupling bound is about 2g·6EI/L² times 0.57 on (1,1,1) and
            // 0.33 on (1,2,2)).
            let moved = if c[BY6] > f { 0.125 * f } else { 8.0 * f };
            c[BY6] = moved;
            c[BZ6] = moved;
            let outcome = transform_roundoff(&pre_k2a_local(&c), &m.transform).map(|_| ());
            let ctx = format!("{} {} with 6EI/L^2 = {moved:e}", case.label, o.label);
            // The least nonzero |T| entry, and whether a 1/L coefficient's
            // product term 2EI/L·|T|·|T| (or 4EI/L's) falls below 2^-1022.
            let t_min = m
                .transform
                .iter()
                .flatten()
                .map(|v| v.abs())
                .filter(|&v| v > 0.0)
                .fold(f64::INFINITY, f64::min);
            let product_underflow = [c[BY2], c[BY4], c[BZ2], c[BZ4]]
                .iter()
                .any(|&v| v > 0.0 && v * t_min * t_min < f64::MIN_POSITIVE);
            let expected = if !o.skew {
                if c.iter().all(|&v| v == 0.0 || v >= f) {
                    Ok(())
                } else {
                    Err(RANGE)
                }
            } else if case.skew_accepted {
                Err(RANGE)
            } else if product_underflow {
                Err(product.clone())
            } else {
                Ok(())
            };
            eprintln!("I9 counterfactual {ctx}: {outcome:?} (least |T| {t_min:.4})");
            assert_eq!(outcome, expected, "{ctx}");
            if o.skew && !case.skew_accepted && outcome.is_ok() {
                accepted_after_raise += 1;
            }
        }
    }
    // Not vacuous: raising 6EI/L² alone turns three of the four refused skew
    // cases into acceptances; the fourth is the product-underflow case.
    assert_eq!(accepted_after_raise, 3);
    let one22 = orientations()[2];
    let m = member(&cases()[4], &one22);
    let mut c = m.coefficients;
    c[BY6] = 8.0 * f;
    c[BZ6] = 8.0 * f;
    assert_eq!(
        transform_roundoff(&pre_k2a_local(&c), &m.transform).map(|_| ()),
        Err(product)
    );
    // Not vacuous on the axis member: S6a (4EI/L = 2EI/L = 0) is accepted
    // there once 6EI/L² is above the floor.
    let s6a = cases()[5];
    let m = member(&s6a, &orientations()[0]);
    let mut c = m.coefficients;
    assert_eq!((c[BY4], c[BY2]), (0.0, 0.0));
    c[BY6] = 8.0 * f;
    c[BZ6] = 8.0 * f;
    assert!(transform_roundoff(&pre_k2a_local(&c), &m.transform).is_ok());
}

// ------------------------------------------------ both representations

/// The formation allowances and operation counts of one element on DOFs
/// 0..12, as the structural adapter's `AssemblyEvidence::new` records them:
/// the element's `transform_roundoff` bounds, its counts plus one scatter,
/// and then gamma(1)·|value| for the entry's one contribution.
fn allowance(r: &TransformationRoundoff, global: &Matrix12, p: usize, q: usize) -> (f64, usize) {
    (
        (0.0 + r.absolute_roundoff[p][q]) + gamma(1) * (0.0 + global[p][q].abs()),
        r.operation_counts[p][q] + 1,
    )
}

/// One cantilever (N0 anchored, N1 free) with a global RZ moment at N1 of
/// 0.1·EI/L, as RV7's product probe loads it.
struct Loaded {
    force: Vec<f64>,
    free: Vec<usize>,
    prescribed: Vec<(usize, f64)>,
    contributions: Vec<StiffnessContribution>,
}

fn loaded(case: &Case, m: &Member) -> Loaded {
    let (_, i, _) = section();
    let l = m.properties.length;
    let mut force = vec![0.0; 12];
    force[DOF_PER_NODE + RZ] = 0.1 * (case.e * p2(600) * i / l) * p2(-600);
    let map = element_dof_map(0, 1);
    let mut contributions = Vec::new();
    for p in 0..ELEMENT_DOF {
        for q in 0..ELEMENT_DOF {
            contributions.push(StiffnessContribution {
                row: map[p],
                col: map[q],
                value: m.global[p][q],
            });
        }
    }
    Loaded {
        force,
        free: (6..12).collect(),
        prescribed: (0..6).map(|d| (d, 0.0)).collect(),
        contributions,
    }
}

/// The skyline of a free-block matrix (given by entry) in `order`.
fn profile(
    n: usize,
    nonzero: impl Fn(usize, usize) -> bool,
    reversed: bool,
) -> (Vec<usize>, Vec<usize>) {
    let order: Vec<usize> = if reversed {
        (0..n).rev().collect()
    } else {
        (0..n).collect()
    };
    let first = (0..n)
        .map(|a| (0..a).find(|&b| nonzero(order[a], order[b])).unwrap_or(a))
        .collect();
    (order, first)
}

type Outcome = Result<Result<StructuralSolution, StructuralError>, StructuralError>;

/// The dense store of one element on DOFs 0..12: the matrix (accumulated
/// with `+=` from zeros, as the dense assembly does), and its allowances and
/// counts.
type DenseParts = (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Vec<usize>>);

fn dense_parts(m: &Member, r: &TransformationRoundoff) -> DenseParts {
    let map = element_dof_map(0, 1);
    let mut k = vec![vec![0.0; 12]; 12];
    let mut roundoff = vec![vec![0.0; 12]; 12];
    let mut counts = vec![vec![0usize; 12]; 12];
    for p in 0..ELEMENT_DOF {
        for q in 0..ELEMENT_DOF {
            k[map[p]][map[q]] += m.global[p][q];
            (roundoff[map[p]][map[q]], counts[map[p]][map[q]]) = allowance(r, &m.global, p, q);
        }
    }
    (k, roundoff, counts)
}

/// Today's dense M03: the element's evidence (the floor), then the dense
/// gate, completed by a skyline factor in the given order or, with
/// `cholesky`, by the product's dense mode (dense Cholesky).
fn dense_m03(case: &Case, m: &Member, reversed: bool, cholesky: bool) -> Outcome {
    let r = transform_roundoff(&pre_k2a_local(&m.coefficients), &m.transform)?;
    let load = loaded(case, m);
    let (k, roundoff, counts) = dense_parts(m, &r);
    let system = StructuralSystem {
        stiffness: &k,
        force: &load.force,
        free_dofs: &load.free,
        prescribed: &load.prescribed,
        contributions: Some(load.contributions.as_slice()),
        symmetry: Some(SymmetryEvidence {
            absolute_roundoff: &roundoff,
            operation_counts: &counts,
            basis: r.basis,
        }),
    };
    if cholesky {
        return Ok(solve_structural_dense(&system));
    }
    Ok((|| {
        let prepared = prepare_structural(&system)?;
        let matrix = prepared.matrix();
        let (order, first) = profile(matrix.len(), |a, b| matrix[a][b] != 0.0, reversed);
        finish_structural(&factor_structural_profile(&prepared, &order, &first)?)
    })())
}

/// K1's pattern path: the element's evidence (the floor), K1's sparse
/// assembly of the realized matrix as an explicit block, one allowance per
/// pattern entry, and the pattern gate with a skyline factor in the given
/// order.
fn pattern_m03(case: &Case, m: &Member, reversed: bool) -> Outcome {
    let r = transform_roundoff(&pre_k2a_local(&m.coefficients), &m.transform)?;
    let load = loaded(case, m);
    let k = assemble_sparse_stiffness(
        2,
        &[],
        &[],
        &[StiffnessBlock {
            node_i: 0,
            node_j: 1,
            stiffness: m.global,
        }],
        &[],
        &SparseAssemblyOptions::new(),
    )
    .unwrap();
    let pattern = k.pattern();
    let mut roundoff = vec![0.0; pattern.entry_count()];
    let mut counts = vec![0usize; pattern.entry_count()];
    for row in 0..pattern.dimension() {
        for index in pattern.row_range(row) {
            // One element on DOFs 0..12: the global DOF is the element DOF.
            (roundoff[index], counts[index]) = allowance(&r, &m.global, row, pattern.column(index));
        }
    }
    let system = SparseStructuralSystem::new(
        &k,
        &load.force,
        &load.free,
        &load.prescribed,
        Some(load.contributions.as_slice()),
        Some(SparseSymmetryEvidence {
            absolute_roundoff: &roundoff,
            operation_counts: &counts,
            basis: r.basis,
        }),
    );
    Ok((|| {
        let prepared = prepare_sparse_structural(&system)?;
        let n = prepared.dimension();
        let mut stored = vec![vec![false; n]; n];
        for (a, b, v) in prepared.lower_entries() {
            stored[a][b] = v != 0.0;
            stored[b][a] = v != 0.0;
        }
        let (order, first) = profile(n, |a, b| stored[a][b], reversed);
        finish_sparse_structural(&factor_sparse_structural_profile(
            &prepared, &order, &first,
        )?)
    })())
}

/// The same outcomes through both representations: today's dense M03 and
/// K1's pattern path, with the same allowances, give the same acceptance or
/// refusal with the same error text, and the same gate outcome after an
/// acceptance, in `Debug` bytes, for a fixed order (natural and reversed).
/// A refusal comes from `transform_roundoff`, which forms the element's
/// evidence for either store (as the adapter's `EvidenceParts::new` does), so
/// it is the same by construction; the gate after an acceptance is where the
/// representations could differ.
#[test]
fn m03_skew_pin_dense_and_pattern_outcomes_are_identical() {
    let mut gate_runs = 0;
    for case in cases() {
        for o in orientations() {
            let m = member(&case, &o);
            let ctx = format!("{} {}", case.label, o.label);
            let expected = o.skew && case.skew_accepted;
            for reversed in [false, true] {
                let dense = dense_m03(&case, &m, reversed, false);
                let pattern = pattern_m03(&case, &m, reversed);
                assert_eq!(
                    format!("{dense:?}"),
                    format!("{pattern:?}"),
                    "{ctx} reversed={reversed}"
                );
                match (&dense, expected) {
                    (Err(error), false) => assert_eq!(error, &RANGE, "{ctx}"),
                    (Ok(gate), true) => {
                        gate_runs += 1;
                        eprintln!(
                            "I9 parity {ctx} reversed={reversed}: floor accepted; gate {}",
                            render_gate(gate)
                        );
                    }
                    (outcome, _) => panic!("{ctx}: floor outcome {outcome:?}"),
                }
            }
            // The product's dense mode (dense Cholesky): the same floor
            // outcome; its gate outcome is recorded.
            let cholesky = dense_m03(&case, &m, false, true);
            match (&cholesky, expected) {
                (Err(error), false) => assert_eq!(error, &RANGE, "{ctx}"),
                (Ok(gate), true) => eprintln!(
                    "I9 parity {ctx} dense Cholesky: floor accepted; gate {}",
                    render_gate(gate)
                ),
                (outcome, _) => panic!("{ctx}: Cholesky floor outcome {outcome:?}"),
            }
        }
    }
    // Not vacuous: 4 rows × 2 skew members × 2 orders reach the gate.
    assert_eq!(gate_runs, 16);
}

fn render_gate(gate: &Result<StructuralSolution, StructuralError>) -> String {
    match gate {
        Ok(s) => format!(
            "Ok quality={:?} u_N1={:?}",
            s.report.quality,
            &s.displacements[6..12]
        ),
        Err(e) => format!("Err {e:?}"),
    }
}

/// A behavioural guard for future work on M03's skew scope. It fails if
/// M03's element-entry floor starts refusing the accepted skew rows, or
/// starts accepting any axis-aligned row, so that any change to M03's scope
/// on skew members is deliberate and visible.
///
/// The accepted skew rows are pinned as today's **documented limitation**,
/// not as desired behaviour: on a skew member M03 accepts subnormal-derived
/// 4EI/L and 2EI/L below its floor, with errors up to about 1.2e-7 (K2a
/// `RETURN_ADDENDUM_1.md` §1.4). K2a refuses every one of these formations by
/// name before M03 runs; this guard is about M03 itself, on which later
/// slices (K2b's scaling, K5, and F1b's move of the product onto the pattern
/// path) lean. Whoever changes this outcome updates this guard and says why.
#[test]
fn m03_skew_scope_guard_documented_limitation_not_desired_behaviour() {
    let mut outcomes = Vec::new();
    for case in cases() {
        for o in orientations() {
            let m = member(&case, &o);
            let accepted = transform_roundoff(&pre_k2a_local(&m.coefficients), &m.transform)
                .map(|_| ())
                .map_err(|e| {
                    assert_eq!(e, RANGE, "{} {}", case.label, o.label);
                });
            outcomes.push((case.label, o.label, accepted.is_ok()));
        }
    }
    let accepted: Vec<_> = outcomes
        .iter()
        .filter(|o| o.2)
        .map(|o| (o.0, o.1))
        .collect();
    assert_eq!(
        accepted,
        vec![
            ("(12E)I=2^-1030", "skew (1,1,1), yref +z"),
            ("(12E)I=2^-1030", "skew (1,2,2), yref +x"),
            ("(12E)I=2^-1040", "skew (1,1,1), yref +z"),
            ("(12E)I=2^-1040", "skew (1,2,2), yref +x"),
            ("(12E)I=2^-1045", "skew (1,1,1), yref +z"),
            ("(12E)I=2^-1045", "skew (1,2,2), yref +x"),
            ("(12E)I=2^-1050", "skew (1,1,1), yref +z"),
            ("(12E)I=2^-1050", "skew (1,2,2), yref +x"),
        ],
        "M03's skew scope changed: every other case (all axis-aligned rows, \
         2^-1055 and S6a) is refused with Range(\"arithmetic outside normal range\")"
    );
    assert_eq!(outcomes.len(), 18);
}
