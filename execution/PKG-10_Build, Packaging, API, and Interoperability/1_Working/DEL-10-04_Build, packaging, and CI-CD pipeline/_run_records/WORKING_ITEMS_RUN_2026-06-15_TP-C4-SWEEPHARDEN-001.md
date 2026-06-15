# WORKING_ITEMS Run Record - TP-C4-SWEEPHARDEN-001

Date: 2026-06-15
Agent: WORKING_ITEMS
Plan item: DEC-025 merge-gate load-robustness (regression repair / smoke-evidence
gap — de-flake the five-surface sweep). Resolves the hand-off deferred as
out-of-scope in `TP-C4-SOLVERREFAUTHOR-001` (DEL-06-02 run record) and earlier
flagged in `TP-C3-LIBREFAUTHOR-001`.
Deliverable: DEL-10-04 Build, packaging, and CI-CD pipeline (DEC-025 sweep +
Playwright e2e harness)
Companion record: same filename under DEL-07-02 (App.test.tsx de-flake).

## Scope

Made the DEC-025 five-surface evidence sweep (`tools/release/run_evidence_sweep.py
--execute`, the commit-bound merge gate) produce a single all-green run on a
loaded developer machine. Robustness is now intrinsic to the committed harness
— no runtime knob an operator must remember (this supersedes the prior session's
uncommitted `VITEST_MAX_FORKS=3` workaround). Two separable harness issues:

1. Playwright worker-teardown hang. With Playwright's default worker count
   (~half the cores), the `desktop_playwright_e2e` surface — which runs last,
   after cargo + wasm + vitest have loaded the machine — left several Chromium
   workers slow to reap; teardown then tripped Playwright's internal 300s
   worker-stop grace ("worker process did not exit within 300000ms after stop,
   force-killed it"), failing a run whose 10/10 + 1/1 tests had all passed.
   Both `apps/desktop/playwright.config.ts` and `playwright.dist.config.ts` now
   honor a validated `PLAYWRIGHT_WORKERS` env override, defaulting to
   `CI ? 1 : undefined` so standalone local runs keep Playwright's fast default.
   `tools/release/run_evidence_sweep.py` sets `PLAYWRIGHT_WORKERS=1`
   (`os.environ.setdefault`, so an explicit caller value still wins) for the
   gate, running e2e workers serially under load.

2. App.test.tsx sub-global per-test timeouts (companion DEL-07-02 record):
   four heavy full-`<App/>` renders inherit the 30s global instead of tighter
   10s/15s overrides.

## Evidence

Files touched (this deliverable's surface):

- `apps/desktop/playwright.config.ts` (worker cap + `PLAYWRIGHT_WORKERS` override)
- `apps/desktop/playwright.dist.config.ts` (same)
- `tools/release/run_evidence_sweep.py` (`import os`; sets `PLAYWRIGHT_WORKERS=1`
  for the gate via `setdefault`)

Cross-surface file (companion record): `apps/desktop/src/App.test.tsx`.

## Validation

- DEC-025 five-surface sweep `python3 tools/release/run_evidence_sweep.py
  --execute`: **overall pass**, all five surfaces exit 0, at `uptime` load
  average oscillating 6–12 on 8 cores. Per surface: cargo_crate_sweep pass
  (7.2s) · python_pytest pass (10.4s) · desktop_vitest pass 372/372 (62s) ·
  desktop_playwright_e2e pass (75.1s) · desktop_production_build pass (12.5s).
  Artifact: `validation/evidence/sweeps/SWEEP_20260615T065727Z_31cd9ece4289-dirty.json`.
- Playwright in-sweep: dev-server lane "Running 10 tests using 1 worker → 10
  passed (54.9s)"; dist lane "Running 1 test using 1 worker → 1 passed (6.1s)".
  No "did not exit" / "force-killed" / "not a part of any test" markers.
- Config resolution checked via `playwright test --list --reporter=json`:
  `config.workers` = 1 under `PLAYWRIGHT_WORKERS=1`, 3 under `=3`, 4 (default) and
  with no env locally; garbage/zero input falls back correctly.
- `python3 -m pytest tests/test_evidence_sweep.py`: **16/16 pass** — the
  `(command, root)` runner contract and surface plan are unchanged.

## Boundaries

Test/CI-harness changes only. No product code, no status-vocabulary or
compliance claims; DEL-10-04 stays CHECKING. The worker cap and timeout changes
do not alter any test assertion or the five-surface plan/order. A green sweep is
local-only development evidence: not a release claim, professional approval,
certification, sealing, authentication, or code-compliance determination.

## Residual Handoff

- The sweep ran on a dirty working tree; the authoritative merge-gate run binds
  to a clean committed HEAD. Git closeout (commit this fix, re-run the sweep
  clean, push — which also unblocks the paused `TP-C4-SOLVERREFAUTHOR-001`
  tranche) is pending human direction. No transient summary was committed.
- The `PLAYWRIGHT_WORKERS` knob is documented in the config comments; should a
  hosted CI lane activate (D-05b), it inherits the `CI ? 1` serial default.
