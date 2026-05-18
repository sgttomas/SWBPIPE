# Datasheet: DEL-06-01 Rule-pack schema

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-06-01 |
| Package ID | PKG-06 |
| Package | Rule Packs and User-Supplied Code Check Engine |
| Type | DATA_MODEL_CHANGE |
| Scope items | SOW-016; SOW-042 |
| Objective | OBJ-005 |
| Anticipated artifacts | `schemas/rule_pack.schema.yaml`; `docs/SPEC.md` update |
| Current evidence status | Setup evidence only; no product implementation |

## Attributes

| Attribute | Required setup meaning | Source |
|---|---|---|
| Rule-pack artifact scope | Private user-defined stress checks, allowables, formulas, required inputs, and pass/fail criteria. | ScopeLedger rows SOW-016, SOW-042 |
| Schema baseline | Future public schema/interchange work must align with JSON Schema 2020-12. | ContextBudgetQA row DEL-06-01; AB-00-04 |
| Versioning | Rule packs must carry stable identity and version metadata. | SOW-042; OPS-K-RULE-3 |
| Checksum basis | JSON rule-pack payloads use canonical JSON with JCS-compatible hashing where checksums are computed. | AB-00-04; SOW-039 |
| Provenance and source notes | Rule-pack values, formulas, and source basis records must carry source/provenance metadata. | OPS-K-IP-2; OPS-K-DATA-3; OPS-K-RULE-3 |
| Redistribution status | Rule packs must be explicitly marked public/private and redistribution status must be recorded. | SOW-042; OPS-K-RULE-3 |
| Private-data boundary | Private rule packs are user-owned data and should not be committed or transmitted publicly by default. | OPS-K-PRIV-1; SOW-042 notes |
| Protected-content boundary | Public artifacts must not reproduce protected standards text, tables, examples, copied formulas, or proprietary allowables. | OPS-K-IP-1; OPS-K-IP-3; OPS-K-RULE-1 |
| Unit handling | Rule-pack inputs, formulas, and allowables must be unit-aware and dimensionally checked by later implementation work. | OPS-K-UNIT-1 |
| Missing data behavior | Missing rule-check-required values become explicit findings, not silent defaults. | OPS-K-DATA-2 |
| Evaluator boundary | Formula representation must remain declarative and sandbox-compatible; exact expression grammar/library remains `TBD`. | SOW-045 note; OI-006 |
| Professional boundary | A rule-pack pass/fail result is not professional certification, sealing, or engineering approval. | OPS-K-AUTH-1; OPS-K-MECH-2 |

## Conditions

- This setup pass does not create or edit `schemas/rule_pack.schema.yaml`, `docs/SPEC.md`, examples, evaluator code, registry code, or public/private storage controls.
- No protected standards text, protected formulas, proprietary commercial rules, code tables, material allowables, SIF/flexibility tables, or project-specific engineering values are included.
- Any future public example rule pack must use invented non-code values and carry an explicit non-engineering notice.
- Private rule packs are user-supplied artifacts. Public repository content may define schema slots and validation behavior, but not the user's protected design basis.
- Exact expression grammar/library, public API transport, physical project package/container, private rule-pack encryption defaults, and detailed storage controls remain `TBD` unless later ruled by the human project authority.

## Construction

The future rule-pack schema should be evaluated for these descriptive record groups:

| Record group | Fields or slots to resolve later |
|---|---|
| Identity | Rule-pack ID, display name, namespace, schema version, rule-pack version, lifecycle status, author/contributor role, and generated/modified timestamps. Exact naming rules are `TBD`. |
| Provenance | Source type, source note, citation pointer, contributor certification pointer, review disposition, import path, and source hash where available. |
| Redistribution | Public/private marker, redistribution status, license/reference pointer, quarantine marker, and public-example safety marker. |
| Checksum | Canonical payload hash algorithm, hash value, hash scope, manifest reference for non-JSON assets, and invalidation behavior on content change. |
| Required inputs | Input IDs, dimensional categories, unit constraints, source requirements, missing-value behavior, and applicability conditions. |
| Variables/results | Bindings to solver result fields, load-case or stress-result categories, units, and unavailable-result diagnostics. |
| Formula declarations | Declarative expression slots, variable bindings, dimensional expectations, evaluation status, and sandbox compatibility flags. Actual protected formulas are excluded. |
| Allowable slots | User-supplied allowable placeholders with units, provenance, source note, redistribution status, and completeness status. Actual allowable values are excluded from setup evidence. |
| Checks and criteria | Check IDs, applicability conditions, formula references, limit references, comparison operator categories, pass/fail/incomplete statuses, and diagnostic outputs. |
| Diagnostics | Structured findings compatible with AB-00-06 for missing inputs, unit mismatch, provenance warning, redistribution warning, protected-content warning, evaluator error, and rule-check blocking. |
| Professional boundary | Reportable status wording that distinguishes mechanics solved, user-rule checked, and human professional approval. |

## References

- `docs/_Registers/Deliverables.csv` row DEL-06-01
- `docs/_Registers/ScopeLedger.csv` rows SOW-016 and SOW-042
- `docs/_Registers/ContextBudgetQA.csv` row DEL-06-01
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4
- `docs/CONTRACT.md` invariants OPS-K-IP-1, OPS-K-IP-2, OPS-K-IP-3, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-RULE-1, OPS-K-RULE-3, OPS-K-UNIT-1, OPS-K-AUTH-1, OPS-K-MECH-2, OPS-K-PRIV-1, OPS-K-AGENT-1..4

