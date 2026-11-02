//! The current exact route's contract identity and family admission seam.
//!
//! Model 0.9.0 declares `3.0.0/exact_pressure_v3` explicitly; an absent
//! contract selects the pressure-free preview route. Straight circular members
//! and linear restraints or springs are the base. Realized bends and objective
//! connectors have their own mechanics, evidence and readers. W-15 (DEL-03-05
//! Design `rigid-component-stiffness-scaling.md` §2.7) admits valve, flange,
//! reducer, rigid and specialty components with the DEC-045 stiffness scale of
//! their mapped straight member, an equivalent pipe of the mapped section. A
//! reducer whose mapped member belongs to a pressure region is admitted only
//! as RT-A3 reducer increment 2a's concentric pressure-bore step (DEL-04-07
//! `route-unification-r4.md` §00 C-3/C-4, §3.1): the body is its own
//! single-member region, collinear between one straight neighbour of another
//! region at each end, every terminal there transferring to the wall, so the
//! junction caps carry the bore-step load ([`reducers_in_regions`] and
//! `pressure_runtime`'s geometric guard). Every other reducer in a region is
//! refused by [`REDUCER_IN_REGION`]. RT-A3 increment 1 (DEL-04-07 Design
//! `pressure-contract.md`, "Tees and pressure junctions") admits the branch
//! connection (tee) as the metadata record it is on the pressure-free route,
//! with its user SIFs as intensified rows on both section bases
//! (`u4b_rows::append_branch_intensified`). Other families retain their named
//! refusals. Admission
//! of another family must supply those consumers together; lifting a refusal
//! alone cannot establish mechanics.

use super::pressure_runtime::{PressureRegionInput, PressureTerminalInput};
use super::{FormulationBasis, PreviewComponent, PreviewLoadCase, PreviewModel, PreviewSupport};
use std::collections::HashSet;

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

/// W-15 §2.7, narrowed by RT-A3 reducer increment 2a: the case-specific
/// refusal of a reducer whose mapped member belongs to a pressure region of
/// that case, other than the admitted concentric pressure-bore step.
pub(crate) const REDUCER_IN_REGION: &str = "EXACT_PRESSURE_REDUCER_IN_REGION";
/// The static text of [`REDUCER_IN_REGION`] (the seam composes no text).
pub(crate) const REDUCER_IN_REGION_TEXT: &str = "reducer maps a member of this pressure region but is not the admitted concentric pressure-bore step; under 3.0.0/exact_pressure_v3 a reducer in a pressure region is admitted only when its mapped member exists, ends at the reducer's node and is the only member of its own region, no nonzero eccentric_offset is authored (an absent offset declares nothing), and at each end of the member exactly one other pipe meets it, a straight member of another region of the case whose region has a terminal there, continuing collinear in the opposite direction within the 64*epsilon representation guard, with transfers_to_wall at every terminal at that node, so the junction caps carry the bore-step load; a multi-member region, an eccentric or slanted body, a tee, bend or connector at an end, or a dry, free or branched end is refused rather than analysed as equal-bore pipe; the absolute direction-agreement guard can also refuse an intended collinear skewed run when a short body is rounded far from the origin, and that refusal does not establish physical eccentricity; a reducer whose mapped member lies outside every pressure region is admitted with its DEC-045 stiffness scale";

/// §8.30 correction 1 (RT-A3; R-4 design C-1): the case-specific refusal of a
/// node where terminals of two or more pressure regions meet and at least one
/// of them is `separately_supported_or_compensated`
/// (`pressure_runtime::refuse_compensated_shared_terminals`). RT-A3 increment
/// 1 (D-1) represents a pressure junction implicitly by such a shared node
/// with every terminal there `transfers_to_wall`, so this refusal stays: a
/// compensated terminal cannot be part of a junction.
pub(crate) const SHARED_TERMINAL_JUNCTION: &str =
    "EXACT_PRESSURE_SHARED_TERMINAL_JUNCTION_UNSUPPORTED";
/// The static tail of [`SHARED_TERMINAL_JUNCTION`]'s text, after the node,
/// its regions and the compensated regions are named.
pub(crate) const SHARED_TERMINAL_JUNCTION_TEXT: &str = "3.0.0/exact_pressure_v3 represents a pressure junction as a node where regions end with transfers_to_wall at every terminal, whose caps (p*Ai along each member's outward axis) sum to the junction's pressure load -sum(p*Ai*s); a compensated terminal at the node would omit its member's share from the pipe solve, so the exact route refuses the node rather than analyse it without that load; the refusal also applies where the omitted share or the resultant is zero or cancels, such as a zero-pressure compensated branch or equal opposed compensated caps; declare transfers_to_wall at every terminal of the node";

/// RT-A3 increment 1 (D-1): the warning of a pressure junction whose regions
/// carry different pressures (`pressure_runtime::junction_closure_warnings`).
pub(crate) const JUNCTION_PRESSURES_DIFFER: &str = "EXACT_PRESSURE_JUNCTION_PRESSURES_DIFFER";
/// The static tail of [`JUNCTION_PRESSURES_DIFFER`]'s text.
pub(crate) const JUNCTION_PRESSURES_DIFFER_TEXT: &str = "the junction is analysed as a closure inside the fitting, such as a blind or a check valve between the regions, and its pressure load is the exact sum of the regions' caps at the node; if the regions share one fluid space, correct their pressures; confirm the closure is intended";
/// RT-A3 increment 1 (D-1): the warning of a pipe at a pressure junction that
/// belongs to no pressure region of the case.
pub(crate) const JUNCTION_PIPE_OUTSIDE_REGIONS: &str = "EXACT_PRESSURE_JUNCTION_PIPE_OUTSIDE_REGIONS";
/// The static tail of [`JUNCTION_PIPE_OUTSIDE_REGIONS`]'s text.
pub(crate) const JUNCTION_PIPE_OUTSIDE_REGIONS_TEXT: &str = "the pipe is analysed as unpressurized and closed off at the node, and the junction's pressure load is the exact sum of the regions' caps there; if the pipe carries the same fluid, add it to a pressure region ending at the node; confirm the closure is intended";

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
        "3.0.0/exact_pressure_v3 under pressure-source-1 semantics. Admitted families: small-displacement homogeneous-isotropic straight circular pipe members (shear deformation as the contract declares: euler_bernoulli, or timoshenko_annulus_energy_v1 with the energy-matched annular shear factor, whose nodal rotations are section rotations; G is derived from E/nu) and linear restraints or springs; an expansion joint authored as an objective connector is admitted only as the connector limitation states. Section bases: stiffness and pipe metal mass use the nominal wall; contents mass and pressure end-cap areas use the reduced bore; every stress row is published on the nominal section and on the reduced section (wall minus mill tolerance and corrosion allowance) and names its basis, and each basis has its own maximum and headline; no governing basis is chosen. Each basis's Lamé stresses are those of its own annulus, while the axial membrane stress divides the one wall force by that basis's area. The selected retained-source response uses the contract's shear model and publishes its stress rows on both section bases. Every other component or support family is refused by name (EXACT_PRESSURE_FAMILY_NOT_ADMITTED) until a later contract revision admits it."
    } else {
        "3.0.0/exact_pressure_v3 under pressure-2 semantics. Admitted families: small-displacement homogeneous-isotropic straight circular pipe members (shear deformation as the contract declares: euler_bernoulli, or timoshenko_annulus_energy_v1 with the energy-matched annular shear factor, whose nodal rotations are section rotations; G is derived from E/nu) and linear restraints or springs; realized curved bends are Euler-Bernoulli under either shear model and, with expansion joints authored as objective connectors, are admitted only as the bend and connector limitations state; valve, flange, rigid and specialty components are admitted as their mapped straight member with the user's DEC-045 stiffness scale, an equivalent pipe of the mapped section whose stress rows are those of that reference section (marked component_body), not a body stress, and reducers when their mapped member lies outside every pressure region or forms a concentric pressure-bore step, the only member of its own region, collinear between single straight members of other regions with transfers_to_wall at both ends, whose junction caps carry the step's pressure load (EXACT_PRESSURE_REDUCER_IN_REGION otherwise); a branch connection (tee) is a record at its node, whose members meet as a rigid joint with no branch flexibility; a tee consumed as mechanics_geometry_only gives its user-entered header and branch SIFs as intensified rows at the ends of its named straight members on each section basis, not a code stress, and a realized arc it names carries none. Pressure regions meeting at a node where every terminal transfers to the wall form a junction, whose caps sum to the fitting's pressure load. Section bases: stiffness and pipe metal mass use the nominal wall; contents mass and pressure end-cap areas use the reduced bore; every stress row is published on the nominal section and on the reduced section (wall minus mill tolerance and corrosion allowance) and names its basis, and each basis has its own maximum and headline; no governing basis is chosen. Each basis's Lamé stresses are those of its own annulus, while the axial membrane stress divides the one wall force by that basis's area. Every other component or support family is refused by name (EXACT_PRESSURE_FAMILY_NOT_ADMITTED) until a later contract revision admits it."
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
            "Ordinary applied loads are exactly the case's declared source ledger with explicit factors; unreferenced stored primitives are excluded. Hydrostatic head, contents-weight state and per-case mass selection are not provided. Generated equivalent-static (seismic or wind) loads have no ledger class and are refused by name (EXACT_EQUIVALENT_STATIC_LOAD_STATE_UNSUPPORTED).".to_string(),
            "History is independent equilibrium only; no installation, contact, friction or predecessor history is represented.".to_string(),
        ]);
    }
    if connectors {
        limitations.push(super::joint::CONNECTOR_LIMITATION.to_string());
    }
    FormulationBasis { profile_id: PRESSURE_PROFILE_ID.to_string(), limitations }
}

/// W-16 slice c (DEL-05-01 Design `exact-equivalent-static.md` §3): a case's
/// `equivalent_static` on a load/reference-state document. Generated loads
/// have no ledger class there, so they are refused for every authored case,
/// listed in the group or not, before the effective cases are selected.
pub(crate) const EQUIVALENT_STATIC_LOAD_STATE: &str = "EXACT_EQUIVALENT_STATIC_LOAD_STATE_UNSUPPORTED";
/// The static text of [`EQUIVALENT_STATIC_LOAD_STATE`].
pub(crate) const EQUIVALENT_STATIC_LOAD_STATE_TEXT: &str = "equivalent_static generation is not provided on a load/reference-state document: a case's applied loads there are exactly its declared source ledger, and generated seismic or wind loads have no ledger class; the exact route refuses the generation rather than apply loads outside the ledger";

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
    /// realized bend; W-15 the rigid family; RT-A3 the branch connection.
    Admitted,
    /// A refusal's code and text, both static (no text is composed here).
    Refused { code: &'static str, message: &'static str },
}

/// The current admission table: the objective connector, the realized bend,
/// the rigid family, the branch connection, equivalent-static generation and
/// load combinations are admitted with their integrated mechanics.
pub(crate) fn admission(contract: ExactContract, family: ExactFamily) -> Admission {
    match (contract, family) {
        // W-15 (DEL-03-05 Design `rigid-component-stiffness-scaling.md` §2.7):
        // valve, flange, reducer, rigid and specialty components solve as their
        // mapped straight member with the DEC-045 scale (`rigid_component`),
        // an equivalent pipe of the mapped section: bore continuity, cap thrust
        // and the annular kappa are the mapped section's. A reducer in a
        // pressure region is admitted case by case only as RT-A3 2a's
        // concentric pressure-bore step and refused otherwise
        // ([`reducers_in_regions`]).
        (
            ExactContract::PressureV3,
            ExactFamily::Valve | ExactFamily::Flange | ExactFamily::Reducer | ExactFamily::RigidComponent,
        ) => Admission::Admitted,
        // T4-U3: an explicit objective connector on v3, with its own
        // classifier (`joint::classify_connectors`), mechanics and evidence.
        (ExactContract::PressureV3, ExactFamily::ObjectiveConnector) => Admission::Admitted,
        // T4-U2: a realized curved bend on v3, with its member-owned pressure
        // term, chain topology and arc recovery (`bend_pressure`).
        (ExactContract::PressureV3, ExactFamily::RealizedBend) => Admission::Admitted,
        // W-16 slice c (DEL-05-01 Design `exact-equivalent-static.md`): a
        // case's DEC-068 seismic or wind generation on a plain v3 document,
        // on the route's D-5 mass basis; load/reference-state documents and
        // replaced spans keep named refusals; a Timoshenko partial span takes
        // the exact Timoshenko vector (W-16t).
        (ExactContract::PressureV3, ExactFamily::EquivalentStatic) => Admission::Admitted,
        // W-16 slice b (DEL-05-02 Design `exact-combinations.md`): model-authored
        // combinations over the primitive cases, formed from their published
        // rows (`exact_combinations`); the retained sub-route keeps refusing them.
        (ExactContract::PressureV3, ExactFamily::Combination) => Admission::Admitted,
        // RT-A3 increment 1: a branch connection (tee) is the pressure-free
        // route's metadata record at its node (no element, no assembled branch
        // flexibility); its user SIFs give the intensified rows on both bases
        // (`u4b_rows::append_branch_intensified`). Pressure reaches a tee only
        // through regions that end at its node (a region's internal node may
        // not touch an outside pipe), where the terminal caps form the junction.
        (ExactContract::PressureV3, ExactFamily::Branch) => Admission::Admitted,
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

/// RT-A3 reducer 2a: one end of an admitted step body, as the geometric guard
/// (`pressure_runtime`) reads it on the built coordinates.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ReducerStepEnd<'a> {
    /// The body's end node, shared with the neighbour only.
    pub node: &'a str,
    /// The reducer's mapped (body) member.
    pub body: &'a str,
    /// The one other pipe at the node, a straight member of another region.
    pub neighbour: &'a str,
}

/// A reducer whose mapped member is listed in a pressure region of a case.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ReducerInRegion<'a> {
    /// (case, region, reducer, member): the refusal's references.
    pub refs: [&'a str; 4],
    /// `Some` when the step's topology holds (the body's two ends), so the
    /// geometric guard decides; `None` when it is refused on topology.
    pub step: Option<[ReducerStepEnd<'a>; 2]>,
}

/// W-15 §2.7 with RT-A3 reducer increment 2a: each (case, region, reducer,
/// member) where a reducer's mapped member is listed in a pressure region of
/// the case, in region and model order, with its step topology. Region
/// membership is the authored `member_pipe_ids`; the equal-bore guard cannot
/// stand in for this check (it compares traversed members only and never
/// reads the reducer). A reducer whose mapped member lies outside every
/// region, including one mapped at a region's boundary node, is not listed.
///
/// The step topology (unit-free, so it is read before normalization) holds
/// only when: (1) the mapped member exists and ends at the reducer's node
/// (W-15 §2.3); (2) it is the region's only member, with no connector edge;
/// (3) no nonzero `eccentric_offset` is authored (a zero signed component
/// counts zero; an absent slot declares nothing); and (4) at each of the
/// body's two distinct end nodes, both of them the region's terminals, every
/// terminal of the case transfers to the wall, no tee record sits there, and
/// exactly one other pipe is incident: a straight member (no realized or
/// geometry-only bend, expansion-joint or connector-replaced span) listed in
/// another region of the case that has a terminal at that node. The
/// collinear, opposite-direction continuation is judged on the built
/// coordinates by `pressure_runtime` within its 64*epsilon representation
/// guard. Scale eligibility stays W-15's: an unscaled or marker body that
/// meets the step is admitted with its existing warnings, and the end sizes
/// are descriptive and enter nothing.
pub(crate) fn reducers_in_regions<'a>(model: &'a PreviewModel, case: &'a PreviewLoadCase) -> Vec<ReducerInRegion<'a>> {
    let mut found = Vec::new();
    let Some(regions) = case.pressure_regions.as_deref() else { return found };
    let mut not_straight = None;
    for region in regions {
        let Some(members) = &region.member_pipe_ids else { continue };
        let region_id = region.id.as_deref().unwrap_or("");
        for component in model.components.iter().filter(|component| component_family(component) == ExactFamily::Reducer) {
            let Some(pipe) = super::rigid_component::mapped_pipe_ref(component) else { continue };
            if members.iter().any(|member| member == pipe) {
                let step = step_topology(model, regions, region, component, pipe, &mut not_straight);
                found.push(ReducerInRegion { refs: [case.id.as_str(), region_id, component.id.as_str(), pipe], step });
            }
        }
    }
    found
}

/// The (case, region, reducer, member) refusals decided on topology alone, in
/// case, region and model order (`pressure_runtime::validate_profile`).
/// Every listed reducer whose topology holds is judged again with its geometry
/// when its case is assembled.
pub(crate) fn reducer_region_refusals(model: &PreviewModel) -> Vec<[&str; 4]> {
    model
        .load_cases
        .iter()
        .flat_map(|case| reducers_in_regions(model, case))
        .filter(|reducer| reducer.step.is_none())
        .map(|reducer| reducer.refs)
        .collect()
}

/// Members that are not straight run for a step neighbour: any member a
/// component names as a bend or expansion-joint member, and any span an
/// objective connector names (conservatively, whether or not it decodes).
fn non_straight_members(model: &PreviewModel) -> HashSet<&str> {
    let mut found = HashSet::new();
    for component in &model.components {
        if let Some(geometry) = &component.geometry {
            for pipe in [geometry.bend_pipe_ref.as_deref(), geometry.expansion_joint_pipe_ref.as_deref()].into_iter().flatten() {
                found.insert(pipe);
            }
        }
        if let Some(span) = component
            .objective_connector
            .as_ref()
            .and_then(|connector| connector.pointer("/topology/span_ref"))
            .and_then(serde_json::Value::as_str)
        {
            found.insert(span);
        }
    }
    found
}

fn ends_at(terminal: &PressureTerminalInput, node: &str) -> bool {
    terminal.node_ref.as_deref() == Some(node)
}

/// [`reducers_in_regions`]'s conditions (1)-(4) for one listed reducer.
fn step_topology<'a>(
    model: &'a PreviewModel,
    regions: &'a [PressureRegionInput],
    region: &'a PressureRegionInput,
    component: &'a PreviewComponent,
    pipe: &'a str,
    not_straight: &mut Option<HashSet<&'a str>>,
) -> Option<[ReducerStepEnd<'a>; 2]> {
    // (1) W-15 §2.3's mapping.
    let body = model.pipe_segments.iter().find(|candidate| candidate.id == pipe)?;
    if (body.from != component.node && body.to != component.node) || body.from == body.to {
        return None;
    }
    // (2) The region's only member.
    if !matches!(region.member_pipe_ids.as_deref(), Some([only]) if only == pipe)
        || region.member_connector_ids.as_ref().is_some_and(|ids| !ids.is_empty())
    {
        return None;
    }
    // (3) No authored nonzero offset; a non-finite component is nonzero.
    if super::rigid_component::eccentric_offset(component)
        .is_some_and(|offset| [offset.x, offset.y, offset.z].iter().any(|value| *value != 0.0))
    {
        return None;
    }
    // (4) Each end: a region terminal joining one straight neighbour of
    // another region, every terminal at the node transferring.
    let terminals = region.terminals.as_deref()?;
    let mut end = |node: &'a str| -> Option<ReducerStepEnd<'a>> {
        if !terminals.iter().any(|terminal| ends_at(terminal, node)) {
            return None;
        }
        if regions
            .iter()
            .flat_map(|other| other.terminals.iter().flatten())
            .any(|terminal| ends_at(terminal, node) && terminal.closure_transfer.as_deref() != Some("transfers_to_wall"))
        {
            return None;
        }
        if model.components.iter().any(|other| component_family(other) == ExactFamily::Branch && other.node == node) {
            return None;
        }
        let mut incident = model
            .pipe_segments
            .iter()
            .filter(|other| other.id != body.id && (other.from == node || other.to == node));
        let neighbour = incident.next()?;
        if incident.next().is_some() || neighbour.from == neighbour.to {
            return None;
        }
        if not_straight.get_or_insert_with(|| non_straight_members(model)).contains(neighbour.id.as_str()) {
            return None;
        }
        let joined = regions.iter().any(|other| {
            !std::ptr::eq(other, region)
                && other.member_pipe_ids.iter().flatten().any(|member| *member == neighbour.id)
                && other.terminals.iter().flatten().any(|terminal| ends_at(terminal, node))
        });
        joined.then_some(ReducerStepEnd { node, body: body.id.as_str(), neighbour: neighbour.id.as_str() })
    };
    Some([end(body.from.as_str())?, end(body.to.as_str())?])
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
    fn only_the_connector_the_realized_bend_the_rigid_family_the_branch_equivalent_static_and_combinations_are_admitted_and_each_v3_refusal_names_its_family() {
        for family in ALL {
            match admission(ExactContract::PressureV3, family) {
                Admission::Admitted
                    if matches!(family, ExactFamily::ObjectiveConnector | ExactFamily::RealizedBend
                        | ExactFamily::Valve | ExactFamily::Flange | ExactFamily::Reducer
                        | ExactFamily::RigidComponent | ExactFamily::Branch
                        | ExactFamily::EquivalentStatic | ExactFamily::Combination) => {}
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
