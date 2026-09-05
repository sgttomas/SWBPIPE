# T4 renderer integration return v3

Status: IMPLEMENTED_FROZEN_REVIEW_PENDING. Ephemeral Agent2, parent N1_WI_PKG07; no delegation. Basis: sealed RENDERER_BRIEF_V3.md, accepted T4_ACCEPTED_SNAPSHOT_V2 and RENDERER_HANDOFF_PREIMAGE_V3.json serial B0 handoff. This is derivative implementation evidence, not decomposition authority or lifecycle acceptance.

Scope: exact five transferred renderers plus new rendererIntegration.test.tsx. Preserved B0 predecessor edits and completed them. Accepted display module three files verified byte-exact against SOURCE_MANIFEST_V2.json; B0 helper toolkit/ResultQuantity.tsx read-only/unchanged by this child. No App/types/styles/service edits. Full HEAD-baseline five-renderer diff plus new test captured in RENDERER_DIFF_V3.patch; hashes in RENDERER_SOURCE_MANIFEST_V3.json.

## Coverage and source binding

| Surface | Typed source | Rendered behavior |
|---|---|---|
| ResultsPanel result rows | item.value/unit/dimension | Shared quantity preference; absent dimension explicit unknown fallback. Separate Entered source column preserves original value/unit. |
| ResultsPanel selected interpretation | ResultQuantity lookup by result ID | Selected value from original typed row; serialized interpretation remains exact. |
| ResultsPanel endpoint pair | ResultQuantity lookup for each endpoint result_id | Both end-i/end-j values converted independently; no parsed value_label. |
| ComparisonPanel reference/target | delta.left_value/right_value/unit, compatible original result dimension | New readouts for both absolute values. Metadata borrowed only if right result value+unit match and available left result does not conflict. Missing/stale/conflicting metadata is explicit unknown fallback. |
| ComparisonPanel raw delta | delta.raw_delta/unit | Absolute temperature becomes temperature_interval only for delta; no delta recomputation or source comparison mutation. |
| KnowledgePanel max displacement | summary.max_displacement | Explicit displacement semantics from accepted summary field; source summary string remains immutable. |
| KnowledgePanel axial force | original selected result | Explicit row dimension preferred; accepted axial-force kind permits force where absent; otherwise unknown. Authored records are never rewritten merely for sharing a computed-record ID. |
| DiagnosticsPanel linked values | ResultQuantity by linked result ID | Typed source readouts instead of parsing value_label; diagnostic/interpretation evidence remains exact. |
| RuleCheckRunPanel candidate options | row.value/unit/dimension, useDisplayQuantity | Options remain valid text-only option elements. Preference never changes selected ID or pack JSON. |
| RuleCheckRunPanel authored reference preview | row.value/unit/dimension | Converted visible candidate/reference values; missing dimension fallback. |
| RuleCheckRunPanel outcome computed/limit | computed_value/limit_value.value, dimension, unit_ref | Added with manager explicit same-file authorization. Unsupported unit reference retains entered quantity and service notice; no lookup guess or source outcome changes. |

Source unit metadata labels now distinguish source evidence from selected readouts. Result table column formerly Dual (approximate) is Entered source, matching its actual contents. Editable inputs and serialized source/evidence text are unchanged. No global formatting, DOM replacement or numeric TS conversion.

## Verification
Desktop command `npm test -- --run src/features/display-units/rendererIntegration.test.tsx src/features/rule-check/RuleCheckRunPanel.test.tsx`: PASS 22/22 (4 new renderer scenarios, 18 existing rule-check scenarios), 2026-09-05 22:58:51 execution clock. New tests cover actual visible quantities across all five surfaces, SI/US/Entered switch, endpoint pair and diagnostic/knowledge output, unknown dimension, stale comparison source, absolute versus interval temperature request, rule selection and pack preservation, unsupported outcome unit fallback. JSON source model/result/knowledge/comparison/outcome and generated interpretation/evidence are compared byte-for-byte before/after.
Project command `./node_modules/.bin/tsc -p apps/desktop/tsconfig.json --noEmit`: PASS. An earlier run exposed fixture ObjectRef keys (corrected here) and an external B0 App.test options error (reported to manager, subsequently fixed by owner); no external fixes performed by this child.

## Handoff
Source is frozen for fresh independent read-only review, then manager fan-in; final integration review remains required. All six owned files included in diff and hash manifest. No lifecycle/D58 release or source-authority acceptance claimed. Required reruns: reviewer findings remediation if any, manager integrated tests/review. Existing source-only evidence/metadata is intentionally not rewritten by the display preference. Unsupported catalog references/dimensions stay explicit fallbacks, not falsely converted. Derivative status: v3 evidence/current source complete for review, broader project packages parent-owned.
