# RU successor validation

## Identity and scope

- Launch brief: `951051bf9a4d78b21795d05db82dc64a014fdfd35673180ab0d5967a8f9f55ab` — match.
- Successor manifest: `72d1cb5f31d7d81e92f05edd6f0b9e8b74c102cc27c898b7139e99644fb529f1` — match.
- Full base diff: `8c01ab323ef27836ecb1e777366b9166adfe28f9948bd990b01441fc2302f30e` — match, 3,025 lines and nine paths.
- V1-to-successor delta: `09881c4a563bd919a52bd855fc954c798565be9d8817dabfbf36a32793f83863` — match, 1,102 lines and six paths; reviewed completely.
- All 26 manifest entries independently rehashed `MATCH`; exact output is `_run_records/successor-manifest-verification.txt`.
- Current nine source hashes match the successor manifest. No source or repository test was written.

## Focused final-byte checks

- `npx vitest run src/features/viewport/routeDraft.test.ts` — **PASS 17/17**, 385 ms.
- `npx vitest run src/features/model-tree/typedInspector.test.tsx` — **PASS 9/9**, 967 ms.
- `npx vitest run src/App.test.tsx -t 'native straight-route Add and Apply|discards a delayed apply after opening a replacement model'` — **PASS 32/32 selected**, 125 skipped, 32.15 s.
- `npx vitest run src/App.test.tsx -t 'retains a malformed restored member|handles offline null'` — **PASS 2/2 selected**, 155 skipped, 5.05 s.
- `PLAYWRIGHT_WORKERS=1 npx playwright test e2e/linear-authoring.spec.ts --project=chromium-desktop` — **PASS 1/1**, 12.8 s, fixed 1024×768.
- Scoped TypeScript `transpileModule` assertion-erasure comparison — **PASS**, emitted JavaScript equal.
- RU affirmative private suite — **FAIL 3/3 as required to report defects**. Both altered payload references were accepted, a malformed info diagnostic was accepted, and coincident external replacement preserved continuation. Exact outputs and commands are preserved in `FAILED_COMMAND_001.md`, `FAILED_COMMAND_002.md`, and `_run_records/repros/`.

The green repository cases verify the repaired normal, warning, stale/cancel, persistence, object-key/array-order, provenance, reservation, undo/redo, seven-row, and 1024 layout paths. They do not contradict the narrower adversarial failures.

A final-verification command typo is preserved in `FAILED_COMMAND_003.md` and `_run_records/final-verification-attempt1.txt`. It created a repository-root file named `=` and omitted the intended e2e path from a diff check; the file was immediately removed with an exact guard. The corrected verification covers the tracked diff plus LF/CR/trailing-whitespace checks over all nine paths and passes.

## Cleanup and exclusions

The Playwright-created `apps/desktop/test-results/.last-run.json` and its now-empty directory were removed after output capture. RU retained only the required successor packet and its `_run_records` evidence. No Rust, WASM, or native build, native-app run, full harness, full sweep, Git mutation, lifecycle action, commit, or push occurred. Browser/Vitest checks used the existing prebuilt WASM service where the product normally loads it.

Actual model identity is not exposed for mechanical self-verification. Parent dispatch requested `gpt-5.6-sol` with high reasoning; this states the request rather than an independently verified identity. Parent: `/root` HELP_HUMAN. Role: same independent bounded ephemeral Agent 2 generalist. No delegation occurred.
