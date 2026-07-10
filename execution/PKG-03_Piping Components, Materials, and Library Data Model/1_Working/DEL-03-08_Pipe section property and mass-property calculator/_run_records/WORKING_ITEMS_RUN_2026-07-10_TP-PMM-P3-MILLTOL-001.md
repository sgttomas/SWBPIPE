# WORKING_ITEMS Run Record - TP-PMM-P3-MILLTOL-001

Date: 2026-07-10
Agent: WORKING_ITEMS (bounded Type-2 implementation worker; owner-directed session)
Deliverable: DEL-03-08 - Pipe section property and mass-property calculator
Package: PKG-03 - Piping Components, Materials, and Library Data Model
Tranche: TP-PMM-P3-MILLTOL-001
Target stage: R5 / physical-model mechanics program P3 tranche (c)
(`plans/PLAN_2026-07-09_physical_model_mechanics.md`; ruling `DEC-068`,
D-36 item 3)

## Scope

First bounded slice of the D-36 P3 workflow-physics set: a user-entered
mill-tolerance dimension slot in `schemas/section.schema.yaml`, consumed by
the effective-wall calculation in `core/section_properties/calculator.py`
alongside corrosion allowance, with provenance, plus the downstream
mill-tolerance reduction in the `core/product_physics` derived section that
tranche (b) consumes as its mass basis.

## Files Touched

- `schemas/section.schema.yaml` (`SectionDimensionKind` gains
  `mill_tolerance`; `SectionDimensionSlot` carries value/provenance/
  review_status unchanged)
- `core/section_properties/calculator.py` (`PipeSectionInput.mill_tolerance`
  optional slot; effective wall subtracts the user-entered absolute mill
  tolerance alongside corrosion allowance; optional-field validation
  exemption sets joined; provenance stamped as for other calculated slots)
- `core/product_physics/src/lib.rs` (`PipeSectionInput.mill_tolerance`
  optional preview field; DEC-018 unit normalization; `derive_pipe_section`
  effective-wall reduction with blocking diagnostic on
  present-but-invalid slot)
- `tests/test_section_properties.py`, `tests/test_component_section_schema.py`
- `validation/hand_calcs/stress/tp_pmm_p3_milltol_effective_wall_stress.md`
  (witness) and `validation/hand_calcs/stress/README.md` (inventory row)
- `validation/benchmarks/stress/{src/lib.rs, README.md}` (fixture
  `STRESS-TP-PMM-P3-MILLTOL-EFFECTIVE-WALL-STRESS`, family registration,
  inventory count, tests)

## Recorded Design Choice

Mill tolerance is stored and consumed as a **user-entered absolute
thickness dimension (length)**, consistent with `SectionDimensionKind`
carrying dimensions only. No fractional (percentage-of-wall) form, catalog
value, or default is encoded; if a fractional convention is wanted later it
is a separate decision. Absence of the optional slot means no reduction —
absence is not a default value of zero. A present-but-invalid slot
(negative, non-finite, or consuming the wall together with corrosion
allowance) is a blocking diagnostic (PRD section 6.2).

## Implemented Evidence

- Effective wall in the Python calculator:
  `t_eff = wall - corrosion_allowance - mill_tolerance`, with the combined
  non-positive-effective-wall blocking diagnostic naming both slots. The
  mass basis (metal/contents areas) flows from the same effective wall, so
  tranche (b) consumes the mill-tolerance effect.
- Preview mechanics: `derive_pipe_section` applies the same reduction to
  area, second moment, torsion constant, section modulus, and internal
  area; `mm`-entered mill tolerance normalizes to the SI solver boundary
  identically to other length inputs (test-proven envelope equality).
- Witness `STRESS-TP-PMM-P3-MILLTOL-EFFECTIVE-WALL-STRESS` derives the
  effective-wall section closed forms and expected mechanics-only stress
  components; the benchmark fixture reproduces them at the crate-internal
  1.0e-9 assertion epsilon and asserts the mill-tolerance reduction
  strictly reduces section modulus versus the corrosion-only wall.
- Tolerance posture per `DEC-024`/`DEC-026`: governed values remain `TBD`;
  no fixture-local `tolerance_policy` override recorded; overrides may only
  tighten.

## Checks

- `python3 -m pytest -q tests`: 374 passed (369 pre-existing + 5 new
  mill-tolerance tests).
- `cargo test` `core/product_physics`: 60 passed (56 pre-existing + 4 new).
- `cargo test` `validation/benchmarks/stress`: 21 passed (19 pre-existing
  + 2 new).
- `cargo fmt --check` clean on touched crates.
- Repo-wide harness `self-check` exit 0.
- `DEC-025` five-surface sweep recorded on the clean head with the final
  tranche of this run (commit-bound summary under
  `validation/evidence/sweeps/`).

## Boundaries And Residuals

- Schema files carry no literal schema-version constant to bump;
  `schema_version` is instance data (fixtures remain `0.1.0`). Recorded as
  an instructions-vs-live-tree delta; no version surface was invented.
- The model-persistence surfaces do not enumerate section dimension kinds;
  no persistence change was required.
- No mill-tolerance catalog fraction, code-content value, or default
  ships; all values user-entered; unknowns remain `TBD`.
- No lifecycle transition, no release-readiness, professional,
  certification, or code-compliance claim.
