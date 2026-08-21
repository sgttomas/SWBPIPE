# Piping SCA-009 Handoff State

**Current state:** `GATE 5 CANDIDATE — CLOSURE RULING PENDING`

## Accepted upstream basis

- Branch basis: `main@89758a32634ee6cedbd1dbadf35e3728fb48d2eb` (merge of
  PR #591); gates accumulated on `claude/piping-sca-009-gate2-20260820`
  (single landing PR #593 per the owner's process direction). Ruled-upon
  commits `f5112824f` (Gate 2) and `d50e72c4b` (Gate 3) intact in history.
- Original durable basis (Gate-1 package):
  `7584de0a8d53d69a135c22fe39a78cb4a30b6cb2`; Gate-1 package merged as
  PR #592 (merge `01a8dd4c0aabd4fe1f71bba7201a4345f9e6cfdc`).

## Gate rulings (verbatim in `ACCEPTANCE_RECORD.md`)

| Gate | Ruling target | State |
|---|---|---|
| 1 | Gate-1 package SHA-256 `2458c1dce9b175330c8b28a4a0e4647988213539ee4edb8d27f7ae74e0e9adc0` | `CONFIRMED` (D1; D2 Option A; D3 vocabulary ratified; D4 single owner; D5 fold with DEL-16-04 generator out; D6 SOW-077) |
| 2 | `Impact_Assessment.md` SHA-256 `bfa25d898e65b82012b2a93988432a121d5f2b842a5469cf7d53593a1a2ba6d0` at `f5112824f` | `APPROVED` with envelope-L override and landing column |
| 3 | `Amendment_Preview.md` SHA-256 `802c2ce92c5a48651f4d06312d4ba26593f0134e3b7c443988e74b79c0e170d4` at `d50e72c4b` (post-cleanup `44eaf63ab9de9a4703972acdedc39b65a760dce4f1b2b566134e861512a71eab`; six pair hashes unchanged) | `APPROVED` — apply exactly the six pairs, DEC-094, no other surface |
| 4 | The six approved postimage hashes | `APPLIED` — all six live files hash-proven (`RUN_SUMMARY.md` proof table; re-proven in `Validation_Record.md` §8) |
| 5 | This candidate tranche | `STAGED — AWAITING CLOSURE RULING` (owner direction: "proceed to Gate 5. CI is still running, which is fine.") |

## Decomposition and pointer state

Live `SOFTWARE_DECOMP.md` on this branch is **revision 0.12** (`SOW-077`;
`DEL-07-09` at envelope L; OBJ-006/OBJ-015 and PKG-07 mappings; telemetry
77/102, `S=9, M=69, L=24, XL=0`; `DEC-094`). Both pointers are untouched:
`_Decomposition/_LATEST.md` still cites revision 0.11 and
`_ScopeChange/_LATEST.md` still points to `SCA-008_2026-07-27_2301/` —
the expected mid-amendment posture under the pointer-last rule. The
pointer advance and the final `CLOSED_FOR_SCOPE_CHANGE_ONLY` state happen
only in the post-ruling closing commit. SCA-008 and all prior snapshots
remain byte-identical history.

## State fields

**DecompositionTruthState:** `AMENDED_PENDING_CLOSURE_RULING` (applied
bytes proven equal to the Gate-3 approved postimages; deterministic
validation recorded in `Validation_Record.md`; owner acceptance
outstanding)

**DerivativePackageState:** `STALE_PENDING_DOWNSTREAM_RERUNS` (see table
below)

**ContentRemediationState:** `NOT_APPLICABLE`

**DownstreamRerunState:** `NONE_RUN — OBLIGATIONS RECORDED`
(`Propagation_Plan.md`)

**MetadataAlignmentState:** `NOT_APPLICABLE`

**AuditState:** `SYNTHESIZED_BASELINES_ONLY` — deterministic pre/post
coverage computed from the actual files
(`Pre_Change_Coverage.json` / `Post_Change_Coverage.json`); the formal
AUDIT_DECOMP pre/post comparison is a recorded downstream obligation, not
run by SCA-009

**ReadyForNextPhase:** `NO — AWAITING GATE-5 CLOSURE RULING`

**ClosureVerdict:** `PENDING OWNER RULING`

**NextOwner:** `Ryan Tufts`

**NextAction:** rule Gate-5 closure on this candidate (snapshot contents
per the `RUN_SUMMARY.md` Gate-5 hash table; validation evidence in
`Validation_Record.md`). On acceptance, the closing commit advances
`_Decomposition/_LATEST.md` then `_ScopeChange/_LATEST.md`
(pointer-last), transcribes the ruling into `ACCEPTANCE_RECORD.md`, and
sets `CLOSED_FOR_SCOPE_CHANGE_ONLY`.

## Derivative-package status

| Derivative package / consumer | Owner instrument | Status | Required action |
|---|---|---|---|
| Formal pre/post `AUDIT_DECOMP` comparison | AUDIT_DECOMP under SCOPE_CHANGE / owning loop | `RECOMPUTE REQUIRED` | Run against the pre-application basis and the applied state; the synthesized baselines are inputs, not substitutes |
| `_DAG/DAG-008` | PROJECT_SETUP / dependency-extract workflow | `STALE_REBUILD_REQUIRED` | Rebuild (topology change: +1 DEL, +1 SOW; proposed DEL-07-09 edges) |
| Deliverable-corpus concordance rows | RECONCILIATION | `STALE_REBUILD_REQUIRED` (targeted) | Current-authority refresh: DEL-07-09 (new), DEL-07-03 (re-pointed, supersession binding), boundary-adjacent DEL-07-01/02, DEL-16-01 |
| `Dependencies.csv` for DEL-07-09 | `dependency-extract` via TASK | `STALE_REBUILD_REQUIRED` (targeted) | Extract after PREPARATION scaffold |
| DEL-07-09 folder scaffold | PREPARATION | `REQUIRED` | Scaffold post-closure |
| Estimate / schedule surfaces | estimate owner / PROJECT_SETUP | `ADVISORY_STALE` | Revision-basis revalidation only |
| Piping dev-loop coordination | receiving loop | `NOTICE ROUTED` | `NOTICE_2026-08-20_PIPING_SCA-009_DEL-07-09_VOCABULARY_PALETTE.md`; adoption under the loop's own instruments; acknowledgment tracked, not gating |

No derivative package may be represented as authoritative decomposition
truth.

## Boundary statements

No professional-reliance, certification, sealing, authentication, or
code-compliance claim is created. No lifecycle value changed (DEL-07-03
remains `IN_PROGRESS`). No estimate, schedule, release, implementation,
or dependency-edge effect on any existing deliverable. Standard claim
fence applies (F-PIP-2; DEC-081 claims taxonomy).
