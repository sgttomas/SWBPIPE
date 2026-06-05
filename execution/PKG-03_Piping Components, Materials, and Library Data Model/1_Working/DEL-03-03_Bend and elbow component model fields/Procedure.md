# Procedure: DEL-03-03 Bend and elbow component model fields

## Purpose

Define the evidence-reconciliation procedure for the bend/elbow component model deliverable without crossing the protected-data or professional-authority boundaries.

## Prerequisites

- Use the sealed context for `DEL-03-03`.
- Apply `SOW-007`, `OBJ-004`, and applicable architecture basis IDs `AB-00-01`, `AB-00-02`, `AB-00-04`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.
- Apply contract invariants OPS-K-IP-1, OPS-K-IP-3, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-RULE-1, OPS-K-MECH-1, and OPS-K-AGENT-1..4.
- Treat dependency satisfaction, lifecycle transitions, review dispositions, downstream solver behavior, and downstream adapter/GUI behavior as unresolved unless authorized by their owning workflow or a later human gate.

## Steps

1. Confirm the assessment task is still scoped to `DEL-03-03` and the bend/elbow component family.
2. Read the active component schema, model schema, strict component fixture, combined fixture pointer, tests, review findings, status, memory, and relevant run records.
3. Reconcile current evidence against the three local review findings without editing `Review_Findings.csv`.
4. Refresh only active DEL-03-03 documentation when stale setup/future language conflicts with implemented schema/fixture/test evidence.
5. Run focused validation for the component/section schema contract.
6. Record the readiness verdict in `MEMORY.md` and a local run record.
7. Keep all protected standards content, tables, copied formulas, and non-public commercial data out of public artifacts.

## Verification

- Active documents distinguish implemented DEL-03-03 evidence from unresolved dependency, lifecycle, and downstream behavior.
- No protected SIF/flexibility table, formula, standard text, or invented engineering value appears in the deliverable kit.
- Focused validation confirms schema/fixture/test evidence for bend/elbow identity, field slots, provenance, and diagnostics.
- `_STATUS.md` and `Review_Findings.csv` are not changed by this TASK.

## Records

- Active four-document kit.
- Review findings and review notes.
- Focused validation output.
- Local run record for this evidence-reconciliation pass.
