# Worker G1 carry-forward notebook — W3 PKG-09 (DEL-09-01, DEL-09-02, DEL-09-03)

Running record of recurring situations and the one treatment applied to each,
so the three ledgers judge the same situation the same way. Agent judgments,
not owner rulings.

## Recurring situations

| Situation | Treatment |
|---|---|
| Setup-era future/setup-only framing (origin `7bee9ae41`) now overtaken by the implemented suite crate | STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE · LOCAL_DESIGN · NONE · RECORD · NO |
| Setup-era TBD answered by a later ruling (DEC-026, DEC-027, DEC-046/C-B, DEC-065) | STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING (post-migration text: STALE_REVIEW_OR_EVIDENCE) |
| Four-document kit named as current (Datasheet/Specification/Guidance/Procedure) | CP-01 · STALE_SETUP_SPECIFICATION · REPRESENTATION_MIGRATED |
| Revision 0.7/0.8 pins; SOW front-matter `decomposition_basis@eaad463` (rev 0.8); removed `INIT.md` pointer | CP-02 · STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE |
| D-41 R5 T7 PDU-055 current declarations (rev 0.8, DAG-007) | CP-03 → CP-02 fields; the Remaining delegation clause is not relied on (A4) |
| SOW text names the former product name or an active identifier carrying it | CP-04 on the SOW SURFACE row only (DEL-09-02, DEL-09-03); DEL-09-01 SOW text carries none, so no CP-04 row there (crate identifiers noted for R3) |
| Open question / open-enrichment row that is still accurately open | ALIGNED (declared state accurate); the unmet work sits on the governing requirement/guidance row |
| Declared-open "approved schema/format" hold settled in code without a ruling | CP-10 |
| Output-matrix OUT-001 and VER-001 parity rows | CP-09: PASS records exist, none matches the frozen SOW (D-48 Wave 2 edit 8fac6631a) → STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN, citing the CHANGE-P2 parity record |
| "Canonical unit catalog / conversion constants remain unresolved/TBD" | Drift against DEC-018 (accepted 2026-06-10) and `core/units`; declarations → STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT (MEDIUM); Remaining → CP-07 with RULED_CRITERION. Upstream B2/B3 wiring and alias/witness holds (DEL-02-02) are real and noted |
| Benchmark "unit-aware and dimensionally checked" requirements | fixture-local unit ids + dimension-label checks only; not bound to the DEC-018 project basis → PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE · NONE · BASELINE (a "checkable" wording is ALIGNED) |
| Pending human disposition of a Review_Findings row, recorded only in Remaining | Remaining row carries the gap: DOCUMENTED_UNIMPLEMENTED · NOT_STARTED · LOCAL_DESIGN · NONE · RECORD · REVIEW |
| Protected-content review "before acceptance" of each fixture | 2026-06-06 SELF_CHECK covers fixtures then present; later fixtures show no located review → UNKNOWN · EVIDENCE_NOT_LOCATED · INVARIANT · IP_DATA;RECORD · REVIEW (W1 DEL-07-02 CLM-026 precedent). Exclusion requirements themselves are judged on the fixture code (invented values) |
| Product caller | `openpipestress-runner` (`core/runner/headless`, DEC-065) depends on all three suite crates, so suite-level claims have a product caller; no PRODUCT_CALLER: NONE marker needed for suite rows |
| Conflict tables "None" | ALIGNED (no governing-source conflict for human ruling found; the DEC-018 unit drift is record drift, not a source conflict) |
