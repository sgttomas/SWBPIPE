---
doc_id: DEL-10-03-LOCAL-FEA-HANDOFF-GUIDANCE
doc_kind: API_CONTRACT.guidance
status: draft
created: 2026-05-03
deliverable_id: DEL-10-03
package_id: PKG-10
scope_items: SOW-031,SOW-049
objective: OBJ-009
---

# Local FEA Handoff Guidance Labels

DEL-10-03 defines a target-neutral handoff contract for selected local shell or
solid FEA review. The normal global OpenPipeStress analysis remains the
centerline/frame model. A local handoff package is a specialized downstream
artifact that references global model context, selected entities, load/result
bases, units, assumptions, warnings, hashes, and provenance.

The labels below are advisory routing labels only. They do not certify,
approve, seal, authenticate, or determine code compliance. Final engineering
reliance remains a human responsibility tied to the exact model, result,
assumption, rule, and report records reviewed.

## Labels

| Label | Meaning | Required boundary |
|---|---|---|
| `global_centerline_expected_sufficient_for_screening` | The centerline/frame result set appears suitable for ordinary global review workflows. | Still requires human review before reliance. |
| `local_detail_review_consider` | A selected local feature, support, discontinuity, attachment, boundary transition, or user-marked area needs extra attention before deciding whether local analysis is needed. | No automatic handoff or acceptance decision. |
| `local_shell_solid_handoff_consider` | The record should carry enough references for a local shell/solid FEA workflow to be prepared outside the global solver. | Target solver, mesh, and exchange format remain separate decisions. |
| `global_to_local_transfer_inputs_incomplete` | The package lacks required references, units, load/result bases, hashes, assumptions, or boundary-transfer metadata. | Treat as blocking for handoff package use until resolved. |
| `human_review_required` | A competent human must review the model context, assumptions, diagnostics, limitations, and downstream local-analysis setup. | Required on every guidance assessment. |
| `TBD` | A decision or classification has not been made. | Must not be silently replaced with a default. |

## Contract Notes

- Handoff records reference user-supplied paths, model/result identifiers,
  hashes, and metadata slots; they do not bundle private project models,
  protected standards content, proprietary vendor data, or credentials.
- Units must be explicit through a units manifest. Coordinate, force, moment,
  displacement, rotation, stress, and temperature units are named; conversion
  authority remains with the units contract.
- Unsupported or approximate behavior must be explicit. Examples include no
  mesh generation, no external solver invocation, no target format selection,
  unresolved local-detail assumptions, or review-required boundary transfer.
- Diagnostics follow the normal OpenPipeStress envelope shape: code, class,
  severity, source, affected object, message, remediation, and provenance.
- Concrete external solver adapters, parser behavior, proprietary target
  formats, and solver-specific execution semantics are outside DEL-10-03.
