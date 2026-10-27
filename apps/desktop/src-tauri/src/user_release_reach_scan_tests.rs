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

/// Codex rounds 1 and 2 at #1682 (P2s): the forms Codex found escaping on the
/// real source inventory are findings there: a raw `assemble_derived` call, a
/// renderer call through a leading `::`, a raw `DerivedUserRelease` literal in
/// its own module, a nested `cfg_attr` applying `path` (Rust would load
/// `alternate.rs`), a derive applied by `cfg_attr`, a `DerivedUserRelease`
/// literal through an associated type assigned `Self`, and the braced and
/// `matches!`-guard constructions of `ProducerEntry::UserRelease`.
#[test]
fn the_real_sources_find_the_escaping_spellings() {
    let base = project_sources(&project());
    let cases = [
        ("src-tauri/src/report_package_save.rs", "fn other(request: Value) { open_pipe_stress_report_package::user_release::r#assemble_derived(request, linked, inputs, manifest); }\n", "assemble_derived is called from Fn(\"other\")"),
        ("src-tauri/src/lib.rs", "fn rooted(input: &I) { ::open_pipe_stress_report_renderer::render_calculation_report(input); }\n", "render_calculation_report is called from Fn(\"rooted\")"),
        ("report_package/src/user_release.rs", "fn forge() -> DerivedUserRelease { r#DerivedUserRelease { _derived: () } }\n", "literal outside check_derived"),
        ("report_package/src/lib.rs", "#[cfg_attr(all(), cfg_attr(all(), path = \"alternate.rs\"))]\nmod ordinary;\n", "a cfg_attr that applies #[path]"),
        ("report_package/src/lib.rs", "#[cfg_attr(all(), derive(UnlistedDerive))]\nstruct Rogue;\n", "the derive UnlistedDerive is not on the derive list"),
        // Codex round 2 at #1682 (P2): an associated-type alias of
        // `DerivedUserRelease`, and the braced and `matches!`-guard
        // constructions of `ProducerEntry::UserRelease`.
        ("report_package/src/user_release.rs", CODEX_R2_ALIAS, "an impl of the trait Identity for DerivedUserRelease, which is not on the allow-list"),
        ("report_package/src/user_release.rs", CODEX_R2_ALIAS, "a struct literal with generic arguments (Alias) in the defining module"),
        ("report_package/src/user_release.rs", "fn braced(derived: DerivedUserRelease) -> ProducerEntry { ProducerEntry::UserRelease { 0: derived } }\n", "ProducerEntry::UserRelease is constructed outside assemble_derived (Fn(\"braced\"))"),
        ("report_package/src/user_release.rs", "fn guarded(derived: DerivedUserRelease) -> bool { matches!((), _ if { let _entry = ProducerEntry::UserRelease(derived); true }) }\n", "ProducerEntry::UserRelease is constructed outside assemble_derived (Macro)"),
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

/// Code review r3 at #1682 (P2s), verbatim on the real sources: each of the
/// three forms compiled in `report_package/src/user_release.rs` and gave no
/// finding. Each now gives findings for its own construct, not only for the
/// shared prelude: an alias declared in a macro's tokens, a raw identifier
/// taken for a block keyword, and a construction in a const generic argument
/// of a `matches!` pattern.
#[test]
fn the_real_sources_find_the_review_r3_forms() {
    const PRELUDE: &str = "trait ReviewIdentity<T = DerivedUserRelease> { type Repr; }\nimpl<T> ReviewIdentity<T> for () { type Repr = T; }\n";
    let repros: [(&str, &[&str]); 4] = [
        ("", &["the type alias Repr in the defining module of DerivedUserRelease"]),
        (
            "fn review_macro_alias() -> DerivedUserRelease { vec![{ type UserReleaseInputs = <() as ReviewIdentity>::Repr; UserReleaseInputs { _derived: () } }].pop().unwrap() }\n",
            &["the type alias UserReleaseInputs in the defining module", "a struct literal in a macro's tokens of UserReleaseInputs in the defining module"],
        ),
        (
            "type r#const = <() as ReviewIdentity>::Repr;\nfn review_raw_keyword() -> DerivedUserRelease { vec![r#const { _derived: () }].pop().unwrap() }\n",
            &["the raw identifier r#const in a reporting crate", "the type alias const in the defining module", "a struct literal in a macro's tokens of const in the defining module"],
        ),
        (
            "type ReviewAlias = <() as ReviewIdentity>::Repr;\nenum ReviewMarker<const N: usize> { V }\nfn review_const_generic_pattern() -> bool { matches!(ReviewMarker::<0>::V, ReviewMarker::<{ let _entry = ProducerEntry::UserRelease(ReviewAlias { _derived: () }); 0 }>::V) }\n",
            &[
                "the type alias ReviewAlias in the defining module",
                "a brace or block expression in generic arguments in a reporting crate",
                "a const generic argument other than a literal in a reporting crate",
                "ProducerEntry::UserRelease is constructed outside assemble_derived (Macro)",
                "a struct literal in a macro's tokens of ReviewAlias in the defining module",
            ],
        ),
    ];
    let base = project_sources(&project());
    let lines = base.files["report_package/src/user_release.rs"].lines().count() + PRELUDE.lines().count();
    for (repro, expected) in repros {
        let mut sources = base.clone();
        sources.files.get_mut("report_package/src/user_release.rs").unwrap().push_str(&format!("{PRELUDE}{repro}"));
        let findings = scan(&sources);
        for expected in expected {
            // The repro's own findings are on its lines, after the prelude's.
            let on_repro = |finding: &&String| repro.is_empty() || finding.split(':').nth(1).and_then(|line| line.parse::<usize>().ok()).is_some_and(|line| line > lines);
            assert!(findings.iter().filter(on_repro).any(|finding| finding.contains(expected)), "{repro}: expected {expected:?} in {findings:#?}");
        }
    }
}

/// Codex round 2 at #1682 (P2), verbatim: a literal through an associated
/// type assigned `Self`.
const CODEX_R2_ALIAS: &str = "trait Identity { type Repr; }\nimpl Identity for DerivedUserRelease { type Repr = Self; }\ntype Alias<T> = <T as Identity>::Repr;\nfn forge() -> DerivedUserRelease { Alias::<DerivedUserRelease> { _derived: () } }\n";

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
#![forbid(unsafe_code)]
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
#![forbid(unsafe_code)]
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
#![forbid(unsafe_code)]
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
    // Codex round 2 at #1682 (P2): an associated type can name
    // `DerivedUserRelease` under another path, which a struct literal then
    // constructs. Every trait impl for it (none is listed), every associated
    // type in an impl for it, and in its defining module every struct literal
    // whose path is not one plain name of a struct the crate defines (no
    // alias sharing it) are findings.
    const IDENTITY: &str = "trait Identity { type Repr; }\n";
    const BLANKET: &str = "trait Identity { type Repr; }\nimpl<T> Identity for T { type Repr = T; }\n";
    const DEFAULTED: &str = "pub(crate) struct W<T = DerivedUserRelease>(T);\ntrait Identity { type Repr; }\nimpl<T> Identity for W<T> { type Repr = T; }\ntype A = <W as Identity>::Repr;\n";
    let repro = format!("{IDENTITY}impl Identity for DerivedUserRelease {{ type Repr = Self; }}\ntype Alias<T> = <T as Identity>::Repr;\nfn forge() -> DerivedUserRelease {{ Alias::<DerivedUserRelease> {{ _derived: () }} }}\n");
    let aliases: Vec<(&str, Sources, &str)> = vec![
        ("Codex's repro: the trait impl", appended(USER_RELEASE, &repro), "an impl of the trait Identity for DerivedUserRelease, which is not on the allow-list"),
        ("Codex's repro: its associated type", appended(USER_RELEASE, &repro), "the associated type Repr assigned Self in an impl for DerivedUserRelease"),
        ("Codex's repro: the literal", appended(USER_RELEASE, &repro), "a struct literal with generic arguments (Alias) in the defining module"),
        (
            "a trait in another module",
            appended(USER_RELEASE, "mod other {\n    pub trait Identity { type Repr; }\n}\nimpl other::Identity for DerivedUserRelease { type Repr = Self; }\nfn forge() -> DerivedUserRelease { <DerivedUserRelease as other::Identity>::Repr { _derived: () } }\n"),
            "an impl of the trait other::Identity for DerivedUserRelease",
        ),
        (
            "a trait in another module: the literal",
            appended(USER_RELEASE, "mod other {\n    pub trait Identity { type Repr; }\n}\nimpl other::Identity for DerivedUserRelease { type Repr = Self; }\nfn forge() -> DerivedUserRelease { <DerivedUserRelease as other::Identity>::Repr { _derived: () } }\n"),
            "a struct literal through a qualified <T as Trait>:: path",
        ),
        (
            "a generic associated type",
            appended(USER_RELEASE, "trait Identity { type Repr<T>; }\nimpl Identity for DerivedUserRelease { type Repr<T> = Self; }\nfn forge() -> DerivedUserRelease { <DerivedUserRelease as Identity>::Repr::<()> { _derived: () } }\n"),
            "the associated type Repr assigned Self in an impl for DerivedUserRelease",
        ),
        (
            "a generic associated type: the literal",
            appended(USER_RELEASE, "trait Identity { type Repr<T>; }\nimpl Identity for DerivedUserRelease { type Repr<T> = Self; }\nfn forge() -> DerivedUserRelease { <DerivedUserRelease as Identity>::Repr::<()> { _derived: () } }\n"),
            "a struct literal through a qualified <T as Trait>:: path",
        ),
        (
            "<DerivedUserRelease as Trait>::Assoc { .. } through a blanket impl",
            appended(USER_RELEASE, &format!("{BLANKET}fn forge() -> DerivedUserRelease {{ <DerivedUserRelease as Identity>::Repr {{ _derived: () }} }}\n")),
            "a struct literal through a qualified <T as Trait>:: path (Identity::Repr)",
        ),
        (
            "a generic alias through a blanket impl",
            appended(USER_RELEASE, &format!("{BLANKET}type Alias<T> = <T as Identity>::Repr;\nfn forge() -> DerivedUserRelease {{ Alias::<DerivedUserRelease> {{ _derived: () }} }}\n")),
            "a struct literal with generic arguments (Alias)",
        ),
        (
            "a plain alias through a defaulted parameter",
            appended(USER_RELEASE, &format!("{DEFAULTED}fn forge() -> DerivedUserRelease {{ A {{ _derived: () }} }}\n")),
            "a struct literal of A in the defining module of DerivedUserRelease, which is not a struct the crate defines",
        ),
        (
            "that alias inside a macro's tokens",
            appended(USER_RELEASE, &format!("{DEFAULTED}fn forge() -> Vec<DerivedUserRelease> {{ vec![A {{ _derived: () }}] }}\n")),
            "a struct literal in a macro's tokens of A",
        ),
        (
            "generic arguments inside a macro's tokens",
            appended(USER_RELEASE, &format!("{BLANKET}type Alias<T> = <T as Identity>::Repr;\nfn forge() -> Vec<DerivedUserRelease> {{ vec![Alias::<DerivedUserRelease> {{ _derived: () }}] }}\n")),
            "a struct literal with generic arguments in a macro's tokens",
        ),
        (
            "Self::Assoc in another type's impl",
            appended(USER_RELEASE, "pub(crate) struct W;\ntrait Identity { type Repr; fn forge() -> Self::Repr; }\nimpl Identity for W { type Repr = <W as Next>::Out; fn forge() -> Self::Repr { Self::Repr { _derived: () } } }\n"),
            "a struct literal of Self::Repr in the defining module of DerivedUserRelease, which is not one plain struct name",
        ),
        (
            "a qualified path in a macro's tokens",
            appended(USER_RELEASE, "pub(crate) struct W;\nimpl Identity for W { type Repr = <W as Next>::Out; fn forge() -> Vec<Self::Repr> { vec![Self::Repr { _derived: () }] } }\n"),
            "a struct literal of a qualified path ending in Repr in a macro's tokens",
        ),
        ("a Self literal in a trait's default method", appended(USER_RELEASE, "trait Make: Sized { fn make() -> Self { Self { _derived: () } } }\n"), "a Self literal outside an impl of one plain struct name"),
        (
            "a Self literal in an impl of an alias",
            appended(USER_RELEASE, &format!("{DEFAULTED}impl A {{ fn forge() -> Self {{ Self {{ _derived: () }} }} }}\n")),
            "a Self literal of A in the defining module",
        ),
        (
            "a renamed import as a literal",
            appended(USER_RELEASE, "mod inner {\n    use super::Basis as Rogue;\n    fn forge() -> Rogue { Rogue { note: None } }\n}\n"),
            "a struct literal of Rogue in the defining module",
        ),
        ("a leading :: literal", appended(USER_RELEASE, "fn forge() -> W { ::W { a: 1 } }\n"), "a struct literal of W in the defining module of DerivedUserRelease, which is not one plain struct name"),
        ("a trait impl for a reference to it", appended(USER_RELEASE, &format!("{IDENTITY}impl Identity for &DerivedUserRelease {{ type Repr = u8; }}\n")), "an impl of the trait Identity for DerivedUserRelease"),
        (
            "an alias after an if-else block in a macro's tokens",
            appended(USER_RELEASE, &format!("{DEFAULTED}fn forge(c: bool) -> Vec<u8> {{ vec![if c {{ 1 }} else {{ 2 }} + A {{ _derived: () }}] }}\n")),
            "a struct literal in a macro's tokens of A",
        ),
        ("a trait impl for it from another file", appended(LIB, "impl Default for crate::user_release::DerivedUserRelease { fn default() -> Self { todo() } }\n"), "an impl of the trait Default for DerivedUserRelease"),
    ];
    // Codex round 2 at #1682 (P2): `ProducerEntry::UserRelease` constructed in
    // braced form, or anywhere in a `matches!` but its pattern.
    let constructions: Vec<(&str, Sources, &str)> = vec![
        ("the braced tuple-variant form", appended("report_package/src/bindings.rs", "fn braced(derived: DerivedUserRelease) -> ProducerEntry { ProducerEntry::UserRelease { 0: derived } }\n"), "ProducerEntry::UserRelease is constructed outside assemble_derived (Fn(\"braced\"))"),
        ("the braced form through Self", appended(USER_RELEASE, "impl ProducerEntry { fn forge(derived: DerivedUserRelease) -> Self { Self::UserRelease { 0: derived } } }\n"), "ProducerEntry::UserRelease is constructed outside assemble_derived (Method(\"forge\"))"),
        ("the braced form in a macro's tokens", appended("report_package/src/bindings.rs", "fn braced(derived: DerivedUserRelease) -> Vec<ProducerEntry> { vec![ProducerEntry::UserRelease { 0: derived }] }\n"), "ProducerEntry::UserRelease is constructed outside assemble_derived (Macro)"),
        ("a matches! guard", appended("report_package/src/bindings.rs", "fn guarded(derived: DerivedUserRelease) -> bool { matches!((), _ if { let _entry = ProducerEntry::UserRelease(derived); true }) }\n"), "ProducerEntry::UserRelease is constructed outside assemble_derived (Macro)"),
        ("an if let guard", appended("report_package/src/bindings.rs", "fn guarded(derived: DerivedUserRelease) -> bool { matches!(Some(1), Some(_) if let Some(_entry) = Some(ProducerEntry::UserRelease(derived))) }\n"), "ProducerEntry::UserRelease is constructed outside assemble_derived (Macro)"),
        ("a matches! scrutinee", appended("report_package/src/bindings.rs", "fn scrutinee(derived: DerivedUserRelease) -> bool { matches!(ProducerEntry::UserRelease(derived), ProducerEntry::UserRelease(_)) }\n"), "ProducerEntry::UserRelease is constructed outside assemble_derived (Macro)"),
        ("a guard after an or-pattern", appended("report_package/src/bindings.rs", "fn guarded(entry: &ProducerEntry, derived: DerivedUserRelease) -> bool { matches!(entry, ProducerEntry::UserRelease(_) | ProducerEntry::General if vec![ProducerEntry::UserRelease(derived)].is_empty()) }\n"), "ProducerEntry::UserRelease is constructed outside assemble_derived (Macro)"),
        ("a matches! in another macro", appended("report_package/src/bindings.rs", "fn nested(derived: DerivedUserRelease) -> String { format!(\"{}\", matches!((), _ if vec![ProducerEntry::UserRelease(derived)].is_empty())) }\n"), "ProducerEntry::UserRelease is constructed outside assemble_derived (Macro)"),
        ("assert!(matches!(..))", appended("report_package/src/bindings.rs", "fn asserted(derived: DerivedUserRelease) { assert!(matches!((), _ if { let _entry = ProducerEntry::UserRelease(derived); true })); }\n"), "ProducerEntry::UserRelease is constructed outside assemble_derived (Macro)"),
        ("an inline const pattern", appended("report_package/src/bindings.rs", "fn constant(derived: DerivedUserRelease) -> bool { matches!((), const { let _entry = ProducerEntry::UserRelease(derived); }) }\n"), "ProducerEntry::UserRelease is constructed outside assemble_derived (Macro)"),
        ("a matches! that does not parse", appended("report_package/src/bindings.rs", "fn broken() -> bool { matches!(1) }\n"), "a matches! that does not parse as an expression, a pattern and a guard"),
    ];
    let cases: Vec<(&str, Sources, &str)> = cases.into_iter().chain(spellings).chain(conditional).chain(aliases).chain(constructions).collect();
    for (case, sources, expected) in cases {
        let findings = scan(&sources);
        assert!(findings.iter().any(|finding| finding.contains(expected)), "{case}: expected {expected:?} in {findings:#?}");
    }
}

/// Code review r3 at #1682: the conservative rules that close whole classes
/// rather than model each form. In the defining subtree every type alias is
/// a finding (none is listed), only the listed macros are invoked and their
/// tokens are parsed and visited as code; in the three reporting crates every
/// raw identifier (none is listed) and every block in generic arguments is a
/// finding; and syntax `syn` does not model is a finding. Each case is a
/// finding.
#[test]
fn user_release_reach_scan_finds_the_review_r3_classes() {
    const RENDERER: &str = "report_renderer/src/lib.rs";
    const PDF: &str = "pdf_emitter/src/lib.rs";
    const BINDINGS: &str = "report_package/src/bindings.rs";
    let alias = "the type alias Local in the defining module of DerivedUserRelease, which is not on the alias allow-list";
    let cases: Vec<(&str, Sources, &str)> = vec![
        // Type aliases in the defining subtree, in every position.
        ("an item-level alias", appended(USER_RELEASE, "type Local = u8;\n"), alias),
        ("an alias in a block", appended(USER_RELEASE, "fn f() -> u8 { type Local = u8; 1 }\n"), alias),
        ("an alias in an inline module", appended(USER_RELEASE, "mod inner {\n    pub(crate) type Local = u8;\n}\n"), alias),
        ("an associated type in an impl", appended(USER_RELEASE, "pub(crate) struct Counter;\nimpl Iterator for Counter { type Local = u8; fn next(&mut self) -> Option<u8> { None } }\n"), alias),
        ("an associated type in a trait", appended(USER_RELEASE, "trait Shape { type Local; }\n"), alias),
        ("an alias in vec!", appended(USER_RELEASE, "fn f() -> Vec<u8> { vec![{ type Local = u8; 1 }] }\n"), alias),
        ("an alias in format!", appended(USER_RELEASE, "fn f() -> String { format!(\"{}\", { type Local = u8; 1 }) }\n"), alias),
        ("an alias in format_args!", appended(USER_RELEASE, "fn f() { let _ = format_args!(\"{}\", { type Local = u8; 1 }); }\n"), alias),
        ("an alias in a json! value", appended(USER_RELEASE, "fn f() -> Value { json!({\"a\": [({ type Local = u8; 1 })]}) }\n"), alias),
        ("an alias in a matches! guard", appended(USER_RELEASE, "fn f() -> bool { matches!(1, _ if { type Local = u8; true }) }\n"), alias),
        ("an alias in a nested listed macro", appended(USER_RELEASE, "fn f() -> String { format!(\"{:?}\", vec![{ type Local = u8; 1 }]) }\n"), alias),
        ("println! in the defining subtree", appended(USER_RELEASE, "fn f() { println!(\"{}\", { type Local = u8; 1 }); }\n"), "the macro println! in the defining module of DerivedUserRelease is not on its parsed-macro list"),
        ("println!'s unparsed block", appended(USER_RELEASE, "fn f() { println!(\"{}\", { type Local = u8; 1 }); }\n"), "a brace group in tokens the scan does not parse, in the defining module of DerivedUserRelease"),
        ("a listed macro with other tokens", appended(USER_RELEASE, "fn f() -> Value { json!({ type Local = u8; 1 }) }\n"), "the tokens of json! do not parse by its grammar"),
        ("a json! object entry without a key", appended(USER_RELEASE, "fn f() -> Value { json!({\"a\": 1,, \"b\": 2}) }\n"), "the tokens of json! do not parse by its grammar"),
        ("assert! in the defining subtree", appended(USER_RELEASE, "fn f() { assert!(true); }\n"), "the macro assert! in the defining module of DerivedUserRelease is not on its parsed-macro list"),
        ("json! by its qualified path in the defining subtree", appended(USER_RELEASE, "fn f() -> Value { serde_json::json!(null) }\n"), "the macro serde_json::json! in the defining module of DerivedUserRelease is not on its parsed-macro list"),
        ("a struct literal of an alias in parsed json! tokens", appended(USER_RELEASE, "pub(crate) struct Plain { a: u8 }\ntype Other = Plain;\nfn f() -> Value { json!({\"a\": Other { a: 1 }.a}) }\n"), "a struct literal in a macro's tokens of Other"),
        // Raw identifiers in the three reporting crates.
        ("r#match as a function name", appended(PDF, "fn r#match() {}\n"), "the raw identifier r#match in a reporting crate"),
        ("r#if as a binding", appended(RENDERER, "fn f() { let r#if = 1; let _ = r#if; }\n"), "the raw identifier r#if in a reporting crate"),
        ("r#if in a macro's tokens", appended(LIB, "fn f(r: u8) { let _ = format!(\"{}\", r#if); }\n"), "the raw identifier r#if in a reporting crate"),
        ("a raw identifier in an attribute", appended(LIB, "#[allow(r#unused)]\nfn f() {}\n"), "the raw identifier r#unused in a reporting crate"),
        ("a raw identifier in an inline test module", appended(RENDERER, "#[cfg(test)]\nmod tests {\n    fn r#match() {}\n}\n"), "the raw identifier r#match in a reporting crate"),
        ("r#if as a struct literal's name in the defining subtree", appended(USER_RELEASE, "fn f() -> Vec<u8> { vec![r#if { a: 1 }.a] }\n"), "a struct literal in a macro's tokens of if"),
        // Blocks in generic arguments in the three reporting crates.
        ("a block argument in type position", appended(RENDERER, "fn f() { let _: Marker<{ 1 + 1 }> = Marker; }\n"), "a brace or block expression in generic arguments in a reporting crate"),
        ("a block argument in an expression", appended(PDF, "fn f() { let _ = Marker::<{ 1 }>::new(); }\n"), "a brace or block expression in generic arguments in a reporting crate"),
        ("a block argument after another argument", appended(LIB, "fn f() { let _ = Pair::<u8, { 2 }>::new(); }\n"), "a brace or block expression in generic arguments in a reporting crate"),
        ("a block argument in a pattern", appended(LIB, "fn f(m: Marker<0>) { match m { Marker::<{ 0 }>::V => {} } }\n"), "a const generic argument other than a literal in a reporting crate"),
        ("a block argument in another macro's tokens", appended(BINDINGS, "fn f() -> Vec<Marker<0>> { vec![Marker::<{ 0 }>::V] }\n"), "a brace or block expression in generic arguments in a reporting crate"),
        ("a block as a const parameter's default", appended(RENDERER, "pub struct Sized<const N: usize = { 1 }>;\n"), "a brace or block expression in generic arguments in a reporting crate"),
        ("a block argument in a qualified type", appended(PDF, "fn f() -> usize { <Marker<{ 1 }> as Count>::N }\n"), "a brace or block expression in generic arguments in a reporting crate"),
        (
            "a construction in an array length in a matches! pattern",
            appended(BINDINGS, "fn f(derived: DerivedUserRelease) -> bool { matches!(0, <[u8; { let _entry = ProducerEntry::UserRelease(derived); 1 }] as Count>::N) }\n"),
            "ProducerEntry::UserRelease is constructed outside assemble_derived (Macro)",
        ),
        // Syntax syn does not model.
        ("a box pattern", appended(LIB, "fn f(x: Box<u8>) { match x { box y => {} } }\n"), "a pattern syn gives as Verbatim"),
        ("a builtin expression", appended(PDF, "fn f() -> usize { builtin # offset_of(S, f) }\n"), "an expression syn gives as Verbatim"),
    ];
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
        // Codex round 2 at #1682 (P2): the forms its rules admit.
        (
            "plain literals of the crate's structs in the defining module",
            appended(
                USER_RELEASE,
                "pub(crate) struct Plain { a: u8 }\nimpl Plain { fn make() -> Self { Self { a: 1 } } }\nfn plain() -> Plain { Plain { a: 1 } }\nfn in_macro(flag: bool, items: &[u8]) -> Vec<Plain> { let _ = json!({\"a\": if flag { 1 } else { 2 }, \"b\": match flag { true => 1, false => 2 }}); let _ = format!(\"{}\", { for item in items { let _ = item; } 1 }); vec![Plain { a: 1 }] }\nfn pattern(p: &Plain) -> bool { matches!(p, Plain { a: 1 }) }\nimpl DerivedUserRelease { fn is_derived(&self) -> bool { true } }\n",
            ),
        ),
        (
            "matches! patterns naming UserRelease with a guard and a trailing comma",
            appended("report_package/src/bindings.rs", "fn arms(entry: &ProducerEntry) -> bool { matches!(entry, ProducerEntry::UserRelease(_) | ProducerEntry::General if true) || matches!(entry, ProducerEntry::UserRelease(_),) }\n"),
        ),
        // Code review r3 at #1682: ordinary code passes the conservative rules.
        (
            "ordinary generics, comparisons and blocks in the reporting crates",
            appended(
                "report_renderer/src/lib.rs",
                "pub struct M<const N: usize>;\npub const K: usize = 2;\nfn g() -> M<2> { M::<2> }\nfn h<T: Clone>(t: &T) -> Vec<T> { if 1 < 2 { vec![t.clone()] } else { Vec::new() } }\nfn k(a: u8, b: u8) -> bool { let c = a < b; let d = { a <= b }; let e: Vec<Vec<u8>> = { Vec::new() }; c && d && e.is_empty() }\nfn arms(a: u8) -> u8 { match a { x if x < 3 => { 1 } _ => { 2 } } }\nfn shifts(a: u32) -> u32 { (a << 2) >> 1 }\nfn closure() -> impl Fn(u8) -> bool { |a| { a < 1 } }\n",
            ),
        ),
        ("raw string literals in a reporting crate", appended("pdf_emitter/src/lib.rs", "const S: &str = r#\"{\"a\": 1}\"#;\nconst T: &str = r##\"r#match\"##;\n")),
        ("a raw identifier in src-tauri, outside the three crates", appended(TAURI, "fn typed() -> u8 { let r#type = 1; r#type }\n")),
        ("a type alias outside the defining subtree", appended(LIB, "type Local = u8;\nfn f() -> Local { 1 }\n")),
        (
            "the listed macros' grammars in the defining subtree",
            appended(
                USER_RELEASE,
                "pub(crate) struct Plain { a: u8 }\nconst KEY: &str = \"k\";\nfn listed(flag: bool, items: &[u8], name: &str) -> Value {\n    let _ = format!(\"{name} {} {count}\", items.len(), count = items.len());\n    let _ = format_args!(\"{}\", 1);\n    let _ = vec![0u8; items.len()];\n    let _ = vec![Plain { a: 1 }, Plain { a: 2 },];\n    let _ = matches!(flag, true if items.is_empty(),);\n    json!({\n        \"a\": [1, {\"b\": null}, [true, false],],\n        KEY: if flag { 1 } else { 2 },\n        (format!(\"{}\", name)): items.iter().map(|item| json!({\"i\": item})).collect::<Vec<_>>(),\n        \"c\": Value::Null,\n        \"d\": -1,\n    })\n}\n",
            ),
        ),
    ];
    for (case, sources) in passing {
        let findings = scan(&sources);
        assert!(findings.is_empty(), "{case}: {findings:#?}");
    }
}

/// Code review r4 at #1682 (P2s), verbatim on the real sources: each compiled
/// in `report_package` and gave no finding. `unsafe` inside a macro's
/// arguments is now a finding of the token rule (and the crate root's
/// `#![forbid(unsafe_code)]` makes the compiler refuse it); a local `mod
/// tauri` re-exporting `std::include` under a listed macro's name is a
/// finding of each macro namespace rule it breaks. A real crate root without
/// the exact `forbid`, or with it altered, is a finding.
#[test]
fn the_real_sources_find_the_review_r4_forms() {
    const ROOT: &str = "report_package/src/lib.rs";
    const FORBID: &str = "#![forbid(unsafe_code)]\n";
    let base = project_sources(&project());
    let lines = base.files[ROOT].lines().count();
    let repros: [(&str, &[&str]); 3] = [
        ("pub(crate) fn forge() -> user_release::DerivedUserRelease { vec![unsafe { std::mem::zeroed() }].pop().unwrap() }\n", &["the unsafe keyword in a reporting crate"]),
        ("pub(crate) fn forge() -> user_release::ProducerEntry { vec![unsafe { std::mem::transmute::<u8, user_release::ProducerEntry>(1) }].pop().unwrap() }\n", &["the unsafe keyword in a reporting crate"]),
        (
            "mod tauri { pub use std::include as generate_context; }\ntauri::generate_context!(\"review_escape.rs\");\n",
            &[
                "the module tauri takes the reserved macro-path name tauri",
                "the import std::include as generate_context, a rename not on the rename allow-list",
                "a non-private use of std::include, which is not on the re-export allow-list",
                "the import std::include binds the source macro include",
            ],
        ),
    ];
    for (repro, expected) in repros {
        let mut sources = base.clone();
        sources.files.get_mut(ROOT).unwrap().push_str(repro);
        sources.files.insert("report_package/src/review_escape.rs".to_string(), "user_release::ProducerEntry::UserRelease(unsafe { std::mem::zeroed() })\n".to_string());
        let findings = scan(&sources);
        let on_repro = |finding: &&String| finding.starts_with(ROOT) && finding.split(':').nth(1).and_then(|line| line.parse::<usize>().ok()).is_some_and(|line| line > lines);
        for expected in expected {
            assert!(findings.iter().filter(on_repro).any(|finding| finding.contains(expected)), "{repro}: expected {expected:?} in {findings:#?}");
        }
    }
    for root in ["report_package/src/lib.rs", "report_renderer/src/lib.rs", "pdf_emitter/src/lib.rs"] {
        for (replacement, extra) in [
            ("", None),
            ("#![forbid(unsafe_code, dead_code)]\n", Some("the attribute #[forbid] is not on the attribute list")),
            ("#![deny(unsafe_code)]\n", Some("the attribute #[deny] is not on the attribute list")),
            ("#![cfg_attr(all(), forbid(unsafe_code))]\n", Some("the attribute #[forbid] (through cfg_attr) is not on the attribute list")),
        ] {
            let mut sources = base.clone();
            let text = sources.files.get_mut(root).unwrap();
            assert_eq!(text.matches(FORBID).count(), 1, "{root}");
            *text = text.replacen(FORBID, replacement, 1);
            let findings = scan(&sources);
            let missing = format!("{root}:0: the crate root does not carry #![forbid(unsafe_code)] exactly");
            assert!(findings.contains(&missing), "{root} {replacement:?}: {findings:#?}");
            if let Some(extra) = extra {
                assert!(findings.iter().any(|finding| finding.starts_with(root) && finding.contains(extra)), "{root} {replacement:?}: expected {extra:?} in {findings:#?}");
            }
        }
    }
}

/// Code review r4 at #1682: the compiler-enforced `forbid(unsafe_code)` and
/// the macro namespace rules. Each case is a finding.
#[test]
fn user_release_reach_scan_finds_the_review_r4_classes() {
    const RENDERER: &str = "report_renderer/src/lib.rs";
    const PDF: &str = "pdf_emitter/src/lib.rs";
    const BINDINGS: &str = "report_package/src/bindings.rs";
    const FORBID: &str = "#![forbid(unsafe_code)]\n";
    let unsafe_keyword = "the unsafe keyword in a reporting crate";
    let missing = "the crate root does not carry #![forbid(unsafe_code)] exactly";
    let mut cases: Vec<(String, Sources, &str)> = Vec::new();
    // `unsafe` in macro arguments, in each reporting crate (outside the
    // defining subtree, where macro tokens are only screened).
    for file in [LIB, BINDINGS, RENDERER, PDF] {
        for (form, text) in [
            ("vec!", "fn forge() -> Vec<u8> { vec![unsafe { std::mem::zeroed() }] }\n"),
            ("format!", "fn forge() -> String { format!(\"{}\", unsafe { std::mem::zeroed::<u8>() }) }\n"),
            ("assert!", "fn forge() { assert!(unsafe { std::mem::zeroed::<bool>() }); }\n"),
            ("transmute in vec!", "fn forge() -> Vec<u8> { vec![unsafe { std::mem::transmute::<u8, u8>(1) }] }\n"),
        ] {
            cases.push((format!("unsafe in {form} in {file}"), appended(file, text), unsafe_keyword));
        }
    }
    // The crate-root attribute: missing, altered, or elsewhere.
    for root in [LIB, RENDERER, PDF] {
        cases.push((format!("{root} without the forbid"), edited(root, FORBID, ""), missing));
        cases.push((format!("{root} with another lint added"), edited(root, FORBID, "#![forbid(unsafe_code, dead_code)]\n"), missing));
        cases.push((format!("{root} with deny"), edited(root, FORBID, "#![deny(unsafe_code)]\n"), missing));
        cases.push((format!("{root} with the forbid as an outer attribute"), edited(root, FORBID, "#[forbid(unsafe_code)]\nfn f() {}\n"), missing));
    }
    let more: Vec<(&str, Sources, &str)> = vec![
        ("a forbid in a non-root file", appended(BINDINGS, "#[forbid(unsafe_code)]\nfn f() {}\n"), "the attribute #[forbid] is not on the attribute list"),
        // (c) Local items named like a macro path root or a listed macro.
        ("Codex's mod tauri shadow", appended(LIB, "mod tauri { pub use std::include as generate_context; }\ntauri::generate_context!(\"escape.rs\");\n"), "the module tauri takes the reserved macro-path name tauri"),
        ("mod tauri in src-tauri", appended(TAURI, "mod tauri {}\n"), "the module tauri takes the reserved macro-path name tauri"),
        ("mod serde_json in a block", appended(PDF, "fn f() { mod serde_json {} }\n"), "the module serde_json takes the reserved macro-path name serde_json"),
        ("mod std", appended(RENDERER, "mod std {}\n"), "the module std takes the reserved macro-path name std"),
        ("extern crate std as tauri", appended(LIB, "extern crate std as tauri;\n"), "extern crate binds the reserved macro-path name tauri"),
        ("a use binding named tauri", appended(TAURI, "mod shadow {}\nuse crate::shadow as tauri;\n"), "binds the listed name tauri"),
        ("a use binding named core", appended(PDF, "use crate::inner as core;\n"), "binds the listed name core"),
        ("a macro_rules named vec", appended(LIB, "macro_rules! vec { ($e:expr) => { $e }; }\n"), "the macro definition vec takes the reserved macro-path name vec"),
        ("a macro_rules named json", appended(BINDINGS, "macro_rules! json { ($e:expr) => { $e }; }\n"), "the macro definition json takes the reserved macro-path name json"),
        ("a macro_rules named tauri in src-tauri", appended(TAURI, "macro_rules! tauri { () => {}; }\n"), "the macro definition tauri takes the reserved macro-path name tauri"),
        // (a) Renames in a reporting crate's use items.
        ("use std::include as x", appended(LIB, "use std::include as x;\nfn f() { x!(\"escape.rs\"); }\n"), "the import std::include as x, a rename not on the rename allow-list"),
        ("its invocation", appended(LIB, "use std::include as x;\nfn f() { x!(\"escape.rs\"); }\n"), "the macro x! is not on the invocation list"),
        ("a rename of an ordinary item", appended(RENDERER, "use std::collections::BTreeMap as Map;\n"), "the import std::collections::BTreeMap as Map, a rename not on the rename allow-list"),
        ("a rename to _", appended(PDF, "use std::fmt::Write as _;\n"), "the import std::fmt::Write as _, a rename not on the rename allow-list"),
        ("an allow-listed name at another path", appended(LIB, "use forwarder as pdf;\n"), "the import forwarder as pdf, a rename not on the rename allow-list"),
        // (b) Non-private uses in a reporting crate.
        ("pub use std::include_str as y", appended(LIB, "pub use std::include_str as y;\n"), "a non-private use of std::include_str, which is not on the re-export allow-list"),
        ("its rename", appended(LIB, "pub use std::include_str as y;\n"), "the import std::include_str as y, a rename not on the rename allow-list"),
        ("pub(crate) use of a macro", appended(BINDINGS, "pub(crate) use std::vec;\n"), "a non-private use of std::vec, which is not on the re-export allow-list"),
        ("pub(super) use of an item", appended(RENDERER, "mod inner { pub(super) use std::collections::BTreeMap; }\n"), "a non-private use of std::collections::BTreeMap"),
        ("an allow-listed re-export in another file", appended(BINDINGS, "pub use open_pipe_stress_report_renderer::serde_error_kind;\n"), "a non-private use of open_pipe_stress_report_renderer::serde_error_kind, which is not on the re-export allow-list"),
        // (d) The source macros by any path, invoked or imported.
        ("std::include!", appended(LIB, "std::include!(\"escape.rs\");\n"), "std::include! brings in source the scan does not read"),
        ("core::include! in src-tauri", appended(TAURI, "core::include!(\"escape.rs\");\n"), "core::include! brings in source the scan does not read"),
        ("include_bytes! bare", appended(PDF, "const B: &[u8] = include_bytes!(\"font.bin\");\n"), "include_bytes! is refused by any path in production"),
        ("std::include_bytes!", appended(RENDERER, "const B: &[u8] = std::include_bytes!(\"font.bin\");\n"), "std::include_bytes! is refused by any path in production"),
        ("std::include_str!", appended(LIB, "const S: &str = std::include_str!(\"../schema.json\");\n"), "std::include_str! names include_str by a path other than its bare name"),
        ("concat_idents!", appended(LIB, "fn f() { concat_idents!(assemble_, derived)(); }\n"), "concat_idents! is refused by any path in production"),
        ("use std::include", appended(BINDINGS, "use std::include;\n"), "the import std::include binds the source macro include"),
        ("use core::{include_bytes}", appended(TAURI, "use core::{include_bytes};\n"), "the import core::include_bytes binds the source macro include_bytes"),
        ("include! nested in another macro's tokens", appended(LIB, "fn f() -> String { format!(\"{}\", std::include!(\"escape.rs\")) }\n"), "std::include! brings in source the scan does not read"),
        ("include_str! of a .rs file nested in tokens", appended(PDF, "fn f() -> String { format!(\"{}\", include_str!(\"escape.rs\")) }\n"), "include_str! takes one string literal that names no .rs file"),
        // Items in tokens the scan does not parse.
        (
            "the shadow inside format! tokens",
            appended(LIB, "fn f() -> String { format!(\"{}\", { mod tauri { pub use std::include as generate_context; } tauri::generate_context!(\"escape.rs\") }) }\n"),
            "the item keyword mod in tokens the scan does not parse",
        ),
        ("a use inside assert! tokens", appended(RENDERER, "fn f() { assert!({ use std::vec as format; true }); }\n"), "the item keyword use in tokens the scan does not parse"),
        ("an extern crate inside println! tokens", appended(TAURI, "fn f() { println!(\"{}\", { extern crate std as tauri; 1 }); }\n"), "the item keyword extern in tokens the scan does not parse"),
        ("a macro_rules! inside format! tokens", appended(PDF, "fn f() -> String { format!(\"{}\", { macro_rules! json { ($e:expr) => { $e }; } json!(1) }) }\n"), "a macro_rules! definition in tokens the scan does not parse"),
        ("a macro 2.0 item inside vec! tokens", appended(LIB, "fn f() -> Vec<u8> { vec![{ macro json($e:expr) { $e } json!(1) }] }\n"), "the item keyword macro in tokens the scan does not parse"),
        // (e) #[macro_use] on a module and on extern crate.
        ("#[macro_use] on an inline module", appended(PDF, "#[macro_use]\nmod inline_macros {}\n"), "#[macro_use]"),
        ("#[macro_use] on an outlined module", with_file_text("report_package/src/macros.rs", "\n", appended(LIB, "#[macro_use]\nmod macros;\n")), "#[macro_use]"),
        ("#[macro_use] extern crate std", appended(RENDERER, "#[macro_use]\nextern crate std;\n"), "#[macro_use]"),
    ];
    cases.extend(more.into_iter().map(|(case, sources, expected)| (case.to_string(), sources, expected)));
    for (case, sources, expected) in cases {
        let findings = scan(&sources);
        assert!(findings.iter().any(|finding| finding.contains(expected)), "{case}: expected {expected:?} in {findings:#?}");
    }
}

/// `invented()`-based sources with one more file.
fn with_file_text(path: &str, text: &str, mut sources: Sources) -> Sources {
    sources.files.insert(path.to_string(), text.to_string());
    sources
}

/// Code review r4 at #1682: ordinary code passes the macro namespace rules
/// and the `unsafe` token rule.
#[test]
fn user_release_reach_scan_admits_the_review_r4_ordinary_forms() {
    let passing: Vec<(&str, Sources)> = vec![
        (
            "casts, ordinary modules and private imports in the reporting crates",
            appended(
                "report_renderer/src/lib.rs",
                "use std::collections::BTreeMap;\nuse std::fmt::Write;\nmod rendering {\n    pub(crate) fn width(a: u32) -> u8 { a as u8 }\n}\nfn f(a: u32, out: &mut String) -> u8 { let _ = write!(out, \"{}\", a as u8); let _: BTreeMap<u8, u8> = BTreeMap::new(); let _ = format!(\"{}\", a as u64); rendering::width(a) }\n",
            ),
        ),
        ("the word unsafe in strings and comments", appended("pdf_emitter/src/lib.rs", "// unsafe is refused here\nconst S: &str = \"unsafe\";\nconst T: &str = r#\"unsafe { }\"#;\nfn unsafe_named_like() -> &'static str { S }\n")),
        ("a bare include_str! of a data file in tokens", appended("pdf_emitter/src/lib.rs", "fn f() -> String { format!(\"{}\", include_str!(\"../schema.json\")) }\n")),
        ("src-tauri's own renames, underscore imports and re-exports", appended(TAURI, "use std::fmt::Write as _;\nuse std::collections::BTreeMap as Map;\npub use native_smoke::take_chosen_path;\nfn f() -> Map<u8, u8> { Map::new() }\n")),
        ("module names that share no reserved name", appended("report_package/src/lib.rs", "mod std_like {}\nmod formatting {}\nmod tauri_bridge {}\n")),
        ("a private use of a listed import in a reporting crate", appended("report_package/src/bindings.rs", "use serde_json::{json, Value};\nfn f() -> Value { json!({\"a\": 1}) }\n")),
    ];
    for (case, sources) in passing {
        let findings = scan(&sources);
        assert!(findings.is_empty(), "{case}: {findings:#?}");
    }
}
