//! The admitted-set forwarding-graph scan (W-11, DEL-08-01 Design
//! `exact-route-report-sections.md` §5.5), shared by the report package's
//! scan (`admitted_tests.rs`) and the desktop native render's
//! (`apps/desktop/src-tauri/src/report_render_exact_tests.rs`, which includes
//! this file by path). A test-only module: each crate declares it under
//! `#[cfg(test)]`.
//!
//! A non-test source is the file with every column-0 `#[cfg(test)]` item
//! removed: the item runs to its first column-0 `}` line, or to the end of its
//! line if that line ends with `;`. A file that a `#[cfg(test)] mod <name>;`
//! declaration names is removed whole. An indented `#[cfg(test)]` is kept, so
//! anything it calls counts as a non-test call. The scan fails closed if an
//! item's end is not found. In non-test source, a graph function's name may
//! occur only in its definition and in direct calls (`name(`) by its allowed
//! callers: an alias, a function value, a `use` of it or a re-export is a
//! finding, so the set cannot leave the graph through a name the scan does not
//! follow. Standard claim fence applies (claims taxonomy per DEC-081).

#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Arg {
    /// A production root: `<set module>::PRODUCTION`.
    Production,
    /// An internal edge: its own `admitted` parameter.
    Admitted,
    /// A graph function that takes no set (the desktop render's command and
    /// renderer call): only its callers are checked.
    None,
}

/// (function, its file, [(caller file, caller function, argument)]). A
/// function takes the set, as its last parameter `admitted: <set type>`,
/// exactly when some caller passes one.
pub(crate) type Row = (&'static str, &'static str, &'static [(&'static str, &'static str, Arg)]);

/// One crate's graph and the names of its set.
pub(crate) struct Graph {
    pub(crate) rows: &'static [Row],
    /// The set's type, e.g. `AdmittedSet`.
    pub(crate) set_type: &'static str,
    /// The module (and file stem) that defines the set, e.g. `admitted`.
    pub(crate) set_module: &'static str,
    /// The widened test-only value, e.g. `EXACT_WITNESS_ADMITTED`.
    pub(crate) witness: &'static str,
    /// The test files that may name the widened value (the witness module and
    /// the scan's own test file).
    pub(crate) witness_files: &'static [&'static str],
}

impl Graph {
    fn set_home(&self) -> String {
        format!("{}.rs", self.set_module)
    }
}

fn indent(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

/// The line without its `//` comment, outside string literals (approximate:
/// a `//` after an even number of unescaped quotes).
pub(crate) fn code_of(line: &str) -> &str {
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
pub(crate) fn non_test_source(file: &str, text: &str) -> Result<(String, Vec<String>), String> {
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
/// (enclosing function, last argument, line). Any other occurrence of the
/// name (an alias, a function value, a `use`, a re-export) is a finding.
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
        let line = offsets.partition_point(|start| *start <= at) - 1;
        let rest = &code[at + name.len()..];
        let Some(paren) = rest.find(|ch: char| !ch.is_whitespace()).filter(|index| rest[*index..].starts_with('(')) else {
            findings.push(format!(
                "{file}:{}: {name} is used other than as a direct call (an alias, a function value or a re-export)",
                line + 1
            ));
            continue;
        };
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

/// The scan of `files` (path relative to `src` → text) against `graph`.
/// Empty when the graph is closed.
pub(crate) fn scan(graph: &Graph, files: &BTreeMap<String, String>) -> Vec<String> {
    let set_type = graph.set_type;
    let set_home = graph.set_home();
    let root_argument = format!("{}::PRODUCTION", graph.set_module);
    let set_parameter = format!("admitted: {set_type}");
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

    for (variant, home, allowed) in graph.rows {
        let takes_set = allowed.iter().any(|(_, _, argument)| *argument != Arg::None);
        // 5 and the definition: one crate-private definition in its file,
        // taking the set last.
        let mut definitions = Vec::new();
        for (file, code) in &sources {
            for line in code.lines() {
                if fn_name(line).as_deref() == Some(*variant) {
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
        if takes_set {
            if let Some(code) = sources.get(*home) {
                if let Some(start) = code.find(&format!("fn {variant}(")) {
                    let open = start + 3 + variant.len();
                    if last_argument(code, open).as_deref() != Some(set_parameter.as_str()) {
                        findings.push(format!("{variant} does not take `{set_parameter}` as its last parameter"));
                    }
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
                    Some((_, _, Arg::Production)) if argument != root_argument => {
                        findings.push(format!("{file}:{line}: the root {enclosing} passes {argument}, not {root_argument}"))
                    }
                    Some((_, _, Arg::Admitted)) if argument != "admitted" => {
                        findings.push(format!("{file}:{line}: the edge {enclosing} passes {argument}, not admitted"))
                    }
                    Some((_, _, Arg::Production)) => {
                        let imports = code.contains(&format!("use crate::{}", graph.set_module))
                            || (file == "lib.rs" && code.contains(&format!("mod {};", graph.set_module)));
                        if !imports {
                            findings.push(format!("{file}: does not import crate::{}", graph.set_module));
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
    let construction = format!("{set_type}(");
    for (file, code) in &sources {
        for line in code.lines().filter(|line| line.contains(&construction)) {
            let allowed = *file == set_home
                && (line.starts_with(&format!("pub(crate) struct {set_type}("))
                    || line.starts_with(&format!("pub(crate) const PRODUCTION: {set_type} = {set_type}(")));
            if !allowed {
                findings.push(format!("{file}: {set_type} is constructed outside {set_home}: {}", line.trim()));
            }
        }
    }
    // 4: nowhere in non-test source; in the raw sources only in its
    // `#[cfg(test)]` definition and the witness files.
    let witness = graph.witness;
    for (file, code) in &sources {
        if code.contains(witness) {
            findings.push(format!("{file}: {witness} in non-test source"));
        }
    }
    let definition_line = format!("pub(crate) const {witness}: {set_type} = ");
    for (file, text) in files {
        if graph.witness_files.contains(&file.as_str()) {
            continue;
        }
        let lines: Vec<&str> = text.lines().collect();
        for (index, line) in lines.iter().enumerate() {
            if !code_of(line).contains(witness) {
                continue;
            }
            let definition =
                *file == set_home && line.starts_with(&definition_line) && index > 0 && lines[index - 1] == "#[cfg(test)]";
            if !definition {
                findings.push(format!("{file}:{}: {witness} used outside the witness module", index + 1));
            }
        }
    }
    findings
}
