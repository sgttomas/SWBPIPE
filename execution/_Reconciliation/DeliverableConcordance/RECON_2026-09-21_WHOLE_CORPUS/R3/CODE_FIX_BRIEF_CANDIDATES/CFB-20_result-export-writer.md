# CFB-20 — Result export and parity: fixture protected-content scan, unit-conversion witnesses, parity refresh

**Candidate brief (H2). Not executed.** Area: Result export (DEL-08-04, DEL-08-06). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Extend a protected-content and claims scan to result fixtures and exports (or record an equivalent check); add target-format numerical unit-conversion witnesses; refresh parity against the frozen SOW and extend engine/desktop parity beyond the empty fixture.

## Affected claims

4 claim rows on 2 deliverable(s): DEL-08-04, DEL-08-06.

Classes (portion in this brief / class total): T6-C01 3/180 (Authority NONE); T6-C03 1/34 (Authority REVIEW).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-20"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-08-04:SOW#CLM-013/V-7` | T6-C03 | REVIEW | H3[T6-C03] | — | OC: run the DEC-058 scan over AC-5 (owner act), noting the engine's former-name phrase gap (FG-DEL-08-05-02) |
| `DEL-08-04:SOW#production-and-verification-method-praxeology/VER-001` | T6-C01 | NONE | — | — | Refresh parity against the frozen SOW, add fixture scanning and exercise rule-pack refs through the product writer |
| `DEL-08-04:STATUS#remaining/R01` | T6-C01 | NONE | — | — | Target-format numerical unit-conversion witnesses |
| `DEL-08-06:SOW#production-and-verification-method-praxeology/VER-001` | T6-C01 | NONE | — | — | Refresh parity against the frozen SOW and extend engine/desktop parity beyond the empty fixture |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-08-04: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-08/DEL-08-04/DEL-08-04_forward.csv`; ImplementationEvidence cited: `tools/validation/validate_claims_language.py`, `tests/test_claims_language_surface.py`, `fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json`, `schemas/results.v0.1.schema.yaml`, `core/reporting/result_export/src/lib.rs` ….
- DEL-08-06: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-08/DEL-08-06/DEL-08-06_forward.csv`; ImplementationEvidence cited: `core/reporting/state_comparison_handoff_sections/engine.py`, `core/reporting/state_comparison_handoff_sections/__init__.py`, `apps/desktop/src/features/report/stateComparisonHandoffSections.ts`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Scanner tooling covers result fixtures and fails on a planted synthetic marker. Running the DEC-058 release scan is the owner's act, not an acceptance step of this brief.
- Unit-conversion witness tests cover each target format.
- Independent review for the INVARIANT row (H3 item for T6-C03).
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

Protected subject: 1 of 4 rows are at INVARIANT tier or carry an IP_DATA, CLAIMS or SECURITY layer (row layers: CLAIMS, IP_DATA). An independent review of any repair is required before reliance. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

1 of 4 claim rows carry a block: H3[T6-C03] (1). `H3[<class>]` names the H3 register item for that class (review before repair; mapped in `H3_TOKEN_MAP.csv`). Unblocked rows may proceed separately once selected.

## Notes and open views

- The DEL-08-04 V-7 remainder (the OC: run the DEC-058 scan over AC-5) is an owner act under DEC-058, which makes the owner the scan owner. It is outside this code-fix scope; only the scanner tooling part stays here, behind the H3 review.

## Dependencies

H3[T6-C03], CFB-22.

