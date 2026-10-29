//! Opt-in Timoshenko straight members on the pressure-free route (DEL-05-03
//! Design `pf-timoshenko.md`). A pressure-free document whose
//! `model_settings.shear_model` is `timoshenko_annulus_energy_v1` forms every
//! straight frame member (geometry-only bends, tee legs and W-15 rigid-family
//! members included) with the exact route's element and shear-factor
//! definition, applied to PF's own section and moduli: κ_E on PF's stiffness
//! annulus (r_o = OD/2, r_i = OD/2 − (t − c_m)) with ν = E/(2G) − 1 from the
//! member's unscaled case-resolved moduli. The range check and every reported ν
//! use PF's stress-recovery order (`u4b_rows::preview_poisson_ratio`); κ_E itself
//! is formed from E and G without 1 + ν (`shear::annulus_energy_shear_factor_of_moduli`).
//! Realized arcs stay
//! Euler–Bernoulli and are disclosed by name (§4).
//!
//! Admission follows consumption (§2): the bases the cases consume are known
//! from their temperature selections before any build, so a factor failure or
//! a ν warning belongs to a consumed basis and names the cases that select it.
//! A seed build that no case consumes forms Euler–Bernoulli members and is never
//! solved. No solved member takes a fallback factor or an Euler–Bernoulli leaf.
//!
//! The result carries its own semantic contract identity (§4); every
//! Euler–Bernoulli result, absent or explicit, keeps preview-physics-2's.
use super::*;
use crate::model_settings::ShearModelSetting;
use open_pipe_stress_frame_kernel::shear;

/// §4: the Timoshenko PF result's semantic contract identity.
pub(crate) const ID: &str = "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1";
/// §4: the Timoshenko PF result's profile.
pub(crate) const PROFILE_ID: &str = "product_preview_mechanics_timoshenko_v1";
/// §4: limitation 0 of the Timoshenko PF identity; the other limitations are preview-physics-2's.
pub(crate) const LIMITATION_0: &str = "Small-displacement, linear-elastic frame preview: straight members, including geometry-only bends, are Timoshenko with the energy-matched annular shear factor (timoshenko_annulus_energy_v1) and Poisson's ratio E/(2G) - 1, and realized arcs are Euler-Bernoulli; a straight member's published nodal rotations are section rotations, not slopes; numerical integrity does not establish physical correctness.";

/// §3: a partial-span member load on a Timoshenko member, refused before any route.
pub(crate) const PARTIAL_SPAN_UNSUPPORTED: &str = "PF_TIMOSHENKO_PARTIAL_SPAN_UNSUPPORTED";
/// §2: the factor-range block of a consumed basis (ν or A_s outside the factor's range).
pub(crate) const SHEAR_FACTOR_UNAVAILABLE: &str = "PF_TIMOSHENKO_SHEAR_FACTOR_UNAVAILABLE";
/// §2: an authored `poisson_ratio` that differs from E/(2G) − 1 and is not used.
pub(crate) const AUTHORED_POISSON_RATIO_NOT_USED: &str = "PF_TIMOSHENKO_AUTHORED_POISSON_RATIO_NOT_USED";
/// §4: the named disclosure of the realized arcs that stay Euler–Bernoulli.
pub(crate) const REALIZED_ARCS_EULER_BERNOULLI: &str = "PF_TIMOSHENKO_REALIZED_ARCS_EULER_BERNOULLI";

/// §4: the arc ids the disclosure names before "and N more".
const ARC_IDS_NAMED: usize = 32;
/// §4: the disclosure's bound in Unicode scalar values (the report renderer's evidence threshold).
const MESSAGE_SCALARS: usize = 1000;
/// §2: the disclosure threshold of the authored-ν warning (not a numerical tolerance).
const POISSON_RATIO_DISCLOSURE: f64 = 1e-6;

/// Whether `model` is a pressure-free document selecting Timoshenko straight members.
pub(crate) fn selects(model: &PreviewModel) -> bool {
    model.pressure_contract.is_none() && model.shear_model() == ShearModelSetting::TimoshenkoAnnulusEnergyV1
}

/// The producer's semantic contract identity on the preview route.
pub(crate) fn semantic_contract_id(model: &PreviewModel) -> &'static str {
    if selects(model) { ID } else { preview_physics::ID }
}

/// The preview route's formulation basis: preview-physics-2's, or for a Timoshenko
/// document the same with its own profile and limitation 0.
pub(crate) fn formulation_basis(model: &PreviewModel) -> FormulationBasis {
    let mut basis = preview_physics::formulation_basis();
    if selects(model) {
        basis.profile_id = PROFILE_ID.to_string();
        basis.limitations[0] = LIMITATION_0.to_string();
    }
    basis
}

/// One consumed material basis: how the diagnostics name it, and the load cases that select it.
#[derive(Debug, Clone)]
pub(crate) struct ConsumedBasis {
    pub(crate) description: String,
    pub(crate) case_ids: Vec<String>,
}

/// §2, "Construction boundary": the bases the document's cases consume, from their
/// temperature selections alone. A case whose selection is itself refused
/// (`modulus_basis_key`'s diagnostics) consumes nothing; it blocks when the case loop reaches it.
#[derive(Debug, Clone, Default)]
pub(crate) struct ConsumedBases {
    pub(crate) base: Option<ConsumedBasis>,
    pub(crate) selected: Vec<(String, ConsumedBasis)>,
}

impl ConsumedBases {
    /// The consumed bases of a Timoshenko document; none for any other document.
    pub(crate) fn of(model: &PreviewModel) -> Self {
        let mut bases = Self::default();
        if !selects(model) {
            return bases;
        }
        for case in &model.load_cases {
            let mut refusals = Vec::new();
            let consumed = match modulus_basis_key(case, &mut refusals) {
                Some(key) => match bases.selected.iter().position(|(selected, _)| *selected == key) {
                    Some(index) => &mut bases.selected[index].1,
                    None => {
                        let description = match (&case.modulus_basis_ref, &case.modulus_basis_temperature) {
                            (Some(point), _) => format!("the material basis at temperature point {point}"),
                            (None, Some(temperature)) => format!("the material basis interpolated at solve temperature {:?} K", temperature.value),
                            (None, None) => unreachable!("a selected basis has a selection"),
                        };
                        bases.selected.push((key, ConsumedBasis { description, case_ids: Vec::new() }));
                        &mut bases.selected.last_mut().expect("pushed").1
                    }
                },
                None if refusals.is_empty() => bases.base.get_or_insert_with(|| ConsumedBasis {
                    description: "the base material basis (no temperature selection)".to_string(),
                    case_ids: Vec::new(),
                }),
                None => continue,
            };
            consumed.case_ids.push(case.id.clone());
        }
        bases
    }

    /// The consumed basis a selection key names, if any case consumes it.
    pub(crate) fn selected(&self, key: &str) -> Option<&ConsumedBasis> {
        self.selected.iter().find(|(selected, _)| selected == key).map(|(_, basis)| basis)
    }
}

/// §2: one straight member's shear area on a consumed basis, from PF's stiffness annulus
/// and the ν that gives the member's unscaled G. `None` after a blocking diagnostic: the
/// member is not formed, and the build blocks. The authored-ν warning is emitted once per
/// material and basis (`warned`), since every member of a material shares its moduli there.
#[allow(clippy::too_many_arguments)]
pub(crate) fn shear_area(
    basis: &ConsumedBasis,
    pipe: &PreviewPipe,
    derived: &DerivedSection,
    material: &MaterialInput,
    elastic_modulus: f64,
    shear_modulus: f64,
    warned: &mut HashSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<f64> {
    let nu = u4b_rows::preview_poisson_ratio(elastic_modulus, shear_modulus);
    let outer_radius = pipe.section.outside_diameter.value / 2.0;
    let inner_radius = outer_radius - derived.wall_thickness;
    let cases = basis.case_ids.join(", ");
    // §2: admission is the factor's range on this ν, unchanged; κ_E is then formed from
    // the moduli without 1 + ν, so it keeps its accuracy as E/(2G) approaches 0.
    let admitted = if !nu.is_finite() {
        Err(shear::ShearFactorError::NonFiniteInput { name: "poisson_ratio", value: nu })
    } else if !(nu > -1.0 && nu < 0.5) {
        Err(shear::ShearFactorError::PoissonRatioOutOfRange { value: nu })
    } else {
        Ok(())
    };
    let factor = admitted
        .and_then(|()| shear::annulus_energy_shear_factor_of_moduli(inner_radius, outer_radius, elastic_modulus, shear_modulus))
        .and_then(|kappa| shear::shear_area(kappa, derived.area));
    let shear_area = match factor {
        Ok(shear_area) => shear_area,
        Err(error) => {
            let mut refs = vec![pipe.id.clone(), material.id.clone()];
            refs.extend(basis.case_ids.iter().cloned());
            diagnostics.push(diag(
                &format!("diagnostic:pf-timoshenko:{}:shear-factor:{}", stable_suffix(&pipe.id), stable_suffix(&basis.case_ids[0])),
                SHEAR_FACTOR_UNAVAILABLE,
                "blocking",
                format!(
                    "timoshenko_annulus_energy_v1 member {} has no energy-matched annular shear factor on {} (load cases {cases}): nu = E/(2G) - 1 = {nu:?} from material {} E = {elastic_modulus:?} Pa and G = {shear_modulus:?} Pa, r_i = {inner_radius:?} m, r_o = {outer_radius:?} m: {error}; no default factor or Euler-Bernoulli member is used",
                    pipe.id, basis.description, material.id,
                ),
                refs,
            ));
            return None;
        }
    };
    if let Some(authored) = material.poisson_ratio.as_ref().map(|nu| nu.value) {
        let scale = authored.abs().max(nu.abs());
        if (authored - nu).abs() > POISSON_RATIO_DISCLOSURE * scale && warned.insert(material.id.clone()) {
            let mut refs = vec![material.id.clone()];
            refs.extend(basis.case_ids.iter().cloned());
            diagnostics.push(diag(
                &format!("diagnostic:pf-timoshenko:{}:authored-poisson-ratio:{}", stable_suffix(&material.id), stable_suffix(&basis.case_ids[0])),
                AUTHORED_POISSON_RATIO_NOT_USED,
                "warning",
                format!(
                    "material {} authors poisson_ratio {authored:?}, which is not used: its timoshenko_annulus_energy_v1 straight members on {} (load cases {cases}) take nu = E/(2G) - 1 = {nu:?} from E = {elastic_modulus:?} Pa and G = {shear_modulus:?} Pa",
                    material.id, basis.description,
                ),
                refs,
            ));
        }
    }
    Some(shear_area)
}

/// §3: on a Timoshenko document, every wind exposed span (a partial-span member load) on a
/// straight member is refused by name, before any route. A partial extent on a realized arc
/// keeps its existing refusal; an unknown pipe keeps the generator's.
pub(crate) fn append_partial_span_refusals(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    if !selects(model) {
        return;
    }
    let arcs = curved_bend_realized_pipe_ids(model);
    for case in &model.load_cases {
        let Some(wind) = case.equivalent_static.as_ref().and_then(|generation| generation.wind.as_ref()) else {
            continue;
        };
        for (index, span) in wind.exposed_spans.iter().enumerate() {
            let Some(pipe) = span.pipe_ref.as_deref() else { continue };
            if arcs.contains(pipe) || !model.pipe_segments.iter().any(|segment| segment.id == pipe) {
                continue;
            }
            diagnostics.push(diag(
                &format!("diagnostic:pf-timoshenko:{}:partial-span-{index}", stable_suffix(&case.id)),
                PARTIAL_SPAN_UNSUPPORTED,
                "blocking",
                format!(
                    "load case {} equivalent_static.wind.exposed_spans[{index}] is a partial-span member load on timoshenko_annulus_energy_v1 straight member {pipe}; partial-span loads on Timoshenko members are not yet supported, so mark the whole span (exposed_pipe_refs) or select euler_bernoulli",
                    case.id,
                ),
                vec![pipe.to_string(), case.id.clone()],
            ));
        }
    }
}

/// §4: the named arc disclosure, once per Timoshenko result that contains a realized arc.
pub(crate) fn append_arc_disclosure(model: &PreviewModel, built: &BuiltModel, diagnostics: &mut Vec<Diagnostic>) {
    if !selects(model) {
        return;
    }
    let mut arcs: Vec<&str> = built.curved_bend_elements.iter().map(|bend| bend.pipe_id.as_str()).collect();
    arcs.sort_unstable();
    arcs.dedup();
    if arcs.is_empty() {
        return;
    }
    diagnostics.push(diag(
        "diagnostic:pf-timoshenko:realized-arcs",
        REALIZED_ARCS_EULER_BERNOULLI,
        "info",
        arc_disclosure_message(built.frame_elements.len(), &arcs),
        arcs.iter().map(|arc| arc.to_string()).collect(),
    ));
}

/// The disclosure's text: the counts, then the arc ids (sorted by code point) up to
/// [`ARC_IDS_NAMED`] and within [`MESSAGE_SCALARS`], then "and N more" for the rest.
pub(crate) fn arc_disclosure_message(straight: usize, arcs: &[&str]) -> String {
    let plural = |count: usize, one: &str, many: &str| if count == 1 { format!("{count} {one}") } else { format!("{count} {many}") };
    let head = format!(
        "timoshenko_annulus_energy_v1: {} with shear deformation (Timoshenko); {} without shear deformation (Euler-Bernoulli): ",
        plural(straight, "straight member", "straight members"),
        plural(arcs.len(), "realized arc", "realized arcs"),
    );
    let text = |named: usize| {
        let rest = arcs.len() - named;
        let mut message = head.clone() + &arcs[..named].join(", ");
        if rest > 0 {
            message += &format!("{}and {rest} more", if named > 0 { " " } else { "" });
        }
        message + "."
    };
    let mut named = 0;
    while named < arcs.len().min(ARC_IDS_NAMED) && text(named + 1).chars().count() <= MESSAGE_SCALARS {
        named += 1;
    }
    text(named)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_disclosure_names_at_most_32_arcs_and_counts_the_rest() {
        assert_eq!(
            arc_disclosure_message(2, &["pipe:BC"]),
            "timoshenko_annulus_energy_v1: 2 straight members with shear deformation (Timoshenko); 1 realized arc without shear deformation (Euler-Bernoulli): pipe:BC."
        );
        let ids: Vec<String> = (0..40).map(|i| format!("pipe:{i:02}")).collect();
        let arcs: Vec<&str> = ids.iter().map(String::as_str).collect();
        let message = arc_disclosure_message(1, &arcs);
        assert!(message.ends_with("pipe:31 and 8 more."), "{message}");
        assert!(!message.contains("pipe:32"), "{message}");
        // Long ids: the 1,000-scalar bound governs before 32 ids, and the rest are counted.
        let long: Vec<String> = (0..40).map(|i| format!("pipe:{}{i:02}", "é".repeat(100))).collect();
        let arcs: Vec<&str> = long.iter().map(String::as_str).collect();
        let message = arc_disclosure_message(3, &arcs);
        assert!(message.chars().count() <= 1000, "{}", message.chars().count());
        let named = arcs.iter().filter(|arc| message.contains(*arc)).count();
        assert!(named > 0 && named < 32, "{named}");
        assert!(message.ends_with(&format!("and {} more.", 40 - named)), "{message}");
        // One id longer than the bound: no id is named and every arc is counted.
        let huge = "x".repeat(2000);
        let message = arc_disclosure_message(0, &[huge.as_str()]);
        assert!(message.ends_with("(Euler-Bernoulli): and 1 more."), "{message}");
    }
}
