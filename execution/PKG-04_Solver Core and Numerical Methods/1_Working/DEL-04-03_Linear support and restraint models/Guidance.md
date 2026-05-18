# Guidance: DEL-04-03 Linear support and restraint models

## Purpose

This deliverable prepares the linear support/restraint slice of the global solver. It should make boundary conditions and support reactions explicit enough for a deterministic mechanics solve while staying code-neutral and protected-data safe.

## Principles

- Treat supports as mechanics-boundary data in the 3D centerline/frame model, not as local shell/solid FEA features. Source: `docs/CONTRACT.md` OPS-K-MECH-1.
- Keep the solver/rule/human boundary visible. A restraint reaction or displacement result is a mechanics result, not an automatic code-compliance finding. Source: OPS-K-MECH-2.
- Prefer explicit required inputs over defaults. If a support stiffness, direction, coordinate basis, imposed displacement value, or target reference is required and absent, emit a finding. Source: OPS-K-DATA-2.
- Keep every numerical support input unit-aware. Translational stiffness, rotational stiffness, displacement, and rotation units need dimensional checks before solve use. Source: OPS-K-UNIT-1.
- Keep linear and nonlinear scope separate. Linear springs and fixed restraints belong here; activation, lift-off, gaps, one-way supports, and friction are DEL-04-04 unless a later sealed brief changes scope.
- Use original or permissive test fixtures. Do not import protected support catalog values, code examples, vendor tables, or copied commercial benchmark cases. Source: OPS-K-IP-1.

## Considerations

- The exact internal representation of an anchor, guide, line stop, vertical support, and spring is TBD. The setup direction is to represent their linear effect through constrained or stiffness-bearing degrees of freedom without inventing proprietary or code-derived defaults.
- Coordinate-frame convention is TBD. The future implementation needs a clear basis for global, local, or user-defined support directions before tests can be treated as deterministic acceptance evidence; this setup kit does not choose that basis.
- Imposed displacement boundary data may interact with load-case handling in PKG-05, but this setup kit only records the support-side boundary-data requirement.
- Diagnostics should be useful to GUI, CLI, report, and adapter consumers through result-envelope fields required by AB-00-06.
- AB-00-01 requires accepted decisions to be recorded when this deliverable resolves deferred choices. This setup kit does not create repo-level ADR files.

## Trade-offs

| Choice | Benefit | Risk / TBD |
|---|---|---|
| Represent rigid restraints as constraints | Clear mechanics boundary condition. | Exact numerical treatment and singularity handling belong to solver implementation and diagnostics. |
| Represent linear springs as unit-bearing stiffness data | Keeps support behavior testable and explicit. | No default stiffnesses may be invented; source/provenance remains required. |
| Keep nonlinear behavior out of this deliverable | Preserves a bounded linear slice. | Users may expect guides/stops to include gaps or one-way action; that expectation must route to DEL-04-04 or remain TBD. |
| Use invented/public benchmark fixtures | Protects IP boundary. | Validation breadth is limited until lawful engineering examples are reviewed. |

## Examples

- Valid setup example category: an invented frame/support fixture with public-domain dimensions and unit-bearing user-entered stiffness values.
- Invalid setup example category: a copied support stiffness table, code-derived default, vendor catalog value without redistribution rights, or commercial software benchmark copied into public tests.

## Conflict Table (for human ruling)

No source conflicts were identified during setup. Open implementation details remain `TBD`, not conflicts.
