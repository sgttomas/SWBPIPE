# Guidance: DEL-03-06 Expansion joint component model

## Purpose

The purpose of this deliverable is to prepare the product to model expansion joints as supplied-data components while preserving OpenPipeStress boundaries: open mechanics and public schemas are allowed, but protected standards content, vendor proprietary values, and invented defaults are not.

## Principles

- Treat stiffness, effective area, movement limits, and hardware data as supplied inputs, not built-in knowledge.
- Make missing required values visible with diagnostics and `TBD` placeholders.
- Preserve provenance on every material/component data path where values may affect analysis, rules, reports, or redistribution.
- Keep the expansion joint model inside PKG-03's data-model responsibility; solver interpretation and rule evaluation remain downstream package concerns.
- Use architecture basis constraints as implementation guardrails, not as permission to implement product code in this setup pass.

## Considerations

Expansion joints can introduce nonlinear, directional, hardware-dependent, or manufacturer-specific behavior. The sealed setup evidence does not include authoritative product data or design rules, so the model should keep unknowns explicit. Specific field shapes, movement-limit taxonomy, hardware enumerations, and downstream solver semantics are `TBD`.

The Pass 3 lensing register specifically keeps the hardware flag/enumeration taxonomy as `TBD`. Do not convert this into a fixed list without authoritative source material or human ruling.

## Trade-offs

| Choice | Benefit | Risk / Constraint |
|---|---|---|
| Supplied-data-only fields | Protects IP boundary and avoids invented defaults. | Requires clear missing-data diagnostics and user/library workflows. |
| Unit-aware generic field structure | Keeps persistence and adapters deterministic. | Exact domain taxonomy remains TBD until implementation design. |
| Provenance-first data model | Supports auditability and public/private library separation. | Requires validation and review fields even for simple examples. |
| Data-model-only setup scope | Respects PKG-03 boundaries. | Solver behavior and rule checks must be handled by downstream deliverables. |

## Examples

No manufacturer example values are provided or invented in this setup pass. Example records for documentation or tests must use invented, clearly non-commercial data and must avoid protected standards or manufacturer data.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict identified in accessible setup sources. | N/A | N/A | N/A | N/A | N/A |
