# RV27 — final gate-evidence check at 729e80b5

**PASS, with the existing Mac-platform/shared-target qualifications preserved.**
No new merge-blocking gate-evidence finding. Candidate remains
`729e80b5c2a33b278623a850d9c75c25372b688c`; all prior review seals verify and
RV27-S1 remains closed. ROOT must still recheck main immediately before merge.
This check does not repair or close the independently confirmed BLOCKING A1 defect.

## Final-head binding

Local resume-records HEAD is exact and clean; PR1068 reports the same candidate
against `a38617d08bfbff6ea0a15b1cb129a16cbc82b2fc`. Hosted full-SHA workflow_dispatch
run **36819769945** is completed/successful at that head. Its numerical job,
four source-remainder partitions and required-coverage aggregate succeeded.

Downloaded the live selection artifact **11142633927** independently. Its plan
is byte-identical to ROOT's preserved plan: full mode, coverage_full=true,
numerical_required=true, all inventory specs selected, head729e80b5 and
base/target_basea38617d. Plan SHA256:
`1e8fdaec4f5d9c451b28f5d37818ff267517657c75cb4859c3fb8d69efaba9f5`.
The records PR checks are also success/skipped as selected. This is source-mode
CI evidence, not a native practitioner witness.

The final-head practitioner log supplied by ROOT records **419 passed**. The Mac
sweep metadata and original SWEEP JSON bind to the exact candidate and clean
starting tree. The sweep checkout still has that HEAD, no tracked-source diff,
and only its generated SWEEP JSON untracked. Driver completion and per-surface
logs show pytest, wasm build, Vitest and desktop build exit 0: 3070 pytest passes,
32 skips, 130 subtests; 134 Vitest files / 2822 tests; successful wasm and Vite
builds. Warnings in the logs are retained.

## Original comparison and controlled recovery

Independently keyed all **40** current cargo logs and all **40** KF2-baseline
logs by manifest, dropping positional indexes. The only result-count delta is
self_weight_wasm: baseline (14,0,0), original candidate no test result. The three
actual failed-test names are identical in both runs:

- product_physics: s11g_tests::t13_committed_fallback_uz_is_byte_identical;
- headless: cli_load_reference_one_both_modes_is_controlled_and_equals_the_library_route;
- headless: load_reference_route_tests::load_reference_one_actual_solve_mints_bound_evidence_and_canonical_document_both_modes.

The original self-weight log is a compiler failure, not an accuracy-test failure
or a zero-test pass. It names incompatible serde_json 1.0.150/1.0.151 types and
contains no executed test results. Its bytes and hash remain independently
inventoried. Neither this review nor ROOT rewrites it as passing.

The separately named isolated result contains 5 unit plus 9 integration passes,
0 failures/ignored, and 0 doc tests. Its tree contains only serde_json 1.0.151.
Its raw log hash is
`62e66b63941bca893238bdea99b738b827abb7de1485ca6c15582c5f530b395a`.
Substituting only this explicitly identified missing-suite recovery into the
keyed comparison produces **zero effective count/failure-name differences**.
The original fail-fast sweep JSON still says fail/incomplete; completed surfaces
and recovery are additive evidence under the standing Mac comparison disposition.

Requested and checked an additive EXECUTION_BINDING.json from ROOT's actual
execution record. It transcribes cargo test --offline --locked --no-fail-fast,
manifest self_weight_wasm, -j4, installed 1.97.1, auto-install=0, incremental=0,
RUST_TEST_THREADS=2 and a dedicated first-use target. Session64582 returned
exit0 at chunkc4a45b. Raw run paths independently show the isolated target.
Current target .rustc_info corroborates rustc1.97.1; the current crate lockfile
is byte-identical to final-head Git and agrees with the one-version tree.

The binding expressly discloses that no immediate pre-isolated HEAD/status
sample was captured. The exact-head sweep setup/meta, ROOT's reported uninterrupted
source history and the current unchanged-source postcheck provide the binding.
This is a truthful transcription plus corroboration, not an original launch log
or continuous no-mutation proof. That limit is retained rather than invented
away. It is sufficient for this bounded unchanged-candidate recovery check.

The evidence is consistent with the existing shared-target artifact limitation
already routed to T9 in WORK_GRAPH.md. It does not prove a universal causal
account or repair that limitation. No source, dependency lock, tool, tolerance
or test was changed to get the isolated pass. A passing rerun is not presented
as source-defect repair, A1 closure, product qualification or release.

## Execution boundary

Read-only metadata, logs, Git/GitHub and small standard-library count/hash checks
only; no Rust, solver, test rerun, candidate edit or Git/index mutation. Every
Git read used GIT_OPTIONAL_LOCKS=0. Wrote this additive gatecheck subtree and owned
scratch only. Prior review/seals remain unchanged. New candidate/main changes
still require reassessment before reliance. ROOT owns the immediate pre-merge
main check and merge operation.
