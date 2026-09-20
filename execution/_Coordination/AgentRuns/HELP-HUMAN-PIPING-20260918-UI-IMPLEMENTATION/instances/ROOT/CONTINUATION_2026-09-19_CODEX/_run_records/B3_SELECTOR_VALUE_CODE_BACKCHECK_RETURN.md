**PASS — all three selector findings are closed on this candidate. No additional actionable issue found in the five-file repair.**

Reviewed `bf15aed87571df3601b99c350fc5296fb337cd91` against `41098f1e18edf58c59b2694119ea76a37ab9a1bc`. The clean wt2 candidate matches manager checkpoint `24f5d9db073fae34d165029f33d1a8bf8f9d0a3c`.

- **Passive dismissal:** initial highlighting no longer counts as an explicit choice. Tab/outside dismissal preserves missing, disabled, null, and unsupported source values. Explicit navigation, Enter, and option clicks can still commit.
- **Option identity and races:** pending intent uses the canonical value. Reordering preserves that identity; membership, label, availability, or controlled-value changes cancel it. Commit-time validation checks the current data again. Pointer callbacks use their rendered option’s value rather than a stale index; the added regression exercises reordering before reconciliation.
- **Shared-section assertions:** both retain the original empty-selection expectation through `data-value=""`. The existing browser scenario passes at both configured sizes.

The source changes leave popup Escape ownership, focus/resource cleanup, New Blank reset, engineering callbacks, geometry, and native responders unchanged.

All four indexed evidence hashes match. Retained evidence reports:

- **6 original family-preservation failures**, preserved unchanged.
- **75 focused tests passed.**
- **88 files / 1,436 unit tests passed.**
- **8 focused browser cases passed**, including both Shared-section cases.

The sealed brief and evidence-index hashes match, and `git diff --check` passes. I ran no tests, UI, writes, Git mutations, or delegation.

This repair is suitable for code fan-in. Full source/dist lanes, final native evidence, the separate fidelity backcheck, ROOT’s affected inspection, clean sweep, and actual-candidate CI remain required before B3 merge. No Runtime adoption or final product acceptance is established.

Independent TASK, parent ROOT; existing Astra/xhigh allocation and telemetry limits apply. Standard F-PIP-2 / DEC-081 claim fence applies.
