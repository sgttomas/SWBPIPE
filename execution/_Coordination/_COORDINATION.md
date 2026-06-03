# Coordination Record

## Active Surface

Keep this surface lean. Use canonical project instructions, approved DAG
artifacts, and deliverable-local records as the active state.

Primary pointers:

- `AGENTS.md`
- `docs/DIRECTIVE.md`
- `docs/CONTRACT.md`
- `docs/TYPES.md`
- `docs/SPEC.md`
- `docs/IP_AND_DATA_BOUNDARY.md`
- `docs/VALIDATION_STRATEGY.md`
- `docs/_Registers/*.csv`
- `execution/_Decomposition/SOFTWARE_DECOMP.md`
- `execution/_DAG/_LATEST.md`
- `execution/_DAG/DAG-005/APPROVAL_RECORD.md`
- `execution/_DAG/DAG-005/DAG_Audit.md`
- `execution/_DAG/DAG-005/DependencyEdges.csv`
- `execution/_DAG/DAG-005/DeliverableNodes.csv`
- `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md` when present
- `execution/PKG-*/1_Working/DEL-*/MEMORY.md`
- `execution/PKG-*/1_Working/DEL-*/_STATUS.md`
- `tools/coordination/list_deliverable_status.py`

## Current Authority

`DAG-005` is approved graph authority as of 2026-05-18. Approval applies to the
active edge set only. Candidate rows remain non-gating unless a later explicit
human gate promotes them and the graph is revalidated.

The DAG is a relationship map, not the freshness surface for deliverable state
or artifact presence. For any deliverable under consideration, inspect the
deliverable-local folder before judging readiness or selecting work. The local
`_STATUS.md`, `MEMORY.md`, `_DEPENDENCIES.md` / `Dependencies.csv`,
`_run_records/**`, four-document kit, semantic/lensing files, review files when
present, and referenced code/tests are the expected discovery structure.

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
  `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`, and
  `execution/_Coordination/NEXT_INSTANCE_STATE.md` when present for current
  entry protocol and resume state.
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

## State Tracking Rules

Use two layers of state. Do not let handoff prose become substitute authority.

Authoritative state:

1. `execution/_Decomposition/SOFTWARE_DECOMP.md` says what must be built and why.
2. `execution/_DAG/DAG-005/` says what depends on what, using approved active
   edges only. Candidate rows remain non-gating unless explicitly promoted by a
   later human gate and graph revalidation.
3. Deliverable-local `_STATUS.md`, `MEMORY.md`, and `_run_records/**` carry
   lifecycle, working memory, and execution evidence inside each deliverable's
   ownership boundary.
4. `tools/coordination/list_deliverable_status.py` is a read-only discovery
   helper. It lists local `_STATUS.md` values and optional DAG node context; it
   does not write state and is not a substitute for deliverable-local
   inspection.

Handoff state:

- `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` contains stable entry
  instructions for the next agent instance.
- `execution/_Coordination/NEXT_INSTANCE_STATE.md` contains compact resume
  state: authority pointers, current program state, immediate next actions,
  residual blockers, and explicit do-not-change boundaries.
- `NEXT_INSTANCE_STATE.md` may summarize authoritative state but must cite the
  owning authoritative artifacts. It must not replace decomposition truth, DAG
  authority, deliverable-local lifecycle files, review records, release
  records, or professional/acceptance records.

At session closeout, update affected deliverable-local `MEMORY.md` and
`_run_records/**`, then update `NEXT_INSTANCE_STATE.md` with only current
pointers, completed work, residual gaps, validation state, and next action.
Leave `NEXT_INSTANCE_PROMPT.md` mostly stable unless the entry protocol itself
changes.

## Local Status And DAG-Guided Development Loop

Use this loop as the default OpenPipeStress development driver until the
`SOFTWARE_DECOMP` objectives are closed. It is project-specific, but general
across packages and deliverables. If a human has already approved an
implementation or review tranche, continue at the execution step; otherwise use
the selection steps to propose exactly one next bounded tranche.

1. **Authority intake.** Read `NEXT_INSTANCE_PROMPT.md`,
   `NEXT_INSTANCE_STATE.md` when present, this coordination record, and the
   baseline intake documents above. Add execution or review intake documents
   according to the tranche type. `SOFTWARE_DECOMP` defines what must be built
   and why. `DAG-005` defines approved active relationships.
   Deliverable-local files define current deliverable state.
2. **Status discovery.** Run
   `python3 tools/coordination/list_deliverable_status.py --dag DAG-005 --format table --summary`
   or the same command with `--format csv` when machine-readable output is
   needed. Record `git status --short` before coordination-sensitive planning
   or execution so evidence is tied to the working-tree state.
3. **Candidate selection.** For ordinary development, select from non-`ISSUED`
   deliverables, normally `IN_PROGRESS`. For human-directed formal review or
   closeout gates, select from `CHECKING`. Treat `SEMANTIC_READY` as
   architecture/preparation basis unless the human explicitly asks to work that
   surface.
4. **Deliverable-local context.** After selecting one deliverable, read its
   local `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`,
   `Dependencies.csv` when present, `MEMORY.md`, `_run_records/**`, review
   files when present, semantic/lensing files when present, the four-document
   kit, and primary deliverable artifacts before acting.
5. **DAG-guided related context.** Use `DAG-005/DependencyEdges.csv` and
   `DAG-005/DeliverableNodes.csv` to discover relevant upstream and downstream
   deliverables for the selected work. Inspect those related deliverable-local
   files only as needed for context. DAG rows do not replace local artifact
   inspection.
6. **Bounded execution.** After approval, dispatch canonical `TASK` workers
   only where the file sets are disjoint and the write scopes are explicit.
   Normally use one deliverable or one clearly owned slice per worker. Do not
   advance lifecycle state unless the human explicitly approves the lifecycle
   gate.
7. **Fan-in and validation.** The parent agent fans in worker results, checks
   for scope drift, runs targeted validation, and runs broader readiness checks
   only when the tranche affects release or shared integration surface. Record
   pass, fail, waived, or deferred evidence without overstating acceptance.
8. **Handoff.** Update affected deliverable-local `MEMORY.md` and
   `_run_records/**`, then update `NEXT_INSTANCE_STATE.md` with current
   pointers, completed work, residual gaps, validation state, and the
   recommended next action. Leave `NEXT_INSTANCE_PROMPT.md` stable unless the
   entry protocol itself changes.

The deliverable status helper is deterministic TOOLMAKER-style support: it
lists local lifecycle status and DAG node presence, but it does not approve or
edit lifecycle states, candidate rows, release claims, professional claims, or
graph authority.

## Execution Discipline

Roles and authority:

| Role | Function | Authority |
|---|---|---|
| Human project authority | Confirms scope, resolves ambiguity, accepts deliverables and gates. | Binding decisions. |
| Type 1 persona agent | Decomposes, routes, reconciles, prepares briefs, and fans in results. | Draft/proposal authority only. |
| Type 2 `TASK` agent or bounded worker | Executes one sealed deliverable or explicit tranche slice with bounded context. | Draft/proposal authority only. |
| Deterministic tools | Validate schemas, tests, hashes, reports, lint rules, and status discovery. | Evidence only. |

Expected deliverable-local kit:

| File | Purpose |
|---|---|
| `_STATUS.md` | Lifecycle state and history. |
| `_CONTEXT.md` | Identity, package, scope items, objectives, and acceptance criteria. |
| `_REFERENCES.md` | Source documents and design references. |
| `_DEPENDENCIES.md` | Upstream/downstream dependencies and local dependency evidence. |
| `Datasheet.md` | Key parameters and structured metadata. |
| `Specification.md` | Requirements and acceptance criteria. |
| `Guidance.md` | Rationale, principles, and implementation guidance. |
| `Procedure.md` | Step-by-step execution procedure and checks. |

Some deliverables also carry `Dependencies.csv`, `Review_Findings.csv`,
semantic or lensing files, `_run_records/**`, implementation artifacts, tests,
or generated evidence. Inspect those local files before relying on summaries.

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

Material changes to scope, package boundaries, IDs, solver behavior, rule-pack
semantics, lifecycle state, graph authority, candidate-edge status, release
posture, or data-boundary policy require a decomposition amendment, owning
workflow update, or explicit human decision as applicable. Do not renumber
stable IDs unless the human explicitly requests it.

Completion requires accepted implementation evidence for all applicable
`SOFTWARE_DECOMP` objectives inside the owning deliverable-local evidence
surface, satisfied or explicitly waived active `DAG-005` dependencies, passing
or explicitly waived readiness checks, closed or formally deferred residual
gaps, and the required human gate for any lifecycle transition, release,
acceptance, professional, or code-compliance assertion.
