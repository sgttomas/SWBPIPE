---
doc_id: DEL-10-03-DATASHEET
doc_kind: deliverable.datasheet
status: draft
created: 2026-04-30
deliverable_id: DEL-10-03
package_id: PKG-10
---

# Datasheet: Local FEA Handoff Data Contract

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-10-03 |
| Package ID | PKG-10 |
| Package | Build, Packaging, API, and Interoperability |
| Type | API_CONTRACT |
| Scope items | SOW-031, SOW-049 |
| Objective | OBJ-009 |
| Anticipated artifacts | `local FEA handoff schema`; `docs/local-analysis notes` |
| Current artifact form | Deliverable-local contract kit; repository-level schemas and docs are outside this setup write scope |
| Lifecycle target for setup | `SEMANTIC_READY` after setup gates pass |
| External FEA implementation | Out of scope |
| Final external format selection | TBD |

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Boundary purpose | Define an export package for selected local shell/solid FEA handoff and advisory labels for when handoff is recommended. | `_CONTEXT.md` Description; `docs/_Registers/Deliverables.csv` row DEL-10-03 |
| Normal global method | OpenPipeStress primary analysis remains a 3D centerline/frame model. | `INIT.md` boundaries; `docs/DIRECTIVE.md` section 3; `docs/CONTRACT.md` OPS-K-MECH-1 |
| Handoff role | Local FEA handoff is a specialized interoperability path for local-detail problems, not the normal global analysis method. | `docs/_Registers/ScopeLedger.csv` rows SOW-031 and SOW-049 |
| Contract baseline | Schema-first command/query/job result envelopes; JSON Schema 2020-12 public schema/interchange basis; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed. | `_CONTEXT.md` Architecture Basis Injection; `docs/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 |
| Adapter boundary | Handoff exports are governed adapter payloads and cannot bypass unit checks, provenance, diagnostics, privacy, protected-content screening, report controls, or professional-boundary language. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-02, AB-00-06, AB-00-07; `docs/SPEC.md` section 1 |
| Criteria authority | Handoff criteria labels are guidance only and do not certify that global beam analysis is sufficient or that local FEA is code-compliant. | `_CONTEXT.md` Context Budget QA; `docs/_Registers/ScopeLedger.csv` row SOW-049; `docs/CONTRACT.md` OPS-K-AUTH-1 |
| Protected-data posture | The public contract must not embed protected standards text, protected tables, copied code formulas, allowables, SIF/flexibility tables, protected dimensional tables, proprietary vendor data, or private project/rule data. | `docs/CONTRACT.md` OPS-K-IP-1/2/3; `docs/IP_AND_DATA_BOUNDARY.md` sections 2-6 |

## Conditions

| Condition | Required handling |
|---|---|
| Selected local region lacks stable model identity | Block or mark `TBD`; do not emit an unverifiable handoff package. |
| Units or dimensional basis are missing | Emit `SOLVE_BLOCKING` or export-blocking diagnostics where applicable; do not silently default. |
| Boundary/load/result provenance is missing | Emit provenance diagnostics and keep the missing evidence visible. |
| User-supplied code or proprietary values are needed | Reference only user/private inputs with provenance and redistribution status; do not bundle those values in public examples. |
| Protected or proprietary content is suspected | Stop public-path export/contribution, mark suspected content, and route to human review. |
| Handoff criteria label is shown to a user | Present as advisory guidance that requires competent human review. |
| External solver/tool behavior is requested | Mark `TBD` or out of scope unless a later approved adapter deliverable supplies the behavior. |

## Construction

The local FEA handoff contract is a conceptual export package. It should be described in schema-ready terms without choosing a final external tool format.

| Contract surface | Minimum concept slots | Setup status |
|---|---|---|
| Handoff package identity | Package ID, source project/model IDs, originating OpenPipeStress version, package schema version, creation timestamp, privacy/export posture. | Concept defined; final schema field names TBD. |
| Selected local-detail scope | Region selection reference, included components/elements/nodes, cut boundary description, selection rationale, advisory criteria label. | Concept defined; selection UX and storage details TBD. |
| Global model context | Units, coordinate frame, model hash, relevant materials/sections/components by reference, solve status, load case/result basis, diagnostics summary. | Concept defined; exact result reference structure TBD. |
| Boundary condition transfer | Cut locations, coordinate frames, displacement/force/moment/resultant context, load-case association, sign convention notes, unit metadata. | Concept defined; no external solver mapping chosen. |
| Local geometry/idealization notes | User-supplied local-detail assumptions, meshing/idealization notes, omitted features, open questions, source/provenance records. | Guidance only; no shell/solid mesh implementation. |
| Handoff guidance label | Advisory labels such as `GLOBAL_CENTERLINE_EXPECTED_SUFFICIENT`, `LOCAL_DETAIL_REVIEW_RECOMMENDED`, `LOCAL_FEA_HANDOFF_RECOMMENDED`, and `HUMAN_REVIEW_REQUIRED`. | Proposed vocabulary for future review, not certification. |
| Diagnostics and limitations | Diagnostic code/class/severity/source/affected object/message/remediation/provenance, warnings, assumptions, limitations, unresolved `TBD`s. | Governed by AB-00-06. |
| Reproducibility manifest | Canonical JSON payload hash where applicable, model/result hashes, source pointers, rule-pack references without exposing private values. | Concept defined; canonicalization edge cases TBD. |

## References

- `_CONTEXT.md` for sealed deliverable identity, acceptance/risk notes, write scope, and architecture-basis injection.
- `docs/CONTRACT.md` for applicable invariants: OPS-K-IP-1/2/3, OPS-K-DATA-1/2/3, OPS-K-UNIT-1, OPS-K-PRIV, OPS-K-AUTH-1, OPS-K-AGENT-1..4, and professional-responsibility boundaries.
- `docs/DIRECTIVE.md` sections 1, 3, 4, and 5 for centerline-first intent, local FEA handoff as specialized path, protected-data limits, and stop rules.
- `docs/TYPES.md` sections 3, 4, 5, 6, 7, and 8 for deliverable type, analysis status, epistemic labels, local FEA handoff vocabulary, provenance labels, and domain object registry.
- `docs/SPEC.md` sections 1, 3, 4, 7, 8, 9, 10, and 11 for architecture layering, domain objects, solver/result/report boundaries, diagnostics, verification, and acceptance semantics.
- `docs/IP_AND_DATA_BOUNDARY.md` sections 2-6 for public/private data boundaries, provenance, quarantine, and private user data.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 for SOW-031, SOW-049, OBJ-009, PKG-10, AB-00-02/03/04/06/07/08, and OI-004.
