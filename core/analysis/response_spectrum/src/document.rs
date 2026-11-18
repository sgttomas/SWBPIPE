//! The spectrum result document (DEL-04-10 REQ-006): family `openpipestress.spectrum_result`,
//! profile `product_preview_spectrum_v1`, semantic table `openpipestress.result_semantics/spectrum-1`,
//! outside the `results.v0.3` envelope.
//!
//! [`spectrum_result_document`] is a pure function from structured inputs to the document value:
//! the model document (which holds the spectra) and the spectrum request it binds, the statement of
//! the modal basis the result rests on, the census of the built dynamic model and the signed
//! contributions. It runs the combinations ([`crate::combine`]) and publishes either the solved
//! document or the refused one, with every refusal found. It forms no modal response and no rigid
//! response: the signed contributions are its inputs.
//!
//! **Conventions shared with the modal result family.** An identity block on every document; the
//! checked canonical profile `openpipestress_jcs_ijson_v1` for every hash, each the SHA-256 of a
//! value's canonical text; sealed rows (`row_id`, `ordinal`, `checksum`) in published tables with a
//! row count and a digest over the rows' checksums; scalar values as value objects with catalog
//! units; fixed limitation text; a committed semantic table named by id and SHA-256; a refused form
//! with a sealed refusals table. The document states that it is experimental and not a public
//! engineering result.
//!
//! **Allocation.** With Q response quantities, M extracted modes (P of them periodic), C clusters,
//! D ≤ 3 requested directions, S spectra of K knots in all, and E census entries, the document holds
//! O(Q·D·(P + 1) + Q·D·C₂ + M + C + K + E + P²) values, where C₂ ≤ C is the number of periodic
//! clusters of more than one mode and the P² term is the CQC coefficients of the periodic cluster
//! pairs. Beyond the document itself the assembler allocates: the combinations' own working set
//! (`combine`); one ordered map of Q row identities for the roster check; the admitted tables of
//! the S spectra used; and, transiently, the canonical text of one row at a time for its checksum
//! and of the model and the request once each for their bindings (each bounded by that input's
//! size). Every contribution value is copied once, from the supplied contributions into its row;
//! nothing recurses on the input's depth beyond the canonical profile's own check.
use crate::combine::combine;
use crate::contract::{
    ClusterClass, ClusterIndex, Combination, CombinationRequest, Direction, DirectionalRule, End, Frame, MemberFamily,
    MissingMass, ModalRule, Quantity, ReactionComponent, RigidRule, RowIdentity, SignConvention, SignStatus,
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
    "A reader of this document checks that it is consistent with itself and with the inputs it is given; that does not \
show that the census is the model's, that a stated frame is the entity's, that the modal basis is an eigenbasis of the \
model, that a signed contribution was recovered correctly, or that the product produced the document.",
];

/// The arithmetic of the two mass fractions of the missing-mass disclosure, stated in the
/// document so that a reader can repeat it bit for bit.
pub const MASS_FRACTION_ARITHMETIC: &str = "S is the exact sum of the stated binary64 effective masses of the periodic \
modes, rounded once to the nearest binary64 (ties to even). The cumulative fraction is the binary64 quotient S / T, \
rounded to nearest. The unrepresented fraction is U / T, rounded to nearest, where U is the exact difference T - S \
rounded once to the nearest binary64 (ties to even). T is the stated total mass of the direction, which includes the \
mass that moves with the supports. Where T is zero both fractions are undefined.";

const PROFILE_LIMIT: f64 = 9_007_199_254_740_992.0;

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
    /// The rows, modal metadata and signed contributions, under the request's choices.
    pub combination: &'a CombinationRequest,
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
                    // The static result's own row-kind token is not stated in REQ-006 or this
                    // unit's sources; the wording above is REQ-006's. Marked for the assigner.
                    "static_row_kind_token": Value::Null,
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
        "static_row_kind_tokens": "not stated: each frame names the static row of the same physical quantity in REQ-006's words; the static result's own row-kind token is left null for the assigner to state",
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

/// The SHA-256 (lowercase hex) of a value's checked canonical text; `None` outside the profile.
pub fn spectrum_doc_sha256(value: &Value) -> Option<String> {
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
                    format!("{v:e} is outside the checked canonical profile (an integral binary64 of magnitude 2^53 or more)")));
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

/// A row's identity fields (no unit, frame or sign fields).
fn identity_fields(q: &Quantity) -> Map<String, Value> {
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
        Quantity::ConstraintReaction { node, holders, .. } => {
            m.insert("node".into(), json!(node));
            m.insert("holders".into(), json!(holders));
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
                        if directions.iter().any(|(e, _, _)| *e == direction) {
                            refusals.push(Refusal::new(RefusalCode::DirectionInvalid, &at, Some("direction"), format!("direction {direction} is requested more than once")));
                        }
                        directions.push((direction, spectrum.to_string(), scale.value()));
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

/// The spectra the request's directions name, admitted from the model's `spectra[]`, each once,
/// in order of first use.
fn read_spectra(model: &Value, choices: &Choices, refusals: &mut Vec<Refusal>) -> Vec<SpectrumTable> {
    let records = model.get("spectra").and_then(Value::as_array);
    let mut out: Vec<SpectrumTable> = Vec::new();
    for (direction, id, _) in &choices.directions {
        if out.iter().any(|t| t.id() == id) {
            continue;
        }
        let found = records.and_then(|r| r.iter().find(|s| s.get("id").and_then(Value::as_str) == Some(id.as_str())));
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

fn origin_json(origin: &Origin) -> Value {
    match origin {
        Origin::SpectrumSolve => json!({"kind": "spectrum_solve", "produced_by_spectrum_solve": true, "reference": null,
            "constructed": [], "statement": "Produced by the product's experimental spectrum solve."}),
        Origin::ConstructedFromReference { package, case, analysis, constructed } => json!({
            "kind": "constructed_from_reference",
            "produced_by_spectrum_solve": false,
            "reference": {"package": package, "case": case, "analysis": analysis},
            "constructed": constructed,
            "statement": "Constructed from the signed contributions of a frozen reference, not produced by a spectrum solve; \
its combined magnitudes are formed from those contributions by the product's combinations. It shows reader agreement \
only and is not evidence that the product recovers these contributions.",
        }),
    }
}

fn binding_json(payload: &str, value: &Value, numbers: &mut Numbers) -> Value {
    let sha = spectrum_doc_sha256(value);
    if sha.is_none() {
        numbers.outside.push(Refusal::new(RefusalCode::DocumentOutsideCanonicalProfile, payload, None,
            format!("the {payload} is outside the checked canonical profile, so it cannot be bound")));
    }
    json!({"payload": payload, "hash_algorithm": SPECTRUM_HASH_ALGORITHM, "canonical_form": SPECTRUM_RESULT_CANONICAL, "sha256": sha})
}

fn modal_result_json(b: &ModalResultBinding) -> Value {
    match b {
        ModalResultBinding::Bound { run_id, request_sha256, model_sha256, materials_sha256, integrity_parameters_sha256, state_case } => json!({
            "binding": "bound", "reason": null, "run_id": run_id, "request_sha256": request_sha256, "model_sha256": model_sha256,
            "materials_sha256": materials_sha256, "integrity_parameters_sha256": integrity_parameters_sha256,
            "state_case": state_case.as_ref().map(|(id, index, sha)| json!({"case_id": id, "case_index": index, "case_sha256": sha})),
        }),
        ModalResultBinding::NotApplicable { reason } => json!({
            "binding": "not_applicable", "reason": reason, "run_id": null, "request_sha256": null, "model_sha256": null,
            "materials_sha256": null, "integrity_parameters_sha256": null, "state_case": null,
        }),
    }
}

fn identity(inputs: &DocumentInputs, numbers: &mut Numbers) -> Map<String, Value> {
    let mut doc = Map::new();
    doc.insert("document_kind".into(), json!(SPECTRUM_RESULT_KIND));
    doc.insert("schema_version".into(), json!(SPECTRUM_RESULT_VERSION));
    doc.insert("semantic_contract".into(), json!({"id": SPECTRUM_RESULT_SEMANTICS, "sha256": spectrum_doc_sha256(&spectrum_semantic_table())}));
    doc.insert("profile".into(), json!({"profile_id": SPECTRUM_RESULT_PROFILE, "limitations": SPECTRUM_RESULT_LIMITATIONS}));
    doc.insert("canonical_profile".into(), json!(SPECTRUM_RESULT_CANONICAL));
    doc.insert("admission".into(), json!({"status": "experimental", "public_engineering_result": false}));
    doc.insert("origin".into(), origin_json(inputs.origin));
    doc.insert("bindings".into(), json!({
        "model": binding_json("model_document", inputs.model, numbers),
        "request": binding_json("spectrum_request", inputs.request, numbers),
    }));
    doc.insert("modal_result".into(), modal_result_json(inputs.modal_result));
    doc
}

fn refused(inputs: &DocumentInputs, refusals: &[Refusal]) -> Value {
    let mut numbers = Numbers { outside: Vec::new() };
    let mut doc = identity(inputs, &mut numbers);
    doc.insert("status".into(), json!("refused"));
    let rows = refusals
        .iter()
        .chain(numbers.outside.iter())
        .enumerate()
        .map(|(i, r)| seal(format!("refusal:{}", i + 1), i, json!({"diagnostic_id": format!("diagnostic:spectrum:refusal:{}", i + 1),
            "code": r.code.as_str(), "severity": "blocking", "object": r.object, "field": r.field, "detail": r.detail})))
        .collect();
    doc.insert("refusals".into(), table(rows));
    Value::Object(doc)
}

/// The checks of the inputs against one another that the combinations do not make.
fn check_agreement(inputs: &DocumentInputs, choices: &Choices, tables: &[SpectrumTable], refusals: &mut Vec<Refusal>) {
    let c = inputs.combination;
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
    let m = inputs.modal;
    if m.mode_standings.len() != c.modes.len() || m.effective_mass_kg.len() != c.modes.len() {
        refusals.push(input_invalid("modal_basis", "modes", "the modal statement does not state every extracted mode".into()));
    }
    if m.extraction.reported_mode_count != c.modes.len() {
        refusals.push(input_invalid("modal_basis", "extraction", format!("the modal endpoint reports {} modes; {} are supplied",
            m.extraction.reported_mode_count, c.modes.len())));
    }
    check_cluster_rule(inputs, refusals);
    check_census_closed(inputs.census, refusals);
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
    let (m, modes) = (inputs.modal, &inputs.combination.modes);
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
        refusals.push(input_invalid("modal_basis", "extraction", "the next computed mode's frequency and circular frequency are not stated together".into()));
        return;
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
/// holders of each constraint and the census's family of each member.
fn check_roster(census: &Census, rows: &[Quantity], refusals: &mut Vec<Refusal>) {
    let expected = census.roster();
    let mut by_identity: BTreeMap<RowIdentity, &Quantity> = BTreeMap::new();
    for q in &expected {
        if by_identity.insert(q.identity(), q).is_some() {
            refusals.push(input_invalid("census", "roster", format!("the census gives {} more than once", q.label())));
        }
    }
    let mut seen = BTreeSet::new();
    for q in rows {
        let id = q.identity();
        match by_identity.get(&id) {
            None => refusals.push(input_invalid(&q.label(), "roster", "the row is not in the roster the census gives".into())),
            Some(e) if *e != q => refusals.push(input_invalid(&q.label(), "roster",
                "the row's member family or constraint holders are not the census's".into())),
            Some(_) => {
                seen.insert(id);
            }
        }
    }
    for q in &expected {
        if !seen.contains(&q.identity()) {
            refusals.push(input_invalid(&q.label(), "roster", "the census gives this row and no row is supplied for it".into()));
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

/// Assembles the spectrum result document of `inputs` (module documentation).
pub fn spectrum_result_document(inputs: &DocumentInputs) -> Value {
    let mut refusals = Vec::new();
    let choices = read_choices(inputs.request, &mut refusals);
    let tables = match &choices {
        Some(ch) => read_spectra(inputs.model, ch, &mut refusals),
        None => Vec::new(),
    };
    if let Some(ch) = &choices {
        if refusals.is_empty() {
            check_agreement(inputs, ch, &tables, &mut refusals);
        }
    }
    if !refusals.is_empty() {
        return refused(inputs, &refusals);
    }
    let choices = choices.expect("read");
    let combination = match combine(inputs.combination) {
        Ok(c) => c,
        Err(r) => return refused(inputs, &r),
    };
    check_roster(inputs.census, &inputs.combination.rows, &mut refusals);
    let disclosures = disclosure_operands(inputs, &choices, &tables, &combination, &mut refusals);
    if !refusals.is_empty() {
        return refused(inputs, &refusals);
    }
    let mut numbers = Numbers { outside: Vec::new() };
    let doc = solved(inputs, &choices, &tables, &combination, disclosures, &mut numbers);
    if !numbers.outside.is_empty() {
        return refused(inputs, &numbers.outside);
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

fn disclosure_operands(inputs: &DocumentInputs, choices: &Choices, tables: &[SpectrumTable], c: &Combination,
    refusals: &mut Vec<Refusal>) -> Disclosures {
    let periodic: Vec<(u32, f64)> = c
        .clusters
        .iter()
        .filter(|s| s.class == ClusterClass::Periodic)
        .flat_map(|s| s.modes.iter().zip(&s.frequencies_hz).map(|(m, f)| (m.0, *f)))
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
            let highest = c.clusters.last().map(|s| s.standing_frequency_hz);
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

fn fixed_disclosures(choices: &Choices, census: &Census) -> Vec<(&'static str, &'static str)> {
    let mut out = vec![
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
    ];
    match choices.missing_mass {
        MissingMass::Applied { .. } => out.push(("rigid_response_premise", "The missing-mass correction is applied. The rigid \
response is the static response, on the dynamic stiffness, to the inertia at the rigid acceleration of everything the periodic \
modes leave out; it assumes that the modes classified as rigid respond in phase with the ground.")),
        MissingMass::NotApplied => out.push(("missing_mass_not_applied", "The missing-mass correction is not applied: every \
extracted mode is periodic, and the mass the periodic modes do not represent is disclosed with its fractions and is not in \
any row.")),
    }
    if !census.arcs.is_empty() {
        out.push(("inherited_arc_limitation", "Realized bends enter the modal basis with the thin curved-beam stiffness, whose \
application range SWBPIPE's verification has not established (LIM-021, an applicability limitation); this result inherits it \
through the modal basis."));
    }
    out
}

#[allow(clippy::too_many_lines)]
fn solved(inputs: &DocumentInputs, choices: &Choices, tables: &[SpectrumTable], c: &Combination, disclosures: Disclosures,
    n: &mut Numbers) -> Value {
    let mut numbers_scratch = Numbers { outside: Vec::new() };
    let mut doc = identity(inputs, &mut numbers_scratch);
    n.outside.extend(numbers_scratch.outside);
    doc.insert("status".into(), json!("solved"));
    let modal = inputs.modal;
    let census = inputs.census;
    let request = inputs.combination;

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
        "reported_mode_count": e.reported_mode_count,
        "established_finite_mode_count": e.established_finite_mode_count,
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
        "reported_mode_count": e.reported_mode_count,
        "established_finite_mode_count": e.established_finite_mode_count,
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
            let mut sum = Dyadic::zero();
            let effective: Vec<Value> = periodic_positions
                .iter()
                .map(|&p| {
                    let m = modal.effective_mass_kg.get(p).map_or(f64::NAN, |e| e[k]);
                    sum = sum.add(&Dyadic::from_f64(if m.is_finite() { m } else { 0.0 }));
                    json!({"mode_ref": mode_ref(request.modes[p].mode.0), "effective_mass": n.fixed("modal_effective_mass", m, &at)})
                })
                .collect();
            let s = sum.to_f64();
            let u = Dyadic::from_f64(t).sub(&sum).to_f64();
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
        seal(format!("rigid-constraint:{}", i + 1), i, json!({"node_id": k.node, "component": k.component.as_str(), "holders": k.holders}))
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
    doc.insert("rows".into(), rows_json(request, c, &periodic_positions, n));

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
    }));
    Value::Object(doc)
}

/// The rows: the column statements, then one signed record, one cluster-sum record (where a
/// periodic cluster has more than one mode) and one combined record per response quantity, in
/// the roster's order.
fn rows_json(request: &CombinationRequest, c: &Combination, periodic_positions: &[usize], n: &mut Numbers) -> Value {
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
    let sum_clusters: Vec<ClusterIndex> = {
        let mut v: Vec<ClusterIndex> = c.cluster_sums.iter().map(|s| s.cluster).collect();
        v.sort();
        v.dedup();
        v
    };
    let sum_columns = sum_clusters
        .iter()
        .enumerate()
        .map(|(i, k)| {
            let modes = c.clusters.iter().find(|s| s.cluster == *k).map(|s| s.modes.iter().map(|m| mode_ref(m.0)).collect::<Vec<_>>()).unwrap_or_default();
            seal(format!("cluster-sum-column:{}", i + 1), i, json!({"column": i, "cluster_ref": cluster_ref(*k), "mode_refs": modes}))
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
        let mut fields = identity_fields(q);
        fields.insert("unit".into(), json!(unit));
        fields.insert("frame".into(), json!(frame));
        fields.insert("sign_status".into(), json!(SignStatus::SignedContribution.as_str()));
        fields.insert("sign_convention".into(), json!(q.sign_convention().as_str()));
        let mut per = Map::new();
        for d in directions {
            per.insert(d.as_str().into(), n.arr(periodic_positions.iter().map(|&p| periodic.get(&(*d, p)).map_or(0.0, |v| v[r])), &at));
        }
        fields.insert("periodic".into(), Value::Object(per));
        let rigid_values = (!rigid.is_empty()).then(|| {
            let mut m = Map::new();
            for d in directions {
                m.insert(d.as_str().into(), n.num(rigid[d][r], &at));
            }
            Value::Object(m)
        });
        fields.insert("rigid".into(), rigid_values.unwrap_or(Value::Null));
        signed.push(seal(quantity_ref.clone(), r, Value::Object(fields)));

        if !sum_clusters.is_empty() {
            let mut s = Map::new();
            for d in directions {
                s.insert(d.as_str().into(), n.arr(sum_clusters.iter().map(|k| sums[&(*k, *d)][r]), &at));
            }
            cluster_sums.push(seal(format!("cluster-sum:{}", r + 1), r, json!({
                "quantity_ref": quantity_ref, "unit": unit, "frame": frame,
                "sign_status": SignStatus::SignedClusterSum.as_str(), "sign_convention": q.sign_convention().as_str(),
                "sums": Value::Object(s),
            })));
        }

        let row = &c.rows[r];
        let mut fields = identity_fields(q);
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

