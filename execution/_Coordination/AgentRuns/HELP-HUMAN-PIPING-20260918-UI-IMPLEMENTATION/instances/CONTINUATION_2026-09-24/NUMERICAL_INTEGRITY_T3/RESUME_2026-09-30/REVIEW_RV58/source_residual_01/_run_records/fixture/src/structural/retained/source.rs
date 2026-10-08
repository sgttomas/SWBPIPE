//! K4: `PrimitiveSource`, W1a's immutable per-case declaration (T3 D1 §4.1.1).
//!
//! - Node coordinates (binary64); straight members
//!   `{node_i, node_j, E, G, A, Iy, Iz, J, y_reference}`, all binary64
//!   operands; global-axis springs `(dof, k > 0)`; the kernel-only
//!   `DirectionalSpring` (ROOT's K4 ruling Q6), formed at p as k·n nᵀ/(nᵀn) from
//!   its binary64 direction; constraints `(dof, value)` (0: rigid restraint,
//!   otherwise prescribed motion, "kernel yes", D1 §4.2); nodal loads
//!   `(dof, value, source_id)`; stations (binary64 fractions of the chord) and
//!   the optional support groups for the magnitude rows (ROOT's K4 rulings O2
//!   and O3: ids on members, springs, stations and groups).
//! - "Only supported families can be constructed, so exclusion is a type-level
//!   fact." The DOF partition is complete by construction: a DOF is free unless
//!   constrained, and a DOF constrained twice is refused.
//! - "Validation rejects nonfinite or nonpositive properties and incomplete
//!   partitions. It also rejects any derived primitive that is subnormal"
//!   (A, Iy, Iz and J, the primitives the product derives). A zero-length member
//!   and a y reference parallel to the chord are refused, decided exactly: "No
//!   axis tolerance is used at p."
//! - Every list is sorted canonically at construction (members, springs,
//!   stations and groups by id, constraints by DOF, loads by DOF, source id and
//!   value bits), so the canonical encoding (ROOT's K4 ruling Q10) and every
//!   result are independent of the order the caller listed them in.
use super::wide_sum::ExactWideSum;
use crate::DOF_PER_NODE;

/// A DOF component, in FK's order (UX, UY, UZ, RX, RY, RZ).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Component {
    Ux,
    Uy,
    Uz,
    Rx,
    Ry,
    Rz,
}

impl Component {
    pub const ALL: [Component; 6] = [
        Component::Ux,
        Component::Uy,
        Component::Uz,
        Component::Rx,
        Component::Ry,
        Component::Rz,
    ];

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn from_index(index: usize) -> Self {
        Self::ALL[index]
    }

    pub(crate) fn is_translation(self) -> bool {
        self.index() < 3
    }
}

/// A DOF: node `k` owns global DOFs 6k..6k+5.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dof {
    pub node: u32,
    pub component: Component,
}

impl Dof {
    pub fn global(self) -> usize {
        self.node as usize * DOF_PER_NODE + self.component.index()
    }

    pub fn from_global(global: usize) -> Self {
        Self {
            node: (global / DOF_PER_NODE) as u32,
            component: Component::from_index(global % DOF_PER_NODE),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StraightMember {
    pub id: u32,
    pub node_i: u32,
    pub node_j: u32,
    pub elastic_modulus: f64,
    pub shear_modulus: f64,
    pub area: f64,
    pub second_moment_y: f64,
    pub second_moment_z: f64,
    pub torsion_constant: f64,
    pub y_reference: [f64; 3],
}

/// A global-axis linear spring to ground.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spring {
    pub id: u32,
    pub dof: Dof,
    pub stiffness: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpringKind {
    Translation,
    Rotation,
}

impl SpringKind {
    /// The first global component of the kind (UX or RX).
    pub(crate) fn offset(self) -> usize {
        match self {
            Self::Translation => 0,
            Self::Rotation => 3,
        }
    }
}

/// A kernel-only spring to ground along a binary64 direction (ROOT's K4 ruling
/// Q6); F2a never builds one.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectionalSpring {
    pub id: u32,
    pub node: u32,
    pub kind: SpringKind,
    pub direction: [f64; 3],
    pub stiffness: f64,
}

/// A rigid restraint (value 0) or a prescribed motion (any other value).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Constraint {
    pub dof: Dof,
    pub value: f64,
}

/// One identified nodal load contribution (the ledger's granularity).
#[derive(Debug, Clone, PartialEq)]
pub struct NodalLoad {
    pub dof: Dof,
    pub value: f64,
    pub source_id: String,
}

/// A station on a member at a binary64 fraction of its chord (0 at node i).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Station {
    pub id: u32,
    pub member: u32,
    pub fraction: f64,
}

/// A support for the magnitude rows (ROOT's K4 ruling O2): the components it
/// restrains at its node, and its springs.
#[derive(Debug, Clone, PartialEq)]
pub struct SupportGroup {
    pub id: u32,
    pub node: u32,
    pub restrained: [bool; 6],
    pub springs: Vec<u32>,
    pub directional_springs: Vec<u32>,
}

/// The caller's lists, in any order.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SourceParts {
    pub nodes: Vec<[f64; 3]>,
    pub members: Vec<StraightMember>,
    pub springs: Vec<Spring>,
    pub directional_springs: Vec<DirectionalSpring>,
    pub constraints: Vec<Constraint>,
    pub loads: Vec<NodalLoad>,
    pub stations: Vec<Station>,
    pub supports: Vec<SupportGroup>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberProperty {
    ElasticModulus,
    ShearModulus,
    Area,
    SecondMomentY,
    SecondMomentZ,
    TorsionConstant,
    YReference,
}

/// Why a source cannot be constructed.
#[derive(Debug, Clone, PartialEq)]
pub enum SourceError {
    CountRange(&'static str),
    NoNodes,
    NonFiniteCoordinate {
        node: u32,
    },
    NodeOutOfRange {
        node: u32,
    },
    DuplicateMemberId {
        id: u32,
    },
    RepeatedMemberNode {
        member: u32,
    },
    NonFiniteProperty {
        member: u32,
        property: MemberProperty,
    },
    NonPositiveProperty {
        member: u32,
        property: MemberProperty,
    },
    SubnormalDerivedPrimitive {
        member: u32,
        property: MemberProperty,
    },
    ZeroLength {
        member: u32,
    },
    DegenerateAxis {
        member: u32,
    },
    DuplicateSpringId {
        id: u32,
    },
    NonPositiveSpring {
        id: u32,
    },
    ZeroDirection {
        id: u32,
    },
    NonFiniteDirection {
        id: u32,
    },
    DuplicateConstraint {
        dof: Dof,
    },
    NonFiniteValue {
        dof: Dof,
    },
    EmptyLoadSource {
        dof: Dof,
    },
    DuplicateStationId {
        id: u32,
    },
    UnknownMember {
        station: u32,
        member: u32,
    },
    StationOutOfRange {
        id: u32,
    },
    DuplicateSupportId {
        id: u32,
    },
    SupportMismatch {
        id: u32,
    },
}

impl std::fmt::Display for SourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "retained source: {self:?}")
    }
}

impl std::error::Error for SourceError {}

/// The validated, canonically ordered source (module documentation).
#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveSource {
    nodes: Vec<[f64; 3]>,
    members: Vec<StraightMember>,
    springs: Vec<Spring>,
    directional_springs: Vec<DirectionalSpring>,
    constraints: Vec<Constraint>,
    loads: Vec<NodalLoad>,
    stations: Vec<Station>,
    supports: Vec<SupportGroup>,
    /// Per global DOF: the constraint value, if constrained.
    constrained: Vec<Option<f64>>,
    /// Body of each node: the connected components of the member graph,
    /// numbered by their lowest node.
    body_of_node: Vec<u32>,
    body_count: u32,
}

fn property_check(
    member: u32,
    property: MemberProperty,
    value: f64,
    derived: bool,
) -> Result<(), SourceError> {
    if !value.is_finite() {
        return Err(SourceError::NonFiniteProperty { member, property });
    }
    if value <= 0.0 {
        return Err(SourceError::NonPositiveProperty { member, property });
    }
    if derived && value.is_subnormal() {
        return Err(SourceError::SubnormalDerivedPrimitive { member, property });
    }
    Ok(())
}

/// (negative, significand, exponent of its unit bit) of a finite nonzero x.
fn binary64_parts(x: f64) -> (bool, u64, i64) {
    let bits = x.to_bits();
    let biased = ((bits >> 52) & 0x7ff) as i64;
    let fraction = bits & ((1u64 << 52) - 1);
    let (significand, lsb) = if biased == 0 {
        (fraction, -1074)
    } else {
        (fraction | (1u64 << 52), biased - 1075)
    };
    (x.is_sign_negative(), significand, lsb)
}

/// True when the exact chord x_j − x_i is zero (both coordinates binary64).
/// Otherwise, whether y is parallel to it, decided exactly: (x_j − x_i) × y = 0.
fn chord_checks(xi: [f64; 3], xj: [f64; 3], y: [f64; 3]) -> (bool, bool) {
    let mut d: Vec<ExactWideSum> = Vec::with_capacity(3);
    let mut zero = true;
    for k in 0..3 {
        let mut s = ExactWideSum::new();
        s.add_binary64(xj[k], false).expect("finite");
        s.add_binary64(xi[k], true).expect("finite");
        zero &= s.is_zero().expect("fresh chord difference: work <= 1152");
        d.push(s);
    }
    if zero {
        return (true, false);
    }
    let mut parallel = true;
    for (a, b) in [(1usize, 2usize), (2, 0), (0, 1)] {
        // d_a·y_b − d_b·y_a
        let mut c = ExactWideSum::new();
        if y[b] != 0.0 {
            let (neg, sig, lsb) = binary64_parts(y[b]);
            c.add_scaled(&d[a], neg, sig, lsb).expect("in span");
        }
        if y[a] != 0.0 {
            let (neg, sig, lsb) = binary64_parts(y[a]);
            c.add_scaled(&d[b], !neg, sig, lsb).expect("in span");
        }
        parallel &= c
            .is_zero()
            .expect("fresh chord cross product: work <= 2688");
    }
    (false, parallel)
}

// Scalar representation checks only; these do not supply a memory allowance.
fn checked_profile_count(free: usize) -> Result<(), SourceError> {
    let product = free
        .checked_add(1)
        .and_then(|n| free.checked_mul(n))
        .ok_or(SourceError::CountRange("free profile"))?;
    if free > u32::MAX as usize {
        return Err(SourceError::CountRange("free block sentinel"));
    }
    std::alloc::Layout::array::<usize>(product / 2)
        .map_err(|_| SourceError::CountRange("profile layout"))?;
    Ok(())
}
#[allow(clippy::too_many_arguments)]
fn source_counts(
    nodes: &[[f64; 3]],
    members: &[StraightMember],
    springs: &[Spring],
    directional: &[DirectionalSpring],
    constraints: &[Constraint],
    loads: &[NodalLoad],
    stations: &[Station],
    supports: &[SupportGroup],
) -> Result<(), SourceError> {
    let fail = || SourceError::CountRange("source representation");
    for len in [
        nodes.len(),
        members.len(),
        springs.len(),
        directional.len(),
        constraints.len(),
        loads.len(),
        stations.len(),
        supports.len(),
    ] {
        u32::try_from(len).map_err(|_| fail())?;
    }
    let n = nodes.len().checked_mul(DOF_PER_NODE).ok_or_else(fail)?;
    std::alloc::Layout::array::<Option<f64>>(n).map_err(|_| fail())?;
    n.checked_add(1).ok_or_else(fail)?;
    let mut bytes = 38usize;
    for (len, stride) in [
        (nodes.len(), 24usize),
        (members.len(), 84),
        (springs.len(), 17),
        (directional.len(), 41),
        (constraints.len(), 13),
        (loads.len(), 17),
        (stations.len(), 16),
        (supports.len(), 22),
    ] {
        bytes = len
            .checked_mul(stride)
            .and_then(|v| bytes.checked_add(v))
            .ok_or_else(fail)?;
    }
    for load in loads {
        u32::try_from(load.source_id.len()).map_err(|_| fail())?;
        bytes = bytes.checked_add(load.source_id.len()).ok_or_else(fail)?;
    }
    for group in supports {
        for children in [&group.springs, &group.directional_springs] {
            u32::try_from(children.len()).map_err(|_| fail())?;
            bytes = children
                .len()
                .checked_mul(4)
                .and_then(|v| bytes.checked_add(v))
                .ok_or_else(fail)?;
        }
    }
    std::alloc::Layout::array::<u8>(bytes).map_err(|_| fail())?;
    let mut q = 0usize;
    for (len, stride) in [
        (nodes.len(), 7usize),
        (members.len(), 12),
        (stations.len(), 6),
        (springs.len(), 1),
        (directional.len(), 3),
        (constraints.len(), 1),
        (supports.len(), 2),
    ] {
        q = len
            .checked_mul(stride)
            .and_then(|v| q.checked_add(v))
            .ok_or_else(fail)?;
    }
    u32::try_from(q).map_err(|_| fail())?;
    std::alloc::Layout::array::<super::recover::QuantityMeta>(q).map_err(|_| fail())?;
    // Pattern, tags, prefix sums and transpose products before their construction.
    let p = members
        .len()
        .checked_mul(144)
        .and_then(|v| {
            directional
                .len()
                .checked_mul(9)
                .and_then(|d| v.checked_add(d))
        })
        .and_then(|v| v.checked_add(springs.len()))
        .ok_or_else(fail)?;
    let z = n.checked_mul(n).ok_or_else(fail)?.min(p);
    for count in [p, z] {
        count.checked_mul(2).ok_or_else(fail)?;
        let prefix = count.checked_add(1).ok_or_else(fail)?;
        std::alloc::Layout::array::<usize>(prefix).map_err(|_| fail())?;
    }
    std::alloc::Layout::array::<(usize, usize)>(p).map_err(|_| fail())?;
    Ok(())
}

impl PrimitiveSource {
    pub fn new(parts: SourceParts) -> Result<Self, SourceError> {
        let SourceParts {
            nodes,
            mut members,
            mut springs,
            mut directional_springs,
            mut constraints,
            mut loads,
            mut stations,
            mut supports,
        } = parts;
        source_counts(
            &nodes,
            &members,
            &springs,
            &directional_springs,
            &constraints,
            &loads,
            &stations,
            &supports,
        )?;
        // V-K seeded fault VK-F10 (§7.3-10, list-order form): the canonical
        // sort skipped (the caller's order kept).
        #[cfg(any(test, feature = "mutation-controls"))]
        let caller_order = (
            members.clone(),
            springs.clone(),
            directional_springs.clone(),
            constraints.clone(),
            loads.clone(),
            stations.clone(),
        );
        if nodes.is_empty() {
            return Err(SourceError::NoNodes);
        }
        let node_count = nodes.len();
        for (index, p) in nodes.iter().enumerate() {
            if p.iter().any(|c| !c.is_finite()) {
                return Err(SourceError::NonFiniteCoordinate { node: index as u32 });
            }
        }
        let in_range = |node: u32| -> Result<(), SourceError> {
            if (node as usize) < node_count {
                Ok(())
            } else {
                Err(SourceError::NodeOutOfRange { node })
            }
        };
        members.sort_by_key(|m| m.id);
        for pair in members.windows(2) {
            if pair[0].id == pair[1].id {
                return Err(SourceError::DuplicateMemberId { id: pair[0].id });
            }
        }
        for m in &members {
            in_range(m.node_i)?;
            in_range(m.node_j)?;
            if m.node_i == m.node_j {
                return Err(SourceError::RepeatedMemberNode { member: m.id });
            }
            use MemberProperty as P;
            property_check(m.id, P::ElasticModulus, m.elastic_modulus, false)?;
            property_check(m.id, P::ShearModulus, m.shear_modulus, false)?;
            property_check(m.id, P::Area, m.area, true)?;
            property_check(m.id, P::SecondMomentY, m.second_moment_y, true)?;
            property_check(m.id, P::SecondMomentZ, m.second_moment_z, true)?;
            property_check(m.id, P::TorsionConstant, m.torsion_constant, true)?;
            if m.y_reference.iter().any(|c| !c.is_finite()) {
                return Err(SourceError::NonFiniteProperty {
                    member: m.id,
                    property: P::YReference,
                });
            }
            let (zero, parallel) = chord_checks(
                nodes[m.node_i as usize],
                nodes[m.node_j as usize],
                m.y_reference,
            );
            if zero {
                return Err(SourceError::ZeroLength { member: m.id });
            }
            if parallel {
                return Err(SourceError::DegenerateAxis { member: m.id });
            }
        }
        springs.sort_by_key(|s| s.id);
        for pair in springs.windows(2) {
            if pair[0].id == pair[1].id {
                return Err(SourceError::DuplicateSpringId { id: pair[0].id });
            }
        }
        for s in &springs {
            in_range(s.dof.node)?;
            if !s.stiffness.is_finite() || s.stiffness <= 0.0 {
                return Err(SourceError::NonPositiveSpring { id: s.id });
            }
        }
        directional_springs.sort_by_key(|s| s.id);
        for pair in directional_springs.windows(2) {
            if pair[0].id == pair[1].id {
                return Err(SourceError::DuplicateSpringId { id: pair[0].id });
            }
        }
        for s in &directional_springs {
            in_range(s.node)?;
            if springs.iter().any(|g| g.id == s.id) {
                return Err(SourceError::DuplicateSpringId { id: s.id });
            }
            if !s.stiffness.is_finite() || s.stiffness <= 0.0 {
                return Err(SourceError::NonPositiveSpring { id: s.id });
            }
            if s.direction.iter().any(|c| !c.is_finite()) {
                return Err(SourceError::NonFiniteDirection { id: s.id });
            }
            if s.direction.iter().all(|&c| c == 0.0) {
                return Err(SourceError::ZeroDirection { id: s.id });
            }
        }
        constraints.sort_by_key(|c| c.dof);
        let mut constrained = vec![None; node_count * DOF_PER_NODE];
        for c in &constraints {
            in_range(c.dof.node)?;
            if !c.value.is_finite() {
                return Err(SourceError::NonFiniteValue { dof: c.dof });
            }
            let slot = &mut constrained[c.dof.global()];
            if slot.is_some() {
                return Err(SourceError::DuplicateConstraint { dof: c.dof });
            }
            *slot = Some(c.value);
        }
        for l in &loads {
            in_range(l.dof.node)?;
            if !l.value.is_finite() {
                return Err(SourceError::NonFiniteValue { dof: l.dof });
            }
            if l.source_id.is_empty() {
                return Err(SourceError::EmptyLoadSource { dof: l.dof });
            }
        }
        loads.sort_by(|a, b| {
            (a.dof, a.source_id.as_bytes(), a.value.to_bits()).cmp(&(
                b.dof,
                b.source_id.as_bytes(),
                b.value.to_bits(),
            ))
        });
        stations.sort_by_key(|s| s.id);
        for pair in stations.windows(2) {
            if pair[0].id == pair[1].id {
                return Err(SourceError::DuplicateStationId { id: pair[0].id });
            }
        }
        for s in &stations {
            if members.binary_search_by_key(&s.member, |m| m.id).is_err() {
                return Err(SourceError::UnknownMember {
                    station: s.id,
                    member: s.member,
                });
            }
            if !s.fraction.is_finite() || !(0.0..=1.0).contains(&s.fraction) {
                return Err(SourceError::StationOutOfRange { id: s.id });
            }
        }
        supports.sort_by_key(|s| s.id);
        for pair in supports.windows(2) {
            if pair[0].id == pair[1].id {
                return Err(SourceError::DuplicateSupportId { id: pair[0].id });
            }
        }
        for group in &mut supports {
            in_range(group.node)?;
            group.springs.sort_unstable();
            group.springs.dedup();
            group.directional_springs.sort_unstable();
            group.directional_springs.dedup();
            let bad_restraint = (0..DOF_PER_NODE).any(|c| {
                group.restrained[c] && constrained[group.node as usize * DOF_PER_NODE + c].is_none()
            });
            let bad_spring = group.springs.iter().any(|id| {
                springs
                    .binary_search_by_key(id, |s| s.id)
                    .map_or(true, |k| springs[k].dof.node != group.node)
            });
            let bad_directional = group.directional_springs.iter().any(|id| {
                directional_springs
                    .binary_search_by_key(id, |s| s.id)
                    .map_or(true, |k| directional_springs[k].node != group.node)
            });
            if bad_restraint || bad_spring || bad_directional {
                return Err(SourceError::SupportMismatch { id: group.id });
            }
        }
        let free = node_count * DOF_PER_NODE - constraints.len();
        checked_profile_count(free)?;
        // Bodies: the connected components of the member graph.
        let mut parent: Vec<usize> = (0..node_count).collect();
        fn root(parent: &mut [usize], mut x: usize) -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        }
        for m in &members {
            let (a, b) = (
                root(&mut parent, m.node_i as usize),
                root(&mut parent, m.node_j as usize),
            );
            if a != b {
                let (low, high) = (a.min(b), a.max(b));
                parent[high] = low;
            }
        }
        let mut body_of_root = vec![u32::MAX; node_count];
        let mut body_of_node = vec![0u32; node_count];
        let mut body_count = 0u32;
        for node in 0..node_count {
            let r = root(&mut parent, node);
            if body_of_root[r] == u32::MAX {
                body_of_root[r] = body_count;
                body_count += 1;
            }
            body_of_node[node] = body_of_root[r];
        }
        #[cfg(any(test, feature = "mutation-controls"))]
        let (members, springs, directional_springs, constraints, loads, stations) =
            if super::seeded::active(super::seeded::Fault::F10) {
                caller_order
            } else {
                (
                    members,
                    springs,
                    directional_springs,
                    constraints,
                    loads,
                    stations,
                )
            };
        Ok(Self {
            nodes,
            members,
            springs,
            directional_springs,
            constraints,
            loads,
            stations,
            supports,
            constrained,
            body_of_node,
            body_count,
        })
    }

    pub fn nodes(&self) -> &[[f64; 3]] {
        &self.nodes
    }
    pub fn members(&self) -> &[StraightMember] {
        &self.members
    }
    pub fn springs(&self) -> &[Spring] {
        &self.springs
    }
    pub fn directional_springs(&self) -> &[DirectionalSpring] {
        &self.directional_springs
    }
    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }
    pub fn loads(&self) -> &[NodalLoad] {
        &self.loads
    }
    pub fn stations(&self) -> &[Station] {
        &self.stations
    }
    pub fn supports(&self) -> &[SupportGroup] {
        &self.supports
    }
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    pub fn dof_count(&self) -> usize {
        self.nodes.len() * DOF_PER_NODE
    }
    /// The constraint value of a global DOF, if constrained.
    pub fn constraint(&self, global: usize) -> Option<f64> {
        self.constrained[global]
    }
    /// The free global DOFs, ascending.
    pub fn free_dofs(&self) -> Vec<usize> {
        (0..self.dof_count())
            .filter(|&g| self.constrained[g].is_none())
            .collect()
    }
    pub fn body_of_node(&self, node: u32) -> u32 {
        self.body_of_node[node as usize]
    }
    pub fn body_count(&self) -> u32 {
        self.body_count
    }
    /// The nodes of a body, ascending.
    pub fn body_nodes(&self, body: u32) -> Vec<u32> {
        (0..self.nodes.len() as u32)
            .filter(|&n| self.body_of_node[n as usize] == body)
            .collect()
    }
    pub fn member_index(&self, id: u32) -> Option<usize> {
        self.members.binary_search_by_key(&id, |m| m.id).ok()
    }

    /// The canonical byte encoding (ROOT's K4 ruling Q10; hashed by F2a and
    /// V-K). Little-endian; binary64 values as their bits.
    pub fn encoding(&self) -> Vec<u8> {
        let mut out = b"K4SRC\x01".to_vec();
        self.encode_stiffness_part(&mut out, true);
        put_u32(&mut out, self.loads.len() as u32);
        for l in &self.loads {
            put_dof(&mut out, l.dof);
            put_u32(&mut out, l.source_id.len() as u32);
            out.extend_from_slice(l.source_id.as_bytes());
            put_f64(&mut out, l.value);
        }
        put_u32(&mut out, self.stations.len() as u32);
        for s in &self.stations {
            put_u32(&mut out, s.id);
            put_u32(&mut out, s.member);
            put_f64(&mut out, s.fraction);
        }
        put_u32(&mut out, self.supports.len() as u32);
        for g in &self.supports {
            put_u32(&mut out, g.id);
            put_u32(&mut out, g.node);
            out.extend(g.restrained.iter().map(|&r| u8::from(r)));
            put_u32(&mut out, g.springs.len() as u32);
            for &id in &g.springs {
                put_u32(&mut out, id);
            }
            put_u32(&mut out, g.directional_springs.len() as u32);
            for &id in &g.directional_springs {
                put_u32(&mut out, id);
            }
        }
        out
    }

    /// The factor-reuse identity (D1 §4.1.7): nodes, members, springs,
    /// directional springs and the constrained DOF set (not their values).
    pub fn stiffness_encoding(&self) -> Vec<u8> {
        let mut out = b"K4STF\x01".to_vec();
        self.encode_stiffness_part(&mut out, false);
        out
    }

    fn encode_stiffness_part(&self, out: &mut Vec<u8>, constraint_values: bool) {
        put_u32(out, self.nodes.len() as u32);
        for p in &self.nodes {
            for &c in p {
                put_f64(out, c);
            }
        }
        put_u32(out, self.members.len() as u32);
        for m in &self.members {
            put_u32(out, m.id);
            put_u32(out, m.node_i);
            put_u32(out, m.node_j);
            for v in [
                m.elastic_modulus,
                m.shear_modulus,
                m.area,
                m.second_moment_y,
                m.second_moment_z,
                m.torsion_constant,
            ] {
                put_f64(out, v);
            }
            for &c in &m.y_reference {
                put_f64(out, c);
            }
        }
        put_u32(out, self.springs.len() as u32);
        for s in &self.springs {
            put_u32(out, s.id);
            put_dof(out, s.dof);
            put_f64(out, s.stiffness);
        }
        put_u32(out, self.directional_springs.len() as u32);
        for s in &self.directional_springs {
            put_u32(out, s.id);
            put_u32(out, s.node);
            out.push(match s.kind {
                SpringKind::Translation => 0,
                SpringKind::Rotation => 1,
            });
            for &c in &s.direction {
                put_f64(out, c);
            }
            put_f64(out, s.stiffness);
        }
        put_u32(out, self.constraints.len() as u32);
        for c in &self.constraints {
            put_dof(out, c.dof);
            if constraint_values {
                put_f64(out, c.value);
            }
        }
    }
}

pub(crate) fn put_u32(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_le_bytes());
}
pub(crate) fn put_i64(out: &mut Vec<u8>, v: i64) {
    out.extend_from_slice(&v.to_le_bytes());
}
pub(crate) fn put_u64(out: &mut Vec<u8>, v: u64) {
    out.extend_from_slice(&v.to_le_bytes());
}
pub(crate) fn put_f64(out: &mut Vec<u8>, v: f64) {
    put_u64(out, v.to_bits());
}
pub(crate) fn put_dof(out: &mut Vec<u8>, dof: Dof) {
    put_u32(out, dof.node);
    out.push(dof.component.index() as u8);
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/source_tests.rs"]
mod tests;

#[cfg(test)]
mod checked_count_tests {
    use super::*;
    #[test]
    fn checked_work_profile_scalar_overflow_refuses_without_allocating() {
        assert!(checked_profile_count(0).is_ok());
        assert!(checked_profile_count(3).is_ok());
        assert!(matches!(
            checked_profile_count(usize::MAX),
            Err(SourceError::CountRange(_))
        ));
        if usize::BITS > 32 {
            assert!(matches!(
                checked_profile_count(u32::MAX as usize + 1),
                Err(SourceError::CountRange(_))
            ));
        }
    }
}
