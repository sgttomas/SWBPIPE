//! Checked, allocation-free arithmetic for the named kernel reference premises.
//!
//! This module does not qualify the running executable and is not an admission
//! adapter. Source, compiler, library, target, allocator request sites and input
//! provenance require separate external correspondence. It contains no observer,
//! environment gate, source/model/graph computation or evidence-file reader.
//!
//! The source construction contracts are single-case H and VR adapters with no
//! aggregate support groups and zero prescribed values. Axis and directional
//! springs are explicit VR inputs. Exact structural counts must describe the
//! same validated source. Scalar validation cannot establish that provenance.

use std::ops::{Add, Div, Mul, Sub};

/// Immutable identity of the reviewed mathematical fact bundle, not an attestation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KernelProofBasis {
    /// Numerical source 40129, pinned Rust 1.97.1 capacity contracts and the
    /// separately reviewed aarch64 request/type facts, including corrected owners.
    Source40129Rust1971Aarch64V1,
}

/// Private immutable facts. No caller-supplied request sizes or current-host sizes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReferenceKernelProfile {
    basis: KernelProofBasis,
}
impl ReferenceKernelProfile {
    pub const fn source40129_rust1971_aarch64_v1() -> Self {
        Self {
            basis: KernelProofBasis::Source40129Rust1971Aarch64V1,
        }
    }
    pub const fn proof_basis(self) -> KernelProofBasis {
        self.basis
    }
}

/// A named construction premise that the later adapter must establish.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceConstruction {
    /// Borrowed exact maps; IDs are the adapter's k6:decimal strings.
    HModelV1,
    /// Exact node clone, separate fresh pushed arrays and exact cloned load IDs.
    VrModelV1,
}

/// Population values are exact or explicitly use the monotone population uppers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PopulationPolicy {
    Exact {
        bodies: u128,
        free_blocks: u128,
    },
    /// B <= nodes and b <= free DOFs. Never used to reconstruct pattern/profile.
    NodesAndFreeDofsUpper,
}

/// Existing source/count outputs. Supplying these does not recompute a graph.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StructuralCounts {
    pub dofs: u128,
    pub free_dofs: u128,
    pub quantities: u128,
    pub source_encoding_bytes: u128,
    pub pattern_entries: u128,
    pub profile_entries: u128,
}

/// Scalar input metadata, with every load and UTF-8 source-ID byte retained.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KernelInput {
    pub nodes: u128,
    pub members: u128,
    pub axis_springs: u128,
    pub directional_springs: u128,
    pub constraints: u128,
    pub load_terms: u128,
    pub stations: u128,
    pub load_id_bytes: u128,
    pub max_load_id_bytes: u128,
    /// The selected source contracts require zero; not silently discarded.
    pub support_groups: u128,
    /// The selected source contracts require zero; outer headers still count.
    pub nonzero_prescribed_terms: u128,
    pub structure: Option<StructuralCounts>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnvelopeError {
    MissingDescriptor(&'static str),
    InvalidDescriptor(&'static str),
    MissingProof(&'static str),
    ArithmeticOverflow,
    ReferenceWidthExceeded,
    InternalPhaseCapacity,
}

/// Validated scalars with private fields. Actual source/count identity remains
/// the adapter's provenance obligation; this validates arithmetic relationships.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KernelDescriptor {
    input: KernelInput,
    structure: StructuralCounts,
    construction: SourceConstruction,
    population_policy: PopulationPolicy,
    bodies: u128,
    blocks: u128,
}
impl KernelDescriptor {
    pub fn new(
        input: KernelInput,
        construction: SourceConstruction,
        population_policy: PopulationPolicy,
    ) -> Result<Self, EnvelopeError> {
        let st = input
            .structure
            .ok_or(EnvelopeError::MissingDescriptor("structural counts"))?;
        let n = input
            .nodes
            .checked_mul(6)
            .ok_or(EnvelopeError::ArithmeticOverflow)?;
        if input.nodes == 0 {
            return Err(EnvelopeError::InvalidDescriptor("empty source"));
        }
        let f = n
            .checked_sub(input.constraints)
            .ok_or(EnvelopeError::InvalidDescriptor("constraints exceed DOFs"))?;
        if input.support_groups != 0 {
            return Err(EnvelopeError::MissingProof("aggregate support children"));
        }
        if input.nonzero_prescribed_terms != 0 {
            return Err(EnvelopeError::MissingProof("nonzero prescribed operands"));
        }
        let x = Shape::new(input, st, 0, 0);
        let q = (7 * x.n_node + 12 * x.m + 6 * x.t + x.axis + 3 * x.di + x.r).raw()?;
        let enc = (38
            + 24 * x.n_node
            + 84 * x.m
            + 17 * x.axis
            + 41 * x.di
            + 13 * x.r
            + 17 * x.l
            + x.ids
            + 16 * x.t)
            .raw()?;
        let pi = (144 * x.m + x.axis + 9 * x.di).raw()?;
        let square = n.checked_mul(n).ok_or(EnvelopeError::ArithmeticOverflow)?;
        let triangle = (C::new(f) * (C::new(f) + 1) / 2).raw()?;
        if st.dofs != n
            || st.free_dofs != f
            || st.quantities != q
            || st.source_encoding_bytes != enc
        {
            return Err(EnvelopeError::InvalidDescriptor(
                "inconsistent derived counts",
            ));
        }
        if st.pattern_entries > square.min(pi)
            || st.profile_entries < f
            || st.profile_entries > triangle
        {
            return Err(EnvelopeError::InvalidDescriptor(
                "pattern/profile population",
            ));
        }
        // The source encodings use u32 length/ID fields. Reject truncation.
        for count in [
            input.nodes,
            input.members,
            input.axis_springs,
            input.directional_springs,
            input.constraints,
            input.load_terms,
            input.stations,
            input.max_load_id_bytes,
        ] {
            if count > u128::from(u32::MAX) {
                return Err(EnvelopeError::ReferenceWidthExceeded);
            }
        }
        for count in [
            n,
            f,
            q,
            enc,
            st.pattern_entries,
            st.profile_entries,
            input.load_id_bytes,
        ] {
            if count > u128::from(u64::MAX) {
                return Err(EnvelopeError::ReferenceWidthExceeded);
            }
        }
        let id_max = input
            .load_terms
            .checked_mul(input.max_load_id_bytes)
            .ok_or(EnvelopeError::ArithmeticOverflow)?;
        // Every ID is nonempty and at least one attains the supplied maximum.
        let id_min = if input.load_terms == 0 {
            0
        } else {
            input
                .max_load_id_bytes
                .checked_add(input.load_terms - 1)
                .ok_or(EnvelopeError::ArithmeticOverflow)?
        };
        if (input.load_terms == 0 && (input.load_id_bytes != 0 || input.max_load_id_bytes != 0))
            || (input.load_terms != 0
                && (input.max_load_id_bytes == 0
                    || input.load_id_bytes < id_min
                    || input.load_id_bytes > id_max
                    || input.max_load_id_bytes > input.load_id_bytes))
        {
            return Err(EnvelopeError::InvalidDescriptor("load ID byte counts"));
        }
        if construction == SourceConstruction::HModelV1 {
            let min_ids = if input.load_terms == 0 {
                0
            } else {
                (C::new(input.load_terms - 1) * 4 + input.max_load_id_bytes).raw()?
            };
            let max_id = 3 + decimal_digits(n - 1);
            if input.axis_springs != 0
                || input.directional_springs != 0
                || input.stations != input.members
                || (input.load_terms != 0 && input.max_load_id_bytes < 4)
                || input.load_id_bytes < min_ids
                || input.max_load_id_bytes > max_id
            {
                return Err(EnvelopeError::InvalidDescriptor("H construction premise"));
            }
        }
        let (bodies, blocks) = match population_policy {
            PopulationPolicy::Exact {
                bodies,
                free_blocks,
            } => {
                if bodies == 0
                    || bodies > input.nodes
                    || free_blocks > f
                    || (free_blocks == 0) != (f == 0)
                {
                    return Err(EnvelopeError::InvalidDescriptor("body/block counts"));
                }
                (bodies, free_blocks)
            }
            PopulationPolicy::NodesAndFreeDofsUpper => (input.nodes, f),
        };
        Ok(Self {
            input,
            structure: st,
            construction,
            population_policy,
            bodies,
            blocks,
        })
    }
    pub const fn input(&self) -> KernelInput {
        self.input
    }
    pub const fn structural_counts(&self) -> StructuralCounts {
        self.structure
    }
    pub const fn construction(&self) -> SourceConstruction {
        self.construction
    }
    pub const fn population_policy(&self) -> PopulationPolicy {
        self.population_policy
    }
    pub const fn body_population(&self) -> u128 {
        self.bodies
    }
    pub const fn block_population(&self) -> u128 {
        self.blocks
    }
}
fn decimal_digits(mut value: u128) -> u128 {
    let mut n = 1;
    while value >= 10 {
        value /= 10;
        n += 1;
    }
    n
}

/// Requested and single-active-old moving request upper, not RSS/footprint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MetricBytes {
    pub requested: u128,
    pub moving: u128,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KernelPhase {
    GroupGeometry,
    PatternTagging,
    Ordering,
    FreeBlocks,
    Preparation,
    Scaffolding,
    SharedBuild,
    InitialSolve,
    Residual,
    Correction,
    Fallback,
    ChosenFallbackClone,
    Recovery,
    VerificationSharedBuild,
    VerificationRefusal,
    ResolutionTop,
    ResolutionHatCheck,
    FirstFormationScale,
    PassDeltaSolve,
    PassRecovery,
    PassFormationScale,
    ContributionOperand,
    Shift,
    ReportBuild,
    AttemptRefusalTransfer,
    StopRule,
    CanonicalBeforeRule,
    CanonicalCertificate,
    PublicationDraft,
    Certificate,
    SelectedFinish,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhaseId {
    pub phase: KernelPhase,
    pub precision: Option<u16>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhaseBound {
    pub id: PhaseId,
    pub bytes: MetricBytes,
}
const EMPTY_PHASE: PhaseBound = PhaseBound {
    id: PhaseId {
        phase: KernelPhase::GroupGeometry,
        precision: None,
    },
    bytes: MetricBytes {
        requested: 0,
        moving: 0,
    },
};
const MAX_PHASES: usize = 96;

/// All values are stationary after return: there is no kernel realloc in a
/// caller's later grow. Named finite padding is retained; this is not exact liveness.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ReturnedBounds {
    pub selected: u128,
    pub refused: u128,
    pub unresolved: u128,
    pub union: u128,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScheduleEnvelope {
    pub solve: MetricBytes,
    pub retained_prefix: u128,
    pub returned: ReturnedBounds,
    /// Slots are verification256,512,1024. Shortened schedule has only slot0.
    pub r7: [Option<MetricBytes>; 3],
    pub r7_max: MetricBytes,
    pub dominant_requested: PhaseId,
    pub dominant_moving: PhaseId,
    phases: [PhaseBound; MAX_PHASES],
    phase_count: usize,
}
impl ScheduleEnvelope {
    pub fn phases(&self) -> &[PhaseBound] {
        &self.phases[..self.phase_count]
    }
    fn new() -> Self {
        Self {
            solve: MetricBytes::default(),
            retained_prefix: 0,
            returned: ReturnedBounds::default(),
            r7: [None; 3],
            r7_max: MetricBytes::default(),
            dominant_requested: EMPTY_PHASE.id,
            dominant_moving: EMPTY_PHASE.id,
            phases: [EMPTY_PHASE; MAX_PHASES],
            phase_count: 0,
        }
    }
    fn put(
        &mut self,
        phase: KernelPhase,
        precision: Option<u16>,
        value: Pair,
    ) -> Result<(), EnvelopeError> {
        let bytes = value.finish()?;
        if self.phase_count == MAX_PHASES {
            return Err(EnvelopeError::InternalPhaseCapacity);
        }
        let id = PhaseId { phase, precision };
        self.phases[self.phase_count] = PhaseBound { id, bytes };
        self.phase_count += 1;
        if bytes.requested > self.solve.requested {
            self.solve.requested = bytes.requested;
            self.dominant_requested = id;
        }
        if bytes.moving > self.solve.moving {
            self.solve.moving = bytes.moving;
            self.dominant_moving = id;
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KernelEnvelope {
    pub proof_basis: KernelProofBasis,
    pub source_constructor: MetricBytes,
    /// source06 Base, including one original source and3328/64 minimum backings.
    pub base: u128,
    pub original_source: u128,
    pub prepared_source_clone: u128,
    /// Precision slots128,256,512,1024; success Arc payloads counted once.
    pub shared: [u128; 4],
    pub solved: [u128; 4],
    /// Verification slots256,512,1024.
    pub verification: [u128; 3],
    pub full: ScheduleEnvelope,
    pub selected128: ScheduleEnvelope,
}

// A failed checked operation poisons all subsequent arithmetic, including max.
// No overflowing branch is silently discarded in favor of a smaller alternative.
#[derive(Clone, Copy)]
struct C(Option<u128>);
impl C {
    const fn new(v: u128) -> Self {
        Self(Some(v))
    }
    fn raw(self) -> Result<u128, EnvelopeError> {
        self.0.ok_or(EnvelopeError::ArithmeticOverflow)
    }
    fn bytes(self) -> Result<u128, EnvelopeError> {
        let v = self.raw()?;
        if v > (1u128 << 63) - 1 {
            Err(EnvelopeError::ReferenceWidthExceeded)
        } else {
            Ok(v)
        }
    }
    fn max(self, b: Self) -> Self {
        Self(self.0.zip(b.0).map(|(a, b)| a.max(b)))
    }
    fn min(self, b: Self) -> Self {
        Self(self.0.zip(b.0).map(|(a, b)| a.min(b)))
    }
}
macro_rules! checked_op {
    ($trait:ident,$method:ident,$checked:ident) => {
        impl $trait for C {
            type Output = Self;
            fn $method(self, b: Self) -> Self {
                Self(self.0.zip(b.0).and_then(|(a, b)| a.$checked(b)))
            }
        }
        impl $trait<u128> for C {
            type Output = Self;
            fn $method(self, b: u128) -> Self {
                self.$method(C::new(b))
            }
        }
        impl $trait<C> for u128 {
            type Output = C;
            fn $method(self, b: C) -> C {
                C::new(self).$method(b)
            }
        }
    };
}
checked_op!(Add, add, checked_add);
checked_op!(Sub, sub, checked_sub);
checked_op!(Mul, mul, checked_mul);
checked_op!(Div, div, checked_div);
fn max(values: &[C]) -> C {
    values.iter().copied().fold(C::new(0), C::max)
}
fn minimum(stride: u128) -> u128 {
    if stride == 1 {
        8
    } else if stride <= 1024 {
        4
    } else {
        1
    }
}
fn capacity(stride: u128, k: C) -> C {
    C(k.0.and_then(|k| {
        if k == 0 {
            Some(0)
        } else {
            k.checked_next_power_of_two()
                .map(|p| p.max(minimum(stride)))
        }
    }))
}
fn g(stride: u128, k: C) -> C {
    stride * capacity(stride, k)
}
fn a(stride: u128, k: C) -> C {
    C(k.0.and_then(|k| {
        if k == 0 {
            Some(0)
        } else {
            k.checked_mul(2).map(|k| k.max(minimum(stride)))
        }
    })) * stride
}
fn old(stride: u128, k: C) -> C {
    match k.0 {
        Some(k) if k <= minimum(stride) => C::new(0),
        Some(k) => g(stride, C::new(k)) / 2,
        None => C(None),
    }
}
fn sort(stride: u128, k: C) -> C {
    C(k.0.map(|k| if k < 2 { 0 } else { k.max(48) })) * stride
}
fn tree(k: C, node: u128) -> C {
    match k.0 {
        Some(0) => C::new(0),
        Some(k) => (C::new(k) - 1) / 5 * node + node,
        None => C(None),
    }
}
#[derive(Clone, Copy)]
struct Pair {
    r: C,
    m: C,
}
impl Pair {
    fn new(r: C, old: C) -> Self {
        Self { r, m: r + old }
    }
    fn exact(r: C) -> Self {
        Self { r, m: r }
    }
    fn plus(self, r: C) -> Self {
        Self {
            r: self.r + r,
            m: self.m + r,
        }
    }
    fn add(self, b: Self) -> Self {
        Self {
            r: self.r + b.r,
            m: (self.m + b.r).max(self.r + b.m),
        }
    }
    fn max(self, b: Self) -> Self {
        Self {
            r: self.r.max(b.r),
            m: self.m.max(b.m),
        }
    }
    fn finish(self) -> Result<MetricBytes, EnvelopeError> {
        Ok(MetricBytes {
            requested: self.r.bytes()?,
            moving: self.m.bytes()?,
        })
    }
}
fn peak(values: &[Pair]) -> Pair {
    values
        .iter()
        .copied()
        .fold(Pair::exact(C::new(0)), Pair::max)
}
fn tracker(rows: C) -> (Pair, C) {
    let lazy = 4304 * capacity(4304, rows).min(C::new(512));
    let table = a(40, rows);
    let kept = g(40, rows);
    let lo = match rows.0 {
        Some(n) if n > 1 => lazy / 2,
        Some(_) => C::new(0),
        None => C(None),
    };
    (
        peak(&[
            Pair::new(lazy + table, lo),
            Pair::new(lazy + table, table / 2),
            Pair::exact(table + sort(40, rows)),
            Pair::new(table + kept, old(40, rows)),
        ]),
        lazy + table,
    )
}
fn tracker_set(q: C, forces: C, bodies: C) -> Pair {
    let offers = q + 2 * forces;
    let keys = 8 * bodies;
    let table = 40 * (2 * offers + 4 * keys);
    let (lazy, lo) = match offers.0 {
        Some(0) => (C::new(0), C::new(0)),
        Some(_) => (C::new(4352 * 4304), C::new(256 * 4304)),
        None => (C(None), C(None)),
    };
    let nodes = tree(keys, 1168) + tree(keys, 200);
    peak(&[
        Pair::new(lazy + table + nodes, lo),
        Pair::new(lazy + table + nodes, a(40, q) / 2),
        Pair::exact(lazy + table + nodes + sort(40, q)),
        Pair::new(lazy + table + nodes + g(40, q), old(40, q)),
    ])
}

#[derive(Clone, Copy)]
struct Shape {
    n_node: C,
    m: C,
    axis: C,
    di: C,
    r: C,
    l: C,
    t: C,
    ids: C,
    n: C,
    f: C,
    z: C,
    h: C,
    q: C,
    enc: C,
    bodies: C,
    blocks: C,
}
impl Shape {
    fn new(i: KernelInput, s: StructuralCounts, bodies: u128, blocks: u128) -> Self {
        Self {
            n_node: C::new(i.nodes),
            m: C::new(i.members),
            axis: C::new(i.axis_springs),
            di: C::new(i.directional_springs),
            r: C::new(i.constraints),
            l: C::new(i.load_terms),
            t: C::new(i.stations),
            ids: C::new(i.load_id_bytes),
            n: C::new(s.dofs),
            f: C::new(s.free_dofs),
            z: C::new(s.pattern_entries),
            h: C::new(s.profile_entries),
            q: C::new(s.quantities),
            enc: C::new(s.source_encoding_bytes),
            bodies: C::new(bodies),
            blocks: C::new(blocks),
        }
    }
}
const WIDTH: [u128; 4] = [48, 48, 80, 144];
const RESIDUAL_WIDTH: [u128; 4] = [48, 80, 144, 144];
const MEMBER: [u128; 4] = [7888, 7888, 13136, 23632];
const RESIDUAL_MEMBER: [u128; 4] = [7888, 13136, 23632, 23632];
const COEFFICIENT: [u128; 4] = [248, 248, 408, 728];
const RESIDUAL_COEFFICIENT: [u128; 4] = [248, 408, 728, 728];
const DIRECTIONAL: [u128; 4] = [440, 440, 728, 1304];
const RESIDUAL_DIRECTIONAL: [u128; 4] = [440, 728, 1304, 1304];
const PIVOT: [u128; 4] = [104, 104, 168, 296];
const BLOCK: [u128; 3] = [208, 336, 592];
const VERIFY_ARC: [u128; 3] = [560, 592, 656];
const BODY: [u128; 3] = [624, 1040, 1872];
const REPORT_BLOCK: [u128; 3] = [976, 1584, 2800];
const SHIFT_START: [u128; 3] = [64, 96, 160];
const SHIFT_RESULT: [u128; 3] = [272, 432, 752];
const PRECISION: [u16; 4] = [128, 256, 512, 1024];

struct Prepared {
    shape: Shape,
    original: C,
    source_clone: C,
    base: C,
    constructor: Pair,
    geometry: Pair,
    pattern: Pair,
    ordering: Pair,
    blocks: Pair,
    prep: Pair,
    scaffolding: Pair,
    shared: [C; 4],
    solved: [C; 4],
    verification: [C; 3],
    ledger_encoding: C,
}

fn h_constructor(
    original: C,
    nodes: C,
    members: C,
    constraints: C,
    loads: C,
    stations: C,
    max_id: C,
) -> Pair {
    peak(&[
        Pair::exact(
            original
                + max(&[
                    sort(88, members),
                    sort(16, constraints),
                    sort(40, loads),
                    sort(16, stations),
                ]),
        ),
        Pair::exact(original + 6432),
        Pair::exact(original + 12 * nodes),
        Pair::new(original + g(8, constraints), old(8, constraints)),
        Pair::new(original, max_id),
    ])
}

/// Raw H source window, including partial constructor failures. Unlike a successful
/// descriptor, raw restraints need not be unique or fit the model's DOF count.
pub(crate) fn h_source_constructor(
    nodes: u128,
    members: u128,
    constraints: u128,
    loads: u128,
    id_bytes: u128,
    max_id_bytes: u128,
) -> Result<MetricBytes, EnvelopeError> {
    for value in [nodes, members, constraints, loads] {
        if value > u128::from(u32::MAX) {
            return Err(EnvelopeError::ReferenceWidthExceeded);
        }
    }
    let (nn, m, r, l, ids) = (
        C::new(nodes),
        C::new(members),
        C::new(constraints),
        C::new(loads),
        C::new(id_bytes),
    );
    let original = 24 * nn + 88 * m + 16 * r + 40 * l + 16 * m + 16 * (6 * nn) + 4 * nn + 2 * ids;
    h_constructor(original, nn, m, r, l, m, C::new(max_id_bytes)).finish()
}

fn prepare(d: &KernelDescriptor) -> Prepared {
    let x = Shape::new(d.input, d.structure, d.bodies, d.blocks);
    let Shape {
        n_node: nn,
        m,
        axis,
        di,
        r,
        l,
        t,
        ids,
        n,
        f,
        z,
        h,
        q,
        enc,
        bodies: b,
        blocks: bc,
    } = x;
    let zero = C::new(0);
    let v = l.min(n);
    let limbs = 68 * v;
    let upper = 78 * m + axis + 6 * di;
    let raw_count = 144 * m + axis + 9 * di;
    let common_source = 24 * nn + 16 * n + 4 * nn;
    let original = match d.construction {
        SourceConstruction::VrModelV1 => {
            common_source
                + g(88, m)
                + g(24, axis)
                + g(48, di)
                + g(16, r)
                + g(40, l)
                + g(16, t)
                + ids
        }
        SourceConstruction::HModelV1 => common_source + 88 * m + 16 * r + 40 * l + 16 * t + 2 * ids,
    };
    let source_clone =
        common_source + 88 * m + 24 * axis + 48 * di + 16 * r + 40 * l + 16 * t + ids;
    let stf = 26 + 24 * nn + 84 * m + 17 * axis + 41 * di + 5 * r;
    let ledger_encoding = 10 + 18 * v + 8 * limbs;
    let geometry = 24 * b + 8 * (4 * b + 4 * nn);
    let geometry_copy = 24 * b + 16 * nn;
    let pattern = 8 * (n + 1) + a(8, z) + 8 * z + 8 * (z + 1) + 8 * upper;
    let ordering = g(8, f) + 8 * n + 24 * f;
    let blocks = 4 * f + g(24, bc) + 8 * (4 * bc + 2 * f) + 4 * bc;
    let prep = 432 + source_clone + 48 * v + v + 8 * limbs + 48 * r + a(1, enc) + g(20, q) + 8 * b;
    let group = 368 + geometry + pattern + ordering + blocks;
    let call = 1464 + 80 + a(1, stf);
    let base = original + prep + group + call + geometry_copy + 3328 + 64;
    let sort_input = max(&[
        sort(88, m),
        sort(24, axis),
        sort(48, di),
        sort(16, r),
        sort(40, l),
        sort(16, t),
    ]);
    let constructor = match d.construction {
        SourceConstruction::VrModelV1 => Pair {
            r: original + max(&[sort_input, C::new(6432), 12 * nn]),
            m: original
                + max(&[
                    sort_input,
                    C::new(6432),
                    12 * nn,
                    old(88, m),
                    old(24, axis),
                    old(48, di),
                    old(16, r),
                    old(40, l),
                    old(16, t),
                ]),
        },
        SourceConstruction::HModelV1 => {
            h_constructor(original, nn, m, r, l, t, C::new(d.input.max_load_id_bytes))
        }
    };
    let grounds = r + axis + 6 * nn;
    let common = g(4, nn) + tree(nn, 240) + 24 * nn + g(8, grounds);
    let assessment = g(24, nn)
        + g(48, grounds)
        + 48 * grounds
        + g(48, 2 * nn + 5)
        + g(192, nn)
        + nn * (3 * g(8, C::new(10)) + 3 * g(8, C::new(1)))
        + 3 * g(8, C::new(2))
        + g(8, C::new(10))
        + g(48, nn)
        + 80
        + g(8, C::new(11));
    let assessment_old = max(&[
        old(4, nn),
        old(8, grounds),
        old(24, nn),
        old(48, grounds),
        old(48, 2 * nn + 5),
        old(192, nn),
        old(48, nn),
        old(8, 2 * nn),
        g(8, C::new(11)) / 2,
    ]);
    let dircap = if d.input.directional_springs == 0 {
        zero
    } else {
        24 * max(&[capacity(24, di), C::new(4), 2 * (di + 3)])
    };
    let geometry = peak(&[
        Pair::new(common + assessment, assessment_old),
        Pair::new(
            common + dircap,
            max(&[dircap / 2, old(4, nn), old(8, grounds), old(8, 2 * nn)]),
        ),
    ]);
    let positions = g(16, upper);
    let raw = 24 * n + 8 * (4 * n + 2 * raw_count);
    let pattern = peak(&[
        Pair::new(
            positions + raw,
            max(&[old(16, upper), a(8, z) / 2, old(8, raw_count)]),
        ),
        Pair::new(positions + g(16, upper) + 8 * (z + 1), old(16, upper)),
    ]);
    let pred_f = if d.structure.free_dofs == 0 {
        zero
    } else {
        f - 1
    };
    let edges = (f * pred_f).min(raw_count);
    let adjacency = 24 * f + 8 * (4 * f + 2 * edges);
    let neighbors = 24 * f + 8 * (4 * f + 4 * edges);
    let qmax = if d.structure.free_dofs == 0 {
        zero
    } else {
        pred_f.max(C::new(1))
    };
    let queue = Pair::new(g(8, qmax), old(8, qmax));
    let reach_old = match d.structure.free_dofs {
        0 | 1 => zero,
        2..=4 => C::new(8),
        _ => g(8, f) / 2,
    };
    let reach = Pair::new(f + g(8, f) + queue.r, reach_old.max(queue.m - queue.r));
    let ecc = Pair::new(f + 4 * g(8, f), old(8, f).max(reach_old));
    // Returned order8f is already in Base: extra degrees/visited are9f, not17f.
    let ordering = peak(&[reach, ecc, queue, Pair::exact(sort(8, pred_f))])
        .plus(adjacency + neighbors + 9 * f)
        .max(Pair::new(
            adjacency + neighbors + 9 * f,
            max(&[old(8, pred_f), old(8, 2 * pred_f), old(8, f)]),
        ));
    let blocks = Pair::new(g(8, f), max(&[reach_old, old(24, bc), old(8, f)]));
    let prep = peak(&[
        Pair::new(g(1104, v), old(1104, v)),
        Pair::new(g(4, nn) + 24 * nn, old(4, nn)),
        Pair::new(zero, enc),
        Pair::new(zero, old(20, q)),
    ]);
    let scaffolding = Pair::new(zero, stf.max(old(8, 2 * nn)));
    let mut shared = [zero; 4];
    let mut solved = [zero; 4];
    let mut verification = [zero; 3];
    for i in 0..4 {
        let w = WIDTH[i];
        let wr = RESIDUAL_WIDTH[i];
        shared[i] = 720
            + m * MEMBER[i]
            + di * DIRECTIONAL[i]
            + z * (w + wr)
            + m * RESIDUAL_COEFFICIENT[i]
            + di * RESIDUAL_DIRECTIONAL[i]
            + h * w
            + f * (48 + PIVOT[i])
            + if i > 0 { bc * w } else { zero };
        solved[i] = 104 + (n + 6 * m + q) * w;
        if i > 0 {
            verification[i - 1] =
                VERIFY_ARC[i - 1] + z * w + 144 * m * wr + 9 * di * wr + bc * BLOCK[i - 1];
        }
    }
    Prepared {
        shape: x,
        original,
        source_clone,
        base,
        constructor,
        geometry,
        pattern,
        ordering,
        blocks,
        prep,
        scaffolding,
        shared,
        solved,
        verification,
        ledger_encoding,
    }
}

/// Evaluate the complete shared kernel component. No partial value is returned
/// on overflow, invalid input or an unsupported construction premise.
///
/// Exact graph/input provenance and eventual executable correspondence remain
/// external obligations. This result must not replace an H/VR global adapter.
pub fn kernel_envelope(
    d: &KernelDescriptor,
    profile: &ReferenceKernelProfile,
) -> Result<KernelEnvelope, EnvelopeError> {
    let p = prepare(d);
    let mut shared = [0; 4];
    let mut solved = [0; 4];
    let mut verification = [0; 3];
    for i in 0..4 {
        shared[i] = p.shared[i].bytes()?;
        solved[i] = p.solved[i].bytes()?;
    }
    for i in 0..3 {
        verification[i] = p.verification[i].bytes()?;
    }
    Ok(KernelEnvelope {
        proof_basis: profile.basis,
        source_constructor: p.constructor.finish()?,
        base: p.base.bytes()?,
        original_source: p.original.bytes()?,
        prepared_source_clone: p.source_clone.bytes()?,
        shared,
        solved,
        verification,
        full: schedule(&p, 4)?,
        selected128: schedule(&p, 2)?,
    })
}
fn schedule(p: &Prepared, count: usize) -> Result<ScheduleEnvelope, EnvelopeError> {
    use KernelPhase::*;
    let Shape {
        n_node: _,
        m,
        axis,
        di,
        r,
        l: _,
        t: _,
        ids: _,
        n,
        f,
        z,
        h,
        q,
        enc,
        bodies: b,
        blocks: bc,
    } = p.shape;
    let zero = C::new(0);
    let mut k = p.base;
    for i in 0..count {
        k = k + p.shared[i] + p.solved[i];
    }
    for i in 1..count {
        k = k + p.verification[i - 1] + 40 * b + g(24, 2 * bc) + 24 * bc;
    }
    let mut out = ScheduleEnvelope::new();
    out.retained_prefix = k.bytes()?;
    for (phase, value) in [
        (GroupGeometry, p.geometry),
        (PatternTagging, p.pattern),
        (Ordering, p.ordering),
        (FreeBlocks, p.blocks),
        (Preparation, p.prep),
        (Scaffolding, p.scaffolding),
    ] {
        out.put(phase, None, value.plus(k))?;
    }
    let mut selected = zero;
    for i in 0..count {
        let precision = Some(PRECISION[i]);
        let w = WIDTH[i];
        let wr = RESIDUAL_WIDTH[i];
        let c = capacity(w, f);
        let (tr, held) = tracker(f);
        let residual_members = if i < 3 { m * RESIDUAL_MEMBER[i] } else { zero };
        let build = peak(&[
            Pair::exact(z),
            Pair::exact(residual_members + z),
            Pair::exact(f * w),
            Pair::exact(5 * f * w + 24 * bc + 8 * f),
            tr,
        ]);
        out.put(SharedBuild, precision, build.plus(k))?;
        let helper = Pair::new((2 * c + f) * w, old(w, f));
        let prefix = f * w + c * w + 96 + 4 * f * w;
        let residual = f * (w + 16);
        out.put(InitialSolve, precision, helper.plus(k + f * w))?;
        out.put(Residual, precision, tr.plus(k + prefix + residual))?;
        out.put(
            Correction,
            precision,
            tr.add(helper).plus(k + prefix + residual + f * w),
        )?;
        let active5 = tr.plus(4 * held);
        let fallback_rows = Pair::new(g(4304, f), old(4304, f));
        let fallback_state = active5.add(fallback_rows).plus(n * w + 4 * (4288 + 96));
        let fallback_blocks = Pair::exact(144 * m * wr + held);
        out.put(
            Fallback,
            precision,
            fallback_state
                .max(fallback_blocks)
                .plus(k + prefix + residual + z * wr),
        )?;
        out.put(
            ChosenFallbackClone,
            precision,
            Pair::exact(k + prefix + residual + held + f * w),
        )?;
        out.put(
            Recovery,
            precision,
            Pair::exact(k + prefix + (12 * m + axis + 3 * di + n) * w),
        )?;
        if i == 0 {
            continue;
        }
        let vi = i - 1;
        let widened = if i < 3 {
            m * RESIDUAL_MEMBER[i] + di * RESIDUAL_DIRECTIONAL[i]
        } else {
            zero
        };
        let vbuild = peak(&[
            Pair::exact(m * COEFFICIENT[i] + 16 * bc + 144 * m * w),
            Pair::exact(m * COEFFICIENT[i] + 16 * bc + widened),
            Pair::exact(
                m * COEFFICIENT[i] + 16 * bc + (4 * f * w + 4 * f).max(2 * f * w + 2 * bc * w),
            ),
        ]);
        out.put(VerificationSharedBuild, precision, vbuild.plus(k))?;
        out.put(
            VerificationRefusal,
            precision,
            Pair::new(k + 16 * bc + g(24, bc), old(24, bc)),
        )?;
        let res = g(16, b);
        let pass = 3 * n * w + (7 * f + c) * w + (q + 6 * m) * w + 50 * n + res;
        // Source-construction premises give zero child operands, not zero headers.
        out.put(
            ResolutionTop,
            precision,
            Pair::new(k + n * w + q * w + 2 * b * w + res, old(16, b)),
        )?;
        out.put(
            ResolutionHatCheck,
            precision,
            Pair::new(k + n * w + q * w + 2 * res, old(16, b)),
        )?;
        out.put(
            FirstFormationScale,
            precision,
            Pair::exact(k + n * w + q * w + (18 * m + axis + 3 * di + n) * w),
        )?;
        let early = 3 * q * w;
        out.put(
            PassDeltaSolve,
            precision,
            helper.plus(k + pass + early - c * w),
        )?;
        out.put(
            PassRecovery,
            precision,
            Pair::exact(k + pass + early + (12 * m + axis + 3 * di + n) * w),
        )?;
        out.put(
            PassFormationScale,
            precision,
            Pair::exact(k + pass + early + (18 * m + axis + 3 * di + n) * w),
        )?;
        out.put(
            ContributionOperand,
            precision,
            Pair::exact(k + pass + early + wr),
        )?;
        let profile = h * w + 36 * f;
        let shifted = h * w + (32 + w) * f + bc;
        let start = g(SHIFT_START[vi], bc);
        let controls =
            bc + 16 * bc + start + SHIFT_RESULT[vi] * bc + a(8, bc) + 16 * bc + start + bc * w;
        let shift_extra = max(&[f * w, 3 * f * w, f * w + bc * w + start]);
        // The previous factor drops before retry; current/next are distinct Vecs.
        out.put(
            Shift,
            precision,
            Pair::new(
                k + pass + early + profile + shifted + controls + shift_extra,
                old(SHIFT_START[vi], bc).max(a(8, bc) / 2),
            ),
        )?;
        let report = 344 + 5 * q * w + (f + c) * w + REPORT_BLOCK[vi] * bc + BODY[vi] * b + res;
        let caller_controls = bc + 16 * bc + start + SHIFT_RESULT[vi] * bc;
        // Cancel moved RES/r_hat/delta by identity BEFORE substituting upper B/b.
        let pass_rest = 3 * n * w + 6 * f * w + (q + 6 * m) * w + 50 * n;
        out.put(
            ReportBuild,
            precision,
            Pair::new(
                k + pass_rest + report + caller_controls + g(24, bc),
                old(24, bc).max(old(SHIFT_START[vi], bc)),
            ),
        )?;
        out.put(
            AttemptRefusalTransfer,
            precision,
            Pair::new(k + report + g(24, bc), old(24, 2 * bc)),
        )?;
        let decision = g(16, 4 * b) + 2 * g(16, 2 * b) + if i == 3 { 16 * b } else { zero };
        let ts = tracker_set(q, q - 7 * p.shape.n_node, b);
        let rule = Pair::exact(q + 8 * b * w)
            .max(ts.plus(q + 4 * b * w + 16 * b + decision))
            .max(Pair::new(
                q + 4 * b * w + 16 * b + decision + ts.r,
                old(16, 4 * b).max(old(16, 2 * b)),
            ));
        let rb = rule.finish()?;
        out.r7[vi] = Some(rb);
        out.r7_max.requested = out.r7_max.requested.max(rb.requested);
        out.r7_max.moving = out.r7_max.moving.max(rb.moving);
        out.put(StopRule, precision, rule.plus(k + report))?;
        out.put(
            CanonicalBeforeRule,
            precision,
            Pair::new(k + report + g(20, q), old(20, q)),
        )?;
        out.put(
            CanonicalCertificate,
            precision,
            Pair::new(k + report + decision + g(20, q), old(20, q)),
        )?;
        let publication = g(64, q);
        let scale = a(16, 4 * b);
        let radius = 8 * q;
        out.put(
            PublicationDraft,
            precision,
            Pair::new(
                k + report + decision + 24 * q + 64 * b + publication + scale,
                old(64, q).max(scale / 2),
            ),
        )?;
        let certificate = publication + scale + radius;
        out.put(
            Certificate,
            precision,
            Pair::exact(k + report + decision + certificate),
        )?;
        let limbs = match i {
            0 | 1 => 4,
            2 => 8,
            _ => 16,
        };
        let rst = 22 + (n + 6 * m) * (9 + 8 * limbs);
        // Independent finish clones; optional floor and both filter vectors are
        // explicitly padded even where their simultaneous logical maxima cannot occur.
        let finish = 1976
            + 40 * b
            + (40 * b + 48 * bc)
            + enc
            + a(1, p.ledger_encoding)
            + a(1, rst)
            + 128 * b
            + 64 * b
            + 8 * r
            + g(24, q)
            + g(40, q)
            + 40 * b
            + g(16, b)
            + 24 * b;
        let finish_old = max(&[p.ledger_encoding, rst, old(24, q), old(40, q), old(16, b)]);
        out.put(
            SelectedFinish,
            precision,
            Pair::new(k + report + decision + certificate + finish, finish_old),
        )?;
        // Report/Decision are not added as stationary returned owners.
        selected = selected.max(k + certificate + finish);
    }
    out.returned = ReturnedBounds {
        selected: selected.bytes()?,
        refused: k.bytes()?,
        unresolved: k.bytes()?,
        union: selected.max(k).bytes()?,
    };
    Ok(out)
}
