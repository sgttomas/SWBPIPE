# Final bounded runtime evidence backcheck

The 33-file consumer contribution is suitable for manager fan-in. No actionable findings remain after source review, F1–F4 repair backchecks and inspection of the allocated runtime outputs. All 33 source hashes match [CHECKED_CONSUMER_FILES.json](../CHECKED_CONSUMER_FILES.json), copied as [SOURCE_HASHES_CHECKED33.json](SOURCE_HASHES_CHECKED33.json). Final scope validation finds exactly the 33 authorized paths with no violations. The detailed machine evidence and raw-log hashes are in [_run_records/CHECKED33_EVIDENCE_BACKCHECK.json](_run_records/CHECKED33_EVIDENCE_BACKCHECK.json).

The reviewer did not execute runtime commands. Independently inspected the manager’s raw logs, command/cwd/environment/exit records, build basis and actual on-disk artifact hashes:

| Allocated check | Observed evidence |
|---|---|
| Units crate | 17 passed, 0 failed, including identity/pressure finite guards |
| Operation-applier display filter | 3 passed, 0 failed; broader applier suites filtered out |
| Candidate WASM build | Operation and self-weight engines built successfully; all eight emitted artifact hashes match BUILD_BASIS |
| Initial focused Vitest | 12 files, 142 cases: 133 passed, 9 failed, all failures confined to the 15-case export adapter file |
| Repaired export adapter Vitest | All 15 cases passed |
| TypeScript | Initial and final noEmit checks exit 0, empty error logs |
| Native precision filter | 3 passed, 0 failed; other 102 native tests filtered out |

The distinct focused Vitest coverage is 127 unaffected passing cases plus all 15 cases in the repaired export file, totaling 142. This is not described as one combined final 12-file rerun. Only that test file changed after the initial frontend run; production source and the other focused files retain their reviewed hashes.

The initial export failures are retained in the evidence. Each failed at or behind numerical admission because the copied historical fixture contains four duplicated warning IDs. The repair maps copied diagnostics to explicit unique synthetic IDs while retaining every diagnostic field and including the old ID in the new label. It does not weaken the production uniqueness guard or change immutable fixture bytes. The helper and suite are expressly labeled synthetic consumer admission, not producer or numerical qualification. Reconstructing the prior file by removing only that mapping and its explanatory labels reproduces its exact former hash.

The added generated native Cargo.lock changes only local product physics from 0.1 to 0.2 and records sparse-direct’s local frame-kernel dependency; no external package versions change. The native command record preserves both its pre-run and generated post-run lock hashes. Cargo.toml and native lib.rs hashes remain unchanged across the passing run.

The native precision cases exercise the reviewed independent tiny positive/negative cantilever oracle, same-unit values through native JSON, isolated SQLite close/reopen/resave, canonical hashes, exact-byte download and rule-result binding; all 26 shared vectors are consumed. The policy fixture’s passing labels test consumer admission only. The real tiny solves preserve actual quality metadata and do not by themselves prove a genuine structural-gate PASS. The actual WASM display tests use the real generated service path, not a converter mock.

These checks qualify only the bounded source/tests above on the recorded candidate and artifacts. Actual native UI workflow evidence, a genuine product gate-qualified browser Current fixture, integrated M03/kernel/protected gates, whole-candidate review and required DEC-025/CI/merge gates remain ROOT-owned. No engineering acceptance, formulation/pressure correctness or release claim follows from this return. Changed sibling dependencies or integrated source require affected evidence reassessment.
