# RV33 H integration/helper backcheck

**One blocking finding: RV33-H1 (P1), runner prepass before the ascent hold.**
Candidate `9086964a1fb656a76cda6d1002d8594efa636fdc` is not suitable for complete
H integration fan-in until that ordering defect is repaired and independently
backchecked. No additional actionable defect was found in the bounded H caller,
raw-helper extraction, diagnostic projections or record migration.

Scope is exactly the sixteen H paths owned by `I21/h_integration_25`, compared
with `4a6cb3402f95d98c6f46f702055fe4b0c5245c56`. H means
`projects/chirality-piping/core/solver/performance_harness`; R is the enclosing
`RESUME_2026-09-30`. VR integration is independently owned and excluded. The prior
kernel review remains closed for its exact earlier candidate; this packet adds
only the helper/integration backcheck described below. Raw metadata and machine
paths are confined to `_run_records`.

## Blocking finding

**RV33-H1 — apply the ascent hold before the model-specific counts prepass.**
At `H/runner/k6_runner.py:1201–1205`, a nonconditional W1 row launches its
counts-binding child before calling admission. The pre-existing ascent check is
inside `admission` at lines525–530. A fresh W1-T2 CHAIN100 row with no CHAIN10
receipt therefore starts model construction and counting, then is recorded as
`not_run` with `deferred:ascent_previous_size_not_recorded`. W1-T3 has the same
ordering problem when its predecessor is absent. Normal solve launch still
remains deferred; the new counts work has already run.

A process-free reproduction against the archived exact candidate records the
sequence `launch(--counts-only)`, then the ascent-deferred admission decision.
All launch/baseline functions were mocked; no real model, count, or solver process
was dispatched by the reproduction. The new child has existing caps/watchdog,
but those limits do not satisfy the missing nonnumeric eligibility condition.
See `_run_records/REPRO_RUNNER_HOLD.py` and `RUNNER_HOLD_REPRO.json`.

Repair by checking nonnumeric eligibility before binding, then checking all
estimate-dependent predicates after the real prepass result arrives. Preserve
already-recorded, named, conditional and ascent decisions and the existing tier
no-op baseline. Do not use fabricated estimate/count values merely to query the
nonnumeric conditions. Add a regression requiring zero model-specific prepasses
for an ascent-held row. The current50 runner tests pass while this defect remains.

ROOT confirmed the finding and commissioned a separate I24 H/VR runner repair.
This review stays on908 and accepts none of that subsequent working delta.
Current H conditional rows skip the binding branch; plan remains process-free;
current never-by-name cases are non-W1 modes and do not enter the new branch.
The confirmed H defect is ascent ordering, not a claim that those existing paths
also launched model work.

## Reviewed behavior

| Surface and candidate anchor | Result |
|---|---|
| `models.rs:169–229`, `canonical.rs:180–188`, `h_envelope.rs:238–278` | Actual builder, fixture and canonical branches carry an inline origin. Canonical family/id text does not select the allocation recipe. K6Model and element representations are unchanged. Retained nodes/member/restraint/load/label/frame recipes match context16 and RV32's constructor mapping, including raw restraint rows and UTF-8 byte lengths. |
| `w1/counts.rs:166–237`, `h_envelope.rs:129–237` | Capture reuses the existing source/graph/free/layout/encoding computations. Success records actual retained ID sum/maximum before source drop. File counts bind scalar populations to the surviving model while retaining external truthful graph/count provenance. Refusal stays a separate private variant using raw popcounts and global-DOF digits; it never synthesizes a successful kernel from failure zeros. |
| `envelope.rs:689–741,817–820` | Extraction retains the same H source-constructor alternatives, with axis/directional sorts omitted only because the successful H descriptor already requires them zero. The raw entry handles partial refusal populations without r<=n or g<n. All public successful kernel values match the unchanged 250-row literal overlay. VR kernel arithmetic is unchanged. |
| `h_envelope.rs:272–455` | Actual launch borrows all six surviving Args Strings, including unused paths. Repeat digits are inserted before the accepted formatter growth rule. Model/Args/runtime/saved-attempt/prefix owners compose with the shared kernel once; source, solve and outer-prefix windows remain separate maxima. Checked arithmetic and explicit errors replace stale/missing-value fallback. |
| `h_envelope.rs:386–413`, main counts serializer | SourceRefused has a positive complete source-window max=sel128 and caller-only fixed baseline. Kernel diagnostic zeros explicitly mean unexecuted phases under source_ok=false. The normal source stage still returns SourceRefused/stop_reason; file and computed paths were tested. No successful-source algebra is applied to raw failure counts. |
| `main.rs:270–307,643–722`; `k6/counts.rs:308–370` | One composed result flows to the counts line and half-cap backstop. Computed composition is after the existing count sample; loaded text/selected ID do not survive into observed W1 stages. No extra model/frame/source clone or second count pass is introduced by descriptor capture. Four other K6 formulas remain byte-for-byte algebraically unchanged apart from the Result wrapper. |
| `h_envelope.rs:458–621`; `tests/k6b_w1.rs:509–1210` | Diagnostic S/U/V and R7 values use shared owner results; own-solve and pass fields project named moving phase extras above Kpad. OriginalK6bPair is explicit reference-only, using the retained two launch recipes and original scalar metadata; ordinary execution uses actual context. No execution-tree reader or current/private-size mixture was introduced. |
| `runner/k6_runner.py:1077–1096,1195–1239` | Eligible runs receive a fresh planned-argv binding; returned estimates drive admission and the recorded ratio denominator. Binding failure raises, without stale fallback. Plan and conditional branches avoid the new binding. Ascent eligibility is incorrectly placed after binding (H1). Existing caps/timeout/watchdog and no-op baseline are retained. |
| `tests/k6b_w1.rs:375–407`; `src/bin/k6_observe/w1.rs:6–11` | C-N1 now stops ten LME after the128 shared build, checks partial positive own work, complete shared work, exact stage/total closure and zero unstaged work, and rejects one-LME under-recording. The ordinary test passed; no historical mutant-kill claim is made. |

Inspection finds no new estimator heap allocation: new facts/results/origin are
scalars, fixed arrays, enums and borrowed views, and the folds do not collect.
Existing builders, parser/count algorithms and error emission retain their
original heap behavior; this is not an assertion of zero whole-process cost or
an allocator measurement. Source/build/input provenance remains external.

The obsolete estimator and duplicate hand formula were replaced with the
accepted reference composition and independently sourced literals. Existing
storage/count/source identity, cardinality, ordinary parity/prefix/dump,
missing-count/FNV and half-cap assertions remain. Corrected phase dominance is
checked by named phase identity. The public-size observation exports remain
separate from reference facts.

## Independent validation

All16 author postimage hashes match908. The core and validation trees are exactly
the same as pre-merge39417b. Six consumed basis manifests (96 payloads), plus the
prior RV33 kernel manifest (25 payloads), verify. The author H packet seal is
`d7507f556bc21fbd8a35e20aca73c9e9209521092bac1efe83da47706afa3135`.

A clean exact-candidate core-subtree archive supplied the full local Cargo
closure and excluded execution evidence. All564 archived files remained
unchanged. A fresh dedicated target used installed Rust1.97.1, offline/locked,
incremental0, four build jobs, two test threads, and the existing memory guard.
No dependencies or tools were installed.

- Release: **33 maintained tests passed** across k6b_bin5, k6b_w1 16,
  k6c_envelope9 and k6c_h_envelope3.
- Unchanged independent kernel overlay: **250 tests passed**, covering193
  reference-upper,24 exact CLI and33 H rows; all **31,500 phase pairs** and
  **7,250 additional scalar/pair assertions** still match after extraction.
- Ordinary debug subprocess suite: **all5 k6b_bin tests passed**, including
  computed/file SourceRefused and actual-path/backstop correspondence.
- Runner: **all50 tests passed**. The additional mocked reproduction confirms
  H1 despite those passes; no blocked model work was actually launched.
- Record audit: exactly **726 authorized estimate scalars** changed across33
  rows. All keys, non-estimate values and row order are preserved. All726 new
  values match independent projections of H19; the adjacent manifest changes
  only the counts hash, while k6b_packet.json and sources.txt remain identical.

These are functional checks, not performance measurements. Large reference rows
use stored metadata; no10k solver/count benchmark was launched. Ordinary small
model/source/solver execution occurred only inside the named existing H tests.
The author's sealed all-targets91 plus allocator F1/F2 result was inspected as
author evidence; this reviewer did not rerun that entire suite or broad CI/DEC-025.
No ordinary artifact qualification or admission replay is established.

## Return boundary

H1 remains blocking on908. ROOT owns the separately commissioned repair and its
exact-candidate backcheck. No maintained/index edits or delegation were made by
this TASK; all Git reads used GIT_OPTIONAL_LOCKS=0. No later source delta is
accepted here, and no automatic continuation or time extension was initiated.

Final executable/input/launch qualification, integration-wide checks, chronological
admission/recalibration and measurements remain separate. This packet does not
accept VR implementation, full E_max, engineering outcome or release.
