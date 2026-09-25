//! Closed source-block statement/binding checks. No imported receipt constructs
//! a live exact response, and a digest is never treated as arithmetic proof.
use crate::semantic_contract::{signature_in, source_blocks_contract};
use open_pipe_stress_canonical_json::canonical_json_checked_v1_text;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

pub const CONTRACT_ID: &str = "openpipestress.result_semantics/0.3.0/source-blocks-1";
pub const CONTRACT_SHA256: &str =
    "5f299065f15a157bbedf9467a598994ae684c4ecb3f851bbcb291981ec550a9f";
// The fixed 64ε recipe envelope is separate from affine input uncertainty.
// It covers divides, squares, ordered additions, sqrt, multiplication and tiny
// normalized underflows with a unit largest component and normal final output.
const NORM_ARITHMETIC_BOUND: f64 = 64.0 * f64::EPSILON;
const NORM_INPUT_RELATIVE_LIMIT: f64 =
    (1.0e-9 - NORM_ARITHMETIC_BOUND) / (1.0 + NORM_ARITHMETIC_BOUND);
// Additional conservative current-carrier reserve for stress and summary.
const STRESS_ARITHMETIC_BOUND: f64 = 128.0 * f64::EPSILON;
const STRESS_INPUT_RELATIVE_LIMIT: f64 =
    (1.0e-9 - STRESS_ARITHMETIC_BOUND) / (1.0 + STRESS_ARITHMETIC_BOUND);
const STRESS_LOCATIONS: [&str; 5] = ["end_i", "quarter_1", "midspan", "quarter_3", "end_j"];
const STRESS_KINDS: [&str; 4] = [
    "element_local_axial_normal_stress",
    "element_local_bending_normal_stress_y",
    "element_local_bending_normal_stress_z",
    "element_local_torsional_shear_stress",
];
const EXACT: &str = "retained_source_blocks_exact_v1";
const COMPONENTS: [&str; 6] = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"];
const SUPPORT_SIGN: &str =
    "support_on_pipe_positive_global_force_right_hand_couple_at_attachment_node";
macro_rules! require {
    ($condition:expr, $code:literal) => {
        if !$condition {
            return Err(concat!("SOURCE_BLOCKS_", $code).into());
        }
    };
}
fn text(v: &Value) -> Result<&str, String> {
    v.as_str()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "SOURCE_BLOCKS_STRING".into())
}
fn array(v: &Value) -> Result<&Vec<Value>, String> {
    v.as_array().ok_or_else(|| "SOURCE_BLOCKS_ARRAY".into())
}
fn number(v: &Value) -> Result<f64, String> {
    v.as_f64()
        .filter(|x| x.is_finite())
        .ok_or_else(|| "SOURCE_BLOCKS_NUMBER".into())
}
fn integer(v: &Value) -> Result<usize, String> {
    v.as_u64()
        .and_then(|n| usize::try_from(n).ok())
        .filter(|n| *n <= 9_007_199_254_740_991)
        .ok_or_else(|| "SOURCE_BLOCKS_INTEGER".into())
}
fn keys(v: &Value, expected: &[&str]) -> bool {
    v.as_object()
        .is_some_and(|o| o.len() == expected.len() && expected.iter().all(|k| o.contains_key(*k)))
}
fn unique(items: &[Value]) -> bool {
    items
        .iter()
        .enumerate()
        .all(|(i, v)| !items[..i].contains(v))
}
fn schema() -> &'static Value {
    static SCHEMA: OnceLock<Value> = OnceLock::new();
    SCHEMA.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../../../../schemas/source_block_recovery.schema.json"
        ))
        .expect("closed receipt schema")
    })
}
fn shape(v: &Value, s: &Value) -> bool {
    if let Some(reference) = s["$ref"].as_str() {
        return schema()
            .pointer(reference.trim_start_matches('#'))
            .is_some_and(|target| shape(v, target));
    }
    if let Some(branches) = s["anyOf"].as_array() {
        if !branches.iter().any(|s| shape(v, s)) {
            return false;
        }
    }
    if let Some(constant) = s.get("const") {
        if v != constant {
            return false;
        }
    }
    if let Some(values) = s["enum"].as_array() {
        if !values.contains(v) {
            return false;
        }
    }
    match s["type"].as_str() {
        Some("null") => v.is_null(),
        Some("object") => {
            let Some(o) = v.as_object() else { return false };
            let Some(properties) = s["properties"].as_object() else {
                return false;
            };
            s["required"].as_array().is_none_or(|a| {
                a.iter()
                    .all(|k| k.as_str().is_some_and(|k| o.contains_key(k)))
            }) && (s["additionalProperties"] != false
                || o.keys().all(|k| properties.contains_key(k)))
                && o.iter()
                    .all(|(k, v)| properties.get(k).is_none_or(|s| shape(v, s)))
        }
        Some("array") => v.as_array().is_some_and(|a| {
            a.len() >= s["minItems"].as_u64().unwrap_or(0) as usize
                && a.len() <= s["maxItems"].as_u64().unwrap_or(16384) as usize
                && a.iter().all(|v| shape(v, &s["items"]))
        }),
        Some("string") => v.as_str().is_some_and(|t| {
            t.chars().count() >= s["minLength"].as_u64().unwrap_or(0) as usize
                && match s["pattern"].as_str() {
                    None => true,
                    Some("^[0-9a-f]{64}$") => hex(t, 64),
                    Some("^[0-9a-f]{16}$") => hex(t, 16),
                    _ => false,
                }
        }),
        Some("number" | "integer") => v.as_f64().is_some_and(|n| {
            n.is_finite()
                && (s["type"] != "integer" || n.fract() == 0.0)
                && n >= s["minimum"].as_f64().unwrap_or(f64::NEG_INFINITY)
                && n <= s["maximum"].as_f64().unwrap_or(f64::INFINITY)
        }),
        None => true,
        _ => false,
    }
}
fn hex(t: &str, n: usize) -> bool {
    t.len() == n
        && t.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
pub fn domain_hash(domain: &str, payload: &Value) -> Result<String, String> {
    let value = json!({"domain":domain,"payload":payload});
    let checked =
        canonical_json_checked_v1_text(&serde_json::to_string(&value).map_err(|e| e.to_string())?)?;
    Ok(format!("{:x}", Sha256::digest(checked.as_bytes())))
}
pub fn validate_receipt_shape(receipt: &Value) -> Result<(), String> {
    require!(shape(receipt, schema()), "RECEIPT_SHAPE");
    Ok(())
}
fn signature(row: &Value) -> Result<Option<&'static Value>, String> {
    let s = signature_in(source_blocks_contract(), row)?;
    Ok(s.filter(|s| s["component"].is_null() || s["component"] == row["metadata"]["component"]))
}
fn ordinary_method(mode: &Value) -> Option<&'static str> {
    match mode.as_str() {
        Some("dense_scrutiny") => Some("ordinary_dense_structural_v1"),
        Some("sparse_interactive") => Some("ordinary_sparse_structural_v1"),
        _ => None,
    }
}
fn rank(v: &Value) -> Option<u8> {
    match v.as_str() {
        Some("checks_passed") => Some(0),
        Some("sensitive") => Some(1),
        Some("not_assessed") => Some(2),
        Some("unresolved") => Some(3),
        Some("failed") => Some(4),
        _ => None,
    }
}
fn ordinary(
    case: &Value,
    q: &Value,
    diagnostics: &HashMap<&str, &Value>,
    evidence: &HashSet<&str>,
) -> Result<bool, String> {
    let o = &case["ordinary_attempt"];
    let report = &o["structural_report_diagnostic_ref"];
    let failure = &o["failure"];
    require!(
        o["requested_mode"] == case["requested_mode"],
        "ORDINARY_MODE"
    );
    require!(
        report.is_null() || diagnostics.contains_key(text(report)?),
        "ORDINARY_REPORT_REF"
    );
    require!(
        failure.is_null() || diagnostics.contains_key(text(&failure["diagnostic_ref"])?),
        "ORDINARY_FAILURE_REF"
    );
    let refs = array(&q["evidence_refs"])?;
    match text(&o["outcome"])? {
        "not_attempted" => require!(
            report.is_null() && failure.is_null() && q["solve_quality"] == "not_assessed",
            "ORDINARY_NOT_ATTEMPTED"
        ),
        "checks_passed" | "sensitive" => require!(
            !report.is_null()
                && failure.is_null()
                && q["solve_quality"] == o["outcome"]
                && refs.contains(report),
            "ORDINARY_REPORT"
        ),
        "rejected" => require!(
            !failure.is_null()
                && matches!(q["solve_quality"].as_str(), Some("unresolved" | "failed"))
                && refs.contains(&failure["diagnostic_ref"]),
            "ORDINARY_REJECTION"
        ),
        _ => return Err("SOURCE_BLOCKS_ORDINARY_OUTCOME".into()),
    }
    if matches!(o["outcome"].as_str(), Some("checks_passed" | "sensitive")) {
        let expected = if o["outcome"] == "checks_passed" {
            "NUMERICAL_INTEGRITY_CHECKS_PASSED"
        } else {
            "NUMERICAL_INTEGRITY_SENSITIVE"
        };
        require!(
            diagnostics[text(report)?]["code"] == expected,
            "ORDINARY_REPORT_KIND"
        );
    }
    Ok(q["solve_quality"] == "checks_passed"
        && q["structural_status"] == "passive_model_basis"
        && q["model_matrix_fidelity"] == "represented_equations_retained"
        && matches!(
            q["accuracy_evidence"].as_str(),
            Some("not_claimed" | "reference_verified")
        )
        && !refs.is_empty()
        && refs
            .iter()
            .all(|r| r.as_str().is_some_and(|s| evidence.contains(s))))
}
fn source_plan(p: &Value, projections: &[Value], model: Option<&Value>) -> Result<(), String> {
    let n = integer(&p["dof_count"])?;
    let stiffness = integer(&p["stiffness_term_count"])?;
    let force = integer(&p["force_term_count"])?;
    require!(
        n > 0 && n <= 256 && stiffness > 0 && stiffness.saturating_add(force) <= 16384,
        "SOURCE_COUNTS"
    );
    let count = integer(&p["functional_count"])?;
    require!(
        count >= projections.len() && count <= 16384,
        "FUNCTIONAL_COUNT"
    );
    let free = array(&p["free_dofs"])?;
    let prescribed = array(&p["prescribed_dofs"])?;
    let mut partition = free
        .iter()
        .chain(prescribed)
        .map(integer)
        .collect::<Result<Vec<_>, _>>()?;
    partition.sort_unstable();
    require!(partition == (0..n).collect::<Vec<_>>(), "SOURCE_PARTITION");
    let mut blocks = Vec::new();
    for block in array(&p["free_blocks"])? {
        blocks.extend(
            array(block)?
                .iter()
                .map(integer)
                .collect::<Result<Vec<_>, _>>()?,
        );
    }
    let mut fs = free.iter().map(integer).collect::<Result<Vec<_>, _>>()?;
    fs.sort_unstable();
    blocks.sort_unstable();
    require!(blocks == fs, "BLOCK_PARTITION");
    let members = array(&p["member_ids"])?;
    let supports = array(&p["support_ids"])?;
    require!(
        !members.is_empty() && unique(members) && unique(supports),
        "SOURCE_IDENTITIES"
    );
    if let Some(model) = model {
        require!(
            n == array(&model["nodes"])?.len() * 6
                && *members
                    == array(&model["pipe_segments"])?
                        .iter()
                        .map(|p| p["id"].clone())
                        .collect::<Vec<_>>()
                && *supports
                    == array(&model["supports"])?
                        .iter()
                        .map(|s| s["id"].clone())
                        .collect::<Vec<_>>(),
            "CURRENT_MODEL_SOURCE_COVERAGE"
        );
    }
    Ok(())
}
fn projection(p: &Value, row: &Value) -> Result<(), String> {
    let value = number(&p["value"])?;
    let lo = number(&p["interval"][0])?;
    let hi = number(&p["interval"][1])?;
    require!(
        format!("{:016x}", value.to_bits()) == text(&p["value_bits"])?
            && value.to_bits() == number(&row["value"])?.to_bits()
            && p["unit"] == row["unit"],
        "PROJECTION_VALUE_BINDING"
    );
    require!(
        lo <= value
            && value <= hi
            && (value == 0.0 || (lo > 0.0 && hi > 0.0) || (lo < 0.0 && hi < 0.0)),
        "PROJECTION_INTERVAL"
    );
    let absolute = number(&p["absolute_error_bound"])?;
    let relative = number(&p["relative_error_bound"])?;
    match text(&p["basis"])? {
        "exact_zero" => require!(
            value == 0.0 && lo == value && hi == value && absolute == 0.0 && relative == 0.0,
            "PROJECTION_EXACT_BASIS"
        ),
        "exact_identity" => require!(
            value != 0.0 && lo == value && hi == value && absolute == 0.0 && relative == 0.0,
            "PROJECTION_EXACT_BASIS"
        ),
        "outward_interval" => require!(
            value != 0.0
                && absolute >= (value - lo).abs().max((hi - value).abs())
                && relative >= absolute / lo.abs().min(hi.abs()),
            "PROJECTION_ENCLOSURE"
        ),
        _ => return Err("SOURCE_BLOCKS_PROJECTION_BASIS".into()),
    }
    require!(
        number(&p["relative_limit"])? == 1e-9 && relative <= 1e-9,
        "PROJECTION_CRITERION"
    );
    let kind = text(&row["kind"])?;
    let md = &row["metadata"];
    let valid = match text(&p["quantity"])? {
        "nodal_translation" => {
            matches!(
                kind,
                "global_nodal_displacement_x"
                    | "global_nodal_displacement_y"
                    | "global_nodal_displacement_z"
            ) && row["unit"] == "mm"
                && md["coordinate_system"] == "global"
                && md["location"] == "node"
        }
        "nodal_rotation" => {
            matches!(
                kind,
                "global_nodal_rotation_x" | "global_nodal_rotation_y" | "global_nodal_rotation_z"
            ) && row["unit"] == "rad"
                && md["coordinate_system"] == "global"
                && md["location"] == "node"
        }
        "member_end_action" => {
            kind.starts_with("element_local_")
                && matches!(row["unit"].as_str(), Some("N" | "N*m"))
                && matches!(md["location"].as_str(), Some("end_i" | "end_j"))
                && md["coordinate_system"] == "element_local"
        }
        "member_station_action" => {
            kind.starts_with("element_local_")
                && matches!(row["unit"].as_str(), Some("N" | "N*m"))
                && matches!(
                    md["location"].as_str(),
                    Some("quarter_1" | "midspan" | "quarter_3")
                )
                && md["coordinate_system"] == "element_local"
        }
        "support_action_component" => {
            kind == "support_reaction_component_v2"
                && md["component"]
                    .as_str()
                    .is_some_and(|s| COMPONENTS.contains(&s))
        }
        _ => false,
    };
    require!(
        valid && signature(row)?.is_some(),
        "PROJECTION_TYPED_QUANTITY"
    );
    Ok(())
}
fn derived(recipe: &Value, row: &Value, inputs: &[&Value]) -> Result<(), String> {
    let kind = text(&row["kind"])?;
    let s = signature(row)?.ok_or("SOURCE_BLOCKS_DERIVED_SIGNATURE")?;
    let kinds: Vec<_> = inputs.iter().filter_map(|r| r["kind"].as_str()).collect();
    match text(recipe)? {
        "translation_norm_scaled_v1" => require!(
            kind == "displacement_magnitude"
                && inputs.len() == 3
                && kinds
                    == [
                        "global_nodal_displacement_x",
                        "global_nodal_displacement_y",
                        "global_nodal_displacement_z"
                    ],
            "TRANSLATION_NORM_INPUTS"
        ),
        "support_force_norm_scaled_v1" => require!(
            kind == "reaction_resultant"
                && inputs.len() == 3
                && inputs
                    .iter()
                    .all(|r| r["kind"] == "support_reaction_component_v2")
                && inputs
                    .iter()
                    .filter_map(|r| r["metadata"]["component"].as_str())
                    .collect::<Vec<_>>()
                    == ["Fx", "Fy", "Fz"],
            "SUPPORT_NORM_INPUTS"
        ),
        "straight_open_stress_v1" => require!(
            inputs.len() == 1
                && stress_action_kind(kind).is_some_and(|expected| inputs[0]["kind"] == expected),
            "STRESS_RECIPE_INPUTS"
        ),
        "reviewed_stress_summary_v1" => require!(
            kind == "open_formula_stress_summary" && !inputs.is_empty(),
            "SUMMARY_RECIPE_INPUTS"
        ),
        "section_property_from_source_v1" => require!(
            s["family"] == "section_property" || kind.starts_with("pipe_section_"),
            "SECTION_RECIPE_KIND"
        ),
        _ => return Err("SOURCE_BLOCKS_DERIVED_RECIPE".into()),
    }
    require!(
        inputs.iter().all(|r| r["entity_ref"] == row["entity_ref"]),
        "DERIVED_ENTITY"
    );
    match recipe.as_str() {
        Some("straight_open_stress_v1") => stress_observation(row, inputs[0])?,
        Some("reviewed_stress_summary_v1") => stress_summary_observation(row, inputs)?,
        _ => (),
    }
    if matches!(
        recipe.as_str(),
        Some("translation_norm_scaled_v1" | "support_force_norm_scaled_v1")
    ) {
        // Match the declared producer recipe in XYZ/FxFyFz order. Scaling
        // keeps the largest normalized component at one, so underflow of a
        // smaller normalized square cannot erase the norm's leading scale.
        let mut values = [0.0; 3];
        for (index, input) in inputs.iter().enumerate() {
            values[index] = number(&input["value"])?;
        }
        let scale = values.iter().fold(0.0_f64, |m, value| m.max(value.abs()));
        let expected = if scale == 0.0 {
            0.0
        } else {
            let normalized = values.map(|value| value / scale);
            let squares = normalized.map(|value| value * value);
            let first_two = squares[0] + squares[1];
            let squared = first_two + squares[2];
            let root = squared.sqrt();
            let value = scale * root;
            require!(
                normalized
                    .iter()
                    .chain(squares.iter())
                    .all(|v| v.is_finite())
                    && first_two.is_finite()
                    && squared.is_finite()
                    && root.is_finite()
                    && value.is_normal(),
                "DERIVED_NORM_RANGE"
            );
            value
        };
        require!(
            number(&row["value"])?.to_bits() == expected.to_bits(),
            "DERIVED_NORM_VALUE"
        );
    }
    Ok(())
}

fn stress_action_kind(kind: &str) -> Option<&'static str> {
    match kind {
        "element_local_axial_normal_stress" => Some("element_local_axial_force"),
        "element_local_bending_normal_stress_y" => Some("element_local_bending_moment_y"),
        "element_local_bending_normal_stress_z" => Some("element_local_bending_moment_z"),
        "element_local_torsional_shear_stress" => Some("element_local_torsional_moment"),
        _ => None,
    }
}
fn normal_or_exact_zero(value: f64, logical_zero: bool) -> bool {
    value.is_finite()
        && if logical_zero {
            value == 0.0
        } else {
            value.is_normal()
        }
}
fn stress_observation(row: &Value, action: &Value) -> Result<(), String> {
    // Area/Z/radius/J and actual Pa arithmetic remain in producer custody.
    let location = text(&row["metadata"]["location"])?;
    let expected_unit = if row["kind"] == "element_local_axial_normal_stress" {
        "N"
    } else {
        "N*m"
    };
    require!(
        row["unit"] == "MPa"
            && STRESS_LOCATIONS.contains(&location)
            && action["metadata"]["location"] == location
            && action["unit"] == expected_unit,
        "STRESS_ACTION_BINDING"
    );
    let source_action = number(&action["value"])?;
    let value = number(&row["value"])?;
    let logical_zero = source_action == 0.0;
    require!(
        normal_or_exact_zero(value, logical_zero),
        "STRESS_OUTPUT_RANGE"
    );
    let signed_action = if location == "end_i" {
        -source_action
    } else {
        source_action
    };
    require!(
        logical_zero || (value > 0.0) == (signed_action > 0.0),
        "STRESS_ACTION_SIGN"
    );
    // This is an observable round trip, not the private source quotient.
    require!(
        normal_or_exact_zero(value * 1_000_000.0, logical_zero),
        "STRESS_PA_OBSERVATION_RANGE"
    );
    Ok(())
}
fn stress_summary_observation(row: &Value, inputs: &[&Value]) -> Result<(), String> {
    require!(
        row["unit"] == "MPa" && inputs.len() == 20,
        "SUMMARY_STRESS_COVERAGE"
    );
    let mut grouped = HashMap::new();
    for input in inputs {
        let location = text(&input["metadata"]["location"])?;
        let kind = text(&input["kind"])?;
        require!(
            STRESS_LOCATIONS.contains(&location)
                && STRESS_KINDS.contains(&kind)
                && input["unit"] == "MPa"
                && !grouped.contains_key(&(location, kind)),
            "SUMMARY_STRESS_COVERAGE"
        );
        let value = number(&input["value"])?;
        require!(
            normal_or_exact_zero(value, value == 0.0),
            "SUMMARY_INPUT_RANGE"
        );
        let pa = value * 1_000_000.0;
        require!(
            normal_or_exact_zero(pa, value == 0.0),
            "SUMMARY_PA_OBSERVATION_RANGE"
        );
        grouped.insert((location, kind), pa);
    }
    let mut any_normal = false;
    for location in STRESS_LOCATIONS {
        let axial = grouped[&(location, STRESS_KINDS[0])];
        let by = grouped[&(location, STRESS_KINDS[1])];
        let bz = grouped[&(location, STRESS_KINDS[2])];
        let logical_zero = axial == 0.0 && by == 0.0 && bz == 0.0;
        any_normal |= !logical_zero;
        // Preserve the declared order on these observable Pa values. They do
        // not reproduce the unavailable preconversion Pa operands bitwise.
        let base_normal = axial + 0.0;
        let bending_total = by.abs() + bz.abs();
        let plus = base_normal + bending_total;
        let minus = base_normal - bending_total;
        require!(
            [base_normal, bending_total, plus, minus]
                .iter()
                .all(|v| v.is_finite()),
            "SUMMARY_SUBTOTAL_RANGE"
        );
        let subtotal = plus.abs().max(minus.abs());
        require!(
            normal_or_exact_zero(subtotal, logical_zero)
                && normal_or_exact_zero(subtotal / 1_000_000.0, logical_zero),
            "SUMMARY_SUBTOTAL_RANGE"
        );
    }
    // Torsion does not contribute to this normal-stress summary.
    let value = number(&row["value"])?;
    require!(
        value >= 0.0 && normal_or_exact_zero(value, !any_normal),
        "SUMMARY_OUTPUT_RANGE"
    );
    Ok(())
}

/// `actual_invocation` is the independent original {request,solver_mode} Value.
/// Without it, a supported consistent receipt returns false, never admission.
pub fn validate(source: &Value, actual_invocation: Option<&Value>) -> Result<bool, String> {
    require!(
        keys(
            source,
            &[
                "schema_version",
                "producer",
                "numerical_quality",
                "formulation_basis",
                "document_kind",
                "run_id",
                "model_ref",
                "status",
                "summary",
                "results",
                "diagnostics",
                "professional_boundary",
                "accepted_model_state_mutated",
                "source_block_recovery"
            ]
        ),
        "RAW_FIELDS"
    );
    require!(
        source["schema_version"] == "0.2.0"
            && source["producer"]
                == json!({"component_name":"open_pipe_stress_product_physics","component_version":"0.2.0","semantic_contract_id":CONTRACT_ID}),
        "PRODUCER"
    );
    let formulation = &source["formulation_basis"];
    require!(
        keys(formulation, &["profile_id", "limitations"])
            && formulation["profile_id"] == "product_preview_mechanics_v1"
            && formulation["limitations"]
                .as_array()
                .is_some_and(|a| !a.is_empty()
                    && a.iter().all(|v| v.as_str().is_some_and(|s| !s.is_empty()))),
        "FORMULATION"
    );
    let receipt = &source["source_block_recovery"];
    validate_receipt_shape(receipt)?;
    let body = &receipt["body"];
    require!(
        receipt["receipt_sha256"] == domain_hash("source_blocks_receipt_v1", body)?,
        "RECEIPT_HASH"
    );
    let mut publication = source.clone();
    publication
        .as_object_mut()
        .unwrap()
        .remove("source_block_recovery");
    require!(
        body["publication_sha256"] == domain_hash("source_blocks_publication_v1", &publication)?,
        "PUBLICATION_HASH"
    );
    let model = if let Some(invocation) = actual_invocation {
        require!(
            keys(invocation, &["request", "solver_mode"])
                && invocation["request"].is_object()
                && ordinary_method(&invocation["solver_mode"]).is_some(),
            "ACTUAL_INVOCATION_SHAPE"
        );
        require!(
            body["invocation"]["value"] == domain_hash("source_blocks_invocation_v1", invocation)?,
            "INVOCATION_HASH"
        );
        let model = &invocation["request"]["model"];
        require!(
            model.is_object()
                && model["project"]["id"] == source["model_ref"]
                && ["nodes", "pipe_segments", "supports", "load_cases"]
                    .iter()
                    .all(|k| model[k].is_array()),
            "CURRENT_MODEL"
        );
        Some(model)
    } else {
        None
    };
    require!(
        array(&source["results"])?.len() <= 16384 && array(&source["diagnostics"])?.len() <= 16384,
        "ROWS"
    );
    if let Some(model) = model {
        for field in ["nodes", "pipe_segments", "supports", "load_cases"] {
            let ids = array(&model[field])?
                .iter()
                .map(|item| {
                    text(&item["id"])?;
                    Ok(item["id"].clone())
                })
                .collect::<Result<Vec<_>, String>>()?;
            require!(unique(&ids), "CURRENT_MODEL_IDS");
        }
    }
    let mut evidence = HashSet::new();
    let mut raw = HashMap::new();
    let mut diagnostics = HashMap::new();
    for row in array(&source["results"])? {
        let id = text(&row["id"])?;
        require!(evidence.insert(id), "EVIDENCE_IDS");
        raw.insert(id, row);
        number(&row["value"])?;
        for k in ["kind", "unit", "entity_ref"] {
            text(&row[k])?;
        }
        if !row["metadata"].is_null() {
            require!(
                keys(
                    &row["metadata"],
                    &[
                        "component",
                        "coordinate_system",
                        "location",
                        "basis",
                        "sign_convention"
                    ]
                ) && row["metadata"]
                    .as_object()
                    .unwrap()
                    .values()
                    .all(|v| v.as_str().is_some_and(|t| !t.is_empty())),
                "ROW_METADATA"
            );
        }
    }
    for diagnostic in array(&source["diagnostics"])? {
        let id = text(&diagnostic["id"])?;
        require!(evidence.insert(id), "EVIDENCE_IDS");
        diagnostics.insert(id, diagnostic);
    }
    let cases = array(&body["cases"])?;
    let case_ids = cases
        .iter()
        .map(|c| c["basis_ref"]["ref_id"].clone())
        .collect::<Vec<_>>();
    require!(unique(&case_ids), "CASE_IDS");
    let q = &source["numerical_quality"];
    require!(
        keys(
            q,
            &[
                "value_representation",
                "publication_quantization",
                "integrity_policy",
                "status",
                "cases"
            ]
        ) && q["value_representation"] == "finite_binary64"
            && q["publication_quantization"] == "none"
            && q["integrity_policy"] == "M03-INTEGRITY-v1",
        "ORDINARY_QUALITY"
    );
    let qualities = array(&q["cases"])?;
    require!(qualities.len() == cases.len(), "ORDINARY_CASE_COUNT");
    for quality in qualities {
        require!(
            keys(
                quality,
                &[
                    "basis_ref",
                    "structural_status",
                    "solve_quality",
                    "model_matrix_fidelity",
                    "accuracy_evidence",
                    "evidence_refs"
                ]
            ) && rank(&quality["solve_quality"]).is_some()
                && matches!(
                    quality["structural_status"].as_str(),
                    Some(
                        "passive_model_basis"
                            | "physical_mechanism_witnessed"
                            | "negative_energy_witnessed"
                            | "numerically_unresolved"
                    )
                )
                && matches!(
                    quality["model_matrix_fidelity"].as_str(),
                    Some(
                        "represented_equations_retained"
                            | "assembly_loss_detected"
                            | "assembly_uncertainty"
                            | "not_assessed"
                    )
                )
                && matches!(
                    quality["accuracy_evidence"].as_str(),
                    Some("not_claimed" | "reference_verified" | "unresolved")
                )
                && array(&quality["evidence_refs"])?
                    .iter()
                    .all(|r| r.as_str().is_some_and(|id| evidence.contains(id))),
            "ORDINARY_CASE"
        );
    }
    let worst = qualities
        .iter()
        .max_by_key(|q| rank(&q["solve_quality"]))
        .map(|q| &q["solve_quality"]);
    require!(
        worst.map_or(q["status"] == "not_assessed", |v| q["status"] == *v),
        "ORDINARY_AGGREGATE"
    );
    if let Some(model) = model {
        require!(
            case_ids
                == array(&model["load_cases"])?
                    .iter()
                    .map(|c| c["id"].clone())
                    .collect::<Vec<_>>(),
            "REQUESTED_CASE_COVERAGE"
        );
    }
    let mut invocation_charged = 0usize;
    let mut covered = HashSet::new();
    let mut qualified = 0usize;
    let mut usable = !cases.is_empty();
    for (index, case) in cases.iter().enumerate() {
        require!(
            integer(&case["ordinary_attempt"]["quality_case_index"])? == index
                && qualities[index]["basis_ref"] == case["basis_ref"],
            "QUALITY_CASE_BINDING"
        );
        require!(
            actual_invocation.is_none_or(|i| case["requested_mode"] == i["solver_mode"]),
            "REQUESTED_MODE"
        );
        let ordinary_ok = ordinary(case, &qualities[index], &diagnostics, &evidence)?;
        let work = &case["work"];
        let rejected = &work["rejected_reservation"];
        invocation_charged = invocation_charged
            .saturating_add(integer(&work["charged"])?)
            .saturating_add(integer(&work["reserved_unobserved_failure"])?);
        require!(invocation_charged <= 64_000_000, "INVOCATION_WORK_LIMIT");
        require!(
            integer(&work["charged"])?
                .saturating_add(integer(&work["reserved_unobserved_failure"])?)
                <= integer(&work["limit"])?
                && integer(&work["limit"])? <= 4_000_000
                && ((rejected["kind"] == "overflow" && rejected["amount"].is_null())
                    || (rejected["kind"] == "finite" && !rejected["amount"].is_null())),
            "WORK_LEDGER"
        );
        let success = case["outcome"] == "qualified";
        let method = case["selected_method"].as_str();
        if success {
            qualified += 1;
            require!(
                case["failure"].is_null()
                    && method.is_some()
                    && *rejected == json!({"kind":"finite","amount":0})
                    && integer(&work["reserved_unobserved_failure"])? == 0,
                "QUALIFIED_OUTCOME"
            );
            if method != Some(EXACT) {
                require!(
                    method == ordinary_method(&case["requested_mode"])
                        && ordinary_ok
                        && case["source"].is_null()
                        && array(&case["projections"])?.is_empty()
                        && array(&case["supports"])?.is_empty(),
                    "ORDINARY_SELECTION"
                );
            } else {
                require!(!case["source"].is_null(), "EXACT_SOURCE_REQUIRED");
            }
        } else {
            let failure = &case["failure"];
            require!(
                method.is_none()
                    && !failure.is_null()
                    && diagnostics.contains_key(text(&failure["diagnostic_ref"])?),
                "FAILED_OUTCOME"
            );
            require!(
                (failure["code"] == "unsupported_block"
                    && !failure["block_order"].is_null()
                    && integer(&failure["block_order"])? > 2
                    && case["source"].is_null())
                    || (failure["code"] != "unsupported_block" && failure["block_order"].is_null()),
                "FAILURE_BLOCK_ORDER"
            );
            require!(
                (case["outcome"] == "unsupported")
                    == matches!(
                        failure["code"].as_str(),
                        Some(
                            "unsupported_family"
                                | "unsupported_block"
                                | "unsupported_source_closure"
                                | "unsupported_derived_quantity"
                                | "support_attribution_ambiguous"
                        )
                    ),
                "FAILURE_CATEGORY"
            );
        }
        let ps = array(&case["projections"])?;
        if !case["source"].is_null() {
            source_plan(&case["source"], ps, model)?;
            require!(
                unique(&[
                    case["source"]["normalized_source_sha256"].clone(),
                    case["source"]["functional_plan_sha256"].clone(),
                    body["invocation"]["value"].clone(),
                    body["publication_sha256"].clone(),
                    receipt["receipt_sha256"].clone()
                ]),
                "COMMITMENT_DOMAIN_SEPARATION"
            );
        }
        for field in ["projection_id", "functional_id", "result_id"] {
            require!(
                unique(&ps.iter().map(|p| p[field].clone()).collect::<Vec<_>>()),
                "PROJECTION_IDENTITIES"
            );
        }
        let projections = ps
            .iter()
            .map(|p| Ok((text(&p["projection_id"])?, p)))
            .collect::<Result<HashMap<_, _>, String>>()?;
        let rows = array(&case["rows"])?;
        require!(
            unique(
                &rows
                    .iter()
                    .map(|r| r["result_id"].clone())
                    .collect::<Vec<_>>()
            ),
            "ROW_IDS"
        );
        if rows.iter().any(|row| {
            matches!(
                row["recipe_id"].as_str(),
                Some("straight_open_stress_v1" | "reviewed_stress_summary_v1")
            )
        }) {
            let relative = ps
                .iter()
                .map(|p| number(&p["relative_error_bound"]))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .fold(0.0_f64, f64::max);
            require!(
                !ps.is_empty() && relative <= STRESS_INPUT_RELATIVE_LIMIT,
                "STRESS_TOTAL_RELATIVE_BOUND"
            );
        }
        let treatments = rows
            .iter()
            .map(|r| Ok((text(&r["result_id"])?, r)))
            .collect::<Result<HashMap<_, _>, String>>()?;
        let expected: HashSet<_> = raw
            .iter()
            .filter_map(|(id, r)| (r["basis_ref"] == case["basis_ref"]).then_some(*id))
            .collect();
        require!(
            treatments.keys().copied().collect::<HashSet<_>>() == expected
                && covered.is_disjoint(&expected),
            "CASE_ROW_COVERAGE"
        );
        covered.extend(expected);
        let mut used = HashSet::new();
        for (&id, treatment) in &treatments {
            let row = raw[id];
            let s = signature(row)?;
            let inputs = array(&treatment["input_result_ids"])?;
            require!(
                unique(inputs)
                    && inputs.iter().all(|i| i
                        .as_str()
                        .is_some_and(|i| i != id && treatments.contains_key(i))),
                "SAME_CASE_INPUTS"
            );
            match text(&treatment["treatment"])? {
                "qualified_projection" => {
                    let p = *projections
                        .get(text(&treatment["projection_id"])?)
                        .ok_or("SOURCE_BLOCKS_PROJECTION_REF")?;
                    require!(
                        method == Some(EXACT)
                            && success
                            && p["result_id"] == id
                            && treatment["recipe_id"].is_null()
                            && inputs.is_empty(),
                        "PROJECTION_ROW"
                    );
                    projection(p, row)?;
                    used.insert(text(&p["projection_id"])?);
                }
                "checked_derived" => {
                    require!(
                        method == Some(EXACT)
                            && success
                            && treatment["projection_id"].is_null()
                            && !treatment["recipe_id"].is_null(),
                        "DERIVED_ROW"
                    );
                    let input_rows = inputs.iter().map(|i| Ok(raw[text(i)?])).collect::<Result<
                        Vec<_>,
                        String,
                    >>(
                    )?;
                    derived(&treatment["recipe_id"], row, &input_rows)?;
                    if matches!(
                        treatment["recipe_id"].as_str(),
                        Some("translation_norm_scaled_v1" | "support_force_norm_scaled_v1")
                    ) {
                        let mut relative = 0.0_f64;
                        for input in inputs {
                            let input_row = treatments[text(input)?];
                            require!(
                                input_row["treatment"] == "qualified_projection",
                                "NORM_PROJECTION_INPUT"
                            );
                            let projection = projections
                                .get(text(&input_row["projection_id"])?)
                                .ok_or("SOURCE_BLOCKS_NORM_PROJECTION_INPUT")?;
                            relative = relative.max(number(&projection["relative_error_bound"])?);
                        }
                        require!(
                            relative <= NORM_INPUT_RELATIVE_LIMIT,
                            "NORM_TOTAL_RELATIVE_BOUND"
                        );
                    }
                    require!(
                        inputs.iter().all(|i| matches!(
                            treatments[i.as_str().unwrap()]["treatment"].as_str(),
                            Some("qualified_projection" | "checked_derived")
                        )),
                        "UNQUALIFIED_DERIVED_INPUT"
                    );
                }
                "ordinary_checked" => require!(
                    success
                        && method != Some(EXACT)
                        && treatment["projection_id"].is_null()
                        && treatment["recipe_id"].is_null()
                        && s.is_some()
                        && row["kind"] != "support_reaction_component_v2",
                    "ORDINARY_ROW"
                ),
                "inspection_only" => {
                    require!(
                        treatment["projection_id"].is_null() && treatment["recipe_id"].is_null(),
                        "INSPECTION_ROW"
                    );
                    if s.is_none_or(|s| s["category"] == "physical_quantity") {
                        usable = false;
                    }
                }
                _ => return Err("SOURCE_BLOCKS_TREATMENT".into()),
            }
        }
        require!(
            used == projections.keys().copied().collect(),
            "PROJECTION_BIJECTION"
        );
        let mut done = HashSet::new();
        while done.len() < treatments.len() {
            let ready = treatments
                .iter()
                .filter_map(|(&id, t)| {
                    (!done.contains(id)
                        && t["input_result_ids"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .all(|i| done.contains(i.as_str().unwrap())))
                    .then_some(id)
                })
                .collect::<Vec<_>>();
            require!(!ready.is_empty(), "ROW_DEPENDENCY_CYCLE");
            done.extend(ready);
        }
        if success && method == Some(EXACT) {
            supports(case, &raw, ps, model)?;
            required_affine(case, &raw, ps, model)?;
        } else {
            require!(
                array(&case["supports"])?.is_empty(),
                "UNSELECTED_SUPPORT_CERTIFICATE"
            );
        }
    }
    let invocation_work = &body["invocation_work"];
    require!(
        integer(&invocation_work["limit"])? <= 64_000_000
            && integer(&invocation_work["charged"])? <= integer(&invocation_work["limit"])?
            && invocation_charged.saturating_add(integer(&invocation_work["publication_charged"])?)
                == integer(&invocation_work["charged"])?,
        "INVOCATION_WORK_LEDGER"
    );
    let observations = array(&body["envelope_observation_result_ids"])?;
    require!(unique(observations), "OBSERVATION_IDS");
    for id in observations {
        let id = text(id)?;
        require!(covered.insert(id), "ENVELOPE_ROW_COVERAGE");
        let row = *raw.get(id).ok_or("SOURCE_BLOCKS_OBSERVATION_REF")?;
        require!(
            row["basis_ref"].is_null()
                && signature(row)?.is_some_and(|s| s["category"] != "physical_quantity"),
            "PHYSICAL_OBSERVATION_ESCAPE"
        );
    }
    require!(
        covered == raw.keys().copied().collect(),
        "ENVELOPE_ROW_COVERAGE"
    );
    require!(
        cases.is_empty()
            || cases
                .iter()
                .any(|c| c["selected_method"] == EXACT || c["outcome"] != "qualified"),
        "SOURCE_METHOD_RECORD_REQUIRED"
    );
    let aggregate = if !cases.is_empty() && qualified == cases.len() {
        "qualified"
    } else if qualified > 0 {
        "partial"
    } else {
        "unavailable"
    };
    require!(body["status"] == aggregate, "AGGREGATE_STATUS");
    summary(source, &raw, model)?;
    if model.is_some_and(|m| m["combinations"].as_array().is_some_and(|a| !a.is_empty())) {
        usable = false;
    }
    Ok(actual_invocation.is_some()
        && aggregate == "qualified"
        && usable
        && source["status"]["mechanics"] == "MECHANICS_SOLVED")
}
fn supports(
    case: &Value,
    raw: &HashMap<&str, &Value>,
    ps: &[Value],
    model: Option<&Value>,
) -> Result<(), String> {
    let records = array(&case["supports"])?;
    let plan = &case["source"];
    require!(
        records
            .iter()
            .map(|s| s["support_id"].clone())
            .collect::<Vec<_>>()
            == *array(&plan["support_ids"])?,
        "SUPPORT_COVERAGE"
    );
    let mut ideals = HashSet::new();
    let mut springs = HashSet::new();
    let mut results = HashSet::new();
    for support in records {
        let id = text(&support["support_id"])?;
        let node = text(&support["node_id"])?;
        let components = array(&support["components"])?;
        require!(
            components
                .iter()
                .filter_map(|c| c["component"].as_str())
                .collect::<HashSet<_>>()
                == HashSet::from(COMPONENTS),
            "SIX_SUPPORT_COMPONENTS"
        );
        let actual = if let Some(model) = model {
            let candidates = array(&model["supports"])?
                .iter()
                .filter(|s| s["id"] == id)
                .collect::<Vec<_>>();
            require!(
                candidates.len() == 1 && candidates[0]["node"] == node,
                "SUPPORT_NODE"
            );
            Some(candidates[0])
        } else {
            None
        };
        let node_index = if let Some(model) = model {
            let nodes = array(&model["nodes"])?;
            let matches = nodes
                .iter()
                .enumerate()
                .filter(|(_, n)| n["id"] == node)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
            require!(matches.len() == 1, "SUPPORT_NODE");
            Some(matches[0])
        } else {
            None
        };
        let mut inferred_node = None;
        for component in components {
            let slot = COMPONENTS
                .iter()
                .position(|c| component["component"] == *c)
                .ok_or("SOURCE_BLOCKS_SUPPORT_COMPONENT")?;
            let result_id = text(&component["result_id"])?;
            let row = *raw.get(result_id).ok_or("SOURCE_BLOCKS_SUPPORT_ROW")?;
            let projection = ps
                .iter()
                .find(|p| p["result_id"] == result_id)
                .ok_or("SOURCE_BLOCKS_SUPPORT_PROJECTION")?;
            require!(
                projection["functional_id"] == component["functional_id"]
                    && projection["quantity"] == "support_action_component"
                    && results.insert(result_id),
                "SUPPORT_PROJECTION"
            );
            require!(
                row["kind"] == "support_reaction_component_v2"
                    && row["entity_ref"] == id
                    && row["basis_ref"] == case["basis_ref"]
                    && row["unit"] == if slot < 3 { "N" } else { "N*m" }
                    && row["metadata"]
                        == json!({"component":COMPONENTS[slot],"coordinate_system":"global","location":"node","basis":"recovered_from_assembled_support_law","sign_convention":SUPPORT_SIGN}),
                "SUPPORT_ROW_SEMANTICS"
            );
            let terms = array(&component["action_terms"])?;
            require!(!terms.is_empty() && unique(terms), "SUPPORT_ACTION_TERMS");
            if let Some(authored) = actual {
                let stiffness = authored
                    .get("stiffness")
                    .filter(|v| !v.is_null())
                    .unwrap_or(&authored["hanger"]["stiffness"]);
                let spring = matches!(
                    authored["family"].as_str(),
                    Some("spring" | "variable_spring_hanger" | "spring_hanger")
                ) || matches!(
                    authored["hanger"]["hanger_type"].as_str(),
                    Some("variable_spring_hanger" | "spring_hanger")
                );
                let dof = ["UX", "UY", "UZ", "RX", "RY", "RZ"][slot];
                let expected = if spring && stiffness["dof"] == dof {
                    "ground_spring"
                } else if !spring
                    && authored["restraints"]
                        .as_array()
                        .is_some_and(|a| a.iter().any(|v| v == dof))
                {
                    "ideal_constraint"
                } else {
                    "structural_zero"
                };
                require!(
                    terms.len() == 1 && terms[0]["kind"] == expected,
                    "ACTUAL_SUPPORT_LAW"
                );
            }
            for term in terms {
                let dof = integer(&term["global_dof"])?;
                require!(
                    dof < integer(&plan["dof_count"])?
                        && dof % 6 == slot
                        && term["source_id"] == id,
                    "SUPPORT_ACTION_OWNER"
                );
                let inferred = *inferred_node.get_or_insert(dof / 6);
                require!(
                    dof / 6 == inferred && node_index.is_none_or(|n| dof / 6 == n),
                    "SUPPORT_ACTION_DOF"
                );
                match text(&term["kind"])? {
                    "ideal_constraint" => require!(
                        array(&plan["prescribed_dofs"])?.contains(&term["global_dof"])
                            && ideals.insert(dof),
                        "IDEAL_ATTRIBUTION_AMBIGUOUS"
                    ),
                    "ground_spring" => {
                        require!(springs.insert((id, dof)), "SPRING_ATTRIBUTION_DUPLICATE")
                    }
                    "structural_zero" => require!(
                        terms.len() == 1
                            && number(&row["value"])? == 0.0
                            && projection["basis"] == "exact_zero",
                        "STRUCTURAL_ZERO"
                    ),
                    _ => return Err("SOURCE_BLOCKS_SUPPORT_TERM_KIND".into()),
                }
            }
        }
    }
    require!(
        ideals
            == array(&plan["prescribed_dofs"])?
                .iter()
                .map(integer)
                .collect::<Result<HashSet<_>, _>>()?,
        "IDEAL_SOURCE_COVERAGE"
    );
    require!(
        results
            == ps
                .iter()
                .filter(|p| p["quantity"] == "support_action_component")
                .map(|p| text(&p["result_id"]))
                .collect::<Result<HashSet<_>, _>>()?,
        "SUPPORT_RESULT_BIJECTION"
    );
    Ok(())
}
fn required_affine(
    case: &Value,
    raw: &HashMap<&str, &Value>,
    ps: &[Value],
    model: Option<&Value>,
) -> Result<(), String> {
    let rows = ps
        .iter()
        .map(|p| {
            raw.get(p["result_id"].as_str().unwrap())
                .copied()
                .ok_or_else(|| "SOURCE_BLOCKS_PROJECTION_ROW".to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    for member in array(&case["source"]["member_ids"])? {
        for location in ["end_i", "end_j", "quarter_1", "midspan", "quarter_3"] {
            for component in [
                "axial_force",
                "shear_force_y",
                "shear_force_z",
                "torsional_moment",
                "bending_moment_y",
                "bending_moment_z",
            ] {
                require!(
                    rows.iter()
                        .filter(|r| r["entity_ref"] == *member
                            && r["metadata"]["component"] == component
                            && r["metadata"]["location"] == location)
                        .count()
                        == 1,
                    "MEMBER_PRIMARY_COVERAGE"
                );
            }
        }
    }
    let case_rows = array(&case["rows"])?
        .iter()
        .map(|r| {
            raw.get(r["result_id"].as_str().unwrap())
                .copied()
                .ok_or_else(|| "SOURCE_BLOCKS_ROW".to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    for member in array(&case["source"]["member_ids"])? {
        for location in ["end_i", "end_j", "quarter_1", "midspan", "quarter_3"] {
            for kind in [
                "element_local_axial_normal_stress",
                "element_local_bending_normal_stress_y",
                "element_local_bending_normal_stress_z",
                "element_local_torsional_shear_stress",
            ] {
                require!(
                    case_rows
                        .iter()
                        .filter(|r| r["entity_ref"] == *member
                            && r["kind"] == kind
                            && r["metadata"]["location"] == location)
                        .count()
                        == 1,
                    "MEMBER_STRESS_COVERAGE"
                );
            }
        }
        require!(case_rows.iter().filter(|r|r["entity_ref"]==*member&&r["kind"]=="open_formula_stress_summary").count()==1,"MEMBER_SUMMARY_COVERAGE");
    }
    for support in array(&case["source"]["support_ids"])? {
        require!(
            case_rows
                .iter()
                .filter(|r| r["entity_ref"] == *support && r["kind"] == "reaction_resultant")
                .count()
                == 1,
            "SUPPORT_MAGNITUDE_COVERAGE"
        );
    }
    if let Some(model) = model {
        for node in array(&model["nodes"])? {
            require!(
                case_rows
                    .iter()
                    .filter(
                        |r| r["entity_ref"] == node["id"] && r["kind"] == "displacement_magnitude"
                    )
                    .count()
                    == 1,
                "NODAL_MAGNITUDE_COVERAGE"
            );
            for kind in [
                "global_nodal_displacement_x",
                "global_nodal_displacement_y",
                "global_nodal_displacement_z",
                "global_nodal_rotation_x",
                "global_nodal_rotation_y",
                "global_nodal_rotation_z",
            ] {
                require!(
                    rows.iter()
                        .filter(|r| r["entity_ref"] == node["id"] && r["kind"] == kind)
                        .count()
                        == 1,
                    "NODAL_PRIMARY_COVERAGE"
                );
            }
        }
    }
    Ok(())
}

fn summary(
    source: &Value,
    raw: &HashMap<&str, &Value>,
    model: Option<&Value>,
) -> Result<(), String> {
    let summary = &source["summary"];
    if let Some(model) = model {
        for (key, field) in [
            ("node_count", "nodes"),
            ("segment_count", "pipe_segments"),
            ("support_count", "supports"),
            ("load_case_count", "load_cases"),
        ] {
            require!(
                integer(&summary[key])? == array(&model[field])?.len(),
                "SUMMARY_MODEL_COUNTS"
            );
        }
    }
    for (field, kind) in [
        ("max_displacement", "displacement_magnitude"),
        ("max_open_formula_stress", "open_formula_stress_summary"),
    ] {
        let headline = &summary[field];
        let candidates = raw
            .values()
            .filter(|r| r["kind"] == kind)
            .collect::<Vec<_>>();
        if candidates.is_empty() {
            require!(headline.is_null(), "SUMMARY_UNSOURCED_HEADLINE");
            continue;
        }
        require!(
            keys(headline, &["value", "unit", "location_ref", "result_ref"]),
            "SUMMARY_HEADLINE"
        );
        let selected = *raw
            .get(text(&headline["result_ref"])?)
            .ok_or("SOURCE_BLOCKS_SUMMARY_REF")?;
        let maximum = candidates
            .iter()
            .map(|r| number(&r["value"]))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .fold(f64::NEG_INFINITY, f64::max);
        require!(
            selected["kind"] == kind
                && selected["unit"] == headline["unit"]
                && selected["entity_ref"] == headline["location_ref"]
                && number(&selected["value"])?.to_bits() == number(&headline["value"])?.to_bits()
                && number(&headline["value"])? == maximum,
            "SUMMARY_RESULT_BINDING"
        );
    }
    Ok(())
}

#[cfg(test)]
mod norm_tests {
    use super::*;

    fn check(values: [f64; 3], value: f64, support: bool) -> Result<(), String> {
        let kinds = [
            "global_nodal_displacement_x",
            "global_nodal_displacement_y",
            "global_nodal_displacement_z",
        ];
        let inputs: Vec<Value> = values.into_iter().enumerate().map(|(index, value)| json!({
            "kind": if support { "support_reaction_component_v2" } else { kinds[index] },
            "metadata": { "component": COMPONENTS[index] }, "entity_ref": "synthetic-norm-owner", "value": value,
        })).collect();
        let row = json!({"kind":if support {"reaction_resultant"}else{"displacement_magnitude"},
            "unit":if support {"N"}else{"mm"},"entity_ref":"synthetic-norm-owner","value":value});
        derived(
            &json!(if support {
                "support_force_norm_scaled_v1"
            } else {
                "translation_norm_scaled_v1"
            }),
            &row,
            &inputs.iter().collect::<Vec<_>>(),
        )
    }

    #[test]
    fn scaled_norm_accepts_small_normal_values_and_harmless_normalized_underflow() {
        for support in [false, true] {
            assert_eq!(check([3.0, 4.0, 0.0], 5.0, support), Ok(()));
            let small = 2.0_f64.powi(-600);
            assert_eq!(check([small, 0.0, 0.0], small, support), Ok(()));
            assert_eq!(
                check([f64::MIN_POSITIVE, 0.0, 0.0], f64::MIN_POSITIVE, support),
                Ok(())
            );
            assert_eq!(check([1.0, f64::from_bits(1), 0.0], 1.0, support), Ok(()));
        }
    }

    #[test]
    fn scaled_norm_rejects_subnormal_output_overflow_and_one_ulp_value_changes() {
        for support in [false, true] {
            assert_eq!(
                check([f64::from_bits(1), 0.0, 0.0], f64::from_bits(1), support),
                Err("SOURCE_BLOCKS_DERIVED_NORM_RANGE".into())
            );
            assert_eq!(
                check([f64::MAX, f64::MAX, 0.0], 1.0, support),
                Err("SOURCE_BLOCKS_DERIVED_NORM_RANGE".into())
            );
            assert_eq!(
                check(
                    [1.0, 0.0, 0.0],
                    f64::from_bits(1.0_f64.to_bits() + 1),
                    support
                ),
                Err("SOURCE_BLOCKS_DERIVED_NORM_VALUE".into())
            );
            assert_eq!(check([-0.0, 0.0, -0.0], 0.0, support), Ok(()));
            assert_eq!(
                check([-0.0, 0.0, -0.0], -0.0, support),
                Err("SOURCE_BLOCKS_DERIVED_NORM_VALUE".into())
            );
            assert!(check([f64::NAN, 0.0, 0.0], 0.0, support).is_err());
        }
    }
}

#[cfg(test)]
mod stress_range_tests {
    use super::*;

    fn scalar(index: usize, action_value: f64, value: f64, location: &str) -> Result<(), String> {
        let kind = STRESS_KINDS[index];
        let component = [
            "axial_normal_stress",
            "bending_normal_stress_y",
            "bending_normal_stress_z",
            "torsional_shear_stress",
        ][index];
        let action = json!({"kind":stress_action_kind(kind).unwrap(),"unit":if index==0{"N"}else{"N*m"},"entity_ref":"stress-owner","metadata":{"location":location},"value":action_value});
        let row = json!({"kind":kind,"unit":"MPa","entity_ref":"stress-owner","metadata":{"component":component,"location":location},"value":value});
        derived(&json!("straight_open_stress_v1"), &row, &[&action])
    }
    fn summary(normal: f64, torsion: f64, value: f64) -> Result<(), String> {
        let mut inputs = Vec::new();
        for location in STRESS_LOCATIONS {
            for (index, kind) in STRESS_KINDS.iter().enumerate() {
                inputs.push(json!({"kind":kind,"unit":"MPa","entity_ref":"stress-owner","metadata":{"location":location},"value":if index==3{torsion}else{normal}}));
            }
        }
        derived(
            &json!("reviewed_stress_summary_v1"),
            &json!({"kind":"open_formula_stress_summary","unit":"MPa","entity_ref":"stress-owner","value":value}),
            &inputs.iter().collect::<Vec<_>>(),
        )
    }
    #[test]
    fn normal_stress_and_exact_logical_zero_remain_admitted() {
        for index in 0..4 {
            assert_eq!(scalar(index, 1.0, 1e-6, "midspan"), Ok(()));
            assert_eq!(scalar(index, -1.0, -1e-6, "midspan"), Ok(()));
            assert_eq!(scalar(index, -1.0, 1e-6, "end_i"), Ok(()));
            assert_eq!(scalar(index, 0.0, 0.0, "midspan"), Ok(()));
            assert_eq!(scalar(index, 0.0, -0.0, "midspan"), Ok(()));
            assert_eq!(
                scalar(index, 1.0, -1e-6, "midspan"),
                Err("SOURCE_BLOCKS_STRESS_ACTION_SIGN".into())
            );
        }
    }
    #[test]
    fn nonzero_action_cannot_become_zero_or_subnormal_mpa() {
        for index in 0..4 {
            for value in [0.0, 1.675317e-318] {
                assert_eq!(
                    scalar(index, 1e-306, value, "midspan"),
                    Err("SOURCE_BLOCKS_STRESS_OUTPUT_RANGE".into())
                );
            }
            assert_eq!(
                scalar(index, 0.0, 1e-6, "midspan"),
                Err("SOURCE_BLOCKS_STRESS_OUTPUT_RANGE".into())
            );
            assert_eq!(
                scalar(index, 1.0, f64::MAX, "midspan"),
                Err("SOURCE_BLOCKS_STRESS_PA_OBSERVATION_RANGE".into())
            );
        }
    }
    #[test]
    fn summary_excludes_torsion_but_requires_normal_nonzero_result() {
        assert_eq!(summary(0.0, 1.0, 0.0), Ok(()));
        assert_eq!(summary(0.0, 0.0, -0.0), Ok(()));
        assert_eq!(summary(1.0, 0.0, 3.0), Ok(()));
        assert_eq!(
            summary(1.0, 0.0, 0.0),
            Err("SOURCE_BLOCKS_SUMMARY_OUTPUT_RANGE".into())
        );
        assert_eq!(
            summary(1.0, 0.0, 1.675317e-318),
            Err("SOURCE_BLOCKS_SUMMARY_OUTPUT_RANGE".into())
        );
        assert_eq!(
            summary(0.0, 1.0, 1.0),
            Err("SOURCE_BLOCKS_SUMMARY_OUTPUT_RANGE".into())
        );
        assert_eq!(
            summary(1.675317e-318, 0.0, 0.0),
            Err("SOURCE_BLOCKS_SUMMARY_INPUT_RANGE".into())
        );
        assert_eq!(
            summary(1e302, 0.0, 1.0),
            Err("SOURCE_BLOCKS_SUMMARY_SUBTOTAL_RANGE".into())
        );
    }
}
