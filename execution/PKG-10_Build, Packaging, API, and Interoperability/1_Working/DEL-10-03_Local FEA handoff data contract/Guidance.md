---
doc_id: DEL-10-03-GUIDANCE
doc_kind: deliverable.guidance
status: draft
created: 2026-04-30
deliverable_id: DEL-10-03
package_id: PKG-10
---

# Guidance: Local FEA Handoff Data Contract

## Purpose

This deliverable gives future adapter, API, report, validation, and documentation work a governed data-contract boundary for local shell/solid FEA handoff. It exists to make selected local-detail handoff possible without changing the normal global analysis method, bypassing unit/provenance/privacy controls, importing protected data, or overstating what software can decide.

## Principles

- Treat local FEA handoff as an optional interoperability path. The OpenPipeStress global model remains a 3D centerline/frame model unless later scope explicitly changes that boundary.
- Keep the handoff package descriptive and reproducible. It should identify the source model, selected local region, units, load/result basis, diagnostics, assumptions, provenance, and hashes.
- Keep criteria labels advisory. They can help a user decide where further local review may be warranted, but they cannot certify adequacy, compliance, approval, or professional acceptability.
- Prefer schema-ready concepts and `TBD` placeholders over invented external solver fields, mesh settings, or proprietary exchange behavior.
- Require every dimensional or engineering value in the handoff package to carry units and source/provenance where reliance may be affected.
- Preserve public/private and protected-content boundaries. User-private values may be referenced as private inputs; protected or proprietary data must not become public project content.
- Route future export behavior through governed adapter/API envelopes so diagnostics, privacy controls, provenance checks, and report controls remain active.

## Considerations

The local FEA handoff boundary has several audiences:

| Audience | What it needs | Boundary implication |
|---|---|---|
| Global piping analyst | A clear advisory reason why local-detail review is being considered. | Provide guidance labels and limitations, not certification. |
| External FEA specialist | Enough context to reconstruct a local problem responsibly. | Include selected region, units, coordinate frame, load/result basis, boundary-condition concepts, diagnostics, and assumptions. |
| Adapter/API implementer | Stable schema-ready contract surfaces. | Keep final tool formats and export mechanics TBD until a later implementation brief. |
| Reviewer/auditor | Evidence that handoff did not bypass governance. | Preserve provenance, privacy classification, hashes, diagnostics, protected-content checks, and professional-boundary notices. |

Advisory label examples for future schema review:

| Label | Intended meaning | Boundary note |
|---|---|---|
| `GLOBAL_CENTERLINE_EXPECTED_SUFFICIENT` | Current evidence suggests the centerline model is the appropriate analysis level for the stated purpose. | Guidance only; not certification. |
| `LOCAL_DETAIL_REVIEW_RECOMMENDED` | A local feature, assumption, or uncertainty should be reviewed before relying on global results alone. | Requires human interpretation. |
| `LOCAL_FEA_HANDOFF_RECOMMENDED` | A selected local-detail problem may need shell/solid treatment outside the global solver. | Does not implement or validate external FEA. |
| `HUMAN_REVIEW_REQUIRED` | Professional review is required before project reliance. | Always true for professional use. |

General factors that may support local-detail review include local geometry or restraint behavior that is not represented by the centerline model, local load introduction, attachment/nozzle/equipment interface concerns, localized discontinuity behavior, or unresolved assumptions. These are screening prompts, not automatic rules.

General factors that may support staying with global centerline analysis include a question limited to system flexibility, displacements, reactions, member forces/moments, and open-mechanics stress recovery within the model scope, with complete solve-required data, unit consistency, and visible assumptions. This is still subject to competent review.

## Trade-offs

| Trade-off | Guidance |
|---|---|
| More local-detail information vs. private-data exposure | Export only what is needed for the selected handoff and preserve privacy/provenance classifications. |
| Early format selection vs. interoperability flexibility | Keep final external formats TBD until adapter work can evaluate unit, provenance, privacy, and protected-content gates. |
| Advisory criteria vs. false certainty | Use labels and rationale, not pass/fail certification. |
| Boundary condition detail vs. solver-specific behavior | Describe source model results, cut context, units, and sign conventions without choosing external solver mapping. |
| Public examples vs. protected data risk | Use invented examples only in future documentation and validation. |

## Examples

No engineering numeric example, protected standards example, proprietary vendor example, copied commercial-software example, or tool-specific instruction is included here.

Acceptable invented examples for later work:

- A handoff package manifest that references a source model hash, selected region ID, unit system, local coordinate frame, load-case/result basis, diagnostics summary, and privacy classification.
- A local-review note that records an advisory label, human-readable rationale, open assumptions, and required reviewer follow-up.
- A result-envelope concept showing that a handoff export was blocked because units, provenance, or selected-region identity were incomplete.

## Human-Ruling Queue

| Topic | Current disposition |
|---|---|
| Final local FEA handoff schema filename and repository location | TBD; outside this setup write scope |
| Final external FEA format list | TBD |
| Exact adapter implementation and external solver invocation behavior | TBD |
| Exact handoff package field names and JSON Schema layout | TBD |
| Exact advisory criteria label vocabulary | PROPOSAL in this deliverable; requires later human/API review |
| Mapping from global model results to external shell/solid boundary conditions | TBD; no solver-specific behavior selected |
| Validation fixtures for handoff export | TBD; future invented/public examples only |

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict detected during setup. Remaining issues are explicit TBDs or advisory vocabulary proposals rather than contradictory source claims. | NA | NA | NA | Keep TBDs visible until human/project authority records decisions. | TBD |
