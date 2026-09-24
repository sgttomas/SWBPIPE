# Owner decisions — current continuation

Source: actual owner responses to the three questions in this task, 2026-09-24 UTC. Supplied question/answer text is recorded below; this is not a transport-byte custody assertion. These are actual owner decisions, distinct from the earlier manager proposals.

## C4 Off mode

Question: “C4 label decision: in Off mode, should ordinary labels disappear while selected/hovered/current-row context can still appear? Explicitly hidden geometry would remain hidden. This resolves the owner-held choice in C4_LABEL_DECISION_PACKAGE.md.”

Owner answer: **“Retain inspection context (recommended)”**.

Applied ruling: Off hides ordinary annotations, retaining selected/hovered/current-row context where placement is possible. Explicit Hide still takes precedence. No absent engineering annotation capability is invented.

## C4 capacity

Question: “C4 capacity decision: when labels cannot all fit, should Budget keep its cap and prevent overlap, prioritize inspection context, and report omitted annotations? All would remove the count cap but still prevent overlap. This qualifies the adopted ‘always show’ promise, so it needs your ruling.”

Owner answer: **“Allow context labels to exceed the cap; keep non-overlap”**.

Applied ruling: context annotations may exceed the nominal Budget count; non-overlap remains required. This rejects the earlier proposal's hard cap on total displayed annotations. Finite spatial capacity can still prevent placement; report actual omitted annotation identities/counts separately from hidden geometry. ROOT's implementation interpretation, to be checked by design preparation: place primary/hover/current-row and remaining selected context first; ordinary annotations fill nominal capacity still available; no ordinary overflow. All removes the nominal count cap while retaining collision, containment and picking constraints. Do not silently change protected benchmark workloads/oracles/numeric limits; assess the new overflow behavior and retain the required second-profile owner freeze.

## Bounded live-controller activation

Question: “Authorize the bounded live-controller tranche now? The current proposal connects development Codex through a local JSON CLI to Piping’s existing controller for inspect/preview/submit/status, starting with single and atomic batched Node-coordinate edits. Apply stays with you; recovery is limited to the same running controller session, and completion includes real human single/batch witnesses. Runtime/App configuration changes are unnecessary. This is the explicit activation held by DEC-042/DEC-103, separate from UI continuation. Reviewable package: projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/DESIGN_MANAGER/RETURN.md.”

Owner answer: **“Authorize this bounded tranche (recommended)”**.

This activates the bounded scope in that current package and the retained LIVE_CONTROL_ACTIVATION_PROPOSAL revision 2 / PIPING_LIVE_CONTROL_CONTRACT_DRAFT revision 3. It includes Piping controller publication acknowledgements, native private bridge and JSON CLI, inspect/preview/submit/status, Node coordinate single/atomic-batch witnesses, and actual human review/Apply. External tools cannot Apply. Recovery is within the same controller session; restart expires handles or reports unknown outcome for reconciliation. It does not activate embedded Runtime, CAEPIPE operation/correlation, arbitrary agent proficiency, new result schemas, engineering acceptance or release.

Execution sequencing stays compact fit first, then shared-controller handback/wire acknowledgement. Disjoint native/CLI implementation may proceed against the frozen wire while independent Piping work continues. One writer owns App/workspaceSession; B4 manager owns desktop tests/browser/native resources until explicit release. The live branch is now an activated bounded milestone with implementation/native integration and a required real-human witness; its completion is not inferred from UI merge or automated CUA. No new approval is required for authorized implementation, review or ordinary Git operations.
