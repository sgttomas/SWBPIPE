//! The unsaved-changes guard on window close and app quit (W-17, HF-08).
//!
//! The webview owns the model and its unsaved-changes prompt; the native host
//! owns the close and quit requests. While the last synchronized shell state
//! (`sync_native_shell_state`) says the working model has edits no save holds,
//! a close or quit is held and the webview is asked
//! (`openpipestress-native-close-request`). The webview acknowledges at once,
//! shows its prompt (Save Local Project first, Discard, Cancel) and resolves
//! the request: proceed closes or quits, anything else keeps the app open.
//!
//! A crashed or unresponsive webview cannot hold the app open: a request it
//! does not acknowledge within `ACK_TIMEOUT` proceeds. A repeated close or
//! quit while a request is open asks again with a fresh deadline, so a webview
//! that hangs after acknowledging is caught by the user's next attempt. With
//! no edits the request proceeds at once without asking.
use std::sync::Mutex;
use std::time::Duration;

use serde::Serialize;

/// The DOM event the host dispatches into the main webview.
pub const CLOSE_REQUEST_DOM_EVENT: &str = "openpipestress-native-close-request";

/// How long the host waits for the webview to acknowledge a request.
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
    /// Close or quit now.
    Proceed,
    /// Hold the request, dispatch it to the webview and arm the
    /// acknowledgement deadline for `attempt`.
    Ask { request_id: u64, attempt: u64, kind: CloseKind },
}

#[derive(Debug)]
struct Pending {
    request_id: u64,
    attempt: u64,
    kind: CloseKind,
    acknowledged: bool,
}

#[derive(Debug, Default)]
struct State {
    model_edited: bool,
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

    /// The webview's latest "the working model has unsaved edits".
    pub fn set_model_edited(&self, edited: bool) {
        self.state().model_edited = edited;
    }

    /// A close or quit request from the user.
    pub fn request(&self, kind: CloseKind) -> CloseDecision {
        let mut state = self.state();
        if state.approved {
            return CloseDecision::Proceed;
        }
        if let Some(pending) = state.pending.as_mut() {
            // Ask again with a fresh deadline; a quit outranks a window close.
            pending.attempt += 1;
            pending.acknowledged = false;
            if kind == CloseKind::Quit {
                pending.kind = CloseKind::Quit;
            }
            return CloseDecision::Ask { request_id: pending.request_id, attempt: pending.attempt, kind: pending.kind };
        }
        if !state.model_edited {
            return CloseDecision::Proceed;
        }
        state.next_request_id += 1;
        let request_id = state.next_request_id;
        state.pending = Some(Pending { request_id, attempt: 1, kind, acknowledged: false });
        CloseDecision::Ask { request_id, attempt: 1, kind }
    }

    /// The webview received the request. False when it is not the open one.
    pub fn acknowledge(&self, request_id: u64) -> bool {
        match self.state().pending.as_mut() {
            Some(pending) if pending.request_id == request_id => {
                pending.acknowledged = true;
                true
            }
            _ => false,
        }
    }

    /// The acknowledgement deadline of `attempt` passed. `Some(kind)` when
    /// that attempt was never acknowledged: the request proceeds.
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

/// The script that delivers a request to the main webview.
pub fn close_request_dispatch_script(request_id: u64, kind: CloseKind) -> String {
    let detail = serde_json::json!({ "requestId": request_id, "kind": kind });
    format!("window.dispatchEvent(new CustomEvent('{CLOSE_REQUEST_DOM_EVENT}', {{ detail: {detail} }}));")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn edited() -> CloseGuard {
        let guard = CloseGuard::default();
        guard.set_model_edited(true);
        guard
    }

    #[test]
    fn without_edits_close_and_quit_proceed_without_asking() {
        let guard = CloseGuard::default();
        assert_eq!(guard.request(CloseKind::Window), CloseDecision::Proceed);
        assert_eq!(guard.request(CloseKind::Quit), CloseDecision::Proceed);
    }

    #[test]
    fn with_edits_the_webview_is_asked_and_cancel_keeps_the_app_open() {
        let guard = edited();
        let CloseDecision::Ask { request_id, attempt, kind } = guard.request(CloseKind::Quit) else { panic!("must ask") };
        assert_eq!((attempt, kind), (1, CloseKind::Quit));
        assert!(guard.acknowledge(request_id));
        assert_eq!(guard.acknowledgement_deadline(request_id, attempt), None, "an acknowledged request waits for the user");
        assert_eq!(guard.resolve(request_id, false), None);
        // Still edited: the next attempt asks again, as a new request.
        let CloseDecision::Ask { request_id: next, .. } = guard.request(CloseKind::Window) else { panic!("must ask") };
        assert_ne!(next, request_id);
    }

    #[test]
    fn the_users_proceed_approves_every_later_request() {
        let guard = edited();
        let CloseDecision::Ask { request_id, .. } = guard.request(CloseKind::Window) else { panic!("must ask") };
        assert!(guard.acknowledge(request_id));
        assert_eq!(guard.resolve(request_id, true), Some(CloseKind::Window));
        assert_eq!(guard.request(CloseKind::Window), CloseDecision::Proceed);
    }

    #[test]
    fn an_unacknowledged_request_proceeds_at_its_deadline() {
        let guard = edited();
        let CloseDecision::Ask { request_id, attempt, .. } = guard.request(CloseKind::Quit) else { panic!("must ask") };
        assert_eq!(guard.acknowledgement_deadline(request_id, attempt), Some(CloseKind::Quit));
        assert_eq!(guard.request(CloseKind::Quit), CloseDecision::Proceed);
        assert_eq!(guard.resolve(request_id, false), None, "a late answer changes nothing");
    }

    #[test]
    fn a_repeat_request_asks_again_and_its_own_deadline_bounds_a_hung_webview() {
        let guard = edited();
        let CloseDecision::Ask { request_id, attempt: first, .. } = guard.request(CloseKind::Window) else { panic!("must ask") };
        assert!(guard.acknowledge(request_id));
        // The webview hangs after acknowledging; the user quits again.
        let CloseDecision::Ask { request_id: same, attempt: second, kind } = guard.request(CloseKind::Quit) else { panic!("must ask") };
        assert_eq!((same, kind), (request_id, CloseKind::Quit));
        assert_eq!(second, first + 1);
        assert_eq!(guard.acknowledgement_deadline(request_id, first), None, "the first deadline is superseded");
        assert_eq!(guard.acknowledgement_deadline(request_id, second), Some(CloseKind::Quit));
    }

    #[test]
    fn stale_answers_and_acknowledgements_are_ignored() {
        let guard = edited();
        assert!(!guard.acknowledge(7));
        assert_eq!(guard.resolve(7, true), None);
        let CloseDecision::Ask { request_id, .. } = guard.request(CloseKind::Window) else { panic!("must ask") };
        assert_eq!(guard.resolve(request_id + 1, true), None);
        assert_eq!(guard.abandon_to_proceed(request_id), Some(CloseKind::Window));
    }

    #[test]
    fn the_dispatch_script_carries_the_request_as_json() {
        let script = close_request_dispatch_script(3, CloseKind::Quit);
        let detail = script
            .strip_prefix("window.dispatchEvent(new CustomEvent('openpipestress-native-close-request', { detail: ")
            .and_then(|rest| rest.strip_suffix(" }));"))
            .expect("the script dispatches the close-request event");
        let detail: serde_json::Value = serde_json::from_str(detail).unwrap();
        assert_eq!(detail, serde_json::json!({ "requestId": 3, "kind": "quit" }));
    }
}
