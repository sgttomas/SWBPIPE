# Worker carry-forward notes — W3 PKG-15 (DEL-15-01 to DEL-15-04)

Running record of recurring judgments so the same situation gets the same
treatment across the four deliverables. Agent judgments, not owner rulings.

## Product-caller finding (F7)

- `core/handoff/**` Python modules (target mapping, exporter, external prover
  metadata) have no product caller at the freeze: only `tests/**` import them.
- The desktop product exposes handoff and external-prover packets through
  `apps/desktop/src/features/handoff/HandoffPanel.tsx` and
  `apps/desktop/src/features/external-prover/ExternalProverBoundaryPanel.tsx`
  in the Exports section of `App.tsx`. These TypeScript builders emit their own
  technical-preview shapes. The handoff preview does not follow
  `schemas/handoff_package.schema.json`; the external-prover preview adds
  `unit_policy_evidence`, which the strict external-prover schema does not
  allow. No test validates either desktop packet against a schema.
- Treatment: claims about a schema or engine are satisfied by it and carry
  `PRODUCT_CALLER: NONE`. Claims about product behaviour (generate
  schema-compliant packages, BACKEND_FEATURE_SLICE export path) are
  `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE`.

## Stale pointers (CP-02)

- Text naming decomposition revision 0.7/0.8 or DAG-002/DAG-006/DAG-007 as the
  current or approved basis: `STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE ·
  LOCAL_DESIGN · NONE · RECORD · NO`. A dependency row ID such as
  `DAG-002-E0805` used only as an identifier is not a finding.
- D-41 R5 T7 PDU-055 declarations: `CanonicalSituation=CP-03`, fields per
  CP-02; delegation to Remaining not relied on (A4).
- `CONTEXT` SURFACE rows: CP-02 (the file pins revision 0.7 as its basis).

## CS-04 sub-claims

- `.s01`: PKG-00 at SEMANTIC_READY statement; all PKG-00 members are
  IN_PROGRESS at the freeze. `STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT`.
- `.s02`: Still TBD list keeps the physical project package/container
  (DEC-012 portion superseded by DEC-017/SCA-003) and the import/export format
  list (OI-004, SCA-004). `STALE_REVIEW_OR_EVIDENCE ·
  SCOPE_REDIRECTED_BY_RULING`.

## Four-document residue (CP-01)

- Setup-era text naming Datasheet/Specification/Guidance/Procedure as current
  records; first present at `7bee9ae41` by `git log -S` →
  `STALE_SETUP_SPECIFICATION · REPRESENTATION_MIGRATED`.

## Rename residue (CP-04)

- Recorded once on each SOW SURFACE row (default variant). Sources: the
  `openpipestress.org` schema `$id` values (DEC-101 (iv) not effected) and
  "OpenPipeStress" in SOW prose. Items judged on substance.

## Parity (CP-09)

- All four: PASS parity records exist, none matches the frozen SOW (SOWs
  changed 2026-07-16/17 by D-48 Wave 2 after the 2026-07-14 parity).
  Matrix OUT-001 and VER-001 → `STALE_REVIEW_OR_EVIDENCE ·
  EVIDENCE_OVERTAKEN`.

## STATUS

- CP-05 on every STATUS SURFACE (Last Updated 2026-07-12 vs a 2026-07-16
  history entry).
- Remaining "obtain human dispositions" items: accurate, open, no governing
  row carries them → `DOCUMENTED_UNIMPLEMENTED · NOT_STARTED · LOCAL_DESIGN ·
  NONE · RECORD · REVIEW` (F2 second branch).

## SOW property-name drift

- DEL-15-01: slot descriptions name properties the schema never had
  (model_hash provenance; units_manifest entries/diagnostics; entity_ids
  record fields) → `IMPLEMENTED_DIFFERENTLY · OTHER · LOCAL_DESIGN`,
  `FG-DEL-15-01-01`, MEDIUM confidence.

## Gate evidence

- `tests/**` Python tests: the B4-4 sweep ran `pytest -q tests` with 1,138
  passed and no failures; cited as `GATE:…/SUMMARY.json`, not rerun.

## Progress log

- DEL-15-01 sealed (93 rows). DEL-15-02 sealed (91 rows); splits CLM-011
  (.r01-.r12) and CLM-021 (.r01-.r07).
- DEL-15-02 findings: FG-DEL-15-02-01 construction field names (OTHER);
  FG-DEL-15-02-02 builder defaults absent mapping_status to "mapped" and
  value_kind to "metadata" without diagnostic → `IMPLEMENTED_DIFFERENTLY ·
  POSSIBLE_DEFECT · INVARIANT · NONE · BASELINE · NO` (R005, R007, CLM-021.r03,
  AC-001), MEDIUM confidence, code reading only.
- CT-001 (dependency enum conflict, human ruling TBD) overtaken by v3.1
  normalization in the mirror and the DAG-007 type-system approval →
  CP-02 fields with cause SCOPE_REDIRECTED_BY_RULING.
- Generator scripts moved out of the deliverable folder to the session
  scratchpad `_scratch_w3p15/` (deleted at the end).
- DEL-15-03 sealed (97 rows); splits CLM-006, CLM-013, CLM-020.
  FG-DEL-15-03-01 product gap (no product path generates schema-compliant
  packages; exporter consumes rather than generates) → PARTIALLY_IMPLEMENTED ·
  PARTIAL_SLICE · PROJECT_BASELINE · RECORD · NO.
  FG-DEL-15-03-02 PR #307 redaction makes export payload content '[REDACTED]'
  (hash, units, IDs, refs, assumptions, warnings, boundary values) →
  IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT · PROJECT_BASELINE/INVARIANT (units,
  REQ-005) · BASELINE · OWNER, MEDIUM, to R4 with PR #307.
  FG-DEL-15-03-03 T2A "label carried unchanged" text behind redaction →
  DOC_BEHIND_CODE · LOCAL_DESIGN (STALE_REVIEW_OR_EVIDENCE on declarations).
- DEL-15-04 sealed (87 rows); splits CLM-004, CLM-005, CLM-010, CLM-012.
  FG-01 product surface fixed preview packet, not schema-valid
  (unit_policy_evidence) → PARTIALLY_IMPLEMENTED · PARTIAL_SLICE ·
  PROJECT_BASELINE. FG-02 human-acceptance reference not representable (CP-11,
  DEC-081) → DOCUMENTED_UNIMPLEMENTED/PARTIALLY · DEFERRED_BY_RULING ·
  INVARIANT · CLAIMS · OWNER. FG-03 no comparison-report link kind (MEDIUM).
  INIT.md citations (file removed 2026-07-04) → CP-02.
- All four forward ledgers sealed; next: batch check, then routing file.
