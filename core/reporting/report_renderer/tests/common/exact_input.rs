//! An invented exact-route renderer input (W-11, DEL-08-01 Design
//! `exact-route-report-sections.md` §3.2-§3.5), built over a test's own
//! pressure-free fixture input. Shared by path with the PDF emitter's tests.
//! Every digest and id is invented; the statements are the embedded table's.

#![allow(dead_code)]

use open_pipe_stress_report_generator as report;
use open_pipe_stress_report_renderer::exact_statements::exact_statement_table;
use open_pipe_stress_report_renderer::{
    ExactAdmissionBinding, ExactRouteSection, LibraryReference, RenderableReportInput, ReportHeadline,
};
use open_pipe_stress_report_sections as sections;

pub const PRESSURE_2: &str = "openpipestress.result_semantics/0.3.0/pressure-2";
pub const RECEIVED: &str = "1111111111111111111111111111111111111111111111111111111111111111";
pub const MODEL: &str = "2222222222222222222222222222222222222222222222222222222222222222";
pub const MANIFEST: &str = "3333333333333333333333333333333333333333333333333333333333333333";
pub const DERIVATIVE: &str = "4444444444444444444444444444444444444444444444444444444444444444";
pub const BYTES: &str = "5555555555555555555555555555555555555555555555555555555555555555";
pub const LOAD_REFERENCE_LINE: &str = "This result includes resolved load/reference states (reference configuration invented-configuration; load cases case:invented-a, case:invented-b; history independent_equilibrium). Their published limitations are listed under Limitations.";
pub const WITHHELD: &str =
    "withheld: a stress maximum on this basis is not available for every member in every load case";

/// `base` with the exact-route block, its admission binding bound to the
/// report's own records, one exact limitation entry per table statement, the
/// profile reference and one library record.
pub fn with_exact_route(mut input: RenderableReportInput) -> RenderableReportInput {
    let run = input.report_sections.run_ref.ref_id.clone();
    let envelope_id = format!("result-envelope:{run}");
    let calc = &mut input.calculation_report;
    let mut received = calc.audit_manifest_refs[0].clone();
    received.reference = report::Reference::new("result_envelope", envelope_id.clone());
    received.checksum.payload_ref = received.reference.clone();
    received.checksum.value = RECEIVED.to_string();
    calc.audit_manifest_refs.push(received);
    calc.model_input_summary.model_hash.value = MODEL.to_string();
    let manifest_ref = calc.model_input_summary.input_manifest_ref.ref_id.clone();
    input.exact_route = Some(ExactRouteSection {
        semantic_contract_id: PRESSURE_2.to_string(),
        admission: ExactAdmissionBinding {
            semantic_contract_id: PRESSURE_2.to_string(),
            run_ref: run.clone(),
            result_envelope_id: envelope_id,
            received_result_sha256: RECEIVED.to_string(),
            model_sha256: MODEL.to_string(),
            input_manifest_ref: manifest_ref,
            input_manifest_sha256: MANIFEST.to_string(),
            derivative_sha256: DERIVATIVE.to_string(),
            native_job_id: "invented-job-1".to_string(),
            received_bytes_sha256: BYTES.to_string(),
        },
        load_reference_states: Some(LOAD_REFERENCE_LINE.to_string()),
        headlines: vec![
            ReportHeadline {
                basis: "nominal_section".to_string(),
                label: "Nominal section (nominal wall)".to_string(),
                value_display: "41.3549 MPa".to_string(),
                location_ref: Some("pipe:invented-span".to_string()),
                source_ref: Some("result:invented-nominal-maximum".to_string()),
            },
            ReportHeadline {
                basis: "reduced_section".to_string(),
                label: "Reduced section (wall minus mill tolerance and corrosion allowance)".to_string(),
                value_display: WITHHELD.to_string(),
                location_ref: None,
                source_ref: None,
            },
        ],
    });
    let template = input.report_sections.limitations[0].clone();
    let entry = |id: String, statement: &str| sections::Limitation {
        limitation_id: id,
        source: sections::Reference::new("semantic_contract", PRESSURE_2),
        affected_scope: sections::Reference::new("analysis_run", run.clone()),
        statement: statement.to_string(),
        effect: template.effect.clone(),
        provenance: template.provenance.clone(),
    };
    let table = exact_statement_table().unwrap();
    for (index, (_, statement)) in table.entries().enumerate() {
        input.report_sections.limitations.push(entry(format!("limitation:{run}:exact:{}", index + 1), statement));
    }
    input.report_sections.limitations.push(entry(
        format!("limitation:{run}:exact:profile-reference"),
        "The profile limitations of the semantic table openpipestress.result_semantics/0.3.0/pressure-2 (sha256 6666666666666666666666666666666666666666666666666666666666666666) are bound by reference and are not restated in this report.",
    ));
    input.library_references = Some(vec![LibraryReference {
        reference_id: "library-ref:1".to_string(),
        library_kind: "material".to_string(),
        library_id: "invented-material-library".to_string(),
        library_version: "sha256:7777777777777777777777777777777777777777777777777777777777777777".to_string(),
        record_id: "not recorded".to_string(),
        entity_refs: vec!["material:invented-a".to_string(), "material:invented-b".to_string()],
        provenance: template.provenance.clone(),
    }]);
    input
}
