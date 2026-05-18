# Guidance: DEL-16-01 Structured model operation schema

## Purpose

This deliverable exists to make model edits explicit, reviewable, and controllable before they change accepted model state. The key source constraint is SOW-069: GUI and agent edits must be structured model operations that pass schema validation, constraint validation, diff preview, and controlled application through the model engine.

Sources: `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-069, OBJ-015, PKG-16; `_CONTEXT.md` Context Envelope.

## Principles

| Principle | Guidance |
|---|---|
| Operation records are proposals until controlled application | Treat GUI and agent edits as operation data, not as direct persisted-project mutation. |
| Schema shape is not engineering authority | The schema can define fields, references, diagnostics, and provenance hooks; it must not assert engineering acceptance or code compliance. |
| Preserve source boundaries | Fixtures and examples must remain invented/schema-shape-only unless a source has documented redistribution rights and review disposition. |
| Prefer references over duplicate model structure | Use stable references into canonical model/domain objects where possible; do not redefine the whole model inside operation records. |
| Expose missing inputs | Missing solve-required or rule-check-required information should surface as findings/TBDs, never hidden defaults. |
| Keep validation ownership separate | DEL-16-01 defines operation structure; DEL-16-02 owns validation and diff-preview behavior. |

Sources: `docs/CONTRACT.md` OPS-K-DATA-2, OPS-K-AUTH-1, OPS-K-AGENT-1, OPS-K-AGENT-4; `docs/TYPES.md` `Model`, `Reference`, `TraceabilityLink`, `Diagnostic`; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-16-02 row.

## Considerations

- The available sources do not define exact operation field names, payload envelopes, target-addressing grammar, patch format, or fixture contents. These remain TBD.
- The decomposition records open issue `OI-012`, which explicitly says physical model schema boundaries, operation granularity, and transformation loss classes require technical architecture detail before implementation briefs are sealed.
- Agent operation autonomy remains TBD under `OI-016`; the default SCA-002 scope requires user acceptance and audit trail for proposed changes.
- JSON Schema 2020-12 is an accepted baseline, but code-generation tooling and exact schema file layout remain implementation-level TBD.
- The local `Dependencies.csv` rows show upstream architecture, domain model, design knowledge, persistence, and professional-boundary predecessors. They are an approved DAG-002 mirror/evidence surface, not independent authority to rewrite dependency semantics.

## Trade-offs

| Topic | Conservative Position |
|---|---|
| Broad operation enum vs specialized records | TBD. The deliverable names categories, but available sources do not decide whether each category is a subtype, variant, or separate schema. |
| Patch-like payload vs command-like payload | TBD. Sources require structured operations and controlled application but do not choose a patch grammar. |
| Embedding snapshots vs referencing targets | Prefer references unless a later source requires embedded snapshots; this avoids duplicating canonical model schema. |
| Agent-authored operations | Preserve proposal status and human gate boundaries; do not allow an operation record to imply professional acceptance. |
| Fixtures | Keep fixtures invented and schema-focused; avoid real owner/project data, code-derived values, or proprietary catalog examples. |

## Examples

TBD. The available sources list operation categories but do not provide authoritative example payloads or field-level schema. Any future examples should be invented, protected-content-safe fixtures with explicit provenance and no engineering defaults.

## Conflict Table (for human ruling)

| Conflict ID | Conflict (short statement) | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
|---|---|---|---|---|---|---|
| CT-001 | Operation granularity is required by SOW-069 but not defined in available sources. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-069 | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-012 | Specification Requirements; Procedure Steps | Keep exact schema granularity as TBD until architecture/human ruling. | TBD |
| CT-002 | Agent edits are in scope, but autonomy level is unresolved. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-069 | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-016 | Specification Requirements; Procedure Verification | Treat all agent operations as proposals requiring controlled application and later acceptance workflow. | TBD |
