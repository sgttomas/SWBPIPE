# B1 return

## Result

PASS. The sealed preimage `be0650f640986a9282db43803107439f0c08d71a7c37639508e3758fa6c13248` was updated only in `projects/chirality-piping/apps/desktop/e2e/r2-smoke.spec.ts`. The final test-file SHA-256 is `da2efafca0594a417cfd3c03aa99c143325048e6419a7d3a1c4ff4434a5b82b6`.

The unified diff against that sealed preimage is `SEALED_PREIMAGE_TO_FINAL.diff`, SHA-256 `cf53ac74b52ba38757aca4600f9ce2a8998c9a40d564ad1b402b2273c9ea4e69`. A byte comparison confirmed that file equals a fresh `diff -u` from the hash-matched sealed candidate to the final source.

## Validation

Working directory: `projects/chirality-piping/apps/desktop`

```sh
PLAYWRIGHT_WORKERS=1 ../../node_modules/.bin/playwright test e2e/r2-smoke.spec.ts --grep 'R2 desktop preview smoke covers solve, results, report, and viewport overlay|R2 from-blank GUI journey authors the A12 rehearsal script' --workers=1
```

Exit `0`; `4 passed (1.6m)`:

| Project | Journey | Result |
| --- | --- | --- |
| `chromium-desktop` | `R2 desktop preview smoke covers solve, results, report, and viewport overlay` | PASS, 20.0s |
| `chromium-desktop` | `R2 from-blank GUI journey authors the A12 rehearsal script` | PASS, 10.6s |
| `chromium-compact` | `R2 desktop preview smoke covers solve, results, report, and viewport overlay` | PASS, 19.4s |
| `chromium-compact` | `R2 from-blank GUI journey authors the A12 rehearsal script` | PASS, 10.2s |

Prepared engine artifacts were consumed by direct Playwright invocation; no build was run.

## Assertion and numbering map

The new `applyReviewedDraft` helper asserts the visible `m` control, `Single operation`, exact operation ID, `Validated model hash: sha256:`, the entity `not_present` diff, `[m]`, enabled Apply action, published tree entity, total applied count, `local_wasm_engine` receipt route, local-session acceptance basis, session-only persistence, and the professional-approval boundary.

| Journey/action | Old queue / receipt assertion | New review or queue / receipt assertion |
| --- | --- | --- |
| Main node `node:V-001` | `editor-intent-1` / `applied-1-editor-intent-1` | review 1 / `applied-viewport-draft-review-1`; total applied 1 |
| Main combination | `editor-intent-2` / `applied-2-editor-intent-2` | `editor-intent-1` / `applied-2-editor-intent-1` |
| Blank start node `node:R2-100` | `editor-intent-1` / applied 1 | review 1 / `applied-viewport-draft-review-1`; total applied 1 |
| Blank loaded node `node:R2-110` | `editor-intent-2` / applied 2 | review 2 / `applied-viewport-draft-review-2`; total applied 2 |
| Blank material | `editor-intent-3` / applied 3 | `editor-intent-1` / applied 3 |
| Blank section | `editor-intent-4` / applied 4 | `editor-intent-2` / applied 4 |
| Blank pipe `pipe:R2-100` | `editor-intent-5` / applied 5 | review 3 / `applied-viewport-draft-review-3`; total applied 5 |
| Blank support | `editor-intent-6` / applied 6 | `editor-intent-3` / applied 6 |
| Blank load case | `editor-intent-7` / applied 7 | `editor-intent-4` / applied 7 |
| Blank primitive load | `editor-intent-8` / applied 8 | `editor-intent-5` / applied 8 |
| Blank combination | `editor-intent-9` / applied 9 | `editor-intent-6` / applied 9 |

`applyQueuedIntent` now takes separate queued and applied sequences, preserving the legacy `editor-intent-N` path for forms and placeholders. The separate placeholder test is unchanged. Existing explicit provenance, storage/unit round-trip, model, solve-clearing, edited-model solver-boundary, and report assertions remain in place.

## Containment

B1 wrote only the authorized test file and this B1 evidence directory (`STATUS.md`, `RETURN.md`, and `SEALED_PREIMAGE_TO_FINAL.diff`). Unrelated shared-workspace F4 changes were not touched. `git diff --check` passed for the test file. No blocker or rerun requirement remains for B1.
