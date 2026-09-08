# U7 Successor Manager Validation V1

Verdict: `PASS_FOR_SAME_RU_BACKCHECK`.

U7 manager remained read-only over product paths after R2 dispatch. The child return and status hash correctly, all nine live source hashes match its table, and Git status contains exactly the same six tracked plus three untracked U7 product paths as V1. R2 changed six paths within that fence and left PropertyInspector, typed inspector tests, and Playwright source byte-identical to V1.

The reconstructed V1 tree matches all nine V1 manifest hashes. The successor full-base diff contains nine path headers; the byte-exact V1-to-successor delta contains six. Both use portable repository-relative paths. Scoped `git diff --check`, trailing-whitespace scan, CR scan, LF-final-newline checks, and manifest rehash pass.

Evidence accepted for backcheck: route 17/17; App 157/157; inspector 9/9; desktop TypeScript/Vite build PASS; Chromium 1/1 at 1024×768. The initially failing build is preserved in the child return. Its final repair is only TypeScript `!` assertions at already runtime-gated `acceptance`/`applied_model` uses; emitted JavaScript control and data flow are unchanged. The affected App set, final build, and final browser passed after the narrowing repair, so repeating the 157-test file solely for erased assertions was not required.

RU-F1 through RU-F5 have source and repository-test closures in `RU_FIVE_FINDING_CLOSURE_V1.md`. Consumer evidence is updated in `SEVEN_ROW_CONSUMER_PROOF_V2.md`. Technical acceptance remains pending same-RU backcheck; packaged/native and root fan-in remain held.
