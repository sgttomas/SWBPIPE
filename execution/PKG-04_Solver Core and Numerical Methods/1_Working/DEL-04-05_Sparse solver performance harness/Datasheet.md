# Datasheet: DEL-04-05 Sparse solver performance harness

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-04-05 |
| Package ID | PKG-04 |
| Package | Solver Core and Numerical Methods |
| Deliverable type | TEST_SUITE |
| Decomposition basis | `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 |
| Scope items | SOW-035 |
| Objectives | OBJ-003, OBJ-008 |
| Context envelope | M |
| Lifecycle state during setup | Draft setup evidence only; not implementation and not ISSUED |

## Attributes

| Attribute | Setup value |
|---|---|
| Harness purpose | Define a deterministic performance/regression harness for sparse solver behavior on practical piping-model sizes and numerical-conditioning cases. |
| Solver boundary | The harness observes solver behavior; it does not implement solver logic, select a numerical library, or change sparse solve algorithms. |
| Determinism posture | Repeated runs for the same model, units, solver version, and settings must be reproducible enough for regression comparison; exact tolerances remain `TBD`. |
| Performance target posture | Specific runtime, memory, scale, and conditioning thresholds are `TBD` pending solver prototype and human approval. |
| Data posture | Fixtures must use original, invented, public-permissive, or otherwise lawful inputs; no proprietary benchmark data or protected standards examples are introduced. |
| Reporting posture | Results and diagnostics must preserve warnings, assumptions, provenance, solver version, and limitations without claiming certification or code compliance. |

## Conditions

The setup context authorizes only documentation of the future harness shape. It does not authorize benchmark implementation, threshold selection, dependency-version selection, or use of proprietary model data.

The harness must remain compatible with the architecture-basis constraints for module boundaries, result/diagnostic envelopes, layered tests, and protected-content review. Numerical library choice, sparse-solver settings, conditioning metrics, practical model-size bands, timing methodology, hardware normalization, and CI gating thresholds remain `TBD`.

## Construction

| Construction item | Status |
|---|---|
| Performance tests | Anticipated; no tests are implemented in this setup pass. |
| Benchmark harness | Anticipated; module path, command shape, fixture format, and runner integration are `TBD`. |
| Conditioning cases | Required conceptually by SOW-035; concrete matrices/models and acceptable ranges are `TBD`. |
| Regression records | Required conceptually for OBJ-008; retention format and comparison policy are `TBD`. |
| Diagnostics/result-envelope hooks | Required by AB-00-06; exact codes/classes for harness failures are `TBD`. |

## References

- `_CONTEXT.md` for deliverable identity, scope, objectives, anticipated artifacts, and architecture-basis injection.
- `_REFERENCES.md` for governing local references.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4, rows for PKG-04, DEL-04-05, SOW-035, OBJ-003, OBJ-008, AB-00-01, AB-00-02, AB-00-06, and AB-00-08.
- `docs/_Registers/Deliverables.csv` row DEL-04-05.
- `docs/_Registers/ScopeLedger.csv` row SOW-035.
- `docs/_Registers/ContextBudgetQA.csv` row DEL-04-05.
- `docs/CONTRACT.md` invariants OPS-K-SOLVER-1, OPS-K-UNIT-1, OPS-K-MECH-1, OPS-K-REPORT-1, OPS-K-AGENT-1..4, and OPS-K-IP-1.

## Open Setup Questions

| Question | Needed from |
|---|---|
| Which sparse numerical library and solver settings are approved for implementation? | Solver lead / architecture decision |
| What practical model-size bands and conditioning metrics are meaningful for release gates? | Solver lead / validation owner |
| What deterministic timing methodology is acceptable across local and CI environments? | QA/release owner |
| Which invented or public-permissive fixtures may represent practical piping models without protected data? | Validation/IP review owner |
