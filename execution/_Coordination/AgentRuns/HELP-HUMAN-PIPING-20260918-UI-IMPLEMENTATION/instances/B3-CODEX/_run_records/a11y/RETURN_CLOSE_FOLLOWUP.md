# B3 page Close keyboard follow-up

Continuation of TASK `/root/b3_manager/a11y_tests`, parent B3-CODEX WORKING_ITEMS `/root/b3_manager`, same delegated-harness-native mechanism and declared GPT-6 Astra / low allocation. Parent explicitly assigned the residual covered-page focus-return defect. Scope remained this new ordinary spec and new evidence filenames; no product/Git writes, delegation or native UI. Browser/server work waited for manager's slot signal. Earlier `RETURN.md`, `SHA256SUMS.txt`, logs and witnesses remain unchanged historical evidence.

## Reproduction and repaired outcome

Baseline source HEAD was `f5bc01f8beaa1c210b988fe28aea95573781f963`. A representative Libraries case opened its rail button with actual Tab/Enter, reached page Close with actual Tab, and pressed Enter. Without any subsequent focus call, activeElement was BODY and actual Chromium partial AX marked it ignored with role none. Originating rail focus assertion failed. Exact log `close-baseline.log` (1 failed), compact witness `close-baseline-witness/chromium-desktop-libraries-keyboard-close-focus.json`. Retained-state assertions passed in that run. The full body text in the baseline witness was subsequently limited to 200 characters for future witnesses only; assertions unchanged.

Manager applied automatic focus-return repair only after baseline completion. `close-repaired.log`: **18 passed in 36.8 seconds**, covering all 9 cases at 1440×920 and 1280×800. Existing 12 cases remain passing. New six cases automatically focus Libraries' originating `rail-page-libraries` opener, or safe `rail-stage-model` fallback for Project/Analyze when the originating menu command has unmounted. Each focus destination is visible, outside inert content, topmost at its center, and a nonignored button in Chromium's actual partial AX tree. No manual focus occurs after Close activation. Canvas, inspector and draft maintain exact DOM identity, draft text remains unchanged, and modeling workspace returns to the actual native AX tree.

Final spec SHA256 `d78772d001e99c2a4b5e1710a3b245865aa2022a424406ef2f71dce51686bc6e`. `close-candidate-sha256.txt` binds actual tested product and spec bytes; manager owns product changes. `CLOSE_SHA256SUMS.txt` seals this follow-up evidence and final spec. Prior spec hash/evidence remain historical; this appended regression extends coverage rather than rewriting their verdict.

## Commands and resource handling

From repository root, baseline used:

```sh
sh projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/tools/with_e2e_lock.sh sh -c 'cd projects/chirality-piping/apps/desktop && B3_A11Y_EVIDENCE_DIR="$PWD/../../execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/B3-CODEX/_run_records/a11y/close-baseline-witness" PLAYWRIGHT_WORKERS=1 npx playwright test e2e/b3-accessibility.spec.ts --grep="keyboard page Close.*libraries" --project=chromium-desktop --reporter=list'
```

Repaired used the same lock wrapper and command, changed evidence directory to `close-repaired-witness`, and omitted `--grep` and `--project` to run the complete bounded spec on both configured projects. Raw stdout/stderr preserved in the named logs. Existing environment remains source port 5174, Node v24.18.0, npm 11.16.0, Playwright 1.60.0 and Chrome 153.0.8010.48. Final run ended and browser lock released; no browser/server activity continues.

## Limits

This closes the tested browser keyboard Close regression only. Native HTML selector popup cancellation remains unverified, as previously reported. No native-host, full-suite, dist, 1024px, colour, usability or acceptance claim. Earlier tests' manual draft focus establishes refocusability only; the appended cases independently establish automatic return after keyboard Close. Parent owns review, integration and final wider validation.
