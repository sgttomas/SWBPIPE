---
run-id: WORKING_ITEMS_RUN_2026-06-13_TP-C2-CHECKDEF-001
timestamp: 2026-06-13T17:25:00-0600
completed: 2026-06-13T17:45:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/apps/desktop
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C2-CHECKDEF-001 — check-definitions form builder (Phase C2 slice 5)

## Tranche and authority basis

- Tranche: completion-plan Phase C item C2 remaining scope — the
  `check_definitions` form builder, the next unblocked item on the R3/Phase C
  dependency spine after slice 4 (`TP-C2-DECLEDITOR-001`) landed the
  required-input/value-slot declaration form builders and routed
  `check_definitions` as the last C2 document-structure authoring member
  (`plans/PLAN_2026-06-10_prd_completion.md` §3 C2; selected per the
  `_COORDINATION.md` Application Integration And Issuance Loop step 3.1).
- Authority: DEL-07-03 (Material, component, and rule-pack editors); PRD §14.5
  (rule-pack editor authoring surfaces); DEC-022 (typed AST is the sole
  canonical, checksum-bound grammar; AST/reference binding only). **D-02b
  AWAITING_RULING** — no writable expression text syntax may ship; this slice
  binds *references* and selects closed-vocabulary tokens, never expression
  text, so it does not touch the D-02b gate. Document shapes cross-checked
  against the `CheckDefinition`, `DiagnosticPolicy`, `Reference`,
  `AnalysisStatus`, and `RulePackDiagnosticCode` `$defs` in
  `schemas/rule_pack.schema.yaml` and the already-backend-validated template
  check in `buildDraftRulePackDocument` (`apps/desktop/src/services/rulePackService.ts`).
  OPS-K-PRIV-1 (private rule packs stay local-only), CONTRACT no-silent-defaults,
  OPS-K-AUTH-1.

## Changes (app-owned slice; all under apps/desktop/src and e2e)

1. **`features/rule-packs/CheckDefinitionsEditor.tsx` (new)** — a structured
   editor for the rule pack's `check_definitions`, the member that binds the
   declared inputs/slots and a formula into an acceptability check. Add / remove
   / edit each check through form controls:
   - **Identity:** `check_id`, `name`, `description` (text).
   - **Reference binding:** `formula_ref` is a picker over the document's
     declared `formula_declarations` ids; `required_input_refs` and
     `value_slot_refs` are add/remove lists of pickers over the declared
     `required_inputs` / `value_slots` ids (the same ids the declarations editor
     authors and the composer binds). Each ref array is held at its `minItems:1`
     schema floor. A ref pointing at an id that is not (or no longer) declared
     — including the `"TBD"` placeholder when nothing is declared yet — surfaces
     as an `(unresolved) <id>` option rather than being silently snapped to the
     first declared id; the backend validation independently flags dangling
     references.
   - **`acceptability_basis`** (closed-set `<select>`, reusing the declarations
     editor's `EnumSelect`).
   - **`result_statuses`** — a checkbox multi-select over the six
     `AnalysisStatus` tokens, held at the `minItems:1` floor (the last checked
     status cannot be unchecked). An out-of-vocabulary stored status is kept
     visible and removable as a `(current) <token>` checkbox, never dropped.
   - **`diagnostic_policy`** — the eight condition→code bindings as `EnumSelect`s
     over the `RulePackDiagnosticCode` set.
   - **Add check** seeds a schema-valid default (`CheckDefinition` shape verified
     by `toEqual`) with a fresh unique id, single-ref arrays bound to the first
     declared input/slot/formula, the user-rule result statuses, the default
     diagnostic policy, and `draftPlaceholderProvenance()`. **Remove check** is
     blocked at the `minItems:1` floor with a stated reason.
   - **Enum vocabularies** (`ACCEPTABILITY_BASES`, `ANALYSIS_STATUSES`,
     `DIAGNOSTIC_CODES`, `DIAGNOSTIC_POLICY_FIELDS`) are exported consts copied
     verbatim from the schema so the controls can only emit schema-valid tokens.
   - **Lossless:** a patched check keeps every member it carried (provenance,
     description, any `Reference.version`) via structural spread; an edited ref
     preserves its sibling ref members; a diagnostic-policy edit preserves the
     other seven bindings; untouched sibling checks round-trip verbatim.
   - `provenance` is seeded private-by-default and preserved losslessly but not
     exposed as a form field — consistent with the declarations editor and the
     plan's "diagnostics/classification/provenance metadata remain
     raw-JSON-editable" residual.
2. **`features/rule-packs/DeclarationsEditor.tsx`** — exported the shared field
   primitives `Field`, `TextField`, and `EnumSelect` for reuse by the new editor
   (the codebase already shares `DimensionSelect` this way). No behavior change.
3. **`features/rule-packs/RulePackManagerPanel.tsx`** — mounts
   `CheckDefinitionsEditor` after the `ExpressionComposer` inside the
   valid-parsed-draft branch (all three editors read the same draft document; a
   new `handleCheckDefinitionsChange` re-serializes the edit). Updated the header
   comment and the document-JSON textarea label: `check_definitions` is no longer
   a raw-JSON member — the remaining raw-JSON surface is the advanced metadata
   (diagnostics, classification, checksums, provenance, professional_boundary,
   open_decisions).
4. **`src/styles.css`** — added `.rule-pack-check-statuses` (status checkbox
   grid) and `.rule-pack-check-subgroup` (a small indented left border for the
   ref/policy sub-groups, aiding the nested-form readability the A3 usability
   lane cares about).
5. **Tests** — `CheckDefinitionsEditor.test.tsx` (new): pure-helper tests
   (read ids; exact schema-valid `defaultCheckDefinition`/`defaultDiagnosticPolicy`
   shapes via `toEqual`; `"TBD"` ref fallback; lossless array rewrite; enum
   set-equality) and component tests (add with fresh id bound to declared ids;
   edit identity preserving the rest; acceptability-basis edit; formula-ref
   rebind; input-ref rebind/add/remove with floor; diagnostic-policy edit
   preserving the other seven; result-status toggle on/off; floor on the last
   status incl. an out-of-vocabulary one; remove blocked at the floor;
   add-then-remove lossless; sibling preserved; `(current)` and `(unresolved)`
   escape hatches; disabled threading). `RulePackManagerPanel.test.tsx` extended
   with two integration tests: the check editor mounts and an added check grows
   the canonical document; a required input declared in the declarations editor
   is bindable in a check's reference picker. `e2e/r2-smoke.spec.ts` "rule-pack
   manager" test extended (slice 5): the check editor is visible, the template
   check's formula ref is bound, adding a check grows the document JSON, and
   toggling a result status rewrites it.

## Validation

- Targeted Vitest `src/features/rule-packs` **67/67** (was 43; +24 across the
  new `CheckDefinitionsEditor` suite and the two panel integration tests).
- `tsc -b` clean.
- Full desktop Vitest **308/308** (14 files; was 285 + 23 net). No
  `App.test.tsx` load-flake.
- Playwright `-g "rule-pack manager"` **2/2** (chromium-desktop 1440×920 +
  chromium-compact 1280×800; the check-definitions editor driven from blank
  through visible controls at both viewports).
- DEC-025 five-surface evidence sweep run at the committed HEAD; summary
  committed alongside this tranche. The sweep's recorded `dirty_paths` are
  pre-existing external-scope noise (`init/` app-dev coordination files and
  `plans/monorepo_structure_assessment_2026-06-13.md`), not products of this
  tranche — recorded and bypassed per the entry prompt's external-scope rule.
- Evidence posture (H4): a homogeneous extension of the rule-pack editor with a
  Vitest unit suite and the rule-pack Playwright journey; both were extended, so
  the default UI evidence posture is met without a manual-only exception.

## Pre-commit adversarial review and dispositions

An independent read-only reviewer ran four lenses (schema conformance,
governance/D-02b/IP boundary, React correctness, test honesty) over the diff and
the authoritative schema, and independently re-ran a JSON-Schema (Draft 2020-12)
validator against the two default shapes, the enum set-equality checks, `tsc`,
and the Vitest suite. Verdict: **no BLOCKER, no SHOULD-FIX.** Affirmative
verifications: `defaultCheckDefinition`/`defaultDiagnosticPolicy` validate
against `CheckDefinition`/`DiagnosticPolicy` (`additionalProperties:false`); all
four exported enum consts equal their schema sets by order and membership; no
writable expression-text path and no invented numeric/standards value;
private-by-default provenance preserved; all four `minItems:1` floors enforced
(buttons + handler guards, defense in depth); lossless patching proven
non-vacuous (dropping the spread reddens the `toEqual` lossless tests); no
stale-closure lost-update; controlled-select escape hatches prevent value-snap.
Dispositions:

- **NIT accepted (not blocking) — RefSelect/EnumSelect lack a disabled-state
  tooltip.** The reference and enum `<select>`s show no busy-reason `title` when
  disabled mid-request. This matches the established convention (the add/remove
  *buttons* carry disabled-reason tooltips; the shared `EnumSelect`/
  `DimensionSelect` selects do not), so changing it would diverge from
  `DeclarationsEditor` and touch a shared primitive outside this slice. The
  controls are correctly disabled; only the tooltip is absent.
- **NIT fixed — result-status floor coverage gap.** Added a Vitest case pinning
  that the `minItems:1` result-status floor blocks unchecking the last status
  even when it is an out-of-vocabulary `(current)` token (the floor keys on the
  selected count, not the vocabulary; the logic already handled it, this makes it
  non-regressable). Re-validation after the fix: rule-packs Vitest 67/67,
  `tsc -b` clean.

## Boundary review

- D-02b gate holds: a check binds *references* (ids) and selects
  closed-vocabulary tokens (acceptability basis, result statuses, diagnostic
  codes) through pickers and checkboxes. There is no expression text syntax and
  no text rendering of any AST anywhere in this editor; the typed AST stays the
  sole edited, checksum-bound expression form (DEC-022).
- No invented engineering or standards values: refs default to declared ids or
  the explicit `"TBD"` placeholder; result/diagnostic vocabularies are the
  software's own status/diagnostic enums; provenance is the private placeholder.
  The software authors the check *shape* only — it never decides pass/fail
  acceptance (the engine independently blocks incomplete checks with
  `RULE_INPUTS_INCOMPLETE`, FR-012/PRD §22.4 — that end-to-end path is C4, not
  this slice) and never fills in a code or standards value (PRD §14.5, IP
  boundary).
- Private rule packs persist only in the local SQLite store; the GUI never
  transmits or commits them; no protected tables/constants introduced. The
  panel's permanent notice still states no professional, certification, sealing,
  authentication, approval, or code-compliance claim. No lifecycle state
  changed; DEL-07-03 remains CHECKING.

## Residuals and hand-offs (next Phase C work)

- **C2 form-builder series complete.** Every rule-pack document-structure
  authoring member (`required_inputs`, `value_slots`, the formula expression
  AST incl. table-backed nodes, and `check_definitions`) now has a structured
  form builder. The remaining raw-JSON-editable members are the advanced
  metadata (diagnostics, classification, checksums, provenance,
  professional_boundary, open_decisions) — intentionally raw-JSON per the plan,
  not a form-builder gap.
- **C3** private library management GUI (PRD §13, §14.6) — the next
  dependency-spine item.
- **C4** end-to-end rule checks on authored models (`USER_RULE_CHECKED` /
  `USER_RULE_FAILED` / `RULE_INPUTS_INCOMPLETE` driven from solves of
  user-authored models) — the engine-side path this editor's checks feed.
- D-02b remains AWAITING_RULING (writable rule-expression text syntax); no
  writable text input ships until it is ruled.
