# CI focus repair TASK return

Completed diagnosis, manager-activated repair and focused verification. Four maintained paths are frozen in CANDIDATE.json, aggregate digest `712c9871f020f7eea67e5ea88bf23b130dcd83419a9898d8f5fc51d653cd4f37`. Product basis is 01063c7f1cfb7a651261401f62a5a72a5daa94dc; current shared HEAD is recorded separately. No Git, styles, controller, VirtualList, CI policy, native bundle/CUA or other-project source edits occurred.

## Supported cause

Hosted trace proves click → disabled endpoint/zero offset → null active-element label, but lacks focus event timing. A real local bundled Chromium148 probe supplies the missing causal observation: disabling the focused button synchronously moves focus to BODY before the layout effect. Installed Chrome153 leaves the disabled button focused long enough for the old guard to recover. The minimal native-button comparison confirms that distinction outside React. Thus the earlier Linux-only explanation is unnecessary; no Linux events are invented. The matching-engine local reproduction and raw hosted failure remain distinct from the previous local clean sweep pass.

## Repair

EngineeringTable and OverflowRail now capture the exact focused pan button/pair/owner before the offset or scroll mutation. The one-shot request is consumed after commit. Recovery requires the same connected visible active pair, unchanged owner, a now-disabled initiating button, and focus still on that button or BODY. Another live focus owner, disconnection, hidden/inert/inactive pair or owner replacement prevents recovery. The original normal guard remains; native disabled semantics, no-op boundaries, draft ownership and model/history behavior are preserved. No timers/global focus listeners or assertion weakening was introduced.

New OverflowRail unit cases cover both directions for tool/status rails, synchronous BODY focus, intentional outside focus without stale-request reuse, hidden pairs and changed owners. These explicitly bounded synthetic cases complement real browser proof. Existing six browser cases retain the original immediate column-focus assertion and add status-rail focus plus Enter/Space endpoint checks.

## Results and limits

- Final units: **58 passed**, no unhandled errors; TypeScript and whitespace checks passed.
- Bundled Chromium148 matching hosted version: **6 passed**, including all previously failing immediate assertions.
- Installed Chrome153 regression: **6 passed**.
- The first new unit fixture failed because jsdom would not blur an already-disabled button; correction models the observed BODY focus explicitly without weakening expectations. That failure is preserved. One diagnostic CDP enrichment failed because command-line access required an unavailable automation flag; the successful follow-up retains the limitation and captures actual executable launch logs.

The hosted failure remains blocking until ROOT's new hosted run; local148 passes are evidence of the repair, not a substituted CI pass. No broad sweep or native WebKit/Tauri witness occurred. ROOT owns independent review, clean candidate/hosted gates and any native verification.

## Resources and canonical evidence

All probes/backchecks completed sequentially. Vite5174 stopped with expected exit130; neither5174 nor5175 has a listener. No native process was touched.

Canonical payload: this RETURN, DIAGNOSIS_RETURN, PROVENANCE, CANDIDATE, COMMANDS, the three probe script versions, and raw evidence under `_run_records`: exact representative CI trace/context plus all-six log and hashes; CI browser identity/action extract; initial probe record/log; failed enrichment log; compressed enriched probe `focus-probe-r3.json.gz` with PROBE_ARCHIVE identity and launch log; compact FOCUS_CAUSAL_SUMMARY; failed/passed unit logs; final TypeScript; both browser logs/reports; commands and CHECK_RESULTS. The uncompressed 4.7MB enriched-probe JSON is a redundant local duplicate and need not be staged. Temporary new traces/screenshots and the whole downloaded CI artifact directory are not canonical.

Maintained files: EngineeringTable.tsx, OverflowRail.tsx, new OverflowRail.test.tsx and b4-table-editing.spec.ts under Piping desktop. All four hashes were checked at freeze. Return resource ownership to ROOT/manager now.
