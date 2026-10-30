//! Determinism, inventory, gate, and archive-form tests for the full
//! report-package container (DEC-028 / DEC-057 / DEC-061, row E3). All data
//! invented; no protected standards content.

mod common;

use common::{invented_result_document_variant, Fixture};
use open_pipe_stress_report_package::{
    assemble_report_package_container, CONTAINER_DOCUMENT_KIND, CONTAINER_EXTENSION,
    PACKAGE_MANIFEST_FILE_NAME, ROLE_AUDIT_MANIFEST, ROLE_PACKAGE_MANIFEST, ROLE_REPORT_HTML,
    ROLE_REPORT_PDF, ROLE_RESULT_EXPORT, ROLE_STATE_COMPARISON_HANDOFF,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

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
            "result_export_result-envelope-run-headless-preview-del1005-export-results-invented.json",
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
    changed_fixture.result_documents = vec![invented_result_document_variant(None, Some(99.75))];
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
    fixture.result_documents.clear();
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
