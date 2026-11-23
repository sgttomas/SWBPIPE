//! The signed-contribution contract of DEL-04-10's combinations (unit S4), corrected to the row
//! contract of REQ-006.
//!
//! These are the types through which a producer of signed per-mode, per-direction response
//! values (the modal-response recovery, a later unit) hands them to the combinations, and
//! through which the combinations hand back unsigned combined magnitudes. The spectrum result
//! document (`crate::document`) carries the same identities, units, frames, sign conventions and
//! sign statuses. Nothing here carries standing as an engineering result.
//!
//! **What is supplied, and by whom.** Every value entering a combination is supplied:
//! - the modal metadata of each extracted mode, as DEL-04-09's modal result publishes it: its
//!   `mode_index`, its cluster (`cluster:N`) and its frequency f in Hz as represented in binary64
//!   (CLM-001, Repeated modes). The combinations infer no cluster and apply no cluster tolerance;
//! - per mode of a periodic cluster and per requested direction, the signed contribution R_i of
//!   every row, already scaled by the direction's scale factor (CLM-001, Modal response);
//! - when the missing-mass correction is applied, per requested direction the signed rigid
//!   response of every row, with the clusters its load left out as periodic (CLM-001, Rigid
//!   response).
//!
//! **The rows** are REQ-006's roster ([`Quantity`]): a node's displacement or rotation; a straight
//! member's end action in global and member axes; a realized arc's end action in global axes and
//! its arc tangent frame; a connector's six generalized forces and its end actions; a tie rod's
//! axial force; a rigid constraint's reaction, naming every support that holds it; and a compliant
//! support's force or moment on each global component. A row is identified by its fields
//! ([`RowIdentity`]), never by a composed key. Nothing here states a restraint along a skew
//! direction: those rows are not stated (REQ-006).
//!
//! **Frames and signs.** Every row states one of the five frames of [`Frame`]. Every signed row (a
//! contribution or a cluster sum) states its [`SignConvention`], the static result's convention of
//! the same quantity (REQ-006, "Frame and sign"); a combined row states its frame and that it is an
//! unsigned combined magnitude, and carries no sign convention. A combined magnitude belongs to
//! the frame its contributions were supplied in: a contribution is rotated into its row's frame
//! before it is combined, and a combination of member-axis contributions is not a rotation of a
//! global-axis combination.
//!
//! Units are SI entries of the unit catalog fixed by the row's component ([`RowUnit`]): metre,
//! radian, newton and newton metre. No unit is converted here.
use open_pipe_stress_units::UnitId;
use std::fmt;

/// The fixed statement of every combination output: what the numbers are, and what they are not.
pub const COMBINATION_STATEMENT: &str = "Unsigned combined magnitudes formed by the stated modal, rigid-response and \
directional combination rules from the supplied signed contributions, summing the modes of each supplied cluster \
with their signs first. Under an absolute-sum rule a magnitude is the sum of the magnitudes it combines. The \
contributions and their modal metadata are inputs to this unit; it forms no modal response, solves no eigenproblem \
or rigid response, and is not a response-spectrum analysis result. It establishes neither physical correctness nor \
code compliance.";

/// A global direction of support motion (REQ-001, Directions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    X,
    Y,
    Z,
}

impl Direction {
    pub const ALL: [Direction; 3] = [Direction::X, Direction::Y, Direction::Z];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::X => "X",
            Self::Y => "Y",
            Self::Z => "Z",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|d| d.as_str() == text)
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A mode's `mode_index` in the modal result (1-based, ascending in frequency).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModeIndex(pub u32);

impl fmt::Display for ModeIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mode:{}", self.0)
    }
}

/// A cluster's index in the modal result (`cluster:N`, 1-based, chained in ascending order).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClusterIndex(pub u32);

impl fmt::Display for ClusterIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cluster:{}", self.0)
    }
}

/// One extracted mode, as the modal result publishes it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModalMode {
    pub mode: ModeIndex,
    pub cluster: ClusterIndex,
    /// f in Hz, the represented binary64 the modal result publishes.
    pub frequency_hz: f64,
}

/// The component of a node displacement or rotation, in global axes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DisplacementComponent {
    Ux,
    Uy,
    Uz,
    Rx,
    Ry,
    Rz,
}

/// The component of an end action: three forces and three moments along the axes of the
/// row's frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ForceComponent {
    Fx,
    Fy,
    Fz,
    Mx,
    My,
    Mz,
}

/// The global component of a support reaction or a compliant support's force or moment: the
/// degree of freedom it acts on, a force for UX, UY, UZ and a moment about the node for RX, RY,
/// RZ. There is no component along a skew direction: those rows are not stated (REQ-006, "Not
/// yet stated: a restraint along a skew direction").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ReactionComponent {
    Ux,
    Uy,
    Uz,
    Rx,
    Ry,
    Rz,
}

/// A connector's generalized force, in its own axes, work-conjugate to its relative
/// displacements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConnectorComponent {
    Axial,
    LateralY,
    LateralZ,
    Torsion,
    BendingY,
    BendingZ,
}

/// An end: node i or node j of a member, an arc or a connector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum End {
    I,
    J,
}

/// The frame of a row (REQ-006, "Frame and sign"): the spectrum result family's own names,
/// not the static result's coordinate-system values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Frame {
    /// Global axes.
    GlobalAxes,
    /// A straight member's axes: x from end i to end j, y the part of the member's reference
    /// vector normal to x, normalized, z = x × y.
    MemberAxes,
    /// The tangent frame of an arc at the row's end, the static result's (DEL-04-01/REQ-018).
    ArcTangentFrame,
    /// A connector's own axes.
    ConnectorAxes,
    /// A tie rod's axis, from its attachment at end i to its attachment at end j.
    RodAxis,
}

impl Frame {
    pub const ALL: [Frame; 5] = [Self::GlobalAxes, Self::MemberAxes, Self::ArcTangentFrame, Self::ConnectorAxes, Self::RodAxis];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GlobalAxes => "global_axes",
            Self::MemberAxes => "member_axes",
            Self::ArcTangentFrame => "arc_tangent_frame",
            Self::ConnectorAxes => "connector_axes",
            Self::RodAxis => "rod_axis",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|f| f.as_str() == text)
    }
}

/// Whether a member of the dynamic stiffness is a straight member or a realized arc. It is not
/// part of a row's identity (the entity of either is the member); it decides which frames the
/// member's end rows may state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MemberFamily {
    Straight,
    Arc,
}

impl MemberFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Straight => "straight",
            Self::Arc => "arc",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        [Self::Straight, Self::Arc].into_iter().find(|f| f.as_str() == text)
    }
}

/// The SI unit of a row, an entry of the unit catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RowUnit {
    Meter,
    Radian,
    Newton,
    NewtonMeter,
}

impl RowUnit {
    /// The catalog entry.
    pub const fn unit_id(self) -> UnitId {
        match self {
            Self::Meter => UnitId::Meter,
            Self::Radian => UnitId::Radian,
            Self::Newton => UnitId::Newton,
            Self::NewtonMeter => UnitId::NewtonMeter,
        }
    }

    /// The catalog id, `unit:…`.
    pub fn catalog_id(self) -> &'static str {
        self.unit_id().catalog_id()
    }

    /// The catalog symbol.
    pub fn symbol(self) -> &'static str {
        self.unit_id().definition().symbol
    }
}

/// The sign convention of a signed row: a contribution or a stated cluster sum (REQ-006, "Frame
/// and sign", "As the static result publishes them"). A combined row has none.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SignConvention {
    /// A node displacement or rotation: positive along the global axis.
    PositiveAlongAxis,
    /// An end action of a member, an arc or a connector, in the row's frame: the generalized force
    /// acting on the element at its end node.
    ActionOnElementAtEndNode,
    /// A connector's generalized force: positive along the connector axes.
    PositiveAlongConnectorAxes,
    /// A tie rod's axial force: positive in tension.
    TensionPositive,
    /// A reaction or a compliant support's force: the force or moment the support applies to the
    /// piping.
    ActionOfSupportOnPiping,
}

impl SignConvention {
    pub const ALL: [SignConvention; 5] = [
        Self::PositiveAlongAxis,
        Self::ActionOnElementAtEndNode,
        Self::PositiveAlongConnectorAxes,
        Self::TensionPositive,
        Self::ActionOfSupportOnPiping,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PositiveAlongAxis => "positive_along_axis",
            Self::ActionOnElementAtEndNode => "action_on_element_at_end_node",
            Self::PositiveAlongConnectorAxes => "positive_along_connector_axes",
            Self::TensionPositive => "tension_positive",
            Self::ActionOfSupportOnPiping => "action_of_support_on_piping",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|c| c.as_str() == text)
    }

    /// The convention in words. Positive rotations and moments follow the right-hand rule about
    /// their axis.
    pub const fn statement(self) -> &'static str {
        match self {
            Self::PositiveAlongAxis => "A displacement is positive along its global axis; a rotation is positive by the \
right-hand rule about its global axis.",
            Self::ActionOnElementAtEndNode => "The generalized force acting on the element at its end node, each component \
positive along the axis of the row's frame; a moment is about the end node, positive by the right-hand rule. For a \
member or an arc the internal section force at end i is its negative.",
            Self::PositiveAlongConnectorAxes => "A connector's generalized force, positive along its connector axes and \
work-conjugate to its relative displacements; a moment is positive by the right-hand rule.",
            Self::TensionPositive => "A tie rod's axial force, positive in tension, its axis running from its attachment \
at end i to its attachment at end j.",
            Self::ActionOfSupportOnPiping => "The force or moment the support applies to the piping, positive along the \
global axis; a moment is about the node, positive by the right-hand rule.",
        }
    }
}

/// The identity of one response quantity (one row; REQ-006, "Roster" and "Identity"). Its unit,
/// frame and sign convention follow from it.
///
/// Every entity identity is the model document's, carried byte for byte: any non-blank string
/// (DEL-02-01). A row is identified by its fields ([`RowIdentity`]); [`Quantity::label`] composes
/// a readable label from them that nothing parses and nothing compares.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Quantity {
    /// A node's displacement or rotation, global axes, relative to the supports.
    NodeDisplacement { node: String, component: DisplacementComponent },
    /// A straight member's or a realized arc's end action. A straight member's rows are in global
    /// axes or member axes; an arc's in global axes or its tangent frame at that end. An arc end
    /// stated in member axes is refused (`SPECTRUM_ROW_INVALID`).
    ElementEndAction { member: String, family: MemberFamily, end: End, component: ForceComponent, frame: Frame },
    /// A connector's generalized force, connector axes, once for the connector (no end).
    ConnectorGeneralizedForce { connector: String, component: ConnectorComponent },
    /// A connector's end action, global axes, moments about its end node.
    ConnectorEndAction { connector: String, end: End, component: ForceComponent },
    /// A tie rod's axial force along its rod axis (no end), identified by its connector's
    /// identity and its own, which is unique only within its connector.
    TieRodForce { connector: String, rod: String },
    /// The reaction of a rigid constraint: the force or moment of one global component that the
    /// modal model rigidly restrains at a node, identified by the node and the component, and
    /// carrying the identities of every support that holds it (not part of its identity).
    ConstraintReaction { node: String, component: ReactionComponent, holders: Vec<String> },
    /// A compliant support's own force or moment −K_s·u on one global component.
    CompliantSupportForce { support: String, component: ReactionComponent },
}

/// The fields that identify a row (REQ-006, "Identity"): the kind of quantity, the kind of
/// model entity, that entity's identity (a tie rod's is its connector's and its own), the end
/// where the quantity has one, the component and the frame. Rows are de-duplicated on it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RowIdentity {
    pub quantity_kind: &'static str,
    pub entity_kind: &'static str,
    pub entity: Vec<String>,
    pub end: Option<End>,
    pub component: &'static str,
    pub frame: Frame,
}

const DISPLACEMENT_NAMES: [(DisplacementComponent, &str); 6] = [
    (DisplacementComponent::Ux, "ux"),
    (DisplacementComponent::Uy, "uy"),
    (DisplacementComponent::Uz, "uz"),
    (DisplacementComponent::Rx, "rx"),
    (DisplacementComponent::Ry, "ry"),
    (DisplacementComponent::Rz, "rz"),
];
const FORCE_NAMES: [(ForceComponent, &str); 6] = [
    (ForceComponent::Fx, "Fx"),
    (ForceComponent::Fy, "Fy"),
    (ForceComponent::Fz, "Fz"),
    (ForceComponent::Mx, "Mx"),
    (ForceComponent::My, "My"),
    (ForceComponent::Mz, "Mz"),
];
const REACTION_NAMES: [(ReactionComponent, &str); 6] = [
    (ReactionComponent::Ux, "UX"),
    (ReactionComponent::Uy, "UY"),
    (ReactionComponent::Uz, "UZ"),
    (ReactionComponent::Rx, "RX"),
    (ReactionComponent::Ry, "RY"),
    (ReactionComponent::Rz, "RZ"),
];
const CONNECTOR_NAMES: [(ConnectorComponent, &str); 6] = [
    (ConnectorComponent::Axial, "axial"),
    (ConnectorComponent::LateralY, "lateral_y"),
    (ConnectorComponent::LateralZ, "lateral_z"),
    (ConnectorComponent::Torsion, "torsion"),
    (ConnectorComponent::BendingY, "bending_y"),
    (ConnectorComponent::BendingZ, "bending_z"),
];
/// A tie rod's one component.
pub const TIE_ROD_COMPONENT: &str = "axial";

fn name_of<T: PartialEq + Copy>(table: &[(T, &'static str)], value: T) -> &'static str {
    table.iter().find(|(v, _)| *v == value).map(|(_, n)| *n).expect("every component is named")
}

fn value_of<T: Copy>(table: &[(T, &'static str)], name: &str) -> Option<T> {
    table.iter().find(|(_, n)| *n == name).map(|(v, _)| *v)
}

impl DisplacementComponent {
    pub fn as_str(self) -> &'static str {
        name_of(&DISPLACEMENT_NAMES, self)
    }
    pub fn parse(text: &str) -> Option<Self> {
        value_of(&DISPLACEMENT_NAMES, text)
    }
    pub fn unit(self) -> RowUnit {
        match self {
            Self::Ux | Self::Uy | Self::Uz => RowUnit::Meter,
            Self::Rx | Self::Ry | Self::Rz => RowUnit::Radian,
        }
    }
}

impl ForceComponent {
    pub fn as_str(self) -> &'static str {
        name_of(&FORCE_NAMES, self)
    }
    pub fn parse(text: &str) -> Option<Self> {
        value_of(&FORCE_NAMES, text)
    }
    pub fn unit(self) -> RowUnit {
        match self {
            Self::Fx | Self::Fy | Self::Fz => RowUnit::Newton,
            Self::Mx | Self::My | Self::Mz => RowUnit::NewtonMeter,
        }
    }
}

impl ReactionComponent {
    pub fn as_str(self) -> &'static str {
        name_of(&REACTION_NAMES, self)
    }
    pub fn parse(text: &str) -> Option<Self> {
        value_of(&REACTION_NAMES, text)
    }
    pub fn unit(self) -> RowUnit {
        match self {
            Self::Ux | Self::Uy | Self::Uz => RowUnit::Newton,
            Self::Rx | Self::Ry | Self::Rz => RowUnit::NewtonMeter,
        }
    }
}

impl ConnectorComponent {
    pub fn as_str(self) -> &'static str {
        name_of(&CONNECTOR_NAMES, self)
    }
    pub fn parse(text: &str) -> Option<Self> {
        value_of(&CONNECTOR_NAMES, text)
    }
    pub fn unit(self) -> RowUnit {
        match self {
            Self::Axial | Self::LateralY | Self::LateralZ => RowUnit::Newton,
            Self::Torsion | Self::BendingY | Self::BendingZ => RowUnit::NewtonMeter,
        }
    }
}

impl End {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::I => "i",
            Self::J => "j",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        [Self::I, Self::J].into_iter().find(|e| e.as_str() == text)
    }
}

/// The quantity kinds, as the contract names them.
pub const QUANTITY_KINDS: [&str; 7] = [
    "node_displacement",
    "element_end_action",
    "connector_generalized_force",
    "connector_end_action",
    "tie_rod_force",
    "constraint_reaction",
    "compliant_support_force",
];

impl Quantity {
    /// The quantity's kind.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::NodeDisplacement { .. } => QUANTITY_KINDS[0],
            Self::ElementEndAction { .. } => QUANTITY_KINDS[1],
            Self::ConnectorGeneralizedForce { .. } => QUANTITY_KINDS[2],
            Self::ConnectorEndAction { .. } => QUANTITY_KINDS[3],
            Self::TieRodForce { .. } => QUANTITY_KINDS[4],
            Self::ConstraintReaction { .. } => QUANTITY_KINDS[5],
            Self::CompliantSupportForce { .. } => QUANTITY_KINDS[6],
        }
    }

    /// The kind of model entity the row belongs to.
    pub fn entity_kind(&self) -> &'static str {
        match self {
            Self::NodeDisplacement { .. } => "node",
            Self::ElementEndAction { .. } => "member",
            Self::ConnectorGeneralizedForce { .. } | Self::ConnectorEndAction { .. } => "connector",
            Self::TieRodForce { .. } => "tie_rod",
            Self::ConstraintReaction { .. } => "rigid_constraint",
            Self::CompliantSupportForce { .. } => "compliant_support",
        }
    }

    /// The entity's identity: one model identity, or a tie rod's connector and rod.
    pub fn entity(&self) -> Vec<&str> {
        match self {
            Self::NodeDisplacement { node, .. } | Self::ConstraintReaction { node, .. } => vec![node],
            Self::ElementEndAction { member, .. } => vec![member],
            Self::ConnectorGeneralizedForce { connector, .. } | Self::ConnectorEndAction { connector, .. } => vec![connector],
            Self::TieRodForce { connector, rod } => vec![connector, rod],
            Self::CompliantSupportForce { support, .. } => vec![support],
        }
    }

    /// The end, where the quantity has one.
    pub fn end(&self) -> Option<End> {
        match self {
            Self::ElementEndAction { end, .. } | Self::ConnectorEndAction { end, .. } => Some(*end),
            _ => None,
        }
    }

    pub fn component_name(&self) -> &'static str {
        match self {
            Self::NodeDisplacement { component, .. } => component.as_str(),
            Self::ElementEndAction { component, .. } | Self::ConnectorEndAction { component, .. } => component.as_str(),
            Self::ConnectorGeneralizedForce { component, .. } => component.as_str(),
            Self::TieRodForce { .. } => TIE_ROD_COMPONENT,
            Self::ConstraintReaction { component, .. } | Self::CompliantSupportForce { component, .. } => component.as_str(),
        }
    }

    pub fn unit(&self) -> RowUnit {
        match self {
            Self::NodeDisplacement { component, .. } => component.unit(),
            Self::ElementEndAction { component, .. } | Self::ConnectorEndAction { component, .. } => component.unit(),
            Self::ConnectorGeneralizedForce { component, .. } => component.unit(),
            Self::TieRodForce { .. } => RowUnit::Newton,
            Self::ConstraintReaction { component, .. } | Self::CompliantSupportForce { component, .. } => component.unit(),
        }
    }

    /// The row's frame: stated for an element end action, fixed by the kind otherwise.
    pub fn frame(&self) -> Frame {
        match self {
            Self::ElementEndAction { frame, .. } => *frame,
            Self::ConnectorGeneralizedForce { .. } => Frame::ConnectorAxes,
            Self::TieRodForce { .. } => Frame::RodAxis,
            _ => Frame::GlobalAxes,
        }
    }

    /// The sign convention of the quantity's signed rows.
    pub fn sign_convention(&self) -> SignConvention {
        match self {
            Self::NodeDisplacement { .. } => SignConvention::PositiveAlongAxis,
            Self::ElementEndAction { .. } | Self::ConnectorEndAction { .. } => SignConvention::ActionOnElementAtEndNode,
            Self::ConnectorGeneralizedForce { .. } => SignConvention::PositiveAlongConnectorAxes,
            Self::TieRodForce { .. } => SignConvention::TensionPositive,
            Self::ConstraintReaction { .. } | Self::CompliantSupportForce { .. } => SignConvention::ActionOfSupportOnPiping,
        }
    }

    /// The supports that hold a rigid constraint; empty for every other kind.
    pub fn holders(&self) -> &[String] {
        match self {
            Self::ConstraintReaction { holders, .. } => holders,
            _ => &[],
        }
    }

    /// The fields that identify the row.
    pub fn identity(&self) -> RowIdentity {
        RowIdentity {
            quantity_kind: self.kind(),
            entity_kind: self.entity_kind(),
            entity: self.entity().into_iter().map(str::to_string).collect(),
            end: self.end(),
            component: self.component_name(),
            frame: self.frame(),
        }
    }

    /// A readable label composed from the identity fields, for messages and display only: a model
    /// identity may contain any character, so nothing parses or compares a label.
    pub fn label(&self) -> String {
        let end = self.end().map(|e| format!(" end {}", e.as_str())).unwrap_or_default();
        format!("{} {} [{}]{end} {} ({})", self.kind(), self.entity_kind(), self.entity().join("] ["), self.component_name(),
            self.frame().as_str())
    }
}

/// What a published number is, by its sign status (REQ-006, AX-003).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignStatus {
    /// A signed per-mode, per-direction periodic value, or a signed rigid response. It is an
    /// input of the combinations, not a peak response.
    SignedContribution,
    /// The signed sum of one cluster's per-mode contributions in one direction, the unit the
    /// modal rule combines (CLM-001, Repeated modes). It is not a peak response.
    SignedClusterSum,
    /// An unsigned combined magnitude. It is never a signed quantity.
    UnsignedCombinedMagnitude,
}

impl SignStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedContribution => "signed_contribution",
            Self::SignedClusterSum => "signed_cluster_sum",
            Self::UnsignedCombinedMagnitude => "unsigned_combined_magnitude",
        }
    }

    /// The sentence a surface states with a number of this status.
    pub const fn statement(self) -> &'static str {
        match self {
            Self::SignedContribution => "signed contribution, not a peak response",
            Self::SignedClusterSum => "signed sum of one cluster's contributions, not a peak response",
            Self::UnsignedCombinedMagnitude => "unsigned combined magnitude; it has no sign",
        }
    }
}

/// An unsigned combined magnitude. It is constructed only by the combinations, holds a
/// non-negative finite value certified within 1e-9 relative of its rule's exact value on the
/// represented operands (or an exact zero), and offers no signed use (AX-003).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Magnitude(f64);

impl Magnitude {
    /// A finite non-negative value; the combinations never form another.
    pub(crate) fn new(value: f64) -> Self {
        debug_assert!(value.is_finite() && value >= 0.0, "a magnitude is finite and non-negative");
        Self(value)
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

/// The modal combination rule (CLM-001, Modal combination; REQ-003). A required choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModalRule {
    Srss,
    Cqc,
    /// The grouping method: groups in which each frequency is within 10 % of the group's lowest.
    Grouping,
    /// The ten percent method: every pair whose frequencies are within 10 % of the lower.
    TenPercent,
    AbsoluteSum,
}

impl ModalRule {
    pub const ALL: [ModalRule; 5] = [Self::Srss, Self::Cqc, Self::Grouping, Self::TenPercent, Self::AbsoluteSum];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Srss => "SRSS",
            Self::Cqc => "CQC",
            Self::Grouping => "GROUPING",
            Self::TenPercent => "TEN_PERCENT",
            Self::AbsoluteSum => "ABSOLUTE_SUM",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|r| r.as_str() == text)
    }
}

/// The directional combination rule (CLM-001, Directional combination). A required choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DirectionalRule {
    Srss,
    /// 100-40-40: the full magnitude of the largest direction plus 2/5 of each other.
    HundredFortyForty,
    AbsoluteSum,
}

impl DirectionalRule {
    pub const ALL: [DirectionalRule; 3] = [Self::Srss, Self::HundredFortyForty, Self::AbsoluteSum];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Srss => "SRSS",
            Self::HundredFortyForty => "100-40-40",
            Self::AbsoluteSum => "ABSOLUTE_SUM",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|r| r.as_str() == text)
    }
}

/// The combination of the rigid response with the periodic part (REQ-001, Missing mass).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RigidRule {
    Srss,
    AbsoluteSum,
}

impl RigidRule {
    pub const ALL: [RigidRule; 2] = [Self::Srss, Self::AbsoluteSum];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Srss => "SRSS",
            Self::AbsoluteSum => "ABSOLUTE_SUM",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|r| r.as_str() == text)
    }
}

/// Whether the missing-mass correction is applied; a required choice (REQ-001).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MissingMass {
    /// Every extracted mode is periodic; no rigid response is supplied.
    NotApplied,
    /// Clusters whose lowest frequency is below `rigid_frequency_hz` are periodic, the rest
    /// rigid; the supplied rigid response is combined with the periodic part by `rule`.
    Applied { rigid_frequency_hz: f64, rule: RigidRule },
}

/// The signed contributions of one mode in one direction, one value per row in row order.
#[derive(Debug, Clone, PartialEq)]
pub struct ModalContribution {
    pub mode: ModeIndex,
    pub direction: Direction,
    pub values: Vec<f64>,
}

/// The signed rigid response of one direction, one value per row in row order, with the
/// clusters its load left out as periodic (its premise, CLM-001 Rigid response).
#[derive(Debug, Clone, PartialEq)]
pub struct RigidContribution {
    pub direction: Direction,
    pub periodic_clusters_left_out: Vec<ClusterIndex>,
    pub values: Vec<f64>,
}

/// Everything a combination takes. Every member is the user's choice or a supplied value;
/// nothing is defaulted (REQ-001, AX-001).
#[derive(Debug, Clone, PartialEq)]
pub struct CombinationRequest {
    /// The spectra's common damping ratio, 0 ≤ ζ ≤ 0.2; CQC uses it and refuses 0.
    pub damping_ratio: f64,
    pub modal_rule: ModalRule,
    pub directional_rule: DirectionalRule,
    pub missing_mass: MissingMass,
    /// The requested directions, each once.
    pub directions: Vec<Direction>,
    /// Every extracted mode, in ascending `mode_index`.
    pub modes: Vec<ModalMode>,
    /// The rows, each once.
    pub rows: Vec<Quantity>,
    /// One entry per mode of a periodic cluster and requested direction.
    pub periodic: Vec<ModalContribution>,
    /// When the correction is applied, one entry per requested direction; otherwise none.
    pub rigid: Vec<RigidContribution>,
}

/// Whether a cluster is periodic or rigid (CLM-001, Rigid response).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClusterClass {
    Periodic,
    Rigid,
}

impl ClusterClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Periodic => "periodic",
            Self::Rigid => "rigid",
        }
    }
}

/// The statement of one cluster (REQ-003): its modes, their frequencies, the frequency that
/// stands for it (its lowest) and its classification.
#[derive(Debug, Clone, PartialEq)]
pub struct ClusterStatement {
    pub cluster: ClusterIndex,
    pub modes: Vec<ModeIndex>,
    pub frequencies_hz: Vec<f64>,
    pub standing_frequency_hz: f64,
    pub class: ClusterClass,
}

/// The signed cluster sums of one periodic cluster of more than one mode in one direction, one per
/// row: the binary64 nearest each exact sum, ties to even. A cluster of one mode has none: its
/// contribution is the sum and is not repeated.
#[derive(Debug, Clone, PartialEq)]
pub struct ClusterSum {
    pub cluster: ClusterIndex,
    pub direction: Direction,
    pub values: Vec<f64>,
}

/// One direction's combined magnitudes of one row.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectionalPart {
    pub direction: Direction,
    /// The modal rule over the periodic cluster sums.
    pub periodic: Magnitude,
    /// |R_r|, when the correction is applied.
    pub rigid: Option<Magnitude>,
    /// The periodic part with the rigid response by the rigid rule, or the periodic part.
    pub combined: Magnitude,
}

/// The combined magnitudes of one row.
#[derive(Debug, Clone, PartialEq)]
pub struct CombinedRow {
    pub quantity: Quantity,
    pub per_direction: Vec<DirectionalPart>,
    /// The directional rule over the requested directions.
    pub combined: Magnitude,
    /// Under 100-40-40, the direction carrying the full value: the largest directional
    /// magnitude, the first in X, Y, Z order on a tie.
    pub governing_direction: Option<Direction>,
}

/// The informational correlation coefficient of one unordered pair of distinct periodic clusters,
/// `first` below `second`: [`crate::correlation_coefficient`] at their standing frequencies and the
/// damping ratio.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CorrelationStatement {
    pub first: ClusterIndex,
    pub second: ClusterIndex,
    pub rho: f64,
}

/// The output of a combination: the rules, the cluster statement, what the closeness rules and
/// CQC used, the signed cluster sums and the unsigned combined rows.
///
/// Field semantics (released by Agent 0 for readers): published cluster sums are the correctly
/// rounded binary64 of exact sums while combinations use the retained exact sums; the published
/// correlation is informational, the value [`crate::correlation_coefficient`] gives for the pair,
/// while combined magnitudes come from the enclosure at the level required; a reader derives its arithmetic from the original
/// represented signed contributions, frequencies, damping and method, and never from the rounded
/// display values.
/// No standing beyond these values is implied.
#[derive(Debug, Clone, PartialEq)]
pub struct Combination {
    pub damping_ratio: f64,
    pub modal_rule: ModalRule,
    pub directional_rule: DirectionalRule,
    pub missing_mass: MissingMass,
    pub directions: Vec<Direction>,
    /// Every supplied cluster in ascending order.
    pub clusters: Vec<ClusterStatement>,
    /// Under CQC, one coefficient per unordered pair of distinct periodic clusters, in ascending
    /// order of the pair (no diagonal, no second triangle). Informational; the combinations use
    /// the enclosures themselves.
    pub correlation: Option<Vec<CorrelationStatement>>,
    /// Under the grouping method, the groups of periodic clusters.
    pub groups: Option<Vec<Vec<ClusterIndex>>>,
    /// Under the ten percent method, every coupled pair of periodic clusters.
    pub close_pairs: Option<Vec<(ClusterIndex, ClusterIndex)>>,
    /// Per requested direction (X, Y, Z order) and periodic cluster of more than one mode.
    pub cluster_sums: Vec<ClusterSum>,
    pub rows: Vec<CombinedRow>,
}
