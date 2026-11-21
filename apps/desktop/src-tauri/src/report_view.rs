//! A window that shows a held report document without passing it through the
//! main window's webview (DEL-08-01 REQ-013, REQ-022, REQ-023 "Reopening";
//! DEL-04-07 "Memory budget"; DEL-12-02). Its first provider is the current
//! render ([`open_rendered_report_view`]); a saved report's provider will use
//! the same interface.
//!
//! Threat model (what this module is held to, and what its reviewers read it
//! against):
//!
//! - The window is served nothing but the document its provider handed it
//!   and this module's own wrapper page; only to its own webview; only while
//!   the provider says the document is still showable.
//! - Its webview runs no script, keeps no data, has no developer tools, loads
//!   nothing and navigates nowhere but its own two addresses.
//! - Nothing leaves through it: no save, print, copy-out command or hand-on
//!   of any kind is built. This module defines three commands, the opening of
//!   the current render, the closing of its windows and the take of closure
//!   notices, and none returns any part of a document.
//! - It never extends the life of the current render's document: it keeps
//!   only a weak reference to it ([`ViewBytes::Borrowed`]), and takes a share
//!   only for the length of one serve's copy.
//! - Its memory is reserved: the one copy a serve makes is reserved, in the
//!   interactive class, from the opening until the page has loaded or the
//!   window has closed (below, "Memory").
//!
//! Interface. A provider hands [`open`] a [`ViewDocument`]: an identity
//! ([`ViewIdentity`], a kind and a SHA-256), the bytes ([`ViewBytes`]), a
//! title, at most [`MAX_WORDS`] sentences and at most [`MAX_DETAILS`] label
//! and value pairs, all plain text this module escapes, and `showable`, the
//! provider's answer to whether the document may still be shown. One window
//! per identity: offering an identity already open brings its window forward,
//! sets its title to the one just offered, and changes, serves and reserves
//! nothing else, so the words and details a provider gives must be true for
//! every offer of an identity; only the title follows the last offer. Several
//! identities may be open at once. The window's addresses are derived from the
//! identity only, and no address, error, title or log line here carries a
//! file path or a location.
//!
//! The page. The window loads a wrapper page this module generates: the
//! title, the provider's words and then its details (smaller, muted,
//! fixed-width, selectable and wrapped, never cut), and the document in a
//! frame filling the rest, `sandbox` with no allowance. Two addresses under
//! [`SCHEME`]: the wrapper (`/<kind>/<sha256>`) and the document
//! (`/<kind>/<sha256>/document`). The document is served exactly as held,
//! with the print window's policy ([`DOCUMENT_CSP`], no load of any kind);
//! the wrapper with a policy that admits only the frame of its own document
//! address and its own inline style ([`wrapper_csp`]). Every response carries
//! `no-store`.
//!
//! Serving ([`respond_in`]). A request is answered only when it comes from
//! the webview of the window whose record holds that identity, for exactly
//! that record's wrapper or document address, while its `showable()` is true
//! and, for the document, while its bytes are there; otherwise 404 with no
//! body. The document address is served at most once per window, so exactly
//! one served copy can exist: a reload, or a second frame, gets 404.
//!
//! Established from the pinned sources (tauri 2.11.1, tauri-runtime-wry
//! 2.11.1, wry 0.55.1, as `apps/desktop/src-tauri/Cargo.lock` pins them):
//!
//! - The navigation handler is called for every navigation action WebKit
//!   asks the webview's navigation delegate about, with no filter by frame:
//!   wry 0.55.1 `src/wkwebview/navigation.rs` 50-82 reads only the action's
//!   request URL and calls the handler (or cancels a download, there being no
//!   download handler), and `src/wkwebview/class/wry_navigation_delegate.rs`
//!   118-124 wraps tauri's handler with no frame test; tauri-runtime-wry
//!   2.11.1 `src/lib.rs` 4898-4905 passes it the parsed URL. So the allowed
//!   set is exactly the wrapper and the document address
//!   ([`navigation_allowed`]). Whether WebKit asks the delegate about a
//!   subframe's navigation is WebKit's behaviour, not in these sources; if it
//!   does not, the frame's load is still bounded by the wrapper's policy
//!   (`frame-src` of the one document address) and its sandbox.
//! - Script: `javascript_disabled` is applied by tauri-runtime-wry 2.11.1
//!   `src/lib.rs` 4853-4855 as wry's `with_javascript_disabled`, which wry
//!   0.55.1 `src/wkwebview/mod.rs` 349-356 sets as
//!   `defaultWebpagePreferences.allowsContentJavaScript = false` on the
//!   webview's configuration. Whether WebKit applies that preference to a
//!   frame's document is WebKit's behaviour and cannot be established from
//!   these sources. A frame's script is refused independently twice: the
//!   frame's `sandbox` grants no `allow-scripts`, and the document's policy
//!   admits no script (`default-src 'none'`).
//! - A custom scheme's request reports its webview's label: wry 0.55.1
//!   `src/wkwebview/mod.rs` 251-297 installs one handler per scheme on the
//!   webview's own `WKWebViewConfiguration`, carrying that webview's id, and
//!   `src/wkwebview/class/url_scheme_handler.rs` 71-84 reads that id for
//!   every task it starts, for a frame's or a sub-resource's request as for
//!   the main frame's; tauri-runtime-wry 2.11.1 `src/lib.rs` 4816 makes the
//!   id the webview's label, and tauri 2.11.1 `src/manager/webview.rs`
//!   234-238 hands it to the handler as `webview_label`. No request made in
//!   one webview can report another's label.
//! - The page-load `Finished` event is wry's `didFinishNavigation` (wry 0.55.1
//!   `src/wkwebview/navigation.rs` 39-46), which WebKit sends for the main
//!   frame's navigation.
//!
//! The view window's page cannot call a command: no script runs in its
//! webview, and the frame's document is sandboxed without `allow-scripts`.
//! The closure notices ([`take_report_view_closures`]) are answered only to
//! the main webview.
//!
//! Memory (DEL-04-07 "Memory budget"). The protocol handler must return owned
//! bytes, so a serve copies the document once. That copy is reserved: the
//! open command's own interactive lease grows by the document's allocation
//! envelope ([`serve_copy_bytes`]) before any window or record exists
//! (refused [`VIEW_MEMORY_UNAVAILABLE`], opening nothing), moves into the
//! window's record and is released when the page has loaded, when the window
//! has closed or when its record is removed, whichever is first. A window
//! closed while its one serve is copying releases that reservation before the
//! copy ends. The wrapper page is generated per request and is not reserved:
//! its size is bounded by the provider's title, words and details. A `Borrowed`
//! window keeps a weak reference to the held document: when the document's
//! last holder drops it, its bytes and its custody reservation are released,
//! and only the small shared record's allocation stays until the window's
//! record goes.
//!
//! Lock order. This module's open-windows lock is held while a record's
//! `showable()` is called, so a provider's `showable` must not call back into
//! this module; the current render's reads render custody, the established
//! order of the print window (`rendered_report_print`): its open-window lock,
//! then custody, and custody is never held while this lock is taken. So
//! [`close_unshowable`] is never called with the custody mutex held. The
//! closure notices' lock is taken alone. Leases and documents taken out of a
//! record are dropped after the lock is released, and every window operation
//! (build, focus, title, destroy) runs after it is released.
//!
//! Closing. A window closes when the person closes it, when its provider's
//! `showable()` turns false at a sweep ([`close_unshowable`], run beside every
//! close of a retired print window), when its provider withdraws its kind
//! ([`close_kind`]: the report page asks it when it drops its preview,
//! [`close_rendered_report_views`]), and when the main window is destroyed.
//! Only the sweep and a withdrawal record a closure notice, by kind and
//! reason, which the main webview takes once
//! ([`take_report_view_closures`]); it carries nothing of the document. The window is an ordinary window: not always on top, no
//! special level.
//!
//! Platform. Only macOS opens the window in this version
//! ([`view_windows_supported`], whose one line lifts the gate); elsewhere
//! [`open`] refuses by name ([`VIEW_PLATFORM_UNSUPPORTED`]) after admission,
//! opening nothing. Everything else is compiled, and tested, on every
//! platform.
use crate::command_ingress::{self, respond, ArgsDecoding, CommandError};
use crate::memory_pool::{Lease, ResourceRefusal};
use crate::rendered_report_routes::{self as routes, HeldDocument, RouteError, SourceMark, ViewAdmitted};
use crate::solve_jobs::SolveJobRegistry;
use serde::Serialize;
use std::collections::BTreeSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, Once, PoisonError, Weak};
use tauri::{AppHandle, Manager, Runtime};

/// The platform gate: its one line is what lifts it.
pub(crate) fn view_windows_supported() -> bool {
    std::env::consts::OS == "macos"
}

/// The protocol that serves an open view window its wrapper and document.
pub(crate) const SCHEME: &str = "swbpipe-report-view";
const HOST: &str = "localhost";
/// The document's policy: the print window's, which admits no load of any kind.
pub(crate) const DOCUMENT_CSP: &str = crate::rendered_report_print::CSP;
/// At most this many sentences on the strip.
pub(crate) const MAX_WORDS: usize = 4;
/// At most this many label and value pairs on the strip.
pub(crate) const MAX_DETAILS: usize = 6;
const LABEL_PREFIX: &str = "report-view-";

/// The current render's kind.
pub(crate) const KIND_CURRENT_RENDER: &str = "current_render";
/// The reason a sweep gives for closing a window.
pub(crate) const REASON_NO_LONGER_SHOWABLE: &str = "no_longer_showable";

/// The document offered breaks the interface's bounds (identity form, words, details).
pub(crate) const VIEW_DOCUMENT_REFUSED: &str = "REPORT-VIEW-DOCUMENT-REFUSED";
/// The document is no longer showable, or its bytes are gone, at the opening.
pub(crate) const VIEW_NOT_SHOWABLE: &str = "REPORT-VIEW-NOT-SHOWABLE";
/// The open command's lease could not grow by the serve copy's reservation.
pub(crate) const VIEW_MEMORY_UNAVAILABLE: &str = "REPORT-VIEW-MEMORY-UNAVAILABLE";
/// This platform opens no view window in this version.
pub(crate) const VIEW_PLATFORM_UNSUPPORTED: &str = "REPORT-VIEW-PLATFORM-UNSUPPORTED";
/// The view window could not be created.
pub(crate) const VIEW_WINDOW_FAILED: &str = "REPORT-VIEW-WINDOW-FAILED";

// The words a person sees (DEL-08-01; set by the report's wording lane). Each
// is here once.

/// The view window's title for the current render: "Calculation report", or
/// with the project's name when one is supplied (none is, in this version).
pub(crate) fn current_render_title(project_name: Option<&str>) -> String {
    match project_name {
        Some(name) => format!("Calculation report: {name}"),
        None => "Calculation report".to_string(),
    }
}
/// The current render's sentence on the strip. It says "of the current
/// result" truly only because the window closes when the render stops being
/// current (`a_view_of_the_current_render_closes_at_every_retirement`).
pub(crate) const CURRENT_RENDER_WORDS: &str = "Calculation report of the current result. View only: save or print it from the Report page.";
/// The label of the current render's one detail, the whole canonical hash.
pub(crate) const CURRENT_RENDER_HASH_LABEL: &str = "SHA-256";
/// The frame's accessible title.
pub(crate) const FRAME_TITLE: &str = "Report document";

/// The wrapper's own inline style: the strip's two treatments and the frame.
const WRAPPER_STYLE: &str = "html,body{margin:0;height:100%}\
body{display:flex;flex-direction:column;font-family:-apple-system,BlinkMacSystemFont,\"Helvetica Neue\",sans-serif;font-size:14px;color:#1d1d1f;background:#fff}\
.strip{flex:none;padding:8px 14px;border-bottom:1px solid #d2d2d7;background:#f5f5f7}\
.words p{margin:0 0 4px}\
.details{margin:4px 0 0;font-family:ui-monospace,Menlo,monospace;font-size:11px;color:#6e6e73;-webkit-user-select:text;user-select:text}\
.details div{display:flex;gap:6px;min-width:0}\
.details dt{flex:none;margin:0}\
.details dd{margin:0;min-width:0;overflow-wrap:anywhere;word-break:break-all}\
iframe{flex:1 1 auto;min-height:0;width:100%;border:0}";

/// What a view window shows: one document, named by its kind and SHA-256.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ViewIdentity {
    pub(crate) kind: &'static str,
    pub(crate) sha256_hex: String,
}

impl ViewIdentity {
    /// A kind of lowercase letters and underscores, and a SHA-256 of 64
    /// lowercase hexadecimal characters: nothing an address could carry
    /// beyond them.
    fn is_well_formed(&self) -> bool {
        !self.kind.is_empty()
            && self.kind.bytes().all(|byte| byte.is_ascii_lowercase() || byte == b'_')
            && self.sha256_hex.len() == 64
            && self.sha256_hex.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    }

    pub(crate) fn wrapper_path(&self) -> String {
        format!("/{}/{}", self.kind, self.sha256_hex)
    }

    pub(crate) fn document_path(&self) -> String {
        format!("/{}/{}/document", self.kind, self.sha256_hex)
    }

    pub(crate) fn wrapper_address(&self) -> Option<tauri::Url> {
        tauri::Url::parse(&format!("{SCHEME}://{HOST}{}", self.wrapper_path())).ok()
    }

    pub(crate) fn document_address(&self) -> Option<tauri::Url> {
        tauri::Url::parse(&format!("{SCHEME}://{HOST}{}", self.document_path())).ok()
    }
}

/// The bytes a view window shows.
#[derive(Clone)]
pub(crate) enum ViewBytes {
    /// The window is a holder of the bytes: for a provider whose document
    /// nothing else holds ([`HeldDocument::hold`]).
    #[allow(dead_code)]
    Held(Arc<HeldDocument>),
    /// The window never keeps the bytes alive: each serve takes a share only
    /// for its copy, and gets nothing once they are gone. The current render
    /// uses this, so a retirement releases its document whatever windows are
    /// open.
    Borrowed(Weak<HeldDocument>),
}

impl ViewBytes {
    /// A share of the bytes for one use, if they are still there.
    fn share(&self) -> Option<Arc<HeldDocument>> {
        match self {
            Self::Held(document) => Some(Arc::clone(document)),
            Self::Borrowed(document) => document.upgrade(),
        }
    }
}

/// The provider's answer to whether its document may still be shown. It is
/// called with this module's open-windows lock held (module docs, "Lock
/// order") and must not call back into this module.
pub(crate) type Showable = Arc<dyn Fn() -> bool + Send + Sync>;

/// What a provider offers [`open`] (module docs, "Interface").
pub(crate) struct ViewDocument {
    pub(crate) identity: ViewIdentity,
    pub(crate) document: ViewBytes,
    /// The window's title; the one field a later offer of an open identity changes.
    pub(crate) title: String,
    /// At most [`MAX_WORDS`] sentences, plain text. True for every offer of the identity.
    pub(crate) words: Vec<String>,
    /// At most [`MAX_DETAILS`] (label, value) pairs, plain text. True for every offer of the identity.
    pub(crate) details: Vec<(String, String)>,
    /// Carried in the record and read by nothing (`may_leave_is_read_by_nothing`).
    pub(crate) may_leave: bool,
    pub(crate) showable: Showable,
}

/// An open view window's record.
pub(crate) struct OpenView {
    label: String,
    identity: ViewIdentity,
    document: ViewBytes,
    title: String,
    words: Vec<String>,
    details: Vec<(String, String)>,
    #[allow(dead_code)]
    may_leave: bool,
    showable: Showable,
    document_served: bool,
    /// The reservation of the one served copy, until the page has loaded.
    lease: Option<Lease>,
}

static OPEN: Mutex<Vec<OpenView>> = Mutex::new(Vec::new());
static CLOSED: Mutex<BTreeSet<(&'static str, &'static str)>> = Mutex::new(BTreeSet::new());
static APP: Mutex<Option<AppHandle>> = Mutex::new(None);
static NEXT_LABEL: AtomicU64 = AtomicU64::new(1);
static MAIN_HOOK: Once = Once::new();

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

/// What the open command returns: `opened` or `brought_forward`, with the
/// identity it concerns.
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub(crate) struct ViewReceipt {
    pub(crate) outcome: &'static str,
    pub(crate) kind: &'static str,
    pub(crate) sha256_hex: String,
}

/// A view module refusal: the routes' refusal shape.
pub(crate) type ViewError = RouteError;

fn refused(code: &str, message: impl Into<String>) -> ViewError {
    RouteError::at(code, "view", message)
}

/// The bytes the open command's lease grows by: the allocation envelope of
/// the one copy a serve makes of a document of `len` bytes
/// (`ingress::allocation_envelope`: the copy is one exact `Vec<u8>`).
pub(crate) fn serve_copy_bytes(len: usize) -> u64 {
    crate::ingress::allocation_envelope(len as u64).unwrap_or(u64::MAX)
}

/// The refusal when the open command's lease cannot grow by the serve copy's
/// reservation: the refusal's JSON, then `memory_refusal_note`.
pub(crate) fn memory_unavailable(refusal: &ResourceRefusal) -> ViewError {
    let note = routes::memory_refusal_note(refusal, "Try again when more memory is free.");
    refused(
        VIEW_MEMORY_UNAVAILABLE,
        format!(
            "The desktop could not reserve memory to show the report in a window ({}); no window was opened.{}{note}",
            serde_json::to_string(refusal).unwrap_or_default(),
            if note.is_empty() { "" } else { " " }
        ),
    )
}

fn platform_unsupported() -> ViewError {
    refused(VIEW_PLATFORM_UNSUPPORTED, "The report opens in a window of its own on macOS only; on this platform no window was opened.")
}

fn not_showable() -> ViewError {
    refused(VIEW_NOT_SHOWABLE, "The report is no longer the one to show: it was retired, replaced or released. No window was opened.")
}

/// The bounds of the interface: a well-formed identity, at most
/// [`MAX_WORDS`] words and [`MAX_DETAILS`] details.
fn validate(document: &ViewDocument) -> Result<(), ViewError> {
    if !document.identity.is_well_formed() {
        return Err(refused(VIEW_DOCUMENT_REFUSED, "The report's identity is not one a view window can be named by; no window was opened."));
    }
    if document.words.len() > MAX_WORDS || document.details.len() > MAX_DETAILS {
        return Err(refused(VIEW_DOCUMENT_REFUSED, format!("A view window shows at most {MAX_WORDS} sentences and {MAX_DETAILS} details; no window was opened.")));
    }
    Ok(())
}

/// How an offer was installed ([`reserve_in`]).
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Reserved {
    /// A record was installed under this label: build its window.
    Opened { label: String, title: String },
    /// The identity is open under this label; its record now carries `title`.
    BroughtForward { label: String, title: String },
}

/// Sets the open record's title for `identity`, if one is open, and returns
/// its label and the new title.
fn bring_forward(open: &mut [OpenView], identity: &ViewIdentity, title: &str) -> Option<(String, String)> {
    let record = open.iter_mut().find(|record| &record.identity == identity)?;
    record.title = title.to_string();
    Some((record.label.clone(), record.title.clone()))
}

/// Installs the record of an offer (module docs, "Interface" and "Memory"):
/// an identity already open is brought forward with nothing grown; otherwise
/// `lease` grows by the serve copy's reservation first (refused
/// [`VIEW_MEMORY_UNAVAILABLE`], nothing recorded), then, under the
/// open-windows lock, the identity is checked again and the record installed
/// only while `showable()` is true, with the lease in it. No window is built
/// here.
pub(crate) fn reserve_in(open: &Mutex<Vec<OpenView>>, document: ViewDocument, mut lease: Lease, label: impl FnOnce() -> String) -> Result<Reserved, ViewError> {
    validate(&document)?;
    if let Some((label, title)) = bring_forward(&mut lock(open), &document.identity, &document.title) {
        return Ok(Reserved::BroughtForward { label, title });
    }
    let len = document.document.share().map(|share| share.html().len()).ok_or_else(not_showable)?;
    lease.grow_to(lease.bytes().saturating_add(serve_copy_bytes(len))).map_err(|refusal| memory_unavailable(&refusal))?;
    let ViewDocument { identity, document, title, words, details, may_leave, showable } = document;
    let mut records = lock(open);
    if let Some((label, title)) = bring_forward(&mut records, &identity, &title) {
        drop(records);
        drop(lease);
        return Ok(Reserved::BroughtForward { label, title });
    }
    if !showable() {
        drop(records);
        drop(lease);
        return Err(not_showable());
    }
    let label = label();
    records.push(OpenView {
        label: label.clone(),
        identity,
        document,
        title: title.clone(),
        words,
        details,
        may_leave,
        showable,
        document_served: false,
        lease: Some(lease),
    });
    Ok(Reserved::Opened { label, title })
}

/// The window named `label` has loaded its page: its serve copy's
/// reservation is released (dropped after the lock). Returns whether its
/// record is still there; a window with none is to be closed.
pub(crate) fn page_loaded_in(open: &Mutex<Vec<OpenView>>, label: &str) -> bool {
    let mut records = lock(open);
    let Some(record) = records.iter_mut().find(|record| record.label == label) else { return false };
    let lease = record.lease.take();
    drop(records);
    drop(lease);
    true
}

/// Removes the record of the window named `label` (its own `Destroyed`
/// event or a failed construction), returning it for the caller to drop
/// after the lock is released.
pub(crate) fn release_label_in(open: &Mutex<Vec<OpenView>>, label: &str) -> Option<OpenView> {
    let mut records = lock(open);
    let at = records.iter().position(|record| record.label == label)?;
    Some(records.swap_remove(at))
}

/// A window a sweep closed: its label and identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ViewClosure {
    pub(crate) label: String,
    pub(crate) identity: ViewIdentity,
}

/// The sweep: for every open record, asks its `showable()` under the
/// open-windows lock and removes those that answer false; then, with the
/// lock released, records a closure notice per kind in `closed` and drops
/// the removed records (their leases and any held bytes). Returns what it
/// closed; the caller destroys those windows.
pub(crate) fn close_unshowable_in(open: &Mutex<Vec<OpenView>>, closed: &Mutex<BTreeSet<(&'static str, &'static str)>>) -> Vec<ViewClosure> {
    close_where_in(open, closed, |record| !(record.showable)())
}

/// The provider's withdrawal: removes every record of `kind`, whatever its
/// `showable()` says (the report page no longer shows that document as
/// current), and records its notice as the sweep does.
pub(crate) fn close_kind_in(open: &Mutex<Vec<OpenView>>, closed: &Mutex<BTreeSet<(&'static str, &'static str)>>, kind: &str) -> Vec<ViewClosure> {
    close_where_in(open, closed, |record| record.identity.kind == kind)
}

fn close_where_in(open: &Mutex<Vec<OpenView>>, closed: &Mutex<BTreeSet<(&'static str, &'static str)>>, select: impl Fn(&OpenView) -> bool) -> Vec<ViewClosure> {
    let mut removed = Vec::new();
    {
        let mut records = lock(open);
        let mut at = 0;
        while at < records.len() {
            if select(&records[at]) {
                removed.push(records.swap_remove(at));
            } else {
                at += 1;
            }
        }
    }
    let closures: Vec<ViewClosure> = removed.iter().map(|record| ViewClosure { label: record.label.clone(), identity: record.identity.clone() }).collect();
    drop(removed);
    if !closures.is_empty() {
        let mut notices = lock(closed);
        for closure in &closures {
            notices.insert((closure.identity.kind, REASON_NO_LONGER_SHOWABLE));
        }
    }
    closures
}

/// Every open window (the main window's destruction, the scripted-run close):
/// takes every record and returns their labels. Records no notice.
pub(crate) fn take_all_in(open: &Mutex<Vec<OpenView>>) -> Vec<String> {
    let removed = std::mem::take(&mut *lock(open));
    removed.iter().map(|record| record.label.clone()).collect()
}

/// A closure notice: a kind and a reason, nothing of the document.
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub(crate) struct ViewClosureNotice {
    pub(crate) kind: &'static str,
    pub(crate) reason: &'static str,
}

/// Takes the pending closure notices, once: for the main webview only; any
/// other caller gets none and clears none.
pub(crate) fn take_closures_in(closed: &Mutex<BTreeSet<(&'static str, &'static str)>>, webview_label: &str) -> Vec<ViewClosureNotice> {
    if webview_label != "main" {
        return Vec::new();
    }
    std::mem::take(&mut *lock(closed)).into_iter().map(|(kind, reason)| ViewClosureNotice { kind, reason }).collect()
}

/// Which part a request was answered with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Part {
    Wrapper,
    Document,
}

/// What the wrapper shows, copied under the lock.
struct WrapperParts {
    identity: ViewIdentity,
    title: String,
    words: Vec<String>,
    details: Vec<(String, String)>,
}

enum Selected {
    Wrapper(WrapperParts),
    Document(ViewIdentity, Arc<HeldDocument>),
    Nothing,
}

/// Under the open-windows lock: the part `webview_label` may be served at
/// `path` (module docs, "Serving"). The document's share is taken here and
/// the record marked as having served it.
fn select_in(open: &Mutex<Vec<OpenView>>, webview_label: &str, path: &str) -> Selected {
    let mut records = lock(open);
    let Some(record) = records.iter_mut().find(|record| record.label == webview_label) else { return Selected::Nothing };
    if !(record.showable)() {
        return Selected::Nothing;
    }
    if path == record.identity.wrapper_path() {
        return Selected::Wrapper(WrapperParts { identity: record.identity.clone(), title: record.title.clone(), words: record.words.clone(), details: record.details.clone() });
    }
    if path == record.identity.document_path() && !record.document_served {
        if let Some(document) = record.document.share() {
            record.document_served = true;
            return Selected::Document(record.identity.clone(), document);
        }
    }
    Selected::Nothing
}

/// The wrapper's policy: only the frame of its own document address and its
/// own inline style.
pub(crate) fn wrapper_csp(identity: &ViewIdentity) -> String {
    let document = identity.document_address().map(|address| address.to_string()).unwrap_or_default();
    format!("default-src 'none'; frame-src {document}; style-src 'unsafe-inline'; base-uri 'none'; form-action 'none'")
}

fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            other => out.push(other),
        }
    }
    out
}

/// The wrapper page: the title, the words, the details and the frame of the
/// document. Every provider text is escaped; no script, no external resource.
fn wrapper_html(parts: &WrapperParts) -> String {
    let mut strip = String::new();
    if !parts.words.is_empty() {
        strip.push_str("<div class=\"words\">");
        for words in &parts.words {
            strip.push_str(&format!("<p>{}</p>", escape(words)));
        }
        strip.push_str("</div>\n");
    }
    if !parts.details.is_empty() {
        strip.push_str("<dl class=\"details\">");
        for (label, value) in &parts.details {
            strip.push_str(&format!("<div><dt>{}</dt><dd>{}</dd></div>", escape(label), escape(value)));
        }
        strip.push_str("</dl>\n");
    }
    let document = parts.identity.document_address().map(|address| address.to_string()).unwrap_or_default();
    format!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<title>{}</title>\n<style>{WRAPPER_STYLE}</style>\n</head>\n<body>\n<header class=\"strip\">\n{strip}</header>\n<iframe sandbox=\"\" src=\"{}\" title=\"{}\"></iframe>\n</body>\n</html>\n",
        escape(&parts.title),
        escape(&document),
        escape(FRAME_TITLE)
    )
}

fn response(status: u16, csp: &str, body: Vec<u8>) -> tauri::http::Response<Vec<u8>> {
    tauri::http::Response::builder()
        .status(status)
        .header("Content-Type", "text/html; charset=utf-8")
        .header("Content-Security-Policy", csp)
        .header("Cache-Control", "no-store")
        .body(body)
        .unwrap_or_else(|_| tauri::http::Response::new(Vec::new()))
}

/// The protocol's response to one request from `webview_label` for `path`
/// (module docs, "Serving"), and which part it served. `between` runs after
/// the document's share is taken and before its copy (a seam for tests).
pub(crate) fn respond_between(open: &Mutex<Vec<OpenView>>, webview_label: &str, path: &str, between: impl FnOnce()) -> (tauri::http::Response<Vec<u8>>, Option<(ViewIdentity, Part)>) {
    match select_in(open, webview_label, path) {
        Selected::Wrapper(parts) => {
            let csp = wrapper_csp(&parts.identity);
            let body = wrapper_html(&parts).into_bytes();
            (response(200, &csp, body), Some((parts.identity, Part::Wrapper)))
        }
        Selected::Document(identity, document) => {
            between();
            // The one served copy, exactly the held bytes; the share ends with it.
            let body = document.html().as_bytes().to_vec();
            drop(document);
            (response(200, DOCUMENT_CSP, body), Some((identity, Part::Document)))
        }
        Selected::Nothing => (response(404, DOCUMENT_CSP, Vec::new()), None),
    }
}

/// Tests' [`respond_between`] with no seam, as [`serve`] answers.
#[cfg(test)]
pub(crate) fn respond_in(open: &Mutex<Vec<OpenView>>, webview_label: &str, path: &str) -> tauri::http::Response<Vec<u8>> {
    respond_between(open, webview_label, path, || {}).0
}

/// The custom protocol's handler (registered in `run`).
pub(crate) fn serve<R: Runtime>(context: tauri::UriSchemeContext<'_, R>, request: tauri::http::Request<Vec<u8>>) -> tauri::http::Response<Vec<u8>> {
    let (response, served) = respond_between(&OPEN, context.webview_label(), request.uri().path(), || {});
    #[cfg(feature = "native-smoke")]
    if let Some((identity, part)) = &served {
        crate::native_smoke::record_report_view(context.webview_label(), identity.kind, &identity.sha256_hex, *part == Part::Document, response.body());
    }
    let _ = served;
    response
}

/// Whether a view window may navigate to `target`: only to its wrapper or
/// its document address (module docs, the navigation handler's fact).
pub(crate) fn navigation_allowed(identity: &ViewIdentity, target: &tauri::Url) -> bool {
    identity.wrapper_address().as_ref() == Some(target) || identity.document_address().as_ref() == Some(target)
}

/// The view window's configuration, as `WebviewWindowBuilder::from_config`
/// consumes it: its label and wrapper address, its title and size, focused
/// and centred, an ordinary window (not always on top), and a webview that
/// runs no script (`javascript_disabled`), keeps no data (`incognito`) and
/// has no developer tools.
pub(crate) fn view_window_config(label: &str, address: tauri::Url, title: &str) -> tauri::utils::config::WindowConfig {
    let mut config = tauri::utils::config::WindowConfig::default();
    config.label = label.to_string();
    config.create = false;
    config.url = tauri::WebviewUrl::CustomProtocol(address);
    config.title = title.to_string();
    config.width = 900.0;
    config.height = 1000.0;
    config.center = true;
    config.focus = true;
    config.always_on_top = false;
    config.javascript_disabled = true;
    config.incognito = true;
    config.devtools = Some(false);
    config
}

/// Opens a window for `document` (module docs). `lease` is the open
/// command's own interactive lease: grown and kept by the record for an
/// opened window, dropped otherwise.
pub(crate) fn open(app: &AppHandle, document: ViewDocument, lease: Lease) -> Result<ViewReceipt, ViewError> {
    if !view_windows_supported() {
        return Err(platform_unsupported());
    }
    let identity = document.identity.clone();
    let receipt = |outcome: &'static str| ViewReceipt { outcome, kind: identity.kind, sha256_hex: identity.sha256_hex.clone() };
    let reserved = reserve_in(&OPEN, document, lease, || format!("{LABEL_PREFIX}{}", NEXT_LABEL.fetch_add(1, Ordering::Relaxed)))?;
    *lock(&APP) = Some(app.clone());
    install_main_hook(app);
    match reserved {
        Reserved::BroughtForward { label, title } => {
            if let Some(window) = app.get_webview_window(&label) {
                let _ = window.set_title(&title);
                let _ = window.set_focus();
            }
            Ok(receipt("brought_forward"))
        }
        Reserved::Opened { label, title } => {
            if build_window(app, &label, &identity, &title).is_err() {
                drop(release_label_in(&OPEN, &label));
                return Err(refused(VIEW_WINDOW_FAILED, "The report's window could not be opened."));
            }
            // A sweep may have removed the record before the window existed.
            if !lock(&OPEN).iter().any(|record| record.label == label) {
                destroy_window(&label);
            }
            Ok(receipt("opened"))
        }
    }
}

fn build_window(app: &AppHandle, label: &str, identity: &ViewIdentity, title: &str) -> Result<(), ()> {
    use tauri::{webview::PageLoadEvent, WebviewWindowBuilder, WindowEvent};
    let address = identity.wrapper_address().ok_or(())?;
    let config = view_window_config(label, address, title);
    let allowed = identity.clone();
    let window = WebviewWindowBuilder::from_config(app, &config)
        .map_err(|_| ())?
        .on_navigation(move |target| navigation_allowed(&allowed, target))
        .on_page_load(|window, payload| {
            if payload.event() == PageLoadEvent::Finished && !page_loaded_in(&OPEN, window.label()) {
                let _ = window.destroy();
            }
        })
        .build()
        .map_err(|_| ())?;
    let own = label.to_string();
    window.on_window_event(move |event| {
        if let WindowEvent::Destroyed = event {
            drop(release_label_in(&OPEN, &own));
        }
    });
    Ok(())
}

fn install_main_hook(app: &AppHandle) {
    let Some(main) = app.get_webview_window("main") else { return };
    MAIN_HOOK.call_once(move || {
        main.on_window_event(|event| {
            if let tauri::WindowEvent::Destroyed = event {
                // Terminal owner shutdown: every view window closes; no notice.
                let _ = close_all();
            }
        });
    });
}

fn destroy_window(label: &str) {
    let app = lock(&APP).clone();
    if let Some(window) = app.and_then(|app| app.get_webview_window(label)) {
        if let Err(error) = window.destroy() {
            eprintln!("report-view window close failed: {error}");
        }
    }
}

/// The sweep (module docs, "Closing"): closes every view window whose
/// provider no longer finds its document showable, recording a notice per
/// kind, and returns what it closed. Never called with the custody mutex
/// held. Returning a closure does not confirm the window's destruction.
pub(crate) fn close_unshowable() -> Vec<ViewClosure> {
    let closures = close_unshowable_in(&OPEN, &CLOSED);
    for closure in &closures {
        destroy_window(&closure.label);
    }
    closures
}

/// Closes every view window of `kind` on its provider's withdrawal
/// ([`close_kind_in`]) and returns what it closed.
pub(crate) fn close_kind(kind: &str) -> Vec<ViewClosure> {
    let closures = close_kind_in(&OPEN, &CLOSED, kind);
    for closure in &closures {
        destroy_window(&closure.label);
    }
    closures
}

/// Closes every view window (the main window's destruction, the scripted
/// run's close) and returns their labels. Records no notice.
pub(crate) fn close_all() -> Vec<String> {
    let labels = take_all_in(&OPEN);
    for label in &labels {
        destroy_window(label);
    }
    labels
}

/// The current render's offer (DEL-08-01 REQ-022): named by its canonical
/// hash, borrowed, showable while `is_current(generation)`.
pub(crate) fn current_render_document(admitted: ViewAdmitted, is_current: impl Fn(u64) -> bool + Send + Sync + 'static) -> ViewDocument {
    let generation = admitted.generation;
    ViewDocument {
        identity: ViewIdentity { kind: KIND_CURRENT_RENDER, sha256_hex: admitted.sha256_hex.clone() },
        document: ViewBytes::Borrowed(admitted.document),
        title: current_render_title(None),
        words: vec![CURRENT_RENDER_WORDS.to_string()],
        details: vec![(CURRENT_RENDER_HASH_LABEL.to_string(), admitted.sha256_hex)],
        may_leave: false,
        showable: Arc::new(move || is_current(generation)),
    }
}

/// Opens the current render in a window of its own (DEL-08-01 REQ-022): the
/// request names the render by its SHA-256 and its source mark and carries no
/// document; admission is the view's ([`routes::RenderCustody::admit_view`]),
/// then [`open`] under the command's ingress lease.
#[tauri::command]
pub(crate) async fn open_rendered_report_view(
    app: AppHandle,
    invoke: tauri::ipc::Request<'_>,
    registry: tauri::State<'_, SolveJobRegistry>,
) -> Result<tauri::ipc::Response, CommandError<RouteError>> {
    let (mut args, lease) = command_ingress::admit(&registry, &invoke, "open_rendered_report_view", ArgsDecoding::Moved)?;
    let sha256_hex: String = args.decode("sha256Hex")?;
    let source: SourceMark = args.decode("source")?;
    drop(args);
    let outcome = routes::CUSTODY
        .admit_view(&sha256_hex, source)
        .map(|admitted| current_render_document(admitted, |generation| routes::CUSTODY.is_current(generation)))
        .and_then(|document| open(&app, document, lease));
    respond(outcome.map_err(CommandError::Command))
}

/// The report page no longer shows the current render as current (its
/// preview was dropped): every window of the current render closes, from the
/// main webview only. The render, and any save or print of it, are untouched.
#[tauri::command]
pub(crate) async fn close_rendered_report_views(webview: tauri::Webview) {
    if webview.label() == "main" {
        let _ = close_kind(KIND_CURRENT_RENDER);
    }
}

/// The pending closure notices, taken once, for the main webview (module
/// docs, "Closing").
#[tauri::command]
pub(crate) fn take_report_view_closures(webview: tauri::Webview) -> Vec<ViewClosureNotice> {
    take_closures_in(&CLOSED, webview.label())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_pool::{test_host::ScriptedHost, LeasePurpose, MemoryPool, PressureLevel};
    use crate::rendered_report_routes::{held_document_bytes, sha256_hex, RenderCustody, BLOCKED, NOT_CURRENT, PUBLIC_REPORT, SOURCE_NOT_REGISTERED, SOURCE_RETIRED, UNBOUND};

    const HTML: &str = "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<title>Invented</title>\n</head>\n<body>\n<p>Invented report</p>\n</body>\n</html>\n";
    const FIRST: SourceMark = SourceMark { incarnation: 1, sequence: 0 };

    fn label(name: &'static str) -> impl FnOnce() -> String {
        move || name.to_string()
    }

    /// A custody holding a render of `HTML` whose document is reserved from `pool`.
    fn rendered(pool: &Arc<MemoryPool>) -> Arc<RenderCustody> {
        let custody = Arc::new(RenderCustody::new());
        custody.record_in(pool, Some(PUBLIC_REPORT), HTML, &sha256_hex(HTML.as_bytes()), false, false);
        custody
    }

    /// The current render's offer against `custody`, as the command builds it.
    fn offer(custody: &Arc<RenderCustody>) -> ViewDocument {
        let admitted = custody.admit_view(&sha256_hex(HTML.as_bytes()), FIRST).expect("the current render is admitted");
        let watched = Arc::clone(custody);
        current_render_document(admitted, move |generation| watched.is_current(generation))
    }

    /// The open command's lease from an ample pool of its own.
    fn command_lease(pool: &Arc<MemoryPool>) -> Lease {
        pool.acquire_interactive(1).expect("an ample pool admits the command's lease")
    }

    fn opened(open: &Mutex<Vec<OpenView>>, document: ViewDocument, pool: &Arc<MemoryPool>, name: &'static str) {
        assert_eq!(
            reserve_in(open, document, command_lease(pool), label(name)).unwrap(),
            Reserved::Opened { label: name.to_string(), title: current_render_title(None) }
        );
    }

    fn header(response: &tauri::http::Response<Vec<u8>>, name: &str) -> Option<String> {
        response.headers().get(name).map(|value| value.to_str().unwrap().to_string())
    }

    fn assert_refused(response: tauri::http::Response<Vec<u8>>) {
        assert_eq!(response.status(), 404);
        assert!(response.body().is_empty());
        assert_eq!(header(&response, "Cache-Control").as_deref(), Some("no-store"));
        assert_eq!(header(&response, "Content-Security-Policy").as_deref(), Some(DOCUMENT_CSP));
    }

    fn identity() -> ViewIdentity {
        ViewIdentity { kind: KIND_CURRENT_RENDER, sha256_hex: sha256_hex(HTML.as_bytes()) }
    }

    fn source_file(name: &str) -> String {
        std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join(name)).unwrap()
    }

    /// This file's production text: everything before its test module.
    fn production(text: &str) -> &str {
        &text[..text.find("#[cfg(test)]\nmod tests").expect("the test module")]
    }

    // ---- Threat model: served only its own document and wrapper, only to its own webview,
    // ---- only while showable.

    #[test]
    fn the_window_is_served_only_its_wrapper_and_document_only_to_its_own_webview() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Mutex::new(Vec::new());
        opened(&open, offer(&custody), &routes::test_pool(), "report-view-1");
        let id = identity();
        assert_refused(respond_in(&open, "main", &id.wrapper_path()));
        assert_refused(respond_in(&open, "report-view-2", &id.document_path()));
        assert_refused(respond_in(&open, "report-view-1", "/"));
        assert_refused(respond_in(&open, "report-view-1", &format!("{}/", id.wrapper_path())));
        assert_refused(respond_in(&open, "report-view-1", &format!("/{}/{}", "saved_report", id.sha256_hex)));
        let wrapper = respond_in(&open, "report-view-1", &id.wrapper_path());
        assert_eq!(wrapper.status(), 200);
        assert_eq!(header(&wrapper, "Content-Security-Policy"), Some(wrapper_csp(&id)));
        assert_eq!(header(&wrapper, "Cache-Control").as_deref(), Some("no-store"));
        let document = respond_in(&open, "report-view-1", &id.document_path());
        assert_eq!(document.status(), 200);
        assert_eq!(document.body(), HTML.as_bytes(), "byte for byte as held");
        assert_eq!(header(&document, "Content-Security-Policy").as_deref(), Some(DOCUMENT_CSP));
        assert_eq!(header(&document, "Cache-Control").as_deref(), Some("no-store"));
        assert_eq!(header(&document, "Content-Type").as_deref(), Some("text/html; charset=utf-8"));
    }

    /// The document address is served at most once per window, so one served copy can exist.
    #[test]
    fn the_document_is_served_at_most_once_per_window() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Mutex::new(Vec::new());
        opened(&open, offer(&custody), &routes::test_pool(), "report-view-1");
        let id = identity();
        assert_eq!(respond_in(&open, "report-view-1", &id.document_path()).status(), 200);
        assert_refused(respond_in(&open, "report-view-1", &id.document_path()));
        assert_eq!(respond_in(&open, "report-view-1", &id.wrapper_path()).status(), 200, "the wrapper carries no document");
    }

    /// Once the provider says the document is not showable, neither address is served, even
    /// before a sweep removes the record.
    #[test]
    fn nothing_is_served_once_the_document_is_not_showable() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Mutex::new(Vec::new());
        opened(&open, offer(&custody), &routes::test_pool(), "report-view-1");
        assert!(custody.retire_active());
        let id = identity();
        assert_refused(respond_in(&open, "report-view-1", &id.wrapper_path()));
        assert_refused(respond_in(&open, "report-view-1", &id.document_path()));
    }

    /// A borrowed document whose holder has dropped it serves nothing, whatever `showable` says.
    #[test]
    fn a_borrowed_document_that_is_gone_serves_nothing() {
        let pool = routes::test_pool();
        let held = HeldDocument::hold(&pool, HTML, &|| false).unwrap();
        let open = Mutex::new(Vec::new());
        let document = ViewDocument {
            identity: identity(),
            document: ViewBytes::Borrowed(Arc::downgrade(&held)),
            title: "Invented".to_string(),
            words: Vec::new(),
            details: Vec::new(),
            may_leave: false,
            showable: Arc::new(|| true),
        };
        assert!(matches!(reserve_in(&open, document, command_lease(&routes::test_pool()), label("report-view-1")).unwrap(), Reserved::Opened { .. }));
        drop(held);
        assert_eq!(pool.reserved_total(), 0);
        assert_refused(respond_in(&open, "report-view-1", &identity().document_path()));
    }

    /// The wrapper admits only its own document's frame and its own inline style; the frame is
    /// sandboxed with no allowance; the page has no script and loads nothing else.
    #[test]
    fn the_wrapper_admits_only_its_document_frame_and_its_own_style() {
        let id = identity();
        let csp = wrapper_csp(&id);
        let document = id.document_address().unwrap().to_string();
        assert_eq!(csp, format!("default-src 'none'; frame-src {document}; style-src 'unsafe-inline'; base-uri 'none'; form-action 'none'"));
        assert!(!csp.contains("script-src") && !csp.contains('*') && !csp.contains("http"));
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Mutex::new(Vec::new());
        opened(&open, offer(&custody), &routes::test_pool(), "report-view-1");
        let wrapper = String::from_utf8(respond_in(&open, "report-view-1", &id.wrapper_path()).into_body()).unwrap();
        assert_eq!(wrapper.matches("<iframe").count(), 1);
        assert!(wrapper.contains(&format!("<iframe sandbox=\"\" src=\"{document}\" title=\"{FRAME_TITLE}\"></iframe>")), "{wrapper}");
        for forbidden in ["<script", "<link", "<img", "<object", "<embed", "<form", "<a ", "<meta http-equiv", "<base", "url(", "@import", "javascript:", "http:", "https:"] {
            assert!(!wrapper.contains(forbidden), "{forbidden} in {wrapper}");
        }
        assert!(DOCUMENT_CSP.starts_with("default-src 'none';") && !DOCUMENT_CSP.contains("script-src"));
    }

    /// The strip: the title, the words, then the details, each provider text escaped; nothing else,
    /// and nothing of the result's standing.
    #[test]
    fn the_strip_shows_the_words_then_the_details_escaped() {
        let parts = WrapperParts {
            identity: identity(),
            title: "<b>Title</b> & co".to_string(),
            words: vec!["First <script>x</script>.".to_string(), "Second \"quoted\" 'single'.".to_string()],
            details: vec![("SHA-256".to_string(), identity().sha256_hex), ("<i>label</i>".to_string(), "a&b".to_string())],
        };
        let page = wrapper_html(&parts);
        assert!(page.contains("<title>&lt;b&gt;Title&lt;/b&gt; &amp; co</title>"));
        assert!(page.contains("<p>First &lt;script&gt;x&lt;/script&gt;.</p><p>Second &quot;quoted&quot; &#39;single&#39;.</p>"));
        assert!(page.contains(&format!("<div><dt>SHA-256</dt><dd>{}</dd></div><div><dt>&lt;i&gt;label&lt;/i&gt;</dt><dd>a&amp;b</dd></div>", identity().sha256_hex)));
        assert!(page.find("class=\"words\"").unwrap() < page.find("class=\"details\"").unwrap());
        assert!(!page.contains("<script"));
        // The details wrap and are never cut: no overflow clipping or ellipsis in the style.
        assert!(WRAPPER_STYLE.contains("overflow-wrap:anywhere") && !WRAPPER_STYLE.contains("ellipsis") && !WRAPPER_STYLE.contains("overflow:hidden"));
        assert!(WRAPPER_STYLE.contains("user-select:text") && WRAPPER_STYLE.contains("monospace"));
    }

    /// The current render's words and detail, as set by the wording lane: an engineer's words first,
    /// the hash after them as a detail, no hash in the title.
    #[test]
    fn the_current_render_is_offered_with_its_words_and_its_hash_as_a_detail() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let document = offer(&custody);
        assert_eq!(document.identity, identity());
        assert_eq!(document.title, "Calculation report");
        assert_eq!(current_render_title(Some("Invented line")), "Calculation report: Invented line");
        assert_eq!(document.words, vec!["Calculation report of the current result. View only: save or print it from the Report page.".to_string()]);
        assert_eq!(document.details, vec![("SHA-256".to_string(), sha256_hex(HTML.as_bytes()))]);
        assert!(matches!(document.document, ViewBytes::Borrowed(_)));
        assert!((document.showable)());
        assert!(custody.retire_active());
        assert!(!(document.showable)(), "the line says \"current\" only while it is");
    }

    /// The configuration handed to `WebviewWindowBuilder::from_config`: no script, no kept data, no
    /// developer tools, an ordinary window (not always on top), at its wrapper address.
    #[test]
    fn the_view_window_config_requests_a_scriptless_incognito_ordinary_window() {
        let address = identity().wrapper_address().unwrap();
        assert_eq!(address.as_str(), format!("swbpipe-report-view://localhost/current_render/{}", identity().sha256_hex));
        let config = view_window_config("report-view-7", address.clone(), "Calculation report");
        assert!(config.javascript_disabled && config.incognito);
        assert_eq!(config.devtools, Some(false));
        assert!(!config.always_on_top && !config.visible_on_all_workspaces && !config.content_protected);
        assert!(config.parent.is_none() && config.proxy_url.is_none() && config.data_directory.is_none() && config.additional_browser_args.is_none());
        assert_eq!(config.url, tauri::WebviewUrl::CustomProtocol(address));
        assert_eq!((config.label.as_str(), config.title.as_str()), ("report-view-7", "Calculation report"));
        assert!(config.label.starts_with(LABEL_PREFIX));
    }

    #[test]
    fn the_view_window_navigates_only_to_its_two_addresses() {
        let id = identity();
        assert!(navigation_allowed(&id, &id.wrapper_address().unwrap()));
        assert!(navigation_allowed(&id, &id.document_address().unwrap()));
        let other = ViewIdentity { kind: KIND_CURRENT_RENDER, sha256_hex: "d".repeat(64) };
        for target in [
            other.wrapper_address().unwrap().to_string(),
            other.document_address().unwrap().to_string(),
            format!("{}?q=1", id.wrapper_address().unwrap()),
            format!("{}#part", id.document_address().unwrap()),
            format!("swbpipe-report-view://elsewhere{}", id.wrapper_path()),
            format!("swbpipe-report-print://localhost/{}", id.sha256_hex),
            "https://example.invalid/".to_string(),
            "about:blank".to_string(),
            "tauri://localhost/".to_string(),
        ] {
            assert!(!navigation_allowed(&id, &tauri::Url::parse(&target).unwrap()), "{target}");
        }
    }

    /// Addresses come from the identity only; an identity that could carry anything else, and
    /// more words or details than the bounds, refuse the open with nothing reserved or recorded.
    #[test]
    fn an_offer_outside_the_bounds_is_refused_with_nothing_reserved() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let make = |edit: &dyn Fn(&mut ViewDocument)| {
            let mut document = offer(&custody);
            edit(&mut document);
            document
        };
        let cases: Vec<ViewDocument> = vec![
            make(&|document| document.words = vec!["w".to_string(); MAX_WORDS + 1]),
            make(&|document| document.details = vec![("l".to_string(), "v".to_string()); MAX_DETAILS + 1]),
            make(&|document| document.identity.kind = "../etc"),
            make(&|document| document.identity.kind = ""),
            make(&|document| document.identity.sha256_hex = "A".repeat(64)),
            make(&|document| document.identity.sha256_hex = format!("{}/x", "a".repeat(62))),
        ];
        for document in cases {
            let lease_pool = routes::test_pool();
            let open = Mutex::new(Vec::new());
            let refusal = reserve_in(&open, document, command_lease(&lease_pool), || panic!("no label for a refused offer")).unwrap_err();
            assert_eq!((refusal.code.as_str(), refusal.stage.as_str()), (VIEW_DOCUMENT_REFUSED, "view"));
            assert!(lock(&open).is_empty());
            assert_eq!(lease_pool.reserved_total(), 0);
        }
        let at_bounds = make(&|document| {
            document.words = vec!["w".to_string(); MAX_WORDS];
            document.details = vec![("l".to_string(), "v".to_string()); MAX_DETAILS];
        });
        assert!(reserve_in(&Mutex::new(Vec::new()), at_bounds, command_lease(&routes::test_pool()), label("report-view-1")).is_ok());
    }

    /// Nothing leaves through a view window: this module defines exactly three commands, the
    /// opening of the current render, the closing of its windows and the take of closure notices,
    /// and none returns a document.
    #[test]
    fn the_module_builds_no_command_through_which_anything_leaves() {
        let text = source_file("report_view.rs");
        let production = production(&text);
        assert_eq!(production.matches("#[tauri::command]").count(), 3);
        assert!(production.contains("#[tauri::command]\npub(crate) async fn open_rendered_report_view("));
        assert!(production.contains("#[tauri::command]\npub(crate) async fn close_rendered_report_views(webview: tauri::Webview) {"));
        assert!(production.contains("#[tauri::command]\npub(crate) fn take_report_view_closures(webview: tauri::Webview) -> Vec<ViewClosureNotice> {"));
        for forbidden in ["DialogExt", "blocking_save_file", ".print()", "clipboard", "std::fs", "File::create", "atomic_report_package_save"] {
            assert!(!production.contains(forbidden), "{forbidden}");
        }
    }

    /// `may_leave` is carried and read by nothing: outside this file's tests, every occurrence in
    /// the crate's source is its definition, its moves in this file's record constructor, or a
    /// literal in a constructor.
    #[test]
    fn may_leave_is_read_by_nothing() {
        fn visit(dir: &std::path::Path, found: &mut Vec<(String, String)>) {
            for entry in std::fs::read_dir(dir).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    visit(&path, found);
                } else if path.extension().is_some_and(|extension| extension == "rs") {
                    let text = std::fs::read_to_string(&path).unwrap();
                    let text = if path.ends_with("report_view.rs") { production(&text).to_string() } else { text };
                    for line in text.lines().filter(|line| line.contains("may_leave")) {
                        found.push((path.file_name().unwrap().to_string_lossy().into_owned(), line.trim().to_string()));
                    }
                }
            }
        }
        let mut found = Vec::new();
        visit(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), &mut found);
        let mut shorthand = 0;
        for (file, line) in &found {
            if line.starts_with("///") || line.starts_with("//") {
                continue;
            }
            match line.as_str() {
                "pub(crate) may_leave: bool," | "may_leave: bool," => assert_eq!(file, "report_view.rs", "{file}: {line}"),
                "may_leave: false," | "may_leave: true," => {}
                "may_leave," => {
                    assert_eq!(file, "report_view.rs", "{file}: {line}");
                    shorthand += 1;
                }
                "let ViewDocument { identity, document, title, words, details, may_leave, showable } = document;" if file == "report_view.rs" => shorthand += 1,
                other => panic!("{file} reads may_leave: {other}"),
            }
        }
        assert_eq!(shorthand, 2, "the record constructor's destructuring and its field, and nothing else");
    }

    // ---- A4: memory.

    /// A refused growth opens nothing, records nothing and reserves nothing; its refusal names the
    /// pool's refusal and closes with the words for its kind.
    #[test]
    fn a_refused_growth_opens_nothing_records_nothing_and_reserves_nothing() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let host = ScriptedHost::with(Some(16 << 30), Some(64 << 30), PressureLevel::Normal);
        let lease_pool = MemoryPool::new(host.clone());
        let lease = lease_pool.acquire_interactive(1).unwrap();
        host.set(|script| script.headroom_bytes = Some(16));
        let open = Mutex::new(Vec::new());
        let refusal = reserve_in(&open, offer(&custody), lease, || panic!("no record for a refused growth")).unwrap_err();
        assert_eq!((refusal.code.as_str(), refusal.stage.as_str()), (VIEW_MEMORY_UNAVAILABLE, "view"));
        assert!(refusal.message.contains("\"kind\":\"headroom_insufficient\""), "{}", refusal.message);
        assert!(refusal.message.ends_with("no window was opened. Try again when more memory is free."), "{}", refusal.message);
        assert!(lock(&open).is_empty());
        assert_eq!(lease_pool.reserved_total(), 0, "the command's lease was dropped and nothing grew");
        assert_eq!(pool.reserved_total(), held_document_bytes(HTML.len()), "the render's document alone");
    }

    #[test]
    fn a_memory_refusal_closes_with_the_words_for_its_kind() {
        let lead = "The desktop could not reserve memory to show the report in a window";
        for (refusal, ending) in [
            (ResourceRefusal::HeadroomInsufficient { required_bytes: 9, available_bytes: 1 }, "no window was opened. Try again when more memory is free."),
            (ResourceRefusal::PoolExhausted { requested_bytes: 9, reserved_bytes: 1, pool_bytes: 2 }, "no window was opened. The desktop's memory pool does not have that much free; it may have room once other work in the desktop has finished."),
            (ResourceRefusal::IngressExceedsBudget { required_bytes: 9, budget_bytes: 1 }, "no window was opened. This needs more memory than the desktop allows one command."),
            (ResourceRefusal::CapacityUnknown, "no window was opened. The desktop could not read the machine's memory."),
        ] {
            let error = memory_unavailable(&refusal);
            assert_eq!(error.code, VIEW_MEMORY_UNAVAILABLE);
            assert_eq!(error.message, format!("{lead} ({}); {ending}", serde_json::to_string(&refusal).unwrap()));
        }
    }

    /// The opening reserves one copy's envelope in the record; a window closed before its page
    /// loads returns it.
    #[test]
    fn a_window_closed_before_its_page_loads_returns_the_reservation() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let lease_pool = routes::test_pool();
        let open = Mutex::new(Vec::new());
        opened(&open, offer(&custody), &lease_pool, "report-view-1");
        assert_eq!(lease_pool.reserved_total(), 1 + serve_copy_bytes(HTML.len()));
        assert!(lease_pool.reserved(LeasePurpose::Ingress) >= serve_copy_bytes(HTML.len()));
        drop(release_label_in(&open, "report-view-1"));
        assert_eq!(lease_pool.reserved_total(), 0);
        assert!(!page_loaded_in(&open, "report-view-1"), "a late page load finds no record: the window is closed");
    }

    #[test]
    fn after_its_page_loads_the_reservation_is_gone_while_the_window_stays_open() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let lease_pool = routes::test_pool();
        let open = Mutex::new(Vec::new());
        opened(&open, offer(&custody), &lease_pool, "report-view-1");
        assert_eq!(respond_in(&open, "report-view-1", &identity().document_path()).status(), 200);
        assert!(page_loaded_in(&open, "report-view-1"));
        assert_eq!(lease_pool.reserved_total(), 0);
        assert_eq!(lock(&open).len(), 1, "the window stays open");
        assert!(page_loaded_in(&open, "report-view-1"), "a later load changes nothing");
        assert_eq!(lease_pool.reserved_total(), 0);
    }

    /// A second offer of an open identity brings it forward: it reserves nothing, serves nothing,
    /// takes the offered title and changes nothing else.
    #[test]
    fn a_brought_forward_offer_takes_its_title_and_reserves_and_serves_nothing() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Mutex::new(Vec::new());
        opened(&open, offer(&custody), &routes::test_pool(), "report-view-1");
        let lease_pool = routes::test_pool();
        let mut again = offer(&custody);
        again.title = "Calculation report: Invented line".to_string();
        again.words = vec!["Other words".to_string()];
        let reserved = reserve_in(&open, again, command_lease(&lease_pool), || panic!("no new record")).unwrap();
        assert_eq!(reserved, Reserved::BroughtForward { label: "report-view-1".to_string(), title: "Calculation report: Invented line".to_string() });
        assert_eq!(lease_pool.reserved_total(), 0, "the command's lease was dropped ungrown");
        let records = lock(&open);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].title, "Calculation report: Invented line");
        assert_eq!(records[0].words, vec![CURRENT_RENDER_WORDS.to_string()], "the words are as first offered");
        assert!(!records[0].document_served, "nothing was served");
    }

    /// One window per identity; several identities at once.
    #[test]
    fn one_window_per_identity_and_several_identities_at_once() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Mutex::new(Vec::new());
        opened(&open, offer(&custody), &routes::test_pool(), "report-view-1");
        let saved = HeldDocument::hold(&pool, "<p>saved</p>", &|| false).unwrap();
        let other = ViewDocument {
            identity: ViewIdentity { kind: "saved_report", sha256_hex: sha256_hex(b"<p>saved</p>") },
            document: ViewBytes::Held(saved),
            title: "Saved".to_string(),
            words: Vec::new(),
            details: Vec::new(),
            may_leave: true,
            showable: Arc::new(|| true),
        };
        assert!(matches!(reserve_in(&open, other, command_lease(&routes::test_pool()), label("report-view-2")).unwrap(), Reserved::Opened { .. }));
        assert_eq!(lock(&open).len(), 2);
        assert_eq!(respond_in(&open, "report-view-2", &format!("/saved_report/{}/document", sha256_hex(b"<p>saved</p>"))).body(), b"<p>saved</p>");
        assert_refused(respond_in(&open, "report-view-2", &identity().document_path()));
    }

    /// A held view keeps its bytes until its record goes; then they and their reservation go.
    #[test]
    fn a_held_view_holds_its_bytes_until_its_record_goes() {
        let pool = routes::test_pool();
        let held = HeldDocument::hold(&pool, HTML, &|| false).unwrap();
        assert_eq!(pool.reserved(LeasePurpose::Custody), held_document_bytes(HTML.len()));
        let open = Mutex::new(Vec::new());
        let document = ViewDocument {
            identity: ViewIdentity { kind: "saved_report", sha256_hex: sha256_hex(HTML.as_bytes()) },
            document: ViewBytes::Held(held),
            title: "Saved".to_string(),
            words: Vec::new(),
            details: Vec::new(),
            may_leave: false,
            showable: Arc::new(|| true),
        };
        assert!(reserve_in(&open, document, command_lease(&routes::test_pool()), label("report-view-1")).is_ok());
        assert_eq!(pool.reserved(LeasePurpose::Custody), held_document_bytes(HTML.len()));
        drop(release_label_in(&open, "report-view-1"));
        assert_eq!(pool.reserved_total(), 0);
    }

    /// The constructor for a holder other than the render reserves, copies and counts as custody
    /// exactly as the render's completion does, and a refusal copies nothing.
    #[test]
    fn a_held_document_for_another_holder_is_reserved_and_counted_as_custody() {
        let pool = routes::test_pool();
        let held = HeldDocument::hold(&pool, HTML, &|| false).unwrap();
        assert_eq!(held.html(), HTML);
        assert_eq!(held.reserved_bytes(), held_document_bytes(HTML.len()));
        assert_eq!((pool.reserved(LeasePurpose::Custody), pool.reserved_total()), (held_document_bytes(HTML.len()), held_document_bytes(HTML.len())));
        drop(held);
        assert_eq!(pool.reserved_total(), 0);
        let refusing = MemoryPool::new(ScriptedHost::with(None, Some(64 << 30), PressureLevel::Normal));
        assert!(HeldDocument::hold(&refusing, HTML, &|| false).is_err());
        assert_eq!(refusing.reserved_total(), 0);
    }

    // ---- B1: the view's admission of the current render.

    /// The view's admission refuses by admission's codes, in admission's order, and admits a render
    /// whatever the route's context, intent and evidence would say; it takes no share.
    #[test]
    fn the_view_admission_refuses_by_the_routes_codes_and_takes_no_share() {
        let sha = sha256_hex(HTML.as_bytes());
        let none = RenderCustody::new();
        none.activate();
        assert_eq!(none.admit_view(&sha, FIRST).unwrap_err().code, NOT_CURRENT);
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        assert_eq!(custody.admit_view(&sha, SourceMark { incarnation: 1, sequence: 1 }).unwrap_err().code, SOURCE_NOT_REGISTERED);
        assert_eq!(custody.admit_view(&"0".repeat(64), FIRST).unwrap_err().code, UNBOUND);
        for (blocked, withheld) in [(true, false), (false, true)] {
            let blocked_custody = RenderCustody::new();
            blocked_custody.record(Some(PUBLIC_REPORT), HTML, &sha, blocked, withheld);
            // A blocked render is refused before its hash is compared, as admission orders them.
            assert_eq!(blocked_custody.admit_view(&"0".repeat(64), FIRST).unwrap_err().code, BLOCKED);
        }
        let admitted = custody.admit_view(&sha, FIRST).unwrap();
        assert_eq!((admitted.sha256_hex.as_str(), admitted.byte_count, admitted.export_context.as_deref()), (sha.as_str(), HTML.len(), Some(PUBLIC_REPORT)));
        assert_eq!(admitted.document.strong_count(), 1, "only the record holds the document");
        // A local-private render is admitted to a view with no intent: nothing leaves.
        let private = RenderCustody::new();
        private.record(Some(routes::LOCAL_PRIVATE), HTML, &sha, false, false);
        assert!(private.admit_view(&sha, FIRST).is_ok());
        custody.advance(SourceMark { incarnation: 1, sequence: 1 }).unwrap();
        assert_eq!(custody.admit_view(&sha, FIRST).unwrap_err().code, SOURCE_RETIRED);
    }

    // ---- B3: every retirement closes the current render's view, and releases its document.

    #[derive(Clone, Copy, Debug)]
    enum Retirement {
        Retire,
        Entry,
        Advance,
        Activation,
        Completion,
    }

    const RETIREMENTS: [Retirement; 5] = [Retirement::Retire, Retirement::Entry, Retirement::Advance, Retirement::Activation, Retirement::Completion];

    fn retire(custody: &RenderCustody, pool: &Arc<MemoryPool>, retirement: Retirement) {
        match retirement {
            Retirement::Retire => assert!(custody.retire_active()),
            Retirement::Entry => drop(custody.begin_next(FIRST).unwrap()),
            Retirement::Advance => assert!(custody.advance(SourceMark { incarnation: 1, sequence: 1 }).unwrap().1),
            Retirement::Activation => {
                let _ = custody.activate();
            }
            // A later render's completion, recorded after its entry.
            Retirement::Completion => {
                let _ = custody.record_in(pool, Some(PUBLIC_REPORT), "<p>later</p>", &sha256_hex(b"<p>later</p>"), false, false);
            }
        }
    }

    /// With a view of the current render open, every retirement closes it at the sweep, records
    /// one notice of its kind, and the retired render's document is released with its reservation,
    /// the window notwithstanding.
    #[test]
    fn a_view_of_the_current_render_closes_at_every_retirement() {
        for retirement in RETIREMENTS {
            let pool = routes::test_pool();
            let custody = rendered(&pool);
            let open = Mutex::new(Vec::new());
            let closed = Mutex::new(BTreeSet::new());
            opened(&open, offer(&custody), &routes::test_pool(), "report-view-1");
            assert!(close_unshowable_in(&open, &closed).is_empty(), "{retirement:?}: a current view is kept");
            retire(&custody, &pool, retirement);
            let later = match retirement {
                Retirement::Completion => held_document_bytes(b"<p>later</p>".len()),
                _ => 0,
            };
            assert_eq!(pool.reserved_total(), later, "{retirement:?}: the retired document is released though its window is open");
            assert_eq!(close_unshowable_in(&open, &closed), vec![ViewClosure { label: "report-view-1".to_string(), identity: identity() }], "{retirement:?}");
            assert!(lock(&open).is_empty());
            assert_eq!(take_closures_in(&closed, "main"), vec![ViewClosureNotice { kind: KIND_CURRENT_RENDER, reason: REASON_NO_LONGER_SHOWABLE }]);
        }
    }

    /// Every close of a retired print window in the routes is followed at once by the view sweep,
    /// outside the custody mutex (the routes' functions hold no custody guard at that point).
    #[test]
    fn every_print_close_in_the_routes_is_followed_by_the_view_sweep() {
        let text = source_file("rendered_report_routes.rs");
        let lines: Vec<&str> = text.lines().collect();
        let closes: Vec<usize> = lines.iter().enumerate().filter(|(_, line)| line.trim() == "crate::rendered_report_print::close_retired();").map(|(at, _)| at).collect();
        assert_eq!(closes.len(), 5, "record_render, begin_render, retire, activate and advance");
        for at in &closes {
            assert_eq!(lines[at + 1].trim(), "crate::report_view::close_unshowable();", "line {}", at + 2);
        }
        assert_eq!(text.matches("crate::report_view::close_unshowable();").count(), 5);
    }

    /// Agent 0's test: a retirement during a serve of the current render's document. The
    /// retirement returns at once; the served bytes are the retired document's own, answered to
    /// that window only; the sweep then closes the window; and the document's reservation returns
    /// to the pool when the serve ends, not before and not later.
    #[test]
    fn a_retirement_during_a_serve_returns_at_once_and_the_reservation_returns_when_the_serve_ends() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Arc::new(Mutex::new(Vec::new()));
        let closed = Arc::new(Mutex::new(BTreeSet::new()));
        opened(&open, offer(&custody), &routes::test_pool(), "report-view-1");
        let held = held_document_bytes(HTML.len());
        let (taken_tx, taken_rx) = std::sync::mpsc::channel();
        let (go_tx, go_rx) = std::sync::mpsc::channel::<()>();
        let server = {
            let open = Arc::clone(&open);
            std::thread::spawn(move || {
                respond_between(&open, "report-view-1", &identity().document_path(), || {
                    taken_tx.send(()).unwrap();
                    go_rx.recv().unwrap();
                })
            })
        };
        taken_rx.recv().unwrap();
        // The serve holds its share and its copy has not begun. The retirement returns at once.
        let started = std::time::Instant::now();
        assert!(custody.retire_active());
        assert!(started.elapsed() < std::time::Duration::from_secs(1), "the retirement did not wait for the serve");
        assert_eq!(pool.reserved(LeasePurpose::Custody), held, "not before: the serve's share keeps the reservation");
        assert_eq!(close_unshowable_in(&open, &closed).len(), 1, "the sweep closes the window during the serve");
        assert_eq!(pool.reserved(LeasePurpose::Custody), held, "still the serve's");
        assert_refused(respond_in(&open, "report-view-2", &identity().document_path()));
        go_tx.send(()).unwrap();
        let (response, served) = server.join().unwrap();
        assert_eq!(response.status(), 200);
        assert_eq!(response.body(), HTML.as_bytes(), "the retired document's own bytes");
        assert_eq!(served, Some((identity(), Part::Document)));
        assert_eq!(pool.reserved_total(), 0, "not later: released as the serve ended");
    }

    /// Only the sweep reports a closure: a window the person closed, and windows closed with the
    /// main window, report nothing. A notice is taken once, by the main webview only, and carries
    /// the kind and the reason and nothing of the document.
    #[test]
    fn only_the_sweep_reports_a_closure_and_the_notice_carries_nothing_of_the_document() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Mutex::new(Vec::new());
        let closed = Mutex::new(BTreeSet::new());
        opened(&open, offer(&custody), &routes::test_pool(), "report-view-1");
        drop(release_label_in(&open, "report-view-1"));
        assert!(custody.retire_active());
        assert!(close_unshowable_in(&open, &closed).is_empty());
        assert!(take_closures_in(&closed, "main").is_empty(), "the person's close reports nothing");
        let custody = rendered(&pool);
        opened(&open, offer(&custody), &routes::test_pool(), "report-view-2");
        assert_eq!(take_all_in(&open), vec!["report-view-2".to_string()]);
        assert!(custody.retire_active());
        assert!(close_unshowable_in(&open, &closed).is_empty());
        assert!(take_closures_in(&closed, "main").is_empty(), "the main window's close reports nothing");
        let custody = rendered(&pool);
        opened(&open, offer(&custody), &routes::test_pool(), "report-view-3");
        assert!(custody.retire_active());
        close_unshowable_in(&open, &closed);
        assert!(take_closures_in(&closed, "report-view-3").is_empty(), "no other webview takes or clears the notice");
        let notices = take_closures_in(&closed, "main");
        assert_eq!(notices, vec![ViewClosureNotice { kind: KIND_CURRENT_RENDER, reason: REASON_NO_LONGER_SHOWABLE }]);
        let json = serde_json::to_string(&notices).unwrap();
        assert_eq!(json, "[{\"kind\":\"current_render\",\"reason\":\"no_longer_showable\"}]");
        assert!(take_closures_in(&closed, "main").is_empty(), "taken once");
    }

    /// B4 (a): when the report page drops its preview it withdraws the current render's windows:
    /// they close whatever custody says (the render may still be current while a source change is
    /// in transit), with a notice; the render and its document are untouched; another kind stays.
    #[test]
    fn a_withdrawal_closes_the_current_renders_windows_and_leaves_the_render() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Mutex::new(Vec::new());
        let closed = Mutex::new(BTreeSet::new());
        opened(&open, offer(&custody), &routes::test_pool(), "report-view-1");
        let saved = HeldDocument::hold(&pool, "<p>saved</p>", &|| false).unwrap();
        let other = ViewDocument {
            identity: ViewIdentity { kind: "saved_report", sha256_hex: sha256_hex(b"<p>saved</p>") },
            document: ViewBytes::Held(saved),
            title: "Saved".to_string(),
            words: Vec::new(),
            details: Vec::new(),
            may_leave: false,
            showable: Arc::new(|| true),
        };
        assert!(reserve_in(&open, other, command_lease(&routes::test_pool()), label("report-view-2")).is_ok());
        assert_eq!(close_kind_in(&open, &closed, KIND_CURRENT_RENDER), vec![ViewClosure { label: "report-view-1".to_string(), identity: identity() }]);
        assert_eq!(lock(&open).iter().map(|record| record.label.clone()).collect::<Vec<_>>(), vec!["report-view-2".to_string()]);
        assert!(custody.admit_view(&sha256_hex(HTML.as_bytes()), FIRST).is_ok(), "the render stays current");
        assert_eq!(take_closures_in(&closed, "main"), vec![ViewClosureNotice { kind: KIND_CURRENT_RENDER, reason: REASON_NO_LONGER_SHOWABLE }]);
        assert!(close_kind_in(&open, &closed, KIND_CURRENT_RENDER).is_empty());
        assert!(take_closures_in(&closed, "main").is_empty(), "nothing closed, nothing reported");
    }

    // ---- B4 (b): only the panel's render can be named.

    /// The render custody is written only by the render command: `record_render` is called only
    /// from `render_calculation_report_value`, under a ticket only `render_calculation_report`
    /// holds, and the report package crate, where the shareable package renders, cannot name the
    /// shell's custody.
    #[test]
    fn only_the_render_command_records_a_render_that_a_view_can_name() {
        let lib = source_file("lib.rs");
        // The crate root's own test module starts at its `mod tests`; the render's two calls
        // stand before it, each once.
        let production = &lib[..lib.find("\n#[cfg(test)]\nmod tests {").expect("the crate root's tests")];
        assert_eq!(production.matches("rendered_report_routes::record_render(").count(), 1);
        assert_eq!(production.matches("rendered_report_routes::with_render_ticket(").count(), 1);
        let command = &production[production.find("async fn render_calculation_report(").unwrap()..];
        assert!(command[..command.find("\n}\n").unwrap()].contains("rendered_report_routes::with_render_ticket("));
        let value = &production[production.find("fn render_calculation_report_value(").unwrap()..];
        let value = &value[..value.find("\n}\n").unwrap()];
        assert!(value.find("refuse_user_release_render(&input)?;").unwrap() < value.find("record_render(").unwrap(), "a user-released input is refused before anything is recorded");
        let routes_text = source_file("rendered_report_routes.rs");
        assert_eq!(routes_text.matches("CUSTODY.complete_in(").count(), 1, "the one completion is record_render's");
        let package = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../core/reporting/report_package/src");
        for entry in std::fs::read_dir(package).unwrap() {
            let text = std::fs::read_to_string(entry.unwrap().path()).unwrap_or_default();
            assert!(!text.contains("rendered_report_routes") && !text.contains("record_render"));
        }
    }

    /// The render command refuses a user-released input before recording anything; its entry has
    /// retired the render before it, so no current render exists for a view to open.
    #[test]
    fn a_view_is_refused_after_a_refused_user_released_render() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let input = serde_json::json!({ "report_context": { "export_context": "user_released_report" } });
        // The render's entry, then its refusal (no ticket is held here, so nothing global is touched).
        drop(custody.begin_next(FIRST).unwrap());
        let refused = crate::render_calculation_report_value(input).unwrap_err();
        assert!(refused.starts_with(crate::RENDER_USER_RELEASE_PACKAGE_ONLY), "{refused}");
        assert_eq!(custody.admit_view(&sha256_hex(HTML.as_bytes()), FIRST).unwrap_err().code, NOT_CURRENT);
    }

    // ---- Platform.

    #[test]
    fn the_platform_gate_is_one_constant_and_refuses_by_name() {
        // The gate opens on macOS only, and nowhere else in this module names a platform.
        assert_eq!(view_windows_supported(), std::env::consts::OS == "macos");
        let refusal = platform_unsupported();
        assert_eq!((refusal.code.as_str(), refusal.stage.as_str()), (VIEW_PLATFORM_UNSUPPORTED, "view"));
        let text = source_file("report_view.rs");
        assert_eq!(production(&text).matches("std::env::consts::OS").count(), 1, "the gate is the one platform condition here");
        assert!(!production(&text).contains("target_os"));
    }
}
