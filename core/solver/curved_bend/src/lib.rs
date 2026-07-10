//! Code-neutral curved-bend macro-element mechanics.
//!
//! This crate forms the 12x12 stiffness of a circular-arc bend from a
//! closed-form unit-load (Castigliano) end-flexibility integration. The
//! in-plane and out-of-plane flexibility factors are user-entered opaque
//! numbers. This crate contains open mechanics routines only. It does not
//! encode design code compliance checks, protected standards content,
//! flexibility-factor or stress-intensification formulas, or private
//! project data.

use std::error::Error;
use std::f64::consts::PI;
use std::fmt;

use open_pipe_stress_frame_kernel::{
    solve_dense, transform_global_stiffness, FrameKernelError, FrameNode, FrameOrientation,
    Matrix12, DOF_PER_NODE, ELEMENT_DOF,
};

/// Node-level 6x6 matrix in the frame-kernel DOF order [ux, uy, uz, rx, ry, rz].
pub type Matrix6 = [[f64; DOF_PER_NODE]; DOF_PER_NODE];

const AXIS_TOLERANCE: f64 = 1.0e-12;
// Relative agreement demanded between |node_i - center| and |node_j - center|.
const RADIUS_MATCH_TOLERANCE: f64 = 1.0e-9;
// Included-angle admissibility window (radians): the open interval (0, pi).
const MIN_INCLUDED_ANGLE: f64 = 1.0e-9;

#[derive(Debug, Clone, PartialEq)]
pub enum CurvedBendError {
    Kernel(FrameKernelError),
    RadiusMismatch { radius_i: f64, radius_j: f64 },
    DegenerateArc { detail: &'static str },
    IncludedAngleOutOfRange { included_angle: f64 },
}

impl fmt::Display for CurvedBendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Kernel(error) => write!(f, "frame kernel error: {error}"),
            Self::RadiusMismatch { radius_i, radius_j } => write!(
                f,
                "arc center must be equidistant from both end nodes, got radii {radius_i} and {radius_j}"
            ),
            Self::DegenerateArc { detail } => write!(f, "degenerate arc geometry: {detail}"),
            Self::IncludedAngleOutOfRange { included_angle } => write!(
                f,
                "arc included angle must lie strictly between 0 and pi radians, got {included_angle}"
            ),
        }
    }
}

impl Error for CurvedBendError {}

impl From<FrameKernelError> for CurvedBendError {
    fn from(error: FrameKernelError) -> Self {
        Self::Kernel(error)
    }
}

/// Derived circular-arc geometry in the element's local bend-plane frame.
///
/// Local axes (rows of `local_axes`, expressed in global coordinates):
/// x is the radial direction from the arc center to node `i`, z is the
/// bend-plane normal `(i - center) x (j - center)` normalized, and
/// y = z cross x completes the right-handed basis. Node `i` sits at arc
/// angle 0 and node `j` at `included_angle`, measured about local z.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArcGeometry {
    pub radius: f64,
    pub included_angle: f64,
    pub local_axes: [[f64; 3]; 3],
}

/// Two-node circular-arc bend macro-element.
///
/// The element carries user-entered in-plane and out-of-plane bending
/// flexibility factors. Factors equal to 1 reproduce the plain
/// Euler-Bernoulli curved beam (bending, torsion, and axial strain energy;
/// shear deformation excluded, consistent with the frame kernel).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurvedBendMacroElement {
    pub node_i: FrameNode,
    pub node_j: FrameNode,
    pub center: [f64; 3],
    pub elastic_modulus: f64,
    pub shear_modulus: f64,
    pub area: f64,
    /// Cross-section second moment of area, identical about both bending axes.
    pub second_moment: f64,
    pub torsion_constant: f64,
    /// User-entered in-plane bending flexibility factor (opaque number).
    pub in_plane_flexibility_factor: f64,
    /// User-entered out-of-plane bending flexibility factor (opaque number).
    pub out_of_plane_flexibility_factor: f64,
}

impl CurvedBendMacroElement {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        node_i: FrameNode,
        node_j: FrameNode,
        center: [f64; 3],
        elastic_modulus: f64,
        shear_modulus: f64,
        area: f64,
        second_moment: f64,
        torsion_constant: f64,
        in_plane_flexibility_factor: f64,
        out_of_plane_flexibility_factor: f64,
    ) -> Result<Self, CurvedBendError> {
        if node_i.index == node_j.index {
            return Err(FrameKernelError::RepeatedElementNodeIndex {
                node_index: node_i.index,
            }
            .into());
        }
        validate_finite_vector("center", center)?;
        validate_positive_finite("elastic_modulus", elastic_modulus)?;
        validate_positive_finite("shear_modulus", shear_modulus)?;
        validate_positive_finite("area", area)?;
        validate_positive_finite("second_moment", second_moment)?;
        validate_positive_finite("torsion_constant", torsion_constant)?;
        validate_positive_finite("in_plane_flexibility_factor", in_plane_flexibility_factor)?;
        validate_positive_finite(
            "out_of_plane_flexibility_factor",
            out_of_plane_flexibility_factor,
        )?;

        let element = Self {
            node_i,
            node_j,
            center,
            elastic_modulus,
            shear_modulus,
            area,
            second_moment,
            torsion_constant,
            in_plane_flexibility_factor,
            out_of_plane_flexibility_factor,
        };
        element.geometry()?;
        element.orientation()?;
        Ok(element)
    }

    /// Validated circular-arc geometry derived from the end nodes and center.
    pub fn geometry(&self) -> Result<ArcGeometry, CurvedBendError> {
        let radial_i = subtract(self.node_i.coordinates, self.center);
        let radial_j = subtract(self.node_j.coordinates, self.center);
        let radius_i = norm(radial_i);
        let radius_j = norm(radial_j);
        if radius_i <= AXIS_TOLERANCE || radius_j <= AXIS_TOLERANCE {
            return Err(CurvedBendError::DegenerateArc {
                detail: "arc end node coincides with the arc center",
            });
        }
        if (radius_i - radius_j).abs() > RADIUS_MATCH_TOLERANCE * radius_i.max(radius_j) {
            return Err(CurvedBendError::RadiusMismatch { radius_i, radius_j });
        }
        let radius = 0.5 * (radius_i + radius_j);

        let plane_normal = cross(radial_i, radial_j);
        let included_angle = norm(plane_normal).atan2(dot(radial_i, radial_j));
        if !(MIN_INCLUDED_ANGLE..=PI - MIN_INCLUDED_ANGLE).contains(&included_angle) {
            return Err(CurvedBendError::IncludedAngleOutOfRange { included_angle });
        }

        let x_axis = normalize(radial_i, "arc radial axis at node i")?;
        let z_axis = normalize(plane_normal, "arc bend-plane normal")?;
        let y_axis = cross(z_axis, x_axis);
        Ok(ArcGeometry {
            radius,
            included_angle,
            local_axes: [x_axis, y_axis, z_axis],
        })
    }

    pub fn radius(&self) -> Result<f64, CurvedBendError> {
        Ok(self.geometry()?.radius)
    }

    pub fn included_angle(&self) -> Result<f64, CurvedBendError> {
        Ok(self.geometry()?.included_angle)
    }

    pub fn arc_length(&self) -> Result<f64, CurvedBendError> {
        let geometry = self.geometry()?;
        Ok(geometry.radius * geometry.included_angle)
    }

    /// Local element frame: x radial at node `i`, z bend-plane normal.
    pub fn orientation(&self) -> Result<FrameOrientation, CurvedBendError> {
        let geometry = self.geometry()?;
        Ok(FrameOrientation::new(geometry.local_axes)?)
    }

    /// End-flexibility matrix at node `j` with node `i` fixed, in the local
    /// frame, DOF order [ux, uy, uz, rx, ry, rz].
    ///
    /// Entries are exact closed-form unit-load integrals over the arc of the
    /// bending, torsion, and axial strain-energy products. The user factors
    /// scale only the in-plane and out-of-plane bending-curvature terms.
    pub fn end_flexibility(&self) -> Result<Matrix6, CurvedBendError> {
        let geometry = self.geometry()?;
        let gram = trig_gram(geometry.included_angle);
        let cases = unit_load_actions(geometry.radius, geometry.included_angle);
        let bending_rigidity = self.elastic_modulus * self.second_moment;
        let torsion_rigidity = self.shear_modulus * self.torsion_constant;
        let axial_rigidity = self.elastic_modulus * self.area;

        let mut flexibility = [[0.0; DOF_PER_NODE]; DOF_PER_NODE];
        for row in 0..DOF_PER_NODE {
            for col in row..DOF_PER_NODE {
                let value = geometry.radius
                    * (self.in_plane_flexibility_factor
                        * quad(
                            &gram,
                            cases[row].in_plane_moment,
                            cases[col].in_plane_moment,
                        )
                        / bending_rigidity
                        + self.out_of_plane_flexibility_factor
                            * quad(
                                &gram,
                                cases[row].out_of_plane_moment,
                                cases[col].out_of_plane_moment,
                            )
                            / bending_rigidity
                        + quad(&gram, cases[row].torsion, cases[col].torsion) / torsion_rigidity
                        + quad(&gram, cases[row].axial, cases[col].axial) / axial_rigidity);
                flexibility[row][col] = value;
                flexibility[col][row] = value;
            }
        }
        Ok(flexibility)
    }

    /// 12x12 stiffness in the local frame, DOF order [node i; node j].
    pub fn local_stiffness(&self) -> Result<Matrix12, CurvedBendError> {
        let geometry = self.geometry()?;
        let flexibility = self.end_flexibility()?;
        let tip_stiffness = invert_symmetric6(&flexibility)?;
        let chord = [
            geometry.radius * (geometry.included_angle.cos() - 1.0),
            geometry.radius * geometry.included_angle.sin(),
            0.0,
        ];
        Ok(assemble_macro_stiffness(&tip_stiffness, chord))
    }

    /// 12x12 stiffness in global coordinates.
    pub fn global_stiffness(&self) -> Result<Matrix12, CurvedBendError> {
        let local = self.local_stiffness()?;
        let orientation = self.orientation()?;
        Ok(transform_global_stiffness(&local, &orientation))
    }
}

// Coefficients of {1, cos(theta), sin(theta)} for an internal action along
// the arc parameter theta in [0, included_angle].
type TrigSeries = [f64; 3];

struct UnitLoadActions {
    in_plane_moment: TrigSeries,
    out_of_plane_moment: TrigSeries,
    torsion: TrigSeries,
    axial: TrigSeries,
}

// Internal actions at arc angle theta for the six unit loads applied at
// node j, from segment equilibrium of the arc between theta and j. The
// section point is p(theta) = center + R(cos theta, sin theta, 0), the
// section axes are radial r = (cos, sin, 0), tangent t = (-sin, cos, 0),
// and the bend-plane normal z. For a unit force f the section moment is
// (p(phi) - p(theta)) x f; for a unit moment it is the moment itself.
// In-plane bending is the z moment component, out-of-plane bending the r
// component, torsion the t component, and axial force is f dot t.
fn unit_load_actions(radius: f64, included_angle: f64) -> [UnitLoadActions; 6] {
    let sin_end = included_angle.sin();
    let cos_end = included_angle.cos();
    let zero: TrigSeries = [0.0; 3];
    [
        // Unit force along local x.
        UnitLoadActions {
            in_plane_moment: [-radius * sin_end, 0.0, radius],
            out_of_plane_moment: zero,
            torsion: zero,
            axial: [0.0, 0.0, -1.0],
        },
        // Unit force along local y.
        UnitLoadActions {
            in_plane_moment: [radius * cos_end, -radius, 0.0],
            out_of_plane_moment: zero,
            torsion: zero,
            axial: [0.0, 1.0, 0.0],
        },
        // Unit force along local z: out-of-plane bending R sin(phi - theta)
        // and torsion R (1 - cos(phi - theta)) couple through the arc.
        UnitLoadActions {
            in_plane_moment: zero,
            out_of_plane_moment: [0.0, radius * sin_end, -radius * cos_end],
            torsion: [radius, -radius * cos_end, -radius * sin_end],
            axial: zero,
        },
        // Unit moment about local x.
        UnitLoadActions {
            in_plane_moment: zero,
            out_of_plane_moment: [0.0, 1.0, 0.0],
            torsion: [0.0, 0.0, -1.0],
            axial: zero,
        },
        // Unit moment about local y.
        UnitLoadActions {
            in_plane_moment: zero,
            out_of_plane_moment: [0.0, 0.0, 1.0],
            torsion: [0.0, 1.0, 0.0],
            axial: zero,
        },
        // Unit moment about local z.
        UnitLoadActions {
            in_plane_moment: [1.0, 0.0, 0.0],
            out_of_plane_moment: zero,
            torsion: zero,
            axial: zero,
        },
    ]
}

// Exact integrals over [0, phi] of pairwise products of {1, cos, sin}.
fn trig_gram(included_angle: f64) -> [[f64; 3]; 3] {
    let sin_end = included_angle.sin();
    let cos_end = included_angle.cos();
    let sin_double = (2.0 * included_angle).sin();
    [
        [included_angle, sin_end, 1.0 - cos_end],
        [
            sin_end,
            0.5 * included_angle + 0.25 * sin_double,
            0.5 * sin_end * sin_end,
        ],
        [
            1.0 - cos_end,
            0.5 * sin_end * sin_end,
            0.5 * included_angle - 0.25 * sin_double,
        ],
    ]
}

fn quad(gram: &[[f64; 3]; 3], left: TrigSeries, right: TrigSeries) -> f64 {
    let mut sum = 0.0;
    for row in 0..3 {
        for col in 0..3 {
            sum += left[row] * gram[row][col] * right[col];
        }
    }
    sum
}

// Inverse of a symmetric positive-definite 6x6 via the frame-kernel dense
// solver, one unit column at a time in fixed order, then symmetrized.
fn invert_symmetric6(matrix: &Matrix6) -> Result<Matrix6, CurvedBendError> {
    let dense: Vec<Vec<f64>> = matrix.iter().map(|row| row.to_vec()).collect();
    let mut inverse = [[0.0; DOF_PER_NODE]; DOF_PER_NODE];
    for col in 0..DOF_PER_NODE {
        let mut rhs = vec![0.0; DOF_PER_NODE];
        rhs[col] = 1.0;
        let solution = solve_dense(&dense, &rhs)?;
        for (row, value) in solution.iter().enumerate() {
            inverse[row][col] = *value;
        }
    }
    // The symmetric pair update touches both (row, col) and (col, row).
    #[allow(clippy::needless_range_loop)]
    for row in 0..DOF_PER_NODE {
        for col in (row + 1)..DOF_PER_NODE {
            let average = 0.5 * (inverse[row][col] + inverse[col][row]);
            inverse[row][col] = average;
            inverse[col][row] = average;
        }
    }
    Ok(inverse)
}

// Rigid equilibrium transfer H from node j loads to node i reactions:
// f_i = -H f_j with H = [[I, 0], [skew(chord), I]], chord = x_j - x_i.
fn equilibrium_transfer(chord: [f64; 3]) -> Matrix6 {
    let mut transfer = [[0.0; DOF_PER_NODE]; DOF_PER_NODE];
    for (diagonal, row) in transfer.iter_mut().enumerate() {
        row[diagonal] = 1.0;
    }
    transfer[3][1] = -chord[2];
    transfer[3][2] = chord[1];
    transfer[4][0] = chord[2];
    transfer[4][2] = -chord[0];
    transfer[5][0] = -chord[1];
    transfer[5][1] = chord[0];
    transfer
}

// K = [[H K_jj H^T, -H K_jj], [-K_jj H^T, K_jj]].
fn assemble_macro_stiffness(tip_stiffness: &Matrix6, chord: [f64; 3]) -> Matrix12 {
    let transfer = equilibrium_transfer(chord);
    let coupled = multiply6(&transfer, tip_stiffness);
    let anchored = multiply6_transpose_right(&coupled, &transfer);
    let mut stiffness = [[0.0; ELEMENT_DOF]; ELEMENT_DOF];
    for row in 0..DOF_PER_NODE {
        for col in 0..DOF_PER_NODE {
            stiffness[row][col] = anchored[row][col];
            stiffness[row][col + DOF_PER_NODE] = -coupled[row][col];
            stiffness[row + DOF_PER_NODE][col] = -coupled[col][row];
            stiffness[row + DOF_PER_NODE][col + DOF_PER_NODE] = tip_stiffness[row][col];
        }
    }
    stiffness
}

fn multiply6(left: &Matrix6, right: &Matrix6) -> Matrix6 {
    let mut result = [[0.0; DOF_PER_NODE]; DOF_PER_NODE];
    for row in 0..DOF_PER_NODE {
        for col in 0..DOF_PER_NODE {
            for inner in 0..DOF_PER_NODE {
                result[row][col] += left[row][inner] * right[inner][col];
            }
        }
    }
    result
}

fn multiply6_transpose_right(left: &Matrix6, right: &Matrix6) -> Matrix6 {
    let mut result = [[0.0; DOF_PER_NODE]; DOF_PER_NODE];
    for row in 0..DOF_PER_NODE {
        for col in 0..DOF_PER_NODE {
            for inner in 0..DOF_PER_NODE {
                result[row][col] += left[row][inner] * right[col][inner];
            }
        }
    }
    result
}

fn validate_positive_finite(name: &'static str, value: f64) -> Result<(), CurvedBendError> {
    if !value.is_finite() {
        return Err(FrameKernelError::NonFiniteInput { name, value }.into());
    }
    if value <= 0.0 {
        return Err(FrameKernelError::NonPositiveInput { name, value }.into());
    }
    Ok(())
}

fn validate_finite_vector(name: &'static str, vector: [f64; 3]) -> Result<(), CurvedBendError> {
    for value in vector {
        if !value.is_finite() {
            return Err(FrameKernelError::NonFiniteInput { name, value }.into());
        }
    }
    Ok(())
}

fn normalize(vector: [f64; 3], detail: &'static str) -> Result<[f64; 3], CurvedBendError> {
    let magnitude = norm(vector);
    if magnitude <= AXIS_TOLERANCE {
        return Err(CurvedBendError::DegenerateArc { detail });
    }
    Ok([
        vector[0] / magnitude,
        vector[1] / magnitude,
        vector[2] / magnitude,
    ])
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

fn cross(left: [f64; 3], right: [f64; 3]) -> [f64; 3] {
    [
        left[1] * right[2] - left[2] * right[1],
        left[2] * right[0] - left[0] * right[2],
        left[0] * right[1] - left[1] * right[0],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_pipe_stress_frame_kernel::{reduce_system, FrameElement, FrameSection};

    const ASSERT_TOLERANCE: f64 = 1.0e-9;

    const ELASTIC_MODULUS: f64 = 100.0;
    const SHEAR_MODULUS: f64 = 40.0;
    const AREA: f64 = 3.0;
    const SECOND_MOMENT: f64 = 5.0;
    const TORSION_CONSTANT: f64 = 7.0;
    const RADIUS: f64 = 2.0;

    fn assert_close(actual: f64, expected: f64) {
        assert_close_tolerance(actual, expected, ASSERT_TOLERANCE);
    }

    fn assert_close_tolerance(actual: f64, expected: f64, tolerance: f64) {
        let scale = expected.abs().max(1.0);
        assert!(
            (actual - expected).abs() <= tolerance * scale,
            "actual {actual} differs from expected {expected} beyond tolerance {tolerance}"
        );
    }

    // Quarter circle in the global x-y plane centered at the origin so the
    // local frame coincides with the global frame: node i at (R, 0, 0),
    // node j at (0, R, 0), bend-plane normal +z.
    fn quarter_circle_element(k_in: f64, k_out: f64) -> CurvedBendMacroElement {
        let node_i = FrameNode::new(0, [RADIUS, 0.0, 0.0]).unwrap();
        let node_j = FrameNode::new(1, [0.0, RADIUS, 0.0]).unwrap();
        CurvedBendMacroElement::new(
            node_i,
            node_j,
            [0.0, 0.0, 0.0],
            ELASTIC_MODULUS,
            SHEAR_MODULUS,
            AREA,
            SECOND_MOMENT,
            TORSION_CONSTANT,
            k_in,
            k_out,
        )
        .unwrap()
    }

    // Cantilever tip response: fix all node-i DOFs of the assembled 12x12
    // global stiffness, apply a unit load component at node j, and solve
    // with the frame-kernel reduction and dense solver.
    fn tip_response(element: &CurvedBendMacroElement, loaded_dof: usize) -> [f64; DOF_PER_NODE] {
        let stiffness = element.global_stiffness().unwrap();
        let dense: Vec<Vec<f64>> = stiffness.iter().map(|row| row.to_vec()).collect();
        let mut force = vec![0.0; ELEMENT_DOF];
        force[DOF_PER_NODE + loaded_dof] = 1.0;
        let restrained: Vec<usize> = (0..DOF_PER_NODE).collect();
        let reduced = reduce_system(&dense, &force, &restrained).unwrap();
        let solution = solve_dense(&reduced.stiffness, &reduced.force).unwrap();
        let mut response = [0.0; DOF_PER_NODE];
        response.copy_from_slice(&solution);
        response
    }

    fn matrix12_max_abs(matrix: &Matrix12) -> f64 {
        matrix
            .iter()
            .flatten()
            .fold(0.0_f64, |max, value| max.max(value.abs()))
    }

    #[test]
    fn geometry_accessors_report_quarter_circle_arc() {
        let element = quarter_circle_element(1.0, 1.0);
        assert_close(element.radius().unwrap(), RADIUS);
        assert_close(element.included_angle().unwrap(), PI / 2.0);
        assert_close(element.arc_length().unwrap(), RADIUS * PI / 2.0);

        let orientation = element.orientation().unwrap();
        for axis in 0..3 {
            for component in 0..3 {
                let expected = if axis == component { 1.0 } else { 0.0 };
                assert_close(orientation.local_axes[axis][component], expected);
            }
        }
    }

    // Straight-limit convergence: a shallow arc over a fixed chord must
    // approach the frame-kernel straight element of the same chord. The
    // difference is O(included_angle), so this test uses a documented
    // looser tolerance and additionally checks first-order convergence.
    #[test]
    fn straight_limit_matches_frame_kernel_straight_element() {
        let chord_length = 2.0;
        let section = FrameSection::new(
            ELASTIC_MODULUS,
            SHEAR_MODULUS,
            AREA,
            SECOND_MOMENT,
            SECOND_MOMENT,
            TORSION_CONSTANT,
        )
        .unwrap();
        let node_i = FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap();
        let node_j = FrameNode::new(1, [chord_length, 0.0, 0.0]).unwrap();
        let straight = FrameElement::new(node_i, node_j, section, [0.0, 1.0, 0.0])
            .unwrap()
            .global_stiffness()
            .unwrap();
        let straight_scale = matrix12_max_abs(&straight);

        let relative_error = |included_angle: f64| -> f64 {
            let radius = chord_length / (2.0 * (included_angle / 2.0).sin());
            let sagitta_offset = (radius * radius - 0.25 * chord_length * chord_length).sqrt();
            let center = [0.5 * chord_length, -sagitta_offset, 0.0];
            let curved = CurvedBendMacroElement::new(
                node_i,
                node_j,
                center,
                ELASTIC_MODULUS,
                SHEAR_MODULUS,
                AREA,
                SECOND_MOMENT,
                TORSION_CONSTANT,
                1.0,
                1.0,
            )
            .unwrap()
            .global_stiffness()
            .unwrap();
            let mut max_difference = 0.0_f64;
            for row in 0..ELEMENT_DOF {
                for col in 0..ELEMENT_DOF {
                    max_difference =
                        max_difference.max((curved[row][col] - straight[row][col]).abs());
                }
            }
            max_difference / straight_scale
        };

        let error_coarse = relative_error(1.0e-2);
        let error_fine = relative_error(1.0e-3);
        assert!(
            error_fine <= 1.0e-3,
            "straight-limit relative error {error_fine} exceeds documented tolerance 1.0e-3"
        );
        assert!(
            error_fine < error_coarse,
            "error must shrink as the arc flattens: fine {error_fine} vs coarse {error_coarse}"
        );
    }

    // Expected values below are independent closed-form Castigliano
    // integrals written out longhand for the quarter circle (phi = pi/2),
    // NOT calls into the crate's flexibility machinery. In-plane bending
    // moment fields at arc angle theta for unit tip loads:
    //   Fx: M = -R (1 - sin theta), N = -sin theta
    //   Fy: M = -R cos theta,       N =  cos theta
    //   Mz: M = 1
    // with integrals over [0, pi/2]:
    //   int (1 - sin)^2 = 3 pi / 4 - 2, int sin^2 = int cos^2 = pi / 4,
    //   int (1 - sin) cos = 1 / 2,      int sin cos = 1 / 2,
    //   int (1 - sin) = pi / 2 - 1,     int cos = 1.
    #[test]
    fn quarter_circle_in_plane_tip_response_matches_closed_form() {
        for k_in in [1.0, 2.5] {
            let element = quarter_circle_element(k_in, 1.0);
            let bending = ELASTIC_MODULUS * SECOND_MOMENT;
            let axial = ELASTIC_MODULUS * AREA;
            let r = RADIUS;

            let f_xx = k_in * r * r * r * (0.75 * PI - 2.0) / bending + r * (0.25 * PI) / axial;
            let f_xy = k_in * r * r * r * 0.5 / bending - r * 0.5 / axial;
            let f_xm = k_in * r * r * (1.0 - 0.5 * PI) / bending;
            let f_yy = k_in * r * r * r * (0.25 * PI) / bending + r * (0.25 * PI) / axial;
            let f_ym = -k_in * r * r / bending;
            let f_mm = k_in * r * (0.5 * PI) / bending;

            let under_fx = tip_response(&element, 0);
            assert_close(under_fx[0], f_xx);
            assert_close(under_fx[1], f_xy);
            assert_close(under_fx[2], 0.0);
            assert_close(under_fx[3], 0.0);
            assert_close(under_fx[4], 0.0);
            assert_close(under_fx[5], f_xm);

            let under_fy = tip_response(&element, 1);
            assert_close(under_fy[0], f_xy);
            assert_close(under_fy[1], f_yy);
            assert_close(under_fy[5], f_ym);

            let under_mz = tip_response(&element, 5);
            assert_close(under_mz[0], f_xm);
            assert_close(under_mz[1], f_ym);
            assert_close(under_mz[5], f_mm);
        }
    }

    // Independent longhand out-of-plane integrals for the quarter circle.
    // A unit tip force along z produces bending R sin(pi/2 - theta) about
    // the radial axis and torsion R (1 - cos(pi/2 - theta)) about the
    // tangent; the arc geometry couples them. With C1 = pi / 4 and
    // C2 = 3 pi / 4 - 2:
    //   delta_z = P R^3 [k_out C1 / (EI) + C2 / (GJ)].
    #[test]
    fn quarter_circle_out_of_plane_tip_response_matches_closed_form() {
        for k_out in [1.0, 1.75] {
            let element = quarter_circle_element(1.0, k_out);
            let bending = ELASTIC_MODULUS * SECOND_MOMENT;
            let torsion = SHEAR_MODULUS * TORSION_CONSTANT;
            let r = RADIUS;
            let c1 = 0.25 * PI;
            let c2 = 0.75 * PI - 2.0;

            let f_zz = r * r * r * (k_out * c1 / bending + c2 / torsion);
            // int cos^2 = pi/4; int (1 - sin)(-sin) = pi/4 - 1.
            let f_zx = r * r * (k_out * c1 / bending + (0.25 * PI - 1.0) / torsion);
            // int sin cos = 1/2; int (1 - sin) cos = 1/2.
            let f_zy = r * r * (k_out * 0.5 / bending + 0.5 / torsion);
            let f_xx = r * (k_out * c1 / bending + c1 / torsion);
            let f_xy = r * (k_out * 0.5 / bending - 0.5 / torsion);
            let f_yy = r * (k_out * c1 / bending + c1 / torsion);

            let under_fz = tip_response(&element, 2);
            assert_close(under_fz[0], 0.0);
            assert_close(under_fz[1], 0.0);
            assert_close(under_fz[2], f_zz);
            assert_close(under_fz[3], f_zx);
            assert_close(under_fz[4], f_zy);
            assert_close(under_fz[5], 0.0);

            let under_mx = tip_response(&element, 3);
            assert_close(under_mx[2], f_zx);
            assert_close(under_mx[3], f_xx);
            assert_close(under_mx[4], f_xy);

            let under_my = tip_response(&element, 4);
            assert_close(under_my[2], f_zy);
            assert_close(under_my[3], f_xy);
            assert_close(under_my[4], f_yy);
        }
    }

    // The end flexibility is affine in each user factor: F = k B + C with
    // B the bending strain-energy part. k_in must scale only the in-plane
    // bending part and k_out only the out-of-plane bending part, leaving
    // torsion and axial contributions untouched.
    #[test]
    fn flexibility_factors_scale_only_their_bending_terms() {
        let base = quarter_circle_element(1.0, 1.0).end_flexibility().unwrap();
        let in_two = quarter_circle_element(2.0, 1.0).end_flexibility().unwrap();
        let in_five = quarter_circle_element(5.0, 1.0).end_flexibility().unwrap();
        let out_two = quarter_circle_element(1.0, 2.0).end_flexibility().unwrap();
        let out_five = quarter_circle_element(1.0, 5.0).end_flexibility().unwrap();

        let bending = ELASTIC_MODULUS * SECOND_MOMENT;
        let torsion = SHEAR_MODULUS * TORSION_CONSTANT;
        let r = RADIUS;

        // Pure in-plane moment case carries bending energy only, so the
        // rotational flexibility scales exactly with k_in.
        assert_close_tolerance(in_five[5][5], 5.0 * base[5][5], 1.0e-12);
        assert_close_tolerance(in_five[5][5], 5.0 * r * (0.5 * PI) / bending, 1.0e-12);

        for row in 0..DOF_PER_NODE {
            for col in 0..DOF_PER_NODE {
                let in_slope = in_two[row][col] - base[row][col];
                let out_slope = out_two[row][col] - base[row][col];
                // Affine in k_in: F(5) = F(1) + 4 (F(2) - F(1)).
                assert_close_tolerance(in_five[row][col], base[row][col] + 4.0 * in_slope, 1.0e-12);
                assert_close_tolerance(
                    out_five[row][col],
                    base[row][col] + 4.0 * out_slope,
                    1.0e-12,
                );
                // Each factor touches only its own plane's block.
                let in_plane_dof = |dof: usize| dof == 0 || dof == 1 || dof == 5;
                if !(in_plane_dof(row) && in_plane_dof(col)) {
                    assert_close_tolerance(in_slope, 0.0, 1.0e-15);
                }
                if in_plane_dof(row) || in_plane_dof(col) {
                    assert_close_tolerance(out_slope, 0.0, 1.0e-15);
                }
            }
        }

        // The k_out-independent residual C = F(1) - (F(2) - F(1)) of the
        // out-of-plane block is the torsion strain energy alone; longhand
        // quarter-circle torsion integrals:
        //   C_zz = R^3 (3 pi / 4 - 2) / (GJ), C_zx = R^2 (pi / 4 - 1) / (GJ),
        //   C_zy = R^2 / (2 GJ), C_xx = C_yy = R pi / (4 GJ),
        //   C_xy = -R / (2 GJ).
        let residual = |row: usize, col: usize| -> f64 {
            base[row][col] - (out_two[row][col] - base[row][col])
        };
        assert_close(residual(2, 2), r * r * r * (0.75 * PI - 2.0) / torsion);
        assert_close(residual(2, 3), r * r * (0.25 * PI - 1.0) / torsion);
        assert_close(residual(2, 4), r * r * 0.5 / torsion);
        assert_close(residual(3, 3), r * (0.25 * PI) / torsion);
        assert_close(residual(4, 4), r * (0.25 * PI) / torsion);
        assert_close(residual(3, 4), -r * 0.5 / torsion);
    }

    #[test]
    fn rigid_body_modes_produce_zero_force() {
        let element = quarter_circle_element(2.5, 1.75);
        let stiffness = element.global_stiffness().unwrap();
        let scale = matrix12_max_abs(&stiffness);
        let position_i = element.node_i.coordinates;
        let position_j = element.node_j.coordinates;

        let mut modes: Vec<[f64; ELEMENT_DOF]> = Vec::new();
        for axis in 0..3 {
            let mut translation = [0.0; ELEMENT_DOF];
            translation[axis] = 1.0;
            translation[DOF_PER_NODE + axis] = 1.0;
            modes.push(translation);

            let mut unit_rotation = [0.0; 3];
            unit_rotation[axis] = 1.0;
            let mut rotation = [0.0; ELEMENT_DOF];
            let swing_i = cross(unit_rotation, position_i);
            let swing_j = cross(unit_rotation, position_j);
            for component in 0..3 {
                rotation[component] = swing_i[component];
                rotation[DOF_PER_NODE + component] = swing_j[component];
            }
            rotation[3 + axis] = 1.0;
            rotation[DOF_PER_NODE + 3 + axis] = 1.0;
            modes.push(rotation);
        }

        for mode in &modes {
            for row in 0..ELEMENT_DOF {
                let mut force = 0.0;
                for col in 0..ELEMENT_DOF {
                    force += stiffness[row][col] * mode[col];
                }
                assert!(
                    force.abs() <= ASSERT_TOLERANCE * scale,
                    "rigid-body mode produced force {force} at row {row}"
                );
            }
        }
    }

    #[test]
    fn stiffness_is_symmetric_and_energy_nonnegative() {
        let element = quarter_circle_element(2.5, 1.75);
        let stiffness = element.global_stiffness().unwrap();
        let scale = matrix12_max_abs(&stiffness);

        for row in 0..ELEMENT_DOF {
            for col in 0..ELEMENT_DOF {
                assert!(
                    (stiffness[row][col] - stiffness[col][row]).abs() <= 1.0e-10 * scale,
                    "asymmetry at ({row}, {col})"
                );
            }
        }

        // Deterministic non-rigid probe vectors; energy must be
        // nonnegative and strictly positive for deforming shapes.
        let mut probes: Vec<[f64; ELEMENT_DOF]> = Vec::new();
        for dof in 0..ELEMENT_DOF {
            let mut probe = [0.0; ELEMENT_DOF];
            probe[dof] = 1.0;
            probes.push(probe);
        }
        let mut alternating = [0.0; ELEMENT_DOF];
        for (dof, entry) in alternating.iter_mut().enumerate() {
            *entry = if dof % 2 == 0 { 1.0 } else { -1.0 };
        }
        probes.push(alternating);

        for probe in &probes {
            let mut energy = 0.0;
            for row in 0..ELEMENT_DOF {
                for col in 0..ELEMENT_DOF {
                    energy += probe[row] * stiffness[row][col] * probe[col];
                }
            }
            assert!(
                energy >= -ASSERT_TOLERANCE * scale,
                "negative strain energy {energy}"
            );
        }
    }

    #[test]
    fn rotated_geometry_transforms_stiffness_congruently() {
        let element = quarter_circle_element(2.5, 1.75);
        let base = element.global_stiffness().unwrap();

        // Fixed rotation composed from axis rotations about z then y then x.
        let (angle_a, angle_b, angle_c) = (0.7_f64, 0.4_f64, 0.2_f64);
        let rotation_z = [
            [angle_a.cos(), -angle_a.sin(), 0.0],
            [angle_a.sin(), angle_a.cos(), 0.0],
            [0.0, 0.0, 1.0],
        ];
        let rotation_y = [
            [angle_b.cos(), 0.0, angle_b.sin()],
            [0.0, 1.0, 0.0],
            [-angle_b.sin(), 0.0, angle_b.cos()],
        ];
        let rotation_x = [
            [1.0, 0.0, 0.0],
            [0.0, angle_c.cos(), -angle_c.sin()],
            [0.0, angle_c.sin(), angle_c.cos()],
        ];
        let multiply3 = |left: [[f64; 3]; 3], right: [[f64; 3]; 3]| -> [[f64; 3]; 3] {
            let mut result = [[0.0; 3]; 3];
            for row in 0..3 {
                for col in 0..3 {
                    for inner in 0..3 {
                        result[row][col] += left[row][inner] * right[inner][col];
                    }
                }
            }
            result
        };
        let rotation = multiply3(rotation_x, multiply3(rotation_y, rotation_z));
        let rotate = |point: [f64; 3]| -> [f64; 3] {
            let mut rotated = [0.0; 3];
            for row in 0..3 {
                for col in 0..3 {
                    rotated[row] += rotation[row][col] * point[col];
                }
            }
            rotated
        };

        let rotated_element = CurvedBendMacroElement::new(
            FrameNode::new(0, rotate(element.node_i.coordinates)).unwrap(),
            FrameNode::new(1, rotate(element.node_j.coordinates)).unwrap(),
            rotate(element.center),
            ELASTIC_MODULUS,
            SHEAR_MODULUS,
            AREA,
            SECOND_MOMENT,
            TORSION_CONSTANT,
            2.5,
            1.75,
        )
        .unwrap();
        let rotated = rotated_element.global_stiffness().unwrap();

        // Expected congruent transform: block-diagonal rotation applied to
        // each 3-vector block of the base stiffness.
        let mut block_rotation = [[0.0; ELEMENT_DOF]; ELEMENT_DOF];
        for block in 0..4 {
            for row in 0..3 {
                for col in 0..3 {
                    block_rotation[3 * block + row][3 * block + col] = rotation[row][col];
                }
            }
        }
        let mut temp = [[0.0; ELEMENT_DOF]; ELEMENT_DOF];
        for row in 0..ELEMENT_DOF {
            for col in 0..ELEMENT_DOF {
                for inner in 0..ELEMENT_DOF {
                    temp[row][col] += block_rotation[row][inner] * base[inner][col];
                }
            }
        }
        let mut expected = [[0.0; ELEMENT_DOF]; ELEMENT_DOF];
        for row in 0..ELEMENT_DOF {
            for col in 0..ELEMENT_DOF {
                for inner in 0..ELEMENT_DOF {
                    expected[row][col] += temp[row][inner] * block_rotation[col][inner];
                }
            }
        }

        let scale = matrix12_max_abs(&base);
        for row in 0..ELEMENT_DOF {
            for col in 0..ELEMENT_DOF {
                assert!(
                    (rotated[row][col] - expected[row][col]).abs() <= ASSERT_TOLERANCE * scale,
                    "frame invariance violated at ({row}, {col})"
                );
            }
        }
    }

    #[test]
    fn constructor_rejects_repeated_node_index() {
        let node_i = FrameNode::new(0, [RADIUS, 0.0, 0.0]).unwrap();
        let node_j = FrameNode::new(0, [0.0, RADIUS, 0.0]).unwrap();
        let error = CurvedBendMacroElement::new(
            node_i,
            node_j,
            [0.0, 0.0, 0.0],
            ELASTIC_MODULUS,
            SHEAR_MODULUS,
            AREA,
            SECOND_MOMENT,
            TORSION_CONSTANT,
            1.0,
            1.0,
        )
        .unwrap_err();
        assert_eq!(
            error,
            CurvedBendError::Kernel(FrameKernelError::RepeatedElementNodeIndex { node_index: 0 })
        );
    }

    #[test]
    fn constructor_rejects_nonpositive_and_nonfinite_scalars() {
        let build = |elastic_modulus: f64, k_in: f64, k_out: f64| {
            CurvedBendMacroElement::new(
                FrameNode::new(0, [RADIUS, 0.0, 0.0]).unwrap(),
                FrameNode::new(1, [0.0, RADIUS, 0.0]).unwrap(),
                [0.0, 0.0, 0.0],
                elastic_modulus,
                SHEAR_MODULUS,
                AREA,
                SECOND_MOMENT,
                TORSION_CONSTANT,
                k_in,
                k_out,
            )
        };

        assert_eq!(
            build(0.0, 1.0, 1.0).unwrap_err(),
            CurvedBendError::Kernel(FrameKernelError::NonPositiveInput {
                name: "elastic_modulus",
                value: 0.0,
            })
        );
        assert_eq!(
            build(ELASTIC_MODULUS, 0.0, 1.0).unwrap_err(),
            CurvedBendError::Kernel(FrameKernelError::NonPositiveInput {
                name: "in_plane_flexibility_factor",
                value: 0.0,
            })
        );
        assert_eq!(
            build(ELASTIC_MODULUS, 1.0, -2.0).unwrap_err(),
            CurvedBendError::Kernel(FrameKernelError::NonPositiveInput {
                name: "out_of_plane_flexibility_factor",
                value: -2.0,
            })
        );
        assert!(matches!(
            build(ELASTIC_MODULUS, f64::NAN, 1.0).unwrap_err(),
            CurvedBendError::Kernel(FrameKernelError::NonFiniteInput {
                name: "in_plane_flexibility_factor",
                ..
            })
        ));
        assert!(matches!(
            build(ELASTIC_MODULUS, 1.0, f64::INFINITY).unwrap_err(),
            CurvedBendError::Kernel(FrameKernelError::NonFiniteInput {
                name: "out_of_plane_flexibility_factor",
                ..
            })
        ));
    }

    #[test]
    fn constructor_rejects_degenerate_geometry() {
        let build = |i: [f64; 3], j: [f64; 3], center: [f64; 3]| {
            CurvedBendMacroElement::new(
                FrameNode::new(0, i).unwrap(),
                FrameNode::new(1, j).unwrap(),
                center,
                ELASTIC_MODULUS,
                SHEAR_MODULUS,
                AREA,
                SECOND_MOMENT,
                TORSION_CONSTANT,
                1.0,
                1.0,
            )
        };

        // Center not equidistant from the end nodes.
        assert!(matches!(
            build([1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 0.0]).unwrap_err(),
            CurvedBendError::RadiusMismatch { .. }
        ));
        // Center coincident with an end node.
        assert_eq!(
            build([0.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 0.0]).unwrap_err(),
            CurvedBendError::DegenerateArc {
                detail: "arc end node coincides with the arc center",
            }
        );
        // Diametrically opposite end nodes: included angle pi.
        assert!(matches!(
            build([1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 0.0]).unwrap_err(),
            CurvedBendError::IncludedAngleOutOfRange { .. }
        ));
        // Coincident end coordinates on distinct nodes: included angle 0.
        assert!(matches!(
            build([1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 0.0]).unwrap_err(),
            CurvedBendError::IncludedAngleOutOfRange { .. }
        ));
        // Non-finite center coordinate.
        assert!(matches!(
            build([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [f64::NAN, 0.0, 0.0]).unwrap_err(),
            CurvedBendError::Kernel(FrameKernelError::NonFiniteInput { name: "center", .. })
        ));
    }
}
