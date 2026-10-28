//! The unsaved-changes guard on window close and app quit (W-17, HF-08).
//!
//! The webview owns the model, its dirty state, its modal dialogs and the
//! unsaved-changes prompt; the native host owns the close and quit requests.
//! Every close or quit is held and the webview is asked
//! (`openpipestress-native-close-request`, carrying the request id and the
//! attempt). The host never decides from a cached dirty flag: the webview
//! answers from its current state (Codex r1 P1).
//!
//! The protocol, per attempt:
//! - The webview acknowledges the attempt (`acknowledge`). The host accepts
//!   only the open request's current attempt, and only before its deadline.
//!   The webview shows a prompt only after the host accepted; once accepted,
//!   no deadline closes the app (the user is choosing).
//! - The acknowledgement deadline (`ACK_TIMEOUT`) and the acknowledgement are
//!   decided under one lock, so exactly one of them wins. A deadline that wins
//!   approves the request and the app closes; the late acknowledgement is
//!   refused, so the webview shows nothing on a closing window. This is the
//!   bounded fallback for a crashed or unresponsive webview.
//! - The webview resolves the request (`resolve`): proceed closes or quits;
//!   anything else keeps the app open.
//! - A repeat close or quit while a request is open is a new attempt of that
//!   request with a fresh deadline. A live webview acknowledges it and decides
//!   what it means (nothing while its prompt is showing; an explicit way out
//!   while a save it is waiting for is outstanding). A webview that hung after
//!   an earlier acknowledgement does not acknowledge, and the app closes at the
//!   new deadline.
use std::sync::Mutex;
use std::time::Duration;

use serde::Serialize;

/// The DOM event the host dispatches into the main webview.
pub const CLOSE_REQUEST_DOM_EVENT: &str = "openpipestress-native-close-request";

/// How long the host waits for the webview to acknowledge an attempt.
pub const ACK_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CloseKind {
    /// The main window's close button.
    Window,
    /// SWBPIPE › Quit SWBPIPE (⌘Q).
    Quit,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CloseDecision {
    /// Close or quit now (the request was already approved).
    Proceed,
    /// Hold the request, dispatch this attempt to the webview and arm its
    /// acknowledgement deadline.
    Ask { request_id: u64, attempt: u64, kind: CloseKind },
}

#[derive(Debug)]
struct Pending {
    request_id: u64,
    attempt: u64,
    kind: CloseKind,
    /// Whether the current attempt was acknowledged.
    acknowledged: bool,
}

#[derive(Debug, Default)]
struct State {
    approved: bool,
    next_request_id: u64,
    pending: Option<Pending>,
}

#[derive(Debug, Default)]
pub struct CloseGuard {
    state: Mutex<State>,
}

impl CloseGuard {
    fn state(&self) -> std::sync::MutexGuard<'_, State> {
        // A panic while holding the lock leaves plain data; keep using it.
        self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// A close or quit request from the user. Always asks, unless the request
    /// was already approved.
    pub fn request(&self, kind: CloseKind) -> CloseDecision {
        let mut state = self.state();
        if state.approved {
            return CloseDecision::Proceed;
        }
        if let Some(pending) = state.pending.as_mut() {
            // A new attempt of the open request; a quit outranks a window close.
            pending.attempt += 1;
            pending.acknowledged = false;
            if kind == CloseKind::Quit {
                pending.kind = CloseKind::Quit;
            }
            return CloseDecision::Ask { request_id: pending.request_id, attempt: pending.attempt, kind: pending.kind };
        }
        state.next_request_id += 1;
        let request_id = state.next_request_id;
        state.pending = Some(Pending { request_id, attempt: 1, kind, acknowledged: false });
        CloseDecision::Ask { request_id, attempt: 1, kind }
    }

    /// The webview received `attempt`. True when the host accepts it: it is
    /// the open request's current attempt and its deadline has not passed.
    /// Only then may the webview show a prompt.
    pub fn acknowledge(&self, request_id: u64, attempt: u64) -> bool {
        match self.state().pending.as_mut() {
            Some(pending) if pending.request_id == request_id && pending.attempt == attempt => {
                pending.acknowledged = true;
                true
            }
            _ => false,
        }
    }

    /// The acknowledgement deadline of `attempt` passed. `Some(kind)` when
    /// that attempt is still current and was never acknowledged: the request
    /// is approved and proceeds.
    pub fn acknowledgement_deadline(&self, request_id: u64, attempt: u64) -> Option<CloseKind> {
        let mut state = self.state();
        let expired = matches!(state.pending.as_ref(),
            Some(pending) if pending.request_id == request_id && pending.attempt == attempt && !pending.acknowledged);
        if !expired {
            return None;
        }
        state.approved = true;
        state.pending.take().map(|pending| pending.kind)
    }

    /// The user's decision in the webview. `Some(kind)` when the request
    /// proceeds; `None` when it was kept open or is not the open request.
    pub fn resolve(&self, request_id: u64, proceed: bool) -> Option<CloseKind> {
        let mut state = self.state();
        if !matches!(state.pending.as_ref(), Some(pending) if pending.request_id == request_id) {
            return None;
        }
        let pending = state.pending.take()?;
        if !proceed {
            return None;
        }
        state.approved = true;
        Some(pending.kind)
    }

    /// The webview could not be asked (no window, or the dispatch failed).
    pub fn abandon_to_proceed(&self, request_id: u64) -> Option<CloseKind> {
        let mut state = self.state();
        if !matches!(state.pending.as_ref(), Some(pending) if pending.request_id == request_id) {
            return None;
        }
        state.approved = true;
        state.pending.take().map(|pending| pending.kind)
    }
}

/// The script that delivers an attempt to the main webview.
pub fn close_request_dispatch_script(request_id: u64, attempt: u64, kind: CloseKind) -> String {
    let detail = serde_json::json!({ "requestId": request_id, "attempt": attempt, "kind": kind });
    format!("window.dispatchEvent(new CustomEvent('{CLOSE_REQUEST_DOM_EVENT}', {{ detail: {detail} }}));")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ask(guard: &CloseGuard, kind: CloseKind) -> (u64, u64, CloseKind) {
        match guard.request(kind) {
            CloseDecision::Ask { request_id, attempt, kind } => (request_id, attempt, kind),
            CloseDecision::Proceed => panic!("must ask"),
        }
    }

    #[test]
    fn every_close_and_quit_asks_the_webview_none_proceeds_from_a_cache() {
        // Codex r1 P1: an edit whose shell sync has not landed is still asked about.
        let guard = CloseGuard::default();
        let (request_id, attempt, kind) = ask(&guard, CloseKind::Window);
        assert_eq!((attempt, kind), (1, CloseKind::Window));
        assert!(guard.acknowledge(request_id, attempt));
        assert_eq!(guard.resolve(request_id, false), None);
        let (next, _, kind) = ask(&guard, CloseKind::Quit);
        assert_ne!(next, request_id);
        assert_eq!(kind, CloseKind::Quit);
    }

    #[test]
    fn an_accepted_acknowledgement_holds_the_request_until_the_user_decides() {
        let guard = CloseGuard::default();
        let (request_id, attempt, _) = ask(&guard, CloseKind::Quit);
        assert!(guard.acknowledge(request_id, attempt));
        assert_eq!(guard.acknowledgement_deadline(request_id, attempt), None, "no deadline closes while the user is choosing");
        assert_eq!(guard.resolve(request_id, true), Some(CloseKind::Quit));
        assert_eq!(guard.request(CloseKind::Window), CloseDecision::Proceed);
    }

    #[test]
    fn a_late_acknowledgement_is_refused_after_the_deadline_closed_the_app() {
        // Codex r1 P1: the webview must not show a prompt on a closing window.
        let guard = CloseGuard::default();
        let (request_id, attempt, _) = ask(&guard, CloseKind::Quit);
        assert_eq!(guard.acknowledgement_deadline(request_id, attempt), Some(CloseKind::Quit));
        assert!(!guard.acknowledge(request_id, attempt));
        assert_eq!(guard.resolve(request_id, false), None, "a late answer changes nothing");
        assert_eq!(guard.request(CloseKind::Quit), CloseDecision::Proceed);
    }

    #[test]
    fn an_acknowledgement_of_a_superseded_attempt_is_refused() {
        let guard = CloseGuard::default();
        let (request_id, first, _) = ask(&guard, CloseKind::Window);
        let (same, second, _) = ask(&guard, CloseKind::Window);
        assert_eq!(same, request_id);
        assert!(!guard.acknowledge(request_id, first));
        assert!(guard.acknowledge(request_id, second));
    }

    #[test]
    fn a_repeat_request_asks_again_and_its_own_deadline_bounds_a_hung_webview() {
        let guard = CloseGuard::default();
        let (request_id, first, _) = ask(&guard, CloseKind::Window);
        assert!(guard.acknowledge(request_id, first));
        // The webview hangs after acknowledging; the user quits again.
        let (same, second, kind) = ask(&guard, CloseKind::Quit);
        assert_eq!((same, kind), (request_id, CloseKind::Quit));
        assert_eq!(second, first + 1);
        assert_eq!(guard.acknowledgement_deadline(request_id, first), None, "the first deadline is superseded");
        assert_eq!(guard.acknowledgement_deadline(request_id, second), Some(CloseKind::Quit));
    }

    #[test]
    fn a_repeat_acknowledged_by_a_live_webview_keeps_the_request_open_for_its_answer() {
        // Codex r1 P2: a live webview answers the repeat (for example, quit
        // without waiting for an outstanding save); the host waits for it.
        let guard = CloseGuard::default();
        let (request_id, first, _) = ask(&guard, CloseKind::Quit);
        assert!(guard.acknowledge(request_id, first));
        let (_, second, _) = ask(&guard, CloseKind::Quit);
        assert!(guard.acknowledge(request_id, second));
        assert_eq!(guard.acknowledgement_deadline(request_id, second), None);
        assert_eq!(guard.resolve(request_id, true), Some(CloseKind::Quit));
    }

    #[test]
    fn stale_answers_are_ignored_and_an_unreachable_webview_proceeds() {
        let guard = CloseGuard::default();
        assert!(!guard.acknowledge(7, 1));
        assert_eq!(guard.resolve(7, true), None);
        let (request_id, _, _) = ask(&guard, CloseKind::Window);
        assert_eq!(guard.resolve(request_id + 1, true), None);
        assert_eq!(guard.abandon_to_proceed(request_id), Some(CloseKind::Window));
    }

    #[test]
    fn the_dispatch_script_carries_the_request_and_attempt_as_json() {
        let script = close_request_dispatch_script(3, 2, CloseKind::Quit);
        let detail = script
            .strip_prefix("window.dispatchEvent(new CustomEvent('openpipestress-native-close-request', { detail: ")
            .and_then(|rest| rest.strip_suffix(" }));"))
            .expect("the script dispatches the close-request event");
        let detail: serde_json::Value = serde_json::from_str(detail).unwrap();
        assert_eq!(detail, serde_json::json!({ "requestId": 3, "attempt": 2, "kind": "quit" }));
    }
}
