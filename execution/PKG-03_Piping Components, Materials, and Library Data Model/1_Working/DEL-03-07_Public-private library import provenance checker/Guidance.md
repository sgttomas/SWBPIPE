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
- Exact external import formats are still TBD, so this setup evidence should describe validator behavior without committing to parser details.

## Trade-offs

| Topic | Preferred posture | Reason |
|---|---|---|
| Missing provenance | Reject, block, or flag as not publicly acceptable. | Avoid silent acceptance of unsupported public data. |
| Unclear license status | Mark review-needed/TBD. | The checker cannot make legal conclusions. |
| Private imports | Permit local-private handling only when privacy markers and storage boundaries are respected. | Supports private libraries without bundling protected data. |
| Public test data | Use invented fixtures. | Prevent protected data from entering the public repository. |
| Strictness | Conservative by default. | Scope item notes emphasize contributor certification and missing-provenance flags. |

## Examples

No concrete import examples are provided in this setup pass because the brief forbids protected/vendor data examples and external import formats are TBD. Future examples must use invented data and must not imply a real license, real provenance trail, or legal redistribution conclusion unless a human-approved source is cited.

## Conflict Table (for human ruling)

| Conflict ID | Topic | Contenders | Human ruling |
|---|---|---|---|
| DEL-03-07-C1 | Exact license/redistribution disposition vocabulary | Source scope requires metadata and review, but no approved enum names are defined in this deliverable. | TBD |
| DEL-03-07-C2 | Public acceptance authority | Validator can flag missing/uncertain metadata; legal or maintainer acceptance authority is not defined locally. | TBD |
