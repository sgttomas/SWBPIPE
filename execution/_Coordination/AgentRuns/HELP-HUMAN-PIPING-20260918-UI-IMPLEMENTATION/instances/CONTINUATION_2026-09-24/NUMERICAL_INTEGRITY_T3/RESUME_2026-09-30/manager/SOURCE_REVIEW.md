# Manager source review and execution record (checkpoint 0)

This record is manager integration evidence, not independent numerical review.
The manager read the governing instructions and brief from <COORD> at
3446bdf51d90b8f1812d161cc638d85c97bbef16; hashes are in DISPATCH.json.
The product source inspected is at d01ad98a754698631f927709d08284c272de85e8.
SOURCE_SHA256SUMS identifies the exact additional files read.

## Source checks completed

- FK/src/structural/retained/adaptive.rs:296 computes the four coupled scales
  using binary64 products/quotients. :1904 forms verification scales from
  retained rows; :2600-2620 forms publication scales from binary64 outcomes.
  :379-406 retains the special small-scale branch only for 0 < S_pub < 2^-988,
  while S_pub=0 and S_pub>=2^-988 use the base absolute bound. This confirms the
  audit's source junction. It establishes neither realized false publication
  nor a safe exclusion of that publication.
- FK/src/structural/retained/bound.rs:485,509,525 keeps at, bt and ct together
  in nl_pass. :683-687 retains c from u_pass across nl_pass and later forms
  block maxima; the returned ct is also retained then.
- FK/src/structural/retained/bound.rs:877-924 creates shifted_factor's work as
  a function local which is not returned. shift_schedule invokes nl_pass only
  after shifted_factor returns (:1056-1059); work is not live during nl_pass.
- FK/src/structural/retained/verify.rs:448-501 retains bounded coefficients
  through Uc construction. The branch-local members_w ends before uc_bounds.
  Therefore the assembly/build transients cannot simply be added as if all
  overlapped, and omission of Uc's own overlapping vectors remains distinct.
- H/src/k6/w1/counts.rs:411-559 is the legacy max-over-phase estimate; it
  independently defines fixed, shared/build, state/solve, verification/build,
  pass, report/decision and final publication terms. VR/src/scale.rs contains
  a separate estimate. The manager has not selected a shared interface or
  claimed a complete corrected E_max.

FK = projects/chirality-piping/core/solver/frame_kernel.
H = projects/chirality-piping/core/solver/performance_harness.
VR = projects/chirality-piping/validation/benchmarks/numerical_robustness.

## Actual execution

Commands were ordinary source reads (`cat`, `rg --files`, `rg -n`,
`sed -n`), SHA-256 reads (`shasum -a 256`), and Git revision/status
queries. Native tool transcript preserves full exact commands and outputs.
Source reads returned exit 0. Initial product HEAD queries each returned
d01ad98a754698631f927709d08284c272de85e8; initial status returned no paths.
The coordination HEAD query returned 3446bdf51d90b8f1812d161cc638d85c97bbef16.
The source manifest was emitted by `shasum -a 256` from <A1_WT>.
No numerical program or Python interpreter was run by the manager.

The two initial `git status --short` commands did not set
GIT_OPTIONAL_LOCKS=0. Both were intended read queries and returned a clean
status. No mutation subcommand was issued, but optional index stat-cache
refresh cannot be retrospectively excluded. This is a provenance uncertainty,
not an observed index mutation or evidence of a complete no-write history.
ROOT was informed and required GIT_OPTIONAL_LOCKS=0 on every further Git read.
The manager passed that requirement to both children and made no index repair.

The only manager file writes are the records in this directory, materialized
with apply_patch and the source-manifest redirection above. No maintained
solver/harness/validation, guard, instruction, graph or ruling was edited.
No Rust build/run, memory observation or experiment was started. Host/guard
state was not assumed from historical records and was not changed.

## Oracle handoff field check

I22 emitted ORACLE_INPUTS.json with SHA256
fdc86cca269c660e88a9a977990bf52bf3beadd08b67dc8fe8ab30358fdcd091.
The manager freshly hashed it, inspected its initial full primitive schema,
and ran a recursive scalar-path scan for expected, parameter, free_dofs,
note, outcome, result and hypothesis fields; that scan returned [].
ROOT separately reviewed the fields and released the data to a fresh oracle.
This validates only a data-interface restriction, not expected numerical truth.

ROOT reported that its first oracle saw old expected_rows before freezing
independent derivation and replaced that executor. The manager did not use an
oracle result from that executor. Existing matrix truth and old comparisons
remain provisional; denominator conventions remain unresolved until the fresh
oracle checks accepted text.

## Final child-integration source checks

The manager read H/observations/k6b/counts.jsonl's TREE n10000 AX row:
f=60000, q=250013, historical w(16)=144. A direct functions.exec JavaScript
calculation of 2*f*w - 8*(3*q+f) returned 10799688. This verifies I21's
principal arithmetic, not actual Option layout or the complete E_max.

H/src/k6/w1/staged.rs:52-87 formats refusal/source error before obs.end.
H/src/bin/k6_observe/main.rs:280-317 resets its observed stage after the
stage_begin line and snapshots before stage output. :946-989 keeps prefix
peaks separate and publishes repeats_heap_peak from observer.max_stage_peak.
VR/examples/vk_scale.rs:433-437 covers lane work; :483-514 subsequently
runs RCM/sparse parity and publishes repeats_heap_peak from alloc::peak.
Thus the two same-named metrics have different windows as I21 reports.

One manager read incorrectly guessed H/src/bin/k6_observe/staged.rs and
returned exit 1 after successfully printing the committed TREE counts row.
The subsequent rg --files lookup found H/src/k6/w1/staged.rs and its complete
needed section was read successfully. This failed path guess is not a check.

Child manifests were rechecked from their own directories: I22 six payloads
OK; I21 seven payloads OK. Read-only Git diff/cached-diff queries returned no
tracked changes. Untracked paths were inventoried, separating ROOT's oracle
assignments from manager/I22/I21 ownership. No oracle truth was inspected.
