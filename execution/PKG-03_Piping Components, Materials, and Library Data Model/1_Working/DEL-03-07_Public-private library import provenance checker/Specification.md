# Specification: DEL-03-07 Public/private library import provenance checker

## Scope

This deliverable specifies reconciled evidence for a backend feature slice that validates already-parsed public/private component, section, and material library payloads for provenance, license, redistribution status, privacy posture, unit metadata, and protected-content handling.

It covers:

- metadata presence checks for source, provenance, license, contributor/reviewer disposition, and redistribution status;
- public/private data boundary checks for component, section, and material library imports;
- diagnostics for missing provenance, unresolved redistribution metadata, protected-content suspicion, missing unit metadata, and review-required public imports;
- PKG-02-style diagnostic-envelope projection for import findings;
- provenance-focused tests using invented fixture payloads.

It excludes:

- concrete external import formats and parsers, which are TBD;
- legal conclusions about whether a license grants redistribution rights;
- protected standards text, copied tables, vendor data, or derived protected examples;
- global solver, rule evaluator, or GUI implementation work.

## Requirements

| ID | Requirement | Evidence basis | Verification |
|---|---|---|---|
| DEL-03-07-R1 | The checker shall require recorded source, provenance, license, contributor certification, review status, and redistribution-status metadata before a public component/material data import can be accepted. | SOW-019; SOW-044; OPS-K-IP-2; OPS-K-DATA-3 | `test_public_component_import_requires_accepted_public_provenance` and missing-provenance tests cover present/missing metadata fields. |
| DEL-03-07-R2 | The checker shall flag or reject imports with missing provenance or missing redistribution-status metadata without creating silent defaults. | SOW-044; OPS-K-DATA-2; OPS-K-AGENT-1 | Negative tests assert diagnostic output, rejected outcome, and no accepted record. |
| DEL-03-07-R3 | The checker shall separate public and private library handling so private project, material, component, and rule-pack data is not transmitted or committed publicly by default. | OPS-K-PRIV-1; OPS-K-DATA-1 | Tests distinguish public-import disposition from private-local records. |
| DEL-03-07-R4 | The checker shall quarantine or escalate suspected protected content rather than paraphrasing, transforming, or accepting it into public data. | OPS-K-IP-1; OPS-K-IP-3; OPS-K-GOV-4 | Tests use invented markers only and assert quarantine outcome. |
| DEL-03-07-R5 | The checker shall preserve unit metadata for imported numeric component/material values where such values are present. | OPS-K-UNIT-1; AB-00-04; AB-00-07 | Unit-aware fixture tests use invented values and verify missing unit metadata blocks public acceptance. |
| DEL-03-07-R6 | The checker shall produce diagnostics/result envelopes compatible with the schema-first command/query/job boundary. | AB-00-02; AB-00-06; AB-00-07 | `test_import_findings_map_to_pkg02_diagnostic_envelope` asserts stable diagnostic fields and no bypass around import validation. |
| DEL-03-07-R7 | The checker shall not make legal acceptance claims; unresolved license or rights questions shall remain review dispositions requiring human/project authority. | OPS-K-AGENT-1; OPS-K-GOV-4 | Tests assert `TBD`/review-needed style statuses for unresolved rights metadata. |

## Standards

| Standard or governing source | Applicability | Status |
|---|---|---|
| docs/CONTRACT.md | Invariant source for IP, data, unit, privacy, governance, and agent behavior. | Locally accessible |
| execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.7 | Scope and objective source for DEL-03-07. | Locally accessible |
| External import format specifications | Potential parser/import constraints once selected. | TBD |
| Legal license interpretation sources | May govern redistribution-right acceptance. | TBD; human/legal review required |

## Verification

- Run `python3 -m pytest tests/test_library_import_provenance.py` for accepted, private-local, rejected, quarantine, and diagnostic-envelope outcomes.
- Use only invented, minimal fixtures for public tests.
- Confirm accepted public imports contain source, provenance, license, redistribution status, contributor/review disposition, and unit metadata where applicable.
- Confirm missing or uncertain rights/provenance fields yield explicit diagnostics rather than accepted defaults.
- Confirm suspected protected content yields quarantine/escalation status and no public data output.
- Confirm diagnostics do not include protected content excerpts.

## Documentation

Implemented and reconciled artifacts are:

- `core/library_import/provenance_checker.py`;
- `core/library_import/README.md`;
- `tests/test_library_import_provenance.py`;
- active deliverable docs recording implementation evidence and remaining TBDs.

Remaining non-implementation decisions are external import formats, public-source/catalog policy, fixture-value authority for engineering reliance, dependency satisfaction outside this bounded evidence, human disposition of local review findings, and lifecycle closure.
