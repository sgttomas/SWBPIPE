//! T4-U2: pressure through realized bends under `3.0.0/exact_pressure_v3`
//! (H-2), the load side and the arc recovery.
//!
//! - **Load (H-2).** A realized arc in a pressure region owns the term
//!   K_b·u_free(ε_p) − c_b. `pressure_runtime` carries −c_b, the terminal caps
//!   and the bend-adjacent remainders +pAi(t_in − t_out) in its source groups;
//!   the case ledger's producer `add_curved_bend_pressure_equivalent_load`
//!   (in `lib.rs`, with the other producers) pushes K_b·u_free(ε_p) exactly as
//!   the arc's thermal free-expansion identity pushes K_b·u_free(ε_th): one
//!   exact product K_rc·fl(ε_p·d_c) per nonzero column, self-equilibrated.
//!   S11-G reads the product's own rounding (`RoundedProduct`) and, as the
//!   operand bound, ε_p's formation error ([`strain_operand_bound`]).
//! - **Recovery.** The arc's elastic end actions are K_b(d − u_free(ε_th +
//!   ε_p)) − p_uniform (`recover_curved_bend_local_forces`, the tangent-frame
//!   `element_local_*` rows). At each end and station the wall force is
//!   N_w = N_el + pAi and the effective force S = N_el; M and V are elastic
//!   ([`append_arc_pressure_results`]). The wall end action is published
//!   along the arc's end tangent. No straight Lamé row is published on an
//!   arc; T4-U4b publishes the toroidal hoop and radial rows and the arc
//!   maximum (`u4b_rows`).
//!
//! Straight members are untouched: their ledger and recovery stay the
//! pre-cancelled straight ones (SP-1).

use super::pressure_exact::{SectionBasis, ARC_PRESSURE_STRAIN_ROUNDINGS};
use super::pressure_runtime::{ExactPressureCase, ExactPressurePipeState};
use super::{
    diag, section_basis_row, Diagnostic, PreviewLoadCase, ResultBasisRef, ResultItem,
    ResultMetadata, StationResultants, UX,
};
use open_pipe_stress_frame_kernel::load_ledger::{gamma, product_upward};
use open_pipe_stress_frame_kernel::structural::CertifiedMemberRecovery;

/// The v3 limitation a model with a realized bend adds (static text, T3 O-10).
pub(super) const BEND_LIMITATION: &str = "Realized curved bends (curved_bend_macro_element with the user's bend radius and flexibility factor k, one k for both bending planes) are admitted under pressure: each owns the term K_b*u_free(eps_p) - c_b, with eps_p = (1-2nu)pAi/(E As) (the straight Lame mean Poisson term on the arc) and c_b its own end caps along the end tangents; on an arc N_w = N_el + pAi and S = N_el, and shear and moments are elastic. A region changes direction only at realized bends: a bend-end direction change up to alpha_tan = 1e-3 rad is carried exactly by the remainder pAi(t_in - t_out); a larger change (a mitre) and a straight-straight kink are refused. Geometry-only (chord) bends carry no pressure and are refused. On arcs the hoop is the toroidal membrane cut average (bore_load_transferred_toroidal_membrane_v1) and the radial values are -p at the bore and 0 outside, the transverse shear is the straight-annulus Saint-Venant field driven by the tangent-frame shears (local_straight_annulus_on_arc), and the normal-stress maximum is bounded along the arc; ovalization, hoop bending from moments, bending boundary layers, tangent-end effects and the curvature correction of the shear field are not represented; the excluded effects above apply to every bend result.";

/// S11-G: the operand bound of one bend-pressure term K_rc·fl(ε̃_p·d_c).
///
/// ε̃_p carries k = [`ARC_PRESSURE_STRAIN_ROUNDINGS`] relative roundings from
/// its source operands, so |ε_p − ε̃_p| ≤ γ_k|ε_p| ≤ γ_2k|ε̃_p| (k ≤ 1/(2u)).
/// With |ε̃_p·d_c| ≤ (1 + u)|fl(ε̃_p·d_c)| the held operand's error is at most
/// γ_(2k+2)·|K_rc|·|fl(ε̃_p·d_c)|, each product rounded upward. The product's
/// own rounding is `RoundedProduct`'s exact defect, not counted here.
pub(super) fn strain_operand_bound(k: f64, held: f64) -> f64 {
    product_upward(
        gamma(2 * ARC_PRESSURE_STRAIN_ROUNDINGS + 2),
        product_upward(k.abs(), held.abs()),
    )
}

/// The arc's pressure strain to subtract in recovery with the thermal free
/// expansion (`None` for an arc outside every pressure region).
pub(super) fn recovery_strain(
    exact: Option<&ExactPressureCase>,
    pipe_index: usize,
) -> Option<f64> {
    exact
        .and_then(|case| case.pipe_states.get(&pipe_index))
        .and_then(|state| state.arc)
        .map(|arc| arc.strain)
}

const WALL_END_SIGN: &str = "node-on-element wall action along the arc end tangent (local x toward end j at that end); N_w = N_el + pAi; cap transfer is not subtracted from wall recovery";
const WALL_FORCE_SIGN: &str = "tension-positive material wall section resultant Nw = N_el + pAi along the arc tangent; shear and moments are the elastic section actions";
const EFFECTIVE_FORCE_SIGN: &str = "effective wall-fluid resultant S=Nw-pAi (the arc's elastic axial force); not material stress or a support reaction";
const MEMBRANE_SIGN: &str = "tension-positive axial wall membrane stress Nw/As on the arc section; no added longitudinal pressure scalar";
const RECOVERY_FAILED: &str = "EXACT_PRESSURE_RECOVERY_FAILED";
const RECOVERY_UNREPRESENTABLE: &str = "arc pressure section recovery is unrepresentable";
/// A section location outside the certified recovery's five sections.
pub(super) const ARC_SECTION_UNKNOWN: &str = "EXACT_PRESSURE_ARC_SECTION_UNKNOWN";

/// The certified recovery's section index of an arc section location
/// (`CERTIFIED_SECTION_FRACTIONS`: end_i, quarter_1, midspan, quarter_3,
/// end_j); `None` for any other name, which the caller refuses.
pub(super) fn certified_section_index(location: &str) -> Option<usize> {
    match location {
        "end_i" => Some(0),
        "quarter_1" => Some(1),
        "midspan" => Some(2),
        "quarter_3" => Some(3),
        "end_j" => Some(4),
        _ => None,
    }
}

/// The pressure family's rows on a realized arc (H-2): `pipe_wall_*`,
/// `pipe_effective_*` and `pipe_axial_membrane_*` at the two ends and three
/// stations from the elastic section actions (tangent frame), and the wall
/// end actions along the end tangents. The elastic axial rows the arc
/// published are replaced, as on a pressurized straight; no Lamé row is
/// published on an arc. On a pre-guard Passed case (`certified` = the arc's
/// certified wall and effective recoveries, K-D5) each N_w, S and N_w/As is
/// the certified row rounded once; otherwise the binary64 `Scaled` path.
#[allow(clippy::too_many_arguments)]
pub(super) fn append_arc_pressure_results(
    results: &mut Vec<ResultItem>,
    diagnostics: &mut Vec<Diagnostic>,
    case: &PreviewLoadCase,
    pipe_id: &str,
    state: &ExactPressurePipeState,
    endpoint_resultants: &[[f64; 6]; 2],
    stations: &[StationResultants],
    certified: Option<(&CertifiedMemberRecovery, &CertifiedMemberRecovery)>,
) {
    results.retain(|r| {
        r.entity_ref != pipe_id
            || !matches!(
                r.kind.as_str(),
                "element_local_axial_force"
                    | "element_local_axial_normal_stress"
                    | "pipe_section_pressure_hoop_stress"
                    | "pipe_section_pressure_longitudinal_stress"
            )
    });
    let mut sections = vec![
        ("end_i", endpoint_resultants[0][UX]),
        ("end_j", endpoint_resultants[1][UX]),
    ];
    sections.extend(stations.iter().map(|station| (station.location, station.resultants[UX])));
    let wanted = sections.len();
    let mut recovered = Vec::with_capacity(wanted);
    for (location, elastic) in sections {
        // D-5: N_w = N_el + pAi once on the pressure bore; σ_m = N_w/A_b for
        // the nominal and the reduced section.
        let values = match certified {
            Some((wall, effective)) => {
                // Fail closed: an unknown location is never read as end_j.
                let Some(index) = certified_section_index(location) else {
                    diagnostics.push(diag(
                        "diagnostic:exact-pressure:arc-section-location",
                        ARC_SECTION_UNKNOWN,
                        "blocking",
                        format!("arc section location {location:?} is not one of the certified recovery's five sections"),
                        vec![case.id.clone(), pipe_id.to_string()],
                    ));
                    return;
                };
                let wall_row = wall.sections[index][0];
                let membrane = |basis: SectionBasis| wall_row.divided(state.sections.basis(basis).wall_area_m2()).ok().map(|row| row.published);
                membrane(SectionBasis::Nominal).zip(membrane(SectionBasis::Reduced)).map(|(nominal, reduced)| {
                    (wall_row.published, effective.sections[index][0].published, [nominal, reduced])
                })
            }
            None => state.sections.recover_arc_wall_effective(elastic, state.pressure).ok().and_then(|forces| {
                let nominal = state.sections.membrane(SectionBasis::Nominal, forces).ok()?;
                let reduced = state.sections.membrane(SectionBasis::Reduced, forces).ok()?;
                Some((forces.wall_n, forces.effective_n, [nominal, reduced]))
            }),
        };
        let Some(values) = values else {
            break;
        };
        recovered.push((location, values));
    }
    // The first unrepresentable section ends the arc's recovery; its one
    // diagnostic is formed once, outside the sections loop.
    if recovered.len() != wanted {
        diagnostics.push(diag(
            "diagnostic:exact-pressure:arc-section",
            RECOVERY_FAILED,
            "blocking",
            RECOVERY_UNREPRESENTABLE,
            vec![case.id.clone(), pipe_id.to_string()],
        ));
        return;
    }
    // `stress` names a stress row's section basis (D-5): each nominal row is
    // immediately followed by its reduced twin; force rows are published once.
    let mut append = |kind: &str, component: &str, location: &str, value: f64, stress: Option<SectionBasis>, sign: &str| {
        let row = ResultItem { component_body: None,
            id: format!(
                "result:pressure-exact:{}:{}:{}:{}:{}:{}",
                case.id.len(),
                case.id,
                pipe_id.len(),
                pipe_id,
                location,
                component
            ),
            kind: kind.to_string(),
            value,
            unit: if stress.is_some() { "Pa" } else { "N" }.to_string(),
            entity_ref: pipe_id.to_string(),
            basis_ref: Some(ResultBasisRef {
                ref_type: "load_case".to_string(),
                ref_id: case.id.clone(),
            }),
            source_result_refs: Vec::new(),
            metadata: Some(ResultMetadata {
                component: component.to_string(),
                coordinate_system: if stress.is_some() { "pipe_section" } else { "element_local" }.to_string(),
                location: location.to_string(),
                basis: if stress.is_some() {
                    "recovered_from_open_mechanics_stress_components"
                } else {
                    "recovered_from_local_element_stiffness"
                }
                .to_string(),
                sign_convention: sign.to_string(),
            }),
        };
        results.push(match stress {
            Some(basis) => section_basis_row(row, basis, false),
            None => row,
        });
    };
    // The wall end actions: end_i = -N_w(0), end_j = +N_w(1), each along its
    // end tangent (RV1 S-7), never the chord.
    for (location, sign) in [("end_i", -1.0), ("end_j", 1.0)] {
        let (_, (wall, _, _)) = recovered
            .iter()
            .find(|(name, _)| *name == location)
            .expect("both ends are recovered");
        append("pipe_wall_endpoint_action_v2", "wall_axial_end_action", location, sign * wall, None, WALL_END_SIGN);
    }
    for (location, (wall, effective, membranes)) in &recovered {
        append("pipe_wall_axial_force_v2", "wall_axial_force", location, *wall, None, WALL_FORCE_SIGN);
        append("pipe_effective_axial_force_v2", "effective_axial_force", location, *effective, None, EFFECTIVE_FORCE_SIGN);
        for (basis, membrane) in SectionBasis::BOTH.into_iter().zip(*membranes) {
            append("pipe_axial_membrane_stress_v2", "axial_membrane_stress", location, membrane, Some(basis), MEMBRANE_SIGN);
        }
    }
}
