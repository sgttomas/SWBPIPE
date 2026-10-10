//! T4-U2a: the exact route's contract identity and family-dispatching
//! admission seam.
//!
//! Two exact contracts exist on model documents 0.3.0 and 0.4.0:
//! `2.0.0/exact_straight_pressure_v2` (straight-only, unchanged) and its
//! successor `3.0.0/exact_pressure_v3` (H-1), published under the reserved
//! `pressure-1` result semantics. Straight circular pipe members and linear
//! restraints or springs are the base of both contracts. Every other object
//! family the exact route can meet is classified here and dispatched through
//! one admission table, [`admission`]:
//!
//! - under v2 the table returns v2's existing codes and texts unchanged
//!   (SP-1); the call sites keep v2's emission order;
//! - under v3 every family is refused with the blocking
//!   `EXACT_PRESSURE_FAMILY_NOT_ADMITTED`, naming the family, until a later
//!   unit admits it.
//!
//! **Extending the seam.** A unit that admits a family adds one arm,
//! `(ExactContract::PressureV3, ExactFamily::X) => Admission::Admitted`, to
//! [`admission`] and supplies that family's mechanics, evidence and readers in
//! the same change: T4-U2 admits [`ExactFamily::RealizedBend`]; T4-U3 admits
//! [`ExactFamily::ObjectiveConnector`]. Admitting a family here without its
//! mechanics is unsafe (T4-I1 section 5.2: lifting the refusal alone panics).

use super::{FormulationBasis, PreviewComponent, PreviewModel, PreviewSupport};

/// The reserved successor result semantics that `3.0.0/exact_pressure_v3`
/// publishes under, on 0.3.0 and 0.4.0 documents alike (H-1).
pub(crate) const PRESSURE_SEMANTIC_CONTRACT_ID: &str =
    "openpipestress.result_semantics/0.3.0/pressure-1";
/// The formulation profile of every `pressure-1` envelope.
pub(crate) const PRESSURE_PROFILE_ID: &str = "exact_pressure_v3";

/// The named seam refusal for a family a v3 document may not yet contain.
pub(crate) const FAMILY_NOT_ADMITTED: &str = "EXACT_PRESSURE_FAMILY_NOT_ADMITTED";

/// An exact pressure contract declared on a 0.3.0 or 0.4.0 model document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExactContract {
    /// `2.0.0/exact_straight_pressure_v2`: straight members only; byte-identical.
    StraightV2,
    /// `3.0.0/exact_pressure_v3`: the successor, admitted family by family.
    PressureV3,
}

impl ExactContract {
    pub(crate) const fn version(self) -> &'static str {
        match self {
            ExactContract::StraightV2 => "2.0.0",
            ExactContract::PressureV3 => "3.0.0",
        }
    }

    pub(crate) const fn mode(self) -> &'static str {
        match self {
            ExactContract::StraightV2 => "exact_straight_pressure_v2",
            ExactContract::PressureV3 => "exact_pressure_v3",
        }
    }

    /// The contract named by an authored (version, mode) pair, on any document.
    pub(crate) fn from_declared(version: Option<&str>, mode: Option<&str>) -> Option<Self> {
        [ExactContract::StraightV2, ExactContract::PressureV3]
            .into_iter()
            .find(|contract| version == Some(contract.version()) && mode == Some(contract.mode()))
    }

    /// The exact contract of a model: only 0.3.0 and 0.4.0 documents carry one.
    pub(crate) fn of(model: &PreviewModel) -> Option<Self> {
        if !matches!(model.schema_version.as_str(), "0.3.0" | "0.4.0") {
            return None;
        }
        model.pressure_contract.as_ref().and_then(|contract| {
            Self::from_declared(contract.version.as_deref(), contract.mode.as_deref())
        })
    }

    /// Whether p < 0 is admitted (plan section 4.3 item 2): v3 only. v2's
    /// readers refuse p_pa < 0, so v2 refuses it at the producer (T4-U0).
    pub(crate) const fn admits_negative_pressure(self) -> bool {
        matches!(self, ExactContract::PressureV3)
    }
}

/// v3's own formulation basis (its approximation text). It names the admitted
/// families, states the D-3 exclusions and that external-pressure stability
/// and collapse are not assessed. A 0.4.0 document adds the load/reference
/// state method's statements.
pub(crate) fn pressure_v3_formulation_basis(load_state: bool) -> FormulationBasis {
    let mut limitations = vec![
        "3.0.0/exact_pressure_v3 under pressure-1 semantics. Admitted families: small-displacement homogeneous-isotropic straight circular pipe members (Euler-Bernoulli; source OD/effective wall define the single section basis; G is derived from E/nu) and linear restraints or springs. Every other component, support, combination or equivalent-static family is refused by name (EXACT_PRESSURE_FAMILY_NOT_ADMITTED) until a later contract revision admits it.".to_string(),
        "Pressure is a signed internal differential with zero external pressure increment, uniform within each explicit case-scoped collinear equal-bore region with explicit closure-transfer paths. A negative differential is admitted, but external-pressure stability and collapse are not assessed.".to_string(),
        "Not modelled: steady-flow momentum and transient pressure loads; bend opening under pressure (Bourdon effect); pressure stiffening of flexibility factors and stress intensification; ovalization.".to_string(),
        "Mechanical, pressure-eigen and cap-transfer contributions remain distinct; rounded cap/eigen ledgers are observational and do not replace source-grouped pressure assembly.".to_string(),
        "Structural line loads act as entered; contents density does not imply hydrostatic pressure head or a coupled static-fluid pressure/weight state.".to_string(),
        "Signed support actions are attributed only for admitted linear restraints and springs; ambiguous coincident rigid attribution is refused.".to_string(),
        "The circular normal-stress maximum bounds the supplied binary64 section-statical coefficients; solver/coefficient formation error is separate, and incomplete coverage withholds the headline.".to_string(),
        "No corrosion-allowance input or shear-deformation selection is provided. Retained-source recovery is not joined for this contract; only ordinary structural recovery is published. No code compliance or professional acceptance is produced, and actual numerical admission remains separate.".to_string(),
    ];
    if load_state {
        limitations.extend([
            "One resolved case per load case supplies each member's selected E/nu with derived G, actual/selected/installation temperatures and explicit expansion definition; each case is solved independently.".to_string(),
            "Thermal and fit reference strain compose as lambda_fit*lambda_thermal-1 and enter once as an axial eigenstrain; uniform member temperature only, no gradients, finite strain, inelasticity or fit-up joints.".to_string(),
            "Global rigid support translations/rotations are prescribed absolute boundary values through the partitioned solve; reactions come from the unreduced equations. Spring base motion, device preload/reference, inactive or locked supports are not provided.".to_string(),
            "Ordinary applied loads are exactly the case's declared source ledger with explicit factors; unreferenced stored primitives are excluded. Hydrostatic head, contents-weight state and per-case mass selection are not provided.".to_string(),
            "History is independent equilibrium only; no installation, contact, friction or predecessor history is represented.".to_string(),
        ]);
    }
    FormulationBasis { profile_id: PRESSURE_PROFILE_ID.to_string(), limitations }
}

/// Whether a model's cases may join retained-source recovery (and its
/// receipts). v3 is not joined: its `pressure-1` identity has no retained-source
/// counterpart, so a v3 invocation publishes ordinary structural recovery only.
pub(crate) fn joins_retained_source(model: &PreviewModel) -> bool {
    ExactContract::of(model) != Some(ExactContract::PressureV3)
}

/// Every object family the exact route can meet besides its straight base.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExactFamily {
    /// A bend or elbow realized as a curved macro element (`curved_bend_macro_element`).
    RealizedBend,
    /// A bend or elbow analysed as its straight chord (D-2: never pressure-carrying).
    GeometryOnlyBend,
    Valve,
    Flange,
    Reducer,
    Branch,
    /// `rigid` and `specialty` components.
    RigidComponent,
    /// Any component carrying an explicit `objective_connector`.
    ObjectiveConnector,
    /// An expansion joint in the explicit annotation-only mode (`not_solver_consumed`).
    ExpansionJointAnnotation,
    /// Every other expansion-joint shape (legacy flexibility or app-authored joints).
    ExpansionJointLegacy,
    /// `other`, `TBD` or an unrecognized component kind.
    OtherComponent,
    NonlinearSupport,
    ConstantEffortSupport,
    Combination,
    EquivalentStatic,
}

impl ExactFamily {
    /// The family as named in diagnostics.
    pub(crate) const fn name(self) -> &'static str {
        match self {
            ExactFamily::RealizedBend => "realized curved bend (curved_bend_macro_element)",
            ExactFamily::GeometryOnlyBend => "geometry-only bend (a straight chord with no flexibility)",
            ExactFamily::Valve => "valve",
            ExactFamily::Flange => "flange",
            ExactFamily::Reducer => "reducer",
            ExactFamily::Branch => "branch connection (tee)",
            ExactFamily::RigidComponent => "rigid or specialty component",
            ExactFamily::ObjectiveConnector => "objective connector",
            ExactFamily::ExpansionJointAnnotation => {
                "expansion joint (annotation only, not_solver_consumed)"
            }
            ExactFamily::ExpansionJointLegacy => "legacy expansion joint",
            ExactFamily::OtherComponent => "component",
            ExactFamily::NonlinearSupport => "nonlinear support",
            ExactFamily::ConstantEffortSupport => "constant-effort support",
            ExactFamily::Combination => "load combination",
            ExactFamily::EquivalentStatic => "equivalent-static generation",
        }
    }

    /// A family-specific clause appended to the v3 refusal, where one helps.
    const fn note(self) -> &'static str {
        match self {
            ExactFamily::GeometryOnlyBend => {
                "; geometry-only bends never carry pressure on the exact route (D-2) and remain on the pressure-free route"
            }
            ExactFamily::ExpansionJointAnnotation => {
                "; annotation-only joints are analysed as pipe on the pressure-free route only"
            }
            _ => "",
        }
    }
}

/// The table's verdict for one family under one contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Admission {
    /// The extension point: T4-U2 and T4-U3 add the first admitted entries.
    #[allow(dead_code)]
    Admitted,
    Refused { code: &'static str, message: String },
}

/// The admission table. Under v2 it reproduces v2's existing codes and texts
/// (SP-1). Under v3 no family is admitted yet.
pub(crate) fn admission(contract: ExactContract, family: ExactFamily) -> Admission {
    match contract {
        ExactContract::StraightV2 => {
            let (code, message) = straight_v2_refusal(family);
            Admission::Refused { code, message: message.to_string() }
        }
        ExactContract::PressureV3 => Admission::Refused {
            code: FAMILY_NOT_ADMITTED,
            message: format!(
                "{} is not yet admitted under 3.0.0/exact_pressure_v3; the exact route refuses it rather than analyse it as straight pipe{}",
                family.name(),
                family.note()
            ),
        },
    }
}

/// v2's codes and texts, verbatim from its first exact composition.
fn straight_v2_refusal(family: ExactFamily) -> (&'static str, &'static str) {
    match family {
        ExactFamily::NonlinearSupport | ExactFamily::ConstantEffortSupport => (
            "EXACT_PRESSURE_COMPOSITION_UNSUPPORTED",
            "the first exact pressure profile supports linear restraints and springs, not nonlinear or constant-effort support composition",
        ),
        ExactFamily::Combination => (
            "EXACT_PRESSURE_COMBINATION_UNSUPPORTED",
            "exact pressure combinations require signed physical-state combination and pressure-region evidence; scalar row algebra is not a valid fallback",
        ),
        ExactFamily::EquivalentStatic => (
            "EXACT_PRESSURE_COMPOSITION_UNSUPPORTED",
            "equivalent-static generation is outside the first exact pressure composition",
        ),
        // Metadata-only fitting records are also excluded: accepting one as a
        // straight span would misrepresent the explicit first composition.
        _ => (
            "EXACT_PRESSURE_COMPOSITION_UNSUPPORTED",
            "the first exact pressure profile supports straight circular pipes only; fitting/component records require their own integrated mechanics proof",
        ),
    }
}

/// The family of a component record. An explicit objective connector takes
/// precedence over the component's kind.
pub(crate) fn component_family(component: &PreviewComponent) -> ExactFamily {
    if component.objective_connector.is_some() {
        return ExactFamily::ObjectiveConnector;
    }
    let consumption = component
        .mechanics_interface
        .as_ref()
        .and_then(|interface| interface.solver_consumption.as_deref());
    match component.kind.as_str() {
        "bend" | "elbow" if super::is_curved_bend_macro_component(component) => {
            ExactFamily::RealizedBend
        }
        "bend" | "elbow" => ExactFamily::GeometryOnlyBend,
        "valve" => ExactFamily::Valve,
        "flange" => ExactFamily::Flange,
        "reducer" => ExactFamily::Reducer,
        "branch" | "tee" | "branch_connection" => ExactFamily::Branch,
        "rigid" | "specialty" => ExactFamily::RigidComponent,
        "expansion_joint" if consumption == Some("not_solver_consumed") && !has_legacy_joint_fields(component) => {
            ExactFamily::ExpansionJointAnnotation
        }
        "expansion_joint" => ExactFamily::ExpansionJointLegacy,
        _ => ExactFamily::OtherComponent,
    }
}

/// T4-I10 section 4.2: a joint with a pipe reference or any of the four
/// user rates is a legacy flexibility joint whatever its consumption mode.
fn has_legacy_joint_fields(component: &PreviewComponent) -> bool {
    component
        .geometry
        .as_ref()
        .is_some_and(|geometry| geometry.expansion_joint_pipe_ref.is_some())
        || component.modifiers.as_ref().is_some_and(|modifiers| {
            modifiers.axial_stiffness_user_value.is_some()
                || modifiers.lateral_stiffness_user_value.is_some()
                || modifiers.angular_stiffness_user_value.is_some()
                || modifiers.torsional_stiffness_user_value.is_some()
        })
}

/// The family of a support outside the straight base, or `None` for a linear
/// restraint or spring (admitted by every exact contract).
pub(crate) fn support_family(support: &PreviewSupport) -> Option<ExactFamily> {
    if support.nonlinear.is_some() {
        Some(ExactFamily::NonlinearSupport)
    } else if super::is_constant_effort_support(support) {
        Some(ExactFamily::ConstantEffortSupport)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [ExactFamily; 15] = [
        ExactFamily::RealizedBend,
        ExactFamily::GeometryOnlyBend,
        ExactFamily::Valve,
        ExactFamily::Flange,
        ExactFamily::Reducer,
        ExactFamily::Branch,
        ExactFamily::RigidComponent,
        ExactFamily::ObjectiveConnector,
        ExactFamily::ExpansionJointAnnotation,
        ExactFamily::ExpansionJointLegacy,
        ExactFamily::OtherComponent,
        ExactFamily::NonlinearSupport,
        ExactFamily::ConstantEffortSupport,
        ExactFamily::Combination,
        ExactFamily::EquivalentStatic,
    ];

    #[test]
    fn no_family_is_admitted_yet_and_each_v3_refusal_names_its_family() {
        for family in ALL {
            match admission(ExactContract::PressureV3, family) {
                Admission::Refused { code, message } => {
                    assert_eq!(code, FAMILY_NOT_ADMITTED);
                    assert!(message.starts_with(family.name()), "{message}");
                    assert!(message.contains("not yet admitted under 3.0.0/exact_pressure_v3"));
                }
                Admission::Admitted => panic!("{family:?} admitted before its unit"),
            }
            assert!(matches!(
                admission(ExactContract::StraightV2, family),
                Admission::Refused { code, .. } if code != FAMILY_NOT_ADMITTED
            ));
        }
    }

    #[test]
    fn contract_identity_is_exact_on_both_fields() {
        for (version, mode, expected) in [
            (Some("2.0.0"), Some("exact_straight_pressure_v2"), Some(ExactContract::StraightV2)),
            (Some("3.0.0"), Some("exact_pressure_v3"), Some(ExactContract::PressureV3)),
            (Some("3.0.0"), Some("exact_straight_pressure_v2"), None),
            (Some("2.0.0"), Some("exact_pressure_v3"), None),
            (None, Some("exact_pressure_v3"), None),
            (Some("3.0.0"), None, None),
            (Some("1.0.0"), Some("legacy_pressure_v1"), None),
        ] {
            assert_eq!(ExactContract::from_declared(version, mode), expected, "{version:?} {mode:?}");
        }
        assert!(ExactContract::PressureV3.admits_negative_pressure());
        assert!(!ExactContract::StraightV2.admits_negative_pressure());
    }
}
