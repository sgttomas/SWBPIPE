//! The bounded export review (DEL-17-04 Design `desktop-mbf-export-import.md` §3.3 and §3.9.5; slice 4b.1): what the
//! desktop's MBF worker task returns in place of the writer's own report, in `review` mode and with the files in
//! `save` mode.
//!
//! [`export_review`] classifies the writer's result as §3.3 does (this logic moved here from the dialog's
//! `mbfExport.ts`; the product descriptions keyed by code stay there), over the full result, then serializes it
//! within an allowance (at most [`MBF_REVIEW_MAX_BYTES`], R = 1 MiB, counted after escaping):
//!
//! - **Status first.** The writer's status, the blocking state (`status` is not `written`, or any entry is not
//!   written) and the global refusal (`refused_unexplained`: the writer refused and named nothing that explains it)
//!   are computed over the full result before any bound is applied, and are always carried.
//! - **Entries.** `not_written` (each `unsupported` loss entry, then each error diagnostic with no loss entry of the
//!   same code and entity), then `omitted` (each `omitted` loss entry), then `warnings` (each warning diagnostic with
//!   no `omitted` loss entry of the same code and entity), each in the writer's order, each with its code, entity and
//!   category. A loss entry and the diagnostic of the same code and entity that the writer raised with it are one
//!   entry, so they are listed together or counted together. Then the materials whose identity-file origin is
//!   `undeclared`. Info diagnostics and the writer's messages are not carried: no engine message text, and so no
//!   text about another program, leaves the review.
//! - **Overflow.** The fixed fields (with every count at its widest) are reserved first. Entries are listed in order
//!   while they fit; the first that does not, and every later entry and material, is counted, not listed. The review
//!   is then `truncated`, and carries the count not shown and how many of those block, in total and per list.
//! - **Long strings.** A code, entity, unit, definition, digest, status or material longer than
//!   [`REVIEW_FIELD_MAX_BYTES`] is carried as `{"prefix", "bytes", "sha256"}`: its first bytes up to the bound, cut on
//!   a character boundary, its full length in bytes and its SHA-256. The representation is written from the string in
//!   place: nothing copies or escapes the whole string first, and every entry's size is counted before it is written.
//! - **Output ceiling.** `output_ceiling` is `within` or `beyond` (the same width, so the review's length does not
//!   depend on it), decided by the caller's `output_fits` from the review's exact length: the save result of the
//!   desktop worker (MBF, identity file, this review and the framing) must be at most [`MBF_EXPORT_OUTPUT_MAX_BYTES`].
//!
//! The text is ASCII JSON: every non-ASCII or control character is a `\u` escape. It is deterministic: the same
//! result, digests, allowance and ceiling decision give the same bytes. The function reads the result without
//! recursion, so the nesting bound of the crate's walks is not reached.

use crate::value::Value;
use crate::writer::WriterResult;
use sha2::{Digest, Sha256};
use std::collections::HashSet;

/// The review's `document_kind`.
pub const REVIEW_DOCUMENT_KIND: &str = "swbpipe.mbf_export_review";
/// The review's `review_format`.
pub const REVIEW_FORMAT: u64 = 1;
/// R: the most bytes of a serialized review, after escaping (Agent 0, 2026-11-02).
pub const MBF_REVIEW_MAX_BYTES: usize = 1 << 20;
/// The most bytes of a desktop MBF save result: MBF, identity file, bounded review and framing (Agent 0,
/// 2026-11-02).
pub const MBF_EXPORT_OUTPUT_MAX_BYTES: u64 = 64 << 20;
/// The per-field bound: a longer string is carried as its prefix (at most this many bytes), its length and its
/// SHA-256. Escaped, a prefix is at most six times as long, so one entry stays well below R.
pub const REVIEW_FIELD_MAX_BYTES: usize = 512;
/// A truncated review blocks the save (§3.3).
pub const REVIEW_TRUNCATED: &str = "MBF-REVIEW-TRUNCATED";
/// A save result above [`MBF_EXPORT_OUTPUT_MAX_BYTES`] is refused (§3.7).
pub const EXPORT_OUTPUT_TOO_LARGE: &str = "MBF-EXPORT-OUTPUT-TOO-LARGE";

/// The review's lists, in the order they are listed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReviewList {
    NotWritten,
    Omitted,
    Warnings,
    UndeclaredMaterials,
}

impl ReviewList {
    pub const ALL: [ReviewList; 4] = [ReviewList::NotWritten, ReviewList::Omitted, ReviewList::Warnings, ReviewList::UndeclaredMaterials];

    pub fn name(self) -> &'static str {
        match self {
            ReviewList::NotWritten => "not_written",
            ReviewList::Omitted => "omitted",
            ReviewList::Warnings => "warnings",
            ReviewList::UndeclaredMaterials => "undeclared_materials",
        }
    }

    /// Whether each entry of the list blocks the save on its own.
    pub fn blocks(self) -> bool {
        self == ReviewList::NotWritten
    }

    fn index(self) -> usize {
        self as usize
    }
}

/// The written files a review states: their digests (lowercase hex SHA-256 of the MBF text and of the identity file's
/// bytes) and lengths. None for a refused result.
#[derive(Clone, Copy, Debug)]
pub struct ReviewFiles<'a> {
    pub mbf_sha256: &'a str,
    pub sidecar_sha256: &'a str,
    pub mbf_bytes: u64,
    pub sidecar_bytes: u64,
}

/// What the review states, for the caller's decisions (the text carries the same).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReviewSummary {
    /// The writer's status is `written` and the files are given.
    pub written: bool,
    /// The writer refused, or an entry is not written.
    pub blocking: bool,
    /// The writer refused and named nothing that explains it.
    pub refused_unexplained: bool,
    /// Every entry and material is listed.
    pub complete: bool,
    pub output_within_ceiling: bool,
    /// Per list ([`ReviewList::ALL`] order): its entries, and how many are listed.
    pub totals: [usize; 4],
    pub listed: [usize; 4],
}

impl ReviewSummary {
    pub fn not_shown(&self) -> usize {
        (0..4).map(|index| self.totals[index] - self.listed[index]).sum()
    }

    pub fn not_shown_blocking(&self) -> usize {
        ReviewList::ALL.iter().filter(|list| list.blocks()).map(|list| self.totals[list.index()] - self.listed[list.index()]).sum()
    }

    /// The save may continue: written, nothing blocks, the review is complete and the result is within the ceiling.
    pub fn save_eligible(&self) -> bool {
        self.written && !self.blocking && self.complete && self.output_within_ceiling
    }
}

/// A formed review: its text (at most the allowance) and its summary.
#[derive(Clone, Debug)]
pub struct Review {
    pub text: String,
    pub summary: ReviewSummary,
}

/// The allowance cannot hold even the review's fixed fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReviewAllowanceTooSmall {
    pub needed: usize,
    pub allowance: usize,
}

// -- serialization ---------------------------------------------------------------------------------------------------

/// Where the serializer writes: a counter (to size a part before writing it) or the text.
trait Sink {
    fn push(&mut self, text: &str);
}

struct Count(usize);

impl Sink for Count {
    fn push(&mut self, text: &str) {
        self.0 += text.len();
    }
}

impl Sink for String {
    fn push(&mut self, text: &str) {
        self.push_str(text);
    }
}

fn push_u64<S: Sink>(out: &mut S, mut value: u64) {
    let mut digits = [0_u8; 20];
    let mut at = digits.len();
    loop {
        at -= 1;
        digits[at] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    out.push(std::str::from_utf8(&digits[at..]).expect("ASCII digits"));
}

fn push_i64<S: Sink>(out: &mut S, value: i64) {
    if value < 0 {
        out.push("-");
    }
    push_u64(out, value.unsigned_abs());
}

fn push_bool<S: Sink>(out: &mut S, value: bool) {
    out.push(if value { "true" } else { "false" });
}

fn push_hex4<S: Sink>(out: &mut S, unit: u32) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let escape = [b'\\', b'u', HEX[(unit >> 12) as usize & 15], HEX[(unit >> 8) as usize & 15], HEX[(unit >> 4) as usize & 15], HEX[unit as usize & 15]];
    out.push(std::str::from_utf8(&escape).expect("ASCII escape"));
}

/// `text` as a JSON string of printable ASCII: `"` and `\` escaped, the usual short escapes, and every other control,
/// DEL or non-ASCII character as `\u` (a surrogate pair beyond the BMP). Written in runs, with no copy of `text`.
fn push_json_str<S: Sink>(out: &mut S, text: &str) {
    out.push("\"");
    let mut start = 0;
    for (at, ch) in text.char_indices() {
        let short = match ch {
            '"' => Some("\\\""),
            '\\' => Some("\\\\"),
            '\n' => Some("\\n"),
            '\r' => Some("\\r"),
            '\t' => Some("\\t"),
            '\u{8}' => Some("\\b"),
            '\u{c}' => Some("\\f"),
            _ => None,
        };
        let plain = short.is_none() && (' '..='~').contains(&ch);
        if plain {
            continue;
        }
        out.push(&text[start..at]);
        match short {
            Some(escape) => out.push(escape),
            None => {
                let mut units = [0_u16; 2];
                for unit in ch.encode_utf16(&mut units) {
                    push_hex4(out, u32::from(*unit));
                }
            }
        }
        start = at + ch.len_utf8();
    }
    out.push(&text[start..]);
    out.push("\"");
}

/// Lowercase hex SHA-256 of `bytes`, on the stack.
fn sha256_hex(bytes: &[u8]) -> [u8; 64] {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let digest = Sha256::digest(bytes);
    let mut out = [0_u8; 64];
    for (index, byte) in digest.iter().enumerate() {
        out[2 * index] = HEX[(byte >> 4) as usize];
        out[2 * index + 1] = HEX[(byte & 15) as usize];
    }
    out
}

/// The largest index at most `bound` that is a character boundary of `text`.
fn char_floor(text: &str, bound: usize) -> usize {
    let mut cut = bound.min(text.len());
    while !text.is_char_boundary(cut) {
        cut -= 1;
    }
    cut
}

/// A string field: as a JSON string when it is at most [`REVIEW_FIELD_MAX_BYTES`], else as
/// `{"prefix":…,"bytes":…,"sha256":…}`, formed from `text` in place.
fn push_field<S: Sink>(out: &mut S, text: &str) {
    if text.len() <= REVIEW_FIELD_MAX_BYTES {
        push_json_str(out, text);
        return;
    }
    out.push("{\"prefix\":");
    push_json_str(out, &text[..char_floor(text, REVIEW_FIELD_MAX_BYTES)]);
    out.push(",\"bytes\":");
    push_u64(out, text.len() as u64);
    out.push(",\"sha256\":\"");
    out.push(std::str::from_utf8(&sha256_hex(text.as_bytes())).expect("ASCII hex"));
    out.push("\"}");
}

fn push_optional_field<S: Sink>(out: &mut S, text: Option<&str>) {
    match text {
        Some(text) => push_field(out, text),
        None => out.push("null"),
    }
}

// -- classification --------------------------------------------------------------------------------------------------

/// One review entry: a loss entry (with the diagnostic raised with it) or a diagnostic without one.
struct Entry<'r> {
    list: ReviewList,
    code: &'r str,
    entity: &'r str,
    category: &'static str,
}

fn text<'v>(item: &'v Value, key: &str) -> &'v str {
    item.get_or_null(key).as_str().unwrap_or("")
}

/// §3.3's classification over the full result, in listing order, without copying any string.
struct Classified<'r> {
    result: &'r WriterResult,
    paired: HashSet<(&'r str, &'r str)>,
    paired_omitted: HashSet<(&'r str, &'r str)>,
}

impl<'r> Classified<'r> {
    fn new(result: &'r WriterResult) -> Self {
        let mut paired = HashSet::new();
        let mut paired_omitted = HashSet::new();
        for item in &result.loss_report {
            let key = (text(item, "code"), text(item, "entity"));
            paired.insert(key);
            if text(item, "category") == "omitted" {
                paired_omitted.insert(key);
            }
        }
        Classified { result, paired, paired_omitted }
    }

    fn entries(&self) -> impl Iterator<Item = Entry<'r>> + '_ {
        let loss = |category: &'static str, list: ReviewList| {
            self.result.loss_report.iter().filter(move |item| text(item, "category") == category).map(move |item| Entry {
                list,
                code: text(item, "code"),
                entity: text(item, "entity"),
                category,
            })
        };
        let unpaired_errors = self.result.diagnostics.iter().filter(|item| text(item, "severity") == "error").filter_map(|item| {
            let key = (text(item, "code"), text(item, "entity"));
            (!self.paired.contains(&key)).then_some(Entry { list: ReviewList::NotWritten, code: key.0, entity: key.1, category: "error" })
        });
        let warnings = self.result.diagnostics.iter().filter(|item| text(item, "severity") == "warning").filter_map(|item| {
            let key = (text(item, "code"), text(item, "entity"));
            (!self.paired_omitted.contains(&key)).then_some(Entry { list: ReviewList::Warnings, code: key.0, entity: key.1, category: "warning" })
        });
        loss("unsupported", ReviewList::NotWritten).chain(unpaired_errors).chain(loss("omitted", ReviewList::Omitted)).chain(warnings)
    }

    fn undeclared_materials(&self) -> impl Iterator<Item = &'r str> + 'r {
        self.result.material_map.values().filter(|entry| text(entry, "origin") == "undeclared").map(|entry| text(entry, "swbpipe_material"))
    }
}

fn push_entry<S: Sink>(out: &mut S, entry: &Entry<'_>) {
    out.push("{\"list\":\"");
    out.push(entry.list.name());
    out.push("\",\"code\":");
    push_field(out, entry.code);
    out.push(",\"entity\":");
    push_field(out, entry.entity);
    out.push(",\"category\":\"");
    out.push(entry.category);
    out.push("\"}");
}

// -- the review ------------------------------------------------------------------------------------------------------

/// The fixed fields, before the entries.
#[derive(Clone, Copy)]
struct Header<'r> {
    result: &'r WriterResult,
    files: Option<ReviewFiles<'r>>,
    blocking: bool,
    refused_unexplained: bool,
    complete: bool,
    output_within_ceiling: bool,
    totals: [usize; 4],
    listed: [usize; 4],
    conversions: &'r [(String, String, Option<String>)],
}

impl Header<'_> {
    fn push<S: Sink>(&self, out: &mut S) {
        out.push("{\"document_kind\":\"");
        out.push(REVIEW_DOCUMENT_KIND);
        out.push("\",\"review_format\":");
        push_u64(out, REVIEW_FORMAT);
        out.push(",\"status\":");
        push_field(out, &self.result.status);
        out.push(",\"blocking\":");
        push_bool(out, self.blocking);
        out.push(",\"refused_unexplained\":");
        push_bool(out, self.refused_unexplained);
        out.push(",\"completeness\":\"");
        out.push(if self.complete { "complete" } else { "truncated" });
        // `within` and `beyond` have the same width.
        out.push("\",\"output_ceiling\":\"");
        out.push(if self.output_within_ceiling { "within" } else { "beyond" });
        out.push("\",\"mbf_sha256\":");
        push_optional_field(out, self.files.map(|files| files.mbf_sha256));
        out.push(",\"sidecar_sha256\":");
        push_optional_field(out, self.files.map(|files| files.sidecar_sha256));
        out.push(",\"mbf_bytes\":");
        push_u64(out, self.files.map_or(0, |files| files.mbf_bytes));
        out.push(",\"sidecar_bytes\":");
        push_u64(out, self.files.map_or(0, |files| files.sidecar_bytes));
        out.push(",\"layout_row_count\":");
        push_i64(out, self.result.layout_row_count);
        out.push(",\"field_max_bytes\":");
        push_u64(out, REVIEW_FIELD_MAX_BYTES as u64);
        let not_shown = |list: ReviewList| (self.totals[list.index()] - self.listed[list.index()]) as u64;
        out.push(",\"not_shown\":");
        push_u64(out, ReviewList::ALL.iter().map(|list| not_shown(*list)).sum());
        out.push(",\"not_shown_blocking\":");
        push_u64(out, ReviewList::ALL.iter().filter(|list| list.blocks()).map(|list| not_shown(*list)).sum());
        out.push(",\"lists\":{");
        for (index, list) in ReviewList::ALL.iter().enumerate() {
            if index > 0 {
                out.push(",");
            }
            out.push("\"");
            out.push(list.name());
            out.push("\":{\"total\":");
            push_u64(out, self.totals[list.index()] as u64);
            out.push(",\"listed\":");
            push_u64(out, self.listed[list.index()] as u64);
            out.push(",\"not_shown\":");
            push_u64(out, not_shown(*list));
            out.push(",\"not_shown_blocking\":");
            push_u64(out, if list.blocks() { not_shown(*list) } else { 0 });
            out.push("}");
        }
        out.push("},\"unit_conversions\":[");
        for (index, (unit, definition, factor)) in self.conversions.iter().enumerate() {
            if index > 0 {
                out.push(",");
            }
            out.push("{\"unit\":");
            push_field(out, unit);
            out.push(",\"definition\":");
            push_field(out, definition);
            out.push(",\"factor\":");
            push_optional_field(out, factor.as_deref());
            out.push("}");
        }
        out.push("],\"entries\":[");
    }
}

const BETWEEN_LISTS: &str = "],\"undeclared_materials\":[";
const TAIL: &str = "]}";

/// The units the writer normalized, each with its exact definition and, where NORMALIZATION has it, its exact factor
/// as `numerator/denominator` (the writer's normalization data, not its assumption sentences).
fn unit_conversions(result: &WriterResult) -> Vec<(String, String, Option<String>)> {
    if result.unit_normalizations.is_empty() {
        return Vec::new();
    }
    let factors = crate::writer::unit_factors();
    result
        .unit_normalizations
        .iter()
        .map(|(unit, definition)| {
            let factor = factors
                .iter()
                .find(|(table, key, _)| *table == "normalization" && key == unit)
                .and_then(|(_, _, factor)| factor.as_ref())
                .map(|(numerator, denominator)| format!("{numerator}/{denominator}"));
            (unit.clone(), definition.clone(), factor)
        })
        .collect()
}

/// The bounded review of `result` (§3.9.5) within `allowance` bytes (the desktop uses [`MBF_REVIEW_MAX_BYTES`]).
/// `files` are the written files' digests and lengths (None for a refused result). `output_fits` is given the
/// review's exact length and decides `output_ceiling`; it is called once, before the text is built.
pub fn export_review(
    result: &WriterResult,
    files: Option<ReviewFiles<'_>>,
    allowance: usize,
    output_fits: impl FnOnce(u64) -> bool,
) -> Result<Review, ReviewAllowanceTooSmall> {
    let classified = Classified::new(result);
    let conversions = unit_conversions(result);
    // Status first, over the full result.
    let mut totals = [0_usize; 4];
    for entry in classified.entries() {
        totals[entry.list.index()] += 1;
    }
    totals[ReviewList::UndeclaredMaterials.index()] = classified.undeclared_materials().count();
    let written = result.status == "written" && files.is_some();
    let blocking = result.status != "written" || totals[ReviewList::NotWritten.index()] > 0;
    let refused_unexplained = result.status != "written" && totals[ReviewList::NotWritten.index()] == 0;
    // The fixed fields at their widest: every list listed in full (each `listed` at its total's width), plus room
    // for each `not_shown` count, which is at most its total, and for the two overall sums.
    let widest = Header {
        result,
        files,
        blocking,
        refused_unexplained,
        complete: false,
        output_within_ceiling: true,
        totals,
        listed: totals,
        conversions: &conversions,
    };
    let fixed = {
        let mut count = Count(0);
        widest.push(&mut count);
        count.0 + count_slack(&totals) + BETWEEN_LISTS.len() + TAIL.len()
    };
    if fixed > allowance {
        return Err(ReviewAllowanceTooSmall { needed: fixed, allowance });
    }
    // Entries in order while they fit; the first that does not, and every later one, is counted.
    let budget = allowance - fixed;
    let mut entries_text = String::new();
    let mut materials_text = String::new();
    let mut used = 0_usize;
    let mut listed = [0_usize; 4];
    let mut truncated = false;
    for entry in classified.entries() {
        if truncated {
            break;
        }
        let mut size = Count(usize::from(!entries_text.is_empty()));
        push_entry(&mut size, &entry);
        if used + size.0 > budget {
            truncated = true;
            break;
        }
        if !entries_text.is_empty() {
            entries_text.push(',');
        }
        push_entry(&mut entries_text, &entry);
        used += size.0;
        listed[entry.list.index()] += 1;
    }
    if !truncated {
        for material in classified.undeclared_materials() {
            let mut size = Count(usize::from(!materials_text.is_empty()));
            push_field(&mut size, material);
            if used + size.0 > budget {
                truncated = true;
                break;
            }
            if !materials_text.is_empty() {
                materials_text.push(',');
            }
            push_field(&mut materials_text, material);
            used += size.0;
            listed[ReviewList::UndeclaredMaterials.index()] += 1;
        }
    }
    let complete = !truncated;
    debug_assert_eq!(complete, listed == totals);
    let header = Header { complete, listed, ..widest };
    let mut header_size = Count(0);
    header.push(&mut header_size);
    let length = header_size.0 + entries_text.len() + BETWEEN_LISTS.len() + materials_text.len() + TAIL.len();
    debug_assert!(length <= allowance, "{length} > {allowance}");
    let output_within_ceiling = output_fits(length as u64);
    let header = Header { output_within_ceiling, ..header };
    let mut text = String::with_capacity(length);
    header.push(&mut text);
    text.push_str(&entries_text);
    drop(entries_text);
    text.push_str(BETWEEN_LISTS);
    text.push_str(&materials_text);
    text.push_str(TAIL);
    debug_assert_eq!(text.len(), length);
    Ok(Review {
        text,
        summary: ReviewSummary { written, blocking, refused_unexplained, complete, output_within_ceiling, totals, listed },
    })
}

/// Room for the counts a header written with every list listed in full writes as `0`: each list's `not_shown` and
/// `not_shown_blocking` (each at most the list's total) and the overall two (each at most the sum of the totals).
fn count_slack(totals: &[usize; 4]) -> usize {
    let digits = |value: usize| value.max(1).ilog10() as usize + 1;
    2 * totals.iter().map(|total| digits(*total)).sum::<usize>() + 2 * digits(totals.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strings_are_escaped_to_printable_ascii() {
        let mut out = String::new();
        push_json_str(&mut out, "a\"b\\c\n\u{1}\u{7f}é😀");
        assert_eq!(out, "\"a\\\"b\\\\c\\n\\u0001\\u007f\\u00e9\\ud83d\\ude00\"");
        let mut count = Count(0);
        push_json_str(&mut count, "a\"b\\c\n\u{1}\u{7f}é😀");
        assert_eq!(count.0, out.len());
    }

    #[test]
    fn a_long_field_is_its_prefix_length_and_digest() {
        let long = format!("{}é{}", "a".repeat(REVIEW_FIELD_MAX_BYTES - 1), "b".repeat(100));
        let mut out = String::new();
        push_field(&mut out, &long);
        let expected_prefix = "a".repeat(REVIEW_FIELD_MAX_BYTES - 1);
        let digest = sha256_hex(long.as_bytes());
        assert_eq!(
            out,
            format!("{{\"prefix\":\"{expected_prefix}\",\"bytes\":{},\"sha256\":\"{}\"}}", long.len(), std::str::from_utf8(&digest).unwrap())
        );
        let mut exact = String::new();
        push_field(&mut exact, &"a".repeat(REVIEW_FIELD_MAX_BYTES));
        assert_eq!(exact.len(), REVIEW_FIELD_MAX_BYTES + 2);
    }

    #[test]
    fn numbers_are_decimal() {
        let mut out = String::new();
        push_u64(&mut out, 0);
        out.push(' ');
        push_u64(&mut out, u64::MAX);
        out.push(' ');
        push_i64(&mut out, i64::MIN);
        assert_eq!(out, format!("0 {} {}", u64::MAX, i64::MIN));
    }
}
