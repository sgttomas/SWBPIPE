//! The rendered report's native routes in the desktop (DEL-08-01 REQ-022,
//! AC-011; DEL-12-02 AC-007): Save canonical HTML… through the system save
//! chooser and the atomic writer, and Print… of the derived print view in a
//! webview of its own (`rendered_report_print`). Both leave by route
//! `DREP-HTML-SAVE-005`, the catalogue's rendered-document route, under the
//! context the render was made in, `public_report` or `local_private`.
//!
//! Binding. The render command records, for the one current render, a
//! generation, the SHA-256 the desktop itself computes of the canonical HTML,
//! the render's export context, whether its gates blocked it, the report
//! source it was rendered for and, for a render that is not blocked, the
//! canonical HTML itself ([`RenderCustody`], [`HeldDocument`]). The desktop
//! keeps the current render's document: one copy, shared by the record and
//! the routes reading it, never copied per request. A blocked render, a
//! metadata-only (content-withheld) render and one whose HTML does not hash to
//! the renderer's named hash hold no document. The held document's memory is
//! reserved from the native memory pool before it is copied, as the render
//! command's own interactive admission (`memory_pool` module docs: the host's
//! pressure state does not gate it; the pool, a known capacity and fresh
//! headroom do), counted as custody, and released when its last holder drops
//! it; a refused reservation records nothing and the render command refuses by
//! name ([`RENDER_CUSTODY_UNAVAILABLE`]). Retiring the render, a later render
//! request, a registered source advance and an activation drop the document
//! with the record. So custody holds one document at a time. A save shares it
//! only while it writes (never while its chooser is open) and a print only
//! while it derives the print view, so a retired document outlives its
//! retirement by at most that write or derivation, still reserved until then.
//!
//! A route request carries no document: it names the render by the SHA-256
//! the webview holds from the render's reply, and is admitted only when that
//! is the current render's. The context and the block are read from the
//! record, never from the request, whose context, intent and evidence must
//! agree with it. The saved file is exactly the held bytes; the printed
//! document is derived from them here (`derived_print_view`), so neither
//! route sends anything the webview composed.
//!
//! Currency (DEL-08-01 REQ-022). The workspace numbers each change of the
//! report's source (its model, result, runs, manifest, summary, rule check,
//! comparison choice, composition and context) synchronously, and sends each
//! number here ([`advance_report_source`]). This process keeps the highest
//! number it has registered for the workspace's current incarnation. A
//! workspace observes the current incarnation ([`observe_report_source`]) and
//! activates its own only conditionally ([`activate_report_source`] with the
//! incarnation it expects is current, compared under the custody mutex):
//! success retires the one before, a mismatch refuses and changes nothing, and
//! there is no unconditional activation. A message naming another incarnation
//! is refused. A render, save or print
//! request names the source number it was prepared for and is admitted only
//! when that is the registered one: an older number is retired, a newer one is
//! not yet registered and is refused rather than taken as authority. A
//! registered advance retires the current render and its print window.
//!
//! A render request is identified by its source and an ordinal its source
//! owner allocated when its preparation began ([`RenderRequest`]). It takes a
//! ticket at entry ([`RenderCustody::begin_render`]), which retires the
//! current render; its completion is recorded only while both its ticket (no
//! later render request and no retirement since) and its source number are
//! current. Entry also requires its ordinal to be above the incarnation's
//! entry watermark, the highest ordinal entered or retired before entry, so an
//! older request delivered late, or one retired before it entered, cannot
//! mint authority. The report panel retires a render it discards by naming
//! that request ([`retire_rendered_report`]): only the exact pending or
//! completed request is retired; one not yet entered raises the watermark
//! without touching the current render; another incarnation's, or an older
//! request's, retirement changes nothing.
//!
//! The save re-checks the record after its chooser, and then the atomic
//! writer renames the synced temporary file over the destination only inside
//! [`RenderCustody::publish_if_current`]: the custody mutex is held from that
//! currency check until the rename returns (never during the temporary file's
//! preparation or the directory sync), so a registered retirement or advance
//! either precedes the check, and the save is refused with the destination as
//! it was and the temporary file removed, or follows the rename, and the save
//! stands. The print serves its document only while the record is current and
//! hands it to the native print of its own window only inside
//! [`RenderCustody::hand_off_if_current`], ordered with retirements in the same
//! way; handing off establishes nothing about the dialog or a print job.
//!
//! A change made in the workspace is ordered here only once its number is
//! registered: until then an earlier request may still be admitted and
//! published, and the workspace reports that outcome as such. No time bound
//! is promised between the change and its registration.
//!
//! Lock order. `rendered_report_print`'s open-window lock may be held while
//! this custody mutex is taken (its page load and protocol read currency);
//! this mutex is never held while that lock, or the solve-job registry's
//! `jobs` mutex, is taken. Work that will hold job custody across a
//! publication (issue 366) takes this mutex first and the registry's second.
//! A held document dropped under this mutex (a retirement) releases its
//! reservation by taking the memory pool's lock, which takes no other lock.
//!
//! A user-released report has no route here: the render command refuses its
//! input (`refuse_user_release_render`), and a user release lets out only its
//! package (DEL-08-01 REQ-019, "Admission"); a record under any context but
//! the two above is refused by name.
use crate::atomic_report_package_save::{self as atomic, AtomicSaveError, AtomicSaveFormat, AtomicSaveOutcome, GuardedSaveError};
use crate::command_ingress::{self, respond, ArgsDecoding, CommandError};
use crate::memory_pool::{Lease, LeasePurpose, MemoryPool};
use crate::solve_jobs::SolveJobRegistry;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::thread::ThreadId;
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
/// The render the request names (by its SHA-256) is not the current render.
pub(crate) const UNBOUND: &str = "REPORT-ROUTE-HTML-UNBOUND";
/// The current render's context is not one this route admits.
pub(crate) const CONTEXT_NOT_ADMITTED: &str = "REPORT-ROUTE-CONTEXT-NOT-ADMITTED";
/// The request names another context than the current render's.
pub(crate) const CONTEXT_MISMATCH: &str = "REPORT-ROUTE-CONTEXT-MISMATCH";
/// A local-private render leaves only with the panel's explicit intent.
pub(crate) const INTENT_REQUIRED: &str = "REPORT-ROUTE-LOCAL-PRIVATE-INTENT-REQUIRED";
/// The controlled-export evidence does not admit this route.
pub(crate) const EVIDENCE_REFUSED: &str = "REPORT-ROUTE-EVIDENCE-REFUSED";
/// The render was retired, replaced or its source advanced before the file was published.
pub(crate) const RETIRED_DURING_SAVE: &str = "REPORT-HTML-SAVE-RETIRED";
/// The chooser returned a destination that is not a path.
pub(crate) const CHOOSER_FAILED: &str = "REPORT-HTML-SAVE-CHOOSER-FAILED";
/// The route's worker did not complete.
pub(crate) const WORKER_FAILED: &str = "REPORT-ROUTE-WORKER-FAILED";
/// No workspace incarnation is active in this process.
pub(crate) const SOURCE_NOT_ACTIVE: &str = "REPORT-SOURCE-NOT-ACTIVE";
/// The message names a workspace incarnation that is not the current one.
pub(crate) const SOURCE_INCARNATION_RETIRED: &str = "REPORT-SOURCE-INCARNATION-RETIRED";
/// The request names a source number this process has not registered yet.
pub(crate) const SOURCE_NOT_REGISTERED: &str = "REPORT-SOURCE-NOT-REGISTERED";
/// The request names a source number older than the registered one.
pub(crate) const SOURCE_RETIRED: &str = "REPORT-SOURCE-RETIRED";
/// A conditional activation expected another current incarnation; nothing changed.
pub(crate) const SOURCE_ACTIVATION_CONFLICT: &str = "REPORT-SOURCE-ACTIVATION-CONFLICT";
/// A later render request or a retirement superseded this render's ticket.
pub(crate) const RENDER_SUPERSEDED: &str = "REPORT-RENDER-SUPERSEDED";
/// The memory pool refused the reservation for keeping the render's document:
/// the render is not recorded.
pub(crate) const RENDER_CUSTODY_UNAVAILABLE: &str = "REPORT-RENDER-CUSTODY-UNAVAILABLE";

/// The canonical HTML file (`.html`) under the desktop's atomic writer.
pub(crate) const REPORT_HTML_FORMAT: AtomicSaveFormat = AtomicSaveFormat {
    extension: "html",
    code_prefix: "REPORT-HTML-SAVE",
    temp_prefix: ".openpipestress-report-html-",
    log_label: "report-html",
};

/// A report source number within a workspace incarnation.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct SourceMark {
    pub(crate) incarnation: u64,
    pub(crate) sequence: u64,
}

/// The current render as the desktop recorded it. `document` is the canonical
/// HTML it holds: `None` exactly when the render is blocked.
#[derive(Clone, Debug)]
pub(crate) struct RenderRecord {
    pub(crate) generation: u64,
    pub(crate) sha256_hex: String,
    pub(crate) export_context: Option<String>,
    pub(crate) blocked: bool,
    pub(crate) source: SourceMark,
    pub(crate) document: Option<Arc<HeldDocument>>,
}

/// The canonical HTML of a render the desktop keeps (module docs), with the
/// memory pool's custody reservation covering it. Shared by `Arc` between the
/// record and the routes reading it; the bytes and their reservation are
/// released together when the last holder drops it.
pub(crate) struct HeldDocument {
    html: String,
    lease: Lease,
}

impl HeldDocument {
    /// The canonical HTML.
    pub(crate) fn html(&self) -> &str {
        &self.html
    }

    /// The bytes the reservation covers.
    #[cfg(test)]
    pub(crate) fn reserved_bytes(&self) -> u64 {
        self.lease.bytes()
    }
}

impl std::fmt::Debug for HeldDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HeldDocument").field("bytes", &self.html.len()).field("lease", &self.lease).finish()
    }
}

/// The pool bytes reserved for keeping `len` bytes of canonical HTML: the
/// allocation envelope of its exact buffer and of the shared record that owns
/// it (`ingress::allocation_envelope`).
pub(crate) fn held_document_bytes(len: usize) -> u64 {
    let envelope = |size: usize| crate::ingress::allocation_envelope(size as u64).unwrap_or(u64::MAX);
    // The `Arc`'s allocation: its two counts and the document.
    envelope(len).saturating_add(envelope(2 * std::mem::size_of::<usize>() + std::mem::size_of::<HeldDocument>()))
}

/// Reserves the memory for `html` from `pool` and then copies it into a held
/// document. The reservation is the render command's own: an interactive
/// admission (`memory_pool` module docs), which the host's pressure state does
/// not gate, re-sampled while refused only for headroom, then counted as
/// custody. A refusal copies nothing.
fn hold_document(pool: &Arc<MemoryPool>, html: &str) -> Result<Arc<HeldDocument>, RouteError> {
    let mut lease = pool.acquire_interactive_resampling(held_document_bytes(html.len())).map_err(|refusal| {
        RouteError::at(
            RENDER_CUSTODY_UNAVAILABLE,
            "render",
            format!(
                "The desktop could not reserve memory to keep this render's document ({}); the render was not recorded and nothing can be saved or printed from it. Render the report again.",
                serde_json::to_string(&refusal).unwrap_or_default()
            ),
        )
    })?;
    lease.set_purpose(LeasePurpose::Custody);
    Ok(Arc::new(HeldDocument { html: html.to_owned(), lease }))
}

/// A render request's identity: the registered source it was prepared for and
/// the ordinal its source owner allocated, monotonically, when its preparation
/// began. Two renders of the same source have different ordinals.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct RenderRequest {
    pub(crate) source: SourceMark,
    pub(crate) ordinal: u64,
}

/// A render request's ticket, taken at entry ([`RenderCustody::begin_render`]).
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct RenderTicket {
    id: u64,
    source: SourceMark,
}

struct Slot {
    generation: u64,
    current: Option<RenderRecord>,
    /// The registered source: incarnation 0 is none.
    source: SourceMark,
    last_incarnation: u64,
    /// The latest render ticket issued; a retirement advances it.
    ticket: u64,
    /// The request the current ticket was issued to, pending or completed;
    /// cleared by every retirement.
    active: Option<RenderRequest>,
    /// The highest request ordinal entered, or retired before entry, in the
    /// current incarnation; an entry at or below it is refused. Reset by
    /// activation. It is not the active request's identity.
    entry_watermark: u64,
    /// A print hand-off is in progress ([`RenderCustody::hand_off_if_current`]).
    handoff: bool,
}

/// The one current render's record and the registered report source (see the
/// module docs).
pub(crate) struct RenderCustody {
    slot: Mutex<Slot>,
    handoff_done: Condvar,
}

/// The desktop's custody of its current render.
pub(crate) static CUSTODY: RenderCustody = RenderCustody::new();

fn source_error(code: &str, message: impl Into<String>) -> RouteError {
    RouteError::at(code, "source", message)
}

impl RenderCustody {
    pub(crate) const fn new() -> Self {
        Self {
            slot: Mutex::new(Slot { generation: 0, current: None, source: SourceMark { incarnation: 0, sequence: 0 }, last_incarnation: 0, ticket: 0, active: None, entry_watermark: 0, handoff: false }),
            handoff_done: Condvar::new(),
        }
    }

    /// For reading: admission and currency checks may run during a print hand-off.
    fn lock(&self) -> MutexGuard<'_, Slot> {
        self.slot.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// For any change: waits until no print hand-off is in progress.
    fn lock_for_change(&self) -> MutexGuard<'_, Slot> {
        let mut slot = self.lock();
        while slot.handoff {
            slot = self.handoff_done.wait(slot).unwrap_or_else(PoisonError::into_inner);
        }
        slot
    }

    fn check_source(slot: &Slot, mark: SourceMark) -> Result<(), RouteError> {
        if slot.source.incarnation == 0 {
            return Err(source_error(SOURCE_NOT_ACTIVE, "No workspace is registered with the desktop for report output yet."));
        }
        if mark.incarnation != slot.source.incarnation {
            return Err(source_error(SOURCE_INCARNATION_RETIRED, "The request belongs to a workspace incarnation that is no longer current."));
        }
        if mark.sequence > slot.source.sequence {
            return Err(source_error(SOURCE_NOT_REGISTERED, "The request names a report source the desktop has not registered yet; it is refused rather than taken as current."));
        }
        if mark.sequence < slot.source.sequence {
            return Err(source_error(SOURCE_RETIRED, "The report's source changed after this request was prepared: the request is retired."));
        }
        Ok(())
    }

    fn retire_locked(slot: &mut Slot) -> bool {
        slot.generation += 1;
        slot.ticket += 1;
        slot.active = None;
        slot.current.take().is_some()
    }

    /// The registered source as it is now: incarnation 0 when none is active.
    /// Observation authorises nothing and does not show that any activation
    /// still in flight has or has not been processed.
    pub(crate) fn observe(&self) -> SourceMark {
        self.lock().source
    }

    /// Conditional activation: starts a new workspace incarnation only when
    /// `expected` is the current one (0 when none is active), comparing under
    /// the custody mutex. On success it retires the current incarnation with
    /// its render, pending render tickets and source number, and returns the
    /// new, monotonically higher incarnation at source number 0. On mismatch it
    /// refuses by name and changes nothing. There is no unconditional
    /// activation: a confirmed activation from the observed incarnation fences
    /// every activation issued earlier with an expectation no greater than it.
    pub(crate) fn activate_if(&self, expected: u64) -> Result<SourceMark, RouteError> {
        let mut slot = self.lock_for_change();
        if slot.source.incarnation != expected {
            return Err(source_error(SOURCE_ACTIVATION_CONFLICT, "Another workspace incarnation became current first; observe it and activate again."));
        }
        slot.last_incarnation += 1;
        slot.source = SourceMark { incarnation: slot.last_incarnation, sequence: 0 };
        Self::retire_locked(&mut slot);
        slot.entry_watermark = 0;
        Ok(slot.source)
    }

    /// Tests' activation from the incarnation observed now.
    #[cfg(test)]
    pub(crate) fn activate(&self) -> SourceMark {
        self.activate_if(self.observe().incarnation).expect("no other activation intervenes in this test")
    }

    /// Registers a source number for the current incarnation: the highest
    /// number registered stands (an older or repeated one changes nothing). A
    /// higher number retires the current render and pending render requests'
    /// completions. Returns the registered source and whether a render was
    /// retired.
    pub(crate) fn advance(&self, mark: SourceMark) -> Result<(SourceMark, bool), RouteError> {
        let mut slot = self.lock_for_change();
        if slot.source.incarnation == 0 {
            return Err(source_error(SOURCE_NOT_ACTIVE, "No workspace is registered with the desktop for report output yet."));
        }
        if mark.incarnation != slot.source.incarnation {
            return Err(source_error(SOURCE_INCARNATION_RETIRED, "The source change belongs to a workspace incarnation that is no longer current."));
        }
        let mut retired = false;
        if mark.sequence > slot.source.sequence {
            slot.source.sequence = mark.sequence;
            retired = Self::retire_locked(&mut slot);
        }
        Ok((slot.source, retired))
    }

    /// A render request's entry: refused unless its source is the registered
    /// one and its ordinal is above the entry watermark (a later request has
    /// entered, or this one was retired before it entered); otherwise raises
    /// the watermark to its ordinal, retires the current render and any
    /// earlier pending request, makes it the active request and returns its
    /// ticket.
    pub(crate) fn begin_render(&self, request: RenderRequest) -> Result<RenderTicket, RouteError> {
        let mut slot = self.lock_for_change();
        Self::check_source(&slot, request.source)?;
        if request.ordinal <= slot.entry_watermark {
            return Err(RouteError::at(RENDER_SUPERSEDED, "render", "A later render request entered, or this request was retired before it entered; it is not rendered."));
        }
        slot.entry_watermark = request.ordinal;
        Self::retire_locked(&mut slot);
        slot.active = Some(request);
        Ok(RenderTicket { id: slot.ticket, source: request.source })
    }

    /// Records a completed render as the current one, only while its ticket
    /// and its source number are both current, and returns its generation.
    /// The SHA-256 is computed here; a render whose canonical HTML does not
    /// hash to the renderer's named hash, a blocked render and a
    /// metadata-only refusal are recorded as blocked and hold no document.
    /// Any other render's document is reserved from `pool` and copied before
    /// the custody mutex is taken (the admission may re-sample the host); a
    /// refused reservation records nothing ([`RENDER_CUSTODY_UNAVAILABLE`]),
    /// and a superseded completion drops the document it reserved.
    pub(crate) fn complete_in(
        &self,
        pool: &Arc<MemoryPool>,
        ticket: RenderTicket,
        export_context: Option<&str>,
        html: &str,
        renderer_sha256_hex: &str,
        export_blocked: bool,
        content_withheld: bool,
    ) -> Result<u64, RouteError> {
        let sha256_hex = sha256_hex(html.as_bytes());
        let blocked = export_blocked || content_withheld || sha256_hex != renderer_sha256_hex;
        let document = if blocked { None } else { Some(hold_document(pool, html)?) };
        let mut slot = self.lock_for_change();
        if ticket.id != slot.ticket {
            return Err(RouteError::at(RENDER_SUPERSEDED, "render", "A later render request or a retirement superseded this render; it is not recorded."));
        }
        Self::check_source(&slot, ticket.source)?;
        slot.generation += 1;
        let generation = slot.generation;
        slot.current = Some(RenderRecord { generation, sha256_hex, export_context: export_context.map(str::to_string), blocked, source: ticket.source, document });
        Ok(generation)
    }

    /// Tests' completion, its document reserved from a pool of its own on an
    /// ample scripted host.
    #[cfg(test)]
    pub(crate) fn complete(&self, ticket: RenderTicket, export_context: Option<&str>, html: &str, renderer_sha256_hex: &str, export_blocked: bool, content_withheld: bool) -> Result<u64, RouteError> {
        self.complete_in(&test_pool(), ticket, export_context, html, renderer_sha256_hex, export_blocked, content_withheld)
    }

    /// The current render's held document (tests).
    #[cfg(test)]
    pub(crate) fn held_document(&self) -> Option<Arc<HeldDocument>> {
        self.lock().current.as_ref().and_then(|record| record.document.clone())
    }

    /// The panel's retirement of the render request it names (module docs).
    /// Another incarnation's request changes nothing. The exact active request,
    /// pending or completed, is retired with its ticket. A request above the
    /// entry watermark has not entered: the watermark is raised to it, so it
    /// cannot enter later, and the current render, which is another request's,
    /// stays. Any other request (older than one that has entered) changes
    /// nothing. Returns whether a completed render was retired.
    pub(crate) fn retire_request(&self, request: RenderRequest) -> bool {
        let mut slot = self.lock_for_change();
        if slot.source.incarnation == 0 || request.source.incarnation != slot.source.incarnation {
            return false;
        }
        if slot.active == Some(request) {
            return Self::retire_locked(&mut slot);
        }
        if request.ordinal > slot.entry_watermark {
            slot.entry_watermark = request.ordinal;
        }
        false
    }

    /// Runs `read` with the custody mutex held for reading, giving it whether
    /// a generation is the current one; no change of custody can intervene
    /// while it runs, and it never waits for a print hand-off. The print
    /// record's installation runs here under the open-window lock (the
    /// established order); `read` must not take the custody mutex or perform
    /// window operations.
    pub(crate) fn read_currency<T>(&self, read: impl FnOnce(&dyn Fn(u64) -> bool) -> T) -> T {
        let slot = self.lock();
        let current = slot.current.as_ref().map(|record| record.generation);
        let answer = read(&|generation| current == Some(generation));
        drop(slot);
        answer
    }

    /// Whether the render of `generation` is still the current one.
    pub(crate) fn is_current(&self, generation: u64) -> bool {
        self.lock().current.as_ref().is_some_and(|record| record.generation == generation)
    }

    /// The held document of the render of `generation`, only while that
    /// render is current.
    pub(crate) fn document_if_current(&self, generation: u64) -> Option<Arc<HeldDocument>> {
        self.lock().current.as_ref().filter(|record| record.generation == generation).and_then(|record| record.document.clone())
    }

    /// Runs `publish` with the custody mutex held, only while the render of
    /// `generation` is current; refuses otherwise. The save's rename runs
    /// here (module docs).
    pub(crate) fn publish_if_current<T>(&self, generation: u64, publish: impl FnOnce() -> T) -> Result<T, RouteError> {
        let slot = self.lock_for_change();
        if !slot.current.as_ref().is_some_and(|record| record.generation == generation) {
            return Err(RouteError::at(
                RETIRED_DURING_SAVE,
                "currency",
                "The report was retired, rendered again or its source changed before the file was published: nothing was written to the destination.",
            ));
        }
        let published = publish();
        drop(slot);
        Ok(published)
    }

    /// Runs `hand_off` (the native print of the print window) only while the
    /// render of `generation` is current, with every change of custody held
    /// until it returns; reads are not held, so the window's own protocol and
    /// page-load reads cannot deadlock against it. Returns whether it ran.
    pub(crate) fn hand_off_if_current(&self, generation: u64, hand_off: impl FnOnce()) -> bool {
        {
            let mut slot = self.lock_for_change();
            if !slot.current.as_ref().is_some_and(|record| record.generation == generation) {
                return false;
            }
            slot.handoff = true;
        }
        struct Done<'a>(&'a RenderCustody);
        impl Drop for Done<'_> {
            fn drop(&mut self) {
                self.0.lock().handoff = false;
                self.0.handoff_done.notify_all();
            }
        }
        let _done = Done(self);
        hand_off();
        true
    }

    /// Admits a route request against the current record (module docs):
    /// refused by name when its source is not the registered one, when no
    /// render is current, when it is blocked, when the SHA-256 it names is not
    /// the current render's, when its context is not admitted or differs from
    /// the request's, when a local-private render lacks the explicit intent,
    /// and when the evidence does not admit the route. The admitted request
    /// shares the record's held document.
    pub(crate) fn admit(&self, request: &RouteRequest) -> Result<Admitted, RouteError> {
        let record = {
            let slot = self.lock();
            Self::check_source(&slot, request.source)?;
            slot.current.clone()
        }
        .ok_or_else(|| RouteError::refused(NOT_CURRENT, "No rendered report is current: it was retired or replaced. Render the report again."))?;
        if record.source != request.source {
            return Err(RouteError::refused(NOT_CURRENT, "No rendered report is current for this source. Render the report again."));
        }
        // A blocked render holds no document; neither does anything else, so
        // the two are one refusal.
        let document = match (&record.document, record.blocked) {
            (Some(document), false) => Arc::clone(document),
            _ => return Err(RouteError::refused(BLOCKED, "The current render is blocked by its export gates: it is neither saved nor printed.")),
        };
        if request.sha256_hex != record.sha256_hex {
            return Err(RouteError::refused(
                UNBOUND,
                "The render the request names is not the current one: the SHA-256 it names is not the one the desktop recorded for the current render. Render the report again.",
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
        Ok(Admitted { generation: record.generation, sha256_hex: record.sha256_hex.clone(), export_context: context.to_string(), document })
    }

    /// The registered source.
    #[cfg(test)]
    pub(crate) fn source(&self) -> SourceMark {
        self.lock().source
    }

    /// Tests' next request on `mark`, as a source owner allocating ordinals in
    /// order would make it.
    #[cfg(test)]
    pub(crate) fn next_request(&self, mark: SourceMark) -> RenderRequest {
        RenderRequest { source: mark, ordinal: self.lock().entry_watermark + 1 }
    }

    /// Tests' entry of the next request on `mark`.
    #[cfg(test)]
    pub(crate) fn begin_next(&self, mark: SourceMark) -> Result<RenderTicket, RouteError> {
        self.begin_render(self.next_request(mark))
    }

    /// Tests' retirement, by the panel, of the request that is active now (if any).
    #[cfg(test)]
    pub(crate) fn retire_active(&self) -> bool {
        let active = self.lock().active;
        active.is_some_and(|request| self.retire_request(request))
    }

    /// Tests' completed render under the registered source (activating an
    /// incarnation first when none is), as a render request and its
    /// completion would record it.
    #[cfg(test)]
    pub(crate) fn record(&self, export_context: Option<&str>, html: &str, renderer_sha256_hex: &str, export_blocked: bool, content_withheld: bool) -> u64 {
        self.record_in(&test_pool(), export_context, html, renderer_sha256_hex, export_blocked, content_withheld)
    }

    /// [`Self::record`] with the document reserved from `pool`.
    #[cfg(test)]
    pub(crate) fn record_in(&self, pool: &Arc<MemoryPool>, export_context: Option<&str>, html: &str, renderer_sha256_hex: &str, export_blocked: bool, content_withheld: bool) -> u64 {
        if self.source().incarnation == 0 {
            self.activate();
        }
        let ticket = self.begin_next(self.source()).expect("the registered source admits a render");
        self.complete_in(pool, ticket, export_context, html, renderer_sha256_hex, export_blocked, content_withheld).expect("an unsuperseded render completes")
    }
}

/// Tests' memory pool: a 16 GiB scripted host with ample headroom at normal
/// pressure, so a held document's reservation is admitted.
#[cfg(test)]
pub(crate) fn test_pool() -> Arc<MemoryPool> {
    MemoryPool::new(crate::memory_pool::test_host::ScriptedHost::normal())
}

/// The render tickets of the render commands running now, each held for its
/// worker thread with the memory pool its document is reserved from
/// ([`with_render_ticket`]).
static RENDER_TICKETS: Mutex<Vec<(ThreadId, RenderTicket, Arc<MemoryPool>)>> = Mutex::new(Vec::new());

fn render_tickets() -> MutexGuard<'static, Vec<(ThreadId, RenderTicket, Arc<MemoryPool>)>> {
    RENDER_TICKETS.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Takes the render ticket held for this thread, and its pool, if any.
fn take_render_ticket() -> Option<(RenderTicket, Arc<MemoryPool>)> {
    let thread = std::thread::current().id();
    let mut tickets = render_tickets();
    let at = tickets.iter().position(|(owner, _, _)| *owner == thread)?;
    let (_, ticket, pool) = tickets.swap_remove(at);
    Some((ticket, pool))
}

/// Runs the render command's worker with its ticket and the native memory
/// pool held for this thread, which [`record_render`] consumes; the ticket
/// goes with the worker when the render does not reach its record.
pub(crate) fn with_render_ticket<T>(ticket: RenderTicket, pool: Arc<MemoryPool>, render: impl FnOnce() -> T) -> T {
    struct Clear;
    impl Drop for Clear {
        fn drop(&mut self) {
            take_render_ticket();
        }
    }
    take_render_ticket();
    render_tickets().push((std::thread::current().id(), ticket, pool));
    let _clear = Clear;
    render()
}

/// Records a completed render in the desktop's custody under the render
/// command's ticket, keeping its canonical HTML unless it is blocked
/// ([`RenderCustody::complete_in`]), and closes a print window whose render it
/// replaces (the render's one call). A completion whose ticket or source
/// number is no longer current, or whose document's reservation the pool
/// refuses, is refused by name and records nothing. A render made without a
/// ticket (a test of the renderer itself) records nothing.
pub(crate) fn record_render(export_context: Option<&str>, html: &str, renderer_sha256_hex: &str, export_blocked: bool, content_withheld: bool) -> Result<(), String> {
    let Some((ticket, pool)) = take_render_ticket() else { return Ok(()) };
    match CUSTODY.complete_in(&pool, ticket, export_context, html, renderer_sha256_hex, export_blocked, content_withheld) {
        Ok(_) => {
            crate::rendered_report_print::close_retired();
            Ok(())
        }
        Err(error) => Err(format!("{}: {}", error.code, error.message)),
    }
}

/// A render request's entry (the render command's first step after its
/// arguments): takes its ticket and closes the print window of the render it
/// retires.
pub(crate) fn begin_render(request: RenderRequest) -> Result<RenderTicket, String> {
    let ticket = CUSTODY.begin_render(request).map_err(|error| format!("{}: {}", error.code, error.message))?;
    crate::rendered_report_print::close_retired();
    Ok(ticket)
}

/// The report panel retires a render request it issued and no longer shows
/// (it unmounted, or a render it had replaced completed): only that request,
/// [`RenderCustody::retire_request`], then the print window of a render now
/// retired. It runs off the main thread, as every change of custody does, so
/// that it can wait for a print hand-off.
#[tauri::command]
pub(crate) async fn retire_rendered_report(source: SourceMark, request: u64) {
    CUSTODY.retire_request(RenderRequest { source, ordinal: request });
    crate::rendered_report_print::close_retired();
}

/// The registered report source as it is now (module docs); it authorises
/// nothing by itself.
#[tauri::command]
pub(crate) async fn observe_report_source() -> SourceMark {
    CUSTODY.observe()
}

/// Starts the workspace's incarnation for report output, only when
/// `expected_incarnation` is the current one (module docs).
#[tauri::command]
pub(crate) async fn activate_report_source(expected_incarnation: u64) -> Result<SourceMark, RouteError> {
    let mark = CUSTODY.activate_if(expected_incarnation)?;
    crate::rendered_report_print::close_retired();
    Ok(mark)
}

/// Registers a report source number (module docs) and returns the registered
/// source.
#[tauri::command]
pub(crate) async fn advance_report_source(incarnation: u64, sequence: u64) -> Result<SourceMark, RouteError> {
    let (registered, _) = CUSTODY.advance(SourceMark { incarnation, sequence })?;
    // Judged against current custody whatever this advance retired, so an
    // earlier advance's delayed close is also completed here safely.
    crate::rendered_report_print::close_retired();
    Ok(registered)
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

/// A route request: the SHA-256 naming the render (as the render's reply gave
/// it), the context and intent the panel names, the controlled export's
/// evidence and the report source number the render was prepared for. It
/// carries no document.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RouteRequest {
    pub(crate) sha256_hex: String,
    pub(crate) export_context: String,
    pub(crate) explicit_local_private_intent: bool,
    pub(crate) evidence: RouteEvidence,
    pub(crate) source: SourceMark,
}

/// An admitted request's record, sharing the render's held document.
#[derive(Clone, Debug)]
pub(crate) struct Admitted {
    pub(crate) generation: u64,
    pub(crate) sha256_hex: String,
    pub(crate) export_context: String,
    pub(crate) document: Arc<HeldDocument>,
}

/// A route's refusal or failure. `destination` says what became of the
/// destination: `untouched` (nothing was chosen or written), `as_it_was`
/// (the write failed, or the save was retired, before the new file was
/// published) or `unknown` (the route's worker did not complete, so whether
/// the file was published is not known).
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
        sha256_hex: args.decode("sha256Hex")?,
        export_context: args.decode("exportContext")?,
        explicit_local_private_intent: args.decode("explicitLocalPrivateIntent")?,
        evidence: args.decode("evidence")?,
        source: args.decode("source")?,
    })
}

/// The save's publication gate: the atomic writer hands it the rename, which
/// runs only while the admitted render is current, under the custody mutex
/// (module docs).
pub(crate) struct PublicationGate<'a> {
    custody: &'a RenderCustody,
    generation: u64,
}

impl PublicationGate<'_> {
    pub(crate) fn run(self, rename: &mut dyn FnMut() -> io::Result<()>) -> Result<io::Result<()>, RouteError> {
        self.custody.publish_if_current(self.generation, rename)
    }
}

/// The HTML save after decoding: admission, the chooser, the currency check
/// and the write of exactly the held document's bytes, whose rename runs
/// through the publication gate. `choose` is the save chooser and `write` the
/// guarded atomic writer (seams in tests).
pub(crate) fn save_in(
    custody: &RenderCustody,
    request: RouteRequest,
    choose: impl FnOnce(&str) -> Option<Result<PathBuf, String>>,
    write: impl FnOnce(&Path, &[u8], PublicationGate<'_>) -> Result<AtomicSaveOutcome, GuardedSaveError<RouteError>>,
) -> Result<HtmlSaveReceipt, RouteError> {
    let admitted = custody.admit(&request)?;
    let suggested = suggested_file_name(&admitted);
    // The chooser may stay open for any time: the save holds no share of the
    // document across it, and takes the current render's again to write.
    let Admitted { generation, sha256_hex, export_context, document } = admitted;
    let byte_count = document.html().len();
    drop(document);
    let receipt = |outcome: &'static str, file_name: String, replaced_existing: bool, durability: &str| HtmlSaveReceipt {
        outcome,
        route_id: ROUTE_ID,
        export_context: export_context.clone(),
        sha256_hex: sha256_hex.clone(),
        byte_count,
        file_name,
        replaced_existing,
        durability: durability.to_string(),
    };
    let Some(chosen) = choose(&suggested) else {
        return Ok(receipt("cancelled", suggested.clone(), false, "not_applicable"));
    };
    // The chooser's own error text may name the path; it is not repeated.
    let path = chosen.map_err(|_| RouteError::at(CHOOSER_FAILED, "chooser", "The save chooser returned a destination that is not a file path; nothing was written."))?;
    let Some(document) = custody.document_if_current(generation) else {
        return Err(RouteError::at(
            RETIRED_DURING_SAVE,
            "currency",
            "The report was retired or rendered again while the chooser was open: it is no longer the report for the current result and model, and nothing was written.",
        ));
    };
    let gate = PublicationGate { custody, generation };
    let outcome = write(&path, document.html().as_bytes(), gate).map_err(|error| match error {
        GuardedSaveError::Save(error) => RouteError::write(error),
        // Retired before the rename: the destination is as it was and the owned
        // temporary file was removed, or its removal failure is named.
        GuardedSaveError::Refused { refusal, cleanup_code } => RouteError {
            destination: "as_it_was",
            cleanup_code,
            selected_basename: path.file_name().map(|name| name.to_string_lossy().into_owned()),
            ..refusal
        },
    })?;
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
        let outcome = save_in(&CUSTODY, request, |file_name| choose_report_html_save_path(&app, file_name), |path, bytes, gate| {
            atomic::atomic_save_bytes_as_guarded_with_io(&REPORT_HTML_FORMAT, path, bytes, &mut atomic::SystemAtomicSaveIo, |rename| gate.run(rename))
        });
        let response = respond(outcome.map_err(CommandError::Command));
        drop(lease);
        response
    })
    .await
    .unwrap_or_else(|_| {
        Err(CommandError::Command(RouteError {
            destination: "unknown",
            ..RouteError::at(WORKER_FAILED, "worker", "The save's worker did not complete: whether the file was written is not known. Check the file you selected.")
        }))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atomic_report_package_save::{test_support::atomic_save_test_lock, AtomicSaveIo, DirectorySyncResult};
    use std::fs::{self, File};
    use std::io;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    const HTML: &str = "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<title>Invented</title>\n</head>\n<body>\n<p>Invented report Ω</p>\n</body>\n</html>\n";

    fn evidence(context: &str, intent: bool) -> RouteEvidence {
        RouteEvidence { route_id: ROUTE_ID.to_string(), export_context: context.to_string(), explicit_local_private_intent: intent, blocked: false, blocking_count: 0 }
    }

    /// A request naming the render of `html` by its SHA-256, as the webview names the render
    /// whose reply gave it that HTML and hash; the request carries no document.
    fn request(html: &str, context: &str, intent: bool) -> RouteRequest {
        RouteRequest { sha256_hex: sha256_hex(html.as_bytes()), export_context: context.to_string(), explicit_local_private_intent: intent, evidence: evidence(context, intent), source: FIRST }
    }

    /// The source the tests' custody registers first: incarnation 1, number 0.
    const FIRST: SourceMark = SourceMark { incarnation: 1, sequence: 0 };

    /// The command's writer: the system's file operations under the publication gate.
    fn system_write(path: &Path, bytes: &[u8], gate: PublicationGate<'_>) -> Result<AtomicSaveOutcome, GuardedSaveError<RouteError>> {
        atomic::atomic_save_bytes_as_guarded_with_io(&REPORT_HTML_FORMAT, path, bytes, &mut atomic::SystemAtomicSaveIo, |rename| gate.run(rename))
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
        let error = save_in(custody, request, |_| panic!("no chooser on a refusal"), |_, _, _| panic!("no write on a refusal")).unwrap_err();
        assert_eq!((error.stage.as_str(), error.destination), ("admission", "untouched"), "{error:?}");
        error
    }

    #[test]
    fn no_current_render_is_refused_before_the_chooser() {
        let custody = RenderCustody::new();
        custody.activate();
        assert_eq!(refused(&custody, request(HTML, PUBLIC_REPORT, false)).code, NOT_CURRENT);
        let custody = rendered(Some(PUBLIC_REPORT));
        assert!(custody.retire_active());
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
    fn a_request_naming_another_render_is_refused() {
        let custody = rendered(Some(PUBLIC_REPORT));
        let substituted = HTML.replace("Invented report", "Substituted report");
        assert_eq!(refused(&custody, request(&substituted, PUBLIC_REPORT, false)).code, UNBOUND);
        // One byte more, as a trailing newline would be.
        assert_eq!(refused(&custody, request(&format!("{HTML}\n"), PUBLIC_REPORT, false)).code, UNBOUND);
        // A hash in another spelling, or none, names no render.
        let upper = RouteRequest { sha256_hex: sha256_hex(HTML.as_bytes()).to_uppercase(), ..request(HTML, PUBLIC_REPORT, false) };
        assert_eq!(refused(&custody, upper).code, UNBOUND);
        let empty = RouteRequest { sha256_hex: String::new(), ..request(HTML, PUBLIC_REPORT, false) };
        assert_eq!(refused(&custody, empty).code, UNBOUND);
        // An earlier render's hash is not the current one's.
        custody.record(Some(PUBLIC_REPORT), "<p>later</p>", &sha256_hex(b"<p>later</p>"), false, false);
        assert_eq!(refused(&custody, request(HTML, PUBLIC_REPORT, false)).code, UNBOUND);
    }

    // The held document (DEL-08-01 REQ-022: the desktop keeps the current render's document and a
    // route names it). Each pool here is the test's own, so its reserved bytes are exact.

    /// The pool's custody reservation, and the total, now.
    fn reserved(pool: &Arc<MemoryPool>) -> (u64, u64) {
        (pool.reserved(LeasePurpose::Custody), pool.reserved_total())
    }

    /// The writer receives exactly the held document's buffer, not a copy and nothing the
    /// request carried, and the file is those bytes; the receipt counts and hashes them.
    #[test]
    fn the_save_writes_exactly_the_held_document() {
        let _lock = atomic_save_test_lock();
        let pool = test_pool();
        let custody = RenderCustody::new();
        custody.record_in(&pool, Some(PUBLIC_REPORT), HTML, &sha256_hex(HTML.as_bytes()), false, false);
        let held = custody.held_document().expect("an unblocked render holds its document");
        assert_eq!(held.html(), HTML);
        assert_eq!(reserved(&pool), (held_document_bytes(HTML.len()), held_document_bytes(HTML.len())));
        assert_eq!(held.reserved_bytes(), held_document_bytes(HTML.len()));
        let dir = scratch("held");
        let mut written_from = None;
        let receipt = save_in(&custody, request(HTML, PUBLIC_REPORT, false), |name| Some(Ok(dir.join(name))), |path, bytes, gate| {
            written_from = Some(bytes.as_ptr());
            system_write(path, bytes, gate)
        })
        .unwrap();
        assert_eq!(written_from, Some(held.html().as_ptr()), "the writer was handed the held buffer itself");
        let written = fs::read(dir.join(&receipt.file_name)).unwrap();
        assert_eq!(written, HTML.as_bytes());
        assert_eq!((receipt.byte_count, receipt.sha256_hex.as_str()), (HTML.len(), sha256_hex(&written).as_str()));
        fs::remove_dir_all(dir).unwrap();
    }

    /// A blocked render, a withheld (metadata-only) render and one whose HTML does not hash to
    /// the renderer's named hash hold no document and reserve nothing; a save naming them is
    /// refused before the chooser.
    #[test]
    fn a_blocked_withheld_or_mismatched_render_holds_no_document() {
        let sha = sha256_hex(HTML.as_bytes());
        for (blocked, withheld, named) in [(true, false, sha.clone()), (false, true, sha.clone()), (false, false, "0".repeat(64))] {
            let pool = test_pool();
            let custody = RenderCustody::new();
            custody.record_in(&pool, Some(PUBLIC_REPORT), HTML, &named, blocked, withheld);
            assert!(custody.held_document().is_none(), "blocked={blocked} withheld={withheld}");
            assert_eq!(reserved(&pool), (0, 0));
            assert_eq!(refused(&custody, request(HTML, PUBLIC_REPORT, false)).code, BLOCKED);
        }
    }

    /// Retiring the render, a later render request's entry, a registered source advance and an
    /// activation each drop the held document with the record and release its reservation.
    #[test]
    fn every_retirement_drops_the_document_and_releases_its_reservation() {
        for change in ["retire", "later request", "advance", "activation"] {
            let pool = test_pool();
            let custody = RenderCustody::new();
            custody.record_in(&pool, Some(PUBLIC_REPORT), HTML, &sha256_hex(HTML.as_bytes()), false, false);
            let held = Arc::downgrade(&custody.held_document().unwrap());
            assert_eq!(reserved(&pool).0, held_document_bytes(HTML.len()), "{change}");
            match change {
                "retire" => assert!(custody.retire_active()),
                "later request" => drop(custody.begin_next(FIRST).unwrap()),
                "advance" => assert_eq!(custody.advance(NEXT).unwrap(), (NEXT, true)),
                _ => drop(custody.activate()),
            }
            assert!(custody.held_document().is_none(), "{change}");
            assert!(held.upgrade().is_none(), "{change}: no holder keeps the document");
            assert_eq!(reserved(&pool), (0, 0), "{change}: the reservation returned");
        }
    }

    /// A save holds no share of the document while its chooser is open: a retirement there
    /// releases the reservation at once, and the save writes nothing.
    #[test]
    fn a_save_retired_during_the_chooser_writes_nothing_and_releases_the_document() {
        let pool = test_pool();
        let custody = RenderCustody::new();
        custody.record_in(&pool, Some(PUBLIC_REPORT), HTML, &sha256_hex(HTML.as_bytes()), false, false);
        let dir = scratch("retired-held");
        let error = save_in(&custody, request(HTML, PUBLIC_REPORT, false), |name| {
            assert_eq!(reserved(&pool).0, held_document_bytes(HTML.len()));
            assert!(custody.retire_active());
            assert_eq!(reserved(&pool), (0, 0), "nothing of a retired document stays reserved behind an open chooser");
            Some(Ok(dir.join(name)))
        }, |_, _, _| panic!("no write after retirement"))
        .unwrap_err();
        assert_eq!((error.code.as_str(), error.destination), (RETIRED_DURING_SAVE, "untouched"));
        assert_eq!(fs::read_dir(&dir).unwrap().count(), 0);
        assert_eq!(reserved(&pool), (0, 0));
        fs::remove_dir_all(dir).unwrap();
    }

    /// A refused reservation records nothing: the completion is refused by name, no render is
    /// current and nothing stays reserved. The pool's remaining bytes, an unknown capacity and
    /// insufficient headroom each refuse.
    #[test]
    fn a_refused_reservation_records_no_render() {
        use crate::memory_pool::{test_host::ScriptedHost, PressureLevel};
        for host in [
            ScriptedHost::with(Some(64), Some(64 << 30), PressureLevel::Normal),
            ScriptedHost::with(None, Some(64 << 30), PressureLevel::Normal),
            ScriptedHost::with(Some(16 << 30), Some(8), PressureLevel::Normal),
        ] {
            let pool = MemoryPool::with_timing(host, Duration::from_millis(1), Duration::ZERO, Duration::from_millis(1));
            let custody = RenderCustody::new();
            custody.activate();
            let ticket = custody.begin_next(FIRST).unwrap();
            let refusal = custody.complete_in(&pool, ticket, Some(PUBLIC_REPORT), HTML, &sha256_hex(HTML.as_bytes()), false, false).unwrap_err();
            assert_eq!((refusal.code.as_str(), refusal.stage.as_str()), (RENDER_CUSTODY_UNAVAILABLE, "render"));
            assert!(!refusal.message.contains("Invented"), "the refusal names nothing of the document");
            assert_eq!(refused(&custody, request(HTML, PUBLIC_REPORT, false)).code, NOT_CURRENT);
            assert_eq!(reserved(&pool), (0, 0));
        }
    }

    /// The document's reservation is the render command's own, an interactive admission: the
    /// host's pressure state does not gate it (`memory_pool` module docs), so a render is
    /// recorded and saved from under a pressure warning and under critical pressure alike.
    #[test]
    fn a_render_is_recorded_while_the_host_reports_memory_pressure() {
        use crate::memory_pool::{test_host::ScriptedHost, PressureLevel};
        for pressure in [PressureLevel::Warning, PressureLevel::Critical] {
            let pool = MemoryPool::new(ScriptedHost::with(Some(16 << 30), Some(64 << 30), pressure));
            let custody = RenderCustody::new();
            custody.record_in(&pool, Some(PUBLIC_REPORT), HTML, &sha256_hex(HTML.as_bytes()), false, false);
            assert_eq!(reserved(&pool).0, held_document_bytes(HTML.len()));
            assert!(custody.admit(&request(HTML, PUBLIC_REPORT, false)).is_ok());
        }
    }

    /// A reservation refused only for headroom is tried again on fresh samples within the
    /// pool's re-sampling window, and admitted at the first that passes.
    #[test]
    fn a_reservation_short_of_headroom_is_taken_again_on_a_fresh_sample() {
        use crate::memory_pool::{test_host::ScriptedHost, PressureLevel};
        let host = ScriptedHost::with(Some(16 << 30), Some(8), PressureLevel::Normal);
        let pool = MemoryPool::with_timing(host.clone(), Duration::from_millis(1), Duration::from_secs(30), Duration::from_millis(5));
        let refill = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(40));
            host.set(|script| script.headroom_bytes = Some(64 << 30));
        });
        let custody = RenderCustody::new();
        custody.record_in(&pool, Some(PUBLIC_REPORT), HTML, &sha256_hex(HTML.as_bytes()), false, false);
        refill.join().unwrap();
        assert_eq!(reserved(&pool).0, held_document_bytes(HTML.len()));
    }

    /// Custody holds one document at a time: each later render's reservation replaces the
    /// earlier one's, so the pool never carries more than one document's bytes for it.
    #[test]
    fn custody_holds_one_document_at_a_time() {
        let pool = test_pool();
        let custody = RenderCustody::new();
        for round in 0..4usize {
            let html = format!("{HTML}{}", "x".repeat(round * 1000));
            custody.record_in(&pool, Some(PUBLIC_REPORT), &html, &sha256_hex(html.as_bytes()), false, false);
            assert_eq!(reserved(&pool), (held_document_bytes(html.len()), held_document_bytes(html.len())), "round {round}");
        }
        assert!(custody.retire_active());
        assert_eq!(reserved(&pool), (0, 0));
    }

    /// A completion superseded after its document was reserved drops it.
    #[test]
    fn a_superseded_completion_releases_the_document_it_reserved() {
        let pool = test_pool();
        let custody = RenderCustody::new();
        custody.activate();
        let ticket = custody.begin_next(FIRST).unwrap();
        custody.retire_active();
        let late = custody.complete_in(&pool, ticket, Some(PUBLIC_REPORT), HTML, &sha256_hex(HTML.as_bytes()), false, false);
        assert_eq!(late.map_err(|error| error.code), Err(RENDER_SUPERSEDED.to_string()));
        assert_eq!(reserved(&pool), (0, 0));
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
            }, system_write)
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
        let receipt = save_in(&custody, request(HTML, PUBLIC_REPORT, false), |_| None, |_, _, _| panic!("no write after a cancel")).unwrap();
        assert_eq!((receipt.outcome, receipt.durability.as_str()), ("cancelled", "not_applicable"));
        let failed = save_in(&custody, request(HTML, PUBLIC_REPORT, false), |_| Some(Err("/private/path".to_string())), |_, _, _| panic!("no write")).unwrap_err();
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
                    custody.retire_active();
                }
                Some(Ok(dir.join(name)))
            }, |_, _, _| panic!("no write after retirement"))
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
        save_in(&custody, request(HTML, PUBLIC_REPORT, false), |_| Some(Ok(dir.join("report.html"))), |path, bytes, gate| {
            atomic::atomic_save_bytes_as_guarded_with_io(&REPORT_HTML_FORMAT, path, bytes, &mut Injected(fail), |rename| gate.run(rename))
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
        let error = save_in(&custody, request(HTML, PUBLIC_REPORT, false), |_| Some(Ok(dir.join("report.htm"))), system_write)
        .unwrap_err();
        assert_eq!((error.code.as_str(), error.destination), ("REPORT-HTML-SAVE-DESTINATION-EXTENSION", "as_it_was"));
        assert_eq!(fs::read_dir(&dir).unwrap().count(), 0);
        fs::remove_dir_all(dir).unwrap();
    }

    // Currency regressions (DEL-08-01 REQ-022, AC-011, as Agent 0 narrowed them under the owner's
    // grant: a source change the desktop application's native process has registered before
    // atomic publication, or before print submission, retires the earlier output). R1 and R2 are
    // the independent round-1 review's interleavings of PR 269; each drives the custody this
    // module records and the actual atomic writer, and none takes its expected value from the
    // implementation. They failed on the reviewed behaviour (d8d68c3f9) and pass with the repair.

    const REPORT_A: &str = "<!DOCTYPE html>\n<html><body><p>Invented report A</p></body></html>\n";
    const REPORT_B: &str = "<!DOCTYPE html>\n<html><body><p>Invented report B</p></body></html>\n";
    const NEXT: SourceMark = SourceMark { incarnation: 1, sequence: 1 };

    /// A render request's entry and completion under `mark`, as the render command makes them.
    fn render_at(custody: &RenderCustody, mark: SourceMark, html: &str) -> Result<u64, RouteError> {
        let ticket = custody.begin_next(mark)?;
        custody.complete(ticket, Some(LOCAL_PRIVATE), html, &sha256_hex(html.as_bytes()), false, false)
    }

    /// A render completed under the registered source (the custody activated first).
    fn complete(custody: &RenderCustody, html: &str) -> u64 {
        custody.record(Some(LOCAL_PRIVATE), html, &sha256_hex(html.as_bytes()), false, false)
    }

    fn at(html: &str, source: SourceMark) -> RouteRequest {
        RouteRequest { source, ..request(html, LOCAL_PRIVATE, true) }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum Change {
        Retire,
        Replace,
        Advance,
    }

    /// The system's file operations, except that after the temporary file is synced (the last
    /// step before publication) another thread makes `change` (the panel's retirement, a new
    /// render's completion or a registered source advance). It runs on its own thread and is
    /// waited for for a bounded time, so that the publication gate holding the custody mutex
    /// cannot deadlock the test; the order the writer then observes is recorded.
    struct ChangeAfterTempSync {
        custody: std::sync::Arc<RenderCustody>,
        change: Change,
        thread: Option<std::thread::JoinHandle<()>>,
        changed: std::sync::Arc<std::sync::atomic::AtomicBool>,
        renamed_after_change: Option<bool>,
    }

    fn make_change(custody: &RenderCustody, change: Change) {
        match change {
            Change::Retire => {
                custody.retire_active();
            }
            Change::Replace => {
                complete(custody, REPORT_B);
            }
            Change::Advance => {
                custody.advance(NEXT).unwrap();
            }
        }
    }

    impl AtomicSaveIo for ChangeAfterTempSync {
        fn write_all(&mut self, temp: &mut File, bytes: &[u8]) -> io::Result<()> {
            atomic::SystemAtomicSaveIo.write_all(temp, bytes)
        }
        fn flush(&mut self, temp: &mut File) -> io::Result<()> {
            atomic::SystemAtomicSaveIo.flush(temp)
        }
        fn sync_temp(&mut self, temp: &File) -> io::Result<()> {
            atomic::SystemAtomicSaveIo.sync_temp(temp)?;
            let (custody, change, changed) = (std::sync::Arc::clone(&self.custody), self.change, std::sync::Arc::clone(&self.changed));
            let (sent, received) = std::sync::mpsc::channel();
            self.thread = Some(std::thread::spawn(move || {
                make_change(&custody, change);
                changed.store(true, std::sync::atomic::Ordering::SeqCst);
                let _ = sent.send(());
            }));
            let _ = received.recv_timeout(std::time::Duration::from_secs(5));
            Ok(())
        }
        fn rename(&mut self, source: &Path, destination: &Path) -> io::Result<()> {
            self.renamed_after_change = Some(self.changed.load(std::sync::atomic::Ordering::SeqCst));
            atomic::SystemAtomicSaveIo.rename(source, destination)
        }
        fn cleanup_temp(&mut self, temp_path: &Path) -> io::Result<()> {
            atomic::SystemAtomicSaveIo.cleanup_temp(temp_path)
        }
    }

    /// R1. A retirement, a replacement or a registered source advance after the chooser's
    /// currency check and after the temporary file is synced, but before it is renamed over the
    /// destination, prevents publication: the earlier destination stays byte for byte, no
    /// temporary file is left beside it, and the save is refused by a named retirement.
    #[test]
    fn a_render_retired_or_replaced_after_the_temporary_file_sync_is_not_published() {
        let _lock = atomic_save_test_lock();
        for change in [Change::Retire, Change::Replace, Change::Advance] {
            let custody = std::sync::Arc::new(RenderCustody::new());
            complete(&custody, REPORT_A);
            let dir = scratch("retired-before-rename");
            let destination = dir.join("report.html");
            let earlier = b"earlier destination file, invented".to_vec();
            fs::write(&destination, &earlier).unwrap();
            let mut io = ChangeAfterTempSync {
                custody: std::sync::Arc::clone(&custody),
                change,
                thread: None,
                changed: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
                renamed_after_change: None,
            };
            let outcome = save_in(&custody, request(REPORT_A, LOCAL_PRIVATE, true), |_| Some(Ok(destination.clone())), |path, bytes, gate| {
                atomic::atomic_save_bytes_as_guarded_with_io(&REPORT_HTML_FORMAT, path, bytes, &mut io, |rename| gate.run(rename))
            });
            io.thread.take().expect("the writer reached the temporary-file sync").join().unwrap();
            let left: Vec<String> = fs::read_dir(&dir).unwrap().map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned()).collect();
            let found = fs::read(&destination).unwrap();
            let what = format!(
                "{change:?} after the temporary-file sync and before publication: rename reached = {:?}, outcome = {:?}, destination now = {:?}, directory = {left:?}",
                io.renamed_after_change,
                outcome.as_ref().map(|receipt| (receipt.outcome, receipt.durability.as_str())).map_err(|error| error.code.as_str()),
                String::from_utf8_lossy(&found)
            );
            assert_eq!(io.renamed_after_change, None, "the retired report is not renamed over the destination; {what}");
            assert_eq!(found, earlier, "the earlier destination stays byte for byte; {what}");
            assert_eq!(left, vec!["report.html".to_string()], "no temporary file is left beside the destination; {what}");
            let error = outcome.expect_err(&format!("a retired save is refused, not saved; {what}"));
            assert_eq!((error.code.as_str(), error.stage.as_str(), error.destination, error.cleanup_code.as_deref()), (RETIRED_DURING_SAVE, "currency", "as_it_was", None), "{what}");
            assert_eq!(error.selected_basename.as_deref(), Some("report.html"));
            fs::remove_dir_all(dir).unwrap();
        }
    }

    /// The system's file operations, except that the rename starts another thread's change and
    /// holds the rename for a while before running it: the change must wait for the rename's
    /// return (the custody mutex is held from the currency check through it).
    struct ChangeDuringRename {
        custody: std::sync::Arc<RenderCustody>,
        change: Change,
        thread: Option<std::thread::JoinHandle<()>>,
        changed: std::sync::Arc<std::sync::atomic::AtomicBool>,
        changed_before_rename_returned: Option<bool>,
    }

    impl AtomicSaveIo for ChangeDuringRename {
        fn write_all(&mut self, temp: &mut File, bytes: &[u8]) -> io::Result<()> {
            atomic::SystemAtomicSaveIo.write_all(temp, bytes)
        }
        fn flush(&mut self, temp: &mut File) -> io::Result<()> {
            atomic::SystemAtomicSaveIo.flush(temp)
        }
        fn sync_temp(&mut self, temp: &File) -> io::Result<()> {
            atomic::SystemAtomicSaveIo.sync_temp(temp)
        }
        fn rename(&mut self, source: &Path, destination: &Path) -> io::Result<()> {
            let (custody, change, changed) = (std::sync::Arc::clone(&self.custody), self.change, std::sync::Arc::clone(&self.changed));
            self.thread = Some(std::thread::spawn(move || {
                make_change(&custody, change);
                changed.store(true, std::sync::atomic::Ordering::SeqCst);
            }));
            std::thread::sleep(std::time::Duration::from_millis(200));
            let renamed = atomic::SystemAtomicSaveIo.rename(source, destination);
            self.changed_before_rename_returned = Some(self.changed.load(std::sync::atomic::Ordering::SeqCst));
            renamed
        }
        fn cleanup_temp(&mut self, temp_path: &Path) -> io::Result<()> {
            atomic::SystemAtomicSaveIo.cleanup_temp(temp_path)
        }
    }

    /// R1, the other order. A change that arrives while the rename runs waits for its return:
    /// the publication came first and stands as saved, and the later change retires the render
    /// without undoing the file. Directory durability is reported separately, after the gate.
    #[test]
    fn a_retirement_after_publication_waits_for_the_rename_and_does_not_undo_the_save() {
        let _lock = atomic_save_test_lock();
        for change in [Change::Retire, Change::Advance] {
            let custody = std::sync::Arc::new(RenderCustody::new());
            let generation = complete(&custody, REPORT_A);
            let dir = scratch("retired-after-rename");
            let destination = dir.join("report.html");
            let mut io = ChangeDuringRename {
                custody: std::sync::Arc::clone(&custody),
                change,
                thread: None,
                changed: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
                changed_before_rename_returned: None,
            };
            let receipt = save_in(&custody, request(REPORT_A, LOCAL_PRIVATE, true), |_| Some(Ok(destination.clone())), |path, bytes, gate| {
                atomic::atomic_save_bytes_as_guarded_with_io(&REPORT_HTML_FORMAT, path, bytes, &mut io, |rename| gate.run(rename))
            })
            .unwrap();
            io.thread.take().unwrap().join().unwrap();
            assert_eq!(io.changed_before_rename_returned, Some(false), "{change:?} waited for the rename's return");
            assert_eq!((receipt.outcome, receipt.durability.as_str()), ("saved", "durable"));
            assert_eq!(fs::read(&destination).unwrap(), REPORT_A.as_bytes());
            assert!(!custody.is_current(generation), "the later {change:?} retired the render");
            fs::remove_dir_all(dir).unwrap();
        }
    }

    /// The delayed-registration case Agent 0 permits: the workspace changed, but its source
    /// advance had not been registered here when the save published. The earlier selected
    /// report is saved (the workspace reports that outcome as such); the advance registered
    /// afterwards retires the render and leaves the file as saved.
    #[test]
    fn an_advance_registered_after_publication_leaves_the_earlier_report_saved() {
        let _lock = atomic_save_test_lock();
        let custody = RenderCustody::new();
        let generation = complete(&custody, REPORT_A);
        let dir = scratch("advance-after-publication");
        let destination = dir.join("report.html");
        // The workspace's change to number 1 is in transit: not registered yet.
        let receipt = save_in(&custody, request(REPORT_A, LOCAL_PRIVATE, true), |_| Some(Ok(destination.clone())), system_write).unwrap();
        assert_eq!(receipt.outcome, "saved");
        assert_eq!(custody.advance(NEXT).unwrap(), (NEXT, true));
        assert!(!custody.is_current(generation));
        assert_eq!(fs::read(&destination).unwrap(), REPORT_A.as_bytes());
        // With the advance registered first, the same request is refused before any chooser.
        let refusal = save_in(&custody, request(REPORT_A, LOCAL_PRIVATE, true), |_| panic!("no chooser"), |_, _, _| panic!("no write")).unwrap_err();
        assert_eq!((refusal.code.as_str(), refusal.stage.as_str(), refusal.destination), (SOURCE_RETIRED, "source", "untouched"));
        fs::remove_dir_all(dir).unwrap();
    }

    /// R2. Two render requests overlap: A enters, B enters, B completes and is shown and its
    /// print window opened, then A's worker completes late. B stays the current render,
    /// admissible to save and print and its window served and printed; A's completion is refused
    /// by name and the obsolete A is not admitted.
    #[test]
    fn a_late_completion_of_an_earlier_render_request_does_not_replace_the_newer_render() {
        use crate::rendered_report_print::{admit_print_in, on_load_in, served_document_in, OnLoad, OpenPrint};
        let custody = RenderCustody::new();
        custody.activate();
        let ticket_a = custody.begin_next(FIRST).unwrap(); // request A enters; its worker is still rendering
        let ticket_b = custody.begin_next(FIRST).unwrap(); // request B enters
        custody.complete(ticket_b, Some(LOCAL_PRIVATE), REPORT_B, &sha256_hex(REPORT_B.as_bytes()), false, false).unwrap();
        let document = admit_print_in(&custody, &at(REPORT_B, FIRST)).expect("B is current once it completes");
        let path = format!("/{}", document.print_sha256_hex);
        let open = std::sync::Mutex::new(Some(OpenPrint { label: "report-print-1".to_string(), document: std::sync::Arc::new(document), served: false, printed: false }));
        let late = custody.complete(ticket_a, Some(LOCAL_PRIVATE), REPORT_A, &sha256_hex(REPORT_A.as_bytes()), false, false);
        let observed = (
            late.map_err(|error| error.code),
            custody.admit(&at(REPORT_B, FIRST)).is_ok(),
            custody.admit(&at(REPORT_A, FIRST)).is_ok(),
            served_document_in(&open, &custody, "report-print-1", &path).is_some(),
            match on_load_in(&open, &custody, "report-print-1") {
                OnLoad::Print(..) => "print",
                OnLoad::Close => "close",
                OnLoad::Nothing => "nothing",
            },
        );
        assert_eq!(
            observed,
            (Err(RENDER_SUPERSEDED.to_string()), true, false, true, "print"),
            "(A's late completion, shown B admitted, obsolete A admitted, B's print window served, B's window printed)"
        );
    }

    /// R2. A completion that arrives after the render was explicitly retired (the panel discarded
    /// it or unmounted while the request was in flight) does not make that render current again.
    #[test]
    fn a_completion_after_explicit_retirement_does_not_make_the_render_current_again() {
        let custody = RenderCustody::new();
        custody.activate();
        let a = custody.next_request(FIRST);
        let ticket = custody.begin_render(a).unwrap(); // request A enters
        assert!(!custody.retire_request(a), "A was pending, not completed"); // the panel retires A (`retire_rendered_report`) while it is in flight
        let late = custody.complete(ticket, Some(LOCAL_PRIVATE), REPORT_A, &sha256_hex(REPORT_A.as_bytes()), false, false);
        assert_eq!(late.map_err(|error| error.code), Err(RENDER_SUPERSEDED.to_string()));
        assert_eq!(custody.admit(&at(REPORT_A, FIRST)).unwrap_err().code, NOT_CURRENT, "a render retired while in flight is refused as not current");
    }

    /// A registered source advance retires the earlier source's requests as well as completions:
    /// a render request that arrives after the advance is refused at entry, and one pending when
    /// the advance arrived (no response yet accepted by the panel) is not recorded on completion.
    #[test]
    fn a_registered_source_advance_retires_old_requests_and_pending_completions() {
        let custody = RenderCustody::new();
        custody.activate();
        let pending = custody.begin_next(FIRST).unwrap();
        assert_eq!(custody.advance(NEXT).unwrap(), (NEXT, false));
        let late = custody.complete(pending, Some(LOCAL_PRIVATE), REPORT_A, &sha256_hex(REPORT_A.as_bytes()), false, false);
        assert_eq!(late.map_err(|error| error.code), Err(RENDER_SUPERSEDED.to_string()), "the advance superseded the pending ticket");
        assert_eq!(custody.admit(&at(REPORT_A, FIRST)).unwrap_err().code, SOURCE_RETIRED);
        // An old request delivered after the advance is refused at entry.
        assert_eq!(custody.begin_next(FIRST).unwrap_err().code, SOURCE_RETIRED);
        // A request on the registered source renders and is admitted.
        render_at(&custody, NEXT, REPORT_B).unwrap();
        assert!(custody.admit(&at(REPORT_B, NEXT)).is_ok());
    }

    /// A request ahead of registration is refused rather than taken as authority: the native side
    /// has not yet registered its source number.
    #[test]
    fn a_request_ahead_of_registration_is_refused() {
        let custody = RenderCustody::new();
        assert_eq!(custody.begin_next(FIRST).unwrap_err().code, SOURCE_NOT_ACTIVE, "no incarnation yet");
        custody.activate();
        assert_eq!(custody.begin_next(NEXT).unwrap_err().code, SOURCE_NOT_REGISTERED);
        complete(&custody, REPORT_A);
        let refusal = save_in(&custody, at(REPORT_A, NEXT), |_| panic!("no chooser"), |_, _, _| panic!("no write")).unwrap_err();
        assert_eq!((refusal.code.as_str(), refusal.destination), (SOURCE_NOT_REGISTERED, "untouched"));
        assert_eq!(custody.source(), FIRST, "a refused request registers nothing");
    }

    /// Advances arrive out of order and repeated: the highest registered number stands, and an
    /// older or repeated one neither lowers it nor retires a render made at the higher number.
    #[test]
    fn out_of_order_and_repeated_advances_keep_the_highest_registered_number() {
        let custody = RenderCustody::new();
        custody.activate();
        let five = SourceMark { incarnation: 1, sequence: 5 };
        let three = SourceMark { incarnation: 1, sequence: 3 };
        assert_eq!(custody.advance(five).unwrap().0, five);
        let generation = render_at(&custody, five, REPORT_A).unwrap();
        assert_eq!(custody.advance(three).unwrap(), (five, false));
        assert_eq!(custody.advance(five).unwrap(), (five, false));
        assert!(custody.is_current(generation));
        assert_eq!(custody.begin_next(three).unwrap_err().code, SOURCE_RETIRED);
    }

    /// Activating an incarnation retires its predecessor: the earlier incarnation's render and
    /// pending ticket are retired, and its delayed advances and requests are refused rather than
    /// reviving it.
    #[test]
    fn activation_retires_the_previous_incarnation_and_its_delayed_messages_cannot_revive_it() {
        let custody = RenderCustody::new();
        assert_eq!(custody.activate(), FIRST);
        let generation = complete(&custody, REPORT_A);
        let pending = custody.begin_next(FIRST).unwrap();
        let second = custody.activate();
        assert_eq!(second, SourceMark { incarnation: 2, sequence: 0 });
        assert!(!custody.is_current(generation));
        let late = custody.complete(pending, Some(LOCAL_PRIVATE), REPORT_A, &sha256_hex(REPORT_A.as_bytes()), false, false);
        assert_eq!(late.map_err(|error| error.code), Err(RENDER_SUPERSEDED.to_string()));
        assert_eq!(custody.advance(SourceMark { incarnation: 1, sequence: 9 }).unwrap_err().code, SOURCE_INCARNATION_RETIRED);
        assert_eq!(custody.begin_next(FIRST).unwrap_err().code, SOURCE_INCARNATION_RETIRED);
        assert_eq!(custody.source(), second, "the delayed message registered nothing");
        render_at(&custody, second, REPORT_B).unwrap();
        assert!(custody.admit(&at(REPORT_B, second)).is_ok());
    }

    /// The render command's ticket reaches its completion on its own worker thread only within
    /// its scope: another thread sees none, and nothing is left after the scope.
    #[test]
    fn a_render_ticket_is_consumed_only_within_its_worker_scope() {
        let ticket = || RenderTicket { id: u64::MAX, source: SourceMark { incarnation: u64::MAX, sequence: 7 } };
        let held = |id: u64| render_tickets().iter().any(|(owner, ticket, _)| *owner == std::thread::current().id() && ticket.id == id);
        assert!(!held(u64::MAX));
        with_render_ticket(ticket(), test_pool(), || {
            assert!(held(u64::MAX));
            std::thread::spawn(|| assert!(take_render_ticket().map_or(true, |(found, _)| found.source.incarnation != u64::MAX))).join().unwrap();
            assert!(held(u64::MAX));
        });
        assert!(!held(u64::MAX));
        let pool = test_pool();
        with_render_ticket(ticket(), Arc::clone(&pool), || {
            assert_eq!(take_render_ticket().map(|(found, with)| (found.source.sequence, Arc::ptr_eq(&with, &pool))), Some((7, true)), "the ticket travels with its pool");
        });
    }

    // F2 (round-two review): conditional activation across workspace lifetimes. A replaced
    // owner's activation request may still be queued or processed with its reply never
    // delivered; the replacement's confirmed conditional activation fences it either way.

    /// The comparison happens under the custody mutex: a match retires the current incarnation
    /// with its render and tickets and advances monotonically; a mismatch refuses and changes
    /// nothing.
    #[test]
    fn conditional_activation_compares_under_the_mutex_and_a_mismatch_changes_nothing() {
        let custody = RenderCustody::new();
        assert_eq!(custody.observe(), SourceMark { incarnation: 0, sequence: 0 });
        assert_eq!(custody.activate_if(0).unwrap(), FIRST);
        custody.advance(NEXT).unwrap();
        let generation = render_at(&custody, NEXT, REPORT_A).unwrap();
        let pending = custody.begin_next(NEXT).unwrap();
        let conflict = custody.activate_if(0).unwrap_err();
        assert_eq!((conflict.code.as_str(), conflict.stage.as_str()), (SOURCE_ACTIVATION_CONFLICT, "source"));
        assert_eq!(custody.observe(), NEXT, "a refused activation changes nothing");
        assert_eq!(custody.complete(pending, Some(LOCAL_PRIVATE), REPORT_B, &sha256_hex(REPORT_B.as_bytes()), false, false).map_err(|e| e.code), Ok(generation + 2));
        let second = custody.activate_if(1).unwrap();
        assert_eq!(second, SourceMark { incarnation: 2, sequence: 0 });
        assert!(custody.admit(&at(REPORT_B, NEXT)).is_err(), "the match retired the former incarnation's render");
    }

    /// Both crossing orders of an old owner's activation (expecting incarnation 1) and its
    /// replacement's, which observed 1 too.
    #[test]
    fn a_confirmed_conditional_activation_fences_an_earlier_one_in_either_order() {
        // The replacement wins; the old request, processed late, conflicts and changes nothing.
        let custody = RenderCustody::new();
        custody.activate_if(0).unwrap();
        let observed = custody.observe().incarnation;
        assert_eq!(custody.activate_if(observed).unwrap().incarnation, 2);
        assert_eq!(custody.activate_if(1).unwrap_err().code, SOURCE_ACTIVATION_CONFLICT);
        assert_eq!(custody.observe().incarnation, 2);
        // The old request wins first and its reply is never delivered: the replacement's
        // conditional call conflicts; the live owner observes and activates again, which retires
        // the old request's incarnation whatever its owner believes.
        let custody = RenderCustody::new();
        custody.activate_if(0).unwrap();
        let replacement_expects = custody.observe().incarnation;
        let old = custody.activate_if(1).unwrap(); // processed; reply lost
        assert_eq!(custody.activate_if(replacement_expects).unwrap_err().code, SOURCE_ACTIVATION_CONFLICT);
        let live = custody.activate_if(custody.observe().incarnation).unwrap();
        assert_eq!((old.incarnation, live.incarnation), (2, 3));
        assert_eq!(custody.advance(SourceMark { incarnation: old.incarnation, sequence: 4 }).unwrap_err().code, SOURCE_INCARNATION_RETIRED);
        assert_eq!(custody.begin_next(old).unwrap_err().code, SOURCE_INCARNATION_RETIRED);
    }

    /// Two activations racing on the same expectation: exactly one succeeds.
    #[test]
    fn racing_conditional_activations_on_one_expectation_admit_exactly_one() {
        for _ in 0..50 {
            let custody = std::sync::Arc::new(RenderCustody::new());
            custody.activate_if(0).unwrap();
            let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
            let racers: Vec<_> = (0..2)
                .map(|_| {
                    let (custody, barrier) = (std::sync::Arc::clone(&custody), std::sync::Arc::clone(&barrier));
                    std::thread::spawn(move || {
                        barrier.wait();
                        custody.activate_if(1).is_ok()
                    })
                })
                .collect();
            let won: usize = racers.into_iter().map(|racer| usize::from(racer.join().unwrap())).sum();
            assert_eq!((won, custody.observe().incarnation), (1, 2));
        }
    }

    // M06 and M04 (round-three response): a retirement names its render request (source and the
    // ordinal its owner allocated when preparation began), and entry refuses an ordinal at or
    // below the incarnation's entry watermark. The expected outcomes are the settled design's.

    const fn req(source: SourceMark, ordinal: u64) -> RenderRequest {
        RenderRequest { source, ordinal }
    }

    fn render_request(custody: &RenderCustody, request: RenderRequest, html: &str) -> Result<u64, RouteError> {
        let ticket = custody.begin_render(request)?;
        custody.complete(ticket, Some(LOCAL_PRIVATE), html, &sha256_hex(html.as_bytes()), false, false)
    }

    /// 1. An old panel's retirement processed after its replacement activated, rendered and
    /// opened a print leaves the replacement untouched; the new incarnation's watermark starts
    /// afresh.
    #[test]
    fn an_old_retirement_after_a_replacement_incarnation_changes_nothing() {
        use crate::rendered_report_print::{admit_print_in, select_retired_in, OpenPrint};
        let custody = RenderCustody::new();
        custody.activate();
        let old = req(FIRST, 5);
        render_request(&custody, old, REPORT_A).unwrap();
        let second = custody.activate();
        let replacement = req(second, 1);
        let generation = render_request(&custody, replacement, REPORT_B).expect("a new incarnation's watermark starts at 0");
        let document = admit_print_in(&custody, &at(REPORT_B, second)).unwrap();
        let open = std::sync::Mutex::new(Some(OpenPrint { label: "report-print-1".to_string(), document: std::sync::Arc::new(document), served: false, printed: false }));
        assert!(!custody.retire_request(old), "another incarnation's retirement changes nothing");
        assert!(custody.is_current(generation));
        assert!(custody.admit(&at(REPORT_B, second)).is_ok());
        assert_eq!(select_retired_in(&open, &custody), None, "the replacement's print window is kept");
    }

    /// 2. A retirement of an earlier request processed after a newer render on the same source
    /// leaves the newer ticket, custody and window untouched.
    #[test]
    fn an_old_retirement_after_a_newer_same_basis_render_changes_nothing() {
        use crate::rendered_report_print::{admit_print_in, select_retired_in, OpenPrint};
        let custody = RenderCustody::new();
        custody.activate();
        let (a, b) = (req(FIRST, 1), req(FIRST, 2));
        render_request(&custody, a, REPORT_A).unwrap();
        let generation = render_request(&custody, b, REPORT_A).unwrap(); // the same basis and bytes, a different request
        let document = admit_print_in(&custody, &at(REPORT_A, FIRST)).unwrap();
        let open = std::sync::Mutex::new(Some(OpenPrint { label: "report-print-2".to_string(), document: std::sync::Arc::new(document), served: false, printed: false }));
        assert!(!custody.retire_request(a));
        assert!(custody.is_current(generation));
        assert_eq!(select_retired_in(&open, &custody), None);
        // B's pending successor is likewise kept when A's retirement arrives during it.
        let c = req(FIRST, 3);
        let ticket = custody.begin_render(c).unwrap();
        assert!(!custody.retire_request(b), "B is no longer active; its retirement does not reach C");
        assert!(custody.complete(ticket, Some(LOCAL_PRIVATE), REPORT_B, &sha256_hex(REPORT_B.as_bytes()), false, false).is_ok());
    }

    /// 3 and 7. A retirement that arrives before its request entered raises the watermark: the
    /// later entry is refused and nothing is resurrected. The already-admitted earlier render,
    /// pending or completed, stays valid (its completion checks its ticket and source, not the
    /// watermark), and a genuinely newer request still enters.
    #[test]
    fn a_retirement_before_entry_refuses_that_entry_and_keeps_an_unrelated_admitted_render() {
        let custody = RenderCustody::new();
        custody.activate();
        let (a, b, c) = (req(FIRST, 1), req(FIRST, 2), req(FIRST, 3));
        let pending_a = custody.begin_render(a).unwrap(); // A admitted and still rendering
        assert!(!custody.retire_request(b), "B had not entered: nothing completed was retired");
        let generation_a = custody.complete(pending_a, Some(LOCAL_PRIVATE), REPORT_A, &sha256_hex(REPORT_A.as_bytes()), false, false).expect("A's completion is unaffected by B's cancellation");
        assert!(custody.admit(&at(REPORT_A, FIRST)).is_ok());
        let entry = custody.begin_render(b).unwrap_err();
        assert_eq!((entry.code.as_str(), entry.stage.as_str()), (RENDER_SUPERSEDED, "render"));
        assert!(custody.is_current(generation_a), "B's refused entry retired nothing");
        assert!(!custody.retire_request(b), "a repeated retirement of B changes nothing");
        assert!(custody.is_current(generation_a));
        render_request(&custody, c, REPORT_B).expect("a newer request still enters");
        assert!(!custody.is_current(generation_a));
    }

    /// 4. Entry, then the targeted retirement, then completion: the completion is refused.
    #[test]
    fn a_targeted_retirement_after_entry_refuses_the_completion() {
        let custody = RenderCustody::new();
        custody.activate();
        let a = req(FIRST, 1);
        let generation = render_request(&custody, a, REPORT_A).unwrap();
        assert!(custody.retire_request(a), "the exact completed request is retired");
        assert!(!custody.is_current(generation));
        let b = req(FIRST, 2);
        let ticket = custody.begin_render(b).unwrap();
        assert!(!custody.retire_request(b), "the exact pending request is retired with its ticket");
        let late = custody.complete(ticket, Some(LOCAL_PRIVATE), REPORT_B, &sha256_hex(REPORT_B.as_bytes()), false, false);
        assert_eq!(late.map_err(|error| error.code), Err(RENDER_SUPERSEDED.to_string()));
        assert_eq!(custody.admit(&at(REPORT_B, FIRST)).unwrap_err().code, NOT_CURRENT);
        assert_eq!(custody.begin_render(b).unwrap_err().code, RENDER_SUPERSEDED, "a retired request cannot enter again");
    }

    /// 6. Same-basis requests A then B, prepared in that order. B enters first: A, delivered
    /// late, is refused at entry and B stays bound. A enters first: B supersedes it and A's late
    /// completion is refused.
    #[test]
    fn same_basis_requests_are_ordered_by_their_ordinals_in_either_entry_order() {
        let (a, b) = (req(FIRST, 1), req(FIRST, 2));
        let custody = RenderCustody::new();
        custody.activate();
        let generation_b = render_request(&custody, b, REPORT_B).unwrap();
        assert_eq!(custody.begin_render(a).unwrap_err().code, RENDER_SUPERSEDED);
        assert!(custody.is_current(generation_b));
        assert!(custody.admit(&at(REPORT_B, FIRST)).is_ok());

        let custody = RenderCustody::new();
        custody.activate();
        let ticket_a = custody.begin_render(a).unwrap();
        let generation_b = render_request(&custody, b, REPORT_B).unwrap();
        let late = custody.complete(ticket_a, Some(LOCAL_PRIVATE), REPORT_A, &sha256_hex(REPORT_A.as_bytes()), false, false);
        assert_eq!(late.map_err(|error| error.code), Err(RENDER_SUPERSEDED.to_string()));
        assert!(custody.is_current(generation_b));
    }

    /// A source advance retires the active request; the watermark stays, so an older ordinal
    /// cannot enter on the newer source either, and a retirement of the retired request is
    /// harmless.
    #[test]
    fn an_advance_retires_the_active_request_and_keeps_the_entry_watermark() {
        let custody = RenderCustody::new();
        custody.activate();
        let a = req(FIRST, 4);
        render_request(&custody, a, REPORT_A).unwrap();
        custody.advance(NEXT).unwrap();
        assert_eq!(custody.begin_render(req(NEXT, 3)).unwrap_err().code, RENDER_SUPERSEDED);
        let generation = render_request(&custody, req(NEXT, 5), REPORT_B).unwrap();
        assert!(!custody.retire_request(a));
        assert!(custody.is_current(generation));
    }
}
