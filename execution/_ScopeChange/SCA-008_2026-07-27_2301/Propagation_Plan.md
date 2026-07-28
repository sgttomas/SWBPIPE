# Piping SCA-008 Gate 4 — exact propagation plan

## State and accepted basis

`CANDIDATE — GATE 4 NOT APPROVED`

This plan is limited to the owner-approved Gate-3 candidate:

- durable main basis:
  `4f8448143608588b06e968cf1ff07054ee1da60c`;
- approved Gate-3 candidate-set manifest:
  `e42f7bddabffa3b18db09dab9aef9710a1f55302df9a8cd774e87b6070786c72`;
- approved Gate-3 artifact manifest:
  `528e3a0637c154aa90003da14c782438065b1499ea4d99844eded2d450758f3f`;
- exact decomposition postimage:
  `82b835b5fd36d0fd337da5b084dbf146caa29c18d0e1ef8f96a06fcfa4363a07`;
- exact DEL-16-04 status postimage:
  `39af5d4b4ebfcf0c0c46b122f29d33f33b0774094be317201ac8a8269572b366`.

Gate 4 approves propagation only. It applies nothing.

## Exact Gate-5 write scope

The complete prospective live tranche is exactly 20 paths: two approved
working-surface postimages, two pointers, thirteen immutable snapshot members,
and three notices.

### Working surfaces

| Path | Role | Gate-5 action | Exact control |
|---|---|---|---|
| `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md` | working surface | `REPLACE` | Gate-3 exact postimage SHA-256 `82b835b5…` |
| `projects/chirality-piping/execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-04_Agent rationale and professional-boundary controls/_STATUS.md` | working surface | `REPLACE` | Gate-3 exact postimage SHA-256 `39af5d4b…`; sibling `MEMORY.md` paired read required; lifecycle remains `IN_PROGRESS` |

No `_CONTEXT.md` edit is required. `DEC-091` is `ADD OTHER`, not a new
package or deliverable, so no PREPARATION scaffold or INIT-TASK is required.

### Pointer postimages

| Path | Role | Preimage SHA-256 | Exact postimage |
|---|---|---|---|
| `projects/chirality-piping/execution/_Decomposition/_LATEST.md` | snapshot / handoff artifact | `f0d547636d9009194dd8913c2c78252f3f7a378ed3f0a2e28549abe25f6c9090` | `postimages/DECOMPOSITION_LATEST.md` / `fa6f3bcdea5e27f916a607b9bcfe2c3bee445f21dd533b13f9b57f7f38cfc807` |
| `projects/chirality-piping/execution/_ScopeChange/_LATEST.md` | snapshot / handoff artifact | `4ac5e2532468ea3e8136916452f51b3ac8c92f01eba3c25a3a46a295edf5f775` | `postimages/SCOPE_CHANGE_LATEST.md` / `58852dd8bf88f504f7b09a9978258d969294e563cdf7d965af8450c6617dab44` |

The decomposition pointer is staged after its postimage validates. The
scope-change pointer is the last live write and cannot advance until all 13
snapshot members exist and pass completeness checks.

### Immutable snapshot

Exact folder:

`projects/chirality-piping/execution/_ScopeChange/SCA-008_2026-07-27_2301/`

Required members:

1. `Brief.md`
2. `Impact_Assessment.md`
3. `Amendment_Preview.md`
4. `Propagation_Plan.md`
5. `Amendment_Actions.csv`
6. `Pre_Change_Coverage.json`
7. `Post_Change_Coverage.json`
8. `Decision_Log.md`
9. `Handoff_State.md`
10. `RUN_SUMMARY.md`
11. `ACCEPTANCE_RECORD.md`
12. `Supersession_Delta.csv`
13. `Supersession_Map.csv`

`Snapshot_Source_Map.csv` supplies the exact copy source or deterministic
generator and validation rule for each member. The exact brief and
supersession CSV candidates are under `snapshot_candidates/`.

`Supersession_Delta.csv` is header-only because SCA-008 does not rewrite or
repeal an admitted historical authority fact. `Supersession_Map.csv` carries
forward the three SCA-005 rows using
`tools/coordination/accumulate_supersession_map.py` with no current delta
rows. Because the current tool emits CRLF, the approved exact map normalizes
only line endings to LF; `diff --strip-trailing-cr`, all three row hashes, row
order, and field values must remain identical. This is not a hand merge.

### Exact coordination notices

| Receiving loop | Live path | Exact candidate SHA-256 |
|---|---|---|
| Root | `execution/_Coordination/NOTICE_2026-07-27_PIPING_SCA-008_CURRENT_EFFECT_RECONCILIATION.md` | `notices/ROOT_NOTICE.md` / `2b3ba1ba975de706a65a468040a684789748fb48fcce4f9182a4ebdc4f8dcb94` |
| App | `projects/chirality-app-dev/execution/_Coordination/NOTICE_2026-07-27_PIPING_SCA-008_CURRENT_EFFECT_RECONCILIATION.md` | `notices/APP_NOTICE.md` / `1082985345615fe20ccf7d61603631caa62f79981ecfd9a8507888d03adbdf26` |
| Tier-0 | `_DomainEngines/_Coordination/NOTICE_2026-07-27_PIPING_SCA-008_CURRENT_EFFECT_RECONCILIATION.md` | `notices/TIER0_NOTICE.md` / `5937c299355e5bc5b0d681c2620ea30b295bb5425801bdcc599b060147f418fd` |

Notices are coordination, not authority. Delivery is required; receipt and
acknowledgment are tracked as open coordination state and are not a closure
veto.

## Surfaces explicitly unchanged

| Surface / family | Package role | Disposition |
|---|---|---|
| `projects/chirality-piping/docs/PRD.md`, including R7 | working surface | `NO_CHANGE` |
| `ScopeLedger.csv`, `Deliverables.csv`, `ContextBudgetQA.csv` | authoritative companion registers | `NO_CHANGE` |
| D-30, D-31, D-58 proposal/ruling/effective closeout and decision-register rows | working surfaces / governance inputs | `NO_CHANGE` |
| DEC-041 and DEC-063 existing decomposition rows | working surface | `NO_CHANGE` |
| DEL-16-04 sibling `MEMORY.md` | working surface | `PAIRED_READ_ONLY` |
| every ScopeOfWork and every other `_CONTEXT.md`, `_STATUS.md`, `MEMORY.md`, `_REFERENCES.md` | working surfaces | `NO_CHANGE` |
| `Dependencies.csv`, DAG edges, source, schemas, runtime, profiles, implementation evidence | downstream or product truth | `NO_CHANGE` |
| estimates and schedules | downstream snapshots | `NO_CHANGE` |
| SCA-007 snapshot | historical snapshot | `NO_CHANGE / IMMUTABLE INCOMPLETE RESIDUE` |
| lifecycle, release, professional-reliance records | governed downstream state | `NO_CHANGE` |

No stable ID is renumbered or reused. No package, deliverable, scope item,
objective, requirement, or dependency edge is added, removed, reclassified,
merged, or split.

## Gate-5 transactional sequence

### P0 — execution-time preconditions

Before any write:

1. verify the then-current accepted basis descends from or is byte-equivalent
   on all 20 target preconditions to
   `4f8448143608588b06e968cf1ff07054ee1da60c`;
2. reproduce the four live preimages:
   decomposition `dbcbd4c596…`, status `632a0d418…`, decomposition pointer
   `f0d547636…`, and scope-change pointer `4ac5e253…`;
3. confirm `DEC-091` remains next-free and absent, SCA-008 remains next-free,
   and the exact snapshot and notice paths remain absent;
4. reproduce the three D-58 current-effect records;
5. read DEL-16-04 `_STATUS.md` and sibling `MEMORY.md`, recording that memory
   is non-authoritative and introduces no contrary caveat;
6. reproduce 76 scope rows, 18 packages, 101 deliverables, 101 context rows,
   18 objectives, and envelope distribution `S=9, M=69, L=23, XL=0`; and
7. run fresh pre-change `AUDIT_DECOMP`. Admit COV-230 and COV-231 as the two
   expected active-SCA-007 blockers; any additional blocker pauses Gate 5.

Any basis, preimage, ID, count, or target-path drift returns to impact review.

### P1 — isolated candidate assembly

Build the complete prospective 20-path state in a temporary staging root.
Apply only the two approved Gate-3 postimages, exact pointer candidates,
13-member snapshot contract, and three exact notices.

Create `Pre_Change_Coverage.json` from the fresh pre-write audit. Populate all
runtime-generated snapshot fields only from named audit, gate, path-manifest,
and hash outputs in `Snapshot_Source_Map.csv`.

Generate the supersession map with the registered accumulator, normalize only
CRLF to LF, and compare against
`snapshot_candidates/Supersession_Map.csv` SHA-256
`e476eb6fc3f98c7e2da86a4fea98115e11c6eef8f3f9eb23cdaa0ac4f0d20fa6`.

### P2 — staged post-change audit

Run `AUDIT_DECOMP` against the complete staged post-state with SCA-008 as the
active snapshot. Copy the resulting exact coverage summary to
`Post_Change_Coverage.json`, refresh only runtime-derived hashes and evidence
anchors in the fixed snapshot templates, then rerun the staged audit.

Required result:

- COV-230 absent;
- COV-231 absent;
- no new blocker;
- all 13 snapshot members present;
- active pointer parity at SCA-008 / revision 0.11;
- 76 / 18 / 101 / 101 / 18 topology unchanged;
- envelopes `S=9, M=69, L=23, XL=0`;
- exactly one DEC-091 and unchanged DEC-001 through DEC-090;
- DEL-16-04 remains `IN_PROGRESS`;
- all 20 exact paths match the approved or deterministically generated
  candidate bytes; and
- `git diff --check` passes in the staging root.

### P3 — live application and pointer-last rule

Only after P2 passes:

1. apply the exact decomposition and DEL-16-04 postimages;
2. materialize the complete, already-validated SCA-008 snapshot;
3. add the three exact notices;
4. write the decomposition pointer;
5. recheck the snapshot's 13-member completeness and hashes; and
6. write `_ScopeChange/_LATEST.md` last.

Run the final live `AUDIT_DECOMP`, exact-path manifest verification,
whitespace checks, reference resolution, ID uniqueness, register parity, and
pointer parity. A failed final check invokes rollback; it is not recorded as
closure.

## SCA-007 blocker handling

SCA-008 never edits SCA-007. The repair is current-state-only:

- the pre-audit preserves COV-230 and COV-231 as evidence;
- SCA-008 contains the missing active-snapshot artifact classes;
- `_ScopeChange/_LATEST.md` advances only after completeness validates;
- the post-audit must no longer report COV-230 or COV-231 against active
  state; and
- `Handoff_State.md` names SCA-007 as immutable historical incomplete
  residue.

If either blocker remains after staged or live pointer advancement, SCA-008
cannot close.

## Downstream rerun and derivative-state plan

These are handoffs, not Gate-5 executions:

| Consumer / package | Owner | State after SCA-008 | Required later action |
|---|---|---|---|
| `_DAG/DAG-008` | PROJECT_SETUP / dependency-extract | `STALE_REVALIDATION_REQUIRED` | Rebuild or revalidate against revision 0.11; prove unchanged topology and edges |
| DEC-063 / DEL-16-04 concordance outputs | RECONCILIATION | `STALE_REBUILD_REQUIRED` | Refresh current-authority reading; preserve frozen historical runs |
| ScopeOfWork corpus | WORKING_ITEMS | `CURRENT_WITH_RELIANCE_HOLD` | No edit; do not release work requiring a current embedded-agent mechanism |
| estimate snapshots | estimate owner | `CURRENT_BY_NO_STRUCTURAL_EFFECT` | No recomputation required unless a local basis policy requires it |
| schedule snapshots | PROJECT_SETUP schedule owner | `CURRENT_BY_NO_STRUCTURAL_EFFECT` | No sequencing change; record revision revalidation only if consumed |
| Root/App/Tier-0 notices | receiving-loop owners | `ROUTED_COORDINATION` | Disposition under their own instruments and cadence |
| product/runtime/source/schema/test surfaces | owning workflows | `NO_CHANGE` | No action authorized |

No downstream rerun is represented as complete by SCA-008.

## Expected closure state

If and only if all final checks pass:

| Field | Value |
|---|---|
| `DecompositionTruthState` | `COMPLETE` |
| `DerivativePackageState` | `COMPLETE` for SCA-owned snapshot/pointer surfaces |
| `ContentRemediationState` | `NOT_REQUIRED` |
| `DownstreamRerunState` | `FROZEN` |
| `MetadataAlignmentState` | `NOT_REQUIRED` |
| `AuditState` | `WARNINGS` if only pre-existing non-blocking warnings remain; otherwise `NON_BLOCKING_PASS` |
| `ReadyForNextPhase` | `REGEN_ONLY` |
| Closure verdict | `CLOSED_FOR_SCOPE_CHANGE_ONLY` |

Open DAG/reconciliation reruns and notice acknowledgments remain explicit
handoff items. Closure never means product, runtime, lifecycle, release, or
professional acceptance.

## Rollback

Before live application, discard the temporary staging root only.

After an uncommitted live application failure:

1. restore the exact four preimages listed in P0;
2. remove only
   `projects/chirality-piping/execution/_ScopeChange/SCA-008_2026-07-27_2301/`;
3. remove only the three exact new notice paths;
4. rerun `AUDIT_DECOMP` and confirm active state has returned to SCA-007 /
   revision 0.10 with the same two pre-existing blockers; and
5. retain the failed Gate-5 evidence outside authoritative live state for
   diagnosis.

Rollback does not revive current reliance on the App-era mechanism. D-58
continues to govern. After Git integration, rollback requires a new
owner-directed forward SCOPE_CHANGE; accepted history is never deleted.

## CHANGE handoff after later Gate-5 confirmation

Only after the owner confirms the post-change state, hand off exactly the
validated 20-path tranche to CHANGE with recommended message:

`docs: accept Piping decomposition revision 0.11`

Gate 4 does not authorize that Git action.
