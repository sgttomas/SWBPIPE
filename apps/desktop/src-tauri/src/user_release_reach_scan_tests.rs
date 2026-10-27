//! The reach scan on the real sources (W-18b slice (r2c); DEL-08-01 Design
//! `report-user-release.md` §2.8, §6). Standard claim fence applies (claims
//! taxonomy per DEC-081).

use crate::user_release_reach_scan::{project_sources, scan, scan_reading, Sources, CRATES};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

fn project() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

#[test]
fn user_release_reach_scan() {
    let sources = project_sources(&project());
    let findings = scan(&sources);
    assert!(findings.is_empty(), "{findings:#?}");
}

/// The scan reads every production file it must, including the modules under
/// a non-test `cfg` (§2.8, "What the four crates hold today"), and no
/// test-only file.
#[test]
fn the_scan_reads_the_production_files() {
    let (findings, read) = scan_reading(&project_sources(&project()));
    assert!(findings.is_empty(), "{findings:#?}");
    for file in [
        "src-tauri/src/main.rs",
        "src-tauri/src/native_smoke.rs",
        "src-tauri/src/host_memory.rs",
        "src-tauri/src/report_package_save.rs",
        "src-tauri/src/report_package_bridge.rs",
        "report_package/src/user_release.rs",
        "report_package/src/wire.rs",
        "report_package/src/bindings.rs",
        "report_package/src/reader.rs",
        "pdf_emitter/src/lib.rs",
        "report_renderer/src/object_form.rs",
    ] {
        assert!(read.contains(file), "{file}: {read:?}");
    }
    for file in [
        "src-tauri/src/user_release_reach_scan.rs",
        "src-tauri/src/user_release_reach_scan_tests.rs",
        "src-tauri/src/user_release_save_tests.rs",
        "src-tauri/src/report_render_exact_tests.rs",
        "report_package/src/admitted_tests.rs",
        "report_package/src/forwarding_graph_scan.rs",
    ] {
        assert!(!read.contains(file), "{file} is test-only");
    }
}

/// Also asserted (§2.8): the manifests that depend on the renderer are
/// exactly `src-tauri`'s, `pdf_emitter`'s and `report_package`'s, of which
/// only `report_package`'s depends on `pdf_emitter`. That no source under the
/// headless runner names the admitting entry or the bridge function is the
/// runner's own test (`core/runner/headless/tests/user_release_runner_refused.rs`),
/// so this crate's tests read no runner source.
#[test]
fn only_the_three_crates_depend_on_the_renderer() {
    fn walk(dir: &Path, out: &mut Vec<std::path::PathBuf>, wanted: &dyn Fn(&Path) -> bool) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        let mut entries: Vec<_> = entries.map(|entry| entry.unwrap().path()).collect();
        entries.sort();
        for path in entries {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            if path.is_dir() {
                if !matches!(name.as_str(), "target" | "node_modules" | ".git" | "dist") && !name.starts_with('.') {
                    walk(&path, out, wanted);
                }
            } else if wanted(&path) {
                out.push(path);
            }
        }
    }
    let project = project();
    let mut manifests = Vec::new();
    walk(&project, &mut manifests, &|path| path.file_name().is_some_and(|name| name == "Cargo.toml"));
    let dependents = |krate: &str| -> BTreeSet<String> {
        manifests
            .iter()
            .filter(|path| {
                let text = std::fs::read_to_string(path).unwrap();
                text.lines().any(|line| line.trim_start().starts_with(&format!("{krate} ")) || line.trim_start().starts_with(&format!("{krate}=")))
            })
            .map(|path| path.parent().unwrap().strip_prefix(&project).unwrap().to_string_lossy().replace('\\', "/"))
            .collect()
    };
    assert_eq!(
        dependents("open_pipe_stress_report_renderer"),
        BTreeSet::from(["apps/desktop/src-tauri".to_string(), "core/reporting/pdf_emitter".to_string(), "core/reporting/report_package".to_string()])
    );
    assert_eq!(dependents("open_pipe_stress_pdf_emitter"), BTreeSet::from(["core/reporting/report_package".to_string()]));
}

/// Codex round 1 at #1682 (P2, P2b): the spellings Codex found escaping on the
/// real source inventory are findings there: a raw `assemble_derived` call, a
/// renderer call through a leading `::`, a raw `DerivedUserRelease` literal in
/// its own module, a nested `cfg_attr` applying `path` (Rust would load
/// `alternate.rs`) and a derive applied by `cfg_attr`.
#[test]
fn the_real_sources_find_the_escaping_spellings() {
    let base = project_sources(&project());
    let cases = [
        ("src-tauri/src/report_package_save.rs", "fn other(request: Value) { open_pipe_stress_report_package::user_release::r#assemble_derived(request, linked, inputs, manifest); }\n", "assemble_derived is called from Fn(\"other\")"),
        ("src-tauri/src/lib.rs", "fn rooted(input: &I) { ::open_pipe_stress_report_renderer::render_calculation_report(input); }\n", "render_calculation_report is called from Fn(\"rooted\")"),
        ("report_package/src/user_release.rs", "fn forge() -> DerivedUserRelease { r#DerivedUserRelease { _derived: () } }\n", "literal outside check_derived"),
        ("report_package/src/lib.rs", "#[cfg_attr(all(), cfg_attr(all(), path = \"alternate.rs\"))]\nmod ordinary;\n", "a cfg_attr that applies #[path]"),
        ("report_package/src/lib.rs", "#[cfg_attr(all(), derive(UnlistedDerive))]\nstruct Rogue;\n", "the derive UnlistedDerive is not on the derive list"),
    ];
    for (file, text, expected) in cases {
        let mut sources = base.clone();
        sources.files.get_mut(file).unwrap().push_str(text);
        sources.files.insert("report_package/src/ordinary.rs".to_string(), "\n".to_string());
        sources.files.insert("report_package/src/alternate.rs".to_string(), "pub fn forward(input: &I) -> O { crate::pdf::assemble_full_report_package(input) }\n".to_string());
        let findings = scan(&sources);
        assert!(findings.iter().any(|finding| finding.contains(expected)), "{file}: expected {expected:?} in {findings:#?}");
    }
}

// --- The invented closed set and its negative fixtures (§6) -----------------

/// `app_invoke_handler`'s basis text, read from the real `lib.rs` (its pin
/// is checked by the real scan).
fn basis_macro() -> String {
    let text = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs")).unwrap();
    let start = text.find("macro_rules! app_invoke_handler").unwrap();
    let end = start + text[start..].find("\n}\n").unwrap() + 2;
    text[start..end].to_string()
}

/// An invented closed set of the four crates: every scanned function defined
/// once in its file and called exactly once by each allowed caller, the
/// entry arguments as the rules require, and the listed macros only.
fn invented() -> Sources {
    let file = |text: &str| text.trim_start_matches('\n').to_string();
    let mut files = BTreeMap::new();
    files.insert(
        "report_renderer/src/lib.rs".to_string(),
        file("
pub fn render_calculation_report(input: &RenderableReportInput) -> RenderOutcome {
    let assembled = assemble_report_sections(input);
    RenderOutcome::from(assembled)
}
pub fn assemble_report_sections(input: &RenderableReportInput) -> Vec<Section> {
    let _ = format!(\"{}\", input.title);
    Vec::new()
}
"),
    );
    files.insert(
        "pdf_emitter/src/lib.rs".to_string(),
        file("
use open_pipe_stress_report_renderer as renderer;
pub fn emit_calculation_report_pdf(input: &renderer::RenderableReportInput) -> PdfEmissionOutcome {
    let render = renderer::render_calculation_report(input);
    pdf_from_render(input, &render)
}
pub fn pdf_from_render(input: &renderer::RenderableReportInput, render: &renderer::RenderOutcome) -> PdfEmissionOutcome {
    let (bytes, pages, text) = emit_pdf_document(input, &render.blocking_reasons);
    PdfEmissionOutcome::new(bytes, pages, text)
}
pub fn assemble_full_report_package(input: &renderer::RenderableReportInput) -> FullReportPackageOutcome {
    let render = renderer::render_calculation_report(input);
    let pdf = pdf_from_render(input, &render);
    FullReportPackageOutcome::new(render, pdf)
}
fn emit_pdf_document(input: &renderer::RenderableReportInput, reasons: &[String]) -> (Vec<u8>, usize, Vec<String>) {
    let assembled = renderer::assemble_report_sections(input);
    (Vec::new(), assembled.len(), reasons.to_vec())
}
"),
    );
    files.insert(
        "report_package/src/lib.rs".to_string(),
        file("
use open_pipe_stress_pdf_emitter as pdf;
use open_pipe_stress_report_renderer as renderer;
pub(crate) mod admitted;
pub mod bindings;
pub mod user_release;
pub mod wire;
pub use open_pipe_stress_report_renderer::serde_error_kind;
#[cfg(test)]
mod admitted_tests;
use crate::user_release::ProducerEntry;
pub fn assemble_report_package_container(input: &ReportPackageContainerInput) -> ReportPackageContainerOutcome {
    assemble_report_package_container_in(input, &ProducerEntry::General, admitted::PRODUCTION)
}
pub(crate) fn assemble_report_package_container_in(input: &ReportPackageContainerInput, entry: &ProducerEntry, admitted: AdmittedSet) -> ReportPackageContainerOutcome {
    let report = pdf::assemble_full_report_package(input.report);
    ReportPackageContainerOutcome::new(report, entry.is_user_release())
}
"),
    );
    files.insert("report_package/src/admitted.rs".to_string(), file("pub(crate) struct AdmittedSet(&'static [&'static str]);\n"));
    files.insert("report_package/src/admitted_tests.rs".to_string(), file("fn scan() { assemble_derived(r) }\n"));
    files.insert(
        "report_package/src/bindings.rs".to_string(),
        file("
use crate::user_release::ProducerEntry;
pub fn check_request(request: &ReportPackageRequest) -> Result<(), String> {
    check_request_in(request, &ProducerEntry::General, admitted::PRODUCTION)
}
pub(crate) fn check_request_in(request: &ReportPackageRequest, entry: &ProducerEntry, admitted: AdmittedSet) -> Result<(), String> {
    if !matches!(entry, ProducerEntry::UserRelease(_)) && request.marked() {
        return Err(String::new());
    }
    Ok(())
}
"),
    );
    files.insert(
        "report_package/src/wire.rs".to_string(),
        file("
use crate::user_release::ProducerEntry;
pub fn assemble_wire_request(request: ReportPackageRequest) -> Result<ReportPackageContainerOutcome, String> {
    assemble_wire_request_in(request, &ProducerEntry::General, admitted::PRODUCTION)
}
pub(crate) fn assemble_wire_request_in(request: ReportPackageRequest, entry: &ProducerEntry, admitted: AdmittedSet) -> Result<ReportPackageContainerOutcome, String> {
    crate::bindings::check_request_in(&request, entry, admitted)?;
    Ok(crate::assemble_report_package_container_in(&request.input(), entry, admitted))
}
"),
    );
    files.insert(
        "report_package/src/user_release.rs".to_string(),
        file("
use crate::admitted;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
pub(crate) struct DerivedUserRelease {
    _derived: (),
}
pub(crate) enum ProducerEntry {
    General,
    UserRelease(DerivedUserRelease),
}
impl ProducerEntry {
    pub(crate) fn is_user_release(&self) -> bool {
        matches!(self, ProducerEntry::UserRelease(_))
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = \"snake_case\")]
pub struct Basis {
    #[serde(default, skip_serializing_if = \"Option::is_none\")]
    pub note: Option<String>,
}
fn check_derived(request: &Value) -> Result<DerivedUserRelease, String> {
    let _ = json!({\"derived\": true});
    Ok(DerivedUserRelease { _derived: () })
}
pub fn assemble_derived(request: Value) -> Result<ReportPackageContainerOutcome, String> {
    let derived = check_derived(&request)?;
    crate::wire::assemble_wire_request_in(request.into(), &ProducerEntry::UserRelease(derived), admitted::PRODUCTION)
}
#[cfg(test)]
mod tests {
    fn literal() -> super::DerivedUserRelease { super::DerivedUserRelease { _derived: () } }
}
"),
    );
    files.insert(
        "src-tauri/src/lib.rs".to_string(),
        file(&format!(
            "
mod host_memory;
#[cfg(feature = \"native-smoke\")]
mod native_smoke;
mod report_package_bridge;
mod report_package_save;
#[cfg(test)]
mod report_render_exact_tests;
use serde_json::{{json, Value}};
#[tauri::command]
fn render_calculation_report(input: Value) -> Result<Value, String> {{
    render_calculation_report_value(input)
}}
fn render_calculation_report_value(input: Value) -> Result<Value, String> {{
    let renderable = decode(input)?;
    let outcome = open_pipe_stress_report_renderer::render_calculation_report(&renderable);
    Ok(json!({{\"html\": outcome.html}}))
}}

{}
pub fn run() {{
    let builder = tauri::Builder::default();
    let builder = builder.invoke_handler(app_invoke_handler!());
    let builder = builder.invoke_handler(app_invoke_handler!(native_smoke::take_chosen_path));
    builder.run(tauri::generate_context!());
}}
",
            basis_macro()
        )),
    );
    files.insert("src-tauri/src/main.rs".to_string(), file("fn main() {\n    println!(\"{}\", 1);\n}\n"));
    files.insert("src-tauri/src/report_render_exact_tests.rs".to_string(), file("fn t() { open_pipe_stress_report_renderer::render_calculation_report(&i); }\n"));
    files.insert("src-tauri/src/native_smoke.rs".to_string(), file("pub fn take_chosen_path() {}\n"));
    files.insert("src-tauri/src/host_memory.rs".to_string(), file("#[cfg(target_os = \"macos\")]\nmod mac {\n    pub fn sample() {}\n}\n"));
    files.insert(
        "src-tauri/src/report_package_bridge.rs".to_string(),
        file("
use open_pipe_stress_report_package as package;
pub fn assemble_user_released_request(request: Value) -> Result<Outcome, String> {
    package::user_release::assemble_derived(request)
}
"),
    );
    files.insert(
        "src-tauri/src/report_package_save.rs".to_string(),
        file("
fn admit_user_released(request: Value) -> Result<Outcome, String> {
    crate::report_package_bridge::assemble_user_released_request(request)
}
"),
    );
    let manifests = CRATES.iter().map(|(key, _)| (key.to_string(), format!("[package]\nname = \"{key}\"\n\n[lib]\npath = \"src/lib.rs\"\n"))).collect();
    Sources { files, manifests, bin_dirs: BTreeSet::new() }
}

/// `invented()` with one exact edit (`old` must occur once in `file`).
fn edited(file: &str, old: &str, new: &str) -> Sources {
    let mut sources = invented();
    let text = sources.files.get_mut(file).unwrap_or_else(|| panic!("no {file}"));
    assert_eq!(text.matches(old).count(), 1, "{file}: {old}");
    *text = text.replacen(old, new, 1);
    sources
}

/// `invented()` with `text` appended to `file`.
fn appended(file: &str, text: &str) -> Sources {
    let mut sources = invented();
    sources.files.get_mut(file).unwrap().push_str(text);
    sources
}

const WRITER_CALL: &str = "    let report = pdf::assemble_full_report_package(input.report);\n";
const LIB: &str = "report_package/src/lib.rs";
const TAURI: &str = "src-tauri/src/lib.rs";
const SAVE: &str = "src-tauri/src/report_package_save.rs";
const BRIDGE: &str = "src-tauri/src/report_package_bridge.rs";
const USER_RELEASE: &str = "report_package/src/user_release.rs";

#[test]
fn the_invented_closed_set_passes() {
    let findings = scan(&invented());
    assert!(findings.is_empty(), "{findings:#?}");
}

/// Each change made alone to the invented closed set is a finding (§6, Codex
/// rounds 2-4 at revision 4), unless it is marked as passing.
#[test]
fn user_release_reach_scan_finds_each_break() {
    let with_file = |path: &str, text: &str, mut sources: Sources| {
        sources.files.insert(path.to_string(), text.to_string());
        sources
    };
    let cases: Vec<(&str, Sources, &str)> = vec![
        ("an added call site of assemble_derived", appended("src-tauri/src/report_package_save.rs", "fn other(r: Value) { package::user_release::assemble_derived(r); }\n"), "assemble_derived"),
        ("a forwarding pub fn", appended(LIB, "pub fn forward(input: &renderer::RenderableReportInput) -> pdf::FullReportPackageOutcome { pdf::assemble_full_report_package(input) }\n"), "assemble_full_report_package is called from Fn(\"forward\")"),
        ("emit_calculation_report_pdf from report_package", appended(LIB, "pub fn a(i: &I) { pdf::emit_calculation_report_pdf(i); }\n"), "emit_calculation_report_pdf is called"),
        ("the renderer from report_package", appended(LIB, "pub fn a(i: &I) { renderer::render_calculation_report(i); }\n"), "render_calculation_report is called"),
        ("a fabricated render", appended(LIB, "pub fn a(i: &I) { pdf::pdf_from_render(i, &RenderOutcome::fabricated()); }\n"), "pdf_from_render is called"),
        ("the renderer from pdf_emitter", appended("pdf_emitter/src/lib.rs", "pub fn a(i: &I) { renderer::render_calculation_report(i); }\n"), "render_calculation_report is called from Fn(\"a\")"),
        ("pdf_from_render from another pdf_emitter fn", appended("pdf_emitter/src/lib.rs", "pub fn a(i: &I, r: &R) { pdf_from_render(i, r); }\n"), "pdf_from_render is called from Fn(\"a\")"),
        ("assemble_report_sections from pdf_emitter", appended("pdf_emitter/src/lib.rs", "pub fn a(i: &I) { renderer::assemble_report_sections(i); }\n"), "assemble_report_sections is called from Fn(\"a\")"),
        ("a second writer call", edited(LIB, WRITER_CALL, &format!("{WRITER_CALL}{WRITER_CALL}")), "calls assemble_full_report_package 2 times"),
        ("use ... as build", appended(LIB, "use open_pipe_stress_pdf_emitter::assemble_full_report_package as build;\npub fn b(i: &I) { build(i); }\n"), "named in a use"),
        ("pub use of the function", appended(LIB, "pub use open_pipe_stress_pdf_emitter::assemble_full_report_package;\n"), "re-exports the renderer or PDF crate"),
        ("pub use pdf::*", appended(LIB, "pub use pdf::*;\n"), "a glob import of pdf"),
        ("pub use the crate as pdf", appended(LIB, "pub use open_pipe_stress_pdf_emitter as pdf;\n"), "re-exports the renderer or PDF crate"),
        ("a fn-pointer constant", appended(LIB, "const BUILD: fn(&renderer::RenderableReportInput) -> pdf::FullReportPackageOutcome = pdf::assemble_full_report_package;\n"), "named other than as a definition or a direct call"),
        ("map with the function value", edited(LIB, WRITER_CALL, "    let report = std::iter::once(input.report).map(pdf::assemble_full_report_package).next().unwrap();\n"), "named other than as a definition or a direct call"),
        ("a closure in the writer", edited(LIB, WRITER_CALL, "    let build = |input| pdf::assemble_full_report_package(input);\n    let report = build(input.report);\n"), "is called from Closure"),
        ("a de-indented multi-line closure", edited(LIB, WRITER_CALL, "    let build = |input| {\npdf::assemble_full_report_package(input)\n    };\n    let report = build(input.report);\n"), "is called from Closure"),
        ("a macro_rules in report_package", appended(LIB, "macro_rules! build { ($i:expr) => { pdf::assemble_full_report_package($i) }; }\n"), "the macro definition build is not on the allow-list"),
        ("the writer through that macro", edited(LIB, WRITER_CALL, "    let report = build!(input.report);\n"), "calls assemble_full_report_package 0 times"),
        ("package::assemble_full_report_package in src-tauri", appended(TAURI, "fn glob(i: &I) { package::assemble_full_report_package(i); }\n"), "assemble_full_report_package is called from Fn(\"glob\")"),
        ("a renderer import in src-tauri", appended(TAURI, "use open_pipe_stress_report_renderer::render_calculation_report;\n"), "src-tauri imports"),
        ("a second renderer call in src-tauri", appended(TAURI, "fn again(i: &I) { open_pipe_stress_report_renderer::render_calculation_report(i); }\n"), "render_calculation_report is called from Fn(\"again\")"),
        ("a DerivedUserRelease literal outside check_derived", appended("report_package/src/user_release.rs", "fn forge() -> DerivedUserRelease { DerivedUserRelease { _derived: () } }\n"), "literal outside check_derived"),
        ("check_derived outside assemble_derived", appended("report_package/src/user_release.rs", "fn other(r: &Value) { check_derived(r); }\n"), "check_derived is called from Fn(\"other\")"),
        ("UserRelease passed from another function", appended("report_package/src/user_release.rs", "fn smuggle(r: R, derived: DerivedUserRelease) { crate::wire::assemble_wire_request_in(r, &ProducerEntry::UserRelease(derived), admitted::PRODUCTION); }\n"), "assemble_wire_request_in is called from Fn(\"smuggle\")"),
        ("UserRelease at a public root", edited("report_package/src/bindings.rs", "check_request_in(request, &ProducerEntry::General, admitted::PRODUCTION)", "check_request_in(request, &ProducerEntry::UserRelease(derived), admitted::PRODUCTION)"), "check_request passes &ProducerEntry::UserRelease(derived)"),
        ("a production #[path] module", appended(LIB, "#[cfg(test)]\nmod escape;\n#[cfg(not(test))]\n#[path = \"escape.rs\"]\npub mod live_escape;\n"), "a #[path] on the production module live_escape"),
        ("the test and production pair without #[path]", with_file("report_package/src/escape.rs", "pub fn forward(input: &crate::renderer::RenderableReportInput) -> crate::pdf::FullReportPackageOutcome { crate::pdf::assemble_full_report_package(input) }\n", appended(LIB, "#[cfg(test)]\nmod escape;\n#[cfg(not(test))]\npub mod escape;\n")), "report_package/src/escape.rs:1: assemble_full_report_package is called from Fn(\"forward\")"),
        ("a second call in native_smoke.rs", edited("src-tauri/src/native_smoke.rs", "pub fn take_chosen_path() {}", "pub fn take_chosen_path() {}\nfn smoke(i: &I) { open_pipe_stress_report_renderer::render_calculation_report(i); }"), "src-tauri/src/native_smoke.rs:2: render_calculation_report is called"),
        ("a call in an inline macos module", edited("src-tauri/src/host_memory.rs", "    pub fn sample() {}", "    pub fn sample(i: &I) { open_pipe_stress_report_renderer::render_calculation_report(i); }"), "host_memory.rs:3: render_calculation_report is called"),
        ("include! of Rust", appended(LIB, "include!(\"forward.rs\");\n"), "include! brings in source"),
        ("include_str! of a .rs file", appended(LIB, "const S: &str = include_str!(\"forward.rs\");\n"), "takes one string literal that names no .rs file"),
        ("include_str! of concat!", appended(LIB, "const S: &str = include_str!(concat!(\"for\", \"ward.json\"));\n"), "takes one string literal"),
        ("a trait object", appended(LIB, "pub fn builder() -> Box<dyn Fn(&renderer::RenderableReportInput) -> pdf::FullReportPackageOutcome> { Box::new(pdf::assemble_full_report_package) }\n"), "named other than as a definition or a direct call"),
        ("a generic Fn argument", appended(LIB, "fn apply<F: Fn(&I) -> O>(f: F, input: &I) -> O { f(input) }\npub fn via(input: &I) -> O { apply(pdf::assemble_full_report_package, input) }\n"), "named other than as a definition or a direct call"),
        ("an impl Fn return", appended(LIB, "pub fn builder() -> impl Fn(&I) -> O { pdf::assemble_full_report_package }\n"), "named other than as a definition or a direct call"),
        ("an impl Fn closure return", appended(LIB, "pub fn builder() -> impl Fn(&I) -> O { |input| pdf::assemble_full_report_package(input) }\n"), "is called from Closure"),
        ("a method on a wrapper", appended(LIB, "pub struct Builder;\nimpl Builder { pub fn build(&self, input: &I) -> O { pdf::assemble_full_report_package(input) } }\n"), "is called from Method(\"build\")"),
        ("a trait impl method", appended(LIB, "impl From<&I> for O { fn from(input: &I) -> O { pdf::assemble_full_report_package(input) } }\n"), "is called from Method(\"from\")"),
        ("an associated const", appended(LIB, "pub struct Builder;\nimpl Builder { const BUILD: fn(&I) -> O = pdf::assemble_full_report_package; }\n"), "named other than as a definition or a direct call"),
        ("a nested fn in the writer", edited(LIB, WRITER_CALL, "    fn inner(input: &I) -> O { pdf::assemble_full_report_package(input) }\n    let report = inner(input.report);\n"), "is called from Fn(\"inner\")"),
        ("an async block in the writer", edited(LIB, WRITER_CALL, "    let report = async { pdf::assemble_full_report_package(input.report) };\n"), "is called from Async"),
        ("a method call by the scanned name", appended(LIB, "pub fn m(input: &I) { input.assemble_full_report_package(); }\n"), "named other than as a definition or a direct call"),
        ("a macro that loads a module", with_file("report_package/src/escape.rs", "pub fn forward(input: &I) -> O { crate::pdf::assemble_full_report_package(input) }\n", appended(LIB, "macro_rules! load_escape { () => { pub mod escape; }; }\nload_escape!();\n")), "the macro definition load_escape is not on the allow-list"),
        ("its invocation", with_file("report_package/src/escape.rs", "pub fn forward() {}\n", appended(LIB, "macro_rules! load_escape { () => { pub mod escape; }; }\nload_escape!();\n")), "the macro load_escape! is not on the invocation list"),
        ("a parameterized path", appended(TAURI, "macro_rules! build { ($r:ident, $i:expr) => { $r::render_calculation_report($i) }; }\nfn b(i: &I) { build!(open_pipe_stress_report_renderer, i); }\n"), "the macro build! is not on the invocation list"),
        ("an unlisted external macro", appended(LIB, "fn e(i: &I) { forwarder::emit!(i); }\n"), "the macro forwarder::emit! is not on the invocation list"),
        ("use forwarder::emit", appended(LIB, "use forwarder::emit;\nfn e(i: &I) { emit!(i); }\n"), "the macro emit! is not on the invocation list"),
        ("use forwarder::emit as json", appended(LIB, "use forwarder::emit as json;\nfn e(i: &I) { json!(i); }\n"), "binds the listed name json"),
        ("use forwarder::emit as format", appended(LIB, "use forwarder::emit as format;\n"), "binds the listed name format"),
        ("#[macro_use] extern crate", appended(LIB, "#[macro_use]\nextern crate forwarder;\n"), "#[macro_use]"),
        ("a new local macro without a scanned name", appended("pdf_emitter/src/lib.rs", "macro_rules! twice { ($e:expr) => { ($e, $e) }; }\n"), "the macro definition twice is not on the allow-list"),
        ("its invocation", appended("pdf_emitter/src/lib.rs", "macro_rules! twice { ($e:expr) => { ($e, $e) }; }\nfn t() { twice!(1); }\n"), "the macro twice! is not on the invocation list"),
        ("a macro 2.0 item", appended("pdf_emitter/src/lib.rs", "pub macro twice($e:expr) { ($e, $e) }\n"), "Verbatim"),
        ("app_invoke_handler with a handler added", edited(TAURI, "            save_local_result_json $(, $extra)*", "            save_local_result_json, forward $(, $extra)*"), "differs from its pinned text"),
        ("app_invoke_handler with whitespace changed", edited(TAURI, "            save_report_package,\n", "            save_report_package,  \n"), "differs from its pinned text"),
        ("app_invoke_handler with a comment", edited(TAURI, "            save_report_package,\n", "            save_report_package, // invented\n"), "differs from its pinned text"),
        ("app_invoke_handler moved to another file", {
            let mut sources = invented();
            let basis = basis_macro();
            let text = sources.files.get_mut(TAURI).unwrap();
            *text = text.replacen(&basis, "", 1);
            sources.files.get_mut("src-tauri/src/report_package_save.rs").unwrap().push_str(&basis);
            sources
        }, "the macro definition app_invoke_handler is not on the allow-list"),
        ("app_invoke_handler defined twice", appended(TAURI, &format!("\n{}\n", basis_macro())), "is defined 2 times"),
        ("an attribute macro", appended(LIB, "#[forwarder::instrument]\nfn f() {}\n"), "the attribute #[forwarder::instrument] is not on the attribute list"),
        ("tauri::command on the bridge function", edited("src-tauri/src/report_package_bridge.rs", "pub fn assemble_user_released_request", "#[tauri::command]\npub fn assemble_user_released_request"), "#[tauri::command] marks the scanned function assemble_user_released_request"),
        ("serialize_with", edited("report_package/src/user_release.rs", "    pub note: Option<String>,", "    #[serde(serialize_with = \"forward\")]\n    pub note: Option<String>,"), "the serde key serialize_with is not on the serde key list"),
        ("skip_serializing_if = forward", edited("report_package/src/user_release.rs", "skip_serializing_if = \"Option::is_none\"", "skip_serializing_if = \"forward\""), "a skip_serializing_if naming a function outside the two admitted"),
        ("an unlisted derive path", edited("report_package/src/user_release.rs", "#[derive(Clone, Debug, Serialize, Deserialize)]", "#[derive(Clone, forwarder::Forward)]"), "the derive forwarder::Forward is not on the derive list"),
        ("derive Hash", edited("report_package/src/user_release.rs", "#[derive(Clone, Debug, Serialize, Deserialize)]", "#[derive(Clone, Hash)]"), "the derive Hash is not on the derive list"),
        ("cfg_attr applying an attribute macro", appended(LIB, "#[cfg_attr(not(test), forwarder::instrument)]\nfn f() {}\n"), "through cfg_attr"),
        ("cfg_attr applying path", appended(LIB, "#[cfg_attr(not(test), path = \"escape.rs\")]\nmod escape;\n"), "a cfg_attr that applies #[path]"),
        ("a missing module file", appended(LIB, "mod missing;\n"), "the module missing has no file"),
        ("a module that matches both candidates", with_file("report_package/src/both/mod.rs", "\n", with_file("report_package/src/both.rs", "\n", appended(LIB, "mod both;\n"))), "matches both"),
        ("a production file that does not parse", appended(LIB, "fn broken( {\n"), "does not parse"),
        ("a [[bin]] target", {
            let mut sources = invented();
            sources.manifests.get_mut("report_package").unwrap().push_str("\n[[bin]]\nname = \"x\"\n");
            sources
        }, "declares a [[bin]] target"),
        ("a src/bin directory", {
            let mut sources = invented();
            sources.bin_dirs.insert("pdf_emitter".to_string());
            sources
        }, "has a src/bin directory"),
        ("a glob import", appended("src-tauri/src/report_package_bridge.rs", "use open_pipe_stress_report_package::*;\n"), "a glob import of open_pipe_stress_report_package"),
    ];
    // Codex round 1 at #1682 (P2): names are compared without `r#` and paths
    // without a leading `::`, `crate::`, `self::` or `super::` as the scan
    // resolves modules; every other spelling fails closed.
    let spellings: Vec<(&str, Sources, &str)> = vec![
        ("a raw assemble_derived call", appended(SAVE, "fn other(r: Value) { open_pipe_stress_report_package::user_release::r#assemble_derived(r, l, i, m); }\n"), "assemble_derived is called from Fn(\"other\")"),
        ("a raw assemble_derived on the real-source form", edited(BRIDGE, "    package::user_release::assemble_derived(request)\n", "    package::user_release::assemble_derived(request)\n}\nfn twice(request: Value) -> Result<Outcome, String> {\n    open_pipe_stress_report_package::user_release::r#assemble_derived(request, linked, inputs, manifest)\n"), "assemble_derived is called from Fn(\"twice\")"),
        ("the renderer through a leading ::", appended(TAURI, "fn rooted(input: &I) { ::open_pipe_stress_report_renderer::render_calculation_report(input); }\n"), "render_calculation_report is called from Fn(\"rooted\")"),
        ("the renderer by a raw name", appended(TAURI, "fn raw(i: &I) { open_pipe_stress_report_renderer::r#render_calculation_report(i); }\n"), "render_calculation_report is called from Fn(\"raw\")"),
        ("the renderer crate by a raw name", appended(TAURI, "fn raw(i: &I) { r#open_pipe_stress_report_renderer::render_calculation_report(i); }\n"), "render_calculation_report is called from Fn(\"raw\")"),
        ("the renderer pair through Self::", appended(TAURI, "struct W;\nimpl W { fn r(i: &I) { Self::render_calculation_report(i); } }\n"), "render_calculation_report is called from Method(\"r\")"),
        ("the renderer pair through a re-export path", appended(TAURI, "fn re(i: &I) { open_pipe_stress_report_package::forwarded::assemble_report_sections(i); }\n"), "assemble_report_sections is called from Fn(\"re\")"),
        ("a fn pointer to the renderer through ::", appended(TAURI, "const R: fn(&I) -> O = ::open_pipe_stress_report_renderer::render_calculation_report;\n"), "render_calculation_report named in a path other than a direct call"),
        ("a closure bound to the renderer through ::", appended(TAURI, "fn c(i: &I) { let f = |i| ::open_pipe_stress_report_renderer::render_calculation_report(i); f(i); }\n"), "render_calculation_report is called from Closure"),
        ("the renderer path in macro tokens, raw", appended(TAURI, "fn m() { let _ = format!(\"{:p}\", ::open_pipe_stress_report_renderer::r#render_calculation_report as fn(&I) -> O); }\n"), "render_calculation_report named in a macro's or an attribute's tokens"),
        ("a raw scanned name in macro tokens", appended(LIB, "fn m() { let _ = format!(\"{:p}\", pdf::r#assemble_full_report_package as fn(&I) -> O); }\n"), "assemble_full_report_package named in a macro's or an attribute's tokens"),
        ("a raw method call by the scanned name", appended(LIB, "pub fn m(input: &I) { input.r#assemble_full_report_package(); }\n"), "named other than as a definition or a direct call"),
        ("a raw scanned fn pointer", appended(LIB, "const BUILD: fn(&I) -> O = pdf::r#assemble_full_report_package;\n"), "named other than as a definition or a direct call"),
        ("a <T as Trait>:: call by the scanned name", appended(SAVE, "fn qualified(r: Value) { <Bridge as Assemble>::assemble_user_released_request(r); }\n"), "named through a qualified <T as Trait>:: path"),
        ("a <T>:: call by the scanned name", appended(LIB, "pub fn q(i: &I) { <pdf::Emitter>::assemble_full_report_package(i); }\n"), "named through a qualified <T as Trait>:: path"),
        ("use ::renderer as an alias in src-tauri", appended(TAURI, "use ::open_pipe_stress_report_renderer as r;\n"), "src-tauri imports open_pipe_stress_report_renderer"),
        ("use of a raw renderer crate name in src-tauri", appended(TAURI, "use r#open_pipe_stress_report_renderer as r;\n"), "src-tauri imports open_pipe_stress_report_renderer"),
        ("use of a raw scanned name as an alias", appended(LIB, "use open_pipe_stress_pdf_emitter::r#assemble_full_report_package as build;\n"), "assemble_full_report_package named in a use"),
        ("the PDF crate bound under another alias", appended(LIB, "use open_pipe_stress_pdf_emitter as forge;\n"), "binds the renderer or PDF crate as forge"),
        ("a re-export of the PDF alias through crate::", appended(LIB, "pub use crate::pdf as p;\n"), "re-exports the renderer or PDF crate"),
        ("a re-export of the renderer alias through self::", appended(LIB, "pub(crate) use self::renderer as r;\n"), "re-exports the renderer or PDF crate"),
        ("a raw DerivedUserRelease literal in its module", appended(USER_RELEASE, "fn forge() -> DerivedUserRelease { r#DerivedUserRelease { _derived: () } }\n"), "literal outside check_derived"),
        ("a Self literal in an impl of DerivedUserRelease", appended(USER_RELEASE, "impl DerivedUserRelease { fn forge() -> Self { Self { _derived: () } } }\n"), "literal outside check_derived"),
        ("a type alias of DerivedUserRelease", appended(USER_RELEASE, "type D = DerivedUserRelease;\nfn forge() -> D { D { _derived: () } }\n"), "DerivedUserRelease named in a type alias"),
        ("an associated type of DerivedUserRelease", appended(USER_RELEASE, "impl Tr for X { type D = DerivedUserRelease; }\n"), "DerivedUserRelease named in a type alias"),
        ("a derive on DerivedUserRelease", edited(USER_RELEASE, "pub(crate) struct DerivedUserRelease {", "#[derive(Default)]\npub(crate) struct DerivedUserRelease {"), "the DerivedUserRelease definition carries #[derive]"),
        ("a cfg_attr derive on DerivedUserRelease", edited(USER_RELEASE, "pub(crate) struct DerivedUserRelease {", "#[cfg_attr(all(), derive(Clone))]\npub(crate) struct DerivedUserRelease {"), "the DerivedUserRelease definition carries #[derive]"),
        ("an unsafe block in a reporting crate", appended(USER_RELEASE, "fn forge() -> DerivedUserRelease { unsafe { std::mem::zeroed() } }\n"), "an unsafe block in a reporting crate"),
        ("an unsafe fn in a reporting crate", appended("pdf_emitter/src/lib.rs", "unsafe fn f() {}\n"), "an unsafe fn in a reporting crate"),
        ("an extern block in a reporting crate", appended(LIB, "extern \"C\" { fn forge() -> u8; }\n"), "an extern block in a reporting crate"),
        ("UserRelease bound by an import", appended(USER_RELEASE, "use crate::user_release::ProducerEntry::UserRelease as U;\n"), "binds ProducerEntry::UserRelease"),
        ("UserRelease as a constructor value", appended(USER_RELEASE, "fn smuggle(derived: DerivedUserRelease) -> ProducerEntry { let f = ProducerEntry::UserRelease; f(derived) }\n"), "ProducerEntry::UserRelease is constructed outside assemble_derived"),
        ("a raw UserRelease construction", appended(USER_RELEASE, "fn smuggle(derived: DerivedUserRelease) -> ProducerEntry { ProducerEntry::r#UserRelease(derived) }\n"), "ProducerEntry::UserRelease is constructed outside assemble_derived"),
        ("a raw module name resolves to its file", with_file("report_package/src/escape.rs", "pub fn forward(input: &I) -> O { crate::pdf::assemble_full_report_package(input) }\n", appended(LIB, "mod r#escape;\n")), "report_package/src/escape.rs:1: assemble_full_report_package is called from Fn(\"forward\")"),
        ("an unlisted macro behind a leading ::", appended(LIB, "fn e(i: &I) { ::forwarder::emit!(i); }\n"), "the macro forwarder::emit! is not on the invocation list"),
    ];
    // Codex round 1 at #1682 (P2): a conditional attribute is checked by the
    // rules of a direct one, at any depth, whatever its predicate.
    let conditional: Vec<(&str, Sources, &str)> = vec![
        (
            "a nested cfg_attr applying path",
            with_file("report_package/src/alternate.rs", "pub fn forward(input: &I) -> O { crate::pdf::assemble_full_report_package(input) }\n", with_file("report_package/src/ordinary.rs", "\n", appended(LIB, "#[cfg_attr(all(), cfg_attr(all(), path = \"alternate.rs\"))]\nmod ordinary;\n"))),
            "a cfg_attr that applies #[path]",
        ),
        ("a cfg_attr applying path under a false predicate", with_file("report_package/src/ordinary.rs", "\n", appended(LIB, "#[cfg_attr(any(), path = \"alternate.rs\")]\nmod ordinary;\n")), "a cfg_attr that applies #[path]"),
        ("an unlisted derive through cfg_attr", appended(LIB, "#[cfg_attr(all(), derive(UnlistedDerive))]\nstruct Rogue;\n"), "the derive UnlistedDerive is not on the derive list"),
        ("an unlisted derive through a nested cfg_attr", appended(LIB, "#[cfg_attr(all(), cfg_attr(not(test), derive(UnlistedDerive)))]\nstruct Rogue;\n"), "the derive UnlistedDerive is not on the derive list"),
        ("several attributes in one cfg_attr", appended(LIB, "#[cfg_attr(all(), derive(Clone), derive(Hash))]\nstruct Rogue;\n"), "the derive Hash is not on the derive list"),
        ("a serde key through cfg_attr", edited(USER_RELEASE, "    pub note: Option<String>,", "    #[cfg_attr(all(), serde(serialize_with = \"forward\"))]\n    pub note: Option<String>,"), "the serde key serialize_with is not on the serde key list"),
        ("macro_use through cfg_attr", appended(LIB, "#[cfg_attr(all(), macro_use)]\nmod inline_macros {}\n"), "#[macro_use]"),
        ("tauri::command through cfg_attr on a scanned function", edited(BRIDGE, "pub fn assemble_user_released_request", "#[cfg_attr(all(), tauri::command)]\npub fn assemble_user_released_request"), "#[tauri::command] marks the scanned function assemble_user_released_request"),
        ("an attribute macro through a nested cfg_attr", appended(LIB, "#[cfg_attr(all(), cfg_attr(all(), forwarder::instrument))]\nfn f() {}\n"), "the attribute #[forwarder::instrument] (through cfg_attr) is not on the attribute list"),
        ("a macro in a cfg_attr's doc value", appended(LIB, "#[cfg_attr(all(), doc = include!(\"forward.rs\"))]\nfn f() {}\n"), "include! brings in source"),
    ];
    let cases: Vec<(&str, Sources, &str)> = cases.into_iter().chain(spellings).chain(conditional).collect();
    for (case, sources, expected) in cases {
        let findings = scan(&sources);
        assert!(findings.iter().any(|finding| finding.contains(expected)), "{case}: expected {expected:?} in {findings:#?}");
    }
}

/// Each of these must pass (§6, "Passing cases").
#[test]
fn user_release_reach_scan_admits_the_listed_forms() {
    let passing: Vec<(&str, Sources)> = vec![
        (
            "each listed std macro, with no scanned name",
            appended(LIB, "fn macros(v: &mut String) { let _ = format!(\"x\"); let _ = vec![1]; let _ = format_args!(\"x\"); let _ = matches!(1, 1); let _ = write!(v, \"x\"); eprintln!(\"x\"); assert!(true); debug_assert!(true); println!(\"x\"); debug_assert_ne!(1, 2); let _ = env!(\"CARGO_PKG_NAME\"); if false { unreachable!(\"x\") } if false { panic!(\"x\") } }\n"),
        ),
        ("json! and params! bare and qualified", appended(TAURI, "use rusqlite::params;\nfn values() { let _ = json!({}); let _ = params![1]; let _ = serde_json::json!({}); let _ = rusqlite::params![1]; }\n")),
        ("forward_to_deserialize_any! in a Deserializer impl", appended(TAURI, "impl<'de> serde::Deserializer<'de> for D { serde::forward_to_deserialize_any! { bool } }\n")),
        ("tauri::command on an ordinary command", appended(TAURI, "#[tauri::command]\nfn ordinary() {}\n")),
        ("each listed derive and the basis serde keys", appended("report_package/src/user_release.rs", "#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]\n#[serde(tag = \"kind\", untagged)]\npub enum E {\n    #[default]\n    A,\n    #[serde(rename = \"b\", skip)]\n    B,\n}\n#[derive(Serialize)]\npub struct F {\n    #[serde(flatten)]\n    pub g: G,\n    #[serde(skip_serializing_if = \"std::ops::Not::not\")]\n    pub h: bool,\n}\n#[cfg_attr(not(test), allow(dead_code))]\nfn unused() {}\n")),
        ("a #[cfg(test)] thread_local! item and a test module's own macro", appended(LIB, "#[cfg(test)]\nthread_local! { static X: u8 = 0; }\n#[cfg(test)]\nmod probes {\n    macro_rules! probe { () => {}; }\n    fn t() { probe!(); pdf::assemble_full_report_package(i); }\n}\n")),
        ("an include_str! of a .json file", appended(LIB, "const S: &str = include_str!(\"../schema.json\");\n")),
        ("a #[cfg(test)] item holding a call", appended(LIB, "#[cfg(test)]\nfn t(i: &I) { pdf::assemble_full_report_package(i); renderer::render_calculation_report(i); }\n")),
        ("a ProducerEntry::UserRelease(_) pattern in check_request_in", invented()),
        ("a test-only outlined module", appended(LIB, "#[cfg(test)]\nmod witness_tests;\n")),
        // Codex round 1 at #1682 (P2): the normalized spellings of admitted forms.
        ("listed macros and an attribute behind a leading ::", appended(TAURI, "#[::tauri::command]\nfn rooted() { let _ = ::serde_json::json!({}); }\n")),
        ("the local Tauri command through crate::", appended(TAURI, "fn again(input: Value) -> Result<Value, String> { crate::render_calculation_report(input) }\n")),
        ("a nested cfg_attr applying listed attributes", appended("report_package/src/user_release.rs", "#[cfg_attr(all(), cfg_attr(not(test), allow(dead_code)), derive(Clone, Debug))]\nstruct Listed;\n")),
    ];
    for (case, sources) in passing {
        let findings = scan(&sources);
        assert!(findings.is_empty(), "{case}: {findings:#?}");
    }
}
