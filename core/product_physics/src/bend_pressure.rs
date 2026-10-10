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
//!   ε_p)) − p_uniform (`recover_curved_bend_local_forces`, the chord-frame
//!   `element_local_*` rows). At each end and station the wall force is
//!   N_w = N_el + pAi and the effective force S = N_el; M and V are elastic
//!   ([`append_arc_pressure_results`]). The wall end action is published
//!   along the arc's end tangent. Lamé rows and the straight-statics maximum
//!   are withheld on arcs (plan section 4.3 item 1; U0's policy).
//!
//! Straight members are untouched: their ledger and recovery stay the
//! pre-cancelled straight ones (SP-1).

use super::pressure_exact::ARC_PRESSURE_STRAIN_ROUNDINGS;
use super::pressure_runtime::{ExactPressureCase, ExactPressurePipeState};
use super::{
    diag, Diagnostic, PreviewLoadCase, ResultBasisRef, ResultItem, ResultMetadata,
    StationResultants, UX,
};
use open_pipe_stress_frame_kernel::load_ledger::{gamma, product_upward};

/// The v3 limitation a model with a realized bend adds (static text, T3 O-10).
pub(super) const BEND_LIMITATION: &str = "Realized curved bends (curved_bend_macro_element with the user's bend radius and flexibility factor k, one k for both bending planes) are admitted under pressure: each owns the term K_b*u_free(eps_p) - c_b, with eps_p = (1-2nu)pAi/(E As) (the straight Lame mean Poisson term on the arc) and c_b its own end caps along the end tangents; on an arc N_w = N_el + pAi and S = N_el, and shear and moments are elastic. A region changes direction only at realized bends: a bend-end direction change up to alpha_tan = 1e-3 rad is carried exactly by the remainder pAi(t_in - t_out); a larger change (a mitre) and a straight-straight kink are refused. Geometry-only (chord) bends carry no pressure and are refused. On arcs the straight Lame hoop and radial values and the governing normal-stress maximum are withheld; the excluded effects above apply to every bend result.";

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

/// The pressure family's rows on a realized arc (H-2): `pipe_wall_*`,
/// `pipe_effective_*` and `pipe_axial_membrane_*` at the two ends and three
/// stations from the elastic section actions (tangent frame), and the wall
/// end actions along the end tangents. The elastic axial rows the arc
/// published are replaced, as on a pressurized straight; no Lamé row is
/// published on an arc.
#[allow(clippy::too_many_arguments)]
pub(super) fn append_arc_pressure_results(
    results: &mut Vec<ResultItem>,
    diagnostics: &mut Vec<Diagnostic>,
    case: &PreviewLoadCase,
    pipe_id: &str,
    state: &ExactPressurePipeState,
    endpoint_resultants: &[[f64; 6]; 2],
    stations: &[StationResultants],
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
    let mut recovered = Vec::with_capacity(sections.len());
    for (location, elastic) in sections {
        match state.annulus.recover_arc_wall_effective_membrane(elastic, state.pressure) {
            Ok(values) => recovered.push((location, values)),
            Err(_) => {
                diagnostics.push(diag(
                    "diagnostic:exact-pressure:arc-section",
                    RECOVERY_FAILED,
                    "blocking",
                    RECOVERY_UNREPRESENTABLE,
                    vec![case.id.clone(), pipe_id.to_string()],
                ));
                return;
            }
        }
    }
    let mut append = |kind: &str, component: &str, location: &str, value: f64, stress: bool, sign: &str| {
        results.push(ResultItem {
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
            unit: if stress { "Pa" } else { "N" }.to_string(),
            entity_ref: pipe_id.to_string(),
            basis_ref: Some(ResultBasisRef {
                ref_type: "load_case".to_string(),
                ref_id: case.id.clone(),
            }),
            source_result_refs: Vec::new(),
            metadata: Some(ResultMetadata {
                component: component.to_string(),
                coordinate_system: if stress { "pipe_section" } else { "element_local" }.to_string(),
                location: location.to_string(),
                basis: if stress {
                    "recovered_from_open_mechanics_stress_components"
                } else {
                    "recovered_from_local_element_stiffness"
                }
                .to_string(),
                sign_convention: sign.to_string(),
            }),
        });
    };
    // The wall end actions: end_i = -N_w(0), end_j = +N_w(1), each along its
    // end tangent (RV1 S-7), never the chord.
    for (location, sign) in [("end_i", -1.0), ("end_j", 1.0)] {
        let (_, (wall, _, _)) = recovered
            .iter()
            .find(|(name, _)| *name == location)
            .expect("both ends are recovered");
        append("pipe_wall_endpoint_action_v2", "wall_axial_end_action", location, sign * wall, false, WALL_END_SIGN);
    }
    for (location, (wall, effective, membrane)) in &recovered {
        append("pipe_wall_axial_force_v2", "wall_axial_force", location, *wall, false, WALL_FORCE_SIGN);
        append("pipe_effective_axial_force_v2", "effective_axial_force", location, *effective, false, EFFECTIVE_FORCE_SIGN);
        append("pipe_axial_membrane_stress_v2", "axial_membrane_stress", location, *membrane, true, MEMBRANE_SIGN);
    }
}
