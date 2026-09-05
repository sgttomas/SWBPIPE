# W7 package return

**Status: technically verified source handoff; native integration remains with HELP_HUMAN.**

PKG-07 primary selection: DEL-07-01/02 broader viewport and inspector UX. Supporting integration: DEL-07-03/04/05/06/08; DEL-07-09 organization contract only. Accepted scope remains SOFTWARE_DECOMP 0.12 / SCA-009 / DAG-010. The report/map are design inputs, not amended authority.

## Delivered and verified

Persistent canvas; compact icon/task toolbar; transient searchable command discovery; collapsible model browser; independently scrolling creation/property rail; contextual creation disclosures; typed text versus quantity readouts; bounded lower review/workflow dock; concise statuses with technical Details; native minimum client logical size 1024 × 768. Existing commands, validation, exact batch application, cancellation guards and session undo semantics remain.

Source freeze: `snapshots/SOURCE_V2/MANIFEST.json` (`c53fd038fd753ea39049b37de98343d6427abb11446439fd1b41b95703eda5f2`); frozen full diff `5519dffef63def83b43bc11f90dbd6e063f65621a503169c27fb719015cf164f`. Thirteen files, listed in the manifest. All current hashes match.

- Final desktop suite: 45 files / 748 tests PASS, 160.37 seconds.
- Final production build: PASS, 5.361 seconds; existing Vite chunk-size warning.
- Browser layout regression: 2/2 PASS at desktop and owner-selected compact size.
- Final browser workflow: 12 states PASS, zero page errors, including direct Support focus and real Wasm batch validation/application/one undo.
- Fresh independent R1 review of 100% of all 13 changed files: PASS, no actionable findings. Prior P2 closed. `children/R1/RETURN_V2.md` is current.
- Source scope validation and `git diff --check`: PASS. Concurrent units/policy work belongs to root’s other instances.

At 1024 × 768, the final canvas is 708 × 459 for default/selection/support, 673 × 476 for routing, 708 × 233 for load/batch review, and 708 × 195 for the populated result view. These are observations, not ratified usability thresholds. Every measured canvas stays within its workspace and there is no horizontal page overflow. Baseline Toolkit instead pushed the canvas below the visible workspace. Screenshots and exact rectangles are in `after-*.png` and `after-geometry.json`.

## Delegation and accepted returns

U1 implemented App, Toolkit, viewport presentation and styles. U2 implemented inspector/tree presentation. Their concurrent writes were disjoint. U1 amendments added host-size observation after an actual browser reproduction, then a direct Support focus repair following R1 review. U2 amendments reduced irrelevant expanded forms and technical copy following visual fan-in. Accepted child returns are U1 `RETURN_V2.md`, U2 `RETURN_V3.md`, and R1 `RETURN_V2.md`. Earlier returns, failed tests, the failed review, and exploratory screenshots remain preserved.

Actual models and token/context occupancy were unavailable and remain unknown. Native child roles/non-delegation are instruction/config asserted. `RUNTIME_EVENTS.jsonl` and `RUNTIME_SUMMARY.json` record execution.

## Residuals and handoff

This is the first workspace slice. It retains Queue → Review → Apply; MAP-031’s inline human Add/Apply and complete typed-property widgets remain explicit residuals. No broader routing geometry, connection semantics, canonical analytical bridge, durable history or live provider work is claimed. D-58 and DEL-07-06 independent usability holds remain.

Browser fixture results and browser-memory storage are not native solver/SQLite evidence. Root owns packaged build identity, the native connected walkthrough, cross-package fan-in, harness checks and the clean committed-head DEC-025 sweep. W7 has released the build to N1 and stopped its Vite/browser/build processes. No further W7 source/config/test or SMOKE changes are planned.

Two selected deliverable-local run records were added under DEL-07-01/02. No ScopeOfWork, authoritative scope, lifecycle status, dependency or user model changed. Derivative evidence is current for SOURCE_V2; no scope-driven regeneration is required. Any source change invalidates the corresponding freeze/review/check binding and requires proportionate re-verification.
