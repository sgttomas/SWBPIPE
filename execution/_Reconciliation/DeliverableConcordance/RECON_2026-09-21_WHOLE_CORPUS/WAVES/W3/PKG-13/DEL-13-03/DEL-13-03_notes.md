# DEL-13-03 notes — W3 PKG-13, worker G1

Forward: 112 rows (63 required keys). Blocks split into .rNN: CLM-004, 005, 006, 013, 023 and 028. Sub-claims: CLM-022.s01 and three .sNN on Architecture Basis Injection. Reverse: 323 capabilities. Shared judgments: `../_WORKER_DEL-13-01_NOTES.md`.

## Path aliases
- The engine is `core/constraints/validation/engine.py` with `__init__.py`; the test is `tests/test_constraint_validation.py`.
- The dependency validator is cited as `projects/chirality-piping/tools/validation/validate_dependencies_schema.py`. A root copy of the same name also exists.

## Judgment calls
- **CLM-013.r01 (R1) is PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE.** The engine checks completeness, references, provenance and units. It computes no conflicts: for conflict kinds it only emits an AVAILABLE info diagnostic or re-emits a `validation_status` supplied in the input.
- **CLM-013.r06 and CLM-028.r06 are PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · OWNER.** The output is a Python dict, not a DEC-010 result envelope. The envelope is held under D-41 T2C because no producer/home is accepted.
- **STATUS#remaining/R01 is ALIGNED** with `OPEN_ACTION: CLM-013.r06`.
- **FG-DEL-13-03-01 (CLM-004.r05 and ABI.s03) is IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT · PROJECT_BASELINE · OWNER (MEDIUM).** DEC-009 adopts Rust core/application services; the engine is stdlib Python, and I found no ruling that permits it. This is likely a corpus-wide pattern (many `core/*` modules are Python), so R3 may want to cluster it.
- **F7:** `validate_constraint_envelope` has only test callers. Every ALIGNED engine row carries `PRODUCT_CALLER: NONE`.
- **SOW SURFACE is CP-04.** The engine module docstring says "OpenPipeStress". The SOW text itself is clean.
- **CP-09.** Parity binds sha256 cde7f4b4…, not the frozen fef7cbf2….
- **Held-residual declarations are ALIGNED as accurate declarations.** This covers CLM-008, 017, 025 and 033, each with GAP_WORDING_CHECKED. The open work sits on CLM-013.r06.

## Canonical departures
None.

## Convention friction
- F1 against "hold" declarations: a block that accurately declares a residual as held was kept ALIGNED, and the gap was placed on the requirement row. A verifier may read F1 more strictly.
- CLM-021 (four-documents source hierarchy) was disposed directly as CP-01, not split.

## UNKNOWN rows
None.

## Reverse pass and view of sealed rows
The reverse pass did not change my view of this ledger. While auditing CLM-006.r04 I confirmed that the engine's dimension set equals the PKG-02 DimensionId (it includes `force_per_length`). That exposed the error in the sealed DEL-13-01 and DEL-13-02 schemas, which is recorded in their notes.

## Batch consistency
PASS, 0 findings.

## Selectability
NOT_APPLICABLE on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
