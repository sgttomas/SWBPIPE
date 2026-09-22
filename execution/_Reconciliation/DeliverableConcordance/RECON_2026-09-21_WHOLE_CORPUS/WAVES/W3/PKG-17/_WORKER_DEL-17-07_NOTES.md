# Worker carry-forward notebook — W3 PKG-17 G3 (DEL-17-07, DEL-17-08, DEL-17-09)

Recurring situations and the treatment chosen, so the three ledgers agree.

- **D-41 R5 T7 PDU-055 current declaration blocks** (three per SOW): CP-03 block,
  treatment CP-02 (pins SOFTWARE_DECOMP rev 0.8 and DAG-007; frozen basis is
  rev 0.12 / DAG-010). STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE ·
  LOCAL_DESIGN · NONE · RECORD · NO; CanonicalSituation CP-02. Delegation to
  `## Remaining` recorded as not relied on (A4).
- **Rename residue** in SOW prose ("OpenPipeStress"): CP-04 default variant on the
  SOW SURFACE row only; items judged on substance. Code strings with the former
  name that the SOW does not name (schema `$id`, provenance source names, download
  filenames, `document_kind`) are recorded in notes only.
- **Setup-era Phase A wording** (text present at `7bee9ae41`, e.g. "shall not
  implement code in this Phase A pass", "four-document kit exists"):
  STALE_SETUP_SPECIFICATION (F3). Four-document references (`Specification.md`,
  `Procedure.md`, Guidance) use CP-01 · REPRESENTATION_MIGRATED.
- **Deleted plan pointer** (`plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md`, deleted
  by `349a2ab33` on 2026-06-03): origin text → STALE_SETUP_SPECIFICATION ·
  BASIS_POINTER_STALE · LOCAL_DESIGN · NONE · RECORD · NO, recorded on the block
  that cites the path (References block, conflict row). No CS code (F3 over CP-02).
- **Architecture Basis Injection** (shared body): "Rust core/application services
  where implementation-facing" vs Python core builders plus TypeScript desktop
  packet builders, no ruling found: IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR ·
  PROJECT_BASELINE · NONE · BASELINE;RECORD · OWNER, MEDIUM.
- **OUT-001 matrix rows**: CP-09, PASS parity records exist, none match frozen SOW →
  STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN.
- **Output OUT-001 (purpose), AC-001, VER-001**: judged on substance against the
  landed slice.
- **Python builder with no product caller**: the desktop panels reimplement the
  packet in TypeScript and do not call the Python module. ALIGNED rows citing
  only the Python builder carry `PRODUCT_CALLER: NONE`.
- **Layer for non-protected partial implementation**: RECORD.
- **Remaining items with open human-review actions** and no governing claim row:
  F2 second branch, DOCUMENTED_UNIMPLEMENTED · NOT_STARTED · LOCAL_DESIGN · NONE ·
  RECORD · REVIEW.
- **_STATUS Last Updated older than history**: CP-05 on STATUS SURFACE.
- **Fixture provenance records** (the SOW template) absent: PARTIALLY_IMPLEMENTED ·
  PARTIAL_SLICE · INVARIANT · IP_DATA;RECORD · REVIEW.
