//! Host-owned Codex integration components for RT-E1.
//! No Tauri command or live-session route is enabled by these modules alone.

pub(crate) mod catalog;
pub(crate) mod dispatcher;
pub(crate) mod framing;
#[cfg(target_os = "macos")]
pub(crate) mod loopback;
#[cfg(target_os = "macos")]
pub(crate) mod macos;
pub(crate) mod numeric;
pub(crate) mod protocol;
#[cfg(target_os = "macos")]
pub(crate) mod sandbox;
pub(crate) mod session_driver;
pub(crate) mod stdio_pump;
pub(crate) mod webview_bridge;
#[cfg(target_os = "macos")]
pub(crate) mod relay_policy;
#[cfg(target_os = "macos")]
pub(crate) mod http_relay;
#[cfg(target_os = "macos")]
pub(crate) mod supplier;
#[cfg(target_os = "macos")]
pub(crate) mod provider_store;
#[cfg(target_os = "macos")]
#[allow(unsafe_code)]
mod keychain_native;

#[cfg(target_os = "macos")]
#[allow(unsafe_code)]
mod process_scope;

#[cfg(target_os = "macos")]
pub(crate) mod live_host;
#[cfg(target_os = "macos")]
mod live_runtime;

#[cfg(all(target_os = "macos", feature = "native-smoke"))]
pub(crate) mod session_fixture;
