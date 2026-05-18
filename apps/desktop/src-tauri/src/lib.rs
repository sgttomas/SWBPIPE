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
}

#[derive(Debug, Serialize)]
struct LocalProjectSummary {
    project_id: String,
    project_name: String,
    database_path: String,
    storage_mode: &'static str,
    migration_status: &'static str,
    fts_indexed: bool,
    copied_external_files: bool,
    message: String,
}

#[derive(Debug, Serialize)]
struct LocalProjectEnvelope {
    summary: LocalProjectSummary,
    model: Value,
}

#[derive(Debug, Deserialize)]
struct SaveLocalProjectRequest {
    project_id: String,
    project_name: String,
    model: Value,
}

fn app_store_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_local_data_dir()
        .map_err(|error| error.to_string())?;
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    Ok(dir.join(PROJECT_STORE_FILE))
}

fn open_project_store(path: &Path) -> Result<Connection, String> {
    let connection = Connection::open(path).map_err(|error| error.to_string())?;
    initialize_project_store(&connection)?;
    Ok(connection)
}

fn initialize_project_store(connection: &Connection) -> Result<(), String> {
    connection
        .execute_batch(
            "
            PRAGMA foreign_keys = ON;
            PRAGMA journal_mode = WAL;
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
) -> Result<(), String> {
    let model_json = serde_json::to_string(model).map_err(|error| error.to_string())?;
    let now = now_unix_seconds()?;
    let transaction = connection
        .transaction()
        .map_err(|error| error.to_string())?;
    transaction
        .execute(
            "
            INSERT INTO local_projects (project_id, project_name, model_json, created_at_unix, updated_at_unix)
            VALUES (?1, ?2, ?3, ?4, ?4)
            ON CONFLICT(project_id) DO UPDATE SET
                project_name = excluded.project_name,
                model_json = excluded.model_json,
                updated_at_unix = excluded.updated_at_unix
            ",
            params![project_id, project_name, model_json, now],
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
            params![project_id, project_name, model_json],
        )
        .map_err(|error| error.to_string())?;
    transaction.commit().map_err(|error| error.to_string())
}

fn load_project(
    connection: &Connection,
    project_id: Option<&str>,
) -> Result<Option<(String, String, Value)>, String> {
    let row = match project_id {
        Some(id) => connection
            .query_row(
                "
                SELECT project_id, project_name, model_json
                FROM local_projects
                WHERE project_id = ?1
                ",
                params![id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                    ))
                },
            )
            .optional()
            .map_err(|error| error.to_string())?,
        None => connection
            .query_row(
                "
                SELECT project_id, project_name, model_json
                FROM local_projects
                ORDER BY updated_at_unix DESC, project_id ASC
                LIMIT 1
                ",
                [],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                    ))
                },
            )
            .optional()
            .map_err(|error| error.to_string())?,
    };
    row.map(|(id, name, model_json)| {
        serde_json::from_str::<Value>(&model_json)
            .map(|model| (id, name, model))
            .map_err(|error| error.to_string())
    })
    .transpose()
}

fn project_summary(
    project_id: String,
    project_name: String,
    database_path: PathBuf,
    message: String,
) -> LocalProjectSummary {
    LocalProjectSummary {
        project_id,
        project_name,
        database_path: database_path.display().to_string(),
        storage_mode: "local_sqlite",
        migration_status: "current",
        fts_indexed: true,
        copied_external_files: false,
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

#[tauri::command]
fn sample_agent_proposal() -> Result<Value, String> {
    let result = run_preview_mechanics(None)?;
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
    let diagnostic_ref = diagnostics
        .iter()
        .find(|item| item.get("severity").and_then(Value::as_str) == Some("warning"))
        .and_then(|item| item.get("id").and_then(Value::as_str))
        .unwrap_or("diagnostic:physics:rule-inputs-missing");
    let rationale_ref = if primary_ref.starts_with("result:force:") {
        primary_ref.as_str()
    } else {
        diagnostic_ref
    }
    .to_string();
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
            "affected_entity_ids": [primary_ref],
            "changes": [
                {
                    "change_id": "change:add-review-note",
                    "change_kind": "attach_design_knowledge",
                    "target_ref": primary_ref,
                    "before": "No computed mechanics force review note attached.",
                    "after": "Attach review note referencing the current computed preview force, stress, and diagnostic context."
                }
            ]
        },
        "rationale": format!("Generated from current preview mechanics context; primary reference is {rationale_ref}."),
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
    Ok(json!({
        "proposal": proposal,
        "application_status": "not_applied",
        "accepted_model_mutated": false
    }))
}

#[tauri::command]
fn get_local_storage_capability(app: AppHandle) -> Result<StorageCapability, String> {
    let path = app_store_path(&app)?;
    let connection = open_project_store(&path)?;
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
    })
}

#[tauri::command]
fn create_local_project(app: AppHandle, model: Value) -> Result<LocalProjectEnvelope, String> {
    let path = app_store_path(&app)?;
    let mut connection = open_project_store(&path)?;
    let project_id = project_id_from_model(&model);
    let project_name = project_name_from_model(&model);
    upsert_project(&mut connection, &project_id, &project_name, &model)?;
    Ok(LocalProjectEnvelope {
        summary: project_summary(
            project_id,
            project_name,
            path,
            "Created local SQLite project snapshot without external file copies.".to_string(),
        ),
        model,
    })
}

#[tauri::command]
fn open_local_project(
    app: AppHandle,
    project_id: Option<String>,
) -> Result<Option<LocalProjectEnvelope>, String> {
    let path = app_store_path(&app)?;
    let connection = open_project_store(&path)?;
    load_project(&connection, project_id.as_deref())?
        .map(|(id, name, model)| {
            Ok(LocalProjectEnvelope {
                summary: project_summary(
                    id,
                    name,
                    path.clone(),
                    "Opened local SQLite project snapshot.".to_string(),
                ),
                model,
            })
        })
        .transpose()
}

#[tauri::command]
fn save_local_project(
    app: AppHandle,
    request: SaveLocalProjectRequest,
) -> Result<LocalProjectEnvelope, String> {
    let path = app_store_path(&app)?;
    let mut connection = open_project_store(&path)?;
    upsert_project(
        &mut connection,
        &request.project_id,
        &request.project_name,
        &request.model,
    )?;
    Ok(LocalProjectEnvelope {
        summary: project_summary(
            request.project_id,
            request.project_name,
            path,
            "Saved current project model snapshot without external file copies.".to_string(),
        ),
        model: request.model,
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
        initialize_project_store(&connection).expect("project store schema initializes");
        assert!(fts5_available(&connection));

        let model = json!({
            "schema_version": "0.1.0",
            "document_kind": "openpipestress.product_preview.model",
            "project": {
                "id": "project:test-local",
                "name": "Test Local Project"
            }
        });
        upsert_project(
            &mut connection,
            "project:test-local",
            "Test Local Project",
            &model,
        )
        .expect("project snapshot saves");

        let loaded = load_project(&connection, Some("project:test-local"))
            .expect("project snapshot loads")
            .expect("saved project exists");
        assert_eq!(loaded.0, "project:test-local");
        assert_eq!(loaded.1, "Test Local Project");
        assert_eq!(loaded.2, model);

        let indexed_count: i64 = connection
            .query_row(
                "SELECT count(*) FROM local_project_fts WHERE local_project_fts MATCH ?1",
                params!["Test"],
                |row| row.get(0),
            )
            .expect("fts5 query succeeds");
        assert_eq!(indexed_count, 1);
    }
}
