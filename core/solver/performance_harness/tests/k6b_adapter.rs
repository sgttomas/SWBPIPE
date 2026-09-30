//! K6b: the adapter (T3 K6b plan §3.2, Q6(c)).
//!
//! - Its K4SRC equals the bytes that the runner's independent Python writes
//!   from the runner's independent Python generator (`runner/k6b_sources.py`,
//!   committed as `observations/k6b/sources.txt`), for RF-LARGE at 10 and 100
//!   members and the DEC-053 nine. The one-off check against R1's own models
//!   (`references.py --model`, all 24 RF-LARGE cases) is recorded, not a CI
//!   test (CI has no R1 records).
//! - `PrimitiveSource::encoding` is deterministic and independent of the order
//!   the parts are listed in.

use open_pipe_stress_frame_kernel::structural::retained_api::PrimitiveSource;
use open_pipe_stress_solver_performance_harness::k6::models::{model, sealed_model_ids};
use open_pipe_stress_solver_performance_harness::k6::w1::adapter::{source, source_parts};
use open_pipe_stress_solver_performance_harness::k6::w1::counts::fnv64;

const SOURCES: &str = include_str!("../observations/k6b/sources.txt");

/// (length, FNV-1a) of the committed K4SRC of `id`.
fn committed(id: &str) -> (usize, u64) {
    let line = SOURCES
        .lines()
        .find(|l| l.split(' ').next() == Some(id))
        .unwrap_or_else(|| panic!("{id} is not in sources.txt"));
    let f: Vec<&str> = line.split(' ').collect();
    (
        f[1].parse().expect("a length"),
        u64::from_str_radix(f[3], 16).expect("an FNV-1a"),
    )
}

fn small_ids() -> Vec<String> {
    sealed_model_ids()
        .into_iter()
        .filter(|id| !id.contains("n01000") && !id.contains("n10000"))
        .collect()
}

#[test]
fn the_adapters_bytes_equal_the_independent_python_bytes() {
    let ids = small_ids();
    assert_eq!(ids.len(), 21, "12 RF-LARGE models and the nine");
    for id in ids {
        let m = model(&id).expect("model");
        let bytes = source(&m).expect("the source is accepted").encoding();
        assert_eq!((bytes.len(), fnv64(&bytes)), committed(&id), "{id}");
    }
}

#[test]
fn the_encoding_is_deterministic_and_independent_of_list_order() {
    for id in ["RF-LARGE-TREE-n00010-ROT", "RF-LARGE-CONT-n00010-AX"] {
        let m = model(id).expect("model");
        let once = source(&m).unwrap().encoding();
        assert_eq!(source(&m).unwrap().encoding(), once, "{id}");
        let mut parts = source_parts(&m);
        parts.members.reverse();
        parts.constraints.reverse();
        parts.loads.reverse();
        parts.stations.reverse();
        let reordered = PrimitiveSource::new(parts).unwrap().encoding();
        assert_eq!(reordered, once, "{id}");
    }
}
