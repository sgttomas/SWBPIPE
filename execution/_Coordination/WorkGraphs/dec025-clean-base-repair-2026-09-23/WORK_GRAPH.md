# Work graph — DEC-025 clean-base repair

## Intent and selected route

- Stable run identity: `PIP-DEC025-BASELINE-2026-09-23`.
- Intended result: repair the six Python failures reproducible at clean `10b672cae` that blocked the DEC-025 sweep for draft PR #867; obtain a passing five-surface sweep on the final commit, independent review, CI and a merged repair PR.
- Steering basis: HELP_HUMAN's bounded Piping assignment relayed to WORKING_ITEMS on 2026-09-23; no product, lifecycle, issuance or release decision is implied.
- Included: current test expectations for local-first wording and approved DAG-011; the PKG-00 validator and current hash attestation after accepted reconciliation/SCA-011; required checks and closeout. App/Piping `Remaining` retirement, root path validation and manuals have separate owners.
- Route: DEL-12-01 SOW-029 policy test, DEL-10-04 SOW-032 release-readiness test, and PKG-00/DEL-00-08 SOW-063 architecture-quality check. The approved project graph is DAG-011. DEC-025 governs the final sweep.
- Open questions: none for the candidate repair. D-43 historical consolidation hashes remain historical; a separately labeled current table binds live bytes.

## Deliverable scope

| Deliverable / basis | What exists | Change or resolution | Work nodes |
|---|---|---|---|
| DEL-12-01 / SOW-029 | Live storage policy says `SWBPIPE`; test expected former name. | Align exact wording assertion with current policy. | W1 |
| DEL-10-04 / SOW-032 | Release tool follows `_LATEST.md` correctly; four tests pin DAG-010. | Assert adopted DAG-011 and retain approved-pointer behavior. | W1 |
| PKG-00 / DEL-00-08 / SOW-063, D-43 | Eight current ArchitectureBasis files differ from original D-43 consolidation bytes after accepted reconciliation; current decomposition is 0.13 and preserved setup context says `revision-0.7`. | Keep historical hashes unchanged; add current hashes, retain live cryptographic checks, update validator currentness pins and negative test. | W1 |

## Work

| ID / outcome | Deliverables and work scope | Needs / why | Completion check | State / result |
|---|---|---|---|---|
| W1 Repair six failures | Three bounded test/validator surfaces and additive manifest table; single integration owner | Clean-base diagnosis and accepted D-43, DAG-011, SCA-011 sources | Focused tests and PKG-00 validator pass; original six failures explained | COMPLETE at `d53ed95e2`: 28 focused tests and validator PASS; independent P2 raw-byte finding repaired and backchecked |
| V1 Verify final candidate | Same surfaces plus registered DEC-025 five-surface sweep | W1 frozen and rebased after preceding main integrations | Commit-bound clean sweep PASS; independent fresh-context complete-diff review; required CI on actual head | BLOCKED by reproducible full-suite Vitest failure outside W1 (one of 1,621 tests; stress-neutral panel empty branch); read-only diagnosis assigned separately. Failed clean sweep retained below. |
| C1 Bounded closeout | DEL-12-01, DEL-10-04, DEL-00-08 and manifest/currentness records | W1 and V1 evidence | Confirm no further documentation, reconciliation or Task Management change is warranted; retain historical D-43 table | READY after V1 |
| M1 Run index | Affected deliverable MEMORY rows | C1 result and PR identity | Terse rows linking repair graph and PR | READY after C1 |
| F1 Final PR | All repair files, graph, pointer, run index | W1/V1/C1/M1 complete | Required CI and independent review cover actual PR head; merge verified | PLANNED |

## Current state and recovery

- Checked basis: `origin/main` at `10b672caed0a0e013ac72508ac72f6b3ce274286`; accepted DAG-011 pointer and SCA-011 owner decision. Focused reproduction: six failures and 21 passes. Repair candidate `d53ed95e2`: 28 focused tests and PKG-00 validator PASS. Clean DEC-025 sweep `validation/evidence/sweeps/SWEEP_20260923T134800Z_d53ed95e2a99.json`: cargo 38 manifests PASS, Python 1,139 tests plus 7 subtests PASS, desktop Vitest 1 of 1,621 FAIL, e2e/build NOT RUN. Full Vitest rerun reproduced only the same failure; isolated stress-neutral file passed 8/8.
- Next work: diagnose full-suite-only stress-neutral empty branch without weakening its assertion, rebase after PR #869 integration, then run complete sweep and independent review, perform bounded closeout and open/merge PR.
- Local/unmerged work: branch `codex/piping-dec025-baseline`, isolated `/private/tmp/chirality-piping-dec025-baseline`; draft PR #867 belongs to another Piping undertaking and remains blocked by this prerequisite.
- Active operations and ownership: WORKING_ITEMS owns all writes in this graph. HELP_HUMAN coordinates independent reviewer capacity.
- Graph maintainer: WORKING_ITEMS for this repair.
