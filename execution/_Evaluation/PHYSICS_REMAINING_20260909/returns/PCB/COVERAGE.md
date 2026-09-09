# PCB coverage

## Assessed surfaces

| Area | Current surfaces inspected | Disposition |
|---|---|---|
| Pressure assembly and recovery | `core/product_physics/src/lib.rs`; `core/loads/stress_recovery/src/lib.rs`; `core/solver/straight_pipe/src/lib.rs`; focused pressure tests | Reproduced endpoint action/cut defect; selected exact straight-pipe reference and versioned output meanings. |
| Pressure candidate evidence | 2026-09-07 E1 pressure brief; 2026-09-08 DEL-05-03 four-case investigation and its source manifest | Treated as candidate evidence, independently checked against current source and primary references, then adopted here as a recommendation under the V2 authority amendment. It remains non-authoritative until the owning workflow adopts it. |
| Expansion-joint connector | `frame_kernel::UserStiffnessElement`; product expansion-joint mapper; E1 connector brief; current M9 return | Reproduced finite-end rigid-rotation failure; selected an exact small-displacement two-frame energy formulation and schema requirements. |
| Physical-to-analytical transform | `contract.py`; canonical fixture; focused unsupported-component mutation; DEL-13-04 SOW/status | Confirmed deterministic diagnostics and blocking omission for expansion joints. No silent transform defect reproduced. |
| Solver-boundary adapter | `_solver_boundary_adapter.py`; focused adapter test; non-test use search | Confirmed 0.1 straight-pipe/basic-load boundary and no product-runtime consumer. |
| PKG-09 verification | physics-audit regression README; pressure investigation expectations; M9 scope | Identified missing pressure, connector constitutive, bridge, and full mixed-reference acceptance coverage. |
| Primary references | MIT OCW exact/thick cylinder note; Abaqus pipe-pressure, beam section-force, and connector documentation | Used for engineering selection and terminology, accessed 2026-09-09. No protected piping-design extraction was used. |

## Focused reproductions

1. Pressure sign arithmetic with the existing invented annulus: current raw pair `[+P,-P]` yields endpoint stresses `+5.26086956522/-5.26086956522 MPa`, while the station cut is `-5.26086956522 MPa`.
2. Connector rigid rotation: `L=2 m`, `k_lat=2000 N/m`, `epsilon=0.001` yields false raw deformation `0.002 m`, force `4 N`, energy `0.004 J`, and couple `8 N*m`; the selected objective deformation is exactly zero.
3. Canonical physical fixture mutated to an expansion-joint `component_link`: no analytical component or element; diagnostics `PTA-COMPONENT-TYPE-UNSUPPORTED`, `PTA-ELEMENT-COMPONENT-UNSUPPORTED`, `PTA-ELEMENT-TYPE-UNSUPPORTED`, and two unresolved load-target diagnostics; `has_blocking_findings=true`.
4. `tests/test_analytical_solver_boundary_adapter.py` passed under system Python. `tests/test_physical_to_analytical_transform.py` stopped because the environment lacks `jsonschema>=4,<5`.

## Exclusions

- No source, test, schema, app, deliverable, decision, dependency, DAG, register, receipt, status, pointer, or Git edit.
- No blanket test suite, Cargo build, shared target directory, GUI launch, or heavy solver run.
- No pressure gradient, end-discontinuity, bend ovalization, pressure stiffening, plasticity, large displacement, nonlinear/manufacturer connector, tie-rod, hinge, gimbal, pressure-vessel code, or allowable-stress assessment.
- No industry-standard solver comparison or third-party validation.
- No extracted equation under `domains/piping-design/` was used as authority.

## Remaining unknowns and retained boundaries

- Curved endpoint stress needs a separately proved chord-action to tangent-section transform. The straight sign rule cannot be copied without rotation.
- Extending generalized-plane-strain pressure coupling to curved pipes and combining expansion-joint effective area with wall force require separate free-body and constitutive proofs.
- Manufacturer connector stiffness may be anisotropic, coupled, temperature dependent, or defined between different reference planes. The selected schema represents these meanings but cannot infer absent data.
- A large-rotation connector requires a finite-rotation corotational model; the selected operator is intentionally small-displacement.
- Exact public schema version numbers, deprecation duration, and migration release timing remain for the owning product/governance workflow. The technical rule is new-writer/new-version, explicit legacy replay, and fail-closed unknown-version handling.
- Full transform-test status remains unknown until the missing development dependency is supplied. The focused transform reproduction does not replace that suite.
- Acceptance tolerances remain unset. The proposed oracles use exact identities where possible and retain raw residuals for later policy.
