# Datasheet: DEL-04-06 Solver diagnostics and singularity detection

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-04-06 |
| Name | Solver diagnostics and singularity detection |
| Package | PKG-04 Solver Core and Numerical Methods |
| Type | BACKEND_FEATURE_SLICE |
| Scope items | SOW-053, SOW-035 |
| Objectives | OBJ-003, OBJ-008, OBJ-012 |
| Context envelope | M |

## Attributes

| Attribute | Setup value | Source |
|---|---|---|
| Primary subject | Deterministic diagnostics for singular, ill-conditioned, nonconverged, and invalid-restraint states. | `_CONTEXT.md`; `docs/_Decomposition/SOFTWARE_DECOMP.md` row DEL-04-06 |
| Solver-result boundary | Diagnostics describe mechanics-solver status and do not decide code compliance. | `docs/CONTRACT.md` OPS-K-MECH-2; OPS-K-AUTH-1 |
| Diagnostic envelope fields | Code, class, severity, source, affected object, message, remediation, and provenance. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06 |
| Warning classes | `SOLVE_BLOCKING`, `NONLINEAR_WARNING`, and other shared classes as applicable to solver status reporting. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06; `docs/SPEC.md` section 7 |
| Numerical thresholds | TBD. Do not invent singularity, conditioning, or convergence thresholds during setup. | `docs/_Decomposition/SOFTWARE_DECOMP.md` OI-005 |
| Sparse solver library/settings | TBD. | `_CONTEXT.md` Still TBD |

## Diagnostic Category Register (P3 Candidate Worklist Applied)

| Category | Setup meaning | Open item |
|---|---|---|
| Singular system | Solver cannot obtain a determinate mechanics solution for the constrained system. | Detection method and threshold TBD. |
| Ill-conditioned system | Solver can proceed far enough to identify numerical conditioning concern. | Conditioning metric and threshold TBD. |
| Nonconverged analysis | Iterative solve does not reach accepted convergence criteria. | Convergence criteria TBD. |
| Invalid restraint | Model restraint definition blocks a valid solve before or during assembly/solution. | Exact validation taxonomy TBD. |
| Solver status | Machine-readable summary state carried with diagnostics in result envelopes. | Final enum/schema TBD. |

## Conditions

- Solver changes require deterministic verification tests before release. Source: `docs/CONTRACT.md` OPS-K-SOLVER-1.
- Nonlinear solver diagnostics must report convergence, active-set state, and unresolved non-convergence when nonlinear support behavior is involved. Source: `docs/CONTRACT.md` OPS-K-SOLVER-2.
- Missing solve-required values and invalid restraints must become explicit findings rather than silent defaults. Source: `docs/CONTRACT.md` OPS-K-DATA-2.
- Units in solver-facing values and outputs remain dimensionally checked. Source: `docs/CONTRACT.md` OPS-K-UNIT-1.

## Construction

| Artifact | Description | Setup status |
|---|---|---|
| Solver diagnostics module | Backend module or equivalent service surface for solver diagnostic classification and status reporting. | TBD implementation path |
| Singular-model tests | Deterministic tests for singular and invalidly restrained models. | Required in principle; fixtures TBD |
| Conditioning warnings | Reportable warnings for ill-conditioning or sparse-solver numerical quality. | Warning criteria TBD |
| Status reporting | Machine-readable result status compatible with command/query/job result envelopes. | Governed by AB-00-03 and AB-00-06 |

## References

- `_CONTEXT.md`
- `_REFERENCES.md`
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4
- `docs/_Registers/Deliverables.csv` row DEL-04-06
- `docs/_Registers/ScopeLedger.csv` rows SOW-053 and SOW-035
- `docs/CONTRACT.md`
- `docs/SPEC.md`
- `docs/TYPES.md`
