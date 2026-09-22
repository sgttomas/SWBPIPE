# CFB-13 — AB-00-06 diagnostic fields on producer diagnostics

**Candidate brief (H2). Not executed.** Area: Engine diagnostics (DEL-04-01, DEL-05-02, DEL-05-03, DEL-06-02, DEL-06-03, DEL-07-03, DEL-12-03). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Carry class, severity, source, remediation and provenance (the AB-00-06 field set) on algebra, stress, evaluator, completeness-checker, editor and telemetry diagnostics, either on the producer type or on a documented product mapping; carry them through the desktop preview diagnostic payload or record where they are reduced.

## Affected claims

11 claim rows on 7 deliverable(s): DEL-04-01, DEL-05-02, DEL-05-03, DEL-06-02, DEL-06-03, DEL-07-03, DEL-12-03.

Classes (portion in this brief / class total): T6-C01 2/180 (Authority NONE); T6-C02 9/116 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-13"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-04-01:SOW#CLM-010/DEL-04-01-REQ-011` | T6-C02 | NONE | — | — | Carry class, remediation and provenance through the desktop preview diagnostic payload, or record the boundary where they are dropped. |
| `DEL-04-01:SOW#CLM-012.r03` | T6-C02 | NONE | — | — | Add a check that the desktop preview diagnostic carries the AB-00-06 field set, or record where it is reduced. |
| `DEL-05-02:SOW#CLM-004` | T6-C02 | NONE | — | — | Carry class, severity, source, remediation and provenance on algebra diagnostics, or map them where the product publishes them. |
| `DEL-05-02:SOW#CLM-010/REQ-05-02-008` | T6-C02 | NONE | — | — | Add the AB-00-06 envelope fields to algebra diagnostics or to their product mapping. |
| `DEL-05-02:SOW#CLM-012/REQ-05-02-008` | T6-C02 | NONE | — | — | Add verification of the diagnostic envelope fields REQ-05-02-008 names. |
| `DEL-05-03:SOW#CLM-011/DEL-05-03-RQ-005` | T6-C02 | NONE | — | — | Carry source and provenance on stress diagnostics (StressFinding) or on their product mapping. |
| `DEL-06-02:SOW#CLM-006.r04` | T6-C02 | NONE | — | — | Bind evaluator findings into a governed result envelope with the AB-00-06 fields (Remaining R02) |
| `DEL-06-02:SOW#CLM-016/REQ-06-02-009` | T6-C02 | NONE | — | — | Bind evaluator findings into a governed result envelope carrying provenance and warnings, and inspect it (Remaining R02) |
| `DEL-06-03:SOW#CLM-013.r03` | T6-C02 | NONE | — | — | Carry remediation text, class and provenance on completeness findings through a governed diagnostics envelope |
| `DEL-07-03:SOW#CLM-005.r08` | T6-C01 | NONE | — | — | Map editor findings onto the six SPEC section 8 classes or record that editor findings use their own blocking/advisory vocabulary. |
| `DEL-12-03:SOW#CLM-011/TEL-REQ-008` | T6-C01 | NONE | — | — | Align TelemetryDiagnostic with the project Diagnostic definition, or rule that the envelope is not yet 'available' for telemetry. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-04-01: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-04/DEL-04-01/DEL-04-01_forward.csv`; ImplementationEvidence cited: `core/solver/diagnostics/src/lib.rs`, `core/runner/headless/src/result_envelope_binding.rs`, `core/product_physics/src/lib.rs`, `apps/desktop/src-tauri/src/lib.rs`.
- DEL-05-02: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-05/DEL-05-02/DEL-05-02_forward.csv`; ImplementationEvidence cited: `core/loads/load_case_algebra/src/lib.rs#L171`, `core/product_physics/src/lib.rs#L710`, `core/loads/load_case_algebra/src/lib.rs`.
- DEL-05-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-05/DEL-05-03/DEL-05-03_forward.csv`; ImplementationEvidence cited: `core/loads/stress_recovery/src/lib.rs#L43`, `core/loads/stress_recovery/src/lib.rs#L254`, `core/product_physics/src/lib.rs#L710`.
- DEL-06-02: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-06/DEL-06-02/DEL-06-02_forward.csv`; ImplementationEvidence cited: `core/rules/expression_evaluator/src/lib.rs`, `core/rules/rule_check_runner/src/lib.rs`, `schemas/rule_check_run_result.schema.json`.
- DEL-06-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-06/DEL-06-03/DEL-06-03_forward.csv`; ImplementationEvidence cited: `core/rules/completeness_checker/src/lib.rs`, `core/rules/rule_check_runner/src/lib.rs`, `schemas/rule_check_run_result.schema.json`.
- DEL-07-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W1/PKG-07/DEL-07-03/DEL-07-03_forward.csv`; ImplementationEvidence cited: `core/gui/editors/engine.py`, `core/gui/editors/__init__.py`, `apps/desktop/src/features/library/LibraryManagerPanel.tsx`, `apps/desktop/src/features/rule-packs/RulePackManagerPanel.tsx`, `apps/desktop/src/services/rulePackService.ts`.
- DEL-12-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-12/DEL-12-03/DEL-12-03_forward.csv`; ImplementationEvidence cited: `core/security/telemetry_policy/controls.py`, `schemas/model.schema.yaml`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- For each producer named, a test asserts the AB-00-06 fields are present on its product-facing diagnostic, or a record names the mapping boundary.
- Editor findings map to the six SPEC section 8 classes, or a record states their own vocabulary.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

None. No affected row's class or T8/T12 reading needs an owner or review decision.

## Notes and open views

- Several rows say "bind into a governed result envelope"; the envelope home for ValidationResult/TransformResult is a held selection (B12, T6-C04 D7). Rows here are T6-C02/C01 (no hold recorded), so no block is set; check D7 before choosing a home.

## Dependencies

B12 (D7, context), CFB-19.

