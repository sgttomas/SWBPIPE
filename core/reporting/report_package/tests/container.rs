//! Determinism, inventory, gate, and archive-form tests for the full
//! report-package container (DEC-028 / DEC-057 / DEC-061, row E3). All data
//! invented; no protected standards content.

use open_pipe_stress_audit_manifest as audit;
use open_pipe_stress_report_package::{
    assemble_report_package_container, PackageRef, ReportPackageContainerInput,
    CONTAINER_DOCUMENT_KIND, CONTAINER_EXTENSION, PACKAGE_MANIFEST_FILE_NAME, ROLE_AUDIT_MANIFEST,
    ROLE_PACKAGE_MANIFEST, ROLE_REPORT_HTML, ROLE_REPORT_PDF, ROLE_RESULT_EXPORT,
    ROLE_STATE_COMPARISON_HANDOFF,
};
use open_pipe_stress_report_renderer::{RenderableReportInput, ResultRow};
use open_pipe_stress_report_sections as sections;
use open_pipe_stress_result_export as export;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const FIXTURE: &str =
    include_str!("../../../../fixtures/reports/invented/calculation_report_fixture.json");

// ---------------------------------------------------------------------------
// Invented fixtures.
// ---------------------------------------------------------------------------

fn invented_section_provenance() -> sections::Provenance {
    sections::Provenance {
        source_name: "Invented OpenPipeStress report package test".to_string(),
        source_location: "core/reporting/report_package/tests/container.rs".to_string(),
        source_license: "project_fixture".to_string(),
        contributor: "OpenPipeStress".to_string(),
        contributor_certification: "Invented non-engineering data only.".to_string(),
        redistribution_status: sections::RedistributionStatus::InventedNonEngineeringExample,
        review_status: sections::ReviewStatus::Accepted,
        privacy_classification: sections::PrivacyClassification::InventedPublicExample,
    }
}

fn invented_report_sections() -> sections::ReportSections {
    sections::ReportSections {
        report_section_id: "invented-report-sections-001".to_string(),
        model_ref: sections::Reference::new("model", "invented-model"),
        run_ref: sections::Reference::new("analysis_run", "invented-run"),
        diagnostics: vec![],
        analysis_status_disclosures: vec![
            sections::AnalysisStatusDisclosure {
                status: sections::AnalysisStatus::MechanicsSolved,
                source: sections::Reference::new("solver", "invented-solver"),
                affected_object: sections::Reference::new("model", "invented-model"),
                explanation: "Invented linear-static preview mechanics solved.".to_string(),
                human_review_required: true,
                human_acceptance_ref: None,
            },
            sections::AnalysisStatusDisclosure {
                status: sections::AnalysisStatus::HumanReviewRequired,
                source: sections::Reference::new("report_package", "invented-package"),
                affected_object: sections::Reference::new("report", "invented-report-001"),
                explanation: "Human professional review is required before any reliance."
                    .to_string(),
                human_review_required: true,
                human_acceptance_ref: None,
            },
        ],
        provenance_notes: vec![invented_section_provenance()],
        user_supplied_values: vec![],
        assumptions: vec![],
        limitations: vec![sections::Limitation {
            limitation_id: "invented-limitation-001".to_string(),
            source: sections::Reference::new("report_package", "invented-package"),
            affected_scope: sections::Reference::new("model", "invented-model"),
            statement: "Invented preview data only; not engineering output.".to_string(),
            effect: sections::ReportEffect {
                mechanics_solve_qualified: true,
                user_rule_check_qualified: false,
                report_completeness: sections::ReportCompleteness::Qualified,
                human_review_required: true,
            },
            provenance: invented_section_provenance(),
        }],
        unresolved_tbds: vec![],
        professional_boundary: sections::ProfessionalBoundary::project_default(),
    }
}

fn fixture_report_input() -> RenderableReportInput {
    let fixture: Value = serde_json::from_str(FIXTURE).expect("fixture parses");
    let calculation_report = serde_json::from_value(fixture["calculation_report"].clone())
        .expect("calculation_report deserializes via the serde feature");
    RenderableReportInput {
        report_title: "Invented Calculation Report (Technical Preview)".to_string(),
        calculation_report,
        report_sections: invented_report_sections(),
        result_rows: vec![ResultRow {
            row_id: "invented-row-001".to_string(),
            label: "Max bending stress (invented)".to_string(),
            case_ref: "load_case:invented-weight".to_string(),
            quantity_display: "12.5 MPa".to_string(),
            source_ref: "result_envelope:invented-result".to_string(),
        }],
    }
}

fn invented_audit_manifest() -> audit::AuditManifest {
    audit::AuditManifest {
        manifest_id: "invented-manifest-001".to_string(),
        model_hash: Some(audit::hash_canonical_json(
            audit::PayloadKind::ModelJson,
            "invented-model",
            &audit::CanonicalJson::object([(
                "model_id",
                audit::CanonicalJson::String("invented-model".to_string()),
            )]),
        )),
        input_manifest_hash: Some(audit::hash_canonical_json(
            audit::PayloadKind::InputManifestJson,
            "invented-input-manifest",
            &audit::CanonicalJson::object([(
                "input_manifest_id",
                audit::CanonicalJson::String("invented-input-manifest".to_string()),
            )]),
        )),
        solver_version: audit::SolverVersionStamp {
            solver_name: "open_pipe_stress_core".to_string(),
            solver_version: "0.1.0".to_string(),
            solver_build_ref: "invented-build".to_string(),
        },
        unit_system_ref: "invented-si".to_string(),
        rule_pack_refs: vec![],
        assets: vec![],
        professional_boundary: audit::ProfessionalBoundary::project_default(),
    }
}

fn export_checksum(id: &str) -> export::ChecksumRef {
    export::ChecksumRef {
        algorithm: "sha256".to_string(),
        canonicalization: "JCS".to_string(),
        payload_ref: export::Reference::new("payload", id),
        value: format!("{id}-invented-hash"),
    }
}

fn export_provenance() -> export::Provenance {
    export::Provenance {
        source_name: "invented report package fixture".to_string(),
        source_location: "core/reporting/report_package/tests/container.rs".to_string(),
        source_license: "project invented".to_string(),
        contributor: "OpenPipeStress".to_string(),
        contributor_certification: "invented non-engineering example".to_string(),
        redistribution_status: export::RedistributionStatus::InventedNonEngineeringExample,
        review_status: "accepted".to_string(),
    }
}

fn invented_result_envelope(magnitude: f64) -> export::ResultEnvelope {
    export::ResultEnvelope {
        envelope_id: "invented-result-envelope-001".to_string(),
        schema_version: "0.1.0".to_string(),
        model_ref: export::Reference::new("model", "invented-model"),
        run_ref: export::Reference::new("analysis_run", "invented-run"),
        solver_name: "open_pipe_stress_core".to_string(),
        solver_version: "0.1.0".to_string(),
        solver_build_ref: "invented-build".to_string(),
        unit_system_ref: export::Reference::new("unit_system", "invented-si"),
        load_basis_refs: vec![export::Reference::new("load_case", "LC1")],
        result_sets: vec![export::ResultSet {
            set_id: "invented-set-1".to_string(),
            set_type: export::ResultSetType::Mechanics.as_str().to_string(),
            basis_ref: export::Reference::new("load_case", "LC1"),
            values: vec![export::QuantityResult {
                result_id: "invented-stress-1".to_string(),
                family: export::ResultFamily::Stress,
                object_ref: export::Reference::new("node", "N1"),
                basis_ref: export::Reference::new("load_case", "LC1"),
                station_ref: None,
                magnitude,
                unit: "Pa".to_string(),
                dimension: export::DimensionId::Stress,
                metadata: None,
                diagnostics: Vec::new(),
                trace_chain: Vec::new(),
                provenance: export_provenance(),
            }],
        }],
        diagnostics: Vec::new(),
        provenance: export_provenance(),
        reproducibility: export::ReproducibilityRefs {
            model_hash: export_checksum("model"),
            run_hashes: vec![export_checksum("run")],
            audit_manifest_ref: export::Reference::new("audit_manifest", "invented-manifest-001"),
            deterministic_ordering: true,
        },
        analysis_status: vec![
            export::AnalysisStatus::MechanicsSolved,
            export::AnalysisStatus::HumanReviewRequired,
        ],
        rule_pack_refs: vec![],
        professional_boundary: export::ProfessionalBoundary::project_default(),
    }
}

fn invented_sections_record() -> Value {
    json!({
        "schema_version": "0.1.0",
        "deliverable_id": "DEL-08-06",
        "package_id": "PKG-08",
        "scope_item": "SOW-024",
        "section_set_id": "invented-section-set-001",
        "section_contract_status": "backend_report_section_records_only",
        "sections": {
            "state_run_sections": [],
            "comparison_sections": [],
            "handoff_sections": []
        },
        "diagnostics": [],
        "professional_boundary": {
            "human_review_required": true,
            "software_makes_compliance_claim": false,
            "software_makes_certification_claim": false,
            "software_makes_sealing_claim": false,
            "software_makes_approval_claim": false,
            "software_makes_authentication_claim": false,
            "software_creates_professional_reliance_record": false,
            "software_creates_external_validation_record": false
        }
    })
}

struct Fixture {
    report: RenderableReportInput,
    audit_manifest: audit::AuditManifest,
    result_envelopes: Vec<export::ResultEnvelope>,
    sections_records: Vec<Value>,
}

impl Fixture {
    fn new() -> Self {
        Self {
            report: fixture_report_input(),
            audit_manifest: invented_audit_manifest(),
            result_envelopes: vec![invented_result_envelope(12.5)],
            sections_records: vec![invented_sections_record()],
        }
    }

    fn input(&self) -> ReportPackageContainerInput<'_> {
        ReportPackageContainerInput {
            package_id: "invented-package-001".to_string(),
            export_profile_id: "invented-profile-native-full-report".to_string(),
            source_model_ref: PackageRef::new("model", "invented-model"),
            source_basis_refs: vec![PackageRef::new("analysis_run", "invented-run")],
            report: &self.report,
            audit_manifest: &self.audit_manifest,
            result_envelopes: &self.result_envelopes,
            state_comparison_handoff_records: &self.sections_records,
        }
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

// ---------------------------------------------------------------------------
// Minimal independent ZIP reader (test-side witness of the archive form).
// ---------------------------------------------------------------------------

struct ZipEntry {
    name: String,
    crc32: u32,
    data: Vec<u8>,
    dos_time: u16,
    dos_date: u16,
}

fn read_u16(bytes: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([bytes[offset], bytes[offset + 1]])
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}

/// Parse a stored-only zip: EOCD (no comment) -> central directory -> local
/// entries, in central-directory order.
fn parse_zip(bytes: &[u8]) -> Vec<ZipEntry> {
    let eocd = bytes.len() - 22;
    assert_eq!(read_u32(bytes, eocd), 0x0605_4b50, "EOCD signature");
    let entry_count = read_u16(bytes, eocd + 10) as usize;
    let mut central = read_u32(bytes, eocd + 16) as usize;

    let mut entries = Vec::new();
    for _ in 0..entry_count {
        assert_eq!(
            read_u32(bytes, central),
            0x0201_4b50,
            "central directory signature"
        );
        let name_len = read_u16(bytes, central + 28) as usize;
        let extra_len = read_u16(bytes, central + 30) as usize;
        let comment_len = read_u16(bytes, central + 32) as usize;
        let local_offset = read_u32(bytes, central + 42) as usize;
        let name = String::from_utf8(bytes[central + 46..central + 46 + name_len].to_vec())
            .expect("member name is ASCII");

        assert_eq!(
            read_u32(bytes, local_offset),
            0x0403_4b50,
            "local signature"
        );
        let method = read_u16(bytes, local_offset + 8);
        assert_eq!(method, 0, "members must be stored (uncompressed)");
        let dos_time = read_u16(bytes, local_offset + 10);
        let dos_date = read_u16(bytes, local_offset + 12);
        let crc = read_u32(bytes, local_offset + 14);
        let size = read_u32(bytes, local_offset + 18) as usize;
        let local_name_len = read_u16(bytes, local_offset + 26) as usize;
        let local_extra_len = read_u16(bytes, local_offset + 28) as usize;
        let data_start = local_offset + 30 + local_name_len + local_extra_len;
        let data = bytes[data_start..data_start + size].to_vec();

        entries.push(ZipEntry {
            name,
            crc32: crc,
            data,
            dos_time,
            dos_date,
        });
        central += 46 + name_len + extra_len + comment_len;
    }
    entries
}

/// Independent table-driven CRC-32 (IEEE), cross-checking the emitter's
/// bitwise implementation.
fn reference_crc32(bytes: &[u8]) -> u32 {
    let mut table = [0u32; 256];
    for (index, slot) in table.iter_mut().enumerate() {
        let mut value = index as u32;
        for _ in 0..8 {
            value = if value & 1 != 0 {
                0xEDB8_8320 ^ (value >> 1)
            } else {
                value >> 1
            };
        }
        *slot = value;
    }
    let mut crc: u32 = 0xFFFF_FFFF;
    for byte in bytes {
        crc = table[((crc ^ u32::from(*byte)) & 0xFF) as usize] ^ (crc >> 8);
    }
    !crc
}

// ---------------------------------------------------------------------------
// Tests.
// ---------------------------------------------------------------------------

#[test]
fn fixture_package_assembles_deterministic_unblocked_container() {
    let fixture = Fixture::new();
    let first = assemble_report_package_container(&fixture.input());
    let second = assemble_report_package_container(&Fixture::new().input());

    assert!(
        !first.export_blocked,
        "fixture package must pass all gates; blocking reasons: {:?}",
        first.blocking_reasons
    );
    assert_eq!(
        first.container_bytes, second.container_bytes,
        "container assembly must be byte-deterministic across independent builds"
    );
    assert_eq!(first.container_sha256_hex, second.container_sha256_hex);
    assert_eq!(
        first.package_identity_sha256_hex,
        second.package_identity_sha256_hex
    );
    assert_eq!(first.members, second.members);

    assert_eq!(
        first.container_file_name,
        format!("invented-package-001.{CONTAINER_EXTENSION}")
    );
    assert_eq!(first.document_kind, CONTAINER_DOCUMENT_KIND);
    assert_eq!(first.container_sha256_hex.len(), 64);
    assert!(first
        .container_sha256_hex
        .chars()
        .all(|ch| ch.is_ascii_hexdigit() && !ch.is_ascii_uppercase()));

    // No timestamp or varying identifier anywhere: identity equals content.
    assert_eq!(
        first.package_identity_sha256_hex,
        sha256_hex(&first.members[0].bytes)
    );
}

#[test]
fn member_inventory_carries_all_e3_named_members_in_fixed_order() {
    let fixture = Fixture::new();
    let outcome = assemble_report_package_container(&fixture.input());

    let roles: Vec<&str> = outcome
        .members
        .iter()
        .map(|member| member.role.as_str())
        .collect();
    assert_eq!(
        roles,
        vec![
            ROLE_PACKAGE_MANIFEST,
            ROLE_REPORT_HTML,
            ROLE_REPORT_PDF,
            ROLE_AUDIT_MANIFEST,
            ROLE_RESULT_EXPORT,
            ROLE_STATE_COMPARISON_HANDOFF,
        ]
    );
    let names: Vec<&str> = outcome
        .members
        .iter()
        .map(|member| member.file_name.as_str())
        .collect();
    assert_eq!(
        names,
        vec![
            PACKAGE_MANIFEST_FILE_NAME,
            "calculation_report.html",
            "calculation_report.pdf",
            "audit_manifest.json",
            "result_export_invented-result-envelope-001.json",
            "state_comparison_handoff_sections_invented-section-set-001.json",
        ]
    );
}

#[test]
fn manifest_lists_every_member_with_matching_jcs_hashes() {
    let fixture = Fixture::new();
    let outcome = assemble_report_package_container(&fixture.input());

    let manifest_member = &outcome.members[0];
    let manifest: Value =
        serde_json::from_slice(&manifest_member.bytes).expect("manifest member parses as JSON");

    assert_eq!(
        manifest["artifact"],
        "openpipestress.full_report_package_manifest"
    );
    assert_eq!(manifest["document_kind"], CONTAINER_DOCUMENT_KIND);
    assert_eq!(manifest["container_extension"], CONTAINER_EXTENSION);
    assert_eq!(manifest["package_id"], "invented-package-001");
    assert_eq!(
        manifest["export_profile_id"],
        "invented-profile-native-full-report"
    );
    assert_eq!(manifest["source_model_ref"]["ref_id"], "invented-model");
    assert_eq!(manifest["source_basis_refs"][0]["ref_id"], "invented-run");
    assert_eq!(
        manifest["manifest_self"]["file_name"],
        PACKAGE_MANIFEST_FILE_NAME
    );
    assert_eq!(manifest["diagnostics"]["export_blocked"], false);
    assert_eq!(
        manifest["professional_boundary"]["human_review_required"],
        true
    );
    assert!(manifest["sidecars"]["id_map_members"]
        .as_array()
        .expect("sidecar list present")
        .is_empty());

    // REQ-011: the manifest rows cover every non-manifest member; REQ-007:
    // each recorded hash matches the member's exact (JCS for JSON) bytes.
    let rows = manifest["members"].as_array().expect("member rows");
    assert_eq!(rows.len(), outcome.members.len() - 1);
    for (row, member) in rows.iter().zip(outcome.members.iter().skip(1)) {
        assert_eq!(row["role"], member.role.as_str());
        assert_eq!(row["file_name"], member.file_name.as_str());
        assert_eq!(row["media_type"], member.media_type.as_str());
        assert_eq!(row["sha256_hex"], member.sha256_hex.as_str());
        assert_eq!(row["byte_length"], member.byte_length);
        assert_eq!(row["sha256_hex"], sha256_hex(&member.bytes));
    }
}

#[test]
fn canonical_json_members_are_jcs_idempotent() {
    let fixture = Fixture::new();
    let outcome = assemble_report_package_container(&fixture.input());

    for member in &outcome.members {
        if member.media_type != "application/json" {
            continue;
        }
        let parsed: Value = serde_json::from_slice(&member.bytes).expect("JSON member parses");
        let recanonicalized = open_pipe_stress_canonical_json::canonical_json(&parsed);
        assert_eq!(
            recanonicalized.as_bytes(),
            member.bytes.as_slice(),
            "member {} bytes must already be the JCS rendering",
            member.file_name
        );
        assert_eq!(member.hash_basis, "jcs_canonical_json_bytes");
        assert!(member.carries_canonical_identity);
    }
}

#[test]
fn rendered_report_members_bind_to_the_dec_061_emission_hashes() {
    let fixture = Fixture::new();
    let outcome = assemble_report_package_container(&fixture.input());

    let html = outcome
        .members
        .iter()
        .find(|member| member.role == ROLE_REPORT_HTML)
        .expect("HTML member present");
    let pdf = outcome
        .members
        .iter()
        .find(|member| member.role == ROLE_REPORT_PDF)
        .expect("PDF member present");

    assert_eq!(html.sha256_hex, outcome.report.render.sha256_hex);
    assert_eq!(pdf.sha256_hex, outcome.report.pdf.sha256_hex);
    assert_eq!(html.hash_basis, "exact_bytes");
    assert_eq!(pdf.hash_basis, "exact_bytes");
    assert!(pdf.bytes.starts_with(b"%PDF-1.4\n"));
}

#[test]
fn zip_container_round_trips_members_byte_identically() {
    let fixture = Fixture::new();
    let outcome = assemble_report_package_container(&fixture.input());

    let entries = parse_zip(&outcome.container_bytes);
    assert_eq!(entries.len(), outcome.members.len());
    for (entry, member) in entries.iter().zip(outcome.members.iter()) {
        assert_eq!(entry.name, member.file_name);
        assert_eq!(
            entry.data, member.bytes,
            "member {} bytes",
            member.file_name
        );
        assert_eq!(
            entry.crc32,
            reference_crc32(&member.bytes),
            "member {} CRC-32",
            member.file_name
        );
        // Fixed ZIP-epoch timestamp: 1980-01-01 00:00:00, never a real time.
        assert_eq!(entry.dos_time, 0);
        assert_eq!(entry.dos_date, 0x0021);
    }
}

#[test]
fn input_changes_change_member_and_package_identity_hashes() {
    let fixture = Fixture::new();
    let baseline = assemble_report_package_container(&fixture.input());

    let mut changed_fixture = Fixture::new();
    changed_fixture.result_envelopes = vec![invented_result_envelope(99.75)];
    let changed = assemble_report_package_container(&changed_fixture.input());

    let member_hash = |outcome: &open_pipe_stress_report_package::ReportPackageContainerOutcome,
                       role: &str| {
        outcome
            .members
            .iter()
            .find(|member| member.role == role)
            .expect("member present")
            .sha256_hex
            .clone()
    };

    assert_ne!(
        member_hash(&baseline, ROLE_RESULT_EXPORT),
        member_hash(&changed, ROLE_RESULT_EXPORT)
    );
    assert_ne!(
        baseline.package_identity_sha256_hex,
        changed.package_identity_sha256_hex
    );
    assert_ne!(baseline.container_sha256_hex, changed.container_sha256_hex);
    // Unchanged members keep their hashes.
    assert_eq!(
        member_hash(&baseline, ROLE_REPORT_HTML),
        member_hash(&changed, ROLE_REPORT_HTML)
    );
}

#[test]
fn invalid_audit_manifest_blocks_the_package() {
    let mut fixture = Fixture::new();
    fixture.audit_manifest.model_hash = None;
    let outcome = assemble_report_package_container(&fixture.input());

    assert!(outcome.export_blocked);
    assert!(outcome
        .blocking_reasons
        .iter()
        .any(|reason| reason.contains("MissingModelHash")));

    // The blocked state is recorded inside the manifest member (REQ-041).
    let manifest: Value =
        serde_json::from_slice(&outcome.members[0].bytes).expect("manifest parses");
    assert_eq!(manifest["diagnostics"]["export_blocked"], true);
}

#[test]
fn missing_named_members_block_the_package() {
    let mut fixture = Fixture::new();
    fixture.result_envelopes.clear();
    fixture.sections_records.clear();
    let outcome = assemble_report_package_container(&fixture.input());

    assert!(outcome.export_blocked);
    assert!(outcome
        .blocking_reasons
        .iter()
        .any(|reason| reason.contains("at least one result export member")));
    assert!(outcome
        .blocking_reasons
        .iter()
        .any(|reason| reason.contains("state/comparison/handoff report-section member")));
}

#[test]
fn non_del_08_06_sections_record_blocks_the_package() {
    let mut fixture = Fixture::new();
    fixture.sections_records = vec![json!({
        "deliverable_id": "DEL-99-99",
        "section_set_id": "invented-wrong-envelope"
    })];
    let outcome = assemble_report_package_container(&fixture.input());

    assert!(outcome.export_blocked);
    assert!(outcome
        .blocking_reasons
        .iter()
        .any(|reason| reason.contains("DEL-08-06")));
}

#[test]
fn blocked_report_input_blocks_the_package_and_still_assembles() {
    let mut fixture = Fixture::new();
    fixture.report.calculation_report.report_id = String::new();
    let outcome = assemble_report_package_container(&fixture.input());

    assert!(outcome.export_blocked);
    assert!(outcome.report.export_blocked);
    assert!(!outcome.container_bytes.is_empty());
    assert_eq!(outcome.members.len(), 6);
}

#[test]
fn package_file_name_is_sanitized_and_opsproj_suffixed() {
    let fixture = Fixture::new();
    let mut input = fixture.input();
    input.package_id = "Invented Package 001".to_string();
    let outcome = assemble_report_package_container(&input);

    assert_eq!(outcome.container_file_name, "invented-package-001.opsproj");
    assert!(!outcome.export_blocked);
}
