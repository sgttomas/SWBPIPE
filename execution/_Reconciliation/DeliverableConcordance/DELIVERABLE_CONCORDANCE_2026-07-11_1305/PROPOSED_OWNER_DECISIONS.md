# Proposed Owner Decision Findings — R3 Evidence Only

> **Epistemic status:** agent-authored, non-operative R3 synthesis from the
> corrected claim ledgers and all 18 package fan-in records at frozen source
> SHA `551f84ef6be656f1603ce0acfa5e3935aa9683c7`. These are candidate findings
> and routes, not owner/product/review rulings, decision options, work
> selections, repair authorizations, or R4 packets.

## OD-001 — Rotational-deformation visualization home remains unknown

- Finding: the implementation emits rotational rows, but the dated residual
  does not resolve whether visualization is deferred DEL-07-05 scope or
  intentionally outside it.
- Evidence: CU-001; `DEL-07-05-REM-002` (`UNKNOWN`).
- Authority route: OWNER.
- Smallest next action: identify the scope/home only; preserve UNKNOWN until a
  record resolves it.

## OD-002 — Property-inspector architecture home

- Finding: DEL-00-05 owns GUI state/interaction architecture while DEL-07-02
  implements model-tree/property-inspector behavior; no accepted delegation
  record resolves the architecture home.
- Evidence: CU-002; `DEL-00-05-REQ-004`, `DEL-07-02-REQ-002`.
- Authority route: OWNER.
- Smallest next action: record one owner or an explicit delegation without
  moving implementation in this run.

## OD-003 — Eight R1 surfaces lack deliverable attribution

- Finding: build readiness, export-unit disclosure, telemetry panel,
  product-preview module, root workspace manifest, and three tool surfaces
  have `DeliverableAttribution=NONE_FOUND` and now carry run-level unmapped
  claim IDs.
- Evidence: CU-003; `UNMAPPED-SURF-011`, `UNMAPPED-SURF-021`,
  `UNMAPPED-SURF-050`, `UNMAPPED-SURF-104`, `UNMAPPED-SURF-170`,
  `UNMAPPED-SURF-211`, `UNMAPPED-SURF-212`, `UNMAPPED-SURF-213` in
  `CLAIM_CONCORDANCE.csv`; corresponding `IMPLEMENTATION_SURFACES.csv`
  SURF-011, SURF-021, SURF-050, SURF-104, SURF-170, SURF-211, SURF-212,
  SURF-213.
- Authority route: OWNER; tooling/reconciliation surfaces may remain governed
  infrastructure if that is explicitly recorded.
- Smallest next action: classify each surface without creating a repair or
  decomposition change here.

## OD-004 — ISSUED baseline refresh path

- Finding: DEL-01-01 is unambiguously ISSUED while three kit surfaces carry
  prior lifecycle/current-authority text. This is surface drift, not lifecycle
  uncertainty.
- Evidence: CU-004; `DEL-01-01-DECL-001`, `DEL-01-01-DECL-002`,
  `DEL-01-01-DECL-004`.
- Authority route: SCOPE_CHANGE/change path.
- Smallest next action: preserve the ISSUED baseline until a separately
  authorized change path chooses whether to refresh it.

## OD-005 — Active-kit currentness and setup-era repair routing

- Finding: rev-0.7/DAG-006 pointers, overtaken TBDs, and no-implementation/
  future-only declarations recur across active kits. They are one currentness
  family with document-specific facts, not hundreds of product gaps.
- Evidence: CU-005; representative `DEL-00-04-DECL-002`,
  `DEL-01-03-DECL-002`, `DEL-08-02-DECL-001`, `DEL-13-04-DECL-001`, and
  `DEL-17-08-DECL-001`; all package summaries.
- Authority route: OWNER for calibration/repair policy; R5 only after a ruling.
- Smallest next action: state the active-kit refresh rule while preserving
  dated MEMORY/run history and the separate ISSUED path.

## OD-006 — Repeated pending review findings need one disposition each

- Finding: technically addressed or nonblocking RF/PKG-02 findings recur in
  consumers while `HumanDisposition` remains TBD or the finding is unhomed.
  Technical remediation does not close formal review state.
- Evidence: CU-006; representative `DEL-09-03-REM-001`,
  `DEL-12-03-REM-001`, `DEL-13-02-REM-001`, `DEL-14-03-REM-001`,
  `DEL-15-02-REM-001`, `DEL-16-01-REM-001`, and `DEL-17-05-REM-001`.
- Authority route: owning human REVIEW/OWNER workflow.
- Smallest next action: disposition and home the underlying finding once, then
  update consumer evidence only through the authorized later phase.

## OD-007 — Privacy/redaction defaults and authoritative runtime seams

- Finding: contract metadata and invented fixtures do not decide private-by-
  default behavior for every user payload or identify every runtime seam that
  must enforce redaction/protected-content controls.
- Evidence: CU-011; `DEL-12-02-REQ-012`, `DEL-14-01-REQ-009`,
  `DEL-15-02-REQ-011`, `DEL-17-05-REQ-014`.
- Authority route: OWNER/PRODUCT for defaults and seam ownership; engineering
  evaluation follows only after that boundary is set.
- Smallest next action: name the defaults and owned seams without claiming
  security, privacy, or legal sufficiency.

## OD-008 — Runtime schema version policy

- Finding: accepted 0.2.0 documents meet stale 0.1.0 UI literals, producing
  `unsupported_schema_review_required` warnings in several consumers.
- Evidence: CU-013; `DEL-02-05-REQ-006` and its exact frozen path/line
  pointers.
- Authority route: OWNER/PRODUCT.
- Smallest next action: confirm the accepted runtime compatibility policy and
  treat the literals as one bounded candidate, not five independent issues.

## OD-009 — Overtaken evidence/review records

- Finding: the 822-reference runner witness and the claimed-absent validator
  paths are stale evidence records; current re-executed behavior separately
  passes.
- Evidence: CU-014; `DEL-10-05-REM-003`, `DEL-17-02-REM-001`.
- Authority route: OWNER/REVIEW.
- Smallest next action: disposition or retire only the overtaken records while
  preserving historical provenance and current evidence.

## OD-010 — Optional live CAEPIPE profile and evidence boundary

- Finding: the bounded parser/run-record package has no user-owned live
  CAEPIPE execution; invocation profile, target version, first MBF profile,
  and source-confirmed CSV coverage remain unresolved.
- Evidence: CU-015; `DEL-17-05-ACC-006`, `DEL-17-05-REM-002`.
- Authority route: OWNER/PRODUCT before any optional live execution;
  engineering or vendor evidence cannot be inferred from fixture tests.
- Smallest next action: resolve the user-owned profile/responsibility gates
  while retaining regression/handoff-only classification.

## OD-011 — Format/profile scope before additional validation

- Finding: PCF and glTF/GLB work is deliberately conservative. Current JSON
  glTF and subset exporters do not imply binary GLB, broad entity coverage,
  viewer compatibility, or comprehensive target support.
- Evidence: CU-016; `DEL-17-08-ACC-011`, `DEL-17-08-REQ-009`,
  `DEL-17-08-REQ-015`, `DEL-17-08-REQ-026`; `PACKAGE_SUMMARIES/PKG-17.md`
  risks 5 and 7.
- Authority route: OWNER/PRODUCT chooses scope; ENGINEERING validates only the
  selected scope.
- Smallest next action: preserve current subset declarations until a new scope
  is explicitly selected.

## OD-012 — External source and legal-review homes

- Finding: public-source spot checks and human-deferred source scopes are not
  redistribution/legal clearance or professional approval.
- Evidence: CU-017; `DEL-11-03-REM-001`, `DEL-01-02-REM-001`.
- Authority route: OWNER/REVIEW and counsel where the governing record names
  it.
- Smallest next action: home the bounded review scopes and obtain their named
  human disposition without broadening agent source checks.

## OD-013 — Application-service/interface homes

- Finding: runner, constraint, operation, and export helpers/panels exist at
  adjacent grains, but several canonical application-service/API homes and
  schema-validation seams remain incomplete.
- Evidence: CU-020; `DEL-10-05-REQ-001`, `DEL-13-03-REQ-006`,
  `DEL-16-04-REQ-009`, `DEL-17-08-ACC-008`.
- Authority route: OWNER/PRODUCT.
- Smallest next action: assign the interface homes before interpreting adjacent
  surfaces as end-to-end integration.

## Ownership checks that found no conflict

- CU-018: retain DEL-16-01 as the model-operation contract owner;
  DEL-16-02..04 are validation/audit/rationale/application consumers.
- CU-019: retain DEL-17-02 as common export-contract owner and DEL-17-01 as
  source-basis owner; DEL-17-03..09 own target-local implementations.

These are current reconciliation findings, not perpetual ownership rulings.

## R3/R4 fence

This file stops at evidence-backed candidate findings, routing, and smallest
next checks. It does not draft alternatives, recommend or record a ruling,
authorize work, change scope/dependencies/registers, or perform R4/R5.
