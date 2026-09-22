# CFB-12 — Mechanics solver verification: restore envelope-binding tests and add boundary tests

**Candidate brief (H2). Not executed.** Area: Solver and nonlinear integration (DEL-04-01, DEL-04-04, DEL-04-05). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Restore a report-facing fixture test asserting the envelope carries nonlinear component identity and NONLINEAR_ASSEMBLED_LOOP assumption/limitation diagnostics (removed by b43cc00c4, PR #787); add a solver-boundary test that the nonlinear path assembles only frame, user-stiffness and curved-bend elements; record the DEL-04-05 module boundary review or cite one; bind harness records to the result schema (or record the exemption) with the named report-facing test; add a fixture provenance index record; record the DEC-025 local sweep as the CI gate REQ-010 means (the hosted-CI alternative needs a new decision packet under DEC-059).

## Affected claims

6 claim rows on 3 deliverable(s): DEL-04-01, DEL-04-04, DEL-04-05.

Classes (portion in this brief / class total): T6-C01 2/180 (Authority NONE); T6-C02 1/116 (Authority NONE); T6-C03 2/34 (Authority REVIEW); T7-C06 1/42 (Authority REVIEW).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-12"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-04-01:SOW#CLM-012/DEL-04-01-REQ-010` | T6-C01 | NONE | — | — | Decide whether the local DEC-025 registered sweep is the CI gate meant here, or add solver cargo tests to hosted CI (see D-05b). |
| `DEL-04-04:SOW#CLM-010/DEL-04-04-REQ-01` | T6-C03 | REVIEW | H3[T6-C03] | — | Add or cite a solver-boundary test asserting the nonlinear path assembles only frame, user-stiffness and curved-bend elements (no local FEA substitution). |
| `DEL-04-04:SOW#CLM-010/DEL-04-04-REQ-08` | T7-C06 | REVIEW | H3[T7-C06];C2 | — | Restore a report-facing fixture test that asserts the envelope carries the nonlinear component identity and NONLINEAR_ASSEMBLED_LOOP assumption/limitation diagnostics wh… |
| `DEL-04-05:SOW#CLM-012/DEL-04-05-RQ-001` | T6-C02 | NONE | — | — | Record the module boundary review named by the requirement, or cite one covering the frozen crate. |
| `DEL-04-05:SOW#CLM-012/DEL-04-05-RQ-005` | T6-C03 | REVIEW | H3[T6-C03] | — | Bind harness records to the result schema, or record that harness records are exempt, and add the named report-facing test. |
| `DEL-04-05:SOW#CLM-022` | T6-C01 | NONE | — | — | Add or cite a fixture provenance index record. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-04-01: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-04/DEL-04-01/DEL-04-01_forward.csv`; ImplementationEvidence cited: `projects/chirality-piping/tools/release/run_evidence_sweep.py`, `.github/workflows/piping-desktop-e2e.yml`.
- DEL-04-04: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-04/DEL-04-04/DEL-04-04_forward.csv`; ImplementationEvidence cited: `core/solver/nonlinear_integration/src/lib.rs::assembled_loop_assumptions`, `core/solver/nonlinear_integration/src/lib.rs::solve_active_set_frame_with_mode_and_springs`, `core/runner/headless/src/result_envelope_binding.rs::NONLINEAR_LOOP_ASSUMPTION_CODE`, `core/product_physics/src/lib.rs::nonlinear_assembled_loop_context`, `core/solver/nonlinear_integration/src/lib.rs::assembled_loop_component_version` ….
- DEL-04-05: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-04/DEL-04-05/DEL-04-05_forward.csv`; ImplementationEvidence cited: `core/solver/performance_harness/Cargo.toml`, `core/solver/performance_harness/README.md`, `core/solver/performance_harness/src/lib.rs::HarnessRunRecord`, `validation/benchmarks/sparse_default_promotion_observation.dec053.json`, `validation/benchmarks/sparse_suitability_observation.dec050.json` ….

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- The restored envelope-binding test fails if the NONLINEAR_ASSEMBLED_LOOP codes are dropped.
- A boundary test fails if a local-FEA element type enters the nonlinear assembly.
- Independent review for the INVARIANT rows (H3 items T6-C03, T7-C06).
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

Protected subject: 3 of 6 rows are at INVARIANT tier or carry an IP_DATA, CLAIMS or SECURITY layer (row layers: BASELINE). An independent review of any repair is required before reliance. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

3 of 6 claim rows carry a block: C2 (1); H3[T6-C03] (2); H3[T7-C06] (1). `H3[<class>]` names the H3 register item for that class (review before repair; mapped in `H3_TOKEN_MAP.csv`). Unblocked rows may proceed separately once selected.

## Notes and open views

- DEL-04-04 REQ-08 is the corpus's only VERIFICATION_REMOVED row; C2 owns whether PR #787's removal is ruled on.
- Hosted CI: DEC-059 keeps the DEC-025 local registered sweep as the merge gate under every branch and requires a new decision packet for any hosted merge-gating; DEC-025 prohibits Actions on the private monorepo. The "add solver cargo tests to hosted CI" option therefore needs a new decision packet and is not executable under this brief. The executable part is recording the DEC-025 sweep as the gate meant by REQ-010.

## Dependencies

C2, H3[T6-C03], H3[T7-C06].

