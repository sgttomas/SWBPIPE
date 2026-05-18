# Procedure: DEL-07-08 Design-authoring state and comparison workspace

## Purpose

This procedure describes how to produce and verify the DEL-07-08 GUI workspace deliverable in a later implementation pass. It is based on local decomposition, governing references, and the approved DAG-002 dependency mirror. It does not create product code.

## Prerequisites

| Prerequisite | Evidence |
|---|---|
| Accepted deliverable scope and objectives | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` / DEL-07-08 row, SOW-076, OBJ-015, OBJ-016 |
| Approved dependency basis | `_DEPENDENCIES.md` and `Dependencies.csv`; `execution/_DAG/DAG-002/APPROVAL_RECORD.md` / Approval Conditions |
| Architecture-basis constraints | `_CONTEXT.md` / Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` / AB-00-01 through AB-00-08 as applicable |
| GUI warning and diagnostic vocabulary | `docs/SPEC.md` / GUI requirements; `execution/_Decomposition/SOFTWARE_DECOMP.md` / AB-00-06 |
| Data and professional boundaries | `docs/CONTRACT.md`; `docs/IP_AND_DATA_BOUNDARY.md` |
| Upstream contracts for implementation | DEL-07-01, DEL-07-02, DEL-07-04, DEL-07-05, DEL-13-01, DEL-13-03, DEL-13-04, DEL-14-01, DEL-14-03, DEL-14-04, DEL-14-05, DEL-16-01, DEL-16-02, and DEL-16-03 as listed in `Dependencies.csv`. |

## Steps

1. Confirm the implementation brief is still bounded to SOW-076 and DEL-07-08. If the work expands beyond design knowledge panels, warnings, operation/diff review, state/run browsing, comparison tables, and graphical overlays, escalate for scope split before implementation.
2. Inventory available upstream contracts from the active dependency set. Do not substitute invented schemas, fake operation states, or placeholder comparison semantics for missing upstream contracts; mark unsupported items as TBD.
3. Define the workspace information architecture around the SOW-076 surfaces:
   - design knowledge panel;
   - constraint/warning panel;
   - operation/diff review;
   - state/run browser;
   - comparison table;
   - graphical comparison overlay.
4. Route any GUI-originated model changes through application-service command intents or upstream structured model operation contracts. Do not directly mutate durable project payloads from the GUI layer.
5. Preserve diagnostic structure in warning UI. Where diagnostics are supplied, keep code, class, severity, source, affected object, message, remediation, and provenance available to the user.
6. Implement state/run browsing only against upstream model-state and analysis-run records. Exact query shape, routing, pagination, and storage access are TBD pending implementation context.
7. Implement comparison tables and overlays only against upstream comparison records, mapping/tolerance contracts, or deterministic diff output. Exact visual encodings and tolerance display rules are TBD.
8. Review all labels, empty states, and status text for professional-boundary risk. The GUI must not claim automatic certification, sealing, approval, authentication, or code compliance.
9. Add tests appropriate to the implemented surface. Expected classes include component/unit tests, interaction tests, and Playwright/rendering checks where a frontend scaffold exists. Exact commands and thresholds are TBD.
10. Record remaining TBDs, assumptions, missing upstream contracts, and warning/provenance limitations in the deliverable review notes for human evaluation.

## Verification

| Check | Expected result |
|---|---|
| Scope check | Implemented surfaces map to SOW-076 and do not absorb unrelated package work. |
| Dependency check | GUI semantics are backed by approved upstream contracts or explicitly marked TBD. |
| Mutation-boundary check | Durable project state changes pass through application-service commands or structured operation contracts. |
| Warning check | Missing data, provenance, assumptions, nonlinear uncertainty, and IP-boundary states remain visible. |
| Comparison-boundary check | Comparison output is review/audit support only and does not imply professional validation. |
| Public/private data check | No protected standards text, owner standards, private rule-pack payloads, proprietary values, or real secrets are embedded in public artifacts. |
| Test check | GUI tests run where implementation exists; missing harness details are recorded as TBD. |

## Records

Maintain or produce the following records as implementation evidence:

- source-grounded requirements or acceptance notes for SOW-076;
- UI test output, including interaction and rendering checks where applicable;
- dependency notes showing which upstream contracts were consumed;
- screenshots or visual review artifacts if created by the implementation brief;
- unresolved TBD and ASSUMPTION list;
- professional-boundary and protected-content review notes.

No implementation records exist in this setup workflow.
