//! T4-U4b section quantities in binary64 (DEL-04-07 Design
//! `u4b-u5b-implementation.md` §2.3–§2.6; definitions in
//! `validation/references/t4_u4b/README.md`).
//!
//! Each published value is a fixed sequence of correctly rounded binary64
//! operations, written out below, so a reader can recompute it bit for bit
//! from the publication's own rows. Magnitudes use the shared correctly
//! rounded `norm2` (E15), never the platform `hypot`.
//!
//! - Section normal-stress extremes σ⁺/σ⁻ (MPa) from the component stress rows
//!   of one station and basis, with the fibre rule (§2.3).
//! - The Saint-Venant transverse-shear factor c_b and maximum τ_v,max (MPa)
//!   of the straight annulus (`local_straight_annulus_on_arc` on arcs), §2.4.
//! - The toroidal membrane hoop at β = 0°, 90° and 180° and the radial surface
//!   values (Pa), guarded by the torus domain R > r_o (§2.5).
//! - The directional intensified bending station value (Pa), §2.6.
//!
//! **The sequences are normative.** Each function documents its sequence of
//! correctly rounded operations; a reader recomputes the published value bit
//! for bit with exactly that sequence, never with a tolerance. This matters
//! most for the toroidal intrados: near R = r_o the quotient
//! (2R − r_i)/(2R − (r_i + r_o)) amplifies rounding by about 2R/t (about 80ε
//! at D = 1.2192 m, t = 6.35 mm, R one ulp above r_o), so an equivalent but
//! reassociated sequence can differ by many ulps. The expected bits of every
//! sequence, near-edge hoops included, are frozen in
//! `tests/u4b_section_bits/section_quantity_bits.json`, computed independently
//! in exact rational arithmetic by `gen_section_quantity_bits.py`.
//!
//! These are code-neutral mechanics quantities; no code stress, allowable,
//! SIF table or flexibility formula is implied. The directional factors are
//! user inputs.

use open_pipe_stress_correct_norm::norm2;

/// RN(x / 1e6): a Pa row in MPa. On a pressurized pressure-2 member σ_N is
/// RN(`pipe_axial_membrane_stress_v2` / 1e6) of the same station and basis.
pub fn pa_to_mpa(pa: f64) -> f64 {
    pa / 1e6
}

/// Signed section normal-stress extremes at one station and basis (MPa).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NormalStressExtremes {
    pub sigma_plus: f64,
    pub sigma_minus: f64,
}

impl NormalStressExtremes {
    /// max(|σ⁺|, |σ⁻|).
    pub fn max_abs(&self) -> f64 {
        self.sigma_plus.abs().max(self.sigma_minus.abs())
    }
}

/// σ⁺ = RN(σ_N + h) and σ⁻ = RN(σ_N − h), with h = `norm2`(σ_y, σ_z): σ_N is
/// the axial (or membrane) normal stress row and σ_y, σ_z the
/// `element_local_bending_normal_stress_y`/`_z` rows (MPa), all of one
/// station and basis.
pub fn normal_stress_extremes(sigma_n: f64, sigma_y: f64, sigma_z: f64) -> NormalStressExtremes {
    let h = norm2(sigma_y, sigma_z);
    NormalStressExtremes {
        sigma_plus: sigma_n + h,
        sigma_minus: sigma_n - h,
    }
}

/// The fibres of σ⁺ and σ⁻ on the outer surface, (y, z) in the section's local
/// frame: ±r_o(−σ_z, σ_y)/h with h = `norm2`(σ_y, σ_z), each coordinate
/// RN(r_o·RN(±σ/h)). `None` when h = 0: every outer fibre ties.
pub fn normal_extreme_fibres(
    outer_radius: f64,
    sigma_y: f64,
    sigma_z: f64,
) -> Option<[[f64; 2]; 2]> {
    let h = norm2(sigma_y, sigma_z);
    if h == 0.0 {
        return None;
    }
    let plus = [outer_radius * (-sigma_z / h), outer_radius * (sigma_y / h)];
    Some([plus, [-plus[0], -plus[1]]])
}

/// Why a quantity in this module is not formed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SectionQuantityError {
    /// ν outside −1 < ν < 1/2, where the Saint-Venant field is defined.
    PoissonOutOfDomain { nu: f64 },
    /// R does not exceed r_o (`ARC_TOROIDAL_HOOP_DOMAIN`).
    ToroidalDomain { bend_radius: f64, outer_radius: f64 },
    /// A non-finite or out-of-range section or load input.
    InvalidInput { name: &'static str },
}

fn positive(name: &'static str, value: f64) -> Result<f64, SectionQuantityError> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(SectionQuantityError::InvalidInput { name })
    }
}

/// c_b = RN(num/den), with
/// num = RN(RN(RN(3+2ν)·RN(r_o·r_o)) + RN(RN(1+2ν)·RN(r_i,b·r_i,b))) and
/// den = RN(RN(4·RN(1+ν))·I_b) (m⁻²); r_o = D/2, r_i,b and I_b of the basis
/// section. Requires −1 < ν < 1/2 and 0 < r_i,b < r_o.
pub fn saint_venant_shear_factor(
    nu: f64,
    outer_radius: f64,
    inner_radius: f64,
    second_moment: f64,
) -> Result<f64, SectionQuantityError> {
    if !(nu.is_finite() && nu > -1.0 && nu < 0.5) {
        return Err(SectionQuantityError::PoissonOutOfDomain { nu });
    }
    let ro = positive("outer_radius", outer_radius)?;
    let ri = positive("inner_radius", inner_radius)?;
    let inertia = positive("second_moment", second_moment)?;
    if ri >= ro {
        return Err(SectionQuantityError::InvalidInput {
            name: "inner_radius",
        });
    }
    let num = (3.0 + 2.0 * nu) * (ro * ro) + (1.0 + 2.0 * nu) * (ri * ri);
    let den = (4.0 * (1.0 + nu)) * inertia;
    let factor = num / den;
    if factor.is_finite() && factor > 0.0 {
        Ok(factor)
    } else {
        Err(SectionQuantityError::InvalidInput {
            name: "saint_venant_shear_factor",
        })
    }
}

/// c_b of [`saint_venant_shear_factor`] from x, a binary64 value of
/// 2G/E = 1/(1+ν*) for the exact ν* = E/(2G) − 1, instead of a rounded ν (the
/// preview route below RN(E/(2G)) = 1/2, where 1+ν formed from the rounded
/// ν = RN(RN(E/(2G)) − 1) is inexact), with no 1+ν formed: c_b = RN(num/den), with
/// num = RN(RN(2·RN(RN(r_o·r_o) + RN(r_i,b·r_i,b))) + RN(x·RN(RN(r_o − r_i,b)·RN(r_o + r_i,b))))
/// and den = RN(4·I_b) (m⁻²). x is open_pipe_stress_frame_kernel's
/// `shear::moduli_ratio_terms` x = RN(RN(2·G)/E).
///
/// **Identity** (exact arithmetic, x* = 1/(1+ν*)): (3+2ν*)/(1+ν*) = 2 + x* and
/// (1+2ν*)/(1+ν*) = 2 − x* (each (a+bν*)/(1+ν*) is a·x* + b·t*, t* = 1 − x*), so
/// ((3+2ν*)r_o² + (1+2ν*)r_i²)/(4(1+ν*)I) = ((2+x*)r_o² + (2−x*)r_i²)/(4I)
/// = (2(r_o² + r_i²) + x*(r_o − r_i)(r_o + r_i))/(4I). Every term is
/// nonnegative for x* > 0, so no sum cancels, even for ν* < −1/2 where 1+2ν* < 0.
///
/// **Error.** With u = 2^-53, x = x*(1 + e), |e| ≤ u, and r_o, r_i,b, I_b
/// taken as exact, to first order: the squares u each and their sum 2u (2·
/// exact); r_o − r_i,b and r_o + r_i,b u each, their product 3u, times x 5u;
/// num 6u; den exact; |c_b/c_b* − 1| ≤ 7u for every x > 0, barring overflow
/// and underflow. Requires a finite x > 2/3 (ν < 1/2) and 0 < r_i,b < r_o.
/// Its bits are not in `section_quantity_bits.json`; product_physics's
/// `u4b_preview_shear_factor_from_moduli_is_within_its_bound` binds it to
/// exact rational values.
pub fn saint_venant_shear_factor_of_ratio(
    ratio: f64,
    outer_radius: f64,
    inner_radius: f64,
    second_moment: f64,
) -> Result<f64, SectionQuantityError> {
    if !(ratio.is_finite() && ratio > 2.0 / 3.0) {
        return Err(SectionQuantityError::InvalidInput {
            name: "modulus_ratio",
        });
    }
    let ro = positive("outer_radius", outer_radius)?;
    let ri = positive("inner_radius", inner_radius)?;
    let inertia = positive("second_moment", second_moment)?;
    if ri >= ro {
        return Err(SectionQuantityError::InvalidInput {
            name: "inner_radius",
        });
    }
    let num = 2.0 * (ro * ro + ri * ri) + ratio * ((ro - ri) * (ro + ri));
    let den = 4.0 * inertia;
    let factor = num / den;
    if factor.is_finite() && factor > 0.0 {
        Ok(factor)
    } else {
        Err(SectionQuantityError::InvalidInput {
            name: "saint_venant_shear_factor",
        })
    }
}

/// τ_v,max = RN(RN(t·c_b)/1e6) (MPa), t = `norm2`(V_y, V_z) (N): the largest
/// Saint-Venant transverse shear, on the inner surface at the two fibres
/// normal to V.
pub fn transverse_shear_maximum_mpa(shear_y: f64, shear_z: f64, factor: f64) -> f64 {
    norm2(shear_y, shear_z) * factor / 1e6
}

/// The toroidal membrane hoop at the intrados (β = 0°, `y_pos`), the crowns
/// (90°, `z_pos`) and the extrados (180°, `y_neg`), in Pa.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToroidalHoop {
    pub intrados: f64,
    pub crown: f64,
    pub extrados: f64,
}

/// The torus domain R > r_o, by exact comparison of the binary64 values.
fn torus_domain(bend_radius: f64, outer_radius: f64) -> Result<(), SectionQuantityError> {
    positive("bend_radius", bend_radius)?;
    positive("outer_radius", outer_radius)?;
    if bend_radius > outer_radius {
        Ok(())
    } else {
        Err(SectionQuantityError::ToroidalDomain {
            bend_radius,
            outer_radius,
        })
    }
}

/// σ_h(β) = (p r_i,b/t_b)(2R − r_i,b cos β)/(2R − (r_i,b + r_o) cos β) at
/// cos β = 1, 0, −1 (`bore_load_transferred_toroidal_membrane_v1`), on the
/// basis bore r_i,b and wall t_b. The sequence: a = RN(RN(p·r_i,b)/t_b),
/// s = RN(r_i,b + r_o); crown = a; intrados = RN(a·RN(RN(2R − r_i,b)/RN(2R − s)));
/// extrados = RN(a·RN(RN(2R + r_i,b)/RN(2R + s))). Refused outside R > r_o.
pub fn toroidal_membrane_hoop(
    pressure: f64,
    inner_radius: f64,
    wall: f64,
    outer_radius: f64,
    bend_radius: f64,
) -> Result<ToroidalHoop, SectionQuantityError> {
    if !pressure.is_finite() {
        return Err(SectionQuantityError::InvalidInput { name: "pressure" });
    }
    let ri = positive("inner_radius", inner_radius)?;
    let t = positive("wall", wall)?;
    torus_domain(bend_radius, outer_radius)?;
    let a = pressure * ri / t;
    let two_r = 2.0 * bend_radius;
    let sum = ri + outer_radius;
    let intrados_den = two_r - sum;
    if !(intrados_den > 0.0) {
        return Err(SectionQuantityError::ToroidalDomain {
            bend_radius,
            outer_radius,
        });
    }
    let hoop = ToroidalHoop {
        intrados: a * ((two_r - ri) / intrados_den),
        crown: a,
        extrados: a * ((two_r + ri) / (two_r + sum)),
    };
    if hoop.intrados.is_finite() && hoop.crown.is_finite() && hoop.extrados.is_finite() {
        Ok(hoop)
    } else {
        Err(SectionQuantityError::InvalidInput {
            name: "toroidal_membrane_hoop",
        })
    }
}

/// The toroidal radial surface values (Pa): (−p at the basis bore, 0 at r_o),
/// published exactly where the hoop is (R > r_o).
pub fn toroidal_radial_surface(
    pressure: f64,
    outer_radius: f64,
    bend_radius: f64,
) -> Result<(f64, f64), SectionQuantityError> {
    if !pressure.is_finite() {
        return Err(SectionQuantityError::InvalidInput { name: "pressure" });
    }
    torus_domain(bend_radius, outer_radius)?;
    Ok((-pressure, 0.0))
}

/// intensified = RN(1e6·`norm2`(RN(i_ip·σ_z), RN(i_op·σ_y))) (Pa), from the
/// station's bending stress rows σ_y, σ_z (MPa) and the captured binary64
/// directional factors (i_ip multiplies M_z, i_op multiplies M_y).
pub fn directional_intensified_bending_pa(
    in_plane_factor: f64,
    out_of_plane_factor: f64,
    sigma_y: f64,
    sigma_z: f64,
) -> f64 {
    1e6 * norm2(in_plane_factor * sigma_z, out_of_plane_factor * sigma_y)
}

/// Test-only field functions (§2.2: not published; §7.1: the stress_recovery
/// unit tests compare them with the oracle's field and fibre values).
#[cfg(test)]
pub(crate) mod fields {
    /// The Saint-Venant flexure tractions (τ_xy, τ_xz) on the +x face of the
    /// straight annulus at (y, z) = r(cos β, sin β), driven by (V_y, V_z):
    /// τ_r = −C c (r² − r_o²)(r² − r_i²)/r² (V·e_r) and τ_β = C g(r) (V·e_β),
    /// C = 1/(2(1+ν)I), c = (3+2ν)/4,
    /// g(r) = c(r_o² + r_i² + r_o² r_i²/r²) − (1 − 2ν) r²/4.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn saint_venant_field(
        shear_y: f64,
        shear_z: f64,
        nu: f64,
        outer_radius: f64,
        inner_radius: f64,
        second_moment: f64,
        r: f64,
        cos_beta: f64,
        sin_beta: f64,
    ) -> (f64, f64) {
        let big_c = 1.0 / (2.0 * (1.0 + nu) * second_moment);
        let c = (3.0 + 2.0 * nu) / 4.0;
        let (ro2, ri2, r2) = (
            outer_radius * outer_radius,
            inner_radius * inner_radius,
            r * r,
        );
        let g = c * (ro2 + ri2 + ro2 * ri2 / r2) - (1.0 - 2.0 * nu) * r2 / 4.0;
        let v_r = shear_y * cos_beta + shear_z * sin_beta;
        let v_beta = -shear_y * sin_beta + shear_z * cos_beta;
        let tau_r = -big_c * c * (r2 - ro2) * (r2 - ri2) / r2 * v_r;
        let tau_beta = big_c * g * v_beta;
        (
            tau_r * cos_beta - tau_beta * sin_beta,
            tau_r * sin_beta + tau_beta * cos_beta,
        )
    }

    /// σ_x = N_eff/A + (M_y z − M_z y)/I at a fibre (y, z) (STRESS_REFERENCE §1).
    pub(crate) fn fibre_normal(
        axial: f64,
        area: f64,
        m_y: f64,
        m_z: f64,
        inertia: f64,
        y: f64,
        z: f64,
    ) -> f64 {
        axial / area + (m_y * z - m_z * y) / inertia
    }

    /// The general-β toroidal membrane hoop, for the diagonal oracle fibres.
    pub(crate) fn toroidal_hoop_at(
        pressure: f64,
        inner_radius: f64,
        wall: f64,
        outer_radius: f64,
        bend_radius: f64,
        cos_beta: f64,
    ) -> f64 {
        let two_r = 2.0 * bend_radius;
        (pressure * inner_radius / wall) * (two_r - inner_radius * cos_beta)
            / (two_r - (inner_radius + outer_radius) * cos_beta)
    }
}

#[cfg(test)]
#[path = "section_stress_quantities_tests.rs"]
mod tests;
