//! Result export envelope support.
//!
//! This crate validates bounded in-memory result export records and prepares
//! deterministic ordering for schema-first JSON result envelopes. It does not
//! parse project files, call solver internals, render reports, run GUI or CLI
//! workflows, implement adapters, access host resources, or emit professional
//! or code-compliance claims.

use serde_json::{json, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnalysisStatus {
    ModelIncomplete,
    MechanicsSolved,
    RuleInputsIncomplete,
    UserRuleChecked,
    UserRuleFailed,
    HumanReviewRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticClass {
    SolveBlocking,
    RuleCheckBlocking,
    ProvenanceWarning,
    AssumptionWarning,
    NonlinearWarning,
    IpBoundaryWarning,
    UnitWarning,
    ExportBlocking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Blocking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DimensionId {
    Dimensionless,
    Length,
    Angle,
    Force,
    Moment,
    Stress,
    Area,
    SectionModulus,
    SecondMomentArea,
    Ratio,
    Time,
    Temperature,
    Pressure,
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResultFamily {
    Displacement,
    Rotation,
    Force,
    Moment,
    Reaction,
    Stress,
    SectionProperty,
    Ratio,
    RuleCheck,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResultSetType {
    Mechanics,
    StressRecovery,
    LoadVectorEvidence,
    StationResultants,
    SectionPropertyEvidence,
    UserRuleCheck,
    DiagnosticOnly,
    Tbd,
}

impl ResultSetType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Mechanics => "mechanics",
            Self::StressRecovery => "stress_recovery",
            Self::LoadVectorEvidence => "load_vector_evidence",
            Self::StationResultants => "station_resultants",
            Self::SectionPropertyEvidence => "section_property_evidence",
            Self::UserRuleCheck => "user_rule_check",
            Self::DiagnosticOnly => "diagnostic_only",
            Self::Tbd => "TBD",
        }
    }
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
pub struct Reference {
    pub ref_type: String,
    pub ref_id: String,
}

impl Reference {
    pub fn new(ref_type: impl Into<String>, ref_id: impl Into<String>) -> Self {
        Self {
            ref_type: ref_type.into(),
            ref_id: ref_id.into(),
        }
    }

    fn is_complete(&self) -> bool {
        !self.ref_type.trim().is_empty() && !self.ref_id.trim().is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provenance {
    pub source_name: String,
    pub source_location: String,
    pub source_license: String,
    pub contributor: String,
    pub contributor_certification: String,
    pub redistribution_status: RedistributionStatus,
    pub review_status: String,
}

impl Provenance {
    fn is_complete(&self) -> bool {
        !self.source_name.trim().is_empty()
            && !self.source_location.trim().is_empty()
            && !self.source_license.trim().is_empty()
            && !self.contributor.trim().is_empty()
            && !self.contributor_certification.trim().is_empty()
            && !self.review_status.trim().is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecksumRef {
    pub algorithm: String,
    pub canonicalization: String,
    pub payload_ref: Reference,
    pub value: String,
}

impl ChecksumRef {
    fn is_complete(&self) -> bool {
        !self.algorithm.trim().is_empty()
            && !self.canonicalization.trim().is_empty()
            && self.payload_ref.is_complete()
            && !self.value.trim().is_empty()
            && !self.value.trim().eq_ignore_ascii_case("TBD")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResultTraceLink {
    pub trace_id: String,
    pub trace_type: String,
    pub source_ref: Reference,
    pub target_ref: Reference,
    pub provenance: Provenance,
    pub diagnostics: Vec<Diagnostic>,
}

impl ResultTraceLink {
    fn is_complete(&self) -> bool {
        !self.trace_id.trim().is_empty()
            && !self.trace_type.trim().is_empty()
            && !self.trace_type.trim().eq_ignore_ascii_case("TBD")
            && self.source_ref.is_complete()
            && self.target_ref.is_complete()
            && self.provenance.is_complete()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuantityResult {
    pub result_id: String,
    pub family: ResultFamily,
    pub object_ref: Reference,
    pub basis_ref: Reference,
    pub station_ref: Option<Reference>,
    pub magnitude: f64,
    pub unit: String,
    pub dimension: DimensionId,
    pub metadata: Option<ResultMetadata>,
    pub diagnostics: Vec<Diagnostic>,
    pub trace_chain: Vec<ResultTraceLink>,
    pub provenance: Provenance,
}

impl QuantityResult {
    fn ordering_key(&self) -> (&str, ResultFamily, &str, &str) {
        (
            self.basis_ref.ref_id.as_str(),
            self.family,
            self.object_ref.ref_id.as_str(),
            self.result_id.as_str(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultMetadata {
    pub component: String,
    pub coordinate_system: String,
    pub location: String,
    pub basis: String,
    pub sign_convention: String,
}

impl ResultMetadata {
    fn is_complete(&self) -> bool {
        !self.component.trim().is_empty()
            && !self.component.trim().eq_ignore_ascii_case("TBD")
            && !self.coordinate_system.trim().is_empty()
            && !self.coordinate_system.trim().eq_ignore_ascii_case("TBD")
            && !self.location.trim().is_empty()
            && !self.location.trim().eq_ignore_ascii_case("TBD")
            && !self.basis.trim().is_empty()
            && !self.basis.trim().eq_ignore_ascii_case("TBD")
            && !self.sign_convention.trim().is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResultSet {
    pub set_id: String,
    pub set_type: String,
    pub basis_ref: Reference,
    pub values: Vec<QuantityResult>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: String,
    pub class: DiagnosticClass,
    pub severity: DiagnosticSeverity,
    pub source: Reference,
    pub affected_object: Reference,
    pub message: String,
    pub remediation: String,
    pub provenance: Provenance,
}

impl Diagnostic {
    fn export_blocking(
        code: impl Into<String>,
        affected_object: Reference,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            class: DiagnosticClass::ExportBlocking,
            severity: DiagnosticSeverity::Blocking,
            source: Reference::new("result_export", "DEL-08-04"),
            affected_object,
            message: message.into(),
            remediation: "Provide explicit result export metadata before relying on this envelope."
                .to_string(),
            provenance: invented_provenance(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfessionalBoundary {
    pub human_review_required: bool,
    pub software_makes_compliance_claim: bool,
    pub software_makes_certification_claim: bool,
    pub software_makes_sealing_claim: bool,
    pub software_makes_approval_claim: bool,
    pub software_makes_authentication_claim: bool,
}

impl ProfessionalBoundary {
    pub fn project_default() -> Self {
        Self {
            human_review_required: true,
            software_makes_compliance_claim: false,
            software_makes_certification_claim: false,
            software_makes_sealing_claim: false,
            software_makes_approval_claim: false,
            software_makes_authentication_claim: false,
        }
    }

    fn preserves_boundary(&self) -> bool {
        self.human_review_required
            && !self.software_makes_compliance_claim
            && !self.software_makes_certification_claim
            && !self.software_makes_sealing_claim
            && !self.software_makes_approval_claim
            && !self.software_makes_authentication_claim
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RulePackExportRef {
    pub rule_pack_id: String,
    pub version: String,
    pub checksum: ChecksumRef,
    pub source_notice: String,
    pub redistribution_status: RedistributionStatus,
    pub completeness_status: String,
    pub private_payload_redacted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReproducibilityRefs {
    pub model_hash: ChecksumRef,
    pub run_hashes: Vec<ChecksumRef>,
    pub audit_manifest_ref: Reference,
    pub deterministic_ordering: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResultEnvelope {
    pub envelope_id: String,
    pub schema_version: String,
    pub model_ref: Reference,
    pub run_ref: Reference,
    pub solver_name: String,
    pub solver_version: String,
    pub solver_build_ref: String,
    pub unit_system_ref: Reference,
    pub load_basis_refs: Vec<Reference>,
    pub result_sets: Vec<ResultSet>,
    pub diagnostics: Vec<Diagnostic>,
    pub provenance: Provenance,
    pub reproducibility: ReproducibilityRefs,
    pub analysis_status: Vec<AnalysisStatus>,
    pub rule_pack_refs: Vec<RulePackExportRef>,
    pub professional_boundary: ProfessionalBoundary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportValidation {
    pub diagnostics: Vec<Diagnostic>,
}

impl ExportValidation {
    pub fn has_blocking_diagnostics(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Blocking)
    }
}

pub fn validate_result_envelope(envelope: &ResultEnvelope) -> ExportValidation {
    let mut diagnostics = Vec::new();

    if envelope.envelope_id.trim().is_empty() {
        diagnostics.push(Diagnostic::export_blocking(
            "RESULT_EXPORT_ID_MISSING",
            Reference::new("result_envelope", "UNKNOWN"),
            "result envelope must have an envelope identifier",
        ));
    }

    if !envelope.model_ref.is_complete() || !envelope.run_ref.is_complete() {
        diagnostics.push(Diagnostic::export_blocking(
            "RESULT_EXPORT_REFERENCE_MISSING",
            Reference::new("result_envelope", &envelope.envelope_id),
            "result envelope must reference a model and run",
        ));
    }

    if envelope.solver_name.trim().is_empty() || envelope.solver_version.trim().is_empty() {
        diagnostics.push(Diagnostic::export_blocking(
            "RESULT_EXPORT_SOLVER_VERSION_MISSING",
            Reference::new("result_envelope", &envelope.envelope_id),
            "result export must include solver name and version",
        ));
    }

    if !envelope.unit_system_ref.is_complete() {
        diagnostics.push(Diagnostic::export_blocking(
            "RESULT_EXPORT_UNIT_SYSTEM_MISSING",
            Reference::new("result_envelope", &envelope.envelope_id),
            "result export must identify the unit-system basis",
        ));
    }

    if envelope.load_basis_refs.is_empty()
        || envelope
            .load_basis_refs
            .iter()
            .any(|reference| !reference.is_complete())
    {
        diagnostics.push(Diagnostic::export_blocking(
            "RESULT_EXPORT_LOAD_BASIS_MISSING",
            Reference::new("result_envelope", &envelope.envelope_id),
            "result export must identify load-case or combination basis references",
        ));
    }

    if !envelope
        .analysis_status
        .contains(&AnalysisStatus::HumanReviewRequired)
    {
        diagnostics.push(Diagnostic::export_blocking(
            "RESULT_EXPORT_HUMAN_REVIEW_STATUS_MISSING",
            Reference::new("result_envelope", &envelope.envelope_id),
            "result exports must preserve human-review-required status",
        ));
    }

    if !envelope.professional_boundary.preserves_boundary() {
        diagnostics.push(Diagnostic::export_blocking(
            "RESULT_EXPORT_PROFESSIONAL_BOUNDARY_VIOLATION",
            Reference::new("result_envelope", &envelope.envelope_id),
            "result export must not make compliance, certification, sealing, approval, or authentication claims",
        ));
    }

    if !envelope.provenance.is_complete() {
        diagnostics.push(Diagnostic::export_blocking(
            "RESULT_EXPORT_PROVENANCE_MISSING",
            Reference::new("result_envelope", &envelope.envelope_id),
            "result export must include source and redistribution provenance",
        ));
    }

    if !envelope.reproducibility.model_hash.is_complete()
        || !envelope.reproducibility.audit_manifest_ref.is_complete()
        || !envelope.reproducibility.deterministic_ordering
    {
        diagnostics.push(Diagnostic::export_blocking(
            "RESULT_EXPORT_REPRODUCIBILITY_MISSING",
            Reference::new("result_envelope", &envelope.envelope_id),
            "result export must include model hash, audit manifest reference, and deterministic ordering",
        ));
    }

    check_rule_pack_refs(envelope, &mut diagnostics);
    check_result_sets(envelope, &mut diagnostics);

    ExportValidation { diagnostics }
}

pub fn sorted_result_values(envelope: &ResultEnvelope) -> Vec<&QuantityResult> {
    let mut values: Vec<_> = envelope
        .result_sets
        .iter()
        .flat_map(|set| set.values.iter())
        .collect();
    values.sort_by_key(|value| value.ordering_key());
    values
}

pub fn result_export_document(envelope: &ResultEnvelope) -> Value {
    json!({
        "schema_version": envelope.schema_version,
        "deliverable_id": "DEL-08-04",
        "package_id": "PKG-08",
        "scope_item": "SOW-046",
        "objectives": ["OBJ-007", "OBJ-009"],
        "export_format_status": {
            "baseline_format": "schema_first_json_result_envelope",
            "additional_formats": "TBD",
            "public_transport_protocol": "TBD",
            "local_fea_package_format": "TBD",
            "external_adapter_formats": "TBD"
        },
        "result_envelope": result_envelope_json(envelope)
    })
}

fn result_envelope_json(envelope: &ResultEnvelope) -> Value {
    json!({
        "schema_version": envelope.schema_version,
        "envelope_id": envelope.envelope_id,
        "model_ref": reference_json(&envelope.model_ref),
        "run_ref": reference_json(&envelope.run_ref),
        "solver_version": {
            "solver_name": envelope.solver_name,
            "solver_version": envelope.solver_version,
            "solver_build_ref": envelope.solver_build_ref
        },
        "unit_system_ref": reference_json(&envelope.unit_system_ref),
        "load_basis_refs": envelope.load_basis_refs.iter().map(reference_json).collect::<Vec<_>>(),
        "result_sets": envelope.result_sets.iter().map(result_set_json).collect::<Vec<_>>(),
        "diagnostics": envelope.diagnostics.iter().map(diagnostic_json).collect::<Vec<_>>(),
        "provenance": provenance_json(&envelope.provenance),
        "reproducibility": reproducibility_json(&envelope.reproducibility),
        "analysis_status": envelope.analysis_status.iter().map(|status| analysis_status_json(*status)).collect::<Vec<_>>(),
        "rule_pack_refs": envelope.rule_pack_refs.iter().map(rule_pack_ref_json).collect::<Vec<_>>(),
        "professional_boundary": professional_boundary_json(&envelope.professional_boundary),
        "downstream_use": {
            "review": true,
            "regression_comparison": true,
            "report_consumption": true,
            "headless_automation": true,
            "governed_downstream_tooling": true,
            "additional_export_formats": "TBD"
        }
    })
}

fn result_set_json(set: &ResultSet) -> Value {
    json!({
        "set_id": set.set_id,
        "set_type": set.set_type,
        "basis_ref": reference_json(&set.basis_ref),
        "values": set.values.iter().map(quantity_result_json).collect::<Vec<_>>()
    })
}

fn quantity_result_json(value: &QuantityResult) -> Value {
    let mut item = json!({
        "result_id": value.result_id,
        "family": result_family_json(value.family),
        "object_ref": reference_json(&value.object_ref),
        "basis_ref": reference_json(&value.basis_ref),
        "magnitude": value.magnitude,
        "unit": value.unit,
        "dimension": dimension_json(value.dimension),
        "provenance": provenance_json(&value.provenance)
    });

    let object = item
        .as_object_mut()
        .expect("quantity result serialization produces an object");
    if let Some(station_ref) = &value.station_ref {
        object.insert("station_ref".to_string(), reference_json(station_ref));
    }
    if let Some(metadata) = &value.metadata {
        object.insert("metadata".to_string(), metadata_json(metadata));
    }
    if !value.diagnostics.is_empty() {
        object.insert(
            "diagnostics".to_string(),
            Value::Array(value.diagnostics.iter().map(diagnostic_json).collect()),
        );
    }
    if !value.trace_chain.is_empty() {
        object.insert(
            "trace_chain".to_string(),
            Value::Array(value.trace_chain.iter().map(trace_link_json).collect()),
        );
    }
    item
}

fn metadata_json(metadata: &ResultMetadata) -> Value {
    json!({
        "component": metadata.component,
        "coordinate_system": metadata.coordinate_system,
        "location": metadata.location,
        "basis": metadata.basis,
        "sign_convention": metadata.sign_convention
    })
}

fn diagnostic_json(diagnostic: &Diagnostic) -> Value {
    json!({
        "code": diagnostic.code,
        "class": diagnostic_class_json(diagnostic.class),
        "severity": diagnostic_severity_json(diagnostic.severity),
        "source": reference_json(&diagnostic.source),
        "affected_object": reference_json(&diagnostic.affected_object),
        "message": diagnostic.message,
        "remediation": diagnostic.remediation,
        "provenance": provenance_json(&diagnostic.provenance)
    })
}

fn trace_link_json(trace_link: &ResultTraceLink) -> Value {
    let mut item = json!({
        "trace_id": trace_link.trace_id,
        "trace_type": trace_link.trace_type,
        "source_ref": reference_json(&trace_link.source_ref),
        "target_ref": reference_json(&trace_link.target_ref),
        "provenance": provenance_json(&trace_link.provenance)
    });

    if !trace_link.diagnostics.is_empty() {
        item.as_object_mut()
            .expect("trace link serialization produces an object")
            .insert(
                "diagnostics".to_string(),
                Value::Array(trace_link.diagnostics.iter().map(diagnostic_json).collect()),
            );
    }
    item
}

fn reproducibility_json(reproducibility: &ReproducibilityRefs) -> Value {
    json!({
        "model_hash": checksum_json(&reproducibility.model_hash),
        "run_hashes": reproducibility.run_hashes.iter().map(checksum_json).collect::<Vec<_>>(),
        "audit_manifest_ref": reference_json(&reproducibility.audit_manifest_ref),
        "deterministic_ordering": reproducibility.deterministic_ordering
    })
}

fn rule_pack_ref_json(rule_pack: &RulePackExportRef) -> Value {
    json!({
        "rule_pack_id": rule_pack.rule_pack_id,
        "version": rule_pack.version,
        "checksum": checksum_json(&rule_pack.checksum),
        "source_notice": rule_pack.source_notice,
        "redistribution_status": redistribution_status_json(rule_pack.redistribution_status),
        "completeness_status": rule_pack.completeness_status,
        "private_payload_redacted": rule_pack.private_payload_redacted
    })
}

fn checksum_json(checksum: &ChecksumRef) -> Value {
    json!({
        "algorithm": checksum.algorithm,
        "canonicalization": checksum.canonicalization,
        "payload_ref": reference_json(&checksum.payload_ref),
        "value": checksum.value
    })
}

fn provenance_json(provenance: &Provenance) -> Value {
    json!({
        "source_name": provenance.source_name,
        "source_location": provenance.source_location,
        "source_license": provenance.source_license,
        "contributor": provenance.contributor,
        "contributor_certification": provenance.contributor_certification,
        "redistribution_status": redistribution_status_json(provenance.redistribution_status),
        "review_status": provenance.review_status
    })
}

fn professional_boundary_json(boundary: &ProfessionalBoundary) -> Value {
    json!({
        "human_review_required": boundary.human_review_required,
        "software_makes_compliance_claim": boundary.software_makes_compliance_claim,
        "software_makes_certification_claim": boundary.software_makes_certification_claim,
        "software_makes_sealing_claim": boundary.software_makes_sealing_claim,
        "software_makes_approval_claim": boundary.software_makes_approval_claim,
        "software_makes_authentication_claim": boundary.software_makes_authentication_claim
    })
}

fn reference_json(reference: &Reference) -> Value {
    json!({
        "ref_type": reference.ref_type,
        "ref_id": reference.ref_id
    })
}

fn analysis_status_json(status: AnalysisStatus) -> &'static str {
    match status {
        AnalysisStatus::ModelIncomplete => "MODEL_INCOMPLETE",
        AnalysisStatus::MechanicsSolved => "MECHANICS_SOLVED",
        AnalysisStatus::RuleInputsIncomplete => "RULE_INPUTS_INCOMPLETE",
        AnalysisStatus::UserRuleChecked => "USER_RULE_CHECKED",
        AnalysisStatus::UserRuleFailed => "USER_RULE_FAILED",
        AnalysisStatus::HumanReviewRequired => "HUMAN_REVIEW_REQUIRED",
    }
}

fn diagnostic_class_json(class: DiagnosticClass) -> &'static str {
    match class {
        DiagnosticClass::SolveBlocking => "SOLVE_BLOCKING",
        DiagnosticClass::RuleCheckBlocking => "RULE_CHECK_BLOCKING",
        DiagnosticClass::ProvenanceWarning => "PROVENANCE_WARNING",
        DiagnosticClass::AssumptionWarning => "ASSUMPTION_WARNING",
        DiagnosticClass::NonlinearWarning => "NONLINEAR_WARNING",
        DiagnosticClass::IpBoundaryWarning => "IP_BOUNDARY_WARNING",
        DiagnosticClass::UnitWarning => "UNIT_WARNING",
        DiagnosticClass::ExportBlocking => "EXPORT_BLOCKING",
    }
}

fn diagnostic_severity_json(severity: DiagnosticSeverity) -> &'static str {
    match severity {
        DiagnosticSeverity::Info => "info",
        DiagnosticSeverity::Warning => "warning",
        DiagnosticSeverity::Blocking => "blocking",
    }
}

fn dimension_json(dimension: DimensionId) -> &'static str {
    match dimension {
        DimensionId::Dimensionless => "dimensionless",
        DimensionId::Length => "length",
        DimensionId::Angle => "angle",
        DimensionId::Force => "force",
        DimensionId::Moment => "moment",
        DimensionId::Stress => "stress",
        DimensionId::Area => "area",
        DimensionId::SectionModulus => "section_modulus",
        DimensionId::SecondMomentArea => "second_moment_area",
        DimensionId::Ratio => "ratio",
        DimensionId::Time => "time",
        DimensionId::Temperature => "temperature",
        DimensionId::Pressure => "pressure",
        DimensionId::Tbd => "TBD",
    }
}

fn result_family_json(family: ResultFamily) -> &'static str {
    match family {
        ResultFamily::Displacement => "displacement",
        ResultFamily::Rotation => "rotation",
        ResultFamily::Force => "force",
        ResultFamily::Moment => "moment",
        ResultFamily::Reaction => "reaction",
        ResultFamily::Stress => "stress",
        ResultFamily::SectionProperty => "section_property",
        ResultFamily::Ratio => "ratio",
        ResultFamily::RuleCheck => "rule_check",
    }
}

fn redistribution_status_json(status: RedistributionStatus) -> &'static str {
    match status {
        RedistributionStatus::PublicPermissive => "public_permissive",
        RedistributionStatus::PrivateOnly => "private_only",
        RedistributionStatus::Unknown => "unknown",
        RedistributionStatus::ProtectedSuspected => "protected_suspected",
        RedistributionStatus::InventedNonEngineeringExample => "invented_non_engineering_example",
        RedistributionStatus::Tbd => "TBD",
    }
}

fn check_rule_pack_refs(envelope: &ResultEnvelope, diagnostics: &mut Vec<Diagnostic>) {
    for rule_pack in &envelope.rule_pack_refs {
        if rule_pack.rule_pack_id.trim().is_empty()
            || rule_pack.version.trim().is_empty()
            || !rule_pack.checksum.is_complete()
            || rule_pack.source_notice.trim().is_empty()
            || rule_pack.completeness_status.trim().is_empty()
            || !rule_pack.private_payload_redacted
        {
            diagnostics.push(Diagnostic::export_blocking(
                "RESULT_EXPORT_RULE_PACK_REF_INCOMPLETE",
                Reference::new("rule_pack", &rule_pack.rule_pack_id),
                "rule-pack exports must include identity, version, checksum, source notice, completeness, and redaction metadata",
            ));
        }

        if rule_pack.redistribution_status == RedistributionStatus::ProtectedSuspected {
            diagnostics.push(Diagnostic::export_blocking(
                "RESULT_EXPORT_PROTECTED_RULE_PACK_SUSPECTED",
                Reference::new("rule_pack", &rule_pack.rule_pack_id),
                "suspected protected rule-pack content must not be exported as a public payload",
            ));
        }
    }
}

fn check_result_sets(envelope: &ResultEnvelope, diagnostics: &mut Vec<Diagnostic>) {
    if envelope.result_sets.is_empty() {
        diagnostics.push(Diagnostic::export_blocking(
            "RESULT_EXPORT_RESULT_SET_MISSING",
            Reference::new("result_envelope", &envelope.envelope_id),
            "result export must include at least one result set",
        ));
        return;
    }

    for set in &envelope.result_sets {
        if set.set_id.trim().is_empty() || !set.basis_ref.is_complete() {
            diagnostics.push(Diagnostic::export_blocking(
                "RESULT_EXPORT_RESULT_SET_INCOMPLETE",
                Reference::new("result_set", &set.set_id),
                "result set must include identity, type, and basis reference",
            ));
        }

        if !is_supported_result_set_type(&set.set_type) {
            diagnostics.push(Diagnostic::export_blocking(
                "RESULT_EXPORT_RESULT_SET_TYPE_UNSUPPORTED",
                Reference::new("result_set", &set.set_id),
                "result set type must use the governed result-export vocabulary",
            ));
        }

        for value in &set.values {
            if value.result_id.trim().is_empty()
                || !value.object_ref.is_complete()
                || !value.basis_ref.is_complete()
                || !value.magnitude.is_finite()
                || value.unit.trim().is_empty()
                || value.dimension == DimensionId::Tbd
                || !value.provenance.is_complete()
            {
                diagnostics.push(Diagnostic::export_blocking(
                    "RESULT_EXPORT_VALUE_METADATA_INCOMPLETE",
                    Reference::new("result_value", &value.result_id),
                    "result values must carry object, basis, finite magnitude, unit, dimension, and provenance metadata",
                ));
            }

            if matches!(
                value.family,
                ResultFamily::Force | ResultFamily::Moment | ResultFamily::SectionProperty
            ) && value
                .metadata
                .as_ref()
                .map(|metadata| !metadata.is_complete())
                .unwrap_or(true)
            {
                diagnostics.push(Diagnostic::export_blocking(
                    "RESULT_EXPORT_FORCE_MOMENT_METADATA_INCOMPLETE",
                    Reference::new("result_value", &value.result_id),
                    "force, moment, and section-property results must carry component, coordinate system, endpoint location, recovery basis, and sign convention metadata",
                ));
            }

            for trace_link in &value.trace_chain {
                if !trace_link.is_complete() {
                    diagnostics.push(Diagnostic::export_blocking(
                        "RESULT_EXPORT_TRACE_LINK_INCOMPLETE",
                        Reference::new("result_value", &value.result_id),
                        "result trace links must include identity, trace type, source, target, and provenance references",
                    ));
                }
            }
        }
    }
}

fn is_supported_result_set_type(set_type: &str) -> bool {
    [
        ResultSetType::Mechanics,
        ResultSetType::StressRecovery,
        ResultSetType::LoadVectorEvidence,
        ResultSetType::StationResultants,
        ResultSetType::SectionPropertyEvidence,
        ResultSetType::UserRuleCheck,
        ResultSetType::DiagnosticOnly,
    ]
    .iter()
    .any(|candidate| candidate.as_str() == set_type)
}

fn invented_provenance() -> Provenance {
    Provenance {
        source_name: "OpenPipeStress result export validator".to_string(),
        source_location: "core/reporting/result_export".to_string(),
        source_license: "project".to_string(),
        contributor: "OpenPipeStress".to_string(),
        contributor_certification: "diagnostic metadata only".to_string(),
        redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
        review_status: "accepted".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TP_RESULT_017_SERIALIZED_FIXTURE: &str = include_str!(
        "../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json"
    );
    const TP_SECTION_021_SERIALIZED_FIXTURE: &str = include_str!(
        "../../../../fixtures/results/invented/tp_phys_015_section_property_stress_evidence_envelope.json"
    );

    fn checksum(id: &str) -> ChecksumRef {
        ChecksumRef {
            algorithm: "sha256".to_string(),
            canonicalization: "JCS".to_string(),
            payload_ref: Reference::new("payload", id),
            value: format!("{id}-hash"),
        }
    }

    fn provenance() -> Provenance {
        Provenance {
            source_name: "invented mechanics fixture".to_string(),
            source_location: "validation/benchmarks/invented".to_string(),
            source_license: "project invented".to_string(),
            contributor: "OpenPipeStress".to_string(),
            contributor_certification: "invented non-engineering example".to_string(),
            redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
            review_status: "accepted".to_string(),
        }
    }

    fn quantity(
        id: &str,
        family: ResultFamily,
        unit: &str,
        dimension: DimensionId,
    ) -> QuantityResult {
        QuantityResult {
            result_id: id.to_string(),
            family,
            object_ref: Reference::new("node", "N1"),
            basis_ref: Reference::new("load_case", "LC1"),
            station_ref: None,
            magnitude: 1.25,
            unit: unit.to_string(),
            dimension,
            metadata: None,
            diagnostics: Vec::new(),
            trace_chain: Vec::new(),
            provenance: provenance(),
        }
    }

    fn local_force_quantity(id: &str) -> QuantityResult {
        QuantityResult {
            result_id: id.to_string(),
            family: ResultFamily::Force,
            object_ref: Reference::new("pipe", "pipe:P-120"),
            basis_ref: Reference::new("load_case", "LC1"),
            station_ref: None,
            magnitude: 120.0,
            unit: "N".to_string(),
            dimension: DimensionId::Force,
            metadata: Some(ResultMetadata {
                component: "axial_force".to_string(),
                coordinate_system: "element_local".to_string(),
                location: "end_i".to_string(),
                basis: "recovered_from_local_element_stiffness".to_string(),
                sign_convention:
                    "positive value follows the element-local DOF at the i-end force vector"
                        .to_string(),
            }),
            diagnostics: Vec::new(),
            trace_chain: Vec::new(),
            provenance: provenance(),
        }
    }

    fn envelope() -> ResultEnvelope {
        ResultEnvelope {
            envelope_id: "result-envelope-1".to_string(),
            schema_version: "0.1.0".to_string(),
            model_ref: Reference::new("model", "invented-model"),
            run_ref: Reference::new("analysis_run", "run-1"),
            solver_name: "open_pipe_stress_core".to_string(),
            solver_version: "0.1.0".to_string(),
            solver_build_ref: "test-build".to_string(),
            unit_system_ref: Reference::new("unit_system", "invented-si"),
            load_basis_refs: vec![Reference::new("load_case", "LC1")],
            result_sets: vec![ResultSet {
                set_id: "set-1".to_string(),
                set_type: ResultSetType::Mechanics.as_str().to_string(),
                basis_ref: Reference::new("load_case", "LC1"),
                values: vec![
                    quantity("stress-1", ResultFamily::Stress, "Pa", DimensionId::Stress),
                    quantity(
                        "disp-1",
                        ResultFamily::Displacement,
                        "m",
                        DimensionId::Length,
                    ),
                    quantity(
                        "reaction-1",
                        ResultFamily::Reaction,
                        "N",
                        DimensionId::Force,
                    ),
                    local_force_quantity("result:force:pipe-P-120:axial"),
                ],
            }],
            diagnostics: Vec::new(),
            provenance: provenance(),
            reproducibility: ReproducibilityRefs {
                model_hash: checksum("model"),
                run_hashes: vec![checksum("run")],
                audit_manifest_ref: Reference::new("audit_manifest", "manifest-1"),
                deterministic_ordering: true,
            },
            analysis_status: vec![
                AnalysisStatus::MechanicsSolved,
                AnalysisStatus::HumanReviewRequired,
            ],
            rule_pack_refs: vec![RulePackExportRef {
                rule_pack_id: "invented-rule-pack".to_string(),
                version: "0.1.0".to_string(),
                checksum: checksum("rule-pack"),
                source_notice: "invented non-engineering example".to_string(),
                redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
                completeness_status: "complete_for_user_rule_check".to_string(),
                private_payload_redacted: true,
            }],
            professional_boundary: ProfessionalBoundary::project_default(),
        }
    }

    fn tp_result_017_envelope() -> ResultEnvelope {
        let load_case_ref = Reference::new("load_case", "LC-TP-PHYS-014");
        let provenance = provenance();
        ResultEnvelope {
            envelope_id: "MECH-TP-PHYS-015-CANONICAL-SOLVE-RESULT-ENVELOPE".to_string(),
            schema_version: "0.1.0".to_string(),
            model_ref: Reference::new("analytical_solver_model", "ANALYTICAL-TP-PHYS-014"),
            run_ref: Reference::new(
                "analysis_run",
                "RUN-TP-PHYS-015A-CANONICAL-SOLVE-RESULT-ENVELOPE",
            ),
            solver_name: "open_pipe_stress_validation_mechanics_benchmark".to_string(),
            solver_version: "0.1.0".to_string(),
            solver_build_ref: "validation-local-cargo-test".to_string(),
            unit_system_ref: Reference::new("unit_system", "PKG09-FIXTURE-UNITS-EXPLICIT-N-M-RAD-K"),
            load_basis_refs: vec![load_case_ref.clone()],
            result_sets: vec![
                ResultSet {
                    set_id: "result-set:tp-phys-015:mechanics".to_string(),
                    set_type: ResultSetType::Mechanics.as_str().to_string(),
                    basis_ref: load_case_ref.clone(),
                    values: vec![
                        QuantityResult {
                            result_id: "result:disp:node-N-2:uy".to_string(),
                            family: ResultFamily::Displacement,
                            object_ref: Reference::new("node", "N-2"),
                            basis_ref: load_case_ref.clone(),
                            station_ref: None,
                            magnitude: -0.04533333333333334,
                            unit: "m".to_string(),
                            dimension: DimensionId::Length,
                            metadata: None,
                            diagnostics: Vec::new(),
                            trace_chain: Vec::new(),
                            provenance: provenance.clone(),
                        },
                        QuantityResult {
                            result_id: "result:rotation:node-N-2:rz".to_string(),
                            family: ResultFamily::Rotation,
                            object_ref: Reference::new("node", "N-2"),
                            basis_ref: load_case_ref.clone(),
                            station_ref: None,
                            magnitude: -0.014666666666666668,
                            unit: "rad".to_string(),
                            dimension: DimensionId::Angle,
                            metadata: None,
                            diagnostics: Vec::new(),
                            trace_chain: Vec::new(),
                            provenance: provenance.clone(),
                        },
                        QuantityResult {
                            result_id: "result:reaction:support-N-1:uy".to_string(),
                            family: ResultFamily::Reaction,
                            object_ref: Reference::new("support", "N-1-anchor"),
                            basis_ref: load_case_ref.clone(),
                            station_ref: None,
                            magnitude: 12.0,
                            unit: "N".to_string(),
                            dimension: DimensionId::Force,
                            metadata: None,
                            diagnostics: Vec::new(),
                            trace_chain: Vec::new(),
                            provenance: provenance.clone(),
                        },
                    ],
                },
                ResultSet {
                    set_id: "result-set:tp-phys-015:load-vector".to_string(),
                    set_type: ResultSetType::LoadVectorEvidence.as_str().to_string(),
                    basis_ref: load_case_ref.clone(),
                    values: vec![QuantityResult {
                        result_id: "result:load-vector:node-N-1:uy".to_string(),
                        family: ResultFamily::Force,
                        object_ref: Reference::new("solver_load_vector", "node-N-1"),
                        basis_ref: load_case_ref.clone(),
                        station_ref: None,
                        magnitude: -6.0,
                        unit: "N".to_string(),
                        dimension: DimensionId::Force,
                        metadata: Some(ResultMetadata {
                            component: "nodal_force_y".to_string(),
                            coordinate_system: "global".to_string(),
                            location: "node".to_string(),
                            basis: "assembled_solver_load_vector".to_string(),
                            sign_convention: "positive follows the solver global UY force entry"
                                .to_string(),
                        }),
                        diagnostics: vec![Diagnostic {
                            code: "TP_RESULT_017_VALUE_TRACE_NOTE".to_string(),
                            class: DiagnosticClass::AssumptionWarning,
                            severity: DiagnosticSeverity::Info,
                            source: Reference::new("result_export_fixture", "TP-RESULT-017"),
                            affected_object: Reference::new(
                                "result_value",
                                "result:load-vector:node-N-1:uy",
                            ),
                            message: "Fixture records per-value diagnostics without changing solver behavior."
                                .to_string(),
                            remediation: "Use accepted trace-chain ruling before adding multi-hop value trace fields."
                                .to_string(),
                            provenance: provenance.clone(),
                        }],
                        trace_chain: vec![ResultTraceLink {
                            trace_id: "trace:tp-phys-015:adapter-dto-to-load-vector-uy"
                                .to_string(),
                            trace_type: "solver_input_to_result_value".to_string(),
                            source_ref: Reference::new(
                                "adapter_dto",
                                "dto:load_application:LC-TP-PHYS-014:0",
                            ),
                            target_ref: Reference::new(
                                "result_value",
                                "result:load-vector:node-N-1:uy",
                            ),
                            provenance: provenance.clone(),
                            diagnostics: Vec::new(),
                        }],
                        provenance: provenance.clone(),
                    }],
                },
                ResultSet {
                    set_id: "result-set:tp-phys-015:station-resultants".to_string(),
                    set_type: ResultSetType::StationResultants.as_str().to_string(),
                    basis_ref: load_case_ref.clone(),
                    values: vec![QuantityResult {
                        result_id: "result:moment:element-E-1:midspan:bending-z".to_string(),
                        family: ResultFamily::Moment,
                        object_ref: Reference::new("element", "E-1"),
                        basis_ref: load_case_ref.clone(),
                        station_ref: Some(Reference::new("result_station", "E-1:midspan")),
                        magnitude: 4.0,
                        unit: "N-m".to_string(),
                        dimension: DimensionId::Moment,
                        metadata: Some(ResultMetadata {
                            component: "bending_moment_z".to_string(),
                            coordinate_system: "element_local".to_string(),
                            location: "midspan".to_string(),
                            basis: "interpolated_from_endpoint_resultants".to_string(),
                            sign_convention:
                                "positive follows the element-local z bending resultant convention"
                                    .to_string(),
                        }),
                        diagnostics: Vec::new(),
                        trace_chain: Vec::new(),
                        provenance: provenance.clone(),
                    }],
                },
            ],
            diagnostics: vec![Diagnostic {
                code: "TP_PHYS_015A_RESULT_BOUNDARY_EVIDENCE".to_string(),
                class: DiagnosticClass::AssumptionWarning,
                severity: DiagnosticSeverity::Info,
                source: Reference::new("mechanics_benchmark", "DEL-09-01"),
                affected_object: Reference::new(
                    "result_envelope",
                    "MECH-TP-PHYS-015-CANONICAL-SOLVE-RESULT-ENVELOPE",
                ),
                message: "Invented validation-local result envelope records canonical payload solve-result evidence.".to_string(),
                remediation:
                    "Use adjacent export and runner contracts before relying on serialized runtime behavior."
                        .to_string(),
                provenance: provenance.clone(),
            }],
            provenance,
            reproducibility: ReproducibilityRefs {
                model_hash: checksum("analytical-solver-model"),
                run_hashes: vec![checksum("analysis-run")],
                audit_manifest_ref: Reference::new(
                    "task_run_record",
                    "TASK_RUN_2026-05-17_TP-RESULT-017",
                ),
                deterministic_ordering: true,
            },
            analysis_status: vec![
                AnalysisStatus::MechanicsSolved,
                AnalysisStatus::HumanReviewRequired,
            ],
            rule_pack_refs: Vec::new(),
            professional_boundary: ProfessionalBoundary::project_default(),
        }
    }

    fn section_property_quantity(
        id: &str,
        component: &str,
        magnitude: f64,
        unit: &str,
        dimension: DimensionId,
    ) -> QuantityResult {
        let evidence_ref = Reference::new(
            "section_property_evidence",
            "SECTION-PROP-TP-STRESS-016-INVENTED-PIPE-OD2-WALL0P25",
        );
        QuantityResult {
            result_id: id.to_string(),
            family: ResultFamily::SectionProperty,
            object_ref: evidence_ref.clone(),
            basis_ref: Reference::new(
                "section_property_calculation",
                "SECTION-PROP-TP-STRESS-016-INVENTED-PIPE-OD2-WALL0P25",
            ),
            station_ref: None,
            magnitude,
            unit: unit.to_string(),
            dimension,
            metadata: Some(ResultMetadata {
                component: component.to_string(),
                coordinate_system: "pipe_section".to_string(),
                location: "summary".to_string(),
                basis: "derived_from_user_entered_section_geometry".to_string(),
                sign_convention: "positive scalar section property".to_string(),
            }),
            diagnostics: Vec::new(),
            trace_chain: Vec::new(),
            provenance: provenance(),
        }
    }

    fn tp_section_021_envelope() -> ResultEnvelope {
        let load_case_ref = Reference::new("load_case", "LC-TP-PHYS-014");
        let section_evidence_ref = Reference::new(
            "section_property_evidence",
            "SECTION-PROP-TP-STRESS-016-INVENTED-PIPE-OD2-WALL0P25",
        );
        let stress_result_ref = Reference::new(
            "result_value",
            "result:stress:element-E-1:midspan:bending-normal-z",
        );
        let provenance = provenance();
        ResultEnvelope {
            envelope_id: "STRESS-TP-PHYS-015-SECTION-EVIDENCE-RESULT-ENVELOPE".to_string(),
            schema_version: "0.1.0".to_string(),
            model_ref: Reference::new("analytical_solver_model", "ANALYTICAL-TP-PHYS-014"),
            run_ref: Reference::new(
                "analysis_run",
                "RUN-TP-SECTION-021-SECTION-EVIDENCE-TRANSPORT",
            ),
            solver_name: "open_pipe_stress_validation_stress_benchmark".to_string(),
            solver_version: "0.1.0".to_string(),
            solver_build_ref: "validation-local-cargo-test".to_string(),
            unit_system_ref: Reference::new("unit_system", "PKG09-FIXTURE-UNITS-EXPLICIT-N-M-RAD-K"),
            load_basis_refs: vec![load_case_ref.clone()],
            result_sets: vec![
                ResultSet {
                    set_id: "result-set:tp-section-021:section-property-evidence".to_string(),
                    set_type: ResultSetType::SectionPropertyEvidence.as_str().to_string(),
                    basis_ref: section_evidence_ref.clone(),
                    values: vec![
                        section_property_quantity(
                            "result:section-property:tp-stress-016:area",
                            "section_area",
                            1.3744467859455345,
                            "m^2",
                            DimensionId::Area,
                        ),
                        section_property_quantity(
                            "result:section-property:tp-stress-016:section-modulus-y",
                            "section_modulus_y",
                            0.5368932757599744,
                            "m^3",
                            DimensionId::SectionModulus,
                        ),
                        section_property_quantity(
                            "result:section-property:tp-stress-016:section-modulus-z",
                            "section_modulus_z",
                            0.5368932757599744,
                            "m^3",
                            DimensionId::SectionModulus,
                        ),
                        section_property_quantity(
                            "result:section-property:tp-stress-016:torsion-constant",
                            "torsion_constant",
                            1.0737865515199487,
                            "m^4",
                            DimensionId::SecondMomentArea,
                        ),
                        section_property_quantity(
                            "result:section-property:tp-stress-016:torsion-radius",
                            "torsion_radius",
                            1.0,
                            "m",
                            DimensionId::Length,
                        ),
                    ],
                },
                ResultSet {
                    set_id: "result-set:tp-section-021:stress-recovery".to_string(),
                    set_type: ResultSetType::StressRecovery.as_str().to_string(),
                    basis_ref: load_case_ref.clone(),
                    values: vec![QuantityResult {
                        result_id: stress_result_ref.ref_id.clone(),
                        family: ResultFamily::Stress,
                        object_ref: Reference::new("element", "E-1"),
                        basis_ref: load_case_ref,
                        station_ref: Some(Reference::new("result_station", "E-1:midspan")),
                        magnitude: 7.450270250336039,
                        unit: "Pa".to_string(),
                        dimension: DimensionId::Stress,
                        metadata: Some(ResultMetadata {
                            component: "bending_normal_stress_z".to_string(),
                            coordinate_system: "element_local".to_string(),
                            location: "midspan".to_string(),
                            basis: "recovered_from_open_mechanics_stress_components".to_string(),
                            sign_convention:
                                "positive follows the element-local z bending stress convention"
                                    .to_string(),
                        }),
                        diagnostics: Vec::new(),
                        trace_chain: vec![ResultTraceLink {
                            trace_id: "trace:tp-section-021:section-evidence-to-bending-stress-z"
                                .to_string(),
                            trace_type: "section_property_evidence_to_result_value".to_string(),
                            source_ref: section_evidence_ref,
                            target_ref: stress_result_ref,
                            provenance: provenance.clone(),
                            diagnostics: Vec::new(),
                        }],
                        provenance: provenance.clone(),
                    }],
                },
            ],
            diagnostics: vec![Diagnostic {
                code: "TP_SECTION_021_SECTION_EVIDENCE_TRANSPORT".to_string(),
                class: DiagnosticClass::AssumptionWarning,
                severity: DiagnosticSeverity::Info,
                source: Reference::new("stress_benchmark", "DEL-09-02"),
                affected_object: Reference::new(
                    "result_envelope",
                    "STRESS-TP-PHYS-015-SECTION-EVIDENCE-RESULT-ENVELOPE",
                ),
                message: "Invented validation-local result envelope records governed section-property evidence transport.".to_string(),
                remediation:
                    "Use project audit and runner contracts before treating this as public runtime behavior."
                        .to_string(),
                provenance: provenance.clone(),
            }],
            provenance,
            reproducibility: ReproducibilityRefs {
                model_hash: checksum("analytical-solver-model"),
                run_hashes: vec![checksum("section-evidence-stress-run")],
                audit_manifest_ref: Reference::new(
                    "task_run_record",
                    "TASK_RUN_2026-05-17_TP-SECTION-021A",
                ),
                deterministic_ordering: true,
            },
            analysis_status: vec![
                AnalysisStatus::MechanicsSolved,
                AnalysisStatus::HumanReviewRequired,
            ],
            rule_pack_refs: Vec::new(),
            professional_boundary: ProfessionalBoundary::project_default(),
        }
    }

    #[test]
    fn valid_envelope_has_no_blocking_diagnostics() {
        let validation = validate_result_envelope(&envelope());
        assert!(validation.diagnostics.is_empty());
    }

    #[test]
    fn missing_units_and_dimensions_are_blocking() {
        let mut envelope = envelope();
        envelope.result_sets[0].values[0].unit.clear();
        envelope.result_sets[0].values[0].dimension = DimensionId::Tbd;

        let validation = validate_result_envelope(&envelope);

        assert!(validation.has_blocking_diagnostics());
        assert!(validation
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "RESULT_EXPORT_VALUE_METADATA_INCOMPLETE"));
    }

    #[test]
    fn local_force_and_moment_metadata_is_mandatory() {
        let mut envelope = envelope();
        let force = envelope
            .result_sets
            .iter_mut()
            .flat_map(|set| set.values.iter_mut())
            .find(|value| value.result_id == "result:force:pipe-P-120:axial")
            .expect("fixture force result exists");
        force.metadata = None;

        let validation = validate_result_envelope(&envelope);

        assert!(validation.has_blocking_diagnostics());
        assert!(validation.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "RESULT_EXPORT_FORCE_MOMENT_METADATA_INCOMPLETE"
        }));
    }

    #[test]
    fn load_vector_and_station_resultant_set_types_are_governed_vocabulary() {
        let mut envelope = envelope();
        envelope.result_sets = vec![
            ResultSet {
                set_id: "set-load-vector".to_string(),
                set_type: ResultSetType::LoadVectorEvidence.as_str().to_string(),
                basis_ref: Reference::new("load_case", "LC1"),
                values: vec![QuantityResult {
                    result_id: "result:load-vector:node-N1:uy".to_string(),
                    family: ResultFamily::Force,
                    object_ref: Reference::new("solver_load_vector", "node-N1"),
                    basis_ref: Reference::new("load_case", "LC1"),
                    station_ref: None,
                    magnitude: -6.0,
                    unit: "N".to_string(),
                    dimension: DimensionId::Force,
                    metadata: Some(ResultMetadata {
                        component: "nodal_force_y".to_string(),
                        coordinate_system: "global".to_string(),
                        location: "node".to_string(),
                        basis: "assembled_solver_load_vector".to_string(),
                        sign_convention: "positive follows the solver global UY force entry"
                            .to_string(),
                    }),
                    diagnostics: vec![Diagnostic {
                        code: "TP_RESULT_017_VALUE_TRACE_NOTE".to_string(),
                        class: DiagnosticClass::AssumptionWarning,
                        severity: DiagnosticSeverity::Info,
                        source: Reference::new("result_export_fixture", "TP-RESULT-017"),
                        affected_object: Reference::new(
                            "result_value",
                            "result:load-vector:node-N1:uy",
                        ),
                        message:
                            "Fixture records per-value diagnostics without changing solver behavior."
                                .to_string(),
                        remediation:
                            "Use accepted trace-chain ruling before adding multi-hop value trace fields."
                                .to_string(),
                        provenance: provenance(),
                    }],
                    trace_chain: Vec::new(),
                    provenance: provenance(),
                }],
            },
            ResultSet {
                set_id: "set-station-resultants".to_string(),
                set_type: ResultSetType::StationResultants.as_str().to_string(),
                basis_ref: Reference::new("load_case", "LC1"),
                values: vec![QuantityResult {
                    result_id: "result:moment:element-E1:midspan:bending-z".to_string(),
                    family: ResultFamily::Moment,
                    object_ref: Reference::new("element", "E1"),
                    basis_ref: Reference::new("load_case", "LC1"),
                    station_ref: Some(Reference::new("result_station", "E1:midspan")),
                    magnitude: 4.0,
                    unit: "N-m".to_string(),
                    dimension: DimensionId::Moment,
                    metadata: Some(ResultMetadata {
                        component: "bending_moment_z".to_string(),
                        coordinate_system: "element_local".to_string(),
                        location: "midspan".to_string(),
                        basis: "interpolated_from_endpoint_resultants".to_string(),
                        sign_convention:
                            "positive follows the element-local z bending resultant convention"
                                .to_string(),
                    }),
                    diagnostics: Vec::new(),
                    trace_chain: Vec::new(),
                    provenance: provenance(),
                }],
            },
            ResultSet {
                set_id: "set-section-property-evidence".to_string(),
                set_type: ResultSetType::SectionPropertyEvidence.as_str().to_string(),
                basis_ref: Reference::new(
                    "section_property_evidence",
                    "SECTION-PROP-TP-STRESS-016-INVENTED-PIPE-OD2-WALL0P25",
                ),
                values: vec![section_property_quantity(
                    "result:section-property:tp-stress-016:section-modulus-z",
                    "section_modulus_z",
                    0.5368932757599744,
                    "m^3",
                    DimensionId::SectionModulus,
                )],
            },
        ];

        let validation = validate_result_envelope(&envelope);

        assert!(validation.diagnostics.is_empty());
    }

    #[test]
    fn unsupported_result_set_type_is_blocking() {
        let mut envelope = envelope();
        envelope.result_sets[0].set_type = "mechanics_solve_result_boundary".to_string();

        let validation = validate_result_envelope(&envelope);

        assert!(validation.has_blocking_diagnostics());
        assert!(validation
            .diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.code == "RESULT_EXPORT_RESULT_SET_TYPE_UNSUPPORTED" }));
    }

    #[test]
    fn tp_phys_015_fixture_serializes_to_schema_wrapper() {
        let envelope = tp_result_017_envelope();
        let validation = validate_result_envelope(&envelope);
        assert!(validation.diagnostics.is_empty());

        let serialized = result_export_document(&envelope);
        let fixture: Value = serde_json::from_str(TP_RESULT_017_SERIALIZED_FIXTURE)
            .expect("serialized fixture is valid JSON");

        assert_eq!(serialized, fixture);
        assert_eq!(serialized["deliverable_id"], "DEL-08-04");
        assert_eq!(
            serialized["result_envelope"]["solver_version"]["solver_name"],
            "open_pipe_stress_validation_mechanics_benchmark"
        );
        assert_eq!(
            serialized["result_envelope"]["result_sets"][1]["set_type"],
            ResultSetType::LoadVectorEvidence.as_str()
        );
        assert_eq!(
            serialized["result_envelope"]["result_sets"][1]["values"][0]["diagnostics"][0]["code"],
            "TP_RESULT_017_VALUE_TRACE_NOTE"
        );
        assert_eq!(
            serialized["result_envelope"]["result_sets"][1]["values"][0]["trace_chain"][0]
                ["source_ref"]["ref_id"],
            "dto:load_application:LC-TP-PHYS-014:0"
        );
        assert_eq!(
            serialized["result_envelope"]["result_sets"][1]["values"][0]["trace_chain"][0]
                ["target_ref"]["ref_id"],
            "result:load-vector:node-N-1:uy"
        );
    }

    #[test]
    fn tp_section_021_fixture_serializes_section_property_evidence() {
        let envelope = tp_section_021_envelope();
        let validation = validate_result_envelope(&envelope);
        assert!(validation.diagnostics.is_empty());

        let serialized = result_export_document(&envelope);
        let fixture: Value = serde_json::from_str(TP_SECTION_021_SERIALIZED_FIXTURE)
            .expect("section-property evidence fixture is valid JSON");

        assert_eq!(serialized, fixture);
        assert_eq!(
            serialized["result_envelope"]["result_sets"][0]["set_type"],
            ResultSetType::SectionPropertyEvidence.as_str()
        );
        assert_eq!(
            serialized["result_envelope"]["result_sets"][0]["values"][1]["dimension"],
            "section_modulus"
        );
        assert_eq!(
            serialized["result_envelope"]["result_sets"][1]["values"][0]["trace_chain"][0]
                ["trace_type"],
            "section_property_evidence_to_result_value"
        );
    }

    #[test]
    fn human_review_required_status_is_mandatory() {
        let mut envelope = envelope();
        envelope.analysis_status = vec![AnalysisStatus::MechanicsSolved];

        let validation = validate_result_envelope(&envelope);

        assert!(validation.has_blocking_diagnostics());
        assert!(validation
            .diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.code == "RESULT_EXPORT_HUMAN_REVIEW_STATUS_MISSING" }));
    }

    #[test]
    fn professional_boundary_violation_is_blocking() {
        let mut envelope = envelope();
        envelope
            .professional_boundary
            .software_makes_compliance_claim = true;

        let validation = validate_result_envelope(&envelope);

        assert!(validation.has_blocking_diagnostics());
        assert!(validation.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "RESULT_EXPORT_PROFESSIONAL_BOUNDARY_VIOLATION"
        }));
    }

    #[test]
    fn rule_pack_payload_must_remain_redacted() {
        let mut envelope = envelope();
        envelope.rule_pack_refs[0].private_payload_redacted = false;

        let validation = validate_result_envelope(&envelope);

        assert!(validation.has_blocking_diagnostics());
        assert!(validation
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "RESULT_EXPORT_RULE_PACK_REF_INCOMPLETE"));
    }

    #[test]
    fn protected_rule_pack_reference_blocks_export() {
        let mut envelope = envelope();
        envelope.rule_pack_refs[0].redistribution_status = RedistributionStatus::ProtectedSuspected;

        let validation = validate_result_envelope(&envelope);

        assert!(validation.has_blocking_diagnostics());
        assert!(validation.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "RESULT_EXPORT_PROTECTED_RULE_PACK_SUSPECTED"
        }));
    }

    #[test]
    fn result_values_sort_deterministically_for_regression_comparison() {
        let envelope = envelope();
        let ids: Vec<_> = sorted_result_values(&envelope)
            .into_iter()
            .map(|value| value.result_id.as_str())
            .collect();

        assert_eq!(
            ids,
            vec![
                "disp-1",
                "result:force:pipe-P-120:axial",
                "reaction-1",
                "stress-1"
            ]
        );
    }
}
