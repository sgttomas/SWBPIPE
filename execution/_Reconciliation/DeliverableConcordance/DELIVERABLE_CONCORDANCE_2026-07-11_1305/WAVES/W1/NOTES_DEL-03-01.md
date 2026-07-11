# NOTES — DEL-03-01 Material library schema with provenance (W1)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W1. Frozen source tree
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Ledger:
`CLAIM_CONCORDANCE_DEL-03-01.csv` (16 rows, 20 columns, RFC-4180 clean).

## 1. Histograms (recounted from the CSV)

Disposition histogram:
- ALIGNED: 15
- PARTIALLY_IMPLEMENTED: 1

ClaimType histogram:
- REQUIREMENT: 9
- EXCLUSION: 1
- DECLARED_STATE: 6

(Supporting, not required) ClaimClass: SCHEMA 7, DOCUMENTATION 6, GOVERNANCE 3.
SourceReliability: UNVERIFIED 9, NOT_APPLICABLE 6, REVIEWED 1.

## 2. Row census rationale

- **9 REQUIREMENT rows** — one per current requirement ID REQ-03-01-001..009
  (self-identifying scheme; matches `DELIVERABLE_INVENTORY.csv`). Substance
  dispositions only (convention 1): 8 ALIGNED, 1 PARTIALLY_IMPLEMENTED
  (REQ-007). No requirement row takes `STALE_SETUP_SPECIFICATION`.
- **0 ACCEPTANCE rows** — the Specification's "Verification" section is a
  prose bullet list of verification approaches and a "Verification approach"
  table column that merely restate the requirements; there are no distinct
  `VER-03-01-*` acceptance IDs. Per addendum 12 / brief row-census rule,
  verification tables that only restate requirements do NOT get mirrored
  ACCEPTANCE rows. (Contrast the DEL-07-05 exemplar, which had explicit
  `VER-07-05-*` IDs.)
- **1 EXCLUSION row** — the durable product-boundary sentence in
  Specification.md Scope paragraph 2 (no engineering values, no standards
  reproduction, no code-compliance/certification, no review-disposition or
  dependency-satisfaction closure), captured as one bundled exclusion at the
  grain the surface states it.
- **6 DECLARED_STATE rows** (addendum 1 census) — Specification, Datasheet,
  Guidance, Procedure, `_STATUS.md`, `MEMORY.md`. No deliverable-owned in-tree
  README exists. `_REVIEW.md` and `Review_Findings.csv` are review/evidence
  artifacts, not declared-state prose surfaces, so they get no DECL row (they
  are cited as evidence on the REQ-009 and EXC rows instead).
- **0 REMAINING_WORK rows** — the only `_STATUS.md` `## Remaining` entry is the
  seeded `(gated: D-41)` bootstrap item, recorded verbatim ONLY in the
  `_STATUS.md` surface row's `RecordedRemaining` (DECL-005) and excluded from
  all residual/gate/selectability analysis (addendum 2). There are no
  non-bootstrap recorded residuals (`DELIVERABLE_INVENTORY.csv`:
  NonBootstrapItems=NONE).
- **0 IMPLEMENTED_UNMAPPED rows** — the three material surfaces in this
  deliverable's orbit are all mapped to DEL-03-01: `schemas/material.schema.yaml`
  (SURF-189, SPEC-named), `fixtures/material/invented_material_library_valid.json`
  (SURF-154), and `tests/test_material_schema.py` (PY-38). The shared app
  surfaces that co-attribute DEL-03-01 (SURF-005 src-tauri, SURF-026 library
  feature, SURF-030 model-tree, SURF-099 operation_applier) are material app
  surfaces owned primarily by other deliverables (DEL-07-03 etc.) and are not
  unmapped, so none is ledgered here at DEL-03-01 grain.

## 3. Notable finding — kit is reconciled, NOT setup-era (contrast siblings)

All four four-document-kit DECL rows (Specification, Datasheet, Guidance,
Procedure) are **ALIGNED**, not `STALE_SETUP_SPECIFICATION`. This is a
deliberate contrast with setup-era siblings such as DEL-02-01 (whose kit still
declares the schema as a "future/not-yet-implemented" artifact and therefore
takes STALE dispositions). DEL-03-01's kit was reconciled to present-tense by
`TP-DEL-03-01-EVIDENCE-RECONCILIATION` (MEMORY.md 2026-06-05, run record
`_run_records/WORKING_ITEMS_RUN_2026-06-05_...`), which "updated deliverable-local
evidence language only," and Procedure step 6 is itself a standing stale-language
check. Each kit surface describes the frozen implemented slice (schema + fixture
+ test present and passing). `MEMORY.md` and `_STATUS.md` are likewise ALIGNED
(dated historical entries; per addendum 1, drift inside dated log entries is a
note, never a staleness disposition).

## 4. Self-flagged rows

- **DEL-03-01-REQ-007** — disposition `PARTIALLY_IMPLEMENTED`. The requirement
  ("preserves versioned, provenance-preserving, schema-governed serialization
  behavior where material data is serialized") is satisfied at the schema level
  (schema_version semver, provenance required throughout, no defaults, 2020-12
  dialect) but the actual serialization/round-trip persistence is a documented
  downstream deferral to DEL-02-05 (Specification's own REQ-03-01-007
  verification says "round-trip persistence integration remains downstream
  TBD"). Judgment call: a reviewer could read the conditional "where material
  data is serialized" as vacuously ALIGNED for a schema-only deliverable; I read
  the bounded-portion-exists case as PARTIALLY_IMPLEMENTED per §7. No named
  human ruling permits the deferral, so `ACCEPTED_DIVERGENCE` was not used
  (addendum 11).
- **DEL-03-01-REQ-009** — SourceReliability upgraded to `REVIEWED` (all other
  REQ rows are UNVERIFIED). Basis: Human Gate A disposition 2026-06-05 accepted
  `Review_Findings.csv` PKG03-DEL-03-01-PKG02-001 (the dimension-vocabulary
  finding) as ACCEPT_AS_IS/RESOLVED — a recorded human disposition covering
  exactly this claim's cited record (addendum 6). Also flagged: the test
  hard-codes `CANONICAL_DIMENSIONS` rather than importing
  `schemas/units.schema.yaml`; I independently confirmed the material 9-value
  enum is a subset of the live `units.schema.yaml` DimensionId at the frozen
  SHA, so the disposition is robust, but the test's private copy is a
  verification-robustness caveat (it omits `force_per_length`, which only makes
  the subset check stricter, not looser).
- **DEL-03-01-DECL-004** (Procedure) — kept ALIGNED with a metadata-drift note:
  Procedure Prerequisites still names "approved DAG-006 context" while the
  current canonical graph authority is DAG-007 (`Dependencies.csv` /
  `_DEPENDENCIES.md`). Judgment call: this is a stale prerequisite/context
  pointer, not a declaration about the implemented slice, so the widened
  `STALE_SETUP_SPECIFICATION` (addendum 4) does not attach; a reviewer may
  prefer to treat it as a light R5 documentation-refresh candidate.

## 5. Fence-adjacency note (per dispatch: provenance vs F-PIP-1)

The dispatch flagged that material-library provenance claims sit adjacent to
the F-PIP-1 protected-content fence. Provenance/source-reliability disclosures
were verified structurally without any compliance determination:
- REQ-003: the `Provenance` `$def` requires source_name/source_location/
  source_license/contributor/contributor_certification/redistribution_status/
  review_status and is required at every material level; the fixture populates
  it with `source_license=TBD`, `redistribution_status=TBD`,
  `review_status=pending` (i.e. an explicitly UNACCEPTED disclosure, not a
  cleared one).
- REQ-002 / REQ-006 / REQ-008 / EXC-001 rest partly on the test's
  `FORBIDDEN_PUBLIC_DATA_TEXT` scan (ASME, B31, ASTM, CODE_COMPLIANT, certified
  material, sealed, automatic compliance, professional approval by the
  software), re-executed PASS at the frozen SHA. No row asserts release
  readiness, issuance, certification, sealing, professional approval, or code
  compliance (F-PIP-1..5 held). Human protected-content/redistribution review
  remains TBD (Procedure Records) — hence SourceReliability=UNVERIFIED on those
  rows; the concordance disposition (structural support present) is agent
  judgment, not a compliance ruling.

## 6. Evidence-execution log

- **Re-executed side-effect-free at the frozen SHA** (addendum 9):
  `tests/test_material_schema.py`, run two ways from
  `FROZEN/projects/chirality-piping`:
  1. `PYTHONDONTWRITEBYTECODE=1 python3 -B tests/test_material_schema.py` — exit 0.
  2. `PYTHONDONTWRITEBYTECODE=1 python3 -B -m pytest -q -p no:cacheprovider
     tests/test_material_schema.py` — `2 passed`.
  No build step; pure stdlib schema/fixture reads. `git -C FROZEN status
  --porcelain` was empty BEFORE and AFTER both runs; no `__pycache__` or
  `.pytest_cache` was created anywhere in the frozen tree; HEAD unchanged at
  `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
- **Independent cross-check** (side-effect-free read): material
  `MaterialPropertyDimension` enum confirmed a subset of the live
  `schemas/units.schema.yaml` `DimensionId` enum at the frozen SHA (supports
  REQ-009).
- **Cited as recorded** (addendum 10): the DEC-025 sweep row PY-38
  `SWEEP_20260711T040758Z_e648462f1d05.json` surface `python_pytest=pass` at
  commit `e648462f1` (ancestor of the frozen SHA), carried on each row with the
  qualifier `content-identical at frozen SHA
  551f84ef6be656f1603ce0acfa5e3935aa9683c7 (diff empty over tests/, schemas/,
  fixtures/)` — the diff-scope taken from `VERIFICATION_INDEX.csv` PY-38, which
  I did not personally re-run but whose product-code paths I did re-execute
  directly, so the marker rests on my own frozen-SHA pass plus the recorded
  sweep.
- **Direct inspection** (side-effect-free listing/reads): four-document kit,
  `_STATUS.md`, `MEMORY.md`, `_REVIEW.md`, `Review_Findings.csv`,
  `Dependencies.csv`/`_DEPENDENCIES.md`, `_CONTEXT.md`, `_REFERENCES.md`,
  `_run_records/**`, schema, fixture; lifecycle-correction Decision_Log records
  and the D-41 register row confirmed present in tree.

## 7. Convention-friction notes

- **ClaimClass for schema-vs-governance requirements.** REQ-002 (no bundling of
  protected allowable tables) and REQ-008 (agent-output boundary) are encoded
  GOVERNANCE because the governing concern is an IP/agent prohibition rather
  than a structural capability; REQ-006 (quarantine/escalation status) is
  encoded SCHEMA because it is delivered as schema enums/vocabulary even though
  its purpose is IP handling. The binding set does not adjudicate this
  schema-vs-governance line for mixed rows; the split above is a judgment call
  and is called out for the reviewer.
- **REMAINING_WORK vs downstream deferrals.** The deliverable carries several
  genuine forward items that are NOT in `_STATUS.md ## Remaining`: public
  material source catalog, public fixture value policy, temperature
  interpolation policy (drafted D-38 per DEC-068 item 1 / schema OpenDecision),
  allowable value storage policy, protected-content/redistribution human
  review, dependency satisfaction, and round-trip persistence. These are homed
  as schema `OpenDecision` records and/or downstream deliverables (DEL-02-05
  persistence, DEL-03-07 import provenance checker), captured on the relevant
  REQ rows' `RemainingWork`/disposition rather than promoted to
  `REMAINING_STATE_MISMATCH` — the `_STATUS.md` surface does not falsely claim
  "no other work," so residual-homing (addendum 3) resolves them elsewhere, not
  to a mismatch finding.
- **`AuthorityNeeded` all NO.** No row's concordance disposition requires an
  authority to resolve (dispositions are settled agent judgments). The
  deliverable's owner/human-gated open decisions above are its real forward
  work but do not change any disposition, so none is routed here; they are
  recorded in this note instead. Nothing rose to `DEFERRED_AGENT_WORKFLOW`,
  `AUTHORITY_CONFLICT`, or `ENGINEERING_AUTHORITY_REQUIRED`.
- **No addendum-13 cap triggered.** No row rests on validation evidence in
  `TECHNICALLY_ADDRESSED_PENDING_HUMAN` state — the two PKG-02 findings that
  once held that state were subsequently RESOLVED (ACCEPT_AS_IS) by Human Gate A
  2026-06-05, so no Confidence is capped at MEDIUM on that ground.

## 8. Boundary-compliance statement

All fences held. Discovery was read-only outside the two output files
(`WAVES/W1/CLAIM_CONCORDANCE_DEL-03-01.csv`, `WAVES/W1/NOTES_DEL-03-01.md`); no
`_STATUS.md`, register, product, DAG, or cross-project file was modified; no
lifecycle transition was applied (`LIFECYCLE_REASSESSMENT_REQUIRED` was not
needed and none was recorded). No release-readiness, issuance, certification,
sealing, professional-approval, or code-compliance claim appears anywhere in the
outputs (F-PIP-1..5). All dispositions are agent judgments, not owner or
engineering rulings. The frozen evidence worktree stayed clean: `git status
--porcelain` empty before and after every re-execution, no artifacts written
under the frozen tree, HEAD pinned at
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`.

## STOP-worthy contradiction

NONE.
