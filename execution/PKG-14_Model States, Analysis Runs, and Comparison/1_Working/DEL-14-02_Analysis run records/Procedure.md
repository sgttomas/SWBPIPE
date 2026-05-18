# Procedure: DEL-14-02 Analysis run records

## Purpose

This procedure describes how later Type 2 implementation work should produce and verify the DEL-14-02 analysis-run record artifacts from the current setup-stage sources. It is a production workflow guide, not evidence that implementation has already occurred.

## Prerequisites

| Prerequisite | Status | Source |
|---|---|---|
| Accepted decomposition basis | Available: `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.5 | `_CONTEXT.md` |
| Deliverable identity and scope | Available: DEL-14-02, SOW-072, OBJ-016 | `_CONTEXT.md`; registers |
| Approved dependency evidence | Available as local DAG-002 mirror in `Dependencies.csv` | `_DEPENDENCIES.md`; `Dependencies.csv` |
| Architecture basis constraints | Available in `_CONTEXT.md`; exact package-specific implementation choices remain decision-gated | `_CONTEXT.md` |
| Field-level schema design | TBD | No accessible source defines final field names/cardinality. |
| Implementation code and tests | TBD | PREPARATION notes say no Type 2 implementation artifacts were drafted. |

## Steps

1. Re-read `_CONTEXT.md`, `_REFERENCES.md`, `Dependencies.csv`, and the DEL-14-02 rows in the decomposition and registers.
2. Confirm that the work remains limited to DEL-14-02 and does not require protected standards text, private project data, or sibling deliverable edits.
3. Draft or update `schemas/analysis_run.schema.json` using JSON Schema 2020-12 conventions, subject to repository schema patterns available at implementation time.
4. Represent the SOW-072 binding categories: exact model state, solver version, solver/settings basis, units, load-case basis, diagnostics, results, rule-pack references, library references, and result hashes.
5. Mark unsupported field-level choices as TBD until a source, accepted architecture decision, or human ruling supports them.
6. Ensure unit-bearing values are unit-aware or produce explicit diagnostics for missing/ambiguous units.
7. Ensure hash records identify payload scope and canonicalization basis. Use the accepted JSON/JCS-compatible basis where JSON payloads are hashed.
8. Preserve professional-boundary constraints: do not generate approval, certification, sealing, authentication, or code-compliance labels as software statuses.
9. Preserve protected/private-data constraints: do not embed private formulas, protected standards text, protected tables, proprietary values, or private rule-pack/library payloads in public fixtures or examples.
10. Add run reproducibility tests covering stable serialization/hash behavior, binding to model-state/run basis, unit diagnostics, and professional-boundary status behavior.
11. Record remaining TBDs and assumptions in implementation notes or tests rather than silently defaulting engineering behavior.

## Verification

| Check | Expected result |
|---|---|
| Source traceability | Requirements and tests cite accessible sources or mark unsupported details as TBD/ASSUMPTION. |
| Schema parse | `schemas/analysis_run.schema.json` parses and satisfies repository schema validation once implemented. |
| Binding coverage | Tests or schema review confirm all SOW-072 categories are present or explicitly referenced. |
| Reproducibility | Stable run-record payloads produce stable hashes under the selected canonicalization basis. |
| Unit handling | Missing unit metadata for unit-bearing values produces diagnostics rather than silent defaults. |
| Boundary controls | Statuses and examples avoid automatic professional approval/compliance claims and protected/private data leakage. |
| Dependency preservation | Existing DAG-002 mirror rows remain ACTIVE unless later CHANGE/RECONCILIATION authority changes them. |

## Records

The implementation pass should leave:

- `schemas/analysis_run.schema.json`.
- Run reproducibility tests.
- Validation output for schema and tests.
- Notes for any TBD schema fields, hash partitions, migration behavior, or fixture policies.
- Protected-content/private-data review notes for public examples.
- Any dependency-register changes only if they preserve approved DAG-002 rows or have explicit human/change authority.
