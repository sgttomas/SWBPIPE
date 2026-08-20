# WORKING_ITEMS Run — DEL-09-04 GUI Workflow Closure

- Date: 2026-08-20
- RunID: `HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE`
- Package / deliverable: `PKG-09` / `DEL-09-04`
- Base: `57803893d1eb161f395e0574c256dd27920bf1d4`
- Result: `PASS`; ready for CHANGE commit

## Product result

The focused Playwright journey now binds its expected states to the same invented repository-local model and mechanics-result fixtures the desktop uses. Through visible controls it covers pre-solve, solved, edit/apply, local save/list/reopen, and edited-model solve-blocked states; distinct missing-input, provenance, assumption, and solve-blocking warnings; explicit public/protected/private/professional boundaries; result rows 830→0; and zero external requests.

The final amended case requires visible Solve readiness and visible viewport reset status after apply before moving to the visible empty Results state. No `apps/desktop/src/**` product repair was required.

## Evidence

- Playwright discovery: exactly two cases, one in each registered project.
- Host Playwright final proof: exit 0; 2 passed in 46.2 seconds (`chromium-desktop` 22.7s, `chromium-compact` 22.1s).
- Desktop Vitest: 523/523 passed.
- Desktop production build: passed.
- Practitioner harness: 350/350 passed.
- Harness self-check: exit 0 with established findings only.
- Changed-path containment and `git diff --check`: passed.
- Integrated review 01: FAIL on one hidden-DOM assertion; repaired in place.
- Fresh integrated review 02: PASS, zero actionable findings; 17/17 frozen identities verified twice and 100% reviewed.

Evidence and the full proof loop are under the RunID root, including `FROZEN_DIFF_MANIFEST_V2.md`, `REMEDIATION.md`, normalized check JSON, and both review returns.

## Deliverable effect and boundaries

The validation-manual GUI inventory moves only from `PLANNED/TBD` to `DRAFT_EVIDENCE`, and `_STATUS.md ## Remaining` removes only the GUI-workflow evidence clause. Lifecycle remains `IN_PROGRESS`. Case pages remain `DRAFT_EVIDENCE`; `MAINTAINER_REVIEWED` promotion and final public comparison values remain open. No threshold, release, publication, acceptance, or professional-reliance act occurred.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Rerun

CHANGE must commit the exact accepted path set. Agent 0 then runs the complete DEC-025 sweep at clean committed HEAD; PR CI remains additional proof. If the e2e file identity changes, rerun both registered viewport projects and fresh integrated review.
