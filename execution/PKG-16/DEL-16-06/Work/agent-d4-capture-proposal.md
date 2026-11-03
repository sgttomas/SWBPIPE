# D4 supplier-envelope capture: bounded authorization proposal

Capture specification CAP-1, amendments D1 and HC-1. This specifies the smallest later observation to
replace the reconstructed supplier envelope in D4-4. The maintained adapter is
`tools/agent_supplier_capture.py`, with mock-only tests beside it. The exact
reviewed/integrated revision is identified by its PR; `result.json` records the
selected revision and source/input digests on an authorized invocation. No
supplier, desktop App, listener, provider, model, credential or actual capture
was launched during preparation. This document authorizes no execution.

## Supplier-only route; no desktop App required

The unknown is what stock Codex serializes after the selected initialize,
thread/start and two turn/start requests. A standalone app-server driver can
observe that without launching a Tauri App, a model, oMLX or the solver.
The dynamic calls receive the already-invented stub14 host outcomes, rather
than applying a model operation. This establishes serialization for those
synthetic exchanges only; the actual webview journey remains stub14's evidence.

The adapter selects these existing maintained components:

- `tools/agent_host_witness.py::Native`: owned unreaped supplier/session group,
  bounded stdio pump and shutdown. Do not call its `run()` (that reads Keychain,
  uses one tool, fixes 8,192 context and reaches oMLX).
- `tools/agent_journey_fixture.py::StubProvider`, `Script`, `TURNS`: the existing
  loopback Responses event-stream script. Its current request records contain
  metadata only. The capture-only subclass bounds, parses and admits raw
  **request body bytes**, then retains those unchanged bytes with byte counts
  and digests. Malformed/unexpected/auth-bearing requests fail without body
  retention; any failed capture removes its partial body files. No HTTP header
  values or inherited environment are retained.
- `embedded_agent/protocol.rs::{initialize,start_thread,submit_text}` supplies
  the selected RPC shapes, and `live_runtime.rs::INSTRUCTIONS` supplies the exact
  base text. Use all seven catalog declarations, with references resolved as
  `Catalog::declarations()` does; use a fresh empty cwd and synthetic home.
- The selected macOS config recipe is `embedded_agent/macos.rs::supplier_config`:
  provider id `swbpipe_relay`, model id `Qwen3.6-35B-A3B-8bit`, context 32,768,
  reasoning low, no admitted environments, read-only/never, built-ins disabled.
  Reproduce that recipe with the capture stub as its loopback provider; the
  original `Native` recipe is **not** equivalent without this adaptation.

`CaptureNative.configuration` uses the selected `macos.rs` CONFIG_BODY, the
package's client version, production client name `swbpipe`, provider id and
base instructions, with a direct no-auth stub URL and the relay `env_key`
removed. Declarations and all outgoing frame maps use recursive lexical key
order and unescaped UTF-8, matching product serde_json's selected configuration.
Its frame limit is 128 KiB. The historical Native recipe keeps its
8,192 context and optional credential carrier. Its process/pump/stop/close
mechanism is reused. A narrow partial-startup repair initializes ownership
bookkeeping before spawn and conditionally joins a started reader, so reader
startup failure can still close the original unreaped group; no new signaling
or reap mechanism is introduced. The default StubProvider POST/GET behavior is
unchanged; the capture subclass uses the factored handlers for admission.

Host results come only from maintained
`fixtures/model_operations/agent_w9a_capture_outcomes.json` (six synthetic
outcomes). The field shapes and six numerical rows derive from verified W9-A
stub14; opaque handles/hashes are deliberately invented. A regression compares
the numerical rows to maintained W9-A references. This fixture is not an
actual current tool execution, and tests/runs do not depend on a dated scratch
report. Script refuses a history whose outputs differ from the selected fixture.

The capture owns one fresh supplier process from pinned stock ARM Codex
0.160.0, executable SHA-256
`112fae7a5a1223e673c8a1791d32338f37df8b527ff1159bb8adac6c4dbf1b4b`.
No acquisition, supplier re-sign, login, developer credential store or real
provider credential is involved. Route directly to the no-auth stub; record
the difference from the product's HTTP relay. Preserve product-equivalent
thread/config bytes except the disposable endpoint/cwd/scope values, bind
those actual bytes, and classify any endpoint-derived prompt text explicitly.
No capability token is needed or supplied. The source-selected process wrapper
uses a minimal synthetic HOME/CODEX_HOME/environment, denies the real HOME
except the selected executable/runtime, and limits outbound network to the
owned stub port. The actual product relay/sandbox route is not exercised;
record that distinction. Do not relax isolation if startup refuses.

## Concrete later invocation

From a clean checkout of the reviewed adapter revision, under the host's ordinary
slot wrapper (memguard running), the complete adapter command is:

```sh
python3 tools/agent_supplier_capture.py --supplier "$D4_SUPPLIER" --output "$D4_CAPTURE_OUTPUT"
```

Agent 0 has the exact retained ARM supplier locator beneath the original
retained bundle's `Contents/Resources/agent-supplier/codex-aarch64-apple-darwin`.
Bind `D4_SUPPLIER` to that file and `D4_CAPTURE_OUTPUT` to one fresh private
evidence directory outside Git, both named in the authorization. The tool
checks the executable pin, refuses an existing output path, accepts no provider,
model, key or retry argument, and writes the packet described below. It neither
asks for nor manufactures authorization. Preparing this command is not running it.
The checkout must have no tracked or untracked changes and output must be
outside Git; admission refuses otherwise before preparing a listener/supplier.
The packet binds `agent_w9a_journey.json` as well as the six-outcome fixture,
catalog and source modules. Checkout revision/cleanliness are checked again
after cleanup; any change prevents completion.

## Boundaries, artifacts and cleanup

Request one supplier-only scripted capture, under the ordinary host slot wrapper
with memguard; no model and no exclusive timing/RSS claim. Maximum five minutes
wall time, two turns and the existing six host tool calls: summary/proposal;
summary/observe/solve/results. Default expected Responses request count is eight
(three then five), as stub14. Unexpected calls, extra turns or retries fail the
capture. Enforce existing 128 KiB stdio frame bounds; request-body bound 2 MiB
each, aggregate body bound 16 MiB, and response bounds from the maintained stub.
No filesystem/network tool request is admitted. HTTP is loopback to the owned
stub only; no request to localhost:8000 or any external host.

Record one bounded private packet: input revision; exact supplier/config/RPC/
catalog/Script/TURNS/outcome hashes; the eight nonsecret raw Responses body
files; response-script identity; original RPC/tool-call binding; clean terminal
states; shutdown/absence evidence. The historical stub14 outcome report was
`3d30ebf1bc5c767d8ce6d9f05cbfb57abfe77f8c45ca7efa3eb4e3cbb15e72e6`;
the capture's synthetic fixture is a distinct maintained input, bound by the
packet's digests. Record the actual reviewed adapter revision/hash before
execution. The maintained witness process wrapper uses an
isolated temporary runtime outside the real HOME, as its source explains.

Always close the owned supplier through its unreaped session/group mechanism,
stop/join the owned stub, close pipes, confirm exit and executable-path absence,
then remove the synthetic HOME/runtime. Never signal a saved PID, remove the
original supplier/bundle, or operate on another session's scratch. If cleanup
is unconfirmed, stop and report; no second attempt. Keep only bounded nonsecret
capture bytes for the named recount consumer; remove temporary build/copy output.

The guard stops the owned group at 285 seconds, reserving fifteen seconds of
the proposed five-minute interval for inherited shutdown and provider join.
The stdio driver independently checks deadlines and bounded event counts.
Overrun, expired guard, unexpected late HTTP input, resident executable or any
unconfirmed cleanup prevents a complete capture. Cleanup phases are attempted
independently; uncertain custody retains its runtime for recovery. The guard is
not an OS real-time scheduling guarantee, so record actual elapsed time.
Parser errors, incomplete connections, idle timeouts, oversized headers and unsupported HTTP methods latch failure
through the common error hook. Socket timeouts apply before parsing; capture
handlers are non-daemon and joined before the final locked failure read.
Discovery admission and failure latching use the same lock, so simultaneous
duplicate discovery is not admitted. Harmless protocol notifications without
a thread id are informational; foreign-thread and built-in execution items
still refuse. Unsupported server requests get product's `-32601` response but
also fail this strictly scripted capture; no extra host capability is admitted.

On success retain exactly eight raw request files, `config.toml`, and bounded
`result.json` containing setup/turn RPC inputs, original dynamic-call identities,
source/config/input hashes, supplier exit, executable-path absence and cleanup
standing. On failure keep fixed reason codes and cleanup facts, without request
body files. Body retention is limited to this no-credential, invented-data scope:
the recursive credential-field/text guard is defense in depth, not a universal
secret scanner or permission to capture another session's traffic.

### Tools-vector refusal diagnostics (D1)

The seven-tool/name/protocol guard is unchanged. A tools-vector refusal now
uses one fixed subcondition: `request_tools_vector_type`, `request_tools_count`,
`request_tools_member_type`, `request_tools_name_type`,
`request_tools_name_duplicate` or `request_tools_name_set`. The existing secret
guard still runs first and may refuse without a tools diagnostic.

Only that refusal adds `toolsDiagnostic` to `result.json`. It contains fixed
JSON type categories for the request/vector/members, a member count capped at
eight with a `count_capped` boolean, and type-tag counts (`function`, `namespace`
or fixed non-value categories). It summarizes at most the first eight members.
Recognized tool identities are seven allowlisted IDs with boolean values;
unknown names are only a capped count and type-category counts. No untrusted
name string, nested wrapper content, schema, description, argument, header or
auth value enters the summary. The `namespace` tag is a diagnostic category,
not a new admitted tool shape. When capped, missing IDs mean only "not seen in
the sample", not absence from the whole vector.

The summary is assigned once under the existing capture lock and read after
handler cleanup. Its fixed fields/enumerations and maximum count of eight bound
the output independently of input-name length. Success stores the same raw
bytes and no diagnostic; failure still removes every request body, including
previously admitted bodies. No custody, timeout, body-budget, listener, provider,
secret-guard or CLI authority is added by this metadata-only repair.

### Consumed attempt and proposed diagnostic retry

The owner authorized one actual attempt at selected `53a708da44fee73a16876286533792ab88334c7f`.
It refused `request_tools` in 2.804251125 seconds, with supplier exit 0,
`cleanupFailures=[]`, `residents=[]` and no retained runtime. No request body was
admitted or retained. The source-bound record remains unchanged at
`/private/tmp/swbpipe-d4-envelope-53a708da4/result.json`, SHA-256
`7de3f3889648d412d9fe536d750e9276f5c9b376b027f878abcaa8bc05bbfff4`;
its no-key config is SHA-256
`47957540e6d1413437d83d8baf21a7e3450adbfccd700e6d82ebc07c6a97dcb1`.
That attempt's grant is consumed. D1 was prepared entirely offline and does not
retrofit a diagnosis into that earlier packet.

What source proves: the old adapter required a JSON array of exactly seven
object members with distinct string names, each declared catalog suffix present
once; the combined refusal did not distinguish those checks. The rejected
request's vector shape, count and names remain unknown. An exact pinned
supplier serialization checkout was not available in the consulted local
source locations; a generic implementation path or source URL would not prove
the rejected shape. No legitimate old/new protocol-shape mismatch has been
established, so no admission contract was broadened.

For Agent 0/owner decision only, propose one new diagnostic attempt using the
reviewed D1 revision and the same pinned supplier, with fresh output
`/private/tmp/swbpipe-d4-envelope-toolsdiag-1`. Keep seven tools, two turns, six
host calls, eight bodies, the 2 MiB/16 MiB/128 KiB bounds, five-minute interval
and owned cleanup unchanged. Its command is the concrete invocation above with
that new output; do not reuse or overwrite the earlier directory. A repeated
refusal may yield only the bounded structural summary. It does not authorize
a relaxed guard, a second retry, raw rejected-body retention or a joined/live
witness. No attempt was launched while implementing or testing D1.

Threat model for scoped review: malformed/truncated/oversized HTTP or stdio,
duplicate/unexpected calls/requests and false completion, wrong protocol binding,
accidental secret-bearing headers/known credential fields, I/O/reader startup
failure, timeout and cleanup failure in this private no-key scripted run. An
adversarial same-uid process, OS compromise, universal secret detection and
runtime equivalence to the desktop/oMLX path are outside the assurance. Tests
inject all supplier/listener/clock effects; mock completion proves adapter
behavior, not actual envelope bytes or real cleanup/termination.

## Recount and what still remains live

### HC-1: supported recipe correction after exact-source trace

An owner-authorized D1 attempt at `deed5855033fbace953fc221c644ec501d9328ea`
refused `request_tools_count` after 0.709694125 seconds, with supplier exit 0,
no cleanup failures/residents and no retained runtime/body. Its capped first
eight entries were function objects, with four recognized IDs (summary, nodes,
records, propose) and four unknown string names; `count_capped=true` proves at
least nine total entries. It does not identify unknown names or the exact
total, and the other IDs not seen in the sample need not be absent. No tool
execution/reachability is proved by those declarations. Its untouched result
at `/private/tmp/swbpipe-d4-envelope-toolsdiag-1/result.json` has SHA-256
`f3f73e66c36824ffd637ff8ce2cc853dd3c9f5ff2a3f7084aa4bc4421c3ebd55`;
config SHA-256 is `535c9af20dc2b13a0b62769731839b032779ef53e65c44fa763757fd5bdc9bb7`.
Both prior attempt grants are consumed; HC-1 authorizes no new attempt.

Public source was inspected only at the maintained supplier's exact
`openai/codex` commit `a956835d020762cb2b570053af06f643a11c0ecc`.
The annotated release tag and release metadata resolve to it, and the ARM
archive digest matches the maintained supplier pin. This binds metadata and
source selection, not an independently reproduced binary build.

- [`core/src/tools/spec_plan.rs:150`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/tools/spec_plan.rs#L150)
  adds core/extension tools before dynamicTools; lines 1426–1437 register each
  supplied function. Dynamic tools are additive, not an exclusive ceiling.
- [`core/src/config/mod.rs:2705`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/config/mod.rs#L2705)
  defaults `experimental_request_user_input` on when the tools entry is absent;
  spec_plan lines 1169–1175 register it independently of environments.
- [`features/src/lib.rs:1702`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L1702)
  defaults `goals` on. App-server `extensions.rs:76–88` installs the goal
  contributor when its backend exists and binds that feature. The goal
  [`extension.rs:529`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/ext/goal/src/extension.rs#L529)
  contributes get_goal/create_goal/update_goal;
  [`runtime.rs:126`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/ext/goal/src/runtime.rs#L126)
  gates visibility as well as availability on enabled. Ephemeral state can
  refuse their execution while declarations remain visible.
- [`config/src/config_toml.rs:672`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/config/src/config_toml.rs#L672)
  defines the nested `experimental_request_user_input.enabled` control.
  The goal feature and this tools control are supported configuration, not
  guessed keys or a supplier patch.
- [`core/src/client.rs:888`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/client.rs#L888)
  carries the visible prompt.tools vector to Responses; `tools/src/tool_spec.rs`
  serializes the full vector. Existing environment/shell/edit restrictions do
  not filter session helpers out at this stage.

The old selected product/capture recipe omitted both controls. With the normal
goal backend and fallback metadata, source predicts eleven declarations:
seven supplied tools plus the user-input helper and three goal helpers. That
is a source-derived explanation consistent with the actual sample, not an
observed eleven-entry packet or identification of its four unknown strings.
These supplier-handled session/UI/state helpers are distinct from shell/edit/web
execution. `update_plan` defaults off at this exact revision and is not the
identified default-on source. No hidden-name assumption changes admission.

HC-1 adds only `goals=false` to the existing production `[features]` and
`[tools.experimental_request_user_input] enabled=false`. CaptureNative inherits
that body without adapter/filter/allowlist changes. Tests parse both recipes,
prove the controls' nested placement/false values, and compare every prior
field unchanged, including relay credential custody and endpoint differences.
The maintained source-read graph validates without changing an unrelated hash
or inventory; the existing source-bound capture packet records the selected
macos.rs digest at any future authorized run. Historical packet hashes and
old executable identities are not repinned to these new bytes.

**Concrete next capture proposal:** from the reviewed HC-1 integrated revision,
use the same command and pinned supplier with a fresh output
`/private/tmp/swbpipe-d4-envelope-helper-controls-1`, ordinary slot wrapper and
memguard. Keep seven tools/two turns/six host calls/eight bodies, all previous
size/time/isolation/owned-cleanup limits, and stop on any refusal without retry.
No desktop build is needed for this source-derived supplier-only recipe.
A later joined native witness must first rebuild/rebind its executable because
the old one embeds the old recipe. No supplier, listener, App, model, server,
monitor, credential or capture was launched to implement/test HC-1; a new
capture/owner decision remains required, not requested or inferred here.

Recount raw serialized instructions/input/tools through the pinned installed
oMLX Responses conversion and strict Qwen template path, not the adapter's
assumed chat messages. Bind special-token, generation-prompt and thinking
options. Compare the earlier reconstructed bodies/counts to locate supplier
instructions, schema transforms, developer/reminder blocks and history
retention; revise the context reserves if needed. No actual model generation
or accepted model operation is inferred from the script.

Installed source hashes read locally (no full settings disclosure):

| Source beneath oMLX Resources | SHA-256 |
| --- | --- |
| `omlx/server.py` | `fdffe13f58a66c88a7517c04e2dbbfd35085f5e3ff8de2f68352c73a4570e767` |
| `omlx/model_settings.py` | `b26777fc4775dcd3f007a3e6cfca05f6806a8cf45720184eeb7bd37e6f200a63` |
| `omlx/engine/batched.py` | `da44167320e79fb17b81f7da4983bc82b2abd3649594b44c3680d10f2a660895` |
| `omlx/api/responses_utils.py` | `d7f123d9e770133655d94eedb07b347fc9215b4f85000b0a84f3d9c1dc2e5c93` |
| `omlx/api/tool_calling.py` | `9b770f83fa8b0019ae536f7476365826cced9acfd21fb95ebd11c3552469d6c8` |
| `omlx/api/utils.py` | `009ca414afe2ae488b51a1a24bc4b4ea0498929a2ed8824d3d53e6bfaded455f` |
| `omlx/utils/tokenizer.py` | `cf4b076ab581fd4f79c48f90165197a1139eada8bcff770beefe27653ee3eddc` |
| `omlx/reasoning_effort.py` | `28bae031925ed870c14caf11c6fca9ca650bff1fb8cec03af04fa6ad24d196be` |

Source-only defaults narrow the plan: model/profile files absent, no persisted
template/forced kwargs/thinking override, Qwen local tokenizer unless the
loader is customized, `trust_remote_code=false`. Null MCP config does not prove
no runtime MCP manager: server.py's Responses path can merge its tools.
Actual launch/base/custom-loader/engine selection, template runtime kwargs,
effective MCP tool set, served context and usable output allowance remain
later authorized live prerequisites. `max_tokens=32768` is an output setting,
not proof that that output plus the prompt fits 32,768. Authentication, system
headroom, model unload and owner-attended Apply remain separate observations.

The smallest authorization sought here is the single **supplier-only, no-key,
no-model scripted loopback capture** above. It does not include the desktop App,
Keychain, oMLX/model start or a joined witness. Agent 0 presents/coordinates any
such authority; ordinary preparation does not consume it.
