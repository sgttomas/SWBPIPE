# C4 label modes and capacity — bounded decision package

Status: ROOT recommendation, awaiting owner ruling. B4.1 table work is independent. No label policy, protected benchmark or appearance criterion is changed by this package.

## Evidence

Adopted UX_SPEC_V1.md §4.5, in the20260916 UI design run, states Budget/All/Off, one label per3600 CSS square pixels, selected/hovered plates always shown, the current row's node always labelled, and labels never overlapping. The same paragraph says restraint/load plates follow label mode. Source SHA2562141c1e844109d4869287048c695e79677f423acae965d5dd0436b7f6aeff682, line477.

C4's read-only source inventory is retained at _run_records/C4_PREPARATION_RETURN.md (SHA256b5294fef754bb3a820e30b32912caf4f6b45d507c5e98fa53a3f559a011d8fdb). Existing production is a boolean label toggle and capped80 candidate helper. The historical first profile assumes Off produces zero labels; it remains immutable. The redesigned second profile is not frozen.

Two limits require a clear product rule. Off can mean no ordinary labels while inspection context remains, or literally no labels. Also, if139 context labels are requested at a138-label Budget boundary, a hard138 cap and every requested label shown cannot both hold. Finite drawing space creates the same conflict with a never-overlap requirement. These are logical boundary examples, not measured performance results.

## Recommended dispositions

1. **Off retains inspection context.** Hide ordinary annotations, but retain selected/hovered plates and the current row's node annotation where placement is possible. Hide still takes precedence: an explicitly hidden entity gains no label. Preserve exact existing data semantics; this does not invent missing restraint/load plate capabilities.
2. **Capacity and clarity take precedence over an impossible always-visible promise.** Treat context annotations as first priority for placement. In Budget, the computed cap applies to actual displayed annotations. Never overlap labels or cover geometry's required picking target to force more into the view. When capacity/space is exhausted, report the number of requested annotations not shown and retain their identities for inspection; do not describe them as hidden geometry. Primary, hover and current-row context precede remaining selected annotations. In All, remove the nominal count cap, retain collision/containment rules and report any unplaced annotations.

This explicitly qualifies “always show” under insufficient capacity/space. It preserves3600-area budgeting, actual rendered counts, geometric/picking and non-overlap requirements. The alternative is to let required labels exceed the Budget cap or overlap, which ROOT does not recommend without a separate capacity/performance decision.

## Implementation and reversal

On approval, record this as the exact C4 disposition and implement it in the new label policy/control and connected scenarios. Keep a separate count for unplaced annotations; C3's hidden count continues to mean actual Hide only. First-profile inputs/oracles/fixtures remain byte-identical. The second-profile admission policy reflects the adopted semantics but stays DRAFT_UNFROZEN until the already required owner package/freeze before any timed qualification.

Mode/priority decisions are localized in the new label policy, so either disposition can be revised without changing canonical model operations, history or result standing. No solver, schema, model-data, release or independent-usability hold changes.

