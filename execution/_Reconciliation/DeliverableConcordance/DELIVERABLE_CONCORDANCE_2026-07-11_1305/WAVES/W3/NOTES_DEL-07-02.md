# NOTES — DEL-07-02 Model tree and property inspector (R2 wave W3)

Ledger: `WAVES/W3/CLAIM_CONCORDANCE_DEL-07-02.csv` (20 rows, RFC-4180 CRLF,
header byte-identical to sibling W3 ledgers). Frozen evidence worktree
`.claude-worktrees/piping-frozen-551f84ef6` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
Package PKG-07; deliverable status IN_PROGRESS.

Run-level `NormativeSource` alias (addendum 12): "Specification.md" cites the
kit at `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/`.

## Histograms (recomputed from the CSV)

ClaimType:
- REQUIREMENT: 8
- EXCLUSION: 2
- DECLARED_STATE: 6
- REMAINING_WORK: 4
- (total: 20)

Disposition:
- ALIGNED: 16
- STALE_SETUP_SPECIFICATION: 4
- (total: 20)

No ACCEPTANCE, IMPLEMENTED_UNMAPPED, PARTIALLY_IMPLEMENTED, UNKNOWN,
ACCEPTED_DIVERGENCE, or LIFECYCLE_REASSESSMENT_REQUIRED rows (see census notes).

## Census decisions

- **8 REQUIREMENT rows**: DEL-07-02-RQ-001..008 re-verified against the frozen
  Specification.md requirements table (matches the R1 inventory
  `RequirementIDs`). ClaimIDs use addendum-12 fixed form `DEL-07-02-REQ-NNN`;
  the native RQ id is recorded in `NormativeSource` (assignment direction).
- **No ACCEPTANCE rows**: the Specification "Verification", "Verification
  Coverage Slots", and Procedure "Verification" tables are verification-area
  restatements of the requirements, not named acceptance criteria at
  addendum-12 grain. Per the row-census rule (verification tables that merely
  restate requirements do NOT get mirrored ACCEPTANCE rows) none are emitted.
  (Contrast the R0b exemplar DEL-07-05, whose Specification carried explicit
  `VER-07-05-NNN` acceptance IDs.)
- **2 EXCLUSION rows**: EXC-001 = durable-boundary portion of Specification
  Scope paragraph 2 (no engineering defaults / no protected standards content /
  no professional-approval/code-compliance claim); EXC-002 = Guidance
  adjacent-deliverable scope split (specialized editors -> DEL-07-03;
  missing-data blocking UX -> DEL-07-04). The setup-era portion of the Scope
  sentence (does not implement GUI source, edit tests, select libraries) is
  overtaken and ledgered on DECL-001, not double-counted as an exclusion.
- **6 DECLARED_STATE surface rows** (addendum 1 census): Specification,
  Datasheet, Guidance, Procedure, `_STATUS.md`, `MEMORY.md`. No deliverable-owned
  in-tree README exists (checked `core/gui/model_tree/`,
  `apps/desktop/src/features/model-tree/`, `.../model-workspace/`), so no README
  DECL row. `_CONTEXT.md`, `_REFERENCES.md`, `_SEMANTIC*.md`, `_REVIEW.md`,
  `_DEPENDENCIES.md` are not addendum-1 census surfaces and are used only as
  evidence.
- **4 REMAINING_WORK rows**: the 4 real ungated non-bootstrap residuals of the
  5-item `## Remaining` (modulus_basis_records model-level GUI emission; broaden
  unit pickers; backfill unit tests/factoring; broader canvas gestures / full
  UX). The 5th item — the seeded `(gated: D-41)` bootstrap — is transcribed
  verbatim ONLY into the `_STATUS.md` surface row (DECL-005) `RecordedRemaining`
  and excluded from residual/gate/selectability analysis (addendum 2). The R1
  inventory `SelectableUnderCurrentLoop=YES` at inventory grain is reproduced
  per-row mechanically: all 4 residuals are UNGATED and the deliverable is
  IN_PROGRESS, so each REM row and DECL-005 carry `SelectableUnderCurrentLoop=YES`
  (convention 6, DAG/lifecycle/gate only; the owner suspension is a run-level
  caveat, not applied per-row).
- **No IMPLEMENTED_UNMAPPED rows**: the two material surfaces in this
  deliverable's orbit (`core/gui/model_tree` = SURF-075, attributed DEL-07-02
  only; `apps/desktop/src/features/model-tree` = SURF-030, attributed to
  DEL-07-02 among others) are already mapped in `IMPLEMENTATION_SURFACES.csv`;
  none of the 8 `NONE_FOUND`-attribution surfaces are in this orbit.

## Cross-reference recorded (DEL-00-05-REQ-004 property-inspector architecture home)

W1 ledgered `DEL-00-05-REQ-004` PARTIALLY_IMPLEMENTED (AuthorityNeeded=OWNER),
noting the property-inspector *architecture* is absent from every DEL-00-05
surface and "no record delegates it to DEL-07-02", with RemainingWork "Define
property-inspector behavior at the architecture level within DEL-00-05, or
record an accepted delegation of inspector-behavior architecture to DEL-07-02".

Concordance fact from the DEL-07-02 side (frozen SHA): DEL-07-02's kit
implements and owns the inspector *slice* (RQ-002 "where this slice owns the
inspector surface"; RQ-006 command/query boundary under AB-00-03/AB-00-05) but
records NO delegation-of-architecture home received from DEL-00-05 — neither the
four-document kit, `_CONTEXT.md` Architecture Basis Injection, nor `Dependencies.csv`
carries such a delegation. Both endpoints therefore leave the architecture-home
question open in the same direction. Per the brief, this pilot does not resolve
the homing question: it is recorded on DEL-07-02-REQ-002 with
`AuthorityNeeded=OWNER` (RemainingWork cross-references DEL-00-05-REQ-004) and
surfaced here. REQ-002's *implementation substance* is ALIGNED; the OWNER route
is the architecture-attribution adjudication, not an implementation gap
(consistent with W2 calibration item 14: adjudication router, not a work queue).

## Self-flagged rows

- **DEL-07-02-REQ-002** — (a) Forward-looking capability-grain call (W1
  calibration item 6): marked ALIGNED at requirement/contract grain rather than
  PARTIALLY_IMPLEMENTED; the landed inspector covers the enumerated entity field
  groups this slice owns, while full model-tree/property-editor UX is carried as
  residual REM-004 and rule-pack/private-library specialized editors are DEL-07-03
  scope. (b) Carries the DEL-00-05-REQ-004 architecture-home cross-reference and
  the `AuthorityNeeded=OWNER` route (above).
- **DEL-07-02-REQ-004** — Grain call: ALIGNED at contract grain because the
  engine emits explicit missing-value findings and fills no defaults, but the
  requirement's "rule-check-required" clause depends on rule-pack integration
  (PKG-06 / DEL-07-04 orbit) and missing-data *blocking* UX is DEL-07-04 scope.
  Reviewer eyes welcome on whether that conditional clause warrants a partial.
- **DEL-07-02-DECL-006** — Census-inclusion judgment: `MEMORY.md` here is
  entirely dated log entries (2026-05-08..2026-06-18) with NO undated header/state
  block (contrast W2 calibration item 9's "Boundaries Preserved"-style blocks). A
  strict reading of addendum 1 ("MEMORY.md when [it carries] current-state
  declarations") could omit the row; I include it as ALIGNED with an explicit
  in-row note that no undated current-state declaration exists to be stale, for
  reviewer visibility. If the census convention is that MEMORY without an undated
  block gets no row, drop DECL-006 (histograms would become DECLARED_STATE 5,
  ALIGNED 15, total 19).

## Evidence-execution log

- **Re-executed side-effect-free** (addendum 9), frozen porcelain empty BEFORE
  and AFTER:
  - `PYTHONDONTWRITEBYTECODE=1 python3 tests/test_model_tree_property_inspector.py`
    -> exit 0 (script main).
  - `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -p no:cacheprovider -q
    tests/test_model_tree_property_inspector.py` -> `1 passed`.
    Asserts the DEL-07-02 contract: deterministic tree/inspector,
    `selection_resolved`, `PROPERTY_VALUE_UNRESOLVED` diagnostic present,
    `mutation_policy=application_service_command_intents_only`,
    `software_makes_compliance_claim` False, FORBIDDEN terms
    (certified/sealed/code compliant/professional acceptance) absent.
  - `git -C FROZEN status --porcelain` returned empty (count 0) immediately
    before and after each re-execution and after all read/grep operations.
- **Addendum-10 content-identical qualifier** — actually diffed:
  `git diff e648462f1d0521e26df15d04a988391343018886 551f84ef6be656f1603ce0acfa5e3935aa9683c7`
  over `apps/desktop/src/features/model-tree/`, `apps/desktop/src/features/model-workspace/`,
  `apps/desktop/src/features/load-cases/`, `apps/desktop/src/types.ts`, and
  `core/gui/model_tree/` returned **empty** (0 lines). The sweep commit
  `e648462f1d05` is an ancestor of the frozen SHA (verified
  `merge-base --is-ancestor` -> yes). Hence the qualifier string on the REQ rows
  is load-bearing and true; no differing path is silently covered (W1 calibration
  item 4 discipline).
- **Cited recorded passes** (not re-executed at frozen SHA 551f84ef6): DEC-025
  five-surface sweep `validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json`
  at commit `e648462f1d05...` (working tree clean; `overall_status=pass`;
  `desktop_vitest` pass, `desktop_playwright_e2e` pass, `python_pytest` pass,
  `cargo_crate_sweep` pass, `desktop_production_build` pass). Desktop Vitest /
  Playwright legs cannot be re-executed without risking node/npm writes into the
  frozen tree, so they are cited as recorded passes with the exact marker plus
  the addendum-10 qualifier above.
- **Direct inspection / grep at frozen SHA** (side-effect-free): confirmed the
  frozen `SOFTWARE_DECOMP.md` header is `revision: 0.8`, `status: current_basis`
  (vs the kit's rev-0.7 pointer); confirmed `schemas/model.schema.yaml` defines a
  model-level `modulus_basis_records` table while `grep -r modulus_basis_records
  apps/desktop/src` returns nothing (REM-001 residual real); confirmed
  `schemaSlotEmission.test.tsx` `SLOT_KEYS` emits per-load-case `modulus_basis_ref`
  only; confirmed `Review_Findings.csv` is header-only (no overtaken review-prose
  situation — W1 calibration item 3 does not apply here).

## Convention friction notes

- **W1 calibration item 1 (rev-0.7 pointer drift)**: the Datasheet References row
  (and, off-census, `_CONTEXT.md` / `_REFERENCES.md`) cites decomp revision 0.7;
  frozen header is 0.8. Encoded STALE on DECL-002 with drift facts in-row and
  `AuthorityNeeded=OWNER` because the Datasheet also carries overtaken TBD
  registers (Conditions / Setup-Slot / Open-Question rows all TBD while the slice
  is implemented) — the item-1 "OWNER only where the kit also carries overtaken
  TBD registers" branch. Pure pointer drift alone would be NO. Owner-calibration
  caveat: the rev-0.7 -> 0.8 drift is a corpus-wide pointer-staleness pattern
  already adjudicated STALE-side in W1/W2, not a per-deliverable finding.
- **DECL routing (item-1 OWNER vs NO)**: Specification and Datasheet DECL rows are
  OWNER (setup-only declaration + overtaken TBD registers / rev pointer — R5
  document repair). Guidance and Procedure DECL rows are STALE with
  `AuthorityNeeded=NO`, matching the adopted W2 precedent (DEL-05-01 DECL-003/004):
  their core principles/steps remain accurate for the implemented slice; only the
  setup-era framing/TBD-library prose is overtaken, a lighter document refresh.
- **Bootstrap `_STATUS` cell scoping (W2 calibration item 11)**: DECL-005 uses the
  exclusion variant — `GateOrStageConstraint` and `RemainingSource` are scoped to
  the 4 non-bootstrap residuals (`UNGATED; UNGATED; UNGATED; UNGATED` and the 4
  sources); the bootstrap item is NOT annotated into the gate/source cells. Its
  verbatim text (byte-exact, `§§6–8`/en-dash preserved per W1 item 5) appears only
  in `RecordedRemaining`.
- **SourceReliability leg-keying (W2 calibration item 13)**: requirement,
  exclusion, and remaining-work rows key to `UNVERIFIED` (not REVIEWED). Their
  load-bearing verification legs are agent-run desktop Vitest/Playwright and the
  agent-run contract test, recorded and agent-audited but with no standing human
  disposition on the DEL-07-02 evidence (the 2026-06-06 human disposition record
  closed findings for other PKG-07 deliverables; the 2026-06-06 CHECKING set was
  later reversed to IN_PROGRESS by K-CONFLICT-1). Validation legs are
  NOT_APPLICABLE (GUI/governance claim classes). DECLARED_STATE prose rows are
  `NOT_APPLICABLE` per addendum 6 (overrides item-13 leg-keying for prose).
- **AuthorityNeeded as router (W2 calibration item 14)**: exactly one OWNER route
  (REQ-002, the DEL-00-05 architecture-home adjudication) plus the four STALE DECL
  document-repair routes (2 OWNER, 2 NO). No gate-named tokens (e.g. D-38) appear
  in this deliverable's residuals; the only gate token is the D-41 bootstrap,
  which is excluded from analysis.
- **Verification != validation**: no unit/e2e test is promoted to engineering
  validation; every requirement row's ValidationEvidence is explicitly
  NOT_APPLICABLE for the GUI/governance claim class (H4 desktop verification
  posture governs).
- **F-PIP-1..5**: no release-readiness, issuance, certification, sealing,
  professional-approval, or code-compliance claim appears anywhere in the ledger
  or these notes.

## Boundary-compliance statement

All fences held. Discovery was read-only outside the two W3 output files
(`CLAIM_CONCORDANCE_DEL-07-02.csv`, `NOTES_DEL-07-02.md`). No lifecycle
transition was applied (`LIFECYCLE_REASSESSMENT_REQUIRED` was not needed and is
not used); no DAG mutation; no cross-project edit; no edit to any `_STATUS.md`,
register, or product file. The frozen evidence worktree porcelain was empty
before and after every read, grep, diff, and side-effect-free re-execution
(bytecode suppressed, pytest cache disabled, no build artifacts written into the
tree). Dispositions are agent judgments, routed via `AuthorityNeeded`, never
phrased as owner or engineering rulings. No STOP-worthy contradiction was found:
the frozen register's D-41 `AWAITING_RULING` vs the run's RULED activation is the
codified ruling-after-freeze mechanic (RUN_BASIS), not a conflict.
