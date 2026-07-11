# NOTES — DEL-02-01 Canonical domain model schema (W1)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W1. Frozen tree
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Ledger:
`WAVES/W1/CLAIM_CONCORDANCE_DEL-02-01.csv` (20 rows).

## Histograms (recounted from the CSV)

**Disposition:**

| Disposition | Count |
|---|---|
| ALIGNED | 16 |
| STALE_SETUP_SPECIFICATION | 4 |
| **Total** | **20** |

**ClaimType:**

| ClaimType | Count |
|---|---|
| REQUIREMENT | 12 |
| EXCLUSION | 2 |
| DECLARED_STATE | 6 |
| **Total** | **20** |

(ACCEPTANCE = 0, REMAINING_WORK = 0, IMPLEMENTED_UNMAPPED = 0 — see census
notes below.)

## ClaimID → requirement mapping (REQ-02-01-* scheme, self-identifying)

REQ-02-01-01..12 map one-to-one to DEL-02-01-REQ-001..012 (numeric suffix
preserved). Exclusion rows DEL-02-01-EXC-001/002 and declared-state rows
DEL-02-01-DECL-001..006 are run-local ClaimIDs (addendum 12 fixed form
`DEL-02-01-<TYPE>-NNN`).

## Row census rationale

- **12 REQUIREMENT rows** — one per current requirement ID in
  `Specification.md` (REQ-02-01-01..12). All ALIGNED: `schemas/model.schema.yaml`
  (46 KB, JSON Schema 2020-12, implemented at frozen SHA) plus the two pytest
  suites satisfy each requirement's substance.
- **0 ACCEPTANCE rows** — judgment call (self-flagged). The `Specification.md`
  requirement table carries a per-requirement Verification column that merely
  restates each requirement; the standalone Verification section and the
  Acceptance Crosswalks (Object-Family, Unit Applicability, Provenance/Status,
  Diagnostics) are per-requirement expansions, and REQ-02-01-12 is itself the
  meta-acceptance requirement (layered checks). Under the brief's tightening
  ("verification tables that merely restate requirements do NOT get mirrored
  ACCEPTANCE rows") none rise to a distinct acceptance gate, so no mirrored
  ACCEPTANCE rows were created. The R0b exemplar DEL-07-05 carried 5 ACCEPTANCE
  rows, but it had discretely ID'd VER-07-05-001..005 items (document review,
  boundary review, dependency-register validation, semantic-setup review,
  future-implementation test) that DEL-02-01 does not separately enumerate;
  the exemplar predates the adopted addenda.
- **2 EXCLUSION rows** — the seven `Out of scope` bullets grouped into two
  coherent exclusion claims following the DEL-07-05-C01 grouping precedent:
  EXC-001 = the six implementation/behavior deferrals (solver, GUI editor,
  library population, rule-pack evaluator, report generator, physical
  package/container); EXC-002 = the professional-boundary exclusion
  (no code-compliance/certification/approval/authentication claim). Both ALIGNED.
- **6 DECLARED_STATE rows** — addendum-1 census: one per four-document kit
  surface (Specification, Datasheet, Guidance, Procedure) + `_STATUS.md` +
  `MEMORY.md`. No deliverable-owned in-tree README exists (folder listing
  confirmed), so no README row. `_REVIEW.md` / `Review_Findings.csv` are
  mechanical-review evidence, not in the addendum-1 census, and are cited as
  evidence rather than given their own declared-state row.
- **0 REMAINING_WORK rows** — the only `## Remaining` item is the seeded
  `(gated: D-41)` concordance bootstrap, which per addendum 2 is recorded
  verbatim ONLY in the `_STATUS.md` surface row's `RecordedRemaining`
  (DEL-02-01-DECL-005) and excluded from all residual/gate/selectability
  analysis. No evidence-backed omitted residual met the bar for its own row
  (see self-flagged candidates below).
- **0 IMPLEMENTED_UNMAPPED rows** — `schemas/model.schema.yaml` (SURF-190) and
  `fixtures/domain` (SURF-151) are both mapped to DEL-02-01 (and many others)
  in `IMPLEMENTATION_SURFACES.csv`, so they are implementation evidence, not
  unmapped surfaces. No unmapped material surface sits in this deliverable's
  orbit.

## Key finding — four-document kit is setup-era relative to the implemented schema

The central concordance result: the four-document kit (Specification, Datasheet,
Guidance, Procedure) repeatedly declares `schemas/model.schema.yaml` as a
*future / not-yet-implemented* artifact "outside the current write scope," but at
the frozen SHA the schema **is** implemented (46 KB, all required `$defs`,
DEC-068 load-generation slots) and `docs/TYPES.md` Section 8 already carries the
canonical-schema vocabulary. The four declared-state prose rows therefore take
`STALE_SETUP_SPECIFICATION` (widened definition, addendum 4). The requirement
*substance* is unaffected — the 12 REQ rows are ALIGNED. This mirrors the
DEL-07-05 pattern (stale declared-state prose over aligned requirements). All
four are flagged as R5 repair candidates with `AuthorityNeeded=OWNER`; per the
fences this is recorded as a disposition, never applied, and never phrased as an
owner/engineering ruling.

## Parallel-surface orientation item (from loop Receipt 10)

Confirmed and encoded. The canonical schema uses `g_factors.{x,y,z}` and
`exposed_element_refs` (schema lines ~590–640; pinned by
`tests/test_model_schema.py` DEC-068 assertions and
`tests/test_physical_to_analytical_transform.py`). The GUI+applier session shape
uses `g_factor_x|y|z` and `exposed_pipe_refs`
(`apps/desktop/src/features/model-tree/schemaSlotEmission.test.tsx`;
`core/model_operations/operation_applier/src/lib.rs` `field_path` emission;
`fixtures/model_operations/contract_corpus/case_70_*`). These are parallel
surfaces with tests pinning the correspondence — the applier translates session
field paths to canonical schema paths. For DEL-02-01 the canonical schema is the
validation authority and is internally consistent, so per the deliverable's own
requirements this is **ALIGNED with a note**, not `IMPLEMENTED_DIFFERENTLY`: the
divergence lives entirely in downstream consumers (PKG-05/07/16), which are a
translated serialization surface, not a schema-validation bypass. Encoded on
REQ-02-01-02 (coverage) and REQ-02-01-10 (non-bypass), both self-flagged.

## Self-flagged rows

- **DEL-02-01-REQ-002** — parallel-surface note: canonical `g_factors.{x,y,z}` /
  `exposed_element_refs` vs downstream session `g_factor_x|y|z` /
  `exposed_pipe_refs`. Reviewer eyes on the ALIGNED-with-note vs
  IMPLEMENTED_DIFFERENTLY call (I read it as ALIGNED because the divergence is
  downstream of DEL-02-01's artifact).
- **DEL-02-01-REQ-010** — same parallel surface encoded as a translation, not a
  bypass; and the MEDIUM confidence reflects that end-to-end non-bypass
  enforcement verification is owned by DEL-02-03/PKG-10, not exercised within
  DEL-02-01's own suite.
- **DEL-02-01-REQ-009 / REQ-012** — MEDIUM confidence: deterministic
  serialize round-trip and migration are scoped to DEL-02-05 (accepted
  architecture boundary, not ACCEPTED_DIVERGENCE — no named ruling *permits* a
  deferral, it is a scope split, so ALIGNED per addendum 11). Reviewer may wish
  to confirm the round-trip "documented deferral" reading of REQ-02-01-12.
- **ACCEPTANCE-row census (0)** — flagged as a judgment call the conventions
  leave open (see census rationale). A reviewer preferring the exemplar grain
  could add ACCEPTANCE rows for the Procedure Verification-table gates.
- **DEL-02-01-DECL-003 (Guidance)** — MEDIUM confidence
  `STALE_SETUP_SPECIFICATION`: only the setup-era framing (TYPES.md-update-
  pending, draft-vocabulary, future-schema) is stale; the principles,
  trade-offs, and the two still-open conflict-table items (C-02-01-001
  OBJ-001 vs OBJ-012; C-02-01-002 stale revision pointers) remain valid.
  Judgment call on whether a partially-stale guidance surface should take the
  staleness disposition or ALIGNED-with-note.
- **Candidate omitted residuals not given rows** — (a) MEMORY conflict items
  C-02-01-001/002 have a home (Guidance conflict table) and proposed
  dispositions with `Human ruling: TBD`, so they are open governance conflicts
  surfaced on the DECL rows rather than REMAINING_WORK residuals; (b) the
  disclosed `start_fraction.value < end_fraction.value` sibling-comparison
  limitation (MEMORY 2026-05-17) is a bounded, disclosed JSON-Schema-2020-12
  expressiveness limit, noted on REQ-02-01-04, not tracked work. Flagged for
  reviewer confirmation that neither warrants a REMAINING_WORK/UNKNOWN row.

## Evidence-execution log

- **Re-executed (side-effect-free, addendum 9):**
  `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -p no:cacheprovider
  tests/test_model_schema.py tests/test_invented_example_models.py` at the
  frozen tree → **11 passed** (4 in test_model_schema.py, 7 in
  test_invented_example_models.py). `git -C FROZEN status --porcelain` empty
  BEFORE and AFTER. No `CARGO_TARGET_DIR` needed (Python-only); pytest cache
  disabled via `-p no:cacheprovider`. The JSON Schema validation helper tests
  ran (jsonschema present) rather than skipping.
- **Cited as recorded (not re-executed for the sweep row):** DEC-025 five-surface
  sweep `validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json`
  `python_pytest=pass` at commit `e648462f1d05` (ancestor of the frozen SHA),
  carried with the addendum-10 qualifier `content-identical at frozen SHA
  551f84ef6be656f1603ce0acfa5e3935aa9683c7 (diff empty over tests/, schemas/,
  fixtures/)` as recorded in `VERIFICATION_INDEX.csv` rows PY-41 and PY-35.
- **DecisionBasis resolution:** DEC-068 resolves to
  `execution/_Coordination/_DECISIONS/D-36_RULING_2026-07-09.md` and
  `SOFTWARE_DECOMP.md` §12 (present in the frozen tree); DEC-008 / AB-00-0x
  resolve via `SOFTWARE_DECOMP.md` §12 and `_CONTEXT.md` Architecture Basis
  Injection. Requirement rows with no governing decision carry `NONE_FOUND`.

## Convention friction

- **SourceReliability on requirement rows = UNVERIFIED.** The schema/test
  evidence is project-original, agent-generated, agent-audited. A human approval
  exists (2026-06-03, TP-CODE-EVIDENCE-MIGRATION-RECONCILIATION-001) that
  accepted the source/test evidence for a bounded CHECKING transition, but the
  deliverable was reset to IN_PROGRESS (2026-06-17 housekeeping; 2026-07-02
  K-CONFLICT-1 affirmation) "pending further development," and the schema then
  gained the DEC-068 (2026-07-09) load-generation slots — so the *current*
  record post-dates the last human disposition. Per addendum 6 that yields
  UNVERIFIED, not REVIEWED. Note the nuance: DEC-068's D-36 ruling *does*
  human-approve the g_factors/modulus-basis slot shape specifically ("this shape
  is approved"), but not the schema record as a whole; UNVERIFIED at row grain
  is the conservative encoding.
- **STALE_SETUP_SPECIFICATION applied only to DECLARED_STATE rows**, never to
  requirement/exclusion rows (convention 1 as restated in the brief).
- **ACCEPTANCE grain** — the brief's tightening vs the exemplar left the
  acceptance-row count to judgment; recorded 0 with rationale above.
- **SelectableUnderCurrentLoop = NO on every row** — mechanical (convention 6):
  no ungated non-bootstrap residual exists; the bootstrap is excluded
  (addendum 2). Matches `DELIVERABLE_INVENTORY.csv` (DEL-02-01 Selectable = NO).
  The owner suspension is a run-level caveat, not encoded per-row.

## Boundary-compliance statement

All fences held. Discovery was read-only outside the two W1 output files; no
lifecycle transition, DAG mutation, cross-project edit, or edit to any
`_STATUS.md`/register/product file was made. No release-readiness, issuance,
certification, sealing, professional-approval, or code-compliance claim appears
anywhere in these outputs (F-PIP-1..5); the EXC-002 row records that the
deliverable itself makes no such claim. All dispositions are agent judgments,
routed via `AuthorityNeeded` (OWNER on the four stale-prose R5 repair
candidates), never phrased as owner/engineering rulings. No agent-workflow
redesign proposed. The frozen evidence worktree porcelain was empty before and
after every check; writes were confined to
`WAVES/W1/CLAIM_CONCORDANCE_DEL-02-01.csv` and `WAVES/W1/NOTES_DEL-02-01.md`.
