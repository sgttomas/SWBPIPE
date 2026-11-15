//! The rendered report's native routes in the desktop (DEL-08-01 REQ-022,
//! AC-011; DEL-12-02 AC-007): Save canonical HTML… through the system save
//! chooser and the atomic writer, and Print… of the derived print view in a
//! webview of its own (`rendered_report_print`). Both leave by route
//! `DREP-HTML-SAVE-005`, the catalogue's rendered-document route, under the
//! context the render was made in, `public_report` or `local_private`.
//!
//! Binding. The render command records, for the one current render, a
//! generation, the SHA-256 the desktop itself computes of the canonical HTML,
//! the render's export context and whether its gates blocked it
//! ([`RenderCustody`]); it keeps no copy of the document. A route receives the
//! canonical HTML in its raw body (priced by the ingress lease), computes its
//! SHA-256 and admits it only when that is the current render's. The context
//! and the block are read from the record, never from the request, whose
//! context, intent and evidence must agree with it. The saved file is exactly
//! the admitted bytes; the printed document is derived from them here
//! (`derived_print_view`), so HTML that the render did not produce is refused
//! and neither route sends anything the webview composed.
//!
//! Currency (DEL-08-01 REQ-022). Each render request retires the record and a
//! completed render replaces it, whatever its outcome; the report panel
//! retires it whenever it discards the render it shows
//! ([`retire_rendered_report`]). The save re-checks the record after its
//! chooser and writes nothing if it changed; the print serves and prints its
//! document only while the record is current and closes its window when it is
//! not.
//!
//! A user-released report has no route here: the render command refuses its
//! input (`refuse_user_release_render`), and a user release lets out only its
//! package (DEL-08-01 REQ-019, "Admission"); a record under any context but
//! the two above is refused by name.
use crate::atomic_report_package_save::{self as atomic, AtomicSaveError, AtomicSaveFormat, AtomicSaveOutcome};
use crate::command_ingress::{self, respond, ArgsDecoding, CommandError};
use crate::solve_jobs::SolveJobRegistry;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, PoisonError};
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

/// The catalogue's rendered-document route (`redactionExportControls.ts`,
/// `route_control.py`): the canonical HTML save and the print view.
pub(crate) const ROUTE_ID: &str = "DREP-HTML-SAVE-005";
pub(crate) const PUBLIC_REPORT: &str = "public_report";
pub(crate) const LOCAL_PRIVATE: &str = "local_private";

/// No render is current: none was made, or it was retired or replaced.
pub(crate) const NOT_CURRENT: &str = "REPORT-ROUTE-NOT-CURRENT";
/// The current render is blocked by its gates (DEL-08-01 REQ-014).
pub(crate) const BLOCKED: &str = "REPORT-ROUTE-BLOCKED";
/// The submitted HTML's SHA-256 is not the current render's.
pub(crate) const UNBOUND: &str = "REPORT-ROUTE-HTML-UNBOUND";
/// The current render's context is not one this route admits.
pub(crate) const CONTEXT_NOT_ADMITTED: &str = "REPORT-ROUTE-CONTEXT-NOT-ADMITTED";
/// The request names another context than the current render's.
pub(crate) const CONTEXT_MISMATCH: &str = "REPORT-ROUTE-CONTEXT-MISMATCH";
/// A local-private render leaves only with the panel's explicit intent.
pub(crate) const INTENT_REQUIRED: &str = "REPORT-ROUTE-LOCAL-PRIVATE-INTENT-REQUIRED";
/// The controlled-export evidence does not admit this route.
pub(crate) const EVIDENCE_REFUSED: &str = "REPORT-ROUTE-EVIDENCE-REFUSED";
/// The render was retired or replaced while the chooser was open.
pub(crate) const RETIRED_DURING_SAVE: &str = "REPORT-HTML-SAVE-RETIRED";
/// The chooser returned a destination that is not a path.
pub(crate) const CHOOSER_FAILED: &str = "REPORT-HTML-SAVE-CHOOSER-FAILED";
/// The route's worker did not complete.
pub(crate) const WORKER_FAILED: &str = "REPORT-ROUTE-WORKER-FAILED";

/// The canonical HTML file (`.html`) under the desktop's atomic writer.
pub(crate) const REPORT_HTML_FORMAT: AtomicSaveFormat = AtomicSaveFormat {
    extension: "html",
    code_prefix: "REPORT-HTML-SAVE",
    temp_prefix: ".openpipestress-report-html-",
    log_label: "report-html",
};

/// The current render as the desktop recorded it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RenderRecord {
    pub(crate) generation: u64,
    pub(crate) sha256_hex: String,
    pub(crate) export_context: Option<String>,
    pub(crate) blocked: bool,
}

struct Slot {
    generation: u64,
    current: Option<RenderRecord>,
}

/// The one current render's record (see the module docs).
pub(crate) struct RenderCustody {
    slot: Mutex<Slot>,
}

/// The desktop's custody of its current render.
pub(crate) static CUSTODY: RenderCustody = RenderCustody::new();

impl RenderCustody {
    pub(crate) const fn new() -> Self {
        Self { slot: Mutex::new(Slot { generation: 0, current: None }) }
    }

    fn lock(&self) -> MutexGuard<'_, Slot> {
        self.slot.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Records a completed render as the current one and returns its
    /// generation. The SHA-256 is computed here; a render whose canonical
    /// HTML does not hash to the renderer's named hash, a blocked render and
    /// a metadata-only refusal are recorded as blocked.
    pub(crate) fn record(&self, export_context: Option<&str>, html: &str, renderer_sha256_hex: &str, export_blocked: bool, content_withheld: bool) -> u64 {
        let sha256_hex = sha256_hex(html.as_bytes());
        let blocked = export_blocked || content_withheld || sha256_hex != renderer_sha256_hex;
        let mut slot = self.lock();
        slot.generation += 1;
        let generation = slot.generation;
        slot.current = Some(RenderRecord { generation, sha256_hex, export_context: export_context.map(str::to_string), blocked });
        generation
    }

    /// Retires the current render, if any. Returns whether one was current.
    pub(crate) fn retire(&self) -> bool {
        let mut slot = self.lock();
        slot.generation += 1;
        slot.current.take().is_some()
    }

    /// Whether the render of `generation` is still the current one.
    pub(crate) fn is_current(&self, generation: u64) -> bool {
        self.lock().current.as_ref().is_some_and(|record| record.generation == generation)
    }

    /// Admits a route request against the current record (module docs):
    /// refused by name when no render is current, when it is blocked, when
    /// the HTML is not its canonical HTML, when its context is not admitted
    /// or differs from the request's, when a local-private render lacks the
    /// explicit intent, and when the evidence does not admit the route.
    pub(crate) fn admit(&self, request: &RouteRequest) -> Result<Admitted, RouteError> {
        let record = self.lock().current.clone().ok_or_else(|| {
            RouteError::refused(NOT_CURRENT, "No rendered report is current: it was retired or replaced. Render the report again.")
        })?;
        if record.blocked {
            return Err(RouteError::refused(BLOCKED, "The current render is blocked by its export gates: it is neither saved nor printed."));
        }
        let sha256_hex = sha256_hex(request.html.as_bytes());
        if sha256_hex != record.sha256_hex {
            return Err(RouteError::refused(
                UNBOUND,
                "The submitted HTML is not the current render's canonical HTML: its SHA-256 differs from the one the desktop recorded for that render.",
            ));
        }
        let context = match record.export_context.as_deref() {
            Some(context) if context == PUBLIC_REPORT || context == LOCAL_PRIVATE => context,
            found => {
                return Err(RouteError::refused(
                    CONTEXT_NOT_ADMITTED,
                    format!(
                        "Route {ROUTE_ID} admits only the public_report and local_private contexts; the current render carries {}.",
                        if found.is_none() { "no export context" } else { "another export context" }
                    ),
                ))
            }
        };
        if request.export_context != context {
            return Err(RouteError::refused(CONTEXT_MISMATCH, format!("The request names another export context than the current render's ({context}).")));
        }
        if context == LOCAL_PRIVATE && !request.explicit_local_private_intent {
            return Err(RouteError::refused(INTENT_REQUIRED, "A local-private report leaves only with the explicit local-private selection."));
        }
        let evidence = &request.evidence;
        if evidence.route_id != ROUTE_ID
            || evidence.export_context != context
            || evidence.blocked
            || evidence.blocking_count != 0
            || evidence.explicit_local_private_intent != request.explicit_local_private_intent
        {
            return Err(RouteError::refused(
                EVIDENCE_REFUSED,
                format!("The controlled export's evidence does not admit route {ROUTE_ID} for this render (route, context, block or intent differ)."),
            ));
        }
        Ok(Admitted { generation: record.generation, sha256_hex, export_context: context.to_string() })
    }
}

/// Records a completed render in the desktop's custody and closes a print
/// window whose render it replaces (the render command's one call).
pub(crate) fn record_render(export_context: Option<&str>, html: &str, renderer_sha256_hex: &str, export_blocked: bool, content_withheld: bool) {
    let generation = CUSTODY.record(export_context, html, renderer_sha256_hex, export_blocked, content_withheld);
    crate::rendered_report_print::close_unless_current(Some(generation));
}

/// Retires the desktop's current render and closes its print window.
pub(crate) fn retire_render() {
    CUSTODY.retire();
    crate::rendered_report_print::close_unless_current(None);
}

/// The report panel discards the render it shows (its source, selection or
/// composition changed, it rendered again or it unmounted).
#[tauri::command]
pub(crate) fn retire_rendered_report() {
    retire_render();
}

/// The controlled export's evidence for the route, as the panel computed it
/// (`renderedReportRoutes.ts`). It admits nothing alone (DEL-12-02 REQ-018).
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct RouteEvidence {
    pub(crate) route_id: String,
    pub(crate) export_context: String,
    pub(crate) explicit_local_private_intent: bool,
    pub(crate) blocked: bool,
    pub(crate) blocking_count: u64,
}

/// A route request: the canonical HTML, the context and intent the panel
/// names, and the controlled export's evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RouteRequest {
    pub(crate) html: String,
    pub(crate) export_context: String,
    pub(crate) explicit_local_private_intent: bool,
    pub(crate) evidence: RouteEvidence,
}

/// An admitted request's record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Admitted {
    pub(crate) generation: u64,
    pub(crate) sha256_hex: String,
    pub(crate) export_context: String,
}

/// A route's refusal or failure. `destination` says what became of the
/// destination: `untouched` (nothing was chosen or written) or `as_it_was`
/// (the write failed before the new file was published).
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub(crate) struct RouteError {
    pub(crate) code: String,
    pub(crate) stage: String,
    pub(crate) message: String,
    pub(crate) destination: &'static str,
    pub(crate) cleanup_code: Option<String>,
    pub(crate) selected_basename: Option<String>,
}

impl RouteError {
    pub(crate) fn refused(code: &str, message: impl Into<String>) -> Self {
        Self::at(code, "admission", message)
    }

    pub(crate) fn at(code: &str, stage: &str, message: impl Into<String>) -> Self {
        Self { code: code.to_string(), stage: stage.to_string(), message: message.into(), destination: "untouched", cleanup_code: None, selected_basename: None }
    }

    fn write(error: AtomicSaveError) -> Self {
        Self {
            code: error.code,
            stage: error.stage,
            message: error.message,
            destination: "as_it_was",
            cleanup_code: error.cleanup_code,
            selected_basename: Some(error.selected_basename),
        }
    }
}

/// The HTML save's outcome: `saved` (with `durability` `durable` or
/// `saved_durability_uncertain`, the whole file published either way) or
/// `cancelled` at the chooser.
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub(crate) struct HtmlSaveReceipt {
    pub(crate) outcome: &'static str,
    pub(crate) route_id: &'static str,
    pub(crate) export_context: String,
    pub(crate) sha256_hex: String,
    pub(crate) byte_count: usize,
    pub(crate) file_name: String,
    pub(crate) replaced_existing: bool,
    pub(crate) durability: String,
}

pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|byte| format!("{byte:02x}")).collect()
}

/// The chooser's suggested name, as the webview's download names were.
pub(crate) fn suggested_file_name(admitted: &Admitted) -> String {
    let short = &admitted.sha256_hex[..12];
    if admitted.export_context == LOCAL_PRIVATE {
        format!("openpipestress-local-private-report-{short}.html")
    } else {
        format!("openpipestress-report-{short}.html")
    }
}

/// The route's arguments, moved out of the admitted body.
pub(crate) fn route_request_args(args: &mut command_ingress::Args) -> Result<RouteRequest, crate::solve_jobs::SolveJobError> {
    Ok(RouteRequest {
        html: args.decode("html")?,
        export_context: args.decode("exportContext")?,
        explicit_local_private_intent: args.decode("explicitLocalPrivateIntent")?,
        evidence: args.decode("evidence")?,
    })
}

/// The HTML save after decoding: admission, the chooser, the currency check
/// and the write. `choose` is the save chooser and `write` the atomic
/// writer (seams in tests).
pub(crate) fn save_in(
    custody: &RenderCustody,
    request: RouteRequest,
    choose: impl FnOnce(&str) -> Option<Result<PathBuf, String>>,
    write: impl FnOnce(&Path, &[u8]) -> Result<AtomicSaveOutcome, AtomicSaveError>,
) -> Result<HtmlSaveReceipt, RouteError> {
    let admitted = custody.admit(&request)?;
    let suggested = suggested_file_name(&admitted);
    let receipt = |outcome: &'static str, file_name: String, replaced_existing: bool, durability: &str| HtmlSaveReceipt {
        outcome,
        route_id: ROUTE_ID,
        export_context: admitted.export_context.clone(),
        sha256_hex: admitted.sha256_hex.clone(),
        byte_count: request.html.len(),
        file_name,
        replaced_existing,
        durability: durability.to_string(),
    };
    let Some(chosen) = choose(&suggested) else {
        return Ok(receipt("cancelled", suggested.clone(), false, "not_applicable"));
    };
    // The chooser's own error text may name the path; it is not repeated.
    let path = chosen.map_err(|_| RouteError::at(CHOOSER_FAILED, "chooser", "The save chooser returned a destination that is not a file path; nothing was written."))?;
    if !custody.is_current(admitted.generation) {
        return Err(RouteError::at(
            RETIRED_DURING_SAVE,
            "currency",
            "The report was retired or rendered again while the chooser was open: it is no longer the report for the current result and model, and nothing was written.",
        ));
    }
    let outcome = write(&path, request.html.as_bytes()).map_err(RouteError::write)?;
    Ok(receipt("saved", outcome.selected_basename, outcome.replaced_existing, &outcome.durability))
}

fn choose_report_html_save_path(app: &AppHandle, file_name: &str) -> Option<Result<PathBuf, String>> {
    #[cfg(feature = "native-smoke")]
    if let Some(path) = crate::native_smoke::take_chosen_path(app, "choose_report_html_save_path") {
        return Some(Ok(path));
    }
    app.dialog()
        .file()
        .set_file_name(file_name)
        .add_filter("HTML document", &["html"])
        .blocking_save_file()
        .map(|selected| selected.into_path().map_err(|error| error.to_string()))
}

/// Save canonical HTML… (route `DREP-HTML-SAVE-005`). The chooser may not run
/// on Tauri's main thread, so admission, the chooser and the write run on a
/// blocking worker with the ingress lease held.
#[tauri::command]
pub(crate) async fn save_rendered_report_html(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError<RouteError>> {
    let (mut args, lease) = command_ingress::admit(&registry, &invoke, "save_rendered_report_html", ArgsDecoding::Moved)?;
    let request = route_request_args(&mut args)?;
    drop(args);
    tauri::async_runtime::spawn_blocking(move || {
        let outcome = save_in(&CUSTODY, request, |file_name| choose_report_html_save_path(&app, file_name), |path, bytes| {
            atomic::atomic_save_bytes_as(&REPORT_HTML_FORMAT, path, bytes)
        });
        let response = respond(outcome.map_err(CommandError::Command));
        drop(lease);
        response
    })
    .await
    .unwrap_or_else(|_| {
        Err(CommandError::Command(RouteError::at(WORKER_FAILED, "worker", "The save's worker did not complete; the destination may be unchanged.")))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atomic_report_package_save::{test_support::atomic_save_test_lock, AtomicSaveIo, DirectorySyncResult};
    use std::fs::{self, File};
    use std::io;
    use std::time::{SystemTime, UNIX_EPOCH};

    const HTML: &str = "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<title>Invented</title>\n</head>\n<body>\n<p>Invented report Ω</p>\n</body>\n</html>\n";

    fn evidence(context: &str, intent: bool) -> RouteEvidence {
        RouteEvidence { route_id: ROUTE_ID.to_string(), export_context: context.to_string(), explicit_local_private_intent: intent, blocked: false, blocking_count: 0 }
    }

    fn request(html: &str, context: &str, intent: bool) -> RouteRequest {
        RouteRequest { html: html.to_string(), export_context: context.to_string(), explicit_local_private_intent: intent, evidence: evidence(context, intent) }
    }

    fn rendered(context: Option<&str>) -> RenderCustody {
        let custody = RenderCustody::new();
        custody.record(context, HTML, &sha256_hex(HTML.as_bytes()), false, false);
        custody
    }

    fn scratch(name: &str) -> PathBuf {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let dir = std::env::temp_dir().join(format!("swbpipe-report-html-{name}-{}-{nanos}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Every refusal comes before the chooser and the writer.
    fn refused(custody: &RenderCustody, request: RouteRequest) -> RouteError {
        let error = save_in(custody, request, |_| panic!("no chooser on a refusal"), |_, _| panic!("no write on a refusal")).unwrap_err();
        assert_eq!((error.stage.as_str(), error.destination), ("admission", "untouched"), "{error:?}");
        error
    }

    #[test]
    fn no_current_render_is_refused_before_the_chooser() {
        let custody = RenderCustody::new();
        assert_eq!(refused(&custody, request(HTML, PUBLIC_REPORT, false)).code, NOT_CURRENT);
        let custody = rendered(Some(PUBLIC_REPORT));
        assert!(custody.retire());
        assert_eq!(refused(&custody, request(HTML, PUBLIC_REPORT, false)).code, NOT_CURRENT);
    }

    #[test]
    fn a_blocked_or_withheld_render_is_refused_before_the_chooser() {
        let sha = sha256_hex(HTML.as_bytes());
        for (blocked, withheld) in [(true, false), (false, true)] {
            let custody = RenderCustody::new();
            custody.record(Some(PUBLIC_REPORT), HTML, &sha, blocked, withheld);
            assert_eq!(refused(&custody, request(HTML, PUBLIC_REPORT, false)).code, BLOCKED);
        }
        // A canonical HTML that does not hash to the renderer's named hash is held blocked.
        let custody = RenderCustody::new();
        custody.record(Some(PUBLIC_REPORT), HTML, &"0".repeat(64), false, false);
        assert_eq!(refused(&custody, request(HTML, PUBLIC_REPORT, false)).code, BLOCKED);
    }

    #[test]
    fn html_the_render_did_not_produce_is_refused() {
        let custody = rendered(Some(PUBLIC_REPORT));
        let substituted = HTML.replace("Invented report", "Substituted report");
        assert_eq!(refused(&custody, request(&substituted, PUBLIC_REPORT, false)).code, UNBOUND);
        // One byte more, as a trailing newline would be.
        assert_eq!(refused(&custody, request(&format!("{HTML}\n"), PUBLIC_REPORT, false)).code, UNBOUND);
        // An earlier render's HTML is not the current one's.
        custody.record(Some(PUBLIC_REPORT), "<p>later</p>", &sha256_hex(b"<p>later</p>"), false, false);
        assert_eq!(refused(&custody, request(HTML, PUBLIC_REPORT, false)).code, UNBOUND);
    }

    #[test]
    fn a_context_the_route_does_not_admit_is_refused_by_name() {
        for context in [Some("user_released_report"), Some("downstream_tool"), Some("public_example"), None] {
            let custody = rendered(context);
            let error = refused(&custody, request(HTML, context.unwrap_or(PUBLIC_REPORT), true));
            assert_eq!(error.code, CONTEXT_NOT_ADMITTED);
            assert!(!error.message.contains("user_released_report"), "the refusal names no value from the input");
        }
    }

    #[test]
    fn the_request_context_intent_and_evidence_must_agree_with_the_record() {
        let custody = rendered(Some(LOCAL_PRIVATE));
        assert_eq!(refused(&custody, request(HTML, PUBLIC_REPORT, false)).code, CONTEXT_MISMATCH);
        assert_eq!(refused(&custody, request(HTML, LOCAL_PRIVATE, false)).code, INTENT_REQUIRED);
        let public = rendered(Some(PUBLIC_REPORT));
        assert_eq!(refused(&public, request(HTML, LOCAL_PRIVATE, true)).code, CONTEXT_MISMATCH);
        let mutations: [fn(&mut RouteEvidence); 5] = [
            |e| e.route_id = "DREP-PACKAGE-SAVE-009".to_string(),
            |e| e.export_context = PUBLIC_REPORT.to_string(),
            |e| e.blocked = true,
            |e| e.blocking_count = 1,
            |e| e.explicit_local_private_intent = false,
        ];
        for mutate in mutations {
            let mut changed = request(HTML, LOCAL_PRIVATE, true);
            mutate(&mut changed.evidence);
            assert_eq!(refused(&custody, changed).code, EVIDENCE_REFUSED);
        }
    }

    #[test]
    fn the_saved_file_is_the_canonical_html_byte_for_byte() {
        let _lock = atomic_save_test_lock();
        for (context, intent, prefix) in [(PUBLIC_REPORT, false, "openpipestress-report-"), (LOCAL_PRIVATE, true, "openpipestress-local-private-report-")] {
            let custody = rendered(Some(context));
            let dir = scratch("bytes");
            let mut suggested = String::new();
            let receipt = save_in(&custody, request(HTML, context, intent), |name| {
                suggested = name.to_string();
                Some(Ok(dir.join(name)))
            }, |path, bytes| atomic::atomic_save_bytes_as(&REPORT_HTML_FORMAT, path, bytes))
            .unwrap();
            let expected = sha256_hex(HTML.as_bytes());
            assert_eq!(suggested, format!("{prefix}{}.html", &expected[..12]));
            let written = fs::read(dir.join(&suggested)).unwrap();
            assert_eq!(written, HTML.as_bytes());
            // Recomputed here, independently of the route's own digest.
            let recomputed: String = Sha256::digest(&written).iter().map(|b| format!("{b:02x}")).collect();
            assert_eq!(recomputed, expected);
            assert_eq!(receipt.sha256_hex, recomputed);
            assert_eq!(
                (receipt.outcome, receipt.route_id, receipt.export_context.as_str(), receipt.byte_count, receipt.file_name.as_str(), receipt.durability.as_str()),
                ("saved", ROUTE_ID, context, HTML.len(), suggested.as_str(), "durable")
            );
            fs::remove_dir_all(dir).unwrap();
        }
    }

    #[test]
    fn a_cancelled_chooser_writes_nothing() {
        let custody = rendered(Some(PUBLIC_REPORT));
        let receipt = save_in(&custody, request(HTML, PUBLIC_REPORT, false), |_| None, |_, _| panic!("no write after a cancel")).unwrap();
        assert_eq!((receipt.outcome, receipt.durability.as_str()), ("cancelled", "not_applicable"));
        let failed = save_in(&custody, request(HTML, PUBLIC_REPORT, false), |_| Some(Err("/private/path".to_string())), |_, _| panic!("no write")).unwrap_err();
        assert_eq!((failed.code.as_str(), failed.destination), (CHOOSER_FAILED, "untouched"));
        assert!(!failed.message.contains("/private/path"));
    }

    #[test]
    fn a_render_retired_or_replaced_during_the_chooser_is_not_saved() {
        for replace in [false, true] {
            let custody = rendered(Some(PUBLIC_REPORT));
            let dir = scratch("retired");
            let error = save_in(&custody, request(HTML, PUBLIC_REPORT, false), |name| {
                if replace {
                    custody.record(Some(PUBLIC_REPORT), HTML, &sha256_hex(HTML.as_bytes()), false, false);
                } else {
                    custody.retire();
                }
                Some(Ok(dir.join(name)))
            }, |_, _| panic!("no write after retirement"))
            .unwrap_err();
            // Replaced by an identical render is still a new render: the save retires.
            assert_eq!((error.code.as_str(), error.stage.as_str(), error.destination), (RETIRED_DURING_SAVE, "currency", "untouched"));
            assert_eq!(fs::read_dir(&dir).unwrap().count(), 0);
            fs::remove_dir_all(dir).unwrap();
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Fail {
        Write,
        Rename,
        DirectorySync,
    }

    struct Injected(Fail);

    impl AtomicSaveIo for Injected {
        fn write_all(&mut self, temp: &mut File, bytes: &[u8]) -> io::Result<()> {
            if self.0 == Fail::Write {
                return Err(io::Error::other("injected write failure"));
            }
            io::Write::write_all(temp, bytes)
        }
        fn flush(&mut self, temp: &mut File) -> io::Result<()> {
            io::Write::flush(temp)
        }
        fn sync_temp(&mut self, temp: &File) -> io::Result<()> {
            temp.sync_all()
        }
        fn rename(&mut self, source: &Path, destination: &Path) -> io::Result<()> {
            if self.0 == Fail::Rename {
                return Err(io::Error::other("injected rename failure"));
            }
            fs::rename(source, destination)
        }
        fn cleanup_temp(&mut self, temp_path: &Path) -> io::Result<()> {
            fs::remove_file(temp_path)
        }
        fn sync_parent(&mut self, _parent: &Path) -> DirectorySyncResult {
            if self.0 == Fail::DirectorySync {
                return DirectorySyncResult::Failed { kind: "Other".to_string(), raw_os_error: None };
            }
            DirectorySyncResult::Durable
        }
    }

    fn save_with(fail: Fail, dir: &Path) -> Result<HtmlSaveReceipt, RouteError> {
        let custody = rendered(Some(PUBLIC_REPORT));
        save_in(&custody, request(HTML, PUBLIC_REPORT, false), |_| Some(Ok(dir.join("report.html"))), |path, bytes| {
            atomic::atomic_save_bytes_as_with_io(&REPORT_HTML_FORMAT, path, bytes, &mut Injected(fail))
        })
    }

    #[test]
    fn a_failed_write_leaves_the_destination_as_it_was() {
        for fail in [Fail::Write, Fail::Rename] {
            let dir = scratch("failed");
            let destination = dir.join("report.html");
            fs::write(&destination, b"earlier file").unwrap();
            let error = save_with(fail, &dir).unwrap_err();
            assert_eq!(error.destination, "as_it_was");
            assert!(error.code.starts_with("REPORT-HTML-SAVE-"), "{}", error.code);
            assert_eq!(fs::read(&destination).unwrap(), b"earlier file");
            // No temporary file is left beside it.
            assert_eq!(fs::read_dir(&dir).unwrap().count(), 1);
            fs::remove_dir_all(dir).unwrap();
        }
    }

    #[test]
    fn an_unconfirmed_directory_sync_reports_the_published_file_as_not_durable() {
        let dir = scratch("durability");
        let receipt = save_with(Fail::DirectorySync, &dir).unwrap();
        assert_eq!((receipt.outcome, receipt.durability.as_str()), ("saved", "saved_durability_uncertain"));
        assert_eq!(fs::read(dir.join("report.html")).unwrap(), HTML.as_bytes());
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn the_writer_admits_only_the_html_extension() {
        let dir = scratch("extension");
        let custody = rendered(Some(PUBLIC_REPORT));
        let error = save_in(&custody, request(HTML, PUBLIC_REPORT, false), |_| Some(Ok(dir.join("report.htm"))), |path, bytes| {
            atomic::atomic_save_bytes_as(&REPORT_HTML_FORMAT, path, bytes)
        })
        .unwrap_err();
        assert_eq!((error.code.as_str(), error.destination), ("REPORT-HTML-SAVE-DESTINATION-EXTENSION", "as_it_was"));
        assert_eq!(fs::read_dir(&dir).unwrap().count(), 0);
        fs::remove_dir_all(dir).unwrap();
    }

    // Currency regressions (DEL-08-01 REQ-022, AC-011: "A report prepared for a result that is
    // replaced before the save completes is not saved"). They are the independent round-1
    // review's R1 and R2 interleavings of PR 269, kept as maintained tests. Each drives the
    // custody this module records and the actual atomic writer; none takes its expected value
    // from the implementation. They fail on the reviewed behaviour until the repair lands.

    const REPORT_A: &str = "<!DOCTYPE html>\n<html><body><p>Invented report A</p></body></html>\n";
    const REPORT_B: &str = "<!DOCTYPE html>\n<html><body><p>Invented report B</p></body></html>\n";

    /// What a render request's worker does when it completes (`record_render`'s custody step).
    fn complete(custody: &RenderCustody, html: &str) -> u64 {
        custody.record(Some(LOCAL_PRIVATE), html, &sha256_hex(html.as_bytes()), false, false)
    }

    /// The system's file operations, except that after the temporary file is synced (the last
    /// step before publication) another thread retires the render or replaces it with a new
    /// one, as the panel's `retire_rendered_report` or a new render's completion would. The
    /// retirement runs on its own thread so that a repair that guards publication against
    /// retirement can hold it off without deadlocking the test; it is waited for for a bounded
    /// time and the order the writer then observes is recorded.
    struct RetireAfterTempSync {
        custody: std::sync::Arc<RenderCustody>,
        replace: bool,
        retirement: Option<std::thread::JoinHandle<()>>,
        retired: std::sync::Arc<std::sync::atomic::AtomicBool>,
        renamed_after_retirement: Option<bool>,
    }

    impl AtomicSaveIo for RetireAfterTempSync {
        fn write_all(&mut self, temp: &mut File, bytes: &[u8]) -> io::Result<()> {
            atomic::SystemAtomicSaveIo.write_all(temp, bytes)
        }
        fn flush(&mut self, temp: &mut File) -> io::Result<()> {
            atomic::SystemAtomicSaveIo.flush(temp)
        }
        fn sync_temp(&mut self, temp: &File) -> io::Result<()> {
            atomic::SystemAtomicSaveIo.sync_temp(temp)?;
            let (custody, replace, retired) = (std::sync::Arc::clone(&self.custody), self.replace, std::sync::Arc::clone(&self.retired));
            let (sent, received) = std::sync::mpsc::channel();
            self.retirement = Some(std::thread::spawn(move || {
                if replace {
                    complete(&custody, REPORT_B);
                } else {
                    custody.retire();
                }
                retired.store(true, std::sync::atomic::Ordering::SeqCst);
                let _ = sent.send(());
            }));
            // Held off past this bound, the retirement is taken to follow publication.
            let _ = received.recv_timeout(std::time::Duration::from_secs(5));
            Ok(())
        }
        fn rename(&mut self, source: &Path, destination: &Path) -> io::Result<()> {
            self.renamed_after_retirement = Some(self.retired.load(std::sync::atomic::Ordering::SeqCst));
            atomic::SystemAtomicSaveIo.rename(source, destination)
        }
        fn cleanup_temp(&mut self, temp_path: &Path) -> io::Result<()> {
            atomic::SystemAtomicSaveIo.cleanup_temp(temp_path)
        }
    }

    /// R1. Retirement or replacement after the chooser's currency check and after the temporary
    /// file is synced, but before it is renamed over the destination, must not publish the
    /// retired report: the earlier destination stays byte for byte, no temporary file is left
    /// beside it, and the save is refused by a named retirement, not reported `saved`.
    #[test]
    fn a_render_retired_or_replaced_after_the_temporary_file_sync_is_not_published() {
        let _lock = atomic_save_test_lock();
        for replace in [false, true] {
            let custody = std::sync::Arc::new(RenderCustody::new());
            complete(&custody, REPORT_A);
            let dir = scratch("retired-before-rename");
            let destination = dir.join("report.html");
            let earlier = b"earlier destination file, invented".to_vec();
            fs::write(&destination, &earlier).unwrap();
            let mut io = RetireAfterTempSync {
                custody: std::sync::Arc::clone(&custody),
                replace,
                retirement: None,
                retired: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
                renamed_after_retirement: None,
            };
            let outcome = save_in(&custody, request(REPORT_A, LOCAL_PRIVATE, true), |_| Some(Ok(destination.clone())), |path, bytes| {
                atomic::atomic_save_bytes_as_with_io(&REPORT_HTML_FORMAT, path, bytes, &mut io)
            });
            io.retirement.take().expect("the writer reached the temporary-file sync").join().unwrap();
            let left: Vec<String> = fs::read_dir(&dir).unwrap().map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned()).collect();
            let found = fs::read(&destination).unwrap();
            let which = if replace { "replaced by a new render" } else { "retired" };
            match io.renamed_after_retirement {
                // The retirement was held off until the new file was published: that save
                // completed before the render was retired, and is reported as saved.
                Some(false) => {
                    assert_eq!(outcome.as_ref().map(|receipt| receipt.outcome), Ok("saved"), "{which}: published before retirement");
                    assert_eq!(found, REPORT_A.as_bytes());
                }
                // The render was retired before publication (the reviewed behaviour reaches here).
                renamed => {
                    let what = format!(
                        "{which} after the temporary-file sync and before publication: rename reached after retirement = {renamed:?}, outcome = {:?}, destination now = {:?}, directory = {left:?}",
                        outcome.as_ref().map(|receipt| (receipt.outcome, receipt.durability.as_str())).map_err(|error| error.code.as_str()),
                        String::from_utf8_lossy(&found)
                    );
                    assert_eq!(renamed, None, "the retired report must not be renamed over the destination; {what}");
                    assert_eq!(found, earlier, "the earlier destination must stay byte for byte; {what}");
                    assert_eq!(left, vec!["report.html".to_string()], "no temporary file is left beside the destination; {what}");
                    let error = outcome.expect_err(&format!("a retired save is refused, not saved; {what}"));
                    assert!(error.code.starts_with("REPORT-HTML-SAVE-") && error.code.contains("RETIRED"), "the refusal names the retirement; {what}");
                }
            }
            fs::remove_dir_all(dir).unwrap();
        }
    }

    /// R2. Two render requests overlap: A starts, B starts (each request retires the record at
    /// entry, as `render_calculation_report` does first), B completes and is shown and its print
    /// window opened, then A's worker completes late. B must stay the current render, admissible
    /// to save and print and its window served and printed; the obsolete A must not be admitted.
    #[test]
    fn a_late_completion_of_an_earlier_render_request_does_not_replace_the_newer_render() {
        use crate::rendered_report_print::{admit_print_in, on_load_in, served_document_in, OnLoad, OpenPrint};
        let custody = RenderCustody::new();
        custody.retire(); // request A enters; its worker is still rendering
        custody.retire(); // request B enters (the selection or source changed meanwhile)
        complete(&custody, REPORT_B); // B completes; the panel shows it
        let document = admit_print_in(&custody, &request(REPORT_B, LOCAL_PRIVATE, true)).expect("B is current once it completes");
        let path = format!("/{}", document.print_sha256_hex);
        let open = std::sync::Mutex::new(Some(OpenPrint { label: "report-print-1".to_string(), document: std::sync::Arc::new(document), served: false, printed: false }));
        complete(&custody, REPORT_A); // A's worker completes late; the panel ignores its response
        let code = |result: Result<Admitted, RouteError>| result.map(|_| "admitted".to_string()).unwrap_err_or_admitted();
        // Reported together, so that a failure shows every consequence at once.
        let observed = (
            code(custody.admit(&request(REPORT_B, LOCAL_PRIVATE, true))),
            code(custody.admit(&request(REPORT_A, LOCAL_PRIVATE, true))),
            served_document_in(&open, &custody, "report-print-1", &path).is_some(),
            match on_load_in(&open, &custody, "report-print-1") {
                OnLoad::Print(..) => "print",
                OnLoad::Close => "close",
                OnLoad::Nothing => "nothing",
            },
        );
        assert_eq!(
            observed,
            ("admitted".to_string(), NOT_CURRENT.to_string(), true, "print"),
            "(shown B admitted, obsolete A refused, B's print window served, B's window printed not closed) after a late completion of the earlier request"
        );
    }

    /// R2. A completion that arrives after the render was explicitly retired (the panel's source
    /// changed or it unmounted while the request was in flight) must not make that render
    /// current again.
    #[test]
    fn a_completion_after_explicit_retirement_does_not_make_the_render_current_again() {
        let custody = RenderCustody::new();
        custody.retire(); // request A enters
        custody.retire(); // the panel retires (`retire_rendered_report`) while A is in flight
        complete(&custody, REPORT_A); // A's worker completes afterwards
        let observed = custody.admit(&request(REPORT_A, LOCAL_PRIVATE, true)).map(|_| "admitted".to_string()).unwrap_err_or_admitted();
        assert_eq!(observed, NOT_CURRENT, "a render retired while in flight is refused as not current after its completion");
    }

    trait CodeOrAdmitted {
        fn unwrap_err_or_admitted(self) -> String;
    }

    impl CodeOrAdmitted for Result<String, RouteError> {
        fn unwrap_err_or_admitted(self) -> String {
            self.unwrap_or_else(|error| error.code)
        }
    }
}
