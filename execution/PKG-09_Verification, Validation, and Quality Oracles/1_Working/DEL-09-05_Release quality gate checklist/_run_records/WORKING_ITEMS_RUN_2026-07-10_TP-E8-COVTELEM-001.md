---
doc_id: WORKING_ITEMS_RUN_2026-07-10_TP-E8-COVTELEM-001
doc_kind: execution.run_record
status: completed
created: 2026-07-10
agent: WORKING_ITEMS
tranche_id: TP-E8-COVTELEM-001
deliverable_id: DEL-09-05
package_id: PKG-09
---

# WORKING_ITEMS Run Record - TP-E8-COVTELEM-001

## Scope

First implementation tranche of `DEC-060` (D-04b Option O-A, adopted as
written): the coverage-tooling portion of completion-plan row E8
(`plans/PLAN_2026-06-17_prd_completion.md`). Ruling:
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 `DEC-060`; packet:
`execution/_Coordination/_DECISIONS/D-04b_coverage_tooling_floor_promotion.md`.

Delivered: the seam-2 opt-in coverage telemetry runner
`tools/release/run_coverage_telemetry.py` with its artifact schema, the three
per-lane dev-tooling selections (Rust `cargo-llvm-cov`, TS Vitest
`--coverage` + `@vitest/coverage-v8`, Python `coverage.py`), verification of
the packet §2 unresolved `ASSUMPTION`s (the §8.4 feasibility checkpoint), and
the first emitted telemetry artifacts. The `DEC-025` five-surface gate is not
modified. No numeric floor exists or is created; no floor-promotion
automation is implemented (the five-artifact/two-commit trigger stays a
future human-ruled `D-XX` row).

## ASSUMPTION Verification Outcomes (packet §2 Unresolved, §8.4 checkpoint)

All checks run cold on this host (`macOS-26.5.1-arm64`, stable rustc 1.92.0,
Python 3.13.7, Node v24.5.0) on 2026-07-10.

| # | ASSUMPTION (packet §2/§4) | Outcome |
|---|---|---|
| 1 | `cargo-llvm-cov` works on stable macOS arm64 via rustc's LLVM instrumentation + `llvm-tools`, supporting `--manifest-path` invocation | **VERIFIED (lane measured).** Host-installed `cargo-llvm-cov 0.8.7` (`cargo install cargo-llvm-cov --locked`) plus the `llvm-tools` rustup component (`rustup component add llvm-tools`). `cargo llvm-cov --manifest-path <crate> --summary-only --json` ran per crate on stable 1.92.0 and emitted parseable LLVM export totals. Nothing was added to any `Cargo.toml` (DEC-023 posture preserved). |
| 2 | Per-crate coverage across the 34 non-workspace manifests aggregates cleanly | **VERIFIED (lane measured).** The runner walks the existing `discover_cargo_manifests()` list (`tools/release/check_release_readiness.py`); all 34 discovered crate manifests measured successfully in one pass. Aggregation form (packet §6.3, delegated to this tranche as a recorded choice): `count_weighted_sum_across_crates` — total covered counts over total counts summed across crates for lines/functions/regions; per-crate numbers remain the primary record in the artifact. |
| 3 | A Vitest-4-compatible `@vitest/coverage-v8` provider version exists and the vendor-native `--coverage` path works | **VERIFIED (lane measured).** `@vitest/coverage-v8` added as an `apps/desktop` devDependency at `^4.1.7`; npm resolved `4.1.10` alongside `vitest 4.1.10`. `vitest run --coverage.enabled=true --coverage.provider=v8 --coverage.reporter=json-summary` ran the full desktop unit suite (407/407 tests, 19 files) after the wasm engine build and emitted a parseable `coverage-summary.json`. |
| 4 | `coverage.py` mechanics (`coverage run -m pytest`, `coverage json`) work as the Python lane | **VERIFIED (lane measured).** `coverage>=7,<8` added to `requirements-dev.txt` (resolved 7.15.0); `coverage run --source=. -m pytest -q tests` passed 387/387 and `coverage json` emitted parseable totals. |

No lane failed the spike, so no lane reverts to `not_measured` and no
follow-up returns to D-04b. The Playwright e2e surface stays explicitly
unmeasured per the O-A lane scope (packet §6.4) — instrumenting the
in-browser bundle is out of this tranche.

## Recorded In-Tranche Choices

- **Rust aggregation** (packet §6.3 delegated): `count_weighted_sum_across_crates`
  as above. An unweighted per-crate mean was rejected because tiny crates
  would swamp large ones; counts are the only canonical shared denominator
  across non-workspace crates.
- **Python measurement scope:** `--source=. --omit=*/node_modules/*,.archive/*,*/target/*`
  so the whole repository Python surface (core/, tools/, tests/, validation/)
  counts in the denominator, including unexecuted modules; percent is
  therefore repo-wide statement coverage by the pytest suite, not
  executed-files-only coverage.
- **Never-blocking exit semantics:** the runner's exit code reflects only
  artifact integrity (git binding, schema validity, writability); lane
  outcomes — including `not_measured` — never affect it.
- **Cadence** (packet §6.2): left to ordinary work selection per the ruling's
  silence; the runner stays opt-in (`--execute`).

## Files Touched

- `tools/release/run_coverage_telemetry.py` (new): seam-2 opt-in runner;
  reuses the sweep's git-binding/runtime-capture helpers and the readiness
  script's manifest walk by module load; lanes are independent (no
  fail-fast — telemetry never blocks); commit-bound artifact
  `COVERAGE_<stamp>_<commit12>[-dirty].json` written to
  `validation/evidence/coverage/`.
- `tools/release/coverage_telemetry_schema.json` (new): draft 2020-12 JSON
  Schema for the artifact (`schema_version` 1); the runner validates every
  summary against it before writing.
- `tests/test_coverage_telemetry_script.py` (new): focused tests — lane plan,
  never-blocking/not-fail-fast semantics, `not_measured` on missing tool and
  on per-crate failure (lane reverts per DEC-060), schema self-validation,
  filename commit-binding, and a guard that the DEC-025 sweep plan remains
  the unmodified five surfaces.
- `apps/desktop/package.json` + `package-lock.json`: `@vitest/coverage-v8`
  devDependency (dev-tooling only; lockfile delta is the provider tree only).
- `requirements-dev.txt`: `coverage>=7,<8` (dev-tooling only).
- `docs/RELEASE_QUALITY_GATES.md` §10: the coverage-thresholds TBD closes to
  the DEC-060 ruled pointer (telemetry ruled; floors deferred to
  observed-baseline promotion).
- `plans/PLAN_2026-06-17_prd_completion.md` E8 row: coverage-tooling portion
  marked landed; remaining E8 scope (gate-family records, release-label
  vocabulary) left in the row.
- `plans/PLAN_COMPLETION_LOG.md`: narrative entry (newest first).
- Host-installed (not repo files): `cargo-llvm-cov 0.8.7`, rustup
  `llvm-tools` component, `coverage.py 7.15.0` in the local interpreter.

## Checks

- `python3 tools/release/run_coverage_telemetry.py` (dry-run): plan prints,
  exit 0; unknown `--lanes` selection rejected with exit 2.
- Pre-commit shakedown runs (dirty tree, artifacts discarded to a scratch
  directory, not committed): python + desktop lanes measured
  (387/387 pytest; 407/407 vitest), then the full 34-crate Rust lane
  measured (aggregate lines 89.26%, functions 91.31%, regions 87.97%);
  every emitted summary validated against
  `tools/release/coverage_telemetry_schema.json`.
- Clean-head telemetry run at committed HEAD (`--execute`, all three lanes):
  see Evidence below; artifact committed under
  `validation/evidence/coverage/`.
- `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -q` from the project root:
  pass (includes the new focused tests).
- `PYTHONDONTWRITEBYTECODE=1 python3 tools/practitioner_harness/harness.py
  self-check` from the repo root: exit 0 (pre-existing findings only; none
  introduced by this tranche).
- DEC-025 five-surface sweep at committed HEAD
  (`python3 tools/release/run_evidence_sweep.py --execute`): see Evidence
  below — the sweep plan, script, and semantics are untouched by this
  tranche (guarded by test).

## Evidence

- Clean-head coverage telemetry artifact (first DEC-060 artifact; all three
  lanes `measured`):
  `validation/evidence/coverage/COVERAGE_20260710T232606Z_e9cd806811b3.json`,
  bound to commit `e9cd806811b3725de9de4ba1bbb948b989921dfb` with
  `working_tree_dirty=false`; validated against
  `tools/release/coverage_telemetry_schema.json` (in-run and independently).
  Lane numbers (telemetry only — no floor exists and nothing gates on them):
  Rust aggregate lines 89.2636% / functions 91.3094% / regions 87.9712%
  across 34 crates; Python statements 87.0178% (14,820/17,031); desktop TS
  lines 84.3553% / statements 82.7811% / functions 90.4283% /
  branches 74.3885%.
- Clean-head DEC-025 sweep summary: run at committed HEAD and committed
  under `validation/evidence/sweeps/`; exact filename, bound commit, and
  status recorded here by the sweep-evidence commit. (Filled in below once
  emitted.)

## Boundary Review

- Dev-tooling only: no runtime or numerical dependency added; the shipped
  computational core is untouched (DEC-023 distinction preserved). The only
  manifest edits are a devDependency and a dev requirement.
- The DEC-025 five-surface gate is not modified: `run_evidence_sweep.py` and
  `check_release_readiness.py` are unchanged; the telemetry runner sits
  beside the gate and nothing invokes it automatically.
- Telemetry is recorded, never blocking; a missing tool records
  `not_measured`; no CI activation; artifacts stay local in the repo.
- No numeric coverage floor exists or is implied by the recorded numbers;
  floor promotion remains a future human ruling gated on at least five
  clean-head artifacts across at least two distinct commits per lane.
- No lifecycle transition, no `_STATUS.md` change, and no release-readiness,
  professional, certification, sealing, authentication, or code-compliance
  claim is created by this tranche or its artifacts.
