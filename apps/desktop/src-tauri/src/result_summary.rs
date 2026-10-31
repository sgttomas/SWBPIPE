//! C1 native custody, slice 2 (DEL-04-05 `c1-native-custody-plan.md`;
//! design `result-volume-design.md` §3.1, §4.1): S, the summary of a
//! retained solve result, built in this process from the held bytes, so the
//! webview's panels and viewport can later read it without holding the
//! envelope (the switch is slice 4; nothing reads S yet).
//!
//! - **The command** `read_preview_mechanics_job_summary`, raw args body
//!   `{jobId, modelJson, standing}`: `modelJson` is the model text the panels
//!   show (the builders' `model`); `standing` is `null` (no reader standing)
//!   or `{eligible, findings}`, the reader's numerical standing and findings
//!   the session holds for the result (`resultsSessionState.ts`). The reply
//!   is one binary response: S's JSON header and its little-endian f64
//!   planes (`result_planes`).
//! - **What S carries.** The envelope's identity, status, summary, numerical
//!   quality, formulation basis, professional boundary and diagnostics; its
//!   `contract_evidence` without the per-row combination evidence
//!   (`exact_combinations[].rows`, `.recoveries`, `.missing_sources`, counted
//!   in `combination_evidence_counts`; design §3.5); the names of any other
//!   top-level member; the row count and distinct units; the numerical health
//!   (`operation_applier::numerical_health_json` with the standing's
//!   findings, or `null` without a standing); the tables of
//!   `result_summary_tables`; and the planes. S's JSON part grows with the
//!   model's entities and the result's sets, not with its station rows:
//!   those are plane cells.
//! - **Admission and memory** (plan §8). Every lease is pool-admitted before
//!   what it covers is allocated, and none exceeds the per-invocation budget
//!   or, whatever budget is configured, the owner-adopted 12 GiB default:
//!   - the args lease, as `native_verdict`'s: the raw body's copy, grown by
//!     the args' admission bound and by the model text's priced parse;
//!   - a diagnostic lease for each text parsed to find a refusal
//!     (`native_verdict::prescan_in_order`);
//!   - the parse lease ([`SolveJobRegistry::acquire_result_parse`]) of the
//!     held text's `Value` materialization M, the prescan's bound. It covers
//!     the numerical health's parse of the held text and then S's own parse,
//!     one after the other: `numerical_health_json` takes the text, and its
//!     `&Value` entry is private to the engine crate, so S cannot hand it the
//!     same parse (a desktop-only C1 cannot change that API);
//!   - the response lease (`acquire_result_parse` again, its own cap) of the
//!     response's exact length, admitted after the counting pass and before
//!     the response is allocated; the planes are then written into it in place.
//! - **Off the executor.** As `native_verdict`: on the async runtime only the
//!   header check, the copy's reservation and the copy; everything else in a
//!   blocking task.
//! - **Residuals, unpriced** (for slice 7's measurement, plan §7 item 3): the
//!   numerical health's working memory; the tables' and the value table's
//!   `Value`s and the header text (bounded by entities × sets, not rows); the
//!   ids the intensified-bending rows name and their rows' ordinals; and
//!   Tauri's copies of the response after the command returns.
use crate::command_ingress::{self, ArgsDecoding, CommandError};
use crate::ingress;
use crate::memory_pool::Lease;
use crate::native_verdict;
use crate::result_planes;
use crate::result_summary_tables::{self as tables, Standing};
use crate::solve_jobs::SolveJobRegistry;
use open_pipe_stress_result_export::{reader_verdict as rv, semantic_contract};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use tauri::http::HeaderMap;
use tauri::ipc::InvokeBody;

/// The command's name, as `command_ingress` reports it.
pub(crate) const COMMAND: &str = "read_preview_mechanics_job_summary";
/// The blocking read ended without an outcome (it panicked).
pub(crate) const SUMMARY_READ_FAILED: &str = "RESULT-SUMMARY-READ-FAILED";
pub(crate) const DOCUMENT_KIND: &str = "swbpipe.desktop.result_summary";
pub(crate) const VERSION: u64 = 1;

/// The envelope's top-level members S carries.
const ENVELOPE_MEMBERS: [&str; 13] = [
    "document_kind",
    "schema_version",
    "producer",
    "run_id",
    "model_ref",
    "accepted_model_state_mutated",
    "status",
    "summary",
    "numerical_quality",
    "formulation_basis",
    "professional_boundary",
    "diagnostics",
    "contract_evidence",
];
/// The per-row members of each `contract_evidence.exact_combinations` record.
const PER_ROW_COMBINATION_EVIDENCE: [&str; 3] = ["rows", "recoveries", "missing_sources"];

/// The caller's standing: the reader's numerical eligibility and findings.
#[derive(Debug, Deserialize)]
pub(crate) struct StandingArg {
    pub(crate) eligible: bool,
    pub(crate) findings: Vec<String>,
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
pub(crate) async fn summary(registry: &SolveJobRegistry, body: &InvokeBody, headers: &HeaderMap) -> Result<Vec<u8>, CommandError<String>> {
    let (raw, lease) = reserve(registry, body, headers)?;
    let registry = registry.clone();
    tauri::async_runtime::spawn_blocking(move || run(registry, raw, lease))
        .await
        .unwrap_or_else(|_| Err(CommandError::Command(SUMMARY_READ_FAILED.into())))
}

/// The blocking part: the arguments' admission, then S.
pub(crate) fn run(registry: SolveJobRegistry, raw: Vec<u8>, copy: Lease) -> Result<Vec<u8>, CommandError<String>> {
    #[cfg(test)]
    tests::gate::pass(tests::gate::key(&registry, "admit"));
    let copy_bytes = copy.bytes();
    let (mut args, mut lease) = command_ingress::admit_raw(&registry, &raw, COMMAND, ArgsDecoding::Moved, |bytes| {
        let mut lease = copy;
        lease.grow_to(copy_bytes.saturating_add(bytes)).map_err(|refusal| registry.refuse(refusal))?;
        Ok(lease)
    })?;
    drop(raw);
    let job_id: String = args.decode("jobId")?;
    let model_json: String = args.decode("modelJson")?;
    let standing: Option<StandingArg> = args.decode("standing")?;
    drop(args);
    let held = registry.held_result(&job_id)?;
    let texts = [("source", Some(held.text())), ("model", Some(model_json.as_str()))];
    let bounds = native_verdict::prescan_in_order(&registry, &texts)?;
    let (source_bound, _) = bounds[0].expect("the source is always prescanned");
    let model_bytes = command_ingress::text_parse_bytes(&model_json, ArgsDecoding::Moved)
        .map_err(|invalid| CommandError::Command(native_verdict::prescan_refusal("model", invalid)))?;
    // Capped as the args lease was admitted (`acquire_interactive_capped`).
    lease
        .grow_to(lease.bytes().saturating_add(model_bytes))
        .map_err(|refusal| CommandError::Ingress(registry.refuse(refusal)))?;
    let reading = registry.acquire_result_parse(source_bound.materialization_bytes)?;
    #[cfg(test)]
    tests::gate::pass(tests::gate::key(&registry, "read"));
    // The health parses the held text itself; its `Value` is gone before S's parse.
    let health = match &standing {
        Some(standing) => {
            let findings = serde_json::to_string(&standing.findings).expect("strings");
            let text = open_pipe_stress_operation_applier::numerical_health_json(held.text(), Some(&findings)).map_err(CommandError::Command)?;
            serde_json::from_str::<Value>(&text).map_err(|error| CommandError::Command(format!("{SUMMARY_READ_FAILED}: numerical health: {error}")))?
        }
        None => Value::Null,
    };
    let source = parse("source", held.text())?;
    let model = parse("model", &model_json)?;
    let standing = standing.map(|s| Standing { eligible: s.eligible, health: tables::health_view(&health) });
    let (response, written) = build(&job_id, &source, &model, standing.as_ref(), &health, |bytes| registry.acquire_result_parse(bytes))?;
    drop((written, source, model, reading, held, lease));
    Ok(response)
}

/// A text the prescan admitted, parsed; the reader's refusal text when
/// serde_json refuses it (a number beyond binary64).
fn parse(label: &str, text: &str) -> Result<Value, CommandError<String>> {
    serde_json::from_str(text).map_err(|error| CommandError::Command(format!("{}: {label} is not valid JSON: {error}", native_verdict::INPUT_JSON_INVALID)))
}

/// `countResultFamilies(result.results, semanticLabels(result))`: each row's
/// family from its signature under the source's table, read as
/// `reader_verdict::row_semantics` reads it (the table of `reader_verdict::route`,
/// the signature of `semantic_contract::signature_in`), with no outcome
/// materialized per row. A source without a table, a refused row, a row
/// without a signature, or one not a physical quantity is `other`.
pub(crate) fn family_counts(source: &Value) -> Value {
    let rows = tables::rows(source);
    let table = rv::route(source).ok().map(|(_, table)| table);
    tables::family_counts(
        rows.len(),
        rows.iter().map(|row| match table.map(|table| semantic_contract::signature_in(table, row)) {
            Some(Ok(Some(signature))) if signature["category"] == "physical_quantity" => signature["family"].as_str().unwrap_or("other"),
            _ => "other",
        }),
    )
}

/// The envelope members S carries, the per-row combination evidence counted
/// rather than copied, and the names of the members it leaves out.
fn envelope(source: &Value) -> (Value, Value, Value) {
    let mut carried = Map::new();
    let mut counts = Vec::new();
    for member in ENVELOPE_MEMBERS {
        let Some(value) = source.get(member) else { continue };
        if member != "contract_evidence" {
            carried.insert(member.into(), value.clone());
            continue;
        }
        let mut evidence = value.clone();
        if let Some(Value::Array(records)) = evidence.get_mut("exact_combinations") {
            for record in records.iter_mut().filter_map(Value::as_object_mut) {
                let mut count = Map::new();
                count.insert("combination_id".into(), record.get("combination_id").cloned().unwrap_or(Value::Null));
                for key in PER_ROW_COMBINATION_EVIDENCE {
                    if let Some(removed) = record.remove(key) {
                        count.insert(key.into(), json!(removed.as_array().map_or(0, Vec::len)));
                    }
                }
                counts.push(Value::Object(count));
            }
        }
        carried.insert(member.into(), evidence);
    }
    let omitted: Vec<&String> = source
        .as_object()
        .map(|o| o.keys().filter(|key| key.as_str() != "results" && !ENVELOPE_MEMBERS.contains(&key.as_str())).collect())
        .unwrap_or_default();
    (Value::Object(carried), json!(omitted), Value::Array(counts))
}

/// S's bytes for a parsed source and model, the caller's standing and the
/// numerical health, after `reserve` admits the response's exact length
/// (whose guard is returned with them).
pub(crate) fn build<R, E>(
    job_id: &str,
    source: &Value,
    model: &Value,
    standing: Option<&Standing>,
    health: &Value,
    reserve: impl FnOnce(u64) -> Result<R, E>,
) -> Result<(Vec<u8>, R), E> {
    let rows = tables::rows(source);
    let cases = tables::support_load_cases(model, source);
    let overlay_cases = tables::overlay_cases(model, source);
    let groups = tables::stress_groups(source, &cases, standing);
    let support_summaries = tables::support_load_summaries(model, source, &cases, standing);
    let selection = result_planes::select(source, &cases, &groups, standing);
    let (planes, planes_bytes) = result_planes::descriptors(&selection);
    let (carried, omitted, combination_counts) = envelope(source);
    let key_results = tables::key_results(model, source, &overlay_cases, &support_summaries);
    let units: Vec<Value> = {
        let mut seen = std::collections::HashSet::new();
        rows.iter().map(|row| row.get("unit")).filter(|unit| seen.insert(tables::js_key(*unit))).map(|unit| unit.cloned().unwrap_or(Value::Null)).collect()
    };
    let header = json!({
        "document_kind": DOCUMENT_KIND,
        "version": VERSION,
        "job_id": job_id,
        "envelope": carried,
        "omitted_members": omitted,
        "combination_evidence_counts": combination_counts,
        "row_count": rows.len(),
        "units": units,
        "numerical_health": health,
        "standing": standing.map(|s| json!({"eligible": s.eligible, "health_view": s.health.is_some()})),
        "tables": {
            "results_panel": {
                "family_counts": family_counts(source),
                "frame_counts": tables::frame_counts(source),
                "navigator_cases": tables::navigator_cases(source),
            },
            "support_load_cases": cases.iter().map(|case| {
                json!({"key": case.key, "basis": case.basis, "label": case.label, "envelope": case.envelope})
            }).collect::<Vec<_>>(),
            "overlay_cases": overlay_cases,
            "displacement_extremes": tables::displacement_extremes(model, source, &cases, standing),
            "support_load_summaries": support_summaries,
            "stress_ratio": tables::stress_ratio(source, &groups),
            "component_bodies": tables::component_bodies(source),
            "key_results": key_results,
            "withheld_support_state_rows": tables::withheld_support_state_rows(source, standing),
            "colour_quantities": tables::colour_quantities(model, source),
        },
        "display": {"rows": selection.display_count(), "source_refs": selection.source_count()},
        "stress": {"rows": selection.stress_count()},
        "values": selection.values.as_slice(),
        "planes": planes,
        "planes_bytes": planes_bytes,
    });
    let header = serde_json::to_vec(&header).expect("S serializes");
    let len = result_planes::response_len(header.len(), planes_bytes);
    let guard = reserve(len as u64)?;
    let mut out = Vec::with_capacity(len);
    result_planes::write(&mut out, &header, source, &cases, &selection, standing);
    debug_assert!(out.len() == len);
    Ok((out, guard))
}

#[cfg(test)]
#[path = "result_summary_tests.rs"]
pub(crate) mod tests;
