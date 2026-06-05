# Guidance: DEL-05-05 Concentrated and distributed user load application

## Purpose

This guidance frames the implemented DEL-05-05 user-load slice so
user-applied concentrated and distributed loads can be used and extended
without crossing into code-specific load combination logic, invented
engineering defaults, or professional approval claims.

## Principles

- Treat load magnitudes, locations, directions, distribution definitions, and coordinate references as user/model data unless a separate sealed source authorizes otherwise.
- Preserve the solver/rule separation: mechanics may be solved, while acceptability and code compliance remain user/rule-pack and human-judgment concerns.
- Keep dimensional analysis explicit. A load input that lacks a required unit,
  direction basis, location basis, distribution basis, target, quantity, or
  straight-pipe geometry should become a deterministic finding or preserved
  TBD rather than a silent default.
- Keep result hooks narrow: they should expose mechanical force/moment/result
  data and provenance needed by stress recovery, reporting, export, GUI, and
  headless execution work, not code-specific conclusions.
- Use `core/loads/user_loads` as the current implementation evidence for this
  deliverable; do not infer additional solver, schema, GUI, or report behavior
  beyond that crate without a sealed task.

## Considerations

- SOW-052 is the direct scope item for concentrated forces, concentrated moments, and distributed user loads.
- SOW-013 is relevant as the primitive load-case context, but this deliverable should not absorb the full primitive load-case engine.
- AB-00-03 and AB-00-06 make status and result-envelope separation important for downstream GUI, CLI, reports, and rule-pack consumers.
- AB-00-08 means the implemented module must keep deterministic tests as the
  evidence basis before release use.
- Generic `apply_user_loads` handles nodal concentrated forces, nodal
  concentrated moments, and element uniform distributed-load records. It
  reports element-station loads as requiring straight-pipe equivalent recovery.
- `apply_straight_pipe_equivalent_user_loads` handles oriented straight-pipe
  equivalent recovery for full-span and partial-span distributed loads and
  element-station concentrated forces.
- `apply_straight_pipe_equivalent_user_loads_with_axial_effects` appends
  already-prepared primitive axial-effect contributions through
  `StraightPipeElement` equivalent-load behavior; it does not introduce a new
  axial-effect mechanics model in this crate.

## Trade-offs

- A narrower user-load module reduces risk of accidentally encoding protected or jurisdiction-specific load combinations.
- Keeping final result-envelope/API/persistence/GUI/CLI/report integration as
  TBD avoids overstating the implementation slice while preserving current
  recovery-hook evidence.
- The straight-pipe equivalent APIs support the current oriented straight-pipe
  scope; other element families or higher-level model assembly should remain
  separately dispatched.
- Distributed-load support is implemented for uniform translational loads over
  normalized spans, including valid partial spans. Other distribution shapes,
  magnitudes, factors, or defaults are not invented here.

## Examples

- Existing Rust unit tests use artificial mechanics fixtures for concentrated
  forces, concentrated moments, uniform distributed loads, partial spans,
  oriented straight-pipe recovery, station point forces, and axial effects.
- Public or product-facing examples remain TBD unless separately approved with
  non-protected fixture values, explicit unit provenance, and no compliance
  claims.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict detected in current implementation evidence. | N/A | N/A | N/A | N/A | N/A |
