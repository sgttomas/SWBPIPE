# Notes — DEL-07-08 Design-authoring state and comparison workspace (W3)

Deliverable `DEL-07-08` (PKG-07, IN_PROGRESS). Frozen tree `551f84ef6`. Ledger:
`CLAIM_CONCORDANCE_DEL-07-08.csv` (17 rows, 20 columns, RFC-4180/CRLF clean).
Requirement scheme `REQ-07-08-*` re-verified against the frozen `Specification.md`
Requirements table and `DELIVERABLE_INVENTORY.csv`: REQ-07-08-001 through -010 (10).
ClaimIDs use the addendum-12 fixed form `DEL-07-08-<TYPE>-NNN`; native IDs recorded
in `NormativeSource`.

Run-level `NormativeSource` alias (addendum 12): kit paths (`Specification.md`,
`Datasheet.md`, `Guidance.md`, `Procedure.md`, `_STATUS.md`, `MEMORY.md`,
`Dependencies.csv`, `Review_Findings.csv`, `_REVIEW.md`) are relative to the
deliverable folder `execution/PKG-07_Graphical User Interface and Engineering
Workflow/1_Working/DEL-07-08_Design-authoring state and comparison workspace/`;
code/test/app/schema/DAG/decomp paths are repo-root-relative within the frozen
worktree.

## 1. Histograms (recomputed from the CSV)

ClaimType histogram (17):
- REQUIREMENT: 10
- ACCEPTANCE: 0
- EXCLUSION: 1
- DECLARED_STATE: 6
- REMAINING_WORK: 0
- IMPLEMENTED_UNMAPPED: 0

Disposition histogram (17):
- ALIGNED: 13
- STALE_SETUP_SPECIFICATION: 4

## 2. Self-flagged rows

- **DEL-07-08-REQ-004** (ALIGNED, MEDIUM) — grain call under calibration item 6.
  The proposed/validated/accepted/audited operation distinction is preserved at
  the composition-contract grain (`_operation_review_row` review_state
  `held_for_user_acceptance` vs `explicit_user_acceptance_recorded`;
  `can_be_represented_as_accepted_operation` requires an upstream audit record;
  `workspace_application_status=not_applied`). The requirement's own wording
  defers exact UI state labels "TBD pending upstream operation contracts," so I
  chose ALIGNED-at-contract-grain rather than PARTIALLY_IMPLEMENTED. Reviewer
  eyes: confirm the contract-vs-UI-label grain is the intended read.
- **DEL-07-08-REQ-005** (ALIGNED, MEDIUM) — grain/evidence call. State/run
  browser records are first-class with `persistence_hash_boundary` and per-row
  `hash_boundary`, but the DEV-001 PKG-02 audit recorded that DEL-02-05
  persistence/round-trip/hash-invalidation compatibility for this GUI layer is
  *indirect* (finding `PKG07-DEL0708-PKG02-001`), later `ACCEPT_AS_IS` /
  `RESOLVED`. Chose ALIGNED (the human disposition accepts the indirect posture)
  at contract grain; exact PKG-14 data-contract shape is a requirement-own TBD.
- **DEL-07-08-REQ-006** (ALIGNED, MEDIUM) — grain call under calibration item 6.
  Structured diagnostic fields and warning classes are preserved where supplied,
  but the six named DEL-00-06 warning classes
  (SOLVE_BLOCKING/RULE_CHECK_BLOCKING/PROVENANCE/ASSUMPTION/NONLINEAR/IP_BOUNDARY)
  are not all exercised in the preview fixtures (the fixture uses
  `warning_class=incomplete_data`). Chose ALIGNED-at-contract-grain (the "where
  available" clause is conditional/vacuous for the unexercised subset) rather
  than PARTIALLY.
- **DEL-07-08-REQ-009** (ALIGNED, MEDIUM) — grain/evidence call. Architecture
  basis (TypeScript/React/Vite/Tauri 2) is followed by `apps/desktop` and
  `DesignWorkspacePanel.tsx`; the Three.js viewport-facing binding is not
  exercised by this contract-facing slice (overlays are descriptor-only), and
  exact package versions are a requirement-own TBD. Verification for the React
  stack is recorded (DEC-025 sweep) and marked not re-executed at the frozen SHA
  (node/build tooling would write into the frozen tree).
- **DEL-07-08-DECL-003** (Guidance, STALE_SETUP_SPECIFICATION, MEDIUM) — judgment
  call. Guidance's "no component implementations or fixtures available ... remain
  TBD until an implementation brief" Examples framing is overtaken by the
  implemented slice and invented fixtures, but the five conservative principles
  remain valid. Encoded STALE under widened addendum 4 (setup-era future-tense
  prose), naming the surviving-valid principles in-row. A reviewer could read the
  Examples section as historical-not-stale; flagged for that reason.
- **DEL-07-08-DECL-004** (Procedure, STALE_SETUP_SPECIFICATION, MEDIUM) — judgment
  call. The Procedure declares a "later implementation pass ... does not create
  product code" posture and "No implementation records exist in this setup
  workflow." Unlike some setup procedures the STALE rests on the setup-only
  framing no longer describing the frozen implemented slice (widened addendum 4),
  not a missing-tool defect. A reviewer could reasonably read a self-limited
  setup-production procedure as historical-not-stale; flagged.
- **DEL-07-08-DECL-006** (MEMORY, ALIGNED, MEDIUM) — calibration item 9 judgment
  call. `MEMORY.md` opens with an *undated* header block (a current declaration
  under item 9) describing the implemented core Python builder; it is faithful to
  `core/gui/design_workspace/engine.py` and the invented fixtures, and its
  boundary choices still hold — so ALIGNED, not STALE. The undated header does
  not enumerate the later desktop app surface (`design-workspace` panel); that
  surface family is supplied by the *in-file* dated 2026-06-17 TP-UNITS entry, so
  I applied the item-9 ALIGNED-with-note allowance and named the correcting
  in-file entry in-row. Flagged because a reviewer might read the undated header's
  omission of the desktop family as widened-addendum-4 incompleteness.

## 3. Evidence-execution log

Re-executed side-effect-free at frozen SHA `551f84ef6`
(`PYTHONDONTWRITEBYTECODE=1`, pytest `-p no:cacheprovider`; `git -C <frozen>
status --porcelain` **empty before and after**):
- `python3 -m pytest -p no:cacheprovider tests/test_design_authoring_comparison_workspace.py`
  → **4 passed** (exit 0). The four tests assert: deterministic composition and
  review-record preservation across all six panels (`design_knowledge_panel`,
  `constraint_warning_panel`, `state_run_browser`, `comparison_tables`,
  `graphical_overlays`, plus `workspace_hash`/`persistence_hash_contract` /
  per-row `hash_boundary`); operation review requires an explicit user-acceptance
  record (`held_for_user_acceptance` vs `explicit_user_acceptance_recorded`,
  `workspace_application_status=not_applied`); missing inputs and mutation
  signals stay visible as diagnostics; and output boundary language makes no
  prohibited compliance/approval claims. Cited on REQ-001..008, REQ-010, EXC-001.
  This is the `VERIFICATION_INDEX.csv` PY-25 surface, which independently records
  the file CONTENT_IDENTICAL at the frozen SHA.

Cited as recorded (NOT re-executed, per the GUI/node-tooling caution):
- DEC-025 evidence sweep
  `validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json` at commit
  `e648462f1` (ancestor of frozen SHA): `python_pytest`, `desktop_vitest`,
  `desktop_playwright_e2e`, `desktop_production_build` all pass, overall pass,
  clean tree. Marked `not re-executed at frozen SHA 551f84ef6` on every row that
  cites it (REQ-001, REQ-009, REQ-010, DECL-006). I did not run vitest/playwright/
  cargo or the desktop production build in the frozen tree (they would write
  node/build artifacts into it); no addendum-12 byte-identical out-of-tree copy
  pattern was used. I did **not** assert the addendum-10 content-identical
  qualifier for the TypeScript/Rust paths (no `diff` run over them), so those
  legs carry only the recorded-pass + not-re-executed marker.
- MEMORY 2026-06-17 dated figures (App Vitest 56/56, R2/R3 Playwright smoke 14/14,
  full desktop Vitest 399/399, production build) cited as recorded corroboration
  on REQ-010 / DECL-006, not re-executed.
- DEV-001 PKG-02 downstream audit finding `PKG07-DEL0708-PKG02-001`
  (`Review_Findings.csv`; `_REVIEW.md`): `HumanDisposition=ACCEPT_AS_IS`,
  `Status=RESOLVED` (2026-06-06 human disposition). Used as `DecisionBasis` on
  REQ-005.

Porcelain at the frozen root `.claude-worktrees/piping-frozen-551f84ef6` (HEAD
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`) was verified **empty before and
after** the pytest re-execution.

## 4. Convention-friction notes

- **Rev-0.7 authority-pointer drift (calibration item 1) — owner-calibration
  caveat, recorded once here.** `Specification.md` Standards table cites
  `execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.7` as accepted basis;
  `_CONTEXT.md` ("Accepted Revision: 0.7", "Decomposition Revision: revision
  0.7"), `_REFERENCES.md` ("Accepted revision 0.7 current decomposition basis"),
  and the *dated* 2026-06-04 MEMORY entry ("revision 0.7 plus approved DAG-006")
  carry the same pointer. The frozen
  `execution/_Decomposition/SOFTWARE_DECOMP.md` header is `revision: 0.8`,
  `status: current_basis`. This is pure authority-pointer drift. The
  Specification is a census DECLARED_STATE surface and is *already*
  STALE_SETUP_SPECIFICATION on the separate setup-only-vs-implemented-slice
  ground; per calibration item 1 the rev-0.7 drift fact is recorded in-row on
  DECL-001 (`RemainingWork`), not as a new disposition. `_CONTEXT.md` /
  `_REFERENCES.md` are **not** addendum-1 census surfaces (census = four-doc kit
  + `_STATUS.md` + `MEMORY.md` + in-tree READMEs), and the MEMORY pointer is
  *dated* (historical, protected under addendum 1), so no additional census row
  takes STALE for the rev pointer. `AuthorityNeeded=NO` applies to the pure
  pointer drift itself; DECL-001 routes `OWNER` on the setup-only ground (an R5
  document-repair candidate that would also fold in the rev-0.7 refresh).
- **DecisionBasis path drift on REQ-002 (convention 7).** The kit's REQ-07-08-002
  `NormativeSource`/basis cites `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md
  / DAG2-RD-015`, which does **not** resolve — the edge review lives at
  `execution/_DAG/DAG-002/DAG-002_EdgeDispositionReview.md`, and DAG2-RD-015 is
  also carried in `execution/_DAG/DAG-006/DependencyEdges.csv` /
  `APPROVAL_RECORD.md`. I recorded the resolvable path in `DecisionBasis` and
  disclosed the kit's non-resolving path in-cell (requirement rows never take
  STALE per convention 1; this is a pointer note, not a disposition).
- **Setup-kit-vs-implemented-slice STALE (widened addendum 4).** All four
  four-document kit surfaces (Specification, Datasheet, Guidance, Procedure)
  declare a setup-only / "does not create product code" / "TBD until
  implementation exists" state, while the frozen tree carries a committed
  implemented slice: `core/gui/design_workspace/engine.py` (SURF-073, six-panel
  composer), `tests/test_design_authoring_comparison_workspace.py` (PY-25),
  `apps/desktop/src/features/design-workspace/DesignWorkspacePanel.tsx`
  (SURF-015), `apps/desktop/src/features/knowledge/KnowledgePanel.tsx`
  (SURF-025). Requirement substance is carried on the ALIGNED REQ rows
  (convention 1); the setup-only *declaration* staleness is ledgered on the four
  DECL kit rows and routed `AuthorityNeeded=OWNER` as R5 document-repair
  candidates. This follows the W3 DEL-07-07 precedent.
- **SourceReliability keyed to the weakest load-bearing leg (calibration item
  13).** All REQUIREMENT/EXCLUSION rows are `UNVERIFIED`: the load-bearing
  verification evidence is agent-generated (the re-executed PY-25 contract test
  and/or the recorded DEC-025 sweep) with no human disposition at the evidence
  grain. The DEV-001 `ACCEPT_AS_IS` human disposition covers the PKG-02
  persistence/hash compatibility *finding* on REQ-005, not the pytest/vitest/
  playwright evidence, so it does not lift any row to `REVIEWED`. DECLARED_STATE
  prose rows are `NOT_APPLICABLE` (addendum 6).
- **AuthorityNeeded as an adjudication router, not a work queue (calibration item
  14).** No requirement carries `OWNER`/`ENGINEERING`: the contract-level TBDs
  (REQ-004 exact operation-state labels; REQ-005 exact PKG-14 data-contract
  shape; REQ-006 unexercised warning-class subset; REQ-009 exact versions;
  REQ-010 exact harness/thresholds) are deferred by the requirements' own wording
  with no numeric/authority claim being promoted, so `AuthorityNeeded=NO`. Only
  the four STALE kit rows route `OWNER` (document-repair). No gate-named tokens
  (e.g. a `D-##` requirement token) appear in this deliverable's requirements
  (the sole gate token is the bootstrap `(gated: D-41)` in `_STATUS.md`,
  addendum-2-excluded).
- **Bootstrap `_STATUS` scoping (addendum 2 / calibration items 5 & 11).**
  DEL-07-08's `## Remaining` holds *only* the seeded `(gated: D-41)` item.
  Transcribed byte-exact into the DECL-005 `RecordedRemaining` (kept `§§6–8` and
  the en-dash; no transliteration). Gate/source cells use the exclusion variant
  (`NONE_RECORDED`, scoped to non-bootstrap content) — the bootstrap item is not
  annotated into gate/source cells. No standalone REMAINING_WORK row (addendum
  2). Matches `DELIVERABLE_INVENTORY.csv` (`NonBootstrapItems=NONE`,
  `SelectableUnderCurrentLoop=NO`).
- **Zero ACCEPTANCE rows (addendum 12 grain).** The Specification "Verification"
  table maps each requirement to a future-tense/TBD verification approach that
  merely restates the requirement ("automated GUI smoke tests are TBD until
  implementation exists", "Test ... once those contracts are available"); the
  Datasheet "Conditions" and Procedure "Verification" checks (scope / dependency
  / mutation-boundary / warning / comparison-boundary / public-private / test)
  likewise restate requirements/boundaries. None add independent named acceptance
  grain (there is no VER-07-08-* scheme), so no mirrored ACCEPTANCE rows were
  emitted.
- **Zero IMPLEMENTED_UNMAPPED rows.** All material surfaces in DEL-07-08's orbit
  are already deliverable-attributed in `IMPLEMENTATION_SURFACES.csv`: SURF-073
  `core/gui/design_workspace` (DEL-07-08), SURF-015
  `apps/desktop/src/features/design-workspace` (DEL-07-08), SURF-025
  `apps/desktop/src/features/knowledge` (DEL-07-08;DEL-08-05), SURF-010
  `apps/desktop/src/features/agent-proposals` (multi-DEL incl. DEL-07-08). Shared
  schema/crate surfaces (SURF-079 viewport_editor, SURF-174/190/197/209 schemas,
  SURF-225 validate_dependencies_schema.py) are cross-package and already mapped;
  none are unmapped in this deliverable's orbit.
- **Verification != validation (shared hash/audit semantics with PKG-08).** All
  GUI/GOVERNANCE/WORKFLOW/INTEROP rows carry `ValidationEvidence=NOT_APPLICABLE`
  with the claim-class reason; no unit/e2e suite was promoted to engineering
  validation. Design-state/comparison claims touch hash/audit semantics shared
  with PKG-08 (workspace_hash, persistence_hash_contract, operation audit
  routing) and PKG-04 (diagnostics) — these are recorded as concordance facts for
  THIS deliverable's GUI claims only; no PKG-08 or PKG-04 row was re-adjudicated.

## 5. Boundary-compliance statement

All fences held. Discovery was read-only: the only writes were the two output
files under `RUN/WAVES/W3/` (`CLAIM_CONCORDANCE_DEL-07-08.csv`,
`NOTES_DEL-07-08.md`) plus the uniquely named generator script
`build_csv_DEL-07-08.py` in the session scratchpad (outside every project tree).
No `_STATUS.md`, register, DAG, product, or cross-project file was edited; no
lifecycle transition was applied (`LIFECYCLE_REASSESSMENT_REQUIRED` not used); no
DAG mutation. No release-readiness, issuance, certification, sealing,
professional-approval, or code-compliance claim appears anywhere in the outputs
(F-PIP-1..5). Dispositions are agent judgments, routed via `AuthorityNeeded`,
never phrased as owner or engineering rulings. No `DEFERRED_AGENT_WORKFLOW`
disposition was needed. The frozen evidence worktree
(`.claude-worktrees/piping-frozen-551f84ef6`, HEAD
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`) porcelain was verified **empty before
and after** the pytest re-execution; no writes were made under the frozen tree,
including git-ignored paths.

No STOP-worthy contradiction found.
