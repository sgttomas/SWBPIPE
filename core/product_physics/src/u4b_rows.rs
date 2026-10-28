//! T4-U4b (DEL-04-07 Design `u4b-u5b-implementation.md` §2): the section
//! quantities published on both ordinary routes.
//!
//! - On every member, at the five stations and each section basis: the
//!   signed normal-stress extremes σ⁺/σ⁻ (`element_local_normal_stress_extreme`)
//!   formed from the published stress rows of the same station and basis
//!   (§2.3), and the Saint-Venant transverse-shear maximum
//!   (`element_local_transverse_shear_stress_maximum`) formed from the
//!   published shear rows: the j-side section rows inside, the end-force rows
//!   at the ends (§2.4).
//! - On realized arcs: the certified arc maximum
//!   (`pipe_elastic_normal_stress_maximum_v2`, frame kernel
//!   `certify_arc_stress_maximum`) with its closed fourteen-key record (§2.3);
//!   with the directional SIF pair, the intensified rows and their along-arc
//!   maximum (§2.6); on a pressurized pressure-2 arc with R > r_o, the toroidal
//!   membrane hoop and radial rows (§2.5). R <= r_o is the one domain outcome:
//!   the warning `ARC_TOROIDAL_HOOP_DOMAIN` and no hoop or radial row. Any
//!   other hoop failure (a non-finite pressure, a nonpositive bore or wall, a
//!   non-finite value from finite inputs) blocks the run by name
//!   (`ARC_TOROIDAL_HOOP_EVALUATION_FAILED`, severity blocking), so the case
//!   publishes no row (`blocked_envelope`).
//!
//! Every station value is a fixed sequence of correctly rounded binary64
//! operations on published rows (`stress_recovery::section_stress_quantities`),
//! so a reader recomputes it bit for bit. These are code-neutral mechanics
//! quantities, never code or equivalent stresses.
use super::*;
use open_pipe_stress_frame_kernel::structural::{
    certify_arc_stress_maximum, ArcStressMaximumProblem, ArcStressMeasure,
    ArcUniformIntensity, CertifiedArcStressMaximum,
};
use open_pipe_stress_stress_recovery::section_stress_quantities as q;
use pressure_exact::SectionBasis;

pub(crate) const EXTREME_KIND: &str = "element_local_normal_stress_extreme";
pub(crate) const SHEAR_MAX_KIND: &str = "element_local_transverse_shear_stress_maximum";
pub(crate) const HOOP_KIND: &str = "pipe_toroidal_membrane_hoop_stress_v1";
pub(crate) const RADIAL_KIND: &str = "pipe_toroidal_radial_surface_stress_v1";
pub(crate) const DIRECTIONAL_KIND: &str = "curved_bend_directional_intensified_bending_stress_v1";
pub(crate) const DIRECTIONAL_MAX_KIND: &str = "curved_bend_directional_intensified_bending_maximum_v1";
pub(crate) const MAXIMUM_KIND: &str = "pipe_elastic_normal_stress_maximum_v2";

pub(crate) const EXTREMES_BASIS: &str = "section_extremes_from_published_stress_rows_v1";
pub(crate) const SHEAR_STRAIGHT_BASIS: &str = "saint_venant_annulus_flexure_v1";
pub(crate) const SHEAR_ARC_BASIS: &str = "local_straight_annulus_on_arc";
pub(crate) const HOOP_BASIS: &str = "bore_load_transferred_toroidal_membrane_v1";
pub(crate) const DIRECTIONAL_BASIS: &str = "user_directional_sif_times_tangent_frame_bending_stress_v1";
pub(crate) const MAXIMUM_BASIS: &str = "recovered_from_open_mechanics_stress_components";

pub(crate) const SIGMA_PLUS_SIGN: &str = "signed section normal-stress extreme sigma+ = N/A + hypot(My,Mz)/Z at the station (N_w/A_s on a pressurized pressure-2 member), formed from the published axial or membrane and bending stress rows of the same station and basis; at the outer-surface fibre r_o(-sigma_z, sigma_y)/hypot(sigma_y, sigma_z) of the section frame, every outer fibre tied when the bending rows are zero; not a code or equivalent stress";
pub(crate) const SIGMA_MINUS_SIGN: &str = "signed section normal-stress extreme sigma- = N/A - hypot(My,Mz)/Z at the station (N_w/A_s on a pressurized pressure-2 member), formed from the published axial or membrane and bending stress rows of the same station and basis; at the outer-surface fibre -r_o(-sigma_z, sigma_y)/hypot(sigma_y, sigma_z) of the section frame, every outer fibre tied when the bending rows are zero; not a code or equivalent stress";
/// pressure-2: the member's authored and validated nu.
pub(crate) const SHEAR_SIGN_EXACT: &str = "nonnegative Saint-Venant transverse shear maximum |V|((3+2nu)ro^2+(1+2nu)ri^2)/(4(1+nu)I) of the basis annulus, on the inner surface at the two fibres normal to V; V from the published j-side section shears inside and the end-force shears at the ends; nu is the member's authored nu; torsional shear separate";
/// preview-physics-2: nu = E/(2G) - 1 from the solve's moduli.
pub(crate) const SHEAR_SIGN_PREVIEW: &str = "nonnegative Saint-Venant transverse shear maximum |V|((3+2nu)ro^2+(1+2nu)ri^2)/(4(1+nu)I) of the section annulus, on the inner surface at the two fibres normal to V; V from the published j-side section shears inside and the end-force shears at the ends; nu = E/(2G) - 1 from the moduli the solve uses, assuming an isotropic material; torsional shear separate";
pub(crate) const SHEAR_ARC_SUFFIX: &str = "; on an arc the straight-annulus field is driven by the tangent-frame shears without curvature correction";
pub(crate) const HOOP_SIGN: &str = "tension-positive toroidal membrane hoop: the area average over the wall cut at the named section angle (intrados beta = 0, crown 90 deg, extrados 180 deg) of a complete torus in an axisymmetric membrane state under p on the basis bore; station-independent";
pub(crate) const RADIAL_SIGN: &str = "tension-positive radial stress of the torus section: -p at the basis bore and 0 at the outer surface";
pub(crate) const DIRECTIONAL_SIGN: &str = "nonnegative hypot(i_ip*Mz, i_op*My)/Z in the arc tangent frame with the user-entered directional SIFs of its intensified_measures record (i_ip on the in-plane Mz, i_op on the out-of-plane My); not a code stress; no flexibility factor, axial or torsion term; never combined";
pub(crate) const ARC_MAXIMUM_SIGN: &str = "nonnegative maximum |Nw/As|+hypot(My,Mz)/Z along the arc, bounded over the binary64 arc problem formed from the published end-j force and the uniform intensities; torsional shear remains separate; no code stress or equivalent stress claim";
pub(crate) const DIRECTIONAL_MAX_SIGN: &str = "nonnegative along-arc maximum of hypot(i_ip*Mz, i_op*My)/Z with the user-entered directional SIFs of its intensified_measures record, bounded over the binary64 arc problem formed from the published end-j force and the uniform intensities; never combined";

/// The arc maximum's closed record tokens (§2.3).
pub(crate) const ARC_APPROXIMATION: &str = "arc_closed_form_section_statics";
pub(crate) const ARC_DIRECTIONAL_APPROXIMATION: &str = "arc_closed_form_directional_intensified";
pub(crate) const ARC_COEFFICIENT_BASIS: &str = "arc_end_j_force_and_uniform_intensities_binary64_enclosures";
pub(crate) const ARC_ENCLOSURE_SCOPE: &str = "published_binary64_end_j_force_and_uniform_intensity_operand_enclosures; solution error is separate";

/// §2.4: the named warning of a preview member whose derived nu lies outside
/// the Saint-Venant domain (no transverse-shear rows on it).
pub(crate) const POISSON_OUT_OF_DOMAIN: &str = "TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN";
/// §2.5: the named warning of a pressurized arc with R not above r_o.
pub(crate) const TOROIDAL_DOMAIN: &str = "ARC_TOROIDAL_HOOP_DOMAIN";
pub(crate) const TOROIDAL_DOMAIN_TEXT: &str = "bend radius R does not exceed the outer radius; the toroidal membrane hoop is undefined";
/// §2.5: any other hoop or radial failure blocks the run by this name.
pub(crate) const TOROIDAL_EVALUATION_FAILED: &str = "ARC_TOROIDAL_HOOP_EVALUATION_FAILED";
pub(crate) const TOROIDAL_EVALUATION_TEXT: &str = "the toroidal hoop or radial value of a pressurized arc with R > r_o is not a finite binary64 value from finite inputs; no result is published";

/// §2.5: the static region statement of the two arc definitions.
pub(crate) fn arc_stress_definitions() -> serde_json::Value {
    serde_json::json!([
        {"definition": HOOP_BASIS, "statement": "the toroidal membrane hoop is the cut average of a complete torus in an axisymmetric membrane state under p on each basis bore, and the radial values are -p at the bore and 0 outside; ovalization, hoop bending from moments, bending boundary layers, tangent-end effects, the through-thickness distribution and transverse-shear resultants on the cuts are not represented"},
        {"definition": SHEAR_ARC_BASIS, "statement": "the transverse shear is the straight-annulus Saint-Venant field of each basis driven by the tangent-frame shears; the curvature correction of the shear field, ovalization and end effects are not represented"},
    ])
}

pub(crate) const STATIONS: [&str; 5] = ["end_i", "end_j", "quarter_1", "midspan", "quarter_3"];
/// ψ/φ of each station (the station rows' fractions).
pub(crate) fn station_fraction(location: &str) -> f64 {
    match location {
        "end_i" => 0.0,
        "quarter_1" => 0.25,
        "midspan" => 0.5,
        "quarter_3" => 0.75,
        _ => 1.0,
    }
}
pub(crate) fn id_location(location: &str) -> &str {
    match location {
        "end_i" => "end-i",
        "end_j" => "end-j",
        "quarter_1" => "quarter-1",
        "quarter_3" => "quarter-3",
        other => other,
    }
}

/// One section basis's annulus operands of the SV factor and the hoop (§2.4,
/// §2.5): r_o = D/2, the basis bore r_i,b, wall t_b, I_b, A_b and Z_b.
#[derive(Debug, Clone, Copy)]
pub(crate) struct BasisSection {
    pub basis: Option<SectionBasis>,
    pub outer_radius: f64,
    pub inner_radius: f64,
    pub wall: f64,
    pub inertia: f64,
    pub area: f64,
    pub modulus: f64,
}

/// The bases of one member: nominal and reduced on pressure-2 (each its own
/// annulus), the single section on preview-physics-2. The preview bore is
/// (D - 2t)/2 with t = t_n - c_m, exactly as `derive_pipe_section` forms the
/// inner diameter (halving is exact).
pub(crate) fn member_bases(section: &DerivedSection, exact: Option<pressure_exact::ExactSectionBases>) -> Vec<BasisSection> {
    match exact {
        Some(g) => SectionBasis::BOTH
            .into_iter()
            .map(|basis| {
                let s = g.basis(basis);
                BasisSection {
                    basis: Some(basis),
                    outer_radius: s.outer_radius_m(),
                    inner_radius: s.inner_radius_m(),
                    wall: s.effective_wall_thickness_m(),
                    inertia: s.second_moment_m4(),
                    area: s.wall_area_m2(),
                    modulus: s.section_modulus_m3(),
                }
            })
            .collect(),
        None => {
            let od = 2.0 * section.torsion_radius;
            vec![BasisSection {
                basis: None,
                outer_radius: section.torsion_radius,
                inner_radius: (od - 2.0 * section.wall_thickness) / 2.0,
                wall: section.wall_thickness,
                inertia: section.second_moment,
                area: section.area,
                modulus: section.section_modulus,
            }]
        }
    }
}

fn basis_suffix(basis: Option<SectionBasis>) -> &'static str {
    basis.map_or("", |b| b.basis_suffix())
}

/// The value of the one published row of `pipe` with this kind, location and
/// basis (component when given), or `None`.
fn published(results: &[ResultItem], pipe: &str, kind: &str, component: Option<&str>, location: &str, basis: Option<SectionBasis>) -> Option<f64> {
    let suffix = basis_suffix(basis);
    results
        .iter()
        .find(|r| {
            r.entity_ref == pipe
                && r.kind == kind
                && r.metadata.as_ref().is_some_and(|m| {
                    m.location == location
                        && component.is_none_or(|c| m.component == c)
                        && (if suffix.is_empty() { !m.basis.contains("; section_basis=") } else { m.basis.ends_with(suffix) })
                })
        })
        .map(|r| r.value)
}

/// §2.3: σ_N of one station and basis (MPa): the axial component row, or on a
/// pressurized pressure-2 member RN(membrane / 1e6).
fn sigma_n(results: &[ResultItem], pipe: &str, location: &str, basis: Option<SectionBasis>) -> Option<f64> {
    published(results, pipe, "element_local_axial_normal_stress", None, location, basis).or_else(|| {
        published(results, pipe, "pipe_axial_membrane_stress_v2", None, location, basis).map(q::pa_to_mpa)
    })
}

fn row(id: String, kind: &str, value: f64, unit: &str, pipe: &str, component: &str, location: &str, basis: &str, sign: &str) -> ResultItem {
    ResultItem { component_body: None,
        id,
        kind: kind.to_string(),
        value,
        unit: unit.to_string(),
        entity_ref: pipe.to_string(),
        basis_ref: None,
        source_result_refs: Vec::new(),
        metadata: Some(ResultMetadata {
            component: component.to_string(),
            coordinate_system: "pipe_section".to_string(),
            location: location.to_string(),
            basis: basis.to_string(),
            sign_convention: sign.to_string(),
        }),
    }
}

/// A row on its basis: nominal/reduced suffixes on pressure-2 (`hyphen` for
/// `result:stress:` ids), unchanged on preview.
fn on_basis(row: ResultItem, basis: Option<SectionBasis>, hyphen: bool) -> ResultItem {
    match basis {
        Some(basis) => section_basis_row(row, basis, hyphen),
        None => row,
    }
}

/// T4-U4b §4.1 (W1's failed-ordinary manifest): one straight preview member's 15 derived
/// rows, σ⁺, σ⁻ and τ_v,max at the five stations in `append_station_quantities`' order, with
/// their ordinary ids and metadata and placeholder values that the freeze forms from the
/// projected rows. `transverse_shear` is false for a member whose ν is out of domain: its
/// τ rows are absent and the manifest carries `TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN`, as the
/// ordinary rows do.
pub(crate) fn preview_straight_derived_placeholders(results: &mut Vec<ResultItem>, pipe_id: &str, transverse_shear: bool) {
    let suffix = stable_suffix(pipe_id);
    for location in STATIONS {
        let loc = id_location(location);
        for (component, sign, tail) in [("sigma_plus", SIGMA_PLUS_SIGN, "sigma-plus"), ("sigma_minus", SIGMA_MINUS_SIGN, "sigma-minus")] {
            results.push(row(format!("result:stress:{suffix}:{loc}:{tail}"), EXTREME_KIND, 0.0, "MPa", pipe_id, component, location, EXTREMES_BASIS, sign));
        }
        if transverse_shear {
            results.push(row(format!("result:stress:{suffix}:{loc}:transverse-shear-max"), SHEAR_MAX_KIND, 0.0, "MPa", pipe_id, "tau_v_max", location, SHEAR_STRAIGHT_BASIS, SHEAR_SIGN_PREVIEW));
        }
    }
}

/// §2.4: the named warning of a member whose ν is outside -1 < ν < 1/2 (no τ_v,max is
/// published for it in the case); shared by the ordinary rows and W1's failed-ordinary manifest.
pub(crate) fn poisson_out_of_domain(case_id: &str, pipe_id: &str, nu: f64) -> Diagnostic {
    diag(
        &format!("diagnostic:transverse-shear:{}:{}:poisson", stable_suffix(case_id), stable_suffix(pipe_id)),
        POISSON_OUT_OF_DOMAIN,
        "warning",
        format!("member {} has nu = {} outside -1 < nu < 1/2, where the Saint-Venant transverse-shear field is defined; no transverse-shear maximum is published for it in {}", pipe_id, scalar_string(nu), case_id),
        vec![case_id.to_string(), pipe_id.to_string()],
    )
}

/// Inputs of one member's station quantities.
pub(crate) struct MemberQuantities<'a> {
    pub case_id: &'a str,
    pub pipe_id: &'a str,
    pub arc: bool,
    pub exact: bool,
    pub bases: Vec<BasisSection>,
    /// ν by the route's rule (§2.4); out of domain is checked here.
    pub poisson: ShearPoisson,
    /// (i_ip, i_op) of a realized arc with the directional pair (§2.6).
    pub directional: Option<(f64, f64)>,
    /// (p, R) of a pressurized pressure-2 arc (§2.5).
    pub toroidal: Option<(f64, f64)>,
}

/// What the station quantities of one member produced, for the maxima and
/// evidence: per basis, the station max(|σ⁺|,|σ⁻|) in Pa rounded upward and
/// the directional station values in Pa, each with its fraction; and the
/// directional station records.
#[derive(Debug, Default)]
pub(crate) struct StationOutputs {
    pub normal_witnesses: Vec<(Option<SectionBasis>, Vec<(f64, f64)>)>,
    pub directional_witnesses: Vec<(Option<SectionBasis>, Vec<(f64, f64)>)>,
    pub directional_records: Vec<serde_json::Value>,
}

/// Pa rounded upward from a published MPa value (the station cover's
/// conversion; a reader applies the same operation).
pub(crate) fn pa_upward(mpa: f64) -> f64 {
    let pa = mpa * 1e6;
    if pa == 0.0 { 0.0 } else { pa.next_up() }
}

/// The final id of a case row: the first case's and every `_v2` row's id
/// unchanged; any other row qualified by its case (the envelope's rule).
pub(crate) fn final_row_id(model: &PreviewModel, case_id: &str, id: &str, kind: &str) -> String {
    if model.load_cases.first().is_some_and(|c| c.id == case_id) || kind.ends_with("_v2") {
        id.to_string()
    } else {
        qualified_load_case_result_id(case_id, id)
    }
}

/// §2.3–§2.6: append one member's σ⁺/σ⁻, τ_v,max, directional and toroidal
/// station rows from its published rows (this case's `results`).
pub(crate) fn append_station_quantities(
    model: &PreviewModel,
    results: &mut Vec<ResultItem>,
    diagnostics: &mut Vec<Diagnostic>,
    m: &MemberQuantities<'_>,
) -> StationOutputs {
    let mut out = StationOutputs::default();
    let suffix = stable_suffix(m.pipe_id);
    let factors: Vec<Result<f64, q::SectionQuantityError>> = m
        .bases
        .iter()
        .map(|b| m.poisson.shear_factor(b.outer_radius, b.inner_radius, b.inertia))
        .collect();
    if let Some(Err(q::SectionQuantityError::PoissonOutOfDomain { nu })) = factors.first() {
        diagnostics.push(poisson_out_of_domain(m.case_id, m.pipe_id, *nu));
    }
    let shear_sign = format!(
        "{}{}",
        if m.exact { SHEAR_SIGN_EXACT } else { SHEAR_SIGN_PREVIEW },
        if m.arc { SHEAR_ARC_SUFFIX } else { "" }
    );
    let shear_basis = if m.arc { SHEAR_ARC_BASIS } else { SHEAR_STRAIGHT_BASIS };
    // §2.5: the hoop and radial values per basis. Only the domain condition
    // R <= r_o is the named warning `ARC_TOROIDAL_HOOP_DOMAIN` (no rows); any
    // other failure (a non-finite pressure, a nonpositive bore or wall, an
    // overflowing value) blocks the run by its own name,
    // `ARC_TOROIDAL_HOOP_EVALUATION_FAILED`, so no row of the case is published.
    let hoop = m.toroidal.map(|(p, r)| {
        m.bases
            .iter()
            .map(|b| q::toroidal_membrane_hoop(p, b.inner_radius, b.wall, b.outer_radius, r).and_then(|h| q::toroidal_radial_surface(p, b.outer_radius, r).map(|radial| (h, radial))))
            .collect::<Vec<_>>()
    });
    if let (Some((_, r)), Some(values)) = (m.toroidal, &hoop) {
        if values.iter().any(|v| matches!(v, Err(q::SectionQuantityError::ToroidalDomain { .. }))) {
            diagnostics.push(diag(
                &format!("diagnostic:toroidal-hoop:{}:{}:domain", stable_suffix(m.case_id), suffix),
                TOROIDAL_DOMAIN,
                "warning",
                format!("{TOROIDAL_DOMAIN_TEXT}: arc {} has R = {} and r_o = {}; its toroidal hoop and radial rows are not published in {}", m.pipe_id, scalar_string(r), scalar_string(m.bases[0].outer_radius), m.case_id),
                vec![m.case_id.to_string(), m.pipe_id.to_string()],
            ));
        } else if let Some((b, Err(error))) = m.bases.iter().zip(values).find(|(_, v)| v.is_err()) {
            diagnostics.push(diag(
                &format!("diagnostic:toroidal-hoop:{}:{}:evaluation{}", stable_suffix(m.case_id), suffix, b.basis.map_or(String::new(), |b| format!(":{}", b.hyphen()))),
                TOROIDAL_EVALUATION_FAILED,
                "blocking",
                format!("{TOROIDAL_EVALUATION_TEXT}: arc {} in {}: {error:?}", m.pipe_id, m.case_id),
                vec![m.case_id.to_string(), m.pipe_id.to_string()],
            ));
        }
    }
    for b in &m.bases {
        out.normal_witnesses.push((b.basis, Vec::new()));
        if m.directional.is_some() {
            out.directional_witnesses.push((b.basis, Vec::new()));
        }
    }
    for location in STATIONS {
        let loc = id_location(location);
        // σ⁺/σ⁻ then τ, each nominal row followed by its reduced twin.
        let mut extremes = Vec::new();
        for (slot, b) in m.bases.iter().enumerate() {
            let operands = sigma_n(results, m.pipe_id, location, b.basis).zip(
                published(results, m.pipe_id, "element_local_bending_normal_stress_y", None, location, b.basis)
                    .zip(published(results, m.pipe_id, "element_local_bending_normal_stress_z", None, location, b.basis)),
            );
            let Some((n, (y, z))) = operands else { continue };
            let e = q::normal_stress_extremes(n, y, z);
            out.normal_witnesses[slot].1.push((station_fraction(location), pa_upward(e.max_abs())));
            extremes.push((b.basis, e, y, z));
        }
        // D-5 (section-bases-and-timoshenko.md §2.5): the component outer and
        // the basis inner, so each nominal row is followed by its reduced twin.
        for (component, sign, tail) in [
            ("sigma_plus", SIGMA_PLUS_SIGN, "sigma-plus"),
            ("sigma_minus", SIGMA_MINUS_SIGN, "sigma-minus"),
        ] {
            for &(basis, e, _, _) in &extremes {
                let value = if component == "sigma_plus" { e.sigma_plus } else { e.sigma_minus };
                results.push(on_basis(
                    row(format!("result:stress:{suffix}:{loc}:{tail}"), EXTREME_KIND, value, "MPa", m.pipe_id, component, location, EXTREMES_BASIS, sign),
                    basis,
                    true,
                ));
            }
        }
        // τ_v,max: the section shear rows inside; the end-force rows at the ends.
        let shears = published(results, m.pipe_id, "element_local_shear_force_y", None, location, None)
            .zip(published(results, m.pipe_id, "element_local_shear_force_z", None, location, None));
        if let Some((vy, vz)) = shears {
            for (b, factor) in m.bases.iter().zip(&factors) {
                let Ok(factor) = factor else { continue };
                results.push(on_basis(
                    row(format!("result:stress:{suffix}:{loc}:transverse-shear-max"), SHEAR_MAX_KIND, q::transverse_shear_maximum_mpa(vy, vz, *factor), "MPa", m.pipe_id, "tau_v_max", location, shear_basis, &shear_sign),
                    b.basis,
                    true,
                ));
            }
        }
        // §2.6: the directional intensified rows of an arc with the pair.
        if let Some((i_ip, i_op)) = m.directional {
            for &(basis, _, y, z) in &extremes {
                let value = q::directional_intensified_bending_pa(i_ip, i_op, y, z);
                let item = on_basis(
                    row(format!("result:intensified-directional:{suffix}:{loc}"), DIRECTIONAL_KIND, value, "Pa", m.pipe_id, "intensified_bending", location, DIRECTIONAL_BASIS, DIRECTIONAL_SIGN),
                    basis,
                    true,
                );
                let mut record = serde_json::json!({"pipe_id": m.pipe_id, "result_id": final_row_id(model, m.case_id, &item.id, &item.kind),
                    "location": location, "i_in_plane": i_ip, "i_out_of_plane": i_op});
                if let Some(basis) = basis {
                    record["section_basis"] = serde_json::json!(basis.token());
                }
                out.directional_records.push(record);
                if let Some(w) = out.directional_witnesses.iter_mut().find(|w| w.0 == basis) {
                    w.1.push((station_fraction(location), value));
                }
                results.push(item);
            }
        }
        // §2.5: the toroidal hoop and radial rows (R > r_o).
        if let Some(values) = &hoop {
            for (component, pick) in [("toroidal_intrados_hoop_stress", 0usize), ("toroidal_crown_hoop_stress", 1), ("toroidal_extrados_hoop_stress", 2)] {
                for (b, value) in m.bases.iter().zip(values) {
                    let Ok((h, _)) = value else { continue };
                    let v = [h.intrados, h.crown, h.extrados][pick];
                    results.push(on_basis(
                        row(pressure_row_id(m.case_id, m.pipe_id, location, component), HOOP_KIND, v, "Pa", m.pipe_id, component, location, HOOP_BASIS, HOOP_SIGN),
                        b.basis,
                        false,
                    ));
                }
            }
            for (component, inner) in [("toroidal_inner_radial_stress", true), ("toroidal_outer_radial_stress", false)] {
                for (b, value) in m.bases.iter().zip(values) {
                    let Ok((_, (ri, ro))) = value else { continue };
                    results.push(on_basis(
                        row(pressure_row_id(m.case_id, m.pipe_id, location, component), RADIAL_KIND, if inner { *ri } else { *ro }, "Pa", m.pipe_id, component, location, HOOP_BASIS, RADIAL_SIGN),
                        b.basis,
                        false,
                    ));
                }
            }
        }
    }
    out
}

fn pressure_row_id(case_id: &str, pipe_id: &str, location: &str, component: &str) -> String {
    format!("result:pressure-exact:{}:{}:{}:{}:{}:{}", case_id.len(), case_id, pipe_id.len(), pipe_id, location, component)
}

/// The arc operands of the certified maximum (§2.3): the published end_j rows
/// through the end-j frame, the arc's uniform intensities (scale 1: each is
/// the scaled global force per unit arc length the station rows use), R, φ,
/// pA_i on a pressurized arc, and the basis A and Z.
pub(crate) struct ArcOperands<'a> {
    pub bend: &'a CurvedBendMacroBuild,
    pub end_forces: &'a [f64],
    pub intensities: &'a [[f64; 3]],
    pub pressure_area_product: Option<f64>,
}

/// One arc's certified maximum on one basis (§2.3, §2.6).
pub(crate) fn arc_maximum(
    operands: &ArcOperands<'_>,
    basis: &BasisSection,
    measure: ArcStressMeasure,
    witnesses: &[(f64, f64)],
) -> Result<CertifiedArcStressMaximum, String> {
    if operands.end_forces.len() < 2 * DOF_PER_NODE {
        return Err("arc_maximum_end_forces_missing".into());
    }
    let axes = curved_bend_end_axes(operands.bend, true).map_err(|_| "arc_maximum_end_axes_unavailable".to_string())?;
    let geometry = operands.bend.macro_element.geometry().map_err(|_| "arc_maximum_geometry_unavailable".to_string())?;
    let mut end_j = [0.0; 6];
    end_j.copy_from_slice(&operands.end_forces[DOF_PER_NODE..2 * DOF_PER_NODE]);
    let intensities: Vec<ArcUniformIntensity> =
        operands.intensities.iter().map(|g| ArcUniformIntensity { global: *g, scale: 1.0 }).collect();
    let problem = ArcStressMaximumProblem {
        bend_radius: geometry.radius,
        included_angle: geometry.included_angle,
        end_j_axes: axes,
        end_j_force: end_j,
        uniform_intensities: &intensities,
        pressure_area_product: operands.pressure_area_product,
        area: basis.area,
        section_modulus: basis.modulus,
    };
    // §2.3 step 5: the station cover is the published station values (Pa,
    // rounded upward); their fractions stay with the caller's tests.
    let cover: Vec<f64> = witnesses.iter().map(|&(_, value_pa)| value_pa).collect();
    let outcome = certify_arc_stress_maximum(&problem, measure, &cover).map_err(|failure| failure.reason().to_string());
    #[cfg(test)]
    probe::record(operands, basis, measure, witnesses, &outcome);
    outcome
}

/// Test-only record of every arc maximum's operands and outcome on this
/// thread, so a test can bind each certified maximum to the station function
/// that formed its station rows (`u4b_tests`).
#[cfg(test)]
pub(crate) mod probe {
    use super::*;
    #[derive(Debug, Clone)]
    pub(crate) struct ArcProbe {
        pub bend: CurvedBendMacroBuild,
        pub end_forces: Vec<f64>,
        pub intensities: Vec<[f64; 3]>,
        pub pressure_area_product: Option<f64>,
        pub basis: BasisSection,
        pub measure: ArcStressMeasure,
        pub witnesses: Vec<(f64, f64)>,
        pub outcome: Result<CertifiedArcStressMaximum, String>,
    }
    thread_local! {
        pub(crate) static PROBE: std::cell::RefCell<Option<Vec<ArcProbe>>> = const { std::cell::RefCell::new(None) };
    }
    pub(super) fn record(operands: &ArcOperands<'_>, basis: &BasisSection, measure: ArcStressMeasure, witnesses: &[(f64, f64)], outcome: &Result<CertifiedArcStressMaximum, String>) {
        PROBE.with(|probe| {
            if let Some(records) = probe.borrow_mut().as_mut() {
                records.push(ArcProbe {
                    bend: operands.bend.clone(),
                    end_forces: operands.end_forces.to_vec(),
                    intensities: operands.intensities.to_vec(),
                    pressure_area_product: operands.pressure_area_product,
                    basis: *basis,
                    measure,
                    witnesses: witnesses.to_vec(),
                    outcome: outcome.clone(),
                });
            }
        });
    }
}

/// The published value of a certified arc maximum: the binary64 midpoint of
/// [value_lower, upper_bound], as straights publish.
pub(crate) fn arc_published_value(m: &CertifiedArcStressMaximum) -> f64 {
    m.value_lower + 0.5 * (m.upper_bound - m.value_lower)
}

/// The arc maximum's closed fourteen-key record (§2.3), without
/// `section_basis` (pressure-2 adds it).
pub(crate) fn arc_record(pipe_id: &str, result_id: &str, m: &CertifiedArcStressMaximum, approximation: &str) -> serde_json::Value {
    serde_json::json!({"pipe_id": pipe_id, "result_id": result_id,
        "station_fraction": m.fraction, "span_index": 0, "local_fraction": m.fraction,
        "value_lower_pa": m.value_lower, "value_upper_pa": m.upper_bound, "global_upper_bound_pa": m.upper_bound,
        "certified_gap_pa": m.certified_gap, "subdivisions": m.subdivisions,
        "approximation": approximation, "coefficient_basis": ARC_COEFFICIENT_BASIS,
        "enclosure_scope": ARC_ENCLOSURE_SCOPE})
}

/// The arc maximum row (§2.3): `pipe_elastic_normal_stress_maximum_v2` on
/// `governing_station`, the straight id form.
pub(crate) fn arc_maximum_row(case_id: &str, pipe_id: &str, value: f64, basis: Option<SectionBasis>) -> ResultItem {
    let item = row(
        format!("result:elastic-maximum:{}:{}:{}:{}", case_id.len(), case_id, pipe_id.len(), pipe_id),
        MAXIMUM_KIND, value, "Pa", pipe_id, "maximum_absolute_normal_stress", "governing_station", MAXIMUM_BASIS, ARC_MAXIMUM_SIGN,
    );
    on_basis(item, basis, false)
}

/// The directional maximum row (§2.6) on `governing_station`.
pub(crate) fn directional_maximum_row(case_id: &str, pipe_id: &str, value: f64, basis: Option<SectionBasis>) -> ResultItem {
    let item = row(
        format!("result:intensified-directional-maximum:{}:{}:{}:{}", case_id.len(), case_id, pipe_id.len(), pipe_id),
        DIRECTIONAL_MAX_KIND, value, "Pa", pipe_id, "intensified_bending_maximum", "governing_station", DIRECTIONAL_BASIS, DIRECTIONAL_MAX_SIGN,
    );
    on_basis(item, basis, true)
}

/// The directional pair of a realized arc's bend component (§2.6), validated
/// before the solve.
pub(crate) fn directional_pair(model: &PreviewModel, bend: &CurvedBendMacroBuild) -> Option<(f64, f64)> {
    let modifiers = model.components.iter().find(|c| c.id == bend.component_id)?.modifiers.as_ref()?;
    Some((modifiers.sif_in_plane_user_value.as_ref()?.value, modifiers.sif_out_of_plane_user_value.as_ref()?.value))
}

/// §2.4: ν on preview-physics-2, RN(RN(E/RN(2·G)) − 1) from the moduli the
/// solve uses for this member and case.
pub(crate) fn preview_poisson_ratio(elastic_modulus: f64, shear_modulus: f64) -> f64 {
    elastic_modulus / (2.0 * shear_modulus) - 1.0
}

/// A member's ν by its route (§2.4).
#[derive(Debug, Clone, Copy)]
pub(crate) enum ShearPoisson {
    /// pressure-2: the member's authored ν.
    Authored(f64),
    /// preview-physics-2: ν = E/(2G) − 1 from the moduli the solve uses.
    Moduli { elastic_modulus: f64, shear_modulus: f64 },
}

impl ShearPoisson {
    /// c_b of the annulus (r_o, r_i,b, I_b) by this route's ν rule.
    pub(crate) fn shear_factor(self, outer_radius: f64, inner_radius: f64, second_moment: f64) -> Result<f64, q::SectionQuantityError> {
        match self {
            Self::Authored(nu) => q::saint_venant_shear_factor(nu, outer_radius, inner_radius, second_moment),
            Self::Moduli { elastic_modulus, shear_modulus } => preview_shear_factor(elastic_modulus, shear_modulus, outer_radius, inner_radius, second_moment),
        }
    }
}

/// §2.4: c_b on preview-physics-2 from the moduli. Admission is ν's
/// ([`preview_poisson_ratio`], −1 < ν < 1/2, `TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN`).
/// With q = RN(E/RN(2·G)):
/// - q ≥ 1/2 (and a NaN q): `saint_venant_shear_factor` of that ν, whose
///   1+ν = q and 1+2ν = 2q − 1 are exact (Sterbenz), so its bits stand;
/// - q < 1/2 (ν < −1/2, G > E): `saint_venant_shear_factor_of_ratio` of
///   x = RN(RN(2·G)/E) (frame kernel `shear::moduli_ratio_terms`), where
///   1+ν = RN(q − 1) + 1 would lose up to 2^-54/q relative.
///
/// **Error** (u = 2^-53, E, G, r_o, r_i,b and I_b exact, first order): on
/// q ≥ 1/2, 3+2ν = RN(2q + 1) carries (3/4 + 1)u, its product with RN(r_o²)
/// 3.75u, RN(1+2ν)·RN(r_i,b²) an absolute 2q*·u·r_i,b² (≤ (3/4)(2q*+1)u·r_o²)
/// plus 2u of its own, so num 5.5u, den 2u and c_b 8.5u; on q < 1/2, 7u. So
/// |c_b/c_b* − 1| ≤ 8.5u for every E/(2G) in (0, 3/2), and τ_v,max, three
/// more correctly rounded operations, is within 11.5u of |V|·c_b*/1e6.
pub(crate) fn preview_shear_factor(elastic_modulus: f64, shear_modulus: f64, outer_radius: f64, inner_radius: f64, second_moment: f64) -> Result<f64, q::SectionQuantityError> {
    let ratio = elastic_modulus / (2.0 * shear_modulus);
    let nu = ratio - 1.0;
    if !(ratio < 0.5) {
        return q::saint_venant_shear_factor(nu, outer_radius, inner_radius, second_moment);
    }
    if !(nu > -1.0) {
        return Err(q::SectionQuantityError::PoissonOutOfDomain { nu });
    }
    let (x, _) = open_pipe_stress_frame_kernel::shear::moduli_ratio_terms(elastic_modulus, shear_modulus)
        .map_err(|_| q::SectionQuantityError::InvalidInput { name: "moduli" })?;
    q::saint_venant_shear_factor_of_ratio(x, outer_radius, inner_radius, second_moment)
}
