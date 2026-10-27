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

/// Admission's refusal `code`, with its first path and its message. The lease
/// is grown by the route-aware reservation first, as the command grows it,
/// and admission's measured peak must stay within that growth and any growth
/// admission makes itself (Codex round 1 at #1682, P1: every route-010
/// refusal within its reservation).
#[track_caller]
fn refused(args: ReportPackageSaveRequest, registry: &SolveJobRegistry, code: &str) -> (String, String) {
    let mut lease = registry.acquire_interactive_ingress(1024).unwrap();
    let growth = ingress_lease_bytes(&args).ok();
    if let Some(growth) = growth {
        lease.grow_to(1024 + growth).unwrap();
    }
    let reserved = lease.bytes();
    let (outcome, measured) = crate::ingress::counting::measure(|| admit(args, registry, &mut lease));
    if let Some(growth) = growth {
        let grown = lease.bytes() - reserved;
        assert!(measured.peak_priced <= growth + grown, "{code}: peak {} within the reservation {}", measured.peak_priced, growth + grown);
    }
    match outcome {
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
    // A library-applied material, and its provenance string alone (rule 2);
    // the detector's own cases are `user_release_admission`'s. Since Codex
    // round 1 at #1682 (P1) route 010's disposition screen reads no library
    // store: it refuses the object `library_ref` itself, at its pointer,
    // before step 5's rule 1 would name it (`/model`, "holds a library origin
    // (rule library_ref"), with the same code.
    let mut library = basis.model.clone();
    library["materials"][0]["provenance_record"] = json!({"library_ref": {"library_kind": "material", "library_id": "matlib.invented", "library_version": "1.0.0", "record_id": "steel.1"}});
    let (request, manifest) = rebound(&basis, &basis.request(), &library);
    let (path, message) = refused(basis.args_for(&request, &library, &manifest), &registry, LIBRARY_REFERENCED);
    assert_eq!(path, "/model/materials/0/provenance_record/library_ref");
    assert!(message.contains("is a library reference") && message.contains("does not read the local library store"), "{message}");
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

// --- Many findings (Codex round 1 at #1682, P1) -----------------------------

/// Codex round 1 at #1682 (P1): `model.extra` holds `levels` nested objects,
/// each keyed by `format!("{i:03}{}", "x".repeat(key_bytes - 3))`, and at the
/// leaf an array of `findings` objects `{"redistribution_status":
/// "protected_suspected"}`.
fn deep_findings(levels: usize, key_bytes: usize, findings: usize) -> Value {
    let mut value = json!(vec![json!({"redistribution_status": "protected_suspected"}); findings]);
    for level in (0..levels).rev() {
        value = json!({ format!("{level:03}{}", "x".repeat(key_bytes - 3)): value });
    }
    value
}

/// The wide variant: `findings` blocking objects at depth 2 (`/extra/<i>`).
fn wide_findings(findings: usize) -> Value {
    json!(vec![json!({"redistribution_status": "protected_suspected"}); findings])
}

/// The basis model with `extra`, and the coherent request and input manifest
/// rebound to it (steps 1-3 hold up to the disposition screen).
fn with_extra(basis: &Basis, extra: Value) -> (Value, Value, Value) {
    let mut model = basis.model.clone();
    model["extra"] = extra;
    let (request, manifest) = rebound(basis, &basis.request(), &model);
    (request, model, manifest)
}

/// The shapes of the P1 regressions: Codex's, scaled 2x and 10x (keys and
/// findings), and the wide variant.
fn many_finding_shapes() -> Vec<(&'static str, Value)> {
    vec![
        ("80 levels of 4,000-byte keys, 128 findings (Codex's repro)", deep_findings(80, 4_000, 128)),
        ("80 levels of 8,000-byte keys, 256 findings (2x)", deep_findings(80, 8_000, 256)),
        ("80 levels of 40,000-byte keys, 1,280 findings (10x)", deep_findings(80, 40_000, 1_280)),
        ("100,000 findings at depth 2 (wide)", wide_findings(100_000)),
    ]
}

/// What one route-010 admission measured through the raw command.
struct Admission {
    outcome: Result<crate::report_package_save::AdmittedReportPackage, SaveRefusal>,
    /// The invocation budget: the ingress lease plus the route-aware growth,
    /// the complete calculated reservation.
    reservation: u64,
    /// The route-aware growth (`ingress_lease_bytes`), and what admission
    /// grew the lease by itself (rule 3 (b)).
    growth: u64,
    grown_in_admission: u64,
    measured: crate::ingress::counting::Measured,
}

/// Admits `args` as the command does: `command_ingress::admit` with a
/// `save_report_package` body, under an invocation budget set to the complete
/// calculated reservation, the lease grown by `ingress_lease_bytes`, then
/// admission with this session's attested solve of the basis, measured by the
/// counting allocator from the moved texts to its outcome.
fn admit_through_the_command(basis: &Basis, args: ReportPackageSaveRequest) -> Admission {
    let mut body = args_body(&args);
    body["analysisRunCanonicalText"] = json!(args.analysis_run_text);
    // The reservation, on a first registry; the budget, on the one admitted.
    let reservation = {
        let registry = registry(ScriptedHost::normal());
        let (mut decoded, lease) = admit_json(&registry, "save_report_package", ArgsDecoding::Moved, &body).unwrap();
        let request = crate::report_package_save::save_report_package_args(&mut decoded).unwrap();
        lease.bytes() + ingress_lease_bytes(&request).unwrap()
    };
    let (registry, _) = basis.session(PreviewSolverMode::SparseInteractive);
    let (pool_bytes, _) = registry.pool().settings();
    registry.configure_pool(pool_bytes, reservation, false).unwrap();
    let (mut decoded, mut lease) = admit_json(&registry, "save_report_package", ArgsDecoding::Moved, &body).unwrap();
    drop(body);
    let request = crate::report_package_save::save_report_package_args(&mut decoded).unwrap();
    drop(decoded);
    let growth = ingress_lease_bytes(&request).unwrap();
    assert_eq!(lease.bytes() + growth, reservation);
    lease.grow_to(reservation).unwrap();
    let (outcome, measured) = crate::ingress::counting::measure(|| admit(request, &registry, &mut lease));
    let grown_in_admission = lease.bytes() - reservation;
    Admission { outcome, reservation, growth, grown_in_admission, measured }
}

/// Codex round 1 at #1682 (P1): on its repro (80 levels of 4,000-byte keys
/// over 128 blocking findings) route 010's disposition screen collected every
/// finding with its full pointer and joined them into one message, a peak of
/// 272,569,728 priced bytes against a reservation of 52,842,048. Route 010's
/// screen now stops at its first finding and builds only that finding's
/// pointer, and the refusal carries that one diagnostic, cut to the route's
/// bounds (`report_package_save::ROUTE_010_PATH_BYTES` and
/// `ROUTE_010_DETAIL_BYTES`). For the repro, the shape scaled 2x and 10x (keys
/// and findings) and the wide variant, admitted through the raw command under
/// a budget of exactly its reservation, the measured peak stays within the
/// growth the lease was given, and the refusal is the first finding's alone.
#[test]
fn route_010_refuses_many_findings_within_its_reservation() {
    let basis = basis();
    for (name, extra) in many_finding_shapes() {
        let (request, model, manifest) = with_extra(&basis, extra);
        let admission = admit_through_the_command(&basis, basis.args_for(&request, &model, &manifest));
        let Admission { outcome, reservation, growth, grown_in_admission, measured } = admission;
        eprintln!(
            "route 010, {name}: reservation {reservation} B (growth {growth} B, grown in admission {grown_in_admission} B), peak requested {} B, peak priced {} B",
            measured.peak_requested, measured.peak_priced
        );
        assert!(measured.peak_priced <= growth + grown_in_admission, "{name}: peak {} within {}", measured.peak_priced, growth + grown_in_admission);
        match outcome {
            Err(SaveRefusal::Command(error)) => {
                assert_eq!(error.code, crate::report_package_save::PROTECTED_CONTENT_BLOCKED, "{name}: {}", error.message);
                assert_eq!(error.diagnostics.len(), 1, "{name}");
                let path = &error.diagnostics[0].path;
                assert!(path.len() <= crate::report_package_save::ROUTE_010_PATH_BYTES, "{name}: {} B", path.len());
                assert!(path.starts_with("/model/extra"), "{name}: {path}");
                let serialized = serde_json::to_string(&error).unwrap();
                assert!(serialized.len() <= 16 * 1024, "{name}: the refusal serializes to {} B", serialized.len());
                eprintln!("route 010, {name}: {} ({} B serialized)", error.message.chars().take(200).collect::<String>(), serialized.len());
            }
            Err(SaveRefusal::Resource(refusal)) => {
                assert!(matches!(refusal, crate::memory_pool::ResourceRefusal::IngressExceedsBudget { .. }), "{name}: {refusal:?}");
            }
            Ok(_) => panic!("{name}: admitted"),
        }
    }
}

/// The other route-010 refusals whose text names a site (Codex round 1 at
/// #1682, P1): a rule-pack reference (step 5), a non-project private class
/// (step 6), a library provenance string (step 5, rule 2) and an object
/// `library_ref` (refused by route 010's screen) at the end of 80 levels of
/// 4,000-byte keys, and the rule-pack agreement over 10,000 package entries.
/// Each is admitted through the raw command under a budget of exactly its
/// reservation, refused with its code within that reservation, and bounded.
#[test]
fn route_010_refuses_each_deep_site_within_its_reservation() {
    let basis = basis();
    let deep = |leaf: Value| {
        let mut value = leaf;
        for level in (0..80).rev() {
            value = json!({ format!("{level:03}{}", "x".repeat(3_997)): value });
        }
        value
    };
    let mut cases: Vec<(&str, ReportPackageSaveRequest, &str)> = Vec::new();
    for (name, leaf, code) in [
        ("a rule-pack reference", json!({"rule_pack_refs": [{"rule_pack_id": "invented-pack", "version": "1.0.0"}]}), RULE_PACK_REFERENCED),
        ("a non-project private class", json!({"privacy_class": "private_material_data"}), PRIVATE_CLASS),
        ("a library provenance string", json!({"note": "library:material:matlib.invented#steel.1"}), LIBRARY_REFERENCED),
        ("an object library_ref", json!({"library_ref": {"library_kind": "material", "library_id": "matlib.invented", "record_id": "steel.1"}}), LIBRARY_REFERENCED),
    ] {
        let (request, model, manifest) = with_extra(&basis, deep(leaf));
        cases.push((name, basis.args_for(&request, &model, &manifest), code));
    }
    let mut request = basis.request();
    request["report"]["calculation_report"]["rule_pack_refs"] = json!((0..10_000)
        .map(|index| json!({"rule_pack_id": format!("invented-pack-{index:05}"), "version": "1.0.0", "checksum": {"value": "0".repeat(64)}}))
        .collect::<Vec<_>>());
    cases.push(("10,000 package rule packs", basis.args_for(&request, &basis.model, &basis.manifest), crate::report_package_save::RULE_PACKS_DISAGREE));
    for (name, args, code) in cases {
        let Admission { outcome, reservation, growth, grown_in_admission, measured } = admit_through_the_command(&basis, args);
        eprintln!(
            "route 010, {name}: reservation {reservation} B (growth {growth} B, grown in admission {grown_in_admission} B), peak requested {} B, peak priced {} B",
            measured.peak_requested, measured.peak_priced
        );
        assert!(measured.peak_priced <= growth + grown_in_admission, "{name}: peak {} within {}", measured.peak_priced, growth + grown_in_admission);
        match outcome {
            Err(SaveRefusal::Command(error)) => {
                assert_eq!(error.code, code, "{name}: {}", error.message);
                assert!(error.diagnostics.len() <= 1, "{name}");
                assert!(error.message.len() <= crate::report_package_save::ROUTE_010_DETAIL_BYTES, "{name}: {} B", error.message.len());
                eprintln!("route 010, {name}: {}", error.message.chars().take(240).collect::<String>());
            }
            other => panic!("{name}: {:?}", other.map(|_| ())),
        }
    }
}

/// The derivation's refusal names the first differing pointer as a refusal
/// shows it (240 characters), and builds no more of it (Codex round 1 at
/// #1682, P1): a difference below 80 levels of 4,000-byte keys is found and
/// reported holding only the walk's key sets and the shown text.
#[test]
fn the_derivation_refusal_holds_no_long_pointer() {
    let deep = |leaf: Value| {
        let mut value = leaf;
        for level in (0..80).rev() {
            value = json!({ format!("{level:03}{}", "x".repeat(3_997)): value });
        }
        value
    };
    let derivation = open_pipe_stress_report_package::user_release::UserReleaseDerivation {
        input_manifest: deep(json!(1)),
        input_manifest_sha256: String::new(),
        input_manifest_ref: String::new(),
        request: json!({}),
    };
    let found = deep(json!(2));
    let (refusal, measured) = crate::ingress::counting::measure(|| open_pipe_stress_report_package::user_release::check_matches_derivation(&derivation, &found, &json!({})));
    let refusal = refusal.unwrap_err();
    eprintln!("derivation refusal: {} B, peak requested {} B, peak priced {} B", refusal.len(), measured.peak_requested, measured.peak_priced);
    assert!(refusal.starts_with(CONTENT_NOT_DERIVED) && refusal.ends_with('\u{2026}'), "{refusal}");
    assert!(measured.peak_priced <= 64 * 1024, "{measured:?}");
}

/// A route-010 refusal is bounded (`report_package_save::bound_route_010_refusal`):
/// one diagnostic, its path an enclosing pointer within the path bound, its
/// message and the joined message within the detail bound, the code within
/// its bound; a refusal within the bounds is unchanged.
#[test]
fn route_010_refusals_are_bounded() {
    use crate::report_package_save::{bound_route_010_refusal_for_tests as bound, ReportPackageSaveCommandError, ROUTE_010_DETAIL_BYTES, ROUTE_010_PATH_BYTES};
    use crate::export_disposition_screen::DispositionDiagnostic;
    let item = |path: &str, message: &str| DispositionDiagnostic { code: "INVENTED-CODE".to_string(), path: path.to_string(), message: message.to_string() };
    let short = ReportPackageSaveCommandError::refused(vec![item("/model/a", "short")]);
    let unchanged = bound(ReportPackageSaveCommandError::refused(vec![item("/model/a", "short")]));
    assert_eq!(serde_json::to_value(&unchanged).unwrap(), serde_json::to_value(&short).unwrap());
    let long_key = "é".repeat(700);
    let path = format!("/model/extra/{long_key}/b~1c/0/redistribution_status");
    let long_message = format!("{path} holds a finding; {}", "and more text ".repeat(400));
    let error = bound(ReportPackageSaveCommandError::refused(vec![item(&path, &long_message), item("/model/b", "second"), item("/model/c", "third")]));
    assert_eq!(error.diagnostics.len(), 1);
    let first = &error.diagnostics[0];
    assert_eq!(first.path, "/model/extra", "the nearest enclosing pointer within the bound");
    assert!(path.starts_with(&format!("{}/", first.path)));
    assert!(first.message.len() <= ROUTE_010_DETAIL_BYTES && error.message.len() <= ROUTE_010_DETAIL_BYTES);
    assert!(first.message.contains("bytes elided") && first.message.ends_with("(2 further findings are not listed.)"), "{}", first.message);
    assert!(first.message.contains(&format!("The finding's JSON pointer is {} bytes", path.len())), "{}", first.message);
    assert!(error.message.starts_with("INVENTED-CODE at /model/extra: "), "{}", error.message);
    let mid = format!("/model/{}/{}", "a".repeat(600), "b".repeat(600));
    assert_eq!(bound(ReportPackageSaveCommandError::refused(vec![item(&mid, "x")])).diagnostics[0].path, format!("/model/{}", "a".repeat(600)));
    assert!(bound(ReportPackageSaveCommandError::refused(vec![item(&mid, "x")])).diagnostics[0].path.len() <= ROUTE_010_PATH_BYTES);
    let bridge = bound(ReportPackageSaveCommandError::bridge(format!("INVENTED-CODE: {}", "y".repeat(10_000))));
    assert!(bridge.message.len() <= ROUTE_010_DETAIL_BYTES && bridge.message.starts_with("INVENTED-CODE: yyy") && bridge.code == "INVENTED-CODE");
    let codeless = bound(ReportPackageSaveCommandError::bridge("z".repeat(10_000)));
    assert!(codeless.code.len() <= 128 && codeless.message.len() <= ROUTE_010_DETAIL_BYTES);
}

/// Route 009 with the same inputs refuses with exactly the bytes it gave
/// before route 010's bounded screen (pinned at `031723bd24`): every finding,
/// with its full pointer, in one joined message. The 10x shape is not run on
/// route 009: its unbounded refusal would request about 16 GB.
#[test]
fn route_009_refusals_of_many_findings_are_unchanged() {
    let basis = basis();
    let pinned: [(&str, usize, usize, &str); 3] = [
        ("80 levels of 4,000-byte keys, 128 findings (Codex's repro)", 128, 81_990_707, "d2a00d98e0c09b5537e906bdd186c4e164067e9e24d91a44a7a611cabccf3417"),
        ("80 levels of 8,000-byte keys, 256 findings (2x)", 256, 327_821_107, "d90af1685d7b6f28cab3abf797abf2922b854a598fad6dea6de61dd0387219ca"),
        ("100,000 findings at depth 2 (wide)", 100_000, 39_378_307, "f87a6476a65425f2fb59dc4316fb9f354b596fe7c02cb80217c4933223c00590"),
    ];
    let shapes = many_finding_shapes();
    let mut measured = Vec::new();
    for (name, _, _, _) in pinned {
        let extra = shapes.iter().find(|(shape, _)| *shape == name).unwrap().1.clone();
        let (mut request, model, manifest) = with_extra(&basis, extra);
        request["export_profile_id"] = json!(open_pipe_stress_report_package::bindings::DESKTOP_PROFILE);
        request["report"]["report_context"]["export_context"] = json!("local_private");
        request["report"]["report_context"].as_object_mut().unwrap().remove("user_release");
        let args = save_request_with(&request, &model, &manifest);
        let error = crate::report_package_save::admit_report_package(args, &software(), &mut no_store).unwrap_err();
        assert_eq!(error.code, crate::report_package_save::PROTECTED_CONTENT_BLOCKED);
        let serialized = serde_json::to_string(&error).unwrap();
        eprintln!("route 009, {name}: {} diagnostics, {} B, sha256 {}", error.diagnostics.len(), serialized.len(), sha256_hex(&serialized));
        measured.push((error.diagnostics.len(), serialized.len(), sha256_hex(&serialized)));
    }
    for ((name, diagnostics, bytes, sha256), (found, length, digest)) in pinned.iter().zip(&measured) {
        assert_eq!(found, diagnostics, "{name}");
        assert_eq!((*length, digest.as_str()), (*bytes, *sha256), "{name}: route 009's refusal bytes");
    }
}

// --- Every route-010 refusal leaves bounded (Codex round 2 at #1682, P2) ----

/// Each string field of a save error, with its route-010 bound: the error is
/// destructured whole, so a field added to it fails to compile here until it
/// is bounded and listed.
fn string_fields(error: &crate::report_package_save::ReportPackageSaveCommandError) -> Vec<(&'static str, usize, usize)> {
    use crate::report_package_save::{
        ReportPackageSaveCommandError, ROUTE_010_BASENAME_BYTES, ROUTE_010_DETAIL_BYTES, ROUTE_010_LABEL_BYTES, ROUTE_010_NOTE_BYTES, ROUTE_010_PATH_BYTES,
    };
    let ReportPackageSaveCommandError {
        code,
        stage,
        message,
        os_error_kind,
        os_error_code: _,
        cleanup_code,
        cleanup_message,
        selected_basename,
        path_containment,
        limitation,
        diagnostics,
    } = error;
    let optional = |text: &Option<String>| text.as_ref().map_or(0, String::len);
    let mut fields = vec![
        ("code", code.len(), 128),
        ("stage", stage.len(), ROUTE_010_LABEL_BYTES),
        ("message", message.len(), ROUTE_010_DETAIL_BYTES),
        ("os_error_kind", optional(os_error_kind), ROUTE_010_LABEL_BYTES),
        ("cleanup_code", optional(cleanup_code), 128),
        ("cleanup_message", optional(cleanup_message), ROUTE_010_NOTE_BYTES),
        ("selected_basename", selected_basename.len(), ROUTE_010_BASENAME_BYTES),
        ("path_containment", path_containment.len(), ROUTE_010_LABEL_BYTES),
        ("limitation", limitation.len(), ROUTE_010_NOTE_BYTES),
    ];
    for item in diagnostics {
        fields.push(("diagnostic code", item.code.len(), 128));
        fields.push(("diagnostic path", item.path.len(), ROUTE_010_PATH_BYTES));
        fields.push(("diagnostic message", item.message.len(), ROUTE_010_DETAIL_BYTES));
    }
    fields
}

/// `error` as the save command returns it on route 010 (`SaveRoute::exit`),
/// checked against every bound: at most one diagnostic, each string field
/// within its bound, the text within `ROUTE_010_REFUSAL_TEXT_BYTES` and the
/// serialized command error within `ROUTE_010_REFUSAL_JSON_BYTES`. Returns
/// the bounded error's serialized command error.
#[track_caller]
fn leaves_bounded(name: &str, error: crate::report_package_save::ReportPackageSaveCommandError) -> (String, crate::report_package_save::ReportPackageSaveCommandError) {
    use crate::command_ingress::CommandError;
    use crate::report_package_save::{SaveRoute, ROUTE_010_REFUSAL_JSON_BYTES, ROUTE_010_REFUSAL_TEXT_BYTES};
    let outcome: Result<(), _> = SaveRoute::UserReleased.exit(Err(CommandError::Command(error)));
    let Err(exited) = outcome else { unreachable!() };
    let serialized = serde_json::to_string(&exited).unwrap();
    let CommandError::Command(exited) = exited else { panic!("{name}: the save's own error left as another kind") };
    let bounded = exited.error();
    assert!(bounded.diagnostics.len() <= 1, "{name}: {} diagnostics", bounded.diagnostics.len());
    let fields = string_fields(bounded);
    for (field, length, bound) in &fields {
        assert!(length <= bound, "{name}: {field} is {length} B, above {bound} B");
    }
    let text: usize = fields.iter().map(|(_, length, _)| length).sum();
    assert!(text <= ROUTE_010_REFUSAL_TEXT_BYTES, "{name}: {text} B of text");
    assert!(serialized.len() <= ROUTE_010_REFUSAL_JSON_BYTES, "{name}: serialized {} B", serialized.len());
    assert_eq!(serialized, serde_json::to_string(bounded).unwrap(), "{name}: the command error serializes as the save error it holds");
    eprintln!("route 010, {name}: {text} B of text, {} B serialized", serialized.len());
    let bounded = serde_json::from_str::<Value>(&serialized).unwrap();
    (serialized, rebuilt(&bounded))
}

/// A save error read back from its serialized form (for comparing fields).
fn rebuilt(value: &Value) -> crate::report_package_save::ReportPackageSaveCommandError {
    let text = |key: &str| value[key].as_str().map(str::to_string);
    crate::report_package_save::ReportPackageSaveCommandError {
        code: text("code").unwrap(),
        stage: text("stage").unwrap(),
        message: text("message").unwrap(),
        os_error_kind: text("os_error_kind"),
        os_error_code: value["os_error_code"].as_i64().map(|code| code as i32),
        cleanup_code: text("cleanup_code"),
        cleanup_message: text("cleanup_message"),
        selected_basename: text("selected_basename").unwrap(),
        path_containment: text("path_containment").unwrap(),
        limitation: text("limitation").unwrap(),
        diagnostics: value["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| crate::export_disposition_screen::DispositionDiagnostic {
                code: item["code"].as_str().unwrap().to_string(),
                path: item["path"].as_str().unwrap().to_string(),
                message: item["message"].as_str().unwrap().to_string(),
            })
            .collect(),
    }
}

/// Route 009 returns `error` through the same exit unchanged, byte for byte.
#[track_caller]
fn route_009_leaves_unchanged(name: &str, error: crate::report_package_save::ReportPackageSaveCommandError) {
    use crate::command_ingress::CommandError;
    let before = serde_json::to_string(&error).unwrap();
    let outcome: Result<(), _> = crate::report_package_save::SaveRoute::LocalPrivate.exit(Err(CommandError::Command(error)));
    let after = serde_json::to_string(&outcome.unwrap_err()).unwrap();
    assert_eq!(after, before, "{name}: route 009's error changed");
}

/// Codex round 2 at #1682 (P2): the refusal bound covered admission only, so
/// an error of the save dialog or of the atomic writer left whole. Codex's
/// repro, against the production atomic writer: a 50,000-byte basename under
/// `/tmp`, refused before any file is written, kept 50,407 bytes of strings
/// (50,588 serialized). Through the route's one exit it is bounded, the
/// basename keeping its head and its extension; route 009 returns it
/// unchanged.
#[test]
fn route_010_atomic_writer_refusals_leave_bounded() {
    let destination = std::path::PathBuf::from(format!("/tmp/{}.opsproj", "x".repeat(50_000)));
    let unbounded = || crate::report_package_save::ReportPackageSaveCommandError::from(crate::atomic_report_package_save::atomic_save_bytes(&destination, b"").unwrap_err());
    let error = unbounded();
    let strings: usize = string_fields(&error).iter().map(|(_, length, _)| length).sum();
    eprintln!("Codex's repro, unbounded: {strings} B of strings, {} B serialized", serde_json::to_string(&error).unwrap().len());
    assert!(strings > 50_000, "{strings}");
    assert!(!destination.exists());
    let (_, bounded) = leaves_bounded("Codex's repro", error);
    assert!(bounded.selected_basename.starts_with("xxxx") && bounded.selected_basename.ends_with("xxxx.opsproj"), "{}", bounded.selected_basename);
    assert!(bounded.selected_basename.contains("bytes elided"), "{}", bounded.selected_basename);
    let original = unbounded();
    assert_eq!((bounded.code.as_str(), bounded.stage.as_str(), bounded.os_error_code), (original.code.as_str(), original.stage.as_str(), original.os_error_code));
    route_009_leaves_unchanged("Codex's repro", unbounded());
}

/// A long OS error (Codex round 2 at #1682, P2): the writer's message is the
/// OS error's text and its cleanup message another's, and either may be long;
/// every string field the writer fills is bounded on route 010, and route 009
/// returns it unchanged.
#[test]
fn route_010_long_os_errors_leave_bounded() {
    let long = |unit: &str, count: usize| unit.repeat(count);
    let error = || {
        crate::report_package_save::ReportPackageSaveCommandError::from(crate::atomic_report_package_save::AtomicSaveError {
            code: format!("REPORT-PACKAGE-{}", long("C", 1_000)),
            stage: long("s", 1_000),
            message: format!("{} (os error 5)", long("é", 30_000)),
            os_error_kind: Some(long("K", 1_000)),
            os_error_code: Some(5),
            cleanup_code: Some(long("D", 1_000)),
            cleanup_message: Some(long("\u{1}", 30_000)),
            selected_basename: format!("{}.opsproj", long("名", 20_000)),
            path_containment: long("p", 1_000),
            limitation: long("l", 30_000),
        })
    };
    let (_, bounded) = leaves_bounded("a long OS error", error());
    assert!(bounded.message.ends_with("(os error 5)") && bounded.selected_basename.ends_with(".opsproj"), "{} {}", bounded.message, bounded.selected_basename);
    assert_eq!(bounded.os_error_code, Some(5));
    route_009_leaves_unchanged("a long OS error", error());
}

/// The save dialog's refusals on route 010 (Codex round 2 at #1682, P2),
/// through the command's admission and save (`save_report_package_session`)
/// and its one exit: an unusable path whose text is long, and a chosen path
/// with a long basename, which the production atomic writer refuses before
/// any file is written. Route 009's form of each is unchanged by the exit.
#[test]
fn route_010_picker_refusals_leave_bounded() {
    use crate::report_package_save::{save_report_package_session, SaveRefusal, PICKER_PATH_INVALID};
    let basis = basis();
    let dir = sandbox("user-release-picker");
    let long_name = dir.join(format!("{}.opsproj", "名".repeat(20_000)));
    let picks: Vec<(&str, Box<dyn Fn() -> Option<Result<std::path::PathBuf, String>>>)> = vec![
        ("an unusable path with a long text", Box::new(|| Some(Err(format!("{} is not a file path", "y".repeat(50_000)))))),
        ("a picked path with a long basename", Box::new(|| Some(Ok(long_name.clone())))),
    ];
    for (name, pick) in picks {
        let (registry, mut lease) = basis.session(PreviewSolverMode::SparseInteractive);
        let mut session = UserReleaseSession { registry: &registry, lease: &mut lease };
        let outcome = save_report_package_session(basis.args(), &software(), &mut no_store, &crate::report_package_save::no_custody, &mut session, |_| pick());
        let error = match outcome {
            Err(SaveRefusal::Command(error)) => error,
            other => panic!("{name}: {:?}", other.map(|receipt| receipt.outcome)),
        };
        let unbounded = serde_json::to_string(&error).unwrap().len();
        assert!(unbounded > crate::report_package_save::ROUTE_010_REFUSAL_JSON_BYTES / 2, "{name}: {unbounded} B");
        let same = rebuilt(&serde_json::to_value(&error).unwrap());
        let (_, bounded) = leaves_bounded(name, error);
        if name.starts_with("an unusable") {
            assert_eq!(bounded.code, PICKER_PATH_INVALID);
            assert!(bounded.message.starts_with(&format!("{PICKER_PATH_INVALID}: yyy")) && bounded.message.ends_with("is not a file path"), "{}", bounded.message);
        } else {
            assert!(bounded.code.starts_with("REPORT-PACKAGE-"), "{}", bounded.code);
            assert!(bounded.selected_basename.ends_with(".opsproj"), "{}", bounded.selected_basename);
        }
        route_009_leaves_unchanged(name, same);
    }
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 0, "nothing was written");
}

/// Every form of the save command's error, at its largest, leaves route 010
/// within the stated bounds (Codex round 2 at #1682, P2): the command error's
/// every variant is enumerated (a variant added fails to compile here), and
/// the save's own error is built by each of its constructors (a
/// multi-finding refusal, one coded finding, a producer or bridge message,
/// the atomic writer's error) and as a literal with every string field large
/// and of characters JSON escapes to six bytes. Bounding allocates within
/// `ROUTE_010_BOUNDED_REFUSAL_BYTES`, which route 010's refusal formatting
/// allowance prices.
#[test]
fn every_route_010_error_form_leaves_within_its_bound() {
    use crate::command_ingress::CommandError;
    use crate::export_disposition_screen::DispositionDiagnostic;
    use crate::report_package_save::{ReportPackageSaveCommandError, SaveRoute, ROUTE_010_BOUNDED_REFUSAL_BYTES};
    let escaped = |count: usize| "\u{1}".repeat(count);
    let item = |count: usize| DispositionDiagnostic { code: escaped(count), path: format!("/model/{}", escaped(count)), message: escaped(count) };
    let forms: Vec<(&str, Box<dyn Fn() -> ReportPackageSaveCommandError>)> = vec![
        ("refused, three large findings", Box::new(move || ReportPackageSaveCommandError::refused(vec![item(40_000), item(40_000), item(40_000)]))),
        ("refused, no finding", Box::new(|| ReportPackageSaveCommandError::refused(Vec::new()))),
        ("one coded finding", Box::new(move || crate::report_package_save::coded_for_tests(&format!("/model/{}", escaped(40_000)), &format!("{}: {}", escaped(40_000), escaped(40_000))))),
        ("a producer or bridge message", Box::new(move || ReportPackageSaveCommandError::bridge(format!("{}: {}", escaped(40_000), escaped(40_000))))),
        (
            "the atomic writer's error",
            Box::new(move || {
                ReportPackageSaveCommandError::from(crate::atomic_report_package_save::AtomicSaveError {
                    code: escaped(40_000),
                    stage: escaped(40_000),
                    message: escaped(40_000),
                    os_error_kind: Some(escaped(40_000)),
                    os_error_code: Some(i32::MIN),
                    cleanup_code: Some(escaped(40_000)),
                    cleanup_message: Some(escaped(40_000)),
                    selected_basename: escaped(40_000),
                    path_containment: escaped(40_000),
                    limitation: escaped(40_000),
                })
            }),
        ),
        (
            "every field large",
            Box::new(move || ReportPackageSaveCommandError {
                code: escaped(40_000),
                stage: escaped(40_000),
                message: escaped(40_000),
                os_error_kind: Some(escaped(40_000)),
                os_error_code: Some(i32::MIN),
                cleanup_code: Some(escaped(40_000)),
                cleanup_message: Some(escaped(40_000)),
                selected_basename: escaped(40_000),
                path_containment: escaped(40_000),
                limitation: escaped(40_000),
                diagnostics: vec![item(40_000), item(40_000)],
            }),
        ),
    ];
    let mut largest = 0;
    for (name, form) in &forms {
        let (serialized, _) = leaves_bounded(name, form());
        largest = largest.max(serialized.len());
        let error = form();
        let (_, measured) = crate::ingress::counting::measure(|| SaveRoute::UserReleased.exit::<()>(Err(CommandError::Command(error))));
        assert!(measured.peak_priced <= ROUTE_010_BOUNDED_REFUSAL_BYTES, "{name}: bounding peaked at {} B", measured.peak_priced);
        route_009_leaves_unchanged(name, form());
    }
    eprintln!("the largest route-010 command error: {largest} B serialized");
    // The other variants are the ingress layer's and the response's, the same
    // on every route; the exit leaves them unchanged.
    let registry = registry(ScriptedHost::normal());
    let others: Vec<CommandError<ReportPackageSaveCommandError>> = vec![
        CommandError::Ingress(registry.refuse(crate::memory_pool::ResourceRefusal::IngressExceedsBudget { required_bytes: u64::MAX, budget_bytes: u64::MAX })),
        CommandError::Response("invented response error".to_string()),
    ];
    for other in others {
        let before = serde_json::to_string(&other).unwrap();
        let after = SaveRoute::UserReleased.exit::<()>(Err(other)).unwrap_err();
        assert_eq!(serde_json::to_string(&after).unwrap(), before);
        match after {
            CommandError::Ingress(_) | CommandError::Response(_) => {}
            CommandError::Command(_) => panic!("an ingress or response error left as the save's own"),
        }
    }
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
