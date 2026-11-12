//! The rendered report built by the host from the result it holds (DEL-08-01
//! Design `exact-route-report-sections.md` §3.6; increment 1A-i, the native
//! half). On the existing render path the webview takes a copy of the result,
//! builds the whole render input and resubmits the result text, and the host
//! re-derives every result-dependent member to check the webview's. Here the
//! webview sends a **frame**, the render input without those members, with the
//! job identity, the model (intake) text and the input manifest text; the host
//! reads the result it already holds natively (one parse of the held text,
//! never a copy of it) and fills the members with the functions that check
//! them. The filled input then takes the same admission as the webview's input
//! on the existing path (`report_render_exact.rs`) before the renderer.
//!
//! **The frame rule.** The frame carries what the session owns: the title,
//! the provenance records and values, the library references and findings,
//! the rule-pack reference and user-rule outcomes, the persistence reference,
//! the report context (its export context and `generated_at`), the composition,
//! and the members it takes from the session's run record and model (the
//! status disclosures, the audit references, the model hash, the input-manifest
//! reference, the load-case summary). A frame that carries a member the host
//! fills is refused by that member's path ([`FRAME_MEMBER_DERIVED`]); nothing
//! the frame carries is overwritten. The members the host fills:
//! - `result_rows`, `calculation_report.result_export_refs`,
//!   `calculation_report.model_input_summary.unit_display_summary`,
//!   `report_context.result_coordinate_systems` and
//!   `calculation_report.report_section_refs`;
//! - the result's Diagnostics, before the frame's own entries in
//!   `report_sections.diagnostics` (a frame entry with a result diagnostic's
//!   code and affected object is refused, whatever else it carries; the
//!   admission admits only the session entries the host derives from the
//!   bound model, `report_session_diagnostics`);
//! - the exact or pressure-free Timoshenko limitation entries, after the
//!   frame's one technical-preview entry in `report_sections.limitations` (a
//!   frame with any other entry is refused);
//! - `exact_route` or `pf_timoshenko`, with the admission binding;
//! - `model_input_tables`, `P(M, C)` of the bound model and the frame's own
//!   composition.
//!
//! **The admission binding's derivative hash.** The webview reads it from the
//! Current result document it derives (`buildCurrentResultExport`). Here the
//! same document is derived natively: the analysis-run record is the native
//! builder's (`analysis_run_v03_build`, with the session's rule-check
//! aggregate and rule-pack exclusions), its base and origin the Rust mirrors of
//! the desktop's (`report_package::user_release`), and the document
//! `result_export::derivative::derive_document`. A record with a rule-pack
//! reference exclusion is not served yet ([`FILL_NOT_SERVED`]): the mirror of
//! the document's rule-pack diagnostics does not exist natively.
//!
//! **Size.** The derivation reserves [`DERIVATION_RESERVE_FACTOR`] times the
//! held text before any parse; when the invocation's budget cannot hold it, the
//! render is refused as a resource shortfall ([`check_fill_size`]). At present
//! that admits about 49 MB of result text, an estimate of present sizing (the
//! measured multiple with its margin, a 12 GiB budget with nothing else held),
//! not a guaranteed size. It is below the 64 MiB the webview's bounded copy
//! already admits: this command raises no size limit; it removes the
//! resubmitted request text and the webview's copy only.
//!
//! Routes: an exact-route (pressure-2, pressure-source-1) or pressure-free
//! Timoshenko result is filled; any other result in custody is refused
//! ([`FILL_NOT_SERVED`]). Standard claim fence applies (claims taxonomy per
//! DEC-081).

use crate::analysis_run_v03_build::{self as record_build, Exclusion, ManifestArg};
use crate::command_ingress::{Args, CommandError};
use crate::exact_source_binding::{self as binding, sections_mismatch, BoundSource, ExactRefusal};
use crate::memory_pool::{Lease, ResourceRefusal};
use crate::report_render_exact::{check_text_sizes, exact_render_growth_bytes, PF_TIMOSHENKO_SECTION};
use crate::solve_jobs::{sha256_in_place, HeldResult, SolveJobRegistry};
use open_pipe_stress_report_package::bindings::{
    expected_report_diagnostics, expected_report_result_refs, expected_report_result_rows, expected_unit_display_summary,
};
use open_pipe_stress_report_package::exact_sections::{expected_exact_limitations, expected_exact_route_core, is_exact_identity};
use open_pipe_stress_report_package::model_input::model_input_tables;
use open_pipe_stress_report_package::pf_timoshenko::{expected_limitation as expected_pf_limitation, is_pf_timoshenko_identity};
use open_pipe_stress_report_package::user_release::{current_received_origin, current_result_document_base, report_envelope, result_coordinate_systems};
use open_pipe_stress_result_export::derivative::{derive_document, digest};
use serde_json::{json, Map, Value};
use std::sync::Arc;

/// A frame that carries a member the host fills.
pub(crate) const FRAME_MEMBER_DERIVED: &str = "REPORT-RENDER-FRAME-MEMBER-DERIVED";
/// A frame without a member the fill writes into or copies from.
pub(crate) const FRAME_INCOMPLETE: &str = "REPORT-RENDER-FRAME-INCOMPLETE";
/// A result in custody, or a session basis, that the render from custody does
/// not serve yet.
pub(crate) const FILL_NOT_SERVED: &str = "REPORT-RENDER-FILL-NOT-SERVED";

/// The members the host fills whole, by path, as the refusal names them.
const FILLED_MEMBERS: [&[&str]; 8] = [
    &["result_rows"],
    &["model_input_tables"],
    &["exact_route"],
    &[PF_TIMOSHENKO_SECTION],
    &["calculation_report", "result_export_refs"],
    &["calculation_report", "report_section_refs"],
    &["calculation_report", "model_input_summary", "unit_display_summary"],
    &["report_context", "result_coordinate_systems"],
];

fn derived(path: &str) -> String {
    format!("{FRAME_MEMBER_DERIVED}: the frame carries {path}, which the host fills from the result in its custody.")
}

fn incomplete(path: &str) -> String {
    format!("{FRAME_INCOMPLETE}: the frame has no {path} of the render input's form.")
}

/// The frame rule (module docs): no member the host fills, by key (a null
/// counts), and exactly one limitation entry. Names the path only, never a
/// value. The frame's Diagnostics are the session's: the fill refuses one
/// that copies a result diagnostic, and the admission admits only those the
/// host derives (`report_session_diagnostics`).
pub(crate) fn check_frame(frame: &Value) -> Result<(), String> {
    for path in FILLED_MEMBERS {
        let (last, parents) = path.split_last().expect("a path");
        let parent = parents.iter().try_fold(frame, |value, key| value.get(*key));
        if parent.and_then(Value::as_object).is_some_and(|object| object.contains_key(*last)) {
            return Err(derived(&path.join(".")));
        }
    }
    let sections = frame.get("report_sections");
    match sections.and_then(|sections| sections.get("limitations")).and_then(Value::as_array).map(Vec::len) {
        Some(1) => Ok(()),
        Some(count) if count > 1 => Err(derived("report_sections.limitations[1]")),
        _ => Err(incomplete("report_sections.limitations technical-preview entry")),
    }
}

/// The render from custody's arguments (raw args body `{jobId, frame,
/// intakeText, inputManifestText, ruleCheckAggregate, rulePackExclusions}`):
/// no result text.
pub(crate) struct CustodyRenderRequest {
    pub(crate) job_id: String,
    pub(crate) frame: Value,
    pub(crate) intake: String,
    pub(crate) input_manifest: String,
    /// The session's rule-check aggregate, or none (the analysis-run record's
    /// argument of the same name).
    pub(crate) aggregate: Option<String>,
    /// The model's rule-pack references the solve left out of the manifest.
    pub(crate) exclusions: Vec<Exclusion>,
}

impl CustodyRenderRequest {
    pub(crate) fn decode(args: &mut Args) -> Result<Self, CommandError> {
        Ok(Self {
            job_id: args.decode("jobId")?,
            frame: args.value("frame")?,
            intake: args.decode("intakeText")?,
            input_manifest: args.decode("inputManifestText")?,
            aggregate: args.decode("ruleCheckAggregate")?,
            exclusions: args.decode::<Option<Vec<Exclusion>>>("rulePackExclusions")?.unwrap_or_default(),
        })
    }
}

/// The filled render input and what its admission reads: the parsed held
/// result and model, the custody's held share, the intake and manifest texts.
pub(crate) struct Filled {
    pub(crate) input: Value,
    pub(crate) bound: BoundSource,
    pub(crate) held: Arc<HeldResult>,
    pub(crate) intake: String,
    pub(crate) input_manifest: String,
}

fn refused(refusal: ExactRefusal) -> CommandError {
    CommandError::Command(refusal.message)
}

fn mismatch(section: &str) -> String {
    sections_mismatch(section).message
}

/// Which section the held result takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Section {
    Exact,
    PfTimoshenko,
}

/// The section of `source`'s identity, or [`FILL_NOT_SERVED`].
pub(crate) fn section_of(source: &Value) -> Result<Section, String> {
    match source.pointer("/producer/semantic_contract_id").and_then(Value::as_str) {
        Some(id) if is_exact_identity(id) => Ok(Section::Exact),
        Some(id) if is_pf_timoshenko_identity(id) => Ok(Section::PfTimoshenko),
        _ => Err(format!(
            "{FILL_NOT_SERVED}: the result in custody is not a pressure-2, pressure-source-1 or pressure-free Timoshenko result; the render from custody serves those only, and any other result renders through the existing render command."
        )),
    }
}

/// What the admission binding takes from the derivative and the manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Derivation {
    pub(crate) input_manifest_ref: String,
    pub(crate) input_manifest_sha256: String,
    pub(crate) derivative_sha256: String,
    /// The derived document's received-carrier digest (`digest(source)`).
    pub(crate) received_result_sha256: String,
}

/// The Current result document's binding fields, derived natively (module
/// docs): the analysis-run record from the held share and the parsed `source`
/// (the native builder's checks and refusals, with the session's
/// `aggregate` and `exclusions`), then `derive_document` on the desktop's base
/// and origin. Each binding field is read from the document and compared with
/// the value recomputed from the inputs, as `admitExactReportSource` compares
/// them; a difference refuses, naming the admission field.
#[allow(clippy::too_many_arguments)]
pub(crate) fn derive_binding(
    held: &HeldResult,
    source: &mut Value,
    model: &Value,
    model_sha256: &str,
    manifest_text: String,
    manifest: &Value,
    aggregate: Option<&str>,
    exclusions: &[Exclusion],
    reserve: &mut dyn FnMut(u64) -> Result<Option<crate::memory_pool::CommandLease>, CommandError<String>>,
) -> Result<(Derivation, String), CommandError> {
    if !exclusions.is_empty() {
        return Err(CommandError::Command(format!(
            "{FILL_NOT_SERVED}: the session's record names rule-pack reference exclusions; the derivative's rule-pack diagnostics are not derived natively yet, so this report renders through the existing render command."
        )));
    }
    let manifest_sha256 = crate::analysis_run_v03_build::hex(&sha256_in_place(manifest_text.as_bytes()));
    let model_ref = manifest.pointer("/model_basis/model_ref").and_then(Value::as_str).ok_or_else(|| CommandError::Command(mismatch("input_manifest.model_basis")))?;
    let evidence = ManifestArg {
        manifest_ref: crate::analysis_runs::input_manifest_ref(model_ref, &manifest_sha256),
        manifest_sha256: manifest_sha256.clone(),
        canonical_bytes: manifest_text,
    };
    record_build::verify_manifest(&evidence, Some(manifest)).map_err(CommandError::Command)?;
    let manifest_ref = evidence.manifest_ref["ref"].as_str().unwrap_or_default().to_string();
    let manifest_sha256_value = Value::String(manifest_sha256.clone());
    let aggregate = aggregate.map(|text| Value::String(text.to_string()));
    let mut load_basis = Value::Null;
    let mut run_record = {
        let built = record_build::preview_parsed(
            held,
            source,
            &evidence,
            &manifest_sha256_value,
            manifest,
            aggregate.as_ref(),
            exclusions,
            &mut load_basis,
            reserve,
        )?;
        record_build::header_record(&built)
    };
    // The record's producer diagnostics, as it carries them (the header leaves
    // the row-scale members out); with no exclusion they are all it carries.
    let diagnostics = source.get("diagnostics").and_then(Value::as_array).cloned().unwrap_or_default();
    run_record["analysis_run"]["diagnostics"] = Value::Array(diagnostics.into_iter().map(|item| json!({"source_annotation": item})).collect());
    let source: &Value = source;
    let base_failed = |detail: String| CommandError::Command(format!("{}: admission.derivative_sha256 ({detail})", binding::SECTIONS_MISMATCH));
    let origin = current_received_origin(model, source, &manifest_ref).map_err(base_failed)?;
    let base = current_result_document_base(model, source, &run_record, &manifest_ref, &manifest["solver_basis"]).map_err(base_failed)?;
    let document = derive_document(base, model, source, origin, None).map_err(base_failed)?;
    let envelope = &document["result_envelope"];
    let text = |pointer: &str| envelope.pointer(pointer).and_then(Value::as_str).unwrap_or_default();
    let run = source["run_id"].as_str().unwrap_or_default();
    let received = text("/reproducibility/source_origin_bindings/0/received_carrier_checksum/value").to_string();
    let derivative = text("/reproducibility/derivative_hash/value").to_string();
    let record_manifest_hash = run_record.pointer("/analysis_run/reproducibility/input_manifest_hashes/0/value").and_then(Value::as_str).unwrap_or_default();
    let lower_hex_64 = |value: &str| value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
    for (field, holds) in [
        ("admission.semantic_contract_id", text("/producer/semantic_contract_id") == source["producer"]["semantic_contract_id"].as_str().unwrap_or("\u{0}")),
        ("admission.run_ref", text("/run_ref/ref_id") == run && !run.is_empty()),
        ("admission.result_envelope_id", text("/envelope_id") == format!("result-envelope:{run}")),
        ("admission.received_result_sha256", lower_hex_64(&received)),
        ("admission.model_sha256", text("/reproducibility/model_hash/value") == model_sha256),
        ("admission.input_manifest_ref", text("/reproducibility/audit_manifest_ref/ref_id") == manifest_ref),
        ("admission.input_manifest_sha256", record_manifest_hash == manifest_sha256),
        ("admission.derivative_sha256", lower_hex_64(&derivative)),
    ] {
        if !holds {
            return Err(CommandError::Command(mismatch(field)));
        }
    }
    let manifest_text = evidence.canonical_bytes;
    Ok((Derivation { input_manifest_ref: manifest_ref, input_manifest_sha256: manifest_sha256, derivative_sha256: derivative, received_result_sha256: received }, manifest_text))
}

/// The custody facts and derivation the admission binding is written from.
pub(crate) struct FillBasis<'a> {
    pub(crate) source: &'a Value,
    pub(crate) model: &'a Value,
    pub(crate) model_sha256: &'a str,
    pub(crate) derivation: &'a Derivation,
    pub(crate) native_job_id: &'a str,
    pub(crate) received_bytes_sha256: &'a str,
}

fn object_at<'v>(value: &'v mut Value, path: &[&str]) -> Result<&'v mut Map<String, Value>, String> {
    let mut at = value;
    for key in path {
        at = at.get_mut(*key).ok_or_else(|| incomplete(&path.join(".")))?;
    }
    at.as_object_mut().ok_or_else(|| incomplete(&path.join(".")))
}

/// The fill (module docs): the frame, checked by [`check_frame`], with every
/// result-derived member written by the producer whose check the admission
/// then runs. A producer's refusal is the admission's own for that member
/// (`REPORT-EXACT-SECTIONS-MISMATCH: <member>`, or the model-input binding's).
pub(crate) fn fill(mut frame: Value, basis: &FillBasis<'_>) -> Result<Value, String> {
    check_frame(&frame)?;
    let FillBasis { source, model, model_sha256, derivation, native_job_id, received_bytes_sha256 } = *basis;
    let section = section_of(source)?;
    let provenance = frame
        .pointer("/calculation_report/provenance")
        .filter(|provenance| provenance.is_object())
        .cloned()
        .ok_or_else(|| incomplete("calculation_report.provenance"))?;
    let rows = expected_report_result_rows(source).map_err(|_| mismatch("result_rows"))?;
    let refs = expected_report_result_refs(source, &provenance).map_err(|_| mismatch("result_export_refs"))?;
    let units = model.pointer("/project/units").filter(|units| units.is_object()).ok_or_else(|| mismatch("unit_display_summary"))?;
    let unit_display = expected_unit_display_summary(units, source);
    let coordinate_systems = result_coordinate_systems(source).map_err(|_| mismatch("report_context.result_coordinate_systems"))?;
    let diagnostics = expected_report_diagnostics(source).map_err(|_| mismatch("diagnostics"))?;
    let run = source["run_id"].as_str().filter(|run| !run.is_empty()).ok_or_else(|| mismatch("admission.run_ref"))?.to_string();

    // The admission binding: the identity and run from the source, the digests
    // of the source and model, the manifest's identity, the derivative's hash
    // and the two custody facts.
    let identity = source["producer"]["semantic_contract_id"].as_str().unwrap_or_default().to_string();
    let received = digest(source).map_err(|_| mismatch("admission.received_result_sha256"))?;
    if received != derivation.received_result_sha256 {
        return Err(mismatch("admission.received_result_sha256"));
    }
    // The renderer's `ExactAdmissionBinding`, field for field (written as a
    // value: the shell names no renderer type, `user_release_reach_scan`).
    let admission = json!({
        "semantic_contract_id": identity,
        "run_ref": run,
        "result_envelope_id": format!("result-envelope:{run}"),
        "received_result_sha256": received,
        "model_sha256": model_sha256,
        "input_manifest_ref": derivation.input_manifest_ref,
        "input_manifest_sha256": derivation.input_manifest_sha256,
        "derivative_sha256": derivation.derivative_sha256,
        "native_job_id": native_job_id,
        "received_bytes_sha256": received_bytes_sha256,
    });

    let sections = object_at(&mut frame, &["report_sections"])?;
    // The result's Diagnostics first, with the session's provenance, then the frame's own.
    let own = match sections.remove("diagnostics") {
        None | Some(Value::Null) => Vec::new(),
        Some(Value::Array(items)) => items,
        Some(_) => return Err(incomplete("report_sections.diagnostics")),
    };
    // A frame entry with a result diagnostic's code and affected object is
    // refused whatever else it carries (review of f3d02469c, finding 2); the
    // admission then admits only the session entries the host derives.
    if let Some(index) = crate::report_session_diagnostics::first_copy_of_a_result_diagnostic(&own, &diagnostics) {
        return Err(derived(&format!("report_sections.diagnostics[{index}]")));
    }
    let mut entries: Vec<Value> = diagnostics
        .into_iter()
        .map(|mut entry| {
            entry["provenance"] = provenance.clone();
            entry
        })
        .collect();
    entries.extend(own);
    sections.insert("diagnostics".to_string(), Value::Array(entries));
    // The route's limitation entries after the technical-preview entry, with its effect and provenance.
    let limitations = sections.get_mut("limitations").and_then(Value::as_array_mut).ok_or_else(|| incomplete("report_sections.limitations"))?;
    let preview = limitations[0].clone();
    let shared = |entry: Value| -> Value {
        let mut entry = entry;
        entry["effect"] = preview["effect"].clone();
        entry["provenance"] = preview["provenance"].clone();
        entry
    };
    let route_section = match section {
        Section::Exact => {
            let core = expected_exact_route_core(source).map_err(|_| mismatch("exact_route"))?;
            let expected = expected_exact_limitations(source, &core.run_ref).map_err(|_| mismatch("limitations"))?;
            for entry in expected {
                limitations.push(shared(json!({
                    "limitation_id": entry.limitation_id,
                    "source": entry.source,
                    "affected_scope": entry.affected_scope,
                    "statement": entry.statement,
                })));
            }
            // The renderer's `ExactRouteSection`, field for field.
            let headlines = serde_json::to_value(&core.headlines).map_err(|_| mismatch("headlines"))?;
            ("exact_route", json!({
                "semantic_contract_id": core.semantic_contract_id,
                "admission": admission,
                "load_reference_states": core.load_reference_states,
                "headlines": headlines,
            }))
        }
        Section::PfTimoshenko => {
            let entry = expected_pf_limitation(source, &run).map_err(|_| mismatch("limitations"))?;
            limitations.push(shared(entry));
            (PF_TIMOSHENKO_SECTION, json!({"semantic_contract_id": identity, "admission": admission}))
        }
    };
    let section_id = sections.get("report_section_id").and_then(Value::as_str).ok_or_else(|| incomplete("report_sections.report_section_id"))?.to_string();

    // The sections are complete: their checksum, as the desktop takes it.
    let sections_checksum = digest(&frame["report_sections"]).map_err(|_| mismatch("report_section_refs"))?;
    let section_ref = report_envelope(
        json!({"ref_type": "report_sections", "ref_id": section_id}),
        "schemas/report_sections.schema.yaml",
        Some(&sections_checksum),
        &provenance,
    );
    let calculation = object_at(&mut frame, &["calculation_report"])?;
    calculation.insert("result_export_refs".to_string(), Value::Array(refs));
    calculation.insert("report_section_refs".to_string(), Value::Array(vec![section_ref]));
    object_at(&mut frame, &["calculation_report", "model_input_summary"])?.insert("unit_display_summary".to_string(), unit_display);
    object_at(&mut frame, &["report_context"])?.insert("result_coordinate_systems".to_string(), json!(coordinate_systems));
    let tables = model_input_tables(model, frame.get("report_composition").filter(|value| !value.is_null()))?;
    let input = frame.as_object_mut().ok_or_else(|| incomplete("root object"))?;
    input.insert("result_rows".to_string(), Value::Array(rows));
    input.insert(route_section.0.to_string(), route_section.1);
    if let Some(tables) = tables {
        input.insert("model_input_tables".to_string(), tables);
    }
    Ok(frame)
}

/// The derivation's reservation, as a multiple of the held result text's bytes
/// (review of f3d02469c, finding 1). [`derive_binding`] holds at once the
/// analysis-run record, the Current result document (`derive_document`, which
/// copies the source evidence and builds per-row values, annotations,
/// accounting and witnesses), the document's clone for validation and the
/// canonical strings of its digests. Its peak, priced as the pool prices
/// (`ingress::allocation_envelope` per allocation), measured with the counting
/// allocator: 108 to 192 times the held text on the nine committed references
/// (`report_native_fill_tests`) and 174 times on an 82-node nonlinear rack
/// result of 10.7 MB solved at this head. 240 is the largest of those, 192,
/// with a quarter added. A measured provision, not a proven bound; the
/// regression `the_derivation_stays_within_its_reservation_on_every_committed_reference`
/// fails when a committed reference exceeds it.
pub(crate) const DERIVATION_RESERVE_FACTOR: u64 = 240;

/// The derivation's reservation for a held text of `text_bytes`, when the
/// invocation's `budget_bytes` holds it beside the `held_bytes` the command
/// already holds or is about to take. Otherwise the pool's own refusal for a
/// growth above the budget, `ingress_exceeds_budget` with the bytes required
/// in all: a capability shortfall of this invocation, in the resource-refusal
/// family every native command uses (the desktop reads it as
/// `RESOURCE_UNAVAILABLE`), never a statement about the result's standing or
/// binding. Decided before any parse: nothing result-sized is allocated
/// before it.
pub(crate) fn check_fill_size(text_bytes: u64, held_bytes: u64, budget_bytes: u64) -> Result<u64, ResourceRefusal> {
    let reserved = text_bytes.saturating_mul(DERIVATION_RESERVE_FACTOR);
    let required_bytes = held_bytes.saturating_add(reserved);
    if required_bytes > budget_bytes {
        return Err(ResourceRefusal::IngressExceedsBudget { required_bytes, budget_bytes });
    }
    Ok(reserved)
}

/// The fill from custody, before the admission (`report_render_exact.rs`): the
/// frame rule; the job's custody digests (the existing path's step 1); the
/// size caps; the model text against the custody's intake digest; the lease
/// grown by the existing render's priced amounts and the derivation's
/// reservation ([`check_fill_size`]), priced on the held text, before any
/// parse; one parse of the held text with the native standing; the
/// manifest's parse; the native derivation; then [`fill`].
pub(crate) fn fill_from_custody(request: CustodyRenderRequest, registry: &SolveJobRegistry, lease: &mut Lease) -> Result<Filled, CommandError> {
    let CustodyRenderRequest { job_id, frame, intake, input_manifest, aggregate, exclusions } = request;
    check_frame(&frame).map_err(CommandError::Command)?;
    let digests = binding::check_custody(&json!({"native_job_id": job_id}), &|job| registry.custody_binding(job)).map_err(refused)?;
    let held = registry.held_result(&job_id)?;
    check_text_sizes(Some(held.text()), Some(&intake), Some(&input_manifest)).map_err(CommandError::Command)?;
    if sha256_in_place(intake.as_bytes()) != digests.intake_sha256 {
        return Err(refused(binding::not_delivered("model")));
    }
    let growth = exact_render_growth_bytes(held.text(), &intake, &input_manifest).map_err(CommandError::Command)?;
    // The derivation's reservation (review of f3d02469c, finding 1), refused as
    // a resource shortfall before any parse when the invocation's budget cannot
    // hold it, then taken with the render's growth in one step before any parse.
    let reserved = check_fill_size(held.text().len() as u64, lease.bytes().saturating_add(growth), lease.budget_bytes())
        .map_err(|refusal| CommandError::Ingress(registry.refuse(refusal)))?;
    lease
        .grow_to(lease.bytes().saturating_add(growth).saturating_add(reserved))
        .map_err(|refusal| CommandError::Ingress(registry.refuse(refusal)))?;
    // Steps 3 and 4 of the existing binding, on the held text (borrowed, never
    // copied): its one parse. Step 5 runs in the admission, on the filled binding.
    let mut bound = binding::read_eligible_source(&digests, held.text(), &intake).map_err(refused)?;
    let section = section_of(&bound.source).map_err(CommandError::Command)?;
    let manifest = crate::report_package_save::parse_canonical("input_manifest", &input_manifest)
        .map_err(|_| refused(sections_mismatch("admission.input_manifest_sha256")))?;
    // The record builder's row fingerprints are admitted under the command's lease.
    let mut reserve = |bytes: u64| -> Result<Option<crate::memory_pool::CommandLease>, CommandError<String>> {
        lease.grow_to(lease.bytes().saturating_add(bytes)).map_err(|refusal| CommandError::Ingress(registry.refuse(refusal)))?;
        Ok(None)
    };
    let (derivation, input_manifest) = derive_binding(
        &held,
        &mut bound.source,
        &bound.model,
        &bound.model_sha256,
        input_manifest,
        &manifest,
        aggregate.as_deref(),
        &exclusions,
        &mut reserve,
    )?;
    drop(manifest);
    let received_bytes = crate::analysis_run_v03_build::hex(&digests.result_sha256);
    let basis = FillBasis {
        source: &bound.source,
        model: &bound.model,
        model_sha256: &bound.model_sha256,
        derivation: &derivation,
        native_job_id: &job_id,
        received_bytes_sha256: &received_bytes,
    };
    let input = fill(frame, &basis).map_err(CommandError::Command)?;
    debug_assert!(matches!(
        (section, input.get("exact_route").is_some(), input.get(PF_TIMOSHENKO_SECTION).is_some()),
        (Section::Exact, true, false) | (Section::PfTimoshenko, false, true)
    ));
    Ok(Filled { input, bound, held, intake, input_manifest })
}
