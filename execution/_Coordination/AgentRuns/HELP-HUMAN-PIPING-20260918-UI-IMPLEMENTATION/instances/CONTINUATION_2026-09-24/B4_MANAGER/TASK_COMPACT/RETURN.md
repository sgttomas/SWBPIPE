# TASK compact drawer return

Completed implementation and focused browser checks; source frozen for the parent's independent review. Six owned source/test files and SHA-256 identities are in `CANDIDATE.json`, aggregate digest `5b83325516e87361da145a0d27807f8cd9a24b0d27b20e8c7e525ec7689836f1`. Product basis was `42bd234f6`; the parent's subsequent coordination/Runtime-only changes advanced shared HEAD. `CHECK_EXITS.json` records final HEAD and actual exits. No commits, staging, branch changes, instruction changes, or delegated descendants were performed.

## Changes

- `App.tsx` now allocates bounded table bodies in all views, passing an explicit compact presentation for collapsible drawers.
- `ModelTree.tsx` consolidates compact controls into one retained toolbar: Tree/Grid, filter, labelled native family selector with current family count, direct/review switch, Queue/Clear, and Details. The portal moves controls only; table and input ancestors retain their identity. Complete review boundary and retained-direct-draft explanations are available in a labelled Details dialog. Escape closes Details and returns focus to its trigger instead of also closing the narrow drawer. Queue still navigates to Operations through the existing route.
- `EngineeringTable.tsx` places compact feedback in the fixed footer, preserving its alert/status roles and editor error linkage. Long footer content remains horizontally scrollable rather than consuming vertical rows.
- Scoped CSS reserves 30px toolbar, 29px column header, and 32px footer, leaving a full 36px comfortable row at the 127px minimum. Row heights, VirtualList, shell budgets/reserve, engine, operation, controller, model, result, and persistence code remain unchanged.
- Twelve focused cases were added to the already-selected `b4-table-editing.spec.ts`; the existing dist appearance assertion now checks the native family selector when the compact presentation applies, retaining old tab assertions elsewhere. No CI mapping change was needed.

These are named presentation/interaction changes: compact family tabs become a select, review explanation moves to an explicit Details dialog, and compact feedback shares the footer. They do not change operation semantics.

## Focused validation

- Final source: 65 ModelTree/EngineeringTable unit tests passed; TypeScript build check passed.
- Final source: all 32 desktop B4/Sections browser cases passed (20 existing cases plus 12 new compact cases). Existing cases cover material/section direct and review behavior, invalid/stale editor retention, sorted and virtual input ownership, text Undo, command history, and save/reopen.
- The same 12 compact cases were rerun with JSON reporter: 12 passed, zero skipped/flaky/unexpected, to retain exact successful geometry and pointer-hit evidence. They cover Model at 1280×800 and stacked Both at 1024×768; comfortable/compact densities; requested drawer 180/280/500; direct invalid editor and footer actions; family round trips including legacy Pipes; review Keep/Queue/Clear; valid direct Apply/undo/redo; disclosure Escape and Tree dismissal; collapse/reveal and filter retention; row wheel containment; header/body column alignment. Existing Queue navigation was explicitly preserved by returning to Model before subsequent measurements.
- Minimum inner host was 127px, default 227px, with 36px and 136px row allocations respectively. Expanded requested 500px remained constrained by the existing modeling reserve: actual host 319/327px Model and 287/295px stacked Both, depending on density. No protected budget was modified.
- Host scroll height equalled client height in checked states; only rows scrolled vertically. Toolbar/footer can expose additional content through horizontal scrolling. Recorded `before`/`after` scroll positions distinguish initially exposed controls from controls revealed before pointer hit checks; do not claim every control/message is simultaneously visible at every width.
- Repaired during implementation: Details Escape originally bubbled to the narrow drawer handler; local dialog Escape ownership repaired it and final browser checks passed. Initial unit collection lacked generated WASM assets; building those assets repaired that setup failure. Initial test expectations incorrectly assumed Queue kept Model visible and requested expanded height equalled rendered height; tests now preserve actual shell behavior instead of altering it. Raw initial failures remain retained.

## Limits and next owner

This is browser evidence, not native Tauri/WebKit proof or practitioner acceptance. Native Model pointer/wheel evidence and reachable native Both behavior remain for the parent after source review. The native 1280px minimum prevents reaching the <1280px stacked-Both breakpoint; only the browser established that layout here. The separate historical wide-Both/Inspector Provenance pointer issue is not closed by this slice. The new dist assertion was typechecked and reviewed as test source, but its full dist appearance matrix was not executed by this TASK. Clean-candidate DEC-025, broad final checks, complete-diff independent review, PR/merge, and native evidence remain parent-owned.

Deliverable consequences: generic table semantics DEL-07-02; specialized library families DEL-07-03; shared hosting/focus/history DEL-07-11; review/apply DEL-07-08; runtime validation through DEL-16 owners. No lifecycle/release acceptance or final loop receipt/MEMORY row is claimed. The next family remains Pipes under the parent's separate activation.

## Resources and canonical evidence

All test/browser processes completed. Owned Vite on 5174 stopped with exit130; `lsof -nP -iTCP:5174 -sTCP:LISTEN` returned no listener. No native/CUA resources were used. Local npm dependencies and generated WASM assets are ignored setup artifacts, not tracked deliverables.

Track this canonical payload (all paths relative to this TASK directory):

- `RETURN.md`, `PROVENANCE.json`, `INPUTS.json`, `CANDIDATE.json`, `CHECK_EXITS.json`, `COMMANDS.md`.
- `unit.log`, `unit-after-wasm.log`, `unit-final.log`, `wasm.log`, `typecheck-final.log`, `browser.log`, `browser-r2.log`, `browser-r3.log`, `browser-r4.log`.
- `baseline-measurement.mjs`, `baseline-geometry.json`, `candidate-measurement.mjs`, `candidate-geometry.json` (exploratory chronology), plus final `browser-r4-report.json` and decoded `GEOMETRY_FINAL.json`.
- Selected final screenshots: `browser-r4/b4-table-editing-B4-compac-3abeb-table-180-explicit-viewport-chromium-desktop/compact-drawer.png` (comfortable Model minimum) and `browser-r4/b4-table-editing-B4-compac-38279-mpact-180-explicit-viewport-chromium-desktop/compact-drawer.png` (compact stacked Both minimum).

Other repeated screenshots, Playwright caches, and trace ZIPs are not needed for the canonical payload; retain locally if useful, without force-adding ignored traces. The successful JSON report binds geometry/scroll attachments to all 12 final cases. The selected screenshots are corroborating presentations, not a substitute for those assertions.
