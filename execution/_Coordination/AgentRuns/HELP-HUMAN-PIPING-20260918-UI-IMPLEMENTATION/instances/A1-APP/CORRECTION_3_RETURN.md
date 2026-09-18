A1-APP correction 3 return

Retained by ROOT. What follows is the child's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed.

---

The dist lane is green again: with `PLAYWRIGHT_WORKERS=1`, as the sweep runs it, `npm run test:e2e:dist` gives 53 passed, 0 failed, 0 skipped, exit 0. I am Claude Fable 5.1, a Type 2 task working alone. I ran no state-changing git command. The only file changed is `apps/desktop/e2e/ui-foundation-dist.spec.ts`.

**Same cause confirmed.** The error context is at `apps/desktop/test-results/ui-foundation-dist-decorat-2824e-w-controls-stay-interactive-chromium-desktop-dist/error-context.md`.
- It shows `Expected: > 757.40625`, `Received: 754.971875`.
- The failing assertion is `expect(end.y).toBeGreaterThan(axisRect.y)`.
- These are the same two numbers and the same assertion as the source-lane failure. The frozen endpoint at 0.81 of canvas height misses the top of the gizmo by 2.43 px because removing the footer made the canvas taller. Nothing else is involved.

**Change applied.** At `ui-foundation-dist.spec.ts:1452` the window is now 1440 × 899, with the four-line comment copied byte for byte from the source twin. Only the window height changed. The endpoints (0.29/0.58 and 0.08/0.81), the gizmo and scale-bar bounds checks, the tolerances and every assertion are untouched.

**Other tests checked in the dist spec.** Six other sites use height 920, at lines 120, 946, 1181 (paired with 1280 × 800), 1265, 1288 and 1320.
- None of them asserts that a canvas fraction falls inside the gizmo or any other overlay rectangle, and none pins an absolute pixel coordinate.
- The only other frozen-fraction drag is `dragEmpty` near line 1397, from 0.08/0.10 to 0.29/0.28 in the canvas's top-left empty region. A taller canvas does not push it into any overlay.
- The decorative-overlay test is the only place the 0.29/0.58 and 0.08/0.81 endpoints appear.
- All of these tests passed in both of my runs. I changed nothing else.

**Runs from `apps/desktop`.** Nothing was listening on 5173, 5174 or 5175 before either run.
- `npm run test:e2e:dist` with default workers: both wasm engines rebuilt, `tsc -b` and `vite build` were clean, and 53 tests passed with 0 failed.
  - The run still exited 1 on "worker-N process did not exit within 300000ms after stop, force-killed it", reported three times.
  - That is the worker-teardown condition the Playwright config's comment describes, and the reason the sweep sets `PLAYWRIGHT_WORKERS=1`. It is not a test failure.
- `PLAYWRIGHT_WORKERS=1 npm run test:e2e:dist`: 53 passed in 6.2 minutes, exit 0.

I did not re-run vitest, the source-lane Playwright run or `cargo test`, because no product source changed.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
