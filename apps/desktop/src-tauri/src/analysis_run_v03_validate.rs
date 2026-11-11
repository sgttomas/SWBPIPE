//! C1 slice 6 (DEL-04-05 `c1-native-custody-plan.md`): a saved or recorded
//! run's historical findings, computed in this process from the stored
//! texts held in native custody, so a reopened run is inspected without its
//! result rows or result refs crossing to the webview.
//!
//! - **The same findings.** This is `buildHistoricalRunContext` (a project's
//!   saved run) and `buildRecordedRunContext` (a recorded run) of
//!   `HistoricalRunContext.tsx`, with the record validator they call
//!   (`validateAnalysisRunV03`, `verifyAnalysisRunRecord` and
//!   `validateRecordedBinding` of `analysisRunCompatibility.ts`), in Rust.
//!   The TypeScript stays the oracle: `historicalReopenParity.test.ts` writes
//!   its findings over the full rows for every case of
//!   `historical_reopen_cases.json`, and `stored_custody_tests.rs` requires
//!   these functions to give the same list, in the same order, on the stored
//!   texts.
//! - **Readings.** Every member read follows JavaScript: a member read on
//!   `undefined` or `null` throws a `TypeError` (here [`TYPE_ERROR`], which is
//!   not a named refusal, so the caller's catch turns it into the finding the
//!   TypeScript gives an unnamed throw); `typeof … === "object"` admits
//!   arrays; numbers are binary64 (`same_js_data`). The record builder's
//!   readers, table interpretation, load-basis rules and canonical writer
//!   are reused from `analysis_run_v03_build` unchanged (explicit imports).
//! - **Standing.** `numericalResultStanding(result, model)` of an unregistered
//!   (saved or recorded) result is Rust's dispatch standing for the model
//!   (`reader_verdict::dispatch`, the function the wasm export wraps), or the
//!   retained routes' `RETAINED_PRECISION_VALIDATION_REQUIRED` (saved and
//!   recorded data never register).
//! - **Recorded before RT-A4** (RT-A4 §10.4, DEL-05-02
//!   `absolute-sum-srss-combinations.md`, "Recorded results from before
//!   RT-A4"). A current (0.3.0) record whose `reproducibility.semantic_contract`
//!   names a registry variant (`recorded_tables::VARIANTS`, by exact
//!   `{id, sha256}`; `recordedPreRtA4RecordBinding`) is read under it, as the
//!   webview's contexts read it: its standing findings are the historical
//!   reader's (`rv::dispatch_recorded`), kept apart from `findings` with the
//!   table they were read under (`recorded_table_read`,
//!   `historical_reader_findings`); a successor's receipt is read by
//!   `rv::retained_recorded`; and the record validator reads the statement
//!   through the recorded dispatch, `rv::validate_recorded` and the variant's
//!   table. Every check's failure (`check_findings`, `reopenFindings`'s
//!   `fail`) stays an ordinary finding, and a refused recorded read is one.
//!   Current reads and admission are unchanged: only these reopen readings
//!   select a variant, each inside `recorded_tables::read_recorded`, and the
//!   read grants no standing, current status or authority.
//! - **Nothing changes.** The stored texts are read, never rewritten; the
//!   findings grant no standing, current status, overlay, rule check,
//!   comparison or report readiness. The retained reader reads a value written
//!   back with JavaScript's numbers, as the builder's does, in place on the
//!   parsed copy, never on the stored bytes.
use crate::analysis_run_v03_build::{
    canonical_sha256, dispatch, duplicate_ids, expected_load_basis, get_opt, has_outside_profile, hex, interpret, js_string, keep_negative_zero_js_numbers,
    nullish, reader_refusal, rows_of, strict_eq, strict_eq_str, truthy, validate_route, Route, Sink, Style, Writer, OUTSIDE_PROFILE, RETAINED_RECEIPT_MISMATCH,
    RULE_STATUSES, SOURCE_CONTRACT_UNSUPPORTED,
};
use crate::analysis_runs::{
    manifest_excluded_rule_packs, manifest_members, rule_pack_identity, rule_pack_ref_diagnostic, RULE_PACKS_NOT_RESOLVED, RULE_PACK_REF_CHECKSUM_MISMATCH,
    RULE_PACK_REF_UNRESOLVED,
};
use open_pipe_stress_canonical_json::{ecma_number_to_string, CHECKED_PROFILE_V1};
use open_pipe_stress_result_export::recorded_tables::{self, RecordedVariant};
use open_pipe_stress_result_export::{reader_verdict as rv, retained_precision as rp};
use serde::Serialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use crate::native_persistence::same_js_data;

/// JavaScript's `TypeError` (a member read on `undefined` or `null`): not a
/// named refusal.
pub(crate) const TYPE_ERROR: &str = "TypeError: a member was read on undefined or null";
const ANALYSIS_RUN_V03_TEXT: &str = "0.3.0";
const CREATED_AT_UNAVAILABLE: &str = "created_at_unavailable";
const RETAINED_PRECISION_VALIDATION_REQUIRED: &str = "RETAINED_PRECISION_VALIDATION_REQUIRED";
const ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH: &str = "ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH";
const ANALYSIS_RUN_SCHEMA_VERSION_RETIRED: &str = "ANALYSIS_RUN_SCHEMA_VERSION_RETIRED";
const RESULT_DIMENSIONS: [&str; 15] = [
    "dimensionless", "length", "angle", "force", "moment", "stress", "area", "section_modulus", "second_moment_area", "ratio", "time",
    "temperature", "pressure", "linear_stiffness", "rotational_stiffness",
];

// ---------------------------------------------------------------------------
// JavaScript's readings
// ---------------------------------------------------------------------------

/// `base.key`, throwing as JavaScript does on `undefined` or `null`.
fn at<'v>(base: Option<&'v Value>, key: &str) -> Result<Option<&'v Value>, String> {
    match base {
        None | Some(Value::Null) => Err(TYPE_ERROR.into()),
        Some(Value::Object(map)) => Ok(map.get(key)),
        Some(_) => Ok(None),
    }
}

/// `typeof value === "object" && value !== null` (arrays included).
fn objectish(value: Option<&Value>) -> bool {
    matches!(value, Some(Value::Object(_) | Value::Array(_)))
}

/// `isRecord`: a plain object.
fn record(value: Option<&Value>) -> bool {
    matches!(value, Some(Value::Object(_)))
}

fn is_str(value: Option<&Value>) -> bool {
    matches!(value, Some(Value::String(_)))
}

fn is_array(value: Option<&Value>) -> bool {
    matches!(value, Some(Value::Array(_)))
}

/// `same(a, b)` of the validator: `undefined` only equals `undefined`;
/// otherwise both values' `checkedJsonText` (which throws first for `a`, then
/// for `b`) and their equality as data.
fn same(a: Option<&Value>, b: Option<&Value>) -> Result<bool, String> {
    match (a, b) {
        (None, _) | (_, None) => Ok(a.is_none() && b.is_none()),
        (Some(a), Some(b)) => {
            checked(a)?;
            checked(b)?;
            Ok(same_js_data(a, b))
        }
    }
}

/// `checkedJsonText`'s refusal for a parsed JSON value, if any.
fn checked(value: &Value) -> Result<(), String> {
    match has_outside_profile(value) {
        true => Err(OUTSIDE_PROFILE.into()),
        false => Ok(()),
    }
}

/// `canonicalSha256HexCheckedV1(value)`: the checked text's SHA-256, streamed.
pub(crate) fn checked_sha256(value: &Value) -> Result<String, String> {
    checked(value)?;
    Ok(canonical_sha256(value))
}

/// `canonicalSha256Hex(value)` (RFC 8785, no profile check), streamed.
pub(crate) fn jcs_sha256(value: &Value) -> String {
    let mut hasher = Sha256::new();
    js_canonical_value(&mut Writer::new(&mut hasher, Style::Canonical), value);
    hex(&hasher.finalize())
}

/// Historical JSON is read by JavaScript before RFC 8785 hashing. Even serde's
/// exact integer variants therefore pass through binary64. This does not
/// normalize the stored value or change the checked-profile writer.
fn js_canonical_value<S: Sink>(w: &mut Writer<'_, S>, value: &Value) {
    match value {
        Value::Number(number) => w.raw(&ecma_number_to_string(number.as_f64().expect("a JSON number is binary64-readable"))),
        Value::Array(items) => {
            w.raw("[");
            for (index, item) in items.iter().enumerate() {
                if index > 0 { w.raw(","); }
                js_canonical_value(w, item);
            }
            w.raw("]");
        }
        Value::Object(map) => {
            let mut keys: Vec<_> = map.keys().collect();
            keys.sort_by(|a, b| a.encode_utf16().cmp(b.encode_utf16()));
            w.raw("{");
            for (index, key) in keys.into_iter().enumerate() {
                if index > 0 { w.raw(","); }
                w.string(key);
                w.raw(":");
                js_canonical_value(w, &map[key]);
            }
            w.raw("}");
        }
        _ => w.value(value),
    }
}

/// The error codes the validator's caller shows as they are (`NAMED_REFUSAL`:
/// `^[A-Z][A-Z0-9]*(?:[_-][A-Z0-9]+)+(?![\w-])`).
pub(crate) fn named_refusal(message: &str) -> bool {
    let bytes = message.as_bytes();
    let upper_digit = |b: u8| b.is_ascii_uppercase() || b.is_ascii_digit();
    if bytes.first().is_none_or(|b| !b.is_ascii_uppercase()) {
        return false;
    }
    // Greedy match with backtracking over the separators: find the longest
    // prefix of the form, then require the lookahead at some admissible end.
    let mut ends = Vec::new();
    let mut index = 1;
    while index < bytes.len() && upper_digit(bytes[index]) {
        index += 1;
    }
    let mut groups = 0;
    loop {
        if index < bytes.len() && (bytes[index] == b'_' || bytes[index] == b'-') && index + 1 < bytes.len() && upper_digit(bytes[index + 1]) {
            index += 1;
            while index < bytes.len() && upper_digit(bytes[index]) {
                index += 1;
            }
            groups += 1;
            ends.push(index);
        } else {
            break;
        }
    }
    let word = |b: u8| b.is_ascii_alphanumeric() || b == b'_' || b == b'-';
    // An end admits the lookahead when the next byte is not a word character
    // or `-`. Backtracking can end a group early only inside its run of
    // capitals and digits, after which the next byte is again one of them.
    groups > 0 && ends.iter().rev().any(|&end| end >= bytes.len() || !word(bytes[end]))
}

// ---------------------------------------------------------------------------
// Shapes (`HistoricalRunContext.tsx`)
// ---------------------------------------------------------------------------

/// `isHistoricalMechanicsEvidence`.
pub(crate) fn historical_evidence(value: &Value) -> bool {
    let status = value.get("status");
    record(Some(value))
        && is_str(value.get("run_id"))
        && is_str(value.get("model_ref"))
        && record(status)
        && is_str(get_opt(status, "mechanics"))
        && is_str(get_opt(status, "rule_check"))
        && is_array(value.get("results"))
        && is_array(value.get("diagnostics"))
}

/// `isRenderableMechanicsResult`, one row and one diagnostic at a time.
pub(crate) fn renderable(value: &Value) -> bool {
    let status = value.get("status");
    if !(record(Some(value))
        && is_str(value.get("schema_version"))
        && is_str(value.get("document_kind"))
        && is_str(value.get("run_id"))
        && is_str(value.get("model_ref"))
        && record(status)
        && is_str(get_opt(status, "mechanics"))
        && is_str(get_opt(status, "rule_check"))
        && record(value.get("summary")))
    {
        return false;
    }
    let (Some(Value::Array(rows)), Some(Value::Array(diagnostics))) = (value.get("results"), value.get("diagnostics")) else { return false };
    let row_safe = |row: &Value| {
        if !(record(Some(row))
            && is_str(row.get("id"))
            && is_str(row.get("kind"))
            && row.get("value").and_then(Value::as_f64).is_some_and(f64::is_finite)
            && is_str(row.get("unit"))
            && is_str(row.get("entity_ref")))
        {
            return false;
        }
        if let Some(dimension) = row.get("dimension") {
            if !dimension.as_str().is_some_and(|text| RESULT_DIMENSIONS.contains(&text)) {
                return false;
            }
        }
        if let Some(basis) = row.get("basis_ref") {
            if !(record(Some(basis)) && is_str(basis.get("ref_type")) && is_str(basis.get("ref_id"))) {
                return false;
            }
        }
        if let Some(refs) = row.get("source_result_refs") {
            if !refs.as_array().is_some_and(|items| items.iter().all(Value::is_string)) {
                return false;
            }
        }
        if let Some(metadata) = row.get("metadata") {
            if !(record(Some(metadata)) && ["component", "coordinate_system", "location", "basis", "sign_convention"].iter().all(|key| is_str(metadata.get(*key)))) {
                return false;
            }
        }
        true
    };
    let diagnostic_safe = |item: &Value| {
        record(Some(item))
            && is_str(item.get("code"))
            && ["info", "warning", "error", "blocking"].contains(&js_string(item.get("severity")).as_str())
            && is_str(item.get("message"))
            && item.get("id").is_none_or(Value::is_string)
            && item.get("affected_refs").is_none_or(|refs| refs.as_array().is_some_and(|items| items.iter().all(Value::is_string)))
    };
    rows.iter().all(row_safe) && diagnostics.iter().all(diagnostic_safe)
}

/// `sourceContract(source)` as the header dispatch reads it, for
/// `isRetainedRoute`.
fn retained_route(source: &Value) -> bool {
    matches!(rv::route(source), Ok((name, _)) if matches!(name, "retained_preview_physics" | "retained_physics"))
}

/// `numericalResultStanding(result, model).findings` for an unregistered
/// result: the retained routes' registration requirement, or Rust's dispatch
/// standing for the model.
pub(crate) fn standing(source: &Value, model: Option<&Value>) -> Value {
    if retained_route(source) {
        return json!({"route": rv::route(source).map(|(name, _)| name).unwrap_or("unsupported"), "status": "needs_recompute", "eligible": false, "findings": [RETAINED_PRECISION_VALIDATION_REQUIRED]});
    }
    let model = model.filter(|model| !model.is_null());
    let dispatched = rv::dispatch(source, model, None);
    let standing = &dispatched["standing"];
    json!({"route": standing["route"], "status": standing["status"], "eligible": standing["eligible"], "findings": standing["findings"]})
}

fn standing_findings(standing: &Value) -> Vec<String> {
    standing["findings"].as_array().map(|items| items.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default()
}

/// `recordedPreRtA4RecordBinding(record)`: the registry variant a
/// record's `analysis_run.reproducibility.semantic_contract` names by exact
/// `{id, sha256}`, or none.
pub(crate) fn recorded_record_binding(record: &Value) -> Option<&'static RecordedVariant> {
    let binding = record.get("analysis_run")?.get("reproducibility")?.get("semantic_contract")?;
    recorded_tables::variant_of(binding)
}

/// A variant's binding, `{id, sha256}`.
pub(crate) fn binding_value(variant: &RecordedVariant) -> Value {
    json!({"id": variant.id, "sha256": variant.sha256})
}

/// A registry variant by its identity and digest (what a held restored
/// handle keeps: the variant itself has no comparison).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct RecordedTable {
    pub(crate) id: &'static str,
    pub(crate) sha256: &'static str,
}

impl RecordedTable {
    pub(crate) fn of(variant: &'static RecordedVariant) -> Self {
        Self { id: variant.id, sha256: variant.sha256 }
    }
    /// The registry member (always one: a table is made only from one).
    pub(crate) fn variant(self) -> &'static RecordedVariant {
        recorded_tables::VARIANTS.iter().find(|variant| variant.id == self.id && variant.sha256 == self.sha256).expect("a registry member")
    }
    pub(crate) fn binding(self) -> Value {
        json!({"id": self.id, "sha256": self.sha256})
    }
}

/// `recordedReaderDispatch(source, binding, model)`: Rust's recorded dispatch.
pub(crate) fn recorded_dispatch(source: &Value, variant: &RecordedVariant, model: Option<&Value>) -> Value {
    rv::dispatch_recorded(source, &binding_value(variant), model, None)
}

/// A route name a dispatch states, as the static name the record readers use.
fn route_name(route: &Value) -> &'static str {
    match route.as_str() {
        Some("preview_physics") => "preview_physics",
        Some("pressure") => "pressure",
        Some("pressure_source") => "pressure_source",
        Some("retained_preview_physics") => "retained_preview_physics",
        Some("retained_physics") => "retained_physics",
        _ => "unsupported",
    }
}

fn retained_name(name: &str) -> bool {
    matches!(name, "retained_preview_physics" | "retained_physics")
}

/// `recordedResultStanding(result, binding, model)`: under the record's
/// variant, the ordinary routes take the recorded dispatch's standing; a
/// successor's is its registration's (a saved statement never registers); the
/// pressure-source route's is the current one.
fn recorded_standing(source: &Value, variant: &RecordedVariant, model: Option<&Value>) -> Value {
    let route = route_name(&recorded_dispatch(source, variant, None)["route"]);
    if retained_name(route) {
        return json!({"route": route, "status": "needs_recompute", "eligible": false, "findings": [RETAINED_PRECISION_VALIDATION_REQUIRED]});
    }
    if route == "pressure_source" {
        return standing(source, model);
    }
    let dispatched = recorded_dispatch(source, variant, model.filter(|model| !model.is_null()));
    let standing = &dispatched["standing"];
    json!({"route": standing["route"], "status": standing["status"], "eligible": standing["eligible"], "findings": standing["findings"]})
}

/// `validateRetainedPrecisionRecorded(source, binding)`'s refusal (its
/// message), on the parsed result written back with JavaScript's numbers, as
/// [`retained_refusal`] reads the current one.
fn retained_recorded_refusal(source: &mut Value, variant: &RecordedVariant) -> Option<String> {
    if has_outside_profile(source) {
        return Some(RETAINED_RECEIPT_MISMATCH.into());
    }
    keep_negative_zero_js_numbers(source);
    let outcome = rv::retained_recorded(source, &binding_value(variant), None);
    (outcome["ok"] != Value::Bool(true)).then(|| outcome["error"]["code"].as_str().unwrap_or_default().to_string())
}

/// A reopen's findings in their scopes (RT-A4 §10.4 display rule;
/// `reopenFindings` of `HistoricalRunContext.tsx`): `findings` shows every
/// check's finding and, under the current tables, the current standing's;
/// `checks` holds the checks' alone; `historical` the historical reader's
/// under a recorded table.
struct Found {
    findings: Vec<String>,
    checks: Vec<String>,
    historical: Vec<String>,
}

impl Found {
    fn new(initial: &[&str]) -> Self {
        let initial: Vec<String> = initial.iter().map(|code| code.to_string()).collect();
        Self { findings: initial.clone(), checks: initial, historical: Vec::new() }
    }
    /// A check's finding: always an ordinary finding.
    fn fail(&mut self, code: impl Into<String>) {
        let code = code.into();
        self.checks.push(code.clone());
        self.findings.push(code);
    }
    /// `fail(code, true)`: a code already found once is added once only.
    fn fail_once(&mut self, code: impl Into<String>) {
        let code = code.into();
        if self.checks.contains(&code) {
            return;
        }
        self.checks.push(code.clone());
        if !self.findings.contains(&code) {
            self.findings.push(code);
        }
    }
    /// `readStanding(found, result, model, binding)`: the statement's standing
    /// findings in their scope; a refused recorded dispatch is a check's
    /// finding. Returns the standing read.
    fn read_standing(&mut self, source: &Value, model: Option<&Value>, recorded: Option<&'static RecordedVariant>) -> Value {
        let Some(variant) = recorded else {
            let read = standing(source, model);
            self.findings.extend(standing_findings(&read));
            return read;
        };
        let dispatched = recorded_dispatch(source, variant, None);
        let refusal = (dispatched["ok"] != Value::Bool(true)).then(|| dispatched["error"]["code"].as_str().unwrap_or("RESULT_READER_UNAVAILABLE").to_string());
        if let Some(refusal) = &refusal {
            self.fail(refusal.clone());
        }
        let read = recorded_standing(source, variant, model);
        self.historical.extend(standing_findings(&read).into_iter().filter(|finding| Some(finding) != refusal.as_ref()));
        read
    }
}

/// `validateRetainedPrecision(source)`'s refusal (its message), on a copy of
/// the parsed result written back with JavaScript's numbers (`source` is
/// rewritten in place, numbers only, as the builder's retained reader does).
fn retained_refusal(source: &mut Value) -> Option<String> {
    if has_outside_profile(source) {
        return Some(RETAINED_RECEIPT_MISMATCH.into());
    }
    keep_negative_zero_js_numbers(source);
    rp::validate(source, None).err().map(|error| error.code)
}

/// `isRetainedRoute(recordedBinding ? recordedReaderDispatch(result, binding).route : sourceContract(result))`.
fn reopen_retained_route(source: &Value, recorded: Option<&'static RecordedVariant>) -> bool {
    match recorded {
        Some(variant) => retained_name(route_name(&recorded_dispatch(source, variant, None)["route"])),
        None => retained_route(source),
    }
}

/// The successor's reader on a reopen: `validateRetainedPrecisionRecorded` under a variant,
/// else `validateRetainedPrecision`.
fn reopen_retained_refusal(source: &mut Value, recorded: Option<&'static RecordedVariant>) -> Option<String> {
    match recorded {
        Some(variant) => retained_recorded_refusal(source, variant),
        None => retained_refusal(source),
    }
}

/// `sameSavedReceipt(copy, receipt)`: both checked and equal as data.
fn same_saved_receipt(copy: Option<&Value>, receipt: Option<&Value>) -> bool {
    match (copy, receipt) {
        (Some(copy), Some(receipt)) => checked(copy).is_ok() && checked(receipt).is_ok() && same_js_data(copy, receipt),
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// The record validator (`analysisRunCompatibility.ts`)
// ---------------------------------------------------------------------------

/// `verifyAnalysisRunRecord(record)` on a 0.3.0 record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Verification {
    Match,
    Mismatch,
    Unverifiable,
}

/// `verifyAnalysisRunRecord`: the record's single `analysis_run_record`
/// checksum against its projection's checked hash. `Err` where the
/// TypeScript throws.
pub(crate) fn verify_record(record: &Value) -> Result<Verification, String> {
    let version = at(Some(record), "schema_version")?;
    match version.and_then(Value::as_str) {
        Some("0.1.0" | "0.2.0") => {
            return Err(format!(
                "{ANALYSIS_RUN_SCHEMA_VERSION_RETIRED}: analysis-run {} is no longer read; re-run the analysis to record a current {ANALYSIS_RUN_V03_TEXT} analysis run",
                js_string(version)
            ))
        }
        Some(ANALYSIS_RUN_V03_TEXT) => {}
        _ => return Err(format!("ANALYSIS-RUN-SCHEMA-VERSION-UNSUPPORTED: {}", js_string(version))),
    }
    let run = at(Some(record), "analysis_run")?;
    let Some(Value::Array(hashes)) = at(run, "hashes")? else { return Err(TYPE_ERROR.into()) };
    let mut claims = Vec::new();
    for hash in hashes {
        if strict_eq_str(at(Some(hash), "payload_scope")?, "analysis_run_record") {
            claims.push(hash);
        }
    }
    if claims.len() != 1 {
        return Ok(Verification::Unverifiable);
    }
    let claim = claims[0];
    let payload_ref = at(Some(claim), "payload_ref")?;
    if !strict_eq_str(at(Some(claim), "algorithm")?, "sha256")
        || !strict_eq_str(at(Some(claim), "canonicalization")?, CHECKED_PROFILE_V1)
        || !strict_eq_str(at(payload_ref, "object_type")?, "AnalysisRun")
        || !strict_eq(at(payload_ref, "ref")?, at(run, "run_id")?)
    {
        return Ok(Verification::Mismatch);
    }
    let expected = projection_checked_sha256(record)?;
    Ok(if strict_eq_str(at(Some(claim), "value")?, &expected) { Verification::Match } else { Verification::Mismatch })
}

/// `canonicalSha256HexCheckedV1(analysisRecordProjection(record))`, streamed:
/// the record's canonical text with its one `analysis_run_record` checksum
/// left out, never built as a copy.
pub(crate) fn projection_checked_sha256(record: &Value) -> Result<String, String> {
    checked(record)?;
    let mut hasher = Sha256::new();
    write_projection(&mut Writer::new(&mut hasher, Style::Canonical), record, 0);
    Ok(hex(&hasher.finalize()))
}

/// The canonical writer over `record`, leaving out `analysis_run.hashes`'
/// `analysis_run_record` entries (depth 0 is the record, 1 its
/// `analysis_run`).
fn write_projection<S: Sink>(w: &mut Writer<'_, S>, value: &Value, depth: usize) {
    let Value::Object(map) = value else { return w.value(value) };
    if depth > 1 {
        return w.value(value);
    }
    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort_by(|a, b| a.encode_utf16().cmp(b.encode_utf16()));
    w.raw("{");
    for (at, key) in keys.into_iter().enumerate() {
        if at > 0 {
            w.raw(",");
        }
        w.string(key);
        w.raw(":");
        let member = &map[key];
        match (depth, key.as_str(), member) {
            (0, "analysis_run", _) => write_projection(w, member, 1),
            (1, "hashes", Value::Array(items)) => {
                w.raw("[");
                let mut first = true;
                for item in items.iter().filter(|item| item.get("payload_scope").and_then(Value::as_str) != Some("analysis_run_record")) {
                    if !first {
                        w.raw(",");
                    }
                    first = false;
                    w.value(item);
                }
                w.raw("]");
            }
            _ => w.value(member),
        }
    }
    w.raw("}");
}

/// The recorded form's binding (`RecordedStateBinding`): the stored state
/// and the stored manifest, parsed.
pub(crate) struct RecordedState<'a> {
    pub(crate) state_id: &'a str,
    pub(crate) model_hash: &'a str,
    pub(crate) manifest: &'a Value,
}

/// `rulePackRefDiagnosticEntries(items)`: each appended item exactly the
/// rule-pack reference diagnostic its code and identity determine, no
/// identity twice; `None` otherwise.
fn rule_pack_entries(items: &[Value]) -> Option<Vec<String>> {
    let mut identities: Vec<String> = Vec::new();
    for item in items {
        let annotation = item.as_object().and_then(|map| map.get("source_annotation"));
        if !truthy(Some(item)) || !objectish(Some(item)) || item.as_object().map_or(item.as_array().map_or(0, Vec::len), |map| map.len()) != 1 || !record(annotation) {
            return None;
        }
        let annotation = annotation.expect("checked");
        let code = annotation.get("code").and_then(Value::as_str)?;
        if ![RULE_PACK_REF_UNRESOLVED, RULE_PACK_REF_CHECKSUM_MISMATCH].contains(&code) {
            return None;
        }
        let refs = annotation.get("affected_refs").and_then(Value::as_array)?;
        let identity = match refs.as_slice() {
            [Value::String(identity)] if !identity.is_empty() => identity,
            _ => return None,
        };
        if !rule_pack_identity_parses(identity) || identities.contains(identity) || !same_js_data(annotation, &rule_pack_ref_diagnostic(code, identity)) {
            return None;
        }
        identities.push(identity.clone());
    }
    Some(identities)
}

/// `parseRulePackIdentity(identity) !== null`.
fn rule_pack_identity_parses(identity: &str) -> bool {
    let parts: Vec<&str> = identity.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    let decode = |component: &str| -> Option<String> {
        let bytes = component.as_bytes();
        for (index, byte) in bytes.iter().enumerate() {
            if *byte == b'%' && !(bytes[index + 1..].starts_with(b"25") || bytes[index + 1..].starts_with(b"40")) {
                return None;
            }
        }
        Some(component.replace("%40", "@").replace("%25", "%"))
    };
    match (decode(parts[0]), decode(parts[1])) {
        (Some(id), Some(version)) => rule_pack_identity(&id, &version) == identity,
        _ => false,
    }
}

/// `rulePackDiagnosticsRefusal(identities, manifest)`.
fn rule_pack_diagnostics_refusal(identities: &[String], manifest: &Value) -> Result<Option<String>, String> {
    let excluded: Vec<String> = manifest_excluded_rule_packs(manifest)?.iter().map(|(id, version)| rule_pack_identity(id, version)).collect();
    if identities == excluded.as_slice() {
        return Ok(None);
    }
    let none_resolved = manifest.get("active_rule_packs").and_then(Value::as_array).is_some_and(Vec::is_empty);
    Ok(Some(if identities.is_empty() && !excluded.is_empty() && none_resolved {
        format!("{RULE_PACKS_NOT_RESOLVED}: recorded before rule-pack resolution; re-record this run")
    } else {
        "ANALYSIS_RULE_PACK_REF_DIAGNOSTICS_MISMATCH".into()
    }))
}

/// `validateRecordedBinding`, after the store's own read has checked the
/// stored manifest's members against the record (`analysis_runs`
/// `manifest_members`); its member list is that function's, reordered and
/// named with the TypeScript's codes.
fn validate_recorded_binding(run: Option<&Value>, source: &Value, binding: &RecordedState<'_>) -> Result<(), String> {
    if !same_js_data(at(run, "model_state_ref")?.unwrap_or(&Value::Null), &json!({"object_type": "ModelState", "ref": binding.state_id}))
        || at(run, "model_state_ref")?.is_none()
    {
        return Err("ANALYSIS_SOURCE_MODEL_STATE_MISMATCH".into());
    }
    let created = at(run, "created_at")?;
    let missing_time = match created {
        Some(Value::String(text)) if !text.is_empty() => match at(at(run, "reproducibility")?, "determinism_notes")? {
            None | Some(Value::Null) => false,
            Some(Value::Array(items)) => items.iter().any(|item| item.as_str() == Some(CREATED_AT_UNAVAILABLE)),
            Some(Value::String(text)) => text.contains(CREATED_AT_UNAVAILABLE),
            Some(_) => return Err(TYPE_ERROR.into()),
        },
        _ => true,
    };
    if missing_time {
        return Err("ANALYSIS_RECORDED_REVISION_TIME_MISSING".into());
    }
    let model_basis = binding.manifest.get("model_basis");
    if !strict_eq(get_opt(model_basis, "model_ref"), source.get("model_ref")) {
        return Err("ANALYSIS_RECORDED_INPUT_MANIFEST_MISMATCH".into());
    }
    let payload = at(model_basis, "model_payload")?.unwrap_or(&Value::Null);
    if format!("sha256:{}", jcs_sha256(payload)) != binding.model_hash {
        return Err("ANALYSIS_RECORDED_MODEL_STATE_HASH_MISMATCH".into());
    }
    let sha = jcs_sha256(binding.manifest);
    let members = manifest_members(binding.manifest, &sha, true)?;
    let code = |name: &str| match name {
        "input manifest ref" | "input manifest checksum" => "ANALYSIS_RECORDED_INPUT_MANIFEST_MISMATCH",
        "solver version" => "ANALYSIS_RECORDED_SOLVER_MISMATCH",
        "settings ref" => "ANALYSIS_RECORDED_SETTINGS_MISMATCH",
        "unit-system ref" => "ANALYSIS_RECORDED_UNITS_MISMATCH",
        "load basis" => "ANALYSIS_RECORDED_LOAD_BASIS_MISMATCH",
        _ => "ANALYSIS_RECORDED_REFERENCES_MISMATCH",
    };
    // The TypeScript's order: manifest refs and hashes, solver, settings, units, load basis, rule packs, libraries.
    let order = ["input manifest ref", "input manifest checksum", "solver version", "settings ref", "unit-system ref", "load basis", "rule-pack refs", "library refs"];
    for name in order {
        let Some((_, pointer, expected)) = members.iter().find(|(member, _, _)| *member == name) else { continue };
        let actual = match pointer.strip_prefix("/reproducibility/") {
            Some(key) => get_opt(run.and_then(|run| run.get("reproducibility")), key),
            None => run.and_then(|run| run.get(&pointer[1..])),
        };
        if !actual.is_some_and(|actual| same_js_data(actual, expected)) {
            return Err(code(name).into());
        }
    }
    Ok(())
}

/// `validateAnalysisRunV03(record, source, expectedBasisRefs, recordedState,
/// { recordedRead })` in its order. `source` is the parsed stored result; the
/// route's reader may rewrite its numbers in place (module docs). `Err` is the
/// message the TypeScript throws ([`TYPE_ERROR`] for its `TypeError`s).
/// `recorded_read` (the reopen paths only): a record binding a
/// registry variant is read under it, through the recorded dispatch (its
/// refusal thrown as the reader's), `rv::validate_recorded`,
/// `rv::retained_recorded` and the variant's table; without it, or for any
/// other record, only the current tables are read.
pub(crate) fn validate_v03(record: &Value, source: &mut Value, expected_basis: Option<&Value>, recorded: Option<&RecordedState<'_>>, recorded_read: bool) -> Result<(), String> {
    let contract_mismatch = "ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH";
    // `recordedRead(record, source)`: the recorded dispatch's route and the variant's table.
    let variant = if recorded_read { recorded_record_binding(record) } else { None };
    let route = match variant {
        Some(variant) => {
            let dispatched = recorded_dispatch(source, variant, None);
            if let Some((_, message)) = reader_refusal(&dispatched) {
                return Err(message);
            }
            Route { name: route_name(&dispatched["route"]), binding_id: variant.id.into(), binding_sha256: variant.sha256.into(), table: Some(variant.table()) }
        }
        None => match dispatch(source) {
            Ok(route) => route,
            Err(_) => return Err(contract_mismatch.into()),
        },
    };
    if !strict_eq_str(record.get("schema_version"), ANALYSIS_RUN_V03_TEXT) || !strict_eq_str(at(at(Some(record), "run_contract_status")?, "record_contract")?, "strict_analysis_run_v0_3") {
        return Err(contract_mismatch.into());
    }
    match (route.name, variant) {
        ("preview_physics", Some(variant)) => {
            if let Some((_, message)) = reader_refusal(&rv::validate_recorded("preview-physics", source, &binding_value(variant), None, None)) {
                return Err(message);
            }
        }
        ("preview_physics" | "pressure", _) => validate_route(source, &route)?,
        _ => {}
    }
    // validateSourceRuleStatus
    let rule = at(at(Some(&*source), "status")?, "rule_check")?;
    if !nullish(rule) && !rule.and_then(Value::as_str).is_some_and(|rule| RULE_STATUSES.contains(&rule)) {
        return Err("ANALYSIS_SOURCE_RULE_STATUS_INVALID".into());
    }
    let run = at(Some(record), "analysis_run")?;
    let mut expected_basis = expected_basis.cloned();
    if let Some(binding) = recorded {
        validate_recorded_binding(run, source, binding)?;
        let manifest_basis = manifest_members(binding.manifest, &jcs_sha256(binding.manifest), true)?
            .into_iter()
            .find(|(name, _, _)| *name == "load basis")
            .map(|(_, _, value)| value)
            .unwrap_or_else(|| json!([]));
        if expected_basis.as_ref().is_some_and(|expected| !same_js_data(expected, &manifest_basis)) {
            return Err("ANALYSIS_RECORDED_LOAD_BASIS_MISMATCH".into());
        }
        expected_basis = Some(manifest_basis);
    }
    let has_own = |key: &str| run.and_then(Value::as_object).is_some_and(|map| map.contains_key(key));
    if route.name == "pressure_source" {
        validate_route(source, &route)?;
        if !same(at(run, "source_block_recovery")?, source.get("source_block_recovery"))? {
            return Err("ANALYSIS_SOURCE_BLOCK_RECEIPT_MISMATCH".into());
        }
    } else if has_own("source_block_recovery") {
        return Err("SOURCE_RECOVERY_METADATA_CONTRADICTION".into());
    }
    if route.name == "pressure_source" {
        if !same(at(run, "contract_evidence")?, source.get("contract_evidence"))? {
            return Err("ANALYSIS_PHYSICS_SOURCE_EVIDENCE_MISMATCH".into());
        }
    } else if has_own("contract_evidence") {
        return Err("ANALYSIS_PHYSICS_SOURCE_DOWNGRADE_FORBIDDEN".into());
    }
    if route.retained() {
        match variant {
            Some(variant) => {
                if let Some(refusal) = retained_recorded_refusal(source, variant) {
                    return Err(refusal);
                }
            }
            None => validate_route(source, &route)?,
        }
        if !same(at(run, "retained_precision")?, source.get("retained_precision"))? {
            return Err(ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH.into());
        }
    } else if has_own("retained_precision") {
        return Err("ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN".into());
    }
    let source = &*source;
    // The producer's diagnostics, then the rule-pack reference diagnostics.
    let Some(Value::Array(produced)) = source.get("diagnostics") else { return Err(TYPE_ERROR.into()) };
    let diagnostics_mismatch = || Err("ANALYSIS_SOURCE_DIAGNOSTICS_MISMATCH".to_string());
    let recorded_diagnostics = match at(run, "diagnostics")? {
        Some(Value::Array(items)) if items.len() >= produced.len() => items,
        _ => return diagnostics_mismatch(),
    };
    for item in recorded_diagnostics.iter().take(produced.len()) {
        checked(item)?;
    }
    for annotation in produced {
        checked(annotation)?;
    }
    let prefix_same = recorded_diagnostics.iter().zip(produced).all(|(item, annotation)| {
        item.as_object().is_some_and(|map| map.len() == 1 && map.get("source_annotation").is_some_and(|value| same_js_data(value, annotation)))
    });
    if !prefix_same {
        return diagnostics_mismatch();
    }
    let Some(identities) = rule_pack_entries(&recorded_diagnostics[produced.len()..]) else { return diagnostics_mismatch() };
    let recorded_refusal = match recorded {
        Some(binding) => rule_pack_diagnostics_refusal(&identities, binding.manifest)?,
        None => None,
    };
    let not_resolved = format!("{RULE_PACKS_NOT_RESOLVED}: recorded before rule-pack resolution; re-record this run");
    let deferred = recorded_refusal.as_deref() == Some(not_resolved.as_str());
    if let Some(refusal) = recorded_refusal.as_ref().filter(|_| !deferred) {
        return Err(refusal.clone());
    }
    let mechanics = at(Some(source), "status")?.and_then(|status| status.get("mechanics"));
    if !mechanics.and_then(Value::as_str).is_some_and(|text| ["MECHANICS_SOLVED", "MODEL_INCOMPLETE"].contains(&text)) {
        return Err("ANALYSIS_SOURCE_MECHANICS_STATUS_INVALID".into());
    }
    let statuses_ok = match at(run, "analysis_status")? {
        Some(Value::Array(items)) if items.len() == 3 => {
            // `new Set(items).size === 3`: objects are distinct by identity.
            let distinct = (0..3).all(|i| (0..i).all(|j| !(!items[i].is_object() && !items[i].is_array() && strict_eq(Some(&items[i]), Some(&items[j])))));
            distinct
                && items.iter().any(|item| strict_eq(Some(item), mechanics))
                && items.iter().any(|item| item.as_str() == Some("HUMAN_REVIEW_REQUIRED"))
                && items.iter().filter(|item| item.as_str().is_some_and(|text| RULE_STATUSES.contains(&text))).count() == 1
        }
        _ => false,
    };
    if !statuses_ok {
        return Err("ANALYSIS_SOURCE_STATUS_MISMATCH".into());
    }
    let recorded_basis = at(run, "load_basis_refs")?;
    let expected_load = Value::Array(expected_load_basis(source, expected_basis.as_ref())?);
    if !same(recorded_basis, Some(&expected_load))? {
        return Err("ANALYSIS_SOURCE_LOAD_BASIS_MISMATCH".into());
    }
    let solver = at(run, "solver_version")?;
    let producer = source.get("producer");
    if !strict_eq(get_opt(solver, "solver_name"), at(producer, "component_name")?) || !strict_eq(get_opt(solver, "solver_version"), at(producer, "component_version")?) {
        return Err("ANALYSIS_SOURCE_PRODUCER_MISMATCH".into());
    }
    if !strict_eq(at(run, "run_id")?, source.get("run_id")) {
        return Err("ANALYSIS_SOURCE_RUN_MISMATCH".into());
    }
    if recorded.is_none() {
        let expected_state = json!({"object_type": "ModelState", "ref": format!("state:{}:preview", js_string(source.get("model_ref")))});
        if !same(at(run, "model_state_ref")?, Some(&expected_state))? {
            return Err("ANALYSIS_SOURCE_MODEL_STATE_MISMATCH".into());
        }
    }
    let reproducibility = at(run, "reproducibility")?;
    let contract = match variant {
        Some(variant) => binding_value(variant),
        None => recorded_run_semantic_binding(get_opt(reproducibility, "semantic_contract"), &route),
    };
    if !same(at(reproducibility, "semantic_contract")?, Some(&contract))? {
        return Err("ANALYSIS_SEMANTIC_CONTRACT_MISMATCH".into());
    }
    let Some(Value::Array(hashes)) = at(run, "hashes")? else { return Err(TYPE_ERROR.into()) };
    let mut received: Vec<Value> = Vec::new();
    for hash in hashes {
        if strict_eq_str(at(Some(hash), "payload_scope")?, "received_result") {
            received.push(hash.clone());
        }
    }
    let run_id = js_string(source.get("run_id"));
    let received_expected = json!([{"algorithm": "sha256", "canonicalization": CHECKED_PROFILE_V1, "payload_ref": {"object_type": "ResultEnvelope", "ref": format!("result-envelope:{run_id}")}, "payload_scope": "received_result", "value": checked_sha256(source)?}]);
    if !same(Some(&Value::Array(received)), Some(&received_expected))? {
        return Err("ANALYSIS_RECEIVED_SOURCE_MISMATCH".into());
    }
    let rows = rows_of(source).map_err(|_| TYPE_ERROR.to_string())?;
    let refs = at(run, "result_refs")?;
    let refs_length = match refs {
        Some(Value::Array(items)) => Some(items.len()),
        Some(Value::String(text)) => Some(text.encode_utf16().count()),
        _ => None,
    };
    if refs_length != Some(rows.len()) || duplicate_ids(rows, &mut |_| Ok(None)).map_err(|_| TYPE_ERROR.to_string())? {
        return Err("ANALYSIS_ROW_ACCOUNTING_MISMATCH".into());
    }
    for (index, row) in rows.iter().enumerate() {
        let actual = match refs {
            Some(Value::Array(items)) => Some(&items[index]),
            // A string's characters are primitives with none of the members read.
            _ => Some(&Value::Bool(false)),
        };
        let table = route.table.ok_or_else(|| SOURCE_CONTRACT_UNSUPPORTED.to_string())?;
        let (semantic, findings) = interpret(table, row)?;
        let id = row.get("id").cloned().unwrap_or(Value::Null);
        let reference = json!({"object_type": "Result", "ref": id});
        let member = |key: &str| semantic.and_then(|entry| entry.get(key)).filter(|value| !value.is_null());
        let row_contract = {
            let mut contract = contract.clone();
            contract["signature_id"] = member("signature_id").cloned().unwrap_or(Value::Null);
            contract
        };
        let index_matches = at(actual, "source_row_index")?.and_then(Value::as_f64) == Some(index as f64);
        let binding_ok = index_matches
            && same(actual.and_then(|actual| actual.get("result_ref")), Some(&reference))?
            && same(actual.and_then(|actual| actual.get("semantic_contract")), Some(&row_contract))?
            && same(
                actual.and_then(|actual| actual.get("hash_refs")),
                Some(&json!([{"algorithm": "sha256", "canonicalization": CHECKED_PROFILE_V1, "payload_ref": reference, "payload_scope": "result_row", "value": checked_sha256(row)?}])),
            )?;
        if !binding_ok {
            return Err("ANALYSIS_ROW_SOURCE_BINDING_MISMATCH".into());
        }
        let get = |key: &str| actual.and_then(|actual| actual.get(key));
        let category = member("category").cloned().unwrap_or_else(|| json!("unknown"));
        let dimension = member("source_physical_semantic_dimension").cloned().unwrap_or(Value::Null);
        let family = member("family").cloned().unwrap_or(Value::Null);
        let interpretation = json!({"status": member("canonical_disposition").cloned().unwrap_or_else(|| json!("unavailable")), "findings": findings});
        let annotation = json!({"kind": row.get("kind"), "unit": row.get("unit"), "metadata": row.get("metadata").cloned().unwrap_or(Value::Null)});
        if !strict_eq(get("category"), Some(&category))
            || !strict_eq(get("source_dimension"), Some(&dimension))
            || !strict_eq(get("result_family"), Some(&family))
            || !same(get("interpretation"), Some(&interpretation))?
            || !same(get("source_annotation"), Some(&annotation))?
        {
            return Err("ANALYSIS_ROW_INTERPRETATION_MISMATCH".into());
        }
    }
    if verify_record(record)? != Verification::Match {
        return Err("ANALYSIS_RECORD_CHECKSUM_MISMATCH".into());
    }
    if deferred {
        return Err(not_resolved);
    }
    Ok(())
}

/// Model document 0.8.0 (DEL-02-01 Design `model-document-0.8.0.md` §3.2; owner ruling
/// "0.8.0 a2": recorded runs keep their existing read and compare behaviour, with no
/// model-document-version gate). pressure-2's table changed only the model version token of
/// its admitted-families sentence (0.7.0 -> 0.8.0); every row, and so every row reading, is the
/// same. A run recorded under the 0.7.0-era table binds that table's exact digest, which is
/// read as the recorded basis of exactly this current table. Only the record validator reads
/// it: the record builder and the export bind the current table, and any other digest, or a
/// record binding the two tables at once, is refused as before. Model document 0.9.0 (Design
/// `model-document-0.9.0.md` §3.2) edits the same sentence's token again (0.8.0 -> 0.9.0), so a
/// run recorded under the 0.8.0-era table is read the same way. RT-A3 reducer 2a (#73) and 2b
/// (#80) each restated only the table's admitted-families disclosure
/// (`supported_profile_limitations[1]`; every row is the same), so runs recorded under the 0.9.0
/// registration's table (d46e1963…) and under 2a's (032e861f…) are read the same way too. RT-A3
/// increment 3 restates two limitations and appends a combination gate, rows unchanged, so a run
/// recorded under the pre-increment table (a2db16a5…) is read the same way; RT-A4 (DEL-05-02
/// `absolute-sum-srss-combinations.md` §10.2) restates the combination policy and adds three
/// vocabulary tokens, rows unchanged, so a run recorded under the constant-effort table
/// (be29fabc…) is read the same way. RT-A3 increment 4 (DEL-04-07 Design `pressure-contract.md`,
/// "Nonlinear supports on EX") restates two limitations, rows and gates unchanged, so a run
/// recorded under the RT-A4 table (10455bf3…) is read the same way. RT-A3 §4 (DEL-05-02 Work
/// `combination-certificate.md`) restates the combination standing and the combinations
/// limitation, rows and gates unchanged, so a run recorded under main's table at 252bef969
/// (2c20583d…) is read the same way against the current table (21b7426e…).
/// The native mirror of `analysisRunCompatibility.ts` `RECORDED_RUN_TABLE_BASES` (id,
/// recorded, current), row for row and in order; Python's twin is `compatibility.py`
/// `RECORDED_RUN_TABLE_BASES`. `stored_custody_tests.rs` requires all three to be equal.
pub(crate) const RECORDED_RUN_TABLE_BASES: [(&str, &str, &str); 8] = [
    ("openpipestress.result_semantics/0.3.0/pressure-2", "3e2aac3cbe1dd967292dc2524cc7e12a35b162f14795c13b741acaa2b964625d", "c5fe09a2dc82068f46ce8dd0b5873071af1ec13e62c9bacf0a009570d7e5d2ad"),
    ("openpipestress.result_semantics/0.3.0/pressure-2", "5451fbf3e10e085c90b523baa7daf5fc723659e0d874d988d23807be7d2fefe6", "c5fe09a2dc82068f46ce8dd0b5873071af1ec13e62c9bacf0a009570d7e5d2ad"),
    ("openpipestress.result_semantics/0.3.0/pressure-2", "d46e1963d7502b8e41898805128f2807b2670a1d4c738c71876002ed727e5faf", "c5fe09a2dc82068f46ce8dd0b5873071af1ec13e62c9bacf0a009570d7e5d2ad"),
    ("openpipestress.result_semantics/0.3.0/pressure-2", "032e861fc66bf5f34a081b104ec879a39831bfd6472b186954966ea57d65f025", "c5fe09a2dc82068f46ce8dd0b5873071af1ec13e62c9bacf0a009570d7e5d2ad"),
    ("openpipestress.result_semantics/0.3.0/pressure-2", "a2db16a515d8e4e1c1bcd6607861df0e7e58cbf1cf82a0b984e2f1c0f128382e", "c5fe09a2dc82068f46ce8dd0b5873071af1ec13e62c9bacf0a009570d7e5d2ad"),
    ("openpipestress.result_semantics/0.3.0/pressure-2", "be29fabcfacb1070a51bfb56dcfe61b4e4125638e249c3c0e96d092cc8aba43f", "c5fe09a2dc82068f46ce8dd0b5873071af1ec13e62c9bacf0a009570d7e5d2ad"),
    ("openpipestress.result_semantics/0.3.0/pressure-2", "10455bf348b46efac998b6635b537c9e3149a7d1c7a29ea96c0e3783a2de9ad6", "c5fe09a2dc82068f46ce8dd0b5873071af1ec13e62c9bacf0a009570d7e5d2ad"),
    ("openpipestress.result_semantics/0.3.0/pressure-2", "2c20583d3c06e226a13bb7f9649b04aeea0f0f6948b5fa1cbd944afd07413645", "c5fe09a2dc82068f46ce8dd0b5873071af1ec13e62c9bacf0a009570d7e5d2ad"),
];

/// `recordedRunSemanticBinding(recorded, sourceSemanticBinding(source))`: the
/// current table's binding, or the exact recorded basis the record names when
/// it is one of [`RECORDED_RUN_TABLE_BASES`] for that current table.
fn recorded_run_semantic_binding(recorded: Option<&Value>, route: &Route) -> Value {
    let named = recorded.and_then(Value::as_object).filter(|map| map.len() == 2);
    let basis = named.and_then(|named| {
        RECORDED_RUN_TABLE_BASES.iter().find(|(id, recorded, current)| {
            *id == route.binding_id && *current == route.binding_sha256 && named.get("id").and_then(Value::as_str) == Some(id) && named.get("sha256").and_then(Value::as_str) == Some(recorded)
        })
    });
    match basis {
        Some((id, recorded, _)) => json!({"id": id, "sha256": recorded}),
        None => json!({"id": route.binding_id, "sha256": route.binding_sha256}),
    }
}

/// The validator's outcome as the reopen shows it: a named refusal as it
/// states it, an unnamed throw as one finding.
fn validator_finding(error: &str) -> String {
    if named_refusal(error) { error.to_string() } else { "HISTORICAL_ANALYSIS_EVIDENCE_INVALID".to_string() }
}

// ---------------------------------------------------------------------------
// The two contexts' findings
// ---------------------------------------------------------------------------

/// A saved project's stored members other than its run (the open's reply
/// carries them as stored).
pub(crate) struct SavedProject<'a> {
    pub(crate) model: &'a Value,
    pub(crate) editor_intents: &'a Value,
    pub(crate) proposal: &'a Value,
    pub(crate) selected_review_target: &'a Value,
    pub(crate) model_hash: &'a Value,
    pub(crate) project_envelope_hash: &'a Value,
}

/// What a reopened run establishes for the webview: the findings, the run's
/// name, the recomputed envelope payload hash (saved projects) or the
/// recomputed model hash (recorded runs), and the standing for S's tables.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct HistoricalFindings {
    pub(crate) findings: Vec<String>,
    /// (`checkFindings`): the findings of the reopen's checks
    /// alone, every entry of `findings` but the current standing's.
    pub(crate) check_findings: Vec<String>,
    /// (`recordedTableRead`): the registry variant `{id, sha256}`
    /// a run recorded before RT-A4 was read under, or null.
    pub(crate) recorded_table_read: Option<Value>,
    /// (`historicalReaderFindings`): the findings the historical
    /// reader computed under `recorded_table_read`; empty without one.
    pub(crate) historical_reader_findings: Vec<String>,
    pub(crate) run_id: String,
    /// Whether the stored result renders (`isRenderableMechanicsResult`): only
    /// then is it inspected through S and pages.
    pub(crate) renderable: bool,
    /// The stored result's `status.mechanics` when it is evidence.
    pub(crate) mechanics: Option<String>,
    /// The standing S's tables are read with (`null` when not renderable).
    /// The dispatch standing behind the standing findings (`numericalResultStanding`'s): kept for the
    /// findings only, never sent: a reopened run has no current standing.
    #[serde(skip)]
    pub(crate) standing: Value,
    pub(crate) envelope_payload_hash: Option<String>,
    pub(crate) model_hash: Option<Value>,
    /// The registry variant the reopen read under (the held handle's
    /// recorded table); never sent but as `recorded_table_read`.
    #[serde(skip)]
    pub(crate) recorded: Option<RecordedTable>,
}

impl HistoricalFindings {
    /// `found.fail(finding, true)` on a context already built.
    fn fail_once(&mut self, finding: String) {
        if self.check_findings.contains(&finding) {
            return;
        }
        self.check_findings.push(finding.clone());
        if !self.findings.contains(&finding) {
            self.findings.push(finding);
        }
    }

    fn scoped(found: Found, recorded: Option<&'static RecordedVariant>) -> (Vec<String>, Vec<String>, Option<Value>, Vec<String>) {
        let historical = if recorded.is_some() { found.historical } else { Vec::new() };
        (found.findings, found.checks, recorded.map(binding_value), historical)
    }
}

/// JavaScript truthiness of a parsed value (`undefined` is `None`).
fn js_truthy(value: Option<&Value>) -> bool {
    truthy(value)
}

/// `buildHistoricalRunContext(opened)`: `None` when the project stores neither
/// a result nor a record. `result` and `record` are the stored values parsed
/// (`Value::Null` for a stored `null`); `result` may be rewritten in place by
/// the retained reader (numbers only).
pub(crate) fn saved_run_findings(project: &SavedProject<'_>, result: &mut Value, record_envelope: &Value) -> Option<HistoricalFindings> {
    let has_mechanics = !result.is_null();
    let has_record = !record_envelope.is_null();
    if !has_mechanics && !has_record {
        return None;
    }
    let evidence = historical_evidence(result);
    let is_renderable = renderable(result);
    let analysis_run = (!record_envelope.is_null()).then_some(record_envelope);
    let mut found = Found::new(&["HISTORICAL_INPUT_MANIFEST_MISSING"]);
    // a run recorded before RT-A4 binds a registry variant; it is read (its standing
    // findings, its successor receipt and its record) as it read when recorded, and stays historical.
    let recorded = strict_eq_str(record_envelope.get("schema_version"), ANALYSIS_RUN_V03_TEXT).then(|| recorded_record_binding(record_envelope)).flatten();
    let standing = if is_renderable { found.read_standing(result, Some(project.model), recorded) } else { Value::Null };
    if has_mechanics && !is_renderable {
        found.fail("HISTORICAL_MECHANICS_EVIDENCE_MALFORMED");
    }
    let record = analysis_run.filter(|run| js_truthy(Some(run)) && objectish(Some(run))).and_then(|run| run.get("analysis_run")).filter(|run| js_truthy(Some(run)) && objectish(Some(run)));
    let record_shape_ok = |record: Option<&Value>| {
        record.is_some_and(|record| {
            is_array(record.get("hashes")) && is_array(record.get("result_refs")) && is_str(record.get("run_id")) && is_str(get_opt(record.get("model_state_ref"), "ref"))
        })
    };
    if js_truthy(analysis_run) && !record_shape_ok(record) {
        found.fail("HISTORICAL_ANALYSIS_EVIDENCE_INVALID");
    }
    if is_renderable && reopen_retained_route(result, recorded) {
        let receipt_same = record.map(|record| same_saved_receipt(record.get("retained_precision"), result.get("retained_precision")));
        // The reader reads a rewritten copy's numbers; the receipt comparison
        // above read the parsed values first, as the TypeScript compares the
        // objects it holds (the rewrite changes no value's data).
        if let Some(refusal) = reopen_retained_refusal(result, recorded) {
            found.fail(refusal);
        }
        if receipt_same == Some(false) {
            found.fail(ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH);
        }
    }
    let project_id = project.model.pointer("/project/id");
    let result = &*result;
    if evidence && !strict_eq(result.get("model_ref"), project_id) {
        found.fail("HISTORICAL_MODEL_REF_MISMATCH");
    }
    let expected_state_ref = format!("state:{}:preview", js_string(project_id));
    if record.is_some_and(|record| !strict_eq_str(get_opt(record.get("model_state_ref"), "ref"), &expected_state_ref)) {
        found.fail("HISTORICAL_MODEL_STATE_REF_MISMATCH");
    }
    if !evidence || record.is_none() {
        found.fail("HISTORICAL_RUN_EVIDENCE_INCOMPLETE");
    }
    if evidence && record.is_some_and(|record| !strict_eq(result.get("run_id"), record.get("run_id"))) {
        found.fail("HISTORICAL_RUN_REF_MISMATCH");
    }
    let model_hash = (!project.model_hash.is_null()).then_some(project.model_hash);
    if !js_truthy(model_hash) {
        found.fail("HISTORICAL_MODEL_HASH_MISSING");
    }
    if !js_truthy(Some(project.project_envelope_hash)) {
        found.fail("HISTORICAL_ENVELOPE_HASH_MISSING");
    }
    let saved_version = get_opt(analysis_run, "schema_version");
    let current_record = strict_eq_str(saved_version, ANALYSIS_RUN_V03_TEXT);
    if js_truthy(analysis_run) && saved_version.and_then(Value::as_str).is_some_and(|version| ["0.1.0", "0.2.0"].contains(&version)) {
        found.fail(ANALYSIS_RUN_SCHEMA_VERSION_RETIRED);
    } else if js_truthy(analysis_run) && record.is_some() && !current_record {
        found.fail("HISTORICAL_ANALYSIS_SCHEMA_VERSION_UNSUPPORTED");
    }
    let mut envelope_payload_hash = None;
    let hashes = (|| -> Result<(), String> {
        // `computeModelHash(model)` reads `model.project.id`.
        at(at(Some(project.model), "project")?, "id")?;
        let recomputed_model = format!("sha256:{}", jcs_sha256(project.model));
        if js_truthy(model_hash) && (!strict_eq_str(get_opt(model_hash, "value"), &recomputed_model) || !strict_eq(get_opt(model_hash, "payload_ref"), project_id)) {
            found.fail("HISTORICAL_MODEL_HASH_MISMATCH");
        }
        let envelope = envelope_payload_sha256(project, result, record_envelope);
        envelope_payload_hash = Some(format!("sha256:{envelope}"));
        let stored_envelope = Some(project.project_envelope_hash);
        if js_truthy(stored_envelope)
            && (!strict_eq_str(get_opt(stored_envelope, "value"), envelope_payload_hash.as_deref().unwrap_or_default()) || !strict_eq(get_opt(stored_envelope, "payload_ref"), project_id))
        {
            found.fail("HISTORICAL_ENVELOPE_HASH_MISMATCH");
        }
        if evidence && (current_record || !js_truthy(analysis_run)) {
            let stored = match record.and_then(|record| record.get("hashes")) {
                Some(Value::Array(hashes)) => hashes.iter().find(|hash| objectish(Some(hash)) && get_opt(Some(hash), "payload_scope").and_then(Value::as_str) == Some("received_result")),
                _ => None,
            };
            match stored {
                None => found.fail("HISTORICAL_RESULT_HASH_MISSING"),
                Some(stored) => {
                    let claimed = stored.get("value").and_then(Value::as_str).map(|value| value.strip_prefix("sha256:").unwrap_or(value).to_string());
                    let computed = checked_sha256(result)?;
                    let reference = format!("result-envelope:{}", js_string(result.get("run_id")));
                    if claimed.as_deref() != Some(computed.as_str()) || !strict_eq_str(get_opt(stored.get("payload_ref"), "ref"), &reference) {
                        found.fail("HISTORICAL_RESULT_HASH_MISMATCH");
                    }
                }
            }
        }
        if current_record {
            match verify_record(record_envelope)? {
                Verification::Mismatch => found.fail("HISTORICAL_ANALYSIS_HASH_MISMATCH"),
                Verification::Unverifiable => found.fail("HISTORICAL_ANALYSIS_HASH_UNVERIFIABLE"),
                Verification::Match => {}
            }
        }
        Ok(())
    })();
    if hashes.is_err() {
        found.fail("HISTORICAL_HASH_RECOMPUTE_UNAVAILABLE");
    }
    let run_id = match (evidence, record.and_then(|record| record.get("run_id")).and_then(Value::as_str)) {
        (true, _) => js_string(result.get("run_id")),
        (false, Some(run_id)) => run_id.to_string(),
        _ => "unknown saved run".to_string(),
    };
    let (findings, check_findings, recorded_table_read, historical_reader_findings) = HistoricalFindings::scoped(found, recorded);
    Some(HistoricalFindings {
        findings,
        check_findings,
        recorded_table_read,
        historical_reader_findings,
        run_id,
        renderable: is_renderable,
        mechanics: evidence.then(|| js_string(result.pointer("/status/mechanics"))),
        standing,
        envelope_payload_hash,
        model_hash: None,
        recorded: recorded.map(RecordedTable::of),
    })
}

/// The validator for a saved run, after its findings: `findings` gains its
/// finding when the record is current and the result renders.
pub(crate) fn saved_run_validation(found: &mut HistoricalFindings, result: &mut Value, record_envelope: &Value) {
    let record = record_envelope.get("analysis_run").filter(|run| js_truthy(Some(run)) && objectish(Some(run)));
    if !(strict_eq_str(record_envelope.get("schema_version"), ANALYSIS_RUN_V03_TEXT) && found.renderable && record.is_some()) {
        return;
    }
    let basis = record.and_then(|record| record.get("load_basis_refs"));
    if let Err(error) = validate_v03(record_envelope, result, basis, None, true) {
        found.fail_once(validator_finding(&error));
    }
}

/// `computeProjectEnvelopeHash`'s payload hash over the stored values
/// (`editor_intents ?? []`), streamed in RFC 8785's member order.
fn envelope_payload_sha256(project: &SavedProject<'_>, result: &Value, record: &Value) -> String {
    let mut hasher = Sha256::new();
    let mut w = Writer::new(&mut hasher, Style::Canonical);
    w.raw("{\"analysis_run\":");
    js_canonical_value(&mut w, record);
    w.raw(",\"editor_intents\":");
    if project.editor_intents.is_null() {
        w.raw("[]");
    } else {
        js_canonical_value(&mut w, project.editor_intents);
    }
    w.raw(",\"mechanics_result\":");
    js_canonical_value(&mut w, result);
    w.raw(",\"model\":");
    js_canonical_value(&mut w, project.model);
    w.raw(",\"model_hash\":");
    js_canonical_value(&mut w, project.model_hash);
    w.raw(",\"proposal\":");
    js_canonical_value(&mut w, project.proposal);
    w.raw(",\"selected_review_target\":");
    js_canonical_value(&mut w, project.selected_review_target);
    w.raw("}");
    drop(w);
    hex(&hasher.finalize())
}

/// What `buildRecordedRunContext` reads of the store's verified read besides
/// the result, record and manifest.
pub(crate) struct RecordedRead<'a> {
    pub(crate) state_id: &'a str,
    pub(crate) state_project_id: &'a str,
    pub(crate) state_model_hash: &'a str,
    pub(crate) run_state_id: &'a str,
    pub(crate) producer_run_id: &'a str,
    pub(crate) run_record_hash: &'a str,
}

/// `buildRecordedRunContext(read)`: `result` is the stored result parsed, or
/// `None` when its text is not JSON (the store's read admitted it); likewise
/// the manifest.
pub(crate) fn recorded_run_findings(read: &RecordedRead<'_>, result: Option<&mut Value>, manifest: Option<&Value>, record_envelope: &Value) -> HistoricalFindings {
    let mut found = Found::new(&[]);
    let mut parsed_null = Value::Null;
    let result_parsed = result.is_some();
    if !result_parsed {
        found.fail("HISTORICAL_MECHANICS_EVIDENCE_MALFORMED");
    }
    if manifest.is_none() {
        found.fail("RECORDED_RUN_INPUT_MANIFEST_MALFORMED");
    }
    let result: &mut Value = match result {
        Some(result) => result,
        None => &mut parsed_null,
    };
    let analysis_run = (!record_envelope.is_null()).then_some(record_envelope);
    let evidence = historical_evidence(result);
    let is_renderable = renderable(result);
    if !result.is_null() && !is_renderable {
        found.fail_once("HISTORICAL_MECHANICS_EVIDENCE_MALFORMED");
    }
    let model = get_opt(get_opt(manifest, "model_basis"), "model_payload").filter(|model| !model.is_null());
    // a run recorded before RT-A4 reads as it did when recorded (see the saved-run reader).
    let recorded = strict_eq_str(record_envelope.get("schema_version"), ANALYSIS_RUN_V03_TEXT).then(|| recorded_record_binding(record_envelope)).flatten();
    let standing = if is_renderable && js_truthy(model) { found.read_standing(result, model, recorded) } else { Value::Null };
    let record = analysis_run.filter(|run| js_truthy(Some(run)) && objectish(Some(run))).and_then(|run| run.get("analysis_run")).filter(|run| js_truthy(Some(run)) && objectish(Some(run)));
    let shape_ok = record.is_some_and(|record| {
        is_array(record.get("hashes")) && is_array(record.get("result_refs")) && is_str(record.get("run_id")) && is_str(get_opt(record.get("model_state_ref"), "ref"))
    });
    if !shape_ok {
        found.fail("HISTORICAL_ANALYSIS_EVIDENCE_INVALID");
    }
    if is_renderable && reopen_retained_route(result, recorded) {
        let receipt_same = record.map(|record| same_saved_receipt(record.get("retained_precision"), result.get("retained_precision")));
        if let Some(refusal) = reopen_retained_refusal(result, recorded) {
            found.fail(refusal);
        }
        if receipt_same == Some(false) {
            found.fail(ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH);
        }
    }
    let result = &*result;
    let state_ref = record.and_then(|record| record.get("model_state_ref"));
    if !js_truthy(state_ref)
        || !strict_eq_str(get_opt(state_ref, "object_type"), "ModelState")
        || !strict_eq_str(get_opt(state_ref, "ref"), read.state_id)
        || read.run_state_id != read.state_id
    {
        found.fail("HISTORICAL_MODEL_STATE_REF_MISMATCH");
    }
    if evidence && !strict_eq_str(result.get("model_ref"), read.state_project_id) {
        found.fail("HISTORICAL_MODEL_REF_MISMATCH");
    }
    if !evidence || record.is_none() {
        found.fail("HISTORICAL_RUN_EVIDENCE_INCOMPLETE");
    }
    if evidence && record.is_some_and(|record| !strict_eq(result.get("run_id"), record.get("run_id")) || !strict_eq_str(record.get("run_id"), read.producer_run_id)) {
        found.fail("HISTORICAL_RUN_REF_MISMATCH");
    }
    let saved_version = get_opt(analysis_run, "schema_version");
    let current_record = strict_eq_str(saved_version, ANALYSIS_RUN_V03_TEXT);
    if js_truthy(analysis_run) && saved_version.and_then(Value::as_str).is_some_and(|version| ["0.1.0", "0.2.0"].contains(&version)) {
        found.fail(ANALYSIS_RUN_SCHEMA_VERSION_RETIRED);
    } else if js_truthy(analysis_run) && record.is_some() && !current_record {
        found.fail("HISTORICAL_ANALYSIS_SCHEMA_VERSION_UNSUPPORTED");
    }
    let mut model_hash: Option<Value> = None;
    let hashes = (|| -> Result<(), String> {
        if let Some(model) = model.filter(|model| js_truthy(Some(model))) {
            let value = format!("sha256:{}", jcs_sha256(model));
            let project_ref = at(at(Some(model), "project")?, "id")?.cloned().unwrap_or(Value::Null);
            model_hash = Some(json!({"algorithm": "sha256", "canonicalization": "rfc8785_jcs", "payload_scope": "model_payload", "payload_ref": project_ref, "value": value, "hash_status": "computed_local_preview"}));
            if value != read.state_model_hash {
                found.fail("RECORDED_RUN_MODEL_STATE_HASH_MISMATCH");
            }
        }
        if let (Some(manifest), Some(record)) = (manifest, record) {
            let claimed = get_opt(get_opt(record.get("reproducibility"), "input_manifest_hashes").and_then(|hashes| hashes.get(0)), "value");
            if !strict_eq_str(claimed, &jcs_sha256(manifest)) {
                found.fail("RECORDED_RUN_INPUT_MANIFEST_HASH_MISMATCH");
            }
        }
        if evidence && current_record {
            let stored = match record.and_then(|record| record.get("hashes")) {
                Some(Value::Array(hashes)) => hashes.iter().find(|hash| objectish(Some(hash)) && get_opt(Some(hash), "payload_scope").and_then(Value::as_str) == Some("received_result")),
                _ => None,
            };
            match stored {
                None => found.fail("HISTORICAL_RESULT_HASH_MISSING"),
                Some(stored) => {
                    let computed = checked_sha256(result)?;
                    let reference = format!("result-envelope:{}", js_string(result.get("run_id")));
                    if !strict_eq_str(stored.get("value"), &computed) || !strict_eq_str(get_opt(stored.get("payload_ref"), "ref"), &reference) {
                        found.fail("HISTORICAL_RESULT_HASH_MISMATCH");
                    }
                }
            }
        }
        if current_record {
            let verification = verify_record(record_envelope)?;
            match verification {
                Verification::Mismatch => found.fail("HISTORICAL_ANALYSIS_HASH_MISMATCH"),
                Verification::Unverifiable => found.fail("HISTORICAL_ANALYSIS_HASH_UNVERIFIABLE"),
                Verification::Match => {}
            }
            if verification == Verification::Match {
                if let Some(record) = record {
                    let Some(Value::Array(hashes)) = record.get("hashes") else { return Err(TYPE_ERROR.into()) };
                    let mut matched = false;
                    for hash in hashes {
                        if strict_eq_str(at(Some(hash), "payload_scope")?, "analysis_run_record") && strict_eq_str(hash.get("value"), read.run_record_hash) {
                            matched = true;
                            break;
                        }
                    }
                    if !matched {
                        found.fail("RECORDED_RUN_RECORD_HASH_MISMATCH");
                    }
                }
            }
        }
        Ok(())
    })();
    if hashes.is_err() {
        found.fail("HISTORICAL_HASH_RECOMPUTE_UNAVAILABLE");
    }
    let run_id = match (evidence, record.and_then(|record| record.get("run_id")).and_then(Value::as_str)) {
        (true, _) => js_string(result.get("run_id")),
        (false, Some(run_id)) => run_id.to_string(),
        _ => "unknown recorded run".to_string(),
    };
    let (findings, check_findings, recorded_table_read, historical_reader_findings) = HistoricalFindings::scoped(found, recorded);
    HistoricalFindings {
        findings,
        check_findings,
        recorded_table_read,
        historical_reader_findings,
        run_id,
        renderable: is_renderable,
        mechanics: evidence.then(|| js_string(result.pointer("/status/mechanics"))),
        standing,
        envelope_payload_hash: None,
        model_hash,
        recorded: recorded.map(RecordedTable::of),
    }
}

/// The validator for a recorded run, after its findings.
pub(crate) fn recorded_run_validation(found: &mut HistoricalFindings, read: &RecordedRead<'_>, result: &mut Value, manifest: Option<&Value>, record_envelope: &Value) {
    let record = record_envelope.get("analysis_run").filter(|run| js_truthy(Some(run)) && objectish(Some(run)));
    let Some(manifest) = manifest else { return };
    if !(strict_eq_str(record_envelope.get("schema_version"), ANALYSIS_RUN_V03_TEXT) && found.renderable && record.is_some()) {
        return;
    }
    let binding = RecordedState { state_id: read.state_id, model_hash: read.state_model_hash, manifest };
    if let Err(error) = validate_v03(record_envelope, result, None, Some(&binding), true) {
        found.fail_once(validator_finding(&error));
    }
}
