# Guidance: DEL-03-06 Expansion joint component model

## Purpose

The purpose of this deliverable is to document the implemented expansion-joint component data-model slice while preserving OpenPipeStress boundaries: open mechanics and public schemas are allowed, but protected standards content, vendor proprietary values, and invented defaults are not.

## Principles

- Treat stiffness, effective area, movement limits, and hardware data as supplied inputs, not built-in knowledge.
- Make missing required values visible with diagnostics and `TBD` placeholders.
- Preserve provenance on every material/component data path where values may affect analysis, rules, reports, or redistribution.
- Keep the expansion joint model inside PKG-03's data-model responsibility; solver interpretation and rule evaluation remain downstream package concerns.
- Treat current schema, fixture, and test evidence as bounded data-model implementation evidence, not as a release, lifecycle, dependency-closure, or professional-reliance claim.

## Considerations

Expansion joints can introduce nonlinear, directional, hardware-dependent, or manufacturer-specific behavior. Current evidence implements schema slots, fixture omissions, completeness findings, and diagnostics, but it does not include authoritative product data or design rules. Specific per-axis stiffness shape, movement-limit taxonomy, hardware enumerations, and downstream solver semantics remain `TBD`.

The Pass 3 lensing register specifically keeps the hardware flag/enumeration taxonomy as `TBD`. Do not convert this into a fixed list without authoritative source material or human ruling.

## Trade-offs

| Choice | Benefit | Risk / Constraint |
|---|---|---|
| Supplied-data-only fields | Protects IP boundary and avoids invented defaults. | Requires clear missing-data diagnostics and user/library workflows. |
| Unit-aware field structure using accepted dimensions | Keeps persistence and adapters deterministic for the current schema evidence. | Exact per-axis solver mapping and movement/hardware taxonomy remain TBD. |
| Provenance-first data model | Supports auditability and public/private library separation. | Requires validation and review fields even for simple examples. |
| Data-model-only implementation scope | Respects PKG-03 boundaries. | Solver behavior and rule checks must be handled by downstream deliverables. |

## Examples

The current public fixture uses invented, non-engineering expansion-joint records with missing supplied values and blocking diagnostics. No manufacturer example values are provided or invented. Additional examples for documentation or tests must remain clearly non-commercial and must avoid protected standards or manufacturer data.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict identified in accessible evidence sources. | N/A | N/A | N/A | N/A | N/A |
