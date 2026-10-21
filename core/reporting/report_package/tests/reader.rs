//! `.opsproj` reader tests: current-version round trip, one-version refusal
//! by name (`DEC-028`; owner, 2026-10-10: "Close: one version"), malformed
//! and partial containers, and the absence of any migration path. All data
//! invented; no protected standards content.

mod common;

use common::{invented_result_envelope, invented_sections_record, Fixture};
use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_report_package::{
    assemble_report_package_container, read_report_package_container, PackageReadRefusal,
    ReportPackageContainerOutcome, PACKAGE_MANIFEST_SCHEMA_VERSION, SUPPORTED_PACKAGE_VERSION,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

// ---------------------------------------------------------------------------
// Test-side archive builder, independent of the writer.
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct Entry {
    name: String,
    data: Vec<u8>,
    method: u16,
    flags: u16,
    dos_time: u16,
    dos_date: u16,
    extra: Vec<u8>,
}

impl Entry {
    fn new(name: &str, data: Vec<u8>) -> Self {
        Self {
            name: name.to_string(),
            data,
            method: 0,
            flags: 0,
            dos_time: 0,
            dos_date: 0x0021,
            extra: Vec::new(),
        }
    }
}

fn crc32(bytes: &[u8]) -> u32 {
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

fn zip(entries: &[Entry]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut central = Vec::new();
    for entry in entries {
        let offset = out.len() as u32;
        let header = |buffer: &mut Vec<u8>| {
            buffer.extend_from_slice(&entry.flags.to_le_bytes());
            buffer.extend_from_slice(&entry.method.to_le_bytes());
            buffer.extend_from_slice(&entry.dos_time.to_le_bytes());
            buffer.extend_from_slice(&entry.dos_date.to_le_bytes());
            buffer.extend_from_slice(&crc32(&entry.data).to_le_bytes());
            buffer.extend_from_slice(&(entry.data.len() as u32).to_le_bytes());
            buffer.extend_from_slice(&(entry.data.len() as u32).to_le_bytes());
            buffer.extend_from_slice(&(entry.name.len() as u16).to_le_bytes());
            buffer.extend_from_slice(&(entry.extra.len() as u16).to_le_bytes());
        };
        out.extend_from_slice(&0x0403_4b50u32.to_le_bytes());
        out.extend_from_slice(&20u16.to_le_bytes());
        header(&mut out);
        out.extend_from_slice(entry.name.as_bytes());
        out.extend_from_slice(&entry.extra);
        out.extend_from_slice(&entry.data);

        central.extend_from_slice(&0x0201_4b50u32.to_le_bytes());
        central.extend_from_slice(&20u16.to_le_bytes());
        central.extend_from_slice(&20u16.to_le_bytes());
        header(&mut central);
        central.extend_from_slice(&[0u8; 10]); // comment, disk, attributes
        central.extend_from_slice(&offset.to_le_bytes());
        central.extend_from_slice(entry.name.as_bytes());
        central.extend_from_slice(&entry.extra);
    }
    let central_offset = out.len() as u32;
    let central_size = central.len() as u32;
    out.extend_from_slice(&central);
    out.extend_from_slice(&0x0605_4b50u32.to_le_bytes());
    out.extend_from_slice(&[0u8; 4]);
    out.extend_from_slice(&(entries.len() as u16).to_le_bytes());
    out.extend_from_slice(&(entries.len() as u16).to_le_bytes());
    out.extend_from_slice(&central_size.to_le_bytes());
    out.extend_from_slice(&central_offset.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn package() -> ReportPackageContainerOutcome {
    let outcome = assemble_report_package_container(&Fixture::new().input());
    assert!(!outcome.export_blocked, "{:?}", outcome.blocking_reasons);
    outcome
}

fn entries(outcome: &ReportPackageContainerOutcome) -> Vec<Entry> {
    outcome
        .members
        .iter()
        .map(|member| Entry::new(&member.file_name, member.bytes.clone()))
        .collect()
}

fn manifest(entries: &[Entry]) -> Value {
    serde_json::from_slice(&entries[0].data).expect("manifest parses")
}

fn set_manifest(entries: &mut [Entry], manifest: &Value) {
    entries[0].data = canonical_json(manifest).into_bytes();
}

/// Replace a member's bytes and record its new length and hash in the
/// manifest, so only the property under test is defective.
fn replace_member_consistently(entries: &mut [Entry], index: usize, data: Vec<u8>) {
    let mut document = manifest(entries);
    let row = &mut document["members"][index - 1];
    row["sha256_hex"] = json!(sha256_hex(&data));
    row["byte_length"] = json!(data.len());
    entries[index].data = data;
    set_manifest(entries, &document);
}

fn with_manifest(edit: impl FnOnce(&mut Value)) -> Vec<u8> {
    let mut entries = entries(&package());
    let mut document = manifest(&entries);
    edit(&mut document);
    set_manifest(&mut entries, &document);
    zip(&entries)
}

fn with_version(version: Value) -> Vec<u8> {
    with_manifest(|manifest| manifest["schema_version"] = version)
}

type ManifestEdit = Box<dyn FnOnce(&mut Value)>;

fn refusal(bytes: &[u8]) -> PackageReadRefusal {
    read_report_package_container(bytes).expect_err("container must be refused")
}

// ---------------------------------------------------------------------------
// Current version round trip.
// ---------------------------------------------------------------------------

#[test]
fn test_builder_reproduces_the_writer_archive_form() {
    let outcome = package();
    assert_eq!(zip(&entries(&outcome)), outcome.container_bytes);
}

#[test]
fn current_version_round_trips() {
    let outcome = package();
    let read = read_report_package_container(&outcome.container_bytes).expect("reads");

    assert_eq!(read.schema_version, SUPPORTED_PACKAGE_VERSION);
    assert_eq!(read.members, outcome.members);
    assert_eq!(
        read.package_identity_sha256_hex,
        outcome.package_identity_sha256_hex
    );
    assert_eq!(read.container_sha256_hex, outcome.container_sha256_hex);
    assert_eq!(read.package_id, "invented-package-001");
    assert_eq!(
        read.export_profile_id,
        "invented-profile-native-full-report"
    );
    assert_eq!(read.source_model_ref.ref_type, "model");
    assert_eq!(read.source_model_ref.ref_id, "invented-model");
    assert_eq!(read.source_basis_refs.len(), 1);
    assert_eq!(read.source_basis_refs[0].ref_id, "invented-run");
    assert!(!read.export_blocked);
    assert!(read.blocking_reasons.is_empty());
    assert_eq!(read.manifest["schema_version"], "1.0.0");
}

#[test]
fn multi_member_package_round_trips() {
    let mut fixture = Fixture::new();
    let mut second = invented_result_envelope(7.25);
    second.envelope_id = "invented-result-envelope-002".to_string();
    fixture.result_envelopes.push(second);
    let mut sections = invented_sections_record();
    sections["section_set_id"] = json!("invented-section-set-002");
    fixture.sections_records.push(sections);
    let outcome = assemble_report_package_container(&fixture.input());
    assert!(!outcome.export_blocked, "{:?}", outcome.blocking_reasons);
    assert_eq!(outcome.members.len(), 8);

    let read = read_report_package_container(&outcome.container_bytes).expect("reads");
    assert_eq!(read.members, outcome.members);
}

#[test]
fn blocked_package_reads_with_its_blocked_state() {
    let mut fixture = Fixture::new();
    fixture.result_envelopes.clear();
    fixture.sections_records.clear();
    let outcome = assemble_report_package_container(&fixture.input());
    assert!(outcome.export_blocked);

    let read = read_report_package_container(&outcome.container_bytes).expect("reads");
    assert!(read.export_blocked);
    assert_eq!(read.blocking_reasons, outcome.blocking_reasons);
    assert_eq!(read.members, outcome.members);
}

// ---------------------------------------------------------------------------
// One version: older, newer and unrecognized versions are refused by name.
// ---------------------------------------------------------------------------

#[test]
fn supported_version_is_the_writer_version() {
    assert_eq!(SUPPORTED_PACKAGE_VERSION, PACKAGE_MANIFEST_SCHEMA_VERSION);
    assert_eq!(SUPPORTED_PACKAGE_VERSION, "1.0.0");
}

#[test]
fn older_versions_are_refused_by_name_with_a_re_author_diagnostic() {
    for version in ["0.9.0", "0.1.0", "0.0.1", "0.99.99"] {
        let refused = refusal(&with_version(json!(version)));
        assert_eq!(
            refused,
            PackageReadRefusal::OlderVersion {
                found: version.to_string()
            }
        );
        assert_eq!(refused.code(), "OPSPROJ_PACKAGE_VERSION_OLDER");
        let message = refused.to_string();
        assert!(message.contains(version), "{message}");
        assert!(message.contains("1.0.0"), "{message}");
        assert!(message.contains("does not migrate"), "{message}");
        assert!(message.contains("Re-author"), "{message}");
    }
}

#[test]
fn newer_versions_are_refused_by_name() {
    for version in ["1.0.1", "1.1.0", "2.0.0", "10.0.0"] {
        let refused = refusal(&with_version(json!(version)));
        assert_eq!(
            refused,
            PackageReadRefusal::NewerVersion {
                found: version.to_string()
            }
        );
        assert_eq!(refused.code(), "OPSPROJ_PACKAGE_VERSION_NEWER");
        assert!(refused.to_string().contains(version));
    }
}

#[test]
fn unrecognized_versions_are_refused_by_name() {
    let cases = [
        (json!("1.0"), "1.0"),
        (json!("1.0.0.0"), "1.0.0.0"),
        (json!("1.0.0-rc.1"), "1.0.0-rc.1"),
        (json!("1.0.0+build"), "1.0.0+build"),
        (json!("01.0.0"), "01.0.0"),
        (json!("v1.0.0"), "v1.0.0"),
        (json!(" 1.0.0"), " 1.0.0"),
        (json!(""), ""),
        (
            json!("99999999999999999999.0.0"),
            "99999999999999999999.0.0",
        ),
        (json!(1), "1"),
        (json!(null), "null"),
    ];
    for (value, found) in cases {
        let refused = refusal(&with_version(value));
        assert_eq!(
            refused,
            PackageReadRefusal::UnrecognizedVersion {
                found: found.to_string()
            }
        );
        assert_eq!(refused.code(), "OPSPROJ_PACKAGE_VERSION_UNRECOGNIZED");
    }

    let missing = refusal(&with_manifest(|manifest| {
        manifest.as_object_mut().unwrap().remove("schema_version");
    }));
    assert_eq!(
        missing,
        PackageReadRefusal::UnrecognizedVersion {
            found: "<missing>".to_string()
        }
    );
}

#[test]
fn version_refusal_precedes_every_other_package_defect() {
    // An older package whose manifest and members follow another form is
    // still refused by its version name, not by its form.
    let mut entries = entries(&package());
    let mut document = manifest(&entries);
    document["schema_version"] = json!("0.9.0");
    document["members"] = json!("a different inventory form");
    document["legacy_field"] = json!(true);
    entries[0].data = serde_json::to_vec_pretty(&document).unwrap();
    entries[1].method = 8;
    entries[2].dos_date = 0x5000;
    entries.truncate(3);
    assert_eq!(
        refusal(&zip(&entries)),
        PackageReadRefusal::OlderVersion {
            found: "0.9.0".to_string()
        }
    );
}

// ---------------------------------------------------------------------------
// Not a package.
// ---------------------------------------------------------------------------

#[test]
fn wrong_document_kind_is_not_a_package() {
    for kind in [json!("Some Other Package"), json!(null)] {
        let refused = refusal(&with_manifest(|manifest| manifest["document_kind"] = kind));
        assert_eq!(refused.code(), "OPSPROJ_NOT_A_PACKAGE");
    }
}

#[test]
fn manifest_must_be_the_first_member() {
    let mut entries = entries(&package());
    entries.swap(0, 1);
    assert_eq!(refusal(&zip(&entries)).code(), "OPSPROJ_NOT_A_PACKAGE");
}

// ---------------------------------------------------------------------------
// Malformed and partial containers.
// ---------------------------------------------------------------------------

#[test]
fn empty_and_non_archive_inputs_are_refused() {
    for bytes in [
        Vec::new(),
        b"not an archive".to_vec(),
        vec![0u8; 64],
        br#"{"document_kind":"OpenPipeStress Project Package"}"#.to_vec(),
    ] {
        assert_eq!(refusal(&bytes).code(), "OPSPROJ_CONTAINER_MALFORMED");
    }
}

#[test]
fn every_truncation_is_refused() {
    let bytes = package().container_bytes;
    for length in 0..bytes.len() {
        assert!(
            read_report_package_container(&bytes[..length]).is_err(),
            "prefix of length {length} must be refused"
        );
    }
}

#[test]
fn every_single_byte_change_is_refused() {
    // Every byte outside member data (local headers, names, central
    // directory, EOCD), and every 31st byte of member data.
    let outcome = package();
    let bytes = outcome.container_bytes;
    let mut data_ranges = Vec::new();
    let mut offset = 0;
    for member in &outcome.members {
        let start = offset + 30 + member.file_name.len();
        offset = start + member.bytes.len();
        data_ranges.push(start..offset);
    }
    let positions = (0..bytes.len()).filter(|index| {
        data_ranges
            .iter()
            .all(|range| !range.contains(index) || (index - range.start) % 31 == 0)
    });
    let mut mutated = bytes.clone();
    for index in positions {
        mutated[index] ^= 0x01;
        assert!(
            read_report_package_container(&mutated).is_err(),
            "flipping byte {index} must be refused"
        );
        mutated[index] = bytes[index];
    }
}

#[test]
fn trailing_and_leading_bytes_are_refused() {
    let bytes = package().container_bytes;
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert_eq!(refusal(&trailing).code(), "OPSPROJ_CONTAINER_MALFORMED");

    let mut leading = vec![0u8];
    leading.extend_from_slice(&bytes);
    assert!(read_report_package_container(&leading).is_err());
}

#[test]
fn archive_comment_is_refused() {
    let mut bytes = package().container_bytes;
    let length = bytes.len();
    bytes[length - 2..].copy_from_slice(&3u16.to_le_bytes());
    bytes.extend_from_slice(b"abc");
    assert_eq!(refusal(&bytes).code(), "OPSPROJ_CONTAINER_MALFORMED");
}

#[test]
fn compressed_encrypted_and_extra_field_members_are_refused() {
    let base = entries(&package());
    let mut cases: Vec<(&str, Vec<Entry>)> = Vec::new();

    let mut compressed = base.clone();
    compressed[2].method = 8;
    cases.push(("compressed", compressed));

    let mut encrypted = base.clone();
    encrypted[2].flags = 0x0001;
    cases.push(("flags", encrypted));

    let mut descriptor = base.clone();
    descriptor[3].flags = 0x0008;
    cases.push(("flags", descriptor));

    let mut extra = base.clone();
    extra[1].extra = vec![0x55, 0x54, 0x01, 0x00, 0x00];
    cases.push(("extra", extra));

    for (needle, entries) in cases {
        let refused = refusal(&zip(&entries));
        assert_eq!(refused.code(), "OPSPROJ_MEMBER_INTEGRITY", "{needle}");
        assert!(refused.to_string().contains(needle), "{needle}: {refused}");
    }
}

#[test]
fn compressed_manifest_is_refused() {
    let mut entries = entries(&package());
    entries[0].method = 8;
    assert_eq!(
        refusal(&zip(&entries)).code(),
        "OPSPROJ_CONTAINER_MALFORMED"
    );
}

#[test]
fn non_fixed_timestamps_are_refused() {
    let mut entries = entries(&package());
    entries[4].dos_time = 0x6000;
    entries[4].dos_date = 0x5921;
    assert_eq!(
        refusal(&zip(&entries)).code(),
        "OPSPROJ_CONTAINER_MALFORMED"
    );
}

#[test]
fn unsafe_and_duplicate_member_names_are_refused() {
    for name in [
        "../escape.json",
        "dir/member.json",
        "/abs.json",
        ".hidden",
        "UPPER.json",
    ] {
        let mut entries = entries(&package());
        entries.push(Entry::new(name, b"{}".to_vec()));
        let refused = refusal(&zip(&entries));
        assert_eq!(refused.code(), "OPSPROJ_MEMBER_INTEGRITY", "{name}");
        assert!(refused.to_string().contains("plain package member name"));
    }

    let mut entries = entries(&package());
    let duplicate = entries[3].clone();
    entries.push(duplicate);
    assert!(refusal(&zip(&entries))
        .to_string()
        .contains("duplicate member name"));
}

#[test]
fn gap_between_members_is_refused() {
    let outcome = package();
    let bytes = outcome.container_bytes;
    // Insert one byte after the first member's data and shift every later
    // offset, so only the gap is defective.
    let first_end = 30 + outcome.members[0].file_name.len() + outcome.members[0].bytes.len();
    let mut gapped = bytes[..first_end].to_vec();
    gapped.push(0);
    gapped.extend_from_slice(&bytes[first_end..]);
    let eocd = gapped.len() - 22;
    let central_offset = u32::from_le_bytes(gapped[eocd + 16..eocd + 20].try_into().unwrap()) + 1;
    gapped[eocd + 16..eocd + 20].copy_from_slice(&central_offset.to_le_bytes());
    let mut cursor = central_offset as usize;
    for index in 0..outcome.members.len() {
        let name_len = u16::from_le_bytes(gapped[cursor + 28..cursor + 30].try_into().unwrap());
        if index > 0 {
            let offset =
                u32::from_le_bytes(gapped[cursor + 42..cursor + 46].try_into().unwrap()) + 1;
            gapped[cursor + 42..cursor + 46].copy_from_slice(&offset.to_le_bytes());
        }
        cursor += 46 + name_len as usize;
    }
    assert!(refusal(&gapped).to_string().contains("not contiguous"));
}

// ---------------------------------------------------------------------------
// Manifest and member integrity.
// ---------------------------------------------------------------------------

#[test]
fn non_canonical_manifest_is_refused() {
    let mut entries = entries(&package());
    entries[0].data = serde_json::to_vec_pretty(&manifest(&entries)).unwrap();
    assert_eq!(
        refusal(&zip(&entries)),
        PackageReadRefusal::MalformedManifest {
            reason: "manifest bytes are not canonical JSON".to_string()
        }
    );
}

#[test]
fn manifest_must_equal_the_current_version_form() {
    let cases: Vec<(&str, ManifestEdit)> = vec![
        (
            "professional_boundary",
            Box::new(|m| {
                m["professional_boundary"]["software_makes_compliance_claim"] = json!(true)
            }),
        ),
        (
            "boundary_notes",
            Box::new(|m| m["boundary_notes"] = json!([])),
        ),
        ("legacy_field", Box::new(|m| m["legacy_field"] = json!(1))),
        (
            "decision_basis",
            Box::new(|m| m["decision_basis"] = json!(["DEC-028"])),
        ),
        (
            "artifact",
            Box::new(|m| m["artifact"] = json!("openpipestress.other")),
        ),
        (
            "container_extension",
            Box::new(|m| m["container_extension"] = json!("zip")),
        ),
    ];
    for (key, edit) in cases {
        let refused = refusal(&with_manifest(edit));
        assert_eq!(refused.code(), "OPSPROJ_MANIFEST_MALFORMED", "{key}");
        assert!(refused.to_string().contains(key), "{key}: {refused}");
    }
}

#[test]
fn inconsistent_blocked_state_is_refused() {
    let refused = refusal(&with_manifest(|m| {
        m["diagnostics"]["export_blocked"] = json!(true)
    }));
    assert!(refused.to_string().contains("export_blocked disagrees"));
}

#[test]
fn member_hash_and_length_must_match_the_manifest() {
    // Tampered member data with a consistent CRC: the manifest hash catches it.
    let mut entries = entries(&package());
    let html = String::from_utf8(entries[1].data.clone()).unwrap();
    entries[1].data = html.replacen("Invented", "Altered!", 1).into_bytes();
    assert!(refusal(&zip(&entries)).to_string().contains("SHA-256"));

    let mut entries = self::entries(&package());
    entries[1].data.push(b'\n');
    assert!(refusal(&zip(&entries)).to_string().contains("byte length"));
}

#[test]
fn missing_extra_and_reordered_members_are_refused() {
    let base = entries(&package());

    let mut missing = base.clone();
    missing.pop();
    assert!(refusal(&zip(&missing))
        .to_string()
        .contains("manifest lists 5 members but the archive holds 4"));

    let mut extra = base.clone();
    extra.push(Entry::new("result_export_extra.json", b"{}".to_vec()));
    assert_eq!(refusal(&zip(&extra)).code(), "OPSPROJ_MANIFEST_MALFORMED");

    let mut reordered = base.clone();
    reordered.swap(1, 2);
    assert!(refusal(&zip(&reordered))
        .to_string()
        .contains("where the archive holds"));

    // Reordered consistently in both archive and manifest: the fixed member
    // order still refuses it.
    let mut consistent = base.clone();
    consistent.swap(1, 2);
    let mut document = manifest(&consistent);
    let rows = document["members"].as_array_mut().unwrap();
    rows.swap(0, 1);
    set_manifest(&mut consistent, &document);
    assert!(refusal(&zip(&consistent))
        .to_string()
        .contains("out of the fixed member order"));
}

#[test]
fn partial_package_without_required_members_is_refused() {
    let mut entries = entries(&package());
    entries.truncate(3); // manifest, HTML, PDF
    let mut document = manifest(&entries);
    document["members"].as_array_mut().unwrap().truncate(2);
    set_manifest(&mut entries, &document);
    assert!(refusal(&zip(&entries))
        .to_string()
        .contains("must list the rendered report HTML, PDF and audit manifest"));
}

#[test]
fn unknown_role_and_role_form_mismatches_are_refused() {
    let cases: Vec<(&str, ManifestEdit)> = vec![
        (
            "unknown member role",
            Box::new(|m| m["members"][2]["role"] = json!("legacy_role")),
        ),
        (
            "does not fit role",
            Box::new(|m| m["members"][2]["media_type"] = json!("text/plain")),
        ),
        (
            "does not fit role",
            Box::new(|m| m["members"][0]["hash_basis"] = json!("jcs_canonical_json_bytes")),
        ),
        (
            "does not fit role",
            Box::new(|m| m["members"][3]["carries_canonical_identity"] = json!(false)),
        ),
        (
            "must be a string",
            Box::new(|m| m["members"][3]["sha256_hex"] = json!(1)),
        ),
    ];
    for (needle, edit) in cases {
        let refused = refusal(&with_manifest(edit));
        assert!(refused.to_string().contains(needle), "{needle}: {refused}");
    }
}

#[test]
fn non_canonical_json_member_is_refused() {
    let mut entries = entries(&package());
    let document: Value = serde_json::from_slice(&entries[3].data).unwrap();
    replace_member_consistently(
        &mut entries,
        3,
        serde_json::to_vec_pretty(&document).unwrap(),
    );
    let refused = refusal(&zip(&entries));
    assert_eq!(refused.code(), "OPSPROJ_MEMBER_INTEGRITY");
    assert!(refused.to_string().contains("not canonical JSON"));

    let mut entries = self::entries(&package());
    replace_member_consistently(&mut entries, 3, b"not json".to_vec());
    assert!(refusal(&zip(&entries)).to_string().contains("not JSON"));
}

#[test]
fn rendered_members_must_keep_their_form() {
    let mut entries = entries(&package());
    replace_member_consistently(&mut entries, 2, b"not a pdf".to_vec());
    assert!(refusal(&zip(&entries)).to_string().contains("PDF header"));

    let mut entries = self::entries(&package());
    replace_member_consistently(&mut entries, 1, vec![0xff, 0xfe]);
    assert!(refusal(&zip(&entries)).to_string().contains("not UTF-8"));
}

// ---------------------------------------------------------------------------
// No migration path.
// ---------------------------------------------------------------------------

#[test]
fn reader_source_defines_no_migration_or_upgrade_path() {
    for source in [
        include_str!("../src/reader.rs"),
        include_str!("../src/lib.rs"),
    ] {
        for line in source.lines() {
            let code = line.trim_start();
            if code.starts_with("//") {
                continue;
            }
            let lower = code.to_ascii_lowercase();
            assert!(
                !(lower.contains("fn ") && (lower.contains("migrat") || lower.contains("upgrade"))),
                "no migration or upgrade function may exist: {line}"
            );
        }
    }
}
