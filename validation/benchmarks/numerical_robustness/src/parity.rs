//! The binary64 sparse gate's parity checks on RF-MECH and RF-LARGE (brief
//! Scope 3; T3 D1 §4.8's parity protocol items 1–3; ROOT's C4 ruling: at up
//! to 100 members; plan §5.3).
//!
//! The same adapted model is formed through FK's binary64 path (FK's
//! `FrameElement`, global-axis springs, rigid DOFs, loads) in both modes, with
//! the same M03 evidence: the stiffness contributions and the formation
//! allowances in the form the structural adapter records them (as K1's
//! pattern-path tests build them).
//! - **Item 1:** every stored entry of the pattern assembly equals the dense
//!   assembly's entry bit for bit, and every entry outside the pattern is +0.
//! - **Item 2:** the M03 outcome class (Passed, Sensitive, or the refusal)
//!   is the same in both modes.
//! - **Item 3:** where both publish, max|u_sparse − u_dense| ≤ 1e-9·max|u_dense|
//!   (the DEC-053 basis), decided exactly; asserted where both are Passed and
//!   recorded otherwise.
//!
//! W1 is the lane's method; the binary64 values are not compared with R1 here.
use crate::cases::Model;
use crate::exact::Exact;
use open_pipe_stress_frame_kernel::structural::{
    assemble_sparse_stiffness, gamma, solve_structural_dense, transform_roundoff,
    SparseAssemblyOptions, SparseStiffness, SparseStructuralSystem, SparseSymmetryEvidence,
    StiffnessContribution, StructuralError, StructuralSolution, StructuralSystem, SymmetryEvidence,
};
use open_pipe_stress_frame_kernel::{
    assemble_global_stiffness, element_dof_map, FrameElement, FrameNode, FrameSection,
};
use open_pipe_stress_sparse_direct::structural::solve_sparse_structural;
use std::cmp::Ordering;
use std::collections::BTreeMap;

const BASIS: &str = "V-K parity: the structural adapter's formation allowances";

/// A model in FK's binary64 terms.
pub struct Binary64Model {
    pub node_count: usize,
    pub frames: Vec<FrameElement>,
    pub springs: Vec<(usize, f64)>,
    pub force: Vec<f64>,
    pub free: Vec<usize>,
    pub prescribed: Vec<(usize, f64)>,
}

pub fn binary64_model(model: &Model) -> Result<Binary64Model, String> {
    let nodes: Vec<FrameNode> = model
        .nodes
        .iter()
        .enumerate()
        .map(|(k, p)| FrameNode::new(k, *p).map_err(|e| format!("{e:?}")))
        .collect::<Result<_, _>>()?;
    let frames = model
        .members
        .iter()
        .map(|m| {
            let section = FrameSection::new(
                m.elastic_modulus,
                m.shear_modulus,
                m.area,
                m.second_moment_y,
                m.second_moment_z,
                m.torsion_constant,
            )
            .map_err(|e| format!("{e:?}"))?;
            FrameElement::new(
                nodes[m.node_i as usize],
                nodes[m.node_j as usize],
                section,
                m.y_reference,
            )
            .map_err(|e| format!("{e:?}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut springs = Vec::new();
    for s in &model.springs {
        match s.axis {
            Some(a) => springs.push((s.node as usize * 6 + a, s.stiffness)),
            None => return Err("a directional spring has no binary64 form".into()),
        }
    }
    let n = model.nodes.len() * 6;
    let mut force = vec![0.0; n];
    for (node, c, v, _) in &model.loads {
        force[*node as usize * 6 + c] += v;
    }
    let restrained: Vec<usize> = model
        .constraints
        .iter()
        .map(|&(node, c)| node as usize * 6 + c)
        .collect();
    let free = (0..n).filter(|g| !restrained.contains(g)).collect();
    let mut prescribed: Vec<(usize, f64)> = restrained.iter().map(|&g| (g, 0.0)).collect();
    prescribed.sort_by_key(|p| p.0);
    Ok(Binary64Model {
        node_count: model.nodes.len(),
        frames,
        springs,
        force,
        free,
        prescribed,
    })
}

/// Both triangles of every element's and spring's contribution, before
/// coalescence.
fn contributions(m: &Binary64Model) -> Vec<StiffnessContribution> {
    let mut out = Vec::new();
    for e in &m.frames {
        let k = e.global_stiffness().expect("formed");
        let map = element_dof_map(e.node_i.index, e.node_j.index);
        for i in 0..12 {
            for j in 0..12 {
                out.push(StiffnessContribution {
                    row: map[i],
                    col: map[j],
                    value: k[i][j],
                });
            }
        }
    }
    for &(dof, value) in &m.springs {
        out.push(StiffnessContribution {
            row: dof,
            col: dof,
            value,
        });
    }
    out
}

/// The formation allowances and operation counts per (row, col), in the
/// structural adapter's form: each element's `transform_roundoff`, one count
/// per spring, and γ(scatter)·Σ|contributions|.
fn allowances(
    m: &Binary64Model,
    c: &[StiffnessContribution],
) -> BTreeMap<(usize, usize), (f64, usize)> {
    let mut out: BTreeMap<(usize, usize), (f64, usize)> = BTreeMap::new();
    for e in &m.frames {
        let t = e.orientation().expect("oriented").transformation_matrix();
        let evidence =
            transform_roundoff(&e.local_stiffness().expect("formed"), &t).expect("finite");
        let map = element_dof_map(e.node_i.index, e.node_j.index);
        for i in 0..12 {
            for j in 0..12 {
                let slot = out.entry((map[i], map[j])).or_default();
                slot.0 += evidence.absolute_roundoff[i][j];
                slot.1 += evidence.operation_counts[i][j] + 1;
            }
        }
    }
    for &(dof, _) in &m.springs {
        out.entry((dof, dof)).or_default().1 += 1;
    }
    let mut magnitude: BTreeMap<(usize, usize), (f64, usize)> = BTreeMap::new();
    for x in c {
        let slot = magnitude.entry((x.row, x.col)).or_default();
        slot.0 += x.value.abs();
        slot.1 += 1;
    }
    for (key, (mag, scatter)) in magnitude {
        out.entry(key).or_default().0 += gamma(scatter) * mag;
    }
    out
}

/// The M03 outcome class of a result.
pub fn class(r: &Result<StructuralSolution, StructuralError>) -> String {
    match r {
        Ok(s) => format!("{:?}", s.report.quality),
        Err(StructuralError::InvalidInput(w)) => format!("InvalidInput({w})"),
        Err(StructuralError::Range(w)) => format!("Range({w})"),
        Err(StructuralError::Asymmetric { .. }) => "Asymmetric".into(),
        Err(StructuralError::NumericallyUnresolved { reason, .. }) => {
            format!("NumericallyUnresolved({reason})")
        }
        Err(StructuralError::NegativeEnergy { .. }) => "NegativeEnergy".into(),
        Err(StructuralError::Mechanism { .. }) => "Mechanism".into(),
    }
}

#[derive(Clone, Debug)]
pub struct Parity {
    /// Item 1: None when the pattern and dense assemblies agree bit for bit
    /// (only where the dense mode runs).
    pub bitwise_k: Option<Option<String>>,
    pub sparse: String,
    pub dense: Option<String>,
    /// Item 3 where both modes publish: within the DEC-053 basis, and the
    /// ratio max|Δu|/(1e-9·max|u_dense|) for display.
    pub delta: Option<(bool, f64)>,
    pub both_passed: bool,
}

fn max_abs(v: &[f64]) -> f64 {
    v.iter().fold(0.0f64, |m, x| m.max(x.abs()))
}

/// Items 1–3 on one model; `dense` runs the dense mode too (≤ ~100 members).
pub fn parity(model: &Model, dense: bool) -> Result<Parity, String> {
    let m = binary64_model(model)?;
    let c = contributions(&m);
    let allow = allowances(&m, &c);
    let sparse_k: SparseStiffness = assemble_sparse_stiffness(
        m.node_count,
        &m.frames,
        &[],
        &[],
        &m.springs,
        &SparseAssemblyOptions::new(),
    )
    .map_err(|e| format!("{e:?}"))?;
    let n = m.node_count * 6;
    let pattern = sparse_k.pattern();
    let (mut s_round, mut s_count) = (Vec::new(), Vec::new());
    for row in 0..n {
        for e in pattern.row_range(row) {
            let (r, k) = allow
                .get(&(row, pattern.column(e)))
                .copied()
                .unwrap_or((0.0, 0));
            s_round.push(r);
            s_count.push(k);
        }
    }
    let ss = SparseStructuralSystem::new(
        &sparse_k,
        &m.force,
        &m.free,
        &m.prescribed,
        Some(&c),
        Some(SparseSymmetryEvidence {
            absolute_roundoff: &s_round,
            operation_counts: &s_count,
            basis: BASIS,
        }),
    );
    let sparse = solve_sparse_structural(&ss);
    let mut out = Parity {
        bitwise_k: None,
        sparse: class(&sparse),
        dense: None,
        delta: None,
        both_passed: false,
    };
    if !dense {
        return Ok(out);
    }
    let mut dense_k =
        assemble_global_stiffness(m.node_count, &m.frames).map_err(|e| format!("{e:?}"))?;
    for &(dof, v) in &m.springs {
        dense_k[dof][dof] += v;
    }
    let mut mismatch = None;
    'rows: for (row, dense_row) in dense_k.iter().enumerate() {
        for (col, &d) in dense_row.iter().enumerate() {
            let stored = pattern.find(row, col).map(|_| sparse_k.get(row, col));
            let ok = match stored {
                Some(v) => v.to_bits() == d.to_bits(),
                None => d.to_bits() == 0.0f64.to_bits(),
            };
            if !ok {
                mismatch = Some(format!("({row}, {col}): pattern {stored:?}, dense {d:e}"));
                break 'rows;
            }
        }
    }
    out.bitwise_k = Some(mismatch);
    let mut d_round = vec![vec![0.0; n]; n];
    let mut d_count = vec![vec![0usize; n]; n];
    for (&(i, j), &(r, k)) in &allow {
        d_round[i][j] = r;
        d_count[i][j] = k;
    }
    let ds = StructuralSystem {
        stiffness: &dense_k,
        force: &m.force,
        free_dofs: &m.free,
        prescribed: &m.prescribed,
        contributions: Some(&c),
        symmetry: Some(SymmetryEvidence {
            absolute_roundoff: &d_round,
            operation_counts: &d_count,
            basis: BASIS,
        }),
    };
    let dense_r = solve_structural_dense(&ds);
    out.dense = Some(class(&dense_r));
    if let (Ok(s), Ok(d)) = (&sparse, &dense_r) {
        let scale = Exact::from_f64(max_abs(&d.displacements)).scaled(0, -9);
        let (mut worst, mut worst_x) = (Exact::zero(), 0.0f64);
        for (a, b) in s.displacements.iter().zip(&d.displacements) {
            let diff = Exact::from_f64(*a).sub(&Exact::from_f64(*b)).abs();
            if diff.cmp(&worst) == Ordering::Greater {
                worst_x = (a - b).abs();
                worst = diff;
            }
        }
        let within = worst.cmp(&scale) != Ordering::Greater;
        let ratio = worst_x / (1e-9 * max_abs(&d.displacements));
        out.delta = Some((within, ratio));
        out.both_passed = out.sparse == "Passed" && out.dense.as_deref() == Some("Passed");
    }
    Ok(out)
}
