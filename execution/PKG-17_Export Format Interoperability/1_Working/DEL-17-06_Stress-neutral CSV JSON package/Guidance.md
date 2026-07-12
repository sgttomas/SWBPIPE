# Guidance: DEL-17-06 Stress-neutral CSV/JSON package

## Purpose

Use this deliverable to keep the stress-neutral CSV/JSON package narrow and auditable. Its job is to expose result/package evidence in a project-controlled form that humans and downstream tools can inspect, compare, and adapt.

The package must not be described as a solver certification, code-compliance result, professional approval state, formal validation record, or vendor compatibility guarantee.

## Principles

### Keep the package stress-neutral

The package should describe stress-model and result concepts in OpenPipeStress terms, then preserve enough identity, units, diagnostics, provenance, and loss reporting for consumers to understand what was exported. It should not encode target-solver hidden defaults or code-specific acceptance criteria.

### Prefer explicit gaps over silent meaning

A useful stress-neutral export can still be incomplete. Missing columns, omitted entity families, unsupported result classes, unresolved unit mappings, and unsupported comparison semantics should be represented as `TBD`, loss-report entries, diagnostics, or blocking findings rather than guessed defaults.

### Preserve identity before convenience

CSV is useful for review, but row order is not identity. JSON is useful for structured interchange, but object nesting is not enough by itself. Both formats should preserve stable canonical IDs or reference an ID map so rows and objects can be reconciled with source model entities, analysis runs, and comparison records.

### Treat results as evidence

The package may help review, regression comparison, downstream tooling, and adapter development. It does not authenticate an engineering design or establish professional reliance. Any human acceptance reference remains separate and hash-bound outside this package.

If a future workflow needs to point at a human acceptance reference, this package should carry only a neutral external reference such as a model-state, report, analysis-run, or project-governance record identifier plus the reviewed hash basis. The reference target, reviewer authority, and acceptance meaning stay outside the stress-neutral CSV/JSON package, and the package must not convert that reference into a software-generated approval, certification, code-compliance, or professional-reliance status.

## Considerations

| Topic | Guidance |
|---|---|
| CSV tables | Use table-oriented outputs for inspection and spreadsheet workflows, but keep exact table names, columns, ordering, quoting rules, and null handling `TBD` until schema work begins. |
| JSON package | Use JSON for structured import/export and future validation, but do not create a schema in this Phase A task. JSON Schema 2020-12 remains the expected future baseline. |
| Units | Every result value needs explicit unit and dimensional context or a blocking diagnostic. Do not rely on spreadsheet headers alone for unit meaning. |
| Result values | Displacements, rotations, forces, moments, reactions, stress, ratios, and rule-check values are governed by result-export boundaries. Which result families appear in the stress-neutral package is `TBD`. |
| Materials and sections | Include identity/provenance surfaces only unless user-cleared data is present. Do not bundle protected tables, allowables, catalog defaults, SIF/flexibility values, or proprietary values. |
| Load/design cases | Preserve source basis and identity. Do not imply code-specific load combinations unless user/private rule data supplies them with provenance. |
| Comparisons | Use stable IDs and declared tolerance profiles when comparison semantics are later defined. Tolerances and pass/fail language are `TBD` in Phase A. |
| External solver data | CAEPIPE CSV/text parsing can provide regression or handoff evidence under DEL-17-05, but it does not make this package professionally authoritative. |

## Trade-offs

| Choice | Benefit | Risk | Phase A disposition |
|---|---|---|---|
| CSV plus JSON | Serves both spreadsheet review and structured downstream tooling. | Two representations can drift. | Require shared identity, units, manifest, and validation basis; exact synchronization checks `TBD`. |
| Broad table inventory | Makes the package useful across adapters. | Can imply support for semantics not yet implemented. | List candidate table families but keep field-level behavior `TBD`. |
| Loss report on successful exports | Prevents silent omissions and approximations. | Adds package complexity. | Required by DEL-17-02 common contract. |
| Stress-neutral vocabulary | Avoids vendor lock-in. | Consumers may need adapter-specific mapping. | Use profile/manifest/ID map/loss report to keep mappings explicit. |
| Regression comparison support | Supports deterministic review over time. | Users may mistake comparison evidence for professional acceptance. | Comparison remains diagnostic/audit functionality only. |

## Examples

No concrete CSV rows, JSON payloads, proprietary examples, solver outputs, protected standards data, or public engineering fixtures are included in Phase A.

Acceptable future examples should be invented or otherwise rights-cleared and should demonstrate shape, identity, units, diagnostics, loss reporting, and boundary notices without embedding protected or private engineering values.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| none | No source conflict identified in Phase A. | n/a | n/a | n/a | n/a | n/a |
## D-41 R5 T2A canonicalization guidance (2026-07-12)

Do not describe `canonical_json` as JCS. Preserve exact sorted-key compact Python JSON bytes and the precise `deterministic_sorted_compact_json_payload_hash` label unless a later governed change introduces and proves an RFC 8785 implementation.
