# WORKING ITEMS RUN - TP-R4-D9-MULTISUPPORT4CLASS-001

Date: 2026-06-22
Agent: WORKING_ITEMS
Package: PKG-09 - Verification, Validation, and Quality Oracles
Deliverable: DEL-09-03 - Nonlinear support regression suite
Target stage: R4 / Phase D

## Objective

Broaden the accepted multi-support nonlinear validation companion evidence by
adding a tenth public-original fixture that combines all four current nonlinear
support classes in one assembled solve: friction sliding, gap closure,
one-way release, and rotational lift-off release under the existing narrow
`DEC-046` multi-support active-set, free-DOF force/moment, free-DOF work, and
displacement/reaction-delta policies.

## Authority Basis

- PRD section 22.5: R4 requires nonlinear support validation cases to converge.
- `DEC-044`: the assembled nonlinear solve is owned by the PKG-04 integration
  tranche; the nonlinear support classifier remains the per-iteration state
  oracle.
- `DEC-046`: class-tiered convergence policy can promote measured entries, but
  unmeasured/non-seed entries remain `TBD`.
- Prior accepted multi-support companions:
  `TP-R4-D9-MULTISUPPORTPOLICY-001`,
  `TP-R4-D9-MULTISUPPORTBREADTH-001`,
  `TP-R4-D9-MULTISUPPORTFRICTION-001`,
  `TP-R4-D9-MULTISUPPORT3DOF-001`,
  `TP-R4-D9-MULTISUPPORTROT-001`,
  `TP-R4-D9-MULTISUPPORTDERIVED-001`,
  `TP-R4-D9-MULTISUPPORTDERIVEDROT-001`,
  `TP-R4-D9-MULTISUPPORTCASCADE-001`, and
  `TP-R4-D9-MULTISUPPORTNEGAP-001`.

## Changes

- Added `NL-ASSEMBLED-MULTI-DOF-FOUR-CLASS-ACCEPTED-ORIGINAL`, an invented
  two-node assembled frame fixture with free tip `Ux`, `Uy`, `Uz`, and `Rz`
  behavior.
- The fixture applies Ux force for friction sliding, Uy force for positive gap
  closure, Uz force for one-way release, and Rz moment for rotational lift-off
  release. The first iteration records four changed support states and the
  second iteration converges.
- Extended `assembled_multisupport_acceptance_inventory()` and corresponding
  convergence/residual observations to include ten accepted multi-support
  companion fixtures.
- Updated multi-support active-set-count, free-DOF force/moment, free-DOF
  work, and displacement/reaction-delta policy JSON records so evidence
  fixture IDs name the ten-fixture public-original set.
- Added the hand-calculation/provenance note
  `validation/hand_calcs/nonlinear/assembled_multi_support_four_class_acceptance.md`
  and updated hand-calc/benchmark indexes, focused regression guardrails,
  coordination text, plan surfaces, the R4 gap packet, completion log, and
  DEL-09-03 memory.

## Validation

Focused validation:

- `cargo fmt --manifest-path validation/benchmarks/nonlinear/Cargo.toml --check`
  - passed.
- `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml`
  - passed, 17 tests.
- `python3 -m pytest -q tests/test_nonlinear_support_regression.py` - passed,
  8 tests.
- `python3 -m json.tool` over the four updated multi-support DEC-046 JSON
  records - passed.
- `git diff --check` - passed.

Full DEC-025 evidence sweep:

- `python3 tools/release/run_evidence_sweep.py --execute` - passed all five
  surfaces:
  - cargo crate sweep;
  - repository pytest, 363 tests;
  - desktop Vitest, 407 tests;
  - Playwright dev/dist desktop lanes, 18 + 1 tests;
  - desktop production build.
- Passing sweep summary:
  `validation/evidence/sweeps/SWEEP_20260622T130443Z_2ead1626d231-dirty.json`

## Boundaries

- This tranche broadens only the accepted public-original multi-support
  companion fixture set from nine fixtures to ten fixtures.
- It does not promote non-seed force/displacement thresholds beyond the
  accepted ten-fixture set, broader displacement-delta or reaction-delta
  thresholds, general energy thresholds, sparse-default behavior,
  product-preview release thresholds, external validation thresholds, CI
  thresholds, or R4 exit readiness.
- It does not add protected standards content, private project data, hidden
  support defaults, lifecycle transition, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim.

## Residuals

- Remaining D6/D9 work includes default sparse promotion, large-model sparse
  suitability thresholds, non-seed force/displacement threshold promotion
  beyond the accepted ten-fixture set, general energy threshold promotion,
  broader displacement/reaction-delta thresholds beyond the current seed,
  ten-fixture, and product-preview surfaces, deeper spring-hanger behavior,
  external validation threshold evidence, broader R4 validation packaging, and
  final R4 exit-chain evidence.
