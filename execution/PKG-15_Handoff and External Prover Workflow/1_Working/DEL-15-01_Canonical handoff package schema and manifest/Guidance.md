# Guidance: DEL-15-01 Canonical handoff package schema and manifest

## Purpose

DEL-15-01 exists to define a canonical handoff package schema and manifest for downstream modeling and professional validation workflows. The package is evidence-transfer infrastructure: it should make model basis, units, identifiers, warnings, assumptions, provenance, and mapping limitations visible without claiming that software output is professional approval.

Sources: `_CONTEXT.md`; SOW-074; OBJ-017; `execution/_Decomposition/SOFTWARE_DECOMP.md#7-deliverables`.

## Principles

| Principle | Guidance | Source |
|---|---|---|
| Schema-first contract | Treat the handoff package as a stable API/data contract before runtime export behavior is implemented. | Deliverable type `API_CONTRACT`; JSON Schema 2020-12 architecture basis |
| Evidence preservation | Prefer references, hashes, manifests, and provenance over copied private/protected payloads. | `docs/IP_AND_DATA_BOUNDARY.md`; OPS-K-IP-1 through OPS-K-DATA-3 |
| Unit explicitness | Do not allow missing or ambiguous units to become downstream assumptions. | `docs/SPEC.md#4-unit-system-and-dimensional-analysis`; OPS-K-UNIT-1 |
| Stable identity | Preserve stable object/entity identity so downstream mapping can be reviewed and reconciled. | SOW-074; `docs/TYPES.md#2-stable-identifiers` |
| Boundary clarity | Keep target mapping and unsupported behavior explicit, but do not invent the target list or detailed mapping taxonomy in this deliverable. | SOW-074; DEL-15-02 row; OI-015 |
| Professional non-authority | The handoff package may support professional validation workflows, but it must not create automatic approval, certification, sealing, endorsement, or code-compliance states. | OBJ-017; OPS-K-AUTH-1; `docs/TYPES.md#4-analysis-status-vocabulary` |

## Considerations

- The package container and supported target list are explicitly TBD in OI-015. A future schema draft should isolate these as open decisions rather than implying a chosen external tool or file package.
- DAG-002 rows identify important upstream context, including architecture basis, local FEA handoff contract, result export format, audit manifest/model hash, immutable model states, analysis run records, and canonical domain model schema. These rows are dependency evidence, not authority to inspect or modify sibling deliverable folders during this task.
- Target mapping metadata and unsupported-target flags are required by SOW-074, but detailed semantics belong to DEL-15-02. DEL-15-01 should reserve clear schema surfaces for those records.
- Public examples or fixtures for this contract must use invented or otherwise reviewed data. They must not include protected standards text, private rule-pack payloads, proprietary commercial files, or real project data.
- Handoff package validation should focus on schema conformance, unit/provenance completeness, hash/manifests, and professional-boundary language. It should not attempt to prove external solver correctness or code compliance.

## Trade-offs

| Trade-off | Preferred direction | Reason |
|---|---|---|
| Rich payload vs reference-heavy manifest | Prefer reference-heavy records for private/protected or large payloads. | Preserves data boundary and avoids copying private/protected values. |
| Target-neutral vs target-specific fields | Keep DEL-15-01 target-neutral and reserve extension/mapping surfaces. | Target-specific strategy is TBD and detailed target mapping belongs to DEL-15-02. |
| Strict required fields vs unresolved project reality | Require the slots named by SOW-074; mark exact property names and target details TBD until schema drafting. | Avoids inventing implementation evidence or unsupported field vocabulary. |
| Software status vs professional status | Emit diagnostics/findings and human-review-needed notices, not approval/compliance states. | Maintains OPS-K-AUTH-1 and OBJ-017. |

## Examples

No concrete example payload is source-supported in the accessible DEL-15-01 references. Example records are TBD until an invented or reviewed fixture is authorized. Any future example must avoid protected standards content, private project data, private rule-pack content, proprietary values, real secrets, and automatic professional-approval claims.

## Conflict Table (for human ruling)

No direct source conflicts were identified in this setup pass. The following open items are unresolved rather than conflicting:

| Conflict ID | Conflict (short statement) | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
|---|---|---|---|---|---|---|
| OI-015-OPEN | Handoff target list, canonical package container, and target-specific mapping strategy are TBD. | `execution/_Decomposition/SOFTWARE_DECOMP.md#11-open-issues` | `_CONTEXT.md#Scope Detail` | Datasheet Attributes; Specification Requirements; Procedure Steps | Keep as TBD in DEL-15-01 until PKG-15/human decision. | TBD |
