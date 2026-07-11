# Authority and Source Reliability Map — DELIVERABLE_CONCORDANCE_2026-07-11_1305

> R1 inventory artifact (plan §5 function-specific authority map). Immutable
> run evidence, never a queue or selection surface. All source reads bind to
> the frozen tree `main` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7`
> (worktree `.claude-worktrees/piping-frozen-551f84ef6`). Paths below are
> relative to the project root `projects/chirality-piping` unless prefixed
> `{REPO_ROOT}/`. Method authority: the pinned plan
> `plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md` §5 at
> the run SHA; encoding rules: `R1_CONVENTIONS.md` in this run folder
> (owner-adopted 2026-07-11).

**Function vocabulary** (plan §5, fixed): `Normative scope` ·
`Accepted engineering/product decision` · `Declared current state` ·
`Recorded remaining work` · `Implementation evidence` ·
`Verification evidence` · `Validation evidence` · `Lifecycle evidence` ·
`Execution protocol` · `Historical context`. Process-constraint sources are
labeled `FROZEN_PROCESS_INPUT` (§3) and are never assessed by this run.

**Source-state nuance (recorded, not a conflict).** The frozen tree predates
the D-41 ruling flip commit: at `551f84ef6` the register row `D-41` reads
`AWAITING_RULING`, `SOFTWARE_DECOMP.md` §12 ends at `DEC-072`, and the 100
seeded bootstrap items still read `(gated: D-41)`. The activation of record
(D-41 RULED, codified `DEC-073`, merge `e3998349b`) lives on `main`
post-freeze and is recorded in this run's `RUN_BASIS.md`. Ledger rows copy
frozen-tree text verbatim; the run's authorization rests on the post-freeze
ruling, per the plan §4 HARD RULE.

---

## 1. Function map by source family

### F1 — Owner decision and coordination authority (`execution/_Coordination/**`)

| Path pattern | Functions | Reliability notes | Conflicts |
|---|---|---|---|
| `execution/_Coordination/_DECISIONS/_REGISTER.md` | Accepted engineering/product decision (row states + ruling pointers); Lifecycle evidence (decision-act states); Historical context | Non-governing tracking surface by its own header — authority lives in the rulings it points to. Row-state ladder `NOT_PREPARED → AWAITING_RULING → RULED`; ruled rows immutable, residue gets new rows (owner-ruled convention 2026-07-03). At frozen SHA: D-41/D-12/D-07b/D-38 `AWAITING_RULING`, D-06b `NOT_PREPARED`, all other rows `RULED`. | None |
| `execution/_Coordination/_DECISIONS/D-XX_<slug>.md` (packets) | Accepted engineering/product decision (when RULED, via the recorded fill/§ mechanism); Historical context (options analysis) | Packets are agent-prepared `PROPOSAL`s; only the ruled selection (recorded in packet + `DEC-XXX` codification) is decision authority. Unruled packets (D-12, D-07b, D-38, D-41-at-frozen) are proposals only. | None |
| `execution/_Coordination/_DECISIONS/D-XX_RULING_<date>.md` (D-29, D-30, D-31, D-35, D-36, D-37, D-39, D-40) | Accepted engineering/product decision (ruling records of record) | Owner rulings with verbatim words of record; the highest-reliability decision surfaces in the corpus. D-29/D-30 have no `DEC-XXX` §12 codification — the ruling record + register row are the complete record (pointer decisions, see `DECISIONS_INDEX.csv`). | None |
| `execution/_Coordination/_COORDINATION.md` | Accepted engineering/product decision (ruled records it is the named recording surface for — current target stage PRD R5 per `DEC-054`); Declared current state (target stage only) | Rewritten 2026-07-10 (K-AUTH-1) to a ruled-record surface and pointer; explicitly NOT the operative protocol and must not accumulate status. Pre-2026-07-10 text is git history only. | None |
| `execution/_Coordination/_DECISIONS/_run_records/**` | Lifecycle evidence; Historical context (decision-preparation runs) | Agent-authored run records; support packet provenance, never decision authority. | None |
| `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` | FROZEN_PROCESS_INPUT (see §3); Historical context | Session-handoff prompt surface; superseded in practice by the `loop/` convention. Never product-scope authority. | None |

### F2 — Decision codification and decomposition (`execution/_Decomposition/`, `docs/_Registers/`)

| Path pattern | Functions | Reliability notes | Conflicts |
|---|---|---|---|
| `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 (Decision log) | Accepted engineering/product decision (canonical `DEC-001`–`DEC-072` codifications at frozen SHA) | The project's `DEC-XXX` convention of record (named by plan §4). Entries are immutable history; supersessions are forward entries (`DEC-026`→`DEC-024` Part 1; `DEC-035`→`DEC-029`; `DEC-066`/`DEC-070` partially→`DEC-045` solve path; `DEC-051` partially→`DEC-017` network clause). Read newest-forward. | None |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` §§1–11, §13 | Normative scope (v0.8 decomposition basis after SCA-005: packages, deliverables, SSOW, objectives, ledger); Historical context (intake) | §13 gate posture warns downstream surfaces may be stale relative to rev 0.8 until refreshed by owning workflows — treat deliverable-register agreement as a claim to check, not assume. | None |
| `docs/_Registers/Deliverables.csv` (101 rows) | Normative scope (deliverable identity, type, artifacts, scope/objective traceability, context envelopes) | Live register; the R1 deliverable-inventory backbone. | None |
| `docs/_Registers/ScopeLedger.csv`, `docs/_Registers/ContextBudgetQA.csv` | Normative scope (scope-item ownership); Lifecycle evidence (QA telemetry) | Companion registers to the decomposition. | None |

### F3 — Loop protocol surfaces (`loop/**`)

| Path pattern | Functions | Reliability notes | Conflicts |
|---|---|---|---|
| `loop/WORKPLAN_2026-07-10_piping_loop.md` (newest) | Execution protocol ONLY — never product-scope authority (plan §5) | Self-declared agent-authored non-authority; protocol + pointer indexes, no status. Governs how selected work runs; selection itself comes from deliverable `_STATUS.md` `## Remaining`. | None |
| `loop/WORKPLAN_2026-07-04_piping_loop.md` | Historical context (superseded protocol) | Superseded by the 2026-07-10 workplan; never cite as current protocol. | None |
| `loop/LOOP_INIT.md` | Execution protocol (session orientation; defers to newest workplan on any conflict) | Deliberately fact-free by design; cannot go stale. | None |
| `loop/LOOP_RECEIPTS.md` (24 receipts at frozen SHA) | Historical context; handoff context; Accepted engineering/product decision *evidence* (verbatim owner quotes of record, e.g. Receipts 3–4 for `DEC-066`–`DEC-070`) | Receipts carry owner verbatim directions that §12 entries cite as basis; the receipt is evidence of the ruling, the `DEC-XXX`/ruling record is the codification. Never a selection surface. | None |

### F4 — Governed product-scope canon (`docs/**` core surfaces)

| Path pattern | Functions | Reliability notes | Conflicts |
|---|---|---|---|
| `docs/PRD.md` (v0.2 basis per `DEC-056`/SCA-005) | Normative scope (functional requirements §10, milestones §22, non-goals) | v0.2 FR renumber governed by the D-21 Annex A crosswalk (mandatory forward-traceability basis); ruled history keeps speaking v0.1 numbering — read against Annex A, never rewrite. | Noted (ruled-held): §18.1/§18.2 local-first wording predates `DEC-051` open-residency; follow-on reconciliation HELD by the ruling itself — precedence recorded, encode at claim level as ruled divergence, not `AUTHORITY_CONFLICT`. |
| `docs/CONTRACT.md` | Normative scope (operating kernel contract, `OPS-K-*` invariants) | `OPS-K-PRIV-1` amended and `OPS-K-PRIV-2` clarified by `DEC-051`; IP invariants unchanged. | None |
| `docs/SPEC.md` | Normative scope (system specification) | §4.4 no-required-network posture amended by `DEC-051`. §0.2.4 governs machine-local paths in run records. | None |
| `docs/TYPES.md` §9 | Normative scope (canonical lifecycle vocabulary — the governed change-control regime model per `DEC-071`) | The canon-precedence precondition surface; amended pre-activation (D-39, PR #174). Lifecycle labels are change-control regimes, never completeness scores. | None |
| `docs/DIRECTIVE.md`, `docs/INTENT.md`, `docs/PLAN.md` | Normative scope (directives/intent); Historical context (strategy prose) | `DIRECTIVE.md:91` and `PLAN.md:188` carry pre-`DEC-051` wording under the same ruled HELD follow-on as PRD §18. `plans/` and strategy prose retired as selection surfaces (Receipt 11). | Noted (ruled-held), same as PRD row. |
| `docs/VALIDATION_STRATEGY.md`, `docs/RELEASE_QUALITY_GATES.md`, `docs/BUILD_AND_RELEASE.md`, `docs/RELEASE_NOTES_TEMPLATE.md`, `docs/report_notice_template.md` | Normative scope (validation/release/reporting gates) | `BUILD_AND_RELEASE.md` §7/§8 named by `DEC-025`/`DEC-057`/`DEC-058` as recording surfaces. `RELEASE_NOTES_TEMPLATE.md` is on the `DEC-051` HELD follow-on list. | Noted (ruled-held) on the template only. |
| `docs/architecture/**` (incl. `adr/`), `docs/security/**`, `docs/theory/**`, `docs/user_guide/**`, `docs/developer_guide/**`, `docs/contributor_guide/**`, `docs/validation_manual/**`, `docs/local_analysis/**`, `docs/_Examples/**`, `docs/MANIFEST.json`, `docs/README.md` | Normative scope (ADRs per `DEC-020`; module boundaries; manuals as required artifacts of their owning deliverables); Declared current state (guides describing what exists) | Guides/manuals are deliverable artifacts — audit them as claims of their owning deliverables, not as independent authority. `docs/security/telemetry_policy.md` and `redaction_export_controls.md` are on the `DEC-051` HELD follow-on list. | Noted (ruled-held) on the two named security docs. |

### F5 — Professional and data-boundary rules

| Path pattern | Functions | Reliability notes | Conflicts |
|---|---|---|---|
| `docs/PROFESSIONAL_BOUNDARY.md` | Normative scope (professional ceiling; non-transferable responsibility per `DEC-005`/`DEC-015`) | Explicitly preserved unchanged by `DEC-051`. Plan §3 boundary 4: this run makes no compliance/professional claims. | None |
| `docs/IP_AND_DATA_BOUNDARY.md` | Normative scope (IP/protected-content/private-data rules; §4 provenance manifest; §6.1 added by `DEC-051`) | Equation-source boundary (plan §3 boundary 3) rests here plus `DEC-043`: `domains/piping-design` equation extractions never validate solver formulations. | None |
| `governance/MAINTAINERS.md`, `governance/CONTRIBUTION_REVIEW_CHECKLIST.md`, `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md`, `CONTRIBUTING.md`, `LICENSE.md` | Normative scope (governance/license baseline — DEL-01-01, the sole `ISSUED` deliverable) | Change-managed under `ISSUED` regime; contributions closed per `DEC-027` (D-07b gates reopening). | None |

### F6 — Dependency graph authority (`execution/_DAG/**`)

| Path pattern | Functions | Reliability notes | Conflicts |
|---|---|---|---|
| `execution/_DAG/_LATEST.md` | Normative scope (live discovery pointer — resolves to `DAG-007`) | Verified at frozen SHA: pointer names `DAG-007`, status `approved_active_graph_authority`, approved 2026-06-22. Plan §3 boundary 6: DAG-007 is a provenance baseline — R1/R2 re-verify current rows against the frozen tree before relying on them; no DAG mutation under this run. | None |
| `execution/_DAG/DAG-007/**` (incl. `APPROVAL_RECORD.md`, `DependencyEdges.csv`, `TopologicalWaves.md`) | Normative scope (canonical dependency type system + current edge register, as provenance baseline); Accepted engineering/product decision (approval record) | Selectability derivations use these edges; `TopologicalWaves.md` lists waves leaf-first (workplan warning). | None |
| `execution/_DAG/DAG-001`–`DAG-006/**` | Historical context only | Superseded/historical graphs; a legacy DAG label re-emitted as current authority is an R6 backcheck failure (plan §8). | None |

### F7 — Deliverable-local surfaces (`execution/PKG-XX_*/1_Working/DEL-XX-XX_*/**`, 101 deliverables)

| Path pattern | Functions | Reliability notes | Conflicts |
|---|---|---|---|
| `Specification.md`, `Datasheet.md`, `Guidance.md`, `Procedure.md` (four-doc kit) | Normative scope (accepted deliverable scope/exclusions); Declared current state (one `DECLARED_STATE` row per surface per R1 convention addendum 1) | Known corpus condition (plan §2): many kits retain setup-era future-tense prose — classify at claim level (`STALE_SETUP_SPECIFICATION` per widened addendum 4), never by keyword flagging. | None (staleness is a disposition, not an authority conflict) |
| `_STATUS.md` `## Remaining` | Recorded remaining work (the SOLE current work-discovery surface); Lifecycle evidence (state header); Declared current state | Gate/stage suffixes preserved verbatim. The seeded `(gated: D-41)` bootstrap item is program mechanics — recorded verbatim in the surface row only, excluded from residual analysis (R1 convention addendum 2). Frozen tree carries post-D-40 states (PKG-00 8/8 `IN_PROGRESS`; `DEL-01-01` `ISSUED`). | None |
| `MEMORY.md` | Declared current state (when carrying current declarations); Historical context (dated log entries) | Dated log entries are historical records; drift inside them is a note on the MEMORY surface row, never a staleness disposition (addendum 1). | None |
| `Dependencies.csv`, `_DEPENDENCIES.md` | Normative scope (deliverable-local dependency rows — re-verify against DAG-007 type system) | Legacy/noncanonical dependency labels are an R3 finding class, not repaired here. | None |
| `_CONTEXT.md`, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `_REFERENCES.md` | Historical context (sealed context envelopes, provenance) | Setup-era context; never overrides current decisions or registers. | None |
| `_REVIEW.md`, `Review_Findings.csv` | Verification evidence; Validation evidence (as recorded); Lifecycle evidence (dispositions) | Findings carry per-row human dispositions; reliability per the R1 convention-6 ladder (`REVIEWED` requires a named human ruling/disposition covering the cited record). | None |
| `_run_records/**` (117 dirs, ~1,736 files corpus-wide, incl. package-level `1_Working/_run_records` and `_audit`) | Implementation evidence; Verification evidence; Validation evidence (where witness/benchmark-bearing); Lifecycle evidence; Historical context | Family grain: `TASK_RUN_*` / `WORKING_ITEMS_RUN_*` per deliverable. A passing snapshot supports its evaluated source state only — never auto-proves the frozen SHA; overtaken evidence needs the content-identical qualifier (addendum 10). Agent-generated with pending human disposition = `UNVERIFIED` (addendum 6). | None |
| Deliverable-owned in-tree `README.md` files | Declared current state (one row each per addendum 1) | Census per addendum 1. | None |

### F8 — Implementation source (`core/`, `apps/`, `schemas/`, `tools/`, packaging)

| Path pattern | Functions | Reliability notes | Conflicts |
|---|---|---|---|
| `core/**` (21 crate families: solver, loads, units, rules, reporting, runner, model_operations, product_physics, security, serialization, …) | Implementation evidence | Plan §3 boundary 1: implementation is evidence, never scope authority. Code presence ≠ engineering acceptability (boundary 2). | None |
| `apps/desktop/**` (incl. `SMOKE.md`) | Implementation evidence (GUI/app surfaces); Verification evidence (`SMOKE.md` human smoke entries, e.g. TP-MAC-141/190 cited by `DEC-035`/`DEC-048`) | `SMOKE.md` human entries are owner-performed evidence of record. | None |
| `schemas/**` (~30+ schema files) | Implementation evidence (public schema surfaces); Normative scope only where a ruling ratified specific members (`DEC-034`, `DEC-038`, `DEC-039`) | Schema versioning governed by the `DEC-033`/`DEC-038` additive-minor policy; `material.schema.yaml` carries the open `temperature_interpolation_policy` topic (D-38 `AWAITING_RULING`). | None |
| `tools/**` (incl. `tools/coordination/*.py`, `tools/release/*`) | Implementation evidence (deterministic tools); Execution protocol support (loop Step 0 enumeration tools) | In scope as mapped deliverable implementation where they do not alter agent-workflow semantics (plan §3 boundary 9). | None |
| `package.json`, `package-lock.json`, `requirements-dev.txt`, crate manifests, `_harness/` config | Implementation evidence (packaging/build configuration) | Build outputs, `target/`, caches, local DBs are EXCLUDED from source inventory (plan §8 R1). | None |
| `examples/**`, `init/**`, `api/**` | Implementation evidence (public example/API surfaces); Declared current state (example provenance blocks) | Example corpora carry human-review acceptances (`DEC-030`/`DEC-032` for the operation contract corpus). | None |

### F9 — Verification assets

| Path pattern | Functions | Reliability notes | Conflicts |
|---|---|---|---|
| `tests/**` (repo-level pytest suites), per-crate `#[cfg(test)]`/`tests/`, desktop Vitest/Playwright suites | Verification evidence | Unit/integration evidence for implementation behavior; a unit test alone is never promoted to engineering validation (plan §6). | None |
| `fixtures/**` (24 families incl. `rule_expressions/` golden corpus, `model_operations/contract_corpus/`) | Verification evidence (conformance/golden inputs); Validation evidence support (witness inputs) | Corpus human-review status is decision-recorded (`DEC-030`, `DEC-031`, `DEC-032`); future additions need their own review entry. | None |

### F10 — Validation and provenance assets

| Path pattern | Functions | Reliability notes | Conflicts |
|---|---|---|---|
| `validation/hand_calcs/**`, `validation/benchmarks/**`, `validation/witness/**` | Validation evidence (analytic seeds, mechanics/stress/nonlinear benchmarks, witness artifacts) | Tolerance regime per `DEC-026` (class-tiered, tighten-only); convergence regime per `DEC-046`. Mechanics claims require this class of evidence or an explicit open-validation statement. | None |
| `validation/evidence/sweeps/SWEEP_*_<commit12>.json` | Verification evidence (commit-bound `DEC-025` five-surface merge-gate artifacts); Validation evidence support | Commit-bound by filename+payload; binds only its named commit — content-identical qualifier required to carry evidence forward (addendum 10). | None |
| `validation/evidence/coverage/**`, `validation/evidence/gates/**`, `validation/evidence/release_artifacts/**` | Validation evidence / Lifecycle evidence (telemetry, gate records, release-artifact records incl. future `SCAN_*` per `DEC-058`) | Coverage telemetry is recorded-never-blocking (`DEC-060`). No scan yet exists at frozen SHA (nothing published). | None |
| `provenance/build-artifacts/**` | Validation evidence support (build provenance) | Evidence support only. | None |

### F11 — Scope-change records

| Path pattern | Functions | Reliability notes | Conflicts |
|---|---|---|---|
| `execution/_ScopeChange/SCA-001…SCA-005/**` (+ `_LATEST.md`) | Accepted engineering/product decision (accepted SCA gate records; SCA-005 executed the D-29/`DEC-056` v0.2 propagation); Historical context | SCA-005 `Handoff_State.md` is part of the D-29 ruling record set. Named snapshots are provenance baselines (plan §3 boundary 6) — verify current surfaces reflect them rather than citing the snapshot as current truth. | None |
| `docs/_ScopeChange/**` (PRD v0.2 original + brief, SCA drafts) | Historical context (scope-change source material) | The live `docs/PRD.md` is the current normative surface, not the v0.2 original here. | None |

### F12 — Archival, review, and reconciliation records (`execution/_Reconciliation/**`, `_Aggregation/**`, `_Evaluation/**`, `_Change/**`)

| Path pattern | Functions | Reliability notes | Conflicts |
|---|---|---|---|
| `execution/_Reconciliation/Reviews/**` | Verification evidence; Lifecycle evidence (recommendations/dispositions); Historical context | See reviews index (§4). Timestamp-identified run folders; source-state identifiers live inside each record at row grain (e.g. dependency-row and finding IDs), generally not commit-SHA-bound — pre-`DEC-025`-era practice. `STALE_REVIEW_OR_EVIDENCE` applies where overtaken. | None |
| `execution/_Reconciliation/PKG00LockReview/PKG00_LOCK_REVIEW_2026-05-11_2218/**` | Verification evidence (historical); Historical context; declared future checking basis (preserved by `DEC-072`) | Preserved as historical evidence and available as a declared checking basis for PKG-00 `CHECKING` re-entry — not a current lifecycle claim. | None |
| `execution/_Reconciliation/LifecycleCorrection/**` (2 runs + `_LATEST.md`) | Lifecycle evidence (reversal/correction records of record — DEL-04-01 reversal precedent; 2026-07-02 STATUS_HISTORY_MISMATCH exemplar ruling) | The 2026-07-02 run carries a verbatim owner ruling ("…all shall be IN_PROGRESS"). Live precedent for the §3 reversal-only `CHECKING` exit. | None |
| `execution/_Reconciliation/DepClosure/**` (2 runs + `_LATEST.md`) | Verification evidence (historical dependency-closure); Historical context | Predates DAG-007; closure rows cite DAG-002-era edge IDs — historical inputs only. | None |
| `execution/_Reconciliation/DependencyTypeSystem/TYPE_RECTIFICATION_2026-06-16/**`, `DependencySemanticRefresh/SEMANTIC_REFRESH_2026-06-16/**` | Historical context; Accepted engineering/product decision basis (DAG-007 rectification inputs) | Basis records for the DAG-007 approval; DAG-007 itself is the surviving authority. | None |
| `execution/_Reconciliation/PKG-02_FoundationSlice_Hardening_2026-05-16/`, `PKG-02_FourDocInitialization/`, `Reconciliation_Run_Summary_*.md` (3), `_LATEST.md` | Historical context (early reconciliation runs) | `_LATEST.md` repointed 2026-07-01 after the DEV-001 archive retirement (retired material lives under `{REPO_ROOT}` archive paths named in the pointer note; absent from this project tree at frozen SHA). | None |
| `execution/_Aggregation/**` (20 entries + `_LATEST.md`) | Lifecycle evidence; Verification evidence (closeout/audit aggregations); Historical context | Includes the release-readiness gap audits and TP-* closeouts; snapshots of their day's state. | None |
| `execution/_Evaluation/**`, `execution/_Change/**` | Historical context (DEV-001-era evaluation reports; SCA-002 control-plane closeouts) | Explanatory only; cannot override current decisions (plan §3 boundary 7). | None |
| `execution/_Reconciliation/DeliverableConcordance/DELIVERABLE_CONCORDANCE_2026-07-11_1305/**` (working tree, this run) | Execution protocol (run conventions); run evidence container | Not present at the frozen SHA (created under the D-41 activation). `R0_CALIBRATION`/`R0B_CALIBRATION` ledgers are calibration evidence only (RUN_BASIS run-level note); R1+ encodings bind to `R1_CONVENTIONS.md`. | None |

---

## 2. Open decision rows at the frozen SHA (for DecisionBasis lookups)

`D-12` (FR-024/FR-025 disposition), `D-07b` (contributor intake), `D-38`
(temperature interpolation policy) — `AWAITING_RULING`; `D-06b`
(signing/notarization re-decision) — `NOT_PREPARED`; `D-41` — `AWAITING_RULING`
at the frozen SHA, RULED post-freeze (`DEC-073`, see header nuance). Claims
whose closure depends on these rows route `AuthorityNeeded=OWNER` or the named
decision ID; none of these is an `AUTHORITY_CONFLICT`.

## 3. FROZEN_PROCESS_INPUT list (constrain execution; never assessed here)

Per plan §5, these may constrain how this run executes; this program does not
assess, consolidate, retire, or port them. Any finding that depends on them is
classified `DEFERRED_AGENT_WORKFLOW` and copied to
`AGENT_WORKFLOW_OBSERVATIONS.md` without redesign recommendation.

- `{REPO_ROOT}/AGENTS.md` — root agent instructions (carries the `DEC-043`
  knowledge-source reliability routing).
- `{REPO_ROOT}/agents/AGENT_*.md` (38 files) and `{REPO_ROOT}/agents/AGENTS.md`
  — agent role instructions and matrices.
- `{REPO_ROOT}/skills/**` (dbm-*, decomposition-*, deliverable-consistency,
  dependency-extract, …) — skill contracts.
- `{REPO_ROOT}/init/**` and project `init/**` — session launcher prompts.
- `projects/chirality-piping/AGENTS.md` — project-local agent instructions.
- `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` — workflow guidance; its authority
  table naming `docs/TYPES.md` §9 as the canonical lifecycle vocabulary is the
  one fact this run relies on (via the plan §4 precondition and `DEC-071`);
  the workflow content itself is process input.
- `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` — handoff prompt surface.
- `_harness/**` (project harness configuration).

If any older workflow guide conflicts with `loop/WORKPLAN_2026-07-10`, the
stale-process fact is recorded for the separate owner-led program, not
repaired here (plan §5).

## 4. AUTHORITY_CONFLICT findings

**No open `AUTHORITY_CONFLICT` found at the frozen SHA.** Two ruled-precedence
divergences are recorded so wave pilots do not re-flag them as conflicts:

1. **`DEC-051` HELD conforming follow-ons.** `docs/PRD.md` §18.1/§18.2,
   `docs/PLAN.md:188`, `docs/DIRECTIVE.md:91`,
   `docs/security/telemetry_policy.md`,
   `docs/security/redaction_export_controls.md`, and
   `docs/RELEASE_NOTES_TEMPLATE.md` still carry local-first/no-network/
   private-by-default wording superseded for the runtime channel by the
   `DEC-051` open-residency ruling. The ruling itself records the precedence
   (amended surfaces govern; follow-on reconciliation surfaced per
   K-CONFLICT-1 and HELD by the owner). Encode affected claims as ruled
   divergence/stale-wording dispositions with `DecisionBasis=DEC-051`, not
   `AUTHORITY_CONFLICT`.
2. **Frozen-SHA D-41 row state vs. run authorization.** Register/decomp/
   bootstrap items at `551f84ef6` predate the ruling flip (header nuance).
   Precedence is recorded in `RUN_BASIS.md` (plan §4 HARD RULE satisfied on
   `main`); not a live-source conflict.

## 5. Reviews index (recorded reviews, findings, dispositions)

Source-state identifiers are the timestamped run IDs below (format
`<ID>_<YYYY-MM-DD>_<HHMM>`); commit-SHA binding, where present, is inside each
record. Family grain per the task contract; per-deliverable `_run_records/**`
(117 directories, ~1,736 files) are indexed at family grain in row F7 above.

### 5.1 `execution/_Reconciliation/Reviews/` — 38 records (+ `_LATEST.md` → REV_TP-INPROGRESS-CHECKING-READINESS_2026-06-07_1750)

Deliverable-scoped review runs (31): REV_DEL-01-01 (2026-06-03_2327,
2026-06-03_2334); REV_DEL-04-01, -04-02 (2026-06-05_2120); REV_DEL-04-04,
-04-05 (2026-06-05_2242); REV_DEL-05-01 (2026-06-05_2021); REV_DEL-05-04
(2026-06-05_2053); REV_DEL-05-02, -05-03, -05-05 (2026-06-05_2120);
REV_DEL-08-01, -08-02, -08-03, -08-06 (2026-06-06_1025); REV_DEL-09-05,
REV_DEL-11-04 (2026-06-07_1455); REV_DEL-11-03 (2026-06-07_1652);
REV_DEL-12-01, -12-02, -12-03, -12-04 (2026-06-07_1112); REV_DEL-14-03
(2026-06-07_1402); REV_DEL-15-01, -15-02, -15-03, -15-04 (each at
2026-06-07_0028 and 2026-06-07_0050). Typical contents: `Brief.md`,
`Decision_Log.md`, `Review_Summary.md`, `QA_Report.md`, `RUN_SUMMARY.md`;
dispositions include lifecycle recommendations (e.g. DEL-04-01
`RECOMMEND_ADVANCE_TO_CHECKING`, later reversed — see 5.3) and human finding
dispositions (`HumanDisposition=ACCEPT_AS_IS`).

Package-scoped review runs (4): REV_PKG-16 (2026-06-06_1648, 2026-06-07_1606);
REV_PKG-10 (2026-06-07_1326); REV_PKG-15 (2026-06-07_1340).

Cross-corpus readiness sweep (1): REV_TP-INPROGRESS-CHECKING-READINESS_2026-06-07_1750.

Standalone records (2): `PKG-12_Setup_Review_2026-04-30.md`;
`PKG05_BLOCKER_CLOSURE_RULING_PACKET_2026-06-05_2120.md` (human
blocker-closure ruling packet backing the 2026-06-05 PKG-04/PKG-05 reviews).

Reliability: recorded reviews with human dispositions qualify as `REVIEWED`
for the rows they cover (R1 convention 6); all predate the 2026-07-11
lifecycle rebaseline — any `CHECKING` recommendation or application they
record is superseded lifecycle history under `DEC-071`/`DEC-072`, while their
technical findings remain citable evidence for their reviewed state.

### 5.2 `execution/_Reconciliation/PKG00LockReview/PKG00_LOCK_REVIEW_2026-05-11_2218/` — 1 run

`PKG00_Lock_Review_Report.md`, `PKG00_Lock_Review_Register.csv`,
`Downstream_Propagation_Check.csv`, `Decision_Recommendations.md`,
`Source_Index.csv`, `QA_Report.md`, `RUN_SUMMARY.md`. Status: preserved
historical evidence and the natural declared checking basis for future PKG-00
`CHECKING` re-entry (`DEC-072`).

### 5.3 `execution/_Reconciliation/LifecycleCorrection/` — 2 runs (+ `_LATEST.md` → 2026-07-02 run)

- `LIFECYCLE_CORRECTION_2026-05-11_2052/` (`Lifecycle_Correction_Register.csv`,
  `Decision_Log.md`, `Brief.md`, `QA_Report.md`, `Source_Index.csv`,
  `RUN_SUMMARY.md`): the DEL-04-01 `CHECKING`→`IN_PROGRESS` reversal record —
  the live precedent named by plan §2/§3.
- `LIFECYCLE_CORRECTION_2026-07-02_2050/` (`Decision_Log.md`): owner ruling of
  record on the DEL-10-03 exemplar and the STATUS_HISTORY_MISMATCH class
  ("Yes this is the exemplar and all shall be IN_PROGRESS.", Ryan Tufts,
  2026-07-02).

### 5.4 `execution/_Reconciliation/DepClosure/` — 2 runs (+ `_LATEST.md` → SCA002 run)

`CLOSURE_PKG-12-POST-SETUP_2026-04-30_1438/` and
`CLOSURE_SCA002_REV05_COMPATIBILITY_2026-05-03_1441/` (each:
`Dependency_Closure_Report.md`, `Dependency_Closure_IssueLog.csv`,
`Decision_Log.md`, `Brief.md`, `QA_Report.md`, `RUN_SUMMARY.md`, `Evidence/`,
`analyze_closure.py`). Historical: DAG-002-era edge IDs; superseded by DAG-007
for current dependency truth.

### 5.5 Other reconciliation-family records

- `DependencyTypeSystem/TYPE_RECTIFICATION_2026-06-16/` and
  `DependencySemanticRefresh/SEMANTIC_REFRESH_2026-06-16/` — DAG-007
  rectification basis records.
- `PKG-02_FoundationSlice_Hardening_2026-05-16/SUMMARY.md`;
  `PKG-02_FourDocInitialization/{AUDIT_SUMMARY.md,RECONCILIATION_SUMMARY.md}`.
- Loose summaries: `Reconciliation_Run_Summary_2026-04-30_PKG-12_POST_SETUP.md`;
  `…_2026-05-02_DEL0805_CANDIDATE_E0621.md`;
  `…_2026-05-03_SCA002_REV05_COMPATIBILITY_PLANNING.md` (current `_LATEST.md`
  target after the 2026-07-01 repoint).
- `execution/_Aggregation/**` — 20 closeout/audit aggregation runs (e.g.
  `AGG_RELEASE_READINESS_GAP_AUDIT_2026-05-11_*`, `AGG_TP_PHYS_00X_CLOSEOUT_*`,
  `TP-*` lifecycle/evidence audits of 2026-05-31–2026-06-03).
- `execution/_Evaluation/EVALUATION_REPORT.md` (+ protocol, reports,
  content-digests) — DEV-001-era read-only audit report (2026-05-11).

### 5.6 This run's calibration records (working tree)

`DeliverableConcordance/DELIVERABLE_CONCORDANCE_2026-07-11_1305/R0_CALIBRATION/`
and `R0B_CALIBRATION/` — calibration evidence only per the owner-adopted
`R1_CONVENTIONS.md` (R0/R0b ledgers predate the binding convention set; the
DEL-09-01 fixture-count FAIL is recorded in `R0B_REVIEW.md` §3 with the
R1-named repair — count corrected to 21 in the re-encoded wave ledger).
