# Coordination Record

## Active Surface

Keep this surface lean. Use canonical project instructions, approved DAG
artifacts, and deliverable-local records as the active state.

Primary pointers:

- `docs/DIRECTIVE.md`
- `docs/CONTRACT.md`
- `docs/TYPES.md`
- `docs/SPEC.md`
- `docs/IP_AND_DATA_BOUNDARY.md`
- `docs/VALIDATION_STRATEGY.md`
- `docs/BUILD_AND_RELEASE.md`
- `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`
- `docs/_Registers/*.csv`
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
- `execution/PKG-*/1_Working/DEL-*/MEMORY.md`
- `execution/PKG-*/1_Working/DEL-*/_STATUS.md`
- `tools/coordination/list_deliverable_status.py`

## Authority Intake Tiers

Use progressive intake. Do not turn every session into a full manual reread of
the repository, but do ground each session in the governing documents before
selecting or executing work.

Baseline intake for every new session:

- `docs/DIRECTIVE.md` for founding intent and stop rules.
- `docs/CONTRACT.md` for invariant authority.
- `docs/TYPES.md` for lifecycle states, identifiers, artifact vocabulary, and
  domain terms.
- `docs/IP_AND_DATA_BOUNDARY.md` for public/private data and protected-content
  boundaries.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` for the current package and
  deliverable working surface.
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
dependency context. Use `DAG-006` to discover those related deliverables, then
inspect local artifacts directly.

## State Tracking Rules

Use two layers of state. Do not let handoff prose become substitute authority.

Authoritative state:

1. `execution/_Decomposition/SOFTWARE_DECOMP.md` says what must be built and why.
2. `execution/_DAG/DAG-006/` says what depends on what, using approved active
   edges only. Candidate rows remain non-gating unless explicitly promoted by a
   later human gate and graph revalidation.
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
surfaces authorized by the tranche and any explicitly selected deliverable
memory or review surfaces. Leave `NEXT_INSTANCE_PROMPT.md` stable unless the
entry protocol itself changes. Do not create a session-steering coordination
state file.

## Application Integration And Issuance Phase

The project has moved out of ordinary deliverable-by-deliverable buildout.
Current local status discovery reports all deliverables as either `CHECKING`
or `ISSUED`; `DEL-01-01` is the sole currently `ISSUED` deliverable and
remains the accepted governance baseline unless a human-approved change path
opens it.

Until all deliverables are `ISSUED`, ordinary development is app-centric:
produce and harden a working desktop application while using `CHECKING`
deliverables, `DAG-006`, and the decomposition as mature design authority.
Formal issuance remains a separate human-gated lifecycle workflow.

### Working Desktop Application Standard

For this phase, a working desktop application is a technical-preview desktop
app that:

- builds and tests locally through the root/desktop package scripts;
- loads invented preview project/model/design-knowledge data;
- exposes local-only project create/open/save controls without cloud, daemon,
  network, telemetry, or repository-default private-data writes;
- runs preview mechanics through the accepted app/core boundary and renders
  results, diagnostics, report context, and review-only agent proposals;
- shows the professional, release, protected-content, and private-data
  boundaries visibly;
- avoids protected standards content, private project data, release-readiness
  claims, professional approval claims, certification, sealing,
  authentication, or code-compliance claims.

This standard is not a release gate, legal clearance, professional reliance
record, or final `ISSUED` lifecycle decision.

## Application Integration And Issuance Loop

Use this loop as the default OpenPipeStress development driver from this point
until a later coordination update replaces it. If a human has already approved
an implementation, review, issuance, or release-readiness tranche, continue
that tranche within its write bounds. Otherwise propose exactly one next
bounded tranche.

1. **Authority intake.** Read `/Users/ryan/ai-env/projects/chirality/agents/AGENT_TASK.md`,
   the baseline governance documents above, current app intake surfaces needed
   for the selected slice, `SOFTWARE_DECOMP`, and approved `DAG-006` graph
   authority.
2. **Status and git discovery.** Run
   `python3 tools/coordination/list_deliverable_status.py --dag DAG-006 --format table --summary`
   or the same command with `--format csv` when machine-readable output is
   needed. Record `git status --short` before coordination-sensitive planning
   or execution.
3. **Default candidate selection.** For ordinary development, select an
   application-integration tranche from current app gaps, failed or
   insufficient app tests or smoke evidence, missing app-to-core seams, preview
   workflow gaps, build/package gaps, or validation evidence gaps. Do not
   require an `IN_PROGRESS` deliverable candidate.
4. **Issuance candidate selection.** For human-directed lifecycle closeout,
   select from `CHECKING` deliverables. `ISSUED` deliverables are not
   work-selection scope except through a human-approved change path.
5. **Context selection.** Start from the app/code/test surfaces for app
   integration work. Use `DAG-006/DependencyEdges.csv` and
   `DAG-006/DeliverableNodes.csv` only to discover related deliverables whose
   local artifacts need inspection. DAG rows do not replace local artifact
   inspection.
6. **Bounded execution.** After approval, dispatch canonical `TASK` workers or
   work locally inside a sealed tranche. Several workers may run concurrently
   only when file sets are disjoint and write scopes are explicit. App
   tranches may cross deliverable boundaries only when the tranche names the
   app-owned slice and its allowed write targets.
7. **Fan-in and validation.** Fan in worker results, check for scope drift, and
   run validation appropriate to the changed app slice. For app integration,
   expected evidence usually includes `npm test --workspace apps/desktop`,
   `npm run build --workspace apps/desktop`, applicable Rust/Python checks,
   and browser/smoke evidence when UI behavior changes.
8. **Handoff.** Record completed work, residual app gaps, validation state,
   and boundary review in the touched app/evidence surfaces and any explicitly
   selected deliverable memory/review files. Do not advance lifecycle state,
   make release claims, or issue deliverables unless the human explicitly
   approves that gate.

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

Human approval is required for lifecycle changes, candidate promotion, commits,
release claims, acceptance records, or any professional/code compliance claim.
Read-only verification snapshots and derivative gap registers are not release,
professional, code-compliance, or acceptance claims.
