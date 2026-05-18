# Specification: DEL-03-07 Public/private library import provenance checker

## Scope

This deliverable specifies setup evidence for a backend feature slice that will validate imports of public/private component and material library data for provenance, license, redistribution status, and protected-content handling.

It covers:

- metadata presence checks for source, provenance, license, contributor/reviewer disposition, and redistribution status;
- public/private data boundary checks for component and material library imports;
- diagnostics for missing provenance or redistribution metadata;
- provenance-focused tests using invented or placeholder data.

It excludes:

- concrete external import formats, which are TBD;
- legal conclusions about whether a license grants redistribution rights;
- protected standards text, copied tables, vendor data, or derived protected examples;
- global solver, rule evaluator, or GUI implementation work.

## Requirements

| ID | Requirement | Evidence basis | Verification |
|---|---|---|---|
| DEL-03-07-R1 | The checker shall require recorded source, provenance, license, and redistribution-status metadata before a public component/material data import can be accepted. | SOW-019; SOW-044; OPS-K-IP-2; OPS-K-DATA-3 | Provenance tests cover present/missing metadata fields. |
| DEL-03-07-R2 | The checker shall flag or reject imports with missing provenance or missing redistribution-status metadata without creating silent defaults. | SOW-044; OPS-K-DATA-2; OPS-K-AGENT-1 | Negative tests assert diagnostic output and no accepted record. |
| DEL-03-07-R3 | The checker shall separate public and private library handling so private project, material, component, and rule-pack data is not transmitted or committed publicly by default. | OPS-K-PRIV-1; OPS-K-DATA-1 | Tests distinguish public-import disposition from private-local records. |
| DEL-03-07-R4 | The checker shall quarantine or escalate suspected protected content rather than paraphrasing, transforming, or accepting it into public data. | OPS-K-IP-1; OPS-K-IP-3; OPS-K-GOV-4 | Tests use invented markers only and assert stop/escalation status. |
| DEL-03-07-R5 | The checker shall preserve unit metadata for imported numeric component/material values where such values are present. | OPS-K-UNIT-1; AB-00-04; AB-00-07 | Unit-aware fixture tests use invented values and verify unit fields are not dropped. |
| DEL-03-07-R6 | The checker shall produce diagnostics/result envelopes compatible with the schema-first command/query/job boundary. | AB-00-02; AB-00-06; AB-00-07 | Contract tests assert stable diagnostic fields and no bypass around import validation. |
| DEL-03-07-R7 | The checker shall not make legal acceptance claims; unresolved license or rights questions shall remain review dispositions requiring human/project authority. | OPS-K-AGENT-1; OPS-K-GOV-4 | Tests assert `TBD`/review-needed style statuses for unresolved rights metadata. |

## Standards

| Standard or governing source | Applicability | Status |
|---|---|---|
| docs/CONTRACT.md | Invariant source for IP, data, unit, privacy, governance, and agent behavior. | Locally accessible |
| docs/_Decomposition/SOFTWARE_DECOMP.md revision 0.4 | Scope and objective source for DEL-03-07. | Locally accessible |
| External import format specifications | Potential future input format constraints. | TBD |
| Legal license interpretation sources | May govern redistribution-right acceptance. | TBD; human/legal review required |

## Verification

- Run schema/contract tests for all accepted and rejected import outcomes.
- Use only invented, minimal fixtures for public tests.
- Confirm accepted public imports contain source, provenance, license, redistribution status, contributor/review disposition, and unit metadata where applicable.
- Confirm missing or uncertain rights/provenance fields yield explicit diagnostics rather than accepted defaults.
- Confirm suspected protected content yields quarantine/escalation status and no public data output.
- Confirm diagnostics do not include protected content excerpts.

## Documentation

Required future artifacts are:

- library import validator;
- provenance tests;
- fixture notes documenting that all public fixtures are invented or placeholder data;
- review notes identifying any human/legal rulings still needed for license and redistribution disposition.
