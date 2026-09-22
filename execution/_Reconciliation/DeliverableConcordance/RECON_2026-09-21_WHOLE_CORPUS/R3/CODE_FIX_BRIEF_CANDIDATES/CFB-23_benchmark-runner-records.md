# CFB-23 — Benchmark and regression runner records; fixture provenance index; GUI gate evidence

**Candidate brief (H2). Not executed.** Area: Benchmarks and quality gates (DEL-09-01, DEL-09-03, DEL-09-05). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Carry solver version, assumptions, provenance and limitations into suite-run outputs (runner SuiteRunReport); produce the dedicated provenance/redistribution index for all 25 fixtures; distinguish exact from model-approximate expected values in the fixture schema; carry iteration count and policy reference into RegressionCaseDetail; add accessibility/usability evidence to the GUI gate's required evidence (thresholds may stay TBD).

## Affected claims

10 claim rows on 3 deliverable(s): DEL-09-01, DEL-09-03, DEL-09-05.

Classes (portion in this brief / class total): T6-C01 10/180 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-23"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-09-01:SOW#CLM-004.r07` | T6-C01 | NONE | — | — | Carry solver version, assumptions, provenance and limitations into suite-run outputs (runner SuiteRunReport) or record why not. |
| `DEL-09-01:SOW#CLM-006.r08` | T6-C01 | NONE | — | — | Produce the dedicated provenance/redistribution index for the current 25-fixture inventory (the derivative index covers 24; MECH-TP-DEC092 added 2026-08-03 is not in it). |
| `DEL-09-01:SOW#CLM-013/DEL-09-01-RQ-005` | T6-C01 | NONE | — | — | As CLM-004.r07: carry solver version, assumptions, provenance and limitations into suite-run outputs. |
| `DEL-09-01:SOW#CLM-015.r06` | T6-C01 | NONE | — | — | As CLM-004.r07. |
| `DEL-09-01:SOW#CLM-024.r06` | T6-C01 | NONE | — | — | As CLM-004.r07. |
| `DEL-09-01:SOW#CLM-025` | T6-C01 | NONE | — | — | Dedicated provenance index for all 25 fixtures (CLM-006.r08); record solver version with runner settings. |
| `DEL-09-01:SOW#CLM-032.r02` | T6-C01 | NONE | — | — | Distinguish exact from model-approximate expected values in the fixture schema (for example the boosted-rigidity expansion-loop comparison). |
| `DEL-09-03:SOW#CLM-027.r05` | T6-C01 | NONE | — | — | Carry iteration count and policy reference into the regression result record (runner RegressionCaseDetail) or record why not. |
| `DEL-09-05:SOW#CLM-004` | T6-C01 | NONE | — | — | Add accessibility/usability evidence to the GUI gate's required evidence (thresholds may stay TBD). |
| `DEL-09-05:SOW#CLM-021.s02` | T6-C01 | NONE | — | — | Add accessibility/usability evidence to the GUI gate's required evidence. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-09-01: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-09/DEL-09-01/DEL-09-01_forward.csv`; ImplementationEvidence cited: `core/runner/headless/src/benchmark_binding.rs`, `validation/benchmarks/mechanics/src/lib.rs#L5457`, `validation/evidence/benchmarks/BENCHEVID_DEL0901_20260720T062342Z_e315fb8406d4/FAMILY_PROVENANCE_INDEX.csv`, `validation/benchmarks/mechanics/src/lib.rs::BenchmarkProvenance`, `validation/hand_calcs/mechanics/README.md` ….
- DEL-09-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-09/DEL-09-03/DEL-09-03_forward.csv`; ImplementationEvidence cited: `validation/benchmarks/nonlinear/src/lib.rs::assembled_convergence_observations`, `core/runner/headless/src/benchmark_binding.rs`.
- DEL-09-05: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-09/DEL-09-05/DEL-09-05_forward.csv`; ImplementationEvidence cited: `projects/chirality-piping/docs/RELEASE_QUALITY_GATES.md#L69`, `projects/chirality-piping/docs/RELEASE_QUALITY_GATES.md#L91`, `projects/chirality-piping/docs/RELEASE_QUALITY_GATES.md#L107`, `projects/chirality-piping/docs/RELEASE_QUALITY_GATES.md#L125`, `projects/chirality-piping/validation/evidence/gates/GATE_GUI_20260711T032542Z_e2ea37194c8a.json`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Suite-run output schema tests assert solver version, assumptions, provenance and limitations.
- The provenance index lists every current fixture (25 at the freeze).
- The GUI gate record lists accessibility/usability evidence as required.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

None. No affected row's class or T8/T12 reading needs an owner or review decision.

## Notes and open views

- DEL-09-05 accessibility evidence relates to the D-68 vs DEL-00-05 accessibility item in A10 and the usability basis in C3; no block is set because the class is T6-C01.

## Dependencies

A10 (context), C3 (context).

