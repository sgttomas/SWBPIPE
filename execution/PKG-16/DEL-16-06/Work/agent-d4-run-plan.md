# D4: one joined local-model W9-A witness

Plan revision D4-4, 2026-10-31. This is the run specification for Agent 0 to
present with the single-read credential request. It supersedes the earlier
thread draft's 64k assumption, three-attempt estimate and anchor/corner result
scope. No resource window is booked. Preparing or merging this plan authorizes
no credential read, native launch, model/server start or provider request.

## Selected candidate

The code selection is `5bc572386e45010a6a88abda03f38031e86b363c` in
[PR #46](https://github.com/sgttomas/SWBPIPE/pull/46). This is the reviewed
`da24221b90a125931020bad9c1a0671a443b77d5` plus the parent repair of the fourth
review's cleanup exception finding. The repair changes only the Python
controller/tests. The PR records exact-head checks, fourth-review disposition
and the integration commit; the later plan commit does not select newer engine,
frontend or catalog inputs from main. Before running, confirm this selection
is integrated and compare the selected files, not an arbitrary checkout's HEAD.
The separate monitor follow-up adds the maintained producer and a narrow
consumer health check; its exact reviewed head is identified in its PR. For a
live attempt select that follow-up's Python controller and monitor together,
with the same native/frontend/wasm inputs below. The earlier controller hash
remains the pre-monitor identity, not the live-controller identity. No native
rebuild follows from the Python-only addition.

| Input | Selected identity (SHA-256 unless stated otherwise) |
| --- | --- |
| Controller, `tools/agent_journey_fixture.py` | `570ed77fd1c5c05e0c54010d3357ec4e706babb8f2f672bc4beb7d414f7def20` at the code selection |
| Imported lifecycle controller, `tools/agent_live_session_fixture.py` | `c894f1547f8b0f522e994421fbd2863de2fbcb61a5d38346a6859119fb4ed1f4` |
| Webview driver, `apps/desktop/native-smoke/agent-journey.js` | `586f0bcb4b6144a381b975e21107eef43b43cb4ef85e696ee74ff8ff79ce4462` |
| Native-smoke ARM executable, before sealing a private bundle copy | `23be3c5e8ae0e3f3e6fbf3f434bc66752d8271002d020d040175b274af2e1cc3`; retained build output from `cb3897f1c2139fa5dd5c1161fea4187f77944d38` |
| Frontend assets | `bd1c8eff3`; retained `dist/`, 24 files, set digest `8585ec1fa86116ceb31103266fa8bbdc7c176601f7aae1cd4b8a77239ad9ce5c` |
| Operation wasm | `dist/wasm-engine/open_pipe_stress_operation_applier_bg.wasm`, `057b737502f2fcc74c8eba11979dab998c97625d8c29f62b53fb5b6b1ab9f82b`; source-stamp hash `sha256:8c41d387f1522f4b3666e902f3d5b2997290b5dced0979f4c230399657e8360d` |
| Self-weight wasm included in those assets | `b54a4e05ce41109eaa207bae30e6dd9abc340f15db16d51588059a34a4f152d6` |
| Catalog and generated resource | `swbpipe-catalog/4`, both `8e9a4b4e3eec027d092033ea5d95f1648030520e797c5a660ab387531f23be17`; seven tools, no catalog change |
| W9-A proposal fixture | `fixtures/model_operations/agent_w9a_journey.json`, `f07967eb0d33ed6b4e4c9edd843cd6bc8647a2c2c77543b7c0a2bec14a42df9c`, PF route, 11 operations |
| Numerical reference | `validation/references/w9_benchmarks/references.json`, `c7b0a376b596d7353fb0e2d5df602fa3446f88859b89d3d62477a2bbec73ff68` |

The frontend set digest is SHA-256 of UTF-8 compact JSON, sorted keys, no
trailing newline: an array sorted by relative POSIX path, each entry containing
`path`, `bytes`, `sha256` for every regular file under `dist/`. This binds the
selected generated assets for this named witness, not a permanent test input.
The native executable embeds frontend assets. The build/source association is
the historical build record, not a new reproducible-build observation.
The minimal selected executable and asset set is retained for the named D4
consumer under this lane's `scratch/rt-e1/d4-candidate/`; build caches are not a
run dependency. Agent 0 has the absolute host locator.

The historical stub14 used an ad-hoc-sealed private bundle copy; its executable
hash was `c23a852d85fc4e07e50b7b982e75e72c430def174958fceba66dfd60c7d519a7`.
Sealing can change executable bytes. A future authorized preparation must record
the actual deployed hash after sealing; do not label the unsealed hash as the
deployed hash or re-sign the supplier. The original retained bundle is untouched.

## Provider, checkpoint and configuration selection

- Stock supplier: Codex `0.160.0`, ARM executable
  `112fae7a5a1223e673c8a1791d32338f37df8b527ff1159bb8adac6c4dbf1b4b`.
- Server selection: existing oMLX `0.6.4`, application build `2529`, Responses
  API at `http://127.0.0.1:8000/v1`. Actual server version must match before use.
- Checkpoint: `~/Models/MLX/mlx-community/Qwen3.6-35B-A3B-8bit`, beneath model
  root `~/Models/MLX`; served id `Qwen3.6-35B-A3B-8bit`.
- Configuration selection: `~/.omlx/settings.json`, saved nonsecret
  `sampling.max_context_window = 32768`; authentication and verification enabled.
  These are owner-home-relative spellings of the exact absolute locators App
  supplied through Agent 0, not a discovery/search instruction or a second model.
- Runtime identity: the 19 files enumerated in `agent-witness-pin.json`, total
  37,748,365,700 bytes; expected canonical manifest digest
  `29e5650499c3c29e8a7fef2d136c9b50b140dd251d8f893b291e66db47b26b58`.
  `config.json` is pinned to
  `5da49c3a84c5a4a751b720b62b9f8703bdb8a0796b5fc55df3278f8b9d6ce296`.
  Exclude README, `.gitattributes`, cache metadata and documentation from this
  runtime inventory, as the pin does.

App's supplied read-only observation established directory presence, no directory
symlink, identical resolved path, count/total bytes and the `config.json` hash.
This lane then streamed all 19 pinned runtime files under the host slot wrapper
on 2026-10-31: every size/hash matched, and the canonical manifest matched
`29e5650499c3c29e8a7fef2d136c9b50b140dd251d8f893b291e66db47b26b58`.
No checkpoint was loaded and no settings or credential was read. App's last
server connection was refused. Neither declared model capacity nor the
saved 32,768 setting proves the context actually served. Do not read the full
settings file into evidence: it contains authentication configuration. App
reports only the selected nonsecret fields through Agent 0.

App later supplied a nonsecret source-only projection: saved `max_tokens=32768`,
`mcp.config_path=null`, `mcp.expose_tools=true`, no `base_path`, and absent
`model_settings.json`/`model_profiles.json`. Conditional on that launch/base,
ModelSettings defaults add no persisted model/profile template overrides;
template/forced kwargs and thinking toggles are None,
`thinking_budget_enabled=false`, `trust_remote_code=false`. These narrow the
offline reconstruction but do not establish the running engine/custom loader,
launch options, MCP manager or actual output allowance. In particular, 32,768
requested output tokens are not 32,768 available completion tokens after a
nonempty prompt. Keep the planned context/output reserves.

Before booking, remaining ordinary offline prerequisites are selected
asset/probe hash checks and resolution of the supplier/provider serialization
limits below. Repeat the runtime comparison if any file changes before the run.
No download, substitute, 64k assumption or server setting change is permitted
on a mismatch. Standalone native denial preflight and
post-seal checks remain run-preparation steps requiring native authorization;
they have not run under this resume.

## Two turns within 32,768 tokens

Use the controller's exact `TURNS`, low supplier reasoning effort, and ordinary
provider `contextWindow=32768`. The declarations are 17,530 compact JSON bytes,
not tokens. The installed oMLX conversion additionally fills missing schema
descriptions: its resulting template-tool JSON is 20,691 compact bytes. Counting
the catalog alone would omit both that transformation and Qwen's tool preamble.

Offline sizing used App's existing Python runner with `-B -S`,
`PYTHONDONTWRITEBYTECODE=1 HF_HUB_OFFLINE=1 TRANSFORMERS_OFFLINE=1`, and its
`framework-mlx-base` site-packages: tokenizers 0.22.2, jinja2 3.1.6, pinned
tokenizer vocab 248077. No AutoTokenizer, engine, server, settings or model was
imported/started. The three tokenizer/template hashes match the model pin.

The one-time `scratch/rt-e1/d4-sizing.py` has SHA-256
`cbbe8e9367628171e32694332a5c607d7b8be2398754741892e43a59b0f6e5c0`.
Its result `d4-sizing.json` has SHA-256
`a47c7e708dd32c2742dad00afc57e8d629d35e1c651f5b8e8edc3e3d3cdacee1`.
These are bounded run-planning evidence for this consumer, not maintained test
dependencies. The result binds each input file, reconstructed history and
rendered prompt by SHA-256. Histories use the selected `TURNS`, exact Rust
`INSTRUCTIONS` string, resolved seven-tool catalog and stub14's six real tool
outcomes; the selected Script reconstructs calls and completion text. Tool
outputs/arguments use compact sorted-key JSON and invented fixed call ids.

The sizing extracts only pure functions from installed oMLX source: Responses
input/tool conversion, tool-template conversion including description defaults,
and the installed Transformers Jinja compiler. It does not import oMLX. The
renderer therefore uses the actual immutable sandbox, trim/lstrip settings,
loop-controls extension and non-HTML-escaping `tojson` implementation, rather
than Jinja's different default filter. Render text-only, nonpartial messages,
consolidated leading system, `add_generation_prompt=True`, no BOS,
EOS `<|im_end|>`, PAD `<|endoftext|>`. Count with the pinned Tokenizer;
`add_special_tokens=False` and True produced identical token ids. Both
`enable_thinking` and `preserve_thinking` were measured as False and True;
the ranges include all four combinations. Empty stub reasoning is not live
reasoning evidence.

| Reconstructed input snapshot | Measured prompt tokens |
| --- | ---: |
| Turn 1, initial request | 6,458–6,460 |
| Turn 1, after summary | 7,027–7,029 |
| Turn 1, after proposal outcome, before completion | 10,156–10,158 |
| Turn 2, initial request with retained turn 1 | 10,248–10,262 |
| Turn 2, after summary | 10,823–10,837 |
| Turn 2, after observe | 11,825–11,839 |
| Turn 2, after solve | 12,666–12,680 |
| Turn 2, after filtered results, before report | 13,881–13,895 |

These counts establish the specified reconstructed prompts, not the actual
stock-supplier/provider wire envelope. The stub kept request metadata and tool
outcomes, not raw request bodies. Extra supplier developer/reminder instructions,
wire-schema rewriting, per-model template overrides, custom tokenizer routing,
server-added tools and actual reasoning/history serialization remain unresolved.
Source tracing confirms oMLX's Responses conversion → system preparation →
tool conversion → engine template → encode path, but does not establish which
runtime overrides the retained server will use. No settings file was read to
fill that gap. Provider equivalence is conditional on the stated selection.
[`agent-d4-capture-proposal.md`](agent-d4-capture-proposal.md) specifies the
smallest later supplier-only scripted capture to close the missing envelope;
it needs no desktop App or real provider/model and is not authorized by this plan.
Its CAP-1 adapter, `tools/agent_supplier_capture.py`, is maintained with mock-only
tests and source-bound synthetic outcomes; its reviewed revision is recorded in
the capture PR. This completes offline adapter preparation, not the missing
observation. Actual request bytes and a provider-equivalent recount are still
absent. The one-attempt command/limits/cleanup remain subject to Agent 0's later
presentation to the owner, separately from credential/live-witness authority.

| Token budget | Turn 1 | Turn 2 peak, including retained turn 1 |
| --- | ---: | ---: |
| Largest measured reconstructed input | 10,158 | 13,895 |
| Unresolved supplier/provider envelope reserve | 2,048 | 2,048 |
| Live arguments/tool-result variation reserve | 2,048 | 4,096 |
| Live reasoning reserve (all retained/current reasoning) | 4,096 | 4,096 |
| Remaining completion/report reserve | 512 | 2,048 |
| Planned total | 18,862 | 26,183 |
| Margin to 32,768 | 13,906 | 6,585 |

Reserves are planning allowances, not measured live outputs, proved maxima or
per-phase controller caps. Before inference, resolve the missing serialization
identity from the pinned supplier/provider source or an already-authorized
request capture, then recount if it differs. This resume authorizes no native
capture to close that prerequisite. If an unknown envelope or future output
cannot be bounded within its reserve, report that dependency to Agent 0 rather
than claiming a 32k fit. At the authorized live check App must establish actual
served context and compatible output allowance. Record usage if exposed;
context refusal, compaction, unexpected extra work or exhaustion is a failed
attempt, not permission to enlarge the window or retry. The 2 MiB relay request
cap and catalog byte/row limits apply independently of tokens.

Turn 1 asks for the PF W9-A frame as one review batch: E 200 GPa, G 76.923 GPa,
alpha 1.2e-5/degC, nodes A(0,0,0), C(4,0,0), D(4,3,0) m, two pipes OD 0.2 m,
wall 0.01 m, anchors A/D and +100 degC on both pipes. Expected sequence is
summary, proposal, completion. Apply occurs only through Batch review's real
buttons. The driver records its click as a qualification stand-in.

Turn 2 asks for confirmation of Apply, solve and six labeled `node:c` motion
records in mm/rad to at least six significant digits. Expected sequence is
summary, observe-applied, solve, filtered read and report. Filter to the six
global nodal displacement/rotation kinds and `entityRefs=["node:c"]`, one page
of at most 50 rows; do not request an unfiltered 64 KiB page.

## One attempt and resource admission

Agent 0 arranges the joined window with App and the active lanes after plan and
review readiness and owner authorization. Request about 60 minutes including
load, two turns and unload; authorize exactly one attempt. No window is reserved
by this document. Before inference:

1. Complete offline prerequisites, confirm selected input hashes and absence of
   the retained-run latch, prepare a fresh private bundle/HOME, and complete the
   standalone denial preflight. Build probes only through maintained slot tools.
2. App verifies actual server version, selected checkpoint, served id, served
   context/output allowance and authenticated readiness. A saved setting or
   unauthenticated endpoint response alone is insufficient. No settings change.
3. Enter `t3_exclusive.sh` with memguard running, no concurrent builds or other
   model attempt, and at least 50 GiB system free memory. Use the controller's
   `free_bytes()` definition (vm_stat free + inactive + speculative pages), not
   T3 RSS, which omits oMLX. Repeat admission after model load and before inference.
4. Run the reviewed controller in live mode with the selected endpoint/model,
   `--context-window 32768 --min-free-gib 50 --turn-timeout 600 --deadline 1800
   --release-grace 0 --key-stdin --cancel-file <private-run>/cancel`. Select fresh
   output and the verified process/boundary probes. Never place a key in an
   argument, environment, file, plan or report.

The zero release grace is an explicit live selection, rather than the default
120 seconds: on resource cancellation, an anchored supplier must not continue
inference during a grace period. Unknown custody still retains the app and
latches the host; this does not authorize killing it by a remembered PID.

**Maintained monitor:** `tools/agent_journey_memory_monitor.py`, with mock-only
tests in `tools/agent_journey_memory_monitor_test.py`. Its separate follow-up
requires safety-tool review and exact-head checks before use. The producer
samples the same free+inactive+speculative metric via a one-second-bounded
`vm_stat` command, publishes every two seconds, and writes the existing cancel
file on low/unavailable/malformed telemetry or a caught producer failure.
It never takes a PID/process handle and never signals a process.

Use a fresh private mode-0700 directory for the producer's `--directory`:
fixed files are `heartbeat.json`, `cancel` and `done`. The controller takes
`--memory-heartbeat <directory>/heartbeat.json --cancel-file <directory>/cancel`
in addition to the arguments above; use this common cancel path rather than a
different `<private-run>/cancel`. Live mode requires both flags and zero release
grace. Healthy admission must precede the one key read. The coordinator starts
the producer only inside the authorized run window and owns its process/exit
status; creating `done` is allowed only after app/supplier cleanup and App's
unload evidence. Keep the producer evidence and its exit status with the run.

The consumer checks the five-second monotonic health lease before launch,
during the existing safety loop, immediately before READY and before success.
Abrupt producer loss (including kill, crash, hang or failed cancel-file write)
therefore expires the lease independently of producer exception handling.
Missing/malformed/future/expired or low health raises the existing `Refusal`
and enters the existing owned cleanup path even if creating cancel fails.
The lease is not an OS real-time guarantee: a suspended consumer cannot act
until scheduled; the maximum observed response must be recorded in the later
authorized run. The native gate holds before admission; existing anchor/custody
rules are unchanged. No native or real telemetry test has run for the follow-up.
Producer and consumer must use the same Python interpreter's system-wide
monotonic clock. Prevent or record host sleep in the authorized window; Darwin's
clock pauses during sleep, within the excluded suspension of all observers.
Confirm the first healthy heartbeat before presenting the one-read credential
input, rather than spending it on an unready monitor directory.

Safety-tool threat model: accidental telemetry failure/malformed output,
producer exceptions/abrupt loss/stall, publication/cancel I/O failure, and stale
or absent health in the selected private directory. A malicious same-uid writer,
OS compromise and simultaneous suspension of all observers are outside this
bounded tool's assurance. Neither a heartbeat nor a mocked test proves live
headroom, live timing, model unload or universal termination.

Stop the one attempt for admission/preflight/auth/context mismatch, wrong or
multiple batch, refusal/interruption, turn/launch deadline, cancellation,
memory/telemetry failure, supplier observation error, unexplained extra work,
numerical/report disagreement or any unconfirmed cleanup. No automatic second
attempt, silent input repair or tuning of numerical expectations. If a retained
app/latch or resident process remains, report it immediately, keep its custody
and evidence, and require normal person-directed quit plus confirmed absence
before the latch is removed. Do not start another attempt.

## Expected results and evidence

Each expected value comes from `references.json`, case
`W9-A-LFRAME-THERMAL`, rows with `quantity=nodal_motion`, `entity=node:c` and the
named component. Translate reference metres to displayed millimetres by 1000.

| Component | Expected displayed value | Reference exact value before conversion |
| --- | ---: | --- |
| ux | 4.771112073942932 mm | `1935887796/405751901443` m |
| uy | -3.586865438767269 mm | `-1455377472/405751901443` m |
| uz | 0 mm | `0` m |
| rx | 0 rad | `0` rad |
| ry | 0 rad | `0` rad |
| rz | 0.0007867143613246695 rad | `319210848/405751901443` rad |

Use the reference's existing relative 1e-9 criterion and family zero scale for
the numerical window rows. The live report must name each component once, in
the matching unit, within the controller's existing relative 5e-4 comparison
to the window; a displayed zero is reported as zero. The prompt asks for six
significant digits; the judge permits rounding. Preserve both criteria.

Keep the bounded native-smoke report, submitted operation ids/kinds/targets,
seven tree entities, Apply receipt, result rows/model hash, labeled report,
run times, selected/deployed hashes, admission/monitor observations and final
cleanup/unload evidence. In live mode the controller does not capture provider
tool-call arguments/outcomes as the stub does: do not claim stub-only proof of
the exact agent-read rows or tool-call sequence. Compare the window's solve
hash with the applied model wherever the retained receipt exposes both;
otherwise report that binding as unestablished.

App independently reports unload from evidence: selected model absent from the
loaded-model set and model-memory accounting released, with timestamp and
server's final standing. Preserve the server as App directs; no unasked server
shutdown. Require app exit, supplier exit/absence, native cleanup outcome and
no resident app/supplier/helper. Rescue is a failed attempt even if it succeeds.
Missing unload or cleanup evidence prevents a passing witness. End monitoring
only after cleanup/unload are established; remove private bundle/HOME copies
after preserving the bounded nonsecret evidence.

## Smallest owner input and later milestone

After review/plan readiness Agent 0 asks the owner to authorize **one read of
the login-Keychain service `swbpipe-omlx-api-key` for this one joined D4 attempt**.
Reason: the selected existing oMLX configuration requires authenticated
requests. Pass that read directly through the reviewed controller stdin and
native-smoke-only take-once memory substitution
(`SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY=stdin`); the driver saves a no-key provider.
No Keychain write, repeated read, sign-in, secret logging or saved auth change
is requested. Gate the read on completed nonsecret preparation and run authority;
do not consume it during planning. This credential request does not itself
reserve a window or authorize native/model activity; Agent 0 coordinates those
explicitly with the owner and App.

W9-A/stub14 establishes plumbing and this live plan can add one pinned local
model's six-motion journey. Neither establishes criterion 17. Later work still
needs the full reference-system authoring/comparison basis and source-named key
results, the owner's attended Apply, and explicit person-entered k/SIF/modifier
steps wherever those results depend on them (LIM-001). The automatic click is
not human acceptance or authenticated approver identity. This plan supplies no
anchor-reaction or corner-moment coverage and does not advance acceptance,
issuance or release. DEL-16-06 ScopeOfWork and dependency conditions are unchanged.
