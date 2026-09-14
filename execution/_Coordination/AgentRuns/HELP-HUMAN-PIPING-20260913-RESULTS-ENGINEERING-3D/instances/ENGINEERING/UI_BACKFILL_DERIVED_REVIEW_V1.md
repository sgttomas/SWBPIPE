# Additive UI derived-review availability mapping V1

Design backfill only, overriding incomplete scalar-review availability in UI_BACKFILL_V1.md. Root relays this to UI; no sibling acceptance, live control or Current-result claim.

| Input / evidence | Display meaning |
|---|---|
| Exact profile has any region; max_open_formula_stress=null and EXACT_PRESSURE_STRESS_SUMMARY_UNAVAILABLE | Governing scalar stress summary unavailable; physical component results remain available. Never show zero/ratio fallback. |
| EXACT_PRESSURE_COMPONENT_STRESS_REVIEW_UNAVAILABLE Diagnostic.refs=[case,region,component,pipe,endpoint-node], endpoint token in ID/message | This particular component/pipe-end/case derived scalar modifier review is unavailable. Show retained entered SIF/flexibility and source as input provenance; no numeric review value or applied badge. Existing warning code/refs are the handoff carrier, not a new model state enum. |
| Surviving nonpressure component_user_stress_multiplier_review row | Display its existing MPa stress value and explicit source references with exact-profile material provenance. A component can have available and unavailable sides; never collapse availability by component alone. |
| Producer component_stress_modifier_count / report evidence count | Count actual emitted/apparent serialized numeric reviews under each existing scope. Zero is a count, not an assertion of complete coverage, absent entered modifiers or zero stress. Display unavailable warning coverage alongside it; no synthetic unavailable-count field required. |
| Selected/rule old suppressed user-multiplier ID | Missing/unavailable result with disclosed cause; do not redirect selection to an exact stress surface or another pipe side. |

UI/report projection joins warning case/region/component/pipe/endpoint-node references to bound contract evidence and authored geometry. Persist the warning with the proposed future exact envelope; no Lamé scalar computation occurs in the UI. This requires future runtime report/consumer integration, independently of professional 3D navigation/routing design and current result-v0.2 repairs. Existing legacy and authentic Historical packets continue through their original contract.
