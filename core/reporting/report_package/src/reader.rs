//! `.opsproj` package reader with one-version refusal (`DEC-028`; owner,
//! 2026-10-10: "Close: one version").
//!
//! The reader accepts only the current package version,
//! [`SUPPORTED_PACKAGE_VERSION`], exactly as this crate's writer emits it.
//! An older, newer or unrecognized version is refused by name, before the rest
//! of the package is interpreted. There is no migration: the reader returns
//! either the verified current-version package or a refusal, and never
//! rewrites, upgrades or partially returns a package.
//!
//! Reading proceeds in four steps, each refusing on the first defect:
//!
//! 1. Locate the archive structure (end-of-central-directory record, central
//!    directory, local entries) with bounds-checked parsing.
//! 2. Read the first member, which must be the package manifest, check the
//!    `DEC-057` document kind, then gate on `schema_version`.
//! 3. Check the archive form: stored members only, matching local and central
//!    headers, contiguous entries, no extra fields or comments, unique names,
//!    CRC-32 per member.
//! 4. Check the manifest and members: the manifest is canonical JSON and equals
//!    the current-version manifest rebuilt from its variable fields; the member
//!    inventory matches the archive in order, names, roles, lengths and
//!    SHA-256; canonical-JSON members are canonical. Finally the archive must
//!    re-encode to identical bytes.
//!
//! Evidence binds to the canonical members and the manifest's per-member
//! hashes (`DEC-028`); the manifest hash is the package identity. Reading a
//! package establishes its integrity against its own manifest only. It is not
//! an acceptance or issuance act. Standard claim fence applies (F-PIP-2; claims
//! taxonomy per DEC-081).

use std::collections::BTreeSet;
use std::fmt;

use open_pipe_stress_canonical_json::canonical_json;
use serde_json::{Map, Value};

use super::{
    build_deterministic_zip, crc32, package_manifest_document, sha256_hex, ContainerMember,
    ManifestFields, PackageRef, CONTAINER_DOCUMENT_KIND, HASH_BASIS_EXACT, HASH_BASIS_JCS,
    PACKAGE_MANIFEST_FILE_NAME, PACKAGE_MANIFEST_SCHEMA_VERSION, ROLE_AUDIT_MANIFEST,
    ROLE_PACKAGE_MANIFEST, ROLE_REPORT_HTML, ROLE_REPORT_PDF, ROLE_RESULT_EXPORT,
    ROLE_STATE_COMPARISON_HANDOFF,
};

/// The only package version this reader accepts (`DEC-028`, one version).
pub const SUPPORTED_PACKAGE_VERSION: &str = PACKAGE_MANIFEST_SCHEMA_VERSION;

/// A verified current-version package.
#[derive(Debug, Clone, PartialEq)]
pub struct ReadReportPackage {
    /// Always [`SUPPORTED_PACKAGE_VERSION`].
    pub schema_version: String,
    pub package_id: String,
    pub export_profile_id: String,
    pub source_model_ref: PackageRef,
    pub source_basis_refs: Vec<PackageRef>,
    /// SHA-256 of the manifest member bytes: the package identity (`DEC-028`).
    pub package_identity_sha256_hex: String,
    /// SHA-256 of the container bytes (recorded; not the evidence binding).
    pub container_sha256_hex: String,
    /// The parsed manifest document.
    pub manifest: Value,
    /// All members in archive order; the manifest member is first.
    pub members: Vec<ContainerMember>,
    /// The writer's export-blocked state, as recorded in the manifest. A
    /// blocked package reads so it can be diagnosed; callers honor it.
    pub export_blocked: bool,
    pub blocking_reasons: Vec<String>,
}

/// Why a container was refused. Every refusal is final: nothing of the
/// package is returned and nothing is migrated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageReadRefusal {
    /// The manifest names a version older than [`SUPPORTED_PACKAGE_VERSION`].
    OlderVersion { found: String },
    /// The manifest names a version newer than [`SUPPORTED_PACKAGE_VERSION`].
    NewerVersion { found: String },
    /// The manifest's version is missing or is not a `MAJOR.MINOR.PATCH`
    /// version string. `found` is the raw value, or `<missing>`.
    UnrecognizedVersion { found: String },
    /// The container is not an OpenPipeStress Project Package.
    NotAPackage { reason: String },
    /// The archive structure is malformed, truncated or not in the
    /// package's archive form.
    MalformedContainer { reason: String },
    /// The manifest is not the current-version manifest form.
    MalformedManifest { reason: String },
    /// A member disagrees with the manifest inventory or its own form.
    MemberIntegrity { file_name: String, reason: String },
}

impl PackageReadRefusal {
    /// Stable diagnostic code.
    pub fn code(&self) -> &'static str {
        match self {
            Self::OlderVersion { .. } => "OPSPROJ_PACKAGE_VERSION_OLDER",
            Self::NewerVersion { .. } => "OPSPROJ_PACKAGE_VERSION_NEWER",
            Self::UnrecognizedVersion { .. } => "OPSPROJ_PACKAGE_VERSION_UNRECOGNIZED",
            Self::NotAPackage { .. } => "OPSPROJ_NOT_A_PACKAGE",
            Self::MalformedContainer { .. } => "OPSPROJ_CONTAINER_MALFORMED",
            Self::MalformedManifest { .. } => "OPSPROJ_MANIFEST_MALFORMED",
            Self::MemberIntegrity { .. } => "OPSPROJ_MEMBER_INTEGRITY",
        }
    }
}

impl fmt::Display for PackageReadRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let supported = SUPPORTED_PACKAGE_VERSION;
        match self {
            Self::OlderVersion { found } => write!(
                f,
                "{}: package version {found} is older than the supported version {supported}; \
                 this reader does not migrate packages. Re-author the package from its source \
                 project with the current version.",
                self.code()
            ),
            Self::NewerVersion { found } => write!(
                f,
                "{}: package version {found} is newer than the supported version {supported}; \
                 open it with the version that wrote it.",
                self.code()
            ),
            Self::UnrecognizedVersion { found } => write!(
                f,
                "{}: package version {found} is not a recognized version; only {supported} is \
                 supported.",
                self.code()
            ),
            Self::NotAPackage { reason }
            | Self::MalformedContainer { reason }
            | Self::MalformedManifest { reason } => write!(f, "{}: {reason}", self.code()),
            Self::MemberIntegrity { file_name, reason } => {
                write!(f, "{}: member {file_name}: {reason}", self.code())
            }
        }
    }
}

impl std::error::Error for PackageReadRefusal {}

/// Read and verify an `.opsproj` container. Accepts only
/// [`SUPPORTED_PACKAGE_VERSION`]; refuses every other version by name.
pub fn read_report_package_container(
    bytes: &[u8],
) -> Result<ReadReportPackage, PackageReadRefusal> {
    // 1. Archive structure.
    let archive = parse_archive(bytes)?;

    // 2. Manifest member, document kind, version gate.
    let manifest_entry = &archive.entries[0];
    if manifest_entry.name != PACKAGE_MANIFEST_FILE_NAME.as_bytes() {
        return Err(not_a_package(format!(
            "the first archive member must be {PACKAGE_MANIFEST_FILE_NAME}"
        )));
    }
    if manifest_entry.central.method != 0
        || manifest_entry.central.flags != 0
        || manifest_entry.central.compressed_size != manifest_entry.central.uncompressed_size
    {
        return Err(malformed_container(
            "the package manifest member must be stored without compression or encryption",
        ));
    }
    let manifest_bytes = manifest_entry.data(bytes);
    let manifest: Value = serde_json::from_slice(manifest_bytes)
        .map_err(|error| malformed_manifest(format!("manifest is not JSON: {error}")))?;
    let manifest_object = manifest
        .as_object()
        .ok_or_else(|| malformed_manifest("manifest must be a JSON object"))?;
    if manifest_object.get("document_kind").and_then(Value::as_str) != Some(CONTAINER_DOCUMENT_KIND)
    {
        return Err(not_a_package(format!(
            "manifest document_kind must be \"{CONTAINER_DOCUMENT_KIND}\""
        )));
    }
    check_version(manifest_object.get("schema_version"))?;

    // 3. Archive form.
    check_archive_form(bytes, &archive)?;

    // 4. Manifest and members.
    if canonical_json(&manifest).as_bytes() != manifest_bytes {
        return Err(malformed_manifest("manifest bytes are not canonical JSON"));
    }
    let fields = manifest_fields(manifest_object)?;
    let rows = manifest_member_rows(manifest_object)?;
    let members = verify_members(bytes, &archive, &rows)?;

    let rebuilt = package_manifest_document(
        &ManifestFields {
            package_id: &fields.package_id,
            export_profile_id: &fields.export_profile_id,
            source_model_ref: &fields.source_model_ref,
            source_basis_refs: &fields.source_basis_refs,
            export_blocked: fields.export_blocked,
            blocking_reasons: &fields.blocking_reasons,
        },
        &members[1..],
    );
    if rebuilt != manifest {
        return Err(malformed_manifest(format!(
            "manifest differs from the version {SUPPORTED_PACKAGE_VERSION} manifest form at: {}",
            differing_keys(&rebuilt, &manifest).join(", ")
        )));
    }

    if build_deterministic_zip(&members) != bytes {
        return Err(malformed_container(format!(
            "archive encoding differs from the version {SUPPORTED_PACKAGE_VERSION} archive form \
             (timestamps, attributes or header version fields)"
        )));
    }

    Ok(ReadReportPackage {
        schema_version: SUPPORTED_PACKAGE_VERSION.to_string(),
        package_id: fields.package_id,
        export_profile_id: fields.export_profile_id,
        source_model_ref: fields.source_model_ref,
        source_basis_refs: fields.source_basis_refs,
        package_identity_sha256_hex: members[0].sha256_hex.clone(),
        container_sha256_hex: sha256_hex(bytes),
        manifest,
        members,
        export_blocked: fields.export_blocked,
        blocking_reasons: fields.blocking_reasons,
    })
}

// ---------------------------------------------------------------------------
// Version gate.
// ---------------------------------------------------------------------------

fn check_version(value: Option<&Value>) -> Result<(), PackageReadRefusal> {
    let found = match value {
        None => {
            return Err(PackageReadRefusal::UnrecognizedVersion {
                found: "<missing>".to_string(),
            })
        }
        Some(Value::String(text)) => text.clone(),
        Some(other) => {
            return Err(PackageReadRefusal::UnrecognizedVersion {
                found: other.to_string(),
            })
        }
    };
    let supported =
        parse_version(SUPPORTED_PACKAGE_VERSION).expect("supported version is MAJOR.MINOR.PATCH");
    match parse_version(&found) {
        None => Err(PackageReadRefusal::UnrecognizedVersion { found }),
        Some(version) if version < supported => Err(PackageReadRefusal::OlderVersion { found }),
        Some(version) if version > supported => Err(PackageReadRefusal::NewerVersion { found }),
        Some(_) => Ok(()),
    }
}

/// Strict `MAJOR.MINOR.PATCH`: decimal digits, no leading zeros, no
/// pre-release or build suffix.
fn parse_version(text: &str) -> Option<(u64, u64, u64)> {
    let parts: Vec<&str> = text.split('.').collect();
    if parts.len() != 3 {
        return None;
    }
    let mut numbers = [0u64; 3];
    for (slot, part) in numbers.iter_mut().zip(parts) {
        if part.is_empty()
            || !part.bytes().all(|byte| byte.is_ascii_digit())
            || (part.len() > 1 && part.starts_with('0'))
        {
            return None;
        }
        *slot = part.parse().ok()?;
    }
    Some((numbers[0], numbers[1], numbers[2]))
}

// ---------------------------------------------------------------------------
// Archive structure.
// ---------------------------------------------------------------------------

const EOCD_SIGNATURE: u32 = 0x0605_4b50;
const CENTRAL_SIGNATURE: u32 = 0x0201_4b50;
const LOCAL_SIGNATURE: u32 = 0x0403_4b50;
const EOCD_LEN: usize = 22;
const CENTRAL_HEADER_LEN: usize = 46;
const LOCAL_HEADER_LEN: usize = 30;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HeaderFields {
    flags: u16,
    method: u16,
    dos_time: u16,
    dos_date: u16,
    crc32: u32,
    compressed_size: u32,
    uncompressed_size: u32,
}

#[derive(Debug)]
struct CentralEntry {
    central: HeaderFields,
    local: HeaderFields,
    name: Vec<u8>,
    local_name: Vec<u8>,
    central_extra_len: usize,
    comment_len: usize,
    disk_number: u16,
    local_extra_len: usize,
    local_offset: usize,
    data_start: usize,
    data_end: usize,
}

impl CentralEntry {
    fn data<'a>(&self, bytes: &'a [u8]) -> &'a [u8] {
        &bytes[self.data_start..self.data_end]
    }

    fn display_name(&self) -> String {
        String::from_utf8_lossy(&self.name).into_owned()
    }
}

#[derive(Debug)]
struct Archive {
    entries: Vec<CentralEntry>,
    central_offset: usize,
}

fn read_u16(bytes: &[u8], offset: usize) -> Option<u16> {
    let slice = bytes.get(offset..offset.checked_add(2)?)?;
    Some(u16::from_le_bytes([slice[0], slice[1]]))
}

fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    let slice = bytes.get(offset..offset.checked_add(4)?)?;
    Some(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
}

fn truncated(what: &str) -> PackageReadRefusal {
    malformed_container(format!("truncated or out-of-bounds {what}"))
}

fn parse_archive(bytes: &[u8]) -> Result<Archive, PackageReadRefusal> {
    if bytes.len() < EOCD_LEN {
        return Err(malformed_container(
            "container is too short to be a package archive",
        ));
    }
    let eocd = bytes.len() - EOCD_LEN;
    if read_u32(bytes, eocd) != Some(EOCD_SIGNATURE) {
        return Err(malformed_container(
            "end-of-central-directory record is not at the end of the container (truncated, \
             trailing bytes, archive comment, or not an archive)",
        ));
    }
    let field = |offset: usize| read_u16(bytes, eocd + offset).ok_or_else(|| truncated("EOCD"));
    let disk = field(4)?;
    let central_disk = field(6)?;
    let entries_on_disk = field(8)? as usize;
    let entry_count = field(10)? as usize;
    let comment_len = field(20)?;
    let central_size = read_u32(bytes, eocd + 12).ok_or_else(|| truncated("EOCD"))? as usize;
    let central_offset = read_u32(bytes, eocd + 16).ok_or_else(|| truncated("EOCD"))? as usize;
    if disk != 0 || central_disk != 0 || entries_on_disk != entry_count {
        return Err(malformed_container(
            "multi-disk archives are not package archives",
        ));
    }
    if entry_count == 0xFFFF || central_size == 0xFFFF_FFFF || central_offset == 0xFFFF_FFFF {
        return Err(malformed_container(
            "ZIP64 archives are not package archives",
        ));
    }
    if comment_len != 0 {
        return Err(malformed_container("archive comments are not permitted"));
    }
    if entry_count == 0 {
        return Err(malformed_container("archive has no members"));
    }
    if central_offset.checked_add(central_size) != Some(eocd) {
        return Err(malformed_container(
            "central directory does not end at the end-of-central-directory record",
        ));
    }

    let mut entries = Vec::with_capacity(entry_count);
    let mut cursor = central_offset;
    for _ in 0..entry_count {
        let c16 = |offset: usize| {
            read_u16(bytes, cursor + offset).ok_or_else(|| truncated("central directory entry"))
        };
        let c32 = |offset: usize| {
            read_u32(bytes, cursor + offset).ok_or_else(|| truncated("central directory entry"))
        };
        if cursor + CENTRAL_HEADER_LEN > eocd || c32(0)? != CENTRAL_SIGNATURE {
            return Err(malformed_container(
                "central directory entry signature missing",
            ));
        }
        let central = HeaderFields {
            flags: c16(8)?,
            method: c16(10)?,
            dos_time: c16(12)?,
            dos_date: c16(14)?,
            crc32: c32(16)?,
            compressed_size: c32(20)?,
            uncompressed_size: c32(24)?,
        };
        let name_len = c16(28)? as usize;
        let central_extra_len = c16(30)? as usize;
        let comment_len = c16(32)? as usize;
        let disk_number = c16(34)?;
        let local_offset = c32(42)? as usize;
        let name_start = cursor + CENTRAL_HEADER_LEN;
        let next = name_start + name_len + central_extra_len + comment_len;
        if next > eocd {
            return Err(truncated("central directory entry"));
        }
        let name = bytes[name_start..name_start + name_len].to_vec();

        let l16 = |offset: usize| {
            read_u16(bytes, local_offset + offset).ok_or_else(|| truncated("local header"))
        };
        let l32 = |offset: usize| {
            read_u32(bytes, local_offset + offset).ok_or_else(|| truncated("local header"))
        };
        if local_offset + LOCAL_HEADER_LEN > central_offset || l32(0)? != LOCAL_SIGNATURE {
            return Err(malformed_container(
                "local header signature missing at the recorded member offset",
            ));
        }
        let local = HeaderFields {
            flags: l16(6)?,
            method: l16(8)?,
            dos_time: l16(10)?,
            dos_date: l16(12)?,
            crc32: l32(14)?,
            compressed_size: l32(18)?,
            uncompressed_size: l32(22)?,
        };
        let local_name_len = l16(26)? as usize;
        let local_extra_len = l16(28)? as usize;
        let local_name_start = local_offset + LOCAL_HEADER_LEN;
        let data_start = local_name_start + local_name_len + local_extra_len;
        let data_end = data_start
            .checked_add(central.compressed_size as usize)
            .ok_or_else(|| truncated("member data"))?;
        if data_end > central_offset {
            return Err(truncated("member data"));
        }
        let local_name = bytes[local_name_start..local_name_start + local_name_len].to_vec();

        entries.push(CentralEntry {
            central,
            local,
            name,
            local_name,
            central_extra_len,
            comment_len,
            disk_number,
            local_extra_len,
            local_offset,
            data_start,
            data_end,
        });
        cursor = next;
    }
    if cursor != eocd {
        return Err(malformed_container(
            "central directory size disagrees with its entries",
        ));
    }
    Ok(Archive {
        entries,
        central_offset,
    })
}

fn check_archive_form(bytes: &[u8], archive: &Archive) -> Result<(), PackageReadRefusal> {
    let mut expected_offset = 0usize;
    let mut names = BTreeSet::new();
    for entry in &archive.entries {
        let name = entry.display_name();
        let defect = |reason: &str| PackageReadRefusal::MemberIntegrity {
            file_name: name.clone(),
            reason: reason.to_string(),
        };
        if entry.local != entry.central || entry.local_name != entry.name {
            return Err(defect("local header disagrees with the central directory"));
        }
        if entry.central.method != 0 {
            return Err(defect("member is compressed; package members are stored"));
        }
        if entry.central.flags != 0 {
            return Err(defect(
                "member sets general-purpose flags (encryption or data descriptor)",
            ));
        }
        if entry.central.compressed_size != entry.central.uncompressed_size {
            return Err(defect("stored member sizes disagree"));
        }
        if entry.central_extra_len != 0 || entry.local_extra_len != 0 || entry.comment_len != 0 {
            return Err(defect("member carries extra fields or a comment"));
        }
        if entry.disk_number != 0 {
            return Err(defect("member is on another disk"));
        }
        if entry.local_offset != expected_offset {
            return Err(defect(
                "member is not contiguous with the previous member (gap or overlap)",
            ));
        }
        if !is_safe_member_name(&entry.name) {
            return Err(defect("member name is not a plain package member name"));
        }
        if !names.insert(entry.name.clone()) {
            return Err(defect("duplicate member name"));
        }
        if crc32(entry.data(bytes)) != entry.central.crc32 {
            return Err(defect("CRC-32 mismatch"));
        }
        expected_offset = entry.data_end;
    }
    if expected_offset != archive.central_offset {
        return Err(malformed_container(
            "bytes between the last member and the central directory",
        ));
    }
    Ok(())
}

/// Package member names are flat ASCII: lowercase alphanumerics, `-`, `_`
/// and `.`, not starting with `.`. This rules out paths and traversal.
fn is_safe_member_name(name: &[u8]) -> bool {
    !name.is_empty()
        && name[0] != b'.'
        && name.iter().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'-' | b'_' | b'.')
        })
}

// ---------------------------------------------------------------------------
// Manifest fields.
// ---------------------------------------------------------------------------

struct OwnedManifestFields {
    package_id: String,
    export_profile_id: String,
    source_model_ref: PackageRef,
    source_basis_refs: Vec<PackageRef>,
    export_blocked: bool,
    blocking_reasons: Vec<String>,
}

struct MemberRow {
    role: String,
    file_name: String,
    media_type: String,
    hash_basis: String,
    sha256_hex: String,
    byte_length: u64,
    carries_canonical_identity: bool,
}

fn required<'a>(
    object: &'a Map<String, Value>,
    key: &str,
) -> Result<&'a Value, PackageReadRefusal> {
    object
        .get(key)
        .ok_or_else(|| malformed_manifest(format!("manifest field {key} is missing")))
}

fn required_str(object: &Map<String, Value>, key: &str) -> Result<String, PackageReadRefusal> {
    required(object, key)?
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| malformed_manifest(format!("manifest field {key} must be a string")))
}

fn package_ref(value: &Value, key: &str) -> Result<PackageRef, PackageReadRefusal> {
    let object = value
        .as_object()
        .filter(|object| object.len() == 2)
        .ok_or_else(|| {
            malformed_manifest(format!("{key} must be a {{ref_type, ref_id}} object"))
        })?;
    Ok(PackageRef::new(
        required_str(object, "ref_type")?,
        required_str(object, "ref_id")?,
    ))
}

fn manifest_fields(
    manifest: &Map<String, Value>,
) -> Result<OwnedManifestFields, PackageReadRefusal> {
    let source_basis_refs = required(manifest, "source_basis_refs")?
        .as_array()
        .ok_or_else(|| malformed_manifest("source_basis_refs must be an array"))?
        .iter()
        .map(|value| package_ref(value, "source_basis_refs entry"))
        .collect::<Result<Vec<_>, _>>()?;
    let diagnostics = required(manifest, "diagnostics")?
        .as_object()
        .ok_or_else(|| malformed_manifest("diagnostics must be an object"))?;
    let export_blocked = required(diagnostics, "export_blocked")?
        .as_bool()
        .ok_or_else(|| malformed_manifest("diagnostics.export_blocked must be a boolean"))?;
    let blocking_reasons = required(diagnostics, "blocking_reasons")?
        .as_array()
        .ok_or_else(|| malformed_manifest("diagnostics.blocking_reasons must be an array"))?
        .iter()
        .map(|value| {
            value.as_str().map(str::to_string).ok_or_else(|| {
                malformed_manifest("diagnostics.blocking_reasons entries must be strings")
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if export_blocked == blocking_reasons.is_empty() {
        return Err(malformed_manifest(
            "diagnostics.export_blocked disagrees with diagnostics.blocking_reasons",
        ));
    }
    Ok(OwnedManifestFields {
        package_id: required_str(manifest, "package_id")?,
        export_profile_id: required_str(manifest, "export_profile_id")?,
        source_model_ref: package_ref(required(manifest, "source_model_ref")?, "source_model_ref")?,
        source_basis_refs,
        export_blocked,
        blocking_reasons,
    })
}

fn manifest_member_rows(
    manifest: &Map<String, Value>,
) -> Result<Vec<MemberRow>, PackageReadRefusal> {
    required(manifest, "members")?
        .as_array()
        .ok_or_else(|| malformed_manifest("members must be an array"))?
        .iter()
        .map(|value| {
            let row = value
                .as_object()
                .ok_or_else(|| malformed_manifest("members entries must be objects"))?;
            let boolean = |key: &str| {
                required(row, key)?
                    .as_bool()
                    .ok_or_else(|| malformed_manifest(format!("member {key} must be a boolean")))
            };
            Ok(MemberRow {
                role: required_str(row, "role")?,
                file_name: required_str(row, "file_name")?,
                media_type: required_str(row, "media_type")?,
                hash_basis: required_str(row, "hash_basis")?,
                sha256_hex: required_str(row, "sha256_hex")?,
                byte_length: required(row, "byte_length")?.as_u64().ok_or_else(|| {
                    malformed_manifest("member byte_length must be a non-negative integer")
                })?,
                carries_canonical_identity: boolean("carries_canonical_identity")?,
            })
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Member verification.
// ---------------------------------------------------------------------------

/// The fixed form of each member role in the current version.
struct RoleForm {
    rank: u8,
    media_type: &'static str,
    hash_basis: &'static str,
    carries_canonical_identity: bool,
}

fn role_form(role: &str) -> Option<RoleForm> {
    let form = |rank, media_type, hash_basis, carries_canonical_identity| RoleForm {
        rank,
        media_type,
        hash_basis,
        carries_canonical_identity,
    };
    match role {
        ROLE_REPORT_HTML => Some(form(0, "text/html", HASH_BASIS_EXACT, false)),
        ROLE_REPORT_PDF => Some(form(1, "application/pdf", HASH_BASIS_EXACT, false)),
        ROLE_AUDIT_MANIFEST => Some(form(2, "application/json", HASH_BASIS_JCS, true)),
        ROLE_RESULT_EXPORT => Some(form(3, "application/json", HASH_BASIS_JCS, true)),
        ROLE_STATE_COMPARISON_HANDOFF => Some(form(4, "application/json", HASH_BASIS_JCS, true)),
        _ => None,
    }
}

fn file_name_fits_role(role: &str, file_name: &str) -> bool {
    let stem = |prefix: &str| {
        file_name
            .strip_prefix(prefix)
            .and_then(|rest| rest.strip_suffix(".json"))
            .is_some_and(|stem| {
                !stem.is_empty()
                    && stem.bytes().all(|byte| {
                        byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || matches!(byte, b'-' | b'_')
                    })
            })
    };
    match role {
        ROLE_REPORT_HTML => file_name == "calculation_report.html",
        ROLE_REPORT_PDF => file_name == "calculation_report.pdf",
        ROLE_AUDIT_MANIFEST => file_name == "audit_manifest.json",
        ROLE_RESULT_EXPORT => stem("result_export_"),
        ROLE_STATE_COMPARISON_HANDOFF => stem("state_comparison_handoff_sections_"),
        _ => false,
    }
}

fn verify_members(
    bytes: &[u8],
    archive: &Archive,
    rows: &[MemberRow],
) -> Result<Vec<ContainerMember>, PackageReadRefusal> {
    let content_entries = &archive.entries[1..];
    if rows.len() != content_entries.len() {
        return Err(malformed_manifest(format!(
            "manifest lists {} members but the archive holds {}",
            rows.len(),
            content_entries.len()
        )));
    }

    let manifest_entry = &archive.entries[0];
    let manifest_bytes = manifest_entry.data(bytes);
    let mut members = vec![ContainerMember {
        role: ROLE_PACKAGE_MANIFEST.to_string(),
        file_name: PACKAGE_MANIFEST_FILE_NAME.to_string(),
        media_type: "application/json".to_string(),
        hash_basis: HASH_BASIS_JCS.to_string(),
        sha256_hex: sha256_hex(manifest_bytes),
        byte_length: manifest_bytes.len(),
        carries_canonical_identity: true,
        bytes: manifest_bytes.to_vec(),
    }];

    let mut previous_rank: Option<u8> = None;
    for (row, entry) in rows.iter().zip(content_entries) {
        let defect = |reason: String| PackageReadRefusal::MemberIntegrity {
            file_name: row.file_name.clone(),
            reason,
        };
        if entry.name != row.file_name.as_bytes() {
            return Err(defect(format!(
                "manifest lists {} where the archive holds {}",
                row.file_name,
                entry.display_name()
            )));
        }
        let form = role_form(&row.role)
            .ok_or_else(|| defect(format!("unknown member role {}", row.role)))?;
        if !file_name_fits_role(&row.role, &row.file_name) {
            return Err(defect(format!("file name does not fit role {}", row.role)));
        }
        if row.media_type != form.media_type
            || row.hash_basis != form.hash_basis
            || row.carries_canonical_identity != form.carries_canonical_identity
        {
            return Err(defect(format!(
                "media type, hash basis or canonical identity does not fit role {}",
                row.role
            )));
        }
        // Fixed order: HTML, PDF, audit manifest (once each), then result
        // exports, then state/comparison/handoff sections.
        let in_order = match previous_rank {
            None => form.rank == 0,
            Some(previous) if previous < 2 => form.rank == previous + 1,
            Some(previous) => form.rank >= 3 && form.rank >= previous,
        };
        if !in_order {
            return Err(defect(format!(
                "role {} is out of the fixed member order",
                row.role
            )));
        }
        previous_rank = Some(form.rank);

        let data = entry.data(bytes);
        if row.byte_length != data.len() as u64 {
            return Err(defect(
                "byte length disagrees with the manifest".to_string(),
            ));
        }
        let digest = sha256_hex(data);
        if row.sha256_hex != digest {
            return Err(defect("SHA-256 disagrees with the manifest".to_string()));
        }
        match row.role.as_str() {
            ROLE_REPORT_HTML => {
                if std::str::from_utf8(data).is_err() {
                    return Err(defect("rendered report HTML is not UTF-8".to_string()));
                }
            }
            ROLE_REPORT_PDF => {
                if !data.starts_with(b"%PDF-") {
                    return Err(defect("rendered report PDF lacks a PDF header".to_string()));
                }
            }
            _ => {
                let document: Value = serde_json::from_slice(data)
                    .map_err(|error| defect(format!("member is not JSON: {error}")))?;
                if canonical_json(&document).as_bytes() != data {
                    return Err(defect("member bytes are not canonical JSON".to_string()));
                }
            }
        }

        members.push(ContainerMember {
            role: row.role.clone(),
            file_name: row.file_name.clone(),
            media_type: row.media_type.clone(),
            hash_basis: row.hash_basis.clone(),
            sha256_hex: digest,
            byte_length: data.len(),
            carries_canonical_identity: row.carries_canonical_identity,
            bytes: data.to_vec(),
        });
    }
    if previous_rank.is_none_or(|rank| rank < 2) {
        return Err(malformed_manifest(
            "manifest must list the rendered report HTML, PDF and audit manifest members",
        ));
    }
    Ok(members)
}

// ---------------------------------------------------------------------------
// Helpers.
// ---------------------------------------------------------------------------

fn differing_keys(expected: &Value, found: &Value) -> Vec<String> {
    let (Some(expected), Some(found)) = (expected.as_object(), found.as_object()) else {
        return vec!["<document>".to_string()];
    };
    let keys: BTreeSet<&String> = expected.keys().chain(found.keys()).collect();
    keys.into_iter()
        .filter(|key| expected.get(*key) != found.get(*key))
        .cloned()
        .collect()
}

fn not_a_package(reason: impl Into<String>) -> PackageReadRefusal {
    PackageReadRefusal::NotAPackage {
        reason: reason.into(),
    }
}

fn malformed_container(reason: impl Into<String>) -> PackageReadRefusal {
    PackageReadRefusal::MalformedContainer {
        reason: reason.into(),
    }
}

fn malformed_manifest(reason: impl Into<String>) -> PackageReadRefusal {
    PackageReadRefusal::MalformedManifest {
        reason: reason.into(),
    }
}
