# Worker G1 carry-forward notebook — DEL-08-01, DEL-08-02, DEL-08-03 (W3 PKG-08)

Running record of how recurring situations are judged, so the three ledgers
treat the same situation the same way. Agent judgments only; not rulings.

## Recurring situations and chosen treatment

| Situation | Treatment |
|---|---|
| Section wrappers and `Datasheet/Specification/Procedure/Guidance:` headers (pre-typed) | NON_NORMATIVE · NOT_ASSESSED |
| D-41 R5 T7 PDU-055 "current declaration" blocks (pin rev 0.8 + DAG-007; delegate residuals to Remaining) | CP-03 → CP-02: DECLARED_STATE · STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE · LOCAL_DESIGN · NONE · RECORD · NO. Delegation clause not relied on (A4) |
| SOW names the former product name (prose) | CP-04 default on the SOW SURFACE row; items judged on substance |
| STATUS Remaining names `.opsproj` | CP-04 identity-rename variant on the STATUS SURFACE row (PROJECT_BASELINE · NONE · RECORD · OWNER) |
| `_STATUS.md` Last Updated older than its latest History entry | CP-05 on the STATUS SURFACE row |
| SOW cites `docs/SPEC.md` sections shifted by one at the freeze (reporting now §9, warning classes in §8, rule-pack evaluator §7, V&V §10, acceptance §12) | Common defect: minted `SOW.s01` SUBCLAIM, CP-02 STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE. Requirement items judged on substance; reference tables whose substance is the pointers take CP-02 themselves |
| Four-document residue (Datasheet/Specification/Guidance/Procedure lists, check_four_documents setup verification) | CP-01 · STALE_SETUP_SPECIFICATION (origin 7bee9ae41 by `git log -S`) · REPRESENTATION_MIGRATED |
| "This setup session does not implement…/future implementation" framing now overtaken by landed code | STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE · LOCAL_DESIGN · NONE · RECORD · NO |
| Setup-era TBD/open question since ruled (renderer format DEC-021/DEC-061; container DEC-028/DEC-057; notice wording DEC-081) | STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING · LOCAL_DESIGN · NO |
| Setup-era TBD since settled by implementation with no ruling needed (schema fields, linter interface, manifest names, redaction routing) | STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE |
| Identification-table status fields ("SEMANTIC_READY setup artifacts prepared", "Draft setup artifact; not implementation") | Readiness/review state → STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT (F3 exception) |
| `_CONTEXT.md` Architecture Basis Injection | CS-04 row for the pin; `.s01` for "PKG-00 at SEMANTIC_READY" (STALE_REVIEW_OR_EVIDENCE · SCOPE_REDIRECTED_BY_RULING, per the W2 AGENT_READING majority); `.s02` for the "physical project package/container" Still-TBD only where the deliverable touches the container (08-01, 08-02) |
| `_CONTEXT.md` surface as a whole | CP-02 (stale 0.7 pins common to the surface) |
| OUT-001 purpose statement | Judged on substance: the contract document exists with the listed coverage → ALIGNED |
| OUT-001 matrix row and VER-001 | CP-09: PASS parity records exist, none matches the frozen SOW → STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN, citing the CHANGE-P2 parity record |
| AC-001 (acceptance of the contract content) | Judged on the SOW content → ALIGNED when the contract preserves the listed items |
| MEMORY, STATUS#history, PREPARATION notes, "setup sequence completed" | HISTORY · ALIGNED when accurate for its date |
| Rule-pack references in product reports: `renderableReportInput.ts` always sends `rule_pack_refs: []`; `reportPackageRequest.ts` refuses a package unless rule check is `RULE_INPUTS_INCOMPLETE` (REPORT-PACKAGE-RULE-PACK-BINDING-UNAVAILABLE) | App-behaviour claims (reports include/record rule-pack metadata) → PARTIALLY_IMPLEMENTED · PARTIAL_SLICE (F7), per-DEL FindingGroup. Constraint claims ("report only safe metadata", "do not embed payloads") → ALIGNED on the engine with `PRODUCT_CALLER: NONE` for the rule-pack path |
| Versions: audit manifest carries solver name/version/build only; no application/software version anywhere; rendered HTML/PDF carries neither | PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE · VALIDATION (SPEC §9, OPS-K-REPORT-1 named in Notes) |
| Diagnostic class: product-physics diagnostics carry no class; desktop adapters derive class from severity (SOLVE_BLOCKING / ASSUMPTION_WARNING) and substitute a generic remediation; rendered table shows Code/Severity/Message/Remediation only | DEL-08-01 R-08-01-007: PARTIALLY_IMPLEMENTED · PARTIAL_SLICE. DEL-08-03 rows that forbid reclassification/inference: IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT · PROJECT_BASELINE · BASELINE · OWNER. Rendered-field omission rows: PARTIALLY_IMPLEMENTED · PARTIAL_SLICE |
| Canonicalization: audit-manifest crate hashes "project-local deterministic JSON" (explicitly not JCS) and its enum cannot express JCS; product computes model hash with `canonicalSha256Hex` (RFC 8785) but labels it `project_local_deterministic_json` in the package audit manifest | DEL-08-02 JCS rows: IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT · PROJECT_BASELINE · NONE · BASELINE;RECORD · OWNER |
| Tier for code gaps against SPEC/AB/DEC baselines | PROJECT_BASELINE; INVARIANT only when the gap touches professional boundary, IP/data, security/privacy, protected checks or validation (F8) |
| Layers for code gaps | provenance/reproducibility disclosure → VALIDATION; accepted-baseline structure (AB-00-06 classes, AB-00-04 hash basis) → BASELINE; record-only drift → RECORD |
| Verification-plan blocks whose checks are only partly present | PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · LOCAL_DESIGN · VALIDATION |
| AuthorityNeeded | NO for doc catch-up and ordinary implementation work; OWNER for POSSIBLE_DEFECT code-change candidates and CP-04 |
| Product callers | Renderer, sections, generator validation, report package, audit manifest all have a product caller (Tauri `render_calculation_report`, `save_report_package`). Engine-only paths: audit-manifest asset entries (product sends `assets: []`), rule-pack references (product sends none) → `PRODUCT_CALLER: NONE` on ALIGNED rows that rest on them |

## Evidence anchors

- Gate evidence for all reporting crates and Python contract tests:
  `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` (cargo test for
  audit_manifest, pdf_emitter, protected_content_linter, report_generator,
  report_package, report_renderer, report_sections, result_export listed in the
  sweep log; not rerun).
- F3 origin checks (`git log -S` on the frozen history): setup SOW text first
  appears in `7bee9ae41` (2026-05-18); D-41 blocks in `d2073a0c1`
  (2026-07-12); DEC-081 fence text in `8fac6631a` (2026-07-17).
