//! Straight pipe element mechanics.
//!
//! This crate adapts explicit straight-pipe properties into the frame-kernel
//! solver boundary. It does not provide pipe tables, material defaults,
//! code-specific checks, protected standards data, or engineering approval.

use open_pipe_stress_frame_kernel::exact_sum::{ExactAccumulator, SumError};
use open_pipe_stress_frame_kernel::load_ledger::{gamma, product_upward, round_upward, Formation};
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
            && self.weight_force_per_length_unit.dimension == CanonicalDimension::ForcePerLength
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

    /// E1 (S11): each DOF's fixed-end load is one exact sum of every load's
    /// own fixed-end term, rounded once. For one load it equals that load's
    /// term bit for bit.
    pub fn equivalent_nodal_loads_with_spans(
        &self,
        uniform_loads: &[SpannedUniformLocalLoad],
        point_forces: &[PointLocalForce],
    ) -> Result<[f64; ELEMENT_DOF], StraightPipeError> {
        let terms = self.equivalent_nodal_load_terms_with_spans(uniform_loads, point_forces)?;
        exact_array_sum("equivalent_nodal_loads", None, &terms, &[])
    }

    /// One fixed-end term array per load: uniform loads first, then point
    /// forces, in the given order. Each array is formed from its own load only.
    pub fn equivalent_nodal_load_terms_with_spans(
        &self,
        uniform_loads: &[SpannedUniformLocalLoad],
        point_forces: &[PointLocalForce],
    ) -> Result<Vec<[f64; ELEMENT_DOF]>, StraightPipeError> {
        let length = self.length()?;
        let mut terms = Vec::with_capacity(uniform_loads.len() + point_forces.len());
        for load in uniform_loads {
            terms.push(spanned_uniform_equivalent_terms(length, *load)?);
        }
        for load in point_forces {
            terms.push(point_equivalent_terms(length, *load)?);
        }
        Ok(terms)
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

    /// S11-G (`S11G_GUARD.md` section 3.2): today's values of
    /// `equivalent_global_nodal_loads_with_spans` (called unchanged, so the
    /// values are bit-identical) plus, per global slot, the formation record
    /// of the formed value: `Formation::Exact` with the exact scaled intended
    /// formula `scale * T^T FEM(T q_g)` on the held operands q_g, L, (a, b)
    /// and T (scale 3 on rotation slots clears the 1/3 and 2/3 of the
    /// rotation terms; 1 on translation slots). If an exact step leaves the
    /// binary64 range, every slot falls back to `Formation::Bounded` with
    /// gamma_16 * sum|monomials| plus an absolute 64 * 2^-1074 (rounded
    /// upward); a bound that cannot be represented is `+inf`, which fires.
    pub fn equivalent_global_nodal_loads_with_spans_formed(
        &self,
        uniform_loads: &[SpannedGlobalUniformLoad],
    ) -> Result<([f64; ELEMENT_DOF], Vec<Formation>), StraightPipeError> {
        let values = self.equivalent_global_nodal_loads_with_spans(uniform_loads, &[])?;
        let orientation = self.frame_element()?.orientation()?;
        let length = self.length()?;
        let formations = match exact_scaled_intended(&orientation, length, uniform_loads) {
            Some(rows) => rows
                .into_iter()
                .enumerate()
                .map(|(slot, scaled_intended)| Formation::Exact {
                    scale: slot_scale(slot),
                    scaled_intended,
                })
                .collect(),
            None => monomial_bounds(&orientation, length, uniform_loads)
                .into_iter()
                .map(|bound| Formation::Bounded { bound })
                .collect(),
        };
        Ok((values, formations))
    }

    /// S11-G R-b' (`S11G_GUARD.md` section 4): for each end (i, j), the
    /// formation-noise bound of the formed K_e * u bending rows,
    /// gamma_16 * (sum_k |K[ry][k]| * sum_c |T_kc| |u_c| + the same for rz),
    /// with the product's own K_loc, T and binary64 u. Each inner sum is
    /// exact and rounded upward, and so is the final product.
    pub fn bending_formation_bound(
        &self,
        global_model_displacements: &[f64],
    ) -> Result<[f64; 2], StraightPipeError> {
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
        let frame_element = self.frame_element()?;
        let transform = frame_element.orientation()?.transformation_matrix();
        let stiffness = frame_element.local_stiffness()?;
        let bound_error = |value: f64| StraightPipeError::NonFiniteInput {
            name: "bending_formation_bound",
            value,
        };
        let mut local_magnitudes = [0.0; ELEMENT_DOF];
        for (k, magnitude) in local_magnitudes.iter_mut().enumerate() {
            let mut accumulator = ExactAccumulator::new();
            for (c, displacement) in element_displacements.iter().enumerate() {
                accumulator
                    .add_product(transform[k][c].abs(), displacement.abs())
                    .map_err(|_| bound_error(*displacement))?;
            }
            *magnitude = round_upward(&accumulator).map_err(|_| bound_error(f64::INFINITY))?;
        }
        let mut bounds = [0.0; 2];
        for (bound, (ry, rz)) in bounds
            .iter_mut()
            .zip([(RY, RZ), (DOF_PER_NODE + RY, DOF_PER_NODE + RZ)])
        {
            let mut accumulator = ExactAccumulator::new();
            for (k, magnitude) in local_magnitudes.iter().enumerate() {
                accumulator
                    .add_product(stiffness[ry][k].abs(), *magnitude)
                    .map_err(|_| bound_error(*magnitude))?;
                accumulator
                    .add_product(stiffness[rz][k].abs(), *magnitude)
                    .map_err(|_| bound_error(*magnitude))?;
            }
            let row_sum = round_upward(&accumulator).map_err(|_| bound_error(f64::INFINITY))?;
            *bound = product_upward(gamma(16), row_sum);
        }
        Ok(bounds)
    }

    /// E2 (S11): each axial effect's pair, summed exactly and rounded once.
    pub fn equivalent_local_axial_effect_loads(
        &self,
        axial_effects: &[StraightPipeAxialEffect],
    ) -> Result<[f64; ELEMENT_DOF], StraightPipeError> {
        let terms = local_axial_effect_terms(axial_effects)?;
        exact_array_sum("equivalent_local_axial_effect_loads", None, &terms, &[])
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
        let local_forces = self.exact_loaded_local_forces(
            "axial_effect_corrected_local_forces",
            &recovered.local_forces,
            &[],
            &[],
            axial_effects,
        )?;
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
        let local_forces = self.exact_loaded_local_forces(
            "axial_effect_corrected_local_forces",
            &recovered.local_forces,
            &[],
            &[],
            axial_effects,
        )?;
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

        // E4/E6 (S11): each component is one exact sum of the i-end action,
        // fl(V*d) and every load's station term, rounded once.
        let mut sums = StationSums::default();
        sums.axial.push(i_end.axial_force);
        sums.shear_y.push(i_end.shear_force_y);
        sums.shear_z.push(i_end.shear_force_z);
        sums.moment_y.push(i_end.bending_moment_y);
        sums.moment_y.push(i_end.shear_force_z * distance);
        sums.moment_z.push(i_end.bending_moment_z);
        sums.moment_z.push(-(i_end.shear_force_y * distance));

        for load in uniform_loads {
            validate_spanned_uniform_load(*load)?;
            spanned_uniform_station_terms(&mut sums, *load, length, distance);
        }
        for load in point_forces {
            validate_station_fraction("point_force_station_fraction", load.station_fraction)?;
            validate_finite("point_force", load.force)?;
            point_station_terms(&mut sums, *load, length, distance);
        }

        let resultants = StationResultants {
            station_fraction,
            distance_from_i: distance,
            axial_force: exact_terms_sum("axial_force", &sums.axial)?,
            shear_force_y: exact_terms_sum("shear_force_y", &sums.shear_y)?,
            shear_force_z: exact_terms_sum("shear_force_z", &sums.shear_z)?,
            torsional_moment: i_end.torsional_moment,
            bending_moment_y: exact_terms_sum("bending_moment_y", &sums.moment_y)?,
            bending_moment_z: exact_terms_sum("bending_moment_z", &sums.moment_z)?,
        };
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
        let loaded_forces = self.exact_loaded_local_forces(
            "loaded_local_forces",
            &recovered.local_forces,
            uniform_loads,
            point_forces,
            &[],
        )?;
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
        let loaded_forces = self.exact_loaded_local_forces(
            "loaded_local_forces",
            &recovered.local_forces,
            uniform_loads,
            point_forces,
            &[],
        )?;
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
        let loaded_forces = self.exact_loaded_local_forces(
            "loaded_local_forces",
            &recovered.local_forces,
            uniform_loads,
            point_forces,
            &[],
        )?;
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
        let loaded_forces = self.exact_loaded_local_forces(
            "loaded_local_forces",
            &recovered.local_forces,
            uniform_loads,
            point_forces,
            &[],
        )?;
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
        self.exact_loaded_local_forces(
            "load_and_axial_corrected_local_forces",
            local_forces,
            uniform_loads,
            point_forces,
            axial_effects,
        )
    }

    /// E3 (S11): `local_i - each load term - each axial term`, one exact sum
    /// per DOF, rounded once (it replaces two roundings).
    fn exact_loaded_local_forces(
        &self,
        name: &'static str,
        local_forces: &[f64; ELEMENT_DOF],
        uniform_loads: &[SpannedUniformLocalLoad],
        point_forces: &[PointLocalForce],
        axial_effects: &[StraightPipeAxialEffect],
    ) -> Result<[f64; ELEMENT_DOF], StraightPipeError> {
        let load_terms =
            self.equivalent_nodal_load_terms_with_spans(uniform_loads, point_forces)?;
        let axial_terms = local_axial_effect_terms(axial_effects)?;
        exact_array_sum(name, Some(local_forces), &load_terms, &axial_terms)
    }
}

/// One exact sum per DOF: `base - sum(first) - sum(second)` when `base` is
/// given, otherwise `sum(first) + sum(second)`; rounded once (+0.0 for zero).
fn exact_array_sum(
    name: &'static str,
    base: Option<&[f64; ELEMENT_DOF]>,
    first: &[[f64; ELEMENT_DOF]],
    second: &[[f64; ELEMENT_DOF]],
) -> Result<[f64; ELEMENT_DOF], StraightPipeError> {
    let negate = base.is_some();
    let mut result = [0.0; ELEMENT_DOF];
    for (dof, value) in result.iter_mut().enumerate() {
        let mut accumulator = ExactAccumulator::new();
        let mut add = |x: f64| {
            accumulator
                .add(x)
                .map_err(|error| sum_error(name, error, x))
        };
        if let Some(base) = base {
            add(base[dof])?;
        }
        for terms in first.iter().chain(second) {
            add(if negate { -terms[dof] } else { terms[dof] })?;
        }
        *value = rounded(name, &accumulator)?;
    }
    Ok(result)
}

fn sum_error(name: &'static str, error: SumError, value: f64) -> StraightPipeError {
    StraightPipeError::NonFiniteInput {
        name,
        value: match error {
            SumError::NonFinite => value,
            _ => f64::INFINITY,
        },
    }
}

/// Rounds once; an out-of-range net maps to today's non-finite error path.
fn rounded(name: &'static str, accumulator: &ExactAccumulator) -> Result<f64, StraightPipeError> {
    accumulator
        .round()
        .map_err(|_| StraightPipeError::NonFiniteInput {
            name,
            value: if accumulator.signum() < 0 {
                f64::NEG_INFINITY
            } else {
                f64::INFINITY
            },
        })
}

/// Per-effect axial pairs: -N at i, +N at j (formation, one source each).
fn local_axial_effect_terms(
    axial_effects: &[StraightPipeAxialEffect],
) -> Result<Vec<[f64; ELEMENT_DOF]>, StraightPipeError> {
    let mut terms = Vec::with_capacity(axial_effects.len());
    for effect in axial_effects {
        validate_axial_effect(*effect)?;
        let mut pair = [0.0; ELEMENT_DOF];
        pair[UX] = -effect.axial_force;
        pair[DOF_PER_NODE + UX] = effect.axial_force;
        terms.push(pair);
    }
    Ok(terms)
}

/// One uniform load's own fixed-end terms (formation from one source).
fn spanned_uniform_equivalent_terms(
    length: f64,
    load: SpannedUniformLocalLoad,
) -> Result<[f64; ELEMENT_DOF], StraightPipeError> {
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

    let mut terms = [0.0; ELEMENT_DOF];
    match load.direction {
        LocalLoadDirection::X => {
            terms[UX] = axial_i;
            terms[DOF_PER_NODE + UX] = axial_j;
        }
        LocalLoadDirection::Y => {
            terms[UY] = transverse_i;
            terms[RZ] = rotation_i;
            terms[DOF_PER_NODE + UY] = transverse_j;
            terms[DOF_PER_NODE + RZ] = rotation_j;
        }
        LocalLoadDirection::Z => {
            terms[UZ] = transverse_i;
            terms[RY] = -rotation_i;
            terms[DOF_PER_NODE + UZ] = transverse_j;
            terms[DOF_PER_NODE + RY] = -rotation_j;
        }
    }
    Ok(terms)
}

/// One point force's own fixed-end terms (formation from one source).
fn point_equivalent_terms(
    length: f64,
    load: PointLocalForce,
) -> Result<[f64; ELEMENT_DOF], StraightPipeError> {
    validate_station_fraction("point_force_station_fraction", load.station_fraction)?;
    validate_finite("point_force", load.force)?;
    let r = load.station_fraction;
    let n_i = 1.0 - r;
    let n_j = r;
    let h_i = 1.0 - 3.0 * r * r + 2.0 * r * r * r;
    let theta_i = length * (r - 2.0 * r * r + r * r * r);
    let h_j = 3.0 * r * r - 2.0 * r * r * r;
    let theta_j = length * (-r * r + r * r * r);

    let mut terms = [0.0; ELEMENT_DOF];
    match load.direction {
        LocalLoadDirection::X => {
            terms[UX] = load.force * n_i;
            terms[DOF_PER_NODE + UX] = load.force * n_j;
        }
        LocalLoadDirection::Y => {
            terms[UY] = load.force * h_i;
            terms[RZ] = load.force * theta_i;
            terms[DOF_PER_NODE + UY] = load.force * h_j;
            terms[DOF_PER_NODE + RZ] = load.force * theta_j;
        }
        LocalLoadDirection::Z => {
            terms[UZ] = load.force * h_i;
            terms[RY] = -(load.force * theta_i);
            terms[DOF_PER_NODE + UZ] = load.force * h_j;
            terms[DOF_PER_NODE + RY] = -(load.force * theta_j);
        }
    }
    Ok(terms)
}

/// Station terms per component, summed exactly by `exact_terms_sum`.
#[derive(Default)]
struct StationSums {
    axial: Vec<f64>,
    shear_y: Vec<f64>,
    shear_z: Vec<f64>,
    moment_y: Vec<f64>,
    moment_z: Vec<f64>,
}

fn exact_terms_sum(name: &'static str, terms: &[f64]) -> Result<f64, StraightPipeError> {
    let mut accumulator = ExactAccumulator::new();
    for &term in terms {
        accumulator
            .add(term)
            .map_err(|error| sum_error(name, error, term))?;
    }
    rounded(name, &accumulator)
}

fn spanned_uniform_station_terms(
    sums: &mut StationSums,
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
            sums.axial.push(load.force_per_length * active_length);
        }
        LocalLoadDirection::Y => {
            sums.shear_y.push(load.force_per_length * active_length);
            sums.moment_z
                .push(-(load.force_per_length * lever_integral));
        }
        LocalLoadDirection::Z => {
            sums.shear_z.push(load.force_per_length * active_length);
            sums.moment_y.push(load.force_per_length * lever_integral);
        }
    }
}

fn point_station_terms(sums: &mut StationSums, load: PointLocalForce, length: f64, distance: f64) {
    let load_distance = load.station_fraction * length;
    if load_distance > distance {
        return;
    }
    let lever = distance - load_distance;
    match load.direction {
        LocalLoadDirection::X => {
            sums.axial.push(load.force);
        }
        LocalLoadDirection::Y => {
            sums.shear_y.push(load.force);
            sums.moment_z.push(-(load.force * lever));
        }
        LocalLoadDirection::Z => {
            sums.shear_z.push(load.force);
            sums.moment_y.push(load.force * lever);
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

// ------------------------------------------------ S11-G exact intended formula

/// Local slot of each uniform fixed-end polynomial, per local direction:
/// (slot, polynomial, sign). Directions: 0 = X, 1 = Y, 2 = Z.
const UNIFORM_SLOTS: [&[(usize, usize, f64)]; 3] = [
    &[(UX, 0, 1.0), (DOF_PER_NODE + UX, 1, 1.0)],
    &[
        (UY, 2, 1.0),
        (RZ, 4, 1.0),
        (DOF_PER_NODE + UY, 3, 1.0),
        (DOF_PER_NODE + RZ, 5, 1.0),
    ],
    &[
        (UZ, 2, 1.0),
        (RY, 4, -1.0),
        (DOF_PER_NODE + UZ, 3, 1.0),
        (DOF_PER_NODE + RY, 5, -1.0),
    ],
];

/// The six uniform fixed-end polynomials of `spanned_uniform_equivalent_terms`
/// as (coefficient, variable 0 = b or 1 = a, power), with the power of L; the
/// two rotation polynomials are scaled by 3 (exact coefficients).
const UNIFORM_POLYNOMIALS: [(u32, &[(f64, usize, i32)]); 6] = [
    // axial_i = q L ((b - b^2/2) - (a - a^2/2))
    (1, &[(1.0, 0, 1), (-0.5, 0, 2), (-1.0, 1, 1), (0.5, 1, 2)]),
    // axial_j = q L (b^2/2 - a^2/2)
    (1, &[(0.5, 0, 2), (-0.5, 1, 2)]),
    // transverse_i = q L ((b - b^3 + b^4/2) - (a - a^3 + a^4/2))
    (
        1,
        &[
            (1.0, 0, 1),
            (-1.0, 0, 3),
            (0.5, 0, 4),
            (-1.0, 1, 1),
            (1.0, 1, 3),
            (-0.5, 1, 4),
        ],
    ),
    // transverse_j = q L ((b^3 - b^4/2) - (a^3 - a^4/2))
    (1, &[(1.0, 0, 3), (-0.5, 0, 4), (-1.0, 1, 3), (0.5, 1, 4)]),
    // 3 rotation_i = q L^2 ((3b^2/2 - 2b^3 + 3b^4/4) - (same in a))
    (
        2,
        &[
            (1.5, 0, 2),
            (-2.0, 0, 3),
            (0.75, 0, 4),
            (-1.5, 1, 2),
            (2.0, 1, 3),
            (-0.75, 1, 4),
        ],
    ),
    // 3 rotation_j = q L^2 ((-b^3 + 3b^4/4) - (same in a))
    (2, &[(-1.0, 0, 3), (0.75, 0, 4), (1.0, 1, 3), (-0.75, 1, 4)]),
];

fn slot_scale(slot: usize) -> f64 {
    if slot % DOF_PER_NODE >= RX {
        3.0
    } else {
        1.0
    }
}

/// Binary64 components whose exact sum is the accumulator's exact value, by
/// repeated round-and-subtract. `None` when a remainder lies below the
/// binary64 range or the value lies above it.
fn expansion_components(mut rest: ExactAccumulator) -> Option<Vec<f64>> {
    let mut components = Vec::new();
    for _ in 0..80 {
        if rest.is_zero() {
            return Some(components);
        }
        let head = rest.round().ok()?;
        if head == 0.0 {
            return None;
        }
        rest.add(-head).ok()?;
        components.push(head);
    }
    None
}

/// The exact product of two expansions, as components.
fn expansion_product(left: &[f64], right: &[f64]) -> Option<Vec<f64>> {
    let mut accumulator = ExactAccumulator::new();
    for &x in left {
        for &y in right {
            accumulator.add_product(x, y).ok()?;
        }
    }
    expansion_components(accumulator)
}

/// Components of scale * formula for each global slot; `None` on a range
/// failure.
fn exact_scaled_intended(
    orientation: &FrameOrientation,
    length: f64,
    uniform_loads: &[SpannedGlobalUniformLoad],
) -> Option<Vec<Vec<f64>>> {
    let mut slot_accumulators: Vec<ExactAccumulator> =
        (0..ELEMENT_DOF).map(|_| ExactAccumulator::new()).collect();
    for load in uniform_loads {
        let variables = [load.span.end_fraction, load.span.start_fraction];
        // x^p as expansions, p = 1..4, for b and a.
        let powers = variables
            .iter()
            .map(|&variable| {
                let mut table = vec![vec![variable]];
                for p in 1..4 {
                    let next = expansion_product(&table[p - 1], &[variable])?;
                    table.push(next);
                }
                Some(table)
            })
            .collect::<Option<Vec<_>>>()?;
        let mut polynomials = Vec::with_capacity(UNIFORM_POLYNOMIALS.len());
        for (_, monomials) in UNIFORM_POLYNOMIALS {
            let mut accumulator = ExactAccumulator::new();
            for &(coefficient, variable, power) in monomials {
                for &component in &powers[variable][(power - 1) as usize] {
                    accumulator.add_product(coefficient, component).ok()?;
                }
            }
            polynomials.push(expansion_components(accumulator)?);
        }
        for (direction, slots) in UNIFORM_SLOTS.iter().enumerate() {
            // The exact local intensity T[d] . q_g.
            let mut accumulator = ExactAccumulator::new();
            for (axis, &intensity) in load.force_per_length.iter().enumerate() {
                accumulator
                    .add_product(orientation.local_axes[direction][axis], intensity)
                    .ok()?;
            }
            let local = expansion_components(accumulator)?;
            for &(slot, polynomial, sign) in *slots {
                let mut term = expansion_product(&local, &polynomials[polynomial])?;
                for _ in 0..UNIFORM_POLYNOMIALS[polynomial].0 {
                    term = expansion_product(&term, &[length])?;
                }
                for component in term {
                    slot_accumulators[slot].add(sign * component).ok()?;
                }
            }
        }
    }
    let mut local_slots = Vec::with_capacity(ELEMENT_DOF);
    for accumulator in slot_accumulators {
        local_slots.push(expansion_components(accumulator)?);
    }
    let transform = orientation.transformation_matrix();
    let mut rows = Vec::with_capacity(ELEMENT_DOF);
    for row in 0..ELEMENT_DOF {
        let mut accumulator = ExactAccumulator::new();
        for (col, components) in local_slots.iter().enumerate() {
            for &component in components {
                accumulator
                    .add_product(transform[col][row], component)
                    .ok()?;
            }
        }
        rows.push(expansion_components(accumulator)?);
    }
    Some(rows)
}

/// The range fallback: per global slot, gamma_16 * sum|monomials| + an
/// absolute 64 * 2^-1074, rounded upward (`+inf` when not representable).
fn monomial_bounds(
    orientation: &FrameOrientation,
    length: f64,
    uniform_loads: &[SpannedGlobalUniformLoad],
) -> Vec<f64> {
    let mut slot_magnitudes: Vec<ExactAccumulator> =
        (0..ELEMENT_DOF).map(|_| ExactAccumulator::new()).collect();
    let mut failed = false;
    for load in uniform_loads {
        let variables = [load.span.end_fraction.abs(), load.span.start_fraction.abs()];
        for (direction, slots) in UNIFORM_SLOTS.iter().enumerate() {
            let mut intensity_magnitude = ExactAccumulator::new();
            for (axis, &intensity) in load.force_per_length.iter().enumerate() {
                if intensity_magnitude
                    .add_product(
                        orientation.local_axes[direction][axis].abs(),
                        intensity.abs(),
                    )
                    .is_err()
                {
                    failed = true;
                }
            }
            let local = round_upward(&intensity_magnitude).unwrap_or(f64::INFINITY);
            for &(slot, polynomial, _) in *slots {
                let (length_power, monomials) = UNIFORM_POLYNOMIALS[polynomial];
                let mut scale = local;
                for _ in 0..length_power {
                    scale = product_upward(scale, length.abs());
                }
                for &(coefficient, variable, power) in monomials {
                    let mut magnitude = product_upward(scale, coefficient.abs());
                    for _ in 0..power {
                        magnitude = product_upward(magnitude, variables[variable]);
                    }
                    if slot_magnitudes[slot].add(magnitude).is_err() {
                        failed = true;
                    }
                }
            }
        }
    }
    let transform = orientation.transformation_matrix();
    let tiny = f64::from_bits(1);
    let local: Vec<f64> = slot_magnitudes
        .iter()
        .map(|accumulator| round_upward(accumulator).unwrap_or(f64::INFINITY))
        .collect();
    (0..ELEMENT_DOF)
        .map(|row| {
            if failed {
                return f64::INFINITY;
            }
            let mut accumulator = ExactAccumulator::new();
            for (col, &magnitude) in local.iter().enumerate() {
                let part = product_upward(transform[col][row].abs(), magnitude);
                if accumulator.add(part).is_err() {
                    return f64::INFINITY;
                }
            }
            let sum = round_upward(&accumulator).unwrap_or(f64::INFINITY);
            let mut bound = ExactAccumulator::new();
            if bound.add(product_upward(gamma(16), sum)).is_err()
                || bound.add_product(64.0, tiny).is_err()
            {
                return f64::INFINITY;
            }
            round_upward(&bound).unwrap_or(f64::INFINITY)
        })
        .collect()
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
mod s11k_tests;

/// S11-G tests of the SP part: T7 (the formation variant), T17 (R-b's bound
/// on a skew member, V1 DN-1) and T14's SP range fallback.
#[cfg(test)]
mod s11g_tests {
    use super::*;

    const INVENTED_SECTION: (f64, f64, f64, f64, f64, f64) = (
        2.0e11,
        8.0e10,
        5.969026041820607e-3,
        2.701e-5,
        2.701e-5,
        5.402e-5,
    );

    fn member(to: [f64; 3]) -> StraightPipeElement {
        let (e, g, a, iy, iz, j) = INVENTED_SECTION;
        StraightPipeElement::new(
            "invented",
            FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
            FrameNode::new(1, to).unwrap(),
            StraightPipeSectionProperties::new(e, g, a, iy, iz, j, None).unwrap(),
            [0.0, 1.0, 0.0],
        )
        .unwrap()
    }

    fn components(formation: &Formation) -> (f64, Vec<f64>) {
        match formation {
            Formation::Exact {
                scale,
                scaled_intended,
            } => (*scale, scaled_intended.clone()),
            other => panic!("expected Exact, got {other:?}"),
        }
    }

    fn exact_equals(parts: &[f64], expected: &[(f64, f64)]) -> bool {
        let mut accumulator = ExactAccumulator::new();
        for &c in parts {
            accumulator.add(c).unwrap();
        }
        for &(a, b) in expected {
            accumulator.add_product(-a, b).unwrap();
        }
        accumulator.is_zero()
    }

    /// T7: the formation variant's values are bit-identical to today's, and
    /// its intended expansion equals the rational oracle: at b = 1, a = 0 on
    /// an axis-aligned member, 3 rotation_i = q L^2 / 4, transverse_i = q L / 2,
    /// 3 rotation_j = -q L^2 / 4 and transverse_j = q L / 2, exactly.
    #[test]
    fn t7_formed_variant_values_are_bit_identical_and_intended_is_the_oracle() {
        let q = 0.1_f64; // inexact intensity, so the formed values carry defects
        let length = 3.0_f64;
        let pipe = member([length, 0.0, 0.0]);
        let load = SpannedGlobalUniformLoad::full([0.0, q, 0.0]).unwrap();
        let (values, formations) = pipe
            .equivalent_global_nodal_loads_with_spans_formed(&[load])
            .unwrap();
        let today = pipe
            .equivalent_global_nodal_loads_with_spans(&[load], &[])
            .unwrap();
        assert_eq!(values.map(f64::to_bits), today.map(f64::to_bits));
        assert_eq!(formations.len(), ELEMENT_DOF);
        // The frame is the identity here (member along x, y_reference y): exact.
        let (scale, rz_i) = components(&formations[RZ]);
        assert_eq!(scale, 3.0);
        assert!(
            exact_equals(&rz_i, &[(q * 0.25, length * length)])
                || exact_equals(&rz_i, &[(q, 2.25)])
        );
        let (scale, uy_i) = components(&formations[UY]);
        assert_eq!(scale, 1.0);
        assert!(exact_equals(&uy_i, &[(q, 1.5)]));
        let (_, rz_j) = components(&formations[DOF_PER_NODE + RZ]);
        assert!(exact_equals(&rz_j, &[(-q, 2.25)]));
        let (_, uy_j) = components(&formations[DOF_PER_NODE + UY]);
        assert!(exact_equals(&uy_j, &[(q, 1.5)]));
        for slot in [UX, UZ, RX, RY, DOF_PER_NODE + UX, DOF_PER_NODE + UZ] {
            let (_, parts) = components(&formations[slot]);
            assert!(parts.iter().all(|&c| c == 0.0), "slot {slot}: {parts:?}");
        }
        // The defect of the formed rotation value is nonzero: formation noise
        // exists and is measured exactly (value - intended/3).
        let mut defect = ExactAccumulator::new();
        defect.add_product(3.0, values[RZ]).unwrap();
        for &c in &rz_i {
            defect.add(-c).unwrap();
        }
        assert!(
            !defect.is_zero(),
            "precondition: the formed rotation term is inexact"
        );
        // A skew member with a partial span: values bit-identical, and every
        // slot's intended value lies within 1e-14 of the formed value.
        let skew = member([3.0, 1.7, 0.4]);
        let spanned = SpannedGlobalUniformLoad::new(
            [12345.678, -2.5e4, 0.3],
            UniformLoadSpan::new(0.2, 0.9).unwrap(),
        )
        .unwrap();
        let (values, formations) = skew
            .equivalent_global_nodal_loads_with_spans_formed(&[spanned])
            .unwrap();
        let today = skew
            .equivalent_global_nodal_loads_with_spans(&[spanned], &[])
            .unwrap();
        assert_eq!(values.map(f64::to_bits), today.map(f64::to_bits));
        let peak = values.iter().fold(0.0_f64, |m, v| m.max(v.abs()));
        for (slot, formation) in formations.iter().enumerate() {
            let (scale, parts) = components(formation);
            let intended: f64 = parts.iter().copied().sum::<f64>() / scale;
            assert!(
                (intended - values[slot]).abs() <= 1e-14 * peak,
                "slot {slot}: {intended} vs {}",
                values[slot]
            );
        }
    }

    /// T14 (SP part): a subnormal intensity whose exact products fall below
    /// 2^-1074 takes the finite `Bounded` fallback; an overflowing intensity is refused by
    /// today's value checks before any formation exists. Nothing errs beyond
    /// today's own value checks.
    #[test]
    fn t14_formed_variant_range_fallbacks() {
        let pipe = member([3.0, 0.0, 0.0]);
        // An intensity whose formed terms overflow is refused by today's own
        // value checks before any formation exists (the exact expansion combines
        // each polynomial before scaling, so it stays in range whenever the
        // formed values do): the overflow fallback is not reached through SP.
        let big = SpannedGlobalUniformLoad::full([0.0, 8.0e307, 0.0]).unwrap();
        assert!(pipe
            .equivalent_global_nodal_loads_with_spans(&[big], &[])
            .is_err());
        assert!(pipe
            .equivalent_global_nodal_loads_with_spans_formed(&[big])
            .is_err());
        // Just inside the range: the formed values are finite and exact.
        let large = SpannedGlobalUniformLoad::full([0.0, 1.5e307, 0.0]).unwrap();
        let (values, formations) = pipe
            .equivalent_global_nodal_loads_with_spans_formed(&[large])
            .unwrap();
        assert!(values.iter().all(|v| v.is_finite()));
        assert!(formations
            .iter()
            .all(|f| matches!(f, Formation::Exact { .. })));
        // A subnormal intensity of three ulps: 3 rotation_i = 2.25 q is not a
        // multiple of 2^-1074, so the exact expansion leaves the binary64 range
        // and every slot takes the finite fallback.
        let tiny = SpannedGlobalUniformLoad::full([0.0, f64::from_bits(3), 0.0]).unwrap();
        let (_, formations) = pipe
            .equivalent_global_nodal_loads_with_spans_formed(&[tiny])
            .unwrap();
        for formation in &formations {
            match formation {
                Formation::Bounded { bound } => assert!(bound.is_finite() && *bound > 0.0),
                other => panic!("expected the Bounded fallback, got {other:?}"),
            }
        }
    }

    /// T17 (V1 DN-1): on a skew member (direction (3, 1.7, 0.4)) with mixed-sign
    /// displacements at both ends, B equals the hand-derived
    /// gamma_16 * sum_k |K[r][k]| * sum_c |T_kc| |u_c| (rows RY, RZ of each end;
    /// every sum exact and rounded upward) bit for bit, and differs from the
    /// |T u| variant (M14b), which T11's axis-aligned members cannot see.
    #[test]
    fn t17_bending_formation_bound_on_a_skew_member() {
        let pipe = member([3.0, 1.7, 0.4]);
        let u = [
            1.3e-3, -2.1e-3, 0.7e-3, -4.0e-4, 2.5e-4, -1.5e-4, //
            -0.9e-3, 1.7e-3, -2.2e-3, 3.1e-4, -2.7e-4, 1.1e-4,
        ];
        let bound = pipe.bending_formation_bound(&u).unwrap();
        let t = pipe
            .frame_element()
            .unwrap()
            .orientation()
            .unwrap()
            .transformation_matrix();
        let k = pipe.local_stiffness().unwrap();
        // Hand derivation.
        let up = |acc: &ExactAccumulator| {
            let r = acc.round().unwrap();
            let mut e = acc.clone();
            e.add(-r).unwrap();
            if e.signum() > 0 {
                r.next_up()
            } else {
                r
            }
        };
        let mut abs_tu = [0.0; ELEMENT_DOF];
        let mut tu_abs = [0.0; ELEMENT_DOF];
        for row in 0..ELEMENT_DOF {
            let mut sum_abs = ExactAccumulator::new();
            let mut signed = ExactAccumulator::new();
            for col in 0..ELEMENT_DOF {
                sum_abs
                    .add_product(t[row][col].abs(), u[col].abs())
                    .unwrap();
                signed.add_product(t[row][col], u[col]).unwrap();
            }
            abs_tu[row] = up(&sum_abs);
            tu_abs[row] = signed.round().unwrap().abs();
        }
        // Precondition: sum |T||u| differs from |T u| on this member.
        assert!((0..ELEMENT_DOF).any(|r| abs_tu[r].to_bits() != tu_abs[r].to_bits()));
        let g16 = {
            let ku = 16.0 * open_pipe_stress_frame_kernel::load_ledger::UNIT_ROUNDOFF;
            (ku / (1.0 - ku)).next_up()
        };
        for (end, (ry, rz)) in [(RY, RZ), (DOF_PER_NODE + RY, DOF_PER_NODE + RZ)]
            .into_iter()
            .enumerate()
        {
            let mut sum = ExactAccumulator::new();
            let mut variant = ExactAccumulator::new();
            for col in 0..ELEMENT_DOF {
                sum.add_product(k[ry][col].abs(), abs_tu[col]).unwrap();
                sum.add_product(k[rz][col].abs(), abs_tu[col]).unwrap();
                variant.add_product(k[ry][col].abs(), tu_abs[col]).unwrap();
                variant.add_product(k[rz][col].abs(), tu_abs[col]).unwrap();
            }
            let s = up(&sum);
            let p = g16 * s;
            let expected = if g16.mul_add(s, -p) > 0.0 {
                p.next_up()
            } else {
                p
            };
            assert_eq!(bound[end].to_bits(), expected.to_bits(), "end {end}");
            let m14b = g16 * up(&variant);
            assert_ne!(
                bound[end].to_bits(),
                m14b.to_bits(),
                "end {end}: |T u| variant is indistinguishable"
            );
        }
    }
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
    fn audit_direct_station_api_rejects_invalid_point_literals() {
        let pipe = element(None);
        let end = PipeEndResultants {
            end: PipeEnd::I,
            axial_force: 0.,
            shear_force_y: 0.,
            shear_force_z: 0.,
            torsional_moment: 0.,
            bending_moment_y: 0.,
            bending_moment_z: 0.,
        };
        for (fraction, force) in [(2., 10.), (-1., 10.), (0.9, f64::NAN)] {
            assert!(pipe
                .station_resultants_from_i_end(
                    end,
                    0.5,
                    &[],
                    &[PointLocalForce {
                        station_fraction: fraction,
                        direction: LocalLoadDirection::X,
                        force
                    }]
                )
                .is_err());
        }
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
            QuantityUnitMetadata::new(
                "fixture-force-per-length",
                CanonicalDimension::ForcePerLength,
            )
            .unwrap(),
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
        assert_eq!(
            metadata.weight_force_per_length_unit.dimension_id(),
            "force_per_length"
        );
        assert_eq!(
            metadata.source_model_ref.model_role.as_str(),
            "physical_source_of_truth"
        );
    }

    #[test]
    fn straight_pipe_boundary_metadata_rejects_tbd_force_per_length_unit() {
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
        );

        assert!(metadata.is_none());
    }
}
