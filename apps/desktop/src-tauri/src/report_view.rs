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
//!   the provider says the document is still showable and the window is not
//!   closing.
//! - Script: content script is disabled in its webview; the wrapper page and
//!   the document carry none; the wrapper's and the document's content
//!   policies and the frame's sandbox refuse it. The runtime's own bridge
//!   script is present in the webview's main frame: the runtime installs it in
//!   every webview's main frame (tauri 2.11.1 `src/manager/webview.rs`
//!   157-224), and WebKit runs injected scripts where content script is
//!   disabled. Its webview keeps no data, has no developer tools, loads
//!   nothing and navigates nowhere but its own two addresses.
//! - This module builds no command that hands the document on: no save,
//!   print or copy command of its own. A person may select and copy what is
//!   shown, as in the Report page's preview. It defines one command, the
//!   opening of the current render, which returns no part of a document.
//! - It never extends the life of the current render's document: it keeps
//!   only a weak reference to it ([`ViewBytes::Borrowed`]); the opening takes
//!   no share, and a serve takes one only for the length of its copy. A
//!   retirement releases the document at once, whatever a window is doing.
//! - Its memory is reserved: every copy of the document a serve makes in this
//!   process is reserved, in the interactive class, from the opening until
//!   the page has loaded or the window's destruction is observed, and never
//!   for less than the serve (below, "Memory").
//! - A window is never current for longer than the native render it shows:
//!   the sweep beside every retirement of the render asks for its destruction
//!   (below, "Closing").
//!
//! Interface. A provider hands [`open`] a [`ViewDocument`]: an identity
//! ([`ViewIdentity`], a kind and a SHA-256), the bytes ([`ViewBytes`]), their
//! length (`byte_count`, from the provider's own admission: the reservation is
//! computed from it, so the opening takes no share of the bytes), a title, at
//! most [`MAX_WORDS`] sentences and at most [`MAX_DETAILS`] label and value
//! pairs, all plain text this module escapes, and `showable`, the provider's
//! answer to whether the document may still be shown. One window per
//! identity: offering an identity open in a window that exists brings that
//! window forward, sets its title to the one just offered, and changes, serves
//! and reserves nothing else, so the words and details a provider gives must
//! be true for every offer of an identity; only the title follows the last
//! offer. Several identities may be open at once. The window's addresses are
//! derived from the identity only, and no address, error, title or log line
//! here carries a file path or a location.
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
//! Serving ([`respond_between`]). A request is answered only when it comes
//! from the webview of a window whose record is open (not closing), for
//! exactly that record's wrapper or document address as the request delivers
//! it (scheme, host, path and no query), compared as delivered with no
//! spelling normalised (a dot segment is not the address it might stand
//! for), while its `showable()` is true and,
//! for the document, while its bytes are there; otherwise 404 with no body.
//! The document address is served at most once per window: a reload, or a
//! second frame, gets 404; so does a document request after the page has
//! loaded without it.
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
//!   2.11.1 `src/lib.rs` 4898-4905 passes it the parsed URL (a target the
//!   runtime cannot parse as a URL is allowed there without the handler being
//!   asked, `unwrap_or(true)`; what this protocol then serves is bounded by
//!   its own whole-address check, not by the handler). So the allowed set is
//!   exactly the wrapper and the document address
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
//!   frame's navigation. It is not always delivered: tauri 2.11.1
//!   `src/webview/mod.rs` 765 drops a page-load event that arrives before the
//!   manager has attached the webview. A window whose page load is not
//!   delivered keeps its reservation until its destruction is observed.
//! - A fragment never reaches the protocol's handler: wry 0.55.1
//!   `src/wkwebview/class/url_scheme_handler.rs` 93-102 builds the request
//!   from the task's whole URL as an `http::Uri`, and http 1.4.0
//!   `src/uri/path.rs` 117-119 truncates the fragment while parsing it. The
//!   handler sees the address as that conversion delivers it
//!   ([`request_address`]); what keeps the window to its two addresses,
//!   fragment and all, is the navigation rule above.
//! - A window built from a thread other than the main one is queued, not
//!   created: tauri-runtime-wry 2.11.1 `src/lib.rs` 300-335 sends
//!   `Message::CreateWindow` and returns at once, and 4084-4091 only logs a
//!   creation that fails. A window getter sent after it travels the same queue
//!   (`send_user_message`, 235-255) and is handled after it; for a window not
//!   in the runtime's map, `Message::Window` (3372-3682) matches nothing and
//!   drops the getter's reply channel, so the getter (`window_getter!`,
//!   197-211) fails. So a window that answers [`Windows::acknowledge`] (its
//!   title, `WindowMessage::Title`, 3421) exists in the runtime.
//!
//! Memory (DEL-04-07 "Memory budget"). A serve of the document makes two
//! copies of it in this process, as the pinned sources show: the owned body
//! this module's handler builds (the protocol's response body is owned bytes),
//! and the runtime's own copy of that body, an `NSData` that wry 0.55.1
//! `src/wkwebview/class/url_scheme_handler.rs` 271-275 copies the body into
//! before handing it to WebKit (`didReceiveData`, 282). Between the handler
//! and that point the body is moved, never copied: tauri 2.11.1 `src/app.rs`
//! 2454-2456 and wry 0.55.1 `src/lib.rs` 453-456 turn the `Vec<u8>` into
//! `Cow::Owned`, and tauri-runtime-wry 2.11.1 `src/lib.rs` 5197-5205 passes
//! it on. Both are made and dropped inside the responder wry builds
//! (`url_scheme_handler.rs`, from 187), which runs within [`serve`]'s
//! `respond` call. What WebKit does with the `NSData` after `didReceiveData`
//! is WebKit's and is not in these sources; a copy the web content process
//! makes is in another process and is not this pool's. Both copies are
//! reserved: the open command's own interactive lease grows by two allocation
//! envelopes of the document ([`serve_reservation_bytes`]) when the window's
//! record is installed (refused [`VIEW_MEMORY_UNAVAILABLE`], opening nothing)
//! and moves into the record, which shares it with the window's one serve.
//! The reservation is released when both have let go of it: the record when
//! the page has loaded, when the window's destruction is observed, when the
//! opening fails, or when a sweep finds the window already gone, whichever is
//! first; the serve once `respond` has returned. A sweep does not release it
//! when it asks for the window's destruction: the record is marked closing
//! and kept until the destruction is observed. A window whose page load is
//! not delivered keeps the reservation until its destruction is observed; a
//! record belongs only to a window the runtime acknowledged, so that
//! destruction comes. A page that loads without its document marks the
//! document as no longer servable, so no serve can follow the reservation's
//! end. The wrapper page is generated per request and is not reserved: its
//! size is bounded by the provider's title, words and details. A `Borrowed`
//! window keeps a weak reference to the held document: when the document's
//! last holder drops it, its bytes and its custody reservation are released,
//! and only the small shared record's allocation stays until the window's
//! record goes.
//!
//! Threads. The serve runs off the main thread: [`serve`] hands the request
//! to a blocking task and responds from there, as wry 0.55.1 `src/lib.rs`
//! 446-449 allows (its responder is `Send` so that the response can be
//! computed on another thread) and as tauri 2.11.1's own IPC protocol does
//! (`src/ipc/protocol.rs` 139). So the provider's `showable` (which reads
//! render custody, and can wait there while a save publishes) and both copies
//! run off the main thread. The opening runs on the open command's own thread,
//! never the main one: it holds the opening gate from the record's
//! installation to the window's acknowledgement, and the acknowledgement waits
//! for the main thread to handle the window's messages. The main thread never
//! takes the gate: a window's destruction ([`window_destroyed`]), a page load,
//! the serve and the sweeps take only the records' lock, which is never held
//! across a build, an acknowledgement or a wait.
//!
//! Lock order. The opening gate, then the records' lock; under the records'
//! lock, the provider's `showable()` (the current render's reads render
//! custody: the records' lock, then custody, the established order of the
//! print window, and custody is never held while the records' lock is taken,
//! so [`close_unshowable`] is never called with the custody mutex held), the
//! window registry's lookup ([`Windows::find`], the runtime's window map) and
//! the memory pool's verdict on the lease's growth. Records and leases taken
//! out are dropped after the lock is released, and every window operation
//! (build, acknowledgement, focus, title, destroy) runs after it is released.
//!
//! Opening ([`open_in`]). Openings are serialised by one gate from the
//! record's installation to the window's acknowledgement, so two never
//! interleave. Under the gate and the records' lock, in one critical section:
//! after the main window's shutdown, nothing opens; an offer whose
//! `showable()` is false, or whose bytes are gone, opens nothing and brings
//! nothing forward; a record of the same identity (not closing) whose window
//! exists is brought forward, and `brought_forward` is answered only for a
//! window found there and reached; one whose window does not exist is dropped
//! (under the gate it cannot be one under construction); otherwise the record
//! is installed with its grown lease, marked as building. Then, the lock
//! released, the window is built: the runtime attaches the window and its
//! webview before the build returns, and a destruction asked meanwhile can
//! remove the webview the build is still reaching (tauri 2.11.1), so while the
//! record is building a sweep or the shutdown marks it closing and asks no
//! destruction. Once the build has returned the mark is cleared under the
//! lock; a build that failed, or a window that does not answer the
//! acknowledgement, releases the record and its lease before asking the
//! window's destruction ([`VIEW_WINDOW_FAILED`]). After the acknowledgement
//! and after a bring-forward, the record must still be open: no shutdown,
//! present, not closing and showable. Otherwise it is marked closing (or
//! released, if its window is gone), its window destroyed, and the opening
//! refused, as a failed window after the shutdown and as not showable
//! otherwise. A bring-forward restores a minimized window before focusing it, and counts
//! only when the window answered every step.
//!
//! Closing. A window's life is the native render's currency: the sweep
//! ([`close_unshowable`]), run beside every close of a retired print window
//! in the routes, asks the destruction of every window whose `showable()` is
//! false. It marks the record closing (not served, not matched: a later
//! opening of the same identity opens a new window with a new label), and the
//! record goes when the window's destruction is observed; a record whose
//! window is already gone at the sweep goes at once. Destruction is observed
//! in the application's run-event callback (`run` in `lib.rs`,
//! [`window_destroyed`]), which the runtime calls for every window's events
//! with no listener registered on the window (tauri-runtime-wry 2.11.1
//! `src/lib.rs` 4272-4292); a listener added to a window after its
//! construction is queued (1982-1989) and can miss a destruction that comes
//! first. The shutdown ([`shutdown_in`]) sets, in one critical section of the
//! records' lock, the flag after which nothing opens, and marks every record
//! closing as at a sweep; it runs when the main window's destruction is
//! observed and, before that, when a close or quit has been accepted
//! (`finish_close`) and when an exit is requested, before the embedded host
//! can hold the exit while it drains: each time after the render custody's
//! retirement ([`routes::retire_for_shutdown`]), never with its mutex held. The window is an ordinary window: not always
//! on top, no special level. Nothing is reported to the report page when a
//! window closes.
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
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError, Weak};
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

/// The label of the `n`th view window opened in this process: its window's
/// and its webview's alike (`WebviewWindowBuilder` gives both the window's
/// label). It is never the main window's, so the invoke handler's gate
/// (`command_webview::main_webview_only`) refuses every application command
/// called from a view window with `COMMAND-WEBVIEW-REFUSED`.
fn view_label(n: u64) -> String {
    format!("{LABEL_PREFIX}{n}")
}

/// Whether `label` is a view window's ([`view_label`]).
#[cfg_attr(not(feature = "native-smoke"), allow(dead_code))]
pub(crate) fn is_view_label(label: &str) -> bool {
    label.starts_with(LABEL_PREFIX)
}

/// The current render's kind.
pub(crate) const KIND_CURRENT_RENDER: &str = "current_render";

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
/// The second sentence on the strip of a local private render: the product's
/// own warning, word for word (`LOCAL_PRIVATE_REPORT_WARNING` in
/// `RenderedReportPanel.tsx`).
pub(crate) const LOCAL_PRIVATE_WORDS: &str = "Local private report: this rendering contains private project values for your own local review. It is not for sharing or publication.";
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

    /// Whether the bytes are gone (a borrowed document's last holder dropped
    /// it), read without taking a share.
    fn is_gone(&self) -> bool {
        match self {
            Self::Held(_) => false,
            Self::Borrowed(document) => document.strong_count() == 0,
        }
    }
}

/// The provider's answer to whether its document may still be shown. It is
/// called with this module's records' lock held (module docs, "Lock order"),
/// off the main thread, and must not call back into this module.
pub(crate) type Showable = Arc<dyn Fn() -> bool + Send + Sync>;

/// What a provider offers [`open`] (module docs, "Interface").
pub(crate) struct ViewDocument {
    pub(crate) identity: ViewIdentity,
    pub(crate) document: ViewBytes,
    /// The document's length in bytes, from the provider's admission: the
    /// reservation is computed from it, and the opening takes no share.
    pub(crate) byte_count: usize,
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
    /// The document's one serve has been made, or can no longer be made (the
    /// page loaded without it).
    document_spent: bool,
    /// Its destruction has been asked for: it is served nothing and matched by
    /// no offer, and goes when the destruction is observed.
    closing: bool,
    /// Its window is being built: a sweep marks it closing but asks no
    /// destruction, which the opening asks once the build has returned (module
    /// docs, "Opening").
    building: bool,
    /// The reservation of the serve's copies, until the page has loaded or the
    /// window's destruction is observed; the serve holds a share of it while
    /// it runs (module docs, "Memory").
    lease: Option<Arc<Lease>>,
}

/// The records of the open view windows.
static OPEN: Mutex<Vec<OpenView>> = Mutex::new(Vec::new());
/// The opening gate (module docs, "Opening"); never taken on the main thread.
static OPENING: Mutex<()> = Mutex::new(());
/// The main window's shutdown has begun: set and read under the records' lock.
static SHUT: AtomicBool = AtomicBool::new(false);
static APP: Mutex<Option<AppHandle>> = Mutex::new(None);
static NEXT_LABEL: AtomicU64 = AtomicU64::new(1);

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

/// The bytes the open command's lease grows by: the allocation envelopes of
/// the two copies a serve of a document of `len` bytes makes in this process,
/// the owned body and the runtime's own copy of it (module docs, "Memory";
/// `ingress::allocation_envelope`: each is one exact allocation of `len`).
pub(crate) fn serve_reservation_bytes(len: usize) -> u64 {
    crate::ingress::allocation_envelope(len as u64).and_then(|one| one.checked_mul(2)).unwrap_or(u64::MAX)
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

/// What a person reads when the report is no longer the one to show at the opening.
const NOT_SHOWABLE_WORDS: &str = "This report is no longer the current one. No window was opened.";

fn not_showable() -> ViewError {
    refused(VIEW_NOT_SHOWABLE, NOT_SHOWABLE_WORDS)
}

fn window_failed() -> ViewError {
    refused(VIEW_WINDOW_FAILED, "The report's window could not be opened.")
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
    // A held document is here already: its length is read, no share taken.
    if let ViewBytes::Held(held) = &document.document {
        if held.html().len() != document.byte_count {
            return Err(refused(VIEW_DOCUMENT_REFUSED, "The report's length does not match its document; no window was opened."));
        }
    }
    Ok(())
}

/// What an opening and a sweep ask of the window system: the runtime in
/// production ([`RuntimeWindows`]), a model of it in tests.
pub(crate) trait Windows {
    type Window;
    /// The window named `label`, if the window registry holds it. Called under
    /// the records' lock: it must not take this module's locks or wait for
    /// the main thread.
    fn find(&self, label: &str) -> Option<Self::Window>;
    /// Builds the window (in production: queues its creation); `None` when
    /// the builder refuses.
    fn build(&self, label: &str, identity: &ViewIdentity, title: &str) -> Option<Self::Window>;
    /// Whether the window answers something only a window the runtime holds
    /// answers (module docs, the acknowledgement's fact).
    fn acknowledge(&self, window: &Self::Window) -> bool;
    /// Gives the window `title` and brings it forward; whether it answered.
    fn bring_forward(&self, window: &Self::Window, title: &str) -> bool;
    /// Asks for the destruction of the window named `label`, if there is one.
    fn destroy(&self, label: &str);
}

/// How an opening ended.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Opened {
    /// A new window, acknowledged, under this label.
    New { label: String, title: String },
    /// The window of this label, found and reached, now carrying `title`.
    BroughtForward { label: String, title: String },
}

/// The opening (module docs, "Opening"): under `gate`, installs the offer's
/// record or finds the window to bring forward in one critical section of the
/// records' lock, then builds and acknowledges a new window with the lock
/// released, and checks that its record is still open.
pub(crate) fn open_in<W: Windows>(
    open: &Mutex<Vec<OpenView>>,
    gate: &Mutex<()>,
    shut: &AtomicBool,
    windows: &W,
    document: ViewDocument,
    lease: Lease,
    label: impl FnOnce() -> String,
) -> Result<Opened, ViewError> {
    validate(&document)?;
    let _gate = lock(gate);
    let identity = document.identity.clone();
    let (label, title) = match install_in(open, shut, windows, document, lease, label)? {
        Installed::Forward { label, title, window } => {
            if !windows.bring_forward(&window, &title) {
                return Err(window_failed());
            }
            // A sweep or the shutdown may have come while it was brought forward.
            still_open_in(open, shut, windows, &label)?;
            return Ok(Opened::BroughtForward { label, title });
        }
        Installed::New { label, title } => (label, title),
    };
    let built = windows.build(&label, &identity, &title);
    // The build has returned: from here a sweep may ask the window's destruction.
    let mut records = lock(open);
    let at = records.iter().position(|record| record.label == label);
    if let Some(at) = at {
        records[at].building = false;
    }
    let Some(window) = built else {
        let record = at.map(|at| records.swap_remove(at));
        drop(records);
        drop(record);
        windows.destroy(&label);
        return Err(window_failed());
    };
    drop(records);
    if !windows.acknowledge(&window) {
        drop(release_label_in(open, &label));
        windows.destroy(&label);
        return Err(window_failed());
    }
    // A sweep or the shutdown may have come during the build or the acknowledgement.
    still_open_in(open, shut, windows, &label)?;
    Ok(Opened::New { label, title })
}

/// After a step taken with the records' lock released (an acknowledgement, a
/// bring-forward): under the lock, the record of `label`
/// must still be open, with no shutdown, present, not closing and showable.
/// Otherwise it is marked closing (or released, if its window is gone), its
/// window's destruction is asked after the lock, and the opening is refused:
/// a failed window after the shutdown, not showable otherwise.
fn still_open_in<W: Windows>(open: &Mutex<Vec<OpenView>>, shut: &AtomicBool, windows: &W, label: &str) -> Result<(), ViewError> {
    let mut records = lock(open);
    let at = records.iter().position(|record| record.label == label);
    let shutdown = shut.load(Ordering::SeqCst);
    if !shutdown && at.is_some_and(|at| !records[at].closing && (records[at].showable)()) {
        return Ok(());
    }
    let gone = match at {
        Some(at) if windows.find(label).is_none() => Some(records.swap_remove(at)),
        Some(at) => {
            records[at].closing = true;
            None
        }
        None => None,
    };
    drop(records);
    drop(gone);
    windows.destroy(label);
    Err(if shutdown { window_failed() } else { not_showable() })
}

enum Installed<W> {
    New { label: String, title: String },
    Forward { label: String, title: String, window: W },
}

/// The opening's one critical section under the records' lock (module docs,
/// "Opening"). A stale record and an unneeded lease are dropped after the lock.
fn install_in<W: Windows>(
    open: &Mutex<Vec<OpenView>>,
    shut: &AtomicBool,
    windows: &W,
    document: ViewDocument,
    mut lease: Lease,
    label: impl FnOnce() -> String,
) -> Result<Installed<W::Window>, ViewError> {
    let ViewDocument { identity, document, byte_count, title, words, details, may_leave, showable } = document;
    let mut records = lock(open);
    if shut.load(Ordering::SeqCst) {
        drop(records);
        return Err(window_failed());
    }
    if !showable() || document.is_gone() {
        drop(records);
        return Err(not_showable());
    }
    let mut stale = None;
    if let Some(at) = records.iter().position(|record| !record.closing && record.identity == identity) {
        match windows.find(&records[at].label) {
            Some(window) => {
                records[at].title = title.clone();
                let label = records[at].label.clone();
                drop(records);
                return Ok(Installed::Forward { label, title, window });
            }
            // Under the gate no other opening is building it: it is stale.
            None => stale = Some(records.swap_remove(at)),
        }
    }
    let installed = match lease.grow_to(lease.bytes().saturating_add(serve_reservation_bytes(byte_count))) {
        Ok(()) => {
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
                document_spent: false,
                closing: false,
                building: true,
                lease: Some(Arc::new(lease)),
            });
            Ok(Installed::New { label, title })
        }
        Err(refusal) => Err(memory_unavailable(&refusal)),
    };
    drop(records);
    // A second barrier: under the gate and the acknowledgement a stale record's
    // window cannot have come into being since the lookup; if it has, it closes.
    if let Some(record) = stale {
        let stale_label = record.label.clone();
        drop(record);
        windows.destroy(&stale_label);
    }
    installed
}

/// The window named `label` has loaded its page: the record's share of the
/// reservation is released (dropped after the lock), and a document not yet
/// served can no longer be. Returns whether its record is still there; a
/// window with none is to be closed.
pub(crate) fn page_loaded_in(open: &Mutex<Vec<OpenView>>, label: &str) -> bool {
    let mut records = lock(open);
    let Some(record) = records.iter_mut().find(|record| record.label == label) else { return false };
    let lease = record.lease.take();
    record.document_spent = true;
    drop(records);
    drop(lease);
    true
}

/// Removes the record of the window named `label`, returning it for the
/// caller to drop after the lock is released.
pub(crate) fn release_label_in(open: &Mutex<Vec<OpenView>>, label: &str) -> Option<OpenView> {
    let mut records = lock(open);
    let at = records.iter().position(|record| record.label == label)?;
    Some(records.swap_remove(at))
}

/// The sweep (module docs, "Closing"): every open record whose `showable()`
/// answers false is marked closing and its window asked to be destroyed, or,
/// if its window is already gone, removed at once. Returns the labels whose
/// destruction it asked for.
pub(crate) fn close_unshowable_in<W: Windows>(open: &Mutex<Vec<OpenView>>, windows: &W) -> Vec<String> {
    close_where_in(open, None, windows, |record| !(record.showable)())
}

/// The shutdown (module docs, "Closing"): in one critical section of the
/// records' lock, the flag after which nothing opens is set and every record
/// is marked closing, as at a sweep. Returns the labels whose destruction it
/// asked for.
pub(crate) fn shutdown_in<W: Windows>(open: &Mutex<Vec<OpenView>>, shut: &AtomicBool, windows: &W) -> Vec<String> {
    close_where_in(open, Some(shut), windows, |_| true)
}

/// Marks the selected records closing under one critical section (after
/// setting `shut`, when given): a record whose window is being built is only
/// marked (the opening asks its destruction once the build has returned), one
/// whose window exists is marked and its destruction asked after the lock,
/// and one whose window is gone is removed at once.
fn close_where_in<W: Windows>(open: &Mutex<Vec<OpenView>>, shut: Option<&AtomicBool>, windows: &W, select: impl Fn(&OpenView) -> bool) -> Vec<String> {
    let mut gone = Vec::new();
    let mut asked = Vec::new();
    let mut records = lock(open);
    if let Some(shut) = shut {
        shut.store(true, Ordering::SeqCst);
    }
    let mut at = 0;
    while at < records.len() {
        if records[at].closing || !select(&records[at]) {
            at += 1;
        } else if records[at].building {
            records[at].closing = true;
            at += 1;
        } else if windows.find(&records[at].label).is_some() {
            records[at].closing = true;
            asked.push(records[at].label.clone());
            at += 1;
        } else {
            gone.push(records.swap_remove(at));
        }
    }
    drop(records);
    drop(gone);
    for label in &asked {
        windows.destroy(label);
    }
    asked
}

/// A window's destruction, observed (module docs, "Closing"): a view window's
/// record goes, with its lease; the main window's begins shutdown, after which
/// nothing opens, and asks every view window to close. Returns the labels
/// whose destruction it asked for.
pub(crate) fn window_destroyed_in<W: Windows>(open: &Mutex<Vec<OpenView>>, shut: &AtomicBool, windows: &W, label: &str) -> Vec<String> {
    if label == "main" {
        return shutdown_in(open, shut, windows);
    }
    if label.starts_with(LABEL_PREFIX) {
        drop(release_label_in(open, label));
    }
    Vec::new()
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
    /// The document's identity, a share of its bytes and a share of the
    /// window's reservation.
    Document(ViewIdentity, Arc<HeldDocument>, Option<Arc<Lease>>),
    Nothing,
}

/// Under the records' lock: the part `webview_label` may be served at
/// `address` (module docs, "Serving"). For the document, its share and a
/// share of the reservation are taken here and the record marked as spent.
fn select_in(open: &Mutex<Vec<OpenView>>, webview_label: &str, address: &str) -> Selected {
    let mut records = lock(open);
    let Some(record) = records.iter_mut().find(|record| record.label == webview_label) else { return Selected::Nothing };
    if record.closing || !(record.showable)() {
        return Selected::Nothing;
    }
    // The delivered address as it is: no spelling of it is normalised to an
    // address it might stand for.
    if record.identity.wrapper_address().as_ref().map(|address| address.as_str()) == Some(address) {
        return Selected::Wrapper(WrapperParts { identity: record.identity.clone(), title: record.title.clone(), words: record.words.clone(), details: record.details.clone() });
    }
    if record.identity.document_address().as_ref().map(|address| address.as_str()) == Some(address) && !record.document_spent {
        if let Some(document) = record.document.share() {
            record.document_spent = true;
            return Selected::Document(record.identity.clone(), document, record.lease.clone());
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

/// The protocol's answer to one request: the response, which part it served,
/// and, for the document, the serve's share of the window's reservation, to be
/// held until the response has been handed to the runtime and its copies are
/// gone (module docs, "Memory").
pub(crate) struct Answer {
    pub(crate) response: tauri::http::Response<Vec<u8>>,
    pub(crate) served: Option<(ViewIdentity, Part)>,
    pub(crate) reservation: Option<Arc<Lease>>,
}

/// The protocol's answer to one request from `webview_label` for `address`
/// (module docs, "Serving"). `between` runs after the document's share is
/// taken and before its copy (a seam for tests).
pub(crate) fn respond_between(open: &Mutex<Vec<OpenView>>, webview_label: &str, address: &str, between: impl FnOnce()) -> Answer {
    match select_in(open, webview_label, address) {
        Selected::Wrapper(parts) => {
            let csp = wrapper_csp(&parts.identity);
            let body = wrapper_html(&parts).into_bytes();
            Answer { response: response(200, &csp, body), served: Some((parts.identity, Part::Wrapper)), reservation: None }
        }
        Selected::Document(identity, document, reservation) => {
            between();
            // The owned body, exactly the held bytes; the share ends with it.
            let body = document.html().as_bytes().to_vec();
            drop(document);
            Answer { response: response(200, DOCUMENT_CSP, body), served: Some((identity, Part::Document)), reservation }
        }
        Selected::Nothing => Answer { response: response(404, DOCUMENT_CSP, Vec::new()), served: None, reservation: None },
    }
}

/// Tests' [`respond_between`] with no seam, the reservation's share dropped
/// with the answer, as [`serve`] drops it once it has responded.
#[cfg(test)]
pub(crate) fn respond_in(open: &Mutex<Vec<OpenView>>, webview_label: &str, address: &str) -> tauri::http::Response<Vec<u8>> {
    respond_between(open, webview_label, address, || {}).response
}

/// The address a request names, as the http conversion delivers it to the
/// handler (module docs: a fragment never reaches it).
pub(crate) fn request_address(request: &tauri::http::Request<Vec<u8>>) -> String {
    request.uri().to_string()
}

/// The custom protocol's handler (registered in `run`, with its responder):
/// the selection, both copies and the response run on a blocking task off the
/// main thread, and the serve's share of the reservation is dropped once
/// `respond` has returned (module docs, "Memory" and "Threads").
pub(crate) fn serve<R: Runtime>(context: tauri::UriSchemeContext<'_, R>, request: tauri::http::Request<Vec<u8>>, responder: tauri::UriSchemeResponder) {
    let label = context.webview_label().to_string();
    let address = request_address(&request);
    drop(request);
    tauri::async_runtime::spawn_blocking(move || {
        let Answer { response, served, reservation } = respond_between(&OPEN, &label, &address, || {});
        #[cfg(feature = "native-smoke")]
        if let Some((identity, part)) = &served {
            crate::native_smoke::record_report_view(&label, identity.kind, &identity.sha256_hex, *part == Part::Document, response.body());
        }
        let _ = served;
        responder.respond(response);
        // The copies this process's code makes of the body are made and dropped
        // within `respond` (module docs, "Memory"); the share is held until then.
        drop(reservation);
    });
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

/// The runtime's windows ([`Windows`]).
pub(crate) struct RuntimeWindows(Option<AppHandle>);

impl RuntimeWindows {
    fn current() -> Self {
        Self(lock(&APP).clone())
    }
}

impl Windows for RuntimeWindows {
    type Window = tauri::WebviewWindow;

    fn find(&self, label: &str) -> Option<Self::Window> {
        self.0.as_ref().and_then(|app| app.get_webview_window(label))
    }

    fn build(&self, label: &str, identity: &ViewIdentity, title: &str) -> Option<Self::Window> {
        use tauri::{webview::PageLoadEvent, WebviewWindowBuilder};
        let app = self.0.as_ref()?;
        let config = view_window_config(label, identity.wrapper_address()?, title);
        let allowed = identity.clone();
        WebviewWindowBuilder::from_config(app, &config)
            .ok()?
            .on_navigation(move |target| navigation_allowed(&allowed, target))
            .on_page_load(|window, payload| {
                if payload.event() == PageLoadEvent::Finished && !page_loaded_in(&OPEN, window.label()) {
                    let _ = window.destroy();
                }
            })
            .build()
            .ok()
    }

    fn acknowledge(&self, window: &Self::Window) -> bool {
        // A getter, answered only for a window in the runtime's map.
        window.title().is_ok()
    }

    fn bring_forward(&self, window: &Self::Window, title: &str) -> bool {
        // The getter first (only a window in the runtime's map answers it), then each
        // step's own answer; a minimized window is restored before it is focused.
        window.title().is_ok() && window.unminimize().is_ok() && window.set_title(title).is_ok() && window.set_focus().is_ok()
    }

    fn destroy(&self, label: &str) {
        if let Some(window) = self.find(label) {
            if let Err(error) = window.destroy() {
                eprintln!("report-view window close failed: {error}");
            }
        }
    }
}

/// Opens a window for `document` (module docs, "Opening") on the open
/// command's own thread. `lease` is the open command's own interactive lease:
/// grown and kept by the record for a new window, dropped otherwise.
pub(crate) fn open(app: &AppHandle, document: ViewDocument, lease: Lease) -> Result<ViewReceipt, ViewError> {
    if !view_windows_supported() {
        return Err(platform_unsupported());
    }
    *lock(&APP) = Some(app.clone());
    let identity = document.identity.clone();
    let receipt = |outcome: &'static str| ViewReceipt { outcome, kind: identity.kind, sha256_hex: identity.sha256_hex.clone() };
    let windows = RuntimeWindows(Some(app.clone()));
    match open_in(&OPEN, &OPENING, &SHUT, &windows, document, lease, || view_label(NEXT_LABEL.fetch_add(1, Ordering::Relaxed)))? {
        Opened::New { .. } => Ok(receipt("opened")),
        Opened::BroughtForward { .. } => Ok(receipt("brought_forward")),
    }
}

/// A window's destruction, from the application's run-event callback (`run`
/// in `lib.rs`; module docs, "Closing").
pub(crate) fn window_destroyed(label: &str) {
    let _ = window_destroyed_in(&OPEN, &SHUT, &RuntimeWindows::current(), label);
}

/// The sweep (module docs, "Closing"): asks the destruction of every view
/// window whose provider no longer finds its document showable. Never called
/// with the custody mutex held. Returns the labels it asked to destroy.
pub(crate) fn close_unshowable() -> Vec<String> {
    close_unshowable_in(&OPEN, &RuntimeWindows::current())
}

/// Asks every view window to close (the scripted run's close) and returns
/// their labels; each record goes when its destruction is observed.
#[cfg_attr(not(feature = "native-smoke"), allow(dead_code))]
pub(crate) fn close_all() -> Vec<String> {
    close_where_in(&OPEN, None, &RuntimeWindows::current(), |_| true)
}

/// The label of every view window record, in whatever state: being built,
/// open or closing. A record goes once its window's destruction is observed
/// or its opening fails (the scripted run's close waits for none to remain).
#[cfg_attr(not(feature = "native-smoke"), allow(dead_code))]
pub(crate) fn recorded_labels() -> Vec<String> {
    recorded_labels_in(&OPEN)
}

/// [`recorded_labels`] over `open`.
#[cfg_attr(not(feature = "native-smoke"), allow(dead_code))]
fn recorded_labels_in(open: &Mutex<Vec<OpenView>>) -> Vec<String> {
    lock(open).iter().map(|record| record.label.clone()).collect()
}

/// An accepted close or quit (`finish_close` and the exit request in
/// `lib.rs`, after the render custody's retirement and never with its mutex
/// held): nothing opens from here, and every view window is asked to close.
/// Returns the labels whose destruction it asked for.
pub(crate) fn shutdown() -> Vec<String> {
    shutdown_in(&OPEN, &SHUT, &RuntimeWindows::current())
}

/// The current render's offer (DEL-08-01 REQ-022): named by its canonical
/// hash, borrowed, its length from admission, showable while
/// `is_current(generation)`. What it says of the render's context comes from
/// the admitted render's own: a local private render's strip carries the
/// product's warning; a render of any context other than a public or a local
/// private report is refused, since the window could not say what it shows.
pub(crate) fn current_render_document(admitted: ViewAdmitted, is_current: impl Fn(u64) -> bool + Send + Sync + 'static) -> Result<ViewDocument, ViewError> {
    let mut words = vec![CURRENT_RENDER_WORDS.to_string()];
    match admitted.export_context.as_deref() {
        Some(routes::PUBLIC_REPORT) => {}
        Some(routes::LOCAL_PRIVATE) => words.push(LOCAL_PRIVATE_WORDS.to_string()),
        _ => return Err(refused(VIEW_DOCUMENT_REFUSED, "The report's export context is not one a view window can state; no window was opened.")),
    }
    let generation = admitted.generation;
    Ok(ViewDocument {
        identity: ViewIdentity { kind: KIND_CURRENT_RENDER, sha256_hex: admitted.sha256_hex.clone() },
        document: ViewBytes::Borrowed(admitted.document),
        byte_count: admitted.byte_count,
        title: current_render_title(None),
        words,
        details: vec![(CURRENT_RENDER_HASH_LABEL.to_string(), admitted.sha256_hex)],
        may_leave: false,
        showable: Arc::new(move || is_current(generation)),
    })
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
        .and_then(|admitted| current_render_document(admitted, |generation| routes::CUSTODY.is_current(generation)))
        .and_then(|document| open(&app, document, lease));
    respond(outcome.map_err(CommandError::Command))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_pool::{test_host::ScriptedHost, LeasePurpose, MemoryPool, PressureLevel};
    use crate::rendered_report_routes::{held_document_bytes, sha256_hex, RenderCustody, BLOCKED, LOCAL_PRIVATE, NOT_CURRENT, PUBLIC_REPORT, SOURCE_NOT_REGISTERED, SOURCE_RETIRED, UNBOUND};
    use std::collections::BTreeSet;
    use std::sync::atomic::AtomicUsize;
    use std::sync::mpsc::channel;

    const HTML: &str = "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<title>Invented</title>\n</head>\n<body>\n<p>Invented report</p>\n</body>\n</html>\n";
    const FIRST: SourceMark = SourceMark { incarnation: 1, sequence: 0 };

    fn label(name: &'static str) -> impl FnOnce() -> String {
        move || name.to_string()
    }

    /// A custody holding a render of `HTML` in `context` whose document is reserved from `pool`.
    fn rendered_in(pool: &Arc<MemoryPool>, context: &str) -> Arc<RenderCustody> {
        let custody = Arc::new(RenderCustody::new());
        custody.record_in(pool, Some(context), HTML, &sha256_hex(HTML.as_bytes()), false, false);
        custody
    }

    /// A custody holding a public render of `HTML`.
    fn rendered(pool: &Arc<MemoryPool>) -> Arc<RenderCustody> {
        rendered_in(pool, PUBLIC_REPORT)
    }

    /// The current render's offer against `custody`, as the command builds it.
    fn offer(custody: &Arc<RenderCustody>) -> ViewDocument {
        let admitted = custody.admit_view(&sha256_hex(HTML.as_bytes()), custody.source()).expect("the current render is admitted");
        let watched = Arc::clone(custody);
        current_render_document(admitted, move |generation| watched.is_current(generation)).expect("a public or local private render is offered")
    }

    /// The open command's lease from an ample pool of its own.
    fn command_lease(pool: &Arc<MemoryPool>) -> Lease {
        pool.acquire_interactive(1).expect("an ample pool admits the command's lease")
    }

    /// Windows that all exist and answer.
    struct Present;

    impl Windows for Present {
        type Window = String;
        fn find(&self, label: &str) -> Option<String> {
            Some(label.to_string())
        }
        fn build(&self, label: &str, _: &ViewIdentity, _: &str) -> Option<String> {
            Some(label.to_string())
        }
        fn acknowledge(&self, _: &String) -> bool {
            true
        }
        fn bring_forward(&self, _: &String, _: &str) -> bool {
            true
        }
        fn destroy(&self, _: &str) {}
    }

    /// A model of the runtime: the window registry (`find`), the runtime's own window map (what
    /// answers the acknowledgement and a bring-forward), the destructions asked for (and those
    /// asked while a build was running), and a hook called at the build (after the window is
    /// attached, as the runtime attaches it before its construction finishes), at the
    /// acknowledgement and at a bring-forward.
    #[derive(Default)]
    struct Model {
        registry: Mutex<BTreeSet<String>>,
        runtime: Mutex<BTreeSet<String>>,
        destroy_asked: Mutex<Vec<String>>,
        building: AtomicBool,
        destroyed_while_building: Mutex<Vec<String>>,
        /// The runtime never creates what is built (its creation is queued and fails).
        creation_fails: AtomicBool,
        hook: Mutex<Option<Arc<dyn Fn(&str) + Send + Sync>>>,
    }

    impl Model {
        fn at(&self, point: &str) {
            let hook = lock(&self.hook).clone();
            if let Some(hook) = hook {
                hook(point);
            }
        }
        /// The window `label` is destroyed, and its destruction observed.
        fn destroyed(&self, open: &Mutex<Vec<OpenView>>, label: &str) {
            lock(&self.registry).remove(label);
            lock(&self.runtime).remove(label);
            assert!(window_destroyed_in(open, &AtomicBool::new(false), self, label).is_empty());
        }
    }

    impl Windows for Model {
        type Window = String;
        fn find(&self, label: &str) -> Option<String> {
            lock(&self.registry).contains(label).then(|| label.to_string())
        }
        fn build(&self, label: &str, _: &ViewIdentity, _: &str) -> Option<String> {
            self.building.store(true, Ordering::SeqCst);
            lock(&self.registry).insert(label.to_string());
            if !self.creation_fails.load(Ordering::SeqCst) {
                lock(&self.runtime).insert(label.to_string());
            }
            self.at("build");
            self.building.store(false, Ordering::SeqCst);
            Some(label.to_string())
        }
        fn acknowledge(&self, window: &String) -> bool {
            self.at("acknowledge");
            lock(&self.runtime).contains(window)
        }
        fn bring_forward(&self, window: &String, _: &str) -> bool {
            self.at("bring_forward");
            lock(&self.runtime).contains(window)
        }
        fn destroy(&self, label: &str) {
            self.at("destroy");
            if self.building.load(Ordering::SeqCst) {
                lock(&self.destroyed_while_building).push(label.to_string());
            }
            lock(&self.destroy_asked).push(label.to_string());
        }
    }

    /// An opening through [`open_in`] among windows that all exist, with a gate of its own.
    fn reserve_in(open: &Mutex<Vec<OpenView>>, document: ViewDocument, lease: Lease, label: impl FnOnce() -> String) -> Result<Opened, ViewError> {
        open_in(open, &Mutex::new(()), &AtomicBool::new(false), &Present, document, lease, label)
    }

    /// An opening through [`open_in`] against `model`.
    fn open_with(open: &Mutex<Vec<OpenView>>, gate: &Mutex<()>, model: &Model, document: ViewDocument, lease: Lease, name: &'static str) -> Result<Opened, ViewError> {
        open_in(open, gate, &AtomicBool::new(false), model, document, lease, label(name))
    }

    fn opened(open: &Mutex<Vec<OpenView>>, document: ViewDocument, pool: &Arc<MemoryPool>, name: &'static str) {
        assert_eq!(
            reserve_in(open, document, command_lease(pool), label(name)).unwrap(),
            Opened::New { label: name.to_string(), title: current_render_title(None) }
        );
    }

    /// [`respond_in`] for `path` at this protocol's own scheme and host.
    fn respond_at(open: &Mutex<Vec<OpenView>>, webview_label: &str, path: &str) -> tauri::http::Response<Vec<u8>> {
        respond_in(open, webview_label, &format!("{SCHEME}://{HOST}{path}"))
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

    /// This module builds no command that hands the document on (no save, print or copy command
    /// of its own; a person may select and copy what is shown): it defines exactly one command,
    /// the opening of the current render, which returns no document.
    #[test]
    fn the_module_builds_one_command_and_none_that_hands_the_document_on() {
        let text = source_file("report_view.rs");
        let production = production(&text);
        assert_eq!(production.matches("#[tauri::command]").count(), 1);
        assert!(production.contains("#[tauri::command]\npub(crate) async fn open_rendered_report_view("));
        for forbidden in ["DialogExt", "blocking_save_file", ".print()", "clipboard", "std::fs", "File::create", "atomic_report_package_save"] {
            assert!(!production.contains(forbidden), "{forbidden}");
        }
    }

    #[test]
    fn the_window_is_served_only_its_wrapper_and_document_only_to_its_own_webview() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Mutex::new(Vec::new());
        opened(&open, offer(&custody), &routes::test_pool(), "report-view-1");
        let id = identity();
        assert_refused(respond_at(&open, "main", &id.wrapper_path()));
        assert_refused(respond_at(&open, "report-view-2", &id.document_path()));
        assert_refused(respond_at(&open, "report-view-1", "/"));
        assert_refused(respond_at(&open, "report-view-1", &format!("{}/", id.wrapper_path())));
        assert_refused(respond_at(&open, "report-view-1", &format!("/{}/{}", "saved_report", id.sha256_hex)));
        let wrapper = respond_at(&open, "report-view-1", &id.wrapper_path());
        assert_eq!(wrapper.status(), 200);
        assert_eq!(header(&wrapper, "Content-Security-Policy"), Some(wrapper_csp(&id)));
        assert_eq!(header(&wrapper, "Cache-Control").as_deref(), Some("no-store"));
        let document = respond_at(&open, "report-view-1", &id.document_path());
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
        assert_eq!(respond_at(&open, "report-view-1", &id.document_path()).status(), 200);
        assert_refused(respond_at(&open, "report-view-1", &id.document_path()));
        assert_eq!(respond_at(&open, "report-view-1", &id.wrapper_path()).status(), 200, "the wrapper carries no document");
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
        assert_refused(respond_at(&open, "report-view-1", &id.wrapper_path()));
        assert_refused(respond_at(&open, "report-view-1", &id.document_path()));
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
            byte_count: HTML.len(),
            title: "Invented".to_string(),
            words: Vec::new(),
            details: Vec::new(),
            may_leave: false,
            showable: Arc::new(|| true),
        };
        assert!(matches!(reserve_in(&open, document, command_lease(&routes::test_pool()), label("report-view-1")).unwrap(), Opened::New { .. }));
        drop(held);
        assert_eq!(pool.reserved_total(), 0);
        assert_refused(respond_at(&open, "report-view-1", &identity().document_path()));
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
        let wrapper = String::from_utf8(respond_at(&open, "report-view-1", &id.wrapper_path()).into_body()).unwrap();
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


    /// A view window's label, whichever number it takes, is not the main one, so the invoke
    /// handler's gate (`command_webview::only_from_main`, which `main_webview_only` is) refuses
    /// a command called from it with `COMMAND-WEBVIEW-REFUSED` before the command is entered.
    #[test]
    fn a_command_from_a_view_window_is_refused_by_the_main_webview_gate() {
        use crate::command_webview::{from_main, only_from_main, Caller, COMMAND_WEBVIEW_REFUSED};
        struct FromView {
            label: String,
            refused: Arc<Mutex<Vec<&'static str>>>,
        }
        impl Caller for FromView {
            fn from_main(&self) -> bool {
                from_main(&self.label, &self.label)
            }
            fn refuse(self) {
                self.refused.lock().unwrap().push(COMMAND_WEBVIEW_REFUSED);
            }
        }
        let entered = Arc::new(AtomicUsize::new(0));
        let counted = Arc::clone(&entered);
        let gated = only_from_main(move |_: FromView| {
            counted.fetch_add(1, Ordering::SeqCst);
            true
        });
        for n in [0, 1, 2, u64::MAX] {
            let label = view_label(n);
            let config = view_window_config(&label, identity().wrapper_address().unwrap(), "Calculation report");
            assert_eq!(config.label, label, "the window is built under this label");
            assert_ne!(label, crate::command_webview::COMMAND_WEBVIEW);
            let refused = Arc::new(Mutex::new(Vec::new()));
            let handled = gated(FromView { label: label.clone(), refused: Arc::clone(&refused) });
            assert!(handled, "{label}: reported handled");
            assert_eq!(*refused.lock().unwrap(), [COMMAND_WEBVIEW_REFUSED], "{label}: refused once");
        }
        assert_eq!(entered.load(Ordering::SeqCst), 0, "no command was entered from a view window");
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


    /// A document that is no longer showable at the opening is refused in a person's words, and
    /// nothing is reserved or recorded.
    #[test]
    fn a_document_no_longer_showable_is_refused_in_the_persons_words() {
        let custody = rendered(&routes::test_pool());
        let mut document = offer(&custody);
        document.showable = Arc::new(|| false);
        let lease_pool = routes::test_pool();
        let open = Mutex::new(Vec::new());
        let refusal = reserve_in(&open, document, command_lease(&lease_pool), || panic!("no label for a refused offer")).unwrap_err();
        assert_eq!(refusal.code, VIEW_NOT_SHOWABLE);
        assert_eq!(refusal.message, "This report is no longer the current one. No window was opened.");
        assert!(lock(&open).is_empty());
        assert_eq!(lease_pool.reserved_total(), 0);
    }


    /// An offer that is no longer showable brings nothing forward: a window of
    /// the same identity, open for another offer, keeps its title and is not
    /// reported as brought forward.
    #[test]
    fn an_offer_no_longer_showable_brings_no_window_forward() {
        let custody = rendered(&routes::test_pool());
        let open = Mutex::new(Vec::new());
        opened(&open, offer(&custody), &routes::test_pool(), "report-view-1");
        let mut retired = offer(&custody);
        retired.showable = Arc::new(|| false);
        retired.title = "Calculation report: a retired offer".to_string();
        let lease_pool = routes::test_pool();
        let refusal = reserve_in(&open, retired, command_lease(&lease_pool), || panic!("no label for a refused offer")).unwrap_err();
        assert_eq!(refusal.code, VIEW_NOT_SHOWABLE);
        assert_eq!(lease_pool.reserved_total(), 0);
        let records = lock(&open);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].title, current_render_title(None), "the open window keeps the title of the offer it was opened for");
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
                "let ViewDocument { identity, document, byte_count, title, words, details, may_leave, showable } = document;" if file == "report_view.rs" => shorthand += 1,
                other => panic!("{file} reads may_leave: {other}"),
            }
        }
        assert_eq!(shorthand, 2, "the record constructor's destructuring and its field, and nothing else");
    }


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


    /// The opening reserves the serve's two copies' envelopes in the record; a window closed before
    /// its page loads returns them.
    #[test]
    fn a_window_closed_before_its_page_loads_returns_the_reservation() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let lease_pool = routes::test_pool();
        let open = Mutex::new(Vec::new());
        opened(&open, offer(&custody), &lease_pool, "report-view-1");
        assert_eq!(serve_reservation_bytes(HTML.len()), 2 * crate::ingress::allocation_envelope(HTML.len() as u64).unwrap(), "two copies");
        assert_eq!(lease_pool.reserved_total(), 1 + serve_reservation_bytes(HTML.len()));
        assert!(lease_pool.reserved(LeasePurpose::Ingress) >= serve_reservation_bytes(HTML.len()));
        assert!(window_destroyed_in(&open, &AtomicBool::new(false), &Present, "report-view-1").is_empty());
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
        assert_eq!(respond_at(&open, "report-view-1", &identity().document_path()).status(), 200);
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
        assert_eq!(reserved, Opened::BroughtForward { label: "report-view-1".to_string(), title: "Calculation report: Invented line".to_string() });
        assert_eq!(lease_pool.reserved_total(), 0, "the command's lease was dropped ungrown");
        let records = lock(&open);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].title, "Calculation report: Invented line");
        assert_eq!(records[0].words, vec![CURRENT_RENDER_WORDS.to_string()], "the words are as first offered");
        assert!(!records[0].document_spent, "nothing was served");
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
            byte_count: "<p>saved</p>".len(),
            title: "Saved".to_string(),
            words: Vec::new(),
            details: Vec::new(),
            may_leave: true,
            showable: Arc::new(|| true),
        };
        assert!(matches!(reserve_in(&open, other, command_lease(&routes::test_pool()), label("report-view-2")).unwrap(), Opened::New { .. }));
        assert_eq!(lock(&open).len(), 2);
        assert_eq!(respond_at(&open, "report-view-2", &format!("/saved_report/{}/document", sha256_hex(b"<p>saved</p>"))).body(), b"<p>saved</p>");
        assert_refused(respond_at(&open, "report-view-2", &identity().document_path()));
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
            byte_count: HTML.len(),
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


    /// With a view of the current render open, every retirement releases the retired render's
    /// document at once, the window notwithstanding; the sweep then asks the window's destruction:
    /// the record is closing and serves nothing, and goes, with its reservation, when the
    /// destruction is observed.
    #[test]
    fn a_view_of_the_current_render_closes_at_every_retirement() {
        for retirement in RETIREMENTS {
            let pool = routes::test_pool();
            let custody = rendered(&pool);
            let open = Mutex::new(Vec::new());
            let lease_pool = routes::test_pool();
            opened(&open, offer(&custody), &lease_pool, "report-view-1");
            assert!(close_unshowable_in(&open, &Present).is_empty(), "{retirement:?}: a current view is kept");
            retire(&custody, &pool, retirement);
            let later = match retirement {
                Retirement::Completion => held_document_bytes(b"<p>later</p>".len()),
                _ => 0,
            };
            assert_eq!(pool.reserved_total(), later, "{retirement:?}: the retired document is released though its window is open");
            assert_eq!(close_unshowable_in(&open, &Present), vec!["report-view-1".to_string()], "{retirement:?}");
            assert_refused(respond_at(&open, "report-view-1", &identity().wrapper_path()));
            assert_eq!(lease_pool.reserved_total(), 1 + serve_reservation_bytes(HTML.len()), "{retirement:?}: held until the destruction is observed");
            assert!(window_destroyed_in(&open, &AtomicBool::new(false), &Present, "report-view-1").is_empty());
            assert!(lock(&open).is_empty());
            assert_eq!(lease_pool.reserved_total(), 0);
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
    /// that window only; the sweep then closes the window; and both reservations, the document's
    /// in custody and the window's of the serve's copies, return to their pools when the serve
    /// ends (its answer handed on and its share dropped, as `serve` does), not before and not later.
    #[test]
    fn a_retirement_during_a_serve_returns_at_once_and_the_reservations_return_when_the_serve_ends() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let lease_pool = routes::test_pool();
        let open = Arc::new(Mutex::new(Vec::new()));
        opened(&open, offer(&custody), &lease_pool, "report-view-1");
        let held = held_document_bytes(HTML.len());
        let window = 1 + serve_reservation_bytes(HTML.len());
        let balances = || (pool.reserved(LeasePurpose::Custody), lease_pool.reserved_total());
        assert_eq!(balances(), (held, window));
        let (taken_tx, taken_rx) = std::sync::mpsc::channel();
        let (go_tx, go_rx) = std::sync::mpsc::channel::<()>();
        let server = {
            let open = Arc::clone(&open);
            std::thread::spawn(move || {
                let Answer { response, served, reservation } = respond_between(&open, "report-view-1", &format!("{SCHEME}://{HOST}{}", identity().document_path()), || {
                    taken_tx.send(()).unwrap();
                    go_rx.recv().unwrap();
                });
                // The response is handed on (`respond`), then the serve's share is dropped.
                let handed_on = (response.status(), response.into_body());
                drop(reservation);
                (handed_on, served)
            })
        };
        taken_rx.recv().unwrap();
        // The serve holds its shares and its copy has not begun. The retirement returns at once.
        let started = std::time::Instant::now();
        assert!(custody.retire_active());
        assert!(started.elapsed() < std::time::Duration::from_secs(1), "the retirement did not wait for the serve");
        assert_eq!(balances(), (held, window), "not before: the serve's shares keep both reservations");
        assert_eq!(close_unshowable_in(&open, &Present).len(), 1, "the sweep asks the window's destruction during the serve");
        assert_eq!(balances(), (held, window), "the record keeps its share until the destruction is observed");
        assert!(window_destroyed_in(&open, &AtomicBool::new(false), &Present, "report-view-1").is_empty());
        assert!(lock(&open).is_empty(), "the destruction is observed: the record, and its share of the reservation, are gone");
        assert_eq!(balances(), (held, window), "still the serve's: the window's reservation is held while the serve runs");
        assert_refused(respond_at(&open, "report-view-2", &identity().document_path()));
        go_tx.send(()).unwrap();
        let ((status, body), served) = server.join().unwrap();
        assert_eq!(status, 200);
        assert_eq!(body, HTML.as_bytes(), "the retired document's own bytes");
        assert_eq!(served, Some((identity(), Part::Document)));
        assert_eq!(balances(), (0, 0), "not later: both released as the serve ended");
        assert_eq!(pool.reserved_total(), 0);
    }


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


    /// A page that loads before its document was served ends the reservation and marks the
    /// document as no longer servable: no first serve can follow with nothing reserved.
    #[test]
    fn a_page_that_loads_without_its_document_can_serve_it_no_more() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let lease_pool = routes::test_pool();
        let open = Mutex::new(Vec::new());
        opened(&open, offer(&custody), &lease_pool, "report-view-1");
        assert!(page_loaded_in(&open, "report-view-1"));
        assert_eq!(lease_pool.reserved_total(), 0);
        assert_refused(respond_at(&open, "report-view-1", &identity().document_path()));
        assert_eq!(respond_at(&open, "report-view-1", &identity().wrapper_path()).status(), 200, "the wrapper is still served");
        assert_eq!(lease_pool.reserved_total(), 0);
    }


    /// The record and the serve share one reservation: the record's release (its page load, here)
    /// during a serve leaves it held until the serve lets go.
    #[test]
    fn a_page_load_during_a_serve_leaves_the_reservation_to_the_serve() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let lease_pool = routes::test_pool();
        let open = Mutex::new(Vec::new());
        opened(&open, offer(&custody), &lease_pool, "report-view-1");
        let window = 1 + serve_reservation_bytes(HTML.len());
        let answer = respond_between(&open, "report-view-1", &identity().document_address().unwrap().to_string(), || {});
        assert_eq!(answer.response.status(), 200);
        assert!(page_loaded_in(&open, "report-view-1"));
        assert_eq!(lease_pool.reserved_total(), window, "the serve's share holds it");
        drop(answer);
        assert_eq!(lease_pool.reserved_total(), 0);
    }


    /// The destruction is handled from the run-event callback that exists from setup, never from a
    /// listener added to a window after its construction; the protocol is registered with its
    /// responder.
    #[test]
    fn destruction_is_handled_from_the_run_event_callback() {
        let lib = source_file("lib.rs");
        let run = &lib[lib.find("pub fn run() {").unwrap()..];
        let run = &run[..run.find("\n}\n").unwrap()];
        assert!(run.contains("if let tauri::RunEvent::WindowEvent { label, event: tauri::WindowEvent::Destroyed, .. } = &event {\n                report_view::window_destroyed(label);"));
        assert!(run.contains(".register_asynchronous_uri_scheme_protocol(report_view::SCHEME, report_view::serve)"));
        let text = source_file("report_view.rs");
        assert!(!production(&text).contains(".on_window_event("));
        // The serve's selection and copies run off the main thread.
        let serve = &text[text.find("pub(crate) fn serve<R: Runtime>(").unwrap()..];
        assert!(serve[..serve.find("\n}\n").unwrap()].contains("tauri::async_runtime::spawn_blocking(move || {"));
    }


    /// The reservation is computed from the offer's count; with the borrowed bytes already gone the
    /// open is refused as not showable with nothing reserved; a held document's count must match;
    /// and the opening's path takes no share of the bytes.
    #[test]
    fn the_opening_reserves_from_the_count_and_takes_no_share() {
        let pool = routes::test_pool();
        let held = HeldDocument::hold(&pool, HTML, &|| false).unwrap();
        let borrowed = |byte_count: usize, document: Weak<HeldDocument>| ViewDocument {
            identity: identity(),
            document: ViewBytes::Borrowed(document),
            byte_count,
            title: "Invented".to_string(),
            words: Vec::new(),
            details: Vec::new(),
            may_leave: false,
            showable: Arc::new(|| true),
        };
        let lease_pool = routes::test_pool();
        let kept = Mutex::new(Vec::new());
        assert!(reserve_in(&kept, borrowed(10_000, Arc::downgrade(&held)), command_lease(&lease_pool), label("report-view-1")).is_ok());
        assert_eq!(lease_pool.reserved_total(), 1 + serve_reservation_bytes(10_000), "from the count");
        drop(kept);
        let gone = Arc::downgrade(&held);
        drop(held);
        assert_eq!(gone.strong_count(), 0);
        let lease_pool = routes::test_pool();
        let open = Mutex::new(Vec::new());
        let refusal = reserve_in(&open, borrowed(HTML.len(), gone), command_lease(&lease_pool), || panic!("no record for gone bytes")).unwrap_err();
        assert_eq!(refusal.code, VIEW_NOT_SHOWABLE);
        assert!(lock(&open).is_empty());
        assert_eq!(lease_pool.reserved_total(), 0);
        let saved = HeldDocument::hold(&pool, "<p>saved</p>", &|| false).unwrap();
        let mismatched = ViewDocument {
            identity: ViewIdentity { kind: "saved_report", sha256_hex: sha256_hex(b"<p>saved</p>") },
            document: ViewBytes::Held(saved),
            byte_count: 1,
            title: "Saved".to_string(),
            words: Vec::new(),
            details: Vec::new(),
            may_leave: false,
            showable: Arc::new(|| true),
        };
        assert_eq!(reserve_in(&Mutex::new(Vec::new()), mismatched, command_lease(&routes::test_pool()), label("report-view-2")).unwrap_err().code, VIEW_DOCUMENT_REFUSED);
        let text = source_file("report_view.rs");
        let opening = &text[text.find("fn install_in<W: Windows>(").unwrap()..];
        let opening = &opening[..opening.find("\n}\n").unwrap()];
        assert!(!opening.contains("share()") && !opening.contains("upgrade("), "the opening takes no share");
    }


    /// A request from the right webview with the right path is refused when its authority, query or
    /// fragment differ.
    #[test]
    fn a_request_is_answered_only_for_the_whole_address() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Mutex::new(Vec::new());
        opened(&open, offer(&custody), &routes::test_pool(), "report-view-1");
        let id = identity();
        for path in [id.wrapper_path(), id.document_path()] {
            for address in [
                format!("{SCHEME}://elsewhere{path}"),
                format!("{SCHEME}://{HOST}:8080{path}"),
                format!("swbpipe-report-print://{HOST}{path}"),
                format!("{SCHEME}://{HOST}{path}?q=1"),
                format!("{SCHEME}://{HOST}{path}?"),
                path.clone(),
            ] {
                assert_refused(respond_in(&open, "report-view-1", &address));
            }
        }
        assert_eq!(respond_in(&open, "report-view-1", &id.wrapper_address().unwrap().to_string()).status(), 200);
        assert_eq!(respond_in(&open, "report-view-1", &id.document_address().unwrap().to_string()).status(), 200);
    }

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


    // ---- S2: the opening.

    /// Two openings of one identity never interleave: while the first is paused at its build or
    /// at its acknowledgement, the second waits at the gate; then it brings the first's window
    /// forward. Exactly one window, one record, and no lease left over but the window's.
    #[test]
    fn two_openings_of_one_identity_never_interleave() {
        for pause_at in ["build", "acknowledge"] {
            let pool = routes::test_pool();
            let custody = rendered(&pool);
            let open = Arc::new(Mutex::new(Vec::new()));
            let gate = Arc::new(Mutex::new(()));
            let model = Arc::new(Model::default());
            let lease_pool = routes::test_pool();
            let (at_tx, at_rx) = channel::<()>();
            let (go_tx, go_rx) = channel::<()>();
            let go_rx = Mutex::new(go_rx);
            let first = AtomicUsize::new(0);
            *lock(&model.hook) = Some(Arc::new(move |point: &str| {
                if point == pause_at && first.fetch_add(1, Ordering::SeqCst) == 0 {
                    at_tx.send(()).unwrap();
                    lock(&go_rx).recv().unwrap();
                }
            }));
            let start = |name: &'static str| {
                let (open, gate, model, document, lease) = (Arc::clone(&open), Arc::clone(&gate), Arc::clone(&model), offer(&custody), command_lease(&lease_pool));
                std::thread::spawn(move || open_with(&open, &gate, &model, document, lease, name))
            };
            let a = start("report-view-1");
            at_rx.recv().unwrap();
            let b = start("report-view-2");
            std::thread::sleep(std::time::Duration::from_millis(100));
            assert_eq!(lock(&open).len(), 1, "{pause_at}: the second opening waits at the gate");
            go_tx.send(()).unwrap();
            let title = current_render_title(None);
            assert_eq!(a.join().unwrap(), Ok(Opened::New { label: "report-view-1".to_string(), title: title.clone() }), "{pause_at}");
            assert_eq!(b.join().unwrap(), Ok(Opened::BroughtForward { label: "report-view-1".to_string(), title }), "{pause_at}");
            assert_eq!(lock(&open).iter().map(|record| record.label.clone()).collect::<Vec<_>>(), vec!["report-view-1".to_string()]);
            assert_eq!(*lock(&model.registry), BTreeSet::from(["report-view-1".to_string()]), "{pause_at}: one window");
            assert_eq!(lease_pool.reserved_total(), 1 + serve_reservation_bytes(HTML.len()), "{pause_at}: only the window's reservation");
            model.destroyed(&open, "report-view-1");
            assert_eq!(lease_pool.reserved_total(), 0);
        }
    }

    /// The scripted run's close waits for every record to go (`recorded_labels`): a close
    /// (`close_where_in`, as `close_all` makes it) leaves a record being built, only marked, and
    /// one already closing, skipped, both still recorded until their windows' destruction is
    /// observed.
    #[test]
    fn a_record_being_built_or_already_closing_stays_recorded_after_a_close() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Arc::new(Mutex::new(Vec::new()));
        let gate = Arc::new(Mutex::new(()));
        let model = Arc::new(Model::default());
        let lease_pool = routes::test_pool();
        // One window open, then closed once: its record is closing until its destruction is observed.
        open_with(&open, &gate, &model, offer(&custody), command_lease(&lease_pool), "report-view-1").unwrap();
        assert_eq!(close_where_in(&open, None, &*model, |_| true), vec!["report-view-1".to_string()]);
        // A second opening paused at its build.
        let (at_tx, at_rx) = channel::<()>();
        let (go_tx, go_rx) = channel::<()>();
        let go_rx = Mutex::new(go_rx);
        *lock(&model.hook) = Some(Arc::new(move |point: &str| {
            if point == "build" {
                at_tx.send(()).unwrap();
                lock(&go_rx).recv().unwrap();
            }
        }));
        let building = {
            let (open, gate, model, document, lease) = (Arc::clone(&open), Arc::clone(&gate), Arc::clone(&model), offer(&custody), command_lease(&lease_pool));
            std::thread::spawn(move || open_with(&open, &gate, &model, document, lease, "report-view-2"))
        };
        at_rx.recv().unwrap();
        // The close asks nothing of either: the building one is only marked, the closing one skipped.
        assert!(close_where_in(&open, None, &*model, |_| true).is_empty());
        let mut recorded = recorded_labels_in(&open);
        recorded.sort();
        assert_eq!(recorded, vec!["report-view-1".to_string(), "report-view-2".to_string()], "both still recorded");
        go_tx.send(()).unwrap();
        assert!(building.join().unwrap().is_err(), "the opening of a window marked closing while built does not succeed");
        assert!(lock(&model.destroy_asked).contains(&"report-view-2".to_string()), "its opening asked its destruction");
        for label in ["report-view-1", "report-view-2"] {
            model.destroyed(&open, label);
        }
        assert!(recorded_labels_in(&open).is_empty(), "each record goes once its destruction is observed");
    }

    /// An offer whose answer turns false while the opening waits for the records' lock matches
    /// nothing and brings nothing forward: the answer that counts is the one under the lock.
    #[test]
    fn an_offer_retired_before_the_lock_matches_nothing_and_brings_nothing_forward() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Arc::new(Mutex::new(Vec::new()));
        opened(&open, offer(&custody), &routes::test_pool(), "report-view-1");
        let current = Arc::new(AtomicBool::new(true));
        let mut later = offer(&custody);
        later.title = "Calculation report: a later offer".to_string();
        later.showable = {
            let current = Arc::clone(&current);
            Arc::new(move || current.load(Ordering::SeqCst))
        };
        let lease_pool = routes::test_pool();
        let held = lock(&open);
        let opening = {
            let (open, lease) = (Arc::clone(&open), command_lease(&lease_pool));
            std::thread::spawn(move || reserve_in(&open, later, lease, || panic!("no label for a retired offer")))
        };
        std::thread::sleep(std::time::Duration::from_millis(100));
        current.store(false, Ordering::SeqCst);
        drop(held);
        assert_eq!(opening.join().unwrap().unwrap_err().code, VIEW_NOT_SHOWABLE);
        assert_eq!(lease_pool.reserved_total(), 0);
        let records = lock(&open);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].title, current_render_title(None), "nothing was brought forward");
    }

    /// A window that does not answer the acknowledgement (its creation was queued and failed) is a
    /// failed opening: refused by name, no record, the pool back to where it was, and its
    /// destruction asked in case it comes into being.
    #[test]
    fn a_window_that_does_not_acknowledge_is_a_failed_opening_and_leaves_nothing() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Mutex::new(Vec::new());
        let model = Model::default();
        model.creation_fails.store(true, Ordering::SeqCst);
        let lease_pool = routes::test_pool();
        let refusal = open_with(&open, &Mutex::new(()), &model, offer(&custody), command_lease(&lease_pool), "report-view-1").unwrap_err();
        assert_eq!((refusal.code.as_str(), refusal.message.as_str()), (VIEW_WINDOW_FAILED, "The report's window could not be opened."));
        assert!(lock(&open).is_empty());
        assert_eq!(lease_pool.reserved_total(), 0, "no record can be left for a window that never existed");
        assert_eq!(*lock(&model.destroy_asked), vec!["report-view-1".to_string()]);
    }

    /// `brought_forward` is answered only for a window found and reached: a record whose window
    /// is gone (its destruction not yet observed) is dropped with its reservation and a new window
    /// opens; a window the registry holds but the runtime does not answer for refuses the opening.
    #[test]
    fn brought_forward_only_for_a_window_found_and_reached() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Mutex::new(Vec::new());
        let gate = Mutex::new(());
        let model = Model::default();
        let first_pool = routes::test_pool();
        assert!(matches!(open_with(&open, &gate, &model, offer(&custody), command_lease(&first_pool), "report-view-1"), Ok(Opened::New { .. })));
        assert!(matches!(open_with(&open, &gate, &model, offer(&custody), command_lease(&routes::test_pool()), "report-view-x"), Ok(Opened::BroughtForward { label, .. }) if label == "report-view-1"));
        // The person closed it; its destruction is not yet observed.
        lock(&model.registry).remove("report-view-1");
        lock(&model.runtime).remove("report-view-1");
        let opened = open_with(&open, &gate, &model, offer(&custody), command_lease(&routes::test_pool()), "report-view-2");
        assert_eq!(opened, Ok(Opened::New { label: "report-view-2".to_string(), title: current_render_title(None) }));
        assert_eq!(first_pool.reserved_total(), 0, "the stale record's reservation is released");
        assert_eq!(lock(&open).iter().map(|record| record.label.clone()).collect::<Vec<_>>(), vec!["report-view-2".to_string()]);
        assert!(lock(&model.destroy_asked).contains(&"report-view-1".to_string()), "the stale label's window is asked to close, a second barrier");
        // The registry holds it, the runtime does not answer for it.
        lock(&model.runtime).remove("report-view-2");
        let refusal = open_with(&open, &gate, &model, offer(&custody), command_lease(&routes::test_pool()), "report-view-3").unwrap_err();
        assert_eq!(refusal.code, VIEW_WINDOW_FAILED);
    }

    /// The bring-forward path asks the same two questions under the lock as an opening does: an
    /// offer whose borrowed document is gone brings nothing forward, even with the window open.
    #[test]
    fn a_gone_document_brings_nothing_forward() {
        let pool = routes::test_pool();
        let held = HeldDocument::hold(&pool, HTML, &|| false).unwrap();
        let offer_of = |document| ViewDocument {
            identity: identity(),
            document: ViewBytes::Borrowed(document),
            byte_count: HTML.len(),
            title: "Invented".to_string(),
            words: Vec::new(),
            details: Vec::new(),
            may_leave: false,
            showable: Arc::new(|| true),
        };
        let open = Mutex::new(Vec::new());
        let model = Model::default();
        let gate = Mutex::new(());
        assert!(matches!(open_with(&open, &gate, &model, offer_of(Arc::downgrade(&held)), command_lease(&routes::test_pool()), "report-view-1"), Ok(Opened::New { .. })));
        let gone = Arc::downgrade(&held);
        drop(held);
        let mut later = offer_of(gone);
        later.title = "Invented, later".to_string();
        let refusal = open_with(&open, &gate, &model, later, command_lease(&routes::test_pool()), "report-view-2").unwrap_err();
        assert_eq!(refusal.code, VIEW_NOT_SHOWABLE);
        assert_eq!(lock(&open)[0].title, "Invented", "nothing was brought forward");
    }

    /// The acknowledgement is the runtime's window getter, with its records' lock not held.
    #[test]
    fn the_acknowledgement_asks_the_runtime_a_getter() {
        let text = source_file("report_view.rs");
        let production = production(&text);
        assert!(production.contains("    fn acknowledge(&self, window: &Self::Window) -> bool {\n        // A getter, answered only for a window in the runtime's map.\n        window.title().is_ok()\n    }"));
    }

    // ---- S3: a sweep asks the destruction, and the reservation outlives it until observed.

    /// A sweep before the page has loaded keeps the two envelopes until the window's destruction
    /// is observed; after the page has loaded nothing is held; a sweep that finds the window gone
    /// removes the record at once.
    #[test]
    fn a_sweep_keeps_the_reservation_until_the_destruction_is_observed() {
        let window = 1 + serve_reservation_bytes(HTML.len());
        // Before the page has loaded.
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Mutex::new(Vec::new());
        let model = Model::default();
        let lease_pool = routes::test_pool();
        open_with(&open, &Mutex::new(()), &model, offer(&custody), command_lease(&lease_pool), "report-view-1").unwrap();
        assert!(custody.retire_active());
        assert_eq!(close_unshowable_in(&open, &model), vec!["report-view-1".to_string()]);
        assert_eq!(*lock(&model.destroy_asked), vec!["report-view-1".to_string()]);
        assert_eq!(lease_pool.reserved_total(), window, "held while the window may still be serving");
        assert_eq!(lock(&open).len(), 1);
        model.destroyed(&open, "report-view-1");
        assert!(lock(&open).is_empty());
        assert_eq!(lease_pool.reserved_total(), 0);
        // After the page has loaded.
        let custody = rendered(&pool);
        let lease_pool = routes::test_pool();
        open_with(&open, &Mutex::new(()), &model, offer(&custody), command_lease(&lease_pool), "report-view-2").unwrap();
        assert!(page_loaded_in(&open, "report-view-2"));
        assert!(custody.retire_active());
        assert_eq!(close_unshowable_in(&open, &model), vec!["report-view-2".to_string()]);
        assert_eq!(lease_pool.reserved_total(), 0);
        model.destroyed(&open, "report-view-2");
        // The window already gone at the sweep.
        let custody = rendered(&pool);
        let lease_pool = routes::test_pool();
        open_with(&open, &Mutex::new(()), &model, offer(&custody), command_lease(&lease_pool), "report-view-3").unwrap();
        lock(&model.registry).remove("report-view-3");
        assert!(custody.retire_active());
        assert!(close_unshowable_in(&open, &model).is_empty());
        assert!(lock(&open).is_empty(), "the record goes at once");
        assert_eq!(lease_pool.reserved_total(), 0);
    }

    /// A closing record is served nothing and brought forward by no offer: a new opening of the
    /// same identity (a later render of the same content) opens a new window under a new label.
    #[test]
    fn a_closing_record_is_not_served_or_brought_forward() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Mutex::new(Vec::new());
        let gate = Mutex::new(());
        let model = Model::default();
        open_with(&open, &gate, &model, offer(&custody), command_lease(&routes::test_pool()), "report-view-1").unwrap();
        // A later render of the same content retires the first and is current.
        custody.record_in(&pool, Some(PUBLIC_REPORT), HTML, &sha256_hex(HTML.as_bytes()), false, false);
        assert_eq!(close_unshowable_in(&open, &model), vec!["report-view-1".to_string()]);
        assert_refused(respond_at(&open, "report-view-1", &identity().wrapper_path()));
        assert_refused(respond_at(&open, "report-view-1", &identity().document_path()));
        let opened = open_with(&open, &gate, &model, offer(&custody), command_lease(&routes::test_pool()), "report-view-2");
        assert_eq!(opened, Ok(Opened::New { label: "report-view-2".to_string(), title: current_render_title(None) }));
        assert_eq!(lock(&open).len(), 2, "the closing record stays until its destruction is observed");
        assert_eq!(respond_at(&open, "report-view-2", &identity().wrapper_path()).status(), 200);
        assert_refused(respond_at(&open, "report-view-1", &identity().wrapper_path()));
    }

    /// A sweep, or the main window's shutdown, while the window is built: the opening is refused
    /// with what is true (not showable after a sweep, failed after the shutdown) and the window is
    /// destroyed.
    #[test]
    fn a_record_retired_while_its_window_is_built_refuses_the_opening() {
        for shutdown in [false, true] {
            let pool = routes::test_pool();
            let custody = rendered(&pool);
            let open = Arc::new(Mutex::new(Vec::new()));
            let shut = Arc::new(AtomicBool::new(false));
            let model = Arc::new(Model::default());
            {
                let (open, shut, custody, model_ref) = (Arc::clone(&open), Arc::clone(&shut), Arc::clone(&custody), Arc::downgrade(&model));
                *lock(&model.hook) = Some(Arc::new(move |point: &str| {
                    if point != "acknowledge" {
                        return;
                    }
                    let model = model_ref.upgrade().unwrap();
                    if shutdown {
                        let _ = window_destroyed_in(&open, &shut, &*model, "main");
                    } else {
                        assert!(custody.retire_active());
                        let _ = close_unshowable_in(&open, &*model);
                    }
                }));
            }
            let lease_pool = routes::test_pool();
            let refusal = open_in(&open, &Mutex::new(()), &shut, &*model, offer(&custody), command_lease(&lease_pool), label("report-view-1")).unwrap_err();
            assert_eq!(refusal.code, if shutdown { VIEW_WINDOW_FAILED } else { VIEW_NOT_SHOWABLE }, "shutdown: {shutdown}");
            assert!(lock(&model.destroy_asked).contains(&"report-view-1".to_string()));
            model.destroyed(&open, "report-view-1");
            assert_eq!(lease_pool.reserved_total(), 0);
        }
    }

    // ---- Currency boundaries: every retirement of the native render closes its windows.

    /// Lane 4's test: a registered advance retires the current render, and the sweep beside it asks
    /// the destruction of an open window of it; here a local private render. Nothing of it is then
    /// served, and the earlier offer brings nothing forward.
    #[test]
    fn a_registered_advance_closes_an_open_window_of_a_local_private_render() {
        let pool = routes::test_pool();
        let custody = rendered_in(&pool, LOCAL_PRIVATE);
        let open = Mutex::new(Vec::new());
        let model = Model::default();
        let earlier = offer(&custody);
        open_with(&open, &Mutex::new(()), &model, offer(&custody), command_lease(&routes::test_pool()), "report-view-1").unwrap();
        assert!(custody.advance(SourceMark { incarnation: 1, sequence: 1 }).unwrap().1, "the advance retires the render");
        assert_eq!(close_unshowable_in(&open, &model), vec!["report-view-1".to_string()]);
        assert_eq!(*lock(&model.destroy_asked), vec!["report-view-1".to_string()]);
        assert_refused(respond_at(&open, "report-view-1", &identity().wrapper_path()));
        assert_refused(respond_at(&open, "report-view-1", &identity().document_path()));
        assert_eq!(open_with(&open, &Mutex::new(()), &model, earlier, command_lease(&routes::test_pool()), "report-view-2").unwrap_err().code, VIEW_NOT_SHOWABLE);
    }

    /// An activation of a new incarnation (the page's report source session starting again, as
    /// after the web view reloads) retires the earlier render, and the sweep beside it closes its
    /// window. A project switch is an advance, not an activation.
    #[test]
    fn an_activation_closes_an_open_window_of_the_earlier_incarnations_render() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Mutex::new(Vec::new());
        let model = Model::default();
        let earlier = offer(&custody);
        open_with(&open, &Mutex::new(()), &model, offer(&custody), command_lease(&routes::test_pool()), "report-view-1").unwrap();
        let _ = custody.activate();
        assert_eq!(close_unshowable_in(&open, &model), vec!["report-view-1".to_string()]);
        assert_refused(respond_at(&open, "report-view-1", &identity().wrapper_path()));
        assert_eq!(open_with(&open, &Mutex::new(()), &model, earlier, command_lease(&routes::test_pool()), "report-view-2").unwrap_err().code, VIEW_NOT_SHOWABLE);
    }

    /// The main window's destruction (the application quitting) closes every view window, and no
    /// later opening succeeds.
    #[test]
    fn the_main_windows_destruction_closes_every_view_window_and_no_opening_follows() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Mutex::new(Vec::new());
        let shut = AtomicBool::new(false);
        let model = Model::default();
        let saved = HeldDocument::hold(&pool, "<p>saved</p>", &|| false).unwrap();
        open_in(&open, &Mutex::new(()), &shut, &model, offer(&custody), command_lease(&routes::test_pool()), label("report-view-1")).unwrap();
        let other = ViewDocument {
            identity: ViewIdentity { kind: "saved_report", sha256_hex: sha256_hex(b"<p>saved</p>") },
            document: ViewBytes::Held(saved),
            byte_count: "<p>saved</p>".len(),
            title: "Saved".to_string(),
            words: Vec::new(),
            details: Vec::new(),
            may_leave: false,
            showable: Arc::new(|| true),
        };
        open_in(&open, &Mutex::new(()), &shut, &model, other, command_lease(&routes::test_pool()), label("report-view-2")).unwrap();
        assert!(window_destroyed_in(&open, &shut, &model, "print-window-1").is_empty(), "another window's destruction changes nothing");
        let mut asked = window_destroyed_in(&open, &shut, &model, "main");
        asked.sort();
        assert_eq!(asked, vec!["report-view-1".to_string(), "report-view-2".to_string()]);
        assert!(shut.load(Ordering::SeqCst));
        let lease_pool = routes::test_pool();
        let refusal = open_in(&open, &Mutex::new(()), &shut, &model, offer(&custody), command_lease(&lease_pool), || panic!("nothing opens after the shutdown")).unwrap_err();
        assert_eq!(refusal.code, VIEW_WINDOW_FAILED);
        assert_eq!(lease_pool.reserved_total(), 0);
    }

    // ---- The window states a local private rendering, from the provider.

    /// A local private render's strip carries the product's warning; a public one's does not.
    #[test]
    fn a_local_private_render_states_itself_in_the_window_and_a_public_one_does_not() {
        for (context, warned) in [(LOCAL_PRIVATE, true), (PUBLIC_REPORT, false)] {
            let pool = routes::test_pool();
            let custody = rendered_in(&pool, context);
            let open = Mutex::new(Vec::new());
            opened(&open, offer(&custody), &routes::test_pool(), "report-view-1");
            let page = String::from_utf8(respond_at(&open, "report-view-1", &identity().wrapper_path()).into_body()).unwrap();
            assert_eq!(page.contains(&format!("<p>{}</p>", escape(LOCAL_PRIVATE_WORDS))), warned, "{context}");
            assert!(page.contains(&format!("<p>{}</p>", escape(CURRENT_RENDER_WORDS))), "{context}");
        }
    }

    /// An offer of a render of any other context, or none, is refused and opens nothing.
    #[test]
    fn an_offer_of_any_other_context_is_refused() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let admitted = custody.admit_view(&sha256_hex(HTML.as_bytes()), FIRST).unwrap();
        for context in [None, Some("user_released_report".to_string()), Some("invented".to_string())] {
            let refusal = current_render_document(ViewAdmitted { export_context: context.clone(), ..admitted.clone() }, |_| true).err().unwrap();
            assert_eq!(refusal.code, VIEW_DOCUMENT_REFUSED, "{context:?}");
        }
    }

    /// The local private sentence is the product's warning, word for word.
    #[test]
    fn the_local_private_words_are_the_products_warning() {
        let panel = std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../src/features/report/RenderedReportPanel.tsx")).unwrap();
        assert!(panel.contains(&format!("const LOCAL_PRIVATE_REPORT_WARNING =\n  \"{LOCAL_PRIVATE_WORDS}\";")));
        assert!(LOCAL_PRIVATE_WORDS.len() < 200 && 2 <= MAX_WORDS);
    }

    // ---- S4: what the handler sees.

    /// A fragment never reaches the handler: the http conversion drops it, so a request for the
    /// document with a fragment is the document's address; a query is kept and refused.
    #[test]
    fn a_fragment_never_reaches_the_handler_and_a_query_is_refused() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Mutex::new(Vec::new());
        opened(&open, offer(&custody), &routes::test_pool(), "report-view-1");
        let wrapper = identity().wrapper_address().unwrap().to_string();
        let request = |uri: String| tauri::http::Request::builder().uri(uri).body(Vec::new()).unwrap();
        assert_eq!(request_address(&request(format!("{wrapper}#part"))), wrapper, "the fragment is dropped before the handler");
        assert_eq!(respond_in(&open, "report-view-1", &request_address(&request(format!("{wrapper}#part")))).status(), 200);
        assert_refused(respond_in(&open, "report-view-1", &request_address(&request(format!("{wrapper}?q=1")))));
        // What keeps the window to its two addresses, fragment and all, is the navigation rule.
        assert!(!navigation_allowed(&identity(), &tauri::Url::parse(&format!("{wrapper}#part")).unwrap()));
    }

    // ---- Round 3 repairs.

    /// Installs `hook` on `model`, holding the model weakly (the hook lives in the model).
    fn hook_on(model: &Arc<Model>, hook: impl Fn(&str, &Model) + Send + Sync + 'static) {
        let weak = Arc::downgrade(model);
        *lock(&model.hook) = Some(Arc::new(move |point: &str| hook(point, &weak.upgrade().unwrap())));
    }

    /// Repair 1: a sweep while the window is built (attached by the runtime, its construction
    /// not finished) asks no destruction until the build has returned; then the opening destroys
    /// the window and refuses, and no reservation is left once the destruction is observed.
    #[test]
    fn a_sweep_during_the_build_destroys_nothing_until_the_build_returns() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Arc::new(Mutex::new(Vec::new()));
        let model = Arc::new(Model::default());
        {
            let (open, custody) = (Arc::clone(&open), Arc::clone(&custody));
            hook_on(&model, move |point, model| {
                if point == "build" {
                    assert!(custody.retire_active());
                    let _ = close_unshowable_in(&open, model);
                }
            });
        }
        let lease_pool = routes::test_pool();
        let outcome = open_in(&open, &Mutex::new(()), &AtomicBool::new(false), &*model, offer(&custody), command_lease(&lease_pool), label("report-view-1"));
        assert_eq!(*lock(&model.destroyed_while_building), Vec::<String>::new(), "no destruction while the build runs");
        assert_eq!(outcome.as_ref().err().map(|refusal| refusal.code.as_str()), Some(VIEW_NOT_SHOWABLE), "{outcome:?}");
        assert_eq!(*lock(&model.destroy_asked), vec!["report-view-1".to_string()], "asked once the build has returned");
        model.destroyed(&open, "report-view-1");
        assert!(lock(&open).is_empty());
        assert_eq!(lease_pool.reserved_total(), 0);
    }

    /// Repair 2 (a): a sweep while a window is brought forward refuses the opening; it is not
    /// answered as brought forward.
    #[test]
    fn a_sweep_during_a_bring_forward_refuses_the_opening() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Arc::new(Mutex::new(Vec::new()));
        let model = Arc::new(Model::default());
        let gate = Mutex::new(());
        open_with(&open, &gate, &model, offer(&custody), command_lease(&routes::test_pool()), "report-view-1").unwrap();
        {
            let (open, custody) = (Arc::clone(&open), Arc::clone(&custody));
            hook_on(&model, move |point, model| {
                if point == "bring_forward" {
                    assert!(custody.retire_active());
                    let _ = close_unshowable_in(&open, model);
                }
            });
        }
        let outcome = open_with(&open, &gate, &model, offer(&custody), command_lease(&routes::test_pool()), "report-view-2");
        assert_eq!(outcome.as_ref().err().map(|refusal| refusal.code.as_str()), Some(VIEW_NOT_SHOWABLE), "{outcome:?}");
    }

    /// Repair 2 (b): the runtime's bring-forward restores a minimized window before focusing it,
    /// and answers true only when the getter and every step answered.
    #[test]
    fn a_bring_forward_unminimizes_and_answers_only_when_every_step_answers() {
        let text = source_file("report_view.rs");
        assert!(production(&text).contains(
            "    fn bring_forward(&self, window: &Self::Window, title: &str) -> bool {\n        // The getter first (only a window in the runtime's map answers it), then each\n        // step's own answer; a minimized window is restored before it is focused.\n        window.title().is_ok() && window.unminimize().is_ok() && window.set_title(title).is_ok() && window.set_focus().is_ok()\n    }"
        ));
    }

    /// Repair 3 (a): a shutdown before the check after the acknowledgement, with the record not
    /// yet marked, refuses as a failed window and destroys the window.
    #[test]
    fn a_shutdown_before_the_check_after_acknowledgement_refuses_and_destroys() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Arc::new(Mutex::new(Vec::new()));
        let shut = Arc::new(AtomicBool::new(false));
        let model = Arc::new(Model::default());
        {
            let shut = Arc::clone(&shut);
            hook_on(&model, move |point, _| {
                if point == "acknowledge" {
                    shut.store(true, Ordering::SeqCst);
                }
            });
        }
        let lease_pool = routes::test_pool();
        let outcome = open_in(&open, &Mutex::new(()), &shut, &*model, offer(&custody), command_lease(&lease_pool), label("report-view-1"));
        assert_eq!(outcome.as_ref().err().map(|refusal| refusal.code.as_str()), Some(VIEW_WINDOW_FAILED), "{outcome:?}");
        assert!(lock(&model.destroy_asked).contains(&"report-view-1".to_string()));
        model.destroyed(&open, "report-view-1");
        assert_eq!(lease_pool.reserved_total(), 0);
    }

    /// Repair 3 (b): an offer that stops being showable during the acknowledgement, before any
    /// sweep, refuses as not showable; the record is marked closing (nothing is served from it
    /// even if the answer came back) and the window destroyed.
    #[test]
    fn an_offer_unshowable_after_acknowledgement_refuses_marks_closing_and_destroys() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Mutex::new(Vec::new());
        let model = Arc::new(Model::default());
        let current = Arc::new(AtomicBool::new(true));
        let mut document = offer(&custody);
        document.showable = {
            let current = Arc::clone(&current);
            Arc::new(move || current.load(Ordering::SeqCst))
        };
        {
            let current = Arc::clone(&current);
            hook_on(&model, move |point, _| {
                if point == "acknowledge" {
                    current.store(false, Ordering::SeqCst);
                }
            });
        }
        let outcome = open_with(&open, &Mutex::new(()), &model, document, command_lease(&routes::test_pool()), "report-view-1");
        assert_eq!(outcome.as_ref().err().map(|refusal| refusal.code.as_str()), Some(VIEW_NOT_SHOWABLE), "{outcome:?}");
        assert!(lock(&model.destroy_asked).contains(&"report-view-1".to_string()));
        current.store(true, Ordering::SeqCst);
        assert_refused(respond_at(&open, "report-view-1", &identity().wrapper_path()));
        assert!(lock(&open).iter().all(|record| record.closing));
    }

    /// Repair 4: an accepted close or quit retires the render and then closes every view window,
    /// in that order and with neither lock held across the other, before the window is destroyed
    /// or the application exits, and at the start of an exit request, before the embedded host
    /// can hold the exit while it drains.
    #[test]
    fn an_accepted_close_or_quit_retires_the_render_and_closes_the_views_first() {
        let lib = source_file("lib.rs");
        let finish = &lib[lib.find("fn finish_close<R: tauri::Runtime>").expect("finish_close")..];
        assert!(finish.contains(") {\n    rendered_report_routes::retire_for_shutdown();\n    report_view::shutdown();\n    match (kind, app.get_webview_window(\"main\")) {"), "finish_close");
        let run = &lib[lib.find(".run(|app, event| {").expect("the run callback")..];
        let wired = "if let tauri::RunEvent::ExitRequested { .. } = &event {\n                rendered_report_routes::retire_for_shutdown();\n                report_view::shutdown();\n            }";
        let at = run.find(wired).expect("the exit request retires and closes");
        assert!(at < run.find("Host>().shutdown(app)").expect("the embedded host's drain"));
    }

    /// Repair 4, the behaviour: the shutdown sequence retires the render (nothing of it is admitted
    /// any more), then in one critical section sets the flag and marks every open record closing
    /// with its window's destruction asked; nothing is served and nothing opens afterwards.
    #[test]
    fn the_shutdown_sequence_retires_the_render_and_closes_every_view_window() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Mutex::new(Vec::new());
        let shut = AtomicBool::new(false);
        let model = Model::default();
        let lease_pool = routes::test_pool();
        open_in(&open, &Mutex::new(()), &shut, &model, offer(&custody), command_lease(&lease_pool), label("report-view-1")).unwrap();
        custody.retire_for_shutdown();
        assert!(custody.admit_view(&sha256_hex(HTML.as_bytes()), FIRST).is_err(), "nothing of the render is admitted");
        assert_eq!(shutdown_in(&open, &shut, &model), vec!["report-view-1".to_string()]);
        assert!(shut.load(Ordering::SeqCst));
        assert!(lock(&open).iter().all(|record| record.closing));
        assert_refused(respond_at(&open, "report-view-1", &identity().wrapper_path()));
        let refusal = open_in(&open, &Mutex::new(()), &shut, &model, offer(&rendered(&pool)), command_lease(&routes::test_pool()), || panic!("nothing opens after the shutdown")).unwrap_err();
        assert_eq!(refusal.code, VIEW_WINDOW_FAILED);
        model.destroyed(&open, "report-view-1");
        assert_eq!(lease_pool.reserved_total(), 0);
    }

    /// Repair 5: the delivered address is matched as it is: a dot-segment spelling of the
    /// document's address is not the document's, is refused, and spends nothing, so the document
    /// is still served at its own address.
    #[test]
    fn a_dot_segment_address_is_not_the_documents_and_spends_nothing() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Mutex::new(Vec::new());
        opened(&open, offer(&custody), &routes::test_pool(), "report-view-1");
        let id = identity();
        let dotted = format!("{SCHEME}://{HOST}/{kind}/../{kind}/{sha}/document", kind = id.kind, sha = id.sha256_hex);
        assert_refused(respond_in(&open, "report-view-1", &dotted));
        assert_eq!(respond_at(&open, "report-view-1", &id.document_path()).status(), 200, "the document is still unspent");
    }

    /// Repair 1 follow-up (a): a sweep or shutdown while native creation fails during build does not
    /// leave a closing record or its reservation pinned waiting for a `Destroyed` event that cannot arrive.
    #[test]
    fn a_sweep_or_shutdown_during_a_failed_native_creation_leaves_no_reservation() {
        for shutdown in [false, true] {
            let pool = routes::test_pool();
            let custody = rendered(&pool);
            let open = Arc::new(Mutex::new(Vec::new()));
            let shut = Arc::new(AtomicBool::new(false));
            let model = Arc::new(Model::default());
            model.creation_fails.store(true, Ordering::SeqCst);
            {
                let (open, custody, shut) = (Arc::clone(&open), Arc::clone(&custody), Arc::clone(&shut));
                hook_on(&model, move |point, model| {
                    if point == "build" {
                        if shutdown {
                            let _ = shutdown_in(&open, &shut, model);
                        } else {
                            assert!(custody.retire_active());
                            let _ = close_unshowable_in(&open, model);
                        }
                    }
                });
            }
            let lease_pool = routes::test_pool();
            let outcome = open_in(&open, &Mutex::new(()), &shut, &*model, offer(&custody), command_lease(&lease_pool), label("report-view-1"));
            assert_eq!(*lock(&model.destroyed_while_building), Vec::<String>::new(), "shutdown={shutdown}");
            assert_eq!(lease_pool.reserved_total(), 0, "shutdown={shutdown}: no orphan reservation");
            assert!(lock(&open).is_empty(), "shutdown={shutdown}: no orphan closing record");
            assert_eq!(outcome.as_ref().err().map(|refusal| refusal.code.as_str()), Some(VIEW_WINDOW_FAILED), "shutdown={shutdown}: {outcome:?}");
        }
    }

    /// Repair 1 follow-up (b): when acknowledgement fails, the record and its lease are released before
    /// the asynchronous destruction request is issued, so nothing can still be selected or reserved meanwhile.
    #[test]
    fn a_failed_acknowledgement_revokes_the_record_before_asking_destruction() {
        let pool = routes::test_pool();
        let custody = rendered(&pool);
        let open = Arc::new(Mutex::new(Vec::new()));
        let lease_pool = Arc::new(routes::test_pool());
        let model = Arc::new(Model::default());
        model.creation_fails.store(true, Ordering::SeqCst);
        let status_at_destroy = Arc::new(Mutex::new(None));
        let empty_at_destroy = Arc::new(Mutex::new(None));
        let reserved_at_destroy = Arc::new(Mutex::new(None));
        {
            let (open, lease_pool, status_at_destroy, empty_at_destroy, reserved_at_destroy) = (
                Arc::clone(&open),
                Arc::clone(&lease_pool),
                Arc::clone(&status_at_destroy),
                Arc::clone(&empty_at_destroy),
                Arc::clone(&reserved_at_destroy),
            );
            hook_on(&model, move |point, _| {
                if point == "destroy" {
                    *lock(&status_at_destroy) = Some(respond_at(&open, "report-view-1", &identity().wrapper_path()).status().as_u16());
                    *lock(&empty_at_destroy) = Some(lock(&open).is_empty());
                    *lock(&reserved_at_destroy) = Some(lease_pool.reserved_total());
                }
            });
        }
        let outcome = open_in(&open, &Mutex::new(()), &AtomicBool::new(false), &*model, offer(&custody), command_lease(&lease_pool), label("report-view-1"));
        assert_eq!(outcome.as_ref().err().map(|refusal| refusal.code.as_str()), Some(VIEW_WINDOW_FAILED), "{outcome:?}");
        assert_eq!(*lock(&status_at_destroy), Some(404));
        assert_eq!(*lock(&empty_at_destroy), Some(true));
        assert_eq!(*lock(&reserved_at_destroy), Some(0));
        assert!(lock(&open).is_empty());
        assert_eq!(lease_pool.reserved_total(), 0);
    }
}
