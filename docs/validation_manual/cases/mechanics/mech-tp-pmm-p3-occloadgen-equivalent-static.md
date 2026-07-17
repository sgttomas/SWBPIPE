---
doc_id: OPS-VALIDATION-MANUAL-CASE-MECH-TP-PMM-P3-OCCLOADGEN-EQUIVALENT-STATIC
doc_kind: governance.validation_manual_case
status: draft_evidence
created: 2026-07-10
generated_by: docs/validation_manual/cases/generate_validation_case_pages.py
refs:
  - rel: implements
    to: DEL-09-04
  - rel: evidences
    to: DEL-09-01
---

# MECH-TP-PMM-P3-OCCLOADGEN-EQUIVALENT-STATIC

| Item | Record |
|---|---|
| Suite | Mechanics benchmark suite (`DEL-09-01`), crate `validation/benchmarks/mechanics/` |
| Evidence class | Mechanics verification |
| Evidence state | `DRAFT_EVIDENCE` |
| Independent reference | `validation/hand_calcs/mechanics/tp_pmm_p3_occloadgen_equivalent_static.md` |
| Provenance | Invented public-original fixture data only (recorded in the reference note and fixture metadata) |

## Test Purpose And Problem Statement

Invented mechanics benchmark for static-equivalent occasional-load
generation (tranche `TP-PMM-P3-OCCLOADGEN-001`, ruling `DEC-068` item 2).
It verifies that:

1. Seismic: user-entered per-global-axis g-factors and a user-entered
   gravity acceleration multiply the model's own computed mass
   distribution (metal + contents + insulation over the mill-tolerance
   effective wall) into uniform distributed load intensities.
2. Wind: user-entered pressure and shape parameters project onto the
   exposed diameter (outside diameter plus twice the insulation
   thickness) of user-marked spans only.
3. The generated primitive loads pass the equivalent-static boundary
   helper and lump 50/50 to span end nodes exactly as authored loads do.

Pure mechanics from user inputs: no code coefficient, exposure category,
importance factor, catalog value, or default is encoded. The gravity
acceleration is an explicit user-entered input in this fixture, not an
embedded physical constant.

All inputs are invented or user-entered fixture data; nothing is copied from
protected standards, commercial software examples, or proprietary data.

## Input Model

The machine-readable input model is the fixture constructor `tp_pmm_p3_occloadgen_equivalent_static_fixture()` (fixture id `MECH-TP-PMM-P3-OCCLOADGEN-EQUIVALENT-STATIC`) in `validation/benchmarks/mechanics/src/lib.rs`, carrying explicit fixture-local units and public-original provenance metadata. The reference note records the same invented inputs with the longhand derivation.

## Expected Result And Independent Reference

The expected values and their derivation are recorded in the independent
reference note `validation/hand_calcs/mechanics/tp_pmm_p3_occloadgen_equivalent_static.md` and mirrored in the fixture's expected-value
slots. The reference derivation uses elementary open mechanics only.

## Software Result And Reproduction

The measured-vs-reference comparison executes inside the named suite
test(s) `occloadgen_generation_matches_witness_intensities`, `occloadgen_generated_loads_pass_boundary_and_lump_to_end_nodes`, `occloadgen_generation_blocks_without_user_inputs_or_marked_spans`, which run the current in-repo solver path on the fixture
and assert agreement with the reference expectations.

Reproduction (from `projects/chirality-piping`):

```bash
cargo test --manifest-path validation/benchmarks/mechanics/Cargo.toml occloadgen_generation_matches_witness_intensities
cargo test --manifest-path validation/benchmarks/mechanics/Cargo.toml occloadgen_generated_loads_pass_boundary_and_lump_to_end_nodes
cargo test --manifest-path validation/benchmarks/mechanics/Cargo.toml occloadgen_generation_blocks_without_user_inputs_or_marked_spans
```

Recorded run: 2026-07-10, toolchain rustc 1.92.0 / cargo 1.92.0; suite
result `ok. 30 passed; 0 failed; 0 ignored` with the named test(s) passing. The full
suite output and the exact commit are recorded in the run record
`execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_run_records/WORKING_ITEMS_RUN_2026-07-10_TP-E2-VALMANUAL-001.md`.

Headless-runner binding: the `DEC-065` `openpipestress-runner run-benchmark`
verb accepts a schema-first benchmark request but currently exits `1` with
the structured blocking diagnostic
`HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD` because
benchmark payload binding remains future bounded `DEL-10-05` work. Until that
binding lands, the exact reproduction command for this case is the crate test
above; the runner-reproducible external path is recorded in
[headless_runner_reproduction.md](../../headless_runner_reproduction.md).

## Tolerance

Numeric agreement assertions in this suite use the fixture-local absolute assertion epsilon `INTERNAL_ASSERTION_EPSILON = 1.0e-9` recorded in `validation/benchmarks/mechanics/src/lib.rs`; structural and deterministic expectations (prepared boundaries, diagnostics, routing, provenance) are asserted exactly. This matches the `DEC-026` analytic-class relative seed magnitude but the fixture's `tolerance_policy` slots remain unresolved: the governed per-quantity-kind tolerance record required by `DEC-024`/`DEC-026` remains `TBD`, and this page neither tightens nor loosens any governed value.

## Pass/Fail

`PASS` at the recorded run above: every named test asserted the measured-vs-reference expectations and completed with `ok`. This is software-quality evidence only.

## Solver Version

The in-repo OpenPipeStress solver and benchmark crates at the commit recorded
in `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_run_records/WORKING_ITEMS_RUN_2026-07-10_TP-E2-VALMANUAL-001.md`; toolchain rustc 1.92.0 / cargo 1.92.0. No packaged release version
exists; release labels remain `TBD`.

## Boundary

This case page documents software verification evidence for an invented
public-original fixture. It is development verification and screening
evidence (claims registry BS-VALID, DEC-081), and it does not settle any
`TBD` threshold or governed tolerance decision.
