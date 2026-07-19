# Closed Verification Matrix v1 — Candidate Isolation and HEAD-Only Promotion

**Status:** `V7-PASSED; OWNER-CLARIFIED REVIEW CELLS`

This was the complete finite verifier contract for the architectural
correction. V7 verifiers tested its fixed pre-clarification M1–M12 exactly and
both returned `COMMIT-SAFE`. The owner subsequently superseded only the
repeat-review conditions in M8/M9/M11; the current rows record that governing
clarification. No candidate, loader, carry-forward, or activation-boundary
bytes changed.

| ID | Fixed invariant | Required evidence |
|---|---|---|
| M1 | Candidate isolation | The reviewed candidate exists only at R5 `WORKPLAN_CANDIDATE_2026-07-18_piping_loop.md`, outside `loop/WORKPLAN_*.md`; no discoverable 2026-07-18 plan exists in worktree, index, or HEAD. |
| M2 | HEAD-only namespace | LOOP_INIT enumerates only committed-HEAD paths under `projects/chirality-piping/loop/` whose basenames match `^WORKPLAN_.*\.md$`; untracked/staged-only names are excluded. |
| M3 | Deterministic selection | Bytewise `LC_ALL=C` lexical ordering selects exactly the last matching committed path; current HEAD selects `WORKPLAN_2026-07-17_piping_loop.md`. |
| M4 | Regular committed object | The selected HEAD entry is unique, mode `100644`, type `blob`; otherwise loading stops before Step 0. No older-plan fallback is permitted. |
| M5 | Committed-byte loading | Plan bytes are read only with `git show HEAD:<selected-path>`; worktree/index divergence cannot change the loaded plan. |
| M6 | Candidate carry-forward | Compared with the 2026-07-17 plan, governed candidate deltas are only re-mint/candidate metadata and DEC-087 Step 2; Owner intent through Step 1, the common Step-2 prefix, Step 3, Step 4 onward, and fences retain the required exact bytes. |
| M7 | Pre-landing inactivity | Current HEAD, index, and worktree contain no discoverable 2026-07-18 active name; current loader result is 2026-07-17 and DEC-087 is not applied. |
| M8 | Review closure | `S5_BLOCK_RETURN_01.md` remains preserved; the owner explicitly curtailed repeat sibling review because Shared-Block v1 and app-dev did not change; the interrupted repeat-review attempt has no verdict and supplies no authority. |
| M9 | Promotion authorization | CHANGE may materialize the active path only after both v7 verifier returns are `COMMIT-SAFE` and the tranche is `READY_FOR_DURABLE_LANDING`; no earlier actor creates it. |
| M10 | Byte-identical materialization | Candidate and eventual `loop/WORKPLAN_2026-07-18_piping_loop.md` must be byte-identical (`cmp -s` and equal Git blob IDs) before commit. |
| M11 | Atomic activation | Staging cannot activate the new plan because discovery reads HEAD. The same durable commit must contain candidate, byte-identical active copy, D-54/DEC-087, curtailed-review history, and updated LOOP_INIT; only that post-commit HEAD can select 2026-07-18. |
| M12 | Preservation/effect fence | Shared-Block v1, app-dev, D-49 through D-53, DEC-082 through DEC-086, old workplans, receipts, and DEL-09-04 remain unchanged; no refined-discretion effect, receipt, product/code, lifecycle/stage/release/acceptance/prover/publication/merge/push/external action occurs here. |

The v7 returns remain the independent evidence for the unchanged corrected
architecture. The owner's later clarification closes the repeat-review path
without a verdict; durable landing remains the only activation boundary.
