//! The admitted-set forwarding graph scan (W-11, DEL-08-01 Design
//! `exact-route-report-sections.md` §5.5). The set reaches item 0 only along
//! the graph below; the scan reads every `src/**/*.rs` file through
//! `CARGO_MANIFEST_DIR`, removes the test-only items, and asserts:
//! 1. each variant's set of enclosing functions with a call site equals its
//!    row's allowed callers, exactly;
//! 2. a root's call passes `admitted::PRODUCTION` as its last argument (and
//!    its file imports `crate::admitted`); an internal edge passes `admitted`;
//! 3. `AdmittedSet(` occurs in non-test source only in `admitted.rs`, in the
//!    struct and in `PRODUCTION`'s definition;
//! 4. no test-only set exists: in every source, test items and test modules
//!    included, `AdmittedSet(` occurs only in the struct and `PRODUCTION`'s
//!    definition, and no other `AdmittedSet` constant is declared. Since the
//!    owner opened exact-route packages (§5.1; owner, 2026-10-26) the
//!    production set admits the three identities, so the in-crate witness
//!    module uses the public roots;
//! 5. no variant is declared `pub fn`.
//!
//! The scan rule (test-item removal, and the rejection of any use of a graph
//! function other than a direct call by an allowed caller) is
//! `forwarding_graph_scan`'s, shared with the desktop native render's scan.
//! Standard claim fence applies (claims taxonomy per DEC-081).

use crate::forwarding_graph_scan::{non_test_source, Arg, Graph, Row};
use std::collections::BTreeMap;
use std::path::Path;

const ROWS: [Row; 6] = [
    (
        "check_route_admitted_in",
        "document_form.rs",
        &[
            ("document_form.rs", "check_route_admitted", Arg::Production),
            ("wire.rs", "assemble_wire_request_in", Arg::Admitted),
            ("bindings.rs", "check_request_in", Arg::Admitted),
            ("lib.rs", "assemble_report_package_container_in", Arg::Admitted),
            ("reader.rs", "verify_members", Arg::Admitted),
        ],
    ),
    ("assemble_wire_request_in", "wire.rs", &[("wire.rs", "assemble_wire_request", Arg::Production)]),
    (
        "check_request_in",
        "bindings.rs",
        &[("bindings.rs", "check_request", Arg::Production), ("wire.rs", "assemble_wire_request_in", Arg::Admitted)],
    ),
    (
        "assemble_report_package_container_in",
        "lib.rs",
        &[
            ("lib.rs", "assemble_report_package_container", Arg::Production),
            ("wire.rs", "assemble_wire_request_in", Arg::Admitted),
        ],
    ),
    (
        "read_report_package_container_in",
        "reader.rs",
        &[("reader.rs", "read_report_package_container", Arg::Production)],
    ),
    ("verify_members", "reader.rs", &[("reader.rs", "read_report_package_container_in", Arg::Admitted)]),
];

const SCANNER: &str = "admitted_tests.rs";

const GRAPH: Graph = Graph { rows: &ROWS, set_type: "AdmittedSet", set_module: "admitted", scanner_files: &[SCANNER] };

fn scan(files: &BTreeMap<String, String>) -> Vec<String> {
    crate::forwarding_graph_scan::scan(&GRAPH, files)
}

/// Every `.rs` file under `dir`, keyed by its path relative to `root`.
fn read_sources(dir: &Path, root: &Path, out: &mut BTreeMap<String, String>) {
    let mut entries: Vec<_> = std::fs::read_dir(dir).unwrap().map(|entry| entry.unwrap().path()).collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            read_sources(&path, root, out);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            let local = path.strip_prefix(root).unwrap().to_string_lossy().replace('\\', "/");
            out.insert(local, std::fs::read_to_string(&path).unwrap());
        }
    }
}

#[test]
fn admitted_set_forwarding_graph_is_closed() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = BTreeMap::new();
    read_sources(&root, &root, &mut files);
    assert!(files.contains_key("admitted.rs") && files.contains_key(SCANNER), "{:?}", files.keys());
    let findings = scan(&files);
    assert!(findings.is_empty(), "{findings:#?}");
    // The scan removed this file and the other test modules whole.
    let (_, modules) = non_test_source("lib.rs", &files["lib.rs"]).unwrap();
    assert!(modules.contains(&"admitted_tests".to_string()), "{modules:?}");
}

/// An invented crate whose graph is closed; each negative case edits it once.
fn invented_crate() -> BTreeMap<String, String> {
    let file = |text: &str| text.trim_start_matches('\n').to_string();
    BTreeMap::from([
        (
            "admitted.rs".to_string(),
            file("
pub(crate) struct AdmittedSet(&'static [&'static str]);
pub(crate) const PRODUCTION: AdmittedSet = AdmittedSet(&A);
"),
        ),
        (
            "document_form.rs".to_string(),
            file("
use crate::admitted::{self, AdmittedSet};
pub fn check_route_admitted(document: &Value) -> Result<(), String> {
    check_route_admitted_in(document, admitted::PRODUCTION)
}
pub(crate) fn check_route_admitted_in(document: &Value, admitted: AdmittedSet) -> Result<(), String> {
    Ok(())
}
"),
        ),
        (
            "wire.rs".to_string(),
            file("
use crate::admitted::{self, AdmittedSet};
pub fn assemble_wire_request(request: R) -> O {
    assemble_wire_request_in(request, admitted::PRODUCTION)
}
pub(crate) fn assemble_wire_request_in(request: R, admitted: AdmittedSet) -> O {
    for document in &request.documents {
        crate::document_form::check_route_admitted_in(document, admitted)?;
    }
    crate::bindings::check_request_in(&request, admitted)?;
    package::assemble_report_package_container_in(&input, admitted)
}
"),
        ),
        (
            "bindings.rs".to_string(),
            file("
use crate::admitted::{self, AdmittedSet};
pub fn check_request(request: &R) -> Result<(), String> {
    check_request_in(request, admitted::PRODUCTION)
}
pub(crate) fn check_request_in(request: &R, admitted: AdmittedSet) -> Result<(), String> {
    document_form::check_route_admitted_in(document, admitted)?;
    Ok(())
}
"),
        ),
        (
            "lib.rs".to_string(),
            file("
pub(crate) mod admitted;
#[cfg(test)]
mod admitted_tests;
#[cfg(test)]
mod exact_package_witness_tests;
pub fn assemble_report_package_container(input: &I) -> O {
    assemble_report_package_container_in(input, admitted::PRODUCTION)
}
pub(crate) fn assemble_report_package_container_in(input: &I, admitted: AdmittedSet) -> O {
    let checks = [document_form::check_route_admitted_in(document, admitted)];
    O
}
#[cfg(test)]
mod archive_tests {
    fn a() { check_route_admitted_in(d, admitted::PRODUCTION); }
}
"),
        ),
        (
            "reader.rs".to_string(),
            file("
use crate::admitted::{self, AdmittedSet};
pub fn read_report_package_container(bytes: &[u8]) -> R {
    read_report_package_container_in(bytes, admitted::PRODUCTION)
}
pub(crate) fn read_report_package_container_in(bytes: &[u8], admitted: AdmittedSet) -> R {
    let members = verify_members(bytes, &archive, admitted)?;
    Ok(members)
}
fn verify_members(bytes: &[u8], archive: &A, admitted: AdmittedSet) -> R {
    document_form::check_route_admitted_in(&document, admitted).map_err(&defect)?;
}
"),
        ),
        ("admitted_tests.rs".to_string(), file("fn scan() { AdmittedSet(&B) }\n")),
        ("exact_package_witness_tests.rs".to_string(), file("fn w() { assemble_wire_request(r) }\n")),
    ])
}

#[test]
fn the_scanner_finds_each_break_of_the_graph() {
    let closed = invented_crate();
    assert!(scan(&closed).is_empty(), "{:#?}", scan(&closed));
    let edit = |file: &str, old: &str, new: &str| {
        let mut files = closed.clone();
        let text = files.get_mut(file).unwrap();
        assert_eq!(text.matches(old).count(), 1, "{old}");
        *text = text.replace(old, new);
        files
    };
    let cases = [
        (
            "a sixth caller",
            edit("reader.rs", "    Ok(members)\n", "    document_form::check_route_admitted_in(&first, admitted)?;\n    Ok(members)\n"),
            "read_report_package_container_in calls check_route_admitted_in outside the forwarding graph",
        ),
        (
            "a root passing admitted",
            edit("bindings.rs", "check_request_in(request, admitted::PRODUCTION)", "check_request_in(request, admitted)"),
            "the root check_request passes admitted, not admitted::PRODUCTION",
        ),
        (
            "an internal edge passing PRODUCTION",
            edit("wire.rs", "crate::bindings::check_request_in(&request, admitted)?;", "crate::bindings::check_request_in(&request, admitted::PRODUCTION)?;"),
            "the edge assemble_wire_request_in passes admitted::PRODUCTION, not admitted",
        ),
        (
            "a test-only widened set",
            edit(
                "admitted.rs",
                "pub(crate) const PRODUCTION: AdmittedSet = AdmittedSet(&A);\n",
                "pub(crate) const PRODUCTION: AdmittedSet = AdmittedSet(&A);\n#[cfg(test)]\npub(crate) const EXACT_WITNESS_ADMITTED: AdmittedSet = AdmittedSet(&B);\n",
            ),
            "admitted.rs:4: a set of type AdmittedSet other than PRODUCTION",
        ),
        (
            "a set constructed in a test module",
            edit("exact_package_witness_tests.rs", "fn w() { assemble_wire_request(r) }", "fn w() { assemble_wire_request_in(r, AdmittedSet(&B)) }"),
            "exact_package_witness_tests.rs:1: a set of type AdmittedSet other than PRODUCTION",
        ),
        (
            "an indented test item's call counts",
            edit("lib.rs", "    O\n}\n#[cfg(test)]", "    O\n}\nfn helper() {\n    #[cfg(test)]\n    check_route_admitted_in(d, admitted);\n}\n#[cfg(test)]"),
            "helper calls check_route_admitted_in outside the forwarding graph",
        ),
        (
            "a public variant",
            edit("reader.rs", "pub(crate) fn read_report_package_container_in", "pub fn read_report_package_container_in"),
            "read_report_package_container_in is declared pub fn",
        ),
        (
            "a construction outside admitted.rs",
            edit("wire.rs", "    package::assemble", "    let wide = AdmittedSet(&C);\n    package::assemble"),
            "AdmittedSet is constructed outside admitted.rs",
        ),
        (
            // Codex review round 1: an alias in another function, with every
            // existing edge intact.
            "an alias of a variant",
            edit(
                "lib.rs",
                "    O\n}\n#[cfg(test)]",
                "    O\n}\nfn other(document: &Value, admitted: AdmittedSet) {\n    let check = crate::document_form::check_route_admitted_in;\n    check(document, admitted);\n}\n#[cfg(test)]",
            ),
            "lib.rs:14: check_route_admitted_in is used other than as a direct call",
        ),
        (
            "a variant passed as a function value",
            edit("reader.rs", "    Ok(members)\n", "    documents.iter().try_for_each(|d| Some(d).map(document_form::check_route_admitted_in));\n    Ok(members)\n"),
            "check_route_admitted_in is used other than as a direct call",
        ),
        (
            "a re-export of a variant",
            edit("wire.rs", "use crate::admitted::{self, AdmittedSet};\n", "use crate::admitted::{self, AdmittedSet};\npub(crate) use crate::bindings::check_request_in as check;\n"),
            "wire.rs:2: check_request_in is used other than as a direct call",
        ),
        (
            "a test item without an end",
            edit("lib.rs", "    fn a() { check_route_admitted_in(d, admitted::PRODUCTION); }\n}\n", "    fn a() { check_route_admitted_in(d, admitted::PRODUCTION); }\n    }\n"),
            "a #[cfg(test)] item has no end",
        ),
    ];
    for (case, files, expected) in cases {
        let findings = scan(&files);
        assert!(findings.iter().any(|finding| finding.contains(expected)), "{case}: {findings:#?}");
    }
}
