# PARTIAL Windows headless artifact/startup diagnostics — RT-V7 Later

Maturity: experimental finite diagnostic. The four partial native statements
executed on `bba14a5cef7659457771281eb5f376673ccffef5`; the source repair from
round-2 review and its 39 mutation controls await exact-head native evidence. Integrated production basis is main
`9ef6c4c46938b6ee7fe7dddf785cbb707bbf7577`. The runner entry/manifest/lock are
unchanged by this unit; inherited solver corrections/guards remain intact.
macOS-first release, governing memory policy and retained qualification do not
change. Implementation: `validation/platform/windows_runner_startup_probe/`.

## Selected statements and exclusions

The supplemental workflow is explicitly labelled **PARTIAL metadata/errors/
cleanup — solve BLOCKED**. Exactly four serial cases execute:

- Valid invented request metadata: matching request/operation refs, clean actual
  `validate-input` metadata validation, exit 0, no job or mechanics result.
- Exactly `{`: typed `HEADLESS_RUNNER_CLI_INPUT_JSON_INVALID`, exit 2, correct
  runner diagnostic source and affected input, no request validation or mechanics.
- Invented metadata without `solve`: typed `HEADLESS_RUNNER_SOLVE_PAYLOAD_MISSING`,
  exit 1, correct affected request and clean pre-refusal metadata validation.
- Actual CLI blocked at input open with no payload/EOF, followed by independent
  timeout termination and observed zero-survivor cleanup.

The first/repeated successful solve witnesses remain BLOCKED/UNVERIFIED at the
unchanged 1 MiB capture cap. Their ten proposed live-output mutations have never
executed. They are listed separately by `--blocked-inventory`, not included in
passes or replaced with an error response. Metadata success does not establish
model checking or mechanics. No Windows/desktop/solver/Nbound qualification follows.

The controller accepts only the compiled bytes of the two selected invented
fixtures or the exact malformed byte. Case/verb mismatch and substitute inputs
fail before runner creation, preventing automatic dispatch of a full solve.
`fixtures/solve.json` remains the failed evidence basis and is not an active
case. Input <=64 KiB and combined captured stdout/stderr <=1 MiB remain enforced.

The selected output shapes have no mechanics rows and at most 40 scalar leaves.
With bounded ASCII paths (<128 bytes), fixed messages (<512 bytes) and a
conservative 2 KiB allowance per decision/finding pair plus payload/framing,
these outputs fit below 128 KiB. This source sizing applies only to these fixed
metadata/error paths; the runtime guard and actual native checks remain required.

## Explicit test intent and operation-specific verification

The three output-bearing cases explicitly add the documented
`--explicit-local-private-intent` argument for genuine invented P7-labelled
records and their fixed invented diagnostic only. The argv changes; existing
flag semantics, guard behavior, warnings and audit records are unchanged. This
is no user-data, protected-write, `--output` path, user-release or public-export
authority. Payload-carried intent is not used. The blocked-input case uses no
flag and sends no bytes. No solve is retried under this argument.

Predicates require exact CLI identity, operation, observed exit and privacy
outcome; clean metadata validation; or the exact refusal code/class/severity,
source, affected reference and request-validation state. Job, mechanics,
result-validation and suite fields must be present and null. Input refs are
checked against the bound fixture because the CLI does not echo them; no echo
or model validation is inferred. Audit decision paths cover every payload
scalar, source-defined warning paths/reasons and findings are required even if supplied counts remain consistent, and expected local-
private warnings remain. Nonblocking warnings do not themselves fail.

Thirteen mutations of each actual captured baseline exercise this same predicate:
empty/malformed object, absent payload, wrong operation/exit/runner status/
validation/refs, wrong or missing typed diagnostics, missing audit, stripped
warnings and unexpected privacy blocking. Wrong input refs mutate the paired
bound fixture context; other mutations alter captured output or observed exit.
The expected count is 39 diagnostic mutations across three baselines. They do
not count as the original ten live solve mutations. Case count, inventory,
missing-marker, wrong-candidate and empty/unknown-selection controls are explicit.

## Artifact, startup gate and independent custody

Hosted native x64 `windows-2025`, Rust 1.97.1 MSVC, builds the unchanged existing
`core/runner/headless` stable CLI with default features, `--locked --release`
and a fresh target. The existing runner lock SHA256 remains
`14397ed0733124d4dd8ac34ea14ec15b0274b30a0a0c2a65f2955e57b9e35e2c`.
The controller/nine PE tests use rustc directly; no new Cargo graph or registry
input is introduced. Exact candidate, native architecture, lock, executable hash
and launched process image are checked. Build/link/case diagnostics stream live.

CLI `main` parses arguments then `read_input` uses `std::fs::read_to_string`;
execution/JSON parsing follows complete input/EOF. A noninheritable local pipe
server rejects remote clients, restricts its DACL and verifies the actual runner
PID while withholding every byte/EOF. `INPUT_CONNECTED_PAYLOAD_WITHHELD` proves
source-bound input open only, not ReadFile quiescence or settled startup. Pinned
std source context is inspected; native statements establish actual IO behavior.

Creation-time Job assignment, explicit inherited standard handles and independent
outer observer custody cover case plus one runner. No Job handle inherits. The
outer Job holds at most two processes and uses a 512 MiB diagnostic safety cap,
not an admission/grant state machine. IO waits, cancellation, capture, case exit
and Job drain all have finite deadlines. Process HANDLE signalling precedes
exit-code inspection, including 259. Custody ends only after observed active
zero; EOF and kill requests do not suffice. The actual blocked-descendant case
separately exercises this backstop. Detailed phase/deadline/handle limits are in
the maintained README, alongside official Win32/std references.

PE parsing reports actual import/delay-import/main-stack/image header facts,
with bounded structural checks and nine positive/negative parser tests. Live
input-gate samples separately expose private usage, working set/peak, VA region
types, thread count and sanitized module basenames. These sequential samples
have timing gaps/races and cannot price Native/System bypass, other stacks,
commit/physical liability or future page touch. No desktop PE or RT-E1 parent
thread is observed. R64 and Nphysical/Ncommit/native/stack bounds remain unqualified.

## Remaining item: ControlledExport representation cost (DEL-10-05)

This is a product-owner-visible remaining item, not a selected fix. Actual
`1f2a25058543bfcf6bd6c4796e493820050775ad` run `40853089288` and
`d945106fa377cc9174ccf501e05ddc3c5b1e2a18` run `40854395895` both built the
unchanged runner/controller, passed nine PE tests, reached the PID-bound gate,
and rejected the supported/tested solve invocation's ControlledExport output at
1 MiB. Independent cleanup observed zero survivors. Neither captured a successful
solve contract; later metadata/error/blocked-timeout cases were not reached.

`core/runner/headless/src/bin/openpipestress-runner.rs` main applies
`redaction_binding::control_local_private` then pretty-serializes all of it.
`redaction_binding.rs::walk` emits a decision per scalar and a finding for each
private/unknown leaf, including full field paths. The old probe command omitted
local-private intent, so a successful captured private payload would also have
been blocked. Supported intent retains warnings/payload; it removes no audit.
The partial cases correct that invocation detail only for their approved data.

Ordinary required-collection validation needs two nodes, one pipe and an explicit
load. Zero-valued node/component/station/support rows remain published. A source-
only illustration on maintained `fixtures/results/preview_physics_connected_sparse.json`
(two nodes, one pipe, one support; 89 rows/1,179 mechanics leaves) yields
**1,070,408 bytes of audit records alone before payload** without intent, or
1,089,272 with intent, under the current record shapes. This is not a new solve,
universal minimum, Windows bound or claim that every possible response has that
size. It explains why reducing loaded coordinates did not establish a fitting
response; no further solve input/native attempt is selected.

Related source-qualified input: [solver PR #1742](https://github.com/sgttomas/chirality/pull/1742),
**Open / RT-U4 finding F-1**, read at PR head
`0be06842da58968b5be55b3535dd5eb7ebac3f0a`. It reports a separate release-CLI
measurement on main `6ff1e9e88d` with explicit intent, per-output-field audit
expansion and peak RSS, and leaves the large-model RT-U4 timing unmeasured. That
Mac observation is not this Windows evidence, nor a Windows bound. The item is
relevant to DEL-10-05's headless CLI exposure. The desktop worker's different
compact producer transport is not qualified by this diagnostic. This unit does
not edit #1742, CLI methods, projections, serializer, cap or engineering data.

The actual d945 PE recorded a 1,048,576-byte main-stack reserve and 4,096-byte
stack commit. One input-gate sample recorded private usage 778,240 B, working
set 4,341,760 B and four threads. They remain candidate-bound diagnostic facts,
not other-stack/Native/System allowances or retained registration evidence.

## Public boundary, reconciliation and remaining evidence

All validation source/fixtures ship in the existing public export. Invented
fixtures honestly retain `invented_non_engineering_example`/`pending`; the
maintained generator derives exact P7 hashes, and a narrow project attribute
rule preserves their committed bytes under autocrlf and survives public cutdown.
The source-compatible CLI request shape includes required `tbd_decisions`; its
published HeadlessRunnerRequest definition excludes that field. No schema is
changed and full conformance to that inconsistent definition is not claimed.

Full committed-candidate export is still required before fan-in. Its job is
explicitly deferred while the separate exporter batching fix is unverified;
after the verified repair merges, merge main, regenerate source-bound shared
outputs, enable it and require a successful full raw-HEAD boundary/lint run.
Skipped/deferred export is not passing evidence or publication authority.
No fixed-batch or allowance edit is part of this diagnostic.

DEL-04-07 ScopeOfWork and needs keep their meaning. Additive pointers preserve
all fixture/arena/numerical/temperature entries; all scan.files and applicable
caller/reader/anchor digests are re-derived on integrated source. Both routing
selfchecks and selected Root checks run with no local Cargo. DEL-07-11 shell
remains unproven; DEL-10-04 receives supplemental diagnostic evidence only;
DEL-04-08 Windows registration remains Stale. No governing commitment changes.

Round 1 independent Sol/medium/default review at `1f2a250585` supported custody/
resource qualifications and found a solve-output false-green predicate plus a
stale official link. Repairs are preserved in Git; their ten live solve mutations
remain unexecuted. The material partial-coverage and operation-specific validator
changes received round-2 independent review under the existing false-green,
source/platform identity, fake quiescence, custody/EOF/259, timeout/leak/privacy
threat model (maximum three rounds). Native run `40862607292` at `bba14a5cef7659457771281eb5f376673ccffef5`
passed nine PE tests, all four partial cases, the substitute-input guard,
independent active-zero cleanup and the original 36 diagnostic mutations.
Captured metadata/malformed/missing-payload outputs were 11,198 / 20,358 /
19,535 bytes. This establishes only those named statements on that candidate.

Round 2 source review found a P2 false-green audit check: changing a required
warning decision to include, removing its finding and decrementing counts could
pass. The direct repair now requires fixed-case source-defined warning paths,
reasons, classifications and finding text, independently of supplied counts.
A thirteenth mutation per captured baseline consistently suppresses the
`$.result_validation` warning while updating classifications/findings/counts;
it must fail the same predicate. Expected current total: 39. That native
regression, final export and exact-head required checks remain pending; no
additional review round is selected for this small repair.
