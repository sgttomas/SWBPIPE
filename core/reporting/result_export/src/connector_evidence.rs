//! T4-U3 (S14): the objective connector's evidence on `pressure-1`
//! (`3.0.0/exact_pressure_v3`) only. `contract_evidence.connector` holds one
//! record per connector (JR §7: topology and replaced span, frame, installed
//! reference temperature and q_ref, matrix scale and provenance), and every
//! `connector_*` row is bound to exactly one record. Any other contract
//! refuses both (`CONNECTOR_UNSUPPORTED`).
//!
//! The row signatures are the `connector_*` rows of
//! `fixtures/results/semantic_contract_v0_3_pressure_1.json`, held here as
//! code constants: the reader adds no table or static of its own (T4-RV14 F1).
//! The rows are a declared coverage limit (outside R-b′ and the straight-member
//! recovery bound); this reader checks their bindings and semantics, never
//! their values.
use serde_json::Value;
use std::collections::{HashMap, HashSet};

type Check = Result<(), String>;

pub(crate) const LOCAL: &str = "connector_local";
pub(crate) const LOCAL_FRAME: &str = "connector_axes_q";
pub(crate) const LOCAL_SIGN: &str = "generalized coordinates of the connector frame Q: q - q_ref and g = K(q - q_ref); positive along the connector axes";
pub(crate) const END_SIGN: &str =
    "global end action on the connector at its node (node on element), f = B^T g; the connector acts on its node with -f";
pub(crate) const MOTION_BASIS: &str = "symmetric_midpoint_small_rotation_v1";
/// (kind, unit, components, locations). Generalized rows are local; end rows
/// are global at `end_i` and `end_j`.
pub const KINDS: [(&str, &str, [&str; 3], &[&str]); 6] = [
    ("connector_generalized_translation_v1", "m", ["qt_x", "qt_y", "qt_z"], &[LOCAL]),
    ("connector_generalized_rotation_v1", "rad", ["qr_x", "qr_y", "qr_z"], &[LOCAL]),
    ("connector_generalized_force_v1", "N", ["gt_x", "gt_y", "gt_z"], &[LOCAL]),
    ("connector_generalized_moment_v1", "N*m", ["gr_x", "gr_y", "gr_z"], &[LOCAL]),
    ("connector_endpoint_force_v1", "N", ["Fx", "Fy", "Fz"], &["end_i", "end_j"]),
    ("connector_endpoint_moment_v1", "N*m", ["Mx", "My", "Mz"], &["end_i", "end_j"]),
];
/// Rows per connector per solved case: 4 local kinds × 3 + 2 end kinds × 3 × 2 ends.
pub(crate) const ROWS_PER_CASE: usize = 24;
const RECORD_KEYS: &[&str] = &[
    "component_id",
    "topology",
    "replaced_pipe_id",
    "node_i",
    "node_j",
    "motion_basis",
    "connector_axes_global",
    "end_i_node_axes_global",
    "end_j_node_axes_global",
    "end_i_offset_local_m",
    "end_j_offset_local_m",
    "q_ref",
    "reference_state",
    "work_matrix",
    "calibration",
    "hardware",
    "pressure_model",
    "temperature_applicability",
    "installed_reference_temperature_k",
    "provenance",
];
const MATRIX_KEYS: &[&str] = &[
    "representation",
    "coordinate_order",
    "translation_scale_m",
    "rotation_scale_rad",
    "coefficient_unit",
    "upper_triangle",
    "source_reference",
];

fn require_connector(ok: bool, code: &str) -> Check {
    if ok {
        Ok(())
    } else {
        Err(format!("SOURCE_PHYSICS_CONNECTOR_{code}"))
    }
}
fn keys(v: &Value, required: &[&str]) -> bool {
    v.as_object()
        .is_some_and(|o| o.len() == required.len() && required.iter().all(|k| o.contains_key(*k)))
}
fn text(v: &Value) -> Option<&str> {
    v.as_str().filter(|s| !s.is_empty())
}
fn numbers(v: &Value, len: usize) -> bool {
    v.as_array()
        .is_some_and(|a| a.len() == len && a.iter().all(|x| x.as_f64().is_some_and(f64::is_finite)))
}
fn matrix3(v: &Value) -> bool {
    v.as_array()
        .is_some_and(|rows| rows.len() == 3 && rows.iter().all(|row| numbers(row, 3)))
}

/// T4-U2 (export): the canonical metadata vocabulary the connector rows add
/// to physics-1's under pressure-1 (components, frame, location, basis).
pub const VOCABULARY_COMPONENTS: [&str; 12] = [
    "qt_x", "qt_y", "qt_z", "qr_x", "qr_y", "qr_z", "gt_x", "gt_y", "gt_z", "gr_x", "gr_y", "gr_z",
];
pub const VOCABULARY_COORDINATE_SYSTEMS: [&str; 1] = [LOCAL_FRAME];
pub const VOCABULARY_LOCATIONS: [&str; 1] = [LOCAL];

/// The pressure-1 signature of a connector row (the `connector_*` rows of
/// the pressure-1 table, held as code): `None` for another kind; a unit or
/// component outside the kind's is a contradiction, as `signature_in` says.
pub(crate) fn signature(row: &Value) -> Result<Option<Value>, String> {
    let kind = row["kind"].as_str().unwrap_or_default();
    let Some((_, unit, components, _)) = KINDS.iter().find(|(k, ..)| *k == kind) else {
        return Ok(None);
    };
    if row["unit"] != *unit {
        return Err(format!("SOURCE_UNIT_CONTRADICTION: {kind}"));
    }
    let component = row["metadata"]["component"].as_str().unwrap_or_default();
    if !components.contains(&component) {
        return Err(format!("SOURCE_COMPONENT_CONTRADICTION: {kind}"));
    }
    let (dimension, family) = match kind {
        "connector_generalized_translation_v1" => ("length", "displacement"),
        "connector_generalized_rotation_v1" => ("angle", "rotation"),
        "connector_generalized_force_v1" | "connector_endpoint_force_v1" => ("force", "force"),
        _ => ("moment", "moment"),
    };
    Ok(Some(serde_json::json!({"kind": kind, "unit": unit, "component": component,
        "source_physical_semantic_dimension": dimension, "derivative_target_dimension": dimension,
        "category": "physical_quantity", "family": family, "canonical_disposition": "exported_quantity",
        "legacy_declared_dimension": null, "legacy_run_creation_admission": "throws_ANALYSIS_RUN_RESULT_DIMENSION_UNDECLARED",
        "governing_ratio_eligible": false})))
}

/// The canonical metadata of a connector row under pressure-1's vocabulary
/// (physics-1's plus the connector entries above), or `None` when a value
/// lies outside it.
pub(crate) fn canonical_metadata(row: &Value) -> Option<Value> {
    let md = &row["metadata"];
    let kind = row["kind"].as_str()?;
    let (_, _, components, locations) = KINDS.iter().find(|(k, ..)| *k == kind)?;
    let ok = md["component"].as_str().is_some_and(|c| components.contains(&c))
        && md["location"].as_str().is_some_and(|l| locations.contains(&l))
        && matches!(md["coordinate_system"].as_str(), Some(LOCAL_FRAME | "global"))
        && md["basis"] == ROW_BASIS
        && md["sign_convention"].as_str().is_some_and(|s| !s.is_empty());
    ok.then(|| {
        serde_json::json!({"component": md["component"], "coordinate_system": md["coordinate_system"],
            "location": md["location"], "basis": md["basis"], "sign_convention": md["sign_convention"]})
    })
}

/// Whether a row kind belongs to the connector family (any version).
pub(crate) fn is_connector_kind(kind: &str) -> bool {
    kind.starts_with("connector_")
}

/// The row basis of every connector row: one stable string (T4-U2, the
/// export vocabulary). The replaced span is bound through the record named by
/// the row's `entity_ref`, never through the basis text.
pub const ROW_BASIS: &str = "objective_connector_v1;symmetric_midpoint_small_rotation_v1";

fn connector_record(r: &Value) -> Result<(&str, &str), String> {
    require_connector(keys(r, RECORD_KEYS), "RECORD_SHAPE")?;
    let (component, span, node_i, node_j) = (
        text(&r["component_id"]),
        text(&r["replaced_pipe_id"]),
        text(&r["node_i"]),
        text(&r["node_j"]),
    );
    require_connector(
        component.is_some()
            && span.is_some()
            && node_i.is_some()
            && node_j.is_some()
            && node_i != node_j
            && text(&r["provenance"]).is_some(),
        "RECORD_IDENTITY",
    )?;
    require_connector(
        r["topology"] == "replaces_span"
            && r["motion_basis"] == MOTION_BASIS
            && r["calibration"] == "constant_structural_elasticity_v1"
            && r["hardware"] == "untied"
            && r["pressure_model"] == "unpressurized"
            && r["temperature_applicability"] == "fixed_installed_parameters_v1"
            && matches!(r["reference_state"].as_str(), Some("stress_free" | "prestressed")),
        "RECORD_LAW",
    )?;
    require_connector(
        matrix3(&r["connector_axes_global"])
            && matrix3(&r["end_i_node_axes_global"])
            && matrix3(&r["end_j_node_axes_global"])
            && numbers(&r["end_i_offset_local_m"], 3)
            && numbers(&r["end_j_offset_local_m"], 3)
            && numbers(&r["q_ref"], 6)
            && r["installed_reference_temperature_k"]
                .as_f64()
                .is_some_and(|t| t.is_finite() && t > 0.0),
        "RECORD_FRAME",
    )?;
    let m = &r["work_matrix"];
    require_connector(
        keys(m, MATRIX_KEYS)
            && m["representation"] == "scaled_work_coefficients_v1"
            && m["coordinate_order"] == serde_json::json!(["tx", "ty", "tz", "rx", "ry", "rz"])
            && m["translation_scale_m"]
                .as_f64()
                .is_some_and(|s| s.is_finite() && s > 0.0)
            && m["rotation_scale_rad"].as_f64() == Some(1.0)
            && m["coefficient_unit"] == "N*m"
            && numbers(&m["upper_triangle"], 21)
            && text(&m["source_reference"]).is_some(),
        "WORK_MATRIX",
    )?;
    Ok((component.unwrap(), span.unwrap()))
}

/// The connector records and rows of one envelope. `admitted` is true only
/// under `pressure-1`; `members` are the published case members (pipe IDs of
/// any case); `cases` the exact case IDs; `solved` the mechanics status.
pub(crate) fn validate(
    evidence: &Value,
    rows: &HashMap<&str, &Value>,
    members: &HashSet<&str>,
    cases: &HashSet<&str>,
    solved: bool,
    admitted: bool,
) -> Check {
    let records = evidence.as_array().ok_or("SOURCE_PHYSICS_ARRAY_INVALID")?;
    require_connector(admitted || records.is_empty(), "UNSUPPORTED")?;
    let mut spans: HashMap<&str, &str> = HashMap::new();
    let mut replaced = HashSet::new();
    for r in records {
        let (component, span) = connector_record(r)?;
        require_connector(spans.insert(component, span).is_none(), "RECORD_DUPLICATE")?;
        require_connector(replaced.insert(span), "SPAN_DUPLICATE")?;
        // S21: a replaced span is in no published per-pipe list.
        require_connector(!members.contains(span), "REPLACED_SPAN_PUBLISHED")?;
    }
    let mut slots = HashSet::new();
    let mut counts: HashMap<(&str, &str), usize> = HashMap::new();
    for row in rows.values() {
        let entity = row["entity_ref"].as_str().unwrap_or_default();
        require_connector(!replaced.contains(entity), "REPLACED_SPAN_PUBLISHED")?;
        let kind = row["kind"].as_str().unwrap_or_default();
        if !is_connector_kind(kind) {
            continue;
        }
        require_connector(admitted, "UNSUPPORTED")?;
        let (_, unit, components, locations) = KINDS
            .iter()
            .find(|(k, ..)| *k == kind)
            .ok_or("SOURCE_PHYSICS_CONNECTOR_ROW_KIND")?;
        require_connector(spans.contains_key(entity), "ROW_UNBOUND")?;
        let md = &row["metadata"];
        require_connector(
            keys(md, &["component", "coordinate_system", "location", "basis", "sign_convention"]),
            "ROW_METADATA_SHAPE",
        )?;
        let component = md["component"].as_str().unwrap_or_default();
        let location = md["location"].as_str().unwrap_or_default();
        let local = location == LOCAL;
        require_connector(
            row["unit"] == *unit
                && components.contains(&component)
                && locations.contains(&location)
                && md["coordinate_system"] == if local { LOCAL_FRAME } else { "global" }
                && md["basis"] == ROW_BASIS
                && md["sign_convention"] == if local { LOCAL_SIGN } else { END_SIGN },
            "ROW_SEMANTICS",
        )?;
        let case = row["basis_ref"]["ref_id"].as_str().unwrap_or_default();
        require_connector(cases.contains(case), "ROW_CASE")?;
        require_connector(slots.insert((case, entity, kind, component, location)), "ROW_DUPLICATE")?;
        *counts.entry((case, entity)).or_default() += 1;
    }
    if solved {
        for case in cases {
            for component in spans.keys() {
                require_connector(
                    counts.get(&(*case, *component)) == Some(&ROWS_PER_CASE),
                    "ROW_COVERAGE",
                )?;
            }
        }
    }
    Ok(())
}
