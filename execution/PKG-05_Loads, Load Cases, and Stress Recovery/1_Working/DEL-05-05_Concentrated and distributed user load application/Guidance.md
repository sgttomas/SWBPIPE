# Guidance: DEL-05-05 Concentrated and distributed user load application

## Purpose

This setup evidence frames the future DEL-05-05 work so user-applied concentrated and distributed loads can be implemented without crossing into code-specific load combination logic or invented engineering defaults.

## Principles

- Treat load magnitudes, locations, directions, distribution definitions, and coordinate references as user/model data unless a future sealed source authorizes otherwise.
- Preserve the solver/rule separation: mechanics may be solved, while acceptability and code compliance remain user/rule-pack and human-judgment concerns.
- Keep dimensional analysis explicit. A load input that lacks a required unit, direction basis, location basis, or distribution basis should become a finding/TBD rather than a silent default.
- Keep result hooks narrow: they should expose mechanical force/moment/result data and provenance needed by stress recovery and reporting, not code-specific conclusions.

## Considerations

- SOW-052 is the direct scope item for concentrated forces, concentrated moments, and distributed user loads.
- SOW-013 is relevant as the primitive load-case context, but this deliverable should not absorb the full primitive load-case engine.
- AB-00-03 and AB-00-06 make status and result-envelope separation important for downstream GUI, CLI, reports, and rule-pack consumers.
- AB-00-08 means the future module should be designed with deterministic tests from the start.

## Trade-offs

- A narrower user-load module reduces risk of accidentally encoding protected or jurisdiction-specific load combinations.
- Deferring exact schema and solver-hook names keeps this setup evidence aligned with the brief, but leaves implementation interfaces as TBD for a later sealed task.
- Distributed-load support will likely require clear modeling semantics, but no distribution shapes, magnitudes, or defaults are invented here.

## Examples

- TBD: future examples require sealed, non-protected fixture values and explicit unit provenance.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict detected in setup evidence. | N/A | N/A | N/A | N/A | N/A |

