---
run-id: WORKING_ITEMS_RUN_2026-06-13_TP-C2-DECLEDITOR-001
timestamp: 2026-06-13T15:25:00-0600
completed: 2026-06-13T15:55:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/apps/desktop/src
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C2-DECLEDITOR-001 — required-input / value-slot declaration form builders (Phase C2 slice 4)

## Tranche and authority basis

- Tranche: completion-plan Phase C item C2 remaining scope — the
  variable-declaration form builders for `required_inputs` and `value_slots`,
  the next unblocked item on the R3/Phase C dependency spine after slice 3
  (`TP-C2-TABLENODE-001`) completed structural authorability of the expression
  AST and routed the declaration/check form builders as the remaining C2 work
  (`plans/PLAN_2026-06-10_prd_completion.md` §3 C2; selected per the
  `_COORDINATION.md` Application Integration And Issuance Loop step 3.1).
- Authority: DEL-07-03 (Material, component, and rule-pack editors); PRD §14.5
  (rule-pack editor authoring surfaces); DEC-022 (typed AST is the sole
  canonical, checksum-bound grammar; AST-only authoring). **D-02b
  AWAITING_RULING** — no writable expression text syntax may ship, so the
  declarations editor is purely structured form controls (it authors variable
  *declarations*, never expression text). Document shapes cross-checked against
  the `RequiredInput`, `UserSuppliedValueSlot`, `QuantityIntent`, and
  `ProvenanceRecord` `$defs` in `schemas/rule_pack.schema.yaml` and the
  template `buildDraftRulePackDocument` in
  `apps/desktop/src/services/rulePackService.ts`. OPS-K-PRIV-1 (private rule
  packs stay local-only), CONTRACT no-silent-defaults, OPS-K-AUTH-1.

## Authority-vs-plan discrepancy surfaced and corrected (not the authority)

The slice-2/slice-3 records and the plan C2 row routed the residual as
"required-input / value-slot / **load-combination** form builders." The
authoritative `rule_pack.schema.yaml` top-level object is
`additionalProperties:false` and has **no `load_combinations` member** — load
combinations are a *model* concept (Phase A4 load-case manager), not a
rule-pack member. The rule pack's third user-authored document-structure
member (after `required_inputs` and `value_slots`) is **`check_definitions`**,
which ties `required_input_refs` + `value_slot_refs` + `formula_ref` into a
check. Per the `_COORDINATION.md` State-Tracking rule ("when the plan
disagrees with authoritative surfaces, surface the discrepancy and correct the
plan, not the authority"), this run lands the two real members and corrects the
plan/log wording to name `check_definitions` as the remaining C2 form-builder
sub-surface.

## Changes (app-owned slice; all under apps/desktop/src and e2e)

1. **`features/rule-packs/DeclarationsEditor.tsx` (new)** — a structured editor
   for the rule pack's variable declarations: its `required_inputs` and
   user-supplied `value_slots` (the named variables a formula's `variable_ref`
   binds to). Add / remove / edit each entry through form controls:
   - **Required input** fields: `input_id`, `name` (text); `source_kind`,
     `required_for`, `completeness_status` (closed-set `<select>`); the
     `quantity_intent` `dimension` (shared `DimensionSelect`) and `unit_ref`
     (text). **Add required input** seeds a schema-valid default with a fresh
     unique id; **Remove** is blocked at the `minItems:1` schema floor with a
     stated reason.
   - **Value slot** fields: `slot_id` (text); `slot_kind`, `value_status`,
     `required_for`, `completeness_status` (`<select>`); the `quantity_intent`
     `dimension`/`unit_ref`. Same add/remove + floor handling. No numeric
     allowable *value* is authored — `UserSuppliedValueSlot` carries a
     dimension/unit intent and a `value_status`, not a number, so the honest
     `value_status` marker (`not_provided` by default) stands in instead of an
     invented value.
   - **Enum vocabularies** (`SOURCE_KINDS`, `REQUIRED_FOR_TARGETS`,
     `COMPLETENESS_STATUSES`, `SLOT_KINDS`, `VALUE_STATUSES`) are exported
     consts copied verbatim from the schema, so the controls can only emit
     schema-valid tokens; a stored token outside the current vocabulary
     surfaces as a "(current) X" option rather than being silently snapped
     (CONTRACT no-silent-defaults).
   - **Lossless preservation:** a patched entry keeps every member it already
     carried (provenance, the const-true relaxation flags, missing-value
     diagnostic) via structural spread; a nested `quantity_intent` edit
     preserves the const-true `unit_required`/`dimension_check_required` flags;
     untouched sibling entries round-trip verbatim. The editor never drops or
     rewrites a member it does not expose.
   - Defaults reuse `draftPlaceholderProvenance()` so each added entry carries a
     fresh private-by-default placeholder provenance (`private_only`,
     `required_before_public_use`) — no aliasing, no invented source.
2. **`services/rulePackService.ts`** — extracted `draftPlaceholderProvenance()`
   (the existing template's placeholder provenance, byte-identical) and had
   `buildDraftRulePackDocument` call it, so the document template and the new
   declaration defaults share one provenance source. No change to the produced
   draft.
3. **`features/rule-packs/ExpressionComposer.tsx`** — exported the shared
   `DimensionSelect` for reuse by the declarations editor, and (review
   follow-up, see below) gave it the same "(current) X" escape hatch the
   declarations editor's `EnumSelect` uses, so an out-of-vocabulary stored
   dimension is surfaced rather than display-snapped to `"TBD"`. This also
   tightens the composer's own literal/table dimension fields.
4. **`features/rule-packs/RulePackManagerPanel.tsx`** — mounts
   `DeclarationsEditor` ahead of the `ExpressionComposer` inside the
   valid-parsed-draft branch (both read the same draft document; the editor's
   `onChange` re-serializes via a new `handleDeclarationsChange`); updated the
   header comment and the document-JSON textarea label to name the now-built
   declarations surface and the remaining raw-JSON members
   (`check_definitions`, diagnostics, metadata).
5. **Tests** — `DeclarationsEditor.test.tsx` (new): pure-helper tests
   (read ids, unique-id rule, exact schema-valid default shapes via `toEqual`,
   lossless array rewrite, enum set-equality) and component tests (add with
   fresh id; edit every field; nested `quantity_intent` edit preserves the
   const flags; remove blocked at the floor for both arrays; add-then-remove
   lossless; sibling preserved verbatim; out-of-vocabulary `source_kind` and
   `dimension` surfaced as "(current)"; disabled threading).
   `RulePackManagerPanel.test.tsx` extended with an integration test: a
   required input added through the editor appears in the composer's variable
   picker. `e2e/r2-smoke.spec.ts` "rule-pack manager" test extended to add a
   required input from blank and assert both the canonical document JSON growth
   and the composer variable-browser reflection.

## Validation

- Targeted Vitest `src/features/rule-packs` **43/43** (was 26; +17 net across
  the new `DeclarationsEditor` suite and the panel integration test).
- `tsc -b` clean.
- Full desktop Vitest **285/285** (13 files; was 268 at the clean baseline this
  session + 16 declarations + 1 dimension-honesty test). No `App.test.tsx`
  load-flake this run — the host was unloaded; the full suite passed on the
  first attempt before and after the shared-`DimensionSelect` fix.
- Playwright `-g "rule-pack manager"` **2/2** (chromium-desktop 1440×920 +
  chromium-compact 1280×800; the declarations editor driven from blank through
  visible controls at both viewports), re-run green after the shared-component
  fix.
- DEC-025 five-surface sweep recorded at the committed HEAD (summary committed
  alongside this tranche).
- Evidence posture (H4): a homogeneous extension of the rule-pack editor with a
  Vitest unit suite and the rule-pack Playwright journey; both were extended,
  so the default UI evidence posture is met without a manual-only exception.

## Pre-commit adversarial review and dispositions

An independent read-only reviewer ran four lenses over the diff and the
authoritative schema: (1) schema conformance, (2) governance / D-02b / IP
boundary, (3) React correctness, (4) test honesty. Three lenses returned zero
findings and affirmatively verified: order-and-set-equal enum vocabularies
against the schema; exact `RequiredInput`/`UserSuppliedValueSlot` default
shapes with `additionalProperties:false` holding and the const-true flags
literally true; uppercase `"TBD"` (not the prior-slice `"tbd"` defect); lossless
nested-`quantity_intent` edits (mutation-tested — dropping the spread turns a
test red); no writable expression text syntax, no invented numeric value,
private-by-default placeholders, no compliance/validity claim; no lost-update
between the two editors; `minItems:1` floor for both arrays; non-vacuous
lossless tests. Dispositions:

- **Should-fix fixed — `DimensionSelect` silent display-snap.** The shared
  `DimensionSelect` snapped an out-of-vocabulary stored dimension to `"TBD"`
  for display while the document still held the real token — contradicting the
  no-silent-snap principle the declarations editor's `EnumSelect` implements
  (and which a future-schema document is the only way to reach). Gave
  `DimensionSelect` the same "(current) X" option and bound `value={value}`;
  added a Vitest case opening a required input with an unknown dimension token
  and asserting it is surfaced, not snapped. Display-only, writes nothing
  unless the user picks; a schema-conformant document never hits the path.
- **Nit accepted (not blocking) — index-based row keys.** `key={`input-${i}`}`
  matches the existing composer/table convention; every field is controlled and
  re-derives from the document, so the index-key staleness class does not
  manifest. Left unchanged for editor consistency. (Empty `input_id`/`name`/
  `unit_ref` are likewise caught downstream by the schema `Id`/`minLength` and
  the backend validate/save path, never silently defaulted — consistent with
  the composer's empty-`variable_ref` handling.)

Re-validation after the fix: rule-packs Vitest **43/43**, full Vitest
**285/285**, `tsc -b` clean, Playwright `-g "rule-pack manager"` **2/2**.

## Boundary review

- D-02b gate holds: purely structured form controls. The declarations editor
  authors variable *declarations* (ids, kinds, dimensions, units, statuses) —
  there is no expression text syntax and no text rendering of any AST. The
  typed AST stays the sole edited, checksum-bound expression form (DEC-022).
- No invented engineering or standards values: declarations carry dimension/
  unit *intent* and explicit `TBD`/`not_provided` placeholders; the software
  never fills in a code or standards value (PRD §14.5, IP boundary).
- Private rule packs persist only in the local SQLite store; the GUI never
  transmits or commits them; no protected tables/constants introduced.
- Validation/checksum/completeness remain software findings; the completeness/
  value status fields are user-declared metadata and do not gate pass/fail —
  the engine independently blocks rule checks on missing inputs
  (`RULE_INPUTS_INCOMPLETE`, FR-012/PRD §22.4), which this slice does not touch
  (that end-to-end path is C4). The panel's permanent notice still states no
  professional, certification, sealing, authentication, approval, or
  code-compliance claim. No lifecycle state changed; DEL-07-03 remains CHECKING.

## Residuals and hand-offs (next C2 work)

- **`check_definitions` form builder** — the remaining rule-pack
  document-structure authoring member still edited as raw JSON: a form to bind
  `required_input_refs` + `value_slot_refs` + `formula_ref` into a check, with
  `acceptability_basis`, `result_statuses`, and `diagnostic_policy`. (This is
  the member the earlier "load-combination" residual wording was mis-naming —
  the schema has no `load_combinations`.) Diagnostics/classification/provenance
  metadata remain raw-JSON-editable.
- **C4** engine-side rule evaluation on solved user models (USER_RULE_CHECKED /
  USER_RULE_FAILED / RULE_INPUTS_INCOMPLETE end-to-end) — a separate Phase C
  item, not this slice.
