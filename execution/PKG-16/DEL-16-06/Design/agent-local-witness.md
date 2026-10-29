# Pinned local dynamic-tool witness

Status: actual supplier/local-model round trip and bounded denial probes
observed on 2026-10-28 (local date). Desktop session integration, new Tauri
permissions and criterion 17's full reference-system journey are not exercised.
This is RT-E1 transport evidence, not AC-005 completion or product qualification.

## Selected artifacts

[`agent-witness-pin.json`](agent-witness-pin.json) records the exact supplier,
server and installed model identity. Codex stock 0.160.0 was acquired from the
official release URL in that file: archive 95,893,648 bytes, SHA-256
`07c3c7ca376a8f791115342f53138dda37e97cfa29b8125d0652d93784894b5d`;
extracted executable 241,555,024 bytes, SHA-256
`112fae7a5a1223e673c8a1791d32338f37df8b527ff1159bb8adac6c4dbf1b4b`.
The archive matched the release asset's published digest and the executable
reported `codex-cli 0.160.0`. It remains outside the repository.

The server was the existing oMLX 0.6.4, application build 2529, at
`http://127.0.0.1:8000/v1`. The already-installed model source identity is
`mlx-community/Qwen3.6-35B-A3B-8bit`; oMLX serves it as
`Qwen3.6-35B-A3B-8bit`. Its 19 runtime configuration/tokenizer/template/weight
files total 37,748,365,700 bytes. The sorted file manifest digest is
`29e5650499c3c29e8a7fef2d136c9b50b140dd251d8f893b291e66db47b26b58`,
using the exact manifest construction recorded in the pin. No model/server
download, model substitution or server-setting change occurred.

The tested catalog is `swbpipe-catalog/2`, SHA-256
`fe1439998d709e5f3e9641a4d39567000ced56c498fab5b6b8f2028df7c9f669`,
integrated through #1736 (merge `31084e1f79`). The exact tested tool source
`tools/agent_host_witness.py` has SHA-256
`29103be6ba846d681e634c23b0fa4fdd4078c59b317b123a94916be0befe6e3e`.
Its source and focused offline tests are maintained in this project; there is
no dependency on the App checkout or uncommitted App specimen.

## Observed round trip

The stock process initialized over private stdio with experimental API opt-in.
A thread offered only the catalog's `swbpipe_read_model_summary` dynamic
definition. It used the configured local provider, an 8,192-token client context
window, low reasoning effort and a short invented-data instruction. A model
turn produced one actual `item/tool/call`. The witness checked the original
conversation/turn/call and RPC identity, admitted empty arguments against the
catalog schema, validated the invented result schema, and returned that result
to the original RPC id as an `inputText` content item with `success: true`.

The model's completed follow-up contained the exact host-generated workspace
handle `witness-627a35754e8c3e71` and node count `725`. Neither value appeared in
the user request. This is a real model call/result/follow-up observation; the
workspace summary is invented and is not a read from the live SWBPIPE desktop.

Four subsequent turns requested harmless shell execution, reading an invented
canary outside the admitted runtime directory, writing an invented marker
outside that directory, and fetching `https://example.com`. Each completed with
an unavailable-capability response. No corresponding built-in execution item
was observed, the undisclosed canary did not appear in a response, and the write
marker did not exist. The host admitted no dynamic call for these denial turns.

These are bounded behavioral observations for this pin/model/configuration.
Verbal refusal and absent execution items alone are not universal proof of tool
absence or every possible supplier capability. The witness reports that limit
explicitly. It does not expose client RPCs such as `command/exec`, `process/*`
or `fs/*` through the dynamic adapter.

## Isolation and resource basis

The child received a fresh Codex home, separate synthetic OS home, empty working
directory and a minimal environment. Its only provider credential came from the
owner-supplied login-keychain item into the private child environment. The
configuration stores only the environment-variable name; no credential value
is written to files, stderr, output, the repository or a command argument.
The witness does not read or reuse the developer's Codex credential store.

Configured controls include read-only sandboxing, `approvalPolicy: never`,
`environments: []`, disabled web search, and disabled shell/unified-exec,
code-mode host, plugins/apps, collaboration and browser/computer/image features.
The outer macOS sandbox permits outbound network only to localhost port 8000,
denies arbitrary child execution, denies user-home reads except the identified
binary and disposable runtime, and denies filesystem writes outside that
runtime. No network listener or MCP process is added by the witness adapter;
oMLX's existing provider listener is separate.

Source grounding at the selected supplier tag:
[`spec_plan.rs`](https://github.com/openai/codex/blob/rust-v0.160.0/codex-rs/core/src/tools/spec_plan.rs#L1079)
omits shell tools when no environment is admitted or `ShellTool` is disabled;
its apply-patch and image-view registration also require an admitted
environment (around lines 1269–1291). The selected binary's generated
`ThreadStartParams` schema describes empty `environments` as disabling
environment access. This supports the selected recipe; it is not a substitute
for the actual behavioral observations above or future pin-specific checks.

Startup diagnosis found that this denied-home policy prevents app-server
initialization when its working-directory ancestors lie in the real home.
Paired no-model probes held the restrictions constant: the T3-tree home/cwd
failed, while disposable home/cwd outside the real home initialized. The
witness therefore uses a temporary runtime outside the real home and removes
it after stopping its owned supplier process group. The identified binary,
source and maintained evidence stay in the T3 tree/repository. This does not
establish durable conversation storage for the future desktop host.

The owner authorized the separate long-lived-server route; oMLX did not occupy
a build slot. Before inference the kernel available-memory estimate exceeded
50 GiB. A two-second monitor terminated the supplier on insufficient headroom
or unavailable memory telemetry. The tool unloaded its selected model on exit;
the subsequent status check reported zero loaded models and zero model-memory
bytes, and the owned server process group was stopped. No Cargo build, memory
pricing or timing benchmark occurred.

## Reproduction and remaining evidence

Run the focused offline check with the project's declared Python dependencies:

```sh
python3 tools/agent_host_witness_test.py
```

The opt-in live tool needs the identified binary, the owner-provisioned keychain
item, the existing configured server, sufficient memory and a fresh fixture
directory. Its `--help` lists explicit inputs. It validates the binary and
catalog digests before launching and permits only the authorized served model.
Credentials never belong in its arguments or output. Start only one model,
unload when idle, and stop immediately for an owner-announced exclusive window.
CPU-heavy build/test work still uses the host's slot wrappers.

Independent scrutiny of the tool and its stated evidence boundary belongs in
the PR. Remaining: controlled malformed/interruption observations beyond the
offline checks;
the Rust host/dispatcher, ingress-admitted completion and webview session
bridge; real human proposal review and stale/session/interrupt behavior; a
minimal functional panel; and the larger operation vocabulary needed for the
full reference-system journey. No host implementation, acceptance/audit
durability, engineering acceptance, code compliance or release is inferred.
