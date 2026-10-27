//! The native refusals of route `DREP-PACKAGE-USER-RELEASE-010` that read the
//! bound model and its run record (W-18b slice (r2b); DEL-08-01 Design
//! `report-user-release.md` §2.5 steps 5, 6 and 8, §2.7), beside
//! `export_disposition_screen`, whose one walk they share:
//! - the library rules 1-3 ([`refuse_library_referenced`]): a `library_ref`
//!   member at any depth, whatever its value (its own detector,
//!   [`library_ref_member_sites`]); a library provenance string; the legacy
//!   hanger selection evidence, in its text or as embedded JSON, which is
//!   prescanned and priced first and parsed one candidate at a time;
//! - the rule-pack rule ([`refuse_rule_pack_referenced`]);
//! - the private-class rule ([`refuse_private_classes`]): a label can add a
//!   refusal, and no label grants inclusion;
//! - the run-record check ([`check_run_record`]): Record Run's binding checks
//!   (`analysis_runs::check_record_for_values`) and route 010's own.
//!
//! Route 010's admission (slice (r2c)) calls them; nothing here admits,
//! renders or writes anything, and nothing here records a user release. The
//! detector finds the marks library applies have written into a model; it
//! cannot find a library origin a user has erased, and it does not show that
//! any value is the user's own (the reliance limit, §2.7).
//! Standard claim fence applies (claims taxonomy per DEC-081).

#![cfg_attr(not(test), allow(dead_code))]

use crate::analysis_runs;
use crate::command_ingress::{text_parse_bytes, ArgsDecoding};
use crate::export_disposition_screen::{library_ref_identity, library_ref_member_sites, rule_pack_ref_member_sites};
use crate::stored_read::ReadMemory;
use serde_json::{json, Value};
use std::ops::ControlFlow;

/// The model carries a detectable library reference (§2.7).
pub(crate) const LIBRARY_REFERENCED: &str = "REPORT-USER-RELEASE-LIBRARY-REFERENCED";
/// The model or its input manifest carries a rule-pack reference (§2.7).
pub(crate) const RULE_PACK_REFERENCED: &str = "REPORT-USER-RELEASE-RULE-PACK-REFERENCED";
/// An object carries a non-project private class (§2.7).
pub(crate) const PRIVATE_CLASS: &str = "REPORT-USER-RELEASE-PRIVATE-CLASS";
/// The run record fails a Record Run binding check or one of route 010's
/// (§2.5 step 8).
pub(crate) const ANALYSIS_RECORD_UNBOUND: &str = "REPORT-USER-RELEASE-ANALYSIS-RECORD-UNBOUND";

/// Rule 2's library provenance prefixes (`libraryProvenanceString`,
/// `libraryApply.ts`: `library:<kind>:<library id>#<record id>`).
pub(crate) const LIBRARY_PROVENANCE_PREFIXES: [&str; 4] = ["library:material:", "library:section:", "library:component:", "library:hanger:"];
/// Rule 3 (a): the legacy hanger selection's `kind`.
pub(crate) const LEGACY_HANGER_SELECTION_KIND: &str = "user_imported_hanger_selection";
/// Rule 3 (b): a member of the legacy hanger selection evidence.
pub(crate) const LEGACY_HANGER_MEMBERS: [&str; 4] = ["library_kind", "library_id", "library_metadata", "record_snapshot"];
/// The prescan's findings for an embedded candidate that cannot be checked
/// (`ingress::prescan`, `command_ingress::text_parse_bytes`): refused under
/// rule 3. Every other finding is one serde_json's parse refuses too, so its
/// string is text only.
const UNCHECKABLE_FINDINGS: [&str; 4] =
    ["nesting deeper than 127 containers", "reserved RawValue token key", "input too large to price", "the priced parse overflows"];
/// The non-project private classes (§2.7).
pub(crate) const NON_PROJECT_PRIVATE_CLASSES: [&str; 7] = [
    "private_material_data",
    "private_component_data",
    "private_rule_pack_data",
    "owner_standard_data",
    "company_design_basis_data",
    "path_data",
    "secret_like_data",
];
/// The members whose value is an object's own privacy class.
const PRIVACY_MEMBERS: [&str; 2] = ["privacy_classification", "privacy_class"];
/// A shown identity is cut to this many characters.
const SHOWN_CHARS: usize = 160;

/// Which library rule found a site (§2.7, in its order).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LibraryRule {
    /// Rule 1: a `library_ref` member, whatever its value.
    LibraryRef,
    /// Rule 2: a string holding a library provenance reference.
    ProvenanceString,
    /// Rule 3: a string holding legacy hanger selection evidence.
    LegacyHangerSelection,
}

impl LibraryRule {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            LibraryRule::LibraryRef => "library_ref",
            LibraryRule::ProvenanceString => "provenance_string",
            LibraryRule::LegacyHangerSelection => "legacy_hanger_selection",
        }
    }
}

/// The first library site of a model: its JSON pointer, the rule, and the
/// library kind and id where the site states them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LibrarySite {
    pub(crate) pointer: String,
    pub(crate) rule: LibraryRule,
    pub(crate) kind: Option<String>,
    pub(crate) id: Option<String>,
}

impl LibrarySite {
    /// The refusal: the code, the site's pointer, the rule, and the kind and
    /// id where the site states them. It never quotes the site's text.
    pub(crate) fn refusal(&self) -> String {
        let library = match (&self.kind, &self.id) {
            (Some(kind), Some(id)) => format!(", library {} {}", shown(kind), shown(id)),
            (Some(kind), None) => format!(", library kind {}", shown(kind)),
            (None, Some(id)) => format!(", library {}", shown(id)),
            (None, None) => String::new(),
        };
        format!(
            "{LIBRARY_REFERENCED}: {} holds a library origin (rule {}{library}). A shareable report package does not yet carry library record values, so it cannot be saved for this model; its local private package is unaffected.",
            self.pointer,
            self.rule.as_str()
        )
    }
}

fn shown(text: &str) -> String {
    if text.chars().count() > SHOWN_CHARS {
        text.chars().take(SHOWN_CHARS).chain(std::iter::once('\u{2026}')).collect()
    } else {
        text.to_string()
    }
}

/// One step of a path inside a value.
#[derive(Clone, Copy)]
enum Step<'a> {
    Key(&'a str),
    Index(usize),
}

fn pointer_of(path: &[Step<'_>]) -> String {
    let mut pointer = String::new();
    for step in path {
        pointer.push('/');
        match step {
            Step::Key(key) => pointer.push_str(&key.replace('~', "~0").replace('/', "~1")),
            Step::Index(index) => pointer.push_str(&index.to_string()),
        }
    }
    pointer
}

/// Visits every string value at any depth (object member values and array
/// elements; keys are not tested), in the one walk's order: arrays in order,
/// object members in code-point key order. It builds no path text; `path`
/// grows by one step per level.
fn visit_strings<'a>(value: &'a Value, path: &mut Vec<Step<'a>>, visit: &mut dyn FnMut(&'a str, &[Step<'a>]) -> ControlFlow<()>) -> ControlFlow<()> {
    match value {
        Value::String(text) => visit(text, path),
        Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                path.push(Step::Index(index));
                let flow = visit_strings(item, path, visit);
                path.pop();
                flow?;
            }
            ControlFlow::Continue(())
        }
        Value::Object(object) => {
            for (key, child) in object {
                path.push(Step::Key(key));
                let flow = visit_strings(child, path, visit);
                path.pop();
                flow?;
            }
            ControlFlow::Continue(())
        }
        _ => ControlFlow::Continue(()),
    }
}

/// Rule 2 within one string: the earliest library provenance reference, with
/// its kind and the library id after the prefix (up to `#`, `;` or white
/// space; `None` when empty).
fn provenance_reference(text: &str) -> Option<(&'static str, Option<&str>)> {
    let (at, prefix) = LIBRARY_PROVENANCE_PREFIXES.iter().filter_map(|prefix| text.find(prefix).map(|at| (at, *prefix))).min_by_key(|(at, _)| *at)?;
    let kind = &prefix["library:".len()..prefix.len() - 1];
    let rest = &text[at + prefix.len()..];
    let id = rest.split(|c: char| c == '#' || c == ';' || c.is_whitespace()).next().filter(|id| !id.is_empty());
    Some((kind, id))
}

/// A string is an embedded-JSON candidate when, after leading JSON white
/// space (space, tab, line feed, carriage return), it begins with `{`.
fn is_candidate(text: &str) -> bool {
    text.trim_start_matches([' ', '\t', '\n', '\r']).starts_with('{')
}

/// What rule 3 (b) makes of one candidate before any parse.
enum Candidate {
    /// The prescan admits it; its priced parse.
    Checkable(u64),
    /// The prescan refuses it for a reason serde_json's parse would not:
    /// refused under rule 3.
    Uncheckable,
    /// The prescan refuses it as JSON the parse refuses too: text only.
    TextOnly,
}

fn candidate(text: &str) -> Candidate {
    match text_parse_bytes(text, ArgsDecoding::Moved) {
        Ok(bytes) => Candidate::Checkable(bytes),
        Err(finding) if UNCHECKABLE_FINDINGS.contains(&finding.reason) => Candidate::Uncheckable,
        Err(_) => Candidate::TextOnly,
    }
}

/// Rule 3 (b) within one parsed embedded object: a legacy member, or rule 1,
/// 2 or 3 (a) within the parsed value (strings inside are tested by the
/// substring rules, not parsed again).
fn embedded_holds_legacy(parsed: &Value) -> bool {
    let Value::Object(object) = parsed else { return false };
    if LEGACY_HANGER_MEMBERS.iter().any(|member| object.contains_key(*member)) {
        return true;
    }
    if !library_ref_member_sites(parsed).is_empty() {
        return true;
    }
    visit_strings(parsed, &mut Vec::new(), &mut |text, _| {
        if text.contains(LEGACY_HANGER_SELECTION_KIND) || provenance_reference(text).is_some() {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    })
    .is_break()
}

/// The first library site of the bound model `M` (§2.7): rule 1, then rule 2,
/// then rule 3, each in document order. Rule 3 (b) first prescans every
/// candidate (allocating nothing) and calls `grow` once with the largest
/// checkable candidate's priced parse (`text_parse_bytes(·, Moved)`), the
/// command's ingress-lease growth, before any embedded parse; it then parses
/// the candidates one at a time, dropping each before the next, so at most
/// one embedded value is held beside `M`. `grow`'s refusal is returned as it
/// is. No growth happens when rule 1 or 2 finds a site, or when no candidate
/// is checkable.
pub(crate) fn library_reference_site(model: &Value, grow: &mut dyn FnMut(u64) -> Result<(), String>) -> Result<Option<LibrarySite>, String> {
    // Rule 1: every member named `library_ref`, whatever its value.
    if let Some(member) = library_ref_member_sites(model).into_iter().next() {
        let identity = member.reference.and_then(library_ref_identity);
        return Ok(Some(LibrarySite {
            pointer: member.pointer,
            rule: LibraryRule::LibraryRef,
            kind: identity.map(|(kind, _, _)| kind.to_string()),
            id: identity.map(|(_, id, _)| id.to_string()),
        }));
    }
    // Rule 2: a library provenance reference in any string.
    let mut found = None;
    let _ = visit_strings(model, &mut Vec::new(), &mut |text, path| match provenance_reference(text) {
        Some((kind, id)) => {
            found = Some(LibrarySite { pointer: pointer_of(path), rule: LibraryRule::ProvenanceString, kind: Some(kind.to_string()), id: id.map(str::to_string) });
            ControlFlow::Break(())
        }
        None => ControlFlow::Continue(()),
    });
    if found.is_some() {
        return Ok(found);
    }
    // Rule 3 (b)'s pricing: every candidate is prescanned before any is parsed.
    let mut largest: Option<u64> = None;
    let _ = visit_strings(model, &mut Vec::new(), &mut |text, _| {
        if is_candidate(text) {
            if let Candidate::Checkable(bytes) = candidate(text) {
                largest = Some(largest.map_or(bytes, |held| held.max(bytes)));
            }
        }
        ControlFlow::Continue(())
    });
    if let Some(bytes) = largest {
        grow(bytes)?;
    }
    // Rule 3, in document order.
    let legacy = |path: &[Step<'_>], id: Option<String>| LibrarySite {
        pointer: pointer_of(path),
        rule: LibraryRule::LegacyHangerSelection,
        kind: Some("hanger".to_string()),
        id,
    };
    let _ = visit_strings(model, &mut Vec::new(), &mut |text, path| {
        if text.contains(LEGACY_HANGER_SELECTION_KIND) {
            found = Some(legacy(path, None));
            return ControlFlow::Break(());
        }
        if !is_candidate(text) {
            return ControlFlow::Continue(());
        }
        match candidate(text) {
            Candidate::Uncheckable => {
                found = Some(legacy(path, None));
                ControlFlow::Break(())
            }
            Candidate::TextOnly => ControlFlow::Continue(()),
            Candidate::Checkable(_) => {
                // One embedded value at a time, dropped before the next.
                let Ok(parsed) = serde_json::from_str::<Value>(text) else { return ControlFlow::Continue(()) };
                if embedded_holds_legacy(&parsed) {
                    let id = parsed.get("library_id").and_then(Value::as_str).map(str::to_string);
                    found = Some(legacy(path, id));
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            }
        }
    });
    Ok(found)
}

/// Route 010's step 5, library half: `REPORT-USER-RELEASE-LIBRARY-REFERENCED`
/// naming the first site ([`library_reference_site`]).
pub(crate) fn refuse_library_referenced(model: &Value, grow: &mut dyn FnMut(u64) -> Result<(), String>) -> Result<(), String> {
    match library_reference_site(model, grow)? {
        Some(site) => Err(site.refusal()),
        None => Ok(()),
    }
}

/// Route 010's step 5, rule-pack half (§2.7): the model is
/// rule-pack-referenced when the one walk finds any `rule_pack_refs[]` entry
/// (or a `rule_pack_refs` member that is not a list), or when the input
/// manifest records any active rule pack (or its `active_rule_packs` is not a
/// list).
pub(crate) fn refuse_rule_pack_referenced(model: &Value, input_manifest: &Value) -> Result<(), String> {
    let refusal = |where_: &str| {
        format!(
            "{RULE_PACK_REFERENCED}: {where_}. A shareable report package does not yet carry rule-pack references or rule checks, so it cannot be saved for this model; its local private package is unaffected."
        )
    };
    if let Some(pointer) = rule_pack_ref_member_sites(model).into_iter().next() {
        return Err(refusal(&format!("the model's {pointer} is a rule-pack reference")));
    }
    match input_manifest.get("active_rule_packs") {
        Some(Value::Array(packs)) if packs.is_empty() => Ok(()),
        Some(Value::Array(_)) => Err(refusal("the run's input manifest records an active rule pack (/active_rule_packs/0)")),
        _ => Err(refusal("the run's input manifest has no active_rule_packs list")),
    }
}

/// Route 010's step 6 (§2.7, defence): any object whose own
/// `privacy_classification` or `privacy_class` is a non-project private class
/// is refused, naming the member's pointer under `root` (for example
/// `/request` or `/model`). A label can add a refusal; no label grants
/// inclusion.
pub(crate) fn refuse_private_classes(value: &Value, root: &str) -> Result<(), String> {
    fn walk<'a>(value: &'a Value, path: &mut Vec<Step<'a>>) -> Option<(String, &'a str)> {
        match value {
            Value::Array(items) => items.iter().enumerate().find_map(|(index, item)| {
                path.push(Step::Index(index));
                let found = walk(item, path);
                path.pop();
                found
            }),
            Value::Object(object) => object.iter().find_map(|(key, child)| {
                path.push(Step::Key(key));
                let own = PRIVACY_MEMBERS
                    .contains(&key.as_str())
                    .then(|| child.as_str().filter(|class| NON_PROJECT_PRIVATE_CLASSES.contains(class)))
                    .flatten()
                    .map(|class| (pointer_of(path), class));
                let found = own.or_else(|| walk(child, path));
                path.pop();
                found
            }),
            _ => None,
        }
    }
    match walk(value, &mut Vec::new()) {
        Some((pointer, class)) => Err(format!(
            "{PRIVATE_CLASS}: {root}{pointer} is {class}, a private class that is not the user's project data; a shareable report package cannot carry it."
        )),
        None => Ok(()),
    }
}

/// The analysis statuses a solve's own transient record carries (route 010):
/// `HUMAN_REVIEW_REQUIRED`, the source's mechanics status and its own rule
/// status, each once, sorted (`buildAnalysisRunV03`, `previewService.ts`).
fn expected_statuses(source: &Value) -> Option<Value> {
    let mechanics = source.pointer("/status/mechanics").and_then(Value::as_str)?;
    let rule = source.pointer("/status/rule_check").and_then(Value::as_str)?;
    let mut statuses: Vec<&str> = vec!["HUMAN_REVIEW_REQUIRED", mechanics, rule];
    statuses.sort_unstable();
    statuses.dedup();
    Some(json!(statuses))
}

fn builder_boundary() -> Value {
    json!({
        "human_review_required": true,
        "software_makes_compliance_claim": false,
        "software_makes_certification_claim": false,
        "software_makes_sealing_claim": false,
        "software_makes_approval_claim": false,
        "software_makes_authentication_claim": false
    })
}

/// Route 010's step 8 (§2.5): Record Run's binding checks
/// (`analysis_runs::check_record_for_values`) of the run record `R` against
/// the source `S`, the input manifest `I` and its bound SHA-256, then route
/// 010's own: the record's analysis status is exactly `HUMAN_REVIEW_REQUIRED`,
/// `S`'s mechanics status and `S`'s own rule status (no rule-check
/// aggregate), and its run name, unresolved TBDs and professional boundary are
/// the builder's (`analysisRunCompatibility.ts`). Since step 5 refuses every
/// model with a rule-pack reference, a record made before rule-pack
/// resolution is refused too. Any failure is
/// `REPORT-USER-RELEASE-ANALYSIS-RECORD-UNBOUND`, with Record Run's detail.
pub(crate) fn check_run_record(
    project_id: &str,
    run_record: &Value,
    source: &Value,
    input_manifest: &Value,
    input_manifest_hex: &str,
    memory: &dyn ReadMemory,
) -> Result<(), String> {
    let unbound = |detail: &str| format!("{ANALYSIS_RECORD_UNBOUND}: {detail}");
    let not_resolved = analysis_runs::check_record_for_values(project_id, run_record, source, input_manifest, input_manifest_hex, memory)
        .map_err(|detail| unbound(&detail))?;
    if not_resolved {
        return Err(unbound("the run was recorded before rule-pack resolution"));
    }
    let run = &run_record["analysis_run"];
    if expected_statuses(source).as_ref() != run.get("analysis_status") {
        return Err(unbound(
            "the run record's analysis status is not the solve's own (HUMAN_REVIEW_REQUIRED, its mechanics status and its own rule status, with no rule-check aggregate)",
        ));
    }
    let run_id = run.get("run_id").and_then(Value::as_str).unwrap_or_default();
    let builder_members = [
        ("run_name", "/run_name", json!(format!("{run_id} analysis record"))),
        ("unresolved TBDs", "/reproducibility/unresolved_tbd", json!([])),
        ("professional boundary", "/professional_boundary", builder_boundary()),
    ];
    for (name, pointer, expected) in builder_members {
        if run.pointer(pointer) != Some(&expected) {
            return Err(unbound(&format!("the run record's {name} is not the solve builder's")));
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "user_release_admission_tests.rs"]
mod tests;
