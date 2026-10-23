//! The current exact route's contract identity and family admission seam.
//!
//! Model 0.7.0 declares `3.0.0/exact_pressure_v3` explicitly; an absent
//! contract selects the pressure-free preview route. Straight circular members
//! and linear restraints or springs are the base. Realized bends and objective
//! connectors have their own mechanics, evidence and readers. Other families
//! retain their named refusals. Admission of another family must supply those
//! consumers together; lifting a refusal alone cannot establish mechanics.

use super::{FormulationBasis, PreviewComponent, PreviewModel, PreviewSupport};

/// The ordinary result semantics of the current exact contract.
pub(crate) const PRESSURE_SEMANTIC_CONTRACT_ID: &str =
    "openpipestress.result_semantics/0.3.0/pressure-2";
/// The captured current exact contract's retained-source publication.
pub(crate) const PRESSURE_SOURCE_SEMANTIC_CONTRACT_ID: &str =
    "openpipestress.result_semantics/0.3.0/pressure-source-1";
/// The formulation profile of every `pressure-2` envelope.
pub(crate) const PRESSURE_PROFILE_ID: &str = "exact_pressure_v3";

/// The named seam refusal for a family a v3 document may not yet contain.
pub(crate) const FAMILY_NOT_ADMITTED: &str = "EXACT_PRESSURE_FAMILY_NOT_ADMITTED";

/// The sole exact pressure contract declared on a current model document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExactContract {
    /// `3.0.0/exact_pressure_v3`: the successor, admitted family by family.
    PressureV3,
}

impl ExactContract {
    pub(crate) const fn version(self) -> &'static str {
        match self {
            ExactContract::PressureV3 => "3.0.0",
        }
    }

    pub(crate) const fn mode(self) -> &'static str {
        match self {
            ExactContract::PressureV3 => "exact_pressure_v3",
        }
    }

    /// The contract named by an authored (version, mode) pair, on any document.
    pub(crate) fn from_declared(version: Option<&str>, mode: Option<&str>) -> Option<Self> {
        [ExactContract::PressureV3]
            .into_iter()
            .find(|contract| version == Some(contract.version()) && mode == Some(contract.mode()))
    }

    /// The exact contract of a current document with an explicit v3 identity.
    pub(crate) fn of(model: &PreviewModel) -> Option<Self> {
        if model.schema_version != crate::MODEL_SCHEMA_VERSION {
            return None;
        }
        model.pressure_contract.as_ref().and_then(|contract| {
            Self::from_declared(contract.version.as_deref(), contract.mode.as_deref())
        })
    }

}

/// v3's own formulation basis (its approximation text): the v3 join's
/// formulation text. It names the admitted families, the shear model and the
/// D-5 section bases, states the D-3 exclusions and that external-pressure
/// stability and collapse are not assessed. An authored load/reference-state
/// group adds the state method's statements; a model with an objective
/// connector (T4-U3) then adds the connector law's statement. `selected` names
/// the publication, not whether a model is in the source family: an
/// unselected envelope remains pressure-2. (T4-U6 removed the held pre-join
/// text, which only a test helper still reached.)
pub(crate) fn pressure_v3_formulation_basis_with_source(
    load_state: bool,
    connectors: bool,
    selected: bool,
) -> FormulationBasis {
    // Whole static strings: neither identity nor limitations are substituted at
    // run time. D-5/D-6 (DEL-05-03 Design `section-bases-and-timoshenko.md`
    // §2.4): the identity's single-section parenthesis becomes the shear-model
    // parenthesis, and the section-bases sentences follow the sentence that
    // names the admitted families.
    let identity = if selected {
        "3.0.0/exact_pressure_v3 under pressure-source-1 semantics. Admitted families: small-displacement homogeneous-isotropic straight circular pipe members (shear deformation as the contract declares: euler_bernoulli, or timoshenko_annulus_energy_v1 with the energy-matched annular shear factor, whose nodal rotations are section rotations; G is derived from E/nu) and linear restraints or springs; an expansion joint authored as an objective connector is admitted only as the connector limitation states. Section bases: stiffness and pipe metal mass use the nominal wall; contents mass and pressure end-cap areas use the reduced bore; every stress row is published on the nominal section and on the reduced section (wall minus mill tolerance and corrosion allowance) and names its basis, and each basis has its own maximum and headline; no governing basis is chosen. Each basis's Lamé stresses are those of its own annulus, while the axial membrane stress divides the one wall force by that basis's area. The selected retained-source response uses the contract's shear model and publishes its stress rows on both section bases. Every other component, support, combination or equivalent-static family is refused by name (EXACT_PRESSURE_FAMILY_NOT_ADMITTED) until a later contract revision admits it."
    } else {
        "3.0.0/exact_pressure_v3 under pressure-2 semantics. Admitted families: small-displacement homogeneous-isotropic straight circular pipe members (shear deformation as the contract declares: euler_bernoulli, or timoshenko_annulus_energy_v1 with the energy-matched annular shear factor, whose nodal rotations are section rotations; G is derived from E/nu) and linear restraints or springs; realized curved bends are Euler-Bernoulli under either shear model and, with expansion joints authored as objective connectors, are admitted only as the bend and connector limitations state. Section bases: stiffness and pipe metal mass use the nominal wall; contents mass and pressure end-cap areas use the reduced bore; every stress row is published on the nominal section and on the reduced section (wall minus mill tolerance and corrosion allowance) and names its basis, and each basis has its own maximum and headline; no governing basis is chosen. Each basis's Lamé stresses are those of its own annulus, while the axial membrane stress divides the one wall force by that basis's area. Every other component, support, combination or equivalent-static family is refused by name (EXACT_PRESSURE_FAMILY_NOT_ADMITTED) until a later contract revision admits it."
    };
    let source = if selected {
        "Every member section declares its corrosion allowance and the contract declares its shear model. This pressure-source-1 envelope carries a selected retained-source response. The source family is straight members, linear restraints or springs, nodal force and moment loads and explicitly empty pressure regions; an authored load/reference-state group includes resolved eigenloads and prescribed motions. Other source families are declined by name. Other cases keep their ordinary recovery and standing. No code compliance or professional acceptance is produced, and actual numerical admission remains separate."
    } else {
        "Every member section declares its corrosion allowance and the contract declares its shear model. This envelope publishes ordinary structural recovery. Retained-source recovery is available only for straight members, linear restraints or springs, nodal force and moment loads and explicitly empty pressure regions; an authored load/reference-state group includes resolved eigenloads and prescribed motions. Other source families are declined by name. No code compliance or professional acceptance is produced, and actual numerical admission remains separate."
    };
    let mut limitations = vec![
        identity.to_string(),
        "Pressure is a signed internal differential with zero external pressure increment, uniform within each explicit case-scoped collinear equal-bore region with explicit closure-transfer paths. A negative differential is admitted, but external-pressure stability and collapse are not assessed.".to_string(),
        "Not modelled: steady-flow momentum and transient pressure loads; bend opening under pressure (Bourdon effect); pressure stiffening of flexibility factors and stress intensification; ovalization.".to_string(),
        "Mechanical, pressure-eigen and cap-transfer contributions remain distinct; rounded cap/eigen ledgers are observational and do not replace source-grouped pressure assembly.".to_string(),
        "Structural line loads act as entered; contents density does not imply hydrostatic pressure head or a coupled static-fluid pressure/weight state.".to_string(),
        "Signed support actions are attributed only for admitted linear restraints and springs; ambiguous coincident rigid attribution is refused.".to_string(),
        "The circular normal-stress maximum bounds the supplied binary64 polynomial coefficient enclosures; solution error is separate, and incomplete coverage withholds the headline.".to_string(),
        source.to_string(),
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
    if connectors {
        limitations.push(super::joint::CONNECTOR_LIMITATION.to_string());
    }
    FormulationBasis { profile_id: PRESSURE_PROFILE_ID.to_string(), limitations }
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
    /// The family as named in diagnostics (the head of its v3 refusal,
    /// [`ExactFamily::v3_refusal`]; pinned by this module's tests).
    #[cfg(test)]
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
    #[cfg(test)]
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

    /// The v3 refusal, `"{name} is not yet admitted under 3.0.0/exact_pressure_v3;
    /// the exact route refuses it rather than analyse it as straight pipe{note}"`,
    /// held whole as one static text per family: the seam composes no text at
    /// run time (T3 O-10), and the tests pin each text to its name and note.
    const fn v3_refusal(self) -> &'static str {
        match self {
            ExactFamily::RealizedBend => {
                "realized curved bend (curved_bend_macro_element) is not yet admitted under 3.0.0/exact_pressure_v3; the exact route refuses it rather than analyse it as straight pipe"
            }
            ExactFamily::GeometryOnlyBend => {
                "geometry-only bend (a straight chord with no flexibility) is not admitted under 3.0.0/exact_pressure_v3; the exact route refuses it rather than analyse it as straight pipe; realize this bend (mechanics_interface.solver_consumption curved_bend_macro_element, with its bend radius and the user's flexibility factor k) to analyse it on the exact route; geometry-only bends never carry pressure on the exact route (D-2) and remain on the pressure-free route"
            }
            ExactFamily::Valve => {
                "valve is not yet admitted under 3.0.0/exact_pressure_v3; the exact route refuses it rather than analyse it as straight pipe"
            }
            ExactFamily::Flange => {
                "flange is not yet admitted under 3.0.0/exact_pressure_v3; the exact route refuses it rather than analyse it as straight pipe"
            }
            ExactFamily::Reducer => {
                "reducer is not yet admitted under 3.0.0/exact_pressure_v3; the exact route refuses it rather than analyse it as straight pipe"
            }
            ExactFamily::Branch => {
                "branch connection (tee) is not yet admitted under 3.0.0/exact_pressure_v3; the exact route refuses it rather than analyse it as straight pipe"
            }
            ExactFamily::RigidComponent => {
                "rigid or specialty component is not yet admitted under 3.0.0/exact_pressure_v3; the exact route refuses it rather than analyse it as straight pipe"
            }
            ExactFamily::ObjectiveConnector => {
                "objective connector is not yet admitted under 3.0.0/exact_pressure_v3; the exact route refuses it rather than analyse it as straight pipe"
            }
            ExactFamily::ExpansionJointAnnotation => {
                "expansion joint (annotation only, not_solver_consumed) is not yet admitted under 3.0.0/exact_pressure_v3; the exact route refuses it rather than analyse it as straight pipe; annotation-only joints are analysed as pipe on the pressure-free route only"
            }
            ExactFamily::ExpansionJointLegacy => {
                "legacy expansion joint is not yet admitted under 3.0.0/exact_pressure_v3; the exact route refuses it rather than analyse it as straight pipe"
            }
            ExactFamily::OtherComponent => {
                "component is not yet admitted under 3.0.0/exact_pressure_v3; the exact route refuses it rather than analyse it as straight pipe"
            }
            ExactFamily::NonlinearSupport => {
                "nonlinear support is not yet admitted under 3.0.0/exact_pressure_v3; the exact route refuses it rather than analyse it as straight pipe"
            }
            ExactFamily::ConstantEffortSupport => {
                "constant-effort support is not yet admitted under 3.0.0/exact_pressure_v3; the exact route refuses it rather than analyse it as straight pipe"
            }
            ExactFamily::Combination => {
                "load combination is not yet admitted under 3.0.0/exact_pressure_v3; the exact route refuses it rather than analyse it as straight pipe"
            }
            ExactFamily::EquivalentStatic => {
                "equivalent-static generation is not yet admitted under 3.0.0/exact_pressure_v3; the exact route refuses it rather than analyse it as straight pipe"
            }
        }
    }
}

/// The table's verdict for one family under one contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Admission {
    /// The extension point: T4-U3 admits the objective connector; T4-U2 the
    /// realized bend.
    Admitted,
    /// A refusal's code and text, both static (no text is composed here).
    Refused { code: &'static str, message: &'static str },
}

/// The current admission table: the objective connector and realized bend
/// are admitted with their integrated mechanics.
pub(crate) fn admission(contract: ExactContract, family: ExactFamily) -> Admission {
    match (contract, family) {
        // T4-U3: an explicit objective connector on v3, with its own
        // classifier (`joint::classify_connectors`), mechanics and evidence.
        (ExactContract::PressureV3, ExactFamily::ObjectiveConnector) => Admission::Admitted,
        // T4-U2: a realized curved bend on v3, with its member-owned pressure
        // term, chain topology and arc recovery (`bend_pressure`).
        (ExactContract::PressureV3, ExactFamily::RealizedBend) => Admission::Admitted,
        (ExactContract::PressureV3, _) => Admission::Refused {
            code: FAMILY_NOT_ADMITTED,
            message: family.v3_refusal(),
        },
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
        // D-4 (T4-U3): an explicit annotation is exempt from the legacy
        // recognition, whatever other fields it carries.
        "expansion_joint" if consumption == Some("not_solver_consumed") => {
            ExactFamily::ExpansionJointAnnotation
        }
        "expansion_joint" => ExactFamily::ExpansionJointLegacy,
        _ => ExactFamily::OtherComponent,
    }
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
    fn only_the_connector_and_the_realized_bend_are_admitted_and_each_v3_refusal_names_its_family() {
        for family in ALL {
            match admission(ExactContract::PressureV3, family) {
                Admission::Admitted
                    if matches!(family, ExactFamily::ObjectiveConnector | ExactFamily::RealizedBend) => {}
                // T4-U2 (D-2): a chord bend is never admitted; its text asks
                // the user to realize the bend.
                Admission::Refused { code, message } if family == ExactFamily::GeometryOnlyBend => {
                    assert_eq!(code, FAMILY_NOT_ADMITTED);
                    assert!(message.starts_with(family.name()), "{message}");
                    assert!(message.contains("is not admitted under 3.0.0/exact_pressure_v3"));
                    assert!(message.contains("realize this bend"));
                    assert!(message.ends_with(family.note()), "{message}");
                }
                Admission::Refused { code, message } => {
                    assert_eq!(code, FAMILY_NOT_ADMITTED);
                    assert!(message.starts_with(family.name()), "{message}");
                    assert!(message.contains("not yet admitted under 3.0.0/exact_pressure_v3"));
                    // The static text is exactly the composed one (T4-U2a's
                    // original `format!`), so the bytes are unchanged.
                    assert_eq!(
                        message,
                        format!(
                            "{} is not yet admitted under 3.0.0/exact_pressure_v3; the exact route refuses it rather than analyse it as straight pipe{}",
                            family.name(),
                            family.note()
                        )
                    );
                }
                Admission::Admitted => panic!("{family:?} admitted before its unit"),
            }
        }
    }

    #[test]
    fn contract_identity_is_exact_on_both_fields() {
        for (version, mode, expected) in [
            (Some("2.0.0"), Some("exact_straight_pressure_v2"), None),
            (Some("3.0.0"), Some("exact_pressure_v3"), Some(ExactContract::PressureV3)),
            (Some("3.0.0"), Some("exact_straight_pressure_v2"), None),
            (Some("2.0.0"), Some("exact_pressure_v3"), None),
            (None, Some("exact_pressure_v3"), None),
            (Some("3.0.0"), None, None),
            (Some("1.0.0"), Some("legacy_pressure_v1"), None),
        ] {
            assert_eq!(ExactContract::from_declared(version, mode), expected, "{version:?} {mode:?}");
        }
    }
}
