---
run-id: WORKING_ITEMS_RUN_2026-06-13_TP-C2-COMPOSER-001
timestamp: 2026-06-13T12:30:00-0600
completed: 2026-06-13T13:30:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/apps/desktop/src
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C2-COMPOSER-001 — structured AST expression composer (Phase C2 slice 2)

## Tranche and authority basis

- Tranche: completion-plan Phase C item C2 remaining scope — the "structured
  AST expression composer" and the "in-request busy guard", the next
  unblocked items on the R3/Phase C dependency spine after slice 1
  (`TP-C2-EDITOR-001`) landed the manager + document-level authoring
  (`plans/PLAN_2026-06-10_prd_completion.md` §3 C2; selected per the
  `_COORDINATION.md` Application Integration And Issuance Loop step 3.1).
- Authority: DEL-07-03 R-007/R-008/R-010/R-012; PRD §14.5 ("Expression
  editor", "Variable browser", "Validation diagnostics"); PRD §14.1
  (rule-pack manager surface). DEC-022 (the typed AST is the sole canonical,
  checksum-bound grammar; AST-only authoring). **D-02b AWAITING_RULING** —
  no writable expression text syntax *and* no text rendering of the AST may
  ship (read-only rendering is itself an open D-02b §3 Q5 question), so the
  composer is purely structured. OPS-K-PRIV-1, OPS-K-RULE-2, OPS-K-RULE-3,
  OPS-K-AUTH-1.

## Changes (app-owned slice; all under apps/desktop/src and e2e)

1. **`features/rule-packs/ExpressionComposer.tsx`** (new) — a recursive
   form/tree editor that builds the frozen grammar v1.0.0 typed expression
   AST for a rule pack's selected formula. Structured controls (selects,
   number/text inputs) for the full non-table node set: `literal`,
   `variable_ref`, `unary` (negate/abs/not), `binary`
   (add/subtract/multiply/divide), `compare` (six operators), `logical`
   (and/or), `select`, `aggregate` (min/max, n-ary with add/remove operand,
   last-operand removal blocked with a stated reason). A variable picker /
   read-only variable browser is sourced from the pack's declared
   `required_inputs` and `value_slots` (PRD §14.5 "Variable browser").
   Exported pure helpers (`collectRulePackVariables`,
   `defaultExpressionNode`, `readFormulaDeclarations`,
   `setFormulaExpression`, `parseRulePackDocument`, `nodeKind`,
   `isEditableNode`, `isTableNode`). Node encoding cross-checked against
   `fixtures/rule_expressions/conformance_corpus/README.md` and
   `core/rules/expression_evaluator`.
   - **Lossless preservation:** table-backed nodes (`interpolate`/`lookup`)
     and any unrecognized node tag are rendered read-only with an honest
     explanation and **no node-type selector**, so the composer can never
     silently drop or rewrite a subtree it does not edit. The structured
     table sub-editor is the recorded next slice.
   - **D-02b:** no text input and no text rendering of the AST anywhere —
     purely structured controls.
2. **`features/rule-packs/RulePackManagerPanel.tsx`** — parses the draft
   document and renders the `ExpressionComposer` above the canonical
   document-JSON textarea; composer edits re-serialize straight back into
   `draft.text` (the single source of truth the validate/checksum/save flow
   already reads). Adds the **in-request busy guard** (`inFlight`): every
   async backend handler sets it in a `finally`, and the textarea, composer,
   and all actions disable (with a stated `BUSY_REASON` title) while a
   request is awaiting — closing the slice-1 residual where an async
   response (`compute-checksum` / `open` calling `setDraft`) could clobber a
   mid-request edit. Boundary note updated to "Expressions are composed as a
   structured AST … no expression text syntax … until the D-02b human
   ruling."
3. **Tests** — `ExpressionComposer.test.tsx` (12 Vitest cases: helper
   encodings for every node type, variable collection, lossless rewrite of
   one formula preserving siblings, honest document parsing; component drive
   of node-type switching, literal value/dimension/unit edits, aggregate
   add/remove, **full byte-for-byte round-trip of a preserved table node**,
   and **preservation of an unrecognized node through an editable-sibling
   edit**). `RulePackManagerPanel.test.tsx` +1 integration case (composer
   appears only with a parseable draft; a structured edit rewrites the
   canonical document JSON). `e2e/r2-smoke.spec.ts` rule-pack test extended
   to drive the composer through visible controls and assert the full
   composed `compare` structure, at both viewports.
4. **`styles.css`** — composer node/field/operand layout classes.

## Validation

- Targeted Vitest `src/features/rule-packs` **21/21** (12 new composer +
  9 panel, incl. the strengthened table/unknown preservation cases).
- `tsc -b` clean.
- Playwright `-g "rule-pack manager"` **2/2** (chromium-desktop 1440×920 +
  chromium-compact 1280×800; composer driven from blank through visible
  controls).
- DEC-025 five-surface evidence sweep recorded in the follow-up evidence
  commit (`validation/evidence/sweeps/`). Cargo crate sweep and Python pytest
  (359) pass; the desktop Vitest surface's only failures are the
  `App.test.tsx` timeout flakes described below (pre-existing,
  host-environmental — they also fail on pristine HEAD).
- **Environmental note (recorded, not a regression):** `src/App.test.tsx`
  exhibits machine-load-sensitive per-test **timeout** flakiness on this
  host — the failing test set varies run-to-run (observed 12 → 5 → 3 → 2
  failures, different tests each time) and **reproduces identically on
  pristine HEAD** with this tranche's panel change reverted (3 failures,
  no overlap with the tranche). The flakes are heavy full-`<App />` solve/
  diagnostics tests tipping over their 5 s/10 s timeouts under CPU
  contention, not assertion failures, and are unrelated to this slice
  (which adds only a small no-draft hint to the always-mounted panel in the
  paths `App.test.tsx` exercises). Baseline method: `git stash` the panel,
  re-run `src/App.test.tsx`.

## Pre-commit adversarial review and dispositions

A four-lens review (react-correctness, governance/D-02b boundary,
AST-encoding correctness, evidence honesty) ran over the diff before commit.
Three lenses returned **zero findings** and affirmatively verified: D-02b
compliance (no text syntax/rendering), DEC-022 AST-only authoring, exact
grammar v1.0.0 node/operator/dimension encoding, lossless table/unknown
preservation, the busy guard's `finally` correctness, and dead-control-audit
compliance. The evidence-honesty lens surfaced three legitimate
**test-honesty** weaknesses (no implementation bugs); all fixed in this
tranche:

- **Vacuous table-preservation assertion** (`toMatchObject` would pass even
  if table rows were dropped) → strengthened to `toEqual` on the full
  interpolate node including table rows.
- **No test that an unrecognized node survives a sibling edit** → added a
  component case that flips a root `logical` operator and asserts the
  unknown sibling subtree is unchanged.
- **Shallow e2e assertion** (only checked `.node`) → strengthened to
  `toMatchObject` the full composed `compare` structure.

The fourth lens note (type-switching discards the replaced node's fields) is
**intentional**: switching a node's type is an explicit user replacement, not
a silent drop. Lossless preservation is scoped to read-only table/unknown
subtrees, untouched sibling formulas, and other document members.

## Residuals and hand-offs (next C2 slices)

- **Table-node structured sub-editor** for `interpolate`/`lookup` (rows,
  argument/result dimensions and units, monotonicity) — preserved read-only
  today.
- **Required-input / value-slot / load-combination form builders** (the
  remaining PRD §14.5 sub-surfaces) as structured forms instead of raw JSON.
- **C4** engine-side rule evaluation on solved user models
  (USER_RULE_CHECKED / USER_RULE_FAILED end-to-end) is a separate Phase C
  item, not this slice.

## Boundary review

- No writable expression text syntax and no text rendering of the AST ship —
  the D-02b gate holds; the composer is purely structured controls. The
  typed AST stays the sole edited and checksum-bound form (DEC-022).
- Private rule packs persist only in the local SQLite store; the GUI never
  transmits or commits them; the draft template carries only placeholders
  and invented wording (no protected code equations or standards values).
- Validation/checksum statuses remain software findings; the panel's
  permanent notice still states no professional, certification, sealing,
  authentication, approval, or code-compliance claim. No lifecycle state
  changed; DEL-07-03 remains CHECKING.
