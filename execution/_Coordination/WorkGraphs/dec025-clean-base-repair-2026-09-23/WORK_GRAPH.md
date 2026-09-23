# Work graph — DEC-025 clean-base repair

## Intent and selected route

- Stable run identity: `PIP-DEC025-BASELINE-2026-09-23`.
- Intended result: repair the six Python failures reproducible at clean `10b672cae` that blocked the DEC-025 sweep for draft PR #867; obtain a passing five-surface sweep on the final commit, independent review, CI and a merged repair PR.
- Steering basis: HELP_HUMAN's bounded Piping assignment relayed to WORKING_ITEMS on 2026-09-23; no product, lifecycle, issuance or release decision is implied.
- Included: current test expectations for local-first wording and approved DAG-011; the PKG-00 validator and current hash attestation after accepted reconciliation/SCA-011; a bounded full-suite-only timing repair in the DEL-17-06 stress-neutral test; required checks and closeout. App/Piping `Remaining` retirement, root path validation and manuals have separate owners.
- Route: DEL-12-01 SOW-029 policy test, DEL-10-04 SOW-032 release-readiness test, PKG-00/DEL-00-08 SOW-063 architecture-quality check, and DEL-17-06 SOW-046/SOW-074 stress-neutral test. The approved project graph is DAG-011. DEC-025 governs the final sweep.
- Open questions: none for the candidate repair. D-43 historical consolidation hashes remain historical; a separately labeled current table binds live bytes.

## Deliverable scope

| Deliverable / basis | What exists | Change or resolution | Work nodes |
|---|---|---|---|
| DEL-12-01 / SOW-029 | Live storage policy says `SWBPIPE`; test expected former name. | Align exact wording assertion with current policy. | W1 |
| DEL-10-04 / SOW-032 | Release tool follows `_LATEST.md` correctly; four tests pin DAG-010. | Assert adopted DAG-011 and retain approved-pointer behavior. | W1 |
| PKG-00 / DEL-00-08 / SOW-063, D-43 | Eight current ArchitectureBasis files differ from original D-43 consolidation bytes after accepted reconciliation; current decomposition is 0.13 and preserved setup context says `revision-0.7`. | Keep historical hashes unchanged; add current hashes, retain live cryptographic checks, update validator currentness pins and negative test. | W1 |
| DEL-17-06 / SOW-046, SOW-074 | Stress-neutral packet construction hashes 830 rows asynchronously; a native-save test waits only the default 1s for the button. | Give that one readiness wait 10s while preserving every native-save, intent and source-generation assertion. | W2 |

## Work

| ID / outcome | Deliverables and work scope | Needs / why | Completion check | State / result |
|---|---|---|---|---|
| W1 Repair six failures | Three bounded test/validator surfaces and additive manifest table; single integration owner | Clean-base diagnosis and accepted D-43, DAG-011, SCA-011 sources | Focused tests and PKG-00 validator pass; original six failures explained | COMPLETE at `d53ed95e2`: 28 focused tests and validator PASS; independent P2 raw-byte finding repaired and backchecked |
| W2 Repair suite-only timing failure | DEL-17-06 test only | Failed d53 sweep and read-only diagnosis: async packet remained pending past default 1s wait | Isolated 8 tests and full 100-file Vitest pass with all assertions retained | COMPLETE in working candidate: 8/8 isolated and 1,621/1,621 full Vitest PASS |
| V1 Verify final candidate | W1/W2 plus registered DEC-025 five-surface sweep | Both repairs frozen on merged `600d963ec` main basis | Commit-bound clean sweep PASS; independent fresh-context complete-diff review; required CI on actual head | COMPLETE — `SWEEP_20260923T141926Z_8a8a247673b9.json` binds a clean 8a8a24767 five-surface PASS; independent source and evidence-tail review PASS; required `harness` SUCCESS on exact PR head |
| P1 Substantive PR | W1/W2 implementation, current-hash attestation, graph and sweep evidence | V1 PASS and independent review | Merge repair PR after required CI on its actual head | COMPLETE — [PR #872](https://github.com/sgttomas/chirality/pull/872) merged as `db5bb38730f4f59e6c3f3152bea38af2f0c4bb1d` |
| C1 Bounded closeout | DEL-12-01, DEL-10-04, PKG-00/DEL-00-08, DEL-17-06 and manifest/currentness records | P1 merged | Compare delivered result with owned SOW and governing records; make warranted edits, route any exceptional concern, preserve D-43 history | COMPLETE — source-faithful no-change outcome for SOW, dependencies, status, scope, decisions and Task Management; central comparison in `execution/_Coordination/AgentRuns/PIP-DEC025-BASELINE-2026-09-23/EVIDENCE.md` |
| M1 Run index | MEMORY rows only for deliverables with actual work | C1 result and P1 URL | Terse local work entries linked to graph and PR | COMPLETE in this final-records candidate — DEL-00-08, DEL-10-04, DEL-12-01 and DEL-17-06 |
| F1 Final records PR | Graph, closeout consequences and run index | P1/C1/M1 complete | Independent review and required CI cover final records head; merge verified | ACTIVE — [PR #873](https://github.com/sgttomas/chirality/pull/873) is the final records candidate. F1 completes only when this reviewed PR passes required CI on its actual head and actually merges; Git/PR evidence establishes the merge afterward. |

## Current state and recovery

- Checked basis: substantive [PR #872](https://github.com/sgttomas/chirality/pull/872) merged at `db5bb38730f4f59e6c3f3152bea38af2f0c4bb1d`; exact source sweep and earlier failed attempts are linked from `execution/_Coordination/AgentRuns/PIP-DEC025-BASELINE-2026-09-23/EVIDENCE.md`. Accepted DAG-011 and SCA-011 remain current. Four affected deliverable records were compared against the merged source using `chirality-root:bundled:workflow:bounded-reconciliation`; no substantive SOW/dependency/status amendment or Task Management intake was warranted.
- Next work: independently backcheck the PR #873 URL/closure tail, pass required CI on its final head and merge; verify actual merged state and optional late CI outcome from PR #872.
- Local/unmerged work: [PR #873](https://github.com/sgttomas/chirality/pull/873) on branch `codex/piping-dec025-records-closeout` in an isolated worktree; draft PR #867 belongs to another Piping undertaking and can resume its own gates after this baseline repair.
- Active operations and ownership: WORKING_ITEMS owns all writes in this graph. HELP_HUMAN coordinates independent reviewer capacity.
- Graph maintainer: WORKING_ITEMS for this repair.

## Supplied basis

The actual instruction/method bytes read for this run were Root `AGENTS.md` (`1bb670ca339a990b153cf033dac2d8e29ca71bdea0accee4200e6dd1feed3d57`), Piping `AGENTS.md` (`60d9af6492e415f68ba91176530316545ddc25e4c0fddc6fb3192beb10d10100`), `agents/AGENT_WORKING_ITEMS.md` (`9ae4bea25bd95750a6878a9d53fbbbd7cd058d72c652a36baf4aa90601799665`), `.agents/skills/software-defect-diagnosis/SKILL.md` (`f281558734732ab0a97cf1407ba80a53c25e8c2b9dd96069bec589fd521a7a23`), `.agents/skills/chirality-change/SKILL.md` (`2b490e172436417896c1cd25dbcd543c676e3473aa58b7663985d75785ff7dba`) and `workflows/construct-local-work-graph/WORKFLOW.md` (`060f7153fb8a1b75825b23a799c58de45b6538b5e95d758f777079925b121c47`). Authority sources included `_DAG/_LATEST.md` (`6335bc5cb1b0d1c13c41181b0fbd3e3ab4547fa355b6c9ddc49513307b721422`), SCA-011 `OWNER_DECISION.md` (`6797151ce24132d0c6cf4fa4d1976bc4a3b73557e6cc7d52e268140fc346c9d3`) and D-43 ruling (`7ba1e30f0e464a0883b2704482c3e404aeda3e2837d041e0c9a323b39d07015e`).
