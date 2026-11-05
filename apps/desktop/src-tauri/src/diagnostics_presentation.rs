//! C1 native custody (DEL-04-05 `c1-native-custody-plan.md`, "Native diagnostics presentation";
//! RT-A3 §4, DEL-05-02 Work `combination-certificate.md` §2): the facts the Diagnostics list and
//! explanation classify a native result's diagnostics with, computed here from the held bytes and
//! delivered in S's header (`diagnostics_presentation`), so the webview's one classifier
//! (`suppliedDiagnostics.ts` `presentedResultDiagnostics`) reads them without the rows.
//!
//! - **The facts and their whole-value definitions** (the desktop helpers on the same bytes):
//!   - `combination_identities`: `result_export::document_combination_identities` of the source, the
//!     population the desktop's `documentCombinationIdentities` documents as the same (the same
//!     members, the same ambiguous-identity exclusions), sorted;
//!   - `roster_checked`: `rosterCheckedByReader`: false on a retained successor (a
//!     `retained_precision` member, null included, as `!== undefined` reads it); else the current
//!     ordinary reader of the source's route (`reader_verdict::route`, the route `readerDispatch`
//!     states without a model) accepts the source alone: `reader_verdict::validate(reader, source,
//!     None, None)`, the function the wasm `reader_validate_json` wraps, on the parse of the same
//!     bytes, inside S's current-table scope (never a recorded table, as the webview's reader reads a
//!     whole historical result). Read only when the population is not empty (the classifier does not
//!     read it otherwise), and false then. A model-bound validation (the fresh registration's) is not
//!     this reading: with a model the pressure and preview-physics readers read the certificate's
//!     roster with the model, so its pass is not reused;
//!   - `certificate_attested`: `certificateAttestedByRetainedReader`: a retained successor with the
//!     certificate member whose exact bytes the registered retained reader accepted
//!     (`retainedPrecisionRegistration(result)?.validation`). No native custody holds such a
//!     registration: a fresh desktop solve never registers a retained successor natively
//!     (`previewService.ts` `nativeEnvelopeRefusal`, RETAINED_PRECISION_NATIVE_REGISTRATION_UNAVAILABLE)
//!     and a restored run never registers. A retained reader's generic success or the member's
//!     presence is not that registration, so the fact is false for every held snapshot.
//! - **One snapshot.** The facts, the diagnostics S carries and the validation are all read from the
//!   one share S's command holds (`SolveJobRegistry::held_result`: the bytes and their recorded
//!   table, immutable), and the facts' cache ([`PresentationCache`]) is a field of that share, so a
//!   disposal, reset, new solve or restore (even of the same bytes) cannot carry a cached fact to
//!   another snapshot. Nothing is read from the request, a stored claim, route metadata or a
//!   diagnostic's wording.
//! - **The binding the webview checks** (`binding`): the held bytes' length and SHA-256, the route,
//!   the route's current table binding (`reader_verdict::binding_of`, as the native dispatch states
//!   it) and the recorded table the handle holds (or null). A webview handle whose custody differs
//!   (another length, route, table or recorded table), or whose custody has ended, reads no fact
//!   and quotes every message (`nativeResult.ts` `nativeDiagnosticsPresentation`).
//! - **Memory.** S's command is not a ledger command; the facts are part of its working set: the
//!   population (ids borrowed from the parse, then the sorted ids), the validation's working memory
//!   (unpriced reader working memory, U3) and the digest (streamed); their header text is inside
//!   S's response, admitted at its exact length. The cache holds a digest and a flag per snapshot.
use crate::analysis_run_v03_validate::RecordedTable;
use open_pipe_stress_result_export::reader_verdict as rv;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::sync::OnceLock;

/// S's header member.
pub(crate) const MEMBER: &str = "diagnostics_presentation";
/// The facts' version.
pub(crate) const VERSION: u64 = 1;

/// The ordinary routes' readers (`suppliedDiagnostics.ts` `ROUTE_READERS`); a retained route has none.
const ROUTE_READERS: [(&str, &str); 3] = [("preview_physics", "preview-physics"), ("pressure", "pressure"), ("pressure_source", "pressure-source")];

/// A held snapshot's facts already read: the bytes' digest and the route reader's source-only
/// acceptance. A field of the share ([`crate::solve_jobs::HeldResult`]); both are facts of its
/// immutable bytes under the current tables, so they are read at most once each.
#[derive(Debug, Default)]
pub(crate) struct PresentationCache {
    source_sha256: OnceLock<[u8; 32]>,
    roster_checked: OnceLock<bool>,
}

/// The reader of a route, when it is an ordinary one.
fn route_reader(route: &str) -> Option<&'static str> {
    ROUTE_READERS.iter().find(|(name, _)| *name == route).map(|(_, reader)| *reader)
}

/// The facts of the held `text`, parsed as `source`, held under `recorded` (S's command passes the
/// share's own cache; a build without a share reads afresh).
pub(crate) fn facts(text: &str, source: &Value, recorded: Option<RecordedTable>, cache: Option<&PresentationCache>) -> Value {
    let digest = || -> [u8; 32] { Sha256::digest(text.as_bytes()).into() };
    let sha = match cache {
        Some(cache) => *cache.source_sha256.get_or_init(digest),
        None => digest(),
    };
    let (route, table) = match rv::route(source) {
        Ok((route, _)) => (route, rv::binding_of(route, source)),
        Err(_) => (rv::UNSUPPORTED, Value::Null),
    };
    let mut identities: Vec<String> = open_pipe_stress_result_export::document_combination_identities(source).into_iter().collect();
    identities.sort_unstable();
    let retained = source.get("retained_precision").is_some();
    let roster_checked = !identities.is_empty()
        && !retained
        && route_reader(route).is_some_and(|reader| {
            let read = || rv::validate(reader, source, None, None)["ok"] == true;
            match cache {
                Some(cache) => *cache.roster_checked.get_or_init(read),
                None => read(),
            }
        });
    json!({
        "version": VERSION,
        "binding": {
            "result_bytes": text.len(),
            "source_sha256": sha.iter().map(|byte| format!("{byte:02x}")).collect::<String>(),
            "route": route,
            "reader_table": table,
            "recorded_table": recorded.map_or(Value::Null, RecordedTable::binding),
        },
        "combination_identities": identities,
        "roster_checked": roster_checked,
        "certificate_attested": certificate_attested(),
    })
}

/// `certificateAttestedByRetainedReader` for a held snapshot: false (module docs: no native custody
/// holds a registered retained reader's acceptance of its bytes).
fn certificate_attested() -> bool {
    false
}

#[cfg(test)]
#[path = "diagnostics_presentation_tests.rs"]
mod tests;
