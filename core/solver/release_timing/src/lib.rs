//! The release timing driver's self-supervision (`release_timing --supervise`): the Python
//! runner's watch, classification and records, ported so that the driver can be carried to a
//! stock Mac and run with nothing installed. The driver mode itself is `src/main.rs`; nothing
//! here is used by it or by any timed stage.

pub mod classify;
pub mod host;
#[cfg(target_os = "macos")]
pub mod supervise;
