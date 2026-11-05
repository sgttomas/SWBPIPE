//! The weight-basis identity (DEL-04-09 Design §2.5) and the acceleration
//! agreement (§2.3 item 5).
//!
//! For k = X, Y, Z, F_k = a·M·r_k on the full mass (a the request's
//! acceleration, or 1 m/s² when no component body enters) is compared with
//! the self-weight generator's loads at gravity a along k over the same
//! mass-bearing members, run in inspection on the **authored** model
//! (`self_weight::inspect_generated_self_weight`; the generator normalizes its
//! own copies and takes the route from the document):
//! - straight members nodally, against `straight_pipe`'s consistent
//!   uniform-load vector for the generated intensity;
//! - arcs and component bodies on the resultant force and moment about the
//!   node centroid (the arc's generated loads through `curved_bend`'s
//!   consistent vector, a body against its generated wrench), and the whole
//!   model on the resultant.
//!
//! **Tolerance (D-TOL): not pinned.** The identity tolerance is `TBD` in the
//! design until it is measured and pinned under DEC-026. [`check_weight_basis`] therefore
//! takes the tolerance as an explicit argument with no default and no
//! production value; it reports every measured discrepancy so the measurement
//! can be recorded. A discrepancy beyond the supplied tolerance refuses with
//! `MODAL_MASS_WEIGHT_BASIS_MISMATCH`, naming the member or component.
//!
//! The generated plan is consumed only for this in-memory comparison. It is
//! never applied, serialized into a result, stored or given any publication or
//! authority meaning, and neither its scratch case id nor its empty source
//! hash appears in anything returned here.
use super::mass::{node_centroid, nodal_resultant, rigid_translation, symmetric_apply, MemberKind, ModalMass};
use super::prepare::Prepared;
use super::{MODAL_MASS_WEIGHT_BASIS_MISMATCH, MODAL_WEIGHT_ACCELERATION_MISMATCH};
use crate::self_weight::{
    self, AppliedSelfWeightState, SelfWeightGravity, COMPONENT_WEIGHT_METHOD, GENERATED_METHOD, MANUAL_OVERRIDE_METHOD,
};
use crate::{diag, normalize_quantity, stable_suffix, Diagnostic, LoadTargetInput};
use open_pipe_stress_modal_eigen::dd::Dd;
use open_pipe_stress_straight_pipe::GlobalUniformLoad;
use open_pipe_stress_units::Dimension;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashMap};

const AXES: [&str; 3] = ["global_x", "global_y", "global_z"];

/// The identity's tolerance pair: an entry fails when its discrepancy exceeds
/// `absolute + relative·scale`. It has no default (module docs).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct IdentityTolerance {
    pub(crate) relative: f64,
    pub(crate) absolute: f64,
}

/// One measured comparison.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Discrepancy {
    /// The member, component or `model` compared.
    pub(crate) subject: String,
    /// What `subject` names: `member` (a pipe id), `component_body` (a
    /// component id) or `model` (the aggregate), so a member named `model`
    /// is never taken for the aggregate (I3a).
    pub(crate) subject_kind: &'static str,
    pub(crate) axis: usize,
    /// `nodal` (straights) or `resultant`.
    pub(crate) kind: &'static str,
    /// The largest |difference|, N or N·m.
    pub(crate) absolute: f64,
    /// The largest |difference| relative to the reference's scale (the
    /// largest reference magnitude of the same kind, force or moment).
    pub(crate) relative: f64,
}

/// The non-blocking row of a current, full-scope applied self-weight case.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AppliedCaseRow {
    pub(crate) case_id: String,
    pub(crate) axis: usize,
    pub(crate) force_relative: f64,
    pub(crate) moment_relative: f64,
}

/// What the identity measured and disclosed.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct IdentityReport {
    /// The identity's acceleration: a, or 1 m/s² with no component body.
    pub(crate) acceleration: f64,
    pub(crate) discrepancies: Vec<Discrepancy>,
    pub(crate) applied_rows: Vec<AppliedCaseRow>,
    /// Applied self-weight primitives excluded from the comparisons (stale,
    /// manually overridden) and partial-scope cases, by name.
    pub(crate) disclosures: Vec<String>,
}

impl IdentityReport {
    /// The largest discrepancy of `kind` (absolute, relative).
    pub(crate) fn largest_discrepancy(&self, kind: &str) -> (f64, f64) {
        self.discrepancies.iter().filter(|d| d.kind == kind).fold((0.0, 0.0), |(a, r), d| (a.max(d.absolute), r.max(d.relative)))
    }
}

fn weight_basis_mismatch(subject: &str, axis: usize, message: String) -> Diagnostic {
    diag(
        &format!("diagnostic:modal:weight-basis:{}:{}", stable_suffix(subject), AXES[axis]),
        MODAL_MASS_WEIGHT_BASIS_MISMATCH,
        "blocking",
        message,
        vec![subject.to_string(), AXES[axis].to_string()],
    )
}

fn evidence_of(provenance: Option<&str>) -> Option<Value> {
    provenance.and_then(|text| serde_json::from_str(text).ok())
}

/// §2.3 item 5: every current applied component-weight primitive's recorded
/// acceleration magnitude equals a, bit for bit. Stale and manually overridden
/// primitives are disclosed and take no part.
fn acceleration_agreement(prepared: &Prepared, a: Option<f64>, disclosures: &mut Vec<String>) -> Result<(), Vec<Diagnostic>> {
    let mut refusals = Vec::new();
    for status in self_weight::inspect_applied_self_weight(prepared.original, None) {
        let case = &prepared.original.load_cases.iter().find(|c| c.id == status.case_id).expect("inspected case");
        let load = &case.primitive_loads[status.primitive_index];
        match status.state {
            AppliedSelfWeightState::Stale | AppliedSelfWeightState::ManualOverride => {
                disclosures.push(format!("applied self-weight primitive {} of case {} is {:?} and takes no part in the comparison", load.id, case.id, status.state));
                continue;
            }
            AppliedSelfWeightState::Fresh => {}
            _ => continue,
        }
        let Some(evidence) = evidence_of(load.provenance.as_deref()) else { continue };
        if evidence["method"] != COMPONENT_WEIGHT_METHOD {
            continue;
        }
        let recorded = evidence["normalized_acceleration_m_per_s2"].as_f64().map(f64::abs);
        let agrees = matches!((recorded, a), (Some(g), Some(a)) if g.to_bits() == a.to_bits());
        if !agrees {
            refusals.push(diag(
                &format!("diagnostic:modal:weight-to-mass-acceleration:{}:{}", stable_suffix(&case.id), stable_suffix(&load.id)),
                MODAL_WEIGHT_ACCELERATION_MISMATCH,
                "blocking",
                format!(
                    "load case {} applies component weight primitive {} at acceleration magnitude {} m/s^2, but the request's weight_to_mass_acceleration is {}; the modal component masses would not be the masses the model's own static weight implies",
                    case.id,
                    load.id,
                    recorded.map_or("unrecorded".to_string(), |g| format!("{g:e}")),
                    a.map_or("absent".to_string(), |a| format!("{a:e} m/s^2")),
                ),
                vec![case.id.clone(), load.id.clone(), "weight_to_mass_acceleration".to_string()],
            ));
        }
    }
    if refusals.is_empty() {
        Ok(())
    } else {
        Err(refusals)
    }
}

/// The reference's scale for each entry kind of a 6-per-node vector: forces
/// by their largest magnitude, moments by the larger of their own largest
/// magnitude and the force scale times `length` (the compared body's size), so
/// that a moment that is zero in exact arithmetic is judged against the
/// moments its forces make, not against its own rounding.
fn identity_scales(reference: &[f64], length: f64) -> [f64; 2] {
    let mut scale = [0.0f64; 2];
    for (i, r) in reference.iter().enumerate() {
        scale[(i % 6) / 3] = scale[(i % 6) / 3].max(r.abs());
    }
    scale[1] = scale[1].max(scale[0] * length);
    if scale[0] == 0.0 && length > 0.0 {
        scale[0] = scale[1] / length;
    }
    scale
}

/// (largest |difference|, largest |difference| relative to its kind's scale).
fn largest_difference(formed: &[Dd], reference: &[f64], length: f64) -> (f64, f64) {
    let scale = identity_scales(reference, length);
    let (mut abs, mut rel) = (0.0f64, 0.0f64);
    for (i, (f, r)) in formed.iter().zip(reference).enumerate() {
        let d = f.sub(Dd::from_f64(*r)).abs().to_f64();
        abs = abs.max(d);
        let s = scale[(i % 6) / 3];
        rel = rel.max(if d == 0.0 { 0.0 } else if s == 0.0 { f64::INFINITY } else { d / s });
    }
    (abs, rel)
}

fn identity_exceeds(formed: &[Dd], reference: &[f64], length: f64, tolerance: IdentityTolerance) -> bool {
    let scale = identity_scales(reference, length);
    formed.iter().zip(reference).enumerate().any(|(i, (f, r))| {
        let d = f.sub(Dd::from_f64(*r)).abs().to_f64();
        !(d <= tolerance.absolute + tolerance.relative * scale[(i % 6) / 3])
    })
}

/// The diagonal of the bounding box of `positions` (m).
fn bounding_diagonal(positions: &[[f64; 3]]) -> f64 {
    let mut low = [f64::INFINITY; 3];
    let mut high = [f64::NEG_INFINITY; 3];
    for p in positions {
        for k in 0..3 {
            low[k] = low[k].min(p[k]);
            high[k] = high[k].max(p[k]);
        }
    }
    (0..3).map(|k| (high[k] - low[k]).max(0.0).powi(2)).sum::<f64>().sqrt()
}

fn to_f64s(values: &[Dd]) -> Vec<f64> {
    values.iter().map(|v| v.to_f64()).collect()
}

/// The generator's rows, for one axis.
struct Generated {
    /// Pipe id to generated intensity (N/m).
    intensities: BTreeMap<String, f64>,
    /// Component id to (node id, force, moment), N and N·m.
    wrenches: BTreeMap<String, (String, [f64; 3], [f64; 3])>,
}

fn generated_rows(plan: &self_weight::SelfWeightOperationPlan) -> Option<Generated> {
    let mut intensities = BTreeMap::new();
    let mut wrenches = BTreeMap::new();
    for row in &plan.source_evidence {
        if row["method"] == GENERATED_METHOD {
            let pipe = row["pipe_id"].as_str()?.to_string();
            let value = row["generated_payload"]["magnitude"]["value"].as_f64()?;
            intensities.insert(pipe, value);
        } else if row["method"] == COMPONENT_WEIGHT_METHOD {
            let id = row["component_id"].as_str()?.to_string();
            let node = row["component_node"].as_str()?.to_string();
            let force: [f64; 3] = serde_json::from_value(row["force_N"].clone()).ok()?;
            let moment: [f64; 3] = serde_json::from_value(row["moment_N_m"].clone()).ok()?;
            wrenches.insert(id, (node, force, moment));
        } else {
            return None;
        }
    }
    Some(Generated { intensities, wrenches })
}

/// The weight-basis identity and the acceleration agreement on a prepared
/// model and its formed mass (module docs). Only a [`Prepared`] reaches the
/// inspection, so it runs only after every validation the static solve
/// requires has passed.
pub(crate) fn check_weight_basis(prepared: &Prepared, mass: &ModalMass, tolerance: IdentityTolerance) -> Result<IdentityReport, Vec<Diagnostic>> {
    let mut disclosures = Vec::new();
    acceleration_agreement(prepared, mass.acceleration, &mut disclosures)?;
    let a = if mass.bodies.is_empty() { 1.0 } else { mass.acceleration.expect("a component body enters only with a") };
    let positions: Vec<[f64; 3]> = prepared.built.nodes.iter().map(|n| n.coordinates).collect();
    let point = node_centroid(&positions);
    let node_index: HashMap<&str, usize> =
        prepared.model.nodes.iter().enumerate().map(|(i, n)| (n.id.as_str(), i)).collect();
    let members = super::mass::mass_bearing_members(prepared.original);
    let mut discrepancies = Vec::new();
    let mut refusals = Vec::new();
    for k in 0..3 {
        let gravity = SelfWeightGravity { value: a, unit: "m/s^2".into(), axis: AXES[k].into() };
        let plan = self_weight::inspect_generated_self_weight(prepared.original, &members, &gravity).map_err(|mut found| {
            found.push(weight_basis_mismatch("model", k, format!("the self-weight generator refused its inspection along {}, so the weight basis cannot be compared", AXES[k])));
            found
        })?;
        let Some(generated) = generated_rows(&plan) else {
            return Err(vec![weight_basis_mismatch("model", k, "the self-weight inspection returned a row this check does not read".into())]);
        };
        // The same members on both sides.
        let formed_pipes: BTreeSet<&str> = mass.members.iter().map(|m| m.pipe.as_str()).collect();
        let generated_pipes: BTreeSet<&str> = generated.intensities.keys().map(String::as_str).collect();
        for pipe in formed_pipes.symmetric_difference(&generated_pipes) {
            refusals.push(weight_basis_mismatch(pipe, k, format!("member {pipe} carries distributed mass on one basis only (the modal mass and the self-weight generator disagree on it)")));
        }
        let formed_bodies: BTreeSet<&str> = mass.bodies.iter().map(|b| b.component_id.as_str()).collect();
        let generated_bodies: BTreeSet<&str> = generated.wrenches.keys().map(String::as_str).collect();
        for component in formed_bodies.symmetric_difference(&generated_bodies) {
            refusals.push(weight_basis_mismatch(component, k, format!("component {component} is a body on one basis only")));
        }
        let mut reference_loads: Vec<(usize, [Dd; 6])> = Vec::new();
        for member in &mass.members {
            let Some(&w) = generated.intensities.get(&member.pipe) else { continue };
            let mut intensity = [0.0; 3];
            intensity[k] = w;
            match &member.kind {
                MemberKind::Straight(element) => {
                    let Some(pipe) = prepared.built.pipes.iter().find(|p| p.element_id == member.pipe) else {
                        refusals.push(weight_basis_mismatch(&member.pipe, k, format!("member {} has no built straight element", member.pipe)));
                        continue;
                    };
                    let reference = GlobalUniformLoad::new(intensity)
                        .and_then(|load| pipe.equivalent_global_nodal_loads(&[load], &[]));
                    let reference = match reference {
                        Ok(reference) => reference,
                        Err(error) => {
                            refusals.push(weight_basis_mismatch(&member.pipe, k, format!("the uniform-load vector of member {} is not formed: {error}", member.pipe)));
                            continue;
                        }
                    };
                    let formed: Vec<Dd> = symmetric_apply(element, &rigid_translation::<12>(k)).iter().map(|v| v.mul_f64(a)).collect();
                    let length = bounding_diagonal(&[positions[member.node_i], positions[member.node_j]]);
                    let (abs, rel) = largest_difference(&formed, &reference, length);
                    discrepancies.push(Discrepancy { subject: member.pipe.clone(), subject_kind: "member", axis: k, kind: "nodal", absolute: abs, relative: rel });
                    if identity_exceeds(&formed, &reference, length, tolerance) {
                        refusals.push(weight_basis_mismatch(&member.pipe, k, format!("a·M·r_k of straight member {} differs from its consistent uniform-load vector along {} by {abs:e} ({rel:e} relative)", member.pipe, AXES[k])));
                    }
                    reference_loads.push((member.node_i, std::array::from_fn(|i| Dd::from_f64(reference[i]))));
                    reference_loads.push((member.node_j, std::array::from_fn(|i| Dd::from_f64(reference[6 + i]))));
                }
                MemberKind::Arc { bend, lumps } => {
                    let reference = match prepared.built.curved_bend_elements[*bend].macro_element.consistent_uniform_nodal_loads(intensity) {
                        Ok(reference) => reference,
                        Err(error) => {
                            refusals.push(weight_basis_mismatch(&member.pipe, k, format!("the arc's consistent load vector of member {} is not formed: {error}", member.pipe)));
                            continue;
                        }
                    };
                    let arc_reference = [
                        (member.node_i, std::array::from_fn(|i| Dd::from_f64(reference[i]))),
                        (member.node_j, std::array::from_fn(|i| Dd::from_f64(reference[6 + i]))),
                    ];
                    let formed = [
                        (member.node_i, symmetric_apply(&lumps[0], &rigid_translation::<6>(k)).map(|v| v.mul_f64(a))),
                        (member.node_j, symmetric_apply(&lumps[1], &rigid_translation::<6>(k)).map(|v| v.mul_f64(a))),
                    ];
                    compare_resultant((&member.pipe, "member"), k, &formed, &arc_reference, &positions, About::Own(&[]), tolerance, &mut discrepancies, &mut refusals);
                    reference_loads.extend(arc_reference);
                }
            }
        }
        for body in &mass.bodies {
            let Some((node, force, moment)) = generated.wrenches.get(&body.component_id) else { continue };
            let Some(&node) = node_index.get(node.as_str()) else {
                refusals.push(weight_basis_mismatch(&body.component_id, k, format!("component {}'s generated wrench names no node", body.component_id)));
                continue;
            };
            let wrench = [(node, [force[0], force[1], force[2], moment[0], moment[1], moment[2]].map(Dd::from_f64))];
            let formed = [(body.node, symmetric_apply(&body.block, &rigid_translation::<6>(k)).map(|v| v.mul_f64(a)))];
            // The body's own size: its node and its centre of gravity.
            let at = positions[body.node];
            let gravity_centre = [at[0] + body.center_of_gravity[0], at[1] + body.center_of_gravity[1], at[2] + body.center_of_gravity[2]];
            compare_resultant((&body.component_id, "component_body"), k, &formed, &wrench, &positions, About::Own(&[gravity_centre]), tolerance, &mut discrepancies, &mut refusals);
            reference_loads.extend(wrench);
        }
        // The whole model, on the resultant.
        let product = mass.rigid_product(k);
        let formed: Vec<(usize, [Dd; 6])> =
            (0..mass.node_count).map(|n| (n, std::array::from_fn(|i| product[6 * n + i].mul_f64(a)))).collect();
        compare_resultant(("model", "model"), k, &formed, &reference_loads, &positions, About::Model(point), tolerance, &mut discrepancies, &mut refusals);
    }
    if !refusals.is_empty() {
        return Err(refusals);
    }
    let applied_rows = applied_case_rows(prepared, mass, &positions, point, &mut disclosures);
    Ok(IdentityReport { acceleration: a, discrepancies, applied_rows, disclosures })
}

/// The point a resultant's moment is taken about, which also sets the size
/// that scales a moment's tolerance.
#[derive(Clone, Copy)]
enum About<'a> {
    /// One member or component: the centroid of its own nodes, with its size
    /// from those nodes and the extra points given (a body's centre of
    /// gravity). Nothing outside the compared subject enters, so a distant
    /// part of the model cannot inflate the moment scale and hide an error.
    Own(&'a [[f64; 3]]),
    /// The whole model: the node centroid, with the model's own size.
    Model([f64; 3]),
}

#[allow(clippy::too_many_arguments)]
fn compare_resultant(
    (subject, subject_kind): (&str, &'static str),
    k: usize,
    formed: &[(usize, [Dd; 6])],
    reference: &[(usize, [Dd; 6])],
    positions: &[[f64; 3]],
    about: About,
    tolerance: IdentityTolerance,
    discrepancies: &mut Vec<Discrepancy>,
    refusals: &mut Vec<Diagnostic>,
) {
    let mut extent: Vec<[f64; 3]> = formed.iter().chain(reference).map(|(n, _)| positions[*n]).collect();
    let point = match about {
        About::Own(extra) => {
            let mut own: Vec<[f64; 3]> = extent.clone();
            own.sort_by(|a, b| a.iter().zip(b).map(|(x, y)| x.total_cmp(y)).find(|o| o.is_ne()).unwrap_or(std::cmp::Ordering::Equal));
            own.dedup();
            let point = node_centroid(&own);
            extent.extend_from_slice(extra);
            point
        }
        About::Model(point) => {
            extent.push(point);
            point
        }
    };
    let f = nodal_resultant(formed.iter().copied(), positions, point);
    let r = nodal_resultant(reference.iter().copied(), positions, point);
    let reference_values = to_f64s(&r);
    let length = bounding_diagonal(&extent);
    let (abs, rel) = largest_difference(&f, &reference_values, length);
    discrepancies.push(Discrepancy { subject: subject.to_string(), subject_kind, axis: k, kind: "resultant", absolute: abs, relative: rel });
    if identity_exceeds(&f, &reference_values, length, tolerance) {
        refusals.push(weight_basis_mismatch(subject, k, format!(
            "the resultant of a·M·r_k for {subject} along {} differs from the self-weight resultant about {} by {abs:e} ({rel:e} relative)",
            AXES[k],
            match about { About::Own(_) => "its own node centroid", About::Model(_) => "the model's node centroid" },
        )));
    }
}

/// §2.5's last bullet: a current applied self-weight case whose scope covers
/// every mass-bearing member is compared on the resultant as a non-blocking
/// row; a partial scope or an excluded primitive is a disclosure.
fn applied_case_rows(
    prepared: &Prepared,
    mass: &ModalMass,
    positions: &[[f64; 3]],
    point: [f64; 3],
    disclosures: &mut Vec<String>,
) -> Vec<AppliedCaseRow> {
    let members: BTreeSet<String> = super::mass::mass_bearing_members(prepared.original).into_iter().collect();
    let statuses = self_weight::inspect_applied_self_weight(prepared.original, None);
    let node_index: HashMap<&str, usize> =
        prepared.model.nodes.iter().enumerate().map(|(i, n)| (n.id.as_str(), i)).collect();
    let mut rows = Vec::new();
    for case in &prepared.original.load_cases {
        let case_statuses: Vec<_> = statuses.iter().filter(|s| s.case_id == case.id).collect();
        if case_statuses.is_empty() || case_statuses.iter().any(|s| s.state != AppliedSelfWeightState::Fresh) {
            continue;
        }
        let mut scope = BTreeSet::new();
        let mut axis = None;
        let mut acceleration = None;
        let mut loads: Vec<(usize, [Dd; 6])> = Vec::new();
        let mut readable = true;
        for status in &case_statuses {
            let load = &case.primitive_loads[status.primitive_index];
            let Some(evidence) = evidence_of(load.provenance.as_deref()) else { readable = false; break };
            if evidence["method"] == MANUAL_OVERRIDE_METHOD {
                readable = false;
                break;
            }
            let pipe = if evidence["method"] == GENERATED_METHOD { evidence["pipe_id"].as_str() } else { evidence["omitted_pipe_id"].as_str() };
            scope.extend(pipe.map(str::to_string));
            let g = evidence["normalized_acceleration_m_per_s2"].as_f64();
            let k = evidence["gravity"]["axis"].as_str().and_then(|x| AXES.iter().position(|a| *a == x));
            if axis.is_some_and(|a| Some(a) != k) || acceleration.is_some_and(|x: f64| Some(x.to_bits()) != g.map(f64::to_bits)) {
                readable = false;
                break;
            }
            axis = k;
            acceleration = g;
            let mut magnitude = load.magnitude.clone();
            let mut scratch = Vec::new();
            let dimension = match load.dimension.as_str() {
                "force_per_length" => Dimension::ForcePerLength,
                "force" => Dimension::Force,
                _ => Dimension::Moment,
            };
            normalize_quantity(&mut magnitude, dimension, "diagnostic:modal:applied", Vec::new(), &mut scratch);
            if !scratch.is_empty() {
                readable = false;
                break;
            }
            let Some(k) = k else { readable = false; break };
            match &load.target {
                LoadTargetInput::Element { pipe } => {
                    let mut intensity = [0.0; 3];
                    intensity[k] = magnitude.value;
                    let vector = if let Some(element) = prepared.built.pipes.iter().find(|p| &p.element_id == pipe) {
                        GlobalUniformLoad::new(intensity).and_then(|l| element.equivalent_global_nodal_loads(&[l], &[])).ok().map(|v| (element.node_i.index, element.node_j.index, v))
                    } else if let Some(bend) = prepared.built.curved_bend_elements.iter().find(|b| &b.pipe_id == pipe) {
                        bend.macro_element.consistent_uniform_nodal_loads(intensity).ok().map(|v| (bend.node_i, bend.node_j, v))
                    } else {
                        None
                    };
                    let Some((i, j, v)) = vector else { readable = false; break };
                    loads.push((i, std::array::from_fn(|n| Dd::from_f64(v[n]))));
                    loads.push((j, std::array::from_fn(|n| Dd::from_f64(v[6 + n]))));
                }
                LoadTargetInput::Node { node } => {
                    let Some(&n) = node_index.get(node.as_str()) else { readable = false; break };
                    let index = match load.direction.as_str() {
                        "global_x" => 0,
                        "global_y" => 1,
                        "global_z" => 2,
                        "rotation_x" => 3,
                        "rotation_y" => 4,
                        _ => 5,
                    };
                    let mut vector = [Dd::ZERO; 6];
                    vector[index] = Dd::from_f64(magnitude.value);
                    loads.push((n, vector));
                }
                LoadTargetInput::Support { .. } => {
                    readable = false;
                    break;
                }
            }
        }
        let (Some(k), Some(g)) = (axis, acceleration) else { continue };
        if !readable {
            disclosures.push(format!("applied self-weight case {} is not compared: its generated primitives could not be read as one gravity case", case.id));
            continue;
        }
        if scope != members {
            disclosures.push(format!("applied self-weight case {} covers {} of the {} mass-bearing members; a partial-scope case is disclosed, not compared", case.id, scope.len(), members.len()));
            continue;
        }
        let product = mass.rigid_product(k);
        let formed: Vec<(usize, [Dd; 6])> =
            (0..mass.node_count).map(|n| (n, std::array::from_fn(|i| product[6 * n + i].mul_f64(g)))).collect();
        let f = nodal_resultant(formed, positions, point);
        let r = nodal_resultant(loads, positions, point);
        let scale = identity_scales(&to_f64s(&r), bounding_diagonal(positions));
        let relative = |range: std::ops::Range<usize>, scale: f64| {
            let d = range.map(|i| f[i].sub(r[i]).abs().to_f64()).fold(0.0, f64::max);
            if d == 0.0 { 0.0 } else if scale == 0.0 { f64::INFINITY } else { d / scale }
        };
        rows.push(AppliedCaseRow { case_id: case.id.clone(), axis: k, force_relative: relative(0..3, scale[0]), moment_relative: relative(3..6, scale[1]) });
    }
    rows
}
