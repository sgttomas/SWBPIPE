# U7/I1 viewport-routing terminal return V1

Status: `IMPLEMENTATION_COMPLETE_AWAITING_ROOT_REVIEW_NATIVE_AND_GIT`

Accepted basis:

- assessment SHA-256 `0dd7f2433dea46cbc6ba92d3aa5f8c10e63ff6fb68344e1067ae4fed275b1252`
- design amendment SHA-256 `66974cc4c4773c2940c0a1abf5b903b220da99db8f7842cc0b86d1203a375719`
- sealed brief SHA-256 `986ac8fe0e5c317e445805ba65a29c6c1259eab28459ef968b93259046418d55`
- terminal design review SHA-256 `1d17f43618667557b2aa673b2e1f24d5eafb2d4e1fe81a70d2e7bd14a267fd99`
- source release SHA-256 `b3701fc0b27fc71cd999922affdf90b828acec63101822c25cda057a4efd0f10`

## Exact source/test manifest

| Project-relative path | Lines | Bytes | SHA-256 |
|---|---:|---:|---|
| `apps/desktop/src/features/viewport/PipeViewport.tsx` | 3097 | 131119 | `d48533921280f794ef655285fb0e899b8552a3e614674fb134efc33aa81365cb` |
| `apps/desktop/src/features/viewport/viewportRouting.ts` | 178 | 5311 | `3b76cb03e8369e1678c48587328be1a4b8c7fe22cc694036513e9c3a8967e8a4` |
| `apps/desktop/src/features/viewport/viewportRouting.test.ts` | 137 | 6387 | `960b69dfdbdf2e05544c60f8c43806fae9e64c886eb2648068fddbf7229bd011` |
| `apps/desktop/src/App.test.tsx` | 16607 | 699312 | `46357cd1128c5f7d6818f950848b1af4fcf921318a9ab72022045cbc0d3c7232` |
| `apps/desktop/src/styles.css` | 3438 | 75080 | `e33086d3bf832cc0f79592eff86f86b6c50356a3ee680f614665cc482a485304` |
| `apps/desktop/e2e/linear-authoring.spec.ts` | 213 | 14153 | `3508ec3ae9e926a0ffc13120ce1ec0a535e8d71acbbdfa8fe250619e3014785c` |

The exact current six-file source snapshot is archived at `SOURCE_SNAPSHOT_V1.tar.gz.b64`; `SOURCE_SNAPSHOT_RESOLVER_V1.md` records its digest and restore command. The basis-HEAD diff is root/CHANGE-owned because I1's sealed authority prohibited Git, and the manager performed the read-only Git containment/diff inspection.

## Behavior and evidence map

| Requirement | Implementation/evidence |
|---|---|
| From-anchored XY/XZ/YZ planes and exact fixed-axis equations | private helper; focused geometry tests; elevated XZ browser witness |
| Free plus only applicable X/Y/Z axes; incompatible reset | private helper; all-plane focused tests; browser disabled-axis and retained-X witness |
| Three grid, dashed ghost, endpoint marker without pointer-driven renderer rebuild | dedicated transient object updater ref; browser hover/capture witness |
| Same-primary-pointer 4 CSS pixel capture gate | helper tests cover exactly 4, greater than 4, mismatch, cancel and return-after-drag; browser orbit drag changes no fields |
| Mixed-unit conversion and stale rejection | one three-axis `convertDisplayQuantities` request; exact ID/unit/finite/cardinality validator; generation-gate tests; `3.2 m` request / `3200 mm` result tests |
| No-WebGL fail closed | projector absent; App regression proves pointer leaves values blank and manual fields remain usable |
| Explicit disabled reasons | adjacent plane/axis, pointer, node Add, and route Add reasons with matching titles/descriptions |
| Existing/new distinction | exact resolved-ID ghost helper and browser witness; pointer aids disabled for existing mode |
| Own-commit continuation | App and browser prove From moves to accepted end; plane/applicable axis/unit/material/dimensions/y-reference/pipe provenance retain; identities/end provenance clear; new-end mode remains ready |
| Frozen Add/review/Apply and operations | terminal 163-test App suite preserves single existing operation, ordered atomic new-end batch, stale/cancel/hash/revision/receipt/warning/zero/incomplete-node behavior |

## Validation

- focused helper: 20/20 passed
- focused App: 3/3 selected passed; additional affected selections passed
- full affected App: 163/163 passed
- TypeScript: `tsc -b` passed
- full desktop build: `npm run build` passed
- Playwright first run: 2/2 failed on matcher semantics against a literally disabled fieldset; raw failure preserved
- Playwright terminal rerun: 2/2 passed across `chromium-desktop` and `chromium-compact`
- whitespace/final-newline scan: passed for all six paths

## Containment and limits

I1 edited only the six authorized source/test paths and evidence under `instances/U7/children/I1/**`. Root/CHANGE supplied the prepared public Wasm and self-weight assets. Authorized Vitest/Playwright/build commands produced ignored cache, `test-results`, TypeScript build-info, and `dist` outputs; none is claimed as product source. I1 ran no Git, Cargo/Rust, native/Tauri build, unrelated Wasm build, whole-repository sweep, or delegated child.

No runtime behavior outside viewport route/node pointer authoring and transient routing presentation was intentionally changed. Native mixed-unit pointer capture plus save/reopen after explicit Apply remains the root-owned acceptance gate; this return does not claim native acceptance or final lifecycle closure.
