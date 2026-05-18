---
doc_id: DEL-10-01-GUIDANCE
doc_kind: deliverable.guidance
status: draft
created: 2026-04-30
deliverable_id: DEL-10-01
package_id: PKG-10
---

# Guidance: Public API and Plugin Boundary

## Purpose

This deliverable gives later API, plugin, adapter, headless runner, result export, report, and security work a common boundary contract. It exists to keep interoperability extensible without creating a shortcut around units, provenance, diagnostics, public/private data controls, rule-pack sandboxing, report controls, or professional-responsibility limits.

## Principles

- Treat the public API as a governed service boundary, not a direct tunnel into solver internals, storage internals, private libraries, rule-pack evaluator internals, or report-generation controls.
- Keep transport binding separate from envelope semantics. The envelope can be specified now; HTTP, IPC, CLI JSON, in-process library calls, or other transport choices remain TBD until approved.
- Prefer schema-first contracts and explicit `TBD` placeholders over invented endpoint syntax or plugin manifest fields.
- Let adapters translate external data, but require imported data to pass schema, unit, provenance, redistribution, diagnostics, and data-boundary checks before core use.
- Treat rule-pack hooks as security-sensitive. The hook may expose governed evaluation capabilities later, but it must never authorize arbitrary code execution by implication.
- Preserve the three-state authority boundary: mechanics solved, user-rule checked, and professional approval are distinct.

## Considerations

The API/plugin boundary has two different audiences:

| Audience | What it needs | Boundary implication |
|---|---|---|
| Application services and GUI/headless clients | Stable command/query/job/result envelopes | Do not expose direct mutable domain-core internals. |
| Adapter and plugin authors | Extension points and failure modes | Deny by default; require manifest, capability, diagnostics, provenance, and review status concepts. |
| Project reviewers | Evidence that extension cannot bypass governance | Trace every capability to unit, provenance, rule, privacy, report, and professional-boundary controls. |
| Future implementers | Concrete enough contract to build against | Carry unresolved transport/runtime/schema-layout decisions as visible `TBD`, not hidden assumptions. |

Public API and plugin docs should avoid marketing language such as "certified", "approved", "code compliant", or "professionally accepted". They should describe computed states and human-review obligations using the project vocabulary in `docs/TYPES.md`.

## Trade-offs

| Trade-off | Guidance |
|---|---|
| More public surface vs. safer core boundary | Start with fewer governed operation families and expand only when validation and privacy controls are defined. |
| Early OpenAPI file vs. schema-first envelope clarity | Because repository-level `api/openapi.yaml` is outside this write scope and public transport is TBD, keep the current artifact as an equivalent deliverable-local contract. |
| Plugin flexibility vs. no-bypass controls | Flexibility is acceptable only after capability declarations, sandbox behavior, private-data restrictions, and diagnostics are explicit. |
| External format support vs. protected data risk | External formats remain TBD until each adapter can enforce provenance, redistribution, protected-content, unit, and diagnostics gates. |
| Rule-pack integration vs. arbitrary execution risk | Rule hooks should expose declarative, sandboxed, unit-aware evaluation semantics only; exact grammar/library remains TBD. |

## Examples

No engineering numeric example, protected standards example, proprietary vendor example, or copied commercial-software example is included here.

Acceptable invented boundary examples for later work are conceptual only. They are not approved schema fields, endpoint definitions, transport bindings, or plugin permissions:

- A model-import command envelope that contains schema version, unit system, provenance summary, private/public classification, payload hash, and diagnostic output slots.
- A solve-job request envelope that records model hash, solver version request, cancellation token concept, progress diagnostic stream concept, and final mechanics-result envelope concept.
- A rule-pack-evaluation hook that references rule-pack ID/version/checksum/source notice and returns user-rule status plus missing-input diagnostics without claiming compliance.

## Human-Ruling Queue

| Topic | Current disposition |
|---|---|
| Final public API transport | TBD |
| Repository-level `api/openapi.yaml` or equivalent schema file layout | TBD; outside this setup write scope |
| Concrete plugin runtime and loading mechanism | TBD |
| Permission taxonomy and capability names | TBD |
| Exact import/export format list and priorities | TBD |
| Rule expression grammar/library | TBD; PKG-06/security decision path |
| CI/provider thresholds for API/plugin validation gates | TBD |

## Semantic Lensing Hold Points

The setup lensing pass identified these review hold-points for later work:

- Future repository-level API artifacts need an approved path and schema/transport ruling before endpoint-level details are treated as implementable.
- Plugin capability grants need an approval owner, permission taxonomy, and sandbox/privacy review before any non-default capability is available.
- Conceptual examples remain examples only; exact fields, message shapes, and capability names stay TBD.
- Any exception to private-data deny-by-default behavior requires PKG-12/security rationale and explicit human approval.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict detected during setup. Remaining issues are explicit TBDs rather than contradictory sources. | NA | NA | NA | Keep TBDs visible until human/project authority records decisions. | TBD |
