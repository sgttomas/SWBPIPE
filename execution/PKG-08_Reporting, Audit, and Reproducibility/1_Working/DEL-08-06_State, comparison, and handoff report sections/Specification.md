# Specification: DEL-08-06 State, comparison, and handoff report sections

## Scope

This deliverable covers backend report sections for state/run records, deterministic comparison records, and handoff manifest records. The sections must support auditable calculation reports and preserve professional, IP, privacy, provenance, checksum, unit, and missing-data boundaries.

This deliverable excludes final report styling/layout, GUI presentation, CLI runtime behavior, API transport, arbitrary project-file reading, solver-internal execution, protected-content linting implementation, private redaction/export control implementation, and any automatic professional approval or code-compliance status. These exclusions are source-grounded in `docs/SPEC.md` section 9 and the approved upstream dependency mirror.

Implementation file paths, concrete API names, exact schema fragments, and dependency versions are TBD unless later resolved by human-approved implementation work.

## Requirements

| ID | Requirement | Source | Verification |
|---|---|---|---|
| DEL-08-06-R1 | The deliverable shall provide report-section behavior for model states, analysis runs, comparisons, and handoff manifests. | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-08-06 | Tests or review evidence shall show all three anticipated artifact families are represented. |
| DEL-08-06-R2 | Report sections shall support SOW-024 content: inputs, sources, warnings, assumptions, results, rule-pack checksums, and limitations. | `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` row SOW-024; `docs/SPEC.md` section 9 | Section fixtures or schema tests shall include fields or references for each SOW-024 content category, or explicit TBD diagnostics where unavailable. |
| DEL-08-06-R3 | Report sections shall preserve the professional boundary and shall not declare code compliance, certification, sealing, approval, authentication, endorsement, or professional reliance. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/SPEC.md` section 9; `docs/DIRECTIVE.md` sections 2.2 and 3 | Boundary-wording tests or protected phrase checks shall reject automatic professional-approval claims. |
| DEL-08-06-R4 | Software-generated analysis statuses used by report sections shall not include `HUMAN_APPROVED_FOR_PROJECT` or equivalent automatic human approval/professional reliance labels. | `docs/SPEC.md` sections 4.3 and 9; `docs/TYPES.md` companion schema notes | Status serialization tests shall verify only permitted software-generated status values are emitted. |
| DEL-08-06-R5 | Missing solve-required or rule-check-required values shall remain explicit findings with diagnostics and provenance, not hidden defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` sections 4.3 and 9 | Tests shall verify missing inputs produce explicit report-facing findings. |
| DEL-08-06-R6 | Numeric values included through result/report references shall carry unit and dimensional metadata or explicit diagnostics from source envelopes. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/SPEC.md` section 9 | Unit/dimension validation tests shall reject unqualified numeric report values unless paired with diagnostics. |
| DEL-08-06-R7 | Report sections shall preserve stable references, checksums, source notes, privacy classification, review state, and provenance for input/report/result payloads where available. | `docs/SPEC.md` sections 4.4 and 9; `_CONTEXT.md` Architecture Basis AB-00-04 | Tests shall compare stable identifiers and checksum/provenance fields across source envelope and rendered section records. |
| DEL-08-06-R8 | Public report sections, templates, fixtures, and examples shall not copy protected standards text, protected tables, protected examples, proprietary formulas, proprietary engineering values, private project data, private rule-pack payloads, private library content, or real secrets. | `docs/CONTRACT.md` OPS-K-IP-1 and OPS-K-REPORT-2; `docs/SPEC.md` section 9; `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 7 | Protected-content/provenance gates shall inspect configured public surfaces; human/legal review remains required when uncertain. |
| DEL-08-06-R9 | Report sections may reference private rule packs, libraries, owner requirements, and project values only by identifier, checksum, source note, privacy class, and review state when public/private boundaries require it. | `docs/SPEC.md` section 9; `docs/IP_AND_DATA_BOUNDARY.md` sections 6 and 7 | Fixture review shall confirm private payload contents are not copied into public artifacts. |
| DEL-08-06-R10 | State/run report sections shall consume immutable model-state and analysis-run records through their stable record identities, hashes, warnings, assumptions, diagnostics, solver version, settings, units, load cases, results, rule/library refs, and result hashes where available. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-071 and SOW-072; `Dependencies.csv` rows DAG-002-E0861 and DAG-002-E0862 | Integration tests shall use approved or invented fixtures with explicit references to state/run source records. |
| DEL-08-06-R11 | Comparison report sections shall preserve deterministic comparison semantics, including stable IDs, manual mappings where required, unit-normalized result deltas, tolerance/profile references, unmatched classifications, diagnostics, and settings where available. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-073 and rows DEL-14-03 through DEL-14-05; `Dependencies.csv` rows DAG-002-E0863 through DAG-002-E0865 | Comparison fixture tests shall verify deterministic ordering and boundary wording; exact threshold policy is TBD. |
| DEL-08-06-R12 | Handoff manifest report sections shall preserve handoff package manifest data, model hash, units manifest, entity IDs, library/rule references, unresolved assumptions, warnings, target mapping metadata, unsupported-target flags, and external-prover boundary metadata where available. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-074 and SOW-075; `Dependencies.csv` rows DAG-002-E0866 through DAG-002-E0868 | Handoff fixture tests shall verify manifest references and unsupported-target/professional-boundary disclosures. |
| DEL-08-06-R13 | Report-section behavior shall remain behind schema-first service boundaries and shall not bypass governance, validation, diagnostics, privacy, protected-content, report, solver, rule, or human-acceptance boundaries. | `_CONTEXT.md` Architecture Basis AB-00-02, AB-00-03, AB-00-06, AB-00-07; `docs/SPEC.md` sections 4.5 and 9 | Architecture or module-boundary review shall verify the section builder depends on source envelopes/contracts, not on bypass paths. |
| DEL-08-06-R14 | Tests shall be layered and include report-relevant protected-content/provenance gates where applicable. | `_CONTEXT.md` Architecture Basis AB-00-08; `docs/CONTRACT.md` enforcement map | Test-plan review shall identify Cargo/validation/protected-content gates or mark unavailable gates as TBD. |

## Standards

| Standard or governing basis | Status for this deliverable | Source |
|---|---|---|
| JSON Schema 2020-12 contracts | Applicable as architecture basis; exact schema fragments for this deliverable are TBD. | `_CONTEXT.md` Architecture Basis; `docs/TYPES.md` schema registry |
| JCS-compatible canonical JSON hash basis | Applicable where JSON payloads are hashed; non-JSON/binary partitioning remains TBD. | `_CONTEXT.md` Architecture Basis AB-00-04; `docs/SPEC.md` section 4.4 |
| Project invariant catalog | Applicable for IP/data, report, authority, unit, and agent constraints. | `docs/CONTRACT.md` |
| Public/private report boundary policy | Applicable for public report sections and examples. | `docs/IP_AND_DATA_BOUNDARY.md` section 7 |
| Protected standards or code-specific engineering standards text | Not accessible and not to be reproduced. Clause-level engineering requirements are TBD unless supplied through authorized user/private sources. | `docs/CONTRACT.md` OPS-K-IP-1; `docs/DIRECTIVE.md` section 4.2 |

## Verification

| Verification topic | Minimum check | Evidence state |
|---|---|---|
| Scope coverage | Confirm state/run, comparison, and handoff report-section artifacts exist or are explicitly represented in tests. | TBD |
| SOW-024 content | Confirm inputs, sources, warnings, assumptions, results, rule-pack checksums, and limitations are represented. | TBD |
| Professional boundary | Confirm generated wording cannot imply professional approval, certification, sealing, authentication, endorsement, code compliance, or reliance. | TBD |
| IP/protected content | Confirm public sections/templates/fixtures are scanned or reviewed for protected/private content risk. | TBD |
| Units and numeric values | Confirm numeric values carry unit/dimension metadata or diagnostics. | TBD |
| Provenance/checksums | Confirm source notes, checksum refs, review state, privacy class, and stable refs survive assembly. | TBD |
| Determinism | Confirm comparison and section ordering are deterministic for identical source envelopes. | TBD |
| Missing data | Confirm missing source values become explicit findings, warnings, limitations, or unresolved TBDs. | TBD |

## Documentation

Expected deliverable-local records and downstream artifacts:

- State/run report sections.
- Comparison report section.
- Handoff manifest report section.
- Tests or review notes proving boundary wording, SOW-024 coverage, deterministic assembly, provenance/checksum preservation, and protected-content avoidance.
- Source notes that identify upstream record contracts consumed by the sections.
- Any unresolved implementation/API/schema decisions recorded as TBD rather than inferred.
