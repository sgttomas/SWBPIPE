//! Closed K-D5 intended-system residual and repaired-member remainder.
//!
//! The trusted ledger classification and source-enumerated manifests close all
//! formed loads before evaluation. Midpoints are re-formed at p128 from authored
//! operands, never taken from a formed ledger value. Every stiffness pass uses a
//! stack matrix. Exact row accumulators form rho, s and s2; split truncations and
//! load enclosure radii stay in psi rather than shifting a residual midpoint.
//!
//! D5C1 judges the first displacement correction. A second correction supplies
//! recovery-only inputs; contraction and the infinity-norm Neumann bound fail
//! closed with named details. The gate's own estimate and prepared matrix are
//! used. Both row and column sums bound the contribution, assembly and projection
//! differences. No member-bound input enters StructuralReport.
use super::retained::wide::{Wide2, WideArith, WideError};
use super::{binary_exponent, StructuralError, StructuralSystem};
use crate::connector::ObjectiveConnector;
use crate::tie_rod::TieRod;
use crate::exact_sum::{ExactAccumulator, SumError};
use crate::{element_dof_map, BeamTheory, FrameElement, DOF_PER_NODE, ELEMENT_DOF};
use std::borrow::Cow;

/// Precision of the re-formation (D1 §4.3.1 step 1).
pub const FORMATION_PRECISION: u32 = 128;
/// The protected relative criterion.
pub const FORMATION_CRITERION: f64 = 1e-9;
/// The trigger factor on |w| (D5C-4: 2, calibrated on the product path).
pub const FORMATION_FACTOR: f64 = 2.0;

/// Binary64 inputs of one realized curved bend (DEC-070 macro-element), as
/// the product's `CurvedBendMacroElement` holds them: the nodes, the user
/// radius R and the plane reference y (T4-U1). No binary64 centre, derived
/// radius or included angle enters; K-D5 forms φ and the chord at p from
/// (x_j − x_i, R, y_reference).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurvedFormation {
    pub node_i: usize,
    pub node_j: usize,
    pub coordinates_i: [f64; 3],
    pub coordinates_j: [f64; 3],
    pub radius: f64,
    pub y_reference: [f64; 3],
    pub elastic_modulus: f64,
    pub shear_modulus: f64,
    pub area: f64,
    pub second_moment: f64,
    pub torsion_constant: f64,
    pub in_plane_flexibility_factor: f64,
    pub out_of_plane_flexibility_factor: f64,
}

/// Stable source-enumerated identity of one summand of a formed ledger term.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LoadSummandIdentity {
    pub family: String,
    pub source: String,
    pub ordinal: usize,
}

/// Expected closure binding, enumerated by the producer from its complete sources.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadClosureEntry {
    pub identity: LoadSummandIdentity,
    pub operand_bits: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThermalPointPrimitive {
    pub temperature_kelvin: f64,
    pub value: f64,
}
#[derive(Debug, Clone, PartialEq)]
pub enum CoefficientPrimitive {
    Constant(f64),
    Table(Vec<ThermalPointPrimitive>),
}
#[derive(Debug, Clone, PartialEq)]
pub enum ExpansionLawPrimitive {
    EngineeringSecant {
        datum_kelvin: f64,
        data: CoefficientPrimitive,
    },
    EngineeringDilation {
        datum_kelvin: f64,
        points: Vec<ThermalPointPrimitive>,
    },
    DifferentialPerDatumLength {
        datum_kelvin: f64,
        points: Vec<ThermalPointPrimitive>,
    },
    LogarithmicPerCurrentLength {
        datum_kelvin: f64,
        points: Vec<ThermalPointPrimitive>,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum ThermalPrimitive {
    UnchangedReference,
    ExplicitIntervalStrain {
        strain: f64,
    },
    ConstantAlphaInterval {
        coefficient_per_kelvin: f64,
        temperature_change_kelvin: f64,
    },
    FreeLengthState {
        installation_kelvin: f64,
        operating_kelvin: f64,
        law: ExpansionLawPrimitive,
    },
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FitPrimitive {
    None,
    EngineeringStrain {
        strain: f64,
    },
    /// Only for a reference length actually authored as an input.
    NaturalLengthChange {
        change_m: f64,
        reference_length_m: f64,
    },
    NaturalLengthChangeStraight {
        change_m: f64,
        coordinates_i: [f64; 3],
        coordinates_j: [f64; 3],
    },
    NaturalLengthChangeCurved {
        change_m: f64,
        arc: CurvedFormation,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub struct EigenstrainPrimitive {
    pub thermal: ThermalPrimitive,
    pub fit: FitPrimitive,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MassPrimitive {
    pub outer_diameter: f64,
    pub wall_thickness: f64,
    pub mill_tolerance: f64,
    pub pipe_density: f64,
    pub contents_density: f64,
    pub insulation_thickness: f64,
    pub insulation_density: f64,
    /// T4-U6 (DEL-05-03 Design §1, D-5 mass): `Some(c_a)` selects the
    /// exact-contract basis, pipe metal on the nominal wall t and contents on
    /// the pressure bore D/2 - ((t - c_m) - c_a); `None` keeps the preview
    /// basis, both on the single wall t - c_m.
    pub corrosion_allowance: Option<f64>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum IntensityPrimitive {
    Authored([f64; 3]),
    SelfWeight {
        mass: MassPrimitive,
        gravity: [f64; 3],
    },
    /// A generated intensity, re-formed from its raw factors rather than its value.
    Product {
        factors: Vec<f64>,
        direction: [f64; 3],
    },
    Seismic {
        mass: MassPrimitive,
        coefficient: f64,
        gravity: f64,
        direction: [f64; 3],
    },
    Wind {
        pressure: f64,
        shape_factor: f64,
        outer_diameter: f64,
        insulation_thickness: f64,
        direction: [f64; 3],
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum PressureGeometry {
    Straight { frame: FrameElement },
    Curved { arc: CurvedFormation },
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PressureDirection {
    Chord {
        axis: usize,
    },
    /// Endpoint tangent, component in global axes.
    Tangent {
        end_j: bool,
        axis: usize,
    },
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PressureCoefficient {
    Constant(f64),
    Poisson { poisson_ratio: f64, sign: f64 },
}
#[derive(Debug, Clone, PartialEq)]
pub enum LoadFormula {
    /// A product of two binary64 operands at one nodal DOF, before its
    /// ordinary binary64 fold: the explicit 0.4 source-factor product
    /// (magnitude × factor), or W-16 R-c's spring base load (k × g).
    NodalProduct {
        global_dof: usize,
        magnitude: f64,
        factor: f64,
    },
    UniformStraight {
        frame: FrameElement,
        intensity: IntensityPrimitive,
        start_fraction: f64,
        end_fraction: f64,
    },
    UniformCurved {
        arc: CurvedFormation,
        intensity: IntensityPrimitive,
    },
    FreeStrainStraight {
        frame: FrameElement,
        strain: EigenstrainPrimitive,
    },
    FreeStrainCurved {
        arc: CurvedFormation,
        strain: EigenstrainPrimitive,
        column: Option<usize>,
    },
    PressureOperand {
        geometry: PressureGeometry,
        pressure: f64,
        outer_diameter: f64,
        wall_thickness: f64,
        coefficient: PressureCoefficient,
        direction: PressureDirection,
    },
    PressureFreeStrainCurved {
        arc: CurvedFormation,
        pressure: f64,
        outer_diameter: f64,
        wall_thickness: f64,
        poisson_ratio: f64,
        column: Option<usize>,
    },
    ConnectorReference {
        connector: ObjectiveConnector,
    },
    /// T4-U5b: the reference load +Cᵀ k e_ref of `tie_rod`, rod `rod` of
    /// connector `connector` (indices into `FormationSource::connectors` and
    /// that connector's `tie_rods()`), re-formed at p from the rod's own
    /// authored operands (node coordinates, Q, offsets in Q, k and e_ref)
    /// with n and l in ball arithmetic. The closure binds it exactly: that
    /// source rod must equal `tie_rod` at the primitive's force exponent.
    /// Summand identity `{connector}:{rod}:tie_rod_reference` (family
    /// `tie_rod::TIE_ROD_REFERENCE_FAMILY`); omitted only when e_ref is
    /// exactly 0. Never a connector member load: the connector's certified
    /// member loads keep only its q_ref reference.
    TieRodReference {
        connector: usize,
        rod: usize,
        tie_rod: TieRod,
    },
    /// T4-U5a: dead generalized joint thrust, from authored binary64 operands.
    JointEffectiveArea {
        connector: ObjectiveConnector,
        end: usize,
        pressure: f64,
        effective_area: f64,
        axis: usize,
        sign: f64,
    },
    /// Nonlinear K-D5 (DEL-04-01 kd5-nonlinear-active-set.md §1.6): an
    /// explicit-normal sliding friction force, -direction*coefficient*|normal|,
    /// re-formed as the exact product of the binary64 inputs (not fl(mu*N)).
    SlidingFrictionExplicit {
        global_dof: usize,
        coefficient: f64,
        normal: f64,
        direction: f64,
    },
    /// Nonlinear K-D5 (§1.4, Stage B): a derived-normal sliding friction force,
    /// re-formed as the Stage-A ball (midpoint, radius) that contains the
    /// coupled force; the radius is a load enclosure radius in psi.
    SlidingFrictionCoupled {
        global_dof: usize,
        coefficient: f64,
        direction: f64,
        beta: f64,
        source_dof: usize,
        midpoint: f64,
        radius: f64,
    },
}
/// One supplied primitive; `local_row` selects its component in a member vector.
#[derive(Debug, Clone, PartialEq)]
pub struct LoadPrimitive {
    pub term_index: usize,
    pub identity: LoadSummandIdentity,
    pub local_row: usize,
    pub formula: LoadFormula,
    pub force_scale_exponent: i32,
}
impl LoadPrimitive {
    pub fn closure_entry(&self) -> LoadClosureEntry {
        let mut operand_bits = Vec::new();
        self.formula.binding(&mut operand_bits);
        operand_bits.extend([
            self.local_row as u64,
            self.force_scale_exponent as i64 as u64,
        ]);
        LoadClosureEntry {
            identity: self.identity.clone(),
            operand_bits,
        }
    }
    pub fn force_scaled(&self, exponent: i32) -> Option<Self> {
        let mut out = self.clone();
        out.force_scale_exponent = out.force_scale_exponent.checked_add(exponent)?;
        Some(out)
    }
}
/// Recovery-only inputs, never part of the published structural report.
#[derive(Debug, Clone, PartialEq)]
pub struct MemberBoundInputs {
    pub w: Vec<f64>,
    pub w2: Vec<f64>,
    /// Global-DOF equilibration exponents; i32::MIN means prescribed (no remainder).
    pub exponents: Vec<i32>,
    pub lambda_sigma: f64,
}

/// The primitives of every stiffness contribution of a structural system,
/// for the formation check (the "typed optional formation source", carried
/// by `FormationCheckedSystem`). Node `k` owns global DOFs 6k..6k+5.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FormationSource<'a> {
    pub node_count: usize,
    pub loads: Cow<'a, [LoadPrimitive]>,
    pub frames: Vec<FrameElement>,
    /// T4-U8 (Design §4 item 2): each frame's member identity, parallel to
    /// `frames`: the source prefix its member's load primitives carry (the
    /// product's `member:{len}:{id}:`). A straight load primitive and a
    /// certified straight recovery are bound, bit for bit and beam-theory leaf
    /// included, to the frame of their own member. Without these identities
    /// (empty, or not one non-empty identity per frame) a source that holds a
    /// straight load primitive fails closed.
    pub frame_members: Vec<String>,
    /// T4-U3 (S10): objective connectors, re-formed by `connector_matrix`
    /// (at 2^b, K-D5's scaled source holds `ObjectiveConnector::force_scaled`).
    /// T4-U5b: each connector's tie rods are re-formed by `tie_rod_matrix`
    /// as their own elements, after every connector and before the curved
    /// elements (the assembly's slot order).
    pub connectors: Vec<ObjectiveConnector>,
    pub curved: Vec<CurvedFormation>,
    /// Ground springs (global DOF, stiffness), exact binary64 values.
    pub springs: Vec<(usize, f64)>,
    /// Contributions the check cannot re-form, named (D5C-2). Any entry
    /// demotes the case with `FormationCheckUnavailable`.
    pub unavailable: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormationCheckReason {
    /// The estimate exceeded the criterion (`estimate`).
    Estimate,
    /// The check could not re-form or evaluate the case
    /// (`formation_check_unavailable`); `detail` names the family or failure.
    FormationCheckUnavailable { detail: String },
}

/// Present on `StructuralSolution` only when the check demotes a case; never
/// part of `StructuralReport` (D5C-3), so a report that is not demoted is
/// byte-unchanged and a demoted one differs only in `quality: Sensitive`.
#[derive(Debug, Clone, PartialEq)]
pub struct FormationCheck {
    pub reason: FormationCheckReason,
    /// The demoting free row (the largest trigger value); `None` when unavailable.
    pub global_dof: Option<usize>,
    /// 2·|w_i| at that row, in the row's physical unit (m or rad).
    pub doubled_correction: f64,
    /// max(|q_i|, S*_kind) at that row.
    pub scale: f64,
    /// Trigger value 2|w_i| / (1e-9·scale); +∞ for the zero-scale clause.
    /// The quoted EF reading is half of it.
    pub ratio: f64,
}

impl FormationCheck {
    fn unavailable(detail: impl Into<String>) -> Self {
        Self {
            reason: FormationCheckReason::FormationCheckUnavailable {
                detail: detail.into(),
            },
            global_dof: None,
            doubled_correction: f64::NAN,
            scale: f64::NAN,
            ratio: f64::NAN,
        }
    }
}

/// Why an evaluation could not complete; every variant fails closed.
#[derive(Debug)]
enum Failure {
    Wide(WideError),
    Sum(SumError),
    Structural(StructuralError),
    Shape(&'static str),
    Closure(String),
    Second(String),
    Unavailable(&'static str),
}
impl From<WideError> for Failure {
    fn from(e: WideError) -> Self {
        Self::Wide(e)
    }
}
impl From<SumError> for Failure {
    fn from(e: SumError) -> Self {
        Self::Sum(e)
    }
}
impl From<StructuralError> for Failure {
    fn from(e: StructuralError) -> Self {
        Self::Structural(e)
    }
}
impl Failure {
    fn detail(&self) -> String {
        match self {
            Self::Wide(e) => format!("retained arithmetic: {e}"),
            Self::Sum(e) => format!("exact residual sum: {e}"),
            Self::Structural(e) => format!("correction solve: {e}"),
            Self::Shape(s) => format!("formation source: {s}"),
            Self::Closure(s) => format!("load closure: {s}"),
            Self::Second(s) => format!("second solve: {s}"),
            Self::Unavailable(s) => (*s).into(),
        }
    }
}

/// Runs the closed check and returns demotion evidence plus recovery inputs.
pub(super) fn check<F, A, P>(
    system: &StructuralSystem<'_>,
    source: &FormationSource,
    ledger: Option<&crate::load_ledger::AssembledForce>,
    scale_exponents: &[i32],
    u: &[f64],
    condition: (f64, f64, f64),
    assembled: &A,
    projected: &P,
    solve: &F,
) -> (Option<FormationCheck>, Option<MemberBoundInputs>)
where
    F: Fn(&[f64]) -> Result<Vec<f64>, StructuralError>,
    A: Fn(usize, usize) -> f64,
    P: Fn(usize, usize) -> f64,
{
    check_at(
        system,
        source,
        ledger,
        scale_exponents,
        u,
        condition,
        assembled,
        projected,
        solve,
        true,
        None,
    )
}
/// `check` at a supplied u with the D5C-1 rule on or off and an optional
/// per-attempt epsilon_E cache (kd5-nonlinear-active-set.md §1.5): the
/// confirmation attempt evaluates several systems that share one factor.
#[allow(clippy::too_many_arguments)]
pub(super) fn check_at<F, A, P>(
    system: &StructuralSystem<'_>,
    source: &FormationSource,
    ledger: Option<&crate::load_ledger::AssembledForce>,
    scale_exponents: &[i32],
    u: &[f64],
    condition: (f64, f64, f64),
    assembled: &A,
    projected: &P,
    solve: &F,
    judged: bool,
    epsilon_cache: Option<&std::cell::Cell<Option<f64>>>,
) -> (Option<FormationCheck>, Option<MemberBoundInputs>)
where
    F: Fn(&[f64]) -> Result<Vec<f64>, StructuralError>,
    A: Fn(usize, usize) -> f64,
    P: Fn(usize, usize) -> f64,
{
    if let Some(family) = source.unavailable.first() {
        return (Some(FormationCheck::unavailable(family.clone())), None);
    }
    match evaluate(
        system,
        source,
        ledger,
        scale_exponents,
        u,
        condition,
        assembled,
        projected,
        solve,
        judged,
        epsilon_cache,
    ) {
        Ok(result) => result,
        Err(failure) => (Some(FormationCheck::unavailable(failure.detail())), None),
    }
}
fn load_closure(
    source: &FormationSource,
    ledger: &crate::load_ledger::AssembledForce,
) -> Result<(), Failure> {
    let terms = ledger.terms();
    let records = ledger.closure_records();
    if records.len() != terms.len() {
        return Err(Failure::Closure("record shape".into()));
    }
    for p in source.loads.iter() {
        if p.term_index >= terms.len() {
            return Err(Failure::Closure(
                "extra or mismatched-term primitive".into(),
            ));
        }
        if !records[p.term_index].formed {
            return Err(Failure::Closure(format!(
                "{}: input term carries primitive",
                terms[p.term_index].source
            )));
        }
        if p.local_row >= 12 {
            return Err(Failure::Closure(format!(
                "{}: primitive component",
                terms[p.term_index].source
            )));
        }
        let pair = match &p.formula {
            LoadFormula::NodalProduct { global_dof, .. }
            | LoadFormula::SlidingFrictionExplicit { global_dof, .. }
            | LoadFormula::SlidingFrictionCoupled { global_dof, .. } => {
                if *global_dof != terms[p.term_index].dof || p.local_row != global_dof % 6 {
                    return Err(Failure::Closure(format!(
                        "{}: nodal source DOF binding",
                        terms[p.term_index].source
                    )));
                }
                None
            }
            LoadFormula::UniformStraight { frame, .. }
            | LoadFormula::FreeStrainStraight { frame, .. } => {
                Some((frame.node_i.index, frame.node_j.index))
            }
            LoadFormula::UniformCurved { arc, .. }
            | LoadFormula::FreeStrainCurved { arc, .. }
            | LoadFormula::PressureFreeStrainCurved { arc, .. } => Some((arc.node_i, arc.node_j)),
            LoadFormula::ConnectorReference { connector } => {
                Some((connector.node_i().index, connector.node_j().index))
            }
            LoadFormula::TieRodReference {
                connector,
                rod,
                tie_rod,
            } => {
                // One intended system: source connector `connector`'s rod
                // `rod` is `tie_rod` at the primitive's force exponent (a W2
                // source holds `force_scaled`), compared bit for bit.
                let held = tie_rod
                    .force_scaled(crate::ForceScale {
                        exponent: p.force_scale_exponent,
                    })
                    .ok();
                let bound = source
                    .connectors
                    .get(*connector)
                    .and_then(|c| c.tie_rods().get(*rod))
                    .is_some_and(|r| Some(*r) == held);
                if !bound {
                    return Err(Failure::Closure(format!(
                        "{}: tie-rod source binding",
                        terms[p.term_index].source
                    )));
                }
                Some((tie_rod.node_i().index, tie_rod.node_j().index))
            }
            LoadFormula::JointEffectiveArea { connector, end, axis, sign, .. } => {
                let node = match end {
                    0 => connector.node_i().index,
                    1 => connector.node_j().index,
                    _ => return Err(Failure::Closure("joint effective-area end binding".into())),
                };
                if *axis >= 3 || terms[p.term_index].dof != node * 6 + axis
                    || p.local_row != *axis || !matches!(*sign, -1.0 | 1.0)
                {
                    return Err(Failure::Closure("joint effective-area DOF binding".into()));
                }
                None
            }
            LoadFormula::PressureOperand {
                geometry,
                direction,
                ..
            } => {
                let (ni, nj) = match geometry {
                    PressureGeometry::Straight { frame } => {
                        (frame.node_i.index, frame.node_j.index)
                    }
                    PressureGeometry::Curved { arc } => (arc.node_i, arc.node_j),
                };
                let axis = match direction {
                    PressureDirection::Chord { axis } | PressureDirection::Tangent { axis, .. } => {
                        *axis
                    }
                };
                if ![ni, nj].contains(&(terms[p.term_index].dof / 6))
                    || axis != terms[p.term_index].dof % 6
                    || p.local_row != axis
                {
                    return Err(Failure::Closure(format!(
                        "{}: pressure term DOF binding",
                        terms[p.term_index].source
                    )));
                }
                None
            }
        };
        if let Some((ni, nj)) = pair {
            if ni >= source.node_count
                || nj >= source.node_count
                || element_dof_map(ni, nj)[p.local_row] != terms[p.term_index].dof
            {
                return Err(Failure::Closure(format!(
                    "{}: term DOF binding",
                    terms[p.term_index].source
                )));
            }
        }
        if p.force_scale_exponent != records[p.term_index].force_scale_exponent {
            return Err(Failure::Closure(format!(
                "{}: force exponent binding",
                terms[p.term_index].source
            )));
        }
    }
    for (i, (term, record)) in terms.iter().zip(records).enumerate() {
        if !record.formed {
            continue;
        }
        let manifest = record
            .manifest
            .as_ref()
            .ok_or_else(|| Failure::Closure(format!("{}: absent manifest", term.source)))?;
        if manifest.is_empty() {
            return Err(Failure::Closure(format!(
                "{}: missing summands",
                term.source
            )));
        }
        let actual_count = source.loads.iter().filter(|p| p.term_index == i).count();
        if actual_count != manifest.len() {
            return Err(Failure::Closure(format!(
                "{}: missing, duplicate or extra summand",
                term.source
            )));
        }
        for (j, expected) in manifest.iter().enumerate() {
            if manifest[..j]
                .iter()
                .any(|e| e.identity == expected.identity)
            {
                return Err(Failure::Closure(format!(
                    "{}: duplicate manifest identity",
                    term.source
                )));
            }
            let mut matching = source
                .loads
                .iter()
                .filter(|p| p.term_index == i && p.identity == expected.identity);
            let p = matching.next().ok_or_else(|| {
                Failure::Closure(format!(
                    "{}: missing or mismatched summand identity",
                    term.source
                ))
            })?;
            if matching.next().is_some() {
                return Err(Failure::Closure(format!(
                    "{}: duplicate summand",
                    term.source
                )));
            }
            if p.closure_entry() != *expected {
                return Err(Failure::Closure(format!(
                    "{}: wrong operand binding",
                    term.source
                )));
            }
        }
    }
    Ok(())
}
fn intended_stiffness_pass<F>(source: &FormationSource, mut consume: F) -> Result<(), Failure>
where
    F: FnMut(usize, usize, &Element) -> Result<(), Failure>,
{
    let mut arith = WideArith::new(FORMATION_PRECISION)?;
    for e in &source.frames {
        let k = frame_matrix(&mut arith, e)?;
        consume(e.node_i.index, e.node_j.index, &k)?;
    }
    for e in &source.connectors {
        let k = connector_matrix(&mut arith, e)?;
        consume(e.node_i().index, e.node_j().index, &k)?;
    }
    // T4-U5b: each connector's tie rods, after every connector.
    for e in source.connectors.iter().flat_map(|c| c.tie_rods()) {
        let k = tie_rod_matrix(&mut arith, e)?;
        consume(e.node_i().index, e.node_j().index, &k)?;
    }
    for e in &source.curved {
        let k = curved_matrix(&mut arith, e)?;
        consume(e.node_i, e.node_j, &k)?;
    }
    Ok(())
}
fn subtract_action(
    source: &FormationSource,
    free_index: &[Option<usize>],
    x: &[f64],
    rows: &mut [ExactAccumulator],
    psi: &mut [f64],
) -> Result<(), Failure> {
    intended_stiffness_pass(source, |ni, nj, k| {
        if ni >= source.node_count || nj >= source.node_count || ni == nj {
            return Err(Failure::Shape("element node"));
        }
        let map = element_dof_map(ni, nj);
        for a in 0..12 {
            let Some(r) = free_index[map[a]] else {
                continue;
            };
            for c in 0..12 {
                let value = x[map[c]];
                if value == 0.0 || k[a][c].is_zero() {
                    continue;
                }
                if k[a][c].neg().add_product_to(&mut rows[r], value)? {
                    psi[r] = add_up(psi[r], mul_up(f64::from_bits(1), value.abs()));
                }
            }
        }
        Ok(())
    })?;
    for &(dof, k) in &source.springs {
        if dof >= x.len() {
            return Err(Failure::Shape("spring DOF"));
        }
        if let Some(r) = free_index[dof] {
            rows[r].add_product(-k, x[dof])?;
        }
    }
    Ok(())
}
fn correction<F>(
    rows: &[ExactAccumulator],
    free: &[usize],
    exponents: &[i32],
    n: usize,
    solve: &F,
) -> Result<Vec<f64>, Failure>
where
    F: Fn(&[f64]) -> Result<Vec<f64>, StructuralError>,
{
    let top = rows
        .iter()
        .zip(exponents)
        .map(|(r, &e)| scaled_exponent(r, e))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .max();
    let Some(top) = top else {
        return Ok(vec![0.0; n]);
    };
    let shift = -top;
    let mut rhs = Vec::with_capacity(free.len());
    for (r, &e) in rows.iter().zip(exponents) {
        let v = r.round_scaled(e + shift)?;
        rhs.push(if v.is_subnormal() { 0.0 } else { v });
    }
    let delta = solve(&rhs)?;
    if delta.len() != free.len() || delta.iter().any(|v| !v.is_finite()) {
        return Err(Failure::Shape("correction solve output"));
    }
    let mut out = vec![0.0; n];
    for (r, &i) in free.iter().enumerate() {
        out[i] = pow2(delta[r], exponents[r] - shift);
        if !out[i].is_finite() {
            return Err(Failure::Shape("correction solve range"));
        }
    }
    Ok(out)
}
fn exact_abs_scaled_up(acc: &ExactAccumulator, exponent: i32) -> Result<f64, Failure> {
    if acc.is_zero() {
        return Ok(0.0);
    }
    let value = acc.round_scaled(exponent)?.abs();
    Ok(value.next_up())
}
/// Triangle bound for BOTH row and column norms; no symmetry premise on K_int.
fn epsilon_e<A, P>(
    system: &StructuralSystem<'_>,
    source: &FormationSource,
    index: &[Option<usize>],
    exponents: &[i32],
    assembled: &A,
    projected: &P,
) -> Result<f64, Failure>
where
    A: Fn(usize, usize) -> f64,
    P: Fn(usize, usize) -> f64,
{
    let contributions = system
        .contributions
        .ok_or(Failure::Shape("stiffness contributions absent"))?;
    let n = system.free_dofs.len();
    let mut row_sums = vec![0.0; n];
    let mut col_sums = vec![0.0; n];
    let mut cursor = 0;
    let mut charge = |r: usize, c: usize, value: f64| {
        row_sums[r] = add_up(row_sums[r], value);
        col_sums[c] = add_up(col_sums[c], value);
    };
    intended_stiffness_pass(source, |ni, nj, k| {
        let map = element_dof_map(ni, nj);
        for r in 0..12 {
            for c in 0..12 {
                let entry = contributions
                    .get(cursor)
                    .ok_or(Failure::Shape("element contribution block missing"))?;
                cursor += 1;
                if entry.row != map[r] || entry.col != map[c] {
                    return Err(Failure::Shape("element contribution block identity"));
                }
                let (Some(fr), Some(fc)) = (index[map[r]], index[map[c]]) else {
                    continue;
                };
                let mut diff = ExactAccumulator::new();
                let split = k[r][c].split_binary64()?;
                for &x in split.terms() {
                    diff.add(x)?;
                }
                diff.add(-entry.value)?;
                let mut bound = exact_abs_scaled_up(&diff, exponents[fr] + exponents[fc])?;
                if split.truncated_below_min_subnormal() {
                    bound = add_up(
                        bound,
                        scale_up(f64::from_bits(1), exponents[fr] + exponents[fc]),
                    );
                }
                charge(fr, fc, bound);
            }
        }
        Ok(())
    })?;
    for &(dof, k) in &source.springs {
        let entry = contributions
            .get(cursor)
            .ok_or(Failure::Shape("spring contribution missing"))?;
        cursor += 1;
        if entry.row != dof || entry.col != dof || entry.value.to_bits() != k.to_bits() {
            return Err(Failure::Shape("spring contribution binding"));
        }
    }
    if cursor != contributions.len() {
        return Err(Failure::Shape("extra stiffness contribution"));
    }
    // Exact assembly error, one reusable row of accumulators rather than a matrix.
    let mut sums = vec![ExactAccumulator::new(); n];
    for (r, &i) in system.free_dofs.iter().enumerate() {
        for sum in &mut sums {
            *sum = ExactAccumulator::new();
        }
        for e in contributions {
            if e.row == i {
                if let Some(c) = index[e.col] {
                    sums[c].add(e.value)?;
                }
            }
        }
        for (c, &j) in system.free_dofs.iter().enumerate() {
            let stored = assembled(i, j);
            sums[c].add(-stored)?;
            let assembly = exact_abs_scaled_up(&sums[c], exponents[r] + exponents[c])?;
            let mut projection = ExactAccumulator::new();
            let scaled = Wide2::from_f64(stored)?
                .mul_pow2(i64::from(exponents[r]) + i64::from(exponents[c]))?;
            let split = scaled.split_binary64()?;
            for &x in split.terms() {
                projection.add(x)?;
            }
            projection.add(-projected(r, c))?;
            let mut change = exact_abs_scaled_up(&projection, 0)?;
            if split.truncated_below_min_subnormal() {
                change = add_up(change, f64::from_bits(1));
            }
            charge(r, c, add_up(assembly, change));
        }
    }
    Ok(row_sums.into_iter().chain(col_sums).fold(0.0, f64::max))
}
fn evaluate<F, A, P>(
    system: &StructuralSystem<'_>,
    source: &FormationSource,
    ledger: Option<&crate::load_ledger::AssembledForce>,
    scale_exponents: &[i32],
    u: &[f64],
    condition: (f64, f64, f64),
    assembled: &A,
    projected: &P,
    solve: &F,
    judged: bool,
    epsilon_cache: Option<&std::cell::Cell<Option<f64>>>,
) -> Result<(Option<FormationCheck>, Option<MemberBoundInputs>), Failure>
where
    F: Fn(&[f64]) -> Result<Vec<f64>, StructuralError>,
    A: Fn(usize, usize) -> f64,
    P: Fn(usize, usize) -> f64,
{
    let ledger = ledger.ok_or_else(|| Failure::Closure("no ledger".into()))?;
    load_closure(source, ledger)?;
    straight_frames_bound(source)?;
    let n = system.force.len();
    let free = system.free_dofs;
    if source.node_count.checked_mul(DOF_PER_NODE) != Some(n)
        || u.len() != n
        || scale_exponents.len() != free.len()
    {
        return Err(Failure::Shape("DOF map"));
    }
    let mut index = vec![None; n];
    for (r, &i) in free.iter().enumerate() {
        index[i] = Some(r);
    }
    let mut rho = vec![ExactAccumulator::new(); free.len()];
    let mut psi = vec![0.0; free.len()];
    for (i, term) in ledger.terms().iter().enumerate() {
        if term.dof >= n {
            return Err(Failure::Shape("force term DOF"));
        }
        if !ledger.closure_records()[i].formed {
            if let Some(r) = index[term.dof] {
                term.accumulate(&mut rho[r], false)?;
            }
        } else {
            for p in source.loads.iter().filter(|p| p.term_index == i) {
                let (v, radius) = load_reformation(p)
                    .map_err(|e| Failure::Closure(format!("{}: reformation {e:?}", term.source)))?;
                let Some(r) = index[term.dof] else {
                    continue;
                };
                let value = v[p.local_row];
                if value.add_product_to(&mut rho[r], 1.0)? {
                    psi[r] = add_up(psi[r], f64::from_bits(1));
                }
                psi[r] = add_up(psi[r], radius[p.local_row]);
            }
        }
    }
    subtract_action(source, &index, u, &mut rho, &mut psi)?;
    let w = correction(&rho, free, scale_exponents, n, solve)?;
    let scales = if judged {
        body_scales(source, free, u)
    } else {
        Vec::new()
    };
    let mut worst: Option<FormationCheck> = None;
    // An unjudged evaluation (the nonlinear influence system, §1.4 A-2)
    // publishes nothing: D5C-1 does not apply to it.
    for &i in free.iter().filter(|_| judged) {
        let (tr, ro) = scales[i / 6];
        let scale = u[i].abs().max(if i % 6 >= 3 { ro } else { tr });
        let doubled = FORMATION_FACTOR * w[i].abs();
        let ratio = if scale == 0.0 {
            if w[i] != 0.0 {
                f64::INFINITY
            } else {
                0.0
            }
        } else {
            let criterion = FORMATION_CRITERION * scale;
            if doubled > criterion {
                doubled / criterion
            } else {
                0.0
            }
        };
        if ratio > 0.0 && worst.as_ref().is_none_or(|v| ratio > v.ratio) {
            worst = Some(FormationCheck {
                reason: FormationCheckReason::Estimate,
                global_dof: Some(i),
                doubled_correction: doubled,
                scale,
                ratio,
            });
        }
    }
    if worst.is_some() {
        return Ok((worst, None));
    }

    // Reuse rho's allocation as s, then as s2. No cached element matrices.
    subtract_action(source, &index, &w, &mut rho, &mut psi)
        .map_err(|e| Failure::Second(e.detail()))?;
    let w2 = correction(&rho, free, scale_exponents, n, solve)
        .map_err(|e| Failure::Second(e.detail()))?;
    let mut norm_w = Wide2::ZERO;
    let mut norm_w2 = Wide2::ZERO;
    for (r, &i) in free.iter().enumerate() {
        let x = Wide2::from_f64(w[i].abs())?.mul_pow2(-i64::from(scale_exponents[r]))?;
        let y = Wide2::from_f64(w2[i].abs())?.mul_pow2(-i64::from(scale_exponents[r]))?;
        if x.cmp_value(&norm_w).is_gt() {
            norm_w = x;
        }
        if y.cmp_value(&norm_w2).is_gt() {
            norm_w2 = y;
        }
    }
    if !norm_w2.is_zero() {
        let threshold = norm_w.mul_pow2(-20)?;
        if norm_w2.cmp_value(&threshold).is_gt() {
            return Err(Failure::Unavailable("contraction"));
        }
    }
    subtract_action(source, &index, &w2, &mut rho, &mut psi)
        .map_err(|e| Failure::Second(e.detail()))?;
    let mut sigma = 0.0_f64;
    for ((r, &p), &e) in rho.iter().zip(&psi).zip(scale_exponents) {
        let magnitude = exact_abs_scaled_up(r, e)?;
        sigma = sigma.max(add_up(magnitude, scale_up(p, e)));
    }
    drop(rho);
    let epsilon = match epsilon_cache.and_then(std::cell::Cell::get) {
        Some(epsilon) => epsilon,
        None => {
            let epsilon = epsilon_e(
                system,
                source,
                &index,
                scale_exponents,
                assembled,
                projected,
            )?;
            if let Some(cache) = epsilon_cache {
                cache.set(Some(epsilon));
            }
            epsilon
        }
    };
    let (_rcond, est, _norm) = condition;
    let inverse = mul_up(1024.0, est);
    let neumann = mul_up(inverse, epsilon);
    if !neumann.is_finite() || neumann >= 1.0 {
        return Err(Failure::Unavailable("Neumann"));
    }
    let denominator = (1.0 - neumann).next_down();
    if denominator <= 0.0 {
        return Err(Failure::Unavailable("Neumann"));
    }
    let lambda = div_up(inverse, denominator);
    let lambda_sigma = mul_up(lambda, sigma);
    if !lambda_sigma.is_finite() {
        return Err(Failure::Unavailable("non-finite Lambda sigma"));
    }
    let mut exponents = vec![i32::MIN; n];
    for (r, &i) in free.iter().enumerate() {
        exponents[i] = scale_exponents[r];
    }
    Ok((
        None,
        Some(MemberBoundInputs {
            w,
            w2,
            exponents,
            lambda_sigma,
        }),
    ))
}

/// Binary exponent of ρ_i·2^e, found without rounding it out of range.
fn scaled_exponent(row: &ExactAccumulator, e: i32) -> Result<Option<i32>, Failure> {
    if row.is_zero() {
        return Ok(None);
    }
    for shift in [
        0, 600, -600, 1200, -1200, 1800, -1800, 2400, -2400, 3000, -3000,
    ] {
        if let Ok(v) = row.round_scaled(e + shift) {
            if v.is_normal() {
                return Ok(Some(binary_exponent(v) - shift));
            }
        }
    }
    Err(Failure::Shape("residual exponent"))
}

/// value·2^k in binary64 steps; overflow gives ±∞ and underflow ±0 (the
/// rule treats ∞ as a demotion and uses the unscaled value for zero-scale).
fn pow2(mut value: f64, mut k: i32) -> f64 {
    while k != 0 {
        let step = k.clamp(-512, 512);
        value *= 2.0_f64.powi(step);
        k -= step;
    }
    value
}

/// (translation, rotation) S* of each node's body (D1 §4.1.6.1 items 1, 4–6):
/// bodies are the connected components of the element graph; S(kind) is the
/// largest free |u| of that kind in the body; L_b from the body's extents.
fn body_scales(source: &FormationSource, free: &[usize], u: &[f64]) -> Vec<(f64, f64)> {
    let count = source.node_count;
    let mut parent: Vec<usize> = (0..count).collect();
    fn root(parent: &mut [usize], mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    }
    let mut coordinates: Vec<Option<[f64; 3]>> = vec![None; count];
    let mut edges: Vec<(usize, [f64; 3], usize, [f64; 3])> = Vec::new();
    for e in &source.frames {
        edges.push((
            e.node_i.index,
            e.node_i.coordinates,
            e.node_j.index,
            e.node_j.coordinates,
        ));
    }
    for c in &source.connectors {
        let (i, j) = (c.node_i(), c.node_j());
        edges.push((i.index, i.coordinates, j.index, j.coordinates));
    }
    for e in &source.curved {
        edges.push((e.node_i, e.coordinates_i, e.node_j, e.coordinates_j));
    }
    for &(a, pa, b, pb) in &edges {
        coordinates[a] = Some(pa);
        coordinates[b] = Some(pb);
        let (ra, rb) = (root(&mut parent, a), root(&mut parent, b));
        if ra != rb {
            parent[ra] = rb;
        }
    }
    let mut s_tr = vec![0.0_f64; count];
    let mut s_ro = vec![0.0_f64; count];
    for &i in free {
        let b = root(&mut parent, i / DOF_PER_NODE);
        if i % DOF_PER_NODE < 3 {
            s_tr[b] = s_tr[b].max(u[i].abs());
        } else {
            s_ro[b] = s_ro[b].max(u[i].abs());
        }
    }
    let mut low = vec![[f64::INFINITY; 3]; count];
    let mut high = vec![[f64::NEG_INFINITY; 3]; count];
    for node in 0..count {
        if let Some(p) = coordinates[node] {
            let b = root(&mut parent, node);
            for k in 0..3 {
                low[b][k] = low[b][k].min(p[k]);
                high[b][k] = high[b][k].max(p[k]);
            }
        }
    }
    let mut body = vec![(0.0, 0.0); count];
    for b in 0..count {
        if root(&mut parent, b) != b {
            continue;
        }
        let d: Vec<f64> = (0..3)
            .map(|k| {
                if high[b][k] >= low[b][k] {
                    high[b][k] - low[b][k]
                } else {
                    0.0
                }
            })
            .collect();
        let l_b = ((d[0] * d[0] + d[1] * d[1]) + d[2] * d[2]).sqrt();
        body[b] = if l_b > 0.0 {
            (s_tr[b].max(l_b * s_ro[b]), s_ro[b].max(s_tr[b] / l_b))
        } else {
            (s_tr[b], s_ro[b])
        };
    }
    (0..count)
        .map(|node| body[root(&mut parent, node)])
        .collect()
}

fn bits(out: &mut Vec<u64>, values: impl IntoIterator<Item = f64>) {
    out.extend(values.into_iter().map(f64::to_bits));
}
/// T4-U8 (Design §4 item 2): the binding's beam-theory leaf. An
/// Euler–Bernoulli frame adds nothing, so its binding is unchanged; a
/// Timoshenko frame adds this tag and the bits of A_s. The tag is the bit
/// pattern of a NaN: every word that follows a frame binding in a formula's
/// binding is a small family tag or the bits of a finite operand, so the
/// encoding is prefix-free over re-formable primitives.
const TIMOSHENKO_BINDING_TAG: u64 = u64::MAX;
/// T4-U6 (Design §1, D-5 mass): the mass binding's allowance leaf, on the
/// same pattern. A preview mass (`corrosion_allowance: None`) adds nothing,
/// so its binding is unchanged; an exact-contract mass adds this tag (a NaN
/// bit pattern, never the bits of the finite operand that follows a mass
/// binding) and the bits of c_a.
const MASS_ALLOWANCE_BINDING_TAG: u64 = u64::MAX - 1;
fn frame_binding(out: &mut Vec<u64>, f: &FrameElement) {
    out.extend([f.node_i.index as u64, f.node_j.index as u64]);
    bits(out, f.node_i.coordinates);
    bits(out, f.node_j.coordinates);
    bits(out, f.y_reference);
    let p = &f.section;
    bits(
        out,
        [
            p.elastic_modulus,
            p.shear_modulus,
            p.area,
            p.second_moment_y,
            p.second_moment_z,
            p.torsion_constant,
        ],
    );
    if let BeamTheory::Timoshenko { shear_area } = p.beam_theory {
        out.extend([TIMOSHENKO_BINDING_TAG, shear_area.to_bits()]);
    }
}
/// Two frames bit-equal in every leaf of `frame_binding`, the beam-theory
/// leaf included (no heap copy).
pub(super) fn frames_bit_equal(x: &FrameElement, y: &FrameElement) -> bool {
    let words = |f: &FrameElement| {
        let (p, n) = (&f.section, [f.node_i.coordinates, f.node_j.coordinates, f.y_reference]);
        let leaves = [
            p.elastic_modulus,
            p.shear_modulus,
            p.area,
            p.second_moment_y,
            p.second_moment_z,
            p.torsion_constant,
        ];
        let theory = match p.beam_theory {
            BeamTheory::EulerBernoulli => (0_u64, 0_u64),
            BeamTheory::Timoshenko { shear_area } => (TIMOSHENKO_BINDING_TAG, shear_area.to_bits()),
        };
        (f.node_i.index, f.node_j.index, n.map(|v| v.map(f64::to_bits)), leaves.map(f64::to_bits), theory)
    };
    words(x) == words(y)
}
/// The frame of a straight load primitive.
pub(super) fn straight_primitive_frame(formula: &LoadFormula) -> Option<&FrameElement> {
    match formula {
        LoadFormula::UniformStraight { frame, .. }
        | LoadFormula::FreeStrainStraight { frame, .. }
        | LoadFormula::PressureOperand {
            geometry: PressureGeometry::Straight { frame },
            ..
        } => Some(frame),
        _ => None,
    }
}
/// T4-U8 (Design §4 item 2): every straight load primitive's frame is
/// bit-equal, its beam-theory leaf included, to the FormationSource frame of
/// its own member, compared at the primitive's force exponent (a W2 source
/// holds each frame with E and G times 2^b; a primitive keeps its authored
/// leaves and its exponent). Otherwise the case fails closed: a primitive
/// frame without the stiffness frame's leaf would certify a mixed system (a
/// stale Euler–Bernoulli load vector with Timoshenko K).
///
/// The member is keyed by identity (`FormationSource::frame_members`: the
/// primitive's source begins with its member's identity), so parallel members
/// on one node pair are each bound to their own frame. A source without one
/// non-empty member identity per frame cannot key a straight primitive and
/// fails closed. A primitive whose member has no frame is bound only where its
/// node pair has no frame either (a span whose stiffness is a connector or a
/// spring stand-in has no stiffness frame to mix with; the closure manifest
/// binds it); beside another member's frame it fails closed.
fn straight_frames_bound(source: &FormationSource) -> Result<(), Failure> {
    match unbound_straight_primitive(source) {
        None => Ok(()),
        Some(p) => Err(Failure::Closure(format!(
            "{}: straight primitive frame is not its formation-source frame",
            p.identity.source
        ))),
    }
}
/// The first straight primitive of `source` that `straight_frames_bound`
/// refuses, if any (`primitive_frame_bound` for each, with one cursor).
pub(super) fn unbound_straight_primitive<'a>(source: &'a FormationSource<'_>) -> Option<&'a LoadPrimitive> {
    let mut cursor = FrameCursor::default();
    source
        .loads
        .iter()
        .find(|p| !primitive_frame_bound(source, p, &mut cursor))
}
/// Where `primitive_frame_bound` searches first, on the stack: the last few
/// matched members (a node's grouped pressure operands alternate between the
/// members meeting it), then the frames from the last match on, wrapping, so
/// a producer that lists its members in frame order is checked in about one
/// pass over both lists. It never changes a result, only the search order.
#[derive(Debug, Default)]
pub(super) struct FrameCursor {
    recent: [Option<usize>; 4],
    slot: usize,
    start: usize,
}
/// Whether every frame of `source` carries one non-empty member identity
/// (`FormationSource::frame_members`).
pub(super) fn frame_members_complete(source: &FormationSource) -> bool {
    source.frame_members.len() == source.frames.len()
        && source.frame_members.iter().all(|m| !m.is_empty())
}
/// The index of the frame whose member identity is exactly `member`, when the
/// source's member identities are complete.
pub(super) fn member_frame_index(source: &FormationSource, member: &str) -> Option<usize> {
    if member.is_empty() || !frame_members_complete(source) {
        return None;
    }
    source.frame_members.iter().position(|m| m == member)
}
/// `p` is not a straight primitive; or its member (the frame whose identity
/// begins its source) has a source frame bit-equal to its frame at its force
/// exponent; or its member has no frame and neither has its node pair
/// (`straight_frames_bound`). A source without complete member identities
/// binds no straight primitive.
pub(super) fn primitive_frame_bound(
    source: &FormationSource,
    p: &LoadPrimitive,
    cursor: &mut FrameCursor,
) -> bool {
    let Some(frame) = straight_primitive_frame(&p.formula) else {
        return true;
    };
    if !frame_members_complete(source) {
        return false;
    }
    let pair = |f: &FrameElement| {
        let (a, b) = (f.node_i.index, f.node_j.index);
        (a.min(b), a.max(b))
    };
    let own = |k: usize| p.identity.source.starts_with(source.frame_members[k].as_str());
    // E and G times 2^exponent, exactly (any exponent, as the ledger's).
    let scaled = frame
        .force_scaled(crate::ForceScale {
            exponent: p.force_scale_exponent,
        })
        .ok();
    // Unscalable leaves match no frame.
    let bound = |k: usize| scaled.as_ref().is_some_and(|s| frames_bit_equal(&source.frames[k], s));
    if let Some(&k) = cursor.recent.iter().flatten().find(|&&k| own(k)) {
        return bound(k);
    }
    let count = source.frames.len();
    let start = if cursor.start < count { cursor.start } else { 0 };
    let mut paired = false;
    for k in (start..count).chain(0..start) {
        if own(k) {
            cursor.start = k;
            cursor.recent[cursor.slot] = Some(k);
            cursor.slot = (cursor.slot + 1) % cursor.recent.len();
            return bound(k);
        }
        paired |= pair(&source.frames[k]) == pair(frame);
    }
    !paired
}
fn curved_binding(out: &mut Vec<u64>, f: &CurvedFormation) {
    out.extend([f.node_i as u64, f.node_j as u64]);
    bits(out, f.coordinates_i);
    bits(out, f.coordinates_j);
    bits(out, f.y_reference);
    bits(
        out,
        [
            f.radius,
            f.elastic_modulus,
            f.shear_modulus,
            f.area,
            f.second_moment,
            f.torsion_constant,
            f.in_plane_flexibility_factor,
            f.out_of_plane_flexibility_factor,
        ],
    );
}
fn mass_binding(out: &mut Vec<u64>, m: &MassPrimitive) {
    bits(
        out,
        [
            m.outer_diameter,
            m.wall_thickness,
            m.mill_tolerance,
            m.pipe_density,
            m.contents_density,
            m.insulation_thickness,
            m.insulation_density,
        ],
    );
    if let Some(allowance) = m.corrosion_allowance {
        out.extend([MASS_ALLOWANCE_BINDING_TAG, allowance.to_bits()]);
    }
}
fn intensity_binding(out: &mut Vec<u64>, v: &IntensityPrimitive) {
    match v {
        IntensityPrimitive::Authored(w) => {
            out.push(0);
            bits(out, *w);
        }
        IntensityPrimitive::SelfWeight { mass, gravity } => {
            out.push(1);
            mass_binding(out, mass);
            bits(out, *gravity);
        }
        IntensityPrimitive::Product { factors, direction } => {
            out.extend([2, factors.len() as u64]);
            bits(out, factors.iter().copied());
            bits(out, *direction);
        }
        IntensityPrimitive::Seismic {
            mass,
            coefficient,
            gravity,
            direction,
        } => {
            out.push(3);
            mass_binding(out, mass);
            bits(out, [*coefficient, *gravity]);
            bits(out, *direction);
        }
        IntensityPrimitive::Wind {
            pressure,
            shape_factor,
            outer_diameter,
            insulation_thickness,
            direction,
        } => {
            out.push(4);
            bits(
                out,
                [
                    *pressure,
                    *shape_factor,
                    *outer_diameter,
                    *insulation_thickness,
                ],
            );
            bits(out, *direction);
        }
    }
}
fn points_binding(out: &mut Vec<u64>, p: &[ThermalPointPrimitive]) {
    out.push(p.len() as u64);
    for p in p {
        bits(out, [p.temperature_kelvin, p.value]);
    }
}
fn strain_binding(out: &mut Vec<u64>, v: &EigenstrainPrimitive) {
    match &v.thermal {
        ThermalPrimitive::UnchangedReference => out.push(0),
        ThermalPrimitive::ExplicitIntervalStrain { strain } => {
            out.push(1);
            bits(out, [*strain]);
        }
        ThermalPrimitive::ConstantAlphaInterval {
            coefficient_per_kelvin,
            temperature_change_kelvin,
        } => {
            out.push(2);
            bits(out, [*coefficient_per_kelvin, *temperature_change_kelvin]);
        }
        ThermalPrimitive::FreeLengthState {
            installation_kelvin,
            operating_kelvin,
            law,
        } => {
            out.push(3);
            bits(out, [*installation_kelvin, *operating_kelvin]);
            match law {
                ExpansionLawPrimitive::EngineeringSecant { datum_kelvin, data } => {
                    out.push(0);
                    bits(out, [*datum_kelvin]);
                    match data {
                        CoefficientPrimitive::Constant(x) => {
                            out.push(0);
                            bits(out, [*x]);
                        }
                        CoefficientPrimitive::Table(p) => {
                            out.push(1);
                            points_binding(out, p);
                        }
                    }
                }
                ExpansionLawPrimitive::EngineeringDilation {
                    datum_kelvin,
                    points,
                } => {
                    out.push(1);
                    bits(out, [*datum_kelvin]);
                    points_binding(out, points);
                }
                ExpansionLawPrimitive::DifferentialPerDatumLength {
                    datum_kelvin,
                    points,
                } => {
                    out.push(2);
                    bits(out, [*datum_kelvin]);
                    points_binding(out, points);
                }
                ExpansionLawPrimitive::LogarithmicPerCurrentLength {
                    datum_kelvin,
                    points,
                } => {
                    out.push(3);
                    bits(out, [*datum_kelvin]);
                    points_binding(out, points);
                }
            }
        }
    }
    match v.fit {
        FitPrimitive::None => out.push(0),
        FitPrimitive::EngineeringStrain { strain } => {
            out.push(1);
            bits(out, [strain]);
        }
        FitPrimitive::NaturalLengthChange {
            change_m,
            reference_length_m,
        } => {
            out.push(2);
            bits(out, [change_m, reference_length_m]);
        }
        FitPrimitive::NaturalLengthChangeStraight {
            change_m,
            coordinates_i,
            coordinates_j,
        } => {
            out.push(3);
            bits(out, [change_m]);
            bits(out, coordinates_i);
            bits(out, coordinates_j);
        }
        FitPrimitive::NaturalLengthChangeCurved { change_m, arc } => {
            out.push(4);
            bits(out, [change_m]);
            curved_binding(out, &arc);
        }
    }
}
/// The `ConnectorReference` binding (tag 6, then the connector's operands),
/// which `JointEffectiveArea` and `TieRodReference` also carry. A
/// connector's tie rods are not part of it.
fn connector_binding(out: &mut Vec<u64>, c: &ObjectiveConnector) {
    out.push(6);
    out.extend([c.node_i().index as u64, c.node_j().index as u64]);
    bits(out, c.node_i().coordinates);
    bits(out, c.node_j().coordinates);
    let (i, j) = c.offsets();
    bits(out, i);
    bits(out, j);
    for r in c.axes() {
        bits(out, r);
    }
    for r in c.stiffness() {
        bits(out, r);
    }
    bits(out, c.q_ref());
}
/// T4-U5b: a tie rod's authored operands: its end nodes and coordinates, Q,
/// its offsets in Q, k and e_ref.
fn tie_rod_binding(out: &mut Vec<u64>, r: &TieRod) {
    out.extend([r.node_i().index as u64, r.node_j().index as u64]);
    bits(out, r.node_i().coordinates);
    bits(out, r.node_j().coordinates);
    for row in r.axes() {
        bits(out, row);
    }
    let (i, j) = r.local_offsets();
    bits(out, i);
    bits(out, j);
    bits(out, [r.stiffness(), r.reference_extension()]);
}
impl LoadFormula {
    fn binding(&self, out: &mut Vec<u64>) {
        match self {
            Self::NodalProduct {
                global_dof,
                magnitude,
                factor,
            } => {
                out.extend([7, *global_dof as u64]);
                bits(out, [*magnitude, *factor]);
            }
            Self::UniformStraight {
                frame,
                intensity,
                start_fraction,
                end_fraction,
            } => {
                out.push(0);
                frame_binding(out, frame);
                intensity_binding(out, intensity);
                bits(out, [*start_fraction, *end_fraction]);
            }
            Self::UniformCurved { arc, intensity } => {
                out.push(1);
                curved_binding(out, arc);
                intensity_binding(out, intensity);
            }
            Self::FreeStrainStraight { frame, strain } => {
                out.push(2);
                frame_binding(out, frame);
                strain_binding(out, strain);
            }
            Self::FreeStrainCurved {
                arc,
                strain,
                column,
            } => {
                out.push(3);
                curved_binding(out, arc);
                strain_binding(out, strain);
                out.push(column.map_or(u64::MAX, |c| c as u64));
            }
            Self::PressureOperand {
                geometry,
                pressure,
                outer_diameter,
                wall_thickness,
                coefficient,
                direction,
            } => {
                out.push(4);
                match geometry {
                    PressureGeometry::Straight { frame } => {
                        out.push(0);
                        frame_binding(out, frame);
                    }
                    PressureGeometry::Curved { arc } => {
                        out.push(1);
                        curved_binding(out, arc);
                    }
                }
                bits(out, [*pressure, *outer_diameter, *wall_thickness]);
                match coefficient {
                    PressureCoefficient::Constant(c) => {
                        out.push(0);
                        bits(out, [*c]);
                    }
                    PressureCoefficient::Poisson {
                        poisson_ratio,
                        sign,
                    } => {
                        out.push(1);
                        bits(out, [*poisson_ratio, *sign]);
                    }
                }
                match direction {
                    PressureDirection::Chord { axis } => out.extend([0, *axis as u64]),
                    PressureDirection::Tangent { end_j, axis } => {
                        out.extend([1, u64::from(*end_j), *axis as u64])
                    }
                }
            }
            Self::PressureFreeStrainCurved {
                arc,
                pressure,
                outer_diameter,
                wall_thickness,
                poisson_ratio,
                column,
            } => {
                out.push(5);
                curved_binding(out, arc);
                bits(
                    out,
                    [*pressure, *outer_diameter, *wall_thickness, *poisson_ratio],
                );
                out.push(column.map_or(u64::MAX, |c| c as u64));
            }
            Self::ConnectorReference { connector: c } => connector_binding(out, c),
            Self::TieRodReference {
                connector,
                rod,
                tie_rod,
            } => {
                out.extend([11, *connector as u64, *rod as u64]);
                tie_rod_binding(out, tie_rod);
            }
            Self::JointEffectiveArea { connector, end, pressure, effective_area, axis, sign } => {
                out.push(10);
                connector_binding(out, connector);
                out.extend([*end as u64, *axis as u64]);
                bits(out, [*pressure, *effective_area, *sign]);
            }
            Self::SlidingFrictionExplicit {
                global_dof,
                coefficient,
                normal,
                direction,
            } => {
                out.extend([8, *global_dof as u64]);
                bits(out, [*coefficient, *normal, *direction]);
            }
            Self::SlidingFrictionCoupled {
                global_dof,
                coefficient,
                direction,
                beta,
                source_dof,
                midpoint,
                radius,
            } => {
                out.extend([9, *global_dof as u64, *source_dof as u64]);
                bits(out, [*coefficient, *direction, *beta, *midpoint, *radius]);
            }
        }
    }
}

use super::arc_certificate::{
    abs_up, add_up, curved_uniform_load_balls, div_up, finite, mul_up, ArcCertificateFailure, Ball,
    Balls,
};
type LoadFailure = ArcCertificateFailure;
fn bad_load(site: &'static str) -> LoadFailure {
    LoadFailure::DivisorReachesZero { site }
}
fn ball_lift3(b: &Balls, x: [f64; 3]) -> Result<[Ball; 3], LoadFailure> {
    Ok([b.lift(x[0])?, b.lift(x[1])?, b.lift(x[2])?])
}
pub(super) fn ball_chord_axes(
    b: &mut Balls,
    xi: [f64; 3],
    xj: [f64; 3],
    yr: [f64; 3],
) -> Result<([[Ball; 3]; 3], Ball), LoadFailure> {
    let (xi, xj) = (ball_lift3(b, xi)?, ball_lift3(b, xj)?);
    let mut d = [Ball::ZERO; 3];
    for k in 0..3 {
        d[k] = b.sub(xj[k], xi[k])?;
    }
    let d2 = b.dot(&d, &d)?;
    let l = b.sqrt(d2, "load chord")?;
    let mut ex = [Ball::ZERO; 3];
    for k in 0..3 {
        ex[k] = b.div(d[k], l, "load chord")?;
    }
    let yr = ball_lift3(b, yr)?;
    let projection = b.dot(&yr, &ex)?;
    let mut yc = [Ball::ZERO; 3];
    for k in 0..3 {
        let p = b.mul(projection, ex[k])?;
        yc[k] = b.sub(yr[k], p)?;
    }
    let y2 = b.dot(&yc, &yc)?;
    let ym = b.sqrt(y2, "load axes")?;
    let mut ey = [Ball::ZERO; 3];
    for k in 0..3 {
        ey[k] = b.div(yc[k], ym, "load axes")?;
    }
    let ez = b.cross(&ex, &ey)?;
    Ok(([ex, ey, ez], l))
}
fn ball_pi(b: &mut Balls) -> Result<Ball, LoadFailure> {
    let a = b.atan(Ball::ONE)?;
    let twice = b.pow2(a, 1)?;
    b.pow2(twice, 1)
}
fn mass_reformation(b: &mut Balls, m: &MassPrimitive) -> Result<Ball, LoadFailure> {
    let pi = ball_pi(b)?;
    let od = b.lift(m.outer_diameter)?;
    let wall = b.lift(m.wall_thickness)?;
    let tol = b.lift(m.mill_tolerance)?;
    let t = b.sub(wall, tol)?;
    let r = b.half(od)?;
    // D-5 (Design §1): the exact basis puts the metal on the nominal wall and
    // the contents on the pressure bore r - ((t - c_m) - c_a); the preview
    // basis puts both on the single wall t - c_m, in today's operation order.
    let (metal_wall, bore_wall) = match m.corrosion_allowance {
        Some(allowance) => {
            let ca = b.lift(allowance)?;
            (wall, Some(b.sub(t, ca)?))
        }
        None => (t, None),
    };
    let ri = b.sub(r, metal_wall)?;
    let ro2 = b.mul(r, r)?;
    let ri2 = b.mul(ri, ri)?;
    let pipe_area = b.sub(ro2, ri2)?;
    let pipe_area = b.mul(pi, pipe_area)?;
    let rho = b.lift(m.pipe_density)?;
    let pipe_mass = b.mul(rho, pipe_area)?;
    let bore2 = match bore_wall {
        Some(bore_wall) => {
            let bore = b.sub(r, bore_wall)?;
            b.mul(bore, bore)?
        }
        None => ri2,
    };
    let inside_area = b.mul(pi, bore2)?;
    let rho = b.lift(m.contents_density)?;
    let contents = b.mul(rho, inside_area)?;
    let ins = b.lift(m.insulation_thickness)?;
    let ro = b.add(r, ins)?;
    let ro2i = b.mul(ro, ro)?;
    let ring = b.sub(ro2i, ro2)?;
    let area = b.mul(pi, ring)?;
    let rho = b.lift(m.insulation_density)?;
    let insulation = b.mul(rho, area)?;
    let sum = b.add(pipe_mass, contents)?;
    b.add(sum, insulation)
}
fn intensity_balls(b: &mut Balls, v: &IntensityPrimitive) -> Result<[Ball; 3], LoadFailure> {
    let (factor, direction) = match v {
        IntensityPrimitive::Authored(w) => return ball_lift3(b, *w),
        IntensityPrimitive::SelfWeight { mass, gravity } => (mass_reformation(b, mass)?, *gravity),
        IntensityPrimitive::Product { factors, direction } => {
            let mut p = Ball::ONE;
            for &v in factors {
                let x = b.lift(v)?;
                p = b.mul(p, x)?;
            }
            (p, *direction)
        }
        IntensityPrimitive::Seismic {
            mass,
            coefficient,
            gravity,
            direction,
        } => {
            let m = mass_reformation(b, mass)?;
            let c = b.lift(*coefficient)?;
            let g = b.lift(*gravity)?;
            let cg = b.mul(c, g)?;
            (b.mul(m, cg)?, *direction)
        }
        IntensityPrimitive::Wind {
            pressure,
            shape_factor,
            outer_diameter,
            insulation_thickness,
            direction,
        } => {
            let p = b.lift(*pressure)?;
            let c = b.lift(*shape_factor)?;
            let od = b.lift(*outer_diameter)?;
            let ins = b.lift(*insulation_thickness)?;
            let twoins = b.pow2(ins, 1)?;
            let d = b.add(od, twoins)?;
            let pc = b.mul(p, c)?;
            (b.mul(pc, d)?, *direction)
        }
    };
    let mut out = [Ball::ZERO; 3];
    for k in 0..3 {
        let d = b.lift(direction[k])?;
        out[k] = b.mul(factor, d)?;
    }
    Ok(out)
}
pub(super) fn intensity_reformation(v: &IntensityPrimitive) -> Result<[Ball; 3], LoadFailure> {
    let mut b = Balls::new()?;
    intensity_balls(&mut b, v)
}
fn table_value(
    b: &mut Balls,
    points: &[ThermalPointPrimitive],
    t: f64,
) -> Result<Ball, LoadFailure> {
    if !t.is_finite()
        || points.len() < 2
        || points.windows(2).any(|p| {
            !p[0].temperature_kelvin.is_finite()
                || p[0].temperature_kelvin >= p[1].temperature_kelvin
        })
    {
        return Err(bad_load("thermal table"));
    }
    for p in points {
        if p.temperature_kelvin == t {
            return b.lift(p.value);
        }
    }
    let pair = points
        .windows(2)
        .find(|p| p[0].temperature_kelvin < t && t < p[1].temperature_kelvin)
        .ok_or_else(|| bad_load("thermal coverage"))?;
    let ta = b.lift(pair[0].temperature_kelvin)?;
    let tb = b.lift(pair[1].temperature_kelvin)?;
    let t = b.lift(t)?;
    let dt = b.sub(t, ta)?;
    let span = b.sub(tb, ta)?;
    let f = b.div(dt, span, "thermal interpolation")?;
    let a = b.lift(pair[0].value)?;
    let z = b.lift(pair[1].value)?;
    let diff = b.sub(z, a)?;
    let step = b.mul(f, diff)?;
    b.add(a, step)
}
fn table_integral(
    b: &mut Balls,
    points: &[ThermalPointPrimitive],
    start: f64,
    end: f64,
) -> Result<Ball, LoadFailure> {
    if start == end {
        return Ok(Ball::ZERO);
    }
    let (lo, hi) = (start.min(end), start.max(end));
    let mut sum = Ball::ZERO;
    let mut at = lo;
    table_value(b, points, lo)?;
    table_value(b, points, hi)?;
    for pair in points.windows(2) {
        let a = lo.max(pair[0].temperature_kelvin);
        let z = hi.min(pair[1].temperature_kelvin);
        if a >= z {
            continue;
        }
        if a != at {
            return Err(bad_load("thermal integration coverage"));
        }
        let va = table_value(b, points, a)?;
        let vz = table_value(b, points, z)?;
        let total = b.add(va, vz)?;
        let half = b.half(total)?;
        let av = b.lift(a)?;
        let zv = b.lift(z)?;
        let length = b.sub(zv, av)?;
        let term = b.mul(length, half)?;
        sum = b.add(sum, term)?;
        at = z;
    }
    if at != hi {
        return Err(bad_load("thermal integration coverage"));
    }
    Ok(if start > end { sum.neg() } else { sum })
}
/// Ball Taylor with |x| <= 1/16 and a geometric tail; scaling/squaring is stack-only.
fn ball_exp(b: &mut Balls, mut x: Ball) -> Result<Ball, LoadFailure> {
    let mut halves = 0;
    while add_up(abs_up(&x.m)?, x.r) > 0.0625 {
        if halves == 16 {
            return Err(bad_load("thermal exponential range"));
        }
        x = b.half(x)?;
        halves += 1;
    }
    let mut sum = Ball::ONE;
    let mut term = Ball::ONE;
    for n in 1..=48 {
        let p = b.mul(term, x)?;
        let divisor = b.lift(n as f64)?;
        term = b.div(p, divisor, "thermal exponential")?;
        sum = b.add(sum, term)?;
    }
    let p = b.mul(term, x)?;
    let d = b.lift(49.0)?;
    let next = b.div(p, d, "thermal exponential tail")?;
    sum.r = finite(
        add_up(sum.r, mul_up(2.0, add_up(abs_up(&next.m)?, next.r))),
        "thermal exponential tail",
    )?;
    for _ in 0..halves {
        sum = b.mul(sum, sum)?;
    }
    Ok(sum)
}
/// A raw strain is known zero only when its intended p128 midpoint and its
/// conservative enclosure radius are both exactly zero. Reformation failure
/// never establishes zero. Callers bind source-derived fit geometry first.
pub fn eigenstrain_is_exact_zero(source: &EigenstrainPrimitive) -> bool {
    let Ok(mut balls) = Balls::new() else {
        return false;
    };
    matches!(eigenstrain_reformation(&mut balls,source),Ok(value) if value.m.is_zero()&&value.r==0.0)
}

pub(super) fn eigenstrain_reformation(
    b: &mut Balls,
    v: &EigenstrainPrimitive,
) -> Result<Ball, LoadFailure> {
    let thermal = match &v.thermal {
        ThermalPrimitive::UnchangedReference => Ball::ZERO,
        ThermalPrimitive::ExplicitIntervalStrain { strain } => b.lift(*strain)?,
        ThermalPrimitive::ConstantAlphaInterval {
            coefficient_per_kelvin,
            temperature_change_kelvin,
        } => {
            let a = b.lift(*coefficient_per_kelvin)?;
            let t = b.lift(*temperature_change_kelvin)?;
            b.mul(a, t)?
        }
        ThermalPrimitive::FreeLengthState {
            installation_kelvin: i,
            operating_kelvin: o,
            law,
        } => {
            if i == o {
                Ball::ZERO
            } else {
                match law {
                    ExpansionLawPrimitive::EngineeringSecant { datum_kelvin, data } => {
                        let sample = |b: &mut Balls, t: f64| -> Result<Ball, LoadFailure> {
                            let alpha = match data {
                                CoefficientPrimitive::Constant(a) => b.lift(*a)?,
                                CoefficientPrimitive::Table(p) => table_value(b, p, t)?,
                            };
                            let tv = b.lift(t)?;
                            let d = b.lift(*datum_kelvin)?;
                            let dt = b.sub(tv, d)?;
                            b.mul(alpha, dt)
                        };
                        let ei = sample(b, *i)?;
                        let eo = sample(b, *o)?;
                        let diff = b.sub(eo, ei)?;
                        let den = b.add(Ball::ONE, ei)?;
                        b.div(diff, den, "thermal free length")?
                    }
                    ExpansionLawPrimitive::EngineeringDilation { points, .. } => {
                        let ei = table_value(b, points, *i)?;
                        let eo = table_value(b, points, *o)?;
                        let diff = b.sub(eo, ei)?;
                        let den = b.add(Ball::ONE, ei)?;
                        b.div(diff, den, "thermal dilation")?
                    }
                    ExpansionLawPrimitive::DifferentialPerDatumLength {
                        datum_kelvin,
                        points,
                    } => {
                        let ei = table_integral(b, points, *datum_kelvin, *i)?;
                        let change = table_integral(b, points, *i, *o)?;
                        let den = b.add(Ball::ONE, ei)?;
                        b.div(change, den, "thermal differential")?
                    }
                    ExpansionLawPrimitive::LogarithmicPerCurrentLength { points, .. } => {
                        let change = table_integral(b, points, *i, *o)?;
                        let stretch = ball_exp(b, change)?;
                        b.sub(stretch, Ball::ONE)?
                    }
                }
            }
        }
    };
    let fit = match v.fit {
        FitPrimitive::None => Ball::ZERO,
        FitPrimitive::EngineeringStrain { strain } => b.lift(strain)?,
        FitPrimitive::NaturalLengthChange {
            change_m,
            reference_length_m,
        } => {
            let c = b.lift(change_m)?;
            let l = b.lift(reference_length_m)?;
            b.div(c, l, "fit reference length")?
        }
        FitPrimitive::NaturalLengthChangeStraight {
            change_m,
            coordinates_i,
            coordinates_j,
        } => {
            let xi = ball_lift3(b, coordinates_i)?;
            let xj = ball_lift3(b, coordinates_j)?;
            let mut chord = [Ball::ZERO; 3];
            for k in 0..3 {
                chord[k] = b.sub(xj[k], xi[k])?;
            }
            let square = b.dot(&chord, &chord)?;
            let length = b.sqrt(square, "fit chord length")?;
            let change = b.lift(change_m)?;
            b.div(change, length, "fit chord length")?
        }
        FitPrimitive::NaturalLengthChangeCurved { change_m, arc } => {
            let geometry = super::arc_certificate::arc_geometry(b, &arc)?;
            let enclosure = b.mul(geometry.radius, geometry.phi)?;
            let mut arith = WideArith::new(FORMATION_PRECISION)?;
            let (radius, phi) = curved_radius_angle(&mut arith, &arc)?;
            let length = arith.mul(&radius, &phi)?;
            // Recenter the real-arc enclosure at K_int's own p128 R*phi.
            // The two equivalent arctangent formulas need not have identical midpoint bits.
            let difference = b.sub(enclosure, Ball::exact(length))?;
            let length = Ball {
                m: length,
                r: finite(
                    add_up(abs_up(&difference.m)?, difference.r),
                    "fit arc length",
                )?,
            };
            let change = b.lift(change_m)?;
            b.div(change, length, "fit arc length")?
        }
    };
    // λ_fit λ_thermal − 1 = ε_fit + ε_thermal + ε_fit ε_thermal.
    let sum = b.add(fit, thermal)?;
    let cross = b.mul(fit, thermal)?;
    b.add(sum, cross)
}
fn uniform_straight_balls(
    b: &mut Balls,
    f: &FrameElement,
    w: [Ball; 3],
    start: f64,
    end: f64,
) -> Result<[Ball; 12], LoadFailure> {
    if !(0.0 <= start && start < end && end <= 1.0) {
        return Err(bad_load("uniform span"));
    }
    let (axes, l) = ball_chord_axes(b, f.node_i.coordinates, f.node_j.coordinates, f.y_reference)?;
    let l2 = b.mul(l, l)?;
    let mut local_w = [Ball::ZERO; 3];
    for k in 0..3 {
        local_w[k] = b.dot(&axes[k], &w)?;
    }
    let primitive = |b: &mut Balls, x: f64| -> Result<[Ball; 6], LoadFailure> {
        let x = b.lift(x)?;
        let x2 = b.mul(x, x)?;
        let x3 = b.mul(x2, x)?;
        let x4 = b.mul(x3, x)?;
        let h2 = b.half(x2)?;
        let h4 = b.half(x4)?;
        let q4 = b.half(h4)?;
        let one3 = b.lift(3.0)?;
        let third3 = b.div(x3, one3, "uniform coefficient")?;
        let two3 = b.pow2(third3, 1)?;
        let ax = b.sub(x, h2)?;
        let tr0 = b.sub(x, x3)?;
        let tr = b.add(tr0, h4)?;
        let r0 = b.sub(h2, two3)?;
        let r = b.add(r0, q4)?;
        let trj = b.sub(x3, h4)?;
        let rj = b.sub(q4, third3)?;
        Ok([ax, h2, tr, r, trj, rj])
    };
    let pa = primitive(b, start)?;
    let pb = primitive(b, end)?;
    let mut v = [Ball::ZERO; 6];
    for k in 0..6 {
        let d = b.sub(pb[k], pa[k])?;
        v[k] = b.mul(if k == 3 || k == 5 { l2 } else { l }, d)?;
    }
    let mut local = [Ball::ZERO; 12];
    for end in 0..2 {
        let offset = end * 6;
        local[offset] = b.mul(local_w[0], v[end])?;
        local[offset + 1] = b.mul(local_w[1], v[2 + 2 * end])?;
        local[offset + 2] = b.mul(local_w[2], v[2 + 2 * end])?;
        local[offset + 5] = b.mul(local_w[1], v[3 + 2 * end])?;
        local[offset + 4] = b.mul(local_w[2], v[3 + 2 * end])?.neg();
    }
    let mut out = [Ball::ZERO; 12];
    for block in 0..4 {
        for c in 0..3 {
            let mut sum = Ball::ZERO;
            for k in 0..3 {
                let t = b.mul(axes[k][c], local[3 * block + k])?;
                sum = b.add(sum, t)?;
            }
            out[3 * block + c] = sum;
        }
    }
    Ok(out)
}
fn curved_strain_balls(
    b: &mut Balls,
    arc: &CurvedFormation,
    strain: Ball,
    column: Option<usize>,
) -> Result<[Ball; 12], LoadFailure> {
    if column.is_some_and(|c| !(6..9).contains(&c)) {
        return Err(bad_load("arc strain column"));
    }
    let mut a = WideArith::new(FORMATION_PRECISION)?;
    let k = curved_matrix(&mut a, arc)?;
    let mut free = [Ball::ZERO; 12];
    for c in 0..3 {
        let xi = b.lift(arc.coordinates_i[c])?;
        let xj = b.lift(arc.coordinates_j[c])?;
        let d = b.sub(xj, xi)?;
        free[c + 6] = b.mul(strain, d)?;
    }
    let mut out = [Ball::ZERO; 12];
    for r in 0..12 {
        let mut sum = Ball::ZERO;
        for c in 6..9 {
            if column.is_some_and(|col| col != c) {
                continue;
            }
            let t = b.mul(Ball::exact(k[r][c]), free[c])?;
            sum = b.add(sum, t)?;
        }
        out[r] = sum;
    }
    Ok(out)
}
/// Complete stack-only vector of the same intended formula used by residual/recovery.
pub(super) fn load_reformation(p: &LoadPrimitive) -> Result<([Wide2; 12], [f64; 12]), LoadFailure> {
    let mut b = Balls::new()?;
    let mut out = match &p.formula {
        LoadFormula::NodalProduct {
            magnitude, factor, ..
        } => {
            let magnitude = b.lift(*magnitude)?;
            let factor = b.lift(*factor)?;
            [b.mul(magnitude, factor)?; 12]
        }
        LoadFormula::UniformStraight {
            frame,
            intensity,
            start_fraction,
            end_fraction,
        } => {
            // T4-U8 (Design §3.4, §4 item 4): a full span has the
            // Euler–Bernoulli vector in a Timoshenko element; a partial span
            // does not, and its exact Timoshenko vector is not re-formed
            // here, so it is unavailable (fail closed), never formed with the
            // stale Euler–Bernoulli vector.
            if frame.section.beam_theory.is_timoshenko()
                && !(*start_fraction == 0.0 && *end_fraction == 1.0)
            {
                return Err(LoadFailure::Wide(WideError::CountRange(
                    "timoshenko partial-span uniform load: unavailable",
                )));
            }
            let w = intensity_balls(&mut b, intensity)?;
            uniform_straight_balls(&mut b, frame, w, *start_fraction, *end_fraction)?
        }
        LoadFormula::UniformCurved { arc, intensity } => {
            let w = intensity_balls(&mut b, intensity)?;
            curved_uniform_load_balls(arc, w)?.0
        }
        LoadFormula::FreeStrainStraight { frame, strain } => {
            let eps = eigenstrain_reformation(&mut b, strain)?;
            let (axes, _) = ball_chord_axes(
                &mut b,
                frame.node_i.coordinates,
                frame.node_j.coordinates,
                frame.y_reference,
            )?;
            let em = b.lift(frame.section.elastic_modulus)?;
            let area = b.lift(frame.section.area)?;
            let ea = b.mul(em, area)?;
            let thrust = b.mul(ea, eps)?;
            let mut out = [Ball::ZERO; 12];
            for c in 0..3 {
                let f = b.mul(thrust, axes[0][c])?;
                out[c] = f.neg();
                out[c + 6] = f;
            }
            out
        }
        LoadFormula::FreeStrainCurved {
            arc,
            strain,
            column,
        } => {
            let eps = eigenstrain_reformation(&mut b, strain)?;
            curved_strain_balls(&mut b, arc, eps, *column)?
        }
        LoadFormula::PressureFreeStrainCurved {
            arc,
            pressure,
            outer_diameter,
            wall_thickness,
            poisson_ratio,
            column,
        } => {
            let force = pressure_force(&mut b, *pressure, *outer_diameter, *wall_thickness)?;
            let nu = b.lift(*poisson_ratio)?;
            let two = b.pow2(nu, 1)?;
            let coef = b.sub(Ball::ONE, two)?;
            let numerator = b.mul(coef, force)?;
            let e = b.lift(arc.elastic_modulus)?;
            let a = b.lift(arc.area)?;
            let ea = b.mul(e, a)?;
            let eps = b.div(numerator, ea, "pressure strain")?;
            curved_strain_balls(&mut b, arc, eps, *column)?
        }
        LoadFormula::PressureOperand {
            geometry,
            pressure,
            outer_diameter,
            wall_thickness,
            coefficient,
            direction,
        } => {
            let force = pressure_force(&mut b, *pressure, *outer_diameter, *wall_thickness)?;
            let coefficient = match coefficient {
                PressureCoefficient::Constant(c) => b.lift(*c)?,
                PressureCoefficient::Poisson {
                    poisson_ratio,
                    sign,
                } => {
                    let nu = b.lift(*poisson_ratio)?;
                    let sign = b.lift(*sign)?;
                    b.mul(nu, sign)?
                }
            };
            let thrust = b.mul(force, coefficient)?;
            let (xi, xj, yr) = match geometry {
                PressureGeometry::Straight { frame } => (
                    frame.node_i.coordinates,
                    frame.node_j.coordinates,
                    frame.y_reference,
                ),
                PressureGeometry::Curved { arc } => {
                    (arc.coordinates_i, arc.coordinates_j, arc.y_reference)
                }
            };
            let (axes, l) = ball_chord_axes(&mut b, xi, xj, yr)?;
            let dir = match *direction {
                PressureDirection::Chord { axis } if axis < 3 => axes[0][axis],
                PressureDirection::Tangent { end_j, axis } if axis < 3 => {
                    let PressureGeometry::Curved { arc } = geometry else {
                        return Err(bad_load("pressure tangent"));
                    };
                    let r = b.lift(arc.radius)?;
                    let diameter = b.pow2(r, 1)?;
                    let s = b.div(l, diameter, "pressure tangent")?;
                    let ss = b.mul(s, s)?;
                    let c2 = b.sub(Ball::ONE, ss)?;
                    let c = b.sqrt(c2, "pressure tangent")?;
                    let chord = b.mul(c, axes[0][axis])?;
                    let bow = b.mul(s, axes[1][axis])?;
                    if end_j {
                        b.sub(chord, bow)?
                    } else {
                        b.add(chord, bow)?
                    }
                }
                _ => return Err(bad_load("pressure axis")),
            };
            let value = b.mul(thrust, dir)?;
            [value; 12]
        }
        LoadFormula::JointEffectiveArea { connector, end, pressure, effective_area, axis, sign } => {
            if *end >= 2 || *axis >= 3 || !matches!(*sign, -1.0 | 1.0)
                || !effective_area.is_finite() || *effective_area <= 0.0
            {
                return Err(bad_load("joint effective-area operands"));
            }
            let pressure = b.lift(*pressure)?;
            let area = b.lift(*effective_area)?;
            let sign = b.lift(*sign)?;
            let direction = b.lift(connector.axes()[*axis][0])?;
            let thrust = b.mul(pressure, area)?;
            let signed = b.mul(thrust, sign)?;
            [b.mul(signed, direction)?; 12]
        }
        LoadFormula::ConnectorReference { connector } => {
            let mut a = WideArith::new(FORMATION_PRECISION)?;
            let bt = connector_b(&mut a, connector)?;
            let k = connector.stiffness();
            let q = connector.q_ref();
            let mut effort = [Ball::ZERO; 6];
            for r in 0..6 {
                for c in 0..6 {
                    let kv = b.lift(k[r][c])?;
                    let qv = b.lift(q[c])?;
                    let t = b.mul(kv, qv)?;
                    effort[r] = b.add(effort[r], t)?;
                }
            }
            let mut out = [Ball::ZERO; 12];
            for c in 0..12 {
                for r in 0..6 {
                    let t = b.mul(Ball::exact(bt[r][c]), effort[r])?;
                    out[c] = b.add(out[c], t)?;
                }
            }
            out
        }
        LoadFormula::TieRodReference { tie_rod, .. } => tie_rod_reference_balls(&mut b, tie_rod)?,
        LoadFormula::SlidingFrictionExplicit {
            coefficient,
            normal,
            direction,
            ..
        } => [sliding_friction_exact(&mut b, *coefficient, *normal, *direction)?; 12],
        LoadFormula::SlidingFrictionCoupled {
            coefficient,
            direction,
            beta,
            midpoint,
            radius,
            ..
        } => {
            if !(coefficient.is_finite() && *coefficient > 0.0)
                || !matches!(*direction, 1.0 | -1.0)
                || !matches!(*beta, 1.0 | -1.0)
                || !midpoint.is_finite()
                || !(radius.is_finite() && *radius >= 0.0)
            {
                return Err(bad_load("coupled sliding friction operands"));
            }
            [Ball {
                m: Wide2::from_f64(*midpoint)?,
                r: *radius,
            }; 12]
        }
    };
    for value in &mut out {
        value.m = value.m.mul_pow2(i64::from(p.force_scale_exponent))?;
        value.r = scale_up(value.r, p.force_scale_exponent);
        finite(value.r, "scaled load radius")?;
    }
    Ok((out.map(|v| v.m), out.map(|v| v.r)))
}
/// -direction*coefficient*|normal| as the exact product of two binary64
/// values (kd5-nonlinear-active-set.md §1.3): its two-product hi + lo is
/// exact whenever lo cannot underflow, and then the radius is 0. A product
/// near the subnormal range falls back to the ball product with its radius.
fn sliding_friction_exact(
    b: &mut Balls,
    coefficient: f64,
    normal: f64,
    direction: f64,
) -> Result<Ball, LoadFailure> {
    if !(coefficient.is_finite() && coefficient > 0.0)
        || !normal.is_finite()
        || !matches!(direction, 1.0 | -1.0)
    {
        return Err(bad_load("explicit sliding friction operands"));
    }
    let magnitude = normal.abs();
    let hi = coefficient * magnitude;
    let value = if hi.is_finite() && (hi == 0.0 || hi >= 2.0_f64.powi(-900)) {
        let lo = coefficient.mul_add(magnitude, -hi);
        let mut a = WideArith::new(FORMATION_PRECISION)?;
        Ball {
            m: a.add(&Wide2::from_f64(hi)?, &Wide2::from_f64(lo)?)?,
            r: 0.0,
        }
    } else {
        let c = b.lift(coefficient)?;
        let n = b.lift(magnitude)?;
        b.mul(c, n)?
    };
    Ok(if direction > 0.0 { value.neg() } else { value })
}
fn pressure_force(b: &mut Balls, p: f64, od: f64, t: f64) -> Result<Ball, LoadFailure> {
    let pi = ball_pi(b)?;
    let od = b.lift(od)?;
    let t = b.lift(t)?;
    let ro = b.half(od)?;
    let ri = b.sub(ro, t)?;
    let ri2 = b.mul(ri, ri)?;
    let area = b.mul(pi, ri2)?;
    let p = b.lift(p)?;
    b.mul(p, area)
}
fn scale_up(mut value: f64, mut exponent: i32) -> f64 {
    if value == 0.0 {
        return 0.0;
    }
    while exponent != 0 {
        let step = exponent.clamp(-512, 512);
        value = mul_up(value, 2.0_f64.powi(step));
        exponent -= step;
    }
    value
}

// --------------------------------------------------------------- re-formation

pub(super) type Element = [[Wide2; ELEMENT_DOF]; ELEMENT_DOF];
pub(super) type Vec3 = [Wide2; 3];
type M6 = [[Wide2; 6]; 6];

fn lift(x: f64) -> Result<Wide2, WideError> {
    Wide2::from_f64(x)
}
fn lift3(v: [f64; 3]) -> Result<Vec3, WideError> {
    Ok([lift(v[0])?, lift(v[1])?, lift(v[2])?])
}
fn sub3(a: &mut WideArith, x: &Vec3, y: &Vec3) -> Result<Vec3, WideError> {
    Ok([
        a.sub(&x[0], &y[0])?,
        a.sub(&x[1], &y[1])?,
        a.sub(&x[2], &y[2])?,
    ])
}
fn dot3(a: &mut WideArith, x: &Vec3, y: &Vec3) -> Result<Wide2, WideError> {
    let p0 = a.mul(&x[0], &y[0])?;
    let p1 = a.mul(&x[1], &y[1])?;
    let p2 = a.mul(&x[2], &y[2])?;
    let s = a.add(&p0, &p1)?;
    a.add(&s, &p2)
}
fn cross3(a: &mut WideArith, x: &Vec3, y: &Vec3) -> Result<Vec3, WideError> {
    let c = |a: &mut WideArith, p: usize, q: usize| -> Result<Wide2, WideError> {
        let l = a.mul(&x[p], &y[q])?;
        let r = a.mul(&x[q], &y[p])?;
        a.sub(&l, &r)
    };
    Ok([c(a, 1, 2)?, c(a, 2, 0)?, c(a, 0, 1)?])
}
fn scale3(a: &mut WideArith, x: &Vec3, d: &Wide2) -> Result<Vec3, WideError> {
    Ok([a.div(&x[0], d)?, a.div(&x[1], d)?, a.div(&x[2], d)?])
}
fn norm3(a: &mut WideArith, x: &Vec3) -> Result<Wide2, WideError> {
    let s = dot3(a, x, x)?;
    a.sqrt(&s)
}

/// Local axes (rows: x, y, z) from the actual chord and a y reference, as
/// `FrameOrientation::from_x_axis_and_y_reference` defines them, at p.
pub(super) fn chord_axes(
    a: &mut WideArith,
    xi: [f64; 3],
    xj: [f64; 3],
    y_reference: [f64; 3],
) -> Result<([Vec3; 3], Wide2), WideError> {
    let d = sub3(a, &lift3(xj)?, &lift3(xi)?)?;
    let length = norm3(a, &d)?;
    let ex = scale3(a, &d, &length)?;
    let yr = lift3(y_reference)?;
    let projection = dot3(a, &yr, &ex)?;
    let mut yc = [Wide2::ZERO; 3];
    for k in 0..3 {
        let t = a.mul(&projection, &ex[k])?;
        yc[k] = a.sub(&yr[k], &t)?;
    }
    let ym = norm3(a, &yc)?;
    let ey = scale3(a, &yc, &ym)?;
    let ez = cross3(a, &ex, &ey)?;
    Ok(([ex, ey, ez], length))
}

/// Tᵀ K T with T = diag(R, R, R, R), R's rows the local axes, at p.
fn rotate(a: &mut WideArith, k: &Element, axes: &[Vec3; 3]) -> Result<Element, WideError> {
    let mut out = [[Wide2::ZERO; ELEMENT_DOF]; ELEMENT_DOF];
    for bi in 0..4 {
        for bj in 0..4 {
            let mut empty = true;
            for p in 0..3 {
                for q in 0..3 {
                    empty &= k[3 * bi + p][3 * bj + q].is_zero();
                }
            }
            if empty {
                continue;
            }
            // temp = K_block · R
            let mut temp = [[Wide2::ZERO; 3]; 3];
            for p in 0..3 {
                for c in 0..3 {
                    let mut s = Wide2::ZERO;
                    for q in 0..3 {
                        let kv = &k[3 * bi + p][3 * bj + q];
                        if kv.is_zero() || axes[q][c].is_zero() {
                            continue;
                        }
                        let t = a.mul(kv, &axes[q][c])?;
                        s = a.add(&s, &t)?;
                    }
                    temp[p][c] = s;
                }
            }
            // out_block = Rᵀ · temp
            for r in 0..3 {
                for c in 0..3 {
                    let mut s = Wide2::ZERO;
                    for p in 0..3 {
                        if axes[p][r].is_zero() || temp[p][c].is_zero() {
                            continue;
                        }
                        let t = a.mul(&axes[p][r], &temp[p][c])?;
                        s = a.add(&s, &t)?;
                    }
                    out[3 * bi + r][3 * bj + c] = s;
                }
            }
        }
    }
    Ok(out)
}

fn small(v: f64) -> Wide2 {
    // Exact lifts of the small integers and powers of two used below.
    Wide2::from_f64(v).expect("finite constant")
}

/// A straight frame re-formed from its primitives (D1 §4.3.1 step 1): the
/// frame, L, EA/L, GJ/L, 12EI/L³, 6EI/L², 4EI/L, 2EI/L and TᵀKT, all at p.
///
/// T4-U8 (Design §4 item 1): a Timoshenko leaf forms, per plane and at p
/// from the frame's own leaves, φ = 12EI/(G·A_s·L²), 1+φ, 4+φ and 2−φ, then
/// 12EI/(L³(1+φ)), 6EI/(L²(1+φ)), (4+φ)EI/(L(1+φ)) and (2−φ)EI/(L(1+φ)) in
/// the same positions. EA/L and GJ/L are unchanged (A is the metal area).
/// Every consumer (ρ, s, s₂, ε_E, certified recovery, reactions, stress
/// spans) sees this one intended element.
pub(super) fn frame_matrix(a: &mut WideArith, e: &FrameElement) -> Result<Element, WideError> {
    let (axes, length) = chord_axes(a, e.node_i.coordinates, e.node_j.coordinates, e.y_reference)?;
    let s = &e.section;
    let (em, gm) = (lift(s.elastic_modulus)?, lift(s.shear_modulus)?);
    let l2 = a.mul(&length, &length)?;
    let l3 = a.mul(&l2, &length)?;
    let ea = a.mul(&em, &lift(s.area)?)?;
    let axial = a.div(&ea, &length)?;
    let gj = a.mul(&gm, &lift(s.torsion_constant)?)?;
    let torsion = a.div(&gj, &length)?;
    // (G·A_s)·L², the Timoshenko denominator of φ; None for Euler–Bernoulli.
    let shear = match s.beam_theory {
        BeamTheory::EulerBernoulli => None,
        BeamTheory::Timoshenko { shear_area } => {
            let gas = a.mul(&gm, &lift(shear_area)?)?;
            Some(a.mul(&gas, &l2)?)
        }
    };
    let mut coefficients = |i: f64| -> Result<[Wide2; 4], WideError> {
        let ei = a.mul(&em, &lift(i)?)?;
        let t12 = a.mul(&small(12.0), &ei)?;
        let t6 = a.mul(&small(6.0), &ei)?;
        let Some(gasl2) = shear else {
            let t4 = a.mul(&small(4.0), &ei)?;
            let t2 = a.mul(&small(2.0), &ei)?;
            return Ok([
                a.div(&t12, &l3)?,
                a.div(&t6, &l2)?,
                a.div(&t4, &length)?,
                a.div(&t2, &length)?,
            ]);
        };
        let phi = a.div(&t12, &gasl2)?;
        let one_plus_phi = a.add(&Wide2::ONE, &phi)?;
        let four_plus_phi = a.add(&small(4.0), &phi)?;
        let two_minus_phi = a.sub(&small(2.0), &phi)?;
        let d3 = a.mul(&l3, &one_plus_phi)?;
        let d2 = a.mul(&l2, &one_plus_phi)?;
        let d1 = a.mul(&length, &one_plus_phi)?;
        let t4 = a.mul(&four_plus_phi, &ei)?;
        let t2 = a.mul(&two_minus_phi, &ei)?;
        Ok([
            a.div(&t12, &d3)?,
            a.div(&t6, &d2)?,
            a.div(&t4, &d1)?,
            a.div(&t2, &d1)?,
        ])
    };
    let [z12, z6, z4, z2] = coefficients(s.second_moment_z)?;
    let [y12, y6, y4, y2] = coefficients(s.second_moment_y)?;
    let mut k = [[Wide2::ZERO; ELEMENT_DOF]; ELEMENT_DOF];
    let pair = |k: &mut Element, i: usize, j: usize, v: Wide2| {
        k[i][i] = v;
        k[j][j] = v;
        k[i][j] = v.neg();
        k[j][i] = v.neg();
    };
    pair(&mut k, 0, 6, axial);
    pair(&mut k, 3, 9, torsion);
    // FK add_bending_z (uy, rz) and add_bending_y (uz, ry) patterns.
    let bz = [
        [z12, z6, z12.neg(), z6],
        [z6, z4, z6.neg(), z2],
        [z12.neg(), z6.neg(), z12, z6.neg()],
        [z6, z2, z6.neg(), z4],
    ];
    let by = [
        [y12, y6.neg(), y12.neg(), y6.neg()],
        [y6.neg(), y4, y6, y2],
        [y12.neg(), y6, y12, y6],
        [y6.neg(), y2, y6, y4],
    ];
    for (idx, t) in [([1usize, 5, 7, 11], bz), ([2usize, 4, 8, 10], by)] {
        for r in 0..4 {
            for c in 0..4 {
                k[idx[r]][idx[c]] = t[r][c];
            }
        }
    }
    rotate(a, &k, &axes)
}

/// T4-U3 (S10): an objective connector's BᵀKB re-formed at p from the
/// binary64 decode (node coordinates, global offsets, Q and K), sharing no
/// arithmetic with `ObjectiveConnector::b` or `global_stiffness`:
/// r = (x_j − x_i) + (a_j − a_i) (never x + a), M_i = S(a_i) + S(r)/2,
/// M_j = S(r)/2 − S(a_j), B_t = Qᵀ[−I, M_i, I, M_j], B_r = Qᵀ[0, −I, 0, I],
/// then K·B and Bᵀ(K·B).
pub(super) fn connector_matrix(
    a: &mut WideArith,
    c: &ObjectiveConnector,
) -> Result<Element, WideError> {
    let b = connector_b(a, c)?;
    let stiffness = c.stiffness();
    let mut k = [[Wide2::ZERO; 6]; 6];
    for row in 0..6 {
        for col in 0..6 {
            k[row][col] = lift(stiffness[row][col])?;
        }
    }
    let mut kb = [[Wide2::ZERO; ELEMENT_DOF]; 6];
    for row in 0..6 {
        for col in 0..ELEMENT_DOF {
            let mut s = Wide2::ZERO;
            for l in 0..6 {
                if k[row][l].is_zero() || b[l][col].is_zero() {
                    continue;
                }
                let t = a.mul(&k[row][l], &b[l][col])?;
                s = a.add(&s, &t)?;
            }
            kb[row][col] = s;
        }
    }
    let mut out = [[Wide2::ZERO; ELEMENT_DOF]; ELEMENT_DOF];
    for i in 0..ELEMENT_DOF {
        for j in 0..ELEMENT_DOF {
            let mut s = Wide2::ZERO;
            for row in 0..6 {
                if b[row][i].is_zero() || kb[row][j].is_zero() {
                    continue;
                }
                let t = a.mul(&b[row][i], &kb[row][j])?;
                s = a.add(&s, &t)?;
            }
            out[i][j] = s;
        }
    }
    Ok(out)
}

fn connector_b(
    a: &mut WideArith,
    c: &ObjectiveConnector,
) -> Result<[[Wide2; ELEMENT_DOF]; 6], WideError> {
    let xi = lift3(c.node_i().coordinates)?;
    let xj = lift3(c.node_j().coordinates)?;
    let (ai, aj) = c.offsets();
    let (ai, aj) = (lift3(ai)?, lift3(aj)?);
    let d = sub3(a, &xj, &xi)?;
    let o = sub3(a, &aj, &ai)?;
    let r = [
        a.add(&d[0], &o[0])?,
        a.add(&d[1], &o[1])?,
        a.add(&d[2], &o[2])?,
    ];
    let half = [r[0].mul_pow2(-1)?, r[1].mul_pow2(-1)?, r[2].mul_pow2(-1)?];
    let skew = |v: &Vec3| -> [[Wide2; 3]; 3] {
        [
            [Wide2::ZERO, v[2].neg(), v[1]],
            [v[2], Wide2::ZERO, v[0].neg()],
            [v[1].neg(), v[0], Wide2::ZERO],
        ]
    };
    let (s_r, s_ai, s_aj) = (skew(&half), skew(&ai), skew(&aj));
    let mut m_i = [[Wide2::ZERO; 3]; 3];
    let mut m_j = [[Wide2::ZERO; 3]; 3];
    for row in 0..3 {
        for col in 0..3 {
            m_i[row][col] = a.add(&s_ai[row][col], &s_r[row][col])?;
            m_j[row][col] = a.sub(&s_r[row][col], &s_aj[row][col])?;
        }
    }
    let axes = c.axes();
    let mut q = [[Wide2::ZERO; 3]; 3];
    for row in 0..3 {
        q[row] = lift3(axes[row])?;
    }
    let mut b = [[Wide2::ZERO; ELEMENT_DOF]; 6];
    for axis in 0..3 {
        for col in 0..3 {
            let qc = q[col][axis];
            b[axis][col] = qc.neg();
            b[axis][6 + col] = qc;
            b[3 + axis][3 + col] = qc.neg();
            b[3 + axis][9 + col] = qc;
            let mut ti = Wide2::ZERO;
            let mut tj = Wide2::ZERO;
            for k in 0..3 {
                let pi = a.mul(&q[k][axis], &m_i[k][col])?;
                ti = a.add(&ti, &pi)?;
                let pj = a.mul(&q[k][axis], &m_j[k][col])?;
                tj = a.add(&tj, &pj)?;
            }
            b[axis][3 + col] = ti;
            b[axis][9 + col] = tj;
        }
    }
    Ok(b)
}

/// T4-U5b: a tie rod's C (1 × 12) at p from its authored operands: each
/// global offset o = Q·offset at p (never the binary64 fl(Q·offset)), the
/// difference d = (x_j − x_i) + (o_j − o_i) (never x + o), l = |d|, n = d/l,
/// C = [−n, −(o_i × n), n, (o_j × n)]. A zero length fails (division).
fn tie_rod_c(a: &mut WideArith, r: &TieRod) -> Result<[Wide2; ELEMENT_DOF], WideError> {
    let axes = r.axes();
    let (local_i, local_j) = r.local_offsets();
    let mut rows = [[Wide2::ZERO; 3]; 3];
    for k in 0..3 {
        rows[k] = lift3(axes[k])?;
    }
    let (local_i, local_j) = (lift3(local_i)?, lift3(local_j)?);
    let mut oi = [Wide2::ZERO; 3];
    let mut oj = [Wide2::ZERO; 3];
    for k in 0..3 {
        oi[k] = dot3(a, &rows[k], &local_i)?;
        oj[k] = dot3(a, &rows[k], &local_j)?;
    }
    let x = sub3(a, &lift3(r.node_j().coordinates)?, &lift3(r.node_i().coordinates)?)?;
    let o = sub3(a, &oj, &oi)?;
    let d = [a.add(&x[0], &o[0])?, a.add(&x[1], &o[1])?, a.add(&x[2], &o[2])?];
    let length = norm3(a, &d)?;
    let n = scale3(a, &d, &length)?;
    let arm_i = cross3(a, &oi, &n)?;
    let arm_j = cross3(a, &oj, &n)?;
    let mut c = [Wide2::ZERO; ELEMENT_DOF];
    for k in 0..3 {
        c[k] = n[k].neg();
        c[3 + k] = arm_i[k].neg();
        c[6 + k] = n[k];
        c[9 + k] = arm_j[k];
    }
    Ok(c)
}

/// T4-U5b (S10's sibling): a tie rod's Cᵀ k C re-formed at p from its
/// authored operands (`tie_rod_c`), sharing no arithmetic with
/// `TieRod::c` or `global_stiffness`. Each entry C_a·(k·C_b) is formed once
/// for a ≤ b and mirrored.
pub(super) fn tie_rod_matrix(a: &mut WideArith, r: &TieRod) -> Result<Element, WideError> {
    let c = tie_rod_c(a, r)?;
    let k = lift(r.stiffness())?;
    let mut out = [[Wide2::ZERO; ELEMENT_DOF]; ELEMENT_DOF];
    for j in 0..ELEMENT_DOF {
        if c[j].is_zero() {
            continue;
        }
        let kc = a.mul(&k, &c[j])?;
        for i in 0..=j {
            if c[i].is_zero() {
                continue;
            }
            let value = a.mul(&c[i], &kc)?;
            out[i][j] = value;
            out[j][i] = value;
        }
    }
    Ok(out)
}

/// T4-U5b: a tie rod's geometry in ball arithmetic from its authored
/// operands: C, n and l, each ball containing the exact value.
pub(super) struct TieRodBalls {
    pub(super) c: [Ball; ELEMENT_DOF],
    pub(super) direction: [Ball; 3],
}
pub(super) fn tie_rod_balls(b: &mut Balls, r: &TieRod) -> Result<TieRodBalls, LoadFailure> {
    let axes = r.axes();
    let (local_i, local_j) = r.local_offsets();
    let (local_i, local_j) = (ball_lift3(b, local_i)?, ball_lift3(b, local_j)?);
    let mut oi = [Ball::ZERO; 3];
    let mut oj = [Ball::ZERO; 3];
    for k in 0..3 {
        let row = ball_lift3(b, axes[k])?;
        oi[k] = b.dot(&row, &local_i)?;
        oj[k] = b.dot(&row, &local_j)?;
    }
    let xi = ball_lift3(b, r.node_i().coordinates)?;
    let xj = ball_lift3(b, r.node_j().coordinates)?;
    let mut d = [Ball::ZERO; 3];
    for k in 0..3 {
        let x = b.sub(xj[k], xi[k])?;
        let o = b.sub(oj[k], oi[k])?;
        d[k] = b.add(x, o)?;
    }
    let square = b.dot(&d, &d)?;
    let length = b.sqrt(square, "tie rod length")?;
    let mut n = [Ball::ZERO; 3];
    for k in 0..3 {
        n[k] = b.div(d[k], length, "tie rod direction")?;
    }
    let arm_i = b.cross(&oi, &n)?;
    let arm_j = b.cross(&oj, &n)?;
    let mut c = [Ball::ZERO; ELEMENT_DOF];
    for k in 0..3 {
        c[k] = n[k].neg();
        c[3 + k] = arm_i[k].neg();
        c[6 + k] = n[k];
        c[9 + k] = arm_j[k];
    }
    Ok(TieRodBalls { c, direction: n })
}
/// T4-U5b: +Cᵀ k e_ref in ball arithmetic (`LoadFormula::TieRodReference`).
fn tie_rod_reference_balls(b: &mut Balls, r: &TieRod) -> Result<[Ball; ELEMENT_DOF], LoadFailure> {
    let geometry = tie_rod_balls(b, r)?;
    let k = b.lift(r.stiffness())?;
    let e = b.lift(r.reference_extension())?;
    let effort = b.mul(k, e)?;
    let mut out = [Ball::ZERO; ELEMENT_DOF];
    for (slot, c) in out.iter_mut().zip(geometry.c) {
        *slot = b.mul(c, effort)?;
    }
    Ok(out)
}
/// T4-U5b: the bound on |v_a − t_a| for the binary64 reference load v
/// (`TieRod::reference_load`) and the exact t = +Cᵀ k e_ref of the authored
/// operands: |v_a − m_a| (formed at p, plus its own rounding) + r_a from
/// K-D5's own enclosure, every step rounded upward.
pub(crate) fn tie_rod_reference_load_bounds(
    r: &TieRod,
    values: &[f64; ELEMENT_DOF],
) -> Result<[f64; ELEMENT_DOF], crate::FrameKernelError> {
    const U128: f64 = f64::from_bits(895_u64 << 52); // 2^-128
    let range = |_| crate::FrameKernelError::NumericalRange {
        name: "tie rod reference load bound",
    };
    let mut b = Balls::new().map_err(range)?;
    let balls = tie_rod_reference_balls(&mut b, r).map_err(range)?;
    let mut a = WideArith::new(FORMATION_PRECISION).map_err(|_| crate::FrameKernelError::NumericalRange {
        name: "tie rod reference load bound",
    })?;
    let mut bounds = [0.0; ELEMENT_DOF];
    for (slot, (ball, &value)) in bounds.iter_mut().zip(balls.iter().zip(values)) {
        let held = Wide2::from_f64(value).map_err(|_| crate::FrameKernelError::NumericalRange {
            name: "tie rod reference load bound",
        })?;
        let gap = a.sub(&held, &ball.m).map_err(|_| crate::FrameKernelError::NumericalRange {
            name: "tie rod reference load bound",
        })?;
        let gap = abs_up(&gap).map_err(range)?;
        let bound = add_up(add_up(gap, mul_up(U128, gap)), ball.r);
        if !bound.is_finite() {
            return Err(crate::FrameKernelError::NumericalRange {
                name: "tie rod reference load bound",
            });
        }
        *slot = bound;
    }
    Ok(bounds)
}

struct IntendedCurvedGeometry {
    d: Vec3,
    radius: Wide2,
    length: Wide2,
    half_sin: Wide2,
    half_cos: Wide2,
    sin: Wide2,
    one_minus_cos: Wide2,
    cos: Wide2,
    phi: Wide2,
}
fn curved_geometry_values(
    a: &mut WideArith,
    e: &CurvedFormation,
) -> Result<IntendedCurvedGeometry, WideError> {
    let xi = lift3(e.coordinates_i)?;
    let xj = lift3(e.coordinates_j)?;
    let radius = lift(e.radius)?;
    // 1. d, L, s = sin(φ/2) = L/2R and c = cos(φ/2) = √(4R² − L²)/2R.
    let d = sub3(a, &xj, &xi)?;
    let length_squared = dot3(a, &d, &d)?;
    let length = a.sqrt(&length_squared)?;
    let diameter = radius.mul_pow2(1)?;
    let r2 = a.mul(&radius, &radius)?;
    let four_r2 = r2.mul_pow2(2)?;
    let span = a.sub(&four_r2, &length_squared)?;
    if span.is_zero() || span.is_sign_negative() {
        return Err(WideError::AngleDomain);
    }
    let half_sin = a.div(&length, &diameter)?;
    let span_root = a.sqrt(&span)?;
    let half_cos = a.div(&span_root, &diameter)?;
    // 2. sin φ = 2sc, cos φ = 1 − 2s² and the stable 1 − cos φ = 2s².
    let one = Wide2::ONE;
    let sc_half = a.mul(&half_sin, &half_cos)?;
    let sin = sc_half.mul_pow2(1)?;
    let ss_half = a.mul(&half_sin, &half_sin)?;
    let one_minus_cos = ss_half.mul_pow2(1)?;
    let cos = a.sub(&one, &one_minus_cos)?;
    // 3. φ by the K3a included-angle arctangent.
    let phi = a.included_angle(&sin, &cos)?;
    Ok(IntendedCurvedGeometry {
        d,
        radius,
        length,
        half_sin,
        half_cos,
        sin,
        one_minus_cos,
        cos,
        phi,
    })
}
fn curved_radius_angle(
    a: &mut WideArith,
    e: &CurvedFormation,
) -> Result<(Wide2, Wide2), WideError> {
    let geometry = curved_geometry_values(a, e)?;
    Ok((geometry.radius, geometry.phi))
}

/// A realized curved bend re-formed as the objective element of T4-U1
/// (T4-I6 U1_REFERENCE B1) from (d = x_j − x_i, R, y_reference), at p.
pub(super) fn curved_matrix(a: &mut WideArith, e: &CurvedFormation) -> Result<Element, WideError> {
    let IntendedCurvedGeometry {
        d,
        radius,
        length,
        half_sin,
        half_cos,
        sin,
        one_minus_cos,
        cos,
        phi,
    } = curved_geometry_values(a, e)?;
    let one = Wide2::ONE;
    // 4. The closed-form end flexibility at p, and its inverse.
    let sc = a.mul(&sin, &cos)?;
    let sin2 = sc.mul_pow2(1)?; // sin 2φ = 2 sin φ cos φ
    let ss = a.mul(&sin, &sin)?;
    let half_ss = ss.mul_pow2(-1)?;
    let half_phi = phi.mul_pow2(-1)?;
    let quarter_sin2 = sin2.mul_pow2(-2)?;
    let gram = [
        [phi, sin, one_minus_cos],
        [sin, a.add(&half_phi, &quarter_sin2)?, half_ss],
        [one_minus_cos, half_ss, a.sub(&half_phi, &quarter_sin2)?],
    ];
    let cases = unit_load_actions(a, &radius, &sin, &cos)?;
    let em = lift(e.elastic_modulus)?;
    let bending = a.mul(&em, &lift(e.second_moment)?)?;
    let torsion = a.mul(&lift(e.shear_modulus)?, &lift(e.torsion_constant)?)?;
    let axial = a.mul(&em, &lift(e.area)?)?;
    let fin = lift(e.in_plane_flexibility_factor)?;
    let fout = lift(e.out_of_plane_flexibility_factor)?;
    let mut flexibility = [[Wide2::ZERO; 6]; 6];
    for row in 0..6 {
        for col in row..6 {
            let q_in = quad(a, &gram, &cases[row][0], &cases[col][0])?;
            let q_out = quad(a, &gram, &cases[row][1], &cases[col][1])?;
            let q_t = quad(a, &gram, &cases[row][2], &cases[col][2])?;
            let q_a = quad(a, &gram, &cases[row][3], &cases[col][3])?;
            let t_in = a.mul(&fin, &q_in)?;
            let t_in = a.div(&t_in, &bending)?;
            let t_out = a.mul(&fout, &q_out)?;
            let t_out = a.div(&t_out, &bending)?;
            let t_t = a.div(&q_t, &torsion)?;
            let t_a = a.div(&q_a, &axial)?;
            let s = a.add(&t_in, &t_out)?;
            let s = a.add(&s, &t_t)?;
            let s = a.add(&s, &t_a)?;
            let v = a.mul(&radius, &s)?;
            flexibility[row][col] = v;
            flexibility[col][row] = v;
        }
    }
    let tip = invert6(a, &flexibility)?;
    // 5. Local axes: d̂ = d/L, n̂ the unit component of y normal to d̂ (the
    //    bow side); x = −s·d̂ + c·n̂ (radial at node i), y = c·d̂ + s·n̂,
    //    z = n̂ × d̂.
    let dh = scale3(a, &d, &length)?;
    let yr = lift3(e.y_reference)?;
    let projection = dot3(a, &yr, &dh)?;
    let mut normal = [Wide2::ZERO; 3];
    for k in 0..3 {
        let t = a.mul(&projection, &dh[k])?;
        normal[k] = a.sub(&yr[k], &t)?;
    }
    let normal_length = norm3(a, &normal)?;
    let nh = scale3(a, &normal, &normal_length)?;
    let mut ex = [Wide2::ZERO; 3];
    let mut ey = [Wide2::ZERO; 3];
    for k in 0..3 {
        let sd = a.mul(&half_sin, &dh[k])?;
        let cn = a.mul(&half_cos, &nh[k])?;
        ex[k] = a.sub(&cn, &sd)?;
        let cd = a.mul(&half_cos, &dh[k])?;
        let sn = a.mul(&half_sin, &nh[k])?;
        ey[k] = a.add(&cd, &sn)?;
    }
    let ez = cross3(a, &nh, &dh)?;
    let axes = [ex, ey, ez];
    // 6. H from the actual chord x_j − x_i in the local axes.
    let chord = [
        dot3(a, &axes[0], &d)?,
        dot3(a, &axes[1], &d)?,
        dot3(a, &axes[2], &d)?,
    ];
    let mut h = [[Wide2::ZERO; 6]; 6];
    for (i, row) in h.iter_mut().enumerate() {
        row[i] = one;
    }
    h[3][1] = chord[2].neg();
    h[3][2] = chord[1];
    h[4][0] = chord[2];
    h[4][2] = chord[0].neg();
    h[5][0] = chord[1].neg();
    h[5][1] = chord[0];
    let coupled = mul6(a, &h, &tip, false)?;
    let anchored = mul6(a, &coupled, &h, true)?;
    let mut k = [[Wide2::ZERO; ELEMENT_DOF]; ELEMENT_DOF];
    for r in 0..6 {
        for c in 0..6 {
            k[r][c] = anchored[r][c];
            k[r][c + 6] = coupled[r][c].neg();
            k[r + 6][c] = coupled[c][r].neg();
            k[r + 6][c + 6] = tip[r][c];
        }
    }
    rotate(a, &k, &axes)
}

/// The product's `unit_load_actions` series (in-plane, out-of-plane,
/// torsion, axial) for the six unit loads at node j, from R, sin φ and cos φ.
fn unit_load_actions(
    a: &mut WideArith,
    radius: &Wide2,
    sin: &Wide2,
    cos: &Wide2,
) -> Result<[[Vec3; 4]; 6], WideError> {
    let z = Wide2::ZERO;
    let one = Wide2::ONE;
    let r = *radius;
    let rs = a.mul(&r, sin)?;
    let rc = a.mul(&r, cos)?;
    let zero3 = [z, z, z];
    Ok([
        [[rs.neg(), z, r], zero3, zero3, [z, z, one.neg()]],
        [[rc, r.neg(), z], zero3, zero3, [z, one, z]],
        [zero3, [z, rs, rc.neg()], [r, rc.neg(), rs.neg()], zero3],
        [zero3, [z, one, z], [z, z, one.neg()], zero3],
        [zero3, [z, z, one], [z, one, z], zero3],
        [[one, z, z], zero3, zero3, zero3],
    ])
}

fn quad(a: &mut WideArith, g: &[Vec3; 3], left: &Vec3, right: &Vec3) -> Result<Wide2, WideError> {
    let mut s = Wide2::ZERO;
    for r in 0..3 {
        if left[r].is_zero() {
            continue;
        }
        for c in 0..3 {
            if right[c].is_zero() || g[r][c].is_zero() {
                continue;
            }
            let t = a.mul(&left[r], &g[r][c])?;
            let t = a.mul(&t, &right[c])?;
            s = a.add(&s, &t)?;
        }
    }
    Ok(s)
}

fn mul6(a: &mut WideArith, x: &M6, y: &M6, transpose_right: bool) -> Result<M6, WideError> {
    let mut out = [[Wide2::ZERO; 6]; 6];
    for r in 0..6 {
        for c in 0..6 {
            let mut s = Wide2::ZERO;
            for k in 0..6 {
                let yv = if transpose_right { &y[c][k] } else { &y[k][c] };
                if x[r][k].is_zero() || yv.is_zero() {
                    continue;
                }
                let t = a.mul(&x[r][k], yv)?;
                s = a.add(&s, &t)?;
            }
            out[r][c] = s;
        }
    }
    Ok(out)
}

/// Gauss–Jordan inverse at p with partial pivoting, then the symmetric part.
fn invert6(a: &mut WideArith, f: &M6) -> Result<M6, WideError> {
    let mut m = *f;
    let mut inv = [[Wide2::ZERO; 6]; 6];
    for (i, row) in inv.iter_mut().enumerate() {
        row[i] = Wide2::ONE;
    }
    for col in 0..6 {
        let mut pivot = col;
        for r in col + 1..6 {
            if m[r][col].abs().cmp_value(&m[pivot][col].abs()).is_gt() {
                pivot = r;
            }
        }
        m.swap(col, pivot);
        inv.swap(col, pivot);
        let d = m[col][col];
        for c in 0..6 {
            m[col][c] = a.div(&m[col][c], &d)?;
            inv[col][c] = a.div(&inv[col][c], &d)?;
        }
        for r in 0..6 {
            if r == col || m[r][col].is_zero() {
                continue;
            }
            let factor = m[r][col];
            for c in 0..6 {
                let t = a.mul(&factor, &m[col][c])?;
                m[r][c] = a.sub(&m[r][c], &t)?;
                let t = a.mul(&factor, &inv[col][c])?;
                inv[r][c] = a.sub(&inv[r][c], &t)?;
            }
        }
    }
    let mut out = inv;
    for r in 0..6 {
        for c in r + 1..6 {
            let s = a.add(&inv[r][c], &inv[c][r])?;
            let v = s.mul_pow2(-1)?;
            out[r][c] = v;
            out[c][r] = v;
        }
    }
    Ok(out)
}

#[cfg(test)]
#[path = "formation_check_tests.rs"]
mod tests;
#[cfg(test)]
#[path = "formation_check_timoshenko_tests.rs"]
mod timoshenko_tests;
#[cfg(test)]
#[path = "tie_rod_kd5_tests.rs"]
mod tie_rod_tests;
