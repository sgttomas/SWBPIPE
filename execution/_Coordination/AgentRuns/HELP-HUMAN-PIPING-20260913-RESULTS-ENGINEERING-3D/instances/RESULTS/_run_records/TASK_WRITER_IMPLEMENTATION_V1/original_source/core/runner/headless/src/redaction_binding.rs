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
#[cfg(test)]
const PAYLOAD_KEYS: [&str; 10] = [
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

/// Mirror the existing core contract's item classification without adding
/// filesystem, transport, or side-effect behavior.  The runner route uses a
/// fixed local/private projection; the canonical corpus test below pins the
/// vocabulary and precedence rules shared with Python and TypeScript.
#[cfg(test)]
fn classify_contract_item(
    item: &Map<String, Value>,
    export_context: &str,
    wrapper_intent: bool,
) -> ContractClassification {
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
        || unsafe_present;
    let provenance = item.get("provenance").and_then(Value::as_object);
    let privacy = item
        .get("privacy_classification")
        .or_else(|| provenance.and_then(|value| value.get("privacy_classification")));
    let redistribution = item
        .get("redistribution_status")
        .or_else(|| provenance.and_then(|value| value.get("redistribution_status")));
    let review = item
        .get("review_status")
        .or_else(|| provenance.and_then(|value| value.get("review_status")));
    let item_intent = INTENT_KEYS
        .iter()
        .find_map(|key| item.get(*key).map(truthy))
        .or_else(|| {
            item.get("export_policy")
                .and_then(Value::as_object)
                .and_then(|policy| {
                    INTENT_KEYS
                        .iter()
                        .find_map(|key| policy.get(*key).map(truthy))
                })
        });
    let explicit_intent = match item_intent {
        Some(false) => false,
        Some(true) => true,
        None => wrapper_intent,
    };
    let privacy_text = privacy.and_then(Value::as_str);
    let redistribution_text = redistribution.and_then(Value::as_str);
    let review_text = review.and_then(Value::as_str).unwrap_or("unknown");

    let contains_payload = truthy_option(item.get("contains_payload"))
        || truthy_option(item.get("payload_present"))
        || PAYLOAD_KEYS.iter().any(|key| item.contains_key(*key));
    let secret_present = truthy_option(item.get("secret_material_present"))
        || SECRET_KEYS.iter().any(|key| item.contains_key(*key));
    let cloud_present = truthy_option(item.get("cloud_or_network_reference"))
        || CLOUD_KEYS.iter().any(|key| item.contains_key(*key));
    let sql_present = truthy_option(item.get("direct_sql_access"))
        || SQL_KEYS.iter().any(|key| item.contains_key(*key));
    let storage_bypass = [
        "storage_bypass",
        "storage_bypass_requested",
        "direct_storage_bypass",
        "application_service_bypass",
    ]
    .iter()
    .any(|key| truthy_option(item.get(*key)));
    let path_present = truthy_option(item.get("concrete_path_present"))
        || PATH_KEYS.iter().any(|key| item.contains_key(*key));

    let unknown_action = if export_context == "local_private" {
        "warning_only"
    } else {
        "redact_value"
    };
    let (action, reason) = if truthy_option(item.get("professional_claim")) {
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
    } else if path_present {
        ("redact_value", "CONCRETE_PATH_REDACTED")
    } else if privacy_text == Some("protected_suspected")
        || redistribution_text == Some("protected_suspected")
        || matches!(review_text, "quarantined" | "rejected")
    {
        ("block_export", "PROTECTED_CONTENT_BLOCKED")
    } else if !metadata_present {
        (unknown_action, "MISSING_METADATA_REDACTED")
    } else if matches!(redistribution_text, None | Some("unknown") | Some("TBD")) {
        (unknown_action, "REDISTRIBUTION_STATUS_UNKNOWN")
    } else if matches!(privacy_text, None | Some("unknown") | Some("TBD")) {
        (unknown_action, "UNKNOWN_PROVENANCE_WARNING")
    } else if matches!(privacy_text, Some("public") | Some("public_metadata"))
        && redistribution_text == Some("public_permissive")
    {
        ("include", "SAFE_PUBLIC_METADATA")
    } else if privacy_text == Some("invented_public_example")
        && redistribution_text == Some("invented_non_engineering_example")
    {
        ("include", "SAFE_INVENTED_PUBLIC_EXAMPLE")
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
    let finding_class = match reason {
        "SAFE_PUBLIC_METADATA" | "SAFE_INVENTED_PUBLIC_EXAMPLE" => None,
        "PRIVATE_DATA_REDACTED"
        | "PRIVATE_LOCAL_ALLOWED_WITH_WARNING"
        | "LOCAL_PRIVATE_INTENT_REQUIRED"
        | "SECRET_MATERIAL_BLOCKED" => Some("PRIVATE_DATA_WARNING"),
        "UNKNOWN_PROVENANCE_WARNING"
        | "REDISTRIBUTION_STATUS_UNKNOWN"
        | "MISSING_METADATA_REDACTED" => Some("PROVENANCE_WARNING"),
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

#[cfg(test)]
fn truthy_option(value: Option<&Value>) -> bool {
    value.map(truthy).unwrap_or(false)
}

#[cfg(test)]
fn truthy(value: &Value) -> bool {
    match value {
        Value::Bool(value) => *value,
        Value::String(value) => matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "true" | "yes" | "explicit" | "present" | "local_private" | "explicit_local_private"
        ),
        Value::Null | Value::Array(_) | Value::Object(_) => false,
        Value::Number(value) => value.as_f64().map(|number| number != 0.0).unwrap_or(false),
    }
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
            let expected = case["expected"].as_object().expect("expected object");
            let actual = classify_contract_item(item, context, explicit_intent);
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
}
