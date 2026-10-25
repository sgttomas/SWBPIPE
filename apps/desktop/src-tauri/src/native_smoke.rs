//! Native smoke harness for the desktop walking skeleton (DEL-07-11 VER-003).
//!
//! Compiled only with the `native-smoke` cargo feature, which no product build
//! enables. `apps/desktop/native-smoke/run.mjs` builds the app with it and
//! launches it with these environment variables:
//!
//! - `SWBPIPE_NATIVE_SMOKE_DRIVER`: a script evaluated in the main webview once
//!   the page has loaded (`apps/desktop/native-smoke/driver.js`);
//! - `SWBPIPE_NATIVE_SMOKE_PLAN`: JSON handed to the driver;
//! - `SWBPIPE_NATIVE_SMOKE_REPORT`: where `native_smoke_finish` writes the report.
//!
//! The harness substitutes exactly two things for a person at the machine: it
//! names the file the next model-document chooser returns (`native_smoke_choose`;
//! the Open Model Document…, Save Model Document…, Save Handoff… and Save Report
//! Package… choosers) instead of showing the chooser, and it delivers native menu commands through the same
//! dispatch a menu click uses (`native_smoke_menu`). Everything else, from the
//! open checks to solve, persistence and export, runs on the product's code.

use serde_json::Value;
use std::{collections::VecDeque, fs, path::PathBuf, sync::Mutex};
use tauri::{webview::PageLoadEvent, AppHandle, Builder, Manager, Runtime};

#[derive(Default)]
pub struct SmokeState {
    chosen: Mutex<VecDeque<PathBuf>>,
    driver_started: Mutex<bool>,
}

/// The path the next chooser (model document open or save, handoff or report package) returns, if the driver named one.
pub fn take_chosen_path<R: Runtime>(app: &AppHandle<R>) -> Option<PathBuf> {
    app.try_state::<SmokeState>()?.chosen.lock().ok()?.pop_front()
}

#[tauri::command]
pub fn native_smoke_choose(app: AppHandle, path: String) -> Result<(), String> {
    app.state::<SmokeState>()
        .chosen
        .lock()
        .map_err(|_| "native smoke state poisoned".to_string())?
        .push_back(PathBuf::from(path));
    Ok(())
}

/// Drops every chooser path not yet taken, so a save refused before its chooser
/// cannot answer a later one.
#[tauri::command]
pub fn native_smoke_clear_choices(app: AppHandle) -> Result<(), String> {
    app.state::<SmokeState>()
        .chosen
        .lock()
        .map_err(|_| "native smoke state poisoned".to_string())?
        .clear();
    Ok(())
}

#[tauri::command]
pub fn native_smoke_menu(app: AppHandle, command_id: String) {
    crate::dispatch_native_menu_command(&app, &command_id);
}

#[tauri::command]
pub fn native_smoke_plan() -> Result<Value, String> {
    let text = std::env::var("SWBPIPE_NATIVE_SMOKE_PLAN").map_err(|_| "SWBPIPE_NATIVE_SMOKE_PLAN unset".to_string())?;
    serde_json::from_str(&text).map_err(|error| error.to_string())
}

/// Another writer saves this stored project, as a second window or process
/// would: the same model and review context, without the run. It goes through
/// the product's own persistence functions, so the stored revision (and its
/// envelope hash) moves exactly as a real save would move it.
#[tauri::command]
pub fn native_smoke_external_save(app: AppHandle, project_id: String) -> Result<(), String> {
    let path = crate::app_store_path(&app)?;
    let (mut connection, _) = crate::open_project_store(&path)?;
    let record = crate::load_project(&connection, Some(&project_id))?
        .ok_or_else(|| format!("{project_id} is not stored"))?;
    let none = Value::Null;
    let envelope_hash = crate::computed_project_envelope_hash(
        &project_id, &record.model, &record.editor_intents, &record.proposal,
        &record.selected_review_target, &none, &none, &record.model_hash,
    );
    crate::upsert_project(
        &mut connection, &project_id, &record.project_name, &record.model, &record.editor_intents,
        &record.proposal, &record.selected_review_target, &none, &none, &record.model_hash,
        &envelope_hash,
    )
}

#[tauri::command]
pub fn native_smoke_log(line: String) {
    eprintln!("[native-smoke] {line}");
}

#[tauri::command]
pub fn native_smoke_finish(app: AppHandle, report: Value) -> Result<(), String> {
    let path = std::env::var("SWBPIPE_NATIVE_SMOKE_REPORT").map_err(|_| "SWBPIPE_NATIVE_SMOKE_REPORT unset".to_string())?;
    let text = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())?;
    fs::write(&path, text).map_err(|error| format!("{path}: {error}"))?;
    app.exit(0);
    Ok(())
}

/// Manage the smoke state and start the driver after the first completed page load.
pub fn install<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    builder.manage(SmokeState::default()).on_page_load(|webview, payload| {
        if payload.event() != PageLoadEvent::Finished {
            return;
        }
        let state = webview.app_handle().state::<SmokeState>();
        let Ok(mut started) = state.driver_started.lock() else { return };
        if *started {
            return;
        }
        let Ok(driver) = std::env::var("SWBPIPE_NATIVE_SMOKE_DRIVER") else {
            eprintln!("[native-smoke] SWBPIPE_NATIVE_SMOKE_DRIVER unset; no driver started");
            return;
        };
        match fs::read_to_string(&driver) {
            Ok(script) => {
                *started = true;
                if let Err(error) = webview.eval(&script) {
                    eprintln!("[native-smoke] driver eval failed: {error}");
                }
            }
            Err(error) => eprintln!("[native-smoke] {driver}: {error}"),
        }
    })
}

/// Runs the product's handoff verifier (`verify_handoff_package`, DEL-15-01
/// W-6b) on a saved package, so the smoke checks what the save wrote with the
/// verifier the design names.
#[tauri::command]
pub fn native_smoke_verify_handoff_package(path: String) -> Result<crate::handoff_package_publish::HandoffVerification, String> {
    crate::handoff_package_publish::verify_handoff_package(std::path::Path::new(&path))
}

/// Re-reads a saved report package with the product's reader
/// (`read_report_package_container`, DEL-08-01 W-18) and reports what the smoke
/// checks: the file's SHA-256, the manifest's member inventory beside each
/// member's recomputed SHA-256 and length, and the result member's text.
#[tauri::command]
pub fn native_smoke_read_report_package(path: String) -> Result<Value, String> {
    use sha2::{Digest, Sha256};
    let bytes = fs::read(&path).map_err(|error| format!("{path}: {error}"))?;
    let read = open_pipe_stress_report_package::read_report_package_container(&bytes).map_err(|refusal| refusal.to_string())?;
    let hex = |data: &[u8]| format!("{:x}", Sha256::digest(data));
    let members: Vec<Value> = read
        .members
        .iter()
        .map(|member| {
            serde_json::json!({
                "role": member.role,
                "file_name": member.file_name,
                "byte_length": member.bytes.len(),
                "sha256_hex": hex(&member.bytes),
            })
        })
        .collect();
    let result_member = read
        .members
        .iter()
        .find(|member| member.role == open_pipe_stress_report_package::ROLE_RESULT_EXPORT)
        .map(|member| String::from_utf8_lossy(&member.bytes).into_owned());
    Ok(serde_json::json!({
        "file_sha256_hex": hex(&bytes),
        "file_byte_length": bytes.len(),
        "schema_version": read.schema_version,
        "export_profile_id": read.export_profile_id,
        "export_blocked": read.export_blocked,
        "package_identity_sha256_hex": read.package_identity_sha256_hex,
        "inventory": read.manifest["members"],
        "members": members,
        "result_member_text": result_member,
    }))
}
