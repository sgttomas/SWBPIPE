# Production-dist lane diagnosis for PR905

HELP_HUMAN (ROOT). The [browser repair](../BROWSER_REPAIR/RETURN.md) observed six failures in the production-dist Playwright lane (`playwright.dist.config.ts`) at candidate `8e216efbf`. They failed identically with the unrepaired stylesheet. The hosted source workflow does not run this lane. The CI-bound DEC-025 sweep binds surface 4 to the hosted source run, so it does not run this lane either. The lane is still a maintained check of PR905's own changed specs, so each failure was attributed against current main in the same environment.

| Failure | Main `aa312755e`, same host/Chromium 1194 | Attribution and disposition |
|---|---|---|
| `ui-foundation-dist.spec.ts` "production appearance … 1024x768" ×4 (focus outline width 0 on `workspace-select`) | The same four fail (`ui-foundation-dist.spec.ts:637` on main); 45 other cases pass | Not introduced by PR905. Unchanged here. Carried as an open environment/appearance observation for the Results/appearance UI work. |
| `ui-foundation-dist.spec.ts` "populated Results title and entered values retain resolved contrast" ×2 | Pass on main (main's version of the test solved the fixture in the browser) | PR905's rewritten test defect. See below. Repaired. |

PR905 rewrote the contrast test to select `support:NL-140`, inspect the bundled reference, select a reference row, and then require the current selection readout to be unchanged. It captured the readout with `innerText` straight after selection. The readout (`src/features/viewport/PipeViewport.tsx:2474`) sits inside the viewport command bar's collapsible `<details>`. There Chromium returned an empty `innerText`, while the later `toHaveText` compares normalized `textContent`: "Selected support: support:NL-140; 0 queued". The application kept the current selection. The test's baseline capture was wrong.

The repair (`apps/desktop/e2e/ui-foundation-dist.spec.ts`) first asserts that the readout names `Selected support: support:NL-140`, then captures its `textContent` for the unchanged-after-reference-selection comparison. This strengthens the check: the earlier version compared against an empty baseline and could not have passed. No application code or criterion changed. Both cases pass on a rebuilt dist of `8e216efbf` plus this change ([log](_run_records/candidate-contrast-repaired.log)); builds and the main comparison are in [_run_records](_run_records/).

File sha256: before `888a63ded7086a5e289db7e327b81a78fb9664823e15b2d9c659db60ca31942e`, after `aeea81e76be910ca3bbf5a174d61996c4a0134dcc3097fbf9372b6b9b594089a`.
