# Handoff state

Run `HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`, ROOT (HELP_HUMAN, Agent 0). Working surface; superseded lines are struck, not deleted.

## Where the run stands

- 2026-09-18: run opened from `origin/main` `04aefa980`. Authority: `instances/ROOT/ACTIVATION_2026-09-18.md`. Plan: `ORCHESTRATION_PLAN.md`. Order: Tranche A1, Tranche B (shell lane and canvas lane), Tranche A2.

- 2026-09-18, Step 0 probes. Nested delegation: a child launched by the host's Agent tool launched its own child and received its reply, so Type 1 lane managers are available for Tranche B. Host capability: in this worktree `npm ci`, `npm run build:wasm:desktop` and `playwright test e2e/workspace-layout.spec.ts --project=chromium-desktop` ran to completion, 6 passed; the DEC-025 sweep's Playwright surfaces can run on this host.
- 2026-09-18, Tranche A1 launched: three sealed briefs (`briefs/_INDEX.md`).

- 2026-09-18, Tranche A1 integrated (returns in `briefs/_INDEX.md`). One addition beyond the brief, by ROOT's correction: a copy-forward of the local project store from the former bundle identifier's directory, because the renamed identifier would otherwise have orphaned existing stores.

- 2026-09-18, A1-REVIEW: PASS for the code on backcheck. Reviewer's observations kept for a later tidy, none actionable: the carry-forward needs write access to the former directory for SQLite's wal-index and otherwise starts with an empty store, leaving the former store intact; a first-launch pause proportional to store size; `-journal` beside the new store is not in the pre-cleanup list.

## Open with the owner

- ROOT chose the bundle identifier `com.swbpipe.desktop` for the brief, from the owner's registered domain. The owner may replace it; it should be settled before the App ID is created.
- The acceptance sentence survives inside two emitted data constants (`core/rules/rule_check_runner` and `core/rules/rule_pack_document`, the `professional_boundary_notice` schema `const`). The app no longer renders it. Changing it is a schema change and is left for the owner to direct.
- `DEC-102` needs a domain word for the two evidence labels; the product uses the design system's working word "Evidence".
- The external-harness panel's neutral wording ("External run evidence") is the implementer's, for the owner to replace.
- Not checked: whether webview local storage follows the bundle identifier on macOS; if it does, interface preferences reset once under the new identifier. No project data is involved.
- The new Apple App ID for the renamed bundle identifier. It blocks signed builds only.

## Departures from the plan

- Tranche A1 is cut into three children by path (lint and registry; `apps/desktop/**`; documents, core and tools), not the plan's four by topic. The plan's children 2 and 3 would both have written `App.tsx`, `App.test.tsx` and several panels; one writer per path is the rule.

- 2026-09-18, first sweep failed and was corrected: the dist-lane twin of the footer-geometry test was missed by the candidate; correction 3 and backcheck 2 are in `briefs/_INDEX.md`. Lesson for Tranche B briefs: every child that touches a `*.spec.ts` runs both Playwright lanes. Reviewer's observation, not acted on: the source and dist twins of the decorative-overlay test differ in wheel-target tolerance at `origin/main`; twin specs that drift are a maintenance risk. The design run's working handoff state now points here.

- 2026-09-18T22:01Z: Tranche A1 merged (PR #800, `7866f0a3c`). Tranche B opened on `codex/swbpipe-b1-tokens-20260918`: B1-TOKENS sealed and launched; the two lane-manager briefs sealed, launching after B1 merges. Frontier: B1's return, then its review, sweep, PR and merge.

- 2026-09-18, by 23:02Z: B1-TOKENS returned and was accepted after correction 1 (the disabled ink keeps its value); candidate `bf1ad5662`. Four workspace variables keep their values, with reasons, in `briefs/_INDEX.md`. B1-REVIEW sealed and launched.

- 2026-09-18T23:19Z: B1-REVIEW returned PASS with no actionable finding. Its four observations are carried: the binding test for the token file's `labels` table, the divider that also frames three bordered things, and a comment's contrast floor go to the shell lane by addendum; the fourth is a note on reading B1's two retained returns together. Frontier: the DEC-025 sweep on the clean commit through the browser-test lock, which is also the first browser run of the corrected state; then the closeout record, pull request, CI and merge. The A1 follow-ups proceed on their own branch in the second worktree.

- 2026-09-18T23:36Z: slice B1's DEC-025 sweep passed on `9e806691f` (all five surfaces; `_run_records/CLOSEOUT_CHECKS_B1.json`). Frontier: pull request, CI, merge; then the lane managers and the design-system contrast amendment.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
