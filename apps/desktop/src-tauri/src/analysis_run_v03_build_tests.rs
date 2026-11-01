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
