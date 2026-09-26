//! Closed admission of the load-reference-1 raw evidence namespace
//! (`contract_evidence.load_reference_states` beside `pressure`, `connector` and
//! `exact_cases`). These checks establish internal source consistency, not
//! solver accuracy, producer origin or model freshness.
//!
//! The order of checks and every error string are shared with the Python
//! reader `core/analysis_runs/load_reference_evidence.py`; change both together.
//! Checks iterate arrays in document order so that the first failure is
//! deterministic in both languages.
//!
//! After the load-reference-specific pre-pass, the unchanged physics-1
//! validator checks rows, extrema, regions and the pressure RHS on a projected
//! copy. The projection only removes or neutralizes fields that the pre-pass has
//! already bound: `load_reference_states`, the two extra resolved-member keys of
//! `pipe_materials`, the `material_basis` constant and the region
//! `temperature_basis` constant.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

type Check = Result<(), String>;

pub const RECORD_CONTRACT: &str = "openpipestress.load_reference_state/1.0.0";
pub const MATERIAL_BASIS: &str = "resolved_per_member_load_reference_state_v1";
pub const NOT_JOINED: &str = "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED";
pub const REGION_TEMPERATURE_BASIS: &str = "resolved_member_state";
pub const G_BASIS: &str = "E/[2(1+nu)] from the selected pair";
pub const COMPOSITION: &str = "lambda_fit*lambda_thermal-1";
pub const BOUNDARY: &str = "every restrained DOF prescribed; reduced K_ff u_f = f_f - K_fc g_c; complete u includes g; reactions from unreduced K u - f";
pub const EIGENLOAD: &str =
    "axial E_member*A_s*total_eigenstrain assembled once and removed once in recovery";
/// Frozen `schemas/load_reference_state.schema.json` (relayed by the manager).
pub const TRANSPORT_SCHEMA_SHA256: &str =
    "640fd4477ac2c84f3c02268cfccc3958f51ee6508b3a67e5538b52d84899af65";
const TRANSPORT_SCHEMA_BYTES: &str =
    include_str!("../../../../schemas/load_reference_state.schema.json");

const EVIDENCE_KEYS: &[&str] = &[
    "pressure",
    "connector",
    "exact_cases",
    "load_reference_states",
];
const CASE_KEYS: &[&str] = &[
    "load_case_id",
    "profile_mode",
    "material_basis",
    "pipe_materials",
    "pipe_sections",
    "pipe_stress_extrema",
    "stress_maximum_coverage",
    "pressure_rhs_assembly",
];
const MATERIAL_KEYS: &[&str] = &[
    "pipe_id",
    "material_id",
    "E_pa",
    "nu",
    "G_pa",
    "constitutive_basis",
    "material_selection_kind",
    "thermal_consumed",
    "alpha_per_kelvin",
    "resolved_eigenstrain",
    "provenance",
];
const RECORD_KEYS: &[&str] = &[
    "load_case_id",
    "contract",
    "profile",
    "reference_configuration_id",
    "provenance",
    "reference_geometry",
    "history",
    "solve",
    "source_recovery",
    "members",
    "support_components",
    "contributions",
    "excluded_sources",
];
const MEMBER_KEYS: &[&str] = &[
    "pipe_id",
    "material_id",
    "material_selection_kind",
    "consumed_material_points",
    "interpolation_fraction",
    "applicability_reference",
    "analysis_basis_override",
    "selected_E_pa",
    "selected_nu",
    "derived_G_pa",
    "G_basis",
    "retained_G_ignored",
    "operating_temperature_k",
    "material_selection_temperature_k",
    "reference_basis",
    "installation_temperature_k",
    "thermal_definition",
    "expansion_law_id",
    "coefficient_datum_k",
    "consumed_law_point_indices",
    "consumed_law_segments",
    "consulted_law_point_indices",
    "consulted_law_segments",
    "installation_datum_stretch",
    "operating_datum_stretch",
    "thermal_strain",
    "thermal_stretch",
    "fit_strain",
    "fit_stretch",
    "total_eigenstrain",
    "eigenstrain_composition",
    "fit_kind",
    "fit_input",
    "reference_length_m",
];
const POINT_KEYS: &[&str] = &[
    "point_id",
    "temperature_k",
    "E_pa",
    "nu",
    "retained_G_ignored",
];
const SEGMENT_KEYS: &[&str] = &["use", "lower_index", "upper_index", "start_k", "end_k"];
const SUPPORT_KEYS: &[&str] = &[
    "support_id",
    "node_id",
    "dof",
    "global_dof",
    "law_kind",
    "prescribed_value",
    "unit",
    "meaning",
    "physical_state_source",
];
const STORED_KEYS: &[&str] = &[
    "source_id",
    "owner_kind",
    "classification",
    "factor",
    "category",
    "dimension",
    "authored_normalized_magnitude",
    "applied_magnitude",
];
const MEMBER_STATE_KEYS: &[&str] = &[
    "source_id",
    "owner_kind",
    "classification",
    "consumed_input_refs",
    "value",
];
const SUPPORT_STATE_KEYS: &[&str] = &["source_id", "owner_kind", "classification", "value"];
const PRESSURE_REGION_KEYS: &[&str] = &["source_id", "owner_kind", "classification", "factor"];
const EXCLUDED_KEYS: &[&str] = &[
    "source_id",
    "owner_kind",
    "classification",
    "category",
    "reason",
];
const REGION_KEYS: &[&str] = &[
    "profile_version",
    "profile_mode",
    "load_case_id",
    "region_id",
    "member_pipe_ids",
    "pressure_basis",
    "p_pa",
    "external_pressure_increment_pa",
    "approximation",
    "geometry_representation_guard",
    "geometry",
    "materials",
    "applied_loads",
    "terminals",
    "provenance",
    "result_ids",
];
const REGION_MATERIAL_KEYS: &[&str] = &[
    "pipe_id",
    "material_id",
    "E_pa",
    "nu",
    "G_pa",
    "constitutive_basis",
    "thermal_consumed",
    "alpha_per_kelvin",
    "provenance",
    "temperature_basis",
];
const DOFS: &[&str] = &["UX", "UY", "UZ", "RX", "RY", "RZ"];
const LAW_DEFINITIONS: &[&str] = &[
    "engineering_secant",
    "engineering_dilation",
    "differential_per_datum_length",
    "logarithmic_per_current_length",
];
const DIRECT_DEFINITIONS: &[&str] = &[
    "unchanged_reference",
    "explicit_interval_strain",
    "constant_alpha_interval",
];

fn code(name: &str) -> String {
    format!("SOURCE_LOAD_REFERENCE_{name}")
}
fn require(ok: bool, name: &str) -> Check {
    if ok {
        Ok(())
    } else {
        Err(code(name))
    }
}
fn keys(v: &Value, required: &[&str]) -> bool {
    v.as_object()
        .is_some_and(|o| o.len() == required.len() && required.iter().all(|k| o.contains_key(*k)))
}
fn text(v: &Value) -> Result<&str, String> {
    v.as_str()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| code("STRING_INVALID"))
}
fn number(v: &Value) -> Result<f64, String> {
    v.as_f64()
        .filter(|n| n.is_finite())
        .ok_or_else(|| code("NUMBER_INVALID"))
}
fn opt_number(v: &Value) -> Result<Option<f64>, String> {
    if v.is_null() {
        Ok(None)
    } else {
        number(v).map(Some)
    }
}
fn opt_text(v: &Value) -> Result<Option<&str>, String> {
    if v.is_null() {
        Ok(None)
    } else {
        text(v).map(Some)
    }
}
fn boolean(v: &Value) -> Result<bool, String> {
    v.as_bool().ok_or_else(|| code("BOOLEAN_INVALID"))
}
fn index(v: &Value) -> Result<u64, String> {
    v.as_u64().ok_or_else(|| code("INTEGER_INVALID"))
}
fn array(v: &Value) -> Result<&Vec<Value>, String> {
    v.as_array().ok_or_else(|| code("ARRAY_INVALID"))
}
fn num_eq(a: &Value, b: &Value) -> bool {
    match (a.as_f64(), b.as_f64()) {
        (Some(x), Some(y)) => x.is_finite() && y.is_finite() && x == y,
        _ => false,
    }
}
/// Structural equality with numeric comparison by binary64 value, so that the
/// Rust and Python readers agree on `1` versus `1.0`.
fn same(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(_), Value::Number(_)) => num_eq(a, b),
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(p, q)| same(p, q))
        }
        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len() && x.iter().all(|(k, v)| y.get(k).is_some_and(|w| same(v, w)))
        }
        _ => a == b,
    }
}
fn without(v: &Value, removed: &[&str]) -> Value {
    let mut copy = v.clone();
    if let Some(o) = copy.as_object_mut() {
        for key in removed {
            o.remove(*key);
        }
    }
    copy
}
fn sha64(v: &Value) -> bool {
    v.as_str().is_some_and(|t| {
        t.len() == 64
            && t.bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    })
}
fn finite_tree(v: &Value) -> Check {
    match v {
        Value::Number(_) => {
            number(v)?;
        }
        Value::Array(a) => {
            for x in a {
                finite_tree(x)?;
            }
        }
        Value::Object(o) => {
            for x in o.values() {
                finite_tree(x)?;
            }
        }
        _ => {}
    }
    Ok(())
}
fn ulps(a: f64, b: f64) -> u64 {
    a.to_bits().abs_diff(b.to_bits())
}
fn positive(value: Option<f64>) -> bool {
    value.is_none_or(|v| v > 0.0)
}

/// Frozen closed carrier schema for the transport-only check.
pub fn transport_schema() -> Result<&'static Value, String> {
    static SCHEMA: OnceLock<Result<Value, String>> = OnceLock::new();
    SCHEMA
        .get_or_init(|| {
            let digest = format!("{:x}", Sha256::digest(TRANSPORT_SCHEMA_BYTES.as_bytes()));
            if digest != TRANSPORT_SCHEMA_SHA256 {
                return Err(code("TRANSPORT_SCHEMA_HASH"));
            }
            serde_json::from_str(TRANSPORT_SCHEMA_BYTES).map_err(|_| code("TRANSPORT_SCHEMA_HASH"))
        })
        .as_ref()
        .map_err(Clone::clone)
}

/// Raw load-reference-1 publication: closed evidence, cross-bindings, rows and
/// diagnostics. Never authenticates the producer.
pub fn validate_load_reference_evidence(source: &Value) -> Check {
    validate(source, true)
}

/// Retained load-reference-1 statements without raw rows or diagnostics:
/// frozen schema shape, closed evidence and internal joins only. This cannot
/// qualify a publication or a current invocation.
pub fn validate_load_reference_transport_metadata(source: &Value) -> Check {
    validate(source, false)
}

fn validate(source: &Value, raw: bool) -> Check {
    // S1 foreign method namespaces.
    require(
        source.get("source_block_recovery").is_none() && source.get("carrier_evidence").is_none(),
        "FOREIGN_METHOD_EVIDENCE",
    )?;
    // S2 finite numbers everywhere.
    finite_tree(source)?;
    let evidence = &source["contract_evidence"];
    if !raw {
        let schema = transport_schema()?;
        require(
            crate::source_blocks::shape_in(
                evidence,
                &schema["$defs"]["LoadReferenceContractEvidence"],
                schema,
            ),
            "TRANSPORT_SHAPE",
        )?;
    }
    // S3-S5 namespace.
    require(keys(evidence, EVIDENCE_KEYS), "EVIDENCE_SHAPE")?;
    require(
        array(&evidence["connector"])?.is_empty(),
        "CONNECTOR_UNSUPPORTED",
    )?;
    let pressure = array(&evidence["pressure"])?;
    let exact = array(&evidence["exact_cases"])?;
    let records = array(&evidence["load_reference_states"])?;
    // S6 numerical case identities.
    let mut quality_ids: Vec<&str> = Vec::new();
    for case in array(&source["numerical_quality"]["cases"])? {
        let basis = &case["basis_ref"];
        require(
            keys(basis, &["ref_type", "ref_id"]) && basis["ref_type"] == "load_case",
            "NUMERICAL_CASE_BASIS",
        )?;
        let id = text(&basis["ref_id"])?;
        require(!quality_ids.contains(&id), "NUMERICAL_CASE_DUPLICATE")?;
        quality_ids.push(id);
    }
    // S7 exact cases, their resolved materials and section identities.
    let mut cases: Vec<(&str, &Value)> = Vec::new();
    for case in exact {
        require(keys(case, CASE_KEYS), "CASE_SHAPE")?;
        let id = text(&case["load_case_id"])?;
        require(cases.iter().all(|(c, _)| *c != id), "CASE_DUPLICATE")?;
        require(case["material_basis"] == MATERIAL_BASIS, "MATERIAL_BASIS")?;
        let mut pipes = Vec::new();
        for material in array(&case["pipe_materials"])? {
            require(keys(material, MATERIAL_KEYS), "MATERIAL_SHAPE")?;
            let pipe = text(&material["pipe_id"])?;
            require(!pipes.contains(&pipe), "MATERIAL_DUPLICATE")?;
            pipes.push(pipe);
        }
        let mut sections = Vec::new();
        for section in array(&case["pipe_sections"])? {
            let pipe = text(&section["pipe_id"])?;
            require(!sections.contains(&pipe), "SECTION_DUPLICATE")?;
            sections.push(pipe);
        }
        cases.push((id, case));
    }
    // S8 record identities.
    let mut record_ids: Vec<&str> = Vec::new();
    for record in records {
        require(keys(record, RECORD_KEYS), "RECORD_SHAPE")?;
        let id = text(&record["load_case_id"])?;
        require(!record_ids.contains(&id), "RECORD_DUPLICATE")?;
        record_ids.push(id);
    }
    // S9 case trijection. An unsolved envelope retains the empty namespace.
    let solved = !raw || source["status"]["mechanics"] == "MECHANICS_SOLVED";
    if solved {
        let quality: HashSet<&str> = quality_ids.iter().copied().collect();
        require(
            !record_ids.is_empty()
                && record_ids.iter().copied().collect::<HashSet<_>>() == quality
                && cases.iter().map(|(c, _)| *c).collect::<HashSet<_>>() == quality,
            "CASE_COVERAGE",
        )?;
    } else {
        require(
            records.is_empty() && exact.is_empty() && pressure.is_empty(),
            "UNSOLVED_EVIDENCE",
        )?;
    }
    // S10 each record in document order.
    for record in records {
        validate_record(record, &cases, pressure)?;
    }
    // S11 one model geometry and one requested mode per envelope.
    if let Some(first) = records.first() {
        for record in records {
            require(
                record["reference_geometry"]["projection_sha256"]
                    == first["reference_geometry"]["projection_sha256"],
                "REFERENCE_GEOMETRY_CONSISTENCY",
            )?;
            require(
                record["solve"]["requested_mode"] == first["solve"]["requested_mode"],
                "SOLVE_CONSISTENCY",
            )?;
        }
    }
    // S12 pressure-region materials are the resolved member pair of their case.
    for region in pressure {
        require(keys(region, REGION_KEYS), "REGION_SHAPE")?;
        let case_id = text(&region["load_case_id"])?;
        text(&region["region_id"])?;
        let case = cases
            .iter()
            .find(|(c, _)| *c == case_id)
            .map(|(_, v)| *v)
            .ok_or_else(|| code("REGION_CASE_UNRESOLVED"))?;
        for material in array(&region["materials"])? {
            require(
                keys(material, REGION_MATERIAL_KEYS),
                "REGION_MATERIAL_SHAPE",
            )?;
            require(
                material["temperature_basis"] == REGION_TEMPERATURE_BASIS,
                "REGION_TEMPERATURE_BASIS",
            )?;
            let pipe = text(&material["pipe_id"])?;
            let member = array(&case["pipe_materials"])?
                .iter()
                .find(|m| m["pipe_id"] == pipe)
                .ok_or_else(|| code("REGION_MATERIAL_BINDING"))?;
            require(
                same(
                    &without(material, &["temperature_basis"]),
                    &without(member, &["material_selection_kind", "resolved_eigenstrain"]),
                ),
                "REGION_MATERIAL_BINDING",
            )?;
        }
    }
    // S13 exactly one not-joined info diagnostic per resolved case.
    if raw && solved {
        let diagnostics = array(&source["diagnostics"])?;
        for case_id in &record_ids {
            let expected = format!(
                "diagnostic:load-state:{}:source-recovery-not-joined",
                case_id.replace(':', "-")
            );
            let hits: Vec<&Value> = diagnostics
                .iter()
                .filter(|d| d["code"] == NOT_JOINED && d["id"] == expected.as_str())
                .collect();
            require(
                hits.len() == 1
                    && hits[0]["severity"] == "info"
                    && hits[0]["affected_refs"] == json!([case_id]),
                "NOT_JOINED_DIAGNOSTIC",
            )?;
        }
        require(
            diagnostics
                .iter()
                .filter(|d| d["code"] == NOT_JOINED)
                .count()
                == record_ids.len(),
            "NOT_JOINED_DIAGNOSTIC",
        )?;
    }
    // S14 inherited physics-1 checks on the projected copy.
    let projected = project(source);
    let inherited = if raw {
        crate::physics_evidence::validate_physics_evidence(&projected)
    } else {
        crate::physics_evidence::validate_transport_metadata(&projected)
    };
    inherited.map_err(|e| format!("{}: {e}", code("PHYSICS_EVIDENCE")))
}

fn project(source: &Value) -> Value {
    let mut projected = source.clone();
    let evidence = &mut projected["contract_evidence"];
    if let Some(o) = evidence.as_object_mut() {
        o.remove("load_reference_states");
    }
    if let Some(cases) = evidence["exact_cases"].as_array_mut() {
        for case in cases {
            case["material_basis"] = json!("base_material_common_E_nu");
            if let Some(materials) = case["pipe_materials"].as_array_mut() {
                for material in materials {
                    if let Some(o) = material.as_object_mut() {
                        o.remove("material_selection_kind");
                        o.remove("resolved_eigenstrain");
                    }
                }
            }
        }
    }
    if let Some(regions) = evidence["pressure"].as_array_mut() {
        for region in regions {
            if let Some(materials) = region["materials"].as_array_mut() {
                for material in materials {
                    material["temperature_basis"] = json!({"selection": "base_material"});
                }
            }
        }
    }
    projected
}

fn validate_record(record: &Value, cases: &[(&str, &Value)], pressure: &[Value]) -> Check {
    let case_id = text(&record["load_case_id"])?;
    // R1-R7 record-level closed statements.
    require(record["contract"] == RECORD_CONTRACT, "RECORD_CONTRACT")?;
    require(
        record["profile"] == crate::semantic_contract::LOAD_REFERENCE_PROFILE,
        "RECORD_PROFILE",
    )?;
    let configuration = text(&record["reference_configuration_id"])?;
    text(&record["provenance"])?;
    let geometry = &record["reference_geometry"];
    require(
        keys(geometry, &["kind", "projection_sha256"]),
        "REFERENCE_GEOMETRY_SHAPE",
    )?;
    require(
        geometry["kind"] == "authored_model_geometry" && sha64(&geometry["projection_sha256"]),
        "REFERENCE_GEOMETRY",
    )?;
    let history = &record["history"];
    require(keys(history, &["kind"]), "HISTORY_SHAPE")?;
    require(history["kind"] == "independent_equilibrium", "HISTORY")?;
    let solve = &record["solve"];
    require(
        keys(
            solve,
            &["requested_mode", "recovery_method", "boundary", "eigenload"],
        ),
        "SOLVE_SHAPE",
    )?;
    require(
        matches!(
            (
                solve["requested_mode"].as_str(),
                solve["recovery_method"].as_str()
            ),
            (
                Some("sparse_interactive"),
                Some("ordinary_sparse_structural_v1")
            ) | (Some("dense_scrutiny"), Some("ordinary_dense_structural_v1"))
        ) && solve["boundary"] == BOUNDARY
            && solve["eigenload"] == EIGENLOAD,
        "SOLVE",
    )?;
    let recovery = &record["source_recovery"];
    require(keys(recovery, &["status", "code"]), "SOURCE_RECOVERY_SHAPE")?;
    require(
        recovery["status"] == "not_joined" && recovery["code"] == NOT_JOINED,
        "SOURCE_RECOVERY",
    )?;
    // R8 members.
    let members = array(&record["members"])?;
    let mut member_ids: Vec<&str> = Vec::new();
    for member in members {
        validate_member(member)?;
        let pipe = text(&member["pipe_id"])?;
        require(!member_ids.contains(&pipe), "MEMBER_DUPLICATE")?;
        member_ids.push(pipe);
    }
    // R9 prescribed support components.
    let components = array(&record["support_components"])?;
    let mut component_ids: Vec<(&str, &str)> = Vec::new();
    let mut support_nodes: HashMap<&str, &str> = HashMap::new();
    let mut node_bases: HashMap<&str, u64> = HashMap::new();
    for component in components {
        validate_support_component(component)?;
        let support = text(&component["support_id"])?;
        let node = text(&component["node_id"])?;
        let dof = text(&component["dof"])?;
        require(
            !component_ids.contains(&(support, dof)),
            "SUPPORT_COMPONENT_DUPLICATE",
        )?;
        component_ids.push((support, dof));
        let base = index(&component["global_dof"])? / 6;
        require(
            *support_nodes.entry(support).or_insert(node) == node
                && *node_bases.entry(node).or_insert(base) == base,
            "SUPPORT_COMPONENT_NODE",
        )?;
    }
    // R10 contributions.
    let contributions = array(&record["contributions"])?;
    let mut contribution_ids: Vec<&str> = Vec::new();
    for contribution in contributions {
        validate_contribution(contribution)?;
        let id = text(&contribution["source_id"])?;
        require(!contribution_ids.contains(&id), "CONTRIBUTION_DUPLICATE")?;
        contribution_ids.push(id);
    }
    // R11 excluded sources.
    let mut excluded_ids: Vec<&str> = Vec::new();
    for excluded in array(&record["excluded_sources"])? {
        require(keys(excluded, EXCLUDED_KEYS), "EXCLUDED_SHAPE")?;
        require(
            excluded["owner_kind"] == "stored_primitive"
                && excluded["classification"] == "excluded",
            "EXCLUDED_CLASSIFICATION",
        )?;
        let id = text(&excluded["source_id"])?;
        text(&excluded["category"])?;
        text(&excluded["reason"])?;
        require(
            !contribution_ids.contains(&id) && !excluded_ids.contains(&id),
            "EXCLUDED_OVERLAP",
        )?;
        excluded_ids.push(id);
    }
    // R12-R14 binding to the exact case of the same load case.
    let case = cases
        .iter()
        .find(|(c, _)| *c == case_id)
        .map(|(_, v)| *v)
        .ok_or_else(|| code("RECORD_CASE_UNRESOLVED"))?;
    let sections: HashSet<&str> = array(&case["pipe_sections"])?
        .iter()
        .map(|s| s["pipe_id"].as_str().unwrap_or(""))
        .collect();
    require(
        !member_ids.is_empty() && member_ids.iter().copied().collect::<HashSet<_>>() == sections,
        "MEMBER_COVERAGE",
    )?;
    let materials = array(&case["pipe_materials"])?;
    require(
        materials
            .iter()
            .map(|m| m["pipe_id"].as_str().unwrap_or(""))
            .collect::<HashSet<_>>()
            == member_ids.iter().copied().collect(),
        "MEMBER_MATERIAL_COVERAGE",
    )?;
    for member in members {
        let material = materials
            .iter()
            .find(|m| m["pipe_id"] == member["pipe_id"])
            .ok_or_else(|| code("MEMBER_MATERIAL_COVERAGE"))?;
        require(
            num_eq(&material["E_pa"], &member["selected_E_pa"])
                && num_eq(&material["nu"], &member["selected_nu"])
                && num_eq(&material["G_pa"], &member["derived_G_pa"])
                && material["material_id"] == member["material_id"]
                && material["material_selection_kind"] == member["material_selection_kind"]
                && num_eq(
                    &material["resolved_eigenstrain"],
                    &member["total_eigenstrain"],
                )
                && material["thermal_consumed"] == false
                && material["alpha_per_kelvin"].is_null(),
            "MEMBER_MATERIAL_BINDING",
        )?;
        text(&material["provenance"])?;
    }
    // R15 contribution ledgers are bijections with their owners.
    let of_kind = |kind: &str| -> HashSet<&str> {
        contributions
            .iter()
            .filter(|c| c["owner_kind"] == kind)
            .filter_map(|c| c["source_id"].as_str())
            .collect()
    };
    let find = |id: &str| contributions.iter().find(|c| c["source_id"] == id);
    let expected_members: HashSet<String> = member_ids
        .iter()
        .map(|p| format!("member_state:{p}"))
        .collect();
    require(
        of_kind("resolved_member_state")
            == expected_members
                .iter()
                .map(String::as_str)
                .collect::<HashSet<_>>(),
        "MEMBER_CONTRIBUTION_COVERAGE",
    )?;
    for member in members {
        let pipe = text(&member["pipe_id"])?;
        let c = find(&format!("member_state:{pipe}"))
            .ok_or_else(|| code("MEMBER_CONTRIBUTION_COVERAGE"))?;
        require(
            num_eq(&c["value"], &member["total_eigenstrain"])
                && c["consumed_input_refs"]
                    == json!([
                        format!("{configuration}:{pipe}"),
                        format!("{case_id}:element_state:{pipe}")
                    ]),
            "MEMBER_CONTRIBUTION_BINDING",
        )?;
    }
    let expected_supports: HashSet<String> = component_ids
        .iter()
        .map(|(s, d)| format!("support_state:{s}:{d}"))
        .collect();
    require(
        of_kind("support_state")
            == expected_supports
                .iter()
                .map(String::as_str)
                .collect::<HashSet<_>>(),
        "SUPPORT_CONTRIBUTION_COVERAGE",
    )?;
    for component in components {
        let id = format!(
            "support_state:{}:{}",
            text(&component["support_id"])?,
            text(&component["dof"])?
        );
        let c = find(&id).ok_or_else(|| code("SUPPORT_CONTRIBUTION_COVERAGE"))?;
        require(
            num_eq(&c["value"], &component["prescribed_value"]),
            "SUPPORT_CONTRIBUTION_BINDING",
        )?;
    }
    let mut expected_regions: HashSet<String> = HashSet::new();
    for region in pressure {
        if region["load_case_id"] == case_id {
            expected_regions.insert(format!("pressure_region:{}", text(&region["region_id"])?));
        }
    }
    require(
        of_kind("pressure_region")
            == expected_regions
                .iter()
                .map(String::as_str)
                .collect::<HashSet<_>>(),
        "PRESSURE_CONTRIBUTION_COVERAGE",
    )?;
    Ok(())
}

fn validate_member(member: &Value) -> Check {
    // M1-M3 shape, identity and fixed statements.
    require(keys(member, MEMBER_KEYS), "MEMBER_SHAPE")?;
    text(&member["pipe_id"])?;
    text(&member["material_id"])?;
    require(
        member["G_basis"] == G_BASIS && member["eigenstrain_composition"] == COMPOSITION,
        "MEMBER_BASIS",
    )?;
    // M4 selected pair and derived G.
    let e = number(&member["selected_E_pa"])?;
    let nu = number(&member["selected_nu"])?;
    let g = number(&member["derived_G_pa"])?;
    require(
        e > 0.0 && nu > -1.0 && nu < 0.5 && g > 0.0,
        "MEMBER_MATERIAL_RANGE",
    )?;
    let direct = e / (2.0 * (1.0 + nu));
    require(
        direct.is_finite() && ulps(g, direct) <= 2,
        "MEMBER_G_BINDING",
    )?;
    let retained = boolean(&member["retained_G_ignored"])?;
    // M6 consumed material points.
    let points = array(&member["consumed_material_points"])?;
    let mut consumed = Vec::new();
    for point in points {
        require(keys(point, POINT_KEYS), "MATERIAL_POINT_SHAPE")?;
        text(&point["point_id"])?;
        let temperature = opt_number(&point["temperature_k"])?;
        require(positive(temperature), "TEMPERATURE_RANGE")?;
        let pe = number(&point["E_pa"])?;
        let pnu = number(&point["nu"])?;
        require(pe > 0.0 && pnu > -1.0 && pnu < 0.5, "MATERIAL_POINT_RANGE")?;
        let point_retained = boolean(&point["retained_G_ignored"])?;
        consumed.push((temperature, pe, pnu, point_retained));
    }
    // M7-M9 optional selection data and temperatures.
    let fraction = opt_number(&member["interpolation_fraction"])?;
    let applicability = opt_text(&member["applicability_reference"])?;
    let basis_override = &member["analysis_basis_override"];
    if !basis_override.is_null() {
        require(
            keys(basis_override, &["reason", "provenance"]),
            "OVERRIDE_SHAPE",
        )?;
        text(&basis_override["reason"])?;
        text(&basis_override["provenance"])?;
    }
    let operating = opt_number(&member["operating_temperature_k"])?;
    let selection = opt_number(&member["material_selection_temperature_k"])?;
    let installation = opt_number(&member["installation_temperature_k"])?;
    let datum = opt_number(&member["coefficient_datum_k"])?;
    require(
        positive(operating) && positive(selection) && positive(installation) && positive(datum),
        "TEMPERATURE_RANGE",
    )?;
    // M10 material selection.
    let selected = match member["material_selection_kind"].as_str() {
        Some("explicit_base_properties") => {
            consumed.is_empty()
                && fraction.is_none()
                && applicability.is_some()
                && selection.is_none()
        }
        Some("exact_point") => {
            consumed.len() == 1
                && fraction.is_none()
                && applicability.is_none()
                && e == consumed[0].1
                && nu == consumed[0].2
                && selection == consumed[0].0
                && retained == consumed[0].3
        }
        Some("temperature_interpolation") => {
            applicability.is_none()
                && selection.is_some()
                && retained == consumed.iter().any(|p| p.3)
                && match consumed.len() {
                    1 => {
                        fraction.is_none()
                            && e == consumed[0].1
                            && nu == consumed[0].2
                            && selection == consumed[0].0
                    }
                    2 => {
                        fraction.is_some_and(|f| f > 0.0 && f < 1.0)
                            && consumed.iter().all(|p| p.0.is_some())
                    }
                    _ => false,
                }
        }
        _ => return Err(code("MATERIAL_SELECTION_KIND")),
    };
    require(selected, "MATERIAL_SELECTION")?;
    // M11 reference basis.
    require(
        match member["reference_basis"].as_str() {
            Some("temperature_reference") => installation.is_some(),
            Some("direct_strain_reference") => installation.is_none(),
            _ => false,
        },
        "REFERENCE_BASIS",
    )?;
    // M12 strains and stretches.
    let thermal_strain = number(&member["thermal_strain"])?;
    let thermal_stretch = number(&member["thermal_stretch"])?;
    let fit_strain = number(&member["fit_strain"])?;
    let fit_stretch = number(&member["fit_stretch"])?;
    let total = number(&member["total_eigenstrain"])?;
    require(
        thermal_stretch > 0.0
            && fit_stretch > 0.0
            && thermal_stretch == 1.0 + thermal_strain
            && fit_stretch == 1.0 + fit_strain
            && 1.0 + total > 0.0,
        "STRETCH_BINDING",
    )?;
    // M13 consumed and consulted law data.
    let mut law_data_empty = true;
    for key in ["consumed_law_point_indices", "consulted_law_point_indices"] {
        let mut previous: Option<u64> = None;
        for item in array(&member[key])? {
            let i = index(item)?;
            require(previous.is_none_or(|p| p < i), "LAW_INDICES")?;
            previous = Some(i);
            law_data_empty = false;
        }
    }
    for key in ["consumed_law_segments", "consulted_law_segments"] {
        for segment in array(&member[key])? {
            require(keys(segment, SEGMENT_KEYS), "LAW_SEGMENT_SHAPE")?;
            let sample = match segment["use"].as_str() {
                Some("interpolation_sample") => true,
                Some("integration_interval") => false,
                _ => return Err(code("LAW_SEGMENT")),
            };
            let lower = index(&segment["lower_index"])?;
            let upper = index(&segment["upper_index"])?;
            let start = number(&segment["start_k"])?;
            let end = number(&segment["end_k"])?;
            require(
                lower <= upper && start <= end && (!sample || start == end),
                "LAW_SEGMENT",
            )?;
            law_data_empty = false;
        }
    }
    let installation_stretch = opt_number(&member["installation_datum_stretch"])?;
    let operating_stretch = opt_number(&member["operating_datum_stretch"])?;
    let law_id = opt_text(&member["expansion_law_id"])?;
    // M14 thermal definition.
    let definition = member["thermal_definition"].as_str().unwrap_or("");
    if LAW_DEFINITIONS.contains(&definition) {
        require(
            law_id.is_some()
                && datum.is_some()
                && installation_stretch.is_some_and(|v| v > 0.0)
                && operating_stretch.is_some_and(|v| v > 0.0)
                && installation.is_some()
                && operating.is_some(),
            "THERMAL_LAW_BINDING",
        )?;
    } else if DIRECT_DEFINITIONS.contains(&definition) {
        require(
            law_id.is_none()
                && datum.is_none()
                && installation_stretch.is_none()
                && operating_stretch.is_none()
                && law_data_empty
                && (definition != "unchanged_reference" || thermal_strain == 0.0),
            "THERMAL_LAW_BINDING",
        )?;
    } else {
        return Err(code("THERMAL_DEFINITION"));
    }
    // M15-M16 reference length and fit.
    let length = number(&member["reference_length_m"])?;
    require(length > 0.0, "REFERENCE_LENGTH")?;
    let fit = &member["fit_input"];
    let fitted = match member["fit_kind"].as_str() {
        Some("none") => fit.is_null() && fit_strain == 0.0,
        Some("natural_length_change") => {
            keys(fit, &["length_change_m"])
                && fit["length_change_m"].as_f64().is_some_and(|change| {
                    change.is_finite() && fit_strain == change / length && length + change > 0.0
                })
        }
        Some("fit_strain") => {
            keys(fit, &["strain"])
                && fit["strain"]
                    .as_f64()
                    .is_some_and(|strain| strain.is_finite() && fit_strain == strain)
        }
        _ => return Err(code("FIT_KIND")),
    };
    require(fitted, "FIT_BINDING")
}

fn validate_support_component(component: &Value) -> Check {
    require(keys(component, SUPPORT_KEYS), "SUPPORT_COMPONENT_SHAPE")?;
    text(&component["support_id"])?;
    text(&component["node_id"])?;
    let dof = DOFS
        .iter()
        .position(|d| component["dof"] == *d)
        .ok_or_else(|| code("SUPPORT_COMPONENT_DOF"))?;
    let global = index(&component["global_dof"])?;
    require(global % 6 == dof as u64, "SUPPORT_COMPONENT_DOF")?;
    require(
        component["law_kind"] == "rigid_prescribed"
            && component["meaning"] == "absolute_reference_displacement"
            && component["physical_state_source"] == "support_state.boundary_motion",
        "SUPPORT_COMPONENT_LAW",
    )?;
    require(
        component["unit"] == if dof < 3 { "m" } else { "rad" },
        "SUPPORT_COMPONENT_UNIT",
    )?;
    number(&component["prescribed_value"])?;
    Ok(())
}

fn validate_contribution(contribution: &Value) -> Check {
    let (shape, classification) = match contribution["owner_kind"].as_str() {
        Some("stored_primitive") => (STORED_KEYS, "ordinary_applied"),
        Some("resolved_member_state") => (MEMBER_STATE_KEYS, "eigenstrain"),
        Some("support_state") => (SUPPORT_STATE_KEYS, "prescribed_boundary"),
        Some("pressure_region") => (PRESSURE_REGION_KEYS, "pressure_eigen_and_closure"),
        _ => return Err(code("CONTRIBUTION_KIND")),
    };
    require(keys(contribution, shape), "CONTRIBUTION_SHAPE")?;
    require(
        contribution["classification"] == classification,
        "CONTRIBUTION_CLASSIFICATION",
    )?;
    text(&contribution["source_id"])?;
    match contribution["owner_kind"].as_str() {
        Some("stored_primitive") => {
            let factor = contribution["factor"]
                .as_f64()
                .filter(|f| f.is_finite() && *f != 0.0)
                .ok_or_else(|| code("CONTRIBUTION_FACTOR"))?;
            text(&contribution["category"])?;
            text(&contribution["dimension"])?;
            let authored = number(&contribution["authored_normalized_magnitude"])?;
            let applied = number(&contribution["applied_magnitude"])?;
            require(
                applied == authored * factor,
                "CONTRIBUTION_APPLIED_MAGNITUDE",
            )
        }
        Some("resolved_member_state") => {
            for reference in array(&contribution["consumed_input_refs"])? {
                text(reference)?;
            }
            number(&contribution["value"]).map(|_| ())
        }
        Some("support_state") => number(&contribution["value"]).map(|_| ()),
        _ => require(contribution["factor"].is_null(), "CONTRIBUTION_FACTOR"),
    }
}
