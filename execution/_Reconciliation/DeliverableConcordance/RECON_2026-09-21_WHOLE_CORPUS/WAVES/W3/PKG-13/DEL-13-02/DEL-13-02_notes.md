# DEL-13-02 notes — W3 PKG-13, worker G1

Forward: 85 rows (73 required keys; CLM-019 split into .rNN; CLM-024.s01; two .sNN on Architecture Basis Injection). Reverse: 323 capabilities. Shared judgments: `../_WORKER_DEL-13-01_NOTES.md`.

## Path aliases
- `schemas/constraint.schema.json` and `tests/test_constraint_schema.py` are under the project root.
- The SOW cites `INIT.md`. That file was removed from the project on 2026-07-04 (9c4caf8fd) and superseded by the `init/` launcher prompts.

## Judgment calls
- **INIT.md.** The dead citation is disposed CP-02 on the reference/prerequisite blocks (CLM-006, CLM-017). Requirement rows that merely cite INIT.md among other sources stay ALIGNED, because their substance holds (C6(b)); the drift is noted on each.
- **R-13-02-005 is PARTIALLY_IMPLEMENTED · POSSIBLE_DEFECT (MEDIUM).** This is the same `Parameter.value`/`value_kind` loophole as DEL-13-01. Note that the DEL-13-03 engine catches this mismatch at runtime (`CV-PARAMETER-QUANTITY-KIND-MISMATCH`), but the schema does not.
- **STATUS#remaining/R01 is DOCUMENTED_UNIMPLEMENTED · NOT_STARTED · OWNER.** It asks for the human disposition of finding PKG13-DEL-13-02-PKG02-001. The text is accurate, and no governing row carries the action (F2).
- **CP-09.** PASS parity records bind SOW sha256 43d9ea2f…, not the frozen 78a2ff6e…. This corrects EVIDENCE_MAP's NONE_FOUND.
- **SOW SURFACE is CP-04** (constraint schema `$id`/title).

## Canonical departures
None.

## Convention friction
CONTEXT#scope-detail is a run-together block with no CS row. It was judged normally: the statements match ScopeLedger.

## UNKNOWN rows
None.

## Reverse pass and view of sealed rows
The reverse pass did not change my view. A finding made after sealing (while auditing DEL-13-03) does. It is an error I would correct:
- The PKG-02 DimensionId in `schemas/units.schema.yaml` includes `force_per_length`. `constraint.schema.json` Quantity.dimension lacks it.
- So CLM-003 (unit posture "matches the accepted PKG-02 dimension vocabulary"), CLM-009 ("PKG-02 dimension vocabulary alignment") and CLM-012/R-13-02-005 ("exact alignment") should not be ALIGNED. They should be STALE_REVIEW_OR_EVIDENCE or IMPLEMENTED_DIFFERENTLY (cause POSSIBLE_DEFECT).
- The DEL-13-03 engine accepts `force_per_length`, so a value can pass the engine and still fail the schema enum.
- For the verifier and a fresh worker.

## Batch consistency
PASS, 0 findings.

## Selectability
NOT_APPLICABLE on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
