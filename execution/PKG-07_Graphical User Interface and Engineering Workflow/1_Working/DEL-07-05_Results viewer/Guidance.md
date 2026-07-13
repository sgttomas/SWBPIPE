# Guidance: DEL-07-05 Results viewer

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-07-05-DECL-003`.

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

The current result surface already provides bounded filtering, grouping, tabular review, and a translational deformation overlay. Surviving implementation work should extend that slice without changing the boundaries in this document, including decisions about:

- result category navigation;
- load case and combination selection;
- envelope/range selection;
- node, element, support, and equipment-load targeting;
- tabular values versus graphical overlays;
- rotational-deformation visualization for emitted `rx`/`ry`/`rz` rows, which is owned by DEL-07-05 but not implemented in the current overlay;
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
| Exact UI layout, component library, state library, and broader overlay behavior | TBD; rotational-deformation visualization is a named DEL-07-05 residual under DEC-074 O1 / PDU-061 |
| Exact rule-ratio terminology when private rule packs differ by user design basis | TBD |
| Exact equipment-load aggregation/display semantics | TBD |
