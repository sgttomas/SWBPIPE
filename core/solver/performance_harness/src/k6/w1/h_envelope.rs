//! Complete conditional H stage/prefix envelopes. The named reference profile
//! does not qualify the running executable. Actual input/count/build/launch
//! correspondence remains external. Descriptors allocate no heap storage.
use super::counts::{W1Counts, W1Estimate};
use super::envelope::{
    self, EnvelopeError, KernelDescriptor, KernelEnvelope, KernelInput, KernelPhase, MetricBytes,
    PopulationPolicy, ReferenceKernelProfile, ScheduleEnvelope, SourceConstruction,
    StructuralCounts,
};
use crate::k6::models::{K6Model, ModelOrigin, ModelRecipe};
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReferenceHProfile {
    kernel: ReferenceKernelProfile,
}
impl ReferenceHProfile {
    pub const fn source40129_rust1971_aarch64_v1() -> Self {
        Self {
            kernel: ReferenceKernelProfile::source40129_rust1971_aarch64_v1(),
        }
    }
}

#[derive(Clone, Copy)]
struct C(Option<u128>);
impl C {
    fn new(n: u128) -> Self {
        Self(Some(n))
    }
    fn raw(self) -> Result<u128, EnvelopeError> {
        self.0.ok_or(EnvelopeError::ArithmeticOverflow)
    }
    fn bytes(self) -> Result<u128, EnvelopeError> {
        let n = self.raw()?;
        if n > (1u128 << 63) - 1 {
            Err(EnvelopeError::ReferenceWidthExceeded)
        } else {
            Ok(n)
        }
    }
    fn max(self, rhs: Self) -> Self {
        Self(self.0.zip(rhs.0).map(|(a, b)| a.max(b)))
    }
}
macro_rules! op {
    ($t:ident,$f:ident,$c:ident) => {
        impl $t for C {
            type Output = Self;
            fn $f(self, b: Self) -> Self {
                Self(self.0.zip(b.0).and_then(|(a, b)| a.$c(b)))
            }
        }
        impl $t<u128> for C {
            type Output = Self;
            fn $f(self, b: u128) -> Self {
                self.$f(C::new(b))
            }
        }
        impl $t<C> for u128 {
            type Output = C;
            fn $f(self, b: C) -> C {
                C::new(self).$f(b)
            }
        }
    };
}
op!(Add, add, checked_add);
op!(Mul, mul, checked_mul);
op!(Sub, sub, checked_sub);
fn g(s: u128, n: C) -> C {
    C(n.0.and_then(|n| {
        if n == 0 {
            Some(0)
        } else {
            n.checked_next_power_of_two().map(|c| {
                c.max(if s == 1 {
                    8
                } else if s <= 1024 {
                    4
                } else {
                    1
                })
            })
        }
    })) * s
}
fn formatted(n: C) -> C {
    match n.0 {
        Some(0) => C::new(0),
        Some(_) => (2 * n).max(C::new(8)),
        None => C(None),
    }
}
fn digits(mut n: u128) -> u128 {
    let mut d = 1;
    while n >= 10 {
        n /= 10;
        d += 1;
    }
    d
}
fn sum_lengths<'a>(v: impl Iterator<Item = &'a str>, format: bool) -> C {
    v.fold(C::new(0), |total, s| {
        total
            + if format {
                formatted(C::new(s.len() as u128))
            } else {
                C::new(s.len() as u128)
            }
    })
}

/// Scalar source facts captured before the existing source's drop, or rebound
/// to externally qualified same-model counts. Failure never fabricates a kernel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HSourceFacts {
    capture: HSourceCapture,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HSourceCapture {
    Success(KernelDescriptor),
    SourceRefused { constructor: MetricBytes },
}

fn descriptor(c: &W1Counts, ids: u128, max_id: u128) -> Result<KernelDescriptor, EnvelopeError> {
    KernelDescriptor::new(
        KernelInput {
            nodes: c.nodes as u128,
            members: c.members as u128,
            axis_springs: 0,
            directional_springs: 0,
            constraints: c.constraints as u128,
            load_terms: c.loads as u128,
            stations: c.stations as u128,
            load_id_bytes: ids,
            max_load_id_bytes: max_id,
            support_groups: 0,
            nonzero_prescribed_terms: 0,
            structure: Some(StructuralCounts {
                dofs: c.dofs as u128,
                free_dofs: c.free_dofs as u128,
                quantities: c.rows as u128,
                source_encoding_bytes: c.source_encoding_len as u128,
                pattern_entries: c.pattern_entries as u128,
                profile_entries: c.profile_entries as u128,
            }),
        },
        SourceConstruction::HModelV1,
        PopulationPolicy::Exact {
            bodies: c.bodies as u128,
            free_blocks: c.blocks as u128,
        },
    )
}

impl HSourceFacts {
    pub const fn kind(self) -> HOutcomeKind {
        match self.capture {
            HSourceCapture::Success(_) => HOutcomeKind::SuccessfulSource,
            HSourceCapture::SourceRefused { .. } => HOutcomeKind::SourceRefused,
        }
    }
    pub(crate) fn successful(c: &W1Counts, ids: u128, max_id: u128) -> Result<Self, EnvelopeError> {
        Ok(Self {
            capture: HSourceCapture::Success(descriptor(c, ids, max_id)?),
        })
    }
    /// Counts files supply provenance externally; scalar consistency is checked
    /// against the surviving model without rerunning source/graph/encoding code.
    pub fn from_counts(model: &K6Model, c: &W1Counts) -> Result<Self, EnvelopeError> {
        let mut r = C::new(0);
        for (_, mask) in &model.restraints {
            r = r + mask.iter().filter(|x| **x).count() as u128;
        }
        let mut ids = C::new(0);
        let mut max_id = 0;
        for (global, _) in &model.loads {
            let d = 3 + digits(*global as u128);
            ids = ids + d;
            max_id = max_id.max(d);
        }
        let r = r.raw()?;
        let ids = ids.raw()?;
        if c.nodes != model.nodes.len()
            || c.members != model.members.len()
            || c.stations != model.members.len()
            || c.loads != model.loads.len()
            || c.constraints as u128 != r
            || c.dofs as u128 != (6 * C::new(model.nodes.len() as u128)).raw()?
        {
            return Err(EnvelopeError::InvalidDescriptor("H model/count population"));
        }
        if c.source_ok {
            Self::successful(c, ids, max_id)
        } else {
            if [
                c.free_dofs,
                c.bodies,
                c.pattern_entries,
                c.profile_entries,
                c.half_bandwidth,
                c.blocks,
                c.rows,
                c.source_encoding_len,
            ]
            .iter()
            .any(|x| *x != 0)
                || c.source_encoding_fnv64 != 0
            {
                return Err(EnvelopeError::InvalidDescriptor(
                    "refused source has executed-phase counts",
                ));
            }
            Ok(Self {
                capture: HSourceCapture::SourceRefused {
                    constructor: envelope::h_source_constructor(
                        model.nodes.len() as u128,
                        model.members.len() as u128,
                        r,
                        model.loads.len() as u128,
                        ids,
                        max_id,
                    )?,
                },
            })
        }
    }
}

/// The provenance token must come from the same actual constructor as `model`.
/// No field is added to K6Model or its owned element representations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HModelFacts {
    bytes: u128,
}
impl HModelFacts {
    pub fn capture(model: &K6Model, origin: ModelOrigin) -> Result<Self, EnvelopeError> {
        let n = C::new(model.nodes.len() as u128);
        let m = C::new(model.members.len() as u128);
        let r = C::new(model.restraints.len() as u128);
        let l = C::new(model.loads.len() as u128);
        let (nodes, loads, node_fmt, member_fmt) = match origin.0 {
            ModelRecipe::Builder => (g(48, n), g(16, l), true, false),
            ModelRecipe::Fixture => (g(48, n).max(32 * n), g(16, l), true, true),
            ModelRecipe::Canonical => (48 * n, 16 * l, false, false),
        };
        let total = nodes
            + 64 * m
            + g(16, r)
            + loads
            + g(136, m)
            + sum_lengths(model.nodes.iter().map(|(s, _)| s.as_str()), node_fmt)
            + sum_lengths(
                model.members.iter().map(|(s, _, _, _)| s.as_str()),
                member_fmt,
            )
            + model.id.len() as u128
            + model.source.len() as u128;
        Ok(Self {
            bytes: total.bytes()?,
        })
    }
    pub const fn retained_bytes(self) -> u128 {
        self.bytes
    }
}

/// The exact strings retained by Args, including unused paths. A counts-only
/// prepass describes the normal continuation with these same supplied arguments.
#[derive(Clone, Copy)]
pub struct HLaunch<'a> {
    pub retained_arguments: [Option<&'a str>; 6],
    pub repeats: usize,
    pub prefixes: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOutcomeKind {
    SuccessfulSource,
    SourceRefused,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HComposedEstimate {
    pub kind: HOutcomeKind,
    pub dominant_kernel: Option<super::envelope::PhaseId>,
    pub selected128_dominant_kernel: Option<super::envelope::PhaseId>,
    /// Moving fields serialized in the existing numerical schema.
    pub legacy: W1Estimate,
    pub full: MetricBytes,
    pub selected128: MetricBytes,
    pub source_window: MetricBytes,
    pub solve_window: MetricBytes,
    pub prefix_window: MetricBytes,
}
fn plus(p: MetricBytes, c: C) -> Result<MetricBytes, EnvelopeError> {
    Ok(MetricBytes {
        requested: (c + p.requested).bytes()?,
        moving: (c + p.moving).bytes()?,
    })
}
fn maximum(a: MetricBytes, b: MetricBytes) -> MetricBytes {
    MetricBytes {
        requested: a.requested.max(b.requested),
        moving: a.moving.max(b.moving),
    }
}
/// Reference-only stage emission branches (without returned kernel owners).
/// Exposes the repeat-digit grammar separately from a possibly larger phase max.
pub fn stage_line_bounds(
    repeats: usize,
    _profile: &ReferenceHProfile,
) -> Result<(MetricBytes, MetricBytes), EnvelopeError> {
    stage_line_bytes(digits(repeats as u128))
}
fn stage_line_bytes(repeat_digits: u128) -> Result<(MetricBytes, MetricBytes), EnvelopeError> {
    // Source12/h_numeric19 refined error text D=196808. Add actual repeat digits
    // before String growth; Line/Error are sequential allocation identities.
    let extra = C::new(repeat_digits) - 1;
    let no_line = formatted(C::new(315) + extra) + 78;
    let err_line = formatted(C::new(313) + 6 * 196808 + extra) + 78;
    let no_emit = MetricBytes {
        requested: no_line.bytes()?,
        moving: (no_line + 315 + extra).bytes()?,
    };
    let err_emit = MetricBytes {
        requested: (err_line + 393616).bytes()?,
        moving: (err_line + 393616 + 313 + 6 * 196808 + extra).bytes()?,
    };
    Ok((no_emit, err_emit))
}
fn schedule(
    s: &ScheduleEnvelope,
    caller: C,
    prefixes: bool,
    repeat_digits: u128,
) -> Result<(MetricBytes, MetricBytes), EnvelopeError> {
    let solve = plus(
        maximum(
            s.solve,
            MetricBytes {
                requested: (C::new(s.retained_prefix) + 393616).bytes()?,
                moving: (C::new(s.retained_prefix) + 590424).bytes()?,
            },
        ),
        caller,
    )?;
    if !prefixes {
        return Ok((solve, MetricBytes::default()));
    }
    let (no_emit, err_emit) = stage_line_bytes(repeat_digits)?;
    let prefix = maximum(
        maximum(
            s.solve,
            MetricBytes {
                requested: (C::new(s.retained_prefix) + 393616).bytes()?,
                moving: (C::new(s.retained_prefix) + 590424).bytes()?,
            },
        ),
        maximum(
            plus(err_emit, C::new(s.retained_prefix))?,
            plus(no_emit, C::new(s.returned.selected))?,
        ),
    );
    Ok((solve, plus(prefix, caller + 387)?))
}
fn compose(
    model: HModelFacts,
    source: HSourceFacts,
    args: u128,
    prefixes: bool,
    repeats: usize,
    profile: &ReferenceHProfile,
) -> Result<HComposedEstimate, EnvelopeError> {
    let caller = C::new(model.bytes) + args + 1700;
    let mut fields = W1Estimate {
        decide: 0,
        solve: [0; 4],
        pass: [0; 3],
        fixed: caller.bytes()?,
        shared: [0; 4],
        state: [0; 4],
        verify: [0; 3],
        max: 0,
        sel128: 0,
    };
    if let HSourceCapture::SourceRefused { constructor } = source.capture {
        let source_window = plus(
            maximum(
                constructor,
                MetricBytes {
                    requested: 150,
                    moving: 225,
                },
            ),
            caller,
        )?;
        fields.max = source_window.moving;
        fields.sel128 = fields.max;
        // Explicitly unexecuted kernel phases under source_ok=false, not unknown facts.
        return Ok(HComposedEstimate {
            kind: HOutcomeKind::SourceRefused,
            dominant_kernel: None,
            selected128_dominant_kernel: None,
            legacy: fields,
            full: source_window,
            selected128: source_window,
            source_window,
            solve_window: MetricBytes::default(),
            prefix_window: MetricBytes::default(),
        });
    }
    let HSourceCapture::Success(d) = source.capture else {
        unreachable!()
    };
    let kernel = envelope::kernel_envelope(&d, &profile.kernel)?;
    let caller =
        caller + 3328 + 120 * C::new(d.body_population()) + 144 * C::new(d.block_population());
    let source_window = plus(
        maximum(
            kernel.source_constructor,
            MetricBytes {
                requested: 150,
                moving: 225,
            },
        ),
        caller,
    )?;
    let (solve_window, prefix_window) =
        schedule(&kernel.full, caller, prefixes, digits(repeats as u128))?;
    let (ss, sp) = schedule(
        &kernel.selected128,
        caller,
        prefixes,
        digits(repeats as u128),
    )?;
    let full = maximum(source_window, maximum(solve_window, prefix_window));
    let selected128 = maximum(source_window, maximum(ss, sp));
    fields.fixed = (caller + kernel.base + if prefixes { 387 } else { 0 }).bytes()?;
    fields.shared = kernel.shared;
    fields.state = kernel.solved;
    fields.verify = kernel.verification;
    fields.decide = kernel.full.r7_max.moving;
    fields.max = full.moving;
    fields.sel128 = selected128.moving;
    diagnostics(&kernel, &mut fields)?;
    Ok(HComposedEstimate {
        kind: HOutcomeKind::SuccessfulSource,
        dominant_kernel: Some(kernel.full.dominant_moving),
        selected128_dominant_kernel: Some(kernel.selected128.dominant_moving),
        legacy: fields,
        full,
        selected128,
        source_window,
        solve_window,
        prefix_window,
    })
}
fn diagnostics(k: &KernelEnvelope, e: &mut W1Estimate) -> Result<(), EnvelopeError> {
    use KernelPhase::*;
    for p in k.full.phases() {
        let i = match p.id.precision {
            Some(128) => 0,
            Some(256) => 1,
            Some(512) => 2,
            Some(1024) => 3,
            _ => continue,
        };
        let extra = p
            .bytes
            .moving
            .checked_sub(k.full.retained_prefix)
            .ok_or(EnvelopeError::ArithmeticOverflow)?;
        match p.id.phase {
            InitialSolve | Residual | Correction | Fallback | ChosenFallbackClone | Recovery => {
                e.solve[i] = e.solve[i].max(extra)
            }
            ResolutionTop
            | ResolutionHatCheck
            | FirstFormationScale
            | PassDeltaSolve
            | PassRecovery
            | PassFormationScale
            | ContributionOperand
            | Shift
            | ReportBuild
            | AttemptRefusalTransfer
                if i > 0 =>
            {
                e.pass[i - 1] = e.pass[i - 1].max(extra)
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn estimate(
    model: HModelFacts,
    source: HSourceFacts,
    launch: HLaunch<'_>,
    profile: &ReferenceHProfile,
) -> Result<HComposedEstimate, EnvelopeError> {
    let args = sum_lengths(launch.retained_arguments.into_iter().flatten(), false).bytes()?;
    compose(
        model,
        source,
        args,
        launch.prefixes,
        launch.repeats,
        profile,
    )
}

/// Explicit immutable OriginalK6bPair reference context. It is never selected by
/// the executable's ordinary actual-argument path. Counts are original qualified
/// metadata; these scalar recipes do not generate the large models.
pub fn original_k6b_pair(
    id: &str,
    c: &W1Counts,
    profile: &ReferenceHProfile,
) -> Result<HComposedEstimate, EnvelopeError> {
    let &(.., first, second) = ORIGINAL_LAUNCH_BYTES
        .iter()
        .find(|(name, _, _)| *name == id)
        .ok_or(EnvelopeError::MissingDescriptor("OriginalK6bPair model"))?;
    if !c.source_ok {
        return Err(EnvelopeError::InvalidDescriptor(
            "OriginalK6bPair successful source",
        ));
    }
    let (n, m, r, l, t) = (
        C::new(c.nodes as u128),
        C::new(c.members as u128),
        C::new(c.constraints as u128),
        C::new(c.loads as u128),
        C::new(c.stations as u128),
    );
    let ids = (C::new(c.source_encoding_len as u128)
        - (38 + 24 * n + 84 * m + 13 * r + 17 * l + 16 * t))
        .raw()?;
    let max_id = if c.loads == 0 {
        0
    } else {
        (3 + digits((6 * n - 1).raw()?)).min((C::new(ids) - 4 * (l - 1)).raw()?)
    };
    let src = HSourceFacts::successful(c, ids, max_id)?;
    let is_dec = id.starts_with("DEC053:");
    let nodes = if is_dec {
        g(48, n).max(32 * n)
    } else {
        g(48, n)
    };
    let member_label = 1 + digits(c.members as u128);
    let node_label = 1 + digits((n - 1).raw()?);
    let model = HModelFacts {
        bytes: (nodes
            + 64 * m
            + g(16, C::new(c.nodes.min(c.constraints) as u128))
            + g(16, l)
            + g(136, m)
            + n * formatted(C::new(node_label))
            + m * if is_dec {
                formatted(C::new(member_label))
            } else {
                C::new(member_label)
            }
            + id.len() as u128
            + if is_dec {
                id.len() as u128
            } else {
                crate::k6::models::R1_SOURCE.len() as u128
            })
        .bytes()?,
    };
    let mut a = compose(model, src, first, true, 5, profile)?;
    let b = compose(model, src, second, false, 5, profile)?;
    a.full = maximum(a.full, b.full);
    a.selected128 = maximum(a.selected128, b.selected128);
    a.legacy.max = a.full.moving;
    a.legacy.sel128 = a.selected128.moving;
    a.legacy.fixed = a.legacy.fixed.max(b.legacy.fixed);
    Ok(a)
}

// Exact retained String byte totals for the two accepted original argv recipes.
// Scope: reference regeneration only; arbitrary later paths require actual capture.
const ORIGINAL_LAUNCH_BYTES: &[(&str, u128, u128)] = &[
    ("RF-LARGE-CHAIN-n00010-AX", 345, 111),
    ("RF-LARGE-CHAIN-n00010-ROT", 347, 112),
    ("RF-LARGE-TREE-n00010-AX", 343, 110),
    ("RF-LARGE-TREE-n00010-ROT", 345, 111),
    ("RF-LARGE-CONT-n00010-AX", 343, 110),
    ("RF-LARGE-CONT-n00010-ROT", 345, 111),
    ("DEC053:invented-cantilever-chain-8", 365, 121),
    ("DEC053:invented-cantilever-chain-24", 367, 122),
    ("DEC053:invented-cantilever-chain-48", 367, 122),
    ("DEC053:invented-grid-frame-4x3", 357, 117),
    ("DEC053:invented-grid-frame-6x8", 357, 117),
    ("DEC053:invented-grid-frame-7x8", 357, 117),
    ("DEC053:invented-grid-frame-5x5", 357, 117),
    ("DEC053:invented-cantilever-chain-32", 367, 122),
    ("DEC053:invented-grid-frame-5x6", 357, 117),
    ("RF-LARGE-CHAIN-n00100-AX", 345, 111),
    ("RF-LARGE-CHAIN-n00100-ROT", 347, 112),
    ("RF-LARGE-TREE-n00100-AX", 343, 110),
    ("RF-LARGE-TREE-n00100-ROT", 345, 111),
    ("RF-LARGE-CONT-n00100-AX", 343, 110),
    ("RF-LARGE-CONT-n00100-ROT", 345, 111),
    ("RF-LARGE-CHAIN-n01000-AX", 345, 111),
    ("RF-LARGE-CHAIN-n01000-ROT", 347, 112),
    ("RF-LARGE-TREE-n01000-AX", 343, 110),
    ("RF-LARGE-TREE-n01000-ROT", 345, 111),
    ("RF-LARGE-CONT-n01000-AX", 343, 110),
    ("RF-LARGE-CONT-n01000-ROT", 345, 111),
    ("RF-LARGE-CHAIN-n10000-AX", 345, 111),
    ("RF-LARGE-CHAIN-n10000-ROT", 347, 112),
    ("RF-LARGE-TREE-n10000-AX", 343, 110),
    ("RF-LARGE-TREE-n10000-ROT", 345, 111),
    ("RF-LARGE-CONT-n10000-AX", 343, 110),
    ("RF-LARGE-CONT-n10000-ROT", 345, 111),
];
