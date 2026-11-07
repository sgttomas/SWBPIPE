//! The bounded import review (DEL-17-04 Design `desktop-mbf-export-import.md` §4.3; slice 5): what the desktop's MBF
//! import worker (task 5) returns in place of the importer's own report, model and id map.
//!
//! [`import_review`] reads the importer's result over its whole extent, then serializes it within an allowance (the
//! desktop uses [`crate::MBF_REVIEW_MAX_BYTES`], R = 1 MiB, counted after escaping), with the export review's mechanics
//! (`export_review.rs`: the same escaping, per-field bound and in-place `{prefix, bytes, sha256}` form):
//!
//! - **Status first.** The importer's status, the document check, the assumptions' completeness and every count are
//!   computed over the full result before any bound applies. Overflow never makes a refused import adoptable.
//! - **Fixed leading members**, byte for byte, so the host checks them without parsing:
//!   `{"document_kind":"swbpipe.mbf_import_review","format":1,"status":"<imported|refused>","adoptable":<true|false>,
//!   "model_sha256":<"hex"|null>,"model_bytes":<n>,`.
//! - **Lists**, in order: `refusals` (each error diagnostic: code, line, entity; they block), `document_check` (the
//!   worker's blocking document-check findings: code, path; they block), `assumptions` (every applied mapping
//!   assumption as its stable identifier and structured parameters, REQ-015), `declaration_needed` (the MBF material
//!   names refused `MBF-IMPORT-MATERIAL-PROVENANCE-REQUIRED`), `loss_entries` (code, entity, category) and
//!   `target_options` (each name, recorded, not applied). Entries are listed while they fit; the first that does not,
//!   and every later one, is counted, not listed, and the review is `truncated`, which blocks adoption
//!   ([`IMPORT_REVIEW_TRUNCATED`]).
//! - **Assumptions.** Each entry of the importer's `assumption_facts` pairs with one of its assumption strings. A count
//!   mismatch, an identifier outside [`crate::ASSUMPTION_IDS`] or the importer's `unavailable` marker blocks adoption
//!   ([`ASSUMPTION_UNAVAILABLE`]); the strings themselves are never carried.
//! - **Not carried.** No engine message text, no assumption sentence, no model value, no token, epoch, time or path.
//!   The same result, source, declaration digest, document check and transfer decision give byte-identical text, so the
//!   review's SHA-256 binds the source digests, the declaration, the model digest and everything the user saw.
//!
//! The function reads the result without recursion.

use crate::export_review::{push_bool, push_field, push_json_str, push_optional_field, push_u64, Count, ReviewAllowanceTooSmall, Sink, REVIEW_FIELD_MAX_BYTES};
use crate::value::Value;
use crate::{ImportResult, ASSUMPTION_IDS};

/// The review's `document_kind`.
pub const IMPORT_REVIEW_DOCUMENT_KIND: &str = "swbpipe.mbf_import_review";
/// The review's `format`.
pub const IMPORT_REVIEW_FORMAT: u64 = 1;
/// A truncated review blocks adoption (§4.3).
pub const IMPORT_REVIEW_TRUNCATED: &str = "MBF-IMPORT-REVIEW-TRUNCATED";
/// An applied mapping assumption lacks its bounded source-safe representation (§4.3).
pub const ASSUMPTION_UNAVAILABLE: &str = "MBF-IMPORT-ASSUMPTION-UNAVAILABLE";
/// The adopt result or reply would exceed the transfer limit (§4.8.4).
pub const MODEL_TOO_LARGE: &str = "MBF-IMPORT-MODEL-TOO-LARGE";
/// The importer refused, or the document check blocked (§4.6).
pub const NOT_ADOPTABLE: &str = "MBF-IMPORT-NOT-ADOPTABLE";

/// The provenance source name the importer gives a material whose origin a bound identity file recorded
/// (`importer/assemble.rs` `material_qualifies`); the review counts such materials.
const RECORDED_ORIGIN_SOURCE: &str = "material origin recorded by the SWBPIPE MBF writer sidecar";

/// The identity file beside the MBF, as the host read it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReviewedIdentityFile<'a> {
    Absent,
    Present { sha256: &'a str, bytes: u64 },
}

/// The chosen files the review states.
#[derive(Clone, Copy, Debug)]
pub struct ImportReviewSource<'a> {
    /// The file name (the import's `source_label`).
    pub file_name: &'a str,
    pub mbf_sha256: &'a str,
    pub mbf_bytes: u64,
    pub identity_file: ReviewedIdentityFile<'a>,
}

/// The worker's document check of the candidate model (kind, version, W-13 validator).
#[derive(Clone, Copy, Debug)]
pub enum ReviewDocumentCheck<'a> {
    /// No candidate model (a refused import).
    NotRun,
    /// Opened with this schema status.
    Passed { schema_status: &'a str },
    /// Blocking findings, `(code, path)`.
    Blocked(&'a [(String, String)]),
}

/// The candidate model: the canonical bytes' SHA-256 (lowercase hex) and length.
#[derive(Clone, Copy, Debug)]
pub struct ReviewModel<'a> {
    pub sha256: &'a str,
    pub bytes: u64,
}

/// The review's lists, in the order they are listed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum List {
    Refusals,
    DocumentCheck,
    Assumptions,
    DeclarationNeeded,
    LossEntries,
    TargetOptions,
}

const LISTS: [List; 6] = [List::Refusals, List::DocumentCheck, List::Assumptions, List::DeclarationNeeded, List::LossEntries, List::TargetOptions];

impl List {
    fn name(self) -> &'static str {
        match self {
            List::Refusals => "refusals",
            List::DocumentCheck => "document_check",
            List::Assumptions => "assumptions",
            List::DeclarationNeeded => "declaration_needed",
            List::LossEntries => "loss_entries",
            List::TargetOptions => "target_options",
        }
    }

    /// Whether an entry of the list not shown blocks adoption on its own (truncation blocks in any case).
    fn blocks(self) -> bool {
        matches!(self, List::Refusals | List::DocumentCheck | List::Assumptions)
    }
}

/// What the review states, for the caller's decisions (the text carries the same).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportReviewSummary {
    pub imported: bool,
    pub document_passed: bool,
    pub assumptions_available: bool,
    pub complete: bool,
    pub transfer_within: bool,
    pub adoptable: bool,
    /// Per list (refusals, document_check, assumptions, declaration_needed, loss_entries, target_options): its
    /// entries, and how many are listed.
    pub totals: [usize; 6],
    pub listed: [usize; 6],
}

impl ImportReviewSummary {
    /// The review's blocking codes, in a fixed order (empty when adoptable).
    pub fn blocking_codes(&self) -> Vec<&'static str> {
        let mut codes = Vec::new();
        if !self.imported || !self.document_passed {
            codes.push(NOT_ADOPTABLE);
        }
        if !self.complete {
            codes.push(IMPORT_REVIEW_TRUNCATED);
        }
        if !self.assumptions_available {
            codes.push(ASSUMPTION_UNAVAILABLE);
        }
        if !self.transfer_within {
            codes.push(MODEL_TOO_LARGE);
        }
        codes
    }
}

/// A formed review: its text (at most the allowance) and its summary.
#[derive(Clone, Debug)]
pub struct ImportReview {
    pub text: String,
    pub summary: ImportReviewSummary,
}

// -- reading the result ------------------------------------------------------------------------------------------------

fn text<'v>(item: &'v Value, key: &str) -> &'v str {
    item.get_or_null(key).as_str().unwrap_or("")
}

/// A list entry, borrowed from the result.
enum Entry<'r> {
    Refusal { code: &'r str, line: u64, entity: &'r str },
    Finding { code: &'r str, path: &'r str },
    Assumption(&'r crate::AssumptionFact),
    Material(&'r str),
    Loss { code: &'r str, entity: &'r str, category: &'r str },
    TargetOption(&'r str),
}

fn line_of(item: &Value) -> u64 {
    crate::value::small_int(item.get_or_null("line")).and_then(|line| u64::try_from(line).ok()).unwrap_or(0)
}

fn entries<'r>(result: &'r ImportResult, check: ReviewDocumentCheck<'r>, list: List) -> Box<dyn Iterator<Item = Entry<'r>> + 'r> {
    let errors = || result.diagnostics.iter().filter(|item| text(item, "severity") == "error");
    match list {
        List::Refusals => Box::new(errors().map(|item| Entry::Refusal { code: text(item, "code"), line: line_of(item), entity: text(item, "location") })),
        List::DocumentCheck => match check {
            ReviewDocumentCheck::Blocked(findings) => Box::new(findings.iter().map(|(code, path)| Entry::Finding { code, path })),
            _ => Box::new(std::iter::empty()),
        },
        List::Assumptions => Box::new(result.assumption_facts.iter().map(Entry::Assumption)),
        List::DeclarationNeeded => Box::new(errors().filter(|item| text(item, "code") == "MBF-IMPORT-MATERIAL-PROVENANCE-REQUIRED").map(|item| {
            let location = text(item, "location");
            Entry::Material(location.strip_prefix("MATERIAL ").unwrap_or(location))
        })),
        List::LossEntries => Box::new(
            result.loss_report.iter().map(|item| Entry::Loss { code: text(item, "code"), entity: text(item, "location"), category: text(item, "category") }),
        ),
        List::TargetOptions => Box::new(result.target_options.keys().map(|name| Entry::TargetOption(name.as_str()))),
    }
}

fn push_entry<S: Sink>(out: &mut S, entry: &Entry<'_>) {
    match entry {
        Entry::Refusal { code, line, entity } => {
            out.push("{\"code\":");
            push_field(out, code);
            out.push(",\"line\":");
            push_u64(out, *line);
            out.push(",\"entity\":");
            push_field(out, entity);
            out.push("}");
        }
        Entry::Finding { code, path } => {
            out.push("{\"code\":");
            push_field(out, code);
            out.push(",\"path\":");
            push_field(out, path);
            out.push("}");
        }
        Entry::Assumption(fact) => {
            out.push("{\"id\":");
            push_field(out, fact.id);
            out.push(",\"parameters\":{");
            for (index, (name, value)) in fact.parameters.iter().enumerate() {
                if index > 0 {
                    out.push(",");
                }
                push_json_str(out, name);
                out.push(":");
                push_field(out, value);
            }
            out.push("}}");
        }
        Entry::Material(name) => push_field(out, name),
        Entry::Loss { code, entity, category } => {
            out.push("{\"code\":");
            push_field(out, code);
            out.push(",\"entity\":");
            push_field(out, entity);
            out.push(",\"category\":");
            push_field(out, category);
            out.push("}");
        }
        Entry::TargetOption(name) => {
            out.push("{\"name\":");
            push_field(out, name);
            out.push(",\"state\":\"recorded_not_applied\"}");
        }
    }
}

/// The id-map members the identity summary reads (A-3's identity set), each with its class name.
pub const IDENTITY_MAPS: [(&str, &str); 9] = [
    ("node", "node_map"),
    ("element", "element_map"),
    ("bend", "bend_map"),
    ("support", "support_map"),
    ("load_case", "load_case_map"),
    ("material", "material_map"),
    ("load", "load_map"),
    ("pressure_region", "region_map"),
    ("section", "section_map"),
];

/// The ids an id-map member holds: a dict's values, a list value counting each of its items.
fn identity_count(id_map: &Value, member: &str) -> u64 {
    match id_map.get(member).and_then(Value::as_dict) {
        Some(map) => map.values().map(|value| value.as_list().map_or(1, |items| items.len() as u64)).sum(),
        None => 0,
    }
}

fn binding_reason(result: &ImportResult, source: &ImportReviewSource<'_>) -> (&'static str, &'static str) {
    let binding = text(&result.source, "binding");
    match binding {
        "bound" => ("bound", "identity_file_digest_matched"),
        "rebound_unconfirmed" => ("rebound_unconfirmed", "rebound_by_node_number"),
        _ if source.identity_file == ReviewedIdentityFile::Absent => ("unbound", "no_identity_file"),
        _ if result.diagnostics.iter().any(|item| text(item, "code") == "MBF-IMPORT-SIDECAR-NOT-APPLIED") => ("unbound", "identity_file_for_other_bytes"),
        _ if result.status != "imported" => ("unbound", "identity_file_refused_or_import_refused"),
        _ => ("unbound", "identity_file_not_applied"),
    }
}

fn recorded_origins(result: &ImportResult) -> u64 {
    let Some(materials) = result.model.as_ref().and_then(|model| model.get("materials")).and_then(Value::as_list) else { return 0 };
    materials.iter().filter(|material| text(material.get_or_null("provenance_record"), "source_name") == RECORDED_ORIGIN_SOURCE).count() as u64
}

// -- the review --------------------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct Header<'r> {
    result: &'r ImportResult,
    source: ImportReviewSource<'r>,
    declaration_sha256: Option<&'r str>,
    model: Option<ReviewModel<'r>>,
    check: ReviewDocumentCheck<'r>,
    adoptable: bool,
    complete: bool,
    transfer_within: bool,
    assumptions_available: bool,
    totals: [usize; 6],
    listed: [usize; 6],
}

impl Header<'_> {
    fn push<S: Sink>(&self, out: &mut S) {
        // The fixed leading members (§4.3), in this order and byte form.
        out.push("{\"document_kind\":\"");
        out.push(IMPORT_REVIEW_DOCUMENT_KIND);
        out.push("\",\"format\":");
        push_u64(out, IMPORT_REVIEW_FORMAT);
        out.push(",\"status\":\"");
        out.push(if self.result.status == "imported" { "imported" } else { "refused" });
        out.push("\",\"adoptable\":");
        push_bool(out, self.adoptable);
        out.push(",\"model_sha256\":");
        match self.model {
            Some(model) => {
                out.push("\"");
                out.push(model.sha256);
                out.push("\"");
            }
            None => out.push("null"),
        }
        out.push(",\"model_bytes\":");
        push_u64(out, self.model.map_or(0, |model| model.bytes));
        // The rest of the fixed part.
        out.push(",\"completeness\":\"");
        out.push(if self.complete { "complete" } else { "truncated" });
        // `within` and `beyond` have the same width.
        out.push("\",\"transfer_limit\":\"");
        out.push(if self.transfer_within { "within" } else { "beyond" });
        out.push("\",\"blocks\":{\"import_refused\":");
        push_bool(out, self.result.status != "imported");
        out.push(",\"document_check\":");
        push_bool(out, matches!(self.check, ReviewDocumentCheck::Blocked(_)));
        out.push(",\"truncated\":");
        push_bool(out, !self.complete);
        out.push(",\"model_too_large\":");
        push_bool(out, !self.transfer_within);
        out.push(",\"assumption_unavailable\":");
        push_bool(out, !self.assumptions_available);
        out.push("},\"field_max_bytes\":");
        push_u64(out, REVIEW_FIELD_MAX_BYTES as u64);
        out.push(",\"source\":{\"file_name\":");
        push_field(out, self.source.file_name);
        out.push(",\"mbf_sha256\":");
        push_field(out, self.source.mbf_sha256);
        out.push(",\"mbf_bytes\":");
        push_u64(out, self.source.mbf_bytes);
        out.push(",\"identity_file\":");
        match self.source.identity_file {
            ReviewedIdentityFile::Absent => out.push("{\"state\":\"absent\"}"),
            ReviewedIdentityFile::Present { sha256, bytes } => {
                out.push("{\"state\":\"present\",\"sha256\":");
                push_field(out, sha256);
                out.push(",\"bytes\":");
                push_u64(out, bytes);
                out.push("}");
            }
        }
        let (binding, reason) = binding_reason(self.result, &self.source);
        out.push("},\"binding\":{\"state\":\"");
        out.push(binding);
        out.push("\",\"reason\":\"");
        out.push(reason);
        out.push("\"},\"declaration\":{\"sha256\":");
        push_optional_field(out, self.declaration_sha256);
        out.push(",\"recorded_origins\":");
        push_u64(out, recorded_origins(self.result));
        out.push("},\"units\":{\"unit_mode\":");
        push_field(out, text(&self.result.source, "unit_mode"));
        out.push(",\"angles\":");
        push_field(out, text(&self.result.source, "angles"));
        out.push(",\"vertical_axis\":");
        push_field(out, text(&self.result.source, "vertical_axis"));
        out.push(",\"project_units\":");
        match self.result.model.as_ref().and_then(|model| model.get_or_null("project").get("units")).and_then(Value::as_dict) {
            Some(units) => {
                out.push("{");
                let mut first = true;
                for (name, value) in units.iter() {
                    let Some(value) = value.as_str() else { continue };
                    if !first {
                        out.push(",");
                    }
                    first = false;
                    push_field(out, name);
                    out.push(":");
                    push_field(out, value);
                }
                out.push("}");
            }
            None => out.push("\"not_available\""),
        }
        out.push("},\"model\":{\"document_check\":\"");
        out.push(match self.check {
            ReviewDocumentCheck::NotRun => "not_run",
            ReviewDocumentCheck::Passed { .. } => "passed",
            ReviewDocumentCheck::Blocked(_) => "blocked",
        });
        out.push("\",\"schema_status\":");
        push_optional_field(out, match self.check {
            ReviewDocumentCheck::Passed { schema_status } => Some(schema_status),
            _ => None,
        });
        let bound = text(&self.result.source, "binding") == "bound";
        out.push("},\"identity\":{\"basis\":\"");
        out.push(if bound { "restored_from_identity_file" } else { "formed_from_mbf_names" });
        out.push("\",\"classes\":[");
        for (index, (class, member)) in IDENTITY_MAPS.iter().enumerate() {
            if index > 0 {
                out.push(",");
            }
            let count = identity_count(&self.result.id_map, member);
            out.push("{\"class\":\"");
            out.push(class);
            out.push("\",\"count\":");
            push_u64(out, count);
            out.push(",\"restored\":");
            push_u64(out, if bound { count } else { 0 });
            out.push(",\"formed_from_mbf_names\":");
            push_u64(out, if bound { 0 } else { count });
            out.push("}");
        }
        out.push("],\"unrecoverable\":");
        push_u64(out, self.result.loss_report.iter().filter(|item| text(item, "code") == "MBF-IMPORT-ID-UNRECOVERABLE").count() as u64);
        let not_shown = |index: usize| (self.totals[index] - self.listed[index]) as u64;
        out.push("},\"counts\":{\"not_shown\":");
        push_u64(out, (0..6).map(not_shown).sum());
        out.push(",\"not_shown_blocking\":");
        push_u64(out, LISTS.iter().enumerate().filter(|(_, list)| list.blocks()).map(|(index, _)| not_shown(index)).sum());
        out.push(",\"lists\":{");
        for (index, list) in LISTS.iter().enumerate() {
            if index > 0 {
                out.push(",");
            }
            out.push("\"");
            out.push(list.name());
            out.push("\":{\"total\":");
            push_u64(out, self.totals[index] as u64);
            out.push(",\"listed\":");
            push_u64(out, self.listed[index] as u64);
            out.push(",\"not_shown\":");
            push_u64(out, not_shown(index));
            out.push(",\"not_shown_blocking\":");
            push_u64(out, if list.blocks() { not_shown(index) } else { 0 });
            out.push("}");
        }
        out.push("}}");
    }
}

/// Room for the counts a header with every list listed in full writes as `0` (each list's two `not_shown` counts,
/// each at most its total, and the overall two, each at most the sum).
fn count_slack(totals: &[usize; 6]) -> usize {
    let digits = |value: usize| value.max(1).ilog10() as usize + 1;
    2 * totals.iter().map(|total| digits(*total)).sum::<usize>() + 2 * digits(totals.iter().sum())
}

/// The length of the lists' framing: `,"<name>":[` and `]` around each list, and the final `}`.
fn framing_len() -> usize {
    LISTS.iter().map(|list| list.name().len() + 6).sum::<usize>() + 1
}

/// Whether every applied assumption has its structured counterpart (§4.3).
fn assumptions_available(result: &ImportResult) -> bool {
    result.assumption_facts.len() == result.assumptions.len() && result.assumption_facts.iter().all(|fact| ASSUMPTION_IDS.contains(&fact.id))
}

/// The bounded review of `result` (§4.3) within `allowance` bytes. `model` is the candidate model's digest and length
/// (None when the importer refused); `document_check` the worker's check of it; `declaration_sha256` the SHA-256 of
/// the declaration bytes the worker was given. `transfer_fits` is given the review's exact length, as it would be if
/// adoptable, and decides `transfer_limit` (called once, and only when there is a candidate model).
pub fn import_review(
    result: &ImportResult,
    source: ImportReviewSource<'_>,
    declaration_sha256: Option<&str>,
    model: Option<ReviewModel<'_>>,
    document_check: ReviewDocumentCheck<'_>,
    allowance: usize,
    transfer_fits: impl FnOnce(u64) -> bool,
) -> Result<ImportReview, ReviewAllowanceTooSmall> {
    // Status first, over the full result.
    let imported = result.status == "imported";
    let document_passed = matches!(document_check, ReviewDocumentCheck::Passed { .. });
    let available = assumptions_available(result);
    let mut totals = [0_usize; 6];
    for (index, list) in LISTS.iter().enumerate() {
        totals[index] = entries(result, document_check, *list).count();
    }
    // The fixed part at its widest: every flag `false`, `truncated`, every list listed in full, plus count slack.
    let widest = Header {
        result,
        source,
        declaration_sha256,
        model,
        check: document_check,
        adoptable: false,
        complete: false,
        transfer_within: true,
        assumptions_available: available,
        totals,
        listed: totals,
    };
    let fixed = {
        let mut count = Count(0);
        widest.push(&mut count);
        count.0 + count_slack(&totals) + framing_len() + 16
    };
    if fixed > allowance {
        return Err(ReviewAllowanceTooSmall { needed: fixed, allowance });
    }
    let budget = allowance - fixed;
    let mut bodies: [String; 6] = Default::default();
    let mut used = 0_usize;
    let mut listed = [0_usize; 6];
    let mut truncated = false;
    'lists: for (index, list) in LISTS.iter().enumerate() {
        for entry in entries(result, document_check, *list) {
            let mut size = Count(usize::from(!bodies[index].is_empty()));
            push_entry(&mut size, &entry);
            if used + size.0 > budget {
                truncated = true;
                break 'lists;
            }
            if !bodies[index].is_empty() {
                bodies[index].push(',');
            }
            push_entry(&mut bodies[index], &entry);
            used += size.0;
            listed[index] += 1;
        }
    }
    let complete = !truncated;
    debug_assert_eq!(complete, listed == totals);
    let otherwise_adoptable = imported && document_passed && available && complete && model.is_some();
    let length_with = |adoptable: bool, transfer_within: bool| {
        let header = Header { adoptable, complete, transfer_within, listed, ..widest };
        let mut count = Count(0);
        header.push(&mut count);
        count.0 + framing_len() + bodies.iter().map(String::len).sum::<usize>()
    };
    let transfer_within = match model {
        Some(_) => transfer_fits(length_with(otherwise_adoptable, true) as u64),
        None => true,
    };
    let adoptable = otherwise_adoptable && transfer_within;
    let header = Header { adoptable, complete, transfer_within, listed, ..widest };
    let length = length_with(adoptable, transfer_within);
    debug_assert!(length <= allowance, "{length} > {allowance}");
    let mut text = String::with_capacity(length);
    header.push(&mut text);
    for (index, list) in LISTS.iter().enumerate() {
        text.push_str(",\"");
        text.push_str(list.name());
        text.push_str("\":[");
        text.push_str(&bodies[index]);
        text.push(']');
        bodies[index] = String::new();
    }
    text.push('}');
    debug_assert_eq!(text.len(), length);
    Ok(ImportReview {
        text,
        summary: ImportReviewSummary { imported, document_passed, assumptions_available: available, complete, transfer_within, adoptable, totals, listed },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_framing_length_is_the_lists_framing() {
        let mut framing = String::new();
        for list in LISTS {
            framing.push_str(",\"");
            framing.push_str(list.name());
            framing.push_str("\":[");
            framing.push(']');
        }
        framing.push('}');
        assert_eq!(framing.len(), framing_len());
    }
}
