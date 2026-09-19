# Sealed addendum 4 to the B-SHELL lane brief — slice B2G, and ROOT's answers on B3

Sealed by ROOT (HELP_HUMAN, Agent 0) on 2026-09-19. Everything in the lane brief and addenda 1 to 3 still binds.

## B2F

The independent review returned PASS with no actionable finding (`{RUN}/lanes/B-SHELL/reviews/B2F-REVIEW_RETURN.md`). ROOT takes B2F to the sweep, the pull request and the merge from a branch of its own cut at the reviewed head, so the lane branch is free at once. Merge `origin/main` when ROOT tells you B2F has merged, at a clean point.

## Slice B2G — a failed or superseded save or create keeps the open project's integrity record

Your brought-back item 2, with the reviewer's refinement (its check 9), which ROOT adopts. `handleCreateProject` and `handleSaveProject` null `modelHashIntegrity` and `projectEnvelopeHashIntegrity` before their first `await`. Nothing is persisted when the request fails or is superseded, so the open-time verification is still valid, and a recorded `mismatch_review_required` disappears from a project that stays open. It under-reports a mismatch: a result-integrity defect.

- **Test first**, in `src/App.projectHandlers.test.tsx`: for a project opened with a recorded mismatch, a save that fails and a create that fails each leave `integrity=mismatch_review_required` on the validation panel; assert the envelope cell's line as well as the model cell's, which B2F's tests did not. Each fails on the B2F tree.
- **Smallest repair:** the two nulls move to where the save or create lands (after its `stillCurrent` check, on the success path). **On success the behaviour is exactly today's:** the cells are nulled and not re-derived. Whether a successful save should re-derive is a design question; do not answer it here; list it in the return.
- Untouched: everything B2F left untouched; `runMenuCommand`; every existing test.
- Small enough to do yourself or with one child. Checks: type check, unit suite, build; both Playwright lanes once on the final candidate. Return it as slice B2G and stop **only for B2G's review findings**: you may start B3 stage 1 on top of the frozen B2G candidate while ROOT reviews it, as long as a B2G finding can still be routed and fixed first.

Your brought-back items 1, 3 and 4 are left as they are; ROOT lists item 1 (the native menu during a busy request, in `src-tauri/**`) for the owner.

## B3: the cut and ROOT's answers

The cut in `{RUN}/instances/B-SHELL/B3_CUT_PROPOSAL.md` (`26c0d353…`) is accepted: one slice, one child, two stages, one return.

1. **The agent strip.** Draw the 44 px strip and the Agent toggle, disabled with its reason shown. The visible reason is **"Agent: not available yet"**, reachable by pointer and by keyboard. The move of the three agent panels into the column is a later slice after B5; propose it then.
2. **The benchmark instrument's hooks.** Keep each hook on its honest successor; never adjust a recorded value; never edit `e2e/ui-foundation/**`. Return the table (hook, old element and geometry, new element and geometry) **at the end of stage 1's planning or as soon as the child has it, before stage 2 lands**, so ROOT can give it to the canvas lane, which owns the instrument. If a first-profile recorded value cannot stay true under the new shell, stop and bring it to ROOT with the numbers.
3. **The native menu** stays as it is (`src-tauri/**` is outside the lane). Accepted.
4. **The window minimum** stays as it is. Accepted, with the narrow fallback no worse than today's.
5. **Your three decisions** (view memory as session state; run log and run menu in B6; status chips to specification §5.4 in this slice with each chip rule a named semantic change) are accepted. For the moved `status-pill-*` assertions, give the inventory in the return: old text, new text, the rule.

The design-fidelity review by screenshot of B3's candidate, for structure only, still runs before B3 merges (addendum 3).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
