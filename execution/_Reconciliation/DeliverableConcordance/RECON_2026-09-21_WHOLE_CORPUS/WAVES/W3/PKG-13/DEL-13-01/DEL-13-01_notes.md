# DEL-13-01 notes — W3 PKG-13, worker G1

Forward: 104 rows (66 required keys, 5 blocks split into .rNN: CLM-004, 005, 011, 018, 019; two .sNN on Architecture Basis Injection). Reverse: 323 capabilities. Shared judgments: `../_WORKER_DEL-13-01_NOTES.md`.

## Path aliases
- "repo root" in the SOW means the project root `projects/chirality-piping/`. The schema is at `schemas/design_knowledge.schema.json` there.
- Parity records are at the repository root: `execution/_Coordination/AgentRuns/SOW-STAGE1-20260712/.../RECON-FANIN/evidence/DEL-13-01/` and `SOW-STAGE2-EXEC-20260712-01/.../TASK-PIP-13-01/stage1_evidence/`.

## Judgment calls
- **CP-09 corrects EVIDENCE_MAP.** EVIDENCE_MAP lists NONE_FOUND. The A3 discovery step found two PASS parity records, both for SOW sha256 6c76b2c7…, not the frozen 62701b31… (D-48 Wave 2 edit). So OUT-001 (matrix), AC-001 and VER-001 are STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN, not UNKNOWN.
- **REQ-13-01-011 is IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT · INVARIANT · VALIDATION (MEDIUM).** The live desktop Knowledge panel reads `fixtures/product_preview/invented_design_knowledge.json` through Tauri `load_design_knowledge`. That file is a preview shape: its provenance is the bare string `invented_example`, and it is never validated against the DEL-13-01 schema. The data is invented, so the IP boundary holds in substance.
- **REQ-13-01-006 is PARTIALLY_IMPLEMENTED · POSSIBLE_DEFECT.** `Parameter.value` is not tied to `value_kind`, so a "quantity" parameter can be a bare string with no unit. Tier PROJECT_BASELINE with layer BASELINE was chosen because units are not among C3's named INVARIANT subjects; the verifier may prefer INVARIANT.
- **CLM-005.r07 is IMPLEMENTED_UNDOCUMENTED · OWNERSHIP_ELSEWHERE.** MEMORY attributes a Knowledge-panel computed-unit row to DEL-13-01, and the SOW omits it.
- **SOW SURFACE is CP-04.** The `$id` host is `openpipestress.org` and the schema title uses the former name.
- **F7:** `PRODUCT_CALLER: NONE` is recorded on the schema rows. Only the schema's test loads it.

## Canonical departures
None. The CS rows are inherited unchanged. `.s02` (Still-TBD container item) follows the W2 verifier reading for DEC-017.

## Convention friction
- A dead or stale source citation inside an otherwise-met requirement is kept ALIGNED, with the drift noted (C6(b)). Pointer-only blocks are disposed CP-02 directly.
- The pin-origin exception (F3) makes every DAG/rev pin STALE_REVIEW_OR_EVIDENCE, even where the text dates from 7bee9ae41.

## UNKNOWN rows
None.

## Reverse pass and view of sealed rows
The reverse pass did not change my view. A finding made after sealing (while auditing DEL-13-03) does. It is an error I would correct:
- `schemas/units.schema.yaml` DimensionId (the PKG-02 vocabulary) includes `force_per_length`, and has since the initial migration.
- `design_knowledge.schema.json` Quantity.dimension lacks `force_per_length`. The test compares the schema against a local copy of the set that also lacks it.
- So CLM-005.r05 ("dimension enum matches the accepted PKG-02 vocabulary") is not ALIGNED. It should be STALE_REVIEW_OR_EVIDENCE or IMPLEMENTED_DIFFERENTLY (cause POSSIBLE_DEFECT; the schema is behind PKG-02).
- CLM-003 (unit posture) and CLM-011.r04 are affected the same way.
- For the verifier and a fresh worker.

## Batch consistency
`--batch` over the four G1 ledgers: PASS, 0 findings.

## Selectability
`SelectableUnderCurrentLoop` is NOT_APPLICABLE on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
