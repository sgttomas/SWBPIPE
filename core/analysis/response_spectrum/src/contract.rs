//! The signed-contribution contract of DEL-04-10's combinations (unit S4).
//!
//! These are the types through which a producer of signed per-mode, per-direction response
//! values (the modal-response recovery, a later unit) hands them to the combinations, and
//! through which the combinations hand back unsigned combined magnitudes. A later document unit
//! and its readers can build on the same identities, units, frames and sign tags. This module
//! fixes them; it does not publish a spectrum result (`spectrum-1`, REQ-006), and nothing here
//! carries standing as an engineering result.
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
//! **Sign conventions** a producer follows so that a sum of signed contributions means what the
//! method says (the frozen references' conventions, `validation/references/f2_spectrum/discrete/
//! README.md`, "Conventions a product must match or map"):
//! - a node displacement or rotation is in global axes, positive along the axis;
//! - an element end force or moment is the generalized force acting **on the element** at that
//!   end, in global axes or in the member's axes as the row's [`Frame`] states. A combined
//!   magnitude belongs to the frame it was combined in: a combination of member-axis
//!   contributions is not a rotation of a global-axis combination;
//! - a support reaction is the generalized force the support applies **to the piping**, in
//!   global axes; `Fd` is the signed component along a skew support's unit direction d/|d|.
//!
//! Units are SI and fixed by the row's component: m, rad, N or N*m (the unit catalog's
//! symbols). No unit is converted here.
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

/// The component of an element end force or moment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ForceComponent {
    Fx,
    Fy,
    Fz,
    Mx,
    My,
    Mz,
}

/// The component of a support reaction, in global axes; `Fd` along a skew support's d/|d|.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ReactionComponent {
    Fx,
    Fy,
    Fz,
    Mx,
    My,
    Mz,
    Fd,
}

/// An element end: node i or node j of the member.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum End {
    I,
    J,
}

/// The axes a row's value is expressed in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Frame {
    /// Global axes.
    Global,
    /// The member's axes (x along the member, y from its reference vector, z = x × y). Only an
    /// element end force may be supplied in member axes.
    MemberAxes,
}

impl Frame {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Global => "global",
            Self::MemberAxes => "member_axes",
        }
    }
}

/// The SI unit of a row, a symbol of the unit catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RowUnit {
    Meter,
    Radian,
    Newton,
    NewtonMeter,
}

impl RowUnit {
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Meter => "m",
            Self::Radian => "rad",
            Self::Newton => "N",
            Self::NewtonMeter => "N*m",
        }
    }
}

/// The identity of one response quantity (one row). Its key, unit and frame follow from it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Quantity {
    NodeDisplacement { node: String, component: DisplacementComponent },
    ElementEndForce { member: String, end: End, component: ForceComponent, frame: Frame },
    SupportReaction { support: String, component: ReactionComponent },
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
const REACTION_NAMES: [(ReactionComponent, &str); 7] = [
    (ReactionComponent::Fx, "Fx"),
    (ReactionComponent::Fy, "Fy"),
    (ReactionComponent::Fz, "Fz"),
    (ReactionComponent::Mx, "Mx"),
    (ReactionComponent::My, "My"),
    (ReactionComponent::Mz, "Mz"),
    (ReactionComponent::Fd, "Fd"),
];

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
            Self::Fx | Self::Fy | Self::Fz | Self::Fd => RowUnit::Newton,
            Self::Mx | Self::My | Self::Mz => RowUnit::NewtonMeter,
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
}

impl Quantity {
    /// The quantity's kind, as the contract names it.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::NodeDisplacement { .. } => "node_displacement",
            Self::ElementEndForce { .. } => "element_end_force",
            Self::SupportReaction { .. } => "support_reaction",
        }
    }

    /// The model object the row belongs to: a node, member or support id.
    pub fn object(&self) -> &str {
        match self {
            Self::NodeDisplacement { node, .. } => node,
            Self::ElementEndForce { member, .. } => member,
            Self::SupportReaction { support, .. } => support,
        }
    }

    pub fn component_name(&self) -> &'static str {
        match self {
            Self::NodeDisplacement { component, .. } => component.as_str(),
            Self::ElementEndForce { component, .. } => component.as_str(),
            Self::SupportReaction { component, .. } => component.as_str(),
        }
    }

    pub fn unit(&self) -> RowUnit {
        match self {
            Self::NodeDisplacement { component, .. } => component.unit(),
            Self::ElementEndForce { component, .. } => component.unit(),
            Self::SupportReaction { component, .. } => component.unit(),
        }
    }

    pub fn frame(&self) -> Frame {
        match self {
            Self::ElementEndForce { frame, .. } => *frame,
            _ => Frame::Global,
        }
    }

    /// The row key, in the frozen references' form: `N3.uy`, `P1.i.Fx` (global axes),
    /// `P1.i.Fx_m` (member axes), `A1.Fx`.
    pub fn key(&self) -> String {
        match self {
            Self::NodeDisplacement { node, component } => format!("{node}.{}", component.as_str()),
            Self::ElementEndForce { member, end, component, frame } => {
                let suffix = if *frame == Frame::MemberAxes { "_m" } else { "" };
                format!("{member}.{}.{}{suffix}", end.as_str(), component.as_str())
            }
            Self::SupportReaction { support, component } => format!("{support}.{}", component.as_str()),
        }
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

/// The signed cluster sums of one periodic cluster in one direction, one per row: the binary64
/// nearest each exact sum.
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

/// The output of a combination: the rules, the cluster statement, what the closeness rules and
/// CQC used, the signed cluster sums and the unsigned combined rows.
#[derive(Debug, Clone, PartialEq)]
pub struct Combination {
    pub damping_ratio: f64,
    pub modal_rule: ModalRule,
    pub directional_rule: DirectionalRule,
    pub missing_mass: MissingMass,
    pub directions: Vec<Direction>,
    /// Every supplied cluster in ascending order.
    pub clusters: Vec<ClusterStatement>,
    /// Under CQC, ρ over the periodic clusters in ascending order (row i, column j): the binary64
    /// nearest the midpoint of each first-level enclosure (binary64-guided, ends rounded outward
    /// to 64 bits; relative width about 2^-48). Informational; the combinations use the
    /// enclosures themselves.
    pub correlation: Option<Vec<Vec<f64>>>,
    /// Under the grouping method, the groups of periodic clusters.
    pub groups: Option<Vec<Vec<ClusterIndex>>>,
    /// Under the ten percent method, every coupled pair of periodic clusters.
    pub close_pairs: Option<Vec<(ClusterIndex, ClusterIndex)>>,
    /// Per requested direction and periodic cluster.
    pub cluster_sums: Vec<ClusterSum>,
    pub rows: Vec<CombinedRow>,
}
