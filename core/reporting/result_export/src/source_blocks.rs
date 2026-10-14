//! Closed source-block statement/binding checks of the exact-route composites
//! (physics-source-1, load-reference-source-1, pressure-source-1). B4/D1 (b)
//! retired the preview route's non-composite source-blocks-1 identity, its table
//! and its receipt schema. No imported receipt constructs a live exact response,
//! and a digest is never treated as arithmetic proof.
use crate::semantic_contract::signature_in;
use open_pipe_stress_canonical_json::canonical_json_checked_v1_text;
use open_pipe_stress_correct_norm::norm3;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};

// The fixed 64ε recipe envelope is separate from affine input uncertainty.
// It is an arithmetic-error reserve, not a comparison tolerance: the norm
// recipe (E15: one correctly rounded norm3, at most half an ulp) and its
// normal final output lie well inside it.
const NORM_ARITHMETIC_BOUND: f64 = 64.0 * f64::EPSILON;
const NORM_INPUT_RELATIVE_LIMIT: f64 =
    (1.0e-9 - NORM_ARITHMETIC_BOUND) / (1.0 + NORM_ARITHMETIC_BOUND);
// Additional conservative current-carrier reserve for the composites' straight stress.
const STRESS_ARITHMETIC_BOUND: f64 = 128.0 * f64::EPSILON;
const STRESS_INPUT_RELATIVE_LIMIT: f64 =
    (1.0e-9 - STRESS_ARITHMETIC_BOUND) / (1.0 + STRESS_ARITHMETIC_BOUND);
const EXACT: &str = "retained_source_blocks_exact_v1";
const COMPONENTS: [&str; 6] = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"];
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
        .filter(|n| *n <= 9_007_199_254_740_991)
        .and_then(|n| usize::try_from(n).ok())
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
// JSON Schema compares numbers by value: producer 0.0 must satisfy const/enum 0.
// serde_json::Number representation equality alone distinguishes integer and f64.
fn schema_scalar_equal(value: &Value, expected: &Value) -> bool {
    match (value.as_f64(), expected.as_f64()) {
        (Some(a), Some(b)) => a.is_finite() && b.is_finite() && a == b,
        _ => value == expected,
    }
}
pub(crate) fn shape_in(v: &Value, s: &Value, root: &Value) -> bool {
    shape_in_resources(v, s, root, &[])
}
/// Closed local shapes plus explicitly supplied repository resources. Unknown
/// names and missing fragments fail closed; no loading, network or inference.
pub(crate) fn shape_in_resources(v: &Value, s: &Value, root: &Value, resources: &[(&str, &Value)]) -> bool {
    if let Some(reference) = s["$ref"].as_str() {
        let (target_root, pointer) = if let Some(pointer) = reference.strip_prefix('#') {
            (root, pointer)
        } else {
            let Some((name, pointer)) = reference.split_once('#') else { return false };
            let Some((_, resource)) = resources.iter().find(|(key, _)| *key == name) else { return false };
            (*resource, pointer)
        };
        return target_root.pointer(pointer)
            .is_some_and(|target| shape_in_resources(v, target, target_root, resources));
    }
    if let Some(branches) = s["anyOf"].as_array() {
        if !branches.iter().any(|s| shape_in_resources(v, s, root, resources)) {
            return false;
        }
    }
    if let Some(branches) = s["oneOf"].as_array() {
        if branches.iter().filter(|s| shape_in_resources(v, s, root, resources)).count() != 1 {
            return false;
        }
    }
    if let Some(branches) = s["allOf"].as_array() {
        if !branches.iter().all(|s| shape_in_resources(v, s, root, resources)) {
            return false;
        }
    }
    if let Some(types) = s["type"].as_array() {
        return types.iter().any(|kind| {
            let mut branch = s.clone();
            branch["type"] = kind.clone();
            shape_in_resources(v, &branch, root, resources)
        });
    }
    if let Some(constant) = s.get("const") {
        if !schema_scalar_equal(v, constant) {
            return false;
        }
    }
    if let Some(values) = s["enum"].as_array() {
        if !values
            .iter()
            .any(|expected| schema_scalar_equal(v, expected))
        {
            return false;
        }
    }
    match s["type"].as_str().or_else(|| {
        if s.get("properties").is_some() {
            Some("object")
        } else if s.get("items").is_some() {
            Some("array")
        } else {
            None
        }
    }) {
        Some("null") => v.is_null(),
        Some("boolean") => v.is_boolean(),
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
                    .all(|(k, v)| properties.get(k).is_none_or(|s| shape_in_resources(v, s, root, resources)))
        }
        Some("array") => v.as_array().is_some_and(|a| {
            a.len() >= s["minItems"].as_u64().unwrap_or(0) as usize
                && a.len() <= s["maxItems"].as_u64().unwrap_or(16384) as usize
                && a.iter().all(|v| shape_in_resources(v, &s["items"], root, resources))
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
                && n > s["exclusiveMinimum"].as_f64().unwrap_or(f64::NEG_INFINITY)
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
/// A receipt's canonical commitment; shared by raw and transported statements.
fn validate_receipt_hash(receipt: &Value) -> Result<(), String> {
    require!(
        receipt["receipt_sha256"] == domain_hash("source_blocks_receipt_v1", &receipt["body"])?,
        "RECEIPT_HASH"
    );
    Ok(())
}
/// Bind the receipt to the raw publication. Callers validate the receipt first.
pub fn validate_publication_hash(source: &Value) -> Result<(), String> {
    let mut publication = source.clone();
    publication
        .as_object_mut()
        .ok_or("SOURCE_BLOCKS_RAW_FIELDS")?
        .remove("source_block_recovery");
    require!(
        source["source_block_recovery"]["body"]["publication_sha256"]
            == domain_hash("source_blocks_publication_v1", &publication)?,
        "PUBLICATION_HASH"
    );
    Ok(())
}
fn signature(row: &Value) -> Result<Option<&'static Value>, String> {
    let s = signature_in(crate::semantic_contract::pressure_source_contract(), row)?;
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
fn derived_in(
    recipe: &Value,
    row: &Value,
    inputs: &[&Value],
) -> Result<(), String> {
    // B4/D1 (b): the composites' only shared non-composite recipe is the
    // translation norm. The support-norm, straight-stress, stress-summary and
    // section-property recipes were source-blocks-1's (its preview case rows,
    // retired with B3), so they are refused as unknown recipes.
    let kind = text(&row["kind"])?;
    signature(row)?.ok_or("SOURCE_BLOCKS_DERIVED_SIGNATURE")?;
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
        _ => return Err("SOURCE_BLOCKS_DERIVED_RECIPE".into()),
    }
    require!(
        inputs.iter().all(|r| r["entity_ref"] == row["entity_ref"]),
        "DERIVED_ENTITY"
    );
    // E15: the declared recipe is one correctly rounded norm3 of the published
    // XYZ components. A nonzero result must be finite and normal; the claimed
    // value is bound to it bit for bit.
    let mut values = [0.0; 3];
    for (index, input) in inputs.iter().enumerate() {
        values[index] = number(&input["value"])?;
    }
    let expected = norm3(values[0], values[1], values[2]);
    require!(
        values.iter().all(|v| v.is_finite())
            && (expected == 0.0 || expected.is_normal()),
        "DERIVED_NORM_RANGE"
    );
    require!(
        number(&row["value"])?.to_bits() == expected.to_bits(),
        "DERIVED_NORM_VALUE"
    );
    Ok(())
}

/// `actual_invocation` is the independent original {request,solver_mode} Value.
/// Without it, a supported consistent receipt returns false, never admission.
/// Explicit method context; never alters received bytes to fit another contract.
pub(crate) fn validate_source_contract_in(
    source: &Value, actual_invocation: Option<&Value>, contract: crate::physics_source::SourceContract,
) -> Result<bool, String> {
    if let Some(invocation) = actual_invocation { contract.check_invocation(invocation)?; }
    contract.check_received(source)?;
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
                "source_block_recovery",
                "contract_evidence"
            ]
        ),
        "RAW_FIELDS"
    );
    require!(
        source["schema_version"] == "0.2.0"
            && source["producer"]
                == json!({"component_name":"open_pipe_stress_product_physics","component_version":"0.2.0","semantic_contract_id":contract.identity()}),
        "PRODUCER"
    );
    let formulation = &source["formulation_basis"];
    require!(
        keys(formulation, &["profile_id", "limitations"])
            && formulation["profile_id"]
                == contract.profile()
            && formulation["limitations"]
                .as_array()
                .is_some_and(|a| !a.is_empty()
                    && a.iter().all(|v| v.as_str().is_some_and(|s| !s.is_empty()))),
        "FORMULATION"
    );
    let receipt = &source["source_block_recovery"];
    crate::physics_source::validate_receipt_shape_in(receipt, contract)?;
    let body = &receipt["body"];
    validate_receipt_hash(receipt)?;
    validate_publication_hash(source)?;
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
                && integer(&work["limit"])? <= 8_000_000
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
        if rows.iter().any(|row| row["recipe_id"] == "retained_source_straight_stress_v1") {
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
        // Receipt row order, never hash order: the first refusal must be stable
        // across runs and match the Python reader's insertion-ordered walk.
        for treatment in rows {
            let id = text(&treatment["result_id"])?;
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
                    if crate::physics_source::is_composite_recipe(&treatment["recipe_id"]) {
                        crate::physics_source::validate_derived(
                            source,
                            case,
                            treatment,
                            row,
                            &input_rows,
                        )?;
                    } else {
                        derived_in(&treatment["recipe_id"], row, &input_rows)?;
                    }
                    if treatment["recipe_id"] == "translation_norm_scaled_v1" {
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
                "ordinary_physics_checked" => {
                    require!(
                        success
                            && method != Some(EXACT)
                            && treatment["projection_id"].is_null()
                            && treatment["recipe_id"].is_null()
                            && s.is_some(),
                        "ORDINARY_ROW"
                    )
                }
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
            crate::physics_source::validate_source_case_in(source, case, actual_invocation, contract)?;
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
                        == json!({"component":COMPONENTS[slot],"coordinate_system":"global","location":"node","basis":"recovered_from_assembled_support_law","sign_convention":crate::physics_source::SUPPORT_SIGN}),
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
        require!(
            case_rows
                .iter()
                .filter(|r| r["entity_ref"] == *member
                    && r["kind"]
                        == "pipe_elastic_normal_stress_maximum_v2")
                .count()
                == 1,
            "MEMBER_SUMMARY_COVERAGE"
        );
    }
    for support in array(&case["source"]["support_ids"])? {
        require!(
            case_rows
                .iter()
                .filter(|r| r["entity_ref"] == *support
                    && r["kind"]
                        == "support_reaction_force_magnitude_v2")
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
        (
            "max_open_formula_stress",
            "pipe_elastic_normal_stress_maximum_v2",
        ),
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

    fn check(values: [f64; 3], value: f64) -> Result<(), String> {
        let kinds = [
            "global_nodal_displacement_x",
            "global_nodal_displacement_y",
            "global_nodal_displacement_z",
        ];
        let inputs: Vec<Value> = values.into_iter().enumerate().map(|(index, value)| json!({
            "kind": kinds[index], "entity_ref": "synthetic-norm-owner", "value": value,
        })).collect();
        let row = json!({"kind":"displacement_magnitude","unit":"mm","entity_ref":"synthetic-norm-owner","value":value});
        derived_in(&json!("translation_norm_scaled_v1"), &row, &inputs.iter().collect::<Vec<_>>())
    }

    /// E15: the receipt recipe binds the exact correctly rounded value. Over the
    /// published (3,4,0) components the recipe gives exactly 5, so the
    /// independently single-rounded 5 + 1 ulp that G7 admits is refused here;
    /// the former max-scaled recipe's value is refused where the two differ.
    #[test]
    fn e15_receipt_norm_recipe_binds_the_correctly_rounded_value() {
        let controls: Value = serde_json::from_str(include_str!(
            "../../../../fixtures/results/preview_magnitude_controls.json"
        ))
        .unwrap();
        let control = &controls["controls"][0];
        assert_eq!(control["id"], "independent_single_rounding_one_ulp");
        let bits = |v: &Value| f64::from_bits(u64::from_str_radix(v.as_str().unwrap(), 16).unwrap());
        let components = [0, 1, 2].map(|i| bits(&control["component_bits"][i]));
        assert_eq!(components, [3.0, 4.0, 0.0]);
        let one_ulp = bits(&control["magnitude_bits"]);
        assert_eq!(one_ulp.to_bits(), 5.0_f64.to_bits() + 1);
        let scaled = |v: [f64; 3]| {
            let m = v.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
            let [a, b, c] = v.map(|x| x / m);
            m * ((a * a + b * b) + c * c).sqrt()
        };
        let differing = [-1.327, -8.603, -8.186];
        let correct = f64::from_bits(0x4027_e5fc_308f_80ed);
        assert_eq!(norm3(differing[0], differing[1], differing[2]).to_bits(), correct.to_bits());
        assert_eq!(scaled(differing).to_bits(), correct.to_bits() - 1);
        assert_eq!(check(components, 5.0), Ok(()));
        assert_eq!(check(components, one_ulp), Err("SOURCE_BLOCKS_DERIVED_NORM_VALUE".into()));
        assert_eq!(check(differing, correct), Ok(()));
        assert_eq!(
            check(differing, scaled(differing)),
            Err("SOURCE_BLOCKS_DERIVED_NORM_VALUE".into())
        );
    }

    #[test]
    fn norm_recipe_accepts_small_normal_values_and_tiny_components() {
        assert_eq!(check([3.0, 4.0, 0.0], 5.0), Ok(()));
        let small = 2.0_f64.powi(-600);
        assert_eq!(check([small, 0.0, 0.0], small), Ok(()));
        assert_eq!(check([f64::MIN_POSITIVE, 0.0, 0.0], f64::MIN_POSITIVE), Ok(()));
        assert_eq!(check([1.0, f64::from_bits(1), 0.0], 1.0), Ok(()));
    }

    #[test]
    fn norm_recipe_rejects_subnormal_output_overflow_and_one_ulp_value_changes() {
        assert_eq!(
            check([f64::from_bits(1), 0.0, 0.0], f64::from_bits(1)),
            Err("SOURCE_BLOCKS_DERIVED_NORM_RANGE".into())
        );
        assert_eq!(check([f64::MAX, f64::MAX, 0.0], 1.0), Err("SOURCE_BLOCKS_DERIVED_NORM_RANGE".into()));
        assert_eq!(
            check([1.0, 0.0, 0.0], f64::from_bits(1.0_f64.to_bits() + 1)),
            Err("SOURCE_BLOCKS_DERIVED_NORM_VALUE".into())
        );
        assert_eq!(check([-0.0, 0.0, -0.0], 0.0), Ok(()));
        assert_eq!(check([-0.0, 0.0, -0.0], -0.0), Err("SOURCE_BLOCKS_DERIVED_NORM_VALUE".into()));
        assert!(check([f64::NAN, 0.0, 0.0], 0.0).is_err());
    }
}

#[cfg(test)]
mod retired_recipe_tests {
    use super::*;

    /// B4/D1 (b): source-blocks-1's support-norm, straight-stress,
    /// stress-summary and section-property recipes went with its preview case
    /// rows; a composite receipt naming one is refused as an unknown recipe.
    #[test]
    fn source_blocks_1_recipes_are_unknown() {
        let row = json!({"kind":"displacement_magnitude","unit":"mm","entity_ref":"owner","value":0.0});
        for recipe in [
            "support_force_norm_scaled_v1",
            "straight_open_stress_v1",
            "reviewed_stress_summary_v1",
            "section_property_from_source_v1",
        ] {
            assert_eq!(
                derived_in(&json!(recipe), &row, &[]),
                Err("SOURCE_BLOCKS_DERIVED_RECIPE".into()),
                "{recipe}"
            );
        }
    }
}

/// RV95 N-5 (I74 decision 9; RV101 NT-1 and A2-N3): the direct test of
/// `integer`'s 2^53−1 bound. At the public API two earlier layers mask it (the
/// receipt shape and the checked-profile hashes; `tests/source_blocks.rs`,
/// `rv95_n5_the_integer_bound_is_masked_at_the_public_api`), so RV95's mutant
/// S1 (the bound removed) is killed only here.
#[cfg(test)]
mod rv95_n5_integer_tests {
    use super::*;
    const TWO_53: u64 = 1 << 53;
    const INTEGER: &str = "SOURCE_BLOCKS_INTEGER";

    #[test]
    fn integer_admits_two_to_53_minus_1_and_refuses_two_to_53() {
        assert_eq!(integer(&json!(0)), Ok(0));
        assert_eq!(integer(&json!(TWO_53 - 1)), Ok((TWO_53 - 1) as usize));
        for value in [json!(TWO_53), json!(TWO_53 + 1), json!(u64::MAX)] {
            assert_eq!(integer(&value), Err(INTEGER.into()), "{value}");
        }
    }

    /// The fields read through `integer`: the thirteen receipt fields, which
    /// are I76's twelve and `failure.block_order` (RV101 NT-1), and the four
    /// summary counts. The production text's `integer(...)` calls read exactly
    /// these; each refuses 2^53 as read, and where a private check reaches
    /// `integer` before any other layer (the source plan's four counts and the
    /// summary counts) it is exercised there, with 2^53−1 reaching the next
    /// check instead.
    #[test]
    fn every_field_read_through_integer_refuses_two_to_53() {
        let text = include_str!("source_blocks.rs");
        let production = &text[..text.find("\n#[cfg(test)]").unwrap()];
        let mut arguments = std::collections::BTreeSet::new();
        for (at, _) in production.match_indices("integer(&") {
            let rest = &production[at + "integer(&".len()..];
            let mut depth = 0usize;
            let end = rest
                .char_indices()
                .find(|(_, c)| match c {
                    '(' | '[' => {
                        depth += 1;
                        false
                    }
                    ']' => {
                        depth -= 1;
                        false
                    }
                    ')' => depth == 0,
                    _ => false,
                })
                .unwrap()
                .0;
            arguments.insert(&rest[..end]);
        }
        // The receipt path each argument reads (case-relative unless named).
        let receipt = [
            (r#"case["ordinary_attempt"]["quality_case_index"]"#, "/ordinary_attempt/quality_case_index"),
            (r#"work["charged"]"#, "/work/charged"),
            (r#"work["reserved_unobserved_failure"]"#, "/work/reserved_unobserved_failure"),
            (r#"work["limit"]"#, "/work/limit"),
            (r#"failure["block_order"]"#, "/failure/block_order"),
            (r#"p["dof_count"]"#, "/source/dof_count"),
            (r#"plan["dof_count"]"#, "/source/dof_count"),
            (r#"p["stiffness_term_count"]"#, "/source/stiffness_term_count"),
            (r#"p["force_term_count"]"#, "/source/force_term_count"),
            (r#"p["functional_count"]"#, "/source/functional_count"),
            (r#"term["global_dof"]"#, "/supports/0/components/0/action_terms/0/global_dof"),
            (r#"invocation_work["limit"]"#, "/invocation_work/limit"),
            (r#"invocation_work["charged"]"#, "/invocation_work/charged"),
            (r#"invocation_work["publication_charged"]"#, "/invocation_work/publication_charged"),
        ];
        let mut expected: std::collections::BTreeSet<&str> = receipt.iter().map(|(a, _)| *a).collect();
        expected.insert("summary[key]");
        assert_eq!(arguments, expected, "the production `integer` call sites");
        let fields: std::collections::BTreeSet<&str> = receipt.iter().map(|(_, f)| *f).collect();
        assert_eq!(fields.len(), 13, "I76's twelve receipt fields and failure.block_order");
        assert!(fields.contains("/failure/block_order"));
        for field in &fields {
            for (value, want) in [(TWO_53, Err(INTEGER.to_string())), (TWO_53 - 1, Ok((TWO_53 - 1) as usize))] {
                let mut receipt = json!({});
                let mut at = &mut receipt;
                for key in field.trim_start_matches('/').split('/') {
                    at = match key.parse::<usize>() {
                        Ok(0) => {
                            *at = json!([{}]);
                            &mut at[0]
                        }
                        _ => {
                            at[key] = json!({});
                            &mut at[key]
                        }
                    };
                }
                *at = json!(value);
                assert_eq!(integer(receipt.pointer(field).unwrap()), want, "{field} = {value}");
            }
        }
        // The source plan's counts, through `source_plan` itself.
        let plan = json!({"dof_count": 6, "stiffness_term_count": 1, "force_term_count": 0, "functional_count": 0});
        for (key, next) in [
            ("dof_count", "SOURCE_BLOCKS_SOURCE_COUNTS"),
            ("stiffness_term_count", "SOURCE_BLOCKS_SOURCE_COUNTS"),
            ("force_term_count", "SOURCE_BLOCKS_SOURCE_COUNTS"),
            ("functional_count", "SOURCE_BLOCKS_FUNCTIONAL_COUNT"),
        ] {
            let mut edge = plan.clone();
            edge[key] = json!(TWO_53);
            assert_eq!(source_plan(&edge, &[], None), Err(INTEGER.into()), "{key}");
            edge[key] = json!(TWO_53 - 1);
            assert_eq!(source_plan(&edge, &[], None), Err(next.into()), "{key}");
        }
        // The summary counts, through `summary` itself.
        let model = json!({"nodes": [], "pipe_segments": [], "supports": [], "load_cases": []});
        for key in ["node_count", "segment_count", "support_count", "load_case_count"] {
            let mut source = json!({"summary": {"node_count": 0, "segment_count": 0, "support_count": 0, "load_case_count": 0}});
            source["summary"][key] = json!(TWO_53);
            assert_eq!(summary(&source, &HashMap::new(), Some(&model)), Err(INTEGER.into()), "{key}");
            source["summary"][key] = json!(TWO_53 - 1);
            assert_eq!(
                summary(&source, &HashMap::new(), Some(&model)),
                Err("SOURCE_BLOCKS_SUMMARY_MODEL_COUNTS".into()),
                "{key}"
            );
        }
    }
}


#[cfg(test)]
mod explicit_resource_tests {
    use super::{shape_in, shape_in_resources};
    use serde_json::json;

    #[test]
    fn explicit_resource_shapes_are_closed_and_resolve_in_the_resource_root() {
        let root = json!({"$defs":{"Quantity":{"const":"wrong local root"}}});
        let resource = json!({"$defs":{
            "Quantity":{"type":"number"},
            "Record":{"type":"object","additionalProperties":false,"required":["value"],
                "properties":{"value":{"$ref":"#/$defs/Quantity"}}}
        }});
        let shape = json!({"$ref":"load_reference_state.schema.json#/$defs/Record"});
        let value = json!({"value":2.0});
        assert!(!shape_in(&value, &shape, &root));
        assert!(shape_in_resources(&value, &shape, &root, &[("load_reference_state.schema.json", &resource)]));
        assert!(!shape_in_resources(&json!({"value":2.0,"extra":0}), &shape, &root, &[("load_reference_state.schema.json", &resource)]));
        assert!(!shape_in_resources(&json!({"value":"2"}), &shape, &root, &[("load_reference_state.schema.json", &resource)]));
        assert!(!shape_in_resources(&value, &shape, &root, &[("different.schema.json", &resource)]));
        let missing = json!({"$ref":"load_reference_state.schema.json#/$defs/Missing"});
        assert!(!shape_in_resources(&value, &missing, &root, &[("load_reference_state.schema.json", &resource)]));
        let discovery = json!({"$ref":"https://example.invalid/schema.json#/$defs/Record"});
        assert!(!shape_in_resources(&value, &discovery, &root, &[("load_reference_state.schema.json", &resource)]));
        let nested = json!({"type":"array","items":{"type":"object","additionalProperties":false,
            "required":["records"],"properties":{"records":{"type":"array","items":shape}}}});
        let nested_value = json!([{"records":[{"value":2.0}]}]);
        assert!(shape_in_resources(&nested_value, &nested, &root, &[("load_reference_state.schema.json", &resource)]));
        assert!(!shape_in_resources(&nested_value, &nested, &root, &[]));
        let missing_nested = json!({"type":"array","items":missing});
        assert!(!shape_in_resources(&json!([value]), &missing_nested, &root, &[("load_reference_state.schema.json", &resource)]));
    }
}
