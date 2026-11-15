//! Print… of the rendered report (DEL-08-01 REQ-013, REQ-022; DEL-12-02
//! AC-007), route `DREP-HTML-SAVE-005` (`rendered_report_routes`).
//!
//! The webview's own print does not reach the system in the desktop: the
//! shell replaces `window.print` only in the main frame (tauri 2.11.1
//! `src/webview/scripts/print.js`, installed for the main frame only at
//! `src/plugin.rs` 370-374), and the main frame's print is of the whole
//! application window. So the desktop admits the canonical HTML against its
//! render custody, derives the print view from those bytes itself
//! (`derived_print_view`: the canonical document with a banner naming its
//! hash), and loads that view into a window of its own, whose webview runs no
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
//! is retired or replaced, and when the main window is destroyed.
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

/// Admits the request against `custody` and derives the print view from the
/// admitted canonical HTML.
pub(crate) fn admit_print_in(custody: &RenderCustody, request: &RouteRequest) -> Result<PrintDocument, RouteError> {
    let admitted = custody.admit(request)?;
    let print_html = open_pipe_stress_report_renderer::derived_print_view(&request.html, &admitted.sha256_hex).into_bytes();
    let print_sha256_hex = routes::sha256_hex(&print_html);
    Ok(PrintDocument {
        generation: admitted.generation,
        export_context: admitted.export_context,
        canonical_sha256_hex: admitted.sha256_hex,
        print_html,
        print_sha256_hex,
    })
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

/// Closes the open print window unless its render is the one of
/// `generation` (`None`: whatever it is). Returns whether an open record was retired
/// and its window was asked to close; the result does not confirm destruction.
pub(crate) fn close_unless_current(generation: Option<u64>) -> bool {
    let label = {
        let mut open = lock(&OPEN);
        match open.as_ref() {
            Some(print) if Some(print.document.generation) != generation => open.take().map(|print| print.label),
            _ => None,
        }
    };
    let retired = label.is_some();
    if let Some(label) = label {
        destroy_window(&label);
    }
    retired
}

/// The print window currently held by the native print path, for the smoke
/// close operation to observe that exact handle disappearing.
#[cfg(feature = "native-smoke")]
pub(crate) fn open_window_label() -> Option<String> {
    lock(&OPEN).as_ref().map(|print| print.label.clone())
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
/// ingress lease, then the print window.
#[tauri::command]
pub(crate) async fn print_rendered_report(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError<RouteError>> {
    let (mut args, lease) = command_ingress::admit(&registry, &invoke, "print_rendered_report", ArgsDecoding::Moved)?;
    let request = routes::route_request_args(&mut args)?;
    drop(args);
    let document = admit_print_in(&routes::CUSTODY, &request);
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
    let label = {
        let mut open = lock(&OPEN);
        if let Some(print) = open.as_ref() {
            let label = print.label.clone();
            drop(open);
            if let Some(window) = app.get_webview_window(&label) {
                let _ = window.set_focus();
            }
            return Err(RouteError::at(PRINT_ALREADY_OPEN, "print", "A print window is already open; it was brought forward. Close it to print again."));
        }
        let label = format!("{LABEL_PREFIX}{}", NEXT_LABEL.fetch_add(1, Ordering::Relaxed));
        *open = Some(OpenPrint { label: label.clone(), document: Arc::new(document), served: false, printed: false });
        label
    };
    *lock(&APP) = Some(app.clone());
    install_main_hook(app);
    let failed = |label: &str| {
        let mut open = lock(&OPEN);
        if open.as_ref().is_some_and(|print| print.label == label) {
            *open = None;
        }
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
            let mut open = lock(&OPEN);
            if open.as_ref().is_some_and(|print| print.label == own) {
                *open = None;
            }
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
                close_unless_current(None);
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
        OnLoad::Print(document, served) => present(window, &document, served),
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
    use crate::rendered_report_routes::{sha256_hex, RouteEvidence, BLOCKED, LOCAL_PRIVATE, NOT_CURRENT, PUBLIC_REPORT, ROUTE_ID, UNBOUND};

    const HTML: &str = "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<title>Invented</title>\n</head>\n<body>\n<p>Invented report</p>\n</body>\n</html>\n";

    fn request(html: &str, context: &str, intent: bool) -> RouteRequest {
        RouteRequest {
            html: html.to_string(),
            export_context: context.to_string(),
            explicit_local_private_intent: intent,
            evidence: RouteEvidence { route_id: ROUTE_ID.to_string(), export_context: context.to_string(), explicit_local_private_intent: intent, blocked: false, blocking_count: 0 },
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

    #[test]
    fn a_refused_print_derives_no_document() {
        assert_eq!(admit_print_in(&RenderCustody::new(), &request(HTML, PUBLIC_REPORT, false)).unwrap_err().code, NOT_CURRENT);
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
        custody.retire();
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
        custody.retire();
        refused(respond_in(&open, &custody, "report-print-7", &path));
    }
}
