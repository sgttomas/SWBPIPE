//! Bit identity of the exact stages under workspace reuse (design §5.3.8,
//! Agent 0's option (c)): the residual (check 2), orthogonality (check 3),
//! normalization (§1.5) and formation (check 1) outputs, and `ExactSum`'s
//! own results, against the implementation before the reuse.
//!
//! **How the base comparison works.** This file uses only API that exists
//! at the base revision `68a039e1c` (before the reuse). It writes every
//! output as text (`{:?}` of each result: Rust prints an `f64` as its
//! shortest round-trip decimal, so equal text is equal bits for every
//! finite value, and NaN is never an output here), in sections, and hashes
//! each section (FNV-1a, 64 bit). `fixtures/workspace_reuse_base.txt` holds
//! the section line counts and hashes captured by running this same file at
//! the base revision (`WORKSPACE_REUSE_CAPTURE=<path>`); the test asserts
//! that the head reproduces them. `WORKSPACE_REUSE_DUMP=<path>` writes the
//! full text, so the base and head dumps can be compared line by line.
//!
//! The calls are made in sequences that exercise a reused workspace: every
//! mode of a reference pencil in turn on one pencil and one basis; early
//! refusals (before and after the workspace is filled) followed by valid
//! calls; a wide-span vector (exponents spread over ±400 binades, so many
//! limbs) after narrow ones and narrow ones after it (growth, then reset
//! of the high limbs); orthogonality between residual passes on the same
//! pencil (the two stages share its workspace); empty, zero and cancelling
//! inputs; and a direct `ExactSum` series with exponents over ±3,500.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use open_pipe_stress_modal_eigen::dd::{scale_by_pow2, Dd};
use open_pipe_stress_modal_eigen::dense::{solve_dense_pencil, DenseOptions};
use open_pipe_stress_modal_eigen::exact::ExactSum;
use open_pipe_stress_modal_eigen::modal_quantities::{normalize_mode, Dof, FreeDof, ModalBasis};
use open_pipe_stress_modal_eigen::orthogonality::{m_orthogonality, m_orthogonality_reformed};
use open_pipe_stress_modal_eigen::reformation::mass::ComponentBody;
use open_pipe_stress_modal_eigen::reformation::section::{Insulation, MassBasis, PipeSection};
use open_pipe_stress_modal_eigen::reformation::{
    formation_difference, residual_estimate, ArcOperand, ElementOperand, Moduli, ReformedPencil,
    SpringOperand, StraightOperand,
};
use open_pipe_stress_modal_eigen::SymmetricMatrix;
use open_pipe_stress_sparse_direct::SymmetricMatrixEntry;

type Pair = (usize, usize);

/// An eigen case of `fixtures/reformation_cases.txt` (the records this test
/// needs; the format is the one `reformation.rs` reads).
#[derive(Default)]
struct Case {
    id: String,
    basis: String,
    nodes: usize,
    elements: Vec<ElementOperand>,
    free: Vec<usize>,
    k: BTreeMap<Pair, Dd>,
    m: BTreeMap<Pair, Dd>,
}

fn bits(s: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(s, 16).unwrap())
}
fn opt(s: &str) -> Option<f64> {
    (s != "-").then(|| bits(s))
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

fn eigen_cases() -> Vec<Case> {
    let text = include_str!("fixtures/reformation_cases.txt");
    let mut cases = Vec::new();
    let mut c: Option<Case> = None;
    let mut coords: Vec<[f64; 3]> = Vec::new();
    for line in text.lines() {
        let f: Vec<&str> = line.split(' ').collect();
        let u = |i: usize| -> usize { f[i].parse().unwrap() };
        match f[0] {
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
            "straight" => {
                let (i, j) = (u(2), u(3));
                c.as_mut()
                    .unwrap()
                    .elements
                    .push(ElementOperand::Straight(StraightOperand {
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
                    }))
            }
            "arc" => {
                let (i, j) = (u(2), u(3));
                let k = bits(f[18]);
                c.as_mut()
                    .unwrap()
                    .elements
                    .push(ElementOperand::Arc(ArcOperand {
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
                    }))
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
            "free" => {
                c.as_mut().unwrap().free = f[1..].iter().map(|s| s.parse().unwrap()).collect()
            }
            "k" | "m" => {
                let v = Dd {
                    hi: bits(f[3]),
                    lo: bits(f[4]),
                };
                let case = c.as_mut().unwrap();
                let target = if f[0] == "k" {
                    &mut case.k
                } else {
                    &mut case.m
                };
                target.insert((u(1), u(2)), v);
            }
            "end" => {
                let case = c.take().unwrap();
                if !case.free.is_empty() {
                    cases.push(case);
                }
            }
            _ => {}
        }
    }
    cases
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

/// As `reformation.rs`'s test-local options.
fn dense_options() -> DenseOptions {
    DenseOptions {
        infinite_mu_relative_threshold: 1e-12,
        mass_pivot_relative_threshold: 1e-12,
        resolution_floor_constant: 1.0,
        resolution_floor_multiple: 10.0,
        formed_backward_error_multiple: 64.0,
        screen_error_estimate_threshold: 1e-6,
        max_ql_iterations_per_eigenvalue: 30,
    }
}

const DOFS: [Dof; 6] = [Dof::Ux, Dof::Uy, Dof::Uz, Dof::Rx, Dof::Ry, Dof::Rz];

fn p2(e: i32) -> f64 {
    scale_by_pow2(1.0, e)
}

/// The text of one case's outputs, in call order.
fn case_section(case: &Case, out: &mut String) {
    let n = case.free.len();
    let mut line = |s: String| {
        out.push_str(&s);
        out.push('\n');
    };
    let basis = match case.basis.as_str() {
        "PF" => MassBasis::EffectiveWall,
        _ => MassBasis::NominalWallPressureBore,
    };
    let mut map = vec![None; 6 * case.nodes];
    for (index, &dof) in case.free.iter().enumerate() {
        map[dof] = Some(index);
    }
    let (k, m) = (entries(&case.k), entries(&case.m));
    let mass = dense(n, &m);
    let solution = solve_dense_pencil(&dense(n, &k), &mass, &dense_options()).unwrap();
    let modes = solution.mode_count();
    let vectors: Vec<&[f64]> = (0..modes).map(|i| solution.vector(i)).collect();

    // Check 1: the re-formed pencil and its formation difference, on the
    // formed entries and on a formed set with one entry moved (a
    // cancellation of 40 bits).
    let pencil = ReformedPencil::new(n, &map, basis, &case.elements).unwrap();
    line(format!("assembled K {:?}", pencil.assembled_stiffness()));
    line(format!("assembled M {:?}", pencil.assembled_mass()));
    let difference = formation_difference(&pencil, &k, &m);
    line(format!("formation {difference:?}"));
    let mut moved = k.clone();
    moved[0].value *= 1.0 + p2(-40);
    line(format!(
        "formation moved {:?}",
        formation_difference(&pencil, &moved, &m)
    ));
    let scaling = difference.unwrap().scaling;

    // Vectors: zero; wide (each component scaled by 2^((i mod 5)·200 − 400));
    // cancelling (mode 0 plus its negative on alternate components).
    let zero = vec![0.0; n];
    let wide: Vec<f64> = vectors[0]
        .iter()
        .enumerate()
        .map(|(i, &v)| v * p2((i % 5) as i32 * 200 - 400))
        .collect();
    let cancelling: Vec<f64> = vectors[0]
        .iter()
        .enumerate()
        .map(|(i, &v)| if i % 2 == 0 { v } else { -v })
        .collect();

    // Check 2: every mode in turn on one pencil, with refusals, growth and
    // a shrink between them.
    let lambda0 = solution.eigenvalues[0];
    let mut residual = |label: &str, lambda: f64, v: &[f64]| {
        line(format!(
            "residual {label} {:?}",
            residual_estimate(&pencil, &scaling, lambda, v)
        ));
    };
    residual("mode 0", lambda0, vectors[0]);
    residual("refused eigenvalue", -1.0, vectors[0]);
    residual("zero", lambda0, &zero);
    residual("wide", lambda0, &wide);
    residual("mode 0 after wide", lambda0, vectors[0]);
    residual("cancelling", lambda0, &cancelling);
    for (i, v) in vectors.iter().enumerate() {
        residual(&format!("mode {i}"), solution.eigenvalues[i], v);
    }
    residual("refused length", lambda0, &vectors[0][..n - 1]);
    residual("zero again", lambda0, &zero);

    // Check 3 on the same pencil (its workspace is shared with check 2).
    let mut orthogonality = |label: &str, set: &[&[f64]]| {
        line(format!(
            "orthogonality {label} {:?}",
            m_orthogonality_reformed(&pencil, set)
        ));
    };
    orthogonality("all", &vectors);
    orthogonality("empty", &[]);
    orthogonality("one", &vectors[..1]);
    orthogonality("refused zero", &[vectors[0], &zero]);
    orthogonality("wide", &[&wide, vectors[0], &cancelling]);
    orthogonality("all after wide", &vectors);
    line(format!(
        "orthogonality formed {:?}",
        m_orthogonality(&mass, &vectors)
    ));
    line(format!(
        "orthogonality formed wide {:?}",
        m_orthogonality(&mass, &[&wide, vectors[0], &cancelling])
    ));

    // Check 2 again, in reverse, after check 3 used the workspace.
    for (i, v) in vectors.iter().enumerate().rev() {
        line(format!(
            "residual again mode {i} {:?}",
            residual_estimate(&pencil, &scaling, solution.eigenvalues[i], v)
        ));
    }

    // Normalization: every mode in turn on one basis, with refusals and
    // below-normal lists of different lengths.
    let free_dofs: Vec<FreeDof> = case
        .free
        .iter()
        .map(|&g| FreeDof {
            node: g / 6,
            dof: DOFS[g % 6],
        })
        .collect();
    let excitation: Vec<Vec<f64>> = (0..3)
        .map(|direction| {
            (0..n)
                .map(|i| {
                    (0..n)
                        .filter(|&j| case.free[j] % 6 == direction)
                        .map(|j| mass.get(i, j))
                        .sum()
                })
                .collect()
        })
        .collect();
    let excitation = [&excitation[0][..], &excitation[1][..], &excitation[2][..]];
    let first_translation = free_dofs.iter().position(|f| f.dof.is_translation());
    // Mostly below the normal range: the first translation 1, the others
    // 2⁻¹⁰⁷⁰ (rotations 0).
    let sparse: Vec<f64> = (0..n)
        .map(|i| {
            match (
                Some(i) == first_translation,
                free_dofs[i].dof.is_translation(),
            ) {
                (true, _) => 1.0,
                (false, true) => p2(-1070),
                (false, false) => 0.0,
            }
        })
        .collect();
    // Refused mid-shape: the first translation 2⁻⁴⁰ (the pivot under τ_t =
    // 0), the others 2⁻¹⁰⁷⁰ (below normal), rotations 2¹⁰⁰⁰ (overflow).
    let overflowing: Vec<f64> = (0..n)
        .map(|i| {
            match (
                Some(i) == first_translation,
                free_dofs[i].dof.is_translation(),
            ) {
                (true, _) => p2(-40),
                (false, true) => p2(-1070),
                (false, false) => p2(1000),
            }
        })
        .collect();
    for tau_t in [1e-3, 0.0] {
        let basis = ModalBasis::new(&mass, &free_dofs, excitation, [1.0; 3], 1.0, tau_t).unwrap();
        let mut normalize = |label: &str, mode: usize, v: &[f64]| {
            line(format!(
                "normalize tau {tau_t:?} {label} {:?}",
                normalize_mode(mode, v, &basis)
            ));
        };
        normalize("sparse", 0, &sparse);
        normalize("mode 0", 0, vectors[0]);
        normalize("refused overflow", 0, &overflowing);
        normalize("refused zero", 0, &zero);
        normalize("wide", 0, &wide);
        for (i, v) in vectors.iter().enumerate() {
            normalize(&format!("mode {i}"), i, v);
        }
        normalize("sparse again", 0, &sparse);
        normalize("cancelling", 0, &cancelling);
    }
}

/// `ExactSum` directly: random sums with exponents over ±3,500, products
/// with subnormal, huge and dd factors, the terms of one sum times factors
/// into another, cancellations and non-finite terms.
fn exact_section(out: &mut String) {
    let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let value = |next: &mut dyn FnMut() -> u64| -> f64 {
        let r = next();
        // Every field, subnormals included; never ∞ or NaN.
        let field = r % 2047;
        f64::from_bits((r >> 63) << 63 | field << 52 | (next() & ((1u64 << 52) - 1)))
    };
    for round in 0..300 {
        let mut a = ExactSum::new();
        let mut b = ExactSum::new();
        let mut added = Vec::new();
        for _ in 0..(1 + round % 23) {
            let x = value(&mut next);
            let e = (next() % 7001) as i32 - 3500;
            a.add_scaled(x, e);
            added.push((x, e));
            match next() % 4 {
                0 => a.add_product(&[x, value(&mut next)]),
                1 => a.add_product(&[x, value(&mut next), value(&mut next)]),
                2 => a.add_dd_product(
                    Dd {
                        hi: x,
                        lo: x * p2(-60),
                    },
                    &[value(&mut next)],
                ),
                _ => a.add_product_scaled(&[x, value(&mut next)], e),
            }
        }
        let terms = a.terms();
        b.add_terms_product(&terms, &[value(&mut next), value(&mut next)]);
        writeln!(out, "round {round} a {:?} terms {terms:?}", a.to_wide()).unwrap();
        writeln!(
            out,
            "round {round} b {:?} terms {:?}",
            b.to_wide(),
            b.terms()
        )
        .unwrap();
        // Cancel the scaled values in reverse: what remains are the
        // products.
        for &(x, e) in added.iter().rev() {
            a.add_scaled(-x, e);
        }
        writeln!(
            out,
            "round {round} cancelled {:?} terms {:?}",
            a.to_wide(),
            a.terms()
        )
        .unwrap();
        if round % 10 == 0 {
            a.add_f64(f64::INFINITY);
            writeln!(out, "round {round} non-finite {:?}", a.to_wide()).unwrap();
            a.add_f64(f64::NEG_INFINITY);
            writeln!(out, "round {round} nan {}", a.to_wide().to_f64().is_nan()).unwrap();
        }
    }
    // The empty and the exactly cancelled sums.
    let mut empty = ExactSum::new();
    writeln!(out, "empty {:?} {:?}", empty.to_wide(), empty.terms()).unwrap();
    empty.add_f64(p2(-1074));
    empty.add_scaled(-1.0, -1074);
    writeln!(out, "cancelled {:?} {:?}", empty.to_wide(), empty.terms()).unwrap();
}

fn fnv1a(text: &str) -> u64 {
    text.bytes().fold(0xcbf2_9ce4_8422_2325, |h, b| {
        (h ^ b as u64).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

#[test]
fn exact_stage_outputs_match_the_base_revision_bit_for_bit() {
    let mut sections: Vec<(String, String)> = Vec::new();
    let mut text = String::new();
    exact_section(&mut text);
    sections.push(("exact_sum".into(), text));
    for case in eigen_cases() {
        let mut text = String::new();
        case_section(&case, &mut text);
        sections.push((format!("case {}", case.id), text));
    }
    let digests: Vec<String> = sections
        .iter()
        .map(|(name, text)| {
            format!(
                "section {name} lines {} fnv1a64 {:016x}",
                text.lines().count(),
                fnv1a(text)
            )
        })
        .collect();
    if let Ok(path) = std::env::var("WORKSPACE_REUSE_DUMP") {
        let mut dump = String::new();
        for (name, text) in &sections {
            writeln!(dump, "== {name}").unwrap();
            dump.push_str(text);
        }
        std::fs::write(path, dump).unwrap();
    }
    if let Ok(path) = std::env::var("WORKSPACE_REUSE_CAPTURE") {
        std::fs::write(path, digests.join("\n") + "\n").unwrap();
        return;
    }
    let fixture = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/workspace_reuse_base.txt"
    ))
    .unwrap();
    let expected: Vec<&str> = fixture
        .lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty())
        .collect();
    assert_eq!(digests.len(), expected.len(), "section count");
    for (actual, expected) in digests.iter().zip(&expected) {
        assert_eq!(actual, expected);
    }
}
