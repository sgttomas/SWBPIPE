mod model_document_migration;

use model_document_migration::{
    evaluate_model_document, migration_ledger_record, model_document_migrations,
    EvaluatedModelDocument, ModelDocumentMigrationStatus,
};
use open_pipe_stress_operation_applier::{apply_operation, validate_operation};
use open_pipe_stress_product_physics::{run_linear_static_preview, LinearStaticPreviewRequest};
use open_pipe_stress_units::{catalog_definitions, UnitDefinition};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, MutexGuard},
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::{AppHandle, Manager};

const PROJECT_STORE_FILE: &str = "openpipestress-projects.sqlite3";

fn fixture_path(file_name: &str) -> Result<PathBuf, String> {
    let cwd = std::env::current_dir().map_err(|error| error.to_string())?;
    let candidates = [
        cwd.join("../../fixtures/product_preview").join(file_name),
        cwd.join("../../../fixtures/product_preview")
            .join(file_name),
        cwd.join("fixtures/product_preview").join(file_name),
    ];
    candidates
        .into_iter()
        .find(|path| path.exists())
        .ok_or_else(|| format!("preview fixture not found: {file_name}"))
}

fn read_fixture(file_name: &str) -> Result<Value, String> {
    let path = fixture_path(file_name)?;
    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    serde_json::from_str(&text).map_err(|error| error.to_string())
}

#[derive(Debug, Serialize)]
struct UnitCatalogResponse {
    schema_version: &'static str,
    catalog_id: &'static str,
    decision_basis: &'static str,
    calculation_basis: &'static str,
    storage_convention: &'static str,
    entry_count: usize,
    entries: Vec<UnitCatalogEntry>,
    boundary: UnitCatalogBoundary,
}

#[derive(Debug, Serialize)]
struct UnitCatalogEntry {
    unit_id: &'static str,
    symbol: &'static str,
    dimension_id: &'static str,
    canonical: bool,
    transform_kind: &'static str,
    factor_representation: &'static str,
    offset_representation: Option<&'static str>,
    provenance: &'static str,
    review_status: &'static str,
}

#[derive(Debug, Serialize)]
struct UnitCatalogBoundary {
    source: &'static str,
    protected_content_included: bool,
    private_project_data_included: bool,
    professional_approval_claimed: bool,
    code_compliance_claimed: bool,
}

fn unit_transform_kind(definition: &UnitDefinition) -> &'static str {
    if definition.canonical {
        "identity"
    } else {
        definition.transform_to_canonical.kind.as_schema_value()
    }
}

fn unit_catalog_entry(definition: UnitDefinition) -> UnitCatalogEntry {
    UnitCatalogEntry {
        unit_id: definition.id.catalog_id(),
        symbol: definition.symbol,
        dimension_id: definition.dimension.as_str(),
        canonical: definition.canonical,
        transform_kind: unit_transform_kind(&definition),
        factor_representation: definition.factor_representation,
        offset_representation: definition.offset_representation,
        provenance: definition.provenance.as_schema_value(),
        review_status: definition.review_status.as_schema_value(),
    }
}

fn desktop_unit_catalog() -> UnitCatalogResponse {
    let entries = catalog_definitions()
        .map(unit_catalog_entry)
        .collect::<Vec<_>>();
    UnitCatalogResponse {
        schema_version: "0.1.0",
        catalog_id: "unit-system:dec-018-si-dual-display",
        decision_basis: "DEC-018",
        calculation_basis: "si_canonical",
        storage_convention: "entered_units_preserved",
        entry_count: entries.len(),
        entries,
        boundary: UnitCatalogBoundary {
            source: "core/units open_pipe_stress_units catalog",
            protected_content_included: false,
            private_project_data_included: false,
            professional_approval_claimed: false,
            code_compliance_claimed: false,
        },
    }
}

#[derive(Debug, Serialize)]
struct StorageCapability {
    engine: &'static str,
    bundled: bool,
    fts5_available: bool,
    network_required: bool,
    daemon_required: bool,
    telemetry_enabled: bool,
    path_policy: &'static str,
    large_file_policy: &'static str,
    database_path: String,
    compile_options: Vec<String>,
    migration_framework: &'static str,
    migration_status: String,
    store_schema_version: i64,
    store_schema_target_version: i64,
    migrations_applied_on_open: Vec<String>,
}

#[derive(Debug, Serialize)]
struct LocalProjectSummary {
    project_id: String,
    project_name: String,
    database_path: String,
    storage_mode: &'static str,
    migration_status: String,
    migration_framework: &'static str,
    store_schema_version: i64,
    store_schema_target_version: i64,
    migrations_applied_on_open: Vec<String>,
    fts_indexed: bool,
    copied_external_files: bool,
    editor_intent_count: usize,
    proposal_count: usize,
    selected_review_target_count: usize,
    selected_review_target_ref: String,
    persisted_mechanics_result_count: usize,
    persisted_analysis_run_count: usize,
    persisted_analysis_run_ref: String,
    persisted_model_hash_count: usize,
    persisted_model_hash_ref: String,
    persisted_project_envelope_hash_count: usize,
    persisted_project_envelope_hash_ref: String,
    message: String,
}

#[derive(Debug, Serialize)]
struct LocalProjectIndexEntry {
    project_id: String,
    project_name: String,
    storage_mode: &'static str,
    created_at_unix: i64,
    updated_at_unix: i64,
}

#[derive(Debug, Serialize)]
struct LocalProjectEnvelope {
    summary: LocalProjectSummary,
    model: Value,
    editor_intents: Value,
    proposal: Value,
    selected_review_target: Value,
    mechanics_result: Value,
    analysis_run: Value,
    model_hash: Value,
    project_envelope_hash: Value,
    model_document_migration: Value,
    model_migration_ledger: Value,
}

#[derive(Debug, Deserialize)]
struct SaveLocalProjectRequest {
    project_id: String,
    project_name: String,
    model: Value,
    #[serde(default)]
    editor_intents: Value,
    #[serde(default)]
    proposal: Value,
    #[serde(default)]
    selected_review_target: Value,
    #[serde(default)]
    mechanics_result: Value,
    #[serde(default)]
    analysis_run: Value,
    #[serde(default)]
    model_hash: Value,
    #[serde(default)]
    project_envelope_hash: Value,
    /// Open-time migration status passed back by the UI so persisting a
    /// migrated document can write its ledger record (DEC-019).
    #[serde(default)]
    model_document_migration: Value,
}

#[derive(Debug, Deserialize)]
struct SelectedReviewTarget {
    target_type: String,
    id: String,
}

fn app_store_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_local_data_dir()
        .map_err(|error| error.to_string())?;
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    Ok(dir.join(PROJECT_STORE_FILE))
}

const STORE_SCHEMA_TARGET_VERSION: i64 = 9;
const STORE_MIGRATION_FRAMEWORK: &str = "versioned_sqlite_user_version_migration_ledger";

struct StoreMigration {
    version: i64,
    migration_id: &'static str,
    apply: fn(&Connection) -> Result<(), String>,
}

#[derive(Debug, Clone, Serialize)]
struct StoreMigrationEvidence {
    migration_framework: &'static str,
    store_schema_version_before_open: i64,
    store_schema_version: i64,
    store_schema_target_version: i64,
    migrations_applied_on_open: Vec<String>,
    pending_migration_count: i64,
    migration_status: String,
}

fn store_migrations() -> Vec<StoreMigration> {
    vec![
        StoreMigration {
            version: 1,
            migration_id: "store-v1-base-project-and-fts-tables",
            apply: migrate_v1_base_tables,
        },
        StoreMigration {
            version: 2,
            migration_id: "store-v2-editor-intents-column",
            apply: |connection| {
                ensure_column(
                    connection,
                    "editor_intents_json",
                    "ALTER TABLE local_projects ADD COLUMN editor_intents_json TEXT NOT NULL DEFAULT '[]'",
                )
            },
        },
        StoreMigration {
            version: 3,
            migration_id: "store-v3-proposal-column",
            apply: |connection| {
                ensure_column(
                    connection,
                    "proposal_json",
                    "ALTER TABLE local_projects ADD COLUMN proposal_json TEXT NOT NULL DEFAULT 'null'",
                )
            },
        },
        StoreMigration {
            version: 4,
            migration_id: "store-v4-selected-review-target-column",
            apply: |connection| {
                ensure_column(
                    connection,
                    "selected_review_target_json",
                    "ALTER TABLE local_projects ADD COLUMN selected_review_target_json TEXT NOT NULL DEFAULT 'null'",
                )
            },
        },
        StoreMigration {
            version: 5,
            migration_id: "store-v5-mechanics-result-column",
            apply: |connection| {
                ensure_column(
                    connection,
                    "mechanics_result_json",
                    "ALTER TABLE local_projects ADD COLUMN mechanics_result_json TEXT NOT NULL DEFAULT 'null'",
                )
            },
        },
        StoreMigration {
            version: 6,
            migration_id: "store-v6-analysis-run-column",
            apply: |connection| {
                ensure_column(
                    connection,
                    "analysis_run_json",
                    "ALTER TABLE local_projects ADD COLUMN analysis_run_json TEXT NOT NULL DEFAULT 'null'",
                )
            },
        },
        StoreMigration {
            version: 7,
            migration_id: "store-v7-model-hash-column",
            apply: |connection| {
                ensure_column(
                    connection,
                    "model_hash_json",
                    "ALTER TABLE local_projects ADD COLUMN model_hash_json TEXT NOT NULL DEFAULT 'null'",
                )
            },
        },
        StoreMigration {
            version: 8,
            migration_id: "store-v8-project-envelope-hash-column",
            apply: |connection| {
                ensure_column(
                    connection,
                    "project_envelope_hash_json",
                    "ALTER TABLE local_projects ADD COLUMN project_envelope_hash_json TEXT NOT NULL DEFAULT 'null'",
                )
            },
        },
        // Evidence-only ledger of applied model-document migrations (DEC-019).
        // The model-document schema *version* authority stays in-document.
        StoreMigration {
            version: 9,
            migration_id: "store-v9-model-document-migration-ledger-column",
            apply: |connection| {
                ensure_column(
                    connection,
                    "model_migration_ledger_json",
                    "ALTER TABLE local_projects ADD COLUMN model_migration_ledger_json TEXT NOT NULL DEFAULT '[]'",
                )
            },
        },
    ]
}

fn migrate_v1_base_tables(connection: &Connection) -> Result<(), String> {
    connection
        .execute_batch(
            "
            CREATE TABLE IF NOT EXISTS local_projects (
                project_id TEXT PRIMARY KEY,
                project_name TEXT NOT NULL,
                model_json TEXT NOT NULL,
                created_at_unix INTEGER NOT NULL,
                updated_at_unix INTEGER NOT NULL
            );
            CREATE VIRTUAL TABLE IF NOT EXISTS local_project_fts USING fts5(
                project_id UNINDEXED,
                project_name,
                model_text
            );
            ",
        )
        .map_err(|error| error.to_string())
}

fn store_user_version(connection: &Connection) -> Result<i64, String> {
    connection
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .map_err(|error| error.to_string())
}

fn open_project_store(path: &Path) -> Result<(Connection, StoreMigrationEvidence), String> {
    let connection = Connection::open(path).map_err(|error| error.to_string())?;
    let migration = apply_store_migrations(&connection)?;
    Ok((connection, migration))
}

// Each migration step is idempotent (IF NOT EXISTS / column probes), so legacy
// stores written before the user_version ledger existed reconcile cleanly from
// version 0 without data loss.
fn apply_store_migrations(connection: &Connection) -> Result<StoreMigrationEvidence, String> {
    connection
        .execute_batch(
            "
            PRAGMA foreign_keys = ON;
            PRAGMA journal_mode = WAL;
            ",
        )
        .map_err(|error| error.to_string())?;
    let version_before = store_user_version(connection)?;
    let mut migrations_applied = Vec::new();
    for migration in store_migrations() {
        if migration.version > version_before {
            (migration.apply)(connection)?;
            connection
                .execute_batch(&format!("PRAGMA user_version = {}", migration.version))
                .map_err(|error| error.to_string())?;
            migrations_applied.push(migration.migration_id.to_string());
        }
    }
    let version_after = store_user_version(connection)?;
    let migration_status = if migrations_applied.is_empty() {
        format!("current_store_schema_v{version_after}_no_pending_migrations")
    } else {
        format!("migrated_on_open_store_schema_v{version_before}_to_v{version_after}")
    };
    Ok(StoreMigrationEvidence {
        migration_framework: STORE_MIGRATION_FRAMEWORK,
        store_schema_version_before_open: version_before,
        store_schema_version: version_after,
        store_schema_target_version: STORE_SCHEMA_TARGET_VERSION,
        migrations_applied_on_open: migrations_applied,
        pending_migration_count: STORE_SCHEMA_TARGET_VERSION - version_after,
        migration_status,
    })
}

fn ensure_column(
    connection: &Connection,
    column_name: &str,
    alter_sql: &str,
) -> Result<(), String> {
    let mut statement = connection
        .prepare("PRAGMA table_info(local_projects)")
        .map_err(|error| error.to_string())?;
    let column_names = statement
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    if !column_names.iter().any(|name| name == column_name) {
        connection
            .execute(alter_sql, [])
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn sqlite_compile_options(connection: &Connection) -> Result<Vec<String>, String> {
    let mut statement = connection
        .prepare("PRAGMA compile_options")
        .map_err(|error| error.to_string())?;
    let options = statement
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    Ok(options)
}

fn fts5_available(connection: &Connection) -> bool {
    connection
        .execute_batch(
            "
            CREATE VIRTUAL TABLE IF NOT EXISTS temp.openpipestress_fts_probe USING fts5(value);
            DROP TABLE temp.openpipestress_fts_probe;
            ",
        )
        .is_ok()
}

fn now_unix_seconds() -> Result<i64, String> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?;
    Ok(duration.as_secs() as i64)
}

fn project_name_from_model(model: &Value) -> String {
    model
        .get("project")
        .and_then(|project| project.get("name"))
        .and_then(Value::as_str)
        .filter(|name| !name.trim().is_empty())
        .unwrap_or("Untitled OpenPipeStress Project")
        .trim()
        .to_string()
}

fn project_id_from_model(model: &Value) -> String {
    model
        .get("project")
        .and_then(|project| project.get("id"))
        .and_then(Value::as_str)
        .filter(|id| !id.trim().is_empty())
        .unwrap_or("project:untitled-local")
        .trim()
        .to_string()
}

fn upsert_project(
    connection: &mut Connection,
    project_id: &str,
    project_name: &str,
    model: &Value,
    editor_intents: &Value,
    proposal: &Value,
    selected_review_target: &Value,
    mechanics_result: &Value,
    analysis_run: &Value,
    model_hash: &Value,
    project_envelope_hash: &Value,
    model_migration_ledger: &Value,
) -> Result<(), String> {
    let model_json = serde_json::to_string(model).map_err(|error| error.to_string())?;
    let editor_intents_json =
        serde_json::to_string(editor_intents).map_err(|error| error.to_string())?;
    let proposal_json = serde_json::to_string(proposal).map_err(|error| error.to_string())?;
    let selected_review_target_json =
        serde_json::to_string(selected_review_target).map_err(|error| error.to_string())?;
    let mechanics_result_json =
        serde_json::to_string(mechanics_result).map_err(|error| error.to_string())?;
    let analysis_run_json =
        serde_json::to_string(analysis_run).map_err(|error| error.to_string())?;
    let model_hash_json = serde_json::to_string(model_hash).map_err(|error| error.to_string())?;
    let project_envelope_hash_json =
        serde_json::to_string(project_envelope_hash).map_err(|error| error.to_string())?;
    let model_migration_ledger_json =
        serde_json::to_string(model_migration_ledger).map_err(|error| error.to_string())?;
    let search_text = format!(
        "{model_json}\n{editor_intents_json}\n{proposal_json}\n{selected_review_target_json}\n{analysis_run_json}"
    );
    let now = now_unix_seconds()?;
    let transaction = connection
        .transaction()
        .map_err(|error| error.to_string())?;
    transaction
        .execute(
            "
            INSERT INTO local_projects (project_id, project_name, model_json, editor_intents_json, proposal_json, selected_review_target_json, mechanics_result_json, analysis_run_json, model_hash_json, project_envelope_hash_json, model_migration_ledger_json, created_at_unix, updated_at_unix)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?12)
            ON CONFLICT(project_id) DO UPDATE SET
                project_name = excluded.project_name,
                model_json = excluded.model_json,
                editor_intents_json = excluded.editor_intents_json,
                proposal_json = excluded.proposal_json,
                selected_review_target_json = excluded.selected_review_target_json,
                mechanics_result_json = excluded.mechanics_result_json,
                analysis_run_json = excluded.analysis_run_json,
                model_hash_json = excluded.model_hash_json,
                project_envelope_hash_json = excluded.project_envelope_hash_json,
                model_migration_ledger_json = excluded.model_migration_ledger_json,
                updated_at_unix = excluded.updated_at_unix
            ",
            params![
                project_id,
                project_name,
                model_json,
                editor_intents_json,
                proposal_json,
                selected_review_target_json,
                mechanics_result_json,
                analysis_run_json,
                model_hash_json,
                project_envelope_hash_json,
                model_migration_ledger_json,
                now
            ],
        )
        .map_err(|error| error.to_string())?;
    transaction
        .execute(
            "DELETE FROM local_project_fts WHERE project_id = ?1",
            params![project_id],
        )
        .map_err(|error| error.to_string())?;
    transaction
        .execute(
            "
            INSERT INTO local_project_fts (project_id, project_name, model_text)
            VALUES (?1, ?2, ?3)
            ",
            params![project_id, project_name, search_text],
        )
        .map_err(|error| error.to_string())?;
    transaction.commit().map_err(|error| error.to_string())
}

struct StoredProjectRecord {
    project_id: String,
    project_name: String,
    model: Value,
    editor_intents: Value,
    proposal: Value,
    selected_review_target: Value,
    mechanics_result: Value,
    analysis_run: Value,
    model_hash: Value,
    project_envelope_hash: Value,
    model_migration_ledger: Value,
}

fn load_project(
    connection: &Connection,
    project_id: Option<&str>,
) -> Result<Option<StoredProjectRecord>, String> {
    type StoredRow = (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    );
    fn stored_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<StoredRow> {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, String>(5)?,
            row.get::<_, String>(6)?,
            row.get::<_, String>(7)?,
            row.get::<_, String>(8)?,
            row.get::<_, String>(9)?,
            row.get::<_, String>(10)?,
        ))
    }
    let row = match project_id {
        Some(id) => connection
            .query_row(
                "
                SELECT project_id, project_name, model_json, editor_intents_json, proposal_json, selected_review_target_json, mechanics_result_json, analysis_run_json, model_hash_json, project_envelope_hash_json, model_migration_ledger_json
                FROM local_projects
                WHERE project_id = ?1
                ",
                params![id],
                stored_row,
            )
            .optional()
            .map_err(|error| error.to_string())?,
        None => connection
            .query_row(
                "
                SELECT project_id, project_name, model_json, editor_intents_json, proposal_json, selected_review_target_json, mechanics_result_json, analysis_run_json, model_hash_json, project_envelope_hash_json, model_migration_ledger_json
                FROM local_projects
                ORDER BY updated_at_unix DESC, project_id ASC
                LIMIT 1
                ",
                [],
                stored_row,
            )
            .optional()
            .map_err(|error| error.to_string())?,
    };
    row.map(
        |(
            id,
            name,
            model_json,
            editor_intents_json,
            proposal_json,
            selected_review_target_json,
            mechanics_result_json,
            analysis_run_json,
            model_hash_json,
            project_envelope_hash_json,
            model_migration_ledger_json,
        )| {
            let model =
                serde_json::from_str::<Value>(&model_json).map_err(|error| error.to_string())?;
            let editor_intents = serde_json::from_str::<Value>(&editor_intents_json)
                .map_err(|error| error.to_string())?;
            let proposal =
                serde_json::from_str::<Value>(&proposal_json).map_err(|error| error.to_string())?;
            let selected_review_target =
                serde_json::from_str::<Value>(&selected_review_target_json)
                    .map_err(|error| error.to_string())?;
            let mechanics_result = serde_json::from_str::<Value>(&mechanics_result_json)
                .map_err(|error| error.to_string())?;
            let analysis_run = serde_json::from_str::<Value>(&analysis_run_json)
                .map_err(|error| error.to_string())?;
            let model_hash = serde_json::from_str::<Value>(&model_hash_json)
                .map_err(|error| error.to_string())?;
            let project_envelope_hash = serde_json::from_str::<Value>(&project_envelope_hash_json)
                .map_err(|error| error.to_string())?;
            let model_migration_ledger =
                serde_json::from_str::<Value>(&model_migration_ledger_json)
                    .map_err(|error| error.to_string())?;
            Ok(StoredProjectRecord {
                project_id: id,
                project_name: name,
                model,
                editor_intents,
                proposal,
                selected_review_target,
                mechanics_result,
                analysis_run,
                model_hash,
                project_envelope_hash,
                model_migration_ledger,
            })
        },
    )
    .transpose()
}

fn list_projects(connection: &Connection) -> Result<Vec<LocalProjectIndexEntry>, String> {
    let mut statement = connection
        .prepare(
            "
            SELECT project_id, project_name, created_at_unix, updated_at_unix
            FROM local_projects
            ORDER BY updated_at_unix DESC, project_id ASC
            ",
        )
        .map_err(|error| error.to_string())?;
    let entries = statement
        .query_map([], |row| {
            Ok(LocalProjectIndexEntry {
                project_id: row.get::<_, String>(0)?,
                project_name: row.get::<_, String>(1)?,
                storage_mode: "local_sqlite",
                created_at_unix: row.get::<_, i64>(2)?,
                updated_at_unix: row.get::<_, i64>(3)?,
            })
        })
        .map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    Ok(entries)
}

fn normalized_editor_intents(editor_intents: Option<Value>) -> Value {
    match editor_intents {
        Some(value) if value.is_array() => value,
        _ => json!([]),
    }
}

fn normalized_proposal(proposal: Option<Value>) -> Value {
    match proposal {
        Some(value) if value.is_object() => value,
        _ => Value::Null,
    }
}

fn normalized_selected_review_target(selected_review_target: Option<Value>) -> Value {
    match selected_review_target {
        Some(value) if value.is_object() => value,
        _ => Value::Null,
    }
}

fn normalized_mechanics_result(mechanics_result: Option<Value>) -> Value {
    match mechanics_result {
        Some(value) if value.is_object() => value,
        _ => Value::Null,
    }
}

fn normalized_analysis_run(analysis_run: Option<Value>) -> Value {
    match analysis_run {
        Some(value) if value.is_object() => value,
        _ => Value::Null,
    }
}

fn normalized_model_hash(model_hash: Option<Value>) -> Value {
    match model_hash {
        Some(value) if value.is_object() => value,
        _ => Value::Null,
    }
}

fn normalized_project_envelope_hash(project_envelope_hash: Option<Value>) -> Value {
    match project_envelope_hash {
        Some(value) if value.is_object() => value,
        _ => Value::Null,
    }
}

fn proposal_count(proposal: &Value) -> usize {
    if proposal.is_object() {
        1
    } else {
        0
    }
}

fn editor_intent_count(editor_intents: &Value) -> usize {
    editor_intents.as_array().map_or(0, Vec::len)
}

fn selected_review_target_count(selected_review_target: &Value) -> usize {
    if selected_review_target.is_object() {
        1
    } else {
        0
    }
}

fn object_count(value: &Value) -> usize {
    if value.is_object() {
        1
    } else {
        0
    }
}

fn persisted_analysis_run_ref(analysis_run: &Value, mechanics_result: &Value) -> String {
    analysis_run
        .get("analysis_run")
        .and_then(|run| run.get("run_id"))
        .and_then(Value::as_str)
        .or_else(|| mechanics_result.get("run_id").and_then(Value::as_str))
        .map(str::to_string)
        .unwrap_or_else(|| "not_persisted".to_string())
}

fn persisted_model_hash_ref(model_hash: &Value) -> String {
    model_hash
        .get("value")
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_else(|| "not_persisted".to_string())
}

fn persisted_project_envelope_hash_ref(project_envelope_hash: &Value) -> String {
    project_envelope_hash
        .get("value")
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_else(|| "not_persisted".to_string())
}

fn selected_review_target_ref(selected_review_target: &Value) -> String {
    let Some(target) = selected_review_target.as_object() else {
        return "not_selected".to_string();
    };
    let target_type = target
        .get("target_type")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let id = target
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    format!("{target_type}: {id}")
}

fn project_summary(
    project_id: String,
    project_name: String,
    database_path: PathBuf,
    migration: &StoreMigrationEvidence,
    editor_intents: &Value,
    proposal: &Value,
    selected_review_target: &Value,
    mechanics_result: &Value,
    analysis_run: &Value,
    model_hash: &Value,
    project_envelope_hash: &Value,
    message: String,
) -> LocalProjectSummary {
    LocalProjectSummary {
        project_id,
        project_name,
        database_path: database_path.display().to_string(),
        storage_mode: "local_sqlite",
        migration_status: migration.migration_status.clone(),
        migration_framework: migration.migration_framework,
        store_schema_version: migration.store_schema_version,
        store_schema_target_version: migration.store_schema_target_version,
        migrations_applied_on_open: migration.migrations_applied_on_open.clone(),
        fts_indexed: true,
        copied_external_files: false,
        editor_intent_count: editor_intent_count(editor_intents),
        proposal_count: proposal_count(proposal),
        selected_review_target_count: selected_review_target_count(selected_review_target),
        selected_review_target_ref: selected_review_target_ref(selected_review_target),
        persisted_mechanics_result_count: object_count(mechanics_result),
        persisted_analysis_run_count: object_count(analysis_run),
        persisted_analysis_run_ref: persisted_analysis_run_ref(analysis_run, mechanics_result),
        persisted_model_hash_count: object_count(model_hash),
        persisted_model_hash_ref: persisted_model_hash_ref(model_hash),
        persisted_project_envelope_hash_count: object_count(project_envelope_hash),
        persisted_project_envelope_hash_ref: persisted_project_envelope_hash_ref(
            project_envelope_hash,
        ),
        message,
    }
}

#[tauri::command]
fn get_unit_catalog() -> UnitCatalogResponse {
    desktop_unit_catalog()
}

#[tauri::command]
fn render_calculation_report(input: Value) -> Result<Value, String> {
    // DEC-021 (A7): deterministic hash-bound HTML rendering with the
    // three-point protected-content gates evaluated in the renderer crate.
    // The derived print view is non-hash-bound and names the canonical hash.
    let renderable: open_pipe_stress_report_renderer::RenderableReportInput =
        serde_json::from_value(input).map_err(|error| format!("RENDER-INPUT-INVALID: {error}"))?;
    let outcome = open_pipe_stress_report_renderer::render_calculation_report(&renderable);
    let derived_print_html =
        open_pipe_stress_report_renderer::derived_print_view(&outcome.html, &outcome.sha256_hex);
    let mut payload = serde_json::to_value(&outcome).map_err(|error| error.to_string())?;
    payload["derived_print_html"] = Value::String(derived_print_html);
    Ok(payload)
}

#[tauri::command]
fn load_preview_model() -> Result<Value, String> {
    read_fixture("invented_preview_model.json")
}

#[tauri::command]
fn load_design_knowledge() -> Result<Value, String> {
    read_fixture("invented_design_knowledge.json")
}

fn solve_preview_mechanics(model_payload: Value) -> Result<Value, String> {
    let model: open_pipe_stress_product_physics::PreviewModel =
        serde_json::from_value(model_payload).map_err(|error| error.to_string())?;
    serde_json::to_value(run_linear_static_preview(LinearStaticPreviewRequest {
        model,
        materials: vec![],
    }))
    .map_err(|error| error.to_string())
}

fn resolve_solve_model_payload(model: Option<Value>) -> Result<Value, String> {
    match model {
        Some(value) => Ok(value),
        None => read_fixture("invented_preview_model.json"),
    }
}

#[tauri::command]
fn run_preview_mechanics(model: Option<Value>) -> Result<Value, String> {
    solve_preview_mechanics(resolve_solve_model_payload(model)?)
}

const SOLVE_JOB_CANCELLATION_SCOPE: &str = "cooperative_checkpoints_not_preemptive";

#[derive(Debug, Clone, Serialize)]
struct SolveJobStartReceipt {
    job_id: String,
    backend_cancellation_token: String,
    state: String,
    cancellation_scope: &'static str,
}

#[derive(Debug, Clone, Serialize)]
struct SolveJobStatusReport {
    job_id: String,
    state: String,
    cancellation_requested: bool,
    cancellation_status: String,
    cancellation_scope: &'static str,
    result: Option<Value>,
    error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct SolveJobCancellationReceipt {
    job_id: String,
    accepted: bool,
    cancellation_status: String,
    job_state: String,
    cancellation_scope: &'static str,
    cancellation_success_claimed: bool,
}

#[derive(Debug, Clone)]
struct SolveJobRecord {
    cancellation_token: String,
    state: String,
    cancellation_requested: bool,
    cancellation_status: String,
    result: Option<Value>,
    error_message: Option<String>,
}

#[derive(Debug, Default)]
struct SolveJobTable {
    next_job_number: u64,
    records: HashMap<String, SolveJobRecord>,
}

#[derive(Debug, Default)]
struct SolveJobRegistry {
    jobs: Arc<Mutex<SolveJobTable>>,
}

fn lock_solve_jobs(
    jobs: &Arc<Mutex<SolveJobTable>>,
) -> Result<MutexGuard<'_, SolveJobTable>, String> {
    jobs.lock()
        .map_err(|_| "solve job registry lock poisoned".to_string())
}

fn start_solve_job(jobs: &Arc<Mutex<SolveJobTable>>) -> Result<SolveJobStartReceipt, String> {
    let mut table = lock_solve_jobs(jobs)?;
    table.next_job_number += 1;
    let job_number = table.next_job_number;
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_nanos();
    let job_id = format!("backend-solve-job-{job_number}");
    let cancellation_token = format!("solve-cancel-token-{job_number}-{nanos:x}");
    table.records.insert(
        job_id.clone(),
        SolveJobRecord {
            cancellation_token: cancellation_token.clone(),
            state: "queued".to_string(),
            cancellation_requested: false,
            cancellation_status: "not_requested".to_string(),
            result: None,
            error_message: None,
        },
    );
    Ok(SolveJobStartReceipt {
        job_id,
        backend_cancellation_token: cancellation_token,
        state: "queued".to_string(),
        cancellation_scope: SOLVE_JOB_CANCELLATION_SCOPE,
    })
}

fn execute_solve_job(
    jobs: &Arc<Mutex<SolveJobTable>>,
    job_id: &str,
    model_payload: Result<Value, String>,
) {
    {
        let Ok(mut table) = lock_solve_jobs(jobs) else {
            return;
        };
        let Some(record) = table.records.get_mut(job_id) else {
            return;
        };
        if record.cancellation_requested {
            record.state = "cancelled".to_string();
            record.cancellation_status = "cancelled_before_solver_start".to_string();
            return;
        }
        record.state = "running".to_string();
    }
    let outcome = model_payload.and_then(solve_preview_mechanics);
    publish_solve_outcome(jobs, job_id, outcome);
}

fn publish_solve_outcome(
    jobs: &Arc<Mutex<SolveJobTable>>,
    job_id: &str,
    outcome: Result<Value, String>,
) {
    let Ok(mut table) = lock_solve_jobs(jobs) else {
        return;
    };
    let Some(record) = table.records.get_mut(job_id) else {
        return;
    };
    if record.cancellation_requested {
        record.state = "cancelled".to_string();
        record.cancellation_status =
            "cancelled_before_result_publication_result_discarded".to_string();
        record.result = None;
        record.error_message = outcome.err();
        return;
    }
    match outcome {
        Ok(result) => {
            record.state = "completed".to_string();
            record.result = Some(result);
            record.error_message = None;
        }
        Err(error) => {
            record.state = "failed".to_string();
            record.result = None;
            record.error_message = Some(error);
        }
    }
}

fn solve_job_status(
    jobs: &Arc<Mutex<SolveJobTable>>,
    job_id: &str,
) -> Result<SolveJobStatusReport, String> {
    let table = lock_solve_jobs(jobs)?;
    let record = table
        .records
        .get(job_id)
        .ok_or_else(|| format!("unknown solve job: {job_id}"))?;
    Ok(SolveJobStatusReport {
        job_id: job_id.to_string(),
        state: record.state.clone(),
        cancellation_requested: record.cancellation_requested,
        cancellation_status: record.cancellation_status.clone(),
        cancellation_scope: SOLVE_JOB_CANCELLATION_SCOPE,
        result: record.result.clone(),
        error_message: record.error_message.clone(),
    })
}

fn cancel_solve_job(
    jobs: &Arc<Mutex<SolveJobTable>>,
    job_id: &str,
    cancellation_token: &str,
) -> Result<SolveJobCancellationReceipt, String> {
    let mut table = lock_solve_jobs(jobs)?;
    let record = table
        .records
        .get_mut(job_id)
        .ok_or_else(|| format!("unknown solve job: {job_id}"))?;
    if record.cancellation_token != cancellation_token {
        return Ok(SolveJobCancellationReceipt {
            job_id: job_id.to_string(),
            accepted: false,
            cancellation_status: "rejected_invalid_cancellation_token".to_string(),
            job_state: record.state.clone(),
            cancellation_scope: SOLVE_JOB_CANCELLATION_SCOPE,
            cancellation_success_claimed: false,
        });
    }
    if matches!(record.state.as_str(), "completed" | "failed" | "cancelled") {
        return Ok(SolveJobCancellationReceipt {
            job_id: job_id.to_string(),
            accepted: false,
            cancellation_status: format!(
                "request_after_terminal_state_{}_no_cancellation_performed",
                record.state
            ),
            job_state: record.state.clone(),
            cancellation_scope: SOLVE_JOB_CANCELLATION_SCOPE,
            cancellation_success_claimed: false,
        });
    }
    record.cancellation_requested = true;
    record.cancellation_status =
        "cancellation_requested_awaiting_cooperative_checkpoint".to_string();
    Ok(SolveJobCancellationReceipt {
        job_id: job_id.to_string(),
        accepted: true,
        cancellation_status: record.cancellation_status.clone(),
        job_state: record.state.clone(),
        cancellation_scope: SOLVE_JOB_CANCELLATION_SCOPE,
        cancellation_success_claimed: false,
    })
}

#[tauri::command]
fn start_preview_mechanics_job(
    state: tauri::State<'_, SolveJobRegistry>,
    model: Option<Value>,
) -> Result<SolveJobStartReceipt, String> {
    let receipt = start_solve_job(&state.jobs)?;
    let jobs = Arc::clone(&state.jobs);
    let job_id = receipt.job_id.clone();
    std::thread::spawn(move || {
        execute_solve_job(&jobs, &job_id, resolve_solve_model_payload(model));
    });
    Ok(receipt)
}

#[tauri::command]
fn poll_preview_mechanics_job(
    state: tauri::State<'_, SolveJobRegistry>,
    job_id: String,
) -> Result<SolveJobStatusReport, String> {
    solve_job_status(&state.jobs, &job_id)
}

#[tauri::command]
fn cancel_preview_mechanics_job(
    state: tauri::State<'_, SolveJobRegistry>,
    job_id: String,
    cancellation_token: String,
) -> Result<SolveJobCancellationReceipt, String> {
    cancel_solve_job(&state.jobs, &job_id, &cancellation_token)
}

fn build_sample_agent_proposal(
    result: Value,
    selected_target: Option<SelectedReviewTarget>,
) -> Value {
    let diagnostics = result
        .get("diagnostics")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let results = result
        .get("results")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let primary_ref = results
        .iter()
        .find(|item| {
            item.get("id").and_then(Value::as_str) == Some("result:force:pipe-P-120:axial")
        })
        .and_then(|item| item.get("id").and_then(Value::as_str))
        .or_else(|| {
            results
                .iter()
                .find(|item| {
                    item.get("kind").and_then(Value::as_str) == Some("element_local_axial_force")
                })
                .and_then(|item| item.get("id").and_then(Value::as_str))
        })
        .or_else(|| {
            diagnostics
                .iter()
                .find(|item| item.get("severity").and_then(Value::as_str) == Some("warning"))
                .and_then(|item| item.get("id").and_then(Value::as_str))
        })
        .unwrap_or("diagnostic:physics:rule-inputs-missing")
        .to_string();
    let (target_ref, target_kind) = selected_target
        .map(|target| (target.id, target.target_type.replace('_', " ")))
        .unwrap_or_else(|| (primary_ref, "computed mechanics".to_string()));
    let proposal = json!({
        "schema_version": "0.1.0",
        "document_kind": "openpipestress.product_preview.agent_proposal",
        "proposal_id": "proposal:physics-diagnostic-review",
        "model_ref": result.get("model_ref").cloned().unwrap_or_else(|| json!("project:invented-loop-01")),
        "prompt": "Review current computed mechanics diagnostics and suggest a non-mutating follow-up.",
        "operation": {
            "operation_id": "op:review-computed-diagnostic",
            "operation_kind": "attach_design_knowledge",
            "operation_status": "draft_user_review_required",
            "affected_entity_ids": [target_ref],
            "changes": [
                {
                    "change_id": "change:add-review-note",
                    "change_kind": "attach_design_knowledge",
                    "target_ref": target_ref,
                    "before": format!("No review note attached for the selected {target_kind} context."),
                    "after": format!("Attach review note referencing the current computed preview {target_kind} context.")
                }
            ]
        },
        "rationale": format!("Generated from current preview mechanics context; selected review reference is {target_ref}. This narrative is review-only and does not mutate accepted model state."),
        "assumptions": [
            "The model is invented and not a project basis.",
            "No protected criteria, allowables, or owner-standard values are available."
        ],
        "validation": {
            "schema_validation": "passed",
            "constraint_validation": "warning_computed_context_requires_human_review",
            "diff_preview_status": "generated_from_computed_context",
            "application_status": "not_applied"
        },
        "audit_boundary": {
            "requires_user_acceptance": true,
            "mutates_accepted_model_state": false,
            "acceptance_recorded_as_review_only": true
        },
        "professional_boundary": {
            "human_review_required": true,
            "software_makes_compliance_claim": false,
            "software_makes_certification_claim": false,
            "software_makes_sealing_claim": false,
            "software_makes_approval_claim": false
        }
    });
    json!({
        "proposal": proposal,
        "application_status": "not_applied",
        "accepted_model_mutated": false
    })
}

#[tauri::command]
fn sample_agent_proposal(
    mechanics_result: Option<Value>,
    selected_target: Option<SelectedReviewTarget>,
) -> Result<Value, String> {
    let result = match mechanics_result {
        Some(value) => value,
        None => run_preview_mechanics(None)?,
    };
    Ok(build_sample_agent_proposal(result, selected_target))
}

#[tauri::command]
fn validate_model_operation(
    model: Value,
    intent: Value,
    claimed_model_hash: Option<Value>,
) -> Result<Value, String> {
    let outcome = validate_operation(&model, &intent, claimed_model_hash.as_ref());
    serde_json::to_value(outcome).map_err(|error| error.to_string())
}

#[tauri::command]
fn apply_model_operation(
    model: Value,
    intent: Value,
    claimed_model_hash: Option<Value>,
) -> Result<Value, String> {
    let outcome = apply_operation(&model, &intent, claimed_model_hash.as_ref());
    serde_json::to_value(outcome).map_err(|error| error.to_string())
}

#[tauri::command]
fn get_local_storage_capability(app: AppHandle) -> Result<StorageCapability, String> {
    let path = app_store_path(&app)?;
    let (connection, migration) = open_project_store(&path)?;
    Ok(StorageCapability {
        engine: "SQLite",
        bundled: true,
        fts5_available: fts5_available(&connection),
        network_required: false,
        daemon_required: false,
        telemetry_enabled: false,
        path_policy: "app-local user data path; no repository-default writes",
        large_file_policy:
            "reference external files by path/hash metadata; do not silently copy large files",
        database_path: path.display().to_string(),
        compile_options: sqlite_compile_options(&connection)?,
        migration_framework: migration.migration_framework,
        migration_status: migration.migration_status.clone(),
        store_schema_version: migration.store_schema_version,
        store_schema_target_version: migration.store_schema_target_version,
        migrations_applied_on_open: migration.migrations_applied_on_open.clone(),
    })
}

fn refuse_model_document(status: &ModelDocumentMigrationStatus) -> String {
    format!(
        "Model document refused for editing: status={}; source_schema_version={}; target_schema_version={}; {}",
        status.status, status.source_schema_version, status.target_schema_version, status.detail
    )
}

fn hash_value_string(hash: &Value) -> String {
    hash.get("value")
        .and_then(Value::as_str)
        .unwrap_or("not_reported")
        .to_string()
}

/// Resolve the model document, migration status, and ledger array to persist
/// for create/save (DEC-019): refuse unsupported/newer documents, migrate
/// older ones through the published chain, and append a ledger record when a
/// migrated document is being persisted (whether migrated here or at open).
fn prepare_model_document_for_persist(
    connection: &Connection,
    project_id: &str,
    model: Value,
    open_time_status: &Value,
    incoming_model_hash: &Value,
) -> Result<(Value, ModelDocumentMigrationStatus, Value), String> {
    let EvaluatedModelDocument {
        migrated_document,
        status,
    } = evaluate_model_document(&model, &model_document_migrations());
    if matches!(
        status.status.as_str(),
        "newer_than_supported" | "unsupported_schema" | "failed"
    ) {
        return Err(refuse_model_document(&status));
    }

    let prior = load_project(connection, Some(project_id))?;
    let mut ledger = prior
        .as_ref()
        .map(|record| record.model_migration_ledger.clone())
        .filter(Value::is_array)
        .unwrap_or_else(|| Value::Array(Vec::new()));
    let pre_migration_model_hash = prior
        .as_ref()
        .map(|record| hash_value_string(&record.model_hash))
        .unwrap_or_else(|| "not_previously_stored".to_string());
    let post_migration_model_hash = hash_value_string(incoming_model_hash);

    let open_time_migrated =
        open_time_status.get("status").and_then(Value::as_str) == Some("migrated");
    let (model_to_persist, mut persisted_status, record_basis) = if status.status == "migrated" {
        let migrated = migrated_document
            .ok_or_else(|| "migrated status without a migrated document".to_string())?;
        (migrated, status.clone(), Some(status))
    } else if open_time_migrated {
        // The UI already holds the in-memory-migrated document from open; the
        // incoming document is current and the open-time status carries the
        // evidence to record.
        let recorded = ModelDocumentMigrationStatus {
            status: "migrated".to_string(),
            source_schema_version: open_time_status
                .get("source_schema_version")
                .and_then(Value::as_str)
                .unwrap_or("TBD")
                .to_string(),
            target_schema_version: open_time_status
                .get("target_schema_version")
                .and_then(Value::as_str)
                .unwrap_or(model_document_migration::SUPPORTED_MODEL_SCHEMA_VERSION)
                .to_string(),
            migration_framework: model_document_migration::MODEL_MIGRATION_FRAMEWORK.to_string(),
            db_migration_status: "store_user_version_ledger_separate_ddl_only".to_string(),
            product_schema_migration_status: "migrated".to_string(),
            applied_migration_ids: open_time_status
                .get("applied_migration_ids")
                .and_then(Value::as_array)
                .map(|items| {
                    items
                        .iter()
                        .filter_map(Value::as_str)
                        .map(str::to_string)
                        .collect()
                })
                .unwrap_or_default(),
            persistence_state: "in_memory_only_not_yet_saved".to_string(),
            detail: "Open-time in-memory migration persisted on save.".to_string(),
        };
        (model, status, Some(recorded))
    } else {
        (model, status, None)
    };

    if let Some(basis) = record_basis {
        let record = migration_ledger_record(
            &basis,
            &pre_migration_model_hash,
            &post_migration_model_hash,
            now_unix_seconds()?,
        );
        ledger
            .as_array_mut()
            .expect("ledger normalized to array above")
            .push(record);
        persisted_status.persistence_state = "persisted_with_ledger_record".to_string();
        persisted_status.product_schema_migration_status = "migrated".to_string();
        persisted_status.status = "migrated".to_string();
        persisted_status.detail =
            "Migrated model document persisted with a ledger record (DEC-019).".to_string();
    } else {
        persisted_status.persistence_state = "stored_document_current".to_string();
    }

    Ok((model_to_persist, persisted_status, ledger))
}

#[tauri::command]
fn create_local_project(
    app: AppHandle,
    model: Value,
    editor_intents: Option<Value>,
    proposal: Option<Value>,
    selected_review_target: Option<Value>,
    mechanics_result: Option<Value>,
    analysis_run: Option<Value>,
    model_hash: Option<Value>,
    project_envelope_hash: Option<Value>,
) -> Result<LocalProjectEnvelope, String> {
    let path = app_store_path(&app)?;
    let (mut connection, migration) = open_project_store(&path)?;
    let project_id = project_id_from_model(&model);
    let project_name = project_name_from_model(&model);
    let editor_intents = normalized_editor_intents(editor_intents);
    let proposal = normalized_proposal(proposal);
    let selected_review_target = normalized_selected_review_target(selected_review_target);
    let mechanics_result = normalized_mechanics_result(mechanics_result);
    let analysis_run = normalized_analysis_run(analysis_run);
    let model_hash = normalized_model_hash(model_hash);
    let project_envelope_hash = normalized_project_envelope_hash(project_envelope_hash);
    let (model, document_migration, model_migration_ledger) = prepare_model_document_for_persist(
        &connection,
        &project_id,
        model,
        &Value::Null,
        &model_hash,
    )?;
    upsert_project(
        &mut connection,
        &project_id,
        &project_name,
        &model,
        &editor_intents,
        &proposal,
        &selected_review_target,
        &mechanics_result,
        &analysis_run,
        &model_hash,
        &project_envelope_hash,
        &model_migration_ledger,
    )?;
    Ok(LocalProjectEnvelope {
        summary: project_summary(
            project_id,
            project_name,
            path,
            &migration,
            &editor_intents,
            &proposal,
            &selected_review_target,
            &mechanics_result,
            &analysis_run,
            &model_hash,
            &project_envelope_hash,
            "Created local SQLite project snapshot without external file copies.".to_string(),
        ),
        model,
        editor_intents,
        proposal,
        selected_review_target,
        mechanics_result,
        analysis_run,
        model_hash,
        project_envelope_hash,
        model_document_migration: serde_json::to_value(document_migration)
            .map_err(|error| error.to_string())?,
        model_migration_ledger,
    })
}

#[tauri::command]
fn open_local_project(
    app: AppHandle,
    project_id: Option<String>,
) -> Result<Option<LocalProjectEnvelope>, String> {
    let path = app_store_path(&app)?;
    let (connection, migration) = open_project_store(&path)?;
    load_project(&connection, project_id.as_deref())?
        .map(|record| {
            // DEC-019: evaluate the restored document; migrate in memory when
            // a chain applies; refuse newer/unsupported documents for editing.
            let EvaluatedModelDocument {
                migrated_document,
                status,
            } = evaluate_model_document(&record.model, &model_document_migrations());
            if matches!(
                status.status.as_str(),
                "newer_than_supported" | "unsupported_schema" | "failed"
            ) {
                return Err(refuse_model_document(&status));
            }
            let model = migrated_document.unwrap_or(record.model);
            Ok(LocalProjectEnvelope {
                summary: project_summary(
                    record.project_id,
                    record.project_name,
                    path.clone(),
                    &migration,
                    &record.editor_intents,
                    &record.proposal,
                    &record.selected_review_target,
                    &record.mechanics_result,
                    &record.analysis_run,
                    &record.model_hash,
                    &record.project_envelope_hash,
                    "Opened local SQLite project snapshot.".to_string(),
                ),
                model,
                editor_intents: record.editor_intents,
                proposal: record.proposal,
                selected_review_target: record.selected_review_target,
                mechanics_result: record.mechanics_result,
                analysis_run: record.analysis_run,
                model_hash: record.model_hash,
                project_envelope_hash: record.project_envelope_hash,
                model_document_migration: serde_json::to_value(status)
                    .map_err(|error| error.to_string())?,
                model_migration_ledger: record.model_migration_ledger,
            })
        })
        .transpose()
}

#[tauri::command]
fn list_local_projects(app: AppHandle) -> Result<Vec<LocalProjectIndexEntry>, String> {
    let path = app_store_path(&app)?;
    let (connection, _migration) = open_project_store(&path)?;
    list_projects(&connection)
}

#[tauri::command]
fn save_local_project(
    app: AppHandle,
    request: SaveLocalProjectRequest,
) -> Result<LocalProjectEnvelope, String> {
    let path = app_store_path(&app)?;
    let (mut connection, migration) = open_project_store(&path)?;
    let editor_intents = normalized_editor_intents(Some(request.editor_intents));
    let proposal = normalized_proposal(Some(request.proposal));
    let selected_review_target =
        normalized_selected_review_target(Some(request.selected_review_target));
    let mechanics_result = normalized_mechanics_result(Some(request.mechanics_result));
    let analysis_run = normalized_analysis_run(Some(request.analysis_run));
    let model_hash = normalized_model_hash(Some(request.model_hash));
    let project_envelope_hash =
        normalized_project_envelope_hash(Some(request.project_envelope_hash));
    let (model, document_migration, model_migration_ledger) = prepare_model_document_for_persist(
        &connection,
        &request.project_id,
        request.model,
        &request.model_document_migration,
        &model_hash,
    )?;
    upsert_project(
        &mut connection,
        &request.project_id,
        &request.project_name,
        &model,
        &editor_intents,
        &proposal,
        &selected_review_target,
        &mechanics_result,
        &analysis_run,
        &model_hash,
        &project_envelope_hash,
        &model_migration_ledger,
    )?;
    Ok(LocalProjectEnvelope {
        summary: project_summary(
            request.project_id,
            request.project_name,
            path,
            &migration,
            &editor_intents,
            &proposal,
            &selected_review_target,
            &mechanics_result,
            &analysis_run,
            &model_hash,
            &project_envelope_hash,
            "Saved current project model snapshot without external file copies.".to_string(),
        ),
        model,
        editor_intents,
        proposal,
        selected_review_target,
        mechanics_result,
        analysis_run,
        model_hash,
        project_envelope_hash,
        model_document_migration: serde_json::to_value(document_migration)
            .map_err(|error| error.to_string())?,
        model_migration_ledger,
    })
}

pub fn run() {
    tauri::Builder::default()
        .manage(SolveJobRegistry::default())
        .invoke_handler(tauri::generate_handler![
            load_preview_model,
            load_design_knowledge,
            run_preview_mechanics,
            start_preview_mechanics_job,
            poll_preview_mechanics_job,
            cancel_preview_mechanics_job,
            sample_agent_proposal,
            validate_model_operation,
            apply_model_operation,
            get_unit_catalog,
            get_local_storage_capability,
            create_local_project,
            open_local_project,
            list_local_projects,
            save_local_project,
            render_calculation_report
        ])
        .run(tauri::generate_context!())
        .expect("error while running OpenPipeStress technical preview");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn unit_entry<'a>(catalog: &'a UnitCatalogResponse, unit_id: &str) -> &'a UnitCatalogEntry {
        catalog
            .entries
            .iter()
            .find(|entry| entry.unit_id == unit_id)
            .unwrap_or_else(|| panic!("unit catalog entry not found: {unit_id}"))
    }

    #[test]
    fn get_unit_catalog_exposes_b2_schema_facing_metadata() {
        let catalog = get_unit_catalog();

        assert_eq!(catalog.decision_basis, "DEC-018");
        assert_eq!(catalog.calculation_basis, "si_canonical");
        assert_eq!(catalog.storage_convention, "entered_units_preserved");
        assert_eq!(catalog.entry_count, catalog.entries.len());
        assert_eq!(catalog.entry_count, open_pipe_stress_units::catalog().len());
        assert!(!catalog.boundary.protected_content_included);
        assert!(!catalog.boundary.private_project_data_included);
        assert!(!catalog.boundary.professional_approval_claimed);
        assert!(!catalog.boundary.code_compliance_claimed);

        let ids = catalog
            .entries
            .iter()
            .map(|entry| entry.unit_id)
            .collect::<BTreeSet<_>>();
        assert_eq!(ids.len(), catalog.entries.len());
        assert!(catalog
            .entries
            .iter()
            .all(|entry| !entry.factor_representation.trim().is_empty()));
        assert!(catalog
            .entries
            .iter()
            .all(|entry| entry.review_status == "accepted"));

        let inch = unit_entry(&catalog, "unit:inch");
        assert_eq!(inch.symbol, "in");
        assert_eq!(inch.dimension_id, "length");
        assert_eq!(inch.transform_kind, "multiplicative");
        assert!(inch.factor_representation.contains("25.4 mm/in"));
        assert_eq!(inch.provenance, "exact_public_definition");

        let fahrenheit = unit_entry(&catalog, "unit:degree_fahrenheit");
        assert_eq!(fahrenheit.transform_kind, "affine");
        assert!(fahrenheit
            .offset_representation
            .expect("fahrenheit offset")
            .contains("459.67"));

        assert_eq!(
            unit_entry(&catalog, "unit:pound_force").provenance,
            "conventional_public_constant"
        );
        assert_eq!(
            unit_entry(&catalog, "unit:newton_per_meter_linear").provenance,
            "project_governed_decision"
        );
        assert_eq!(
            unit_entry(&catalog, "unit:meter").transform_kind,
            "identity"
        );

        let payload = serde_json::to_value(&catalog).expect("catalog serializes");
        assert_eq!(payload["entries"][0]["unit_id"].is_string(), true);
        assert_eq!(
            payload["boundary"]["code_compliance_claimed"],
            Value::Bool(false)
        );
    }

    #[test]
    fn render_calculation_report_command_renders_fixture_unblocked() {
        let fixture_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../fixtures/reports/invented/calculation_report_fixture.json");
        let fixture: Value =
            serde_json::from_str(&std::fs::read_to_string(fixture_path).expect("fixture read"))
                .expect("fixture parses");
        let provenance = json!({
            "source_name": "Invented OpenPipeStress renderer test",
            "source_location": "apps/desktop/src-tauri/src/lib.rs",
            "source_license": "project_fixture",
            "contributor": "OpenPipeStress",
            "contributor_certification": "Invented non-engineering data only.",
            "redistribution_status": "invented_non_engineering_example",
            "review_status": "accepted",
            "privacy_classification": "invented_public_example"
        });
        let input = json!({
            "report_title": "Invented Calculation Report (Technical Preview)",
            "calculation_report": fixture["calculation_report"],
            "report_sections": {
                "report_section_id": "invented-report-sections-001",
                "model_ref": {"ref_type": "model", "ref_id": "invented-model"},
                "run_ref": {"ref_type": "analysis_run", "ref_id": "invented-run"},
                "diagnostics": [],
                "analysis_status_disclosures": [
                    {
                        "status": "MECHANICS_SOLVED",
                        "source": {"ref_type": "solver", "ref_id": "invented-solver"},
                        "affected_object": {"ref_type": "model", "ref_id": "invented-model"},
                        "explanation": "Invented preview mechanics solved.",
                        "human_review_required": true,
                        "human_acceptance_ref": null
                    },
                    {
                        "status": "HUMAN_REVIEW_REQUIRED",
                        "source": {"ref_type": "report_renderer", "ref_id": "invented-render"},
                        "affected_object": {"ref_type": "report", "ref_id": "invented-report-001"},
                        "explanation": "Human professional review is required before any reliance.",
                        "human_review_required": true,
                        "human_acceptance_ref": null
                    }
                ],
                "provenance_notes": [provenance],
                "user_supplied_values": [],
                "assumptions": [],
                "limitations": [
                    {
                        "limitation_id": "invented-limitation-001",
                        "source": {"ref_type": "report_renderer", "ref_id": "invented-render"},
                        "affected_scope": {"ref_type": "model", "ref_id": "invented-model"},
                        "statement": "Invented preview data only; not engineering output.",
                        "effect": {
                            "mechanics_solve_qualified": true,
                            "user_rule_check_qualified": false,
                            "report_completeness": "qualified",
                            "human_review_required": true
                        },
                        "provenance": provenance
                    }
                ],
                "unresolved_tbds": [],
                "professional_boundary": {
                    "human_review_required": true,
                    "software_makes_compliance_claim": false,
                    "software_makes_certification_claim": false,
                    "software_makes_sealing_claim": false,
                    "software_makes_approval_claim": false,
                    "software_makes_authentication_claim": false
                }
            },
            "result_rows": [
                {
                    "row_id": "invented-row-001",
                    "label": "Max bending stress (invented)",
                    "case_ref": "load_case:invented-weight",
                    "quantity_display": "12.5 MPa",
                    "source_ref": "result_envelope:invented-result"
                }
            ]
        });

        let outcome = render_calculation_report(input).expect("command renders");

        assert_eq!(outcome["export_blocked"], Value::Bool(false));
        assert_eq!(outcome["sha256_hex"].as_str().expect("hash").len(), 64);
        assert!(outcome["html"]
            .as_str()
            .expect("html")
            .starts_with("<!DOCTYPE html>"));
        let derived = outcome["derived_print_html"]
            .as_str()
            .expect("derived view");
        assert!(derived.contains("DERIVED VIEW"));
        assert!(derived.contains(outcome["sha256_hex"].as_str().expect("hash")));

        let rejected = render_calculation_report(json!({"report_title": "broken"}));
        assert!(rejected
            .expect_err("invalid input")
            .starts_with("RENDER-INPUT-INVALID"));
    }

    fn rehearsal_step_text<'a>(step: &'a Value, key: &str) -> &'a str {
        step.get(key)
            .and_then(Value::as_str)
            .unwrap_or_else(|| panic!("rehearsal step missing string field `{key}`"))
    }

    fn rehearsal_intent(step: &Value) -> Value {
        let payload = step
            .get("payload")
            .unwrap_or_else(|| panic!("rehearsal step {} missing payload", step["operation_id"]));
        json!({
            "operation_id": rehearsal_step_text(step, "operation_id"),
            "operation_kind": rehearsal_step_text(step, "operation_kind"),
            "operation_status": "proposed",
            "author_type": "user",
            "source": {
                "source_ref": "fixtures/product_preview/r2_from_blank_rehearsal.json",
                "source_channel": "local_desktop_preview",
                "source_role": "a12_rehearsal_script"
            },
            "target": step["target"].clone(),
            "change": {
                "change_id": format!(
                    "change:{}",
                    rehearsal_step_text(step, "operation_id").trim_start_matches("op:")
                ),
                "change_kind": rehearsal_step_text(step, "change_kind"),
                "field_label": rehearsal_step_text(step, "field_label"),
                "field_path": rehearsal_step_text(step, "field_path"),
                "before": rehearsal_step_text(step, "before"),
                "after": serde_json::to_string(payload).expect("rehearsal payload serializes"),
                "unit": rehearsal_step_text(step, "unit"),
                "dimension": rehearsal_step_text(step, "dimension"),
                "source_note": rehearsal_step_text(step, "source_note")
            },
            "validation": {
                "schema_validation": "not_run",
                "constraint_validation": "not_run",
                "unit_validation": "not_run",
                "diff_preview_status": "not_generated",
                "application_status": "not_applied"
            },
            "audit_boundary": {
                "mutation_route": "structured_operations_only",
                "direct_model_mutation_allowed": false,
                "requires_user_acceptance": true,
                "mutates_accepted_model_state": false
            },
            "professional_boundary": {
                "human_review_required": true,
                "software_makes_compliance_claim": false,
                "software_makes_certification_claim": false,
                "software_makes_sealing_claim": false,
                "software_makes_approval_claim": false,
                "software_makes_authentication_claim": false
            },
            "rationale": "A12 from-blank rehearsal step; proves the structured-operation seam composes without raw model-file editing."
        })
    }

    fn apply_rehearsal_steps(rehearsal: &Value) -> (Value, Vec<Value>) {
        let mut model = rehearsal["blank_model"].clone();
        let mut receipts = Vec::new();
        let steps = rehearsal["steps"]
            .as_array()
            .expect("rehearsal steps array");
        for step in steps {
            let intent = rehearsal_intent(step);
            let outcome = apply_model_operation(model, intent, None)
                .unwrap_or_else(|error| panic!("rehearsal operation command failed: {error}"));
            assert_eq!(
                outcome["validation"]["application_status"],
                json!("applied_to_session_model"),
                "rehearsal step {} should apply; diagnostics={:?}",
                step["operation_id"],
                outcome["diagnostics"]
            );
            assert_eq!(
                outcome["acceptance"]["acceptance_is_professional_approval"],
                json!(false)
            );
            model = outcome["applied_model"].clone();
            receipts.push(outcome);
        }
        (model, receipts)
    }

    fn count_array(value: &Value, key: &str) -> usize {
        value
            .get(key)
            .and_then(Value::as_array)
            .unwrap_or_else(|| panic!("missing array `{key}`"))
            .len()
    }

    fn read_report_fixture() -> Value {
        let fixture_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../fixtures/reports/invented/calculation_report_fixture.json");
        serde_json::from_str(&std::fs::read_to_string(fixture_path).expect("fixture read"))
            .expect("fixture parses")
    }

    fn provenance(source_location: &str) -> Value {
        json!({
            "source_name": "OpenPipeStress A12 from-blank rehearsal",
            "source_location": source_location,
            "source_license": "project_fixture",
            "contributor": "OpenPipeStress",
            "contributor_certification": "Invented non-engineering data only.",
            "redistribution_status": "invented_non_engineering_example",
            "review_status": "accepted",
            "privacy_classification": "invented_public_example"
        })
    }

    fn report_row_from_result(result: &Value) -> Value {
        let row_id = result["id"].as_str().unwrap_or("result:unknown");
        let kind = result["kind"].as_str().unwrap_or("result");
        let unit = result["unit"].as_str().unwrap_or("unit");
        let value = result["value"]
            .as_f64()
            .map(|number| number.to_string())
            .unwrap_or_else(|| "TBD".to_string());
        let case_ref = result
            .get("basis_ref")
            .and_then(Value::as_object)
            .map(|basis| {
                format!(
                    "{}:{}",
                    basis
                        .get("ref_type")
                        .and_then(Value::as_str)
                        .unwrap_or("basis"),
                    basis.get("ref_id").and_then(Value::as_str).unwrap_or("TBD")
                )
            })
            .unwrap_or_else(|| "TBD".to_string());
        json!({
            "row_id": format!("row:{row_id}"),
            "label": kind,
            "case_ref": case_ref,
            "quantity_display": format!("{value} {unit}"),
            "source_ref": format!("result:{row_id}")
        })
    }

    fn rehearsal_report_input(model: &Value, solved: &Value) -> Value {
        let mut fixture = read_report_fixture();
        let model_id = model["project"]["id"].as_str().expect("rehearsal model id");
        let project_name = model["project"]["name"]
            .as_str()
            .expect("rehearsal project name");
        let run_id = solved["run_id"].as_str().expect("rehearsal run id");
        let source_location = "fixtures/product_preview/r2_from_blank_rehearsal.json";
        let provenance = provenance(source_location);

        fixture["calculation_report"]["report_id"] = json!(format!("report:{run_id}"));
        fixture["calculation_report"]["model_input_summary"]["project_ref"] =
            json!({"ref_type": "project", "ref_id": model_id});
        fixture["calculation_report"]["model_input_summary"]["model_ref"] =
            json!({"ref_type": "model", "ref_id": model_id});
        fixture["calculation_report"]["model_input_summary"]["persistence_ref"] = json!({"ref_type": "project_persistence", "ref_id": "session_state_only_not_yet_saved"});
        fixture["calculation_report"]["model_input_summary"]["unit_system_ref"] =
            json!({"ref_type": "unit_system", "ref_id": "a12-rehearsal-si-local"});
        fixture["calculation_report"]["model_input_summary"]["provenance"] = provenance.clone();
        fixture["calculation_report"]["load_case_summary"] = Value::Array(
            model["load_cases"]
                .as_array()
                .expect("rehearsal load cases")
                .iter()
                .map(|load_case| {
                    json!({
                        "load_ref": {
                            "ref_type": "load_case",
                            "ref_id": load_case["id"].as_str().unwrap_or("load:TBD")
                        },
                        "label": load_case["label"].as_str().unwrap_or("TBD"),
                        "basis": load_case["kind"].as_str().unwrap_or("TBD"),
                        "source": { "ref_type": "model", "ref_id": model_id },
                        "provenance": provenance.clone()
                    })
                })
                .collect(),
        );
        fixture["calculation_report"]["analysis_status"] = json!([
            "HUMAN_REVIEW_REQUIRED",
            solved["status"]["mechanics"]
                .as_str()
                .unwrap_or("MODEL_INCOMPLETE"),
            solved["status"]["rule_check"]
                .as_str()
                .unwrap_or("RULE_INPUTS_INCOMPLETE")
        ]);
        fixture["calculation_report"]["provenance"] = provenance.clone();

        let first_result = solved["results"]
            .as_array()
            .and_then(|rows| rows.first())
            .expect("rehearsal solve emits at least one result row");
        let report_sections = json!({
            "report_section_id": format!("sections:{run_id}"),
            "model_ref": {"ref_type": "model", "ref_id": model_id},
            "run_ref": {"ref_type": "analysis_run", "ref_id": run_id},
            "diagnostics": [],
            "analysis_status_disclosures": [
                {
                    "status": solved["status"]["mechanics"].as_str().unwrap_or("MECHANICS_SOLVED"),
                    "source": {"ref_type": "analysis_run", "ref_id": run_id},
                    "affected_object": {"ref_type": "model", "ref_id": model_id},
                    "explanation": "A12 invented from-blank rehearsal solved through the backend preview mechanics command.",
                    "human_review_required": true,
                    "human_acceptance_ref": null
                },
                {
                    "status": "HUMAN_REVIEW_REQUIRED",
                    "source": {"ref_type": "report_renderer", "ref_id": format!("report:{run_id}")},
                    "affected_object": {"ref_type": "report", "ref_id": format!("report:{run_id}")},
                    "explanation": "Human professional review is required before any reliance.",
                    "human_review_required": true,
                    "human_acceptance_ref": null
                }
            ],
            "provenance_notes": [provenance.clone()],
            "user_supplied_values": [],
            "assumptions": [],
            "limitations": [
                {
                    "limitation_id": format!("limitation:{run_id}:technical-preview"),
                    "source": {"ref_type": "report_renderer", "ref_id": format!("report:{run_id}")},
                    "affected_scope": {"ref_type": "model", "ref_id": model_id},
                    "statement": "Technical-preview output over invented user-local data; not validated engineering output.",
                    "effect": {
                        "mechanics_solve_qualified": true,
                        "user_rule_check_qualified": false,
                        "report_completeness": "qualified",
                        "human_review_required": true
                    },
                    "provenance": provenance.clone()
                }
            ],
            "unresolved_tbds": [],
            "professional_boundary": {
                "human_review_required": true,
                "software_makes_compliance_claim": false,
                "software_makes_certification_claim": false,
                "software_makes_sealing_claim": false,
                "software_makes_approval_claim": false,
                "software_makes_authentication_claim": false
            }
        });

        json!({
            "report_title": format!("{project_name} - Calculation Report (Technical Preview)"),
            "calculation_report": fixture["calculation_report"].clone(),
            "report_sections": report_sections,
            "result_rows": [report_row_from_result(first_result)]
        })
    }

    #[test]
    fn r2_from_blank_rehearsal_authors_solves_and_renders_report() {
        let rehearsal =
            read_fixture("r2_from_blank_rehearsal.json").expect("A12 rehearsal fixture loads");
        assert_eq!(
            rehearsal["boundary"]["protected_content_included"],
            json!(false)
        );
        assert_eq!(
            rehearsal["boundary"]["private_payload_included"],
            json!(false)
        );

        let (model, receipts) = apply_rehearsal_steps(&rehearsal);
        assert_eq!(
            receipts.len(),
            rehearsal["steps"].as_array().expect("steps").len()
        );
        let expected = &rehearsal["expected"];
        assert_eq!(
            count_array(&model, "nodes"),
            expected["nodes"].as_u64().unwrap() as usize
        );
        assert_eq!(
            count_array(&model, "pipe_segments"),
            expected["pipe_segments"].as_u64().unwrap() as usize
        );
        assert_eq!(
            count_array(&model, "supports"),
            expected["supports"].as_u64().unwrap() as usize
        );
        assert_eq!(
            count_array(&model, "materials"),
            expected["materials"].as_u64().unwrap() as usize
        );
        assert_eq!(
            count_array(&model, "sections"),
            expected["sections"].as_u64().unwrap() as usize
        );
        assert_eq!(
            count_array(&model, "load_cases"),
            expected["load_cases"].as_u64().unwrap() as usize
        );
        assert_eq!(
            count_array(&model, "combinations"),
            expected["combinations"].as_u64().unwrap() as usize
        );

        let solved = run_preview_mechanics(Some(model.clone()))
            .expect("from-blank authored model solves through backend mechanics");
        assert_eq!(solved["model_ref"], model["project"]["id"]);
        assert_eq!(
            solved["status"]["mechanics"], expected["mechanics_status"],
            "unexpected rehearsal solve status; diagnostics={:?}",
            solved["diagnostics"]
        );
        assert!(
            solved["results"].as_array().expect("result rows").len()
                >= expected["minimum_result_rows"].as_u64().unwrap() as usize
        );
        assert!(solved["results"]
            .as_array()
            .expect("result rows")
            .iter()
            .any(|row| row["basis_ref"]["ref_id"] == json!("combination:R2-C-100")));

        let report_input = rehearsal_report_input(&model, &solved);
        let rendered =
            render_calculation_report(report_input).expect("A7 renderer accepts rehearsal report");
        assert_eq!(
            rendered["export_blocked"],
            expected["report_export_blocked"]
        );
        assert_eq!(rendered["sha256_hex"].as_str().expect("hash").len(), 64);
        let html = rendered["html"].as_str().expect("rendered html");
        assert!(html.contains("R2 From Blank Rehearsal"));
        assert!(html.contains("human_review_required"));
        assert!(!html.contains("<script"));
        assert!(!html.contains("code compliant"));
    }

    #[test]
    fn r2_from_blank_saved_project_opens_solves_and_renders_report() {
        let rehearsal =
            read_fixture("r2_from_blank_rehearsal.json").expect("A12 rehearsal fixture loads");
        let (model, receipts) = apply_rehearsal_steps(&rehearsal);
        assert_eq!(
            receipts.len(),
            rehearsal["steps"].as_array().expect("steps").len()
        );

        let mut connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        let migration =
            apply_store_migrations(&connection).expect("project store migrations apply");
        assert!(fts5_available(&connection));

        let project_id = model["project"]["id"]
            .as_str()
            .expect("rehearsal project id");
        let project_name = model["project"]["name"]
            .as_str()
            .expect("rehearsal project name");
        upsert_project(
            &mut connection,
            project_id,
            project_name,
            &model,
            &json!([]),
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &json!([]),
        )
        .expect("A12 authored model saves to local project store");

        let loaded = load_project(&connection, Some(project_id))
            .expect("saved A12 project loads")
            .expect("saved A12 project exists");
        assert_eq!(loaded.project_id, project_id);
        assert_eq!(loaded.project_name, project_name);
        assert_eq!(loaded.model, model);
        assert_eq!(loaded.mechanics_result, Value::Null);
        assert_eq!(loaded.analysis_run, Value::Null);

        let solved = run_preview_mechanics(Some(loaded.model.clone()))
            .expect("saved authored model solves through backend mechanics");
        assert_eq!(solved["model_ref"], json!(project_id));
        assert_eq!(
            solved["status"]["mechanics"],
            rehearsal["expected"]["mechanics_status"]
        );
        assert!(
            solved["results"].as_array().expect("result rows").len()
                >= rehearsal["expected"]["minimum_result_rows"]
                    .as_u64()
                    .unwrap() as usize
        );

        let rendered = render_calculation_report(rehearsal_report_input(&loaded.model, &solved))
            .expect("saved-project report renders");
        assert_eq!(
            rendered["export_blocked"],
            rehearsal["expected"]["report_export_blocked"]
        );
        assert_eq!(rendered["sha256_hex"].as_str().expect("hash").len(), 64);

        let summary = project_summary(
            loaded.project_id,
            loaded.project_name,
            PathBuf::from(":memory:"),
            &migration,
            &json!([]),
            &Value::Null,
            &Value::Null,
            &solved,
            &json!({
                "analysis_run": {
                    "run_id": solved["run_id"].clone()
                }
            }),
            &Value::Null,
            &Value::Null,
            "Saved A12 authored project opened, solved, and rendered.".to_string(),
        );
        assert_eq!(summary.persisted_mechanics_result_count, 1);
        assert_eq!(
            summary.persisted_analysis_run_ref,
            solved["run_id"].as_str().expect("run id")
        );
    }

    #[test]
    fn local_project_store_uses_sqlite_fts5_and_round_trips_model_snapshot() {
        let mut connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        let migration =
            apply_store_migrations(&connection).expect("project store migrations apply");
        assert!(fts5_available(&connection));

        let model = json!({
        "schema_version": "0.1.0",
        "document_kind": "openpipestress.product_preview.model",
            "project": {
                "id": "project:test-local",
                "name": "Test Local Project"
            }
        });
        let editor_intents = json!([
            {
                "queue_id": "editor-intent-1",
                "operation_id": "op:editor-intent-material:invented-carbon-steel-elastic_modulus.value",
                "validation": {
                    "application_status": "not_applied"
                },
                "audit_boundary": {
                    "mutates_accepted_model_state": false
                },
                "professional_boundary": {
                    "software_makes_compliance_claim": false
                }
            }
        ]);
        let proposal = json!({
            "proposal_id": "proposal:physics-diagnostic-review",
            "operation": {
                "operation_id": "op:review-computed-diagnostic",
                "operation_status": "draft_user_review_required"
            },
            "validation": {
                "application_status": "not_applied"
            },
            "audit_boundary": {
                "mutates_accepted_model_state": false
            },
            "professional_boundary": {
                "software_makes_compliance_claim": false
            }
        });
        let selected_review_target = json!({
            "target_type": "result",
            "id": "result:stress:pipe-P-120:end-j:torsional-shear"
        });
        let mechanics_result = json!({
            "run_id": "run:preview-linear-static-001",
            "model_ref": "project:test-local",
            "results": [
                {
                    "id": "result:force:pipe-P-120:axial",
                    "kind": "element_local_axial_force"
                }
            ],
            "diagnostics": []
        });
        let analysis_run = json!({
            "deliverable_id": "DEL-14-02",
            "analysis_run": {
                "run_id": "run:preview-linear-static-001",
                "run_kind": "mechanics_solve",
                "analysis_status": ["HUMAN_REVIEW_REQUIRED", "MECHANICS_SOLVED"]
            }
        });
        let model_hash = json!({
            "algorithm": "sha256",
            "canonicalization": "rfc8785_jcs",
            "payload_scope": "model_payload",
            "payload_ref": "project:test-local",
            "value": "sha256:0123abcd0123abcd0123abcd0123abcd0123abcd0123abcd0123abcd0123abcd",
            "hash_status": "computed_local_preview"
        });
        let project_envelope_hash = json!({
            "algorithm": "sha256",
            "canonicalization": "rfc8785_jcs",
            "payload_scope": "project_envelope_payload",
            "payload_excludes": "storage_summary_and_envelope_hash_carrier_fields",
            "payload_ref": "project:test-local",
            "value": "sha256:4567ef014567ef014567ef014567ef014567ef014567ef014567ef014567ef01",
            "hash_status": "computed_local_preview"
        });
        upsert_project(
            &mut connection,
            "project:test-local",
            "Test Local Project",
            &model,
            &editor_intents,
            &proposal,
            &selected_review_target,
            &mechanics_result,
            &analysis_run,
            &model_hash,
            &project_envelope_hash,
            &serde_json::json!([]),
        )
        .expect("project snapshot saves");

        let loaded = load_project(&connection, Some("project:test-local"))
            .expect("project snapshot loads")
            .expect("saved project exists");
        assert_eq!(loaded.project_id, "project:test-local");
        assert_eq!(loaded.project_name, "Test Local Project");
        assert_eq!(loaded.model, model);
        assert_eq!(loaded.editor_intents, editor_intents);
        assert_eq!(loaded.proposal, proposal);
        assert_eq!(loaded.selected_review_target, selected_review_target);
        assert_eq!(loaded.mechanics_result, mechanics_result);
        assert_eq!(loaded.analysis_run, analysis_run);
        assert_eq!(loaded.model_hash, model_hash);
        assert_eq!(loaded.project_envelope_hash, project_envelope_hash);
        assert_eq!(
            loaded.editor_intents[0]["validation"]["application_status"],
            json!("not_applied")
        );
        assert_eq!(
            loaded.editor_intents[0]["audit_boundary"]["mutates_accepted_model_state"],
            json!(false)
        );
        assert_eq!(
            loaded.editor_intents[0]["professional_boundary"]["software_makes_compliance_claim"],
            json!(false)
        );
        assert_eq!(
            loaded.proposal["operation"]["operation_id"],
            json!("op:review-computed-diagnostic")
        );
        assert_eq!(
            loaded.proposal["validation"]["application_status"],
            json!("not_applied")
        );
        assert_eq!(
            loaded.proposal["audit_boundary"]["mutates_accepted_model_state"],
            json!(false)
        );
        assert_eq!(
            loaded.selected_review_target["id"],
            json!("result:stress:pipe-P-120:end-j:torsional-shear")
        );
        assert_eq!(
            loaded.analysis_run["analysis_run"]["run_id"],
            json!("run:preview-linear-static-001")
        );
        let summary = project_summary(
            "project:test-local".to_string(),
            "Test Local Project".to_string(),
            PathBuf::from(":memory:"),
            &migration,
            &editor_intents,
            &proposal,
            &selected_review_target,
            &mechanics_result,
            &analysis_run,
            &model_hash,
            &project_envelope_hash,
            "Saved test project snapshot.".to_string(),
        );
        assert_eq!(summary.editor_intent_count, 1);
        assert_eq!(summary.proposal_count, 1);
        assert_eq!(summary.selected_review_target_count, 1);
        assert_eq!(
            summary.selected_review_target_ref,
            "result: result:stress:pipe-P-120:end-j:torsional-shear"
        );
        assert_eq!(summary.persisted_mechanics_result_count, 1);
        assert_eq!(summary.persisted_analysis_run_count, 1);
        assert_eq!(
            summary.persisted_analysis_run_ref,
            "run:preview-linear-static-001"
        );
        assert_eq!(summary.persisted_model_hash_count, 1);
        assert_eq!(
            summary.persisted_model_hash_ref,
            "sha256:0123abcd0123abcd0123abcd0123abcd0123abcd0123abcd0123abcd0123abcd"
        );
        assert_eq!(summary.persisted_project_envelope_hash_count, 1);
        assert_eq!(
            summary.persisted_project_envelope_hash_ref,
            "sha256:4567ef014567ef014567ef014567ef014567ef014567ef014567ef014567ef01"
        );
        let empty_summary = project_summary(
            "project:test-local".to_string(),
            "Test Local Project".to_string(),
            PathBuf::from(":memory:"),
            &migration,
            &json!([]),
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            "Saved empty test project snapshot.".to_string(),
        );
        assert_eq!(empty_summary.persisted_mechanics_result_count, 0);
        assert_eq!(empty_summary.persisted_analysis_run_count, 0);
        assert_eq!(empty_summary.persisted_analysis_run_ref, "not_persisted");
        assert_eq!(empty_summary.persisted_model_hash_count, 0);
        assert_eq!(empty_summary.persisted_model_hash_ref, "not_persisted");
        assert_eq!(empty_summary.persisted_project_envelope_hash_count, 0);
        assert_eq!(
            empty_summary.persisted_project_envelope_hash_ref,
            "not_persisted"
        );

        let indexed_count: i64 = connection
            .query_row(
                "SELECT count(*) FROM local_project_fts WHERE local_project_fts MATCH ?1",
                params!["editor"],
                |row| row.get(0),
            )
            .expect("fts5 query succeeds");
        assert_eq!(indexed_count, 1);
        let proposal_indexed_count: i64 = connection
            .query_row(
                "SELECT count(*) FROM local_project_fts WHERE local_project_fts MATCH ?1",
                params!["review"],
                |row| row.get(0),
            )
            .expect("fts5 proposal query succeeds");
        assert_eq!(proposal_indexed_count, 1);
        let target_indexed_count: i64 = connection
            .query_row(
                "SELECT count(*) FROM local_project_fts WHERE local_project_fts MATCH ?1",
                params!["torsional"],
                |row| row.get(0),
            )
            .expect("fts5 selected target query succeeds");
        assert_eq!(target_indexed_count, 1);
    }

    #[test]
    fn store_migration_ledger_reports_fresh_open_and_idempotent_reopen_evidence() {
        let connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        let first_open = apply_store_migrations(&connection).expect("fresh store migrations apply");
        assert_eq!(first_open.migration_framework, STORE_MIGRATION_FRAMEWORK);
        assert_eq!(first_open.store_schema_version_before_open, 0);
        assert_eq!(first_open.store_schema_version, STORE_SCHEMA_TARGET_VERSION);
        assert_eq!(
            first_open.store_schema_target_version,
            STORE_SCHEMA_TARGET_VERSION
        );
        assert_eq!(first_open.pending_migration_count, 0);
        assert_eq!(
            first_open.migrations_applied_on_open,
            vec![
                "store-v1-base-project-and-fts-tables",
                "store-v2-editor-intents-column",
                "store-v3-proposal-column",
                "store-v4-selected-review-target-column",
                "store-v5-mechanics-result-column",
                "store-v6-analysis-run-column",
                "store-v7-model-hash-column",
                "store-v8-project-envelope-hash-column",
                "store-v9-model-document-migration-ledger-column",
            ]
        );
        assert_eq!(
            first_open.migration_status,
            "migrated_on_open_store_schema_v0_to_v9"
        );

        let second_open =
            apply_store_migrations(&connection).expect("current store reopen succeeds");
        assert_eq!(
            second_open.store_schema_version_before_open,
            STORE_SCHEMA_TARGET_VERSION
        );
        assert_eq!(
            second_open.store_schema_version,
            STORE_SCHEMA_TARGET_VERSION
        );
        assert!(second_open.migrations_applied_on_open.is_empty());
        assert_eq!(second_open.pending_migration_count, 0);
        assert_eq!(
            second_open.migration_status,
            "current_store_schema_v9_no_pending_migrations"
        );
    }

    #[test]
    fn store_migration_ledger_reconciles_legacy_store_and_preserves_rows() {
        let connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        connection
            .execute_batch(
                "
                CREATE TABLE local_projects (
                    project_id TEXT PRIMARY KEY,
                    project_name TEXT NOT NULL,
                    model_json TEXT NOT NULL,
                    created_at_unix INTEGER NOT NULL,
                    updated_at_unix INTEGER NOT NULL
                );
                INSERT INTO local_projects (
                    project_id, project_name, model_json, created_at_unix, updated_at_unix
                ) VALUES (
                    'project:legacy-local',
                    'Legacy Local Project',
                    '{\"project\":{\"id\":\"project:legacy-local\",\"name\":\"Legacy Local Project\"}}',
                    100,
                    100
                );
                ",
            )
            .expect("legacy pre-ledger store shape creates");
        assert_eq!(
            store_user_version(&connection).expect("user_version reads"),
            0
        );

        let migration = apply_store_migrations(&connection).expect("legacy store migrations apply");
        assert_eq!(migration.store_schema_version_before_open, 0);
        assert_eq!(migration.store_schema_version, STORE_SCHEMA_TARGET_VERSION);
        assert_eq!(migration.migrations_applied_on_open.len(), 9);
        assert_eq!(
            migration.migration_status,
            "migrated_on_open_store_schema_v0_to_v9"
        );

        let loaded = load_project(&connection, Some("project:legacy-local"))
            .expect("legacy project loads after migration")
            .expect("legacy project row preserved");
        assert_eq!(loaded.project_id, "project:legacy-local");
        assert_eq!(loaded.project_name, "Legacy Local Project");
        assert_eq!(loaded.editor_intents, json!([]));
        assert_eq!(loaded.proposal, Value::Null);
        assert_eq!(loaded.selected_review_target, Value::Null);
        assert_eq!(loaded.mechanics_result, Value::Null);
        assert_eq!(loaded.analysis_run, Value::Null);
        assert_eq!(loaded.model_hash, Value::Null);
        assert_eq!(loaded.project_envelope_hash, Value::Null);
    }

    #[test]
    fn store_migration_ledger_stages_v7_store_to_v8_applying_only_pending_migration() {
        let connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        connection
            .execute_batch(
                "
                CREATE TABLE local_projects (
                    project_id TEXT PRIMARY KEY,
                    project_name TEXT NOT NULL,
                    model_json TEXT NOT NULL,
                    editor_intents_json TEXT NOT NULL DEFAULT '[]',
                    proposal_json TEXT NOT NULL DEFAULT 'null',
                    selected_review_target_json TEXT NOT NULL DEFAULT 'null',
                    mechanics_result_json TEXT NOT NULL DEFAULT 'null',
                    analysis_run_json TEXT NOT NULL DEFAULT 'null',
                    model_hash_json TEXT NOT NULL DEFAULT 'null',
                    created_at_unix INTEGER NOT NULL,
                    updated_at_unix INTEGER NOT NULL
                );
                CREATE VIRTUAL TABLE local_project_fts USING fts5(
                    project_id UNINDEXED,
                    project_name,
                    model_text
                );
                INSERT INTO local_projects (
                    project_id, project_name, model_json, created_at_unix, updated_at_unix
                ) VALUES (
                    'project:v7-local',
                    'V7 Local Project',
                    '{\"project\":{\"id\":\"project:v7-local\",\"name\":\"V7 Local Project\"}}',
                    100,
                    100
                );
                PRAGMA user_version = 7;
                ",
            )
            .expect("v7-shape store creates");
        assert_eq!(
            store_user_version(&connection).expect("user_version reads"),
            7
        );

        let migration = apply_store_migrations(&connection).expect("v7 store migrations apply");
        assert_eq!(migration.store_schema_version_before_open, 7);
        assert_eq!(migration.store_schema_version, STORE_SCHEMA_TARGET_VERSION);
        assert_eq!(
            migration.migrations_applied_on_open,
            vec![
                "store-v8-project-envelope-hash-column",
                "store-v9-model-document-migration-ledger-column"
            ]
        );
        assert_eq!(
            migration.migration_status,
            "migrated_on_open_store_schema_v7_to_v9"
        );

        let loaded = load_project(&connection, Some("project:v7-local"))
            .expect("v7 project loads after staged migration")
            .expect("v7 project row preserved");
        assert_eq!(loaded.project_id, "project:v7-local");
        assert_eq!(loaded.model_hash, Value::Null);
        assert_eq!(loaded.project_envelope_hash, Value::Null);
        assert_eq!(loaded.model_migration_ledger, json!([]));
    }

    #[test]
    fn local_project_store_lists_project_index_ordered_by_most_recent_update() {
        let mut connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        apply_store_migrations(&connection).expect("project store migrations apply");
        assert_eq!(
            list_projects(&connection).expect("empty store lists").len(),
            0
        );

        let first_model = json!({
            "project": { "id": "project:test-alpha", "name": "Alpha Project" }
        });
        let second_model = json!({
            "project": { "id": "project:test-beta", "name": "Beta Project" }
        });
        upsert_project(
            &mut connection,
            "project:test-alpha",
            "Alpha Project",
            &first_model,
            &json!([]),
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &serde_json::json!([]),
        )
        .expect("first project saves");
        upsert_project(
            &mut connection,
            "project:test-beta",
            "Beta Project",
            &second_model,
            &json!([]),
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &serde_json::json!([]),
        )
        .expect("second project saves");
        connection
            .execute(
                "UPDATE local_projects SET updated_at_unix = updated_at_unix + 60 WHERE project_id = ?1",
                params!["project:test-beta"],
            )
            .expect("recency update applies");

        let entries = list_projects(&connection).expect("populated store lists");
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].project_id, "project:test-beta");
        assert_eq!(entries[0].project_name, "Beta Project");
        assert_eq!(entries[0].storage_mode, "local_sqlite");
        assert!(entries[0].created_at_unix > 0);
        assert!(entries[0].updated_at_unix >= entries[0].created_at_unix);
        assert_eq!(entries[1].project_id, "project:test-alpha");
        assert!(entries[0].updated_at_unix > entries[1].updated_at_unix);

        let opened_by_id = load_project(&connection, Some("project:test-alpha"))
            .expect("open by id loads")
            .expect("alpha project exists");
        assert_eq!(opened_by_id.project_id, "project:test-alpha");
        assert_eq!(opened_by_id.model, first_model);
        let opened_most_recent = load_project(&connection, None)
            .expect("open most recent loads")
            .expect("most recent project exists");
        assert_eq!(opened_most_recent.project_id, "project:test-beta");
    }

    #[test]
    fn agent_proposal_preserves_selected_review_target_without_mutation_or_claims() {
        let result = json!({
            "model_ref": "project:invented-loop-01",
            "results": [
                {
                    "id": "result:force:pipe-P-120:axial",
                    "kind": "element_local_axial_force"
                }
            ],
            "diagnostics": [
                {
                    "id": "diagnostic:physics:high-displacement-review",
                    "severity": "warning"
                }
            ]
        });
        let output = build_sample_agent_proposal(
            result,
            Some(SelectedReviewTarget {
                target_type: "diagnostic".to_string(),
                id: "diagnostic:physics:high-displacement-review".to_string(),
            }),
        );
        let proposal = output.get("proposal").expect("proposal envelope exists");
        assert_eq!(
            proposal["operation"]["affected_entity_ids"][0],
            json!("diagnostic:physics:high-displacement-review")
        );
        assert_eq!(
            proposal["operation"]["changes"][0]["target_ref"],
            json!("diagnostic:physics:high-displacement-review")
        );
        assert_eq!(
            proposal["operation"]["operation_status"],
            json!("draft_user_review_required")
        );
        assert_eq!(
            proposal["validation"]["application_status"],
            json!("not_applied")
        );
        assert!(proposal["rationale"]
            .as_str()
            .expect("rationale is text")
            .contains("selected review reference is diagnostic:physics:high-displacement-review"));
        assert_eq!(output["accepted_model_mutated"], json!(false));
        assert_eq!(
            proposal["audit_boundary"]["mutates_accepted_model_state"],
            json!(false)
        );
        assert_eq!(
            proposal["professional_boundary"]["software_makes_compliance_claim"],
            json!(false)
        );
        assert_eq!(
            proposal["professional_boundary"]["software_makes_approval_claim"],
            json!(false)
        );
    }

    #[test]
    fn solve_job_seam_completes_and_reports_result_without_cancellation() {
        let registry = SolveJobRegistry::default();
        let receipt = start_solve_job(&registry.jobs).expect("job starts");
        assert_eq!(receipt.state, "queued");
        assert!(receipt
            .backend_cancellation_token
            .starts_with("solve-cancel-token-"));
        assert_eq!(
            receipt.cancellation_scope,
            "cooperative_checkpoints_not_preemptive"
        );

        execute_solve_job(
            &registry.jobs,
            &receipt.job_id,
            resolve_solve_model_payload(None),
        );

        let status = solve_job_status(&registry.jobs, &receipt.job_id).expect("status available");
        assert_eq!(status.state, "completed");
        assert!(!status.cancellation_requested);
        assert_eq!(status.cancellation_status, "not_requested");
        let result = status.result.expect("completed job publishes a result");
        assert!(
            result["results"]
                .as_array()
                .expect("result rows present")
                .len()
                > 0
        );
        assert_eq!(status.error_message, None);
    }

    #[test]
    fn run_preview_mechanics_uses_supplied_model_payload() {
        let mut model =
            read_fixture("invented_preview_model.json").expect("bundled preview model loads");
        model["project"]["id"] = json!("project:edited-solve-command");
        model["materials"][0]["elastic_modulus"]["value"] = json!(195000000000.0);

        let solved = run_preview_mechanics(Some(model)).expect("supplied model solves");

        assert_eq!(solved["model_ref"], json!("project:edited-solve-command"));
        assert_eq!(solved["status"]["mechanics"], json!("MECHANICS_SOLVED"));
        assert!(
            solved["results"]
                .as_array()
                .expect("result rows present")
                .len()
                > 0
        );
    }

    #[test]
    fn run_preview_mechanics_reports_blank_model_incomplete_without_defaults() {
        let blank = json!({
            "schema_version": "0.1.0",
            "document_kind": "openpipestress.product_preview.model",
            "data_boundary": {
                "public_examples_policy": "blank_user_created_local_document_no_bundled_engineering_values",
                "protected_source_policy": "no_protected_standards_content_inserted",
                "private_data_policy": "local_user_document_not_committed_to_repository",
                "professional_boundary": "human_review_required_no_software_approval_claim"
            },
            "project": {
                "id": "project:blank-local-rust-test",
                "name": "Blank Local Model",
                "description": "User-created local blank model document.",
                "units": {"length": "m", "force": "N"}
            },
            "analysis_status": {
                "mechanics": "MODEL_INCOMPLETE",
                "rule_check": "RULE_INPUTS_INCOMPLETE",
                "professional_acceptance": "NOT_PROVIDED"
            },
            "materials": [],
            "nodes": [],
            "pipe_segments": [],
            "supports": [],
            "components": [],
            "load_cases": [],
            "combinations": [],
            "diagnostics": []
        });

        let solved = run_preview_mechanics(Some(blank))
            .expect("blank model produces an honest result envelope");
        let diagnostics = solved["diagnostics"]
            .as_array()
            .expect("diagnostics should be present");

        assert_eq!(solved["model_ref"], json!("project:blank-local-rust-test"));
        assert_eq!(solved["status"]["mechanics"], json!("MODEL_INCOMPLETE"));
        assert!(solved["results"]
            .as_array()
            .expect("result rows array present")
            .is_empty());
        assert!(diagnostics
            .iter()
            .any(|item| item["code"] == json!("NODE_INPUT_MISSING")));
        assert!(diagnostics
            .iter()
            .any(|item| item["code"] == json!("PIPE_INPUT_MISSING")));
        assert!(diagnostics
            .iter()
            .any(|item| item["code"] == json!("LOAD_INPUT_MISSING")));
    }

    #[test]
    fn solve_job_seam_uses_supplied_model_payload() {
        let registry = SolveJobRegistry::default();
        let receipt = start_solve_job(&registry.jobs).expect("job starts");
        let mut model =
            read_fixture("invented_preview_model.json").expect("bundled preview model loads");
        model["project"]["id"] = json!("project:edited-solve-job");
        model["materials"][0]["elastic_modulus"]["value"] = json!(195000000000.0);

        execute_solve_job(&registry.jobs, &receipt.job_id, Ok(model));

        let status = solve_job_status(&registry.jobs, &receipt.job_id).expect("status available");
        assert_eq!(status.state, "completed");
        let result = status.result.expect("completed job publishes a result");
        assert_eq!(result["model_ref"], json!("project:edited-solve-job"));
        assert_eq!(result["status"]["mechanics"], json!("MECHANICS_SOLVED"));
        assert!(
            result["results"]
                .as_array()
                .expect("result rows present")
                .len()
                > 0
        );
        assert_eq!(status.error_message, None);
    }

    #[test]
    fn solve_job_cancelled_before_execution_never_starts_the_solver() {
        let registry = SolveJobRegistry::default();
        let receipt = start_solve_job(&registry.jobs).expect("job starts");

        let cancel = cancel_solve_job(
            &registry.jobs,
            &receipt.job_id,
            &receipt.backend_cancellation_token,
        )
        .expect("cancel receipt");
        assert!(cancel.accepted);
        assert_eq!(
            cancel.cancellation_status,
            "cancellation_requested_awaiting_cooperative_checkpoint"
        );
        assert!(!cancel.cancellation_success_claimed);

        execute_solve_job(
            &registry.jobs,
            &receipt.job_id,
            resolve_solve_model_payload(None),
        );

        let status = solve_job_status(&registry.jobs, &receipt.job_id).expect("status available");
        assert_eq!(status.state, "cancelled");
        assert!(status.cancellation_requested);
        assert_eq!(status.cancellation_status, "cancelled_before_solver_start");
        assert_eq!(status.result, None);
    }

    #[test]
    fn solve_job_cancelled_mid_run_discards_result_at_publication_checkpoint() {
        let registry = SolveJobRegistry::default();
        let receipt = start_solve_job(&registry.jobs).expect("job starts");
        {
            let mut table = registry.jobs.lock().expect("registry lock");
            table
                .records
                .get_mut(&receipt.job_id)
                .expect("record present")
                .state = "running".to_string();
        }

        let cancel = cancel_solve_job(
            &registry.jobs,
            &receipt.job_id,
            &receipt.backend_cancellation_token,
        )
        .expect("cancel receipt");
        assert!(cancel.accepted);
        assert_eq!(cancel.job_state, "running");

        publish_solve_outcome(
            &registry.jobs,
            &receipt.job_id,
            solve_preview_mechanics(resolve_solve_model_payload(None).expect("fixture loads")),
        );

        let status = solve_job_status(&registry.jobs, &receipt.job_id).expect("status available");
        assert_eq!(status.state, "cancelled");
        assert_eq!(
            status.cancellation_status,
            "cancelled_before_result_publication_result_discarded"
        );
        assert_eq!(status.result, None);
    }

    #[test]
    fn solve_job_cancellation_after_completion_is_rejected_without_success_claim() {
        let registry = SolveJobRegistry::default();
        let receipt = start_solve_job(&registry.jobs).expect("job starts");
        execute_solve_job(
            &registry.jobs,
            &receipt.job_id,
            resolve_solve_model_payload(None),
        );

        let cancel = cancel_solve_job(
            &registry.jobs,
            &receipt.job_id,
            &receipt.backend_cancellation_token,
        )
        .expect("cancel receipt");
        assert!(!cancel.accepted);
        assert_eq!(
            cancel.cancellation_status,
            "request_after_terminal_state_completed_no_cancellation_performed"
        );
        assert_eq!(cancel.job_state, "completed");
        assert!(!cancel.cancellation_success_claimed);

        let status = solve_job_status(&registry.jobs, &receipt.job_id).expect("status available");
        assert_eq!(status.state, "completed");
        assert!(status.result.is_some());
    }

    #[test]
    fn solve_job_cancellation_with_invalid_token_is_rejected_and_does_not_flag_the_job() {
        let registry = SolveJobRegistry::default();
        let receipt = start_solve_job(&registry.jobs).expect("job starts");

        let cancel = cancel_solve_job(&registry.jobs, &receipt.job_id, "wrong-token")
            .expect("cancel receipt");
        assert!(!cancel.accepted);
        assert_eq!(
            cancel.cancellation_status,
            "rejected_invalid_cancellation_token"
        );

        let status = solve_job_status(&registry.jobs, &receipt.job_id).expect("status available");
        assert!(!status.cancellation_requested);
        assert_eq!(status.cancellation_status, "not_requested");

        assert!(solve_job_status(&registry.jobs, "backend-solve-job-missing").is_err());
        assert!(cancel_solve_job(&registry.jobs, "backend-solve-job-missing", "token").is_err());
    }

    #[test]
    fn solve_job_with_invalid_model_payload_fails_with_recorded_error() {
        let registry = SolveJobRegistry::default();
        let receipt = start_solve_job(&registry.jobs).expect("job starts");
        execute_solve_job(
            &registry.jobs,
            &receipt.job_id,
            Ok(json!({"not_a_preview_model": true})),
        );

        let status = solve_job_status(&registry.jobs, &receipt.job_id).expect("status available");
        assert_eq!(status.state, "failed");
        assert_eq!(status.result, None);
        assert!(status.error_message.is_some());
    }

    fn fixture_inspector_intent(before: &str, after: &str) -> Value {
        json!({
            "operation_id": "op:editor-intent-material-elastic-modulus",
            "operation_kind": "modify",
            "operation_status": "proposed",
            "author_type": "user",
            "target": { "object_type": "Material", "ref": "material:invented-carbon-steel" },
            "change": {
                "change_id": "change:material-elastic-modulus",
                "change_kind": "set_field",
                "field_label": "Elastic modulus",
                "field_path": "elastic_modulus.value",
                "before": before,
                "after": after,
                "unit": "Pa",
                "dimension": "stress",
                "source_note": "unit metadata required"
            },
            "validation": {
                "schema_validation": "not_run",
                "constraint_validation": "not_run",
                "unit_validation": "not_run",
                "diff_preview_status": "not_generated",
                "application_status": "not_applied"
            },
            "audit_boundary": {
                "mutation_route": "structured_operations_only",
                "direct_model_mutation_allowed": false,
                "requires_user_acceptance": true,
                "mutates_accepted_model_state": false
            },
            "professional_boundary": {
                "human_review_required": true,
                "software_makes_compliance_claim": false,
                "software_makes_certification_claim": false,
                "software_makes_sealing_claim": false,
                "software_makes_approval_claim": false,
                "software_makes_authentication_claim": false
            },
            "rationale": "command-path test edit"
        })
    }

    fn fixture_load_magnitude_intent(before: &str, after: &str) -> Value {
        json!({
            "operation_id": "op:test-load-L-100-Y-magnitude",
            "operation_kind": "modify",
            "operation_status": "proposed",
            "author_type": "user",
            "target": { "object_type": "Load", "ref": "load:L-100" },
            "change": {
                "change_id": "change:test-load-L-100-Y-magnitude",
                "change_kind": "update_load",
                "field_label": "load:L-100-Y magnitude",
                "field_path": "primitive_loads.1.magnitude.value",
                "before": before,
                "after": after,
                "unit": "N",
                "dimension": "force",
                "source_note": "explicit load data persistence/solve regression"
            },
            "validation": {
                "schema_validation": "not_run",
                "constraint_validation": "not_run",
                "unit_validation": "not_run",
                "diff_preview_status": "not_generated",
                "application_status": "not_applied"
            },
            "audit_boundary": {
                "mutation_route": "structured_operations_only",
                "direct_model_mutation_allowed": false,
                "requires_user_acceptance": true,
                "mutates_accepted_model_state": false
            },
            "professional_boundary": {
                "human_review_required": true,
                "software_makes_compliance_claim": false,
                "software_makes_certification_claim": false,
                "software_makes_sealing_claim": false,
                "software_makes_approval_claim": false,
                "software_makes_authentication_claim": false
            },
            "rationale": "saved edited load data solve regression"
        })
    }

    fn result_value(result: &Value, result_id: &str) -> f64 {
        result["results"]
            .as_array()
            .expect("result rows present")
            .iter()
            .find(|item| item["id"].as_str() == Some(result_id))
            .and_then(|item| item["value"].as_f64())
            .unwrap_or_else(|| panic!("missing numeric result row {result_id}"))
    }

    #[test]
    fn apply_model_operation_command_applies_inspector_intent_to_bundled_fixture_model() {
        let model =
            read_fixture("invented_preview_model.json").expect("bundled preview model loads");
        let intent = fixture_inspector_intent("200000000000", "195000000000");
        let outcome =
            apply_model_operation(model.clone(), intent, None).expect("command returns outcome");

        assert_eq!(
            outcome["validation"]["application_status"],
            "applied_to_session_model"
        );
        assert_eq!(
            outcome["acceptance"]["acceptance_basis"],
            "user_initiated_apply_in_local_session"
        );
        assert_eq!(
            outcome["acceptance"]["acceptance_is_professional_approval"],
            json!(false)
        );
        assert_eq!(
            outcome["applied_model"]["materials"][0]["elastic_modulus"]["value"],
            json!(195000000000.0)
        );
        // The input model the command received is untouched; the applied model
        // is a new document, and downstream solve still accepts it.
        assert_eq!(
            model["materials"][0]["elastic_modulus"]["value"],
            json!(200000000000_i64)
        );
        let solved = run_preview_mechanics(Some(outcome["applied_model"].clone()))
            .expect("edited model still solves through the preview mechanics path");
        assert!(solved["results"]
            .as_array()
            .map(|rows| !rows.is_empty())
            .unwrap_or(false));
    }

    #[test]
    fn saved_edited_load_model_round_trips_and_solves_from_restored_payload() {
        let mut connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        apply_store_migrations(&connection).expect("store migrations apply");

        let mut model =
            read_fixture("invented_preview_model.json").expect("bundled preview model loads");
        let baseline_solve =
            solve_preview_mechanics(model.clone()).expect("baseline fixture model solves");
        let baseline_displacement = result_value(&baseline_solve, "result:disp:node-N-140");
        model["project"]["id"] = json!("project:edited-load-roundtrip");
        model["project"]["name"] = json!("Edited Load Roundtrip");

        let outcome =
            apply_model_operation(model, fixture_load_magnitude_intent("350", "425"), None)
                .expect("load magnitude operation applies");
        assert_eq!(
            outcome["validation"]["application_status"],
            json!("applied_to_session_model")
        );
        assert_eq!(
            outcome["acceptance"]["persistence_status"],
            json!("session_state_only_not_yet_saved")
        );
        let edited_model = outcome["applied_model"].clone();
        assert_eq!(
            edited_model["load_cases"][0]["primitive_loads"][1]["magnitude"]["value"].as_f64(),
            Some(425.0)
        );

        let (model_to_persist, document_status, ledger) = prepare_model_document_for_persist(
            &connection,
            "project:edited-load-roundtrip",
            edited_model,
            &Value::Null,
            &json!({ "value": "sha256:edited-load-roundtrip-model" }),
        )
        .expect("edited current model document is ready to persist");
        assert_eq!(document_status.status, "current");
        assert_eq!(document_status.persistence_state, "stored_document_current");
        assert_eq!(ledger, json!([]));

        upsert_project(
            &mut connection,
            "project:edited-load-roundtrip",
            "Edited Load Roundtrip",
            &model_to_persist,
            &json!([]),
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &json!({ "value": "sha256:edited-load-roundtrip-model" }),
            &Value::Null,
            &ledger,
        )
        .expect("edited load model persists to the local store");

        let restored = load_project(&connection, Some("project:edited-load-roundtrip"))
            .expect("store read succeeds")
            .expect("edited project row exists");
        assert_eq!(restored.project_id, "project:edited-load-roundtrip");
        assert_eq!(
            restored.model["load_cases"][0]["primitive_loads"][1]["magnitude"]["value"].as_f64(),
            Some(425.0)
        );
        assert_eq!(
            restored.model_hash["value"],
            json!("sha256:edited-load-roundtrip-model")
        );

        let restored_solve =
            solve_preview_mechanics(restored.model).expect("restored edited model solves");
        assert_eq!(
            restored_solve["model_ref"],
            json!("project:edited-load-roundtrip")
        );
        assert_eq!(
            restored_solve["status"]["mechanics"],
            json!("MECHANICS_SOLVED")
        );
        assert!(
            restored_solve["results"]
                .as_array()
                .expect("result rows present")
                .len()
                > 0
        );
        let edited_displacement = result_value(&restored_solve, "result:disp:node-N-140");
        assert_ne!(edited_displacement, baseline_displacement);
    }

    #[test]
    fn persist_preparation_refuses_newer_and_unsupported_model_documents() {
        let mut connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        apply_store_migrations(&connection).expect("store migrations apply");
        let _ = &mut connection;

        let newer = json!({ "schema_version": "0.2.0", "project": { "id": "project:newer", "name": "Newer" } });
        let error = prepare_model_document_for_persist(
            &connection,
            "project:newer",
            newer,
            &Value::Null,
            &Value::Null,
        )
        .expect_err("newer-than-supported documents must be refused");
        assert!(error.contains("newer_than_supported"), "{error}");

        let unsupported =
            json!({ "project": { "id": "project:unversioned", "name": "Unversioned" } });
        let error = prepare_model_document_for_persist(
            &connection,
            "project:unversioned",
            unsupported,
            &Value::Null,
            &Value::Null,
        )
        .expect_err("documents without a valid schema_version must be refused");
        assert!(error.contains("unsupported_schema"), "{error}");
    }

    #[test]
    fn persist_preparation_keeps_current_documents_without_ledger_records() {
        let connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        apply_store_migrations(&connection).expect("store migrations apply");
        let model = json!({ "schema_version": "0.1.0", "project": { "id": "project:current", "name": "Current" } });
        let (persisted, status, ledger) = prepare_model_document_for_persist(
            &connection,
            "project:current",
            model.clone(),
            &Value::Null,
            &Value::Null,
        )
        .expect("current documents persist");
        assert_eq!(persisted, model);
        assert_eq!(status.status, "current");
        assert_eq!(status.persistence_state, "stored_document_current");
        assert_eq!(ledger, json!([]));
    }

    #[test]
    fn saving_an_open_time_migrated_document_appends_a_ledger_record_with_pre_and_post_hashes() {
        let mut connection = Connection::open_in_memory().expect("in-memory sqlite opens");
        apply_store_migrations(&connection).expect("store migrations apply");
        let stored_model = json!({ "schema_version": "0.1.0", "project": { "id": "project:migrated", "name": "Migrated" } });
        upsert_project(
            &mut connection,
            "project:migrated",
            "Migrated",
            &stored_model,
            &json!([]),
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &Value::Null,
            &json!({ "value": "sha256:pre-migration" }),
            &Value::Null,
            &serde_json::json!([]),
        )
        .expect("seed row persists");

        let open_time_status = json!({
            "status": "migrated",
            "source_schema_version": "0.0.9",
            "target_schema_version": "0.1.0",
            "applied_migration_ids": ["model-doc-0.0.9-to-0.1.0-test"]
        });
        let (persisted, status, ledger) = prepare_model_document_for_persist(
            &connection,
            "project:migrated",
            stored_model.clone(),
            &open_time_status,
            &json!({ "value": "sha256:post-migration" }),
        )
        .expect("migrated document persists with evidence");

        assert_eq!(persisted, stored_model);
        assert_eq!(status.status, "migrated");
        assert_eq!(status.persistence_state, "persisted_with_ledger_record");
        let records = ledger.as_array().expect("ledger array");
        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0]["pre_migration_model_hash"],
            json!("sha256:pre-migration")
        );
        assert_eq!(
            records[0]["post_migration_model_hash"],
            json!("sha256:post-migration")
        );
        assert_eq!(records[0]["source_schema_version"], json!("0.0.9"));
        assert_eq!(
            records[0]["applied_migration_ids"],
            json!(["model-doc-0.0.9-to-0.1.0-test"])
        );
        assert_eq!(records[0]["destructive_rewrite"], json!(false));
        assert_eq!(
            records[0]["professional_boundary"]["software_makes_compliance_claim"],
            json!(false)
        );
    }

    #[test]
    fn validate_model_operation_command_blocks_stale_intent_without_mutation() {
        let model =
            read_fixture("invented_preview_model.json").expect("bundled preview model loads");
        let intent = fixture_inspector_intent("123", "195000000000");
        let outcome =
            validate_model_operation(model, intent, None).expect("command returns outcome");

        assert_eq!(outcome["validation"]["application_status"], "not_applied");
        assert_eq!(
            outcome["validation"]["before_state_validation"],
            "blocked_stale"
        );
        assert!(outcome["applied_model"].is_null());
        let codes: Vec<&str> = outcome["diagnostics"]
            .as_array()
            .expect("diagnostics array")
            .iter()
            .filter_map(|item| item["code"].as_str())
            .collect();
        assert!(codes.contains(&"OP-STALE-BEFORE-VALUE"));
    }
}
