//! Code-neutral concentrated and distributed user load application.
//!
//! This crate prepares explicit user loads for solver-boundary consumers. It
//! does not encode design-code load combinations, public default factors,
//! protected standards content, proprietary project data, rule-pack checks, or
//! professional approval.

use open_pipe_stress_frame_kernel::{DOF_PER_NODE, ELEMENT_DOF, RX, RY, RZ, UX, UY, UZ};
use open_pipe_stress_primitive_loads::{
    BoundaryMetadataError, BoundaryQuantityRecord, BoundaryRecordRef, CanonicalSchemaBinding,
    LoadDimension, PrimitiveAxialEffectContribution, QuantityUnitMetadata,
};
use open_pipe_stress_straight_pipe::{
    GlobalPointForce, SpannedGlobalUniformLoad, StraightPipeAxialEffect, StraightPipeElement,
    UniformLoadSpan,
};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserLoadKind {
    ConcentratedForce,
    ConcentratedMoment,
    UniformDistributedLoad,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserLoadDirection {
    GlobalX,
    GlobalY,
    GlobalZ,
    RotationX,
    RotationY,
    RotationZ,
}

impl UserLoadDirection {
    pub fn dof_index(self) -> usize {
        match self {
            Self::GlobalX => UX,
            Self::GlobalY => UY,
            Self::GlobalZ => UZ,
            Self::RotationX => RX,
            Self::RotationY => RY,
            Self::RotationZ => RZ,
        }
    }

    pub fn is_rotational(self) -> bool {
        matches!(self, Self::RotationX | Self::RotationY | Self::RotationZ)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UserLoadQuantity {
    pub value: f64,
    pub dimension: LoadDimension,
}

impl UserLoadQuantity {
    pub fn new(value: f64, dimension: LoadDimension) -> Result<Self, UserLoadError> {
        validate_finite("user load quantity", value)?;
        Ok(Self { value, dimension })
    }

    pub fn to_model_load_boundary_record(
        &self,
        record: BoundaryRecordRef,
        unit: QuantityUnitMetadata,
        provenance_ref: impl Into<String>,
    ) -> Result<BoundaryQuantityRecord, BoundaryMetadataError> {
        require_schema_binding(
            record.schema_binding,
            CanonicalSchemaBinding::ModelLoadRecord,
        )?;
        BoundaryQuantityRecord::from_load_dimension(
            record,
            self.value,
            self.dimension,
            unit,
            provenance_ref,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserLoadTarget {
    Node(usize),
    Element {
        element_index: usize,
        span: ElementLoadSpan,
        element_length: Option<f64>,
    },
    ElementStation {
        element_index: usize,
        station_fraction: f64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElementLoadSpan {
    pub start_fraction: f64,
    pub end_fraction: f64,
}

impl ElementLoadSpan {
    pub fn full() -> Self {
        Self {
            start_fraction: 0.0,
            end_fraction: 1.0,
        }
    }

    pub fn new(start_fraction: f64, end_fraction: f64) -> Result<Self, UserLoadError> {
        validate_finite("span start fraction", start_fraction)?;
        validate_finite("span end fraction", end_fraction)?;
        Ok(Self {
            start_fraction,
            end_fraction,
        })
    }

    pub fn length_fraction(&self) -> f64 {
        self.end_fraction - self.start_fraction
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserLoad {
    pub load_id: String,
    pub kind: UserLoadKind,
    pub target: Option<UserLoadTarget>,
    pub direction: UserLoadDirection,
    pub quantity: Option<UserLoadQuantity>,
    pub provenance_ref: Option<String>,
}

impl UserLoad {
    pub fn concentrated_force(
        load_id: impl Into<String>,
        node_index: usize,
        direction: UserLoadDirection,
        quantity: UserLoadQuantity,
    ) -> Self {
        Self {
            load_id: load_id.into(),
            kind: UserLoadKind::ConcentratedForce,
            target: Some(UserLoadTarget::Node(node_index)),
            direction,
            quantity: Some(quantity),
            provenance_ref: None,
        }
    }

    pub fn concentrated_moment(
        load_id: impl Into<String>,
        node_index: usize,
        direction: UserLoadDirection,
        quantity: UserLoadQuantity,
    ) -> Self {
        Self {
            load_id: load_id.into(),
            kind: UserLoadKind::ConcentratedMoment,
            target: Some(UserLoadTarget::Node(node_index)),
            direction,
            quantity: Some(quantity),
            provenance_ref: None,
        }
    }

    pub fn uniform_distributed(
        load_id: impl Into<String>,
        element_index: usize,
        direction: UserLoadDirection,
        quantity: UserLoadQuantity,
        span: ElementLoadSpan,
        element_length: Option<f64>,
    ) -> Self {
        Self {
            load_id: load_id.into(),
            kind: UserLoadKind::UniformDistributedLoad,
            target: Some(UserLoadTarget::Element {
                element_index,
                span,
                element_length,
            }),
            direction,
            quantity: Some(quantity),
            provenance_ref: None,
        }
    }

    pub fn element_concentrated_force(
        load_id: impl Into<String>,
        element_index: usize,
        station_fraction: f64,
        direction: UserLoadDirection,
        quantity: UserLoadQuantity,
    ) -> Self {
        Self {
            load_id: load_id.into(),
            kind: UserLoadKind::ConcentratedForce,
            target: Some(UserLoadTarget::ElementStation {
                element_index,
                station_fraction,
            }),
            direction,
            quantity: Some(quantity),
            provenance_ref: None,
        }
    }

    pub fn with_provenance_ref(mut self, provenance_ref: impl Into<String>) -> Self {
        self.provenance_ref = Some(provenance_ref.into());
        self
    }

    pub fn missing_target(
        load_id: impl Into<String>,
        kind: UserLoadKind,
        direction: UserLoadDirection,
        quantity: Option<UserLoadQuantity>,
    ) -> Self {
        Self {
            load_id: load_id.into(),
            kind,
            target: None,
            direction,
            quantity,
            provenance_ref: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContributionTrace {
    pub load_id: String,
    pub kind: UserLoadKind,
    pub provenance_ref: Option<String>,
}

impl ContributionTrace {
    fn from_load(load: &UserLoad) -> Self {
        Self {
            load_id: load.load_id.clone(),
            kind: load.kind,
            provenance_ref: load.provenance_ref.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodalLoadContribution {
    pub node_index: usize,
    pub global_dof: usize,
    pub value: f64,
    pub dimension: LoadDimension,
    pub trace: ContributionTrace,
}

impl NodalLoadContribution {
    pub fn to_model_load_boundary_record(
        &self,
        record: BoundaryRecordRef,
        unit: QuantityUnitMetadata,
    ) -> Result<BoundaryQuantityRecord, BoundaryMetadataError> {
        require_schema_binding(
            record.schema_binding,
            CanonicalSchemaBinding::ModelLoadRecord,
        )?;
        BoundaryQuantityRecord::from_load_dimension(
            record,
            self.value,
            self.dimension,
            unit,
            self.trace
                .provenance_ref
                .as_deref()
                .unwrap_or(self.trace.load_id.as_str()),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElementDistributedLoadContribution {
    pub element_index: usize,
    pub direction: UserLoadDirection,
    pub magnitude: UserLoadQuantity,
    pub span: ElementLoadSpan,
    pub equivalent_total: Option<f64>,
    pub trace: ContributionTrace,
}

impl ElementDistributedLoadContribution {
    pub fn to_model_load_boundary_record(
        &self,
        record: BoundaryRecordRef,
        unit: QuantityUnitMetadata,
    ) -> Result<BoundaryQuantityRecord, BoundaryMetadataError> {
        require_schema_binding(
            record.schema_binding,
            CanonicalSchemaBinding::ModelLoadRecord,
        )?;
        BoundaryQuantityRecord::from_load_dimension(
            record,
            self.magnitude.value,
            self.magnitude.dimension,
            unit,
            self.trace
                .provenance_ref
                .as_deref()
                .unwrap_or(self.trace.load_id.as_str()),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryHookKind {
    NodalForce,
    NodalMoment,
    ElementDistributedLoad,
    ElementAxialEffect,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResultRecoveryHook {
    pub load_id: String,
    pub hook_kind: RecoveryHookKind,
    pub target_ref: String,
    pub dimension: LoadDimension,
    pub provenance_ref: Option<String>,
}

impl ResultRecoveryHook {
    pub fn to_result_boundary_record(
        &self,
        value: f64,
        record: BoundaryRecordRef,
        unit: QuantityUnitMetadata,
    ) -> Result<BoundaryQuantityRecord, BoundaryMetadataError> {
        match record.schema_binding {
            CanonicalSchemaBinding::ModelResultValue
            | CanonicalSchemaBinding::ResultsQuantityResult => {
                BoundaryQuantityRecord::from_load_dimension(
                    record,
                    value,
                    self.dimension,
                    unit,
                    self.provenance_ref
                        .as_deref()
                        .unwrap_or(self.load_id.as_str()),
                )
            }
            CanonicalSchemaBinding::ModelLoadRecord => {
                Err(BoundaryMetadataError::SchemaBindingMismatch {
                    expected: CanonicalSchemaBinding::ModelResultValue,
                    actual: CanonicalSchemaBinding::ModelLoadRecord,
                })
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingCode {
    MissingLoadTarget,
    MissingLoadQuantity,
    NodeOutOfRange,
    ElementOutOfRange,
    InvalidLoadDimension,
    InvalidLoadDirection,
    UnsupportedTargetForLoadKind,
    InvalidDistributionSpan,
    NonPositiveElementLength,
    MissingElementGeometry,
    NonFiniteAxialEffect,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserLoadFinding {
    pub code: FindingCode,
    pub load_id: String,
    pub message: String,
}

impl UserLoadFinding {
    fn new(code: FindingCode, load_id: &str, message: impl Into<String>) -> Self {
        Self {
            code,
            load_id: load_id.to_string(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserLoadApplication {
    pub nodal_loads: Vec<NodalLoadContribution>,
    pub element_distributed_loads: Vec<ElementDistributedLoadContribution>,
    pub recovery_hooks: Vec<ResultRecoveryHook>,
    pub findings: Vec<UserLoadFinding>,
}

impl UserLoadApplication {
    pub fn is_blocked(&self) -> bool {
        !self.findings.is_empty()
    }

    pub fn global_load_vector(&self, node_count: usize) -> Vec<f64> {
        let mut vector = vec![0.0; node_count * DOF_PER_NODE];
        for load in &self.nodal_loads {
            if load.global_dof < vector.len() {
                vector[load.global_dof] += load.value;
            }
        }
        vector
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserLoadError {
    NonFiniteInput { name: &'static str, value: f64 },
}

impl fmt::Display for UserLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteInput { name, value } => {
                write!(f, "{name} must be finite, got {value}")
            }
        }
    }
}

impl Error for UserLoadError {}

pub fn apply_user_loads(
    node_count: usize,
    element_count: usize,
    loads: &[UserLoad],
) -> UserLoadApplication {
    let mut nodal_loads = Vec::new();
    let mut element_distributed_loads = Vec::new();
    let mut recovery_hooks = Vec::new();
    let mut findings = Vec::new();

    for load in loads {
        let Some(target) = &load.target else {
            findings.push(UserLoadFinding::new(
                FindingCode::MissingLoadTarget,
                &load.load_id,
                "user load requires an explicit target",
            ));
            continue;
        };
        let Some(quantity) = load.quantity else {
            findings.push(UserLoadFinding::new(
                FindingCode::MissingLoadQuantity,
                &load.load_id,
                "user load requires an explicit quantity",
            ));
            continue;
        };

        match target {
            UserLoadTarget::Node(node_index) => prepare_nodal_user_load(
                load,
                *node_index,
                quantity,
                node_count,
                &mut nodal_loads,
                &mut recovery_hooks,
                &mut findings,
            ),
            UserLoadTarget::Element {
                element_index,
                span,
                element_length,
            } => prepare_element_user_load(
                load,
                *element_index,
                quantity,
                *span,
                *element_length,
                element_count,
                &mut element_distributed_loads,
                &mut recovery_hooks,
                &mut findings,
            ),
            UserLoadTarget::ElementStation { .. } => findings.push(UserLoadFinding::new(
                FindingCode::MissingElementGeometry,
                &load.load_id,
                "element-station user loads require straight-pipe equivalent load recovery",
            )),
        }
    }

    nodal_loads.sort_by(|a, b| {
        a.node_index
            .cmp(&b.node_index)
            .then(a.global_dof.cmp(&b.global_dof))
            .then(a.trace.load_id.cmp(&b.trace.load_id))
    });
    element_distributed_loads.sort_by(|a, b| {
        a.element_index
            .cmp(&b.element_index)
            .then(a.direction.dof_index().cmp(&b.direction.dof_index()))
            .then(a.trace.load_id.cmp(&b.trace.load_id))
    });
    recovery_hooks.sort_by(|a, b| {
        a.target_ref
            .cmp(&b.target_ref)
            .then(a.load_id.cmp(&b.load_id))
    });

    UserLoadApplication {
        nodal_loads,
        element_distributed_loads,
        recovery_hooks,
        findings,
    }
}

pub fn apply_straight_pipe_equivalent_user_loads(
    element_index: usize,
    pipe: &StraightPipeElement,
    loads: &[UserLoad],
) -> UserLoadApplication {
    let node_count = pipe.node_i.index.max(pipe.node_j.index) + 1;
    let mut nodal_loads = Vec::new();
    let mut element_distributed_loads = Vec::new();
    let mut recovery_hooks = Vec::new();
    let mut findings = Vec::new();

    for load in loads {
        let Some(target) = &load.target else {
            findings.push(UserLoadFinding::new(
                FindingCode::MissingLoadTarget,
                &load.load_id,
                "user load requires an explicit target",
            ));
            continue;
        };
        let Some(quantity) = load.quantity else {
            findings.push(UserLoadFinding::new(
                FindingCode::MissingLoadQuantity,
                &load.load_id,
                "user load requires an explicit quantity",
            ));
            continue;
        };

        match target {
            UserLoadTarget::Node(node_index) => prepare_nodal_user_load(
                load,
                *node_index,
                quantity,
                node_count,
                &mut nodal_loads,
                &mut recovery_hooks,
                &mut findings,
            ),
            UserLoadTarget::Element {
                element_index: target_element,
                span,
                element_length,
            } => prepare_straight_pipe_distributed_load(
                load,
                *target_element,
                quantity,
                *span,
                *element_length,
                element_index,
                pipe,
                &mut nodal_loads,
                &mut element_distributed_loads,
                &mut recovery_hooks,
                &mut findings,
            ),
            UserLoadTarget::ElementStation {
                element_index: target_element,
                station_fraction,
            } => prepare_straight_pipe_point_force(
                load,
                *target_element,
                *station_fraction,
                quantity,
                element_index,
                pipe,
                &mut nodal_loads,
                &mut recovery_hooks,
                &mut findings,
            ),
        }
    }

    nodal_loads.sort_by(|a, b| {
        a.node_index
            .cmp(&b.node_index)
            .then(a.global_dof.cmp(&b.global_dof))
            .then(a.trace.load_id.cmp(&b.trace.load_id))
    });
    element_distributed_loads.sort_by(|a, b| {
        a.element_index
            .cmp(&b.element_index)
            .then(a.direction.dof_index().cmp(&b.direction.dof_index()))
            .then(a.trace.load_id.cmp(&b.trace.load_id))
    });
    recovery_hooks.sort_by(|a, b| {
        a.target_ref
            .cmp(&b.target_ref)
            .then(a.load_id.cmp(&b.load_id))
    });

    UserLoadApplication {
        nodal_loads,
        element_distributed_loads,
        recovery_hooks,
        findings,
    }
}

pub fn apply_straight_pipe_equivalent_user_loads_with_axial_effects(
    element_index: usize,
    pipe: &StraightPipeElement,
    loads: &[UserLoad],
    axial_effects: &[PrimitiveAxialEffectContribution],
) -> UserLoadApplication {
    let mut application = apply_straight_pipe_equivalent_user_loads(element_index, pipe, loads);

    for contribution in axial_effects {
        prepare_straight_pipe_axial_effect_load(
            contribution,
            element_index,
            pipe,
            &mut application.nodal_loads,
            &mut application.recovery_hooks,
            &mut application.findings,
        );
    }

    application.nodal_loads.sort_by(|a, b| {
        a.node_index
            .cmp(&b.node_index)
            .then(a.global_dof.cmp(&b.global_dof))
            .then(a.trace.load_id.cmp(&b.trace.load_id))
    });
    application.recovery_hooks.sort_by(|a, b| {
        a.target_ref
            .cmp(&b.target_ref)
            .then(a.load_id.cmp(&b.load_id))
    });

    application
}

fn prepare_straight_pipe_axial_effect_load(
    contribution: &PrimitiveAxialEffectContribution,
    expected_element: usize,
    pipe: &StraightPipeElement,
    nodal_loads: &mut Vec<NodalLoadContribution>,
    recovery_hooks: &mut Vec<ResultRecoveryHook>,
    findings: &mut Vec<UserLoadFinding>,
) {
    if contribution.element_index != expected_element {
        findings.push(UserLoadFinding::new(
            FindingCode::ElementOutOfRange,
            &contribution.load_id,
            format!(
                "axial-effect element index {} does not match straight-pipe element {expected_element}",
                contribution.element_index
            ),
        ));
        return;
    }

    let axial_effect = match StraightPipeAxialEffect::new(contribution.axial_force) {
        Ok(axial_effect) => axial_effect,
        Err(error) => {
            findings.push(UserLoadFinding::new(
                FindingCode::NonFiniteAxialEffect,
                &contribution.load_id,
                error.to_string(),
            ));
            return;
        }
    };
    let equivalent = match pipe.equivalent_global_axial_effect_loads(&[axial_effect]) {
        Ok(loads) => loads,
        Err(error) => {
            findings.push(UserLoadFinding::new(
                FindingCode::MissingElementGeometry,
                &contribution.load_id,
                error.to_string(),
            ));
            return;
        }
    };
    let trace = ContributionTrace {
        load_id: contribution.load_id.clone(),
        kind: UserLoadKind::ConcentratedForce,
        provenance_ref: None,
    };
    push_equivalent_nodal_loads_with_trace(pipe, &equivalent, trace, nodal_loads);
    recovery_hooks.push(ResultRecoveryHook {
        load_id: contribution.load_id.clone(),
        hook_kind: RecoveryHookKind::ElementAxialEffect,
        target_ref: format!(
            "element:{}:axial-effect:{}",
            contribution.element_index, contribution.load_id
        ),
        dimension: LoadDimension::Force,
        provenance_ref: None,
    });
}

fn prepare_straight_pipe_distributed_load(
    load: &UserLoad,
    target_element: usize,
    quantity: UserLoadQuantity,
    span: ElementLoadSpan,
    element_length: Option<f64>,
    expected_element: usize,
    pipe: &StraightPipeElement,
    nodal_loads: &mut Vec<NodalLoadContribution>,
    element_distributed_loads: &mut Vec<ElementDistributedLoadContribution>,
    recovery_hooks: &mut Vec<ResultRecoveryHook>,
    findings: &mut Vec<UserLoadFinding>,
) {
    if target_element != expected_element {
        findings.push(UserLoadFinding::new(
            FindingCode::ElementOutOfRange,
            &load.load_id,
            format!("element index {target_element} does not match straight-pipe element {expected_element}"),
        ));
        return;
    }
    if load.kind != UserLoadKind::UniformDistributedLoad {
        findings.push(UserLoadFinding::new(
            FindingCode::UnsupportedTargetForLoadKind,
            &load.load_id,
            "straight-pipe element spans are limited to distributed user loads",
        ));
        return;
    }
    let span = match UniformLoadSpan::new(span.start_fraction, span.end_fraction) {
        Ok(span) => span,
        Err(error) => {
            findings.push(UserLoadFinding::new(
                FindingCode::InvalidDistributionSpan,
                &load.load_id,
                error.to_string(),
            ));
            return;
        }
    };
    let Some(force_per_length) = global_vector_for_direction(load.direction, quantity.value) else {
        findings.push(UserLoadFinding::new(
            FindingCode::InvalidLoadDirection,
            &load.load_id,
            "straight-pipe distributed loads require a translational direction",
        ));
        return;
    };
    if quantity.dimension != LoadDimension::ForcePerLength {
        findings.push(UserLoadFinding::new(
            FindingCode::InvalidLoadDimension,
            &load.load_id,
            "straight-pipe distributed load recovery requires force per length",
        ));
        return;
    }
    if let Some(length) = element_length {
        match pipe.length() {
            Ok(pipe_length) if (pipe_length - length).abs() <= 1.0e-9 => {}
            Ok(_) => {
                findings.push(UserLoadFinding::new(
                    FindingCode::MissingElementGeometry,
                    &load.load_id,
                    "supplied element length does not match straight-pipe geometry",
                ));
                return;
            }
            Err(error) => {
                findings.push(UserLoadFinding::new(
                    FindingCode::MissingElementGeometry,
                    &load.load_id,
                    error.to_string(),
                ));
                return;
            }
        }
    }

    let uniform_load =
        SpannedGlobalUniformLoad::new(force_per_length, span).expect("quantity and span are valid");
    let equivalent = match pipe.equivalent_global_nodal_loads_with_spans(&[uniform_load], &[]) {
        Ok(loads) => loads,
        Err(error) => {
            findings.push(UserLoadFinding::new(
                FindingCode::MissingElementGeometry,
                &load.load_id,
                error.to_string(),
            ));
            return;
        }
    };
    push_equivalent_nodal_loads(pipe, load, &equivalent, nodal_loads);
    element_distributed_loads.push(ElementDistributedLoadContribution {
        element_index: target_element,
        direction: load.direction,
        magnitude: quantity,
        span: ElementLoadSpan {
            start_fraction: span.start_fraction,
            end_fraction: span.end_fraction,
        },
        equivalent_total: pipe
            .length()
            .ok()
            .map(|length| quantity.value * length * span.length_fraction()),
        trace: ContributionTrace::from_load(load),
    });
    recovery_hooks.push(ResultRecoveryHook {
        load_id: load.load_id.clone(),
        hook_kind: RecoveryHookKind::ElementDistributedLoad,
        target_ref: format!(
            "element:{target_element}:span:{}-{}",
            span.start_fraction, span.end_fraction
        ),
        dimension: quantity.dimension,
        provenance_ref: load.provenance_ref.clone(),
    });
}

fn prepare_straight_pipe_point_force(
    load: &UserLoad,
    target_element: usize,
    station_fraction: f64,
    quantity: UserLoadQuantity,
    expected_element: usize,
    pipe: &StraightPipeElement,
    nodal_loads: &mut Vec<NodalLoadContribution>,
    recovery_hooks: &mut Vec<ResultRecoveryHook>,
    findings: &mut Vec<UserLoadFinding>,
) {
    if target_element != expected_element {
        findings.push(UserLoadFinding::new(
            FindingCode::ElementOutOfRange,
            &load.load_id,
            format!("element index {target_element} does not match straight-pipe element {expected_element}"),
        ));
        return;
    }
    if load.kind != UserLoadKind::ConcentratedForce {
        findings.push(UserLoadFinding::new(
            FindingCode::UnsupportedTargetForLoadKind,
            &load.load_id,
            "straight-pipe station recovery currently supports concentrated forces only",
        ));
        return;
    }
    let Some(force) = global_vector_for_direction(load.direction, quantity.value) else {
        findings.push(UserLoadFinding::new(
            FindingCode::InvalidLoadDirection,
            &load.load_id,
            "straight-pipe point forces require a translational direction",
        ));
        return;
    };
    if quantity.dimension != LoadDimension::Force {
        findings.push(UserLoadFinding::new(
            FindingCode::InvalidLoadDimension,
            &load.load_id,
            "straight-pipe point force recovery requires force",
        ));
        return;
    }

    let point = match GlobalPointForce::new(station_fraction, force) {
        Ok(point) => point,
        Err(error) => {
            findings.push(UserLoadFinding::new(
                FindingCode::InvalidDistributionSpan,
                &load.load_id,
                error.to_string(),
            ));
            return;
        }
    };
    let equivalent = match pipe.equivalent_global_nodal_loads(&[], &[point]) {
        Ok(loads) => loads,
        Err(error) => {
            findings.push(UserLoadFinding::new(
                FindingCode::MissingElementGeometry,
                &load.load_id,
                error.to_string(),
            ));
            return;
        }
    };
    push_equivalent_nodal_loads(pipe, load, &equivalent, nodal_loads);
    recovery_hooks.push(ResultRecoveryHook {
        load_id: load.load_id.clone(),
        hook_kind: RecoveryHookKind::NodalForce,
        target_ref: format!("element:{target_element}:station:{station_fraction}"),
        dimension: quantity.dimension,
        provenance_ref: load.provenance_ref.clone(),
    });
}

fn push_equivalent_nodal_loads(
    pipe: &StraightPipeElement,
    load: &UserLoad,
    equivalent: &[f64; ELEMENT_DOF],
    nodal_loads: &mut Vec<NodalLoadContribution>,
) {
    push_equivalent_nodal_loads_with_trace(
        pipe,
        equivalent,
        ContributionTrace::from_load(load),
        nodal_loads,
    );
}

fn push_equivalent_nodal_loads_with_trace(
    pipe: &StraightPipeElement,
    equivalent: &[f64; ELEMENT_DOF],
    trace: ContributionTrace,
    nodal_loads: &mut Vec<NodalLoadContribution>,
) {
    for (local_dof, &value) in equivalent.iter().enumerate() {
        if value == 0.0 {
            continue;
        }
        let (node_index, dof_index) = if local_dof < DOF_PER_NODE {
            (pipe.node_i.index, local_dof)
        } else {
            (pipe.node_j.index, local_dof - DOF_PER_NODE)
        };
        nodal_loads.push(NodalLoadContribution {
            node_index,
            global_dof: node_index * DOF_PER_NODE + dof_index,
            value,
            dimension: if dof_index < RX {
                LoadDimension::Force
            } else {
                LoadDimension::Moment
            },
            trace: trace.clone(),
        });
    }
}

fn global_vector_for_direction(direction: UserLoadDirection, value: f64) -> Option<[f64; 3]> {
    match direction {
        UserLoadDirection::GlobalX => Some([value, 0.0, 0.0]),
        UserLoadDirection::GlobalY => Some([0.0, value, 0.0]),
        UserLoadDirection::GlobalZ => Some([0.0, 0.0, value]),
        UserLoadDirection::RotationX
        | UserLoadDirection::RotationY
        | UserLoadDirection::RotationZ => None,
    }
}

fn prepare_nodal_user_load(
    load: &UserLoad,
    node_index: usize,
    quantity: UserLoadQuantity,
    node_count: usize,
    nodal_loads: &mut Vec<NodalLoadContribution>,
    recovery_hooks: &mut Vec<ResultRecoveryHook>,
    findings: &mut Vec<UserLoadFinding>,
) {
    if node_index >= node_count {
        findings.push(UserLoadFinding::new(
            FindingCode::NodeOutOfRange,
            &load.load_id,
            format!("node index {node_index} is outside node count {node_count}"),
        ));
        return;
    }

    let Some(hook_kind) = validate_nodal_kind_and_dimension(load, quantity, findings) else {
        return;
    };

    let global_dof = node_index * DOF_PER_NODE + load.direction.dof_index();
    nodal_loads.push(NodalLoadContribution {
        node_index,
        global_dof,
        value: quantity.value,
        dimension: quantity.dimension,
        trace: ContributionTrace::from_load(load),
    });
    recovery_hooks.push(ResultRecoveryHook {
        load_id: load.load_id.clone(),
        hook_kind,
        target_ref: format!("node:{node_index}:dof:{global_dof}"),
        dimension: quantity.dimension,
        provenance_ref: load.provenance_ref.clone(),
    });
}

fn prepare_element_user_load(
    load: &UserLoad,
    element_index: usize,
    quantity: UserLoadQuantity,
    span: ElementLoadSpan,
    element_length: Option<f64>,
    element_count: usize,
    element_distributed_loads: &mut Vec<ElementDistributedLoadContribution>,
    recovery_hooks: &mut Vec<ResultRecoveryHook>,
    findings: &mut Vec<UserLoadFinding>,
) {
    if element_index >= element_count {
        findings.push(UserLoadFinding::new(
            FindingCode::ElementOutOfRange,
            &load.load_id,
            format!("element index {element_index} is outside element count {element_count}"),
        ));
        return;
    }

    if load.kind != UserLoadKind::UniformDistributedLoad {
        findings.push(UserLoadFinding::new(
            FindingCode::UnsupportedTargetForLoadKind,
            &load.load_id,
            "element targets are limited to distributed user loads",
        ));
        return;
    }
    if load.direction.is_rotational() {
        findings.push(UserLoadFinding::new(
            FindingCode::InvalidLoadDirection,
            &load.load_id,
            "distributed user loads require a translational direction",
        ));
        return;
    }
    if quantity.dimension != LoadDimension::ForcePerLength {
        findings.push(UserLoadFinding::new(
            FindingCode::InvalidLoadDimension,
            &load.load_id,
            "distributed user load dimension must be force per length",
        ));
        return;
    }
    if !(0.0..=1.0).contains(&span.start_fraction)
        || !(0.0..=1.0).contains(&span.end_fraction)
        || span.start_fraction >= span.end_fraction
    {
        findings.push(UserLoadFinding::new(
            FindingCode::InvalidDistributionSpan,
            &load.load_id,
            "distributed user load span must satisfy 0 <= start < end <= 1",
        ));
        return;
    }

    let equivalent_total = match element_length {
        Some(length) if length.is_finite() && length > 0.0 => {
            Some(quantity.value * length * span.length_fraction())
        }
        Some(_) => {
            findings.push(UserLoadFinding::new(
                FindingCode::NonPositiveElementLength,
                &load.load_id,
                "element length for equivalent load must be positive and finite",
            ));
            return;
        }
        None => None,
    };

    element_distributed_loads.push(ElementDistributedLoadContribution {
        element_index,
        direction: load.direction,
        magnitude: quantity,
        span,
        equivalent_total,
        trace: ContributionTrace::from_load(load),
    });
    recovery_hooks.push(ResultRecoveryHook {
        load_id: load.load_id.clone(),
        hook_kind: RecoveryHookKind::ElementDistributedLoad,
        target_ref: format!(
            "element:{element_index}:span:{}-{}",
            span.start_fraction, span.end_fraction
        ),
        dimension: quantity.dimension,
        provenance_ref: load.provenance_ref.clone(),
    });
}

fn validate_nodal_kind_and_dimension(
    load: &UserLoad,
    quantity: UserLoadQuantity,
    findings: &mut Vec<UserLoadFinding>,
) -> Option<RecoveryHookKind> {
    match load.kind {
        UserLoadKind::ConcentratedForce => {
            if load.direction.is_rotational() {
                findings.push(UserLoadFinding::new(
                    FindingCode::InvalidLoadDirection,
                    &load.load_id,
                    "concentrated forces require a translational direction",
                ));
                return None;
            }
            if quantity.dimension != LoadDimension::Force {
                findings.push(UserLoadFinding::new(
                    FindingCode::InvalidLoadDimension,
                    &load.load_id,
                    "concentrated force dimension must be force",
                ));
                return None;
            }
            Some(RecoveryHookKind::NodalForce)
        }
        UserLoadKind::ConcentratedMoment => {
            if !load.direction.is_rotational() {
                findings.push(UserLoadFinding::new(
                    FindingCode::InvalidLoadDirection,
                    &load.load_id,
                    "concentrated moments require a rotational direction",
                ));
                return None;
            }
            if quantity.dimension != LoadDimension::Moment {
                findings.push(UserLoadFinding::new(
                    FindingCode::InvalidLoadDimension,
                    &load.load_id,
                    "concentrated moment dimension must be moment",
                ));
                return None;
            }
            Some(RecoveryHookKind::NodalMoment)
        }
        UserLoadKind::UniformDistributedLoad => {
            findings.push(UserLoadFinding::new(
                FindingCode::UnsupportedTargetForLoadKind,
                &load.load_id,
                "distributed user loads require an element target",
            ));
            None
        }
    }
}

fn require_schema_binding(
    actual: CanonicalSchemaBinding,
    expected: CanonicalSchemaBinding,
) -> Result<(), BoundaryMetadataError> {
    if actual == expected {
        Ok(())
    } else {
        Err(BoundaryMetadataError::SchemaBindingMismatch { expected, actual })
    }
}

fn validate_finite(name: &'static str, value: f64) -> Result<(), UserLoadError> {
    if !value.is_finite() {
        return Err(UserLoadError::NonFiniteInput { name, value });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_pipe_stress_frame_kernel::FrameNode;
    use open_pipe_stress_straight_pipe::{StraightPipeElement, StraightPipeSectionProperties};

    fn q(value: f64, dimension: LoadDimension) -> UserLoadQuantity {
        UserLoadQuantity::new(value, dimension).unwrap()
    }

    fn boundary_ref(schema_binding: CanonicalSchemaBinding, target_ref: &str) -> BoundaryRecordRef {
        BoundaryRecordRef::new(
            "load:boundary",
            schema_binding,
            target_ref,
            "load-case:user",
            "payload:model",
            "hash:model",
        )
        .unwrap()
    }

    fn unit(
        unit_name: &str,
        dimension: open_pipe_stress_primitive_loads::CanonicalDimension,
    ) -> QuantityUnitMetadata {
        QuantityUnitMetadata::new(unit_name, dimension, "unit-system:si").unwrap()
    }

    #[test]
    fn concentrated_force_maps_to_translational_nodal_dof() {
        let load = UserLoad::concentrated_force(
            "user-fx",
            1,
            UserLoadDirection::GlobalX,
            q(1250.0, LoadDimension::Force),
        )
        .with_provenance_ref("manual-input");

        let prepared = apply_user_loads(3, 0, &[load]);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.nodal_loads.len(), 1);
        assert_eq!(prepared.nodal_loads[0].global_dof, DOF_PER_NODE);
        assert_eq!(prepared.nodal_loads[0].value, 1250.0);
        assert_eq!(prepared.global_load_vector(3)[DOF_PER_NODE], 1250.0);
        assert_eq!(
            prepared.recovery_hooks[0].hook_kind,
            RecoveryHookKind::NodalForce
        );
        assert_eq!(
            prepared.recovery_hooks[0].provenance_ref.as_deref(),
            Some("manual-input")
        );
    }

    #[test]
    fn nodal_user_load_boundary_record_requires_unit_and_model_load_binding() {
        let load = UserLoad::concentrated_force(
            "user-fx",
            1,
            UserLoadDirection::GlobalX,
            q(1250.0, LoadDimension::Force),
        )
        .with_provenance_ref("provenance:manual");
        let prepared = apply_user_loads(3, 0, &[load]);

        let record = prepared.nodal_loads[0]
            .to_model_load_boundary_record(
                boundary_ref(CanonicalSchemaBinding::ModelLoadRecord, "node:1"),
                unit(
                    "N",
                    open_pipe_stress_primitive_loads::CanonicalDimension::Force,
                ),
            )
            .unwrap();

        assert_eq!(record.value, 1250.0);
        assert_eq!(record.provenance_ref, "provenance:manual");
        assert_eq!(
            record.record.schema_binding.schema_ref(),
            "schemas/model.schema.yaml#/$defs/LoadRecord"
        );
    }

    #[test]
    fn distributed_user_load_boundary_record_uses_force_per_length_dimension_metadata() {
        let load = UserLoad::uniform_distributed(
            "user-wy",
            2,
            UserLoadDirection::GlobalY,
            q(-12.0, LoadDimension::ForcePerLength),
            ElementLoadSpan::full(),
            Some(6.0),
        )
        .with_provenance_ref("provenance:manual");
        let prepared = apply_user_loads(0, 3, &[load]);

        let record = prepared.element_distributed_loads[0]
            .to_model_load_boundary_record(
                boundary_ref(CanonicalSchemaBinding::ModelLoadRecord, "element:2"),
                unit(
                    "N/m",
                    open_pipe_stress_primitive_loads::CanonicalDimension::ForcePerLength,
                ),
            )
            .unwrap();

        assert_eq!(record.unit.unit, "N/m");
        assert_eq!(record.unit.dimension.as_str(), "force_per_length");
        assert!(record
            .round_trip_key()
            .contains("payload_hash_ref=hash:model"));
    }

    #[test]
    fn recovery_hook_boundary_record_rejects_load_record_binding() {
        let load = UserLoad::concentrated_force(
            "user-fx",
            0,
            UserLoadDirection::GlobalX,
            q(10.0, LoadDimension::Force),
        );
        let prepared = apply_user_loads(1, 0, &[load]);

        let err = prepared.recovery_hooks[0]
            .to_result_boundary_record(
                10.0,
                boundary_ref(CanonicalSchemaBinding::ModelLoadRecord, "node:0"),
                unit(
                    "N",
                    open_pipe_stress_primitive_loads::CanonicalDimension::Force,
                ),
            )
            .unwrap_err();

        assert_eq!(
            err,
            BoundaryMetadataError::SchemaBindingMismatch {
                expected: CanonicalSchemaBinding::ModelResultValue,
                actual: CanonicalSchemaBinding::ModelLoadRecord,
            }
        );
    }

    #[test]
    fn concentrated_moment_maps_to_rotational_nodal_dof() {
        let load = UserLoad::concentrated_moment(
            "user-mz",
            0,
            UserLoadDirection::RotationZ,
            q(-80.0, LoadDimension::Moment),
        );

        let prepared = apply_user_loads(1, 0, &[load]);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.nodal_loads[0].global_dof, RZ);
        assert_eq!(prepared.nodal_loads[0].dimension, LoadDimension::Moment);
        assert_eq!(
            prepared.recovery_hooks[0].hook_kind,
            RecoveryHookKind::NodalMoment
        );
    }

    #[test]
    fn uniform_distributed_load_records_span_and_equivalent_total() {
        let load = UserLoad::uniform_distributed(
            "user-wy",
            2,
            UserLoadDirection::GlobalY,
            q(-12.0, LoadDimension::ForcePerLength),
            ElementLoadSpan::new(0.25, 0.75).unwrap(),
            Some(6.0),
        );

        let prepared = apply_user_loads(0, 3, &[load]);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.element_distributed_loads.len(), 1);
        assert_eq!(prepared.element_distributed_loads[0].element_index, 2);
        assert_eq!(
            prepared.element_distributed_loads[0].equivalent_total,
            Some(-36.0)
        );
        assert_eq!(
            prepared.recovery_hooks[0].hook_kind,
            RecoveryHookKind::ElementDistributedLoad
        );
    }

    #[test]
    fn distributed_load_can_defer_equivalent_total_until_element_length_known() {
        let load = UserLoad::uniform_distributed(
            "user-wz",
            0,
            UserLoadDirection::GlobalZ,
            q(4.0, LoadDimension::ForcePerLength),
            ElementLoadSpan::full(),
            None,
        );

        let prepared = apply_user_loads(0, 1, &[load]);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.element_distributed_loads[0].equivalent_total, None);
    }

    #[test]
    fn invalid_dimensions_and_directions_are_findings() {
        let bad_force = UserLoad::concentrated_force(
            "bad-force",
            0,
            UserLoadDirection::RotationX,
            q(1.0, LoadDimension::Force),
        );
        let bad_moment = UserLoad::concentrated_moment(
            "bad-moment",
            0,
            UserLoadDirection::GlobalX,
            q(1.0, LoadDimension::Moment),
        );
        let bad_distributed = UserLoad::uniform_distributed(
            "bad-dist",
            0,
            UserLoadDirection::GlobalZ,
            q(1.0, LoadDimension::Force),
            ElementLoadSpan::full(),
            Some(1.0),
        );

        let prepared = apply_user_loads(1, 1, &[bad_force, bad_moment, bad_distributed]);

        assert!(prepared.is_blocked());
        assert_eq!(prepared.findings.len(), 3);
        assert!(prepared.findings.iter().any(|finding| {
            finding.code == FindingCode::InvalidLoadDirection && finding.load_id == "bad-force"
        }));
        assert!(prepared.findings.iter().any(|finding| {
            finding.code == FindingCode::InvalidLoadDirection && finding.load_id == "bad-moment"
        }));
        assert!(prepared.findings.iter().any(|finding| {
            finding.code == FindingCode::InvalidLoadDimension && finding.load_id == "bad-dist"
        }));
    }

    #[test]
    fn missing_and_out_of_range_targets_are_findings() {
        let missing = UserLoad::missing_target(
            "missing",
            UserLoadKind::ConcentratedForce,
            UserLoadDirection::GlobalX,
            Some(q(1.0, LoadDimension::Force)),
        );
        let out_of_range = UserLoad::concentrated_force(
            "out-of-range",
            4,
            UserLoadDirection::GlobalX,
            q(1.0, LoadDimension::Force),
        );

        let prepared = apply_user_loads(1, 0, &[missing, out_of_range]);

        assert!(prepared.is_blocked());
        assert_eq!(prepared.findings[0].code, FindingCode::MissingLoadTarget);
        assert_eq!(prepared.findings[1].code, FindingCode::NodeOutOfRange);
        assert!(prepared.nodal_loads.is_empty());
    }

    #[test]
    fn missing_quantity_blocks_load_application() {
        let load = UserLoad {
            load_id: "missing-quantity".to_string(),
            kind: UserLoadKind::ConcentratedForce,
            target: Some(UserLoadTarget::Node(0)),
            direction: UserLoadDirection::GlobalX,
            quantity: None,
            provenance_ref: Some("manual-input".to_string()),
        };

        let prepared = apply_user_loads(1, 0, &[load]);

        assert!(prepared.is_blocked());
        assert_eq!(prepared.findings.len(), 1);
        assert_eq!(prepared.findings[0].code, FindingCode::MissingLoadQuantity);
        assert_eq!(prepared.findings[0].load_id, "missing-quantity");
        assert!(prepared.nodal_loads.is_empty());
        assert!(prepared.recovery_hooks.is_empty());
    }

    #[test]
    fn unsupported_target_kind_combinations_are_findings() {
        let distributed_on_node = UserLoad {
            load_id: "distributed-on-node".to_string(),
            kind: UserLoadKind::UniformDistributedLoad,
            target: Some(UserLoadTarget::Node(0)),
            direction: UserLoadDirection::GlobalY,
            quantity: Some(q(2.0, LoadDimension::ForcePerLength)),
            provenance_ref: None,
        };
        let force_on_element = UserLoad {
            load_id: "force-on-element".to_string(),
            kind: UserLoadKind::ConcentratedForce,
            target: Some(UserLoadTarget::Element {
                element_index: 0,
                span: ElementLoadSpan::full(),
                element_length: Some(1.0),
            }),
            direction: UserLoadDirection::GlobalX,
            quantity: Some(q(3.0, LoadDimension::Force)),
            provenance_ref: None,
        };

        let prepared = apply_user_loads(1, 1, &[distributed_on_node, force_on_element]);

        assert!(prepared.is_blocked());
        assert_eq!(prepared.findings.len(), 2);
        assert!(prepared.findings.iter().any(|finding| {
            finding.code == FindingCode::UnsupportedTargetForLoadKind
                && finding.load_id == "distributed-on-node"
        }));
        assert!(prepared.findings.iter().any(|finding| {
            finding.code == FindingCode::UnsupportedTargetForLoadKind
                && finding.load_id == "force-on-element"
        }));
        assert!(prepared.nodal_loads.is_empty());
        assert!(prepared.element_distributed_loads.is_empty());
        assert!(prepared.recovery_hooks.is_empty());
    }

    #[test]
    fn invalid_span_and_length_are_findings() {
        let invalid_span = UserLoad::uniform_distributed(
            "bad-span",
            0,
            UserLoadDirection::GlobalZ,
            q(1.0, LoadDimension::ForcePerLength),
            ElementLoadSpan::new(0.8, 0.2).unwrap(),
            Some(2.0),
        );
        let invalid_length = UserLoad::uniform_distributed(
            "bad-length",
            0,
            UserLoadDirection::GlobalZ,
            q(1.0, LoadDimension::ForcePerLength),
            ElementLoadSpan::full(),
            Some(0.0),
        );

        let prepared = apply_user_loads(0, 1, &[invalid_span, invalid_length]);

        assert!(prepared.is_blocked());
        assert_eq!(prepared.findings.len(), 2);
        assert!(prepared
            .findings
            .iter()
            .any(|finding| { finding.code == FindingCode::InvalidDistributionSpan }));
        assert!(prepared
            .findings
            .iter()
            .any(|finding| { finding.code == FindingCode::NonPositiveElementLength }));
    }

    #[test]
    fn contributions_are_sorted_deterministically() {
        let loads = vec![
            UserLoad::concentrated_force(
                "z-late",
                1,
                UserLoadDirection::GlobalZ,
                q(1.0, LoadDimension::Force),
            ),
            UserLoad::concentrated_force(
                "x-early",
                0,
                UserLoadDirection::GlobalX,
                q(1.0, LoadDimension::Force),
            ),
        ];

        let prepared = apply_user_loads(2, 0, &loads);

        assert_eq!(prepared.nodal_loads[0].trace.load_id, "x-early");
        assert_eq!(prepared.nodal_loads[1].trace.load_id, "z-late");
    }

    fn straight_pipe() -> StraightPipeElement {
        StraightPipeElement::new(
            "user-load-pipe",
            FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
            FrameNode::new(1, [2.0, 0.0, 0.0]).unwrap(),
            StraightPipeSectionProperties::new(2.0e11, 7.7e10, 0.01, 8.0e-6, 9.0e-6, 1.7e-5, None)
                .unwrap(),
            [0.0, 1.0, 0.0],
        )
        .unwrap()
    }

    fn straight_pipe_along_global_y() -> StraightPipeElement {
        StraightPipeElement::new(
            "user-load-pipe-y",
            FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
            FrameNode::new(1, [0.0, 2.0, 0.0]).unwrap(),
            StraightPipeSectionProperties::new(2.0e11, 7.7e10, 0.01, 8.0e-6, 9.0e-6, 1.7e-5, None)
                .unwrap(),
            [1.0, 0.0, 0.0],
        )
        .unwrap()
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= 1.0e-12,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn straight_pipe_equivalent_user_loads_recover_full_span_distribution() {
        let pipe = straight_pipe();
        let load = UserLoad::uniform_distributed(
            "user-line-y",
            0,
            UserLoadDirection::GlobalY,
            q(-4.0, LoadDimension::ForcePerLength),
            ElementLoadSpan::full(),
            Some(2.0),
        );

        let prepared = apply_straight_pipe_equivalent_user_loads(0, &pipe, &[load]);
        let vector = prepared.global_load_vector(2);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.element_distributed_loads.len(), 1);
        assert_eq!(prepared.nodal_loads.len(), 4);
        assert_close(vector[UY], -4.0);
        assert_close(vector[DOF_PER_NODE + UY], -4.0);
        assert_close(vector[RZ], -4.0 * 4.0 / 12.0);
        assert_close(vector[DOF_PER_NODE + RZ], 4.0 * 4.0 / 12.0);
    }

    #[test]
    fn straight_pipe_equivalent_user_loads_transform_global_load_directions() {
        let pipe = straight_pipe_along_global_y();
        let load = UserLoad::uniform_distributed(
            "user-line-global-x",
            0,
            UserLoadDirection::GlobalX,
            q(3.0, LoadDimension::ForcePerLength),
            ElementLoadSpan::full(),
            Some(2.0),
        );

        let prepared = apply_straight_pipe_equivalent_user_loads(0, &pipe, &[load]);
        let vector = prepared.global_load_vector(2);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.nodal_loads.len(), 4);
        assert_close(vector[UX], 3.0);
        assert_close(vector[DOF_PER_NODE + UX], 3.0);
        assert_close(vector[RZ], -1.0);
        assert_close(vector[DOF_PER_NODE + RZ], 1.0);
        assert_close(vector[UY], 0.0);
        assert_close(vector[DOF_PER_NODE + UY], 0.0);
    }

    #[test]
    fn straight_pipe_equivalent_user_loads_recover_partial_span_distribution() {
        let pipe = straight_pipe();
        let load = UserLoad::uniform_distributed(
            "user-line-y-partial",
            0,
            UserLoadDirection::GlobalY,
            q(-4.0, LoadDimension::ForcePerLength),
            ElementLoadSpan::new(0.25, 0.75).unwrap(),
            Some(2.0),
        );

        let prepared = apply_straight_pipe_equivalent_user_loads(0, &pipe, &[load]);
        let vector = prepared.global_load_vector(2);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.element_distributed_loads.len(), 1);
        assert_eq!(prepared.nodal_loads.len(), 4);
        assert_eq!(
            prepared.element_distributed_loads[0].equivalent_total,
            Some(-4.0)
        );
        assert_eq!(
            prepared.recovery_hooks[0].target_ref,
            "element:0:span:0.25-0.75"
        );
        assert_close(vector[UY], -2.0);
        assert_close(vector[RZ], -11.0 / 12.0);
        assert_close(vector[DOF_PER_NODE + UY], -2.0);
        assert_close(vector[DOF_PER_NODE + RZ], 11.0 / 12.0);
    }

    #[test]
    fn straight_pipe_equivalent_user_loads_include_axial_effects() {
        let pipe = straight_pipe();
        let load = UserLoad::uniform_distributed(
            "user-line-y",
            0,
            UserLoadDirection::GlobalY,
            q(-4.0, LoadDimension::ForcePerLength),
            ElementLoadSpan::full(),
            Some(2.0),
        );
        let axial_effect = PrimitiveAxialEffectContribution {
            load_id: "thermal-axial".to_string(),
            element_index: 0,
            axial_force: 125.0,
        };

        let prepared = apply_straight_pipe_equivalent_user_loads_with_axial_effects(
            0,
            &pipe,
            &[load],
            &[axial_effect],
        );
        let vector = prepared.global_load_vector(2);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.element_distributed_loads.len(), 1);
        assert_eq!(prepared.nodal_loads.len(), 6);
        assert_close(vector[UX], -125.0);
        assert_close(vector[DOF_PER_NODE + UX], 125.0);
        assert_close(vector[UY], -4.0);
        assert_close(vector[DOF_PER_NODE + UY], -4.0);
        assert!(prepared.recovery_hooks.iter().any(|hook| {
            hook.load_id == "thermal-axial"
                && hook.hook_kind == RecoveryHookKind::ElementAxialEffect
                && hook.target_ref == "element:0:axial-effect:thermal-axial"
                && hook.dimension == LoadDimension::Force
        }));
        assert!(prepared.nodal_loads.iter().any(|load| {
            load.trace.load_id == "thermal-axial"
                && load.trace.kind == UserLoadKind::ConcentratedForce
                && load.dimension == LoadDimension::Force
        }));
    }

    #[test]
    fn straight_pipe_equivalent_user_loads_reject_wrong_axial_effect_element() {
        let pipe = straight_pipe();
        let axial_effect = PrimitiveAxialEffectContribution {
            load_id: "wrong-element-axial".to_string(),
            element_index: 1,
            axial_force: 125.0,
        };

        let prepared = apply_straight_pipe_equivalent_user_loads_with_axial_effects(
            0,
            &pipe,
            &[],
            &[axial_effect],
        );

        assert!(prepared.is_blocked());
        assert_eq!(prepared.findings.len(), 1);
        assert_eq!(prepared.findings[0].code, FindingCode::ElementOutOfRange);
        assert_eq!(prepared.findings[0].load_id, "wrong-element-axial");
        assert!(prepared.nodal_loads.is_empty());
        assert!(prepared.recovery_hooks.is_empty());
    }

    #[test]
    fn straight_pipe_equivalent_user_loads_reject_nonfinite_axial_effect() {
        let pipe = straight_pipe();
        let axial_effect = PrimitiveAxialEffectContribution {
            load_id: "nonfinite-axial".to_string(),
            element_index: 0,
            axial_force: f64::NAN,
        };

        let prepared = apply_straight_pipe_equivalent_user_loads_with_axial_effects(
            0,
            &pipe,
            &[],
            &[axial_effect],
        );

        assert!(prepared.is_blocked());
        assert_eq!(prepared.findings.len(), 1);
        assert_eq!(prepared.findings[0].code, FindingCode::NonFiniteAxialEffect);
        assert_eq!(prepared.findings[0].load_id, "nonfinite-axial");
        assert!(prepared.nodal_loads.is_empty());
        assert!(prepared.recovery_hooks.is_empty());
    }

    #[test]
    fn straight_pipe_equivalent_user_loads_recover_interior_point_force() {
        let pipe = straight_pipe();
        let load = UserLoad::element_concentrated_force(
            "user-point-y",
            0,
            0.25,
            UserLoadDirection::GlobalY,
            q(8.0, LoadDimension::Force),
        );

        let prepared = apply_straight_pipe_equivalent_user_loads(0, &pipe, &[load]);
        let vector = prepared.global_load_vector(2);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.nodal_loads.len(), 4);
        assert_eq!(vector[UY], 6.75);
        assert_eq!(vector[RZ], 2.25);
        assert_eq!(vector[DOF_PER_NODE + UY], 1.25);
        assert_eq!(vector[DOF_PER_NODE + RZ], -0.75);
    }

    #[test]
    fn straight_pipe_equivalent_user_loads_reject_invalid_span_and_unsupported_station_moment() {
        let pipe = straight_pipe();
        let invalid_span = UserLoad::uniform_distributed(
            "invalid-span",
            0,
            UserLoadDirection::GlobalY,
            q(1.0, LoadDimension::ForcePerLength),
            ElementLoadSpan::new(0.75, 0.25).unwrap(),
            Some(1.0),
        );
        let moment = UserLoad {
            load_id: "station-moment".to_string(),
            kind: UserLoadKind::ConcentratedMoment,
            target: Some(UserLoadTarget::ElementStation {
                element_index: 0,
                station_fraction: 0.5,
            }),
            direction: UserLoadDirection::RotationZ,
            quantity: Some(q(2.0, LoadDimension::Moment)),
            provenance_ref: None,
        };

        let prepared = apply_straight_pipe_equivalent_user_loads(0, &pipe, &[invalid_span, moment]);

        assert!(prepared.is_blocked());
        assert_eq!(prepared.findings.len(), 2);
        assert_eq!(
            prepared.findings[0].code,
            FindingCode::InvalidDistributionSpan
        );
        assert_eq!(
            prepared.findings[1].code,
            FindingCode::UnsupportedTargetForLoadKind
        );
        assert!(prepared.nodal_loads.is_empty());
    }

    #[test]
    fn generic_user_load_application_reports_element_station_geometry_gap() {
        let load = UserLoad::element_concentrated_force(
            "station-load",
            0,
            0.5,
            UserLoadDirection::GlobalY,
            q(8.0, LoadDimension::Force),
        );

        let prepared = apply_user_loads(2, 1, &[load]);

        assert!(prepared.is_blocked());
        assert_eq!(
            prepared.findings[0].code,
            FindingCode::MissingElementGeometry
        );
    }

    #[test]
    fn non_finite_quantity_is_rejected_at_construction() {
        let err = UserLoadQuantity::new(f64::NAN, LoadDimension::Force).unwrap_err();
        assert!(matches!(err, UserLoadError::NonFiniteInput { .. }));
    }
}
