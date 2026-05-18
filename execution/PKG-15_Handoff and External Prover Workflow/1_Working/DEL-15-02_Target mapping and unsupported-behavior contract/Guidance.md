# Guidance: DEL-15-02 Target mapping and unsupported-behavior contract

## Purpose

This deliverable prevents silent loss of critical handoff assumptions or unsupported target behavior. It supplies the contract surface for explaining how internal OpenPipeStress entities map into a handoff target and where a target cannot carry the source semantics exactly.

## Principles

| Principle | Guidance | Source |
|---|---|---|
| Explicit non-support | Unsupported target behavior must be visible as a finding or flag, not hidden by defaults. | `_CONTEXT.md` Context Envelope; `docs/CONTRACT.md` OPS-K-DATA-2 |
| Unit safety | Unit-bearing values and exports must preserve unit and dimensional metadata or diagnostics. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/DIRECTIVE.md` Principles |
| Provenance preservation | Mapped entities, rule/library references, and reliance-affecting metadata should carry source/provenance references where supported by upstream contracts. | `docs/CONTRACT.md` OPS-K-IP-2; `docs/TYPES.md` Provenance |
| Private-data restraint | Public handoff artifacts should expose references, checksums, review status, and allowed metadata without copying private or protected payloads. | `docs/IP_AND_DATA_BOUNDARY.md` Private user data; `docs/SPEC.md` result export boundary |
| Professional-boundary restraint | Handoff support is not software certification, sealing, approval, authentication, or code-compliance determination. | `docs/CONTRACT.md` OPS-K-AUTH-1; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEC-015 |
| Deferred target specificity | Target lists, package container details, and target-specific mapping strategy remain TBD until accepted. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-015 |

## Considerations

- Treat target mapping as traceability and disclosure metadata. It does not prove that the target tool reproduced the source model correctly.
- Treat unsupported and approximate behavior flags as review evidence. They should help a human identify gaps, not resolve those gaps automatically.
- Favor references to rule packs, libraries, manifests, diagnostics, and hashes over copying private formulas, protected standards text, proprietary values, or private rule-pack payloads.
- Keep target-specific behavior conservative. If an external target cannot represent an OpenPipeStress entity, operation, unit semantic, diagnostic, provenance link, or assumption exactly, record `TBD`, unsupported, or approximate behavior rather than silently narrowing the model.
- Keep exact taxonomy values and schema property names as TBD until a governing source or accepted implementation artifact exists.

## Trade-offs

| Trade-off | Conservative position |
|---|---|
| Generic target contract vs target-specific convenience | Prefer generic, schema-first metadata until accepted target-specific commercial parser work is in scope. |
| Rich mapping details vs private-data exposure | Preserve traceability and review evidence while excluding private/protected payloads by default. |
| Approximation support vs false confidence | Allow approximate behavior disclosure, but do not let it imply equivalence, validation, approval, or compliance. |
| Automated workflow vs human authority | The software may package and disclose handoff evidence; professional reliance remains external and human-owned. |

## Examples

| Example topic | Status |
|---|---|
| Specific commercial target mapping record | TBD - no accepted target list or target-specific mapping strategy is available in the accessible source set. |
| Unsupported behavior taxonomy values | TBD - taxonomy artifact is anticipated, but exact values are not established by accessible source material. |
| Approximate behavior flag semantics | ASSUMPTION - should indicate non-exact target representation where supported by the final contract; exact semantics remain TBD. |

## Conflict Table (for human ruling)

| Conflict ID | Conflict (short statement) | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
|---|---|---|---|---|---|---|
| CT-001 | Dependency-extract v3.1 enum guidance would normalize dependency rows, but the project task rule requires preserving approved DAG-002 mirror rows as ACTIVE without reclassification. | `skills/dependency-extract/SKILL.md` Function 3 and canonical enums | User task instruction; `_DEPENDENCIES.md` Authority Boundary; `Dependencies.csv` approved DAG-002 rows | `Dependencies.csv`; `_DEPENDENCIES.md`; final workflow report | Preserve the approved mirror unchanged for this run. | TBD |

## References

- `_CONTEXT.md`
- `_REFERENCES.md`
- `Dependencies.csv`
- `execution/_Decomposition/SOFTWARE_DECOMP.md`
- `docs/CONTRACT.md`
- `docs/DIRECTIVE.md`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `docs/IP_AND_DATA_BOUNDARY.md`
