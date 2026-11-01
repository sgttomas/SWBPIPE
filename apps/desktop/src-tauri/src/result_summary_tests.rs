//! C1 slice 2: S, the summary of a retained result (`result_summary`).
//!
//! - **Committed outcomes.** Every case of `fixtures/results/result_summary_cases.json`
//!   (written by `resultSummaryCases.ts`: the result fixtures the TypeScript
//!   builders' tests and the replay use, each with its model, and invented
//!   cases for the withheld, missing, tied, signed and mixed-unit paths) is
//!   built here and must equal its committed response in
//!   `fixtures/results/result_summary_outcomes.json`, byte for byte.
//!   `RESULT_SUMMARY_OUTCOMES_WRITE=1` rewrites that file. The vitest leg
//!   (`resultSummaryParity.test.ts`) decodes the same responses and compares
//!   them with the TypeScript builders over the full rows.
//! - Each case whose standing states findings is also read through the
//!   command on a real retained job, with the same bytes.
//! - The planes are checked bit for bit against the rows, and the layout's
//!   refusals by name.
use super::*;
use crate::command_ingress::test_support::headers;
use crate::command_ingress::INGRESS_ARGS_JSON_V1;
pub(crate) use crate::native_verdict::tests::gate;
use crate::memory_pool::test_host::ScriptedHost;
use crate::memory_pool::{MemoryPool, PressureLevel, ResourceRefusal};
use crate::result_planes::{decode, Decoded, DISPLAY_COLUMNS, MAGIC, SOURCE_OFFSETS, SOURCE_REFS, STRESS_COLUMNS};
use crate::solve_jobs::test_support::wait_terminal;
use crate::solve_jobs::SolveJobError;
use crate::solve_worker::test_support::Gate;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use std::time::Duration;

const CASES: &str = "fixtures/results/result_summary_cases.json";
const OUTCOMES: &str = "fixtures/results/result_summary_outcomes.json";
const MODEL: &[u8] = br#"{"project":{"id":"p"},"nodes":[1,2,3]}"#;

fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}
fn read_text(path: &str) -> String {
    std::fs::read_to_string(root().join(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}

// ---------------------------------------------------------------------------
// Cases and outcomes
// ---------------------------------------------------------------------------

enum StandingSpec {
    None,
    Findings { eligible: bool, findings: Vec<String> },
    Health { eligible: bool, health: Value },
}

struct SummaryCase {
    name: String,
    source: String,
    model: String,
    standing: StandingSpec,
}

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

fn cases() -> Vec<SummaryCase> {
    let file: Value = serde_json::from_str(&read_text(CASES)).unwrap();
    file["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| {
            let standing = match &case["standing"] {
                Value::Null => StandingSpec::None,
                s if s.get("health").is_some() => StandingSpec::Health { eligible: s["eligible"].as_bool().unwrap(), health: s["health"].clone() },
                s => StandingSpec::Findings {
                    eligible: s["eligible"].as_bool().unwrap(),
                    findings: s["findings"].as_array().unwrap().iter().map(|f| f.as_str().unwrap().to_string()).collect(),
                },
            };
            SummaryCase { name: case["name"].as_str().unwrap().into(), source: text_of(&case["result"]), model: text_of(&case["model"]), standing }
        })
        .collect()
}

/// A case's response, built from its parsed texts.
fn built(case: &SummaryCase) -> Vec<u8> {
    let source: Value = serde_json::from_str(&case.source).unwrap();
    let model: Value = serde_json::from_str(&case.model).unwrap();
    let (health, standing) = match &case.standing {
        StandingSpec::None => (Value::Null, None),
        StandingSpec::Findings { eligible, findings } => {
            let findings = serde_json::to_string(findings).unwrap();
            let text = open_pipe_stress_operation_applier::numerical_health_json(&case.source, Some(&findings)).unwrap();
            let health: Value = serde_json::from_str(&text).unwrap();
            let view = tables::health_view(&health);
            (health, Some(Standing { eligible: *eligible, health: view }))
        }
        StandingSpec::Health { eligible, health } => (health.clone(), Some(Standing { eligible: *eligible, health: tables::health_view(health) })),
    };
    build(&case.name, &source, &model, standing.as_ref(), &health, |_| Ok::<(), ()>(())).unwrap().0
}

pub(crate) fn base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = (chunk[0] as u32) << 16 | (*chunk.get(1).unwrap_or(&0) as u32) << 8 | *chunk.get(2).unwrap_or(&0) as u32;
        for i in 0..4 {
            if i <= chunk.len() {
                out.push(ALPHABET[(n >> (18 - 6 * i) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
}

#[test]
fn summaries_are_the_committed_outcomes() {
    let cases = cases();
    assert!(cases.len() >= 30, "{} cases", cases.len());
    let entries: Vec<Value> = cases
        .iter()
        .map(|case| {
            let bytes = built(case);
            json!({"name": case.name, "length": bytes.len(), "sha256": sha256(&bytes), "response": base64(&bytes)})
        })
        .collect();
    let outcomes = json!({
        "document_kind": "swbpipe.test.result_summary_outcomes",
        "note": "Written by RESULT_SUMMARY_OUTCOMES_WRITE=1 cargo test summaries_are_the_committed_outcomes (apps/desktop/src-tauri); read by resultSummaryParity.test.ts.",
        "cases": entries,
    });
    let text = serde_json::to_string_pretty(&outcomes).unwrap() + "\n";
    if std::env::var("RESULT_SUMMARY_OUTCOMES_WRITE").is_ok() {
        std::fs::write(root().join(OUTCOMES), &text).unwrap();
        return;
    }
    let committed: Value = serde_json::from_str(&read_text(OUTCOMES)).unwrap();
    let committed = committed["cases"].as_array().unwrap();
    assert_eq!(committed.len(), entries.len(), "the committed outcomes list another set of cases");
    let misses: Vec<String> = entries
        .iter()
        .zip(committed)
        .filter(|(got, want)| got["name"] != want["name"] || got["sha256"] != want["sha256"] || got["response"] != want["response"])
        .map(|(got, want)| format!("{} (committed {}): {} != {}", got["name"], want["name"], got["sha256"], want["sha256"]))
        .collect();
    assert!(misses.is_empty(), "{} summaries differ from the committed outcomes:\n{}", misses.len(), misses.join("\n"));
}

// ---------------------------------------------------------------------------
// The planes, bit for bit
// ---------------------------------------------------------------------------

fn code_value<'v>(values: &'v [Value], code: f64) -> Option<&'v Value> {
    assert!(code.fract() == 0.0 && code >= -1.0 && code < values.len() as f64, "code {code} of {} values", values.len());
    (code >= 0.0).then(|| &values[code as usize])
}

/// The display rows: every row of a viewport kind, and each row an
/// intensified-bending row names (the last row of its id).
fn expected_display(rows: &[Value]) -> (Vec<usize>, Vec<Vec<i64>>) {
    let kinds: Vec<&str> = [
        "displacement_magnitude",
        "global_nodal_displacement_x",
        "global_nodal_displacement_y",
        "global_nodal_displacement_z",
        "global_nodal_rotation_x",
        "global_nodal_rotation_y",
        "global_nodal_rotation_z",
        tables::REACTION_COMPONENT_KIND,
        tables::REACTION_FORCE_MAGNITUDE_KIND,
        tables::REACTION_MOMENT_MAGNITUDE_KIND,
    ]
    .into_iter()
    .chain(tables::COLOUR_QUANTITIES.iter().map(|q| q.1))
    .collect();
    let last = |id: &Value| rows.iter().rposition(|row| row.get("id") == Some(id)).map_or(-1, |i| i as i64);
    let mut sources = std::collections::BTreeSet::new();
    let mut refs = vec![Vec::new(); rows.len()];
    for (i, row) in rows.iter().enumerate() {
        if row["kind"] == "component_equal_factor_intensified_bending_stress_v1" {
            for reference in row["source_result_refs"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
                let at = last(reference);
                refs[i].push(at);
                if at >= 0 {
                    sources.insert(at as usize);
                }
            }
        }
    }
    let display: Vec<usize> = (0..rows.len()).filter(|i| rows[*i]["kind"].as_str().is_some_and(|k| kinds.contains(&k)) || sources.contains(i)).collect();
    let refs = display.iter().map(|i| refs[*i].clone()).collect();
    (display, refs)
}

fn same_bits(plane: f64, value: Option<&Value>) -> bool {
    match tables::finite(value) {
        Some(x) => plane.to_bits() == x.to_bits(),
        None => plane.is_nan(),
    }
}

fn check_planes(name: &str, source: &Value, decoded: &Decoded) {
    let rows = tables::rows(source);
    let header = &decoded.header;
    let values = header["values"].as_array().unwrap();
    let (display, refs) = expected_display(rows);
    let ordinals = decoded.plane("display.ordinal");
    assert_eq!(ordinals.iter().map(|o| *o as usize).collect::<Vec<_>>(), display, "{name}: display rows");
    assert_eq!(header["display"]["rows"], json!(display.len()), "{name}");
    let members: [fn(&Value) -> Option<&Value>; 10] = [
        |r| r.get("kind"),
        |r| r.get("entity_ref"),
        |r| r.get("basis_ref"),
        |r| r.get("unit"),
        |r| tables::meta(r, "coordinate_system"),
        |r| tables::meta(r, "component"),
        |r| tables::meta(r, "location"),
        |r| tables::meta(r, "basis"),
        |r| tables::meta(r, "sign_convention"),
        |r| r.get("dimension"),
    ];
    let offsets = decoded.plane(SOURCE_OFFSETS);
    let source_refs = decoded.plane(SOURCE_REFS);
    assert_eq!(offsets.len(), display.len() + 1, "{name}");
    assert_eq!(offsets[display.len()] as usize, source_refs.len(), "{name}");
    for (i, &ordinal) in display.iter().enumerate() {
        let row = &rows[ordinal];
        assert!(same_bits(decoded.plane("display.value")[i], row.get("value")), "{name}: row {ordinal}'s value");
        for (column, member) in members.iter().enumerate() {
            let plane = decoded.plane(DISPLAY_COLUMNS[column + 2]);
            assert_eq!(code_value(values, plane[i]), member(row), "{name}: row {ordinal}'s {}", DISPLAY_COLUMNS[column + 2]);
        }
        let got: Vec<i64> = source_refs[offsets[i] as usize..offsets[i + 1] as usize].iter().map(|r| *r as i64).collect();
        assert_eq!(got, refs[i], "{name}: row {ordinal}'s sources");
    }
    // Stress rows: each group's admitted rows, in row order, bit for bit.
    let cases = header["tables"]["support_load_cases"].as_array().unwrap();
    let mut covered = 0usize;
    for group in header["tables"]["stress_ratio"]["groups"].as_array().unwrap() {
        let (first, count) = (group["rows"]["first"].as_u64().unwrap() as usize, group["rows"]["count"].as_u64().unwrap() as usize);
        assert_eq!(first, covered, "{name}: groups are contiguous");
        covered += count;
        let mut previous = None;
        for i in first..first + count {
            let ordinal = decoded.plane("stress.ordinal")[i] as usize;
            assert!(previous.map_or(true, |p| p < ordinal), "{name}: row order in {}", group["id"]);
            previous = Some(ordinal);
            let row = &rows[ordinal];
            assert_eq!(row["kind"], group["ref"], "{name}");
            assert!(same_bits(decoded.plane("stress.value")[i], row.get("value")), "{name}: stress row {ordinal}");
            let case = &cases[decoded.plane("stress.case")[i] as usize];
            assert_eq!(Some(case["key"].as_str().unwrap().to_string()), tables::basis_key(row.get("basis_ref")), "{name}");
            assert_eq!(code_value(values, decoded.plane("stress.entity_ref")[i]), row.get("entity_ref"), "{name}");
            assert_eq!(code_value(values, decoded.plane("stress.unit")[i]), row.get("unit"), "{name}");
            let text = |key| tables::meta(row, key).filter(|v| v.is_string()).cloned().unwrap_or(Value::Null);
            assert_eq!(code_value(values, decoded.plane("stress.station")[i]), Some(&text("location")), "{name}");
            assert_eq!(code_value(values, decoded.plane("stress.component")[i]), Some(&text("component")), "{name}");
            let basis = tables::section_basis_of(row).map_or(Value::Null, |b| json!(b));
            assert_eq!(code_value(values, decoded.plane("stress.section_basis")[i]), Some(&basis), "{name}");
            code_value(values, decoded.plane("stress.case_label")[i]).expect("a label");
        }
    }
    assert_eq!(covered, decoded.plane("stress.ordinal").len(), "{name}");
    for column in STRESS_COLUMNS {
        assert_eq!(decoded.plane(column).len(), covered, "{name}: {column}");
    }
}

#[test]
fn the_planes_are_the_rows_bit_for_bit() {
    for case in cases() {
        let bytes = built(&case);
        let decoded = decode(&bytes).unwrap_or_else(|e| panic!("{}: {e}", case.name));
        let source: Value = serde_json::from_str(&case.source).unwrap();
        assert_eq!(decoded.header["row_count"], json!(tables::rows(&source).len()));
        check_planes(&case.name, &source, &decoded);
    }
}

/// The layout: the magic, the header's little-endian length, zero padding to
/// 8, the planes 8-aligned from the buffer's start, nothing after the last.
#[test]
fn the_layout_is_exact_and_little_endian() {
    let case = cases().into_iter().find(|c| c.name.starts_with("results/preview_physics_connected_sparse")).unwrap();
    let bytes = built(&case);
    assert_eq!(&bytes[..8], MAGIC);
    let header_len = u64::from_le_bytes(bytes[8..16].try_into().unwrap()) as usize;
    let header: Value = serde_json::from_slice(&bytes[16..16 + header_len]).unwrap();
    let start = (16 + header_len).div_ceil(8) * 8;
    assert!(bytes[16 + header_len..start].iter().all(|b| *b == 0));
    assert_eq!(bytes.len(), start + header["planes_bytes"].as_u64().unwrap() as usize);
    let first = &header["planes"][0];
    assert_eq!(first["name"], "display.ordinal");
    let ordinal = f64::from_le_bytes(bytes[start..start + 8].try_into().unwrap());
    assert_eq!(ordinal, decode(&bytes).unwrap().plane("display.ordinal")[0]);
}

/// Assembles a response from a header and planes, for the refusal cases.
fn assemble(mut header: Value, planes: &[(&str, u64, Vec<f64>)], pad: u8, extra: &[u8]) -> Vec<u8> {
    let mut body = Vec::new();
    let mut descriptors = Vec::new();
    for (name, offset, values) in planes {
        descriptors.push(json!({"name": name, "offset": offset, "count": values.len()}));
        body.extend(values.iter().flat_map(|v| v.to_le_bytes()));
    }
    header["planes"] = json!(descriptors);
    if header.get("planes_bytes").is_none() {
        header["planes_bytes"] = json!(body.len());
    }
    let text = serde_json::to_vec(&header).unwrap();
    let mut out = MAGIC.to_vec();
    out.extend((text.len() as u64).to_le_bytes());
    out.extend(&text);
    while out.len() % 8 != 0 {
        out.push(pad);
    }
    out.extend(body);
    out.extend(extra);
    out
}

#[test]
fn a_malformed_or_truncated_response_is_refused_by_name() {
    let good = assemble(json!({"x": 1}), &[("a", 0, vec![1.0, -0.0]), ("b", 16, vec![f64::NAN])], 0, &[]);
    let decoded = decode(&good).unwrap();
    assert_eq!(decoded.plane("a")[1].to_bits(), (-0.0f64).to_bits());
    assert!(decoded.plane("b")[0].is_nan());
    let refused = |bytes: &[u8], reason: &str| {
        let error = decode(bytes).unwrap_err();
        assert!(error.starts_with(result_planes::MALFORMED) && error.contains(reason), "{error} (wanted {reason})");
    };
    // Every truncation is refused.
    for len in 0..good.len() {
        assert!(decode(&good[..len]).is_err(), "a response cut at {len} bytes was read");
    }
    refused(&good[..15], "not a result summary");
    let mut foreign = good.clone();
    foreign[0] = b'X';
    refused(&foreign, "not a result summary");
    let mut long = good.clone();
    long[8..16].copy_from_slice(&u64::MAX.to_le_bytes());
    refused(&long, "the header runs past the end");
    refused(&[good.clone(), vec![0]].concat(), "remaining bytes");
    refused(&assemble(json!({"x": 1}), &[("a", 0, vec![1.0])], 7, &[]), "padding is not zero");
    refused(&assemble(json!({"x": 1}), &[("a", 0, vec![1.0]), ("b", 16, vec![2.0])], 0, &[]), "not where the previous one ends");
    refused(&assemble(json!({"x": 1}), &[("a", 0, vec![1.0]), ("a", 8, vec![2.0])], 0, &[]), "stated twice");
    refused(&assemble(json!({"planes_bytes": 16}), &[("a", 0, vec![1.0])], 0, &[0; 8]), "bytes follow the last plane");
    refused(&assemble(json!({"planes_bytes": 8}), &[("a", 0, vec![1.0, 2.0])], 0, &[]), "remaining bytes");
    let mut text_header = good.clone();
    text_header[16] = b'[';
    refused(&text_header, "the header is not JSON");
}

// ---------------------------------------------------------------------------
// S grows with entities and sets, not with station rows
// ---------------------------------------------------------------------------

/// An invented rack: `pipes` members in a line, a support at every fourth
/// node, three load cases and a combination; per member and set, `kinds`
/// stress kinds at the five stations and two section bases each.
fn rack(pipes: usize, kinds: usize) -> (Value, Value, Value) {
    let nodes: Vec<Value> = (0..=pipes).map(|i| json!({"id": format!("node:{i}"), "position": {"x": i, "y": 0, "z": 0}})).collect();
    let pipe_segments: Vec<Value> = (0..pipes).map(|i| json!({"id": format!("pipe:{i}"), "from": format!("node:{i}"), "to": format!("node:{}", i + 1)})).collect();
    let supports: Vec<Value> = (0..=pipes).step_by(4).map(|i| json!({"id": format!("support:{i}"), "node": format!("node:{i}"), "family": "guide"})).collect();
    let sets: Vec<Value> = ["L1", "L2", "L3"].iter().map(|c| json!({"ref_type": "load_case", "ref_id": c})).chain([json!({"ref_type": "combination", "ref_id": "C1"})]).collect();
    let model = json!({
        "nodes": nodes, "pipe_segments": pipe_segments, "supports": supports, "components": [],
        "load_cases": [{"id": "L1", "label": "One"}, {"id": "L2", "label": "Two"}, {"id": "L3", "label": "Three"}],
        "combinations": [{"id": "C1", "label": "Sum"}]
    });
    let mut rows = Vec::new();
    let mut value = 0.5f64;
    let mut next = || {
        value = value * 1.000_37 + 0.013;
        value
    };
    for (s, set) in sets.iter().enumerate() {
        for i in 0..=pipes {
            for kind in ["global_nodal_displacement_x", "global_nodal_displacement_y", "global_nodal_displacement_z", "global_nodal_rotation_x", "global_nodal_rotation_y", "global_nodal_rotation_z", "displacement_magnitude"] {
                rows.push(json!({"id": format!("r:{s}:{kind}:node:{i}"), "kind": kind, "entity_ref": format!("node:{i}"), "basis_ref": set, "value": next(), "unit": "mm", "metadata": {"coordinate_system": "global"}}));
            }
        }
        for i in (0..=pipes).step_by(4) {
            for component in ["Fx", "Fy", "Fz", "Mx", "My", "Mz"] {
                rows.push(json!({"id": format!("r:{s}:reaction:{i}:{component}"), "kind": tables::REACTION_COMPONENT_KIND, "entity_ref": format!("support:{i}"), "basis_ref": set, "value": next(), "unit": "N", "metadata": {"coordinate_system": "global", "component": component, "sign_convention": "support-on-pipe"}}));
            }
        }
        for i in 0..pipes {
            for (kind, _) in tables::STRESS_KINDS.iter().take(kinds) {
                for station in ["end_i", "quarter_1", "midspan", "quarter_3", "end_j"] {
                    for basis in ["nominal_section", "reduced_section"] {
                        rows.push(json!({
                            "id": format!("r:{s}:{kind}:pipe:{i}:{station}:{basis}"), "kind": kind, "entity_ref": format!("pipe:{i}"), "basis_ref": set,
                            "value": next(), "unit": "Pa", "metadata": {"coordinate_system": "pipe_section", "location": station, "component": "sigma_plus", "basis": format!("station; section_basis={basis}"), "sign_convention": "j-side section action"}
                        }));
                    }
                }
            }
        }
    }
    let source = json!({
        "schema_version": "0.2.0", "document_kind": "mechanics_result", "run_id": "run:rack", "model_ref": "model:rack",
        "status": {"mechanics": "MECHANICS_SOLVED", "rule_check": "not_run", "professional_acceptance": "not_claimed"},
        "summary": {}, "results": rows, "diagnostics": []
    });
    let health_cases: Vec<Value> = ["L1", "L2", "L3"]
        .iter()
        .map(|c| json!({"case_id": c, "state": "checks_passed", "code": null, "integrity_diagnostic_id": null, "rcond": null, "digits": null, "conditioning_sensitive": false, "causes": []}))
        .collect();
    let health = json!({
        "route": "generic", "verdict_only": false,
        "cases": health_cases,
        "combinations": [{"combination_id": "C1", "state": "checks_passed", "cases": ["L1", "L2", "L3"], "withheld": false}]
    });
    (source, model, health)
}

fn rack_summary(pipes: usize, kinds: usize) -> (usize, usize, usize) {
    let (source, model, health) = rack(pipes, kinds);
    let standing = Standing { eligible: true, health: tables::health_view(&health) };
    let bytes = build("rack", &source, &model, Some(&standing), &health, |_| Ok::<(), ()>(())).unwrap().0;
    let header_len = u64::from_le_bytes(bytes[8..16].try_into().unwrap()) as usize;
    let decoded = decode(&bytes).unwrap();
    check_planes("rack", &source, &decoded);
    (header_len, bytes.len() - header_len, serde_json::to_string(&source).unwrap().len())
}

#[test]
fn s_json_grows_with_entities_and_sets_not_station_rows() {
    // The same 40 members and 4 sets, with 1 and then 12 stress kinds at every
    // station and basis: 12 times the station rows.
    let (header_1, planes_1, text_1) = rack_summary(40, 1);
    let (header_12, planes_12, text_12) = rack_summary(40, 12);
    assert!(text_12 > 5 * text_1, "{text_1} -> {text_12}");
    assert!(planes_12 > 5 * planes_1, "{planes_1} -> {planes_12}");
    // The JSON part adds only each new kind's group and its name.
    assert!(header_12 < header_1 + 12 * 1024, "S's JSON part grew with station rows: {header_1} -> {header_12}");
    assert!(header_12 * 10 < text_12, "S's JSON part is not small beside the envelope: {header_12} of {text_12}");
    // Twice the members, about twice the JSON part.
    let (header_80, _, _) = rack_summary(80, 12);
    assert!(header_80 < header_12 * 9 / 4 && header_80 > header_12 * 3 / 2, "{header_12} -> {header_80}");
}

// ---------------------------------------------------------------------------
// The command: admission, custody and the executor
// ---------------------------------------------------------------------------

fn gated(host: &Arc<ScriptedHost>) -> (SolveJobRegistry, Arc<Gate>) {
    let gate = Gate::new("summary");
    let pool = MemoryPool::with_timing(host.clone(), Duration::from_millis(5), Duration::from_millis(40), Duration::from_millis(5));
    pool.set_max_running_workers(16);
    let registry = SolveJobRegistry::with_hooks(pool, gate.launcher(), Arc::new(|job: Box<dyn FnOnce() + Send + 'static>| std::thread::Builder::new().spawn(job).map(drop)));
    (registry, gate)
}

fn retained(registry: &SolveJobRegistry, gate: &Gate, text: &str) -> String {
    let started = gate.started_count();
    let receipt = registry.start(MODEL, None).unwrap_or_else(|e| panic!("{e:?}"));
    gate.wait_started(started + 1);
    gate.release(text);
    let status = wait_terminal(registry, &receipt.job_id);
    assert_eq!(status.state, "completed", "{status:?}");
    receipt.job_id
}

fn args(job_id: &str, model: &str, standing: Value) -> Value {
    json!({"jobId": job_id, "modelJson": model, "standing": standing})
}
fn body(args: &Value) -> InvokeBody {
    InvokeBody::Raw(serde_json::to_vec(args).unwrap())
}
fn read_summary(registry: &SolveJobRegistry, args: &Value) -> Result<Vec<u8>, CommandError<String>> {
    let (raw, lease) = reserve(registry, &body(args), &headers(Some(INGRESS_ARGS_JSON_V1)))?;
    run(registry.clone(), raw, lease)
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

const DEMO: &str = "fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json";
const DEMO_MODEL: &str = "fixtures/product_preview/invented_demo_model.json";
fn eligible() -> Value {
    json!({"eligible": true, "findings": []})
}

/// The command's bytes are `build`'s for every case whose standing states
/// findings (or none), read from a real retained job; the result stays
/// retained and the command's own leases are released.
#[test]
fn the_command_reads_each_case_from_a_retained_job() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let mut read = 0;
    for case in cases() {
        let standing = match &case.standing {
            StandingSpec::Health { .. } => continue,
            StandingSpec::None => Value::Null,
            StandingSpec::Findings { eligible, findings } => json!({"eligible": eligible, "findings": findings}),
        };
        let job = retained(&registry, &gate, &case.source);
        let held = registry.pool().reserved_total();
        let mut want = built(&case);
        // The job id is the command's; the case's name stands for it in `built`.
        let got = read_summary(&registry, &args(&job, &case.model, standing)).unwrap_or_else(|e| panic!("{}: {e:?}", case.name));
        let decoded = decode(&got).unwrap();
        assert_eq!(decoded.header["job_id"], json!(job));
        let mut header = decoded.header.clone();
        header["job_id"] = json!(case.name);
        let want_decoded = decode(&want).unwrap();
        assert_eq!(header, want_decoded.header, "{}", case.name);
        assert_eq!(decoded.planes.len(), want_decoded.planes.len());
        for ((name, got), (_, want)) in decoded.planes.iter().zip(&want_decoded.planes) {
            assert!(got.iter().zip(want).all(|(a, b)| a.to_bits() == b.to_bits()) && got.len() == want.len(), "{}: {name}", case.name);
        }
        want.clear();
        assert_eq!(registry.pool().reserved_total(), held, "{}", case.name);
        assert!(registry.status(&job).unwrap().result_available);
        registry.dispose_result(&job).unwrap();
        read += 1;
    }
    assert!(read >= 20, "{read}");
    assert_eq!(registry.pool().reserved_total(), 0);
}

#[test]
fn the_summary_needs_a_known_job_and_well_formed_args() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let model = read_text(DEMO_MODEL);
    assert!(invalid_message(read_summary(&registry, &args("job-404", &model, Value::Null)).unwrap_err()).starts_with("unknown solve job"));
    let job = retained(&registry, &gate, &read_text(DEMO));
    // The standing is null or {eligible, findings}; the model text is required.
    for bad in [json!({"jobId": job, "standing": null}), args(&job, &model, json!({"eligible": "yes", "findings": []})), args(&job, &model, json!({"eligible": true}))] {
        let message = invalid_message(read_summary(&registry, &bad).unwrap_err());
        assert!(message.contains(COMMAND), "{message}");
    }
    match read_summary(&registry, &args(&job, "{", eligible())).unwrap_err() {
        CommandError::Command(text) => assert_eq!(text, "WASM-ENGINE-INPUT-JSON-INVALID: model is not valid JSON: EOF while parsing an object at line 1 column 1"),
        other => panic!("{other:?}"),
    }
    match reserve(&registry, &body(&args(&job, &model, Value::Null)), &headers(None)) {
        Err(CommandError::Ingress(SolveJobError::InvalidRequest { message })) => assert!(message.starts_with("INGRESS-HEADER-REQUIRED"), "{message}"),
        other => panic!("{:?}", other.map(|_| ())),
    }
    // A delivered result is no longer held natively.
    registry.take_result(&job).unwrap();
    let message = invalid_message(read_summary(&registry, &args(&job, &model, Value::Null)).unwrap_err());
    assert!(message.starts_with("PREVIEW-SOLVE-RESULT-ALREADY-DELIVERED"), "{message}");
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// The parse lease is admitted before the parse allocates: refused by name,
/// nothing near the parse allocated, and the projection never reached.
#[test]
fn a_refused_parse_lease_allocates_nothing_and_never_reaches_the_projection() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let text = read_text(DEMO);
    let model = read_text(DEMO_MODEL);
    let job = retained(&registry, &gate, &text);
    let materialization = ingress::prescan(text.as_bytes()).unwrap().materialization_bytes;
    let held = gate::hold(&registry, "read", true);
    let (raw, lease) = reserve(&registry, &body(&args(&job, &model, eligible())), &headers(Some(INGRESS_ARGS_JSON_V1))).unwrap();
    host.set(|s| s.headroom_bytes = Some(materialization / 2));
    let handle = registry.clone();
    let (outcome, measured) = ingress::counting::measure(|| run(handle, raw, lease));
    assert!(matches!(refusal(outcome.unwrap_err()), ResourceRefusal::HeadroomInsufficient { .. }));
    assert_eq!(held.0.entered(), 0, "the projection was reached");
    assert!(measured.peak_priced < 64 * 1024 && measured.peak_priced * 100 < materialization, "{measured:?} against {materialization}");
    host.set(|s| s.pressure = PressureLevel::Warning);
    host.set(|s| s.headroom_bytes = Some(64 * crate::memory_pool::test_host::GIB));
    assert!(matches!(refusal(read_summary(&registry, &args(&job, &model, eligible())).unwrap_err()), ResourceRefusal::PressureElevated { .. }));
    host.set(|s| s.pressure = PressureLevel::Normal);
    assert!(read_summary(&registry, &args(&job, &model, eligible())).is_ok());
    assert_eq!(held.0.entered(), 1);
    drop(held);
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

/// Per-row combination evidence is counted from the borrowed source, never
/// cloned into a temporary Value tree while the source parse is held.
#[test]
fn combination_evidence_is_filtered_before_it_is_copied() {
    let source = |payload: &str| json!({
        "contract_evidence": {"other": {"status": "kept"}, "exact_combinations": [{
            "combination_id": "comb:1", "status": "kept",
            "rows": vec![json!({"payload": payload}); 128],
            "recoveries": vec![json!({"payload": payload}); 128],
            "missing_sources": vec![json!({"payload": payload}); 128],
        }]},
    });
    let small = source("row");
    let large = source(&"x".repeat(4096));
    let expected = envelope(&small);
    let (projected, allocations) = ingress::counting::measure(|| envelope(&large));
    assert_eq!(projected, expected);
    assert_eq!(projected.2[0], json!({"combination_id": "comb:1", "rows": 128, "recoveries": 128, "missing_sources": 128}));
    assert!(allocations.peak_priced < 64 * 1024, "{allocations:?}");
    assert_eq!(large["contract_evidence"]["exact_combinations"][0]["rows"][0]["payload"].as_str().unwrap().len(), 4096);
}

/// The response's lease is asked for its exact length after the counting
/// pass and before the response is allocated; a refusal allocates nothing
/// of the planes' size.
#[test]
fn the_response_is_admitted_at_its_exact_length_before_it_is_allocated() {
    let (source, model, health) = rack(10, 18);
    let standing = Standing { eligible: true, health: tables::health_view(&health) };
    let mut asked = None;
    let (bytes, ()) = build("rack", &source, &model, Some(&standing), &health, |length| {
        asked = Some(length);
        Ok::<(), ()>(())
    })
    .unwrap();
    assert_eq!(asked, Some(bytes.len() as u64));
    let planes = decode(&bytes).unwrap().header["planes_bytes"].as_u64().unwrap();
    let (refused, measured) = ingress::counting::measure(|| build("rack", &source, &model, Some(&standing), &health, |_| Err::<(), _>("refused")));
    assert_eq!(refused.unwrap_err(), "refused");
    // Before admission the working set is the tables' and header's, bounded by
    // entities and sets: the same members with 18 times the station rows (the
    // planes' rows) hold almost the same, far below the planes' size.
    let (few, ..) = rack(10, 1);
    let (_, few_measured) = ingress::counting::measure(|| build("rack", &few, &model, Some(&standing), &health, |_| Err::<(), _>("refused")));
    assert!(measured.peak_priced < few_measured.peak_priced + planes / 4, "{measured:?} against {few_measured:?} and planes of {planes} bytes");
    let (_, admitted) = ingress::counting::measure(|| build("rack", &source, &model, Some(&standing), &health, |_| Ok::<(), ()>(())));
    assert!(admitted.peak_priced >= bytes.len() as u64, "{admitted:?}");
}

/// The take is refused busy while a summary reads the result, and a
/// disposal during a read leaves the read whole; the bytes and their lease
/// go together when the read ends.
#[test]
fn custody_holds_while_a_summary_reads() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let text = read_text(DEMO);
    let model = read_text(DEMO_MODEL);
    let job = retained(&registry, &gate, &text);
    let want = read_summary(&registry, &args(&job, &model, eligible())).unwrap();
    let result_lease = registry.pool().reserved_total();
    for dispose in [false, true] {
        let held = gate::hold(&registry, "read", false);
        let (registry2, job2, model2) = (registry.clone(), job.clone(), model.clone());
        let reader = std::thread::spawn(move || read_summary(&registry2, &args(&job2, &model2, eligible())));
        assert!(held.0.wait_entered(1, Duration::from_secs(30)));
        if dispose {
            let receipt = registry.dispose_result(&job).unwrap();
            assert!(receipt.released);
            assert!(!registry.status(&job).unwrap().result_available);
        } else {
            let message = match registry.take_result(&job).unwrap_err() {
                SolveJobError::InvalidRequest { message } => message,
                other => panic!("{other:?}"),
            };
            assert!(message.starts_with("PREVIEW-SOLVE-RESULT-BUSY"), "{message}");
        }
        held.0.open();
        let got = reader.join().unwrap().unwrap();
        let (mut a, b) = (decode(&got).unwrap(), decode(&want).unwrap());
        a.header["job_id"] = b.header["job_id"].clone();
        assert_eq!(a.header, b.header);
        drop(held);
        if !dispose {
            assert_eq!(registry.pool().reserved_total(), result_lease);
        }
    }
    // The disposed result's bytes and lease went when the read ended.
    assert_eq!(registry.pool().reserved_total(), 0);
    let message = invalid_message(read_summary(&registry, &args(&job, &model, eligible())).unwrap_err());
    assert!(message.starts_with("PREVIEW-SOLVE-RESULT-UNAVAILABLE"), "{message}");
}

fn runtime_workers() -> usize {
    let workers = tauri::async_runtime::handle().inner().metrics().num_workers();
    assert!(workers > 0 && workers * 2 + 2 <= 512, "{workers} runtime workers");
    workers
}

/// Admission and projection run in a blocking task: more summaries than the
/// runtime has workers are held inside it at once, at each stage, and
/// another async command still completes.
#[test]
fn admission_and_projection_run_off_the_async_executor() {
    let host = ScriptedHost::normal();
    let (registry, gate) = gated(&host);
    let job = retained(&registry, &gate, &read_text(DEMO));
    // A model text of 2 MiB: admission prescans and parses it.
    let model = read_text(DEMO_MODEL);
    let padded = format!("{}, \"pad\": \"{}\"}}", model.trim_end().trim_end_matches('}'), "x".repeat(2 << 20));
    let commands = runtime_workers() * 2 + 2;
    for stage in ["admit", "read"] {
        let held = gate::hold(&registry, stage, false);
        let handles: Vec<_> = (0..commands)
            .map(|_| {
                let (registry, body) = (registry.clone(), body(&args(&job, &padded, eligible())));
                tauri::async_runtime::spawn(async move { summary(&registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1))).await })
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
            assert!(decode(&tauri::async_runtime::block_on(handle).unwrap().unwrap()).is_ok(), "{stage}");
        }
        drop(held);
    }
    registry.dispose_result(&job).unwrap();
    assert_eq!(registry.pool().reserved_total(), 0);
}

// ---------------------------------------------------------------------------
// JavaScript's readings
// ---------------------------------------------------------------------------

#[test]
fn numbers_and_strings_read_as_javascript_reads_them() {
    for (x, text) in [
        (0.0, "0"),
        (-0.0, "0"),
        (1.0, "1"),
        (-2.5, "-2.5"),
        (0.1, "0.1"),
        (1e21, "1e+21"),
        (1e20, "100000000000000000000"),
        (1.5e-7, "1.5e-7"),
        (1e-6, "0.000001"),
        (123456789.125, "123456789.125"),
        (5e-324, "5e-324"),
        (1.7976931348623157e308, "1.7976931348623157e+308"),
        (2f64.powi(53) + 2.0, "9007199254740994"),
    ] {
        assert_eq!(tables::js_number(x), text, "{x:e}");
    }
    // serde_json reads `-0` as negative zero, as JSON.parse does.
    let zero: Value = serde_json::from_str("-0").unwrap();
    assert_eq!(zero.as_f64().unwrap().to_bits(), (-0.0f64).to_bits());
    // UTF-16 order puts a supplementary character before U+FFFD; code points do not.
    assert_eq!(tables::utf16_cmp("\u{1F600}", "\u{FFFD}"), std::cmp::Ordering::Less);
    assert!("\u{1F600}" > "\u{FFFD}");
}


