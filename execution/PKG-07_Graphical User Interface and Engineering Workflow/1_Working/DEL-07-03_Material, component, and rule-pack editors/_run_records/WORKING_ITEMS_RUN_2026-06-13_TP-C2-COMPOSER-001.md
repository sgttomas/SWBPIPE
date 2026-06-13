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
- **All five DEC-025 surfaces independently green on the maintainer's clean
  machine** at this committed HEAD (`add584756`): cargo crate sweep ✓; Python
  pytest **359** ✓; desktop Vitest **263/263** (standalone `vitest run`) ✓;
  Playwright e2e **6/6** (clean `--workers=1` run, both viewports, incl. the
  rule-pack composer journey) ✓; `tsc -b` clean. Targeted maintainer runs
  match the author runs (rule-packs Vitest 21/21; Playwright `-g "rule-pack
  manager"` 2/2).
- **DEC-025 atomic sweep — `overall: fail` on a gate-determinism flake, not a
  regression.** The committed sweep artifact
  (`validation/evidence/sweeps/SWEEP_20260613T200943Z_add584756bb1-dirty.json`)
  records cargo ✓, pytest ✓, then `desktop_vitest` **262/263** with a single
  failure: `App.test.tsx > round trips review-only proposal operations …` —
  a **10 s per-test timeout**, not an assertion, on a test this tranche does
  not modify (later surfaces `not_run` because the sweep stops at first
  failure). **Root cause:** the sweep runs cargo + pytest *before* Vitest in
  one process, so the machine is warm when the tight-timeout (5 s/10 s)
  full-`<App />` solve tests in `App.test.tsx` run, and 1–9 of them tip over
  their timeout. The same Vitest suite is **263/263 when run standalone**
  (no preload). On the author's busier host the in-sweep flake set varied
  run-to-run (12 → 9 → 7 → 5 → 3 → 2 → 1 failures, different tests each time)
  and **reproduced on pristine HEAD** with this tranche's panel change
  reverted — confirming it is load-induced and tranche-independent.
- **Follow-up (gate infra, not this tranche):** the "deterministic local
  merge gate" is not deterministic under its own sequential load because of
  `App.test.tsx`'s 5 s/10 s per-test timeouts. Candidate fixes: raise those
  test timeouts, isolate `App.test.tsx` into its own Vitest pass, or run the
  Vitest surface before cargo/pytest. Recorded for a separate change.

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
