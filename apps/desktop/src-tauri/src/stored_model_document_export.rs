//! Recovery export of a stored model document (W-17, DEL-02-02 Design
//! `project-unit-selection.md` §3).
//!
//! A stored project or saved state whose document the desktop refuses at open
//! (`MODEL-FILE-SHAPE` at `project.units`) is written to a file the user
//! chooses, exactly as the store holds it, so the user can add the map and open
//! the file with Open Model Document…. Nothing is adopted into a session and
//! nothing is computed from the document for one.
//!
//! - The bytes written are the stored text itself: a project's
//!   `local_projects.model_json` (the compact JSON the store serialized when it
//!   saved the project, not necessarily RFC 8785 canonical) or a saved state's
//!   `model_states.model_canonical` (its RFC 8785 text, whose SHA-256 is the
//!   state's model hash). The read is a plain `SELECT`; the store is not
//!   written.
//! - The open checks (version, document validator, desktop shape) are not
//!   applied: the document is refused at open for one of them, and the export
//!   exists to recover it.
//! - The save path's privacy controls still apply: the explicit local-private
//!   intent (`MODEL-DOC-SAVE-INTENT-REQUIRED` without it) and the native
//!   disposition screening of the exact document written
//!   (`model_document_save::screen_model_document_dispositions`: a protected,
//!   quarantined or rejected disposition refuses `PROTECTED_CONTENT_BLOCKED`).
//! - The chooser and the atomic `.json` writer are the Save Model Document
//!   ones (`lib.rs` `choose_model_document_save_path`,
//!   `model_document_save::write_admitted_model_document`).

use crate::export_disposition_screen::diagnostic;
use crate::model_document_save::{
    screen_model_document_dispositions, suggested_model_document_file_name, AdmittedModelDocument, LibraryLookup,
    ModelDocumentSaveDiagnostic,
};
use rusqlite::{params, Connection, OptionalExtension};
use serde_json::Value;

pub const INTENT_REQUIRED: &str = "MODEL-DOC-SAVE-INTENT-REQUIRED";
pub const NOT_FOUND: &str = "MODEL-DOC-EXPORT-NOT-FOUND";
pub const STORE_UNAVAILABLE: &str = "MODEL-DOC-EXPORT-STORE-UNAVAILABLE";
pub const NOT_JSON: &str = "MODEL-DOC-EXPORT-NOT-JSON";

/// The stored text of a project's document (`state_id` absent) or of one of
/// its saved states, exactly as stored; `None` when there is no such row.
pub fn read_stored_model_text(
    connection: &Connection,
    project_id: &str,
    state_id: Option<&str>,
) -> Result<Option<String>, String> {
    match state_id {
        None => connection.query_row(
            "SELECT model_json FROM local_projects WHERE project_id = ?1",
            params![project_id],
            |row| row.get::<_, String>(0),
        ),
        Some(state_id) => connection.query_row(
            "SELECT model_canonical FROM model_states WHERE project_id = ?1 AND state_id = ?2",
            params![project_id, state_id],
            |row| row.get::<_, String>(0),
        ),
    }
    .optional()
    .map_err(|error| error.to_string())
}

/// What is exported, for the refusals.
pub fn stored_document_name(project_id: &str, state_id: Option<&str>) -> String {
    match state_id {
        None => format!("project `{project_id}`"),
        Some(state_id) => format!("saved state `{state_id}` of project `{project_id}`"),
    }
}

/// Admits the stored text for writing, unchanged: it must be JSON (for the
/// screening and the suggested file name), and its dispositions are screened.
pub fn admit_stored_model_text(
    text: String,
    lookup: &mut LibraryLookup<'_>,
) -> Result<AdmittedModelDocument, Vec<ModelDocumentSaveDiagnostic>> {
    let document: Value = serde_json::from_str(&text).map_err(|error| {
        vec![diagnostic(NOT_JSON, "", format!("The stored document is not JSON ({error}); nothing was written."))]
    })?;
    let warnings = screen_model_document_dispositions(&document, lookup)?;
    let suggested_file_name = suggested_model_document_file_name(&document);
    Ok(AdmittedModelDocument { text, suggested_file_name, warnings })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model_document_save::{write_admitted_model_document, ModelDocumentSaveOutcome, PROTECTED_CONTENT_BLOCKED};
    use open_pipe_stress_operation_applier::sha256_hex;
    use serde_json::json;
    use std::fs;

    /// A document without `project.units`, stored as the store serializes it.
    fn unitless_text() -> String {
        let document = json!({
            "schema_version": "0.7.0",
            "document_kind": "openpipestress.model",
            "project": { "id": "project:invented-no-units", "name": "Invented", "description": "" },
            "materials": [{ "id": "material:invented", "provenance_record": { "review_status": "accepted" } }]
        });
        serde_json::to_string(&document).unwrap()
    }

    fn store(model_json: &str, state_text: &str) -> Connection {
        let connection = Connection::open_in_memory().unwrap();
        connection
            .execute_batch(
                "CREATE TABLE local_projects(project_id TEXT PRIMARY KEY, model_json TEXT NOT NULL);
                 CREATE TABLE model_states(project_id TEXT NOT NULL, state_id TEXT NOT NULL, model_canonical TEXT NOT NULL);",
            )
            .unwrap();
        connection
            .execute("INSERT INTO local_projects VALUES ('project:invented-no-units', ?1)", params![model_json])
            .unwrap();
        connection
            .execute("INSERT INTO model_states VALUES ('project:invented-no-units', 'state:invented-1', ?1)", params![state_text])
            .unwrap();
        connection
    }

    fn no_store(_: &str, _: &str, _: &str) -> Result<Option<Value>, String> {
        Ok(None)
    }

    #[test]
    fn reads_the_stored_text_exactly_and_writes_nothing_to_the_store() {
        // Key order and spacing that a re-serialization would change.
        let state_text = "{\"schema_version\":\"0.7.0\",\"project\":{\"name\":\"Invented\",\"id\":\"project:invented-no-units\"}, \"x\":1.50}";
        let connection = store(&unitless_text(), state_text);
        let changes = connection.total_changes();
        let project = read_stored_model_text(&connection, "project:invented-no-units", None).unwrap();
        let state = read_stored_model_text(&connection, "project:invented-no-units", Some("state:invented-1")).unwrap();
        assert_eq!(project.as_deref(), Some(unitless_text().as_str()));
        assert_eq!(state.as_deref(), Some(state_text));
        assert_eq!(read_stored_model_text(&connection, "project:other", None).unwrap(), None);
        assert_eq!(read_stored_model_text(&connection, "project:invented-no-units", Some("state:other")).unwrap(), None);
        assert_eq!(connection.total_changes(), changes);
    }

    #[test]
    fn writes_the_stored_bytes_unchanged() {
        let state_text = "{\"schema_version\":\"0.7.0\",\"project\":{\"name\":\"Invented\",\"id\":\"project:invented-no-units\"}, \"x\":1.50}";
        let admitted = admit_stored_model_text(state_text.to_string(), &mut no_store).unwrap();
        assert_eq!(admitted.suggested_file_name, "invented-no-units.json");
        let root = std::env::temp_dir().join(format!(
            "swbpipe-stored-export-{}-{}",
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let destination = root.join("recovered.json");
        match write_admitted_model_document(&destination, admitted) {
            ModelDocumentSaveOutcome::Saved { byte_count, model_hash, .. } => {
                assert_eq!(byte_count, state_text.len() as u64);
                assert_eq!(model_hash, format!("sha256:{}", sha256_hex(state_text)));
            }
            other => panic!("not saved: {other:?}"),
        }
        assert_eq!(fs::read(&destination).unwrap(), state_text.as_bytes());
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn screens_dispositions_and_refuses_text_that_is_not_json() {
        let blocked = unitless_text().replace("\"accepted\"", "\"quarantined\"");
        let refused = admit_stored_model_text(blocked, &mut no_store).unwrap_err();
        assert_eq!(
            refused.iter().map(|item| (item.code.as_str(), item.path.as_str())).collect::<Vec<_>>(),
            vec![(PROTECTED_CONTENT_BLOCKED, "materials[0].provenance_record")]
        );
        let refused = admit_stored_model_text("{not json".to_string(), &mut no_store).unwrap_err();
        assert_eq!(refused[0].code, NOT_JSON);
    }
}
