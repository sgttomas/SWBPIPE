# Guidance: DEL-07-05 Results viewer

## Purpose

The results viewer should make analysis outputs reviewable without hiding the conditions that make those outputs trustworthy, incomplete, or unsuitable for professional reliance. Its job is to help users inspect mechanics results, user-rule-check outputs, warnings, assumptions, and report/export readiness; it is not a solver, code interpreter, or professional approval mechanism.

## Principles

- Keep result categories visible and separable: displacements, rotations, forces, moments, reactions, equipment loads, stresses, and ratios are related but not interchangeable.
- Treat result envelopes and diagnostics as the boundary of authority for what the GUI may show.
- Preserve unit and dimensional context at every numerical display.
- Show missing solve-required data and rule-check-required data as findings, not as empty success states.
- Treat stress ratios as user-rule-pack outputs. Do not invent thresholds, allowables, categories, or pass/fail meanings.
- Preserve export/report traceability: visible result review should align with reproducibility metadata instead of becoming an isolated screen state.
- Keep professional-boundary notices close enough to result status that software output is not mistaken for certification, sealing, approval, or code compliance.

## Considerations

The result surface is broad enough to require filtering, grouping, and cross-highlighting in a future implementation brief, but this setup does not choose the exact interaction design. Future implementation should decide how to organize the following without changing the boundaries in this document:

- result category navigation;
- load case and combination selection;
- envelope/range selection;
- node, element, support, and equipment-load targeting;
- tabular values versus graphical overlays;
- unit display and conversion controls;
- diagnostic badges, warning panels, and blocked states;
- report/export readiness indicators.

## Trade-offs

| Trade-off | Guidance |
|---|---|
| Dense tables vs. graphical overlays | Use each where it supports review. Tables support exact inspection; overlays support spatial pattern recognition. Both must remain unit-aware and diagnostics-qualified. |
| Ratio display vs. code-compliance claims | Ratios may show user-rule-pack calculations when inputs are complete. The UI must not turn those ratios into automatic professional approval or public code compliance claims. |
| Convenience filters vs. hidden warnings | Filters should not suppress blocking diagnostics or professional-boundary status without a deliberate, visible review state. |
| Export readiness vs. report generation | The viewer may expose report/export readiness signals, but report generation and structured result exports remain PKG-08 surfaces unless a later sealed brief says otherwise. |

## Examples

The following are structural examples only and include no engineering values:

- A load-combination selection reveals displacements and rotations at selected nodes, with unit labels and any solver diagnostics attached.
- A support selection reveals restraint reactions and active-state warnings when nonlinear support state is uncertain.
- A stress result view shows mechanical stress values when available and shows `RULE_INPUTS_INCOMPLETE` if a requested ratio depends on missing user rule-pack inputs.
- A report/export readiness indicator shows whether result hashes, solver version, rule-pack checksum, warnings, and assumptions are available for downstream reporting.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict identified during setup. | N/A | N/A | N/A | N/A | N/A |

## Open Issues

| Issue | Status |
|---|---|
| Exact result-envelope schema fields consumed by the viewer | TBD |
| Exact UI layout, component library, state library, and overlay behavior | TBD |
| Exact rule-ratio terminology when private rule packs differ by user design basis | TBD |
| Exact equipment-load aggregation/display semantics | TBD |
