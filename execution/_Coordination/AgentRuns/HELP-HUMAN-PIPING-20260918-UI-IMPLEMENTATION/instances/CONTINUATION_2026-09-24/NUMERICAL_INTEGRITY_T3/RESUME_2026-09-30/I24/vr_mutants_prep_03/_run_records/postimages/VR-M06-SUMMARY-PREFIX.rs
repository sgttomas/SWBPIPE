//! Complete conditional VR caller composition, using the shared kernel envelope.
//! Source40129/Rust1971/aarch64 reference facts are mathematical premises;
//! they do not qualify the current executable, allocator, input or launch.
use crate::cases::{Case, ControlKind, Model};
use crate::scale::Counts;
use open_pipe_stress_frame_kernel::structural::retained_api::PrimitiveSource;
use open_pipe_stress_solver_performance_harness::k6::w1::envelope::{
    kernel_envelope, KernelDescriptor, KernelInput, SourceConstruction, StructuralCounts,
};
pub use open_pipe_stress_solver_performance_harness::k6::w1::envelope::{
    EnvelopeError, PopulationPolicy, ReferenceKernelProfile,
};
use serde_json::Value;
use std::ops::{Add, Div, Mul, Sub};

// Poisoned arithmetic: an overflow in any alternative cannot be hidden by max.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct N(Option<i128>);
impl From<i32> for N {
    fn from(v: i32) -> Self {
        Self(Some(v as i128))
    }
}
impl From<i128> for N {
    fn from(v: i128) -> Self {
        Self(Some(v))
    }
}
impl From<usize> for N {
    fn from(v: usize) -> Self {
        Self(Some(v as i128))
    }
}
impl N {
    fn max(self, b: impl Into<N>) -> Self {
        Self(self.0.zip(b.into().0).map(|(a, b)| a.max(b)))
    }
    fn min(self, b: impl Into<N>) -> Self {
        Self(self.0.zip(b.into().0).map(|(a, b)| a.min(b)))
    }
    fn raw(self) -> Result<i128, EnvelopeError> {
        self.0.ok_or(EnvelopeError::ArithmeticOverflow)
    }
    fn bytes(self) -> Result<u128, EnvelopeError> {
        let x = self.raw()?;
        if !(0..=i64::MAX as i128).contains(&x) {
            Err(EnvelopeError::ReferenceWidthExceeded)
        } else {
            Ok(x as u128)
        }
    }
}
macro_rules! op {
    ($tr:ident,$m:ident,$c:ident) => {
        impl $tr for N {
            type Output = Self;
            fn $m(self, b: Self) -> Self {
                Self(self.0.zip(b.0).and_then(|(a, b)| a.$c(b)))
            }
        }
        impl $tr<i128> for N {
            type Output = Self;
            fn $m(self, b: i128) -> Self {
                self.$m(N::from(b))
            }
        }
        impl $tr<N> for i128 {
            type Output = N;
            fn $m(self, b: N) -> N {
                N::from(self).$m(b)
            }
        }
    };
}
op!(Add, add, checked_add);
op!(Sub, sub, checked_sub);
op!(Mul, mul, checked_mul);
op!(Div, div, checked_div);
fn num(v: usize) -> N {
    v.into()
}
fn mx(v: &[N]) -> N {
    v.iter().copied().fold(0.into(), N::max)
}
fn sum(v: impl Iterator<Item = N>) -> N {
    v.fold(0.into(), |a, b| a + b)
}
fn mu(s: i128) -> i128 {
    if s == 1 {
        8
    } else if s <= 1024 {
        4
    } else {
        1
    }
}
fn p(s: i128, k: N) -> N {
    N(k.0.and_then(|k| {
        if k == 0 {
            Some(0)
        } else if k < 0 {
            None
        } else {
            (k as u128)
                .checked_next_power_of_two()
                .and_then(|v| i128::try_from(v).ok())
                .map(|v| v.max(mu(s)))
        }
    }))
}
fn g(s: i128, k: N) -> N {
    s * p(s, k)
}
fn old(s: i128, k: N) -> N {
    match k.0 {
        Some(k) if k <= mu(s) => 0.into(),
        _ => g(s, k) / 2,
    }
}
fn a(s: i128, k: N) -> N {
    match k.0 {
        Some(0) => 0.into(),
        _ => s * (2 * k).max(mu(s)),
    }
}
fn sort(s: i128, k: N) -> N {
    match k.0 {
        Some(k) if k < 2 => 0.into(),
        _ => s * k.max(48),
    }
}
fn fmt(d: N) -> N {
    match d.0 {
        Some(0) => 0.into(),
        _ => (2 * d).max(8),
    }
}
fn tree(k: N, leaf: i128, internal: i128) -> N {
    match k.0 {
        Some(0) => 0.into(),
        Some(k) if k <= 11 => leaf.into(),
        _ => (1 + (k - 1) / 5) * leaf.max(internal),
    }
}
fn bulk(k: N, leaf: i128, internal: i128) -> N {
    if k.0 == Some(0) {
        return 0.into();
    }
    let mut ret = (1 + k / 12) * leaf;
    let mut v = N::from(12);
    while matches!(k.0.zip(v.0),Some((k,v)) if k>=v) {
        ret = ret + (1 + k / (v * 12)) * internal;
        v = v * 12;
    }
    ret
}
fn jt(k: N) -> N {
    tree(k, 632, 728)
}
fn json_shape(v: &Value) -> (N, N, bool) {
    match v {
        Value::String(s) => (num(s.len()), 0.into(), true),
        Value::Array(xs) => {
            let (mut r, mut o, mut valid) = (g(32, num(xs.len())), old(32, num(xs.len())), true);
            for x in xs {
                let (h, j, b) = json_shape(x);
                r = r + h;
                o = o.max(j);
                valid &= b;
            }
            (r, o, valid)
        }
        Value::Object(xs) => {
            let (mut r, mut o, mut valid) = (jt(num(xs.len())), N::from(0), true);
            for (k, v) in xs {
                let (h, j, b) = json_shape(v);
                r = r + num(k.len()) + h;
                o = o.max(j);
                valid &= b;
            }
            (r, o, valid)
        }
        Value::Number(x) => (0.into(), 0.into(), x.as_u64().is_some()),
        _ => (0.into(), 0.into(), true),
    }
}
fn model_heap(m: &Model, cloned: bool) -> (N, N) {
    let nn = num(m.nodes.len());
    let chars = sum(m.node_names.iter().map(|s| num(s.len())))
        + sum(m.members.iter().map(|x| num(x.name.len())))
        + sum(m.springs.iter().map(|x| num(x.key.len())))
        + sum(m.omitted_springs.iter().map(|s| num(s.len())))
        + sum(m.loads.iter().map(|x| num(x.3.len())));
    (
        (if cloned { 48 * nn } else { 2 * g(24, nn) })
            + 112 * num(m.members.len())
            + 88 * num(m.springs.len())
            + 24 * num(m.omitted_springs.len())
            + 16 * num(m.constraints.len())
            + 48 * num(m.loads.len())
            + 16 * num(m.stations.len())
            + chars,
        if cloned { 0.into() } else { old(24, nn) },
    )
}
fn case_heap(c: &Case) -> (N, N, N) {
    let mut r = sum([&c.id, &c.family, &c.basis, &c.units, &c.k4src_sha256]
        .into_iter()
        .map(|s| num(s.len())))
        + num(c.model_sha256.as_ref().map_or(0, String::len));
    r = r
        + 96 * num(c.rows.len())
        + sum(c.rows.iter().map(|x| {
            num(x.key.len())
                + num(x.expected.len())
                + num(x.class.len())
                + num(x.scale.as_ref().map_or(0, String::len))
        }))
        + 64 * num(c.controls.len());
    for x in &c.controls {
        r = r
            + num(x.id.len())
            + match &x.kind {
                ControlKind::Value(v) => {
                    48 * num(v.len()) + sum(v.iter().map(|(k, v)| num(k.len()) + num(v.len())))
                }
                ControlKind::Outcome(s) => num(s.len()),
            };
    }
    r = r + 24 * num(c.not_covered.len()) + sum(c.not_covered.iter().map(|s| num(s.len())));
    let mut extra = N::from(0);
    let mut grow = N::from(0);
    for map in [Some(&c.scales), c.s_full.as_ref()].into_iter().flatten() {
        let k = num(map.len());
        let nodes = bulk(k, 544, 640);
        r = r + nodes + sum(map.iter().map(|(k, v)| num(k.len()) + num(v.len())));
        extra = extra.max(48 * k + sort(48, k).max(nodes) - nodes);
    }
    if let Some(m) = &c.model {
        let (h, o) = model_heap(m, false);
        r = r + h;
        grow = grow.max(o)
    }
    (r, r + extra, r + extra.max(grow))
}

// A non-authenticating scalar consistency tag; exact raw input identity remains
// in the external source binding, not inferred from this tag or a family name.
fn family_tag(s: &str) -> u64 {
    s.bytes().fold(0xcbf29ce484222325, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(0x100000001b3)
    })
}

/// Inline facts captured during the original family load, without retaining raw text.
#[derive(Clone, Copy, Debug)]
pub struct FamilyInputFacts {
    raw: N,
    count: N,
    retained: N,
    prefix: [N; 2],
    filename_bytes: N,
    family_tag: Option<u64>,
    valid: bool,
}
impl Default for FamilyInputFacts {
    fn default() -> Self {
        Self {
            raw: 0.into(),
            count: 0.into(),
            retained: 0.into(),
            prefix: [0.into(); 2],
            filename_bytes: 0.into(),
            family_tag: None,
            valid: false,
        }
    }
}
impl FamilyInputFacts {
    pub(crate) fn begin(raw: &str, filename_bytes: usize) -> Self {
        Self {
            raw: num(raw.len()),
            filename_bytes: num(filename_bytes),
            valid: !raw.as_bytes().contains(&b'\\'),
            ..Self::default()
        }
    }
    pub(crate) fn observe(&mut self, v: &Value, c: &Case) {
        let tag = family_tag(&c.family);
        self.valid &= self.family_tag.is_none_or(|old| old == tag);
        self.family_tag = Some(tag);
        let (j, o, valid) = json_shape(v);
        let (r, hr, hm) = case_heap(c);
        self.valid &= valid;
        self.count = self.count + 1;
        self.prefix[0] = self.prefix[0].max(self.retained + mx(&[j, j + hr, r]));
        self.prefix[1] = self.prefix[1].max(self.retained + mx(&[j + o, j + hm, r]));
        self.retained = self.retained + r;
    }
    fn load(self, manifest: N, e: i128) -> N {
        let paths = manifest
            + a(1, manifest + 6)
            + a(1, manifest + 7 + self.filename_bytes)
            + e * (manifest + 7 + self.filename_bytes);
        let best = self.prefix[e as usize].max(self.retained + e * old(472, self.count));
        paths + read(self.raw, e).max(read(self.raw, 0) + g(472, self.count) + best)
    }
}
/// The original external model read/parse/hash operands, reduced to inline facts.
#[derive(Clone, Copy, Debug)]
pub struct ExternalModelFacts {
    raw: N,
    json: N,
    old: N,
    model: N,
    model_old: N,
    valid: bool,
}
impl ExternalModelFacts {
    pub(crate) fn capture(raw: &str, v: &Value, m: &Model) -> Self {
        let (json, old, valid) = json_shape(v);
        let (model, model_old) = model_heap(m, false);
        Self {
            raw: num(raw.len()),
            json,
            old,
            model,
            model_old,
            valid: valid && !raw.as_bytes().contains(&b'\\'),
        }
    }
}
/// All consumed argv tokens, including argv[0] and overwritten arguments.
#[derive(Clone, Copy, Debug, Default)]
pub struct ArgumentFacts {
    count: u128,
    bytes: u128,
    executable_bytes: u128,
    overflow: bool,
}
impl ArgumentFacts {
    pub fn observe(&mut self, s: &str) {
        if self.count == 0 {
            self.executable_bytes = s.len() as u128;
        }
        match (
            self.count.checked_add(1),
            self.bytes.checked_add(s.len() as u128),
        ) {
            (Some(c), Some(b)) => {
                self.count = c;
                self.bytes = b
            }
            _ => self.overflow = true,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InvocationKind {
    SingleCaseFamilyReferenceV1,
    ActualNormal,
    CountsContinuation,
}
#[derive(Clone, Copy, Debug)]
pub struct VrInvocation<'a> {
    kind: InvocationKind,
    manifest: &'a str,
    case: &'a str,
    model_path: Option<&'a str>,
    args: ArgumentFacts,
}
impl<'a> VrInvocation<'a> {
    pub fn actual(
        manifest: &'a str,
        case: &'a str,
        model_path: Option<&'a str>,
        args: ArgumentFacts,
        counts_only: bool,
    ) -> Self {
        Self {
            kind: if counts_only {
                InvocationKind::CountsContinuation
            } else {
                InvocationKind::ActualNormal
            },
            manifest,
            case,
            model_path,
            args,
        }
    }
    pub fn single_case_family_reference(case: &'a str) -> Self {
        let mut args = ArgumentFacts::default();
        for s in [
            "/reference/target/release/examples/vk_scale",
            "--case",
            case,
            "--heap-cap-bytes",
            "8053063680",
        ] {
            args.observe(s)
        }
        Self {
            kind: InvocationKind::SingleCaseFamilyReferenceV1,
            manifest:
                "/reference/projects/chirality-piping/validation/benchmarks/numerical_robustness",
            case,
            model_path: None,
            args,
        }
    }
    fn argv(self) -> N {
        let actual =
            N(i128::try_from(self.args.count).ok()) * 24 + N(i128::try_from(self.args.bytes).ok());
        if self.kind == InvocationKind::ActualNormal {
            return actual;
        }
        let ext = self.model_path.map_or(0, |s| s.len());
        let canonical = 24 * N::from(if self.model_path.is_some() { 7 } else { 5 })
            + N(i128::try_from(self.args.executable_bytes).ok())
            + 6
            + num(self.case.len())
            + 16
            + 20
            + if self.model_path.is_some() {
                12 + num(ext)
            } else {
                0.into()
            };
        if self.kind == InvocationKind::SingleCaseFamilyReferenceV1 {
            actual.max(canonical).max(canonical + 24 + 13 - 11)
        } else {
            actual.max(canonical)
        }
    }
}
/// Source-derived descriptors borrow only owners that survive the source drop.
///
/// The caller supplies the source made by this model's `source_parts`, existing
/// counts of that source, and the history from its actual loaders. Capture checks
/// scalar consistency; it does not recompute graphs/encoding or certify arbitrary
/// model/source pairs. The ordinary CLI retains its existing model/K4SRC checks.
/// Exact input identity, finite row/numerator premises and build correspondence
/// remain the external reference-profile obligations.
pub struct VrEstimateContext<'a> {
    case: &'a Case,
    model: &'a Model,
    family: FamilyInputFacts,
    external: Option<ExternalModelFacts>,
    invocation: VrInvocation<'a>,
    descriptor: KernelDescriptor,
    counts: Counts,
}
impl<'a> VrEstimateContext<'a> {
    pub fn capture(
        case: &'a Case,
        model: &'a Model,
        source: &PrimitiveSource,
        counts: &Counts,
        family: FamilyInputFacts,
        external: Option<ExternalModelFacts>,
        invocation: VrInvocation<'a>,
        policy: PopulationPolicy,
    ) -> Result<Self, EnvelopeError> {
        if invocation.case != case.id
            || (invocation.kind != InvocationKind::SingleCaseFamilyReferenceV1
                && case.family != "RF-LARGE")
        {
            return Err(EnvelopeError::InvalidDescriptor("case/invocation identity"));
        }
        if !family.valid
            || family.family_tag != Some(family_tag(&case.family))
            || external.is_some_and(|f| !f.valid)
        {
            return Err(EnvelopeError::MissingProof(
                "missing family input or escaped/non-u64 parser scratch",
            ));
        }
        if invocation.args.count == 0 {
            return Err(EnvelopeError::MissingDescriptor(
                "argument history including argv[0]",
            ));
        }
        if invocation.args.overflow {
            return Err(EnvelopeError::ArithmeticOverflow);
        }
        if case.model.is_none() != external.is_some()
            || external.is_some() != invocation.model_path.is_some()
        {
            return Err(EnvelopeError::InvalidDescriptor(
                "model construction history",
            ));
        }
        let ids = sum(source.loads().iter().map(|l| num(l.source_id.len()))).bytes()?;
        let maxid = source
            .loads()
            .iter()
            .map(|l| l.source_id.len())
            .max()
            .unwrap_or(0) as u128;
        let input = KernelInput {
            nodes: source.node_count() as u128,
            members: source.members().len() as u128,
            axis_springs: source.springs().len() as u128,
            directional_springs: source.directional_springs().len() as u128,
            constraints: source.constraints().len() as u128,
            load_terms: source.loads().len() as u128,
            stations: source.stations().len() as u128,
            load_id_bytes: ids,
            max_load_id_bytes: maxid,
            support_groups: source.supports().len() as u128,
            nonzero_prescribed_terms: source
                .constraints()
                .iter()
                .filter(|c| c.value != 0.0)
                .count() as u128,
            structure: Some(StructuralCounts {
                dofs: counts.dofs as u128,
                free_dofs: counts.free_dofs as u128,
                quantities: counts.rows as u128,
                source_encoding_bytes: counts.source_encoding_len as u128,
                pattern_entries: counts.pattern_entries as u128,
                profile_entries: counts.profile_entries as u128,
            }),
        };
        if counts.nodes != source.node_count()
            || counts.members != source.members().len()
            || counts.springs != source.springs().len() + source.directional_springs().len()
            || counts.constraints != source.constraints().len()
            || counts.loads != source.loads().len()
            || counts.stations != source.stations().len()
            || counts.bodies != source.body_count() as usize
            || model.nodes.len() != counts.nodes
            || model.members.len() != counts.members
            || model.springs.len() != counts.springs
            || model.constraints.len() != counts.constraints
            || model.loads.len() != counts.loads
            || model.stations.len() != counts.stations
        {
            return Err(EnvelopeError::InvalidDescriptor(
                "source/model/count identity",
            ));
        }
        if let PopulationPolicy::Exact {
            bodies,
            free_blocks,
        } = policy
        {
            if bodies != counts.bodies as u128 || free_blocks != counts.blocks as u128 {
                return Err(EnvelopeError::InvalidDescriptor(
                    "exact body/block provenance",
                ));
            }
        }
        if model.nodes != source.nodes()
            || (invocation.kind != InvocationKind::SingleCaseFamilyReferenceV1
                && counts.springs != 0)
        {
            return Err(EnvelopeError::InvalidDescriptor("VR construction premise"));
        }
        let descriptor = KernelDescriptor::new(input, SourceConstruction::VrModelV1, policy)?;
        Ok(Self {
            case,
            model,
            family,
            external,
            invocation,
            descriptor,
            counts: counts.clone(),
        })
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VrProofBasis {
    Source40129Rust1971Aarch64V1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReferenceVRProfile {
    basis: VrProofBasis,
}
impl ReferenceVRProfile {
    pub const fn proof_basis(self) -> VrProofBasis {
        self.basis
    }
    pub const fn source40129_rust1971_aarch64_v1() -> Self {
        Self {
            basis: VrProofBasis::Source40129Rust1971Aarch64V1,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Estimate {
    /// Standalone Model children plus the lane publication map.
    pub model: u128,
    /// Maximum R7 transient, excluding kernel prefix/current report/caller.
    pub decide: u128,
    /// Kernel Base plus the retained comparison-interface caller owners.
    /// A diagnostic component, never added again to a complete global maximum.
    pub fixed: u128,
    /// Complete prefix/kernel/returned/late/output maximum for all precisions.
    pub max: u128,
    /// Complete 128/256/v256 schedule with the same caller alternatives.
    pub sel128: u128,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CompleteEstimate {
    pub proof_basis: VrProofBasis,
    pub population_policy: PopulationPolicy,
    /// Fifteen complete caller-only alternatives, in CALLER_PHASES order.
    pub caller_requested: [u128; 15],
    pub caller_moving: [u128; 15],
    /// Solve then four stationary returned-outcome interface addends.
    pub joins_requested: [u128; 5],
    pub joins_moving: [u128; 5],
    pub requested: Estimate,
    pub moving: Estimate,
    pub context: InvocationKind,
}
fn read(l: N, e: i128) -> N {
    if l.0 == Some(0) {
        0.into()
    } else {
        (2 * (l + 32)).max(8) + e * l
    }
}
fn hash(l: N, e: i128) -> N {
    let d = 64 * ((l + 72) / 64);
    let m = (2 * d).max(8);
    if e == 0 {
        m + 144
    } else {
        (m + d).max(m + 208)
    }
}
fn encoding(x: N, e: i128) -> N {
    a(1, x) + e * x
}

// Reviewed Nat bit/exponent/capacity inequalities; never evaluates a predicate.
#[derive(Clone, Copy)]
struct Exact {
    bits: N,
    limbs: N,
    cap: N,
    alo: N,
    ahi: N,
    blo: N,
    bhi: N,
}
fn state(bits: N, alo: N, ahi: N, blo: N, bhi: N, cap: Option<N>) -> Exact {
    let limbs = (bits + 31) / 32;
    Exact {
        bits,
        limbs,
        cap: cap.unwrap_or(limbs),
        alo,
        ahi,
        blo,
        bhi,
    }
}
fn rr(x: Exact) -> N {
    4 * x.cap
}
fn clone_exact(x: Exact, da: i128, db: i128) -> Exact {
    state(x.bits, x.alo + da, x.ahi + da, x.blo + db, x.bhi + db, None)
}
fn decimal(s: &str) -> Result<(Exact, N), EnvelopeError> {
    let b = s.as_bytes();
    let mut i = usize::from(matches!(b.first(), Some(b'+' | b'-')));
    let (mut digits, mut frac, mut nonzero, mut point) = (0usize, 0usize, false, false);
    while i < b.len() && b[i] != b'e' && b[i] != b'E' {
        match b[i] {
            b'0'..=b'9' => {
                digits += 1;
                frac += usize::from(point);
                nonzero |= b[i] != b'0'
            }
            b'.' if !point => point = true,
            _ => return Err(EnvelopeError::InvalidDescriptor("decimal lexeme")),
        }
        i += 1;
    }
    if digits == 0 {
        return Err(EnvelopeError::InvalidDescriptor("decimal lexeme"));
    }
    let exponent = if i < b.len() {
        s[i + 1..]
            .parse::<i32>()
            .map_err(|_| EnvelopeError::InvalidDescriptor("decimal exponent"))? as i128
    } else {
        0
    };
    let bits = if nonzero { 4 * num(digits) } else { 0.into() };
    let j = N::from(exponent) - num(frac);
    Ok((
        state(bits, 0.into(), 0.into(), j, j, Some(p(4, (bits + 31) / 32))),
        num(digits),
    ))
}
fn lift(x: Exact, ka: N, kb: N, e: i128) -> (Exact, N) {
    if x.bits.0 == Some(0) {
        return (
            state(0.into(), 0.into(), 0.into(), 0.into(), 0.into(), None),
            0.into(),
        );
    }
    let bits = x.bits + ka;
    let ll = (bits + 31) / 32;
    let w = ka / 32;
    let k = w + x.limbs + if ka.0 == Some(0) { 0 } else { 1 };
    let cs = w.max(4).max(2 * k);
    let hs = 4 * cs + e * 4 * k;
    let b2 = bits + 4 * kb;
    let l2 = (b2 + 31) / 32;
    let cp = if kb.0 == Some(0) {
        ll
    } else {
        ll.max(4).max(2 * l2)
    };
    let hp = 4 * cp
        + if kb.0 == Some(0) {
            0.into()
        } else {
            e * 4 * l2
        };
    (
        state(b2, 0.into(), 0.into(), 0.into(), 0.into(), Some(cp)),
        hs.max(4 * cs + hp),
    )
}
fn align(x: Exact, y: Exact, e: i128) -> (Exact, Exact, N, N, N, N, N) {
    let lo = x.alo.min(y.alo);
    let blo = x.blo.min(y.blo);
    let (xx, hx) = lift(x, (x.ahi - lo).max(0), (x.bhi - blo).max(0), e);
    let (yy, hy) = lift(y, (y.ahi - lo).max(0), (y.bhi - blo).max(0), e);
    (
        xx,
        yy,
        hx.max(rr(xx) + hy),
        lo,
        x.ahi.max(y.ahi),
        blo,
        x.bhi.max(y.bhi),
    )
}
fn cmp(x: Exact, y: Exact, e: i128) -> N {
    align(x, y, e).2
}
fn plus(x: Exact, y: Exact, e: i128) -> (Exact, N) {
    let (a, b, h, lo, hi, jlo, jhi) = align(x, y, e);
    let bits = a.bits.max(b.bits) + 1;
    let cap = a.limbs.max(b.limbs) + 1;
    (
        state(bits, lo, hi, jlo, jhi, Some(cap)),
        h.max(rr(a) + rr(b) + 4 * cap),
    )
}
fn minus(x: Exact, y: Exact, e: i128) -> (Exact, N) {
    let ny = clone_exact(y, 0, 0);
    let (r, h) = plus(x, ny, e);
    (r, rr(ny) + h)
}
fn times(x: Exact, y: Exact, e: i128) -> (Exact, N) {
    let q = x.limbs + y.limbs + 1;
    let cap = (2 * q).max(4);
    if x.bits.0 == Some(0) || y.bits.0 == Some(0) {
        return (
            state(
                0.into(),
                x.alo + y.alo,
                x.ahi + y.ahi,
                x.blo + y.blo,
                x.bhi + y.bhi,
                Some(0.into()),
            ),
            0.into(),
        );
    }
    (
        state(
            x.bits + y.bits,
            x.alo + y.alo,
            x.ahi + y.ahi,
            x.blo + y.blo,
            x.bhi + y.bhi,
            Some(cap),
        ),
        8 * q + 4 * cap + e * 4 * q,
    )
}
fn exact_max(x: Exact, y: Exact, e: i128) -> (Exact, N) {
    let r = state(
        x.bits.max(y.bits),
        x.alo.min(y.alo),
        x.ahi.max(y.ahi),
        x.blo.min(y.blo),
        x.bhi.max(y.bhi),
        Some(x.limbs.max(y.limbs)),
    );
    (r, cmp(x, y, e).max(rr(r)))
}
fn tol(x: Exact, y: Exact, e: i128) -> (Exact, N) {
    let ax = clone_exact(x, 0, 0);
    let (m, h) = exact_max(ax, y, e);
    let t = clone_exact(m, 0, -9);
    (t, (rr(ax) + h).max(rr(ax) + rr(m) + rr(t)))
}
fn pred(o: Exact, x: Exact, y: Exact, e: i128) -> N {
    let (d, hd) = minus(o, x, e);
    let ad = clone_exact(d, 0, 0);
    let (t, ht) = tol(x, y, e);
    mx(&[
        hd,
        rr(d) + rr(ad),
        rr(d) + rr(ad) + ht,
        rr(d) + rr(ad) + rr(t) + cmp(ad, t, e),
    ])
}
fn f64_state() -> Exact {
    state(
        53.into(),
        (-1074).into(),
        971.into(),
        0.into(),
        0.into(),
        Some(2.into()),
    )
}
fn magnitude(x: Exact, y: Exact, e: i128) -> N {
    let f = f64_state();
    let (q1, h1) = times(f, f, e);
    let (q2, h2) = times(f, f, e);
    let (q, hq) = plus(q1, q2, e);
    let (t, ht) = tol(x, y, e);
    let (u, hu) = plus(x, t, e);
    let (u2, hu2) = times(u, u, e);
    let (l, hl) = minus(x, t, e);
    let (l2, hl2) = times(l, l, e);
    16 + mx(&[
        h1,
        rr(q1) + h2,
        rr(q1) + rr(q2) + hq,
        rr(q) + ht,
        rr(q) + rr(t) + hu,
        rr(q) + rr(t) + rr(u) + hu2.max(rr(u2) + cmp(q, u2, e)),
        rr(q) + rr(t) + rr(u) + hl,
        rr(q) + rr(t) + rr(u) + rr(l) + hl2.max(rr(l2) + cmp(q, l2, e)),
    ])
}
fn floor_cost(x: Exact, y: Exact, e: i128) -> N {
    let a = clone_exact(x, 0, 0);
    let (c, h) = exact_max(a, y, e);
    let f = clone_exact(f64_state(), -34, 0);
    mx(&[rr(a) + h, rr(c) + 8 + rr(f), rr(c) + rr(f) + cmp(c, f, e)])
}
fn range_cost(x: Exact, e: i128) -> N {
    let a = clone_exact(x, 0, 0);
    let tiny = state(
        1.into(),
        (-1075).into(),
        (-1075).into(),
        0.into(),
        0.into(),
        Some(2.into()),
    );
    let p = state(
        1.into(),
        1024.into(),
        1024.into(),
        0.into(),
        0.into(),
        Some(2.into()),
    );
    let v = state(
        1.into(),
        970.into(),
        970.into(),
        0.into(),
        0.into(),
        Some(2.into()),
    );
    let (hug, h) = minus(p, v, e);
    mx(&[
        rr(a),
        rr(a) + 24 + h,
        rr(a) + 8 + rr(hug) + cmp(a, tiny, e).max(cmp(a, hug, e)),
    ])
}
fn parse_cost(d: N, e: i128) -> N {
    let j = (d + 8) / 9;
    let l = (4 * d + 31) / 32;
    mx(&[
        fmt(d) + e * d,
        fmt(d) + a(16, j) + e * 16 * j,
        fmt(d) + a(16, j) + g(4, l) + e * 4 * l,
    ])
}
fn row_scale<'a>(c: &'a Case, row: &'a crate::cases::Row) -> Result<&'a str, EnvelopeError> {
    row.scale
        .as_deref()
        .or_else(|| c.scales.get(&row.class).map(String::as_str))
        .ok_or(EnvelopeError::MissingDescriptor("row comparison scale"))
}
fn exact_case(c: &Case, e: i128) -> Result<(N, N, N), EnvelopeError> {
    let (mut rowmax, mut ctrlmax, mut heldmax) = (N::from(0), N::from(0), N::from(0));
    for row in &c.rows {
        let (x, dx) = decimal(&row.expected)?;
        let (y, dy) = decimal(row_scale(c, row)?)?;
        let held = rr(x) + rr(y);
        heldmax = heldmax.max(held);
        rowmax = rowmax.max(mx(&[
            parse_cost(dx, e),
            rr(x) + parse_cost(dy, e),
            held + floor_cost(x, y, e),
            held + 8 + pred(f64_state(), x, y, e),
            held + magnitude(x, y, e),
            held + range_cost(x, e),
        ]));
    }
    for co in &c.controls {
        if let ControlKind::Value(v) = &co.kind {
            for (key, obs) in v {
                let row = c
                    .rows
                    .iter()
                    .find(|r| &r.key == key)
                    .ok_or(EnvelopeError::InvalidDescriptor("control row"))?;
                let (x, dx) = decimal(&row.expected)?;
                let (y, dy) = decimal(row_scale(c, row)?)?;
                let (z, dz) = decimal(obs)?;
                ctrlmax = ctrlmax.max(mx(&[
                    parse_cost(dx, e),
                    rr(x) + parse_cost(dy, e),
                    rr(x) + rr(y) + parse_cost(dz, e),
                    rr(x) + rr(y) + rr(z) + pred(z, x, y, e),
                ]));
            }
        }
    }
    Ok((rowmax, ctrlmax, heldmax))
}
fn obj(k: i128, keys: i128, children: N) -> N {
    jt(k.into()) + keys + children
}
fn output(k: i128, keys: i128, children: N, phase: bool) -> N {
    let shift = if phase { 4 } else { 0 };
    jt((k + shift + 1).into())
        + keys
        + if phase { 45 } else { 0 }
        + 17
        + children
        + if phase { jt(4.into()) } else { 0.into() }
        + 97
}
fn record(c: &Case, b: N, d: N, e: i128) -> (N, N, N) {
    let geom = 2 + 23 * b + 27 * d.min(2 * num(c.model.as_ref().map_or(0, |m| m.nodes.len())));
    let stage = obj(19, 159, 0.into());
    let storage = obj(3, 45, 0.into());
    let verify = obj(5, 57, 0.into());
    let tally = obj(7, 76, 0.into());
    let attempt = obj(18, 222, 25 + 171 + 38 + 2 * stage + storage);
    let ret = mx(&[
        obj(
            16,
            188,
            17 + num(c.id.len())
                + num(c.family.len())
                + 64
                + 16
                + geom
                + 128
                + 4 * attempt
                + 3 * verify
                + tally,
        ),
        obj(
            9,
            75,
            17 + num(c.id.len())
                + num(c.family.len())
                + 64
                + 100
                + geom
                + 128
                + 4 * attempt
                + 3 * verify
                + tally,
        ),
        obj(
            7,
            47,
            17 + num(c.id.len()) + num(c.family.len()) + 64 + 89 + tally,
        ),
    ]);
    let out = 100 + geom;
    let hf = mx(&[
        fmt(89.into()) + e * 89,
        fmt(89.into()) + fmt(out) + e * out,
        fmt(out) + out,
        fmt(geom) + (e * geom).max(geom),
        fmt(171.into()) + N::from(171).max(e * 171),
    ]);
    (ret, ret + mx(&[stage, verify, tally, hf]), out)
}
fn rcm(f: N, edges: N, e: i128, six: bool) -> N {
    let neighbors = 24 * f + 32 * edges + 32 * f;
    let base = neighbors + 17 * f;
    let q = if f.0 == Some(0) {
        0.into()
    } else {
        g(8, (f - 1).max(1))
    };
    let qo = if f.0 == Some(0) {
        0.into()
    } else {
        old(8, (f - 1).max(1))
    };
    let ro = match f.0 {
        Some(f) if f <= 1 => 0.into(),
        Some(f) if f <= 4 => 8.into(),
        _ => old(8, f),
    };
    let reach = f + g(8, f) + q + e * ro.max(qo);
    let ecc = f + (if six { 6 } else { 4 }) * g(8, f) + e * old(8, f);
    base + mx(&[
        e * old(8, 2 * (f - 1).max(0)),
        sort(8, (f - 1).max(0)),
        reach,
        ecc,
        q + e * qo,
    ])
}
fn adjacency(n: N, f: N, edges: N, e: i128) -> (N, N) {
    let nodes = if edges.0 == Some(0) {
        0.into()
    } else {
        (f + edges / 5) * 200
    };
    let r = 80 * f + 16 * edges;
    let h = 9 * n + 24 * f + nodes + r + a(8, 12.into()) + e * N::from(96).max(8 * (f - 1).max(0));
    (r, h)
}
fn sparse(
    mdl: &Model,
    nn: N,
    m: N,
    n: N,
    f: N,
    r: N,
    s: N,
    d: N,
    e: i128,
    pattern_upper: N,
) -> Result<N, EnvelopeError> {
    if d.0 != Some(0) {
        if mdl.springs.first().is_none_or(|s| s.axis.is_some()) {
            return Err(EnvelopeError::MissingProof(
                "directional first-spring return",
            ));
        }
        let prefix = g(32, nn) + g(136, m);
        let grows = old(32, nn).max(old(136, m));
        return Ok((prefix + (e * grows).max(41)).max(prefix + 66144 + e * grows.max(33072)));
    }
    let contributions = 144 * m + s;
    let z = pattern_upper;
    let zf = (f * f).min(z);
    let lower = zf;
    let hs = f * (f + 1) / 2;
    let tau = m + s;
    let terms = 12 * m + s;
    let k = r.min(n);
    let j = f.min(n);
    let b = g(136, m) + g(16, s) + 8 * n + g(8, f) + 16 * r;
    let hb = b
        + g(32, nn)
        + 8 * r
        + mx(&[
            sort(16, r),
            e * old(32, nn),
            e * old(136, m),
            e * old(16, s),
            e * old(8, f),
        ]);
    let con = g(24, contributions);
    let intended = tree(z, 368, 464);
    let neighbors = 56 * nn + 64 * m;
    let rows = 56 * n + 16 * z;
    let pattern = 8 * (n + 1) + a(8, z) + 8 * z;
    let hp = pattern + neighbors + n + rows + e * mx(&[8 * 4 * m, 8 * n, 8 * z]);
    let kg = pattern + 8 * z;
    let formation = 1168 * m + 16 * m + 8 * s;
    let ha = formation + hp.max(kg);
    let w = b + con + intended + kg + 2 * g(8, z) + 8 * n;
    let front = mx(&[
        hb,
        b + con + e * old(24, contributions),
        b + con + 2 * intended,
        b + con + intended + ha,
        w + e * old(8, z),
    ]);
    let sums = 64 * z + 16 * contributions;
    let hsums = sums + g(8, tau) + e * old(8, tau);
    let diff = 80 * z + 16 * contributions;
    let hdiff = diff + g(8, tau + 1) + e * old(8, tau + 1);
    let rnd = g(96, z) + 8 * (2 * contributions + z);
    let rndclone = 96 * z + 8 * (2 * contributions + z);
    let audit = mx(&[
        hsums,
        sums + hdiff,
        sums + diff + rnd + e * old(96, z),
        sums + diff + rnd + g(16, k) + e * old(16, k),
    ]);
    let basis = nbytes("V-K parity: the structural adapter's formation allowances");
    let core = 4 * f + 8 * (f + 1) + 3 * g(8, zf) + 8 * zf + 16 * f + basis;
    let prep = core + rnd;
    let rowprep = (g(16, j) + e * old(16, j).max(old(8, zf))).max(g(24, k) + e * old(24, k));
    let hprep = mx(&[n, 4 * f + 8 * n, core + 8 * n + rowprep.max(audit)]);
    let entries = a(24, lower);
    let adj = 56 * f + 16 * zf;
    let hrcm = rcm(f, zf, e, true);
    let profile = f.max(24 * f + 8 * (f + 1) + a(8, hs) + e * 8 * hs);
    let order = mx(&[
        entries + e * 24 * lower,
        entries + adj + e * old(8, (f - 1).max(0)),
        entries + adj + hrcm,
        entries + adj + 8 * f + profile,
    ]);
    let factor = 40 * f + 8 * hs + g(48, f);
    let pivot = 48 * f;
    let hfactor = (n + 16 * f).max(factor + 41 * f + e * old(48, f));
    let residual = g(104, f);
    let res = n.max(residual + e * old(104, f));
    let rowterms = 1 + 2 * terms;
    let hinted = hsums.max(
        sums + residual
            + (2 * g(8, rowterms) + e * old(8, rowterms)).max(g(8, rowterms) + e * old(104, f)),
    );
    let finish = mx(&[
        n,
        n.max(40 * f),
        16 * f,
        8 * f + 8 * n + res,
        8 * f + 8 * n + residual + g(8, f) + e * old(8, f),
        8 * f + 8 * n + residual + g(8, f) + 16 * f,
        8 * f + 8 * n + residual + hinted,
        8 * f + 8 * n + 2 * residual + 4 * f + rndclone + basis,
    ]);
    let solver = mx(&[
        hprep,
        prep + order,
        prep + hfactor,
        prep + factor + pivot + finish,
        prep + n.max(8 * f + 8 * n),
    ]);
    let solution = 8 * n + 4 * f + pivot + 2 * residual + rndclone + basis;
    Ok(mx(&[
        front,
        w + solver,
        w + solution.max(8 * n) + 200 + e * 100,
        front + 66144 + e * 33072,
    ]))
}
fn nbytes(s: &str) -> N {
    num(s.len())
}

struct Caller {
    phases: [N; 15],
    joins: [N; 5],
    model: N,
    fixed: N,
}
fn caller(c: &VrEstimateContext<'_>, e: i128) -> Result<Caller, EnvelopeError> {
    let case = c.case;
    let mdl = c.model;
    let k = &c.counts;
    let id = nbytes(&case.id);
    let family = nbytes(&case.family);
    let rows = num(case.rows.len());
    let controls = num(case.controls.len());
    let nc = num(case.not_covered.len());
    let keymax = mx(&case
        .rows
        .iter()
        .map(|r| nbytes(&r.key))
        .fold([N::from(0)], |[a], b| [a.max(b)]));
    let exmax = case
        .rows
        .iter()
        .map(|r| nbytes(&r.expected))
        .fold(N::from(0), N::max);
    let keysum = sum(case.rows.iter().map(|r| nbytes(&r.key)));
    let names = sum(mdl.members.iter().map(|m| nbytes(&m.name)));
    let external = c.external.is_some();
    let argsret = id + num(c.invocation.model_path.map_or(0, str::len));
    let (rm, _) = model_heap(mdl, !external);
    let rc = case_heap(case).0;
    let base = 1700 + argsret + id + rc + rm + if external { 128 } else { 0 };
    let cut = base + 128;
    let (nn, m, n, f, r, z, q, x) = (
        num(k.nodes),
        num(k.members),
        num(k.dofs),
        num(k.free_dofs),
        num(k.constraints),
        num(k.pattern_entries),
        num(k.rows),
        num(k.source_encoding_len),
    );
    let s = num(mdl.springs.iter().filter(|s| s.axis.is_some()).count());
    let d = num(mdl.springs.iter().filter(|s| s.axis.is_none()).count());
    // Exact and reference-upper populations are explicit kernel descriptor policy.
    let b = N(i128::try_from(c.descriptor.body_population()).ok());
    let pi = 144 * m + s + 9 * d;
    let edges = (f * (f - 1).max(0)).min(pi);
    let ids = sum(mdl.loads.iter().map(|l| nbytes(&l.3)));
    // Fixed-input reached-type premise checked without constructing an adjacency.
    for spring in &mdl.springs {
        if !mdl
            .members
            .iter()
            .any(|m| m.node_i == spring.node || m.node_j == spring.node)
        {
            return Err(EnvelopeError::MissingProof("untouched_node_triple_set"));
        }
    }
    let source = 24 * nn
        + g(88, m)
        + g(24, s)
        + g(48, d)
        + g(16, r)
        + g(40, num(k.loads))
        + g(16, num(k.stations))
        + 16 * n
        + 4 * nn
        + ids;
    let parts = source - 16 * n - 4 * nn;
    let (exactrow, exactctrl, held) = exact_case(case, e)?;
    let (rec, hrec, dout) = record(case, b, d, e);
    let why = mx(&[9.into(), 40.into(), 75.into(), keymax + 18]);
    let dfail = mx(&[
        id + keymax + exmax + 6 + why,
        id + keymax + 94,
        id + 93,
        id + 56,
        id + 36,
        id + 39 + dout,
        id + 75,
        id + 20,
        id + 16 + dout,
    ]);
    let failures = rows + 4;
    let ctrlchildren = sum(case.controls.iter().map(|x| fmt(id + 1 + nbytes(&x.id))));
    let ctrldiag = 2 * g(24, controls) + ctrlchildren;
    let rowdiag = g(24, failures)
        + failures * fmt(dfail)
        + g(24, rows)
        + keysum
        + 2 * g(24, 2 * rows)
        + 2 * rows * (fmt(id + keymax + 92) + fmt(id + keymax + 1));
    let diag = ctrldiag + rowdiag;
    let floorret = tree(m, 456, 552) + names;
    let floorh = names + (40 * m + sort(40, m)).max(40 * m + bulk(m, 456, 552));
    let controlh = (24 * rows + sort(24, rows)).max(24 * rows + bulk(rows, 280, 376));
    let lookup = tree(rows, 280, 376);
    let dctrl = id
        + 1
        + case
            .controls
            .iter()
            .map(|x| nbytes(&x.id))
            .fold(N::from(0), N::max);
    let control = controlh
        .max(lookup + ctrldiag + mx(&[exactctrl, e * old(24, controls), fmt(dctrl) + e * dctrl]));
    let published = tree(q, 848, 944);
    let ncfull = if case.s_full.is_some() {
        g(24, rows) + keysum
    } else {
        0.into()
    };
    let grow = mx(&[
        old(24, failures),
        old(24, 2 * rows),
        old(24, rows),
        old(24, controls),
        dfail,
        id + keymax + 92,
    ]);
    let diagphase = mx(&[
        exactrow,
        N::from(if e == 1 { 375 } else { 300 }),
        held + fmt(dfail) + e * dfail + fmt(why).max(75),
        held + e * grow,
    ]);
    let comparison = diag + published + 4 * nn + floorret + diagphase.max(0);
    let compbuild = mx(&[
        diag + published + 16 * nn,
        diag + published + 4 * nn + floorh,
        comparison,
    ]);
    let manifest = nbytes(c.invocation.manifest);
    let listpath = manifest + 6 + nbytes("/expected_unresolved.json");
    let expectedpaths = manifest + a(1, manifest + 6) + a(1, listpath) + e * listpath;
    // Exact fixed expected-list input: 1357 raw bytes, reviewed JSON tree and scratch.
    let expected = expectedpaths
        + mx(&[
            if listpath.0.is_some_and(|l| l < 384) {
                0.into()
            } else {
                listpath + 1
            },
            read(1357.into(), e),
            read(1357.into(), 0)
                + expected_json()
                + mx(&[
                    e * expected_json_old(),
                    78.into(),
                    N::from(if e == 1 { 120 } else { 80 }),
                ]),
        ]);
    let (radj, hadj) = adjacency(n, f, edges, e);
    let countprof = rcm(f, edges, e, false).max(24 * f + g(8, f) + e * old(8, f));
    let counts = mx(&[
        hadj,
        radj + countprof,
        g(8, f) + e * old(8, f),
        g(8, f) + nn + tree(2 * m, 104, 200),
        g(8, f) + g(20, q) + e * old(20, q),
        g(8, f) + g(20, q) + encoding(x, e),
    ]);
    let sourceh = source
        + mx(&[
            sort(88, m),
            sort(24, s),
            sort(48, d),
            sort(16, r),
            sort(40, num(k.loads)),
            sort(16, num(k.stations)),
            6432.into(),
            12 * nn,
            e * mx(&[
                old(88, m),
                old(24, s),
                old(48, d),
                old(16, r),
                old(40, num(k.loads)),
                old(16, num(k.stations)),
            ]),
        ]);
    let mut load = 1700 + argsret + id + c.family.load(manifest, e);
    if let Some(ext) = c.external {
        let plen = num(c.invocation.model_path.unwrap().len());
        let extpeak = mx(&[
            if plen.0.is_some_and(|l| l < 384) {
                0.into()
            } else {
                plen + 1
            },
            read(ext.raw, e),
            read(ext.raw, 0) + ext.json + e * ext.old,
            read(ext.raw, 0) + ext.json + ext.model + e * ext.model_old,
            read(ext.raw, 0) + ext.json + ext.model + hash(ext.raw, e),
        ]);
        load = load.max(1700 + argsret + id + rc + extpeak);
    } else {
        load = load.max(1700 + argsret + id + rc + rm)
    }
    let start = output(8, 89, 5 + id + if external { 128 } else { 0 }, true);
    let countsout = output(23, 255, 6 + id + 64, true);
    let dsource = id + nbytes(": source refused: ") + 75;
    let dhash = (id + nbytes(": K4SRC sha256 ") + 64 + nbytes(", the generator's ") + 64)
        .max(id + nbytes(": model file sha256 ") + 64 + nbytes(", committed ") + 64);
    let prefix = mx(&[
        1700 + c.invocation.argv(),
        load,
        base + start,
        base + sourceh,
        base + source + encoding(x, e).max(a(1, x) + hash(x, e)),
        cut + source + counts,
        base + sourceh
            + (fmt(dsource) + e * dsource).max(fmt(dsource) + output(2, 10, 5 + dsource, false)),
        cut + source + (fmt(dhash) + e * dhash).max(fmt(dhash) + output(2, 10, 5 + dhash, false)),
    ]);
    let enc = a(1, x);
    let common = cut + ctrldiag + enc + g(24, 4.into()) + 4 * fmt(dfail);
    let outcome = cut + enc + 78 + compbuild;
    let recordcaller = cut + enc + 78 + diag + published + hrec + id + family + 16;
    let run = diag + published + rec + id + family + 16;
    let w1 = cut + 78 + run + rec + output(10, 126, 2 + id + dout, true);
    let head = failures.min(50);
    let reportchildren = 6
        + id
        + 32 * head
        + head * dfail
        + 32 * rows
        + keysum
        + 32 * nc
        + sum(case.not_covered.iter().map(|x| nbytes(x)))
        + if case.s_full.is_some() {
            32 * rows + keysum
        } else {
            0.into()
        }
        + 64 * rows
        + 2 * rows * (id + keymax + 92)
        + 32 * controls
        + sum(case.controls.iter().map(|x| id + 1 + nbytes(&x.id)));
    let report = cut + 78 + run + rec + ncfull + 8 * head + output(21, 309, reportchildren, true);
    let fullfloor =
        cut + 78 + run + rec + floorh.max(floorret + ncfull + exactrow.max(e * old(24, rows)));
    let recordout = cut + 78 + run + rec + ncfull + output(3, 14, 6 + id + rec, false);
    let late = cut + 78 + rec + ncfull;
    let rlate = mx(&[
        hadj,
        radj + rcm(f, edges, e, false),
        radj + 8 * f + rcm(f, edges, e, true),
        radj + 16 * f,
    ]);
    let latercm = (late + rlate).max(late + output(3, 13, 3 + id, true));
    let sparse = mx(&[
        late + sparse(
            mdl,
            nn,
            m,
            n,
            f,
            r,
            s,
            d,
            e,
            // The reference composition uses the independent generic sparse
            // pattern upper. Actual RF-LARGE uses its reviewed W1/SD structural
            // pattern correspondence (no springs), never W1's skyline.
            if c.invocation.kind == InvocationKind::SingleCaseFamilyReferenceV1 {
                (n * n).min(144 * m + s)
            } else {
                z
            },
        )?,
        late + 66144 + output(3, 13, 8 + id + 33072, true),
        late + 200 + output(3, 14, 8 + id + 100, true),
    ]);
    let summary = late + 632;
    let refused = cut
        + parts
        + ctrldiag
        + mx(&[
            fmt(91.into()) + e * 91,
            fmt(id + keymax + 94) + e * (id + keymax + 94),
            diag + hrec + 225,
        ]);
    Ok(Caller {
        phases: [
            prefix,
            cut + countsout,
            cut + 632 + if e == 1 { 54 } else { 32 } + id + 11,
            cut + output(5, 46, 7 + id + nbytes("estimate_exceeds_half_cap"), false),
            cut + parts + control,
            cut + ctrldiag + sourceh,
            cut + ctrldiag + source + encoding(x, e).max(enc + hash(x, e)),
            refused,
            w1,
            fullfloor,
            report,
            recordout,
            latercm,
            sparse,
            summary,
        ],
        joins: [
            common,
            outcome,
            recordcaller,
            cut + ctrldiag + enc + expected + g(24, 4.into()) + 4 * fmt(dfail),
            cut + enc + 78 + diag + fmt(dout) + fmt(id + 39 + dout) + e * (id + 39 + dout),
        ],
        model: rm + published,
        fixed: cut + enc + 78 + ctrldiag + published,
    })
}
// Independently bound expected_unresolved.json input tree (source02/RV28 parse05).
fn expected_json() -> N {
    3222.into()
}
fn expected_json_old() -> N {
    0.into()
}

/// Both call sites use this complete interface. There is no partial/fallback max.
pub fn estimate(
    context: &VrEstimateContext<'_>,
    kernel_profile: &ReferenceKernelProfile,
    vr_profile: &ReferenceVRProfile,
) -> Result<CompleteEstimate, EnvelopeError> {
    let kernel = kernel_envelope(&context.descriptor, kernel_profile)?;
    let mut results = [Estimate {
        model: 0,
        decide: 0,
        fixed: 0,
        max: 0,
        sel128: 0,
    }; 2];
    let mut caller_values = [[0; 15]; 2];
    let mut join_values = [[0; 5]; 2];
    for e in 0..2 {
        let c = caller(context, e as i128)?;
        for (out, value) in caller_values[e].iter_mut().zip(c.phases) {
            *out = value.bytes()?;
        }
        for (out, value) in join_values[e].iter_mut().zip(c.joins) {
            *out = value.bytes()?;
        }
        let common = mx(&c.phases);
        let metric =
            |x: open_pipe_stress_solver_performance_harness::k6::w1::envelope::MetricBytes| {
                if e == 0 {
                    x.requested
                } else {
                    x.moving
                }
            };
        let convert = |x: u128| N(i128::try_from(x).ok());
        let joined = |solve, returned| {
            mx(&[
                common,
                c.joins[0] + convert(solve),
                c.joins[1] + convert(returned),
                c.joins[2] + convert(returned),
                c.joins[3] + convert(returned),
                c.joins[4] + convert(returned),
            ])
            .bytes()
        };
        results[e] = Estimate {
            model: c.model.bytes()?,
            fixed: (c.fixed + convert(kernel.base)).bytes()?,
            decide: metric(kernel.full.r7_max),
            max: joined(metric(kernel.full.solve), kernel.full.returned.union)?,
            sel128: joined(
                metric(kernel.selected128.solve),
                kernel.selected128.returned.union,
            )?,
        };
    }
    Ok(CompleteEstimate {
        proof_basis: vr_profile.proof_basis(),
        population_policy: context.descriptor.population_policy(),
        caller_requested: caller_values[0],
        caller_moving: caller_values[1],
        joins_requested: join_values[0],
        joins_moving: join_values[1],
        requested: results[0],
        moving: results[1],
        context: context.invocation.kind,
    })
}

/// Source-defined alternatives; kernel absence here is a lifetime fact.
pub const CALLER_PHASES: [&str; 15] = [
    "prefix_before_cut",
    "postcut_counts_output",
    "counts_only_summary_sample",
    "half_cap_refusal_output",
    "sourceparts_controls",
    "lane_source_constructor",
    "lane_encoding_hash",
    "source_refused_caller_only",
    "record2_w1_output",
    "full_floor",
    "report",
    "record3_output",
    "late_RCM",
    "late_sparse_parity",
    "summary_sample",
];

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn poisoned_alternative_and_reference_width_fail_explicitly() {
        assert_eq!(
            (N::from(i128::MAX) + 1).max(0).bytes(),
            Err(EnvelopeError::ArithmeticOverflow)
        );
        assert_eq!(
            N::from(i64::MAX as i128 + 1).bytes(),
            Err(EnvelopeError::ReferenceWidthExceeded)
        );
        assert_eq!(
            g(48, N::from(i128::MAX)).bytes(),
            Err(EnvelopeError::ArithmeticOverflow)
        );
    }
    #[test]
    fn all_tokens_and_overwritten_values_are_in_the_actual_prefix() {
        let mut args = ArgumentFacts::default();
        let tokens = [
            "executable",
            "--case",
            "overwritten-case",
            "--case",
            "final",
            "--model-file",
            "long/path",
            "--heap-cap-bytes",
            "0008053063680",
        ];
        for token in tokens {
            args.observe(token);
        }
        let invocation = VrInvocation::actual("manifest", "final", Some("long/path"), args, false);
        assert_eq!(
            invocation.argv().bytes().unwrap(),
            (24 * tokens.len() + tokens.iter().map(|s| s.len()).sum::<usize>()) as u128
        );
        let mut longer = args;
        longer.observe("an-overwritten-extra-value");
        assert!(
            VrInvocation::actual("manifest", "final", Some("long/path"), longer, false)
                .argv()
                .bytes()
                .unwrap()
                > invocation.argv().bytes().unwrap()
        );
    }
    #[test]
    fn continuation_bounds_normal_cap_spelling_and_retains_actual_counts_prefix() {
        let mut args = ArgumentFacts::default();
        for token in [
            "exe",
            "--case",
            "id",
            "--heap-cap-bytes",
            "536870912",
            "--counts-only",
        ] {
            args.observe(token);
        }
        let c = VrInvocation::actual("manifest", "id", None, args, true);
        let normal = VrInvocation::actual("manifest", "id", None, args, false);
        assert!(c.argv().bytes().unwrap() >= normal.argv().bytes().unwrap());
        let mut future = ArgumentFacts::default();
        for token in [
            "exe",
            "--case",
            "id",
            "--heap-cap-bytes",
            "18446744073709551615",
        ] {
            future.observe(token);
        }
        assert!(
            c.argv().bytes().unwrap()
                >= VrInvocation::actual("manifest", "id", None, future, false)
                    .argv()
                    .bytes()
                    .unwrap()
        );
        let changed =
            VrInvocation::actual("manifest", "id", Some("changed/model/path"), args, true);
        assert!(changed.argv().bytes().unwrap() > c.argv().bytes().unwrap());
    }
    #[test]
    fn whitespace_is_measured_and_unbound_escape_scratch_is_named() {
        let raw = "{\"nodes\": []}";
        let a = FamilyInputFacts::begin(raw, 10);
        let b = FamilyInputFacts::begin("  {\"nodes\": []}  ", 10);
        assert_eq!((b.raw - a.raw).bytes().unwrap(), 4);
        assert!(!FamilyInputFacts::begin("{\"x\":\"\\n\"}", 10).valid);
    }
}
