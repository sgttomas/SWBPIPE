# Guidance: DEL-14-01 Immutable model state records

## Purpose

Immutable model state records give OpenPipeStress a reproducible saved-state surface for design iteration, comparison, reporting, and handoff workflows. The deliverable is a schema/data-model change, not a professional approval workflow.

Sources: `_CONTEXT.md`; OBJ-016 and PKG-14 in `execution/_Decomposition/SOFTWARE_DECOMP.md`; `docs/SPEC.md` sections 4.4 and 9.

## Principles

- Treat the model state as a saved snapshot record, not as a mutable "current model" alias.
- Keep flexible review metadata such as names, tags, notes, external references, unresolved assumptions, and warnings explicit.
- Bind reproducibility to deterministic hashes with explicit payload scope; do not imply that a hash proves engineering correctness.
- Preserve the separation between mechanics/software evidence and human professional reliance.
- Keep protected standards data, owner standards, proprietary vendor data, and private project data out of public examples and fixtures unless separately reviewed and cleared.
- Prefer `TBD` over invented schema details when exact property names, cardinality, hash partitioning, or persistence APIs are not source-supported.

## Considerations

The accepted architecture basis makes this deliverable schema-first and hash-aware, but it does not yet select exact dependency versions, physical project package/container details, or package-specific implementation choices (`_CONTEXT.md`, Architecture Basis Injection). Those choices should remain explicit implementation `TBD`s until a later authorized task resolves them.

The local DAG-002 mirror identifies several upstream evidence surfaces: architecture-basis deliverables, canonical model schema, project persistence/serialization, audit manifest/model hash, and analysis-status vocabulary. These rows are coordination evidence only; they do not dispatch implementation work or authorize lifecycle promotion (`Dependencies.csv`; `execution/_DAG/DAG-002/APPROVAL_RECORD.md`).

The model state should not borrow formal prover-status language. SOW-071 and the context budget note both state that model states are flexible metadata records, not formal prover approval states.

## Trade-offs

| Topic | Conservative setup position |
|---|---|
| Metadata flexibility vs. formal status lifecycle | Support flexible names, tags, notes, references, assumptions, and warnings; avoid hard-coded external approval statuses. |
| Hashing precision vs. premature implementation choice | Require deterministic hash metadata and payload scope, but leave exact implementation library and non-JSON partitioning TBD. |
| Snapshot immutability vs. persistence mechanics | Specify read-only snapshot semantics; defer the exact enforcement mechanism to implementation. |
| External references vs. private-data risk | Permit explicit references while preserving privacy/protected-content checks and avoiding embedded private payloads. |

## Examples

No concrete example payload is source-supported in the accessible material. Future examples should use invented or cleared public data, document provenance, and avoid protected standards text or proprietary project values.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None recorded | No direct source conflict identified during setup. | TBD | TBD | TBD | TBD | TBD |
