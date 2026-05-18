//! Straight pipe element mechanics.
//!
//! This crate adapts explicit straight-pipe properties into the frame-kernel
//! solver boundary. It does not provide pipe tables, material defaults,
//! code-specific checks, protected standards data, or engineering approval.

use open_pipe_stress_frame_kernel::{
    CanonicalDimension, CanonicalModelReference, FrameElement, FrameKernelError,
    FrameKernelUnitBasis, FrameNode, FrameOrientation, FrameSection, Matrix12,
    QuantityUnitMetadata, UnitSystemRef, DOF_PER_NODE, ELEMENT_DOF, RX, RY, RZ, UX, UY, UZ,
};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StraightPipeSectionProperties {
    pub elastic_modulus: f64,
    pub shear_modulus: f64,
    pub area: f64,
    pub second_moment_y: f64,
    pub second_moment_z: f64,
    pub torsion_constant: f64,
    pub mass_per_length: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StraightPipeElement {
    pub element_id: String,
    pub node_i: FrameNode,
    pub node_j: FrameNode,
    pub section: StraightPipeSectionProperties,
    pub y_reference: [f64; 3],
}

#[derive(Debug, Clone, PartialEq)]
pub struct WeightHook {
    pub mass_per_length: f64,
    pub gravity: f64,
    pub weight_force_per_length: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StraightPipeAxialEffect {
    pub axial_force: f64,
}

impl StraightPipeAxialEffect {
    pub fn new(axial_force: f64) -> Result<Self, StraightPipeError> {
        validate_finite("axial_force", axial_force)?;
        Ok(Self { axial_force })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalElementForces {
    pub local_displacements: [f64; ELEMENT_DOF],
    pub local_forces: [f64; ELEMENT_DOF],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalLoadDirection {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UniformLocalLoad {
    pub direction: LocalLoadDirection,
    pub force_per_length: f64,
}

impl UniformLocalLoad {
    pub fn new(
        direction: LocalLoadDirection,
        force_per_length: f64,
    ) -> Result<Self, StraightPipeError> {
        validate_finite("force_per_length", force_per_length)?;
        Ok(Self {
            direction,
            force_per_length,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UniformLoadSpan {
    pub start_fraction: f64,
    pub end_fraction: f64,
}

impl UniformLoadSpan {
    pub fn full() -> Self {
        Self {
            start_fraction: 0.0,
            end_fraction: 1.0,
        }
    }

    pub fn new(start_fraction: f64, end_fraction: f64) -> Result<Self, StraightPipeError> {
        validate_load_span(start_fraction, end_fraction)?;
        Ok(Self {
            start_fraction,
            end_fraction,
        })
    }

    pub fn length_fraction(&self) -> f64 {
        self.end_fraction - self.start_fraction
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpannedUniformLocalLoad {
    pub span: UniformLoadSpan,
    pub direction: LocalLoadDirection,
    pub force_per_length: f64,
}

impl SpannedUniformLocalLoad {
    pub fn new(
        direction: LocalLoadDirection,
        force_per_length: f64,
        span: UniformLoadSpan,
    ) -> Result<Self, StraightPipeError> {
        validate_load_span(span.start_fraction, span.end_fraction)?;
        validate_finite("force_per_length", force_per_length)?;
        Ok(Self {
            span,
            direction,
            force_per_length,
        })
    }

    pub fn full(
        direction: LocalLoadDirection,
        force_per_length: f64,
    ) -> Result<Self, StraightPipeError> {
        Self::new(direction, force_per_length, UniformLoadSpan::full())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlobalUniformLoad {
    pub force_per_length: [f64; 3],
}

impl GlobalUniformLoad {
    pub fn new(force_per_length: [f64; 3]) -> Result<Self, StraightPipeError> {
        validate_finite_vector("force_per_length", force_per_length)?;
        Ok(Self { force_per_length })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpannedGlobalUniformLoad {
    pub span: UniformLoadSpan,
    pub force_per_length: [f64; 3],
}

impl SpannedGlobalUniformLoad {
    pub fn new(
        force_per_length: [f64; 3],
        span: UniformLoadSpan,
    ) -> Result<Self, StraightPipeError> {
        validate_load_span(span.start_fraction, span.end_fraction)?;
        validate_finite_vector("force_per_length", force_per_length)?;
        Ok(Self {
            span,
            force_per_length,
        })
    }

    pub fn full(force_per_length: [f64; 3]) -> Result<Self, StraightPipeError> {
        Self::new(force_per_length, UniformLoadSpan::full())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointLocalForce {
    pub station_fraction: f64,
    pub direction: LocalLoadDirection,
    pub force: f64,
}

impl PointLocalForce {
    pub fn new(
        station_fraction: f64,
        direction: LocalLoadDirection,
        force: f64,
    ) -> Result<Self, StraightPipeError> {
        validate_station_fraction("station_fraction", station_fraction)?;
        validate_finite("point_force", force)?;
        Ok(Self {
            station_fraction,
            direction,
            force,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlobalPointForce {
    pub station_fraction: f64,
    pub force: [f64; 3],
}

impl GlobalPointForce {
    pub fn new(station_fraction: f64, force: [f64; 3]) -> Result<Self, StraightPipeError> {
        validate_station_fraction("station_fraction", station_fraction)?;
        validate_finite_vector("point_force", force)?;
        Ok(Self {
            station_fraction,
            force,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StationResultants {
    pub station_fraction: f64,
    pub distance_from_i: f64,
    pub axial_force: f64,
    pub shear_force_y: f64,
    pub shear_force_z: f64,
    pub torsional_moment: f64,
    pub bending_moment_y: f64,
    pub bending_moment_z: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipeEnd {
    I,
    J,
}

impl PipeEnd {
    fn offset(self) -> usize {
        match self {
            Self::I => 0,
            Self::J => DOF_PER_NODE,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PipeEndResultants {
    pub end: PipeEnd,
    pub axial_force: f64,
    pub shear_force_y: f64,
    pub shear_force_z: f64,
    pub torsional_moment: f64,
    pub bending_moment_y: f64,
    pub bending_moment_z: f64,
}

impl PipeEndResultants {
    pub fn from_local_forces(local_forces: &[f64; ELEMENT_DOF], end: PipeEnd) -> Self {
        let offset = end.offset();
        Self {
            end,
            axial_force: local_forces[offset + UX],
            shear_force_y: local_forces[offset + UY],
            shear_force_z: local_forces[offset + UZ],
            torsional_moment: local_forces[offset + RX],
            bending_moment_y: local_forces[offset + RY],
            bending_moment_z: local_forces[offset + RZ],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StraightPipeBoundaryMetadata {
    pub frame_units: FrameKernelUnitBasis,
    pub mass_per_length_unit: QuantityUnitMetadata,
    pub gravity_unit: QuantityUnitMetadata,
    pub weight_force_per_length_unit: QuantityUnitMetadata,
    pub analytical_element_ref: CanonicalModelReference,
    pub source_model_ref: CanonicalModelReference,
}

impl StraightPipeBoundaryMetadata {
    pub fn new(
        frame_units: FrameKernelUnitBasis,
        mass_per_length_unit: QuantityUnitMetadata,
        gravity_unit: QuantityUnitMetadata,
        weight_force_per_length_unit: QuantityUnitMetadata,
        analytical_element_ref: CanonicalModelReference,
        source_model_ref: CanonicalModelReference,
    ) -> Option<Self> {
        let metadata = Self {
            frame_units,
            mass_per_length_unit,
            gravity_unit,
            weight_force_per_length_unit,
            analytical_element_ref,
            source_model_ref,
        };
        metadata.has_expected_dimensions().then_some(metadata)
    }

    pub fn unit_system_ref(&self) -> &UnitSystemRef {
        &self.frame_units.unit_system_ref
    }

    pub fn has_expected_dimensions(&self) -> bool {
        self.frame_units.has_expected_dimensions()
            && self.mass_per_length_unit.dimension == CanonicalDimension::MassPerLength
            && self.gravity_unit.dimension == CanonicalDimension::Acceleration
            && self.weight_force_per_length_unit.dimension == CanonicalDimension::Tbd
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StraightPipeError {
    MissingInput {
        name: &'static str,
    },
    NonFiniteInput {
        name: &'static str,
        value: f64,
    },
    NonPositiveInput {
        name: &'static str,
        value: f64,
    },
    InvalidStationFraction {
        name: &'static str,
        value: f64,
    },
    InvalidLoadSpan {
        start_fraction: f64,
        end_fraction: f64,
    },
    InvalidDisplacementLength {
        expected: usize,
        actual: usize,
    },
    FrameKernel(FrameKernelError),
}

impl fmt::Display for StraightPipeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingInput { name } => write!(f, "missing solve-required input {name}"),
            Self::NonFiniteInput { name, value } => {
                write!(f, "{name} must be finite, got {value}")
            }
            Self::NonPositiveInput { name, value } => {
                write!(f, "{name} must be positive, got {value}")
            }
            Self::InvalidStationFraction { name, value } => {
                write!(f, "{name} must satisfy 0 <= value <= 1, got {value}")
            }
            Self::InvalidLoadSpan {
                start_fraction,
                end_fraction,
            } => write!(
                f,
                "uniform load span must satisfy 0 <= start < end <= 1, got {start_fraction}-{end_fraction}"
            ),
            Self::InvalidDisplacementLength { expected, actual } => {
                write!(
                    f,
                    "displacement vector length must be {expected}, got {actual}"
                )
            }
            Self::FrameKernel(error) => write!(f, "{error}"),
        }
    }
}

impl Error for StraightPipeError {}

impl From<FrameKernelError> for StraightPipeError {
    fn from(error: FrameKernelError) -> Self {
        Self::FrameKernel(error)
    }
}

impl StraightPipeSectionProperties {
    pub fn new(
        elastic_modulus: f64,
        shear_modulus: f64,
        area: f64,
        second_moment_y: f64,
        second_moment_z: f64,
        torsion_constant: f64,
        mass_per_length: Option<f64>,
    ) -> Result<Self, StraightPipeError> {
        validate_positive_finite("elastic_modulus", elastic_modulus)?;
        validate_positive_finite("shear_modulus", shear_modulus)?;
        validate_positive_finite("area", area)?;
        validate_positive_finite("second_moment_y", second_moment_y)?;
        validate_positive_finite("second_moment_z", second_moment_z)?;
        validate_positive_finite("torsion_constant", torsion_constant)?;
        if let Some(value) = mass_per_length {
            validate_positive_finite("mass_per_length", value)?;
        }

        Ok(Self {
            elastic_modulus,
            shear_modulus,
            area,
            second_moment_y,
            second_moment_z,
            torsion_constant,
            mass_per_length,
        })
    }

    pub fn frame_section(&self) -> Result<FrameSection, StraightPipeError> {
        Ok(FrameSection::new(
            self.elastic_modulus,
            self.shear_modulus,
            self.area,
            self.second_moment_y,
            self.second_moment_z,
            self.torsion_constant,
        )?)
    }
}

impl StraightPipeElement {
    pub fn new(
        element_id: impl Into<String>,
        node_i: FrameNode,
        node_j: FrameNode,
        section: StraightPipeSectionProperties,
        y_reference: [f64; 3],
    ) -> Result<Self, StraightPipeError> {
        let element = Self {
            element_id: element_id.into(),
            node_i,
            node_j,
            section,
            y_reference,
        };
        element.frame_element()?;
        Ok(element)
    }

    pub fn length(&self) -> Result<f64, StraightPipeError> {
        Ok(self.frame_element()?.length()?)
    }

    pub fn frame_element(&self) -> Result<FrameElement, StraightPipeError> {
        Ok(FrameElement::new(
            self.node_i,
            self.node_j,
            self.section.frame_section()?,
            self.y_reference,
        )?)
    }

    pub fn local_stiffness(&self) -> Result<Matrix12, StraightPipeError> {
        Ok(self.frame_element()?.local_stiffness()?)
    }

    pub fn global_stiffness(&self) -> Result<Matrix12, StraightPipeError> {
        Ok(self.frame_element()?.global_stiffness()?)
    }

    pub fn weight_hook(&self, gravity: f64) -> Result<WeightHook, StraightPipeError> {
        validate_positive_finite("gravity", gravity)?;
        let mass_per_length =
            self.section
                .mass_per_length
                .ok_or(StraightPipeError::MissingInput {
                    name: "mass_per_length",
                })?;
        let weight_force_per_length = mass_per_length * gravity;
        validate_positive_finite("weight_force_per_length", weight_force_per_length)?;

        Ok(WeightHook {
            mass_per_length,
            gravity,
            weight_force_per_length,
        })
    }

    pub fn recover_local_forces(
        &self,
        global_element_displacements: &[f64],
    ) -> Result<LocalElementForces, StraightPipeError> {
        if global_element_displacements.len() != ELEMENT_DOF {
            return Err(StraightPipeError::InvalidDisplacementLength {
                expected: ELEMENT_DOF,
                actual: global_element_displacements.len(),
            });
        }
        validate_finite_slice("global_element_displacements", global_element_displacements)?;

        let frame_element = self.frame_element()?;
        let orientation = frame_element.orientation()?;
        let local_stiffness = frame_element.local_stiffness()?;
        let local_displacements =
            transform_global_displacements_to_local(&orientation, global_element_displacements);
        let local_forces = multiply_matrix_vector(&local_stiffness, &local_displacements);

        Ok(LocalElementForces {
            local_displacements,
            local_forces,
        })
    }

    pub fn recover_end_resultants(
        &self,
        global_element_displacements: &[f64],
        end: PipeEnd,
    ) -> Result<PipeEndResultants, StraightPipeError> {
        let recovered = self.recover_local_forces(global_element_displacements)?;
        Ok(PipeEndResultants::from_local_forces(
            &recovered.local_forces,
            end,
        ))
    }

    pub fn recover_local_forces_from_global_model(
        &self,
        global_model_displacements: &[f64],
    ) -> Result<LocalElementForces, StraightPipeError> {
        let required = (self.node_i.index.max(self.node_j.index) + 1) * DOF_PER_NODE;
        if global_model_displacements.len() < required {
            return Err(StraightPipeError::InvalidDisplacementLength {
                expected: required,
                actual: global_model_displacements.len(),
            });
        }
        validate_finite_slice("global_model_displacements", global_model_displacements)?;

        let mut element_displacements = [0.0; ELEMENT_DOF];
        copy_node_displacements(
            global_model_displacements,
            self.node_i.index,
            &mut element_displacements[0..DOF_PER_NODE],
        );
        copy_node_displacements(
            global_model_displacements,
            self.node_j.index,
            &mut element_displacements[DOF_PER_NODE..ELEMENT_DOF],
        );

        self.recover_local_forces(&element_displacements)
    }

    pub fn recover_end_resultants_from_global_model(
        &self,
        global_model_displacements: &[f64],
        end: PipeEnd,
    ) -> Result<PipeEndResultants, StraightPipeError> {
        let recovered = self.recover_local_forces_from_global_model(global_model_displacements)?;
        Ok(PipeEndResultants::from_local_forces(
            &recovered.local_forces,
            end,
        ))
    }

    pub fn equivalent_nodal_loads(
        &self,
        uniform_loads: &[UniformLocalLoad],
        point_forces: &[PointLocalForce],
    ) -> Result<[f64; ELEMENT_DOF], StraightPipeError> {
        let spanned_uniforms = full_span_local_loads(uniform_loads);
        self.equivalent_nodal_loads_with_spans(&spanned_uniforms, point_forces)
    }

    pub fn equivalent_nodal_loads_with_spans(
        &self,
        uniform_loads: &[SpannedUniformLocalLoad],
        point_forces: &[PointLocalForce],
    ) -> Result<[f64; ELEMENT_DOF], StraightPipeError> {
        let length = self.length()?;
        let mut loads = [0.0; ELEMENT_DOF];

        for load in uniform_loads {
            add_spanned_uniform_equivalent_load(&mut loads, length, *load)?;
        }
        for load in point_forces {
            add_point_equivalent_load(&mut loads, length, *load)?;
        }

        validate_finite_array("equivalent_nodal_loads", &loads)?;
        Ok(loads)
    }

    pub fn equivalent_global_nodal_loads(
        &self,
        uniform_loads: &[GlobalUniformLoad],
        point_forces: &[GlobalPointForce],
    ) -> Result<[f64; ELEMENT_DOF], StraightPipeError> {
        let spanned_uniforms = full_span_global_loads(uniform_loads);
        self.equivalent_global_nodal_loads_with_spans(&spanned_uniforms, point_forces)
    }

    pub fn equivalent_global_nodal_loads_with_spans(
        &self,
        uniform_loads: &[SpannedGlobalUniformLoad],
        point_forces: &[GlobalPointForce],
    ) -> Result<[f64; ELEMENT_DOF], StraightPipeError> {
        let orientation = self.frame_element()?.orientation()?;
        let mut local_uniforms = Vec::with_capacity(uniform_loads.len() * 3);
        let mut local_points = Vec::with_capacity(point_forces.len() * 3);

        for load in uniform_loads {
            let local = transform_global_vector_to_local(&orientation, load.force_per_length);
            local_uniforms.push(SpannedUniformLocalLoad::new(
                LocalLoadDirection::X,
                local[0],
                load.span,
            )?);
            local_uniforms.push(SpannedUniformLocalLoad::new(
                LocalLoadDirection::Y,
                local[1],
                load.span,
            )?);
            local_uniforms.push(SpannedUniformLocalLoad::new(
                LocalLoadDirection::Z,
                local[2],
                load.span,
            )?);
        }

        for load in point_forces {
            let local = transform_global_vector_to_local(&orientation, load.force);
            local_points.push(PointLocalForce::new(
                load.station_fraction,
                LocalLoadDirection::X,
                local[0],
            )?);
            local_points.push(PointLocalForce::new(
                load.station_fraction,
                LocalLoadDirection::Y,
                local[1],
            )?);
            local_points.push(PointLocalForce::new(
                load.station_fraction,
                LocalLoadDirection::Z,
                local[2],
            )?);
        }

        let local_loads = self.equivalent_nodal_loads_with_spans(&local_uniforms, &local_points)?;
        let global_loads = transform_local_element_vector_to_global(&orientation, &local_loads);
        validate_finite_array("equivalent_global_nodal_loads", &global_loads)?;
        Ok(global_loads)
    }

    pub fn equivalent_local_axial_effect_loads(
        &self,
        axial_effects: &[StraightPipeAxialEffect],
    ) -> Result<[f64; ELEMENT_DOF], StraightPipeError> {
        let mut loads = [0.0; ELEMENT_DOF];
        for effect in axial_effects {
            validate_axial_effect(*effect)?;
            loads[UX] -= effect.axial_force;
            loads[DOF_PER_NODE + UX] += effect.axial_force;
        }
        validate_finite_array("equivalent_local_axial_effect_loads", &loads)?;
        Ok(loads)
    }

    pub fn equivalent_global_axial_effect_loads(
        &self,
        axial_effects: &[StraightPipeAxialEffect],
    ) -> Result<[f64; ELEMENT_DOF], StraightPipeError> {
        let orientation = self.frame_element()?.orientation()?;
        let local_loads = self.equivalent_local_axial_effect_loads(axial_effects)?;
        let global_loads = transform_local_element_vector_to_global(&orientation, &local_loads);
        validate_finite_array("equivalent_global_axial_effect_loads", &global_loads)?;
        Ok(global_loads)
    }

    pub fn recover_local_forces_with_axial_effects(
        &self,
        global_element_displacements: &[f64],
        axial_effects: &[StraightPipeAxialEffect],
    ) -> Result<LocalElementForces, StraightPipeError> {
        let recovered = self.recover_local_forces(global_element_displacements)?;
        let equivalent_loads = self.equivalent_local_axial_effect_loads(axial_effects)?;
        let local_forces = subtract_equivalent_loads(&recovered.local_forces, &equivalent_loads);
        validate_finite_array("axial_effect_corrected_local_forces", &local_forces)?;
        Ok(LocalElementForces {
            local_displacements: recovered.local_displacements,
            local_forces,
        })
    }

    pub fn recover_local_forces_from_global_model_with_axial_effects(
        &self,
        global_model_displacements: &[f64],
        axial_effects: &[StraightPipeAxialEffect],
    ) -> Result<LocalElementForces, StraightPipeError> {
        let recovered = self.recover_local_forces_from_global_model(global_model_displacements)?;
        let equivalent_loads = self.equivalent_local_axial_effect_loads(axial_effects)?;
        let local_forces = subtract_equivalent_loads(&recovered.local_forces, &equivalent_loads);
        validate_finite_array("axial_effect_corrected_local_forces", &local_forces)?;
        Ok(LocalElementForces {
            local_displacements: recovered.local_displacements,
            local_forces,
        })
    }

    pub fn recover_end_resultants_with_axial_effects(
        &self,
        global_element_displacements: &[f64],
        end: PipeEnd,
        axial_effects: &[StraightPipeAxialEffect],
    ) -> Result<PipeEndResultants, StraightPipeError> {
        let recovered = self
            .recover_local_forces_with_axial_effects(global_element_displacements, axial_effects)?;
        Ok(PipeEndResultants::from_local_forces(
            &recovered.local_forces,
            end,
        ))
    }

    pub fn recover_end_resultants_from_global_model_with_axial_effects(
        &self,
        global_model_displacements: &[f64],
        end: PipeEnd,
        axial_effects: &[StraightPipeAxialEffect],
    ) -> Result<PipeEndResultants, StraightPipeError> {
        let recovered = self.recover_local_forces_from_global_model_with_axial_effects(
            global_model_displacements,
            axial_effects,
        )?;
        Ok(PipeEndResultants::from_local_forces(
            &recovered.local_forces,
            end,
        ))
    }

    pub fn recover_end_resultants_with_spans_and_axial_effects(
        &self,
        global_element_displacements: &[f64],
        end: PipeEnd,
        uniform_loads: &[SpannedUniformLocalLoad],
        point_forces: &[PointLocalForce],
        axial_effects: &[StraightPipeAxialEffect],
    ) -> Result<PipeEndResultants, StraightPipeError> {
        let recovered = self.recover_local_forces(global_element_displacements)?;
        let loaded_forces = self.apply_load_and_axial_corrections(
            &recovered.local_forces,
            uniform_loads,
            point_forces,
            axial_effects,
        )?;
        Ok(PipeEndResultants::from_local_forces(&loaded_forces, end))
    }

    pub fn recover_end_resultants_from_global_model_with_spans_and_axial_effects(
        &self,
        global_model_displacements: &[f64],
        end: PipeEnd,
        uniform_loads: &[SpannedUniformLocalLoad],
        point_forces: &[PointLocalForce],
        axial_effects: &[StraightPipeAxialEffect],
    ) -> Result<PipeEndResultants, StraightPipeError> {
        let recovered = self.recover_local_forces_from_global_model(global_model_displacements)?;
        let loaded_forces = self.apply_load_and_axial_corrections(
            &recovered.local_forces,
            uniform_loads,
            point_forces,
            axial_effects,
        )?;
        Ok(PipeEndResultants::from_local_forces(&loaded_forces, end))
    }

    pub fn recover_station_resultants_with_axial_effects(
        &self,
        global_element_displacements: &[f64],
        station_fraction: f64,
        axial_effects: &[StraightPipeAxialEffect],
    ) -> Result<StationResultants, StraightPipeError> {
        let recovered = self
            .recover_local_forces_with_axial_effects(global_element_displacements, axial_effects)?;
        let i_end = PipeEndResultants::from_local_forces(&recovered.local_forces, PipeEnd::I);
        self.station_resultants_from_i_end(i_end, station_fraction, &[], &[])
    }

    pub fn recover_station_resultant_sweep_with_axial_effects(
        &self,
        global_element_displacements: &[f64],
        station_fractions: &[f64],
        axial_effects: &[StraightPipeAxialEffect],
    ) -> Result<Vec<StationResultants>, StraightPipeError> {
        let recovered = self
            .recover_local_forces_with_axial_effects(global_element_displacements, axial_effects)?;
        let i_end = PipeEndResultants::from_local_forces(&recovered.local_forces, PipeEnd::I);
        self.station_resultant_sweep_from_i_end(i_end, station_fractions, &[], &[])
    }

    pub fn recover_station_resultants_with_spans_and_axial_effects(
        &self,
        global_element_displacements: &[f64],
        station_fraction: f64,
        uniform_loads: &[SpannedUniformLocalLoad],
        point_forces: &[PointLocalForce],
        axial_effects: &[StraightPipeAxialEffect],
    ) -> Result<StationResultants, StraightPipeError> {
        let recovered = self.recover_local_forces(global_element_displacements)?;
        let loaded_forces = self.apply_load_and_axial_corrections(
            &recovered.local_forces,
            uniform_loads,
            point_forces,
            axial_effects,
        )?;
        let i_end = PipeEndResultants::from_local_forces(&loaded_forces, PipeEnd::I);
        self.station_resultants_from_i_end_with_spans(
            i_end,
            station_fraction,
            uniform_loads,
            point_forces,
        )
    }

    pub fn recover_station_resultant_sweep_with_spans_and_axial_effects(
        &self,
        global_element_displacements: &[f64],
        station_fractions: &[f64],
        uniform_loads: &[SpannedUniformLocalLoad],
        point_forces: &[PointLocalForce],
        axial_effects: &[StraightPipeAxialEffect],
    ) -> Result<Vec<StationResultants>, StraightPipeError> {
        let recovered = self.recover_local_forces(global_element_displacements)?;
        let loaded_forces = self.apply_load_and_axial_corrections(
            &recovered.local_forces,
            uniform_loads,
            point_forces,
            axial_effects,
        )?;
        let i_end = PipeEndResultants::from_local_forces(&loaded_forces, PipeEnd::I);
        self.station_resultant_sweep_from_i_end_with_spans(
            i_end,
            station_fractions,
            uniform_loads,
            point_forces,
        )
    }

    pub fn recover_station_resultants_from_global_model_with_axial_effects(
        &self,
        global_model_displacements: &[f64],
        station_fraction: f64,
        axial_effects: &[StraightPipeAxialEffect],
    ) -> Result<StationResultants, StraightPipeError> {
        let recovered = self.recover_local_forces_from_global_model_with_axial_effects(
            global_model_displacements,
            axial_effects,
        )?;
        let i_end = PipeEndResultants::from_local_forces(&recovered.local_forces, PipeEnd::I);
        self.station_resultants_from_i_end(i_end, station_fraction, &[], &[])
    }

    pub fn recover_station_resultant_sweep_from_global_model_with_axial_effects(
        &self,
        global_model_displacements: &[f64],
        station_fractions: &[f64],
        axial_effects: &[StraightPipeAxialEffect],
    ) -> Result<Vec<StationResultants>, StraightPipeError> {
        let recovered = self.recover_local_forces_from_global_model_with_axial_effects(
            global_model_displacements,
            axial_effects,
        )?;
        let i_end = PipeEndResultants::from_local_forces(&recovered.local_forces, PipeEnd::I);
        self.station_resultant_sweep_from_i_end(i_end, station_fractions, &[], &[])
    }

    pub fn recover_station_resultants_from_global_model_with_spans_and_axial_effects(
        &self,
        global_model_displacements: &[f64],
        station_fraction: f64,
        uniform_loads: &[SpannedUniformLocalLoad],
        point_forces: &[PointLocalForce],
        axial_effects: &[StraightPipeAxialEffect],
    ) -> Result<StationResultants, StraightPipeError> {
        let recovered = self.recover_local_forces_from_global_model(global_model_displacements)?;
        let loaded_forces = self.apply_load_and_axial_corrections(
            &recovered.local_forces,
            uniform_loads,
            point_forces,
            axial_effects,
        )?;
        let i_end = PipeEndResultants::from_local_forces(&loaded_forces, PipeEnd::I);
        self.station_resultants_from_i_end_with_spans(
            i_end,
            station_fraction,
            uniform_loads,
            point_forces,
        )
    }

    pub fn recover_station_resultant_sweep_from_global_model_with_spans_and_axial_effects(
        &self,
        global_model_displacements: &[f64],
        station_fractions: &[f64],
        uniform_loads: &[SpannedUniformLocalLoad],
        point_forces: &[PointLocalForce],
        axial_effects: &[StraightPipeAxialEffect],
    ) -> Result<Vec<StationResultants>, StraightPipeError> {
        let recovered = self.recover_local_forces_from_global_model(global_model_displacements)?;
        let loaded_forces = self.apply_load_and_axial_corrections(
            &recovered.local_forces,
            uniform_loads,
            point_forces,
            axial_effects,
        )?;
        let i_end = PipeEndResultants::from_local_forces(&loaded_forces, PipeEnd::I);
        self.station_resultant_sweep_from_i_end_with_spans(
            i_end,
            station_fractions,
            uniform_loads,
            point_forces,
        )
    }

    pub fn station_resultants_from_i_end(
        &self,
        i_end: PipeEndResultants,
        station_fraction: f64,
        uniform_loads: &[UniformLocalLoad],
        point_forces: &[PointLocalForce],
    ) -> Result<StationResultants, StraightPipeError> {
        let spanned_uniforms = full_span_local_loads(uniform_loads);
        self.station_resultants_from_i_end_with_spans(
            i_end,
            station_fraction,
            &spanned_uniforms,
            point_forces,
        )
    }

    pub fn station_resultants_from_i_end_with_spans(
        &self,
        i_end: PipeEndResultants,
        station_fraction: f64,
        uniform_loads: &[SpannedUniformLocalLoad],
        point_forces: &[PointLocalForce],
    ) -> Result<StationResultants, StraightPipeError> {
        validate_station_fraction("station_fraction", station_fraction)?;
        let length = self.length()?;
        let distance = station_fraction * length;

        let mut resultants = StationResultants {
            station_fraction,
            distance_from_i: distance,
            axial_force: i_end.axial_force,
            shear_force_y: i_end.shear_force_y,
            shear_force_z: i_end.shear_force_z,
            torsional_moment: i_end.torsional_moment,
            bending_moment_y: i_end.bending_moment_y + i_end.shear_force_z * distance,
            bending_moment_z: i_end.bending_moment_z - i_end.shear_force_y * distance,
        };

        for load in uniform_loads {
            validate_spanned_uniform_load(*load)?;
            accumulate_spanned_uniform_station_resultants(&mut resultants, *load, length, distance);
        }
        for load in point_forces {
            accumulate_point_station_resultants(&mut resultants, *load, length, distance);
        }

        validate_station_resultants(&resultants)?;
        Ok(resultants)
    }

    pub fn station_resultant_sweep_from_i_end(
        &self,
        i_end: PipeEndResultants,
        station_fractions: &[f64],
        uniform_loads: &[UniformLocalLoad],
        point_forces: &[PointLocalForce],
    ) -> Result<Vec<StationResultants>, StraightPipeError> {
        let spanned_uniforms = full_span_local_loads(uniform_loads);
        self.station_resultant_sweep_from_i_end_with_spans(
            i_end,
            station_fractions,
            &spanned_uniforms,
            point_forces,
        )
    }

    pub fn station_resultant_sweep_from_i_end_with_spans(
        &self,
        i_end: PipeEndResultants,
        station_fractions: &[f64],
        uniform_loads: &[SpannedUniformLocalLoad],
        point_forces: &[PointLocalForce],
    ) -> Result<Vec<StationResultants>, StraightPipeError> {
        station_fractions
            .iter()
            .map(|&station_fraction| {
                self.station_resultants_from_i_end_with_spans(
                    i_end,
                    station_fraction,
                    uniform_loads,
                    point_forces,
                )
            })
            .collect()
    }

    pub fn recover_station_resultants(
        &self,
        global_element_displacements: &[f64],
        station_fraction: f64,
        uniform_loads: &[UniformLocalLoad],
        point_forces: &[PointLocalForce],
    ) -> Result<StationResultants, StraightPipeError> {
        let spanned_uniforms = full_span_local_loads(uniform_loads);
        self.recover_station_resultants_with_spans(
            global_element_displacements,
            station_fraction,
            &spanned_uniforms,
            point_forces,
        )
    }

    pub fn recover_station_resultants_with_spans(
        &self,
        global_element_displacements: &[f64],
        station_fraction: f64,
        uniform_loads: &[SpannedUniformLocalLoad],
        point_forces: &[PointLocalForce],
    ) -> Result<StationResultants, StraightPipeError> {
        let recovered = self.recover_local_forces(global_element_displacements)?;
        let equivalent_loads =
            self.equivalent_nodal_loads_with_spans(uniform_loads, point_forces)?;
        let loaded_forces = subtract_equivalent_loads(&recovered.local_forces, &equivalent_loads);
        let i_end = PipeEndResultants::from_local_forces(&loaded_forces, PipeEnd::I);
        self.station_resultants_from_i_end_with_spans(
            i_end,
            station_fraction,
            uniform_loads,
            point_forces,
        )
    }

    pub fn recover_station_resultant_sweep(
        &self,
        global_element_displacements: &[f64],
        station_fractions: &[f64],
        uniform_loads: &[UniformLocalLoad],
        point_forces: &[PointLocalForce],
    ) -> Result<Vec<StationResultants>, StraightPipeError> {
        let spanned_uniforms = full_span_local_loads(uniform_loads);
        self.recover_station_resultant_sweep_with_spans(
            global_element_displacements,
            station_fractions,
            &spanned_uniforms,
            point_forces,
        )
    }

    pub fn recover_station_resultant_sweep_with_spans(
        &self,
        global_element_displacements: &[f64],
        station_fractions: &[f64],
        uniform_loads: &[SpannedUniformLocalLoad],
        point_forces: &[PointLocalForce],
    ) -> Result<Vec<StationResultants>, StraightPipeError> {
        let recovered = self.recover_local_forces(global_element_displacements)?;
        let equivalent_loads =
            self.equivalent_nodal_loads_with_spans(uniform_loads, point_forces)?;
        let loaded_forces = subtract_equivalent_loads(&recovered.local_forces, &equivalent_loads);
        let i_end = PipeEndResultants::from_local_forces(&loaded_forces, PipeEnd::I);
        self.station_resultant_sweep_from_i_end_with_spans(
            i_end,
            station_fractions,
            uniform_loads,
            point_forces,
        )
    }

    pub fn recover_station_resultants_from_global_model(
        &self,
        global_model_displacements: &[f64],
        station_fraction: f64,
        uniform_loads: &[UniformLocalLoad],
        point_forces: &[PointLocalForce],
    ) -> Result<StationResultants, StraightPipeError> {
        let spanned_uniforms = full_span_local_loads(uniform_loads);
        self.recover_station_resultants_from_global_model_with_spans(
            global_model_displacements,
            station_fraction,
            &spanned_uniforms,
            point_forces,
        )
    }

    pub fn recover_station_resultant_sweep_from_global_model(
        &self,
        global_model_displacements: &[f64],
        station_fractions: &[f64],
        uniform_loads: &[UniformLocalLoad],
        point_forces: &[PointLocalForce],
    ) -> Result<Vec<StationResultants>, StraightPipeError> {
        let spanned_uniforms = full_span_local_loads(uniform_loads);
        self.recover_station_resultant_sweep_from_global_model_with_spans(
            global_model_displacements,
            station_fractions,
            &spanned_uniforms,
            point_forces,
        )
    }

    pub fn recover_station_resultant_sweep_from_global_model_with_spans(
        &self,
        global_model_displacements: &[f64],
        station_fractions: &[f64],
        uniform_loads: &[SpannedUniformLocalLoad],
        point_forces: &[PointLocalForce],
    ) -> Result<Vec<StationResultants>, StraightPipeError> {
        let recovered = self.recover_local_forces_from_global_model(global_model_displacements)?;
        let equivalent_loads =
            self.equivalent_nodal_loads_with_spans(uniform_loads, point_forces)?;
        let loaded_forces = subtract_equivalent_loads(&recovered.local_forces, &equivalent_loads);
        let i_end = PipeEndResultants::from_local_forces(&loaded_forces, PipeEnd::I);
        self.station_resultant_sweep_from_i_end_with_spans(
            i_end,
            station_fractions,
            uniform_loads,
            point_forces,
        )
    }

    pub fn recover_station_resultants_from_global_model_with_spans(
        &self,
        global_model_displacements: &[f64],
        station_fraction: f64,
        uniform_loads: &[SpannedUniformLocalLoad],
        point_forces: &[PointLocalForce],
    ) -> Result<StationResultants, StraightPipeError> {
        let recovered = self.recover_local_forces_from_global_model(global_model_displacements)?;
        let equivalent_loads =
            self.equivalent_nodal_loads_with_spans(uniform_loads, point_forces)?;
        let loaded_forces = subtract_equivalent_loads(&recovered.local_forces, &equivalent_loads);
        let i_end = PipeEndResultants::from_local_forces(&loaded_forces, PipeEnd::I);
        self.station_resultants_from_i_end_with_spans(
            i_end,
            station_fraction,
            uniform_loads,
            point_forces,
        )
    }

    fn apply_load_and_axial_corrections(
        &self,
        local_forces: &[f64; ELEMENT_DOF],
        uniform_loads: &[SpannedUniformLocalLoad],
        point_forces: &[PointLocalForce],
        axial_effects: &[StraightPipeAxialEffect],
    ) -> Result<[f64; ELEMENT_DOF], StraightPipeError> {
        let load_equivalent =
            self.equivalent_nodal_loads_with_spans(uniform_loads, point_forces)?;
        let axial_equivalent = self.equivalent_local_axial_effect_loads(axial_effects)?;
        let loaded_forces = subtract_equivalent_loads(local_forces, &load_equivalent);
        let loaded_forces = subtract_equivalent_loads(&loaded_forces, &axial_equivalent);
        validate_finite_array("load_and_axial_corrected_local_forces", &loaded_forces)?;
        Ok(loaded_forces)
    }
}

fn add_spanned_uniform_equivalent_load(
    loads: &mut [f64; ELEMENT_DOF],
    length: f64,
    load: SpannedUniformLocalLoad,
) -> Result<(), StraightPipeError> {
    validate_spanned_uniform_load(load)?;
    let a = load.span.start_fraction;
    let b = load.span.end_fraction;
    let q = load.force_per_length;

    let axial_i = q * length * ((b - 0.5 * b * b) - (a - 0.5 * a * a));
    let axial_j = q * length * (0.5 * b * b - 0.5 * a * a);
    let transverse_i =
        q * length * ((b - b.powi(3) + 0.5 * b.powi(4)) - (a - a.powi(3) + 0.5 * a.powi(4)));
    let rotation_i = q
        * length
        * length
        * ((0.5 * b * b - (2.0 / 3.0) * b.powi(3) + 0.25 * b.powi(4))
            - (0.5 * a * a - (2.0 / 3.0) * a.powi(3) + 0.25 * a.powi(4)));
    let transverse_j = q * length * ((b.powi(3) - 0.5 * b.powi(4)) - (a.powi(3) - 0.5 * a.powi(4)));
    let rotation_j = q
        * length
        * length
        * (((-b.powi(3) / 3.0) + 0.25 * b.powi(4)) - ((-a.powi(3) / 3.0) + 0.25 * a.powi(4)));
    validate_finite("spanned_uniform_i_force", axial_i)?;
    validate_finite("spanned_uniform_j_force", axial_j)?;
    validate_finite("spanned_uniform_i_transverse", transverse_i)?;
    validate_finite("spanned_uniform_i_rotation", rotation_i)?;
    validate_finite("spanned_uniform_j_transverse", transverse_j)?;
    validate_finite("spanned_uniform_j_rotation", rotation_j)?;

    match load.direction {
        LocalLoadDirection::X => {
            loads[UX] += axial_i;
            loads[DOF_PER_NODE + UX] += axial_j;
        }
        LocalLoadDirection::Y => {
            loads[UY] += transverse_i;
            loads[RZ] += rotation_i;
            loads[DOF_PER_NODE + UY] += transverse_j;
            loads[DOF_PER_NODE + RZ] += rotation_j;
        }
        LocalLoadDirection::Z => {
            loads[UZ] += transverse_i;
            loads[RY] -= rotation_i;
            loads[DOF_PER_NODE + UZ] += transverse_j;
            loads[DOF_PER_NODE + RY] -= rotation_j;
        }
    }
    Ok(())
}

fn add_point_equivalent_load(
    loads: &mut [f64; ELEMENT_DOF],
    length: f64,
    load: PointLocalForce,
) -> Result<(), StraightPipeError> {
    validate_station_fraction("point_force_station_fraction", load.station_fraction)?;
    validate_finite("point_force", load.force)?;
    let r = load.station_fraction;
    let n_i = 1.0 - r;
    let n_j = r;
    let h_i = 1.0 - 3.0 * r * r + 2.0 * r * r * r;
    let theta_i = length * (r - 2.0 * r * r + r * r * r);
    let h_j = 3.0 * r * r - 2.0 * r * r * r;
    let theta_j = length * (-r * r + r * r * r);

    match load.direction {
        LocalLoadDirection::X => {
            loads[UX] += load.force * n_i;
            loads[DOF_PER_NODE + UX] += load.force * n_j;
        }
        LocalLoadDirection::Y => {
            loads[UY] += load.force * h_i;
            loads[RZ] += load.force * theta_i;
            loads[DOF_PER_NODE + UY] += load.force * h_j;
            loads[DOF_PER_NODE + RZ] += load.force * theta_j;
        }
        LocalLoadDirection::Z => {
            loads[UZ] += load.force * h_i;
            loads[RY] -= load.force * theta_i;
            loads[DOF_PER_NODE + UZ] += load.force * h_j;
            loads[DOF_PER_NODE + RY] -= load.force * theta_j;
        }
    }
    Ok(())
}

fn accumulate_spanned_uniform_station_resultants(
    resultants: &mut StationResultants,
    load: SpannedUniformLocalLoad,
    length: f64,
    distance: f64,
) {
    let span_start = load.span.start_fraction * length;
    let span_end = load.span.end_fraction * length;
    let active_end = distance.min(span_end);
    if active_end <= span_start {
        return;
    }
    let active_length = active_end - span_start;
    let lever_integral =
        distance * active_length - 0.5 * (active_end * active_end - span_start * span_start);

    match load.direction {
        LocalLoadDirection::X => {
            resultants.axial_force += load.force_per_length * active_length;
        }
        LocalLoadDirection::Y => {
            resultants.shear_force_y += load.force_per_length * active_length;
            resultants.bending_moment_z -= load.force_per_length * lever_integral;
        }
        LocalLoadDirection::Z => {
            resultants.shear_force_z += load.force_per_length * active_length;
            resultants.bending_moment_y += load.force_per_length * lever_integral;
        }
    }
}

fn accumulate_point_station_resultants(
    resultants: &mut StationResultants,
    load: PointLocalForce,
    length: f64,
    distance: f64,
) {
    let load_distance = load.station_fraction * length;
    if load_distance > distance {
        return;
    }
    let lever = distance - load_distance;
    match load.direction {
        LocalLoadDirection::X => {
            resultants.axial_force += load.force;
        }
        LocalLoadDirection::Y => {
            resultants.shear_force_y += load.force;
            resultants.bending_moment_z -= load.force * lever;
        }
        LocalLoadDirection::Z => {
            resultants.shear_force_z += load.force;
            resultants.bending_moment_y += load.force * lever;
        }
    }
}

fn transform_global_displacements_to_local(
    orientation: &FrameOrientation,
    global_element_displacements: &[f64],
) -> [f64; ELEMENT_DOF] {
    let transform = orientation.transformation_matrix();
    let mut local = [0.0; ELEMENT_DOF];
    for row in 0..ELEMENT_DOF {
        for col in 0..ELEMENT_DOF {
            local[row] += transform[row][col] * global_element_displacements[col];
        }
    }
    local
}

fn transform_global_vector_to_local(orientation: &FrameOrientation, global: [f64; 3]) -> [f64; 3] {
    [
        orientation.local_axes[0][0] * global[0]
            + orientation.local_axes[0][1] * global[1]
            + orientation.local_axes[0][2] * global[2],
        orientation.local_axes[1][0] * global[0]
            + orientation.local_axes[1][1] * global[1]
            + orientation.local_axes[1][2] * global[2],
        orientation.local_axes[2][0] * global[0]
            + orientation.local_axes[2][1] * global[1]
            + orientation.local_axes[2][2] * global[2],
    ]
}

fn transform_local_element_vector_to_global(
    orientation: &FrameOrientation,
    local_element_vector: &[f64; ELEMENT_DOF],
) -> [f64; ELEMENT_DOF] {
    let transform = orientation.transformation_matrix();
    let mut global = [0.0; ELEMENT_DOF];
    for row in 0..ELEMENT_DOF {
        for col in 0..ELEMENT_DOF {
            global[row] += transform[col][row] * local_element_vector[col];
        }
    }
    global
}

fn multiply_matrix_vector(matrix: &Matrix12, vector: &[f64; ELEMENT_DOF]) -> [f64; ELEMENT_DOF] {
    let mut result = [0.0; ELEMENT_DOF];
    for row in 0..ELEMENT_DOF {
        for col in 0..ELEMENT_DOF {
            result[row] += matrix[row][col] * vector[col];
        }
    }
    result
}

fn subtract_equivalent_loads(
    local_forces: &[f64; ELEMENT_DOF],
    equivalent_loads: &[f64; ELEMENT_DOF],
) -> [f64; ELEMENT_DOF] {
    let mut result = [0.0; ELEMENT_DOF];
    for index in 0..ELEMENT_DOF {
        result[index] = local_forces[index] - equivalent_loads[index];
    }
    result
}

fn copy_node_displacements(source: &[f64], node_index: usize, target: &mut [f64]) {
    let offset = node_index * DOF_PER_NODE;
    target.copy_from_slice(&source[offset..offset + DOF_PER_NODE]);
}

fn validate_positive_finite(name: &'static str, value: f64) -> Result<(), StraightPipeError> {
    if !value.is_finite() {
        return Err(StraightPipeError::NonFiniteInput { name, value });
    }
    if value <= 0.0 {
        return Err(StraightPipeError::NonPositiveInput { name, value });
    }
    Ok(())
}

fn validate_finite(name: &'static str, value: f64) -> Result<(), StraightPipeError> {
    if !value.is_finite() {
        return Err(StraightPipeError::NonFiniteInput { name, value });
    }
    Ok(())
}

fn validate_finite_vector(name: &'static str, values: [f64; 3]) -> Result<(), StraightPipeError> {
    for value in values {
        validate_finite(name, value)?;
    }
    Ok(())
}

fn validate_station_fraction(name: &'static str, value: f64) -> Result<(), StraightPipeError> {
    if !value.is_finite() {
        return Err(StraightPipeError::NonFiniteInput { name, value });
    }
    if !(0.0..=1.0).contains(&value) {
        return Err(StraightPipeError::InvalidStationFraction { name, value });
    }
    Ok(())
}

fn validate_load_span(start_fraction: f64, end_fraction: f64) -> Result<(), StraightPipeError> {
    validate_station_fraction("span_start_fraction", start_fraction)?;
    validate_station_fraction("span_end_fraction", end_fraction)?;
    if start_fraction >= end_fraction {
        return Err(StraightPipeError::InvalidLoadSpan {
            start_fraction,
            end_fraction,
        });
    }
    Ok(())
}

fn validate_spanned_uniform_load(load: SpannedUniformLocalLoad) -> Result<(), StraightPipeError> {
    validate_load_span(load.span.start_fraction, load.span.end_fraction)?;
    validate_finite("force_per_length", load.force_per_length)
}

fn validate_axial_effect(effect: StraightPipeAxialEffect) -> Result<(), StraightPipeError> {
    validate_finite("axial_force", effect.axial_force)
}

fn validate_finite_slice(name: &'static str, values: &[f64]) -> Result<(), StraightPipeError> {
    for &value in values {
        if !value.is_finite() {
            return Err(StraightPipeError::NonFiniteInput { name, value });
        }
    }
    Ok(())
}

fn full_span_local_loads(uniform_loads: &[UniformLocalLoad]) -> Vec<SpannedUniformLocalLoad> {
    uniform_loads
        .iter()
        .map(|load| SpannedUniformLocalLoad {
            span: UniformLoadSpan::full(),
            direction: load.direction,
            force_per_length: load.force_per_length,
        })
        .collect()
}

fn full_span_global_loads(uniform_loads: &[GlobalUniformLoad]) -> Vec<SpannedGlobalUniformLoad> {
    uniform_loads
        .iter()
        .map(|load| SpannedGlobalUniformLoad {
            span: UniformLoadSpan::full(),
            force_per_length: load.force_per_length,
        })
        .collect()
}

fn validate_finite_array(
    name: &'static str,
    values: &[f64; ELEMENT_DOF],
) -> Result<(), StraightPipeError> {
    validate_finite_slice(name, values)
}

fn validate_station_resultants(resultants: &StationResultants) -> Result<(), StraightPipeError> {
    validate_finite("station_fraction", resultants.station_fraction)?;
    validate_finite("distance_from_i", resultants.distance_from_i)?;
    validate_finite("axial_force", resultants.axial_force)?;
    validate_finite("shear_force_y", resultants.shear_force_y)?;
    validate_finite("shear_force_z", resultants.shear_force_z)?;
    validate_finite("torsional_moment", resultants.torsional_moment)?;
    validate_finite("bending_moment_y", resultants.bending_moment_y)?;
    validate_finite("bending_moment_z", resultants.bending_moment_z)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn section(mass_per_length: Option<f64>) -> StraightPipeSectionProperties {
        StraightPipeSectionProperties::new(
            2.0e11,
            7.7e10,
            0.01,
            8.0e-6,
            9.0e-6,
            1.7e-5,
            mass_per_length,
        )
        .unwrap()
    }

    fn element(mass_per_length: Option<f64>) -> StraightPipeElement {
        StraightPipeElement::new(
            "pipe-1",
            FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
            FrameNode::new(1, [2.0, 0.0, 0.0]).unwrap(),
            section(mass_per_length),
            [0.0, 1.0, 0.0],
        )
        .unwrap()
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1.0e-6,
            "expected {actual} to be within tolerance of {expected}"
        );
    }

    #[test]
    fn straight_pipe_stiffness_matches_frame_kernel_boundary() {
        let pipe = element(Some(25.0));
        let frame = pipe.frame_element().unwrap();

        assert_eq!(pipe.length().unwrap(), 2.0);
        assert_eq!(
            pipe.local_stiffness().unwrap(),
            frame.local_stiffness().unwrap()
        );
        assert_eq!(
            pipe.global_stiffness().unwrap(),
            frame.global_stiffness().unwrap()
        );
    }

    #[test]
    fn weight_hook_requires_explicit_mass_per_length() {
        let pipe = element(Some(25.0));
        let hook = pipe.weight_hook(9.81).unwrap();

        assert_eq!(hook.mass_per_length, 25.0);
        assert_eq!(hook.gravity, 9.81);
        assert_eq!(hook.weight_force_per_length, 245.25);

        let missing = element(None).weight_hook(9.81).unwrap_err();
        assert_eq!(
            missing,
            StraightPipeError::MissingInput {
                name: "mass_per_length"
            }
        );
    }

    #[test]
    fn weight_hook_rejects_nonfinite_or_overflowing_values() {
        let pipe = element(Some(25.0));
        let bad_gravity = pipe.weight_hook(f64::NAN).unwrap_err();

        assert!(matches!(
            bad_gravity,
            StraightPipeError::NonFiniteInput {
                name: "gravity",
                value
            } if value.is_nan()
        ));

        let heavy_pipe = element(Some(1.0e308));
        let overflow = heavy_pipe.weight_hook(10.0).unwrap_err();
        assert_eq!(
            overflow,
            StraightPipeError::NonFiniteInput {
                name: "weight_force_per_length",
                value: f64::INFINITY
            }
        );
    }

    #[test]
    fn section_properties_reject_nonpositive_inputs() {
        let error =
            StraightPipeSectionProperties::new(2.0e11, 7.7e10, 0.0, 8.0e-6, 9.0e-6, 1.7e-5, None)
                .unwrap_err();

        assert_eq!(
            error,
            StraightPipeError::NonPositiveInput {
                name: "area",
                value: 0.0
            }
        );
    }

    #[test]
    fn section_properties_reject_nonfinite_mass_per_length() {
        let error = StraightPipeSectionProperties::new(
            2.0e11,
            7.7e10,
            0.01,
            8.0e-6,
            9.0e-6,
            1.7e-5,
            Some(f64::INFINITY),
        )
        .unwrap_err();

        assert_eq!(
            error,
            StraightPipeError::NonFiniteInput {
                name: "mass_per_length",
                value: f64::INFINITY
            }
        );
    }

    #[test]
    fn recovers_local_axial_end_forces_from_global_displacements() {
        let pipe = element(Some(25.0));
        let axial_extension = 0.001;
        let mut displacements = [0.0; ELEMENT_DOF];
        displacements[DOF_PER_NODE + UX] = axial_extension;

        let recovered = pipe.recover_local_forces(&displacements).unwrap();
        let expected_axial = pipe.section.elastic_modulus * pipe.section.area
            / pipe.length().unwrap()
            * axial_extension;

        assert_close(recovered.local_forces[UX], -expected_axial);
        assert_close(recovered.local_forces[DOF_PER_NODE + UX], expected_axial);
    }

    #[test]
    fn recovers_direct_pipe_end_resultants_without_sign_reinterpretation() {
        let pipe = element(Some(25.0));
        let axial_extension = 0.001;
        let torsional_rotation = 0.002;
        let mut displacements = [0.0; ELEMENT_DOF];
        displacements[DOF_PER_NODE + UX] = axial_extension;
        displacements[DOF_PER_NODE + RX] = torsional_rotation;

        let end_i = pipe
            .recover_end_resultants(&displacements, PipeEnd::I)
            .unwrap();
        let end_j = pipe
            .recover_end_resultants(&displacements, PipeEnd::J)
            .unwrap();
        let expected_axial = pipe.section.elastic_modulus * pipe.section.area
            / pipe.length().unwrap()
            * axial_extension;
        let expected_torsion = pipe.section.shear_modulus * pipe.section.torsion_constant
            / pipe.length().unwrap()
            * torsional_rotation;

        assert_eq!(end_i.end, PipeEnd::I);
        assert_eq!(end_j.end, PipeEnd::J);
        assert_close(end_i.axial_force, -expected_axial);
        assert_close(end_j.axial_force, expected_axial);
        assert_close(end_i.torsional_moment, -expected_torsion);
        assert_close(end_j.torsional_moment, expected_torsion);
        assert_close(end_j.bending_moment_y, 0.0);
        assert_close(end_j.bending_moment_z, 0.0);
    }

    #[test]
    fn recovers_rotated_member_axial_forces_in_local_coordinates() {
        let pipe = StraightPipeElement::new(
            "pipe-y",
            FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
            FrameNode::new(1, [0.0, 3.0, 0.0]).unwrap(),
            section(Some(25.0)),
            [1.0, 0.0, 0.0],
        )
        .unwrap();
        let axial_extension = 0.0015;
        let mut displacements = [0.0; ELEMENT_DOF];
        displacements[DOF_PER_NODE + UY] = axial_extension;

        let recovered = pipe.recover_local_forces(&displacements).unwrap();
        let expected_axial = pipe.section.elastic_modulus * pipe.section.area
            / pipe.length().unwrap()
            * axial_extension;

        assert_close(
            recovered.local_displacements[DOF_PER_NODE + UX],
            axial_extension,
        );
        assert_close(recovered.local_forces[UX], -expected_axial);
        assert_close(recovered.local_forces[DOF_PER_NODE + UX], expected_axial);
    }

    #[test]
    fn recovers_pipe_end_resultants_from_global_model_dofs() {
        let pipe = element(Some(25.0));
        let axial_extension = 0.001;
        let mut model_displacements = vec![0.0; 2 * DOF_PER_NODE];
        model_displacements[DOF_PER_NODE + UX] = axial_extension;

        let resultants = pipe
            .recover_end_resultants_from_global_model(&model_displacements, PipeEnd::J)
            .unwrap();
        let expected_axial = pipe.section.elastic_modulus * pipe.section.area
            / pipe.length().unwrap()
            * axial_extension;

        assert_eq!(resultants.end, PipeEnd::J);
        assert_close(resultants.axial_force, expected_axial);
        assert_close(resultants.shear_force_y, 0.0);
        assert_close(resultants.shear_force_z, 0.0);
    }

    #[test]
    fn uniform_equivalent_nodal_loads_follow_local_direction_signs() {
        let pipe = element(Some(25.0));
        let loads = pipe
            .equivalent_nodal_loads(
                &[
                    UniformLocalLoad::new(LocalLoadDirection::X, 3.0).unwrap(),
                    UniformLocalLoad::new(LocalLoadDirection::Y, -4.0).unwrap(),
                    UniformLocalLoad::new(LocalLoadDirection::Z, 5.0).unwrap(),
                ],
                &[],
            )
            .unwrap();

        assert_close(loads[UX], 3.0);
        assert_close(loads[DOF_PER_NODE + UX], 3.0);
        assert_close(loads[UY], -4.0);
        assert_close(loads[DOF_PER_NODE + UY], -4.0);
        assert_close(loads[RZ], -4.0 * 4.0 / 12.0);
        assert_close(loads[DOF_PER_NODE + RZ], 4.0 * 4.0 / 12.0);
        assert_close(loads[UZ], 5.0);
        assert_close(loads[DOF_PER_NODE + UZ], 5.0);
        assert_close(loads[RY], -5.0 * 4.0 / 12.0);
        assert_close(loads[DOF_PER_NODE + RY], 5.0 * 4.0 / 12.0);
    }

    #[test]
    fn global_equivalent_nodal_loads_transform_through_pipe_orientation() {
        let pipe = StraightPipeElement::new(
            "pipe-y",
            FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
            FrameNode::new(1, [0.0, 2.0, 0.0]).unwrap(),
            section(Some(25.0)),
            [1.0, 0.0, 0.0],
        )
        .unwrap();
        let loads = pipe
            .equivalent_global_nodal_loads(&[GlobalUniformLoad::new([3.0, 0.0, 0.0]).unwrap()], &[])
            .unwrap();

        assert_close(loads[UX], 3.0);
        assert_close(loads[DOF_PER_NODE + UX], 3.0);
        assert_close(loads[RZ], -1.0);
        assert_close(loads[DOF_PER_NODE + RZ], 1.0);
        assert_close(loads[UY], 0.0);
        assert_close(loads[DOF_PER_NODE + UY], 0.0);
    }

    #[test]
    fn spanned_uniform_equivalent_nodal_loads_integrate_shape_functions() {
        let pipe = element(Some(25.0));
        let loads = pipe
            .equivalent_nodal_loads_with_spans(
                &[SpannedUniformLocalLoad::new(
                    LocalLoadDirection::Y,
                    -4.0,
                    UniformLoadSpan::new(0.25, 0.75).unwrap(),
                )
                .unwrap()],
                &[],
            )
            .unwrap();

        assert_close(loads[UY], -2.0);
        assert_close(loads[RZ], -11.0 / 12.0);
        assert_close(loads[DOF_PER_NODE + UY], -2.0);
        assert_close(loads[DOF_PER_NODE + RZ], 11.0 / 12.0);
    }

    #[test]
    fn spanned_global_equivalent_nodal_loads_transform_through_pipe_orientation() {
        let pipe = StraightPipeElement::new(
            "pipe-y",
            FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
            FrameNode::new(1, [0.0, 2.0, 0.0]).unwrap(),
            section(Some(25.0)),
            [1.0, 0.0, 0.0],
        )
        .unwrap();
        let loads = pipe
            .equivalent_global_nodal_loads_with_spans(
                &[SpannedGlobalUniformLoad::new(
                    [3.0, 0.0, 0.0],
                    UniformLoadSpan::new(0.25, 0.75).unwrap(),
                )
                .unwrap()],
                &[],
            )
            .unwrap();

        assert_close(loads[UX], 1.5);
        assert_close(loads[RZ], -0.6875);
        assert_close(loads[DOF_PER_NODE + UX], 1.5);
        assert_close(loads[DOF_PER_NODE + RZ], 0.6875);
        assert_close(loads[UY], 0.0);
        assert_close(loads[DOF_PER_NODE + UY], 0.0);
    }

    #[test]
    fn axial_effect_equivalent_global_loads_follow_local_x_orientation() {
        let pipe = StraightPipeElement::new(
            "pipe-y",
            FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
            FrameNode::new(1, [0.0, 2.0, 0.0]).unwrap(),
            section(Some(25.0)),
            [1.0, 0.0, 0.0],
        )
        .unwrap();

        let loads = pipe
            .equivalent_global_axial_effect_loads(&[StraightPipeAxialEffect::new(125.0).unwrap()])
            .unwrap();

        assert_close(loads[UY], -125.0);
        assert_close(loads[DOF_PER_NODE + UY], 125.0);
        assert_close(loads[UX], 0.0);
        assert_close(loads[DOF_PER_NODE + UX], 0.0);
    }

    #[test]
    fn axial_effect_recovery_corrects_fixed_end_local_forces() {
        let pipe = element(Some(25.0));
        let effect = StraightPipeAxialEffect::new(320.0).unwrap();

        let recovered = pipe
            .recover_local_forces_with_axial_effects(&[0.0; ELEMENT_DOF], &[effect])
            .unwrap();
        let end_i = pipe
            .recover_end_resultants_with_axial_effects(&[0.0; ELEMENT_DOF], PipeEnd::I, &[effect])
            .unwrap();
        let end_j = pipe
            .recover_end_resultants_with_axial_effects(&[0.0; ELEMENT_DOF], PipeEnd::J, &[effect])
            .unwrap();

        assert_close(recovered.local_forces[UX], 320.0);
        assert_close(recovered.local_forces[DOF_PER_NODE + UX], -320.0);
        assert_close(end_i.axial_force, 320.0);
        assert_close(end_j.axial_force, -320.0);
    }

    #[test]
    fn axial_effect_station_sweep_recovers_constant_axial_resultant() {
        let pipe = element(Some(25.0));
        let effect = StraightPipeAxialEffect::new(240.0).unwrap();
        let sweep = pipe
            .recover_station_resultant_sweep_from_global_model_with_axial_effects(
                &[0.0; 2 * DOF_PER_NODE],
                &[1.0, 0.0, 0.5],
                &[effect],
            )
            .unwrap();

        assert_eq!(
            sweep
                .iter()
                .map(|station| station.station_fraction)
                .collect::<Vec<_>>(),
            vec![1.0, 0.0, 0.5]
        );
        for station in sweep {
            assert_close(station.axial_force, 240.0);
            assert_close(station.shear_force_y, 0.0);
            assert_close(station.bending_moment_z, 0.0);
        }
    }

    #[test]
    fn combined_load_and_axial_end_recovery_uses_existing_equivalent_corrections() {
        let pipe = element(Some(25.0));
        let uniform_loads = [SpannedUniformLocalLoad::new(
            LocalLoadDirection::Y,
            -4.0,
            UniformLoadSpan::new(0.25, 0.75).unwrap(),
        )
        .unwrap()];
        let point_forces = [PointLocalForce::new(0.5, LocalLoadDirection::X, 10.0).unwrap()];
        let axial_effects = [StraightPipeAxialEffect::new(240.0).unwrap()];

        let end_i = pipe
            .recover_end_resultants_with_spans_and_axial_effects(
                &[0.0; ELEMENT_DOF],
                PipeEnd::I,
                &uniform_loads,
                &point_forces,
                &axial_effects,
            )
            .unwrap();
        let end_j = pipe
            .recover_end_resultants_with_spans_and_axial_effects(
                &[0.0; ELEMENT_DOF],
                PipeEnd::J,
                &uniform_loads,
                &point_forces,
                &axial_effects,
            )
            .unwrap();

        assert_close(end_i.axial_force, 235.0);
        assert_close(end_i.shear_force_y, 2.0);
        assert_close(end_i.bending_moment_z, 11.0 / 12.0);
        assert_close(end_j.axial_force, -245.0);
        assert_close(end_j.shear_force_y, 2.0);
        assert_close(end_j.bending_moment_z, -11.0 / 12.0);
    }

    #[test]
    fn combined_load_and_axial_station_sweep_preserves_order_and_accumulates_loads() {
        let pipe = element(Some(25.0));
        let uniform_loads = [SpannedUniformLocalLoad::new(
            LocalLoadDirection::Y,
            -4.0,
            UniformLoadSpan::new(0.25, 0.75).unwrap(),
        )
        .unwrap()];
        let point_forces = [PointLocalForce::new(0.5, LocalLoadDirection::X, 10.0).unwrap()];
        let axial_effects = [StraightPipeAxialEffect::new(240.0).unwrap()];

        let sweep = pipe
            .recover_station_resultant_sweep_from_global_model_with_spans_and_axial_effects(
                &[0.0; 2 * DOF_PER_NODE],
                &[0.75, 0.25],
                &uniform_loads,
                &point_forces,
                &axial_effects,
            )
            .unwrap();

        assert_eq!(
            sweep
                .iter()
                .map(|station| station.station_fraction)
                .collect::<Vec<_>>(),
            vec![0.75, 0.25]
        );
        assert_close(sweep[0].axial_force, 245.0);
        assert_close(sweep[0].shear_force_y, -2.0);
        assert_close(sweep[0].bending_moment_z, -1.0 / 12.0);
        assert_close(sweep[1].axial_force, 235.0);
        assert_close(sweep[1].shear_force_y, 2.0);
        assert_close(sweep[1].bending_moment_z, -1.0 / 12.0);
    }

    #[test]
    fn axial_effect_rejects_nonfinite_force() {
        let pipe = element(Some(25.0));
        let effect = StraightPipeAxialEffect {
            axial_force: f64::NAN,
        };

        let error = pipe
            .equivalent_local_axial_effect_loads(&[effect])
            .unwrap_err();

        assert!(matches!(
            error,
            StraightPipeError::NonFiniteInput {
                name: "axial_force",
                value
            } if value.is_nan()
        ));
    }

    #[test]
    fn interior_point_force_equivalent_loads_are_deterministic() {
        let pipe = element(Some(25.0));
        let loads = pipe
            .equivalent_nodal_loads(
                &[],
                &[PointLocalForce::new(0.25, LocalLoadDirection::Y, 8.0).unwrap()],
            )
            .unwrap();

        assert_close(loads[UY], 6.75);
        assert_close(loads[RZ], 2.25);
        assert_close(loads[DOF_PER_NODE + UY], 1.25);
        assert_close(loads[DOF_PER_NODE + RZ], -0.75);
    }

    #[test]
    fn global_point_force_equivalent_loads_transform_through_pipe_orientation() {
        let pipe = StraightPipeElement::new(
            "pipe-y",
            FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
            FrameNode::new(1, [0.0, 2.0, 0.0]).unwrap(),
            section(Some(25.0)),
            [1.0, 0.0, 0.0],
        )
        .unwrap();
        let loads = pipe
            .equivalent_global_nodal_loads(
                &[],
                &[GlobalPointForce::new(0.25, [8.0, 0.0, 0.0]).unwrap()],
            )
            .unwrap();

        assert_close(loads[UX], 6.75);
        assert_close(loads[RZ], -2.25);
        assert_close(loads[DOF_PER_NODE + UX], 1.25);
        assert_close(loads[DOF_PER_NODE + RZ], 0.75);
        assert_close(loads[UY], 0.0);
        assert_close(loads[DOF_PER_NODE + UY], 0.0);
    }

    #[test]
    fn station_resultants_accumulate_from_i_end_loads() {
        let pipe = element(Some(25.0));
        let i_end = PipeEndResultants {
            end: PipeEnd::I,
            axial_force: 10.0,
            shear_force_y: 3.0,
            shear_force_z: -2.0,
            torsional_moment: 7.0,
            bending_moment_y: 5.0,
            bending_moment_z: -6.0,
        };

        let station = pipe
            .station_resultants_from_i_end(
                i_end,
                0.75,
                &[UniformLocalLoad::new(LocalLoadDirection::Y, 4.0).unwrap()],
                &[PointLocalForce::new(0.25, LocalLoadDirection::Z, -8.0).unwrap()],
            )
            .unwrap();

        assert_close(station.distance_from_i, 1.5);
        assert_close(station.axial_force, 10.0);
        assert_close(station.shear_force_y, 9.0);
        assert_close(station.shear_force_z, -10.0);
        assert_close(station.torsional_moment, 7.0);
        assert_close(station.bending_moment_z, -15.0);
        assert_close(station.bending_moment_y, -6.0);
    }

    #[test]
    fn station_resultants_accumulate_spanned_uniform_loads_only_to_station() {
        let pipe = element(Some(25.0));
        let station = pipe
            .station_resultants_from_i_end_with_spans(
                PipeEndResultants {
                    end: PipeEnd::I,
                    axial_force: 0.0,
                    shear_force_y: 0.0,
                    shear_force_z: 0.0,
                    torsional_moment: 0.0,
                    bending_moment_y: 0.0,
                    bending_moment_z: 0.0,
                },
                0.5,
                &[SpannedUniformLocalLoad::new(
                    LocalLoadDirection::Y,
                    -4.0,
                    UniformLoadSpan::new(0.25, 0.75).unwrap(),
                )
                .unwrap()],
                &[],
            )
            .unwrap();

        assert_close(station.shear_force_y, -2.0);
        assert_close(station.bending_moment_z, 0.5);
    }

    #[test]
    fn station_resultant_sweep_preserves_requested_order_for_spanned_loads() {
        let pipe = element(Some(25.0));
        let sweep = pipe
            .station_resultant_sweep_from_i_end_with_spans(
                PipeEndResultants {
                    end: PipeEnd::I,
                    axial_force: 0.0,
                    shear_force_y: 4.0,
                    shear_force_z: 0.0,
                    torsional_moment: 0.0,
                    bending_moment_y: 0.0,
                    bending_moment_z: 4.0,
                },
                &[0.75, 0.25, 0.5, 1.0],
                &[SpannedUniformLocalLoad::new(
                    LocalLoadDirection::Y,
                    -4.0,
                    UniformLoadSpan::new(0.25, 0.75).unwrap(),
                )
                .unwrap()],
                &[],
            )
            .unwrap();

        assert_eq!(
            sweep
                .iter()
                .map(|station| station.station_fraction)
                .collect::<Vec<_>>(),
            vec![0.75, 0.25, 0.5, 1.0]
        );
        assert_close(sweep[0].shear_force_y, 0.0);
        assert_close(sweep[0].bending_moment_z, 0.0);
        assert_close(sweep[1].shear_force_y, 4.0);
        assert_close(sweep[1].bending_moment_z, 2.0);
        assert_close(sweep[2].shear_force_y, 2.0);
        assert_close(sweep[2].bending_moment_z, 0.5);
        assert_close(sweep[3].shear_force_y, 0.0);
        assert_close(sweep[3].bending_moment_z, 0.0);
    }

    #[test]
    fn station_resultant_sweep_rejects_invalid_station_values() {
        let pipe = element(Some(25.0));
        let error = pipe
            .recover_station_resultant_sweep(&[0.0; ELEMENT_DOF], &[0.0, 1.1], &[], &[])
            .unwrap_err();

        assert_eq!(
            error,
            StraightPipeError::InvalidStationFraction {
                name: "station_fraction",
                value: 1.1
            }
        );
    }

    #[test]
    fn load_and_station_inputs_reject_invalid_values() {
        let pipe = element(Some(25.0));
        let error = pipe
            .station_resultants_from_i_end(
                PipeEndResultants {
                    end: PipeEnd::I,
                    axial_force: 0.0,
                    shear_force_y: 0.0,
                    shear_force_z: 0.0,
                    torsional_moment: 0.0,
                    bending_moment_y: 0.0,
                    bending_moment_z: 0.0,
                },
                1.2,
                &[],
                &[],
            )
            .unwrap_err();
        assert_eq!(
            error,
            StraightPipeError::InvalidStationFraction {
                name: "station_fraction",
                value: 1.2
            }
        );

        assert!(matches!(
            UniformLocalLoad::new(LocalLoadDirection::X, f64::NAN).unwrap_err(),
            StraightPipeError::NonFiniteInput {
                name: "force_per_length",
                value
            } if value.is_nan()
        ));
        assert_eq!(
            UniformLoadSpan::new(0.75, 0.25).unwrap_err(),
            StraightPipeError::InvalidLoadSpan {
                start_fraction: 0.75,
                end_fraction: 0.25
            }
        );
    }

    #[test]
    fn recovers_local_bending_shear_from_global_model_displacements() {
        let pipe = element(Some(25.0));
        let transverse_displacement = 0.002;
        let mut model_displacements = vec![0.0; 2 * DOF_PER_NODE];
        model_displacements[DOF_PER_NODE + UY] = transverse_displacement;

        let recovered = pipe
            .recover_local_forces_from_global_model(&model_displacements)
            .unwrap();

        assert!(recovered.local_forces[UY].abs() > 0.0);
        assert!(recovered.local_forces[DOF_PER_NODE + UY].abs() > 0.0);
        assert!(
            (recovered.local_forces[UY] + recovered.local_forces[DOF_PER_NODE + UY]).abs() < 1.0e-6
        );
    }

    #[test]
    fn recovery_extracts_noncontiguous_global_model_dofs() {
        let pipe = StraightPipeElement::new(
            "pipe-noncontiguous",
            FrameNode::new(1, [0.0, 0.0, 0.0]).unwrap(),
            FrameNode::new(3, [2.0, 0.0, 0.0]).unwrap(),
            section(Some(25.0)),
            [0.0, 1.0, 0.0],
        )
        .unwrap();
        let axial_extension = 0.001;
        let mut model_displacements = vec![0.0; 4 * DOF_PER_NODE];
        model_displacements[3 * DOF_PER_NODE + UX] = axial_extension;

        let recovered = pipe
            .recover_local_forces_from_global_model(&model_displacements)
            .unwrap();
        let expected_axial = pipe.section.elastic_modulus * pipe.section.area
            / pipe.length().unwrap()
            * axial_extension;

        assert_close(
            recovered.local_displacements[DOF_PER_NODE + UX],
            axial_extension,
        );
        assert_close(recovered.local_forces[UX], -expected_axial);
        assert_close(recovered.local_forces[DOF_PER_NODE + UX], expected_axial);
    }

    #[test]
    fn recovery_rejects_invalid_displacement_length() {
        let pipe = element(Some(25.0));

        let error = pipe.recover_local_forces(&[0.0; 3]).unwrap_err();

        assert_eq!(
            error,
            StraightPipeError::InvalidDisplacementLength {
                expected: ELEMENT_DOF,
                actual: 3
            }
        );
    }

    #[test]
    fn recovery_rejects_nonfinite_displacements() {
        let pipe = element(Some(25.0));
        let mut displacements = [0.0; ELEMENT_DOF];
        displacements[UX] = f64::NAN;

        let error = pipe.recover_local_forces(&displacements).unwrap_err();

        assert!(matches!(
            error,
            StraightPipeError::NonFiniteInput {
                name: "global_element_displacements",
                value
            } if value.is_nan()
        ));
    }

    #[test]
    fn global_model_recovery_reports_required_model_length() {
        let pipe = StraightPipeElement::new(
            "pipe-noncontiguous",
            FrameNode::new(1, [0.0, 0.0, 0.0]).unwrap(),
            FrameNode::new(3, [2.0, 0.0, 0.0]).unwrap(),
            section(Some(25.0)),
            [0.0, 1.0, 0.0],
        )
        .unwrap();

        let error = pipe
            .recover_local_forces_from_global_model(&vec![0.0; 4 * DOF_PER_NODE - 1])
            .unwrap_err();

        assert_eq!(
            error,
            StraightPipeError::InvalidDisplacementLength {
                expected: 4 * DOF_PER_NODE,
                actual: 4 * DOF_PER_NODE - 1
            }
        );
    }

    #[test]
    fn straight_pipe_boundary_metadata_carries_units_and_model_refs() {
        let frame_units = FrameKernelUnitBasis::from_unit_ids(
            "fixture-unit-system",
            "fixture-length",
            "fixture-force",
            "fixture-moment",
            "fixture-stress",
            "fixture-area",
            "fixture-second-moment-area",
            "fixture-displacement",
            "fixture-rotation",
        )
        .unwrap();
        let metadata = StraightPipeBoundaryMetadata::new(
            frame_units,
            QuantityUnitMetadata::new("fixture-mass-per-length", CanonicalDimension::MassPerLength)
                .unwrap(),
            QuantityUnitMetadata::new("fixture-acceleration", CanonicalDimension::Acceleration)
                .unwrap(),
            QuantityUnitMetadata::new("fixture-force-per-length", CanonicalDimension::Tbd).unwrap(),
            CanonicalModelReference::new(
                "analytical-model",
                open_pipe_stress_frame_kernel::CanonicalModelRole::AnalyticalSolverModel,
                "element:pipe-1",
            )
            .unwrap(),
            CanonicalModelReference::new(
                "physical-model",
                open_pipe_stress_frame_kernel::CanonicalModelRole::PhysicalSourceOfTruth,
                "pipe:pipe-1",
            )
            .unwrap(),
        )
        .unwrap();

        assert!(metadata.has_expected_dimensions());
        assert_eq!(
            metadata.unit_system_ref().unit_system_id,
            "fixture-unit-system"
        );
        assert_eq!(
            metadata.mass_per_length_unit.dimension_id(),
            "mass_per_length"
        );
        assert_eq!(metadata.weight_force_per_length_unit.dimension_id(), "TBD");
        assert_eq!(
            metadata.source_model_ref.model_role.as_str(),
            "physical_source_of_truth"
        );
    }
}
