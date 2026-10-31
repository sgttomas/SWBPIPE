//! Re-formation (design §3.4 checks 1 to 3) against the frozen class (a)
//! references of `validation/references/f1_modal/mass_discrete/`.
//!
//! Source: `tests/fixtures/reformation_cases.txt`, written by
//! `reformation_cases.py` beside it from `mass_references.json` and
//! `eigen_references.json`, read-only (its header states the format). The
//! expectations are exact values (rationals, a + b·π, or 80-digit values)
//! carried as double-double pairs within 2⁻¹⁰⁶ relative of the reference.
//!
//! **Tolerance model.** A double-double operation has relative error a small
//! multiple of 2⁻¹⁰⁶ ≈ 1.2·10⁻³² (`dd`), and p128 one of 2⁻¹²⁸. A re-formed
//! entry is a chain of at most about 60 such operations from its binary64
//! leaves (section, element formula, the rotation's 27 products), each of
//! magnitude at most the matrix's largest entry, so its error is below
//! 60·4·2⁻¹⁰⁶ ≈ 3·10⁻³⁰ of that largest entry; the reference adds 2⁻¹⁰⁶.
//! Every comparison is therefore normwise: |x̂ − x| ≤ `NORMWISE`·max|x| over
//! the matrix (or the quantity), with `NORMWISE` = 10⁻²⁹, three times that
//! bound. Entries formed by cancellation (exact zeros of an inclined member,
//! J_c,22 ≈ α⁵/20 of a small arc) are held to the same normwise bound, which
//! is the statement the design makes ("well-scaled entries"); the small-arc
//! case is also checked entrywise in its local frame through the lumps.
//! The measured largest discrepancies are printed (run with
//! `--nocapture`) and reported in the README.

use std::collections::BTreeMap;

use open_pipe_stress_frame_kernel::structural::element_reformation::{
    reform_straight, StraightOperands, WideOperand,
};
use open_pipe_stress_modal_eigen::clusters::cluster_ids;
use open_pipe_stress_modal_eigen::dd::Dd;
use open_pipe_stress_modal_eigen::dense::{
    established_finite_mode_count, solve_dense_pencil, DenseEigenSolution, DenseOptions,
};
use open_pipe_stress_modal_eigen::orthogonality::{m_orthogonality, m_orthogonality_reformed};
use open_pipe_stress_modal_eigen::reformation::mass::{
    component_mass, component_range, ComponentBody, ComponentRangeOperand,
};
use open_pipe_stress_modal_eigen::reformation::section::{
    derive_section, FormedSection, Insulation, MassBasis, PipeSection,
};
use open_pipe_stress_modal_eigen::reformation::{
    cluster_error_estimates, formation_difference, reform_element_mass, residual_estimate,
    ArcOperand, ElementOperand, Moduli, ReformationFailure, ReformedPencil, SpringOperand,
    StraightOperand,
};
use open_pipe_stress_modal_eigen::SymmetricMatrix;
use open_pipe_stress_sparse_direct::SymmetricMatrixEntry;

const NORMWISE: f64 = 1e-29;

/// An empirical threshold for this fixture, not a derived bound and not the
/// production tolerance (that is measured under DEC-026): the dense modes of
/// the eleven eigen cases are M̂-orthogonal within it. Measured largest
/// value 1.3·10⁻⁸ (printed by the reference test). A general first-order
/// bound n·u·√(λ_iλ_j)/λ₁ was withdrawn after review round 2: it omits the
/// reduction and back-transform errors (see
/// `orthogonality_of_a_close_pencil_is_finite_and_reported`).
const FIXTURE_ORTHOGONALITY_THRESHOLD: f64 = 1e-7;

/// Test-local mass-pivot threshold τ_m of the finite-mode count (as in
/// `sturm_counts.rs`).
const MASS_PIVOT_RELATIVE_THRESHOLD: f64 = 1e-12;

/// η of a correctly computed pair: below 10⁻¹², under which the design
/// places no residual threshold τ_r (§3.4 check 2, "τ_r lies above
/// 10⁻¹²"), so a test-local bound there never demotes a correct mode. The
/// measured values are printed (about 10⁻¹⁴ at most).
const ETA_CORRECT: f64 = 1e-12;

/// Test-local resolution-floor parameters c_μ and κ_μ (design §3.2 item 5);
/// the production values are recorded under DEC-026, not here. With
/// c_μ = 1, the review-round-3 artifact lies about 2× below δ_μ and the
/// smallest retained μ of every admitted fixture case at least 7.4·10² above
/// it (E-RIGID-SCALE-1e6), so κ_μ = 10 sits between them (README).
const RESOLUTION_FLOOR_CONSTANT: f64 = 1.0;
const RESOLUTION_FLOOR_MULTIPLE: f64 = 10.0;

/// The smallest retained μ over κ_μ·δ_μ: how far the resolution floor is
/// below the retained spectrum (+∞ with a zero floor or no mode).
fn floor_margin(solution: &DenseEigenSolution) -> f64 {
    let smallest = solution.mu.iter().copied().fold(f64::INFINITY, f64::min);
    smallest / (RESOLUTION_FLOOR_MULTIPLE * solution.resolution_floor)
}

/// Test-local dense options (as in `sturm_counts.rs`).
fn dense_options() -> DenseOptions {
    DenseOptions {
        infinite_mu_relative_threshold: 1e-12,
        mass_pivot_relative_threshold: MASS_PIVOT_RELATIVE_THRESHOLD,
        resolution_floor_constant: RESOLUTION_FLOOR_CONSTANT,
        resolution_floor_multiple: RESOLUTION_FLOOR_MULTIPLE,
        max_ql_iterations_per_eigenvalue: 30,
    }
}

type Pair = (usize, usize);

#[derive(Debug, Default)]
struct Case {
    id: String,
    basis: String,
    nodes: usize,
    elements: Vec<ElementOperand>,
    /// Member k of the fixture to its element index.
    member: BTreeMap<usize, usize>,
    sections: BTreeMap<(usize, String), Dd>,
    emass: BTreeMap<usize, BTreeMap<Pair, Dd>>,
    lumps: BTreeMap<(usize, usize), BTreeMap<Pair, Dd>>,
    cmass: BTreeMap<usize, BTreeMap<Pair, Dd>>,
    mfull: BTreeMap<Pair, Dd>,
    free: Vec<usize>,
    k: BTreeMap<Pair, Dd>,
    m: BTreeMap<Pair, Dd>,
    eigenvalues: Vec<Dd>,
}

impl Case {
    fn basis(&self) -> MassBasis {
        match self.basis.as_str() {
            "PF" => MassBasis::EffectiveWall,
            "EX" => MassBasis::NominalWallPressureBore,
            other => panic!("basis {other}"),
        }
    }
}

fn bits(s: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(s, 16).unwrap())
}
fn opt(s: &str) -> Option<f64> {
    (s != "-").then(|| bits(s))
}
fn dd(hi: &str, lo: &str) -> Dd {
    Dd {
        hi: bits(hi),
        lo: bits(lo),
    }
}
fn v3(f: &[&str]) -> [f64; 3] {
    [bits(f[0]), bits(f[1]), bits(f[2])]
}
fn section(f: &[&str]) -> PipeSection {
    PipeSection {
        outside_diameter: bits(f[0]),
        wall_thickness: bits(f[1]),
        mill_tolerance: bits(f[2]),
        corrosion_allowance: opt(f[3]),
        material_density: bits(f[4]),
        contents_density: opt(f[5]),
        insulation: opt(f[6]).map(|thickness| Insulation {
            thickness,
            density: opt(f[7]).unwrap(),
        }),
    }
}

fn fixture() -> Vec<Case> {
    let text = include_str!("fixtures/reformation_cases.txt");
    let mut cases = Vec::new();
    let mut c: Option<Case> = None;
    let mut coords: Vec<[f64; 3]> = Vec::new();
    for line in text.lines() {
        let f: Vec<&str> = line.split(' ').collect();
        let u = |i: usize| -> usize { f[i].parse().unwrap() };
        match f[0] {
            "source" => {}
            "case" => {
                c = Some(Case {
                    id: f[1].into(),
                    basis: f[2].into(),
                    nodes: u(3),
                    ..Case::default()
                });
                coords.clear();
            }
            "node" => coords.push(v3(&f[2..5])),
            "straight" | "arc" => {
                let case = c.as_mut().unwrap();
                let (i, j) = (u(2), u(3));
                let element = if f[0] == "straight" {
                    ElementOperand::Straight(StraightOperand {
                        node_i: i,
                        node_j: j,
                        coordinates_i: coords[i],
                        coordinates_j: coords[j],
                        y_reference: v3(&f[4..7]),
                        section: section(&f[7..15]),
                        moduli: Moduli {
                            elastic_modulus: bits(f[15]),
                            shear_modulus: bits(f[16]),
                            stiffness_scale: opt(f[17]),
                        },
                        carries_mass: f[18] == "1",
                        formed_section: None,
                    })
                } else {
                    let k = bits(f[18]);
                    ElementOperand::Arc(ArcOperand {
                        node_i: i,
                        node_j: j,
                        coordinates_i: coords[i],
                        coordinates_j: coords[j],
                        radius: bits(f[4]),
                        y_reference: v3(&f[5..8]),
                        section: section(&f[8..16]),
                        moduli: Moduli {
                            elastic_modulus: bits(f[16]),
                            shear_modulus: bits(f[17]),
                            stiffness_scale: None,
                        },
                        in_plane_flexibility_factor: k,
                        out_of_plane_flexibility_factor: k,
                        carries_mass: f[19] == "1",
                        formed_section: None,
                    })
                };
                case.member.insert(u(1), case.elements.len());
                case.elements.push(element);
            }
            "body" => c
                .as_mut()
                .unwrap()
                .elements
                .push(ElementOperand::ComponentBody(ComponentBody {
                    node: u(1),
                    weight: bits(f[2]),
                    acceleration: bits(f[3]),
                    center_of_gravity: v3(&f[4..7]),
                })),
            "spring" => {
                c.as_mut()
                    .unwrap()
                    .elements
                    .push(ElementOperand::Spring(SpringOperand::Axis {
                        dof: u(1),
                        stiffness: bits(f[2]),
                    }))
            }
            "section" => {
                let v = dd(f[3], f[4]);
                c.as_mut().unwrap().sections.insert((u(1), f[2].into()), v);
            }
            "emass" => {
                let v = dd(f[4], f[5]);
                let case = c.as_mut().unwrap();
                case.emass.entry(u(1)).or_default().insert((u(2), u(3)), v);
            }
            "lump" => {
                let v = dd(f[5], f[6]);
                let case = c.as_mut().unwrap();
                case.lumps
                    .entry((u(1), u(2)))
                    .or_default()
                    .insert((u(3), u(4)), v);
            }
            "cmass" => {
                let v = dd(f[4], f[5]);
                let case = c.as_mut().unwrap();
                case.cmass.entry(u(1)).or_default().insert((u(2), u(3)), v);
            }
            "mfull" => {
                let v = dd(f[3], f[4]);
                c.as_mut().unwrap().mfull.insert((u(1), u(2)), v);
            }
            "free" => {
                c.as_mut().unwrap().free = f[1..].iter().map(|s| s.parse().unwrap()).collect()
            }
            "k" | "m" => {
                let v = dd(f[3], f[4]);
                let case = c.as_mut().unwrap();
                let target = if f[0] == "k" {
                    &mut case.k
                } else {
                    &mut case.m
                };
                target.insert((u(1), u(2)), v);
            }
            "eig" => {
                let v = dd(f[2], f[3]);
                c.as_mut().unwrap().eigenvalues.push(v);
            }
            "end" => cases.push(c.take().unwrap()),
            other => panic!("unknown fixture record {other}"),
        }
    }
    cases
}

fn mass_cases() -> Vec<Case> {
    fixture()
        .into_iter()
        .filter(|c| c.free.is_empty())
        .collect()
}
fn eigen_cases() -> Vec<Case> {
    fixture()
        .into_iter()
        .filter(|c| !c.free.is_empty())
        .collect()
}

/// max |x̂ − x| / max |x| over a set of positions (both maps' keys).
fn normwise(actual: &BTreeMap<Pair, Dd>, expected: &BTreeMap<Pair, Dd>) -> f64 {
    let scale = expected
        .values()
        .fold(0.0f64, |m, v| m.max(v.abs().to_f64()));
    let mut worst = 0.0f64;
    for key in actual.keys().chain(expected.keys()) {
        let a = actual.get(key).copied().unwrap_or(Dd::ZERO);
        let e = expected.get(key).copied().unwrap_or(Dd::ZERO);
        worst = worst.max(a.sub(e).abs().to_f64() / scale);
    }
    worst
}

fn square<const N: usize>(
    m: &open_pipe_stress_modal_eigen::reformation::ElementMatrix,
) -> BTreeMap<Pair, Dd> {
    let mut out = BTreeMap::new();
    for r in 0..N {
        for c in 0..N {
            let v = m.get(r, c);
            if !v.is_zero() {
                out.insert((r, c), v);
            }
        }
    }
    out
}

fn upper(full: &BTreeMap<Pair, Dd>) -> BTreeMap<Pair, Dd> {
    full.iter()
        .filter(|(k, _)| k.0 <= k.1)
        .map(|(k, v)| (*k, *v))
        .collect()
}

#[test]
fn section_quantities_match_on_both_bases() {
    let mut worst = 0.0f64;
    let mut count = 0;
    for case in mass_cases() {
        for (&k, &index) in &case.member {
            let s = match &case.elements[index] {
                ElementOperand::Straight(s) => s.section,
                ElementOperand::Arc(a) => a.section,
                _ => unreachable!(),
            };
            let q = derive_section(&s, case.basis()).unwrap();
            for (name, value) in [
                ("inner_diameter_metal", q.inner_diameter_metal),
                ("inner_diameter_bore", q.inner_diameter_bore),
                ("metal", q.metal_mass_per_length),
                ("contents", q.contents_mass_per_length),
                ("insulation", q.insulation_mass_per_length),
                ("mbar", q.mass_per_length),
                ("jbar", q.polar_rotary_inertia_per_length),
                ("ibar", q.bending_rotary_inertia_per_length),
                ("A", q.area),
                ("I", q.second_moment),
                ("J", q.torsion_constant),
            ] {
                let expected = case.sections[&(k, name.to_string())];
                let scale = expected.abs().to_f64().max(f64::MIN_POSITIVE);
                let error = if expected.is_zero() {
                    value.abs().to_f64()
                } else {
                    value.sub(expected).abs().to_f64() / scale
                };
                assert!(
                    error <= NORMWISE,
                    "{} {} member {k} {name}: {error:e}",
                    case.id,
                    case.basis
                );
                worst = worst.max(error);
                count += 1;
            }
        }
    }
    eprintln!("section quantities: {count} compared, largest relative difference {worst:e}");
}

#[test]
fn element_masses_match_on_both_bases() {
    let (mut straight, mut arc, mut body, mut small_arc) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    for case in mass_cases() {
        let basis = case.basis();
        for (&k, &index) in &case.member {
            let contributions = reform_element_mass(&case.elements[index], basis).unwrap();
            match &case.elements[index] {
                ElementOperand::Straight(s) if s.carries_mass => {
                    let e = normwise(&upper(&square::<12>(&contributions[0])), &case.emass[&k]);
                    assert!(
                        e <= NORMWISE,
                        "{} {} straight {k}: {e:e}",
                        case.id,
                        case.basis
                    );
                    straight = straight.max(e);
                }
                ElementOperand::Straight(_) => {
                    assert!(contributions.is_empty());
                    assert!(!case.emass.contains_key(&k));
                }
                ElementOperand::Arc(_) => {
                    for side in 0..2 {
                        let e =
                            normwise(&square::<6>(&contributions[side]), &case.lumps[&(k, side)]);
                        assert!(
                            e <= NORMWISE,
                            "{} {} arc {k}/{side}: {e:e}",
                            case.id,
                            case.basis
                        );
                        if case.id == "M-ARC-SMALL" {
                            small_arc = small_arc.max(e);
                        } else {
                            arc = arc.max(e);
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        for element in &case.elements {
            if let ElementOperand::ComponentBody(b) = element {
                let contributions = reform_element_mass(element, basis).unwrap();
                let e = normwise(&square::<6>(&contributions[0]), &case.cmass[&b.node]);
                assert!(e <= NORMWISE, "{} {} body: {e:e}", case.id, case.basis);
                body = body.max(e);
            }
        }
    }
    eprintln!(
        "element masses, largest normwise difference: straight {straight:e}, arc {arc:e}, \
         M-ARC-SMALL {small_arc:e}, component {body:e}"
    );
}

/// The small arc's lump entries are each nonzero at the 10⁻²¹ to 10⁻⁸ scale
/// of its first and second moments; check them entrywise as well: every
/// entry above 10⁻²⁴ of the lump's largest agrees to 10⁻²⁰ relative, which
/// a closed-form evaluation of J_c,22 ≈ α⁵/20 (α = asin 10⁻⁴) would miss by
/// about 10⁻¹⁵ (its terms are α-sized, 10¹⁶ times larger, at 2⁻¹⁰⁶).
#[test]
fn small_arc_lumps_keep_their_leading_values() {
    let case = mass_cases()
        .into_iter()
        .find(|c| c.id == "M-ARC-SMALL" && c.basis == "PF")
        .unwrap();
    let contributions = reform_element_mass(&case.elements[0], case.basis()).unwrap();
    let mut worst = 0.0f64;
    for side in 0..2 {
        let expected = &case.lumps[&(0, side)];
        let scale = expected
            .values()
            .fold(0.0f64, |m, v| m.max(v.abs().to_f64()));
        for (&(r, c), &e) in expected {
            if e.abs().to_f64() < 1e-24 * scale {
                continue;
            }
            let a = contributions[side].get(r, c);
            let rel = a.sub(e).abs().to_f64() / e.abs().to_f64();
            assert!(rel <= 1e-20, "lump {side} ({r}, {c}): {rel:e}");
            worst = worst.max(rel);
        }
    }
    eprintln!("M-ARC-SMALL lumps, largest entrywise relative difference {worst:e}");
}

#[test]
fn assembled_full_mass_matches_on_both_bases() {
    let mut worst = 0.0f64;
    for case in mass_cases() {
        let n = 6 * case.nodes;
        let map: Vec<Option<usize>> = (0..n).map(Some).collect();
        let pencil = ReformedPencil::new(n, &map, case.basis(), &case.elements).unwrap();
        let assembled: BTreeMap<Pair, Dd> = pencil
            .assembled_mass()
            .into_iter()
            .filter(|(_, v)| !v.is_zero())
            .map(|((r, c), v)| ((c, r), v))
            .collect();
        let e = normwise(&assembled, &case.mfull);
        assert!(e <= NORMWISE, "{} {}: {e:e}", case.id, case.basis);
        worst = worst.max(e);
    }
    eprintln!("assembled full M, largest normwise difference {worst:e}");
}

/// `range_premise_witnesses` of `mass_references.json`, each refused on the
/// named operand; the reference's own component (W = 2560 N,
/// a = 9.8125 m/s²) passes.
#[test]
fn component_range_refusals() {
    let body = |w: f64, a: f64, r: [f64; 3]| ComponentBody {
        node: 0,
        weight: w,
        acceleration: a,
        center_of_gravity: r,
    };
    let witnesses = [
        (
            body(5e-324, 9.8125, [0.0; 3]),
            ComponentRangeOperand::Weight,
        ),
        (
            body(1e-300, 1e10, [0.0; 3]),
            ComponentRangeOperand::MassQuotient,
        ),
        (
            body(f64::MAX, 0.5, [0.0; 3]),
            ComponentRangeOperand::MassQuotient,
        ),
        (
            body(1.0, 1e-300, [1e10, 0.0, 0.0]),
            ComponentRangeOperand::SpatialBlock,
        ),
        (
            body(f64::MAX, 1.001953125, [0.0; 3]),
            ComponentRangeOperand::WeightProduct,
        ),
    ];
    for (b, operand) in witnesses {
        assert_eq!(component_mass(&b), Err(operand), "{b:?}");
        let element = ElementOperand::ComponentBody(b);
        assert_eq!(
            reform_element_mass(&element, MassBasis::EffectiveWall),
            Err(ReformationFailure::ComponentMassOutOfRange(operand))
        );
    }
    // The spatial witness passes the three scalar parts.
    assert!((1.0f64 / 1e-300).is_normal() && (1e-300 * (1.0f64 / 1e-300)).is_normal());
    assert!(component_mass(&body(2560.0, 9.8125, [0.25, -0.375, 0.125])).is_ok());
}

/// The dense eigenpairs of the formed pencil: the fixture's K_ff and M_ff
/// rounded once to binary64 (the hi words).
struct Formed {
    k: Vec<SymmetricMatrixEntry>,
    m: Vec<SymmetricMatrixEntry>,
    solution: DenseEigenSolution,
    map: Vec<Option<usize>>,
}

fn entries(map: &BTreeMap<Pair, Dd>) -> Vec<SymmetricMatrixEntry> {
    map.iter()
        .map(|(&(row, col), v)| SymmetricMatrixEntry {
            row,
            col,
            value: v.hi,
        })
        .collect()
}

fn dense(n: usize, list: &[SymmetricMatrixEntry]) -> SymmetricMatrix {
    let mut data = vec![0.0; n * n];
    for e in list {
        data[e.row * n + e.col] = e.value;
        data[e.col * n + e.row] = e.value;
    }
    SymmetricMatrix::from_row_major(n, data).unwrap()
}

fn formed(case: &Case, k: Vec<SymmetricMatrixEntry>, m: Vec<SymmetricMatrixEntry>) -> Formed {
    let n = case.free.len();
    let solution = solve_dense_pencil(&dense(n, &k), &dense(n, &m), &dense_options()).unwrap();
    let mut map = vec![None; 6 * case.nodes];
    for (index, &dof) in case.free.iter().enumerate() {
        map[dof] = Some(index);
    }
    Formed {
        k,
        m,
        solution,
        map,
    }
}

fn pencil(case: &Case, f: &Formed) -> ReformedPencil {
    ReformedPencil::new(case.free.len(), &f.map, case.basis(), &case.elements).unwrap()
}

/// The largest η over the computed modes of a formed pencil, on the
/// re-formed pencil.
fn largest_eta(case: &Case, f: &Formed) -> (f64, f64) {
    let p = pencil(case, f);
    let d = formation_difference(&p, &f.k, &f.m).unwrap();
    let mut eta = 0.0f64;
    let mut estimate = 0.0f64;
    for mode in 0..f.solution.mode_count() {
        let r = residual_estimate(
            &p,
            &d.scaling,
            f.solution.eigenvalues[mode],
            f.solution.vector(mode),
        )
        .unwrap();
        eta = eta.max(r.backward_error);
        estimate = estimate.max(r.error_estimate);
    }
    (eta, estimate)
}

/// K̂ against the 80-digit K_ff, the formation difference of the formed
/// (rounded-once) pencil, and η, e_λ and the eigenvalue error of every
/// computed mode against the published eigenvalues.
#[test]
fn reference_pencils_reform_and_their_dense_modes_have_small_residuals() {
    for case in eigen_cases() {
        let f = formed(&case, entries(&case.k), entries(&case.m));
        // Every case with an established count is resolved under the floor.
        if f.solution.established_finite_mode_count.is_some() {
            assert!(!f.solution.finite_count_unresolved, "{}", case.id);
        }
        eprintln!(
            "{}: resolution-floor margin {:.2e}",
            case.id,
            floor_margin(&f.solution)
        );
        let p = pencil(&case, &f);
        let k_hat: BTreeMap<Pair, Dd> = p.assembled_stiffness().into_iter().collect();
        let m_hat: BTreeMap<Pair, Dd> = p.assembled_mass().into_iter().collect();
        let (ek, em) = (normwise(&k_hat, &case.k), normwise(&m_hat, &case.m));
        assert!(
            ek <= NORMWISE && em <= NORMWISE,
            "{}: K {ek:e}, M {em:e}",
            case.id
        );
        let d = formation_difference(&p, &f.k, &f.m).unwrap();
        // The formed entries are the exact ones rounded once, so ΔK and ΔM
        // are rounding (≤ u = 2⁻⁵³ relative per entry) beside a re-formation
        // 10¹³ times more accurate.
        assert!(d.stiffness.norm <= 2.0 * f64::EPSILON && d.mass.norm <= 2.0 * f64::EPSILON);
        let mut eta = 0.0f64;
        let mut ratio = 0.0f64;
        let mut estimates = Vec::new();
        for mode in 0..f.solution.mode_count() {
            let lambda = f.solution.eigenvalues[mode];
            let r = residual_estimate(&p, &d.scaling, lambda, f.solution.vector(mode)).unwrap();
            assert!(
                r.backward_error <= ETA_CORRECT,
                "{} mode {mode}: η {:e}",
                case.id,
                r.backward_error
            );
            eta = eta.max(r.backward_error);
            estimates.push(r.error_estimate);
            if let Some(reference) = case.eigenvalues.get(mode) {
                let error = Dd::from_f64(lambda)
                    .sub(*reference)
                    .abs()
                    .div(*reference)
                    .to_f64();
                ratio = ratio.max(error / r.error_estimate);
            }
        }
        let ids = cluster_ids(&f.solution.eigenvalues, 1e-8).unwrap();
        let cluster = cluster_error_estimates(&estimates, &ids);
        // Each cluster's e_λ is the largest over its members, found here
        // directly.
        assert_eq!(cluster.len(), ids.iter().max().unwrap() + 1);
        for (id, &value) in cluster.iter().enumerate() {
            let members = estimates
                .iter()
                .zip(&ids)
                .filter(|&(_, &i)| i == id)
                .map(|(&e, _)| e);
            assert_eq!(value, members.fold(f64::NEG_INFINITY, f64::max), "{}", case.id);
        }
        let vectors: Vec<&[f64]> = (0..f.solution.mode_count())
            .map(|i| f.solution.vector(i))
            .collect();
        let orth_hat = m_orthogonality_reformed(&p, &vectors).unwrap();
        let orth = m_orthogonality(&dense(case.free.len(), &f.m), &vectors).unwrap();
        assert!(
            orth_hat.max_ratio <= FIXTURE_ORTHOGONALITY_THRESHOLD,
            "{}: orthogonality on M̂ {:e}",
            case.id,
            orth_hat.max_ratio
        );
        eprintln!(
            "{}: n {}, modes {}, K̂ {ek:.1e}, M̂ {em:.1e}, ‖DΔKD‖ {:.1e}, ‖DΔMD‖ {:.1e}, \
             off-pattern {}/{}, max η {eta:.2e}, max e_λ {:.2e}, max cluster e_λ {:.2e}, \
             max |λ−λ_ref|/λ_ref ÷ e_λ {ratio:.2e}, orthogonality on M̂ {:.2e} (formed M {:.2e})",
            case.id,
            case.free.len(),
            f.solution.mode_count(),
            d.stiffness.norm,
            d.mass.norm,
            d.stiffness.off_pattern,
            d.mass.off_pattern,
            estimates.iter().fold(0.0f64, |m, &e| m.max(e)),
            cluster.iter().fold(0.0f64, |m, &e| m.max(e)),
            orth_hat.max_ratio,
            orth.max_ratio,
        );
    }
}

/// A formed entry moved away from its re-formation shows in the formation
/// difference and in η.
#[test]
fn a_perturbed_formed_entry_shows_in_eta_and_the_formation_difference() {
    let case = eigen_cases()
        .into_iter()
        .find(|c| c.id == "E-CANT-4")
        .unwrap();
    let base = formed(&case, entries(&case.k), entries(&case.m));
    let (eta0, _) = largest_eta(&case, &base);
    let p = pencil(&case, &base);
    let d0 = formation_difference(&p, &base.k, &base.m).unwrap();
    for (target, relative) in [("k", 1e-8), ("m", 1e-8), ("k", 1e-5)] {
        let (mut k, mut m) = (base.k.clone(), base.m.clone());
        // The tip's transverse stiffness (or mass) diagonal, free row 19.
        let list = if target == "k" { &mut k } else { &mut m };
        let entry = list
            .iter_mut()
            .find(|e| e.row == 19 && e.col == 19)
            .unwrap();
        entry.value *= 1.0 + relative;
        let f = formed(&case, k, m);
        let d = formation_difference(&p, &f.k, &f.m).unwrap();
        let (eta, _) = largest_eta(&case, &f);
        // Relative to the re-formed scaled norm of the same matrix.
        let moved = if target == "k" {
            d.stiffness.norm / d.scaling.reformed_stiffness_norm
        } else {
            d.mass.norm / d.scaling.reformed_mass_norm
        };
        eprintln!(
            "E-CANT-4 {target}[19][19]·(1 + {relative:e}): relative formation difference {moved:.2e} \
             (was {:.2e}), max η {eta:.2e} (was {eta0:.2e})",
            if target == "k" {
                d0.stiffness.norm / d0.scaling.reformed_stiffness_norm
            } else {
                d0.mass.norm / d0.scaling.reformed_mass_norm
            }
        );
        assert!(moved >= 0.01 * relative, "{moved:e}");
        assert!(eta >= 1e3 * eta0 && eta >= 1e-3 * relative, "{eta:e}");
    }
}

/// Review round 4: a doubled derived area. The formed K takes A twice (a
/// section-derivation defect), so the axial eigenvalues double and nothing
/// else moves. K̂ is re-formed from A, I and J re-derived from OD and wall,
/// not from the kernel's properties, so η of the computed pairs is large;
/// were K̂ formed from the kernel's (doubled) properties it would match the
/// formed K and give η at rounding level. The section-derivation difference
/// names the defect directly.
#[test]
fn a_doubled_derived_area_shows_in_eta() {
    let case = eigen_cases()
        .into_iter()
        .find(|c| c.id == "E-CANT-4")
        .unwrap();
    let n = case.free.len();
    let base = formed(&case, entries(&case.k), entries(&case.m));
    // The formed K with 2A, element by element through the seam, rounded
    // once per assembled entry; springs and arcs are absent here.
    let defective = |factor: f64| -> Vec<SymmetricMatrixEntry> {
        let mut acc: BTreeMap<Pair, Dd> = BTreeMap::new();
        for element in &case.elements {
            let ElementOperand::Straight(s) = element else {
                panic!("straights only")
            };
            let q = derive_section(&s.section, case.basis()).unwrap();
            let wide = |x: Dd| WideOperand { hi: x.hi, lo: x.lo };
            let k = reform_straight(&StraightOperands {
                coordinates_i: s.coordinates_i,
                coordinates_j: s.coordinates_j,
                y_reference: s.y_reference,
                elastic_modulus: WideOperand::from_f64(s.moduli.elastic_modulus),
                shear_modulus: WideOperand::from_f64(s.moduli.shear_modulus),
                area: wide(q.area.mul_f64(factor)),
                second_moment_y: wide(q.second_moment),
                second_moment_z: wide(q.second_moment),
                torsion_constant: wide(q.torsion_constant),
                shear_area: None,
            })
            .unwrap();
            let dofs: Vec<usize> = (0..6)
                .map(|x| 6 * s.node_i + x)
                .chain((0..6).map(|x| 6 * s.node_j + x))
                .collect();
            for a in 0..12 {
                for b in 0..=a {
                    let (Some(ra), Some(rb)) = (base.map[dofs[a]], base.map[dofs[b]]) else {
                        continue;
                    };
                    let key = if ra >= rb { (ra, rb) } else { (rb, ra) };
                    let slot = acc.entry(key).or_insert(Dd::ZERO);
                    *slot = slot.add(Dd::from_terms(&k.terms(a, b)));
                }
            }
        }
        acc.into_iter()
            .filter(|(_, v)| v.hi != 0.0)
            .map(|((row, col), v)| SymmetricMatrixEntry {
                row,
                col,
                value: v.to_f64(),
            })
            .collect()
    };
    let control = formed(&case, defective(1.0), base.m.clone());
    let doubled = formed(&case, defective(2.0), base.m.clone());
    let (eta_control, _) = largest_eta(&case, &control);
    let (eta_doubled, _) = largest_eta(&case, &doubled);
    let p = pencil(&case, &doubled);
    let d = formation_difference(&p, &doubled.k, &doubled.m).unwrap();
    // The section-derivation difference of the kernel's doubled area.
    let mut with_formed = case.elements.clone();
    for element in &mut with_formed {
        if let ElementOperand::Straight(s) = element {
            let q = derive_section(&s.section, case.basis()).unwrap();
            s.formed_section = Some(FormedSection {
                area: q.area.mul_f64(2.0).to_f64(),
                second_moment: q.second_moment.to_f64(),
                torsion_constant: q.torsion_constant.to_f64(),
            });
        }
    }
    let sections = ReformedPencil::new(n, &doubled.map, case.basis(), &with_formed)
        .unwrap()
        .section_derivation_difference()
        .unwrap();
    eprintln!(
        "E-CANT-4 doubled area: max η {eta_doubled:.2e} (control {eta_control:.2e}), \
         ‖DΔKD‖ {:.2e}, section-derivation difference A {:.2e}, I {:.2e}, J {:.2e}",
        d.stiffness.norm, sections.area, sections.second_moment, sections.torsion_constant
    );
    assert!(eta_control <= ETA_CORRECT);
    assert!(eta_doubled >= 1e-2, "{eta_doubled:e}");
    assert!(d.stiffness.norm >= 0.5);
    assert!((sections.area - 1.0).abs() <= 1e-15 && sections.second_moment <= 1e-16);
}

#[test]
fn reruns_are_bit_identical() {
    let case = eigen_cases()
        .into_iter()
        .find(|c| c.id == "E-L-LINE-K3")
        .unwrap();
    let run = || {
        let f = formed(&case, entries(&case.k), entries(&case.m));
        let p = pencil(&case, &f);
        let d = formation_difference(&p, &f.k, &f.m).unwrap();
        let r: Vec<_> = (0..f.solution.mode_count())
            .map(|i| {
                residual_estimate(
                    &p,
                    &d.scaling,
                    f.solution.eigenvalues[i],
                    f.solution.vector(i),
                )
                .unwrap()
            })
            .collect();
        (p.assembled_stiffness(), p.assembled_mass(), d, r)
    };
    let (a, b) = (run(), run());
    let key = |x: &(Vec<(Pair, Dd)>, Vec<(Pair, Dd)>, _, Vec<_>)| {
        let mut out: Vec<u64> = Vec::new();
        for (_, v) in x.0.iter().chain(&x.1) {
            out.push(v.hi.to_bits());
            out.push(v.lo.to_bits());
        }
        out
    };
    assert_eq!(key(&a), key(&b));
    assert_eq!(a.2, b.2);
    assert_eq!(a.3, b.3);
}

/// Independent review of PR #33, finding 7: η, κ_λ and e_λ against their
/// closed forms (design §3.4 check 2) on a 2×2 pencil with hand-perturbed
/// vectors. Two nodes, each with one free Ux, an axis spring and a
/// component body at its node: K = diag(1, 4), M = diag(1, 2), formed
/// exactly, so D = diag(1, ½), ‖K̃̂‖ = 1 and ‖M̃̂‖ = max(1, ½) = 1.
/// - λ = 1 with φ = (1, ε): r = (0, 2ε), ‖Dr‖ = ε, ‖φ̃‖² = 1 + 4ε²,
///   φᵀMφ = 1 + 2ε², η = ε/(2√(1 + 4ε²)), κ_λ = 2(1 + 4ε²)/(1 + 2ε²) and
///   e_λ = ε√(1 + 4ε²)/(1 + 2ε²).
/// - λ = 2 with φ = (δ, 1): r = (−δ, 0), ‖Dr‖ = δ, ‖φ̃‖² = 4 + δ²,
///   φᵀMφ = 2 + δ², η = δ/(3√(4 + δ²)), κ_λ = 3(4 + δ²)/(2(2 + δ²)) and
///   e_λ = δ√(4 + δ²)/(2(2 + δ²)).
/// The library forms each in double-double and rounds once; the closed
/// forms are evaluated here in double-double and rounded once, so the two
/// agree within 4u (two roundings of at most u each, beside 2⁻¹⁰⁴).
#[test]
fn residual_estimates_match_their_closed_forms() {
    let mut map = vec![None; 12];
    map[0] = Some(0);
    map[6] = Some(1);
    let body = |node, weight| {
        ElementOperand::ComponentBody(ComponentBody {
            node,
            weight,
            acceleration: 1.0,
            center_of_gravity: [0.0; 3],
        })
    };
    let elements = [
        ElementOperand::Spring(SpringOperand::Axis {
            dof: 0,
            stiffness: 1.0,
        }),
        ElementOperand::Spring(SpringOperand::Axis {
            dof: 6,
            stiffness: 4.0,
        }),
        body(0, 1.0),
        body(1, 2.0),
    ];
    let p = ReformedPencil::new(2, &map, MassBasis::EffectiveWall, &elements).unwrap();
    let entry = |row, col, value| SymmetricMatrixEntry { row, col, value };
    let d = formation_difference(
        &p,
        &[entry(0, 0, 1.0), entry(1, 1, 4.0)],
        &[entry(0, 0, 1.0), entry(1, 1, 2.0)],
    )
    .unwrap();
    assert_eq!(d.stiffness.norm, 0.0);
    assert_eq!(d.mass.norm, 0.0);
    assert_eq!(d.scaling.reformed_stiffness_norm, 1.0);
    assert_eq!(d.scaling.reformed_mass_norm, 1.0);
    let u = f64::EPSILON / 2.0;
    let close = |actual: f64, expected: Dd, what: &str| {
        let expected = expected.to_f64();
        assert!(
            (actual - expected).abs() <= 4.0 * u * expected,
            "{what}: {actual:e} vs {expected:e}"
        );
    };
    let c = Dd::from_f64;
    for x in [2f64.powi(-10), 1e-3, 0.3] {
        let xx = Dd::product(x, x);
        let r = residual_estimate(&p, &d.scaling, 1.0, &[1.0, x]).unwrap();
        let (a, b) = (xx.mul_f64(4.0).add_f64(1.0), xx.mul_f64(2.0).add_f64(1.0));
        close(r.scaled_residual, c(x), "‖Dr‖, λ = 1");
        close(r.generalized_mass, b, "φᵀMφ, λ = 1");
        close(r.backward_error, c(x).div(a.sqrt().mul_f64(2.0)), "η, λ = 1");
        close(r.condition, a.mul_f64(2.0).div(b), "κ_λ, λ = 1");
        close(r.error_estimate, a.sqrt().mul_f64(x).div(b), "e_λ, λ = 1");
        let r = residual_estimate(&p, &d.scaling, 2.0, &[x, 1.0]).unwrap();
        let (a, b) = (xx.add_f64(4.0), xx.add_f64(2.0));
        close(r.scaled_residual, c(x), "‖Dr‖, λ = 2");
        close(r.generalized_mass, b, "φᵀMφ, λ = 2");
        close(r.backward_error, c(x).div(a.sqrt().mul_f64(3.0)), "η, λ = 2");
        close(r.condition, a.mul_f64(3.0).div(b.mul_f64(2.0)), "κ_λ, λ = 2");
        close(r.error_estimate, a.sqrt().mul_f64(x).div(b.mul_f64(2.0)), "e_λ, λ = 2");
    }
    // The exact pairs have zero residual.
    let exact = residual_estimate(&p, &d.scaling, 2.0, &[0.0, 1.0]).unwrap();
    assert_eq!((exact.backward_error, exact.error_estimate), (0.0, 0.0));
    // A cluster's e_λ is the largest over its members.
    assert_eq!(
        cluster_error_estimates(&[1e-9, 3e-9, 2e-9, 5e-10], &[0, 0, 0, 1]),
        vec![3e-9, 5e-10]
    );
}

/// Finding 2: the component's rotational block is formed from the
/// mass-weighted products the range premise admits. With a = 1, m = W
/// exactly, and the witnesses below pass `component_range`:
/// - W = 10³⁰⁰, r = (10⁻¹⁷⁰, 0, 0): |r|² underflowed and J_yy was 0, not
///   ≈ 10⁻⁴⁰;
/// - W = 10⁻³⁰⁰, r = (10¹⁶⁰, 0, 0): |r|² overflowed and J_yy was NaN, not
///   ≈ 10²⁰.
#[test]
fn component_rotational_block_is_formed_from_mass_weighted_products() {
    let body = |weight, acceleration, x| ComponentBody {
        node: 0,
        weight,
        acceleration,
        center_of_gravity: [x, 0.0, 0.0],
    };
    for (w, x) in [(1e300, 1e-170), (1e-300, 1e160)] {
        let b = body(w, 1.0, x);
        assert_eq!(component_range(&b), Ok(()));
        let m = component_mass(&b).unwrap();
        // m·r·r in binary64 takes two roundings.
        let expected = (w * x) * x;
        for d in [4, 5] {
            let j = m[d][d].to_f64();
            assert!(
                (j - expected).abs() <= f64::EPSILON * expected,
                "W {w:e}: J[{d}][{d}] = {j:e}, expected {expected:e}"
            );
        }
        assert_eq!(m[3][3].to_f64(), 0.0);
        for row in &m {
            for e in row {
                assert!(e.hi.is_finite() && e.lo.is_finite(), "W {w:e}");
            }
        }
    }
    // Admitted in binary64 (fl(fl(m·r)·r) is the largest finite number),
    // but the double-double product m·r·r exceeds it by more than half an
    // ulp and overflows: the re-formed entry is refused, not published.
    let b = body(19.92351542432765, 9.604308447003245, 9.30910318889974e153);
    assert_eq!(component_range(&b), Ok(()));
    assert_eq!(
        component_mass(&b),
        Err(ComponentRangeOperand::SpatialBlock)
    );
}

/// A one-member or two-member cantilever of one section, node 0 fixed, its
/// formed K_ff and M_ff the re-formed matrices rounded once.
fn cantilever(lengths: &[f64], section: PipeSection) -> (SymmetricMatrix, SymmetricMatrix) {
    let moduli = Moduli {
        elastic_modulus: 2.0e11,
        shear_modulus: 7.7e10,
        stiffness_scale: None,
    };
    let mut x = 0.0;
    let mut elements = Vec::new();
    for (k, &length) in lengths.iter().enumerate() {
        elements.push(ElementOperand::Straight(StraightOperand {
            node_i: k,
            node_j: k + 1,
            coordinates_i: [x, 0.0, 0.0],
            coordinates_j: [x + length, 0.0, 0.0],
            y_reference: [0.0, 1.0, 0.0],
            section,
            moduli,
            carries_mass: true,
            formed_section: None,
        }));
        x += length;
    }
    let n = 6 * lengths.len();
    let map: Vec<Option<usize>> = (0..6 * (lengths.len() + 1))
        .map(|g| g.checked_sub(6))
        .collect();
    let p = ReformedPencil::new(n, &map, MassBasis::EffectiveWall, &elements).unwrap();
    let to_dense = |list: Vec<((usize, usize), Dd)>| {
        let mut data = vec![0.0; n * n];
        for ((r, c), v) in list {
            data[r * n + c] = v.to_f64();
            data[c * n + r] = v.to_f64();
        }
        SymmetricMatrix::from_row_major(n, data).unwrap()
    };
    (to_dense(p.assembled_stiffness()), to_dense(p.assembled_mass()))
}

/// Finding 3, the design's admission witnesses (§3.2 item 5): the
/// equilibrated pivots of a small-bore one-element cantilever (OD 6.35 mm,
/// wall 0.89 mm, 1 m, steel 7850 kg/m³ with water contents 1000 kg/m³) are
/// all at least 0.22, although its raw pivots span the translational and
/// rotational scales; a short-member model likewise. Each is admitted with
/// its full finite count, on both halves.
#[test]
fn small_bore_and_short_member_models_are_admitted_with_their_full_count() {
    let small_bore = PipeSection {
        outside_diameter: 0.00635,
        wall_thickness: 0.00089,
        mill_tolerance: 0.0,
        corrosion_allowance: None,
        material_density: 7850.0,
        contents_density: Some(1000.0),
        insulation: None,
    };
    let (k, m) = cantilever(&[1.0], small_bore);
    let raw_ratio = (0..6).map(|i| m.get(i, i)).fold(f64::INFINITY, f64::min)
        / (0..6).map(|i| m.get(i, i)).fold(0.0, f64::max);
    eprintln!("small-bore cantilever: smallest/largest mass diagonal {raw_ratio:.2e}");
    assert_eq!(established_finite_mode_count(&m, 0.22), Some(6));
    let models = [
        ("small-bore cantilever", k, m),
        {
            // A 0.3 m member then a 0.02 m member (L/OD ≈ 0.12) of NPS 6.
            let section = PipeSection {
                outside_diameter: 0.1683,
                wall_thickness: 0.0071,
                ..small_bore
            };
            let (k, m) = cantilever(&[0.3, 0.02], section);
            ("short-member cantilever", k, m)
        },
    ];
    for (name, k, m) in &models {
        let n = m.dimension();
        assert_eq!(
            established_finite_mode_count(m, MASS_PIVOT_RELATIVE_THRESHOLD),
            Some(n),
            "{name}"
        );
        let solution = solve_dense_pencil(k, m, &dense_options()).unwrap();
        eprintln!(
            "{name}: n {n}, modes {}, μ range {:.2e} to {:.2e}, δ_μ {:.2e}, \
             resolution-floor margin {:.2e}",
            solution.mode_count(),
            solution.mu.last().unwrap(),
            solution.mu_max,
            solution.resolution_floor,
            floor_margin(&solution)
        );
        assert_eq!(solution.mode_count(), n, "{name}");
        assert!(!solution.finite_count_unresolved, "{name}");
    }
}

/// Review round 2, item 4: K = M = [[2, 1], [1, ½ + 2⁻³⁰]]. Every exact
/// λ is 1, but K is nearly singular (det 2⁻²⁹), so the reduction and
/// back-transform carry large errors (the second λ computes as
/// 1 + 1.2·10⁻⁷). The measured ratio, about
/// 2.65·10⁻¹², exceeds the withdrawn first-order bound
/// 10·n·u·√(λ_iλ_j)/λ₁ ≈ 2.2·10⁻¹⁵, which omits the reduction and
/// back-transform errors. The ratio is finite and reported; no bound is
/// asserted on it.
#[test]
fn orthogonality_of_a_close_pencil_is_finite_and_reported() {
    let rows = vec![vec![2.0, 1.0], vec![1.0, 0.5 + 2f64.powi(-30)]];
    let matrix = SymmetricMatrix::from_rows(&rows).unwrap();
    let solution = solve_dense_pencil(&matrix, &matrix, &dense_options()).unwrap();
    assert_eq!(solution.mode_count(), 2);
    let vectors: Vec<&[f64]> = (0..2).map(|i| solution.vector(i)).collect();
    let result = m_orthogonality(&matrix, &vectors).unwrap();
    let withdrawn = 10.0 * 2.0 * (f64::EPSILON / 2.0);
    eprintln!(
        "close pencil: λ {:?}, orthogonality {:.3e} (withdrawn first-order bound {withdrawn:.2e})",
        solution.eigenvalues, result.max_ratio
    );
    assert!(result.max_ratio.is_finite());
    assert_eq!(result.pair, Some((0, 1)));
}
