# Portable embedded agent delivery

**Standing: authorized implementation contract; implementation and qualification
remain incomplete.** RT-E1 / exit criterion 17. The activation basis is the
owner's 2026-10-29 direction in [embedded-agent-host.md](embedded-agent-host.md).
This design replaces the fixed-provider assumptions of the unwired prototype;
it does not enable that prototype unchanged.

## Runtime and dispatcher first

Reuse the bounded private-stdio protocol/session driver. Add the dispatcher
before the webview completion command, proposal review and minimal panel.
Construction/start is explicit. No process, thread, listener, keychain lookup,
network connection or supplier acquisition occurs during module/global startup.
Preserve the early solver-worker dispatch and all solver-specific lifecycle,
memory pools, allocator and watchdog rules.

The Rust dispatcher binds calls to the originating host session, admitted
webview, workspace and application generation. It retains original
thread/turn/call/tool identities and the original RPC destination privately;
webview completion supplies only an opaque request reference and bounded
outcome. Source-byte/numeric admission precedes materialization at each raw
boundary. Current-basis calls and retained proposal/run evidence keep the
catalog's distinct rules. Reconnection never redispatches old work.

Actual domain operation outcomes remain outside protocol send state. A queue
refusal, detached turn, lost connection or failed delivery cannot erase an
applied/unknown outcome, rewrite its original/current bases, or authorize a
retry. Proposal registration never applies; the person's later act uses the
existing immutable batch review and sole operation engine. Preserve assumption
and rationale context and native operation discriminators. Extend the catalog
itself for criterion 17; no parallel field/operation language is introduced.

## User provider configuration and credentials

The person configures provider endpoint, model identifier and credential mode.
Local plain HTTP is limited to literal loopback addresses; a non-loopback
provider origin requires HTTPS with certificate and hostname validation. Parse
the origin with a maintained URL parser. Reject userinfo, fragments, ambiguous
ports and unsupported schemes; validate any configured API path prefix.
Do not inherit developer provider settings, proxy environment, Codex homes,
instructions, MCP servers or authentication files.

Provider metadata may use the app's owned settings route; secrets never enter
that document. Credentials entered through the app go directly to a
product-generic keychain service/account, scoped to an opaque provider profile.
Secret values are never returned in settings responses or persisted in files,
argv, logs, error text, crash reports or the repository. Credential absence,
locked/unavailable keychain and failed writes have explicit outcomes; none
falls back to a file or a different provider. Native API wrappers, permissions
and persistence behavior receive independent review before integration.

The relay retrieves the provider secret privately. Codex receives only a
fresh per-start loopback capability, not the provider key. Keep both transient
values out of diagnostics. Bearer credentials are attached only to the
configured outbound provider origin. Credential mode without a key is an
explicit configuration for an endpoint that needs none, not an auth fallback.
The minimal panel must not show configuration/credential entry on platforms
where this route is unavailable.

## Rust-owned loopback relay

Use the relay for both local and hosted providers, so Codex always has the same
single loopback destination. Rust owns a runtime-selected port and retains the
listeners through supplier/worker cleanup. Reserve both IPv4/IPv6 loopback
routes admitted by Seatbelt's localhost filter, or refuse startup if equivalent
ownership cannot be established; never permit an unowned alternate endpoint.
The supplier profile allows outbound loopback only at that port, keeps fork
denied and permits exec only at the verified supplier resource's runtime path.
The accepted libinfo rule/condition remains explicit. No general outbound
network, system-profile import or unsandboxed retry is introduced.

The current unwired implementation supplies only the scope/configuration part:
`loopback.rs` owns the two named addresses and a fresh capability; `macos.rs`
retains it through original supplier-group cleanup and constructs secret-free
configuration for an explicit model; `sandbox.rs` substitutes its owned port and narrows the old IP grant to TCP,
without a UDP grant.
A bound scope does not signal a ready/authenticating relay. Its component checks
do not qualify Seatbelt alias coverage or an HTTP relay.
The future relay owner must keep listener custody through worker termination,
admit kernel/HTTP/stream/thread costs separately, qualify listener non-inheritance
on the selected native path, enforce the request rules below
and qualify the exact profile before a start route can become available.

Authenticate each local request with the per-start capability. Reject absolute
request targets, CONNECT, upgrades and routing headers that could select
another destination. Enforce the configured API-prefix boundary, bounded
headers/bodies/concurrency and admitted streaming buffers. Do not accumulate
whole provider responses or block protocol polling behind provider I/O.
Exact caps and buffer/thread leases belong to the implementation review; they
must precede allocation/start and survive blocked workers until observed exit.

The host connects only to the configured provider origin, with TLS for hosted
providers. Disable implicit proxy environment and redirects; preserve verified
server-name identity through DNS/address changes. Strip hop-by-hop headers and
all client authorization/routing overrides before applying the configured
credential. No automatic relay replay, origin change, auth fallback or diagnostic
body capture is allowed. Timeouts and cancellation close owned streams and
report delivery uncertainty; they do not invent domain cancellation.

## Supplier, paths and platform availability

Pin official stock Codex 0.160.0 separately for macOS arm64 and x86_64 by byte
size/SHA-256. Prefer target-architecture bundled sidecar resources under the
existing signing/packaging route; include the supplier licence and required
third-party notices. Verify runtime resource bytes and architecture before exec.
No developer checkout/cache path, username, model name, endpoint, fixed provider
port or provider-specific keychain item belongs in product configuration.

If bundling is actually blocked, use a verified first-use download into app
support: bounded transfer, pinned version/hash, private staging and atomic
publication, with no execution before verification. No silent alternate version,
architecture, search-PATH executable or unverified cache fallback. Signing,
minimum-OS compatibility and redistribution evidence remain required; no support
range is changed by assumption. The existing arm64 witness binary declares
macOS 11.0 minimum; Intel metadata/evidence is still to be established.

Private home/cwd, profile and exact exec path are derived at runtime from
app-owned locations. Test invented homes/usernames, aliases and unrelated data
without relying on a developer home. Settings/resource/keychain lookup failures
produce unavailable states. Windows and Linux show the reason that qualified
isolation is unavailable and never execute an unsandboxed supplier. Windows
isolation and VM provisioning are not opened by this work.

## Cleanup and automated qualification before exposure

Retain each actual original Child/leader reservation unreaped through original
group operations. Never reconstruct signal authority from saved PIDs or a
stale marker. Preserve EOF, leader exit, worker join, group-empty observation,
protocol completion and actual domain outcome as distinct facts. Unconfirmed
cleanup retains refusal state and owned accounting; a subsequent launch reaps
within retained authority or refuses stale state.

Before a launcher command/panel start can be exposed, automate the selected
pin/profile/relay path's filesystem, network, no-fork/self-spawn/exec boundaries
and cleanup under normal stop, stdin EOF and hard app death, including active
turn and pending tool cases. Maintain independent rescue authority in fixtures;
never hard-kill the sole supplier owner to discover whether cleanup works.
The original no-fork proof covers the pre-observer startup interval. Test
unrelated-group preservation, stalled streams, partial startup, refusal/restart
and credential absence from diagnostic/crash output. Test both supported Mac
architectures/platform cases or state exactly which portability evidence is
missing; an arm64 pass is not an Intel witness.

After cleanup/native isolation duties are established, complete the full local
model dynamic-tool round trip through the integrated runtime and actual review
path. Criterion 17 additionally needs its build/solve/explain operation schemas,
stale/duplicate/invalid completion and interruption discriminators. The owner
is AFK: these evidence runs are automated, not dependent on App's held GUI A/B.
No software result records professional acceptance, issuance or release.

All new runtime processes, listener workers, queues and retained outcomes are
priced/admitted separately from solver machinery. Retained-route pricing that
moves uses the exclusive-window protocol. Heavy work stays in chirality-t3
trees through the slot/Cargo wrappers under shared 64 GiB and 22% free limits.
oMLX is the current local witness provider, outside slots with at least 50 GiB
headroom and stopped when idle; it is not a product prerequisite.
