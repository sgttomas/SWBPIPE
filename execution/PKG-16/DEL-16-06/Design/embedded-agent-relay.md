# Embedded provider HTTP relay

**Standing: implemented component, not a product start route or stock-session
qualification.** Implements the authorized origin/loopback seam in
[embedded-agent-portability.md](embedded-agent-portability.md). The provider,
credential and model are supplied explicitly by the future trusted coordinator;
no provider setting, secret, process or worker is loaded on module startup.
ScopeOfWork commitments, AC-005 and dependency conditions remain unchanged.

## Authority and request path

`relay_policy.rs` parses a bounded ASCII API-base URL using the maintained URL
parser. Userinfo, queries/fragments, malformed/zero ports, escapes, dot segments
and backslashes are refused. HTTP requires a literal loopback IP; a DNS name or
non-loopback address requires HTTPS. The supplier always uses its private
`/v1/` loopback API. A request's suffix maps into the configured API prefix by
setting the parsed URL's path; input cannot replace the origin, scheme or port.
The minimal subset supports GET and Content-Length-framed POST, without query
parameters, request chunking, CONNECT, upgrades, Expect or route-override headers.
Unsupported compatible-provider variants fail closed; no widening follows.

The local HTTP/1.1 parser requires the actual owned IPv4 or IPv6 authority and
the exact per-start Bearer capability before reading the body or contacting the
provider. Both listeners remain at one host-owned port. Duplicate headers and
ambiguous framing are refused. Every incoming header is discarded. The host
constructs its own Host, Content-Type, Accept, Connection and Authorization
headers; model/client API keys, organizations and forwarding hints cannot become
outbound routing or authorization. No-key mode is explicit, never a fallback
from a missing/failed credential. `Secret` has bounded visible-ASCII input and
no Serialize, Clone, Display or payload-bearing Debug implementation.

Outbound transport uses Hyper's one-request HTTP/1 connection API over an owned
Tokio TCP socket, optionally wrapped in Rustls. It has no redirect, proxy,
connection-pool or request-replay machinery. The system resolver receives only
the configured provider hostname. At most 16 returned addresses are tried under
one DNS/connect/TLS deadline, before any HTTP bytes are sent. A failed HTTP
exchange is never reissued. Each outbound socket requests finite send/receive
buffers. These dependencies are enabled only for macOS; the source introduces
no direct native FFI, framework import, descriptor export or Windows route.

TLS uses Rustls TLS 1.2/1.3 and Ring with **rustls-platform-verifier 0.7.1
as published**, using Security.framework SecTrust on macOS. Agent 0 chose OS
trust so engineering firms' locally installed inspection-proxy roots work across
machines. There is no bundled-root fallback, locally authored/forked verifier,
insecure verifier, hostname bypass, trust-failure fallback or network-fetch disable.
All trust failures refuse the provider exchange before application bytes/auth.
The platform verifier's SSL server-name policy retains hostname verification.

Agent 0 explicitly allowed ordinary SecTrust validation traffic: OCSP/CRL
revocation and AIA intermediate retrieval by the OS trust service to certificate
CAs. It carries certificate identifiers, not prompt, model, project or provider
credential data. This is normal platform TLS validation as used by URLSession;
disabling it weakens revocation checking and would require a maintained-verifier
fork. Origin pinning governs **agent data**, which remains at exactly the
configured provider origin. The sandboxed supplier gains no network authority:
only the host and OS trust service validate. No redirects or environment proxy
are enabled. The installed SDK's `SecTrustSetNetworkFetchAllowed` discussion
states that missing-certificate fetching defaults on for SSL policies; the
published verifier creates that SSL policy and does not disable fetching.
The permissions reviewer must assess this explicit distinction; it is not a
claim that platform validation makes no off-origin connections.

The Cargo lock binds the verifier/dependency versions. The ephemeral TLS fixture
uses no trusted-root installation. Tests do not establish a real organisation's
inspection path, Intel, minimum-OS or packaged TLS compatibility on every Mac.

## Bounds and lifetime

| Resource | Bound or requested allowance |
| --- | --- |
| Local and upstream HTTP headers | 16 KiB, 64 fields; upstream caps configured before Hyper parses |
| Local request body | 2 MiB, Content-Length checked before allocation; one bounded owned body |
| Provider response body | 32 MiB cumulative, streamed; never collected whole |
| Stream writes/credential guard input | 16 KiB per slice |
| Provider key | 8,192 visible ASCII bytes maximum |
| Active client exchanges | 4; each owns at most one provider connection task |
| Listener backlog | 8 per family |
| Local/outbound socket buffers | 16 KiB requested for each send and receive buffer |
| Main runtime worker | 1 MiB requested stack |
| Resolver blocking workers | At most 2, 512 KiB requested stack each |
| Startup channel | One fixed result; no payload/credential diagnostics |

Before worker creation, GENERAL admission reserves 16 MiB state/vendor allowance,
2 MiB thread stacks, 8 MiB nominal kernel allowance and four 16 MiB connection
allowances: **90 MiB**, separate from the loopback scope's 64 KiB and the existing
launcher/driver/dispatcher reservations. Header/body/stream allocations occur
within those held allowances, including after headroom pressure changes.
These are requested/nominal allocations, not allocator enforcement, measured RSS,
a portable kernel hard cap or a retained-solver pricing claim. OS buffer rounding,
TLS/library allocations and actual integrated-session footprint still need native
qualification; numerical coefficients and solver machinery are unchanged.

Header collection has a five-second absolute timeout; request-body and provider
I/O waits have 30-second bounds; an exchange has a 300-second overall bound.
Cancellation drops owned HTTP/socket futures. A connection task is aborted when
its owning handler drops. Runtime destruction waits for its resolver workers;
an OS resolver or synchronous SecTrust evaluation can remain blocked beyond
the asynchronous timeout. A blocked trust evaluation holds the relay worker,
not the separate host protocol/UI polling path. `request_stop`
therefore requests shutdown, while `poll().joined` is the separate actual thread
observation. Failed startup also requires join observation. Dropping unfinished
custody deliberately retains backing and listeners even if the worker later
finishes. No stopped relay/scope is automatically restarted or reused.

The loopback scope grants its relay service once. A one-shot `Endpoint` is minted
only after Ready is observed and carries both listener custody and live readiness,
without the provider secret. The launcher requires that endpoint at preparation
and checks it again immediately before spawn; a bare port/bound scope cannot
substitute. Stop/failure revokes its readiness. The existing original supplier
Child/group custody and unknown-cleanup refusal remain separate duties.

Responses permit JSON and SSE. Only a fixed content-type projection and numeric
status reach the supplier; hop/routing/cookie/redirect headers do not. Provider
redirects, compressed output and oversized headers/declared bodies are refused.
A streaming overflow/error ends without a successful final HTTP chunk. Exact
provider-key bytes are withheld across chunk boundaries; a detected reflection
terminates delivery. This is not detection of encoded/partial disclosure by a
hostile configured provider. Secret/header copies are transient but not proven
zeroized or excluded from OS crash output. No provider body or secret is logged.

Finished/failed counters describe relay transport, not inference success,
application, acceptance, solve cancellation or rollback. Stop, interrupted
streaming and uncertain peer delivery do not erase actual domain records and
cannot cause tool replay. The dispatcher/domain owners retain those records.

## Verification and remaining integration

Focused automated fixtures use invented loopback providers and credentials;
no stock supplier, oMLX/model, keychain or App GUI session runs. They exercise
both local families, path/auth/framing refusal without provider contact,
header replacement, no-key mode, redirect refusal, no delivery replay,
pre-completion streaming, response limits, stalled consumer/provider shutdown,
readiness/one-shot endpoint revocation and retained abandoned-worker accounting.

An isolated owned child clears its environment and sets all proxy variables to
an owned trap. The positive round trip reaches only its real fixture provider.
A read-only lsof observation uses the child's own identity and owned listener
ports, with IPv4/IPv6 positive controls; inherited parent listeners must be absent.
No discovered PID becomes signal authority, and raw identity-bearing observer
output is not printed. This tests the generic exec seam, not stock-Codex custody.
An ephemeral private OpenSSL X.509 v3 certificate fixture verifies untrusted TLS is
refused before provider authorization/application bytes; no trust setting changes.

Before product exposure, integrate generic keychain/settings and sidecar selection,
qualify actual native socket/TLS footprint, stock profile/relay compatibility,
full supplier stdio EOF/hard-app-death cleanup and crash-output exclusion, and
connect the actual workspace/review coordinator. Owner authorization is already
recorded; none of these missing duties recreates an activation permission hold.
