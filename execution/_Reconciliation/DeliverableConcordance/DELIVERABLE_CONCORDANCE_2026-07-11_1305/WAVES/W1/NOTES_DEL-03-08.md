# NOTES — DEL-03-08 Pipe section property and mass-property calculator (W1)

Wave W1 · frozen SHA `551f84ef6be656f1603ce0acfa5e3935aa9683c7` · binding set
`R1_CONVENTIONS.md` (conventions 1–8 + addenda 1–13). Ledger:
`CLAIM_CONCORDANCE_DEL-03-08.csv` (15 rows).

## Run-level path aliases (addendum 12)

- `KIT/` = `projects/chirality-piping/execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-08_Pipe section property and mass-property calculator/`
- Bare code/test/schema/validation paths (`core/…`, `tests/…`, `schemas/…`,
  `validation/…`) are relative to the working root `projects/chirality-piping/`.

## Requirement-ID normalization (addendum 12)

The deliverable's native scheme is `DEL-03-08-RQ-*` (Specification requirements
table). Normalized to the addendum-12 `<DEL-ID>-<TYPE>-NNN` ClaimID form:

| Native (Specification) | ClaimID |
|---|---|
| DEL-03-08-RQ-001 | DEL-03-08-REQ-001 |
| DEL-03-08-RQ-002 | DEL-03-08-REQ-002 |
| DEL-03-08-RQ-003 | DEL-03-08-REQ-003 |
| DEL-03-08-RQ-004 | DEL-03-08-REQ-004 |
| DEL-03-08-RQ-005 | DEL-03-08-REQ-005 |
| DEL-03-08-RQ-006 | DEL-03-08-REQ-006 |
| DEL-03-08-RQ-007 | DEL-03-08-REQ-007 |

All 7 native requirement IDs map 1:1; none dropped or merged.

## Disposition histogram (reproduces from CSV column `Disposition`)

| Disposition | Count |
|---|---|
| ALIGNED | 12 |
| STALE_SETUP_SPECIFICATION | 2 |
| VERIFIED_NOT_VALIDATED | 1 |
| **Total** | **15** |

## ClaimType histogram (reproduces from CSV column `ClaimType`)

| ClaimType | Count |
|---|---|
| REQUIREMENT | 7 |
| EXCLUSION | 2 |
| DECLARED_STATE | 6 |
| **Total** | **15** |

## Census decisions

- **Requirements (7):** one row per current requirement ID; substance
  dispositions only (no requirement row takes STALE_SETUP_SPECIFICATION).
- **Acceptance rows: none.** The Specification `## Verification` table merely
  restates the requirement areas (unit safety, missing input, provenance, IP
  boundary, solver boundary); no addendum-12-grain acceptance criteria exist, so
  no mirrored ACCEPTANCE rows.
- **Exclusions (2):** EXC-001 (global solver / rule-pack / code-compliance out of
  scope) and EXC-002 (no bundled public/protected data — tables, defaults,
  conversion constants, SIF/flexibility, code values). Both are repeatedly and
  stably declared scope exclusions (_CONTEXT package exclusions, Datasheet
  posture rows, README, MEMORY). EXC-001 overlaps REQ-007 in substance; the dual
  encoding (positive isolation requirement + negative scope exclusion) matches
  the sibling PKG-03 ledger (DEL-03-07 EXC-004 vs its solver requirement).
- **DECLARED_STATE (6):** exactly one per four-document kit surface
  (Specification, Datasheet, Guidance, Procedure) + `_STATUS.md` + `MEMORY.md`
  (addendum 1). **No row for `core/section_properties/README.md`:** the
  addendum-1 "deliverable-owned in-tree README" is read as a README living in the
  deliverable folder; DEL-03-08's folder has none, and the sibling DEL-03-07
  ledger likewise gave its `core/library_import/README.md` no DECLARED_STATE row.
  The code-surface README is treated as implementation evidence (cited on REQ
  rows), and its own mill_tolerance omission is captured as a note.
- **REMAINING_WORK rows: none.** `_STATUS.md ## Remaining` carries only the
  seeded `(gated: D-41)` bootstrap item, which is recorded verbatim in the
  DECL-005 (`_STATUS`) `RecordedRemaining` cell and excluded from all
  residual/gate/selectability analysis (addendum 2). `DELIVERABLE_INVENTORY.csv`
  confirms `NonBootstrapItems=NONE`. The many doc-level `TBD`s are declared open
  items (not `_STATUS ## Remaining`), captured in the `RemainingWork` column of
  the requirement rows; none is permitted by a named ruling, so none becomes
  ACCEPTED_DIVERGENCE (addendum 11) and none earns its own row.
- **IMPLEMENTED_UNMAPPED rows: none.** The material surface
  `core/section_properties` (SURF-124) is named in the deliverable kit and
  attributed to DEL-03-08 — mapped, not unmapped. The shared Rust crate
  `core/product_physics` (SURF-102) is broadly attributed (incl. DEL-03-08 for
  the mill_tolerance preview mechanics) and is not in this deliverable's
  exclusive orbit; no unmapped row is owed.
- `SelectableUnderCurrentLoop = NO` on every row: no recorded non-bootstrap item
  exists (conventions 6/12). The owner suspension is a run-level caveat, not
  encoded per-row.

## Central evidence finding — mill_tolerance doc drift

The 2026-07-10 run `TP-PMM-P3-MILLTOL-001` added a user-entered `mill_tolerance`
input to `schemas/section.schema.yaml` (line 539) and
`core/section_properties/calculator.py` (`PipeSectionInput.mill_tolerance` L46;
effective wall `t_eff = wall - corrosion - mill_tolerance` L157), with 5 new
tests. The four-document kit + README were last reconciled 2026-06-05 and none
mentions `mill_tolerance` (confirmed: `grep -il mill` over Specification,
Datasheet, Guidance, Procedure, _STATUS, MEMORY, README → no match). This is a
post-alignment drift (addendum 4). It lands on the DECLARED_STATE surface rows
that make an enumerative input claim now contradicted by the omission —
Specification (DECL-001) and Datasheet (DECL-002) → STALE_SETUP_SPECIFICATION —
and as a drift NOTE on the non-enumerative surfaces (Guidance DECL-003, README as
implementation-evidence note, MEMORY DECL-006). The requirement rows keep
substance dispositions (no requirement row takes STALE_SETUP_SPECIFICATION).

## Central evidence finding — MECHANICS validation posture (REQ-003)

The calculator's section-property closed forms are independently derived in
hand-calc witnesses (`HC-STRESS-013`, `HC-STRESS-014`) and Rust benchmark
fixtures that name `core/section_properties/calculator.py` as governing
`calculator_ref` (`validation/benchmarks/stress/src/lib.rs:1448`,
`is_governed_section_property_evidence`), and a section-property witness
`WIT-001` exists. But **no test executes the Python calculator against those
oracle values**: `tests/test_section_properties.py` re-derives the same closed
forms (verification), and `tests/test_calculation_witness.py` only gates witness
rendering fidelity. The Python production path's numeric correctness is therefore
verified, not validation-bound → `VERIFIED_NOT_VALIDATED` (method §6; convention
7 MECHANICS; verification never promoted to validation). Routed
`AuthorityNeeded=ENGINEERING`, Confidence MEDIUM.

## Self-flagged rows

- **DEL-03-08-REQ-002** — ClaimClass judgment MECHANICS-vs-WORKFLOW: missing-value
  rejection is calculator solve-required guard behavior. Classed MECHANICS with an
  explicit in-cell reason why validation does not apply (deterministic guard,
  negative unit tests, no numeric oracle). The sibling DEL-03-07 flagged the
  analogous GOVERNANCE-vs-WORKFLOW call.
- **DEL-03-08-REQ-003** — VERIFIED_NOT_VALIDATED judgment (see MECHANICS finding
  above): validation evidence for the closed forms exists but binds by metadata
  reference, not by executing the production Python path. A reviewer may hold
  that the witness/benchmark `calculator_ref` binding is sufficient to read this
  as validated; I judged execution-binding is required for a MECHANICS numeric
  claim.
- **DEL-03-08-REQ-004** — ClaimClass SCHEMA-vs-GOVERNANCE (provenance/redistribution
  flow through the calculator schema hook) and bounded-scope judgment: dispositioned
  ALIGNED for the calculator-hook obligation only, with private-library record
  linkage / schema field placement carried as TBD RemainingWork (not treated as
  an unimplemented requirement).
- **DEL-03-08-REQ-005** — Confidence MEDIUM: no deliverable-specific protected-content
  review record was located beyond the repo-wide linter
  `tests/test_report_protected_content_linter.py`; the "no paraphrased protected
  tables" assurance rests on an agent-recorded scan (SourceReliability UNVERIFIED).
- **DEL-03-08-DECL-001, DEL-03-08-DECL-002** — STALE_SETUP_SPECIFICATION judgment
  driven by the mill_tolerance omission. Reviewer eyes requested on whether one
  omitted optional input warrants STALE on the authoritative Spec/Datasheet
  surfaces (my read) versus an ALIGNED-with-note treatment (the conservative
  option applied to Guidance/README here). DECL-002 also carries a secondary
  decomposition-revision citation lag (0.7 vs frozen 0.8), noted but not the
  driver.
- **DEL-03-08-DECL-003** — ALIGNED-with-note judgment: Guidance's optional-contributor
  list is illustrative ("such as"), so the mill_tolerance omission is a note, not
  STALE. Also notes the Conflict Table still shows
  TECHNICALLY_ADDRESSED_PENDING_HUMAN while Review_Findings.csv/MEMORY record Gate A
  ACCEPT_AS_IS/RESOLVED (2026-06-05).
- **DEL-03-08-DECL-004** — ALIGNED-with-note (verifier-prompted, re-verified by
  this pilot): Procedure L38's Review-finding check row still expects findings to
  remain `TECHNICALLY_ADDRESSED_PENDING_HUMAN` with `HumanDisposition=TBD`, but
  Review_Findings.csv rows PKG03-DEL-03-08-PKG02-001..003 carry Gate A
  `ACCEPT_AS_IS`/`RESOLVED` (2026-06-05) — overtaken pending prose, same drift
  class as DECL-003's Conflict Table note. Disposition unchanged (ALIGNED): a
  single overtaken expected-result row in a process document, consistent with
  the treatment applied to Guidance.
- **DEL-03-08-DECL-006** — MEMORY historical-drift notes (addendum 1, note not
  disposition): superseded test count ("8" → 13) and no MEMORY entry for the
  2026-07-10 mill_tolerance run (only in `_run_records`).

## Evidence-execution log

- **Re-executed (side-effect-free, addendum 9):**
  `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -p no:cacheprovider -q
  tests/test_section_properties.py` at the frozen worktree → **13 passed**.
  `git -C <FROZEN> status --porcelain` **empty before and after**. Redirected no
  build artifacts (pure-Python, cache provider disabled). Cited on all REQ/EXC
  evidence rows as `RE-EXECUTED at frozen SHA 551f84ef6`.
- **Cited as recorded, qualifier independently re-diffed:** the
  VERIFICATION_INDEX `PY-66` recorded pass is cited with the addendum-10
  content-identical qualifier. Per the W1 fan-in repair, this pilot re-ran the
  diff itself at the frozen worktree:
  `git diff --name-only e648462f1d0521e26df15d04a988391343018886
  551f84ef6be656f1603ce0acfa5e3935aa9683c7` over the cited path set →
  **empty over tests/, schemas/, fixtures/, tools/, api/, core/, apps/,
  examples/, validation/witness/; docs/ differs only in
  AGENTIC_DEVELOPMENT_WORKFLOW.md and TYPES.md** (ancestor relation confirmed
  via `git merge-base --is-ancestor`). The ledger qualifier string names this
  true path set; an earlier draft's `…, docs/)` literal (which dropped PY-66's
  exclusion clause and was false as written) was repaired in place on 8 cells
  (REQ-001..007, EXC-002). My own re-execution supersedes the recorded pass for
  the pass claim.
- **Validation artifacts inspected, not executed:** confirmed
  `validation/witness/fixtures/tp_phys_015_section_property_stress_witness.json`
  has zero references to `section_properties`/`calculator` (standalone hand-calc
  witness); confirmed `validation/benchmarks/stress/src/lib.rs` carries
  `calculator_ref == "core/section_properties/calculator.py"` as evidence
  metadata (not a Python execution). No Rust/`cargo` build was run (could not
  guarantee target-dir side-effect-freedom within the read-only frozen tree
  without a redirect I did not need for this deliverable).
- **In-tree ruling records confirmed present:**
  `execution/_Reconciliation/LifecycleCorrection/LIFECYCLE_CORRECTION_2026-07-02_2050/Decision_Log.md`
  (DecisionBasis for DECL-005); `Review_Findings.csv` rows PKG03-DEL-03-08-PKG02-001..003
  (ACCEPT_AS_IS/RESOLVED — DecisionBasis for REQ-003/004/006).

## Convention friction notes

- **Addendum 1 "deliverable-owned in-tree README":** ambiguous whether a
  code-surface README (`core/section_properties/README.md`) counts. Resolved
  against inclusion, following the sibling DEL-03-07 precedent (its
  `core/library_import/README.md` got no DECLARED_STATE row). Flagging in case the
  intended reading is broader.
- **STALE granularity for a single omitted input:** the binding set gives one
  controlled `STALE_SETUP_SPECIFICATION` value but no threshold for *how much*
  drift is stale. I scoped STALE to the authoritative enumerative surfaces
  (Spec, Datasheet) and used ALIGNED-with-note for illustrative surfaces
  (Guidance) and implementation-evidence surfaces (README). This mirrors the
  sibling calibration where a bare revision-citation lag stayed ALIGNED-with-note.
- **MECHANICS validation via `calculator_ref` metadata:** convention 7 classes
  numeric correctness as MECHANICS and the method bars promoting verification to
  validation, but the binding set does not say whether a benchmark/witness that
  *names* the production module as governing source (without executing it)
  satisfies the validation bar. I read it as not satisfied (execution binding
  required) → VERIFIED_NOT_VALIDATED. Flagged for reviewer confirmation.
- **AuthorityNeeded on ALIGNED rows with TBD residuals:** used OWNER/ENGINEERING
  to route the policy TBDs (optional-contributor requiredness, schema field
  placement, fixture-value policy, solver-integration policy, numeric validation)
  even where the row's disposition is ALIGNED, since the residual still needs a
  named authority; NO where no action is owed.

## Boundary-compliance statement

- All fences held. Discovery was READ-ONLY outside the two output files
  (`CLAIM_CONCORDANCE_DEL-03-08.csv`, `NOTES_DEL-03-08.md`). No lifecycle
  transition applied; no DAG mutation; no cross-project edit; no edit to any
  `_STATUS.md`, register, schema, test, or product file.
- No release-readiness, issuance, certification, sealing, professional-approval,
  or code-compliance claim is made anywhere in these outputs (F-PIP-1..5). REQ-006
  records the *deliverable's* code-neutrality as an evidence finding, not a
  compliance assertion by this pilot.
- All dispositions are agent judgments routed via `AuthorityNeeded`; none is
  phrased as an owner or engineering ruling. No `DEFERRED_AGENT_WORKFLOW`
  condition arose.
- Frozen evidence worktree `.claude-worktrees/piping-frozen-551f84ef6`:
  `git status --porcelain` empty before and after all reads and the sandboxed
  pytest re-execution. Writes confined to `RUN/WAVES/W1/`.
