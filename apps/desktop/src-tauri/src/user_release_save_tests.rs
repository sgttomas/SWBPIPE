//! Route `DREP-PACKAGE-USER-RELEASE-010`'s native admission (W-18b slice
//! (r2c); DEL-08-01 Design `report-user-release.md` §2.5, §2.7, §2.10, §6):
//! the outer command's route-aware pricing, steps 1-10, the save and the
//! read-back, and the refusal formatting's price. The basis is the committed
//! derivation vectors (`fixtures/reports/invented/user_release_derivation_vectors.json`):
//! an invented model, its committed solve, the solve's transient run record,
//! the input manifest and an invented record. The solve is attested in the
//! test's registry as publication records it. Every value is invented;
//! "admitted" means only that the checks hold, and nothing here records a
//! user release beyond the invented record. Standard claim fence applies
//! (claims taxonomy per DEC-081).

use crate::command_ingress::test_support::{admit_json, registry};
use crate::command_ingress::{text_parse_bytes, ArgsDecoding};
use crate::memory_pool::test_host::ScriptedHost;
use crate::memory_pool::Lease;
use crate::report_package_save::test_support::{args_body, hash_of, no_store, sandbox, save_request_with, software};
use crate::report_package_save::{
    admit_report_package_session, ingress_lease_bytes, parse_lease_bytes, save_admitted_report_package_in, ReportPackageSaveRequest,
    SaveRefusal, UserReleaseSession, REPORT_PACKAGE_USER_RELEASE_ROUTE_ID,
};
use crate::solve_attestation::SolveAttestation;
use crate::solve_jobs::SolveJobRegistry;
use crate::user_release_admission::{refusal_formatting_bytes, refuse_library_referenced, LibraryRefusal};
use open_pipe_stress_operation_applier::{canonical_json, sha256_hex};
use open_pipe_stress_product_physics::PreviewSolverMode;
use open_pipe_stress_report_package::read_report_package_container;
use open_pipe_stress_report_package::user_release::{
    derive, derive_input_manifest, input_manifest_ref, SoftwareIdentity, UserReleaseBasis, ANALYSIS_RECORD_UNBOUND, CONTENT_NOT_DERIVED,
    EXACT_ROUTE_NOT_ADMITTED, LIBRARY_REFERENCED, NOT_THIS_ROUTE, PRIVATE_CLASS, RECORD_INVALID, RESULT_NOT_ATTESTED, RULE_PACK_REFERENCED,
    UNBOUND,
};
use open_pipe_stress_report_renderer::UserReleaseRecord;
use serde_json::{json, Value};

const VECTORS: &str = include_str!("../../../../fixtures/reports/invented/user_release_derivation_vectors.json");
const MODEL: &str = include_str!("../../../../fixtures/model_operations/precision_connected_ui_model.json");
const SOURCE: &str = include_str!("../../../../fixtures/results/preview_physics_connected_sparse.json");
const EXACT_STRAIGHT: &str = include_str!("../../../../fixtures/reports/invented/exact_report_package_requests/pressure_2_straight.json");

/// The first vector's basis: the model, its committed sparse solve, the run
/// record, the input manifest and the record.
struct Basis {
    model: Value,
    source: Value,
    manifest: Value,
    run_record: Value,
    record: UserReleaseRecord,
}

fn basis() -> Basis {
    let vector = serde_json::from_str::<Value>(VECTORS).unwrap()["vectors"][0].clone();
    assert_eq!(vector["source_fixture"], "fixtures/results/preview_physics_connected_sparse.json");
    assert_eq!(vector["software"], json!({"name": software().name, "version": software().version}));
    Basis {
        model: serde_json::from_str(MODEL).unwrap(),
        source: serde_json::from_str(SOURCE).unwrap(),
        manifest: vector["input_manifest"].clone(),
        run_record: vector["run_record"].clone(),
        record: serde_json::from_value(vector["user_release"].clone()).unwrap(),
    }
}

impl Basis {
    /// The coherent request: the derivation of this basis.
    fn request(&self) -> Value {
        let identity = SoftwareIdentity { name: software().name, version: software().version };
        derive(&UserReleaseBasis {
            model: &self.model,
            source: &self.source,
            solver_mode: "sparse_interactive",
            run_record: &self.run_record,
            software: &identity,
            record: &self.record,
        })
        .unwrap()
        .request
    }

    /// The route-010 arguments for `request`, `model`, `manifest` and this
    /// basis's run record, with clean route-010 evidence.
    fn args_for(&self, request: &Value, model: &Value, manifest: &Value) -> ReportPackageSaveRequest {
        let mut args = save_request_with(request, model, manifest);
        user_release_evidence(&mut args);
        args.analysis_run_text = Some(canonical_json(&self.run_record));
        args
    }

    fn args(&self) -> ReportPackageSaveRequest {
        self.args_for(&self.request(), &self.model, &self.manifest)
    }

    /// A registry whose attestation table holds this basis's solve, as
    /// publication records it, and a 1 KiB ingress lease in it.
    fn session(&self, mode: PreviewSolverMode) -> (SolveJobRegistry, Lease) {
        let registry = registry(ScriptedHost::normal());
        registry.record_attested_solve_for_tests(1, mode, SolveAttestation::of(&self.model, &self.source).unwrap());
        let lease = registry.acquire_interactive_ingress(1024).unwrap();
        (registry, lease)
    }
}

/// Route-010 evidence (§2.5 step 1), bound to the request text.
fn user_release_evidence(args: &mut ReportPackageSaveRequest) {
    let screening = &mut args.screening;
    screening.route_id = REPORT_PACKAGE_USER_RELEASE_ROUTE_ID.to_string();
    screening.export_context = "user_released_report".to_string();
    screening.explicit_local_private_intent = false;
    screening.explicit_user_release_intent = true;
    let local_first = args.local_first.as_mut().unwrap();
    local_first.route_id = REPORT_PACKAGE_USER_RELEASE_ROUTE_ID.to_string();
    local_first.export_context = "user_released_report".to_string();
    local_first.storage_context = "private_report".to_string();
    local_first.reason_code = "USER_RELEASE_METADATA_ALLOWED".to_string();
    local_first.explicit_local_private_intent = false;
    local_first.explicit_user_release_intent = true;
}

fn admit(args: ReportPackageSaveRequest, registry: &SolveJobRegistry, lease: &mut Lease) -> Result<crate::report_package_save::AdmittedReportPackage, SaveRefusal> {
    let mut session = UserReleaseSession { registry, lease };
    admit_report_package_session(args, &software(), &mut no_store, &crate::report_package_save::no_custody, Some(&mut session))
}

#[track_caller]
fn refused(args: ReportPackageSaveRequest, registry: &SolveJobRegistry, code: &str) -> (String, String) {
    let mut lease = registry.acquire_interactive_ingress(1024).unwrap();
    match admit(args, registry, &mut lease) {
        Ok(_) => panic!("expected {code}: admitted"),
        Err(SaveRefusal::Resource(refusal)) => panic!("expected {code}: resource {refusal:?}"),
        Err(SaveRefusal::Command(error)) => {
            assert_eq!(error.code, code, "{}", error.message);
            let path = error.diagnostics.first().map(|item| item.path.clone()).unwrap_or_default();
            (path, error.message)
        }
    }
}

/// `request` rebound to `model` and its derived input manifest: the audit
/// hashes and the record's bound identities follow, so steps 1-4 hold and
/// only a later step can refuse it.
fn rebound(basis: &Basis, request: &Value, model: &Value) -> (Value, Value) {
    let manifest = derive_input_manifest(model, &basis.source, "sparse_interactive").unwrap();
    let (model_hex, manifest_hex) = (sha256_hex(&canonical_json(model)), sha256_hex(&canonical_json(&manifest)));
    let manifest_ref = input_manifest_ref(model["project"]["id"].as_str().unwrap(), &manifest_hex);
    let mut request = request.clone();
    request["audit_manifest"]["model_hash"]["value"] = json!(model_hex);
    request["audit_manifest"]["input_manifest_hash"]["value"] = json!(manifest_hex);
    request["audit_manifest"]["input_manifest_hash"]["payload_ref"] = json!(manifest_ref);
    let bound = &mut request["report"]["report_context"]["user_release"]["bound_to"];
    bound["model_document_sha256"] = json!(model_hex);
    bound["input_manifest_sha256"] = json!(manifest_hex);
    bound["input_manifest_ref"] = json!(manifest_ref);
    (request, manifest)
}

// --- The admitted save ------------------------------------------------------

/// The coherent request is admitted on route 010, saved to a sandbox through
/// the counting picker, and read back with its record, binding and member
/// hashes intact; the receipt records the user release.
#[test]
fn route_010_admits_saves_and_reads_back_a_user_released_package() {
    let basis = basis();
    let (registry, mut lease) = basis.session(PreviewSolverMode::SparseInteractive);
    let admitted = admit(basis.args(), &registry, &mut lease).unwrap_or_else(|refusal| panic!("{refusal:?}"));
    assert!(!admitted.package.export_blocked, "{:?}", admitted.package.blocking_reasons);
    assert_eq!(admitted.package.container_file_name, "desktop-user-released-report-run-preview-linear-static-001.opsproj");
    let dir = sandbox("user-release-admitted");
    let mut picked = 0;
    let receipt = save_admitted_report_package_in(admitted, &crate::report_package_save::no_custody, |name| {
        picked += 1;
        Some(Ok(dir.join(name)))
    })
    .unwrap();
    assert_eq!(picked, 1);
    assert!(receipt.user_release_recorded);
    assert_eq!(receipt.user_release_model_document_sha256, basis.record.bound_to.model_document_sha256);
    let bytes = std::fs::read(dir.join("desktop-user-released-report-run-preview-linear-static-001.opsproj")).unwrap();
    assert_eq!(read_report_package_container(&bytes).unwrap().container_sha256_hex, receipt.container_sha256_hex);
    let read = read_report_package_container(&bytes).unwrap();
    let mut record = read.user_release.clone().unwrap();
    let binding = record.as_object_mut().unwrap().remove("content_binding").unwrap();
    assert_eq!(record, serde_json::to_value(&basis.record).unwrap());
    assert_eq!(binding["members"].as_array().unwrap().len(), 4);
    assert!(read.members.iter().all(|member| member.role != "state_comparison_handoff_sections"));
    let _ = std::fs::remove_dir_all(&dir);
}

// --- The outer command (§2.5, "The outer command") ---------------------------

/// `user_release_exact_route_refused`, the native cases (§6): entering as the
/// command does (`command_ingress::admit` with a `save_report_package` body,
/// then `ingress_lease_bytes`), a route-010 request carrying an exact text,
/// valid JSON, not JSON or over its cap, is refused with the exact-route code
/// before the text is sized, prescanned or priced, so the lease never grows.
/// A request whose result document is exact, or whose report carries an
/// `exact_route`, is priced by route 010's reservation alone and refused at
/// step 2a, with no custody query and nothing saved. The same exact request on
/// route 009 is still admitted to its own pricing.
#[test]
fn user_release_exact_route_refused() {
    let basis = basis();
    let registry = registry(ScriptedHost::normal());
    let over = "x".repeat((crate::report_package_save::MAX_REQUEST_TEXT_BYTES + 1) as usize);
    for member in ["exact_result_text", "exact_intake_text"] {
        for text in ["{\"valid\":true}", "not json", over.as_str()] {
            let mut args = basis.args();
            match member {
                "exact_result_text" => args.exact_result_text = Some(text.to_string()),
                _ => args.exact_intake_text = Some(text.to_string()),
            }
            let mut body = args_body(&args);
            body["analysisRunCanonicalText"] = json!(args.analysis_run_text);
            body["exactResultText"] = json!(args.exact_result_text);
            body["exactIntakeText"] = json!(args.exact_intake_text);
            let (mut decoded, lease) = admit_json(&registry, "save_report_package", ArgsDecoding::Moved, &body).unwrap();
            let admitted_bytes = lease.bytes();
            let request = crate::report_package_save::save_report_package_args(&mut decoded).unwrap();
            let refusal = ingress_lease_bytes(&request).unwrap_err();
            assert_eq!(refusal.code, EXACT_ROUTE_NOT_ADMITTED, "{member} {}: {}", text.len(), refusal.message);
            assert_eq!(refusal.diagnostics[0].path, format!("/{member}"));
            assert_eq!(lease.bytes(), admitted_bytes, "no growth at all");
            // A direct caller of the admission gets the same refusal.
            refused(request, &registry, EXACT_ROUTE_NOT_ADMITTED);
        }
    }
    // Step 2a: an exact document, and an exact_route in a pressure-free report.
    let exact_document = serde_json::from_str::<Value>(EXACT_STRAIGHT).unwrap()["request"]["result_documents"][0].clone();
    let mut with_exact = basis.request();
    with_exact["result_documents"][0] = exact_document;
    let mut with_route = basis.request();
    with_route["report"]["exact_route"] = json!({"semantic_contract_id": "openpipestress.result_semantics/0.3.0/pressure-2"});
    for request in [&with_exact, &with_route] {
        let args = basis.args_for(request, &basis.model, &basis.manifest);
        let reservation = ingress_lease_bytes(&args).unwrap();
        let ordinary = parse_lease_bytes(&args.request_text, &args.model_text, &args.input_manifest_text).unwrap();
        let run = text_parse_bytes(args.analysis_run_text.as_deref().unwrap(), ArgsDecoding::Moved).unwrap();
        let derivation = text_parse_bytes(&args.request_text, ArgsDecoding::Moved).unwrap() + text_parse_bytes(&args.input_manifest_text, ArgsDecoding::Moved).unwrap();
        let formatting = refusal_formatting_bytes(args.request_text.len() as u64, args.model_text.len() as u64);
        assert_eq!(reservation, ordinary + run + derivation + formatting, "route 010's reservation, no exact-text growth");
        let (path, _) = refused(args, &registry, EXACT_ROUTE_NOT_ADMITTED);
        assert_eq!(path, "/request/result_documents/0");
    }
}

// --- Steps 1-9: each refusal (§2.5, §2.10, §6) -------------------------------

#[test]
fn route_010_refuses_forged_evidence() {
    let basis = basis();
    let registry = registry(ScriptedHost::normal());
    let edits: [(&str, fn(&mut ReportPackageSaveRequest)); 5] = [
        ("route-010 evidence with local_private", |args| args.screening.export_context = "local_private".to_string()),
        ("user-release intent false", |args| args.screening.explicit_user_release_intent = false),
        ("local-private intent with route 010", |args| args.screening.explicit_local_private_intent = true),
        ("route-009 evidence over a user-released request", |args| args.screening.route_id = crate::report_package_save::REPORT_PACKAGE_SAVE_ROUTE_ID.to_string()),
        ("local-first evidence with another storage context", |args| args.local_first.as_mut().unwrap().storage_context = "local_private".to_string()),
    ];
    for (name, edit) in edits {
        let mut args = basis.args();
        edit(&mut args);
        let mut lease = registry.acquire_interactive_ingress(1024).unwrap();
        match admit(args, &registry, &mut lease) {
            Err(SaveRefusal::Command(error)) => assert!(error.code.ends_with("EVIDENCE-INVALID"), "{name}: {}", error.code),
            other => panic!("{name}: {:?}", other.map(|_| ())),
        }
    }
    // Evidence for other bytes.
    let mut args = basis.args();
    args.screening.payload_sha256 = hash_of("other");
    refused(args, &registry, crate::report_package_save::EVIDENCE_UNBOUND);
}

/// No replay (§2.10), native rows, and the record's form (§2.5 step 4).
#[test]
fn route_010_binds_the_record() {
    let basis = basis();
    let (registry, _) = basis.session(PreviewSolverMode::SparseInteractive);
    let record_edit = |edit: fn(&mut Value)| {
        let mut request = basis.request();
        edit(&mut request["report"]["report_context"]["user_release"]);
        basis.args_for(&request, &basis.model, &basis.manifest)
    };
    for (member, edit) in [
        ("bound_to/model_ref", (|record: &mut Value| record["bound_to"]["model_ref"] = json!("project:other")) as fn(&mut Value)),
        ("bound_to/model_document_sha256", |record| record["bound_to"]["model_document_sha256"] = json!("0".repeat(64))),
        ("bound_to/analysis_run_ref", |record| record["bound_to"]["analysis_run_ref"] = json!("run:other")),
        ("bound_to/input_manifest_sha256", |record| record["bound_to"]["input_manifest_sha256"] = json!("0".repeat(64))),
        ("bound_to/result_envelope_id", |record| record["bound_to"]["result_envelope_id"] = json!("result-envelope:other")),
        ("scope/report_id", |record| record["scope"]["report_id"] = json!("report:run:other")),
    ] {
        let (path, _) = refused(record_edit(edit), &registry, UNBOUND);
        assert_eq!(path, format!("/request/report/report_context/user_release/{member}"));
    }
    for edit in [
        (|record: &mut Value| record["statement"] = json!("I release this.")) as fn(&mut Value),
        |record| record["standing"] = json!("issued"),
        |record| record["scope"]["not_carried"] = json!([]),
        |record| record["content_binding"] = json!({"algorithm": "sha256", "members": []}),
        |record| record["user_released_at"] = json!("yesterday"),
    ] {
        refused(record_edit(edit), &registry, RECORD_INVALID);
    }
    // A record on the local-private route 009.
    let mut args = save_request_with(&basis.request(), &basis.model, &basis.manifest);
    let mut request: Value = serde_json::from_str(&args.request_text).unwrap();
    request["export_profile_id"] = json!(open_pipe_stress_report_package::bindings::DESKTOP_PROFILE);
    request["report"]["report_context"]["export_context"] = json!("local_private");
    args = save_request_with(&request, &basis.model, &basis.manifest);
    let (path, _) = refused(args, &registry, NOT_THIS_ROUTE);
    assert_eq!(path, "/request/report/report_context/user_release");
    // Another model's text: the audit hash no longer names it.
    let mut other = basis.model.clone();
    other["project"]["name"] = json!("Another invented project");
    refused(basis.args_for(&basis.request(), &other, &basis.manifest), &registry, crate::report_package_save::MODEL_UNBOUND);
}

/// Steps 5-7: the reference refusals, the private classes and the solve
/// attestation, each with the record and the audit coherent.
#[test]
fn route_010_refuses_references_private_classes_and_unattested_results() {
    let basis = basis();
    let (registry, _) = basis.session(PreviewSolverMode::SparseInteractive);
    // A library-applied material (rule 1), and its provenance string alone
    // (rule 2); the detector's own cases are `user_release_admission`'s.
    let mut library = basis.model.clone();
    library["materials"][0]["provenance_record"] = json!({"library_ref": {"library_kind": "material", "library_id": "matlib.invented", "library_version": "1.0.0", "record_id": "steel.1"}});
    let (request, manifest) = rebound(&basis, &basis.request(), &library);
    let (path, message) = refused(basis.args_for(&request, &library, &manifest), &registry, LIBRARY_REFERENCED);
    assert_eq!(path, "/model");
    assert!(message.contains("/materials/0/provenance_record/library_ref holds a library origin (rule library_ref"), "{message}");
    let mut marked = basis.model.clone();
    marked["materials"][0]["provenance"] = json!("library:material:matlib.invented#steel.1");
    let (request, manifest) = rebound(&basis, &basis.request(), &marked);
    let (_, message) = refused(basis.args_for(&request, &marked, &manifest), &registry, LIBRARY_REFERENCED);
    assert!(message.contains("/materials/0/provenance holds a library origin (rule provenance_string"), "{message}");
    // A rule-pack reference.
    let mut packed = basis.model.clone();
    packed["rule_pack_refs"] = json!([{"rule_pack_id": "invented-pack", "version": "1.0.0"}]);
    let (request, manifest) = rebound(&basis, &basis.request(), &packed);
    refused(basis.args_for(&request, &packed, &manifest), &registry, RULE_PACK_REFERENCED);
    // A non-project private class at depth in the request.
    let mut request = basis.request();
    request["report"]["report_sections"]["provenance_notes"][0]["privacy_classification"] = json!("private_rule_pack_data");
    let (path, message) = refused(basis.args_for(&request, &basis.model, &basis.manifest), &registry, PRIVATE_CLASS);
    assert_eq!(path, "/request");
    assert!(message.contains("/request/report/report_sections/provenance_notes/0/privacy_classification is private_rule_pack_data"), "{message}");
    // An unattested carrier, a mode mismatch and an evicted attestation.
    let unattested = crate::command_ingress::test_support::registry(ScriptedHost::normal());
    refused(basis.args(), &unattested, RESULT_NOT_ATTESTED);
    let (dense, _) = basis.session(PreviewSolverMode::DenseScrutiny);
    refused(basis.args(), &dense, RESULT_NOT_ATTESTED);
    let (evicted, _) = basis.session(PreviewSolverMode::SparseInteractive);
    for job in 2..=(crate::solve_attestation::MAX_ATTESTED_SOLVES as u64 + 1) {
        let other = json!({"invented": job});
        evicted.record_attested_solve_for_tests(job, PreviewSolverMode::SparseInteractive, SolveAttestation::of(&other, &other).unwrap());
    }
    refused(basis.args(), &evicted, RESULT_NOT_ATTESTED);
    // A source row annotation added only in the request: not the attested solve.
    let mut annotated = basis.request();
    annotated["source_results"][0]["results"][0]["metadata"]["invented_marker"] = json!("added");
    refused(basis.args_for(&annotated, &basis.model, &basis.manifest), &registry, RESULT_NOT_ATTESTED);
}

/// Steps 8 and 9: the run record and the derivation.
#[test]
fn route_010_checks_the_run_record_and_the_derivation() {
    let basis = basis();
    let (registry, _) = basis.session(PreviewSolverMode::SparseInteractive);
    // Another run record (an aggregate rule status, its checksum recomputed).
    let mut args = basis.args();
    let mut record = basis.run_record.clone();
    record["analysis_run"]["run_name"] = json!("another run name");
    let hash = crate::analysis_runs::run_record_hash(&record).unwrap();
    record["analysis_run"]["hashes"][0]["value"] = json!(hash);
    args.analysis_run_text = Some(canonical_json(&record));
    refused(args, &registry, ANALYSIS_RECORD_UNBOUND);
    // No run record text.
    let mut args = basis.args();
    args.analysis_run_text = None;
    refused(args, &registry, ANALYSIS_RECORD_UNBOUND);
    // An extra member at a request pointer the record does not bind.
    let mut request = basis.request();
    request["report"]["report_sections"]["assumptions"] = json!([{"invented": true}]);
    let (_, message) = refused(basis.args_for(&request, &basis.model, &basis.manifest), &registry, CONTENT_NOT_DERIVED);
    assert!(message.ends_with("at /report/report_sections/assumptions/0"), "{message}");
    // A handoff record set.
    let mut request = basis.request();
    request["state_comparison_handoff_records"] = json!([{"deliverable_id": "DEL-08-06", "section_set_id": "invented"}]);
    refused(basis.args_for(&request, &basis.model, &basis.manifest), &registry, CONTENT_NOT_DERIVED);
    // A non-fixed solver setting in the input manifest.
    let mut manifest = basis.manifest.clone();
    manifest["solver_basis"]["settings"]["sparse_evidence_lane"] = json!(false);
    let (request, _) = rebound(&basis, &basis.request(), &basis.model);
    let mut request = request;
    let hex = sha256_hex(&canonical_json(&manifest));
    let reference = input_manifest_ref(basis.model["project"]["id"].as_str().unwrap(), &hex);
    request["audit_manifest"]["input_manifest_hash"]["value"] = json!(hex);
    request["audit_manifest"]["input_manifest_hash"]["payload_ref"] = json!(reference);
    request["report"]["report_context"]["user_release"]["bound_to"]["input_manifest_sha256"] = json!(hex);
    request["report"]["report_context"]["user_release"]["bound_to"]["input_manifest_ref"] = json!(reference);
    let (code, _) = match admit(basis.args_for(&request, &basis.model, &manifest), &registry, &mut registry.acquire_interactive_ingress(1024).unwrap()) {
        Err(SaveRefusal::Command(error)) => (error.code, error.message),
        other => panic!("{:?}", other.map(|_| ())),
    };
    assert!(code == ANALYSIS_RECORD_UNBOUND || code == CONTENT_NOT_DERIVED, "{code}");
}

// --- The lease and the refusal formatting (§2.7 "Cost"; Codex r2 at #1673) --

/// Codex round 2 at #1673: formatting the refusal from a site whose pointer
/// runs through 124 enclosing keys of 4,000 bytes allocated 8,388,656 bytes,
/// beyond rule 3 (b)'s growth plus the site allowance. Route 010 prices that
/// formatting in its ingress lease (`refusal_formatting_bytes`). The measured
/// peak of detection, formatting and the save error, for that shape and the
/// worst-escaping variant (keys of `~`, whose pointer segments double), on the
/// model itself (rule 1) and inside embedded JSON (rule 3 (b)), stays within
/// the growth plus the site plus that allowance.
#[test]
fn route_010_prices_the_refusal_formatting_on_the_124_level_shape() {
    let nested = |levels: usize, key: &str, leaf: Value| {
        let mut value = leaf;
        for level in 0..levels {
            value = json!({ format!("{level:03}{key}"): value });
        }
        value
    };
    let site_allowance = |pointer: u64| {
        let envelope = |bytes: u64| crate::ingress::allocation_envelope(bytes).unwrap();
        envelope(pointer) + 2 * envelope(4 * 161)
    };
    let plain = "x".repeat(3_997);
    let tildes = "~".repeat(3_997);
    for (name, key) in [("4,000-byte keys", plain.as_str()), ("4,000-byte keys of ~", tildes.as_str())] {
        for (rule, model) in [
            ("rule 1, on the model", nested(124, key, json!({"library_ref": null}))),
            ("rule 3 (b), embedded", nested(124, key, json!(canonical_json(&json!({"library_kind": "hanger", "record_snapshot": {"invented": true}}))))),
        ] {
            let model_bytes = canonical_json(&model).len() as u64;
            let allowance = refusal_formatting_bytes(model_bytes, model_bytes);
            let registry = registry(ScriptedHost::normal());
            let mut lease = registry.acquire_interactive_ingress(1024).unwrap();
            let (error, measured) = crate::ingress::counting::measure(|| match refuse_library_referenced(&model, &mut lease) {
                Err(LibraryRefusal::Referenced(message)) => crate::report_package_save::coded_for_tests("/model", &message),
                other => panic!("{other:?}"),
            });
            let growth = lease.bytes() - 1024;
            let pointer = error.diagnostics[0].message.split(" holds ").next().unwrap().len() as u64;
            let bound = growth + site_allowance(pointer) + allowance;
            eprintln!(
                "{name}, {rule}: model {model_bytes} B, pointer {pointer} B, growth {growth} B, peak requested {} B, peak priced {} B, bound {bound} B (allowance {allowance} B)",
                measured.peak_requested, measured.peak_priced
            );
            assert!(error.code == LIBRARY_REFERENCED, "{}", error.code);
            assert!(measured.peak_priced <= bound, "{name}, {rule}: peak {} within {bound}", measured.peak_priced);
        }
    }
}

/// The whole route-010 admission of the coherent request, measured by the
/// counting allocator from the moved texts to the assembled package, stays
/// within route 010's lease growth (`ingress_lease_bytes`), which the lease
/// grows by before any text is parsed; the growth is refused one byte above
/// the budget (DEL-04-07).
#[test]
fn route_010_admission_stays_within_its_lease() {
    let basis = basis();
    let (registry, _) = basis.session(PreviewSolverMode::SparseInteractive);
    let args = basis.args();
    let growth = ingress_lease_bytes(&args).unwrap();
    let mut lease = registry.acquire_interactive_ingress(1024).unwrap();
    lease.grow_to(1024 + growth).unwrap();
    let (admitted, measured) = crate::ingress::counting::measure(|| admit(args, &registry, &mut lease).map(|admitted| admitted.package.export_blocked));
    assert_eq!(admitted.unwrap(), false);
    eprintln!("route 010 admission: growth {growth} B, peak requested {} B, peak priced {} B", measured.peak_requested, measured.peak_priced);
    assert!(measured.peak_priced <= growth, "peak {} within the growth {growth}", measured.peak_priced);
    // The growth is refused above the budget the lease was admitted under.
    let (pool_bytes, budget) = registry.pool().settings();
    registry.configure_pool(pool_bytes, 1024 + growth - 1, false).unwrap();
    let mut low = registry.acquire_interactive_ingress(1024).unwrap();
    assert_eq!(
        low.grow_to(1024 + growth),
        Err(crate::memory_pool::ResourceRefusal::IngressExceedsBudget { required_bytes: 1024 + growth, budget_bytes: 1024 + growth - 1 })
    );
    registry.configure_pool(pool_bytes, budget, false).unwrap();
}

// --- The render command (§2.8) ----------------------------------------------

/// The render command refuses, before it decodes, a pressure-free input under
/// `user_released_report`, and a `local_private` input with a record, with no
/// body.
#[test]
fn the_render_command_refuses_a_user_released_input() {
    let basis = basis();
    let report = basis.request()["report"].clone();
    let error = crate::render_calculation_report_value(report.clone()).unwrap_err();
    assert!(error.starts_with(&format!("{}:", crate::RENDER_USER_RELEASE_PACKAGE_ONLY)), "{error}");
    let mut local = report.clone();
    local["report_context"]["export_context"] = json!("local_private");
    let error = crate::render_calculation_report_value(local).unwrap_err();
    assert!(error.starts_with(&format!("{}:", crate::RENDER_USER_RELEASE_PACKAGE_ONLY)) && error.contains("a user_release record"), "{error}");
    let mut null_record = report;
    null_record["report_context"]["export_context"] = json!("public_report");
    null_record["report_context"]["user_release"] = Value::Null;
    assert!(crate::render_calculation_report_value(null_record).unwrap_err().starts_with(crate::RENDER_USER_RELEASE_PACKAGE_ONLY));
}

/// `user_release_render_admits_only_the_user_released_class`, the native
/// cases (§6): the derived request's session provenance is the user-released
/// class, so route 010 renders it (`route_010_admits_saves_and_reads_back_a_user_released_package`);
/// a record of another non-public class is refused natively (step 6); and a
/// record written in positional form is refused by the object-form check at
/// the native render and at the save, and by the producer.
#[test]
fn user_release_render_admits_only_the_user_released_class() {
    let basis = basis();
    let (registry, _) = basis.session(PreviewSolverMode::SparseInteractive);
    let mut other_class = basis.request();
    other_class["report"]["report_sections"]["limitations"][0]["provenance"]["privacy_classification"] = json!("private_material_data");
    refused(basis.args_for(&other_class, &basis.model, &basis.manifest), &registry, PRIVATE_CLASS);
    // A positional record.
    let mut positional = basis.request();
    let record = positional["report"]["report_context"]["user_release"].take();
    let object = record.as_object().unwrap();
    positional["report"]["report_context"]["user_release"] = json!(
        ["record_kind", "record_version", "statement", "statement_basis", "standing", "scope", "bound_to", "user_released_at"]
            .iter()
            .map(|key| object[*key].clone())
            .collect::<Vec<_>>()
    );
    let error = crate::report_render_exact::check_object_form(&positional["report"]).unwrap_err();
    assert!(error.starts_with("REPORT-RENDER-INPUT-POSITIONAL:") && error.contains("user_release"), "{error}");
    let (_, message) = refused(basis.args_for(&positional, &basis.model, &basis.manifest), &registry, crate::report_package_save::REQUEST_POSITIONAL);
    assert!(message.contains("user_release"), "{message}");
}
