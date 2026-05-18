//! Audit manifest and model hash support.
//!
//! This crate creates deterministic audit metadata for OpenPipeStress model/run
//! evidence. It hashes project-local deterministic JSON values and
//! caller-supplied asset bytes, but it does not parse project files, implement
//! or validate full JCS canonicalization, store private payloads, import
//! protected standards content, or emit professional/code-compliance claims.

use std::collections::{BTreeMap, HashSet};

/// Project-local deterministic JSON value used for hash inputs.
///
/// Object keys are ordered by `BTreeMap` serialization. Number strings are
/// caller-supplied and are not normalized under RFC 8785/JCS rules.
#[derive(Debug, Clone, PartialEq)]
pub enum CanonicalJson {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<CanonicalJson>),
    Object(BTreeMap<String, CanonicalJson>),
}

impl CanonicalJson {
    pub fn object(entries: impl IntoIterator<Item = (impl Into<String>, CanonicalJson)>) -> Self {
        Self::Object(
            entries
                .into_iter()
                .map(|(key, value)| (key.into(), value))
                .collect(),
        )
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        self.canonical_string().into_bytes()
    }

    pub fn canonical_string(&self) -> String {
        match self {
            Self::Null => "null".to_string(),
            Self::Bool(value) => value.to_string(),
            Self::Number(value) => value.clone(),
            Self::String(value) => quote_json_string(value),
            Self::Array(values) => {
                let items: Vec<_> = values.iter().map(Self::canonical_string).collect();
                format!("[{}]", items.join(","))
            }
            Self::Object(entries) => {
                let items: Vec<_> = entries
                    .iter()
                    .map(|(key, value)| {
                        format!("{}:{}", quote_json_string(key), value.canonical_string())
                    })
                    .collect();
                format!("{{{}}}", items.join(","))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    Sha256,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Canonicalization {
    ProjectLocalDeterministicJson,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayloadKind {
    ModelJson,
    InputManifestJson,
    RulePackReference,
    BinaryAsset,
    ExternalArtifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivacyClass {
    PublicMetadata,
    PrivateProjectData,
    PrivateRulePackData,
    Redacted,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashRecord {
    pub algorithm: HashAlgorithm,
    pub canonicalization: Canonicalization,
    pub payload_kind: PayloadKind,
    pub payload_ref: String,
    pub value: String,
}

impl HashRecord {
    pub fn sha256_project_local_json(
        payload_kind: PayloadKind,
        payload_ref: impl Into<String>,
        payload: &[u8],
    ) -> Self {
        Self {
            algorithm: HashAlgorithm::Sha256,
            canonicalization: Canonicalization::ProjectLocalDeterministicJson,
            payload_kind,
            payload_ref: payload_ref.into(),
            value: sha256_hex(payload),
        }
    }

    pub fn sha256_none(
        payload_kind: PayloadKind,
        payload_ref: impl Into<String>,
        payload: &[u8],
    ) -> Self {
        Self {
            algorithm: HashAlgorithm::Sha256,
            canonicalization: Canonicalization::None,
            payload_kind,
            payload_ref: payload_ref.into(),
            value: sha256_hex(payload),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.value.trim().is_empty() || self.value.trim().eq_ignore_ascii_case("TBD")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SolverVersionStamp {
    pub solver_name: String,
    pub solver_version: String,
    pub solver_build_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RulePackAuditRef {
    pub rule_pack_id: String,
    pub rule_pack_version: String,
    pub source_notice: String,
    pub redistribution_status: RedistributionStatus,
    pub checksum: Option<HashRecord>,
    pub private_payload_redacted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetManifestEntry {
    pub asset_id: String,
    pub payload_kind: PayloadKind,
    pub privacy_class: PrivacyClass,
    pub hash: Option<HashRecord>,
    pub provenance: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfessionalBoundary {
    pub software_makes_compliance_claim: bool,
    pub software_makes_certification_claim: bool,
    pub software_makes_sealing_claim: bool,
    pub software_makes_approval_claim: bool,
    pub human_review_required: bool,
}

impl ProfessionalBoundary {
    pub fn project_default() -> Self {
        Self {
            software_makes_compliance_claim: false,
            software_makes_certification_claim: false,
            software_makes_sealing_claim: false,
            software_makes_approval_claim: false,
            human_review_required: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestFindingCode {
    MissingModelHash,
    MissingSolverVersion,
    MissingUnitSystem,
    MissingRulePackChecksum,
    MissingRulePackSourceNotice,
    MissingAssetHash,
    MissingAssetProvenance,
    ProtectedContentSuspected,
    DuplicateReference,
    ProfessionalBoundaryViolation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestFindingSeverity {
    Info,
    Warning,
    Blocking,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestFinding {
    pub code: ManifestFindingCode,
    pub severity: ManifestFindingSeverity,
    pub subject_id: String,
    pub message: String,
}

impl ManifestFinding {
    fn new(
        code: ManifestFindingCode,
        severity: ManifestFindingSeverity,
        subject_id: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            severity,
            subject_id: subject_id.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditManifest {
    pub manifest_id: String,
    pub model_hash: Option<HashRecord>,
    pub input_manifest_hash: Option<HashRecord>,
    pub solver_version: SolverVersionStamp,
    pub unit_system_ref: String,
    pub rule_pack_refs: Vec<RulePackAuditRef>,
    pub assets: Vec<AssetManifestEntry>,
    pub professional_boundary: ProfessionalBoundary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestValidationResult {
    pub findings: Vec<ManifestFinding>,
}

impl ManifestValidationResult {
    pub fn has_blocking_findings(&self) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.severity == ManifestFindingSeverity::Blocking)
    }
}

/// Hashes the project-local deterministic JSON representation.
///
/// This records `ProjectLocalDeterministicJson` metadata and is not a JCS
/// conformance claim.
pub fn hash_canonical_json(
    payload_kind: PayloadKind,
    payload_ref: impl Into<String>,
    value: &CanonicalJson,
) -> HashRecord {
    HashRecord::sha256_project_local_json(payload_kind, payload_ref, &value.canonical_bytes())
}

pub fn hash_asset(payload_ref: impl Into<String>, bytes: &[u8]) -> HashRecord {
    HashRecord::sha256_none(PayloadKind::BinaryAsset, payload_ref, bytes)
}

pub fn validate_manifest(manifest: &AuditManifest) -> ManifestValidationResult {
    let mut findings = Vec::new();

    match &manifest.model_hash {
        Some(hash) if !hash.is_empty() => {}
        _ => findings.push(ManifestFinding::new(
            ManifestFindingCode::MissingModelHash,
            ManifestFindingSeverity::Blocking,
            &manifest.manifest_id,
            "audit manifest must identify the canonical model payload hash",
        )),
    }

    if manifest.solver_version.solver_name.trim().is_empty()
        || manifest.solver_version.solver_version.trim().is_empty()
    {
        findings.push(ManifestFinding::new(
            ManifestFindingCode::MissingSolverVersion,
            ManifestFindingSeverity::Blocking,
            &manifest.manifest_id,
            "solver name and version must be recorded for reproducibility",
        ));
    }

    if manifest.unit_system_ref.trim().is_empty() {
        findings.push(ManifestFinding::new(
            ManifestFindingCode::MissingUnitSystem,
            ManifestFindingSeverity::Blocking,
            &manifest.manifest_id,
            "unit-system reference must be recorded",
        ));
    }

    check_duplicate_refs(manifest, &mut findings);
    check_rule_pack_refs(&manifest.rule_pack_refs, &mut findings);
    check_assets(&manifest.assets, &mut findings);
    check_professional_boundary(manifest, &mut findings);

    ManifestValidationResult { findings }
}

fn check_duplicate_refs(manifest: &AuditManifest, findings: &mut Vec<ManifestFinding>) {
    let mut refs = HashSet::new();

    if let Some(hash) = &manifest.model_hash {
        if !refs.insert(hash.payload_ref.as_str()) {
            findings.push(ManifestFinding::new(
                ManifestFindingCode::DuplicateReference,
                ManifestFindingSeverity::Warning,
                &hash.payload_ref,
                "manifest hash references should be unique",
            ));
        }
    }

    for rule_pack in &manifest.rule_pack_refs {
        if !refs.insert(rule_pack.rule_pack_id.as_str()) {
            findings.push(ManifestFinding::new(
                ManifestFindingCode::DuplicateReference,
                ManifestFindingSeverity::Warning,
                &rule_pack.rule_pack_id,
                "rule-pack references should be unique",
            ));
        }
    }

    for asset in &manifest.assets {
        if !refs.insert(asset.asset_id.as_str()) {
            findings.push(ManifestFinding::new(
                ManifestFindingCode::DuplicateReference,
                ManifestFindingSeverity::Warning,
                &asset.asset_id,
                "asset references should be unique",
            ));
        }
    }
}

fn check_rule_pack_refs(rule_pack_refs: &[RulePackAuditRef], findings: &mut Vec<ManifestFinding>) {
    for rule_pack in rule_pack_refs {
        match &rule_pack.checksum {
            Some(checksum) if !checksum.is_empty() => {}
            _ => findings.push(ManifestFinding::new(
                ManifestFindingCode::MissingRulePackChecksum,
                ManifestFindingSeverity::Blocking,
                &rule_pack.rule_pack_id,
                "rule-pack checksum must be captured when a rule pack participates",
            )),
        }

        if rule_pack.source_notice.trim().is_empty() {
            findings.push(ManifestFinding::new(
                ManifestFindingCode::MissingRulePackSourceNotice,
                ManifestFindingSeverity::Warning,
                &rule_pack.rule_pack_id,
                "rule-pack source notice should be recorded",
            ));
        }

        if rule_pack.redistribution_status == RedistributionStatus::ProtectedSuspected {
            findings.push(ManifestFinding::new(
                ManifestFindingCode::ProtectedContentSuspected,
                ManifestFindingSeverity::Blocking,
                &rule_pack.rule_pack_id,
                "rule-pack reference is marked as suspected protected content",
            ));
        }
    }
}

fn check_assets(assets: &[AssetManifestEntry], findings: &mut Vec<ManifestFinding>) {
    for asset in assets {
        match &asset.hash {
            Some(hash) if !hash.is_empty() => {}
            _ => findings.push(ManifestFinding::new(
                ManifestFindingCode::MissingAssetHash,
                ManifestFindingSeverity::Blocking,
                &asset.asset_id,
                "non-JSON asset manifest entry must include its own hash",
            )),
        }

        if asset.provenance.trim().is_empty() {
            findings.push(ManifestFinding::new(
                ManifestFindingCode::MissingAssetProvenance,
                ManifestFindingSeverity::Warning,
                &asset.asset_id,
                "asset provenance should be recorded",
            ));
        }

        if asset.privacy_class == PrivacyClass::PrivateProjectData {
            findings.push(ManifestFinding::new(
                ManifestFindingCode::ProtectedContentSuspected,
                ManifestFindingSeverity::Warning,
                &asset.asset_id,
                "private project asset is referenced by metadata only and must not be copied into public artifacts",
            ));
        }
    }
}

fn check_professional_boundary(manifest: &AuditManifest, findings: &mut Vec<ManifestFinding>) {
    let boundary = manifest.professional_boundary;
    if boundary.software_makes_compliance_claim
        || boundary.software_makes_certification_claim
        || boundary.software_makes_sealing_claim
        || boundary.software_makes_approval_claim
        || !boundary.human_review_required
    {
        findings.push(ManifestFinding::new(
            ManifestFindingCode::ProfessionalBoundaryViolation,
            ManifestFindingSeverity::Blocking,
            &manifest.manifest_id,
            "audit manifest must preserve the professional responsibility boundary",
        ));
    }
}

fn quote_json_string(value: &str) -> String {
    let mut out = String::from("\"");
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch if ch <= '\u{1f}' => out.push_str(&format!("\\u{:04x}", ch as u32)),
            ch => out.push(ch),
        }
    }
    out.push('"');
    out
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = sha256(bytes);
    let mut out = String::with_capacity(64);
    for byte in digest {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

fn sha256(input: &[u8]) -> [u8; 32] {
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

    let mut message = input.to_vec();
    let bit_len = (message.len() as u64) * 8;
    message.push(0x80);
    while (message.len() % 64) != 56 {
        message.push(0);
    }
    message.extend_from_slice(&bit_len.to_be_bytes());

    let mut h = H0;
    for chunk in message.chunks(64) {
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

    let mut out = [0u8; 32];
    for (chunk, word) in out.chunks_mut(4).zip(h) {
        chunk.copy_from_slice(&word.to_be_bytes());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model_payload(revision: &str) -> CanonicalJson {
        CanonicalJson::object([
            (
                "model_id",
                CanonicalJson::String("invented-model".to_string()),
            ),
            ("revision", CanonicalJson::String(revision.to_string())),
            (
                "unit_system",
                CanonicalJson::String("invented-si".to_string()),
            ),
        ])
    }

    fn valid_manifest(model_hash: HashRecord) -> AuditManifest {
        AuditManifest {
            manifest_id: "manifest-1".to_string(),
            model_hash: Some(model_hash),
            input_manifest_hash: None,
            solver_version: SolverVersionStamp {
                solver_name: "open_pipe_stress_core".to_string(),
                solver_version: "0.1.0".to_string(),
                solver_build_ref: "test-build".to_string(),
            },
            unit_system_ref: "invented-si".to_string(),
            rule_pack_refs: vec![RulePackAuditRef {
                rule_pack_id: "invented-rule-pack".to_string(),
                rule_pack_version: "0.1.0".to_string(),
                source_notice: "invented non-engineering example".to_string(),
                redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
                checksum: Some(HashRecord::sha256_project_local_json(
                    PayloadKind::RulePackReference,
                    "invented-rule-pack",
                    b"{\"rule_pack\":\"invented\"}",
                )),
                private_payload_redacted: true,
            }],
            assets: vec![AssetManifestEntry {
                asset_id: "binary-asset-1".to_string(),
                payload_kind: PayloadKind::BinaryAsset,
                privacy_class: PrivacyClass::Redacted,
                hash: Some(hash_asset("binary-asset-1", b"not-json")),
                provenance: "invented asset bytes".to_string(),
            }],
            professional_boundary: ProfessionalBoundary::project_default(),
        }
    }

    #[test]
    fn project_local_json_hash_is_stable_under_object_key_order() {
        let a = CanonicalJson::object([
            ("z", CanonicalJson::String("last".to_string())),
            ("a", CanonicalJson::String("first".to_string())),
        ]);
        let b = CanonicalJson::object([
            ("a", CanonicalJson::String("first".to_string())),
            ("z", CanonicalJson::String("last".to_string())),
        ]);

        assert_eq!(a.canonical_string(), "{\"a\":\"first\",\"z\":\"last\"}");
        assert_eq!(
            hash_canonical_json(PayloadKind::ModelJson, "model", &a).value,
            hash_canonical_json(PayloadKind::ModelJson, "model", &b).value
        );
    }

    #[test]
    fn material_model_input_change_changes_hash() {
        let initial = hash_canonical_json(PayloadKind::ModelJson, "model", &model_payload("r1"));
        let changed = hash_canonical_json(PayloadKind::ModelJson, "model", &model_payload("r2"));

        assert_ne!(initial.value, changed.value);
    }

    #[test]
    fn records_non_json_asset_hash_separately_from_model_hash() {
        let model_hash = hash_canonical_json(PayloadKind::ModelJson, "model", &model_payload("r1"));
        let asset_hash = hash_asset("asset", b"binary-like content");

        assert_eq!(
            model_hash.canonicalization,
            Canonicalization::ProjectLocalDeterministicJson
        );
        assert_eq!(asset_hash.canonicalization, Canonicalization::None);
        assert_eq!(asset_hash.payload_kind, PayloadKind::BinaryAsset);
        assert_ne!(model_hash.value, asset_hash.value);
    }

    #[test]
    fn valid_manifest_with_rule_pack_checksum_has_no_findings() {
        let model_hash = hash_canonical_json(PayloadKind::ModelJson, "model", &model_payload("r1"));
        let result = validate_manifest(&valid_manifest(model_hash));

        assert!(result.findings.is_empty());
    }

    #[test]
    fn missing_model_hash_solver_version_and_unit_system_are_blocking() {
        let mut manifest = valid_manifest(hash_canonical_json(
            PayloadKind::ModelJson,
            "model",
            &model_payload("r1"),
        ));
        manifest.model_hash = None;
        manifest.solver_version.solver_version.clear();
        manifest.unit_system_ref.clear();

        let result = validate_manifest(&manifest);

        assert!(result.has_blocking_findings());
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == ManifestFindingCode::MissingModelHash));
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == ManifestFindingCode::MissingSolverVersion));
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == ManifestFindingCode::MissingUnitSystem));
    }

    #[test]
    fn missing_rule_pack_checksum_is_blocking() {
        let model_hash = hash_canonical_json(PayloadKind::ModelJson, "model", &model_payload("r1"));
        let mut manifest = valid_manifest(model_hash);
        manifest.rule_pack_refs[0].checksum = None;

        let result = validate_manifest(&manifest);

        assert!(result.has_blocking_findings());
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == ManifestFindingCode::MissingRulePackChecksum));
    }

    #[test]
    fn private_asset_metadata_warns_without_copying_payload() {
        let model_hash = hash_canonical_json(PayloadKind::ModelJson, "model", &model_payload("r1"));
        let mut manifest = valid_manifest(model_hash);
        manifest.assets[0].privacy_class = PrivacyClass::PrivateProjectData;

        let result = validate_manifest(&manifest);

        assert!(!result.has_blocking_findings());
        assert!(result.findings.iter().any(|finding| {
            finding.code == ManifestFindingCode::ProtectedContentSuspected
                && finding.severity == ManifestFindingSeverity::Warning
        }));
    }

    #[test]
    fn professional_boundary_violation_is_blocking() {
        let model_hash = hash_canonical_json(PayloadKind::ModelJson, "model", &model_payload("r1"));
        let mut manifest = valid_manifest(model_hash);
        manifest
            .professional_boundary
            .software_makes_compliance_claim = true;

        let result = validate_manifest(&manifest);

        assert!(result.has_blocking_findings());
        assert!(result
            .findings
            .iter()
            .any(|finding| { finding.code == ManifestFindingCode::ProfessionalBoundaryViolation }));
    }

    #[test]
    fn status_text_does_not_include_approval_or_compliance_claims() {
        let boundary = ProfessionalBoundary::project_default();
        assert!(!boundary.software_makes_compliance_claim);
        assert!(!boundary.software_makes_certification_claim);
        assert!(!boundary.software_makes_sealing_claim);
        assert!(!boundary.software_makes_approval_claim);
        assert!(boundary.human_review_required);
    }
}
