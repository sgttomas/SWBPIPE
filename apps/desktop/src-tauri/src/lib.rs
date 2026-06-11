use open_pipe_stress_product_physics::{run_linear_static_preview, LinearStaticPreviewRequest};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
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

const STORE_SCHEMA_TARGET_VERSION: i64 = 7;
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
            INSERT INTO local_projects (project_id, project_name, model_json, editor_intents_json, proposal_json, selected_review_target_json, mechanics_result_json, analysis_run_json, model_hash_json, created_at_unix, updated_at_unix)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?10)
            ON CONFLICT(project_id) DO UPDATE SET
                project_name = excluded.project_name,
                model_json = excluded.model_json,
                editor_intents_json = excluded.editor_intents_json,
                proposal_json = excluded.proposal_json,
                selected_review_target_json = excluded.selected_review_target_json,
                mechanics_result_json = excluded.mechanics_result_json,
                analysis_run_json = excluded.analysis_run_json,
                model_hash_json = excluded.model_hash_json,
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
        ))
    }
    let row = match project_id {
        Some(id) => connection
            .query_row(
                "
                SELECT project_id, project_name, model_json, editor_intents_json, proposal_json, selected_review_target_json, mechanics_result_json, analysis_run_json, model_hash_json
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
                SELECT project_id, project_name, model_json, editor_intents_json, proposal_json, selected_review_target_json, mechanics_result_json, analysis_run_json, model_hash_json
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
        message,
    }
}

#[tauri::command]
fn load_preview_model() -> Result<Value, String> {
    read_fixture("invented_preview_model.json")
}

#[tauri::command]
fn load_design_knowledge() -> Result<Value, String> {
    read_fixture("invented_design_knowledge.json")
}

#[tauri::command]
fn run_preview_mechanics(model: Option<Value>) -> Result<Value, String> {
    let model_payload = match model {
        Some(value) => value,
        None => read_fixture("invented_preview_model.json")?,
    };
    let model: open_pipe_stress_product_physics::PreviewModel =
        serde_json::from_value(model_payload).map_err(|error| error.to_string())?;
    serde_json::to_value(run_linear_static_preview(LinearStaticPreviewRequest {
        model,
        materials: vec![],
    }))
    .map_err(|error| error.to_string())
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
            "Created local SQLite project snapshot without external file copies.".to_string(),
        ),
        model,
        editor_intents,
        proposal,
        selected_review_target,
        mechanics_result,
        analysis_run,
        model_hash,
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
                    "Opened local SQLite project snapshot.".to_string(),
                ),
                model: record.model,
                editor_intents: record.editor_intents,
                proposal: record.proposal,
                selected_review_target: record.selected_review_target,
                mechanics_result: record.mechanics_result,
                analysis_run: record.analysis_run,
                model_hash: record.model_hash,
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
    upsert_project(
        &mut connection,
        &request.project_id,
        &request.project_name,
        &request.model,
        &editor_intents,
        &proposal,
        &selected_review_target,
        &mechanics_result,
        &analysis_run,
        &model_hash,
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
            "Saved current project model snapshot without external file copies.".to_string(),
        ),
        model: request.model,
        editor_intents,
        proposal,
        selected_review_target,
        mechanics_result,
        analysis_run,
        model_hash,
    })
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            load_preview_model,
            load_design_knowledge,
            run_preview_mechanics,
            sample_agent_proposal,
            get_local_storage_capability,
            create_local_project,
            open_local_project,
            list_local_projects,
            save_local_project
        ])
        .run(tauri::generate_context!())
        .expect("error while running OpenPipeStress technical preview");
}

#[cfg(test)]
mod tests {
    use super::*;

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
            "canonicalization": "jcs_like_sorted_object_keys",
            "payload_scope": "model_payload",
            "payload_ref": "project:test-local",
            "value": "sha256:0123abcd0123abcd0123abcd0123abcd0123abcd0123abcd0123abcd0123abcd",
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
            "Saved empty test project snapshot.".to_string(),
        );
        assert_eq!(empty_summary.persisted_mechanics_result_count, 0);
        assert_eq!(empty_summary.persisted_analysis_run_count, 0);
        assert_eq!(empty_summary.persisted_analysis_run_ref, "not_persisted");
        assert_eq!(empty_summary.persisted_model_hash_count, 0);
        assert_eq!(empty_summary.persisted_model_hash_ref, "not_persisted");

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
        let first_open =
            apply_store_migrations(&connection).expect("fresh store migrations apply");
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
            ]
        );
        assert_eq!(
            first_open.migration_status,
            "migrated_on_open_store_schema_v0_to_v7"
        );

        let second_open =
            apply_store_migrations(&connection).expect("current store reopen succeeds");
        assert_eq!(
            second_open.store_schema_version_before_open,
            STORE_SCHEMA_TARGET_VERSION
        );
        assert_eq!(second_open.store_schema_version, STORE_SCHEMA_TARGET_VERSION);
        assert!(second_open.migrations_applied_on_open.is_empty());
        assert_eq!(second_open.pending_migration_count, 0);
        assert_eq!(
            second_open.migration_status,
            "current_store_schema_v7_no_pending_migrations"
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
        assert_eq!(store_user_version(&connection).expect("user_version reads"), 0);

        let migration =
            apply_store_migrations(&connection).expect("legacy store migrations apply");
        assert_eq!(migration.store_schema_version_before_open, 0);
        assert_eq!(migration.store_schema_version, STORE_SCHEMA_TARGET_VERSION);
        assert_eq!(migration.migrations_applied_on_open.len(), 7);
        assert_eq!(
            migration.migration_status,
            "migrated_on_open_store_schema_v0_to_v7"
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
}
