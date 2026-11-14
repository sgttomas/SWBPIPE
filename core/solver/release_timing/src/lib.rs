//! The release timing driver's self-supervision (`release_timing --supervise`): the Python
//! runner's watch, classification and records, ported so that the driver can be carried to a
//! stock Mac and run with nothing installed. The driver mode itself is `src/main.rs`; nothing
//! here is used by it or by any timed stage.

pub mod classify;
pub mod host;
#[cfg(target_os = "macos")]
pub mod supervise;

/// The build's identity, embedded by `build.rs` from the tree the binary was linked from (the
/// script runs on every build): `source_commit`, `source_tree`, `source_dirty` (`true`, `false`,
/// or `"unknown"` where git cannot establish it, never `false` then), `build_profile`, `rustc`
/// and `cargo`. `release_timing --build-identity` prints it; the supervise mode's metadata
/// records it.
pub const BUILD_IDENTITY: &str = env!("RELEASE_TIMING_BUILD_IDENTITY");

/// The argument, followed by a pid, that `release_timing --supervise` puts first on the driver it
/// starts: the driver watches that pid and stops when it is gone (`src/main.rs`
/// `watch_supervisor`). A driver started without it watches nothing. It is not part of the
/// driver's usage.
pub const SUPERVISOR_PID_ARG: &str = "--supervised-by";
