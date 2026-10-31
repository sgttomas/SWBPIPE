//! C1 native custody, slice 3 (DEL-04-05 `c1-native-custody-plan.md`;
//! design `result-volume-design.md` §2(2), §3.2): the detail of a retained
//! solve result, read in this process from the held bytes in bounded pages,
//! so the webview can show and look up rows without holding the envelope
//! (the switch is slice 4; nothing reads these commands yet).
//!
//! - **The index.** On a held result's first detail query, its row byte
//!   ranges are indexed: one `(start, end)` pair per element of `results`,
//!   the exact token the producer published, in publication order. A page's
//!   rows are those byte ranges of the held text, so a row returned is the
//!   envelope's row, byte for byte (design §2(2)). The index also keeps the
//!   route's semantic table (`reader_verdict::route`, the table
//!   `reader_verdict::row_semantics` reads), the largest row's priced parse
//!   and the held text's scan allowances. It lives in the [`HeldResult`]
//!   beside the bytes, with its own lease, and is released with them by the
//!   last holder (a disposal, the take, or a session reset). Concurrent first
//!   queries build it once: the others wait for it and reuse it; a refused or
//!   panicked build leaves none, and the next query builds again.
//! - **Its passes** (an ordinary refinement of the plan, which proposed
//!   sharing S's pass: S parses into a `Value`, which keeps no byte offsets,
//!   and S is built per model and standing, while the index is not). Once per
//!   held result: the prescan of the held text (allocation-free); one parse
//!   into a `Value`, under the parse lease of its materialization M, only to
//!   read the route's table (`route` reads the whole source), dropped before
//!   anything else allocates; then, without building anything, a pass that
//!   finds the last `results` member (as `Value` keeps the last of a
//!   duplicated key), a pass that counts its elements, and a pass that
//!   records their ranges into an index allocated at its exact capacity,
//!   prescanning each row (allocation-free) for its priced parse. The
//!   three borrowing passes use serde_json's `RawValue` over the borrowed
//!   text; they hold no second tree and no copy of the envelope.
//! - **The commands** (raw args bodies under `command_ingress`, as slices 1
//!   and 2; on the async runtime only the header check, the copy's
//!   reservation and the copy, everything else in a blocking task):
//!   - `query_preview_mechanics_job_rows {jobId, filter, offset, limit}`: the
//!     rows `filter` admits, in publication order, `limit` (1 to
//!     [`MAX_PAGE_ROWS`]) of them from the `offset`-th, with their total. The
//!     filter is the results panel's (`rowsUnderFilters` and its selected
//!     entity: text, family, frame, case, kind, entity), the embedded agent's
//!     current page (`readResults`: kinds and entities, each any of a list)
//!     and the plan's (set, station, and row ids, each exact), all as the
//!     TypeScript reads them (below). A page stops early, with
//!     `truncated: true`, rather than exceed [`MAX_RESPONSE_BYTES`]; it then
//!     holds at least one row, so paging from `offset + returned` reaches
//!     every row. A row whose response alone exceeds it is refused by name
//!     ([`RESPONSE_LIMIT`]).
//!   - `lookup_preview_mechanics_job_rows {jobId, ordinals, ids, kinds,
//!     match}`: rows by their exact ordinal, and by id or kind with
//!     `match: "first"` (`results.find`) or `"last"` (the row a
//!     `new Map(results.map(r => [r.id, r]))` keeps). Every row with an id is
//!     the query with `filter.ids`. Duplicate ids keep these meanings.
//!   - `read_preview_mechanics_job_row_semantics {jobId, ordinals}`: one
//!     batched `reader_verdict::row_semantics_each` over the rows at those
//!     ordinals with the held source, the same table `readerRowSemantics`
//!     reads through the wasm engine. It parses the held text each call,
//!     under the parse lease of M, as the wasm engine parses the source text
//!     each call: the public reader entries take the source, not its table.
//!   - `query_preview_mechanics_job_source_recovery {jobId, offset, limit,
//!     raw}`: the facts `ResultsPanel` and `PhysicsSourceEvidencePanel` read
//!     from `source_block_recovery` (its presence, the producer's recorded
//!     status, the recorded and physical case counts and the panel's coverage
//!     agreement), with a page of the recorded cases' facts and, with `raw`,
//!     each case's exact recorded evidence text. These are recorded labels
//!     only: they grant no eligibility, which the readers decide. S and the
//!     full envelope never carry this member.
//! - **The response** of the row, lookup and recovery commands is binary: the
//!   magic `SWBDETv1`, the header's length as a little-endian u64, the JSON
//!   header, then the texts it lists, concatenated (each by its length in
//!   `lengths`, in order), with nothing after the last. The semantics
//!   response is the outcome array's JSON text.
//! - **Filters, as the TypeScript reads them.** Each member is optional;
//!   those present must all admit a row (`family`, `frame` and `case` equal
//!   to `"all"`, and empty lists, admit every row, as the panel's and the
//!   agent's do). In the panel's order: `family` (the row's family as
//!   `semanticLabels` reads it); `text` (`filterResults`: trimmed and lower
//!   cased, a substring of the row's searchable members, joined by spaces and
//!   lower cased, with JavaScript's truthiness, `String` and number text);
//!   `kinds`; `frame` (`resultFrame`); `case` (`rowCaseKey`); `entities`;
//!   `ids`; `set` (`basis_ref.ref_id`); `station` (`metadata.location`). A
//!   member the TypeScript would throw on (a `null` row, a
//!   `source_result_refs` that is not iterable) refuses the query by name
//!   ([`FILTER_TYPE_ERROR`]), where the panel's filter would throw.
//! - **Admission and memory** (plan §8). Every lease is pool-admitted before
//!   what it covers is allocated, and none exceeds the per-invocation budget
//!   or, whatever budget is configured, the owner-adopted 12 GiB default:
//!   - the args lease, as the verdict's: the raw body's copy, grown by the
//!     args' admission bound;
//!   - building the index: the diagnostic leases of
//!     `native_verdict::prescan_in_order`; the parse lease of M while the
//!     route is read; then the scan lease (the prescan's allowances) over
//!     the borrowing passes, with the index lease of the index's exact
//!     allocation admitted before it is allocated. The index lease is held by
//!     the index and lives exactly as long (adaptive, re-sampled, as every
//!     native reader's parse; a persisting ingress-purpose lease, so it is
//!     never credited as resident);
//!   - each query's working lease, before it allocates: the largest row's
//!     priced parse for a scan that reads rows (one row at a time), each
//!     requested row's for the semantics, plus M for the semantics' source
//!     parse, or the recovery's member parses (with the receipt's once more
//!     for the recorded values a page copies), twice the requested ids' and
//!     kinds' bytes for a lookup's header, plus a fixed allowance for the
//!     header and the selection, which the request's limit bounds
//!     ([`WORKING_ALLOWANCE_BYTES`]);
//!   - the response lease of the response's exact length, admitted after
//!     the selection and before the response is allocated; it covers
//!     construction and ends when the command returns.
//! - **Residuals, unpriced** (plan §7 item 3, for slice 7): the readers' own
//!   working memory (`route`'s, `row_semantics_each`'s outcomes for the
//!   requested rows, one row's searchable text); serde_json's key scratch
//!   beyond the prescan's allowances (none: the allowances price it); and the
//!   returned buffer and Tauri's copies during delivery after the command
//!   returns, including concurrent replies.
use crate::command_ingress::{self, Args, ArgsDecoding, CommandError};
use crate::ingress;
use crate::memory_pool::Lease;
use crate::native_verdict;
use crate::result_summary;
use crate::result_summary_tables as tables;
use crate::solve_jobs::{HeldResult, SolveJobRegistry};
use open_pipe_stress_result_export::reader_verdict as rv;
use serde::de::{self, DeserializeSeed, Deserializer, IgnoredAny, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;
use serde_json::value::RawValue;
use serde_json::{json, Map, Value};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::{Arc, PoisonError};
use tauri::http::HeaderMap;
use tauri::ipc::InvokeBody;

pub(crate) const ROWS_COMMAND: &str = "query_preview_mechanics_job_rows";
pub(crate) const LOOKUP_COMMAND: &str = "lookup_preview_mechanics_job_rows";
pub(crate) const SEMANTICS_COMMAND: &str = "read_preview_mechanics_job_row_semantics";
pub(crate) const RECOVERY_COMMAND: &str = "query_preview_mechanics_job_source_recovery";
/// The blocking read ended without an outcome (it panicked).
pub(crate) const DETAIL_READ_FAILED: &str = "RESULT-DETAIL-READ-FAILED";
/// A request outside the commands' bounds (a limit, a count or an ordinal).
pub(crate) const REQUEST_INVALID: &str = "RESULT-DETAIL-REQUEST-INVALID";
/// A response that would exceed [`MAX_RESPONSE_BYTES`] with one row or case.
pub(crate) const RESPONSE_LIMIT: &str = "RESULT-DETAIL-RESPONSE-LIMIT";
/// A filter member the TypeScript filter would throw on.
pub(crate) const FILTER_TYPE_ERROR: &str = "RESULT-DETAIL-FILTER-TYPE-ERROR";
pub(crate) const MAGIC: &[u8; 8] = b"SWBDETv1";
pub(crate) const VERSION: u64 = 1;
pub(crate) const ROWS_KIND: &str = "swbpipe.desktop.result_detail_rows";
pub(crate) const LOOKUP_KIND: &str = "swbpipe.desktop.result_detail_lookup";
pub(crate) const RECOVERY_KIND: &str = "swbpipe.desktop.result_source_recovery";

/// The most rows (or recorded cases) one page returns.
pub(crate) const MAX_PAGE_ROWS: u64 = 1000;
/// The most keys one lookup, rows one semantics batch, or values one filter
/// list states.
pub(crate) const MAX_KEYS: usize = 1000;
/// The largest response a detail command returns: today's webview limit on a
/// whole result (`MAX_DISPLAYABLE_RESULT_BYTES`, PR #51), so no row a
/// displayable result holds is out of reach of a page.
pub(crate) const MAX_RESPONSE_BYTES: u64 = crate::solve_jobs::MAX_DISPLAYABLE_RESULT_BYTES;
/// Each query's fixed allowance for its header and selection, which the
/// request's limit and keys bound: at most [`MAX_KEYS`] entries of a few
/// numbers each (requested keys and recorded values are leased by size).
pub(crate) const WORKING_ALLOWANCE_BYTES: u64 = 4 << 20;
/// One span of the index.
const SPAN_BYTES: u64 = std::mem::size_of::<Span>() as u64;
/// The fixed part of an upper bound on the row header's length per row
/// (an ordinal and a length of at most 20 digits each, and separators).
const HEADER_BYTES_PER_ROW: u64 = 44;
const HEADER_FIXED_BYTES: u64 = 512;

/// A row's byte range in the held text.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Span {
    start: usize,
    end: usize,
}

/// The row index of a held result (module docs): its rows' byte ranges, in
/// publication order, with the route's table, the largest row's priced
/// parse and the text's scan allowances, and the lease of its allocation.
#[derive(Debug)]
pub(crate) struct DetailIndex {
    spans: Vec<Span>,
    /// The route's semantic table; none when the dispatch refuses the source.
    table: Option<&'static Value>,
    /// The largest row's `Value` materialization, by its own prescan.
    row_parse_bytes: u64,
    /// The held text's prescan allowances: a borrowing pass's allocations.
    scan_bytes: u64,
    /// The held text's `Value` materialization M.
    source_parse_bytes: u64,
    /// Held, never read: the index's allocation stays leased while it lives.
    #[cfg_attr(not(test), allow(dead_code))]
    lease: Lease,
}

impl DetailIndex {
    pub(crate) fn len(&self) -> usize {
        self.spans.len()
    }
    /// The index lease's bytes.
    #[cfg(test)]
    pub(crate) fn lease_bytes(&self) -> u64 {
        self.lease.bytes()
    }
    fn row<'t>(&self, text: &'t str, ordinal: usize) -> &'t str {
        let span = self.spans[ordinal];
        &text[span.start..span.end]
    }
}

/// The index lease's bytes for `rows` spans: the spans' exact allocation and
/// the shared index's own.
pub(crate) fn index_lease_bytes(rows: usize) -> u64 {
    let spans = ingress::allocation_envelope((rows as u64).saturating_mul(SPAN_BYTES)).unwrap_or(u64::MAX);
    let shared = ingress::allocation_envelope(std::mem::size_of::<DetailIndex>() as u64 + 16).unwrap_or(u64::MAX);
    spans.saturating_add(shared)
}

// ---------------------------------------------------------------------------
// The commands' common steps
// ---------------------------------------------------------------------------

/// The executor's part of each command: O(1) checks, the reservation of the
/// raw body's copy, then the copy (`native_verdict::reserve`'s steps).
pub(crate) fn reserve(registry: &SolveJobRegistry, body: &InvokeBody, headers: &HeaderMap, command: &'static str) -> Result<(Vec<u8>, Lease), CommandError<String>> {
    let raw = command_ingress::raw_args_body(body, headers, command)?;
    let lease = registry.acquire_interactive_ingress_capped(ingress::allocation_envelope(raw.len() as u64).unwrap_or(u64::MAX))?;
    Ok((raw.to_vec(), lease))
}

/// A command: [`reserve`] on the caller's executor, then `run` in a blocking
/// task that owns the copy, its lease and a registry handle.
pub(crate) async fn command<T: Send + 'static>(
    registry: &SolveJobRegistry,
    body: &InvokeBody,
    headers: &HeaderMap,
    command: &'static str,
    run: fn(SolveJobRegistry, Vec<u8>, Lease) -> Result<T, CommandError<String>>,
) -> Result<T, CommandError<String>> {
    let (raw, lease) = reserve(registry, body, headers, command)?;
    let registry = registry.clone();
    tauri::async_runtime::spawn_blocking(move || run(registry, raw, lease))
        .await
        .unwrap_or_else(|_| Err(CommandError::Command(DETAIL_READ_FAILED.into())))
}

/// The arguments' admission: the args lease grows from the copy's by their
/// admission bound before they are parsed.
fn admit(registry: &SolveJobRegistry, raw: Vec<u8>, copy: Lease, command: &'static str) -> Result<(Args, Lease), CommandError<String>> {
    let copy_bytes = copy.bytes();
    let admitted = command_ingress::admit_raw(registry, &raw, command, ArgsDecoding::Moved, |bytes| {
        let mut lease = copy;
        lease.grow_to(copy_bytes.saturating_add(bytes)).map_err(|refusal| registry.refuse(refusal))?;
        Ok(lease)
    })?;
    Ok(admitted)
}

fn invalid(message: String) -> CommandError<String> {
    CommandError::Command(format!("{REQUEST_INVALID}: {message}"))
}

/// A page's limit: 1 to [`MAX_PAGE_ROWS`].
fn page_limit(limit: u64) -> Result<usize, CommandError<String>> {
    if limit == 0 || limit > MAX_PAGE_ROWS {
        return Err(invalid(format!("limit {limit} is outside 1..={MAX_PAGE_ROWS}")));
    }
    Ok(limit as usize)
}

/// The held result's index, built on first use (module docs). The slot's
/// lock serializes concurrent first builds; the others then reuse the one
/// built. A refused or panicked build leaves the slot empty.
pub(crate) fn index(registry: &SolveJobRegistry, held: &HeldResult) -> Result<Arc<DetailIndex>, CommandError<String>> {
    let mut slot = held.detail_slot().lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(index) = slot.as_ref() {
        return Ok(Arc::clone(index));
    }
    #[cfg(test)]
    tests::gate::pass(tests::gate::key(registry, "index"));
    let text = held.text();
    let bounds = native_verdict::prescan_in_order(registry, &[("source", Some(text))])?;
    let (bound, scan_bytes) = bounds[0].expect("the source is always prescanned");
    // The route reads the whole source; its `Value` is gone before the index allocates.
    let table = {
        let _parse = registry.acquire_result_parse(bound.materialization_bytes)?;
        let source: Value = serde_json::from_str(text)
            .map_err(|error| CommandError::Command(format!("{}: source is not valid JSON: {error}", native_verdict::INPUT_JSON_INVALID)))?;
        rv::route(&source).ok().map(|(_, table)| table)
    };
    let (spans, row_parse_bytes, lease) = spans(registry, text, scan_bytes)?;
    let index = Arc::new(DetailIndex { spans, table, row_parse_bytes, scan_bytes, source_parse_bytes: bound.materialization_bytes, lease });
    *slot = Some(Arc::clone(&index));
    Ok(index)
}

/// The index's spans of `text`'s rows, with the largest row's priced parse
/// and the index lease: the borrowing passes under the scan lease
/// (`scan_bytes`, the prescan's allowances), the index lease admitted at the
/// spans' exact allocation before they are allocated.
pub(crate) fn spans(registry: &SolveJobRegistry, text: &str, scan_bytes: u64) -> Result<(Vec<Span>, u64, Lease), CommandError<String>> {
    let scan = registry.acquire_result_parse(scan_bytes)?;
    let results = last_members(text, ["results"]).map_err(read_failed)?[0];
    let mut count = 0usize;
    if let Some(results) = results {
        elements(results.get(), |_| count += 1).map_err(read_failed)?;
    }
    let lease = registry.acquire_result_parse(index_lease_bytes(count))?;
    #[cfg(test)]
    if tests::PANIC_IN_BUILD.with(std::cell::Cell::get) {
        panic!("a test's panicked build");
    }
    let mut spans = Vec::with_capacity(count);
    let mut row_parse_bytes = 0u64;
    let mut refused = None;
    if let Some(results) = results {
        let base = text.as_ptr() as usize;
        elements(results.get(), |row| {
            let token = row.get();
            let start = token.as_ptr() as usize - base;
            // At most the count: the index never reallocates.
            if spans.len() < count {
                spans.push(Span { start, end: start + token.len() });
            }
            match ingress::prescan(token.as_bytes()) {
                Ok(bound) => row_parse_bytes = row_parse_bytes.max(bound.materialization_bytes),
                Err(invalid) => refused = Some(invalid),
            }
        })
        .map_err(read_failed)?;
    }
    if let Some(invalid) = refused {
        return Err(CommandError::Command(format!("{DETAIL_READ_FAILED}: a row of results is refused by the prescan: {}", invalid.reason)));
    }
    if spans.len() != count {
        return Err(CommandError::Command(format!("{DETAIL_READ_FAILED}: results changed between passes")));
    }
    drop(scan);
    Ok((spans, row_parse_bytes, lease))
}

fn read_failed(error: serde_json::Error) -> CommandError<String> {
    CommandError::Command(format!("{DETAIL_READ_FAILED}: {error}"))
}

// ---------------------------------------------------------------------------
// Borrowing passes over the held text (nothing built, nothing copied)
// ---------------------------------------------------------------------------

/// The last member of each of `keys` in the object `text` is, as raw text
/// (`Value` keeps the last of a duplicated key); none for a text that is not
/// an object.
struct LastMembers<'k, const N: usize>([&'k str; N]);

impl<'de, const N: usize> Visitor<'de> for LastMembers<'_, N> {
    type Value = [Option<&'de RawValue>; N];
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("any JSON value")
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut found = [None; N];
        loop {
            // One key, compared with each wanted key in turn without re-reading it.
            let Some(which) = map.next_key_seed(WhichKey(&self.0))? else { break };
            match which {
                Some(at) => found[at] = Some(map.next_value::<&'de RawValue>()?),
                None => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        Ok(found)
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        while seq.next_element::<IgnoredAny>()?.is_some() {}
        Ok([None; N])
    }
    fn visit_bool<E: de::Error>(self, _: bool) -> Result<Self::Value, E> {
        Ok([None; N])
    }
    fn visit_i64<E: de::Error>(self, _: i64) -> Result<Self::Value, E> {
        Ok([None; N])
    }
    fn visit_u64<E: de::Error>(self, _: u64) -> Result<Self::Value, E> {
        Ok([None; N])
    }
    fn visit_f64<E: de::Error>(self, _: f64) -> Result<Self::Value, E> {
        Ok([None; N])
    }
    fn visit_str<E: de::Error>(self, _: &str) -> Result<Self::Value, E> {
        Ok([None; N])
    }
    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok([None; N])
    }
}

/// Which of the wanted keys a map key is, read without allocating (an
/// escaped key is decoded into serde_json's scratch, which the scan lease
/// prices).
struct WhichKey<'a, 'k, const N: usize>(&'a [&'k str; N]);

impl<'de, const N: usize> DeserializeSeed<'de> for WhichKey<'_, '_, N> {
    type Value = Option<usize>;
    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Option<usize>, D::Error> {
        deserializer.deserialize_str(self)
    }
}

impl<'de, const N: usize> Visitor<'de> for WhichKey<'_, '_, N> {
    type Value = Option<usize>;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a key")
    }
    fn visit_str<E: de::Error>(self, key: &str) -> Result<Option<usize>, E> {
        Ok(self.0.iter().position(|wanted| *wanted == key))
    }
}

/// The last member of each of `keys` of the object `text` is.
pub(crate) fn last_members<'de, const N: usize>(text: &'de str, keys: [&str; N]) -> Result<[Option<&'de RawValue>; N], serde_json::Error> {
    let mut deserializer = serde_json::Deserializer::from_str(text);
    let found = (&mut deserializer).deserialize_any(LastMembers(keys))?;
    deserializer.end()?;
    Ok(found)
}

/// Each element of the array `text` is, as raw text, in order; none for a
/// text that is not an array.
struct Elements<F>(F);

impl<'de, F: FnMut(&'de RawValue)> Visitor<'de> for Elements<F> {
    type Value = ();
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("any JSON value")
    }
    fn visit_seq<A: SeqAccess<'de>>(mut self, mut seq: A) -> Result<(), A::Error> {
        while let Some(element) = seq.next_element::<&'de RawValue>()? {
            (self.0)(element);
        }
        Ok(())
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
        while map.next_entry::<IgnoredAny, IgnoredAny>()?.is_some() {}
        Ok(())
    }
    fn visit_bool<E: de::Error>(self, _: bool) -> Result<(), E> {
        Ok(())
    }
    fn visit_i64<E: de::Error>(self, _: i64) -> Result<(), E> {
        Ok(())
    }
    fn visit_u64<E: de::Error>(self, _: u64) -> Result<(), E> {
        Ok(())
    }
    fn visit_f64<E: de::Error>(self, _: f64) -> Result<(), E> {
        Ok(())
    }
    fn visit_str<E: de::Error>(self, _: &str) -> Result<(), E> {
        Ok(())
    }
    fn visit_unit<E: de::Error>(self) -> Result<(), E> {
        Ok(())
    }
}

pub(crate) fn elements<'de>(text: &'de str, on: impl FnMut(&'de RawValue)) -> Result<(), serde_json::Error> {
    let mut deserializer = serde_json::Deserializer::from_str(text);
    (&mut deserializer).deserialize_any(Elements(on))?;
    deserializer.end()
}

// ---------------------------------------------------------------------------
// The filter
// ---------------------------------------------------------------------------

/// A rows query's filter (module docs): each member optional.
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Filter {
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    family: Option<String>,
    #[serde(default)]
    frame: Option<String>,
    #[serde(default)]
    case: Option<String>,
    #[serde(default)]
    set: Option<String>,
    #[serde(default)]
    station: Option<String>,
    #[serde(default)]
    kinds: Option<Vec<String>>,
    #[serde(default)]
    entities: Option<Vec<String>>,
    #[serde(default)]
    ids: Option<Vec<String>>,
}

/// A filter read for matching.
struct Prepared<'f> {
    family: Option<&'f str>,
    query: Option<String>,
    kinds: Option<HashSet<&'f str>>,
    frame: Option<&'f str>,
    case: Option<&'f str>,
    entities: Option<HashSet<&'f str>>,
    ids: Option<HashSet<&'f str>>,
    set: Option<&'f str>,
    station: Option<&'f str>,
}

const ALL: &str = "all";

fn list<'a>(values: &'a Option<Vec<String>>, name: &str) -> Result<Option<HashSet<&'a str>>, CommandError<String>> {
    match values {
        Some(values) if values.len() > MAX_KEYS => Err(invalid(format!("filter.{name} states {} values, more than {MAX_KEYS}", values.len()))),
        Some(values) if !values.is_empty() => Ok(Some(values.iter().map(String::as_str).collect())),
        _ => Ok(None),
    }
}

impl Filter {
    fn prepare(&self) -> Result<Prepared<'_>, CommandError<String>> {
        fn not_all(value: &Option<String>) -> Option<&str> {
            value.as_deref().filter(|value| *value != ALL)
        }
        // `filterText.trim().toLowerCase()`; an empty query admits every row.
        let query = self.text.as_deref().map(|text| text.trim_matches(tables::js_space).to_lowercase()).filter(|query| !query.is_empty());
        Ok(Prepared {
            family: not_all(&self.family),
            query,
            kinds: list(&self.kinds, "kinds")?,
            frame: not_all(&self.frame),
            case: not_all(&self.case),
            entities: list(&self.entities, "entities")?,
            ids: list(&self.ids, "ids")?,
            set: self.set.as_deref(),
            station: self.station.as_deref(),
        })
    }
}

impl Prepared<'_> {
    fn admits_every_row(&self) -> bool {
        self.family.is_none()
            && self.query.is_none()
            && self.kinds.is_none()
            && self.frame.is_none()
            && self.case.is_none()
            && self.entities.is_none()
            && self.ids.is_none()
            && self.set.is_none()
            && self.station.is_none()
    }

    /// Whether the row is admitted, in the panel's order; `Err` where the
    /// TypeScript filter throws.
    fn admits(&self, table: Option<&'static Value>, row: &Value) -> Result<bool, String> {
        if let Some(family) = self.family {
            if result_summary::row_family(table, row) != family {
                return Ok(false);
            }
        }
        if let Some(query) = &self.query {
            if !searchable(row)?.contains(query.as_str()) {
                return Ok(false);
            }
        }
        if let Some(kinds) = &self.kinds {
            if !string_in(member(row, "kind")?, kinds) {
                return Ok(false);
            }
        }
        if let Some(frame) = self.frame {
            member(row, "metadata")?;
            if tables::result_frame(row) != frame {
                return Ok(false);
            }
        }
        if let Some(case) = self.case {
            let key = tables::basis_key(member(row, "basis_ref")?);
            if key.as_deref().unwrap_or(UNSTATED_CASE) != case {
                return Ok(false);
            }
        }
        if let Some(entities) = &self.entities {
            if !string_in(member(row, "entity_ref")?, entities) {
                return Ok(false);
            }
        }
        if let Some(ids) = &self.ids {
            if !string_in(member(row, "id")?, ids) {
                return Ok(false);
            }
        }
        if let Some(set) = self.set {
            let id = member(row, "basis_ref")?.filter(|basis| !basis.is_null()).and_then(|basis| basis.get("ref_id"));
            if !matches!(id, Some(Value::String(id)) if id == set) {
                return Ok(false);
            }
        }
        if let Some(station) = self.station {
            let location = member(row, "metadata")?.filter(|metadata| !metadata.is_null()).and_then(|metadata| metadata.get("location"));
            if !matches!(location, Some(Value::String(location)) if location == station) {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

/// `rowCaseKey`'s key of a row that states no case.
const UNSTATED_CASE: &str = "unstated";

/// `row.key`: a member of a row; a `null` row throws, as in JavaScript.
fn member<'a>(row: &'a Value, key: &str) -> Result<Option<&'a Value>, String> {
    if row.is_null() {
        return Err(format!("cannot read `{key}` of a null row"));
    }
    Ok(row.get(key))
}

/// `values.includes(value)` / `value === wanted` for string values.
fn string_in(value: Option<&Value>, values: &HashSet<&str>) -> bool {
    matches!(value, Some(Value::String(value)) if values.contains(value.as_str()))
}

/// `filterResults`' searchable text of a row: its members, the truthy ones,
/// as `String` writes them, joined by spaces and lower cased.
fn searchable(row: &Value) -> Result<String, String> {
    let mut parts: Vec<Cow<'_, str>> = Vec::new();
    for key in ["id", "kind", "entity_ref", "unit"] {
        let value = member(row, key)?;
        if tables::truthy(value) {
            parts.push(tables::js_string(value));
        }
    }
    // `String(item.value)` is always present; it is falsy only when empty.
    let value = tables::js_string(row.get("value"));
    if !value.is_empty() {
        parts.push(value);
    }
    let basis = row.get("basis_ref").filter(|basis| !basis.is_null());
    let metadata = row.get("metadata").filter(|metadata| !metadata.is_null());
    for (owner, key) in [
        (basis, "ref_type"),
        (basis, "ref_id"),
        (metadata, "component"),
        (metadata, "coordinate_system"),
        (metadata, "location"),
        (metadata, "basis"),
        (metadata, "sign_convention"),
    ] {
        let value = owner.and_then(|owner| owner.get(key));
        if tables::truthy(value) {
            parts.push(tables::js_string(value));
        }
    }
    // `...(item.source_result_refs ?? [])`.
    match row.get("source_result_refs") {
        None | Some(Value::Null) => {}
        Some(Value::Array(items)) => {
            for item in items {
                if tables::truthy(Some(item)) {
                    parts.push(tables::js_string(Some(item)));
                }
            }
        }
        // A string spreads into its code points, each truthy.
        Some(Value::String(text)) => parts.extend(text.chars().map(|c| Cow::Owned(c.to_string()))),
        Some(_) => return Err("source_result_refs is not iterable".into()),
    }
    Ok(parts.join(" ").to_lowercase())
}

// ---------------------------------------------------------------------------
// The response
// ---------------------------------------------------------------------------

/// The response's exact length for a header and texts.
fn response_len(header: &[u8], texts: &[&str]) -> u64 {
    texts.iter().fold(16 + header.len() as u64, |sum, text| sum + text.len() as u64)
}

/// The response, after `reserve` admits its exact length (its guard is
/// returned with it).
fn respond<R, E>(header: &Value, texts: &[&str], reserve: impl FnOnce(u64) -> Result<R, E>) -> Result<(Vec<u8>, R), E> {
    let header = serde_json::to_vec(header).expect("a header serializes");
    let len = response_len(&header, texts);
    let guard = reserve(len)?;
    let mut out = Vec::with_capacity(len as usize);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(header.len() as u64).to_le_bytes());
    out.extend_from_slice(&header);
    for text in texts {
        out.extend_from_slice(text.as_bytes());
    }
    debug_assert!(out.len() as u64 == len);
    Ok((out, guard))
}

fn response_limit(what: String, bytes: u64, limit: u64) -> CommandError<String> {
    CommandError::Command(format!("{RESPONSE_LIMIT}: {what} needs a response of {bytes} bytes, more than the {limit}-byte limit of one detail response"))
}

// ---------------------------------------------------------------------------
// query_preview_mechanics_job_rows
// ---------------------------------------------------------------------------

/// A page of the rows a filter admits.
pub(crate) struct Page<'t> {
    pub(crate) total: u64,
    pub(crate) ordinals: Vec<usize>,
    pub(crate) rows: Vec<&'t str>,
    pub(crate) truncated: bool,
}

/// The page of `index`'s rows of `text` that `filter` admits, `limit` from
/// the `offset`-th, stopping before a row that would take the response's
/// upper bound beyond `max_bytes` (refused by name when the first does).
pub(crate) fn page<'t>(
    text: &'t str,
    index: &DetailIndex,
    filter: &Filter,
    offset: u64,
    limit: usize,
    max_bytes: u64,
) -> Result<Page<'t>, CommandError<String>> {
    let prepared = filter.prepare()?;
    let mut page = Page { total: 0, ordinals: Vec::with_capacity(limit.min(index.len())), rows: Vec::with_capacity(limit.min(index.len())), truncated: false };
    let mut bound = HEADER_FIXED_BYTES + 16;
    let mut take = |ordinal: usize, page: &mut Page<'t>| -> Result<(), CommandError<String>> {
        if page.rows.len() >= limit || page.truncated {
            return Ok(());
        }
        let row = index.row(text, ordinal);
        let next = bound + HEADER_BYTES_PER_ROW + row.len() as u64;
        if next > max_bytes {
            if page.rows.is_empty() {
                return Err(response_limit(format!("row {ordinal}"), next, max_bytes));
            }
            page.truncated = true;
            return Ok(());
        }
        bound = next;
        page.ordinals.push(ordinal);
        page.rows.push(row);
        Ok(())
    };
    if prepared.admits_every_row() {
        page.total = index.len() as u64;
        let start = offset.min(index.len() as u64) as usize;
        for ordinal in start..index.len() {
            take(ordinal, &mut page)?;
            if page.rows.len() >= limit || page.truncated {
                break;
            }
        }
        return Ok(page);
    }
    for ordinal in 0..index.len() {
        let row: Value = serde_json::from_str(index.row(text, ordinal)).map_err(read_failed)?;
        let admitted = prepared
            .admits(index.table, &row)
            .map_err(|reason| CommandError::Command(format!("{FILTER_TYPE_ERROR}: row {ordinal}: {reason}")))?;
        drop(row);
        if !admitted {
            continue;
        }
        if page.total >= offset {
            take(ordinal, &mut page)?;
        }
        page.total += 1;
    }
    Ok(page)
}

fn rows_header(job_id: &str, page: &Page<'_>, offset: u64) -> Value {
    json!({
        "document_kind": ROWS_KIND,
        "version": VERSION,
        "job_id": job_id,
        "total": page.total,
        "offset": offset,
        "returned": page.rows.len(),
        "truncated": page.truncated,
        "ordinals": page.ordinals,
        "lengths": page.rows.iter().map(|row| row.len()).collect::<Vec<_>>(),
    })
}

pub(crate) async fn query_rows(registry: &SolveJobRegistry, body: &InvokeBody, headers: &HeaderMap) -> Result<Vec<u8>, CommandError<String>> {
    command(registry, body, headers, ROWS_COMMAND, run_rows).await
}

pub(crate) fn run_rows(registry: SolveJobRegistry, raw: Vec<u8>, copy: Lease) -> Result<Vec<u8>, CommandError<String>> {
    run_rows_within(registry, raw, copy, MAX_RESPONSE_BYTES)
}

pub(crate) fn run_rows_within(registry: SolveJobRegistry, raw: Vec<u8>, copy: Lease, max_bytes: u64) -> Result<Vec<u8>, CommandError<String>> {
    #[cfg(test)]
    tests::gate::pass(tests::gate::key(&registry, "admit"));
    let (mut args, lease) = admit(&registry, raw, copy, ROWS_COMMAND)?;
    let job_id: String = args.decode("jobId")?;
    let filter: Option<Filter> = args.decode("filter")?;
    let offset: u64 = args.decode("offset")?;
    let limit = page_limit(args.decode("limit")?)?;
    drop(args);
    let filter = filter.unwrap_or_default();
    let held = registry.held_result(&job_id)?;
    let index = index(&registry, &held)?;
    let scans = !filter.prepare()?.admits_every_row();
    let working = registry.acquire_result_parse(WORKING_ALLOWANCE_BYTES.saturating_add(if scans { index.row_parse_bytes } else { 0 }))?;
    #[cfg(test)]
    tests::gate::pass(tests::gate::key(&registry, "read"));
    let page = page(held.text(), &index, &filter, offset, limit, max_bytes)?;
    let header = rows_header(&job_id, &page, offset);
    let (response, written) = respond(&header, &page.rows, |bytes| registry.acquire_result_parse(bytes))?;
    drop((written, page));
    drop((working, index, held, lease));
    Ok(response)
}

// ---------------------------------------------------------------------------
// lookup_preview_mechanics_job_rows
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Match {
    First,
    Last,
}

/// For each id (or kind), the first or last row with it, by ordinal.
fn find_by(text: &str, index: &DetailIndex, ids: &[String], kinds: &[String], matching: Match) -> Result<(Vec<Option<usize>>, Vec<Option<usize>>), CommandError<String>> {
    let mut found_ids: Vec<Option<usize>> = vec![None; ids.len()];
    let mut found_kinds: Vec<Option<usize>> = vec![None; kinds.len()];
    if ids.is_empty() && kinds.is_empty() {
        return Ok((found_ids, found_kinds));
    }
    let mut id_keys: HashMap<&str, Vec<usize>> = HashMap::new();
    for (at, id) in ids.iter().enumerate() {
        id_keys.entry(id.as_str()).or_default().push(at);
    }
    let mut kind_keys: HashMap<&str, Vec<usize>> = HashMap::new();
    for (at, kind) in kinds.iter().enumerate() {
        kind_keys.entry(kind.as_str()).or_default().push(at);
    }
    let wanted = id_keys.len() + kind_keys.len();
    let mut seen: HashSet<(bool, &str)> = HashSet::new();
    for ordinal in 0..index.len() {
        let row: Value = serde_json::from_str(index.row(text, ordinal)).map_err(read_failed)?;
        // `candidate.id === id`: a row's member; a `null` row throws.
        let (id, kind) = match &row {
            Value::Null => return Err(CommandError::Command(format!("{FILTER_TYPE_ERROR}: row {ordinal}: cannot read `id` of a null row"))),
            row => (row.get("id"), row.get("kind")),
        };
        for (value, keys, found, is_id) in [(id, &id_keys, &mut found_ids, true), (kind, &kind_keys, &mut found_kinds, false)] {
            let Some(Value::String(value)) = value else { continue };
            let Some((key, at)) = keys.get_key_value(value.as_str()) else { continue };
            if matching == Match::First && found[at[0]].is_some() {
                continue;
            }
            for &position in at {
                found[position] = Some(ordinal);
            }
            seen.insert((is_id, key));
        }
        if matching == Match::First && seen.len() == wanted {
            break;
        }
    }
    Ok((found_ids, found_kinds))
}

pub(crate) async fn lookup_rows(registry: &SolveJobRegistry, body: &InvokeBody, headers: &HeaderMap) -> Result<Vec<u8>, CommandError<String>> {
    command(registry, body, headers, LOOKUP_COMMAND, run_lookup).await
}

pub(crate) fn run_lookup(registry: SolveJobRegistry, raw: Vec<u8>, copy: Lease) -> Result<Vec<u8>, CommandError<String>> {
    run_lookup_within(registry, raw, copy, MAX_RESPONSE_BYTES)
}

pub(crate) fn run_lookup_within(registry: SolveJobRegistry, raw: Vec<u8>, copy: Lease, max_bytes: u64) -> Result<Vec<u8>, CommandError<String>> {
    let (mut args, lease) = admit(&registry, raw, copy, LOOKUP_COMMAND)?;
    let job_id: String = args.decode("jobId")?;
    let ordinals: Vec<u64> = args.decode::<Option<Vec<u64>>>("ordinals")?.unwrap_or_default();
    let ids: Vec<String> = args.decode::<Option<Vec<String>>>("ids")?.unwrap_or_default();
    let kinds: Vec<String> = args.decode::<Option<Vec<String>>>("kinds")?.unwrap_or_default();
    let matching: Match = args.decode("match")?;
    drop(args);
    let keys = ordinals.len() + ids.len() + kinds.len();
    if keys == 0 || keys > MAX_KEYS {
        return Err(invalid(format!("a lookup states {keys} keys, not 1..={MAX_KEYS}")));
    }
    let held = registry.held_result(&job_id)?;
    let index = index(&registry, &held)?;
    let scans = !(ids.is_empty() && kinds.is_empty());
    // The header copies each requested id and kind (and writes it once more).
    let keys_bytes = ids.iter().chain(&kinds).map(|key| 2 * key.len() as u64).sum::<u64>();
    let working = registry.acquire_result_parse(
        WORKING_ALLOWANCE_BYTES.saturating_add(keys_bytes).saturating_add(if scans { index.row_parse_bytes } else { 0 }),
    )?;
    let text = held.text();
    let (by_id, by_kind) = find_by(text, &index, &ids, &kinds, matching)?;
    // Each row once among the texts; entries name it by position.
    let mut texts: Vec<&str> = Vec::new();
    let mut positions: HashMap<usize, usize> = HashMap::new();
    let mut entry = |ordinal: Option<usize>| -> Value {
        match ordinal {
            Some(ordinal) => {
                let at = *positions.entry(ordinal).or_insert_with(|| {
                    texts.push(index.row(text, ordinal));
                    texts.len() - 1
                });
                json!({"ordinal": ordinal, "text": at})
            }
            None => Value::Null,
        }
    };
    let by_ordinal: Vec<Value> = ordinals
        .iter()
        .map(|&ordinal| {
            let found = (ordinal < index.len() as u64).then_some(ordinal as usize);
            json!({"ordinal": ordinal, "row": entry(found)})
        })
        .collect();
    let by_id: Vec<Value> = ids.iter().zip(by_id).map(|(id, found)| json!({"id": id, "row": entry(found)})).collect();
    let by_kind: Vec<Value> = kinds.iter().zip(by_kind).map(|(kind, found)| json!({"kind": kind, "row": entry(found)})).collect();
    let header = json!({
        "document_kind": LOOKUP_KIND,
        "version": VERSION,
        "job_id": job_id,
        "row_count": index.len(),
        "match": match matching { Match::First => "first", Match::Last => "last" },
        "ordinals": by_ordinal,
        "ids": by_id,
        "kinds": by_kind,
        "lengths": texts.iter().map(|text| text.len()).collect::<Vec<_>>(),
    });
    let (response, written) = respond(&header, &texts, |bytes| {
        if bytes > max_bytes {
            return Err(response_limit(format!("a lookup of {keys} keys"), bytes, max_bytes));
        }
        registry.acquire_result_parse(bytes).map_err(CommandError::from)
    })?;
    drop((written, working, index, held, lease));
    Ok(response)
}

// ---------------------------------------------------------------------------
// read_preview_mechanics_job_row_semantics
// ---------------------------------------------------------------------------

pub(crate) async fn row_semantics(registry: &SolveJobRegistry, body: &InvokeBody, headers: &HeaderMap) -> Result<String, CommandError<String>> {
    command(registry, body, headers, SEMANTICS_COMMAND, run_semantics).await
}

/// Counts the bytes written.
struct Counter(u64);

impl std::io::Write for Counter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0 += bytes.len() as u64;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub(crate) fn run_semantics(registry: SolveJobRegistry, raw: Vec<u8>, copy: Lease) -> Result<String, CommandError<String>> {
    let (mut args, lease) = admit(&registry, raw, copy, SEMANTICS_COMMAND)?;
    let job_id: String = args.decode("jobId")?;
    let ordinals: Vec<u64> = args.decode("ordinals")?;
    drop(args);
    if ordinals.is_empty() || ordinals.len() > MAX_KEYS {
        return Err(invalid(format!("a semantics batch states {} rows, not 1..={MAX_KEYS}", ordinals.len())));
    }
    let held = registry.held_result(&job_id)?;
    let index = index(&registry, &held)?;
    if let Some(ordinal) = ordinals.iter().find(|&&ordinal| ordinal >= index.len() as u64) {
        return Err(invalid(format!("row {ordinal} is outside the result's {} rows", index.len())));
    }
    let rows_bytes = index.row_parse_bytes.saturating_mul(ordinals.len() as u64);
    let working = registry.acquire_result_parse(index.source_parse_bytes.saturating_add(rows_bytes).saturating_add(WORKING_ALLOWANCE_BYTES))?;
    let text = held.text();
    let source: Value = serde_json::from_str(text).map_err(read_failed)?;
    let rows = ordinals.iter().map(|&ordinal| serde_json::from_str::<Value>(index.row(text, ordinal as usize))).collect::<Result<Vec<_>, _>>().map_err(read_failed)?;
    let outcomes = rv::row_semantics_each(&rows, Some(&source));
    drop((rows, source));
    let mut counter = Counter(0);
    serde_json::to_writer(&mut counter, &outcomes).expect("outcomes serialize");
    let written = registry.acquire_result_parse(counter.0)?;
    let mut out = Vec::with_capacity(counter.0 as usize);
    serde_json::to_writer(&mut out, &outcomes).expect("outcomes serialize");
    drop((outcomes, written, working, index, held, lease));
    Ok(String::from_utf8(out).expect("serde_json writes UTF-8"))
}

// ---------------------------------------------------------------------------
// query_preview_mechanics_job_source_recovery
// ---------------------------------------------------------------------------

/// `record(v)`: a non-array object.
fn record(value: Option<&Value>) -> Option<&Map<String, Value>> {
    value.and_then(Value::as_object)
}

/// `rows(v)`: an array's records, in order, with their positions in it.
fn records(value: Option<&Value>) -> Vec<(usize, &Map<String, Value>)> {
    value.and_then(Value::as_array).map_or_else(Vec::new, |items| items.iter().enumerate().filter_map(|(at, item)| item.as_object().map(|r| (at, r))).collect())
}

/// The panel's `text(v)`: a non-empty string, else `unavailable`.
fn label<'a>(value: Option<&'a Value>) -> &'a str {
    match value {
        Some(Value::String(text)) if !text.is_empty() => text,
        _ => "unavailable",
    }
}

/// A JSON value's kind as the recovery facts state it.
fn kind_of(value: Option<&Value>) -> &'static str {
    match value {
        None => "absent",
        Some(Value::Null) => "null",
        Some(Value::Bool(_)) => "boolean",
        Some(Value::Number(_)) => "number",
        Some(Value::String(_)) => "string",
        Some(Value::Array(_)) => "array",
        Some(Value::Object(_)) => "object",
    }
}

/// The recovery facts and a page of the recorded cases, with their exact
/// texts' positions in `case_texts` when `raw`.
pub(crate) fn recovery_header(
    receipt: Option<&Value>,
    evidence: Option<&Value>,
    case_texts: &[&str],
    offset: u64,
    limit: usize,
    raw: bool,
    max_bytes: u64,
) -> Result<(Value, Vec<usize>), CommandError<String>> {
    let body = record(receipt).and_then(|r| r.get("body"));
    let body_record = record(body);
    let cases = records(body_record.and_then(|b| b.get("cases")));
    let exact_cases = records(record(evidence).and_then(|e| e.get("exact_cases")));
    let receipt_ids: Vec<&str> = cases.iter().map(|(_, case)| label(record(case.get("basis_ref")).and_then(|b| b.get("ref_id")))).collect();
    let physical_ids: Vec<&str> = exact_cases.iter().map(|(_, case)| label(case.get("load_case_id"))).collect();
    let distinct: HashSet<&str> = receipt_ids.iter().copied().collect();
    let coverage = receipt_ids.len() == physical_ids.len() && distinct.len() == receipt_ids.len() && receipt_ids.iter().all(|id| physical_ids.contains(id));
    let start = offset.min(cases.len() as u64) as usize;
    let mut page = Vec::new();
    let mut texts = Vec::new();
    let mut bound = HEADER_FIXED_BYTES + 16;
    for (index, (entry, case)) in cases.iter().enumerate().skip(start).take(limit) {
        let text_len = if raw { case_texts[*entry].len() as u64 } else { 0 };
        let facts = json!({
            "index": index,
            "entry": entry,
            "ref_id": receipt_ids[index],
            "selected_method": case.get("selected_method").cloned().unwrap_or(Value::Null),
            "outcome": case.get("outcome").cloned().unwrap_or(Value::Null),
            "text": if raw { json!(texts.len()) } else { Value::Null },
        });
        let next = bound + HEADER_BYTES_PER_ROW + facts.to_string().len() as u64 + text_len;
        if next > max_bytes {
            if page.is_empty() {
                return Err(response_limit(format!("recorded case {index}"), next, max_bytes));
            }
            break;
        }
        bound = next;
        if raw {
            texts.push(*entry);
        }
        page.push(facts);
    }
    let returned = page.len();
    let status = body_record.and_then(|b| b.get("status"));
    let header = json!({
        "document_kind": RECOVERY_KIND,
        "version": VERSION,
        "present": receipt.is_some(),
        "receipt_kind": kind_of(receipt),
        "body_kind": kind_of(body),
        "status_present": status.is_some(),
        "status": status.cloned().unwrap_or(Value::Null),
        "receipt_sha256": record(receipt).and_then(|r| r.get("receipt_sha256")).cloned().unwrap_or(Value::Null),
        "receipt_version": body_record.and_then(|b| b.get("receipt_version")).cloned().unwrap_or(Value::Null),
        "publication_sha256": body_record.and_then(|b| b.get("publication_sha256")).cloned().unwrap_or(Value::Null),
        "case_count": cases.len(),
        "physical_case_count": exact_cases.len(),
        "receipt_ids_distinct": distinct.len() == receipt_ids.len(),
        "coverage_matches": coverage,
        "offset": offset,
        "returned": returned,
        "truncated": start + returned < cases.len() && returned < limit,
        "cases": page,
    });
    Ok((header, texts))
}

pub(crate) async fn query_source_recovery(registry: &SolveJobRegistry, body: &InvokeBody, headers: &HeaderMap) -> Result<Vec<u8>, CommandError<String>> {
    command(registry, body, headers, RECOVERY_COMMAND, run_recovery).await
}

pub(crate) fn run_recovery(registry: SolveJobRegistry, raw: Vec<u8>, copy: Lease) -> Result<Vec<u8>, CommandError<String>> {
    run_recovery_within(registry, raw, copy, MAX_RESPONSE_BYTES)
}

/// The priced parse of a member's text (the prescan's bound; the held text
/// was prescanned when its index was built, so its members are admitted).
fn member_parse_bytes(text: Option<&RawValue>) -> Result<u64, CommandError<String>> {
    match text {
        None => Ok(0),
        Some(text) => ingress::prescan(text.get().as_bytes())
            .map(|bound| bound.materialization_bytes)
            .map_err(|invalid| CommandError::Command(format!("{DETAIL_READ_FAILED}: a member is refused by the prescan: {}", invalid.reason))),
    }
}

pub(crate) fn run_recovery_within(registry: SolveJobRegistry, raw: Vec<u8>, copy: Lease, max_bytes: u64) -> Result<Vec<u8>, CommandError<String>> {
    let (mut args, lease) = admit(&registry, raw, copy, RECOVERY_COMMAND)?;
    let job_id: String = args.decode("jobId")?;
    let offset: u64 = args.decode("offset")?;
    let limit = page_limit(args.decode("limit")?)?;
    let raw_texts: bool = args.decode::<Option<bool>>("raw")?.unwrap_or(false);
    drop(args);
    let held = registry.held_result(&job_id)?;
    let index = index(&registry, &held)?;
    let text = held.text();
    let scan = registry.acquire_result_parse(index.scan_bytes)?;
    let [receipt_text, evidence_text] = last_members(text, ["source_block_recovery", "contract_evidence"]).map_err(read_failed)?;
    let receipt_bytes = member_parse_bytes(receipt_text)?;
    let parse_bytes = receipt_bytes.saturating_add(member_parse_bytes(evidence_text)?);
    // The page's facts copy recorded values, which are parts of the receipt: at most its parse again.
    let working = registry.acquire_result_parse(parse_bytes.saturating_add(receipt_bytes).saturating_add(WORKING_ALLOWANCE_BYTES))?;
    let parse = |text: Option<&RawValue>| text.map(|text| serde_json::from_str::<Value>(text.get())).transpose().map_err(read_failed);
    let receipt = parse(receipt_text)?;
    let evidence = parse(evidence_text)?;
    // The recorded cases' exact texts, as `Value` reads `body.cases` (the last of each key).
    let mut case_texts: Vec<&str> = Vec::new();
    if raw_texts {
        if let Some(receipt_text) = receipt_text {
            if let [Some(body)] = last_members(receipt_text.get(), ["body"]).map_err(read_failed)? {
                if let [Some(cases)] = last_members(body.get(), ["cases"]).map_err(read_failed)? {
                    elements(cases.get(), |case| case_texts.push(case.get())).map_err(read_failed)?;
                }
            }
        }
    }
    drop(scan);
    let (header, positions) = recovery_header(receipt.as_ref(), evidence.as_ref(), &case_texts, offset, limit, raw_texts, max_bytes)?;
    drop((receipt, evidence));
    let mut header = header;
    header["job_id"] = json!(job_id);
    let texts: Vec<&str> = positions.iter().map(|&entry| case_texts[entry]).collect();
    header["lengths"] = json!(texts.iter().map(|text| text.len()).collect::<Vec<_>>());
    let (response, written) = respond(&header, &texts, |bytes| {
        if bytes > max_bytes {
            return Err(response_limit("the recovery page".into(), bytes, max_bytes));
        }
        registry.acquire_result_parse(bytes).map_err(CommandError::from)
    })?;
    drop((written, working, index, held, lease));
    Ok(response)
}

#[cfg(test)]
#[path = "result_detail_tests.rs"]
pub(crate) mod tests;
