//! Native smoke harness for the desktop walking skeleton (DEL-07-11 VER-003).
//!
//! Compiled only with the `native-smoke` cargo feature, which no product build
//! enables. `apps/desktop/native-smoke/run.mjs` builds the app with it and
//! launches it with these environment variables:
//!
//! - `SWBPIPE_NATIVE_SMOKE_DRIVER`: a script evaluated in the main webview once
//!   the page has loaded (`apps/desktop/native-smoke/driver.js`); `-` reads it
//!   from bounded anonymous stdin, so fixture credentials need no script file;
//! - `SWBPIPE_NATIVE_SMOKE_PLAN`: JSON handed to the driver;
//! - `SWBPIPE_NATIVE_SMOKE_REPORT`: where `native_smoke_finish` writes the report;
//! - `SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY=stdin` (macOS, with the driver on stdin):
//!   stdin's first line is an agent provider key and the rest is the driver.
//!   The key is held only in this process's memory until the embedded agent's
//!   next Start with a provider saved with no key takes it, once
//!   ([`take_provider_secret`]); it is never written, logged, given to the
//!   webview or read from or saved to the Keychain (RT-E1 D4, DEL-16-06
//!   `Work/agent-journey-scenario.md`).
//!
//! The harness substitutes exactly two things for a person at the machine: it
//! names the file the next model-document chooser returns (`native_smoke_choose`;
//! the Open Model Document…, Save Model Document…, Save Handoff… and Save Report
//! Package… choosers) instead of showing the chooser, and it delivers native menu commands through the same
//! dispatch a menu click uses (`native_smoke_menu`). Everything else, from the
//! open checks to solve, persistence and export, runs on the product's code.

use serde_json::Value;
use std::{collections::VecDeque, fs, io::{self, Read}, path::PathBuf, sync::Mutex};
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

#[cfg(target_os = "macos")]
static PROVIDER_SECRET: Mutex<Option<crate::embedded_agent::relay_policy::Secret>> = Mutex::new(None);

/// The provider key the controller passed on stdin, taken at most once.
#[cfg(target_os = "macos")]
pub fn take_provider_secret() -> Option<crate::embedded_agent::relay_policy::Secret> {
    PROVIDER_SECRET.lock().ok()?.take()
}

/// Splits `key\n` from the driver that follows it. The key line is bounded by
/// the relay's secret limit and must satisfy `Secret::new`; nothing about a
/// refused key is echoed.
#[cfg(target_os = "macos")]
fn read_stdin_key_and_driver(mut reader: impl io::BufRead) -> io::Result<(crate::embedded_agent::relay_policy::Secret, String)> {
    use crate::embedded_agent::relay_policy::{Secret, SECRET_BYTES};
    use std::io::BufRead;
    let mut line = Vec::new();
    (&mut reader).take(SECRET_BYTES as u64 + 2).read_until(b'\n', &mut line)?;
    let refused = || io::Error::new(io::ErrorKind::InvalidData, "native smoke provider key refused");
    if line.pop() != Some(b'\n') {
        return Err(refused());
    }
    let secret = String::from_utf8(line).ok().and_then(|text| Secret::new(text).ok()).ok_or_else(refused)?;
    Ok((secret, read_stdin_driver(reader)?))
}

fn read_stdin_driver(reader: impl Read) -> io::Result<String> {
    let mut script = String::new();
    reader.take(256 * 1024 + 1).read_to_string(&mut script)?;
    if script.len() > 256 * 1024 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "native smoke driver exceeds its input limit"));
    }
    Ok(script)
}
#[cfg(test)]
mod input_tests {
    #[test]
    fn anonymous_driver_input_is_bounded_without_a_file_or_environment_payload() {
        assert_eq!(super::read_stdin_driver(&b"void 0;"[..]).unwrap(), "void 0;");
        assert!(super::read_stdin_driver(&vec![b'x'; 256 * 1024 + 1][..]).is_err());
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn the_provider_key_line_precedes_the_driver_and_is_checked() {
        use crate::embedded_agent::relay_policy::SECRET_BYTES;
        let (secret, driver) = super::read_stdin_key_and_driver(&b"invented-fixture-key\nvoid 0;"[..]).unwrap();
        assert_eq!(format!("{secret:?}"), "Secret([REDACTED])");
        assert_eq!(driver, "void 0;");
        let long = [vec![b'k'; SECRET_BYTES + 1], b"\nvoid 0;".to_vec()].concat();
        for input in [&b"no newline"[..], b"\nvoid 0;", b"two words\nvoid 0;", b"tab\tkey\nvoid 0;", &long] {
            let error = super::read_stdin_key_and_driver(input).unwrap_err();
            assert_eq!(error.to_string(), "native smoke provider key refused");
        }
        let oversized = [b"k\n".to_vec(), vec![b'x'; 256 * 1024 + 1]].concat();
        assert!(super::read_stdin_key_and_driver(&oversized[..]).is_err());
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn the_provider_key_is_taken_once() {
        use crate::embedded_agent::relay_policy::Secret;
        *super::PROVIDER_SECRET.lock().unwrap() = Some(Secret::new("invented-fixture-key".into()).unwrap());
        assert!(super::take_provider_secret().is_some());
        assert!(super::take_provider_secret().is_none());
    }
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
        #[cfg(target_os = "macos")]
        let with_key = std::env::var("SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY").ok();
        #[cfg(target_os = "macos")]
        if with_key.is_some() {
            if with_key.as_deref() != Some("stdin") || driver != "-" {
                eprintln!("[native-smoke] SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY needs the value stdin and the driver on stdin; no driver started");
                return;
            }
            match read_stdin_key_and_driver(io::stdin().lock()) {
                Ok((secret, script)) => {
                    let Ok(mut held) = PROVIDER_SECRET.lock() else { return };
                    *held = Some(secret);
                    drop(held);
                    *started = true;
                    if let Err(error) = webview.eval(&script) {
                        eprintln!("[native-smoke] driver eval failed: {error}");
                    }
                }
                Err(error) => eprintln!("[native-smoke] stdin: {error}; no driver started"),
            }
            return;
        }
        match if driver == "-" { read_stdin_driver(io::stdin().lock()) } else { fs::read_to_string(&driver) } {
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
