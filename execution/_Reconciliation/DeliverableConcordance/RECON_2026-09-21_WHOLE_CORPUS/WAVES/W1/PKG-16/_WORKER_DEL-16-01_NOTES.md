# Worker G1 carry-forward notebook — PKG-16 (DEL-16-01..04)

Running record of recurring-situation judgments so the same situation gets the
same treatment across the four PKG-16 ledgers. Agent judgments, not rulings.

## Recurring rules applied

- **W-1 Enumeration growth.** A declared list (change kinds, statuses) that the
  frozen code has since extended: `STALE_REVIEW_OR_EVIDENCE · DOC_BEHIND_CODE ·
  LOCAL_DESIGN · NONE · RECORD · NO`.
- **W-2 Declared-open TBD overtaken by a ruling** (DEC-094 operation taxonomy,
  SCA-003 storage container, DEC-020 single engine, DEC-037 rule grammar):
  `STALE_REVIEW_OR_EVIDENCE · SCOPE_REDIRECTED_BY_RULING · LOCAL_DESIGN · NONE ·
  RECORD · NO` for later declarations; `STALE_SETUP_SPECIFICATION` for
  setup-era `_CONTEXT` text.
- **W-3 Declared-absent/TBD behaviour that frozen code implements within the
  package without a new ruling needed** (e.g. application in the Rust
  operation_applier, desktop operation ledger): `STALE_REVIEW_OR_EVIDENCE ·
  DOC_BEHIND_CODE · LOCAL_DESIGN · NONE · RECORD · NO`.
- **W-4 Contract substitution.** A claim that edits pass through artifact A
  when the runtime uses artifact B: `IMPLEMENTED_DIFFERENTLY` with the cause
  that fits (OWNERSHIP_ELSEWHERE when B is another deliverable's contract,
  DOC_BEHIND_CODE when B landed in this deliverable's own area), `OWNER` when a
  canonical-contract choice is needed.
- **W-5 Accurate-but-incomplete "current implementation is X" statements**:
  `ALIGNED` with a note naming what is omitted; the omission is carried by the
  row that asserts absence or exclusivity (W-3/W-4).
- **W-6 Remaining items.** Accurate governance open items (human dispositions,
  review entries, rulings) → `ALIGNED`, note "open governance action". Accurate
  product-scope gaps → `PARTIALLY_IMPLEMENTED` / `DOCUMENTED_UNIMPLEMENTED` with
  the gap cause (C6(a)), tier PROJECT_BASELINE where the gap is against an
  accepted scope item, layer BASELINE. Text that disagrees with code/evidence →
  `REMAINING_STATE_MISMATCH` (CP-07/CP-08).
- **W-7 CP-04 rename residue.** One row per surface, on the SOW SURFACE row,
  when a named active artifact carries OpenPipeStress identifiers (schema
  `$id`/title, engine provenance `source_name`/`contributor`, crate names,
  `document_kind` strings). Default CP-04 fields.
- **W-8 SOW frontmatter pin** `decomposition_basis ...@e8f59a63` (revision
  0.8): `.s01` sub-claim of the SOW SURFACE key, CP-02.
- **W-9 D-41 PDU-055 current declarations**: CanonicalSituation `CP-03`,
  CP-02 fields (pins revision 0.8 / DAG-007), delegation clause not relied on.
- **W-10 Architecture Basis Injection** (CS-04): two `.sNN` sub-claims each —
  `.s01` Still-TBD list (container ruled by SCA-003, formats SCA-004, grammar
  DEC-037) and `.s02` PKG-00 SEMANTIC_READY framing (D-43/SCA-006); both
  `SETUPRULED`.
- **W-11 Parity rows** (OUT-001 matrix and VER-001): CP-09 stale (no PASS
  record matches the frozen SOW for any PKG-16 deliverable), citing the
  CHANGE-P4 `parity.md` and `claim-map.csv`.
- **W-12 Gate evidence.** Suite-level only:
  `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` (python_pytest over
  `tests/`, cargo test of operation_applier incl. contract corpus). No per-test
  pass asserted beyond what the sweep log shows.
- **W-13 Deliverable-folder paths contain spaces**, so they cannot be evidence
  tokens; they go in `ContextRefs` or Notes aliases (RF-16xx = that
  deliverable's `Review_Findings.csv`).

## Cross-deliverable findings seen so far

- FG-DEL-16-01-01 (W-4): the runtime edit route (EditorOperationIntent via
  operation_applier; taxonomy in DEL-07-01 `viewport_editor.schema.yaml`) does
  not consume `schemas/model_operation.schema.json`; DEC-094 binds vocabulary
  to the applier taxonomy.
- FG-DEL-16-01-02 (W-2): operation-granularity and container TBDs overtaken by
  DEC-094 / SCA-003.
- W-14 "approved DAG-006 mirror" labels are noted only. A broken path
  (DEL-16-02 CLM-004, citing DAG-006/DAG-002_EdgeDispositionReview.md) takes
  CP-02.
- FG-DEL-16-02-01 / FG-DEL-16-03-01 (W-3): the Rust operation_applier
  (DEL-16-02/03 seam; DEC-020) is omitted, and application is declared
  outside the slice or TBD.
- FG-DEL-16-02-02 (W-4): the runtime `schema_validation` field is not a JSON
  Schema check against the DEL-16-01 schema.
- FG-DEL-16-02-03 / DEL-16-03 REQ-002: the runtime route has no constraint
  validation (`constraint_validation: not_run`).
- FG-DEL-16-03-02 / FG-DEL-16-04-02: SOW-070 durable history is incomplete;
  both deliverables judge it the same way (PARTIALLY_IMPLEMENTED).
- FG-DEL-16-04-01 (W-2): the DEC-081 claims registry and lint are
  undocumented in the SOW.
- Batch check over all four sealed ledgers: PASS, 0 findings.
