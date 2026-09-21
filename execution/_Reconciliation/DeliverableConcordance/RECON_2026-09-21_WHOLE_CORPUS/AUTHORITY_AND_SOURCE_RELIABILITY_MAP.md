# Authority and source-reliability map — R1

Agent 0's map of the sources workers may cite, and the function each one has
under `CONVENTIONS.md` A3. It is a classification aid, not a ruling. Where
two governing sources conflict, workers record `AUTHORITY_CONFLICT`; this map
never decides precedence. All paths are at the frozen commit `00115c719` and
relative to `projects/chirality-piping/` unless they start at the repository
root.

## Governing (authority: what is wanted)

| Source | Function | Notes |
|---|---|---|
| `docs/PRD.md` (v0.4 after SCA-010; product name SWBPIPE) | Product requirements | The line-15 historical path keeps the former name by design (SCA-010) |
| `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, `docs/SPEC.md`, `docs/TYPES.md` | Project contract, directive, specification, types (lifecycle vocabulary in TYPES §9) | Project copies. The repository root has files with the same names; cite project copies as `projects/chirality-piping/docs/…` (Part D) |
| `docs/IP_AND_DATA_BOUNDARY.md`, `docs/PROFESSIONAL_BOUNDARY.md`, `docs/claims_registry.md` | Boundary invariants and the claims taxonomy (DEC-081) | Claims restating these are `INVARIANT` tier |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` rev 0.12, including §12 `DEC-001`–`DEC-109` | Decomposition, package and deliverable scope, codified rulings | The package table (name, scope, assigned items, exclusions) backs CS-06 |
| `docs/_Registers/Deliverables.csv`, `ScopeLedger.csv` | Companion registers to the decomposition | Back CS-06 objective, scope-coverage and scope-detail checks |
| `execution/_Coordination/_DECISIONS/_REGISTER.md` rows D-01–D-72 and their ruling records | Owner rulings | A ruling's "adopted effect" governs. Its packet's analysis is context |
| Accepted scope changes: SCA-001 to SCA-010 | Scope amendments | See "Two scope-change pointers" below |
| `execution/_DAG/_LATEST.md` → `DAG-010` | Approved dependency graph | A dependency authority only; it does not settle scope |
| Loop fences F-PIP-1 to F-PIP-4 (`loop/WORKPLAN_2026-07-18b_piping_loop.md`, "Standing constraints") and DEC-043 | Standing boundaries | Retained by the owner-adopted successor procedure (`loop/WORKPLAN_2026-09-19_piping_loop.md`) |

## Adopted by reference (governing via a ruling; flag `AdoptedByReference=YES`)

Some rulings adopt a plan or specification wholesale. The ruling governs;
the adopted document supplies the content it points to. The owner decides at
R4 how far such adoption reaches (A3).

| Ruling | Adopted document |
|---|---|
| D-66 | Execution control frozen at `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION/ORCHESTRATION_PLAN_V1.md` and `WORK_GRAPH_V1.json` (first wave and source exception only; the rest of D-66 unruled) |
| D-67 | `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE/APPROVED_PLAN.md` (analysis-record and stress-neutral 0.2 compatibility, private dormant pressure kernel, verification, production-UI readiness handoff) |
| D-68 | `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260915-PRODUCTION-UI/APPROVED_PLAN.md` (professional modelling workspace foundation; 10,000-pipe viewport targets; bounded accessibility criteria) |
| D-72 and addendum | The redesigned-product performance criteria in the D-72 packet as ruled (items 1–4 and 6, S-1, S-2) |
| DEC-074 O7 | Text in an excluded `PROPOSED_*` file of the July run. Stays unread; rows depending on it are flagged (R0 review §7) |

## Context (explains; never decides a disposition by itself)

- AgentRuns records: work graphs, handoffs, orchestration plans not adopted by
  a ruling, briefs, returns, and design program artifacts. Examples: the UI
  design specification V1.2, frames, and the implementation handoff of
  2026-09-18.
- Owner-direction records in AgentRuns. These are context, **except** under
  A3a, where a verbatim, hash-bound owner decision naming the exact divergence
  may support `ACCEPTED_DIVERGENCE`, routed to R4 for confirmation. One
  example is the 2026-09-18 hard-identifier selection in
  `…/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/ROOT/ACTIVATION_2026-09-18.md`.
  The owner has since directed that active identifiers carrying the former
  name are residue (R0 ruling addendum).
- Merged pull requests and their descriptions (A2: owner intent through the
  standing Git instruction; merge state is a fact).
- Deliverable `MEMORY.md`, `_run_records/**` and `loop/LOOP_RECEIPTS.md`.
- Coordination notices and task-management records.
- Prior reconciliation runs, including July's, as provenance only. Workers do
  not read the July ledgers.

## Evidence (what exists; may change a disposition)

- Code, tests, schemas and fixtures at the frozen commit.
- Validation assets (`VALIDATION_AND_PROVENANCE_INDEX.csv`).
- Run records that are themselves evidence: parity reports and claim maps
  (`EVIDENCE_MAP.csv`), review returns, and validation outputs. Cite them in
  the evidence columns; `SourceReliability` says whether a human disposition
  covers them (C10).
- Gate evidence: `GATE:GATE_EVIDENCE/…` (A6).

## Reliability rules

- **DEC-043.** Unreviewed extracted equation artifacts from the external
  piping-design corpus are never evidence for engineering formula claims. Use
  the maintainer's vetted sources and record their review status.
- `REVIEWED` requires a named human ruling or disposition covering the cited
  record. Agent-produced technical evidence with no human disposition is
  `UNVERIFIED`. Test and run-record evidence is `NOT_APPLICABLE` (C10).
- Verification never stands in for validation (A5).

## Two scope-change pointers

- `docs/_ScopeChange/_LATEST.md` names **SCA-003** (docs-side authority
  record, 2026-05-17).
- `execution/_ScopeChange/_LATEST.md` names **SCA-010** (the latest accepted
  amendment, 2026-09-18).

The docs-side pointer tracks amendments whose authority record lives under
`docs/_ScopeChange/`. Later amendments keep their records under
`execution/_ScopeChange/` only. The two are not in conflict as pointers, but a
claim that cites "the latest scope change" by the docs-side pointer is
citing a stale basis (CP-02). This map does not rule on whether the docs-side
pointer should have advanced.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
