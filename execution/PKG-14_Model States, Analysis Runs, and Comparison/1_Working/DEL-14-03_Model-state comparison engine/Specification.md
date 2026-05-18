# Specification: DEL-14-03 Model-state comparison engine

## Scope

This deliverable specifies a backend feature slice for deterministic comparison of immutable model states. It covers the model-state side of SOW-073 and the SOW-071 state-record context needed by that comparison. It must report added, removed, changed, and unchanged model entities using stable IDs and explicit mapping records where required.

This deliverable excludes the full analysis-run comparison engine (`DEL-14-04`), the mapping/tolerance/export contract definition (`DEL-14-05`), the immutable state data model itself (`DEL-14-01`), and any claim of external validation, certification, code compliance, sealing, or professional approval.

Sources: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` rows `DEL-14-01` through `DEL-14-05`; `docs/_Registers/ScopeLedger.csv` rows `SOW-071` and `SOW-073`.

## Requirements

| ID | Requirement | Source | Verification |
|---|---|---|---|
| REQ-14-03-001 | The engine shall compare two immutable model-state records or references once the `DEL-14-01` state contract is available. | `docs/_Registers/ScopeLedger.csv` row `SOW-071`; `Dependencies.csv` row `DAG-002-E0792` | Unit test with two known state fixtures; schema validation once state schema exists. |
| REQ-14-03-002 | The engine shall use stable IDs as the primary entity matching basis. | `docs/_Registers/ScopeLedger.csv` row `SOW-073`; `docs/TYPES.md` `Reference` definition | Unit tests showing order-independent, stable-ID-based matching. |
| REQ-14-03-003 | The engine shall require explicit mapping records where comparison cannot rely on direct stable-ID correspondence. | `docs/_Registers/ScopeLedger.csv` row `SOW-073`; `Dependencies.csv` row `DAG-002-E0793` | Unit tests for manually mapped entities after `DEL-14-05` defines mapping records. |
| REQ-14-03-004 | The engine shall classify model entities as added, removed, changed, or unchanged. | `_CONTEXT.md` Context Envelope; `execution/_Decomposition/SOFTWARE_DECOMP.md` row `DEL-14-03` | Unit tests for each classification. |
| REQ-14-03-005 | The same two input states, mappings, and comparison settings shall produce deterministic diff output. | `docs/_Registers/ScopeLedger.csv` row `SOW-073`; `docs/CONTRACT.md` `OPS-K-ID-1`; `docs/SPEC.md` section 4.4 | Repeat-run tests comparing serialized result envelopes or canonical hashes where available. |
| REQ-14-03-006 | The comparison shall preserve relevant state metadata, including unresolved assumptions, warnings, notes, external references, and deterministic hashes when those fields are present in the state contract. | `docs/_Registers/ScopeLedger.csv` row `SOW-071`; `docs/SPEC.md` section 3 Domain objects | Tests confirming metadata is not discarded from comparison context. |
| REQ-14-03-007 | Unit-bearing changed values shall not be compared as bare numbers without unit/dimension metadata or an explicit unit-normalization contract. | `docs/SPEC.md` section 4 Unit system and dimensional analysis; `Dependencies.csv` row `DAG-002-E0794` | Tests for unit metadata handling; `TBD` until `DEL-02-02` and `DEL-14-05` provide final contracts. |
| REQ-14-03-008 | Comparison result envelopes shall carry diagnostics/provenance sufficient to expose missing mappings, unsupported comparison categories, unresolved assumptions, and professional-boundary limits. | `docs/SPEC.md` sections 4.3 and 9; `execution/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-06` | Tests asserting diagnostic fields and no forbidden status labels. |
| REQ-14-03-009 | Outputs shall not claim certification, sealing, authentication, professional approval, code compliance, or external validation. | `docs/CONTRACT.md` `OPS-K-AUTH-1`; `docs/DIRECTIVE.md` section 3; `docs/_Registers/ScopeLedger.csv` row `SOW-073` | Protected-claim review of result labels and report/export-facing strings. |
| REQ-14-03-010 | State diff tests shall use invented, public-safe fixtures and must not include protected standards text, proprietary values, or private project data. | `docs/CONTRACT.md` `OPS-K-IP-1`, `OPS-K-RULE-1`; `docs/SPEC.md` section 10 | Fixture provenance review and protected-content gate. |

## Standards

No external engineering standard text is locally available or required for this deliverable. Governing project standards are the local OpenPipeStress documents listed in `_REFERENCES.md`, especially:

- `docs/CONTRACT.md` for invariant boundaries.
- `docs/SPEC.md` for unit, persistence, result-envelope, and validation expectations.
- `docs/TYPES.md` for stable references, traceability links, checksums, diagnostics, and result/report boundary concepts.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.5 for scope partitioning and architecture-basis constraints.

Any clause-level external standard requirement is `TBD` and must not be inferred.

## Verification

| Verification area | Minimum check |
|---|---|
| Stable-ID matching | Reorder equivalent entity lists and confirm unchanged diff output. |
| Added/removed classification | Compare fixtures with entities present in only one state. |
| Changed classification | Compare fixtures with the same stable ID and intentionally changed public-safe fields. |
| Explicit mapping | Use mapping records from `DEL-14-05` once available; until then, mark detailed mapping behavior `TBD`. |
| Unit-bearing value handling | Confirm missing or incompatible unit metadata produces diagnostics rather than silent numeric comparison. |
| Determinism | Repeat the same comparison and compare canonical serialized output or result hash where implemented. |
| Boundary language | Assert no output status or label implies professional acceptance, code compliance, certification, sealing, or external validation. |

## Documentation

Required local evidence for this deliverable:

- state comparison engine artifact or module path, `TBD` until implementation;
- state diff tests;
- notes identifying dependency on `DEL-14-01`, `DEL-14-05`, and `DEL-02-02`;
- comparison result-envelope shape or service contract, `TBD` until implementation;
- protected-content/provenance status for fixtures and examples.
