# D4 supplier-envelope capture: bounded authorization proposal

This specifies the smallest later observation to replace the reconstructed
supplier envelope in D4-4. It authorizes no execution. No capture adapter or
native/supplier/network test was built or run in this monitor assignment.

## Supplier-only route; no desktop App required

The unknown is what stock Codex serializes after the selected initialize,
thread/start and two turn/start requests. A standalone app-server driver can
observe that without launching a Tauri App, a model, oMLX or the solver.
The dynamic calls receive the already-invented stub14 host outcomes, rather
than applying a model operation. This establishes serialization for those
synthetic exchanges only; the actual webview journey remains stub14's evidence.

Existing maintained components suffice as input basis, with a small bounded
capture adapter still to be prepared after this proposal is selected:

- `tools/agent_host_witness.py::Native`: owned unreaped supplier/session group,
  bounded stdio pump and shutdown. Do not call its `run()` (that reads Keychain,
  uses one tool, fixes 8,192 context and reaches oMLX).
- `tools/agent_journey_fixture.py::StubProvider`, `Script`, `TURNS`: the existing
  loopback Responses event-stream script. Its current request records contain
  metadata only. The adapter must retain bounded raw **request body bytes**
  before JSON parsing, along with byte counts and digests, and require no
  Authorization header. Do not retain arbitrary HTTP headers or environments.
- `embedded_agent/protocol.rs::{initialize,start_thread,submit_text}` supplies
  the selected RPC shapes, and `live_runtime.rs::INSTRUCTIONS` supplies the exact
  base text. Use all seven catalog declarations, with references resolved as
  `Catalog::declarations()` does; use a fresh empty cwd and synthetic home.
- The selected macOS config recipe is `embedded_agent/macos.rs::supplier_config`:
  provider id `swbpipe_relay`, model id `Qwen3.6-35B-A3B-8bit`, context 32,768,
  reasoning low, no admitted environments, read-only/never, built-ins disabled.
  Reproduce that recipe with the capture stub as its loopback provider; the
  original `Native` recipe is **not** equivalent without this adaptation.

The capture owns one fresh supplier process from pinned stock ARM Codex
0.160.0, executable SHA-256
`112fae7a5a1223e673c8a1791d32338f37df8b527ff1159bb8adac6c4dbf1b4b`.
No acquisition, supplier re-sign, login, developer credential store or real
provider credential is involved. Route directly to the no-auth stub; record
the difference from the product's HTTP relay. Preserve product-equivalent
thread/config bytes except the disposable endpoint/cwd/scope values, bind
those actual bytes, and classify any endpoint-derived prompt text explicitly.
If matching the supplier recipe needs a capability token, use an invented
capture-only token and do not call that credential evidence. Do not silently
relax the outer isolation policy to make initialization work.

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
states; shutdown/absence evidence. The retained stub14 outcome report is
`3d30ebf1bc5c767d8ce6d9f05cbfb57abfe77f8c45ca7efa3eb4e3cbb15e72e6`;
fixture/catalog/driver identities are in D4-4. Record the adapter's actual
reviewed revision/hash before execution; this proposal supplies no invented
adapter identity. The maintained witness process wrapper may require an
isolated temporary runtime outside the real HOME, as its source explains.

Always close the owned supplier through its unreaped session/group mechanism,
stop/join the owned stub, close pipes, confirm exit and executable-path absence,
then remove the synthetic HOME/runtime. Never signal a saved PID, remove the
original supplier/bundle, or operate on another session's scratch. If cleanup
is unconfirmed, stop and report; no second attempt. Keep only bounded nonsecret
capture bytes for the named recount consumer; remove temporary build/copy output.

## Recount and what still remains live

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
