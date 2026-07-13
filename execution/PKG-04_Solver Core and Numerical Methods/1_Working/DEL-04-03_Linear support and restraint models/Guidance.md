# Guidance: DEL-04-03 Linear support and restraint models

## Purpose

This deliverable documents the implemented linear support/restraint slice of the global solver. It makes boundary conditions explicit enough for deterministic dense-system mechanics preparation and application while staying protected-data safe and outside professional/code-compliance judgment.

## Principles

- Treat supports as mechanics-boundary data in the 3D centerline/frame model, not as local shell/solid FEA features. Source: `docs/CONTRACT.md` OPS-K-MECH-1.
- Keep the solver/rule/human boundary visible. A restraint reaction or displacement result is a mechanics result, not an automatic code-compliance finding. Source: OPS-K-MECH-2.
- Prefer explicit required inputs over defaults. If a support stiffness, direction, coordinate basis, imposed displacement value, or target reference is required and absent, emit a finding. Source: OPS-K-DATA-2.
- Keep every numerical support input unit-aware. Translational stiffness, rotational stiffness, displacement, and rotation units need dimensional checks before solve use. Source: OPS-K-UNIT-1.
- Keep linear and nonlinear scope separate. Linear springs and fixed restraints belong here; activation, lift-off, gaps, one-way supports, and friction are DEL-04-04 unless a later sealed brief changes scope.
- Use original or permissive test fixtures. Do not import protected support catalog values, code examples, vendor tables, or copied commercial benchmark cases. Source: OPS-K-IP-1.
- Keep frame-kernel DOF authority centralized. `FrameDof` is re-exported from `open_pipe_stress_frame_kernel`, and support global DOF indices are computed through `node_dof_index`.

## Considerations

- `core/solver/linear_supports` now represents anchors, guides, line stops, vertical supports, springs, and imposed displacements through explicit node DOFs, restrained DOF lists, spring entries, and imposed displacement entries.
- `SupportQuantity` records value, quantity dimension, optional unit-system reference, and optional unit metadata. It validates finite or positive finite values and rejects unit metadata whose canonical dimension does not match the support quantity. It does not perform unit conversion or provide defaults.
- `prepare_boundary` is the implemented preparation surface. It returns restrained DOFs, spring entries, imposed displacement entries, and findings for missing DOFs, missing stiffness/displacement values, invalid dimensions, repeated restraints, and out-of-range nodes.
- `apply_linear_supports` is the implemented dense-system application surface. It validates through the frame kernel, adds spring stiffness to global stiffness diagonals, and reduces rigid and imposed displacement DOFs through the frame-kernel prescribed-displacement boundary.
- Support coordinate policy remains TBD. The current crate maps explicit node DOFs and does not define arbitrary global, local, or user-defined support direction conventions.
- Imposed displacement boundary data may interact with load-case handling in PKG-05, but this deliverable currently implements the support-side boundary data and frame-kernel prescribed-displacement reduction only.
- Diagnostics should be useful to GUI, CLI, report, and adapter consumers through result-envelope fields required by AB-00-06.
- Final result-envelope integration remains downstream; current `SupportFinding` values are crate-local findings with code, support ID, and message.
- AB-00-01 requires accepted decisions to be recorded when this deliverable resolves deferred choices. This review-readiness alignment does not create repo-level ADR files.

## Trade-offs

| Choice | Benefit | Risk / TBD |
|---|---|---|
| Represent rigid restraints as prescribed zero displacements in dense boundary application | Clear mechanics boundary condition using the frame-kernel prescribed-displacement reducer. | Sparse-solver integration, final result-envelope mapping, and release criteria remain downstream. |
| Represent linear springs as unit-bearing stiffness data | Keeps support behavior testable and explicit. | No default stiffnesses may be invented; source/provenance remains required. |
| Keep nonlinear behavior out of this deliverable | Preserves a bounded linear slice. | Users may expect guides/stops to include gaps or one-way action; that expectation must route to DEL-04-04 or remain TBD. |
| Use invented/public benchmark fixtures | Protects IP boundary. | Validation breadth is limited until lawful engineering examples are reviewed. |

## Examples

- Valid example category: an invented frame/support fixture with public-domain dimensions and unit-bearing user-entered stiffness values.
- Invalid example category: a copied support stiffness table, code-derived default, vendor catalog value without redistribution rights, or commercial software benchmark copied into public tests.

## Conflict Table (for human ruling)

No source conflicts were identified during this alignment. Open policy and integration details remain `TBD`, not conflicts.

## D-41 R5 T7 PDU-054 current declaration

Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The linear-support slice implements the recorded support/restraint families and DEC-049 hanger user data. Its current evidence and residuals are those named by the implemented crate and tests; no review, validation, or lifecycle ruling is made here.
