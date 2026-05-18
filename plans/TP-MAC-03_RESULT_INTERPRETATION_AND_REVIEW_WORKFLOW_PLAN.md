---
doc_id: TP-MAC-03
doc_kind: implementation.plan
status: implemented
created: 2026-05-10
closed: 2026-05-10
---

# TP-MAC-03 Result Interpretation And Review Workflow

## Purpose

Move from "computed preview results exist" to "a user can inspect, understand,
and review result meaning."

This tranche continues the TP-MAC physics-first product path. It does not start
with endpoint-j force/moment recovery. It first builds the interpretation and
review workflow that will make endpoint-j, station recovery, load-combination
results, and richer stress recovery useful and inspectable later.

## Scope

- Add desktop result selection and detail review.
- Derive a desktop-side result interpretation view model from current
  `MechanicsResult.results[]`.
- Show result metadata: family, entity ref, value/unit, component, coordinate
  system, endpoint/location, recovery basis, sign convention, linked
  diagnostics, linked knowledge, source run/audit context, and professional
  boundary.
- Link selected result rows to model/viewport context using `entity_ref`.
- Extend review-only proposal/context flow so selected results can produce
  read-only review explanations.
- Add a mechanics gap ledger covering endpoint-j recovery, station recovery,
  pressure-to-frame load conversion, thermal behavior, support stiffness
  completeness, load combinations, and protected rule/code checks.
- Preserve all TP-MAC boundaries: invented/cleared data only, no hidden
  defaults, no accepted-state mutation, and no compliance, certification,
  sealing, approval, release-readiness, or production-readiness claims.

## Deferred

- Endpoint-j force/moment recovery remains deferred until this interpretation
  workflow creates a concrete consumer for both element ends.
- Formal DEL-08 report-generator/report-section promotion remains deferred
  unless a governed calculation-report artifact is explicitly needed.

## Implementation Notes

- Keep the public solver/result contracts stable for this tranche.
- Do not change `ResultItem.metadata` semantics.
- Use derived desktop/service data for interpretation unless a missing link
  cannot be derived from the current result envelope.
- Introduce stable desktop-only types as needed:
  - `SelectedReviewTarget` for result, diagnostic, or model entity selection.
  - `ResultInterpretation` for result detail and linked review context.
  - `MechanicsGap` for unsupported or deferred mechanics capabilities.
- Result row selection should preserve direct model-tree selection behavior.
- Selecting a result should highlight/select its model entity when `entity_ref`
  maps to a known pipe, node, support, or component.
- Agent proposals and review narratives remain review-only and non-mutating.

## Acceptance Criteria

- Clicking `result:force:pipe-P-120:axial` renders a result detail panel with
  `axial_force`, `element_local`, `end_i`, recovery basis, sign convention, and
  `pipe:P-120`.
- Selecting a result updates/highlights model context where possible.
- A review narrative references the selected computed result id and remains
  non-mutating.
- A mechanics gap ledger lists endpoint-j recovery as deferred/not implemented,
  not as a compliance failure.
- Existing report-packet and DEL-14-02 audit context remain intact.

## Verification

- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_analysis_run_records.py`
- `git diff --check`
- Rerun `apps/desktop/SMOKE.md` in the browser after UI changes.

## Closeout

TP-MAC-03 is implemented for the current desktop preview workflow.

Delivered:

- result-row selection and detail review;
- desktop-only `ResultInterpretation` derived from
  `MechanicsResult.results[]`;
- result metadata display for family, entity ref, value/unit, component,
  coordinate system, endpoint/location, recovery basis, sign convention,
  linked diagnostics, linked knowledge, DEL-14-02 source run/audit context, and
  professional boundary;
- result-to-model context linkage through `entity_ref`;
- selected diagnostic interpretation with affected refs, linked computed
  results, linked knowledge, review-only explanation, and model context linkage
  where affected refs resolve to known entities;
- selected-result and selected-diagnostic review narratives that remain
  non-mutating;
- mechanics gap ledger covering endpoint-j recovery, station recovery,
  pressure-to-frame load conversion, thermal behavior, support stiffness
  completeness, load combinations, and protected rule/code checks.

Acceptance evidence:

- selecting `result:force:pipe-P-120:axial` displays `axial_force`,
  `element_local`, `end_i`, recovery basis, sign convention, and `pipe:P-120`;
- selecting that result updates model/viewport context to `pipe:P-120`;
- selecting `diagnostic-HIGH_DISPLACEMENT_REVIEW` displays affected refs and
  linked `result:disp:node-N-140` value context, and updates model/viewport
  context to `node:N-140`;
- generated review narratives reference the selected result or diagnostic and
  leave proposal acceptance disabled;
- endpoint-j recovery is recorded as deferred, not as a compliance failure;
- report-packet and DEL-14-02 audit context remain intact.

Final verification was run on 2026-05-10:

- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_analysis_run_records.py`
- `git diff --check`
- browser smoke against `http://127.0.0.1:5173/`

Endpoint-j and station recovery remain deferred until a new plan defines a
concrete consumer for both element ends and station-level result inspection.
