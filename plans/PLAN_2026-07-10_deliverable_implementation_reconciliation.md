# Deliverable–Implementation Reconciliation Plan

**Date:** 2026-07-10
**Project:** OpenPipeStress (`projects/chirality-piping`)
**Status:** PROPOSED — non-governing until selected by the human owner
**Prepared for:** a future autonomous reconciliation loop after current remedial and physical-model work reaches a stable handoff
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

The live project currently represents 101 deliverables across `PKG-01` through
`PKG-17`; the observed lifecycle distribution is 92 `IN_PROGRESS`, 8
`CHECKING`, and 1 `ISSUED`. Those values are planning observations only. The run
must re-enumerate current state from the filesystem and must not use lifecycle
labels as implementation-completeness scores.

## 2. Problem statement

Much of the deliverable corpus began as setup-stage kits. Implementation has
advanced substantially since then. The pilot found 20 active specifications
that still use “setup” or “future implementation” language while the project
contains 33 implementation crates, plus application, schema, fixture, and
validation surfaces.

`DEL-04-01` is representative: its specification and datasheet remain largely
future-oriented, while `core/solver/frame_kernel` contains a substantial kernel
and test suite and later review records recognize bounded implementation
readiness. Its lifecycle history also shows that `CHECKING` was deliberately
reversed to `IN_PROGRESS` because bounded implementation evidence did not equal
full-deliverable readiness.

The pass must therefore distinguish stale setup prose from legitimate remaining
scope, and technical implementation from release or professional reliance.

Agent instruction files and the workflows they encode are outside this
program's change scope. They require a separate, owner-intensive analysis before
any consolidation, retirement, or workflow porting is proposed. This program may
read them as frozen operating constraints, but it may not redesign them or place
their modification in the repair queue.

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
5. **No lifecycle transitions.** Status changes and issuance remain under their
   existing human gates. Recommendations may be recorded but not applied by the
   discovery run.
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
   `AGENT_WORKFLOW_HANDOFF.md` for later owner-led analysis. Product code and
   deterministic engineering tools remain in scope when they are themselves
   mapped deliverable implementation and do not alter agent-workflow semantics.

## 4. Start gate and source-state control

Begin only after the human owner identifies the concurrent remedial/physical
model work as stable enough to inspect. At run start:

1. derive `REPO_ROOT` and `WORKING_ROOT` from the checkout;
2. record the exact reviewed source state;
3. enumerate packages, deliverables, lifecycle states, crates, schemas,
   application surfaces, fixtures, and validation suites;
4. record active or overlapping work and defer affected claims;
5. verify that `DAG-007` and current decomposition/register surfaces are the
   intended planning inputs; and
6. create an immutable run folder:
   `execution/_Reconciliation/DeliverableConcordance/<RunID>/`.

Any material source change during a wave marks affected rows `STALE_INPUT`; the
wave must be rerun against a stable source state rather than patched by memory.

## 5. Project authority map

The first artifact is a function-specific authority map covering:

- owner decisions and current project coordination authority;
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

Agent instructions, matrices, and skill contracts are
`FROZEN_EXTERNAL_INPUT` in the authority map. They may constrain how the
reconciliation loop operates, but this plan does not assess whether their
workflows should be retained, merged, ported, or retired.

The map assigns each source one or more functions:

| Function | Meaning |
|---|---|
| Normative scope | Defines required behavior, exclusions, and ownership |
| Accepted engineering/product decision | Selects or constrains an implementation basis through human authority |
| Declared current state | States what exists, is partial, or remains deferred |
| Implementation evidence | Shows a live mechanism, interface, or behavior |
| Verification evidence | Unit/integration evidence for implementation behavior |
| Validation evidence | Benchmark, witness, oracle, or other suitability evidence with provenance |
| Lifecycle evidence | Records workflow state only |
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
| `ImplementationEvidence` | Exact code/schema/app locations or `NONE_FOUND` |
| `VerificationEvidence` | Tests and reviewed source state or `NONE_FOUND` |
| `ValidationEvidence` | Benchmark/witness/provenance or `NOT_APPLICABLE`/`NONE_FOUND` |
| `SourceReliability` | `VETTED`, `REVIEWED`, `UNVERIFIED`, `NOT_APPLICABLE` |
| `LifecycleState` | Copied without interpreting technical completeness |
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
- `LIFECYCLE_ONLY_MISMATCH` — status history or wording differs from evidence
  without establishing a technical gap.
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

### R0 — Calibration pilots

Calibrate on two contrasting implemented deliverables:

1. `DEL-04-01` — mature solver-kernel implementation with setup-era documents,
   lifecycle reversals, and validation distinctions; and
2. `DEL-10-05` — a specification that already separates a bounded implemented
   CLI slice from deferred integration and release scope.

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
- implementation surfaces with no obvious deliverable owner.

Exclude build outputs, `target/`, packaged binaries, caches, and local databases
from source inventory. They may support a specifically identified evidence check
but never establish current scope.

### R2 — Package waves

Derive exact wave ordering from the live decomposition and `DAG-007`. The default
dependency grouping is:

1. `PKG-01` through `PKG-03` — governance, canonical model, units, components,
   materials, and library foundations;
2. `PKG-04` and `PKG-05` — solver core, loads, load cases, and stress recovery;
3. `PKG-06` through `PKG-08` — rules, GUI/workflow, reporting, audit, and
   reproducibility;
4. `PKG-09` through `PKG-12` — validation, build/interoperability,
   documentation, and security/privacy; and
5. `PKG-13` through `PKG-17` — design knowledge, model states/comparison,
   handoff, operations/proposals, and export interoperability.

Claims affected by concurrent physical-model mechanics work remain deferred
until that work has a stable source state. Independent packages may continue.

Each wave produces a package concordance, an engineering-authority queue, an
unmapped-implementation list, conflicts/unknowns, and proposed repair items. It
does not edit deliverables or product code.
`DEFERRED_AGENT_WORKFLOW` rows go only to the separate handoff and never enter
this program's repair queue.

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

Separate the decision queues:

1. **Owner/scope:** adopt, retire, remap, or defer scope.
2. **Engineering:** formulation, equation/source basis, tolerance, benchmark,
   or suitability decisions.
3. **Product/interface:** public API, file format, workflow, or compatibility
   choices.
4. **Review/lifecycle:** evidence sufficiency and recommended status action.

Each packet states options, evidence, provenance/reliability, affected claims,
downstream impact, and the recommended minimal-change disposition. An unresolved
engineering decision must never be converted into a documentation-only closure.

Agent-workflow questions are not owner/scope packets under this plan. Preserve
the observed conflict or dependency without recommending consolidation,
retirement, or a porting strategy; the separate owner-led program owns that
interpretation.

### R5 — Authorized repair tranches

After decisions are recorded through their owning authority, execute separate
bounded tranches to:

- rewrite stale setup-stage specifications into current implemented slice plus
  explicit residual scope;
- update datasheets, guidance, and procedures to current interfaces;
- preserve and annotate historical reviews rather than overwriting them;
- refresh evidence pointers and reviewed source-state identifiers;
- correct remaining-work and TBD declarations;
- update dependency rows through the canonical v3.1/DAG-007 process when
  separately authorized; and
- create implementation or validation work only for residuals that survive the
  reconciliation.

Mechanics implementation, validation work, document reconciliation, dependency
repair, and lifecycle changes should normally remain distinct tranches.
R5 must not edit agent instructions, agent indexes or matrices, skill contracts,
or agent authority/workflow semantics. Split any mixed repair so its independent
deliverable portion can proceed while its agent-workflow portion remains in the
separate handoff.

### R6 — Backcheck and closeout

Re-extract all changed claims and verify:

- current deliverable wording matches accepted scope and the implemented slice;
- verification and validation are not conflated;
- every technical source has an explicit reliability state;
- remaining work is current, owned, and bounded;
- unmapped implementation is resolved or explicitly retained as such;
- no legacy DAG label was re-emitted as current authority;
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
- `ENGINEERING_AUTHORITY_QUEUE.md`
- `OWNER_DECISION_QUEUE.md`
- `CONFLICTS_AND_UNKNOWNS.csv`
- `AGENT_WORKFLOW_HANDOFF.md`
- `AUTHORIZED_REPAIR_QUEUE.csv`
- `COVERAGE_AND_QA.md`
- `RUN_SUMMARY.md`

`execution/_Reconciliation/DeliverableConcordance/_LATEST.md` may point to the
latest completed run. It must not replace or mutate a completed run folder.

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
- every material crate, schema, binary, app surface, and validation suite is
  mapped or listed as unmapped;
- every agent-file or agent-workflow implication is classified
  `DEFERRED_AGENT_WORKFLOW`, recorded without a redesign recommendation, and
  excluded from the authorized repair queue;
- every conflict, unknown, and engineering question has an owner and smallest
  next action;
- package summaries reproduce from the claim ledger; and
- review samples include all safety/professional-boundary claims, all mechanics
  claims, all high-impact public interfaces, and a representative sample of the
  remainder.

Repair is complete only when authorized updates are backchecked and the current
deliverables alone can regenerate the residual worklist without depending on
obsolete plans or unbounded historical run records.

## 11. Autonomous-loop rules

Each autonomous iteration selects one bounded package wave or repair tranche,
re-derives its inputs from the live tree, executes within an explicit write
scope, validates its artifact contract, and records closeout before selecting
the next item. It must stop or defer a claim when:

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
