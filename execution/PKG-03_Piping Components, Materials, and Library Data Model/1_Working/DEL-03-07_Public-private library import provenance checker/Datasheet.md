# Datasheet: DEL-03-07 Public/private library import provenance checker

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-03-07 |
| Package ID | PKG-03 |
| Package Name | Piping Components, Materials, and Library Data Model |
| Deliverable Type | BACKEND_FEATURE_SLICE |
| Scope Items | SOW-019; SOW-044 |
| Objectives | OBJ-002; OBJ-004 |
| Anticipated Artifacts | library import validator; provenance tests |
| Decomposition Basis | execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.7 |
| Status | Implementation evidence reconciled; lifecycle remains governed by `_STATUS.md` |

## Attributes

| Attribute | Source-grounded value |
|---|---|
| Primary function | Validate already-parsed material, section, and component library payloads so source, provenance, license, contributor/reviewer disposition, redistribution metadata, privacy posture, and unit metadata are checked before import acceptance. |
| Data boundary | Public component data requires documented provenance and redistribution rights; private library data must not become a bundled public default. |
| Input format list | TBD. The implemented checker validates already-parsed payloads and intentionally does not parse external import formats. |
| Rights determination authority | TBD. The checker can record and flag metadata, but legal acceptance requires human/project review. |
| Protected-content response | Suspected protected standards or vendor content is quarantined and escalated rather than transformed into public data. |
| Unit handling | Imported component/material numeric values must retain unit and dimension metadata; no unit defaults are introduced by the checker. |

## Conditions

- Public data contributions are acceptable only when provenance and redistribution status are documented.
- Import mechanisms must record source, provenance, and license metadata for component and material data.
- Missing solve-required or rule-check-required values are findings, not silent defaults.
- Public deliverable evidence must not include protected standards text, protected tables, vendor data, or real rights/provenance examples.

## Construction

Reconciled implementation evidence contains:

- `core/library_import/provenance_checker.py`, a stdlib-only validator for already-parsed material, section, and component library payloads;
- `validate_library_import(...)`, returning `ACCEPTED_PUBLIC`, `PRIVATE_LOCAL_ONLY`, `REVIEW_REQUIRED`, `REJECTED`, or `QUARANTINE` outcomes;
- metadata checks for library-level and record-level provenance fields: source name/location, source license, contributor, contributor certification, redistribution status, and review status;
- public/private disposition checks that block private-only or unresolved-rights data from public acceptance while permitting eligible private imports to remain local/private;
- protected-content quarantine signaling for suspected protected or rejected sources;
- nested numeric value checks that require unit/dimension metadata and value-level provenance;
- `ImportFinding.to_diagnostic()` and `ImportValidationResult.diagnostics`, which project findings into a PKG-02-style diagnostic envelope with code, severity, class, source, affected object, remediation, and provenance;
- `core/library_import/README.md`, documenting the boundary that the checker does not parse external formats and does not make legal conclusions;
- `tests/test_library_import_provenance.py`, covering accepted public imports, public rejection for unresolved rights, private-local handling, missing provenance, protected-content quarantine, unit metadata, and diagnostic-envelope mapping using invented fixtures.

Remaining unresolved items are concrete external import formats and parser contracts, accepted source catalogs and legal/license policy, fixture-value authority for engineering reliance, dependency satisfaction outside this bounded evidence, human disposition of review findings, and lifecycle closure.

## References

- `_CONTEXT.md` for deliverable identity, scope envelope, objectives, and architecture-basis injection.
- `_REFERENCES.md` for governing source pointers.
- `docs/_Registers/Deliverables.csv` row DEL-03-07.
- `docs/_Registers/ScopeLedger.csv` rows SOW-019 and SOW-044.
- `docs/_Registers/ContextBudgetQA.csv` row DEL-03-07.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7.
- `docs/CONTRACT.md` invariants OPS-K-IP-1..3, OPS-K-DATA-1..3, OPS-K-UNIT-1, OPS-K-PRIV-1, OPS-K-GOV-4, and OPS-K-AGENT-1..4.
