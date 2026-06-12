# WORKING_ITEMS Run Record — Seam-Verification Regression Repair (TP-SEAM-FIX-001)

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona), executing the human-instructed
  regression-repair tranche from
  `plans/VERIFICATION_2026-06-11_operation_seam_unification.md` (findings
  F-1, F-2). F-3 (corpus fixture review disposition) remains with the human
  project authority and was not touched. F-4 is a standing operational note
  for `D-05` preparation; no code change was required or made.
- Scope: `apps/desktop/playwright.config.ts`,
  `apps/desktop/src/services/operationContractCorpus.test.ts`, plus this
  evidence surface, `apps/desktop/SMOKE.md`, and
  `plans/PLAN_COMPLETION_LOG.md`.

## F-1 — Playwright timeout (fixed; root cause measured)

**Symptom.** `npm run test:e2e:desktop` failed at the configured 30s
per-test timeout (verification measured 33.4s twice; this session
reproduced 37.4s cold).

**Root cause (FACT, measured this host 2026-06-11).** The growth is the
Playwright trace recorder, not the wasm engine:

| Configuration | Spec runtime |
|---|---|
| `--trace off` | 8.8s |
| `trace: retain-on-failure` (configured), warm dev server | 21.6s |
| `trace: retain-on-failure`, cold dev server | 37.4s |

`DEBUG=pw:api` timing attributed ~21s of the cold run to `browser.close`
(trace finalization) with the test body at ~22s; the engine-ready wait
itself resolves in well under 1s and the wasm artifact is only ~330 kB.
`retain-on-failure` records the trace on every run and discards it on
pass; the recorder snapshots the DOM after each action, and the T4 spec
extension added its apply-flow actions at the end of the spec where the
DOM is heaviest (solved 647-row results state), multiplying per-action
snapshot cost. Cold Vite dev-server transforms add the remaining
cold-vs-warm spread.

**Fix.** Raised `timeout` to `120_000` in
`apps/desktop/playwright.config.ts` with a comment recording the measured
basis (per the verification's recommended option). Tracing stays
`retain-on-failure` — the failure-diagnosis value is worth ~13s on a
passing run. Splitting the spec was not taken: the single ordered spec
encodes the by-design interaction (applying clears solve results) that a
split would have to re-derive.

## F-2 — Raw NUL bytes in `operationContractCorpus.test.ts` (fixed)

The two literal `0x00` dedup-key separators in `diagnosticSortKey` are now
`\u0000` string escapes (identical runtime behavior; byte-for-byte NUL
count now 0). Git classified the old blob as binary, so the diff *of this
fix* still renders as binary; diffs of all future changes to the file are
reviewable text.

## Evidence (2026-06-11, this host, sequential per F-4)

- `npm run test:e2e` (desktop; builds wasm first, default config): 1/1 at
  18.8s — comfortably inside the new 120s budget at the default command.
- `npm test --workspace apps/desktop`: 140/140 (7 files).
- Cargo profile sweep (`tools/release/check_release_readiness.py --profile
  cargo --execute`): exit 0, zero failing test-result lines.
- `python3 -m pytest -q tests`: 342/342.
- `npm run build --workspace apps/desktop`: green (`tsc -b` + Vite).

## Boundary review

Test-infrastructure-only change: no engine, fixture, or UI behavior
changed; local-only; no protected content; no release-readiness,
professional approval, certification, sealing, authentication, or
code-compliance claims.
