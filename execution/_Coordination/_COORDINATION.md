# Coordination Record

## Active Surface

Keep this surface lean. Use canonical project instructions, approved DAG
artifacts, and deliverable-local records as the active state.

Path anchors for executable prompts:

- Resolve `REPO_ROOT` with `git rev-parse --show-toplevel`.
- Set `WORKING_ROOT` to `{REPO_ROOT}/projects/chirality-piping`.

Primary pointers:

- `{REPO_ROOT}/AGENTS.md`
- `{REPO_ROOT}/agents/AGENT_WORKING_ITEMS.md`
- `{WORKING_ROOT}/AGENTS.md` - project-local agent posture and closeout discipline
- `docs/DIRECTIVE.md`
- `docs/CONTRACT.md`
- `docs/TYPES.md`
- `docs/SPEC.md`
- `docs/PRD.md`
- `docs/PLAN.md`
- `docs/IP_AND_DATA_BOUNDARY.md`
- `docs/VALIDATION_STRATEGY.md`
- `docs/RELEASE_QUALITY_GATES.md`
- `docs/BUILD_AND_RELEASE.md`
- `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`
- `docs/_Registers/*.csv`
- `plans/PLAN_2026-06-17_prd_completion.md`
- `execution/_Coordination/_DECISIONS/_REGISTER.md`
- `package.json`
- `apps/desktop/package.json`
- `apps/desktop/SMOKE.md`
- `apps/desktop/src/**`
- `apps/desktop/src-tauri/**`
- `core/product_physics/**`
- `fixtures/product_preview/**`
- `execution/_Decomposition/SOFTWARE_DECOMP.md`
- `execution/_DAG/_LATEST.md`
- `execution/_DAG/DAG-006/APPROVAL_RECORD.md`
- `execution/_DAG/DAG-006/DAG_Audit.md`
- `execution/_DAG/DAG-006/DependencyEdges.csv`
- `execution/_DAG/DAG-006/DeliverableNodes.csv`
- `execution/_DAG/DAG-007/` - canonical dependency type-system rectification
  package, pending human approval before pointer promotion
- `execution/PKG-*/1_Working/DEL-*/MEMORY.md`
- `execution/PKG-*/1_Working/DEL-*/_STATUS.md`
- `tools/coordination/list_deliverable_status.py`

## Authority Intake Tiers

Use progressive intake. Do not turn every session into a full manual reread of
the repository, but do ground each session in the governing documents before
selecting or executing work.

Baseline intake for every new session:

- `{REPO_ROOT}/agents/AGENT_WORKING_ITEMS.md` for the active persona.
- `{WORKING_ROOT}/AGENTS.md` for project-local agent posture, write-scope
  discipline, and closeout routing.
- `docs/DIRECTIVE.md` for founding intent and stop rules.
- `docs/CONTRACT.md` for invariant authority.
- `docs/TYPES.md` for lifecycle states, identifiers, artifact vocabulary, and
  domain terms.
- `docs/IP_AND_DATA_BOUNDARY.md` for public/private data and protected-content
  boundaries.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` for the current package and
  deliverable working surface.
- `docs/PRD.md` §10 (functional requirements) and §22 (release milestones) as
  the product completion yardstick for ordinary tranche selection.
- `docs/PLAN.md` — the non-governing strategic roadmap: the definition of
  "complete per the PRD", the current milestone position, the layer-relation
  map, and roadmap-level risks. It routes to the authorities (this record is
  authoritative for the current target stage) and to the completion plan for
  tranche selection; it is strategic orientation, not a work queue.
- `plans/PLAN_2026-06-17_prd_completion.md` — the current completion plan
  (non-governing `PROPOSAL`, supersedes the 2026-06-10 plan) — for phase
  ordering, the dependency spine, the FR completion map, and the human
  decision register `D-01..D-23` (`D-23` ruled by `DEC-048`). Update this
  pointer when a successor plan is accepted.
- `execution/_Coordination/_DECISIONS/_REGISTER.md` for decision-packet
  preparation status and rulings pending with the human project authority.
- `execution/_Coordination/_COORDINATION.md`,
  `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`, and authoritative
  discovery surfaces named here for current entry protocol and state.
- `execution/_DAG/_LATEST.md`, the current DAG approval record, and the current
  DAG node/edge registers for approved relationship context.

Baseline governance documents are the usable authority for ordinary work.
Deliverable-local folders are read for the selected deliverable and for active
DAG-discovered dependencies only. Do not read governance deliverable folders
merely to understand project-wide policy unless the selected work, local
references, or active DAG edges point there.

Execution intake when selecting or implementing a deliverable:

- `docs/SPEC.md` for technical architecture and implementation mechanics.
- Relevant rows from `docs/_Registers/*.csv`; query the rows needed for the
  selected package or deliverable instead of reading every register wholesale.
- The selected deliverable-local folder, including `_STATUS.md`, `MEMORY.md`,
  `_CONTEXT.md`, `_REFERENCES.md`, dependency files, run records, semantic or
  lensing files when present, review files when present, the four-document kit,
  and primary artifacts.
- DAG-discovered upstream and downstream deliverable-local files as needed for
  context.

Review, closeout, or release-readiness intake:

- `docs/VALIDATION_STRATEGY.md` for verification and release-quality
  expectations.
- `docs/RELEASE_QUALITY_GATES.md` for the gate definitions that
  release-readiness evidence, threshold work (`D-04`), and Phase E tranches
  must satisfy.
- Applicable deliverable-local review files, `Review_Findings.csv`, run
  records, validation evidence, source indexes, and aggregation or
  reconciliation snapshots.
- Relevant register rows and active DAG context needed to check closure,
  dependency satisfaction, and lifecycle-gate boundaries.

Application integration intake when selecting or implementing a desktop-app
slice:

- root and desktop package manifests: `package.json`,
  `apps/desktop/package.json`;
- desktop UI/runtime surfaces: `apps/desktop/src/**`,
  `apps/desktop/src-tauri/**`, and `apps/desktop/SMOKE.md`;
- current mechanics bridge and preview data:
  `core/product_physics/**` and `fixtures/product_preview/**`;
- relevant `core/**` modules for the selected app slice;
- `docs/BUILD_AND_RELEASE.md` and `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` for
  build, release-evidence, and contributor workflow context.

Read deliverable-local packets only when the selected app slice needs their
authority, acceptance criteria, unresolved TBDs, review findings, or
dependency context. Use the current approved DAG pointer to discover those
related deliverables, then inspect local artifacts directly. As of 2026-06-16,
`DAG-006` remains the approved pointer; `DAG-007` is the validated canonical
rectification successor pending human approval.

## State Tracking Rules

Use two layers of state. Do not let handoff prose become substitute authority.

Authoritative state:

1. `execution/_Decomposition/SOFTWARE_DECOMP.md` says what must be built and why.
2. `execution/_DAG/_LATEST.md` names the approved DAG pointer. As of
   2026-06-16 it still points to `DAG-006`, an immutable historical snapshot
   with legacy dependency enum values. `execution/_DAG/DAG-007/` is the
   canonical type-system rectification successor: it preserves the approved
   `DAG-006` active topology, excludes candidate rows from `DependencyEdges.csv`,
   and remains pending human approval before pointer promotion. Current and
   future dependency writes must use canonical v3.1 enum values only;
   legacy-readable values are migration inputs, not emit-able current values.
   Candidate rows remain non-gating worklist items unless explicitly promoted by
   a later human gate and graph revalidation. SCCs are the primary diagnostic of
   undecided ordering: any edge that participates in a cycle is non-gating until
   the SCC is resolved by a recorded move (decompose / invert / merge / cut;
   cut/merge are human-gated). The active edge set is kept acyclic by
   construction; unresolved cycles live only in candidate worklists pending
   resolution. Later approved DAG versions are event-driven by a decomposition
   revision / SCA or another explicit governance rectification, not periodic.
   Canonical doctrine: the shared repo-root `docs/CYCLE_DRIVEN_RESOLUTION.md`.
   This project's adoption and rollout record:
   `plans/PLAN_2026-06-13_cycle_driven_resolution_doctrine.md`.
3. Deliverable-local `_STATUS.md`, `MEMORY.md`, and `_run_records/**` carry
   lifecycle, working memory, and execution evidence inside each deliverable's
   ownership boundary.
4. `tools/coordination/list_deliverable_status.py` is a read-only discovery
   helper. It lists local `_STATUS.md` values and optional DAG node
   presence/path context; it does not write state and is not a substitute for
   deliverable-local inspection.
5. The current application implementation state lives in the repository app,
   core, fixture, test, and build surfaces. App code and test evidence do not
   replace decomposition truth, DAG authority, deliverable lifecycle state, or
   human review gates, but they are now the primary ordinary development
   surface for producing a working desktop application.
6. The completion plan (`plans/PLAN_2026-06-17_prd_completion.md`) and the
   decision register (`execution/_Coordination/_DECISIONS/_REGISTER.md`) are
   non-governing guidance surfaces: they order tranche selection toward the
   PRD yardstick and track human decision preparation. They do not replace
   decomposition truth, DAG authority, deliverable lifecycle state, or human
   gates. When they disagree with authoritative surfaces, surface the
   discrepancy and correct the plan or register, not the authority.

Handoff and state discovery:

- `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` contains stable entry
  instructions for the next agent instance.
- There is no separate handoff-state authority. Current state must be
  discovered from authoritative artifacts: `_DAG/_LATEST.md`, approved DAG
  artifacts, deliverable-local `_STATUS.md`, `MEMORY.md`, `_run_records/**`,
  dependency/review files, and current aggregation or review artifacts as
  needed.
- Handoff prose, when created inside a deliverable or review/audit artifact,
  must cite the owning authoritative artifacts. It must not replace
  decomposition truth, DAG authority, deliverable-local lifecycle files, review
  records, release records, or professional/acceptance records.

At session closeout, update affected deliverable-local `MEMORY.md` and
`_run_records/**` and any explicitly approved review/audit evidence surfaces.
For application-integration work, update only the app/code/docs/test/evidence
surfaces authorized by the tranche, the completion-plan and decision-register
rows the tranche or a recorded human ruling affects, and any explicitly
selected deliverable memory or review surfaces. After validation and evidence
recording, autonomously hand off to a `CHANGE` agent/subagent for final
Git/file-state review under `{WORKING_ROOT}/AGENTS.md` closeout discipline.
Leave `NEXT_INSTANCE_PROMPT.md` stable unless the entry protocol itself
changes. Do not create a session-steering coordination state file.

## Application Integration And Issuance Phase

The project has moved out of ordinary deliverable-by-deliverable buildout.
Do not infer lifecycle state from this coordination prose; run the local
status-discovery command and use deliverable-local `_STATUS.md` files as the
authority. The latest discovery during 2026-06-21 closeout reported
`CHECKING=8`, `IN_PROGRESS=92`, and `ISSUED=1`; `DEL-01-01` is the sole
currently `ISSUED` deliverable and remains the accepted governance baseline
unless a human-approved change path opens it.

Until all deliverables are `ISSUED`, ordinary development is app-centric:
produce a working desktop application while using the current approved DAG
pointer, the decomposition, and deliverable-local records according to their
discovered lifecycle state. As of 2026-06-16, that pointer is still `DAG-006`
pending human approval of `DAG-007`. `CHECKING` means design authority is
mature; it does not mean the application has absorbed that deliverable's
scope. `IN_PROGRESS` records remain current development surfaces and must not
be described as issued or closed. The completion plan's FR map tracks
absorption toward the PRD yardstick. Formal issuance remains a separate
human-gated lifecycle workflow.

### Working Desktop Application Standard

The application standard is staged. Every stage keeps every boundary
prohibition below; stages raise only the functional bar, never relax the
boundary.

**Stage floor — technical preview (met as of 2026-06-10).** A desktop app
that:

- builds and tests locally through the root/desktop package scripts;
- loads invented preview project/model/design-knowledge data;
- exposes local-only project create/open/save controls;
- runs preview mechanics through the accepted app/core boundary and renders
  results, diagnostics, report context, and review-only agent proposals;
- shows the professional, release, protected-content, and private-data
  boundaries visibly.

**Current target stage — PRD R4 exit criterion (advanced 2026-06-20 by
`DEC-048`, after the R3 target-stage advance in `DEC-035`).** Phase C/R3
evidence was reviewed through `plans/VERIFICATION_2026-06-20_r3_exit_chain.md`
(`TP-R3VERIFY-001`, SMOKE TP-MAC-190), and the human project authority accepted
the R3 exit review and R3-to-R4 target-stage advancement by approving `D-23`
Option O-A. The ordinary in-stage program is now Phase D: piping components and
nonlinear supports. R4 exit criteria are PRD §22.5: nonlinear support
validation cases converge, and component provenance appears in reports. D1
bend-object app absorption landed for the invented preview path across
visibility/provenance (`TP-R4-D1-BENDVIS-001`) and
mechanics/stress-recovery/report evidence (`TP-R4-D1-BENDSTRESS-001`). D2
branch-object app absorption landed for the invented preview path through
`TP-R4-D2-BRANCHSTRESS-001`. D3 rigid/semi-rigid component app absorption
landed for the invented preview path through `TP-R4-D3-RIGIDVIS-001`, with
`mechanics_geometry_only` data/provenance/diagnostic evidence and no
frame-stiffness behavior change. D4 expansion-joint app absorption landed for
the invented preview path through `TP-R4-D4-EJSTIFF-001`, with user-stiffness
macro-element review rows and no full assembled global solve or pressure-thrust
load generation claim. D5 spring-hanger scope is gated by `D-15`, whose packet
is prepared and awaiting human ruling. If that ruling is still pending, continue
D6/D9 residuals: the loop-core slice `TP-R4-D6-LOOPCORE-001` added
`core/solver/nonlinear_integration` as the dense assembled active-set loop owner
under `DEC-044`/`DEC-046`, and `TP-R4-D6-PHYSINTEG-001` bound that dense loop
into the invented product/app/result-envelope preview path for one explicit
nonlinear support. `TP-R4-D9-ASSEMBLEDSEED-001` added the first assembled
global-loop validation seed for one-way deactivation, gap closure, and lift-off
release; `TP-R4-D9-FRICTIONSEED-001` and
`TP-R4-D9-FRICTIONSLIDE-001` added explicit-normal friction sticking/sliding
assembled and product-preview evidence. `TP-R4-D8-COMPPROVREPORT-001` added
component provenance and missing-provenance warnings to the hash-bound rendered
report path. Derived friction normal-force model integration, measured
convergence values, sparse live-path adoption, broader live-solver coverage,
the branch-assembly benchmark, and the R4 exit evidence package remain open.
The remaining spine continues with D7
sparse live-path timing gate (`D-17`) and broader D9 R4 validation evidence.
`DAG-007` remains pending human approval; `_DAG/_LATEST.md` stays on
`DAG-006`.

Interactive authoring of user-created local models — applying editor
operations to a persisted local model document — remains in scope and does not
violate the technical-preview boundary. The boundary prohibitions concern
claims, network surfaces, and data classes, not interactivity:

- no cloud, daemon, network, telemetry, or repository-default private-data
  writes;
- bundled examples and fixtures remain invented; user-created models stay in
  local project storage and are never committed to the repository;
- no protected standards content and no private project data in the
  repository;
- no release-readiness claims, professional approval claims, certification,
  sealing, authentication, or code-compliance claims.

**Later stages.** After R4-stage evidence is recorded and human-reviewed,
subsequent target stages follow the completion plan: R5 (engineering beta).
Agents propose stage advancement with evidence; only a human-approved
coordination update advances the target stage recorded here.

This standard is not a release gate, legal clearance, professional reliance
record, or final `ISSUED` lifecycle decision.

## Application Integration And Issuance Loop

Use this loop as the default OpenPipeStress development driver from this point
until a later coordination update replaces it. If a human has already approved
an implementation, review, issuance, or release-readiness tranche, continue
that tranche within its write bounds. Otherwise propose exactly one next
bounded tranche.

1. **Authority intake.** Read `{REPO_ROOT}/agents/AGENT_TASK.md`,
   `{REPO_ROOT}/agents/AGENT_WORKING_ITEMS.md`, `{WORKING_ROOT}/AGENTS.md`,
   the baseline governance documents above, current app intake surfaces needed
   for the selected slice, `SOFTWARE_DECOMP`, and the current approved DAG
   pointer. As of 2026-06-16, `_DAG/_LATEST.md` still points to `DAG-006`;
   `DAG-007` is pending human approval.
2. **Status and git discovery.** Run
   `python3 tools/coordination/list_deliverable_status.py --dag DAG-006 --format table --summary`
   or the same command with `--format csv` when machine-readable output is
   needed, until `_DAG/_LATEST.md` is human-approved to point elsewhere. Record
   `git status --short` before coordination-sensitive planning or execution.
3. **Default candidate selection.** For ordinary development, select the
   highest-leverage application-integration tranche in this order:
   1. the earliest unblocked item on the completion plan's dependency spine
      for the current target stage (R4/Phase D per `DEC-048`). C1-C4 and C5
      are landed, and the R3 exit evidence was accepted through `D-23`.
      Select Phase D/R4 work in plan order. D1 bend-object app absorption
      landed for the invented preview path through `TP-R4-D1-BENDVIS-001` and
      `TP-R4-D1-BENDSTRESS-001`; D2 branch-object app absorption landed through
      `TP-R4-D2-BRANCHSTRESS-001`; D3 rigid/semi-rigid component app
      absorption landed through `TP-R4-D3-RIGIDVIS-001`; D4 expansion-joint
      app absorption landed through `TP-R4-D4-EJSTIFF-001`. D5 is gated by
      `D-15`; the D-15 decision packet is prepared and awaiting human ruling.
      If that ruling is still pending, continue D6/D9 residuals after
      `TP-R4-D6-LOOPCORE-001`, `TP-R4-D6-PHYSINTEG-001`,
      `TP-R4-D6-LIVECOVER-001`, `TP-R4-D9-FRICTIONSEED-001`,
      `TP-R4-D9-FRICTIONSLIDE-001`, and the D8 rendered report-provenance
      slice `TP-R4-D8-COMPPROVREPORT-001`: derived friction normal-force model
      integration, measured convergence values, sparse live-path adoption,
      broader live-solver coverage, and D9 validation cases beyond the current
      assembled/product seeds remain open under `DEC-044`/`DEC-046`. D7 is gated by
      `D-17`; `D-20` remains a Phase E lead-up decision; held `D-21` does not
      authorize v0.2/R6/R7 scope;
   2. regression repair: failed or insufficient app tests, build breaks, or
      smoke-evidence gaps in already-landed surfaces;
   3. residual hardening (missing app-to-core seams, preview workflow gaps,
      build/package gaps, validation evidence gaps) when it blocks or
      de-risks a current-stage plan item.
   Do not require an `IN_PROGRESS` deliverable candidate. Do not select
   hardening work ahead of an unblocked current-stage plan item without
   recording why in the tranche proposal.
4. **Decision escalation.** When the next plan item is blocked by a human
   decision (`D-01..D-23` in the completion plan, or any newly discovered
   human-gated TBD):
   - if no decision packet exists, the tranche is a decision-preparation
     tranche: assemble options, evidence, impacts, and a recommendation
     labeled `PROPOSAL` into
     `execution/_Coordination/_DECISIONS/D-XX_<slug>.md` and update
     `execution/_Coordination/_DECISIONS/_REGISTER.md`;
   - if a packet already awaits ruling, proceed to the next unblocked
     implementation item;
   - if no current-stage plan item remains unblocked, stop: do not
     substitute out-of-stage scope or unrelated hardening work. Finish any
     validated work in hand, route it through autonomous `CHANGE` closeout,
     ensure every blocking decision has a prepared packet, end the session
     with the pending-rulings summary, and await the human ruling. A
     continuous or unsupervised loop halts here by design and resumes only
     after a human records the ruling(s);
   - agents never resolve these decisions; rulings are human records,
     captured per existing `DEC`/`SCA` decision practice and reflected back
     into the register and plan;
   - every session summary lists the decision packets awaiting human ruling.
5. **Issuance candidate selection.** For human-directed lifecycle closeout,
   select from `CHECKING` deliverables. `ISSUED` deliverables are not
   work-selection scope except through a human-approved change path.
6. **Context selection.** Start from the app/code/test surfaces for app
   integration work. Use the current approved DAG pointer only to discover
   related deliverables whose local artifacts need inspection. As of
   2026-06-16, that pointer is `DAG-006`; `DAG-007` is pending human approval.
   DAG rows do not replace local artifact inspection.
7. **Bounded execution.** After approval, dispatch canonical `TASK` workers or
   work locally inside a sealed tranche. Several workers may run concurrently
   only when file sets are disjoint and write scopes are explicit. App
   tranches may cross deliverable boundaries only when the tranche names the
   app-owned slice and its allowed write targets.
8. **Fan-in and validation.** Fan in worker results, check for scope drift, and
   run validation appropriate to the changed app slice. For app integration,
   expected evidence usually includes `npm test --workspace apps/desktop`,
   `npm run build --workspace apps/desktop`, applicable Rust/Python checks,
   and browser/smoke evidence when UI behavior changes.
   UI evidence posture: when a tranche changes user-visible desktop
   behavior, the default evidence is an extension of the Playwright e2e
   spec(s) exercising the changed behavior in a real browser; a manual
   live-browser smoke note in `apps/desktop/SMOKE.md` without a spec
   extension is the exception and must record why automation was not
   extended. New React components land with unit tests (Vitest) at or above
   the slice's existing coverage pattern; a component with no unit test is a
   recorded evidence gap, not a silent omission. Homogeneous UI slices
   (several near-identical forms, rows, or panels produced from one
   template) may record one template-level test plus per-instance smoke
   assertions instead of duplicating the full suite per instance, provided
   the run record names every instance covered by the template rule.
   (Inserted 2026-06-11 per the human-approved H4 amendment; draft basis
   `plans/DRAFT_2026-06-11_H4_coordination_evidence_posture.md`.)
9. **Handoff and git closeout.** Record completed work, residual app gaps,
   validation state, and boundary review in the touched app/evidence surfaces
   and any explicitly selected deliverable memory/review files. Update the
   completion-plan and decision-register rows affected by the tranche or by
   any recorded human ruling. Keep the plan a selection instrument, not a
   history: compress landed items to one line — `LANDED <date>`, tranche id,
   residual hand-offs, and pointers to the run record and
   `plans/PLAN_COMPLETION_LOG.md` — and move the narrative detail to that
   log; partially-landed items keep remaining scope in the row. Before
   pushing, run `python3 tools/release/run_evidence_sweep.py --execute` at
   the committed HEAD and commit the summary (DEC-025). Then autonomously hand
   off to a `CHANGE` agent/subagent for final Git/file-state review under
   `{WORKING_ROOT}/AGENTS.md` closeout discipline. Do not advance lifecycle
   state, make release claims, or issue deliverables unless the human
   explicitly approves that gate.

## Execution Discipline

Roles and authority:

| Role | Function | Authority |
|---|---|---|
| Human project authority | Confirms scope, resolves ambiguity, accepts deliverables and gates. | Binding decisions. |
| Type 1 `WORKING_ITEMS` persona agent | Decomposes, routes, reconciles, prepares briefs, and fans in results. | Draft/proposal authority only. |
| Type 2 `TASK` subagent or bounded worker | Executes one sealed deliverable or explicit tranche slice with bounded context. | Draft/proposal authority only. |
| Deterministic tools | Validate schemas, tests, hashes, reports, lint rules, and status discovery. | Evidence only. |

A bounded worker must:

- execute only the assigned `DEL-XX-YY` or explicitly approved tranche slice;
- not expand scope silently;
- not introduce protected public data;
- label unknowns as `TBD`;
- surface conflicts and missing inputs;
- produce tests or evidence appropriate to the deliverable type;
- update only its declared write scope and deliverable-local evidence surface;
- return a concise run summary with artifacts, evidence, warnings, and open
  issues;
- query only the relevant rows from `docs/_Registers/*.csv`;
- use the active DAG only to discover relationship context, never as a
  substitute for inspecting local deliverable artifacts;
- avoid creating project-wide readiness or blocker authority unless the human
  explicitly requests that derivative package.

Review readiness requires:

- deliverable ID and package match the decomposition;
- anticipated artifacts exist or deferrals are recorded;
- tests/lints pass where applicable;
- no suspected protected data is present;
- private data paths are respected;
- assumptions and warnings are visible;
- cross-deliverable dependencies are recorded;
- local status, memory, run records, dependency files, and review files are
  internally consistent or discrepancies are surfaced;
- no claim exceeds its warrant.

Human approval is required for lifecycle changes, candidate promotion, release
claims, acceptance records, or any professional/code compliance claim. The
human project authority has authorized `CHANGE` closeout under
`{WORKING_ROOT}/AGENTS.md` as the normal terminal step for completed,
validated app-integration tranches; this git closeout does not itself create
lifecycle approval, release readiness, professional acceptance,
certification, sealing, authentication, or code-compliance claims.
Read-only verification snapshots and derivative gap registers are not release,
professional, code-compliance, or acceptance claims.
