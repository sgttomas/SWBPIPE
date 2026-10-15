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
pub const KINDS: [(&str, &str, [&str; 3], &[&str]); 8] = [
    ("connector_generalized_translation_v1", "m", ["qt_x", "qt_y", "qt_z"], &[LOCAL]),
    ("connector_generalized_rotation_v1", "rad", ["qr_x", "qr_y", "qr_z"], &[LOCAL]),
    ("connector_generalized_force_v1", "N", ["gt_x", "gt_y", "gt_z"], &[LOCAL]),
    ("connector_generalized_moment_v1", "N*m", ["gr_x", "gr_y", "gr_z"], &[LOCAL]),
    ("connector_endpoint_force_v1", "N", ["Fx", "Fy", "Fz"], &["end_i", "end_j"]),
    ("connector_endpoint_moment_v1", "N*m", ["Mx", "My", "Mz"], &["end_i", "end_j"]),
    ("connector_pressure_force_v1", "N", ["pe_x"; 3], &[LOCAL]),
    ("connector_effective_axial_force_v1", "N", ["s_x"; 3], &[LOCAL]),
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
/// to the straight-member vocabulary under pressure-1 (components, frame, location, basis).
pub const VOCABULARY_COMPONENTS: [&str; 14] = [
    "qt_x", "qt_y", "qt_z", "qr_x", "qr_y", "qr_z", "gt_x", "gt_y", "gt_z", "gr_x", "gr_y", "gr_z", "pe_x", "s_x",
];
pub const VOCABULARY_COORDINATE_SYSTEMS: [&str; 1] = [LOCAL_FRAME];
pub const VOCABULARY_LOCATIONS: [&str; 1] = [LOCAL];

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
    let constant = r["pressure_model"] == "constant_effective_area_reference_v1";
    let mut record_keys = RECORD_KEYS.to_vec();
    if constant { record_keys.extend(["effective_area_m2", "pressure_basis", "pressure_source_reference"]); }
    require_connector(keys(r, &record_keys), "RECORD_SHAPE")?;
    if constant {
        require_connector(r["effective_area_m2"].as_f64().is_some_and(|area| area.is_finite() && area > 0.0)
            && r["pressure_basis"] == "internal_differential_zero_external_v1"
            && text(&r["pressure_source_reference"]).is_some(), "PRESSURE_MODEL")?;
    }
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
            && (r["pressure_model"] == "unpressurized" || constant)
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
    regions: &Value,
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
                    counts.get(&(*case, *component)) == Some(&(ROWS_PER_CASE + if records.iter().any(|record| record["component_id"] == *component && record["pressure_model"] == "constant_effective_area_reference_v1") { 2 } else { 0 })),
                    "ROW_COVERAGE",
                )?;
            }
        }
    }
    validate_pressure(records, regions, rows, cases, solved)?;
    Ok(())
}

pub(crate) const JOINT_APPROXIMATION: &str = "straight_lame_annulus_and_joint_area_difference_h2_small_strain_v3";
pub(crate) const ARC_JOINT_APPROXIMATION: &str = "straight_lame_annulus_realized_arc_and_joint_area_difference_h2_small_strain_v3";

fn number(value: &Value) -> Result<f64, String> {
    value.as_f64().filter(|value| value.is_finite()).ok_or_else(|| "SOURCE_PHYSICS_CONNECTOR_PRESSURE_NUMBER".into())
}
fn equal(value: f64, expected: f64, magnitudes: [f64; 2], projection_reserve: f64) -> bool {
    // Projection binding of separately displayed binary64 quantities. Scale
    // each operand before addition, so large cancellation cannot create an
    // infinite tolerance. The reserve is specific to each projection law;
    // separately rounded g-Pe needs one least subnormal. Pe itself is bound
    // exactly because both carriers use the same RN64(p*Ae).
    let tolerance = 64.0*f64::EPSILON*magnitudes[0].abs()
        + 64.0*f64::EPSILON*magnitudes[1].abs() + projection_reserve;
    value.is_finite() && expected.is_finite() && tolerance.is_finite()
        && (value - expected).abs() <= tolerance
}
fn validate_pressure(records: &[Value], regions: &Value, rows: &HashMap<&str,&Value>, cases: &HashSet<&str>, solved: bool) -> Check {
    let mut participating = HashMap::new();
    for region in regions.as_array().ok_or("SOURCE_PHYSICS_ARRAY_INVALID")? {
        let has_joint = region.get("joint_edges").is_some() || region.get("member_connector_ids").is_some();
        if !has_joint { continue; }
        let ids = region["member_connector_ids"].as_array().ok_or("SOURCE_PHYSICS_CONNECTOR_PRESSURE_MEMBERS")?;
        let edges = region["joint_edges"].as_array().ok_or("SOURCE_PHYSICS_CONNECTOR_PRESSURE_EDGES")?;
        require_connector(!ids.is_empty() && ids.len() == edges.len(), "PRESSURE_EDGE_COVERAGE")?;
        let case = text(&region["load_case_id"]).ok_or("SOURCE_PHYSICS_CONNECTOR_PRESSURE_CASE")?;
        let mut owned = HashSet::new();
        for edge in edges {
            require_connector(keys(edge, &["connector_id","node_i_ref","node_j_ref","face_i_pipe_id","face_j_pipe_id","p_pa","effective_area_m2","face_bore_area_m2","joint_pressure_force_n","area_difference_force_n","attachment_pair_global_n","representation"]), "PRESSURE_EDGE_SHAPE")?;
            let id = text(&edge["connector_id"]).ok_or("SOURCE_PHYSICS_CONNECTOR_PRESSURE_ID")?;
            let record = records.iter().find(|record| record["component_id"] == id).ok_or("SOURCE_PHYSICS_CONNECTOR_PRESSURE_UNBOUND")?;
            require_connector(record["pressure_model"] == "constant_effective_area_reference_v1" && ids.iter().filter(|entry| entry.as_str() == Some(id)).count() == 1
                && owned.insert(id) && participating.insert((case,id),edge).is_none(), "PRESSURE_EDGE_BINDING")?;
            let p = number(&region["p_pa"])?;
            let ae = number(&record["effective_area_m2"])?;
            let pe = p * ae;
            require_connector(edge["p_pa"] == region["p_pa"] && edge["effective_area_m2"] == record["effective_area_m2"]
                && number(&edge["joint_pressure_force_n"])? == pe
                && edge["node_i_ref"] == record["node_i"] && edge["node_j_ref"] == record["node_j"]
                && record["pressure_basis"] == region["pressure_basis"]
                && edge["representation"] == "B: p(Ae - Ai) at the attachments; joint_effective_area + joint_face_cap", "PRESSURE_EDGE_LAW")?;
            require_connector(numbers(&edge["face_bore_area_m2"],2) && numbers(&edge["area_difference_force_n"],2)
                && edge["attachment_pair_global_n"].as_array().is_some_and(|pair| pair.len()==2 && pair.iter().all(|v| numbers(v,3))), "PRESSURE_PAIR_SHAPE")?;
            require_connector(record["end_i_offset_local_m"].as_array().unwrap().iter().chain(record["end_j_offset_local_m"].as_array().unwrap()).all(|v| v.as_f64()==Some(0.0)), "PRESSURE_OFFSET")?;
            for (end, face) in ["face_i_pipe_id","face_j_pipe_id"].into_iter().enumerate() {
                let pipe = text(&edge[face]).ok_or("SOURCE_PHYSICS_CONNECTOR_PRESSURE_FACE")?;
                let terminals = region["terminals"].as_array().ok_or("SOURCE_PHYSICS_CONNECTOR_PRESSURE_TERMINALS")?;
                require_connector(region["member_pipe_ids"].as_array().is_some_and(|members| members.iter().any(|member| member == pipe))
                    && !terminals.iter().any(|terminal| terminal["node_ref"] == edge[if end==0 {"node_i_ref"} else {"node_j_ref"}]), "PRESSURE_FACE_BINDING")?;
                let geometry = region["geometry"].as_array().ok_or("SOURCE_PHYSICS_CONNECTOR_PRESSURE_GEOMETRY")?.iter().find(|geometry| geometry["pipe_id"] == pipe).ok_or("SOURCE_PHYSICS_CONNECTOR_PRESSURE_FACE")?;
                let ai = number(&geometry["Ai_m2"])?;
                require_connector(geometry.get("member_kind").is_none() && edge["face_bore_area_m2"][end].as_f64()==Some(ai), "PRESSURE_FACE_AREA")?;
                let pi = p*ai;
                let magnitudes = [pe,pi];
                require_connector(equal(number(&edge["area_difference_force_n"][end])?,pe-pi,magnitudes,2.0*f64::from_bits(1)), "PRESSURE_DIFFERENCE")?;
                for axis in 0..3 {
                    let d = number(&record["connector_axes_global"][axis][0])?;
                    require_connector(equal(number(&edge["attachment_pair_global_n"][end][axis])?,if end==0 {-(pe-pi)*d} else {(pe-pi)*d},magnitudes,2.0*f64::from_bits(1)), "PRESSURE_PAIR")?;
                }
            }
        }
    }
    if solved {
        for record in records {
            let id = text(&record["component_id"]).unwrap();
            for case in cases {
                let row = |kind,component| rows.values().find(|row| row["entity_ref"]==id && row["kind"]==kind && row["metadata"]["component"]==component && row["basis_ref"]["ref_id"]==*case).copied();
                let pe_row = row("connector_pressure_force_v1","pe_x");
                let s_row = row("connector_effective_axial_force_v1","s_x");
                if record["pressure_model"] == "unpressurized" {
                    require_connector(pe_row.is_none() && s_row.is_none(), "UNPRESSURIZED_ROWS")?;
                    continue;
                }
                let pe = participating.get(&(*case,id)).map_or(Ok(0.0),|edge| number(&edge["joint_pressure_force_n"]))?;
                let observed = number(&pe_row.ok_or("SOURCE_PHYSICS_CONNECTOR_PRESSURE_ROW_MISSING")?["value"])?;
                let g = number(&row("connector_generalized_force_v1","gt_x").ok_or("SOURCE_PHYSICS_CONNECTOR_PRESSURE_G_MISSING")?["value"])?;
                let s = number(&s_row.ok_or("SOURCE_PHYSICS_CONNECTOR_PRESSURE_ROW_MISSING")?["value"])?;
                require_connector(observed == pe, "PRESSURE_ROW_BINDING")?;
                require_connector(equal(s,g-pe,[g,pe],f64::from_bits(1)), "EFFECTIVE_ROW_BINDING")?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod projection_tests {
    use super::equal;
    #[test]
    fn u5a_reader_projection_cancellation_guard_stays_finite_and_covers_subnormals() {
        assert!(equal(0.0,0.0,[1e308,1e308],0.0));
        assert!(!equal(1e300,0.0,[1e308,1e308],0.0));
        let tiny=f64::from_bits(1);
        assert!(equal(0.0,tiny,[tiny,0.0],tiny));
        assert!(!equal(2.0*tiny,0.0,[tiny,0.0],tiny));
    }
}
