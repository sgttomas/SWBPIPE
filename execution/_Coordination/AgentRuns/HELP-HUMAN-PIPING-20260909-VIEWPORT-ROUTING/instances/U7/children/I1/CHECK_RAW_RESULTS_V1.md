# U7/I1 check results V1

## Focused bootstrap gate, preserved

Command: `../../node_modules/.bin/vitest run src/features/viewport/viewportRouting.test.ts`

First attempt result: exit 1 before collection; 0 tests.

```text
FAIL  src/features/viewport/viewportRouting.test.ts
Error: WASM-ENGINE-ASSET-ABSENT: wasm artifact not found on disk: probed:
.../apps/desktop/public/wasm-engine
The wasm operation engine is required in browser mode (DEC-020 / ADR-0001) and no fallback engine exists.
Test Files 1 failed (1)
Tests no tests
```

No Wasm build was run by I1. Root/CHANGE subsequently supplied the prepared asset under its own authority.

## Focused and full test results

```text
../../node_modules/.bin/vitest run src/features/viewport/viewportRouting.test.ts
Test Files 1 passed (1)
Tests 20 passed (20)
Duration 335ms

../../node_modules/.bin/vitest run src/App.test.tsx -t "fails closed for no-WebGL pointer placement|keeps an accepted App route armed|retains a non-global plane"
Test Files 1 passed (1)
Tests 3 passed | 160 skipped (163)

../../node_modules/.bin/vitest run src/App.test.tsx
Test Files 1 passed (1)
Tests 163 passed (163)
Duration 160.32s
```

An earlier full App run was 162/163 because an existing test still expected continuation to reopen in existing-end mode. The test was corrected to the frozen new-end continuation behavior; its focused rerun and the terminal full App run passed.

## Browser attempts, preserved

Exact command for both attempts:

```text
../../node_modules/.bin/playwright test e2e/linear-authoring.spec.ts --project=chromium-desktop --project=chromium-compact
```

First attempt: exit 1, 2 failed. Both projects reached the late existing-route ghost assertion. The DOM resolved to the intended disabled fieldset, but Playwright's semantic matcher reported it as enabled:

```text
Locator: getByTestId('viewport-routing-aids')
Expected: disabled
Received: enabled
locator resolved to <fieldset disabled class="viewport-routing-aids" ...>
e2e/linear-authoring.spec.ts:139:59
2 failed
```

Only the assertion changed, from `toBeDisabled()` to literal `toHaveAttribute("disabled", "")`.

Terminal rerun: exit 0.

```text
chromium-desktop passed (10.0s)
chromium-compact passed (10.4s)
2 passed (11.5s)
```

## Build

```text
npm run build
tsc -b && vite build
1699 modules transformed
vite build completed in 1.21s
exit 0
```

Vite emitted its existing large-chunk advisory; compilation and bundling passed.
