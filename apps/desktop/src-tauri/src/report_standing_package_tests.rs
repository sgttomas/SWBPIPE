//! DEL-08-01 REQ-024, "Package and user release" (Agent 0, 20 November 2026): on
//! the exact route the native package save and the user release admit every
//! result the rendered report admits, and the saved package states the standing
//! the rendered report states for the same result.
//!
//! The two committed connecting-path witnesses
//! (`fixtures/reports/invented/report_standing_witnesses/`): a gap case, every
//! load case quality unresolved, and a coupled sliding friction case,
//! sensitive. Each is solved but not numerically eligible. A test registry job
//! delivers the committed raw result for the committed intake text. From a job
//! holding the same bytes natively the native run-record builder builds the run
//! record; the native
//! user-release derivation derives the user-released request, and the
//! local-private request is composed from it with the committed render input as
//! its report. Each is required equal, by its canonical SHA-256, to the request
//! the desktop's web view built for the same result
//! (`reportStandingWitness.test.tsx` writes the digests), and is then admitted,
//! saved and read back through the production save. The friction witness's
//! local-private section set (8.5 MB) is not committed, so its local-private
//! request is checked by its digest in the web view and saved natively only
//! for the gap witness.
//!
//! Every id is invented; unit transport replay, not a native UI qualification
//! witness. "Admitted" means only that the checks hold. Standard claim fence
//! applies (claims taxonomy per DEC-081).

use crate::command_ingress::test_support::{admit_json, headers};
use crate::command_ingress::{ArgsDecoding, INGRESS_ARGS_JSON_V1};
use crate::exact_source_binding::SECTIONS_MISMATCH;
use crate::memory_pool::test_host::ScriptedHost;
use crate::report_package_save::test_support::{no_store, sandbox, save_request_with, software};
use crate::report_package_save::{
    admit_report_package_in, admit_report_package_session, ingress_lease_bytes, save_admitted_report_package_in, ReportPackageSaveRequest,
    SaveRefusal, UserReleaseSession,
};
use crate::solve_attestation::SolveAttestation;
use crate::solve_jobs::test_support::{delivered_in_custody, held_natively};
use crate::solve_jobs::SolveJobRegistry;
use open_pipe_stress_operation_applier::{canonical_json, sha256_hex};
use open_pipe_stress_product_physics::PreviewSolverMode;
use open_pipe_stress_report_package::{read_report_package_container, ReadReportPackage};
use open_pipe_stress_report_package::user_release::{derive_exact, SoftwareIdentity, UserReleaseBasis};
use open_pipe_stress_report_renderer::standing::{
    frame_statements, kept_for_inspection_frame_statement, standing_line, support_states_frame_statement, NumericalStanding, KEPT_FOR_INSPECTION,
    SUPPORT_STATES_NOT_CONFIRMED,
};
use open_pipe_stress_report_renderer::{escape_html, UserReleaseRecord, USER_RELEASE_EXACT_RECORD_VERSION, USER_RELEASE_NOT_CARRIED};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tauri::ipc::InvokeBody;

const DIR: &str = "fixtures/reports/invented/report_standing_witnesses";
const NAMES: [&str; 2] = ["pressure_3_gap_unresolved", "pressure_3_coupled_friction_sensitive"];
const LOCAL_PRIVATE_PREFIX: &str = "desktop-report-";
const USER_RELEASED_PREFIX: &str = "desktop-user-released-report-";
const LOCAL_PRIVATE_PROFILE: &str = "desktop_local_private_report_package_1";

fn project(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(path)
}

fn read_json(path: &str) -> Value {
    serde_json::from_str(&std::fs::read_to_string(project(path)).unwrap_or_else(|e| panic!("{path}: {e}"))).unwrap()
}

fn digest_of(value: &Value) -> String {
    sha256_hex(&canonical_json(value))
}

/// One witness, delivered by a live test job.
struct Witness {
    name: &'static str,
    render: Value,
    package: Value,
    raw: String,
    source: Value,
    intake: String,
    manifest: Value,
    model: Value,
    registry: SolveJobRegistry,
    _gate: Arc<crate::solve_worker::test_support::Gate>,
    job: String,
}

fn witness(name: &'static str) -> Witness {
    let render = read_json(&format!("{DIR}/{name}.render.json"));
    let package = read_json(&format!("{DIR}/{name}.package.json"));
    assert_eq!(package["render_path"], format!("{DIR}/{name}.render.json"), "{name}");
    assert_eq!(package["raw_path"], render["raw_path"], "{name}");
    let raw = std::fs::read_to_string(project(render["raw_path"].as_str().unwrap())).unwrap();
    let source: Value = serde_json::from_str(&raw).unwrap();
    let intake = render["exact_intake_text"].as_str().unwrap().to_string();
    let manifest: Value = serde_json::from_str(render["input_manifest_text"].as_str().unwrap()).unwrap();
    assert_eq!(canonical_json(&manifest), canonical_json(&package["input_manifest"]["manifest"]), "{name}: one input manifest");
    let model = manifest["model_basis"]["model_payload"].clone();
    let (registry, gate, job) = delivered_in_custody(ScriptedHost::normal(), intake.as_bytes(), Some("sparse_interactive"), &raw);
    Witness { name, render, package, raw, source, intake, manifest, model, registry, _gate: gate, job }
}

impl Witness {
    /// The run record the native builder builds from the delivered job, required
    /// equal to the one the web view built.
    fn run_record(&self) -> Value {
        // The builder reads a result held in native custody: a second job of the same
        // intake text, mode and raw bytes, held natively.
        let (registry, _gate, job) = held_natively(ScriptedHost::normal(), self.intake.as_bytes(), Some("sparse_interactive"), &self.raw);
        let args = json!({"jobId": job, "inputManifest": self.package["input_manifest"], "ruleCheckAggregate": null, "rulePackExclusions": []});
        let body = InvokeBody::Raw(serde_json::to_vec(&args).unwrap());
        let (raw, lease) = crate::analysis_run_v03_build::reserve(&registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1))).unwrap();
        let (bytes, _lease) = crate::analysis_run_v03_build::run_record_bytes_within(registry.clone(), raw, lease, u64::MAX)
            .unwrap_or_else(|error| panic!("{}: {error:?}", self.name));
        let record: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(digest_of(&record), self.package["run_record_sha256"].as_str().unwrap(), "{}: the run record is the web view's", self.name);
        record
    }

    /// The web view's invented job and received-bytes digest, as its requests carry them.
    fn admission(&self) -> (&str, &str) {
        let admission = &self.render["input"]["exact_route"]["admission"];
        (admission["native_job_id"].as_str().unwrap(), admission["received_bytes_sha256"].as_str().unwrap())
    }

    /// The user-released request: the native derivation, required equal to the web view's.
    fn user_released_request(&self, run_record: &Value) -> Value {
        let record: UserReleaseRecord = serde_json::from_value(self.package["user_release"].clone()).unwrap();
        let software = SoftwareIdentity { name: software().name, version: software().version };
        let basis = UserReleaseBasis {
            model: &self.model,
            source: &self.source,
            solver_mode: "sparse_interactive",
            run_record,
            software: &software,
            record: &record,
            composition: None,
        };
        let (job, bytes) = self.admission();
        let request = derive_exact(&basis, job, bytes).unwrap_or_else(|error| panic!("{}: {error}", self.name)).request;
        assert_eq!(digest_of(&request), self.package["user_released_request_sha256"].as_str().unwrap(), "{}: the user-released request is the web view's", self.name);
        request
    }

    /// The local-private request, composed from the derived user-released request (whose
    /// result document, audit manifest and source are the local-private request's, as the
    /// web view's test asserts) with the render input as its report and the committed
    /// section set, required equal to the web view's. `None` where no section set is committed.
    fn local_private_request(&self, user_released: &Value) -> Option<Value> {
        let sections = self.package["local_private_sections"].clone();
        if sections.is_null() {
            return None;
        }
        let mut request = user_released.clone();
        let id = request["package_id"].as_str().unwrap().strip_prefix(USER_RELEASED_PREFIX).unwrap().to_string();
        request["package_id"] = json!(format!("{LOCAL_PRIVATE_PREFIX}{id}"));
        request["export_profile_id"] = json!(LOCAL_PRIVATE_PROFILE);
        request["report"] = self.render["input"].clone();
        request["state_comparison_handoff_records"] = sections;
        assert_eq!(digest_of(&request), self.package["local_private_request_sha256"].as_str().unwrap(), "{}: the local-private request is the web view's", self.name);
        Some(request)
    }

    /// `request` bound to the live job.
    fn bound(&self, request: &Value) -> Value {
        let mut request = request.clone();
        request["report"]["exact_route"]["admission"]["native_job_id"] = json!(self.job);
        request
    }

    fn args(&self, request: &Value) -> ReportPackageSaveRequest {
        let mut args = save_request_with(request, &self.model, &self.manifest);
        args.exact_result_text = Some(self.raw.clone());
        args.exact_intake_text = Some(self.intake.clone());
        args
    }

    fn user_released_args(&self, request: &Value, run_record: &Value) -> ReportPackageSaveRequest {
        let mut args = self.args(request);
        crate::user_release_save_tests::user_release_evidence(&mut args);
        args.analysis_run_text = Some(canonical_json(run_record));
        args
    }

    /// The rendered report of the same result: the production render of the committed
    /// render input with the live job.
    fn rendered_html(&self) -> String {
        let mut input = self.render["input"].clone();
        input["exact_route"]["admission"]["native_job_id"] = json!(self.job);
        let args = json!({
            "input": input,
            "exactResultText": self.raw,
            "exactIntakeText": self.intake,
            "inputManifestText": self.render["input_manifest_text"],
        });
        let (mut decoded, mut lease) = admit_json(&self.registry, "render_calculation_report", ArgsDecoding::Moved, &args).unwrap();
        let rendered = crate::render_calculation_report_args(&mut decoded, &self.registry, &mut lease).unwrap_or_else(|error| panic!("{}: {error:?}", self.name));
        assert_eq!(rendered["export_blocked"], json!(false), "{}", self.name);
        rendered["html"].as_str().unwrap().to_string()
    }

    /// Saves `admitted` to a sandbox and reads it back with the production reader.
    fn saved(&self, admitted: crate::report_package_save::AdmittedReportPackage, label: &str) -> ReadReportPackage {
        assert!(!admitted.package.export_blocked, "{}: {:?}", self.name, admitted.package.blocking_reasons);
        let custody = |job: &str| self.registry.custody_binding(job);
        let dir = sandbox(label);
        let mut chosen = None;
        let receipt = save_admitted_report_package_in(admitted, &custody, |file| {
            let path = dir.join(file);
            chosen = Some(path.clone());
            Some(Ok(path))
        })
        .unwrap_or_else(|error| panic!("{}: {error:?}", self.name));
        let read = read_report_package_container(&std::fs::read(chosen.unwrap()).unwrap()).unwrap_or_else(|refusal| panic!("{}: {refusal:?}", self.name));
        assert_eq!(read.container_sha256_hex, receipt.container_sha256_hex, "{}", self.name);
        assert!(!read.export_blocked, "{}", self.name);
        read
    }

    /// What the saved package's report states of the standing: the standing line the
    /// request's standing record gives, in the frame, and the witness's own case and
    /// support statements, each as the rendered report of the same result states it.
    fn assert_states_the_standing(&self, read: &ReadReportPackage, request: &Value, rendered: &str) {
        let html = read.members.iter().find(|member| member.file_name == "calculation_report.html").expect("the HTML member");
        let html = std::str::from_utf8(&html.bytes).unwrap();
        let record: NumericalStanding = serde_json::from_value(request["report"]["numerical_standing"].clone()).unwrap();
        let line = format!("<div class=\"standing-line\">{}</div>", escape_html(&standing_line(&record)));
        assert!(html.contains(&line), "{}: the package's frame states the standing line", self.name);
        assert!(rendered.contains(&line), "{}: the rendered report states the same standing line", self.name);
        assert!(!html.contains("integrity checked"), "{}", self.name);
        // The frame's statements, each its own line, from the same record: the support states
        // for both witnesses (each has nonlinear supports); "Kept for inspection." only for the
        // gap witness, whose load cases carry the solver's recovery-basis mark.
        let gap = self.name == "pressure_3_gap_unresolved";
        let statement = support_states_frame_statement(&record).expect("each witness has a nonlinear support");
        let frame = format!("<div class=\"standing-support-states\">{}</div>", escape_html(&statement));
        assert!(html.contains(&frame), "{}: the package's frame states the support states", self.name);
        assert!(rendered.contains(&frame), "{}: the rendered report's frame states the same", self.name);
        let kept = kept_for_inspection_frame_statement(&record);
        assert_eq!(kept.is_some(), gap, "{}: only the gap witness carries the recovery-basis mark", self.name);
        assert_eq!(frame_statements(&record).len(), if gap { 2 } else { 1 }, "{}", self.name);
        assert_eq!(html.contains("<div class=\"standing-kept-for-inspection\">"), gap, "{}", self.name);
        if let Some(kept) = &kept {
            let line = format!("<div class=\"standing-kept-for-inspection\">{}</div>", escape_html(kept));
            assert!(html.contains(&format!("{frame}\n{line}")), "{}: the kept-for-inspection line follows on a line of its own", self.name);
            assert!(rendered.contains(&line), "{}", self.name);
        }
        // The PDF member's frame carries the same banners: each statement opens a banner of
        // its own, so its first words are in the PDF's text.
        let pdf = read.members.iter().find(|member| member.file_name.ends_with(".pdf")).expect("the PDF member");
        let pdf_contains = |needle: &str| pdf.bytes.windows(needle.len()).any(|window| window == needle.as_bytes());
        assert!(pdf_contains(SUPPORT_STATES_NOT_CONFIRMED), "{}: the PDF states the support states", self.name);
        assert_eq!(pdf_contains(KEPT_FOR_INSPECTION), gap, "{}: the PDF states kept for inspection only for the gap witness", self.name);
        let phrases: &[&str] = if self.name == "pressure_3_gap_unresolved" {
            &[
                "Numerical health, as the solver reports it: 0 checks passed, 0 sensitive, 2 quality unresolved, 0 not assessed, of 2 load cases.",
                "Quality unresolved: this load case has a gap support. No confirmation covers gap supports, and its member values have no error figure.",
                "Computed state: active. Not confirmed: no confirmation covers gap supports.",
            ]
        } else {
            &[
                "Numerical health, as the solver reports it: 0 checks passed, 1 sensitive, 0 quality unresolved, 0 not assessed, of 1 load case.",
                "Computed state: sliding. Not confirmed: the solver&#39;s confirmation reports this state established, conditional on the solver&#39;s inverse-norm step.",
            ]
        };
        for phrase in phrases {
            assert!(html.contains(phrase), "{}: the package states {phrase}", self.name);
            assert!(rendered.contains(phrase), "{}: the rendered report states {phrase}", self.name);
        }
    }
}

/// The local-private package of a witness that is not numerically eligible is saved,
/// and its report states the standing the rendered report states; a request whose
/// standing record differs from the native derivation is refused by name.
#[test]
fn the_gap_witness_saves_a_local_private_package_stating_its_standing() {
    let w = witness(NAMES[0]);
    let run_record = w.run_record();
    let user_released = w.user_released_request(&run_record);
    let request = w.bound(&w.local_private_request(&user_released).expect("the gap witness carries its section set"));
    let custody = |job: &str| w.registry.custody_binding(job);
    let admitted = admit_report_package_in(w.args(&request), &software(), &mut no_store, &custody)
        .unwrap_or_else(|error| panic!("{}: {} {}", w.name, error.code, error.message));
    let read = w.saved(admitted, "standing-witness-local-private");
    assert!(read.user_release.is_none());
    let rendered = w.rendered_html();
    w.assert_states_the_standing(&read, &request, &rendered);
    // The rendered report is the package's report, byte for byte (DEL-08-01 REQ-013).
    let html = read.members.iter().find(|member| member.file_name == "calculation_report.html").unwrap();
    assert_eq!(std::str::from_utf8(&html.bytes).unwrap(), rendered, "{}: the package's HTML is the rendered report", w.name);
    // A standing record that differs from the native derivation refuses by name.
    let mut altered = request.clone();
    altered["report"]["numerical_standing"]["cases"][0]["health"] = json!("checks_passed");
    let error = admit_report_package_in(w.args(&altered), &software(), &mut no_store, &custody).err().expect("refused");
    assert_eq!(error.code, SECTIONS_MISMATCH, "{}: {}", w.name, error.message);
    assert!(error.message.ends_with(": numerical_standing"), "{}: {}", w.name, error.message);
}

/// Route 010 for each witness: the user-released package is admitted, saved and read
/// back; its record is the exact-route record, which names no result as not carried on
/// the ground of its standing, and its report states the standing the rendered report
/// states. A user-released request whose standing record differs is refused.
#[test]
fn route_010_saves_each_standing_witness_stating_its_standing() {
    for name in NAMES {
        let w = witness(name);
        let run_record = w.run_record();
        let request = w.bound(&w.user_released_request(&run_record));
        w.registry.record_attested_solve_for_tests(1, PreviewSolverMode::SparseInteractive, SolveAttestation::of(&w.model, &w.source).unwrap());
        let custody = |job: &str| w.registry.custody_binding(job);
        let admit = |request: &Value| {
            let args = w.user_released_args(request, &run_record);
            let mut lease = w.registry.acquire_interactive_ingress(1024).unwrap();
            lease.grow_to(1024 + ingress_lease_bytes(&args).unwrap()).unwrap();
            let mut session = UserReleaseSession { registry: &w.registry, lease: &mut lease };
            admit_report_package_session(args, &software(), &mut no_store, &custody, Some(&mut session))
        };
        let admitted = admit(&request).unwrap_or_else(|refusal| panic!("{name}: {refusal:?}"));
        let read = w.saved(admitted, "standing-witness-user-released");
        let record = read.user_release.as_ref().expect("a user-release record");
        assert_eq!(record["record_version"], USER_RELEASE_EXACT_RECORD_VERSION, "{name}");
        assert_eq!(record["scope"]["not_carried"], json!(USER_RELEASE_NOT_CARRIED), "{name}");
        let rendered = w.rendered_html();
        w.assert_states_the_standing(&read, &request, &rendered);
        let html = read.members.iter().find(|member| member.file_name == "calculation_report.html").unwrap();
        assert!(std::str::from_utf8(&html.bytes).unwrap().contains("USER-RELEASED REPORT"), "{name}");
        // A standing record that differs from the native derivation refuses.
        let mut altered = request.clone();
        altered["report"]["numerical_standing"]["cases"][0]["health"] = json!("checks_passed");
        match admit(&altered) {
            Err(SaveRefusal::Command(error)) => assert!(error.message.contains("numerical_standing"), "{name}: {} {}", error.code, error.message),
            other => panic!("{name}: an altered standing record must be refused, got {:?}", other.map(|_| "admitted")),
        }
    }
}

/// The frame statements file of a witness: what the native host gives the user-release
/// dialog for it, committed so that the web view's tests show the renderer's words
/// (`UserReleaseDialog.test.tsx`, `renderedReportGate.app.test.tsx`).
/// `OPENPIPESTRESS_WRITE_FRAME_STATEMENTS=1` rewrites it.
const WRITE_FRAME_STATEMENTS: &str = "OPENPIPESTRESS_WRITE_FRAME_STATEMENTS";

fn frame_statements_reply_for(registry: &SolveJobRegistry, job: &str, model_text: &str) -> Result<Value, String> {
    let args = json!({"jobId": job, "operation": crate::native_verdict::REPORT_FRAME_STATEMENTS, "modelJson": model_text});
    let body = InvokeBody::Raw(serde_json::to_vec(&args).unwrap());
    let (raw, lease) = crate::native_verdict::reserve(registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1))).map_err(|error| format!("{error:?}"))?;
    let text = crate::native_verdict::run(registry.clone(), raw, lease).map_err(|error| match error {
        crate::command_ingress::CommandError::Command(message) => message,
        other => format!("{other:?}"),
    })?;
    Ok(serde_json::from_str(&text).unwrap())
}

/// DEL-08-01 REQ-024, "Package and user release": for a result held in native custody, the
/// native host gives the user-release dialog the frame statements the renderer composes from
/// the result's standing record, the statements the saved package's report states. The gap
/// witness, whose load cases carry the solver's recovery-basis mark, has two lines (support
/// states not confirmed; kept for inspection); the friction witness, with nonlinear supports
/// and no mark, has the first only. Where no standing record can be read the host refuses by
/// name, and so it does for a result it does not hold.
#[test]
fn the_native_host_gives_the_dialog_the_frame_statements_the_package_states() {
    for name in NAMES {
        let w = witness(name);
        let (registry, _gate, job) = held_natively(ScriptedHost::normal(), w.intake.as_bytes(), Some("sparse_interactive"), &w.raw);
        let model_text = serde_json::to_string(&w.model).unwrap();
        let reply = frame_statements_reply_for(&registry, &job, &model_text).unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(reply["document_kind"], crate::native_verdict::FRAME_STATEMENTS_DOCUMENT_KIND, "{name}");
        assert_eq!(reply["version"], 1, "{name}");
        assert_eq!(reply["job_id"], job.as_str(), "{name}");
        let record: NumericalStanding = serde_json::from_value(w.render["input"]["numerical_standing"].clone()).unwrap();
        let statements = frame_statements(&record);
        assert_eq!(reply["frame_statements"], json!(statements), "{name}: the renderer's statements of the report's own record");
        if name == "pressure_3_gap_unresolved" {
            assert_eq!(statements.len(), 2, "{name}");
            assert!(statements[0].starts_with(SUPPORT_STATES_NOT_CONFIRMED) && statements[1].starts_with(KEPT_FOR_INSPECTION), "{name}: {statements:?}");
        } else {
            assert_eq!(statements.len(), 1, "{name}");
            assert!(statements[0].starts_with(SUPPORT_STATES_NOT_CONFIRMED), "{name}: {statements:?}");
        }
        let path = format!("{DIR}/{name}.frame_statements.json");
        let file = format!(
            "{}\n",
            serde_json::to_string_pretty(&json!({
                "description": "DEL-08-01 REQ-024: the frame statements the native host gives the user-release dialog for this witness held in native custody (the verdict command's report_frame_statements operation), the renderer's own lines for its standing record. Written by report_standing_package_tests.rs; unit transport replay, not a native UI qualification witness; every id is invented.",
                "frame_statements": statements,
            }))
            .unwrap()
        );
        if std::env::var_os(WRITE_FRAME_STATEMENTS).is_some() {
            std::fs::write(project(&path), &file).unwrap();
        } else {
            let committed = std::fs::read_to_string(project(&path)).unwrap_or_else(|e| panic!("{path}: {e}; {WRITE_FRAME_STATEMENTS}=1 writes it"));
            assert!(committed == file, "{path} is stale; {WRITE_FRAME_STATEMENTS}=1 rewrites it");
        }
    }
    // No statements without a standing record, and none for a result the host does not hold.
    let unavailable = crate::native_verdict::frame_statements_reply("invented-job", r#"{"numerical_standing":null,"standing_refusal":null}"#).unwrap_err();
    assert!(unavailable.starts_with(&format!("{}: ", crate::native_verdict::FRAME_STATEMENTS_UNAVAILABLE)), "{unavailable}");
    let refused = crate::native_verdict::frame_statements_reply("invented-job", r#"{"numerical_standing":null,"standing_refusal":"NUMERICAL_TAG_UNSUPPORTED: invented"}"#).unwrap_err();
    assert_eq!(refused, format!("{}: NUMERICAL_TAG_UNSUPPORTED: invented", crate::native_verdict::FRAME_STATEMENTS_UNAVAILABLE));
    let w = witness(NAMES[0]);
    let model_text = serde_json::to_string(&w.model).unwrap();
    assert!(frame_statements_reply_for(&w.registry, "invented-unknown-job", &model_text).is_err());
}
