//! Host-owned Codex integration components for RT-E1.
//! No Tauri command or live-session route is enabled by these modules alone.

pub(crate) mod catalog;
pub(crate) mod framing;
#[cfg(target_os = "macos")]
pub(crate) mod macos;
pub(crate) mod numeric;
pub(crate) mod protocol;
#[cfg(target_os = "macos")]
pub(crate) mod sandbox;
