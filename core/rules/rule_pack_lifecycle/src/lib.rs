//! Private rule-pack lifecycle and checksum handling.
//!
//! This crate records metadata and reproducibility evidence for user-owned rule
//! packs. It does not store private rule-pack payloads, parse or canonicalize
//! JSON, implement encryption or access control, expose private formulas, or
//! make professional/code-compliance claims.

use std::fmt;

/// JSON member key that binds the expression grammar version inside the
/// JCS-canonicalized rule-pack payload bytes hashed by `rule_pack_checksum`
/// (DEC-022). Because the member sits inside the hashed bytes, the reported
/// checksum (FR-016) changes whenever the declared grammar version changes.
pub const GRAMMAR_VERSION_PAYLOAD_KEY: &str = "grammar_version";

/// Exact JCS encoding of the grammar-version member
/// (`"grammar_version":"<version>"`): JCS emits sorted object keys with no
/// insignificant whitespace, so this byte sequence must appear verbatim in
/// any JCS payload that declares the version at an object boundary.
pub fn grammar_version_jcs_member(grammar_version: &str) -> String {
    format!(
        "\"{GRAMMAR_VERSION_PAYLOAD_KEY}\":\"{}\"",
        grammar_version.trim()
    )
}

/// Byte-containment evidence check: does `canonical_payload` contain the
/// exact JCS member encoding for `grammar_version`?
///
/// This crate does not parse JSON, so this is a necessary-condition check on
/// caller-supplied bytes (the member could in principle appear in a nested
/// string), not proof of JCS canonical form. Absence is conclusive: a payload
/// without these bytes cannot bind the declared grammar version.
pub fn payload_declares_grammar_version(canonical_payload: &[u8], grammar_version: &str) -> bool {
    let member = grammar_version_jcs_member(grammar_version);
    let needle = member.as_bytes();
    if needle.is_empty() || canonical_payload.len() < needle.len() {
        return false;
    }
    canonical_payload
        .windows(needle.len())
        .any(|window| window == needle)
}

/// Strict `MAJOR.MINOR.PATCH` check: three dot-separated decimal components,
/// no signs, no leading zeros, no pre-release/build suffixes. Mirrors the
/// evaluator-side rule in `core/rules/expression_evaluator`.
pub fn is_semver_grammar_version(version: &str) -> bool {
    let component_ok = |part: &str| {
        !part.is_empty()
            && part.len() <= 9
            && part.bytes().all(|byte| byte.is_ascii_digit())
            && (part == "0" || !part.starts_with('0'))
    };
    let parts: Vec<&str> = version.split('.').collect();
    parts.len() == 3 && parts.iter().all(|part| component_ok(part))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivacyClass {
    PublicSchemaOnly,
    PublicInventedExample,
    PrivateUserData,
    PrivateProjectData,
    Quarantined,
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedistributionStatus {
    PublicPermissive,
    PrivateOnly,
    Unknown,
    ProtectedSuspected,
    InventedNonEngineeringExample,
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewStatus {
    Pending,
    Accepted,
    Rejected,
    Quarantined,
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleStatus {
    Draft,
    Active,
    Superseded,
    Retired,
    Quarantined,
    AcceptedPublicExample,
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumAlgorithm {
    Sha256,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Canonicalization {
    CallerSuppliedJcsBytesUnverified,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayloadScope {
    RulePackPayload,
    RulePackReference,
    InputManifest,
    ReportManifest,
    ExternalArtifact,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecksumRecord {
    pub algorithm: ChecksumAlgorithm,
    pub canonicalization: Canonicalization,
    pub payload_scope: PayloadScope,
    pub payload_ref: String,
    pub value: String,
}

impl ChecksumRecord {
    /// Hash caller-supplied bytes that the caller represents as JCS-compatible.
    ///
    /// This crate does not parse or canonicalize JSON, so the canonicalization
    /// marker is intentionally unverified metadata rather than proof of JCS.
    pub fn sha256_caller_supplied_jcs_bytes(
        payload_scope: PayloadScope,
        payload_ref: impl Into<String>,
        canonical_payload: &[u8],
    ) -> Self {
        Self {
            algorithm: ChecksumAlgorithm::Sha256,
            canonicalization: Canonicalization::CallerSuppliedJcsBytesUnverified,
            payload_scope,
            payload_ref: payload_ref.into(),
            value: sha256_hex(canonical_payload),
        }
    }

    /// Hash caller-supplied JCS-compatible bytes while asserting that the
    /// declared expression grammar version is bound inside the hashed payload
    /// (DEC-022): the exact JCS member encoding must appear in the bytes and
    /// the version must be a strict semantic version. This is the recommended
    /// constructor for `rule_pack_payload`-scope checksums.
    pub fn sha256_caller_supplied_jcs_bytes_with_grammar_version(
        payload_scope: PayloadScope,
        payload_ref: impl Into<String>,
        canonical_payload: &[u8],
        grammar_version: &str,
    ) -> Result<Self, GrammarVersionBindingError> {
        let grammar_version = grammar_version.trim();
        if !is_semver_grammar_version(grammar_version) {
            return Err(GrammarVersionBindingError::MalformedGrammarVersion {
                declared: grammar_version.to_string(),
            });
        }
        if !payload_declares_grammar_version(canonical_payload, grammar_version) {
            return Err(GrammarVersionBindingError::GrammarVersionNotInPayload {
                declared: grammar_version.to_string(),
            });
        }
        Ok(Self::sha256_caller_supplied_jcs_bytes(
            payload_scope,
            payload_ref,
            canonical_payload,
        ))
    }

    pub fn is_empty(&self) -> bool {
        self.value.trim().is_empty() || self.value.trim().eq_ignore_ascii_case("TBD")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrammarVersionBindingError {
    MalformedGrammarVersion { declared: String },
    GrammarVersionNotInPayload { declared: String },
}

impl fmt::Display for GrammarVersionBindingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MalformedGrammarVersion { declared } => write!(
                f,
                "declared grammar version '{declared}' is not a MAJOR.MINOR.PATCH semantic version"
            ),
            Self::GrammarVersionNotInPayload { declared } => write!(
                f,
                "canonical payload bytes do not contain the JCS member binding grammar version '{declared}'"
            ),
        }
    }
}

impl std::error::Error for GrammarVersionBindingError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RulePackLifecycleRecord {
    pub rule_pack_id: String,
    pub name: String,
    pub schema_version: String,
    pub rule_pack_version: String,
    /// Expression grammar version (semver) declared by the pack and bound
    /// inside the JCS-hashed checksum payload per DEC-022.
    pub grammar_version: String,
    pub lifecycle_status: LifecycleStatus,
    pub source_notice: String,
    pub privacy_class: PrivacyClass,
    pub redistribution_status: RedistributionStatus,
    pub review_status: ReviewStatus,
    pub protected_content_review_required: bool,
    pub protected_content_suspected: bool,
    pub checksum: Option<ChecksumRecord>,
    pub professional_boundary: ProfessionalBoundary,
}

impl RulePackLifecycleRecord {
    pub fn is_publicly_exportable(&self) -> bool {
        matches!(
            self.privacy_class,
            PrivacyClass::PublicSchemaOnly | PrivacyClass::PublicInventedExample
        ) && matches!(
            self.redistribution_status,
            RedistributionStatus::PublicPermissive
                | RedistributionStatus::InventedNonEngineeringExample
        ) && self.review_status == ReviewStatus::Accepted
            && !self.protected_content_suspected
    }

    pub fn audit_manifest_entry(&self) -> AuditManifestEntry {
        AuditManifestEntry {
            rule_pack_id: self.rule_pack_id.clone(),
            rule_pack_version: self.rule_pack_version.clone(),
            grammar_version: self.grammar_version.clone(),
            source_notice: self.source_notice.clone(),
            privacy_class: self.privacy_class,
            redistribution_status: self.redistribution_status,
            review_status: self.review_status,
            checksum: self.checksum.clone(),
            private_payload_redacted: matches!(
                self.privacy_class,
                PrivacyClass::PrivateUserData | PrivacyClass::PrivateProjectData
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfessionalBoundary {
    pub software_makes_compliance_claim: bool,
    pub software_makes_certification_claim: bool,
    pub software_makes_sealing_claim: bool,
    pub human_review_required: bool,
    pub human_acceptance_record_software_generated: bool,
}

impl ProfessionalBoundary {
    pub fn project_default() -> Self {
        Self {
            software_makes_compliance_claim: false,
            software_makes_certification_claim: false,
            software_makes_sealing_claim: false,
            human_review_required: true,
            human_acceptance_record_software_generated: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleFindingCode {
    MissingRulePackIdentity,
    MissingVersion,
    /// The pack declares no expression grammar version (DEC-022 requires one
    /// bound inside the checksum payload).
    MissingGrammarVersion,
    /// The declared expression grammar version is not a strict
    /// MAJOR.MINOR.PATCH semantic version.
    GrammarVersionMalformed,
    /// Canonical payload bytes were supplied for a rule-pack-payload-scope
    /// checksum but do not contain the JCS member binding the declared
    /// grammar version, so the checksum does not bind it.
    GrammarVersionNotBound,
    MissingSourceNotice,
    MissingRedistributionStatus,
    MissingReviewStatus,
    MissingChecksum,
    StaleChecksum,
    ProtectedContentSuspected,
    PrivateExportBlocked,
    ProfessionalBoundaryViolation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleFinding {
    pub code: LifecycleFindingCode,
    pub subject_id: String,
    pub message: String,
}

impl LifecycleFinding {
    fn new(
        code: LifecycleFindingCode,
        subject_id: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            subject_id: subject_id.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleValidationInput<'a> {
    pub record: &'a RulePackLifecycleRecord,
    pub expected_checksum: Option<&'a ChecksumRecord>,
    /// Canonical JCS payload bytes for the record's checksum, when the caller
    /// can supply them. When present and the checksum scope is
    /// `RulePackPayload`, validation checks that the declared grammar version
    /// is bound inside these bytes (DEC-022).
    pub canonical_payload: Option<&'a [u8]>,
    pub public_export_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleValidationResult {
    pub findings: Vec<LifecycleFinding>,
    pub audit_manifest_entry: AuditManifestEntry,
}

impl LifecycleValidationResult {
    pub fn is_blocked(&self) -> bool {
        self.findings.iter().any(|finding| {
            matches!(
                finding.code,
                LifecycleFindingCode::MissingRulePackIdentity
                    | LifecycleFindingCode::MissingVersion
                    | LifecycleFindingCode::MissingGrammarVersion
                    | LifecycleFindingCode::GrammarVersionMalformed
                    | LifecycleFindingCode::GrammarVersionNotBound
                    | LifecycleFindingCode::MissingChecksum
                    | LifecycleFindingCode::StaleChecksum
                    | LifecycleFindingCode::ProtectedContentSuspected
                    | LifecycleFindingCode::PrivateExportBlocked
                    | LifecycleFindingCode::ProfessionalBoundaryViolation
            )
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditManifestEntry {
    pub rule_pack_id: String,
    pub rule_pack_version: String,
    pub grammar_version: String,
    pub source_notice: String,
    pub privacy_class: PrivacyClass,
    pub redistribution_status: RedistributionStatus,
    pub review_status: ReviewStatus,
    pub checksum: Option<ChecksumRecord>,
    pub private_payload_redacted: bool,
}

pub fn validate_lifecycle(input: &LifecycleValidationInput<'_>) -> LifecycleValidationResult {
    let record = input.record;
    let mut findings = Vec::new();

    if record.rule_pack_id.trim().is_empty() {
        findings.push(LifecycleFinding::new(
            LifecycleFindingCode::MissingRulePackIdentity,
            "rule_pack_id",
            "rule-pack identity must be stable and non-empty",
        ));
    }

    if record.schema_version.trim().is_empty() || record.rule_pack_version.trim().is_empty() {
        findings.push(LifecycleFinding::new(
            LifecycleFindingCode::MissingVersion,
            &record.rule_pack_id,
            "schema version and rule-pack version must both be recorded",
        ));
    }

    let grammar_version = record.grammar_version.trim();
    if grammar_version.is_empty() {
        findings.push(LifecycleFinding::new(
            LifecycleFindingCode::MissingGrammarVersion,
            &record.rule_pack_id,
            "expression grammar version must be recorded and bound inside the checksum payload",
        ));
    } else if !is_semver_grammar_version(grammar_version) {
        findings.push(LifecycleFinding::new(
            LifecycleFindingCode::GrammarVersionMalformed,
            &record.rule_pack_id,
            "expression grammar version must be a MAJOR.MINOR.PATCH semantic version",
        ));
    } else if let Some(canonical_payload) = input.canonical_payload {
        let checksum_scope_is_pack_payload = record
            .checksum
            .as_ref()
            .is_some_and(|checksum| checksum.payload_scope == PayloadScope::RulePackPayload);
        if checksum_scope_is_pack_payload
            && !payload_declares_grammar_version(canonical_payload, grammar_version)
        {
            findings.push(LifecycleFinding::new(
                LifecycleFindingCode::GrammarVersionNotBound,
                &record.rule_pack_id,
                "canonical payload bytes do not contain the declared grammar_version member, \
                 so the rule-pack checksum does not bind the grammar version",
            ));
        }
    }

    if record.source_notice.trim().is_empty() {
        findings.push(LifecycleFinding::new(
            LifecycleFindingCode::MissingSourceNotice,
            &record.rule_pack_id,
            "source/provenance notice is required for lifecycle and report references",
        ));
    }

    if matches!(
        record.redistribution_status,
        RedistributionStatus::Unknown | RedistributionStatus::Tbd
    ) {
        findings.push(LifecycleFinding::new(
            LifecycleFindingCode::MissingRedistributionStatus,
            &record.rule_pack_id,
            "redistribution status must be explicit before public handling",
        ));
    }

    if matches!(
        record.review_status,
        ReviewStatus::Pending | ReviewStatus::Tbd
    ) {
        findings.push(LifecycleFinding::new(
            LifecycleFindingCode::MissingReviewStatus,
            &record.rule_pack_id,
            "review status remains pending or TBD",
        ));
    }

    if record.protected_content_suspected
        || record.redistribution_status == RedistributionStatus::ProtectedSuspected
        || record.review_status == ReviewStatus::Quarantined
    {
        findings.push(LifecycleFinding::new(
            LifecycleFindingCode::ProtectedContentSuspected,
            &record.rule_pack_id,
            "suspected protected or proprietary content requires quarantine and human review",
        ));
    }

    match &record.checksum {
        Some(checksum) if !checksum.is_empty() => {
            if let Some(expected) = input.expected_checksum {
                if checksum != expected {
                    findings.push(LifecycleFinding::new(
                        LifecycleFindingCode::StaleChecksum,
                        &record.rule_pack_id,
                        "recorded checksum does not match expected payload checksum",
                    ));
                }
            }
        }
        _ => findings.push(LifecycleFinding::new(
            LifecycleFindingCode::MissingChecksum,
            &record.rule_pack_id,
            "checksum metadata is required before lifecycle evidence is complete",
        )),
    }

    if input.public_export_requested && !record.is_publicly_exportable() {
        findings.push(LifecycleFinding::new(
            LifecycleFindingCode::PrivateExportBlocked,
            &record.rule_pack_id,
            "public export is blocked unless public redistribution, review, and privacy status are all acceptable",
        ));
    }

    if violates_professional_boundary(record.professional_boundary) {
        findings.push(LifecycleFinding::new(
            LifecycleFindingCode::ProfessionalBoundaryViolation,
            &record.rule_pack_id,
            "rule-pack lifecycle records must not claim software compliance, certification, sealing, or generated human acceptance",
        ));
    }

    LifecycleValidationResult {
        findings,
        audit_manifest_entry: record.audit_manifest_entry(),
    }
}

fn violates_professional_boundary(boundary: ProfessionalBoundary) -> bool {
    boundary.software_makes_compliance_claim
        || boundary.software_makes_certification_claim
        || boundary.software_makes_sealing_claim
        || boundary.human_acceptance_record_software_generated
        || !boundary.human_review_required
}

pub fn sha256_hex(payload: &[u8]) -> String {
    const H0: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    let mut data = payload.to_vec();
    let bit_len = (data.len() as u64) * 8;
    data.push(0x80);
    while data.len() % 64 != 56 {
        data.push(0);
    }
    data.extend_from_slice(&bit_len.to_be_bytes());

    let mut h = H0;
    for chunk in data.chunks_exact(64) {
        let mut w = [0u32; 64];
        for (i, word) in w.iter_mut().take(16).enumerate() {
            let j = i * 4;
            *word = u32::from_be_bytes([chunk[j], chunk[j + 1], chunk[j + 2], chunk[j + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut hh = h[7];

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    let mut output = String::with_capacity(64);
    for word in h {
        fmt::Write::write_fmt(&mut output, format_args!("{word:08x}"))
            .expect("writing to String cannot fail");
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUBLIC_PAYLOAD: &[u8] =
        br#"{"grammar_version":"1.0.0","rule_pack_id":"invented_demo","rule_pack_version":"0.1.0"}"#;

    fn public_record() -> RulePackLifecycleRecord {
        let payload = PUBLIC_PAYLOAD;
        RulePackLifecycleRecord {
            rule_pack_id: "invented_demo".to_string(),
            name: "Invented Demo".to_string(),
            schema_version: "0.1.0".to_string(),
            rule_pack_version: "0.1.0".to_string(),
            grammar_version: "1.0.0".to_string(),
            lifecycle_status: LifecycleStatus::AcceptedPublicExample,
            source_notice: "Original invented demonstration content only.".to_string(),
            privacy_class: PrivacyClass::PublicInventedExample,
            redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
            review_status: ReviewStatus::Accepted,
            protected_content_review_required: true,
            protected_content_suspected: false,
            checksum: Some(ChecksumRecord::sha256_caller_supplied_jcs_bytes(
                PayloadScope::RulePackPayload,
                "invented_demo",
                payload,
            )),
            professional_boundary: ProfessionalBoundary::project_default(),
        }
    }

    #[test]
    fn sha256_matches_known_vectors() {
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn checksum_records_unverified_caller_supplied_jcs_payload_basis() {
        let checksum = ChecksumRecord::sha256_caller_supplied_jcs_bytes(
            PayloadScope::RulePackPayload,
            "payload:v1",
            br#"{"a":1}"#,
        );
        assert_eq!(checksum.algorithm, ChecksumAlgorithm::Sha256);
        assert_eq!(
            checksum.canonicalization,
            Canonicalization::CallerSuppliedJcsBytesUnverified
        );
        assert_eq!(checksum.payload_ref, "payload:v1");
        assert_eq!(checksum.value.len(), 64);
    }

    #[test]
    fn valid_public_invented_record_has_no_findings() {
        let record = public_record();
        let result = validate_lifecycle(&LifecycleValidationInput {
            expected_checksum: record.checksum.as_ref(),
            canonical_payload: Some(PUBLIC_PAYLOAD),
            public_export_requested: true,
            record: &record,
        });
        assert!(!result.is_blocked());
        assert!(result.findings.is_empty());
        assert!(!result.audit_manifest_entry.private_payload_redacted);
    }

    #[test]
    fn private_rule_pack_blocks_public_export_and_redacts_payload() {
        let mut record = public_record();
        record.privacy_class = PrivacyClass::PrivateUserData;
        record.redistribution_status = RedistributionStatus::PrivateOnly;
        record.review_status = ReviewStatus::Pending;

        let result = validate_lifecycle(&LifecycleValidationInput {
            expected_checksum: record.checksum.as_ref(),
            canonical_payload: Some(PUBLIC_PAYLOAD),
            public_export_requested: true,
            record: &record,
        });

        assert!(result.is_blocked());
        assert!(result
            .findings
            .iter()
            .any(|finding| { finding.code == LifecycleFindingCode::PrivateExportBlocked }));
        assert!(result.audit_manifest_entry.private_payload_redacted);
        assert!(result.audit_manifest_entry.checksum.is_some());
    }

    #[test]
    fn missing_metadata_and_checksum_are_findings() {
        let mut record = public_record();
        record.rule_pack_id.clear();
        record.schema_version.clear();
        record.source_notice.clear();
        record.redistribution_status = RedistributionStatus::Unknown;
        record.checksum = None;

        let result = validate_lifecycle(&LifecycleValidationInput {
            expected_checksum: None,
            canonical_payload: None,
            public_export_requested: false,
            record: &record,
        });
        let codes: Vec<_> = result.findings.iter().map(|finding| finding.code).collect();
        assert!(codes.contains(&LifecycleFindingCode::MissingRulePackIdentity));
        assert!(codes.contains(&LifecycleFindingCode::MissingVersion));
        assert!(codes.contains(&LifecycleFindingCode::MissingSourceNotice));
        assert!(codes.contains(&LifecycleFindingCode::MissingRedistributionStatus));
        assert!(codes.contains(&LifecycleFindingCode::MissingChecksum));
    }

    #[test]
    fn suspected_protected_content_blocks_lifecycle() {
        let mut record = public_record();
        record.protected_content_suspected = true;
        record.review_status = ReviewStatus::Quarantined;

        let result = validate_lifecycle(&LifecycleValidationInput {
            expected_checksum: record.checksum.as_ref(),
            canonical_payload: Some(PUBLIC_PAYLOAD),
            public_export_requested: false,
            record: &record,
        });

        assert!(result.is_blocked());
        assert!(result
            .findings
            .iter()
            .any(|finding| { finding.code == LifecycleFindingCode::ProtectedContentSuspected }));
    }

    #[test]
    fn stale_checksum_is_detected() {
        let record = public_record();
        let expected = ChecksumRecord::sha256_caller_supplied_jcs_bytes(
            PayloadScope::RulePackPayload,
            "invented_demo",
            br#"{"rule_pack_id":"invented_demo","rule_pack_version":"0.2.0"}"#,
        );

        let result = validate_lifecycle(&LifecycleValidationInput {
            expected_checksum: Some(&expected),
            canonical_payload: Some(PUBLIC_PAYLOAD),
            public_export_requested: false,
            record: &record,
        });

        assert!(result.is_blocked());
        assert!(result
            .findings
            .iter()
            .any(|finding| { finding.code == LifecycleFindingCode::StaleChecksum }));
    }

    #[test]
    fn professional_boundary_claims_are_rejected() {
        let mut record = public_record();
        record.professional_boundary.software_makes_compliance_claim = true;

        let result = validate_lifecycle(&LifecycleValidationInput {
            expected_checksum: record.checksum.as_ref(),
            canonical_payload: Some(PUBLIC_PAYLOAD),
            public_export_requested: false,
            record: &record,
        });

        assert!(result.is_blocked());
        assert!(result.findings.iter().any(|finding| {
            finding.code == LifecycleFindingCode::ProfessionalBoundaryViolation
        }));
    }

    #[test]
    fn missing_or_malformed_grammar_version_blocks() {
        let mut record = public_record();
        record.grammar_version.clear();
        let result = validate_lifecycle(&LifecycleValidationInput {
            expected_checksum: record.checksum.as_ref(),
            canonical_payload: None,
            public_export_requested: false,
            record: &record,
        });
        assert!(result.is_blocked());
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == LifecycleFindingCode::MissingGrammarVersion));

        let mut record = public_record();
        record.grammar_version = "1.0".to_string();
        let result = validate_lifecycle(&LifecycleValidationInput {
            expected_checksum: record.checksum.as_ref(),
            canonical_payload: None,
            public_export_requested: false,
            record: &record,
        });
        assert!(result.is_blocked());
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == LifecycleFindingCode::GrammarVersionMalformed));
    }

    #[test]
    fn unbound_grammar_version_in_payload_blocks() {
        let payload = br#"{"rule_pack_id":"invented_demo","rule_pack_version":"0.1.0"}"#;
        let mut record = public_record();
        record.checksum = Some(ChecksumRecord::sha256_caller_supplied_jcs_bytes(
            PayloadScope::RulePackPayload,
            "invented_demo",
            payload,
        ));

        let result = validate_lifecycle(&LifecycleValidationInput {
            expected_checksum: record.checksum.as_ref(),
            canonical_payload: Some(payload),
            public_export_requested: false,
            record: &record,
        });

        assert!(result.is_blocked());
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == LifecycleFindingCode::GrammarVersionNotBound));
    }

    #[test]
    fn grammar_version_binding_constructor_enforces_payload_membership() {
        let bound = ChecksumRecord::sha256_caller_supplied_jcs_bytes_with_grammar_version(
            PayloadScope::RulePackPayload,
            "invented_demo",
            PUBLIC_PAYLOAD,
            "1.0.0",
        );
        assert!(bound.is_ok());

        let unbound = ChecksumRecord::sha256_caller_supplied_jcs_bytes_with_grammar_version(
            PayloadScope::RulePackPayload,
            "invented_demo",
            PUBLIC_PAYLOAD,
            "1.1.0",
        );
        assert_eq!(
            unbound,
            Err(GrammarVersionBindingError::GrammarVersionNotInPayload {
                declared: "1.1.0".to_string()
            })
        );

        let malformed = ChecksumRecord::sha256_caller_supplied_jcs_bytes_with_grammar_version(
            PayloadScope::RulePackPayload,
            "invented_demo",
            PUBLIC_PAYLOAD,
            "1.0",
        );
        assert_eq!(
            malformed,
            Err(GrammarVersionBindingError::MalformedGrammarVersion {
                declared: "1.0".to_string()
            })
        );
    }

    /// Golden checksum-binding corpus: the fixture payloads under
    /// `fixtures/rule_expressions/checksum_binding/` pin (a) byte stability of
    /// the invented payloads, (b) this crate's SHA-256 over them, and (c) the
    /// DEC-022 property that changing or removing the `grammar_version`
    /// member changes the rule-pack checksum.
    #[test]
    fn grammar_version_binding_fixture_corpus_is_stable() {
        let corpus_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../fixtures/rule_expressions/checksum_binding");
        let manifest = std::fs::read_to_string(corpus_dir.join("MANIFEST.tsv"))
            .expect("checksum binding manifest must exist");

        let mut hashes = Vec::new();
        let mut rows = 0;
        for line in manifest.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            rows += 1;
            let mut fields = line.split('\t');
            let file_name = fields.next().expect("manifest row must name a file");
            let expected_sha256 = fields.next().expect("manifest row must carry a hash");
            let bound_version = fields.next().expect("manifest row must carry a version");
            assert!(fields.next().is_none(), "manifest row has extra fields");

            let payload =
                std::fs::read(corpus_dir.join(file_name)).expect("payload fixture must exist");
            let actual = sha256_hex(&payload);
            assert_eq!(
                actual, expected_sha256,
                "golden checksum drift for {file_name}"
            );
            hashes.push(actual);

            match bound_version {
                "none" => {
                    assert!(!payload_declares_grammar_version(&payload, "1.0.0"));
                    assert!(!payload_declares_grammar_version(&payload, "1.1.0"));
                }
                version => {
                    assert!(payload_declares_grammar_version(&payload, version));
                    assert!(
                        ChecksumRecord::sha256_caller_supplied_jcs_bytes_with_grammar_version(
                            PayloadScope::RulePackPayload,
                            file_name,
                            &payload,
                            version,
                        )
                        .is_ok()
                    );
                }
            }
        }

        assert_eq!(
            rows, 3,
            "checksum binding corpus must keep all three payloads"
        );
        let mut deduped = hashes.clone();
        deduped.sort();
        deduped.dedup();
        assert_eq!(
            deduped.len(),
            hashes.len(),
            "grammar version member must be checksum-binding (all payload hashes distinct)"
        );
    }
}
