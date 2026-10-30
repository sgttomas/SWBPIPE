//! Bounded Rust mirror for the fixed local-private runner exposure gate.
//!
//! The action/reason vocabulary is pinned to DEL-12-02.  This module has no
//! transport or filesystem behavior; binaries decide whether a controlled
//! payload may be printed or written.

use serde::Serialize;
use serde_json::{Map, Value};

const INTENT_KEYS: [&str; 3] = [
    "local_private_intent",
    "explicit_local_private_intent",
    "user_intent",
];

#[cfg(test)]
const REDACTION_METADATA_KEYS: [&str; 7] = [
    "field_class",
    "privacy_classification",
    "redistribution_status",
    "review_status",
    "provenance",
    "export_policy",
    "professional_claim",
];
/// The payload keys the TypeScript and Python contracts screen (`PAYLOAD_DETAIL_KEYS`).
#[cfg(test)]
const PAYLOAD_KEYS: [&str; 12] = [
    "payload",
    "content",
    "contents",
    "raw_value",
    "project_payload",
    "rule_payload",
    "material_values",
    "component_values",
    "report_payload",
    "diagnostic_payload",
    "cache_payload",
    "sqlite_payload",
];
/// The storage-privacy metadata keys whose scalar value marks metadata present, as in the
/// TypeScript and Python contracts (`STORAGE_PRIVACY_METADATA_KEYS`).
#[cfg(test)]
const STORAGE_PRIVACY_METADATA_KEYS: [&str; 16] = [
    "contains_payload",
    "payload_present",
    "secret_material_present",
    "concrete_path_present",
    "cloud_or_network_reference",
    "direct_sql_access",
    "storage_bypass",
    "storage_bypass_requested",
    "direct_storage_bypass",
    "application_service_bypass",
    "local_private_intent",
    "explicit_local_private_intent",
    "user_intent",
    "symbolic_path_class",
    "path_class",
    "storage_locality",
];
#[cfg(test)]
const SECRET_KEYS: [&str; 10] = [
    "secret",
    "secret_value",
    "credential",
    "credential_value",
    "token",
    "password",
    "api_key",
    "access_key",
    "authorization_header",
    "private_key",
];
#[cfg(test)]
const PATH_KEYS: [&str; 9] = [
    "path",
    "file_path",
    "absolute_path",
    "local_path",
    "repository_path",
    "db_path",
    "sqlite_path",
    "concrete_path",
    "uri",
];
#[cfg(test)]
const CLOUD_KEYS: [&str; 6] = [
    "cloud_url",
    "remote_url",
    "network_location",
    "cloud_bucket",
    "cloud_sync_target",
    "hosted_database",
];
#[cfg(test)]
const SQL_KEYS: [&str; 7] = [
    "sql",
    "query",
    "table",
    "table_name",
    "sqlite_handle",
    "raw_sqlite_handle",
    "connection_string",
];

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
struct ContractClassification {
    action: &'static str,
    reason_code: &'static str,
    source_metadata_present: bool,
    finding_class: Option<&'static str>,
    finding_severity: Option<&'static str>,
}

/// The user-released export context (DEL-08-01 Design `report-user-release.md`
/// §3.1), the Rust single definition in `report_package::user_release`.  No
/// runner route uses it; the mirror carries its decision rule so the shared
/// corpus holds.
#[cfg(test)]
use open_pipe_stress_report_package::user_release::USER_RELEASED_CONTEXT as USER_RELEASED_REPORT_CONTEXT;

/// Whether an object or its `provenance` object carries an own `__proto__` member. Such an
/// object has no public basis, whatever its labels state (HELP_HUMAN ruling, 2026-10-27,
/// option (b): "fail closed on the key itself, in all three implementations"): a
/// `__proto__` key in export data is anomalous, and it may never lift or restrict a record
/// by accident. The route projector gives the record no direct public basis, and the
/// classifier reads the member as an unknown label only where the decision would otherwise
/// be an inclusion, as the last step of the decision order: every other decision stands. A
/// serde_json map, parsed or built, always shows a `__proto__` key. Identical in redactionExportControls.ts
/// (`carriesProtoMember`) and core/security/redaction/controls.py (`carries_proto_member`).
#[cfg(test)]
fn carries_proto_member(object: &Map<String, Value>) -> bool {
    object.contains_key("__proto__")
        || object
            .get("provenance")
            .and_then(Value::as_object)
            .is_some_and(|provenance| provenance.contains_key("__proto__"))
}

/// Mirror the existing core contract's item classification without adding
/// filesystem, transport, or side-effect behavior.  The runner route uses a
/// fixed local/private projection; the canonical corpus test below pins the
/// vocabulary and precedence rules shared with Python and TypeScript.
/// `wrapper_release_intent` is admitted only in the user-released context
/// (the flag assertion, §3.3), and payload-carried intent is never read there.
#[cfg(test)]
fn classify_contract_item(
    item: &Map<String, Value>,
    export_context: &str,
    wrapper_intent: bool,
    wrapper_release_intent: bool,
) -> ContractClassification {
    assert!(
        !wrapper_release_intent || export_context == USER_RELEASED_REPORT_CONTEXT,
        "explicit user-release intent is admitted only in the {USER_RELEASED_REPORT_CONTEXT} context, not in {export_context}"
    );
    // Every context reads its inputs exactly as the TypeScript and Python contracts do
    // (`classify` / `_classify`): all twelve payload keys, the scalar storage-privacy
    // metadata keys, the provenance fallback for a null privacy or redistribution
    // member and for a null or `unknown` review, the unknown-label sets and the public
    // pairing.  Until the 2026-10-27 alignment the five existing contexts kept an older
    // reading of these inputs (`docs/security/redaction_export_controls.md`).
    let user_released = export_context == USER_RELEASED_REPORT_CONTEXT;
    let unsafe_present = PAYLOAD_KEYS
        .iter()
        .chain(SECRET_KEYS.iter())
        .chain(PATH_KEYS.iter())
        .chain(CLOUD_KEYS.iter())
        .chain(SQL_KEYS.iter())
        .any(|key| item.contains_key(*key));
    let metadata_present = REDACTION_METADATA_KEYS
        .iter()
        .any(|key| item.contains_key(*key))
        || unsafe_present
        || STORAGE_PRIVACY_METADATA_KEYS.iter().any(|key| {
            item.get(*key)
                .is_some_and(|value| !value.is_array() && !value.is_object())
        });
    let provenance = item.get("provenance").and_then(Value::as_object);
    let stated_or_provenance = |key: &str| match item.get(key) {
        Some(value) if !value.is_null() => Some(value),
        own => provenance.map_or(own, |value| value.get(key)),
    };
    let privacy = stated_or_provenance("privacy_classification");
    let redistribution = stated_or_provenance("redistribution_status");
    // A review status that is not a string reads as `unknown` and gives way to the
    // provenance's (input contract); a non-string privacy or redistribution stays a stated
    // label that matches no class.
    let review = match item.get("review_status") {
        Some(value) if value.as_str().is_some_and(|text| text != "unknown") => Some(value),
        own => provenance.map_or(own, |value| value.get("review_status")),
    };
    // Payload-carried local-private intent is not read in the user-released context
    // (§3.2): only the wrapper's release intent decides there.
    let explicit_intent = if user_released {
        false
    } else {
        let item_intent = INTENT_KEYS
            .iter()
            .find_map(|key| item.get(*key).map(intent_flag))
            .or_else(|| {
                item.get("export_policy")
                    .and_then(Value::as_object)
                    .and_then(|policy| {
                        INTENT_KEYS
                            .iter()
                            .find_map(|key| policy.get(*key).map(intent_flag))
                    })
            });
        match item_intent {
            Some(false) => false,
            Some(true) => true,
            None => wrapper_intent,
        }
    };
    let privacy_text = privacy.and_then(Value::as_str);
    let redistribution_text = redistribution.and_then(Value::as_str);
    // As in the TypeScript and Python contracts, any safe privacy class with any public
    // redistribution status is safe, in every context; the reason follows the privacy.
    let safe_public = match (privacy_text, redistribution_text) {
        (
            Some(privacy @ ("public" | "public_metadata" | "invented_public_example")),
            Some("public_permissive" | "invented_non_engineering_example"),
        ) => Some(if privacy == "invented_public_example" {
            "SAFE_INVENTED_PUBLIC_EXAMPLE"
        } else {
            "SAFE_PUBLIC_METADATA"
        }),
        _ => None,
    };
    let review_text = review.and_then(Value::as_str).unwrap_or("unknown");

    let contains_payload = blocking_flag_option(item.get("contains_payload"))
        || blocking_flag_option(item.get("payload_present"))
        || PAYLOAD_KEYS.iter().any(|key| item.contains_key(*key));
    let secret_present = blocking_flag_option(item.get("secret_material_present"))
        || SECRET_KEYS.iter().any(|key| item.contains_key(*key));
    let cloud_present = blocking_flag_option(item.get("cloud_or_network_reference"))
        || CLOUD_KEYS.iter().any(|key| item.contains_key(*key));
    let sql_present = blocking_flag_option(item.get("direct_sql_access"))
        || SQL_KEYS.iter().any(|key| item.contains_key(*key));
    let storage_bypass = [
        "storage_bypass",
        "storage_bypass_requested",
        "direct_storage_bypass",
        "application_service_bypass",
    ]
    .iter()
    .any(|key| blocking_flag_option(item.get(*key)));
    let path_present = blocking_flag_option(item.get("concrete_path_present"))
        || PATH_KEYS.iter().any(|key| item.contains_key(*key));

    let unknown_action = if export_context == "local_private" {
        "warning_only"
    } else {
        "redact_value"
    };
    let (decided_action, decided_reason) = if professional_claim(item.get("professional_claim")) {
        ("block_export", "PROFESSIONAL_BOUNDARY_BLOCKED")
    } else if cloud_present {
        ("block_export", "CLOUD_OR_NETWORK_REFERENCE_BLOCKED")
    } else if sql_present {
        ("block_export", "DIRECT_SQL_ACCESS_BLOCKED")
    } else if storage_bypass {
        ("block_export", "STORAGE_BYPASS_BLOCKED")
    } else if secret_present {
        ("block_export", "SECRET_MATERIAL_BLOCKED")
    } else if contains_payload {
        ("block_export", "PAYLOAD_METADATA_ONLY_REQUIRED")
    } else if privacy_text == Some("protected_suspected")
        || redistribution_text == Some("protected_suspected")
        || matches!(review_text, "quarantined" | "rejected")
    {
        ("block_export", "PROTECTED_CONTENT_BLOCKED")
    } else if export_context == USER_RELEASED_REPORT_CONTEXT {
        // Steps 10-13 of §3.2, before any unknown-metadata branch.
        if matches!(
            privacy_text,
            Some("private_material_data")
                | Some("private_component_data")
                | Some("private_rule_pack_data")
                | Some("owner_standard_data")
                | Some("company_design_basis_data")
                | Some("path_data")
                | Some("secret_like_data")
        ) {
            ("redact_value", "PRIVATE_DATA_REDACTED")
        } else if privacy_text == Some("private_project_data") {
            if wrapper_release_intent {
                ("warning_only", "USER_RELEASED_PROJECT_DATA")
            } else {
                ("block_export", "USER_RELEASE_INTENT_REQUIRED")
            }
        } else if let Some(reason) = safe_public {
            ("include", reason)
        } else {
            ("block_export", "USER_RELEASE_VALUE_UNBOUND")
        }
    } else if !metadata_present {
        (unknown_action, "MISSING_METADATA_REDACTED")
    } else if unknown_label(redistribution) {
        (unknown_action, "REDISTRIBUTION_STATUS_UNKNOWN")
    } else if unknown_label(privacy) {
        (unknown_action, "UNKNOWN_PROVENANCE_WARNING")
    } else if let Some(reason) = safe_public {
        ("include", reason)
    } else if matches!(
        privacy_text,
        Some("private_project_data")
            | Some("private_material_data")
            | Some("private_component_data")
            | Some("private_rule_pack_data")
            | Some("owner_standard_data")
            | Some("company_design_basis_data")
            | Some("path_data")
            | Some("secret_like_data")
    ) || redistribution_text == Some("private_only")
    {
        if export_context == "local_private" && explicit_intent {
            ("warning_only", "PRIVATE_LOCAL_ALLOWED_WITH_WARNING")
        } else if export_context == "local_private" {
            ("block_export", "LOCAL_PRIVATE_INTENT_REQUIRED")
        } else {
            ("redact_value", "PRIVATE_DATA_REDACTED")
        }
    } else {
        (unknown_action, "UNKNOWN_PROVENANCE_WARNING")
    };
    // A concrete path redacts only what would not block (input contract, 2026-10-27): it
    // never turns a protected, quarantined/rejected, intent or user-release block into a
    // redaction.
    let (action, reason) = if path_present && decided_action != "block_export" {
        ("redact_value", "CONCRETE_PATH_REDACTED")
    } else if decided_action == "include" && carries_proto_member(item) {
        // Last, after every other step: an own `__proto__` member turns only a final
        // inclusion into the unknown-label decision (`carries_proto_member`); every other
        // decision stands.
        if user_released {
            ("block_export", "USER_RELEASE_VALUE_UNBOUND")
        } else {
            (unknown_action, "UNKNOWN_PROVENANCE_WARNING")
        }
    } else {
        (decided_action, decided_reason)
    };
    let finding_class = match reason {
        "SAFE_PUBLIC_METADATA" | "SAFE_INVENTED_PUBLIC_EXAMPLE" => None,
        "PRIVATE_DATA_REDACTED"
        | "PRIVATE_LOCAL_ALLOWED_WITH_WARNING"
        | "LOCAL_PRIVATE_INTENT_REQUIRED"
        | "SECRET_MATERIAL_BLOCKED"
        | "USER_RELEASED_PROJECT_DATA"
        | "USER_RELEASE_INTENT_REQUIRED" => Some("PRIVATE_DATA_WARNING"),
        "UNKNOWN_PROVENANCE_WARNING"
        | "REDISTRIBUTION_STATUS_UNKNOWN"
        | "MISSING_METADATA_REDACTED"
        | "USER_RELEASE_VALUE_UNBOUND" => Some("PROVENANCE_WARNING"),
        "PROTECTED_CONTENT_BLOCKED" => Some("IP_BOUNDARY_WARNING"),
        "PROFESSIONAL_BOUNDARY_BLOCKED" => Some("PROFESSIONAL_BOUNDARY_WARNING"),
        _ => Some("STORAGE_BOUNDARY_WARNING"),
    };
    ContractClassification {
        action,
        reason_code: reason,
        source_metadata_present: metadata_present,
        finding_class,
        finding_severity: finding_class.map(|_| {
            if action == "block_export" {
                "BLOCKING"
            } else {
                "WARNING"
            }
        }),
    }
}

/// The generic route projector's direct public-basis reading of a record, as the
/// TypeScript (`projectRouteValue`) and Python (`_declares_direct_public_basis`)
/// projectors read it: its privacy is its own `privacy_classification`, or its
/// `classification` alias only when `privacy_classification` is absent, so a present
/// primary label, null included, is the label (HELP_HUMAN decision, 2026-10-27); a public
/// basis is a safe privacy class with a public redistribution status, both strings. The
/// record's provenance is not read. The runners do not project routes; this mirror holds
/// the shared sweep's projector cases. A record carrying an own `__proto__` member, on itself
/// or its provenance, has no direct public basis (`carries_proto_member`).
#[cfg(test)]
fn route_record_public_basis(record: &Map<String, Value>) -> bool {
    if carries_proto_member(record) {
        return false;
    }
    let privacy = match record.get("privacy_classification") {
        Some(stated) => Some(stated),
        None => record.get("classification"),
    };
    matches!(
        privacy.and_then(Value::as_str),
        Some("public" | "public_metadata" | "invented_public_example")
    ) && matches!(
        record.get("redistribution_status").and_then(Value::as_str),
        Some("public_permissive" | "invented_non_engineering_example")
    )
}

/// The labelled leaf the route projector makes of a record's scalar member `key`, for a
/// member the projector labels by its record's basis alone: a string other than `TBD`
/// whose path holds no `tbd` and no private path token (such as the sweep's `note`).
/// It is public metadata on a public basis, else unknown and pending. Other leaves are
/// not mirrored.
#[cfg(test)]
fn route_basis_leaf(record: &Map<String, Value>, key: &str) -> Map<String, Value> {
    let value = record.get(key).expect("record leaf");
    assert!(
        value.as_str().is_some_and(|text| text != "TBD")
            && !key.to_ascii_lowercase().contains("tbd"),
        "only a non-TBD string leaf is mirrored"
    );
    let (privacy, redistribution, review) = if route_record_public_basis(record) {
        ("public_metadata", "public_permissive", "accepted")
    } else {
        ("unknown", "unknown", "pending")
    };
    Map::from_iter([
        ("field_class".to_string(), serde_json::json!(key)),
        (
            "privacy_classification".to_string(),
            serde_json::json!(privacy),
        ),
        (
            "redistribution_status".to_string(),
            serde_json::json!(redistribution),
        ),
        ("review_status".to_string(), serde_json::json!(review)),
        ("value".to_string(), value.clone()),
    ])
}

/// The path tokens the generic route projector labels private (TypeScript
/// `PRIVATE_ROUTE_TOKENS`, Python `PRIVATE_PATH_TOKENS`).
#[cfg(test)]
const ROUTE_PRIVATE_PATH_TOKENS: [&str; 24] = [
    "component",
    "content",
    "coordinate",
    "design_basis",
    "displacement",
    "force",
    "free_metadata",
    "geometry",
    "diameter",
    "material",
    "moment",
    "owner",
    "path",
    "pipe_segment",
    "project_name",
    "project_ref",
    "project_id",
    "result_value",
    "rotation",
    "rule_detail",
    "stress",
    "text",
    "thickness",
    "value",
];

/// One decision of the mirrored route exporter: its unprefixed path, action and reason.
#[cfg(test)]
type RouteDecision = (String, &'static str, &'static str);

#[cfg(test)]
fn route_action_rank(action: &str) -> usize {
    match action {
        "include" => 0,
        "warning_only" => 1,
        "redact_value" | "redact_field" => 2,
        "omit_field" => 3,
        _ => 4,
    }
}

/// A scalar privacy, redistribution or review label on a record or its provenance object:
/// the route projector's item test (TypeScript `hasContractLabel`, Python
/// `_has_contract_metadata`).
#[cfg(test)]
fn route_item_label(record: &Map<String, Value>) -> bool {
    let scalar_label = |object: &Map<String, Value>| {
        [
            "privacy_classification",
            "redistribution_status",
            "review_status",
        ]
        .iter()
        .any(|key| {
            object
                .get(*key)
                .is_some_and(|value| !value.is_object() && !value.is_array())
        })
    };
    scalar_label(record)
        || record
            .get("provenance")
            .and_then(Value::as_object)
            .is_some_and(scalar_label)
}

/// Any redaction metadata key, or a scalar storage-privacy key (TypeScript
/// `hasExplicitMetadata`, Python `_has_explicit_metadata`).
#[cfg(test)]
fn route_explicit_metadata(record: &Map<String, Value>) -> bool {
    REDACTION_METADATA_KEYS
        .iter()
        .any(|key| record.contains_key(*key))
        || STORAGE_PRIVACY_METADATA_KEYS.iter().any(|key| {
            record
                .get(*key)
                .is_some_and(|value| !value.is_object() && !value.is_array())
        })
}

#[cfg(test)]
fn route_unsafe_detail_key(key: &str) -> bool {
    PAYLOAD_KEYS
        .iter()
        .chain(SECRET_KEYS.iter())
        .chain(PATH_KEYS.iter())
        .chain(CLOUD_KEYS.iter())
        .chain(SQL_KEYS.iter())
        .any(|known| *known == key)
}

/// The labelled leaf the route projector makes of a scalar at `path` (TypeScript
/// `projectRouteValue`'s leaf, Python `_leaf_classification`), for a route with no
/// structural projection.
#[cfg(test)]
fn route_path_leaf(path: &str, value: &Value, public_basis: bool) -> Map<String, Value> {
    let lowered = path.to_lowercase().replace('-', "_");
    let unknown = value.is_null() || value.as_str() == Some("TBD") || lowered.contains("tbd");
    let segments: Vec<String> = lowered
        .replace('[', ".")
        .replace(']', "")
        .split('.')
        .map(str::to_string)
        .collect();
    let leaf = segments
        .last()
        .cloned()
        .unwrap_or_else(|| "unknown".to_string());
    let checksum_value =
        leaf == "value" && (lowered.contains("checksum") || lowered.contains("hash"));
    let unit_map_leaf =
        lowered.contains(".model_units.") || lowered.contains(".target_export_units.");
    let private_leaf = !checksum_value
        && !unit_map_leaf
        && ROUTE_PRIVATE_PATH_TOKENS
            .iter()
            .any(|token| leaf == *token || leaf.ends_with(&format!("_{token}")));
    let private = private_leaf
        || ROUTE_PRIVATE_PATH_TOKENS
            .iter()
            .any(|token| lowered.contains(token));
    let (privacy, redistribution, review) = if unknown {
        ("unknown", "unknown", "pending")
    } else if private {
        ("private_project_data", "private_only", "accepted")
    } else if public_basis {
        ("public_metadata", "public_permissive", "accepted")
    } else {
        ("unknown", "unknown", "pending")
    };
    let field_class = path
        .rsplit('.')
        .next()
        .unwrap_or("")
        .split('[')
        .next()
        .filter(|class| !class.is_empty())
        .unwrap_or("unknown");
    Map::from_iter([
        ("field_class".to_string(), serde_json::json!(field_class)),
        (
            "privacy_classification".to_string(),
            serde_json::json!(privacy),
        ),
        (
            "redistribution_status".to_string(),
            serde_json::json!(redistribution),
        ),
        ("review_status".to_string(), serde_json::json!(review)),
        ("value".to_string(), value.clone()),
    ])
}

/// A decision raised to a value record's item decision where that is stricter, keeping the
/// item's own reason; beneath an omitted unsafe path key, omitted unless it blocks
/// (TypeScript `raisedDecision`, Python `_raised_decision`).
#[cfg(test)]
fn route_raised(
    decided: (&'static str, &'static str),
    floor: Option<(&'static str, &'static str)>,
    omit: bool,
) -> (&'static str, &'static str) {
    let mut raised = decided;
    if let Some(floor) = floor {
        if route_action_rank(floor.0) > route_action_rank(raised.0) {
            raised = floor;
        }
        if omit && raised.0 != "block_export" {
            raised = ("omit_field", floor.1);
        }
    }
    raised
}

/// The generic route exporter's decisions on an intent-stripped value, as TypeScript
/// (`projectRouteValue` then `walk`) and Python (`_project` then `_walk`) make them, for a
/// route with no structural projection; value records as `walkValueRecord` /
/// `_walk_value_record` decide them (HELP_HUMAN decision, 2026-10-28: "take the stricter
/// decision per case").
#[cfg(test)]
#[allow(clippy::too_many_arguments)]
fn route_walk(
    value: &Value,
    path: &str,
    public_basis: bool,
    context: &str,
    intent: bool,
    floor: Option<(&'static str, &'static str)>,
    omit: bool,
    out: &mut Vec<RouteDecision>,
) {
    let decide = |item: &Map<String, Value>| {
        let decided = classify_contract_item(item, context, intent, false);
        (decided.action, decided.reason_code)
    };
    match value {
        Value::Object(record) => {
            let scalar_value = record
                .get("value")
                .is_some_and(|value| !value.is_object() && !value.is_array());
            if scalar_value && route_item_label(record) {
                let (action, reason) = route_raised(decide(record), floor, omit);
                out.push((path.to_string(), action, reason));
                return;
            }
            if scalar_value && route_explicit_metadata(record) {
                let item = decide(record);
                if item.0 == "block_export" {
                    out.push((path.to_string(), item.0, item.1));
                    return;
                }
                let redacts = route_action_rank(item.0) >= route_action_rank("redact_value");
                let basis = route_record_public_basis(record);
                for (key, child) in record {
                    let own_omit = redacts && route_unsafe_detail_key(key);
                    let raise = own_omit
                        || key == "value"
                        || key == "text"
                        || (redacts && key == "privacy_classification");
                    // A floor and an omission inherited from an enclosing value record still
                    // hold here (Codex privacy review of #1722, P1-3): the stricter of the two
                    // floors applies; on a tie, the enclosing one.
                    let member_floor = match (floor, raise) {
                        (Some(inherited), true)
                            if route_action_rank(item.0) > route_action_rank(inherited.0) =>
                        {
                            Some(item)
                        }
                        (Some(inherited), _) => Some(inherited),
                        (None, true) => Some(item),
                        (None, false) => None,
                    };
                    route_walk(
                        child,
                        &format!("{path}.{key}"),
                        basis,
                        context,
                        intent,
                        member_floor,
                        own_omit || omit,
                        out,
                    );
                }
                return;
            }
            let basis = route_record_public_basis(record);
            for (key, child) in record {
                route_walk(
                    child,
                    &format!("{path}.{key}"),
                    basis,
                    context,
                    intent,
                    floor,
                    omit,
                    out,
                );
            }
        }
        Value::Array(items) => {
            for (index, child) in items.iter().enumerate() {
                route_walk(
                    child,
                    &format!("{path}[{index}]"),
                    false,
                    context,
                    intent,
                    floor,
                    omit,
                    out,
                );
            }
        }
        scalar => {
            let leaf = route_path_leaf(path, scalar, public_basis);
            let (action, reason) = route_raised(decide(&leaf), floor, omit);
            out.push((path.to_string(), action, reason));
        }
    }
}

/// The mirrored route exporter's decisions on `{record_key: record}`.
#[cfg(test)]
fn route_record_decisions(
    record_key: &str,
    record: &Value,
    context: &str,
    intent: bool,
) -> Vec<RouteDecision> {
    let input = serde_json::json!({ record_key: record });
    // Reserved internal names (HELP_HUMAN ruling, 2026-10-28: "fail closed on reserved internal
    // names, in all three implementations"): a member so named anywhere in the input blocks the
    // whole export before projection, one decision per such member, naming its path only.
    let mut reserved = Vec::new();
    route_reserved_name_paths(&input, "$", &mut reserved);
    if !reserved.is_empty() {
        reserved.sort();
        return reserved
            .into_iter()
            .map(|path| (path, "block_export", "EXPORT_INPUT_RESERVED_NAME"))
            .collect();
    }
    let payload = strip_intent(input);
    let mut out = Vec::new();
    route_walk(&payload, "$", false, context, intent, None, false, &mut out);
    out
}

/// A member name of the route exporter's internal names: the projector's key prefix, its
/// labelled-leaf marker and its old value-record marker (TypeScript `isReservedName`, Python
/// `_reserved_name`).
#[cfg(test)]
fn route_reserved_name(key: &str) -> bool {
    key.starts_with("__route_key__")
        || key == "_route_projected_leaf"
        || key == "_route_value_record"
}

/// The paths of every member with a reserved name, at any depth; iterative.
#[cfg(test)]
fn route_reserved_name_paths(value: &Value, root: &str, found: &mut Vec<String>) {
    let mut stack: Vec<(&Value, String)> = vec![(value, root.to_string())];
    while let Some((node, at)) = stack.pop() {
        match node {
            Value::Object(map) => {
                for (key, item) in map {
                    if route_reserved_name(key) {
                        found.push(format!("{at}.{key}"));
                    }
                    stack.push((item, format!("{at}.{key}")));
                }
            }
            Value::Array(items) => {
                for (index, item) in items.iter().enumerate() {
                    stack.push((item, format!("{at}[{index}]")));
                }
            }
            _ => {}
        }
    }
}

/// A label the TypeScript and Python contracts read as unknown (`UNKNOWN_*` sets): absent,
/// null, `unknown` or `TBD`.  Any other value, a non-string included, is a stated label.
#[cfg(test)]
fn unknown_label(value: Option<&Value>) -> bool {
    match value {
        None | Some(Value::Null) => true,
        Some(Value::String(text)) => matches!(text.as_str(), "unknown" | "TBD"),
        Some(_) => false,
    }
}

// Input contract (docs/security/redaction_export_controls.md, "Input contract"; HELP_HUMAN
// ruling 2026-10-27).  The flag strings all three implementations recognize, after trimming
// and lowercasing; identical in core/security/redaction/controls.py and
// redactionExportControls.ts.
#[cfg(test)]
const RECOGNIZED_FLAG_STRINGS: [&str; 6] = [
    "true",
    "yes",
    "explicit",
    "present",
    "local_private",
    "explicit_local_private",
];
/// Trimmed from an intent flag: ASCII whitespace only, U+0009-U+000D and U+0020.
#[cfg(test)]
const INTENT_FLAG_TRIM: [char; 6] = ['\u{9}', '\u{a}', '\u{b}', '\u{c}', '\u{d}', '\u{20}'];
/// Trimmed from a blocking flag: the union of what Rust's `str::trim`, Python's
/// `str.strip()` (core/security/redaction/controls.py) and JavaScript's
/// `String.prototype.trim` (redactionExportControls.ts) removed before the contract, so no
/// implementation reads a blocking flag as unset where it read it as set.  The same code
/// points as `BLOCKING_FLAG_TRIM` there.
#[cfg(test)]
const BLOCKING_FLAG_TRIM: [char; 30] = [
    '\u{9}', '\u{a}', '\u{b}', '\u{c}', '\u{d}', '\u{1c}', '\u{1d}', '\u{1e}', '\u{1f}', '\u{20}',
    '\u{85}', '\u{a0}', '\u{1680}', '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}',
    '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}', '\u{2009}', '\u{200a}', '\u{2028}', '\u{2029}',
    '\u{202f}', '\u{205f}', '\u{3000}', '\u{feff}',
];

#[cfg(test)]
fn recognized_flag_string(text: &str) -> bool {
    RECOGNIZED_FLAG_STRINGS.contains(&text.to_ascii_lowercase().as_str())
}

#[cfg(test)]
fn blocking_flag_option(value: Option<&Value>) -> bool {
    value.map(blocking_flag).unwrap_or(false)
}

/// A blocking flag is set by `true`, a non-zero number, or a recognized string after
/// trimming `BLOCKING_FLAG_TRIM` (input contract).
#[cfg(test)]
fn blocking_flag(value: &Value) -> bool {
    match value {
        Value::Bool(value) => *value,
        Value::String(value) => {
            recognized_flag_string(value.trim_matches(|c| BLOCKING_FLAG_TRIM.contains(&c)))
        }
        Value::Null | Value::Array(_) | Value::Object(_) => false,
        Value::Number(value) => value.as_f64().map(|number| number != 0.0).unwrap_or(false),
    }
}

/// An intent flag is set only by `true` or a recognized string after trimming ASCII
/// whitespace (input contract); a number, an array or an object is not intent.
#[cfg(test)]
fn intent_flag(value: &Value) -> bool {
    match value {
        Value::Bool(value) => *value,
        Value::String(value) => {
            recognized_flag_string(value.trim_matches(|c| INTENT_FLAG_TRIM.contains(&c)))
        }
        _ => false,
    }
}

/// A professional claim blocks unless it is absent, null or `false` (input contract).
#[cfg(test)]
fn professional_claim(value: Option<&Value>) -> bool {
    !matches!(value, None | Some(Value::Null) | Some(Value::Bool(false)))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RedactionDecision {
    pub decision_id: String,
    pub path: String,
    pub field_class: String,
    pub privacy_classification: String,
    pub redistribution_status: String,
    pub review_status: String,
    pub export_context: &'static str,
    pub action: &'static str,
    pub reason_code: &'static str,
    pub source_metadata_present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RedactionFinding {
    pub finding_id: String,
    pub code: &'static str,
    pub class: &'static str,
    pub severity: &'static str,
    pub path: String,
    pub action: &'static str,
    pub message: &'static str,
    pub remediation: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RedactionSummary {
    pub decision_count: usize,
    pub finding_count: usize,
    pub redacted_count: usize,
    pub omitted_count: usize,
    pub warning_count: usize,
    pub blocking_count: usize,
    pub cloud_transmission_attempted: bool,
    pub professional_claims_made: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ControlledExport {
    pub payload: Option<Value>,
    pub decisions: Vec<RedactionDecision>,
    pub findings: Vec<RedactionFinding>,
    pub blocked: bool,
    pub summary: RedactionSummary,
}

pub fn control_local_private(payload: Value, explicit_intent: bool) -> ControlledExport {
    let stripped = strip_intent(payload);
    let mut decisions = Vec::new();
    let mut findings = Vec::new();
    walk(
        &stripped,
        "$",
        explicit_intent,
        &mut decisions,
        &mut findings,
    );
    let blocked = decisions
        .iter()
        .any(|decision| decision.action == "block_export");
    let summary = RedactionSummary {
        decision_count: decisions.len(),
        finding_count: findings.len(),
        redacted_count: decisions
            .iter()
            .filter(|decision| matches!(decision.action, "redact_value" | "redact_field"))
            .count(),
        omitted_count: decisions
            .iter()
            .filter(|decision| decision.action == "omit_field")
            .count(),
        warning_count: findings
            .iter()
            .filter(|finding| finding.severity == "WARNING")
            .count(),
        blocking_count: findings
            .iter()
            .filter(|finding| finding.severity == "BLOCKING")
            .count(),
        cloud_transmission_attempted: false,
        professional_claims_made: false,
    };
    ControlledExport {
        payload: if blocked { None } else { Some(stripped) },
        decisions,
        findings,
        blocked,
        summary,
    }
}

fn strip_intent(value: Value) -> Value {
    match value {
        Value::Object(object) => {
            let mut output = Map::new();
            for (key, item) in object {
                if INTENT_KEYS.contains(&key.as_str()) {
                    continue;
                }
                if key == "export_policy" {
                    if let Value::Object(policy) = item {
                        output.insert(
                            key,
                            Value::Object(
                                policy
                                    .into_iter()
                                    .filter(|(policy_key, _)| {
                                        !INTENT_KEYS.contains(&policy_key.as_str())
                                    })
                                    .map(|(policy_key, policy_value)| {
                                        (policy_key, strip_intent(policy_value))
                                    })
                                    .collect(),
                            ),
                        );
                        continue;
                    }
                }
                output.insert(key, strip_intent(item));
            }
            Value::Object(output)
        }
        Value::Array(items) => Value::Array(items.into_iter().map(strip_intent).collect()),
        scalar => scalar,
    }
}

fn walk(
    value: &Value,
    path: &str,
    explicit_intent: bool,
    decisions: &mut Vec<RedactionDecision>,
    findings: &mut Vec<RedactionFinding>,
) {
    if path == "$.report_package" {
        let (action, reason, severity) = if explicit_intent {
            (
                "warning_only",
                "PRIVATE_LOCAL_ALLOWED_WITH_WARNING",
                "WARNING",
            )
        } else {
            ("block_export", "LOCAL_PRIVATE_INTENT_REQUIRED", "BLOCKING")
        };
        decisions.push(RedactionDecision {
            decision_id: format!("REDC-DEC-{:04}", decisions.len() + 1),
            path: path.to_string(),
            field_class: "report_package".to_string(),
            privacy_classification: "private_project_data".to_string(),
            redistribution_status: "private_only".to_string(),
            review_status: "accepted".to_string(),
            export_context: "local_private",
            action,
            reason_code: reason,
            source_metadata_present: true,
        });
        findings.push(RedactionFinding {
            finding_id: format!("REDC-FND-{:04}", findings.len() + 1),
            code: reason,
            class: "PRIVATE_DATA_WARNING",
            severity,
            path: path.to_string(),
            action,
            message: if action == "block_export" {
                "Export is blocked until wrapper-owned local-private intent is explicit."
            } else {
                "Export retained the value in a local/private context with an explicit warning."
            },
            remediation:
                "Use the runner's explicit local-private intent flag for known private values.",
        });
        return;
    }

    match value {
        Value::Object(object) => {
            for (key, item) in object {
                walk(
                    item,
                    &format!("{path}.{key}"),
                    explicit_intent,
                    decisions,
                    findings,
                );
            }
        }
        Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                walk(
                    item,
                    &format!("{path}[{index}]"),
                    explicit_intent,
                    decisions,
                    findings,
                );
            }
        }
        scalar => {
            let lowered = path.to_ascii_lowercase().replace('-', "_");
            let known_private = [
                "mechanics_envelope",
                "model_ref",
                "project_ref",
                "result_refs",
                "runner_result",
                "source_location",
                "suite_run",
                ".value",
            ]
            .iter()
            .any(|token| lowered.contains(token));
            let unknown = scalar.is_null()
                || scalar
                    .as_str()
                    .map(|value| {
                        matches!(
                            value.trim().to_ascii_lowercase().as_str(),
                            "tbd" | "unknown"
                        )
                    })
                    .unwrap_or(false)
                || lowered.contains("tbd");
            let (privacy, redistribution, review, action, reason) = if unknown {
                (
                    "unknown",
                    "unknown",
                    "pending",
                    "warning_only",
                    "UNKNOWN_PROVENANCE_WARNING",
                )
            } else if known_private {
                if explicit_intent {
                    (
                        "private_project_data",
                        "private_only",
                        "accepted",
                        "warning_only",
                        "PRIVATE_LOCAL_ALLOWED_WITH_WARNING",
                    )
                } else {
                    (
                        "private_project_data",
                        "private_only",
                        "accepted",
                        "block_export",
                        "LOCAL_PRIVATE_INTENT_REQUIRED",
                    )
                }
            } else {
                (
                    "public_metadata",
                    "public_permissive",
                    "accepted",
                    "include",
                    "SAFE_PUBLIC_METADATA",
                )
            };
            let decision = RedactionDecision {
                decision_id: format!("REDC-DEC-{:04}", decisions.len() + 1),
                path: path.to_string(),
                field_class: path
                    .rsplit('.')
                    .next()
                    .unwrap_or("unknown")
                    .split('[')
                    .next()
                    .unwrap_or("unknown")
                    .to_string(),
                privacy_classification: privacy.to_string(),
                redistribution_status: redistribution.to_string(),
                review_status: review.to_string(),
                export_context: "local_private",
                action,
                reason_code: reason,
                source_metadata_present: !unknown,
            };
            if action != "include" {
                findings.push(RedactionFinding {
                    finding_id: format!("REDC-FND-{:04}", findings.len() + 1),
                    code: reason,
                    class: if known_private && !unknown {
                        "PRIVATE_DATA_WARNING"
                    } else {
                        "PROVENANCE_WARNING"
                    },
                    severity: if action == "block_export" {
                        "BLOCKING"
                    } else {
                        "WARNING"
                    },
                    path: path.to_string(),
                    action,
                    message: if action == "block_export" {
                        "Export is blocked until wrapper-owned local-private intent is explicit."
                    } else {
                        "Export retained the value in a local/private context with an explicit warning."
                    },
                    remediation: "Use the runner's explicit local-private intent flag for known private values.",
                });
            }
            decisions.push(decision);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn source_intent_is_not_authoritative() {
        let controlled = control_local_private(
            json!({
                "project_ref": "invented-private-project",
                "local_private_intent": true,
                "export_policy": {"explicit_local_private_intent": true}
            }),
            false,
        );
        assert!(controlled.blocked);
        assert_eq!(controlled.payload, None);
        assert!(controlled
            .findings
            .iter()
            .any(|finding| finding.code == "LOCAL_PRIVATE_INTENT_REQUIRED"));
    }

    #[test]
    fn wrapper_intent_retains_known_private_with_warning() {
        let controlled = control_local_private(json!({"project_ref": "invented"}), true);
        assert!(!controlled.blocked);
        assert_eq!(controlled.payload, Some(json!({"project_ref": "invented"})));
        assert!(controlled
            .findings
            .iter()
            .any(|finding| finding.code == "PRIVATE_LOCAL_ALLOWED_WITH_WARNING"));
    }

    #[test]
    fn report_package_is_classified_once_without_scalar_descent_at_all_sizes() {
        for size in [0, 1, 4096, 3_189_621] {
            let bytes = vec![42_u8; size];
            let payload = json!({"report_package": {"container_bytes": bytes}});
            let controlled = control_local_private(payload.clone(), true);
            assert!(!controlled.blocked);
            assert_eq!(controlled.payload, Some(payload));
            assert_eq!(
                controlled
                    .decisions
                    .iter()
                    .filter(|decision| decision.path == "$.report_package")
                    .count(),
                1
            );
            assert_eq!(
                controlled
                    .findings
                    .iter()
                    .filter(|finding| finding.path == "$.report_package")
                    .count(),
                1
            );
        }
    }

    #[test]
    fn report_package_without_intent_blocks_once_and_withholds_payload() {
        let controlled = control_local_private(
            json!({"report_package": {"container_bytes": vec![7_u8; 3_189_621]}}),
            false,
        );
        assert!(controlled.blocked);
        assert_eq!(controlled.payload, None);
        assert_eq!(
            controlled
                .decisions
                .iter()
                .filter(|decision| decision.path == "$.report_package")
                .count(),
            1
        );
        assert_eq!(
            controlled
                .findings
                .iter()
                .filter(|finding| finding.path == "$.report_package")
                .count(),
            1
        );
    }

    #[test]
    fn unknown_local_value_stays_unknown_and_warning_only() {
        let controlled = control_local_private(json!({"open_tbd": "TBD"}), false);
        assert!(!controlled.blocked);
        assert_eq!(controlled.decisions[0].privacy_classification, "unknown");
        assert_eq!(controlled.decisions[0].action, "warning_only");
    }

    #[test]
    fn every_private_named_runner_field_preserves_null_tbd_and_unknown_as_warning_only() {
        for field in ["mechanics_envelope", "runner_result", "suite_run"] {
            for value in [Value::Null, json!("TBD"), json!("unknown")] {
                let controlled = control_local_private(
                    Value::Object(Map::from_iter([(field.to_string(), value)])),
                    false,
                );
                assert!(!controlled.blocked, "{field} must not block when unknown");
                assert_eq!(
                    controlled.payload.is_some(),
                    true,
                    "{field} payload retained"
                );
                assert_eq!(controlled.decisions.len(), 1);
                assert_eq!(controlled.decisions[0].privacy_classification, "unknown");
                assert_eq!(controlled.decisions[0].redistribution_status, "unknown");
                assert_eq!(controlled.decisions[0].review_status, "pending");
                assert_eq!(controlled.decisions[0].action, "warning_only");
                assert_eq!(
                    controlled.decisions[0].reason_code,
                    "UNKNOWN_PROVENANCE_WARNING"
                );
                assert_eq!(controlled.findings[0].class, "PROVENANCE_WARNING");
            }
        }
    }

    #[test]
    fn canonical_parity_corpus_matches_rust_contract_mirror() {
        let corpus: Value = serde_json::from_str(include_str!(
            "../../../../fixtures/redaction_export_controls/cases.json"
        ))
        .expect("shared redaction corpus must remain valid JSON");
        for case in corpus["cases"].as_array().expect("cases array") {
            let case_id = case["case_id"].as_str().expect("case id");
            let item = case["item"].as_object().expect("item object");
            let context = case["export_context"].as_str().expect("context");
            let explicit_intent = case["explicit_local_private_intent"]
                .as_bool()
                .expect("intent bool");
            // Only the user-release cases carry the member; every other case is
            // decided without release intent, as before.
            let release_intent = match case.get("explicit_user_release_intent") {
                None => false,
                Some(value) => value.as_bool().expect("release intent bool"),
            };
            let expected = case["expected"].as_object().expect("expected object");
            let actual = classify_contract_item(item, context, explicit_intent, release_intent);
            assert_eq!(
                actual.action,
                expected["action"].as_str().expect("expected action"),
                "action mismatch for {case_id}"
            );
            assert_eq!(
                actual.reason_code,
                expected["reason_code"].as_str().expect("expected reason"),
                "reason mismatch for {case_id}"
            );
            assert_eq!(
                actual.source_metadata_present,
                expected["source_metadata_present"]
                    .as_bool()
                    .expect("expected metadata flag"),
                "metadata mismatch for {case_id}"
            );
            if expected["finding"].is_null() {
                assert_eq!(actual.finding_class, None, "finding mismatch for {case_id}");
            } else {
                assert_eq!(
                    actual.finding_class,
                    expected["finding"]["class"].as_str(),
                    "finding class mismatch for {case_id}"
                );
                assert_eq!(
                    actual.finding_severity,
                    expected["finding"]["severity"].as_str(),
                    "finding severity mismatch for {case_id}"
                );
            }
        }
    }

    /// The DEL-12-02 input-contract sweep, expanded by the same rule as
    /// `tests/security/test_redaction_input_contract_sweep.py` and
    /// `redactionInputContractSweep.test.ts`: items in family order labels, flags, claims,
    /// alias_items, each decided in the spec's twelve contexts in order; then the
    /// alias_projector records, each decided in the projector's ten contexts in order; then
    /// the proto_items and the proto_projector records in the same way.
    const SWEEP_CASE_COUNT: usize = 100_744;

    struct SweepCase {
        family: &'static str,
        item: Map<String, Value>,
        context: String,
        local_intent: bool,
        release_intent: bool,
        /// The leaf of an `alias_projector` or `proto_projector` record, decided as the route
        /// projector labels it.
        leaf_key: Option<String>,
    }

    fn sweep_json(text: &str) -> Value {
        serde_json::from_str(text).expect("sweep fixture must remain valid JSON")
    }

    fn expand_sweep(spec: &Value) -> Vec<SweepCase> {
        let absent = spec["absent"].as_str().expect("absent marker");
        let put = |target: &mut Map<String, Value>, key: &str, value: &Value| {
            if value.as_str() != Some(absent) {
                target.insert(key.to_string(), value.clone());
            }
        };
        let list = |value: &Value| value.as_array().expect("sweep list").clone();
        let mut items: Vec<(&'static str, Map<String, Value>)> = Vec::new();
        let labels = &spec["labels"];
        for privacy in list(&labels["privacy"]) {
            for redistribution in list(&labels["redistribution"]) {
                for review in list(&labels["review"]) {
                    for placement in list(&labels["placements"]) {
                        let mut item = Map::from_iter([("value".to_string(), json!("x"))]);
                        match placement.as_str().expect("placement") {
                            "item" => {
                                put(&mut item, "privacy_classification", &privacy);
                                put(&mut item, "redistribution_status", &redistribution);
                                put(&mut item, "review_status", &review);
                            }
                            "provenance" => {
                                let mut provenance = Map::new();
                                put(&mut provenance, "privacy_classification", &privacy);
                                put(&mut provenance, "redistribution_status", &redistribution);
                                put(&mut provenance, "review_status", &review);
                                item.insert("provenance".to_string(), Value::Object(provenance));
                            }
                            _ => {
                                // split: the privacy and review on the item, the redistribution
                                // on a public provenance whose review is quarantined only with
                                // the item's.
                                put(&mut item, "privacy_classification", &privacy);
                                let quarantined = review.as_str() == Some("quarantined");
                                let mut provenance = Map::from_iter([
                                    ("privacy_classification".to_string(), json!("public")),
                                    (
                                        "redistribution_status".to_string(),
                                        json!("public_permissive"),
                                    ),
                                    (
                                        "review_status".to_string(),
                                        json!(if quarantined {
                                            "quarantined"
                                        } else {
                                            "accepted"
                                        }),
                                    ),
                                ]);
                                put(&mut provenance, "redistribution_status", &redistribution);
                                item.insert("provenance".to_string(), Value::Object(provenance));
                                put(&mut item, "review_status", &review);
                            }
                        }
                        items.push(("labels", item));
                    }
                }
            }
        }
        let bases = list(&spec["bases"]);
        for key in list(&spec["flags"]["keys"]) {
            let key = key.as_str().expect("flag key");
            for value in list(&spec["flags"]["values"]) {
                for base in &bases {
                    let mut item = Map::from_iter([("value".to_string(), json!("x"))]);
                    item.extend(base.as_object().expect("base object").clone());
                    match key.strip_prefix("export_policy.") {
                        Some(policy_key) => {
                            item.insert(
                                "export_policy".to_string(),
                                Value::Object(Map::from_iter([(
                                    policy_key.to_string(),
                                    value.clone(),
                                )])),
                            );
                        }
                        None => {
                            item.insert(key.to_string(), value.clone());
                        }
                    }
                    items.push(("flags", item));
                }
            }
        }
        for value in list(&spec["claims"]["values"]) {
            for base in &bases {
                let mut item = Map::from_iter([("value".to_string(), json!("x"))]);
                item.extend(base.as_object().expect("base object").clone());
                put(&mut item, "professional_claim", &value);
                items.push(("claims", item));
            }
        }
        let aliases = &spec["aliases"];
        let alias_labels = |privacy: &Value, alias: &Value, redistribution: &Value| {
            let mut labels = Map::new();
            put(&mut labels, "privacy_classification", privacy);
            put(&mut labels, "classification", alias);
            put(&mut labels, "redistribution_status", redistribution);
            labels
        };
        let mut alias_grid = Vec::new();
        for privacy in list(&aliases["privacy_classification"]) {
            for alias in list(&aliases["classification"]) {
                for redistribution in list(&aliases["redistribution"]) {
                    alias_grid.push((privacy.clone(), alias.clone(), redistribution));
                }
            }
        }
        for (privacy, alias, redistribution) in &alias_grid {
            for placement in list(&aliases["item_placements"]) {
                let labels = alias_labels(privacy, alias, redistribution);
                let mut item = Map::from_iter([("value".to_string(), json!("x"))]);
                match placement.as_str().expect("item placement") {
                    "provenance" => {
                        item.insert("provenance".to_string(), Value::Object(labels));
                    }
                    other => {
                        item.extend(labels);
                        if other == "item_private_provenance" {
                            item.insert(
                                "provenance".to_string(),
                                aliases["private_provenance"].clone(),
                            );
                        }
                    }
                }
                items.push(("alias_items", item));
            }
        }
        let contexts = list(&spec["contexts"]);
        let mut cases = Vec::with_capacity(items.len() * contexts.len());
        for (family, item) in items {
            for context in &contexts {
                cases.push(SweepCase {
                    family,
                    item: item.clone(),
                    context: context[0].as_str().expect("context").to_string(),
                    local_intent: context[1].as_bool().expect("local intent"),
                    release_intent: context[2].as_bool().expect("release intent"),
                    leaf_key: None,
                });
            }
        }
        let projector = &aliases["projector"];
        let leaf_key = projector["leaf_key"].as_str().expect("leaf key");
        for (privacy, alias, redistribution) in &alias_grid {
            for placement in list(&projector["placements"]) {
                let mut record = alias_labels(privacy, alias, redistribution);
                if placement.as_str() == Some("record_public_provenance") {
                    record.insert(
                        "provenance".to_string(),
                        projector["public_provenance"].clone(),
                    );
                }
                record.insert(leaf_key.to_string(), projector["leaf_value"].clone());
                for context in list(&projector["contexts"]) {
                    cases.push(SweepCase {
                        family: "alias_projector",
                        item: record.clone(),
                        context: context[0].as_str().expect("context").to_string(),
                        local_intent: context[1].as_bool().expect("local intent"),
                        release_intent: false,
                        leaf_key: Some(leaf_key.to_string()),
                    });
                }
            }
        }
        // A `__proto__` key is an ordinary key of a serde_json map, as of a JSON object.
        let proto = &spec["proto"];
        let member = proto["member"].as_str().expect("proto member");
        let mut proto_grid = Vec::new();
        for privacy in list(&proto["privacy_classification"]) {
            for alias in list(&proto["classification"]) {
                for redistribution in list(&proto["redistribution"]) {
                    for member_value in list(&proto["member_values"]) {
                        proto_grid.push((
                            privacy.clone(),
                            alias.clone(),
                            redistribution.clone(),
                            member_value,
                        ));
                    }
                }
            }
        }
        for (privacy, alias, redistribution, member_value) in &proto_grid {
            for placement in list(&proto["item_placements"]) {
                let mut labels = alias_labels(privacy, alias, redistribution);
                labels.insert(member.to_string(), member_value.clone());
                let mut item = Map::from_iter([("value".to_string(), json!("x"))]);
                if placement.as_str() == Some("provenance") {
                    item.insert("provenance".to_string(), Value::Object(labels));
                } else {
                    item.extend(labels);
                }
                for context in &contexts {
                    cases.push(SweepCase {
                        family: "proto_items",
                        item: item.clone(),
                        context: context[0].as_str().expect("context").to_string(),
                        local_intent: context[1].as_bool().expect("local intent"),
                        release_intent: context[2].as_bool().expect("release intent"),
                        leaf_key: None,
                    });
                }
            }
        }
        for (privacy, alias, redistribution, member_value) in &proto_grid {
            for placement in list(&proto["projector_placements"]) {
                let mut record = alias_labels(privacy, alias, redistribution);
                let mut public_provenance = projector["public_provenance"]
                    .as_object()
                    .expect("public provenance")
                    .clone();
                match placement.as_str().expect("projector placement") {
                    "provenance" => {
                        public_provenance.insert(member.to_string(), member_value.clone());
                        record.insert("provenance".to_string(), Value::Object(public_provenance));
                    }
                    other => {
                        record.insert(member.to_string(), member_value.clone());
                        if other == "record_public_provenance" {
                            record
                                .insert("provenance".to_string(), Value::Object(public_provenance));
                        }
                    }
                }
                record.insert(leaf_key.to_string(), projector["leaf_value"].clone());
                for context in list(&projector["contexts"]) {
                    cases.push(SweepCase {
                        family: "proto_projector",
                        item: record.clone(),
                        context: context[0].as_str().expect("context").to_string(),
                        local_intent: context[1].as_bool().expect("local intent"),
                        release_intent: false,
                        leaf_key: Some(leaf_key.to_string()),
                    });
                }
            }
        }
        cases
    }

    fn sweep_decision(case: &SweepCase) -> String {
        let decided = match &case.leaf_key {
            // The route projector's decision on the record's one leaf.
            Some(key) => classify_contract_item(
                &route_basis_leaf(&case.item, key),
                &case.context,
                case.local_intent,
                false,
            ),
            None => classify_contract_item(
                &case.item,
                &case.context,
                case.local_intent,
                case.release_intent,
            ),
        };
        format!("{}/{}", decided.action, decided.reason_code)
    }

    fn frozen_column(frozen: &Value, name: &str) -> Vec<String> {
        let alphabet: Vec<char> = frozen["alphabet"]
            .as_str()
            .expect("alphabet")
            .chars()
            .collect();
        let codes = frozen["codes"].as_array().expect("codes");
        frozen["decisions"][name]
            .as_array()
            .expect("decision column")
            .iter()
            .flat_map(|line| {
                line.as_str()
                    .expect("decision line")
                    .chars()
                    .collect::<Vec<_>>()
            })
            .map(|symbol| {
                let index = alphabet
                    .iter()
                    .position(|known| *known == symbol)
                    .expect("symbol");
                codes[index].as_str().expect("code").to_string()
            })
            .collect()
    }

    /// "Less strict" is this total order on the classifier's actions, least to most strict.
    /// `redact_value` sits above `warning_only` by the existing contracts: `warning_only`
    /// keeps the item with its value and is the unknown-metadata action only in
    /// `local_private`, while `redact_value` replaces `value` and `text` and is that action
    /// in every public or shared context.  The classifier produces no `redact_field` or
    /// `omit_field`.
    const ACTION_ORDER: [&str; 4] = ["include", "warning_only", "redact_value", "block_export"];
    const BEFORE_COLUMNS: [&str; 3] = ["python_before", "typescript_before", "rust_before"];
    const RAISED: &str = "raised/TypeError";
    /// The Rust mirror had no route projector before the alias decision, so its frozen
    /// column holds no decision for an alias_projector case.
    const NOT_MIRRORED: &str = "not_mirrored/NO_ROUTE_PROJECTOR";
    /// The TypeScript projector before the decision that no JSON member becomes a prototype
    /// (HELP_HUMAN, 2026-10-27: "Parse so that no JSON member can become a prototype") copied
    /// a record by assignment, so a `__proto__` member became the copy's prototype, or was
    /// dropped when it was not an object.  Its proto_projector decisions are of a different
    /// record than the input, so those cases are held to the Python column, the reference
    /// whose reading of the member as an ordinary own member all three now share (this
    /// mirror's column is `not_mirrored`).
    const PROTOTYPE_COPY_COLUMN: (&str, &str) = ("proto_projector", "typescript_before");

    fn held_to(family: &str, name: &str, code: &str) -> bool {
        code != RAISED && code != NOT_MIRRORED && (family, name) != PROTOTYPE_COPY_COLUMN
    }

    fn strictness(code: &str) -> usize {
        let action = code.split('/').next().expect("action");
        ACTION_ORDER
            .iter()
            .position(|known| *known == action)
            .unwrap_or_else(|| panic!("unordered action in {code}"))
    }

    fn sweep_fixtures() -> (Vec<SweepCase>, Value) {
        let spec = sweep_json(include_str!(
            "../../../../fixtures/redaction_export_controls/input_contract_sweep.json"
        ));
        let frozen = sweep_json(include_str!(
            "../../../../fixtures/redaction_export_controls/input_contract_sweep_decisions.json"
        ));
        let cases = expand_sweep(&spec);
        assert_eq!(cases.len(), SWEEP_CASE_COUNT);
        assert_eq!(frozen["case_count"].as_u64(), Some(SWEEP_CASE_COUNT as u64));
        assert_eq!(frozen["action_order"], json!(ACTION_ORDER));
        assert!(cases.iter().all(|case| matches!(
            case.family,
            "labels"
                | "flags"
                | "claims"
                | "alias_items"
                | "alias_projector"
                | "proto_items"
                | "proto_projector"
        )));
        (cases, frozen)
    }

    /// The ruling's proof obligation: every sweep case's decision is at least as strict as
    /// each implementation's frozen pre-contract decision.  Python's former TypeError on an
    /// array or object label is no decision; the case is held to the TypeScript and Rust
    /// columns.  Nor is this mirror's `not_mirrored` on an alias_projector or proto_projector
    /// case; it is held to the Python and TypeScript columns.  A proto_projector case is not
    /// held to the TypeScript column (`PROTOTYPE_COPY_COLUMN`).
    #[test]
    fn sweep_decides_no_case_less_strictly_than_any_frozen_pre_contract_decision() {
        let (cases, frozen) = sweep_fixtures();
        let before: Vec<(&str, Vec<String>)> = BEFORE_COLUMNS
            .iter()
            .map(|name| (*name, frozen_column(&frozen, name)))
            .collect();
        // Each column's count is asserted separately.
        let mut looser: Vec<(&str, Vec<usize>)> = BEFORE_COLUMNS
            .iter()
            .map(|name| (*name, Vec::new()))
            .collect();
        for (index, case) in cases.iter().enumerate() {
            let decided = sweep_decision(case);
            for ((name, column), (_, found)) in before.iter().zip(looser.iter_mut()) {
                assert_eq!(column.len(), SWEEP_CASE_COUNT, "{name}");
                if held_to(case.family, name, &column[index])
                    && strictness(&decided) < strictness(&column[index])
                {
                    found.push(index);
                }
            }
        }
        for (name, found) in &looser {
            assert!(
                found.is_empty(),
                "{name}: {} looser, first {:?}",
                found.len(),
                found.first()
            );
        }
        assert_eq!(
            looser.iter().map(|(name, _)| *name).collect::<Vec<_>>(),
            ["python_before", "typescript_before", "rust_before"]
        );
    }

    /// The TypeScript column is set aside only for the appended proto_projector cases, each
    /// still held to the Python column.  Under the fail-closed rule (HELP_HUMAN ruling,
    /// 2026-10-27, option (b)) the contract is no less strict than the set-aside column
    /// either: stricter in 1,920 cases (among them the 780 where a member's public labels,
    /// inherited by the copy, lifted a sibling into a public basis, and the 120 where a
    /// member's restrictive privacy, inherited by the copy, hid the record's own
    /// `classification` alias, which redact again) and less strict in none.  Against Python's
    /// frozen decisions the contract is stricter in 1,932 `__proto__` cases, by design.
    #[test]
    fn sweep_sets_the_prototype_copy_column_aside_only_for_proto_projector_cases() {
        let (cases, frozen) = sweep_fixtures();
        let contract = frozen_column(&frozen, "contract");
        let python = frozen_column(&frozen, "python_before");
        let typescript = frozen_column(&frozen, "typescript_before");
        let set_aside: Vec<usize> = (0..SWEEP_CASE_COUNT)
            .filter(|index| cases[*index].family == "proto_projector")
            .collect();
        assert_eq!(set_aside.len(), 10_080);
        assert!(set_aside[0] >= 82_600);
        assert!(set_aside
            .iter()
            .all(|index| python[*index] != RAISED && python[*index] != NOT_MIRRORED));
        let count = |looser: bool| {
            set_aside
                .iter()
                .filter(|index| {
                    let (before, after) = (
                        strictness(&typescript[**index]),
                        strictness(&contract[**index]),
                    );
                    if looser {
                        before < after
                    } else {
                        before > after
                    }
                })
                .count()
        };
        assert_eq!((count(true), count(false)), (1_920, 0));
        let proto_cases: Vec<usize> = (0..SWEEP_CASE_COUNT)
            .filter(|index| matches!(cases[*index].family, "proto_items" | "proto_projector"))
            .collect();
        assert_eq!((proto_cases.len(), proto_cases[0]), (18_144, 82_600));
        assert_eq!(
            proto_cases
                .iter()
                .filter(|index| strictness(&contract[**index]) > strictness(&python[**index]))
                .count(),
            1_932
        );
    }

    /// With the pytest and vitest twins, the three implementations agree case for case.
    #[test]
    fn sweep_gives_the_contract_decision_for_every_case() {
        let (cases, frozen) = sweep_fixtures();
        let contract = frozen_column(&frozen, "contract");
        assert_eq!(contract.len(), SWEEP_CASE_COUNT);
        let mismatches: Vec<usize> = (0..SWEEP_CASE_COUNT)
            .filter(|index| sweep_decision(&cases[*index]) != contract[*index])
            .collect();
        assert!(
            mismatches.is_empty(),
            "{} mismatches, first {:?}",
            mismatches.len(),
            &mismatches[..mismatches.len().min(5)]
        );
    }

    /// Named regressions (HELP_HUMAN ruling 2026-10-27).  The per-row cases of the (a'),
    /// (b') and (c) tables are the `input-contract-*` cases of the shared corpus, decided by
    /// `canonical_parity_corpus_matches_rust_contract_mirror` and its two twins.
    #[test]
    fn input_contract_corpus_cases_are_present_and_outside_the_pinned_original_cases() {
        let corpus: Value = serde_json::from_str(include_str!(
            "../../../../fixtures/redaction_export_controls/cases.json"
        ))
        .expect("shared redaction corpus must remain valid JSON");
        let cases = corpus["cases"].as_array().expect("cases array");
        let added: Vec<&Value> = cases
            .iter()
            .filter(|case| {
                case["case_id"]
                    .as_str()
                    .is_some_and(|id| id.starts_with("input-contract-"))
            })
            .collect();
        assert_eq!(added.len(), 49);
        assert!(added
            .iter()
            .all(|case| case.get("explicit_user_release_intent").is_some()));
        assert!(cases[cases.len() - 49..].iter().eq(added.into_iter()));
    }

    /// HELP_HUMAN decision, 2026-10-27: "A null primary label must not let an alias lift a
    /// sibling into `public_report`."  The TypeScript projector read the `classification`
    /// alias with `??` and so included `note`; Python reads it only when
    /// `privacy_classification` is absent and redacts it, as all three now do.
    #[test]
    fn route_projector_reads_no_classification_alias_under_a_null_primary_label() {
        let decide = |record: &Value| {
            let record = record.as_object().expect("record");
            let decided = classify_contract_item(
                &route_basis_leaf(record, "note"),
                "public_report",
                false,
                false,
            );
            (decided.action, decided.reason_code)
        };
        let mut record = json!({
            "privacy_classification": null,
            "classification": "public",
            "redistribution_status": "public_permissive",
            "note": "INVENTED"
        });
        assert!(!route_record_public_basis(
            record.as_object().expect("record")
        ));
        assert_eq!(
            decide(&record),
            ("redact_value", "REDISTRIBUTION_STATUS_UNKNOWN")
        );
        // The alias is still the label when the primary member is absent.
        record
            .as_object_mut()
            .expect("record")
            .remove("privacy_classification");
        assert!(route_record_public_basis(
            record.as_object().expect("record")
        ));
        assert_eq!(decide(&record), ("include", "SAFE_PUBLIC_METADATA"));
    }

    /// A JSON `__proto__` member is an own member (HELP_HUMAN decision, 2026-10-27: "Parse so
    /// that no JSON member can become a prototype"), and it fails closed on the key itself
    /// (HELP_HUMAN ruling, 2026-10-27: "option (b), fail closed on the key itself, in all
    /// three implementations. A record carrying an own `__proto__` member gets no direct
    /// public basis, and its siblings are labelled by path. The item classifier treats the
    /// member as an unknown label.").  One class per kind of member label in the code
    /// review's 180 cases and their converse, as the vitest and pytest twins hold them, with
    /// the member on the record and in its provenance, in the five projector contexts with
    /// and without intent.  This mirror decides the record's basis and the leaf's action; it
    /// does not materialize a projected payload, which the twins assert.
    fn proto_member_classes() -> Vec<(&'static str, Value, Value)> {
        vec![
            (
                "member_unknown_privacy",
                json!({"privacy_classification": "unknown"}),
                json!({"classification": "public", "redistribution_status": "public_permissive"}),
            ),
            (
                "member_private_privacy",
                json!({"privacy_classification": "private_project_data"}),
                json!({"classification": "public", "redistribution_status": "public_permissive"}),
            ),
            (
                "member_private_labels",
                json!({"privacy_classification": "private_project_data", "redistribution_status": "private_only"}),
                json!({"classification": "public", "redistribution_status": "public_permissive"}),
            ),
            (
                "member_private_redistribution",
                json!({"redistribution_status": "private_only"}),
                json!({"privacy_classification": "public", "redistribution_status": "public_permissive"}),
            ),
            (
                "member_public_labels",
                json!({"privacy_classification": "public", "redistribution_status": "public_permissive"}),
                json!({}),
            ),
            (
                "member_public_redistribution",
                json!({"redistribution_status": "public_permissive"}),
                json!({"privacy_classification": "public"}),
            ),
            (
                "member_public_privacy",
                json!({"privacy_classification": "public"}),
                json!({"redistribution_status": "public_permissive"}),
            ),
            (
                "member_private_privacy_public_redistribution",
                json!({"privacy_classification": "private_project_data", "redistribution_status": "public_permissive"}),
                json!({"classification": "public"}),
            ),
        ]
    }

    fn with_member(labels: &Value, member_key: &str, member: &Value) -> Map<String, Value> {
        let mut object = labels.as_object().expect("labels").clone();
        object.insert(member_key.to_string(), member.clone());
        object
    }

    /// The class's record with the member on it and with the member in its provenance.
    fn proto_records(member: &Value, labels: &Value) -> Vec<(&'static str, Value)> {
        let public = json!({"privacy_classification": "public", "redistribution_status": "public_permissive"});
        let mut on_record = with_member(labels, "__proto__", member);
        on_record.insert("note".to_string(), json!("INVENTED"));
        let mut in_provenance = labels.as_object().expect("labels").clone();
        in_provenance.insert(
            "provenance".to_string(),
            Value::Object(with_member(&public, "__proto__", member)),
        );
        in_provenance.insert("note".to_string(), json!("INVENTED"));
        vec![
            ("record", Value::Object(on_record)),
            ("provenance", Value::Object(in_provenance)),
        ]
    }

    fn proto_note_decision(
        record: &Value,
        context: &str,
        intent: bool,
    ) -> (&'static str, &'static str) {
        let decided = classify_contract_item(
            &route_basis_leaf(record.as_object().expect("record"), "note"),
            context,
            intent,
            false,
        );
        (decided.action, decided.reason_code)
    }

    /// A path-labelled sibling of a record with no public basis.
    fn unbased(context: &str) -> (&'static str, &'static str) {
        if context == "local_private" {
            ("warning_only", "REDISTRIBUTION_STATUS_UNKNOWN")
        } else {
            ("redact_value", "REDISTRIBUTION_STATUS_UNKNOWN")
        }
    }

    /// The classifier's decision with the safe-and-public pairing barred: inclusion becomes
    /// the unknown-label decision; every other decision stands.
    fn barred(
        control: (&'static str, &'static str),
        context: &str,
    ) -> (&'static str, &'static str) {
        match (control.0, context) {
            ("include", USER_RELEASED_REPORT_CONTEXT) => {
                ("block_export", "USER_RELEASE_VALUE_UNBOUND")
            }
            ("include", "local_private") => ("warning_only", "UNKNOWN_PROVENANCE_WARNING"),
            ("include", _) => ("redact_value", "UNKNOWN_PROVENANCE_WARNING"),
            _ => control,
        }
    }

    /// The code review's repro, parsed from JSON text.  The record carries an own
    /// `__proto__` member, so it has no direct public basis and `note` is redacted, as base
    /// TypeScript decided and Python did not.
    #[test]
    fn route_projector_fails_closed_on_a_json_proto_member() {
        let payload: Value = serde_json::from_str(
            r#"{"record":{"__proto__":{"privacy_classification":"private_project_data"},"classification":"public","redistribution_status":"public_permissive","note":"INVENTED"}}"#,
        )
        .expect("repro JSON");
        let record = payload["record"].as_object().expect("record");
        assert_eq!(
            record.get("__proto__"),
            Some(&json!({"privacy_classification": "private_project_data"}))
        );
        assert!(carries_proto_member(record));
        assert!(!route_record_public_basis(record));
        assert_eq!(
            proto_note_decision(&payload["record"], "public_report", false),
            ("redact_value", "REDISTRIBUTION_STATUS_UNKNOWN")
        );
    }

    #[test]
    fn route_projector_proto_member_classes_give_no_public_basis() {
        let contexts = [
            "public_report",
            "public_example",
            "shared_model",
            "downstream_tool",
            "local_private",
        ];
        for (name, member, labels) in proto_member_classes() {
            for (placement, record) in proto_records(&member, &labels) {
                assert!(!route_record_public_basis(
                    record.as_object().expect("record")
                ));
                for context in contexts {
                    for intent in [false, true] {
                        assert_eq!(
                            proto_note_decision(&record, context, intent),
                            unbased(context),
                            "{name} {placement} {context} {intent}"
                        );
                    }
                }
            }
        }
    }

    /// On the item or in its provenance, the member bars the safe-and-public pairing and
    /// nothing else, in all twelve sweep contexts.
    #[test]
    fn classifier_proto_member_classes_read_the_member_as_an_unknown_label() {
        let spec = sweep_json(include_str!(
            "../../../../fixtures/redaction_export_controls/input_contract_sweep.json"
        ));
        let contexts = spec["contexts"].as_array().expect("contexts").clone();
        for (name, member, labels) in proto_member_classes() {
            for placement in ["item", "provenance"] {
                let item = |member_key: &str| {
                    let carried = with_member(&labels, member_key, &member);
                    let mut item = Map::from_iter([("value".to_string(), json!("x"))]);
                    if placement == "provenance" {
                        item.insert("provenance".to_string(), Value::Object(carried));
                    } else {
                        item.extend(carried);
                    }
                    item
                };
                for context in &contexts {
                    let export_context = context[0].as_str().expect("context");
                    let decide = |candidate: &Map<String, Value>| {
                        let decided = classify_contract_item(
                            candidate,
                            export_context,
                            context[1].as_bool().expect("local intent"),
                            context[2].as_bool().expect("release intent"),
                        );
                        (decided.action, decided.reason_code)
                    };
                    assert_eq!(
                        decide(&item("__proto__")),
                        barred(decide(&item("proto_member")), export_context),
                        "{name} {placement} {context}"
                    );
                }
            }
        }
    }

    /// One case per member class, on a record and an item whose own labels are public:
    /// Python read the member as an ordinary member (as it reads the member renamed), so the
    /// sibling and the item were included; now both fail closed.
    #[test]
    fn proto_member_classes_are_stricter_than_python_read_them_before_the_ruling() {
        let public = json!({"privacy_classification": "public", "redistribution_status": "public_permissive"});
        for (name, member, _) in proto_member_classes() {
            let record = |member_key: &str| {
                let mut record = with_member(&public, member_key, &member);
                record.insert("note".to_string(), json!("INVENTED"));
                Value::Object(record)
            };
            assert_eq!(
                proto_note_decision(&record("proto_member"), "public_report", false),
                ("include", "SAFE_PUBLIC_METADATA"),
                "{name}"
            );
            assert_eq!(
                proto_note_decision(&record("__proto__"), "public_report", false),
                unbased("public_report"),
                "{name}"
            );
            let item = |member_key: &str| {
                let mut item = with_member(&public, member_key, &member);
                item.insert("value".to_string(), json!("x"));
                let decided = classify_contract_item(&item, "public_report", false, false);
                format!("{}/{}", decided.action, decided.reason_code)
            };
            let (control, barred_item) = (item("proto_member"), item("__proto__"));
            assert_eq!(control, "include/SAFE_PUBLIC_METADATA", "{name}");
            assert_eq!(
                barred_item, "redact_value/UNKNOWN_PROVENANCE_WARNING",
                "{name}"
            );
            assert!(strictness(&barred_item) > strictness(&control));
        }
    }

    /// Codex r2 on #1692: the bar is the last step of the decision order.  Each labels,
    /// flags, claims and alias_items case, decided with an own `__proto__` member added on the
    /// item and, where it has one, in its provenance, gives its own decision with only an
    /// inclusion barred.
    #[test]
    fn proto_member_leaves_every_decision_but_a_final_inclusion_unchanged() {
        let (cases, _) = sweep_fixtures();
        let mut differ = Vec::new();
        for (index, case) in cases.iter().enumerate() {
            if !matches!(case.family, "labels" | "flags" | "claims" | "alias_items") {
                continue;
            }
            let decide = |candidate: &Map<String, Value>| {
                let decided = classify_contract_item(
                    candidate,
                    &case.context,
                    case.local_intent,
                    case.release_intent,
                );
                (decided.action, decided.reason_code)
            };
            let expected = barred(decide(&case.item), &case.context);
            let mut on_item = case.item.clone();
            on_item.insert("__proto__".to_string(), json!({}));
            if decide(&on_item) != expected {
                differ.push((index, "item"));
            }
            if let Some(provenance) = case.item.get("provenance").and_then(Value::as_object) {
                let mut in_provenance = case.item.clone();
                let mut provenance = provenance.clone();
                provenance.insert("__proto__".to_string(), json!({}));
                in_provenance.insert("provenance".to_string(), Value::Object(provenance));
                if decide(&in_provenance) != expected {
                    differ.push((index, "provenance"));
                }
            }
        }
        assert!(
            differ.is_empty(),
            "{} differ, first {:?}",
            differ.len(),
            differ.first()
        );
    }

    /// Codex r2 on #1692: in user_released_report with release intent this gave
    /// `USER_RELEASE_VALUE_UNBOUND`.
    #[test]
    fn proto_member_keeps_the_concrete_path_redaction_in_every_context() {
        let item: Value = serde_json::from_str(
            r#"{"value":"x","privacy_classification":"public","redistribution_status":"public_permissive","concrete_path_present":true,"__proto__":{}}"#,
        )
        .expect("item JSON");
        let item = item.as_object().expect("item").clone();
        let mut renamed = item.clone();
        let member = renamed.remove("__proto__").expect("member");
        renamed.insert("proto_member".to_string(), member);
        let spec = sweep_json(include_str!(
            "../../../../fixtures/redaction_export_controls/input_contract_sweep.json"
        ));
        for context in spec["contexts"].as_array().expect("contexts") {
            for candidate in [&item, &renamed] {
                let decided = classify_contract_item(
                    candidate,
                    context[0].as_str().expect("context"),
                    context[1].as_bool().expect("local intent"),
                    context[2].as_bool().expect("release intent"),
                );
                assert_eq!(
                    (decided.action, decided.reason_code),
                    ("redact_value", "CONCRETE_PATH_REDACTED"),
                    "{context}"
                );
            }
        }
    }

    // Value records (HELP_HUMAN decision, 2026-10-28: "take the stricter decision per
    // case"): the shared sweep `value_record_sweep.json`, expanded by the same rule as the
    // pytest and vitest twins, and one regression per divergence class.
    const VALUE_RECORD_CASE_COUNT: usize = 7_730;

    fn value_record_cases() -> (Value, Vec<(Value, String, bool)>) {
        let spec = sweep_json(include_str!(
            "../../../../fixtures/redaction_export_controls/value_record_sweep.json"
        ));
        let absent = spec["absent"].as_str().expect("absent").to_string();
        let max_axes = spec["max_axes"].as_u64().expect("max_axes") as usize;
        let axes: Vec<(String, Vec<Value>)> = spec["axes"]
            .as_array()
            .expect("axes")
            .iter()
            .map(|axis| {
                (
                    axis[0].as_str().expect("axis key").to_string(),
                    axis[1].as_array().expect("axis values").clone(),
                )
            })
            .collect();
        let mut records = Vec::new();
        // Every combination, the last axis varying fastest, as itertools.product orders it.
        let mut digits = vec![0usize; axes.len()];
        loop {
            let set = axes
                .iter()
                .zip(&digits)
                .filter(|((_, values), digit)| values[**digit].as_str() != Some(absent.as_str()))
                .count();
            if set <= max_axes {
                let mut record = Map::from_iter([("value".to_string(), json!("INVENTED"))]);
                for ((key, values), digit) in axes.iter().zip(&digits) {
                    if values[*digit].as_str() != Some(absent.as_str()) {
                        record.insert(key.clone(), values[*digit].clone());
                    }
                }
                records.push(Value::Object(record));
            }
            let mut position = axes.len();
            let mut exhausted = true;
            while position > 0 {
                position -= 1;
                digits[position] += 1;
                if digits[position] < axes[position].1.len() {
                    exhausted = false;
                    break;
                }
                digits[position] = 0;
            }
            if exhausted {
                break;
            }
        }
        records.extend(
            spec["extra_records"]
                .as_array()
                .expect("extra records")
                .iter()
                .cloned(),
        );
        let mut cases = Vec::new();
        for record in records {
            for context in spec["contexts"].as_array().expect("contexts") {
                cases.push((
                    record.clone(),
                    context[0].as_str().expect("context").to_string(),
                    context[1].as_bool().expect("intent"),
                ));
            }
        }
        (spec, cases)
    }

    /// The sorted decisions with the route key prefix removed from paths, as the frozen
    /// `decision_sets` hold them.  This mirror does not materialize a payload, so it is held
    /// to the contract's decisions case for case; the pytest and vitest twins also hold the
    /// exported payload to the contract and to each before column.
    fn value_record_decision_set(decisions: &[RouteDecision]) -> String {
        let mut normalized: Vec<(String, &str, &str)> = decisions
            .iter()
            .map(|(path, action, reason)| (path.replace("__route_key__", ""), *action, *reason))
            .collect();
        normalized.sort();
        serde_json::to_string(
            &normalized
                .iter()
                .map(|(path, action, reason)| json!([path, action, reason]))
                .collect::<Vec<_>>(),
        )
        .expect("decision set")
    }

    #[test]
    fn value_record_sweep_gives_the_contract_decisions() {
        let (spec, cases) = value_record_cases();
        assert_eq!(cases.len(), VALUE_RECORD_CASE_COUNT);
        let frozen = sweep_json(include_str!(
            "../../../../fixtures/redaction_export_controls/value_record_sweep_decisions.json"
        ));
        assert_eq!(
            frozen["case_count"].as_u64(),
            Some(VALUE_RECORD_CASE_COUNT as u64)
        );
        let record_key = spec["record_key"].as_str().expect("record key");
        let sets = frozen["decision_sets"].as_array().expect("decision sets");
        let contract: Vec<&str> = frozen["columns"]["contract_decisions"]
            .as_array()
            .expect("column")
            .iter()
            .flat_map(|line| line.as_str().expect("line").split(','))
            .map(|index| {
                sets[index.parse::<usize>().expect("index")]
                    .as_str()
                    .expect("set")
            })
            .collect();
        assert_eq!(contract.len(), VALUE_RECORD_CASE_COUNT);
        let mismatches: Vec<usize> = cases
            .iter()
            .enumerate()
            .filter(|(index, (record, context, intent))| {
                value_record_decision_set(&route_record_decisions(
                    record_key, record, context, *intent,
                )) != contract[*index]
            })
            .map(|(index, _)| index)
            .collect();
        assert!(
            mismatches.is_empty(),
            "{} mismatches, first {:?}",
            mismatches.len(),
            mismatches.first()
        );
    }

    fn value_record(json_text: &str, context: &str, intent: bool) -> Vec<RouteDecision> {
        let record: Value = serde_json::from_str(json_text).expect("record JSON");
        let mut decided = route_record_decisions("record", &record, context, intent);
        decided.sort();
        decided
    }

    fn route_decision(path: &str, action: &'static str, reason: &'static str) -> RouteDecision {
        (path.to_string(), action, reason)
    }

    /// Python stricter: TypeScript decided `{value, field_class, note}` as one item and left
    /// `note` visible in `public_report`; it is projected, every leaf redacted.
    #[test]
    fn value_record_with_field_class_and_note_is_projected() {
        let decided = value_record(
            r#"{"value":"INVENTED","field_class":"note","note":"INVENTED"}"#,
            "public_report",
            false,
        );
        assert_eq!(
            decided,
            vec![
                route_decision(
                    "$.record.field_class",
                    "redact_value",
                    "REDISTRIBUTION_STATUS_UNKNOWN"
                ),
                route_decision(
                    "$.record.note",
                    "redact_value",
                    "REDISTRIBUTION_STATUS_UNKNOWN"
                ),
                route_decision("$.record.value", "redact_value", "PRIVATE_DATA_REDACTED"),
            ]
        );
    }

    /// TypeScript stricter: `{value, professional_claim: true}` blocks as one item; Python
    /// redacted its leaves.
    #[test]
    fn value_record_with_a_professional_claim_blocks() {
        assert_eq!(
            value_record(
                r#"{"value":"INVENTED","professional_claim":true}"#,
                "public_report",
                false
            ),
            vec![route_decision(
                "$.record",
                "block_export",
                "PROFESSIONAL_BOUNDARY_BLOCKED"
            )]
        );
    }

    /// Both blocked on different paths (ruling 3(a)): the item's whole-record block.
    #[test]
    fn value_record_blocked_both_ways_takes_the_items_block() {
        assert_eq!(
            value_record(
                r#"{"value":"INVENTED","contains_payload":true}"#,
                "local_private",
                false
            ),
            vec![route_decision(
                "$.record",
                "block_export",
                "PAYLOAD_METADATA_ONLY_REQUIRED"
            )]
        );
    }

    /// Python stricter by a block: the projected `value` leaf needs local-private intent.
    #[test]
    fn value_record_projected_value_leaf_needs_intent() {
        assert_eq!(
            value_record(
                r#"{"value":"INVENTED","export_policy":{}}"#,
                "local_private",
                false
            ),
            vec![route_decision(
                "$.record.value",
                "block_export",
                "LOCAL_PRIVATE_INTENT_REQUIRED"
            )]
        );
    }

    /// The union (the 572 class): the item redacts for the concrete path, so the `value` leaf
    /// is raised to it with the item's own reason (1(a)) and `file_path` is omitted (2(a)).
    #[test]
    fn value_record_union_raises_value_and_omits_the_unsafe_path_key() {
        assert_eq!(
            value_record(
                r#"{"value":"INVENTED","file_path":"INVENTED","export_policy":{}}"#,
                "local_private",
                true
            ),
            vec![
                route_decision("$.record.file_path", "omit_field", "CONCRETE_PATH_REDACTED"),
                route_decision("$.record.value", "redact_value", "CONCRETE_PATH_REDACTED"),
            ]
        );
    }

    /// A tie (same visible data) takes the projection's decision records.
    #[test]
    fn value_record_tie_takes_the_projection_form() {
        assert_eq!(
            value_record(
                r#"{"value":"INVENTED","export_policy":{}}"#,
                "local_private",
                true
            ),
            vec![route_decision(
                "$.record.value",
                "warning_only",
                "PRIVATE_LOCAL_ALLOWED_WITH_WARNING"
            )]
        );
    }

    /// Codex privacy review of #1722, P1-1: the old marker member in an input is not
    /// trusted; the labelled record is a protected item and blocks.
    #[test]
    fn value_record_marker_in_input_is_not_trusted() {
        assert_eq!(
            value_record(
                r#"{"value":"SECRET","privacy_classification":"protected_suspected","_route_value_record":true,"item":{"value":"safe","privacy_classification":"public","redistribution_status":"public_permissive"},"projected":{"__route_key__leak":"SECRET"}}"#,
                "public_report",
                false
            ),
            vec![
                route_decision(
                    "$.record._route_value_record",
                    "block_export",
                    "EXPORT_INPUT_RESERVED_NAME"
                ),
                route_decision(
                    "$.record.projected.__route_key__leak",
                    "block_export",
                    "EXPORT_INPUT_RESERVED_NAME"
                ),
            ]
        );
    }

    /// HELP_HUMAN ruling, 2026-10-28: "fail closed on reserved internal names, in all three
    /// implementations."  At the top level, in a nested value record, in an array and in a
    /// provenance; the decision names the path, never the value.
    #[test]
    fn reserved_internal_names_block_the_export_by_path_only() {
        for (record, reserved_path) in [
            (
                r#"{"value":"outer","field_class":"note","__route_key__value":"SECRET"}"#,
                "$.record.__route_key__value",
            ),
            (
                r#"{"value":"outer","field_class":"note","text":{"value":"inner","field_class":"note","_route_projected_leaf":"SECRET"}}"#,
                "$.record.text._route_projected_leaf",
            ),
            (
                r#"{"value":"outer","field_class":"note","child":[{"note":"x"},{"_route_value_record":"SECRET"}]}"#,
                "$.record.child[1]._route_value_record",
            ),
            (
                r#"{"value":"outer","privacy_classification":"public","provenance":{"__route_key__x":"SECRET"}}"#,
                "$.record.provenance.__route_key__x",
            ),
        ] {
            let decided = value_record(record, "local_private", true);
            assert_eq!(
                decided,
                vec![route_decision(
                    reserved_path,
                    "block_export",
                    "EXPORT_INPUT_RESERVED_NAME"
                )],
                "{record}"
            );
            assert!(decided.iter().all(|(path, _, _)| !path.contains("SECRET")));
        }
    }

    /// P1-2: a labelled item beneath a raised member is raised; the twins assert it is
    /// replaced whole in the payload.
    #[test]
    fn value_record_raised_labelled_item_is_raised() {
        let decided = value_record(
            r#"{"value":"outer","field_class":"note","text":{"value":"safe","privacy_classification":"public","redistribution_status":"public_permissive","note":"SECRET"}}"#,
            "public_report",
            false,
        );
        assert!(decided.contains(&route_decision(
            "$.record.text",
            "redact_value",
            "REDISTRIBUTION_STATUS_UNKNOWN"
        )));
    }

    /// P1-3: a value record beneath a raised member keeps the enclosing floor, in an object,
    /// an array and a nested label member.
    #[test]
    fn value_record_nested_value_record_keeps_the_inherited_floor() {
        for (name, member) in [
            ("text", r#"{"value":"SECRET","export_policy":{}}"#),
            ("text", r#"[{"value":"SECRET","export_policy":{}}]"#),
            (
                "privacy_classification",
                r#"{"value":"SECRET","export_policy":{}}"#,
            ),
        ] {
            let decided = value_record(
                &format!(r#"{{"value":"outer","concrete_path_present":true,"{name}":{member}}}"#),
                "local_private",
                true,
            );
            let nested: Vec<&RouteDecision> = decided
                .iter()
                .filter(|(path, _, _)| path.starts_with(&format!("$.record.{name}")))
                .collect();
            assert!(!nested.is_empty(), "{name} {member}");
            assert!(
                nested
                    .iter()
                    .all(|(_, action, reason)| *action == "redact_value"
                        && *reason == "CONCRETE_PATH_REDACTED"),
                "{name} {member}: {nested:?}"
            );
        }
    }

    /// Before the reorder `concrete_path_present: true` returned `redact_value` /
    /// `CONCRETE_PATH_REDACTED` ahead of the protected-content block, in every context.
    #[test]
    fn concrete_path_never_turns_a_protected_block_into_a_redaction() {
        let item = json!({
            "field_id": "invented.input_contract.path_protected",
            "concrete_path_present": true,
            "privacy_classification": "protected_suspected",
            "redistribution_status": "public_permissive",
            "value": "Invented placeholder value"
        });
        for context in [
            "public_report",
            "public_example",
            "shared_model",
            "downstream_tool",
            "local_private",
            USER_RELEASED_REPORT_CONTEXT,
        ] {
            let decided =
                classify_contract_item(item.as_object().expect("object"), context, false, false);
            assert_eq!(
                (decided.action, decided.reason_code),
                ("block_export", "PROTECTED_CONTENT_BLOCKED"),
                "{context}"
            );
        }
    }

    #[test]
    fn user_release_intent_cannot_enable_export_in_another_context() {
        let item = json!({
            "field_id": "invented.private",
            "privacy_classification": "private_project_data",
            "redistribution_status": "private_only",
            "review_status": "accepted",
            "value": "Invented private value"
        });
        let item = item.as_object().expect("object");
        for context in [
            "public_report",
            "public_example",
            "shared_model",
            "downstream_tool",
            "local_private",
        ] {
            let refused =
                std::panic::catch_unwind(|| classify_contract_item(item, context, true, true));
            assert!(
                refused.is_err(),
                "release intent must be refused in {context}"
            );
        }
        let released = classify_contract_item(item, USER_RELEASED_REPORT_CONTEXT, false, true);
        assert_eq!(released.action, "warning_only");
        assert_eq!(released.reason_code, "USER_RELEASED_PROJECT_DATA");
    }

    #[test]
    fn payload_carried_intent_grants_nothing_in_the_user_released_context() {
        let item = json!({
            "field_id": "invented.private",
            "privacy_classification": "private_project_data",
            "redistribution_status": "private_only",
            "local_private_intent": true,
            "explicit_local_private_intent": true,
            "user_intent": true,
            "user_release_intent": true,
            "explicit_user_release_intent": true,
            "export_policy": {"explicit_local_private_intent": true, "user_release_intent": true},
            "value": "Invented private value"
        });
        let blocked = classify_contract_item(
            item.as_object().expect("object"),
            USER_RELEASED_REPORT_CONTEXT,
            true,
            false,
        );
        assert_eq!(blocked.action, "block_export");
        assert_eq!(blocked.reason_code, "USER_RELEASE_INTENT_REQUIRED");
    }

    /// The user-released context and the five existing contexts read these inputs as
    /// TypeScript and Python do (the shared corpus holds the decisions).  Re-pinned
    /// (W-18b Rust-mirror alignment): this test previously asserted main's old
    /// existing-context answers on the same items, recording the mirror's departure
    /// from TypeScript and Python (it ignored `cache_payload` / `sqlite_payload`, did
    /// not fall back to the provenance for a null member or a null / `unknown` review,
    /// and did not count a scalar storage-privacy key as metadata).  The decision of
    /// 2026-10-27 aligns the mirror, so the existing contexts now give the contract's
    /// answers: the payload and the quarantined provenance block, and the null privacy
    /// takes its provenance's private class.
    #[test]
    fn every_context_mirrors_the_contract_reading_of_payload_provenance_and_storage_inputs() {
        let cases = [
            (
                json!({"privacy_classification": "private_project_data", "redistribution_status": "private_only", "cache_payload": 1, "value": "Invented value"}),
                ("block_export", "PAYLOAD_METADATA_ONLY_REQUIRED"),
            ),
            (
                json!({"privacy_classification": "private_project_data", "redistribution_status": "private_only", "sqlite_payload": 1, "value": "Invented value"}),
                ("block_export", "PAYLOAD_METADATA_ONLY_REQUIRED"),
            ),
            (
                json!({"privacy_classification": "private_project_data", "redistribution_status": "private_only", "review_status": "unknown", "provenance": {"review_status": "quarantined"}, "value": "Invented value"}),
                ("block_export", "PROTECTED_CONTENT_BLOCKED"),
            ),
            (
                json!({"privacy_classification": null, "redistribution_status": "private_only", "provenance": {"privacy_classification": "private_rule_pack_data"}, "value": 4.2}),
                ("redact_value", "PRIVATE_DATA_REDACTED"),
            ),
        ];
        for (item, (action, reason)) in &cases {
            let item = item.as_object().expect("object");
            let released = classify_contract_item(item, USER_RELEASED_REPORT_CONTEXT, false, true);
            assert_eq!((released.action, released.reason_code), (*action, *reason));
        }
        // The existing contexts' reading of the same items, now the contract's.  On main
        // these were `PRIVATE_LOCAL_ALLOWED_WITH_WARNING` for items 0 and 2 and
        // `UNKNOWN_PROVENANCE_WARNING` for item 3.
        let local = |item: &Value, intent| {
            let decided = classify_contract_item(
                item.as_object().expect("object"),
                "local_private",
                intent,
                false,
            );
            (decided.action, decided.reason_code)
        };
        assert_eq!(
            local(&cases[0].0, true),
            ("block_export", "PAYLOAD_METADATA_ONLY_REQUIRED")
        );
        assert_eq!(
            local(&cases[1].0, true),
            ("block_export", "PAYLOAD_METADATA_ONLY_REQUIRED")
        );
        assert_eq!(
            local(&cases[2].0, true),
            ("block_export", "PROTECTED_CONTENT_BLOCKED")
        );
        assert_eq!(
            local(&cases[3].0, true),
            ("warning_only", "PRIVATE_LOCAL_ALLOWED_WITH_WARNING")
        );
        assert_eq!(
            local(&cases[3].0, false),
            ("block_export", "LOCAL_PRIVATE_INTENT_REQUIRED")
        );
        for context in [
            "public_report",
            "public_example",
            "shared_model",
            "downstream_tool",
        ] {
            for (index, (item, (action, reason))) in cases.iter().enumerate() {
                let decided = classify_contract_item(
                    item.as_object().expect("object"),
                    context,
                    false,
                    false,
                );
                assert_eq!(
                    (decided.action, decided.reason_code),
                    (*action, *reason),
                    "item {index} in {context}"
                );
            }
        }
        let no_metadata_but_storage = json!({"storage_locality": "invented_locality", "value": 1});
        let public = classify_contract_item(
            no_metadata_but_storage.as_object().expect("object"),
            "public_report",
            false,
            false,
        );
        // On main the scalar storage key did not count: `MISSING_METADATA_REDACTED`.
        assert!(public.source_metadata_present);
        assert_eq!(
            (public.action, public.reason_code),
            ("redact_value", "REDISTRIBUTION_STATUS_UNKNOWN")
        );
        let released = classify_contract_item(
            no_metadata_but_storage.as_object().expect("object"),
            USER_RELEASED_REPORT_CONTEXT,
            false,
            true,
        );
        assert!(released.source_metadata_present);
        assert_eq!(released.reason_code, "USER_RELEASE_VALUE_UNBOUND");
    }
}
