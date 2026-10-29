# Windows actual headless artifact/startup diagnostic — RT-V7 Later

Maturity: implemented experimental diagnostic; hosted compilation/native evidence
and independent review pending. Source basis is integrated main
`8e9392f10d22d217e34ffc5c28407024fed269d0`. This adds one finite investigation
after `windows-arena-probe.md`'s production startup audit. macOS-first is unchanged.

The maintained implementation and detailed protocol are in
`validation/platform/windows_runner_startup_probe/README.md`. The supplemental
`.github/workflows/piping-windows-runner-startup-probe.yml` builds the actual
unchanged `openpipestress-runner` in native x64 Windows, Rust 1.97.1 MSVC, default
features, `--locked --release`, into a fresh hosted target. The runner lock
SHA256 is `14397ed0733124d4dd8ac34ea14ec15b0274b30a0a0c2a65f2955e57b9e35e2c`.
The independent controller compiles directly with rustc, without another Cargo
manifest, lock or registry package. Existing source, methods, guards, fixtures,
reference values and workflows are untouched.

The runner's `main` parses arguments then calls `read_input` on the named path
using `std::fs::read_to_string`; only after its complete read does it parse JSON
and execute the request. A local pipe server verifies the connected runner PID
while withholding every byte and EOF. The exact marker
`INPUT_CONNECTED_PAYLOAD_WITHHELD` establishes input-path open only. It is not
ReadFile quiescence, settled startup, worker or allocator initialization, or
time-based readiness. Pinned official std source context is streamed by the
hosted script; token checks establish source availability only. Native success
must independently establish complete input/EOF behavior on this artifact. A
source/API incompatibility fails with no wrapper or runner modification.

The actual PE parser enforces bounded structural validity, PE32+ and AMD64,
then reports regular/delay imports, image sizes and main-thread stack header
facts. It does not enforce an import allowlist or a resource budget. Hashes and
actual process image paths bind observations to the executable. The gate samples
private usage, WS/peak WS, VirtualQueryEx private/image/mapped commit/reserve,
thread count and module basenames separately. These snapshots are observations
with timing gaps and module races, not maxima or exact allocator attribution.
No sampled counter isolates Native/System bypass. PE stack values do not bound
other stacks. The first fresh and repeated launches are not a cold-cache proof.

The outer observer owns creation-time Job custody independently of the case
and runner. No Job handle inherits. Exact standard-stream handle lists, bounded
OVERLAPPED input IO and bounded capture threads prevent a stalled case from
blocking the independent timeout/drain observer. At most one runner, two outer
Job processes, inputs <=64 KiB and output <=1 MiB exist. The conservative 512 MiB
Job cap is diagnostic safety only; a hit invalidates the observation. Handles
must signal before exit codes are read, including code 259. Cleanup requires
observed active zero, not a kill request or EOF. The deliberately blocked actual
runner control proves external descendant cleanup separately from successful
runner witnesses. Positive cases check the actual output/status/exit contracts
for the small invented supported sparse solve and request-metadata
validation. Completed-job identity, clean request/result validations and runner diagnostics,
nonblocking mechanics diagnostics (preserving normal warning rows), and
nonempty rows/references are required; ten mutations of actual successful
output must be refused by the same predicate; malformed JSON and missing solve payload check typed refusals.

All new validation source/fixtures ship in the existing public export. The JSON
uses the actual Rust runner request/provenance shape and the actual
`invented_non_engineering_example` label for invented diagnostic content. It
derives a small one-free-UX-coordinate axial diagnostic from the CLI test's
precision-connected model, retaining one axial force and restraining the other
five tip coordinates. The maintained model is unchanged; this is no numerical
reference. The 1 MiB output cap remains fixed. Project attributes preserve all
three shipped fixture byte streams for P7 hashes under Windows autocrlf. Additive DEL-04-07 pointers include this probe. The authoritative P7 generator
adds its three invented fixtures to `tools/release/provenance_scope.json`,
preserving existing entries. All runtime scans are re-derived: the three new
standalone `.rs` files enter the project scan despite having no Cargo manifest.
The full source-read graph has 41 normal release manifests and 1,362 inputs;
registered caller-tree digests and reader/anchor identities remain unchanged.
No schema change is introduced. `schemas/headless_runner.schema.yaml`'s
`HeadlessRunnerRequest` excludes `tbd_decisions` under `additionalProperties:false`,
whereas Rust `RunnerRequest` requires it and maintained CLI examples include it.
These CLI fixtures follow that source contract; full schema conformance is not
claimed and the independent schema inconsistency is not repaired here. Final export evidence must run `--execute` on
the committed raw integrated candidate containing all new files and generated
P7 entries. Before starting export, the hosted step verifies all fixture hashes
against committed P7 entries. It stages the full public tree and builds the
unchanged empty-dependencies native protected-content linter. The exporter's
current fixed-size lint argv batches exceed Windows command capacity for this
tree; final export evidence awaits the separately owned batching repair. No
batching change or reduced scan is part of this probe.
No local Cargo-backed export, private-term suppression or widened allowance is
used. P7 scanner checks and the owner's DEC-058 release scan remain separate.

DEL-04-07 ScopeOfWork, needs and governing memory policy retain their meaning.
DEL-07-11 shell remains unproven; DEL-10-04 gets supplemental diagnostic evidence
only, without discharge of required harness or release duties. DEL-04-08 Windows
retained registration remains Stale. No governing wording is adopted. The
parent must preserve previous fixture/arena/numeric/temperature pointers while
adding this diagnostic, regenerate shared outputs on integrated source, and run
both routing selfchecks and selected Root checks with no Cargo locally.

This is a headless CLI artifact, not the desktop PE or same-app solver worker.
RT-E1 constructor reader/writer threads belong to the parent general interactive
path and are not observed here. Startup/nonarena/native/CRT/TLS/helper/mapping,
kernel IO and future stack-touch costs remain unknown; this establishes no
R64, Nphysical/Ncommit/native/stack bound, production global allocator or retained
qualification, and no numerical meaning. Any future memory finding relevant to
retained registration goes through the parent to MacAgent0 before action. Next
minimal evidence is native execution and independent review of this artifact's
source/EOF/import witnesses, then a separately steered actual worker boundary
and enforced nonarena accounting.

Independent review uses the brief's threat model: stale/wrong artifact/platform;
inferred imports/stacks; unobserved phases or fake readiness; conflated counters
or missed Native/System costs; changed methods/data; leaked handles, EOF, exit259
or cancellation errors; timeout descendant escape; path/private-data/config or
authority overreach. At most three tooling rounds, owned by the parent. Record
out-of-model findings and route a new kind inside the model to Agent0. No review
is started by this Type2 contribution. Official PE, std and Win32 references are
linked in the implementation README; native evidence is not inferred from this
design or from successful light source checks.

## Bounded solve-response constraint

Native run `40854395895` at `d945106fa377cc9174ccf501e05ddc3c5b1e2a18`
built the unchanged runner and controller, passed all nine PE tests, reached
the PID-bound input-open gate, and observed Job active zero after the 1 MiB
capture guard rejected output. The solve did not produce a captured successful
contract witness. Later metadata/error/blocked-timeout cases were not reached.
No successful solve, metadata or error execution is inferred from that run.

The output constraint is in the existing CLI exposure, not simply numerical
payload size. `openpipestress-runner.rs` lines 166–173 applies
`redaction_binding::control_local_private` then pretty-serializes the entire
controlled object. `redaction_binding.rs` lines 997–1175 walks every scalar and
publishes its decision and, for private/unknown fields, a finding with its path.
Mechanics/runner fields require the existing explicit local-private intent;
the probe command currently omits that flag. Adding it would retain warnings
and payload, not remove the audit records or solve the size constraint. No
exposure rule, flag behavior, projection or serializer is changed here.

`validation.rs` lines 176–225 requires at least two nodes, one pipe and an
explicit load on the ordinary path. Component, station and signed-support rows
are published even when their value is zero; fewer loaded coordinates does not
remove their fields. As a source-only size illustration, the maintained
`fixtures/results/preview_physics_connected_sparse.json` two-node/one-pipe/
one-support carrier has 89 result rows and 1,179 mechanics scalar leaves. Applying
the current scalar-walk record shapes to those paths produces 1,070,408 bytes
of pretty-serialized decisions/findings alone without intent, or 1,089,272 with
intent, excluding payload, runner metadata and result references. This is a
representation calculation on an existing carrier, not a new solve or proof
that every possible mechanics response has that size. Current global end rows
also add fields. Both actual tested inputs exceeded the unchanged capture cap;
no smaller supported positive response fitting it has been established. No
further input/native attempt is selected from this investigation.

A second source correction preserves the product's unconditional
`RULE_CHECK_INPUTS_MISSING` warning (`product_physics/src/lib.rs` lines 3381–3387):
mechanics diagnostics must be present and nonblocking, rather than empty.
Request/result validations and runner diagnostics still require their clean
shapes, and the ten negative mutations still use the same predicate. Their
native execution remains pending because a successful captured solve is absent.

The actual inspected PE recorded a 1,048,576-byte main stack reserve and
4,096-byte stack commit; at the input gate one sample recorded 778,240 private
usage bytes, 4,341,760 working-set bytes and four threads. These are diagnostic
facts for the headless artifact only. They do not bound Native/System bypass,
other stacks or future page touch, qualify R64, or activate retained registration.
A different artifact, projection, cap or method is not adopted to obtain green
solve evidence. Metadata/error execution and final committed export remain
separate outstanding evidence; the latter waits for the verified exporter fix.
