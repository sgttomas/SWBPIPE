//! T4-U3: expansion-joint shapes (D-4) and the legacy refusal.
//!
//! Every `expansion_joint` component is exactly one of:
//! - an explicit objective connector (`objective_connector` present);
//! - an annotation (`solver_consumption = not_solver_consumed`, no connector):
//!   analysed as pipe, with a disclosure, on the pressure-free route only;
//!   refused on the exact route by the admission seam;
//! - a legacy joint: a pipe reference and/or any of the four scalar rates
//!   (a flexibility joint or an app-authored joint), or the residual shape
//!   with none of these. Both are refused on every route with
//!   `LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED`; nothing is converted.
//!
//! No joint is silently skipped or analysed as pipe without a disclosure.

use super::{diag, stable_suffix, Diagnostic, PreviewComponent, PreviewModel};

pub(crate) const LEGACY_CODE: &str = "LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED";
pub(crate) const ANNOTATION_CODE: &str = "EXPANSION_JOINT_ANNOTATION_ONLY";
const ANNOTATION_CONSUMPTION: &str = "not_solver_consumed";

/// The shape of an `expansion_joint` component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum JointShape {
    Connector,
    Annotation,
    /// A pipe reference and/or one of the four rates, any other consumption.
    Legacy,
    /// No connector, pipe reference, rates or annotation mode.
    Residual,
}

/// The joint shape of a component, or `None` when it is not an expansion joint.
pub(crate) fn joint_shape(component: &PreviewComponent) -> Option<JointShape> {
    if component.kind != "expansion_joint" {
        return None;
    }
    if component.objective_connector.is_some() {
        return Some(JointShape::Connector);
    }
    let consumption = component
        .mechanics_interface
        .as_ref()
        .and_then(|interface| interface.solver_consumption.as_deref());
    if consumption == Some(ANNOTATION_CONSUMPTION) {
        return Some(JointShape::Annotation);
    }
    Some(if has_legacy_joint_fields(component) {
        JointShape::Legacy
    } else {
        JointShape::Residual
    })
}

/// Whether a component takes the legacy refusal (both D-4 populations and the
/// residual shape). The exact route's admission loop skips these components.
pub(crate) fn is_legacy_joint(component: &PreviewComponent) -> bool {
    matches!(
        joint_shape(component),
        Some(JointShape::Legacy | JointShape::Residual)
    )
}

fn has_legacy_joint_fields(component: &PreviewComponent) -> bool {
    component
        .geometry
        .as_ref()
        .is_some_and(|geometry| geometry.expansion_joint_pipe_ref.is_some())
        || component.modifiers.as_ref().is_some_and(|modifiers| {
            modifiers.axial_stiffness_user_value.is_some()
                || modifiers.lateral_stiffness_user_value.is_some()
                || modifiers.angular_stiffness_user_value.is_some()
                || modifiers.torsional_stiffness_user_value.is_some()
        })
}

fn legacy_pipe_ref(component: &PreviewComponent) -> Option<&str> {
    component
        .geometry
        .as_ref()
        .and_then(|geometry| geometry.expansion_joint_pipe_ref.as_deref())
        .filter(|id| !id.trim().is_empty())
}

/// D-4: one blocking refusal per legacy joint, refs `[component, pipe]`
/// (`[component]` when no pipe is named), on every route and every document
/// version. Emitted first in the profile validation, so it precedes the exact
/// route's composition refusal.
pub(crate) fn refuse_legacy_joints(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    for component in &model.components {
        let Some(shape @ (JointShape::Legacy | JointShape::Residual)) = joint_shape(component)
        else {
            continue;
        };
        let pipe = legacy_pipe_ref(component);
        let mut refs = vec![component.id.clone()];
        refs.extend(pipe.map(str::to_string));
        let what = match (shape, pipe) {
            (JointShape::Residual, _) => format!(
                "expansion joint {} declares no objective connector, pipe mapping, stiffness rates or annotation mode",
                component.id
            ),
            (_, Some(pipe)) => format!(
                "expansion joint {} on pipe {pipe} is a legacy four-rate finite-span joint",
                component.id
            ),
            (_, None) => format!(
                "expansion joint {} is a legacy four-rate finite-span joint",
                component.id
            ),
        };
        diagnostics.push(diag(
            &format!(
                "diagnostic:joint:{}:legacy-reauthor",
                stable_suffix(&component.id)
            ),
            LEGACY_CODE,
            "blocking",
            format!(
                "{what}; it is neither solved nor converted, and it is not analysed as pipe. Re-author it as an objective connector (objective_connector 1.0.0) on a 3.0.0/exact_pressure_v3 document: the connector frame and both end attachments, a 6x6 scaled work matrix with its basis and provenance, the topology (replaces_span), the hardware and the pressure model"
            ),
            refs,
        ));
    }
}

/// The annotation disclosure, on the pressure-free route only (a model document
/// without a pressure contract): the joint is analysed as the pipe it sits on.
pub(crate) fn disclose_annotation_joints(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    if !crate::model_format::is_pressure_free(model) {
        return;
    }
    for component in &model.components {
        if joint_shape(component) != Some(JointShape::Annotation) {
            continue;
        }
        diagnostics.push(diag(
            &format!(
                "diagnostic:joint:{}:annotation-only",
                stable_suffix(&component.id)
            ),
            ANNOTATION_CODE,
            "info",
            format!(
                "expansion joint {} is an annotation only (solver_consumption=not_solver_consumed): the model is analysed as pipe, with no joint flexibility, hardware or pressure thrust, so no result represents the joint",
                component.id
            ),
            vec![component.id.clone()],
        ));
    }
}

// ------------------------------------------------------------------ the objective connector (J-B)

use open_pipe_stress_frame_kernel::connector::{
    ConnectorAttachment, ObjectiveConnector, ScaledWorkMatrix,
};
use open_pipe_stress_frame_kernel::tie_rod::{TieRod, TieRodError, TieRodInput};
use open_pipe_stress_frame_kernel::{FrameNode, Matrix3};
use open_pipe_stress_units::{canonical_unit, convert_for_dimension, unit_by_symbol, Dimension};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

/// The connector's stated law, added to v3's limitations when a model has one.
pub(crate) const CONNECTOR_LIMITATION: &str = "Objective connectors (expansion joints, objective_connector 1.0.0): symmetric-midpoint, small-displacement, small-rotation linear elastic law with the authored 6x6 work matrix, frame, offsets and installed reference q_ref, replacing exactly one straight span (replaces_span); untied hardware or explicit bilateral linear elastic tie rods (explicit_axial_rods_v1) between the connector's end nodes with rigid offsets, each with its own stiffness and reference extension. A pressurized joint (constant_effective_area_reference_v1) is a coaxial edge of one equal-bore pressure region between straight members, with zero attachment offsets, and carries the dead pressure load p(Ae - Ai) at its attachments, Ae being the authored effective area; unequal bore or pressure, offset faces and joints at region terminals or beside realized arcs are refused. Ideal inextensible rods, collective constraints, tension-only or gapped rods, nuts and stops are refused. No finite rotation, follower or geometric stiffness, pressure stiffening, squirm or stability, temperature law or stop is modelled; the replaced span carries no load, member result or self-weight. Connector actions are published as generalized and end-node elastic actions, with each tie-rod force, the joint pressure force pAe and the effective axial force S = g_x + sum T_r (n_r.Q_x) - Pe published separately; they are outside the straight-member recovery coverage.";
pub(crate) const INPUT_INCOMPLETE: &str = "OBJECTIVE_CONNECTOR_INPUT_INCOMPLETE";
pub(crate) const TOPOLOGY_UNRESOLVED: &str = "OBJECTIVE_CONNECTOR_TOPOLOGY_UNRESOLVED";
pub(crate) const BASIS_UNSUPPORTED: &str = "JOINT_STIFFNESS_BASIS_UNSUPPORTED";
pub(crate) const HARDWARE_NOT_DEFINED: &str = "JOINT_HARDWARE_NOT_DEFINED";
pub(crate) const HARDWARE_LAW_UNSUPPORTED: &str = "JOINT_HARDWARE_LAW_UNSUPPORTED";
pub(crate) const PRESSURE_INTERFACE_UNRESOLVED: &str = "JOINT_PRESSURE_INTERFACE_UNRESOLVED";
pub(crate) const PRESSURE_AREA_MISSING: &str = "JOINT_PRESSURE_AREA_MISSING";
pub(crate) const SPAN_LOAD_UNOWNED: &str = "JOINT_REPLACED_SPAN_LOAD_UNOWNED";
/// T4-U5b (DEL-04-07 Design `u4b-u5b-implementation.md` §3.1): a tie-rod
/// hardware record that is incomplete, contradictory or out of range.
pub(crate) const TIE_ROD_INPUT_INVALID: &str = "JOINT_TIE_ROD_INPUT_INVALID";
/// T4-U5b (§3.1): a rod whose represented length is degenerate (the FK
/// representation guard `tie_rod::LENGTH_GUARD`).
pub(crate) const TIE_ROD_ZERO_LENGTH: &str = "JOINT_TIE_ROD_ZERO_LENGTH";
/// T4-U5b: the explicit elastic tie-rod hardware kind and its one law.
pub(crate) const RODS_KIND: &str = "explicit_axial_rods_v1";
pub(crate) const ROD_LAW: &str = "bilateral_linear";
const ROD_KEYS: [&str; 7] = ["id", "law", "offset_i", "offset_j", "stiffness", "reference_extension", "source_reference"];
const CONNECTOR_CONSUMPTION: &str = "objective_connector";
const COORDINATE_ORDER: [&str; 6] = ["tx", "ty", "tz", "rx", "ry", "rz"];

/// The decoded `ObjectiveConnectorV1` (slot table §0.3: one decode feeds
/// every consumer). Every number is in SI (m, rad, N·m); the decode converts
/// its own units because the Value bypasses the model's unit normalization.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConnectorSpec {
    pub component_id: String,
    /// The replaced span (`replaces_span`), a straight pipe of the model.
    pub span_id: String,
    pub node_i: String,
    pub node_j: String,
    pub end_i: ConnectorAttachment,
    pub end_j: ConnectorAttachment,
    pub axes: Matrix3,
    pub stiffness: ScaledWorkMatrix,
    pub q_ref: [f64; 6],
    /// `reference_state = stress_free`: K q_ref = 0 is required.
    pub stress_free: bool,
    pub pressure: Option<JointPressureModel>,
    /// T4-U5b: the explicit elastic tie rods in authored order (empty:
    /// untied). Model findings own the hardware refusals and their
    /// precedence; a refused hardware record decodes to no rods here and is
    /// never built (its blocking finding stops the case first).
    pub tie_rods: Vec<TieRodSpec>,
}

/// T4-U5b (§3.1): one decoded `explicit_axial_rods_v1` rod, in SI: offsets
/// in the connector axes Q (m), stiffness N/m, reference extension m.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TieRodSpec {
    pub id: String,
    pub input: TieRodInput,
    pub source_reference: String,
}

/// The hardware of a connector Value: `Ok(rods)` (empty for `untied`), or
/// its findings, at most one per code (each diagnostic ID is per code).
/// Absent hardware is `JOINT_HARDWARE_NOT_DEFINED`, never untied. A kind or
/// rod law other than the two kinds and `bilateral_linear` is
/// `JOINT_HARDWARE_LAW_UNSUPPORTED`, naming it; any other defect of the
/// closed objects (§3.1) is `JOINT_TIE_ROD_INPUT_INVALID`.
fn hardware(connector: &Value, component: &str) -> Result<Vec<TieRodSpec>, Vec<Finding>> {
    let Some(hardware) = connector.get("hardware") else {
        return Err(vec![(HARDWARE_NOT_DEFINED, format!(
            "objective connector {component}: no hardware is declared (absent hardware is never taken as untied)"))]);
    };
    let object = hardware.as_object();
    let kind = text(hardware, "kind");
    let mut laws = Vec::new();
    let mut problems = Vec::new();
    let mut rods = Vec::new();
    match kind {
        Some("untied") => {
            for key in object.into_iter().flat_map(|o| o.keys()).filter(|k| *k != "kind") {
                problems.push(if key == "rods" {
                    "untied hardware carries rods (contradictory hardware)".to_string()
                } else {
                    format!("untied hardware has the unknown member {key}")
                });
            }
        }
        Some(RODS_KIND) => {
            for key in object.into_iter().flat_map(|o| o.keys()).filter(|k| !matches!(k.as_str(), "kind" | "rods")) {
                problems.push(format!("{RODS_KIND} hardware has the unknown member {key}"));
            }
            match hardware.get("rods").and_then(Value::as_array) {
                None => problems.push(format!("{RODS_KIND} hardware requires a rods array")),
                Some(list) if list.is_empty() => problems.push(format!("{RODS_KIND} hardware requires at least one rod")),
                Some(list) => {
                    let mut ids = HashSet::new();
                    let mut encoded = HashSet::new();
                    for (index, rod) in list.iter().enumerate() {
                        match tie_rod(rod, index, &mut laws) {
                            Ok(spec) => {
                                // Row IDs carry the encoded rod ID, so two IDs
                                // that encode alike are duplicates too.
                                if !ids.insert(spec.id.clone()) || !encoded.insert(stable_suffix(&spec.id)) {
                                    problems.push(format!("tie rod {} is a duplicate id", spec.id));
                                }
                                rods.push(spec);
                            }
                            Err(Some(problem)) => problems.push(problem),
                            Err(None) => {}
                        }
                    }
                }
            }
        }
        other => {
            return Err(vec![(HARDWARE_LAW_UNSUPPORTED, format!(
                "objective connector {component}: hardware {} is not supported; only untied and {RODS_KIND} are",
                other.unwrap_or("(no kind)")))]);
        }
    }
    let mut findings = Vec::new();
    if !laws.is_empty() {
        findings.push((HARDWARE_LAW_UNSUPPORTED, format!(
            "objective connector {component}: {}; only {ROD_LAW} is (ideal inextensible, tension-only and gapped rods, nuts, stops and collective constraints are refused)",
            laws.join("; "))));
    }
    if !problems.is_empty() {
        findings.push((TIE_ROD_INPUT_INVALID, format!(
            "objective connector {component}: invalid tie-rod hardware: {}; it is never assembled with assumed values",
            problems.join("; "))));
    }
    if findings.is_empty() { Ok(rods) } else { Err(findings) }
}

/// One closed `Rod` object (§3.1). `Err(Some(problem))` is an input defect;
/// `Err(None)` a law other than `bilateral_linear`, recorded in `laws`.
fn tie_rod(rod: &Value, index: usize, laws: &mut Vec<String>) -> Result<TieRodSpec, Option<String>> {
    let Some(object) = rod.as_object() else {
        return Err(Some(format!("rods[{index}] is not an object")));
    };
    let id = text(rod, "id").map(str::to_string);
    let name = id.clone().unwrap_or_else(|| format!("rods[{index}]"));
    let mut problems = Vec::new();
    if id.is_none() {
        problems.push("a nonempty id is required".to_string());
    }
    for key in object.keys().filter(|k| !ROD_KEYS.contains(&k.as_str())) {
        problems.push(format!("unknown member {key}"));
    }
    let mut unsupported = false;
    match rod.get("law") {
        None => problems.push("law is missing".into()),
        Some(Value::String(law)) if law == ROD_LAW => {}
        Some(Value::String(law)) if !law.trim().is_empty() => {
            laws.push(format!("tie rod {name} law {law} is not supported"));
            unsupported = true;
        }
        Some(_) => problems.push("law must be a nonempty string".into()),
    }
    let mut offset = |key: &str| -> Option<[f64; 3]> {
        let Some(value) = rod.get(key) else {
            problems.push(format!("{key} is missing"));
            return None;
        };
        if !value.as_object().is_some_and(|o| o.len() == 3 && ["x", "y", "z"].iter().all(|k| o.contains_key(*k))) {
            problems.push(format!("{key} must hold exactly x, y and z"));
            return None;
        }
        let mut out = [0.0; 3];
        for (slot, axis) in out.iter_mut().zip(["x", "y", "z"]) {
            match quantity(&value[axis], Dimension::Length) {
                Ok(v) => *slot = v,
                Err(problem) => {
                    problems.push(format!("{key}.{axis} {problem}"));
                    return None;
                }
            }
        }
        Some(out)
    };
    let offset_i = offset("offset_i");
    let offset_j = offset("offset_j");
    let stiffness = match rod.get("stiffness").map(|v| quantity(v, Dimension::LinearStiffness)) {
        None => { problems.push("stiffness is missing".into()); None }
        Some(Err(problem)) => { problems.push(format!("stiffness {problem}")); None }
        Some(Ok(k)) if k > 0.0 => Some(k),
        Some(Ok(_)) => { problems.push("stiffness must be positive".into()); None }
    };
    let reference_extension = match rod.get("reference_extension").map(|v| quantity(v, Dimension::Length)) {
        None => { problems.push("reference_extension is missing".into()); None }
        Some(Err(problem)) => { problems.push(format!("reference_extension {problem}")); None }
        Some(Ok(e)) => Some(e),
    };
    let source = text(rod, "source_reference");
    if source.is_none() {
        problems.push("a nonempty source_reference is required".into());
    }
    if !problems.is_empty() {
        return Err(Some(format!("tie rod {name}: {}", problems.join(", "))));
    }
    if unsupported {
        return Err(None);
    }
    Ok(TieRodSpec {
        id: id.unwrap(),
        input: TieRodInput {
            offset_i: offset_i.unwrap(),
            offset_j: offset_j.unwrap(),
            stiffness: stiffness.unwrap(),
            reference_extension: reference_extension.unwrap(),
        },
        source_reference: source.unwrap().to_string(),
    })
}

/// A closed `{value, unit}` Quantity of `dimension`, in SI (finite).
fn quantity(value: &Value, dimension: Dimension) -> Result<f64, String> {
    if !value.as_object().is_some_and(|o| o.len() == 2 && o.contains_key("value") && o.contains_key("unit")) {
        return Err("must be a closed {value, unit} quantity".into());
    }
    let number = value["value"].as_f64().ok_or("value must be a number")?;
    let unit = text(value, "unit").ok_or("unit must be a nonempty string")?;
    let converted = si(number, unit, dimension).map_err(|error| format!("is not a finite {} ({error})", dimension_name(dimension)))?;
    if converted.is_finite() { Ok(converted) } else { Err(format!("is not a finite {}", dimension_name(dimension))) }
}

fn dimension_name(dimension: Dimension) -> &'static str {
    match dimension {
        Dimension::Length => "length",
        Dimension::LinearStiffness => "force per length",
        _ => "quantity",
    }
}

/// The code of an FK rod refusal (§3.1): a degenerate represented length is
/// `JOINT_TIE_ROD_ZERO_LENGTH`; every other formation refusal (non-finite or
/// subnormal inputs, an unrepresentable offset arm, stiffness or reference
/// load) is `JOINT_TIE_ROD_INPUT_INVALID` (range).
fn tie_rod_code(error: &TieRodError) -> &'static str {
    match error {
        TieRodError::ZeroLength => TIE_ROD_ZERO_LENGTH,
        _ => TIE_ROD_INPUT_INVALID,
    }
}

/// The rods' formation findings on the model's own node coordinates (§3.1:
/// the zero-length guard and the range refusals, at construction). A
/// connector whose FK constructor refuses is left to the builder's
/// `OBJECTIVE_CONNECTOR_INPUT_INCOMPLETE`.
fn tie_rod_formation_findings(model: &PreviewModel, spec: &ConnectorSpec) -> Vec<Finding> {
    if spec.tie_rods.is_empty() {
        return Vec::new();
    }
    // Round 4: node positions are in the project length unit (before
    // `normalize_model_units`, from `validate_profile` and authoring) or in
    // metres (after it, when the unit reads "m"); the offsets are metres. The
    // rod formation and its degenerate-length guard run on SI positions.
    let Some(unit) = model.project.units.get("length").and_then(Value::as_str) else {
        return Vec::new();
    };
    let node = |id: &str| {
        model.nodes.iter().position(|n| n.id == id).and_then(|index| {
            let p = &model.nodes[index].position;
            let si = |v: f64| si(v, unit, Dimension::Length).ok();
            FrameNode::new(index, [si(p.x)?, si(p.y)?, si(p.z)?]).ok()
        })
    };
    let (Some(i), Some(j)) = (node(&spec.node_i), node(&spec.node_j)) else {
        return Vec::new();
    };
    let Ok(connector) = ObjectiveConnector::new(i, j, spec.end_i, spec.end_j, spec.axes, spec.stiffness, spec.q_ref) else {
        return Vec::new();
    };
    let mut by_code: Vec<(&'static str, Vec<String>)> = Vec::new();
    for rod in &spec.tie_rods {
        if let Err(error) = TieRod::new(&connector, rod.input) {
            let code = tie_rod_code(&error);
            let entry = match by_code.iter_mut().find(|(c, _)| *c == code) {
                Some(entry) => entry,
                None => { by_code.push((code, Vec::new())); by_code.last_mut().unwrap() }
            };
            entry.1.push(format!("tie rod {}: {error}", rod.id));
        }
    }
    by_code.into_iter().map(|(code, problems)| (code, format!(
        "objective connector {}: {}; it is never assembled with assumed values", spec.component_id, problems.join("; ")))).collect()
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct JointPressureModel {
    pub effective_area: f64,
    pub pressure_basis: String,
    pub source_reference: String,
}

fn pressure_model(connector: &Value) -> Result<Option<JointPressureModel>, Finding> {
    let model = &connector["pressure_model"];
    match text(model, "kind") {
        Some("unpressurized") if model.as_object().is_some_and(|object| object.len()==1) => Ok(None),
        Some("unpressurized") => Err((PRESSURE_INTERFACE_UNRESOLVED,"unpressurized pressure model carries only kind".into())),
        Some("constant_effective_area_reference_v1") => {
            let area = model.get("effective_area").and_then(|area| {
                if !area.as_object().is_some_and(|object| object.len()==2) {return None;}
                si(area["value"].as_f64()?, text(area, "unit")?, Dimension::Area).ok()
            }).filter(|area| *area > 0.0);
            let source = text(model, "source_reference");
            if area.is_none() || source.is_none() {
                return Err((PRESSURE_AREA_MISSING, "effective_area must be finite, positive and an area, with a nonempty source_reference".into()));
            }
            let basis = text(model, "pressure_basis").filter(|basis| *basis == "internal_differential_zero_external_v1");
            if basis.is_none() || model.as_object().map_or(true, |object| object.len() != 4) {
                return Err((PRESSURE_INTERFACE_UNRESOLVED, "constant effective-area pressure model requires exactly kind, effective_area, pressure_basis and source_reference, with internal_differential_zero_external_v1 basis".into()));
            }
            Ok(Some(JointPressureModel {
                effective_area: area.unwrap(), pressure_basis: basis.unwrap().into(), source_reference: source.unwrap().into(),
            }))
        }
        kind => Err((PRESSURE_INTERFACE_UNRESOLVED, format!("pressure model {} is not supported; only unpressurized and constant_effective_area_reference_v1 are", kind.unwrap_or("(none)")))),
    }
}

/// One refusal of a connector: (code, message).
type Finding = (&'static str, String);

fn text<'v>(value: &'v Value, key: &str) -> Option<&'v str> {
    value.get(key).and_then(Value::as_str).filter(|t| !t.trim().is_empty())
}

fn si(value: f64, unit: &str, dimension: Dimension) -> Result<f64, String> {
    if !value.is_finite() {
        return Err(format!("{value} is not finite"));
    }
    let from = unit_by_symbol(unit, dimension).map_err(|e| e.to_string())?;
    let to = canonical_unit(dimension).ok_or_else(|| "no canonical unit".to_string())?;
    convert_for_dimension(value, dimension, from, to).map_err(|e| e.to_string())
}

/// `{x, y, z, unit}` in SI.
fn si_vector(value: Option<&Value>, dimension: Dimension) -> Result<[f64; 3], String> {
    let value = value.ok_or("missing")?;
    let unit = text(value, "unit").ok_or("missing unit")?;
    let mut out = [0.0; 3];
    for (axis, key) in ["x", "y", "z"].into_iter().enumerate() {
        let number = value.get(key).and_then(Value::as_f64).ok_or("missing component")?;
        out[axis] = si(number, unit, dimension)?;
    }
    Ok(out)
}

/// A 3×3 row-major matrix whose columns are axes (finite entries).
fn matrix3(value: Option<&Value>) -> Result<Matrix3, String> {
    let rows = value.and_then(Value::as_array).filter(|r| r.len() == 3).ok_or("not a 3x3 matrix")?;
    let mut out = [[0.0; 3]; 3];
    for (row, slot) in rows.iter().zip(out.iter_mut()) {
        let entries = row.as_array().filter(|e| e.len() == 3).ok_or("not a 3x3 matrix")?;
        for (entry, cell) in entries.iter().zip(slot.iter_mut()) {
            *cell = entry.as_f64().filter(|v| v.is_finite()).ok_or("non-finite entry")?;
        }
    }
    Ok(out)
}

fn attachment(value: Option<&Value>) -> Result<(String, ConnectorAttachment), String> {
    let value = value.ok_or("missing")?;
    let node = text(value, "node_ref").ok_or("missing node_ref")?.to_string();
    let node_axes = matrix3(value.get("initial_node_axes_global"))
        .map_err(|e| format!("initial_node_axes_global {e}"))?;
    let offset_local = si_vector(value.get("offset_local"), Dimension::Length)
        .map_err(|e| format!("offset_local {e}"))?;
    Ok((node, ConnectorAttachment { node_axes, offset_local }))
}

fn has_provenance(value: Option<&Value>, keys: &[&str]) -> bool {
    value.is_some_and(|value| keys.iter().all(|key| text(value, key).is_some()))
}

/// Field-level decode (`OBJECTIVE_CONNECTOR_INPUT_INCOMPLETE` for every
/// missing, invalid or contradictory field). The representation checks of Q,
/// the triads and H are the FK constructor's (in the builder).
fn decode_fields(component: &PreviewComponent, connector: &Value) -> Result<ConnectorSpec, String> {
    let mut problems = Vec::new();
    if has_legacy_joint_fields(component) {
        problems.push("legacy pipe reference or scalar rates alongside the connector".to_string());
    }
    let consumption = component
        .mechanics_interface
        .as_ref()
        .and_then(|interface| interface.solver_consumption.as_deref());
    if !matches!(consumption, None | Some(CONNECTOR_CONSUMPTION)) {
        problems.push(format!(
            "solver_consumption {} contradicts the connector (absent or objective_connector only)",
            consumption.unwrap_or_default()
        ));
    }
    if text(connector, "motion_basis") != Some("symmetric_midpoint_small_rotation_v1") {
        problems.push("motion_basis must be symmetric_midpoint_small_rotation_v1".into());
    }
    let end_i = attachment(connector.get("end_i")).map_err(|e| format!("end_i {e}"));
    let end_j = attachment(connector.get("end_j")).map_err(|e| format!("end_j {e}"));
    let axes = matrix3(connector.get("connector_axes_global"))
        .map_err(|e| format!("connector_axes_global {e}"));
    // The readers require a positive absolute temperature (K); so does the
    // decode (T4-RV19 N-6): a value at or below absolute zero is refused.
    match installed_reference_temperature_k(connector) {
        Some(kelvin) if kelvin > 0.0 => {}
        Some(_) => problems.push("installed_reference_temperature must be above absolute zero".into()),
        None => problems.push("installed_reference_temperature is missing or invalid".into()),
    }
    if text(connector, "temperature_applicability") != Some("fixed_installed_parameters_v1") {
        problems.push("temperature_applicability must be fixed_installed_parameters_v1".into());
    }
    let stress_free = match text(connector, "reference_state") {
        Some("stress_free") => Some(true),
        Some("prestressed") => Some(false),
        _ => {
            problems.push("reference_state must be stress_free or prestressed".into());
            None
        }
    };
    let q_ref = connector.get("q_ref").ok_or_else(|| "q_ref missing".to_string()).and_then(|q| {
        let t = si_vector(q.get("translation"), Dimension::Length).map_err(|e| format!("q_ref translation {e}"))?;
        let r = si_vector(q.get("rotation"), Dimension::Angle).map_err(|e| format!("q_ref rotation {e}"))?;
        Ok([t[0], t[1], t[2], r[0], r[1], r[2]])
    });
    let stiffness = connector.get("stiffness").ok_or_else(|| "stiffness missing".to_string()).and_then(|k| {
        if text(k, "version") != Some("1.0.0") || text(k, "representation") != Some("scaled_work_coefficients_v1") {
            return Err("stiffness must be version 1.0.0 scaled_work_coefficients_v1".into());
        }
        let ls = k.get("translation_scale").ok_or("translation_scale missing")?;
        let ls = si(
            ls.get("value").and_then(Value::as_f64).ok_or("translation_scale value missing")?,
            text(ls, "unit").ok_or("translation_scale unit missing")?,
            Dimension::Length,
        )
        .map_err(|e| format!("translation_scale {e}"))?;
        let rotation = k.get("rotation_scale");
        if rotation.and_then(|r| r.get("value")).and_then(Value::as_f64) != Some(1.0)
            || rotation.and_then(|r| text(r, "unit")) != Some("rad")
        {
            return Err("rotation_scale must be exactly 1 rad".into());
        }
        if text(k, "coefficient_unit") != Some("N*m") {
            return Err("coefficient_unit must be N*m".into());
        }
        let order = k.get("coordinate_order").and_then(Value::as_array);
        if order.map(|o| o.iter().map(Value::as_str).collect::<Vec<_>>())
            != Some(COORDINATE_ORDER.iter().map(|c| Some(*c)).collect())
        {
            return Err("coordinate_order must be tx, ty, tz, rx, ry, rz".into());
        }
        let upper = k.get("upper_triangle").and_then(Value::as_array).filter(|u| u.len() == 21)
            .ok_or("upper_triangle must hold 21 numbers")?;
        let mut upper_triangle = [0.0; 21];
        for (slot, value) in upper_triangle.iter_mut().zip(upper) {
            *slot = value.as_f64().filter(|v| v.is_finite()).ok_or("upper_triangle entry not finite")?;
        }
        if !has_provenance(k.get("provenance"), &["source_reference", "measurement_restraints", "basis_transform_reference"]) {
            return Err("stiffness provenance is incomplete".into());
        }
        Ok(ScaledWorkMatrix { upper_triangle, translation_scale: ls })
    });
    if !has_provenance(
        connector.get("provenance"),
        &["source_reference", "measurement_restraints", "basis_transform_reference", "validity_statement"],
    ) {
        problems.push("provenance is incomplete".into());
    }
    let span = connector.get("topology").and_then(|t| text(t, "span_ref")).unwrap_or_default().to_string();
    match (end_i, end_j, axes, q_ref, stiffness, stress_free) {
        (Ok((node_i, end_i)), Ok((node_j, end_j)), Ok(axes), Ok(q_ref), Ok(stiffness), Some(stress_free))
            if problems.is_empty() =>
        {
            Ok(ConnectorSpec {
                component_id: component.id.clone(),
                span_id: span,
                node_i,
                node_j,
                end_i,
                end_j,
                axes,
                stiffness,
                q_ref,
                stress_free,
                // Model findings own the pressure refusal and its precedence.
                pressure: pressure_model(connector).ok().flatten(),
                // T4-U5b: likewise the hardware refusals (`model_findings`).
                tie_rods: hardware(connector, &component.id).unwrap_or_default(),
            })
        }
        (end_i, end_j, axes, q_ref, stiffness, _) => {
            for error in [end_i.err(), end_j.err(), axes.err(), q_ref.err(), stiffness.err()].into_iter().flatten() {
                problems.push(error);
            }
            Err(problems.join("; "))
        }
    }
}

/// The topology, basis, hardware and pressure findings of a decoded
/// connector on its model (`replaces_span` only).
fn model_findings(
    model: &PreviewModel,
    connector: &Value,
    spec: &ConnectorSpec,
    replaced: &mut HashMap<String, String>,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    let nodes: HashSet<&str> = model.nodes.iter().map(|n| n.id.as_str()).collect();
    let topology = connector.get("topology");
    let topology_problem = if spec.node_i == spec.node_j {
        Some("both ends name the same node".to_string())
    } else if !nodes.contains(spec.node_i.as_str()) || !nodes.contains(spec.node_j.as_str()) {
        Some("an end node is not in the model".to_string())
    } else if topology.and_then(|t| text(t, "type")) != Some("replaces_span") {
        Some(format!(
            "topology {} is not supported; only replaces_span is",
            topology.and_then(|t| text(t, "type")).unwrap_or("(none)")
        ))
    } else {
        match model.pipe_segments.iter().find(|p| p.id == spec.span_id) {
            None => Some(format!("replaces_span names {}, which is not a pipe of the model", spec.span_id)),
            Some(pipe) if !((pipe.from == spec.node_i && pipe.to == spec.node_j)
                || (pipe.from == spec.node_j && pipe.to == spec.node_i)) =>
            {
                Some(format!("pipe {} does not join the connector's end nodes", pipe.id))
            }
            Some(pipe) if super::curved_bend_realized_pipe_ids(model).contains(pipe.id.as_str()) => {
                Some(format!("pipe {} is a realized curved span", pipe.id))
            }
            Some(pipe) => match replaced.get(&pipe.id) {
                Some(other) => Some(format!("pipe {} is already replaced by {other}", pipe.id)),
                None => {
                    replaced.insert(pipe.id.clone(), spec.component_id.clone());
                    None
                }
            },
        }
    };
    if let Some(problem) = topology_problem {
        findings.push((TOPOLOGY_UNRESOLVED, format!(
            "objective connector {}: {problem}; the connector is not assembled", spec.component_id)));
    }
    match connector.get("calibration") {
        Some(c) if text(c, "kind") == Some("constant_structural_elasticity_v1")
            && c.get("includes_pressure_dependent_tangent") == Some(&Value::Bool(false))
            && text(c, "installed_geometry").is_some() => {}
        Some(_) => findings.push((BASIS_UNSUPPORTED, format!(
            "objective connector {}: only a constant structural elasticity calibration without pressure-dependent tangent effects is supported", spec.component_id))),
        None => findings.push((BASIS_UNSUPPORTED, format!(
            "objective connector {}: no stiffness calibration basis is declared", spec.component_id))),
    }
    // T4-U5b (§3.1): untied or explicit elastic tie rods; U5a's admission
    // condition 2 is lifted for valid rods. The rods' formation refusals
    // (degenerate length, range) follow their field findings.
    match hardware(connector, &spec.component_id) {
        Ok(_) => findings.extend(tie_rod_formation_findings(model, spec)),
        Err(hardware) => findings.extend(hardware),
    }
    // Only current 0.9.0 exact documents reach the classifier (`ExactContract::of`).
    if let Err(finding) = pressure_model(connector) {
        findings.push(finding);
    }
    findings
}

/// The v3 connector classifier (slot table §4.2), blocking codes only. A
/// connector that passes is admitted; its numbers are checked again by the
/// FK constructor in the builder.
pub(crate) fn classify_connectors(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    let mut replaced = HashMap::new();
    for component in &model.components {
        let Some(connector) = component.objective_connector.as_ref() else {
            continue;
        };
        if component.kind != "expansion_joint" {
            continue;
        }
        let push = |diagnostics: &mut Vec<Diagnostic>, code: &str, extra: Option<&str>, message: String| {
            let mut refs = vec![component.id.clone()];
            if let Some(extra) = extra {
                refs.push(extra.to_string());
            }
            diagnostics.push(diag(
                &format!("diagnostic:joint:{}:{}", stable_suffix(&component.id), stable_suffix(&format!("{code}:{}", extra.unwrap_or_default()))),
                code,
                "blocking",
                message,
                refs,
            ));
        };
        let spec = match decode_fields(component, connector) {
            Ok(spec) => spec,
            Err(problems) => {
                push(diagnostics, INPUT_INCOMPLETE, Some("objective_connector"), format!(
                    "objective connector {} is incomplete or invalid: {problems}; it is never assembled with assumed values", component.id));
                continue;
            }
        };
        let findings = model_findings(model, connector, &spec, &mut replaced);
        let model_refused = !findings.is_empty();
        let span_known = !findings.iter().any(|(code, _)| *code == TOPOLOGY_UNRESOLVED);
        for (code, message) in findings {
            push(diagnostics, code, None, message);
        }
        if span_known {
            refuse_applied_span_loads(model, &spec, diagnostics);
        }
        if !model_refused {
            classify_pressure_participation(model, &spec, diagnostics);
        }
    }
    {
        // Known means an actual component carrying the interface. A model
        // refusal does not turn that known connector into an unknown ID.
        let connector_ids = model.components.iter().filter(|component| component.kind=="expansion_joint" && component.objective_connector.is_some()).map(|component| component.id.as_str()).collect::<HashSet<_>>();
        for case in &model.load_cases {
            for region in case.pressure_regions.iter().flatten() {
                let mut seen = HashSet::new();
                for id in region.member_connector_ids.iter().flatten() {
                    if !connector_ids.contains(id.as_str()) || !seen.insert(id) {
                        pressure_finding(diagnostics, &case.id, region.id.as_deref().unwrap_or_default(), id,
                            "member_connector_ids must name unique objective connectors of the model", None);
                    }
                }
            }
        }
    }
}

/// `extra` names the region member the finding is about, when there is one
/// (the listed replaced span: t4_u5a RV-03 names it).
fn pressure_finding(diagnostics: &mut Vec<Diagnostic>, case: &str, region: &str, connector: &str, reason: &str, extra: Option<&str>) {
    let mut refs = vec![case.to_string(), region.to_string(), connector.to_string()];
    refs.extend(extra.map(str::to_string));
    diagnostics.push(diag(
        &format!("diagnostic:joint:{}:pressure:{}", stable_suffix(connector), stable_suffix(&format!("{case}:{region}"))),
        PRESSURE_INTERFACE_UNRESOLVED, "blocking", format!("load case {case}, region {region}, connector {connector}: {reason}"),
        refs,
    ));
}

const REPLACED_SPAN_LISTED: &str = "the replaced span cannot be a member pipe of the region";

fn classify_pressure_participation(model: &PreviewModel, spec: &ConnectorSpec, diagnostics: &mut Vec<Diagnostic>) {
    let arcs = super::curved_bend_realized_pipe_ids(model);
    for case in &model.load_cases {
        let owners = case.pressure_regions.iter().flatten().filter(|region| region.member_connector_ids.as_ref().is_some_and(|ids| ids.contains(&spec.component_id))).count();
        for region in case.pressure_regions.iter().flatten() {
            let members = region.member_pipe_ids.as_deref().unwrap_or_default();
            let terminals = region.terminals.as_deref().unwrap_or_default();
            let listed = region.member_connector_ids.as_ref().is_some_and(|ids| ids.contains(&spec.component_id));
            let at_end = |node: &str| node == spec.node_i || node == spec.node_j;
            let touches = model.pipe_segments.iter().any(|pipe| members.contains(&pipe.id) && (at_end(&pipe.from) || at_end(&pipe.to)))
                || terminals.iter().any(|terminal| terminal.node_ref.as_deref().is_some_and(at_end));
            let reason = if touches && !listed { Some("region touches the joint but does not list it") }
            else if !listed { None }
            else if owners != 1 { Some("a connector must belong to exactly one region within a load case") }
            else if spec.pressure.is_none() { Some("an unpressurized connector cannot be a pressure-region edge") }
            else if spec.pressure.as_ref().is_some_and(|pressure| region.pressure_basis.as_deref() != Some(&pressure.pressure_basis)) { Some("joint and region pressure bases differ") }
            else if terminals.iter().any(|terminal| terminal.node_ref.as_deref().is_some_and(at_end)) { Some("a joint face cannot be a region terminal") }
            else if members.contains(&spec.span_id) { Some(REPLACED_SPAN_LISTED) }
            else if spec.end_i.offset_local.iter().chain(&spec.end_j.offset_local).any(|offset| *offset != 0.0) { Some("pressurized joint attachment offsets must be exactly zero") }
            else if [&spec.node_i, &spec.node_j].iter().any(|node| {
                let faces = model.pipe_segments.iter().filter(|pipe| pipe.id != spec.span_id && (&pipe.from == *node || &pipe.to == *node)).collect::<Vec<_>>();
                faces.len() != 1 || !members.contains(&faces[0].id) || arcs.contains(faces[0].id.as_str())
                    || connector_specs(model).iter().any(|other| other.component_id != spec.component_id && (&other.node_i == *node || &other.node_j == *node))
            }) { Some("each joint face must meet exactly one straight member pipe of the same region") }
            else { None };
            let extra = (reason == Some(REPLACED_SPAN_LISTED)).then_some(spec.span_id.as_str());
            if let Some(reason) = reason { pressure_finding(diagnostics, &case.id, region.id.as_deref().unwrap_or_default(), &spec.component_id, reason, extra); }
        }
    }
}

/// Current authoring uses the same canonical decoder as public mechanics.
pub fn authoring_findings(value: &Value) -> Result<Vec<Diagnostic>, String> {
    let model: PreviewModel = serde_json::from_value(value.clone()).map_err(|error| error.to_string())?;
    if model.schema_version != crate::MODEL_SCHEMA_VERSION
        || super::exact_admission::ExactContract::of(&model).is_none() {
        return Err(format!("JOINT_PRESSURE_INTERFACE_UNRESOLVED: joint pressure authoring requires current model {} with 3.0.0/exact_pressure_v3", crate::MODEL_SCHEMA_VERSION));
    }
    let mut diagnostics = Vec::new();
    classify_connectors(&model, &mut diagnostics);
    Ok(diagnostics)
}

/// S20 (a) (RV6 C-1): a primitive targeting the replaced span in a case's
/// applied set refuses by presence (any category, magnitude or factor). The
/// applied set is every stored primitive on 0.3.0 and the case's
/// `analysis_state.load_sources` on 0.4.0. A stored primitive no case
/// applies has no effect and is admitted.
fn refuse_applied_span_loads(model: &PreviewModel, spec: &ConnectorSpec, diagnostics: &mut Vec<Diagnostic>) {
    for case in &model.load_cases {
        let applied: Option<HashSet<&str>> = case.analysis_state.value().map(|state| {
            state.load_sources.iter().map(|source| source.source_ref.as_str()).collect()
        });
        for load in &case.primitive_loads {
            let on_span = matches!(&load.target, super::LoadTargetInput::Element { pipe } if *pipe == spec.span_id);
            if !on_span || applied.as_ref().is_some_and(|set| !set.contains(load.id.as_str())) {
                continue;
            }
            diagnostics.push(diag(
                &format!("diagnostic:joint:{}:replaced-span-load:{}", stable_suffix(&spec.component_id), stable_suffix(&format!("{}:{}", case.id, load.id))),
                SPAN_LOAD_UNOWNED,
                "blocking",
                format!("load case {} applies {} ({}) to pipe {}, which objective connector {} replaces; the load has no owner until its producer is implemented and is neither moved to the end nodes nor dropped", case.id, load.id, load.category, spec.span_id, spec.component_id),
                vec![spec.component_id.clone(), spec.span_id.clone(), load.id.clone()],
            ));
        }
    }
}

/// The decoded spec of a component that is an expansion joint with an
/// objective connector whose fields decode (admitted after
/// `classify_connectors` found no blocking code); `None` for any other.
pub(crate) fn connector_spec(component: &PreviewComponent) -> Option<ConnectorSpec> {
    if component.kind != "expansion_joint" {
        return None;
    }
    decode_fields(component, component.objective_connector.as_ref()?).ok()
}

/// The admitted connectors' specs, in model order.
pub(crate) fn connector_specs(model: &PreviewModel) -> Vec<ConnectorSpec> {
    model
        .components
        .iter()
        .filter_map(|component| connector_spec(component))
        .collect()
}

/// The pipes replaced by admitted connectors (S20/S21 lookup set), by ID.
pub(crate) fn replaced_span_ids(model: &PreviewModel) -> HashSet<String> {
    if super::exact_admission::ExactContract::of(model) != Some(super::exact_admission::ExactContract::PressureV3) {
        return HashSet::new();
    }
    connector_specs(model).into_iter().map(|spec| spec.span_id).collect()
}

/// The FK connector of a decoded spec on the built nodes with its record, or
/// the blocking `OBJECTIVE_CONNECTOR_INPUT_INCOMPLETE` naming the
/// constructor's refusal (and the stress-free condition K q_ref = 0, decided
/// exactly). `span_index` is the replaced span's index among the built pipes;
/// a span that was not built is refused here too (its own pipe diagnostic is
/// already blocking, so this refusal never stands alone).
pub(crate) fn build_connector(
    spec: &ConnectorSpec,
    nodes: &[FrameNode],
    node_map: &HashMap<&str, usize>,
    span_index: Option<usize>,
) -> Result<(ObjectiveConnector, ConnectorRecord), Diagnostic> {
    let refuse = |message: String| {
        diag(
            &format!("diagnostic:joint:{}:connector-build", stable_suffix(&spec.component_id)),
            INPUT_INCOMPLETE,
            "blocking",
            format!("objective connector {} cannot be formed: {message}; it is never assembled with assumed values", spec.component_id),
            vec![spec.component_id.clone(), "objective_connector".to_string()],
        )
    };
    let node = |id: &str| {
        node_map
            .get(id)
            .and_then(|&index| nodes.iter().find(|n| n.index == index))
            .copied()
            .ok_or_else(|| refuse(format!("node {id} is not built")))
    };
    let connector = ObjectiveConnector::new(
        node(&spec.node_i)?,
        node(&spec.node_j)?,
        spec.end_i,
        spec.end_j,
        spec.axes,
        spec.stiffness,
        spec.q_ref,
    )
    .map_err(|error| refuse(error.to_string()))?;
    if spec.stress_free && !connector.stress_free() {
        return Err(refuse("reference_state stress_free requires K q_ref = 0 exactly".into()));
    }
    // T4-U5b: the rods ride on their connector, each its own element
    // (`ObjectiveConnector::with_tie_rods`). The model findings refuse every
    // rod the constructor refuses first; this is the same refusal by code.
    let inputs = spec.tie_rods.iter().map(|rod| rod.input).collect::<Vec<_>>();
    let connector = connector.with_tie_rods(&inputs).map_err(|(index, error)| {
        diag(
            &format!("diagnostic:joint:{}:tie-rod-build", stable_suffix(&spec.component_id)),
            tie_rod_code(&error),
            "blocking",
            format!("objective connector {}: tie rod {} cannot be formed: {error}; it is never assembled with assumed values", spec.component_id, spec.tie_rods[index].id),
            vec![spec.component_id.clone(), spec.tie_rods[index].id.clone()],
        )
    })?;
    let span_index = span_index.ok_or_else(|| refuse(format!("replaced span {} is not built", spec.span_id)))?;
    let record = ConnectorRecord {
        component_id: spec.component_id.clone(),
        span_id: spec.span_id.clone(),
        span_index,
        pressure: spec.pressure.clone(),
        tie_rods: spec.tie_rods.iter().map(|rod| TieRodRecord { id: rod.id.clone(), source_reference: rod.source_reference.clone() }).collect(),
    };
    Ok((connector, record))
}

/// The PP record of one built connector (unpriced; beside the FK type in
/// `BuiltModel`, as `CurvedBendMacroBuild.pipe_index` is for a bend).
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConnectorRecord {
    pub component_id: String,
    pub span_id: String,
    /// The replaced span's index in `BuiltModel::pipes`.
    pub span_index: usize,
    pub pressure: Option<JointPressureModel>,
    /// T4-U5b: each built rod's ID and source, in `ObjectiveConnector::tie_rods` order.
    pub tie_rods: Vec<TieRodRecord>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TieRodRecord {
    pub id: String,
    pub source_reference: String,
}

/// T4-U5b (§3.3): the rod force row's fixed metadata.
pub(crate) const TIE_ROD_FORCE_KIND: &str = "connector_tie_rod_force_v1";
pub(crate) const TIE_ROD_COMPONENT: &str = "t_r";
pub(crate) const TIE_ROD_LOCATION: &str = "tie_rod";
pub(crate) const TIE_ROD_FRAME: &str = "element_local";
pub(crate) const TIE_ROD_BASIS: &str = "recovered_from_tie_rod_extension_v1";
pub(crate) const TIE_ROD_SIGN: &str = "tension positive along the rod axis: T_r = k_r(e - e_ref), e = n.(v_bj - v_bi) with n from the end-i attachment to the end-j attachment; e_ref < 0 is pretension";

/// The row ID of a rod's force (§3.3).
pub(crate) fn tie_rod_row_id(component: &str, rod: &str) -> String {
    format!("result:connector:{}:tie-rod-force:{TIE_ROD_LOCATION}:{}:{TIE_ROD_COMPONENT}", stable_suffix(component), stable_suffix(rod))
}

/// S21 (and RV6 C-2): replaced spans are left out of every published
/// per-pipe evidence list, by pipe ID: `pipe_sections`, `pipe_materials`,
/// `pipe_stress_extrema`, the coverage's unavailable IDs, and the 0.4.0
/// `members` record and `member_state:<pipe>` contributions.
pub(crate) fn exclude_replaced_spans(evidence: &mut Value, replaced: &HashSet<&str>) {
    if replaced.is_empty() {
        return;
    }
    let keep = |item: &Value| {
        item.get("pipe_id")
            .and_then(Value::as_str)
            .map_or(true, |id| !replaced.contains(id))
    };
    for key in ["pipe_sections", "pipe_materials", "pipe_stress_extrema", "members"] {
        if let Some(items) = evidence.get_mut(key).and_then(Value::as_array_mut) {
            items.retain(keep);
        }
    }
    // D-5: the exact route's coverage is one record per section basis.
    let coverage = evidence.get_mut("stress_maximum_coverage");
    let records: Vec<&mut Value> = match coverage {
        Some(Value::Array(records)) => records.iter_mut().collect(),
        Some(record) => vec![record],
        None => Vec::new(),
    };
    for record in records {
        if let Some(ids) = record.get_mut("unavailable_pipe_ids").and_then(Value::as_array_mut) {
            ids.retain(|id| id.as_str().map_or(true, |id| !replaced.contains(id)));
        }
    }
    if let Some(items) = evidence.get_mut("contributions").and_then(Value::as_array_mut) {
        items.retain(|item| {
            item.get("source_id")
                .and_then(Value::as_str)
                .and_then(|id| id.strip_prefix("member_state:"))
                .map_or(true, |pipe| !replaced.contains(pipe))
        });
    }
}

/// S14: the connector's published rows, (kind, component, unit, values):
/// generalized translations and rotations q − q_ref (local), generalized
/// force and moment g (local), and the global end actions node-on-element.
/// They are a declared coverage limit: outside R-b′ and the straight-member
/// recovery bound (S-9).
pub(crate) fn connector_rows(
    recovery: &open_pipe_stress_frame_kernel::connector::ConnectorRecovery,
) -> [(&'static str, [&'static str; 3], &'static str, [f64; 3], &'static str); 8] {
    let e = &recovery.end_actions;
    [
        ("connector_generalized_translation_v1", ["qt_x", "qt_y", "qt_z"], "m", [recovery.deformation[0], recovery.deformation[1], recovery.deformation[2]], "connector_local"),
        ("connector_generalized_rotation_v1", ["qr_x", "qr_y", "qr_z"], "rad", [recovery.deformation[3], recovery.deformation[4], recovery.deformation[5]], "connector_local"),
        ("connector_generalized_force_v1", ["gt_x", "gt_y", "gt_z"], "N", [recovery.g[0], recovery.g[1], recovery.g[2]], "connector_local"),
        ("connector_generalized_moment_v1", ["gr_x", "gr_y", "gr_z"], "N*m", [recovery.g[3], recovery.g[4], recovery.g[5]], "connector_local"),
        ("connector_endpoint_force_v1", ["Fx", "Fy", "Fz"], "N", [e[0], e[1], e[2]], "end_i"),
        ("connector_endpoint_moment_v1", ["Mx", "My", "Mz"], "N*m", [e[3], e[4], e[5]], "end_i"),
        ("connector_endpoint_force_v1", ["Fx", "Fy", "Fz"], "N", [e[6], e[7], e[8]], "end_j"),
        ("connector_endpoint_moment_v1", ["Mx", "My", "Mz"], "N*m", [e[9], e[10], e[11]], "end_j"),
    ]
}

/// The disclosure that no joint temperature law is provided (JR §6 J3: the
/// replaced pipe's thermal expansion cannot supply one).
pub(crate) const TEMPERATURE_LAW_CODE: &str = "CONNECTOR_TEMPERATURE_LAW_NOT_PROVIDED";

fn is_pressure_v3(model: &PreviewModel) -> bool {
    super::exact_admission::ExactContract::of(model)
        == Some(super::exact_admission::ExactContract::PressureV3)
}

/// The installed reference temperature of a connector Value in kelvin.
fn installed_reference_temperature_k(connector: &Value) -> Option<f64> {
    let t = connector.get("installed_reference_temperature")?;
    si(t.get("value")?.as_f64()?, text(t, "unit")?, Dimension::Temperature).ok()
}

/// The authored connector Value of a decoded spec.
fn connector_value<'m>(model: &'m PreviewModel, spec: &ConnectorSpec) -> Option<&'m Value> {
    model
        .components
        .iter()
        .find(|c| c.id == spec.component_id)
        .and_then(|c| c.objective_connector.as_ref())
}

/// On a v3 model, one info disclosure per decoded objective connector, on
/// every result (emitted once per invocation by the input validation): the
/// connector's parameters are the authored ones at its installed reference
/// temperature, no joint temperature law acts, and the replaced pipe's thermal
/// expansion is not assigned to the joint.
pub(crate) fn disclose_connector_temperature_law(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    if !is_pressure_v3(model) {
        return;
    }
    for component in &model.components {
        let Some(spec) = connector_spec(component) else {
            continue;
        };
        let temperature = connector_value(model, &spec)
            .and_then(|c| c.get("installed_reference_temperature"))
            .map(|t| format!(" ({} {})", t.get("value").map(Value::to_string).unwrap_or_default(), text(t, "unit").unwrap_or_default()))
            .unwrap_or_default();
        diagnostics.push(diag(
            &format!("diagnostic:joint:{}:temperature-law", stable_suffix(&spec.component_id)),
            TEMPERATURE_LAW_CODE,
            "info",
            format!(
                "objective connector {} has no joint temperature law: its work matrix, frame, offsets and q_ref are the authored parameters at its installed reference temperature{temperature} and do not vary with any case temperature or thermal load, and the replaced pipe {}'s thermal expansion is not assigned to the joint",
                spec.component_id, spec.span_id
            ),
            vec![spec.component_id.clone(), spec.span_id.clone()],
        ));
    }
}

/// `contract_evidence.connector` on a v3 envelope: one record per admitted
/// objective connector, in model order (JR §7: topology and replaced span,
/// frame, reference temperature and q_ref, matrix scale and provenance), in
/// SI. The readers bind every connector row to its record. Empty on any
/// other contract.
///
/// T4-U5b (§3.3): `hardware` is the authored kind; with rods, `tie_rods`
/// lists each built rod (global offsets, stiffness, reference extension,
/// represented length, direction and axial share n.Q_x, as formed by FK).
pub(crate) fn connector_evidence(model: &PreviewModel, connectors: &[ObjectiveConnector], records: &[ConnectorRecord]) -> Value {
    if !is_pressure_v3(model) {
        return Value::Array(Vec::new());
    }
    let records = model
        .components
        .iter()
        .filter_map(|component| connector_spec(component))
        .map(|spec| {
            let authored = connector_value(model, &spec);
            let source = |value: Option<&Value>| value.and_then(|v| text(v, "source_reference")).unwrap_or_default().to_string();
            let mut record = serde_json::json!({
                "component_id": spec.component_id,
                "topology": "replaces_span",
                "replaced_pipe_id": spec.span_id,
                "node_i": spec.node_i,
                "node_j": spec.node_j,
                "motion_basis": "symmetric_midpoint_small_rotation_v1",
                "connector_axes_global": spec.axes,
                "end_i_node_axes_global": spec.end_i.node_axes,
                "end_j_node_axes_global": spec.end_j.node_axes,
                "end_i_offset_local_m": spec.end_i.offset_local,
                "end_j_offset_local_m": spec.end_j.offset_local,
                "q_ref": spec.q_ref,
                "reference_state": if spec.stress_free { "stress_free" } else { "prestressed" },
                "work_matrix": {
                    "representation": "scaled_work_coefficients_v1",
                    "coordinate_order": COORDINATE_ORDER,
                    "translation_scale_m": spec.stiffness.translation_scale,
                    "rotation_scale_rad": 1.0,
                    "coefficient_unit": "N*m",
                    "upper_triangle": spec.stiffness.upper_triangle.to_vec(),
                    "source_reference": source(authored.and_then(|c| c.get("stiffness")).and_then(|k| k.get("provenance"))),
                },
                "calibration": "constant_structural_elasticity_v1",
                "hardware": "untied",
                "pressure_model": "unpressurized",
                "temperature_applicability": "fixed_installed_parameters_v1",
                "installed_reference_temperature_k": authored.and_then(installed_reference_temperature_k),
                "provenance": source(authored.and_then(|c| c.get("provenance"))),
            });
            if let Some(pressure) = &spec.pressure {
                record["pressure_model"] = serde_json::json!("constant_effective_area_reference_v1");
                record["effective_area_m2"] = serde_json::json!(pressure.effective_area);
                record["pressure_basis"] = serde_json::json!(pressure.pressure_basis);
                record["pressure_source_reference"] = serde_json::json!(pressure.source_reference);
            }
            if !spec.tie_rods.is_empty() {
                record["hardware"] = serde_json::json!(RODS_KIND);
                let built = records.iter().position(|r| r.component_id == spec.component_id).and_then(|index| Some((connectors.get(index)?, &records[index])));
                if let Some((connector, built_record)) = built {
                    record["tie_rods"] = Value::Array(connector.tie_rods().iter().zip(&built_record.tie_rods).map(|(rod, meta)| {
                        let (o_i, o_j) = rod.offsets();
                        serde_json::json!({
                            "id": meta.id,
                            "law": ROD_LAW,
                            "offset_global_m": [o_i, o_j],
                            "stiffness_n_per_m": rod.stiffness(),
                            "reference_extension_m": rod.reference_extension(),
                            "length_m": rod.length(),
                            "direction_global": rod.direction(),
                            "axial_share": rod.axial_share(),
                            "source_reference": meta.source_reference,
                        })
                    }).collect());
                }
            }
            record
        })
        .collect();
    Value::Array(records)
}
