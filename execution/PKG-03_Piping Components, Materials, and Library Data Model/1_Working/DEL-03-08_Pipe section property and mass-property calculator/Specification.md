# Specification: DEL-03-08 Pipe section property and mass-property calculator

## Scope

This deliverable specifies the bounded backend feature slice that calculates pipe section and mass properties from user-entered dimensions and material data with unit checks. It also records the private-library and provenance constraints that apply when those inputs are sourced from pipe section, component, or material libraries.

Current implementation evidence is `core/section_properties/calculator.py`, `core/section_properties/README.md`, and `tests/test_section_properties.py`. This evidence reconciliation does not edit code, repo-level schemas, fixtures, tests, lifecycle records, dependency registers, review dispositions, or DAG/coordination files, and it does not introduce protected dimensional, material, contents, insulation, corrosion, code, or vendor data.

## Requirements

| Req ID | Requirement | Source basis | Verification hook |
|---|---|---|---|
| DEL-03-08-RQ-001 | The calculator shall accept only explicit user-entered or lawfully imported private/project input values for dimensions, material density, contents, insulation, and corrosion basis. | SOW-051; OPS-K-DATA-1; OPS-K-IP-1 | `tests/test_section_properties.py` uses explicit invented inputs; public source catalog and fixture-value policy remain `TBD`. |
| DEL-03-08-RQ-002 | The calculator shall reject or flag missing solve-required values instead of applying silent defaults. | OPS-K-DATA-2; SOW-051 | `tests/test_section_properties.py` covers missing wall, missing provenance, mixed units, and invalid geometry. Optional mass-contributor requiredness policy remains `TBD`. |
| DEL-03-08-RQ-003 | All input quantities, intermediate calculations, and outputs shall be unit-aware and dimensionally checked. | SOW-051; OPS-K-UNIT-1; OBJ-012 | Tests assert canonical output dimensions and rejection of mixed units. Approved unit conversion constants and dependency satisfaction remain `TBD`. |
| DEL-03-08-RQ-004 | Library-sourced inputs shall carry provenance and redistribution status through calculator schema hooks. | SOW-018; OPS-K-DATA-3 | `Quantity` and `quantity_from_mapping` require provenance. Accepted schema field placement and private-library record linkage remain `TBD`. |
| DEL-03-08-RQ-005 | Public repository fixtures shall not encode protected pipe dimensional tables, protected material data, proprietary commercial data, or paraphrased protected tables. | OPS-K-IP-1; OPS-K-IP-3 | Protected-content review and fixture review. |
| DEL-03-08-RQ-006 | Calculator outputs intended for solver consumption shall remain code-neutral and shall not claim certification, code compliance, or professional approval. | OPS-K-AGENT-4; AB-00-06 | Review of diagnostics/result envelopes and report-facing text. |
| DEL-03-08-RQ-007 | The calculator shall be isolated from global solver implementation and rule-pack compliance logic. | PKG-03 exclusion; OPS-K-SOLVER-1 | Module boundary review confirms `core/section_properties` is outside global solver and rule-pack logic; downstream solver integration policy remains `TBD`. |

## Standards

No protected standard text or table is available in this deliverable-local context. Any standard, code, or vendor basis must be introduced only as a licensed/private input or as a non-protected pointer with provenance. Clause-level requirements are `TBD`.

## Verification

| Verification area | Current evidence and remaining gate |
|---|---|
| Unit safety | Tests demonstrate canonical dimensions and mixed-unit rejection without hidden conversion. Approved conversion support remains `TBD`. |
| Missing input behavior | Tests demonstrate explicit blocking findings for missing required wall thickness and missing provenance; optional contributor policy remains `TBD`. |
| Provenance | Tests demonstrate provenance requirements at quantity construction/mapping boundaries and derived-output provenance. Library record linkage remains `TBD`. |
| IP/data boundary | Tests use invented synthetic values; formal source catalog and fixture-value policy remain `TBD`. |
| Solver boundary | Calculator outputs remain data/service outputs, not solver certification or code-compliance claims. Downstream integration remains `TBD`. |

## Documentation

Current implementation artifacts are:

- `core/section_properties/calculator.py`;
- `core/section_properties/README.md`;
- `tests/test_section_properties.py`;
- schema-like quantity mapping through `quantity_from_mapping`.

Accepted schema ownership, dependency satisfaction, downstream result-envelope mapping, lifecycle disposition, and human disposition for review findings remain `TBD`.

## Conflict Table (for human ruling)

| Conflict ID | Issue | Contenders | Human ruling |
|---|---|---|---|
| PKG03-DEL-03-08-PKG02-001 | Dimension vocabulary compatibility finding has technical evidence in calculator/tests but awaits human disposition. | `Review_Findings.csv`; `tests/test_section_properties.py` | `TBD` |
| PKG03-DEL-03-08-PKG02-002 | Input provenance finding has technical evidence in calculator/tests but awaits human disposition. | `Review_Findings.csv`; `tests/test_section_properties.py` | `TBD` |
| PKG03-DEL-03-08-PKG02-003 | Diagnostic envelope field finding has technical evidence in calculator/tests but awaits human disposition. | `Review_Findings.csv`; `tests/test_section_properties.py` | `TBD` |
