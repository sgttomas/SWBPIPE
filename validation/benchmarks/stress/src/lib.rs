//! Original stress recovery verification benchmarks for OpenPipeStress.
//!
//! The fixtures in this crate use elementary open mechanics with invented
//! numeric values. They are verification aids only: no code-specific stress
//! equations, protected standards content, allowables, fatigue acceptance
//! criteria, or professional approval claims are encoded here.

use open_pipe_stress_frame_kernel::{FrameNode, DOF_PER_NODE, ELEMENT_DOF, RX, RZ, UX, UY};
#[cfg(test)]
use open_pipe_stress_result_export::sorted_result_values;
use open_pipe_stress_result_export::{
    validate_result_envelope, AnalysisStatus as ExportAnalysisStatus, ChecksumRef,
    Diagnostic as ExportDiagnostic, DiagnosticClass as ExportDiagnosticClass,
    DiagnosticSeverity as ExportDiagnosticSeverity, DimensionId as ExportDimensionId,
    ProfessionalBoundary, Provenance as ExportProvenance, QuantityResult,
    RedistributionStatus as ExportRedistributionStatus, Reference, ReproducibilityRefs,
    ResultEnvelope, ResultFamily as ExportResultFamily, ResultSet, ResultSetType, ResultTraceLink,
};
use open_pipe_stress_straight_pipe::{
    LocalLoadDirection, PipeEnd, PipeEndResultants, PointLocalForce, SpannedUniformLocalLoad,
    StationResultants, StraightPipeAxialEffect, StraightPipeElement, StraightPipeSectionProperties,
    UniformLoadSpan, UniformLocalLoad,
};
use open_pipe_stress_stress_recovery::{
    recover_station_stress_sweep, recover_station_stresses, recover_stress_range,
    recover_stress_range_with_modulus_basis, recover_stresses, AnalysisStatus, ForceResultants,
    PressureBasis, StationStressRecoveryInput, StationStressRecoveryResult,
    StressRangeModulusBasisRecord, StressRangeResult, StressRecoveryInput, StressRecoveryResult,
    StressSectionProperties,
};
use sha2::{Digest, Sha256};

// Ungated (value unchanged) so `recorded_comparison_holds` can expose the
// crate's already-encoded comparison basis outside `#[cfg(test)]`.
const INTERNAL_ASSERTION_EPSILON: f64 = 1.0e-9;
const PKG09_STRESS_FIXTURE_UNIT_SYSTEM_REF: &str = "PKG09-STRESS-FIXTURE-UNITS-EXPLICIT-N-M-PA";
const TP_STRESS_016_SECTION_EVIDENCE_ID: &str =
    "SECTION-PROP-TP-STRESS-016-INVENTED-PIPE-OD2-WALL0P25";
const CANONICAL_DIMENSIONS: &[&str] = &[
    "dimensionless",
    "length",
    "mass",
    "time",
    "temperature",
    "temperature_interval",
    "angle",
    "rotation",
    "force",
    "force_per_length",
    "moment",
    "pressure",
    "stress",
    "area",
    "volume",
    "density",
    "linear_stiffness",
    "rotational_stiffness",
    "displacement",
    "velocity",
    "acceleration",
    "thermal_conductivity",
    "specific_heat",
    "thermal_expansion_coefficient",
    "second_moment_area",
    "section_modulus",
    "mass_per_length",
    "volume_per_length",
    "slope",
    "TBD",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StressBenchmarkFamily {
    AxialNormal,
    BendingNormal,
    TorsionalShear,
    PressureMembrane,
    StressRange,
    IntegratedStraightPipeStress,
    LoadToResultantStress,
    OrientedLoadToStress,
    PartialSpanLoadToStress,
    StationSweepStress,
    ThermalAxialEffectToStress,
    CombinedAxialBendingToStress,
    CanonicalAnalyticalResultantStress,
    MillToleranceEffectiveWallStress,
    ModulusBasisStressRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedistributionStatus {
    PublicOriginal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewDisposition {
    AcceptedInventedFixture,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BenchmarkProvenance {
    pub source_name: &'static str,
    pub source_location: &'static str,
    pub source_license: &'static str,
    pub contributor: &'static str,
    pub contributor_certification: &'static str,
    pub redistribution_status: RedistributionStatus,
    pub review_disposition: ReviewDisposition,
}

impl BenchmarkProvenance {
    pub fn public_original(source_location: &'static str) -> Self {
        Self {
            source_name: "OpenPipeStress original stress recovery benchmark",
            source_location,
            source_license: "project-original-public-content",
            contributor: "OpenPipeStress agentic development workflow",
            contributor_certification:
                "Generated from elementary open mechanics; not copied from protected standards, code formulas, commercial software examples, or proprietary data.",
            redistribution_status: RedistributionStatus::PublicOriginal,
            review_disposition: ReviewDisposition::AcceptedInventedFixture,
        }
    }

    pub fn is_publicly_usable(&self) -> bool {
        self.redistribution_status == RedistributionStatus::PublicOriginal
            && self.review_disposition == ReviewDisposition::AcceptedInventedFixture
            && !self.source_name.is_empty()
            && !self.source_location.is_empty()
            && !self.source_license.is_empty()
            && self
                .contributor_certification
                .contains("not copied from protected standards")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixtureUnitBasis {
    pub unit_system_ref: &'static str,
    pub unit_system_status: &'static str,
    pub force_unit: &'static str,
    pub moment_unit: &'static str,
    pub pressure_unit: &'static str,
    pub stress_unit: &'static str,
    pub length_unit: &'static str,
    pub area_unit: &'static str,
    pub section_modulus_unit: &'static str,
    pub second_moment_area_unit: &'static str,
    pub note: &'static str,
}

impl FixtureUnitBasis {
    pub fn is_explicit_fixture_basis(&self) -> bool {
        self.unit_system_ref == PKG09_STRESS_FIXTURE_UNIT_SYSTEM_REF
            && self.unit_system_status == "fixture-local-explicit-units-no-conversions"
            && !self.force_unit.is_empty()
            && !self.moment_unit.is_empty()
            && !self.pressure_unit.is_empty()
            && !self.stress_unit.is_empty()
            && !self.section_modulus_unit.is_empty()
            && self.note.contains("unit catalog remains TBD")
    }
}

pub const STRESS_FIXTURE_UNIT_BASIS: FixtureUnitBasis = FixtureUnitBasis {
    unit_system_ref: PKG09_STRESS_FIXTURE_UNIT_SYSTEM_REF,
    unit_system_status: "fixture-local-explicit-units-no-conversions",
    force_unit: "N",
    moment_unit: "N-m",
    pressure_unit: "Pa",
    stress_unit: "Pa",
    length_unit: "m",
    area_unit: "m^2",
    section_modulus_unit: "m^3",
    second_moment_area_unit: "m^4",
    note: "Explicit fixture-local unit identifiers only; project unit catalog remains TBD and no conversion constants are encoded.",
};

#[derive(Debug, Clone, PartialEq)]
pub struct ExpectedValue {
    pub name: &'static str,
    pub value: f64,
    pub unit: &'static str,
    pub dimension: &'static str,
    pub tolerance_policy: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StressBenchmark {
    pub fixture_id: &'static str,
    pub family: StressBenchmarkFamily,
    pub description: &'static str,
    pub assumptions: &'static [&'static str],
    pub provenance: BenchmarkProvenance,
    pub unit_basis: FixtureUnitBasis,
    pub expected_values: Vec<ExpectedValue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StressBenchmarkReadinessBoundary {
    pub final_tolerance_policy: &'static str,
    pub release_thresholds: &'static str,
    pub ci_gate_policy: &'static str,
    pub result_envelope_export_integration: &'static str,
    pub benchmark_publication_scope: &'static str,
    pub canonical_unit_conversion_policy: &'static str,
    pub professional_reliance: &'static str,
}

impl StressBenchmarkReadinessBoundary {
    pub fn unresolved_items(&self) -> [&'static str; 7] {
        [
            self.final_tolerance_policy,
            self.release_thresholds,
            self.ci_gate_policy,
            self.result_envelope_export_integration,
            self.benchmark_publication_scope,
            self.canonical_unit_conversion_policy,
            self.professional_reliance,
        ]
    }

    pub fn remains_tbd(&self) -> bool {
        self.unresolved_items().iter().all(|item| *item == "TBD")
    }

    pub fn release_authority_remains_tbd(&self) -> bool {
        [
            self.final_tolerance_policy,
            self.release_thresholds,
            self.ci_gate_policy,
            self.benchmark_publication_scope,
            self.canonical_unit_conversion_policy,
            self.professional_reliance,
        ]
        .iter()
        .all(|item| *item == "TBD")
    }
}

pub const STRESS_BENCHMARK_READINESS_BOUNDARY: StressBenchmarkReadinessBoundary =
    StressBenchmarkReadinessBoundary {
        final_tolerance_policy: "TBD",
        release_thresholds: "TBD",
        ci_gate_policy: "TBD",
        result_envelope_export_integration:
            "bounded_governed_result_envelope_witness_verification_only",
        benchmark_publication_scope: "TBD",
        canonical_unit_conversion_policy: "TBD",
        professional_reliance: "TBD",
    };

#[derive(Debug, Clone, PartialEq)]
pub struct GovernedStressBenchmarkEnvelopeEvidence {
    pub envelope: ResultEnvelope,
    pub quantity_result_count: usize,
    pub export_validation_diagnostic_count: usize,
}

/// Route one rights-safe stress benchmark result through the governed
/// diagnostics/result-envelope seam. This remains verification-local evidence;
/// it does not select release thresholds or assert engineering validation.
pub fn governed_complete_stress_result_envelope() -> GovernedStressBenchmarkEnvelopeEvidence {
    let recovered = recover_complete_fixture();
    let provenance = governed_stress_result_provenance();
    let basis_ref = Reference::new("load_case", "LC-STRESS-COMPLETE-INVENTED");
    let result_source_ref = Reference::new(
        "stress_recovery_result",
        "STRESS-COMPLETE-INVENTED-GOVERNED-RECOVERY",
    );
    let components = [
        ("axial-normal", recovered.components.axial_normal),
        ("bending-normal-y", recovered.components.bending_normal_y),
        ("bending-normal-z", recovered.components.bending_normal_z),
        ("torsional-shear", recovered.components.torsional_shear),
        ("pressure-hoop", recovered.components.pressure_hoop),
        (
            "pressure-longitudinal",
            recovered.components.pressure_longitudinal,
        ),
    ];
    let values = components
        .into_iter()
        .filter_map(|(component, magnitude)| {
            magnitude.map(|magnitude| {
                governed_stress_quantity(
                    component,
                    magnitude,
                    &basis_ref,
                    &result_source_ref,
                    &provenance,
                )
            })
        })
        .collect::<Vec<_>>();
    let quantity_result_count = values.len();
    let envelope_id = "STRESS-BENCHMARK-COMPLETE-GOVERNED-RESULT-ENVELOPE";
    let envelope = ResultEnvelope {
        envelope_id: envelope_id.to_string(),
        schema_version: "0.1.0".to_string(),
        model_ref: Reference::new("analytical_solver_model", "STRESS-COMPLETE-INVENTED-MODEL"),
        run_ref: Reference::new("analysis_run", "RUN-STRESS-COMPLETE-INVENTED"),
        solver_name: "open_pipe_stress_stress_recovery".to_string(),
        solver_version: "0.1.0".to_string(),
        solver_build_ref: "validation-local-cargo-test".to_string(),
        unit_system_ref: Reference::new("unit_system", PKG09_STRESS_FIXTURE_UNIT_SYSTEM_REF),
        load_basis_refs: vec![basis_ref.clone()],
        result_sets: vec![ResultSet {
            set_id: "result-set:stress-complete-invented".to_string(),
            set_type: ResultSetType::StressRecovery.as_str().to_string(),
            basis_ref,
            values,
        }],
        diagnostics: vec![ExportDiagnostic {
            code: "STRESS_BENCHMARK_RESULT_ENVELOPE_BOUNDARY".to_string(),
            class: ExportDiagnosticClass::AssumptionWarning,
            severity: ExportDiagnosticSeverity::Info,
            source: Reference::new("stress_benchmark", "DEL-09-02"),
            affected_object: Reference::new("result_envelope", envelope_id),
            message: "Rights-safe benchmark output is bound to the governed result-envelope vocabulary for verification only.".to_string(),
            remediation: "Do not infer release readiness, engineering validation, or a selected tolerance policy from this bounded witness.".to_string(),
            provenance: provenance.clone(),
        }],
        provenance,
        reproducibility: ReproducibilityRefs {
            model_hash: governed_stress_checksum(
                "analytical_solver_model:STRESS-COMPLETE-INVENTED-MODEL",
                format!("{:?}", complete_stress_input()).as_bytes(),
            ),
            run_hashes: vec![governed_stress_checksum(
                "analysis_run:RUN-STRESS-COMPLETE-INVENTED",
                format!("{recovered:?}").as_bytes(),
            )],
            audit_manifest_ref: Reference::new(
                "task_run_record",
                "WORKING_ITEMS_RUN_2026-07-12_D41-R5-T4-PDU039",
            ),
            deterministic_ordering: true,
        },
        analysis_status: vec![
            ExportAnalysisStatus::MechanicsSolved,
            ExportAnalysisStatus::HumanReviewRequired,
        ],
        rule_pack_refs: Vec::new(),
        professional_boundary: ProfessionalBoundary::project_default(),
    };
    let validation = validate_result_envelope(&envelope);
    GovernedStressBenchmarkEnvelopeEvidence {
        envelope,
        quantity_result_count,
        export_validation_diagnostic_count: validation.diagnostics.len(),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntegratedStraightPipeStressResult {
    pub end_resultants: PipeEndResultants,
    pub stress: StressRecoveryResult,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadToResultantStressResult {
    pub station_resultants: StationResultants,
    pub station_stress: StationStressRecoveryResult,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GovernedStressSectionEvidence {
    pub evidence_id: &'static str,
    pub owner_deliverable: &'static str,
    pub calculator_ref: &'static str,
    pub source_input_ref: &'static str,
    pub outside_diameter: f64,
    pub wall_thickness: f64,
    pub area: f64,
    pub section_modulus_y: f64,
    pub section_modulus_z: f64,
    pub torsion_constant: f64,
    pub torsion_radius: f64,
}

impl GovernedStressSectionEvidence {
    pub fn stress_section_properties(&self) -> StressSectionProperties {
        StressSectionProperties::new(
            Some(self.area),
            Some(self.section_modulus_y),
            Some(self.section_modulus_z),
            Some(self.torsion_constant),
            Some(self.torsion_radius),
        )
    }

    pub fn is_governed_section_property_evidence(&self) -> bool {
        self.evidence_id == TP_STRESS_016_SECTION_EVIDENCE_ID
            && self.owner_deliverable == "DEL-03-08"
            && self.calculator_ref == "core/section_properties/calculator.py"
            && self.area.is_finite()
            && self.section_modulus_y.is_finite()
            && self.section_modulus_z.is_finite()
            && self.torsion_constant.is_finite()
            && self.torsion_radius.is_finite()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StationSweepStressResult {
    pub station_resultants: Vec<StationResultants>,
    pub station_stresses: Vec<StationStressRecoveryResult>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThermalAxialEffectStressResult {
    pub end_resultants: PipeEndResultants,
    pub end_stress: StressRecoveryResult,
    pub station_resultants: StationResultants,
    pub station_stress: StationStressRecoveryResult,
    pub station_sweep_resultants: Vec<StationResultants>,
    pub station_sweep_stresses: Vec<StationStressRecoveryResult>,
}

impl StressBenchmark {
    pub fn tolerance_policy_is_unresolved(&self) -> bool {
        self.expected_values
            .iter()
            .all(|value| value.tolerance_policy.is_none())
    }

    pub fn has_dimensioned_expected_values(&self) -> bool {
        self.expected_values.iter().all(|value| {
            value.value.is_finite()
                && !value.unit.is_empty()
                && CANONICAL_DIMENSIONS.contains(&value.dimension)
        }) && self.unit_basis.is_explicit_fixture_basis()
    }
}

pub fn fixture_inventory() -> Vec<StressBenchmark> {
    vec![
        axial_normal_fixture(),
        bending_normal_fixture(),
        torsional_shear_fixture(),
        pressure_membrane_fixture(),
        stress_range_fixture(),
        integrated_straight_pipe_stress_fixture(),
        tp_phys_004_load_to_resultant_stress_fixture(),
        tp_phys_005_oriented_load_to_stress_fixture(),
        tp_phys_006_partial_span_load_to_stress_fixture(),
        tp_phys_007_station_sweep_stress_fixture(),
        tp_phys_008_thermal_axial_effect_to_stress_fixture(),
        tp_phys_009_combined_axial_bending_to_stress_fixture(),
        tp_phys_015_canonical_resultant_stress_fixture(),
        tp_pmm_p3_milltol_effective_wall_stress_fixture(),
        tp_pmm_p3_modulusbasis_range_stress_fixture(),
    ]
}

/// Additive accessor exposing this crate's already-encoded internal assertion
/// comparison basis (`INTERNAL_ASSERTION_EPSILON`, the same predicate used by
/// this crate's own fixture verification via `assert_close`) so a caller such
/// as the DEL-10-05 headless runner can reuse the recorded comparison basis
/// without re-encoding a tolerance. This is not a release threshold, tolerance
/// policy, or acceptance criterion; those remain `TBD` pending human approval.
pub fn recorded_comparison_holds(observed: f64, recorded: f64) -> bool {
    observed.is_finite()
        && recorded.is_finite()
        && (observed - recorded).abs() <= INTERNAL_ASSERTION_EPSILON
}

pub fn missing_required_families(fixtures: &[StressBenchmark]) -> Vec<StressBenchmarkFamily> {
    let required = [
        StressBenchmarkFamily::AxialNormal,
        StressBenchmarkFamily::BendingNormal,
        StressBenchmarkFamily::TorsionalShear,
        StressBenchmarkFamily::PressureMembrane,
        StressBenchmarkFamily::StressRange,
        StressBenchmarkFamily::IntegratedStraightPipeStress,
        StressBenchmarkFamily::LoadToResultantStress,
        StressBenchmarkFamily::OrientedLoadToStress,
        StressBenchmarkFamily::PartialSpanLoadToStress,
        StressBenchmarkFamily::StationSweepStress,
        StressBenchmarkFamily::ThermalAxialEffectToStress,
        StressBenchmarkFamily::CombinedAxialBendingToStress,
        StressBenchmarkFamily::CanonicalAnalyticalResultantStress,
        StressBenchmarkFamily::MillToleranceEffectiveWallStress,
        StressBenchmarkFamily::ModulusBasisStressRange,
    ];

    required
        .into_iter()
        .filter(|family| !fixtures.iter().any(|fixture| fixture.family == *family))
        .collect()
}

pub fn axial_normal_fixture() -> StressBenchmark {
    StressBenchmark {
        fixture_id: "STRESS-AXIAL-NORMAL-ORIGINAL",
        family: StressBenchmarkFamily::AxialNormal,
        description: "Invented axial resultant divided by invented section area.",
        assumptions: &[
            "Positive axial force is tensile in this fixture.",
            "Area is supplied explicitly by a governed section-property boundary.",
            "The expected value is a mechanics stress component, not an allowable comparison.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/stress/axial_normal.md",
        ),
        unit_basis: STRESS_FIXTURE_UNIT_BASIS,
        expected_values: vec![ExpectedValue {
            name: "axial_normal",
            value: 120.0 / 12.0,
            unit: "Pa",
            dimension: "stress",
            tolerance_policy: None,
        }],
    }
}

pub fn bending_normal_fixture() -> StressBenchmark {
    StressBenchmark {
        fixture_id: "STRESS-BENDING-NORMAL-ORIGINAL",
        family: StressBenchmarkFamily::BendingNormal,
        description: "Invented bending moments divided by invented section moduli.",
        assumptions: &[
            "Section moduli are supplied explicitly and remain positive.",
            "The signs of bending components follow the current stress-recovery API inputs.",
            "The fixture does not encode a code stress category or stress index.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/stress/bending_normal.md",
        ),
        unit_basis: STRESS_FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "bending_normal_y",
                value: 50.0 / 25.0,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "bending_normal_z",
                value: -30.0 / 15.0,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn torsional_shear_fixture() -> StressBenchmark {
    StressBenchmark {
        fixture_id: "STRESS-TORSIONAL-SHEAR-ORIGINAL",
        family: StressBenchmarkFamily::TorsionalShear,
        description: "Invented torque times radius divided by invented torsion constant.",
        assumptions: &[
            "Torsion radius and torsion constant are supplied explicitly.",
            "The expected value is a shear stress component.",
            "The fixture does not encode a code allowable or fatigue criterion.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/stress/torsional_shear.md",
        ),
        unit_basis: STRESS_FIXTURE_UNIT_BASIS,
        expected_values: vec![ExpectedValue {
            name: "torsional_shear",
            value: 40.0 * 2.0 / 80.0,
            unit: "Pa",
            dimension: "stress",
            tolerance_policy: None,
        }],
    }
}

pub fn pressure_membrane_fixture() -> StressBenchmark {
    StressBenchmark {
        fixture_id: "STRESS-PRESSURE-MEMBRANE-ORIGINAL",
        family: StressBenchmarkFamily::PressureMembrane,
        description: "Invented thin-wall pressure membrane components from explicit pressure basis inputs.",
        assumptions: &[
            "Pressure, membrane radius, and wall thickness are explicit fixture inputs.",
            "Hoop and longitudinal membrane components follow the upstream stress-recovery mechanics boundary.",
            "The fixture does not provide pressure design criteria or code equations.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/stress/pressure_membrane.md",
        ),
        unit_basis: STRESS_FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "pressure_hoop",
                value: 100.0 * 3.0 / 0.5,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "pressure_longitudinal",
                value: (100.0 * 3.0 / 0.5) / 2.0,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn stress_range_fixture() -> StressBenchmark {
    StressBenchmark {
        fixture_id: "STRESS-RANGE-MECHANICS-ORIGINAL",
        family: StressBenchmarkFamily::StressRange,
        description: "Invented mechanics-only range between two recovered stress states.",
        assumptions: &[
            "Stress range is computed as absolute component difference between two mechanics states.",
            "The fixture is not a fatigue assessment, allowable comparison, or code compliance check.",
            "The same section properties are used for both invented states.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/stress/stress_range.md",
        ),
        unit_basis: STRESS_FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "axial_normal_range",
                value: (180.0_f64 / 12.0 - 60.0 / 12.0).abs(),
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "bending_normal_y_range",
                value: (80.0_f64 / 25.0 - (-20.0 / 25.0)).abs(),
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "bending_normal_z_range",
                value: (10.0_f64 / 15.0 - 10.0 / 15.0).abs(),
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "torsional_shear_range",
                value: (60.0_f64 * 2.0 / 80.0 - 20.0 * 2.0 / 80.0).abs(),
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn integrated_straight_pipe_stress_fixture() -> StressBenchmark {
    let result = recover_integrated_straight_pipe_stress_fixture();

    StressBenchmark {
        fixture_id: "STRESS-INTEGRATED-STRAIGHT-PIPE-ORIGINAL",
        family: StressBenchmarkFamily::IntegratedStraightPipeStress,
        description: "Invented straight-pipe element end-resultants feeding code-neutral stress recovery.",
        assumptions: &[
            "A two-node straight pipe element supplies local end-j resultants directly from the solver boundary.",
            "The stress helper maps direct end-resultants to mechanics force resultants without code interpretation.",
            "The fixture is not an allowable comparison, rule check, fatigue check, or professional conclusion.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/stress/integrated_straight_pipe_resultants.md",
        ),
        unit_basis: STRESS_FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "end_j_axial_force",
                value: result.end_resultants.axial_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "end_j_torsional_moment",
                value: result.end_resultants.torsional_moment,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "axial_normal",
                value: result.stress.components.axial_normal.unwrap(),
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "torsional_shear",
                value: result.stress.components.torsional_shear.unwrap(),
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn tp_phys_004_load_to_resultant_stress_fixture() -> StressBenchmark {
    let result = recover_tp_phys_004_load_to_resultant_stress_fixture();

    StressBenchmark {
        fixture_id: "STRESS-TP-PHYS-004-LOAD-TO-RESULTANT",
        family: StressBenchmarkFamily::LoadToResultantStress,
        description: "Invented straight-pipe load-to-station-resultant-to-stress recovery fixture.",
        assumptions: &[
            "The station resultants are recovered from explicit straight-pipe loads and fixture hand-calculated displacements.",
            "Only axial, bending, and torsional mechanics components are recovered.",
            "No allowable, ratio, rule-pack, fatigue, or professional conclusion is encoded.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/stress/tp_phys_004_load_to_resultant_stress.md",
        ),
        unit_basis: STRESS_FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "midspan_bending_z",
                value: 4.0,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "bending_normal_z",
                value: result
                    .station_stress
                    .stress
                    .components
                    .bending_normal_z
                    .unwrap(),
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn tp_phys_005_oriented_load_to_stress_fixture() -> StressBenchmark {
    StressBenchmark {
        fixture_id: "STRESS-TP-PHYS-005-ORIENTED-LOAD-TO-STRESS",
        family: StressBenchmarkFamily::OrientedLoadToStress,
        description: "Invented oriented straight-pipe global-model station-resultant-to-stress recovery fixture.",
        assumptions: &[
            "The pipe local x axis is aligned to global Y and local y is set by explicit y_reference [1, 0, 0].",
            "Station resultants are recovered from a global-model displacement vector and local transverse loads.",
            "No allowable, ratio, rule-pack, fatigue, or professional conclusion is encoded.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/stress/tp_phys_005_oriented_load_to_stress.md",
        ),
        unit_basis: STRESS_FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "midspan_bending_z",
                value: 4.0,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "bending_normal_z",
                value: 2.0,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "axial_normal",
                value: 0.0,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn tp_phys_006_partial_span_load_to_stress_fixture() -> StressBenchmark {
    StressBenchmark {
        fixture_id: "STRESS-TP-PHYS-006-PARTIAL-SPAN-LOAD-TO-STRESS",
        family: StressBenchmarkFamily::PartialSpanLoadToStress,
        description: "Invented straight-pipe partial-span distributed-load station-resultant-to-stress recovery fixture.",
        assumptions: &[
            "The local Y distributed load is active only over the normalized span [0.25, 0.75].",
            "Station resultants are recovered at midspan through the spanned straight-pipe load path.",
            "No allowable, ratio, rule-pack, fatigue, or professional conclusion is encoded.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/stress/tp_phys_006_partial_span_load_to_stress.md",
        ),
        unit_basis: STRESS_FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "midspan_bending_z",
                value: 1.0,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "bending_normal_z",
                value: 0.5,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "axial_normal",
                value: 0.0,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn tp_phys_007_station_sweep_stress_fixture() -> StressBenchmark {
    StressBenchmark {
        fixture_id: "STRESS-TP-PHYS-007-STATION-SWEEP-STRESS",
        family: StressBenchmarkFamily::StationSweepStress,
        description: "Invented ordered station-resultant sweep feeding ordered mechanics-only station stress recovery.",
        assumptions: &[
            "Station fractions are intentionally unsorted to verify caller-order preservation.",
            "The station-resultant sweep uses explicit I-end resultants and one spanned local Y load.",
            "The station stress sweep uses explicit invented section properties with no pressure or rule-pack interpretation.",
            "No allowable, ratio, fatigue, compliance, or professional conclusion is encoded.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/stress/tp_phys_007_station_sweep_stress.md",
        ),
        unit_basis: STRESS_FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "station_0_fraction",
                value: 0.75,
                unit: "ratio",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_0_shear_y",
                value: 0.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_0_bending_z",
                value: 0.0,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_0_bending_normal_z",
                value: 0.0,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_1_fraction",
                value: 0.25,
                unit: "ratio",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_1_shear_y",
                value: 4.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_1_bending_z",
                value: 4.0,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_1_bending_normal_z",
                value: 2.0,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_2_fraction",
                value: 0.5,
                unit: "ratio",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_2_shear_y",
                value: 2.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_2_bending_z",
                value: 1.0,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_2_bending_normal_z",
                value: 0.5,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_3_fraction",
                value: 1.0,
                unit: "ratio",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_3_shear_y",
                value: 0.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_3_bending_z",
                value: 0.0,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_3_bending_normal_z",
                value: 0.0,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn tp_phys_008_thermal_axial_effect_to_stress_fixture() -> StressBenchmark {
    StressBenchmark {
        fixture_id: "STRESS-TP-PHYS-008-THERMAL-AXIAL-EFFECT-TO-STRESS",
        family: StressBenchmarkFamily::ThermalAxialEffectToStress,
        description: "Invented straight-pipe thermal axial-effect resultant recovery feeding mechanics-only stress recovery.",
        assumptions: &[
            "A single invented thermal axial effect is supplied as an explicit axial force input.",
            "Zero displacement evidence isolates the axial-effect recovery path from mechanical displacement strain.",
            "Recovered end and station resultants feed stress recovery without pressure-basis or rule-pack interpretation.",
            "No acceptance threshold, ratio, material default, or thermal-expansion default is encoded.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/stress/tp_phys_008_thermal_axial_effect_to_stress.md",
        ),
        unit_basis: STRESS_FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "thermal_axial_effect_force",
                value: 240.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "end_j_axial_force",
                value: -240.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "end_j_axial_normal",
                value: -40.0,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_midspan_axial_force",
                value: 240.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_midspan_axial_normal",
                value: 40.0,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_sweep_count",
                value: 3.0,
                unit: "count",
                dimension: "dimensionless",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn tp_phys_009_combined_axial_bending_to_stress_fixture() -> StressBenchmark {
    let result = recover_tp_phys_009_combined_axial_bending_to_stress_fixture();

    StressBenchmark {
        fixture_id: "STRESS-TP-PHYS-009-COMBINED-AXIAL-BENDING-TO-STRESS",
        family: StressBenchmarkFamily::CombinedAxialBendingToStress,
        description:
            "Invented straight-pipe axial-effect and bending-resultant station stress recovery fixture.",
        assumptions: &[
            "The axial force comes from an explicit straight-pipe axial-effect resultant recovery path.",
            "The station bending resultant comes from explicit open mechanics I-end resultants and a local Y line load.",
            "Recovered station resultants feed stress recovery without pressure-basis or rule-pack interpretation.",
            "No acceptance threshold, ratio, material default, allowable, or professional conclusion is encoded.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/stress/tp_phys_009_combined_axial_bending_to_stress.md",
        ),
        unit_basis: STRESS_FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "axial_effect_force",
                value: 120.0,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_midspan_axial_force",
                value: result.station_resultants.axial_force,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_midspan_bending_z",
                value: result.station_resultants.bending_moment_z,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_midspan_axial_normal",
                value: result
                    .station_stress
                    .stress
                    .components
                    .axial_normal
                    .unwrap(),
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_midspan_bending_normal_z",
                value: result
                    .station_stress
                    .stress
                    .components
                    .bending_normal_z
                    .unwrap(),
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn tp_phys_015_canonical_resultant_stress_fixture() -> StressBenchmark {
    let result = recover_tp_phys_015_canonical_resultant_stress_fixture()
        .expect("TP-PHYS-015 fixture construction must remain valid");

    StressBenchmark {
        fixture_id: "STRESS-TP-PHYS-015-CANONICAL-RESULTANT-STRESS-RECOVERY",
        family: StressBenchmarkFamily::CanonicalAnalyticalResultantStress,
        description:
            "Canonical TP-PHYS-014 analytical payload midspan resultants feeding mechanics-only station stress recovery.",
        assumptions: &[
            "The station resultants come from the governed TP-PHYS-014 analytical_solver_model payload solver path.",
            "Stress section inputs reference governed DEL-03-08 section-property calculation evidence; no section modulus is silently derived inside stress recovery.",
            "Recovered stress components are mechanics quantities only, with no rule checks, allowables, or professional conclusion.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/stress/tp_phys_015_canonical_resultant_stress.md",
        ),
        unit_basis: STRESS_FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "canonical_midspan_shear_y",
                value: result.station_resultants.shear_force_y,
                unit: "N",
                dimension: "force",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "canonical_midspan_bending_z",
                value: result.station_resultants.bending_moment_z,
                unit: "N-m",
                dimension: "moment",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_midspan_axial_normal",
                value: result
                    .station_stress
                    .stress
                    .components
                    .axial_normal
                    .unwrap(),
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_midspan_bending_normal_z",
                value: result
                    .station_stress
                    .stress
                    .components
                    .bending_normal_z
                    .unwrap(),
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_midspan_bending_normal_y",
                value: result
                    .station_stress
                    .stress
                    .components
                    .bending_normal_y
                    .unwrap(),
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "station_midspan_torsional_shear",
                value: result
                    .station_stress
                    .stress
                    .components
                    .torsional_shear
                    .unwrap(),
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn complete_stress_input() -> StressRecoveryInput {
    StressRecoveryInput {
        resultants: ForceResultants::new(Some(120.0), Some(50.0), Some(-30.0), Some(40.0)),
        section: benchmark_section(),
        pressure: Some(PressureBasis::new(Some(100.0), Some(3.0), Some(0.5))),
        statuses: vec![AnalysisStatus::MechanicsSolved],
    }
}

pub fn benchmark_section() -> StressSectionProperties {
    StressSectionProperties::new(Some(12.0), Some(25.0), Some(15.0), Some(80.0), Some(2.0))
}

pub fn recover_complete_fixture() -> StressRecoveryResult {
    recover_stresses(&complete_stress_input())
}

pub fn recover_range_start() -> StressRecoveryResult {
    let input = StressRecoveryInput {
        resultants: ForceResultants::new(Some(60.0), Some(-20.0), Some(10.0), Some(20.0)),
        section: benchmark_section(),
        pressure: None,
        statuses: vec![AnalysisStatus::MechanicsSolved],
    };
    recover_stresses(&input)
}

pub fn recover_range_end() -> StressRecoveryResult {
    let input = StressRecoveryInput {
        resultants: ForceResultants::new(Some(180.0), Some(80.0), Some(10.0), Some(60.0)),
        section: benchmark_section(),
        pressure: None,
        statuses: vec![AnalysisStatus::MechanicsSolved],
    };
    recover_stresses(&input)
}

pub fn recover_range_fixture() -> StressRangeResult {
    let start = StressRecoveryInput {
        resultants: ForceResultants::new(Some(60.0), Some(-20.0), Some(10.0), Some(20.0)),
        section: benchmark_section(),
        pressure: None,
        statuses: vec![AnalysisStatus::MechanicsSolved],
    };
    let end = StressRecoveryInput {
        resultants: ForceResultants::new(Some(180.0), Some(80.0), Some(10.0), Some(60.0)),
        section: benchmark_section(),
        pressure: None,
        statuses: vec![AnalysisStatus::MechanicsSolved],
    };
    recover_stress_range(&start, &end)
}

pub fn recover_integrated_straight_pipe_stress_fixture() -> IntegratedStraightPipeStressResult {
    let pipe = StraightPipeElement::new(
        "stress-pipe-1",
        FrameNode::new(0, [0.0, 0.0, 0.0]).expect("fixture node is valid"),
        FrameNode::new(1, [5.0, 0.0, 0.0]).expect("fixture node is valid"),
        StraightPipeSectionProperties::new(2000.0, 800.0, 4.0, 10.0, 12.0, 2.0, None)
            .expect("fixture section is valid"),
        [0.0, 1.0, 0.0],
    )
    .expect("fixture pipe is valid");
    let mut displacements = [0.0; ELEMENT_DOF];
    displacements[DOF_PER_NODE + UX] = 0.01;
    displacements[DOF_PER_NODE + RX] = 0.02;
    let end_resultants = pipe
        .recover_end_resultants(&displacements, PipeEnd::J)
        .expect("fixture end-resultant recovery is valid");
    let stress_input = StressRecoveryInput {
        resultants: ForceResultants::from_element_end_resultants(
            end_resultants.axial_force,
            end_resultants.bending_moment_y,
            end_resultants.bending_moment_z,
            end_resultants.torsional_moment,
        )
        .expect("fixture resultants are finite"),
        section: StressSectionProperties::new(
            Some(4.0),
            Some(20.0),
            Some(25.0),
            Some(2.0),
            Some(0.5),
        ),
        pressure: None,
        statuses: vec![AnalysisStatus::MechanicsSolved],
    };
    IntegratedStraightPipeStressResult {
        end_resultants,
        stress: recover_stresses(&stress_input),
    }
}

pub fn recover_tp_phys_004_load_to_resultant_stress_fixture() -> LoadToResultantStressResult {
    let pipe = StraightPipeElement::new(
        "stress-tp-phys-004-pipe",
        FrameNode::new(0, [0.0, 0.0, 0.0]).expect("fixture node is valid"),
        FrameNode::new(1, [4.0, 0.0, 0.0]).expect("fixture node is valid"),
        StraightPipeSectionProperties::new(1000.0, 400.0, 3.0, 1.5, 2.0, 1.0, None)
            .expect("fixture section is valid"),
        [0.0, 1.0, 0.0],
    )
    .expect("fixture pipe is valid");
    let mut displacements = [0.0; ELEMENT_DOF];
    displacements[DOF_PER_NODE + UY] = -0.04533333333333334;
    displacements[DOF_PER_NODE + RZ] = -0.014666666666666668;
    let station = pipe
        .recover_station_resultants(
            &displacements,
            0.5,
            &[UniformLocalLoad::new(LocalLoadDirection::Y, -2.0)
                .expect("fixture line load is finite")],
            &[PointLocalForce::new(0.5, LocalLoadDirection::Y, -4.0)
                .expect("fixture point force is finite")],
        )
        .expect("fixture station resultants are valid");
    let input = StationStressRecoveryInput::from_station_resultants(
        "station:midspan",
        &station,
        StressSectionProperties::new(Some(3.0), Some(2.5), Some(2.0), Some(1.0), Some(0.5)),
        None,
        vec![AnalysisStatus::MechanicsSolved],
    )
    .expect("fixture station stress input is valid");

    LoadToResultantStressResult {
        station_resultants: station,
        station_stress: recover_station_stresses(&input),
    }
}

pub fn recover_tp_phys_005_oriented_load_to_stress_fixture() -> LoadToResultantStressResult {
    let pipe = StraightPipeElement::new(
        "stress-tp-phys-005-oriented-pipe",
        FrameNode::new(0, [0.0, 0.0, 0.0]).expect("fixture node is valid"),
        FrameNode::new(1, [0.0, 4.0, 0.0]).expect("fixture node is valid"),
        StraightPipeSectionProperties::new(1000.0, 400.0, 3.0, 1.5, 2.0, 1.0, None)
            .expect("fixture section is valid"),
        [1.0, 0.0, 0.0],
    )
    .expect("fixture pipe is valid");
    let mut global_model_displacements = vec![0.0; 2 * DOF_PER_NODE];
    global_model_displacements[DOF_PER_NODE + UX] = -0.04533333333333334;
    global_model_displacements[DOF_PER_NODE + RZ] = 0.014666666666666668;
    let station = pipe
        .recover_station_resultants_from_global_model(
            &global_model_displacements,
            0.5,
            &[UniformLocalLoad::new(LocalLoadDirection::Y, -2.0)
                .expect("fixture line load is finite")],
            &[PointLocalForce::new(0.5, LocalLoadDirection::Y, -4.0)
                .expect("fixture point force is finite")],
        )
        .expect("fixture station resultants are valid");
    let input = StationStressRecoveryInput::from_station_resultants(
        "station:tp-phys-005:midspan",
        &station,
        StressSectionProperties::new(Some(3.0), Some(2.5), Some(2.0), Some(1.0), Some(0.5)),
        None,
        vec![AnalysisStatus::MechanicsSolved],
    )
    .expect("fixture station stress input is valid");

    LoadToResultantStressResult {
        station_resultants: station,
        station_stress: recover_station_stresses(&input),
    }
}

pub fn recover_tp_phys_006_partial_span_load_to_stress_fixture() -> LoadToResultantStressResult {
    let pipe = StraightPipeElement::new(
        "stress-tp-phys-006-partial-span-pipe",
        FrameNode::new(0, [0.0, 0.0, 0.0]).expect("fixture node is valid"),
        FrameNode::new(1, [4.0, 0.0, 0.0]).expect("fixture node is valid"),
        StraightPipeSectionProperties::new(1000.0, 400.0, 3.0, 1.5, 2.0, 1.0, None)
            .expect("fixture section is valid"),
        [0.0, 1.0, 0.0],
    )
    .expect("fixture pipe is valid");
    let mut displacements = [0.0; ELEMENT_DOF];
    displacements[DOF_PER_NODE + UY] = -0.014;
    displacements[DOF_PER_NODE + RZ] = -0.004333333333333333;
    let partial_span_load = SpannedUniformLocalLoad::new(
        LocalLoadDirection::Y,
        -2.0,
        UniformLoadSpan::new(0.25, 0.75).expect("fixture load span is valid"),
    )
    .expect("fixture line load is finite");
    let station = pipe
        .recover_station_resultants_with_spans(&displacements, 0.5, &[partial_span_load], &[])
        .expect("fixture station resultants are valid");
    let input = StationStressRecoveryInput::from_station_resultants(
        "station:tp-phys-006:midspan",
        &station,
        StressSectionProperties::new(Some(3.0), Some(2.5), Some(2.0), Some(1.0), Some(0.5)),
        None,
        vec![AnalysisStatus::MechanicsSolved],
    )
    .expect("fixture station stress input is valid");

    LoadToResultantStressResult {
        station_resultants: station,
        station_stress: recover_station_stresses(&input),
    }
}

pub fn recover_tp_phys_007_station_sweep_stress_fixture() -> StationSweepStressResult {
    let pipe = StraightPipeElement::new(
        "stress-tp-phys-007-station-sweep-pipe",
        FrameNode::new(0, [0.0, 0.0, 0.0]).expect("fixture node is valid"),
        FrameNode::new(1, [4.0, 0.0, 0.0]).expect("fixture node is valid"),
        StraightPipeSectionProperties::new(1000.0, 400.0, 3.0, 1.5, 2.0, 1.0, None)
            .expect("fixture section is valid"),
        [0.0, 1.0, 0.0],
    )
    .expect("fixture pipe is valid");
    let i_end = PipeEndResultants {
        end: PipeEnd::I,
        axial_force: 0.0,
        shear_force_y: 4.0,
        shear_force_z: 0.0,
        torsional_moment: 0.0,
        bending_moment_y: 0.0,
        bending_moment_z: 8.0,
    };
    let station_fractions = [0.75, 0.25, 0.5, 1.0];
    let spanned_load = SpannedUniformLocalLoad::new(
        LocalLoadDirection::Y,
        -2.0,
        UniformLoadSpan::new(0.25, 0.75).expect("fixture load span is valid"),
    )
    .expect("fixture line load is finite");
    let station_resultants = pipe
        .station_resultant_sweep_from_i_end_with_spans(
            i_end,
            &station_fractions,
            &[spanned_load],
            &[],
        )
        .expect("fixture station resultant sweep is valid");
    let station_stresses = recover_station_stress_sweep(
        "station:tp-phys-007",
        &station_resultants,
        StressSectionProperties::new(Some(3.0), Some(2.5), Some(2.0), Some(1.0), Some(0.5)),
        None,
        vec![AnalysisStatus::MechanicsSolved],
    )
    .expect("fixture station stress sweep is valid");

    StationSweepStressResult {
        station_resultants,
        station_stresses,
    }
}

pub fn recover_tp_phys_008_thermal_axial_effect_to_stress_fixture() -> ThermalAxialEffectStressResult
{
    let pipe = StraightPipeElement::new(
        "stress-tp-phys-008-thermal-axial-pipe",
        FrameNode::new(0, [0.0, 0.0, 0.0]).expect("fixture node is valid"),
        FrameNode::new(1, [6.0, 0.0, 0.0]).expect("fixture node is valid"),
        StraightPipeSectionProperties::new(1200.0, 500.0, 6.0, 2.0, 2.0, 1.0, None)
            .expect("fixture section is valid"),
        [0.0, 1.0, 0.0],
    )
    .expect("fixture pipe is valid");
    let thermal_axial_effect =
        StraightPipeAxialEffect::new(240.0).expect("fixture axial effect is finite");
    let section =
        StressSectionProperties::new(Some(6.0), Some(3.0), Some(3.0), Some(1.0), Some(0.5));

    let end_resultants = pipe
        .recover_end_resultants_with_axial_effects(
            &[0.0; ELEMENT_DOF],
            PipeEnd::J,
            &[thermal_axial_effect],
        )
        .expect("fixture end resultants are valid");
    let end_input = StressRecoveryInput {
        resultants: ForceResultants::from_element_end_resultants(
            end_resultants.axial_force,
            end_resultants.bending_moment_y,
            end_resultants.bending_moment_z,
            end_resultants.torsional_moment,
        )
        .expect("fixture end resultants are finite"),
        section: section.clone(),
        pressure: None,
        statuses: vec![AnalysisStatus::MechanicsSolved],
    };

    let global_model_displacements = vec![0.0; 2 * DOF_PER_NODE];
    let station_resultants = pipe
        .recover_station_resultants_from_global_model_with_axial_effects(
            &global_model_displacements,
            0.5,
            &[thermal_axial_effect],
        )
        .expect("fixture station resultants are valid");
    let station_input = StationStressRecoveryInput::from_station_resultants(
        "station:tp-phys-008:midspan",
        &station_resultants,
        section.clone(),
        None,
        vec![AnalysisStatus::MechanicsSolved],
    )
    .expect("fixture station stress input is valid");

    let station_sweep_resultants = pipe
        .recover_station_resultant_sweep_from_global_model_with_axial_effects(
            &global_model_displacements,
            &[0.0, 0.5, 1.0],
            &[thermal_axial_effect],
        )
        .expect("fixture station resultant sweep is valid");
    let station_sweep_stresses = recover_station_stress_sweep(
        "station:tp-phys-008:sweep",
        &station_sweep_resultants,
        section,
        None,
        vec![AnalysisStatus::MechanicsSolved],
    )
    .expect("fixture station stress sweep is valid");

    ThermalAxialEffectStressResult {
        end_resultants,
        end_stress: recover_stresses(&end_input),
        station_resultants,
        station_stress: recover_station_stresses(&station_input),
        station_sweep_resultants,
        station_sweep_stresses,
    }
}

pub fn recover_tp_phys_009_combined_axial_bending_to_stress_fixture() -> LoadToResultantStressResult
{
    let pipe = StraightPipeElement::new(
        "stress-tp-phys-009-combined-axial-bending-pipe",
        FrameNode::new(0, [0.0, 0.0, 0.0]).expect("fixture node is valid"),
        FrameNode::new(1, [4.0, 0.0, 0.0]).expect("fixture node is valid"),
        StraightPipeSectionProperties::new(1000.0, 400.0, 6.0, 1.5, 2.0, 1.0, None)
            .expect("fixture section is valid"),
        [0.0, 1.0, 0.0],
    )
    .expect("fixture pipe is valid");
    let axial_effect = StraightPipeAxialEffect::new(120.0).expect("fixture axial effect is finite");
    let axial_recovered = pipe
        .recover_local_forces_with_axial_effects(&[0.0; ELEMENT_DOF], &[axial_effect])
        .expect("fixture axial-effect local forces are valid");
    let axial_i_end =
        PipeEndResultants::from_local_forces(&axial_recovered.local_forces, PipeEnd::I);
    let combined_i_end = PipeEndResultants {
        end: PipeEnd::I,
        axial_force: axial_i_end.axial_force,
        shear_force_y: 4.0,
        shear_force_z: 0.0,
        torsional_moment: 0.0,
        bending_moment_y: 0.0,
        bending_moment_z: 8.0,
    };
    let station = pipe
        .station_resultants_from_i_end(
            combined_i_end,
            0.5,
            &[UniformLocalLoad::new(LocalLoadDirection::Y, -2.0)
                .expect("fixture line load is finite")],
            &[],
        )
        .expect("fixture station resultants are valid");
    let input = StationStressRecoveryInput::from_station_resultants(
        "station:tp-phys-009:midspan",
        &station,
        StressSectionProperties::new(Some(6.0), Some(3.0), Some(2.0), Some(1.0), Some(0.5)),
        None,
        vec![AnalysisStatus::MechanicsSolved],
    )
    .expect("fixture station stress input is valid");

    LoadToResultantStressResult {
        station_resultants: station,
        station_stress: recover_station_stresses(&input),
    }
}

pub fn recover_tp_phys_015_canonical_resultant_stress_fixture(
) -> Result<LoadToResultantStressResult, String> {
    let station = StationResultants {
        station_fraction: 0.5,
        distance_from_i: 2.0,
        axial_force: 0.0,
        shear_force_y: 4.0,
        shear_force_z: 0.0,
        torsional_moment: 0.0,
        bending_moment_y: 0.0,
        bending_moment_z: 4.0,
    };
    let section_evidence = tp_stress_016_governed_section_evidence();
    let input = StationStressRecoveryInput::from_station_resultants(
        "station:tp-phys-015:canonical-midspan",
        &station,
        section_evidence.stress_section_properties(),
        None,
        vec![AnalysisStatus::MechanicsSolved],
    )
    .map_err(|error| error.to_string())?;

    Ok(LoadToResultantStressResult {
        station_resultants: station,
        station_stress: recover_station_stresses(&input),
    })
}

pub fn tp_stress_016_governed_section_evidence() -> GovernedStressSectionEvidence {
    GovernedStressSectionEvidence {
        evidence_id: TP_STRESS_016_SECTION_EVIDENCE_ID,
        owner_deliverable: "DEL-03-08",
        calculator_ref: "core/section_properties/calculator.py",
        source_input_ref: "pipe_section_input:invented:od-2p0-wall-0p25",
        outside_diameter: 2.0,
        wall_thickness: 0.25,
        area: 1.3744467859455345,
        section_modulus_y: 0.5368932757599744,
        section_modulus_z: 0.5368932757599744,
        torsion_constant: 1.0737865515199487,
        torsion_radius: 1.0,
    }
}

pub fn recover_asymmetric_pressure_range_fixture() -> StressRangeResult {
    let first = StressRecoveryInput {
        resultants: ForceResultants::new(Some(60.0), Some(-20.0), Some(10.0), Some(20.0)),
        section: benchmark_section(),
        pressure: None,
        statuses: vec![AnalysisStatus::MechanicsSolved],
    };
    let second = StressRecoveryInput {
        resultants: ForceResultants::new(Some(180.0), Some(80.0), Some(10.0), Some(60.0)),
        section: benchmark_section(),
        pressure: Some(PressureBasis::new(Some(100.0), Some(3.0), Some(0.5))),
        statuses: vec![AnalysisStatus::MechanicsSolved],
    };
    recover_stress_range(&first, &second)
}

// --- Mill-tolerance effective-wall stress fixture (TP-PMM-P3-MILLTOL-001) ---

const MILLTOL_OUTSIDE_DIAMETER: f64 = 0.2;
const MILLTOL_NOMINAL_WALL: f64 = 0.01;
const MILLTOL_CORROSION_ALLOWANCE: f64 = 0.002;
const MILLTOL_MILL_TOLERANCE: f64 = 0.00125;
const MILLTOL_AXIAL_FORCE: f64 = 5000.0;
const MILLTOL_BENDING_MOMENT_Y: f64 = 1000.0;
const MILLTOL_BENDING_MOMENT_Z: f64 = -400.0;
const MILLTOL_TORSIONAL_MOMENT: f64 = 250.0;
const MILLTOL_PRESSURE: f64 = 2000.0;

/// Effective wall per the mill-tolerance slot semantics: nominal wall minus
/// corrosion allowance minus the user-entered absolute mill tolerance.
pub fn milltol_effective_wall() -> f64 {
    MILLTOL_NOMINAL_WALL - MILLTOL_CORROSION_ALLOWANCE - MILLTOL_MILL_TOLERANCE
}

fn milltol_section_from_wall(effective_wall: f64) -> StressSectionProperties {
    let od = MILLTOL_OUTSIDE_DIAMETER;
    let id = od - 2.0 * effective_wall;
    let area = core::f64::consts::PI / 4.0 * (od.powi(2) - id.powi(2));
    let second_moment = core::f64::consts::PI / 64.0 * (od.powi(4) - id.powi(4));
    let section_modulus = second_moment / (od / 2.0);
    let torsion_constant = 2.0 * second_moment;
    StressSectionProperties::new(
        Some(area),
        Some(section_modulus),
        Some(section_modulus),
        Some(torsion_constant),
        Some(od / 2.0),
    )
}

pub fn milltol_effective_wall_section() -> StressSectionProperties {
    milltol_section_from_wall(milltol_effective_wall())
}

pub fn milltol_corrosion_only_section() -> StressSectionProperties {
    milltol_section_from_wall(MILLTOL_NOMINAL_WALL - MILLTOL_CORROSION_ALLOWANCE)
}

pub fn milltol_stress_input() -> StressRecoveryInput {
    StressRecoveryInput {
        resultants: ForceResultants::new(
            Some(MILLTOL_AXIAL_FORCE),
            Some(MILLTOL_BENDING_MOMENT_Y),
            Some(MILLTOL_BENDING_MOMENT_Z),
            Some(MILLTOL_TORSIONAL_MOMENT),
        ),
        section: milltol_effective_wall_section(),
        pressure: Some(PressureBasis::new(
            Some(MILLTOL_PRESSURE),
            Some((MILLTOL_OUTSIDE_DIAMETER - milltol_effective_wall()) / 2.0),
            Some(milltol_effective_wall()),
        )),
        statuses: vec![AnalysisStatus::MechanicsSolved],
    }
}

pub fn recover_milltol_fixture() -> StressRecoveryResult {
    recover_stresses(&milltol_stress_input())
}

pub fn tp_pmm_p3_milltol_effective_wall_stress_fixture() -> StressBenchmark {
    let section = milltol_effective_wall_section();
    let area = section.area.expect("fixture area is explicit");
    let section_modulus = section
        .section_modulus_y
        .expect("fixture section modulus is explicit");
    let torsion_constant = section
        .torsion_constant
        .expect("fixture torsion constant is explicit");
    let torsion_radius = section
        .torsion_radius
        .expect("fixture torsion radius is explicit");
    let membrane_radius = (MILLTOL_OUTSIDE_DIAMETER - milltol_effective_wall()) / 2.0;
    let hoop = MILLTOL_PRESSURE * membrane_radius / milltol_effective_wall();
    StressBenchmark {
        fixture_id: "STRESS-TP-PMM-P3-MILLTOL-EFFECTIVE-WALL-STRESS",
        family: StressBenchmarkFamily::MillToleranceEffectiveWallStress,
        description: "Invented resultants recovered over section properties derived from the user-entered mill-tolerance effective wall (nominal wall minus corrosion allowance minus mill tolerance).",
        assumptions: &[
            "Mill tolerance is a user-entered absolute thickness dimension; no fractional form, catalog value, or default is encoded.",
            "Absence of the mill-tolerance slot means no reduction; absence is not a default value of zero.",
            "Effective wall feeds area, section modulus, torsion constant, and the pressure membrane basis identically to the section-property calculator closed forms.",
            "The fixture does not encode a code stress category, stress index, or acceptance criterion.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/stress/tp_pmm_p3_milltol_effective_wall_stress.md",
        ),
        unit_basis: STRESS_FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "axial_normal",
                value: MILLTOL_AXIAL_FORCE / area,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "bending_normal_y",
                value: MILLTOL_BENDING_MOMENT_Y / section_modulus,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "bending_normal_z",
                value: MILLTOL_BENDING_MOMENT_Z / section_modulus,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "torsional_shear",
                value: MILLTOL_TORSIONAL_MOMENT * torsion_radius / torsion_constant,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "pressure_hoop",
                value: hoop,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "pressure_longitudinal",
                value: hoop / 2.0,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
        ],
    }
}

// --- Per-load-case modulus basis stress range fixture
// (TP-PMM-P3-MODULUSBASIS-001, DEC-068 item 1) ---

const MODULUSBASIS_HOT_ELASTIC_MODULUS: f64 = 1.8e11;
const MODULUSBASIS_HOT_EXPANSION_COEFFICIENT: f64 = 1.3e-5;
const MODULUSBASIS_INTERPOLATION_LOWER_TEMPERATURE: f64 = 300.0;
const MODULUSBASIS_INTERPOLATION_UPPER_TEMPERATURE: f64 = 500.0;
const MODULUSBASIS_INTERPOLATION_SOLVE_TEMPERATURE: f64 = 400.0;
const MODULUSBASIS_INTERPOLATION_LOWER_E: f64 = 2.0e11;
const MODULUSBASIS_INTERPOLATION_UPPER_E: f64 = 1.8e11;
const MODULUSBASIS_INTERPOLATION_LOWER_ALPHA: f64 = 1.2e-5;
const MODULUSBASIS_INTERPOLATION_UPPER_ALPHA: f64 = 1.4e-5;
const MODULUSBASIS_TEMPERATURE_CHANGE: f64 = 10.0;
const MODULUSBASIS_METAL_AREA: f64 = 0.004;
pub const MODULUSBASIS_HOT_BASIS_LABEL: &str = "temperature_point:hot";
pub const MODULUSBASIS_COLD_BASIS_LABEL: &str = "material_base_values";

/// Fixed-fixed uniform-temperature axial identity solved with the
/// user-entered hot temperature-point values (exact selection).
pub fn modulusbasis_hot_axial_force() -> f64 {
    MODULUSBASIS_HOT_ELASTIC_MODULUS
        * MODULUSBASIS_METAL_AREA
        * MODULUSBASIS_HOT_EXPANSION_COEFFICIENT
        * MODULUSBASIS_TEMPERATURE_CHANGE
}

fn modulusbasis_linear_interpolation(lower: f64, upper: f64) -> f64 {
    let fraction = (MODULUSBASIS_INTERPOLATION_SOLVE_TEMPERATURE
        - MODULUSBASIS_INTERPOLATION_LOWER_TEMPERATURE)
        / (MODULUSBASIS_INTERPOLATION_UPPER_TEMPERATURE
            - MODULUSBASIS_INTERPOLATION_LOWER_TEMPERATURE);
    lower + fraction * (upper - lower)
}

fn modulusbasis_section() -> StressSectionProperties {
    StressSectionProperties::new(
        Some(MODULUSBASIS_METAL_AREA),
        Some(1.0),
        Some(1.0),
        Some(1.0),
        Some(1.0),
    )
}

pub fn modulusbasis_hot_state_input() -> StressRecoveryInput {
    StressRecoveryInput {
        resultants: ForceResultants::new(
            Some(modulusbasis_hot_axial_force()),
            Some(0.0),
            Some(0.0),
            Some(0.0),
        ),
        section: modulusbasis_section(),
        pressure: None,
        statuses: vec![AnalysisStatus::MechanicsSolved],
    }
}

pub fn modulusbasis_cold_state_input() -> StressRecoveryInput {
    StressRecoveryInput {
        resultants: ForceResultants::new(Some(0.0), Some(0.0), Some(0.0), Some(0.0)),
        section: modulusbasis_section(),
        pressure: None,
        statuses: vec![AnalysisStatus::MechanicsSolved],
    }
}

pub fn recover_modulusbasis_range_fixture() -> StressRangeResult {
    recover_stress_range_with_modulus_basis(
        &modulusbasis_hot_state_input(),
        &modulusbasis_cold_state_input(),
        StressRangeModulusBasisRecord::new(
            MODULUSBASIS_HOT_BASIS_LABEL,
            MODULUSBASIS_COLD_BASIS_LABEL,
        )
        .expect("fixture basis labels are explicit"),
    )
}

pub fn tp_pmm_p3_modulusbasis_range_stress_fixture() -> StressBenchmark {
    let hot_axial_stress = modulusbasis_hot_axial_force() / MODULUSBASIS_METAL_AREA;
    StressBenchmark {
        fixture_id: "STRESS-TP-PMM-P3-MODULUSBASIS-RANGE-STRESS",
        family: StressBenchmarkFamily::ModulusBasisStressRange,
        description: "Hot-solve/cold-eval stress range plus DEC-077 hand-calculated midpoint interpolation of user-entered E and alpha; the range records both modulus bases explicitly.",
        assumptions: &[
            "All temperature-dependent property values are user-entered; no catalog, curve, or default is encoded.",
            "DEC-068 exact-id selection remains available; DEC-077 linear interpolation uses two adjacent user-entered points and never extrapolates.",
            "The range basis record is a verbatim declaration of the solved bases, not a selector.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/stress/tp_pmm_p3_modulusbasis_range_stress.md",
        ),
        unit_basis: STRESS_FIXTURE_UNIT_BASIS,
        expected_values: vec![
            ExpectedValue {
                name: "hot_axial_normal",
                value: hot_axial_stress,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "axial_normal_range",
                value: hot_axial_stress,
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "interpolated_elastic_modulus",
                value: modulusbasis_linear_interpolation(
                    MODULUSBASIS_INTERPOLATION_LOWER_E,
                    MODULUSBASIS_INTERPOLATION_UPPER_E,
                ),
                unit: "Pa",
                dimension: "stress",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "interpolated_thermal_expansion_coefficient",
                value: modulusbasis_linear_interpolation(
                    MODULUSBASIS_INTERPOLATION_LOWER_ALPHA,
                    MODULUSBASIS_INTERPOLATION_UPPER_ALPHA,
                ),
                unit: "1/K",
                dimension: "thermal_expansion_coefficient",
                tolerance_policy: None,
            },
        ],
    }
}

fn governed_stress_result_provenance() -> ExportProvenance {
    ExportProvenance {
        source_name: "OpenPipeStress original stress recovery benchmark".to_string(),
        source_location: "validation/benchmarks/stress/src/lib.rs".to_string(),
        source_license: "project-original-public-content".to_string(),
        contributor: "OpenPipeStress agentic development workflow".to_string(),
        contributor_certification: "Generated from elementary open mechanics; not copied from protected standards, commercial software examples, or proprietary data.".to_string(),
        redistribution_status: ExportRedistributionStatus::InventedNonEngineeringExample,
        review_status: "verification_only".to_string(),
    }
}

fn governed_stress_quantity(
    component: &str,
    magnitude: f64,
    basis_ref: &Reference,
    source_ref: &Reference,
    provenance: &ExportProvenance,
) -> QuantityResult {
    let result_id = format!("result:stress:element-E-STRESS-COMPLETE:{component}");
    QuantityResult {
        result_id: result_id.clone(),
        family: ExportResultFamily::Stress,
        object_ref: Reference::new("element", "E-STRESS-COMPLETE"),
        basis_ref: basis_ref.clone(),
        station_ref: None,
        magnitude,
        unit: "Pa".to_string(),
        dimension: ExportDimensionId::Stress,
        metadata: None,
        diagnostics: Vec::new(),
        trace_chain: vec![ResultTraceLink {
            trace_id: format!("trace:stress-complete:{component}:recovery-to-envelope"),
            trace_type: "governed_stress_recovery_to_result_envelope".to_string(),
            source_ref: source_ref.clone(),
            target_ref: Reference::new("result_value", result_id),
            provenance: provenance.clone(),
            diagnostics: Vec::new(),
        }],
        provenance: provenance.clone(),
    }
}

fn governed_stress_checksum(payload_id: &str, payload: &[u8]) -> ChecksumRef {
    let mut hasher = Sha256::new();
    hasher.update(payload);
    ChecksumRef {
        algorithm: "sha256".to_string(),
        canonicalization: "deterministic_rust_debug_fixture_bytes".to_string(),
        payload_ref: Reference::new("verification_payload", payload_id),
        value: format!("{:x}", hasher.finalize()),
    }
}

#[cfg(test)]
fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= INTERNAL_ASSERTION_EPSILON,
        "expected {expected}, got {actual}"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_pipe_stress_stress_recovery::FindingCode;

    #[test]
    fn inventory_covers_required_stress_families() {
        let fixtures = fixture_inventory();
        assert!(missing_required_families(&fixtures).is_empty());
        assert_eq!(fixtures.len(), 15);
        assert!(fixtures.iter().any(|fixture| {
            fixture.fixture_id == "STRESS-TP-PHYS-007-STATION-SWEEP-STRESS"
                && fixture.family == StressBenchmarkFamily::StationSweepStress
        }));
        assert!(fixtures.iter().any(|fixture| {
            fixture.fixture_id == "STRESS-TP-PHYS-008-THERMAL-AXIAL-EFFECT-TO-STRESS"
                && fixture.family == StressBenchmarkFamily::ThermalAxialEffectToStress
        }));
        assert!(fixtures.iter().any(|fixture| {
            fixture.fixture_id == "STRESS-TP-PHYS-009-COMBINED-AXIAL-BENDING-TO-STRESS"
                && fixture.family == StressBenchmarkFamily::CombinedAxialBendingToStress
        }));
        assert!(fixtures.iter().any(|fixture| {
            fixture.fixture_id == "STRESS-TP-PHYS-015-CANONICAL-RESULTANT-STRESS-RECOVERY"
                && fixture.family == StressBenchmarkFamily::CanonicalAnalyticalResultantStress
        }));
    }

    #[test]
    fn fixture_provenance_is_public_original_and_reviewed() {
        for fixture in fixture_inventory() {
            assert!(
                fixture.provenance.is_publicly_usable(),
                "{} lacks accepted public-original provenance",
                fixture.fixture_id
            );
            assert!(fixture.tolerance_policy_is_unresolved());
            assert!(fixture.has_dimensioned_expected_values());
        }
    }

    #[test]
    fn readiness_boundary_keeps_release_authority_unresolved() {
        assert!(!STRESS_BENCHMARK_READINESS_BOUNDARY.remains_tbd());
        assert!(STRESS_BENCHMARK_READINESS_BOUNDARY.release_authority_remains_tbd());
        assert_eq!(
            STRESS_BENCHMARK_READINESS_BOUNDARY.result_envelope_export_integration,
            "bounded_governed_result_envelope_witness_verification_only"
        );
    }

    #[test]
    fn readmes_match_fixture_inventory_and_unit_basis() {
        let benchmark_readme = include_str!("../README.md");
        let hand_calc_readme = include_str!("../../../hand_calcs/stress/README.md");

        assert!(benchmark_readme.contains(PKG09_STRESS_FIXTURE_UNIT_SYSTEM_REF));
        assert!(hand_calc_readme.contains(PKG09_STRESS_FIXTURE_UNIT_SYSTEM_REF));

        for fixture in fixture_inventory() {
            assert!(
                benchmark_readme.contains(fixture.fixture_id),
                "benchmark README missing {}",
                fixture.fixture_id
            );
            assert!(
                hand_calc_readme.contains(fixture.fixture_id),
                "hand-calc README missing {}",
                fixture.fixture_id
            );
            assert!(
                hand_calc_readme.contains(
                    fixture
                        .provenance
                        .source_location
                        .rsplit('/')
                        .next()
                        .expect("fixture source location has a file name")
                ),
                "hand-calc README missing source file {}",
                fixture.provenance.source_location
            );
        }
    }

    #[test]
    fn recovers_modulusbasis_range_fixture_with_recorded_bases() {
        let fixture = tp_pmm_p3_modulusbasis_range_stress_fixture();
        let expected = |name: &str| {
            fixture
                .expected_values
                .iter()
                .find(|value| value.name == name)
                .unwrap_or_else(|| panic!("missing expected value {name}"))
                .value
        };

        let hot = recover_stresses(&modulusbasis_hot_state_input());
        assert!(!hot.is_blocked());
        assert_close(
            hot.components.axial_normal.unwrap(),
            expected("hot_axial_normal"),
        );

        let range = recover_modulusbasis_range_fixture();
        assert!(!range.is_blocked());
        assert_close(
            range.ranges.axial_normal_range.unwrap(),
            expected("axial_normal_range"),
        );
        let record = range.modulus_basis.as_ref().expect("recorded basis");
        assert_eq!(record.first_state_basis_ref, MODULUSBASIS_HOT_BASIS_LABEL);
        assert_eq!(record.second_state_basis_ref, MODULUSBASIS_COLD_BASIS_LABEL);

        let unrecorded = recover_stress_range(
            &modulusbasis_hot_state_input(),
            &modulusbasis_cold_state_input(),
        );
        assert!(unrecorded.modulus_basis.is_none());
        assert_eq!(unrecorded.ranges, range.ranges);
        assert_close(expected("interpolated_elastic_modulus"), 1.9e11);
        assert_close(
            expected("interpolated_thermal_expansion_coefficient"),
            1.3e-5,
        );
    }

    #[test]
    fn recovers_milltol_effective_wall_fixture() {
        let fixture = tp_pmm_p3_milltol_effective_wall_stress_fixture();
        let result = recover_milltol_fixture();

        assert!(!result.is_blocked());
        let components = &result.components;
        let expected = |name: &str| {
            fixture
                .expected_values
                .iter()
                .find(|value| value.name == name)
                .unwrap_or_else(|| panic!("missing expected value {name}"))
                .value
        };
        assert_close(components.axial_normal.unwrap(), expected("axial_normal"));
        assert_close(
            components.bending_normal_y.unwrap(),
            expected("bending_normal_y"),
        );
        assert_close(
            components.bending_normal_z.unwrap(),
            expected("bending_normal_z"),
        );
        assert_close(
            components.torsional_shear.unwrap(),
            expected("torsional_shear"),
        );
        assert_close(components.pressure_hoop.unwrap(), expected("pressure_hoop"));
        assert_close(
            components.pressure_longitudinal.unwrap(),
            expected("pressure_longitudinal"),
        );
    }

    #[test]
    fn milltol_reduction_strictly_reduces_section_modulus() {
        let with_mill = milltol_effective_wall_section();
        let corrosion_only = milltol_corrosion_only_section();
        assert!(with_mill.section_modulus_y.unwrap() < corrosion_only.section_modulus_y.unwrap());
        assert!(with_mill.area.unwrap() < corrosion_only.area.unwrap());
    }

    #[test]
    fn recovers_axial_normal_fixture() {
        let fixture = axial_normal_fixture();
        let result = recover_complete_fixture();

        assert!(!result.is_blocked());
        assert_close(
            result.components.axial_normal.unwrap(),
            fixture.expected_values[0].value,
        );
    }

    #[test]
    fn recovers_bending_normal_fixture() {
        let fixture = bending_normal_fixture();
        let result = recover_complete_fixture();

        assert!(!result.is_blocked());
        assert_close(
            result.components.bending_normal_y.unwrap(),
            fixture.expected_values[0].value,
        );
        assert_close(
            result.components.bending_normal_z.unwrap(),
            fixture.expected_values[1].value,
        );
    }

    #[test]
    fn recovers_torsional_shear_fixture() {
        let fixture = torsional_shear_fixture();
        let result = recover_complete_fixture();

        assert!(!result.is_blocked());
        assert_close(
            result.components.torsional_shear.unwrap(),
            fixture.expected_values[0].value,
        );
    }

    #[test]
    fn recovers_pressure_membrane_fixture() {
        let fixture = pressure_membrane_fixture();
        let result = recover_complete_fixture();

        assert!(!result.is_blocked());
        assert_close(
            result.components.pressure_hoop.unwrap(),
            fixture.expected_values[0].value,
        );
        assert_close(
            result.components.pressure_longitudinal.unwrap(),
            fixture.expected_values[1].value,
        );
    }

    #[test]
    fn computes_mechanics_only_stress_range_fixture() {
        let fixture = stress_range_fixture();
        let range = recover_range_fixture();

        assert!(!range.is_blocked());
        assert_close(
            range.ranges.axial_normal_range.unwrap(),
            fixture.expected_values[0].value,
        );
        assert_close(
            range.ranges.bending_normal_y_range.unwrap(),
            fixture.expected_values[1].value,
        );
        assert_close(
            range.ranges.bending_normal_z_range.unwrap(),
            fixture.expected_values[2].value,
        );
        assert_close(
            range.ranges.torsional_shear_range.unwrap(),
            fixture.expected_values[3].value,
        );
        assert_eq!(range.ranges.pressure_hoop_range, None);
        assert_eq!(range.ranges.pressure_longitudinal_range, None);
    }

    #[test]
    fn stress_range_blocks_asymmetric_optional_pressure_components() {
        let range = recover_asymmetric_pressure_range_fixture();

        assert!(range.is_blocked());
        assert!(range.ranges.axial_normal_range.is_none());
        assert!(range.findings.iter().any(|finding| {
            finding.code == FindingCode::MissingResultant
                && finding.subject_id == "pressure_hoop_range"
        }));
        assert!(range.findings.iter().any(|finding| {
            finding.code == FindingCode::MissingResultant
                && finding.subject_id == "pressure_longitudinal_range"
        }));
    }

    #[test]
    fn integrated_straight_pipe_resultants_feed_stress_recovery_fixture() {
        let fixture = integrated_straight_pipe_stress_fixture();
        let result = recover_integrated_straight_pipe_stress_fixture();

        assert!(!result.stress.is_blocked());
        assert_eq!(result.end_resultants.end, PipeEnd::J);
        assert_close(
            result.end_resultants.axial_force,
            fixture.expected_values[0].value,
        );
        assert_close(
            result.end_resultants.torsional_moment,
            fixture.expected_values[1].value,
        );
        assert_close(
            result.stress.components.axial_normal.unwrap(),
            fixture.expected_values[2].value,
        );
        assert_close(
            result.stress.components.torsional_shear.unwrap(),
            fixture.expected_values[3].value,
        );
        assert!(result
            .stress
            .statuses
            .contains(&AnalysisStatus::HumanReviewRequired));
        assert!(!result
            .stress
            .statuses
            .contains(&AnalysisStatus::HumanApprovedForProject));
    }

    #[test]
    fn load_to_resultant_station_stress_fixture_recovers_bending_component() {
        let fixture = tp_phys_004_load_to_resultant_stress_fixture();
        let result = recover_tp_phys_004_load_to_resultant_stress_fixture();

        assert!(!result.station_stress.is_blocked());
        assert_eq!(result.station_stress.station_id, "station:midspan");
        assert_close(
            result.station_resultants.bending_moment_z,
            fixture.expected_values[0].value,
        );
        assert_close(
            result
                .station_stress
                .stress
                .components
                .bending_normal_z
                .unwrap(),
            fixture.expected_values[1].value,
        );
        assert_eq!(
            result.station_stress.stress.components.axial_normal,
            Some(0.0)
        );
    }

    #[test]
    fn oriented_load_to_stress_fixture_recovers_midspan_bending_from_global_model() {
        let fixture = tp_phys_005_oriented_load_to_stress_fixture();
        let result = recover_tp_phys_005_oriented_load_to_stress_fixture();

        assert!(!result.station_stress.is_blocked());
        assert_eq!(
            result.station_stress.station_id,
            "station:tp-phys-005:midspan"
        );
        assert_close(
            result.station_resultants.bending_moment_z,
            fixture.expected_values[0].value,
        );
        assert_close(
            result
                .station_stress
                .stress
                .components
                .bending_normal_z
                .unwrap(),
            fixture.expected_values[1].value,
        );
        assert_close(
            result
                .station_stress
                .stress
                .components
                .axial_normal
                .unwrap(),
            fixture.expected_values[2].value,
        );
        assert_eq!(
            result.station_stress.stress.components.axial_normal,
            Some(0.0)
        );
        assert!(result
            .station_stress
            .stress
            .statuses
            .contains(&AnalysisStatus::HumanReviewRequired));
        assert!(!result
            .station_stress
            .stress
            .statuses
            .contains(&AnalysisStatus::HumanApprovedForProject));
    }

    #[test]
    fn partial_span_load_to_stress_fixture_recovers_midspan_bending_from_spanned_load() {
        let fixture = tp_phys_006_partial_span_load_to_stress_fixture();
        let result = recover_tp_phys_006_partial_span_load_to_stress_fixture();

        assert!(!result.station_stress.is_blocked());
        assert_eq!(
            result.station_stress.station_id,
            "station:tp-phys-006:midspan"
        );
        assert_close(
            result.station_resultants.bending_moment_z,
            fixture.expected_values[0].value,
        );
        assert_close(
            result
                .station_stress
                .stress
                .components
                .bending_normal_z
                .unwrap(),
            fixture.expected_values[1].value,
        );
        assert_close(
            result
                .station_stress
                .stress
                .components
                .axial_normal
                .unwrap(),
            fixture.expected_values[2].value,
        );
        assert_eq!(
            result.station_stress.stress.components.axial_normal,
            Some(0.0)
        );
        assert!(result
            .station_stress
            .stress
            .statuses
            .contains(&AnalysisStatus::HumanReviewRequired));
        assert!(!result
            .station_stress
            .stress
            .statuses
            .contains(&AnalysisStatus::HumanApprovedForProject));
    }

    #[test]
    fn station_sweep_stress_fixture_preserves_order_and_recovers_stresses() {
        let result = recover_tp_phys_007_station_sweep_stress_fixture();

        assert_eq!(result.station_resultants.len(), 4);
        assert_eq!(result.station_stresses.len(), 4);

        let expected_fractions = [0.75, 0.25, 0.5, 1.0];
        let expected_shear_y = [0.0, 4.0, 2.0, 0.0];
        let expected_bending_z = [0.0, 4.0, 1.0, 0.0];
        let expected_bending_normal_z = [0.0, 2.0, 0.5, 0.0];

        for index in 0..result.station_resultants.len() {
            let station = &result.station_resultants[index];
            let stress = &result.station_stresses[index];

            assert_eq!(
                stress.station_id,
                format!("station:tp-phys-007:station:{index}")
            );
            assert_close(station.station_fraction, expected_fractions[index]);
            assert_close(stress.station_fraction, expected_fractions[index]);
            assert_close(station.shear_force_y, expected_shear_y[index]);
            assert_close(station.bending_moment_z, expected_bending_z[index]);
            assert_close(
                stress.stress.components.bending_normal_z.unwrap(),
                expected_bending_normal_z[index],
            );
            assert_eq!(stress.stress.components.axial_normal, Some(0.0));
            assert!(!stress.is_blocked());
            assert!(stress
                .stress
                .statuses
                .contains(&AnalysisStatus::HumanReviewRequired));
            assert!(!stress
                .stress
                .statuses
                .contains(&AnalysisStatus::HumanApprovedForProject));
        }
    }

    #[test]
    fn thermal_axial_effect_resultants_feed_mechanics_only_stress_recovery() {
        let fixture = tp_phys_008_thermal_axial_effect_to_stress_fixture();
        let result = recover_tp_phys_008_thermal_axial_effect_to_stress_fixture();

        assert!(!result.end_stress.is_blocked());
        assert!(!result.station_stress.is_blocked());
        assert_eq!(result.end_resultants.end, PipeEnd::J);
        assert_close(
            result.end_resultants.axial_force,
            fixture.expected_values[1].value,
        );
        assert_close(
            result.end_stress.components.axial_normal.unwrap(),
            fixture.expected_values[2].value,
        );
        assert_eq!(result.end_stress.components.pressure_hoop, None);
        assert_eq!(result.end_stress.components.pressure_longitudinal, None);

        assert_eq!(
            result.station_stress.station_id,
            "station:tp-phys-008:midspan"
        );
        assert_close(
            result.station_resultants.axial_force,
            fixture.expected_values[3].value,
        );
        assert_close(
            result
                .station_stress
                .stress
                .components
                .axial_normal
                .unwrap(),
            fixture.expected_values[4].value,
        );
        assert_eq!(result.station_stress.stress.components.pressure_hoop, None);
        assert_eq!(
            result
                .station_stress
                .stress
                .components
                .pressure_longitudinal,
            None
        );

        assert_eq!(
            result.station_sweep_resultants.len(),
            fixture.expected_values[5].value as usize
        );
        assert_eq!(result.station_sweep_stresses.len(), 3);
        let expected_fractions = [0.0, 0.5, 1.0];
        for index in 0..result.station_sweep_resultants.len() {
            let station = &result.station_sweep_resultants[index];
            let stress = &result.station_sweep_stresses[index];

            assert_close(station.station_fraction, expected_fractions[index]);
            assert_close(station.axial_force, 240.0);
            assert_close(stress.station_fraction, expected_fractions[index]);
            assert_close(stress.stress.components.axial_normal.unwrap(), 40.0);
            assert!(!stress.is_blocked());
        }
    }

    #[test]
    fn combined_axial_effect_and_bending_resultants_feed_station_stress_recovery() {
        let fixture = tp_phys_009_combined_axial_bending_to_stress_fixture();
        let result = recover_tp_phys_009_combined_axial_bending_to_stress_fixture();

        assert!(!result.station_stress.is_blocked());
        assert_eq!(
            result.station_stress.station_id,
            "station:tp-phys-009:midspan"
        );
        assert_close(
            result.station_resultants.axial_force,
            fixture.expected_values[1].value,
        );
        assert_close(
            result.station_resultants.bending_moment_z,
            fixture.expected_values[2].value,
        );
        assert_close(
            result
                .station_stress
                .stress
                .components
                .axial_normal
                .unwrap(),
            fixture.expected_values[3].value,
        );
        assert_close(
            result
                .station_stress
                .stress
                .components
                .bending_normal_z
                .unwrap(),
            fixture.expected_values[4].value,
        );
        assert_eq!(result.station_stress.stress.components.pressure_hoop, None);
        assert_eq!(
            result
                .station_stress
                .stress
                .components
                .pressure_longitudinal,
            None
        );
        assert!(result
            .station_stress
            .stress
            .statuses
            .contains(&AnalysisStatus::HumanReviewRequired));
        assert!(!result
            .station_stress
            .stress
            .statuses
            .contains(&AnalysisStatus::HumanApprovedForProject));
    }

    #[test]
    fn canonical_analytical_payload_resultants_feed_station_stress_recovery() {
        let fixture = tp_phys_015_canonical_resultant_stress_fixture();
        let result = recover_tp_phys_015_canonical_resultant_stress_fixture().unwrap();
        let section_evidence = tp_stress_016_governed_section_evidence();

        assert!(!result.station_stress.is_blocked());
        assert!(section_evidence.is_governed_section_property_evidence());
        assert_close(section_evidence.outside_diameter, 2.0);
        assert_close(section_evidence.wall_thickness, 0.25);
        assert_close(
            section_evidence.section_modulus_y,
            section_evidence.section_modulus_z,
        );
        assert_eq!(
            result.station_stress.station_id,
            "station:tp-phys-015:canonical-midspan"
        );
        assert_close(
            result.station_resultants.shear_force_y,
            fixture.expected_values[0].value,
        );
        assert_close(
            result.station_resultants.bending_moment_z,
            fixture.expected_values[1].value,
        );
        assert_close(
            result
                .station_stress
                .stress
                .components
                .axial_normal
                .unwrap(),
            fixture.expected_values[2].value,
        );
        assert_close(
            result
                .station_stress
                .stress
                .components
                .bending_normal_z
                .unwrap(),
            fixture.expected_values[3].value,
        );
        assert_close(
            result
                .station_stress
                .stress
                .components
                .bending_normal_y
                .unwrap(),
            fixture.expected_values[4].value,
        );
        assert_close(
            result
                .station_stress
                .stress
                .components
                .torsional_shear
                .unwrap(),
            fixture.expected_values[5].value,
        );
        assert_eq!(result.station_stress.stress.components.pressure_hoop, None);
        assert_eq!(
            result
                .station_stress
                .stress
                .components
                .pressure_longitudinal,
            None
        );
        assert!(result
            .station_stress
            .stress
            .statuses
            .contains(&AnalysisStatus::HumanReviewRequired));
        assert!(!result
            .station_stress
            .stress
            .statuses
            .contains(&AnalysisStatus::HumanApprovedForProject));
    }

    #[test]
    fn recovered_results_preserve_human_review_boundary() {
        let result = recover_complete_fixture();

        assert!(result.statuses.contains(&AnalysisStatus::MechanicsSolved));
        assert!(result
            .statuses
            .contains(&AnalysisStatus::HumanReviewRequired));
        assert!(!result
            .statuses
            .contains(&AnalysisStatus::HumanApprovedForProject));
        assert!(result.summary.is_some());
    }

    #[test]
    fn complete_benchmark_output_uses_governed_diagnostics_and_result_envelope() {
        let evidence = governed_complete_stress_result_envelope();
        let envelope = &evidence.envelope;

        assert_eq!(evidence.quantity_result_count, 6);
        assert_eq!(evidence.export_validation_diagnostic_count, 0);
        assert_eq!(envelope.result_sets[0].set_type, "stress_recovery");
        assert_eq!(envelope.diagnostics.len(), 1);
        assert_eq!(
            envelope.diagnostics[0].code,
            "STRESS_BENCHMARK_RESULT_ENVELOPE_BOUNDARY"
        );
        assert!(envelope
            .analysis_status
            .contains(&ExportAnalysisStatus::HumanReviewRequired));
        assert!(!envelope
            .analysis_status
            .contains(&ExportAnalysisStatus::UserRuleChecked));
        assert!(sorted_result_values(envelope).iter().all(|value| {
            value.trace_chain.len() == 1
                && value.trace_chain[0].trace_type == "governed_stress_recovery_to_result_envelope"
        }));
        assert!(envelope.professional_boundary.human_review_required);
        assert!(!envelope.professional_boundary.software_makes_approval_claim);
    }
}
