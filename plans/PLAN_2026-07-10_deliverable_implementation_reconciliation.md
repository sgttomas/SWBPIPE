# Deliverable–Implementation Reconciliation Plan

**Date:** 2026-07-10
**Project:** OpenPipeStress (`projects/chirality-piping`)
**Status:** DESIGN PROPOSAL — historical/non-governing method record; never a work-selection surface
**Revised:** 2026-07-10 after the owner-adopted deliverable-driven loop consolidation; 2026-07-11 per owner-session rulings (lifecycle rebaseline, suspension start-gate, activation mechanics, run-state model, shared-kernel extraction)
**Prepared for:** owner-authorized reconciliation executed through the current standing loop after active development reaches a stable handoff
**Write boundary:** `projects/chirality-piping/**` only

## 1. Purpose

Reconcile OpenPipeStress deliverable scope and current-state declarations with
the implemented crates, schemas, desktop application, fixtures, validation
assets, tests, and accepted decisions. The result must make each deliverable a
reliable source for:

1. accepted scope and exclusions;
2. implemented mechanics or product behavior;
3. applicable verification and validation evidence;
4. explicit residual work and bounded deferrals; and
5. lifecycle state without confusing it with technical completeness.

The live project currently represents 101 deliverables across `PKG-00` through
`PKG-17`; the observed lifecycle distribution is 92 `IN_PROGRESS`, 8
`CHECKING`, and 1 `ISSUED`. The 2026-07-11 lifecycle rebaseline ruling (§3)
moves the 8 `CHECKING` deliverables to `IN_PROGRESS` pending transcription, so
the working census is 92 `IN_PROGRESS` / 8 `CHECKING`→`IN_PROGRESS` pending
transcription / 1 `ISSUED`. Those values are planning observations only —
re-enumerate live. The run must re-enumerate current state from the filesystem
and must not use lifecycle labels as implementation-completeness scores.

In lifecycle terms (§3), this program's purpose is that concordance is the
process that makes each deliverable's `## Remaining` section warranted — and,
under the checking-candidacy trigger, the path back to `CHECKING`.

## 2. Problem statement

Much of the deliverable corpus began as setup-stage kits. Implementation has
advanced substantially since then. The pilot found 20 active specifications
that still use “setup” or “future implementation” language (unverified pilot
observation — re-derive at R0) while the project contains 33 implementation
crates, plus application, schema, fixture, and validation surfaces.

`DEL-04-01` is representative: its specification and datasheet remain largely
future-oriented, while `core/solver/frame_kernel` contains a substantial kernel
and test suite and later review records recognize bounded implementation
readiness (unverified pilot observation — re-derive at R0). Its lifecycle
history also shows that `CHECKING` was deliberately reversed to `IN_PROGRESS`
because bounded implementation evidence did not equal full-deliverable
readiness; the reversal fact is recorded in
`execution/_Reconciliation/LifecycleCorrection/LIFECYCLE_CORRECTION_2026-05-11_2052/Lifecycle_Correction_Register.csv`.
Under the §3 lifecycle model that history reads cleanly: a failed check exits
by reversal, and its findings become `## Remaining` items.

The pass must therefore distinguish stale setup prose from legitimate remaining
scope, and technical implementation from release or professional reliance.

Agent instruction files and the workflows they encode are outside this
program's change scope. They require a separate, owner-intensive analysis before
any consolidation, retirement, or workflow porting is proposed. This program may
read them as frozen operating constraints, but it may not redesign them or place
their modification in proposed deliverable updates.

This file lives under `plans/` as method and provenance only. It cannot activate
itself and must never be treated as a queue. Execution requires an explicit owner
direction or ruling, followed by bounded reconciliation work recorded in the
affected deliverables' `_STATUS.md` `## Remaining` sections. The newest
`loop/WORKPLAN_*.md` governs execution; this proposal supplies only the
reconciliation method.

## 3. Non-negotiable boundaries

1. **Implementation is evidence, not scope authority.** Do not absorb an
   experimental or adjacent implementation into a deliverable without an
   accepted scope basis.
2. **No engineering adjudication by inference.** Code presence and passing tests
   do not prove that mechanics, equations, tolerances, benchmarks, or validation
   bases are professionally acceptable.
3. **Equation-source boundary.** Unreviewed PDF/OCR equation artifacts from
   `domains/piping-design` must never be used to validate solver formulations.
   Technical formula claims require maintainer-vetted sources and their review
   status must be explicit.
4. **No compliance or professional claims.** Reconciliation cannot certify code
   compliance, engineering correctness, release readiness, sealing, approval,
   or suitability for reliance.
5. **No lifecycle transitions by discovery.** Lifecycle states are change
   regimes under the model below, and every transition remains an owner act
   under its existing human gate. Discovery runs may record
   `LIFECYCLE_REASSESSMENT_REQUIRED` recommendations but never apply them. The
   one-time rebaseline transitions belong to the preceding rebaseline ruling's
   dedicated tranche (see the lifecycle model below), not to any run under this
   plan.
6. **No automatic DAG mutation.** `DAG-007` is the current canonical dependency
   type-system authority. Older DAGs remain historical inputs. Dependency repair
   requires a separate authorized tranche.
7. **No historical-plan authority.** Plans and archived runs may explain why a
   state exists but cannot override current decisions, decomposition, contracts,
   deliverable documents, or verified implementation.
8. **No cross-project edits.** Root governance, other project workspaces, and
   domain corpora remain outside the write scope unless separately authorized.
9. **Separate agent-workflow program.** Do not modify root or project-local
   `AGENTS.md`, `agents/AGENT_*.md`, agent matrices, agent authority contracts,
   or skill contracts under this plan. Do not infer that a role or workflow is
   unnecessary merely because its active use is not visible in the sampled
   deliverable corpus. Findings that depend on agent-instruction or workflow
   changes are copied without redesign recommendations to
   `AGENT_WORKFLOW_OBSERVATIONS.md` for later owner-led analysis. Product code and
   deterministic engineering tools remain in scope when they are themselves
   mapped deliverable implementation and do not alter agent-workflow semantics.
10. **Single work surface.** This proposal, its evidence bundles, and its
    findings never select work. Owner and engineering decisions remain in the
    existing decision register; executable residuals and authorized repairs live
    only in the owning deliverable's `_STATUS.md` `## Remaining`. Existing
    `(gated: ...)` and `(stage-gated: ...)` suffixes are preserved unless their
    owning authority changes them.

### Lifecycle model — change regimes, not maturity grades

The governing lifecycle model was settled by the owner in-session on
2026-07-11: lifecycle states are change regimes, not maturity grades.

- `IN_PROGRESS` — ordinary edits are permitted.
- `CHECKING` — a frozen candidate under declared-basis review. Exit for change
  is reversal-only; this project's `execution/_Reconciliation/LifecycleCorrection/`
  DEL-04-01 reversals are the live precedent. Review evidence appends to run
  and review records, never to the frozen claim surfaces.
- `ISSUED` — changes flow only through the governed scope-change process.

CHECKING candidacy has one trigger: a warranted-empty `## Remaining` — empty
AND carrying a current evidence basis (a concordance pass or equivalent, bound
to the candidate source state) certifying that the emptiness is warranted. The
entry act is an owner declaration of the checking basis followed by the freeze.
There are no disclosed-deferral carve-outs: any warranted `## Remaining` item —
owner-gated included — keeps the deliverable `IN_PROGRESS`; rescope through the
decision register before freezing, never carve out during review. A failed
check exits by reversal, and its findings become `## Remaining` items.

In these terms, this program's purpose is exact: concordance is the process
that makes `## Remaining` sections warranted — under the trigger above, the
path back to `CHECKING`.

**One-time lifecycle rebaseline (preceding separate owner ruling — not this
plan's work).** Ruled in-session on 2026-07-11 via decision slate: the 8
`PKG-00` architecture deliverables — this project's only `CHECKING` states —
move `CHECKING` → `IN_PROGRESS`. The recorded PKG-00 lock review
(`execution/_Reconciliation/PKG00LockReview/PKG00_LOCK_REVIEW_2026-05-11_2218/`)
is preserved as historical evidence and as the natural declared checking basis
when those deliverables re-enter `CHECKING` under the new trigger. `DEL-01-01`
— the sole `ISSUED` deliverable, the governance/license baseline — stays
`ISSUED` and change-managed. This is an administrative semantics correction,
not an invalidation of prior work. Transcription into a `D-XX` register row
plus ruling record, and the 8 `_STATUS.md` transitions, are executed by a
dedicated rebaseline tranche before discovery begins.

## 4. Activation, start gate, and source-state control

### Activation mechanics

Activation is a recorded governance act, never an inference from this plan's
existence.

1. **Vehicle.** Activation is proposed as a new `D-XX` row (next free ID) in
   `execution/_Coordination/_DECISIONS/_REGISTER.md` with a PROPOSAL packet.
   On ruling, codification follows the project's `DEC-XXX` convention in
   `execution/_Decomposition/SOFTWARE_DECOMP.md` §12. The ruling names the
   activated scope: whole-corpus or a named package subset.
2. **HARD RULE — the ruling lands on `main` before any dispatch.** The ruling
   record and the register-row flip must be merged to `main` before any
   concordance session is dispatched. Concurrent sessions are mutually blind;
   on 2026-07-10 the sibling project suffered a real governance collision —
   contradictory owner-attributed decision states written by parallel
   sessions, reconciled only after the fact. That case study is why this rule
   is hard.
3. **Bootstrap `Remaining` items.** At packet time, each in-scope
   deliverable's `_STATUS.md` `## Remaining` is seeded with exactly:

   `- Run claim-level concordance per the reconciliation method (source: plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md §§6–8) (gated: D-XX)`

   The ruling flips the suffixes. Until it does, the item stays gated and
   nothing under this plan is selectable.
4. **Ordering.** The ruling must satisfy the start gate below, and the
   lifecycle rebaseline tranche (§3) must complete before discovery begins.

### Start gate — named ruling precondition

The start gate is a recorded act, not a prose judgment. The activation ruling
must record the owner's suspension or stable-handoff declaration with its date
and the commit SHA of the declared-stable tree. On 2026-07-11, in-session, the
owner (Ryan Tufts) stated: "I will suspend work in Chirality Piping for the
time being." The activation ruling cites this declaration (or a successor
declaration) together with the SHA at which the tree froze.

Under a suspended, frozen tree, whole-corpus discovery is viable: the
stability rationale for subset-scoping evaporates. Package waves survive for
tractability and checkpointing only (§8 R2), not as a stability control.

### Source-state control

Before any dispatch:

1. confirm the activation ruling (above) authorizes the reconciliation scope
   and creation of its immutable evidence run, and that it is on `main`;
2. confirm selected reconciliation work is recorded in the affected
   deliverables' `_STATUS.md` `## Remaining` per the bootstrap template above,
   with the gate suffix flipped by the ruling; if not, stop for the
   owner-authorized bootstrap update rather than selecting work from this plan;
3. derive `REPO_ROOT` and `WORKING_ROOT` from the checkout;
4. record the exact reviewed source state;
5. enumerate packages, deliverables, lifecycle states, present or absent
   `## Remaining` sections, crates, schemas,
   application surfaces, fixtures, and validation suites;
6. record active or overlapping work and defer affected claims;
7. verify that the live `_DAG/_LATEST.md` pointer, current decomposition, and
   register surfaces are the intended inputs; and
8. create the specifically authorized immutable run folder:
   `execution/_Reconciliation/DeliverableConcordance/<RunID>/`.

Any material source change during a wave marks affected rows `STALE_INPUT`; the
wave must be rerun against a stable source state rather than patched by memory.

## 5. Project authority map

The first artifact is a function-specific authority map covering:

- owner decisions and current project coordination authority;
- the newest `loop/WORKPLAN_*.md`, `loop/LOOP_INIT.md`, and loop receipts as
  execution protocol and handoff context, not product-scope authority;
- `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, current PRD/specification surfaces,
  and professional/data-boundary rules;
- `execution/_Decomposition/SOFTWARE_DECOMP.md` and live registers;
- `DAG-007` and canonical dependency enums;
- each deliverable's local context, four documents, status, dependencies,
  reviews, evidence, and run records;
- source under `core/`, `apps/`, and public schema locations;
- fixtures and verification/validation assets under `fixtures/` and
  `validation/`; and
- archival, superseded, or explanatory-only records.

Agent instructions, matrices, skill contracts, and agent-workflow guidance are
`FROZEN_PROCESS_INPUT` in the authority map. They may constrain execution, but
this plan does not assess whether their workflows should be retained, merged,
ported, or retired. If an older workflow guide conflicts with the newest loop
workplan, record the stale process-document fact for the separate owner-led
program; do not repair it here.

The map assigns each source one or more functions:

| Function | Meaning |
|---|---|
| Normative scope | Defines required behavior, exclusions, and ownership |
| Accepted engineering/product decision | Selects or constrains an implementation basis through human authority |
| Declared current state | States what exists, is partial, or remains deferred |
| Recorded remaining work | `_STATUS.md` `## Remaining`; the sole current work-discovery surface, with source and gate suffixes; absent means no recorded item beyond specification fallback |
| Implementation evidence | Shows a live mechanism, interface, or behavior |
| Verification evidence | Unit/integration evidence for implementation behavior |
| Validation evidence | Benchmark, witness, oracle, or other suitability evidence with provenance |
| Lifecycle evidence | Records workflow state only |
| Execution protocol | Constrains how selected work runs; does not create product scope or select work |
| Historical context | Explains provenance without overriding current authority |

Conflicting live normative sources receive `AUTHORITY_CONFLICT`; agents must not
invent precedence.

## 6. Claim-level concordance contract

Audit requirements and stable scope claims individually. Also inventory material
implementation surfaces that have no deliverable mapping.

Required ledger columns:

| Column | Contract |
|---|---|
| `ClaimID` | Requirement ID or run-local `UNMAPPED-*` identifier |
| `PackageID` / `DeliverableID` | Current owner or `UNMAPPED` |
| `ClaimClass` | `GOVERNANCE`, `SCHEMA`, `MECHANICS`, `WORKFLOW`, `GUI`, `REPORTING`, `INTEROP`, `VALIDATION`, `SECURITY`, `DOCUMENTATION` |
| `ClaimType` | `REQUIREMENT`, `ACCEPTANCE`, `EXCLUSION`, `IMPLEMENTED_UNMAPPED`, `REMAINING_WORK` |
| `NormativeSource` | Exact current authority location |
| `DecisionBasis` | Applicable accepted decision or `NONE_FOUND` |
| `DeclaredState` | Current deliverable statement |
| `RecordedRemaining` | Exact current `_STATUS.md` residual text or `NONE_RECORDED` |
| `RemainingSource` | Source named by the residual or `NONE_RECORDED` |
| `GateOrStageConstraint` | Exact gate/stage suffix or `UNGATED`/`NONE_RECORDED` |
| `SelectableUnderCurrentLoop` | `YES`, `NO`, or `UNKNOWN`, derived from the current DAG/lifecycle/gate rules |
| `ImplementationEvidence` | Exact code/schema/app locations or `NONE_FOUND` |
| `VerificationEvidence` | Tests and reviewed source state or `NONE_FOUND` |
| `ValidationEvidence` | Benchmark/witness/provenance or `NOT_APPLICABLE`/`NONE_FOUND` |
| `SourceReliability` | `VETTED`, `REVIEWED`, `UNVERIFIED`, `NOT_APPLICABLE` |
| `LifecycleState` | Copied as a change regime under the §3 lifecycle model; never read as a maturity grade or technical-completeness score |
| `Disposition` | Controlled value from §7 |
| `Confidence` | `HIGH`, `MEDIUM`, `LOW` |
| `RemainingWork` | Specific residual, `NONE_OBSERVED`, or `UNKNOWN` |
| `AuthorityNeeded` | `NO`, `OWNER`, `ENGINEERING`, `SCOPE_CHANGE`, `REVIEW`, or named decision ID |

Behavioral alignment requires implementation and reproducible verification.
Mechanics claims additionally require appropriate validation or an explicit
statement that validation remains open. A unit test alone must not be promoted
to engineering validation.

## 7. Controlled dispositions

- `ALIGNED` — scope, declaration, implementation, and applicable evidence agree.
- `IMPLEMENTED_UNDOCUMENTED` — material implementation lacks accepted mapping.
- `DOCUMENTED_UNIMPLEMENTED` — current requirement lacks adequate implementation.
- `PARTIALLY_IMPLEMENTED` — only a bounded portion exists.
- `IMPLEMENTED_DIFFERENTLY` — behavior materially differs from current wording.
- `STALE_SETUP_SPECIFICATION` — setup/future language no longer describes the
  accepted implemented slice.
- `STALE_REVIEW_OR_EVIDENCE` — cited review or evidence has been overtaken or
  does not bind the reviewed source state.
- `VERIFIED_NOT_VALIDATED` — behavior is implemented and tested but lacks the
  validation basis required for its claim class.
- `ACCEPTED_DIVERGENCE` — a human decision permits a bounded transitional state,
  limitation, or deferral.
- `LIFECYCLE_REASSESSMENT_REQUIRED` — status history or wording differs from
  evidence without establishing a technical gap; the residue is a lifecycle
  reassessment under the §3 change-regime model, owed to its human gate, not a
  document repair.
- `REMAINING_STATE_MISMATCH` — a landed or ruled-shut item remains recorded, an
  evidence-backed residual is omitted, or ownership/source/gate metadata no
  longer agrees with current authority and implementation.
- `DEFERRED_AGENT_WORKFLOW` — resolution depends on an agent instruction, skill
  contract, authority allocation, or workflow reserved for the separate
  owner-led program.
- `AUTHORITY_CONFLICT` — current normative or decision sources conflict.
- `ENGINEERING_AUTHORITY_REQUIRED` — evidence cannot settle a technical
  formulation, tolerance, validation, or professional-practice question.
- `UNKNOWN` — insufficient evidence; record the smallest next check.

Deliverable summaries must be computed from claim rows and must preserve mixed
dispositions. Do not label a whole deliverable “complete” because a crate exists.

## 8. Execution phases

### Run-level phase execution and program-state homes

R1, R3, and R6 execute directly under the activation ruling as ruled-program
work. `F-PIP-5` forbids new surfaces "without an owner ruling" — the
activation ruling is that ruling; the precedent is this project's
deliverable-less `DepClosure`, `Reviews`, and `LifecycleCorrection` runs under
`execution/_Reconciliation/`. No control deliverable is created for the
program.

The program has exactly three kinds of state, and each already has a home:

1. **One run's phase state** — the immutable run folder, via append-only
   `RUN_BASIS.md` and `RUN_SUMMARY.md` (§9).
2. **Cross-session open/closed visibility** — the activation register row's
   ruling-record cell, checked every loop Step 0.
3. **The recurring process asset** — checking-entry profiles and maturity
   feedback live on a ruled docs profile surface (the sibling project's
   `docs/ISSUE_READINESS_PROFILES.md` under its `D-APP-34` pattern is the
   model; this project selects its own location at activation).

No new standing pointer, register, or status surface exists beyond these (§9).

### R0 — Calibration pilots

After activation is recorded on the live work surface, calibrate on three
contrasting implemented deliverables if the owner-authorized scope includes
them; this plan does not make them selectable:

1. `DEL-04-01` — mature solver-kernel implementation with setup-era documents,
   lifecycle reversals, and validation distinctions;
2. `DEL-10-05` — a specification that already separates a bounded implemented
   CLI slice from deferred integration and release scope; and
3. `DEL-12-02` — current `## Remaining` accurately distinguishes a landed
   application-side redaction binding from broader residual integration while
   older deliverable prose still describes runtime integration as absent.

A reviewer evaluates authority precedence, technical-evidence boundaries,
disposition consistency, and false positives. R0 changes only run artifacts.

### R1 — Read-only project inventory

Build indexes for:

- packages, deliverables, requirement IDs, and lifecycle states;
- crates, binaries, schemas, desktop surfaces, public APIs, fixtures, and tools;
- unit and integration tests;
- hand calculations, mechanics/stress/nonlinear benchmarks, external-oracle
  records, and witness artifacts;
- decisions and their affected deliverables;
- reviews, findings, dispositions, and source-state identifiers; and
- present/absent `## Remaining` items with source, gate, stage, and current
  selectability;
- implementation surfaces with no obvious deliverable owner.

The implementation index includes `core/`, `apps/`, schemas, fixtures,
validation, deterministic tools and CLIs, packaging/build configuration,
project-local public-repository templates, application bindings, evidence
generators, and commit-bound validation/release records. A passing evidence
snapshot supports its evaluated source state; it does not automatically prove
current `HEAD`.

Exclude build outputs, `target/`, packaged binaries, caches, and local databases
from source inventory. They may support a specifically identified evidence check
but never establish current scope.

### R2 — Package waves

Derive exact wave ordering from the live decomposition and `DAG-007`. The default
dependency grouping is:

1. `PKG-00` through `PKG-03` — architecture runway, governance, canonical
   model, units, components, materials, and library foundations;
2. `PKG-04` and `PKG-05` — solver core, loads, load cases, and stress recovery;
3. `PKG-06` through `PKG-08` — rules, GUI/workflow, reporting, audit, and
   reproducibility;
4. `PKG-09` through `PKG-12` — validation, build/interoperability,
   documentation, and security/privacy; and
5. `PKG-13` through `PKG-17` — design knowledge, model states/comparison,
   handoff, operations/proposals, and export interoperability.

Claims affected by concurrent physical-model mechanics work remain deferred
until that work has a stable source state. Independent packages may continue.
Under the suspended, frozen tree required by the §4 start gate this deferral
clause is expected to be vacuous, and waves function as tractability and
checkpointing partitions only.

Each wave produces a package concordance, engineering/owner decision findings,
an unmapped-implementation list, conflicts/unknowns, and proposed deliverable
updates. These are non-operative evidence and do not edit deliverables or
product code.
`DEFERRED_AGENT_WORKFLOW` rows go only to the evidence-only agent-workflow
observations artifact and never enter this program's proposed deliverable updates.

### R3 — Cross-package reconciliation

Check for:

- duplicated or incompatible ownership of APIs and schemas;
- solver/load/stress interfaces whose owning deliverables disagree;
- mechanics behavior not represented in validation deliverables;
- validation claims whose fixtures do not exercise the current implementation;
- GUI, reporting, runner, or export behavior without upstream model ownership;
- setup-era TBDs already settled by decisions or code;
- settled decisions still represented as TBD in code or documents;
- stale dependencies or noncanonical legacy dependency labels;
- implementation without provenance or source-reliability disclosure; and
- remaining work contradicted by another package's current state.

No dependency registers are changed during R3.

### R4 — Human and engineering decision gates

Separate the decision findings and draft packets:

1. **Owner/scope:** adopt, retire, remap, or defer scope.
2. **Engineering:** formulation, equation/source basis, tolerance, benchmark,
   or suitability decisions.
3. **Product/interface:** public API, file format, workflow, or compatibility
   choices.
4. **Review/lifecycle:** evidence sufficiency and recommended status action.

Each packet states options, evidence, provenance/reliability, affected claims,
downstream impact, and the recommended minimal-change disposition. An unresolved
engineering decision must never be converted into a documentation-only closure.

Agent-workflow questions are not owner/scope packets under this method. Preserve
the observed conflict or dependency without recommending consolidation,
retirement, or a porting strategy; the separate owner-led program owns that
interpretation.

### R5 — Authorized repair tranches

After decisions are recorded through their owning authority and the work is
recorded in the owning deliverable's `## Remaining`, execute separate bounded
tranches to:

- rewrite stale setup-stage specifications into current implemented slice plus
  explicit residual scope;
- update datasheets, guidance, and procedures to current interfaces;
- preserve and annotate historical reviews rather than overwriting them;
- refresh evidence pointers and reviewed source-state identifiers;
- correct remaining-work and TBD declarations;
- update dependency rows through the canonical v3.1/DAG-007 process when
  separately authorized; and
- record surviving implementation or validation residuals only in the owning
  deliverable's `## Remaining`, preserving applicable source and gate metadata.

Mechanics implementation, validation work, document reconciliation, dependency
repair, and lifecycle changes should normally remain distinct tranches.
R5 must not edit agent instructions, agent indexes or matrices, skill contracts,
or agent authority/workflow semantics. Split any mixed repair so its independent
deliverable portion can proceed while its agent-workflow portion remains in the
separate evidence-only observations artifact.

Every completed repair follows the standing loop closeout contract: update the
owning `_STATUS.md` `## Remaining`, `MEMORY.md`, and `_run_records/**`; append the
minimal loop receipt required by current guidance; run the applicable validation
and DEC-025 evidence gates; hand off to CHANGE; open a PR; and never self-merge.

### R6 — Backcheck and closeout

Re-extract all changed claims and verify:

- current deliverable wording matches accepted scope and the implemented slice;
- verification and validation are not conflated;
- every technical source has an explicit reliability state;
- remaining work is current, owned, and bounded;
- unmapped implementation is resolved or explicitly retained as such;
- no legacy DAG label was re-emitted as current authority;
- every accepted residual is reflected on the owning deliverable's sole
  `## Remaining` work surface and every landed item is removed there;
- no agent-authored disposition is represented as an owner or engineering
  ruling; and
- lifecycle state was unchanged unless separately authorized.

## 9. Run artifacts

Every immutable run folder must contain:

- `RUN_BASIS.md`
- `AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md`
- `DELIVERABLE_INVENTORY.csv`
- `IMPLEMENTATION_SURFACES.csv`
- `VERIFICATION_INDEX.csv`
- `VALIDATION_AND_PROVENANCE_INDEX.csv`
- `CLAIM_CONCORDANCE.csv`
- `PACKAGE_SUMMARIES/PKG-XX.md`
- `UNMAPPED_IMPLEMENTATION.csv`
- `PROPOSED_ENGINEERING_DECISIONS.md`
- `PROPOSED_OWNER_DECISIONS.md`
- `CONFLICTS_AND_UNKNOWNS.csv`
- `AGENT_WORKFLOW_OBSERVATIONS.md`
- `PROPOSED_DELIVERABLE_UPDATES.csv`
- `COVERAGE_AND_QA.md`
- `RUN_SUMMARY.md`

These are immutable evidence artifacts, not queues or selection surfaces.
`AGENT_WORKFLOW_OBSERVATIONS.md` contains only affected product claims, exact
citations, and why reconciliation cannot resolve them; it makes no workflow
recommendation. Do not create a new `_LATEST.md`, standing pointer, register, or
status surface unless separately ruled; the ruled docs profile surface named at
activation (§8) is the one such separately ruled surface this program uses.
Continuing state is recorded in the affected deliverables and existing decision
register.

## 10. QA and acceptance criteria

Discovery is complete only when:

- 100% of live deliverables and current requirement IDs are indexed;
- every claim has an authority source or is explicitly unmapped;
- every behavioral `ALIGNED` claim has reproducible implementation and test
  evidence tied to the reviewed source state;
- every mechanics claim distinguishes verification from validation;
- every cited equation/formulation source has an acceptable review status;
- every setup-era specification is classified at claim level rather than merely
  flagged by keywords;
- every live deliverable's present or absent `## Remaining` section is checked
  against current implementation, rulings, gates, stage, and residual evidence;
- every material crate, schema, binary, app surface, and validation suite is
  mapped or listed as unmapped;
- every agent-file or agent-workflow implication is classified
  `DEFERRED_AGENT_WORKFLOW`, recorded without a redesign recommendation, and
  excluded from proposed deliverable updates;
- every conflict, unknown, and engineering question has an owner and smallest
  next action;
- package summaries reproduce from the claim ledger; and
- review samples include all safety/professional-boundary claims, all mechanics
  claims, all high-impact public interfaces, and a representative sample of the
  remainder.

Repair is complete only when authorized updates are backchecked and the current
deliverables' specifications and `## Remaining` sections correctly represent
the surviving work without depending on obsolete plans or unbounded historical
run records.

## 11. Autonomous-loop rules

Each autonomous iteration selects an eligible deliverable-local
`## Remaining` item under the newest standing workplan, never a package wave or
repair finding from this proposal. The reconciliation method governs execution
only after that selection. R1, R3, and R6 differ in mechanism, not authority:
they run as ruled-program work directly under the activation ruling (§8)
rather than through a deliverable-local item, and they change only run
artifacts and the ruled program-state homes. Each iteration re-derives its inputs, executes within
an explicit write scope, validates its artifact contract, and records
deliverable-local closeout before selection restarts. It must stop or defer a
claim when:

- an owner, scope-change, engineering, or review ruling is required;
- the relevant implementation is changing in another session;
- an equation or technical source is unverified;
- the claim requires agent-instruction, skill-contract, agent-authority, or
  workflow-porting analysis reserved for the separate owner-led program;
- verification or validation cannot be reproduced;
- authority precedence is unclear;
- a proposed action crosses the project boundary; or
- the action would imply issuance, release readiness, professional approval, or
  code compliance.

Independent work may continue around blocked claims. The loop must preserve
`UNKNOWN` and `ENGINEERING_AUTHORITY_REQUIRED` rather than manufacture closure.

## 12. Completion state

Success means that each OpenPipeStress deliverable reliably states:

- its accepted responsibility and exclusions;
- its current implemented slice;
- the verification and validation actually supporting that slice;
- its explicit limitations and residual work;
- its dependencies and ownership; and
- any human or engineering authority still required.

Completion is an evidence and documentation-coherence state. It is not issuance,
release readiness, professional approval, certification, sealing,
authentication, or a code-compliance determination.

## 13. Sibling method and shared kernel

A sibling method plan exists at
`projects/chirality-app-dev/plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md`.
The divergence between the two plans is intentional: this plan's
engineering-validation and provenance layer — `ClaimClass`,
`SourceReliability`, `VERIFIED_NOT_VALIDATED`, `ENGINEERING_AUTHORITY_REQUIRED`,
and the equation-source boundary — has no sibling counterpart, and the
sibling's stale-inspection-assessment layer has none here. Do not
resynchronize the two plans without owner direction.

A distilled shared kernel exists at repo root
`docs/DELIVERABLE_CONCORDANCE_METHOD.md` as a DRAFT canon-pattern document
(owner-directed 2026-07-11: the method is part of Chirality itself). It is
derivative and NON-BINDING until ratified after both projects' R0
calibrations. While it remains DRAFT, this plan is the self-contained
operative method for OpenPipeStress.
