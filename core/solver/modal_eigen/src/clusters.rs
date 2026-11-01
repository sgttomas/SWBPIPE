//! The cluster definition (design §3.4, check 5) and the reported-set rule
//! (design §1.4).
//!
//! Adjacent ascending eigenvalues λ_i ≤ λ_{i+1} share a cluster when
//! λ_{i+1} − λ_i ≤ δ_c·λ_{i+1}. Membership is chained, so a cluster is a
//! maximal run. This is the only cluster definition in the crate.

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ClusterError {
    InvalidTolerance,
    /// An eigenvalue or frequency is not finite, or is negative.
    InvalidValue {
        index: usize,
    },
    /// The values are not in ascending order at `index`.
    NotAscending {
        index: usize,
    },
    /// Cluster ids are not 0, then each equal to or one above its
    /// predecessor, or their count differs from the frequencies'.
    InvalidClusterIds,
    /// A mode count of zero, or a cut-off that is not a positive finite
    /// frequency.
    InvalidBound,
}

impl fmt::Display for ClusterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTolerance => {
                write!(f, "cluster tolerance must be finite and non-negative")
            }
            Self::InvalidValue { index } => {
                write!(f, "value {index} is not a finite non-negative number")
            }
            Self::NotAscending { index } => write!(f, "values are not ascending at {index}"),
            Self::InvalidClusterIds => write!(f, "cluster ids are not a chained numbering"),
            Self::InvalidBound => write!(f, "extraction bound is not positive"),
        }
    }
}

impl Error for ClusterError {}

fn check_ascending(values: &[f64]) -> Result<(), ClusterError> {
    for (i, &value) in values.iter().enumerate() {
        if !(value.is_finite() && value >= 0.0) {
            return Err(ClusterError::InvalidValue { index: i });
        }
        if i > 0 && value < values[i - 1] {
            return Err(ClusterError::NotAscending { index: i });
        }
    }
    Ok(())
}

/// Cluster ids for ascending eigenvalues, starting at 0 and increasing by one
/// at each break. `delta_c` is the relative cluster tolerance δ_c, an
/// algorithm parameter whose production value the design fixes (M1b).
pub fn cluster_ids(eigenvalues: &[f64], delta_c: f64) -> Result<Vec<usize>, ClusterError> {
    if !(delta_c.is_finite() && delta_c >= 0.0) {
        return Err(ClusterError::InvalidTolerance);
    }
    check_ascending(eigenvalues)?;
    let mut ids = Vec::with_capacity(eigenvalues.len());
    let mut id = 0;
    for (i, &value) in eigenvalues.iter().enumerate() {
        if i > 0 && value - eigenvalues[i - 1] > delta_c * value {
            id += 1;
        }
        ids.push(id);
    }
    Ok(ids)
}

/// The extraction bound on the request (design §1.4): a mode count, an
/// inclusive frequency cut-off (Hz), or both, whichever stops first.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExtractionBound {
    ModeCount(usize),
    FrequencyCutoff(f64),
    Both {
        mode_count: usize,
        frequency_cutoff: f64,
    },
}

/// Which bound stopped the reported set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoppingBound {
    ModeCount,
    FrequencyCutoff,
    /// Both bounds give the same set.
    ModeCountAndFrequencyCutoff,
    /// Neither bound was reached: the computed spectrum ran out first.
    SpectrumExhausted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReportedSet {
    /// Modes reported: the lowest `count`, cluster extension included.
    pub count: usize,
    /// Modes selected by the bound before cluster completion.
    pub bounded_count: usize,
    /// Modes added to complete the cluster at the boundary.
    pub extension_count: usize,
    pub stopped_by: StoppingBound,
}

/// The reported set (design §1.4) over the computed frequencies, ascending,
/// with their cluster ids from [`cluster_ids`].
///
/// A mode count n reaches its bound when at least n modes were computed; a
/// cut-off f_c (inclusive) reaches its bound when some computed mode has
/// f > f_c. With both, the set is the smaller of the two. If the boundary
/// falls inside a cluster, the set is extended to the end of that cluster.
pub fn reported_set(
    frequencies_hz: &[f64],
    cluster_ids: &[usize],
    bound: ExtractionBound,
) -> Result<ReportedSet, ClusterError> {
    check_ascending(frequencies_hz)?;
    if cluster_ids.len() != frequencies_hz.len() {
        return Err(ClusterError::InvalidClusterIds);
    }
    for (i, &id) in cluster_ids.iter().enumerate() {
        let ok = if i == 0 {
            id == 0
        } else {
            id == cluster_ids[i - 1] || id == cluster_ids[i - 1] + 1
        };
        if !ok {
            return Err(ClusterError::InvalidClusterIds);
        }
    }
    let total = frequencies_hz.len();
    let by_count = |n: usize| -> Result<(usize, bool), ClusterError> {
        if n == 0 {
            return Err(ClusterError::InvalidBound);
        }
        Ok((n.min(total), n <= total))
    };
    let by_cutoff = |f_c: f64| -> Result<(usize, bool), ClusterError> {
        if !(f_c.is_finite() && f_c > 0.0) {
            return Err(ClusterError::InvalidBound);
        }
        let count = frequencies_hz.iter().take_while(|&&f| f <= f_c).count();
        Ok((count, count < total))
    };
    let (bounded_count, stopped_by) = match bound {
        ExtractionBound::ModeCount(n) => {
            let (count, reached) = by_count(n)?;
            (
                count,
                if reached {
                    StoppingBound::ModeCount
                } else {
                    StoppingBound::SpectrumExhausted
                },
            )
        }
        ExtractionBound::FrequencyCutoff(f_c) => {
            let (count, reached) = by_cutoff(f_c)?;
            (
                count,
                if reached {
                    StoppingBound::FrequencyCutoff
                } else {
                    StoppingBound::SpectrumExhausted
                },
            )
        }
        ExtractionBound::Both {
            mode_count,
            frequency_cutoff,
        } => {
            let (n_count, n_reached) = by_count(mode_count)?;
            let (f_count, f_reached) = by_cutoff(frequency_cutoff)?;
            let count = n_count.min(f_count);
            let n_stops = n_reached && n_count == count;
            let f_stops = f_reached && f_count == count;
            let stopped_by = match (n_stops, f_stops) {
                (true, true) => StoppingBound::ModeCountAndFrequencyCutoff,
                (true, false) => StoppingBound::ModeCount,
                (false, true) => StoppingBound::FrequencyCutoff,
                (false, false) => StoppingBound::SpectrumExhausted,
            };
            (count, stopped_by)
        }
    };
    let mut count = bounded_count;
    if count > 0 {
        let last = cluster_ids[count - 1];
        while count < total && cluster_ids[count] == last {
            count += 1;
        }
    }
    Ok(ReportedSet {
        count,
        bounded_count,
        extension_count: count - bounded_count,
        stopped_by,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test-local δ_c; the production value is the design's (M1b).
    const DELTA: f64 = 1e-6;

    #[test]
    fn clusters_chain_adjacent_values() {
        let lambdas = [1.0, 1.0, 1.0 + 0.5e-6, 2.0, 3.0, 3.0 + 4e-6, 3.0 + 5e-6];
        assert_eq!(
            cluster_ids(&lambdas, DELTA).unwrap(),
            vec![0, 0, 0, 1, 2, 3, 3]
        );
        // Chaining: each step is within δ_c of the next, though the ends are
        // not.
        let chain = [1.0, 1.0 + 0.9e-6, 1.0 + 1.8e-6, 1.0 + 2.7e-6];
        assert_eq!(cluster_ids(&chain, DELTA).unwrap(), vec![0, 0, 0, 0]);
        // The boundary is inclusive: a gap of exactly δ_c·λ_{i+1} joins.
        assert_eq!(cluster_ids(&[3.0, 4.0], 0.25).unwrap(), vec![0, 0]);
        assert_eq!(cluster_ids(&[3.0, 4.0], 0.249).unwrap(), vec![0, 1]);
        assert_eq!(cluster_ids(&[], DELTA).unwrap(), Vec::<usize>::new());
    }

    #[test]
    fn cluster_inputs_are_checked() {
        assert_eq!(
            cluster_ids(&[2.0, 1.0], DELTA),
            Err(ClusterError::NotAscending { index: 1 })
        );
        assert_eq!(
            cluster_ids(&[1.0, f64::NAN], DELTA),
            Err(ClusterError::InvalidValue { index: 1 })
        );
        assert_eq!(
            cluster_ids(&[1.0], -1.0),
            Err(ClusterError::InvalidTolerance)
        );
    }

    fn set(f: &[f64], ids: &[usize], bound: ExtractionBound) -> ReportedSet {
        reported_set(f, ids, bound).unwrap()
    }

    #[test]
    fn mode_count_with_cluster_completion() {
        let f = [1.0, 2.0, 2.0, 2.0, 5.0, 6.0];
        let ids = [0, 1, 1, 1, 2, 3];
        let r = set(&f, &ids, ExtractionBound::ModeCount(2));
        assert_eq!(
            r,
            ReportedSet {
                count: 4,
                bounded_count: 2,
                extension_count: 2,
                stopped_by: StoppingBound::ModeCount
            }
        );
        let r = set(&f, &ids, ExtractionBound::ModeCount(4));
        assert_eq!((r.count, r.extension_count), (4, 0));
        let r = set(&f, &ids, ExtractionBound::ModeCount(6));
        assert_eq!((r.count, r.stopped_by), (6, StoppingBound::ModeCount));
        let r = set(&f, &ids, ExtractionBound::ModeCount(9));
        assert_eq!(
            (r.count, r.stopped_by),
            (6, StoppingBound::SpectrumExhausted)
        );
    }

    #[test]
    fn cutoff_is_inclusive_and_completion_can_pass_it() {
        let f = [1.0, 2.0, 2.0 + 1e-7, 5.0];
        let ids = [0, 1, 1, 2];
        let r = set(&f, &ids, ExtractionBound::FrequencyCutoff(2.0));
        assert_eq!(
            r,
            ReportedSet {
                count: 3,
                bounded_count: 2,
                extension_count: 1,
                stopped_by: StoppingBound::FrequencyCutoff
            }
        );
        let r = set(&f, &ids, ExtractionBound::FrequencyCutoff(0.5));
        assert_eq!((r.count, r.stopped_by), (0, StoppingBound::FrequencyCutoff));
        let r = set(&f, &ids, ExtractionBound::FrequencyCutoff(5.0));
        assert_eq!(
            (r.count, r.stopped_by),
            (4, StoppingBound::SpectrumExhausted)
        );
    }

    #[test]
    fn both_bounds_take_the_smaller_and_name_it() {
        let f = [1.0, 2.0, 3.0, 4.0];
        let ids = [0, 1, 2, 3];
        let both = |n, f_c| ExtractionBound::Both {
            mode_count: n,
            frequency_cutoff: f_c,
        };
        assert_eq!(
            set(&f, &ids, both(2, 3.5)).stopped_by,
            StoppingBound::ModeCount
        );
        assert_eq!(set(&f, &ids, both(2, 3.5)).count, 2);
        assert_eq!(
            set(&f, &ids, both(3, 1.5)).stopped_by,
            StoppingBound::FrequencyCutoff
        );
        assert_eq!(set(&f, &ids, both(3, 1.5)).count, 1);
        assert_eq!(
            set(&f, &ids, both(2, 2.0)).stopped_by,
            StoppingBound::ModeCountAndFrequencyCutoff
        );
        assert_eq!(
            set(&f, &ids, both(9, 9.0)).stopped_by,
            StoppingBound::SpectrumExhausted
        );
        // The count stops at the whole spectrum while the cut-off does not.
        assert_eq!(
            set(&f, &ids, both(4, 9.0)).stopped_by,
            StoppingBound::ModeCount
        );
    }

    #[test]
    fn reported_set_inputs_are_checked() {
        let f = [1.0, 2.0];
        assert_eq!(
            reported_set(&f, &[0, 1], ExtractionBound::ModeCount(0)),
            Err(ClusterError::InvalidBound)
        );
        assert_eq!(
            reported_set(&f, &[0, 1], ExtractionBound::FrequencyCutoff(0.0)),
            Err(ClusterError::InvalidBound)
        );
        assert_eq!(
            reported_set(&f, &[0, 2], ExtractionBound::ModeCount(1)),
            Err(ClusterError::InvalidClusterIds)
        );
        assert_eq!(
            reported_set(&f, &[0], ExtractionBound::ModeCount(1)),
            Err(ClusterError::InvalidClusterIds)
        );
    }
}
