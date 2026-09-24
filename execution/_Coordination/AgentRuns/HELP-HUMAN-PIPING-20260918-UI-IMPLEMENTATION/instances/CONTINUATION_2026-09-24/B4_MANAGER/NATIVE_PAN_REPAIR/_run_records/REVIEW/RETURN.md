# Independent frozen native pan repair review

Reviewer `/root/native_pan_manager/pan_review`, TASK Type 2, parent `/root/native_pan_manager`, delegated-harness-native; allocation gpt-6-astra/xhigh. Implementer `/root/native_pan_manager/pan_repair` used gpt-6-astra/low. The reviewer did not implement this repair and created no descendants. This is independent-agent review with the same model family, not model diversity.

**Disposition: suitable for bounded manager fan-in. No actionable code findings or protected-oracle weakening found in the complete frozen three-file diff.** Native verification remains outstanding and prevents claiming the reported native defect closed. This return grants no lifecycle acceptance, Git, release, or practitioner-usability authority.

## Candidate and scope

Base/current HEAD is `5112636cb8c8c25bab352cda6dfbdbbf1aa7d086`. The current scoped diff is byte-identical to `../TASK/candidate.diff`, SHA-256 `94ee4d02cc05bcba668f665575310e86af7e912addfa61257adaea81d33f3a9b`. All three current files exactly match `../TASK/FREEZE.json`:

| File under apps/desktop/src/features/workspace/table | SHA-256 |
|---|---|
| EngineeringTable.tsx | 9a6bbecbb166ea989f239d231ec7a77266eca41a2d3d502d6146c4c07d56c370 |
| OverflowRail.tsx | 4406f87b20fb4bac4eb5b3c3b7f707d8fcf5fc2e6789f400e7150c15aae3adfb |
| ModelTree.table.test.tsx | 1942a46f62dec06e38d0804e1cd75a32ccea4e19946d0c675d29c082a9bf99e8 |

`validate_change_scope.py` passed using these three explicit allowed paths and the frozen candidate path list; the live desktop diff contains exactly those paths. Scoped `git diff --check` passed. A read-only comparison confirmed no desktop path differences between native symptom source `1c1b8b072a25a5014f41dcc0e0fd8fd9d1ab0a9a` and the base HEAD. All 14 TASK evidence-manifest entries and all 277 manager browser-source entries match their current bytes. These checks do not review unrelated concurrent documentation changes.

## Behavior reviewed

- `OverflowRail.tsx:10–13` cancels the pointer default and synchronously focuses the actual button with `preventScroll`. `EngineeringTable.tsx:364–367` and `OverflowRail.tsx:72–75` attach it to both directions of all three pan families. Activation remains in the existing click handlers, preserving keyboard-produced clicks. The helper stores no guard or owner token that could survive an abandoned pointer sequence or later project transition.
- `EngineeringTable.tsx:160,329–335` still exempts only the current table's owned chrome or its explicitly owned cell/action destinations. A pointer focus transfer now supplies that existing ownership branch before the click changes view position. Ordinary editor blur to null, an outside control, or a nonmatching owner still reaches Apply. No blanket null-blur suppression or model-operation change was introduced.
- Existing endpoint recovery at `EngineeringTable.tsx:142–157` and `OverflowRail.tsx:37–56` remains unchanged. It captures the focused activating button and limits recovery after synchronous disable to its connected, visible pair, with owner and outside-focus checks. The new explicit focus supplies the prerequisite on hosts that otherwise do not focus buttons on pointer activation. Column and rail changes remain presentation state; persistent editor identity and Apply/Cancel, Info and Details policies are unchanged.
- Caller tracing covered ModelTree's active table owner registration, toolbar/status owner propagation, persistent Material editor, quantity eligibility and structured Apply route. The new fixture waits for the real quantity to become editable and checks the actual numeric intent boundary. No App, model, engine, persistence, CSS, geometry budget, or VirtualList edit is present. Existing tests are retained; only new tests and the required `act` import were added.

## Verification evidence assessed

The corrected pre-repair `fail-first.log` records 18 failures, 11 passes and 9 skips. Six pointer cases fail owned focus; the other twelve independently record an unintended numeric material Apply after uncancelled pointerdown. Their payload carries `210000000000 Pa`, against captured `200000000000`. This supports the proposed source mechanism under the fixture's explicitly simulated default blur ordering. AX58/59 show the real native edited value and unintended canonical/history change, and AX60 shows Undo restoration; those records do not identify the native DOM event ordering.

`verification.log` records 96/96 focused tests passing. After test-only TypeScript corrections, `verification-final.log` records the final ModelTree suite at 38/38. The new 29-case matrix covers six pan controls with pointer and keyboard-produced clicks, pointercancel/drag-away, later ordinary null blur, outside/nonmatching ownership, and Info/Details draft preservation. Existing OverflowRail tests exercise synchronous disable recovery and refuse foreign/hidden recovery. The final `typecheck.log` is empty; successful exit is reported in the implementer's return, not independently encoded in that log. The earlier explicit TypeScript failures are retained and correspond to the corrected optional fixture access and unsupported query option.

Manager browser records bind the same frozen files and show all six retained classic compact cases passing under Google Chrome 153.0.8010.54 (47.4 s) and Chrome for Testing 148.0.7778.96 (28.6 s), each exit 0. Inspection of the retained test confirms nonzero scrollbar probes, pointer pair-focus assertions, Enter/Space endpoint handoff, retained editor/disabled Undo, disclosure behavior, and geometry assertions. The initial Chrome153 attempt failed before tests because the sandbox denied Vite binding to 127.0.0.1:5174; the later approved run passed. It is retained as a failed launch, not counted as test evidence.

Initial fixture/probe failures did not reach an editable input and are not defect reproductions. They and the TypeScript failure remain recoverable. This reviewer ran only read-only source, hash, scope and whitespace checks; no browser, native, build, heavy test, sweep or CI was started.

## Remaining limits and handoff

The new component matrix explicitly models blur/default-action ordering and supplies synthetic geometry. Its keyboard branch uses focus plus a detail-zero click, and its foreign/stale cases use nonmatching tokens, not a full project replacement. The retained browser scenarios use a valid Material provenance text draft; they do not replace a native numeric Elastic draft witness. Component spies check absence of Apply/queue calls and unchanged displayed canonical values, not an integrated undo stack; the browser scenarios separately check disabled model Undo.

ROOT must backcheck the repaired native binary with the original numeric Material draft, both column directions and the reachable toolbar/status controls, pointer pair focus and keyboard handoff, disclosures, and ordinary editor blur/Apply after abandoning navigation. Bind the actual native executable/source and retain draft, canonical value and history observations. Exact null-relatedTarget causality remains inferred until native event evidence exists; successful native behavior would establish the required repair outcome even without that telemetry. Required final candidate checks, DEC-025 sweep, CI and any affected independent review remain with manager/ROOT. Any subsequent product edit invalidates this freeze's coverage and needs affected revalidation/review.

The detailed supplied-source, inspection-level, evidence-hash and check provenance is in `MANIFEST.json`. Only this return and that manifest were written.
