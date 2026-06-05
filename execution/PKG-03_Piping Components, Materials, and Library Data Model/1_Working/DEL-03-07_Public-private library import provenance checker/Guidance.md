# Guidance: DEL-03-07 Public/private library import provenance checker

## Purpose

This deliverable exists to connect PKG-03 library data models with the project governance boundary: component and material data may be imported, but public acceptance requires recorded provenance and redistribution status, and private data must remain private by default.

## Principles

- Treat provenance as required metadata, not optional commentary.
- Treat redistribution status as a review field. The checker records and gates; it does not make legal conclusions.
- Prefer explicit `TBD`, `REVIEW_NEEDED`, or equivalent unresolved states over guessed acceptance.
- Keep public tests and fixtures invented. Do not use protected standards tables, vendor catalogs, or copied examples.
- Preserve public/private separation at the import boundary and at any diagnostic/export boundary.
- Maintain unit metadata for numeric engineering values; do not introduce unitless accepted records.

## Considerations

- SOW-019 requires documented provenance and redistribution rights before public component data can be accepted.
- SOW-044 requires import mechanisms to record source, provenance, and license metadata for component/material data.
- OPS-K-IP-3 and OPS-K-GOV-4 make protected-content suspicion a stop-and-escalate condition, not a transformation task.
- AB-00-07 indicates adapters must validate units, provenance, redistribution, diagnostics, and public/private boundaries.
- Exact external import formats are still TBD, so this deliverable evidence describes already-parsed payload validation without committing to parser details.

## Current Evidence

- `core/library_import/provenance_checker.py` provides `validate_library_import(...)` for already-parsed material, section, and component library payloads.
- `ImportValidationResult` reports `ACCEPTED_PUBLIC`, `PRIVATE_LOCAL_ONLY`, `REVIEW_REQUIRED`, `REJECTED`, or `QUARANTINE`.
- `ImportFinding.to_diagnostic()` projects findings to PKG-02-style diagnostic envelope fields for downstream compatibility evidence.
- `tests/test_library_import_provenance.py` uses invented fixtures to cover public acceptance, unresolved-rights rejection, private-local handling, missing provenance, protected-content quarantine, unit metadata, and diagnostic-envelope mapping.

## Trade-offs

| Topic | Preferred posture | Reason |
|---|---|---|
| Missing provenance | Reject, block, or flag as not publicly acceptable. | Avoid silent acceptance of unsupported public data. |
| Unclear license status | Mark review-needed/TBD. | The checker cannot make legal conclusions. |
| Private imports | Permit local-private handling only when privacy markers and storage boundaries are respected. | Supports private libraries without bundling protected data. |
| Public test data | Use invented fixtures. | Prevent protected data from entering the public repository. |
| Strictness | Conservative by default. | Scope item notes emphasize contributor certification and missing-provenance flags. |

## Examples And Fixtures

Concrete external import examples are not provided because protected/vendor data examples are forbidden and external import formats are TBD. Current test payloads are invented fixtures only; fixture values are not engineering reference values and do not establish a real license, provenance trail, accepted source catalog, or redistribution policy.

## Conflict Table (for human ruling)

| Conflict ID | Topic | Contenders | Human ruling |
|---|---|---|---|
| DEL-03-07-C1 | Exact license/redistribution disposition vocabulary | Source scope requires metadata and review, but no approved enum names are defined in this deliverable. | TBD |
| DEL-03-07-C2 | Public acceptance authority | Validator can flag missing/uncertain metadata; legal or maintainer acceptance authority is not defined locally. | TBD |
| DEL-03-07-C3 | Public source catalog and policy | The checker gates on metadata values, but no approved public source catalog or legal/license policy is defined locally. | TBD |
| DEL-03-07-C4 | Fixture-value authority | Tests use invented minimal values; no fixture value is approved for engineering reliance. | TBD |
