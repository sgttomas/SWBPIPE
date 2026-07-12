# NOTES — DEL-13-04 Physical-to-analytical transformation contract (R2 wave W5)

Frozen source tree: `main` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
Deliverable: PKG-13 / DEL-13-04, status `IN_PROGRESS`.
Discovery pilot: orchestrator, highest-available GPT-5 capability.

## Histograms

The ledger has 31 rows: REQUIREMENT 12, ACCEPTANCE 9, EXCLUSION 4,
DECLARED_STATE 6. Dispositions: ALIGNED 26, PARTIALLY_IMPLEMENTED 2,
VERIFIED_NOT_VALIDATED 1, STALE_SETUP_SPECIFICATION 2. Authority routing:
NO 25, ENGINEERING 3, OWNER 3.

## Self-flagged rows

- `REQ-004` and `ACC-003` are partial at their exact traceability grain:
  object/DTO traceability is implemented, while field-level scalar and full
  runtime result trace-chain production remain explicitly TBD.
- `REQ-007` is `VERIFIED_NOT_VALIDATED`: the implementation and focused tests
  establish a centerline/frame transform boundary, but do not supply an
  engineering validation basis for broader mechanics suitability.
- `REQ-011` remains substantively aligned. Its rule that a local dependency
  mirror is not dispatch authority survives; the Specification's DAG-006
  wording is an overtaken pointer noted in-row rather than a failed rule.
- `DECL-001` and `DECL-002` are stale at the declared-surface grain because
  Specification and Datasheet explicitly present revision 0.7 as current
  authority while the frozen `SOFTWARE_DECOMP.md` declares revision 0.8 as
  `current_basis_for_control_surface_refresh`.

## Evidence execution

At the frozen SHA, with `PYTHONDONTWRITEBYTECODE=1`, external pycache, and
pytest cache disabled:

- `python3 -m pytest -p no:cacheprovider -q tests/test_physical_to_analytical_transform.py tests/test_analytical_solver_boundary_adapter.py` → 20 passed.
- `python3 tools/validation/validate_dependencies_schema.py <DEL-13-04>/Dependencies.csv` → valid.

Ignored-aware porcelain before and after contained exactly the six addendum-9
allow-listed paths and no seventh path. No cargo or in-tree `py_compile` ran.

## Convention notes and fences

The seeded D-41 item is transcribed only on `DECL-005` and excluded from
residual/selectability analysis. No remaining-work or unmapped row is minted.
All source-reliability values use the weakest load-bearing leg; declaration
prose is `NOT_APPLICABLE`. No SECURITY sufficiency marker is used because no
accepted owner-gated sufficiency deferral is claimed.

Post-fan-in owning-pilot correction: the independent PKG-13 verifier identified
the two explicit revision-pointer declarations above; this pilot rechecked the
frozen sources and applied the bounded disposition/evidence/routing correction.

Discovery was read-only outside these two run artifacts. No lifecycle, status,
product, DAG, register, dependency, R4, or R5 change was made. No release,
professional, certification, sealing, approval, or code-compliance claim is
made; dispositions are agent judgments.
