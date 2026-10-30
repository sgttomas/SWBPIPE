//! T4-U3 (S14): the objective connector's evidence on `pressure-2`
//! (`3.0.0/exact_pressure_v3`) only. `contract_evidence.connector` holds one
//! record per connector (JR §7: topology and replaced span, frame, installed
//! reference temperature and q_ref, matrix scale and provenance), and every
//! `connector_*` row is bound to exactly one record. Any other contract
//! refuses both (`CONNECTOR_UNSUPPORTED`).
//!
//! The row signatures are the `connector_*` rows of
//! `fixtures/results/semantic_contract_v0_3_pressure_2.json`, held here as
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
/// T4-U5b (DEL-04-07 Design `u4b-u5b-implementation.md` §3.3): one force row
/// per explicit elastic tie rod, along the rod's own axis, bound to its
/// `tie_rods` evidence entry by row ID (not by metadata).
pub const TIE_ROD_KIND: &str = "connector_tie_rod_force_v1";
pub(crate) const TIE_ROD_LOCATION: &str = "tie_rod";
pub(crate) const TIE_ROD_FRAME: &str = "element_local";
pub const TIE_ROD_BASIS: &str = "recovered_from_tie_rod_extension_v1";
pub(crate) const TIE_ROD_SIGN: &str = "tension positive along the rod axis: T_r = k_r(e - e_ref), e = n.(v_bj - v_bi) with n from the end-i attachment to the end-j attachment; e_ref < 0 is pretension";
pub(crate) const RODS_KIND: &str = "explicit_axial_rods_v1";
const ROD_KEYS: &[&str] = &[
    "id",
    "law",
    "offset_global_m",
    "stiffness_n_per_m",
    "reference_extension_m",
    "length_m",
    "direction_global",
    "axial_share",
    "source_reference",
];
/// (kind, unit, components, locations). Generalized rows are local; end rows
/// are global at `end_i` and `end_j`; a rod's force row is at `tie_rod`.
pub const KINDS: [(&str, &str, [&str; 3], &[&str]); 9] = [
    ("connector_generalized_translation_v1", "m", ["qt_x", "qt_y", "qt_z"], &[LOCAL]),
    ("connector_generalized_rotation_v1", "rad", ["qr_x", "qr_y", "qr_z"], &[LOCAL]),
    ("connector_generalized_force_v1", "N", ["gt_x", "gt_y", "gt_z"], &[LOCAL]),
    ("connector_generalized_moment_v1", "N*m", ["gr_x", "gr_y", "gr_z"], &[LOCAL]),
    ("connector_endpoint_force_v1", "N", ["Fx", "Fy", "Fz"], &["end_i", "end_j"]),
    ("connector_endpoint_moment_v1", "N*m", ["Mx", "My", "Mz"], &["end_i", "end_j"]),
    ("connector_pressure_force_v1", "N", ["pe_x"; 3], &[LOCAL]),
    ("connector_effective_axial_force_v1", "N", ["s_x"; 3], &[LOCAL]),
    (TIE_ROD_KIND, "N", ["t_r"; 3], &[TIE_ROD_LOCATION]),
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
/// to the straight-member vocabulary under pressure-2 (components, frame, location, basis).
pub const VOCABULARY_COMPONENTS: [&str; 15] = [
    "qt_x", "qt_y", "qt_z", "qr_x", "qr_y", "qr_z", "gt_x", "gt_y", "gt_z", "gr_x", "gr_y", "gr_z", "pe_x", "s_x", "t_r",
];
/// The rod row's `element_local` is already in the straight vocabulary.
pub const VOCABULARY_COORDINATE_SYSTEMS: [&str; 1] = [LOCAL_FRAME];
pub const VOCABULARY_LOCATIONS: [&str; 2] = [LOCAL, TIE_ROD_LOCATION];
/// The connector rows' bases: the stable connector basis and (T4-U5b) the rod force's.
pub const VOCABULARY_BASES: [&str; 2] = [ROW_BASIS, TIE_ROD_BASIS];

pub(crate) fn canonical_metadata(row: &Value) -> Option<Value> {
    let md = &row["metadata"];
    let kind = row["kind"].as_str()?;
    let (_, _, components, locations) = KINDS.iter().find(|(k, ..)| *k == kind)?;
    if kind == TIE_ROD_KIND {
        let ok = md["component"] == "t_r"
            && md["location"] == TIE_ROD_LOCATION
            && md["coordinate_system"] == TIE_ROD_FRAME
            && md["basis"] == TIE_ROD_BASIS
            && md["sign_convention"].as_str().is_some_and(|s| !s.is_empty());
        return ok.then(|| {
            serde_json::json!({"component": md["component"], "coordinate_system": md["coordinate_system"],
                "location": md["location"], "basis": md["basis"], "sign_convention": md["sign_convention"]})
        });
    }
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

/// One validated `tie_rods` entry (§3.3): its ID and published axial share.
struct TieRod<'v> {
    id: &'v str,
    axial_share: f64,
}

/// The closed `tie_rods` entries of a record with rods (§3.3, §5.1): unique
/// IDs (also after the row-ID encoding), the `bilateral_linear` law, finite
/// global offsets, positive stiffness and length, a finite reference
/// extension, a unit direction and its axial share n·Q_x rounded once from
/// the record's Q (recomputed exactly here), and a nonempty source.
fn tie_rods<'v>(r: &'v Value) -> Result<Vec<TieRod<'v>>, String> {
    let entries = r["tie_rods"].as_array().filter(|e| !e.is_empty()).ok_or("SOURCE_PHYSICS_CONNECTOR_TIE_ROD_RECORD")?;
    let mut ids = HashSet::new();
    let mut encoded = HashSet::new();
    let mut out = Vec::with_capacity(entries.len());
    for rod in entries {
        require_connector(keys(rod, ROD_KEYS), "TIE_ROD_RECORD")?;
        let id = text(&rod["id"]).ok_or("SOURCE_PHYSICS_CONNECTOR_TIE_ROD_RECORD")?;
        require_connector(ids.insert(id) && encoded.insert(stable_suffix(id)), "TIE_ROD_DUPLICATE")?;
        let positive = |v: &Value| v.as_f64().is_some_and(|x| x.is_finite() && x > 0.0);
        require_connector(
            rod["law"] == "bilateral_linear"
                && rod["offset_global_m"].as_array().is_some_and(|o| o.len() == 2 && o.iter().all(|v| numbers(v, 3)))
                && positive(&rod["stiffness_n_per_m"])
                && rod["reference_extension_m"].as_f64().is_some_and(f64::is_finite)
                && positive(&rod["length_m"])
                && numbers(&rod["direction_global"], 3)
                && text(&rod["source_reference"]).is_some(),
            "TIE_ROD_RECORD",
        )?;
        let n: [f64; 3] = std::array::from_fn(|k| rod["direction_global"][k].as_f64().unwrap());
        // Each component of n = d/l is rounded once: |n|² is 1 to a few ulps.
        let squared = n.iter().map(|v| v * v).sum::<f64>();
        require_connector((squared - 1.0).abs() <= 16.0 * f64::EPSILON, "TIE_ROD_DIRECTION")?;
        let axis: [f64; 3] = std::array::from_fn(|k| r["connector_axes_global"][k][0].as_f64().unwrap_or(f64::NAN));
        let share = rod["axial_share"].as_f64().filter(|v| v.is_finite()).ok_or("SOURCE_PHYSICS_CONNECTOR_TIE_ROD_RECORD")?;
        require_connector(exact_dot3(n, axis).is_some_and(|dot| dot.to_bits() == share.to_bits()), "TIE_ROD_AXIAL_SHARE")?;
        out.push(TieRod { id, axial_share: share });
    }
    Ok(out)
}

fn stable_suffix(id: &str) -> String {
    id.replace(':', "-")
}

/// The base row ID of a rod's force (§3.3), as the producer forms it; a row
/// of any load case other than the first carries it under
/// `result:loadcase:{case}:` (`qualified_load_case_result_id`).
fn tie_rod_row_tail(component: &str, rod: &str) -> String {
    format!("connector:{}:tie-rod-force:{TIE_ROD_LOCATION}:{}:t_r", stable_suffix(component), stable_suffix(rod))
}
fn is_tie_rod_row_id(id: &str, case: &str, tail: &str) -> bool {
    id.strip_prefix("result:") == Some(tail)
        || id.strip_prefix(&format!("result:loadcase:{}:", stable_suffix(case))) == Some(tail)
}

fn connector_record(r: &Value) -> Result<(&str, &str), String> {
    let constant = r["pressure_model"] == "constant_effective_area_reference_v1";
    let rods = r["hardware"] == RODS_KIND;
    // §5.1: an untied record never carries rods (contradictory hardware).
    require_connector(!(r["hardware"] == "untied" && r.get("tie_rods").is_some()), "TIE_ROD_UNTIED")?;
    let mut record_keys = RECORD_KEYS.to_vec();
    if constant { record_keys.extend(["effective_area_m2", "pressure_basis", "pressure_source_reference"]); }
    if rods { record_keys.push("tie_rods"); }
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
            && (r["hardware"] == "untied" || rods)
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
    if rods {
        tie_rods(r)?;
    }
    Ok((component.unwrap(), span.unwrap()))
}

/// The connector records and rows of one envelope. `admitted` is true only
/// under `pressure-2`; `members` are the published case members (pipe IDs of
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
        let rod = kind == TIE_ROD_KIND;
        require_connector(
            row["unit"] == *unit
                && components.contains(&component)
                && locations.contains(&location)
                && md["coordinate_system"] == if rod { TIE_ROD_FRAME } else if local { LOCAL_FRAME } else { "global" }
                && md["basis"] == if rod { TIE_ROD_BASIS } else { ROW_BASIS }
                && md["sign_convention"] == if rod { TIE_ROD_SIGN } else if local { LOCAL_SIGN } else { END_SIGN },
            "ROW_SEMANTICS",
        )?;
        let case = row["basis_ref"]["ref_id"].as_str().unwrap_or_default();
        require_connector(cases.contains(case), "ROW_CASE")?;
        // T4-U5b: a rod row names its rod by its row ID, bound to one entry
        // of its connector's `tie_rods`; rows without a record are refused.
        let rod_id = if rod {
            let record = records.iter().find(|r| r["component_id"] == entity).ok_or("SOURCE_PHYSICS_CONNECTOR_ROW_UNBOUND")?;
            let id = row["id"].as_str().unwrap_or_default();
            let bound = record["tie_rods"].as_array().into_iter().flatten()
                .filter_map(|r| r["id"].as_str())
                .find(|rod| is_tie_rod_row_id(id, case, &tie_rod_row_tail(entity, rod)));
            bound.ok_or("SOURCE_PHYSICS_CONNECTOR_TIE_ROD_UNBOUND")?
        } else {
            ""
        };
        require_connector(slots.insert((case, entity, kind, component, location, rod_id)), "ROW_DUPLICATE")?;
        *counts.entry((case, entity)).or_default() += 1;
    }
    if solved {
        for case in cases {
            for component in spans.keys() {
                let record = records.iter().find(|record| record["component_id"] == *component).unwrap();
                let constant = record["pressure_model"] == "constant_effective_area_reference_v1";
                let rods = record["tie_rods"].as_array().map_or(0, Vec::len);
                // A rod record without its rows is refused by name (§5.1).
                for rod in record["tie_rods"].as_array().into_iter().flatten().filter_map(|r| r["id"].as_str()) {
                    require_connector(slots.contains(&(*case, *component, TIE_ROD_KIND, "t_r", TIE_ROD_LOCATION, rod)), "TIE_ROD_ROWS")?;
                }
                // 24 elastic rows, pe_x on a constant-effective-area joint, s_x
                // there and on every connector with rods, one row per rod.
                let expected = ROWS_PER_CASE + usize::from(constant) + usize::from(constant || rods > 0) + rods;
                require_connector(counts.get(&(*case, *component)) == Some(&expected), "ROW_COVERAGE")?;
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
                // T4-U6 (D-5, §1, §2.3): the joint face p(Ae - Ai) acts on the
                // pressure bore; the region geometry carries Ai only there.
                let ai = number(&geometry["pressure_bore"]["Ai_m2"])?;
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
            let rods = if record["hardware"] == RODS_KIND { tie_rods(record)? } else { Vec::new() };
            for case in cases {
                let row = |kind,component| rows.values().find(|row| row["entity_ref"]==id && row["kind"]==kind && row["metadata"]["component"]==component && row["basis_ref"]["ref_id"]==*case).copied();
                let pe_row = row("connector_pressure_force_v1","pe_x");
                let s_row = row("connector_effective_axial_force_v1","s_x");
                let pe = if record["pressure_model"] == "unpressurized" {
                    // T4-U5b: keyed on hardware. Untied, neither row; with
                    // rods, s_x is the face axial force with Pe = 0 and no pe_x.
                    require_connector(pe_row.is_none() && (s_row.is_some() == !rods.is_empty()), "UNPRESSURIZED_ROWS")?;
                    if rods.is_empty() { continue; }
                    0.0
                } else {
                    let pe = participating.get(&(*case,id)).map_or(Ok(0.0),|edge| number(&edge["joint_pressure_force_n"]))?;
                    let observed = number(&pe_row.ok_or("SOURCE_PHYSICS_CONNECTOR_PRESSURE_ROW_MISSING")?["value"])?;
                    require_connector(observed == pe, "PRESSURE_ROW_BINDING")?;
                    pe
                };
                let g = number(&row("connector_generalized_force_v1","gt_x").ok_or("SOURCE_PHYSICS_CONNECTOR_PRESSURE_G_MISSING")?["value"])?;
                let s = number(&s_row.ok_or("SOURCE_PHYSICS_CONNECTOR_PRESSURE_ROW_MISSING")?["value"])?;
                let mut terms = Vec::with_capacity(rods.len());
                for rod in &rods {
                    let tail = tie_rod_row_tail(id, rod.id);
                    let t = rows.values().find(|row| row["kind"] == TIE_ROD_KIND && row["entity_ref"] == id
                        && row["basis_ref"]["ref_id"] == *case && is_tie_rod_row_id(row["id"].as_str().unwrap_or_default(), case, &tail))
                        .ok_or("SOURCE_PHYSICS_CONNECTOR_TIE_ROD_ROWS")?;
                    terms.push((number(&t["value"])?, rod.axial_share));
                }
                require_connector(effective_axial_binding(s, g, pe, &terms), "EFFECTIVE_ROW_BINDING")?;
            }
        }
    }
    Ok(())
}

/// §5.1's s_x binding: |s_x − Σ| ≤ (64 + n_r)ε(|g_x| + |Pe| + Σ|t_r·axial_share|)
/// + (n_r + 2)·2^-1074, with Σ = g_x − Pe + Σ_r RN(t_r·axial_share_r)
/// accumulated exactly and rounded once, so the reader adds no addition
/// error for any rod count. Each term is scaled before it is added, so a
/// large cancellation cannot make the tolerance infinite. With no rods it is
/// U5a's rule (64ε|g| + 64ε|Pe| and a least-subnormal reserve, two here).
pub(crate) fn effective_axial_binding(s: f64, g: f64, pe: f64, rods: &[(f64, f64)]) -> bool {
    let products: Vec<f64> = rods.iter().map(|(t, share)| t * share).collect();
    let mut sum = ExactSum::default();
    for value in [g, -pe].into_iter().chain(products.iter().copied()) {
        if !value.is_finite() {
            return false;
        }
        sum.add(value);
    }
    let Some(expected) = sum.round() else { return false };
    let n = rods.len() as f64;
    let coefficient = (64.0 + n) * f64::EPSILON;
    let tolerance = coefficient * g.abs() + coefficient * pe.abs()
        + products.iter().map(|p| coefficient * p.abs()).sum::<f64>()
        + (n + 2.0) * f64::from_bits(1);
    s.is_finite() && tolerance.is_finite() && (s - expected).abs() <= tolerance
}

/// RN(a·b + c·d + e·f) for binary64 operands, from the exact products
/// (TwoProduct: p = fl(a·b) and its error fma(a, b, −p), exact while no
/// product underflows; a product below 2^-969 is refused), summed exactly
/// and rounded once. `None` outside that range.
fn exact_dot3(a: [f64; 3], b: [f64; 3]) -> Option<f64> {
    let mut sum = ExactSum::default();
    for k in 0..3 {
        let p = a[k] * b[k];
        if !p.is_finite() || (p != 0.0 && p.abs() < f64::from_bits(54 << 52)) {
            return None;
        }
        sum.add(p);
        sum.add(a[k].mul_add(b[k], -p));
    }
    sum.round()
}

/// An exact sum of binary64 values in fixed point at 2^-1074 (every finite
/// binary64 is an integer multiple of it), two's complement over 36 limbs:
/// up to 2^256 terms of the largest magnitude without overflow.
#[derive(Clone)]
pub(crate) struct ExactSum {
    limbs: [u64; 36],
}
impl Default for ExactSum {
    fn default() -> Self {
        Self { limbs: [0; 36] }
    }
}
impl ExactSum {
    /// Adds a finite binary64 exactly.
    pub(crate) fn add(&mut self, value: f64) {
        debug_assert!(value.is_finite());
        let bits = value.to_bits();
        let exponent = ((bits >> 52) & 0x7ff) as usize;
        let fraction = bits & ((1u64 << 52) - 1);
        let (mantissa, shift) = if exponent == 0 { (fraction, 0) } else { (fraction | (1u64 << 52), exponent - 1) };
        if mantissa == 0 {
            return;
        }
        // value = ±mantissa · 2^(shift − 1074).
        let mut addend = [0u64; 36];
        let (limb, offset) = (shift / 64, shift % 64);
        let wide = (mantissa as u128) << offset;
        addend[limb] = wide as u64;
        addend[limb + 1] = (wide >> 64) as u64;
        if bits >> 63 == 1 {
            // Two's complement negation.
            let mut carry = 1u64;
            for slot in addend.iter_mut() {
                let (v, c) = (!*slot).overflowing_add(carry);
                *slot = v;
                carry = u64::from(c);
            }
        }
        let mut carry = 0u64;
        for (slot, add) in self.limbs.iter_mut().zip(addend) {
            let (v, c1) = slot.overflowing_add(add);
            let (v, c2) = v.overflowing_add(carry);
            *slot = v;
            carry = u64::from(c1) + u64::from(c2);
        }
    }
    /// The sum rounded once to nearest, ties to even; `None` on overflow.
    pub(crate) fn round(&self) -> Option<f64> {
        let negative = self.limbs[35] >> 63 == 1;
        let mut magnitude = self.limbs;
        if negative {
            let mut carry = 1u64;
            for slot in magnitude.iter_mut() {
                let (v, c) = (!*slot).overflowing_add(carry);
                *slot = v;
                carry = u64::from(c);
            }
        }
        let top = (0..36).rev().find(|&i| magnitude[i] != 0);
        let Some(top) = top else { return Some(0.0) };
        let high = 64 * top + 63 - magnitude[top].leading_zeros() as usize; // index of the leading bit
        let bit = |i: usize| (magnitude[i / 64] >> (i % 64)) & 1;
        // Subnormal range: the integer itself is exactly representable.
        let (mut mantissa, mut scale) = if high < 53 {
            (magnitude[0], 0usize)
        } else {
            let low = high - 52;
            let mut m = 0u64;
            for i in (low..=high).rev() {
                m = (m << 1) | bit(i);
            }
            let half = bit(low - 1) == 1;
            let sticky = (0..low - 1).any(|i| bit(i) == 1);
            if half && (sticky || m & 1 == 1) {
                m += 1;
            }
            (m, low)
        };
        if mantissa == 1u64 << 53 {
            mantissa >>= 1;
            scale += 1;
        }
        // value = mantissa · 2^(scale − 1074); build it without double rounding.
        let exponent = scale as i64 - 1074;
        let value = if mantissa == 0 {
            0.0
        } else {
            let lead = 63 - mantissa.leading_zeros() as i64; // ≤ 52
            let e = exponent + lead; // unbiased exponent of the leading bit
            if e > 1023 {
                return None;
            }
            if e >= -1022 {
                let fraction = (mantissa << (52 - lead)) & ((1u64 << 52) - 1);
                f64::from_bits(((e + 1023) as u64) << 52 | fraction)
            } else {
                // Subnormal: exponent is −1074 exactly here (scale = 0).
                f64::from_bits(mantissa)
            }
        };
        Some(if negative { -value } else { value })
    }
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

#[cfg(test)]
mod u5b_effective_axial_tests {
    //! T4-U5b (DEL-04-07 Design `u4b-u5b-implementation.md` §5.1): the s_x
    //! binding with rods and its exact sum.
    use super::{effective_axial_binding, exact_dot3, ExactSum};
    const TINY: f64 = f64::from_bits(1);

    fn exact(values: &[f64]) -> Option<f64> {
        let mut sum = ExactSum::default();
        values.iter().for_each(|v| sum.add(*v));
        sum.round()
    }

    #[test]
    fn exact_sum_rounds_once_across_the_whole_range() {
        assert_eq!(exact(&[]), Some(0.0));
        assert_eq!(exact(&[1.0, 2f64.powi(-60), -1.0]), Some(2f64.powi(-60)));
        assert_eq!(exact(&[f64::MAX, -f64::MAX, TINY]), Some(TINY));
        assert_eq!(exact(&[f64::MAX, f64::MAX]), None);
        assert_eq!(exact(&[TINY, TINY, TINY]), Some(3.0 * TINY));
        assert_eq!(exact(&[f64::MIN_POSITIVE, -TINY]), Some(f64::MIN_POSITIVE - TINY));
        // Ties to even: 1 + 2^-53 is a tie between 1 and 1 + 2^-52.
        assert_eq!(exact(&[1.0, 2f64.powi(-53)]), Some(1.0));
        assert_eq!(exact(&[1.0, 2f64.powi(-53), 2f64.powi(-100)]), Some(1.0 + f64::EPSILON));
        // −1.5 − 2^-53 is a tie as well, to the even −1.5; a further term breaks it.
        assert_eq!(exact(&[-1.5, -2f64.powi(-53)]), Some(-1.5));
        assert_eq!(exact(&[-1.5, -2f64.powi(-53), -2f64.powi(-1000)]), Some(-1.5 - f64::EPSILON));
        // A thousand terms below the ulp of 1 are not lost.
        let mut many = vec![1.0];
        many.extend(std::iter::repeat_n(2f64.powi(-54), 1024));
        assert_eq!(exact(&many), Some(1.0 + 2f64.powi(-44)));
        assert_eq!(exact_dot3([0.6, 0.0, 0.8], [0.6, 0.0, 0.8]), Some(0.6f64.mul_add(0.6, 0.8 * 0.8)));
    }

    #[test]
    fn rods_enter_s_x_exactly_with_their_coefficient_and_reserve() {
        // g_x + T·share − Pe exactly, with cancellation.
        assert!(effective_axial_binding(0.0, 553.25, 800.0, &[(246.75, 1.0)]));
        assert!(!effective_axial_binding(1e-9, 553.25, 800.0, &[(246.75, 1.0)]));
        // The projected share: SK-F's literal form is refused.
        let (t, share) = (144000.0 / 1661.0, 0.8);
        let g = 800.0 - t * share;
        assert!(effective_axial_binding(0.0, g, 800.0, &[(t, share)]));
        assert!(!effective_axial_binding(g + t - 800.0, g, 800.0, &[(t, share)]));
        // Subnormal edges: Σ = 2·2^-1074 and (n_r + 2) = 3 least subnormals of
        // reserve (the relative term underflows to zero), no more.
        assert!(effective_axial_binding(5.0 * TINY, TINY, 0.0, &[(TINY, 1.0)]));
        assert!(effective_axial_binding(-TINY, TINY, 0.0, &[(TINY, 1.0)]));
        assert!(!effective_axial_binding(6.0 * TINY, TINY, 0.0, &[(TINY, 1.0)]));
        // Unpressurized with rods: Pe = 0, s_x is the face axial force.
        assert!(effective_axial_binding(1010.0, 10.0, 0.0, &[(4500.0 / 2009.0, 1.0), (2004500.0 / 2009.0, 1.0)]));
        // Non-finite or overflowing operands are refused, never tolerated.
        assert!(!effective_axial_binding(f64::NAN, 1.0, 0.0, &[]));
        assert!(!effective_axial_binding(f64::MAX, f64::MAX, -f64::MAX, &[]));
    }

    #[test]
    fn a_thousand_rods_add_no_reader_error() {
        // 1,024 parallel rods each contributing 2^-54 beside g_x = 1: the
        // exact Σ is 1 + 2^-44; the (64 + n_r)ε coefficient grows with n_r and
        // still refuses a value off by more than it.
        let rods = vec![(2f64.powi(-54), 1.0); 1024];
        let sum = 1.0 + 2f64.powi(-44);
        assert!(effective_axial_binding(sum, 1.0, 0.0, &rods));
        let tolerance = (64.0 + 1024.0) * f64::EPSILON * (1.0 + 1024.0 * 2f64.powi(-54)) + 1026.0 * TINY;
        assert!(effective_axial_binding(sum + 0.5 * tolerance, 1.0, 0.0, &rods));
        assert!(!effective_axial_binding(sum + 2.0 * tolerance, 1.0, 0.0, &rods));
    }
}

#[cfg(test)]
mod d5_face_area_tests {
    //! T4-U6 (D-5, §1, §2.3, §2.5): a joint face's bore area is the region
    //! geometry's `pressure_bore.Ai_m2`, on distinct bases (a nonzero corrosion
    //! allowance puts the pressure bore off the nominal bore).
    use super::validate_pressure;
    use serde_json::{json, Value};
    use std::collections::{HashMap, HashSet};
    use std::f64::consts::PI;

    const P: f64 = 2.0e6;
    const AE: f64 = 0.0125;
    const OD: f64 = 0.1143;
    const WALL: f64 = 0.006;
    const ALLOWANCE: f64 = 0.0015;

    fn annulus(wall: f64) -> Value {
        let (ro, ri) = (OD * 0.5, OD * 0.5 - wall);
        let area = (PI * wall) * (OD - wall);
        let second = (area * ((ro * ro) + (ri * ri))) * 0.25;
        json!({"wall_m": wall, "ri_m": ri, "As_m2": area, "I_m4": second, "J_m4": second * 2.0, "Z_m3": second / ro})
    }
    fn bore(wall: f64) -> f64 {
        let ri = OD * 0.5 - wall;
        (PI * ri) * ri
    }
    /// One region geometry entry in the D-5 shape (§2.5, plus `traversal_forward`).
    fn geometry(pipe: &str) -> Value {
        let reduced = (WALL - 0.0) - ALLOWANCE;
        json!({"pipe_id": pipe, "geometry_basis": "authored_od_wall_d5_sections_v1", "outside_diameter_m": OD,
            "ro_m": OD * 0.5, "wall_thickness_m": WALL, "mill_tolerance_m": 0.0, "corrosion_allowance_m": ALLOWANCE,
            "nominal_section": annulus(WALL), "reduced_section": annulus(reduced),
            "pressure_bore": {"ri_m": OD * 0.5 - reduced, "Ai_m2": bore(reduced)},
            "beam_theory": {"model": "euler_bernoulli"}, "traversal_forward": true})
    }
    /// A joint region whose edge states `face` as both faces' bore area, with
    /// the area difference and attachment pair projected from it.
    fn region(geometry: [Value; 2], face: f64) -> Value {
        let pe = P * AE;
        let difference = pe - P * face;
        json!([{"load_case_id": "case:A", "p_pa": P, "pressure_basis": "internal_differential_zero_external_v1",
            "member_pipe_ids": ["pipe:A", "pipe:B"], "member_connector_ids": ["connector:J"],
            "terminals": [{"node_ref": "node:0"}, {"node_ref": "node:3"}], "geometry": geometry,
            "joint_edges": [{"connector_id": "connector:J", "node_i_ref": "node:1", "node_j_ref": "node:2",
                "face_i_pipe_id": "pipe:A", "face_j_pipe_id": "pipe:B", "p_pa": P, "effective_area_m2": AE,
                "face_bore_area_m2": [face, face], "joint_pressure_force_n": pe,
                "area_difference_force_n": [difference, difference],
                "attachment_pair_global_n": [[-difference, 0.0, 0.0], [difference, 0.0, 0.0]],
                "representation": "B: p(Ae - Ai) at the attachments; joint_effective_area + joint_face_cap"}]}])
    }
    fn check(regions: &Value) -> Result<(), String> {
        let record = json!({"component_id": "connector:J", "pressure_model": "constant_effective_area_reference_v1",
            "effective_area_m2": AE, "node_i": "node:1", "node_j": "node:2",
            "pressure_basis": "internal_differential_zero_external_v1",
            "end_i_offset_local_m": [0.0, 0.0, 0.0], "end_j_offset_local_m": [0.0, 0.0, 0.0],
            "connector_axes_global": [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]});
        validate_pressure(&[record], regions, &HashMap::new(), &HashSet::new(), false)
    }

    #[test]
    fn u6_joint_face_area_is_the_d5_pressure_bore() {
        let reduced = (WALL - 0.0) - ALLOWANCE;
        assert_ne!(bore(reduced), bore(WALL));
        // The D-5 region geometry is read: the face is the pressure bore.
        assert_eq!(check(&region([geometry("pipe:A"), geometry("pipe:B")], bore(reduced))), Ok(()));
        // A face on the nominal bore contradicts the pressure bore.
        assert_eq!(check(&region([geometry("pipe:A"), geometry("pipe:B")], bore(WALL))),
            Err("SOURCE_PHYSICS_CONNECTOR_PRESSURE_FACE_AREA".into()));
        // A stray single-section `Ai_m2` beside the D-5 entry is not the face
        // area: the pre-D-5 key, set to the nominal bore, does not bind it.
        let stray = |pipe: &str| {
            let mut entry = geometry(pipe);
            entry["Ai_m2"] = json!(bore(WALL));
            entry
        };
        assert_eq!(check(&region([stray("pipe:A"), stray("pipe:B")], bore(WALL))),
            Err("SOURCE_PHYSICS_CONNECTOR_PRESSURE_FACE_AREA".into()));
        // An entry without the pressure bore states no face area.
        let bare = |pipe: &str| {
            let mut entry = geometry(pipe);
            entry.as_object_mut().unwrap().remove("pressure_bore");
            entry
        };
        assert_eq!(check(&region([bare("pipe:A"), bare("pipe:B")], bore(reduced))),
            Err("SOURCE_PHYSICS_CONNECTOR_PRESSURE_NUMBER".into()));
    }
}
