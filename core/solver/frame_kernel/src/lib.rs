//! Code-neutral 3D frame stiffness kernel.
//!
//! This crate contains open mechanics routines only. It does not encode design
//! code compliance checks, protected standards content, or private project data.

use std::error::Error;
use std::fmt;

pub const DOF_PER_NODE: usize = 6;
pub const NODES_PER_ELEMENT: usize = 2;
pub const ELEMENT_DOF: usize = DOF_PER_NODE * NODES_PER_ELEMENT;
pub const UX: usize = 0;
pub const UY: usize = 1;
pub const UZ: usize = 2;
pub const RX: usize = 3;
pub const RY: usize = 4;
pub const RZ: usize = 5;

const AXIS_TOLERANCE: f64 = 1.0e-12;
// Internal dense verification pivot guard; not the project solver tolerance policy.
const DENSE_SOLVE_ZERO_PIVOT_GUARD: f64 = 1.0e-12;

pub type Matrix3 = [[f64; 3]; 3];
pub type Matrix12 = [[f64; ELEMENT_DOF]; ELEMENT_DOF];
pub type DenseMatrix = Vec<Vec<f64>>;
pub type DenseVector = Vec<f64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameDof {
    Ux,
    Uy,
    Uz,
    Rx,
    Ry,
    Rz,
}

impl FrameDof {
    pub const fn local_index(self) -> usize {
        match self {
            Self::Ux => UX,
            Self::Uy => UY,
            Self::Uz => UZ,
            Self::Rx => RX,
            Self::Ry => RY,
            Self::Rz => RZ,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ux => "ux",
            Self::Uy => "uy",
            Self::Uz => "uz",
            Self::Rx => "rx",
            Self::Ry => "ry",
            Self::Rz => "rz",
        }
    }

    pub const fn is_translational(self) -> bool {
        matches!(self, Self::Ux | Self::Uy | Self::Uz)
    }
}

pub const NODE_DOF_ORDER: [FrameDof; DOF_PER_NODE] = [
    FrameDof::Ux,
    FrameDof::Uy,
    FrameDof::Uz,
    FrameDof::Rx,
    FrameDof::Ry,
    FrameDof::Rz,
];

pub const fn node_dof_index(node_index: usize, dof: FrameDof) -> usize {
    node_index * DOF_PER_NODE + dof.local_index()
}

pub fn element_dof_map(node_i: usize, node_j: usize) -> [usize; ELEMENT_DOF] {
    let mut map = [0; ELEMENT_DOF];
    for local_dof in 0..DOF_PER_NODE {
        let dof = NODE_DOF_ORDER[local_dof];
        map[local_dof] = node_dof_index(node_i, dof);
        map[DOF_PER_NODE + local_dof] = node_dof_index(node_j, dof);
    }
    map
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalDimension {
    Dimensionless,
    Length,
    Mass,
    Time,
    Temperature,
    TemperatureInterval,
    Angle,
    Rotation,
    Force,
    Moment,
    Pressure,
    Stress,
    Area,
    Volume,
    Density,
    LinearStiffness,
    RotationalStiffness,
    Displacement,
    Velocity,
    Acceleration,
    ThermalConductivity,
    SpecificHeat,
    ThermalExpansionCoefficient,
    SecondMomentArea,
    SectionModulus,
    MassPerLength,
    ForcePerLength,
    VolumePerLength,
    Slope,
    Tbd,
}

impl CanonicalDimension {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Dimensionless => "dimensionless",
            Self::Length => "length",
            Self::Mass => "mass",
            Self::Time => "time",
            Self::Temperature => "temperature",
            Self::TemperatureInterval => "temperature_interval",
            Self::Angle => "angle",
            Self::Rotation => "rotation",
            Self::Force => "force",
            Self::Moment => "moment",
            Self::Pressure => "pressure",
            Self::Stress => "stress",
            Self::Area => "area",
            Self::Volume => "volume",
            Self::Density => "density",
            Self::LinearStiffness => "linear_stiffness",
            Self::RotationalStiffness => "rotational_stiffness",
            Self::Displacement => "displacement",
            Self::Velocity => "velocity",
            Self::Acceleration => "acceleration",
            Self::ThermalConductivity => "thermal_conductivity",
            Self::SpecificHeat => "specific_heat",
            Self::ThermalExpansionCoefficient => "thermal_expansion_coefficient",
            Self::SecondMomentArea => "second_moment_area",
            Self::SectionModulus => "section_modulus",
            Self::MassPerLength => "mass_per_length",
            Self::ForcePerLength => "force_per_length",
            Self::VolumePerLength => "volume_per_length",
            Self::Slope => "slope",
            Self::Tbd => "TBD",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitSystemRef {
    pub unit_system_id: String,
}

impl UnitSystemRef {
    pub fn new(unit_system_id: impl Into<String>) -> Option<Self> {
        normalize_explicit_id(unit_system_id).map(|unit_system_id| Self { unit_system_id })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantityUnitMetadata {
    pub unit_id: String,
    pub dimension: CanonicalDimension,
}

impl QuantityUnitMetadata {
    pub fn new(unit_id: impl Into<String>, dimension: CanonicalDimension) -> Option<Self> {
        normalize_explicit_id(unit_id).map(|unit_id| Self { unit_id, dimension })
    }

    pub fn dimension_id(&self) -> &'static str {
        self.dimension.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameKernelUnitBasis {
    pub unit_system_ref: UnitSystemRef,
    pub coordinate_unit: QuantityUnitMetadata,
    pub force_unit: QuantityUnitMetadata,
    pub moment_unit: QuantityUnitMetadata,
    pub stress_unit: QuantityUnitMetadata,
    pub area_unit: QuantityUnitMetadata,
    pub second_moment_area_unit: QuantityUnitMetadata,
    pub displacement_unit: QuantityUnitMetadata,
    pub rotation_unit: QuantityUnitMetadata,
}

impl FrameKernelUnitBasis {
    pub fn new(
        unit_system_ref: UnitSystemRef,
        coordinate_unit: QuantityUnitMetadata,
        force_unit: QuantityUnitMetadata,
        moment_unit: QuantityUnitMetadata,
        stress_unit: QuantityUnitMetadata,
        area_unit: QuantityUnitMetadata,
        second_moment_area_unit: QuantityUnitMetadata,
        displacement_unit: QuantityUnitMetadata,
        rotation_unit: QuantityUnitMetadata,
    ) -> Option<Self> {
        let basis = Self {
            unit_system_ref,
            coordinate_unit,
            force_unit,
            moment_unit,
            stress_unit,
            area_unit,
            second_moment_area_unit,
            displacement_unit,
            rotation_unit,
        };
        basis.has_expected_dimensions().then_some(basis)
    }

    pub fn from_unit_ids(
        unit_system_id: impl Into<String>,
        length_unit_id: impl Into<String>,
        force_unit_id: impl Into<String>,
        moment_unit_id: impl Into<String>,
        stress_unit_id: impl Into<String>,
        area_unit_id: impl Into<String>,
        second_moment_area_unit_id: impl Into<String>,
        displacement_unit_id: impl Into<String>,
        rotation_unit_id: impl Into<String>,
    ) -> Option<Self> {
        Self::new(
            UnitSystemRef::new(unit_system_id)?,
            QuantityUnitMetadata::new(length_unit_id, CanonicalDimension::Length)?,
            QuantityUnitMetadata::new(force_unit_id, CanonicalDimension::Force)?,
            QuantityUnitMetadata::new(moment_unit_id, CanonicalDimension::Moment)?,
            QuantityUnitMetadata::new(stress_unit_id, CanonicalDimension::Stress)?,
            QuantityUnitMetadata::new(area_unit_id, CanonicalDimension::Area)?,
            QuantityUnitMetadata::new(
                second_moment_area_unit_id,
                CanonicalDimension::SecondMomentArea,
            )?,
            QuantityUnitMetadata::new(displacement_unit_id, CanonicalDimension::Displacement)?,
            QuantityUnitMetadata::new(rotation_unit_id, CanonicalDimension::Rotation)?,
        )
    }

    pub fn has_expected_dimensions(&self) -> bool {
        self.coordinate_unit.dimension == CanonicalDimension::Length
            && self.force_unit.dimension == CanonicalDimension::Force
            && self.moment_unit.dimension == CanonicalDimension::Moment
            && self.stress_unit.dimension == CanonicalDimension::Stress
            && self.area_unit.dimension == CanonicalDimension::Area
            && self.second_moment_area_unit.dimension == CanonicalDimension::SecondMomentArea
            && self.displacement_unit.dimension == CanonicalDimension::Displacement
            && self.rotation_unit.dimension == CanonicalDimension::Rotation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalModelRole {
    PhysicalSourceOfTruth,
    AnalyticalSolverModel,
}

impl CanonicalModelRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PhysicalSourceOfTruth => "physical_source_of_truth",
            Self::AnalyticalSolverModel => "analytical_solver_model",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalModelReference {
    pub model_id: String,
    pub model_role: CanonicalModelRole,
    pub object_ref: String,
}

impl CanonicalModelReference {
    pub fn new(
        model_id: impl Into<String>,
        model_role: CanonicalModelRole,
        object_ref: impl Into<String>,
    ) -> Option<Self> {
        let model_id = normalize_explicit_id(model_id)?;
        let object_ref = normalize_explicit_id(object_ref)?;
        Some(Self {
            model_id,
            model_role,
            object_ref,
        })
    }
}

fn normalize_explicit_id(value: impl Into<String>) -> Option<String> {
    let value = value.into();
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("TBD") {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FrameKernelError {
    NonFiniteInput {
        name: &'static str,
        value: f64,
    },
    NonPositiveInput {
        name: &'static str,
        value: f64,
    },
    DegenerateAxis {
        detail: &'static str,
    },
    InvalidOrientation {
        detail: &'static str,
    },
    InvalidNodeIndex {
        node_index: usize,
        node_count: usize,
    },
    RepeatedElementNodeIndex {
        node_index: usize,
    },
    InvalidMatrixDimensions {
        rows: usize,
        cols: usize,
    },
    InvalidVectorLength {
        expected: usize,
        actual: usize,
    },
    RepeatedRestrainedDof {
        dof: usize,
    },
    RestrainedDofOutOfRange {
        dof: usize,
        total_dofs: usize,
    },
    RepeatedPrescribedDof {
        dof: usize,
    },
    PrescribedDofOutOfRange {
        dof: usize,
        total_dofs: usize,
    },
    SingularSystem {
        pivot: usize,
    },
}

impl fmt::Display for FrameKernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteInput { name, value } => {
                write!(f, "{name} must be finite, got {value}")
            }
            Self::NonPositiveInput { name, value } => {
                write!(f, "{name} must be positive, got {value}")
            }
            Self::DegenerateAxis { detail } => write!(f, "degenerate axis definition: {detail}"),
            Self::InvalidOrientation { detail } => write!(f, "invalid orientation: {detail}"),
            Self::InvalidNodeIndex {
                node_index,
                node_count,
            } => write!(
                f,
                "node index {node_index} is outside node count {node_count}"
            ),
            Self::RepeatedElementNodeIndex { node_index } => {
                write!(f, "element connects node index {node_index} to itself")
            }
            Self::InvalidMatrixDimensions { rows, cols } => {
                write!(f, "matrix must be square, got {rows} by {cols}")
            }
            Self::InvalidVectorLength { expected, actual } => {
                write!(f, "vector length must be {expected}, got {actual}")
            }
            Self::RepeatedRestrainedDof { dof } => {
                write!(f, "restrained DOF {dof} was repeated")
            }
            Self::RestrainedDofOutOfRange { dof, total_dofs } => write!(
                f,
                "restrained DOF {dof} is outside total DOF count {total_dofs}"
            ),
            Self::RepeatedPrescribedDof { dof } => {
                write!(f, "prescribed DOF {dof} was repeated")
            }
            Self::PrescribedDofOutOfRange { dof, total_dofs } => write!(
                f,
                "prescribed DOF {dof} is outside total DOF count {total_dofs}"
            ),
            Self::SingularSystem { pivot } => write!(f, "singular system at pivot {pivot}"),
        }
    }
}

impl Error for FrameKernelError {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameNode {
    pub index: usize,
    pub coordinates: [f64; 3],
}

impl FrameNode {
    pub fn new(index: usize, coordinates: [f64; 3]) -> Result<Self, FrameKernelError> {
        validate_finite_vector("coordinates", coordinates)?;
        Ok(Self { index, coordinates })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameSection {
    pub elastic_modulus: f64,
    pub shear_modulus: f64,
    pub area: f64,
    pub second_moment_y: f64,
    pub second_moment_z: f64,
    pub torsion_constant: f64,
}

impl FrameSection {
    pub fn new(
        elastic_modulus: f64,
        shear_modulus: f64,
        area: f64,
        second_moment_y: f64,
        second_moment_z: f64,
        torsion_constant: f64,
    ) -> Result<Self, FrameKernelError> {
        validate_positive_finite("elastic_modulus", elastic_modulus)?;
        validate_positive_finite("shear_modulus", shear_modulus)?;
        validate_positive_finite("area", area)?;
        validate_positive_finite("second_moment_y", second_moment_y)?;
        validate_positive_finite("second_moment_z", second_moment_z)?;
        validate_positive_finite("torsion_constant", torsion_constant)?;

        Ok(Self {
            elastic_modulus,
            shear_modulus,
            area,
            second_moment_y,
            second_moment_z,
            torsion_constant,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameProperties {
    pub section: FrameSection,
    pub length: f64,
}

impl FrameProperties {
    pub fn new(section: FrameSection, length: f64) -> Result<Self, FrameKernelError> {
        validate_positive_finite("length", length)?;
        Ok(Self { section, length })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameOrientation {
    /// Rows are local unit axes expressed in global coordinates: x, y, z.
    pub local_axes: Matrix3,
}

impl FrameOrientation {
    pub fn new(local_axes: Matrix3) -> Result<Self, FrameKernelError> {
        for axis in local_axes {
            validate_finite_vector("local_axes", axis)?;
        }
        validate_unit_axis(local_axes[0])?;
        validate_unit_axis(local_axes[1])?;
        validate_unit_axis(local_axes[2])?;

        if dot(local_axes[0], local_axes[1]).abs() > AXIS_TOLERANCE
            || dot(local_axes[0], local_axes[2]).abs() > AXIS_TOLERANCE
            || dot(local_axes[1], local_axes[2]).abs() > AXIS_TOLERANCE
        {
            return Err(FrameKernelError::InvalidOrientation {
                detail: "local axes must be mutually orthogonal",
            });
        }

        let right_handed_z = cross(local_axes[0], local_axes[1]);
        if max_abs_component(subtract(right_handed_z, local_axes[2])) > AXIS_TOLERANCE {
            return Err(FrameKernelError::InvalidOrientation {
                detail: "local axes must form a right-handed basis",
            });
        }

        Ok(Self { local_axes })
    }

    pub fn from_x_axis_and_y_reference(
        local_x_axis: [f64; 3],
        y_reference: [f64; 3],
    ) -> Result<Self, FrameKernelError> {
        validate_finite_vector("local_x_axis", local_x_axis)?;
        validate_finite_vector("y_reference", y_reference)?;

        let x_axis = normalize(local_x_axis, "local x axis")?;
        let projection = dot(y_reference, x_axis);
        let y_candidate = subtract(y_reference, scale(x_axis, projection));
        let y_axis = normalize(y_candidate, "y reference parallel to local x axis")?;
        let z_axis = normalize(cross(x_axis, y_axis), "local z axis")?;

        Self::new([x_axis, y_axis, z_axis])
    }

    pub fn transformation_matrix(&self) -> Matrix12 {
        let mut transform = [[0.0; ELEMENT_DOF]; ELEMENT_DOF];

        for node in 0..NODES_PER_ELEMENT {
            let base = node * DOF_PER_NODE;
            copy_rotation_block(&mut transform, base, base, self.local_axes);
            copy_rotation_block(&mut transform, base + RX, base + RX, self.local_axes);
        }

        transform
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameElement {
    pub node_i: FrameNode,
    pub node_j: FrameNode,
    pub section: FrameSection,
    pub y_reference: [f64; 3],
}

impl FrameElement {
    pub fn new(
        node_i: FrameNode,
        node_j: FrameNode,
        section: FrameSection,
        y_reference: [f64; 3],
    ) -> Result<Self, FrameKernelError> {
        if node_i.index == node_j.index {
            return Err(FrameKernelError::RepeatedElementNodeIndex {
                node_index: node_i.index,
            });
        }
        validate_finite_vector("y_reference", y_reference)?;
        let element = Self {
            node_i,
            node_j,
            section,
            y_reference,
        };
        element.properties()?;
        element.orientation()?;
        Ok(element)
    }

    pub fn length(&self) -> Result<f64, FrameKernelError> {
        let delta = subtract(self.node_j.coordinates, self.node_i.coordinates);
        normalize(delta, "element length")?;
        Ok(norm(delta))
    }

    pub fn local_x_axis(&self) -> Result<[f64; 3], FrameKernelError> {
        normalize(
            subtract(self.node_j.coordinates, self.node_i.coordinates),
            "element length",
        )
    }

    pub fn properties(&self) -> Result<FrameProperties, FrameKernelError> {
        FrameProperties::new(self.section, self.length()?)
    }

    pub fn orientation(&self) -> Result<FrameOrientation, FrameKernelError> {
        FrameOrientation::from_x_axis_and_y_reference(self.local_x_axis()?, self.y_reference)
    }

    pub fn local_stiffness(&self) -> Result<Matrix12, FrameKernelError> {
        local_stiffness(self.properties()?)
    }

    pub fn global_stiffness(&self) -> Result<Matrix12, FrameKernelError> {
        let local = self.local_stiffness()?;
        let orientation = self.orientation()?;
        Ok(transform_global_stiffness(&local, &orientation))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReducedSystem {
    pub stiffness: DenseMatrix,
    pub force: DenseVector,
    pub free_dofs: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BoundaryDofKind {
    Restrained,
    Prescribed,
}

pub fn local_stiffness(properties: FrameProperties) -> Result<Matrix12, FrameKernelError> {
    let section = properties.section;
    validate_positive_finite("elastic_modulus", section.elastic_modulus)?;
    validate_positive_finite("shear_modulus", section.shear_modulus)?;
    validate_positive_finite("area", section.area)?;
    validate_positive_finite("second_moment_y", section.second_moment_y)?;
    validate_positive_finite("second_moment_z", section.second_moment_z)?;
    validate_positive_finite("torsion_constant", section.torsion_constant)?;
    validate_positive_finite("length", properties.length)?;

    let e = section.elastic_modulus;
    let g = section.shear_modulus;
    let area = section.area;
    let iy = section.second_moment_y;
    let iz = section.second_moment_z;
    let j = section.torsion_constant;
    let length = properties.length;
    let length2 = length * length;
    let length3 = length2 * length;

    let axial = e * area / length;
    let torsion = g * j / length;
    let bend_y_12 = 12.0 * e * iy / length3;
    let bend_y_6 = 6.0 * e * iy / length2;
    let bend_y_4 = 4.0 * e * iy / length;
    let bend_y_2 = 2.0 * e * iy / length;
    let bend_z_12 = 12.0 * e * iz / length3;
    let bend_z_6 = 6.0 * e * iz / length2;
    let bend_z_4 = 4.0 * e * iz / length;
    let bend_z_2 = 2.0 * e * iz / length;

    let mut stiffness = [[0.0; ELEMENT_DOF]; ELEMENT_DOF];

    set_symmetric(&mut stiffness, UX, UX + DOF_PER_NODE, -axial);
    stiffness[UX][UX] = axial;
    stiffness[UX + DOF_PER_NODE][UX + DOF_PER_NODE] = axial;

    set_symmetric(&mut stiffness, RX, RX + DOF_PER_NODE, -torsion);
    stiffness[RX][RX] = torsion;
    stiffness[RX + DOF_PER_NODE][RX + DOF_PER_NODE] = torsion;

    add_bending_z(&mut stiffness, bend_z_12, bend_z_6, bend_z_4, bend_z_2);
    add_bending_y(&mut stiffness, bend_y_12, bend_y_6, bend_y_4, bend_y_2);

    Ok(stiffness)
}

pub fn transform_global_stiffness(
    local_stiffness: &Matrix12,
    orientation: &FrameOrientation,
) -> Matrix12 {
    let transform = orientation.transformation_matrix();
    multiply_transpose_left(local_stiffness, &transform)
}

pub fn assemble_global_stiffness(
    node_count: usize,
    elements: &[FrameElement],
) -> Result<DenseMatrix, FrameKernelError> {
    let total_dofs = node_count * DOF_PER_NODE;
    let mut global = vec![vec![0.0; total_dofs]; total_dofs];

    for element in elements {
        validate_node_index(element.node_i.index, node_count)?;
        validate_node_index(element.node_j.index, node_count)?;

        let element_stiffness = element.global_stiffness()?;
        let dof_map = element_dof_map(element.node_i.index, element.node_j.index);

        for local_row in 0..ELEMENT_DOF {
            let global_row = dof_map[local_row];
            for local_col in 0..ELEMENT_DOF {
                let global_col = dof_map[local_col];
                global[global_row][global_col] += element_stiffness[local_row][local_col];
            }
        }
    }

    Ok(global)
}

pub fn reduce_system(
    stiffness: &[Vec<f64>],
    force: &[f64],
    restrained_dofs: &[usize],
) -> Result<ReducedSystem, FrameKernelError> {
    let restrained_displacements = vec![0.0; restrained_dofs.len()];
    reduce_system_for_boundary(
        stiffness,
        force,
        restrained_dofs,
        &restrained_displacements,
        BoundaryDofKind::Restrained,
    )
}

pub fn reduce_system_with_prescribed_displacements(
    stiffness: &[Vec<f64>],
    force: &[f64],
    prescribed_dofs: &[usize],
    prescribed_displacements: &[f64],
) -> Result<ReducedSystem, FrameKernelError> {
    reduce_system_for_boundary(
        stiffness,
        force,
        prescribed_dofs,
        prescribed_displacements,
        BoundaryDofKind::Prescribed,
    )
}

fn reduce_system_for_boundary(
    stiffness: &[Vec<f64>],
    force: &[f64],
    boundary_dofs: &[usize],
    boundary_displacements: &[f64],
    boundary_kind: BoundaryDofKind,
) -> Result<ReducedSystem, FrameKernelError> {
    let size = validate_square_matrix(stiffness)?;
    if force.len() != size {
        return Err(FrameKernelError::InvalidVectorLength {
            expected: size,
            actual: force.len(),
        });
    }
    validate_finite_slice(force)?;
    if boundary_displacements.len() != boundary_dofs.len() {
        return Err(FrameKernelError::InvalidVectorLength {
            expected: boundary_dofs.len(),
            actual: boundary_displacements.len(),
        });
    }
    validate_named_finite_slice("prescribed displacement", boundary_displacements)?;

    let mut constrained = vec![false; size];
    for &dof in boundary_dofs {
        if dof >= size {
            return Err(boundary_out_of_range_error(boundary_kind, dof, size));
        }
        if constrained[dof] {
            return Err(repeated_boundary_error(boundary_kind, dof));
        }
        constrained[dof] = true;
    }

    let free_dofs: Vec<usize> = (0..size).filter(|&dof| !constrained[dof]).collect();
    let mut reduced_stiffness = vec![vec![0.0; free_dofs.len()]; free_dofs.len()];
    let mut reduced_force = vec![0.0; free_dofs.len()];

    for (reduced_row, &global_row) in free_dofs.iter().enumerate() {
        let mut adjusted_force = force[global_row];
        for (&boundary_dof, &boundary_displacement) in
            boundary_dofs.iter().zip(boundary_displacements.iter())
        {
            adjusted_force -= stiffness[global_row][boundary_dof] * boundary_displacement;
        }
        reduced_force[reduced_row] = adjusted_force;
        for (reduced_col, &global_col) in free_dofs.iter().enumerate() {
            reduced_stiffness[reduced_row][reduced_col] = stiffness[global_row][global_col];
        }
    }

    Ok(ReducedSystem {
        stiffness: reduced_stiffness,
        force: reduced_force,
        free_dofs,
    })
}

fn boundary_out_of_range_error(
    boundary_kind: BoundaryDofKind,
    dof: usize,
    total_dofs: usize,
) -> FrameKernelError {
    match boundary_kind {
        BoundaryDofKind::Restrained => {
            FrameKernelError::RestrainedDofOutOfRange { dof, total_dofs }
        }
        BoundaryDofKind::Prescribed => {
            FrameKernelError::PrescribedDofOutOfRange { dof, total_dofs }
        }
    }
}

fn repeated_boundary_error(boundary_kind: BoundaryDofKind, dof: usize) -> FrameKernelError {
    match boundary_kind {
        BoundaryDofKind::Restrained => FrameKernelError::RepeatedRestrainedDof { dof },
        BoundaryDofKind::Prescribed => FrameKernelError::RepeatedPrescribedDof { dof },
    }
}

pub fn solve_dense(stiffness: &[Vec<f64>], force: &[f64]) -> Result<DenseVector, FrameKernelError> {
    let size = validate_square_matrix(stiffness)?;
    if force.len() != size {
        return Err(FrameKernelError::InvalidVectorLength {
            expected: size,
            actual: force.len(),
        });
    }
    validate_finite_slice(force)?;

    let mut matrix = stiffness.to_vec();
    let mut rhs = force.to_vec();

    for pivot in 0..size {
        let mut pivot_row = pivot;
        let mut pivot_value = matrix[pivot][pivot].abs();

        for (row, values) in matrix.iter().enumerate().skip(pivot + 1) {
            let candidate = values[pivot].abs();
            if candidate > pivot_value {
                pivot_row = row;
                pivot_value = candidate;
            }
        }

        if pivot_value <= DENSE_SOLVE_ZERO_PIVOT_GUARD {
            return Err(FrameKernelError::SingularSystem { pivot });
        }

        if pivot_row != pivot {
            matrix.swap(pivot, pivot_row);
            rhs.swap(pivot, pivot_row);
        }

        let pivot_diagonal = matrix[pivot][pivot];
        let pivot_tail = matrix[pivot][(pivot + 1)..].to_vec();
        for (row_index, row_values) in matrix.iter_mut().enumerate().skip(pivot + 1) {
            let factor = row_values[pivot] / pivot_diagonal;
            row_values[pivot] = 0.0;
            for (entry, pivot_entry) in row_values.iter_mut().skip(pivot + 1).zip(pivot_tail.iter())
            {
                *entry -= factor * pivot_entry;
            }
            rhs[row_index] -= factor * rhs[pivot];
        }
    }

    let mut solution = vec![0.0; size];
    for row in (0..size).rev() {
        let mut sum = rhs[row];
        for (col, value) in matrix[row].iter().enumerate().skip(row + 1) {
            sum -= value * solution[col];
        }
        if matrix[row][row].abs() <= DENSE_SOLVE_ZERO_PIVOT_GUARD {
            return Err(FrameKernelError::SingularSystem { pivot: row });
        }
        solution[row] = sum / matrix[row][row];
    }

    Ok(solution)
}

fn add_bending_z(stiffness: &mut Matrix12, k12: f64, k6: f64, k4: f64, k2: f64) {
    let n1_v = UY;
    let n1_rz = RZ;
    let n2_v = UY + DOF_PER_NODE;
    let n2_rz = RZ + DOF_PER_NODE;
    let indices = [n1_v, n1_rz, n2_v, n2_rz];
    let terms = [
        [k12, k6, -k12, k6],
        [k6, k4, -k6, k2],
        [-k12, -k6, k12, -k6],
        [k6, k2, -k6, k4],
    ];
    add_terms(stiffness, indices, terms);
}

fn add_bending_y(stiffness: &mut Matrix12, k12: f64, k6: f64, k4: f64, k2: f64) {
    let n1_w = UZ;
    let n1_ry = RY;
    let n2_w = UZ + DOF_PER_NODE;
    let n2_ry = RY + DOF_PER_NODE;
    let indices = [n1_w, n1_ry, n2_w, n2_ry];
    let terms = [
        [k12, -k6, -k12, -k6],
        [-k6, k4, k6, k2],
        [-k12, k6, k12, k6],
        [-k6, k2, k6, k4],
    ];
    add_terms(stiffness, indices, terms);
}

fn add_terms(stiffness: &mut Matrix12, indices: [usize; 4], terms: [[f64; 4]; 4]) {
    for row in 0..4 {
        for col in 0..4 {
            stiffness[indices[row]][indices[col]] += terms[row][col];
        }
    }
}

fn set_symmetric(stiffness: &mut Matrix12, row: usize, col: usize, value: f64) {
    stiffness[row][col] = value;
    stiffness[col][row] = value;
}

fn multiply_transpose_left(stiffness: &Matrix12, transform: &Matrix12) -> Matrix12 {
    let mut temp = [[0.0; ELEMENT_DOF]; ELEMENT_DOF];
    for row in 0..ELEMENT_DOF {
        for col in 0..ELEMENT_DOF {
            for inner in 0..ELEMENT_DOF {
                temp[row][col] += stiffness[row][inner] * transform[inner][col];
            }
        }
    }

    let mut result = [[0.0; ELEMENT_DOF]; ELEMENT_DOF];
    for row in 0..ELEMENT_DOF {
        for col in 0..ELEMENT_DOF {
            for inner in 0..ELEMENT_DOF {
                result[row][col] += transform[inner][row] * temp[inner][col];
            }
        }
    }
    result
}

fn copy_rotation_block(matrix: &mut Matrix12, row_offset: usize, col_offset: usize, axes: Matrix3) {
    for row in 0..3 {
        for col in 0..3 {
            matrix[row_offset + row][col_offset + col] = axes[row][col];
        }
    }
}

fn validate_square_matrix(matrix: &[Vec<f64>]) -> Result<usize, FrameKernelError> {
    let rows = matrix.len();
    for row in matrix {
        if row.len() != rows {
            return Err(FrameKernelError::InvalidMatrixDimensions {
                rows,
                cols: row.len(),
            });
        }
        validate_finite_slice(row)?;
    }
    Ok(rows)
}

fn validate_node_index(node_index: usize, node_count: usize) -> Result<(), FrameKernelError> {
    if node_index >= node_count {
        return Err(FrameKernelError::InvalidNodeIndex {
            node_index,
            node_count,
        });
    }
    Ok(())
}

fn validate_positive_finite(name: &'static str, value: f64) -> Result<(), FrameKernelError> {
    if !value.is_finite() {
        return Err(FrameKernelError::NonFiniteInput { name, value });
    }
    if value <= 0.0 {
        return Err(FrameKernelError::NonPositiveInput { name, value });
    }
    Ok(())
}

fn validate_finite_vector(name: &'static str, vector: [f64; 3]) -> Result<(), FrameKernelError> {
    for value in vector {
        if !value.is_finite() {
            return Err(FrameKernelError::NonFiniteInput { name, value });
        }
    }
    Ok(())
}

fn validate_finite_slice(values: &[f64]) -> Result<(), FrameKernelError> {
    validate_named_finite_slice("matrix/vector entry", values)
}

fn validate_named_finite_slice(name: &'static str, values: &[f64]) -> Result<(), FrameKernelError> {
    for &value in values {
        if !value.is_finite() {
            return Err(FrameKernelError::NonFiniteInput { name, value });
        }
    }
    Ok(())
}

fn normalize(vector: [f64; 3], detail: &'static str) -> Result<[f64; 3], FrameKernelError> {
    let magnitude = norm(vector);
    if magnitude <= AXIS_TOLERANCE {
        return Err(FrameKernelError::DegenerateAxis { detail });
    }
    Ok(scale(vector, 1.0 / magnitude))
}

fn validate_unit_axis(axis: [f64; 3]) -> Result<(), FrameKernelError> {
    if (norm(axis) - 1.0).abs() > AXIS_TOLERANCE {
        return Err(FrameKernelError::InvalidOrientation {
            detail: "local axes must be unit length",
        });
    }
    Ok(())
}

fn norm(vector: [f64; 3]) -> f64 {
    dot(vector, vector).sqrt()
}

fn dot(left: [f64; 3], right: [f64; 3]) -> f64 {
    left[0] * right[0] + left[1] * right[1] + left[2] * right[2]
}

fn subtract(left: [f64; 3], right: [f64; 3]) -> [f64; 3] {
    [left[0] - right[0], left[1] - right[1], left[2] - right[2]]
}

fn scale(vector: [f64; 3], scalar: f64) -> [f64; 3] {
    [vector[0] * scalar, vector[1] * scalar, vector[2] * scalar]
}

fn cross(left: [f64; 3], right: [f64; 3]) -> [f64; 3] {
    [
        left[1] * right[2] - left[2] * right[1],
        left[2] * right[0] - left[0] * right[2],
        left[0] * right[1] - left[1] * right[0],
    ]
}

fn max_abs_component(vector: [f64; 3]) -> f64 {
    vector[0].abs().max(vector[1].abs()).max(vector[2].abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ASSERT_TOLERANCE: f64 = 1.0e-9;

    #[test]
    fn six_dof_mapping_is_stable_and_node_based() {
        assert_eq!(
            NODE_DOF_ORDER.map(|dof| dof.as_str()),
            ["ux", "uy", "uz", "rx", "ry", "rz"]
        );
        assert_eq!(FrameDof::Ux.local_index(), UX);
        assert_eq!(FrameDof::Uy.local_index(), UY);
        assert_eq!(FrameDof::Uz.local_index(), UZ);
        assert_eq!(FrameDof::Rx.local_index(), RX);
        assert_eq!(FrameDof::Ry.local_index(), RY);
        assert_eq!(FrameDof::Rz.local_index(), RZ);
        assert!(FrameDof::Ux.is_translational());
        assert!(FrameDof::Uy.is_translational());
        assert!(FrameDof::Uz.is_translational());
        assert!(!FrameDof::Rx.is_translational());
        assert!(!FrameDof::Ry.is_translational());
        assert!(!FrameDof::Rz.is_translational());
        assert_eq!(node_dof_index(2, FrameDof::Ry), 16);

        let map = element_dof_map(2, 5);
        assert_eq!(map, [12, 13, 14, 15, 16, 17, 30, 31, 32, 33, 34, 35]);
    }

    #[test]
    fn local_stiffness_contains_standard_terms() {
        let section = FrameSection::new(10.0, 20.0, 3.0, 5.0, 7.0, 11.0).unwrap();
        let properties = FrameProperties::new(section, 2.0).unwrap();
        let stiffness = local_stiffness(properties).unwrap();

        assert_close(stiffness[UX][UX], 15.0);
        assert_close(stiffness[UX][UX + DOF_PER_NODE], -15.0);
        assert_close(stiffness[RX][RX], 110.0);
        assert_close(stiffness[RX][RX + DOF_PER_NODE], -110.0);

        assert_close(stiffness[UY][UY], 105.0);
        assert_close(stiffness[UY][RZ], 105.0);
        assert_close(stiffness[RZ][RZ], 140.0);
        assert_close(stiffness[RZ][RZ + DOF_PER_NODE], 70.0);

        assert_close(stiffness[UZ][UZ], 75.0);
        assert_close(stiffness[UZ][RY], -75.0);
        assert_close(stiffness[RY][RY], 100.0);
        assert_close(stiffness[RY][RY + DOF_PER_NODE], 50.0);
        assert_symmetric_12(&stiffness);
    }

    #[test]
    fn global_x_member_transform_matches_local_stiffness() {
        let section = FrameSection::new(10.0, 20.0, 3.0, 5.0, 7.0, 11.0).unwrap();
        let properties = FrameProperties::new(section, 2.0).unwrap();
        let local = local_stiffness(properties).unwrap();
        let orientation =
            FrameOrientation::from_x_axis_and_y_reference([1.0, 0.0, 0.0], [0.0, 1.0, 0.0])
                .unwrap();

        let global = transform_global_stiffness(&local, &orientation);

        for row in 0..ELEMENT_DOF {
            for col in 0..ELEMENT_DOF {
                assert_close(global[row][col], local[row][col]);
            }
        }
    }

    #[test]
    fn orientation_constructor_rejects_non_orthonormal_axes() {
        let non_unit_error =
            FrameOrientation::new([[2.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]).unwrap_err();
        assert_eq!(
            non_unit_error,
            FrameKernelError::InvalidOrientation {
                detail: "local axes must be unit length"
            }
        );

        let non_orthogonal_error = FrameOrientation::new([
            [1.0, 0.0, 0.0],
            [0.7071067811865475, 0.7071067811865475, 0.0],
            [0.0, 0.0, 1.0],
        ])
        .unwrap_err();
        assert_eq!(
            non_orthogonal_error,
            FrameKernelError::InvalidOrientation {
                detail: "local axes must be mutually orthogonal"
            }
        );

        let left_handed_error =
            FrameOrientation::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, -1.0]])
                .unwrap_err();
        assert_eq!(
            left_handed_error,
            FrameKernelError::InvalidOrientation {
                detail: "local axes must form a right-handed basis"
            }
        );
    }

    #[test]
    fn transformation_matrix_maps_translation_and_rotation_blocks() {
        let orientation =
            FrameOrientation::from_x_axis_and_y_reference([0.0, 1.0, 0.0], [0.0, 0.0, 1.0])
                .unwrap();
        let transform = orientation.transformation_matrix();

        for block_offset in [0, RX, DOF_PER_NODE, DOF_PER_NODE + RX] {
            assert_close(transform[block_offset + UX][block_offset + UY], 1.0);
            assert_close(transform[block_offset + UY][block_offset + UZ], 1.0);
            assert_close(transform[block_offset + UZ][block_offset + UX], 1.0);
        }
        for row in 0..ELEMENT_DOF {
            let row_abs_sum = transform[row].iter().map(|entry| entry.abs()).sum::<f64>();
            assert_close(row_abs_sum, 1.0);
        }
    }

    #[test]
    fn rotated_member_transform_preserves_symmetry() {
        let section = FrameSection::new(10.0, 20.0, 3.0, 5.0, 7.0, 11.0).unwrap();
        let properties = FrameProperties::new(section, 2.0).unwrap();
        let local = local_stiffness(properties).unwrap();
        let orientation =
            FrameOrientation::from_x_axis_and_y_reference([0.0, 1.0, 0.0], [0.0, 0.0, 1.0])
                .unwrap();

        let global = transform_global_stiffness(&local, &orientation);

        assert_symmetric_12(&global);
    }

    #[test]
    fn rotated_member_maps_local_axial_stiffness_to_global_axis() {
        let section = FrameSection::new(10.0, 20.0, 3.0, 5.0, 7.0, 11.0).unwrap();
        let properties = FrameProperties::new(section, 2.0).unwrap();
        let local = local_stiffness(properties).unwrap();
        let orientation =
            FrameOrientation::from_x_axis_and_y_reference([0.0, 1.0, 0.0], [0.0, 0.0, 1.0])
                .unwrap();

        let global = transform_global_stiffness(&local, &orientation);

        assert_close(global[UY][UY], local[UX][UX]);
        assert_close(global[UY][UY + DOF_PER_NODE], local[UX][UX + DOF_PER_NODE]);
        assert_close(global[UX][UX], local[UZ][UZ]);
    }

    #[test]
    fn assembly_accumulates_shared_node_contributions() {
        let section = FrameSection::new(1000.0, 400.0, 2.0, 1.0, 1.0, 1.0).unwrap();
        let node_0 = FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap();
        let node_1 = FrameNode::new(1, [2.0, 0.0, 0.0]).unwrap();
        let node_2 = FrameNode::new(2, [4.0, 0.0, 0.0]).unwrap();
        let element_01 = FrameElement::new(node_0, node_1, section, [0.0, 1.0, 0.0]).unwrap();
        let element_12 = FrameElement::new(node_1, node_2, section, [0.0, 1.0, 0.0]).unwrap();

        let global = assemble_global_stiffness(3, &[element_01, element_12]).unwrap();

        assert_close(global[DOF_PER_NODE + UX][DOF_PER_NODE + UX], 2000.0);
        assert_close(global[DOF_PER_NODE + UY][DOF_PER_NODE + UY], 3000.0);
        assert_close(global[DOF_PER_NODE + RX][DOF_PER_NODE + RX], 400.0);
    }

    #[test]
    fn assembly_uses_stable_dof_map_for_nonzero_node_indices() {
        let section = FrameSection::new(1000.0, 400.0, 2.0, 1.0, 1.0, 1.0).unwrap();
        let node_i = FrameNode::new(2, [0.0, 0.0, 0.0]).unwrap();
        let node_j = FrameNode::new(0, [2.0, 0.0, 0.0]).unwrap();
        let element = FrameElement::new(node_i, node_j, section, [0.0, 1.0, 0.0]).unwrap();

        let global = assemble_global_stiffness(3, &[element]).unwrap();

        let node_i_ux = node_dof_index(2, FrameDof::Ux);
        let node_j_ux = node_dof_index(0, FrameDof::Ux);
        assert_close(global[node_i_ux][node_i_ux], 1000.0);
        assert_close(global[node_j_ux][node_j_ux], 1000.0);
        assert_close(global[node_i_ux][node_j_ux], -1000.0);
        assert_close(global[node_j_ux][node_i_ux], -1000.0);
    }

    #[test]
    fn reduction_removes_restrained_dofs() {
        let stiffness = vec![
            vec![10.0, 1.0, 2.0, 3.0],
            vec![1.0, 20.0, 4.0, 5.0],
            vec![2.0, 4.0, 30.0, 6.0],
            vec![3.0, 5.0, 6.0, 40.0],
        ];
        let force = vec![7.0, 8.0, 9.0, 10.0];

        let reduced = reduce_system(&stiffness, &force, &[1, 3]).unwrap();

        assert_eq!(reduced.free_dofs, vec![0, 2]);
        assert_eq!(reduced.force, vec![7.0, 9.0]);
        assert_eq!(reduced.stiffness, vec![vec![10.0, 2.0], vec![2.0, 30.0]]);
    }

    #[test]
    fn prescribed_reduction_uses_boundary_dof_order_for_force_adjustment() {
        let stiffness = vec![
            vec![10.0, 2.0, 3.0],
            vec![2.0, 20.0, 4.0],
            vec![3.0, 4.0, 30.0],
        ];
        let force = vec![100.0, 200.0, 300.0];

        let reduced =
            reduce_system_with_prescribed_displacements(&stiffness, &force, &[2, 0], &[5.0, 1.0])
                .unwrap();

        assert_eq!(reduced.free_dofs, vec![1]);
        assert_eq!(reduced.stiffness, vec![vec![20.0]]);
        assert_close(reduced.force[0], 178.0);
    }

    #[test]
    fn prescribed_displacement_reduction_adjusts_free_forces() {
        let stiffness = vec![
            vec![10.0, 2.0, 3.0],
            vec![2.0, 20.0, 4.0],
            vec![3.0, 4.0, 30.0],
        ];
        let force = vec![100.0, 200.0, 300.0];

        let reduced =
            reduce_system_with_prescribed_displacements(&stiffness, &force, &[1], &[5.0]).unwrap();

        assert_eq!(reduced.free_dofs, vec![0, 2]);
        assert_eq!(reduced.stiffness, vec![vec![10.0, 3.0], vec![3.0, 30.0]]);
        assert_close(reduced.force[0], 90.0);
        assert_close(reduced.force[1], 280.0);
    }

    #[test]
    fn zero_prescribed_displacements_match_legacy_reduction() {
        let stiffness = vec![
            vec![10.0, 1.0, 2.0, 3.0],
            vec![1.0, 20.0, 4.0, 5.0],
            vec![2.0, 4.0, 30.0, 6.0],
            vec![3.0, 5.0, 6.0, 40.0],
        ];
        let force = vec![7.0, 8.0, 9.0, 10.0];

        let legacy = reduce_system(&stiffness, &force, &[1, 3]).unwrap();
        let prescribed =
            reduce_system_with_prescribed_displacements(&stiffness, &force, &[1, 3], &[0.0, 0.0])
                .unwrap();

        assert_eq!(prescribed, legacy);
    }

    #[test]
    fn prescribed_reduction_rejects_repeated_and_out_of_range_dofs() {
        let stiffness = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let force = vec![1.0, 2.0];

        let repeated_error =
            reduce_system_with_prescribed_displacements(&stiffness, &force, &[0, 0], &[0.0, 1.0])
                .unwrap_err();
        assert_eq!(
            repeated_error,
            FrameKernelError::RepeatedPrescribedDof { dof: 0 }
        );

        let out_of_range_error =
            reduce_system_with_prescribed_displacements(&stiffness, &force, &[2], &[0.0])
                .unwrap_err();
        assert_eq!(
            out_of_range_error,
            FrameKernelError::PrescribedDofOutOfRange {
                dof: 2,
                total_dofs: 2
            }
        );
    }

    #[test]
    fn prescribed_reduction_rejects_invalid_displacement_vector() {
        let stiffness = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let force = vec![1.0, 2.0];

        let length_error =
            reduce_system_with_prescribed_displacements(&stiffness, &force, &[0], &[]).unwrap_err();
        assert_eq!(
            length_error,
            FrameKernelError::InvalidVectorLength {
                expected: 1,
                actual: 0
            }
        );

        let nonfinite_error =
            reduce_system_with_prescribed_displacements(&stiffness, &force, &[0], &[f64::INFINITY])
                .unwrap_err();
        assert_eq!(
            nonfinite_error,
            FrameKernelError::NonFiniteInput {
                name: "prescribed displacement",
                value: f64::INFINITY
            }
        );
    }

    #[test]
    fn boundary_reduction_rejects_out_of_range_restraint() {
        let stiffness = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let force = vec![1.0, 2.0];

        let error = reduce_system(&stiffness, &force, &[2]).unwrap_err();

        assert_eq!(
            error,
            FrameKernelError::RestrainedDofOutOfRange {
                dof: 2,
                total_dofs: 2
            }
        );
    }

    #[test]
    fn boundary_reduction_rejects_force_length_mismatch() {
        let stiffness = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let force = vec![1.0];

        let error = reduce_system(&stiffness, &force, &[]).unwrap_err();

        assert_eq!(
            error,
            FrameKernelError::InvalidVectorLength {
                expected: 2,
                actual: 1
            }
        );
    }

    #[test]
    fn simple_axial_bar_solve_uses_reduced_system() {
        let section = FrameSection::new(1000.0, 400.0, 2.0, 1.0, 1.0, 1.0).unwrap();
        let node_i = FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap();
        let node_j = FrameNode::new(1, [2.0, 0.0, 0.0]).unwrap();
        let element = FrameElement::new(node_i, node_j, section, [0.0, 1.0, 0.0]).unwrap();
        let stiffness = assemble_global_stiffness(2, &[element]).unwrap();
        let mut force = vec![0.0; ELEMENT_DOF];
        force[UX + DOF_PER_NODE] = 100.0;
        let restrained: Vec<usize> = (0..ELEMENT_DOF)
            .filter(|&dof| dof != UX + DOF_PER_NODE)
            .collect();

        let reduced = reduce_system(&stiffness, &force, &restrained).unwrap();
        let displacement = solve_dense(&reduced.stiffness, &reduced.force).unwrap();

        assert_eq!(reduced.free_dofs, vec![UX + DOF_PER_NODE]);
        assert_close(displacement[0], 0.1);
    }

    #[test]
    fn dense_solve_uses_partial_pivoting() {
        let stiffness = vec![vec![0.0, 2.0], vec![1.0, 3.0]];
        let force = vec![4.0, 7.0];

        let solution = solve_dense(&stiffness, &force).unwrap();

        assert_close(solution[0], 1.0);
        assert_close(solution[1], 2.0);
    }

    #[test]
    fn singular_system_is_detected() {
        let stiffness = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        let force = vec![3.0, 6.0];

        let error = solve_dense(&stiffness, &force).unwrap_err();

        assert_eq!(error, FrameKernelError::SingularSystem { pivot: 1 });
    }

    #[test]
    fn zero_pivot_singular_system_is_detected() {
        let stiffness = vec![vec![0.0, 0.0], vec![0.0, 1.0]];
        let force = vec![0.0, 1.0];

        let error = solve_dense(&stiffness, &force).unwrap_err();

        assert_eq!(error, FrameKernelError::SingularSystem { pivot: 0 });
    }

    #[test]
    fn dense_solve_uses_internal_zero_pivot_guard() {
        let stiffness = vec![vec![DENSE_SOLVE_ZERO_PIVOT_GUARD * 0.5]];
        let force = vec![1.0];

        let error = solve_dense(&stiffness, &force).unwrap_err();

        assert_eq!(error, FrameKernelError::SingularSystem { pivot: 0 });
    }

    #[test]
    fn degenerate_orientation_is_rejected() {
        let error = FrameOrientation::from_x_axis_and_y_reference([1.0, 0.0, 0.0], [2.0, 0.0, 0.0])
            .unwrap_err();

        assert_eq!(
            error,
            FrameKernelError::DegenerateAxis {
                detail: "y reference parallel to local x axis"
            }
        );
    }

    #[test]
    fn repeated_element_node_index_is_rejected() {
        let section = FrameSection::new(1000.0, 400.0, 2.0, 1.0, 1.0, 1.0).unwrap();
        let node_i = FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap();
        let node_j = FrameNode::new(0, [2.0, 0.0, 0.0]).unwrap();

        let error = FrameElement::new(node_i, node_j, section, [0.0, 1.0, 0.0]).unwrap_err();

        assert_eq!(
            error,
            FrameKernelError::RepeatedElementNodeIndex { node_index: 0 }
        );
    }

    #[test]
    fn assembly_rejects_node_index_outside_model() {
        let section = FrameSection::new(1000.0, 400.0, 2.0, 1.0, 1.0, 1.0).unwrap();
        let node_i = FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap();
        let node_j = FrameNode::new(2, [2.0, 0.0, 0.0]).unwrap();
        let element = FrameElement::new(node_i, node_j, section, [0.0, 1.0, 0.0]).unwrap();

        let error = assemble_global_stiffness(2, &[element]).unwrap_err();

        assert_eq!(
            error,
            FrameKernelError::InvalidNodeIndex {
                node_index: 2,
                node_count: 2
            }
        );
    }

    #[test]
    fn boundary_reduction_rejects_repeated_restraint() {
        let stiffness = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let force = vec![1.0, 2.0];

        let error = reduce_system(&stiffness, &force, &[0, 0]).unwrap_err();

        assert_eq!(error, FrameKernelError::RepeatedRestrainedDof { dof: 0 });
    }

    #[test]
    fn dense_operations_reject_invalid_numeric_shapes_and_values() {
        let nonsquare_error = solve_dense(&[vec![1.0, 0.0], vec![0.0]], &[1.0, 2.0]).unwrap_err();
        assert_eq!(
            nonsquare_error,
            FrameKernelError::InvalidMatrixDimensions { rows: 2, cols: 1 }
        );

        let nonfinite_error = reduce_system(
            &[vec![1.0, f64::INFINITY], vec![0.0, 1.0]],
            &[1.0, 2.0],
            &[],
        )
        .unwrap_err();
        assert_eq!(
            nonfinite_error,
            FrameKernelError::NonFiniteInput {
                name: "matrix/vector entry",
                value: f64::INFINITY
            }
        );
    }

    #[test]
    fn non_finite_input_is_rejected() {
        let error = FrameSection::new(f64::NAN, 400.0, 2.0, 1.0, 1.0, 1.0).unwrap_err();

        match error {
            FrameKernelError::NonFiniteInput { name, value } => {
                assert_eq!(name, "elastic_modulus");
                assert!(value.is_nan());
            }
            other => panic!("expected non-finite input error, got {other:?}"),
        }
    }

    #[test]
    fn non_positive_section_and_length_inputs_are_rejected() {
        let section_error = FrameSection::new(1000.0, 400.0, 0.0, 1.0, 1.0, 1.0).unwrap_err();
        assert_eq!(
            section_error,
            FrameKernelError::NonPositiveInput {
                name: "area",
                value: 0.0
            }
        );

        let section = FrameSection::new(1000.0, 400.0, 2.0, 1.0, 1.0, 1.0).unwrap();
        let length_error = FrameProperties::new(section, -1.0).unwrap_err();
        assert_eq!(
            length_error,
            FrameKernelError::NonPositiveInput {
                name: "length",
                value: -1.0
            }
        );
    }

    #[test]
    fn frame_unit_basis_uses_accepted_canonical_dimensions() {
        let basis = FrameKernelUnitBasis::from_unit_ids(
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

        assert!(basis.has_expected_dimensions());
        assert_eq!(basis.coordinate_unit.dimension_id(), "length");
        assert_eq!(
            basis.second_moment_area_unit.dimension_id(),
            "second_moment_area"
        );
        assert_eq!(basis.rotation_unit.dimension_id(), "rotation");
    }

    #[test]
    fn frame_unit_basis_rejects_wrong_dimensions() {
        let basis = FrameKernelUnitBasis::new(
            UnitSystemRef::new("fixture-unit-system").unwrap(),
            QuantityUnitMetadata::new("fixture-force", CanonicalDimension::Force).unwrap(),
            QuantityUnitMetadata::new("fixture-force", CanonicalDimension::Force).unwrap(),
            QuantityUnitMetadata::new("fixture-moment", CanonicalDimension::Moment).unwrap(),
            QuantityUnitMetadata::new("fixture-stress", CanonicalDimension::Stress).unwrap(),
            QuantityUnitMetadata::new("fixture-area", CanonicalDimension::Area).unwrap(),
            QuantityUnitMetadata::new(
                "fixture-second-moment-area",
                CanonicalDimension::SecondMomentArea,
            )
            .unwrap(),
            QuantityUnitMetadata::new("fixture-displacement", CanonicalDimension::Displacement)
                .unwrap(),
            QuantityUnitMetadata::new("fixture-rotation", CanonicalDimension::Rotation).unwrap(),
        );

        assert!(basis.is_none());
    }

    #[test]
    fn boundary_metadata_trims_ids_and_rejects_tbd_placeholders() {
        let unit_ref = UnitSystemRef::new(" fixture-unit-system ").unwrap();
        assert_eq!(unit_ref.unit_system_id, "fixture-unit-system");
        assert!(UnitSystemRef::new(" tbd ").is_none());

        let unit =
            QuantityUnitMetadata::new(" fixture-length ", CanonicalDimension::Length).unwrap();
        assert_eq!(unit.unit_id, "fixture-length");
        assert!(QuantityUnitMetadata::new("TBD", CanonicalDimension::Length).is_none());

        let model_ref = CanonicalModelReference::new(
            " fixture-model ",
            CanonicalModelRole::AnalyticalSolverModel,
            " fixture-node-1 ",
        )
        .unwrap();
        assert_eq!(model_ref.model_id, "fixture-model");
        assert_eq!(model_ref.object_ref, "fixture-node-1");
        assert!(CanonicalModelReference::new(
            "fixture-model",
            CanonicalModelRole::AnalyticalSolverModel,
            "TBD"
        )
        .is_none());
    }

    #[test]
    fn canonical_dimension_exposes_force_per_length() {
        let unit = QuantityUnitMetadata::new(
            "fixture-force-per-length",
            CanonicalDimension::ForcePerLength,
        )
        .unwrap();

        assert_eq!(
            CanonicalDimension::ForcePerLength.as_str(),
            "force_per_length"
        );
        assert_eq!(unit.dimension_id(), "force_per_length");
    }

    #[test]
    fn boundary_metadata_rejects_missing_unit_and_model_refs() {
        assert!(FrameKernelUnitBasis::from_unit_ids(
            "TBD",
            "fixture-length",
            "fixture-force",
            "fixture-moment",
            "fixture-stress",
            "fixture-area",
            "fixture-second-moment-area",
            "fixture-displacement",
            "fixture-rotation",
        )
        .is_none());
        assert!(CanonicalModelReference::new(
            "model-1",
            CanonicalModelRole::AnalyticalSolverModel,
            ""
        )
        .is_none());
        assert_eq!(
            CanonicalModelRole::PhysicalSourceOfTruth.as_str(),
            "physical_source_of_truth"
        );
    }

    fn assert_symmetric_12(matrix: &Matrix12) {
        for (row, row_values) in matrix.iter().enumerate() {
            for (col, value) in row_values.iter().enumerate() {
                assert_close(*value, matrix[col][row]);
            }
        }
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= ASSERT_TOLERANCE,
            "expected {expected}, got {actual}"
        );
    }
}
