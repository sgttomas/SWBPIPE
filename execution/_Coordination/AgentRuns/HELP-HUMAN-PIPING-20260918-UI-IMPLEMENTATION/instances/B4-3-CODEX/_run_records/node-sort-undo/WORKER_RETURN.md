# Node sorted-review Undo — diagnosis-only return

**Confirmed in the pinned desktop source browser:** Node review sorted-X editing can lose browser-native text Undo when the active inline input's ancestor actually moves downward. Production source remains unchanged. No repair is included or authorized by this return.

Retained TASK `/root/b3_manager/b4_3_materials`, Codex harness-native Astra/low, parent `/root/b3_manager` WORKING_ITEMS Astra/high. Base `53acac28c4e8554c708d860062fe95408e5ee1de`, product `75fa863ea5cfee9bfe8f5a6ebf0f31047663e4e5`. Applicable diagnosis skill retained; no descendants. Scope/resource restrictions instruction-enforced on unrestricted host.

## Actual observation

Public precision fixture retained its model envelope; initial X coordinates were explicitly set to first2, second15, remaining102+ to make the crossing unambiguous. Ascending Node review X sort: actual key1 staged the first row, Keep draft, reopen, Right Arrow collapsed selection to caret[1,1], actual key9 produced19, then ControlOrMeta+z.

| State | Text | Caret | First two rows | Input ownership |
|---|---|---|---|---|
| Reopened staged baseline | 1 | [1,1] | node00000, node00001 | original input focused |
| Actual key9 | 19 | [2,2] | node00001, node00000 | same input focused |
| Actual text Undo | **19** | **[2,2]** | **node00001, node00000** | same input focused |

The active input moved down exactly36px (Y350.984375→386.984375). A passive child-list MutationObserver counted the actual active wrapper removed once and added once. No focus/fill rescue, replacement conversion, DOM mutation hook or production behavior instrumentation was used. All movement, same-input, focus, caret-after-key, canonical/hash/history preconditions passed. The test failed only at the final expected Undo text1 assertion. Raw `BROWSER_DIAGNOSIS_2.json/log`, trace and screenshot retain the complete observation; `OBSERVATION_2.json` is a compact extraction.

Canonical X stayed2 before and after test-owned Clear. Model hash stayed `sha256:0abe13b6b72512ef8944aa6711b5cdaf76287c10af4eccea205257c621d5bd0f`. Model Undo and Redo remained disabled; edited-marker count0. The sole test-owned review draft and sort were cleared before failure assertions. No canonical mutation, queued operation, save or model-history checkpoint occurred.

The first run is preserved as a setup failure/control observation, not a reproduction: macOS End left selection[0,1], so key9 replaced1 with9, did not cross15 and caused zero host moves; actual Undo returned1. New observed selection data justified the sole retry using real Right Arrow. Movement/caret/Undo oracles and timeouts were not weakened.

## Earliest divergence and cause

Measured divergence is browser-native Undo failing after the actual active-host move, despite retained DOM identity/focus/caret. The source path is:

1. `ModelTree.tsx:727–729` supplies raw Node review drafts as row values.
2. `EngineeringTable.tsx:251` updates the editor and lifted draft on input; `:90` immediately sorts the visible draft value.
3. Node review at `ModelTree.tsx:849` has no persistentEditor opt-in. `EngineeringTable.tsx:291–296` therefore keeps its input inside the reordered row.
4. `VirtualList.tsx:128–131` preserves typed row keys but React moves the keyed wrapper; the observer proves that actual active wrapper movement here.

High-confidence causal inference: moving the native input's ancestor invalidates its browser editing Undo history. This agrees with the separately retained Materials isolated DOM-move probe (`worker/undo-dom-probe-2.log`), which showed the same-node nonmoving control Undo succeeds while same-node append/refocus Undo fails in the same Chromium version. No Node production counterfactual fix was executed. Native Tauri/WebKit behavior, other sort directions/fields, and broad qualification are not established by this single-profile diagnosis.

## Minimal options for parent/ROOT disposition

1. **Recommended bounded correction:** opt only the Node review `EngineeringTable` consumer into the existing reviewed `persistentEditor` host. No new editor, core/VirtualList mutation or Node direct opt-in is implied. This preserves immediate staged sorting and stationary native input ownership. Backcheck the exact consumer change; use this crossing test plus affected Node review character-start, Keep/Cancel/Queue/Clear, filter/virtual pin, hidden/inert/resize and focus/ownership evidence. A separately granted actual native Node witness remains necessary for a native claim.
2. **Explicit deferred reordering:** keep row order fixed while a review editor lives, mark requested sort as deferred, then apply it on editor exit. This avoids moving the inline input, but changes live-sort/navigation semantics and needs new state/copy/oracles. It is broader than reusing the existing host and requires an explicit design disposition; merely showing an obsolete order as currently sorted is not honest.

No automatic Node direct expansion is warranted by this observation: its ordinary edit buffer is not the live staged review sort basis. No claim of universal Node direct correctness follows either.

## Boundaries and artifacts

Only maintained change is the clearly named final diagnostic test in existing `e2e/b4-table-editing.spec.ts`. It is intentionally red on unchanged product and requires parent/ROOT disposition before any mergeable candidate. Exact diagnostic/source binding2 SHA256 `0b25586f818c0221f4c4dcb6455e8fd16dbbbd18203a04f07fe91d9bb78d8542`; all bound production hashes match. Initial binding/log/trace remain preserved separately.

One desktop profile, one worker, shared lock, source5174, pinned Chromium148.0.7778.96, executable SHA256 `2447c7bdea8e8ad38b52594f1ba53e18821c05445e8a07ad1cf3b0f23931d89d`. Collection and exact commands retained; JSON reports include actual browser identity. Final source listener and shared lock absent. No production repair, Git mutation, native/CUA, build, full suite, additional delegation or acceptance claim. Parent/ROOT now decide repair and later verification.
