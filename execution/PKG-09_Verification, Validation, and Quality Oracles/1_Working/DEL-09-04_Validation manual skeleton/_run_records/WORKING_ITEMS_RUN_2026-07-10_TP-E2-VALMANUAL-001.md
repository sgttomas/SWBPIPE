# WORKING_ITEMS_RUN - TP-E2-VALMANUAL-001 Per-Case Validation Manual

**Date:** 2026-07-10
**Package:** PKG-09 - Verification, Validation, and Quality Oracles
**Primary deliverable:** DEL-09-04 - Validation manual skeleton
**Supporting deliverables:** DEL-09-01, DEL-09-02, DEL-09-03 (evidence
sources), DEL-10-05 (headless runner reproduction path)
**Decision basis:** `DEC-054` (carries PRD §16.2/§16.5 validation-manual work
forward as planned Phase E evidence-system work); tolerance/threshold
conventions per `DEC-024`/`DEC-026`/`DEC-046`; runner surface per `DEC-065`
**Phase basis:** `plans/PLAN_2026-06-17_prd_completion.md` Phase E row E2
**Branch:** `claude/piping-e2-validation-manual` from head `3752891a3`

## Scope

Author the per-case validation manual for Phase E row E2: one manual case
page per suitable existing validated case, assembled from EXISTING witnesses
and fixtures in the live tree, with reproduction exercised through the landed
E1 surfaces, without changing solver or benchmark behavior, without settling
any threshold, and without release/professional claims.

## Deliverable Choice And Basis

The validation manual deliverable is `DEL-09-04 - Validation manual skeleton`
(PKG-09). Basis: `docs/validation_manual/index.md` frontmatter declares
`implements: DEL-09-04`; the prior E2 run record
`WORKING_ITEMS_RUN_2026-07-05_TP-VALIDMANUAL-RUNNERREPRO-001.md` in this same
`_run_records/` directory carried the previous E2 slice under DEL-09-04; the
deliverable `_STATUS.md` is `IN_PROGRESS` (2026-07-02 affirmation).

## Implemented

- Added `docs/validation_manual/cases/generate_validation_case_pages.py`
  (committed deterministic generator; hand-calc notes, fixture constructors,
  suite tests, and governed DEC-046 records remain the sources of record).
- Generated and committed 63 case pages:
  - `docs/validation_manual/cases/mechanics/` — 20 cases (all
    `fixture_inventory()` fixtures of `DEL-09-01`, including
    `MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL`);
  - `docs/validation_manual/cases/stress/` — 15 cases (all `DEL-09-02`
    fixtures);
  - `docs/validation_manual/cases/nonlinear/` — 28 cases (5 classifier,
    9 assembled seed incl. the three `DEC-067` transition cases, 1
    observation-only depth case, 13 accepted multi-support cases of
    `DEL-09-03`).
- Each page records the PRD §16.5 fields: test purpose (extracted from the
  hand-calc note `## Purpose` where present), input model (fixture
  constructor + reference note), independent reference, expected result,
  software result and exact reproduction command, tolerance basis, pass/fail,
  and solver version.
- Rebuilt `docs/validation_manual/index.md` as the case index (sections 3.1
  to 3.4) and corrected the verified-stale authority pointer
  (`SOFTWARE_DECOMP.md` revision `0.7` -> `0.8`; `DAG-007` pointer verified
  current against `execution/_DAG/_LATEST.md`).
- Updated `docs/validation_manual/headless_runner_reproduction.md`: recorded
  the 2026-07-10 reproduction re-run and its measured delta, and re-pointed
  the Remaining E2 Work section at the new case pages.
- Corrected a verified-stale sentence in
  `validation/benchmarks/mechanics/README.md` (the expansion-loop fixture IS
  registered in `fixture_inventory()` and the hand-calc README inventory
  lists its note; the README previously said it was not yet registered).
- Updated deliverable `MEMORY.md`.

## Evidence Commands And Recorded Outputs

All commands run from `projects/chirality-piping` at branch head (parent
`3752891a3`), toolchain rustc 1.92.0 / cargo 1.92.0, on 2026-07-10.

Benchmark suite reproduction (real runs):

```bash
cargo test --manifest-path validation/benchmarks/mechanics/Cargo.toml
# test result: ok. 30 passed; 0 failed; 0 ignored (re-run after README fix: identical)
cargo test --manifest-path validation/benchmarks/stress/Cargo.toml
# test result: ok. 22 passed; 0 failed; 0 ignored
cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml
# test result: ok. 19 passed; 0 failed; 0 ignored
```

Headless runner reproduction (real runs; scratch outputs, no witness files
overwritten):

```bash
python3 validation/witness/inputs/generate_tp_runner_015_inputs.py
# regenerated inputs byte-identical to committed fixtures (git status clean)
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- solve --input validation/witness/inputs/tp_runner_015_final_cli_solve_input.json --output <scratch>/tp_runner_015_solve.json
# exit 0; job state COMPLETED; request/runner diagnostics empty; result_refs 830
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- validate-input --input validation/witness/inputs/tp_runner_015_final_cli_validation_blocking_input.json --output <scratch>/tp_runner_015_validation_blocking.json
# exit 1; request diagnostic HEADLESS_RUNNER_LOAD_BASIS_MISSING; no solver result
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- run-benchmark --input validation/witness/inputs/tp_runner_015_final_cli_benchmark_stub_input.json --output <scratch>/tp_runner_015_benchmark_stub.json
# exit 1; diagnostic HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD
```

Generator determinism:

```bash
python3 docs/validation_manual/cases/generate_validation_case_pages.py
# generated 63 case pages; rerun reproduces committed pages byte-for-byte
```

Repo-wide check (from repo root):

```bash
PYTHONDONTWRITEBYTECODE=1 python3 tools/practitioner_harness/harness.py self-check
# exit 0 (pre-existing REVIEW/WARN findings unrelated to this tranche)
```

DEC-025 five-surface evidence sweep at the committed clean head: executed
after the docs/evidence commit `dfa5dd344`. Retry chain (all artifacts
committed under `validation/evidence/sweeps/`):

1. `SWEEP_20260711T012843Z_dfa5dd344429.json` — overall `fail` at
   `desktop_vitest` (`vitest: command not found`): the fresh agent worktree
   had no installed node dev dependencies. Environment provisioning issue,
   not a code or evidence failure.
2. Provisioned dev dependencies with `npm ci` (lockfile unchanged; no
   tracked-file delta), re-ran: `SWEEP_20260711T013150Z_dfa5dd344429-dirty.json`
   — overall `pass`, marked dirty only because artifact 1 was untracked in
   the working tree at sweep time.
3. Final clean-head sweep after committing the chain artifacts — overall
   `pass`; artifact name recorded in the sweep-summary commit on this branch.

## Deltas Found (live tree vs recorded maps)

- `openpipestress-runner solve` on the invented preview model now reports
  830 `result_refs`; the committed `TP-RUNNER-015` generated witness and run
  record say 822. Exit codes, diagnostic codes, and all slice expectations
  are unchanged. Refreshing the committed generated witness outputs is
  bounded `DEL-10-05` work, not done here.
- `validation/benchmarks/mechanics/README.md` claimed the expansion-loop
  fixture was not yet registered in `fixture_inventory()`; the live crate
  registers it and the hand-calc README lists its note. Corrected.
- `docs/validation_manual/index.md` cited decomposition revision `0.7`; the
  live `SOFTWARE_DECOMP.md` is revision `0.8`. Corrected.

## Residuals Left In The E2 Row

- Final public-benchmark tolerances ride the convergence/threshold policy:
  the `DEC-024`/`DEC-026` per-quantity-kind governed verification tolerance
  record remains unfilled (`TBD`); `DEC-046` `TBD` entries stay `TBD`.
- `openpipestress-runner` benchmark/regression payload bindings remain
  structured stubs (`DEL-10-05`), so per-case reproduction runs through the
  suite tests rather than the runner.
- Clean-environment external-reproducibility demonstration record (Phase E
  exit evidence) not claimed here.
- `MAINTAINER_REVIEWED` promotion of any case page, GUI workflow validation
  evidence, and evidence-bundle storage policy remain open.
- Human-gated release-quality and professional-boundary reviews at the R5
  exit remain open.

## Boundaries Preserved

- No threshold invented, tightened beyond evidence, or loosened; every
  tolerance statement in the case pages transcribes existing fixture-local
  constants or governed records; `TBD` entries stay `TBD`.
- No protected standards content, catalog values, SIF/flexibility equations,
  or defaults; all case data is invented public-original fixture data.
- No solver, benchmark-crate, or runner behavior change; no witness or
  observation artifact overwritten.
- No DEL-09-04 lifecycle transition; no release-readiness, release
  packaging/signing/publication, hosted CI, public transport,
  protected-content/legal clearance, professional approval, certification,
  sealing, authentication, or code-compliance claim.
- No edit to `plans/PLAN_COMPLETION_LOG.md` (orchestrator lands the entry).
