//! Which webview may call an application command: the main one only.
//!
//! The desktop opens webviews other than the main window's (the report's
//! print window, `rendered_report_print`), each showing a report document and
//! none meant to call any application command. Their pages carry no script,
//! their content policy refuses page script and the report's frame is
//! sandboxed; this module is the barrier at native dispatch beneath those.
//!
//! It is needed because nothing in Tauri stands there for this app (tauri
//! 2.11.1): the app defines no permission manifest and no capability, so
//! `RuntimeAuthority::has_app_manifest` is false, and `Webview::on_message`
//! (`src/webview/mod.rs`) checks access to an app command only for a remote
//! origin, while `is_local_url` counts every registered custom scheme,
//! the print window's included, as local. Tauri installs its bridge script in
//! every webview's main frame (`src/manager/webview.rs`).
//!
//! [`main_webview_only`] wraps the whole handler `app_invoke_handler!`
//! produces, at both of its uses in `run`. It decides from the calling
//! webview's label and its window's label alone, before the handler is
//! entered, so no command is matched, no argument decoded, no ingress
//! admitted and no lease taken for a refused call, whichever command it
//! names; the caller receives [`COMMAND_WEBVIEW_REFUSED`], which names no
//! command, path or document. Plugin commands (`plugin:…`) never reach this
//! handler: Tauri checks every one against the capabilities, of which this
//! app has none.

use tauri::ipc::Invoke;
use tauri::Runtime;

/// The label of the one webview, and of its window, that may call an
/// application command: the main window's (`tauri.conf.json` gives it no
/// label, so it takes Tauri's default).
pub(crate) const COMMAND_WEBVIEW: &str = "main";

/// What a command invoked from any other webview is rejected with.
pub(crate) const COMMAND_WEBVIEW_REFUSED: &str = "COMMAND-WEBVIEW-REFUSED";

/// Whether a call from the webview `webview_label`, hosted by the window
/// `window_label`, may reach an application command.
pub(crate) fn from_main(webview_label: &str, window_label: &str) -> bool {
    webview_label == COMMAND_WEBVIEW && window_label == COMMAND_WEBVIEW
}

/// An invoke as the gate sees it: where it came from, and how it is refused.
pub(crate) trait Caller {
    /// Whether the call comes from the main webview ([`from_main`]).
    fn from_main(&self) -> bool;
    /// Rejects the call with [`COMMAND_WEBVIEW_REFUSED`].
    fn refuse(self);
}

impl<R: Runtime> Caller for Invoke<R> {
    fn from_main(&self) -> bool {
        let webview = self.message.webview_ref();
        let window = webview.window_ref();
        from_main(webview.label(), window.label())
    }

    fn refuse(self) {
        self.resolver.reject(COMMAND_WEBVIEW_REFUSED);
    }
}

/// `handler`, entered only for a call from the main webview; any other call
/// is refused and reported handled, so Tauri adds no message of its own.
pub(crate) fn only_from_main<I: Caller>(handler: impl Fn(I) -> bool + Send + Sync + 'static) -> impl Fn(I) -> bool + Send + Sync + 'static {
    move |invoke: I| {
        if invoke.from_main() {
            handler(invoke)
        } else {
            invoke.refuse();
            true
        }
    }
}

/// The app's invoke handler, gated: what `Builder::invoke_handler` is given.
pub(crate) fn main_webview_only<R: Runtime>(handler: impl Fn(Invoke<R>) -> bool + Send + Sync + 'static) -> impl Fn(Invoke<R>) -> bool + Send + Sync + 'static {
    only_from_main(handler)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};
    use std::sync::{Arc, Mutex};
    use syn::visit::{self, Visit};

    /// What a call did: the command bodies the handler ran, and the refusals.
    #[derive(Default)]
    struct Record {
        ran: Vec<String>,
        refused: Vec<&'static str>,
    }

    /// An invoke of `command` from the webview `webview` in the window `window`.
    struct FakeInvoke {
        webview: &'static str,
        window: &'static str,
        command: String,
        record: Arc<Mutex<Record>>,
    }

    impl Caller for FakeInvoke {
        fn from_main(&self) -> bool {
            from_main(self.webview, self.window)
        }

        fn refuse(self) {
            self.record.lock().unwrap().refused.push(COMMAND_WEBVIEW_REFUSED);
        }
    }

    /// The handler's command names as Tauri dispatches them (each path's last
    /// segment), read from `lib.rs`: `app_invoke_handler`'s own list, and the
    /// extra commands the native-smoke build passes it in `run`.
    fn handler_commands() -> (Vec<String>, Vec<String>) {
        let lib = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs")).unwrap();
        let file = syn::parse_file(&lib).unwrap();
        let last_segments = |text: &str| -> Vec<String> {
            text.split(',').map(|path| path.split("::").last().unwrap().trim().to_string()).filter(|name| !name.is_empty()).collect()
        };
        let mut own = None;
        for item in &file.items {
            if let syn::Item::Macro(item) = item {
                if item.ident.as_ref().is_some_and(|ident| ident == "app_invoke_handler") {
                    let tokens = item.mac.tokens.to_string();
                    let list = tokens.split("generate_handler ! [").nth(1).and_then(|rest| rest.split('$').next()).expect("the macro's command list");
                    own = Some(last_segments(list));
                }
            }
        }
        struct Uses(Vec<String>);
        impl<'ast> Visit<'ast> for Uses {
            fn visit_macro(&mut self, node: &'ast syn::Macro) {
                if node.path.is_ident("app_invoke_handler") {
                    self.0.push(node.tokens.to_string());
                }
                visit::visit_macro(self, node);
            }
        }
        let mut uses = Uses(Vec::new());
        uses.visit_file(&file);
        let extras: Vec<String> = uses.0.iter().flat_map(|tokens| last_segments(tokens)).collect();
        (own.expect("lib.rs defines app_invoke_handler"), extras)
    }

    /// The handler as `generate_handler!` writes it, a command's body standing
    /// for its observable effect: a known command runs and is handled, an
    /// unknown one is not handled.
    fn fake_handler(known: Vec<String>) -> impl Fn(FakeInvoke) -> bool + Send + Sync + 'static {
        move |invoke: FakeInvoke| {
            if known.contains(&invoke.command) {
                invoke.record.lock().unwrap().ran.push(invoke.command.clone());
                true
            } else {
                false
            }
        }
    }

    fn call(gated: &impl Fn(FakeInvoke) -> bool, webview: &'static str, window: &'static str, command: &str) -> (bool, Record) {
        let record = Arc::new(Mutex::new(Record::default()));
        let handled = gated(FakeInvoke { webview, window, command: command.to_string(), record: Arc::clone(&record) });
        let record = std::mem::take(&mut *record.lock().unwrap());
        (handled, record)
    }

    /// The commands at each end of the macro's list, and the last extra
    /// command of the native-smoke build.
    fn chosen() -> Vec<String> {
        let (own, extras) = handler_commands();
        assert!(own.len() > 100, "the macro's list is read whole: {own:?}");
        assert_eq!(own.first().map(String::as_str), Some("embedded_agent_host"));
        assert_eq!(own.last().map(String::as_str), Some("abort_local_csv_stream"));
        assert_eq!(extras.last().map(String::as_str), Some("native_smoke_finish"));
        vec![own.first().unwrap().clone(), own.last().unwrap().clone(), extras.last().unwrap().clone()]
    }

    #[test]
    fn only_the_main_label_in_the_main_window_is_from_main() {
        assert!(from_main("main", "main"));
        for (webview, window) in [
            ("report-view-1", "report-view-1"),
            ("report-print-1", "report-print-1"),
            ("main", "report-view-1"),
            ("report-view-1", "main"),
            ("Main", "Main"),
            ("main ", "main "),
            ("", ""),
        ] {
            assert!(!from_main(webview, window), "{webview:?} in {window:?}");
        }
    }

    #[test]
    fn a_report_view_or_print_webview_is_refused_before_any_command_runs() {
        let commands = chosen();
        let gated = only_from_main(fake_handler(commands.clone()));
        for label in ["report-view-1", "report-print-1"] {
            for command in &commands {
                let (handled, record) = call(&gated, label, label, command);
                assert!(handled, "{label} {command}: reported handled, so Tauri adds no message naming the command");
                assert!(record.ran.is_empty(), "{label} {command}: no command body ran");
                assert_eq!(record.refused, [COMMAND_WEBVIEW_REFUSED], "{label} {command}: refused once");
            }
            let (handled, record) = call(&gated, label, label, "no_such_command");
            assert!(handled && record.ran.is_empty() && record.refused == [COMMAND_WEBVIEW_REFUSED], "{label}: an unknown command is refused alike");
        }
    }

    #[test]
    fn the_main_webview_reaches_every_command() {
        let (own, extras) = handler_commands();
        let commands: Vec<String> = own.into_iter().chain(extras).collect();
        let gated = only_from_main(fake_handler(commands.clone()));
        for command in &commands {
            let (handled, record) = call(&gated, "main", "main", command);
            assert!(handled, "{command}");
            assert_eq!(record.ran, [command.clone()], "{command}: its body ran");
            assert!(record.refused.is_empty(), "{command}: not refused");
        }
        let (handled, record) = call(&gated, "main", "main", "no_such_command");
        assert!(!handled && record.ran.is_empty() && record.refused.is_empty(), "an unknown command from main is left to Tauri, as before");
    }

    /// The refusal is one fixed code: nothing of the call is put into it.
    #[test]
    fn the_refusal_is_a_fixed_code() {
        assert_eq!(COMMAND_WEBVIEW_REFUSED, "COMMAND-WEBVIEW-REFUSED");
    }

    fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                sources(&path, out);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                out.push(path);
            }
        }
    }

    /// Every `invoke_handler` call in the crate's source: its file and its
    /// argument as written.
    fn invoke_handler_calls() -> Vec<(String, String)> {
        struct Calls<'a>(&'a str, Vec<(String, String)>);
        impl<'ast> Visit<'ast> for Calls<'_> {
            fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
                if node.method == "invoke_handler" {
                    let argument = node.args.iter().map(|arg| shape(arg)).collect::<Vec<_>>().join(", ");
                    self.1.push((self.0.to_string(), argument));
                }
                visit::visit_expr_method_call(self, node);
            }
        }
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut files = Vec::new();
        sources(&root.join("src"), &mut files);
        assert!(files.len() > 20, "the crate's sources are read");
        let mut calls = Vec::new();
        for file in &files {
            let relative = file.strip_prefix(root).unwrap().display().to_string();
            let text = std::fs::read_to_string(file).unwrap();
            let parsed = syn::parse_file(&text).unwrap_or_else(|error| panic!("{relative} does not parse: {error}"));
            let mut visitor = Calls(&relative, Vec::new());
            visitor.visit_file(&parsed);
            calls.extend(visitor.1);
        }
        calls
    }

    /// An expression's shape for comparison: a call's function path and its
    /// arguments, a macro's path; anything else is `other`.
    fn shape(expr: &syn::Expr) -> String {
        let path = |path: &syn::Path| path.segments.iter().map(|segment| segment.ident.to_string()).collect::<Vec<_>>().join("::");
        match expr {
            syn::Expr::Call(call) => match &*call.func {
                syn::Expr::Path(func) => format!("{}({})", path(&func.path), call.args.iter().map(shape).collect::<Vec<_>>().join(", ")),
                _ => "other".to_string(),
            },
            syn::Expr::Macro(mac) => format!("{}!", path(&mac.mac.path)),
            _ => "other".to_string(),
        }
    }

    /// The gate wraps the handler at every place one is installed, so a
    /// command added to `app_invoke_handler`, or passed to it as an extra,
    /// is gated with no further change; a new or changed `invoke_handler`
    /// call fails here.
    #[test]
    fn every_invoke_handler_is_the_gated_app_handler() {
        let gated = ("src/lib.rs".to_string(), "command_webview::main_webview_only(app_invoke_handler!)".to_string());
        assert_eq!(
            invoke_handler_calls(),
            [gated.clone(), gated],
            "the ordinary and the native-smoke build each install the gated handler, and nothing else installs one"
        );
    }
}
