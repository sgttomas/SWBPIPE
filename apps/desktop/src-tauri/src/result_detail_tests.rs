//! C1 slice 3: the paged detail of a retained result (`result_detail`).
//!
//! - **Exact bytes.** Paging through every row of each file-backed corpus
//!   statement and each summary case's result returns each element of
//!   `results` as its exact token: in order, the pages' rows and the
//!   whitespace and commas between them rebuild the `results` text serde's
//!   derive borrows, byte for byte.
//! - **Committed outcomes.** Every query, lookup and recovery page of
//!   `fixtures/results/result_detail_cases.json` (written by
//!   `resultDetailCases.ts`) runs through its command on a retained job and
//!   must equal its committed outcome in
//!   `fixtures/results/result_detail_outcomes.json`
//!   (`RESULT_DETAIL_OUTCOMES_WRITE=1` rewrites it). The vitest leg
//!   (`resultDetailParity.test.ts`) checks the same outcomes against the
//!   TypeScript filters, lookups and panels over the full rows.
//! - **Semantics.** Every row of each file-backed corpus statement, read in
//!   batches through the command, gives the committed reader outcomes'
//!   `rows.sha256` (`base_reader_outcomes.json`), which the wasm engine's
//!   `readerRowSemanticsJson` is pinned to as well.
//! - Bounds, refusals, the index's lease and lifetime, concurrent first
//!   queries and the executor.
use super::*;
use crate::command_ingress::test_support::headers;
use crate::command_ingress::INGRESS_ARGS_JSON_V1;
use crate::memory_pool::test_host::ScriptedHost;
use crate::memory_pool::{PressureLevel, ResourceRefusal};
pub(crate) use crate::native_verdict::tests::gate;
use crate::native_verdict::tests::{committed, file_backed_corpus, gated, retained, runtime_workers};
use crate::solve_jobs::SolveJobError;
use crate::solve_worker::test_support::Gate;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use std::time::Duration;

const CASES: &str = "fixtures/results/result_detail_cases.json";
const OUTCOMES: &str = "fixtures/results/result_detail_outcomes.json";
const DEMO: &str = "fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json";

fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}
fn read_text(path: &str) -> String {
    std::fs::read_to_string(root().join(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}
fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
}

/// A command's two steps in this thread: the executor's reservation, then the blocking part.
fn call<T>(
    registry: &SolveJobRegistry,
    command: &'static str,
    run: impl FnOnce(SolveJobRegistry, Vec<u8>, Lease) -> Result<T, CommandError<String>>,
    args: &Value,
) -> Result<T, CommandError<String>> {
    let body = InvokeBody::Raw(serde_json::to_vec(args).unwrap());
    let (raw, lease) = reserve(registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1)), command)?;
    run(registry.clone(), raw, lease)
}
fn rows(registry: &SolveJobRegistry, args: &Value) -> Result<Decoded, CommandError<String>> {
    call(registry, ROWS_COMMAND, run_rows, args).map(|bytes| decode(&bytes))
}
fn rows_within(registry: &SolveJobRegistry, args: &Value, max: u64) -> Result<Decoded, CommandError<String>> {
    call(registry, ROWS_COMMAND, |r, raw, lease| run_rows_within(r, raw, lease, max), args).map(|bytes| decode(&bytes))
}
fn lookup(registry: &SolveJobRegistry, args: &Value) -> Result<Decoded, CommandError<String>> {
    call(registry, LOOKUP_COMMAND, run_lookup, args).map(|bytes| decode(&bytes))
}
fn semantics(registry: &SolveJobRegistry, args: &Value) -> Result<String, CommandError<String>> {
    call(registry, SEMANTICS_COMMAND, run_semantics, args)
}
fn recovery(registry: &SolveJobRegistry, args: &Value) -> Result<Decoded, CommandError<String>> {
    call(registry, RECOVERY_COMMAND, run_recovery, args).map(|bytes| decode(&bytes))
}

/// A detail response: its header and texts, its layout checked.
#[derive(Debug)]
struct Decoded {
    header: Value,
    texts: Vec<String>,
}

fn decode(bytes: &[u8]) -> Decoded {
    assert_eq!(&bytes[..8], MAGIC);
    let length = u64::from_le_bytes(bytes[8..16].try_into().unwrap()) as usize;
    let header: Value = serde_json::from_slice(&bytes[16..16 + length]).unwrap();
    let mut at = 16 + length;
    let texts = header["lengths"]
        .as_array()
        .unwrap()
        .iter()
        .map(|len| {
            let len = len.as_u64().unwrap() as usize;
            let text = std::str::from_utf8(&bytes[at..at + len]).unwrap().to_string();
            at += len;
            text
        })
        .collect();
    assert_eq!(at, bytes.len(), "nothing follows the last text");
    Decoded { header, texts }
}

fn command_text(error: CommandError<String>) -> String {
    match error {
        CommandError::Command(text) => text,
        other => panic!("not a command refusal: {other:?}"),
    }
}
fn refusal(error: CommandError<String>) -> ResourceRefusal {
    match error {
        CommandError::Ingress(SolveJobError::ResourceUnavailable { refusal, .. }) => refusal,
        other => panic!("not a resource refusal: {other:?}"),
    }
}
fn invalid_message(error: CommandError<String>) -> String {
    match error {
        CommandError::Ingress(SolveJobError::InvalidRequest { message }) => message,
        other => panic!("not an invalid request: {other:?}"),
    }
}

/// `results`' own text, read by serde_json's map of raw members (independent
/// of the index's passes; the last of a duplicated key, as `Value` keeps it).
fn results_text(source: &str) -> Option<String> {
    member_text(source, "results")
}
fn member_text(text: &str, key: &str) -> Option<String> {
    let members: std::collections::HashMap<String, Box<RawValue>> = serde_json::from_str(text).ok()?;
    members.get(key).map(|raw| raw.get().to_string())
}

/// Every row, paged at `limit`, from offset 0 to the end.
fn all_rows(registry: &SolveJobRegistry, job: &str, limit: u64) -> (Vec<u64>, Vec<String>) {
    let (mut ordinals, mut texts) = (Vec::new(), Vec::new());
    loop {
        let page = rows(registry, &json!({"jobId": job, "offset": ordinals.len(), "limit": limit})).unwrap();
        let returned = page.header["returned"].as_u64().unwrap();
        ordinals.extend(page.header["ordinals"].as_array().unwrap().iter().map(|o| o.as_u64().unwrap()));
        texts.extend(page.texts);
        if returned == 0 {
            assert_eq!(page.header["total"].as_u64().unwrap(), ordinals.len() as u64);
            return (ordinals, texts);
        }
    }
}

/// The rows, in order, with only whitespace and commas between them, are
/// `results`' text from its `[` to its `]`.
fn assert_rebuilds(results: &str, texts: &[String], name: &str) {
    let ws = |s: &str| s.trim_start_matches([' ', '\n', '\r', '\t']).len();
    let mut rest = results.strip_prefix('[').unwrap_or_else(|| panic!("{name}: results is not an array"));
    for (i, text) in texts.iter().enumerate() {
        rest = &rest[rest.len() - ws(rest)..];
        if i > 0 {
            rest = rest.strip_prefix(',').unwrap_or_else(|| panic!("{name}: no comma before row {i}"));
            rest = &rest[rest.len() - ws(rest)..];
        }
        rest = rest.strip_prefix(text.as_str()).unwrap_or_else(|| panic!("{name}: row {i} is not the next token"));
    }
    rest = &rest[rest.len() - ws(rest)..];
    assert_eq!(rest, "]", "{name}: rows after the last");
}

// ---------------------------------------------------------------------------
// Exact bytes
// ---------------------------------------------------------------------------

/// Every source the tests page: the file-backed corpus and the summary cases' results.
fn sources() -> Vec<(String, String)> {
    let mut sources: Vec<(String, String)> = file_backed_corpus().into_iter().map(|input| (input.id, input.source)).collect();
    for case in serde_json::from_str::<Value>(&read_text(CASES)).unwrap()["cases"].as_array().unwrap() {
        sources.push((case["name"].as_str().unwrap().into(), text_of(&case["result"])));
    }
    sources
}

#[test]
fn unfiltered_pages_reproduce_every_row_byte_for_byte_in_order() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let mut checked = 0;
    for (name, source) in sources() {
        let job = retained(&registry, &gate, &source);
        let value: Value = serde_json::from_str(&source).unwrap();
        let expected = value["results"].as_array().map_or(0, Vec::len);
        for limit in [1, 7, MAX_PAGE_ROWS] {
            if limit == 1 && expected > 400 {
                continue;
            }
            let (ordinals, texts) = all_rows(&registry, &job, limit);
            assert_eq!(ordinals, (0..expected as u64).collect::<Vec<_>>(), "{name} at {limit}");
            if let Some(results) = results_text(&source).filter(|r| r.starts_with('[')) {
                assert_rebuilds(&results, &texts, &name);
            }
            for (i, text) in texts.iter().enumerate() {
                assert_eq!(serde_json::from_str::<Value>(text).unwrap(), value["results"][i], "{name} row {i}");
            }
        }
        registry.dispose_result(&job).unwrap();
        checked += 1;
    }
    assert!(checked >= 92, "{checked} sources");
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// Bounds: the limit must be 1..=1000; an offset at or past the end returns
/// no row and the total; an empty, absent or non-array `results` has none;
/// the last of a duplicated `results` member is read, as `Value` reads it;
/// tokens keep their own bytes (escapes, exponents, whitespace around them).
#[test]
fn bounds_and_shapes_page_consistently() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let text = r#"{"results": [ {"id":"a","value":1E+0} ,{"id":"b","value":-0.0},
        [1,2] , null,"s" ], "x":{"results":[9]}}"#;
    let job = retained(&registry, &gate, text);
    let (ordinals, texts) = all_rows(&registry, &job, 2);
    assert_eq!(ordinals, vec![0, 1, 2, 3, 4]);
    assert_eq!(texts, vec![r#"{"id":"a","value":1E+0}"#, r#"{"id":"b","value":-0.0}"#, "[1,2]", "null", r#""s""#]);
    for (offset, returned) in [(4u64, 1u64), (5, 0), (6, 0), (u64::MAX, 0)] {
        let page = rows(&registry, &json!({"jobId": job, "offset": offset, "limit": 3})).unwrap();
        assert_eq!((page.header["total"].as_u64(), page.header["returned"].as_u64()), (Some(5), Some(returned)), "offset {offset}");
    }
    for limit in [json!(0), json!(MAX_PAGE_ROWS + 1)] {
        let message = command_text(rows(&registry, &json!({"jobId": job, "offset": 0, "limit": limit})).unwrap_err());
        assert!(message.starts_with(REQUEST_INVALID), "{message}");
    }
    for bad in [json!({"jobId": job, "offset": -1, "limit": 1}), json!({"jobId": job, "offset": 0, "limit": 1.5}), json!({"jobId": job, "offset": 0}), json!({"jobId": job, "offset": 0, "limit": 1, "filter": {"colour": "x"}})] {
        let message = invalid_message(rows(&registry, &bad).unwrap_err());
        assert!(message.starts_with("INGRESS-ARGS-INVALID"), "{message}");
    }
    // A filter list, a lookup and a semantics batch state at most 1,000 values or keys.
    let many: Vec<String> = (0..=MAX_KEYS).map(|i| format!("k{i}")).collect();
    let message = command_text(rows(&registry, &json!({"jobId": job, "filter": {"kinds": many}, "offset": 0, "limit": 1})).unwrap_err());
    assert!(message.starts_with(REQUEST_INVALID), "{message}");
    let message = command_text(lookup(&registry, &json!({"jobId": job, "ids": many, "match": "first"})).unwrap_err());
    assert!(message.starts_with(REQUEST_INVALID), "{message}");
    let message = invalid_message(lookup(&registry, &json!({"jobId": job, "ids": ["a"], "match": "any"})).unwrap_err());
    assert!(message.starts_with("INGRESS-ARGS-INVALID"), "{message}");
    // 1,000 are admitted (and read row 3, the null row, as the panel's filter would: refused).
    let message = command_text(rows(&registry, &json!({"jobId": job, "filter": {"kinds": many[..MAX_KEYS]}, "offset": 0, "limit": 1})).unwrap_err());
    assert!(message.starts_with(&format!("{FILTER_TYPE_ERROR}: row 3")), "{message}");
    registry.dispose_result(&job).unwrap();
    for (text, total) in [(r#"{"results":[]}"#, 0), (r#"{"other":1}"#, 0), (r#"{"results":{"a":1}}"#, 0), (r#"[1,2]"#, 0), (r#"{"results":[1,2,3],"results":[4]}"#, 1)] {
        let job = retained(&registry, &gate, text);
        let (ordinals, texts) = all_rows(&registry, &job, 10);
        assert_eq!(ordinals.len(), total, "{text}");
        if total == 1 {
            assert_eq!(texts, vec!["4"]);
        }
        registry.dispose_result(&job).unwrap();
    }
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// A page stops before the response limit and pages on from where it
/// stopped; a row that alone exceeds the limit is refused by name.
#[test]
fn pages_stay_within_the_response_limit_and_still_reach_every_row() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let text = read_text(DEMO);
    let job = retained(&registry, &gate, &text);
    let (_, all) = all_rows(&registry, &job, MAX_PAGE_ROWS);
    let longest = all.iter().map(String::len).max().unwrap() as u64;
    let max = HEADER_FIXED_BYTES + 16 + 3 * (HEADER_BYTES_PER_ROW + longest);
    let mut got = Vec::new();
    let mut truncated = 0;
    loop {
        let page = rows_within(&registry, &json!({"jobId": job, "offset": got.len(), "limit": MAX_PAGE_ROWS, "filter": {}}), max).unwrap();
        let length = 16 + page.header.to_string().len() as u64 + page.texts.iter().map(|t| t.len() as u64).sum::<u64>();
        assert!(length <= max, "{length} > {max}");
        truncated += page.header["truncated"].as_bool().unwrap() as usize;
        if page.texts.is_empty() {
            break;
        }
        got.extend(page.texts);
    }
    assert_eq!(got, all);
    assert!(truncated > 1);
    let message = command_text(rows_within(&registry, &json!({"jobId": job, "offset": 0, "limit": 1}), longest).unwrap_err());
    assert!(message.starts_with(RESPONSE_LIMIT), "{message}");
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

#[test]
fn invalid_filter_lists_are_refused_before_the_first_index_build() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let job = retained(&registry, &gate, r#"{"results":[{"id":"a"}]}"#);
    let held = registry.held_result(&job).unwrap();
    assert!(held.detail_slot().lock().unwrap().is_none());
    let many: Vec<String> = (0..=MAX_KEYS).map(|i| format!("kind:{i}")).collect();
    let message = command_text(rows(&registry, &json!({"jobId": job, "filter": {"kinds": many}, "offset": 0, "limit": 1})).unwrap_err());
    assert!(message.starts_with(REQUEST_INVALID), "{message}");
    assert!(held.detail_slot().lock().unwrap().is_none());
    drop(held);
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

#[test]
fn semantics_use_tauris_raw_response_body() {
    use tauri::ipc::{InvokeResponseBody, IpcResponse};
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let job = retained(&registry, &gate, r#"{"results":[{"id":"a","kind":"x"}]}"#);
    let json = semantics(&registry, &json!({"jobId": job, "ordinals": [0]})).unwrap();
    match semantics_response(json.clone()).body().unwrap() {
        InvokeResponseBody::Raw(bytes) => assert_eq!(bytes, json.as_bytes()),
        InvokeResponseBody::Json(_) => panic!("Tauri would parse the array before the adapter receives it"),
    }
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

#[test]
fn small_recovery_responses_truncate_without_stranding_a_fitting_case() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let source = json!({
        "results": [],
        "source_block_recovery": {
            "receipt_sha256": "a".repeat(64),
            "body": {"status": "qualified", "receipt_version": 1, "publication_sha256": "b".repeat(64),
                "cases": [
                    {"basis_ref": {"ref_id": "case:a"}, "selected_method": "method", "outcome": "qualified"},
                    {"basis_ref": {"ref_id": "case:b"}, "selected_method": "method", "outcome": "qualified"},
                ]},
        },
        "contract_evidence": {"exact_cases": [{"load_case_id": "case:a"}, {"load_case_id": "case:b"}]},
    }).to_string();
    let job = retained(&registry, &gate, &source);
    for raw in [false, true] {
        let one = json!({"jobId": job, "offset": 0, "limit": 1, "raw": raw});
        let bytes = call(&registry, RECOVERY_COMMAND, run_recovery, &one).unwrap();
        let max = bytes.len() as u64;
        let exact = call(&registry, RECOVERY_COMMAND, |r, b, l| run_recovery_within(r, b, l, max), &one).unwrap();
        assert_eq!(exact, bytes);
        let message = command_text(call(&registry, RECOVERY_COMMAND, |r, b, l| run_recovery_within(r, b, l, max - 1), &one).unwrap_err());
        assert!(message.starts_with(RESPONSE_LIMIT), "{message}");
        let mut got = Vec::new();
        let mut offset = 0;
        while offset < 2 {
            let request = json!({"jobId": job, "offset": offset, "limit": 2, "raw": raw});
            let bytes = call(&registry, RECOVERY_COMMAND, |r, b, l| run_recovery_within(r, b, l, max), &request).unwrap();
            assert!(bytes.len() as u64 <= max);
            let page = decode(&bytes);
            assert_eq!(page.header["returned"], 1);
            assert_eq!(page.header["truncated"], offset == 0);
            got.push(page.header["cases"][0]["ref_id"].clone());
            if raw { assert_eq!(page.texts.len(), 1); }
            offset += 1;
        }
        assert_eq!(got, [json!("case:a"), json!("case:b")]);
    }
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

// ---------------------------------------------------------------------------
// Committed outcomes (the TypeScript filters' parity, resultDetailParity.test.ts)
// ---------------------------------------------------------------------------

/// A case's text: a file's own text, a member of a file (by JSON pointer)
/// written by serde_json, or an inline text.
fn text_of(reference: &Value) -> String {
    if let Some(text) = reference["text"].as_str() {
        return text.to_string();
    }
    let path = reference["path"].as_str().expect("a path or a text");
    let pointer = reference["pointer"].as_str().unwrap_or("");
    let text = read_text(path);
    if pointer.is_empty() {
        return text;
    }
    let value: Value = serde_json::from_str(&text).unwrap();
    serde_json::to_string(value.pointer(pointer).unwrap_or_else(|| panic!("{path}#{pointer}"))).unwrap()
}

/// A page's outcome: its header without the job, and its rows' digest,
/// each row checked against the source's own token at its ordinal.
fn page_outcome(page: &Decoded, tokens: &[String]) -> Value {
    let ordinals: Vec<usize> = page.header["ordinals"].as_array().unwrap().iter().map(|o| o.as_u64().unwrap() as usize).collect();
    for (ordinal, text) in ordinals.iter().zip(&page.texts) {
        assert_eq!(text, &tokens[*ordinal], "row {ordinal}");
    }
    // Consecutive ordinals as runs `[first, last]`, so the committed file stays small.
    let mut runs: Vec<[usize; 2]> = Vec::new();
    for &ordinal in &ordinals {
        match runs.last_mut() {
            Some(run) if run[1] + 1 == ordinal => run[1] = ordinal,
            _ => runs.push([ordinal, ordinal]),
        }
    }
    json!({
        "total": page.header["total"], "returned": page.header["returned"], "truncated": page.header["truncated"],
        "ordinal_runs": runs, "rows_sha256": sha256(page.texts.concat().as_bytes()),
    })
}

/// `refused` for a command refusal, by its code.
fn refused(text: String) -> Value {
    json!({"refused": text.split(':').next().unwrap()})
}

#[test]
fn detail_queries_are_the_committed_outcomes() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let file: Value = serde_json::from_str(&read_text(CASES)).unwrap();
    let cases = file["cases"].as_array().unwrap();
    assert!(cases.len() >= 40, "{} cases", cases.len());
    let mut entries = Vec::new();
    for case in cases {
        let name = case["name"].as_str().unwrap();
        let source = text_of(&case["result"]);
        let job = retained(&registry, &gate, &source);
        let (_, tokens) = all_rows(&registry, &job, MAX_PAGE_ROWS);
        let queries: Vec<Value> = case["queries"].as_array().unwrap().iter().map(|query| {
            let args = json!({"jobId": job, "filter": query["filter"], "offset": query["offset"], "limit": query["limit"]});
            match rows(&registry, &args) {
                Ok(page) => page_outcome(&page, &tokens),
                Err(error) => refused(command_text(error)),
            }
        }).collect();
        let lookups: Vec<Value> = case["lookups"].as_array().unwrap().iter().map(|request| {
            let mut args = request.clone();
            args["jobId"] = json!(job);
            match lookup(&registry, &args) {
                Ok(found) => {
                    let mut header = found.header.clone();
                    header.as_object_mut().unwrap().remove("job_id");
                    // Each found row is the source's token at its ordinal.
                    for group in ["ordinals", "ids", "kinds"] {
                        for entry in header[group].as_array().unwrap() {
                            let row = if group == "ordinals" { &entry["row"] } else { &entry["row"] };
                            if let Some(ordinal) = row["ordinal"].as_u64() {
                                assert_eq!(found.texts[row["text"].as_u64().unwrap() as usize], tokens[ordinal as usize], "{name} lookup");
                            }
                        }
                    }
                    header
                }
                Err(error) => refused(command_text(error)),
            }
        }).collect();
        let recoveries: Vec<Value> = case["recoveries"].as_array().unwrap().iter().map(|request| {
            let mut args = request.clone();
            args["jobId"] = json!(job);
            match recovery(&registry, &args) {
                Ok(found) => {
                    let mut header = found.header.clone();
                    header.as_object_mut().unwrap().remove("job_id");
                    header["texts_sha256"] = json!(found.texts.iter().map(|t| sha256(t.as_bytes())).collect::<Vec<_>>());
                    check_case_texts(&source, &found);
                    header
                }
                Err(error) => refused(command_text(error)),
            }
        }).collect();
        entries.push(json!({"name": name, "queries": queries, "lookups": lookups, "recoveries": recoveries}));
        registry.dispose_result(&job).unwrap();
    }
    assert_eq!(registry.pool().reserved_total(), 0);
    // One case per line.
    let text = format!(
        "{{\n\"document_kind\": \"swbpipe.test.result_detail_outcomes\",\n\"note\": \"Written by result_detail_tests.rs (RESULT_DETAIL_OUTCOMES_WRITE=1) from result_detail_cases.json; read by resultDetailParity.test.ts.\",\n\"cases\": [\n{}\n]\n}}\n",
        entries.iter().map(|entry| serde_json::to_string(entry).unwrap()).collect::<Vec<_>>().join(",\n")
    );
    if std::env::var_os("RESULT_DETAIL_OUTCOMES_WRITE").is_some() {
        std::fs::write(root().join(OUTCOMES), &text).unwrap();
    }
    assert!(read_text(OUTCOMES) == text, "the outcomes differ from {OUTCOMES} (RESULT_DETAIL_OUTCOMES_WRITE=1 rewrites it)");
}

/// Each recorded case text returned is the exact token of `body.cases` at
/// its entry, as serde_json's raw members read it.
fn check_case_texts(source: &str, found: &Decoded) {
    if found.texts.is_empty() {
        return;
    }
    let body = member_text(&member_text(source, "source_block_recovery").unwrap(), "body").unwrap();
    let recorded: Vec<Box<RawValue>> = serde_json::from_str(&member_text(&body, "cases").unwrap()).unwrap();
    for case in found.header["cases"].as_array().unwrap() {
        let text = &found.texts[case["text"].as_u64().unwrap() as usize];
        assert_eq!(text, recorded[case["entry"].as_u64().unwrap() as usize].get());
    }
}

// ---------------------------------------------------------------------------
// Semantics
// ---------------------------------------------------------------------------

/// Every row of each file-backed corpus statement, in batches through the
/// command, gives the committed reader outcomes' digest.
#[test]
fn batched_semantics_are_the_committed_reader_outcomes() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let outcomes = committed();
    let mut checked = 0;
    for input in file_backed_corpus() {
        let Some(pinned) = outcomes[&input.id].get("rows") else { continue };
        let job = retained(&registry, &gate, &input.source);
        let count = serde_json::from_str::<Value>(&input.source).unwrap()["results"].as_array().map_or(0, Vec::len);
        let mut texts = Vec::new();
        for start in (0..count).step_by(MAX_KEYS) {
            let ordinals: Vec<usize> = (start..count.min(start + MAX_KEYS)).collect();
            let text = semantics(&registry, &json!({"jobId": job, "ordinals": ordinals})).unwrap();
            texts.push(text.strip_prefix('[').unwrap().strip_suffix(']').unwrap().to_string());
        }
        let all = format!("[{}]", texts.into_iter().filter(|t| !t.is_empty()).collect::<Vec<_>>().join(","));
        assert_eq!(sha256(all.as_bytes()), pinned["sha256"].as_str().unwrap(), "{}", input.id);
        // A batch in another order reads each row alike.
        if count > 2 {
            let one = semantics(&registry, &json!({"jobId": job, "ordinals": [count - 1, 0, count - 1]})).unwrap();
            let one: Value = serde_json::from_str(&one).unwrap();
            let whole: Value = serde_json::from_str(&all).unwrap();
            assert_eq!(one, json!([whole[count - 1], whole[0], whole[count - 1]]));
        }
        registry.dispose_result(&job).unwrap();
        checked += 1;
    }
    assert!(checked >= 40, "{checked} statements");
    // Out-of-range, empty and oversized batches are refused by name.
    let job = retained(&registry, &gate, r#"{"results":[{"kind":"x"}]}"#);
    for ordinals in [json!([1]), json!([]), json!((0..=MAX_KEYS).map(|_| 0).collect::<Vec<_>>())] {
        let message = command_text(semantics(&registry, &json!({"jobId": job, "ordinals": ordinals})).unwrap_err());
        assert!(message.starts_with(REQUEST_INVALID), "{message}");
    }
    // A source the dispatch refuses reads each row as the reader's refusal.
    let refused: Value = serde_json::from_str(&semantics(&registry, &json!({"jobId": job, "ordinals": [0]})).unwrap()).unwrap();
    assert_eq!(refused, rv::row_semantics_each(&[json!({"kind": "x"})], Some(&json!({"results": [{"kind": "x"}]}))));
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

// ---------------------------------------------------------------------------
// Filters the TypeScript would throw on
// ---------------------------------------------------------------------------

#[test]
fn members_the_typescript_filter_throws_on_are_refused_by_name() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let job = retained(&registry, &gate, r#"{"results":[{"id":"a","kind":"k"},null,{"id":"b","source_result_refs":7}]}"#);
    let page = |filter: Value| rows(&registry, &json!({"jobId": job, "filter": filter, "offset": 0, "limit": 10}));
    for filter in [json!({"text": "a"}), json!({"kinds": ["k"]}), json!({"frame": "global"}), json!({"case": "x:y"}), json!({"entities": ["e"]}), json!({"ids": ["a"]})] {
        let message = command_text(page(filter.clone()).unwrap_err());
        assert!(message.starts_with(&format!("{FILTER_TYPE_ERROR}: row 1")), "{filter}: {message}");
    }
    // The family filter reads no member of the row; a text query that never reaches row 2's refs is fine.
    assert_eq!(page(json!({"family": "other"})).unwrap().header["total"], 3);
    let job2 = retained(&registry, &gate, r#"{"results":[{"id":"b","source_result_refs":7},{"id":"c","source_result_refs":"xyz"}]}"#);
    let message = command_text(rows(&registry, &json!({"jobId": job2, "filter": {"text": "b"}, "offset": 0, "limit": 10})).unwrap_err());
    assert!(message.starts_with(&format!("{FILTER_TYPE_ERROR}: row 0: source_result_refs is not iterable")), "{message}");
    // A string spreads into its characters: "c x y z".
    let job3 = retained(&registry, &gate, r#"{"results":[{"id":"c","source_result_refs":"xyz"}]}"#);
    assert_eq!(rows(&registry, &json!({"jobId": job3, "filter": {"text": " X Y "}, "offset": 0, "limit": 10})).unwrap().header["total"], 1);
    assert_eq!(rows(&registry, &json!({"jobId": job3, "filter": {"text": "xy"}, "offset": 0, "limit": 10})).unwrap().header["total"], 0);
    for job in [job, job2, job3] {
        registry.dispose_result(&job).unwrap();
    }
    assert_eq!(registry.pool().reserved_total(), 0);
}

// ---------------------------------------------------------------------------
// The index: admission, lifetime, concurrency
// ---------------------------------------------------------------------------

/// The index is admitted before it is allocated: refused by name when its
/// lease does not fit, allocating nothing of its size, leaving no index; it
/// is then built once, held while the result is, and released with the bytes
/// by a disposal or the take.
#[test]
fn the_index_is_admitted_before_it_is_allocated_and_lives_with_the_bytes() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    // An invented rack of many rows, so the index's allocation is measurable.
    let rows_text: Vec<String> = (0..20_000).map(|i| format!(r#"{{"id":"r{i}","kind":"k","value":{i}}}"#)).collect();
    let text = format!(r#"{{"results":[{}]}}"#, rows_text.join(","));
    let job = retained(&registry, &gate, &text);
    let result_lease = registry.pool().reserved_total();
    let held = registry.held_result(&job).unwrap();
    let index_bytes = index_lease_bytes(20_000);
    // The headroom covers the scan but not the index with it: refused before the spans allocate.
    let (_, scan_bytes) = ingress::prescan_allowances(text.as_bytes()).unwrap();
    host.set(|s| s.headroom_bytes = Some(result_lease + scan_bytes + index_bytes - 1));
    let (refused, measured) = ingress::counting::measure(|| spans(&registry, held.text(), scan_bytes));
    match refusal(refused.unwrap_err()) {
        ResourceRefusal::HeadroomInsufficient { required_bytes, .. } => assert_eq!(required_bytes, result_lease + scan_bytes + index_bytes),
        other => panic!("{other:?}"),
    }
    assert!(measured.peak_priced < index_bytes / 4, "{measured:?} against {index_bytes}");
    assert_eq!(registry.pool().reserved_total(), result_lease);
    // Admitted, the spans' allocation is the index lease's.
    host.set(|s| s.headroom_bytes = Some(result_lease + scan_bytes + index_bytes));
    let ((spans_built, _, lease), measured) = ingress::counting::measure(|| spans(&registry, held.text(), scan_bytes).unwrap());
    assert_eq!((spans_built.len(), lease.bytes()), (20_000, index_bytes));
    assert!(measured.peak_priced <= index_bytes + scan_bytes, "{measured:?} against {index_bytes} + {scan_bytes}");
    drop((spans_built, lease));
    // The whole build: refused under a headroom that does not cover the index, leaving no index.
    host.set(|s| s.headroom_bytes = Some(result_lease + scan_bytes + index_bytes - 1));
    assert!(index(&registry, &held).is_err());
    assert!(held.detail_slot().lock().unwrap().is_none());
    assert_eq!(registry.pool().reserved_total(), result_lease);
    host.set(|s| s.headroom_bytes = Some(64 << 30));
    // Under pressure the adaptive index lease is refused too.
    host.set(|s| {
        s.headroom_bytes = Some(64 << 30);
        s.pressure = PressureLevel::Warning;
    });
    assert!(matches!(refusal(index(&registry, &held).unwrap_err()), ResourceRefusal::PressureElevated { .. }));
    host.set(|s| s.pressure = PressureLevel::Normal);
    let built = index(&registry, &held).unwrap();
    assert_eq!(built.len(), 20_000);
    assert_eq!(built.lease_bytes(), index_bytes);
    assert!(Arc::ptr_eq(&built, &index(&registry, &held).unwrap()), "built once");
    drop(built);
    drop(held);
    assert_eq!(registry.pool().reserved_total(), result_lease + index_bytes, "the index lives with the held result");
    // A disposal while a reader holds the result releases bytes and index with its share.
    let held = registry.held_result(&job).unwrap();
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), result_lease + index_bytes);
    drop(held);
    assert_eq!(registry.pool().reserved_total(), 0);
    // The take releases the index; the custody lease remains until disposal.
    let job = retained(&registry, &gate, &text);
    rows(&registry, &json!({"jobId": job, "offset": 0, "limit": 1})).unwrap();
    assert_eq!(registry.pool().reserved_total(), result_lease + index_bytes);
    assert_eq!(registry.take_result(&job).unwrap(), text.as_bytes());
    assert_eq!(registry.pool().reserved_total(), 2 * result_lease);
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// A query's own leases (args, working, response) are released when it
/// returns; only the index's stays, with the result. A refused working lease
/// is typed and leaves the index.
#[test]
fn query_leases_end_with_the_query_and_are_refused_typed() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let text = read_text(DEMO);
    let job = retained(&registry, &gate, &text);
    let result_lease = registry.pool().reserved_total();
    rows(&registry, &json!({"jobId": job, "offset": 0, "limit": 5})).unwrap();
    let held = registry.pool().reserved_total();
    assert!(held > result_lease);
    for args in [
        json!({"jobId": job, "filter": {"text": "pipe"}, "offset": 0, "limit": 5}),
        json!({"jobId": job, "ordinals": [0], "ids": ["x"], "match": "last"}),
    ] {
        let _ = if args.get("match").is_some() { lookup(&registry, &args).map(drop) } else { rows(&registry, &args).map(drop) };
        assert_eq!(registry.pool().reserved_total(), held);
    }
    semantics(&registry, &json!({"jobId": job, "ordinals": [0, 1]})).unwrap();
    recovery(&registry, &json!({"jobId": job, "offset": 0, "limit": 5, "raw": true})).unwrap();
    assert_eq!(registry.pool().reserved_total(), held);
    host.set(|s| s.pressure = PressureLevel::Warning);
    assert!(matches!(refusal(rows(&registry, &json!({"jobId": job, "offset": 0, "limit": 5})).unwrap_err()), ResourceRefusal::PressureElevated { .. }));
    host.set(|s| s.pressure = PressureLevel::Normal);
    assert_eq!(registry.pool().reserved_total(), held);
    // Unknown, running and delivered jobs.
    assert!(invalid_message(rows(&registry, &json!({"jobId": "job-404", "offset": 0, "limit": 1})).unwrap_err()).starts_with("unknown solve job"));
    registry.take_result(&job).unwrap();
    let message = invalid_message(rows(&registry, &json!({"jobId": job, "offset": 0, "limit": 1})).unwrap_err());
    assert!(message.starts_with("PREVIEW-SOLVE-RESULT-ALREADY-DELIVERED"), "{message}");
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// Concurrent first queries build the index once: while the first builds,
/// the others wait; all then read the same rows. A take while they hold the
/// result is refused busy; a disposal during a read leaves the read whole.
#[test]
fn concurrent_first_queries_build_one_index() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let text = read_text(DEMO);
    let job = retained(&registry, &gate, &text);
    let building = gate::hold(&registry, "index", false);
    let threads: Vec<_> = (0..6)
        .map(|_| {
            let (registry, job) = (registry.clone(), job.clone());
            std::thread::spawn(move || rows(&registry, &json!({"jobId": job, "offset": 0, "limit": 3})).unwrap().texts)
        })
        .collect();
    assert!(building.0.wait_entered(1, Duration::from_secs(30)));
    std::thread::sleep(Duration::from_millis(50));
    assert_eq!(building.0.entered(), 1, "one build at a time");
    let message = match registry.take_result(&job).unwrap_err() {
        SolveJobError::InvalidRequest { message } => message,
        other => panic!("{other:?}"),
    };
    assert!(message.starts_with("PREVIEW-SOLVE-RESULT-BUSY"), "{message}");
    building.0.open();
    let pages: Vec<_> = threads.into_iter().map(|t| t.join().unwrap()).collect();
    assert!(pages.windows(2).all(|w| w[0] == w[1]));
    assert_eq!(building.0.entered(), 1, "built once");
    drop(building);
    // A disposal during a read: the read completes from its share.
    let reading = gate::hold(&registry, "read", false);
    let (r, j) = (registry.clone(), job.clone());
    let thread = std::thread::spawn(move || rows(&r, &json!({"jobId": j, "offset": 0, "limit": 3})).unwrap().texts);
    assert!(reading.0.wait_entered(1, Duration::from_secs(30)));
    registry.dispose_result(&job).unwrap();
    assert!(registry.pool().reserved_total() > 0);
    reading.0.open();
    assert_eq!(thread.join().unwrap(), pages[0]);
    drop(reading);
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// A panicked build leaves no index, and its leases are released; the next
/// query builds it.
#[test]
fn a_failed_build_leaves_no_index() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let job = retained(&registry, &gate, r#"{"results":[1,2]}"#);
    let result_lease = registry.pool().reserved_total();
    let held = registry.held_result(&job).unwrap();
    let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        PANIC_IN_BUILD.with(|p| p.set(true));
        let _ = index(&registry, &held);
    }));
    PANIC_IN_BUILD.with(|p| p.set(false));
    assert!(panicked.is_err());
    assert!(held.detail_slot().lock().unwrap_or_else(PoisonError::into_inner).is_none());
    assert_eq!(registry.pool().reserved_total(), result_lease);
    assert_eq!(index(&registry, &held).unwrap().len(), 2);
    drop(held);
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

thread_local! {
    pub(crate) static PANIC_IN_BUILD: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Admission, indexing and reads run in a blocking task, never on the async
/// runtime's workers: more commands than the runtime has workers are held
/// at admission and then in the read, and another async command completes.
#[test]
fn admission_and_reads_run_off_the_async_executor() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let text = read_text(DEMO);
    let job = retained(&registry, &gate, &text);
    let commands = runtime_workers() * 2 + 2;
    let args = json!({"jobId": job, "filter": {"text": "pipe"}, "offset": 0, "limit": 5});
    let expected = rows(&registry, &args).unwrap().texts;
    for stage in ["admit", "read"] {
        let held = gate::hold(&registry, stage, false);
        let handles: Vec<_> = (0..commands)
            .map(|_| {
                let (registry, body) = (registry.clone(), InvokeBody::Raw(serde_json::to_vec(&args).unwrap()));
                tauri::async_runtime::spawn(async move { query_rows(&registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1))).await })
            })
            .collect();
        assert!(held.0.wait_entered(commands, Duration::from_secs(60)), "{stage}: only {} of {commands} commands entered", held.0.entered());
        let (sender, receiver) = std::sync::mpsc::channel();
        let (probe_registry, probe_job) = (registry.clone(), job.clone());
        tauri::async_runtime::spawn(async move {
            let _ = sender.send(probe_registry.status(&probe_job).map(|status| status.state));
        });
        let probe = receiver.recv_timeout(Duration::from_secs(10));
        assert_eq!(probe.unwrap_or_else(|_| panic!("{stage}: the runtime ran no other task")).unwrap(), "completed");
        held.0.open();
        for handle in handles {
            assert_eq!(decode(&tauri::async_runtime::block_on(handle).unwrap().unwrap()).texts, expected, "{stage}");
        }
        drop(held);
    }
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// The executor's part neither prescans nor parses; a malformed body is
/// refused only by the blocking part.
#[test]
fn reserve_does_not_prescan_or_parse_the_body() {
    let host = ScriptedHost::normal();
    let (registry, _): (SolveJobRegistry, Arc<Gate>) = gated(&host);
    for command in [ROWS_COMMAND, LOOKUP_COMMAND, SEMANTICS_COMMAND, RECOVERY_COMMAND] {
        let body = InvokeBody::Raw(br#"{"jobId":"#.to_vec());
        let (raw, lease) = reserve(&registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1)), command).unwrap();
        assert!(registry.pool().reserved_total() > 0);
        let error = run_rows(registry.clone(), raw, lease).unwrap_err();
        assert!(matches!(refusal(error), ResourceRefusal::IngressInvalid { .. }));
        assert_eq!(registry.pool().reserved_total(), 0);
        match reserve(&registry, &body, &headers(None), command) {
            Err(CommandError::Ingress(SolveJobError::InvalidRequest { message })) => assert!(message.starts_with("INGRESS-HEADER-REQUIRED"), "{message}"),
            other => panic!("{other:?}"),
        }
    }
}
