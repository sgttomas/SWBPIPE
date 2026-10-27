//! Closed source-block statement/binding checks of the exact-route composite
//! (pressure-source-1). B4/D1 (b) retired the preview route's non-composite
//! source-blocks-1 identity, its table and its receipt schema; B7b retired the
//! v2 composites (physics-source-1, load-reference-source-1). No imported receipt constructs a live exact response,
//! and a digest is never treated as arithmetic proof.
use crate::semantic_contract::signature_in;
use open_pipe_stress_canonical_json::{checked_v1_domain_canonical, ByteSink, DomainPayload};
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
/// H(d,p) = sha256(UTF8(JCS_checked({"domain":d,"payload":p}))), streamed
/// from the borrowed payload into the hasher (T4-S6): the digest and every
/// refusal are those of the former copied, serialized and re-parsed text
/// (`checked_v1_domain_canonical`).
pub fn domain_hash(domain: &str, payload: &Value) -> Result<String, String> {
    domain_digest(domain, DomainPayload::Value(payload))
}
/// `domain_hash` of an object less its top-level member `omit`, without a
/// copy of the object.
pub fn domain_hash_without(domain: &str, object: &serde_json::Map<String, Value>, omit: &str) -> Result<String, String> {
    domain_digest(domain, DomainPayload::ObjectWithout(object, omit))
}
fn domain_digest(domain: &str, payload: DomainPayload<'_>) -> Result<String, String> {
    let mut hasher = Sha256::new();
    checked_v1_domain_canonical(domain, payload, &mut ByteSink(|bytes: &[u8]| hasher.update(bytes)))?;
    Ok(format!("{:x}", hasher.finalize()))
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
    // The publication is the source less its receipt, hashed without a copy.
    let publication = source.as_object().ok_or("SOURCE_BLOCKS_RAW_FIELDS")?;
    require!(
        source["source_block_recovery"]["body"]["publication_sha256"]
            == domain_hash_without("source_blocks_publication_v1", publication, "source_block_recovery")?,
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
/// T4-U4b §4.3: the treatment of the section normal-stress extremes and
/// transverse-shear maxima a selected source case publishes beside its
/// qualified rows, and the two formula recipes it names.
pub(crate) const NOT_QUALIFIED: &str = "derived_not_qualified";
pub(crate) const NOT_QUALIFIED_KINDS: [&str; 2] =
    ["element_local_normal_stress_extreme", "element_local_transverse_shear_stress_maximum"];
/// A `derived_not_qualified` row: its kind's own recipe, and exactly the
/// station rows that recipe reads, in formula order (the axial and bending
/// stress rows of its station and basis for σ⁺/σ⁻; the station's two shear
/// rows, the end-force rows at the ends and the section rows inside, for τ).
/// σ⁺/σ⁻ are recomputed bit for bit (§2.3); τ is nonnegative here and its
/// value is the base reader's check (c_b from the evidence's ν and sections).
fn not_qualified_in(treatment: &Value, row: &Value, inputs: &[&Value]) -> Result<(), String> {
    let kind = text(&row["kind"])?;
    require!(NOT_QUALIFIED_KINDS.contains(&kind), "NOT_QUALIFIED_KIND");
    signature(row)?.ok_or("SOURCE_BLOCKS_NOT_QUALIFIED_SIGNATURE")?;
    let extremes = kind == NOT_QUALIFIED_KINDS[0];
    let recipe = treatment["recipe_id"].as_str();
    require!(
        recipe == Some(if extremes { "section_extremes_from_published_stress_rows_v1" } else { "transverse_shear_from_published_shear_rows_v1" }),
        "NOT_QUALIFIED_RECIPE"
    );
    let md = &row["metadata"];
    let location = &md["location"];
    let basis = md["basis"].as_str().and_then(|b| b.rsplit_once("; section_basis=")).map(|(_, b)| b);
    fn input_basis(r: &Value) -> Option<&str> {
        r["metadata"]["basis"].as_str().and_then(|b| b.rsplit_once("; section_basis=")).map(|(_, b)| b)
    }
    let kinds: Vec<_> = inputs.iter().filter_map(|r| r["kind"].as_str()).collect();
    let same_station = inputs.iter().all(|r| {
        r["entity_ref"] == row["entity_ref"] && r["basis_ref"] == row["basis_ref"] && r["metadata"]["location"] == *location
    });
    if extremes {
        require!(
            same_station
                && basis.is_some()
                && inputs.iter().all(|r| input_basis(r) == basis)
                && kinds == ["element_local_axial_normal_stress", "element_local_bending_normal_stress_y", "element_local_bending_normal_stress_z"],
            "NOT_QUALIFIED_INPUTS"
        );
        let v = |i: usize| inputs[i]["value"].as_f64().ok_or("SOURCE_BLOCKS_NOT_QUALIFIED_INPUTS");
        let h = open_pipe_stress_correct_norm::norm2(v(1)?, v(2)?);
        let expected = match md["component"].as_str() {
            Some("sigma_plus") => v(0)? + h,
            Some("sigma_minus") => v(0)? - h,
            _ => return Err("SOURCE_BLOCKS_NOT_QUALIFIED_KIND".into()),
        };
        require!(row["value"].as_f64().map(f64::to_bits) == Some(expected.to_bits()), "NOT_QUALIFIED_VALUE");
    } else {
        require!(
            same_station && kinds == ["element_local_shear_force_y", "element_local_shear_force_z"],
            "NOT_QUALIFIED_INPUTS"
        );
        require!(
            md["component"] == "tau_v_max" && row["value"].as_f64().is_some_and(|v| v >= 0.0 && v.is_finite()),
            "NOT_QUALIFIED_VALUE"
        );
    }
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

/// A failed case's `block_order`, judged against the current producer (RF
/// source-method extension §7): an `unsupported_block` refusal records an
/// order above 4, since the producer solves exact blocks of order up to 4, and
/// no source; every other code records no order. The receipt schema admits only
/// qualified cases, so at every reader entry the receipt shape refuses a stored
/// failure record first; this bound is therefore tested directly
/// (`rf16_failure_block_order_tests`).
fn failure_block_order(failure: &Value, case: &Value) -> Result<bool, String> {
    Ok((failure["code"] == "unsupported_block"
        && !failure["block_order"].is_null()
        && integer(&failure["block_order"])? > 4
        && case["source"].is_null())
        || (failure["code"] != "unsupported_block" && failure["block_order"].is_null()))
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
            require!(failure_block_order(failure, case)?, "FAILURE_BLOCK_ORDER");
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
            // T4-U4b §4.3: on a selected exact case the two derived kinds carry
            // exactly `derived_not_qualified` (any other kind carrying it is
            // refused in its branch, NOT_QUALIFIED_KIND).
            if success && method == Some(EXACT) && NOT_QUALIFIED_KINDS.contains(&text(&row["kind"])?) {
                require!(treatment["treatment"] == NOT_QUALIFIED, "NOT_QUALIFIED_TREATMENT");
            }
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
                // T4-U4b §4.3: a derived row formed from this publication's own
                // rows; it neither qualifies nor disqualifies the case (`usable`
                // unchanged) and carries no REQ-002 guarantee.
                NOT_QUALIFIED => {
                    require!(
                        method == Some(EXACT) && success && treatment["projection_id"].is_null(),
                        "NOT_QUALIFIED_ROW"
                    );
                    let input_rows = inputs.iter().map(|i| Ok(raw[text(i)?])).collect::<Result<Vec<_>, String>>()?;
                    not_qualified_in(treatment, row, &input_rows)?;
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
                // The solver's support decisions, mirrored at the end of this file:
                // a constant-effort support is refused (`solver_built_support`), as
                // the solver builds none; a spring is family `spring` exactly or a
                // variable spring hanger, as `support_hanger_type` reads them.
                let spring = solver_spring_support(
                    solver_built_support(authored)?,
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
                // T4-U6 (§2.1): one source stress row per basis.
                for basis in crate::section_basis::BASES {
                    require!(
                        case_rows
                            .iter()
                            .filter(|r| r["entity_ref"] == *member
                                && r["kind"] == kind
                                && r["metadata"]["location"] == location
                                && row_section_basis(r) == Some(basis))
                            .count()
                            == 1,
                        "MEMBER_STRESS_COVERAGE"
                    );
                }
            }
        }
        for basis in crate::section_basis::BASES {
            require!(
                case_rows
                    .iter()
                    .filter(|r| r["entity_ref"] == *member
                        && r["kind"] == "pipe_elastic_normal_stress_maximum_v2"
                        && row_section_basis(r) == Some(basis))
                    .count()
                    == 1,
                "MEMBER_SUMMARY_COVERAGE"
            );
        }
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

/// A stress row's section basis, named by its token and its id alike.
fn row_section_basis(row: &Value) -> Option<&'static str> {
    crate::section_basis::row_basis(row).ok().map(|(_, basis)| basis)
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
    // T4-U6 (D-5, §2.2): the composite's stress headline is per basis, bound to
    // its own basis's maxima; no governing basis is chosen.
    require!(
        summary.get("max_open_formula_stress").is_some_and(Value::is_null),
        "SUMMARY_HEADLINE"
    );
    for (field, kind, basis) in [
        ("max_displacement", "displacement_magnitude", None),
        (
            "max_open_formula_stress_nominal_section",
            "pipe_elastic_normal_stress_maximum_v2",
            Some(crate::section_basis::NOMINAL),
        ),
        (
            "max_open_formula_stress_reduced_section",
            "pipe_elastic_normal_stress_maximum_v2",
            Some(crate::section_basis::REDUCED),
        ),
    ] {
        require!(basis.is_none() || summary.get(field).is_some(), "SUMMARY_HEADLINE");
        let headline = &summary[field];
        let candidates = raw
            .values()
            .filter(|r| r["kind"] == kind && basis.is_none_or(|b| row_section_basis(r) == Some(b)))
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
                && basis.is_none_or(|b| row_section_basis(selected) == Some(b))
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

/// RF source-method extension (§7; §8 RF-16). The readers judge a receipt
/// against the current producer, which solves exact blocks of order up to 4.
/// The receipt schema admits only qualified cases (`failure` null), so a
/// stored `unsupported_block` record of any order is refused at the receipt
/// shape before `FAILURE_BLOCK_ORDER` at every reader entry (J2's
/// `RF-unsupported-block-order-3/-4` statements). RF-16 (the bound kept at 2)
/// is therefore killed here, on the check itself.
#[cfg(test)]
mod rf16_failure_block_order_tests {
    use super::{failure_block_order, shape_in};
    use serde_json::{json, Value};

    fn schema() -> Value {
        serde_json::from_str(include_str!("../../../../schemas/physics_source_recovery.schema.json")).unwrap()
    }
    fn failure(code: &str, order: Value) -> Value {
        json!({"stage":"exact_solve","code":code,"diagnostic_ref":"diagnostic:source-recovery:case","block_order":order})
    }

    #[test]
    fn unsupported_block_records_an_order_above_four_and_no_source() {
        let unsourced = json!({"source": null});
        let sourced = json!({"source": {"level": "complete_identified_represented_contributions"}});
        for order in 0..=4u64 {
            assert_eq!(failure_block_order(&failure("unsupported_block", json!(order)), &unsourced), Ok(false), "order {order}");
        }
        for order in [5u64, 6, 89, (1 << 53) - 1] {
            assert_eq!(failure_block_order(&failure("unsupported_block", json!(order)), &unsourced), Ok(true), "order {order}");
            assert_eq!(failure_block_order(&failure("unsupported_block", json!(order)), &sourced), Ok(false), "order {order} with a source");
        }
        assert_eq!(failure_block_order(&failure("unsupported_block", Value::Null), &unsourced), Ok(false));
        assert_eq!(
            failure_block_order(&failure("unsupported_block", json!(1u64 << 53)), &unsourced),
            Err("SOURCE_BLOCKS_INTEGER".into())
        );
        for code in ["unsupported_source_closure", "budget", "not_positive_definite", "arithmetic_range"] {
            assert_eq!(failure_block_order(&failure(code, Value::Null), &unsourced), Ok(true), "{code}");
            for order in [3u64, 4, 5] {
                assert_eq!(failure_block_order(&failure(code, json!(order)), &unsourced), Ok(false), "{code} {order}");
            }
        }
    }

    /// Block length and level are checked through the schema. A receipt case
    /// admits no failure record, so the order bound above is masked by it.
    #[test]
    fn block_length_and_level_are_the_schemas_and_a_case_admits_no_failure_record() {
        let schema = schema();
        let source = &schema["$defs"]["source"]["properties"];
        for n in 1..=4u64 {
            let block: Vec<u64> = (0..n).collect();
            assert!(shape_in(&json!([block]), &source["free_blocks"], &schema), "{n} DOFs");
        }
        assert!(!shape_in(&json!([[0, 1, 2, 3, 4]]), &source["free_blocks"], &schema));
        assert!(!shape_in(&json!([[]]), &source["free_blocks"], &schema));
        for level in ["complete_identified_represented_contributions", "complete_identified_declared_member_congruence"] {
            assert!(shape_in(&json!(level), &source["level"], &schema), "{level}");
        }
        for level in [json!("complete_identified_unknown_member_basis"), json!(""), Value::Null] {
            assert!(!shape_in(&level, &source["level"], &schema), "{level}");
        }
        let case = &schema["$defs"]["case"]["properties"];
        for order in [3u64, 4, 5] {
            assert!(!shape_in(&failure("unsupported_block", json!(order)), &case["failure"], &schema));
        }
        assert!(shape_in(&Value::Null, &case["failure"], &schema));
        assert!(!shape_in(&json!("unsupported"), &case["outcome"], &schema));
    }
}

/// Whether the solver makes an authored support a ground spring: the decision
/// `supports` checks the published action terms against (ACTUAL_SUPPORT_LAW).
/// result_export does not depend on product_physics (product_physics depends
/// on it), so this mirrors `core/product_physics/src/lib.rs`, whose
/// linear-support assembly builds `LinearSupport::spring` when
/// `support.family.as_deref() == Some("spring") || is_variable_spring_hanger(support)`
/// and a rigid support otherwise. `is_variable_spring_hanger` reads
/// `support_hanger_type`: the explicit `hanger.hanger_type` trimmed by
/// `str::trim` when that is non-empty, otherwise the trimmed `family`, matched
/// against `variable_spring_hanger` and `spring_hanger`. An explicit hanger type
/// therefore decides over a hanger family, while family `spring` (a plain
/// ground spring, not a hanger) counts untrimmed whatever the hanger type says.
/// `solver_spring_rule_tests` guards that solver text and checks this function
/// and the reader against a table read from it.
fn solver_spring_support(support: &Value) -> bool {
    support["family"].as_str() == Some("spring")
        || matches!(
            solver_support_hanger_type(support),
            Some("variable_spring_hanger" | "spring_hanger")
        )
}

/// The solver's `support_hanger_type`, mirrored: the explicit
/// `hanger.hanger_type` trimmed by `str::trim` when that is non-empty,
/// otherwise the trimmed `family`.
fn solver_support_hanger_type(support: &Value) -> Option<&str> {
    support["hanger"]["hanger_type"]
        .as_str()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .or_else(|| support["family"].as_str().map(str::trim))
}

/// The solver's `is_constant_effort_support`, mirrored: `support_hanger_type`
/// is `constant_effort_support`. A family `spring` does not override it.
fn solver_constant_effort_support(support: &Value) -> bool {
    matches!(
        solver_support_hanger_type(support),
        Some("constant_effort_support")
    )
}

/// The authored support, refused (CONSTANT_EFFORT_SUPPORT) when the solver
/// builds no support from it. The solver's linear-support assembly
/// (`core/product_physics/src/lib.rs`) drops a support for which
/// `is_constant_effort_support` holds before its spring decision, and source
/// recovery declines any model holding one (`source_recovery.rs`
/// `prepare_sources`), so no solver-produced plan names one in `support_ids`
/// (`source_receipt::tests::constant_effort_supports_never_enter_a_plan`).
fn solver_built_support(support: &Value) -> Result<&Value, String> {
    require!(
        !solver_constant_effort_support(support),
        "CONSTANT_EFFORT_SUPPORT"
    );
    Ok(support)
}

/// The support-reaction reader's terms for one authored support: six
/// components, the UZ slot carrying a ground spring when `spring` holds and an
/// ideal constraint otherwise, the producer's terms for each decision
/// (`core/product_physics/src/source_receipt/rows.rs`: ideal constraints only
/// for a non-spring support's restraints, a ground spring at the stiffness DOF).
/// The authored support restrains UZ and declares UZ stiffness, so only the
/// spring decision separates the two.
#[cfg(test)]
fn support_law_case(declaration: &Value, spring: bool) -> Result<(), String> {
    let mut authored = json!({"id":"support:s","node":"node:0","restraints":["UZ"],"stiffness":{"dof":"UZ"}});
    for (key, value) in declaration.as_object().unwrap() {
        authored[key] = value.clone();
    }
    let model = json!({"nodes":[{"id":"node:0"}],"supports":[authored]});
    let (mut rows, mut ps, mut components) = (Vec::new(), Vec::new(), Vec::new());
    for slot in 0..6 {
        let result_id = format!("result:{slot}");
        let kind = match (slot, spring) {
            (2, true) => "ground_spring",
            (2, false) => "ideal_constraint",
            _ => "structural_zero",
        };
        rows.push(json!({"kind":"support_reaction_component_v2","entity_ref":"support:s","basis_ref":"basis:b","unit":if slot < 3 { "N" } else { "N*m" },"value":if slot == 2 { 1.0 } else { 0.0 },"metadata":{"component":COMPONENTS[slot],"coordinate_system":"global","location":"node","basis":"recovered_from_assembled_support_law","sign_convention":crate::physics_source::SUPPORT_SIGN}}));
        ps.push(json!({"result_id":result_id,"functional_id":format!("functional:{slot}"),"quantity":"support_action_component","basis":if slot == 2 { "exact" } else { "exact_zero" }}));
        components.push(json!({"component":COMPONENTS[slot],"result_id":result_id,"functional_id":format!("functional:{slot}"),"action_terms":[{"kind":kind,"source_id":"support:s","global_dof":slot}]}));
    }
    let prescribed = if spring { json!([]) } else { json!([2]) };
    let case = json!({"basis_ref":"basis:b","supports":[{"support_id":"support:s","node_id":"node:0","components":components}],"source":{"support_ids":["support:s"],"dof_count":6,"prescribed_dofs":prescribed}});
    let raw = ps
        .iter()
        .zip(&rows)
        .map(|(p, r)| (p["result_id"].as_str().unwrap(), r))
        .collect::<HashMap<_, _>>();
    supports(&case, &raw, &ps, Some(&model))
}

/// Regressions: the reader decides a padded or conflicting hanger declaration
/// as the solver does. Each failed before the reader read declarations as the
/// solver does (at `eb9450806e`, ACTUAL_SUPPORT_LAW on the solver's terms).
#[cfg(test)]
mod support_law_regression_tests {
    use super::*;
    const LAW: &str = "SOURCE_BLOCKS_ACTUAL_SUPPORT_LAW";
    const CONSTANT_EFFORT: &str = "SOURCE_BLOCKS_CONSTANT_EFFORT_SUPPORT";

    fn decides(declaration: Value, spring: bool) {
        assert_eq!(support_law_case(&declaration, spring), Ok(()), "{declaration}");
        assert_eq!(support_law_case(&declaration, !spring), Err(LAW.into()), "{declaration}");
    }

    /// Whatever terms are published for it, the support is refused by name.
    fn refuses(declaration: Value) {
        for spring in [true, false] {
            assert_eq!(
                support_law_case(&declaration, spring),
                Err(CONSTANT_EFFORT.into()),
                "{declaration}"
            );
        }
    }

    /// Rust's `str::trim` removes U+0085 and ASCII spaces, so the padded
    /// declaration is a hanger; it keeps U+FEFF and U+001C, so a type padded
    /// with either is an unknown type, which decides over the hanger family.
    #[test]
    fn a_padded_declaration_is_read_as_the_solver_trims_it() {
        decides(json!({"family":"guide","hanger":{"hanger_type":"\u{85}spring_hanger\u{85}"}}), true);
        decides(json!({"family":"guide","hanger":{"hanger_type":" variable_spring_hanger "}}), true);
        decides(json!({"family":"\u{85}spring_hanger\u{85}"}), true);
        decides(json!({"family":" variable_spring_hanger "}), true);
        decides(json!({"family":"variable_spring_hanger","hanger":{"hanger_type":"\u{feff}spring_hanger\u{feff}"}}), false);
        decides(json!({"family":"variable_spring_hanger","hanger":{"hanger_type":"\u{1c}spring_hanger\u{1c}"}}), false);
    }

    /// A non-empty explicit type decides over a hanger family. (A
    /// constant-effort type, which decided this way at `a58d893712`, is now
    /// refused: `a_constant_effort_support_is_refused`.)
    #[test]
    fn an_explicit_hanger_type_decides_over_the_family() {
        decides(json!({"family":"spring_hanger","hanger":{"hanger_type":"guide"}}), false);
        decides(json!({"family":"variable_spring_hanger","hanger":{"hanger_type":"strut"}}), false);
    }

    /// The solver builds no support from a constant-effort declaration
    /// (`is_constant_effort_support`, before its spring decision), so a plan
    /// naming one is refused whatever terms it publishes. Each declaration
    /// failed at `a58d893712` and at `eb9450806e`: one of the two term sets was
    /// accepted (the first, Codex r1's reproduction, as a spring).
    #[test]
    fn a_constant_effort_support_is_refused() {
        refuses(json!({"family":"spring","hanger":{"hanger_type":"constant_effort_support"}}));
        // By family, the explicit type absent or empty.
        refuses(json!({"family":"constant_effort_support"}));
        refuses(json!({"family":"constant_effort_support","hanger":{"hanger_type":"  "}}));
        refuses(json!({"family":"constant_effort_support","hanger":{"hanger_type":"constant_effort_support"}}));
        // By explicit type, over any family.
        refuses(json!({"family":"guide","hanger":{"hanger_type":"constant_effort_support"}}));
        refuses(json!({"family":"spring_hanger","hanger":{"hanger_type":"constant_effort_support"}}));
        // Padded with what `str::trim` removes.
        refuses(json!({"family":" constant_effort_support "}));
        refuses(json!({"family":"\u{85}constant_effort_support\u{85}"}));
        refuses(json!({"family":"spring","hanger":{"hanger_type":"\u{85}constant_effort_support\u{85}"}}));
        refuses(json!({"family":"variable_spring_hanger","hanger":{"hanger_type":" constant_effort_support "}}));
    }
}

/// The reader decides every declaration as the solver does: refused where the
/// solver builds no support (constant effort), and otherwise spring or rigid as
/// the solver builds it. The grid is every schema `SupportFamily`, an absent,
/// null, empty, unknown and padded family, crossed with no hanger record, a
/// null record, a record without a type or with a null type, every schema
/// `SpringHanger.hanger_type`, and an empty, unknown, padded and padding-only
/// type. The solver's answers are an explicit table read from its text, which
/// the first test guards: product_physics depends on result_export, so it
/// cannot be a dev-dependency here, and its `support_hanger_type`,
/// `is_variable_spring_hanger` and `is_constant_effort_support` are
/// `pub(crate)` and its decisions are inline in its linear-support assembly,
/// so no public solver function computes them. Padding uses ASCII spaces,
/// U+0085 (which `str::trim` removes) and U+FEFF and U+001C (which it keeps).
#[cfg(test)]
mod solver_spring_rule_tests {
    use super::*;

    /// The solver's text the tables below are read from
    /// (`core/product_physics/src/lib.rs`), compared through `normalized`.
    const SOLVER_SOURCE: [&str; 3] = [
        "pub(crate) fn support_hanger_type(support: &PreviewSupport) -> Option<&str> {
    support
        .hanger
        .as_ref()
        .and_then(|hanger| hanger.hanger_type.as_deref())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .or_else(|| support.family.as_deref().map(str::trim))
}",
        "pub(crate) fn is_variable_spring_hanger(support: &PreviewSupport) -> bool {
    matches!(
        support_hanger_type(support),
        Some(\"variable_spring_hanger\" | \"spring_hanger\")
    )
}",
        "pub(crate) fn is_constant_effort_support(support: &PreviewSupport) -> bool {
    matches!(
        support_hanger_type(support),
        Some(\"constant_effort_support\")
    )
}",
    ];
    /// The linear-support assembly's exclusion of a constant-effort support,
    /// which precedes its spring decision in the same closure.
    const EXCLUSION: &str = "            if is_constant_effort_support(support) {
                return None;
            }
            let Some(&node) = node_map.get(support.node.as_str()) else {";
    const DECISION: &str = "            if support.family.as_deref() == Some(\"spring\") || is_variable_spring_hanger(support) {
                let stiffness = support_stiffness_input(support).and_then(|input| {";

    /// `text` with comments dropped and whitespace outside string and char
    /// literals collapsed: one space is kept only between two word characters
    /// or two operator characters, where dropping it could join two tokens.
    /// Literals are kept verbatim, so a change inside one still counts.
    fn normalized(text: &str) -> String {
        let chars = text.chars().collect::<Vec<_>>();
        let word = |c: char| c.is_alphanumeric() || c == '_';
        let operator = |c: char| "!#$%&*+-./:<=>?@^|~".contains(c);
        let at = |i: usize| chars.get(i).copied();
        let (mut out, mut gap, mut i) = (String::new(), false, 0);
        while i < chars.len() {
            let c = chars[i];
            if c.is_whitespace() {
                (gap, i) = (true, i + 1);
                continue;
            }
            if c == '/' && at(i + 1) == Some('/') {
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
                gap = true;
                continue;
            }
            if c == '/' && at(i + 1) == Some('*') {
                let mut depth = 0;
                while i < chars.len() {
                    if c_at(&chars, i, "/*") {
                        (depth, i) = (depth + 1, i + 2);
                    } else if c_at(&chars, i, "*/") {
                        (depth, i) = (depth - 1, i + 2);
                        if depth == 0 {
                            break;
                        }
                    } else {
                        i += 1;
                    }
                }
                gap = true;
                continue;
            }
            let last = out.chars().last();
            let joined = last.is_some_and(word) && !gap;
            if gap && last.is_some_and(|l| (word(l) && word(c)) || (operator(l) && operator(c))) {
                out.push(' ');
            }
            gap = false;
            // A literal: (its end, exclusive) or none.
            let mut prefix = i;
            if !joined && c == 'b' && matches!(at(i + 1), Some('"' | '\'' | 'r')) {
                prefix += 1;
            }
            let end = if !joined && at(prefix) == Some('r') && matches!(at(prefix + 1), Some('"' | '#')) {
                let hashes = chars[prefix + 1..].iter().take_while(|&&h| h == '#').count();
                let open = prefix + 1 + hashes;
                (at(open) == Some('"')).then(|| {
                    let close = format!("\"{}", "#".repeat(hashes));
                    (open + 1..chars.len())
                        .find(|&j| c_at(&chars, j, &close))
                        .map_or(chars.len(), |j| j + close.len())
                })
            } else if at(prefix) == Some('"') && (prefix == i || !joined) {
                let mut j = prefix + 1;
                while j < chars.len() && chars[j] != '"' {
                    j += if chars[j] == '\\' { 2 } else { 1 };
                }
                Some((j + 1).min(chars.len()))
            } else if at(prefix) == Some('\'') && at(prefix + 1) == Some('\\') {
                let mut j = prefix + 3;
                while j < chars.len() && chars[j] != '\'' {
                    j += 1;
                }
                Some((j + 1).min(chars.len()))
            } else if at(prefix) == Some('\'') && at(prefix + 2) == Some('\'') {
                Some(prefix + 3)
            } else {
                None
            };
            let end = end.unwrap_or(i + 1);
            out.extend(&chars[i..end]);
            i = end;
        }
        out
    }

    fn c_at(chars: &[char], i: usize, text: &str) -> bool {
        text.chars().enumerate().all(|(k, t)| chars.get(i + k) == Some(&t))
    }

    /// The pinned functions are present and the exclusion precedes the
    /// decision in the same closure, all read through `normalized`.
    fn pinned(solver: &str) -> Result<(), String> {
        let solver = normalized(solver);
        for text in SOLVER_SOURCE {
            if !solver.contains(&normalized(text)) {
                return Err(text.into());
            }
        }
        let exclusion = solver.find(&normalized(EXCLUSION)).ok_or(EXCLUSION)?;
        let decision = solver[exclusion..]
            .find(&normalized(DECISION))
            .ok_or(DECISION)?;
        if solver[exclusion..exclusion + decision].contains("fn ") {
            return Err("the exclusion and the decision are in different functions".into());
        }
        Ok(())
    }

    /// How a declaration reads, by the solver's `support_hanger_type`.
    #[derive(Clone, Copy, PartialEq, Debug)]
    enum Reading {
        /// `variable_spring_hanger` or `spring_hanger`.
        Hanger,
        /// `constant_effort_support`.
        ConstantEffort,
        Other,
    }
    use Reading::*;

    /// What the solver builds from a declaration.
    #[derive(Clone, Copy, PartialEq, Debug)]
    enum Solver {
        Spring,
        Rigid,
        /// Nothing: `is_constant_effort_support` holds.
        NotBuilt,
    }

    const NEL: &str = "\u{85}";
    const BOM: &str = "\u{feff}";
    const FS: &str = "\u{1c}";

    /// How the hanger record decides: `None` when the trimmed explicit type is
    /// absent or empty and the family decides, else the type's reading.
    fn hanger_cases() -> Vec<(String, Option<Value>, Option<Reading>)> {
        let typed = |t: &str, decides| (format!("hanger_type {t:?}"), Some(json!({"hanger_type":t})), decides);
        let mut cases = vec![
            ("no hanger record".into(), None, None),
            ("null hanger record".into(), Some(Value::Null), None),
            ("no hanger_type".into(), Some(json!({})), None),
            ("null hanger_type".into(), Some(json!({"hanger_type":null})), None),
            typed("variable_spring_hanger", Some(Hanger)),
            typed("spring_hanger", Some(Hanger)),
            typed("constant_effort_support", Some(ConstantEffort)),
            typed("", None),
            typed("strut", Some(Other)),
            typed("spring", Some(Other)),
            typed(" spring_hanger ", Some(Hanger)),
            typed(&format!("{NEL}spring_hanger{NEL}"), Some(Hanger)),
            typed(&format!("{BOM}spring_hanger{BOM}"), Some(Other)),
            typed(&format!("{FS}spring_hanger{FS}"), Some(Other)),
            typed("  ", None),
            typed(NEL, None),
            typed(BOM, Some(Other)),
            typed(FS, Some(Other)),
        ];
        for pad in [" ", NEL, BOM, FS] {
            let trims = pad != BOM && pad != FS;
            cases.push(typed(
                &format!("{pad}constant_effort_support{pad}"),
                Some(if trims { ConstantEffort } else { Other }),
            ));
        }
        cases
    }

    /// Each family: whether it is exactly `spring`, and its trimmed reading
    /// (used when the family decides).
    fn family_cases() -> Vec<(String, Option<Value>, bool, Reading)> {
        let named = |f: &str, spring, reading| (format!("family {f:?}"), Some(json!(f)), spring, reading);
        let mut cases = vec![
            named("anchor", false, Other),
            named("guide", false, Other),
            named("line_stop", false, Other),
            named("vertical_support", false, Other),
            named("spring", true, Other),
            named("variable_spring_hanger", false, Hanger),
            named("spring_hanger", false, Hanger),
            named("constant_effort_support", false, ConstantEffort),
            named("nonlinear", false, Other),
            ("no family".into(), None, false, Other),
            ("null family".into(), Some(Value::Null), false, Other),
            named("", false, Other),
            named("strut", false, Other),
        ];
        for pad in [" ", NEL, BOM, FS] {
            let trims = pad != BOM && pad != FS;
            let reading = |r| if trims { r } else { Other };
            cases.push(named(&format!("{pad}spring_hanger{pad}"), false, reading(Hanger)));
            // `spring` is compared untrimmed: padded, it is no ground spring.
            cases.push(named(&format!("{pad}spring{pad}"), false, Other));
            cases.push(named(
                &format!("{pad}constant_effort_support{pad}"),
                false,
                reading(ConstantEffort),
            ));
        }
        cases
    }

    /// The solver's assembly: a constant-effort reading is excluded first;
    /// otherwise family `spring` or a hanger reading is a spring.
    fn grid() -> Vec<(String, Value, Solver)> {
        let mut grid = Vec::new();
        for (family_name, family, exactly_spring, family_reading) in family_cases() {
            for (hanger_name, hanger, decides) in hanger_cases() {
                let mut declaration = json!({});
                if let Some(family) = &family {
                    declaration["family"] = family.clone();
                }
                if let Some(hanger) = &hanger {
                    declaration["hanger"] = hanger.clone();
                }
                let reading = decides.unwrap_or(family_reading);
                let solver = if reading == ConstantEffort {
                    Solver::NotBuilt
                } else if exactly_spring || reading == Hanger {
                    Solver::Spring
                } else {
                    Solver::Rigid
                };
                grid.push((format!("{family_name} | {hanger_name}"), declaration, solver));
            }
        }
        grid
    }

    #[test]
    fn the_tables_are_read_from_the_solver_as_it_stands() {
        assert_eq!(pinned(include_str!("../../../product_physics/src/lib.rs")), Ok(()));
        let schema: Value = serde_json::from_str(include_str!("../../../../schemas/model.schema.yaml")).unwrap();
        let families = schema["$defs"]["SupportFamily"]["enum"].as_array().unwrap();
        let types = schema["$defs"]["SpringHanger"]["properties"]["hanger_type"]["enum"].as_array().unwrap();
        let named = |cases: Vec<Option<Value>>| cases.into_iter().flatten().collect::<Vec<_>>();
        let family_values = named(family_cases().into_iter().map(|c| c.1).collect());
        let type_values = named(hanger_cases().into_iter().map(|c| c.1.map(|h| h["hanger_type"].clone())).collect());
        assert_eq!(family_values[..families.len()], families[..]);
        for t in types {
            assert!(type_values.contains(t), "{t}");
        }
        assert_eq!(grid().len(), family_cases().len() * hanger_cases().len());
        for solver in [Solver::Spring, Solver::Rigid, Solver::NotBuilt] {
            assert!(grid().iter().any(|g| g.2 == solver), "{solver:?}");
        }
    }

    /// The guard reads through formatting, comments included, and fails on a
    /// change of meaning in the pinned text, inside a string literal included.
    #[test]
    fn the_guard_reads_through_formatting_and_not_through_a_change() {
        let solver = include_str!("../../../product_physics/src/lib.rs");
        let edit = |from: &str, to: &str| {
            assert_eq!(solver.matches(from).count(), 1, "{from}");
            solver.replacen(from, to, 1)
        };
        let trim = "        .map(str::trim)\n        .filter(|value| !value.is_empty())\n";
        let constant = "        Some(\"constant_effort_support\")\n    )\n}";
        let exclusion = "            if is_constant_effort_support(support) {\n                return None;\n            }\n            let Some(&node)";
        let decision = "== Some(\"spring\") || is_variable_spring_hanger(support) {\n                let stiffness";
        for reformatted in [
            edit(trim, "        .map(str::trim)\n\n            .filter( |value| ! value.is_empty() ) // reformatted\n"),
            edit(constant, "        Some(\n            \"constant_effort_support\" /* reformatted */\n        )\n    )\n}"),
            edit(exclusion, "            if is_constant_effort_support(support)\n            {\n                return None;\n            }\n\n            let Some(&node)"),
            edit(decision, "== Some(\"spring\")\n                || is_variable_spring_hanger(support)\n            {\n                let stiffness"),
        ] {
            assert_eq!(pinned(&reformatted), Ok(()));
        }
        for changed in [
            edit(trim, "        .map(str::trim_start)\n        .filter(|value| !value.is_empty())\n"),
            edit(trim, "        .map(str::trim)\n        .filter(|value| value.is_empty())\n"),
            edit(constant, "        Some(\"constant_effort_support \")\n    )\n}"),
            edit(exclusion, "            let Some(&node)"),
            edit(decision, "== Some(\"spring\") && is_variable_spring_hanger(support) {\n                let stiffness"),
            edit(decision, "== Some(\"spring \") || is_variable_spring_hanger(support) {\n                let stiffness"),
        ] {
            assert!(pinned(&changed).is_err());
        }
    }

    #[test]
    fn the_reader_decides_every_declaration_as_the_solver_does() {
        for (name, declaration, solver) in grid() {
            assert_eq!(
                solver_constant_effort_support(&declaration),
                solver == Solver::NotBuilt,
                "{name}"
            );
            if solver == Solver::NotBuilt {
                for spring in [true, false] {
                    assert_eq!(
                        support_law_case(&declaration, spring),
                        Err("SOURCE_BLOCKS_CONSTANT_EFFORT_SUPPORT".into()),
                        "{name}"
                    );
                }
                continue;
            }
            let spring = solver == Solver::Spring;
            assert_eq!(solver_spring_support(&declaration), spring, "{name}");
            assert_eq!(support_law_case(&declaration, spring), Ok(()), "{name}");
            assert_eq!(
                support_law_case(&declaration, !spring),
                Err("SOURCE_BLOCKS_ACTUAL_SUPPORT_LAW".into()),
                "{name}"
            );
        }
    }
}
