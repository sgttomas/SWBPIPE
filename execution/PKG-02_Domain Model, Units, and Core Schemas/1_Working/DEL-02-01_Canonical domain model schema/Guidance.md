# Guidance: DEL-02-01 Canonical domain model schema

## Purpose

DEL-02-01 exists to make the OpenPipeStress domain model explicit and machine-readable before solver, GUI, reporting, persistence, plugin, and validation work depend on it. The schema should be a public contract for open mechanics and workflow data while preserving the boundary that code-specific values, protected standards data, proprietary libraries, and professional approval remain user/private or human-controlled. Sources: `docs/DIRECTIVE.md` Sections 1 through 3; `docs/SPEC.md` Sections 1 through 3; `docs/_Registers/ScopeLedger.csv` row SOW-041.

## Principles

- Use the schema as a boundary contract, not as a place to encode protected engineering standards or project-specific compliance rules.
- Keep every addressable model object stable enough for editing, solving, reporting, diagnostics, and audit trails.
- Treat units as schema-visible data. Dimensional values should not be plain numbers without unit context unless the value is dimensionless and the schema makes that clear.
- Treat provenance and redistribution/private status as first-class data where engineering reliance, public contribution, or report disclosure may be affected.
- Preserve the analysis-status vocabulary: mechanics solved, user rule checked, and human-approved-for-project are different states. The software must not turn any of them into automatic code compliance.
- Design result/report records so warnings, assumptions, missing data, diagnostics, source notes, hashes/checksums, and limitations can be reproduced in reports.
- Keep implementation choices that SCA-001 leaves TBD, such as schema code-generation tooling and physical project packaging, visible as TBD until a human-approved decision exists.

## Considerations

- PKG-02 owns the canonical schema surface, but detailed material/component library behavior belongs to PKG-03, rule-pack internals belong to PKG-06, reports belong to PKG-08, adapters/APIs belong to PKG-10, and privacy controls belong to PKG-12. DEL-02-01 should define shared records and references without silently taking over those deliverables.
- `docs/SPEC.md` Section 3 lists a broader minimum domain object set than the short DEL-02-01 description. Include required references or reusable definitions where the canonical model cannot function without them, and mark boundary decisions as `TBD` when the correct owner is unclear.
- The PRD Appendix A example is useful only as a schema-shape illustration. Do not copy it wholesale into public fixtures, and do not treat its demonstration values as engineering defaults.
- Public examples and validation fixtures should be original/invented or documented public/permissive data. If a value looks code-derived, proprietary, or copied from a protected source, quarantine/escalate rather than adapting it.
- The anticipated `docs/TYPES.md` update is outside this run's write scope. New type names or status/provenance labels should be proposed for human review before the file is changed.
- PROPOSAL: Treat object-family names as draft schema vocabulary until `docs/TYPES.md` is updated or a human ruling freezes the names. In particular, use `LoadCase` for primitive solved loads, `Combination` for algebraic combinations, and `Load` only as an umbrella term when following register/decomposition prose.
- Human rulings for conflict-table rows, schema file layout, `$id` URI, code-generation tooling, fixture organization, and persistence/hashing handoff need a durable reference before downstream acceptance depends on them. The exact ruling record type or path is `TBD` until the project authority selects one.

Vocabulary and boundary notes:

| Term or boundary | DEL-02-01 usage guidance | Deferred or owner-sensitive content |
|---|---|---|
| `Load`, `LoadCase`, `Combination` | Model the primitive and algebraic records explicitly; keep `Load` as a broad planning label unless the future type vocabulary says otherwise. | Code-specific load-combination requirements remain user/rule-pack supplied and must not become public defaults. |
| `Support` and `Restraint` | Use a consistent schema vocabulary or alias relationship before freezing names; preserve enough structure for model references and diagnostics. | Nonlinear behavior details and solver convergence semantics remain with solver/support deliverables unless explicitly pulled into shared schema records. |
| `Section` | Include common reference/record structure where elements, materials, components, and reports need it. | Detailed section property calculators and dimensional table population remain outside DEL-02-01. |
| `RulePack` reference | Store reference/checksum/source/private-status links needed by projects, results, and reports. | Rule expression grammar, protected formulas, and evaluator behavior remain outside this deliverable. |
| Result and Report records | Preserve diagnostics, affected-object references, provenance, input manifest, hash/checksum links, warnings, assumptions, and professional-boundary notices. | Report rendering, professional signoff workflow, and human approval records require their owning deliverables or human rulings. |

## Trade-offs

| Topic | Option | Trade-off | Guidance |
|---|---|---|---|
| Schema shape | Single `schemas/model.schema.yaml` with `$defs` | Matches the anticipated artifact but can become large | Use `$defs` and clear references; split later only by human-approved schema architecture change. |
| Domain breadth | Include every object in one schema | Improves consistency but risks crossing into PKG-03/PKG-06/PKG-08 details | Define common identities, references, and envelopes; defer detailed library/rule/report internals to their packages. |
| Required fields | Require all potentially needed engineering data | Strong validation but may block mechanics-only workflows | Distinguish solve-required, rule-check-required, report-required, and optional fields; missing values become explicit findings. |
| Public examples | Use realistic industrial data | Better realism but high IP/provenance risk | Use invented or permissively licensed examples with provenance and non-compliance notices. |
| Hashing and canonicalization | Define implementation now | Improves reproducibility but overlaps DEL-02-05 persistence details | Keep schema compatible with AB-00-04; leave physical package and migration framework TBD. |
| Persistence handoff | Encode compatibility hooks now, but defer storage implementation | Keeps model schema aligned with deterministic round trips while avoiding premature ownership of package/container and migration mechanics | Record version, migration-status, unit, provenance, rule-pack reference, diagnostic, and hash metadata hooks where needed; route physical package/container and migration framework decisions to DEL-02-05 or human authority. |

## Examples

Example schema-shape guidance, not a final schema:

- A `Project` record should identify the project container, unit system, storage/privacy posture, rule-pack references, and report settings.
- A `Node` record should expose coordinates with units and should support six-degree-of-freedom solver semantics by reference to the solver/domain model.
- An `Element` record should reference start/end nodes, material and section/component data, local coordinate basis, and result-station metadata where needed.
- A `Material` or `Component` record should carry user/private or public/permissive provenance status before engineering reliance or redistribution.
- A `Result` record should distinguish mechanical result quantities from user-rule-check outcomes and should attach diagnostics/warnings to affected objects.
- A `Report` record should identify input manifest, model hash, warnings, assumptions, provenance summary, result summaries, rule-pack checksum references, and professional-boundary notice.

All examples above are structural guidance from `docs/SPEC.md` Sections 3, 7, and 8 and `docs/PRD.md` Sections 10, 13, and 15. They are not code-compliance requirements and do not supply engineering values.

## Conflict Table (for human ruling)

| Conflict ID | Conflict (short statement) | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| C-02-01-001 | Objective mapping differs: DEL-02-01 context/deliverable register lists OBJ-001, while SOW-041 scope ledger also maps OBJ-012. | `_CONTEXT.md` Objective Support; `docs/_Registers/Deliverables.csv` row DEL-02-01; `docs/_Decomposition/SOFTWARE_DECOMP.md` Objective table | `docs/_Registers/ScopeLedger.csv` row SOW-041 | Datasheet Identification; Specification Scope; Procedure Records | Treat OBJ-001 as the deliverable-owned objective for this run and treat OBJ-012 as directionally relevant package/scope context until a human updates registers. | TBD |
| C-02-01-002 | Decomposition revision wording is stale in pointers: `_REFERENCES.md` says accepted v0.2 and `docs/README.md` status says v0.3, while `_CONTEXT.md`, the user brief, and the current decomposition basis identify revision 0.4. | `_REFERENCES.md` Decomposition and Registers; `docs/README.md` Status | `_CONTEXT.md` Decomposition Reference and Architecture Basis Injection; user brief | Datasheet References; Procedure Prerequisites; run record | Use `_CONTEXT.md` revision 0.4 and the user-provided `DECOMPOSITION_REF` for this run; leave metadata/governance pointers unchanged because they are outside write scope. | TBD |

No engineering-value conflict was found in the accessible sources. No protected standards/code data was used.
