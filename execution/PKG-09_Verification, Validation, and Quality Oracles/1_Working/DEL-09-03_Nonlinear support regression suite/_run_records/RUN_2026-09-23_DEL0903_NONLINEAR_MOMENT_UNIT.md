---
run_id: RUN-2026-09-23-DEL0903-NONLINEAR-MOMENT-UNIT
package_id: PKG-09
deliverable_id: DEL-09-03
status: CANDIDATE_FOR_REVIEW
branch: claude/nonlinear-moment-unit-20260923
prepared: 2026-09-23
executor: Claude Code (Claude Opus 5.5), untyped session with the owner
---

# DEL-09-03 — Nonlinear-suite moment unit relabel (D-76)

Basis: owner ruling `D-76` (`execution/_Coordination/_DECISIONS/D-76_RULING_2026-09-23.md`, `DEC-114`).

## Finding

The nonlinear fixtures measure length in millimetres (`D-75`; members are
`1.0` long, gap clearances `0.05` and `0.0002` mm) and force in newtons, and the
solver is unit-agnostic, so moments, moment reactions and work/energy residuals
come out in N·mm. The suite labelled them `N-m`. No check compared quantities in
different units, so no verdict was wrong; only the label was.

## Change

100 occurrences of `N-m` became `N-mm` across the benchmark code (13), ten
DEC-046 JSON records (one unit field each), the nonlinear hand-calc notes and
README, and the regression test (5 assertions). No number changed.

## Commands and results (from `projects/chirality-piping`)

| Command | Result |
|---|---|
| `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml` | `ok. 19 passed` |
| `python3 -m pytest -q tests/test_nonlinear_support_regression.py` | `8 passed` |
| `python3 docs/validation_manual/cases/generate_validation_case_pages.py --check` | 64 pages, no mismatch (nonlinear case records carry no moment unit) |

Repository checks, the evidence sweep and independent review are recorded in the PR.

## Boundary

Unit-label correction only. The solver, the desktop app, the product preview,
the mechanics and stress suites (in metres) and all limits are unchanged.
Development verification evidence only (BS-VALID, DEC-081).
