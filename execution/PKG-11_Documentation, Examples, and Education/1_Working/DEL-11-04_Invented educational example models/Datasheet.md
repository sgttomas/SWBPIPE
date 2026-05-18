# Datasheet: DEL-11-04 Invented educational example models

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-11-04 |
| Deliverable name | Invented educational example models |
| Package ID | PKG-11 |
| Package name | Documentation, Examples, and Education |
| Deliverable type | DOC_UPDATE |
| Scope item | SOW-033 |
| Supported objectives | OBJ-001, OBJ-008 |
| Context envelope | M |
| Current production mode | Setup/document production only |

## Attributes

| Attribute | Source-grounded value |
|---|---|
| Primary purpose | Define the setup boundary for future invented-data educational examples used for mechanics-only demonstrations and fake-rule-pack demonstrations. |
| Public-data posture | Future public examples must be original or invented and must not reproduce protected standards text, tables, formulas, examples, allowables, SIF or flexibility data, protected dimensional tables, commercial-software examples, or vendor proprietary data. |
| Example families | Mechanics-only demonstration examples; fake-rule-pack demonstration examples. |
| Materialization status | This setup run does not create actual example model files under `examples/models/invented/*`, tutorials outside this deliverable, source code, schemas, or issued artifacts. |
| Rule-pack boundary | Fake rule-pack examples are fictional non-code teaching devices and must not resemble or approximate real code allowables, formulas, load combinations, or acceptance rules. |
| Mechanics boundary | Mechanics-only examples may illustrate open centerline mechanics concepts and reproducibility expectations, but they must not be presented as design bases or compliance checks. |
| Unit/provenance expectation | Future example data must be unit-aware and must carry provenance such as `PUBLIC_DOMAIN_OR_ORIGINAL` or a stronger documented redistribution basis. |
| Professional boundary | Example outputs are educational and test-support artifacts only; professional reliance requires competent human review outside the software. |

## Conditions

| Condition | Handling |
|---|---|
| A future example needs code-specific data | Leave the value `TBD` or require user/private input; do not invent a realistic code value. |
| A future example would benefit from a real standard example | Stop and route to human/legal review; do not copy or paraphrase protected examples into public artifacts. |
| A fake rule pack needs pass/fail behavior | Use clearly fictional labels and non-engineering placeholder values; avoid code-style formulas and realistic allowables. |
| A mechanics-only example needs numeric quantities | Use original toy quantities with unit labels and provenance; document that they are not engineering recommendations. |
| A tutorial wants to compare against commercial software | Exclude the comparison from public examples unless a separate lawful and approved basis exists. |
| A future example is used in validation or regression | Record the distinction between mechanics verification and code compliance; do not imply professional approval. |

## Construction

This setup artifact defines constraints, acceptance checks, and dependency signals for future invented educational examples. It intentionally does not create external example model files, tutorials outside this deliverable, code, schemas, private data, or `ISSUED` artifacts.

Future example artifacts should include:

- a clear non-engineering notice;
- a source/provenance field for every invented data set;
- a mechanics-only or fake-rule-pack classification;
- a unit basis for every quantity;
- a protected-content review record before public use;
- a statement that user rule checks and professional approval remain separate from mechanics solve results.

## References

| Source | Used for |
|---|---|
| `INIT.md` | Bootstrap boundaries for open mechanics, user rule checks, and professional responsibility. |
| `AGENTS.md` | TASK dispatch boundary and sealed deliverable discipline. |
| `docs/CONTRACT.md` | Invariants for protected data, user-supplied rule data, invented public examples, units, agents, and professional authority. |
| `docs/DIRECTIVE.md` | Product principles for open mechanics, private code data, no silent defaults, validation before reliance, and stop rules. |
| `docs/TYPES.md` | Provenance labels, analysis-status vocabulary, and data-boundary terms. |
| `docs/SPEC.md` | Repository target for invented examples, rule-pack boundaries, reporting notices, and V&V expectations. |
| `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` | Type 2 execution rules and review expectations. |
| `docs/_Decomposition/SOFTWARE_DECOMP.md` | PKG-11 and DEL-11-04 decomposition context. |
| `docs/_Registers/Deliverables.csv` | Machine-readable DEL-11-04 row. |
| `docs/_Registers/ScopeLedger.csv` | SOW-033 scope mapping. |
