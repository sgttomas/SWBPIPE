# RU R3 validation

## Identity and containment

- Launch brief: `d0927807457cf59d1c0a417e89c28d0751d691064a0fb77460937f0fe9d0febf` — match.
- R3 manifest: `bd5cdbdb12a71179df68305ef375071743607e91bea0f9d3fac074240247ca30` — match.
- Full base diff: `6e013cf88417d02c1f1f75d48fac124ba635f8e39b974c6d2cda2477b01b6b10` — match, 3,061 lines and nine paths.
- R2-to-R3 delta: `a0ae4a385ed9b76d8fa10974df218525b555732e333d4c8964ac76714b3a21fb` — match, 478 lines and five paths; reviewed completely.
- All 27 manifest entries independently rehashed `MATCH`; exact output is `_run_records/r3-manifest-verification.txt`.
- The nine live source hashes match the manifest. R3 remains inside the original nine-path fence; no source or repository test was written by RU.

## Focused final-byte checks

- RU producer-derived and token-ownership private suite — **PASS 3/3**, 1.19 s.
- `npx vitest run src/features/viewport/routeDraft.test.ts` — **PASS 17/17**, 425 ms.
- `npx vitest run src/features/model-tree/typedInspector.test.tsx` — **PASS 9/9**, 1.09 s.
- `npx vitest run src/App.test.tsx -t 'native straight-route Add and Apply|retains a malformed restored member|handles offline null'` — **PASS 38/38 selected**, 124 skipped, 43.35 s.
- `PLAYWRIGHT_WORKERS=1 npx playwright test e2e/linear-authoring.spec.ts --project=chromium-desktop` — **PASS 1/1**, 13.5 s, fixed 1024×768.

No review command failed. Exact successful output is retained under `_run_records/`. The author-reported full App 162/162 and build evidence remains bound through the frozen manifest; RU did not run the prohibited full suite or any build.

## Cleanup and limits

The Playwright `.last-run.json` marker and its now-empty task-owned directory were removed after output capture. No Rust, WASM, or native build, native-app run, full suite, full harness, sweep, Git mutation, lifecycle action, commit, or push occurred. Browser/Vitest checks used the existing prebuilt WASM service where the product normally loads it.

Actual model identity is not exposed for mechanical self-verification. Parent dispatch requested `gpt-5.6-sol` with high reasoning; this records the requested configuration. Parent: `/root` HELP_HUMAN. Role: same independent bounded ephemeral Agent 2 generalist. No delegation occurred.
