# Specification: DEL-15-04 External prover boundary metadata

## Scope

This deliverable covers the data-model boundary for external-prover workflow metadata in PKG-15. It is limited to flexible metadata and validation expectations for names, tags, notes, external references, attachments, and related handoff/comparison context.

This deliverable excludes hard-coded professional approval, certification, code-compliance, sealing, authentication, formal prover-status lifecycle, automatic professional acceptance records, and comprehensive commercial stress-software result ingestion for the MVP. Source: `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-15-04; `docs/_Registers/ScopeLedger.csv` row SOW-075; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEC-015 and DEC-016.

## Requirements

| ID | Requirement | Source | Verification |
|---|---|---|---|
| DEL-15-04-R1 | The data model shall support external-prover workflow metadata. | SOW-075 in `docs/_Registers/ScopeLedger.csv`; `_CONTEXT.md` | Boundary validation tests confirm the metadata surface exists. |
| DEL-15-04-R2 | The metadata surface shall support flexible names, tags, notes, external references, and attachments. | `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-15-04 row; SOW-075 note | Schema/review tests check that these categories are representable without requiring a fixed prover lifecycle. |
| DEL-15-04-R3 | The metadata model shall not force a formal prover-status lifecycle. | SOW-075 in `docs/_Registers/ScopeLedger.csv`; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEC-016 | Negative tests reject or flag hard-coded lifecycle authority when represented as automatic software status. |
| DEL-15-04-R4 | The metadata model shall not create automatic professional acceptance records. | SOW-075; `docs/TYPES.md` section 4; `docs/SPEC.md` section 4.4 | Tests verify software-generated metadata cannot assert human acceptance. |
| DEL-15-04-R5 | The metadata model shall not emit approval, certification, code-compliance, sealing, authentication, endorsement, or professional-reliance equivalents as automatic statuses. | `INIT.md` Agent rule; `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/TYPES.md` section 4; `docs/SPEC.md` analysis-status boundary | Boundary validation tests cover prohibited status labels and equivalents. |
| DEL-15-04-R6 | External prover metadata shall remain diagnostic/handoff support rather than proof of external verification sufficiency. | `execution/_DAG/DAG-002/DAG-002_EdgeDispositionReview.md` DAG2-RD-010; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEC-015 | Review/test evidence checks wording and status semantics. |
| DEL-15-04-R7 | Any external human acceptance reference, if later represented, shall be external, human-actor-owned, and bound to reviewed payload hashes. | `docs/TYPES.md` section 4; `docs/SPEC.md` section 4.4 and analysis-status boundary | Tests require external/hash-bound representation and prevent content-change survival without re-review. |
| DEL-15-04-R8 | Public fixtures, examples, report snippets, and metadata examples shall not copy protected standards text, protected tables, proprietary formulas, proprietary engineering values, private project data, private rule-pack payloads, private library content, real secrets, or unauthorized commercial software examples. | `docs/IP_AND_DATA_BOUNDARY.md` sections 2-6; `docs/SPEC.md` report/result export boundary sections | Protected-content/private-data review or lint evidence is required for public examples. |
| DEL-15-04-R9 | Metadata records that become public data contributions shall carry source, provenance, license/redistribution status, contributor certification, and review disposition where applicable. | `docs/IP_AND_DATA_BOUNDARY.md` section 4; `docs/CONTRACT.md` OPS-K-IP-2 | Schema/review checks confirm required provenance slots or explicit `TBD`. |
| DEL-15-04-R10 | The deliverable shall align with the accepted architecture basis: schema-first contracts, JSON Schema 2020-12 where applicable, canonical hash basis where JSON payloads are hashed, and result/diagnostic envelope boundaries where relevant. | `_CONTEXT.md` Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEC-010 | Review confirms no incompatible schema/API/hash assumptions are introduced. |

## Standards

| Standard / Policy | Applicability | Status |
|---|---|---|
| OpenPipeStress governance invariants | Binding project constraints for authority, data boundary, provenance, and agent behavior | Available in `docs/CONTRACT.md` |
| OpenPipeStress vocabulary/status model | Defines automatic status limitations and epistemic labels | Available in `docs/TYPES.md` |
| OpenPipeStress technical specification | Defines schema-first domain surfaces, persistence, adapter, report, result, and authority-boundary behavior | Available in `docs/SPEC.md` |
| OpenPipeStress IP/data boundary policy | Governs protected/private data and public contribution provenance | Available in `docs/IP_AND_DATA_BOUNDARY.md` |
| External engineering codes/standards | No clause-level content is available or required for this setup deliverable | `TBD`; do not quote or encode protected standards content |

## Verification

| Verification Target | Required Evidence |
|---|---|
| Required metadata categories are representable | Boundary validation tests or schema review evidence for names, tags, notes, external references, attachments, and relevant handoff/comparison links |
| Prohibited automatic statuses are blocked | Negative tests for approval, certification, code-compliance, sealing, authentication, professional approval, and formal prover lifecycle labels |
| Human acceptance remains external and hash-bound | Tests or review evidence showing software metadata cannot create professional acceptance and cannot preserve acceptance across bound-hash changes |
| Protected/private data is not introduced | Protected-content/private-data review evidence for any fixtures, examples, or sample external references |
| Dependency boundary remains aligned | Review against local `Dependencies.csv` mirror, especially DEL-01-04, DEL-15-01, DEL-15-02, DEL-15-03, and DEL-14-01 upstream context |

## Documentation

Expected artifacts from `_CONTEXT.md` and `docs/_Registers/Deliverables.csv`:

- external reference fields;
- boundary validation tests.

The exact schema filename, field names, validation implementation, fixtures, and test harness remain `TBD` until a later implementation task defines them under explicit write scope.
