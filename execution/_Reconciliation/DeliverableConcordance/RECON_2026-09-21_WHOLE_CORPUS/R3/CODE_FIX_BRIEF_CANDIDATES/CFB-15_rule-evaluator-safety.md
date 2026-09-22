# CFB-15 — Rule evaluator and completeness checker: protected-content, bypass and bounds tests

**Candidate brief (H2). Not executed.** Area: Rule engine (DEL-06-01, DEL-06-02, DEL-06-03); protected subject. Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Add a schema conditional so public_invented_example privacy class requires the invented redistribution class and a notice; add the evaluator's public-example protected-content test family and explicit evaluation bounds (expression size/depth); write the evaluator threat model and final diagnostic taxonomy; add a test that no protected defaults, code-specific formulas or allowables ship with the completeness checker; add plugin/adapter bypass-attempt tests once a governed dispatch path exists.

## Affected claims

9 claim rows on 3 deliverable(s): DEL-06-01, DEL-06-02, DEL-06-03.

Classes (portion in this brief / class total): T6-C01 1/180 (Authority NONE); T6-C03 8/34 (Authority REVIEW).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-15"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-06-01:SOW#CLM-011/REQ-06-01-011` | T6-C03 | REVIEW | H3[T6-C03] | — | Add a schema conditional (public_invented_example privacy class requires invented_non_engineering_example redistribution and a notice), or record that the DEL-06-05 exam… |
| `DEL-06-02:SOW#CLM-006.r05` | T6-C03 | REVIEW | H3[T6-C03];B10 | — | Bind any adapter or plugin invocation of the evaluator to a governed runtime dispatch path with no-bypass tests (Remaining R03) |
| `DEL-06-02:SOW#CLM-013/REQ-06-02-010` | T6-C03 | REVIEW | H3[T6-C03];B10 | — | Bind any adapter or plugin evaluator invocation to a governed runtime dispatch path with no-bypass tests (Remaining R03) |
| `DEL-06-02:SOW#CLM-013/REQ-06-02-011` | T6-C03 | REVIEW | H3[T6-C03] | — | Add the public-example protected-content test family and evaluator plugin/adapter bypass-attempt tests (Remaining R01) |
| `DEL-06-02:SOW#CLM-016/REQ-06-02-010` | T6-C03 | REVIEW | H3[T6-C03];B10 | — | Add evaluator bypass-attempt tests once a governed dispatch path exists (Remaining R01, R03) |
| `DEL-06-02:SOW#CLM-017.s01` | T6-C01 | NONE | — | — | Write the evaluator threat model and the final diagnostic taxonomy |
| `DEL-06-02:SOW#CLM-023` | T6-C03 | REVIEW | H3[T6-C03] | — | Complete step 9's public/private data-boundary and plugin/adapter bypass test families (Remaining R01) |
| `DEL-06-02:SOW#purpose-and-objective-traceability/OUT-001` | T6-C03 | REVIEW | H3[T6-C03] | — | Add explicit evaluation bounds (expression size or depth limits) and bind evaluator findings into a governed result envelope |
| `DEL-06-03:SOW#CLM-014` | T6-C03 | REVIEW | H3[T6-C03] | — | Add a test (or registered check) asserting that no protected defaults, code-specific formulas or material allowables ship with the checker and its fixtures |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-06-01: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-06/DEL-06-01/DEL-06-01_forward.csv`; ImplementationEvidence cited: `schemas/rule_pack.schema.yaml`, `examples/rule_packs/invented_demo.yaml`.
- DEL-06-02: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-06/DEL-06-02/DEL-06-02_forward.csv`; ImplementationEvidence cited: `core/adapters/framework/adapter_framework.py`, `core/adapters/framework/plugin_verification.py`, `core/rules/expression_evaluator/src/lib.rs`, `core/rules/expression_evaluator/tests/conformance_corpus.rs`, `core/rules/expression_evaluator/README.md` ….
- DEL-06-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-06/DEL-06-03/DEL-06-03_forward.csv`; ImplementationEvidence cited: `core/rules/completeness_checker/src/lib.rs`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Negative tests: a public example lacking the invented redistribution class fails schema validation; an over-limit expression is rejected.
- A registered check asserts no protected defaults ship with the checker (synthetic markers only).
- Independent review (H3 item for T6-C03) before reliance.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

Protected subject: 8 of 9 rows are at INVARIANT tier or carry protected layers (IP_DATA, SECURITY). An independent review of any repair is required before reliance. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

8 of 9 claim rows carry a block: B10 (3); H3[T6-C03] (8). `H3[<class>]` names the H3 register item for that class (review before repair). Unblocked rows may proceed separately once selected.

## Notes and open views

- The three CP-11 no-bypass rows hold only because no dispatch path exists; they wait on B10.
- DEL-06-01 REQ-06-01-011 "or record/narrow" restates an INVARIANT; narrowing is an owner item (UNASSIGNED in the H2 return).

## Dependencies

H3[T6-C03], B10.

