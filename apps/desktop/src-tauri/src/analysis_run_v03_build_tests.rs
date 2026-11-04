//! C1 slice 5's builder: the native v0.3 analysis-run record and its preview
//! (`analysis_run_v03_build`).
//!
//! - **The TypeScript builder's records, byte for byte.** Every case of
//!   `apps/desktop/src/test-support/analysis-run-v03/analysis_run_v03_cases.json` (written by
//!   `analysisRunV03Cases.ts`, which the vitest leg
//!   `analysisRunV03Parity.test.ts` regenerates and compares) carries the
//!   unchanged TypeScript builder's outcome: its record as the JSON value
//!   `JSON.stringify` wrote, or its refusal. The record's stored form is
//!   `serde_json::to_string` of that value as serde_json parses it, which is
//!   exactly what `save_local_project` stores for the record the webview
//!   sends. The native bytes must equal it, and both checksums the record
//!   carries must be the native ones. A builder case runs the builder on its
//!   inputs; a preview case runs the command on a retained job with the
//!   session's manifest evidence and checks the header and the bounded bytes
//!   route.
//! - **The store.** The TypeScript record written through the store's own
//!   project writer reads back as the native bytes, and Record Run's recorded
//!   revision of each is the same text.
//! - **The writers.** The canonical writer gives the maintained canonical
//!   cases (`fixtures/canonical_hash/cases.json`: signed zero, non-ASCII keys,
//!   numeric boundaries) and `canonical_json` on the corpus; the stored writer
//!   writes JavaScript's numbers and serde_json's key order.
//! - **Bounded.** On a scaled invented rack the build allocates nothing that
//!   grows with the record but the leased fingerprints; the header does not
//!   grow with the rows; the bytes route refuses a record over its bound
//!   before allocating and allocates exactly the record at it.
//! - Admission, leases and lifetime, the take and disposal, concurrency, the
//!   executor, a panic, and Tauri's raw response body.
use super::*;
use crate::command_ingress::test_support::headers;
use crate::command_ingress::INGRESS_ARGS_JSON_V1;
use crate::memory_pool::test_host::ScriptedHost;
use crate::memory_pool::{PressureLevel, ResourceRefusal};
pub(crate) use crate::native_verdict::tests::gate;
use crate::native_verdict::tests::{gated, retained, runtime_workers};
use crate::solve_jobs::SolveJobError;
use open_pipe_stress_canonical_json::canonical_json;
use serde_json::{json, Value};
use std::time::Duration;

const CASES: &str = "apps/desktop/src/test-support/analysis-run-v03/analysis_run_v03_cases.json";

thread_local! {
    pub(crate) static PANIC_IN_BUILD: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

use super::old_reference;

/// RT-U4: test-only counts of the builder's work, per thread (each command
/// under test runs on the calling thread): calls of the route's full reader
/// (`validate_route`), row interpretations (`interpret`), values visited by
/// the checked-profile check (`has_outside_profile`, each node) and
/// canonical SHA-256 streams (`canonical_sha256`: the received result's and
/// each row's). The counters change nothing the builder computes.
pub(crate) mod passes {
    use std::cell::Cell;
    pub(crate) const READER_VALIDATIONS: usize = 0;
    pub(crate) const INTERPRETATIONS: usize = 1;
    pub(crate) const OUTSIDE_NODES: usize = 2;
    pub(crate) const CANONICAL_HASHES: usize = 3;
    thread_local! {
        static COUNTS: Cell<[u64; 4]> = const { Cell::new([0; 4]) };
    }
    pub(crate) fn bump(kind: usize) {
        COUNTS.with(|counts| {
            let mut now = counts.get();
            now[kind] += 1;
            counts.set(now);
        });
    }
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub(crate) struct Counts {
        pub(crate) reader_validations: u64,
        pub(crate) interpretations: u64,
        pub(crate) outside_nodes: u64,
        pub(crate) canonical_hashes: u64,
    }
    /// This thread's counts now, for [`since`].
    pub(crate) fn snapshot() -> [u64; 4] {
        COUNTS.with(Cell::get)
    }
    /// The counts added on this thread since `before`.
    pub(crate) fn since(before: [u64; 4]) -> Counts {
        let after = snapshot();
        let d = |kind: usize| after[kind] - before[kind];
        Counts { reader_validations: d(READER_VALIDATIONS), interpretations: d(INTERPRETATIONS), outside_nodes: d(OUTSIDE_NODES), canonical_hashes: d(CANONICAL_HASHES) }
    }
    /// `work`'s result and the counts it added on this thread.
    pub(crate) fn measure<T>(work: impl FnOnce() -> T) -> (T, Counts) {
        let before = snapshot();
        let out = work();
        (out, since(before))
    }
}

fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}
fn read_text(path: &str) -> String {
    std::fs::read_to_string(root().join(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}
fn cases() -> Vec<Value> {
    serde_json::from_str::<Value>(&read_text(CASES)).unwrap()["cases"].as_array().unwrap().clone()
}

/// A case's held text: a file, a file's top-level member (its exact bytes),
/// or a text.
fn source_text(source: &Value) -> String {
    if let Some(text) = source["text"].as_str() {
        return text.to_string();
    }
    let text = read_text(source["path"].as_str().unwrap());
    match source["member"].as_str() {
        None => text,
        Some(member) => crate::result_detail::last_members(&text, [member]).unwrap()[0].unwrap().get().to_string(),
    }
}

fn no_lease(_: u64) -> Result<Option<Lease>, CommandError<String>> {
    Ok(None)
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

/// The stored form of a record the webview sends: the store's text of the
/// value serde_json parses from the webview's `JSON.stringify`.
fn stored(record: &Value) -> String {
    serde_json::to_string(record).unwrap()
}

/// The record's checksum of `scope`.
fn claimed(record: &Value, scope: &str) -> String {
    record["analysis_run"]["hashes"].as_array().unwrap().iter().find(|h| h["payload_scope"] == scope).unwrap()["value"].as_str().unwrap().to_string()
}

/// A command's two steps in this thread: the executor's reservation, then the blocking part.
fn call<T>(registry: &SolveJobRegistry, run: impl FnOnce(SolveJobRegistry, Vec<u8>, Lease) -> Result<T, CommandError<String>>, args: &Value) -> Result<T, CommandError<String>> {
    let body = InvokeBody::Raw(serde_json::to_vec(args).unwrap());
    let (raw, lease) = reserve(registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1)))?;
    run(registry.clone(), raw, lease)
}
fn preview_header(registry: &SolveJobRegistry, args: &Value) -> Result<Value, CommandError<String>> {
    call(registry, super::run, args).map(|bytes| serde_json::from_slice(&bytes).expect("the header is JSON"))
}
fn preview_bytes(registry: &SolveJobRegistry, args: &Value, max: u64) -> Result<(Vec<u8>, Lease), CommandError<String>> {
    call(registry, |r, raw, lease| run_record_bytes_within(r, raw, lease, max), args)
}

fn preview_args(job: &str, case: &Value) -> Value {
    json!({
        "jobId": job,
        "inputManifest": case["input_manifest"],
        "ruleCheckAggregate": case["rule_check_aggregate"],
        "rulePackExclusions": case.get("rule_pack_exclusions").cloned().unwrap_or(json!([])),
    })
}

/// The members the header leaves out (counted or named instead).
const ROW_SCALE: [&str; 5] = ["diagnostics", "result_refs", "contract_evidence", "source_block_recovery", "retained_precision"];

fn header_record(record: &Value) -> Value {
    let mut header = record.clone();
    let run = header["analysis_run"].as_object_mut().unwrap();
    for member in ROW_SCALE {
        run.remove(member);
    }
    header
}

/// One builder case's native outcome on its inputs.
fn build_case(case: &Value) -> Result<(String, String, String), String> {
    let mut source: Value = serde_json::from_str(&source_text(&case["source"])).map_err(|_| SOURCE_CONTRACT_UNSUPPORTED.to_string())?;
    let exclusions: Vec<Exclusion> = serde_json::from_value(case.get("rule_pack_exclusions").cloned().unwrap_or(json!([]))).unwrap();
    let manifest = &case["manifest"];
    let inputs = BuildInputs {
        manifest: ManifestEvidence { manifest_ref: &manifest["manifest_ref"], manifest_sha256: &manifest["manifest_sha256"], manifest: &manifest["manifest"] },
        rule_check_status: case.get("rule_check_status"),
        load_basis_refs: case.get("load_basis_refs"),
        rule_pack_exclusions: &exclusions,
    };
    let built = build_v03(&mut source, &inputs, &mut no_lease).map_err(command_text)?;
    let (bytes, ()) = record_bytes(&built, u64::MAX, |_| Ok(())).unwrap();
    Ok((String::from_utf8(bytes).unwrap(), built.record_sha256.clone(), built.received_sha256.clone()))
}

/// The native outcome against the TypeScript's: the record's stored bytes
/// and checksums, or the refusal (a TypeError as the shape refusal).
fn check_outcome(id: &str, expected: &Value, got: Result<(String, String, String), String>, misses: &mut Vec<String>) {
    match (expected.get("record"), got) {
        (Some(record), Ok((bytes, record_sha256, received_sha256))) => {
            if bytes != stored(record) {
                let at = bytes.bytes().zip(stored(record).bytes()).position(|(a, b)| a != b).unwrap_or(bytes.len().min(stored(record).len()));
                misses.push(format!("{id}: bytes differ at {at}: native …{}… ts …{}…", &bytes[at.saturating_sub(80)..(at + 80).min(bytes.len())], &stored(record)[at.saturating_sub(80)..(at + 80).min(stored(record).len())]));
            } else if record_sha256 != claimed(record, "analysis_run_record") || received_sha256 != claimed(record, "received_result") {
                misses.push(format!("{id}: checksums differ"));
            }
        }
        (None, Err(text)) => {
            let ok = match (expected.get("error").and_then(Value::as_str), expected.get("type_error")) {
                (Some(error), _) => text == error,
                (None, Some(_)) => text.starts_with(INPUT_SHAPE_INVALID),
                _ => false,
            };
            if !ok {
                misses.push(format!("{id}: refused `{text}`, expected {expected}"));
            }
        }
        (Some(_), Err(text)) => misses.push(format!("{id}: refused `{text}`, expected a record")),
        (None, Ok(_)) => misses.push(format!("{id}: built a record, expected {expected}")),
    }
}

/// Every committed case: the native outcome is the TypeScript builder's, the
/// record byte for byte in its stored form, through the builder (builder
/// cases) or the command and the bounded bytes route on a retained job
/// (preview cases).
#[test]
fn committed_cases_are_the_typescript_builders_records_byte_for_byte() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let all = cases();
    assert!(all.len() >= 40, "{} cases", all.len());
    let mut misses = Vec::new();
    let (mut records, mut refusals) = (0, 0);
    for case in &all {
        let id = case["id"].as_str().unwrap();
        let expected = &case["expected"];
        if expected.get("record").is_some() {
            records += 1;
        } else {
            refusals += 1;
        }
        match case["kind"].as_str().unwrap() {
            "builder" => check_outcome(id, expected, build_case(case), &mut misses),
            "preview" => {
                let job = retained(&registry, &gate, &source_text(&case["source"]));
                let args = preview_args(&job, case);
                let header = preview_header(&registry, &args);
                let bytes = preview_bytes(&registry, &args, u64::MAX);
                match (&header, &bytes) {
                    (Ok(header), Ok((bytes, _))) => {
                        let record: Value = serde_json::from_slice(bytes).unwrap();
                        let got = Ok((String::from_utf8(bytes.clone()).unwrap(), claimed(&record, "analysis_run_record"), claimed(&record, "received_result")));
                        check_outcome(id, expected, got, &mut misses);
                        if let Some(expected) = expected.get("record") {
                            if header["record"] != header_record(expected) || header["record_bytes"] != json!(stored(expected).len()) {
                                misses.push(format!("{id}: the header is not the record's"));
                            }
                            let run = &expected["analysis_run"];
                            let producer = run["diagnostics"].as_array().unwrap().len() - header["rule_pack_ref_diagnostics"].as_array().unwrap().len();
                            if header["result_ref_count"] != json!(run["result_refs"].as_array().unwrap().len())
                                || header["producer_diagnostic_count"] != json!(producer)
                                || header["rule_pack_ref_diagnostics"].as_array().unwrap()[..] != run["diagnostics"].as_array().unwrap()[producer..]
                            {
                                misses.push(format!("{id}: the header's counts are not the record's"));
                            }
                        }
                    }
                    (Err(_), Err(_)) => {
                        let (Err(header), Err(_)) = (header, bytes) else { unreachable!() };
                        check_outcome(id, expected, Err(command_text(header)), &mut misses);
                    }
                    _ => misses.push(format!("{id}: the header and the bytes route disagree")),
                }
                registry.dispose_result(&job).unwrap();
            }
            other => panic!("{other}"),
        }
    }
    assert!(misses.is_empty(), "{} of {} cases differ:\n{}", misses.len(), all.len(), misses.join("\n"));
    assert!(records >= 25 && refusals >= 15, "{records} records, {refusals} refusals");
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// The store's own project writer stores the TypeScript record of a preview
/// case as the native bytes, and Record Run's recorded revision of the two
/// is the same text.
#[test]
fn the_store_writes_the_typescript_record_as_the_native_bytes() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let preview = cases().into_iter().filter(|case| case["kind"] == "preview" && case["expected"].get("record").is_some()).collect::<Vec<_>>();
    assert!(preview.len() >= 3);
    let dir = std::env::temp_dir().join(format!("swbpipe-arv03-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    std::fs::create_dir_all(&dir).unwrap();
    for case in &preview {
        let expected = &case["expected"]["record"];
        let job = retained(&registry, &gate, &source_text(&case["source"]));
        let (native, lease) = preview_bytes(&registry, &preview_args(&job, case), u64::MAX).unwrap();
        let path = dir.join(format!("{}.sqlite", case["id"].as_str().unwrap().replace(|c: char| !c.is_ascii_alphanumeric(), "-")));
        let (mut connection, _) = crate::open_project_store(&path).unwrap();
        crate::upsert_project(&mut connection, "invented-project", "Invented project", &json!({}), &json!([]), &Value::Null, &Value::Null, &Value::Null, expected, &Value::Null, &json!({"value": "base"})).unwrap();
        let column: String = connection.query_row("SELECT analysis_run_json FROM local_projects WHERE project_id = 'invented-project'", [], |row| row.get(0)).unwrap();
        assert_eq!(column.as_bytes(), native.as_slice(), "{}", case["id"]);
        let manifest: Value = serde_json::from_str(case["input_manifest"]["canonical_bytes"].as_str().unwrap()).unwrap();
        let memory = crate::stored_read::UnmeteredReadMemory;
        let revision = |record: &Value| {
            let (revision, hash) = crate::analysis_runs::recorded_revision(&memory, record, "state:invented", "2026-10-31T00:00:00.000Z", &manifest).unwrap();
            (serde_json::to_string(&revision).unwrap(), hash)
        };
        assert_eq!(revision(expected), revision(&serde_json::from_slice(&native).unwrap()), "{}", case["id"]);
        drop((native, lease));
        registry.dispose_result(&job).unwrap();
    }
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// The canonical writer is `canonical_json`: the maintained canonical cases
/// (signed zero, non-ASCII and supplementary-plane keys, numeric boundaries)
/// and every committed case's source and manifest.
#[test]
fn the_canonical_writer_is_canonical_json() {
    let pinned: Value = serde_json::from_str(&read_text("fixtures/canonical_hash/cases.json")).unwrap();
    for case in pinned["cases"].as_array().unwrap() {
        let value: Value = serde_json::from_str(case["input_json"].as_str().unwrap()).unwrap();
        let mut text = Vec::new();
        Writer::new(&mut text, Style::Canonical).value(&value);
        assert_eq!(std::str::from_utf8(&text).unwrap(), case["expected_canonical"].as_str().unwrap(), "{}", case["case_id"]);
        assert_eq!(canonical_sha256(&value), case["expected_sha256"].as_str().unwrap(), "{}", case["case_id"]);
    }
    for case in cases() {
        // A held text serde_json cannot read (a lone surrogate) is refused before any writer.
        let Ok(source) = serde_json::from_str::<Value>(&source_text(&case["source"])) else { continue };
        let mut values = vec![source];
        values.extend(case.get("manifest").cloned());
        for value in values {
            let mut text = Vec::new();
            Writer::new(&mut text, Style::Canonical).value(&value);
            assert_eq!(String::from_utf8(text).unwrap(), canonical_json(&value), "{}", case["id"]);
        }
    }
}

/// The stored writer writes each number as JavaScript's `JSON.stringify`
/// writes its binary64 value, read back by serde_json, and each object in
/// serde_json's key order (UTF-8 bytes, where the canonical writer orders by
/// UTF-16 code units).
#[test]
fn the_stored_writer_writes_javascripts_numbers_in_serde_order() {
    let stored_text = |text: &str| {
        let value: Value = serde_json::from_str(text).unwrap();
        let mut out = Vec::new();
        Writer::new(&mut out, Style::Stored).value(&value);
        String::from_utf8(out).unwrap()
    };
    // Each held number, and the text JavaScript's `JSON.stringify` writes for its binary64 value;
    // the store's text is serde_json's of the value it parses from that.
    for (held, javascript) in [
        ("-0.0", "0"),
        ("-0", "0"),
        ("0.0", "0"),
        ("1.0", "1"),
        ("-1.0", "-1"),
        ("1.5", "1.5"),
        ("1e-7", "1e-7"),
        ("0.000001", "0.000001"),
        ("1E2", "100"),
        ("9007199254740991", "9007199254740991"),
        ("9007199254740991.0", "9007199254740991"),
        ("100000000000000000000", "100000000000000000000"),
        ("1e21", "1e+21"),
        ("12345678901234567890", "12345678901234567000"),
        ("1.7976931348623157e308", "1.7976931348623157e+308"),
        ("5e-324", "5e-324"),
    ] {
        let store = serde_json::to_string(&serde_json::from_str::<Value>(javascript).unwrap()).unwrap();
        assert_eq!(stored_text(held), store, "{held}");
    }
    let keys = "{\"z\":1,\"\u{e9}\":2,\"\u{1d11e}\":3,\"\\ue000\":4,\"a\":{\"b\":-0.0}}";
    // serde_json's order is UTF-8 byte order (U+E000 before U+1D11E); RFC 8785's is UTF-16
    // code-unit order (the surrogate pair of U+1D11E before U+E000).
    assert_eq!(stored_text(keys), "{\"a\":{\"b\":0},\"z\":1,\"\u{e9}\":2,\"\u{e000}\":4,\"\u{1d11e}\":3}");
    let mut canonical = Vec::new();
    Writer::new(&mut canonical, Style::Canonical).value(&serde_json::from_str(keys).unwrap());
    assert_eq!(String::from_utf8(canonical).unwrap(), "{\"a\":{\"b\":0},\"z\":1,\"\u{e9}\":2,\"\u{1d11e}\":3,\"\u{e000}\":4}");
    // The retained reader's input keeps a negative zero and JavaScript's other numbers.
    let mut value: Value = serde_json::from_str(r#"[-0.0, 1.0, 2.5, 100000000000000000000]"#).unwrap();
    keep_negative_zero_js_numbers(&mut value);
    let javascript: Value = serde_json::from_str("[-0, 1, 2.5, 100000000000000000000]").unwrap();
    assert_eq!(serde_json::to_string(&value).unwrap(), serde_json::to_string(&javascript).unwrap());
    assert!(value[0].as_f64().unwrap().is_sign_negative());
}

#[test]
fn javascripts_readings_of_json_values() {
    let v = |text: &str| serde_json::from_str::<Value>(text).unwrap();
    for (value, text) in [("null", "null"), ("true", "true"), ("-0.0", "0"), ("1e21", "1e+21"), ("[1,null,[2,3],\"a\"]", "1,,2,3,a"), ("{}", "[object Object]"), ("\"x\"", "x")] {
        assert_eq!(js_string(Some(&v(value))), text, "{value}");
    }
    assert_eq!(js_string(None), "undefined");
    assert!(js_blank(" \t\u{feff}\u{2028}\u{3000}") && !js_blank(" x") && js_blank(""));
    assert!(strict_eq(Some(&v("0.0")), Some(&v("-0"))) && !strict_eq(Some(&v("{}")), Some(&v("{}"))) && strict_eq(None, None));
    assert!(!truthy(Some(&v("-0.0"))) && truthy(Some(&v("[]"))) && !truthy(Some(&v("\"\""))));
    assert!(outside_profile(&serde_json::from_str("9007199254740992").unwrap()) && !outside_profile(&serde_json::from_str("9007199254740991").unwrap()));
    assert!(outside_profile(&serde_json::from_str("1e300").unwrap()) && !outside_profile(&serde_json::from_str("1.5").unwrap()));
}

/// An invented rack of `rows` preview-physics rows, read under a route
/// constructed directly (the readers' evidence checks are not the subject).
fn rack(rows: usize) -> (Value, Route) {
    let row = |i: usize| {
        json!({"id": format!("result:r{i}"), "entity_ref": format!("node:n{i}"), "kind": "global_nodal_rotation_x", "value": i as f64 + 0.5, "unit": "rad",
            "basis_ref": {"ref_type": "load_case", "ref_id": "case:a"},
            "metadata": {"component": "nodal_rotation_x", "coordinate_system": "global", "location": "node", "basis": "solved_from_global_linear_system",
                "sign_convention": "positive value follows the right-hand-rule rotation about the global cartesian X axis"}})
    };
    let source = json!({
        "schema_version": "0.2.0", "document_kind": "MechanicsResult", "run_id": "run:rack", "model_ref": "model:rack",
        "producer": {"component_name": "invented", "component_version": "0.0.0"},
        "status": {"mechanics": "MECHANICS_SOLVED", "rule_check": "RULE_INPUTS_INCOMPLETE"},
        "diagnostics": [], "results": (0..rows).map(row).collect::<Vec<_>>(),
    });
    let table = sc::preview_physics_contract();
    let route = Route { name: "preview_physics", binding_id: table["semantic_contract_id"].as_str().unwrap().into(), binding_sha256: "0".repeat(64), table: Some(table) };
    (source, route)
}

fn rack_inputs<'a>(manifest: &'a Value, load_basis: &'a Value) -> BuildInputs<'a> {
    BuildInputs {
        manifest: ManifestEvidence { manifest_ref: &manifest["manifest_ref"], manifest_sha256: &manifest["manifest_sha256"], manifest: &manifest["manifest"] },
        rule_check_status: None,
        load_basis_refs: Some(load_basis),
        rule_pack_exclusions: &[],
    }
}

/// On a scaled invented rack: the build allocates nothing that grows with
/// the record but the fingerprints, leased at their exact size; the header
/// does not grow with the rows; the bytes route refuses a record one byte
/// over its bound with nothing of its size allocated, and at the bound
/// allocates the record exactly.
#[test]
fn the_preview_is_bounded_and_the_bytes_route_is_admitted_at_its_exact_length() {
    let manifest = json!({"manifest_ref": {"object_type": "InputManifest", "ref": "input-manifest:rack"}, "manifest_sha256": "1".repeat(64),
        "manifest": {"model_basis": {"model_ref": "model:rack"}, "solver_basis": {"solver_name": "invented", "solver_version": "0.0.0", "solver_build_ref": "build:rack"}}});
    let load_basis = json!([{"object_type": "LoadCase", "ref": "case:a"}]);
    let mut headers = Vec::new();
    for rows in [2_000usize, 20_000] {
        let (source, route) = rack(rows);
        let mut leased = Vec::new();
        let (built, measured) = ingress::counting::measure(|| {
            build(&source, route.clone(), &rack_inputs(&manifest, &load_basis), &mut |bytes| {
                leased.push(bytes);
                Ok(None)
            })
            .unwrap()
        });
        let fingerprints = ingress::allocation_envelope((rows * 16) as u64).unwrap();
        assert_eq!(leased, vec![fingerprints]);
        assert!(built.record_bytes > (rows as u64) * 900, "{} bytes for {rows} rows", built.record_bytes);
        assert!(measured.peak_priced < fingerprints + (1 << 20), "{measured:?}: the build holds more than the fingerprints for {rows} rows");
        let ((header, ()), measured) = ingress::counting::measure(|| header_bytes(&built, "job-rack", |_| Ok(())).unwrap());
        assert!(measured.peak_priced <= ingress::allocation_envelope(header.len() as u64).unwrap() + (64 << 10), "{measured:?}");
        headers.push(header.len());
        // Over the bound: refused by name before anything of the record's size.
        let (refused, measured) = ingress::counting::measure(|| record_bytes(&built, built.record_bytes - 1, |_| -> Result<(), CommandError<String>> { panic!("admitted over the bound") }));
        let text = command_text(refused.unwrap_err());
        assert!(text.starts_with(RECORD_TOO_LARGE) && text.contains(&built.record_bytes.to_string()), "{text}");
        assert!(measured.peak_priced < 64 << 10, "{measured:?}");
        // At the bound: the reservation is the record's envelope, the buffer its exact length.
        let mut asked = 0;
        let ((bytes, ()), measured) = ingress::counting::measure(|| record_bytes(&built, built.record_bytes, |n| { asked = n; Ok(()) }).unwrap());
        assert_eq!(asked, ingress::allocation_envelope(built.record_bytes).unwrap());
        assert_eq!(bytes.len() as u64, built.record_bytes);
        assert!(measured.peak_priced < asked + (1 << 20), "{measured:?}");
        let record: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(record["analysis_run"]["result_refs"].as_array().unwrap().len(), rows);
        assert_eq!(serde_json::to_string(&record).unwrap().as_bytes(), bytes.as_slice(), "the bytes are serde_json's own text of their value");
    }
    // Only the counts' digits differ.
    assert!(headers[1] - headers[0] <= 4, "the header grows with the rows: {headers:?}");
}

/// The post-reader builder must also refuse a row annotation that becomes
/// too deep once nested in the record, as the TypeScript checked hash does.
/// Current producers use scalar kind/unit fields; construct this defensive
/// state directly rather than relying on the readers to admit malformed rows.
#[test]
fn every_row_annotation_is_checked_at_its_record_depth() {
    let manifest = json!({"manifest_ref": {"object_type": "InputManifest", "ref": "input-manifest:rack"}, "manifest_sha256": "1".repeat(64),
        "manifest": {"model_basis": {"model_ref": "model:rack"}, "solver_basis": {"solver_name": "invented", "solver_version": "0.0.0", "solver_build_ref": "build:rack"}}});
    let load_basis = json!([{"object_type": "LoadCase", "ref": "case:a"}]);
    for member in ["kind", "unit", "metadata"] {
        let (mut source, route) = rack(1);
        source["results"][0]["kind"] = json!("unknown");
        let mut nested = Value::Null;
        for _ in 0..123 {
            nested = json!([nested]);
        }
        source["results"][0][member] = nested;
        let error = build(&source, route, &rack_inputs(&manifest, &load_basis), &mut no_lease).err().unwrap();
        assert_eq!(command_text(error), RECURSION_LIMIT, "{member}");
    }
}

/// A refused fingerprint lease refuses the build, typed, after every
/// record-contract check.
#[test]
fn a_refused_fingerprint_lease_refuses_the_build() {
    let manifest = json!({"manifest_ref": {"object_type": "InputManifest", "ref": "input-manifest:rack"}, "manifest_sha256": "1".repeat(64),
        "manifest": {"model_basis": {"model_ref": "model:rack"}, "solver_basis": {"solver_name": "invented", "solver_version": "0.0.0", "solver_build_ref": "build:rack"}}});
    let load_basis = json!([{"object_type": "LoadCase", "ref": "case:a"}]);
    let (source, route) = rack(100);
    let error = build(&source, route, &rack_inputs(&manifest, &load_basis), &mut |_| Err(CommandError::Command("refused".into()))).err().unwrap();
    assert_eq!(command_text(error), "refused");
}

fn demo_case() -> Value {
    cases().into_iter().find(|case| case["kind"] == "preview" && case["expected"].get("record").is_some()).expect("a preview case with a record")
}

/// The command's leases end with it; the parse, working, fingerprint and
/// response leases are refused typed under pressure; unknown, taken and
/// delivered jobs are refused; the take is busy while a preview reads, and a
/// disposal during a read leaves it whole.
#[test]
fn leases_end_with_the_command_and_the_held_result_is_shared() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let case = demo_case();
    let text = source_text(&case["source"]);
    let job = retained(&registry, &gate, &text);
    let result_lease = registry.pool().reserved_total();
    let args = preview_args(&job, &case);
    let header = preview_header(&registry, &args).unwrap();
    assert_eq!(registry.pool().reserved_total(), result_lease);
    let (bytes, lease) = preview_bytes(&registry, &args, u64::MAX).unwrap();
    assert_eq!(registry.pool().reserved_total(), result_lease + lease.bytes(), "the record lease lives with the bytes");
    assert_eq!(lease.bytes(), ingress::allocation_envelope(bytes.len() as u64).unwrap());
    drop((bytes, lease));
    assert_eq!(registry.pool().reserved_total(), result_lease);
    // Under the record's own length: refused by name, nothing kept.
    let record_bytes = header["record_bytes"].as_u64().unwrap();
    let text_refused = command_text(preview_bytes(&registry, &args, record_bytes - 1).unwrap_err());
    assert!(text_refused.starts_with(RECORD_TOO_LARGE), "{text_refused}");
    assert_eq!(registry.pool().reserved_total(), result_lease);
    host.set(|s| s.pressure = PressureLevel::Warning);
    assert!(matches!(refusal(preview_header(&registry, &args).unwrap_err()), ResourceRefusal::PressureElevated { .. }));
    host.set(|s| s.pressure = PressureLevel::Normal);
    assert_eq!(registry.pool().reserved_total(), result_lease);
    // The take is refused busy while a preview holds the result; a disposal during the read leaves it whole.
    let reading = gate::hold(&registry, "read", false);
    let (r, a) = (registry.clone(), args.clone());
    let thread = std::thread::spawn(move || preview_header(&r, &a).unwrap());
    assert!(reading.0.wait_entered(1, Duration::from_secs(30)));
    let message = match registry.take_result(&job).unwrap_err() {
        SolveJobError::InvalidRequest { message } => message,
        other => panic!("{other:?}"),
    };
    assert!(message.starts_with("PREVIEW-SOLVE-RESULT-BUSY"), "{message}");
    registry.dispose_result(&job).unwrap();
    assert!(registry.pool().reserved_total() > 0);
    reading.0.open();
    assert_eq!(thread.join().unwrap(), header);
    drop(reading);
    assert_eq!(registry.pool().reserved_total(), 0);
    invalid_message(preview_header(&registry, &args).unwrap_err());
    assert!(invalid_message(preview_header(&registry, &preview_args("job-404", &case)).unwrap_err()).starts_with("unknown solve job"));
    let job = retained(&registry, &gate, &text);
    registry.take_result(&job).unwrap();
    let message = invalid_message(preview_header(&registry, &preview_args(&job, &case)).unwrap_err());
    assert!(message.starts_with("PREVIEW-SOLVE-RESULT-ALREADY-DELIVERED"), "{message}");
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// The parse lease of M is refused before the source is parsed, under a
/// headroom that covers everything but it, and the arguments are refused
/// typed when they are malformed.
#[test]
fn the_parse_is_admitted_before_it_allocates() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let case = demo_case();
    let text = source_text(&case["source"]);
    let job = retained(&registry, &gate, &text);
    let result_lease = registry.pool().reserved_total();
    let (bound, _) = ingress::prescan_allowances(text.as_bytes()).unwrap();
    host.set(|s| s.headroom_bytes = Some(result_lease + bound.materialization_bytes / 2));
    let (refused, measured) = ingress::counting::measure(|| parse_source(&registry, &text));
    assert!(matches!(refusal(refused.unwrap_err()), ResourceRefusal::HeadroomInsufficient { .. }));
    assert!(measured.peak_priced < bound.materialization_bytes / 8, "{measured:?}");
    host.set(|s| s.headroom_bytes = Some(64 << 30));
    for args in [json!({"jobId": job}), json!({"jobId": job, "inputManifest": {"manifest_ref": {}, "manifest_sha256": 5, "canonical_bytes": "{}"}})] {
        let error = preview_header(&registry, &args).unwrap_err();
        assert!(matches!(error, CommandError::Ingress(_)), "{error:?}");
    }
    assert_eq!(registry.pool().reserved_total(), result_lease);
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// A panic inside the build releases every lease the command took.
#[test]
fn a_panicked_build_releases_its_leases() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let case = demo_case();
    let job = retained(&registry, &gate, &source_text(&case["source"]));
    let result_lease = registry.pool().reserved_total();
    let args = preview_args(&job, &case);
    let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        PANIC_IN_BUILD.with(|p| p.set(true));
        let _ = preview_header(&registry, &args);
    }));
    PANIC_IN_BUILD.with(|p| p.set(false));
    assert!(panicked.is_err());
    assert_eq!(registry.pool().reserved_total(), result_lease);
    assert!(preview_header(&registry, &args).is_ok());
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// Admission and the build run in a blocking task, never on the async
/// runtime's workers: more previews than the runtime has workers are held at
/// admission and then in the read, another async command completes, and all
/// return the same header.
#[test]
fn admission_and_the_build_run_off_the_async_executor() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let case = demo_case();
    let job = retained(&registry, &gate, &source_text(&case["source"]));
    let commands = runtime_workers() * 2 + 2;
    let args = preview_args(&job, &case);
    let expected = preview_header(&registry, &args).unwrap();
    for stage in ["admit", "read"] {
        let held = gate::hold(&registry, stage, false);
        let handles: Vec<_> = (0..commands)
            .map(|_| {
                let (registry, body) = (registry.clone(), InvokeBody::Raw(serde_json::to_vec(&args).unwrap()));
                tauri::async_runtime::spawn(async move { analysis_run_preview(&registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1))).await })
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
            let bytes = tauri::async_runtime::block_on(handle).unwrap().unwrap();
            assert_eq!(serde_json::from_slice::<Value>(&bytes).unwrap(), expected, "{stage}");
        }
        drop(held);
    }
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// The executor's part neither prescans nor parses; a malformed body is
/// refused only by the blocking part. The header goes back as Tauri's raw
/// body, so the adapter receives an ArrayBuffer.
#[test]
fn reserve_does_not_parse_and_the_response_is_a_raw_body() {
    use tauri::ipc::{InvokeResponseBody, IpcResponse};
    let host = ScriptedHost::normal();
    let (registry, _) = gated(&host);
    let body = InvokeBody::Raw(br#"{"jobId":"#.to_vec());
    let (raw, lease) = reserve(&registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1))).unwrap();
    assert!(registry.pool().reserved_total() > 0);
    assert!(matches!(refusal(super::run(registry.clone(), raw, lease).unwrap_err()), ResourceRefusal::IngressInvalid { .. }));
    assert_eq!(registry.pool().reserved_total(), 0);
    let bytes = br#"{"document_kind":"swbpipe.desktop.analysis_run_preview"}"#.to_vec();
    match response(bytes.clone()).body().unwrap() {
        InvokeResponseBody::Raw(raw) => assert_eq!(raw, bytes),
        InvokeResponseBody::Json(_) => panic!("Tauri would parse the header before the adapter receives it"),
    }
}

// ---------------------------------------------------------------------------
// RT-U4: the native build's repair, against the builder as it was
// (`old_reference`, verbatim from e231ea144)
// ---------------------------------------------------------------------------

/// Everything the builder's consumers persist, hash or reply, from one
/// built record: both checksums and the stored length; the stored bytes
/// (Save Project's `analysis_run_json`); the preview header's bytes; the
/// header record Record Run checks; the canonical record Save Project's
/// envelope hash streams; and Record Run's recorded revision (its checksum,
/// counted length and stored bytes) for a fixed revision.
#[derive(Debug, PartialEq, Eq)]
struct Persisted {
    received_sha256: String,
    record_sha256: String,
    record_bytes: u64,
    stored: Vec<u8>,
    header: Vec<u8>,
    header_record: Value,
    canonical_record_sha256: String,
    revision_sha256: String,
    revision_length: u64,
    revision_bytes: Vec<u8>,
}

/// The work of each consumer phase: the preview (the route's reader, if
/// read, the build and the header), Record Run's revision and Save
/// Project's record and canonical record.
#[derive(Debug, Default, PartialEq, Eq)]
struct Work {
    preview: passes::Counts,
    revision: passes::Counts,
    save: passes::Counts,
}

const REVISION_CREATED_AT: &str = "2026-11-04T00:00:00.000Z";
const REVISION_STATE: &str = "state:invented:rt-u4";

fn revision_refs() -> (Vec<Value>, Vec<Value>) {
    (vec![json!({"object_type": "RulePack", "ref": "rule-pack:invented@1"})], vec![json!({"object_type": "Library", "ref": "library:invented"})])
}

/// The current builder's consumers on `source` (`validate` reads the route's reader first).
fn persisted_now(source: &mut Value, route: Option<Route>, inputs: &BuildInputs<'_>) -> (Result<Persisted, String>, Work) {
    let mut work = Work::default();
    let start = passes::snapshot();
    let route = match route.map_or_else(|| dispatch(source).and_then(|route| validate_route(source, &route).map(|()| route)), Ok) {
        Ok(route) => route,
        Err(text) => return (Err(text), Work { preview: passes::since(start), ..Work::default() }),
    };
    let source: &Value = source;
    let built = match build(source, route, inputs, &mut no_lease) {
        Ok(built) => built,
        Err(error) => return (Err(command_text(error)), Work { preview: passes::since(start), ..Work::default() }),
    };
    let (header, ()) = header_bytes(&built, "job-rt-u4", |_| Ok(())).unwrap();
    work.preview = passes::since(start);
    let (rule_pack_refs, library_refs) = revision_refs();
    let ((revision_sha256, revision_length, revision_bytes), counts) = passes::measure(|| {
        let (revision, length) =
            Revision { created_at: REVISION_CREATED_AT, state_id: REVISION_STATE, rule_pack_refs: &rule_pack_refs, library_refs: &library_refs, record_sha256: String::new() }.sealed_and_counted(&built);
        let (bytes, ()) = revision.stored_bytes(&built, length, |_| Ok(())).unwrap();
        (revision.record_sha256.clone(), length, bytes)
    });
    work.revision = counts;
    let ((stored, canonical_record_sha256), counts) = passes::measure(|| {
        let (stored, ()) = record_bytes(&built, u64::MAX, |_| Ok(())).unwrap();
        let mut hasher = Sha256::new();
        write_canonical_record(&built, &mut hasher);
        (stored, hex(&hasher.finalize()))
    });
    work.save = counts;
    let persisted = Persisted {
        received_sha256: built.received_sha256.clone(),
        record_sha256: built.record_sha256.clone(),
        record_bytes: built.record_bytes,
        stored,
        header,
        header_record: super::header_record(&built),
        canonical_record_sha256,
        revision_sha256,
        revision_length,
        revision_bytes,
    };
    (Ok(persisted), work)
}

/// The same consumers through the builder as it was (`old_reference`).
fn persisted_before(source: &mut Value, route: Option<Route>, inputs: &BuildInputs<'_>) -> (Result<Persisted, String>, Work) {
    use old_reference as old;
    let mut work = Work::default();
    let start = passes::snapshot();
    let route = match route.map_or_else(|| dispatch(source).and_then(|route| validate_route(source, &route).map(|()| route)), Ok) {
        Ok(route) => route,
        Err(text) => return (Err(text), Work { preview: passes::since(start), ..Work::default() }),
    };
    let source: &Value = source;
    let built = match old::build(source, route, inputs, &mut no_lease) {
        Ok(built) => built,
        Err(error) => return (Err(command_text(error)), Work { preview: passes::since(start), ..Work::default() }),
    };
    let (header, ()) = old::header_bytes(&built, "job-rt-u4", |_| Ok(())).unwrap();
    work.preview = passes::since(start);
    let (rule_pack_refs, library_refs) = revision_refs();
    let ((revision_sha256, revision_length, revision_bytes), counts) = passes::measure(|| {
        let revision = old::Revision { created_at: REVISION_CREATED_AT, state_id: REVISION_STATE, rule_pack_refs: &rule_pack_refs, library_refs: &library_refs, record_sha256: String::new() }.sealed(&built);
        let length = revision.stored_length(&built);
        let (bytes, ()) = revision.stored_bytes(&built, length, |_| Ok(())).unwrap();
        (revision.record_sha256.clone(), length, bytes)
    });
    work.revision = counts;
    let ((stored, canonical_record_sha256), counts) = passes::measure(|| {
        let (stored, ()) = old::record_bytes(&built, u64::MAX, |_| Ok(())).unwrap();
        let mut hasher = Sha256::new();
        old::write_canonical_record(&built, &mut hasher);
        (stored, hex(&hasher.finalize()))
    });
    work.save = counts;
    let persisted = Persisted {
        received_sha256: built.received_sha256.clone(),
        record_sha256: built.record_sha256.clone(),
        record_bytes: built.record_bytes,
        stored,
        header,
        header_record: old::header_record(&built),
        canonical_record_sha256,
        revision_sha256,
        revision_length,
        revision_bytes,
    };
    (Ok(persisted), work)
}

/// The values the checked-profile check visits in `rows` (each read whole:
/// a row inside the profile is read to its last value).
fn row_nodes(rows: &[Value]) -> u64 {
    passes::measure(|| rows.iter().for_each(|row| assert!(!has_outside_profile(row)))).1.outside_nodes
}

/// What every comparison checks: the same outcome, byte for byte; and, for a
/// built record of `rows` rows, the work each phase drops: one row
/// interpretation per row in the preview and in Record Run's revision (their
/// checksum and length now one traversal), and the rows' second
/// checked-profile reading (every value of every row, the `results` array and
/// the source object). The readers run, and the canonical streams, are the same.
fn compare(id: &str, before: (Result<Persisted, String>, Work), now: (Result<Persisted, String>, Work), rows: Option<&[Value]>, misses: &mut Vec<String>) -> Option<(Work, Work)> {
    let ((old, old_work), (new, new_work)) = (before, now);
    if old != new {
        let summary = |outcome: &Result<Persisted, String>| match outcome {
            Ok(p) => format!("record {} received {} {} bytes", p.record_sha256, p.received_sha256, p.record_bytes),
            Err(text) => format!("refused `{text}`"),
        };
        misses.push(format!("{id}: before {} / now {}", summary(&old), summary(&new)));
        return None;
    }
    if let (Ok(_), Some(rows)) = (&new, rows) {
        let n = rows.len() as u64;
        let expected_preview = passes::Counts {
            reader_validations: old_work.preview.reader_validations,
            interpretations: old_work.preview.interpretations - n,
            // The rows' values, their `results` array and the source object itself
            // (the other members are read directly).
            outside_nodes: old_work.preview.outside_nodes - row_nodes(rows) - 2,
            canonical_hashes: old_work.preview.canonical_hashes,
        };
        let expected_revision = passes::Counts { interpretations: old_work.revision.interpretations - n, ..old_work.revision };
        if new_work.preview != expected_preview || new_work.revision != expected_revision || new_work.save != old_work.save {
            misses.push(format!("{id}: work before {old_work:?}, now {new_work:?}, expected preview {expected_preview:?}"));
        }
        assert_eq!(old_work.preview.interpretations, 3 * n, "{id}: before, the preview interpreted each row three times");
        assert_eq!((old_work.revision.interpretations, new_work.revision.interpretations), (3 * n, 2 * n), "{id}");
    } else if new_work.preview.reader_validations != old_work.preview.reader_validations
        || new_work.preview.interpretations > old_work.preview.interpretations
        || new_work.preview.canonical_hashes != old_work.preview.canonical_hashes
    {
        misses.push(format!("{id}: refused with other work: before {old_work:?}, now {new_work:?}"));
    }
    Some((old_work, new_work))
}

/// RT-U4 (2): on every case of the TypeScript builder's corpus that the
/// builder reads (records and refusals; preview cases built from their
/// session manifest), the current builder gives exactly the old builder's
/// outcome, every persisted and hashed byte included (the stored record, both
/// checksums, the stored length, the header, the header record, the canonical
/// record and Record Run's recorded revision), or the same refusal text; and
/// on each record it drops exactly the passes it claims (see [`compare`]).
#[test]
fn the_repaired_builder_writes_the_old_builders_bytes_with_fewer_passes() {
    let mut misses = Vec::new();
    let (mut records, mut refusals) = (0, 0);
    let mut dropped = (0u64, 0u64);
    let add = |total: &mut passes::Counts, c: passes::Counts| {
        total.reader_validations += c.reader_validations;
        total.interpretations += c.interpretations;
        total.outside_nodes += c.outside_nodes;
        total.canonical_hashes += c.canonical_hashes;
    };
    let (mut before_total, mut now_total) = (Work::default(), Work::default());
    for case in cases() {
        let id = case["id"].as_str().unwrap().to_string();
        let Ok(parsed) = serde_json::from_str::<Value>(&source_text(&case["source"])) else { continue };
        let exclusions: Vec<Exclusion> = serde_json::from_value(case.get("rule_pack_exclusions").cloned().unwrap_or(json!([]))).unwrap();
        // A preview case's inputs are its session's (`preview_parsed`'s reading of them).
        let (manifest_value, load_basis, aggregate);
        let inputs = if case["kind"] == "preview" {
            let Ok(manifest) = serde_json::from_str::<Value>(case["input_manifest"]["canonical_bytes"].as_str().unwrap()) else { continue };
            manifest_value = manifest;
            let Ok(basis) = model_load_basis_refs(&manifest_value) else { continue };
            load_basis = basis;
            aggregate = case["rule_check_aggregate"].as_str().map(|text| Value::String(text.into()));
            BuildInputs {
                manifest: ManifestEvidence { manifest_ref: &case["input_manifest"]["manifest_ref"], manifest_sha256: &case["input_manifest"]["manifest_sha256"], manifest: &manifest_value },
                rule_check_status: applied_rule_check_status(aggregate.as_ref(), get_opt(parsed.get("status"), "rule_check")),
                load_basis_refs: Some(&load_basis),
                rule_pack_exclusions: &exclusions,
            }
        } else {
            let manifest = &case["manifest"];
            BuildInputs {
                manifest: ManifestEvidence { manifest_ref: &manifest["manifest_ref"], manifest_sha256: &manifest["manifest_sha256"], manifest: &manifest["manifest"] },
                rule_check_status: case.get("rule_check_status"),
                load_basis_refs: case.get("load_basis_refs"),
                rule_pack_exclusions: &exclusions,
            }
        };
        let rows = parsed.get("results").and_then(Value::as_array).cloned().unwrap_or_default();
        let before = persisted_before(&mut parsed.clone(), None, &inputs);
        let now = persisted_now(&mut parsed.clone(), None, &inputs);
        let built = now.0.is_ok();
        if let Some((old, new)) = compare(&id, before, now, Some(&rows), &mut misses) {
            if built {
                records += 1;
                for (total, work) in [(&mut before_total, &old), (&mut now_total, &new)] {
                    add(&mut total.preview, work.preview);
                    add(&mut total.revision, work.revision);
                    add(&mut total.save, work.save);
                }
                dropped.0 += old.preview.interpretations + old.revision.interpretations - new.preview.interpretations - new.revision.interpretations;
                dropped.1 += old.preview.outside_nodes - new.preview.outside_nodes;
            } else {
                refusals += 1;
            }
        }
    }
    assert!(misses.is_empty(), "{} cases differ:\n{}", misses.len(), misses.join("\n"));
    eprintln!("RT-U4 corpus: {records} records and {refusals} refusals compared; over the records, before {before_total:?}, now {now_total:?}");
    assert!(records >= 40 && refusals >= 40, "{records} records, {refusals} refusals compared");
    assert!(dropped.0 > 0 && dropped.1 > 0, "{dropped:?}");
}

/// RT-U4 (2) on invented racks: the same bytes and the claimed passes at
/// each size, and the same refusal, in the same order, where the rows or the
/// other members leave the checked profile or a row is uninterpretable.
#[test]
fn the_repaired_builder_on_racks_and_their_refusals_is_the_old_builder() {
    let manifest = json!({"manifest_ref": {"object_type": "InputManifest", "ref": "input-manifest:rack"}, "manifest_sha256": "1".repeat(64),
        "manifest": {"model_basis": {"model_ref": "model:rack"}, "solver_basis": {"solver_name": "invented", "solver_version": "0.0.0", "solver_build_ref": "build:rack"}}});
    let load_basis = json!([{"object_type": "LoadCase", "ref": "case:a"}]);
    let inputs = rack_inputs(&manifest, &load_basis);
    let mut misses = Vec::new();
    for rows in [1usize, 7, 300, 2_000] {
        let (source, route) = rack(rows);
        let results = source["results"].as_array().unwrap().clone();
        let (before, now) = (persisted_before(&mut source.clone(), Some(route.clone()), &inputs), persisted_now(&mut source.clone(), Some(route.clone()), &inputs));
        let (old, new) = compare(&format!("rack {rows}"), before, now, Some(&results), &mut misses).expect("the same record");
        eprintln!("RT-U4 rack {rows}: before {old:?}, now {new:?}");
        assert_eq!((old.preview.interpretations, new.preview.interpretations), (3 * rows as u64, 2 * rows as u64));
        // Refusals: (row, member) edits, each alone and with an earlier refusal.
        let unsafe_number = json!(9_007_199_254_740_993.0f64);
        let edits: Vec<(&str, Box<dyn Fn(&mut Value)>)> = vec![
            ("a member outside the profile", Box::new(|s: &mut Value| s["extra"] = json!({"deep": [unsafe_number.clone()]}))),
            ("a diagnostic outside the profile", Box::new(|s: &mut Value| s["diagnostics"] = json!([{"n": unsafe_number.clone()}]))),
            ("the last row outside the profile", Box::new(|s: &mut Value| { let last = s["results"].as_array().unwrap().len() - 1; s["results"][last]["value"] = unsafe_number.clone(); })),
            ("a row outside and a member outside", Box::new(|s: &mut Value| { s["results"][0]["value"] = unsafe_number.clone(); s["extra"] = unsafe_number.clone(); })),
            ("an outside row before an uninterpretable one", Box::new(|s: &mut Value| { let last = s["results"].as_array().unwrap().len() - 1; s["results"][0]["value"] = unsafe_number.clone(); s["results"][last]["unit"] = json!("m"); })),
            ("an uninterpretable row and a member outside", Box::new(|s: &mut Value| { s["results"][0]["unit"] = json!("m"); s["extra"] = unsafe_number.clone(); })),
            ("diagnostics not a list and a member outside", Box::new(|s: &mut Value| { s["diagnostics"] = json!({"n": unsafe_number.clone()}); s["extra"] = unsafe_number.clone(); })),
            ("duplicate row ids", Box::new(|s: &mut Value| { let last = s["results"].as_array().unwrap().len() - 1; s["results"][last]["id"] = json!("result:r0"); })),
        ];
        for (what, edit) in &edits {
            let mut edited = source.clone();
            edit(&mut edited);
            let results = edited["results"].as_array().unwrap().clone();
            let (before, now) = (persisted_before(&mut edited.clone(), Some(route.clone()), &inputs), persisted_now(&mut edited.clone(), Some(route.clone()), &inputs));
            let refused = now.0.as_ref().err().cloned();
            compare(&format!("rack {rows}: {what}"), before, now, Some(&results), &mut misses);
            if !(rows == 1 && *what == "duplicate row ids") {
                assert!(refused.is_some(), "rack {rows}: {what} is refused");
            }
        }
    }
    assert!(misses.is_empty(), "{} differ:\n{}", misses.len(), misses.join("\n"));
}

/// The native verdict command (`read_preview_mechanics_job_verdict`) in this
/// thread, as the session's registration and standing call it.
pub(crate) fn verdict(registry: &SolveJobRegistry, job: &str, reader: Option<&str>, model: Option<&str>) -> Result<String, CommandError<String>> {
    let args = json!({"jobId": job, "reader": reader, "modelJson": model, "invocationJson": null});
    let body = InvokeBody::Raw(serde_json::to_vec(&args).unwrap());
    let (raw, lease) = crate::native_verdict::reserve(registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1)))?;
    crate::native_verdict::run(registry.clone(), raw, lease)
}

fn case_named(id: &str) -> Value {
    cases().into_iter().find(|case| case["id"] == id).unwrap_or_else(|| panic!("{id}"))
}

/// The preview header and the bounded record bytes of `job`, and the work of each.
fn preview_and_bytes(registry: &SolveJobRegistry, job: &str, case: &Value) -> ((Result<Vec<u8>, String>, passes::Counts), (Result<Vec<u8>, String>, passes::Counts)) {
    let args = preview_args(job, case);
    let header = passes::measure(|| call(registry, super::run, &args).map_err(command_text));
    let bytes = passes::measure(|| preview_bytes(registry, &args, u64::MAX).map(|(bytes, _)| bytes).map_err(command_text));
    (header, bytes)
}

/// RT-U4 (1): the registration's pass of the `pressure` reader on a held
/// solve publication is the only reader evidence the builder reuses, and only
/// for that publication.
/// - Unregistered, the preview and the bytes route each read the route's
///   reader (one full validation each). After the session's registration (the
///   verdict command's `pressure` validation with the solve's model) passes on
///   the same share, neither reads it, and both reply byte for byte as before.
/// - Another job holding the same bytes has no evidence and reads the reader;
///   a dispatch, a refused validation (a model the G2 checks refuse, a model
///   that is not JSON) and another reader's pass leave none.
/// - A source the reader refuses has no evidence, and the preview refuses it
///   with the reader's own message, exactly as the old builder does.
#[test]
fn the_registrations_pressure_pass_is_reused_only_for_its_own_publication() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let case = case_named("preview:pressure");
    let text = source_text(&case["source"]);
    let model = case["intake_text"].as_str().unwrap();
    assert_eq!(dispatch(&serde_json::from_str(&text).unwrap()).unwrap().name, "pressure");
    let job = retained(&registry, &gate, &text);
    let ((header, unregistered), (bytes, unregistered_bytes)) = preview_and_bytes(&registry, &job, &case);
    let (header, bytes) = (header.unwrap(), bytes.unwrap());
    assert_eq!((unregistered.reader_validations, unregistered_bytes.reader_validations), (1, 1));
    // The registration's model-bound validation of the same held bytes.
    assert!(!registry.held_result(&job).unwrap().pressure_reader_passed());
    assert_eq!(verdict(&registry, &job, Some("pressure"), Some(model)).unwrap(), crate::solve_jobs::READER_PASSED);
    assert!(registry.held_result(&job).unwrap().pressure_reader_passed());
    let ((again, registered), (bytes_again, registered_bytes)) = preview_and_bytes(&registry, &job, &case);
    assert_eq!((registered.reader_validations, registered_bytes.reader_validations), (0, 0), "the registration's pass is reused");
    assert_eq!((again.unwrap(), bytes_again.unwrap()), (header.clone(), bytes.clone()), "the same header and record, byte for byte");
    // Only the reader's run is saved; everything else is read as before.
    assert_eq!(passes::Counts { reader_validations: 1, ..registered }, unregistered);
    // Another publication of the same bytes holds no evidence of its own.
    let other = retained(&registry, &gate, &text);
    let ((other_header, other_work), _) = preview_and_bytes(&registry, &other, &case);
    assert_eq!(other_work.reader_validations, 1, "no foreign proof");
    let job_name = |bytes: Vec<u8>, job: &str| String::from_utf8(bytes).unwrap().replace(job, "$job");
    assert_eq!(job_name(other_header.unwrap(), &other), job_name(header.clone(), &job));
    // What notes nothing: a dispatch, refusals, another reader.
    let unknown_cases = json!({"project": {"id": "p"}, "load_cases": [{"id": "case:invented-other"}]}).to_string();
    let outcomes = [
        verdict(&registry, &other, None, Some(model)).unwrap(),
        verdict(&registry, &other, Some("pressure"), Some(&unknown_cases)).unwrap(),
        format!("{:?}", verdict(&registry, &other, Some("pressure"), Some("{not json")).unwrap_err()),
        verdict(&registry, &other, Some("pressure-source"), None).unwrap(),
        verdict(&registry, &other, Some("preview-physics"), Some(model)).unwrap(),
    ];
    assert!(outcomes.iter().all(|outcome| outcome != crate::solve_jobs::READER_PASSED), "{outcomes:?}");
    assert!(!registry.held_result(&other).unwrap().pressure_reader_passed());
    // A source the reader refuses: no evidence, and the reader's own refusal.
    let mut tampered: Value = serde_json::from_str(&text).unwrap();
    tampered["contract_evidence"].as_object_mut().unwrap().clear();
    let tampered_text = tampered.to_string();
    let bad = retained(&registry, &gate, &tampered_text);
    let refusal = verdict(&registry, &bad, Some("pressure"), Some(model)).unwrap();
    assert_ne!(refusal, crate::solve_jobs::READER_PASSED, "the reader refuses the tampered source");
    assert!(!registry.held_result(&bad).unwrap().pressure_reader_passed());
    let ((refused, refused_work), _) = preview_and_bytes(&registry, &bad, &case);
    let refused = refused.unwrap_err();
    assert_eq!(refused_work.reader_validations, 1);
    let mut source: Value = serde_json::from_str(&tampered_text).unwrap();
    let route = dispatch(&source).unwrap();
    assert_eq!(validate_route(&mut source, &route).unwrap_err(), refused, "the old builder's refusal");
    for job in [&job, &other, &bad] {
        registry.dispose_result(job).unwrap();
    }
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// RT-U4 (1): no other route reuses a pass. The preview-physics
/// registration's model-bound pass does not establish the model-free
/// reader's (the latter alone re-derives the combination gates from the
/// document, `preview_physics_evidence::validate_with`), so the builder
/// still reads it after that registration.
#[test]
fn a_preview_physics_registration_leaves_the_builders_reader_to_run() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let case = case_named("preview:connected");
    let text = source_text(&case["source"]);
    let job = retained(&registry, &gate, &text);
    let outcome = verdict(&registry, &job, Some("preview-physics"), Some(case["intake_text"].as_str().unwrap())).unwrap();
    assert_eq!(outcome, crate::solve_jobs::READER_PASSED, "the registration passed");
    assert!(!registry.held_result(&job).unwrap().pressure_reader_passed());
    let ((header, work), (_, bytes_work)) = preview_and_bytes(&registry, &job, &case);
    header.unwrap();
    assert_eq!((work.reader_validations, bytes_work.reader_validations), (1, 1));
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// RT-U4 (1), with review F1's interleavings: the evidence is part of the
/// held share. A disposal, a session reset or a C2 full-copy release between
/// the preview's acquisition of its share (the `held` gate) and its read
/// neither gives the read evidence nor takes it away: a registered share
/// reuses the pass and an unregistered one reads the reader, and each replies
/// exactly as an undisturbed read of the same job.
#[test]
fn a_disposal_reset_or_release_during_the_build_never_changes_the_evidence_it_reads() {
    let case = case_named("preview:pressure");
    let text = source_text(&case["source"]);
    let model = case["intake_text"].as_str().unwrap().to_string();
    for registered in [true, false] {
        for interleaving in ["disposal", "reset", "release"] {
            let label = format!("registered {registered}, {interleaving}");
            let host = ScriptedHost::normal();
            let (registry, gate) = gated(&host);
            let job = retained(&registry, &gate, &text);
            if registered {
                assert_eq!(verdict(&registry, &job, Some("pressure"), Some(&model)).unwrap(), crate::solve_jobs::READER_PASSED);
            }
            let args = preview_args(&job, &case);
            let (expected, undisturbed) = passes::measure(|| call(&registry, super::run, &args).unwrap());
            assert_eq!(undisturbed.reader_validations, u64::from(!registered), "{label}");
            let held = gate::hold(&registry, "held", false);
            let reader = std::thread::spawn({
                let (registry, args) = (registry.clone(), args.clone());
                move || passes::measure(|| call(&registry, super::run, &args).map_err(command_text))
            });
            assert!(held.0.wait_entered(1, Duration::from_secs(30)), "{label}: the build acquired its share");
            match interleaving {
                "disposal" => assert!(registry.dispose_result(&job).unwrap().released, "{label}"),
                "reset" => assert!(registry.dispose_all_results().released_results >= 1, "{label}"),
                _ => assert!(!registry.release_full_copy(&job).unwrap(), "{label}: no full copy was held"),
            }
            if interleaving != "release" {
                assert!(registry.held_result(&job).is_err(), "{label}: the job is gone while its build reads");
            }
            held.0.open();
            let (got, work) = reader.join().unwrap();
            drop(held);
            assert_eq!(got.unwrap(), expected, "{label}: the reply of an undisturbed read");
            assert_eq!(work, undisturbed, "{label}: the evidence of its own share, no other");
            if interleaving == "release" {
                registry.dispose_result(&job).unwrap();
            }
            assert_eq!(registry.pool().reserved_total(), 0, "{label}: every share released");
        }
    }
}
