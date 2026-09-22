# Notes — DEL-14-02 Analysis run records (W3, PKG-14)

## Path aliases

- `D` = `projects/chirality-piping/execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-02_Analysis run records/` (the path has spaces; tokens resolve at the freeze).
- Project-root tokens (`schemas/`, `core/`, `apps/`, `tests/`, `fixtures/`) resolve under `projects/chirality-piping/`.
- Parity records are cited at the repository-root path `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/...`, where they exist at the freeze.
- `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` is the suite-level gate record. No per-test status is asserted beyond it ("not rerun").

## Judgment calls

- **Contract version advanced (FG-DEL-14-02-01).** Under D-67, which adopts
  the result-compatibility plan by reference and states that the "new 0.2
  hash contract [is] explicitly adopted", the product builds strict 0.2
  records with checked profile `openpipestress_jcs_ijson_v1` through the
  RFC 8785 Rust canonicalizer. The SOW, whose hash text was written by the
  D-41 T2A repair on 2026-07-12, still describes only the Python 0.1
  `SORTED_COMPACT_JSON` record and "no JCS claim". Those rows are
  `STALE_REVIEW_OR_EVIDENCE · CONTRACT_VERSION_ADVANCED · LOCAL_DESIGN`, with
  `AdoptedByReference=YES`.
- **Rule-pack and library references never populated (FG-DEL-14-02-02).**
  Both builders hard-code `rule_pack_refs: []` and `library_refs: []`
  (`records.py` L132-133; `analysisRunCompatibility.ts` L67), including when a
  rule-check status is recorded. SOW-072 names these as binding categories, so
  R003, R009 (CP-11), CLM-004, CLM-005, CLM-013, CLM-020, CLM-025 and OUT-001
  are `PARTIALLY_IMPLEMENTED · PROJECT_BASELINE · BASELINE`.
- **PDU-033 (FG-DEL-14-02-04)** and **producer diagnostic breadth
  (FG-DEL-14-02-05)** stay open per the 2026-09-15 acceptance. R005, CLM-019
  and R006 are `PARTIALLY_IMPLEMENTED`. Remaining R01 and R02 are `ALIGNED`
  with `OPEN_ACTION` to R006 and R005.
- **R002.** `ALIGNED` (MEDIUM). `model_state_ref` is required, and the exact
  input-manifest binding (with a model-mismatch block) is the "equivalent
  validated binding" the row allows. The synthesised state reference is
  recorded against DEL-14-01.
- **STATUS surface (CP-04, frozen-contract variant).** Remaining item 2 names
  the active identifier `openpipestress_jcs_ijson_v1`. This is a code-change
  candidate for R4, not a deliverable edit. The MEMORY mention is history and
  is not treated as residue.
- **Blocks.** Only CLM-011 is split. Other tables are assessed directly, and
  Notes name the secondary gaps.

## Canonical departures

None. CS-04 has `.s01` and `.s02` only. The Resolved Baseline (Rust
canonicalizer, JCS profile) holds for the 0.2 contract, so no `.s03` was
needed.

## Convention friction

- A record-contract ruling adopted by reference (D-67) overtakes
  post-migration SOW text. `CONTRACT_VERSION_ADVANCED` was chosen over
  `SCOPE_REDIRECTED_BY_RULING` because the change is a schema and hash-contract
  version step.

## Smallest checks for UNKNOWN rows

- CLM-014, CLM-021: locate or produce protected-content/private-data review
  notes for `fixtures/analysis_runs/invented/` and other public examples.

## Reverse pass

- Claimed: RC-14-0059, RC-14-0127, RC-14-0203, RC-14-0256 and RC-14-0241.
- UNKEYED: RC-14-0146, legacy checksum verification. No key covers
  historical 0.1 preimage verification; the nearest key is CLM-011.r07.
- COVERS: RC-14-0077 (shared `core/product_preview`, DEC-074 O3) and
  RC-14-0022 (run audit panel).
- The reverse pass did not change the sealed view. RC-14-0127 mentions "any
  recognised rule-check aggregate". That aggregate feeds status and
  reproducibility, not `rule_pack_refs`, which remains an empty literal, so
  FG-DEL-14-02-02 stands.

## Batch consistency

`validate_ledger_v2.py --batch` over the three PKG-14 worker ledgers
(DEL-14-01, DEL-14-02, DEL-14-03): **PASS, 0 consistency findings**. No
flagged pair needed a justification.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since
2026-09-19 Piping selects work through owner-steered work graphs, not
`## Remaining`.

## Claim fence

These notes and ledgers make no release, approval, compliance or
certification claim. Dispositions are agent judgments, not owner rulings. No
protected standards text, vendor or private data is quoted, and no external
equation artifact is used as evidence (DEC-043).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
