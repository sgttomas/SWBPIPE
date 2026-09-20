**FINDINGS — three P2 issues remain in the compact-selector delta.** The New Blank reset has no additional actionable finding.

Reviewed all 28 desktop files between `5ef9de29179a9b814da81d17c2774663404ac93d` and frozen `41098f1e18edf58c59b2694119ea76a37ab9a1bc`. The clean candidate matches manager checkpoint `9d0b717a664daf5c6f6236b5157982ff0c54e3de` in desktop scope.

1. **P2 — dismissing an unavailable current value silently chooses a replacement.**  
   [CompactSelect.tsx:79](/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt2/projects/chirality-piping/apps/desktop/src/features/workspace/CompactSelect.tsx:79) activates the first enabled option when the current value is absent or disabled. Tab/outside dismissal then commits that option through line 67, even without navigation or an explicit choice. Concrete consumer: a missing, null, or unsupported Support family has a disabled “preserved” placeholder. Opening it and pressing Tab—or clicking Queue—changes the draft family to **Anchor**. This violates the existing exact-source-until-explicit-choice contract. Separate initial highlight from an intentional pending selection, and test unavailable/disabled values through Tab and outside dismissal as well as Escape.

2. **P2 — option updates can retarget a pending choice by index.**  
   [CompactSelect.tsx:65](/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt2/projects/chirality-piping/apps/desktop/src/features/workspace/CompactSelect.tsx:65) reads the latest options using a previously stored index. The options effect at line 131 accepts any enabled option now occupying that index. If options reorder or are replaced while open, Enter/Tab/outside dismissal can commit a different value from the highlighted choice originally made. Native unit-catalog loading and model-derived section lists are dynamic consumers. Preserve the pending choice by value, or cancel/rebase it when its options/value basis changes; never silently substitute another row. Add a regression with an open popup and reordered/removed options.

3. **P2 — two migrated browser assertions still target native input values.**  
   [ui-foundation.spec.ts:1126](/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt2/projects/chirality-piping/apps/desktop/e2e/ui-foundation.spec.ts:1126) and line 1144 call `toHaveValue("")` on Shared section, now a button combobox. The installed Playwright implementation rejects elements other than input, textarea, or select with “Not an input element.” Preserve both empty-selection assertions using the control’s canonical `data-value`; do not remove them. The focused 22-case run does not exercise this existing scenario.

The New Blank reset occurs only after the successful current-request check. It resets stage/view memory, page/tool state, drawers, and restoration references while preserving UI preferences. Failure and supersession paths remain outside that reset boundary. Selector substitutions retain existing engineering callbacks; shared `TextField` migration is explicitly opted in, and unrelated viewport display controls, rendering, picking, and mutation routes are unchanged.

Verification and limits:

- Brief, evidence index, selector disposition, and fidelity return hashes match.
- Indexed logs match and report **88 files / 1,410 unit tests** and **22 focused browser cases** passing.
- `git diff --check` passes.
- The remaining test adaptations use actual rendered controls and preserve their value, operation, geometry, and timeout expectations.
- Findings are source-based; I ran no tests, UI, servers, writes, or delegation.

This delta needs repair and backcheck before fan-in. Final native, full source/dist lanes, affected fidelity/ROOT inspection, clean sweep, and actual-candidate CI remain pending. No Runtime adoption or final B3 merge suitability is established.

Independent TASK, parent ROOT; existing Astra/xhigh allocation and telemetry limits apply. Standard F-PIP-2 / DEC-081 claim fence applies.
