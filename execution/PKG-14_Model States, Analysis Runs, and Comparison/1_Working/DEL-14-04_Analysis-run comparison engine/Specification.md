# Specification: DEL-14-04 Analysis-run comparison engine

**Generated:** 2026-05-03
**Document Role:** Normative draft
**Source Basis:** `_CONTEXT.md`, `_REFERENCES.md`, `Dependencies.csv`, `execution/_Decomposition/SOFTWARE_DECOMP.md`, `docs/CONTRACT.md`, `docs/SPEC.md`, `docs/TYPES.md`, `docs/IP_AND_DATA_BOUNDARY.md`

## Scope

DEL-14-04 covers the backend feature slice for an analysis-run comparison engine. It compares two analysis runs, and where applicable their bound model states, using stable IDs, manual mappings where required, unit-normalized result deltas, diagnostics, settings, and tolerance profiles.

DEL-14-04 excludes comprehensive commercial-prover result ingestion, external validation, professional approval, certification, sealing, authentication, automatic code-compliance determination, and protected standards-data content.

## Requirements

| ReqID | Requirement | Source | Verification |
|---|---|---|---|
| R-14-04-001 | The engine shall compare analysis runs deterministically using stable IDs and manual mappings where required. | SOW-073 in `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` | Deterministic comparison tests with repeated identical inputs; concrete fixture set `TBD`. |
| R-14-04-002 | The engine shall treat analysis-run records as bound to exact model states, solver versions, settings, units, load cases, diagnostics, results, rule-pack references, library references, and result hashes. | SOW-072 in `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` | Tests assert comparison input metadata is preserved or surfaced; schema/API field names `TBD`. |
| R-14-04-003 | The engine shall compute unit-normalized result deltas only where unit and dimension metadata allow valid comparison. | SOW-073; `docs/SPEC.md` unit contract | Unit-normalization tests; missing/ambiguous unit metadata produces diagnostics rather than silent defaults. |
| R-14-04-004 | The engine shall include mapped nodes, elements, supports, terminals, stress/result locations, diagnostics, and settings within the comparison scope where the upstream result records expose them. | DEL-14-04 decomposition row | Coverage tests by result/entity category; exact category enum `TBD`. |
| R-14-04-005 | The engine shall use tolerance profiles for comparison classification. | SOW-073; dependency row for DEL-14-05; OI-014 | Tolerance-profile tests after DEL-14-05 contracts are available; defaults and mapping workflows remain `TBD`. |
| R-14-04-006 | The engine shall preserve professional-boundary language: comparison output is diagnostic/audit evidence and must not claim external validation or engineering acceptance. | SOW-073 notes; `docs/CONTRACT.md` OPS-K-AUTH-1/OPS-K-MECH-2; `docs/SPEC.md` analysis boundary | Protected wording/status tests; report/export assertions when applicable. |
| R-14-04-007 | The engine shall remain compatible with schema-first result/export envelopes rather than creating an ad hoc comparison result surface. | `_CONTEXT.md` architecture basis; `docs/SPEC.md` result export and runner output text | Contract tests against accepted result/export schema once available; concrete schema path `TBD`. |
| R-14-04-008 | The engine shall not introduce protected standards text, protected tables, proprietary values, private rule-pack payloads, or invented engineering defaults into public artifacts. | `docs/CONTRACT.md` OPS-K-IP-1/OPS-K-DATA-1/OPS-K-AGENT-1; `docs/IP_AND_DATA_BOUNDARY.md` | Protected-content/provenance checks; review of examples and fixtures. |
| R-14-04-009 | The deliverable shall include result delta tests as an anticipated artifact. | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` | Test presence and execution through the applicable test gate; command `TBD`. |

## Standards

No external engineering standard text is locally present for this deliverable, and no clause-level engineering requirements are asserted here.

Project-governing references for this draft are:

- `docs/CONTRACT.md` for invariants.
- `docs/SPEC.md` for unit, analysis-boundary, persistence/hash, result-export, runner-output, and validation mechanics.
- `docs/TYPES.md` for vocabulary and boundary notes.
- `docs/IP_AND_DATA_BOUNDARY.md` for protected-content/private-data limits.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` for accepted revision 0.5 scope and open issues.

## Verification

| Verification Item | Required Evidence | Current Status |
|---|---|---|
| Determinism | Same input pair produces identical ordered comparison output. | `TBD` fixture/API shape. |
| Stable-ID/mapping behavior | Stable-ID match path and manual-mapping path are both exercised. | `TBD`; depends on DEL-14-05 mapping contract. |
| Unit-normalized deltas | Same-dimension values compare after accepted unit normalization; incompatible/missing units emit diagnostics. | `TBD`; depends on DEL-02-02 unit contract maturity. |
| Diagnostics/settings deltas | Diagnostic and settings differences are surfaced without compliance claims. | `TBD`; depends on result/run record schema. |
| Tolerance profiles | Tolerance profile affects classification without changing raw delta evidence. | `TBD`; OI-014 open. |
| Boundary wording | Output statuses avoid human approval, certification, sealing, authentication, and automatic code-compliance labels. | Required by governance; concrete assertions `TBD`. |

## Documentation

The implementation brief or later production work should document:

- comparison input contract;
- mapping/tolerance profile dependency points;
- delta output structure;
- diagnostic/status behavior;
- unit-normalization behavior and failure diagnostics;
- deterministic ordering/hash basis;
- result delta test fixture provenance;
- limitations and professional-boundary notice.
