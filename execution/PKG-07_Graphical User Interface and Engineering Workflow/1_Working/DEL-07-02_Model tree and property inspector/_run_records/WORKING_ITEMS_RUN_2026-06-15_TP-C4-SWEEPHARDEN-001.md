# WORKING_ITEMS Run Record - TP-C4-SWEEPHARDEN-001

Date: 2026-06-15
Agent: WORKING_ITEMS
Plan item: DEC-025 merge-gate load-robustness (regression repair / smoke-evidence
gap — de-flake the five-surface sweep). Resolves the hand-off deferred as
out-of-scope in `TP-C4-SOLVERREFAUTHOR-001` (DEL-06-02 run record) and earlier
flagged in `TP-C3-LIBREFAUTHOR-001`.
Deliverable: DEL-07-02 Model tree and property inspector (App test surface;
DEL-07/DEL-14)
Companion record: same filename under DEL-10-04 (e2e harness + sweep tooling).

## Scope

Test/CI-harness-only de-flake of the `App.test.tsx` desktop integration suite so
the commit-bound DEC-025 five-surface sweep produces a single all-green run on a
loaded developer machine, with no assertion weakened and no product code touched.

App-test-surface change (this deliverable):

- Removed four explicit per-test timeout overrides on heavy full-`<App/>`
  (Three.js) render cases in `apps/desktop/src/App.test.tsx`. Each override
  (`}, 10000)` ×3 and `}, 15000)` ×1) sat *below* the file's 30s global
  `testTimeout` (`apps/desktop/vite.config.ts`), so under sweep load the tests
  tripped their own tighter budget and failed with "Test timed out" even though
  every assertion was satisfiable. They now inherit the 30s global, each with a
  one-line comment.
- The four cases: "carries queued editor intents into the report packet…",
  "creates a blank local model document…", "round trips review-only proposal
  operations through local save and open", and "shows computed mechanics
  diagnostics…". NOTE: the originating hand-off named only three; the
  "creates a blank local model document…" case carried the identical
  sub-global-override anti-pattern and was fixed alongside the other three.

The companion DEL-10-04 record covers the Playwright worker-teardown cap and the
sweep's `PLAYWRIGHT_WORKERS=1` default.

## Evidence

Files touched (this deliverable's surface):

- `apps/desktop/src/App.test.tsx` (4 per-test timeout overrides removed → inherit
  30s global; assertions unchanged)

Cross-surface files (companion record): `apps/desktop/playwright.config.ts`,
`apps/desktop/playwright.dist.config.ts`, `tools/release/run_evidence_sweep.py`.

## Validation

- `npm test --workspace apps/desktop -- --run src/App.test.tsx`: **52/52 pass**
  standalone (confirms the override removal does not alter behavior).
- `npm test --workspace apps/desktop` (full desktop Vitest): **372/372 pass**
  at default parallelism — no `VITEST_MAX_FORKS` reduction needed (the prior
  session's attempt-3 workaround is now obsolete).
- DEC-025 five-surface sweep `python3 tools/release/run_evidence_sweep.py
  --execute`: **overall pass**, all five surfaces exit 0, at `uptime` load
  average oscillating 6–12 on 8 cores. `desktop_vitest` surface 372/372 in 62s.
  Artifact: `validation/evidence/sweeps/SWEEP_20260615T065727Z_31cd9ece4289-dirty.json`.
- `python3 -m pytest tests/test_evidence_sweep.py`: **16/16 pass** (sweep
  entrypoint contract intact).

## Boundaries

Test/CI-harness changes only. No product code, no status-vocabulary or
compliance claims; DEL-07-02 and the App deliverables stay CHECKING. Timeouts
were relaxed to the existing 30s global only — no assertion, fixture, or
behavior was changed. The sweep remains local-only development evidence: not a
release claim, professional approval, certification, sealing, authentication, or
code-compliance determination.

## Residual Handoff

- The sweep ran on a dirty working tree; the authoritative merge-gate run binds
  to a clean committed HEAD. Git closeout (commit this fix, re-run the sweep
  clean, push — which also unblocks the paused `TP-C4-SOLVERREFAUTHOR-001`
  tranche) is pending human direction.
- Separate, non-blocking observation: at *abnormal* load (avg ~15, self-induced
  by back-to-back full-suite runs) two `App.test.tsx` cases flaked on
  testing-library's default 1000ms `findBy` `asyncUtilTimeout` — a different
  timeout than the vitest global, in tests never on the override list. It did
  not reproduce at representative sweep load (372/372 twice, incl. inside the
  green sweep) and the prior gate evidence shows no such failure. Left untouched
  to avoid scope creep; a future raise of `asyncUtilTimeout` in
  `apps/desktop/src/test/setup.ts` would close it if desired.
