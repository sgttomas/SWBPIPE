//! Code-neutral curved-bend macro-element mechanics.
//!
//! This crate forms the 12x12 stiffness of a circular-arc bend from a
//! closed-form unit-load (Castigliano) end-flexibility integration. The
//! in-plane and out-of-plane flexibility factors are user-entered opaque
//! numbers. This crate contains open mechanics routines only. It does not
//! encode design code compliance checks, protected standards content,
//! flexibility-factor or stress-intensification formulas, or private
//! project data.

use open_pipe_stress_frame_kernel::exact_sum::{exact_rounded_dot, ExactAccumulator};
use open_pipe_stress_frame_kernel::structural::twice_atan2_nonnegative;
use std::error::Error;
use std::f64::consts::PI;
use std::fmt;

use open_pipe_stress_frame_kernel::{
    solve_dense, transform_global_stiffness, FrameKernelError, FrameNode, FrameOrientation,
    Matrix12, DOF_PER_NODE, ELEMENT_DOF,
};

mod arc_integrals;
use arc_integrals::HalfAngle;

/// Node-level 6x6 matrix in the frame-kernel DOF order [ux, uy, uz, rx, ry, rz].
pub type Matrix6 = [[f64; DOF_PER_NODE]; DOF_PER_NODE];

const AXIS_TOLERANCE: f64 = 1.0e-12;
// Included-angle admissibility window (radians): [1e-9, pi - 1e-9].
const MIN_INCLUDED_ANGLE: f64 = 1.0e-9;

#[derive(Debug, Clone, PartialEq)]
pub enum CurvedBendError {
    Kernel(FrameKernelError),
    /// The bend radius cannot span the chord: 4R² ≤ |x_j − x_i|².
    RadiusCannotSpanChord {
        radius: f64,
        chord_length: f64,
    },
    DegenerateArc {
        detail: &'static str,
    },
    IncludedAngleOutOfRange {
        included_angle: f64,
    },
}

impl fmt::Display for CurvedBendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Kernel(error) => write!(f, "frame kernel error: {error}"),
            // The values stay in the variant (and its Debug). A caller that
            // screens R ≤ L/2 itself (PP) publishes both numbers; this exact
            // refusal is reached only where they agree to rounding, so the
            // text is fixed (no unclassified text arguments, T4-I19 O-10).
            Self::RadiusCannotSpanChord { .. } => write!(
                f,
                "bend radius cannot span the chord: 4R^2 <= |x_j - x_i|^2, so the arc included angle would reach or exceed pi"
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

/// The objective circular arc of T4-U1 (B1 of T4-I6's reference), formed
/// from the node difference d = x_j − x_i, the user radius R and the plane
/// reference y only; no absolute centre is formed anywhere.
///
/// With L = |d|, d̂ = d/L and n̂ the unit component of y normal to d̂ (the
/// arc bows toward +n̂): s = sin(φ/2) = L/(2R), c = cos(φ/2) =
/// √(4R² − L²)/(2R) with 4R² − L² formed from the exact squares of the
/// binary64 R and d and rounded once (accurate near π), and
/// φ = 2·atan2(s, c). Local axes (rows of `local_axes`, global
/// components): x = −s·d̂ + c·n̂ (radial at node i, outward), y = c·d̂ + s·n̂
/// (tangent at node i toward j), z = n̂ × d̂ (bend-plane normal). Node `i`
/// sits at arc angle 0 and node `j` at `included_angle`, measured about
/// local z, and the chord in the local frame is (−sL, cL, 0).
///
/// No libm call forms any of these (T4-I29): φ = 2·atan2(s, c) is FK's
/// `twice_atan2_nonnegative` (K3a's p = 128 arctangent, rounded once to
/// binary64, integer arithmetic only), and sin φ, cos φ, 1 − cos φ are
/// 2sc, 1 − 2s², 2s² (`arc_integrals`), so the element's stiffness and load
/// vector are bitwise identical on every platform.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArcGeometry {
    /// The user bend radius R.
    pub radius: f64,
    pub included_angle: f64,
    pub local_axes: [[f64; 3]; 3],
    /// sin(φ/2) = L/(2R).
    pub sin_half_angle: f64,
    /// cos(φ/2) = √(4R² − L²)/(2R).
    pub cos_half_angle: f64,
    /// L = |x_j − x_i|.
    pub chord_length: f64,
    /// The chord x_j − x_i in the local frame: (−sL, cL, 0).
    pub chord_local: [f64; 3],
    /// d̂ = (x_j − x_i)/L.
    pub chord_unit: [f64; 3],
    /// n̂: the unit in-plane normal to the chord on the bow side.
    pub bow_normal: [f64; 3],
}

impl ArcGeometry {
    fn half_angle(&self) -> HalfAngle {
        HalfAngle {
            phi: self.included_angle,
            s: self.sin_half_angle,
            c: self.cos_half_angle,
        }
    }
}

/// The shared arc definition (T4-I11 D-I): the included angle, the unit end
/// tangents oriented i → j and the in-plane bow normal n̂ of the arc formed
/// from the chord d = x_j − x_i, the radius R and the plane reference y.
/// t_i = c·d̂ + s·n̂ and t_j = c·d̂ − s·n̂ with s = sin(φ/2), c = cos(φ/2).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArcFrame {
    pub included_angle: f64,
    pub tangent_i: [f64; 3],
    pub tangent_j: [f64; 3],
    pub bow_normal: [f64; 3],
}

/// The arc of `ArcGeometry` as a pure function of (d, R, y_reference), or
/// the reason it is not admissible: a non-finite input, R ≤ L/2, a zero
/// chord, a y_reference (numerically) parallel to the chord, or an included
/// angle outside [1e-9, π − 1e-9].
pub fn arc_geometry(
    chord: [f64; 3],
    radius: f64,
    y_reference: [f64; 3],
) -> Result<ArcFrame, CurvedBendError> {
    let geometry = objective_arc(chord, radius, y_reference)?;
    let (s, c) = (geometry.sin_half_angle, geometry.cos_half_angle);
    let (d, n) = (geometry.chord_unit, geometry.bow_normal);
    let tangent_j = [
        c * d[0] - s * n[0],
        c * d[1] - s * n[1],
        c * d[2] - s * n[2],
    ];
    Ok(ArcFrame {
        included_angle: geometry.included_angle,
        tangent_i: geometry.local_axes[1],
        tangent_j,
        bow_normal: n,
    })
}

/// The direction change θ ∈ [0, π] between two directions `a` and `t`
/// (for example a straight member's direction and an arc end tangent):
/// θ = 2·atan2(|â − t̂|, |â + t̂|), accurate for small and near-π angles
/// (FK's libm-free `twice_atan2_nonnegative`).
pub fn kink(a: [f64; 3], t: [f64; 3]) -> Result<f64, CurvedBendError> {
    validate_finite_vector("kink_direction", a)?;
    validate_finite_vector("kink_direction", t)?;
    let a = normalize(a, "kink direction has zero length")?;
    let t = normalize(t, "kink direction has zero length")?;
    let difference = norm(subtract(a, t));
    let sum = norm([a[0] + t[0], a[1] + t[1], a[2] + t[2]]);
    twice_atan2_nonnegative(difference, sum).ok_or(CurvedBendError::DegenerateArc {
        detail: "kink directions do not define an angle",
    })
}

fn objective_arc(
    chord: [f64; 3],
    radius: f64,
    y_reference: [f64; 3],
) -> Result<ArcGeometry, CurvedBendError> {
    validate_finite_vector("arc_chord", chord)?;
    validate_positive_finite("bend_radius", radius)?;
    validate_finite_vector("y_reference", y_reference)?;
    let sum_error = |_| CurvedBendError::DegenerateArc {
        detail: "arc chord or radius is outside the binary64 range",
    };
    // |d|² and 4R² − |d|² from exact products, each rounded once.
    let length_squared = exact_rounded_dot(chord.iter().map(|&v| (v, v))).map_err(sum_error)?;
    let four_radius = 4.0 * radius;
    if !four_radius.is_finite() {
        return Err(CurvedBendError::DegenerateArc {
            detail: "arc chord or radius is outside the binary64 range",
        });
    }
    let span = exact_rounded_dot([
        (four_radius, radius),
        (-chord[0], chord[0]),
        (-chord[1], chord[1]),
        (-chord[2], chord[2]),
    ])
    .map_err(sum_error)?;
    let chord_length = length_squared.sqrt();
    if !(chord_length.is_finite() && chord_length > 0.0) {
        return Err(CurvedBendError::DegenerateArc {
            detail: "arc end nodes coincide",
        });
    }
    if span <= 0.0 {
        return Err(CurvedBendError::RadiusCannotSpanChord {
            radius,
            chord_length,
        });
    }
    let diameter = 2.0 * radius;
    let s = chord_length / diameter;
    let c = span.sqrt() / diameter;
    // s > 0 and c > 0 here (a positive chord and span); an underflowed s or
    // c gives 0 or π, which the window below refuses, as atan2 did.
    let included_angle = twice_atan2_nonnegative(s, c).ok_or(CurvedBendError::DegenerateArc {
        detail: "arc half-angle sine and cosine do not define an angle",
    })?;
    if !(MIN_INCLUDED_ANGLE..=PI - MIN_INCLUDED_ANGLE).contains(&included_angle) {
        return Err(CurvedBendError::IncludedAngleOutOfRange { included_angle });
    }

    let chord_unit = [
        chord[0] / chord_length,
        chord[1] / chord_length,
        chord[2] / chord_length,
    ];
    // n̂: y with its chord component removed (two Gram–Schmidt passes, so n̂
    // is orthogonal to d̂ to rounding however y is inclined), normalized.
    let reference_magnitude = norm(y_reference);
    let mut normal = y_reference;
    for _ in 0..2 {
        let projection = dot(normal, chord_unit);
        normal = [
            normal[0] - projection * chord_unit[0],
            normal[1] - projection * chord_unit[1],
            normal[2] - projection * chord_unit[2],
        ];
    }
    let normal_magnitude = norm(normal);
    if !(normal_magnitude > AXIS_TOLERANCE * reference_magnitude) {
        return Err(CurvedBendError::DegenerateArc {
            detail: "y_reference is parallel to the arc chord, so the bend plane is undefined",
        });
    }
    let normal = [
        normal[0] / normal_magnitude,
        normal[1] / normal_magnitude,
        normal[2] / normal_magnitude,
    ];
    let x_axis = [
        -s * chord_unit[0] + c * normal[0],
        -s * chord_unit[1] + c * normal[1],
        -s * chord_unit[2] + c * normal[2],
    ];
    let y_axis = [
        c * chord_unit[0] + s * normal[0],
        c * chord_unit[1] + s * normal[1],
        c * chord_unit[2] + s * normal[2],
    ];
    let z_axis = cross(normal, chord_unit);
    Ok(ArcGeometry {
        radius,
        included_angle,
        local_axes: [x_axis, y_axis, z_axis],
        sin_half_angle: s,
        cos_half_angle: c,
        chord_length,
        chord_local: [-s * chord_length, c * chord_length, 0.0],
        chord_unit,
        bow_normal: normal,
    })
}

/// Two-node circular-arc bend macro-element.
///
/// The element carries user-entered in-plane and out-of-plane bending
/// flexibility factors. Factors equal to 1 reproduce the plain
/// Euler-Bernoulli curved beam (bending, torsion, and axial strain energy;
/// shear deformation excluded, consistent with the frame kernel).
///
/// The element is defined by (x_i, x_j, R, y_reference) and the section
/// constants (T4-U1): its geometry depends on the nodes only through
/// d = x_j − x_i, so it is invariant under translation, and its stiffness
/// annihilates the rigid motions of the actual nodes (H uses the actual
/// chord (−sL, cL, 0)).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurvedBendMacroElement {
    pub node_i: FrameNode,
    pub node_j: FrameNode,
    /// User bend radius R.
    pub radius: f64,
    /// Plane reference: the arc bows toward +y_reference projected normal
    /// to the chord.
    pub y_reference: [f64; 3],
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
        radius: f64,
        y_reference: [f64; 3],
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
        validate_positive_finite("bend_radius", radius)?;
        validate_finite_vector("y_reference", y_reference)?;
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
            radius,
            y_reference,
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

    /// The node difference d = x_j − x_i (binary64).
    pub fn chord(&self) -> [f64; 3] {
        subtract(self.node_j.coordinates, self.node_i.coordinates)
    }

    /// Validated objective arc geometry (see `ArcGeometry`).
    pub fn geometry(&self) -> Result<ArcGeometry, CurvedBendError> {
        objective_arc(self.chord(), self.radius, self.y_reference)
    }

    /// The shared arc definition (`arc_geometry`) of this element.
    pub fn arc_frame(&self) -> Result<ArcFrame, CurvedBendError> {
        arc_geometry(self.chord(), self.radius, self.y_reference)
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
    /// Entries are the closed-form unit-load integrals over the arc of the
    /// bending, torsion, and axial strain-energy products (B1), each written
    /// as a combination of non-cancelling terms and of the cancelling arc
    /// integrals of `arc_integrals`, which are evaluated stably (series for
    /// φ < π/3, half-angle closed forms above). The user factors scale only
    /// the in-plane and out-of-plane bending-curvature terms.
    pub fn end_flexibility(&self) -> Result<Matrix6, CurvedBendError> {
        let geometry = self.geometry()?;
        Ok(self.flexibility_of(&geometry))
    }

    fn flexibility_of(&self, geometry: &ArcGeometry) -> Matrix6 {
        let r = geometry.radius;
        let half = geometry.half_angle();
        let phi = half.phi;
        let sin = half.sin();
        let cos = half.cos();
        let k_in = self.in_plane_flexibility_factor;
        let k_out = self.out_of_plane_flexibility_factor;
        let bending = self.elastic_modulus * self.second_moment;
        let torsion = self.shear_modulus * self.torsion_constant;
        let axial = self.elastic_modulus * self.area;

        // ∫sin²θ = (φ − SC)/2, ∫cos²θ = (φ + SC)/2, ∫sinθ cosθ = S²/2.
        let sin_sq = 0.5 * half.phi_minus_sin_cos();
        let cos_sq = 0.5 * (phi + sin * cos);
        let sin_cos = 0.5 * sin * sin;
        // ∫(sin φ − sin θ) = φS − (1 − C); ∫(cos φ − cos θ) = −(S − φC).
        let in_plane_x = phi * sin - half.one_minus_cos();
        let sin_minus_phi_cos = half.sin_minus_phi_cos();
        let r2 = r * r;

        let mut f = [[0.0; DOF_PER_NODE]; DOF_PER_NODE];
        // In plane: Fx (0), Fy (1), Mz (5). M_ip = −R(S − sin θ), R(C − cos θ), 1;
        // N = −sin θ, cos θ, 0.
        f[0][0] = r * (k_in * r2 * half.in_plane_xx() / bending + sin_sq / axial);
        f[0][1] = r * (-k_in * r2 * half.in_plane_xy() / bending - sin_cos / axial);
        f[1][1] = r * (k_in * r2 * half.in_plane_yy() / bending + cos_sq / axial);
        f[0][5] = r * (-k_in * r * in_plane_x / bending);
        f[1][5] = r * (-k_in * r * sin_minus_phi_cos / bending);
        f[5][5] = r * (k_in * phi / bending);
        // Out of plane: Fz (2), Mx (3), My (4). M_op = R sin(φ − θ), cos θ,
        // sin θ; T = R(1 − cos(φ − θ)), −sin θ, cos θ.
        f[2][2] = r * (k_out * r2 * sin_sq / bending + r2 * half.torsion_zz() / torsion);
        f[2][3] = r
            * (k_out * r * (0.5 * phi * sin) / bending
                - r * half.one_minus_cos_minus_half_phi_sin() / torsion);
        f[2][4] = r
            * (k_out * r * (0.5 * sin_minus_phi_cos) / bending
                + r * (0.5 * sin_minus_phi_cos) / torsion);
        f[3][3] = r * (k_out * cos_sq / bending + sin_sq / torsion);
        f[3][4] = r * (k_out * sin_cos / bending - sin_cos / torsion);
        f[4][4] = r * (k_out * sin_sq / bending + cos_sq / torsion);
        for row in 0..DOF_PER_NODE {
            for col in 0..row {
                f[row][col] = f[col][row];
            }
        }
        f
    }

    /// 12x12 stiffness in the local frame, DOF order [node i; node j]:
    /// [[H K_t Hᵀ, −H K_t], [−K_t Hᵀ, K_t]] with K_t = F⁻¹ and H the rigid
    /// transfer over the actual chord (−sL, cL, 0).
    pub fn local_stiffness(&self) -> Result<Matrix12, CurvedBendError> {
        let geometry = self.geometry()?;
        let tip_stiffness = invert_symmetric6(&self.flexibility_of(&geometry))?;
        Ok(assemble_macro_stiffness(
            &tip_stiffness,
            geometry.chord_local,
        ))
    }

    /// 12x12 stiffness in global coordinates.
    pub fn global_stiffness(&self) -> Result<Matrix12, CurvedBendError> {
        let local = self.local_stiffness()?;
        let orientation = self.orientation()?;
        Ok(transform_global_stiffness(&local, &orientation))
    }

    /// Consistent equivalent nodal loads, in global coordinates ordered
    /// [node i; node j], for a uniform distributed load of constant global
    /// intensity (force per unit arc length).
    ///
    /// The fixed-end forces and moments come from the force method on the
    /// same exact closed-form unit-load integration used for the element
    /// stiffness: the free-tip deflection under the load is integrated along
    /// the arc (extended {1, cos, sin, theta, theta cos, theta sin} basis, no
    /// quadrature), the clamped-tip redundants follow from compatibility with
    /// the element end flexibility, and the node-i share follows from rigid
    /// equilibrium with the distributed-load resultant. The user flexibility
    /// factors enter the bending curvature terms exactly as in
    /// `end_flexibility`, so the load vector is consistent with the assembled
    /// stiffness. Adding this vector to the global system load produces the
    /// exact end response of the continuously loaded arc; the element's true
    /// end forces are then `K d - p` with `p` this vector.
    pub fn consistent_uniform_nodal_loads(
        &self,
        intensity_global: [f64; 3],
    ) -> Result<[f64; ELEMENT_DOF], CurvedBendError> {
        validate_finite_vector("uniform_load_intensity", intensity_global)?;
        let geometry = self.geometry()?;
        let intensity_local = rotate_to_local(&geometry.local_axes, intensity_global);
        let tip_deflection = self.tip_deflection_under_uniform_load(&geometry, intensity_local);
        let tip_stiffness = invert_symmetric6(&self.flexibility_of(&geometry))?;

        // Clamped-tip redundant (support-on-element force at node j in the
        // both-ends-clamped state): X = -K_jj * delta0.
        let mut clamped_tip_force = [0.0; DOF_PER_NODE];
        for row in 0..DOF_PER_NODE {
            for col in 0..DOF_PER_NODE {
                clamped_tip_force[row] -= tip_stiffness[row][col] * tip_deflection[col];
            }
        }

        // Distributed-load resultant about node i in the local frame:
        // total force R*phi*w and moment R^2 * (sin phi - phi, 1 - cos phi, 0) x w
        // (sin phi - phi and 1 - cos phi in their stable forms).
        let radius = geometry.radius;
        let included_angle = geometry.included_angle;
        let half = geometry.half_angle();
        let moment_arm = [
            -(radius * radius * half.phi_minus_sin()),
            radius * radius * half.one_minus_cos(),
            0.0,
        ];
        let load_moment_about_i = cross(moment_arm, intensity_local);
        let mut load_resultant_at_i = [0.0; DOF_PER_NODE];
        for axis in 0..3 {
            load_resultant_at_i[axis] = radius * included_angle * intensity_local[axis];
            load_resultant_at_i[3 + axis] = load_moment_about_i[axis];
        }

        // Equivalent nodal loads: p_j = -X and p_i = H X + W_i, with H the
        // rigid transfer of node-j forces to node i over the actual chord.
        let transfer = equilibrium_transfer(geometry.chord_local);
        let mut local_loads = [0.0; ELEMENT_DOF];
        for row in 0..DOF_PER_NODE {
            let mut transferred = 0.0;
            for col in 0..DOF_PER_NODE {
                transferred += transfer[row][col] * clamped_tip_force[col];
            }
            local_loads[row] = transferred + load_resultant_at_i[row];
            local_loads[DOF_PER_NODE + row] = -clamped_tip_force[row];
        }

        let mut global_loads = [0.0; ELEMENT_DOF];
        for block in 0..(ELEMENT_DOF / 3) {
            let local_block = [
                local_loads[3 * block],
                local_loads[3 * block + 1],
                local_loads[3 * block + 2],
            ];
            let global_block = rotate_to_global(&geometry.local_axes, local_block);
            global_loads[3 * block] = global_block[0];
            global_loads[3 * block + 1] = global_block[1];
            global_loads[3 * block + 2] = global_block[2];
        }
        Ok(global_loads)
    }

    /// Section resultants at arc fraction `fraction` (0 at node i, 1 at
    /// node j) from segment equilibrium of the arc between the section and
    /// node j, ordered [axial, shear_y, shear_z, torsion, bending_y,
    /// bending_z] in the right-handed arc section frame (x tangent toward
    /// node j, z the bend-plane normal, y = z cross x pointing to the arc
    /// center).
    ///
    /// `node_j_force_global` is the true node-on-element force at node j in
    /// global coordinates (for an assembled solve, `K d - p` with `p` the
    /// consistent load vector) and `intensity_global` the same uniform
    /// distributed intensity passed to `consistent_uniform_nodal_loads`
    /// (zero when the span carries no distributed load). At fraction 1 the
    /// resultants equal the node-j end force; at fraction 0 they equal the
    /// negated node-i end force by whole-element equilibrium.
    pub fn arc_section_resultants(
        &self,
        fraction: f64,
        node_j_force_global: [f64; DOF_PER_NODE],
        intensity_global: [f64; 3],
    ) -> Result<[f64; DOF_PER_NODE], CurvedBendError> {
        if !(fraction.is_finite() && (0.0..=1.0).contains(&fraction)) {
            return Err(FrameKernelError::NonFiniteInput {
                name: "arc_station_fraction",
                value: fraction,
            }
            .into());
        }
        for value in node_j_force_global {
            if !value.is_finite() {
                return Err(FrameKernelError::NonFiniteInput {
                    name: "node_j_force_global",
                    value,
                }
                .into());
            }
        }
        validate_finite_vector("uniform_load_intensity", intensity_global)?;
        let geometry = self.geometry()?;
        let radius = geometry.radius;
        let included_angle = geometry.included_angle;
        let theta = fraction * included_angle;
        let intensity_local = rotate_to_local(&geometry.local_axes, intensity_global);
        let tip_force_local = rotate_to_local(
            &geometry.local_axes,
            [
                node_j_force_global[0],
                node_j_force_global[1],
                node_j_force_global[2],
            ],
        );
        let tip_moment_local = rotate_to_local(
            &geometry.local_axes,
            [
                node_j_force_global[3],
                node_j_force_global[4],
                node_j_force_global[5],
            ],
        );

        // Equilibrium of the arc segment [theta, included_angle]: the section
        // resultant is the j-side action on the section.
        let remaining_angle = included_angle - theta;
        let mut section_force = [0.0; 3];
        for axis in 0..3 {
            section_force[axis] =
                tip_force_local[axis] + radius * remaining_angle * intensity_local[axis];
        }
        // Arm from the section point to node j.
        let half = geometry.half_angle();
        let (sin_end, cos_end) = (half.sin(), half.cos());
        let section_to_j = [
            radius * (cos_end - theta.cos()),
            radius * (sin_end - theta.sin()),
            0.0,
        ];
        let tip_force_moment = cross(section_to_j, tip_force_local);
        // Distributed-load moment about the section point:
        // R^2 * a(theta) x w with a from the closed-form segment integral.
        let distributed_arm = [
            radius * radius * ((sin_end - theta.sin()) - remaining_angle * theta.cos()),
            radius * radius * ((theta.cos() - cos_end) - remaining_angle * theta.sin()),
            0.0,
        ];
        let distributed_moment = cross(distributed_arm, intensity_local);
        let mut section_moment = [0.0; 3];
        for axis in 0..3 {
            section_moment[axis] =
                tip_moment_local[axis] + tip_force_moment[axis] + distributed_moment[axis];
        }

        // Section frame rows in the local frame: x tangent, y = z cross x, z normal.
        let tangent = [-theta.sin(), theta.cos(), 0.0];
        let inward = [-theta.cos(), -theta.sin(), 0.0];
        let normal = [0.0, 0.0, 1.0];
        Ok([
            dot(section_force, tangent),
            dot(section_force, inward),
            dot(section_force, normal),
            dot(section_moment, tangent),
            dot(section_moment, inward),
            dot(section_moment, normal),
        ])
    }

    /// Global unit tangents of the arc at node `i` and node `j`, oriented
    /// i -> j, ordered `[tangent_i, tangent_j]`, derived from the validated
    /// arc geometry (no new geometry source): in the local frame
    /// `t(theta) = (-sin theta, cos theta, 0)` with node `i` at arc angle 0
    /// and node `j` at the included angle.
    pub fn end_tangents(&self) -> Result<[[f64; 3]; 2], CurvedBendError> {
        let frame = self.arc_frame()?;
        Ok([frame.tangent_i, frame.tangent_j])
    }

    /// E11 terms API (S11-K; dormant until S11-F). By linearity the section
    /// resultant is the exact sum of this element's section function applied
    /// to the node-j force alone and to each uniform intensity alone; each
    /// component is rounded once. With a single input it equals
    /// `arc_section_resultants` on that input (a zero component is +0.0).
    pub fn arc_section_resultant_terms(
        &self,
        fraction: f64,
        node_j_force_global: [f64; DOF_PER_NODE],
        intensities_global: &[[f64; 3]],
    ) -> Result<[f64; DOF_PER_NODE], CurvedBendError> {
        let mut terms = Vec::with_capacity(1 + intensities_global.len());
        terms.push(self.arc_section_resultants(fraction, node_j_force_global, [0.0; 3])?);
        for &intensity in intensities_global {
            terms.push(self.arc_section_resultants(fraction, [0.0; DOF_PER_NODE], intensity)?);
        }
        let mut resultants = [0.0; DOF_PER_NODE];
        for (component, value) in resultants.iter_mut().enumerate() {
            let mut accumulator = ExactAccumulator::new();
            for term in &terms {
                accumulator.add(term[component]).map_err(|_| {
                    CurvedBendError::from(FrameKernelError::NonFiniteInput {
                        name: "arc_section_resultant_term",
                        value: term[component],
                    })
                })?;
            }
            *value = accumulator.round().map_err(|_| {
                CurvedBendError::from(FrameKernelError::NonFiniteInput {
                    name: "arc_section_resultants",
                    value: if accumulator.signum() < 0 {
                        f64::NEG_INFINITY
                    } else {
                        f64::INFINITY
                    },
                })
            })?;
        }
        Ok(resultants)
    }

    // Free-tip (node i clamped) deflection at node j under the uniform load,
    // local frame, by the unit-load theorem with the same strain-energy
    // weights as `end_flexibility`.
    fn tip_deflection_under_uniform_load(
        &self,
        geometry: &ArcGeometry,
        intensity_local: [f64; 3],
    ) -> [f64; DOF_PER_NODE] {
        let half = geometry.half_angle();
        let gram = trig_extended_gram(&half);
        let cases = unit_load_actions(geometry.radius, &half);
        let load = distributed_load_actions(geometry.radius, &half, intensity_local);
        let bending_rigidity = self.elastic_modulus * self.second_moment;
        let torsion_rigidity = self.shear_modulus * self.torsion_constant;
        let axial_rigidity = self.elastic_modulus * self.area;

        let mut deflection = [0.0; DOF_PER_NODE];
        for (row, case) in cases.iter().enumerate() {
            deflection[row] = geometry.radius
                * (self.in_plane_flexibility_factor
                    * cross_quad(&gram, case.in_plane_moment, load.in_plane_moment)
                    / bending_rigidity
                    + self.out_of_plane_flexibility_factor
                        * cross_quad(&gram, case.out_of_plane_moment, load.out_of_plane_moment)
                        / bending_rigidity
                    + cross_quad(&gram, case.torsion, load.torsion) / torsion_rigidity
                    + cross_quad(&gram, case.axial, load.axial) / axial_rigidity);
        }
        deflection
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
fn unit_load_actions(radius: f64, half: &HalfAngle) -> [UnitLoadActions; 6] {
    let sin_end = half.sin();
    let cos_end = half.cos();
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

// Coefficients of {1, cos(theta), sin(theta), theta, theta cos(theta),
// theta sin(theta)} for an internal action along the arc under a uniform
// distributed load (constant local intensity, per unit arc length).
type ExtendedSeries = [f64; 6];

struct DistributedLoadActions {
    in_plane_moment: ExtendedSeries,
    out_of_plane_moment: ExtendedSeries,
    torsion: ExtendedSeries,
    axial: ExtendedSeries,
}

// Internal actions at arc angle theta caused by the uniform local intensity
// w on the free segment [theta, phi], from segment equilibrium (same section
// convention as `unit_load_actions`). With the segment force
// f = R (phi - theta) w and segment moment m = R^2 a(theta) x w where
// a(theta) = (sin phi - sin theta - (phi - theta) cos theta,
//             cos theta - cos phi - (phi - theta) sin theta, 0):
// in-plane bending is the z moment component R^2 (a_x w_y - a_y w_x);
// out-of-plane bending m . r reduces to R^2 w_z (1 - cos(phi - theta));
// torsion m . t reduces to R^2 w_z ((phi - theta) - sin(phi - theta));
// axial force is f . t = R (phi - theta)(w_y cos theta - w_x sin theta).
fn distributed_load_actions(
    radius: f64,
    half: &HalfAngle,
    intensity_local: [f64; 3],
) -> DistributedLoadActions {
    let included_angle = half.phi;
    let sin_end = half.sin();
    let cos_end = half.cos();
    let [w_x, w_y, w_z] = intensity_local;
    let radius_squared = radius * radius;

    // a_x over the extended basis: [sin phi, -phi, -1, 0, 1, 0];
    // a_y over the extended basis: [-cos phi, 1, -phi, 0, 0, 1].
    let a_x: ExtendedSeries = [sin_end, -included_angle, -1.0, 0.0, 1.0, 0.0];
    let a_y: ExtendedSeries = [-cos_end, 1.0, -included_angle, 0.0, 0.0, 1.0];
    let mut in_plane_moment = [0.0; 6];
    for term in 0..6 {
        in_plane_moment[term] = radius_squared * (w_y * a_x[term] - w_x * a_y[term]);
    }

    let out_of_plane_moment: ExtendedSeries = [
        radius_squared * w_z,
        -radius_squared * w_z * cos_end,
        -radius_squared * w_z * sin_end,
        0.0,
        0.0,
        0.0,
    ];
    let torsion: ExtendedSeries = [
        radius_squared * w_z * included_angle,
        -radius_squared * w_z * sin_end,
        radius_squared * w_z * cos_end,
        -radius_squared * w_z,
        0.0,
        0.0,
    ];
    let axial: ExtendedSeries = [
        0.0,
        radius * w_y * included_angle,
        -radius * w_x * included_angle,
        0.0,
        -radius * w_y,
        radius * w_x,
    ];
    DistributedLoadActions {
        in_plane_moment,
        out_of_plane_moment,
        torsion,
        axial,
    }
}

// Exact integrals over [0, phi] of products of the trig basis {1, cos, sin}
// (rows) with the extended basis {1, cos, sin, theta, theta cos, theta sin}
// (columns).
// sin, cos, 1 - cos, sin 2phi = 2SC and cos 2phi - 1 = -2S^2 come from the
// half-angle quantities; the theta-weighted entries keep their closed forms
// (they still cancel at small angles; a certified arc load is T4-U1b's).
fn trig_extended_gram(half: &HalfAngle) -> [[f64; 6]; 3] {
    let phi = half.phi;
    let sin_end = half.sin();
    let cos_end = half.cos();
    let one_minus_cos = half.one_minus_cos();
    let sin_double = 2.0 * sin_end * cos_end;
    let cos_double_minus_one = -2.0 * sin_end * sin_end;
    // Antiderivative identities: int theta cos = phi sin + cos - 1;
    // int theta sin = sin - phi cos; int theta cos^2 = phi^2/4
    // + phi sin(2 phi)/4 + (cos(2 phi) - 1)/8; int theta sin^2 = phi^2/4
    // - phi sin(2 phi)/4 - (cos(2 phi) - 1)/8; int theta sin cos
    // = sin(2 phi)/8 - phi cos(2 phi)/4.
    let int_theta_cos = phi * sin_end - one_minus_cos;
    let int_theta_sin = half.sin_minus_phi_cos();
    let int_theta_cos_cos =
        0.25 * phi * phi + 0.25 * phi * sin_double + 0.125 * cos_double_minus_one;
    let int_theta_sin_sin =
        0.25 * phi * phi - 0.25 * phi * sin_double - 0.125 * cos_double_minus_one;
    let int_theta_sin_cos = 0.125 * sin_double - 0.25 * phi * (1.0 + cos_double_minus_one);
    [
        [
            phi,
            sin_end,
            one_minus_cos,
            0.5 * phi * phi,
            int_theta_cos,
            int_theta_sin,
        ],
        [
            sin_end,
            0.5 * phi + 0.25 * sin_double,
            0.5 * sin_end * sin_end,
            int_theta_cos,
            int_theta_cos_cos,
            int_theta_sin_cos,
        ],
        [
            one_minus_cos,
            0.5 * sin_end * sin_end,
            half.phi_minus_sin_cos() * 0.5,
            int_theta_sin,
            int_theta_sin_cos,
            int_theta_sin_sin,
        ],
    ]
}

fn cross_quad(gram: &[[f64; 6]; 3], left: TrigSeries, right: ExtendedSeries) -> f64 {
    let mut sum = 0.0;
    for row in 0..3 {
        for col in 0..6 {
            sum += left[row] * gram[row][col] * right[col];
        }
    }
    sum
}

// Local components of a global vector: rows of `local_axes` are the local
// axes expressed in global coordinates.
fn rotate_to_local(local_axes: &[[f64; 3]; 3], global: [f64; 3]) -> [f64; 3] {
    [
        dot(local_axes[0], global),
        dot(local_axes[1], global),
        dot(local_axes[2], global),
    ]
}

fn rotate_to_global(local_axes: &[[f64; 3]; 3], local: [f64; 3]) -> [f64; 3] {
    let mut global = [0.0; 3];
    for axis in 0..3 {
        for component in 0..3 {
            global[component] += local_axes[axis][component] * local[axis];
        }
    }
    global
}

// Inverse of a symmetric positive-definite 6x6 via the frame-kernel dense
// solver, one unit column at a time in fixed order, then symmetrized. The
// matrix is first multiplied by one power of two c ≥ 1 that brings its
// largest diagonal entry into [1, 2), and the solution by c again: every
// operation of the solve then scales exactly, so the inverse is bit for bit
// the unscaled one wherever that is formed, while the solver's absolute
// pivot guard (1e-12) acts relative to the matrix's own scale and no longer
// refuses a short arc for the size of its flexibilities (T4-U1 phase B).
fn invert_symmetric6(matrix: &Matrix6) -> Result<Matrix6, CurvedBendError> {
    let scale = flexibility_scale(matrix);
    let dense: Vec<Vec<f64>> = matrix
        .iter()
        .map(|row| row.iter().map(|value| scale * value).collect())
        .collect();
    let mut inverse = [[0.0; DOF_PER_NODE]; DOF_PER_NODE];
    for col in 0..DOF_PER_NODE {
        let mut rhs = vec![0.0; DOF_PER_NODE];
        rhs[col] = 1.0;
        let solution = solve_dense(&dense, &rhs)?;
        for (row, value) in solution.iter().enumerate() {
            inverse[row][col] = scale * *value;
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

// 2^-e for the largest diagonal entry m·2^e (1 ≤ m < 2) when that entry is
// a positive normal number below 1; otherwise 1 (no scaling).
fn flexibility_scale(matrix: &Matrix6) -> f64 {
    let largest = (0..DOF_PER_NODE).fold(0.0_f64, |m, i| m.max(matrix[i][i]));
    if !(largest.is_normal() && largest > 0.0 && largest < 1.0) {
        return 1.0;
    }
    let exponent = ((largest.to_bits() >> 52) & 0x7ff) as i64 - 1023;
    f64::from_bits(((1023 - exponent) as u64) << 52)
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
mod platform_bits_tests;

#[cfg(test)]
mod s11k_tests;

#[cfg(test)]
mod short_arc_tests;

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

    // Quarter circle in the global x-y plane about the origin so the local
    // frame coincides with the global frame: node i at (R, 0, 0), node j at
    // (0, R, 0), bowing toward +(1, 1, 0), bend-plane normal +z.
    fn quarter_circle_element(k_in: f64, k_out: f64) -> CurvedBendMacroElement {
        let node_i = FrameNode::new(0, [RADIUS, 0.0, 0.0]).unwrap();
        let node_j = FrameNode::new(1, [0.0, RADIUS, 0.0]).unwrap();
        CurvedBendMacroElement::new(
            node_i,
            node_j,
            RADIUS,
            [1.0, 1.0, 0.0],
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
            let curved = CurvedBendMacroElement::new(
                node_i,
                node_j,
                radius,
                [0.0, 1.0, 0.0],
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
            element.radius,
            rotate(element.y_reference),
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

    // Solve the anchored-at-i arc under an arbitrary 12-slot global load
    // vector with the frame-kernel reduction, returning full displacements.
    fn anchored_solution(
        element: &CurvedBendMacroElement,
        force: &[f64; ELEMENT_DOF],
    ) -> [f64; ELEMENT_DOF] {
        let stiffness = element.global_stiffness().unwrap();
        let dense: Vec<Vec<f64>> = stiffness.iter().map(|row| row.to_vec()).collect();
        let restrained: Vec<usize> = (0..DOF_PER_NODE).collect();
        let reduced = reduce_system(&dense, force, &restrained).unwrap();
        let solution = solve_dense(&reduced.stiffness, &reduced.force).unwrap();
        let mut displacements = [0.0; ELEMENT_DOF];
        for (offset, value) in solution.iter().enumerate() {
            displacements[DOF_PER_NODE + offset] = *value;
        }
        displacements
    }

    // True node-on-element end forces of the loaded arc: K d - p.
    fn end_forces(
        element: &CurvedBendMacroElement,
        displacements: &[f64; ELEMENT_DOF],
        equivalent_loads: &[f64; ELEMENT_DOF],
    ) -> [f64; ELEMENT_DOF] {
        let stiffness = element.global_stiffness().unwrap();
        let mut forces = [0.0; ELEMENT_DOF];
        for row in 0..ELEMENT_DOF {
            for col in 0..ELEMENT_DOF {
                forces[row] += stiffness[row][col] * displacements[col];
            }
            forces[row] -= equivalent_loads[row];
        }
        forces
    }

    // Straight-limit convergence of the consistent uniform load: a shallow
    // arc over a chord along global x must approach the classical straight
    // fixed-end vector [0, wy L/2, wz L/2, 0, -wz L^2/12, wy L^2/12] at
    // node i (opposite end-moment signs at node j).
    #[test]
    fn consistent_uniform_load_matches_straight_fixed_end_in_straight_limit() {
        let chord_length = 2.0;
        let intensity = [0.0, -13.0, 7.0];
        let node_i = FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap();
        let node_j = FrameNode::new(1, [chord_length, 0.0, 0.0]).unwrap();
        let expected_share = |w: f64| w * chord_length / 2.0;
        let expected_moment = |w: f64| w * chord_length * chord_length / 12.0;
        let expected = [
            0.0,
            expected_share(intensity[1]),
            expected_share(intensity[2]),
            0.0,
            -expected_moment(intensity[2]),
            expected_moment(intensity[1]),
            0.0,
            expected_share(intensity[1]),
            expected_share(intensity[2]),
            0.0,
            expected_moment(intensity[2]),
            -expected_moment(intensity[1]),
        ];
        let scale = expected
            .iter()
            .fold(0.0_f64, |max, value| max.max(value.abs()));

        let relative_error = |included_angle: f64| -> f64 {
            let radius = chord_length / (2.0 * (included_angle / 2.0).sin());
            let loads = CurvedBendMacroElement::new(
                node_i,
                node_j,
                radius,
                [0.0, 1.0, 0.0],
                ELASTIC_MODULUS,
                SHEAR_MODULUS,
                AREA,
                SECOND_MOMENT,
                TORSION_CONSTANT,
                1.0,
                1.0,
            )
            .unwrap()
            .consistent_uniform_nodal_loads(intensity)
            .unwrap();
            loads
                .iter()
                .zip(expected.iter())
                .fold(0.0_f64, |max, (actual, target)| {
                    max.max((actual - target).abs())
                })
                / scale
        };

        // The chord-vs-arc difference is O(included_angle); below ~1e-3 rad
        // floating-point cancellation in the closed-form integrals dominates,
        // so this test uses a documented looser tolerance at 1e-3 rad and
        // additionally checks first-order convergence from 1e-2 rad.
        let error_coarse = relative_error(1.0e-2);
        let error_fine = relative_error(1.0e-3);
        assert!(
            error_fine <= 2.0e-3,
            "straight-limit consistent-load relative error {error_fine} exceeds 2.0e-3"
        );
        assert!(
            error_fine < error_coarse,
            "error must shrink as the arc flattens: fine {error_fine} vs coarse {error_coarse}"
        );
    }

    // The equivalent nodal loads must carry the exact distributed-load
    // resultant: sum of forces equals w * arc_length and the moment of the
    // nodal loads about the global origin equals the moment of the
    // distributed load, on the rotated arc as well.
    #[test]
    fn consistent_uniform_load_satisfies_rigid_equilibrium() {
        let base = quarter_circle_element(2.5, 1.75);
        let shifted = CurvedBendMacroElement::new(
            FrameNode::new(0, [RADIUS + 0.4, 0.7, -0.3]).unwrap(),
            FrameNode::new(1, [0.4, 0.7 + RADIUS, -0.3]).unwrap(),
            RADIUS,
            [1.0, 1.0, 0.0],
            ELASTIC_MODULUS,
            SHEAR_MODULUS,
            AREA,
            SECOND_MOMENT,
            TORSION_CONSTANT,
            2.5,
            1.75,
        )
        .unwrap();
        for element in [base, shifted] {
            let intensity = [4.0, -13.0, -9.0];
            let loads = element.consistent_uniform_nodal_loads(intensity).unwrap();
            let geometry = element.geometry().unwrap();
            let arc_length = geometry.radius * geometry.included_angle;

            // Total distributed force and its moment about the global origin
            // from the closed-form arc integrals in the local frame.
            let local_intensity = rotate_to_local(&geometry.local_axes, intensity);
            let total_force_local = [
                local_intensity[0] * arc_length,
                local_intensity[1] * arc_length,
                local_intensity[2] * arc_length,
            ];
            // Moment about node i plus transport to the global origin.
            let moment_arm = [
                geometry.radius
                    * geometry.radius
                    * (geometry.included_angle.sin() - geometry.included_angle),
                geometry.radius * geometry.radius * (1.0 - geometry.included_angle.cos()),
                0.0,
            ];
            let moment_about_i_local = cross(moment_arm, local_intensity);
            let total_force = rotate_to_global(&geometry.local_axes, total_force_local);
            let moment_about_i = rotate_to_global(&geometry.local_axes, moment_about_i_local);
            let position_i = element.node_i.coordinates;
            let moment_about_origin = [
                moment_about_i[0] + position_i[1] * total_force[2] - position_i[2] * total_force[1],
                moment_about_i[1] + position_i[2] * total_force[0] - position_i[0] * total_force[2],
                moment_about_i[2] + position_i[0] * total_force[1] - position_i[1] * total_force[0],
            ];

            let position_j = element.node_j.coordinates;
            for axis in 0..3 {
                let force_sum = loads[axis] + loads[DOF_PER_NODE + axis];
                assert_close(force_sum, total_force[axis]);
            }
            let nodal_moment = |axis: usize| -> f64 {
                let force_i = [loads[0], loads[1], loads[2]];
                let force_j = [loads[6], loads[7], loads[8]];
                let arm_i = cross(position_i, force_i);
                let arm_j = cross(position_j, force_j);
                loads[3 + axis] + loads[DOF_PER_NODE + 3 + axis] + arm_i[axis] + arm_j[axis]
            };
            for axis in 0..3 {
                assert_close(nodal_moment(axis), moment_about_origin[axis]);
            }
        }
    }

    // Independent longhand quarter-circle checks: the anchored arc solved
    // under the consistent load vector must reproduce the unit-load-theorem
    // tip deflections written out longhand. In-plane uniform w_y:
    //   M_q = R^2 w_y (1 - sin - (pi/2 - theta) cos), N_q = R w_y (pi/2 - theta) cos,
    //   u_x = w_y [k_in R^4 (3 - 7 pi/8) / EI - R^2 pi / (8 EA)]
    //   u_y = w_y [k_in R^4 (pi^2/16 - 1/4) / EI + R^2 (pi^2/16 + 1/4) / EA]
    //   r_z = k_in R^3 w_y (pi/2 - 2) / EI
    // Uniform w_x adds r_z = -k_in R^3 w_x (2 - pi/2) / EI. Out-of-plane w_z:
    //   u_z = R^4 w_z [k_out / (2 EI) + (pi^2/8 - pi/2 + 1/2) / GJ].
    #[test]
    fn anchored_arc_under_consistent_uniform_load_matches_longhand_tip_deflection() {
        for (k_in, k_out) in [(1.0, 1.0), (2.0, 1.75)] {
            let element = quarter_circle_element(k_in, k_out);
            let bending = ELASTIC_MODULUS * SECOND_MOMENT;
            let torsion = SHEAR_MODULUS * TORSION_CONSTANT;
            let axial = ELASTIC_MODULUS * AREA;
            let r = RADIUS;
            let (w_x, w_y, w_z) = (4.0, -13.0, -9.0);

            let loads = element
                .consistent_uniform_nodal_loads([w_x, w_y, w_z])
                .unwrap();
            let displacements = anchored_solution(&element, &loads);

            let expected_ux = w_y
                * (k_in * r.powi(4) * (3.0 - 7.0 * PI / 8.0) / bending
                    - r * r * PI / (8.0 * axial))
                + w_x
                    * (k_in * r.powi(4) * (1.25 - PI / 2.0 + PI * PI / 16.0) / bending
                        + r * r * (PI * PI / 16.0 - 0.25) / axial);
            let expected_uy = w_y
                * (k_in * r.powi(4) * (PI * PI / 16.0 - 0.25) / bending
                    + r * r * (PI * PI / 16.0 + 0.25) / axial)
                + w_x * (k_in * r.powi(4) * (PI / 8.0) / bending - r * r * PI / (8.0 * axial));
            let expected_rz =
                k_in * r.powi(3) * (w_y * (PI / 2.0 - 2.0) - w_x * (2.0 - PI / 2.0)) / bending;
            let expected_uz = r.powi(4)
                * w_z
                * (k_out * 0.5 / bending + (PI * PI / 8.0 - PI / 2.0 + 0.5) / torsion);
            let expected_rx =
                r.powi(3) * w_z * (k_out * 0.5 / bending - (PI / 2.0 - 1.5) / torsion);
            let expected_ry = r.powi(3)
                * w_z
                * (k_out * (1.0 - 0.25 * PI) / bending + (1.0 - 0.25 * PI) / torsion);

            assert_close(displacements[DOF_PER_NODE], expected_ux);
            assert_close(displacements[DOF_PER_NODE + 1], expected_uy);
            assert_close(displacements[DOF_PER_NODE + 2], expected_uz);
            assert_close(displacements[DOF_PER_NODE + 3], expected_rx);
            assert_close(displacements[DOF_PER_NODE + 4], expected_ry);
            assert_close(displacements[DOF_PER_NODE + 5], expected_rz);
        }
    }

    // Section resultants must close the equilibrium chain: at fraction 1
    // they equal the node-j end force in the section frame, at fraction 0
    // the negated node-i end force, and at the free tip of an unloaded-end
    // cantilever the interior values reduce to the longhand distributed
    // segment actions.
    #[test]
    fn arc_section_resultants_match_end_forces_and_longhand_interior() {
        let element = quarter_circle_element(2.0, 1.75);
        let (w_x, w_y, w_z) = (4.0, -13.0, -9.0);
        let intensity = [w_x, w_y, w_z];
        let mut force = element.consistent_uniform_nodal_loads(intensity).unwrap();
        // Add a nodal tip load so the end forces are non-trivial.
        force[DOF_PER_NODE + 1] += 500.0;
        force[DOF_PER_NODE + 3] += 40.0;
        let displacements = anchored_solution(&element, &force);
        let equivalent = element.consistent_uniform_nodal_loads(intensity).unwrap();
        let true_forces = end_forces(&element, &displacements, &equivalent);

        let node_j_force = [
            true_forces[6],
            true_forces[7],
            true_forces[8],
            true_forces[9],
            true_forces[10],
            true_forces[11],
        ];
        let geometry = element.geometry().unwrap();
        let phi = geometry.included_angle;

        // Fraction 1: the section frame at node j is [t(phi), -r(phi), z].
        let at_j = element
            .arc_section_resultants(1.0, node_j_force, intensity)
            .unwrap();
        let tangent_j = [-phi.sin(), phi.cos(), 0.0];
        let inward_j = [-phi.cos(), -phi.sin(), 0.0];
        let normal = [0.0, 0.0, 1.0];
        let force_j = [node_j_force[0], node_j_force[1], node_j_force[2]];
        let moment_j = [node_j_force[3], node_j_force[4], node_j_force[5]];
        assert_close(at_j[0], dot(force_j, tangent_j));
        assert_close(at_j[1], dot(force_j, inward_j));
        assert_close(at_j[2], dot(force_j, normal));
        assert_close(at_j[3], dot(moment_j, tangent_j));
        assert_close(at_j[4], dot(moment_j, inward_j));
        assert_close(at_j[5], dot(moment_j, normal));

        // Fraction 0: whole-element equilibrium gives the negated node-i force.
        let at_i = element
            .arc_section_resultants(0.0, node_j_force, intensity)
            .unwrap();
        let tangent_i = [0.0, 1.0, 0.0];
        let inward_i = [-1.0, 0.0, 0.0];
        let force_i = [true_forces[0], true_forces[1], true_forces[2]];
        let moment_i = [true_forces[3], true_forces[4], true_forces[5]];
        assert_close(at_i[0], -dot(force_i, tangent_i));
        assert_close(at_i[1], -dot(force_i, inward_i));
        assert_close(at_i[2], -dot(force_i, normal));
        assert_close(at_i[3], -dot(moment_i, tangent_i));
        assert_close(at_i[4], -dot(moment_i, inward_i));
        assert_close(at_i[5], -dot(moment_i, normal));

        // Free-tip cantilever under the distributed load only: node-j end
        // forces vanish and the interior section actions are the longhand
        // segment-equilibrium values at theta = pi/4.
        let distributed_only = element.consistent_uniform_nodal_loads(intensity).unwrap();
        let free_displacements = anchored_solution(&element, &distributed_only);
        let free_forces = end_forces(&element, &free_displacements, &distributed_only);
        for dof in 0..DOF_PER_NODE {
            assert!(
                free_forces[DOF_PER_NODE + dof].abs() <= 1.0e-9 * 500.0,
                "free tip must carry no end force, got {}",
                free_forces[DOF_PER_NODE + dof]
            );
        }
        let midpoint = element
            .arc_section_resultants(0.5, [0.0; DOF_PER_NODE], intensity)
            .unwrap();
        let theta = phi / 2.0;
        let remaining = phi - theta;
        let r = RADIUS;
        // Longhand distributed segment actions at theta (same fields the
        // deflection integrals used, evaluated pointwise).
        let a_x = (phi.sin() - theta.sin()) - remaining * theta.cos();
        let a_y = (theta.cos() - phi.cos()) - remaining * theta.sin();
        let expected_in_plane = r * r * (a_x * w_y - a_y * w_x);
        // The section frame's y axis is the inward radial (-r), so the
        // reported bending_y is the negated radial moment component.
        let expected_out_of_plane = -(r * r * w_z * (1.0 - (remaining).cos()));
        let expected_torsion = r * r * w_z * (remaining - remaining.sin());
        let expected_axial = r * remaining * (w_y * theta.cos() - w_x * theta.sin());
        assert_close(midpoint[5], expected_in_plane);
        assert_close(midpoint[4], expected_out_of_plane);
        assert_close(midpoint[3], expected_torsion);
        assert_close(midpoint[0], expected_axial);
    }

    #[test]
    fn consistent_uniform_load_and_stations_reject_invalid_inputs() {
        let element = quarter_circle_element(1.0, 1.0);
        assert!(matches!(
            element
                .consistent_uniform_nodal_loads([f64::NAN, 0.0, 0.0])
                .unwrap_err(),
            CurvedBendError::Kernel(FrameKernelError::NonFiniteInput {
                name: "uniform_load_intensity",
                ..
            })
        ));
        assert!(matches!(
            element
                .arc_section_resultants(1.5, [0.0; DOF_PER_NODE], [0.0; 3])
                .unwrap_err(),
            CurvedBendError::Kernel(FrameKernelError::NonFiniteInput {
                name: "arc_station_fraction",
                ..
            })
        ));
        assert!(matches!(
            element
                .arc_section_resultants(-0.1, [0.0; DOF_PER_NODE], [0.0; 3])
                .unwrap_err(),
            CurvedBendError::Kernel(FrameKernelError::NonFiniteInput {
                name: "arc_station_fraction",
                ..
            })
        ));
        assert!(matches!(
            element
                .arc_section_resultants(0.5, [0.0, f64::INFINITY, 0.0, 0.0, 0.0, 0.0], [0.0; 3])
                .unwrap_err(),
            CurvedBendError::Kernel(FrameKernelError::NonFiniteInput {
                name: "node_j_force_global",
                ..
            })
        ));
    }

    #[test]
    fn constructor_rejects_repeated_node_index() {
        let node_i = FrameNode::new(0, [RADIUS, 0.0, 0.0]).unwrap();
        let node_j = FrameNode::new(0, [0.0, RADIUS, 0.0]).unwrap();
        let error = CurvedBendMacroElement::new(
            node_i,
            node_j,
            RADIUS,
            [1.0, 1.0, 0.0],
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
                RADIUS,
                [1.0, 1.0, 0.0],
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
        let build = |i: [f64; 3], j: [f64; 3], radius: f64, y: [f64; 3]| {
            CurvedBendMacroElement::new(
                FrameNode::new(0, i).unwrap(),
                FrameNode::new(1, j).unwrap(),
                radius,
                y,
                ELASTIC_MODULUS,
                SHEAR_MODULUS,
                AREA,
                SECOND_MOMENT,
                TORSION_CONSTANT,
                1.0,
                1.0,
            )
        };

        // Radius cannot span the chord (R < L/2) or spans it exactly (phi = pi).
        assert!(matches!(
            build([1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], 0.9, [0.0, 1.0, 0.0]).unwrap_err(),
            CurvedBendError::RadiusCannotSpanChord { .. }
        ));
        assert!(matches!(
            build([1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], 1.0, [0.0, 1.0, 0.0]).unwrap_err(),
            CurvedBendError::RadiusCannotSpanChord { .. }
        ));
        // Coincident end coordinates on distinct nodes.
        assert_eq!(
            build([1.0, 0.0, 0.0], [1.0, 0.0, 0.0], 1.0, [0.0, 1.0, 0.0]).unwrap_err(),
            CurvedBendError::DegenerateArc {
                detail: "arc end nodes coincide",
            }
        );
        // Included angle below the 1e-9 rad floor (phi ~ 2e-10).
        assert!(matches!(
            build([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 5.0e9, [0.0, 1.0, 0.0]).unwrap_err(),
            CurvedBendError::IncludedAngleOutOfRange { .. }
        ));
        // y_reference parallel to the chord: the bend plane is undefined.
        assert!(matches!(
            build([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 1.0, [2.0, 0.0, 0.0]).unwrap_err(),
            CurvedBendError::DegenerateArc { .. }
        ));
        // Non-finite or non-positive radius and non-finite y_reference.
        assert!(matches!(
            build([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], f64::NAN, [1.0, 1.0, 0.0]).unwrap_err(),
            CurvedBendError::Kernel(FrameKernelError::NonFiniteInput {
                name: "bend_radius",
                ..
            })
        ));
        assert!(matches!(
            build([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], 0.0, [1.0, 1.0, 0.0]).unwrap_err(),
            CurvedBendError::Kernel(FrameKernelError::NonPositiveInput {
                name: "bend_radius",
                ..
            })
        ));
        assert!(matches!(
            build([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], 1.0, [f64::NAN, 1.0, 0.0]).unwrap_err(),
            CurvedBendError::Kernel(FrameKernelError::NonFiniteInput {
                name: "y_reference",
                ..
            })
        ));
    }

    // --- Shared arc definition and kink (T4-I11 D-I) ---

    #[test]
    fn arc_geometry_and_kink_describe_the_quarter_circle() {
        let frame = arc_geometry([-RADIUS, RADIUS, 0.0], RADIUS, [1.0, 1.0, 0.0]).unwrap();
        assert_close(frame.included_angle, PI / 2.0);
        let half = std::f64::consts::FRAC_1_SQRT_2;
        for (actual, expected) in frame
            .tangent_i
            .iter()
            .chain(&frame.tangent_j)
            .chain(&frame.bow_normal)
            .zip([0.0, 1.0, 0.0, -1.0, 0.0, 0.0, half, half, 0.0])
        {
            assert_close(*actual, expected);
        }
        // The element's own frame is this definition.
        let element = quarter_circle_element(1.0, 1.0);
        assert_eq!(element.arc_frame().unwrap(), frame);
        assert_eq!(
            element.end_tangents().unwrap(),
            [frame.tangent_i, frame.tangent_j]
        );
        // kink: exact zero for equal directions, pi/2 for orthogonal ones, and
        // relative accuracy at tiny and near-pi angles.
        assert_eq!(kink(frame.tangent_j, [-2.0, 0.0, 0.0]).unwrap(), 0.0);
        assert_close(kink([1.0, 0.0, 0.0], [0.0, 3.0, 0.0]).unwrap(), PI / 2.0);
        let tiny = kink([1.0, 0.0, 0.0], [1.0, 1.0e-9, 0.0]).unwrap();
        assert!((tiny - 1.0e-9).abs() <= 1e-15 * 1.0e-9, "{tiny}");
        let opposite = kink([1.0, 0.0, 0.0], [-1.0, 1.0e-9, 0.0]).unwrap();
        assert!((PI - opposite - 1.0e-9).abs() <= 1e-15, "{opposite}");
        assert!(kink([0.0; 3], [1.0, 0.0, 0.0]).is_err());
    }

    // --- Arc end tangents (used by the preview-physics bend checks) ---

    #[test]
    fn end_tangents_match_arc_frame() {
        // Quarter circle with local frame == global frame: t_i = (0, 1, 0),
        // t_j = (-1, 0, 0).
        let element = quarter_circle_element(1.0, 1.0);
        let [tangent_i, tangent_j] = element.end_tangents().unwrap();
        for (actual, expected) in tangent_i.iter().zip([0.0, 1.0, 0.0]) {
            assert_close(*actual, expected);
        }
        for (actual, expected) in tangent_j.iter().zip([-1.0, 0.0, 0.0]) {
            assert_close(*actual, expected);
        }

        // General placement: unit tangents perpendicular to their end
        // radials and to the bend-plane normal, oriented i -> j.
        let shifted = CurvedBendMacroElement::new(
            FrameNode::new(0, [RADIUS + 0.4, 0.7, -0.3]).unwrap(),
            FrameNode::new(1, [0.4, 0.7 + RADIUS, -0.3]).unwrap(),
            RADIUS,
            [1.0, 1.0, 0.0],
            ELASTIC_MODULUS,
            SHEAR_MODULUS,
            AREA,
            SECOND_MOMENT,
            TORSION_CONSTANT,
            2.5,
            1.75,
        )
        .unwrap();
        let geometry = shifted.geometry().unwrap();
        let [tangent_i, tangent_j] = shifted.end_tangents().unwrap();
        let center = [0.4, 0.7, -0.3];
        let radial_i = subtract(shifted.node_i.coordinates, center);
        let radial_j = subtract(shifted.node_j.coordinates, center);
        for tangent in [tangent_i, tangent_j] {
            assert_close(norm(tangent), 1.0);
            assert_close(dot(tangent, geometry.local_axes[2]), 0.0);
        }
        assert_close(dot(tangent_i, radial_i), 0.0);
        assert_close(dot(tangent_j, radial_j), 0.0);
        // i -> j orientation: each tangent has positive component along the
        // chord direction for an included angle below pi.
        let chord = subtract(shifted.node_j.coordinates, shifted.node_i.coordinates);
        assert!(dot(tangent_i, chord) > 0.0);
        assert!(dot(tangent_j, chord) > 0.0);
    }
}
