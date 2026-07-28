# Piping SCA-008 Gate 2 — impact assessment

## State

`CANDIDATE — GATE 2 NOT ACCEPTED`

This assessment is derivative, read-only planning evidence. It changes no
PRD, decomposition, register, pointer, snapshot, deliverable, product,
runtime, lifecycle, release, or Git state.

## Revision and current basis

This is the current-basis reissue of the unaccepted Gate-2 assessment
previously identified by SHA-256
`7c98d9af77d2cef08395a7d3b7e9531311180b40b85bf2ccc1118913fcde2744`.
It is evaluated against authoritative remote
`origin/main@3babf48cfc295f29982aeebdceadb60413a8b3cd`, the merge of PR #382.
The prior frozen artifact remains unchanged as review lineage but must not be
used as the owner-acceptance target.

The reissue makes two factual corrections:

1. the unchanged 101-row context-envelope distribution is
   `S=9, M=69, L=23, XL=0`, not `S=9, M=70, L=22, XL=0`; and
2. the separately approved D-58 effective-state closeout is still an applied
   candidate awaiting CHANGE closeout, not yet a live repository file.

No Piping path changed between the prior refresh basis
`4ac8348e0c15795f33bf2192b2964ee1347aca59` and the current basis. All
authoritative Piping inputs and all three D-58 CHANGE preimages remain
byte-identical.

## Impact conclusion

SCA-008 is a narrow current-effect reconciliation. It adds one forward
decomposition decision row, advances revision 0.10 to 0.11, reconciles
source/traceability/coverage-revision language, corrects one live
deliverable-status sentence that still treats App F3 as the sole remaining
gate, creates a complete active SCA snapshot, advances two pointers, and
routes three coordination notices.

It does not add, remove, rename, retire, split, merge, or reclassify any
package, deliverable, scope item, objective, requirement, or dependency edge.
It does not change PRD R7 or make Piping a Root-runtime or App-harness client.
It preserves D-30, D-31, D-58, DEC-041, and DEC-063 as historical records.
It adopts no replacement automation mechanism.

## Action-to-impact summary

| Action | Structural impact | Direct candidate surfaces | Downstream effect |
|---|---|---|---|
| Add `DEC-091` | One forward decision-log row; no product topology change | `execution/_Decomposition/SOFTWARE_DECOMP.md` | Makes D-58's current effect visible without editing history |
| Advance 0.10 → 0.11 | Revision metadata only | `SOFTWARE_DECOMP.md`; `_Decomposition/_LATEST.md` | Revision-pinned derivative packages require revalidation |
| Reconcile source basis and traceability | Narrative/authority references only | `SOFTWARE_DECOMP.md` | D-58 becomes current-effect authority; derivative evaluation stays evidence only |
| Reconcile coverage revision | §10 `Revision` value only; all counts fixed | `SOFTWARE_DECOMP.md` | Post-change audit must reproduce unchanged counts |
| Correct present-tense deliverable metadata | One Remaining entry no longer says App F3 is the sole gate under DEC-063 | `PKG-16/.../DEL-16-04.../_STATUS.md` | Lifecycle remains `IN_PROGRESS`; paired `MEMORY.md` is read-only |
| Create complete SCA-008 snapshot | No product topology change | `_ScopeChange/SCA-008_{timestamp}/` | Replaces incomplete SCA-007 as the active snapshot without rewriting SCA-007 |
| Advance active pointers | Pointer-only | `_Decomposition/_LATEST.md`; `_ScopeChange/_LATEST.md` | Later consumers resolve revision 0.11 and complete SCA-008 |
| Route notices | Coordination only | New notices to Root, App, and Tier-0 coordination surfaces, exact paths at Gate 4 | Acknowledgment tracked but not a closure veto |

## Exact affected surfaces by package role

| Surface | Package role | Gate-2 classification | Reason |
|---|---|---|---|
| `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md` | working surface | `DIRECT_EDIT` after Gates 3–4 | DEC-091, revision/source/traceability/coverage-revision, change history |
| `projects/chirality-piping/execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-04_Agent rationale and professional-boundary controls/_STATUS.md` | working surface | `DIRECT_EDIT` after Gates 3–4 | One current Remaining clause says App F3 is the sole gate under DEC-063; D-58 makes the automation mechanism unresolved |
| sibling `MEMORY.md` for DEL-16-04 | working surface | `NO_CHANGE / REQUIRED PAIRED READ` | Memory supplies non-authoritative continuity and does not override D-58 |
| `projects/chirality-piping/execution/_Decomposition/_LATEST.md` | snapshot / handoff artifact | `DIRECT_EDIT AT GATE 5` | Point to accepted revision 0.11 only after validation |
| `projects/chirality-piping/execution/_ScopeChange/SCA-008_{timestamp}/` | snapshot / handoff artifact | `ADD AT GATE 5` | Immutable complete SCA record |
| `projects/chirality-piping/execution/_ScopeChange/_LATEST.md` | snapshot / handoff artifact | `DIRECT_EDIT AT GATE 5` | Point to complete accepted SCA-008 only |
| Root, App, and Tier-0 receiving-loop notices | snapshot / handoff artifact | `ADD ONLY IF GATE 4 AUTHORIZES EXACT PATHS` | Coordinate the Piping-owned current-effect result; grant no authority |
| pre/post `AUDIT_DECOMP` results | derived publication artifact | `RECOMPUTE` | Required Gate-5 baseline comparison and active-snapshot verification |

The DEL-16-04 metadata correction is a Gate-2 scope refinement based on the
required present-tense reliance scan. It changes neither lifecycle nor
product scope. Gate 3 must show the exact before/after sentence; Gate 4 must
authorize that exact `_STATUS.md` write after the paired MEMORY read.

## Authoritative surfaces that remain unchanged

| Surface / family | Package role | Classification | Preserved effect |
|---|---|---|---|
| `docs/PRD.md`, especially R7 and FR-AGENT-001…005 | working surface | `NO_CHANGE` | Agent-assisted product outcomes remain |
| `docs/_Registers/ScopeLedger.csv` | authoritative companion register | `NO_CHANGE` | 76 `IN` rows and all mappings remain |
| `docs/_Registers/Deliverables.csv` | authoritative companion register | `NO_CHANGE` | 101 stable deliverables remain |
| `docs/_Registers/ContextBudgetQA.csv` | authoritative companion register | `NO_CHANGE` | 101 rows; S=9, M=69, L=23, XL=0 remain |
| D-30 proposal/ruling/consumption JSON | working surface (governance input) | `NO_CHANGE / HISTORICAL` | Historical App-era act and evidence retained; no repin |
| D-31 ruling | working surface (governance input) | `NO_CHANGE / HISTORICAL` | Historical declaration retained |
| D-58 proposal/ruling and pending effective-state closeout | working surface (governance input) | `NO_CHANGE BY SCA / CURRENT AUTHORITY INPUT` | Proposal and ruling supply current-effect direction; the separately approved closeout must land before Gate 3; SCA does not rewrite any of them |
| Piping decision register | working surface (governance input) | `NO_CHANGE BY SCA` | D-58 currentness is handled by its separate approved closeout |
| DEC-041 and DEC-063 rows | working surface | `NO_CHANGE / HISTORICAL` | Exact prior rows remain byte-identical |
| D-T0-24 Piping notice | snapshot / handoff artifact | `NO_CHANGE / COORDINATION INPUT` | Confirms identity separation and non-client posture only |
| all ScopeOfWork files | working surfaces | `NO_CHANGE` | Deterministic scan found no current D-30/D-31/DEC-063 mechanism assertion requiring SCA edit |
| all other `_CONTEXT.md`, `_STATUS.md`, `_MEMORY.md`, and `_REFERENCES.md` files | working surfaces | `NO_CHANGE` | No other current exact mechanism assertion found |
| source, schemas, runtime, profiles, implementation evidence, dependencies, estimates, schedules, lifecycle, release records | outside direct amendment package or downstream truth | `NO_CHANGE` | Not authorized and not structurally implicated |

Historical `_Reconciliation` and `_ScopeChange/SCA-007...` bytes are never
edited. Their claims remain attributable to their accepted historical bases.

## Topology, count, and identifier invariants

| Invariant | Pre-change | Expected post-change | Risk |
|---|---:|---:|---|
| Scope items | 76, all `IN` | 76, all `IN` | none |
| Packages | 18 | 18 | none |
| Deliverables | 101 | 101 | none |
| Context-budget rows | 101 | 101 | none |
| Objectives | 18 | 18 | none |
| Unassigned scope items | 0 | 0 | none |
| Scope items without deliverable mappings | 0 | 0 | none |
| Unmapped objectives | 0 | 0 | none |
| Dependency edges | unchanged | unchanged | no edge edit |
| Existing stable IDs | preserved | preserved | no reuse or renumbering |
| New IDs | none beyond `DEC-091` and SCA-008 snapshot identity | same | execution-time collision scan governs |
| Open issues | 17 | 17 | automation mechanism remains unresolved under D-58/DEC-091, not silently solved |

No parent/child closure work exists because no parent, child, or lineage is
changed. Orphan-risk counts are all zero for packages, deliverables,
scope-item mappings, objectives, and context-budget rows.

## Historical-authority treatment

| Record | Treatment |
|---|---|
| `DEC-041` | Preserve exact historical substrate ruling. |
| `D-30` | Preserve proposal, ruling, and consumption JSON as historical App-era mechanism evidence. Do not repin. |
| `D-31` | Preserve exact historical declaration. |
| `DEC-063` | Preserve exact historical forward note. |
| `D-58` | Cite the accepted proposal and ruling as the current authority that retires reliance on the App-era mechanism and leaves the automation mechanism unresolved. Land the separately approved effective-state closeout before Gate 3; do not rewrite any D-58 surface through SCA-008. |
| `DEC-091` | Forward current-effect row only; must not claim repeal, replacement, client status, or a solved automation condition. |

The exact DEC-091 wording belongs to Gate 3. It must distinguish historical
validity from current reliance and must preserve R7 and non-client status.

## Present-tense reliance scan

The scan covered deliverable `ScopeOfWork.md`, `_CONTEXT.md`, `_STATUS.md`,
`MEMORY.md`, `_DEPENDENCIES.md`, `_REFERENCES.md`, and architecture-basis
files for D-30, D-31, D-APP-48, DEC-041, DEC-063, the automated pull,
App-harness consumption, and F3-only gate claims.

One current working-surface mismatch was found:

`DEL-16-04/_STATUS.md` says the Phase-I item is “gated: app-dev F3
live-binding per DEC-063 remaining gate set.” After D-58, F3 is not the sole
remaining gate because the automation-condition mechanism is unresolved.

The paired `MEMORY.md` was read. It records non-authoritative continuity and
no contrary authority. The status correction should preserve `IN_PROGRESS`,
R7, F3 as a necessary gate, the professional boundary, and all other
Remaining/History text while adding the unresolved automation-mechanism gate.

Historical reconciliation snapshots also contain the old claim. They are
derived historical evidence and remain immutable. A targeted RECONCILIATION
refresh must supersede their current-use reading after SCA-008; SCOPE_CHANGE
does not rewrite them.

## SCA-007 blocker disposition

The pre-change audit has two blockers:

1. `COV-230`: active SCA-007 lacks `Pre_Change_Coverage.json`,
   `Post_Change_Coverage.json`, and `Supersession_Map.csv`.
2. `COV-231`: active SCA-007 RUN_SUMMARY/Handoff claims closure while that
   active-snapshot contract is incomplete.

SCA-008 must not rewrite the historical SCA-007 snapshot or its closure
claims. Gate 5 resolves the *current active-state* blockers by:

- creating a complete immutable SCA-008 snapshot containing the current
  required artifact set, including pre/post coverage and a deterministic
  carried-forward or header-only supersession map;
- validating artifact completeness;
- updating `_ScopeChange/_LATEST.md` only after the snapshot passes; and
- rerunning `AUDIT_DECOMP` against that active post-change state.

Expected result: the two active-snapshot blockers disappear from the
post-change audit. SCA-007 remains immutable historical incomplete residue,
explicitly named in SCA-008 Handoff_State. If either blocker persists after
pointer advancement, Gate 5 cannot close.

## Derivative-package state and reruns

| Derivative package / consumer | Owner | Status after amendment | Required action |
|---|---|---|---|
| SCA-008 Gate-5 audit snapshot | `AUDIT_DECOMP` under SCOPE_CHANGE | `RECOMPUTE REQUIRED` | Full pre/post comparison; verify active snapshot and handoff |
| `_DAG/DAG-008` | PROJECT_SETUP / dependency-extract workflow | `STALE_REVALIDATION_REQUIRED` | Its own handoff says a decomposition revision triggers rebuild/revalidation; topology is expected unchanged but must be proven downstream |
| Deliverable-concordance rows for DEC-063 / DEL-16-04 | RECONCILIATION | `STALE_REBUILD_REQUIRED` | Targeted current-authority refresh after SCA-008; preserve historical frozen run |
| Estimate snapshots | estimate owner | `CURRENT BY NO STRUCTURAL EFFECT` | No recomputation required unless local policy keys solely on revision |
| Schedule snapshots | PROJECT_SETUP schedule owner | `CURRENT BY NO STRUCTURAL EFFECT` | No sequencing/topology change; record revision revalidation if consumed |
| ScopeOfWork corpus | WORKING_ITEMS | `CURRENT EXCEPT HELD LIVE-BINDING RELIANCE` | No SOW edit; do not release work that relies on a current embedded-agent mechanism |
| Product/runtime/source/schema/test surfaces | their owning workflows | `NO_CHANGE` | No rerun or write authorized by SCA-008 |
| Root/App/Tier-0 receiving loops | receiving loop owners | `NOTICE ROUTING REQUIRED` | Receive and disposition under their own instruments; acknowledgment tracked, not gating |

No derivative package may be represented as authoritative decomposition
truth. SCA-008 can close `CLOSED_FOR_SCOPE_CHANGE_ONLY` after authoritative
truth and its active snapshot validate, while DAG and reconciliation reruns
remain explicit downstream work.

## Estimate and schedule risk

There is no estimate or schedule magnitude change because scope membership,
deliverable granularity, dependencies, lifecycle, and sequencing are
unchanged. The only risk is stale basis labeling. Consumers that reject any
decomposition-revision mismatch must revalidate their basis; they must not
infer new effort or edges.

## Temporary drift and hold

From D-58 durability until SCA-008 Gate 5, the decomposition and DEL-16-04
status retain superseded present-tense current-effect wording. During that
interval:

- D-58 governs current reliance;
- the App-era synchronized-consumption mechanism must not be relied on;
- Piping remains a non-client;
- R7 remains in scope;
- work requiring a current embedded-agent automation mechanism stays held;
- no successor mechanism, repin, migration, or compatibility claim may be
  inferred.

If Gate 2 is accepted, Gate 3 remains closed until the separate D-58
effective-state CHANGE closeout is durably merged and the refreshed Gate-3
basis reproduces that accepted state. Gate 3 may then draft exact bytes but
may not write live surfaces.

## Rollback posture

Before Gate 5, rollback is discard-only: delete the temporary candidate
packet; no live state has changed.

After a future applied amendment but before Git closeout, restore the exact
preimages of the directly edited working surfaces and pointers and remove
only the uncommitted SCA-008 snapshot/notices. After merge, rollback requires
a new owner-directed forward SCOPE_CHANGE; historical SCA-008 bytes are not
deleted or rewritten.

Rollback never revives the retired current-reliance claim by implication.
Any restoration of that claim requires its own accepted current mechanism and
owner ruling.

## Gate-3 decision needs

Gate 3 must present and obtain approval for:

1. exact DEC-091 text and its execution-time next-free insertion;
2. exact revision-history/intake/source-basis/coverage-revision changes;
3. exact DEL-16-04 `_STATUS.md` one-clause correction with unchanged lifecycle;
4. exact statement preserving R7, non-client status, all topology, and all
   historical records;
5. exact treatment of D-58 as authority and evaluation packages as evidence;
6. exact complete SCA-008 snapshot inventory and SCA-007 historical-residue
   disclosure;
7. exact candidate receiving loops and notice paths for later Gate-4
   authorization; and
8. proof that no successor mechanism, automation-condition discharge, repin,
   implementation, lifecycle, release, or professional reliance is minted.

Any new affected surface, ID collision, changed D-58 semantics, changed R7,
client-status proposal, topology change, or unresolved active-snapshot
contract requires renewed impact review rather than silent absorption.

## Required sequencing

1. Close and merge the separately approved, exact three-path D-58
   effective-state CHANGE tranche.
2. Record the resulting merge SHA as the SCA-008 Gate-3 basis and verify the
   D-58 closeout, decision-register row, and Receipt-77 exact postimages.
3. Only then prepare the Gate-3 exact amendment candidate.

Gate-2 acceptance may be recorded before step 1 because this assessment
expressly treats the D-58 closeout as a predecessor rather than as a landed
fact. It does not waive or satisfy that predecessor.

## Gate-2 recommendation

Accept this impact assessment, including the bounded DEL-16-04 status
correction and the active-snapshot repair-by-superseding-pointer approach.
Open Gate 3 only.
