//! The modal mass on the self-weight basis (DEL-04-09 Design §2.2 to §2.4),
//! formed in a separate pass over a prepared model's `BuiltModel`: never
//! inside `build_member` or `build_model_for_members`, so no frame of the
//! AC-004 peak stack chain changes (Design §3.5).
//!
//! - **Mass-bearing members** ([`mass_bearing_members`]): every pipe except an
//!   objective connector's replaced span. The one selection serves this
//!   assembly and the weight-basis inspection.
//! - **Straight members:** the kernel's consistent mass
//!   (`modal_eigen::reformation::mass::straight_mass`), with m̄ exactly
//!   `self_weight::stable_mass_per_length` on the model's route and j̄, ī per
//!   Design §2.2 (contents excluded, D-e).
//! - **Realized arcs:** the kernel's two rigid half-arc lumps (`arc_lumps`).
//! - **Component bodies:** for each member that the self-weight eligibility
//!   function (`self_weight::component_weights`, the same predicate, read
//!   once) maps, the body m = W/a at its centre of gravity replaces the
//!   member's distributed mass, after §2.3's four-part binary64 range premise
//!   (`component_range`). Only W and the centre of gravity are read from it.
//!
//! M is held as 6×6 node blocks over the 6N global DOFs, upper blocks only
//! (node i ≤ node j), read through the upper triangle so it is symmetric by
//! construction; entries are accumulated in double-double and rounded once.
//! It converts to a dense `SymmetricMatrix` only on demand.
use super::prepare::Prepared;
use super::{MODAL_COMPONENT_MASS_OUT_OF_RANGE, MODAL_WEIGHT_ACCELERATION_REQUIRED};
use crate::{diag, self_weight, stable_suffix, Diagnostic, PreviewModel, PreviewPipe};
use open_pipe_stress_modal_eigen::dd::Dd;
use open_pipe_stress_modal_eigen::reformation::mass::{
    arc_lumps, component_mass, component_range, straight_mass, Block12, Block6, ComponentBody,
};
use open_pipe_stress_modal_eigen::{MatrixError, SymmetricMatrix};
use std::collections::{BTreeMap, HashMap, HashSet};

/// Every pipe of `model` except an objective connector's replaced span, in
/// `pipe_segments` order: the members that carry mass and self-weight.
pub(crate) fn mass_bearing_members(model: &PreviewModel) -> Vec<String> {
    let replaced = crate::joint::replaced_span_ids(model);
    model.pipe_segments.iter().filter(|pipe| !replaced.contains(&pipe.id)).map(|pipe| pipe.id.clone()).collect()
}

/// A distributed member's element mass.
#[derive(Debug, Clone)]
pub(crate) enum MemberKind {
    /// The consistent 12×12 in global axes.
    Straight(Box<Block12>),
    /// The two half-arc lumps (node i's, node j's), each about its node.
    Arc { bend: usize, lumps: Box<[Block6; 2]> },
}

/// One distributed mass-bearing member.
#[derive(Debug, Clone)]
pub(crate) struct MemberMass {
    pub(crate) pipe: String,
    pub(crate) node_i: usize,
    pub(crate) node_j: usize,
    /// m̄, kg/m, exactly `stable_mass_per_length`.
    pub(crate) mass_per_length: f64,
    pub(crate) kind: MemberKind,
}

/// One component body that replaces its member's distributed mass.
#[derive(Debug, Clone)]
pub(crate) struct BodyMass {
    pub(crate) component_id: String,
    pub(crate) pipe: String,
    pub(crate) node: usize,
    /// W, N, as self-weight normalizes it.
    pub(crate) weight: f64,
    /// r, m, global from the node.
    pub(crate) center_of_gravity: [f64; 3],
    /// fl(W/a), kg.
    pub(crate) mass: f64,
    pub(crate) block: Block6,
}

/// The mass summary rows.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct MassSummary {
    /// T_k = r_kᵀ M r_k over all DOFs, kg, for k = X, Y, Z.
    pub(crate) total_by_direction: [f64; 3],
    /// The centre of gravity of M, m, when M carries translational mass.
    pub(crate) center_of_gravity: Option<[f64; 3]>,
}

/// The formed modal mass and what it was formed from.
#[derive(Debug, Clone)]
pub(crate) struct ModalMass {
    pub(crate) node_count: usize,
    /// Upper node blocks, keyed (i, j) with i ≤ j.
    blocks: BTreeMap<(usize, usize), [[Dd; 6]; 6]>,
    pub(crate) members: Vec<MemberMass>,
    pub(crate) bodies: Vec<BodyMass>,
    /// The request's a, m/s², when given.
    pub(crate) acceleration: Option<f64>,
    /// `SELF_WEIGHT_COMPONENT_WEIGHT_NOT_APPLIED`, as self-weight emits it.
    pub(crate) warnings: Vec<Diagnostic>,
    pub(crate) summary: MassSummary,
}

fn add_block(blocks: &mut BTreeMap<(usize, usize), [[Dd; 6]; 6]>, row: usize, col: usize, block: impl Fn(usize, usize) -> Dd) {
    let (key, transpose) = if row <= col { ((row, col), false) } else { ((col, row), true) };
    let target = blocks.entry(key).or_insert([[Dd::ZERO; 6]; 6]);
    for (a, line) in target.iter_mut().enumerate() {
        for (b, entry) in line.iter_mut().enumerate() {
            let value = if transpose { block(b, a) } else { block(a, b) };
            *entry = entry.add(value);
        }
    }
}

/// y = B·x for a symmetric 12×12 or 6×6 read through its upper triangle.
pub(crate) fn symmetric_apply<const N: usize>(block: &[[Dd; N]; N], x: &[f64; N]) -> [Dd; N] {
    let mut y = [Dd::ZERO; N];
    for r in 0..N {
        for c in 0..N {
            let entry = if r <= c { block[r][c] } else { block[c][r] };
            if !entry.is_zero() && x[c] != 0.0 {
                y[r] = y[r].add(entry.mul_f64(x[c]));
            }
        }
    }
    y
}

/// The rigid unit translation along k on a block of `nodes` nodes.
pub(crate) fn rigid_translation<const N: usize>(k: usize) -> [f64; N] {
    let mut x = [0.0; N];
    for node in 0..N / 6 {
        x[6 * node + k] = 1.0;
    }
    x
}

impl ModalMass {
    /// M[row][col] over the 6N global DOFs (node-major, ux uy uz rx ry rz).
    pub(crate) fn mass_entry(&self, row: usize, col: usize) -> f64 {
        let (r, c) = if row <= col { (row, col) } else { (col, row) };
        self.blocks.get(&(r / 6, c / 6)).map_or(0.0, |block| block[r % 6][c % 6].to_f64())
    }

    /// M[row][col] in double-double, through the full symmetric matrix: the
    /// stored upper block M_ij, or M_jiᵀ below it (R-6/R-7 D-1, C2).
    pub(crate) fn mass_entry_dd(&self, row: usize, col: usize) -> Dd {
        let (r, c) = if row / 6 < col / 6 || (row / 6 == col / 6 && row <= col) { (row, col) } else { (col, row) };
        self.blocks.get(&(r / 6, c / 6)).map_or(Dd::ZERO, |block| block[r % 6][c % 6])
    }

    /// The stored upper node blocks, rounded to binary64, (i, j) with i ≤ j.
    pub(crate) fn node_blocks(&self) -> impl Iterator<Item = ((usize, usize), [[f64; 6]; 6])> + '_ {
        self.blocks.iter().map(|(&key, block)| (key, block.map(|line| line.map(Dd::to_f64))))
    }

    /// M·x over the full DOF set, accumulated in double-double.
    pub(crate) fn apply_mass(&self, x: &[f64]) -> Vec<Dd> {
        let mut y = vec![Dd::ZERO; 6 * self.node_count];
        for (&(i, j), block) in &self.blocks {
            for a in 0..6 {
                for b in 0..6 {
                    let (r, c) = (6 * i + a, 6 * j + b);
                    if r > c {
                        continue;
                    }
                    let entry = block[a][b];
                    if entry.is_zero() {
                        continue;
                    }
                    y[r] = y[r].add(entry.mul_f64(x[c]));
                    if r != c {
                        y[c] = y[c].add(entry.mul_f64(x[r]));
                    }
                }
            }
        }
        y
    }

    /// M·r_k, r_k the rigid unit translation along global axis k.
    pub(crate) fn rigid_product(&self, k: usize) -> Vec<Dd> {
        let mut x = vec![0.0; 6 * self.node_count];
        for node in 0..self.node_count {
            x[6 * node + k] = 1.0;
        }
        self.apply_mass(&x)
    }

    /// M as a dense symmetric matrix, formed on demand only.
    pub(crate) fn to_symmetric(&self) -> Result<SymmetricMatrix, MatrixError> {
        let n = 6 * self.node_count;
        let mut data = vec![0.0; n * n];
        for (&(i, j), block) in &self.blocks {
            for a in 0..6 {
                for b in 0..6 {
                    let (r, c) = (6 * i + a, 6 * j + b);
                    if r <= c {
                        let value = block[a][b].to_f64();
                        data[r * n + c] = value;
                        data[c * n + r] = value;
                    }
                }
            }
        }
        SymmetricMatrix::from_row_major(n, data)
    }
}

/// j̄ and ī per length (Design §2.2): ρ_m·J_metal + ρ_ins·J_ins on the
/// route's metal wall (t − c_m on PF, t on EX), J of an annulus written
/// through the wall so no difference of fourth powers cancels; ī = j̄/2.
/// Contents are excluded (D-e). The inputs are the normalized section that
/// `stable_mass_per_length` has already admitted.
fn rotary_inertia_per_length(pipe: &PreviewPipe, exact: bool) -> Option<(f64, f64)> {
    use std::f64::consts::PI;
    let s = &pipe.section;
    let od = s.outside_diameter.value;
    let mill = s.mill_tolerance.as_ref().map_or(0.0, |q| q.value);
    let wall = if exact { s.wall_thickness.value } else { s.wall_thickness.value - mill };
    let inner = od - 2.0 * wall;
    let metal = PI / 8.0 * (od * od + inner * inner) * (od - wall) * wall;
    let mut polar = s.material_density.as_ref()?.value * metal;
    if let (Some(thickness), Some(density)) = (&s.insulation_thickness, &s.insulation_density) {
        let t = thickness.value;
        let outer = od + 2.0 * t;
        polar += density.value * (PI / 16.0 * (outer * outer + od * od) * (outer + od) * t);
    }
    (polar.is_finite() && polar >= 0.0).then_some((polar, polar / 2.0))
}

fn range_diag(pipe: &str, what: &str) -> Diagnostic {
    diag(
        &format!("diagnostic:modal:mass:{}:range", stable_suffix(pipe)),
        "SELF_WEIGHT_RANGE_INVALID",
        "blocking",
        format!("the modal mass of member {pipe} cannot be formed: {what}"),
        vec![pipe.to_string()],
    )
}

/// The modal mass of a prepared model (module docs). `acceleration` is the
/// request's a (m/s², positive), required when a component body enters.
pub(crate) fn assemble_mass(prepared: &Prepared, acceleration: Option<f64>) -> Result<ModalMass, Vec<Diagnostic>> {
    let model = &prepared.model;
    let built = &prepared.built;
    let node_map: HashMap<&str, usize> = model.nodes.iter().enumerate().map(|(i, n)| (n.id.as_str(), i)).collect();
    let position = |i: usize| built.nodes[i].coordinates;
    let members = mass_bearing_members(model);
    let selected: HashSet<&String> = members.iter().collect();

    // Component bodies: the self-weight predicate over the mass-bearing
    // members, at +1 along global Z (only W and r are read from it).
    let mut found = Vec::new();
    let mut warnings = Vec::new();
    let weights = self_weight::component_weights(model, &selected, "global_z", 1.0, &mut found, &mut warnings);
    if !found.is_empty() {
        return Err(found);
    }
    let mut bodies = Vec::new();
    if !weights.is_empty() {
        let Some(a) = acceleration else {
            let names: Vec<&str> = weights.values().map(|w| w.component.id.as_str()).collect();
            return Err(vec![diag(
                "diagnostic:modal:weight-to-mass-acceleration:required",
                MODAL_WEIGHT_ACCELERATION_REQUIRED,
                "blocking",
                format!("component weights enter the modal mass as bodies of mass W/a ({}); the request names weight_to_mass_acceleration a, which is never defaulted", names.join(", ")),
                std::iter::once("weight_to_mass_acceleration".to_string()).chain(names.iter().map(|n| n.to_string())).collect(),
            )]);
        };
        for (pipe, weight) in &weights {
            let component = weight.component;
            let Some(&node) = node_map.get(component.node.as_str()) else {
                return Err(vec![range_diag(pipe, &format!("component {} names no node", component.id))]);
            };
            let body = ComponentBody { node, weight: weight.weight, acceleration: a, center_of_gravity: weight.cog };
            let block = component_range(&body).and_then(|()| component_mass(&body)).map_err(|operand| {
                vec![diag(
                    &format!("diagnostic:modal:component-mass:{}:range", stable_suffix(&component.id)),
                    MODAL_COMPONENT_MASS_OUT_OF_RANGE,
                    "blocking",
                    format!("the body mass W/a of component {} (W = {} N, a = {a} m/s^2, centre of gravity {:?} m) is outside the binary64 range premise: operand {operand}", component.id, weight.weight, weight.cog),
                    vec![component.id.clone(), operand.to_string()],
                )]
            })?;
            bodies.push(BodyMass {
                component_id: component.id.clone(),
                pipe: pipe.clone(),
                node,
                weight: weight.weight,
                center_of_gravity: weight.cog,
                mass: weight.weight / a,
                block,
            });
        }
    }

    // Distributed members.
    let bends: HashMap<&str, usize> =
        built.curved_bend_elements.iter().enumerate().map(|(i, bend)| (bend.pipe_id.as_str(), i)).collect();
    let mut formed = Vec::new();
    let mut refusals = Vec::new();
    for pipe in model.pipe_segments.iter().filter(|p| selected.contains(&p.id) && !weights.contains_key(&p.id)) {
        let Some(mass_per_length) = self_weight::stable_mass_per_length(pipe, prepared.exact, &mut refusals) else {
            continue;
        };
        let Some((polar, bending)) = rotary_inertia_per_length(pipe, prepared.exact) else {
            refusals.push(range_diag(&pipe.id, "its rotary inertia per length is not finite and nonnegative"));
            continue;
        };
        let (Some(&from), Some(&to)) = (node_map.get(pipe.from.as_str()), node_map.get(pipe.to.as_str())) else {
            refusals.push(range_diag(&pipe.id, "an end node is not in the model"));
            continue;
        };
        let Some(y) = pipe.y_reference else {
            refusals.push(range_diag(&pipe.id, "it has no y_reference"));
            continue;
        };
        let y = [y.x, y.y, y.z];
        let (mbar, ibar, jbar) = (Dd::from_f64(mass_per_length), Dd::from_f64(bending), Dd::from_f64(polar));
        let member = match bends.get(pipe.id.as_str()) {
            Some(&bend) => {
                let b = &built.curved_bend_elements[bend];
                arc_lumps(position(b.node_i), position(b.node_j), b.bend_radius, y, mbar, ibar, jbar).map(|lumps| MemberMass {
                    pipe: pipe.id.clone(),
                    node_i: b.node_i,
                    node_j: b.node_j,
                    mass_per_length,
                    kind: MemberKind::Arc { bend, lumps: Box::new(lumps) },
                })
            }
            None => straight_mass(position(from), position(to), y, mbar, ibar, jbar).map(|element| MemberMass {
                pipe: pipe.id.clone(),
                node_i: from,
                node_j: to,
                mass_per_length,
                kind: MemberKind::Straight(Box::new(element)),
            }),
        };
        match member {
            Ok(member) => formed.push(member),
            Err(error) => refusals.push(range_diag(&pipe.id, &error.to_string())),
        }
    }
    if !refusals.is_empty() {
        return Err(refusals);
    }

    let mut blocks = BTreeMap::new();
    for member in &formed {
        match &member.kind {
            MemberKind::Straight(element) => {
                let (i, j) = (member.node_i, member.node_j);
                add_block(&mut blocks, i, i, |a, b| element[a][b]);
                add_block(&mut blocks, i, j, |a, b| element[a][6 + b]);
                add_block(&mut blocks, j, j, |a, b| element[6 + a][6 + b]);
            }
            MemberKind::Arc { lumps, .. } => {
                add_block(&mut blocks, member.node_i, member.node_i, |a, b| lumps[0][a][b]);
                add_block(&mut blocks, member.node_j, member.node_j, |a, b| lumps[1][a][b]);
            }
        }
    }
    for body in &bodies {
        add_block(&mut blocks, body.node, body.node, |a, b| body.block[a][b]);
    }
    let mut mass = ModalMass {
        node_count: built.nodes.len(),
        blocks,
        members: formed,
        bodies,
        acceleration,
        warnings,
        summary: MassSummary { total_by_direction: [0.0; 3], center_of_gravity: None },
    };
    mass.summary = summarize_mass(&mass, &built.nodes.iter().map(|n| n.coordinates).collect::<Vec<_>>());
    Ok(mass)
}

/// The node centroid: the fixed point resultant moments are taken about.
pub(crate) fn node_centroid(positions: &[[f64; 3]]) -> [f64; 3] {
    let mut sum = [Dd::ZERO; 3];
    for p in positions {
        for k in 0..3 {
            sum[k] = sum[k].add_f64(p[k]);
        }
    }
    let n = Dd::from_f64(positions.len().max(1) as f64);
    sum.map(|s| s.div(n).to_f64())
}

/// Force and moment about `point` of nodal 6-vectors `loads[n]` at `positions[n]`.
pub(crate) fn nodal_resultant(loads: impl IntoIterator<Item = (usize, [Dd; 6])>, positions: &[[f64; 3]], point: [f64; 3]) -> [Dd; 6] {
    let mut out = [Dd::ZERO; 6];
    for (node, load) in loads {
        let d = [0, 1, 2].map(|k| Dd::from_f64(positions[node][k]).sub(Dd::from_f64(point[k])));
        let f = [load[0], load[1], load[2]];
        let moment = [
            d[1].mul(f[2]).sub(d[2].mul(f[1])),
            d[2].mul(f[0]).sub(d[0].mul(f[2])),
            d[0].mul(f[1]).sub(d[1].mul(f[0])),
        ];
        for k in 0..3 {
            out[k] = out[k].add(f[k]);
            out[3 + k] = out[3 + k].add(moment[k]).add(load[3 + k]);
        }
    }
    out
}

fn summarize_mass(mass: &ModalMass, positions: &[[f64; 3]]) -> MassSummary {
    let point = node_centroid(positions);
    let mut totals = [0.0; 3];
    let mut moments = [[Dd::ZERO; 6]; 3];
    for k in 0..3 {
        let product = mass.rigid_product(k);
        let loads = (0..mass.node_count).map(|n| (n, [0, 1, 2, 3, 4, 5].map(|i| product[6 * n + i])));
        moments[k] = nodal_resultant(loads, positions, point);
        totals[k] = moments[k][k].to_f64();
    }
    // With F = T·e_k at the centre c: moment about p = T·(c − p) × e_k.
    let center = (totals.iter().all(|t| *t > 0.0)).then(|| {
        let tz = Dd::from_f64(totals[2]);
        let tx = Dd::from_f64(totals[0]);
        let dx = moments[2][4].neg().div(tz).to_f64();
        let dy = moments[2][3].div(tz).to_f64();
        let dz = moments[0][4].div(tx).to_f64();
        [point[0] + dx, point[1] + dy, point[2] + dz]
    });
    MassSummary { total_by_direction: totals, center_of_gravity: center }
}
