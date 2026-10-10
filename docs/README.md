# SWBPIPE documentation

SWBPIPE computes open, auditable piping mechanics. The responsible engineer
supplies the governing data and accepts engineering reliance.

- [PRD](PRD.md): product commitments and scope.
- [Contract](CONTRACT.md), [specification](SPEC.md), [types](TYPES.md): product invariants and interfaces.
- [User guide](user_guide/index.md), [developer guide](developer_guide/index.md), [contributor guide](contributor_guide/index.md).
- [Build and release](BUILD_AND_RELEASE.md), [CI](CI_STRATEGY.md), [release quality](RELEASE_QUALITY_GATES.md).
- [Validation strategy](VALIDATION_STRATEGY.md), [validation manual](validation_manual/index.md), [theory](theory/centerline_analysis.md).
- [IP and data boundary](IP_AND_DATA_BOUNDARY.md), [professional boundary](PROFESSIONAL_BOUNDARY.md), [claims](claims_registry.md), [security](security/threat_model.md).

Development starts at [LOOP_INIT](../loop/LOOP_INIT.md) and Root `AGENTS.md`.
Current commitments, Design and dependency conditions live in the deliverable
folders. From the repository root, use `python3 -m tools.deliverables --help`
for neighbourhood, impact, changed-path and dependency queries. Historical
plans, registers and source editions remain recoverable through Git/archive tags.
