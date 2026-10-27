//! The user-released report package (W-18b slice (r2c); DEL-08-01 Design
//! `report-user-release.md` §2.1, §2.3, §2.8, §2.9, §2.10, §4, §6): the one
//! admitting entry `user_release::assemble_derived`, the general entries'
//! refusal of every user-release marker, the writer's manifest record, content
//! binding, notes and member rule, and the reader's checks. The basis is the
//! committed derivation vectors (`fixtures/reports/invented/user_release_derivation_vectors.json`),
//! whose expected request is a coherent, pressure-free, desktop-origin
//! user-released request with an invented record. Nothing here records a user
//! release; the reader's checks are equality checks (§2.9). Standard claim
//! fence applies (claims taxonomy per DEC-081).

mod common;
#[path = "common/package_basis.rs"]
mod package_basis;

use common::Fixture;
use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_report_package::bindings::{DESKTOP_PROFILE, RUNNER_PROFILE};
use open_pipe_stress_report_package::document_form::ROUTE_NOT_ADMITTED;
use open_pipe_stress_report_package::exact_sections::EXACT_RUNNER_PROFILE;
use open_pipe_stress_report_package::user_release::{
    assemble_derived, derive, SoftwareIdentity, UserReleaseBasis, UserReleaseInputs, CONTENT_NOT_DERIVED, EXACT_ROUTE_NOT_ADMITTED,
    PACKAGE_NOT_ADMITTED, PACKAGE_UNBOUND, USER_RELEASED_CONTEXT, USER_RELEASED_DESKTOP_PROFILE,
};
use open_pipe_stress_report_package::wire::{assemble_wire_request, decode_report_package_request, LinkedSolverIdentity};
use open_pipe_stress_report_package::{
    assemble_report_package_container, read_report_package_container, PackageReadRefusal, ReportPackageContainerOutcome,
    ROLE_STATE_COMPARISON_HANDOFF,
};
use open_pipe_stress_report_renderer::{
    escape_html, render_calculation_report, user_release_notice_for, ReportContext, ReportExportContext, UserReleaseRecord,
};
use open_pipe_stress_report_package::user_release::{current_received_origin, current_result_document_base};
use open_pipe_stress_result_export::derivative::{derive_document, digest};
use open_pipe_stress_result_export::semantic_contract::{PHYSICS_RETAINED_ID, PRESSURE_ID, PRESSURE_SOURCE_ID};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const VECTORS: &str = include_str!("../../../../fixtures/reports/invented/user_release_derivation_vectors.json");
const MODEL: &str = include_str!("../../../../fixtures/model_operations/precision_connected_ui_model.json");
const SOURCE_FIXTURES: [(&str, &str); 2] = [
    ("fixtures/results/preview_physics_connected_sparse.json", include_str!("../../../../fixtures/results/preview_physics_connected_sparse.json")),
    ("fixtures/results/preview_physics_connected_dense.json", include_str!("../../../../fixtures/results/preview_physics_connected_dense.json")),
];
const RECORD_FIXTURE: &str = include_str!("../../../../fixtures/reports/invented/user_release_record.json");

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|byte| format!("{byte:02x}")).collect()
}

struct Case {
    vector: Value,
    model: Value,
    source: Value,
    software: SoftwareIdentity,
    record: UserReleaseRecord,
}

impl Case {
    fn basis(&self) -> UserReleaseBasis<'_> {
        UserReleaseBasis {
            model: &self.model,
            source: &self.source,
            solver_mode: self.vector["solver_mode"].as_str().unwrap(),
            run_record: &self.vector["run_record"],
            software: &self.software,
            record: &self.record,
        }
    }

    fn inputs(&self) -> UserReleaseInputs<'_> {
        UserReleaseInputs {
            model: &self.model,
            solver_mode: self.vector["solver_mode"].as_str().unwrap(),
            run_record: &self.vector["run_record"],
            software: &self.software,
            record: &self.record,
        }
    }

    fn input_manifest(&self) -> &Value {
        &self.vector["input_manifest"]
    }

    /// The coherent user-released request: the derivation of the vector's
    /// basis, which equals the desktop's request (`user_release_derivation`).
    fn request(&self) -> Value {
        derive(&self.basis()).unwrap().request
    }

    fn assemble(&self, request: Value) -> Result<ReportPackageContainerOutcome, String> {
        assemble_derived(request, &linked(), &self.inputs(), self.input_manifest())
    }
}

fn cases() -> Vec<Case> {
    let file: Value = serde_json::from_str(VECTORS).unwrap();
    file["vectors"]
        .as_array()
        .unwrap()
        .iter()
        .map(|vector| {
            let source = SOURCE_FIXTURES.iter().find(|(name, _)| *name == vector["source_fixture"]).unwrap().1;
            Case {
                model: serde_json::from_str(MODEL).unwrap(),
                source: serde_json::from_str(source).unwrap(),
                software: SoftwareIdentity {
                    name: vector["software"]["name"].as_str().unwrap().to_string(),
                    version: vector["software"]["version"].as_str().unwrap().to_string(),
                },
                record: serde_json::from_value(vector["user_release"].clone()).unwrap(),
                vector: vector.clone(),
            }
        })
        .collect()
}

fn case() -> Case {
    cases().into_iter().next().unwrap()
}

fn linked() -> LinkedSolverIdentity<'static> {
    LinkedSolverIdentity {
        solver_name: "open_pipe_stress_product_physics",
        solver_version: "0.2.0",
        solver_build_ref: "open_pipe_stress_product_physics@0.2.0",
    }
}

fn general(request: &Value) -> Result<ReportPackageContainerOutcome, String> {
    assemble_wire_request(decode_report_package_request(request.clone())?, &linked())
}

#[track_caller]
fn refused_with(result: Result<ReportPackageContainerOutcome, String>, code: &str) -> String {
    match result {
        Ok(outcome) => panic!("expected {code}, assembled (blocked {}: {:?})", outcome.export_blocked, outcome.blocking_reasons),
        Err(error) => {
            assert!(error.starts_with(&format!("{code}:")), "expected {code}, got {error}");
            error
        }
    }
}

/// The eligible exact-route result documents of the committed W-11 package
/// requests (`fixtures/reports/invented/exact_report_package_requests/`),
/// which the local-private package admits.
fn exact_documents() -> [(&'static str, Value); 2] {
    let document = |text: &str| serde_json::from_str::<Value>(text).unwrap()["request"]["result_documents"][0].clone();
    [
        (PRESSURE_ID, document(include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_2_straight.json"))),
        (PRESSURE_SOURCE_ID, document(include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_source_1.json"))),
    ]
}

/// The request with `document` as its result document.
fn with_document(request: &Value, document: &Value) -> Value {
    let mut request = request.clone();
    request["result_documents"][0] = document.clone();
    request
}

/// The request's result document with another producer identity, re-hashed:
/// the preflight reads only the identity.
fn with_identity(request: &Value, id: &str) -> Value {
    let mut request = request.clone();
    let document = &mut request["result_documents"][0];
    document["result_envelope"]["producer"]["semantic_contract_id"] = json!(id);
    request["result_documents"][0] = common::rehash(document.take());
    request
}

// --- Pinned bytes ---------------------------------------------------------

/// Owner property 2 (§4, §6): a package without a user release, and a render
/// outside `user_released_report`, are byte-identical to the parent head's
/// (`8ed4ec6103`, recorded before (r2c) changed the writer and the renderer).
#[test]
fn local_private_package_bytes_unchanged_by_user_release() {
    fn context(export_context: &str) -> ReportContext {
        serde_json::from_value(json!({
            "export_context": export_context, "generated_at": null, "software_name": "SWBPIPE", "software_version": "0.1.0",
            "build_release_identifier": null, "solver_name": null, "solver_version": null, "solver_build_ref": null,
            "analysis_run_name": null, "analysis_run_recorded_at": null, "solver_mode": null, "solver_settings_ref": null,
            "solver_settings": [], "model_coordinate_system": null, "result_coordinate_systems": []
        }))
        .unwrap()
    }
    let fixture = Fixture::new();
    assert_eq!(assemble_report_package_container(&fixture.input()).container_sha256_hex, "fd0f8173d07c8e78f22a8602b5f0989401f89abc1fa783b7e9f99e6b70bf0250");
    let mut local = Fixture::new();
    local.report.report_context = Some(context("local_private"));
    assert_eq!(assemble_report_package_container(&local.input()).container_sha256_hex, "289cd47ecbae182062f384703355294b878b53cb410d108e94215bba13322ab5");
    assert_eq!(render_calculation_report(&local.report).sha256_hex, "bd010f800571c1eef33da72a55169d9990dd53d6a0e345f3e33d7463364f98cb");
    let mut public = Fixture::new();
    public.report.report_context = Some(context("public_report"));
    assert_eq!(render_calculation_report(&public.report).sha256_hex, "9e9d8fad71c5fbb7fbb2d72188b66c29a989bd9a84c5171923c2bc337b64ab08");

    // The desktop local-private and the runner packages of the committed basis.
    let runner = common::basis()["export_results"].clone();
    let solver = &runner["audit_manifest"]["solver_version"];
    let model: Value = serde_json::from_str(MODEL).unwrap();
    let manifest: Value = serde_json::from_str(include_str!("../../../../fixtures/reports/invented/del1005_input_manifest.json")).unwrap();
    let manifest_id = runner["audit_manifest"]["input_manifest_hash"]["payload_ref"].as_str().unwrap().to_string();
    let inputs = package_basis::Inputs {
        model: &model,
        source: &runner["source_results"][0],
        producer_document: &runner["result_documents"][0],
        solve_request: &Value::Null,
        input_manifest: &manifest,
        input_manifest_id: &manifest_id,
        solver_name: solver["solver_name"].as_str().unwrap(),
        solver_version: solver["solver_version"].as_str().unwrap(),
        profile: package_basis::Profile::Desktop,
        run_override: None,
    };
    let desktop = package_basis::build(&inputs).unwrap().export_results;
    let linked = LinkedSolverIdentity {
        solver_name: solver["solver_name"].as_str().unwrap(),
        solver_version: solver["solver_version"].as_str().unwrap(),
        solver_build_ref: solver["solver_build_ref"].as_str().unwrap(),
    };
    let assembled = |request: &Value| assemble_wire_request(decode_report_package_request(request.clone()).unwrap(), &linked).unwrap();
    assert_eq!(assembled(&desktop).container_sha256_hex, "4c1d7234f5b33528e40f7c71b04b552cd3c041ed9626fd0f948858deb6bf8984");
    assert_eq!(assembled(&runner).container_sha256_hex, "03c2efe5f76687e49b0974c6427770a8cf3b6b494546617a9a40f1d46d0295ec");
    // A local-private manifest has no user_release member and its three notes.
    let manifest: Value = serde_json::from_slice(&assembled(&desktop).members[0].bytes).unwrap();
    assert!(manifest.get("user_release").is_none());
    assert_eq!(manifest["boundary_notes"].as_array().unwrap().len(), 3);
}

/// The committed coherent request that the headless runner's refusal tests
/// submit (`core/runner/headless/tests/user_release_runner_refused.rs`,
/// `tests/test_headless_runner_contract.py`) is the first vector's derived
/// request, which the admitting entry admits. Set
/// `OPENPIPESTRESS_UPDATE_USER_RELEASE_RUNNER_REQUEST=1` to rewrite it after a
/// change to the derivation.
#[test]
fn the_committed_runner_refusal_request_is_the_derivation() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/reports/invented/user_release_runner_refusal_request.json");
    let case = case();
    let request = case.request();
    if std::env::var_os("OPENPIPESTRESS_UPDATE_USER_RELEASE_RUNNER_REQUEST").is_some() {
        std::fs::write(&path, canonical_json(&request) + "\n").unwrap();
    }
    let committed: Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    assert_eq!(open_pipe_stress_report_package::user_release::first_difference(&request, &committed), None, "canonically equal");
    assert!(!case.assemble(committed).unwrap().export_blocked);
}

/// The committed invented record equals the first vector's.
#[test]
fn the_committed_record_is_the_vectors() {
    let record: Value = serde_json::from_str(RECORD_FIXTURE).unwrap();
    assert_eq!(record, case().vector["user_release"]);
    let typed: UserReleaseRecord = serde_json::from_value(record).unwrap();
    assert_eq!(typed.form_defect(), None);
}

// --- The admitting entry ----------------------------------------------------

/// Owner property 3 (§6): the admitting entry assembles the coherent request.
/// The manifest's record is the report context's plus a `content_binding` of
/// the four members, the four notes are written, there is no handoff member,
/// both renderings carry the notice, and the reader returns the record.
#[test]
fn the_admitting_entry_writes_the_record_its_binding_and_notes() {
    for case in cases() {
        let outcome = case.assemble(case.request()).unwrap();
        assert!(!outcome.export_blocked, "{:?}", outcome.blocking_reasons);
        let roles: Vec<&str> = outcome.members.iter().map(|member| member.role.as_str()).collect();
        assert_eq!(roles, ["package_manifest", "canonical_rendered_report_html", "canonical_rendered_report_pdf", "audit_manifest", "result_export"]);
        let manifest: Value = serde_json::from_slice(&outcome.members[0].bytes).unwrap();
        let mut record = manifest["user_release"].clone();
        let binding = record.as_object_mut().unwrap().remove("content_binding").unwrap();
        assert_eq!(record, case.vector["user_release"]);
        assert_eq!(binding["algorithm"], "sha256");
        let bound: Vec<(String, String, String)> = binding["members"]
            .as_array()
            .unwrap()
            .iter()
            .map(|entry| (entry["file_name"].as_str().unwrap().into(), entry["role"].as_str().unwrap().into(), entry["sha256"].as_str().unwrap().into()))
            .collect();
        let expected: Vec<(String, String, String)> =
            outcome.members[1..].iter().map(|member| (member.file_name.clone(), member.role.clone(), sha256_hex(&member.bytes))).collect();
        assert_eq!(bound, expected);
        assert_eq!(manifest["export_profile_id"], USER_RELEASED_DESKTOP_PROFILE);
        let notes = manifest["boundary_notes"].as_array().unwrap();
        assert_eq!(notes.len(), 4);
        assert!(notes[2].as_str().unwrap().contains("except user_release.user_released_at"));
        assert!(notes[3].as_str().unwrap().starts_with("The user released their own project data in this package for sharing"));
        let html = std::str::from_utf8(&outcome.members[1].bytes).unwrap();
        assert_eq!(html.matches(&escape_html(&user_release_notice_for(&case.record))).count(), 1);
        assert!(!html.contains("LOCAL PRIVATE REPORT") && !html.contains("content_binding"));
        let pdf = &outcome.members[2].bytes;
        assert!(pdf.windows(b"USER-RELEASED REPORT".len()).any(|window| window == b"USER-RELEASED REPORT"));
        // The reader round trip.
        let read = read_report_package_container(&outcome.container_bytes).unwrap();
        assert_eq!(read.user_release.as_ref(), Some(&manifest["user_release"]));
        // Deterministic.
        assert_eq!(case.assemble(case.request()).unwrap().container_bytes, outcome.container_bytes);
    }
}

/// Every member of the derived request is bound: one changed member is
/// refused by the derivation, naming its pointer, never a value (§2.5 step 9).
#[test]
fn the_admitting_entry_requires_the_whole_request_derived() {
    let case = case();
    let edits: [(&str, fn(&mut Value)); 5] = [
        ("/report/report_title", |request| request["report"]["report_title"] = json!("Another title")),
        ("/package_id", |request| request["package_id"] = json!("desktop-user-released-report-other")),
        ("/report/report_sections/assumptions/0", |request| request["report"]["report_sections"]["assumptions"] = json!([{"invented": true}])),
        // `S` is the request's own source (native admission binds it to the
        // attested solve): another run id derives another package id first.
        ("/package_id", |request| request["source_results"][0]["run_id"] = json!("run:other")),
        ("/report/report_context/user_release/user_released_at", |request| {
            request["report"]["report_context"]["user_release"]["user_released_at"] = json!("2026-10-28T00:00:00Z")
        }),
    ];
    for (pointer, edit) in edits {
        let mut request = case.request();
        edit(&mut request);
        let error = refused_with(case.assemble(request), CONTENT_NOT_DERIVED);
        assert!(error.ends_with(&format!("at {pointer}")), "{error}");
    }
    // The input manifest is bound before the request.
    let mut manifest = case.input_manifest().clone();
    manifest["external_assets"] = json!([{"asset": "invented"}]);
    let error = assemble_derived(case.request(), &linked(), &case.inputs(), &manifest).unwrap_err();
    assert!(error.starts_with(&format!("{CONTENT_NOT_DERIVED}: the input manifest differs")), "{error}");
}

/// No replay (§2.10), producer rows: each bound member mismatched alone, with
/// the derivation coherent (the record is the basis's), is refused by item 10.
#[test]
fn the_producer_binds_each_record_member() {
    let case = case();
    let edits: [(&str, fn(&mut UserReleaseRecord)); 8] = [
        ("bound_to.model_ref", |record| record.bound_to.model_ref = "project:other".into()),
        ("bound_to.model_document_sha256", |record| record.bound_to.model_document_sha256 = "0".repeat(64)),
        ("bound_to.input_manifest_sha256", |record| record.bound_to.input_manifest_sha256 = "0".repeat(64)),
        ("bound_to.input_manifest_ref", |record| record.bound_to.input_manifest_ref = "input-manifest:other".into()),
        ("bound_to.analysis_run_ref", |record| record.bound_to.analysis_run_ref = "run:other".into()),
        ("bound_to.result_envelope_id", |record| record.bound_to.result_document_derivative_sha256 = "0".repeat(64)),
        ("scope.report_id", |record| record.scope.report_id = "report:run:other".into()),
        ("scope.package_id", |record| record.scope.package_id = "desktop-user-released-report-other".into()),
    ];
    for (member, edit) in edits {
        let mut record = case.record.clone();
        edit(&mut record);
        let basis = UserReleaseBasis { record: &record, ..case.basis() };
        let request = derive(&basis).unwrap().request;
        let error = refused_with(assemble_derived(request, &linked(), &UserReleaseInputs { record: basis.record, ..case.inputs() }, case.input_manifest()), PACKAGE_UNBOUND);
        assert!(error.contains(member), "{member}: {error}");
    }
    // A record whose statement is altered is not of its form.
    let mut record = case.record.clone();
    record.standing = "issued".into();
    let basis = UserReleaseBasis { record: &record, ..case.basis() };
    let error = refused_with(assemble_derived(derive(&basis).unwrap().request, &linked(), &UserReleaseInputs { record: basis.record, ..case.inputs() }, case.input_manifest()), PACKAGE_UNBOUND);
    assert!(error.contains("standing"), "{error}");
}

// --- Exact routes (§0) --------------------------------------------------------

/// `user_release_exact_route_refused`, the producer's cases (§6): through the
/// admitting entry an exact document is refused by the preflight, whatever the
/// report carries and whatever the profile, with no pointer named; a retained
/// identity gets item 0's own refusal. Through each general entry the same
/// request gets the marker refusal, or W-11's runner-profile refusal first.
#[test]
fn user_release_exact_route_refused() {
    let case = case();
    for (id, document) in exact_documents() {
        assert_eq!(document["result_envelope"]["producer"]["semantic_contract_id"], id);
        let exact = with_document(&case.request(), &document);
        let mut retitled = exact.clone();
        retitled["report"]["report_title"] = json!("Another title");
        let mut positional = exact.clone();
        let report = positional["report"].take();
        let object = report.as_object().unwrap();
        positional["report"] = json!(["report_title", "calculation_report", "report_sections", "result_rows", "report_context"]
            .iter()
            .map(|key| object[*key].clone())
            .collect::<Vec<_>>());
        let mut local_private = exact.clone();
        local_private["export_profile_id"] = json!(DESKTOP_PROFILE);
        local_private["report"]["report_context"].as_object_mut().unwrap().remove("user_release");
        local_private["report"]["report_context"]["export_context"] = json!("local_private");
        for (name, request) in [("unchanged", &exact), ("retitled", &retitled), ("positional", &positional), ("local-private", &local_private)] {
            let error = refused_with(case.assemble(request.clone()), EXACT_ROUTE_NOT_ADMITTED);
            assert!(!error.contains(" at /"), "{id} {name}: no pointer: {error}");
        }
        // The general entries.
        refused_with(general(&exact), PACKAGE_NOT_ADMITTED);
        let outcome = assemble_report_package_container(&Fixture::new().input_with_profile(USER_RELEASED_DESKTOP_PROFILE));
        assert!(outcome.export_blocked && outcome.container_bytes.is_empty() && outcome.blocking_reasons[0].starts_with(PACKAGE_NOT_ADMITTED));
        let mut runner = exact.clone();
        runner["export_profile_id"] = json!(RUNNER_PROFILE);
        refused_with(general(&runner), EXACT_RUNNER_PROFILE);
    }
    // A retained identity: item 0's own refusal, before the exact check.
    refused_with(case.assemble(with_identity(&case.request(), PHYSICS_RETAINED_ID)), ROUTE_NOT_ADMITTED);
}

// --- The general entries (§2.8, refusal 2) --------------------------------

/// `user_release_runner_refused`, the producer's cases (§6): the coherent
/// request, and each marker alone, are refused by every general entry; the
/// admitting entry admits the coherent request.
#[test]
fn the_general_entries_refuse_every_user_release_marker() {
    let case = case();
    let coherent = case.request();
    let mut profile_only = coherent.clone();
    profile_only["report"]["report_context"].as_object_mut().unwrap().remove("user_release");
    profile_only["report"]["report_context"]["export_context"] = json!("local_private");
    let mut context_only = coherent.clone();
    context_only["export_profile_id"] = json!(DESKTOP_PROFILE);
    context_only["report"]["report_context"].as_object_mut().unwrap().remove("user_release");
    let mut record_only = coherent.clone();
    record_only["export_profile_id"] = json!(DESKTOP_PROFILE);
    record_only["report"]["report_context"]["export_context"] = json!("local_private");
    for (name, request, marker) in [
        ("coherent", &coherent, "export_profile_id"),
        ("profile only", &profile_only, "export_profile_id"),
        ("context only", &context_only, "report.report_context.export_context"),
        ("record only", &record_only, "report.report_context.user_release"),
    ] {
        let error = refused_with(general(request), PACKAGE_NOT_ADMITTED);
        assert!(error.ends_with(&format!("(marker: {marker})")), "{name}: {error}");
    }
    // The writer's general entry refuses each before it renders.
    let fixture = Fixture::new();
    let mut context = fixture.report.clone();
    context.report_context = Some(serde_json::from_value(coherent["report"]["report_context"].clone()).unwrap());
    let mut record = context.clone();
    record.report_context.as_mut().unwrap().export_context = ReportExportContext::LocalPrivate;
    let mut released = context.clone();
    released.report_context.as_mut().unwrap().user_release = None;
    for (name, report, profile) in [
        ("profile", &fixture.report, USER_RELEASED_DESKTOP_PROFILE),
        ("context", &released, "invented-profile-native-full-report"),
        ("record", &record, "invented-profile-native-full-report"),
    ] {
        let mut input = fixture.input_with_profile(profile);
        input.report = report;
        let outcome = assemble_report_package_container(&input);
        assert!(outcome.export_blocked, "{name}");
        assert!(outcome.blocking_reasons[0].starts_with(&format!("{PACKAGE_NOT_ADMITTED}:")), "{name}: {:?}", outcome.blocking_reasons);
        assert!(outcome.members.is_empty() && outcome.container_bytes.is_empty() && outcome.report.render.html.is_empty(), "{name}: nothing rendered");
    }
    assert!(!case.assemble(coherent).unwrap().export_blocked);
}

// --- The reader (§2.9, §2.10) --------------------------------------------

/// Rebuilds a package's archive from its members (stored, fixed order), as
/// the writer encodes it, after `edit` changed members or the manifest.
fn repacked(outcome: &ReportPackageContainerOutcome, edit: impl FnOnce(&mut Vec<(String, Vec<u8>)>, &mut Value)) -> Vec<u8> {
    let mut members: Vec<(String, Vec<u8>)> = outcome.members[1..].iter().map(|member| (member.file_name.clone(), member.bytes.clone())).collect();
    let mut manifest: Value = serde_json::from_slice(&outcome.members[0].bytes).unwrap();
    edit(&mut members, &mut manifest);
    // Restamp the inventory.
    let rows: Vec<Value> = manifest["members"].as_array().unwrap().clone();
    let mut restamped = Vec::new();
    for (name, bytes) in &members {
        let mut row = rows.iter().find(|row| row["file_name"] == name.as_str()).cloned().unwrap_or_else(|| rows[rows.len() - 1].clone());
        row["file_name"] = json!(name);
        row["sha256_hex"] = json!(sha256_hex(bytes));
        row["byte_length"] = json!(bytes.len());
        restamped.push(row);
    }
    manifest["members"] = json!(restamped);
    let mut all = vec![("package_manifest.json".to_string(), canonical_json(&manifest).into_bytes())];
    all.extend(members);
    zip(&all)
}

fn zip(members: &[(String, Vec<u8>)]) -> Vec<u8> {
    fn crc32(bytes: &[u8]) -> u32 {
        let mut crc: u32 = 0xFFFF_FFFF;
        for byte in bytes {
            crc ^= u32::from(*byte);
            for _ in 0..8 {
                crc = (crc >> 1) ^ (0xEDB8_8320 & (crc & 1).wrapping_neg());
            }
        }
        !crc
    }
    let (mut out, mut central) = (Vec::new(), Vec::new());
    for (name, data) in members {
        let offset = out.len() as u32;
        let header = |buffer: &mut Vec<u8>| {
            for value in [0u16, 0, 0, 0x0021] {
                buffer.extend_from_slice(&value.to_le_bytes());
            }
            buffer.extend_from_slice(&crc32(data).to_le_bytes());
            buffer.extend_from_slice(&(data.len() as u32).to_le_bytes());
            buffer.extend_from_slice(&(data.len() as u32).to_le_bytes());
            buffer.extend_from_slice(&(name.len() as u16).to_le_bytes());
            buffer.extend_from_slice(&0u16.to_le_bytes());
        };
        out.extend_from_slice(&0x0403_4b50u32.to_le_bytes());
        out.extend_from_slice(&20u16.to_le_bytes());
        header(&mut out);
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(data);
        central.extend_from_slice(&0x0201_4b50u32.to_le_bytes());
        central.extend_from_slice(&20u16.to_le_bytes());
        central.extend_from_slice(&20u16.to_le_bytes());
        header(&mut central);
        central.extend_from_slice(&[0u8; 10]);
        central.extend_from_slice(&offset.to_le_bytes());
        central.extend_from_slice(name.as_bytes());
    }
    let (central_offset, central_size) = (out.len() as u32, central.len() as u32);
    out.extend_from_slice(&central);
    out.extend_from_slice(&0x0605_4b50u32.to_le_bytes());
    out.extend_from_slice(&[0u8; 4]);
    out.extend_from_slice(&(members.len() as u16).to_le_bytes());
    out.extend_from_slice(&(members.len() as u16).to_le_bytes());
    out.extend_from_slice(&central_size.to_le_bytes());
    out.extend_from_slice(&central_offset.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out
}

#[track_caller]
fn unbound(bytes: &[u8], what: &str) -> String {
    match read_report_package_container(bytes) {
        Ok(_) => panic!("{what}: read"),
        Err(refusal @ PackageReadRefusal::UserReleaseUnbound { .. }) => {
            assert_eq!(refusal.code(), "OPSPROJ_USER_RELEASE_UNBOUND");
            refusal.to_string()
        }
        Err(other) => panic!("{what}: expected OPSPROJ_USER_RELEASE_UNBOUND, got {other}"),
    }
}

fn admitted_package() -> (Case, ReportPackageContainerOutcome) {
    let case = case();
    let outcome = case.assemble(case.request()).unwrap();
    (case, outcome)
}

/// The record's members and binding, re-written in the manifest only.
fn record_edit(edit: impl FnOnce(&mut Value)) -> impl FnOnce(&mut Vec<(String, Vec<u8>)>, &mut Value) {
    move |_, manifest| edit(&mut manifest["user_release"])
}

#[test]
fn the_reader_checks_a_user_released_package() {
    let (_, outcome) = admitted_package();
    // The test's archive builder reproduces the writer's bytes.
    assert_eq!(repacked(&outcome, |_, _| {}), outcome.container_bytes);

    // `model_ref` or `report_id` alone changed (§2.10).
    unbound(&repacked(&outcome, record_edit(|record| record["bound_to"]["model_ref"] = json!("project:other"))), "model_ref");
    unbound(&repacked(&outcome, record_edit(|record| record["scope"]["report_id"] = json!("report:run:other"))), "report_id");
    // An altered statement, standing or `not_carried`.
    unbound(&repacked(&outcome, record_edit(|record| record["statement"] = json!("I release this."))), "statement");
    unbound(&repacked(&outcome, record_edit(|record| record["standing"] = json!("issued"))), "standing");
    unbound(&repacked(&outcome, record_edit(|record| record["scope"]["not_carried"] = json!([]))), "not_carried");
    // An extra member in the record.
    unbound(&repacked(&outcome, record_edit(|record| record["claim"] = json!("invented"))), "extra member");
    // A record under another profile, and the profile without a record.
    unbound(&repacked(&outcome, |_, manifest| manifest["export_profile_id"] = json!(DESKTOP_PROFILE)), "another profile");
    unbound(
        &repacked(&outcome, |_, manifest| {
            manifest.as_object_mut().unwrap().remove("user_release");
        }),
        "no record",
    );
    // A record in a blocked package.
    unbound(
        &repacked(&outcome, |_, manifest| {
            manifest["diagnostics"] = json!({"export_blocked": true, "blocking_reasons": ["invented"]});
        }),
        "blocked",
    );
}

/// `user_release_extra_member_refused` (§6): a second, valid result member,
/// or a valid state/comparison/handoff member, added with the inventory
/// restamped.
#[test]
fn user_release_extra_member_refused() {
    let (_, outcome) = admitted_package();
    // A second valid result member: the derivation's document of the same
    // model and manifest for a source run of another id.
    let (case, _) = admitted_package();
    let mut source = case.source.clone();
    source["run_id"] = json!("run:other");
    let manifest_ref = case.vector["expected"]["input_manifest_ref"].as_str().unwrap();
    let origin = current_received_origin(&case.model, &source, manifest_ref).unwrap();
    let base = current_result_document_base(&case.model, &source, &case.vector["run_record"], manifest_ref, &case.input_manifest()["solver_basis"]).unwrap();
    let other = derive_document(base, &case.model, &source, origin, None).unwrap();
    assert_eq!(other["result_envelope"]["envelope_id"], "result-envelope:run:other");
    unbound(
        &repacked(&outcome, |members, manifest| {
            members.push(("result_export_result-envelope-run-other.json".to_string(), canonical_json(&other).into_bytes()));
            let mut row = manifest["members"][3].clone();
            row["file_name"] = json!("result_export_result-envelope-run-other.json");
            manifest["members"].as_array_mut().unwrap().push(row);
        }),
        "second result member",
    );
    let section = common::invented_sections_record();
    unbound(
        &repacked(&outcome, |members, manifest| {
            let name = format!("state_comparison_handoff_sections_{}.json", section["section_set_id"].as_str().unwrap().replace(':', "-"));
            members.push((name.clone(), canonical_json(&section).into_bytes()));
            let mut row = manifest["members"][3].clone();
            row["file_name"] = json!(name);
            row["role"] = json!(ROLE_STATE_COMPARISON_HANDOFF);
            manifest["members"].as_array_mut().unwrap().push(row);
        }),
        "handoff member",
    );
}

/// `user_release_same_source_member_substitution_refused` (§6): the HTML and
/// PDF of another user-released save of the same model and run (another
/// `user_released_at`), and an equal-identity audit variant, each swapped in
/// with the inventory restamped and the record unchanged.
#[test]
fn user_release_same_source_member_substitution_refused() {
    let (case, outcome) = admitted_package();
    let mut later = case.record.clone();
    later.user_released_at = "2026-10-28T09:30:00Z".to_string();
    let basis = UserReleaseBasis { record: &later, ..case.basis() };
    let other = assemble_derived(derive(&basis).unwrap().request, &linked(), &UserReleaseInputs { record: basis.record, ..case.inputs() }, case.input_manifest()).unwrap();
    for (index, what) in [(1usize, "HTML"), (2, "PDF")] {
        let bytes = other.members[index].bytes.clone();
        let error = unbound(&repacked(&outcome, |members, _| members[index - 1].1 = bytes), what);
        assert!(error.contains("content_binding"), "{what}: {error}");
    }
    // An audit variant with the same identities (another solver build note).
    let mut audit: Value = serde_json::from_slice(&outcome.members[3].bytes).unwrap();
    audit["solver_version"]["solver_build_ref"] = json!("open_pipe_stress_product_physics@0.2.0 (variant)");
    let bytes = canonical_json(&audit).into_bytes();
    assert_ne!(bytes, outcome.members[3].bytes);
    unbound(&repacked(&outcome, |members, _| members[2].1 = bytes), "audit");
    // Only the HTML ties the notice: a notice for another record is refused
    // even with a re-bound content_binding.
    let html = other.members[1].bytes.clone();
    let error = unbound(
        &repacked(&outcome, |members, manifest| {
            members[0].1 = html;
            manifest["user_release"]["content_binding"]["members"][0]["sha256"] = json!(sha256_hex(&members[0].1));
        }),
        "notice",
    );
    assert!(error.contains("notice"), "{error}");
}

/// `user_release_exact_route_refused`, the reader's case (§6): a user-released
/// package whose result member is an exact document, re-hashed coherently.
#[test]
fn the_reader_refuses_an_exact_result_member() {
    let (_, outcome) = admitted_package();
    let [(_, document), _] = exact_documents();
    let derivative = document["result_envelope"]["reproducibility"]["derivative_hash"]["value"].clone();
    let error = unbound(
        &repacked(&outcome, |members, manifest| {
            members[3].1 = canonical_json(&document).into_bytes();
            manifest["user_release"]["bound_to"]["result_document_derivative_sha256"] = derivative;
            let sha = sha256_hex(&members[3].1);
            manifest["user_release"]["content_binding"]["members"][3]["sha256"] = json!(sha);
        }),
        "exact member",
    );
    assert!(error.contains(EXACT_ROUTE_NOT_ADMITTED) || error.contains("pressure-free"), "{error}");
}

/// The documented limit (§2.9, O-G): a package whose record, members,
/// `content_binding` and inventory are all re-written together is accepted.
/// The reader's checks establish consistency, not consent or origin.
#[test]
fn a_coherently_rewritten_package_is_accepted_as_a_documented_limit() {
    let (case, outcome) = admitted_package();
    let mut later = case.record.clone();
    later.user_released_at = "2026-10-29T10:00:00Z".to_string();
    let basis = UserReleaseBasis { record: &later, ..case.basis() };
    let rewritten = assemble_derived(derive(&basis).unwrap().request, &linked(), &UserReleaseInputs { record: basis.record, ..case.inputs() }, case.input_manifest()).unwrap();
    assert_ne!(rewritten.container_bytes, outcome.container_bytes);
    assert!(read_report_package_container(&rewritten.container_bytes).is_ok());
    let _ = (USER_RELEASED_CONTEXT, digest(&json!(1)));
}
