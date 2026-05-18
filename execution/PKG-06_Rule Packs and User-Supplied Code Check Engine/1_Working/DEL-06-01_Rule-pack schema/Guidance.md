# Guidance: DEL-06-01 Rule-pack schema

## Purpose

The rule-pack schema exists to let users evaluate solver results against user-owned design bases without placing protected standards content or proprietary engineering values in the public project. The schema should describe the artifact envelope, metadata, provenance, units, required inputs, formula declarations, allowables, criteria, checksums, and statuses needed for later rule evaluation.

## Principles

- Keep public mechanics and public schema structure separate from private code-specific rule content.
- Treat rule packs as user-supplied data artifacts. The public repository may define slots, validation behavior, and example-safe patterns, but not real protected rule content.
- Require source notes, provenance, redistribution status, and checksum metadata before a rule pack can be treated as traceable.
- Treat missing required inputs, missing allowables, missing units, missing provenance, and unresolved redistribution status as explicit findings.
- Preserve unit awareness and dimensional checking through the schema shape even before the evaluator is implemented.
- Preserve the professional boundary: a rule-pack result can support review, but it is not certification, sealing, or professional approval.
- Mark exact expression grammar/library, private storage/encryption behavior, and physical project packaging as `TBD` unless a later human-approved brief resolves them.

## Considerations

The schema will sit between solver/stress results and the later rule evaluator. It should therefore avoid coupling to a specific commercial code while still being structured enough for deterministic validation. The schema should identify variables, dimensional expectations, required inputs, allowable slots, and check criteria, but should not encode protected equations or source text.

Checksum handling should bind to the content actually evaluated. For JSON payloads, the accepted architecture basis points to canonical JSON with JCS-compatible hashing. If future rule packs include non-JSON assets, those assets should be covered by manifest hashes rather than silently excluded.

Provenance should be more than a free-text note where possible. Future schema work should distinguish source type, source pointer, contributor/reviewer role, review disposition, license or redistribution status, and quarantine status for suspected protected content.

## Trade-offs

| Trade-off | Guidance |
|---|---|
| Schema flexibility vs. evaluator safety | Favor a declarative structure that can later be sandboxed and unit checked. Arbitrary executable code is outside the rule-pack boundary. |
| Public examples vs. data-boundary risk | Public examples should use invented non-code values only. Real code-derived examples belong outside the public repository unless lawful redistribution is proven and accepted. |
| Minimal metadata vs. auditability | Favor explicit metadata and checksum fields because reports and audit manifests depend on traceable rule-pack identity. |
| Immediate completeness vs. unresolved architecture choices | Record unresolved grammar, storage, encryption, and packaging details as `TBD` rather than embedding premature choices. |
| Pass/fail convenience vs. professional responsibility | Use pass/fail/incomplete as user-rule check statuses only. Do not imply code compliance or professional acceptance. |

## Examples

Public example rule packs, when later authorized, should:

- use invented rule names, invented source labels, and invented non-engineering values;
- include a clear non-engineering notice;
- avoid standards names, clause numbers, copied examples, copied equations, protected tables, and real material allowables;
- demonstrate schema mechanics such as provenance, units, missing-input diagnostics, and checksum fields without suggesting technical adequacy for design use.

Private user rule packs may contain user-owned or lawfully licensed design basis content, but those private artifacts are not public repository content and should be handled under the private-data controls owned by later PKG-12 work.

## Conflict Table (for human ruling)

No source conflict was found during setup drafting. Open decisions remain `TBD` rather than conflicts:

| Item | Open decision | Current handling |
|---|---|---|
| OI-006 | Exact rule-pack expression grammar/library | `TBD`; schema should remain declarative and sandbox-compatible. |
| OI-010 | Private rule-pack encryption default | `TBD`; defer to security/privacy work and human ruling. |
| OI-011 | Physical project package/container | `TBD`; checksum schema should not assume a container format. |

