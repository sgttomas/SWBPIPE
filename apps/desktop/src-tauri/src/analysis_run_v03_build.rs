//! C1 native custody, slice 5's builder (DEL-04-05 `c1-native-custody-plan.md`
//! slice 5; design `result-volume-design.md` §4.1): the current (0.3.0)
//! analysis-run record of a retained solve result, built in this process
//! from the held bytes, and the record's preview header for the webview.
//! The webview switch (slice 4) reads its preview header for a fresh solve;
//! the native Record Run and Save Project (`native_persistence`, the rest of
//! slice 5) write its stored forms, its recorded revision ([`Revision`])
//! included.
//!
//! - **The same record.** This is `buildAnalysisRunV03` and its caller
//!   `buildAnalysisRunPreview` (`analysisRunCompatibility.ts`,
//!   `previewService.ts`) in Rust, which stay the oracle: the same input
//!   manifest checks and model binding, the same rule-check status, load
//!   basis, rule-pack exclusion diagnostics, row interpretation, checksums,
//!   immutability and professional boundary, and the same refusals in the
//!   same order (below). The record's bytes are its **stored form**: the
//!   text the store writes for a record the webview sends today
//!   (`serde_json::to_string` of the `Value` the command ingress parses from
//!   the webview's `JSON.stringify`, in `save_local_project`'s
//!   `write_project_rows`; Record Run's recorded revision is the same
//!   serializer over that `Value`). So every number is written as
//!   JavaScript's `JSON.stringify` writes its binary64 value (`-0` as `0`, an
//!   integral float as an integer) and then as serde_json writes the number it
//!   reads back, and every object's members are in serde_json's key order.
//!   The checksums are `openpipestress_jcs_ijson_v1` over the same values, as
//!   `canonicalSha256HexCheckedV1` computes them.
//! - **The readers.** The route, its binding and the route's reader are the
//!   Rust readers on the held bytes, as the TypeScript reads them through the
//!   wasm engine (`reader_verdict::route`, `binding_of` and `validate`). The
//!   retained routes' reader reads what `validateRetainedPrecision` hands the
//!   wasm engine: the held value written back with JavaScript's numbers and
//!   its negative zeros kept (`jsonTextKeepingNegativeZero`), after the
//!   checked-profile check of `retainedJsonText`; this is done in place on
//!   the parsed value (numbers only), so it allocates nothing.
//! - **Refusals, in the TypeScript order.** Each refusal is the message the
//!   TypeScript throws, byte for byte. Where the TypeScript builder throws a
//!   JavaScript `TypeError` (a member read on `undefined` or `null`, a
//!   `.map` of a non-array), this refuses at the same point with
//!   [`INPUT_SHAPE_INVALID`] naming the member; the engine's `TypeError`
//!   texts are not part of the builder's vocabulary. One stated difference:
//!   a record nested past serde_json's parse depth is refused with
//!   `CHECKED-JSON-INVALID: recursion limit exceeded` without the column the
//!   wasm engine reports (it counts bytes of the `JSON.stringify` text, whose
//!   member order the held bytes do not fix). The order of the row phase
//!   follows the TypeScript's `Promise.all`: a row the record contract cannot
//!   interpret is refused first, whichever row it is; only then a row outside
//!   the checked profile.
//! - **Bounded passes, no record tree.** The record is never a `Value` and,
//!   for the preview, never text. The held text is parsed once (the parse
//!   lease of M, as slices 1 to 3); everything else reads that tree:
//!   1. the readers (route, binding, the route's reader);
//!   2. the record contract's checks over the rows (identities, bases, row
//!      interpretation, the checked profile), one row at a time;
//!   3. the received result's checksum: the canonical text of the whole
//!      result streamed into SHA-256, never built;
//!   4. the record checksum: the record's canonical projection streamed into
//!      SHA-256, each row's checksum streamed the same way, each result ref
//!      written straight from the borrowed row;
//!   5. the stored length: the stored form written to a byte counter (the
//!      record checksum's 64 digits counted by length);
//!   6. the record validator's checks, the duplicate-id check reading one
//!      16-byte entry (an 8-byte fingerprint and 8-byte row index) per row
//!      from an allocation leased at its exact size; equal hashes are
//!      checked against the actual ids.
//!
//!   The record's bytes, when a persistence consumer asks for them
//!   ([`record_bytes`]), are a seventh pass writing the stored form into a
//!   buffer of the exact length, leased and checked against the caller's
//!   bound before it is allocated; a record over the bound is refused by name
//!   ([`RECORD_TOO_LARGE`]) with nothing allocated, and is never truncated.
//!   The preview header exists whatever the record's size.
//! - **The command** `read_preview_mechanics_job_analysis_run_preview`, raw
//!   args body `{jobId, inputManifest: {manifest_ref, manifest_sha256,
//!   canonical_bytes}, ruleCheckAggregate, rulePackExclusions}` (the
//!   session's input manifest evidence without its parsed copy, which the
//!   canonical bytes fix; the GUI rule-check aggregate or `null`; the model's
//!   references the solve left out of the manifest). The response is the
//!   header's JSON as a raw byte body (Tauri delivers an ArrayBuffer): the
//!   run's identity, both checksums, the statuses and counts, and every record
//!   member except the row-scale ones (`result_refs`, the producer's
//!   `diagnostics`, the copied `contract_evidence`, `source_block_recovery`
//!   and `retained_precision`), which it counts or names. Nothing is cached:
//!   each call reads the held bytes with the arguments it is given.
//! - **Admission and memory** (plan §8; no new budget or rule). Every lease is
//!   pool-admitted before what it covers is allocated, and none exceeds the
//!   per-invocation budget or the 12 GiB default:
//!   - the args lease, as the other C1 commands': the raw body's copy, grown
//!     by the args' admission bound and by the manifest text's priced parse;
//!   - the parse lease of M (`acquire_result_parse`), held while the tree is
//!     read;
//!   - the working lease, before the passes: a fixed allowance plus the
//!     canonical writer's key-order scratch along the deepest path of the
//!     source and manifest (each object's member references, at most one per
//!     nesting level at a time);
//!   - the fingerprint/index lease, 16 bytes per row, at its exact allocation;
//!   - the response lease, the header's exact length, ending when the command
//!     returns; for [`record_bytes`], the record lease at its exact length,
//!     which the caller holds with the bytes.
//! - **Residuals, unpriced** (for slice 7's measurement): the readers' own
//!   working memory (as slices 1 to 3); serde_json's per-number and per-string
//!   scratch in the writers (one number or escape at a time); the load basis,
//!   statuses and rule-pack diagnostics (scaling with the model and arguments,
//!   not enforced within the fixed working allowance); and the returned buffer and
//!   Tauri's delivery copies after return.
use crate::analysis_runs::{
    input_manifest_ref, manifest_excluded_rule_packs, rule_pack_identity, rule_pack_ref_diagnostic, RULE_PACKS_NOT_RESOLVED,
    RULE_PACKS_NOT_RESOLVED_DETAIL, RULE_PACK_REF_CHECKSUM_MISMATCH, RULE_PACK_REF_UNRESOLVED,
};
use crate::command_ingress::{self, ArgsDecoding, CommandError};
use crate::ingress;
use crate::memory_pool::Lease;
use crate::native_verdict;
use crate::solve_jobs::SolveJobRegistry;
use open_pipe_stress_canonical_json::{canonical_number, ecma_number_to_string, CHECKED_PROFILE_V1};
use open_pipe_stress_result_export::{reader_verdict as rv, retained_precision as rp, semantic_contract as sc};
use serde::Deserialize;
use serde_json::{json, Map, Number, Value};
use sha2::{Digest, Sha256};
use tauri::http::HeaderMap;
use tauri::ipc::InvokeBody;

/// The command's name, as `command_ingress` reports it.
pub(crate) const COMMAND: &str = "read_preview_mechanics_job_analysis_run_preview";
/// The blocking build ended without an outcome (it panicked).
pub(crate) const PREVIEW_READ_FAILED: &str = "ANALYSIS-RUN-PREVIEW-READ-FAILED";
/// A record whose stored length exceeds the caller's bound.
pub(crate) const RECORD_TOO_LARGE: &str = "ANALYSIS-RUN-RECORD-TOO-LARGE";
/// Where the TypeScript builder throws a JavaScript `TypeError`.
pub(crate) const INPUT_SHAPE_INVALID: &str = "ANALYSIS-RUN-INPUT-SHAPE-INVALID";
pub(crate) const DOCUMENT_KIND: &str = "swbpipe.desktop.analysis_run_preview";
pub(crate) const VERSION: u64 = 1;
/// The fixed part of the working lease: the header's small values, the
/// writers' number and escape scratch. The model-sized load basis, statuses
/// and rule-pack diagnostics are explicit residuals, not bounded by this constant.
pub(crate) const WORKING_ALLOWANCE_BYTES: u64 = 4 << 20;

const ANALYSIS_RUN_V03: &str = "0.3.0";
pub(crate) const SOURCE_CONTRACT_UNSUPPORTED: &str = "SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED";
pub(crate) const RULE_STATUSES: [&str; 3] = ["RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED"];
const RULE_CHECK_RUN_STATUSES: [&str; 3] = RULE_STATUSES;
pub(crate) const OUTSIDE_PROFILE: &str = "CHECKED-JSON-NUMBER-OUTSIDE-PROFILE";
const UNDEFINED_VALUE: &str = "CHECKED-JSON-UNSUPPORTED-TYPE: undefined";
const RECURSION_LIMIT: &str = "CHECKED-JSON-INVALID: recursion limit exceeded";
pub(crate) const RETAINED_RECEIPT_MISMATCH: &str = "RETAINED_PRECISION_RECEIPT_MISMATCH";
const MAX_SAFE_INTEGER: f64 = 9_007_199_254_740_991.0;
/// serde_json's parse depth: a container nested this deep is refused.
const PARSE_DEPTH_LIMIT: usize = 128;
const DETERMINISM_NOTES: [&str; 2] = ["created_at_unavailable", "model_state_ref_and_solver_settings_unit_basis_bound_by_input_manifest"];

// ---------------------------------------------------------------------------
// JavaScript's readings of JSON values
// ---------------------------------------------------------------------------

fn shape(what: &str) -> String {
    format!("{INPUT_SHAPE_INVALID}: {what}")
}

/// `base.key`: `Err` where JavaScript throws (`base` undefined or null). A
/// primitive or an array has none of the members read here.
fn get<'v>(base: Option<&'v Value>, key: &str, what: &str) -> Result<Option<&'v Value>, String> {
    match base {
        None | Some(Value::Null) => Err(shape(what)),
        Some(Value::Object(map)) => Ok(map.get(key)),
        Some(_) => Ok(None),
    }
}

/// `base?.key`.
pub(crate) fn get_opt<'v>(base: Option<&'v Value>, key: &str) -> Option<&'v Value> {
    base.and_then(Value::as_object).and_then(|map| map.get(key))
}

pub(crate) fn nullish(value: Option<&Value>) -> bool {
    matches!(value, None | Some(Value::Null))
}

pub(crate) fn truthy(value: Option<&Value>) -> bool {
    match value {
        None | Some(Value::Null) => false,
        Some(Value::Bool(flag)) => *flag,
        Some(Value::Number(number)) => number.as_f64().is_some_and(|f| f != 0.0),
        Some(Value::String(text)) => !text.is_empty(),
        Some(_) => true,
    }
}

/// `a === b`: scalars by value (numbers as binary64), objects and arrays by
/// identity.
pub(crate) fn strict_eq(a: Option<&Value>, b: Option<&Value>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(Value::Null), Some(Value::Null)) => true,
        (Some(Value::Bool(x)), Some(Value::Bool(y))) => x == y,
        (Some(Value::Number(x)), Some(Value::Number(y))) => x.as_f64() == y.as_f64(),
        (Some(Value::String(x)), Some(Value::String(y))) => x == y,
        (Some(x @ (Value::Object(_) | Value::Array(_))), Some(y)) => std::ptr::eq(x, y),
        _ => false,
    }
}

pub(crate) fn strict_eq_str(value: Option<&Value>, text: &str) -> bool {
    value.and_then(Value::as_str) == Some(text)
}

/// `String(value)`, as a template literal writes it.
pub(crate) fn js_string(value: Option<&Value>) -> String {
    match value {
        None => "undefined".into(),
        Some(Value::Null) => "null".into(),
        Some(Value::Bool(flag)) => flag.to_string(),
        Some(Value::Number(number)) => ecma_number_to_string(number.as_f64().unwrap_or(f64::NAN)),
        Some(Value::String(text)) => text.clone(),
        Some(Value::Array(items)) => items.iter().map(|item| if item.is_null() { String::new() } else { js_string(Some(item)) }).collect::<Vec<_>>().join(","),
        Some(Value::Object(_)) => "[object Object]".into(),
    }
}

/// `text.trim()` is empty: every character is ECMAScript white space or a
/// line terminator.
fn js_blank(text: &str) -> bool {
    text.chars().all(|c| {
        matches!(c, '\u{9}' | '\u{a}' | '\u{b}' | '\u{c}' | '\u{d}' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}')
    })
}

fn sha256_hex_text(text: &str) -> bool {
    text.len() == 64 && text.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// A number JavaScript holds outside the checked profile (`checkedJsonText`:
/// an integral binary64 beyond `Number.MAX_SAFE_INTEGER`).
fn outside_profile(number: &Number) -> bool {
    number.as_f64().is_some_and(|f| f.fract() == 0.0 && f.abs() > MAX_SAFE_INTEGER)
}

/// Whether any number in `value` is outside the checked profile.
pub(crate) fn has_outside_profile(value: &Value) -> bool {
    match value {
        Value::Number(number) => outside_profile(number),
        Value::Array(items) => items.iter().any(has_outside_profile),
        Value::Object(map) => map.values().any(has_outside_profile),
        _ => false,
    }
}

/// The first refusal `checkedJsonText` gives for this record member (`None`:
/// `undefined`).
fn checked_violation(value: Option<&Value>) -> Option<&'static str> {
    match value {
        None => Some(UNDEFINED_VALUE),
        Some(value) => has_outside_profile(value).then_some(OUTSIDE_PROFILE),
    }
}

/// The number serde_json reads back from JavaScript's `JSON.stringify` of
/// this number's binary64 value (`-0` written as `0`).
fn js_number(number: &Number) -> Number {
    let text = ecma_number_to_string(number.as_f64().unwrap_or(f64::NAN));
    serde_json::from_str(&text).expect("ECMAScript number text is JSON")
}

/// `value` written back as `jsonTextKeepingNegativeZero` writes JavaScript's
/// parse of it and read again by serde_json: each number as
/// [`js_number`], a negative zero kept. In place; nothing is allocated
/// beyond one number's text at a time.
pub(crate) fn keep_negative_zero_js_numbers(value: &mut Value) {
    match value {
        Value::Number(number) => {
            let f = number.as_f64().unwrap_or(f64::NAN);
            *number = if f == 0.0 && f.is_sign_negative() { Number::from_f64(-0.0).expect("finite") } else { js_number(number) };
        }
        Value::Array(items) => items.iter_mut().for_each(keep_negative_zero_js_numbers),
        Value::Object(map) => map.values_mut().for_each(keep_negative_zero_js_numbers),
        _ => {}
    }
}

/// The deepest nesting of containers in `value` (a scalar is 0).
fn container_depth(value: &Value) -> usize {
    match value {
        Value::Array(items) => 1 + items.iter().map(container_depth).max().unwrap_or(0),
        Value::Object(map) => 1 + map.values().map(container_depth).max().unwrap_or(0),
        _ => 0,
    }
}

/// The canonical writer's key-order scratch along `value`'s heaviest path:
/// each object's member references, held while its members are written.
fn sort_scratch_bytes(value: &Value) -> u64 {
    match value {
        Value::Array(items) => items.iter().map(sort_scratch_bytes).max().unwrap_or(0),
        Value::Object(map) => {
            let own = ingress::allocation_envelope((map.len() * std::mem::size_of::<&String>()) as u64).unwrap_or(u64::MAX);
            own.saturating_add(map.values().map(sort_scratch_bytes).max().unwrap_or(0))
        }
        _ => 0,
    }
}

// ---------------------------------------------------------------------------
// The writers: canonical text (into SHA-256 or a comparison) and the stored
// form (into a counter or a buffer)
// ---------------------------------------------------------------------------

/// A consumer of the bytes written.
pub(crate) trait Sink {
    fn put(&mut self, bytes: &[u8]);
}

impl Sink for Sha256 {
    fn put(&mut self, bytes: &[u8]) {
        self.update(bytes);
    }
}

impl Sink for Vec<u8> {
    fn put(&mut self, bytes: &[u8]) {
        self.extend_from_slice(bytes);
    }
}

/// Counts the bytes written.
#[derive(Default)]
pub(crate) struct Count(pub(crate) u64);

impl Sink for Count {
    fn put(&mut self, bytes: &[u8]) {
        self.0 += bytes.len() as u64;
    }
}

/// Compares the bytes written with `expected`, without keeping them.
struct Compare<'e> {
    expected: &'e [u8],
    at: usize,
    equal: bool,
}

impl Sink for Compare<'_> {
    fn put(&mut self, bytes: &[u8]) {
        let end = self.at + bytes.len();
        self.equal = self.equal && self.expected.get(self.at..end) == Some(bytes);
        self.at = end;
    }
}

impl Compare<'_> {
    fn whole(&self) -> bool {
        self.equal && self.at == self.expected.len()
    }
}

struct Adapter<'s, S: Sink>(&'s mut S);

impl<S: Sink> std::io::Write for Adapter<'_, S> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.put(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Which text a writer produces.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Style {
    /// RFC 8785 (`canonical_json`): keys in UTF-16 order, ECMAScript numbers.
    Canonical,
    /// The store's text of the value the webview sends (module docs): keys in
    /// serde_json's order, numbers as [`js_number`] writes them.
    Stored,
}

pub(crate) struct Writer<'s, S: Sink> {
    out: &'s mut S,
    style: Style,
}

impl<'s, S: Sink> Writer<'s, S> {
    pub(crate) fn new(out: &'s mut S, style: Style) -> Self {
        Self { out, style }
    }
    pub(crate) fn raw(&mut self, text: &str) {
        self.out.put(text.as_bytes());
    }
    pub(crate) fn string(&mut self, text: &str) {
        serde_json::to_writer(Adapter(&mut *self.out), text).expect("strings always encode as JSON");
    }
    fn key(&mut self, key: &str) {
        self.string(key);
        self.raw(":");
    }
    fn number(&mut self, number: &Number) {
        match self.style {
            Style::Canonical => self.raw(&canonical_number(number)),
            Style::Stored => serde_json::to_writer(Adapter(&mut *self.out), &js_number(number)).expect("numbers encode"),
        }
    }
    fn unsigned(&mut self, value: u64) {
        self.raw(&value.to_string());
    }
    pub(crate) fn value(&mut self, value: &Value) {
        match value {
            Value::Null => self.raw("null"),
            Value::Bool(true) => self.raw("true"),
            Value::Bool(false) => self.raw("false"),
            Value::Number(number) => self.number(number),
            Value::String(text) => self.string(text),
            Value::Array(items) => {
                self.raw("[");
                for (index, item) in items.iter().enumerate() {
                    if index > 0 {
                        self.raw(",");
                    }
                    self.value(item);
                }
                self.raw("]");
            }
            Value::Object(map) => {
                self.raw("{");
                match self.style {
                    Style::Stored => {
                        for (index, (key, item)) in map.iter().enumerate() {
                            if index > 0 {
                                self.raw(",");
                            }
                            self.key(key);
                            self.value(item);
                        }
                    }
                    Style::Canonical => {
                        let mut keys: Vec<&String> = Vec::with_capacity(map.len());
                        keys.extend(map.keys());
                        keys.sort_by(|left, right| left.encode_utf16().cmp(right.encode_utf16()));
                        for (index, key) in keys.into_iter().enumerate() {
                            if index > 0 {
                                self.raw(",");
                            }
                            self.key(key);
                            self.value(&map[key.as_str()]);
                        }
                    }
                }
                self.raw("}");
            }
        }
    }
    /// `{"object_type":<type>,"ref":<reference>}`.
    fn object_ref(&mut self, object_type: &str, reference: &str) {
        self.raw("{\"object_type\":");
        self.string(object_type);
        self.raw(",\"ref\":");
        self.string(reference);
        self.raw("}");
    }
    /// One v0.3 checksum descriptor.
    fn checksum(&mut self, canonicalization: &str, payload_ref: impl FnOnce(&mut Self), scope: &str, value: impl FnOnce(&mut Self)) {
        self.raw("{\"algorithm\":\"sha256\",\"canonicalization\":");
        self.string(canonicalization);
        self.raw(",\"payload_ref\":");
        payload_ref(self);
        self.raw(",\"payload_scope\":");
        self.string(scope);
        self.raw(",\"value\":");
        value(self);
        self.raw("}");
    }
}

/// The canonical text of `value` (`canonical_json`) streamed into SHA-256.
pub(crate) fn canonical_sha256(value: &Value) -> String {
    let mut hasher = Sha256::new();
    Writer::new(&mut hasher, Style::Canonical).value(value);
    hex(&hasher.finalize())
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// ---------------------------------------------------------------------------
// The record contract (`buildAnalysisRunV03`)
// ---------------------------------------------------------------------------

/// The input manifest evidence the builder reads (`ManifestEvidence`): its
/// ref, its SHA-256 and the manifest.
#[derive(Clone, Copy)]
pub(crate) struct ManifestEvidence<'a> {
    pub(crate) manifest_ref: &'a Value,
    pub(crate) manifest_sha256: &'a Value,
    pub(crate) manifest: &'a Value,
}

/// A model rule-pack reference the solve left out of the manifest
/// (`ExcludedRulePackRef`).
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Exclusion {
    pub(crate) rule_pack_id: String,
    pub(crate) rule_pack_version: String,
    pub(crate) code: String,
}

/// `buildAnalysisRunV03`'s arguments after the result: the manifest
/// evidence, the rule-check status (`None` is `undefined`), the load basis
/// refs (`None` is `undefined`) and the exclusions.
pub(crate) struct BuildInputs<'a> {
    pub(crate) manifest: ManifestEvidence<'a>,
    pub(crate) rule_check_status: Option<&'a Value>,
    pub(crate) load_basis_refs: Option<&'a Value>,
    pub(crate) rule_pack_exclusions: &'a [Exclusion],
}

/// The route the header dispatch reads, its table binding, and the record
/// contract's table for it.
#[derive(Debug, Clone)]
pub(crate) struct Route {
    pub(crate) name: &'static str,
    pub(crate) binding_id: String,
    pub(crate) binding_sha256: String,
    pub(crate) table: Option<&'static Value>,
}

impl Route {
    pub(crate) fn retained(&self) -> bool {
        matches!(self.name, "retained_preview_physics" | "retained_physics")
    }
}

/// The record builder's tables (`RECORD_TABLES`): each pinned table, by its
/// `semantic_contract_id`.
fn record_table(id: &str) -> Option<&'static Value> {
    [
        sc::pressure_rows_contract(),
        sc::pressure_source_contract(),
        sc::preview_physics_contract(),
        sc::preview_physics_timoshenko_contract(),
        sc::preview_physics_retained_contract(),
        sc::physics_retained_contract(),
    ]
    .into_iter()
    .find(|table| table["semantic_contract_id"].as_str() == Some(id))
}

/// `hasCurrentSourceContract` and `sourceContract`: the dispatch's route and
/// binding, or [`SOURCE_CONTRACT_UNSUPPORTED`].
pub(crate) fn dispatch(source: &Value) -> Result<Route, String> {
    let Ok((name, _)) = rv::route(source) else { return Err(SOURCE_CONTRACT_UNSUPPORTED.into()) };
    let binding = rv::binding_of(name, source);
    match (binding["id"].as_str(), binding["sha256"].as_str()) {
        (Some(id), Some(sha256)) => Ok(Route { name, binding_id: id.into(), binding_sha256: sha256.into(), table: record_table(id) }),
        _ => Err(SOURCE_CONTRACT_UNSUPPORTED.into()),
    }
}

/// A reader's refusal as `ResultReaderError`'s message (`detail ?? code`).
fn reader_refusal(outcome: &Value) -> Option<(String, String)> {
    if outcome["ok"] == Value::Bool(true) {
        return None;
    }
    let code = outcome["error"]["code"].as_str().unwrap_or_default().to_string();
    let message = match &outcome["error"]["detail"] {
        Value::String(detail) => detail.clone(),
        _ => code.clone(),
    };
    Some((code, message))
}

/// The route's reader, as `buildAnalysisRecord` runs it first: the
/// preview-physics and pressure readers; the retained reader on the value
/// `validateRetainedPrecision` writes (module docs; `source` is rewritten in
/// place for it); the pressure-source reader, with the record surfaces'
/// receipt and publication codes.
pub(crate) fn validate_route(source: &mut Value, route: &Route) -> Result<(), String> {
    match route.name {
        "preview_physics" | "pressure" => {
            let reader = if route.name == "pressure" { "pressure" } else { "preview-physics" };
            match reader_refusal(&rv::validate(reader, source, None, None)) {
                Some((_, message)) => Err(message),
                None => Ok(()),
            }
        }
        "retained_preview_physics" | "retained_physics" => {
            if has_outside_profile(source) {
                return Err(RETAINED_RECEIPT_MISMATCH.into());
            }
            keep_negative_zero_js_numbers(source);
            rp::validate(source, None).map(drop).map_err(|error| error.code)
        }
        "pressure_source" => match reader_refusal(&rv::validate("pressure-source", source, None, None)) {
            Some((code, _)) if code == "SOURCE_BLOCKS_RECEIPT_HASH" => Err("SOURCE_RECOVERY_RECEIPT_HASH".into()),
            Some((code, _)) if code == "SOURCE_BLOCKS_PUBLICATION_HASH" => Err("SOURCE_RECOVERY_PUBLICATION_HASH".into()),
            Some((_, message)) => Err(message),
            None => Ok(()),
        },
        _ => Err(SOURCE_CONTRACT_UNSUPPORTED.into()),
    }
}

/// `sourceBasisReference`: `(object type, ref)` or none.
fn source_basis_reference(basis: Option<&Value>) -> Result<Option<(&'static str, &str)>, String> {
    const INVALID: &str = "ANALYSIS_SOURCE_REFERENCE_INVALID";
    match basis {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Object(map)) if map.len() == 2 => {
            let text = |key: &str| map.get(key).and_then(Value::as_str).filter(|text| !text.is_empty());
            match (text("ref_type"), text("ref_id")) {
                (Some(kind), Some(id)) => Ok(Some((
                    match kind {
                        "load_case" => "LoadCase",
                        "combination" => "Combination",
                        _ => "ResultBasis",
                    },
                    id,
                ))),
                _ => Err(INVALID.into()),
            }
        }
        Some(_) => Err(INVALID.into()),
    }
}

/// `source.results` as rows (`.map` throws on anything but an array).
pub(crate) fn rows_of(source: &Value) -> Result<&[Value], String> {
    match get(Some(source), "results", "result")? {
        Some(Value::Array(rows)) => Ok(rows),
        _ => Err(shape("result.results is not an array")),
    }
}

/// `expectedLoadBasis`: the record's load basis refs, after the source's
/// identities and bases and, when given, the expected refs' coverage. The
/// rows are read twice (their bases checked, then covered) rather than
/// collected, so nothing here grows with the rows but the distinct bases of
/// an undefined basis (the builder's own default, which the preview never
/// takes).
pub(crate) fn expected_load_basis(source: &Value, expected: Option<&Value>) -> Result<Vec<Value>, String> {
    const REFERENCE_INVALID: &str = "ANALYSIS_SOURCE_REFERENCE_INVALID";
    const BASIS_INVALID: &str = "ANALYSIS_LOAD_BASIS_INVALID";
    let rows = rows_of(source)?;
    // The identities' array is built before any is checked.
    for row in rows {
        get(Some(row), "id", "a result row")?;
    }
    let text = |value: Option<&Value>| value.and_then(Value::as_str).is_some_and(|text| !text.is_empty());
    if !text(source.get("run_id")) || !text(source.get("model_ref")) || !rows.iter().all(|row| text(row.get("id"))) {
        return Err(REFERENCE_INVALID.into());
    }
    for row in rows {
        source_basis_reference(row.get("basis_ref"))?;
    }
    let Some(expected) = expected else {
        let mut distinct: Vec<(&str, &str)> = Vec::new();
        for row in rows {
            if let Some(reference) = source_basis_reference(row.get("basis_ref"))? {
                if !distinct.contains(&reference) {
                    distinct.push(reference);
                }
            }
        }
        return Ok(distinct.into_iter().map(|(kind, id)| json!({"object_type": kind, "ref": id})).collect());
    };
    let Value::Array(entries) = expected else { return Err(BASIS_INVALID.into()) };
    let mut admitted: Vec<(&str, &str)> = Vec::with_capacity(entries.len());
    for entry in entries {
        let map = entry.as_object().filter(|map| map.len() == 2).ok_or(BASIS_INVALID)?;
        let kind = map.get("object_type").and_then(Value::as_str).filter(|kind| ["LoadCase", "Combination", "ResultBasis"].contains(kind));
        let id = map.get("ref").and_then(Value::as_str).filter(|id| !id.is_empty());
        let (Some(kind), Some(id)) = (kind, id) else { return Err(BASIS_INVALID.into()) };
        if admitted.contains(&(kind, id)) {
            return Err(BASIS_INVALID.into());
        }
        admitted.push((kind, id));
    }
    let admitted: std::collections::HashSet<(&str, &str)> = admitted.into_iter().collect();
    let mut cases_refs = Vec::new();
    let quality = source.get("numerical_quality").filter(|value| !value.is_null());
    match get_opt(quality, "cases") {
        None | Some(Value::Null) => {}
        Some(Value::Array(cases)) => {
            for case in cases {
                if let Some(reference) = source_basis_reference(get(Some(case), "basis_ref", "a numerical_quality case")?)? {
                    cases_refs.push(reference);
                }
            }
        }
        Some(_) => return Err(shape("result.numerical_quality.cases is not an array")),
    }
    let mut covered = true;
    for row in rows {
        if let Some(reference) = source_basis_reference(row.get("basis_ref"))? {
            covered = covered && admitted.contains(&reference);
        }
    }
    if !covered || cases_refs.iter().any(|reference| !admitted.contains(reference)) {
        return Err("ANALYSIS_LOAD_BASIS_SOURCE_SCOPE_MISMATCH".into());
    }
    Ok(entries.clone())
}

/// `sourceBasisMatches`.
fn source_basis_matches(signature: &Value, row: &Value) -> bool {
    match signature.get("source_basis") {
        None => true,
        Some(basis) => strict_eq(Some(basis), get_opt(row.get("metadata"), "basis")),
    }
}

/// A row's record interpretation (`analysisRowSemanticsIn`): its table
/// signature, or none, and the findings.
pub(crate) fn interpret<'t>(table: &'t Value, row: &Value) -> Result<(Option<&'t Value>, &'static [&'static str]), String> {
    let entries = table["rows"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    let kind = row.get("kind");
    let unit = row.get("unit");
    let by_kind: Vec<&Value> = entries.iter().filter(|entry| strict_eq(entry.get("kind"), kind)).collect();
    if by_kind.is_empty() {
        return Ok((None, &["SOURCE_SIGNATURE_UNKNOWN"]));
    }
    let by_unit: Vec<&Value> = by_kind.into_iter().filter(|entry| strict_eq(entry.get("unit"), unit)).collect();
    if by_unit.is_empty() {
        return Err(format!("SOURCE_UNIT_CONTRADICTION: {} / {}", js_string(kind), js_string(unit)));
    }
    let component = get_opt(row.get("metadata"), "component").filter(|value| !value.is_null());
    let component_eq = |entry: &Value| match component {
        None => entry.get("component") == Some(&Value::Null),
        Some(component) => strict_eq(entry.get("component"), Some(component)),
    };
    let dimension_checked = |entry: &Value| -> Result<(), String> {
        let declared = entry.get("legacy_declared_dimension");
        let dimension = row.get("dimension");
        if truthy(dimension) && truthy(declared) && !strict_eq(dimension, declared) {
            return Err(format!("ANALYSIS-RUN-RESULT-DIMENSION-MISMATCH: {}", js_string(row.get("id"))));
        }
        Ok(())
    };
    if let Some(exact) = by_unit.iter().find(|entry| component_eq(entry) && source_basis_matches(entry, row)) {
        dimension_checked(exact)?;
        return Ok((Some(*exact), &[]));
    }
    if let Some(generic) = by_unit.iter().find(|entry| entry.get("component") == Some(&Value::Null) && source_basis_matches(entry, row)) {
        dimension_checked(generic)?;
        return Ok((Some(*generic), if component.is_some() { &[] } else { &["OPTIONAL_SOURCE_METADATA_MISSING"] }));
    }
    if by_unit.iter().any(|entry| component_eq(entry) || entry.get("component") == Some(&Value::Null)) {
        return Err(format!("SOURCE_BASIS_CONTRADICTION: {}", js_string(kind)));
    }
    if let Some(component) = component {
        return Err(format!("SOURCE_COMPONENT_CONTRADICTION: {} / {}", js_string(kind), js_string(Some(component))));
    }
    Ok((None, &["SOURCE_COMPONENT_MISSING_SEMANTICS_UNAVAILABLE"]))
}

/// What the passes established: the record's members other than its rows,
/// borrowed from the source and the inputs, its checksums and its stored
/// length.
pub(crate) struct Built<'a> {
    /// The parsed held result the record was built from (the persistence
    /// writers read it; as the retained reader rewrote it, numbers only).
    pub(crate) source: &'a Value,
    rows: &'a [Value],
    pub(crate) route: Route,
    table: &'static Value,
    pub(crate) run_id: &'a str,
    model_ref: &'a str,
    solver_name: &'a Value,
    solver_version: &'a Value,
    solver_build_ref: &'a Value,
    manifest_ref: &'a Value,
    manifest_sha256: &'a Value,
    settings_ref: String,
    unit_system_ref: String,
    load_basis: Vec<Value>,
    producer_diagnostics: &'a [Value],
    rule_pack_diagnostics: Vec<Value>,
    statuses: Vec<Value>,
    pub(crate) rule_check_status: String,
    /// The copied members, in key order.
    copies: Vec<(&'static str, &'a Value)>,
    pub(crate) received_sha256: String,
    pub(crate) record_sha256: String,
    pub(crate) record_bytes: u64,
}

/// `buildAnalysisRunV03(result, ...)` on a parsed source: the dispatch, the
/// route's reader (which may rewrite `source` in place, module docs), then
/// [`build`].
#[cfg(test)]
pub(crate) fn build_v03<'a>(
    source: &'a mut Value,
    inputs: &BuildInputs<'a>,
    reserve: &mut dyn FnMut(u64) -> Result<Option<Lease>, CommandError<String>>,
) -> Result<Built<'a>, CommandError<String>> {
    let route = dispatch(source).map_err(CommandError::Command)?;
    validate_route(source, &route).map_err(CommandError::Command)?;
    build(source, route, inputs, reserve)
}

/// The record after the route's reader passed (`buildAnalysisRecord` from
/// `validateSourceRuleStatus` on, then `validateAnalysisRunV03`'s checks the
/// record can still fail), in the TypeScript's order. `reserve` admits the
/// fingerprints' allocation.
pub(crate) fn build<'a>(
    source: &'a Value,
    route: Route,
    inputs: &BuildInputs<'a>,
    reserve: &mut dyn FnMut(u64) -> Result<Option<Lease>, CommandError<String>>,
) -> Result<Built<'a>, CommandError<String>> {
    let refuse = |text: String| CommandError::Command(text);
    let manifest = inputs.manifest;
    // validateSourceRuleStatus
    let status = get(Some(source), "status", "result").map_err(refuse)?;
    let status = status.filter(|status| !status.is_null()).ok_or_else(|| refuse(shape("result.status")))?;
    let source_rule = get(Some(status), "rule_check", "result.status").map_err(refuse)?;
    if !nullish(source_rule) && !source_rule.and_then(Value::as_str).is_some_and(|rule| RULE_STATUSES.contains(&rule)) {
        return Err(refuse("ANALYSIS_SOURCE_RULE_STATUS_INVALID".into()));
    }
    let load_basis = expected_load_basis(source, inputs.load_basis_refs).map_err(refuse)?;
    let model_basis = get(Some(manifest.manifest), "model_basis", "inputManifest.manifest").map_err(refuse)?;
    let manifest_model = get(model_basis, "model_ref", "inputManifest.manifest.model_basis").map_err(refuse)?;
    let model_ref = source["model_ref"].as_str().expect("checked with the load basis");
    if !strict_eq_str(manifest_model, model_ref) {
        return Err(refuse("ANALYSIS-RUN-INPUT-MANIFEST-MODEL-MISMATCH".into()));
    }
    // The rule-pack reference diagnostics, against the manifest's exclusions.
    let identities: Vec<String> = inputs.rule_pack_exclusions.iter().map(|e| rule_pack_identity(&e.rule_pack_id, &e.rule_pack_version)).collect();
    let rule_pack_diagnostics: Vec<Value> = inputs
        .rule_pack_exclusions
        .iter()
        .zip(&identities)
        .map(|(exclusion, identity)| json!({"source_annotation": rule_pack_ref_diagnostic(&exclusion.code, identity)}))
        .collect();
    let excluded: Vec<String> = manifest_excluded_rule_packs(manifest.manifest).map_err(refuse)?.iter().map(|(id, version)| rule_pack_identity(id, version)).collect();
    if identities != excluded {
        let none_resolved = manifest.manifest.get("active_rule_packs").and_then(Value::as_array).is_some_and(Vec::is_empty);
        return Err(refuse(if identities.is_empty() && !excluded.is_empty() && none_resolved {
            format!("{RULE_PACKS_NOT_RESOLVED}: {RULE_PACKS_NOT_RESOLVED_DETAIL}")
        } else {
            "ANALYSIS_RULE_PACK_REF_DIAGNOSTICS_MISMATCH".into()
        }));
    }
    // The rows (`Promise.all`): an interpretation refusal first, whichever row.
    let rows = rows_of(source).map_err(refuse)?;
    let mut outside = false;
    for row in rows {
        match route.table {
            None => return Err(refuse(SOURCE_CONTRACT_UNSUPPORTED.into())),
            Some(table) => {
                interpret(table, row).map_err(refuse)?;
            }
        }
        outside = outside || has_outside_profile(row);
    }
    if outside {
        return Err(refuse(OUTSIDE_PROFILE.into()));
    }
    let table = route.table.expect("checked with the rows");
    let rule_check_status = match inputs.rule_check_status.filter(|value| !value.is_null()).or(source_rule.filter(|value| !value.is_null())) {
        None => RULE_STATUSES[0].to_string(),
        Some(Value::String(rule)) if RULE_STATUSES.contains(&rule.as_str()) => rule.clone(),
        Some(_) => return Err(refuse("ANALYSIS_RULE_STATUS_INVALID".into())),
    };
    let statuses = statuses(status.get("mechanics"), &rule_check_status);
    // The record literal's reads, in its order.
    let solver_basis = get(Some(manifest.manifest), "solver_basis", "inputManifest.manifest").map_err(refuse)?;
    let read = |key: &str| get(solver_basis, key, "inputManifest.manifest.solver_basis").map_err(refuse);
    let (solver_name, solver_version, solver_build_ref) = (read("solver_name")?, read("solver_version")?, read("solver_build_ref")?);
    let manifest_ref_ref = get(Some(manifest.manifest_ref), "ref", "inputManifest.manifest_ref").map_err(refuse)?;
    let settings_ref = format!("solver-settings:{}:{}", js_string(manifest_ref_ref), js_string(Some(manifest.manifest_sha256)));
    let unit_system_ref = format!("unit-system:{model_ref}:{}", js_string(Some(manifest.manifest_sha256)));
    let producer_diagnostics = match source.get("diagnostics") {
        Some(Value::Array(items)) => items.as_slice(),
        _ => return Err(refuse(shape("result.diagnostics is not an array"))),
    };
    if has_outside_profile(source) {
        return Err(refuse(OUTSIDE_PROFILE.into()));
    }
    let received_sha256 = canonical_sha256(source);
    let copied = |key: &'static str| (key, source.get(key));
    let copies: Vec<(&'static str, Option<&'a Value>)> = match route.name {
        "pressure_source" => vec![copied("contract_evidence"), copied("source_block_recovery")],
        _ if route.retained() => vec![copied("retained_precision")],
        _ => Vec::new(),
    };
    // The record checksum's checked text: `undefined` or a number outside the
    // profile, in the record's member order, then serde_json's depth.
    let run_id = source["run_id"].as_str().expect("checked with the load basis");
    let first_violation = [solver_name, solver_version, solver_build_ref]
        .into_iter()
        .map(checked_violation)
        .chain(rows.iter().flat_map(|row| [checked_violation(row.get("kind")), checked_violation(row.get("unit"))]))
        .chain([checked_violation(Some(manifest.manifest_ref)), checked_violation(Some(manifest.manifest_sha256))])
        .chain(copies.iter().map(|(_, value)| checked_violation(*value)))
        .flatten()
        .next();
    if let Some(violation) = first_violation {
        return Err(refuse(violation.into()));
    }
    let deep = |value: &Value, level: usize| level + container_depth(value) > PARSE_DEPTH_LIMIT;
    let too_deep = [(solver_name, 4), (solver_version, 4), (solver_build_ref, 5)].iter().any(|(value, level)| value.is_some_and(|v| deep(v, *level)))
        || rows.iter().any(|row| ["kind", "unit", "metadata"].iter().any(|key| row.get(key).is_some_and(|value| deep(value, 6))))
        || producer_diagnostics.iter().any(|item| deep(item, 5))
        || statuses.iter().any(|item| deep(item, 4))
        || deep(manifest.manifest_ref, 6)
        || deep(manifest.manifest_sha256, 6)
        || copies.iter().any(|(_, value)| value.is_some_and(|v| deep(v, 3)));
    if too_deep {
        return Err(refuse(RECURSION_LIMIT.into()));
    }
    let mut built = Built {
        source,
        rows,
        table,
        run_id,
        model_ref,
        solver_name: solver_name.expect("checked"),
        solver_version: solver_version.expect("checked"),
        solver_build_ref: solver_build_ref.expect("checked"),
        manifest_ref: manifest.manifest_ref,
        manifest_sha256: manifest.manifest_sha256,
        settings_ref,
        unit_system_ref,
        load_basis,
        producer_diagnostics,
        rule_pack_diagnostics,
        statuses,
        rule_check_status,
        copies: copies.into_iter().map(|(key, value)| (key, value.expect("checked"))).collect(),
        received_sha256,
        record_sha256: String::new(),
        record_bytes: 0,
        route,
    };
    let mut hasher = Sha256::new();
    write_record(&mut Writer::new(&mut hasher, Style::Canonical), &built, Part::Projection, None, &mut |row| canonical_sha256(row));
    built.record_sha256 = hex(&hasher.finalize());
    let mut count = Count::default();
    let placeholder = "0".repeat(64);
    write_record(&mut Writer::new(&mut count, Style::Stored), &built, Part::Stored, None, &mut |_| placeholder.clone());
    built.record_bytes = count.0;
    validate_built(&built, source, inputs, reserve)?;
    Ok(built)
}

/// `Array.from(new Set(["HUMAN_REVIEW_REQUIRED", mechanics, rule].filter(Boolean))).sort()`.
fn statuses(mechanics: Option<&Value>, rule: &str) -> Vec<Value> {
    let mut items: Vec<Value> = Vec::new();
    for item in [Some(&json!("HUMAN_REVIEW_REQUIRED")), mechanics, Some(&json!(rule))].into_iter().flatten() {
        if truthy(Some(item)) && !items.iter().any(|seen| !seen.is_object() && !seen.is_array() && strict_eq(Some(seen), Some(item))) {
            items.push(item.clone());
        }
    }
    items.sort_by(|left, right| js_string(Some(left)).encode_utf16().cmp(js_string(Some(right)).encode_utf16()));
    items
}

/// `validateAnalysisRunV03(record, result, loadBasisRefs)` on the record just
/// built: the checks such a record can fail, in its order (the appended
/// diagnostics' codes and identities, the mechanics status, the producer,
/// duplicate row ids); the rest hold by construction.
fn validate_built(built: &Built<'_>, source: &Value, inputs: &BuildInputs<'_>, reserve: &mut dyn FnMut(u64) -> Result<Option<Lease>, CommandError<String>>) -> Result<(), CommandError<String>> {
    let refuse = |text: &str| CommandError::Command(text.to_string());
    let mut seen = std::collections::HashSet::new();
    for exclusion in inputs.rule_pack_exclusions {
        let identity = rule_pack_identity(&exclusion.rule_pack_id, &exclusion.rule_pack_version);
        if ![RULE_PACK_REF_UNRESOLVED, RULE_PACK_REF_CHECKSUM_MISMATCH].contains(&exclusion.code.as_str()) || !seen.insert(identity) {
            return Err(refuse("ANALYSIS_SOURCE_DIAGNOSTICS_MISMATCH"));
        }
    }
    if !source["status"]["mechanics"].as_str().is_some_and(|mechanics| ["MECHANICS_SOLVED", "MODEL_INCOMPLETE"].contains(&mechanics)) {
        return Err(refuse("ANALYSIS_SOURCE_MECHANICS_STATUS_INVALID"));
    }
    let producer = source.get("producer").filter(|producer| !producer.is_null()).ok_or_else(|| CommandError::Command(shape("result.producer")))?;
    if !strict_eq(Some(built.solver_name), get_opt(Some(producer), "component_name")) || !strict_eq(Some(built.solver_version), get_opt(Some(producer), "component_version")) {
        return Err(refuse("ANALYSIS_SOURCE_PRODUCER_MISMATCH"));
    }
    if duplicate_ids(built.rows, reserve)? {
        return Err(refuse("ANALYSIS_ROW_ACCOUNTING_MISMATCH"));
    }
    Ok(())
}

/// Whether two rows share an id (`new Set(ids).size !== rows`): one
/// fingerprint and ordinal per row, sorted in place, and the ids of rows
/// whose fingerprints agree compared. The fingerprints' allocation is
/// admitted at its exact size first.
pub(crate) fn duplicate_ids(rows: &[Value], reserve: &mut dyn FnMut(u64) -> Result<Option<Lease>, CommandError<String>>) -> Result<bool, CommandError<String>> {
    use std::hash::{Hash, Hasher};
    let bytes = (rows.len() as u64).saturating_mul(std::mem::size_of::<(u64, usize)>() as u64);
    let _lease = reserve(ingress::allocation_envelope(bytes).unwrap_or(u64::MAX))?;
    let id = |index: usize| rows[index]["id"].as_str().unwrap_or_default();
    let mut prints: Vec<(u64, usize)> = Vec::with_capacity(rows.len());
    for index in 0..rows.len() {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        id(index).hash(&mut hasher);
        prints.push((hasher.finish(), index));
    }
    prints.sort_unstable();
    let mut start = 0;
    while start < prints.len() {
        let end = start + prints[start..].iter().take_while(|print| print.0 == prints[start].0).count();
        let group = &prints[start..end];
        for (at, a) in group.iter().enumerate() {
            if group[at + 1..].iter().any(|b| id(a.1) == id(b.1)) {
                return Ok(true);
            }
        }
        start = end;
    }
    Ok(false)
}

/// Which part of the record a writer writes.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Part {
    /// The record checksum's payload (`analysisRecordProjection`): the record
    /// without its own checksum.
    Projection,
    /// The whole record.
    Stored,
    /// The header's record: every member but the row-scale ones.
    Header,
}

const PROVENANCE: &str = r#"{"professional_claim":false,"review_status":"pending","source_license":"project-governed","source_location":"analysis_run.compatibility.v0.3","source_name":"OpenPipeStress analysis record 0.3"}"#;

/// One result ref, written from the borrowed row (its keys in order).
fn write_result_ref<S: Sink>(w: &mut Writer<'_, S>, built: &Built<'_>, index: usize, row: &Value, row_sha256: &str) {
    let (semantic, findings) = interpret(built.table, row).expect("interpreted by the row pass");
    let id = row["id"].as_str().expect("checked with the load basis");
    let member = |key: &str| semantic.and_then(|entry| entry.get(key)).filter(|value| !value.is_null());
    w.raw("{\"category\":");
    match member("category") {
        Some(value) => w.value(value),
        None => w.string("unknown"),
    }
    w.raw(",\"hash_refs\":[");
    w.checksum(CHECKED_PROFILE_V1, |w| w.object_ref("Result", id), "result_row", |w| w.string(row_sha256));
    w.raw("],\"interpretation\":{\"findings\":[");
    for (at, finding) in findings.iter().enumerate() {
        if at > 0 {
            w.raw(",");
        }
        w.string(finding);
    }
    w.raw("],\"status\":");
    match member("canonical_disposition") {
        Some(value) => w.value(value),
        None => w.string("unavailable"),
    }
    w.raw("},\"privacy_classification\":\"source_evidence\",\"provenance\":");
    w.raw(PROVENANCE);
    w.raw(",\"result_family\":");
    w.value(member("family").unwrap_or(&Value::Null));
    w.raw(",\"result_ref\":");
    w.object_ref("Result", id);
    w.raw(",\"semantic_contract\":{\"id\":");
    w.string(&built.route.binding_id);
    w.raw(",\"sha256\":");
    w.string(&built.route.binding_sha256);
    w.raw(",\"signature_id\":");
    w.value(member("signature_id").unwrap_or(&Value::Null));
    w.raw("},\"source_annotation\":{\"kind\":");
    w.value(row.get("kind").expect("checked with the record checksum"));
    w.raw(",\"metadata\":");
    w.value(row.get("metadata").unwrap_or(&Value::Null));
    w.raw(",\"unit\":");
    w.value(row.get("unit").expect("checked with the record checksum"));
    w.raw("},\"source_dimension\":");
    w.value(member("source_physical_semantic_dimension").unwrap_or(&Value::Null));
    w.raw(",\"source_row_index\":");
    w.unsigned(index as u64);
    w.raw("}");
}

/// The record (or its projection or header part), its members in key order,
/// which is the same in both styles (every key the record itself writes is
/// ASCII); copied values follow the writer's style. `row_sha256` gives each
/// row's checksum.
fn write_record<S: Sink>(w: &mut Writer<'_, S>, built: &Built<'_>, part: Part, revision: Option<&Revision<'_>>, row_sha256: &mut dyn FnMut(&Value) -> String) {
    let copy = |w: &mut Writer<'_, S>, key: &str| {
        if part == Part::Header {
            return;
        }
        if let Some((_, value)) = built.copies.iter().find(|(name, _)| *name == key) {
            w.raw(",");
            w.key(key);
            w.value(value);
        }
    };
    w.raw("{\"analysis_run\":{\"analysis_status\":[");
    for (at, status) in built.statuses.iter().enumerate() {
        if at > 0 {
            w.raw(",");
        }
        w.value(status);
    }
    w.raw("]");
    copy(w, "contract_evidence");
    match revision {
        Some(revision) => {
            w.raw(",\"created_at\":");
            w.string(revision.created_at);
        }
        None => w.raw(",\"created_at\":null"),
    }
    if part != Part::Header {
        w.raw(",\"diagnostics\":[");
        for (at, item) in built.producer_diagnostics.iter().chain(&built.rule_pack_diagnostics).enumerate() {
            if at > 0 {
                w.raw(",");
            }
            if at < built.producer_diagnostics.len() {
                w.raw("{\"source_annotation\":");
                w.value(item);
                w.raw("}");
            } else {
                w.value(item);
            }
        }
        w.raw("]");
    }
    w.raw(",\"hashes\":[");
    if part != Part::Projection {
        let record_sha256 = revision.map_or(built.record_sha256.as_str(), |revision| revision.record_sha256.as_str());
        w.checksum(CHECKED_PROFILE_V1, |w| w.object_ref("AnalysisRun", built.run_id), "analysis_run_record", |w| w.string(record_sha256));
        w.raw(",");
    }
    let envelope = format!("result-envelope:{}", built.run_id);
    w.checksum(CHECKED_PROFILE_V1, |w| w.object_ref("ResultEnvelope", &envelope), "received_result", |w| w.string(&built.received_sha256));
    w.raw("],\"immutability_policy\":{\"hash_invalidates_external_acceptance\":true,\"mutation_policy\":\"changes_create_new_immutable_record_revision\",\"new_mechanics_run_required_for_record_revision\":false,\"record_revision_identity\":\"analysis_run_record_sha256\",\"run_record_is_read_only\":true},\"library_refs\":[");
    write_list(w, revision.map_or(&[][..], |revision| revision.library_refs));
    w.raw("],\"load_basis_refs\":[");
    for (at, reference) in built.load_basis.iter().enumerate() {
        if at > 0 {
            w.raw(",");
        }
        w.value(reference);
    }
    w.raw("],\"model_state_ref\":");
    match revision {
        Some(revision) => w.object_ref("ModelState", revision.state_id),
        None => w.object_ref("ModelState", &format!("state:{}:preview", built.model_ref)),
    }
    w.raw(",\"professional_boundary\":{\"human_review_required\":true,\"software_makes_approval_claim\":false,\"software_makes_authentication_claim\":false,\"software_makes_certification_claim\":false,\"software_makes_compliance_claim\":false,\"software_makes_sealing_claim\":false},\"provenance\":");
    w.raw(PROVENANCE);
    w.raw(",\"reproducibility\":{\"determinism_notes\":[");
    // A recorded revision drops the `created_at_unavailable` note (its `created_at` is set).
    let notes = if revision.is_some() { &DETERMINISM_NOTES[1..] } else { &DETERMINISM_NOTES[..] };
    for (at, note) in notes.iter().enumerate() {
        if at > 0 {
            w.raw(",");
        }
        w.string(note);
    }
    w.raw("],\"input_manifest_hashes\":[");
    w.checksum("rfc8785_jcs", |w| w.value(built.manifest_ref), "input_manifest", |w| w.value(built.manifest_sha256));
    w.raw("],\"input_manifest_refs\":[");
    w.value(built.manifest_ref);
    w.raw("],\"semantic_contract\":{\"id\":");
    w.string(&built.route.binding_id);
    w.raw(",\"sha256\":");
    w.string(&built.route.binding_sha256);
    w.raw("},\"unresolved_tbd\":[]}");
    if part != Part::Header {
        w.raw(",\"result_refs\":[");
        for (index, row) in built.rows.iter().enumerate() {
            if index > 0 {
                w.raw(",");
            }
            let row_hash = row_sha256(row);
            write_result_ref(w, built, index, row, &row_hash);
        }
        w.raw("]");
    }
    copy(w, "retained_precision");
    w.raw(",\"rule_pack_refs\":[");
    write_list(w, revision.map_or(&[][..], |revision| revision.rule_pack_refs));
    w.raw("],\"run_id\":");
    w.string(built.run_id);
    w.raw(",\"run_kind\":\"mechanics_solve\",\"run_name\":");
    w.string(&format!("{} analysis record", built.run_id));
    w.raw(",\"settings_ref\":");
    w.object_ref("SolverSettings", &built.settings_ref);
    w.raw(",\"solver_version\":{\"build_ref\":{\"object_type\":\"ExternalReference\",\"ref\":");
    w.value(built.solver_build_ref);
    w.raw("},\"solver_name\":");
    w.value(built.solver_name);
    w.raw(",\"solver_version\":");
    w.value(built.solver_version);
    w.raw("}");
    copy(w, "source_block_recovery");
    w.raw(",\"unit_system_ref\":");
    w.object_ref("UnitSystem", &built.unit_system_ref);
    w.raw("},\"deliverable_id\":\"DEL-14-02\",\"objectives\":[\"OBJ-016\"],\"package_id\":\"PKG-14\",\"run_contract_status\":{\"external_validation_boundary\":\"reference_only_not_determined_by_software\",\"record_contract\":\"strict_analysis_run_v0_3\",\"result_binding\":\"received_mechanics_result\"},\"schema_version\":\"");
    w.raw(ANALYSIS_RUN_V03);
    w.raw("\",\"scope_item\":\"SOW-072\"}");
}

/// Writes `items` as a JSON list's members (comma-separated).
fn write_list<S: Sink>(w: &mut Writer<'_, S>, items: &[Value]) {
    for (at, item) in items.iter().enumerate() {
        if at > 0 {
            w.raw(",");
        }
        w.value(item);
    }
}

/// Record Run's recorded revision of the record
/// (`analysis_runs::recorded_revision` over the transient record): its
/// `created_at`, its state, the manifest's rule-pack refs and the solved
/// model's library refs, the `created_at_unavailable` note dropped, and its
/// own `analysis_run_record` checksum (the canonical projection of the
/// revision, [`revision_sha256`]). Every other member is the transient
/// record's.
pub(crate) struct Revision<'r> {
    pub(crate) created_at: &'r str,
    pub(crate) state_id: &'r str,
    pub(crate) rule_pack_refs: &'r [Value],
    pub(crate) library_refs: &'r [Value],
    /// Filled by [`Revision::sealed`].
    pub(crate) record_sha256: String,
}

impl Revision<'_> {
    /// The revision with its own checksum: the record's canonical projection
    /// with the revision applied, streamed into SHA-256 (each row's checksum
    /// streamed again).
    pub(crate) fn sealed(mut self, built: &Built<'_>) -> Self {
        let mut hasher = Sha256::new();
        write_record(&mut Writer::new(&mut hasher, Style::Canonical), built, Part::Projection, Some(&self), &mut |row| canonical_sha256(row));
        self.record_sha256 = hex(&hasher.finalize());
        self
    }

    /// The revision's exact stored length, counted (the rows' checksums by length).
    pub(crate) fn stored_length(&self, built: &Built<'_>) -> u64 {
        let mut count = Count::default();
        let placeholder = "0".repeat(64);
        write_record(&mut Writer::new(&mut count, Style::Stored), built, Part::Stored, Some(self), &mut |_| placeholder.clone());
        count.0
    }

    /// The revision's stored bytes, into a buffer of `length` (its
    /// [`Revision::stored_length`]) admitted through `reserve` first.
    pub(crate) fn stored_bytes<R>(&self, built: &Built<'_>, length: u64, reserve: impl FnOnce(u64) -> Result<R, CommandError<String>>) -> Result<(Vec<u8>, R), CommandError<String>> {
        let guard = reserve(ingress::allocation_envelope(length).unwrap_or(u64::MAX))?;
        let mut out = Vec::with_capacity(length as usize);
        write_record(&mut Writer::new(&mut out, Style::Stored), built, Part::Stored, Some(self), &mut |row| canonical_sha256(row));
        assert!(out.len() as u64 == length, "the recorded revision is its counted length");
        Ok((out, guard))
    }
}

/// The record's members other than its row-scale ones (the header's
/// `record`: no `result_refs`, producer `diagnostics` or copied members), as a
/// value, for the binding checks Record Run applies to a record
/// (`analysis_runs::check_native_header`). Its size does not grow with the rows.
pub(crate) fn header_record(built: &Built<'_>) -> Value {
    let mut out = Vec::new();
    write_record(&mut Writer::new(&mut out, Style::Stored), built, Part::Header, None, &mut |_| unreachable!("the header writes no rows"));
    serde_json::from_slice(&out).expect("the header record is JSON")
}

/// The whole transient record's RFC 8785 canonical text (`canonical_json` of
/// its stored value) streamed into `out`: the project envelope payload's
/// `analysis_run` (each row's checksum streamed again).
pub(crate) fn write_canonical_record<S: Sink>(built: &Built<'_>, out: &mut S) {
    write_record(&mut Writer::new(out, Style::Canonical), built, Part::Stored, None, &mut |row| canonical_sha256(row));
}

/// The source's `model_ref` (the project of the run).
pub(crate) fn model_ref<'a>(built: &Built<'a>) -> &'a str {
    built.model_ref
}

/// The record's stored bytes: the length checked against `max_bytes` and
/// admitted through `reserve` before the buffer is allocated (a record over
/// the bound is refused by name, nothing allocated), then written at that
/// exact capacity, each row's checksum computed again.
pub(crate) fn record_bytes<R>(
    built: &Built<'_>,
    max_bytes: u64,
    reserve: impl FnOnce(u64) -> Result<R, CommandError<String>>,
) -> Result<(Vec<u8>, R), CommandError<String>> {
    if built.record_bytes > max_bytes {
        return Err(CommandError::Command(format!(
            "{RECORD_TOO_LARGE}: run `{}`'s analysis-run record is {} bytes, over the limit of {max_bytes} bytes; nothing was produced.",
            crate::stored_read::shown(built.run_id),
            built.record_bytes
        )));
    }
    let guard = reserve(ingress::allocation_envelope(built.record_bytes).unwrap_or(u64::MAX))?;
    let mut out = Vec::with_capacity(built.record_bytes as usize);
    write_record(&mut Writer::new(&mut out, Style::Stored), built, Part::Stored, None, &mut |row| canonical_sha256(row));
    assert!(out.len() as u64 == built.record_bytes, "the stored record is its counted length");
    Ok((out, guard))
}

/// The preview header's bytes (module docs), the response lease admitted at
/// their exact length through `reserve` before they are written.
pub(crate) fn header_bytes<R>(built: &Built<'_>, job_id: &str, reserve: impl FnOnce(u64) -> Result<R, CommandError<String>>) -> Result<(Vec<u8>, R), CommandError<String>> {
    let mut count = Count::default();
    write_header(&mut Writer::new(&mut count, Style::Stored), built, job_id);
    let guard = reserve(ingress::allocation_envelope(count.0).unwrap_or(u64::MAX))?;
    let mut out = Vec::with_capacity(count.0 as usize);
    write_header(&mut Writer::new(&mut out, Style::Stored), built, job_id);
    debug_assert!(out.len() as u64 == count.0);
    Ok((out, guard))
}

fn write_header<S: Sink>(w: &mut Writer<'_, S>, built: &Built<'_>, job_id: &str) {
    w.raw("{\"carried_members\":[");
    for (at, (key, _)) in built.copies.iter().enumerate() {
        if at > 0 {
            w.raw(",");
        }
        w.string(key);
    }
    w.raw("],\"document_kind\":");
    w.string(DOCUMENT_KIND);
    w.raw(",\"job_id\":");
    w.string(job_id);
    w.raw(",\"producer_diagnostic_count\":");
    w.unsigned(built.producer_diagnostics.len() as u64);
    w.raw(",\"record\":");
    write_record(w, built, Part::Header, None, &mut |_| unreachable!("the header writes no rows"));
    w.raw(",\"record_bytes\":");
    w.unsigned(built.record_bytes);
    w.raw(",\"result_ref_count\":");
    w.unsigned(built.rows.len() as u64);
    w.raw(",\"route\":");
    w.string(built.route.name);
    w.raw(",\"rule_check_status\":");
    w.string(&built.rule_check_status);
    w.raw(",\"rule_pack_ref_diagnostics\":[");
    for (at, item) in built.rule_pack_diagnostics.iter().enumerate() {
        if at > 0 {
            w.raw(",");
        }
        w.value(item);
    }
    w.raw("],\"version\":");
    w.unsigned(VERSION);
    w.raw("}");
}

// ---------------------------------------------------------------------------
// The preview (`buildAnalysisRunPreview`) and its command
// ---------------------------------------------------------------------------

/// The session's input manifest evidence, without its parsed copy.
#[derive(Debug, Deserialize)]
pub(crate) struct ManifestArg {
    pub(crate) manifest_ref: Value,
    pub(crate) manifest_sha256: String,
    pub(crate) canonical_bytes: String,
}

/// `verifyCurrentSessionInputManifest` over the canonical bytes and the
/// manifest parsed from them (which they fix: the TypeScript's manifest and
/// its canonical bytes agree whenever its check passes).
pub(crate) fn verify_manifest(evidence: &ManifestArg, manifest: Option<&Value>) -> Result<(), String> {
    const INCOMPLETE: &str = "INPUT-MANIFEST-EVIDENCE-INCOMPLETE: manifest ref and exact SHA-256 are required.";
    const MISMATCH: &str = "INPUT-MANIFEST-HASH-MISMATCH: manifest transformation requires hash and ref recomputation.";
    let reference = Some(&evidence.manifest_ref);
    if !strict_eq_str(get(reference, "object_type", "inputManifest.manifest_ref")?, "InputManifest") {
        return Err(INCOMPLETE.into());
    }
    let Some(Value::String(ref_text)) = get(reference, "ref", "inputManifest.manifest_ref")? else {
        return Err(shape("inputManifest.manifest_ref.ref is not a string"));
    };
    if js_blank(ref_text) || !sha256_hex_text(&evidence.manifest_sha256) {
        return Err(INCOMPLETE.into());
    }
    let Some(manifest) = manifest else { return Err(MISMATCH.into()) };
    let mut compare = Compare { expected: evidence.canonical_bytes.as_bytes(), at: 0, equal: true };
    Writer::new(&mut compare, Style::Canonical).value(manifest);
    let canonical = compare.whole();
    let sha256 = hex(&Sha256::digest(evidence.canonical_bytes.as_bytes()));
    let model_basis = get(Some(manifest), "model_basis", "inputManifest.manifest")?;
    let model_ref = get(model_basis, "model_ref", "inputManifest.manifest.model_basis")?;
    let payload = get(model_basis, "model_payload", "inputManifest.manifest.model_basis")?;
    let project = get(payload, "project", "inputManifest.manifest.model_basis.model_payload")?;
    let payload_ref = get(project, "id", "inputManifest.manifest.model_basis.model_payload.project")?;
    let Some(Value::String(model_ref_text)) = model_ref else { return Err(shape("inputManifest.manifest.model_basis.model_ref is not a string")) };
    let expected_ref = input_manifest_ref(model_ref_text, &sha256);
    if js_blank(model_ref_text)
        || !strict_eq(model_ref, payload_ref)
        || !canonical
        || sha256 != evidence.manifest_sha256
        || expected_ref["ref"].as_str() != Some(ref_text.as_str())
    {
        return Err(MISMATCH.into());
    }
    Ok(())
}

/// `modelLoadBasisRefs(manifest.model_basis.model_payload)`: the model's load
/// cases, then its combinations.
fn model_load_basis_refs(manifest: &Value) -> Result<Value, String> {
    let payload = manifest.pointer("/model_basis/model_payload");
    let mut refs = Vec::new();
    let mut each = |entries: &Value, object_type: &str, what: &str| -> Result<(), String> {
        let Value::Array(entries) = entries else { return Err(shape(&format!("{what} is not an array"))) };
        for entry in entries {
            let id = get(Some(entry), "id", what)?;
            let mut reference = Map::new();
            reference.insert("object_type".into(), json!(object_type));
            if let Some(id) = id {
                reference.insert("ref".into(), id.clone());
            }
            refs.push(Value::Object(reference));
        }
        Ok(())
    };
    let cases = get(payload, "load_cases", "the model payload")?.ok_or_else(|| shape("the model payload's load_cases is not an array"))?;
    each(cases, "LoadCase", "the model payload's load_cases")?;
    match get(payload, "combinations", "the model payload")? {
        None | Some(Value::Null) => {}
        Some(combinations) => each(combinations, "Combination", "the model payload's combinations")?,
    }
    Ok(Value::Array(refs))
}

/// `appliedRuleCheckStatus(solveRuleCheck, aggregate)`'s choice: a
/// recognized aggregate, otherwise the solve's own status (`None`: absent).
fn applied_rule_check_status<'a>(aggregate: Option<&'a Value>, solve: Option<&'a Value>) -> Option<&'a Value> {
    match aggregate {
        Some(Value::String(text)) if RULE_CHECK_RUN_STATUSES.contains(&text.as_str()) => aggregate,
        _ => solve,
    }
}

/// `buildAnalysisRunPreview` after its manifest check, on a parsed source:
/// the solve's status, the dispatch, the model's load basis (written to
/// `load_basis`), the route's reader, then the record.
#[allow(clippy::too_many_arguments)]
pub(crate) fn preview_parsed<'a>(
    source: &'a mut Value,
    evidence: &'a ManifestArg,
    manifest_sha256: &'a Value,
    manifest: &'a Value,
    aggregate: Option<&'a Value>,
    exclusions: &'a [Exclusion],
    load_basis: &'a mut Value,
    reserve: &mut dyn FnMut(u64) -> Result<Option<Lease>, CommandError<String>>,
) -> Result<Built<'a>, CommandError<String>> {
    let refuse = CommandError::Command;
    let status = get(Some(&*source), "status", "result").map_err(refuse)?;
    get(status, "rule_check", "result.status").map_err(refuse)?;
    let route = dispatch(source).map_err(refuse)?;
    *load_basis = model_load_basis_refs(manifest).map_err(refuse)?;
    validate_route(source, &route).map_err(refuse)?;
    let source: &'a Value = source;
    let load_basis: &'a Value = load_basis;
    let inputs = BuildInputs {
        manifest: ManifestEvidence { manifest_ref: &evidence.manifest_ref, manifest_sha256, manifest },
        rule_check_status: applied_rule_check_status(aggregate, get_opt(source.get("status"), "rule_check")),
        load_basis_refs: Some(load_basis),
        rule_pack_exclusions: exclusions,
    };
    build(source, route, &inputs, reserve)
}

/// The executor's part: O(1) checks, the copy's reservation and the copy
/// (`native_verdict::reserve`'s steps under this command's name).
pub(crate) fn reserve(registry: &SolveJobRegistry, body: &InvokeBody, headers: &HeaderMap) -> Result<(Vec<u8>, Lease), CommandError<String>> {
    let raw = command_ingress::raw_args_body(body, headers, COMMAND)?;
    let lease = registry.acquire_interactive_ingress_capped(ingress::allocation_envelope(raw.len() as u64).unwrap_or(u64::MAX))?;
    Ok((raw.to_vec(), lease))
}

/// The command: [`reserve`] on the caller's executor, then [`run`] in a
/// blocking task that owns the copy, its lease and a registry handle.
pub(crate) async fn analysis_run_preview(registry: &SolveJobRegistry, body: &InvokeBody, headers: &HeaderMap) -> Result<Vec<u8>, CommandError<String>> {
    let (raw, lease) = reserve(registry, body, headers)?;
    let registry = registry.clone();
    tauri::async_runtime::spawn_blocking(move || run(registry, raw, lease))
        .await
        .unwrap_or_else(|_| Err(CommandError::Command(PREVIEW_READ_FAILED.into())))
}

/// The header as Tauri's raw byte body: the adapter receives an ArrayBuffer,
/// not a value Tauri parsed first.
pub(crate) fn response(bytes: Vec<u8>) -> tauri::ipc::Response {
    tauri::ipc::Response::new(bytes)
}

/// The decoded arguments, the held result and the parsed manifest and
/// source, with their leases: what both the preview and a later persistence
/// consumer of [`record_bytes`] read.
pub(crate) struct Prepared {
    pub(crate) job_id: String,
    pub(crate) evidence: ManifestArg,
    pub(crate) manifest_sha256: Value,
    pub(crate) manifest: Option<Value>,
    pub(crate) aggregate: Option<Value>,
    pub(crate) exclusions: Vec<Exclusion>,
    pub(crate) held: std::sync::Arc<crate::solve_jobs::HeldResult>,
    /// Held, never read: the args lease lives as long as the decoded arguments.
    #[allow(dead_code)]
    pub(crate) args_lease: Lease,
}

/// The arguments' admission and decoding, the held result, and the input
/// manifest's priced parse (the args lease grows by it before it is parsed;
/// canonical bytes that are not JSON leave no manifest, which the check
/// refuses).
pub(crate) fn admit(registry: &SolveJobRegistry, raw: Vec<u8>, copy: Lease) -> Result<Prepared, CommandError<String>> {
    admit_for(registry, raw, copy, COMMAND, |_| Ok(())).map(|(prepared, ())| prepared)
}

/// [`admit`] for another command reading the same arguments and its own
/// (`extra` decodes them before the held result is found), under that
/// command's name. The persistence commands (`native_persistence`) admit here.
pub(crate) fn admit_for<E>(
    registry: &SolveJobRegistry,
    raw: Vec<u8>,
    copy: Lease,
    command: &'static str,
    extra: impl FnOnce(&mut command_ingress::Args) -> Result<E, CommandError<String>>,
) -> Result<(Prepared, E), CommandError<String>> {
    let copy_bytes = copy.bytes();
    let (mut args, mut lease) = command_ingress::admit_raw(registry, &raw, command, ArgsDecoding::Moved, |bytes| {
        let mut lease = copy;
        lease.grow_to(copy_bytes.saturating_add(bytes)).map_err(|refusal| registry.refuse(refusal))?;
        Ok(lease)
    })?;
    drop(raw);
    let job_id: String = args.decode("jobId")?;
    let evidence: ManifestArg = args.decode("inputManifest")?;
    let aggregate: Option<String> = args.decode("ruleCheckAggregate")?;
    let exclusions: Option<Vec<Exclusion>> = args.decode("rulePackExclusions")?;
    let extra = extra(&mut args)?;
    drop(args);
    let held = registry.held_result(&job_id)?;
    // C1 slice 6: a restored stored result has its stored record; no fresh
    // transient record is previewed over it under the session's manifest.
    if command == COMMAND {
        if let Some(crate::solve_jobs::ResultOrigin::Restored(_)) = registry.result_origin(&job_id) {
            return Err(CommandError::Command(format!(
                "{PREVIEW_READ_FAILED}: `{}` holds a restored stored result; its stored analysis-run record is its record, and no fresh record is previewed over it.",
                crate::stored_read::shown(&job_id)
            )));
        }
    }
    let manifest = match command_ingress::text_parse_bytes(&evidence.canonical_bytes, ArgsDecoding::Moved) {
        Ok(bytes) => {
            // Capped as the args lease was admitted (`acquire_interactive_capped`).
            lease.grow_to(lease.bytes().saturating_add(bytes)).map_err(|refusal| CommandError::Ingress(registry.refuse(refusal)))?;
            serde_json::from_str::<Value>(&evidence.canonical_bytes).ok()
        }
        Err(_) => None,
    };
    Ok((
        Prepared {
            job_id,
            manifest_sha256: Value::String(evidence.manifest_sha256.clone()),
            evidence,
            manifest,
            aggregate: aggregate.map(Value::String),
            exclusions: exclusions.unwrap_or_default(),
            held,
            args_lease: lease,
        },
        extra,
    ))
}

/// The held text parsed under the parse lease of its materialization M,
/// admitted before the parse allocates. A text the readers cannot read is
/// [`SOURCE_CONTRACT_UNSUPPORTED`], as the dispatch reads it through the
/// wasm engine; the `RawValue` token key keeps slice 1's refusal
/// (`native_verdict` module docs).
pub(crate) fn parse_source(registry: &SolveJobRegistry, text: &str) -> Result<(Value, Lease), CommandError<String>> {
    let bound = match ingress::prescan_allowances(text.as_bytes()) {
        Ok((bound, _)) => bound,
        Err(invalid) if invalid.reason == "reserved RawValue token key" => return Err(CommandError::Command(native_verdict::prescan_refusal("source", invalid))),
        Err(_) => return Err(CommandError::Command(SOURCE_CONTRACT_UNSUPPORTED.into())),
    };
    let reading = registry.acquire_result_parse(bound.materialization_bytes)?;
    #[cfg(test)]
    tests::gate::pass(tests::gate::key(registry, "read"));
    let source = serde_json::from_str(text).map_err(|_| CommandError::Command(SOURCE_CONTRACT_UNSUPPORTED.into()))?;
    Ok((source, reading))
}

/// The working lease (module docs): the fixed allowance and the canonical
/// writer's key-order scratch for the deepest of the values it writes.
pub(crate) fn working_lease(registry: &SolveJobRegistry, values: &[&Value]) -> Result<Lease, CommandError<String>> {
    let scratch = values.iter().map(|value| sort_scratch_bytes(value)).max().unwrap_or(0);
    Ok(registry.acquire_result_parse(WORKING_ALLOWANCE_BYTES.saturating_add(scratch))?)
}

/// Admission, the manifest check, the parse and the passes, then `finish`
/// on the record they establish, with the leases held until it returns.
fn with_built<T>(
    registry: &SolveJobRegistry,
    raw: Vec<u8>,
    copy: Lease,
    finish: impl FnOnce(&Built<'_>, &str) -> Result<T, CommandError<String>>,
) -> Result<T, CommandError<String>> {
    #[cfg(test)]
    tests::gate::pass(tests::gate::key(registry, "admit"));
    let prepared = admit(registry, raw, copy)?;
    let mut values = vec![&prepared.evidence.manifest_ref];
    values.extend(prepared.manifest.as_ref());
    let working = working_lease(registry, &values)?;
    verify_manifest(&prepared.evidence, prepared.manifest.as_ref()).map_err(CommandError::Command)?;
    let (mut source, reading) = parse_source(registry, prepared.held.text())?;
    let scratch = working_lease(registry, &[&source])?;
    #[cfg(test)]
    if tests::PANIC_IN_BUILD.with(std::cell::Cell::get) {
        panic!("injected panic inside the build");
    }
    let mut load_basis = Value::Null;
    let mut reserve = |bytes: u64| registry.acquire_result_parse(bytes).map(Some).map_err(CommandError::from);
    let built = preview_parsed(
        &mut source,
        &prepared.evidence,
        &prepared.manifest_sha256,
        prepared.manifest.as_ref().expect("verified"),
        prepared.aggregate.as_ref(),
        &prepared.exclusions,
        &mut load_basis,
        &mut reserve,
    )?;
    let finished = finish(&built, &prepared.job_id);
    drop(built);
    drop((scratch, working, reading, source, prepared));
    finished
}

/// The blocking part of the command: the record's preview header.
pub(crate) fn run(registry: SolveJobRegistry, raw: Vec<u8>, copy: Lease) -> Result<Vec<u8>, CommandError<String>> {
    with_built(&registry, raw, copy, |built, job_id| {
        let (response, written) = header_bytes(built, job_id, |bytes| Ok(registry.acquire_result_parse(bytes)?))?;
        drop(written);
        Ok(response)
    })
}

/// The bounded native route to the record's exact stored bytes, for the
/// persistence that follows (Record Run and Save Project, slice 5; not a
/// webview command): the same arguments and checks as the preview, then
/// [`record_bytes`] under `max_bytes` (the caller's bound, for instance the
/// connection's SQLite length limit), the record lease returned with the
/// bytes so it lives exactly as long.
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn run_record_bytes_within(registry: SolveJobRegistry, raw: Vec<u8>, copy: Lease, max_bytes: u64) -> Result<(Vec<u8>, Lease), CommandError<String>> {
    with_built(&registry, raw, copy, |built, _| record_bytes(built, max_bytes, |bytes| Ok(registry.acquire_result_parse(bytes)?)))
}

#[cfg(test)]
#[path = "analysis_run_v03_build_tests.rs"]
pub(crate) mod tests;

