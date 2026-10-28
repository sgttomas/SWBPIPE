//! The reach scan on the real sources (W-18b slice (r2c); DEL-08-01 Design
//! `report-user-release.md` §2.8, §6). Standard claim fence applies (claims
//! taxonomy per DEC-081).

use crate::user_release_reach_scan::{
    allowed_dependents, build_configuration, build_script_findings, cargo_configs, cargo_crate, crate_sources, dependent_findings, dependent_locations, expected_manifests, in_repo_build_scripts, in_repo_packages, manifest_overrides, path_module_key,
    path_packages, proc_macro_findings, project_manifests, project_sources, repository_root, resolved_identities, scan, scan_reading, scan_reading_all, CrateTarget, Dependents, Sources, BUILD_DEPENDENCIES, BUILD_SCRIPTS, CONSUMERS, CRATES, DEPENDENT_MANIFESTS,
    FOREIGN_BLOCKS, PROC_MACRO_PACKAGES, REPORTING_DEPENDENTS, REPORTING_PACKAGES, RUNNER_DEPENDENCIES, SHELL_DEPENDENCIES, SHELL_TEST_CALLERS, SHELL_UNSAFE_MODULES, TARGETS, TEST_PROOF_ITEMS,
};
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
    // Code review r6 at #1682: the build configuration under which the
    // scanned sources are the ones built.
    let findings = build_configuration(&sources, &project());
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

/// Also asserted (§2.8, "Dependents"; code review r8 at #1682): the
/// packages that depend on each reporting package are exactly the allowed
/// ones, by package name and canonical manifest, as Cargo reports every
/// package in the project and every path dependency reachable from them:
/// `report_package`'s are `src-tauri` and the headless runner, the
/// renderer's are `src-tauri`, `pdf_emitter` and `report_package`, and
/// `pdf_emitter`'s is `report_package`. No manifest text is matched (the
/// round-8 repro was a quoted key). Since code review r9 the scan reads the
/// headless runner's sources under the shell's rules
/// ([`the_scan_reads_the_headless_runner`]); the runner's own name check
/// (`tests/user_release_runner_refused.rs` there) is kept in addition.
#[test]
fn only_the_allowed_packages_depend_on_the_reporting_crates() {
    let project = project();
    let sources = project_sources(&project);
    let expected = expected_manifests(&sources).unwrap();
    let locations = dependent_locations(&project, &expected);
    let allowed = allowed_dependents(&locations).unwrap();
    let at = |manifest: &str| std::fs::canonicalize(project.join(manifest)).unwrap();
    let pair = |name: &str, manifest: &str| (name.to_string(), at(manifest));
    let shell = pair("openpipestress-desktop", "apps/desktop/src-tauri/Cargo.toml");
    let runner = pair("open_pipe_stress_headless_runner", "core/runner/headless/Cargo.toml");
    let package = pair("open_pipe_stress_report_package", "core/reporting/report_package/Cargo.toml");
    let pdf = pair("open_pipe_stress_pdf_emitter", "core/reporting/pdf_emitter/Cargo.toml");
    let exact: Dependents = BTreeMap::from([
        ("open_pipe_stress_report_package".to_string(), BTreeSet::from([shell.clone(), runner.clone()])),
        ("open_pipe_stress_report_renderer".to_string(), BTreeSet::from([shell.clone(), pdf.clone(), package.clone()])),
        ("open_pipe_stress_pdf_emitter".to_string(), BTreeSet::from([package.clone()])),
    ]);
    assert_eq!(allowed, exact);
    assert_eq!(REPORTING_DEPENDENTS.len(), REPORTING_PACKAGES.len());
    assert_eq!(DEPENDENT_MANIFESTS, [("openpipestress-desktop", CONSUMERS[0].1), ("open_pipe_stress_headless_runner", CONSUMERS[1].1)]);
    // Every manifest in the project is read by Cargo, and the found
    // dependents are exactly the allowed ones.
    let (manifests, unreadable) = project_manifests(&project);
    assert!(unreadable.is_empty(), "{unreadable:#?}");
    let read = in_repo_packages(&manifests, &expected);
    assert!(read.findings.is_empty(), "{:#?}", read.findings);
    assert_eq!(read.dependents, exact);
    assert!(dependent_findings(&read.dependents, &allowed).is_empty());
    for manifest in &manifests {
        assert!(read.packages.contains_key(&std::fs::canonicalize(manifest).unwrap()), "{} is not a package Cargo reported", manifest.display());
    }
    for (_, manifest) in exact.values().flatten() {
        assert!(read.packages.contains_key(manifest), "{}", manifest.display());
    }
    // The shell's resolved graph agrees.
    let repository = repository_root(&project).unwrap();
    assert!(resolved_identities("src-tauri", &at("apps/desktop/src-tauri"), &expected, &[repository], &allowed).is_empty());
    // A missing allowed dependent is a finding too: the list is exact.
    let mut fewer = read.dependents.clone();
    fewer.get_mut("open_pipe_stress_pdf_emitter").unwrap().clear();
    let findings = dependent_findings(&fewer, &allowed);
    assert_eq!(findings.len(), 1, "{findings:#?}");
    assert!(findings[0].starts_with("the allowed dependent open_pipe_stress_report_package (") && findings[0].ends_with("of open_pipe_stress_pdf_emitter does not depend on it, as Cargo reports; the list is exact"), "{findings:#?}");
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
    use super::*;
    fn check_request_refuses_markers_and_the_entry_binds_its_form() { let _entry = ProducerEntry::UserRelease(DerivedUserRelease { _derived: () }); }
}
"),
    );
    files.insert(
        "src-tauri/src/lib.rs".to_string(),
        file(&format!(
            "
#![deny(unsafe_code)]
#[allow(unsafe_code)]
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
    files.insert("src-tauri/src/main.rs".to_string(), file("#![deny(unsafe_code)]\nfn main() {\n    println!(\"{}\", 1);\n}\n"));
    // A test module that renders through the shell's own function (code
    // review r7 at #1682: a test-only renderer call is a finding).
    files.insert("src-tauri/src/report_render_exact_tests.rs".to_string(), file("fn t() { crate::render_calculation_report_value(json!({})); }\n"));
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
    // Code review r9 at #1682: the headless runner's roots, each carrying
    // `#![forbid(unsafe_code)]`.
    for root in ["headless/src/lib.rs", "headless/src/bin/headless_preview_runner.rs", "headless/src/bin/openpipestress-runner.rs"] {
        let main = if root.ends_with("lib.rs") { "" } else { "fn main() {}\n" };
        files.insert(root.to_string(), format!("#![forbid(unsafe_code)]\n{main}"));
    }
    let targets = TARGETS
        .iter()
        .map(|(key, targets)| {
            let targets = targets.iter().map(|(kind, name, src_path)| CrateTarget { kind: kind.to_string(), name: name.to_string(), src_path: src_path.to_string() }).collect();
            (key.to_string(), targets)
        })
        .collect();
    let build_dependencies = BUILD_DEPENDENCIES.iter().map(|(key, names)| (key.to_string(), names.iter().map(|name| name.to_string()).collect())).collect();
    // Code review r7 at #1682: the pinned build script and dependency set.
    let dependencies = BTreeMap::from([
        ("src-tauri".to_string(), SHELL_DEPENDENCIES.iter().map(|dependency| dependency.to_string()).collect()),
        ("headless".to_string(), RUNNER_DEPENDENCIES.iter().map(|dependency| dependency.to_string()).collect()),
    ]);
    let build_scripts = BTreeMap::from([("src-tauri/build.rs".to_string(), real_build_script())]);
    Sources { files, targets, build_dependencies, dependencies, build_scripts, bin_dirs: BTreeSet::new(), crate_dirs: BTreeMap::new() }
}

/// `src-tauri`'s real build script.
fn real_build_script() -> String {
    std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("build.rs")).unwrap()
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
        ("a bin target", {
            let mut sources = invented();
            sources.targets.get_mut("report_package").unwrap().push(CrateTarget { kind: "bin".to_string(), name: "x".to_string(), src_path: "report_package/src/lib.rs".to_string() });
            sources
        }, "report_package: Cargo builds the bin target x from report_package/src/lib.rs, which is not on the target allow-list"),
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
        // Read since code review r6 at #1682, under the test-only rules.
        ("a test-only outlined module", with_file_text("report_package/src/witness_tests.rs", "use super::*;\nfn w(i: &I) { pdf::assemble_full_report_package(i); assert_eq!(1, 1); }\n", appended(LIB, "#[cfg(test)]\nmod witness_tests;\n"))),
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

/// Code review r5 at #1682: Cargo reports exactly the allow-listed targets and
/// build-dependencies of each real crate ([`TARGETS`]), and the scan reads
/// each library root Cargo reports.
#[test]
fn cargo_reports_exactly_the_allow_listed_targets() {
    let project = project();
    for (key, root) in CRATES {
        let (targets, build_dependencies, dependencies) = cargo_crate(key, &project.join(root)).unwrap_or_else(|error| panic!("{key}: {error}"));
        if key == "src-tauri" {
            // Code review r7 at #1682: the pinned dependency set is today's.
            assert_eq!(dependencies, SHELL_DEPENDENCIES.iter().map(|dependency| dependency.to_string()).collect::<BTreeSet<_>>());
        }
        if key == "headless" {
            // Code review r9 at #1682: the runner's too.
            assert_eq!(dependencies, RUNNER_DEPENDENCIES.iter().map(|dependency| dependency.to_string()).collect::<BTreeSet<_>>());
        }
        let expected = TARGETS.iter().find(|(crate_key, _)| *crate_key == key).unwrap().1;
        let mut expected: Vec<CrateTarget> = expected.iter().map(|(kind, name, src_path)| CrateTarget { kind: kind.to_string(), name: name.to_string(), src_path: src_path.to_string() }).collect();
        expected.sort();
        assert_eq!(targets, expected, "{key}");
        let expected_build: BTreeSet<String> = BUILD_DEPENDENCIES.iter().find(|(crate_key, _)| *crate_key == key).unwrap().1.iter().map(|name| name.to_string()).collect();
        assert_eq!(build_dependencies, expected_build, "{key}");
    }
    let (findings, read) = scan_reading(&project_sources(&project));
    assert!(findings.is_empty(), "{findings:#?}");
    for root in ["src-tauri/src/lib.rs", "src-tauri/src/main.rs", "report_package/src/lib.rs", "report_renderer/src/lib.rs", "pdf_emitter/src/lib.rs"] {
        assert!(read.contains(root), "{root}: {read:?}");
    }
    for not_read in ["src-tauri/build.rs", "report_package/tests/container.rs", "pdf_emitter/tests/emit.rs"] {
        assert!(!read.contains(not_read), "{not_read} is read");
    }
}

/// Code review r5 at #1682 (P3): `#![forbid(unsafe_code)]` is admitted only
/// among the crate root's own `syn::File` attributes. An inner `#![forbid]` in
/// a nested module of a reporting `src/lib.rs`, inline or outlined, is a
/// finding; so is any other target set, on invented targets.
#[test]
fn user_release_reach_scan_finds_the_review_r5_classes() {
    let not_listed = "the attribute #[forbid] is not on the attribute list";
    let target = |key: &str, kind: &str, name: &str, src_path: &str| {
        let mut sources = invented();
        sources.targets.get_mut(key).unwrap().push(CrateTarget { kind: kind.to_string(), name: name.to_string(), src_path: src_path.to_string() });
        sources
    };
    let mut cases: Vec<(String, Sources, String)> = Vec::new();
    for root in [LIB, "report_renderer/src/lib.rs", "pdf_emitter/src/lib.rs"] {
        cases.push((format!("a nested inline module's #![forbid] in {root}"), appended(root, "mod extra_fence { #![forbid(unsafe_code)] }\n"), not_listed.to_string()));
        cases.push((format!("a nested module's #![forbid] in a block in {root}"), appended(root, "fn f() { mod extra_fence { #![forbid(unsafe_code)] } }\n"), not_listed.to_string()));
    }
    cases.push((
        "an outlined module's #![forbid]".to_string(),
        with_file_text("report_package/src/extra_fence.rs", "#![forbid(unsafe_code)]\n", appended(LIB, "mod extra_fence;\n")),
        format!("report_package/src/extra_fence.rs:1: {not_listed}"),
    ));
    for (kind, name, src_path) in [
        ("bin", "x", "report_renderer/src/main.rs"),
        ("example", "x", "report_renderer/examples/x.rs"),
        ("bench", "x", "report_renderer/benches/x.rs"),
        ("custom-build", "build-script-build", "report_renderer/build.rs"),
        ("proc-macro", "x", "report_renderer/src/lib.rs"),
        ("test", "x", "report_renderer/tests/x.rs"),
        ("test", "render", "report_renderer/tests/other.rs"),
        ("bin", "openpipestress-desktop-2", "src-tauri/src/bin/two.rs"),
    ] {
        let key = src_path.split('/').next().unwrap();
        cases.push((format!("an added {kind} target {name}"), target(key, kind, name, src_path), format!("{key}: Cargo builds the {kind} target {name} from {src_path}, which is not on the target allow-list")));
    }
    cases.push((
        "another library root".to_string(),
        {
            let mut sources = with_file_text("report_package/src/review_alternate.rs", "#![forbid(unsafe_code)]\n", invented());
            let library = sources.targets.get_mut("report_package").unwrap().iter_mut().find(|target| target.kind == "lib").unwrap();
            library.src_path = "report_package/src/review_alternate.rs".to_string();
            sources
        },
        "report_package: Cargo's library root is report_package/src/review_alternate.rs, not report_package/src/lib.rs".to_string(),
    ));
    cases.push((
        "an allow-listed test missing".to_string(),
        {
            let mut sources = invented();
            sources.targets.get_mut("pdf_emitter").unwrap().retain(|target| target.kind != "test");
            sources
        },
        "pdf_emitter: Cargo does not build the allow-listed test target emit from pdf_emitter/tests/emit.rs".to_string(),
    ));
    cases.push((
        "no targets reported".to_string(),
        {
            let mut sources = invented();
            sources.targets.remove("report_renderer");
            sources
        },
        "report_renderer: Cargo reports no target".to_string(),
    ));
    cases.push((
        "a build-dependency of a reporting crate".to_string(),
        {
            let mut sources = invented();
            sources.build_dependencies.get_mut("report_package").unwrap().insert("cc".to_string());
            sources
        },
        "report_package: the build-dependencies are".to_string(),
    ));
    for (case, sources, expected) in cases {
        let findings = scan(&sources);
        assert!(findings.iter().any(|finding| finding.contains(&expected)), "{case}: expected {expected:?} in {findings:#?}");
    }
    // The nested forbid on the real sources gives exactly that finding.
    let mut sources = project_sources(&project());
    sources.files.get_mut(LIB).unwrap().push_str("mod extra_fence { #![forbid(unsafe_code)] }\n");
    let findings = scan(&sources);
    assert_eq!(findings.len(), 1, "{findings:#?}");
    assert!(findings[0].starts_with(LIB) && findings[0].contains(not_listed), "{findings:#?}");
}

/// A scratch copy of one real crate (its manifest, lock file, `src/` and
/// `tests/`), removed on drop, so that Cargo itself reads an edited layout.
struct ScratchCrate {
    base: std::path::PathBuf,
    dir: std::path::PathBuf,
}

impl ScratchCrate {
    fn of(key: &str) -> ScratchCrate {
        fn copy(from: &Path, to: &Path) {
            std::fs::create_dir_all(to).unwrap();
            for entry in std::fs::read_dir(from).unwrap() {
                let path = entry.unwrap().path();
                let target = to.join(path.file_name().unwrap());
                if path.is_dir() {
                    copy(&path, &target);
                } else {
                    std::fs::copy(&path, &target).unwrap();
                }
            }
        }
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let number = NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let base = std::env::temp_dir().join(format!("swbpipe-reach-scan-{}-{number}", std::process::id()));
        let root = CRATES.iter().find(|(crate_key, _)| *crate_key == key).unwrap().1;
        let source = project().join(root);
        let dir = base.join(key);
        std::fs::create_dir_all(&dir).unwrap();
        for file in ["Cargo.toml", "Cargo.lock"] {
            if source.join(file).is_file() {
                std::fs::copy(source.join(file), dir.join(file)).unwrap();
            }
        }
        for directory in ["src", "tests"] {
            if source.join(directory).is_dir() {
                copy(&source.join(directory), &dir.join(directory));
            }
        }
        ScratchCrate { base, dir }
    }

    fn write(&self, path: &str, text: &str) {
        let path = self.dir.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }

    fn edit(&self, path: &str, old: &str, new: &str) {
        let text = std::fs::read_to_string(self.dir.join(path)).unwrap();
        assert_eq!(text.matches(old).count(), 1, "{path}: {old}");
        self.write(path, &text.replacen(old, new, 1));
    }

    /// The scan over the real sources with this crate's part read from the
    /// scratch copy, its targets as Cargo reports them there.
    fn scan(&self, key: &str) -> Vec<String> {
        let mut sources = project_sources(&project());
        sources.files.retain(|file, _| !file.starts_with(&format!("{key}/")));
        sources.bin_dirs.remove(key);
        crate_sources(key, &self.dir, &mut sources);
        scan(&sources)
    }
}

impl Drop for ScratchCrate {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.base);
    }
}

/// Code review r5 at #1682 (P2s), each on a scratch copy of a real crate, so
/// that Cargo decides the targets: an alternate library root (`[ lib ]`, or
/// a `path` line with `"src/lib.rs"` in a comment), an automatically
/// discovered `src/main.rs` in each reporting crate, `src/bin/x.rs`,
/// `build.rs`, `benches/x.rs`, a declared `[[bin]]` and `[[example]]`,
/// `autobins = true` with a binary, a proc-macro library and an unexpected
/// integration test are each findings; the unedited copies pass.
#[test]
fn cargo_targets_in_scratch_copies() {
    const LIB_SECTION: &str = "[lib]\npath = \"src/lib.rs\"\n";
    const FORGE: &str = "pub(crate) fn forge() -> user_release::DerivedUserRelease { vec![unsafe { std::mem::zeroed() }].pop().unwrap() }\n";
    const MAIN: &str = "fn main() { let _ = open_pipe_stress_report_renderer::render_calculation_report; let _ = unsafe { std::mem::zeroed::<u8>() }; }\n";
    let unsafe_keyword = "the unsafe keyword in a reporting crate";
    // Today's real targets pass on the copies.
    for key in ["report_package", "report_renderer", "pdf_emitter"] {
        let findings = ScratchCrate::of(key).scan(key);
        assert!(findings.is_empty(), "{key} unedited: {findings:#?}");
    }
    let alternate = |manifest_lib: &str| {
        let scratch = ScratchCrate::of("report_package");
        let lib = std::fs::read_to_string(scratch.dir.join("src/lib.rs")).unwrap();
        assert_eq!(lib.matches("#![forbid(unsafe_code)]\n").count(), 1);
        scratch.write("src/review_alternate.rs", &format!("{}{FORGE}", lib.replacen("#![forbid(unsafe_code)]\n", "", 1)));
        scratch.edit("Cargo.toml", LIB_SECTION, manifest_lib);
        scratch
    };
    let mut cases: Vec<(String, ScratchCrate, &str, Vec<String>)> = vec![
        (
            "[ lib ] with an alternate path".to_string(),
            alternate("[ lib ]\npath = \"src/review_alternate.rs\"\n"),
            "report_package",
            vec![
                "report_package: Cargo's library root is report_package/src/review_alternate.rs, not report_package/src/lib.rs".to_string(),
                "report_package/src/review_alternate.rs:0: the crate root does not carry #![forbid(unsafe_code)] exactly".to_string(),
                unsafe_keyword.to_string(),
            ],
        ),
        (
            "[lib] with \"src/lib.rs\" in a comment".to_string(),
            alternate("[lib]\npath = \"src/review_alternate.rs\" # \"src/lib.rs\"\n"),
            "report_package",
            vec![
                "report_package: Cargo's library root is report_package/src/review_alternate.rs, not report_package/src/lib.rs".to_string(),
                "report_package/src/review_alternate.rs:0: the crate root does not carry #![forbid(unsafe_code)] exactly".to_string(),
                unsafe_keyword.to_string(),
            ],
        ),
    ];
    let bin_finding = |key: &str, name: &str, src: &str| format!("{key}: Cargo builds the bin target {name} from {key}/{src}, which is not on the target allow-list");
    for (key, package) in [("report_package", "open_pipe_stress_report_package"), ("report_renderer", "open_pipe_stress_report_renderer"), ("pdf_emitter", "open_pipe_stress_pdf_emitter")] {
        let scratch = ScratchCrate::of(key);
        scratch.write("src/main.rs", MAIN);
        cases.push((format!("src/main.rs in {key}"), scratch, key, vec![bin_finding(key, package, "src/main.rs"), format!("{key}/src/main.rs:1: {unsafe_keyword}")]));
    }
    let scratch = ScratchCrate::of("pdf_emitter");
    scratch.write("src/bin/x.rs", MAIN);
    cases.push(("src/bin/x.rs".to_string(), scratch, "pdf_emitter", vec![bin_finding("pdf_emitter", "x", "src/bin/x.rs"), "pdf_emitter has a src/bin directory".to_string(), format!("pdf_emitter/src/bin/x.rs:1: {unsafe_keyword}")]));
    let scratch = ScratchCrate::of("report_package");
    scratch.write("build.rs", "fn main() {}\n");
    cases.push(("build.rs".to_string(), scratch, "report_package", vec!["report_package: Cargo builds the custom-build target build-script-build from report_package/build.rs, which is not on the target allow-list".to_string()]));
    let scratch = ScratchCrate::of("report_renderer");
    scratch.write("benches/x.rs", "fn main() {}\n");
    cases.push(("benches/x.rs".to_string(), scratch, "report_renderer", vec!["report_renderer: Cargo builds the bench target x from report_renderer/benches/x.rs, which is not on the target allow-list".to_string()]));
    let scratch = ScratchCrate::of("report_package");
    scratch.write("src/review_bin.rs", MAIN);
    scratch.edit("Cargo.toml", LIB_SECTION, &format!("{LIB_SECTION}\n[[bin]]\nname = \"review_bin\"\npath = \"src/review_bin.rs\"\n"));
    cases.push(("a [[bin]] entry".to_string(), scratch, "report_package", vec![bin_finding("report_package", "review_bin", "src/review_bin.rs"), format!("report_package/src/review_bin.rs:1: {unsafe_keyword}")]));
    let scratch = ScratchCrate::of("report_package");
    scratch.write("examples/review_example.rs", MAIN);
    scratch.edit("Cargo.toml", LIB_SECTION, &format!("{LIB_SECTION}\n[[example]]\nname = \"review_example\"\npath = \"examples/review_example.rs\"\n"));
    cases.push(("an [[example]] entry".to_string(), scratch, "report_package", vec!["report_package: Cargo builds the example target review_example from report_package/examples/review_example.rs, which is not on the target allow-list".to_string()]));
    let scratch = ScratchCrate::of("pdf_emitter");
    scratch.edit("Cargo.toml", "publish = false\n", "publish = false\nautobins = true\n");
    scratch.write("src/main.rs", MAIN);
    cases.push(("autobins = true with a bin".to_string(), scratch, "pdf_emitter", vec![bin_finding("pdf_emitter", "open_pipe_stress_pdf_emitter", "src/main.rs")]));
    let scratch = ScratchCrate::of("report_renderer");
    scratch.edit("Cargo.toml", LIB_SECTION, "[lib]\npath = \"src/lib.rs\"\nproc-macro = true\n");
    cases.push(("a proc-macro library".to_string(), scratch, "report_renderer", vec!["report_renderer: Cargo builds the proc-macro target open_pipe_stress_report_renderer from report_renderer/src/lib.rs".to_string(), "report_renderer: Cargo reports 0 lib targets, not exactly one".to_string()]));
    let scratch = ScratchCrate::of("report_renderer");
    scratch.write("tests/x.rs", "#[test]\nfn x() {}\n");
    cases.push(("an unexpected tests/x.rs".to_string(), scratch, "report_renderer", vec!["report_renderer: Cargo builds the test target x from report_renderer/tests/x.rs, which is not on the target allow-list".to_string()]));
    for (case, scratch, key, expected) in cases {
        let findings = scratch.scan(key);
        for expected in &expected {
            assert!(findings.iter().any(|finding| finding.contains(expected.as_str())), "{case}: expected {expected:?} in {findings:#?}");
        }
    }
}

/// Code review r6 at #1682 (P2, `cfg(test)`): a `#[cfg(test)]` item in the
/// reporting crates is read under the construction, literal and naming
/// rules. The reviewer's repro on the real sources, its `cfg_attr(test, …)`
/// variants and the other forms of the class are each findings; the one
/// allow-listed test item, and test-only modules, are read.
#[test]
fn the_real_sources_find_the_review_r6_test_only_forms() {
    const FORGE: &str = "#[cfg(test)]\npub fn forge() -> bool { ProducerEntry::UserRelease(DerivedUserRelease { _derived: () }).is_user_release() }\n";
    let base = project_sources(&project());
    let cases: [(&str, &str, &[&str]); 12] = [
        (
            "report_package/src/user_release.rs",
            FORGE,
            &[
                "ProducerEntry::UserRelease is constructed in the test-only item forge, which is not on the test allow-list",
                "a DerivedUserRelease literal in the test-only item forge, which is not on the test allow-list",
            ],
        ),
        // `cfg_attr(test, …)` variants: alongside `cfg(test)`, and an item
        // compiled only under `test` without `cfg(test)` (read as production).
        (
            "report_package/src/user_release.rs",
            "#[cfg_attr(test, allow(dead_code))]\n#[cfg(test)]\npub fn forge() -> bool { ProducerEntry::UserRelease(DerivedUserRelease { _derived: () }).is_user_release() }\n",
            &["ProducerEntry::UserRelease is constructed in the test-only item forge", "a DerivedUserRelease literal in the test-only item forge"],
        ),
        (
            "report_package/src/user_release.rs",
            "#[cfg_attr(not(test), cfg(any()))]\npub fn forge() -> bool { ProducerEntry::UserRelease(DerivedUserRelease { _derived: () }).is_user_release() }\n",
            &["ProducerEntry::UserRelease is constructed outside assemble_derived (Fn(\"forge\"))", "a DerivedUserRelease literal outside check_derived"],
        ),
        (
            "report_package/src/user_release.rs",
            "#[cfg(test)]\nmod forged {\n    #[cfg_attr(test, inline)]\n    pub fn forge() -> super::ProducerEntry { super::ProducerEntry::UserRelease(super::DerivedUserRelease { _derived: () }) }\n}\n",
            &["ProducerEntry::UserRelease is constructed in the test-only item forged::forge", "a struct literal of super::DerivedUserRelease in the defining module"],
        ),
        // A test-only redefinition of check_derived (its production one
        // under `cfg(not(test))`), a literal in it included.
        (
            "report_package/src/user_release.rs",
            "#[cfg(test)]\nfn check_derived() -> Result<DerivedUserRelease, String> { Ok(DerivedUserRelease { _derived: () }) }\n",
            &["check_derived is defined again in a test-only item", "a DerivedUserRelease literal in the test-only item check_derived"],
        ),
        // The literal through a test-only alias, a Self literal, an
        // associated type, a trait impl and a macro's tokens.
        ("report_package/src/user_release.rs", "#[cfg(test)]\ntype D = DerivedUserRelease;\n", &["the type alias D in the defining module of DerivedUserRelease"]),
        (
            "report_package/src/user_release.rs",
            "#[cfg(test)]\nimpl DerivedUserRelease { fn forge() -> Self { Self { _derived: () } } }\n",
            &["a DerivedUserRelease literal in the test-only item forge"],
        ),
        (
            "report_package/src/user_release.rs",
            "#[cfg(test)]\nimpl Default for DerivedUserRelease { fn default() -> Self { Self { _derived: () } } }\n",
            &["an impl of the trait Default for DerivedUserRelease", "a DerivedUserRelease literal in the test-only item default"],
        ),
        (
            "report_package/src/user_release.rs",
            "#[cfg(test)]\nfn forge() -> bool { assert!(ProducerEntry::UserRelease(DerivedUserRelease { _derived: () }).is_user_release()); true }\n",
            &["ProducerEntry::UserRelease is constructed in the test-only item Macro", "a DerivedUserRelease literal in the test-only item Macro"],
        ),
        // A test module elsewhere in the crate, and its outlined file.
        ("report_package/src/wire.rs", "#[cfg(test)]\nmod forged {\n    fn f(d: crate::user_release::DerivedUserRelease) { let _ = crate::user_release::ProducerEntry::UserRelease(d); }\n}\n", &["ProducerEntry::UserRelease is constructed in the test-only item forged::f"]),
        ("report_package/src/lib.rs", "#[cfg(test)]\nmod forged_tests;\n", &["ProducerEntry::UserRelease is constructed in the test-only item f"]),
        // A test-only include! is refused as in production.
        ("report_package/src/user_release.rs", "#[cfg(test)]\nfn forge() { include!(\"forge.rs\"); }\n", &["include! brings in source the scan does not read"]),
    ];
    for (file, text, expected) in cases {
        let mut sources = base.clone();
        sources.files.get_mut(file).unwrap().push_str(text);
        sources.files.insert("report_package/src/forged_tests.rs".to_string(), "fn f(d: crate::user_release::DerivedUserRelease) { let _ = crate::user_release::ProducerEntry::UserRelease(d); }\n".to_string());
        let findings = scan(&sources);
        for expected in expected {
            assert!(findings.iter().any(|finding| finding.contains(expected)), "{file}: expected {expected:?} in {findings:#?}");
        }
    }
    // The real sources: the allow-listed item writes exactly one of each,
    // and the test-only modules are read.
    assert_eq!(TEST_PROOF_ITEMS.len(), 1);
    let (findings, production, test_only) = scan_reading_all(&base);
    assert!(findings.is_empty(), "{findings:#?}");
    for file in ["report_package/src/admitted_tests.rs", "report_package/src/forwarding_graph_scan.rs", "report_package/src/exact_package_witness_tests.rs"] {
        assert!(test_only.contains(file) && !production.contains(file), "{file}: {test_only:?}");
    }
    // Removing the allow-listed item's construction, or adding a second one
    // to it, is a finding.
    let user_release = &base.files["report_package/src/user_release.rs"];
    let line = "        let entry = ProducerEntry::UserRelease(DerivedUserRelease { _derived: () });\n";
    assert_eq!(user_release.matches(line).count(), 1);
    for edit in ["        let entry = ProducerEntry::General;\n".to_string(), format!("{line}{}", line.replace("let entry", "let _again"))] {
        let mut sources = base.clone();
        sources.files.insert("report_package/src/user_release.rs".to_string(), user_release.replacen(line, &edit, 1));
        let findings = scan(&sources);
        assert!(findings.iter().any(|finding| finding.contains("not exactly one of each")), "{edit}: {findings:#?}");
    }
}

/// Code review r6 at #1682 (P2, `cfg(test)`), on a scratch copy that Cargo
/// reads: the reviewer's `#[cfg(test)]` constructor in `user_release.rs` is
/// a finding there too.
#[test]
fn the_review_r6_test_only_constructor_in_a_scratch_copy() {
    let scratch = ScratchCrate::of("report_package");
    let text = std::fs::read_to_string(scratch.dir.join("src/user_release.rs")).unwrap();
    scratch.write(
        "src/user_release.rs",
        &format!("{text}#[cfg(test)]\npub fn forge() -> bool {{ ProducerEntry::UserRelease(DerivedUserRelease {{ _derived: () }}).is_user_release() }}\n"),
    );
    let findings = scratch.scan("report_package");
    assert!(findings.iter().any(|finding| finding.contains("constructed in the test-only item forge")), "{findings:#?}");
    assert!(findings.iter().any(|finding| finding.contains("literal in the test-only item forge")), "{findings:#?}");
}

/// A scratch tree for the build-configuration fixtures, removed on drop.
struct ScratchTree {
    base: std::path::PathBuf,
}

impl ScratchTree {
    fn new() -> ScratchTree {
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let number = NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let base = std::env::temp_dir().join(format!("swbpipe-reach-config-{}-{number}", std::process::id()));
        std::fs::create_dir_all(&base).unwrap();
        ScratchTree { base: std::fs::canonicalize(base).unwrap() }
    }

    fn write(&self, path: &str, text: &str) {
        let path = self.base.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }

    /// A copy of a real crate's manifest (relative `path`s made absolute,
    /// to the real crates), its lock, and `files` (copied, or the given
    /// text), at `to`.
    fn crate_copy(&self, from: &str, to: &str, files: &[(&str, Option<&str>)]) -> std::path::PathBuf {
        let source = std::fs::canonicalize(project().join(from)).unwrap();
        let manifest = std::fs::read_to_string(source.join("Cargo.toml")).unwrap();
        let mut rewritten = String::new();
        for line in manifest.lines() {
            match line.split_once("path = \"../") {
                Some((head, rest)) => {
                    let (relative, tail) = rest.split_once('"').unwrap();
                    let absolute = std::fs::canonicalize(source.join(format!("../{relative}"))).unwrap();
                    rewritten.push_str(&format!("{head}path = \"{}\"{tail}\n", absolute.display()));
                }
                None => {
                    rewritten.push_str(line);
                    rewritten.push('\n');
                }
            }
        }
        self.write(&format!("{to}/Cargo.toml"), &rewritten);
        std::fs::copy(source.join("Cargo.lock"), self.base.join(to).join("Cargo.lock")).unwrap();
        for (file, text) in files {
            match text {
                Some(text) => self.write(&format!("{to}/{file}"), text),
                None => self.write(&format!("{to}/{file}"), &std::fs::read_to_string(source.join(file)).unwrap()),
            }
        }
        self.base.join(to)
    }

    /// The desktop shell's manifest and lock, with stand-in target files:
    /// a consumer Cargo resolves as it resolves `src-tauri`.
    fn desktop_consumer(&self, to: &str) -> std::path::PathBuf {
        self.crate_copy("apps/desktop/src-tauri", to, &[("src/lib.rs", Some("\n")), ("src/main.rs", Some("fn main() {}\n")), ("build.rs", Some("fn main() {}\n"))])
    }
}

impl Drop for ScratchTree {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.base);
    }
}

/// Code review r6 at #1682 (P2, resolved identities), on the real tree:
/// every consumer resolves each reporting package to the directory the scan
/// read, no Cargo configuration is in the repository, and no checked
/// manifest holds `[patch]` or `[replace]`.
#[test]
fn the_build_configuration_resolves_the_scanned_crates() {
    let project = project();
    let sources = project_sources(&project);
    let findings = build_configuration(&sources, &project);
    assert!(findings.is_empty(), "{findings:#?}");
    let expected = expected_manifests(&sources).unwrap();
    for (name, key) in REPORTING_PACKAGES {
        let root = CRATES.iter().find(|(crate_key, _)| *crate_key == key).unwrap().1;
        assert_eq!(expected[name], std::fs::canonicalize(project.join(root)).unwrap().join("Cargo.toml"), "{name}");
    }
    assert_eq!(CONSUMERS.map(|(key, _, _)| key), ["src-tauri", "headless"]);
    let repository = repository_root(&project).unwrap();
    assert!(cargo_configs(&repository).is_empty());
    let consumer = std::fs::canonicalize(project.join("apps/desktop/src-tauri")).unwrap();
    let allowed = allowed_dependents(&dependent_locations(&project, &expected)).unwrap();
    assert!(resolved_identities("src-tauri", &consumer, &expected, &[repository.clone()], &allowed).is_empty());
    // Outside the repository, the real crates are path packages elsewhere.
    let elsewhere = resolved_identities("src-tauri", &consumer, &expected, &[std::env::temp_dir()], &allowed);
    assert!(elsewhere.iter().any(|finding| finding.contains("is outside the repository")), "{elsewhere:#?}");
}

/// Code review r6 at #1682 (P2, resolved identities), on scratch trees that
/// Cargo reads: the reviewer's `paths` override of `report_package` with a
/// forging copy, a `[patch]` and a `[replace]` in the consumer's manifest,
/// and `.cargo/config(.toml)` files in the tree are each findings; an
/// unedited consumer copy passes.
#[test]
fn the_build_configuration_finds_the_review_r6_overrides() {
    let project = project();
    let sources = project_sources(&project);
    let expected = expected_manifests(&sources).unwrap();
    let repository = repository_root(&project).unwrap();

    // An unedited consumer copy resolves the real crates.
    let tree = ScratchTree::new();
    let consumer = tree.desktop_consumer("consumer");
    let allowed = copy_allowed(&project, &expected, &consumer);
    let findings = resolved_identities("copy", &consumer, &expected, &[repository.clone(), tree.base.clone()], &allowed);
    assert!(findings.is_empty(), "{findings:#?}");
    assert!(cargo_configs(&tree.base).is_empty());

    // The reviewer's repro: report_package copied outside the scanned tree,
    // its forbid dropped, a forge() that zeroes DerivedUserRelease with
    // unsafe, and a `paths` override in the consumer's .cargo/config.toml.
    let tree = ScratchTree::new();
    let consumer = tree.desktop_consumer("consumer");
    let replacement = tree.crate_copy("core/reporting/report_package", "substitute/report_package", &[]);
    let real = std::fs::canonicalize(project.join("core/reporting/report_package/src")).unwrap();
    copy_dir(&real, &replacement.join("src"));
    let lib = std::fs::read_to_string(replacement.join("src/lib.rs")).unwrap();
    std::fs::write(replacement.join("src/lib.rs"), lib.replacen("#![forbid(unsafe_code)]\n", "", 1)).unwrap();
    let user_release = std::fs::read_to_string(replacement.join("src/user_release.rs")).unwrap();
    std::fs::write(
        replacement.join("src/user_release.rs"),
        format!("{user_release}pub fn forge() -> bool {{ let derived: DerivedUserRelease = unsafe {{ std::mem::zeroed() }}; ProducerEntry::UserRelease(derived).is_user_release() }}\n"),
    )
    .unwrap();
    tree.write("consumer/.cargo/config.toml", &format!("paths = [\"{}\"]\n", replacement.display()));
    let findings = resolved_identities("copy", &consumer, &expected, &[repository.clone(), tree.base.clone()], &copy_allowed(&project, &expected, &consumer));
    let moved = format!("copy: open_pipe_stress_report_package resolves to {}, not {}", replacement.join("Cargo.toml").display(), expected["open_pipe_stress_report_package"].display());
    assert!(findings.iter().any(|finding| finding.starts_with(&moved)), "{moved}: {findings:#?}");
    let configs = cargo_configs(&tree.base);
    assert_eq!(configs, ["the Cargo configuration consumer/.cargo/config.toml is in the repository tree and not on the allow-list"]);
    // The scan of the in-repository sources alone finds nothing: the
    // identity check is what ties the build to them.
    assert!(scan(&sources).is_empty());

    // [patch] and [replace] in the consumer's manifest.
    for (section, word) in [
        (format!("\n[patch.crates-io]\nopen_pipe_stress_report_package = {{ path = \"{}\" }}\n", replacement.display()), "patch"),
        (format!("\n[replace]\n\"open_pipe_stress_report_package:0.1.0\" = {{ path = \"{}\" }}\n", replacement.display()), "replace"),
    ] {
        let tree = ScratchTree::new();
        let consumer = tree.desktop_consumer("consumer");
        let manifest = std::fs::read_to_string(consumer.join("Cargo.toml")).unwrap();
        let edited = format!("{manifest}{section}");
        std::fs::write(consumer.join("Cargo.toml"), &edited).unwrap();
        let findings = manifest_overrides("copy", &edited);
        assert_eq!(findings, [format!("copy: the manifest names {word}, which could be a [{word}] section")]);
    }
    for (text, count) in [
        ("[ patch . 'crates-io' ]\n", 1),
        ("[\"Patch\".crates-io]\n", 1),
        ("patch.crates-io.serde = { path = \"x\" }\n", 1),
        ("[\"\\u0070atch\".crates-io]\n", 1),
        ("[package]\nname = \"x\"\n", 0),
    ] {
        assert_eq!(manifest_overrides("text", text).len(), count, "{text}");
    }
    for (label, manifest) in CONSUMERS.iter().map(|(key, manifest, _)| (key.to_string(), project.join(manifest))).chain(expected.iter().map(|(name, manifest)| (name.clone(), manifest.clone()))) {
        assert!(manifest_overrides(&label, &std::fs::read_to_string(manifest).unwrap()).is_empty(), "{label}");
    }

    // .cargo/config.toml and .cargo/config anywhere in the tree, a crate's
    // ancestor directory included.
    let tree = ScratchTree::new();
    tree.write("projects/p/.cargo/config.toml", "[build]\nrustflags = [\"--cfg\", \"test\"]\n");
    tree.write(".cargo/config", "\n");
    tree.write("projects/p/core/crate/src/lib.rs", "\n");
    assert_eq!(
        cargo_configs(&tree.base),
        [
            "the Cargo configuration .cargo/config is in the repository tree and not on the allow-list",
            "the Cargo configuration projects/p/.cargo/config.toml is in the repository tree and not on the allow-list",
        ]
    );
}

/// The allowed dependents with the shell read from the copy at `consumer`.
fn copy_allowed(project: &Path, expected: &BTreeMap<String, std::path::PathBuf>, consumer: &Path) -> Dependents {
    let mut locations = dependent_locations(project, expected);
    locations.insert("openpipestress-desktop".to_string(), consumer.join("Cargo.toml"));
    allowed_dependents(&locations).unwrap()
}

fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let path = entry.unwrap().path();
        let target = to.join(path.file_name().unwrap());
        if path.is_dir() {
            copy_dir(&path, &target);
        } else {
            std::fs::copy(&path, &target).unwrap();
        }
    }
}

// --- Code review r7 at #1682 -------------------------------------------------

/// The reviewer's test-only renderer caller, with the production function
/// that would call it in a build that sets `cfg(test)`.
const R7_CALLER: &str = "#[cfg(test)]\nfn r7(input: &open_pipe_stress_report_renderer::RenderableReportInput) -> open_pipe_stress_report_renderer::RenderOutcome { open_pipe_stress_report_renderer::render_calculation_report(input) }\npub fn r7_entry(input: &open_pipe_stress_report_renderer::RenderableReportInput) -> open_pipe_stress_report_renderer::RenderOutcome { r7(input) }\n";
const R7_CALL: &str = "render_calculation_report called in the test-only item r7";
const PIN_DIFFERS: &str = "src-tauri/build.rs: the build script differs from its pin";
const R7_BUILD_LINE: &str = "    println!(\"cargo:rustc-cfg=test\");\n";
const TAURI_BUILD_CALL: &str = "    tauri_build::build()\n";

/// Code review r7 at #1682 (P2, INSIDE): `src-tauri`'s `#[cfg(test)]` items
/// are read under the caller-graph and renderer-call rules. The reviewer's
/// test-only caller and the other forms of the class are each findings on the
/// real sources; ordinary shell test code passes, and the real test modules,
/// those loaded by `#[path]` included, are read.
#[test]
fn the_real_sources_find_the_review_r7_shell_test_forms() {
    let base = project_sources(&project());
    let forge_call = "fn t(i: &open_pipe_stress_report_renderer::RenderableReportInput) { let _ = open_pipe_stress_report_renderer::render_calculation_report(i); }\n";
    let cases: Vec<(&str, String, Vec<(&str, &str)>, &str)> = vec![
        ("the reviewer's test-only caller", R7_CALLER.to_string(), vec![], R7_CALL),
        (
            "a test module calling through a crate alias",
            "#[cfg(test)]\nmod r7_tests {\n    use open_pipe_stress_report_renderer as r;\n    fn t(i: &r::RenderableReportInput) { let _ = r::render_calculation_report(i); }\n}\n".to_string(),
            vec![],
            "render_calculation_report called in the test-only item r7_tests::t",
        ),
        (
            "a glob import of the renderer",
            "#[cfg(test)]\nmod r7_tests {\n    use open_pipe_stress_report_renderer::*;\n    fn t(i: &RenderableReportInput) { let _ = render_calculation_report(i); }\n}\n".to_string(),
            vec![],
            "a glob import of open_pipe_stress_report_renderer in a src-tauri test-only item",
        ),
        (
            "an import of the pair",
            "#[cfg(test)]\nuse open_pipe_stress_report_renderer::render_calculation_report as render_pair;\n".to_string(),
            vec![],
            "binds the renderer or PDF crate or the renderer's pair",
        ),
        ("an import binding the crate", "#[cfg(test)]\nmod r7_tests {\n    use open_pipe_stress_report_renderer::{self as r};\n}\n".to_string(), vec![], "binds the renderer or PDF crate or the renderer's pair"),
        (
            "the pair as a value",
            "#[cfg(test)]\nfn r7() { let f = open_pipe_stress_report_renderer::render_calculation_report; let _ = f; }\n".to_string(),
            vec![],
            "render_calculation_report named in a path other than a direct call in the test-only item r7",
        ),
        (
            "the pair in a macro's tokens",
            "#[cfg(test)]\nfn r7(i: &I) { assert!(open_pipe_stress_report_renderer::render_calculation_report(i).html.is_empty()); }\n".to_string(),
            vec![],
            "render_calculation_report named in a macro's or an attribute's tokens in the test-only item",
        ),
        (
            "a test-only macro joining a renderer path",
            "#[cfg(test)]\nmod r7_tests {\n    macro_rules! call { ($f:ident) => { open_pipe_stress_report_renderer::$f(&i) }; }\n    fn t() { call!(render_calculation_report); }\n}\n".to_string(),
            vec![],
            "the macro definition call in a src-tauri test-only item is not on the test macro allow-list",
        ),
        (
            "a test-only redefinition of an allowed caller",
            "#[cfg(test)]\nmod r7_tests {\n    fn render_calculation_report_value(i: &I) { let _ = open_pipe_stress_report_renderer::render_calculation_report(i); }\n}\n".to_string(),
            vec![],
            "render_calculation_report called in the test-only item r7_tests::render_calculation_report_value",
        ),
        ("an outlined test module", "#[cfg(test)]\nmod r7_tests;\n".to_string(), vec![("src-tauri/src/r7_tests.rs", forge_call)], "src-tauri/src/r7_tests.rs:1: render_calculation_report called in the test-only item t"),
        (
            "a #[path] test module",
            "#[cfg(test)]\n#[path = \"r7_forge.rs\"]\nmod r7_tests;\n".to_string(),
            vec![("src-tauri/src/r7_forge.rs", forge_call)],
            "src-tauri/src/r7_forge.rs:1: render_calculation_report called in the test-only item t",
        ),
        (
            "a #[path] test module in another crate's tree, read as src-tauri's",
            "#[cfg(test)]\n#[path = \"../../../../core/reporting/report_package/tests/common/r7_forge.rs\"]\nmod r7_forge;\n".to_string(),
            vec![("report_package/tests/common/r7_forge.rs", forge_call)],
            "report_package/tests/common/r7_forge.rs:1: render_calculation_report called in the test-only item t",
        ),
        ("a #[path] outside the four crates", "#[cfg(test)]\n#[path = \"../../../../core/units/src/lib.rs\"]\nmod r7_units;\n".to_string(), vec![], "outside the scanned crates"),
        ("an absolute #[path]", "#[cfg(test)]\n#[path = \"/tmp/r7.rs\"]\nmod r7_tests;\n".to_string(), vec![], "is not a relative path"),
        (
            "a #[path] inside an inline test module",
            "#[cfg(test)]\nmod outer {\n    #[path = \"r7_forge.rs\"]\n    mod r7;\n}\n".to_string(),
            vec![("src-tauri/src/r7_forge.rs", forge_call)],
            "a #[path] on the src-tauri test module r7 that is not one string literal",
        ),
        (
            "an outlined module in a #[path] file",
            "#[cfg(test)]\n#[path = \"r7_forge.rs\"]\nmod r7_tests;\n".to_string(),
            vec![("src-tauri/src/r7_forge.rs", "mod inner;\n"), ("src-tauri/src/inner.rs", "\n")],
            "the outlined module inner in a file loaded by #[path]",
        ),
        ("include! in a test-only item", "#[cfg(test)]\nfn r7() { include!(\"r7.rs\"); }\n".to_string(), vec![], "include! brings in source the scan does not read"),
        ("include_str! of an unlisted .rs file", "#[cfg(test)]\nconst R7: &str = include_str!(\"lib.rs\");\n".to_string(), vec![], "include_str! takes one string literal that names no .rs file"),
    ];
    for (case, text, files, expected) in &cases {
        let mut sources = base.clone();
        sources.files.get_mut(TAURI).unwrap().push_str(text);
        for (file, file_text) in files {
            sources.files.insert(file.to_string(), file_text.to_string());
        }
        let findings = scan(&sources);
        assert!(findings.iter().any(|finding| finding.contains(expected)), "{case}: expected {expected:?} in {findings:#?}");
    }
    // A call of the bridge function and of the admitting entry, from test
    // items in other files.
    for (file, text, expected) in [
        (SAVE, "#[cfg(test)]\nfn r7(request: Value) { let _ = crate::report_package_bridge::assemble_user_released_request(request); }\n", "assemble_user_released_request called in the test-only item r7"),
        (BRIDGE, "#[cfg(test)]\nmod r7_tests {\n    fn t(request: Value) { let _ = open_pipe_stress_report_package::user_release::assemble_derived(request); }\n}\n", "assemble_derived called in the test-only item r7_tests::t"),
    ] {
        let mut sources = base.clone();
        sources.files.get_mut(file).unwrap().push_str(text);
        let findings = scan(&sources);
        assert!(findings.iter().any(|finding| finding.contains(expected)), "{file}: expected {expected:?} in {findings:#?}");
    }
    // A changed pinned test macro, and an unlisted one beside it.
    let ingress = "src-tauri/src/command_ingress.rs";
    for (old, new, expected) in [
        ("self.leaf(Node::Scalar(stringify!($method)))", "self.leaf(Node::Scalar(stringify!($method) ))", "the macro probe_scalars differs from its pinned text"),
        ("    macro_rules! probe_unsupported {", "    macro_rules! probe_more { () => {}; }\n    macro_rules! probe_unsupported {", "the macro definition probe_more in a src-tauri test-only item is not on the test macro allow-list"),
    ] {
        let mut sources = base.clone();
        let text = sources.files.get_mut(ingress).unwrap();
        assert_eq!(text.matches(old).count(), 1, "{old}");
        *text = text.replacen(old, new, 1);
        let findings = scan(&sources);
        assert!(findings.iter().any(|finding| finding.contains(expected)), "{expected:?} in {findings:#?}");
    }
    // Passing: ordinary shell test code (renderer items by name, the shell's
    // own render function and command).
    let mut sources = base.clone();
    sources.files.get_mut(TAURI).unwrap().push_str(
        "#[cfg(test)]\nmod r7_ordinary {\n    use open_pipe_stress_report_renderer::{decode_renderable_report_input, escape_html};\n    use super::*;\n    #[track_caller]\n    fn t(input: Value) { let _ = decode_renderable_report_input(input.clone()); let _ = escape_html(\"a\"); let _ = render_calculation_report_value(input); }\n}\n",
    );
    let findings = scan(&sources);
    assert!(findings.is_empty(), "{findings:#?}");
    // The real tree: no test caller is allow-listed, and the test modules,
    // those loaded by #[path] from another crate included, are read.
    assert!(SHELL_TEST_CALLERS.is_empty());
    let (findings, production, test_only) = scan_reading_all(&base);
    assert!(findings.is_empty(), "{findings:#?}");
    for file in [
        "src-tauri/src/user_release_reach_scan.rs",
        "src-tauri/src/user_release_reach_scan_tests.rs",
        "src-tauri/src/user_release_save_tests.rs",
        "src-tauri/src/report_render_exact_tests.rs",
        "src-tauri/src/object_form_tests.rs",
        "src-tauri/src/analysis_runs_tests.rs",
        "src-tauri/src/rule_solver_selection_tests.rs",
        "src-tauri/src/user_release_admission_tests.rs",
        "report_package/tests/common/package_basis.rs",
        "report_package/src/forwarding_graph_scan.rs",
    ] {
        assert!(test_only.contains(file) && !production.contains(file), "{file}: {test_only:?}");
    }
    // The #[path] resolution.
    assert_eq!(path_module_key(SAVE, "../../../../core/reporting/report_package/tests/common/package_basis.rs").unwrap(), "report_package/tests/common/package_basis.rs");
    assert_eq!(path_module_key("src-tauri/src/analysis_runs.rs", "analysis_runs_tests.rs").unwrap(), "src-tauri/src/analysis_runs_tests.rs");
    assert_eq!(path_module_key("src-tauri/src/analysis_runs.rs", "./x/../analysis_runs_tests.rs").unwrap(), "src-tauri/src/analysis_runs_tests.rs");
    for literal in ["/abs.rs", "..\\x.rs", "", "C:x.rs", "../../../../../../../../x.rs"] {
        assert!(path_module_key(SAVE, literal).is_err(), "{literal}");
    }
}

/// Code review r7 at #1682: `src-tauri`'s build script is pinned by content,
/// whatever `cargo:rustc-cfg` it would emit, and its normal and development
/// dependency sets are pinned; on the real sources.
#[test]
fn the_real_sources_find_the_review_r7_build_inputs() {
    let base = project_sources(&project());
    assert_eq!(BUILD_SCRIPTS.len(), 1);
    let real = real_build_script();
    assert_eq!(base.build_scripts["src-tauri/build.rs"], real);
    assert_eq!(real.matches(TAURI_BUILD_CALL).count(), 1);
    for line in [
        R7_BUILD_LINE,
        "    println!(\"cargo::rustc-cfg=test\");\n",
        "    println!(\"cargo:rustc-cfg=r7\");\n",
        "    println!(\"cargo:rustc-cfg=feature=\\\"native-smoke\\\"\");\n",
        "    println!(\"cargo:rustc-cfg=desktop\");\n",
        "    // a comment\n",
    ] {
        let mut sources = base.clone();
        sources.build_scripts.insert("src-tauri/build.rs".to_string(), real.replacen(TAURI_BUILD_CALL, &format!("{line}{TAURI_BUILD_CALL}"), 1));
        let findings = scan(&sources);
        assert!(findings.len() == 1 && findings[0].starts_with(PIN_DIFFERS), "{line}: {findings:#?}");
    }
    // An unread or unpinned build script.
    let mut sources = base.clone();
    sources.build_scripts.clear();
    assert!(scan(&sources).iter().any(|finding| finding.contains("the build script src-tauri/build.rs was not read")));
    let mut sources = base.clone();
    sources.targets.get_mut("report_renderer").unwrap().push(CrateTarget { kind: "custom-build".to_string(), name: "build-script-build".to_string(), src_path: "report_renderer/build.rs".to_string() });
    sources.build_scripts.insert("report_renderer/build.rs".to_string(), "fn main() {}\n".to_string());
    let findings = scan(&sources);
    for expected in ["report_renderer: Cargo builds the custom-build target build-script-build from report_renderer/build.rs, which is not on the target allow-list", "the build script report_renderer/build.rs is not on the build-script pin list"] {
        assert!(findings.iter().any(|finding| finding.contains(expected)), "{expected}: {findings:#?}");
    }
    // The dependency move, and other changes to the pinned sets.
    for (remove, add, expected) in [
        (Some("dev syn"), Some("normal syn"), vec!["Cargo reports the dependency normal syn", "Cargo does not report the pinned dependency dev syn"]),
        (Some("dev proc-macro2"), Some("normal proc-macro2"), vec!["Cargo reports the dependency normal proc-macro2", "Cargo does not report the pinned dependency dev proc-macro2"]),
        (None, Some("normal open_pipe_stress_pdf_emitter"), vec!["Cargo reports the dependency normal open_pipe_stress_pdf_emitter"]),
        (Some("normal serde"), Some("normal serde as serde_core"), vec!["Cargo reports the dependency normal serde as serde_core"]),
    ] {
        let mut sources = base.clone();
        let set = sources.dependencies.get_mut("src-tauri").unwrap();
        if let Some(remove) = remove {
            assert!(set.remove(remove), "{remove}");
        }
        if let Some(add) = add {
            set.insert(add.to_string());
        }
        let findings = scan(&sources);
        for expected in &expected {
            assert!(findings.iter().any(|finding| finding.contains(expected)), "{expected}: {findings:#?}");
        }
    }
}

/// A scratch copy of `src-tauri` that Cargo reads (its manifest with the path
/// dependencies made absolute, lock, build script and `src/`), with the given
/// edits, and the real sources with its part read from the copy.
fn shell_copy(tree: &ScratchTree, build_line: bool, move_parsers: bool, append: Option<&str>) -> Sources {
    let dir = tree.crate_copy("apps/desktop/src-tauri", "src-tauri", &[("build.rs", None)]);
    copy_dir(&project().join("apps/desktop/src-tauri/src"), &dir.join("src"));
    if build_line {
        let text = std::fs::read_to_string(dir.join("build.rs")).unwrap();
        assert_eq!(text.matches(TAURI_BUILD_CALL).count(), 1);
        std::fs::write(dir.join("build.rs"), text.replacen(TAURI_BUILD_CALL, &format!("{R7_BUILD_LINE}{TAURI_BUILD_CALL}"), 1)).unwrap();
    }
    if move_parsers {
        let manifest = std::fs::read_to_string(dir.join("Cargo.toml")).unwrap();
        let parsers: Vec<&str> = manifest.lines().filter(|line| line.starts_with("syn = ") || line.starts_with("proc-macro2 = ")).collect();
        assert_eq!(parsers.len(), 2);
        let mut moved: String = manifest.lines().filter(|line| !parsers.contains(line)).map(|line| format!("{line}\n")).collect();
        assert_eq!(moved.matches("[dependencies]\n").count(), 1);
        moved = moved.replacen("[dependencies]\n", &format!("[dependencies]\n{}\n", parsers.join("\n")), 1);
        std::fs::write(dir.join("Cargo.toml"), moved).unwrap();
    }
    if let Some(text) = append {
        let mut lib = std::fs::read_to_string(dir.join("src/lib.rs")).unwrap();
        lib.push_str(text);
        std::fs::write(dir.join("src/lib.rs"), lib).unwrap();
    }
    let mut sources = project_sources(&project());
    sources.files.retain(|file, _| !file.starts_with("src-tauri/"));
    sources.build_scripts.retain(|file, _| !file.starts_with("src-tauri/"));
    sources.bin_dirs.remove("src-tauri");
    crate_sources("src-tauri", &dir, &mut sources);
    sources
}

/// Code review r7 at #1682 (P2, INSIDE), on scratch copies of `src-tauri`
/// that Cargo reads: the reviewer's full repro (a build script emitting
/// `cargo:rustc-cfg=test`, `syn` and `proc-macro2` moved to normal
/// dependencies, and a test-only renderer caller with a production entry)
/// gives each finding, and each part alone gives its own; the unedited copy
/// passes.
#[test]
fn the_review_r7_repro_in_scratch_copies() {
    let dependency = "is not on the dependency pin list";
    let missing = "Cargo does not report the pinned dependency";
    let cases: [(&str, bool, bool, Option<&str>, &[&str]); 5] = [
        ("unedited", false, false, None, &[]),
        ("the full repro", true, true, Some(R7_CALLER), &[PIN_DIFFERS, dependency, missing, R7_CALL]),
        ("the build script alone", true, false, None, &[PIN_DIFFERS]),
        ("the dependency move alone", false, true, None, &[dependency, missing]),
        ("the test-only caller alone", false, false, Some(R7_CALLER), &[R7_CALL]),
    ];
    for (case, build_line, move_parsers, append, expected) in cases {
        let tree = ScratchTree::new();
        let sources = shell_copy(&tree, build_line, move_parsers, append);
        let findings = scan(&sources);
        for expected in expected {
            assert!(findings.iter().any(|finding| finding.contains(expected)), "{case}: expected {expected:?} in {findings:#?}");
        }
        // Each part gives only its own findings.
        for finding in &findings {
            assert!(expected.iter().any(|expected| finding.contains(expected)), "{case}: unexpected {finding:?}");
        }
        if move_parsers {
            for name in ["syn", "proc-macro2"] {
                assert!(findings.iter().any(|finding| finding.contains(&format!("the dependency normal {name},"))), "{case}: {name}: {findings:#?}");
                assert!(findings.iter().any(|finding| finding.contains(&format!("pinned dependency dev {name}"))), "{case}: {name}: {findings:#?}");
            }
        }
    }
}

/// Code review r7 at #1682: the consumers' in-repository build scripts are
/// `src-tauri`'s, pinned, and `product_physics`'s, whose package depends on
/// no reporting crate; a build script in a package that depends on one is a
/// finding, on scratch packages Cargo reads.
#[test]
fn the_in_repository_build_scripts_are_known() {
    let project = project();
    let sources = project_sources(&project);
    let expected = expected_manifests(&sources).unwrap();
    let consumers: Vec<std::path::PathBuf> = CONSUMERS.iter().map(|(_, manifest, _)| project.join(manifest)).collect();
    let scripts = in_repo_build_scripts(&consumers, &expected).unwrap();
    let found: BTreeSet<(String, bool)> = scripts.iter().map(|script| (script.package.clone(), script.reaches_reporting)).collect();
    assert_eq!(found, BTreeSet::from([("openpipestress-desktop".to_string(), true), ("open_pipe_stress_product_physics".to_string(), false)]), "{scripts:#?}");
    let shell = std::fs::canonicalize(project.join(CONSUMERS[0].1)).unwrap();
    assert!(build_script_findings(&scripts, &shell).is_empty());
    // A consumer whose path dependency runs a build script and depends on
    // report_package, beside one whose build script reaches nothing.
    let tree = ScratchTree::new();
    let package = std::fs::canonicalize(project.join("core/reporting/report_package")).unwrap();
    tree.write("consumer/Cargo.toml", "[package]\nname = \"r7_consumer\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nr7_helper = { path = \"../helper\" }\nr7_leaf = { path = \"../leaf\" }\n");
    tree.write("consumer/src/lib.rs", "\n");
    tree.write("helper/Cargo.toml", &format!("[package]\nname = \"r7_helper\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nopen_pipe_stress_report_package = {{ path = \"{}\" }}\n", package.display()));
    tree.write("helper/build.rs", &format!("fn main() {{\n{R7_BUILD_LINE}}}\n"));
    tree.write("helper/src/lib.rs", "\n");
    tree.write("leaf/Cargo.toml", "[package]\nname = \"r7_leaf\"\nversion = \"0.1.0\"\nedition = \"2021\"\n");
    tree.write("leaf/build.rs", &format!("fn main() {{\n{R7_BUILD_LINE}}}\n"));
    tree.write("leaf/src/lib.rs", "\n");
    let scripts = in_repo_build_scripts(&[tree.base.join("consumer/Cargo.toml")], &expected).unwrap();
    let found: BTreeSet<(String, bool)> = scripts.iter().map(|script| (script.package.clone(), script.reaches_reporting)).collect();
    assert_eq!(found, BTreeSet::from([("r7_helper".to_string(), true), ("r7_leaf".to_string(), false)]), "{scripts:#?}");
    let findings = build_script_findings(&scripts, &shell);
    assert_eq!(findings.len(), 1, "{findings:#?}");
    assert!(findings[0].starts_with("the in-repository package r7_helper runs the build script") && findings[0].contains("depends on a reporting crate"), "{findings:#?}");
}

// --- Code review r8 at #1682 -------------------------------------------------

/// The reviewer's round-8 library addition to the headless runner: a
/// renderer call through a crate alias.
const R8_CALLER: &str = "use open_pipe_stress_report_renderer as r;\npub fn r8(i: &r::RenderableReportInput) -> r::RenderOutcome { r::render_calculation_report(i) }\n";

/// A scratch project that Cargo reads, laid out like the real one: copies of
/// the shell (stand-in sources) and the headless runner (its sources), their
/// path dependencies made absolute to the real crates, so that the allowed
/// dependents at the copies' locations are today's set. `edit` changes the
/// runner copy's manifest text. Returns the project and the runner copy's
/// canonical manifest.
fn dependent_project(tree: &ScratchTree, edit: &dyn Fn(&str) -> String, caller: bool) -> (std::path::PathBuf, std::path::PathBuf) {
    tree.desktop_consumer("project/apps/desktop/src-tauri");
    // The runner's directory from its manifest's path, so that no literal
    // here names the runner's sources (hosted CI routes desktop Rust by the
    // project paths its literals name).
    let runner_dir = Path::new(CONSUMERS[1].1).parent().unwrap().to_str().unwrap();
    let runner = tree.crate_copy(runner_dir, &format!("project/{runner_dir}"), &[]);
    copy_dir(&project().join(runner_dir).join("src"), &runner.join("src"));
    let manifest = std::fs::read_to_string(runner.join("Cargo.toml")).unwrap();
    std::fs::write(runner.join("Cargo.toml"), edit(&manifest)).unwrap();
    if caller {
        let lib = std::fs::read_to_string(runner.join("src/lib.rs")).unwrap();
        std::fs::write(runner.join("src/lib.rs"), format!("{lib}{R8_CALLER}")).unwrap();
    }
    (tree.base.join("project"), runner.join("Cargo.toml"))
}

/// The dependent findings for a scratch project: its manifests and their
/// reachable path dependencies read by Cargo, against the allowed set at its
/// own locations.
fn dependents_in(project: &Path, expected: &BTreeMap<String, std::path::PathBuf>) -> (Dependents, Vec<String>) {
    let (manifests, unreadable) = project_manifests(project);
    assert!(unreadable.is_empty(), "{unreadable:#?}");
    let read = in_repo_packages(&manifests, expected);
    let allowed = allowed_dependents(&dependent_locations(project, expected)).unwrap();
    (read.dependents.clone(), [read.findings, dependent_findings(&read.dependents, &allowed)].concat())
}

/// Code review r8 at #1682 (P2, INSIDE), on scratch copies Cargo reads: each
/// spelling of a dependency on the renderer added to the headless runner
/// (the reviewer's quoted key in `[dependencies]` with the library call, a
/// quoted key under `[dev-dependencies]` and `[build-dependencies]`, a
/// `[target.'cfg(unix)'.dependencies]` entry, dotted keys, a rename, and a
/// path to the renderer under another package name) is exactly one finding;
/// a root-level dotted key in a new package is one for that package; the
/// unedited copies, today's dependents, pass.
#[test]
fn every_spelling_of_a_new_dependent_is_a_finding() {
    let expected = expected_manifests(&project_sources(&project())).unwrap();
    let renderer = expected["open_pipe_stress_report_renderer"].parent().unwrap().display().to_string();
    let entry = |key: &str| format!("{key} = {{ path = \"{renderer}\" }}\n");
    let under_dependencies = |line: String| move |manifest: &str| {
        assert_eq!(manifest.matches("\n[dependencies]\n").count(), 1);
        manifest.replacen("\n[dependencies]\n", &format!("\n[dependencies]\n{line}"), 1)
    };
    let appended = |text: String| move |manifest: &str| format!("{manifest}{text}");

    // Today's dependents pass, read from the copies.
    let tree = ScratchTree::new();
    let (project, _) = dependent_project(&tree, &|manifest: &str| manifest.to_string(), false);
    let (found, findings) = dependents_in(&project, &expected);
    assert!(findings.is_empty(), "{findings:#?}");
    assert_eq!(found["open_pipe_stress_report_package"].iter().map(|(name, _)| name.as_str()).collect::<Vec<_>>(), ["open_pipe_stress_headless_runner", "openpipestress-desktop"]);
    assert_eq!(found["open_pipe_stress_report_renderer"].len(), 3);
    assert_eq!(found["open_pipe_stress_pdf_emitter"].len(), 1);

    let cases: Vec<(&str, Box<dyn Fn(&str) -> String>, bool)> = vec![
        ("the reviewer's quoted key", Box::new(under_dependencies(entry("\"open_pipe_stress_report_renderer\""))), true),
        ("a quoted key under [dev-dependencies]", Box::new(appended(format!("\n[dev-dependencies]\n{}", entry("\"open_pipe_stress_report_renderer\"")))), false),
        ("a quoted key under [build-dependencies]", Box::new(appended(format!("\n[build-dependencies]\n{}", entry("\"open_pipe_stress_report_renderer\"")))), false),
        ("a [target.'cfg(unix)'.dependencies] entry", Box::new(appended(format!("\n[target.'cfg(unix)'.dependencies]\n{}", entry("open_pipe_stress_report_renderer")))), false),
        ("a dotted key in [dependencies]", Box::new(under_dependencies(format!("open_pipe_stress_report_renderer.path = \"{renderer}\"\n"))), false),
        ("a dotted key under a target table", Box::new(appended(format!("\n[target.'cfg(unix)']\ndependencies.open_pipe_stress_report_renderer.path = \"{renderer}\"\n"))), false),
        ("a renamed dependency", Box::new(under_dependencies(format!("r = {{ package = \"open_pipe_stress_report_renderer\", path = \"{renderer}\" }}\n"))), true),
        ("a quoted, single-quoted rename", Box::new(under_dependencies(format!("'r' = {{ \"package\" = \"open_pipe_stress_report_renderer\", path = '{renderer}' }}\n"))), false),
    ];
    for (case, edit, caller) in cases {
        let tree = ScratchTree::new();
        let (project, runner) = dependent_project(&tree, &*edit, caller);
        let (_, findings) = dependents_in(&project, &expected);
        assert_eq!(
            findings,
            [format!("the package open_pipe_stress_headless_runner ({}) depends on open_pipe_stress_report_renderer and is not on its allowed dependent list", std::fs::canonicalize(&runner).unwrap().display())],
            "{case}"
        );
    }

    // A path to the renderer under another package name: Cargo does not
    // resolve it, but the path identifies it.
    let tree = ScratchTree::new();
    let (project, runner) = dependent_project(&tree, &under_dependencies(format!("r = {{ package = \"r8_not_the_renderer\", path = \"{renderer}\" }}\n")), false);
    let (_, findings) = dependents_in(&project, &expected);
    assert!(
        findings.iter().any(|finding| finding.starts_with(&format!("the package open_pipe_stress_headless_runner ({}) depends on open_pipe_stress_report_renderer", std::fs::canonicalize(&runner).unwrap().display()))),
        "{findings:#?}"
    );

    // The literal root-level dotted form, in a new package of the project.
    let tree = ScratchTree::new();
    let (project, _) = dependent_project(&tree, &|manifest: &str| manifest.to_string(), false);
    tree.write("project/core/r8_dotted/Cargo.toml", &format!("dependencies.open_pipe_stress_report_renderer.path = \"{renderer}\"\n\n[package]\nname = \"r8_dotted\"\nversion = \"0.1.0\"\nedition = \"2021\"\n"));
    tree.write("project/core/r8_dotted/src/lib.rs", R8_CALLER);
    let (_, findings) = dependents_in(&project, &expected);
    let dotted = std::fs::canonicalize(project.join("core/r8_dotted/Cargo.toml")).unwrap();
    assert_eq!(findings, [format!("the package r8_dotted ({}) depends on open_pipe_stress_report_renderer and is not on its allowed dependent list", dotted.display())]);

    // A manifest Cargo cannot read is a finding, not a skip.
    let tree = ScratchTree::new();
    let (project, _) = dependent_project(&tree, &|manifest: &str| manifest.to_string(), false);
    tree.write("project/core/r8_broken/Cargo.toml", "[package]\nname = \"r8_broken\"\n[dependencies\n");
    let (_, findings) = dependents_in(&project, &expected);
    assert_eq!(findings.len(), 1, "{findings:#?}");
    assert!(findings[0].contains("r8_broken/Cargo.toml: cargo metadata --offline --locked --no-deps failed, so its dependencies are unknown"), "{findings:#?}");
}

/// Code review r8 at #1682: a package outside the project that a project
/// package reaches by a path dependency, and that depends on the renderer by
/// a quoted key, is a finding in the walk and in the shell's resolved graph;
/// the copy's lock is updated offline first, as a build would need.
#[test]
fn a_reachable_dependent_outside_the_project_is_a_finding() {
    let real = project();
    let expected = expected_manifests(&project_sources(&real)).unwrap();
    let renderer = expected["open_pipe_stress_report_renderer"].parent().unwrap().display().to_string();
    let tree = ScratchTree::new();
    let (project, _) = dependent_project(&tree, &|manifest: &str| manifest.to_string(), false);
    tree.write("outside/helper/Cargo.toml", &format!("[package]\nname = \"r8_helper\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n\"open_pipe_stress_report_renderer\" = {{ path = \"{renderer}\" }}\n"));
    tree.write("outside/helper/src/lib.rs", R8_CALLER);
    let shell = project.join("apps/desktop/src-tauri");
    let manifest = std::fs::read_to_string(shell.join("Cargo.toml")).unwrap();
    assert_eq!(manifest.matches("\n[dependencies]\n").count(), 1);
    let helper = tree.base.join("outside/helper");
    std::fs::write(shell.join("Cargo.toml"), manifest.replacen("\n[dependencies]\n", &format!("\n[dependencies]\n\"r8_helper\" = {{ path = \"{}\" }}\n", helper.display()), 1)).unwrap();
    let helper_manifest = std::fs::canonicalize(helper.join("Cargo.toml")).unwrap();
    let expected_finding = format!("the package r8_helper ({}) depends on open_pipe_stress_report_renderer and is not on its allowed dependent list", helper_manifest.display());
    let (_, findings) = dependents_in(&project, &expected);
    assert_eq!(findings, [expected_finding.clone()]);
    // The resolved graph, after an offline lock update.
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| env!("CARGO").into());
    let status = std::process::Command::new(cargo).args(["metadata", "--format-version", "1", "--offline", "--manifest-path"]).arg(shell.join("Cargo.toml")).current_dir(&shell).stdout(std::process::Stdio::null()).status().unwrap();
    assert!(status.success());
    let allowed = allowed_dependents(&dependent_locations(&project, &expected)).unwrap();
    let repository = repository_root(&real).unwrap();
    let findings = resolved_identities("copy", &std::fs::canonicalize(&shell).unwrap(), &expected, &[repository, tree.base.clone()], &allowed);
    assert_eq!(findings, [format!("copy: in the resolved graph, {expected_finding}")]);
}

// --- Code review r9 at #1682 -------------------------------------------------

/// The reviewer's round-9 caller: the producer's admitting entry called from
/// a module the headless runner loads.
const R9_CALLER: &str = "pub fn r9(request: &serde_json::Value, model: &serde_json::Value, inputs: &open_pipe_stress_report_package::user_release::UserReleaseInputs<'_>) { let _ = open_pipe_stress_report_package::user_release::assemble_derived(request, model, inputs); }\n";
const R9_CALL: &str = "assemble_derived is called from Fn(\"r9\"), which is not an allowed caller";
/// The reviewer's round-9 proc-macro, whose expansion names the admitting
/// entry the scan never reads.
const R9_MACRO: &str = "use proc_macro::TokenStream;\n#[proc_macro]\npub fn entry(_: TokenStream) -> TokenStream { \"open_pipe_stress_report_package::user_release::assemble_derived\".parse().unwrap() }\n";
const FORBID: &str = "#![forbid(unsafe_code)]\n";

/// The headless runner's directory in the project, from [`CRATES`].
fn runner_dir() -> &'static str {
    CRATES.iter().find(|(key, _)| *key == "headless").unwrap().1
}

/// A scratch copy of the headless runner that Cargo reads (its manifest with
/// the path dependencies made absolute, its lock, `src/` and `tests/`), at
/// `headless` in `tree`, with `manifest` applied to its manifest text, `lib`
/// to its `src/lib.rs`, and `files` written; and the real sources with the
/// runner's part read from the copy. Returns the sources and the copy's
/// manifest.
fn runner_copy(tree: &ScratchTree, manifest: &dyn Fn(&str) -> String, lib: &dyn Fn(&str) -> String, files: &[(&str, &str)]) -> (Sources, std::path::PathBuf) {
    let dir = tree.crate_copy(runner_dir(), "headless", &[]);
    copy_dir(&project().join(runner_dir()).join("src"), &dir.join("src"));
    copy_dir(&project().join(runner_dir()).join("tests"), &dir.join("tests"));
    let text = std::fs::read_to_string(dir.join("Cargo.toml")).unwrap();
    std::fs::write(dir.join("Cargo.toml"), manifest(&text)).unwrap();
    let text = std::fs::read_to_string(dir.join("src/lib.rs")).unwrap();
    std::fs::write(dir.join("src/lib.rs"), lib(&text)).unwrap();
    for (path, text) in files {
        tree.write(&format!("headless/{path}"), text);
    }
    let mut sources = project_sources(&project());
    sources.files.retain(|file, _| !file.starts_with("headless/"));
    sources.build_scripts.retain(|file, _| !file.starts_with("headless/"));
    sources.bin_dirs.remove("headless");
    crate_sources("headless", &dir, &mut sources);
    (sources, std::fs::canonicalize(dir.join("Cargo.toml")).unwrap())
}

/// A proc-macro package `name` in `tree` (its `entry!` expands to the
/// admitting entry's path), with `dependencies` as its manifest's
/// `[dependencies]` lines; returns its directory.
fn proc_macro_package(tree: &ScratchTree, name: &str) -> std::path::PathBuf {
    tree.write(&format!("{name}/Cargo.toml"), &format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[lib]\nproc-macro = true\n"));
    tree.write(&format!("{name}/src/lib.rs"), R9_MACRO);
    tree.base.join(name)
}

/// `manifest` with `line` added under its `[dependencies]`.
fn with_dependency(manifest: &str, line: &str) -> String {
    assert_eq!(manifest.matches("\n[dependencies]\n").count(), 1);
    manifest.replacen("\n[dependencies]\n", &format!("\n[dependencies]\n{line}\n"), 1)
}

/// Also asserted (code review r9 at #1682; §2.8, "Allowed dependents"): the
/// scan reads the headless runner's library and binary roots and every
/// production module they load, and reads its test-only modules as test
/// code; each root carries `#![forbid(unsafe_code)]`; the real tree's
/// path-dependency closure, from the allowed dependents, holds every
/// reporting crate and the runner and no proc-macro.
#[test]
fn the_scan_reads_the_headless_runner() {
    let project = project();
    let (findings, production, test_only) = scan_reading_all(&project_sources(&project));
    assert!(findings.is_empty(), "{findings:#?}");
    for file in [
        "headless/src/lib.rs",
        "headless/src/bin/openpipestress-runner.rs",
        "headless/src/bin/headless_preview_runner.rs",
        "headless/src/benchmark_binding.rs",
        "headless/src/reader_binding.rs",
        "headless/src/redaction_binding.rs",
        "headless/src/result_envelope_binding.rs",
    ] {
        assert!(production.contains(file), "{file}: {production:?}");
    }
    assert!(test_only.contains("headless/src/load_reference_route_tests.rs"), "{test_only:?}");
    assert!(!production.contains("headless/src/load_reference_route_tests.rs"));
    // Integration tests are separate test crates no product build compiles:
    // pinned by name and file ([`TARGETS`]), not read (as the other crates'),
    // and covered by the runner's own name check.
    assert!(!production.contains("headless/tests/user_release_runner_refused.rs"));
    let sources = project_sources(&project);
    for root in ["headless/src/lib.rs", "headless/src/bin/openpipestress-runner.rs", "headless/src/bin/headless_preview_runner.rs"] {
        assert_eq!(sources.files[root].matches(FORBID).count(), 1, "{root}");
    }
    let consumers: Vec<std::path::PathBuf> = CONSUMERS.iter().map(|(_, manifest, _)| project.join(manifest)).collect();
    let packages = path_packages(&consumers).unwrap();
    let names: BTreeSet<&str> = packages.values().map(|package| package.name.as_str()).collect();
    for name in ["openpipestress-desktop", "open_pipe_stress_headless_runner", "open_pipe_stress_report_package", "open_pipe_stress_report_renderer", "open_pipe_stress_pdf_emitter", "open_pipe_stress_mechanics_benchmarks"] {
        assert!(names.contains(name), "{name}: {names:?}");
    }
    assert!(packages.values().all(|package| package.proc_macros.is_empty()), "{packages:#?}");
    assert!(proc_macro_findings(&packages).is_empty());
    assert!(PROC_MACRO_PACKAGES.is_empty());
}

/// Code review r9 at #1682 (P2s, INSIDE), on scratch copies of the headless
/// runner that Cargo reads: the reviewer's `#[path]` module calling the
/// admitting entry, a `#[path]` to a file inside `src/`, a test module's
/// `#[path]` out of the crate's directory, a `mod` declared from `build.rs`
/// and from a new `tests/` file, a new normal, development and build
/// dependency, the reviewer's local proc-macro, a renamed one and one reached
/// through another path dependency, and the other rules the runner is now
/// held to, each give their own findings and only those; the unedited copy
/// and a test module's `#[path]` inside the crate pass.
#[test]
fn the_review_r9_repros_in_scratch_copies() {
    let same = |text: &str| text.to_string();
    let append = |extra: &'static str| move |text: &str| format!("{text}{extra}");
    // A real crate the runner does not depend on, by the directory the scan
    // read (no path literal here: hosted CI routes desktop Rust by the
    // project paths this crate's literals name).
    let pdf_dir = || project_sources(&project()).crate_dirs["pdf_emitter"].display().to_string();
    let pdf = pdf_dir();
    let not_pinned = |dependency: &str| format!("headless: Cargo reports the dependency {dependency}, which is not on the dependency pin list");
    let invocation = |path: &str| format!("the macro {path}! is not on the invocation list");
    type Case<'a> = (&'a str, Box<dyn Fn(&str) -> String>, Box<dyn Fn(&str) -> String>, Vec<(&'a str, String)>, Vec<String>);
    let mut cases: Vec<Case> = vec![
        ("unedited", Box::new(same), Box::new(same), vec![], vec![]),
        (
            "the reviewer's #[path] module",
            Box::new(same),
            Box::new(append("#[path = \"../r9.rs\"]\npub mod r9_review;\n")),
            vec![("r9.rs", R9_CALLER.to_string())],
            vec!["a #[path] on the production module r9_review".to_string(), format!("headless/r9.rs:1: {R9_CALL}")],
        ),
        (
            "a #[path] to a file inside src/",
            Box::new(same),
            Box::new(append("#[path = \"r9_inside.rs\"]\nmod r9_inside;\n")),
            vec![("src/r9_inside.rs", R9_CALLER.to_string())],
            vec!["a #[path] on the production module r9_inside".to_string(), format!("headless/src/r9_inside.rs:1: {R9_CALL}")],
        ),
        (
            "a test module's #[path] out of the crate's directory",
            Box::new(same),
            Box::new(append("#[cfg(test)]\n#[path = \"../../r9_outside.rs\"]\nmod r9_outside;\n")),
            vec![],
            vec!["the #[path] \"../../r9_outside.rs\" loads core/runner/r9_outside.rs, outside the scanned crates".to_string()],
        ),
        (
            "a test module's #[path] into another scanned crate",
            Box::new(same),
            Box::new(append("#[cfg(test)]\n#[path = \"../../../reporting/report_package/src/r9_forge.rs\"]\nmod r9_forge;\n")),
            vec![],
            vec!["loads report_package/src/r9_forge.rs, outside its crate's directory, and is not on the path-crossing allow-list".to_string(), "report_package/src/r9_forge.rs: missing".to_string()],
        ),
        ("a test module's #[path] inside the crate", Box::new(same), Box::new(append("#[cfg(test)]\n#[path = \"r9_tests.rs\"]\nmod r9_tests;\n")), vec![("src/r9_tests.rs", "#[test]\nfn r9() {}\n".to_string())], vec![]),
        (
            "a test module calling the entry",
            Box::new(same),
            Box::new(append("#[cfg(test)]\n#[path = \"r9_tests.rs\"]\nmod r9_tests;\n")),
            vec![("src/r9_tests.rs", R9_CALLER.to_string())],
            vec!["headless/src/r9_tests.rs:1: assemble_derived called in the test-only item r9".to_string()],
        ),
        (
            "a mod declared from build.rs",
            Box::new(same),
            Box::new(same),
            vec![("build.rs", "mod r9_build;\nfn main() {}\n".to_string()), ("r9_build.rs", R9_CALLER.to_string())],
            vec![
                "headless: Cargo builds the custom-build target build-script-build from headless/build.rs, which is not on the target allow-list".to_string(),
                "headless: the build script headless/build.rs is not on the build-script pin list".to_string(),
            ],
        ),
        (
            "a mod declared from a new tests/ file",
            Box::new(same),
            Box::new(same),
            vec![("tests/r9.rs", "mod r9_inner;\n".to_string()), ("tests/r9_inner/mod.rs", R9_CALLER.to_string())],
            vec!["headless: Cargo builds the test target r9 from headless/tests/r9.rs, which is not on the target allow-list".to_string()],
        ),
        (
            "a new normal dependency",
            Box::new(move |text: &str| with_dependency(text, &format!("open_pipe_stress_pdf_emitter = {{ path = \"{pdf}\" }}"))),
            Box::new(same),
            vec![],
            vec![not_pinned("normal open_pipe_stress_pdf_emitter")],
        ),
        (
            "a new development dependency",
            Box::new({
                let pdf = pdf_dir();
                move |text: &str| format!("{text}\n[dev-dependencies]\nopen_pipe_stress_pdf_emitter = {{ path = \"{pdf}\" }}\n")
            }),
            Box::new(same),
            vec![],
            vec![not_pinned("dev open_pipe_stress_pdf_emitter")],
        ),
        (
            "a new build dependency",
            Box::new({
                let pdf = pdf_dir();
                move |text: &str| format!("{text}\n[build-dependencies]\nopen_pipe_stress_pdf_emitter = {{ path = \"{pdf}\" }}\n")
            }),
            Box::new(same),
            vec![],
            vec!["headless: the build-dependencies are {\"open_pipe_stress_pdf_emitter\"}, not {}".to_string()],
        ),
        (
            "the root without #![forbid(unsafe_code)]",
            Box::new(same),
            Box::new(|text: &str| {
                assert_eq!(text.matches(FORBID).count(), 1);
                text.replacen(FORBID, "", 1)
            }),
            vec![],
            vec!["headless/src/lib.rs:0: the crate root does not carry #![forbid(unsafe_code)] exactly".to_string()],
        ),
        (
            "a call through a declared foreign symbol",
            Box::new(same),
            Box::new(append("extern \"Rust\" { fn r9_symbol(); }\npub fn r9() { unsafe { r9_symbol() } }\n")),
            vec![],
            vec![
                "an extern block in the headless runner".to_string(),
                "an unsafe block in the headless runner".to_string(),
                "the unsafe keyword in the headless runner".to_string(),
                "which is not on the foreign-block pin list".to_string(),
            ],
        ),
        (
            "a new macro_rules! definition",
            Box::new(same),
            Box::new(append("macro_rules! r9 { () => { open_pipe_stress_report_package::user_release::assemble_derived }; }\n")),
            vec![],
            vec!["the macro definition r9 is not on the allow-list".to_string(), "assemble_derived named in a macro's or an attribute's tokens".to_string()],
        ),
        (
            "an edited pinned macro",
            Box::new(same),
            Box::new(same),
            vec![],
            vec!["the macro base differs from its pinned text".to_string()],
        ),
        (
            "another deserialize_with function",
            Box::new(same),
            Box::new(append("#[derive(Deserialize)]\npub struct R9 {\n    #[serde(deserialize_with = \"r9_read\")]\n    pub value: Value,\n}\n")),
            vec![],
            vec!["a deserialize_with naming a function other than the one admitted".to_string()],
        ),
        (
            "an import of the admitting entry",
            Box::new(same),
            Box::new(append("use open_pipe_stress_report_package::user_release::assemble_derived as r9;\n")),
            vec![],
            vec!["headless imports open_pipe_stress_report_package::user_release::assemble_derived".to_string(), "assemble_derived named in a use".to_string()],
        ),
        (
            "the renderer's pair named bare",
            Box::new(same),
            Box::new(append("pub fn r9() { let _ = render_calculation_report; }\n")),
            vec![],
            vec!["render_calculation_report named other than as a definition or a direct call".to_string()],
        ),
    ];
    // The local proc-macro repros (the reviewer's, renamed, and through
    // another path dependency): the scan's findings here; the proc-macro
    // itself is found by the path-closure check below.
    let tree_macro = ScratchTree::new();
    let r9_macro = proc_macro_package(&tree_macro, "r9_macro").display().to_string();
    let entry = |path: &str| format!("pub fn r9(request: &serde_json::Value) {{ let _ = {path}!()(request); }}\n");
    cases.push((
        "the reviewer's local proc-macro",
        Box::new({
            let r9_macro = r9_macro.clone();
            move |text: &str| with_dependency(text, &format!("r9_macro = {{ path = \"{r9_macro}\" }}"))
        }),
        Box::new(append("pub mod r9_entry;\n")),
        vec![("src/r9_entry.rs", entry("r9_macro::entry"))],
        vec![not_pinned("normal r9_macro"), format!("headless/src/r9_entry.rs:1: {}", invocation("r9_macro::entry"))],
    ));
    cases.push((
        "a renamed proc-macro",
        Box::new({
            let r9_macro = r9_macro.clone();
            move |text: &str| with_dependency(text, &format!("r9 = {{ package = \"r9_macro\", path = \"{r9_macro}\" }}"))
        }),
        Box::new(append("pub mod r9_entry;\n")),
        vec![("src/r9_entry.rs", entry("r9::entry"))],
        vec![not_pinned("normal r9_macro as r9"), invocation("r9::entry")],
    ));
    tree_macro.write("r9_helper/Cargo.toml", &format!("[package]\nname = \"r9_helper\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nr9_macro = {{ path = \"{r9_macro}\" }}\n"));
    tree_macro.write("r9_helper/src/lib.rs", "pub use r9_macro::entry;\n");
    let r9_helper = tree_macro.base.join("r9_helper").display().to_string();
    cases.push((
        "a proc-macro through another path dependency",
        Box::new({
            let r9_helper = r9_helper.clone();
            move |text: &str| with_dependency(text, &format!("r9_helper = {{ path = \"{r9_helper}\" }}"))
        }),
        Box::new(append("pub mod r9_entry;\n")),
        vec![("src/r9_entry.rs", entry("r9_helper::entry"))],
        vec![not_pinned("normal r9_helper"), invocation("r9_helper::entry")],
    ));
    let mut proc_macro_cases = 0;
    for (case, manifest, lib, files, expected) in cases {
        let tree = ScratchTree::new();
        let files: Vec<(&str, &str)> = files.iter().map(|(path, text)| (*path, text.as_str())).collect();
        let (mut sources, copy) = runner_copy(&tree, &*manifest, &*lib, &files);
        if case == "an edited pinned macro" {
            let file = "headless/src/reader_binding.rs";
            let text = sources.files.get_mut(file).unwrap();
            assert_eq!(text.matches("required_members: &[\"source\"],\n            optional_members: $optional,").count(), 1);
            *text = text.replacen("optional_members: $optional,", "optional_members: $optional, ", 1);
        }
        let findings = scan(&sources);
        for expected in &expected {
            assert!(findings.iter().any(|finding| finding.contains(expected.as_str())), "{case}: expected {expected:?} in {findings:#?}");
        }
        // Each case gives only its own findings.
        for finding in &findings {
            assert!(expected.iter().any(|expected| finding.contains(expected.as_str())), "{case}: unexpected {finding:?}");
        }
        // The path closure from the copy (the real tree's, from both
        // allowed dependents, is checked above): a proc-macro is a finding,
        // by target kind, however it is reached or named.
        let packages = path_packages(&[copy]).unwrap();
        let proc_macros = proc_macro_findings(&packages);
        if case.contains("proc-macro") {
            proc_macro_cases += 1;
            assert_eq!(proc_macros.len(), 1, "{case}: {proc_macros:#?}");
            let at = std::fs::canonicalize(Path::new(&r9_macro).join("Cargo.toml")).unwrap();
            assert_eq!(
                proc_macros[0],
                format!(
                    "the in-repository package r9_macro ({}) builds the proc-macro target r9_macro and is a path dependency of a reporting crate or an allowed dependent: its expansion is code the scan does not read",
                    at.display()
                ),
                "{case}"
            );
        } else {
            assert!(proc_macros.is_empty(), "{case}: {proc_macros:#?}");
        }
    }
    assert_eq!(proc_macro_cases, 3);
}

/// Code review r9 at #1682: the path-crossing rule on the real sources. The
/// two listed crossings are read as the shell's test code; the same file
/// loaded by another `src-tauri` test module is a finding.
#[test]
fn the_real_sources_find_the_review_r9_path_crossings() {
    let (findings, _, test_only) = scan_reading_all(&project_sources(&project()));
    assert!(findings.is_empty(), "{findings:#?}");
    for file in ["report_package/tests/common/package_basis.rs", "report_package/src/forwarding_graph_scan.rs"] {
        assert!(test_only.contains(file), "{file}: {test_only:?}");
    }
    let sources = with_file_text(
        "src-tauri/src/lib.rs",
        &format!("{}#[cfg(test)]\n#[path = \"../../../../core/reporting/report_package/tests/common/package_basis.rs\"]\nmod r9_basis;\n", project_sources(&project()).files["src-tauri/src/lib.rs"]),
        project_sources(&project()),
    );
    let findings = scan(&sources);
    assert_eq!(findings.len(), 1, "{findings:#?}");
    assert!(findings[0].starts_with("src-tauri/src/lib.rs:") && findings[0].contains("loads report_package/tests/common/package_basis.rs, outside its crate's directory, and is not on the path-crossing allow-list"), "{findings:#?}");
}

// --- Code review r9 follow-up at #1682: symbol-level FFI ------------------

/// The mangled symbol of the admitting entry, as a foreign item could declare
/// it under another identifier.
const MANGLED: &str = "_ZN31open_pipe_stress_report_package12user_release16assemble_derived17h0123456789abcdefE";

/// A scratch copy of `src-tauri` that Cargo reads (manifest with absolute
/// path dependencies, lock, build script and `src/`), with `edits` applied
/// as (file, old, new): `old` occurring exactly once is replaced, `"^"`
/// prepends and `"$"` appends; and the real sources with its part read from
/// the copy.
fn shell_copy_with(tree: &ScratchTree, edits: &[(&str, &str, String)]) -> Sources {
    let dir = tree.crate_copy("apps/desktop/src-tauri", "src-tauri", &[("build.rs", None)]);
    copy_dir(&project().join("apps/desktop/src-tauri/src"), &dir.join("src"));
    for (file, old, new) in edits {
        let path = dir.join(file);
        let text = std::fs::read_to_string(&path).unwrap();
        let edited = match *old {
            "^" => format!("{new}{text}"),
            "$" => format!("{text}{new}"),
            old => {
                assert_eq!(text.matches(old).count(), 1, "{file}: {old}");
                text.replacen(old, new, 1)
            }
        };
        std::fs::write(&path, edited).unwrap();
    }
    let mut sources = project_sources(&project());
    sources.files.retain(|file, _| !file.starts_with("src-tauri/"));
    sources.build_scripts.retain(|file, _| !file.starts_with("src-tauri/"));
    sources.bin_dirs.remove("src-tauri");
    crate_sources("src-tauri", &dir, &mut sources);
    sources
}

/// Also asserted (code review r9 follow-up at #1682; §2.8, "Symbol-level
/// FFI"): the shell's two roots carry `#![deny(unsafe_code)]`, exactly the
/// ten pinned `mod` items carry `#[allow(unsafe_code)]`, the one foreign
/// block is the pinned one, and no linkage attribute, dynamic lookup or
/// transmute occurs in any scanned crate (the real scan returns none).
#[test]
fn the_real_sources_hold_the_symbol_level_ffi_rules() {
    let sources = project_sources(&project());
    assert!(scan(&sources).is_empty());
    for root in ["src-tauri/src/lib.rs", "src-tauri/src/main.rs"] {
        assert_eq!(sources.files[root].matches("#![deny(unsafe_code)]\n").count(), 1, "{root}");
    }
    let mut allows = 0;
    for (file, text) in &sources.files {
        if file.starts_with("src-tauri/src/user_release_reach_scan") {
            continue;
        }
        allows += text.matches("#[allow(unsafe_code)]\n").count();
    }
    assert_eq!(allows, SHELL_UNSAFE_MODULES.len());
    for (file, name, _) in SHELL_UNSAFE_MODULES {
        let text = &sources.files[file];
        let declared = text.match_indices(&format!("mod {name};\n")).filter(|(at, _)| text[..*at].ends_with('\n')).count();
        assert_eq!(declared, 1, "{file}: {name}");
        let at = text.find(&format!("\nmod {name};\n")).unwrap();
        assert!(text[..at].rsplit('\n').take(3).any(|line| line == "#[allow(unsafe_code)]"), "{file}: {name}");
    }
    assert_eq!(FOREIGN_BLOCKS.len(), 1);
    assert_eq!(sources.files[FOREIGN_BLOCKS[0].0].matches("extern \"C\" {\n        fn mach_host_self() -> libc::mach_port_t;\n    }").count(), 1);
}

/// Code review r9 follow-up at #1682, on scratch copies of `src-tauri` that
/// Cargo reads: each break of the deny layer and of rules 1 to 3 gives its
/// own findings and only those; the unedited copy, with the pinned block as
/// it is, passes.
#[test]
fn the_symbol_level_ffi_repros_in_scratch_copies() {
    let lib = "src/lib.rs";
    let host = "src/host_memory.rs";
    let block = "    extern \"C\" {\n        fn mach_host_self() -> libc::mach_port_t;\n    }\n";
    let outside = "outside the pinned allow(unsafe_code) modules";
    let not_pinned = "which is not on the foreign-block pin list";
    let cases: Vec<(&str, Vec<(&str, &str, String)>, Vec<String>)> = vec![
        ("unedited, the pinned block as it is", vec![], vec![]),
        ("allow(unsafe_code) on another module", vec![(lib, "\nmod memory_pool;\n", "\n#[allow(unsafe_code)]\nmod memory_pool;\n".to_string())], vec![format!("src-tauri/src/lib.rs:19: #[allow(unsafe_code)] {outside}")]),
        (
            "the same through cfg_attr",
            vec![(lib, "\nmod memory_pool;\n", "\n#[cfg_attr(all(), allow(unsafe_code))]\nmod memory_pool;\n".to_string())],
            vec![format!("#[allow(unsafe_code)] (through cfg_attr) {outside}")],
        ),
        ("the same as an inner attribute", vec![("src/memory_pool.rs", "^", "#![allow(unsafe_code)]\n".to_string())], vec![format!("src-tauri/src/memory_pool.rs:1: #[allow(unsafe_code)] {outside}")]),
        ("expect(unsafe_code) on an item", vec![(lib, "$", "#[expect(unsafe_code)]\nfn r9() {}\n".to_string())], vec![format!("#[expect(unsafe_code)] {outside}")]),
        ("allow(unsafe_code) among other lints, on a function", vec![(lib, "$", "#[allow(dead_code, unsafe_code)]\nfn r9() {}\n".to_string())], vec![format!("#[allow(unsafe_code)] {outside}")]),
        ("a removed root deny", vec![(lib, "#![deny(unsafe_code)]\n", String::new())], vec!["src-tauri/src/lib.rs:0: the crate root does not carry #![deny(unsafe_code)] exactly".to_string()]),
        ("the binary root's deny removed", vec![("src/main.rs", "#![deny(unsafe_code)]\n", String::new())], vec!["src-tauri/src/main.rs:0: the crate root does not carry #![deny(unsafe_code)] exactly".to_string()]),
        (
            "a root deny weakened to warn",
            vec![(lib, "#![deny(unsafe_code)]\n", "#![warn(unsafe_code)]\n".to_string())],
            vec!["src-tauri/src/lib.rs:0: the crate root does not carry #![deny(unsafe_code)] exactly".to_string(), format!("#[warn(unsafe_code)] {outside}")],
        ),
        (
            "a pinned module without its allow",
            vec![(lib, "#[allow(unsafe_code)]\nmod solve_jobs;\n", "mod solve_jobs;\n".to_string())],
            vec!["src-tauri/src/lib.rs: the pinned module solve_jobs does not carry #[allow(unsafe_code)] on its mod item".to_string()],
        ),
        ("an unpinned foreign block declaring the mangled symbol", vec![(lib, "$", format!("extern \"Rust\" {{\n    fn {MANGLED}();\n}}\n"))], vec![format!("fn {MANGLED}(); }}, {not_pinned}")]),
        (
            "an extra item in the pinned block",
            vec![(host, block, format!("    extern \"C\" {{\n        fn mach_host_self() -> libc::mach_port_t;\n        fn {MANGLED}();\n    }}\n"))],
            vec![format!("fn {MANGLED}(); }}, {not_pinned}")],
        ),
        ("an attribute on the pinned block", vec![(host, block, format!("    #[cfg(all())]\n{block}"))], vec![not_pinned.to_string()]),
        (
            "link_name aliasing in the pinned block",
            vec![(host, block, format!("    extern \"C\" {{\n        #[link_name = \"{MANGLED}\"]\n        fn mach_host_self() -> libc::mach_port_t;\n    }}\n"))],
            vec!["the linkage attribute #[link_name]".to_string(), not_pinned.to_string()],
        ),
        ("no_mangle", vec![(lib, "$", "#[no_mangle]\npub extern \"C\" fn r9() {}\n".to_string())], vec!["the linkage attribute #[no_mangle]".to_string()]),
        ("export_name", vec![(lib, "$", "#[export_name = \"r9\"]\npub extern \"C\" fn r9() {}\n".to_string())], vec!["the linkage attribute #[export_name]".to_string()]),
        ("no_mangle through cfg_attr", vec![(lib, "$", "#[cfg_attr(all(), no_mangle)]\npub extern \"C\" fn r9() {}\n".to_string())], vec!["the linkage attribute #[no_mangle] (through cfg_attr)".to_string()]),
        ("the unsafe(no_mangle) form", vec![(lib, "$", "#[unsafe(no_mangle)]\npub extern \"C\" fn r9() {}\n".to_string())], vec!["the linkage attribute #[unsafe(no_mangle)]".to_string()]),
        ("link and used", vec![(lib, "$", "#[used]\nstatic R9: u8 = 0;\n#[link(name = \"r9\")]\nfn r9() {}\n".to_string())], vec!["the linkage attribute #[used]".to_string(), "the linkage attribute #[link]".to_string()]),
        (
            "dlsym and transmute",
            vec![(lib, "$", "fn r9(handle: *mut libc::c_void) { let symbol = libc::dlsym(handle, c\"r9\".as_ptr()); let entry: fn() = std::mem::transmute(symbol); entry(); }\n".to_string())],
            vec!["dlsym, dynamic symbol lookup".to_string(), "transmute, a reinterpreting transmute".to_string()],
        ),
        ("libloading", vec![(lib, "$", "use libloading::Library;\nfn r9() { let _ = Library::new(\"r9\"); }\n".to_string())], vec!["libloading, dynamic symbol lookup".to_string()]),
        ("dlopen in a macro's tokens", vec![(lib, "$", "fn r9() { println!(\"{:?}\", libc::dlopen(std::ptr::null(), 0)); }\n".to_string())], vec!["dlopen, dynamic symbol lookup".to_string()]),
    ];
    for (case, edits, expected) in cases {
        let tree = ScratchTree::new();
        let findings = scan(&shell_copy_with(&tree, &edits));
        for expected in &expected {
            assert!(findings.iter().any(|finding| finding.contains(expected.as_str())), "{case}: expected {expected:?} in {findings:#?}");
        }
        for finding in &findings {
            assert!(expected.iter().any(|expected| finding.contains(expected.as_str())), "{case}: unexpected {finding:?}");
        }
    }
}
