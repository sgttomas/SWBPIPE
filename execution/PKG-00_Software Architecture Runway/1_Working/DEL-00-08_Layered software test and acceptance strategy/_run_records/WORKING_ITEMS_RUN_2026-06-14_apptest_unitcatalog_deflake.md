---
run-id: WORKING_ITEMS_RUN_2026-06-14_apptest_unitcatalog_deflake
timestamp: 2026-06-14T01:25:00-0600
completed: 2026-06-14T01:35:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping
write-authorization: HUMAN_DIRECTED_REGRESSION_REPAIR
---

# WORKING_ITEMS_RUN — deflake App.test.tsx unit-catalog basis-label assertions

## Tranche and authority basis

- Human-directed regression repair (the deflake hand-off spawned from
  `TP-C3-LIBREFAUTHOR-001`, then run in-session at the human's direction). A
  flaky Vitest assertion in `apps/desktop/src/App.test.tsx` intermittently
  failed the DEC-025 five-surface evidence sweep (~1 run in 4 under the full
  18-file parallel suite), making the commit-bound merge gate non-deterministic.
- Coordination basis: the loop prefers regression repair (failed/insufficient
  tests, smoke-evidence gaps) ahead of new scope. This repairs a test-evidence
  determinism gap; it changes no application behavior.

## Root cause (test-only timing bug; app behavior correct)

`LoadCaseManagerPanel` (and the PropertyInspector editor-intent panel) load the
DEC-018 unit catalog **asynchronously**: `unitCatalogRoute` starts `null`
(`useState`), and an effect calls `loadUnitCatalog().then(setUnitCatalogRoute)`.
`describeUnitBasis` returns the magnitude/value basis label as:

- `"<unit>, catalog loading"` while `route === null` (not yet resolved), then
- `"<unit>, model metadata"` once the route resolves to
  `unavailable_browser_preview` (the browser/test-mode result).

So the rendered label transitions `"Magnitude (Pa, catalog loading)"` →
`"Magnitude (Pa, model metadata)"`. Four tests asserted the **settled** text
synchronously (`expect(within(x).getByText("… model metadata")).toBeInTheDocument()`)
with no await/barrier. Under parallel load the async catalog resolution
occasionally lagged past the synchronous assertion, so `getByText` found the
"catalog loading" label instead and threw "unable to find element" — the
observed failure at `App.test.tsx:2129`
(`getByText("Magnitude (Pa, model metadata)")`).

This is purely a test race: the app correctly shows "catalog loading" then
"model metadata". Confirmed by inspection of
`apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx` (state line 156,
effect line ~290) and `apps/desktop/src/services/unitCatalogService.ts`
(`describeUnitBasis`). The PropertyInspector create-material/section tests were
**not** flaky because they already gate on a `waitFor` barrier
(`property-unit-catalog-status` → "browser preview uses model metadata") before
their synchronous basis-label assertions — the established correct pattern in
this same file.

## Fix (test-only, idiomatic RTL barrier)

Converted the four unbarriered synchronous assertions to the async
`findByText` form (retries until the settled label renders, deterministic
regardless of when the catalog resolves; a genuinely-absent label still fails
via timeout, preserving the assertion's intent):

- `App.test.tsx:1696` (editor-intent panel, `Proposed value (Pa, model metadata)`)
- `App.test.tsx:1801` and `:2129` (load manager, `Magnitude (Pa, model metadata)`)
- `App.test.tsx:2185` (load manager thermal, `Magnitude (degC, model metadata)`)

No application code changed; no other test changed. The four PropertyInspector
`getByText("… model metadata")` assertions (lines 6293/6294/6355/6356) stay
synchronous — they are already protected by the preceding `waitFor` barrier.

## Evidence

- `npx vitest run src/App.test.tsx` (isolation): **52 pass**.
- `npm run test:desktop` (full 18-file parallel suite) **× 12 consecutive runs:
  345/345 every run** (12 pass / 0 fail). At the prior ~25% flake rate, 12
  consecutive clean runs is ~3% likely by chance — strong evidence the race is
  removed.
- DEC-025 five-surface sweep: **PASS** (see the committed sweep summary under
  `validation/evidence/sweeps/`).

## Boundary compliance

Test-only change; no application behavior, schema, or boundary surface touched.
Status-vocabulary-only; deliverables stay `CHECKING`; no
release/professional/certification/code-compliance claim. Git/test evidence is
source-control hygiene only.

## Residuals

- None for this flake. General guidance (not a gap): any future test asserting a
  unit-catalog basis label (`"… model metadata"` / `"… catalog loading"`) must
  await the settle (`findByText`/`waitFor`) rather than assert synchronously, as
  the existing PropertyInspector tests already do.
