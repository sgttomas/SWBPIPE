//! Model-document schema versioning and migration per DEC-019 (decision D-08).
//!
//! Policy carried here:
//! - the in-document `schema_version` (semver) is the sole authority for the
//!   model-document schema version; the SQLite `user_version` ledger remains
//!   DDL-only;
//! - migration runs in memory on open and persists only on save; stored bytes
//!   are never rewritten as a side effect of open;
//! - `newer_than_supported` and `unsupported_schema` documents are refused for
//!   editing with structured diagnostics — never coerced, never down-migrated;
//! - saving a migrated document writes a per-document ledger record with the
//!   pre- and post-migration model hashes.

use serde::Serialize;
use serde_json::{json, Value};

/// Current model-document schema version this application can author.
/// DEC-033 (2026-06-12): 0.2.0 adds the optional combination shape fields
/// introduced by TP-APP-R2-COMBEXPR-001; any change to what a document can
/// contain bumps minor, with patch reserved for non-shape changes.
pub const SUPPORTED_MODEL_SCHEMA_VERSION: &str = "0.2.0";
/// Framework label fixed by `schemas/project_persistence.schema.yaml` and
/// accepted in DEC-019.
pub const MODEL_MIGRATION_FRAMEWORK: &str = "application_service_separate_db_and_product_schema";

pub struct ModelDocumentMigration {
    pub source_version: &'static str,
    pub target_version: &'static str,
    pub migration_id: &'static str,
    pub apply: fn(&Value) -> Result<Value, String>,
}

/// Published transform chain.
///
/// `0.1.0 -> 0.2.0` (DEC-033): the 0.2.0 shape only adds optional combination
/// members (algebraic combination shape fields per TP-APP-R2-COMBEXPR-001), so
/// every valid 0.1.0 document is a valid 0.2.0 document unchanged. The
/// transform is a documented no-op; the chain walker stamps the target
/// `schema_version`, and the ledger record on save carries this migration id.
pub fn model_document_migrations() -> Vec<ModelDocumentMigration> {
    vec![ModelDocumentMigration {
        source_version: "0.1.0",
        target_version: "0.2.0",
        migration_id: "model-doc-0.1.0-to-0.2.0-additive-combination-shape-noop",
        apply: |document| Ok(document.clone()),
    }]
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ModelDocumentMigrationStatus {
    pub status: String,
    pub source_schema_version: String,
    pub target_schema_version: String,
    pub migration_framework: String,
    pub db_migration_status: String,
    pub product_schema_migration_status: String,
    pub applied_migration_ids: Vec<String>,
    pub persistence_state: String,
    pub detail: String,
}

pub struct EvaluatedModelDocument {
    /// Migrated document when a transform chain was applied; `None` when the
    /// stored document is already current (or when evaluation refused it).
    pub migrated_document: Option<Value>,
    pub status: ModelDocumentMigrationStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SemVer(u64, u64, u64);

fn parse_semver(raw: &str) -> Option<SemVer> {
    let mut parts = raw.trim().split('.');
    let major = parts.next()?.parse::<u64>().ok()?;
    let minor = parts.next()?.parse::<u64>().ok()?;
    let patch = parts.next()?.parse::<u64>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some(SemVer(major, minor, patch))
}

fn status(
    status: &str,
    source: &str,
    applied: Vec<String>,
    persistence_state: &str,
    detail: String,
) -> ModelDocumentMigrationStatus {
    ModelDocumentMigrationStatus {
        status: status.to_string(),
        source_schema_version: source.to_string(),
        target_schema_version: SUPPORTED_MODEL_SCHEMA_VERSION.to_string(),
        migration_framework: MODEL_MIGRATION_FRAMEWORK.to_string(),
        db_migration_status: "store_user_version_ledger_separate_ddl_only".to_string(),
        product_schema_migration_status: status.to_string(),
        applied_migration_ids: applied,
        persistence_state: persistence_state.to_string(),
        detail,
    }
}

/// Evaluate a model document against the supported schema version and the
/// transform chain. Never mutates the input document.
pub fn evaluate_model_document(
    document: &Value,
    chain: &[ModelDocumentMigration],
) -> EvaluatedModelDocument {
    let raw_version = document
        .get("schema_version")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let supported = parse_semver(SUPPORTED_MODEL_SCHEMA_VERSION)
        .expect("supported model schema version must be valid semver");

    let Some(document_version) = parse_semver(&raw_version) else {
        return EvaluatedModelDocument {
            migrated_document: None,
            status: status(
                "unsupported_schema",
                if raw_version.is_empty() { "missing_schema_version" } else { &raw_version },
                Vec::new(),
                "not_applicable_document_refused",
                "Model document has no valid semver schema_version; refusing to open for editing without coercion.".to_string(),
            ),
        };
    };

    if document_version == supported {
        return EvaluatedModelDocument {
            migrated_document: None,
            status: status(
                "current",
                &raw_version,
                Vec::new(),
                "stored_document_current",
                "Model document schema_version matches the supported version; no migration applied.".to_string(),
            ),
        };
    }

    if document_version > supported {
        return EvaluatedModelDocument {
            migrated_document: None,
            status: status(
                "newer_than_supported",
                &raw_version,
                Vec::new(),
                "not_applicable_document_refused",
                format!(
                    "Model document schema_version {raw_version} is newer than the supported {SUPPORTED_MODEL_SCHEMA_VERSION}; refusing to open for editing (no down-migration)."
                ),
            ),
        };
    }

    // Older than supported: walk the ordered transform chain.
    let mut current_document = document.clone();
    let mut current_version_raw = raw_version.clone();
    let mut applied: Vec<String> = Vec::new();
    loop {
        let Some(current_version) = parse_semver(&current_version_raw) else {
            return EvaluatedModelDocument {
                migrated_document: None,
                status: status(
                    "failed",
                    &raw_version,
                    applied,
                    "not_applicable_document_refused",
                    format!(
                        "Migration step produced invalid schema_version `{current_version_raw}`."
                    ),
                ),
            };
        };
        if current_version == supported {
            return EvaluatedModelDocument {
                migrated_document: Some(current_document),
                status: status(
                    "migrated",
                    &raw_version,
                    applied,
                    "in_memory_only_not_yet_saved",
                    format!(
                        "Model document migrated in memory from {raw_version} to {SUPPORTED_MODEL_SCHEMA_VERSION}; stored bytes are unchanged until save."
                    ),
                ),
            };
        }
        let Some(step) = chain
            .iter()
            .find(|item| item.source_version == current_version_raw)
        else {
            return EvaluatedModelDocument {
                migrated_document: None,
                status: status(
                    "unsupported_schema",
                    &raw_version,
                    applied,
                    "not_applicable_document_refused",
                    format!(
                        "No migration path exists from schema_version {current_version_raw} to {SUPPORTED_MODEL_SCHEMA_VERSION}; refusing to open for editing without coercion."
                    ),
                ),
            };
        };
        match (step.apply)(&current_document) {
            Ok(mut next_document) => {
                if let Some(slot) = next_document.get_mut("schema_version") {
                    *slot = json!(step.target_version);
                } else if let Some(object) = next_document.as_object_mut() {
                    object.insert("schema_version".to_string(), json!(step.target_version));
                }
                applied.push(step.migration_id.to_string());
                current_version_raw = step.target_version.to_string();
                current_document = next_document;
            }
            Err(error) => {
                return EvaluatedModelDocument {
                    migrated_document: None,
                    status: status(
                        "failed",
                        &raw_version,
                        applied,
                        "not_applicable_document_refused",
                        format!("Migration step {} failed: {error}", step.migration_id),
                    ),
                };
            }
        }
    }
}

/// Build the per-document ledger record written when a migrated document is
/// persisted (DEC-019 evidence guarantee).
pub fn migration_ledger_record(
    status: &ModelDocumentMigrationStatus,
    pre_migration_model_hash: &str,
    post_migration_model_hash: &str,
    recorded_at_unix: i64,
) -> Value {
    json!({
        "record_kind": "model_document_migration_ledger_record",
        "recorded_at_unix": recorded_at_unix,
        "source_schema_version": status.source_schema_version,
        "target_schema_version": status.target_schema_version,
        "applied_migration_ids": status.applied_migration_ids,
        "migration_framework": status.migration_framework,
        "pre_migration_model_hash": pre_migration_model_hash,
        "post_migration_model_hash": post_migration_model_hash,
        "trigger": "migrate_in_memory_on_open_persisted_on_save",
        "destructive_rewrite": false,
        "professional_boundary": {
            "human_review_required": true,
            "software_makes_compliance_claim": false,
            "software_makes_approval_claim": false,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn document(version: &str) -> Value {
        json!({
            "schema_version": version,
            "document_kind": "openpipestress.product_preview.model",
            "project": { "id": "project:test", "name": "Test" },
            "nodes": []
        })
    }

    /// Injected pre-`0.1.0` test steps composed with the published chain so
    /// the established `0.0.x` fixtures keep exercising multi-step walks up to
    /// the supported version.
    fn test_chain() -> Vec<ModelDocumentMigration> {
        let mut chain = vec![
            ModelDocumentMigration {
                source_version: "0.0.8",
                target_version: "0.0.9",
                migration_id: "model-doc-0.0.8-to-0.0.9-test-step",
                apply: |doc| Ok(doc.clone()),
            },
            ModelDocumentMigration {
                source_version: "0.0.9",
                target_version: "0.1.0",
                migration_id: "model-doc-0.0.9-to-0.1.0-test-rename",
                apply: |doc| {
                    let mut next = doc.clone();
                    next["migrated_marker"] = json!(true);
                    Ok(next)
                },
            },
        ];
        chain.extend(model_document_migrations());
        chain
    }

    #[test]
    fn current_documents_pass_without_migration() {
        let doc = document(SUPPORTED_MODEL_SCHEMA_VERSION);
        let evaluated = evaluate_model_document(&doc, &model_document_migrations());
        assert_eq!(evaluated.status.status, "current");
        assert_eq!(
            evaluated.status.persistence_state,
            "stored_document_current"
        );
        assert!(evaluated.migrated_document.is_none());
        assert!(evaluated.status.applied_migration_ids.is_empty());
        assert_eq!(
            evaluated.status.migration_framework,
            MODEL_MIGRATION_FRAMEWORK
        );
    }

    #[test]
    fn newer_documents_are_refused_without_down_migration() {
        for version in ["0.3.0", "1.0.0", "0.2.1"] {
            let evaluated = evaluate_model_document(&document(version), &test_chain());
            assert_eq!(
                evaluated.status.status, "newer_than_supported",
                "version {version}"
            );
            assert!(evaluated.migrated_document.is_none());
            assert!(evaluated.status.detail.contains("no down-migration"));
        }
    }

    #[test]
    fn invalid_or_missing_versions_are_unsupported_not_coerced() {
        let mut missing = document("x");
        missing.as_object_mut().unwrap().remove("schema_version");
        for doc in [document("not-semver"), document("1.2"), missing] {
            let evaluated = evaluate_model_document(&doc, &test_chain());
            assert_eq!(evaluated.status.status, "unsupported_schema");
            assert!(evaluated.migrated_document.is_none());
        }
    }

    #[test]
    fn chain_migrates_older_documents_in_memory_without_mutating_input() {
        let doc = document("0.0.8");
        let snapshot = doc.clone();
        let evaluated = evaluate_model_document(&doc, &test_chain());
        assert_eq!(doc, snapshot, "input document must not be mutated");
        assert_eq!(evaluated.status.status, "migrated");
        assert_eq!(
            evaluated.status.persistence_state,
            "in_memory_only_not_yet_saved"
        );
        assert_eq!(
            evaluated.status.applied_migration_ids,
            vec![
                "model-doc-0.0.8-to-0.0.9-test-step".to_string(),
                "model-doc-0.0.9-to-0.1.0-test-rename".to_string(),
                "model-doc-0.1.0-to-0.2.0-additive-combination-shape-noop".to_string()
            ]
        );
        let migrated = evaluated.migrated_document.expect("migrated document");
        assert_eq!(
            migrated["schema_version"],
            json!(SUPPORTED_MODEL_SCHEMA_VERSION)
        );
        assert_eq!(migrated["migrated_marker"], json!(true));
    }

    #[test]
    fn published_chain_migrates_0_1_0_documents_with_the_dec_033_noop_entry() {
        let doc = document("0.1.0");
        let evaluated = evaluate_model_document(&doc, &model_document_migrations());
        assert_eq!(evaluated.status.status, "migrated");
        assert_eq!(evaluated.status.source_schema_version, "0.1.0");
        assert_eq!(
            evaluated.status.target_schema_version,
            SUPPORTED_MODEL_SCHEMA_VERSION
        );
        assert_eq!(
            evaluated.status.applied_migration_ids,
            vec!["model-doc-0.1.0-to-0.2.0-additive-combination-shape-noop".to_string()]
        );
        let migrated = evaluated.migrated_document.expect("migrated document");
        // DEC-033 no-op: the document is unchanged except the version stamp.
        let mut expected = doc.clone();
        expected["schema_version"] = json!(SUPPORTED_MODEL_SCHEMA_VERSION);
        assert_eq!(migrated, expected);
    }

    #[test]
    fn gaps_in_the_chain_refuse_with_unsupported_schema() {
        let evaluated = evaluate_model_document(&document("0.0.1"), &test_chain());
        assert_eq!(evaluated.status.status, "unsupported_schema");
        assert!(evaluated.status.detail.contains("No migration path"));
    }

    #[test]
    fn failing_transforms_report_failed_without_partial_documents() {
        let chain = vec![ModelDocumentMigration {
            source_version: "0.0.9",
            target_version: "0.1.0",
            migration_id: "model-doc-failing-step",
            apply: |_| Err("invented transform failure".to_string()),
        }];
        let evaluated = evaluate_model_document(&document("0.0.9"), &chain);
        assert_eq!(evaluated.status.status, "failed");
        assert!(evaluated.migrated_document.is_none());
        assert!(evaluated.status.detail.contains("model-doc-failing-step"));
    }

    #[test]
    fn ledger_records_carry_pre_and_post_hashes_and_no_claims() {
        let evaluated = evaluate_model_document(&document("0.0.9"), &test_chain());
        let record = migration_ledger_record(&evaluated.status, "sha256:pre", "sha256:post", 1234);
        assert_eq!(record["pre_migration_model_hash"], json!("sha256:pre"));
        assert_eq!(record["post_migration_model_hash"], json!("sha256:post"));
        assert_eq!(record["source_schema_version"], json!("0.0.9"));
        assert_eq!(
            record["target_schema_version"],
            json!(SUPPORTED_MODEL_SCHEMA_VERSION)
        );
        assert_eq!(record["destructive_rewrite"], json!(false));
        assert_eq!(
            record["professional_boundary"]["software_makes_compliance_claim"],
            json!(false)
        );
    }
}
