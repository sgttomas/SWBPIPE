//! K6's deterministic storage counts (T3 D1 §4.8) and the derived peak
//! estimates per mode (K6 plan §7, approved with ROOT's rulings N8 and N9).
//!
//! The counts phase works in O(nnz) memory: it never forms an n² array or a
//! profile's values. Its RCM profile is K6's own skyline count on the order
//! from `sparse_direct`'s `reverse_cuthill_mckee`, over the nonzero lower
//! entries of the prepared free block (the entries `order_sparse_structural`
//! orders); the factor stage records `order_sparse_structural`'s own count
//! beside it. The identity-order profile is F1b's O(nnz) rule on the lane's
//! entries. Both are checked against an independent Python oracle.

use super::lanes::{identity_profile, off_diagonal_entries, reduced_entry_system};
use super::models::K6Model;
use super::w1::counts::W1Counts;
use super::Mode;
use open_pipe_stress_frame_kernel::structural::{
    assemble_sparse_stiffness, prepare_sparse_structural, ContributionRounding, PivotEvidence,
    ResidualRow, SparseAssemblyOptions, SparseStructuralSystem, SparseSymmetryEvidence,
    StiffnessContribution,
};
use open_pipe_stress_frame_kernel::FrameElement;
use open_pipe_stress_nonlinear_integration::structural_adapter::SparseAssemblyEvidence;
use open_pipe_stress_sparse_direct::{
    adjacency_from_symmetric_entries, reverse_cuthill_mckee, SymmetricMatrixEntry,
};
use std::io::Write;

/// The symmetry basis `SparseAssemblyEvidence` gives a family of qualified
/// straight frames: a verbatim copy of the adapter's private `symmetry_basis`
/// (`SA:1291-1297`, the all-qualified branch; ROOT's K6 ruling N5). Test E
/// pins it against the adapter's own output, so any drift is caught.
pub const FRAME_FAMILY_SYMMETRY_BASIS: &str = "represented local matrices; two-stage 12-term frame transforms plus directed scatter; curved H*K and (H*K)*H^T six-term stages when traced; inverse accuracy not claimed; objective welded unreleased straight-frame family";

/// F1b's dense-scrutiny constant (`PP@F1b:2878`, commit `130445db2`),
/// quoted, never imported.
pub const F1B_DENSE_BYTES_PER_ENTRY: u128 = 96;
/// F1b's observation-lane constant (`PP@F1b:2958`, commit `130445db2`).
pub const F1B_LANE_BYTES_PER_PROFILE_ENTRY: u128 = 24;
/// `Expansion` (`FK/structural.rs:675-678`: a `Vec<f64>` and a `usize`) is
/// crate-private, so its size is derived: 24 + 8 bytes on a 64-bit target.
pub const EXPANSION_BYTES: u128 = 32;

/// The counts line (K6 plan §4.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct K6Counts {
    pub nodes: usize,
    pub members: usize,
    pub dofs: usize,
    pub free_dofs: usize,
    pub restrained_dofs: usize,
    /// Stored entries of the global pattern (both triangles).
    pub pattern_entries: usize,
    /// Stored entries on or below the diagonal.
    pub lower_entries: usize,
    /// Stored entries of the prepared free block (both triangles).
    pub free_entries: usize,
    /// The free block's lower entries, explicit zeros included.
    pub free_lower_entries: usize,
    /// Those whose prepared value is nonzero (the RCM's input).
    pub free_lower_nonzero: usize,
    pub rcm_profile_entries: u128,
    pub rcm_half_bandwidth: usize,
    pub identity_profile_entries: u128,
    pub identity_half_bandwidth: usize,
    /// The identity lane's explicit element entries (and their off-diagonal ones).
    pub lane_entries: usize,
    pub lane_off_diagonal: usize,
    pub contributions: usize,
    pub dense_entries: u128,
    /// K6b: W1's counts, where the counts phase computed them (the `w1a` mode
    /// and `--counts-only`); `None` in K6's own modes and in K6's records.
    pub w1: Option<W1Counts>,
}

/// The sizes the estimates use, from the types themselves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SizeFacts {
    pub stiffness_contribution: usize,
    pub contribution_rounding: usize,
    pub residual_row: usize,
    pub pivot_evidence: usize,
    pub frame_element: usize,
    pub symmetric_matrix_entry: usize,
}

impl SizeFacts {
    pub fn of_this_build() -> Self {
        Self {
            stiffness_contribution: std::mem::size_of::<StiffnessContribution>(),
            contribution_rounding: std::mem::size_of::<ContributionRounding>(),
            residual_row: std::mem::size_of::<ResidualRow>(),
            pivot_evidence: std::mem::size_of::<PivotEvidence>(),
            frame_element: std::mem::size_of::<FrameElement>(),
            symmetric_matrix_entry: std::mem::size_of::<SymmetricMatrixEntry>(),
        }
    }
}

/// Where `compute` writes the oracle's input (`--dump-pattern`): the prepared
/// free block's nonzero lower positions, then the lane's entry positions.
pub type PatternSink<'w> = Option<&'w mut dyn Write>;

/// The counts of `model`, in O(nnz) memory. With a sink, the positions the
/// oracle needs are written as `k6-pattern v1`.
pub fn compute(
    model: &K6Model,
    frames: &[FrameElement],
    mut sink: PatternSink<'_>,
) -> Result<K6Counts, String> {
    let n_nodes = model.node_count();
    let restrained = model.restrained_dofs();
    let (pattern_entries, lower_entries, dofs, contributions, free, prepared_counts) = {
        let k = assemble_sparse_stiffness(
            n_nodes,
            frames,
            &[],
            &[],
            &[],
            &SparseAssemblyOptions::new(),
        )
        .map_err(|e| format!("assembly: {e:?}"))?;
        let storage = k.storage_counts();
        let evidence = SparseAssemblyEvidence::new(k.pattern(), n_nodes, frames, &[], &[], &[])
            .map_err(|e| format!("evidence: {e:?}"))?;
        let contributions = evidence.storage_counts().contributions;
        let mut is_restrained = vec![false; storage.dimension];
        for &dof in &restrained {
            is_restrained[dof] = true;
        }
        let free: Vec<usize> = (0..storage.dimension)
            .filter(|&d| !is_restrained[d])
            .collect();
        let prescribed: Vec<(usize, f64)> = restrained.iter().map(|&d| (d, 0.0)).collect();
        let force = vec![0.0; storage.dimension];
        let system = SparseStructuralSystem::new(
            &k,
            &force,
            &free,
            &prescribed,
            None,
            Some(SparseSymmetryEvidence {
                absolute_roundoff: evidence.absolute_roundoff(),
                operation_counts: evidence.operation_counts(),
                basis: FRAME_FAMILY_SYMMETRY_BASIS,
            }),
        );
        let prepared = prepare_sparse_structural(&system).map_err(|e| format!("prepare: {e:?}"))?;
        let free_entries = prepared.entry_count();
        let n_free = prepared.dimension();
        let mut free_lower_entries = 0;
        let mut nonzero: Vec<SymmetricMatrixEntry> = Vec::new();
        for (row, col, value) in prepared.lower_entries() {
            free_lower_entries += 1;
            if value != 0.0 {
                nonzero.push(SymmetricMatrixEntry { row, col, value });
            }
        }
        drop(prepared);
        if let Some(out) = sink.as_mut() {
            writeln!(out, "k6-pattern v1").map_err(|e| e.to_string())?;
            writeln!(out, "id {}", model.id).map_err(|e| e.to_string())?;
            writeln!(out, "free_dofs {n_free}").map_err(|e| e.to_string())?;
            writeln!(out, "rcm_entries {}", nonzero.len()).map_err(|e| e.to_string())?;
            for e in &nonzero {
                writeln!(out, "{} {}", e.row, e.col).map_err(|e| e.to_string())?;
            }
        }
        let (rcm_profile, rcm_bandwidth) = rcm_profile(n_free, &nonzero)?;
        (
            storage.stored_entries,
            storage.lower_entries,
            storage.dimension,
            contributions,
            free,
            (
                free_entries,
                free_lower_entries,
                nonzero.len(),
                rcm_profile,
                rcm_bandwidth,
            ),
        )
    };
    let (
        free_entries,
        free_lower_entries,
        free_lower_nonzero,
        rcm_profile_entries,
        rcm_half_bandwidth,
    ) = prepared_counts;
    let lane = reduced_entry_system(n_nodes, frames, &model.force_vector(), &restrained)
        .map_err(|e| format!("lane entries: {e:?}"))?;
    if let Some(out) = sink.as_mut() {
        writeln!(out, "lane_dim {}", lane.dimension).map_err(|e| e.to_string())?;
        writeln!(out, "lane_entries {}", lane.entries.len()).map_err(|e| e.to_string())?;
        for e in &lane.entries {
            writeln!(out, "{} {}", e.row, e.col).map_err(|e| e.to_string())?;
        }
    }
    let (identity_profile_entries, identity_half_bandwidth) = identity_profile(&lane);
    let lane_entries = lane.entries.len();
    let lane_off_diagonal = off_diagonal_entries(&lane);
    drop(lane);
    Ok(K6Counts {
        nodes: n_nodes,
        members: model.member_count(),
        dofs,
        free_dofs: free.len(),
        restrained_dofs: restrained.len(),
        pattern_entries,
        lower_entries,
        free_entries,
        free_lower_entries,
        free_lower_nonzero,
        rcm_profile_entries,
        rcm_half_bandwidth,
        identity_profile_entries,
        identity_half_bandwidth,
        lane_entries,
        lane_off_diagonal,
        contributions,
        dense_entries: (dofs as u128) * (dofs as u128),
        w1: None,
    })
}

/// The skyline of `entries` (nonzero, lower) under the RCM order of their
/// adjacency: `from_entries_with_order`'s first-column rule, counted only.
pub fn rcm_profile(
    dimension: usize,
    entries: &[SymmetricMatrixEntry],
) -> Result<(u128, usize), String> {
    let adjacency = adjacency_from_symmetric_entries(dimension, entries)
        .map_err(|e| format!("adjacency: {e:?}"))?;
    let order = reverse_cuthill_mckee(&adjacency).map_err(|e| format!("rcm: {e:?}"))?;
    drop(adjacency);
    Ok(skyline_count(dimension, entries, &order))
}

/// The skyline count of `entries` under `order` (`order[k]` is the original
/// index of ordered row k); a zero entry is skipped.
pub fn skyline_count(
    dimension: usize,
    entries: &[SymmetricMatrixEntry],
    order: &[usize],
) -> (u128, usize) {
    let mut position = vec![0usize; dimension];
    for (k, &original) in order.iter().enumerate() {
        position[original] = k;
    }
    let mut first: Vec<usize> = (0..dimension).collect();
    for e in entries {
        if e.value == 0.0 {
            continue;
        }
        let (a, b) = (position[e.row], position[e.col]);
        let (hi, lo) = if a >= b { (a, b) } else { (b, a) };
        if lo < first[hi] {
            first[hi] = lo;
        }
    }
    let profile = first
        .iter()
        .enumerate()
        .map(|(row, &f)| (row - f + 1) as u128)
        .sum();
    let bandwidth = first
        .iter()
        .enumerate()
        .map(|(row, &f)| row - f)
        .max()
        .unwrap_or(0);
    (profile, bandwidth)
}

/// F1b's own estimate for the mode, where F1b states one: 96·n² (dense) and
/// 24 bytes per identity-order profile entry (`lane-id`).
pub fn f1b_estimate_bytes(mode: Mode, c: &K6Counts) -> Option<u128> {
    match mode {
        Mode::Dense => Some(F1B_DENSE_BYTES_PER_ENTRY * c.dense_entries),
        Mode::LaneId => Some(F1B_LANE_BYTES_PER_PROFILE_ENTRY * c.identity_profile_entries),
        Mode::Sparse | Mode::LaneLu | Mode::W1a => None,
    }
}

/// E_base (plan §7.1): the O(nnz) state alive beside every solver stage: the
/// stiffness, the evidence, the formation source, the partition and the force.
pub fn base_bytes(c: &K6Counts, s: &SizeFacts) -> u128 {
    let nnz = c.pattern_entries as u128;
    let n = c.dofs as u128;
    let m = c.members as u128;
    let stiffness = 24 * nnz + 8 * (n + 1);
    let evidence = c.contributions as u128 * s.stiffness_contribution as u128
        + 16 * nnz
        + 8 * (n + 1)
        + 16 * nnz
        + 32 * c.nodes as u128
        + 24 * m
        + m * s.frame_element as u128;
    let source = m * s.frame_element as u128;
    let partition = 8 * c.free_dofs as u128 + 16 * c.restrained_dofs as u128 + 16 * n;
    stiffness + evidence + source + partition
}

/// The admission estimate per mode (plan §7; ROOT's rulings N8 and N9). The
/// counts R (contribution-rounding rows) and Z (nonzero entries) are bounded
/// by the pattern's entry count.
pub fn admission_estimate_bytes(
    mode: Mode,
    c: &K6Counts,
    s: &SizeFacts,
    w1: Option<&super::w1::counts::W1Estimate>,
) -> Result<u128, super::w1::envelope::EnvelopeError> {
    let nnz = c.pattern_entries as u128;
    let n = c.dofs as u128;
    let nf = c.free_dofs as u128;
    let nnz_f = c.free_entries as u128;
    let p = c.rcm_profile_entries;
    let base = base_bytes(c, s);
    Ok(match mode {
        Mode::W1a => {
            w1.ok_or(super::w1::envelope::EnvelopeError::MissingDescriptor(
                "complete H estimate",
            ))?
            .max
        }
        Mode::Dense => F1B_DENSE_BYTES_PER_ENTRY * c.dense_entries + base,
        Mode::Sparse => {
            let prepared = 32 * nnz_f + 28 * nf;
            let prep = 8 * n
                + prepared
                + 2 * (EXPANSION_BYTES * nnz + 32 * nnz)
                + nnz * (s.contribution_rounding as u128 + 32);
            let fact = prepared
                + (s.symmetric_matrix_entry as u128 * c.free_lower_entries as u128
                    + 64 * c.free_lower_nonzero as u128
                    + 24 * p
                    + 48 * nf)
                    .max(8 * p + 48 * nf);
            let fin = prepared
                + 8 * p
                + 72 * nf
                + 2 * s.pivot_evidence as u128 * nf
                + 2 * s.residual_row as u128 * nf
                + 2 * EXPANSION_BYTES * nnz
                + 24 * n;
            base + prep.max(fact).max(fin)
        }
        Mode::LaneId => {
            let pid = c.identity_profile_entries;
            24 * nnz
                + 8 * (n + 1)
                + 16 * n
                + 3 * s.symmetric_matrix_entry as u128 * c.lane_entries as u128
                + 8 * nf
                + 2 * 32 * c.lane_off_diagonal as u128
                + (24 * pid)
                    .max(8 * pid + 24 * p)
                    .max(8 * pid + 16 * p + 8 * nf)
                + 64 * nf
        }
        Mode::LaneLu => {
            24 * nnz
                + 8 * (n + 1)
                + 16 * n
                + (8 * n * n + 8 * nf * nf).max(16 * nf * nf)
                + 24 * (n + nf)
                + 16 * nf
        }
    })
}

/// The integer value of `"key":<digits>` in one of K6's own JSON lines.
fn json_integer(line: &str, key: &str) -> Option<u128> {
    let pattern = format!("\"{key}\":");
    let start = line.find(&pattern)? + pattern.len();
    let digits: String = line[start..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digits.parse().ok()
}

/// The string value of `"key":"<text>"` in one of K6's own JSON lines (no
/// escapes are expected in the fields read here).
fn json_string<'l>(line: &'l str, key: &str) -> Option<&'l str> {
    let pattern = format!("\"{key}\":\"");
    let start = line.find(&pattern)? + pattern.len();
    let end = line[start..].find('"')?;
    Some(&line[start..start + end])
}

/// Reads a `counts` line that `k6_observe --counts-only` printed: the model
/// id, the counts, and the model's canonical FNV-1a digest (which the caller
/// checks against its own model before relying on the counts).
pub fn parse_counts_line(line: &str) -> Option<(String, K6Counts, u64)> {
    if json_string(line, "kind")? != "counts" {
        return None;
    }
    let model = json_string(line, "model")?.to_string();
    let digest = u64::from_str_radix(json_string(line, "model_canonical_fnv64")?, 16).ok()?;
    let u = |key: &str| json_integer(line, key).and_then(|v| usize::try_from(v).ok());
    let counts = K6Counts {
        nodes: u("nodes")?,
        members: u("members")?,
        dofs: u("dofs")?,
        free_dofs: u("free_dofs")?,
        restrained_dofs: u("restrained_dofs")?,
        pattern_entries: u("pattern_entries")?,
        lower_entries: u("lower_entries")?,
        free_entries: u("free_entries")?,
        free_lower_entries: u("free_lower_entries")?,
        free_lower_nonzero: u("free_lower_nonzero")?,
        rcm_profile_entries: json_integer(line, "rcm_profile_entries")?,
        rcm_half_bandwidth: u("rcm_half_bandwidth")?,
        identity_profile_entries: json_integer(line, "identity_profile_entries")?,
        identity_half_bandwidth: u("identity_half_bandwidth")?,
        lane_entries: u("lane_entries")?,
        lane_off_diagonal: u("lane_off_diagonal")?,
        contributions: u("contributions")?,
        dense_entries: json_integer(line, "dense_entries")?,
        w1: parse_w1(line),
    };
    Some((model, counts, digest))
}

/// The boolean value of `"key":true|false` in one of K6's own JSON lines.
fn json_bool(line: &str, key: &str) -> Option<bool> {
    let pattern = format!("\"{key}\":");
    let rest = &line[line.find(&pattern)? + pattern.len()..];
    if rest.starts_with("true") {
        Some(true)
    } else if rest.starts_with("false") {
        Some(false)
    } else {
        None
    }
}

/// K6b's W1 counts from a counts line, when it carries them.
fn parse_w1(line: &str) -> Option<W1Counts> {
    let u = |key: &str| json_integer(line, key).and_then(|v| usize::try_from(v).ok());
    Some(W1Counts {
        source_ok: json_bool(line, "w1_source_ok")?,
        nodes: u("w1_nodes")?,
        members: u("w1_members")?,
        stations: u("w1_stations")?,
        constraints: u("w1_constraints")?,
        loads: u("w1_loads")?,
        dofs: u("w1_dofs")?,
        free_dofs: u("w1_free_dofs")?,
        bodies: u("w1_bodies")?,
        pattern_entries: u("w1_pattern_entries")?,
        profile_entries: u("w1_profile_entries")?,
        half_bandwidth: u("w1_half_bandwidth")?,
        blocks: u("w1_blocks")?,
        rows: u("w1_rows")?,
        source_encoding_len: u("w1_source_encoding_len")?,
        source_encoding_fnv64: u64::from_str_radix(
            json_string(line, "w1_source_encoding_fnv64")?,
            16,
        )
        .ok()?,
    })
}
