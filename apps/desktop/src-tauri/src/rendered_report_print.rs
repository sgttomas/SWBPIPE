//! Print… of the rendered report (DEL-08-01 REQ-013, REQ-022; DEL-12-02
//! AC-007), route `DREP-HTML-SAVE-005` (`rendered_report_routes`).
//!
//! The webview's own print does not reach the system in the desktop: the
//! shell replaces `window.print` only in the main frame (tauri 2.11.1
//! `src/webview/scripts/print.js`, installed for the main frame only at
//! `src/plugin.rs` 370-374), and the main frame's print is of the whole
//! application window. So the desktop admits the request, which names the
//! render by its SHA-256 and carries no document, against its render custody,
//! derives the print view itself from the canonical HTML that custody holds for
//! the current render (`derived_print_view`: the canonical document with a
//! banner naming its hash), and loads that view into a window of its own, whose webview runs no
//! script (`disable_javascript`), keeps no data (`incognito`), navigates
//! nowhere but its one address and is served the document over the
//! [`SCHEME`] protocol with a content security policy that admits no load
//! ([`CSP`]). When the page has loaded it calls the native print, which
//! presents the system print dialog as a sheet on that window (wry 0.55.1
//! `src/wkwebview/mod.rs` 862-893: `printOperationWithPrintInfo:` and
//! `runOperationModalForWindow:` on the webview's own window), so the dialog
//! shows the print view and nothing of the application's window.
//!
//! One print at a time: a second request while a print window is open is
//! refused ([`PRINT_ALREADY_OPEN`]) and the open window is brought forward.
//! wry runs the print operation with no delegate and no completion selector
//! (`src/wkwebview/mod.rs` 888-893), so the desktop cannot learn when the
//! dialog ends; the window closes when the person closes it, when its render
//! is retired or replaced (including by a registered change of the report's
//! source), and when the main window is destroyed.
//!
//! Currency at hand-off (DEL-08-01 REQ-022): when the page has loaded, the
//! view is handed to the native print of its own window only while its render
//! is current, with every change of custody held until the hand-off returns
//! ([`submit_in`]). A change registered before it prevents it and closes the
//! window; a change not yet registered may let the earlier view be handed
//! off. Handing off is not printing: nothing here learns whether the person
//! printed or cancelled.
//!
//! Only macOS presents the dialog: tauri documents the native print as
//! "only supported on macOS on `wry`" (`src/webview/webview_window.rs`
//! 2291-2295), and on Windows wry's print evaluates `window.print()` in the
//! page (`src/webview2/mod.rs` 1712-1717), which a script-free webview is not
//! trusted to run. Elsewhere the command refuses by name
//! ([`PRINT_PLATFORM_UNSUPPORTED`]) after admission, opening nothing.
//!
//! The native-smoke build presents no dialog: at the point of the native
//! print, on the print window's own handle, it records the print document
//! (its bytes, the very document the protocol serves to that window, and its
//! SHA-256) for its driver (`native_smoke::record_report_print`), so no page
//! reaches a printer or a print queue. The record shows which document reached
//! the native print path; it does not show that a dialog appeared, what the
//! system's print preview composed, or how a print job ended.
use crate::command_ingress::{self, respond, ArgsDecoding, CommandError};
use crate::memory_pool::Lease;
use crate::rendered_report_routes::{self as routes, RenderCustody, RouteError, RouteRequest};
use crate::solve_jobs::SolveJobRegistry;
use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, Once, PoisonError};
use tauri::{AppHandle, Manager, Runtime};

/// The protocol that serves the open print window its document.
pub(crate) const SCHEME: &str = "swbpipe-report-print";
/// No load of any kind; the report's own inline style only.
pub(crate) const CSP: &str = "default-src 'none'; style-src 'unsafe-inline'; base-uri 'none'; form-action 'none'";
/// A print window is open.
pub(crate) const PRINT_ALREADY_OPEN: &str = "REPORT-PRINT-ALREADY-OPEN";
/// This platform presents no native print of a separate webview.
#[cfg_attr(target_os = "macos", allow(dead_code))]
pub(crate) const PRINT_PLATFORM_UNSUPPORTED: &str = "REPORT-PRINT-PLATFORM-UNSUPPORTED";
/// The print window could not be created.
pub(crate) const PRINT_WINDOW_FAILED: &str = "REPORT-PRINT-WINDOW-FAILED";
/// The print command's lease could not grow to cover deriving the print view:
/// nothing was derived and no window was opened.
pub(crate) const PRINT_DERIVATION_UNAVAILABLE: &str = "REPORT-PRINT-DERIVATION-UNAVAILABLE";

/// An upper bound on the print view's banner (`derived_print_view`): its
/// fixed markup and text (about 140 bytes in UTF-8) and the canonical hash,
/// which here is always the 64 lowercase hexadecimal characters this desktop
/// computed, which `escape_html` leaves unchanged. Checked by
/// `the_print_derivation_bound_covers_the_derived_view`.
pub(crate) const PRINT_BANNER_MAX_BYTES: usize = 256;

/// The bytes the print command's lease grows by before it derives the print
/// view of a canonical document of `canonical_len` bytes. `derived_print_view`
/// builds the banner (`format!` of the escaped hash: the hash's escaped copy
/// and the banner, each at most [`PRINT_BANNER_MAX_BYTES`], a growing buffer
/// holding at most two blocks at once) and one string of exactly
/// `canonical_len` plus the banner's length: allocated at that capacity when the
/// document has its `<body>` line, otherwise grown by `format!`, again with at
/// most two blocks alive at once. So three banner envelopes and two envelopes
/// of the whole view bound what it allocates (`ingress::allocation_envelope`);
/// the print view's SHA-256 adds 64 bytes, inside the banner allowance.
pub(crate) fn print_derivation_bytes(canonical_len: usize) -> u64 {
    let envelope = |size: usize| crate::ingress::allocation_envelope(size as u64).unwrap_or(u64::MAX);
    let banner = envelope(PRINT_BANNER_MAX_BYTES);
    let view = envelope(canonical_len.saturating_add(PRINT_BANNER_MAX_BYTES));
    banner.saturating_mul(3).saturating_add(view.saturating_mul(2))
}

const LABEL_PREFIX: &str = "report-print-";

/// The print view of an admitted render, derived here from its bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct PrintDocument {
    pub(crate) generation: u64,
    pub(crate) export_context: String,
    pub(crate) canonical_sha256_hex: String,
    pub(crate) print_html: Vec<u8>,
    pub(crate) print_sha256_hex: String,
}

/// The open print window's state.
pub(crate) struct OpenPrint {
    pub(crate) label: String,
    pub(crate) document: Arc<PrintDocument>,
    pub(crate) served: bool,
    pub(crate) printed: bool,
}

static OPEN: Mutex<Option<OpenPrint>> = Mutex::new(None);
static APP: Mutex<Option<AppHandle>> = Mutex::new(None);
static NEXT_LABEL: AtomicU64 = AtomicU64::new(1);
static MAIN_HOOK: Once = Once::new();

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

/// What the print command returns: the print window was opened for this
/// document. Whether the dialog was presented is not reported (module docs).
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub(crate) struct PrintReceipt {
    pub(crate) outcome: &'static str,
    pub(crate) route_id: &'static str,
    pub(crate) export_context: String,
    pub(crate) canonical_sha256_hex: String,
    pub(crate) print_sha256_hex: String,
}

/// The print's refusal when its lease cannot grow by the derivation's bound:
/// the refusal's JSON, then `memory_refusal_note`, which advises waiting only
/// where waiting can cure the refusal.
pub(crate) fn derivation_unavailable(refusal: &crate::memory_pool::ResourceRefusal) -> RouteError {
    let note = routes::memory_refusal_note(refusal, "Try again when more memory is free.");
    RouteError::at(
        PRINT_DERIVATION_UNAVAILABLE,
        "print",
        format!(
            "The desktop could not reserve memory to derive the print view ({}); nothing was printed.{}{note}",
            serde_json::to_string(refusal).unwrap_or_default(),
            if note.is_empty() { "" } else { " " }
        ),
    )
}

/// Admits the request against `custody`, grows the print command's `lease`
/// by the derivation's bound ([`print_derivation_bytes`]; refused
/// [`PRINT_DERIVATION_UNAVAILABLE`] with nothing derived), then takes the held
/// canonical HTML of the admitted render only to derive the print view from
/// it (refused [`routes::NOT_CURRENT`] if that render is no longer current).
pub(crate) fn print_document_in(custody: &RenderCustody, request: &RouteRequest, lease: &mut Lease) -> Result<PrintDocument, RouteError> {
    print_document_between(custody, request, lease, || {})
}

/// [`print_document_in`], running `before_take` after the lease has grown and
/// before the document is taken (a seam for tests at that boundary).
fn print_document_between(custody: &RenderCustody, request: &RouteRequest, lease: &mut Lease, before_take: impl FnOnce()) -> Result<PrintDocument, RouteError> {
    let admitted = custody.admit(request)?;
    lease.grow_to(lease.bytes().saturating_add(print_derivation_bytes(admitted.byte_count))).map_err(|refusal| derivation_unavailable(&refusal))?;
    before_take();
    // Bound to the render the request named: the same generation and hash.
    let document = custody.document_if_current(admitted.generation, &admitted.sha256_hex).ok_or_else(|| {
        RouteError::at(routes::NOT_CURRENT, "print", "The report was retired, rendered again or its source changed before its print view was derived; nothing was printed. Render the report again.")
    })?;
    let print_html = open_pipe_stress_report_renderer::derived_print_view(document.html(), &admitted.sha256_hex).into_bytes();
    drop(document);
    let print_sha256_hex = routes::sha256_hex(&print_html);
    Ok(PrintDocument {
        generation: admitted.generation,
        export_context: admitted.export_context,
        canonical_sha256_hex: admitted.sha256_hex,
        print_html,
        print_sha256_hex,
    })
}

/// Tests' admission and derivation, under an interactive lease of an ample
/// pool of its own that the derivation's bound can grow.
#[cfg(test)]
pub(crate) fn admit_print_in(custody: &RenderCustody, request: &RouteRequest) -> Result<PrintDocument, RouteError> {
    let pool = routes::test_pool();
    let mut lease = pool.acquire_interactive(1).expect("an ample pool admits the command's lease");
    print_document_in(custody, request, &mut lease)
}

/// The document the open print window may be served: only to that window's
/// webview, only at its one path, only while its render is current.
pub(crate) fn served_document_in(open: &Mutex<Option<OpenPrint>>, custody: &RenderCustody, webview_label: &str, path: &str) -> Option<Arc<PrintDocument>> {
    let mut open = lock(open);
    let print = open.as_mut()?;
    if print.label != webview_label || path != format!("/{}", print.document.print_sha256_hex) || !custody.is_current(print.document.generation) {
        return None;
    }
    print.served = true;
    Some(Arc::clone(&print.document))
}

/// What the page load does: print the document once while its render is
/// current (`Some`), or close the window (`None`), or nothing (an already
/// printed window's later load).
pub(crate) enum OnLoad {
    Print(Arc<PrintDocument>, bool),
    Close,
    Nothing,
}

pub(crate) fn on_load_in(open: &Mutex<Option<OpenPrint>>, custody: &RenderCustody, webview_label: &str) -> OnLoad {
    let mut open = lock(open);
    match open.as_mut() {
        Some(print) if print.label == webview_label => {
            if !custody.is_current(print.document.generation) {
                OnLoad::Close
            } else if print.printed {
                OnLoad::Nothing
            } else {
                print.printed = true;
                OnLoad::Print(Arc::clone(&print.document), print.served)
            }
        }
        _ => OnLoad::Close,
    }
}

/// The hand-off of a loaded print view to the native print of its own window
/// (DEL-08-01 REQ-022): `submit` runs only while the document's render is
/// current, with every change of custody (a retirement, a later render, a
/// registered source change) held until it returns
/// ([`RenderCustody::hand_off_if_current`]). A change registered first prevents
/// it; one registered after it does not undo it. Returns whether it ran;
/// running establishes nothing about the dialog or a print job.
pub(crate) fn submit_in(custody: &RenderCustody, document: &PrintDocument, submit: impl FnOnce()) -> bool {
    custody.hand_off_if_current(document.generation, submit)
}

/// The protocol's response to one request from `webview_label` for `path`:
/// the open print window's document (200) only when [`served_document_in`]
/// admits the request, otherwise no document (404). Every response carries
/// [`CSP`] and `no-store`. `serve` answers with exactly this.
pub(crate) fn respond_in(open: &Mutex<Option<OpenPrint>>, custody: &RenderCustody, webview_label: &str, path: &str) -> tauri::http::Response<Vec<u8>> {
    let (status, body) = match served_document_in(open, custody, webview_label, path) {
        Some(document) => (200, document.print_html.clone()),
        None => (404, Vec::new()),
    };
    tauri::http::Response::builder()
        .status(status)
        .header("Content-Type", "text/html; charset=utf-8")
        .header("Content-Security-Policy", CSP)
        .header("Cache-Control", "no-store")
        .body(body)
        .unwrap_or_else(|_| tauri::http::Response::new(Vec::new()))
}

/// The custom protocol's handler (registered in `run`).
pub(crate) fn serve<R: Runtime>(context: tauri::UriSchemeContext<'_, R>, request: tauri::http::Request<Vec<u8>>) -> tauri::http::Response<Vec<u8>> {
    respond_in(&OPEN, &routes::CUSTODY, context.webview_label(), request.uri().path())
}

/// The print window's address: the protocol's one path, named by the print
/// document's SHA-256.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub(crate) fn print_address(print_sha256_hex: &str) -> Option<tauri::Url> {
    tauri::Url::parse(&format!("{SCHEME}://localhost/{print_sha256_hex}")).ok()
}

/// Whether the print window may navigate to `target`: only to its one address.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub(crate) fn navigation_allowed(allowed: &tauri::Url, target: &tauri::Url) -> bool {
    target == allowed
}

/// The print window's configuration, exactly as `WebviewWindowBuilder::from_config`
/// consumes it (`open_print_window_macos`): its label and address, its title
/// and size, focused and centred, and a webview that runs no script
/// (`javascript_disabled`, applied by tauri-runtime-wry as wry's
/// `with_javascript_disabled`), keeps no data (`incognito`) and has no
/// developer tools (`devtools: Some(false)`).
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub(crate) fn print_window_config(label: &str, address: tauri::Url, canonical_sha256_hex: &str) -> tauri::utils::config::WindowConfig {
    let mut config = tauri::utils::config::WindowConfig::default();
    config.label = label.to_string();
    config.create = false;
    config.url = tauri::WebviewUrl::CustomProtocol(address);
    config.title = format!("Print: calculation report {}", &canonical_sha256_hex[..12]);
    config.width = 840.0;
    config.height = 1000.0;
    config.center = true;
    config.focus = true;
    config.javascript_disabled = true;
    config.incognito = true;
    config.devtools = Some(false);
    config
}

/// Selects, under the open-window lock, the open print record whose render is
/// no longer the CURRENT one in `custody`, and removes only that record; a
/// record whose render is current is kept, whichever operation asks. The
/// open-window lock is held while custody is read (the established order;
/// custody is never held while this lock is taken). Returns the selected
/// window's label, unique for the process, for the caller to destroy after
/// the lock is released.
pub(crate) fn select_retired_in(open: &Mutex<Option<OpenPrint>>, custody: &RenderCustody) -> Option<String> {
    let mut open = lock(open);
    match open.as_ref() {
        Some(print) if !custody.is_current(print.document.generation) => open.take().map(|print| print.label),
        _ => None,
    }
}

/// How a print record's installation went ([`reserve_in`]).
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Reservation {
    /// Installed under this new label. `displaced` is the label of a record
    /// whose render is no longer current, removed to make room, for the caller
    /// to destroy after the locks are released.
    Reserved { label: String, displaced: Option<String> },
    /// A print of the current render is open under this label; it is kept.
    AlreadyOpen(String),
    /// The document's render is no longer current: nothing was installed.
    Retired,
}

/// Installs the print record of an admitted `document` into the one open
/// slot, deciding under the open-window lock and, inside it, a custody read
/// guard held through the insertion itself (the established order; no change
/// of custody can intervene, and no custody change is waited for). A
/// document whose render is no longer current installs nothing; a record of
/// the current render is kept; a record whose render is no longer current is
/// removed and its label returned. Only the record and its label (from
/// `label`) are decided here; no window is built, destroyed or focused.
pub(crate) fn reserve_in(open: &Mutex<Option<OpenPrint>>, custody: &RenderCustody, document: PrintDocument, label: impl FnOnce() -> String) -> Reservation {
    let mut open = lock(open);
    custody.read_currency(|is_current| {
        if !is_current(document.generation) {
            return Reservation::Retired;
        }
        let displaced = match open.as_ref() {
            Some(print) if is_current(print.document.generation) => return Reservation::AlreadyOpen(print.label.clone()),
            Some(_) => open.take().map(|print| print.label),
            None => None,
        };
        let label = label();
        *open = Some(OpenPrint { label: label.clone(), document: Arc::new(document), served: false, printed: false });
        Reservation::Reserved { label, displaced }
    })
}

/// Removes the open record only while it is the window named `label` (its
/// own construction failure or `Destroyed` event); a newer record is kept.
pub(crate) fn release_label_in(open: &Mutex<Option<OpenPrint>>, label: &str) -> bool {
    let mut open = lock(open);
    if open.as_ref().is_some_and(|print| print.label == label) {
        *open = None;
        return true;
    }
    false
}

/// Forced selection: takes the open record, current or not, and returns its
/// label, under one hold of the open-window lock.
pub(crate) fn take_open_in(open: &Mutex<Option<OpenPrint>>) -> Option<String> {
    lock(open).take().map(|print| print.label)
}

/// Closes the print window whose render custody has retired (a retirement,
/// a later render's entry or completion, a registered source advance or an
/// activation), judged against current custody when the close runs, never
/// against the caller's view: a delayed close cannot remove a newer current
/// print. Only the selected window's exact label is destroyed. Returns
/// whether one was selected; it does not confirm destruction.
pub(crate) fn close_retired() -> bool {
    let label = select_retired_in(&OPEN, &routes::CUSTODY);
    let retired = label.is_some();
    if let Some(label) = label {
        destroy_window(&label);
    }
    retired
}

/// Forced closing (the main window's destruction, the native-smoke close
/// request): selects and takes the open record, current or not
/// ([`take_open_in`]), then, with the lock released, requests destruction of
/// that exact label and returns it, so a caller can confirm the window it
/// actually closed. A record installed afterwards has another label and is
/// not touched. Returning the label does not confirm destruction.
pub(crate) fn close_open_print() -> Option<String> {
    let label = take_open_in(&OPEN);
    if let Some(label) = &label {
        destroy_window(label);
    }
    label
}

fn destroy_window(label: &str) {
    let app = lock(&APP).clone();
    if let Some(window) = app.and_then(|app| app.get_webview_window(label)) {
        if let Err(error) = window.destroy() {
            eprintln!("report-print window close failed: {error}");
        }
    }
}

/// Print… (route `DREP-HTML-SAVE-005`): admission and derivation under the
/// ingress lease, grown first by the derivation's bound, then the print
/// window. The window's record keeps the derived view after the command
/// returns, outside the lease.
#[tauri::command]
pub(crate) async fn print_rendered_report(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError<RouteError>> {
    let (mut args, mut lease) = command_ingress::admit(&registry, &invoke, "print_rendered_report", ArgsDecoding::Moved)?;
    let request = routes::route_request_args(&mut args)?;
    drop(args);
    let document = print_document_in(&routes::CUSTODY, &request, &mut lease);
    drop(request);
    let outcome = document.and_then(|document| open_print_window(&app, document));
    let response = respond(outcome.map_err(CommandError::Command));
    drop(lease);
    response
}

fn open_print_window(app: &AppHandle, document: PrintDocument) -> Result<PrintReceipt, RouteError> {
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (app, document);
        Err(RouteError::at(
            PRINT_PLATFORM_UNSUPPORTED,
            "print",
            "Print is presented natively on macOS only; on this platform the report is not printed. Save the canonical HTML and print it from a browser.",
        ))
    }
    #[cfg(target_os = "macos")]
    {
        open_print_window_macos(app, document)
    }
}

#[cfg(target_os = "macos")]
fn open_print_window_macos(app: &AppHandle, document: PrintDocument) -> Result<PrintReceipt, RouteError> {
    use tauri::{webview::PageLoadEvent, WebviewWindowBuilder, WindowEvent};
    let receipt = PrintReceipt {
        outcome: "print_window_opened",
        route_id: routes::ROUTE_ID,
        export_context: document.export_context.clone(),
        canonical_sha256_hex: document.canonical_sha256_hex.clone(),
        print_sha256_hex: document.print_sha256_hex.clone(),
    };
    // DEL-08-01 REQ-022 (M10): the record is installed only while its render is
    // still current, decided with custody held through the insertion; every
    // window operation below runs after both locks are released.
    let label = match reserve_in(&OPEN, &routes::CUSTODY, document, || format!("{LABEL_PREFIX}{}", NEXT_LABEL.fetch_add(1, Ordering::Relaxed))) {
        Reservation::Retired => {
            return Err(RouteError::at(
                routes::NOT_CURRENT,
                "print",
                "The report was retired, rendered again or its source changed before its print window opened; nothing was printed. Render the report again.",
            ))
        }
        Reservation::AlreadyOpen(label) => {
            if let Some(window) = app.get_webview_window(&label) {
                let _ = window.set_focus();
            }
            return Err(RouteError::at(PRINT_ALREADY_OPEN, "print", "A print window is already open; it was brought forward. Close it to print again."));
        }
        Reservation::Reserved { label, displaced } => {
            if let Some(displaced) = displaced {
                destroy_window(&displaced);
            }
            label
        }
    };
    *lock(&APP) = Some(app.clone());
    install_main_hook(app);
    let failed = |label: &str| {
        release_label_in(&OPEN, label);
        RouteError::at(PRINT_WINDOW_FAILED, "print", "The print window could not be opened; nothing was printed.")
    };
    let url = print_address(&receipt.print_sha256_hex).ok_or_else(|| failed(&label))?;
    let allowed = url.clone();
    let config = print_window_config(&label, url, &receipt.canonical_sha256_hex);
    let built = WebviewWindowBuilder::from_config(app, &config)
        .map_err(|_| failed(&label))?
        .on_navigation(move |target| navigation_allowed(&allowed, target))
        .on_page_load(|window, payload| {
            if payload.event() == PageLoadEvent::Finished {
                page_loaded(&window);
            }
        })
        .build();
    let window = built.map_err(|_| failed(&label))?;
    let own = label.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::Destroyed = event {
            release_label_in(&OPEN, &own);
        }
    });
    Ok(receipt)
}

#[cfg(target_os = "macos")]
fn install_main_hook(app: &AppHandle) {
    let Some(main) = app.get_webview_window("main") else { return };
    MAIN_HOOK.call_once(move || {
        main.on_window_event(|event| {
            if let tauri::WindowEvent::Destroyed = event {
                // Terminal owner shutdown: whichever print is open closes.
                let _ = close_open_print();
            }
        });
    });
}

#[cfg(target_os = "macos")]
fn page_loaded(window: &tauri::WebviewWindow) {
    match on_load_in(&OPEN, &routes::CUSTODY, window.label()) {
        OnLoad::Nothing => {}
        OnLoad::Close => {
            let _ = window.destroy();
        }
        OnLoad::Print(document, served) => {
            if !submit_in(&routes::CUSTODY, &document, || present(window, &document, served)) {
                let _ = window.destroy();
            }
        }
    }
}

#[cfg(target_os = "macos")]
fn present(window: &tauri::WebviewWindow, document: &PrintDocument, served: bool) {
    #[cfg(feature = "native-smoke")]
    crate::native_smoke::record_report_print(&document.print_sha256_hex, &document.canonical_sha256_hex, &document.print_html, served, window.label());
    #[cfg(not(feature = "native-smoke"))]
    {
        let _ = (document, served);
        if let Err(error) = window.print() {
            eprintln!("report-print native print failed: {error}");
            let _ = window.destroy();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rendered_report_routes::{sha256_hex, RouteEvidence, SourceMark, BLOCKED, LOCAL_PRIVATE, NOT_CURRENT, PUBLIC_REPORT, ROUTE_ID, UNBOUND};

    const HTML: &str = "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<title>Invented</title>\n</head>\n<body>\n<p>Invented report</p>\n</body>\n</html>\n";

    /// A request naming the render of `html` by its SHA-256; it carries no document.
    fn request(html: &str, context: &str, intent: bool) -> RouteRequest {
        RouteRequest {
            sha256_hex: sha256_hex(html.as_bytes()),
            export_context: context.to_string(),
            explicit_local_private_intent: intent,
            evidence: RouteEvidence { route_id: ROUTE_ID.to_string(), export_context: context.to_string(), explicit_local_private_intent: intent, blocked: false, blocking_count: 0 },
            source: SourceMark { incarnation: 1, sequence: 0 },
        }
    }

    fn rendered(context: &str) -> RenderCustody {
        let custody = RenderCustody::new();
        custody.record(Some(context), HTML, &sha256_hex(HTML.as_bytes()), false, false);
        custody
    }

    fn open_for(document: PrintDocument) -> Mutex<Option<OpenPrint>> {
        Mutex::new(Some(OpenPrint { label: "report-print-7".to_string(), document: Arc::new(document), served: false, printed: false }))
    }

    #[test]
    fn the_print_view_is_derived_here_from_the_admitted_canonical_html() {
        for (context, intent) in [(PUBLIC_REPORT, false), (LOCAL_PRIVATE, true)] {
            let custody = rendered(context);
            let document = admit_print_in(&custody, &request(HTML, context, intent)).unwrap();
            let canonical = sha256_hex(HTML.as_bytes());
            assert_eq!(document.canonical_sha256_hex, canonical);
            let text = String::from_utf8(document.print_html.clone()).unwrap();
            assert_eq!(text, open_pipe_stress_report_renderer::derived_print_view(HTML, &canonical));
            assert!(text.contains(&format!("Canonical rendered-report artifact SHA-256: {canonical}")));
            assert_eq!(document.print_sha256_hex, sha256_hex(text.as_bytes()));
            assert_eq!(document.export_context, context);
        }
    }

    /// The request names the render and carries no document: the view the protocol serves to
    /// the print window is the one derived from the document custody holds for that render.
    #[test]
    fn the_served_print_view_is_derived_from_the_held_document() {
        let custody = rendered(PUBLIC_REPORT);
        let held = custody.held_document().expect("an unblocked render holds its document");
        let named = request(HTML, PUBLIC_REPORT, false);
        assert_eq!(named.sha256_hex, sha256_hex(held.html().as_bytes()));
        let document = admit_print_in(&custody, &named).unwrap();
        let path = format!("/{}", document.print_sha256_hex);
        let open = open_for(document);
        let served = respond_in(&open, &custody, "report-print-7", &path);
        assert_eq!(served.status(), 200);
        let expected = open_pipe_stress_report_renderer::derived_print_view(held.html(), &named.sha256_hex);
        assert_eq!(served.body(), expected.as_bytes());
    }

    /// A blocked, withheld or hash-mismatched render holds no document, and a print naming it is
    /// refused before any view is derived.
    #[test]
    fn a_print_naming_a_render_that_holds_no_document_is_refused() {
        let sha = sha256_hex(HTML.as_bytes());
        for (blocked, withheld, named) in [(true, false, sha.clone()), (false, true, sha.clone()), (false, false, "0".repeat(64))] {
            let custody = RenderCustody::new();
            custody.record(Some(PUBLIC_REPORT), HTML, &named, blocked, withheld);
            assert!(custody.held_document().is_none());
            assert_eq!(admit_print_in(&custody, &request(HTML, PUBLIC_REPORT, false)).unwrap_err().code, BLOCKED);
        }
    }

    /// The bound the print command's lease grows by covers what the derivation allocates: the
    /// banner fits its allowance, the view is the canonical document plus the banner exactly
    /// (with and without a `<body>` line), and two envelopes of the view and three of the
    /// banner allowance are reserved.
    #[test]
    fn the_print_derivation_bound_covers_the_derived_view() {
        let hash = sha256_hex(HTML.as_bytes());
        let banner = open_pipe_stress_report_renderer::derived_print_view("", &hash).len();
        assert!(banner <= PRINT_BANNER_MAX_BYTES, "banner {banner}");
        for canonical in [HTML.to_string(), HTML.replace("<body>\n", "<body>"), "x".repeat(3 << 20)] {
            let view = open_pipe_stress_report_renderer::derived_print_view(&canonical, &hash).len();
            assert_eq!(view, canonical.len() + banner);
            let envelope = |size: usize| crate::ingress::allocation_envelope(size as u64).unwrap();
            assert!(print_derivation_bytes(canonical.len()) >= 2 * envelope(view) + 3 * envelope(banner));
        }
    }

    /// A print whose lease cannot grow by the derivation's bound is refused by name: nothing is
    /// derived (the document is never taken), the lease is as it was, and the command opens no
    /// window (it opens one only from a derived document).
    #[test]
    fn a_print_whose_lease_cannot_grow_derives_nothing() {
        use crate::memory_pool::{test_host::ScriptedHost, MemoryPool, PressureLevel};
        let custody = rendered(PUBLIC_REPORT);
        let held = Arc::downgrade(&custody.held_document().unwrap());
        let host = ScriptedHost::with(Some(16 << 30), Some(64 << 30), PressureLevel::Normal);
        let pool = MemoryPool::new(host.clone());
        let mut lease = pool.acquire_interactive(1).unwrap();
        host.set(|script| script.headroom_bytes = Some(16));
        let mut taken = false;
        let refusal = print_document_between(&custody, &request(HTML, PUBLIC_REPORT, false), &mut lease, || taken = true).unwrap_err();
        assert_eq!((refusal.code.as_str(), refusal.stage.as_str()), (PRINT_DERIVATION_UNAVAILABLE, "print"));
        assert!(refusal.message.contains("\"kind\":\"headroom_insufficient\""), "{}", refusal.message);
        assert!(!taken, "refused before the document is taken");
        assert_eq!((lease.bytes(), held.strong_count()), (1, 1), "nothing grew and nothing holds the document but its record");
    }

    /// The print's take is bound to the render the request named: a different render (another
    /// document and hash) recorded after admission and before the take is not derived in its
    /// place. The print is refused as not current, nothing is derived, and the pool holds the
    /// replacing render's document alone.
    #[test]
    fn a_print_never_derives_a_render_that_replaced_the_named_one() {
        const OTHER: &str = "<!DOCTYPE html>\n<html><body>\n<p>Invented replacing report</p>\n</body></html>\n";
        let pool = routes::test_pool();
        let custody = RenderCustody::new();
        custody.record_in(&pool, Some(PUBLIC_REPORT), HTML, &sha256_hex(HTML.as_bytes()), false, false);
        let lease_pool = routes::test_pool();
        let mut lease = lease_pool.acquire_interactive(1).unwrap();
        let refusal = print_document_between(&custody, &request(HTML, PUBLIC_REPORT, false), &mut lease, || {
            custody.record_in(&pool, Some(PUBLIC_REPORT), OTHER, &sha256_hex(OTHER.as_bytes()), false, false);
        })
        .unwrap_err();
        assert_eq!((refusal.code.as_str(), refusal.stage.as_str()), (NOT_CURRENT, "print"));
        let held = custody.held_document().unwrap();
        assert_eq!(held.html(), OTHER);
        assert_eq!(Arc::strong_count(&held), 2, "the record and this test: the refused print kept no share");
        let bytes = crate::rendered_report_routes::held_document_bytes(OTHER.len());
        assert_eq!((pool.reserved(crate::memory_pool::LeasePurpose::Custody), pool.reserved_total()), (bytes, bytes));
    }

    /// A render of identical bytes recorded after admission (the same hash, a new generation)
    /// is another render: the print is refused as not current and nothing is derived.
    #[test]
    fn a_print_never_derives_an_identical_render_that_replaced_the_named_one() {
        let pool = routes::test_pool();
        let custody = RenderCustody::new();
        custody.record_in(&pool, Some(PUBLIC_REPORT), HTML, &sha256_hex(HTML.as_bytes()), false, false);
        let lease_pool = routes::test_pool();
        let mut lease = lease_pool.acquire_interactive(1).unwrap();
        let refusal = print_document_between(&custody, &request(HTML, PUBLIC_REPORT, false), &mut lease, || {
            custody.record_in(&pool, Some(PUBLIC_REPORT), HTML, &sha256_hex(HTML.as_bytes()), false, false);
        })
        .unwrap_err();
        assert_eq!((refusal.code.as_str(), refusal.stage.as_str()), (NOT_CURRENT, "print"));
        let held = custody.held_document().unwrap();
        assert_eq!(Arc::strong_count(&held), 2, "the record and this test: the refused print derived nothing and kept no share");
        let bytes = crate::rendered_report_routes::held_document_bytes(HTML.len());
        assert_eq!((pool.reserved(crate::memory_pool::LeasePurpose::Custody), pool.reserved_total()), (bytes, bytes));
    }

    /// The print's refusal advises waiting only where waiting can cure it, and says plainly
    /// what is wrong for a document over the per-invocation budget and an unknown capacity;
    /// its code, stage and the refusal's JSON stay.
    #[test]
    fn a_derivation_refusal_advises_waiting_only_where_waiting_can_help() {
        use crate::memory_pool::ResourceRefusal;
        let lead = "The desktop could not reserve memory to derive the print view";
        for (refusal, ending) in [
            (ResourceRefusal::HeadroomInsufficient { required_bytes: 9, available_bytes: 1 }, "nothing was printed. Try again when more memory is free."),
            (ResourceRefusal::PoolExhausted { requested_bytes: 9, reserved_bytes: 1, pool_bytes: 2 }, "nothing was printed. Try again when more memory is free."),
            (ResourceRefusal::IngressExceedsBudget { required_bytes: 9, budget_bytes: 1 }, "nothing was printed. The document is larger than the memory the desktop allows one command."),
            (ResourceRefusal::CapacityUnknown, "nothing was printed. The desktop could not read the machine's memory."),
        ] {
            let error = derivation_unavailable(&refusal);
            let json = serde_json::to_string(&refusal).unwrap();
            assert_eq!((error.code.as_str(), error.stage.as_str()), (PRINT_DERIVATION_UNAVAILABLE, "print"));
            assert_eq!(error.message, format!("{lead} ({json}); {ending}"));
        }
    }

    #[test]
    fn a_refused_print_derives_no_document() {
        let none = RenderCustody::new();
        none.activate();
        assert_eq!(admit_print_in(&none, &request(HTML, PUBLIC_REPORT, false)).unwrap_err().code, NOT_CURRENT);
        let blocked = RenderCustody::new();
        blocked.record(Some(PUBLIC_REPORT), HTML, &sha256_hex(HTML.as_bytes()), true, false);
        assert_eq!(admit_print_in(&blocked, &request(HTML, PUBLIC_REPORT, false)).unwrap_err().code, BLOCKED);
        let custody = rendered(PUBLIC_REPORT);
        let other = HTML.replace("Invented", "Other");
        assert_eq!(admit_print_in(&custody, &request(&other, PUBLIC_REPORT, false)).unwrap_err().code, UNBOUND);
    }

    #[test]
    fn the_document_is_served_only_to_its_window_at_its_path_while_current() {
        let custody = rendered(PUBLIC_REPORT);
        let document = admit_print_in(&custody, &request(HTML, PUBLIC_REPORT, false)).unwrap();
        let path = format!("/{}", document.print_sha256_hex);
        let open = open_for(document.clone());
        assert!(served_document_in(&open, &custody, "main", &path).is_none());
        assert!(served_document_in(&open, &custody, "report-print-7", "/other").is_none());
        assert!(!lock(&open).as_ref().unwrap().served);
        let served = served_document_in(&open, &custody, "report-print-7", &path).unwrap();
        assert_eq!(served.print_html, document.print_html);
        assert!(lock(&open).as_ref().unwrap().served);
        custody.retire_active();
        assert!(served_document_in(&open, &custody, "report-print-7", &path).is_none());
        assert!(served_document_in(&Mutex::new(None), &custody, "report-print-7", &path).is_none());
    }

    #[test]
    fn the_page_load_prints_once_while_current_and_closes_otherwise() {
        let custody = rendered(PUBLIC_REPORT);
        let document = admit_print_in(&custody, &request(HTML, PUBLIC_REPORT, false)).unwrap();
        let open = open_for(document.clone());
        assert!(matches!(on_load_in(&open, &custody, "report-print-8"), OnLoad::Close));
        match on_load_in(&open, &custody, "report-print-7") {
            OnLoad::Print(printed, served) => {
                assert_eq!(printed.print_sha256_hex, document.print_sha256_hex);
                assert!(!served);
            }
            _ => panic!("the first load of a current document prints"),
        }
        assert!(matches!(on_load_in(&open, &custody, "report-print-7"), OnLoad::Nothing));
        custody.record(Some(PUBLIC_REPORT), HTML, &sha256_hex(HTML.as_bytes()), false, false);
        assert!(matches!(on_load_in(&open, &custody, "report-print-7"), OnLoad::Close));
    }

    #[test]
    fn the_policy_admits_no_load() {
        assert!(CSP.starts_with("default-src 'none';"));
        assert!(!CSP.contains("script-src") && !CSP.contains("http") && !CSP.contains('*'));
    }

    /// The configuration `open_print_window_macos` hands to
    /// `WebviewWindowBuilder::from_config`: no script, no kept data, no
    /// developer tools, the print window's label and its one address. This
    /// establishes what the desktop requests of the webview, not what WebKit
    /// does with it.
    #[test]
    fn the_print_window_config_requests_a_scriptless_incognito_webview_at_its_one_address() {
        let canonical = sha256_hex(HTML.as_bytes());
        let print = "c".repeat(64);
        let address = print_address(&print).unwrap();
        assert_eq!(address.as_str(), format!("swbpipe-report-print://localhost/{print}"));
        let config = print_window_config("report-print-7", address.clone(), &canonical);
        assert!(config.javascript_disabled);
        assert!(config.incognito);
        assert_eq!(config.devtools, Some(false));
        assert_eq!(config.label, "report-print-7");
        assert!(config.label.starts_with(LABEL_PREFIX));
        assert_eq!(config.url, tauri::WebviewUrl::CustomProtocol(address));
        assert_eq!(config.title, format!("Print: calculation report {}", &canonical[..12]));
        assert_eq!((config.width, config.height, config.center, config.focus), (840.0, 1000.0, true, true));
        // Nothing else is asked of the window beyond tauri's defaults: no proxy, no
        // parent, no data directory, no extra browser arguments.
        assert!(config.proxy_url.is_none() && config.parent.is_none() && config.data_directory.is_none() && config.additional_browser_args.is_none());
    }

    #[test]
    fn the_print_window_navigates_only_to_its_one_address() {
        let allowed = print_address(&"c".repeat(64)).unwrap();
        assert!(navigation_allowed(&allowed, &allowed.clone()));
        for other in [
            format!("swbpipe-report-print://localhost/{}", "d".repeat(64)),
            format!("swbpipe-report-print://localhost/{}?q=1", "c".repeat(64)),
            format!("swbpipe-report-print://elsewhere/{}", "c".repeat(64)),
            "https://example.invalid/".to_string(),
            "tauri://localhost/".to_string(),
        ] {
            assert!(!navigation_allowed(&allowed, &tauri::Url::parse(&other).unwrap()), "{other}");
        }
    }

    /// The protocol's actual response, as `serve` returns it: the admitted
    /// print document with the policy and `no-store`, or no document. This
    /// establishes the header the desktop emits, not that WebKit enforces
    /// each directive.
    #[test]
    fn the_protocol_responds_with_the_admitted_document_its_policy_and_no_store_or_no_document() {
        let custody = rendered(PUBLIC_REPORT);
        let document = admit_print_in(&custody, &request(HTML, PUBLIC_REPORT, false)).unwrap();
        let path = format!("/{}", document.print_sha256_hex);
        let open = open_for(document.clone());
        let header = |response: &tauri::http::Response<Vec<u8>>, name: &str| response.headers().get(name).map(|value| value.to_str().unwrap().to_string());
        let refused = |response: tauri::http::Response<Vec<u8>>| {
            assert_eq!(response.status(), 404);
            assert!(response.body().is_empty());
            assert_eq!(header(&response, "Content-Security-Policy").as_deref(), Some(CSP));
            assert_eq!(header(&response, "Cache-Control").as_deref(), Some("no-store"));
        };
        refused(respond_in(&open, &custody, "main", &path));
        refused(respond_in(&open, &custody, "report-print-7", "/other"));
        refused(respond_in(&Mutex::new(None), &custody, "report-print-7", &path));
        assert!(!lock(&open).as_ref().unwrap().served);
        let served = respond_in(&open, &custody, "report-print-7", &path);
        assert_eq!(served.status(), 200);
        assert_eq!(served.body(), &document.print_html);
        assert_eq!(sha256_hex(served.body()), document.print_sha256_hex);
        assert_eq!(header(&served, "Content-Security-Policy").as_deref(), Some(CSP));
        assert_eq!(header(&served, "Cache-Control").as_deref(), Some("no-store"));
        assert_eq!(header(&served, "Content-Type").as_deref(), Some("text/html; charset=utf-8"));
        assert!(lock(&open).as_ref().unwrap().served);
        custody.retire_active();
        refused(respond_in(&open, &custody, "report-print-7", &path));
    }

    /// DEL-08-01 REQ-022: the loaded print view is handed to the native print of its own window
    /// only while its render is current. A retirement or a registered source advance before the
    /// hand-off prevents it; one that arrives during it waits for its return and does not undo it.
    #[test]
    fn the_print_hand_off_runs_only_while_current_and_holds_changes_until_it_returns() {
        for change in ["retire", "advance"] {
            let custody = rendered(PUBLIC_REPORT);
            let document = admit_print_in(&custody, &request(HTML, PUBLIC_REPORT, false)).unwrap();
            if change == "retire" {
                custody.retire_active();
            } else {
                custody.advance(SourceMark { incarnation: 1, sequence: 1 }).unwrap();
            }
            let mut submitted = false;
            assert!(!submit_in(&custody, &document, || submitted = true), "{change} before the hand-off prevents it");
            assert!(!submitted);
        }
        let custody = Arc::new(rendered(PUBLIC_REPORT));
        let document = admit_print_in(&custody, &request(HTML, PUBLIC_REPORT, false)).unwrap();
        let changed = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let mut waiter = None;
        let ran = submit_in(&custody, &document, || {
            let (other, flag) = (Arc::clone(&custody), Arc::clone(&changed));
            waiter = Some(std::thread::spawn(move || {
                other.advance(SourceMark { incarnation: 1, sequence: 1 }).unwrap();
                flag.store(true, std::sync::atomic::Ordering::SeqCst);
            }));
            std::thread::sleep(std::time::Duration::from_millis(200));
            // Reads are not held during the hand-off: the window's protocol read does not block.
            assert!(custody.is_current(document.generation));
            assert!(!changed.load(std::sync::atomic::Ordering::SeqCst), "the advance waits for the hand-off's return");
        });
        assert!(ran);
        waiter.unwrap().join().unwrap();
        assert!(changed.load(std::sync::atomic::Ordering::SeqCst));
        assert!(!custody.is_current(document.generation), "the later advance retired the render after the hand-off");
    }

    /// F3 (round-two review): a close judges the record it selects against CURRENT custody.
    /// Advance A registers source 1 and retires the old render but its close is delayed; a
    /// duplicate advance of 1 retires nothing; the new render's entry closes the retired window,
    /// and a print of the new render opens; when A's close finally runs it keeps that current
    /// print. A retired print still closes, and a current one is kept by any close.
    #[test]
    fn a_delayed_close_from_an_earlier_retirement_keeps_the_newer_current_print() {
        let custody = rendered(PUBLIC_REPORT);
        let first = admit_print_in(&custody, &request(HTML, PUBLIC_REPORT, false)).unwrap();
        let open = open_for(first);
        assert_eq!(select_retired_in(&open, &custody), None, "an unchanged current print is kept");
        let one = SourceMark { incarnation: 1, sequence: 1 };
        assert!(custody.advance(one).unwrap().1, "advance A retires the old render; its close is delayed");
        assert_eq!(custody.advance(one).unwrap(), (one, false), "a duplicate advance retires nothing");
        let ticket = custody.begin_next(one).unwrap();
        assert_eq!(select_retired_in(&open, &custody).as_deref(), Some("report-print-7"), "the new render's entry closes the retired window");
        custody.complete(ticket, Some(PUBLIC_REPORT), HTML, &sha256_hex(HTML.as_bytes()), false, false).unwrap();
        let newer = admit_print_in(&custody, &RouteRequest { source: one, ..request(HTML, PUBLIC_REPORT, false) }).unwrap();
        *lock(&open) = Some(OpenPrint { label: "report-print-8".to_string(), document: Arc::new(newer.clone()), served: false, printed: false });
        assert_eq!(select_retired_in(&open, &custody), None, "A's delayed close keeps the newer current print");
        assert_eq!(lock(&open).as_ref().map(|print| print.label.clone()).as_deref(), Some("report-print-8"));
        // An earlier render's completion closing: the window of a replaced render closes.
        let ticket = custody.begin_next(one).unwrap();
        custody.complete(ticket, Some(PUBLIC_REPORT), HTML, &sha256_hex(HTML.as_bytes()), false, false).unwrap();
        assert_eq!(select_retired_in(&open, &custody).as_deref(), Some("report-print-8"));
        assert!(lock(&open).is_none());
        // A retired print still closes.
        let current = admit_print_in(&custody, &RouteRequest { source: one, ..request(HTML, PUBLIC_REPORT, false) }).unwrap();
        *lock(&open) = Some(OpenPrint { label: "report-print-9".to_string(), document: Arc::new(current), served: false, printed: false });
        custody.retire_active();
        assert_eq!(select_retired_in(&open, &custody).as_deref(), Some("report-print-9"));
    }

    // M10 (round-three response): the record is installed only while its document's render is
    // current, decided with custody held through the insertion.

    fn label_from(next: &'static str) -> impl FnOnce() -> String {
        move || next.to_string()
    }

    /// A retirement, or a new render, between admission and installation: the old document
    /// installs nothing and the slot stays free for a valid print.
    #[test]
    fn a_document_retired_between_admission_and_installation_installs_nothing() {
        for change in ["retire", "advance", "new render"] {
            let custody = rendered(PUBLIC_REPORT);
            let document = admit_print_in(&custody, &request(HTML, PUBLIC_REPORT, false)).unwrap();
            let mut source = SourceMark { incarnation: 1, sequence: 0 };
            match change {
                "retire" => assert!(custody.retire_active()),
                "advance" => {
                    source.sequence = 1;
                    custody.advance(source).unwrap();
                }
                _ => {
                    custody.record(Some(PUBLIC_REPORT), HTML, &sha256_hex(HTML.as_bytes()), false, false);
                }
            }
            let open = Mutex::new(None);
            assert_eq!(reserve_in(&open, &custody, document, || panic!("no label for a retired document")), Reservation::Retired, "{change}");
            assert!(lock(&open).is_none(), "{change}: the slot stays free");
            if change == "retire" || change == "advance" {
                custody.record(Some(PUBLIC_REPORT), HTML, &sha256_hex(HTML.as_bytes()), false, false);
            }
            let valid = admit_print_in(&custody, &RouteRequest { source, ..request(HTML, PUBLIC_REPORT, false) }).unwrap();
            assert_eq!(reserve_in(&open, &custody, valid, label_from("report-print-2")), Reservation::Reserved { label: "report-print-2".to_string(), displaced: None }, "{change}");
        }
    }

    /// The opposite order: installed first, then retired. The existing cleanup selects it and
    /// the hand-off gate refuses its submission.
    #[test]
    fn a_document_installed_before_its_retirement_is_cleaned_up_and_never_submitted() {
        let custody = rendered(PUBLIC_REPORT);
        let document = admit_print_in(&custody, &request(HTML, PUBLIC_REPORT, false)).unwrap();
        let open = Mutex::new(None);
        assert_eq!(reserve_in(&open, &custody, document.clone(), label_from("report-print-3")), Reservation::Reserved { label: "report-print-3".to_string(), displaced: None });
        assert!(custody.retire_active());
        assert!(!submit_in(&custody, &document, || panic!("no submission after retirement")));
        assert_eq!(select_retired_in(&open, &custody).as_deref(), Some("report-print-3"));
    }

    /// An occupied slot of the current render is kept and named; a slot whose render is no
    /// longer current (its cleanup still pending) neither blocks nor survives a current print,
    /// and the delayed cleanup then keeps the newer record.
    #[test]
    fn a_current_occupied_slot_is_kept_and_a_retired_one_gives_way() {
        let custody = rendered(PUBLIC_REPORT);
        let document = admit_print_in(&custody, &request(HTML, PUBLIC_REPORT, false)).unwrap();
        let open = Mutex::new(None);
        reserve_in(&open, &custody, document.clone(), label_from("report-print-4"));
        assert_eq!(reserve_in(&open, &custody, document, || panic!("no label while a current print is open")), Reservation::AlreadyOpen("report-print-4".to_string()));
        assert_eq!(lock(&open).as_ref().map(|print| print.label.clone()).as_deref(), Some("report-print-4"));
        custody.record(Some(PUBLIC_REPORT), HTML, &sha256_hex(HTML.as_bytes()), false, false); // its cleanup is delayed
        let newer = admit_print_in(&custody, &request(HTML, PUBLIC_REPORT, false)).unwrap();
        assert_eq!(
            reserve_in(&open, &custody, newer, label_from("report-print-5")),
            Reservation::Reserved { label: "report-print-5".to_string(), displaced: Some("report-print-4".to_string()) }
        );
        assert_eq!(select_retired_in(&open, &custody), None, "the delayed cleanup keeps the newer current print");
        assert!(!release_label_in(&open, "report-print-4"), "the displaced window's own Destroyed event keeps the newer record");
        assert_eq!(lock(&open).as_ref().map(|print| print.label.clone()).as_deref(), Some("report-print-5"));
    }

    /// Installation racing a source advance followed by its cleanup, as the advance command
    /// makes them: whichever runs first, no record of the retired render remains.
    #[test]
    fn an_installation_racing_an_advance_and_its_cleanup_leaves_no_retired_record() {
        for round in 0..50u64 {
            let custody = Arc::new(rendered(PUBLIC_REPORT));
            let document = admit_print_in(&custody, &request(HTML, PUBLIC_REPORT, false)).unwrap();
            let open = Arc::new(Mutex::new(None));
            let barrier = Arc::new(std::sync::Barrier::new(2));
            let installer = {
                let (custody, open, barrier) = (Arc::clone(&custody), Arc::clone(&open), Arc::clone(&barrier));
                std::thread::spawn(move || {
                    barrier.wait();
                    reserve_in(&open, &custody, document, move || format!("report-print-{round}"))
                })
            };
            let changer = {
                let (custody, open, barrier) = (Arc::clone(&custody), Arc::clone(&open), Arc::clone(&barrier));
                std::thread::spawn(move || {
                    barrier.wait();
                    custody.advance(SourceMark { incarnation: 1, sequence: 1 }).unwrap();
                    select_retired_in(&open, &custody)
                })
            };
            let (installed, closed) = (installer.join().unwrap(), changer.join().unwrap());
            assert!(lock(&open).is_none(), "round {round}: installed {installed:?}, closed {closed:?}");
            match installed {
                Reservation::Reserved { label, .. } => assert_eq!(closed, Some(label), "round {round}: installed first, so the cleanup took it"),
                other => assert_eq!((other, closed), (Reservation::Retired, None), "round {round}"),
            }
        }
    }

    // M20 (round-three response): the forced close selects and takes one record and returns its
    // label; a record installed afterwards is untouched.

    fn record_in(open: &Mutex<Option<OpenPrint>>, custody: &RenderCustody, label: &str) {
        let document = admit_print_in(custody, &request(HTML, PUBLIC_REPORT, false)).unwrap();
        *lock(open) = Some(OpenPrint { label: label.to_string(), document: Arc::new(document), served: false, printed: false });
    }

    #[test]
    fn the_forced_selection_returns_the_record_it_took() {
        let custody = rendered(PUBLIC_REPORT);
        let open = Mutex::new(None);
        assert_eq!(take_open_in(&open), None, "no record: nothing selected");
        // Replacement before selection: B is the record taken.
        record_in(&open, &custody, "report-print-10");
        custody.record(Some(PUBLIC_REPORT), HTML, &sha256_hex(HTML.as_bytes()), false, false);
        record_in(&open, &custody, "report-print-11");
        assert_eq!(take_open_in(&open).as_deref(), Some("report-print-11"));
        assert!(lock(&open).is_none());
        // Replacement after selection: A was taken; B, installed afterwards, stays, and A's own
        // Destroyed event does not clear it.
        record_in(&open, &custody, "report-print-12");
        let selected = take_open_in(&open);
        record_in(&open, &custody, "report-print-13");
        assert_eq!(selected.as_deref(), Some("report-print-12"));
        assert!(!release_label_in(&open, "report-print-12"));
        assert_eq!(lock(&open).as_ref().map(|print| print.label.clone()).as_deref(), Some("report-print-13"));
    }
}
