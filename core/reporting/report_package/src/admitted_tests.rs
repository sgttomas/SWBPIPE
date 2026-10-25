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
//! 4. `EXACT_WITNESS_ADMITTED` occurs only in its `#[cfg(test)]` definition and
//!    in the witness module;
//! 5. no variant is declared `pub fn`.
//!
//! A non-test source is the file with every column-0 `#[cfg(test)]` item
//! removed: the item runs to its first column-0 `}` line, or to the end of its
//! line if that line ends with `;`. A file that a `#[cfg(test)] mod <name>;`
//! declaration names is removed whole. An indented `#[cfg(test)]` is kept, so
//! anything it calls counts as a non-test call. The scan fails closed if an
//! item's end is not found. Standard claim fence applies (claims taxonomy per
//! DEC-081).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Arg {
    /// A production root: `admitted::PRODUCTION`.
    Production,
    /// An internal edge: its own `admitted` parameter.
    Admitted,
}

/// (variant, its file, [(caller file, caller function, argument)]).
type Row = (&'static str, &'static str, &'static [(&'static str, &'static str, Arg)]);

const GRAPH: [Row; 6] = [
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

const WITNESS_MODULE: &str = "exact_package_witness_tests.rs";
const SCANNER: &str = "admitted_tests.rs";

fn indent(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

/// The line without its `//` comment, outside string literals (approximate:
/// a `//` after an even number of unescaped quotes).
fn code_of(line: &str) -> &str {
    let bytes = line.as_bytes();
    let mut quotes = 0usize;
    let mut index = 0;
    while index + 1 < bytes.len() {
        match bytes[index] {
            b'\\' => index += 1,
            b'"' => quotes += 1,
            b'/' if bytes[index + 1] == b'/' && quotes % 2 == 0 => return &line[..index],
            _ => {}
        }
        index += 1;
    }
    line
}

/// The file with its column-0 `#[cfg(test)]` items blanked (line numbers
/// kept), and the names of the `#[cfg(test)] mod <name>;` files.
fn non_test_source(file: &str, text: &str) -> Result<(String, Vec<String>), String> {
    let lines: Vec<&str> = text.lines().collect();
    let mut kept: Vec<&str> = Vec::with_capacity(lines.len());
    let mut test_modules = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        if !lines[index].starts_with("#[cfg(test)]") {
            kept.push(lines[index]);
            index += 1;
            continue;
        }
        // The item's line: the first line after the attribute that is not
        // itself an attribute.
        let mut start = index + 1;
        if lines[index].trim_end() != "#[cfg(test)]" {
            start = index;
        }
        while start < lines.len() && start > index && lines[start].starts_with("#[") {
            start += 1;
        }
        let Some(item) = lines.get(start) else {
            return Err(format!("{file}:{}: a #[cfg(test)] item has no end", index + 1));
        };
        let body = if start == index { item.trim_start_matches("#[cfg(test)]").trim() } else { item.trim() };
        let end = if code_of(body).trim_end().ends_with(';') {
            if let Some(name) = body.strip_prefix("mod ").and_then(|rest| rest.strip_suffix(';')) {
                test_modules.push(name.trim().to_string());
            }
            start
        } else {
            match (start..lines.len()).find(|&line| lines[line].starts_with('}')) {
                Some(end) => end,
                None => return Err(format!("{file}:{}: a #[cfg(test)] item has no end", index + 1)),
            }
        };
        kept.extend(std::iter::repeat_n("", end + 1 - index));
        index = end + 1;
    }
    Ok((kept.join("\n"), test_modules))
}

/// The file a `mod <name>;` declared in `file` names.
fn module_files(file: &str, name: &str) -> [String; 2] {
    let dir = match Path::new(file).file_name().and_then(|name| name.to_str()) {
        Some("lib.rs" | "mod.rs" | "main.rs") => Path::new(file).parent().map(Path::to_path_buf).unwrap_or_default(),
        _ => Path::new(file).with_extension(""),
    };
    let join = |tail: String| dir.join(tail).to_string_lossy().trim_start_matches('/').to_string();
    [join(format!("{name}.rs")), join(format!("{name}/mod.rs"))]
}

/// The last top-level argument of the call whose `(` is at `open`.
fn last_argument(code: &str, open: usize) -> Option<String> {
    let mut depth = 0i32;
    let mut start = open + 1;
    let mut last = String::new();
    for (offset, ch) in code[open..].char_indices() {
        let at = open + offset;
        match ch {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => {
                depth -= 1;
                if depth == 0 {
                    let tail = code[start..at].trim();
                    if !tail.is_empty() {
                        last = tail.to_string();
                    }
                    return Some(last);
                }
            }
            ',' if depth == 1 => {
                let piece = code[start..at].trim();
                if !piece.is_empty() {
                    last = piece.to_string();
                }
                start = at + 1;
            }
            _ => {}
        }
    }
    None
}

fn is_ident(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// Every occurrence of `name(` as a call (not its definition) in `code`:
/// (enclosing function, last argument, line).
fn calls(file: &str, code: &str, name: &str, findings: &mut Vec<String>) -> Vec<(String, String, usize)> {
    let lines: Vec<&str> = code.lines().collect();
    let mut offsets = Vec::with_capacity(lines.len());
    let mut offset = 0;
    for line in &lines {
        offsets.push(offset);
        offset += line.len() + 1;
    }
    let mut out = Vec::new();
    let bytes = code.as_bytes();
    let mut from = 0;
    while let Some(found) = code[from..].find(name) {
        let at = from + found;
        from = at + name.len();
        if at > 0 && is_ident(bytes[at - 1]) || bytes.get(at + name.len()).is_some_and(|byte| is_ident(*byte)) {
            continue;
        }
        let rest = &code[at + name.len()..];
        let Some(paren) = rest.find(|ch: char| !ch.is_whitespace()).filter(|index| rest[*index..].starts_with('(')) else {
            continue;
        };
        let line = offsets.partition_point(|start| *start <= at) - 1;
        let before = code[offsets[line]..at].trim_end();
        if before.ends_with("fn") {
            continue;
        }
        let depth = indent(lines[line]);
        let enclosing = (0..line).rev().find_map(|candidate| {
            let text = lines[candidate];
            if indent(text) >= depth {
                return None;
            }
            fn_name(text)
        });
        let Some(enclosing) = enclosing else {
            findings.push(format!("{file}:{}: a call of {name} has no enclosing function", line + 1));
            continue;
        };
        match last_argument(code, at + name.len() + paren) {
            Some(argument) => out.push((enclosing, argument, line + 1)),
            None => findings.push(format!("{file}:{}: the call of {name} has unbalanced parentheses", line + 1)),
        }
    }
    out
}

/// The function a line declares, if any.
fn fn_name(line: &str) -> Option<String> {
    let code = code_of(line).trim_start();
    let position = code.find("fn ")?;
    let head = &code[..position];
    let qualifiers = head.split_whitespace().all(|word| {
        word == "pub" || word.starts_with("pub(") || ["async", "const", "unsafe", "extern"].contains(&word)
    });
    if !qualifiers {
        return None;
    }
    let name: String = code[position + 3..].trim_start().chars().take_while(|ch| ch.is_alphanumeric() || *ch == '_').collect();
    (!name.is_empty()).then_some(name)
}

/// The scan of `files` (path relative to `src` → text). Empty when the graph
/// is closed.
fn scan(files: &BTreeMap<String, String>) -> Vec<String> {
    let mut findings = Vec::new();
    let mut sources: BTreeMap<String, String> = BTreeMap::new();
    let mut removed: BTreeSet<String> = BTreeSet::new();
    for (file, text) in files {
        match non_test_source(file, text) {
            Ok((code, modules)) => {
                for module in modules {
                    removed.extend(module_files(file, &module));
                }
                sources.insert(file.clone(), code.lines().map(code_of).collect::<Vec<_>>().join("\n"));
            }
            Err(finding) => findings.push(finding),
        }
    }
    sources.retain(|file, _| !removed.contains(file));

    for (variant, home, allowed) in GRAPH {
        // 5 and the definition: one crate-private definition in its file,
        // taking the set last.
        let mut definitions = Vec::new();
        for (file, code) in &sources {
            for line in code.lines() {
                if fn_name(line).as_deref() == Some(variant) {
                    definitions.push((file.clone(), line.trim().to_string()));
                }
            }
        }
        match definitions.as_slice() {
            [(file, line)] if file == home => {
                if line.starts_with("pub fn") || line.starts_with("pub async fn") {
                    findings.push(format!("{variant} is declared pub fn"));
                }
            }
            _ => findings.push(format!("{variant}: expected one definition in {home}, found {definitions:?}")),
        }
        if let Some(code) = sources.get(home) {
            if let Some(start) = code.find(&format!("fn {variant}(")) {
                let open = start + 3 + variant.len();
                if last_argument(code, open).as_deref() != Some("admitted: AdmittedSet") {
                    findings.push(format!("{variant} does not take `admitted: AdmittedSet` as its last parameter"));
                }
            }
        }
        // 1 and 2.
        let mut found: BTreeSet<(String, String)> = BTreeSet::new();
        for (file, code) in &sources {
            for (enclosing, argument, line) in calls(file, code, variant, &mut findings) {
                let edge = allowed.iter().find(|(caller_file, caller, _)| *caller_file == file && *caller == enclosing);
                match edge {
                    None => findings.push(format!("{file}:{line}: {enclosing} calls {variant} outside the forwarding graph")),
                    Some((_, _, Arg::Production)) if argument != "admitted::PRODUCTION" => {
                        findings.push(format!("{file}:{line}: the root {enclosing} passes {argument}, not admitted::PRODUCTION"))
                    }
                    Some((_, _, Arg::Admitted)) if argument != "admitted" => {
                        findings.push(format!("{file}:{line}: the edge {enclosing} passes {argument}, not admitted"))
                    }
                    Some((_, _, Arg::Production)) => {
                        let imports = code.contains("use crate::admitted") || (file == "lib.rs" && code.contains("mod admitted;"));
                        if !imports {
                            findings.push(format!("{file}: does not import crate::admitted"));
                        }
                    }
                    Some(_) => {}
                }
                found.insert((file.clone(), enclosing));
            }
        }
        let expected: BTreeSet<(String, String)> =
            allowed.iter().map(|(file, caller, _)| (file.to_string(), caller.to_string())).collect();
        if found != expected {
            findings.push(format!("{variant}: callers {found:?}, expected {expected:?}"));
        }
    }

    // 3.
    for (file, code) in &sources {
        for line in code.lines().filter(|line| line.contains("AdmittedSet(")) {
            let allowed = file == "admitted.rs"
                && (line.starts_with("pub(crate) struct AdmittedSet(")
                    || line.starts_with("pub(crate) const PRODUCTION: AdmittedSet = AdmittedSet("));
            if !allowed {
                findings.push(format!("{file}: AdmittedSet is constructed outside admitted.rs: {}", line.trim()));
            }
        }
    }
    // 4: nowhere in non-test source; in the raw sources only in its
    // `#[cfg(test)]` definition and the witness module.
    for (file, code) in &sources {
        if code.contains("EXACT_WITNESS_ADMITTED") {
            findings.push(format!("{file}: EXACT_WITNESS_ADMITTED in non-test source"));
        }
    }
    for (file, text) in files {
        if file == WITNESS_MODULE || file == SCANNER {
            continue;
        }
        let lines: Vec<&str> = text.lines().collect();
        for (index, line) in lines.iter().enumerate() {
            if !code_of(line).contains("EXACT_WITNESS_ADMITTED") {
                continue;
            }
            let definition = file == "admitted.rs"
                && line.starts_with("pub(crate) const EXACT_WITNESS_ADMITTED: AdmittedSet = ")
                && index > 0
                && lines[index - 1] == "#[cfg(test)]";
            if !definition {
                findings.push(format!("{file}:{}: EXACT_WITNESS_ADMITTED used outside the witness module", index + 1));
            }
        }
    }
    findings
}

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
#[cfg(test)]
pub(crate) const EXACT_WITNESS_ADMITTED: AdmittedSet = AdmittedSet(&B);
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
        ("admitted_tests.rs".to_string(), file("fn scan() { EXACT_WITNESS_ADMITTED }\n")),
        ("exact_package_witness_tests.rs".to_string(), file("fn w() { assemble_wire_request_in(r, EXACT_WITNESS_ADMITTED) }\n")),
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
            "a witness-set use outside the witness module",
            edit("wire.rs", "crate::bindings::check_request_in(&request, admitted)?;", "crate::bindings::check_request_in(&request, EXACT_WITNESS_ADMITTED)?;"),
            "EXACT_WITNESS_ADMITTED in non-test source",
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
