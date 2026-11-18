//! The spectrum result document (DEL-04-10 REQ-006): family `openpipestress.spectrum_result`,
//! profile `product_preview_spectrum_v1`, semantic table `openpipestress.result_semantics/spectrum-1`,
//! outside the `results.v0.3` envelope.
//!
//! [`spectrum_result_document`] is a pure function from structured inputs to the document value:
//! the model document (which holds the spectra) and the spectrum request it binds, the statement of
//! the modal basis the result rests on, the census of the built dynamic model and the signed
//! contributions with their formation records. It runs the combinations ([`crate::combine`]) and
//! returns either the solved document or the refused one, carrying the refusals of the first stage
//! that refused. It forms no modal response and no rigid response: the signed contributions are its
//! inputs.
//!
//! **Stages**, each run only when every earlier one found nothing:
//! 1. the bindings: the model and the request are each within the checked canonical profile,
//!    depth first;
//! 2. the inputs against one another: the request's choices, the model's spectra the request
//!    names (each identity held once), the combination input's methods, missing-mass fields,
//!    directions and damping, each direction's formation record, the modal statement (its
//!    lengths, extraction endpoint and counts, modal result binding, mass operands, cluster rule,
//!    frequency relation and state), the census (closed on itself, frames, entries) and the origin;
//! 3. the rows: each row's own fields (`SPECTRUM_ROW_INVALID`: a blank identity, an end action in a
//!    frame its member family does not admit, a member stated as two families, a constraint's
//!    holders), then the roster the census gives (each identity once), then the zero contribution
//!    of each component a census rigid constraint restrains;
//! 4. the modes' structure (`SPECTRUM_MODAL_METADATA_INVALID`), then the range of each periodic
//!    mode and of f_r in each spectrum and the extraction premise;
//! 5. the combinations, with their own checks of the choices and contributions;
//! 6. the published numbers, each within the checked canonical profile;
//! 7. the assembled document as a whole within the checked canonical profile.
//!
//! **Formation records.** The signed contributions enter as [`SignedContributions`]: with them, for
//! each requested direction, the record made where they were formed of the spectrum identity, the
//! digest of that spectrum's record in the model that supplied the table, and the scale factor.
//! The assembler refuses a direction whose record is missing, repeated or not the bound request's
//! spectrum and scale factor and the bound model's record. This establishes which request choices
//! and which spectrum record the producer was told the contributions were formed with, and that
//! they are the bound request's and model's; it does not establish that the contributions were
//! formed correctly from them.
//!
//! **Conventions taken from the modal result family.** An identity block on every document; the
//! checked canonical profile `openpipestress_jcs_ijson_v1` for every hash, each the SHA-256 of a
//! value's canonical text; sealed rows (`row_id`, `ordinal`, `checksum`) in published tables with a
//! row count and a digest over the rows' checksums; scalar values as value objects with catalog
//! units; fixed limitation text; a committed semantic table named by id and SHA-256; a refused form
//! with a sealed refusals table. The document states that it is experimental and not a public
//! engineering result.
//!
//! **Allocation** (by reading the code; not measured). With Q rows, M extracted modes (P of them
//! periodic), C clusters (C_p periodic, C₂ of them periodic with more than one mode), D ≤ 3
//! requested directions, S spectra used with K knots in all, and E census entries:
//! - The document holds O(Q·D·(P + C₂ + 1) + M + C + K + E + R) values, where R is the number of
//!   rule-statement rows: C_p·(C_p − 1)/2 correlation rows under CQC, at most C_p group entries
//!   under grouping, and under the ten percent method one row per coupled pair, which is up to
//!   C_p·(C_p − 1)/2, quadratic in the periodic clusters.
//! - Before the combinations: the admitted tables of the S spectra used; an ordered map and an
//!   ordered set of up to Q row identities for the roster; an ordered set of borrowed identity
//!   tuples per census table (E in all) and of the restrained (node, component) pairs; the
//!   classified clusters (C).
//! - The combinations' own working set (`combine`): exact sums of Q·D·C_p dyadics and, under CQC,
//!   tables of C_p·(C_p − 1)/2 enclosures of ρ for each precision tried.
//! - The projection: an ordered map of the supplied contributions by (direction, mode position),
//!   one of the cluster sums by (cluster, direction), one of the census constraints by (node,
//!   component), and per table an array of its rows' checksums. Each contribution value is copied
//!   from the supplied contributions into its signed row; each row's fields are copied once more
//!   when it is sealed (the row is cloned to be hashed), and building a table copies its rows
//!   again.
//! - Hashing: for each hashed value (the model, the request, the modal state, each sealed row,
//!   each table's checksum array, the semantic table and the whole document) three texts or trees
//!   are held while it is hashed: its compact serialization, the checked parse's tree of that text,
//!   and its canonical text, each proportional to the value's size; the iterative depth check that
//!   comes first holds a stack proportional to the number of the value's container members.
//! - Time beyond the size of what is held: each cell of a signed or cluster-sum row is a lookup in
//!   an ordered map, O(log(D·M)); each periodic mode's frequency and f_r is located in each spectrum
//!   used by binary search over its knots, O((P + 1)·S·log K) in all.
//!
//! Nothing here recurses on a caller's value before its depth is checked: the model, the request
//! and the modal state are checked iteratively against the checked parse's depth limit, the
//! state at its place `/modal_basis/state`.
use crate::combine::{combine, Cluster};
use crate::contract::{
    ClusterClass, ClusterIndex, ClusterStatement, Combination, CombinationRequest, Direction, DirectionalRule, End, Frame, MemberFamily,
    MissingMass, ModalRule, Quantity, ReactionComponent, RigidRule, RowIdentity, SignConvention, SignStatus, QUANTITY_KINDS,
};
use serde::Deserialize;
use crate::dyadic::Dyadic;
use crate::record::{Abscissa, AxisScale, SpectrumRecord};
use crate::refusal::{Refusal, RefusalCode};
use crate::table::{Location, OrdinateMeasure, QueryFrequency, ScaleFactor, SpectrumTable};
use crate::{ConnectorComponent, DisplacementComponent, ForceComponent};
use open_pipe_stress_canonical_json::canonical_json_checked_v1_text;
use open_pipe_stress_units::UnitId;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub const SPECTRUM_RESULT_KIND: &str = "openpipestress.spectrum_result";
pub const SPECTRUM_RESULT_VERSION: &str = "0.1.0";
pub const SPECTRUM_RESULT_SEMANTICS: &str = "openpipestress.result_semantics/spectrum-1";
pub const SPECTRUM_RESULT_PROFILE: &str = "product_preview_spectrum_v1";
pub const SPECTRUM_RESULT_CANONICAL: &str = "openpipestress_jcs_ijson_v1";
/// The hash algorithm every binding and seal names.
pub const SPECTRUM_HASH_ALGORITHM: &str = "sha-256";
/// The document kind of the spectrum request the document binds. Its shape is this unit's
/// stand-in until the spectrum entry states it.
pub const SPECTRUM_REQUEST_KIND: &str = "openpipestress.spectrum_request";

/// The profile's fixed limitations.
pub const SPECTRUM_RESULT_LIMITATIONS: [&str; 3] = [
    "Experimental response-spectrum preview under uniform support motion, formed from the signed modal and rigid \
contributions of a small-displacement, linear-elastic modal basis, with damping represented only through the user's \
pseudo-acceleration spectra; it is not a public engineering result, it is not recorded as an analysis run, and it \
establishes neither physical correctness nor code compliance.",
    "Every row is the response to the support motion alone, about the state the modal model was formed at: a combined \
row is an unsigned magnitude in its stated frame, formed in that frame from signed contributions, and is never a \
signed quantity, a total force with the state's own action, or a stress; a signed contribution or a signed cluster \
sum is not a peak response.",
    "That this document is consistent with itself and with the inputs it states does not show that the census is the \
model's, that a stated frame is the entity's, that the modal basis is an eigenbasis of the \
model, that a signed contribution was recovered correctly, or that the product produced the document.",
];

/// The arithmetic of the two mass fractions of the missing-mass disclosure, stated in the
/// document so that a reader can repeat it bit for bit.
pub const MASS_FRACTION_ARITHMETIC: &str = "E is the exact sum of the stated binary64 effective masses of the periodic \
modes. S is E rounded once to the nearest binary64 (ties to even). U is the exact difference T − E, rounded once the same \
way. U is not T minus the published S, and the two can differ: where E rounds to T, T minus the published S is zero while \
the mass the periodic modes leave out is not. The cumulative fraction is the binary64 quotient of the published S by T and \
the unrepresented fraction the binary64 quotient of the published U by T, each rounded to nearest; they state what fraction \
of the published total each published mass is, and are recomputable from the published values. T is the stated total mass \
of the direction, which includes the mass that moves with the supports. Where T is zero both fractions are undefined.";

const PROFILE_LIMIT: f64 = 9_007_199_254_740_992.0;
/// 2^53 − 1, the largest integer the checked canonical profile admits.
const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

// ---------------------------------------------------------------------------------------------
// Inputs.

/// How a document came to be: a field a reader checks.
#[derive(Debug, Clone, PartialEq)]
pub enum Origin {
    /// Produced by the product's spectrum solve (not built).
    SpectrumSolve,
    /// Constructed from a frozen reference's contributions, not produced by a spectrum solve. It
    /// shows reader agreement only.
    ConstructedFromReference {
        /// The reference package's repository path.
        package: String,
        /// The reference case.
        case: String,
        /// The reference analysis (request) within the case.
        analysis: String,
        /// Each thing constructed rather than taken from the reference, stated plainly.
        constructed: Vec<String>,
    },
}

/// How the document names the one modal result it rests on: as that result identifies itself
/// (its run id and binding digests, each named as the modal document names it).
#[derive(Debug, Clone, PartialEq)]
pub enum ModalResultBinding {
    Bound {
        run_id: String,
        request_sha256: String,
        model_sha256: Option<String>,
        materials_sha256: String,
        integrity_parameters_sha256: String,
        /// Where the modal basis names a state case: its id, index and `case_sha256`.
        state_case: Option<(String, usize, String)>,
    },
    /// No modal result exists (a constructed document), with the reason.
    NotApplicable { reason: String },
}

/// A standing as the modal document states it: a token and its reasons.
#[derive(Debug, Clone, PartialEq)]
pub struct Standing {
    pub token: String,
    pub reasons: Vec<String>,
}

/// The modal result's three standings, carried unchanged.
#[derive(Debug, Clone, PartialEq)]
pub struct ModalStandings {
    /// `/admission/status` of the modal document.
    pub result_status: String,
    /// `/admission/public_engineering_result`.
    pub public_engineering_result: bool,
    /// `/admission/parameter_standing`.
    pub parameter_standing: String,
    /// `/admission/parameter_label`.
    pub parameter_label: String,
    /// The solved document's top-level `/standing`.
    pub integrity: Standing,
}

/// The modal endpoint and the completion evidence the extraction decision used.
#[derive(Debug, Clone, PartialEq)]
pub struct Extraction {
    /// The modal endpoint's kind (`below_computed_cluster`, `below_lowest_mode`,
    /// `whole_finite_spectrum`).
    pub endpoint: String,
    /// The reported (extracted) mode count.
    pub reported_mode_count: usize,
    /// The established finite mode count, where the modal result states one.
    pub established_finite_mode_count: Option<usize>,
    /// The frequency of the next computed mode above the extracted set, where one is stated.
    pub next_computed_frequency_hz: Option<f64>,
    /// That mode's circular frequency in rad/s, as the modal result publishes it; `None` exactly
    /// where the frequency is `None`.
    pub next_computed_circular_frequency_rad_s: Option<f64>,
    /// What the completion evidence is, in words.
    pub evidence: String,
}

/// The modal basis as the document uses it, attributed to its source. The modes' indices,
/// clusters and frequencies are the combination request's.
#[derive(Debug, Clone, PartialEq)]
pub struct ModalStatement {
    /// Whose statement this is: `modal_result`, or the frozen reference a constructed document
    /// takes it from.
    pub attributed_to: String,
    /// The modal result's three standings, or `None` where the source states none.
    pub standings: Option<ModalStandings>,
    /// Each mode's integrity standing, aligned with the combination request's modes.
    pub mode_standings: Vec<Option<Standing>>,
    /// What the modal result states of a state case and of each support's state, verbatim; `None`
    /// where it names no state case.
    pub state: Option<Value>,
    pub extraction: Extraction,
    /// T_k in kg, in X, Y, Z order.
    pub total_mass_kg: [f64; 3],
    /// Each mode's effective mass in kg, X, Y, Z, aligned with the combination request's modes.
    pub effective_mass_kg: Vec<[f64; 3]>,
    /// Each mode's circular frequency ω in rad/s as the modal result publishes it, aligned with
    /// the combination request's modes. The cluster rule is evaluated on these.
    pub circular_frequency_rad_s: Vec<f64>,
    /// The cluster tolerance δ_c the modal result binds (its integrity parameters'
    /// `cluster_tolerance`), in unit one.
    pub cluster_tolerance: f64,
}

/// Three axes, the rows x, y and z, each a vector in global axes.
pub type Triad = [[f64; 3]; 3];

#[derive(Debug, Clone, PartialEq)]
pub struct StraightMember {
    pub id: String,
    pub end_i: String,
    pub end_j: String,
    /// The member's reference vector, from which y is formed.
    pub reference_vector: [f64; 3],
    pub axes: Triad,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArcMember {
    pub id: String,
    pub end_i: String,
    pub end_j: String,
    pub tangent_frame_i: Triad,
    pub tangent_frame_j: Triad,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Connector {
    pub id: String,
    pub end_i: String,
    pub end_j: String,
    pub axes: Triad,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TieRod {
    pub connector: String,
    pub rod: String,
    /// The rod axis, a unit vector from the attachment at end i to the attachment at end j.
    pub axis: [f64; 3],
}

#[derive(Debug, Clone, PartialEq)]
pub struct RigidConstraint {
    pub node: String,
    pub component: ReactionComponent,
    pub holders: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompliantSupport {
    pub support: String,
    pub node: String,
    pub components: Vec<ReactionComponent>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SupportWithoutRow {
    pub support: String,
    pub reason: String,
}

/// What the roster was derived from (REQ-006, "Census").
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Census {
    pub nodes: Vec<String>,
    pub straight_members: Vec<StraightMember>,
    pub arcs: Vec<ArcMember>,
    pub connectors: Vec<Connector>,
    pub tie_rods: Vec<TieRod>,
    pub rigid_constraints: Vec<RigidConstraint>,
    pub compliant_supports: Vec<CompliantSupport>,
    pub supports_without_rows: Vec<SupportWithoutRow>,
}

impl Census {
    /// The roster this census gives (REQ-006, "Roster"), in census order: every node's six
    /// components; each straight member's ends in global and member axes; each arc's ends in global
    /// axes and its tangent frame; each connector's six generalized forces and its end actions;
    /// each tie rod's force; each rigid constraint's reaction; each compliant support's components.
    pub fn roster(&self) -> Vec<Quantity> {
        let mut out = Vec::new();
        const D: [DisplacementComponent; 6] = [DisplacementComponent::Ux, DisplacementComponent::Uy, DisplacementComponent::Uz,
            DisplacementComponent::Rx, DisplacementComponent::Ry, DisplacementComponent::Rz];
        const F: [ForceComponent; 6] =
            [ForceComponent::Fx, ForceComponent::Fy, ForceComponent::Fz, ForceComponent::Mx, ForceComponent::My, ForceComponent::Mz];
        const G: [ConnectorComponent; 6] = [ConnectorComponent::Axial, ConnectorComponent::LateralY, ConnectorComponent::LateralZ,
            ConnectorComponent::Torsion, ConnectorComponent::BendingY, ConnectorComponent::BendingZ];
        for node in &self.nodes {
            out.extend(D.iter().map(|&component| Quantity::NodeDisplacement { node: node.clone(), component }));
        }
        let ends = |out: &mut Vec<Quantity>, member: &str, family: MemberFamily, frame: Frame| {
            for end in [End::I, End::J] {
                out.extend(F.iter().map(|&component| Quantity::ElementEndAction { member: member.to_string(), family, end, component, frame }));
            }
        };
        for m in &self.straight_members {
            ends(&mut out, &m.id, MemberFamily::Straight, Frame::GlobalAxes);
            ends(&mut out, &m.id, MemberFamily::Straight, Frame::MemberAxes);
        }
        for a in &self.arcs {
            ends(&mut out, &a.id, MemberFamily::Arc, Frame::GlobalAxes);
            ends(&mut out, &a.id, MemberFamily::Arc, Frame::ArcTangentFrame);
        }
        for c in &self.connectors {
            out.extend(G.iter().map(|&component| Quantity::ConnectorGeneralizedForce { connector: c.id.clone(), component }));
            for end in [End::I, End::J] {
                out.extend(F.iter().map(|&component| Quantity::ConnectorEndAction { connector: c.id.clone(), end, component }));
            }
        }
        for r in &self.tie_rods {
            out.push(Quantity::TieRodForce { connector: r.connector.clone(), rod: r.rod.clone() });
        }
        for c in &self.rigid_constraints {
            out.push(Quantity::ConstraintReaction { node: c.node.clone(), component: c.component, holders: c.holders.clone() });
        }
        for s in &self.compliant_supports {
            out.extend(s.components.iter().map(|&component| Quantity::CompliantSupportForce { support: s.support.clone(), component }));
        }
        out
    }
}

/// How one requested direction's signed contributions were formed, recorded where they are
/// formed.
#[derive(Debug, Clone, PartialEq)]
pub struct FormationRecord {
    pub direction: Direction,
    /// The scale factor the spectrum's ordinates were scaled by, as represented.
    pub scale_factor: f64,
    /// The identity of the spectrum whose table was evaluated.
    pub spectrum_id: String,
    /// The SHA-256 (lowercase hex) of the canonical text of that spectrum's record in the model
    /// document that supplied the table.
    pub spectrum_record_sha256: String,
}

/// The signed contributions as they enter the assembler: the rows, modal metadata and signed
/// contributions, and for each requested direction its formation record.
#[derive(Debug, Clone, PartialEq)]
pub struct SignedContributions {
    pub combination: CombinationRequest,
    pub formation: Vec<FormationRecord>,
}

/// Everything a document is assembled from.
#[derive(Debug, Clone, Copy)]
pub struct DocumentInputs<'a> {
    pub origin: &'a Origin,
    /// The model document, which holds the spectra (`spectra[]`).
    pub model: &'a Value,
    /// The spectrum request.
    pub request: &'a Value,
    pub modal_result: &'a ModalResultBinding,
    pub modal: &'a ModalStatement,
    pub census: &'a Census,
    /// The signed contributions and how they were formed.
    pub contributions: &'a SignedContributions,
}

// ---------------------------------------------------------------------------------------------
// The semantic table.

/// How a quantity kind's frame is bound to the static result's rows (REQ-006, "Bound to the
/// static result's rows"): (frame, member family where it matters, the static row of the same
/// physical quantity in REQ-006's words, or `None` where the static result has none).
type StaticBinding = (Frame, Option<MemberFamily>, Option<&'static str>);

fn quantity_kind_rows() -> Vec<(&'static str, &'static str, bool, Vec<(&'static str, UnitId)>, SignConvention, Vec<StaticBinding>)> {
    let force = |names: [&'static str; 6]| -> Vec<(&'static str, UnitId)> {
        names.iter().enumerate().map(|(i, n)| (*n, if i < 3 { UnitId::Newton } else { UnitId::NewtonMeter })).collect()
    };
    vec![
        ("node_displacement", "node", false,
            vec![("ux", UnitId::Meter), ("uy", UnitId::Meter), ("uz", UnitId::Meter), ("rx", UnitId::Radian), ("ry", UnitId::Radian), ("rz", UnitId::Radian)],
            SignConvention::PositiveAlongAxis,
            vec![(Frame::GlobalAxes, None, Some("the static nodal displacement or rotation of that node and component, in global axes"))]),
        ("element_end_action", "member", true, force(["Fx", "Fy", "Fz", "Mx", "My", "Mz"]), SignConvention::ActionOnElementAtEndNode,
            vec![
                (Frame::GlobalAxes, Some(MemberFamily::Straight), Some("the static global end-action row of that member at that end and component")),
                (Frame::MemberAxes, Some(MemberFamily::Straight), Some("the static element-local end row of that member at that end and component, in the member's local frame; on an EX pressure member the axial row takes the frame and sign of the static wall axial end action")),
                (Frame::GlobalAxes, Some(MemberFamily::Arc), Some("the static global end-action row of that arc at that end and component")),
                (Frame::ArcTangentFrame, Some(MemberFamily::Arc), Some("the static element-local end row of that arc at that end and component, in the arc tangent frame at that end")),
            ]),
        ("connector_generalized_force", "connector", false,
            vec![("axial", UnitId::Newton), ("lateral_y", UnitId::Newton), ("lateral_z", UnitId::Newton), ("torsion", UnitId::NewtonMeter),
                ("bending_y", UnitId::NewtonMeter), ("bending_z", UnitId::NewtonMeter)],
            SignConvention::PositiveAlongConnectorAxes,
            vec![(Frame::ConnectorAxes, None, Some("the static connector-local force of that connector and component, without its reference displacement"))]),
        ("connector_end_action", "connector", true, force(["Fx", "Fy", "Fz", "Mx", "My", "Mz"]), SignConvention::ActionOnElementAtEndNode,
            vec![(Frame::GlobalAxes, None, Some("the static connector end action of that connector, end and component"))]),
        ("tie_rod_force", "tie_rod", false, vec![("axial", UnitId::Newton)], SignConvention::TensionPositive,
            vec![(Frame::RodAxis, None, Some("the static tie-rod force of that connector's rod, without its reference elongation or pretension"))]),
        ("constraint_reaction", "rigid_constraint", false, force(["UX", "UY", "UZ", "RX", "RY", "RZ"]), SignConvention::ActionOfSupportOnPiping,
            vec![(Frame::GlobalAxes, None, Some("where one support holds the constraint, that support's static reaction of the same component where the static result publishes one; a constraint held by more than one support, or by a snubber alone, has no static row"))]),
        ("compliant_support_force", "compliant_support", false, force(["UX", "UY", "UZ", "RX", "RY", "RZ"]), SignConvention::ActionOfSupportOnPiping,
            vec![(Frame::GlobalAxes, None, Some("that support's static action of the same component; a snubber has no static row"))]),
    ]
}

/// The scalar value kinds of the document's value objects and their catalog units.
const VALUE_KINDS: [(&str, UnitId); 22] = [
    ("damping_ratio", UnitId::One),
    ("scale_factor", UnitId::One),
    ("rigid_frequency", UnitId::Hertz),
    ("modal_frequency", UnitId::Hertz),
    ("modal_circular_frequency", UnitId::RadianPerSecond),
    ("next_computed_circular_frequency", UnitId::RadianPerSecond),
    ("cluster_tolerance", UnitId::One),
    ("cluster_standing_frequency", UnitId::Hertz),
    ("next_computed_frequency", UnitId::Hertz),
    ("evaluated_frequency", UnitId::Hertz),
    ("spectrum_range_frequency", UnitId::Hertz),
    ("spectrum_range_period", UnitId::Second),
    ("spectrum_g", UnitId::MeterPerSecondSquared),
    ("directional_total_mass", UnitId::Kilogram),
    ("modal_effective_mass", UnitId::Kilogram),
    ("periodic_effective_mass_sum", UnitId::Kilogram),
    ("unrepresented_effective_mass", UnitId::Kilogram),
    ("periodic_cumulative_effective_mass_fraction", UnitId::One),
    ("unrepresented_mass_fraction", UnitId::One),
    ("correlation_coefficient", UnitId::One),
    ("frame_axis_component", UnitId::One),
    ("spectrum_ordinate_g_multiple", UnitId::One),
];

fn unit_value(id: UnitId) -> Value {
    let d = id.definition();
    json!({"unit_status": "catalog", "unit": id.catalog_id(), "symbol": d.symbol, "dimension": d.dimension.as_str()})
}

fn unit_of(kind: &str) -> UnitId {
    VALUE_KINDS.iter().find(|(k, _)| *k == kind).map(|(_, u)| *u).unwrap_or_else(|| unreachable!("spectrum-1 names every value kind"))
}

/// The `spectrum-1` semantic table as a value; its canonical text is the committed
/// `fixtures/results/semantic_contract_spectrum_1.json`, whose SHA-256 the document names.
pub fn spectrum_semantic_table() -> Value {
    let frames: Vec<Value> = Frame::ALL.iter().map(|f| json!({"frame": f.as_str(), "axes": frame_axes_statement(*f)})).collect();
    let conventions: Vec<Value> =
        SignConvention::ALL.iter().map(|c| json!({"sign_convention": c.as_str(), "statement": c.statement()})).collect();
    let statuses: Vec<Value> = [SignStatus::SignedContribution, SignStatus::SignedClusterSum, SignStatus::UnsignedCombinedMagnitude]
        .iter()
        .map(|s| json!({"sign_status": s.as_str(), "statement": s.statement(), "carries_sign_convention": *s != SignStatus::UnsignedCombinedMagnitude}))
        .collect();
    let kinds: Vec<Value> = quantity_kind_rows()
        .into_iter()
        .map(|(kind, entity, has_end, components, convention, bindings)| {
            json!({
                "quantity_kind": kind,
                "entity_kind": entity,
                "has_end": has_end,
                "components": components.iter().map(|(c, u)| { let mut v = unit_value(*u); v["component"] = json!(c); v }).collect::<Vec<_>>(),
                "sign_convention": convention.as_str(),
                "frames": bindings.iter().map(|(frame, family, static_row)| json!({
                    "frame": frame.as_str(),
                    "member_family": family.map(MemberFamily::as_str),
                    "static_row": static_row,
                })).collect::<Vec<_>>(),
            })
        })
        .collect();
    let values: Vec<Value> = VALUE_KINDS.iter().map(|(k, u)| { let mut v = unit_value(*u); v["kind"] = json!(k); v }).collect();
    json!({
        "id": SPECTRUM_RESULT_SEMANTICS,
        "document_kind": SPECTRUM_RESULT_KIND,
        "schema_version": SPECTRUM_RESULT_VERSION,
        "canonical_profile": SPECTRUM_RESULT_CANONICAL,
        "frames": frames,
        "sign_conventions": conventions,
        "sign_statuses": statuses,
        "quantity_kinds": kinds,
        "value_kinds": values,
        "held": HELD.iter().map(|(held, statement)| json!({"held": held, "statement": statement})).collect::<Vec<_>>(),
        "document_shape": document_shape(),
    })
}

// ---------------------------------------------------------------------------------------------
// The document's shape: every member of every object, with its type and when it may be null.

fn t(kind: &str) -> Value {
    json!({"type": kind})
}

fn tokens<S: AsRef<str>>(list: impl IntoIterator<Item = S>) -> Value {
    json!({"type": "token", "tokens": list.into_iter().map(|s| s.as_ref().to_string()).collect::<Vec<_>>()})
}

fn vo(kinds: &[&str]) -> Value {
    json!({"type": "value_object", "kinds": kinds})
}

fn row_ref(table: &str) -> Value {
    json!({"type": "row_ref", "target": table})
}

fn array(items: Value) -> Value {
    json!({"type": "array", "items": items, "length": null})
}

fn array_n(items: Value, n: usize) -> Value {
    json!({"type": "array", "items": items, "length": n})
}

fn object(shape: &str) -> Value {
    json!({"type": "object", "shape": shape})
}

fn sealed(row: &str) -> Value {
    json!({"type": "table", "row": row})
}

fn direction_map(values: Value) -> Value {
    json!({"type": "direction_map", "values": values})
}

fn mem(name: &str, ty: Value, rule: &str) -> Value {
    json!({"member": name, "type": ty, "null": rule})
}

fn shape(name: &str, members: Vec<Value>) -> Value {
    json!({"shape": name, "members": members, "variants": null})
}

fn shape_by(name: &str, members: Vec<Value>, by: &str, alternatives: Vec<(String, Vec<Value>)>) -> Value {
    let alternatives: Vec<Value> = alternatives.into_iter().map(|(when, members)| json!({"when": when, "members": members})).collect();
    json!({"shape": name, "members": members, "variants": {"by": by, "alternatives": alternatives}})
}

const DOCUMENT_TYPES: [(&str, &str); 13] = [
    ("string", "A JSON string."),
    ("token", "A JSON string that is one of the stated tokens."),
    ("boolean", "true or false."),
    ("integer", "A JSON number written as an integer, from 0 to 2^53 - 1."),
    ("number", "A JSON number within the checked canonical profile (finite, of magnitude below 2^53)."),
    ("value_object", "An object of shape value_object whose kind is one of the stated kinds and whose unit is that kind's catalog unit in value_kinds."),
    ("row_ref", "A string that is the row_id of a row of the sealed table at the JSON pointer its target states, in the same document."),
    ("array", "A JSON array whose every item is of the stated type, of the stated length where one is stated."),
    ("object", "An object of the named shape."),
    ("table", "A sealed table: an object with exactly the members status (the token published), row_count (an integer, the number of rows), digest (the SHA-256, lowercase hex, of the canonical text of the array of the rows' checksum strings in row order) and rows (an array of objects of the named row shape). Each row has, beside its shape's members, row_id (a string, unique in the table), ordinal (an integer, its zero-based position in rows) and checksum (the SHA-256, lowercase hex, of the canonical text of the row without its checksum member)."),
    ("direction_map", "An object whose members are exactly the directions of /rows/directions, each of the stated type."),
    ("opaque_json", "Any JSON value within the checked canonical profile; its content is attributed, not checked here."),
    ("null", "A member whose null rule permits it may be null in place of its type."),
];

const NULL_RULES: [(&str, &str); 16] = [
    ("never", "The member is never null."),
    ("always", "The member is always null."),
    ("permitted", "The member may be null."),
    ("refused_only", "The member may be null only in a document whose status is refused."),
    ("correction_not_applied", "The member is null exactly when /request/missing_mass/applied is false."),
    ("modal_rule_not_cqc", "The member is null exactly when /request/modal_combination is not CQC."),
    ("modal_rule_not_grouping", "The member is null exactly when /request/modal_combination is not GROUPING."),
    ("modal_rule_not_ten_percent", "The member is null exactly when /request/modal_combination is not TEN_PERCENT."),
    ("modal_rule_not_grouping_or_ten_percent", "The member is null exactly when /request/modal_combination is neither GROUPING nor TEN_PERCENT."),
    ("directional_rule_not_100_40_40", "The member is null exactly when /request/directional_combination is not 100-40-40."),
    ("binding_not_applicable", "The member is null exactly when /modal_result/binding is not_applicable."),
    ("binding_bound", "The member is null exactly when /modal_result/binding is bound."),
    ("origin_spectrum_solve", "The member is null exactly when /origin/kind is spectrum_solve."),
    ("endpoint_whole_finite_spectrum", "The member is null exactly when /modal_basis/extraction/endpoint is whole_finite_spectrum."),
    ("no_cluster", "The member is null exactly when /modal_basis/clusters has no rows."),
    ("undefined_reason_stated", "In a value object: value is null exactly when undefined_reason is not null; undefined_reason is null exactly when value is not null."),
];

/// The rows' identity members of one quantity kind, and its units, frames and sign convention.
fn kind_members(kind: &str, entity: &str, has_end: bool, components: &[(&str, UnitId)], bindings: &[StaticBinding],
    convention: SignConvention, signed: bool) -> Vec<Value> {
    let mut out = vec![mem("entity_kind", tokens([entity]), "never")];
    match kind {
        "node_displacement" => out.push(mem("node", t("string"), "never")),
        "element_end_action" => {
            out.push(mem("member", t("string"), "never"));
            out.push(mem("member_family", tokens(["straight", "arc"]), "never"));
        }
        "connector_generalized_force" | "connector_end_action" => out.push(mem("connector", t("string"), "never")),
        "tie_rod_force" => {
            out.push(mem("connector", t("string"), "never"));
            out.push(mem("rod", t("string"), "never"));
        }
        "constraint_reaction" => {
            out.push(mem("node", t("string"), "never"));
            out.push(mem("holders", array(t("string")), "never"));
            out.push(mem("held", tokens(HELD.iter().map(|(h, _)| *h)), "never"));
        }
        _ => out.push(mem("support", t("string"), "never")),
    }
    out.push(if has_end { mem("end", tokens(["i", "j"]), "never") } else { mem("end", tokens(["i", "j"]), "always") });
    out.push(mem("component", tokens(components.iter().map(|(c, _)| *c)), "never"));
    let unique = |items: Vec<&'static str>| items.iter().enumerate().filter(|(i, x)| !items[..*i].contains(x)).map(|(_, x)| *x).collect::<Vec<_>>();
    out.push(mem("unit", tokens(unique(components.iter().map(|(_, u)| u.catalog_id()).collect())), "never"));
    out.push(mem("frame", tokens(unique(bindings.iter().map(|(f, _, _)| f.as_str()).collect())), "never"));
    if signed {
        out.push(mem("sign_convention", tokens([convention.as_str()]), "never"));
    }
    out
}

/// The `document_shape` section of `spectrum-1`: every object of the solved and the refused
/// document, each member's type and null rule, and the alternatives that depend on a member's
/// token.
#[allow(clippy::too_many_lines)]
fn document_shape() -> Value {
    let directions = || tokens(Direction::ALL.iter().map(|d| d.as_str()));
    let clusters = || row_ref("/modal_basis/clusters");
    let modes = || row_ref("/modal_basis/modes");
    let class = || tokens([ClusterClass::Periodic.as_str(), ClusterClass::Rigid.as_str()]);
    let triple = || array_n(t("number"), 3);
    let never = |name: &str, ty: Value| mem(name, ty, "never");
    let kinds = quantity_kind_rows();
    let all_units: Vec<&str> = {
        let mut u: Vec<&str> = kinds.iter().flat_map(|k| k.3.iter().map(|(_, u)| u.catalog_id())).collect();
        u.sort_unstable();
        u.dedup();
        u
    };
    let signed_alternatives = kinds.iter().map(|(kind, entity, has_end, components, convention, bindings)|
        ((*kind).to_string(), kind_members(kind, entity, *has_end, components, bindings, *convention, true))).collect();
    let combined_alternatives = kinds.iter().map(|(kind, entity, has_end, components, convention, bindings)|
        ((*kind).to_string(), kind_members(kind, entity, *has_end, components, bindings, *convention, false))).collect();
    let kind_tokens = || tokens(QUANTITY_KINDS);
    let reaction_components = || tokens(["UX", "UY", "UZ", "RX", "RY", "RZ"]);
    let shapes = vec![
        shape_by("document", vec![
            never("document_kind", tokens([SPECTRUM_RESULT_KIND])),
            never("schema_version", tokens([SPECTRUM_RESULT_VERSION])),
            never("semantic_contract", object("semantic_contract")),
            never("profile", object("profile")),
            never("canonical_profile", tokens([SPECTRUM_RESULT_CANONICAL])),
            never("admission", object("admission")),
            never("origin", object("origin")),
            never("bindings", object("bindings")),
            never("modal_result", object("modal_result")),
            never("status", tokens(["solved", "refused"])),
        ], "status", vec![
            ("solved".into(), vec![
                never("standing", object("standing")),
                never("request", object("request")),
                never("spectra", sealed("spectrum")),
                never("modal_basis", object("modal_basis")),
                never("disclosures", object("disclosures")),
                never("census", object("census")),
                never("rows", object("rows")),
                never("rule_statements", object("rule_statements")),
            ]),
            ("refused".into(), vec![never("refusals", sealed("refusal"))]),
        ]),
        shape("semantic_contract", vec![never("id", tokens([SPECTRUM_RESULT_SEMANTICS])), never("sha256", t("string"))]),
        shape("profile", vec![never("profile_id", tokens([SPECTRUM_RESULT_PROFILE])),
            never("limitations", array_n(tokens(SPECTRUM_RESULT_LIMITATIONS), SPECTRUM_RESULT_LIMITATIONS.len()))]),
        shape("admission", vec![never("status", tokens(["experimental"])), never("public_engineering_result", t("boolean"))]),
        shape_by("origin", vec![
            never("kind", tokens(["spectrum_solve", "constructed_from_reference"])),
            never("produced_by_spectrum_solve", t("boolean")),
            mem("reference", object("origin_reference"), "origin_spectrum_solve"),
            never("constructed", array(t("string"))),
        ], "kind", vec![
            ("spectrum_solve".into(), vec![never("statement", tokens([ORIGIN_STATEMENTS[0]]))]),
            ("constructed_from_reference".into(), vec![never("statement", tokens([ORIGIN_STATEMENTS[1]]))]),
        ]),
        shape("origin_reference", vec![never("package", t("string")), never("case", t("string")), never("analysis", t("string"))]),
        shape("bindings", vec![never("model", object("binding")), never("request", object("binding"))]),
        shape("binding", vec![
            never("binds", tokens(["model_document", "spectrum_request"])),
            never("hash_algorithm", tokens([SPECTRUM_HASH_ALGORITHM])),
            never("canonical_form", tokens([SPECTRUM_RESULT_CANONICAL])),
            mem("sha256", t("string"), "refused_only"),
        ]),
        shape("modal_result", vec![
            never("binding", tokens(["bound", "not_applicable"])),
            mem("reason", t("string"), "binding_bound"),
            mem("run_id", t("string"), "binding_not_applicable"),
            mem("request_sha256", t("string"), "binding_not_applicable"),
            mem("model_sha256", t("string"), "permitted"),
            mem("materials_sha256", t("string"), "binding_not_applicable"),
            mem("integrity_parameters_sha256", t("string"), "binding_not_applicable"),
            mem("state_case", object("state_case"), "permitted"),
        ]),
        shape("state_case", vec![never("case_id", t("string")), never("case_index", t("integer")), never("case_sha256", t("string"))]),
        shape("standing", vec![never("token", t("string")), never("reasons", array(t("string")))]),
        shape("request", vec![
            never("modal_combination", tokens(ModalRule::ALL.iter().map(|r| r.as_str()))),
            never("directional_combination", tokens(DirectionalRule::ALL.iter().map(|r| r.as_str()))),
            never("damping_ratio", vo(&["damping_ratio"])),
            never("missing_mass", object("request_missing_mass")),
            never("directions", array(object("request_direction"))),
        ]),
        shape("request_missing_mass", vec![
            never("applied", t("boolean")),
            mem("rigid_frequency", vo(&["rigid_frequency"]), "correction_not_applied"),
            mem("rigid_combination", tokens(RigidRule::ALL.iter().map(|r| r.as_str())), "correction_not_applied"),
        ]),
        shape("request_direction", vec![never("direction", directions()), never("spectrum_id", t("string")), never("scale_factor", vo(&["scale_factor"]))]),
        shape_by("spectrum", vec![
            never("spectrum_id", t("string")),
            never("damping_ratio", vo(&["damping_ratio"])),
            never("abscissa", tokens(["frequency", "period"])),
            never("abscissa_unit", tokens([UnitId::Hertz.catalog_id(), UnitId::Second.catalog_id()])),
            never("ordinate", tokens(["acceleration", "g_multiple"])),
            never("pseudo_spectral_acceleration", t("boolean")),
            never("interpolation", object("interpolation")),
            never("points", array(array_n(t("number"), 2))),
        ], "ordinate", vec![
            ("acceleration".into(), vec![never("ordinate_unit", tokens([UnitId::MeterPerSecondSquared.catalog_id()])), mem("g", vo(&["spectrum_g"]), "always")]),
            ("g_multiple".into(), vec![never("ordinate_unit", tokens([UnitId::One.catalog_id()])), never("g", vo(&["spectrum_g"]))]),
        ]),
        shape("interpolation", vec![never("abscissa", tokens(["linear", "log"])), never("ordinate", tokens(["linear", "log"]))]),
        shape("modal_basis", vec![
            never("attributed_to", t("string")),
            mem("standings", object("modal_standings"), "permitted"),
            mem("state", t("opaque_json"), "permitted"),
            never("extraction", object("extraction")),
            never("cluster_tolerance", vo(&["cluster_tolerance"])),
            never("modes", sealed("mode")),
            never("clusters", sealed("cluster")),
        ]),
        shape("modal_standings", vec![
            never("result_standing", object("result_standing")),
            never("parameter_standing", object("parameter_standing")),
            never("integrity_standing", object("standing")),
        ]),
        shape("result_standing", vec![never("status", t("string")), never("public_engineering_result", t("boolean"))]),
        shape("parameter_standing", vec![never("parameter_standing", t("string")), never("parameter_label", t("string"))]),
        shape("extraction", vec![
            never("endpoint", tokens(ENDPOINTS)),
            never("reported_mode_count", t("integer")),
            mem("established_finite_mode_count", t("integer"), "permitted"),
            mem("next_computed_frequency", vo(&["next_computed_frequency"]), "endpoint_whole_finite_spectrum"),
            mem("next_computed_circular_frequency", vo(&["next_computed_circular_frequency"]), "endpoint_whole_finite_spectrum"),
            never("completion_evidence", t("string")),
        ]),
        shape("mode", vec![
            never("mode_index", t("integer")),
            never("cluster_ref", clusters()),
            never("frequency", vo(&["modal_frequency"])),
            never("circular_frequency", vo(&["modal_circular_frequency"])),
            never("class", class()),
            mem("integrity_standing", object("standing"), "permitted"),
        ]),
        shape("cluster", vec![
            never("cluster_index", t("integer")),
            never("mode_refs", array(modes())),
            never("frequencies", array(t("number"))),
            never("frequency_unit", tokens([UnitId::Hertz.catalog_id()])),
            never("standing_frequency", vo(&["cluster_standing_frequency"])),
            never("class", class()),
        ]),
        shape("disclosures", vec![
            never("range", sealed("range")),
            never("extraction_premise", object("extraction_premise")),
            never("missing_mass", object("missing_mass")),
            never("fixed", sealed("fixed_disclosure")),
        ]),
        shape("range", vec![
            never("spectrum_id", t("string")),
            never("directions", array(directions())),
            never("abscissa", tokens(["frequency", "period"])),
            never("tabulated_low", vo(&["spectrum_range_frequency", "spectrum_range_period"])),
            never("tabulated_high", vo(&["spectrum_range_frequency", "spectrum_range_period"])),
            never("evaluated", array(object("evaluation"))),
            never("in_range", t("boolean")),
        ]),
        shape("evaluation", vec![never("at", t("string")), never("frequency", vo(&["evaluated_frequency"])), never("location", object("location"))]),
        shape_by("location", vec![never("kind", tokens(["knot", "segment"]))], "kind", vec![
            ("knot".into(), vec![never("index", t("integer"))]),
            ("segment".into(), vec![never("lower", t("integer"))]),
        ]),
        shape("extraction_premise", vec![
            never("applies", t("boolean")),
            mem("rigid_frequency", vo(&["rigid_frequency"]), "correction_not_applied"),
            never("endpoint", tokens(ENDPOINTS)),
            never("reported_mode_count", t("integer")),
            mem("established_finite_mode_count", t("integer"), "permitted"),
            mem("highest_completed_cluster_ref", clusters(), "no_cluster"),
            mem("highest_completed_cluster_lowest_frequency", vo(&["cluster_standing_frequency"]), "no_cluster"),
            mem("next_computed_frequency", vo(&["next_computed_frequency"]), "endpoint_whole_finite_spectrum"),
            never("completion_evidence", t("string")),
            mem("holds", t("boolean"), "correction_not_applied"),
        ]),
        shape("missing_mass", vec![never("arithmetic", tokens([MASS_FRACTION_ARITHMETIC])), never("directions", sealed("missing_mass_direction"))]),
        shape("missing_mass_direction", vec![
            never("direction", directions()),
            never("total_mass", vo(&["directional_total_mass"])),
            never("periodic_effective_masses", array(object("periodic_effective_mass"))),
            never("periodic_effective_mass_sum", vo(&["periodic_effective_mass_sum"])),
            never("unrepresented_effective_mass", vo(&["unrepresented_effective_mass"])),
            never("periodic_cumulative_fraction", vo(&["periodic_cumulative_effective_mass_fraction"])),
            never("unrepresented_fraction", vo(&["unrepresented_mass_fraction"])),
            never("correction_applied", t("boolean")),
        ]),
        shape("periodic_effective_mass", vec![never("mode_ref", modes()), never("effective_mass", vo(&["modal_effective_mass"]))]),
        shape_by("fixed_disclosure", vec![never("disclosure_id", tokens(FIXED_DISCLOSURES.iter().map(|(id, _)| *id)))], "disclosure_id",
            FIXED_DISCLOSURES.iter().map(|(id, text)| ((*id).to_string(), vec![never("text", tokens([*text]))])).collect()),
        shape("census", vec![
            never("nodes", sealed("census_node")),
            never("straight_members", sealed("census_straight_member")),
            never("arcs", sealed("census_arc")),
            never("connectors", sealed("census_connector")),
            never("tie_rods", sealed("census_tie_rod")),
            never("rigid_constraints", sealed("census_rigid_constraint")),
            never("compliant_supports", sealed("census_compliant_support")),
            never("supports_without_rows", sealed("census_support_without_row")),
        ]),
        shape("census_node", vec![never("node_id", t("string"))]),
        shape("census_straight_member", vec![never("member_id", t("string")), never("end_i_node", t("string")), never("end_j_node", t("string")),
            never("reference_vector", triple()), never("member_axes", object("triad"))]),
        shape("census_arc", vec![never("member_id", t("string")), never("end_i_node", t("string")), never("end_j_node", t("string")),
            never("tangent_frame_i", object("triad")), never("tangent_frame_j", object("triad"))]),
        shape("census_connector", vec![never("connector_id", t("string")), never("end_i_node", t("string")), never("end_j_node", t("string")),
            never("connector_axes", object("triad"))]),
        shape("census_tie_rod", vec![never("connector_id", t("string")), never("rod_id", t("string")), never("rod_axis", triple())]),
        shape("census_rigid_constraint", vec![never("node_id", t("string")), never("component", reaction_components()),
            never("holders", array(t("string"))), never("held", tokens(HELD.iter().map(|(h, _)| *h)))]),
        shape("census_compliant_support", vec![never("support_id", t("string")), never("node_id", t("string")),
            never("components", array(reaction_components()))]),
        shape("census_support_without_row", vec![never("support_id", t("string")), never("reason", t("string"))]),
        shape("triad", vec![never("x", triple()), never("y", triple()), never("z", triple())]),
        shape("rows", vec![
            never("directions", array(directions())),
            never("contribution_columns", sealed("contribution_column")),
            never("cluster_sum_columns", sealed("cluster_sum_column")),
            never("signed", sealed("signed_row")),
            never("cluster_sums", sealed("cluster_sum_row")),
            never("combined", sealed("combined_row")),
        ]),
        shape("contribution_column", vec![never("column", t("integer")), never("mode_ref", modes()), never("cluster_ref", clusters())]),
        shape("cluster_sum_column", vec![never("column", t("integer")), never("cluster_ref", clusters()), never("mode_refs", array(modes()))]),
        shape_by("signed_row", vec![
            never("quantity_kind", kind_tokens()),
            never("sign_status", tokens([SignStatus::SignedContribution.as_str()])),
            never("periodic", direction_map(array(t("number")))),
            mem("rigid", direction_map(t("number")), "correction_not_applied"),
        ], "quantity_kind", signed_alternatives),
        shape("cluster_sum_row", vec![
            never("quantity_ref", row_ref("/rows/signed")),
            never("unit", tokens(all_units)),
            never("frame", tokens(Frame::ALL.iter().map(|f| f.as_str()))),
            never("sign_status", tokens([SignStatus::SignedClusterSum.as_str()])),
            never("sign_convention", tokens(SignConvention::ALL.iter().map(|c| c.as_str()))),
            never("sums", direction_map(array(t("number")))),
        ]),
        shape_by("combined_row", vec![
            never("quantity_kind", kind_tokens()),
            never("quantity_ref", row_ref("/rows/signed")),
            never("sign_status", tokens([SignStatus::UnsignedCombinedMagnitude.as_str()])),
            never("per_direction", array(object("per_direction"))),
            never("combined", t("number")),
            mem("governing_direction", directions(), "directional_rule_not_100_40_40"),
        ], "quantity_kind", combined_alternatives),
        shape("per_direction", vec![
            never("direction", directions()),
            never("periodic", t("number")),
            mem("rigid", t("number"), "correction_not_applied"),
            never("combined", t("number")),
        ]),
        shape("rule_statements", vec![
            mem("correlation", sealed("correlation"), "modal_rule_not_cqc"),
            mem("groups", sealed("group"), "modal_rule_not_grouping"),
            mem("coupled_pairs", sealed("coupled_pair"), "modal_rule_not_ten_percent"),
            mem("closeness_rule", tokens([GROUPING_CLOSENESS_RULE, TEN_PERCENT_CLOSENESS_RULE]), "modal_rule_not_grouping_or_ten_percent"),
        ]),
        shape("correlation", vec![never("first_cluster_ref", clusters()), never("second_cluster_ref", clusters()),
            never("rho", vo(&["correlation_coefficient"])), never("status", tokens(["informational"]))]),
        shape("group", vec![never("cluster_refs", array(clusters()))]),
        shape("coupled_pair", vec![never("first_cluster_ref", clusters()), never("second_cluster_ref", clusters())]),
        shape("refusal", vec![
            never("diagnostic_id", t("string")),
            never("code", tokens(RefusalCode::ALL.iter().map(|c| c.as_str()))),
            never("severity", tokens(["blocking"])),
            never("object", t("string")),
            mem("field", t("string"), "permitted"),
            never("detail", t("string")),
        ]),
        shape("value_object", vec![
            never("kind", t("string")),
            mem("value", t("number"), "undefined_reason_stated"),
            never("unit", t("string")),
            never("unit_status", tokens(["catalog"])),
            never("below_normal", t("boolean")),
            mem("undefined_reason", tokens(["not_formed", "total_mass_zero"]), "undefined_reason_stated"),
        ]),
    ];
    json!({
        "statement": "Every object of a spectrum result document has exactly the members its shape declares: the shape's \
members and, where the shape has variants, those of the alternative its `by` member's token selects. Each member is of its \
type, or null where its null rule permits. The document itself is of shape document.",
        "types": DOCUMENT_TYPES.iter().map(|(ty, statement)| json!({"type": ty, "statement": statement})).collect::<Vec<_>>(),
        "null_rules": NULL_RULES.iter().map(|(rule, statement)| json!({"rule": rule, "statement": statement})).collect::<Vec<_>>(),
        "shapes": shapes,
    })
}

fn frame_axes_statement(frame: Frame) -> &'static str {
    match frame {
        Frame::GlobalAxes => "the global X, Y and Z axes",
        Frame::MemberAxes => "a straight member's axes: x from end i to end j, y the part of the member's reference vector normal to x, normalized, z = x cross y; stated per member in the census as vectors in global axes",
        Frame::ArcTangentFrame => "an arc's tangent frame at the row's end, the static result's (DEL-04-01/REQ-018); stated per arc end in the census as vectors in global axes",
        Frame::ConnectorAxes => "a connector's own axes; stated per connector in the census as vectors in global axes",
        Frame::RodAxis => "a tie rod's axis from its attachment at end i to its attachment at end j; stated per rod in the census as one oriented unit vector in global axes",
    }
}

// ---------------------------------------------------------------------------------------------
// Hashing, sealing and values.

/// The checked parse's depth limit: a container nested 128 deep, counting a standalone text's
/// top value as the first, is refused (`open_pipe_stress_canonical_json`, `PARSE_DEPTH_LIMIT`).
const DEPTH_LIMIT: usize = 128;

/// Whether `value`, placed at nesting `level` (a standalone text's top value is at level 1), holds
/// a container at level 128 or deeper. It walks the value with an explicit stack, so a value of
/// any depth is examined without recursion.
fn too_deep(value: &Value, level: usize) -> bool {
    let mut stack = vec![(value, level)];
    while let Some((v, l)) = stack.pop() {
        match v {
            Value::Array(items) => {
                if l >= DEPTH_LIMIT {
                    return true;
                }
                stack.extend(items.iter().map(|item| (item, l + 1)));
            }
            Value::Object(map) => {
                if l >= DEPTH_LIMIT {
                    return true;
                }
                stack.extend(map.values().map(|item| (item, l + 1)));
            }
            _ => {}
        }
    }
    false
}

/// The SHA-256 (lowercase hex) of a value's checked canonical text; `None` outside the profile.
/// The value's depth is checked first, without recursion, so that serializing it cannot exhaust
/// the stack.
pub fn spectrum_doc_sha256(value: &Value) -> Option<String> {
    if too_deep(value, 1) {
        return None;
    }
    let text = serde_json::to_string(value).ok()?;
    let canonical = canonical_json_checked_v1_text(&text).ok()?;
    Some(Sha256::digest(canonical.as_bytes()).iter().map(|b| format!("{b:02x}")).collect())
}

fn seal(row_id: String, ordinal: usize, fields: Value) -> Value {
    let mut row = match fields {
        Value::Object(map) => map,
        _ => unreachable!("a row's fields are an object"),
    };
    row.insert("row_id".into(), json!(row_id));
    row.insert("ordinal".into(), json!(ordinal));
    let checksum = spectrum_doc_sha256(&Value::Object(row.clone()));
    row.insert("checksum".into(), json!(checksum));
    Value::Object(row)
}

fn table(rows: Vec<Value>) -> Value {
    let checksums: Vec<Value> = rows.iter().map(|row| row["checksum"].clone()).collect();
    json!({"status": "published", "row_count": rows.len(), "digest": spectrum_doc_sha256(&Value::Array(checksums)), "rows": rows})
}

/// Records published numbers outside the canonical profile, so that the document is refused
/// rather than carrying a value its hashes cannot cover.
struct Numbers {
    outside: Vec<Refusal>,
}

impl Numbers {
    fn num(&mut self, v: f64, at: &str) -> Value {
        if !(v.is_finite() && v.abs() < PROFILE_LIMIT) {
            if self.outside.len() < 16 {
                self.outside.push(Refusal::new(RefusalCode::DocumentOutsideCanonicalProfile, at, None,
                    format!("{v:e} is outside the checked canonical profile (not finite, or of magnitude 2^53 or more)")));
            }
            return Value::Null;
        }
        json!(v)
    }

    /// An integer the document carries outside a value object: refused at 2^53 or more.
    fn int(&mut self, v: usize, at: &str) -> Value {
        if v as u64 > MAX_SAFE_INTEGER {
            if self.outside.len() < 16 {
                self.outside.push(Refusal::new(RefusalCode::DocumentOutsideCanonicalProfile, at, None,
                    format!("the integer {v} is outside the checked canonical profile (2^53 or more)")));
            }
            return Value::Null;
        }
        json!(v)
    }

    fn arr(&mut self, values: impl IntoIterator<Item = f64>, at: &str) -> Value {
        Value::Array(values.into_iter().map(|v| self.num(v, at)).collect())
    }

    /// A value object: `kind`, `value`, `unit`, `unit_status`, `below_normal`, `undefined_reason`.
    fn quantity(&mut self, kind: &str, value: Option<f64>, undefined: Option<&str>, at: &str) -> Value {
        let (number, reason, below) = match (value, undefined) {
            (_, Some(reason)) => (Value::Null, json!(reason), false),
            (None, None) => (Value::Null, json!("not_formed"), false),
            (Some(v), None) => (self.num(v, at), Value::Null, v != 0.0 && v.abs() < f64::MIN_POSITIVE),
        };
        json!({"kind": kind, "value": number, "unit": unit_of(kind).catalog_id(), "unit_status": "catalog",
            "below_normal": below, "undefined_reason": reason})
    }

    fn fixed(&mut self, kind: &str, value: f64, at: &str) -> Value {
        self.quantity(kind, Some(value), None, at)
    }

    fn triad(&mut self, t: &Triad, at: &str) -> Value {
        json!({"x": self.arr(t[0], at), "y": self.arr(t[1], at), "z": self.arr(t[2], at)})
    }
}

fn standing_json(s: &Standing) -> Value {
    json!({"token": s.token, "reasons": s.reasons})
}

fn mode_ref(index: u32) -> String {
    format!("mode:{index}")
}

fn cluster_ref(index: ClusterIndex) -> String {
    format!("cluster:{}", index.0)
}

fn direction_index(d: Direction) -> usize {
    match d {
        Direction::X => 0,
        Direction::Y => 1,
        Direction::Z => 2,
    }
}

/// How a rigid constraint is held (REQ-006, "Beside the static result"): by one support, or by
/// more than one together, undivided.
fn held(holders: &[String]) -> &'static str {
    if holders.len() == 1 { HELD[0].0 } else { HELD[1].0 }
}

/// The `held` tokens and what each states, in REQ-006's words ("Beside the static result").
pub const HELD: [(&str, &str); 2] = [
    ("single", "Where one support holds the constraint, the row is that support's reaction."),
    ("jointly_undivided", "The row states that it is the reaction of the constraint, that the supports it names hold it together, \
and that it is not divided between them or to be added once per support."),
];

/// A row's identity fields (no unit, frame or sign fields). A constraint's holders are the
/// census's, in its order, where `constraints` holds the constraint.
fn identity_fields(q: &Quantity, constraints: &BTreeMap<(&str, ReactionComponent), &[String]>) -> Map<String, Value> {
    let mut m = Map::new();
    m.insert("quantity_kind".into(), json!(q.kind()));
    m.insert("entity_kind".into(), json!(q.entity_kind()));
    match q {
        Quantity::NodeDisplacement { node, .. } => {
            m.insert("node".into(), json!(node));
        }
        Quantity::ElementEndAction { member, family, .. } => {
            m.insert("member".into(), json!(member));
            m.insert("member_family".into(), json!(family.as_str()));
        }
        Quantity::ConnectorGeneralizedForce { connector, .. } | Quantity::ConnectorEndAction { connector, .. } => {
            m.insert("connector".into(), json!(connector));
        }
        Quantity::TieRodForce { connector, rod } => {
            m.insert("connector".into(), json!(connector));
            m.insert("rod".into(), json!(rod));
        }
        Quantity::ConstraintReaction { node, component, holders } => {
            let holders = constraints.get(&(node.as_str(), *component)).copied().unwrap_or(holders.as_slice());
            m.insert("node".into(), json!(node));
            m.insert("holders".into(), json!(holders));
            m.insert("held".into(), json!(held(holders)));
        }
        Quantity::CompliantSupportForce { support, .. } => {
            m.insert("support".into(), json!(support));
        }
    }
    m.insert("end".into(), json!(q.end().map(End::as_str)));
    m.insert("component".into(), json!(q.component_name()));
    m
}

// ---------------------------------------------------------------------------------------------
// The request.

/// The request's choices as the document carries them.
struct Choices {
    modal_rule: ModalRule,
    directional_rule: DirectionalRule,
    missing_mass: MissingMass,
    /// (direction, spectrum id, scale factor), in X, Y, Z order.
    directions: Vec<(Direction, String, f64)>,
}

fn input_invalid(object: &str, field: &str, detail: String) -> Refusal {
    Refusal::new(RefusalCode::DocumentInputInvalid, object, Some(field), detail)
}

fn read_choices(request: &Value, refusals: &mut Vec<Refusal>) -> Option<Choices> {
    let before = refusals.len();
    if request.get("document_kind").and_then(Value::as_str) != Some(SPECTRUM_REQUEST_KIND) {
        refusals.push(input_invalid("request", "document_kind", format!("the request is not an `{SPECTRUM_REQUEST_KIND}`")));
    }
    let choice = |name: &str, refusals: &mut Vec<Refusal>| -> Option<String> {
        match request.get(name).and_then(Value::as_str) {
            Some(s) => Some(s.to_string()),
            None => {
                refusals.push(Refusal::new(RefusalCode::MethodChoiceMissing, "request", Some(name), format!("no {name} is chosen; it has no default")));
                None
            }
        }
    };
    let modal = choice("modal_combination", refusals).and_then(|s| {
        let r = ModalRule::parse(&s);
        if r.is_none() {
            refusals.push(Refusal::new(RefusalCode::MethodChoiceMissing, "request", Some("modal_combination"), format!("`{s}` is not a modal combination")));
        }
        r
    });
    let directional = choice("directional_combination", refusals).and_then(|s| {
        let r = DirectionalRule::parse(&s);
        if r.is_none() {
            refusals.push(Refusal::new(RefusalCode::MethodChoiceMissing, "request", Some("directional_combination"), format!("`{s}` is not a directional combination")));
        }
        r
    });
    let missing_mass = match request.get("missing_mass") {
        Some(mm) if mm.get("applied") == Some(&json!(false)) => {
            if mm.get("rigid_frequency").is_some() || mm.get("rigid_combination").is_some() {
                refusals.push(Refusal::new(RefusalCode::MethodChoiceMissing, "request", Some("missing_mass"),
                    "a rigid frequency or rigid combination is given without the correction".into()));
            }
            Some(MissingMass::NotApplied)
        }
        Some(mm) if mm.get("applied") == Some(&json!(true)) => {
            let rule = mm.get("rigid_combination").and_then(Value::as_str).and_then(RigidRule::parse);
            if rule.is_none() {
                refusals.push(Refusal::new(RefusalCode::MethodChoiceMissing, "request", Some("rigid_combination"),
                    "the correction is applied with no rigid combination; it has no default".into()));
            }
            let fr = mm.get("rigid_frequency").and_then(|f| {
                let value = f.get("value").and_then(Value::as_f64)?;
                let unit = f.get("unit").and_then(Value::as_str)?;
                Some(QueryFrequency::new(value, unit).map(QueryFrequency::hertz))
            });
            let fr = match fr {
                None => {
                    refusals.push(Refusal::new(RefusalCode::RigidFrequencyInvalid, "request", Some("rigid_frequency"),
                        "the correction is applied with no rigid frequency; it has no default".into()));
                    None
                }
                Some(Err(r)) => {
                    refusals.push(Refusal::new(RefusalCode::RigidFrequencyInvalid, "request", Some("rigid_frequency"), r.detail));
                    None
                }
                Some(Ok(f)) => Some(f),
            };
            match (fr, rule) {
                (Some(rigid_frequency_hz), Some(rule)) => Some(MissingMass::Applied { rigid_frequency_hz, rule }),
                _ => None,
            }
        }
        _ => {
            refusals.push(Refusal::new(RefusalCode::MethodChoiceMissing, "request", Some("missing_mass"),
                "whether to apply the missing-mass correction is not chosen; it has no default".into()));
            None
        }
    };
    let mut directions = Vec::new();
    let mut requested = [false; 3];
    match request.get("directions").and_then(Value::as_array) {
        None => refusals.push(Refusal::new(RefusalCode::DirectionInvalid, "request", Some("directions"), "no direction is requested".into())),
        Some(list) => {
            if list.is_empty() {
                refusals.push(Refusal::new(RefusalCode::DirectionInvalid, "request", Some("directions"), "no direction is requested".into()));
            }
            for (i, d) in list.iter().enumerate() {
                let at = format!("directions[{i}]");
                let direction = d.get("direction").and_then(Value::as_str).and_then(Direction::parse);
                let spectrum = d.get("spectrum").and_then(Value::as_str).filter(|s| !s.trim().is_empty());
                let scale = d.get("scale_factor").and_then(Value::as_f64).map(ScaleFactor::new);
                match (direction, spectrum, scale) {
                    (Some(direction), Some(spectrum), Some(Ok(scale))) => {
                        if std::mem::replace(&mut requested[direction_index(direction)], true) {
                            refusals.push(Refusal::new(RefusalCode::DirectionInvalid, &at, Some("direction"), format!("direction {direction} is requested more than once")));
                        } else {
                            directions.push((direction, spectrum.to_string(), scale.value()));
                        }
                    }
                    (_, _, Some(Err(r))) => refusals.push(Refusal::new(RefusalCode::ScaleFactorInvalid, &at, Some("scale_factor"), r.detail)),
                    _ => refusals.push(Refusal::new(RefusalCode::DirectionInvalid, &at, None,
                        "a direction needs a global direction X, Y or Z, a spectrum and a positive finite scale factor".into())),
                }
            }
        }
    }
    directions.sort_by_key(|(d, _, _)| *d);
    if refusals.len() != before {
        return None;
    }
    Some(Choices { modal_rule: modal?, directional_rule: directional?, missing_mass: missing_mass?, directions })
}

/// The model's record of spectrum identity `id`, where it holds exactly one, and how many it
/// holds.
fn spectrum_records<'m>(model: &'m Value, id: &str) -> (Option<&'m Value>, usize) {
    let records = model.get("spectra").and_then(Value::as_array).map_or(&[][..], Vec::as_slice);
    let mut found = records.iter().filter(|s| s.get("id").and_then(Value::as_str) == Some(id));
    let first = found.next();
    let count = usize::from(first.is_some()) + found.count();
    (first.filter(|_| count == 1), count)
}

/// The spectra the request's directions name, admitted from the model's `spectra[]`, each once,
/// in order of first use. The model holds exactly one record of each identity a direction names.
fn read_spectra(model: &Value, choices: &Choices, refusals: &mut Vec<Refusal>) -> Vec<SpectrumTable> {
    let mut out: Vec<SpectrumTable> = Vec::new();
    let mut named: Vec<&str> = Vec::new();
    for (direction, id, _) in &choices.directions {
        if named.contains(&id.as_str()) {
            continue;
        }
        named.push(id);
        let (found, count) = spectrum_records(model, id);
        if count > 1 {
            refusals.push(input_invalid(direction.as_str(), "spectrum",
                format!("the model holds {count} spectra of identity `{id}`; a requested direction names one")));
            continue;
        }
        let Some(record) = found else {
            refusals.push(Refusal::new(RefusalCode::DirectionInvalid, direction.as_str(), Some("spectrum"),
                format!("the model holds no spectrum `{id}`")));
            continue;
        };
        match SpectrumRecord::deserialize(record) {
            Err(e) => refusals.push(Refusal::new(RefusalCode::RecordInvalid, id, None, format!("the record does not decode: {e}"))),
            Ok(record) => match SpectrumTable::admit(&record) {
                Ok(t) => out.push(t),
                Err(r) => refusals.extend(r),
            },
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------
// The assembler.

/// The origin's fixed statement: produced by a spectrum solve, or constructed from a reference.
pub const ORIGIN_STATEMENTS: [&str; 2] = [
    "Produced by the product's experimental spectrum solve.",
    "Constructed from the signed contributions of a frozen reference, not produced by a spectrum solve; \
its combined magnitudes are formed from those contributions by the product's combinations. It is not evidence that the \
product recovers these contributions.",
];

fn origin_json(origin: &Origin) -> Value {
    match origin {
        Origin::SpectrumSolve => json!({"kind": "spectrum_solve", "produced_by_spectrum_solve": true, "reference": null,
            "constructed": [], "statement": ORIGIN_STATEMENTS[0]}),
        Origin::ConstructedFromReference { package, case, analysis, constructed } => json!({
            "kind": "constructed_from_reference",
            "produced_by_spectrum_solve": false,
            "reference": {"package": package, "case": case, "analysis": analysis},
            "constructed": constructed,
            "statement": ORIGIN_STATEMENTS[1],
        }),
    }
}

/// The digests of the model and the request, each formed once; `None` outside the checked
/// canonical profile.
struct Bindings {
    model: Option<String>,
    request: Option<String>,
}

impl Bindings {
    fn of(inputs: &DocumentInputs) -> Self {
        Self { model: spectrum_doc_sha256(inputs.model), request: spectrum_doc_sha256(inputs.request) }
    }

    /// A refusal for each input outside the checked canonical profile.
    fn refusals(&self) -> Vec<Refusal> {
        [("model_document", &self.model), ("spectrum_request", &self.request)]
            .into_iter()
            .filter(|(_, sha)| sha.is_none())
            .map(|(binds, _)| Refusal::new(RefusalCode::DocumentOutsideCanonicalProfile, binds, None,
                format!("the {binds} is outside the checked canonical profile (too deep, or a number outside it), so it cannot be bound")))
            .collect()
    }
}

/// A state case index outside the checked canonical profile: the same refusal wherever it is
/// found, so that the refused form states it once.
fn state_case_index_refusal(index: usize) -> Refusal {
    Refusal::new(RefusalCode::DocumentOutsideCanonicalProfile, "modal_result", Some("state_case"),
        format!("the state case index {index} is 2^53 or more"))
}

fn binding_json(binds: &str, sha: &Option<String>) -> Value {
    json!({"binds": binds, "hash_algorithm": SPECTRUM_HASH_ALGORITHM, "canonical_form": SPECTRUM_RESULT_CANONICAL, "sha256": sha})
}

fn modal_result_json(b: &ModalResultBinding, n: &mut Numbers) -> Value {
    match b {
        ModalResultBinding::Bound { run_id, request_sha256, model_sha256, materials_sha256, integrity_parameters_sha256, state_case } => {
            let state_case = match state_case {
                Some((id, index, sha)) => {
                    let case_index = if *index as u64 > MAX_SAFE_INTEGER {
                        n.outside.push(state_case_index_refusal(*index));
                        Value::Null
                    } else {
                        json!(index)
                    };
                    json!({"case_id": id, "case_index": case_index, "case_sha256": sha})
                }
                None => Value::Null,
            };
            json!({
                "binding": "bound", "reason": null, "run_id": run_id, "request_sha256": request_sha256, "model_sha256": model_sha256,
                "materials_sha256": materials_sha256, "integrity_parameters_sha256": integrity_parameters_sha256, "state_case": state_case,
            })
        }
        ModalResultBinding::NotApplicable { reason } => json!({
            "binding": "not_applicable", "reason": reason, "run_id": null, "request_sha256": null, "model_sha256": null,
            "materials_sha256": null, "integrity_parameters_sha256": null, "state_case": null,
        }),
    }
}

fn identity(inputs: &DocumentInputs, bindings: &Bindings, numbers: &mut Numbers) -> Map<String, Value> {
    let mut doc = Map::new();
    doc.insert("document_kind".into(), json!(SPECTRUM_RESULT_KIND));
    doc.insert("schema_version".into(), json!(SPECTRUM_RESULT_VERSION));
    doc.insert("semantic_contract".into(), json!({"id": SPECTRUM_RESULT_SEMANTICS, "sha256": spectrum_doc_sha256(&spectrum_semantic_table())}));
    doc.insert("profile".into(), json!({"profile_id": SPECTRUM_RESULT_PROFILE, "limitations": SPECTRUM_RESULT_LIMITATIONS}));
    doc.insert("canonical_profile".into(), json!(SPECTRUM_RESULT_CANONICAL));
    doc.insert("admission".into(), json!({"status": "experimental", "public_engineering_result": false}));
    doc.insert("origin".into(), origin_json(inputs.origin));
    doc.insert("bindings".into(), json!({
        "model": binding_json("model_document", &bindings.model),
        "request": binding_json("spectrum_request", &bindings.request),
    }));
    doc.insert("modal_result".into(), modal_result_json(inputs.modal_result, numbers));
    doc
}

fn refused(inputs: &DocumentInputs, bindings: &Bindings, refusals: &[Refusal]) -> Value {
    let mut numbers = Numbers { outside: Vec::new() };
    let mut doc = identity(inputs, bindings, &mut numbers);
    doc.insert("status".into(), json!("refused"));
    let extra: Vec<&Refusal> = numbers.outside.iter().filter(|r| !refusals.contains(r)).collect();
    let rows = refusals
        .iter()
        .chain(extra)
        .enumerate()
        .map(|(i, r)| seal(format!("refusal:{}", i + 1), i, json!({"diagnostic_id": format!("diagnostic:spectrum:refusal:{}", i + 1),
            "code": r.code.as_str(), "severity": "blocking", "object": r.object, "field": r.field, "detail": r.detail})))
        .collect();
    doc.insert("refusals".into(), table(rows));
    Value::Object(doc)
}

/// The checks of the inputs against one another that the combinations do not make.
fn check_agreement(inputs: &DocumentInputs, choices: &Choices, tables: &[SpectrumTable], refusals: &mut Vec<Refusal>) {
    let c = &inputs.contributions.combination;
    let mismatch = |field: &str, detail: String| input_invalid("combination", field, detail);
    if c.modal_rule != choices.modal_rule {
        refusals.push(mismatch("modal_combination", "the contributions were combined under another modal rule than the request's".into()));
    }
    if c.directional_rule != choices.directional_rule {
        refusals.push(mismatch("directional_combination", "the directional rule is not the request's".into()));
    }
    if c.missing_mass != choices.missing_mass {
        refusals.push(mismatch("missing_mass", "the missing-mass choice, rigid frequency or rigid rule is not the request's".into()));
    }
    let mut requested: Vec<Direction> = c.directions.clone();
    requested.sort();
    let chosen: Vec<Direction> = choices.directions.iter().map(|(d, _, _)| *d).collect();
    if requested != chosen {
        refusals.push(mismatch("directions", "the directions are not the request's".into()));
    }
    // The damping ratio is the spectra's (REQ-001); they must agree with each other.
    let table_refs: Vec<&SpectrumTable> = tables.iter().collect();
    match crate::table::common_damping_ratio(&table_refs) {
        Err(r) => refusals.push(r),
        Ok(Some(z)) if z.to_bits() != c.damping_ratio.to_bits() => {
            refusals.push(mismatch("damping_ratio", format!("the contributions' damping ratio {} is not the spectra's {z}", c.damping_ratio)))
        }
        _ => {}
    }
    check_formation(inputs, choices, refusals);
    let m = inputs.modal;
    let lengths_hold = m.mode_standings.len() == c.modes.len() && m.effective_mass_kg.len() == c.modes.len();
    if !lengths_hold {
        refusals.push(input_invalid("modal_basis", "modes", "the modal statement does not state every extracted mode".into()));
    }
    if m.extraction.reported_mode_count != c.modes.len() {
        refusals.push(input_invalid("modal_basis", "extraction", format!("the modal endpoint reports {} modes; {} are supplied",
            m.extraction.reported_mode_count, c.modes.len())));
    }
    check_extraction(inputs, refusals);
    check_modal_binding(inputs.modal_result, refusals);
    if lengths_hold {
        check_mass_operands(inputs, refusals);
    }
    check_cluster_rule(inputs, refusals);
    check_census_closed(inputs.census, refusals);
    check_census_entries(inputs.census, refusals);
    check_frequency_relation(inputs, refusals);
    check_origin_binding(inputs, refusals);
    if let Some(state) = &inputs.modal.state {
        // The state is placed at /modal_basis/state: the document is level 1, modal_basis level 2.
        if too_deep(state, 3) || spectrum_doc_sha256(state).is_none() {
            refusals.push(Refusal::new(RefusalCode::DocumentOutsideCanonicalProfile, "modal_basis", Some("state"),
                "the modal state statement, placed at /modal_basis/state, is outside the checked canonical profile".into()));
        }
    }
    if inputs.modal.extraction.established_finite_mode_count.is_some_and(|n| n as u64 > MAX_SAFE_INTEGER) {
        refusals.push(Refusal::new(RefusalCode::DocumentOutsideCanonicalProfile, "modal_basis", Some("extraction"),
            "the established finite mode count is 2^53 or more".into()));
    }
}

/// Each requested direction's formation record against the bound request and model: one record
/// per direction with contributions, none for another direction, its spectrum the request's for
/// that direction, its record digest that of the model's record of that identity, and its scale
/// factor the request's, bit for bit.
fn check_formation(inputs: &DocumentInputs, choices: &Choices, refusals: &mut Vec<Refusal>) {
    let records = &inputs.contributions.formation;
    let bad = |d: Direction, detail: String| input_invalid(d.as_str(), "formation", detail);
    let mut counted = [0usize; 3];
    for r in records {
        counted[direction_index(r.direction)] += 1;
    }
    for r in records {
        if !inputs.contributions.combination.directions.contains(&r.direction) {
            refusals.push(bad(r.direction, format!("a formation record is supplied for direction {}, which has no contributions", r.direction)));
        }
    }
    for d in Direction::ALL {
        let n = counted[direction_index(d)];
        if n > 1 {
            refusals.push(bad(d, format!("{n} formation records are supplied for direction {d}")));
        }
    }
    for &d in &inputs.contributions.combination.directions {
        let Some(record) = records.iter().find(|r| r.direction == d) else {
            refusals.push(bad(d, format!("direction {d} has contributions and no formation record")));
            continue;
        };
        let Some((_, spectrum, scale)) = choices.directions.iter().find(|(e, _, _)| *e == d) else {
            continue; // the direction set's mismatch is refused above
        };
        if record.spectrum_id != *spectrum {
            refusals.push(bad(d, format!("the contributions were formed from spectrum `{}`; the request names `{spectrum}`", record.spectrum_id)));
        } else if let (Some(model_record), _) = spectrum_records(inputs.model, spectrum) {
            if spectrum_doc_sha256(model_record).as_deref() != Some(record.spectrum_record_sha256.as_str()) {
                refusals.push(bad(d, format!("the contributions were formed from a record of spectrum `{spectrum}` whose digest is not that of the model's record")));
            }
        }
        if record.scale_factor.to_bits() != scale.to_bits() {
            refusals.push(bad(d, format!("the contributions were formed with scale factor {:e}; the request's is {scale:e}", record.scale_factor)));
        }
    }
}

/// The three endpoints of the modal family (`ReportedEndpoint`, `core/product_physics`).
const ENDPOINTS: [&str; 3] = ["whole_finite_spectrum", "below_computed_cluster", "below_lowest_mode"];

/// The extraction statement's joint invariants: the whole finite spectrum has an established
/// finite count equal to the reported count and no next computed mode; below a computed cluster
/// there is a reported mode and both next-frequency values, the next frequency not below the last
/// reported one; below the lowest mode there is no reported mode and both next-frequency values;
/// an established count is not below the reported count, and exceeds it where a next computed
/// mode is stated; the completion evidence is not blank.
fn check_extraction(inputs: &DocumentInputs, refusals: &mut Vec<Refusal>) {
    let e = &inputs.modal.extraction;
    let modes = &inputs.contributions.combination.modes;
    let bad = |detail: String| input_invalid("modal_basis", "extraction", detail);
    let both = e.next_computed_frequency_hz.is_some() && e.next_computed_circular_frequency_rad_s.is_some();
    let stated = e.next_computed_frequency_hz.is_some() || e.next_computed_circular_frequency_rad_s.is_some();
    let n = e.reported_mode_count;
    match e.endpoint.as_str() {
        "whole_finite_spectrum" => {
            if e.established_finite_mode_count != Some(n) {
                refusals.push(bad(format!("the whole finite spectrum is stated with an established finite mode count of {:?}, not the {n} reported",
                    e.established_finite_mode_count)));
            }
            if stated {
                refusals.push(bad("the whole finite spectrum is stated with a next computed mode".into()));
            }
        }
        "below_computed_cluster" => {
            if n == 0 {
                refusals.push(bad("the extraction is stated below a computed cluster with no reported mode".into()));
            }
            if !both {
                refusals.push(bad("the extraction is stated below a computed cluster without both the next computed mode's frequency and circular frequency".into()));
            }
            if let (Some(f), Some(last)) = (e.next_computed_frequency_hz, modes.last()) {
                if !(f >= last.frequency_hz) {
                    refusals.push(bad(format!("the next computed frequency {f:e} Hz is below the last reported mode's {:e} Hz", last.frequency_hz)));
                }
            }
        }
        "below_lowest_mode" => {
            if n != 0 {
                refusals.push(bad(format!("the extraction is stated below the lowest mode with {n} reported modes")));
            }
            if !both {
                refusals.push(bad("the extraction is stated below the lowest mode without both the next computed mode's frequency and circular frequency".into()));
            }
        }
        other => refusals.push(bad(format!("`{other}` is not an extraction endpoint (one of {})", ENDPOINTS.join(", ")))),
    }
    if let Some(established) = e.established_finite_mode_count {
        if established < n {
            refusals.push(bad(format!("the established finite mode count {established} is below the {n} reported")));
        } else if stated && established == n {
            refusals.push(bad(format!("a next computed mode is stated and the established finite mode count {established} does not exceed the {n} reported")));
        }
    }
    if e.evidence.trim().is_empty() {
        refusals.push(bad("the completion evidence is blank".into()));
    }
}

/// 64 lowercase hexadecimal digits.
fn is_sha256_hex(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// The modal result binding's fields. A bound one: the request, materials and integrity-parameter
/// digests are each 64 lowercase hexadecimal digits, the model digest is one or null, a state
/// case has a non-blank id and such a digest, and the run id is the modal result family's:
/// `modal-run:` and the first 16 hexadecimal digits of the SHA-256 of the canonical text of
/// `{"request_sha256", "integrity_parameters_sha256"}` (`core/product_physics/src/modal/document.rs`,
/// `modal_doc_identity`; the modal reader checks the same, `core/reporting/modal_result`,
/// `mr_binding`). An unbound one: a non-blank reason.
fn check_modal_binding(binding: &ModalResultBinding, refusals: &mut Vec<Refusal>) {
    let bad = |field: &str, detail: String| input_invalid("modal_result", field, detail);
    match binding {
        ModalResultBinding::Bound { run_id, request_sha256, model_sha256, materials_sha256, integrity_parameters_sha256, state_case } => {
            for (field, v) in [("request_sha256", request_sha256), ("materials_sha256", materials_sha256),
                ("integrity_parameters_sha256", integrity_parameters_sha256)] {
                if !is_sha256_hex(v) {
                    refusals.push(bad(field, format!("`{v}` is not 64 lowercase hexadecimal digits")));
                }
            }
            if let Some(v) = model_sha256 {
                if !is_sha256_hex(v) {
                    refusals.push(bad("model_sha256", format!("`{v}` is neither null nor 64 lowercase hexadecimal digits")));
                }
            }
            if run_id.trim().is_empty() {
                refusals.push(bad("run_id", "the run id is blank".into()));
            } else if is_sha256_hex(request_sha256) && is_sha256_hex(integrity_parameters_sha256) {
                let run = spectrum_doc_sha256(&json!({"request_sha256": request_sha256, "integrity_parameters_sha256": integrity_parameters_sha256}));
                let expected = run.map(|r| format!("modal-run:{}", &r[..16]));
                if expected.as_deref() != Some(run_id.as_str()) {
                    refusals.push(bad("run_id", format!("`{run_id}` is not the modal result family's run id of the stated request and integrity-parameter digests")));
                }
            }
            if let Some((id, index, sha)) = state_case {
                if id.trim().is_empty() {
                    refusals.push(bad("state_case", "the state case id is blank".into()));
                }
                if !is_sha256_hex(sha) {
                    refusals.push(bad("state_case", format!("the state case digest `{sha}` is not 64 lowercase hexadecimal digits")));
                }
                if *index as u64 > MAX_SAFE_INTEGER {
                    refusals.push(state_case_index_refusal(*index));
                }
            }
        }
        ModalResultBinding::NotApplicable { reason } => {
            if reason.trim().is_empty() {
                refusals.push(bad("reason", "a modal result binding that is not applicable states no reason".into()));
            }
        }
    }
}

/// The missing-mass operands of each requested direction, before any arithmetic on them: T and
/// every mode's effective mass finite (otherwise outside the canonical profile) and not negative,
/// and every effective mass zero where T is zero.
fn check_mass_operands(inputs: &DocumentInputs, refusals: &mut Vec<Refusal>) {
    let m = inputs.modal;
    let modes = &inputs.contributions.combination.modes;
    for &d in &inputs.contributions.combination.directions {
        let k = direction_index(d);
        let t = m.total_mass_kg[k];
        let entry = format!("missing_mass/{d}");
        let mut operands = vec![("the total mass".to_string(), t)];
        operands.extend(modes.iter().zip(&m.effective_mass_kg).map(|(mode, e)| (format!("{}'s effective mass", mode_ref(mode.mode.0)), e[k])));
        for (what, v) in &operands {
            if !v.is_finite() {
                refusals.push(Refusal::new(RefusalCode::DocumentOutsideCanonicalProfile, &entry, Some("mass"),
                    format!("{what} in direction {d} is {v}, outside the checked canonical profile")));
            } else if *v < 0.0 {
                refusals.push(input_invalid(&entry, "mass", format!("{what} in direction {d} is {v:e}, negative")));
            }
        }
        if t == 0.0 && operands[1..].iter().any(|(_, v)| *v != 0.0) {
            refusals.push(input_invalid(&entry, "mass", format!("the total mass in direction {d} is zero and an effective mass is not")));
        }
    }
}

/// The tolerance of the census frames' orthonormality and handedness, 2^-40.
const FRAME_TOLERANCE: f64 = 1.0 / 1_099_511_627_776.0;

/// a·b as (a0·b0 + a1·b1) + a2·b2, each operation one binary64 rounding (Rust does not contract
/// these into fused multiply-adds).
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    (a[0] * b[0] + a[1] * b[1]) + a[2] * b[2]
}

/// The failures of a frame's rows x, y and z: |x·x − 1|, |y·y − 1|, |z·z − 1|, |x·y|, |y·z|,
/// |z·x| and each component of |x × y − z| at most 2^-40 (NaN fails).
fn frame_failure(t: &Triad) -> Option<String> {
    let [x, y, z] = *t;
    let cross = [x[1] * y[2] - x[2] * y[1], x[2] * y[0] - x[0] * y[2], x[0] * y[1] - x[1] * y[0]];
    let checks = [
        ("|x.x - 1|", (dot(x, x) - 1.0).abs()), ("|y.y - 1|", (dot(y, y) - 1.0).abs()), ("|z.z - 1|", (dot(z, z) - 1.0).abs()),
        ("|x.y|", dot(x, y).abs()), ("|y.z|", dot(y, z).abs()), ("|z.x|", dot(z, x).abs()),
        ("|(x cross y - z)_x|", (cross[0] - z[0]).abs()), ("|(x cross y - z)_y|", (cross[1] - z[1]).abs()),
        ("|(x cross y - z)_z|", (cross[2] - z[2]).abs()),
    ];
    checks.iter().find(|(_, v)| !(*v <= FRAME_TOLERANCE)).map(|(what, v)| format!("{what} = {v:e} exceeds 2^-40"))
}

/// Each census frame is orthonormal and right-handed to 2^-40 and each rod axis a unit vector to
/// 2^-40; each entity occurs once in its table; each rigid constraint names at least one holder and
/// no holder twice.
fn check_census_entries(census: &Census, refusals: &mut Vec<Refusal>) {
    let mut frame = |entry: String, t: &Triad| {
        if let Some(f) = frame_failure(t) {
            refusals.push(input_invalid("census", "frame", format!("{entry}: {f}")));
        }
    };
    for m in &census.straight_members {
        frame(format!("straight member {} member axes", m.id), &m.axes);
    }
    for a in &census.arcs {
        frame(format!("arc {} tangent frame at end i", a.id), &a.tangent_frame_i);
        frame(format!("arc {} tangent frame at end j", a.id), &a.tangent_frame_j);
    }
    for c in &census.connectors {
        frame(format!("connector {} connector axes", c.id), &c.axes);
    }
    for t in &census.tie_rods {
        let v = (dot(t.axis, t.axis) - 1.0).abs();
        if !(v <= FRAME_TOLERANCE) {
            refusals.push(input_invalid("census", "frame", format!("tie rod {} of connector {}: |a.a - 1| = {v:e} exceeds 2^-40", t.rod, t.connector)));
        }
    }
    // Each entity once in its table, compared by its identity fields as borrowed tuples; a label is
    // composed only for a refusal.
    fn once<K: Ord>(table: &str, keys: impl Iterator<Item = K>, label: impl Fn(&K) -> String, refusals: &mut Vec<Refusal>) {
        let mut seen = BTreeSet::new();
        for key in keys {
            if seen.contains(&key) {
                refusals.push(input_invalid("census", table, format!("{} occurs more than once in the census {table} table", label(&key))));
            } else {
                seen.insert(key);
            }
        }
    }
    once("nodes", census.nodes.iter().map(String::as_str), |n| format!("node {n}"), refusals);
    once("straight_members", census.straight_members.iter().map(|m| m.id.as_str()), |id| format!("straight member {id}"), refusals);
    once("arcs", census.arcs.iter().map(|m| m.id.as_str()), |id| format!("arc {id}"), refusals);
    once("connectors", census.connectors.iter().map(|c| c.id.as_str()), |id| format!("connector {id}"), refusals);
    once("tie_rods", census.tie_rods.iter().map(|t| (t.connector.as_str(), t.rod.as_str())),
        |(connector, rod)| format!("tie rod {rod} of connector {connector}"), refusals);
    once("rigid_constraints", census.rigid_constraints.iter().map(|r| (r.node.as_str(), r.component)),
        |(node, component)| format!("rigid constraint at {node} {}", component.as_str()), refusals);
    once("compliant_supports", census.compliant_supports.iter().map(|s| s.support.as_str()), |id| format!("compliant support {id}"), refusals);
    once("supports_without_rows", census.supports_without_rows.iter().map(|s| s.support.as_str()), |id| format!("support {id}"), refusals);
    for r in &census.rigid_constraints {
        let entry = format!("rigid constraint at {} {}", r.node, r.component.as_str());
        if r.holders.is_empty() {
            refusals.push(input_invalid("census", "holders", format!("{entry} names no holder")));
        }
        let mut seen = BTreeSet::new();
        for h in &r.holders {
            if !seen.insert(h) {
                refusals.push(input_invalid("census", "holders", format!("{entry} names holder {h} twice")));
            }
        }
    }
}

/// Each extracted mode's, and the next computed mode's, circular frequency w against its
/// frequency f: |w − c·f| ≤ 8·u·w, with c = 2.0 · π (binary64) and c·f one binary64 product.
fn check_frequency_relation(inputs: &DocumentInputs, refusals: &mut Vec<Refusal>) {
    let c = 2.0 * std::f64::consts::PI;
    let (m, modes) = (inputs.modal, &inputs.contributions.combination.modes);
    let mut pairs: Vec<(String, f64, f64)> =
        modes.iter().zip(&m.circular_frequency_rad_s).map(|(mode, w)| (mode_ref(mode.mode.0), *w, mode.frequency_hz)).collect();
    if let (Some(w), Some(f)) = (m.extraction.next_computed_circular_frequency_rad_s, m.extraction.next_computed_frequency_hz) {
        pairs.push(("the next computed mode".into(), w, f));
    }
    for (what, w, f) in pairs {
        let deviation = (w - c * f).abs();
        if !(deviation <= 8.0 * UNIT_ROUNDOFF * w) {
            refusals.push(input_invalid("modal_basis", "circular_frequency",
                format!("{what}: its circular frequency {w:e} rad/s is not 2 pi times its frequency {f:e} Hz within 8u")));
        }
    }
}

/// A spectrum solve binds its modal result; a document constructed from a reference has none.
fn check_origin_binding(inputs: &DocumentInputs, refusals: &mut Vec<Refusal>) {
    match (inputs.origin, inputs.modal_result) {
        (Origin::SpectrumSolve, ModalResultBinding::NotApplicable { .. }) => refusals.push(input_invalid("modal_result", "binding",
            "a document produced by a spectrum solve states its modal result binding as not applicable".into())),
        (Origin::ConstructedFromReference { .. }, ModalResultBinding::Bound { .. }) => refusals.push(input_invalid("modal_result", "binding",
            "a document constructed from a reference binds a modal result".into())),
        _ => {}
    }
}

/// The census is closed on itself: every node a straight member, an arc, a connector, a rigid
/// constraint or a compliant support names is a row of its nodes table, and every tie rod's
/// connector is a row of its connectors table.
fn check_census_closed(census: &Census, refusals: &mut Vec<Refusal>) {
    let nodes: BTreeSet<&str> = census.nodes.iter().map(String::as_str).collect();
    let connectors: BTreeSet<&str> = census.connectors.iter().map(|c| c.id.as_str()).collect();
    let mut named: Vec<(String, &str)> = Vec::new();
    for m in &census.straight_members {
        named.extend([(format!("straight member {}", m.id), m.end_i.as_str()), (format!("straight member {}", m.id), m.end_j.as_str())]);
    }
    for m in &census.arcs {
        named.extend([(format!("arc {}", m.id), m.end_i.as_str()), (format!("arc {}", m.id), m.end_j.as_str())]);
    }
    for c in &census.connectors {
        named.extend([(format!("connector {}", c.id), c.end_i.as_str()), (format!("connector {}", c.id), c.end_j.as_str())]);
    }
    for r in &census.rigid_constraints {
        named.push((format!("rigid constraint at {} {}", r.node, r.component.as_str()), r.node.as_str()));
    }
    for s in &census.compliant_supports {
        named.push((format!("compliant support {}", s.support), s.node.as_str()));
    }
    for (entry, node) in named {
        if !nodes.contains(node) {
            refusals.push(input_invalid("census", "nodes", format!("{entry} names node {node}, which is not a census node")));
        }
    }
    for t in &census.tie_rods {
        if !connectors.contains(t.connector.as_str()) {
            refusals.push(input_invalid("census", "connectors",
                format!("tie rod {} names connector {}, which is not a census connector", t.rod, t.connector)));
        }
    }
}

/// The unit roundoff 2^-53 of the modal reader's cluster margin.
const UNIT_ROUNDOFF: f64 = 1.0 / 9_007_199_254_740_992.0;

/// The stated clusters against the cluster rule on the stated circular frequencies and cluster
/// tolerance, as the modal reader evaluates it (`core/reporting/modal_result`, `mr_clusters`), in
/// binary64 with the same expressions: for adjacent modes a < b, gap = ω_b² − ω_a², threshold =
/// δ_c·ω_b², margin = 16u(1 + δ_c)·ω_b² with u = 2^-53. A shared cluster across a gap above
/// threshold + margin, or a split below threshold − margin, is refused, naming the two modes; a
/// pair inside the margin is not. When the endpoint is below a computed cluster, the last
/// extracted mode and the next computed one lie in different clusters and are held to the same
/// rule.
fn check_cluster_rule(inputs: &DocumentInputs, refusals: &mut Vec<Refusal>) {
    let (m, modes) = (inputs.modal, &inputs.contributions.combination.modes);
    let omega = &m.circular_frequency_rad_s;
    let delta = m.cluster_tolerance;
    if !(delta > 0.0 && delta < 1.0) {
        refusals.push(input_invalid("modal_basis", "cluster_tolerance", format!("the cluster tolerance {delta:e} is not in (0, 1)")));
        return;
    }
    if omega.len() != modes.len() {
        refusals.push(input_invalid("modal_basis", "modes", "the modal statement does not state every extracted mode's circular frequency".into()));
        return;
    }
    let next = m.extraction.next_computed_circular_frequency_rad_s;
    if next.is_some() != m.extraction.next_computed_frequency_hz.is_some() {
        return; // refused by the extraction check
    }
    if let Some(w) = omega.iter().copied().chain(next).find(|w| !(w.is_finite() && *w > 0.0)) {
        refusals.push(input_invalid("modal_basis", "modes", format!("a circular frequency {w:e} is not positive and finite")));
        return;
    }
    let mut pairs: Vec<(String, String, f64, f64, bool)> = modes
        .windows(2)
        .zip(omega.windows(2))
        .map(|(p, w)| (mode_ref(p[0].mode.0), mode_ref(p[1].mode.0), w[0], w[1], p[0].cluster == p[1].cluster))
        .collect();
    if let (Some(w), Some(last), Some(&wl)) = (next, modes.last(), omega.last()) {
        if m.extraction.endpoint == "below_computed_cluster" {
            pairs.push((mode_ref(last.mode.0), "the next computed mode".into(), wl, w, false));
        }
    }
    for (a, b, wa, wb, same) in pairs {
        let gap = wb * wb - wa * wa;
        let threshold = delta * (wb * wb);
        let margin = 16.0 * UNIT_ROUNDOFF * (1.0 + delta) * (wb * wb);
        if same && gap > threshold + margin {
            refusals.push(input_invalid("modal_basis", "clusters", format!("{a} and {b} share a cluster across a relative gap above the cluster tolerance")));
        } else if !same && gap < threshold - margin {
            refusals.push(input_invalid("modal_basis", "clusters", format!("{a} and {b} are split within the cluster tolerance")));
        }
    }
}

/// The rows are exactly the roster the census gives, each identity once, with the census's
/// family of each member and the census's holders of each constraint as a set (the document emits
/// them in the census's order).
fn check_roster(census: &Census, rows: &[Quantity], refusals: &mut Vec<Refusal>) {
    let expected = census.roster();
    let mut by_identity: BTreeMap<RowIdentity, &Quantity> = BTreeMap::new();
    for q in &expected {
        if by_identity.insert(q.identity(), q).is_some() {
            refusals.push(input_invalid("census", "roster", format!("the census gives {} more than once", q.label())));
        }
    }
    let same = |e: &Quantity, q: &Quantity| match (e, q) {
        (Quantity::ConstraintReaction { holders: a, .. }, Quantity::ConstraintReaction { holders: b, .. }) => {
            a.len() == b.len() && a.iter().collect::<BTreeSet<_>>() == b.iter().collect::<BTreeSet<_>>()
        }
        _ => e == q,
    };
    let mut seen = BTreeSet::new();
    for q in rows {
        let id = q.identity();
        match by_identity.get(&id) {
            None => refusals.push(input_invalid(&q.label(), "roster", "the row is not in the roster the census gives".into())),
            Some(e) if !same(e, q) => refusals.push(input_invalid(&q.label(), "roster",
                "the row's member family or constraint holders are not the census's".into())),
            Some(_) => {
                if !seen.insert(id) {
                    refusals.push(input_invalid(&q.label(), "roster", "the row is supplied more than once".into()));
                }
            }
        }
    }
    for q in &expected {
        if !seen.contains(&q.identity()) {
            refusals.push(input_invalid(&q.label(), "roster", "the census gives this row and no row is supplied for it".into()));
        }
    }
}

fn restrained_component(c: ReactionComponent) -> DisplacementComponent {
    match c {
        ReactionComponent::Ux => DisplacementComponent::Ux,
        ReactionComponent::Uy => DisplacementComponent::Uy,
        ReactionComponent::Uz => DisplacementComponent::Uz,
        ReactionComponent::Rx => DisplacementComponent::Rx,
        ReactionComponent::Ry => DisplacementComponent::Ry,
        ReactionComponent::Rz => DisplacementComponent::Rz,
    }
}

/// A component restrained on a global axis is zero (REQ-006, Nodes): a node displacement or
/// rotation row whose component a census rigid constraint restrains at that node has every
/// signed contribution and rigid value zero. One refusal per row, naming the first nonzero value.
fn check_restrained_zero(inputs: &DocumentInputs, refusals: &mut Vec<Refusal>) {
    let c = &inputs.contributions.combination;
    let restrained: BTreeSet<(&str, DisplacementComponent)> =
        inputs.census.rigid_constraints.iter().map(|r| (r.node.as_str(), restrained_component(r.component))).collect();
    for (r, q) in c.rows.iter().enumerate() {
        let Quantity::NodeDisplacement { node, component } = q else { continue };
        if !restrained.contains(&(node.as_str(), *component)) {
            continue;
        }
        let periodic = c.periodic.iter().map(|p| (format!("{} in direction {}", mode_ref(p.mode.0), p.direction), p.values.get(r)));
        let rigid = c.rigid.iter().map(|p| (format!("the rigid response in direction {}", p.direction), p.values.get(r)));
        if let Some((what, v)) = periodic.chain(rigid).find_map(|(what, v)| v.filter(|v| **v != 0.0).map(|v| (what, *v))) {
            refusals.push(input_invalid(&q.label(), "contribution", format!(
                "a census rigid constraint restrains this component at this node, so it is zero (REQ-006, Nodes); {what} is {v:e}")));
        }
    }
}

fn axis_scale(s: AxisScale) -> &'static str {
    match s {
        AxisScale::Linear => "linear",
        AxisScale::Log => "log",
    }
}

fn location_json(l: Location) -> Value {
    match l {
        Location::Knot { index } => json!({"kind": "knot", "index": index}),
        Location::Segment { lower } => json!({"kind": "segment", "lower": lower}),
    }
}

/// Assembles the spectrum result document of `inputs` (module documentation, "Stages").
pub fn spectrum_result_document(inputs: &DocumentInputs) -> Value {
    let request = &inputs.contributions.combination;
    // 1. The bindings.
    let bindings = Bindings::of(inputs);
    let refusals = bindings.refusals();
    if !refusals.is_empty() {
        return refused(inputs, &bindings, &refusals);
    }
    // 2. The inputs against one another.
    let mut refusals = Vec::new();
    let choices = read_choices(inputs.request, &mut refusals);
    let tables = match &choices {
        Some(ch) => read_spectra(inputs.model, ch, &mut refusals),
        None => Vec::new(),
    };
    let Some(choices) = choices.filter(|_| refusals.is_empty()) else {
        return refused(inputs, &bindings, &refusals);
    };
    check_agreement(inputs, &choices, &tables, &mut refusals);
    if !refusals.is_empty() {
        return refused(inputs, &bindings, &refusals);
    }
    // 3. The rows.
    crate::combine::check_rows(request, &mut refusals, false);
    if refusals.is_empty() {
        check_roster(inputs.census, &request.rows, &mut refusals);
    }
    if refusals.is_empty() {
        check_restrained_zero(inputs, &mut refusals);
    }
    if !refusals.is_empty() {
        return refused(inputs, &bindings, &refusals);
    }
    // 4. The modes, then the range and the extraction premise.
    if !crate::combine::check_modes(request, &mut refusals) {
        return refused(inputs, &bindings, &refusals);
    }
    let clusters = crate::combine::clusters(request);
    let disclosures = disclosure_operands(inputs, &choices, &tables, &clusters, &mut refusals);
    if !refusals.is_empty() {
        return refused(inputs, &bindings, &refusals);
    }
    // 5. The combinations.
    let combination = match combine(request) {
        Ok(c) => c,
        Err(r) => return refused(inputs, &bindings, &r),
    };
    // 6. The published numbers.
    let mut numbers = Numbers { outside: Vec::new() };
    let doc = solved(inputs, &bindings, &choices, &tables, &combination, disclosures, &mut numbers);
    if !numbers.outside.is_empty() {
        return refused(inputs, &bindings, &numbers.outside);
    }
    // 7. The document as a whole.
    if spectrum_doc_sha256(&doc).is_none() {
        return refused(inputs, &bindings, &[Refusal::new(RefusalCode::DocumentOutsideCanonicalProfile, "document", None,
            "the assembled document is outside the checked canonical profile".into())]);
    }
    doc
}

/// The range and extraction-premise operands, checked: a periodic mode or the rigid frequency
/// outside a spectrum's tabulated range, and an applied correction whose extracted set does not
/// reach f_r, are refused by name (REQ-001).
struct Disclosures {
    /// Per spectrum: its locations of each periodic mode's frequency and of f_r.
    ranges: Vec<Vec<(String, f64, Location)>>,
    premise_holds: Option<bool>,
}

fn disclosure_operands(inputs: &DocumentInputs, choices: &Choices, tables: &[SpectrumTable], clusters: &[Cluster],
    refusals: &mut Vec<Refusal>) -> Disclosures {
    let modes = &inputs.contributions.combination.modes;
    let periodic: Vec<(u32, f64)> = clusters
        .iter()
        .filter(|s| s.class == ClusterClass::Periodic)
        .flat_map(|s| s.modes.iter().map(|&p| (modes[p].mode.0, modes[p].frequency_hz)))
        .collect();
    let mut ranges = Vec::with_capacity(tables.len());
    for t in tables {
        let mut evaluated = Vec::with_capacity(periodic.len() + 1);
        let mut at = |what: String, f: f64, refusals: &mut Vec<Refusal>| match QueryFrequency::new(f, "Hz").and_then(|q| t.locate(q)) {
            Ok(l) => evaluated.push((what, f, l)),
            Err(r) => refusals.push(Refusal::new(r.code, &what, Some(t.id()), r.detail)),
        };
        for (m, f) in &periodic {
            at(mode_ref(*m), *f, refusals);
        }
        if let MissingMass::Applied { rigid_frequency_hz, .. } = choices.missing_mass {
            at("rigid_frequency".into(), rigid_frequency_hz, refusals);
        }
        ranges.push(evaluated);
    }
    let premise_holds = match choices.missing_mass {
        MissingMass::NotApplied => None,
        MissingMass::Applied { rigid_frequency_hz, .. } => {
            let whole = inputs.modal.extraction.endpoint == "whole_finite_spectrum";
            // The highest completed cluster is the last one; its lowest frequency is its standing.
            let highest = clusters.last().map(|s| s.standing);
            let holds = whole || highest.is_some_and(|f| f >= rigid_frequency_hz);
            if !holds {
                refusals.push(Refusal::new(RefusalCode::ExtractionBelowRigidFrequency, "modal_basis", Some("extraction"),
                    format!("the lowest frequency of the highest completed cluster is below the rigid frequency {rigid_frequency_hz} Hz and the extracted set is not the whole finite spectrum")));
            }
            Some(holds)
        }
    };
    Disclosures { ranges, premise_holds }
}

/// The fixed disclosures: (id, text). Each document carries the first four, one of the next two
/// by whether the correction is applied, and the last where the census has an arc.
pub const FIXED_DISCLOSURES: [(&str, &str); 7] = [
    ("pseudo_acceleration", "Every spectrum ordinate is a pseudo-spectral acceleration: omega squared times the peak relative \
displacement of a linear oscillator at the spectrum's damping. No other kind of spectrum is converted and no spectrum is extrapolated."),
    ("uniform_support_motion", "Uniform support motion: in each requested direction every rigid support moves with the ground. \
Independent support motion is not built."),
    ("velocity_terms_neglected", "Damping is represented only through the spectra. Each mode's quantities are recovered at its \
displacement extremum, and the velocity-proportional parts of forces and reactions are neglected; this is an adopted \
approximation, and no bound on the resulting force or reaction error is claimed."),
    ("repeated_modes", "The modes of each cluster of the modal basis are summed with their signs first and the modal rule is \
applied across the clusters. A cluster is the modal basis's numerical convention, not a statement that its eigenvalues are \
equal; for a cluster whose modes have unequal frequencies, summing first is an adopted convention, and no independence from \
the modal basis is claimed for it."),
    ("rigid_response_premise", "The missing-mass correction is applied. The rigid \
response is the static response, on the dynamic stiffness, to the inertia at the rigid acceleration of everything the periodic \
modes leave out; it assumes that the modes classified as rigid respond in phase with the ground."),
    ("missing_mass_not_applied", "The missing-mass correction is not applied: every \
extracted mode is periodic, and the mass the periodic modes do not represent is disclosed with its fractions and is not in \
any row."),
    ("inherited_arc_limitation", "Realized bends enter the modal basis with the thin curved-beam stiffness, whose \
application range SWBPIPE's verification has not established (LIM-021, an applicability limitation); this result inherits it \
through the modal basis."),
];

fn fixed_disclosures(choices: &Choices, census: &Census) -> Vec<(&'static str, &'static str)> {
    let mut out = FIXED_DISCLOSURES[..4].to_vec();
    out.push(match choices.missing_mass {
        MissingMass::Applied { .. } => FIXED_DISCLOSURES[4],
        MissingMass::NotApplied => FIXED_DISCLOSURES[5],
    });
    if !census.arcs.is_empty() {
        out.push(FIXED_DISCLOSURES[6]);
    }
    out
}

#[allow(clippy::too_many_lines)]
fn solved(inputs: &DocumentInputs, bindings: &Bindings, choices: &Choices, tables: &[SpectrumTable], c: &Combination,
    disclosures: Disclosures, n: &mut Numbers) -> Value {
    let mut doc = identity(inputs, bindings, n);
    doc.insert("status".into(), json!("solved"));
    let modal = inputs.modal;
    let census = inputs.census;
    let request = &inputs.contributions.combination;

    // The spectrum result's own standing: the modal set's integrity standing, carried.
    let standing = match &modal.standings {
        Some(s) if s.integrity.token == "sensitive" => {
            json!({"token": "sensitive", "reasons": s.integrity.reasons.iter().map(|r| format!("modal set: {r}")).collect::<Vec<_>>()})
        }
        Some(s) => json!({"token": s.integrity.token, "reasons": []}),
        None => json!({"token": "not_assessed", "reasons": [format!("no modal integrity standing is stated: the modal basis is {}'s", modal.attributed_to)]}),
    };
    doc.insert("standing".into(), standing);

    // The request's choices and the spectra used.
    let (rigid_frequency, rigid_rule) = match choices.missing_mass {
        MissingMass::NotApplied => (Value::Null, Value::Null),
        MissingMass::Applied { rigid_frequency_hz, rule } => (n.fixed("rigid_frequency", rigid_frequency_hz, "request/rigid_frequency"), json!(rule.as_str())),
    };
    let directions: Vec<Value> = choices
        .directions
        .iter()
        .map(|(d, s, f)| json!({"direction": d.as_str(), "spectrum_id": s, "scale_factor": n.fixed("scale_factor", *f, "request/scale_factor")}))
        .collect();
    doc.insert("request".into(), json!({
        "modal_combination": choices.modal_rule.as_str(),
        "directional_combination": choices.directional_rule.as_str(),
        "damping_ratio": n.fixed("damping_ratio", request.damping_ratio, "request/damping_ratio"),
        "missing_mass": {"applied": matches!(choices.missing_mass, MissingMass::Applied { .. }), "rigid_frequency": rigid_frequency, "rigid_combination": rigid_rule},
        "directions": directions,
    }));
    let spectra_rows = tables
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let at = format!("spectra/{}", t.id());
            let (abscissa, abscissa_unit) = match t.abscissa() {
                Abscissa::Frequency => ("frequency", UnitId::Hertz),
                Abscissa::Period => ("period", UnitId::Second),
            };
            let (ordinate, ordinate_unit) = match t.ordinate_measure() {
                OrdinateMeasure::MetersPerSecondSquared => ("acceleration", UnitId::MeterPerSecondSquared),
                OrdinateMeasure::GMultiple => ("g_multiple", UnitId::One),
            };
            let points: Vec<Value> = t.knots().iter().zip(t.ordinates()).map(|(x, y)| json!([n.num(*x, &at), n.num(*y, &at)])).collect();
            let g = t.g_m_per_s2().map_or(Value::Null, |g| n.fixed("spectrum_g", g, &at));
            seal(format!("spectrum:{}", i + 1), i, json!({
                "spectrum_id": t.id(),
                "damping_ratio": n.fixed("damping_ratio", t.damping_ratio(), &at),
                "abscissa": abscissa, "abscissa_unit": abscissa_unit.catalog_id(),
                "ordinate": ordinate, "ordinate_unit": ordinate_unit.catalog_id(),
                "pseudo_spectral_acceleration": true,
                "g": g,
                "interpolation": {"abscissa": axis_scale(t.interpolation().abscissa), "ordinate": axis_scale(t.interpolation().ordinate)},
                "points": points,
            }))
        })
        .collect();
    doc.insert("spectra".into(), table(spectra_rows));

    // The modal basis as used.
    let class_of: BTreeMap<ClusterIndex, ClusterClass> = c.clusters.iter().map(|s| (s.cluster, s.class)).collect();
    let mode_rows = request
        .modes
        .iter()
        .enumerate()
        .map(|(i, m)| {
            seal(mode_ref(m.mode.0), i, json!({
                "mode_index": m.mode.0, "cluster_ref": cluster_ref(m.cluster),
                "frequency": n.fixed("modal_frequency", m.frequency_hz, "modes"),
                "circular_frequency": modal.circular_frequency_rad_s.get(i).map_or(Value::Null, |w| n.fixed("modal_circular_frequency", *w, "modes")),
                "class": class_of.get(&m.cluster).map(|k| k.as_str()),
                "integrity_standing": modal.mode_standings.get(i).and_then(Option::as_ref).map_or(Value::Null, standing_json),
            }))
        })
        .collect();
    let cluster_rows = c
        .clusters
        .iter()
        .enumerate()
        .map(|(i, s)| {
            seal(cluster_ref(s.cluster), i, json!({
                "cluster_index": s.cluster.0,
                "mode_refs": s.modes.iter().map(|m| mode_ref(m.0)).collect::<Vec<_>>(),
                "frequencies": n.arr(s.frequencies_hz.iter().copied(), "clusters"), "frequency_unit": UnitId::Hertz.catalog_id(),
                "standing_frequency": n.fixed("cluster_standing_frequency", s.standing_frequency_hz, "clusters"),
                "class": s.class.as_str(),
            }))
        })
        .collect();
    let standings = modal.standings.as_ref().map_or(Value::Null, |s| json!({
        "result_standing": {"status": s.result_status, "public_engineering_result": s.public_engineering_result},
        "parameter_standing": {"parameter_standing": s.parameter_standing, "parameter_label": s.parameter_label},
        "integrity_standing": standing_json(&s.integrity),
    }));
    let e = &modal.extraction;
    let highest = c.clusters.last();
    let (fr, applies) = match choices.missing_mass {
        MissingMass::Applied { rigid_frequency_hz, .. } => (Some(rigid_frequency_hz), true),
        MissingMass::NotApplied => (None, false),
    };
    let extraction = json!({
        "endpoint": e.endpoint,
        "reported_mode_count": n.int(e.reported_mode_count, "extraction"),
        "established_finite_mode_count": e.established_finite_mode_count.map_or(Value::Null, |k| n.int(k, "extraction")),
        "next_computed_frequency": e.next_computed_frequency_hz.map_or(Value::Null, |f| n.fixed("next_computed_frequency", f, "extraction")),
        "next_computed_circular_frequency": e.next_computed_circular_frequency_rad_s
            .map_or(Value::Null, |w| n.fixed("next_computed_circular_frequency", w, "extraction")),
        "completion_evidence": e.evidence,
    });
    doc.insert("modal_basis".into(), json!({
        "attributed_to": modal.attributed_to,
        "standings": standings,
        "state": modal.state.clone().unwrap_or(Value::Null),
        "extraction": extraction,
        "cluster_tolerance": n.fixed("cluster_tolerance", modal.cluster_tolerance, "modal_basis"),
        "modes": table(mode_rows),
        "clusters": table(cluster_rows),
    }));

    // Disclosure operands.
    let range_rows = tables
        .iter()
        .zip(&disclosures.ranges)
        .enumerate()
        .map(|(i, (t, evaluated))| {
            let at = format!("range/{}", t.id());
            let kind = match t.abscissa() {
                Abscissa::Frequency => "spectrum_range_frequency",
                Abscissa::Period => "spectrum_range_period",
            };
            let knots = t.knots();
            let used_by: Vec<&str> = choices.directions.iter().filter(|(_, s, _)| s == t.id()).map(|(d, _, _)| d.as_str()).collect();
            seal(format!("range:{}", i + 1), i, json!({
                "spectrum_id": t.id(), "directions": used_by, "abscissa": match t.abscissa() { Abscissa::Frequency => "frequency", Abscissa::Period => "period" },
                "tabulated_low": n.fixed(kind, knots[0], &at), "tabulated_high": n.fixed(kind, knots[knots.len() - 1], &at),
                "evaluated": evaluated.iter().map(|(what, f, l)| json!({"at": what, "frequency": n.fixed("evaluated_frequency", *f, &at), "location": location_json(*l)})).collect::<Vec<_>>(),
                "in_range": true,
            }))
        })
        .collect();
    let premise = json!({
        "applies": applies,
        "rigid_frequency": fr.map_or(Value::Null, |f| n.fixed("rigid_frequency", f, "premise")),
        "endpoint": e.endpoint,
        "reported_mode_count": n.int(e.reported_mode_count, "premise"),
        "established_finite_mode_count": e.established_finite_mode_count.map_or(Value::Null, |k| n.int(k, "premise")),
        "highest_completed_cluster_ref": highest.map(|s| cluster_ref(s.cluster)),
        "highest_completed_cluster_lowest_frequency": highest.map_or(Value::Null, |s| n.fixed("cluster_standing_frequency", s.standing_frequency_hz, "premise")),
        "next_computed_frequency": e.next_computed_frequency_hz.map_or(Value::Null, |f| n.fixed("next_computed_frequency", f, "premise")),
        "completion_evidence": e.evidence,
        "holds": disclosures.premise_holds,
    });
    let periodic_positions: Vec<usize> = request
        .modes
        .iter()
        .enumerate()
        .filter(|(_, m)| class_of.get(&m.cluster) == Some(&ClusterClass::Periodic))
        .map(|(i, _)| i)
        .collect();
    let mass_rows = c
        .directions
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let k = direction_index(*d);
            let at = format!("missing_mass/{d}");
            let t = modal.total_mass_kg[k];
            // Every operand was checked finite before this stage (check_mass_operands).
            let mut sum = Dyadic::zero();
            let effective: Vec<Value> = periodic_positions
                .iter()
                .map(|&p| {
                    let m = modal.effective_mass_kg.get(p).map_or(0.0, |e| e[k]);
                    sum = sum.add(&Dyadic::from_f64(if m.is_finite() { m } else { 0.0 }));
                    json!({"mode_ref": mode_ref(request.modes[p].mode.0), "effective_mass": n.fixed("modal_effective_mass", m, &at)})
                })
                .collect();
            let s = sum.to_f64();
            let u = Dyadic::from_f64(if t.is_finite() { t } else { 0.0 }).sub(&sum).to_f64();
            let (cumulative, unrepresented) = if t == 0.0 {
                (n.quantity("periodic_cumulative_effective_mass_fraction", None, Some("total_mass_zero"), &at),
                    n.quantity("unrepresented_mass_fraction", None, Some("total_mass_zero"), &at))
            } else {
                (n.fixed("periodic_cumulative_effective_mass_fraction", s / t, &at), n.fixed("unrepresented_mass_fraction", u / t, &at))
            };
            seal(format!("missing-mass:{}", d.as_str()), i, json!({
                "direction": d.as_str(),
                "total_mass": n.fixed("directional_total_mass", t, &at),
                "periodic_effective_masses": effective,
                "periodic_effective_mass_sum": n.fixed("periodic_effective_mass_sum", s, &at),
                "unrepresented_effective_mass": n.fixed("unrepresented_effective_mass", u, &at),
                "periodic_cumulative_fraction": cumulative,
                "unrepresented_fraction": unrepresented,
                "correction_applied": applies,
            }))
        })
        .collect();
    let fixed_rows = fixed_disclosures(choices, census)
        .into_iter()
        .enumerate()
        .map(|(i, (id, text))| seal(format!("disclosure:{id}"), i, json!({"disclosure_id": id, "text": text})))
        .collect();
    doc.insert("disclosures".into(), json!({
        "range": table(range_rows),
        "extraction_premise": premise,
        "missing_mass": {"arithmetic": MASS_FRACTION_ARITHMETIC, "directions": table(mass_rows)},
        "fixed": table(fixed_rows),
    }));

    // The census.
    let nodes = census.nodes.iter().enumerate().map(|(i, id)| seal(format!("node:{}", i + 1), i, json!({"node_id": id}))).collect();
    let straight = census.straight_members.iter().enumerate().map(|(i, m)| {
        let at = "census/straight_members";
        seal(format!("straight-member:{}", i + 1), i, json!({"member_id": m.id, "end_i_node": m.end_i, "end_j_node": m.end_j,
            "reference_vector": n.arr(m.reference_vector, at), "member_axes": n.triad(&m.axes, at)}))
    }).collect();
    let arcs = census.arcs.iter().enumerate().map(|(i, a)| {
        let at = "census/arcs";
        seal(format!("arc:{}", i + 1), i, json!({"member_id": a.id, "end_i_node": a.end_i, "end_j_node": a.end_j,
            "tangent_frame_i": n.triad(&a.tangent_frame_i, at), "tangent_frame_j": n.triad(&a.tangent_frame_j, at)}))
    }).collect();
    let connectors = census.connectors.iter().enumerate().map(|(i, k)| {
        seal(format!("connector:{}", i + 1), i, json!({"connector_id": k.id, "end_i_node": k.end_i, "end_j_node": k.end_j,
            "connector_axes": n.triad(&k.axes, "census/connectors")}))
    }).collect();
    let rods = census.tie_rods.iter().enumerate().map(|(i, r)| {
        seal(format!("tie-rod:{}", i + 1), i, json!({"connector_id": r.connector, "rod_id": r.rod, "rod_axis": n.arr(r.axis, "census/tie_rods")}))
    }).collect();
    let constraints = census.rigid_constraints.iter().enumerate().map(|(i, k)| {
        seal(format!("rigid-constraint:{}", i + 1), i, json!({"node_id": k.node, "component": k.component.as_str(), "holders": k.holders,
            "held": held(&k.holders)}))
    }).collect();
    let compliant = census.compliant_supports.iter().enumerate().map(|(i, s)| {
        seal(format!("compliant-support:{}", i + 1), i, json!({"support_id": s.support, "node_id": s.node,
            "components": s.components.iter().map(|c| c.as_str()).collect::<Vec<_>>()}))
    }).collect();
    let no_row = census.supports_without_rows.iter().enumerate().map(|(i, s)| {
        seal(format!("support-without-row:{}", i + 1), i, json!({"support_id": s.support, "reason": s.reason}))
    }).collect();
    doc.insert("census".into(), json!({
        "nodes": table(nodes), "straight_members": table(straight), "arcs": table(arcs), "connectors": table(connectors),
        "tie_rods": table(rods), "rigid_constraints": table(constraints), "compliant_supports": table(compliant),
        "supports_without_rows": table(no_row),
    }));

    // The rows.
    let constraints: BTreeMap<(&str, ReactionComponent), &[String]> =
        census.rigid_constraints.iter().map(|k| ((k.node.as_str(), k.component), k.holders.as_slice())).collect();
    doc.insert("rows".into(), rows_json(request, c, &periodic_positions, &constraints, n));

    // The rule statements.
    let correlation = c.correlation.as_ref().map(|pairs| {
        table(pairs.iter().enumerate().map(|(i, p)| seal(format!("correlation:{}|{}", cluster_ref(p.first), cluster_ref(p.second)), i, json!({
            "first_cluster_ref": cluster_ref(p.first), "second_cluster_ref": cluster_ref(p.second),
            "rho": n.fixed("correlation_coefficient", p.rho, "correlation"), "status": "informational",
        }))).collect())
    });
    let groups = c.groups.as_ref().map(|g| {
        table(g.iter().enumerate().map(|(i, group)| seal(format!("group:{}", i + 1), i, json!({
            "cluster_refs": group.iter().map(|k| cluster_ref(*k)).collect::<Vec<_>>()}))).collect())
    });
    let pairs = c.close_pairs.as_ref().map(|p| {
        table(p.iter().enumerate().map(|(i, (a, b))| seal(format!("coupled-pair:{}|{}", cluster_ref(*a), cluster_ref(*b)), i, json!({
            "first_cluster_ref": cluster_ref(*a), "second_cluster_ref": cluster_ref(*b)}))).collect())
    });
    doc.insert("rule_statements".into(), json!({
        "correlation": correlation.unwrap_or(Value::Null),
        "groups": groups.unwrap_or(Value::Null),
        "coupled_pairs": pairs.unwrap_or(Value::Null),
        "closeness_rule": closeness_rule(c.modal_rule),
    }));
    Value::Object(doc)
}

/// The grouping method's closeness rule, as the result states it (REQ-003).
pub const GROUPING_CLOSENESS_RULE: &str = "Periodic clusters in ascending order form groups. A group starts at a cluster, \
whose standing frequency f_g is the group's lowest, and takes each next cluster whose standing frequency f satisfies \
(f − f_g) / f_g ≤ 1/10; the next cluster that does not starts a new group. The comparison is decided exactly on the \
represented frequencies.";

/// The ten percent method's closeness rule, as the result states it (REQ-003).
pub const TEN_PERCENT_CLOSENESS_RULE: &str = "Two periodic clusters i < j are coupled when their standing frequencies satisfy \
(f_j − f_i) / f_i ≤ 1/10, whether or not they fall in one group of the grouping method. The comparison is decided exactly \
on the represented frequencies.";

fn closeness_rule(rule: ModalRule) -> Value {
    match rule {
        ModalRule::Grouping => json!(GROUPING_CLOSENESS_RULE),
        ModalRule::TenPercent => json!(TEN_PERCENT_CLOSENESS_RULE),
        ModalRule::Srss | ModalRule::Cqc | ModalRule::AbsoluteSum => Value::Null,
    }
}

/// The rows: the column statements, then one signed record, one cluster-sum record (where a
/// periodic cluster has more than one mode) and one combined record per response quantity, in
/// the roster's order.
fn rows_json(request: &CombinationRequest, c: &Combination, periodic_positions: &[usize],
    constraints: &BTreeMap<(&str, ReactionComponent), &[String]>, n: &mut Numbers) -> Value {
    let directions = &c.directions;
    // Contribution columns: the periodic modes in ascending order.
    let columns = periodic_positions
        .iter()
        .enumerate()
        .map(|(i, &p)| {
            let m = &request.modes[p];
            seal(format!("contribution-column:{}", i + 1), i, json!({"column": i, "mode_ref": mode_ref(m.mode.0), "cluster_ref": cluster_ref(m.cluster)}))
        })
        .collect();
    // Cluster-sum columns: the periodic clusters of more than one mode, in one walk of the
    // classified clusters (the clusters `combine` states sums for).
    let sum_clusters: Vec<&ClusterStatement> =
        c.clusters.iter().filter(|s| s.class == ClusterClass::Periodic && s.modes.len() > 1).collect();
    let sum_columns = sum_clusters
        .iter()
        .enumerate()
        .map(|(i, s)| {
            seal(format!("cluster-sum-column:{}", i + 1), i, json!({"column": i, "cluster_ref": cluster_ref(s.cluster),
                "mode_refs": s.modes.iter().map(|m| mode_ref(m.0)).collect::<Vec<_>>()}))
        })
        .collect();
    // Index of each supplied contribution, by (direction, mode position).
    let position_of: BTreeMap<u32, usize> = request.modes.iter().enumerate().map(|(i, m)| (m.mode.0, i)).collect();
    let mut periodic: BTreeMap<(Direction, usize), &[f64]> = BTreeMap::new();
    for p in &request.periodic {
        if let Some(&pos) = position_of.get(&p.mode.0) {
            periodic.insert((p.direction, pos), &p.values);
        }
    }
    let rigid: BTreeMap<Direction, &[f64]> = request.rigid.iter().map(|r| (r.direction, r.values.as_slice())).collect();
    let sums: BTreeMap<(ClusterIndex, Direction), &[f64]> = c.cluster_sums.iter().map(|s| ((s.cluster, s.direction), s.values.as_slice())).collect();

    let mut signed = Vec::with_capacity(request.rows.len());
    let mut cluster_sums = Vec::new();
    let mut combined = Vec::with_capacity(request.rows.len());
    for (r, q) in request.rows.iter().enumerate() {
        let quantity_ref = format!("quantity:{}", r + 1);
        let at = format!("rows/{quantity_ref}");
        let unit = q.unit().catalog_id();
        let frame = q.frame().as_str();
        let mut fields = identity_fields(q, constraints);
        fields.insert("unit".into(), json!(unit));
        fields.insert("frame".into(), json!(frame));
        fields.insert("sign_status".into(), json!(SignStatus::SignedContribution.as_str()));
        fields.insert("sign_convention".into(), json!(q.sign_convention().as_str()));
        let mut per = Map::new();
        for d in directions {
            per.insert(d.as_str().into(), n.arr(periodic_positions.iter()
                .map(|&p| periodic.get(&(*d, p)).and_then(|v| v.get(r)).copied().unwrap_or(f64::NAN)), &at));
        }
        fields.insert("periodic".into(), Value::Object(per));
        let rigid_values = (!rigid.is_empty()).then(|| {
            let mut m = Map::new();
            for d in directions {
                m.insert(d.as_str().into(), n.num(rigid.get(d).and_then(|v| v.get(r)).copied().unwrap_or(f64::NAN), &at));
            }
            Value::Object(m)
        });
        fields.insert("rigid".into(), rigid_values.unwrap_or(Value::Null));
        signed.push(seal(quantity_ref.clone(), r, Value::Object(fields)));

        if !sum_clusters.is_empty() {
            let mut s = Map::new();
            for d in directions {
                s.insert(d.as_str().into(), n.arr(sum_clusters.iter()
                    .map(|k| sums.get(&(k.cluster, *d)).and_then(|v| v.get(r)).copied().unwrap_or(f64::NAN)), &at));
            }
            cluster_sums.push(seal(format!("cluster-sum:{}", r + 1), r, json!({
                "quantity_ref": quantity_ref, "unit": unit, "frame": frame,
                "sign_status": SignStatus::SignedClusterSum.as_str(), "sign_convention": q.sign_convention().as_str(),
                "sums": Value::Object(s),
            })));
        }

        let Some(row) = c.rows.get(r) else { continue };
        let mut fields = identity_fields(q, constraints);
        fields.insert("quantity_ref".into(), json!(quantity_ref));
        fields.insert("unit".into(), json!(unit));
        fields.insert("frame".into(), json!(frame));
        fields.insert("sign_status".into(), json!(SignStatus::UnsignedCombinedMagnitude.as_str()));
        let per_direction: Vec<Value> = row
            .per_direction
            .iter()
            .map(|p| json!({
                "direction": p.direction.as_str(),
                "periodic": n.num(p.periodic.value(), &at),
                "rigid": p.rigid.map_or(Value::Null, |m| n.num(m.value(), &at)),
                "combined": n.num(p.combined.value(), &at),
            }))
            .collect();
        fields.insert("per_direction".into(), Value::Array(per_direction));
        fields.insert("combined".into(), n.num(row.combined.value(), &at));
        fields.insert("governing_direction".into(), json!(row.governing_direction.map(|d| d.as_str())));
        combined.push(seal(format!("combined:{}", r + 1), r, Value::Object(fields)));
    }
    json!({
        "directions": directions.iter().map(|d| d.as_str()).collect::<Vec<_>>(),
        "contribution_columns": table(columns),
        "cluster_sum_columns": table(sum_columns),
        "signed": table(signed),
        "cluster_sums": table(cluster_sums),
        "combined": table(combined),
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    const IDENTITY: Triad = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

    fn connector(id: &str) -> Connector {
        Connector { id: id.into(), end_i: "n".into(), end_j: "n".into(), axes: IDENTITY }
    }

    fn rod(connector: &str, rod: &str) -> TieRod {
        TieRod { connector: connector.into(), rod: rod.into(), axis: [1.0, 0.0, 0.0] }
    }

    /// Two tie rods whose composed labels are equal, (rod "a of connector b", connector "c") and
    /// (rod "a", connector "b of connector c"), are distinct census entries; the same rod twice
    /// is a repeat.
    #[test]
    fn census_entries_are_compared_by_their_identity_fields() {
        let mut census = Census {
            nodes: vec!["n".into()],
            connectors: vec![connector("c"), connector("b of connector c")],
            tie_rods: vec![rod("c", "a of connector b"), rod("b of connector c", "a")],
            ..Census::default()
        };
        let mut refusals = Vec::new();
        check_census_entries(&census, &mut refusals);
        assert!(refusals.is_empty(), "{refusals:?}");
        census.tie_rods.push(rod("c", "a of connector b"));
        check_census_entries(&census, &mut refusals);
        assert_eq!(refusals.len(), 1, "{refusals:?}");
        assert_eq!((refusals[0].object.as_str(), refusals[0].field.as_deref()), ("census", Some("tie_rods")));
    }
}
