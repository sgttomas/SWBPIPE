# Piping SCA-009 Handoff State

**Current state:** `CLOSED_FOR_SCOPE_CHANGE_ONLY`

## Accepted upstream basis

- Branch basis: `main@89758a32634ee6cedbd1dbadf35e3728fb48d2eb` (merge of
  PR #591); gates accumulated on `claude/piping-sca-009-gate2-20260820`
  (single landing PR #593 per the owner's process direction; merge
  authorized by the Gate-5 ruling, executed by Agent 0 post-CI).
- Original durable basis (Gate-1 package):
  `7584de0a8d53d69a135c22fe39a78cb4a30b6cb2`; Gate-1 package merged as
  PR #592 (merge `01a8dd4c0aabd4fe1f71bba7201a4345f9e6cfdc`).

## Accepted gate rulings (verbatim in `ACCEPTANCE_RECORD.md`)

| Gate | Ruling target | State |
|---|---|---|
| 1 | Gate-1 package SHA-256 `2458c1dce9b175330c8b28a4a0e4647988213539ee4edb8d27f7ae74e0e9adc0` | `CONFIRMED` (D1; D2 Option A; D3 vocabulary ratified; D4 single owner; D5 fold with DEL-16-04 generator out; D6 SOW-077) |
| 2 | `Impact_Assessment.md` SHA-256 `bfa25d898e65b82012b2a93988432a121d5f2b842a5469cf7d53593a1a2ba6d0` at commit `f5112824f` | `APPROVED` with envelope-L override and landing column |
| 3 | `Amendment_Preview.md` SHA-256 `802c2ce92c5a48651f4d06312d4ba26593f0134e3b7c443988e74b79c0e170d4` at commit `d50e72c4b` (post-cleanup `44eaf63ab9de9a4703972acdedc39b65a760dce4f1b2b566134e861512a71eab`; six pair hashes unchanged) | `APPROVED` — apply exactly the six pairs, DEC-094, no other surface |
| 4 | The six approved postimage hashes | `APPLIED` — all six live files hash-proven (`RUN_SUMMARY.md`; `Validation_Record.md` §8) |
| 5 | Gate-5 package SHA-256 `0b07defec01a794659e891a0ff792aea615be442673e97b70b3ef453bd32e8fa` at commit `a08701951` | `ACCEPTED — CLOSED` ("… CLOSE the amendment … set the amendment state CLOSED_FOR_SCOPE_CHANGE_ONLY … Then merge PR #593.") |

## Decomposition and pointer state (post-closing-commit)

- Live `SOFTWARE_DECOMP.md`: **revision 0.12** (`SOW-077`; `DEL-07-09` at
  envelope L; OBJ-006/OBJ-015 and PKG-07 mappings; telemetry 77/102,
  `S=9, M=69, L=24, XL=0`; `DEC-094`).
- `execution/_Decomposition/_LATEST.md`: **advanced to revision 0.12**.
- `execution/_ScopeChange/_LATEST.md`: **advanced to SCA-009**
  (`SCA-009_2026-08-20_0000/`, status accepted; staged pointer-last).
- SCA-008 remains immutable and complete under its own snapshot contract;
  it is superseded only as the active current snapshot. All prior
  snapshots remain byte-identical history.

## State fields

**DecompositionTruthState:** `ACCEPTED — REVISION 0.12 ACTIVE`

**DerivativePackageState:** `STALE_PENDING_POST_CLOSURE_RERUNS` (table
below)

**ContentRemediationState:** `NOT_APPLICABLE`

**DownstreamRerunState:** `NONE_RUN — OBLIGATIONS RECORDED`
(`Propagation_Plan.md`)

**MetadataAlignmentState:** `NOT_APPLICABLE`

**AuditState:** `SYNTHESIZED_BASELINES_ONLY` — deterministic pre/post
coverage in `Pre_/Post_Change_Coverage.json`; the formal AUDIT_DECOMP
pre/post comparison remains a post-closure obligation

**ReadyForNextPhase:** `REGEN_ONLY` — new production against DEL-07-09
waits on the PREPARATION scaffold and the owning loop's own instruments;
the pointer state is complete

**ClosureVerdict:** `CLOSED_FOR_SCOPE_CHANGE_ONLY`

**NextOwner:** `Agent 0 (HELP_HUMAN)` for the authorized merge of PR #593
post-CI; thereafter the owning instruments in the table below

**NextAction:** merge PR #593 (authorized by the Gate-5 ruling; executed
by Agent 0 after CI), then the receiving instruments run their
post-closure obligations in their own cadence.

## Post-closure derivative-package obligations (owning instruments)

| Derivative package / consumer | Owner instrument | Status | Required action |
|---|---|---|---|
| Formal pre/post `AUDIT_DECOMP` comparison | AUDIT_DECOMP under SCOPE_CHANGE / owning loop | `RECOMPUTE REQUIRED` | Run against the pre-application basis and the accepted 0.12 state; the synthesized baselines are inputs, not substitutes |
| `_DAG/DAG-008` | PROJECT_SETUP / dependency-extract workflow | `STALE_REBUILD_REQUIRED` | Rebuild (topology change: +1 DEL, +1 SOW; proposed DEL-07-09 edges → DEL-16-01 / DEL-07-01 / DEL-07-02) |
| Deliverable-corpus concordance rows | RECONCILIATION | `STALE_REBUILD_REQUIRED` (targeted) | Current-authority refresh: DEL-07-09 (new), DEL-07-03 (re-pointed; supersession binding in `Supersession_Delta.csv`), boundary-adjacent DEL-07-01/02, DEL-16-01 |
| `Dependencies.csv` for DEL-07-09 | `dependency-extract` via TASK | `STALE_REBUILD_REQUIRED` (targeted) | Extract after PREPARATION scaffold |
| DEL-07-09 folder scaffold | PREPARATION | `REQUIRED` | Scaffold under the owning loop's dispatch |
| Estimate / schedule surfaces | estimate owner / PROJECT_SETUP | `ADVISORY_STALE` | Revision-basis revalidation only |
| Piping dev-loop coordination | receiving loop | `NOTICE ROUTED` | `NOTICE_2026-08-20_PIPING_SCA-009_DEL-07-09_VOCABULARY_PALETTE.md`; adoption under the loop's own instruments; acknowledgment tracked, not gating |

No derivative package may be represented as authoritative decomposition
truth.

## Boundary statements

This closure is for scope change only. No professional-reliance,
certification, sealing, authentication, or code-compliance claim is
created. No lifecycle value changed (DEL-07-03 remains `IN_PROGRESS`). No
implementation, release, estimate, or schedule effect, and no dependency
edge of any existing deliverable changed. Standard claim fence applies
(F-PIP-2; DEC-081 claims taxonomy).
