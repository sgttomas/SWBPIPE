# Procedure: DEL-03-03 Bend and elbow component model fields

## Purpose

Define the setup procedure for preparing and later executing the bend/elbow component model deliverable without crossing the protected-data or professional-authority boundaries.

## Prerequisites

- Use the sealed context for `DEL-03-03`.
- Apply `SOW-007`, `OBJ-004`, and applicable architecture basis IDs `AB-00-01`, `AB-00-02`, `AB-00-04`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.
- Apply contract invariants OPS-K-IP-1, OPS-K-IP-3, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-RULE-1, OPS-K-MECH-1, and OPS-K-AGENT-1..4.
- Treat exact implementation field names, dependency versions, schema filenames, and test harness locations as `TBD` unless resolved by a later authorized implementation task.

## Steps

1. Confirm the implementation task is still scoped to `DEL-03-03` and the bend/elbow component family.
2. Define bend/elbow identity and geometry field concepts using user-entered data only.
3. Define SIF and flexibility factor field concepts as user-entered or lawfully imported private/library data with provenance.
4. Add source/provenance metadata concepts for each data group that may be source-sensitive.
5. Add validation behavior for missing required values, missing provenance, unit mismatch, and protected-content boundary violations.
6. Add serialization/persistence hooks consistent with schema-first, versioned, deterministic, unit-aware project persistence.
7. Create validation tests for geometry, factors, provenance, missing-value diagnostics, unit awareness, and protected-content gates.
8. Keep all protected standards content, tables, copied formulas, and non-public commercial data out of public artifacts.

## Verification

- Four setup documents exist and use `TBD` for unresolved implementation details.
- No protected SIF/flexibility table, formula, standard text, or invented engineering value appears in the setup kit.
- Planned validation tests map to all field groups and boundary constraints.
- Dependency register validates against v3.1 schema.
- `_STATUS.md` is `SEMANTIC_READY` and never `ISSUED` after this setup pass.

## Records

- Four-document kit.
- Semantic matrix lens and semantic lensing register.
- Dependency register and dependency notes.
- Run records for P1/P2, semantic matrix build, lens register, P3, and dependency extraction.
