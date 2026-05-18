---
doc_id: DEP-CLOSURE-SCA002-REV05-COMPATIBILITY
doc_kind: audit.run_summary
status: complete_with_revision05_warnings
created: 2026-05-03
run_label: SCA002_REV05_COMPATIBILITY
scope: SCA-002 revision 0.5 deliverables and historical DEV-001 dependency mirrors
---

# Dependency Closure Run Summary - SCA002_REV05_COMPATIBILITY

## Purpose

Verify the historical DEV-001 local dependency mirrors and record why they are
insufficient as revision `0.5` dispatch authority after SCA-002.

## Inputs

- Accepted deliverable register: `docs/_Registers/Deliverables.csv`
- Historical aggregate graph: `execution/_DAG/DAG-001/DependencyEdges.csv`
- Historical local mirrors: non-`PKG-00` deliverable-local `Dependencies.csv`
- Implementation evidence: `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv`

## Results

The local dependency-closure audit is structurally clean for the historical
revision `0.4` mirrors:

- Local `Dependencies.csv` files found: 65.
- Local dependency rows loaded: 624.
- Schema-valid local registers: 65.
- Schema-invalid local registers: 0.
- Evidence populated: 624 / 624.
- Active execution graph SCCs: 0.
- Bidirectional active pairs: 0.
- ID normalizations: 0.

Revision `0.5` compatibility warnings remain:

- Accepted revision `0.5` has 92 deliverables; historical local mirrors cover
  only the 65 non-`PKG-00` historical implementation deliverables.
- `DAG-001` has 73 nodes and lacks 19 SCA-002 deliverables.
- Package folders are missing for `PKG-13` through `PKG-16`.
- Contexts are missing for all 19 SCA-002 added deliverables.
- All 73 existing contexts reference the retired docs-side decomposition path;
  65 also reference revision `0.4`.
- Historical dispatch briefs are blocked from reuse for revision `0.5`
  implementation.

## Run Status

`RUN_STATUS = WARNINGS`

The existing local dependency mirrors are internally valid historical evidence,
but they are not revision `0.5` sequencing, dispatch, blocker, or lifecycle
authority.
