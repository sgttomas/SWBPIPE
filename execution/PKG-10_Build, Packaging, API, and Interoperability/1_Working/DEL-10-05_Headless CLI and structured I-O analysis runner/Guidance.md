# Guidance: DEL-10-05 Headless CLI and structured I/O analysis runner

## Purpose

This deliverable keeps the early automation path coherent while GUI maturity, packaging details, and result export schemas are still evolving. The headless runner is useful only if it exercises the same governed service boundaries that GUI and downstream automation will rely on.

## Principles

- Treat the runner as an application-service client, not a solver shortcut.
- Keep command/query/job separation visible so progress, cancellation, diagnostics, and result envelopes stay testable.
- Preserve unit safety and deterministic serialization across input, solve, and output.
- Prefer explicit `TBD` over invented command names, schema fields, CI provider decisions, or platform matrices.
- Keep protected standards data, private project data, private rule packs, and proprietary component/material values out of public fixtures.
- Use result statuses as software findings only; professional reliance still requires competent human review.

## Considerations

The future runner should be useful for R0/R1 verification and regression work before a complete GUI exists. That usefulness depends on stable structured I/O, reproducible runs, and machine-readable diagnostics. It does not require this setup pass to pick final commands, package scripts, or file formats.

The runner is also adjacent to several other deliverables. It must align with the command/query/job model, diagnostics/result envelopes, result export format, build/CI pipeline, unit system, persistence/hash strategy, and solver diagnostics. Those relationships are dependencies and constraints, not permission to edit those deliverables here.

## Trade-offs

| Choice | Benefit | Risk | Setup posture |
|---|---|---|---|
| Early headless path before full GUI | Enables solver verification and automation earlier | Can bypass GUI warnings if service boundaries are ignored | Require command/job/result-envelope alignment |
| Schema-first I/O | Supports tests, regression comparison, and downstream tooling | Final fields can be over-specified too early | Keep exact schema fields TBD |
| JSON/JCS-compatible reproducibility | Supports stable hashes and audit trails | Physical package/container remains unresolved | Record accepted basis and defer container details |
| CLI command surface | Familiar for CI and developer workflows | Command names may ossify before architecture is ready | Do not define final command syntax in setup |

## Examples

No executable CLI examples are provided in this setup run. Future examples must use invented/public-permissive data only and include visible diagnostics for missing data, provenance warnings, and professional-boundary notices where applicable.

## Pass 3 Semantic Lensing Notes

The semantic lensing register reinforced three points for downstream work:

- Exact command names and schema fields remain TBD until implementation scope or human approval resolves them.
- Future implementation must include a verification hook showing that no CLI/source, fixture, manifest, or repo-level automation file is modified outside its sealed write scope.
- Result export compatibility must be tested against the schema-first result-envelope baseline instead of an ad hoc output shape.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No conflicting source statements identified during setup. | N/A | N/A | N/A | N/A | N/A |

