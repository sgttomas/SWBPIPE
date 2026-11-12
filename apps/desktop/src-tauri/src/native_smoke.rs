//! Native smoke harness for the desktop walking skeleton (DEL-07-11 VER-003).
//!
//! Compiled only with the `native-smoke` cargo feature, which no product build
//! enables. `apps/desktop/native-smoke/run.mjs` builds the app with it and
//! launches it with these environment variables:
//!
//! - `SWBPIPE_NATIVE_SMOKE_DRIVER`: a script evaluated in the main webview once
//!   the page has loaded (`apps/desktop/native-smoke/driver.js`); `-` reads it
//!   from bounded anonymous stdin, so fixture credentials need no script file;
//! - `SWBPIPE_NATIVE_SMOKE_PLAN`: JSON handed to the driver;
//! - `SWBPIPE_NATIVE_SMOKE_REPORT`: where `native_smoke_finish` writes the report;
//! - `SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY=stdin` (macOS, with the driver on stdin):
//!   stdin's first line is an agent provider key and the rest is the driver.
//!   The key is held only in this process's memory until the embedded agent's
//!   next Start with a provider saved with no key takes it, once
//!   ([`take_provider_secret`]); it is never written, logged, given to the
//!   webview or read from or saved to the Keychain (RT-E1 D4,
//!   DEL-16-06).
//!
//! The harness substitutes exactly two things for a person at the machine: it
//! names the file the next model-document chooser returns (`native_smoke_choose`;
//! the Open Model Document…, Save Model Document…, Save Handoff…, Save Report
//! Package…, Export MBF's save and Import MBF's open choosers) instead of showing the chooser, and it delivers native menu commands through the same
//! dispatch a menu click uses (`native_smoke_menu`). Everything else, from the
//! open checks to solve, persistence and export, runs on the product's code.

use serde::Serialize;
use serde_json::Value;
use std::{
    collections::VecDeque,
    ffi::OsStr,
    fs,
    io::{self, Read},
    path::{Component, Path, PathBuf},
    sync::Mutex,
};
use tauri::{webview::PageLoadEvent, AppHandle, Builder, Manager, Runtime};

#[derive(Default)]
pub struct SmokeState {
    chosen: Mutex<VecDeque<PathBuf>>,
    driver_started: Mutex<bool>,
}

/// Strict chooser mode. With this variable set to this value, a chooser asked with no answer queued ends
/// the app at once with this exit code, before any native dialog; unset, behaviour is unchanged (other kits
/// rely on the fall-through). A launch that must never show a native dialog sets it; the launch helper
/// (`tools/native_launch_guard.py`, when it reaches main) keeps its own copy of these constants.
pub(crate) const STRICT_CHOOSER_ENV: &str = "SWBPIPE_NATIVE_SMOKE_STRICT_CHOOSER";
pub(crate) const STRICT_CHOOSER_VALUE: &str = "1";
pub(crate) const STRICT_CHOOSER_EXIT: i32 = 86;

/// Whether a chooser that found no queued answer must end the app instead of opening a native dialog.
fn strict_stop(answer_queued: bool, mode: Option<&str>) -> bool {
    !answer_queued && mode == Some(STRICT_CHOOSER_VALUE)
}

/// The path the next chooser returns, if the driver named one: the one decision every chooser of the app makes
/// before its native dialog (model document open and save, handoff and report package, MBF import and export, and
/// the backup folder), through [`chooser_answer`]. `chooser` names the chooser in the strict stop's log line.
pub fn take_chosen_path<R: Runtime>(app: &AppHandle<R>, chooser: &str) -> Option<PathBuf> {
    let state = app.try_state::<SmokeState>();
    chooser_answer(state.as_deref(), chooser)
}

/// A chooser's decision, made without the app handle:
/// - an answer is queued: it is taken, except that in strict chooser mode an answer outside the home this process
///   was given ([`inside_home`]) ends the app here instead;
/// - nothing is queued: in strict chooser mode the app ends here, before that chooser's native dialog; otherwise
///   `None`, and the chooser shows its native dialog.
///
/// The answer is judged when it is taken, on resolved paths. The operations that follow open it by name, so a
/// change of that path after the check is not seen; nothing in the reviewed code makes such a change. The mode
/// governs these choosers only, not any other panel the app or the system can raise.
///
/// Ending the app logs which chooser was reached and why, never the answer's path, and exits STRICT_CHOOSER_EXIT.
/// With strict chooser mode off, a queued answer is taken wherever it points.
fn chooser_answer(state: Option<&SmokeState>, chooser: &str) -> Option<PathBuf> {
    let mut taken = None;
    if let Some(state) = state {
        if let Ok(mut chosen) = state.chosen.lock() {
            taken = chosen.pop_front();
        }
    }
    let mode = std::env::var(STRICT_CHOOSER_ENV).ok();
    if strict_stop(taken.is_some(), mode.as_deref()) {
        strict_end(chooser, "was asked with no answer queued");
    }
    if let Some(answer) = &taken {
        if mode.as_deref() == Some(STRICT_CHOOSER_VALUE)
            && !inside_home(answer, std::env::var_os("HOME").as_deref(), std::env::var_os("CFFIXED_USER_HOME").as_deref())
        {
            strict_end(chooser, "was given a queued answer outside the home");
        }
    }
    taken
}

/// Ends the app in strict chooser mode, naming the chooser and the reason (never a path).
fn strict_end(chooser: &str, reason: &str) -> ! {
    eprintln!("[native-smoke] strict chooser: {chooser} {reason}; ending the app before any native dialog, read or write (exit {STRICT_CHOOSER_EXIT})");
    std::process::exit(STRICT_CHOOSER_EXIT)
}

/// Whether a queued answer lies inside the home this process was given, for strict chooser mode. The home is the
/// value of `HOME`; with `HOME` unset, or `CFFIXED_USER_HOME` set to any other value, no answer is inside. Inside is
/// judged on resolved paths: the home resolved, and the answer resolved as far as it exists ([`resolved_answer`]),
/// so a link inside the home that points out of it is outside, and a path not yet created (a save target) is judged
/// by where it would be created. The home itself is inside. The temporary folder is not special: an answer under
/// `TMPDIR` is inside only because a confined launch puts `TMPDIR` inside the home.
fn inside_home(answer: &Path, home: Option<&OsStr>, fixed_home: Option<&OsStr>) -> bool {
    let Some(home) = home else { return false };
    if fixed_home.is_some_and(|fixed| fixed != home) {
        return false;
    }
    let Ok(home) = fs::canonicalize(home) else { return false };
    resolved_answer(answer).is_some_and(|answer| answer.starts_with(&home))
}

/// `path` resolved as far as it exists: its deepest existing ancestor resolved (links followed), then the components
/// below it appended. `None`, which refuses the answer, for a relative path, an existing ancestor that cannot be
/// resolved (a link whose target is missing included), or a `..` among the appended components. Looks up the path's
/// components; opens, reads and writes nothing.
fn resolved_answer(path: &Path) -> Option<PathBuf> {
    if !path.is_absolute() {
        return None;
    }
    for ancestor in path.ancestors() {
        match fs::symlink_metadata(ancestor) {
            Ok(_) => {
                let base = fs::canonicalize(ancestor).ok()?;
                let rest = path.strip_prefix(ancestor).ok()?;
                if rest.components().any(|component| !matches!(component, Component::Normal(_))) {
                    return None;
                }
                return Some(base.join(rest));
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => continue,
            Err(_) => return None,
        }
    }
    None
}

#[cfg(test)]
mod strict_chooser_tests {
    #[test]
    fn a_chooser_with_no_answer_stops_only_in_strict_mode() {
        use super::{strict_stop, STRICT_CHOOSER_VALUE};
        assert!(strict_stop(false, Some(STRICT_CHOOSER_VALUE)));
        assert!(!strict_stop(true, Some(STRICT_CHOOSER_VALUE)));
        assert!(!strict_stop(false, None));
        assert!(!strict_stop(false, Some("0")));
        assert!(!strict_stop(false, Some("")));
        assert_eq!(super::STRICT_CHOOSER_EXIT, 86);
    }

    use super::{chooser_answer, inside_home, SmokeState, STRICT_CHOOSER_ENV};
    use std::path::{Path, PathBuf};
    use std::process::{Command, Output};

    /// A test folder under this process's temporary folder, removed on every way the test ends (a failed
    /// assertion included): `home` and `outside` side by side, and in `home` a link to `outside` and a link
    /// whose target is missing.
    struct Folder {
        root: PathBuf,
    }

    impl Folder {
        fn new(name: &str) -> Self {
            static SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
            let sequence = SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let root = std::env::temp_dir().join(format!("swbpipe-chooser-test-{name}-{}-{sequence}", std::process::id()));
            let _ = std::fs::remove_dir_all(&root);
            let folder = Self { root };
            std::fs::create_dir_all(folder.home().join("tmp")).unwrap();
            std::fs::create_dir_all(folder.outside()).unwrap();
            std::fs::write(folder.home().join("inside.txt"), b"in").unwrap();
            std::fs::write(folder.outside().join("outside.txt"), b"out").unwrap();
            std::os::unix::fs::symlink(folder.outside(), folder.home().join("link-out")).unwrap();
            std::os::unix::fs::symlink(folder.outside().join("missing"), folder.home().join("dangling")).unwrap();
            folder
        }

        fn home(&self) -> PathBuf {
            self.root.join("home")
        }

        fn outside(&self) -> PathBuf {
            self.root.join("outside")
        }

        fn inside(&self, answer: &Path) -> bool {
            inside_home(answer, Some(self.home().as_os_str()), None)
        }
    }

    impl Drop for Folder {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    #[test]
    fn strict_chooser_home_an_answer_inside_is_inside() {
        let folder = Folder::new("inside");
        assert!(folder.inside(&folder.home().join("inside.txt")));
        assert!(folder.inside(&folder.home()));
        assert!(folder.inside(&folder.home().join("tmp/x.json")), "TMPDIR inside the home is inside by that fact");
    }

    #[test]
    fn strict_chooser_home_an_answer_outside_is_outside() {
        let folder = Folder::new("outside");
        assert!(!folder.inside(&folder.outside().join("outside.txt")));
        assert!(!folder.inside(&folder.outside().join("new.mbf")));
        // The system's temporary folder, not under the home, is outside.
        assert!(!folder.inside(&std::env::temp_dir().join("swbpipe-chooser-test-never-created")));
        assert!(!folder.inside(Path::new("inside.txt")), "a relative answer is refused");
    }

    #[test]
    fn strict_chooser_home_a_link_inside_pointing_out_is_outside() {
        let folder = Folder::new("link");
        assert!(!folder.inside(&folder.home().join("link-out")));
        assert!(!folder.inside(&folder.home().join("link-out/outside.txt")));
        assert!(!folder.inside(&folder.home().join("dangling")), "a link whose target is missing is refused");
    }

    #[test]
    fn strict_chooser_home_a_new_file_under_the_home_is_inside() {
        let folder = Folder::new("new");
        assert!(folder.inside(&folder.home().join("new.mbf")));
        assert!(folder.inside(&folder.home().join("not/yet/made/save.mbf")));
    }

    #[test]
    fn strict_chooser_home_a_new_file_under_a_link_out_is_outside() {
        let folder = Folder::new("new-link");
        assert!(!folder.inside(&folder.home().join("link-out/new.mbf")));
        assert!(!folder.inside(&folder.home().join("link-out/not/yet/save.mbf")));
    }

    #[test]
    fn strict_chooser_home_a_climbing_parent_component_is_outside() {
        let folder = Folder::new("climb");
        assert!(!folder.inside(&folder.home().join("missing/../../outside/new.mbf")));
        assert!(!folder.inside(&folder.home().join("../outside/outside.txt")));
        assert!(!folder.inside(&folder.home().join("missing/..")), "a remaining parent component is refused");
    }

    #[test]
    fn strict_chooser_home_unset_refuses_every_answer() {
        let folder = Folder::new("unset");
        assert!(!inside_home(&folder.home().join("inside.txt"), None, None));
    }

    #[test]
    fn strict_chooser_home_variables_that_differ_refuse_every_answer() {
        let folder = Folder::new("differ");
        let home = folder.home();
        let answer = home.join("inside.txt");
        assert!(!inside_home(&answer, Some(home.as_os_str()), Some(folder.outside().as_os_str())));
        assert!(inside_home(&answer, Some(home.as_os_str()), Some(home.as_os_str())));
    }

    const ENTRY: &str = "native_smoke::strict_chooser_tests::strict_chooser_entry";
    const ENTRY_ENV: &str = "SWBPIPE_TEST_STRICT_CHOOSER_ENTRY";
    const ANSWER_ENV: &str = "SWBPIPE_TEST_STRICT_CHOOSER_ANSWER";

    /// Runs [`chooser_answer`] in a child of this test binary whose environment is cleared and then holds only
    /// `env` (the variables it reads cannot be set in this process, whose other tests run on other threads).
    fn decide(answer: Option<&Path>, env: &[(&str, &std::ffi::OsStr)]) -> Output {
        let mut command = Command::new(std::env::current_exe().unwrap());
        command.args([ENTRY, "--exact", "--test-threads=1", "--nocapture"]).env_clear().env(ENTRY_ENV, "1");
        if let Some(answer) = answer {
            command.env(ANSWER_ENV, answer);
        }
        command.envs(env.iter().copied()).output().unwrap()
    }

    fn strict(home: &Path) -> Vec<(&'static str, &std::ffi::OsStr)> {
        vec![(STRICT_CHOOSER_ENV, std::ffi::OsStr::new("1")), ("HOME", home.as_os_str())]
    }

    fn assert_ended(output: &Output, reason: &str, path: Option<&Path>) {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(86), "{stdout}\n{stderr}");
        assert!(stderr.contains("strict chooser: test-chooser ") && stderr.contains(reason), "{stderr}");
        assert!(!stdout.contains("taken") && !stdout.contains("dialog"), "{stdout}");
        if let Some(path) = path {
            assert!(!stderr.contains(&*path.to_string_lossy()), "the log names the path: {stderr}");
        }
    }

    fn assert_returned(output: &Output, line: &str) {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{:?}\n{stdout}\n{stderr}", output.status);
        // libtest prints the entry's line after the test's name, on the same line.
        assert!(stdout.lines().any(|l| l.ends_with(&format!("... {line}"))), "expected {line:?}:\n{stdout}");
    }

    #[test]
    fn strict_chooser_decision_nothing_queued_ends_only_in_strict_mode() {
        let folder = Folder::new("none");
        assert_ended(&decide(None, &strict(&folder.home())), "was asked with no answer queued", None);
        assert_returned(&decide(None, &[("HOME", folder.home().as_os_str())]), "dialog");
        assert_returned(&decide(None, &[(STRICT_CHOOSER_ENV, std::ffi::OsStr::new("0"))]), "dialog");
    }

    #[test]
    fn strict_chooser_decision_an_answer_inside_is_taken() {
        let folder = Folder::new("taken");
        let answer = folder.home().join("new.mbf");
        assert_returned(&decide(Some(&answer), &strict(&folder.home())), &format!("taken {}", answer.display()));
    }

    #[test]
    fn strict_chooser_decision_an_answer_outside_ends_the_app_before_touching_it() {
        let folder = Folder::new("refused");
        let answer = folder.outside().join("new.mbf");
        assert_ended(&decide(Some(&answer), &strict(&folder.home())), "outside the home", Some(&answer));
        assert!(!answer.exists());
        let mut left: Vec<_> = std::fs::read_dir(folder.outside()).unwrap().map(|entry| entry.unwrap().file_name()).collect();
        left.sort();
        assert_eq!(left, ["outside.txt"]);
        let through_link = folder.home().join("link-out/new.mbf");
        assert_ended(&decide(Some(&through_link), &strict(&folder.home())), "outside the home", Some(&through_link));
    }

    #[test]
    fn strict_chooser_decision_home_unset_or_disagreeing_ends_the_app() {
        let folder = Folder::new("vars");
        let answer = folder.home().join("inside.txt");
        assert_ended(&decide(Some(&answer), &[(STRICT_CHOOSER_ENV, std::ffi::OsStr::new("1"))]), "outside the home", Some(&answer));
        let home = folder.home();
        let mut env = strict(&home);
        let other = folder.outside();
        env.push(("CFFIXED_USER_HOME", other.as_os_str()));
        assert_ended(&decide(Some(&answer), &env), "outside the home", Some(&answer));
    }

    #[test]
    fn strict_chooser_decision_mode_off_takes_an_answer_outside_unchanged() {
        let folder = Folder::new("off");
        let answer = folder.outside().join("outside.txt");
        assert_returned(&decide(Some(&answer), &[("HOME", folder.home().as_os_str())]), &format!("taken {}", answer.display()));
    }

    /// Run only by [`decide`] ([`ENTRY_ENV`]); otherwise it returns at once.
    #[test]
    fn strict_chooser_entry() {
        if std::env::var_os(ENTRY_ENV).is_none() {
            return;
        }
        let state = SmokeState::default();
        if let Some(answer) = std::env::var_os(ANSWER_ENV) {
            state.chosen.lock().unwrap().push_back(answer.into());
        }
        match chooser_answer(Some(&state), "test-chooser") {
            Some(path) => println!("taken {}", path.display()),
            None => println!("dialog"),
        }
    }

    /// Every function in this crate that raises a dialog through the dialog plugin (`.dialog()`) asks
    /// [`super::take_chosen_path`] first, so no chooser can reach its native dialog without the queue and the
    /// strict stop. The seven are named, so the scan cannot pass by finding none. Threat model: an honest edit that
    /// adds a chooser, or drops or moves the call; not a defence against a dialog raised by another route.
    #[test]
    fn strict_chooser_every_chooser_asks_the_queue_before_its_dialog() {
        use syn::visit::Visit;

        #[derive(Default)]
        struct Choosers {
            current: Vec<String>,
            /// Per function: the first `.dialog()` and first `take_chosen_path` positions (line, column).
            seen: std::collections::BTreeMap<String, (Option<(usize, usize)>, Option<(usize, usize)>)>,
        }

        impl Choosers {
            fn note(&mut self, dialog: bool, at: proc_macro2::LineColumn) {
                let Some(name) = self.current.last().cloned() else { return };
                let entry = self.seen.entry(name).or_default();
                let slot = if dialog { &mut entry.0 } else { &mut entry.1 };
                let at = (at.line, at.column);
                if slot.map_or(true, |first| at < first) {
                    *slot = Some(at);
                }
            }
        }

        impl<'ast> Visit<'ast> for Choosers {
            fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
                self.current.push(node.sig.ident.to_string());
                syn::visit::visit_item_fn(self, node);
                self.current.pop();
            }
            fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
                self.current.push(node.sig.ident.to_string());
                syn::visit::visit_impl_item_fn(self, node);
                self.current.pop();
            }
            fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
                if node.method == "dialog" && node.args.is_empty() {
                    self.note(true, node.method.span().start());
                }
                syn::visit::visit_expr_method_call(self, node);
            }
            fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
                if let syn::Expr::Path(path) = &*node.func {
                    if let Some(last) = path.path.segments.last().filter(|last| last.ident == "take_chosen_path") {
                        self.note(false, last.ident.span().start());
                    }
                }
                syn::visit::visit_expr_call(self, node);
            }
        }

        fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
            for entry in std::fs::read_dir(dir).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    sources(&path, out);
                } else if path.extension().is_some_and(|extension| extension == "rs") {
                    out.push(path);
                }
            }
        }

        let mut files = Vec::new();
        sources(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), &mut files);
        let mut choosers = std::collections::BTreeMap::new();
        for file in files {
            let source = std::fs::read_to_string(&file).unwrap();
            let tree = syn::parse_file(&source).unwrap_or_else(|error| panic!("{}: {error}", file.display()));
            let mut visitor = Choosers::default();
            visitor.visit_file(&tree);
            let name = file.file_name().unwrap().to_string_lossy().into_owned();
            for (function, (dialog, take)) in visitor.seen {
                if let Some(dialog) = dialog {
                    assert!(take.is_some_and(|take| take < dialog), "{name}: {function} raises a dialog without asking the queue first");
                    choosers.insert(function, name.clone());
                }
            }
        }
        let names: Vec<(&str, &str)> = choosers.iter().map(|(function, file)| (function.as_str(), file.as_str())).collect();
        assert_eq!(
            names,
            [
                ("choose_backup_folder", "backup.rs"),
                ("choose_handoff_package_path", "lib.rs"),
                ("choose_mbf_import_path", "lib.rs"),
                ("choose_mbf_save_path", "lib.rs"),
                ("choose_model_document_path", "lib.rs"),
                ("choose_model_document_save_path", "lib.rs"),
                ("choose_report_package_save_path", "lib.rs"),
            ]
        );
    }
}

#[tauri::command]
pub fn native_smoke_choose(app: AppHandle, path: String) -> Result<(), String> {
    app.state::<SmokeState>()
        .chosen
        .lock()
        .map_err(|_| "native smoke state poisoned".to_string())?
        .push_back(PathBuf::from(path));
    Ok(())
}

/// Drops every chooser path not yet taken, so a save refused before its chooser
/// cannot answer a later one.
#[tauri::command]
pub fn native_smoke_clear_choices(app: AppHandle) -> Result<(), String> {
    app.state::<SmokeState>()
        .chosen
        .lock()
        .map_err(|_| "native smoke state poisoned".to_string())?
        .clear();
    Ok(())
}

#[tauri::command]
pub fn native_smoke_menu(app: AppHandle, command_id: String) {
    crate::dispatch_native_menu_command(&app, &command_id);
}

#[tauri::command]
pub fn native_smoke_plan() -> Result<Value, String> {
    let text = std::env::var("SWBPIPE_NATIVE_SMOKE_PLAN").map_err(|_| "SWBPIPE_NATIVE_SMOKE_PLAN unset".to_string())?;
    serde_json::from_str(&text).map_err(|error| error.to_string())
}

/// Another writer saves this stored project, as a second window or process
/// would: the same model and review context, without the run. It goes through
/// the product's own persistence functions, so the stored revision (and its
/// envelope hash) moves exactly as a real save would move it.
#[tauri::command]
pub fn native_smoke_external_save(app: AppHandle, project_id: String) -> Result<(), String> {
    let path = crate::app_store_path(&app)?;
    let (mut connection, _) = crate::open_project_store(&path)?;
    let record = crate::load_project(&connection, Some(&project_id))?
        .ok_or_else(|| format!("{project_id} is not stored"))?;
    let none = Value::Null;
    let envelope_hash = crate::computed_project_envelope_hash(
        &project_id, &record.model, &record.editor_intents, &record.proposal,
        &record.selected_review_target, &none, &none, &record.model_hash,
    );
    crate::upsert_project(
        &mut connection, &project_id, &record.project_name, &record.model, &record.editor_intents,
        &record.proposal, &record.selected_review_target, &none, &none, &record.model_hash,
        &envelope_hash,
    )
}

#[tauri::command]
pub fn native_smoke_log(line: String) {
    eprintln!("[native-smoke] {line}");
}

#[tauri::command]
pub fn native_smoke_finish(app: AppHandle, report: Value) -> Result<(), String> {
    let path = std::env::var("SWBPIPE_NATIVE_SMOKE_REPORT").map_err(|_| "SWBPIPE_NATIVE_SMOKE_REPORT unset".to_string())?;
    let text = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())?;
    fs::write(&path, text).map_err(|error| format!("{path}: {error}"))?;
    app.exit(0);
    Ok(())
}

/// Whether stdin carries a provider key: only when the variable is absent is it not, and
/// only the exact value `stdin` with the driver on stdin selects it. Any other value,
/// including one that is not Unicode, is refused (no driver runs), so a key line can
/// never fall through to the driver reader. The value is never echoed.
#[cfg(target_os = "macos")]
fn provider_key_on_stdin(value: Option<std::ffi::OsString>, driver: &str) -> Result<bool, ()> {
    match value {
        None => Ok(false),
        Some(value) if value == "stdin" && driver == "-" => Ok(true),
        Some(_) => Err(()),
    }
}

#[cfg(target_os = "macos")]
static PROVIDER_SECRET: Mutex<Option<crate::embedded_agent::relay_policy::Secret>> = Mutex::new(None);

/// The provider key the controller passed on stdin, taken at most once.
#[cfg(target_os = "macos")]
pub fn take_provider_secret() -> Option<crate::embedded_agent::relay_policy::Secret> {
    PROVIDER_SECRET.lock().ok()?.take()
}

/// Splits `key\n` from the driver that follows it. The key line is bounded by
/// the relay's secret limit and must satisfy `Secret::new`; nothing about a
/// refused key is echoed.
#[cfg(target_os = "macos")]
fn read_stdin_key_and_driver(mut reader: impl io::BufRead) -> io::Result<(crate::embedded_agent::relay_policy::Secret, String)> {
    use crate::embedded_agent::relay_policy::{Secret, SECRET_BYTES};
    use std::io::BufRead;
    let mut line = Vec::new();
    (&mut reader).take(SECRET_BYTES as u64 + 2).read_until(b'\n', &mut line)?;
    let refused = || io::Error::new(io::ErrorKind::InvalidData, "native smoke provider key refused");
    if line.pop() != Some(b'\n') {
        return Err(refused());
    }
    let secret = String::from_utf8(line).ok().and_then(|text| Secret::new(text).ok()).ok_or_else(refused)?;
    Ok((secret, read_stdin_driver(reader)?))
}

fn read_stdin_driver(reader: impl Read) -> io::Result<String> {
    let mut script = String::new();
    reader.take(256 * 1024 + 1).read_to_string(&mut script)?;
    if script.len() > 256 * 1024 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "native smoke driver exceeds its input limit"));
    }
    Ok(script)
}
#[cfg(test)]
mod input_tests {
    #[test]
    fn anonymous_driver_input_is_bounded_without_a_file_or_environment_payload() {
        assert_eq!(super::read_stdin_driver(&b"void 0;"[..]).unwrap(), "void 0;");
        assert!(super::read_stdin_driver(&vec![b'x'; 256 * 1024 + 1][..]).is_err());
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn the_provider_key_line_precedes_the_driver_and_is_checked() {
        use crate::embedded_agent::relay_policy::SECRET_BYTES;
        let (secret, driver) = super::read_stdin_key_and_driver(&b"invented-fixture-key\nvoid 0;"[..]).unwrap();
        assert_eq!(format!("{secret:?}"), "Secret([REDACTED])");
        assert_eq!(driver, "void 0;");
        let long = [vec![b'k'; SECRET_BYTES + 1], b"\nvoid 0;".to_vec()].concat();
        for input in [&b"no newline"[..], b"\nvoid 0;", b"two words\nvoid 0;", b"tab\tkey\nvoid 0;", &long] {
            let error = super::read_stdin_key_and_driver(input).unwrap_err();
            assert_eq!(error.to_string(), "native smoke provider key refused");
        }
        let oversized = [b"k\n".to_vec(), vec![b'x'; 256 * 1024 + 1]].concat();
        assert!(super::read_stdin_key_and_driver(&oversized[..]).is_err());
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn only_an_absent_variable_or_the_exact_value_stdin_is_accepted() {
        use std::{ffi::OsString, os::unix::ffi::OsStringExt};
        assert_eq!(super::provider_key_on_stdin(None, "-"), Ok(false));
        assert_eq!(super::provider_key_on_stdin(None, "driver.js"), Ok(false));
        assert_eq!(super::provider_key_on_stdin(Some("stdin".into()), "-"), Ok(true));
        // A key line must never reach the driver reader: every other present value refuses.
        for value in [OsString::from("stdin"), OsString::from("STDIN"), OsString::from(""), OsString::from("stdin "), OsString::from_vec(vec![0xff, b's'])] {
            let file = value == "stdin";
            assert_eq!(super::provider_key_on_stdin(Some(value), if file { "driver.js" } else { "-" }), Err(()));
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn the_provider_key_is_taken_once() {
        use crate::embedded_agent::relay_policy::Secret;
        *super::PROVIDER_SECRET.lock().unwrap() = Some(Secret::new("invented-fixture-key".into()).unwrap());
        assert!(super::take_provider_secret().is_some());
        assert!(super::take_provider_secret().is_none());
    }
}

/// Manage the smoke state and start the driver after the first completed page load.
pub fn install<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    builder.manage(SmokeState::default()).on_page_load(|webview, payload| {
        if payload.event() != PageLoadEvent::Finished {
            return;
        }
        let state = webview.app_handle().state::<SmokeState>();
        let Ok(mut started) = state.driver_started.lock() else { return };
        if *started {
            return;
        }
        let Ok(driver) = std::env::var("SWBPIPE_NATIVE_SMOKE_DRIVER") else {
            eprintln!("[native-smoke] SWBPIPE_NATIVE_SMOKE_DRIVER unset; no driver started");
            return;
        };
        #[cfg(target_os = "macos")]
        let Ok(with_key) = provider_key_on_stdin(std::env::var_os("SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY"), &driver) else {
            eprintln!("[native-smoke] SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY needs the value stdin and the driver on stdin; no driver started");
            return;
        };
        #[cfg(target_os = "macos")]
        if with_key {
            match read_stdin_key_and_driver(io::stdin().lock()) {
                Ok((secret, script)) => {
                    let Ok(mut held) = PROVIDER_SECRET.lock() else { return };
                    *held = Some(secret);
                    drop(held);
                    *started = true;
                    if let Err(error) = webview.eval(&script) {
                        eprintln!("[native-smoke] driver eval failed: {error}");
                    }
                }
                Err(error) => eprintln!("[native-smoke] stdin: {error}; no driver started"),
            }
            return;
        }
        match if driver == "-" { read_stdin_driver(io::stdin().lock()) } else { fs::read_to_string(&driver) } {
            Ok(script) => {
                *started = true;
                if let Err(error) = webview.eval(&script) {
                    eprintln!("[native-smoke] driver eval failed: {error}");
                }
            }
            Err(error) => eprintln!("[native-smoke] {driver}: {error}"),
        }
    })
}

/// Runs the product's handoff verifier (`verify_handoff_package`, DEL-15-01
/// W-6b) on a saved package, so the smoke checks what the save wrote with the
/// verifier the design names.
#[tauri::command]
pub fn native_smoke_verify_handoff_package(path: String) -> Result<crate::handoff_package_publish::HandoffVerification, String> {
    crate::handoff_package_publish::verify_handoff_package(std::path::Path::new(&path))
}

/// Re-reads a saved report package with the product's reader
/// (`read_report_package_container`, DEL-08-01 W-18) and reports what the smoke
/// checks: the file's SHA-256, the manifest's member inventory beside each
/// member's recomputed SHA-256 and length, and the result member's text.
#[tauri::command]
pub fn native_smoke_read_report_package(path: String) -> Result<Value, String> {
    use sha2::{Digest, Sha256};
    let bytes = fs::read(&path).map_err(|error| format!("{path}: {error}"))?;
    let read = open_pipe_stress_report_package::read_report_package_container(&bytes).map_err(|refusal| refusal.to_string())?;
    let hex = |data: &[u8]| format!("{:x}", Sha256::digest(data));
    let members: Vec<Value> = read
        .members
        .iter()
        .map(|member| {
            serde_json::json!({
                "role": member.role,
                "file_name": member.file_name,
                "byte_length": member.bytes.len(),
                "sha256_hex": hex(&member.bytes),
            })
        })
        .collect();
    let result_member = read
        .members
        .iter()
        .find(|member| member.role == open_pipe_stress_report_package::ROLE_RESULT_EXPORT)
        .map(|member| String::from_utf8_lossy(&member.bytes).into_owned());
    Ok(serde_json::json!({
        "file_sha256_hex": hex(&bytes),
        "file_byte_length": bytes.len(),
        "schema_version": read.schema_version,
        "export_profile_id": read.export_profile_id,
        "export_blocked": read.export_blocked,
        "package_identity_sha256_hex": read.package_identity_sha256_hex,
        "inventory": read.manifest["members"],
        "members": members,
        "result_member_text": result_member,
    }))
}

/// C1 P3, smoke builds only. Reads the current run's held bytes and the model
/// path already selected by this launch's plan; returns scalar allocation
/// metrics only. No result/model text or standing is published by this probe.
#[tauri::command]
pub async fn native_smoke_profile_readers(
    registry: tauri::State<'_, crate::solve_jobs::SolveJobRegistry>,
    invoke: tauri::ipc::Request<'_>,
) -> Result<tauri::ipc::Response, crate::command_ingress::CommandError<String>> {
    use crate::{command_ingress, ingress};
    let command = "native_smoke_profile_readers";
    let raw = command_ingress::raw_args_body(invoke.body(), invoke.headers(), command)?;
    let lease = registry.acquire_interactive_ingress_capped(
        ingress::allocation_envelope(raw.len() as u64).unwrap_or(u64::MAX),
    )?;
    let raw = raw.to_vec();
    let registry = registry.inner().clone();
    tauri::async_runtime::spawn_blocking(move || open_pipe_stress_result_export::recorded_tables::entry_scope(move || profile_readers(&registry, raw, lease)))
        .await
        .map_err(|_| {
            command_ingress::CommandError::Command(
                "C1-PROFILE-FAILED: blocking probe failed".into(),
            )
        })?
        .map(tauri::ipc::Response::new)
}

fn profile_readers(
    registry: &crate::solve_jobs::SolveJobRegistry,
    raw: Vec<u8>,
    copy: crate::memory_pool::Lease,
) -> Result<String, crate::command_ingress::CommandError<String>> {
    use crate::{
        command_ingress::{self, ArgsDecoding, CommandError},
        ingress, result_js_text,
        solve_jobs::sha256_in_place,
    };
    use open_pipe_stress_result_export::reader_verdict as rv;
    use serde_json::json;
    use sha2::{Digest, Sha256};
    use std::io::Read;
    let fail = |message: &str| CommandError::Command(format!("C1-PROFILE-REFUSED: {message}"));
    let copy_bytes = copy.bytes();
    let (mut args, _args_lease) = command_ingress::admit_raw(
        registry,
        &raw,
        "native_smoke_profile_readers",
        ArgsDecoding::Moved,
        |bytes| {
            let mut lease = copy;
            lease
                .grow_to(copy_bytes.saturating_add(bytes))
                .map_err(|refusal| registry.refuse(refusal))?;
            Ok(lease)
        },
    )?;
    drop(raw);
    let job: String = args.decode("jobId")?;
    drop(args);
    // Rejected before any model file is opened or allocation measurement starts.
    let binding = registry
        .custody_binding(&job)
        .ok_or_else(|| fail("a fresh held solve is required"))?;
    let held = registry.held_result(&job)?;
    if sha256_in_place(held.text().as_bytes()) != binding.result_sha256 {
        return Err(fail("publication digest mismatch"));
    }
    let plan = native_smoke_plan().map_err(CommandError::Command)?;
    if plan["c1_probes"] != true || plan["c1_profile_readers"] != true {
        return Err(fail("the launch plan did not select the C1 reader probe"));
    }
    let path = plan["model_path"]
        .as_str()
        .ok_or_else(|| fail("the launch has no model path"))?;
    let mut file = fs::File::open(path).map_err(|_| fail("the launch model cannot be opened"))?;
    let file_len = file
        .metadata()
        .map_err(|_| fail("model metadata unavailable"))?
        .len();
    let _file_lease = registry
        .acquire_result_parse(ingress::allocation_envelope(file_len).unwrap_or(u64::MAX))?;
    let mut bytes = vec![0; usize::try_from(file_len).map_err(|_| fail("model length overflow"))?];
    file.read_exact(&mut bytes)
        .map_err(|_| fail("the launch model length changed"))?;
    let mut extra = [0u8; 1];
    if file
        .read(&mut extra)
        .map_err(|_| fail("model read failed"))?
        != 0
    {
        return Err(fail("the launch model grew"));
    }
    let model_text = String::from_utf8(bytes).map_err(|_| fail("the launch model is not UTF-8"))?;
    let started = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let model_bound = ingress::prescan(model_text.as_bytes())
        .map_err(|_| fail("the launch model does not pass ingress"))?;
    let _model_parse = registry.acquire_result_parse(model_bound.materialization_bytes)?;
    let model = serde_json::from_str::<Value>(&model_text)
        .map_err(|_| fail("the launch model is not JSON"))?;
    // JSON.stringify of the maintained file must be the exact model intake the
    // solve admitted. The writer's scratch is admitted before its counting pass.
    let _scratch = registry.acquire_result_parse(result_js_text::scratch_bytes(&model))?;
    let mut hash = Sha256::new();
    result_js_text::write(&model_text, result_js_text::NegativeZero::AsZero, &mut hash)
        .map_err(CommandError::Command)?;
    if <[u8; 32]>::from(hash.finalize()) != binding.intake_sha256 {
        return Err(fail("the launch model is not the solve's exact intake"));
    }
    let normalized_len = result_js_text::length(&model_text, result_js_text::NegativeZero::AsZero)
        .map_err(CommandError::Command)?;
    let _normalized_text_lease = registry
        .acquire_result_parse(ingress::allocation_envelope(normalized_len).unwrap_or(u64::MAX))?;
    let mut normalized_text = Vec::with_capacity(
        usize::try_from(normalized_len).map_err(|_| fail("model length overflow"))?,
    );
    result_js_text::write(
        &model_text,
        result_js_text::NegativeZero::AsZero,
        &mut normalized_text,
    )
    .map_err(CommandError::Command)?;
    drop((model, model_text, _file_lease, _scratch));
    let (model, model_metrics) =
        profile_phase(|| serde_json::from_slice::<Value>(&normalized_text));
    let model = model.map_err(|_| fail("normalized model is not JSON"))?;
    let prefix = b"{\"request\":{\"model\":";
    let suffix = format!(
        ",\"materials\":[]}},\"solver_mode\":\"{}\"}}",
        binding.solver_mode.as_str()
    );
    let invocation_len = normalized_len
        .saturating_add(prefix.len() as u64)
        .saturating_add(suffix.len() as u64);
    let _invocation_text = registry
        .acquire_result_parse(ingress::allocation_envelope(invocation_len).unwrap_or(u64::MAX))?;
    let mut invocation_text = Vec::with_capacity(
        usize::try_from(invocation_len).map_err(|_| fail("invocation length overflow"))?,
    );
    invocation_text.extend_from_slice(prefix);
    invocation_text.extend_from_slice(&normalized_text);
    invocation_text.extend_from_slice(suffix.as_bytes());
    let invocation_bound =
        ingress::prescan(&invocation_text).map_err(|_| fail("invocation does not pass ingress"))?;
    let _invocation_parse =
        registry.acquire_result_parse(invocation_bound.materialization_bytes)?;
    let (invocation, invocation_metrics) =
        profile_phase(|| serde_json::from_slice::<Value>(&invocation_text));
    let invocation = invocation.map_err(|_| fail("invocation parse failed"))?;
    drop((
        normalized_text,
        _normalized_text_lease,
        invocation_text,
        _invocation_text,
    ));
    let source_bound = ingress::prescan(held.text().as_bytes())
        .map_err(|_| fail("held source does not pass ingress"))?;
    let _source_parse = registry.acquire_result_parse(source_bound.materialization_bytes)?;
    let (source, source_metrics) = profile_phase(|| serde_json::from_str::<Value>(held.text()));
    let source = source.map_err(|_| fail("held source parse failed"))?;
    let (verdict, dispatch_metrics) =
        profile_phase(|| rv::dispatch(&source, Some(&model), Some(&invocation)));
    if verdict["ok"] != true {
        return Err(fail("the held source dispatch refused"));
    }
    let route = verdict["route"]
        .as_str()
        .ok_or_else(|| fail("dispatch returned no route"))?;
    let reader = match route {
        "preview_physics" => "preview-physics",
        "pressure_source" => "pressure-source",
        "pressure" => "pressure",
        _ => {
            return Err(fail(
                "the probe only covers the desktop's three fresh registration routes",
            ))
        }
    };
    let (validation, registration_metrics) = profile_phase(|| {
        if route == "pressure_source" {
            rv::validate(reader, &source, None, Some(&invocation))
        } else {
            rv::validate(reader, &source, Some(&model), None)
        }
    });
    if validation["ok"] != true {
        return Err(fail("the registration reader refused"));
    }
    drop(validation);
    // Borrow every bounded batch directly from the same held parse; no batch
    // tree is cloned. Keep maximum working peaks, not a sum of batch peaks.
    let rows = source["results"]
        .as_array()
        .ok_or_else(|| fail("the source has no rows"))?;
    let row_count = rows.len();
    let mut semantics = ProfilePhase::default();
    let mut batches = 0usize;
    for chunk in rows.chunks(1000) {
        let (outcome, measured) = profile_phase(|| rv::row_semantics_each(chunk, Some(&source)));
        if outcome
            .as_array()
            .is_none_or(|items| items.iter().any(|item| item["ok"] != true))
        {
            return Err(fail("row semantics refused"));
        }
        drop(outcome);
        semantics.merge(measured);
        batches += 1;
    }
    let findings_text = serde_json::to_string(&verdict["standing"]["findings"])
        .map_err(|_| fail("findings serialization failed"))?;
    let eligible = verdict["standing"]["eligible"]
        .as_bool()
        .ok_or_else(|| fail("dispatch has no eligibility"))?;
    // The engine's health Value entry is private. Report the combined phase
    // and a labelled working upper bound; do not call it a direct working peak.
    let route = route.to_owned();
    drop((source, verdict));
    let (health_text, health_metrics) = profile_phase(|| {
        open_pipe_stress_operation_applier::numerical_health_json(held.text(), Some(&findings_text))
    });
    let health_text = health_text.map_err(CommandError::Command)?;
    let health_bound = ingress::prescan(health_text.as_bytes())
        .map_err(|_| fail("health reply failed ingress"))?;
    let _health_lease =
        registry.acquire_result_parse(health_bound.materialization_bytes.saturating_add(
            ingress::allocation_envelope(health_text.len() as u64).unwrap_or(u64::MAX),
        ))?;
    let (health, health_parse_metrics) =
        profile_phase(|| serde_json::from_str::<Value>(&health_text));
    let health = health.map_err(|_| fail("health parse failed"))?;
    drop(health_text);
    let (source, summary_parse_metrics) =
        profile_phase(|| serde_json::from_str::<Value>(held.text()));
    let source = source.map_err(|_| fail("summary source parse failed"))?;
    let (standing, health_view_metrics) =
        profile_phase(|| crate::result_summary_tables::Standing {
            eligible,
            health: crate::result_summary_tables::health_view(&health),
        });
    let (summary, summary_metrics) = profile_phase(|| {
        let presentation = crate::diagnostics_presentation::facts(held.text(), &source, held.recorded_table(), Some(held.presentation()));
        crate::result_summary::build(&job, &source, &model, Some(&standing), &health, None, &presentation, |bytes| {
            registry.acquire_result_parse(bytes)
        })
    });
    let (summary, summary_lease) = summary?;
    let summary_bytes = summary.len();
    drop((summary, summary_lease, standing, health, source));
    // Reset/disposal invalidates the observation even though our Arc kept its
    // bytes safe. This probe never re-registers a handle or grants authority.
    if registry.custody_binding(&job) != Some(binding) {
        return Err(fail("custody changed during the probe"));
    }
    let finished = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let metrics = json!({
        "schema":"swbpipe.c1-reader-allocations/v1", "started_ms":started, "finished_ms":finished,
        "job_id":job, "route":route, "reader":reader, "source_bytes":held.text().len(), "rows":row_count,
        "publication_sha256":format!("sha256:{}", binding.result_sha256.iter().map(|byte| format!("{byte:02x}")).collect::<String>()),
        "intake_sha256":format!("sha256:{}", binding.intake_sha256.iter().map(|byte| format!("{byte:02x}")).collect::<String>()),
        "source_parse_bound_bytes":source_bound.materialization_bytes,
        "model_parse_bound_bytes":model_bound.materialization_bytes,
        "invocation_parse_bound_bytes":invocation_bound.materialization_bytes,
        "model_parse":model_metrics, "invocation_parse":invocation_metrics, "source_parse":source_metrics,
        "dispatch_working":dispatch_metrics, "registration_working":registration_metrics,
        "semantics_working_max":semantics, "semantics_batches":batches, "semantics_batch_limit":1000,
        "health_parse_and_working":health_metrics,
        "health_working_upper_bound_requested":health_metrics.allocation.peak_requested.saturating_sub(source_metrics.allocation.live_requested.max(0) as u64),
        "health_working_upper_bound_priced":health_metrics.allocation.peak_priced.saturating_sub(source_metrics.allocation.live_priced.max(0) as u64),
        "health_response_parse":health_parse_metrics, "health_view_working":health_view_metrics,
        "summary_source_parse":summary_parse_metrics, "summary_working_and_response":summary_metrics,
        "summary_response_bytes":summary_bytes,
        "coverage":"per-thread Rust requested sizes and allocation_envelope; realloc counts old+new; no other threads, native-library allocations, caches, stacks, RSS or delivery; health working is an upper bound from combined peak minus directly measured tree-live bytes"
    });
    let _reply_lease = registry.acquire_result_parse(64 * 1024)?;
    let reply =
        serde_json::to_string(&metrics).map_err(|_| fail("metrics serialization failed"))?;
    if reply.len() > 64 * 1024 {
        return Err(fail("metrics exceed 64 KiB"));
    }
    Ok(reply)
}

#[derive(Default, Clone, Copy, Serialize)]
struct ProfilePhase {
    allocation: crate::ingress::counting::Measured,
    elapsed_us: u128,
}

impl ProfilePhase {
    fn merge(&mut self, other: Self) {
        self.allocation.peak_requested = self
            .allocation
            .peak_requested
            .max(other.allocation.peak_requested);
        self.allocation.peak_priced = self
            .allocation
            .peak_priced
            .max(other.allocation.peak_priced);
        self.allocation.allocations += other.allocation.allocations;
        self.allocation.live_requested = self
            .allocation
            .live_requested
            .max(other.allocation.live_requested);
        self.allocation.live_priced = self
            .allocation
            .live_priced
            .max(other.allocation.live_priced);
        self.elapsed_us += other.elapsed_us;
    }
}

fn profile_phase<T>(read: impl FnOnce() -> T) -> (T, ProfilePhase) {
    let start = std::time::Instant::now();
    let (outcome, allocation) = crate::ingress::counting::measure(read);
    (
        outcome,
        ProfilePhase {
            allocation,
            elapsed_us: start.elapsed().as_micros(),
        },
    )
}

#[cfg(test)]
mod reader_probe_tests {
    #[test]
    fn unknown_handle_refuses_before_file_access_or_measurement() {
        use crate::{
            ingress,
            memory_pool::{test_host::ScriptedHost, MemoryPool},
            solve_jobs::SolveJobRegistry,
        };
        let registry = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
        let raw = br#"{"jobId":"not-a-held-job"}"#.to_vec();
        let lease = registry
            .acquire_interactive_ingress_capped(
                ingress::allocation_envelope(raw.len() as u64).unwrap(),
            )
            .unwrap();
        let error = super::profile_readers(&registry, raw, lease).unwrap_err();
        assert!(format!("{error:?}").contains("fresh held solve is required"));
        assert_eq!(registry.pool_status().reserved_bytes, 0);
    }
}

// C1 diagnostic evidence: fixed phases, launch-local counters, no input/identity
// text. stderr is consumed and synchronously persisted by the existing runner.
static READER_DIAGNOSTIC_CALLS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static READER_DIAGNOSTIC_EVENTS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
static READER_DIAGNOSTIC_ORIGIN: std::sync::OnceLock<std::time::Instant> =
    std::sync::OnceLock::new();

pub(crate) fn reader_diagnostic_call() -> u64 {
    if std::env::var_os("SWBPIPE_NATIVE_SMOKE_READER_DIAGNOSTICS").as_deref()
        != Some(std::ffi::OsStr::new("1"))
    {
        return 0;
    }
    READER_DIAGNOSTIC_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1
}

fn reader_diagnostic_event(
    call: u64,
    phase: &'static str,
    edge: &'static str,
    status: &'static str,
    bytes: u64,
) {
    if call == 0 {
        return;
    }
    #[cfg(test)]
    reader_phase_tests::record(phase, edge, status);
    let sequence = READER_DIAGNOSTIC_EVENTS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    if sequence > 256 {
        return;
    }
    if sequence == 256 || call > 32 {
        if sequence == 256 || call == 33 && edge == "enter" && phase == "args_admit" {
            eprintln!("[native-smoke] reader-phase:{{\"layer\":\"native\",\"overflow\":true}}");
        }
        return;
    }
    let mono_ms = READER_DIAGNOSTIC_ORIGIN
        .get_or_init(std::time::Instant::now)
        .elapsed()
        .as_secs_f64()
        * 1000.0;
    eprintln!("[native-smoke] reader-phase:{{\"layer\":\"native\",\"call\":{call},\"phase\":\"{phase}\",\"edge\":\"{edge}\",\"status\":\"{status}\",\"mono_ms\":{mono_ms},\"bytes\":{bytes}}}");
}

pub(crate) fn reader_diagnostic_phase<T, E>(
    call: u64,
    phase: &'static str,
    bytes: u64,
    work: impl FnOnce() -> Result<T, E>,
) -> Result<T, E> {
    struct Exit {
        call: u64,
        phase: &'static str,
        bytes: u64,
        done: bool,
    }
    impl Drop for Exit {
        fn drop(&mut self) {
            if !self.done {
                reader_diagnostic_event(self.call, self.phase, "exit", "incomplete", self.bytes);
            }
        }
    }
    reader_diagnostic_event(call, phase, "enter", "pending", bytes);
    let mut exit = Exit {
        call,
        phase,
        bytes,
        done: false,
    };
    let result = work();
    reader_diagnostic_event(
        call,
        phase,
        "exit",
        if result.is_ok() { "ok" } else { "error" },
        bytes,
    );
    exit.done = true;
    result
}

#[cfg(test)]
mod reader_phase_tests {
    use super::*;
    thread_local! {
        static EVENTS: std::cell::RefCell<Vec<(&'static str, &'static str, &'static str)>> = const { std::cell::RefCell::new(Vec::new()) };
    }
    pub(super) fn record(phase: &'static str, edge: &'static str, status: &'static str) {
        EVENTS.with(|events| {
            let mut events = events.borrow_mut();
            if events.len() < 256 {
                events.push((phase, edge, status));
            }
        });
    }
    fn take() -> Vec<(&'static str, &'static str, &'static str)> {
        EVENTS.with(|events| std::mem::take(&mut *events.borrow_mut()))
    }
    #[test]
    fn entries_precede_every_native_phase_and_failed_or_unwound_exits_are_honest() {
        for phase in [
            "args_admit",
            "source_prescan",
            "source_admit",
            "inputs_admit",
            "source_parse",
            "inputs_parse",
            "validate",
            "serialize",
            "source_drop",
            "summary_health",
            "summary_build",
        ] {
            take();
            let result = reader_diagnostic_phase(1, phase, 10, || {
                assert_eq!(take(), vec![(phase, "enter", "pending")]);
                Err::<(), _>("fixture cancelled/disposed")
            });
            assert!(result.is_err());
            assert_eq!(take(), vec![(phase, "exit", "error")]);
            let panic = std::panic::catch_unwind(|| {
                reader_diagnostic_phase(1, phase, 10, || -> Result<(), ()> {
                    assert_eq!(take(), vec![(phase, "enter", "pending")]);
                    panic!("fixture native task failure")
                })
            });
            assert!(panic.is_err());
            assert_eq!(take(), vec![(phase, "exit", "incomplete")]);
        }
    }
}
