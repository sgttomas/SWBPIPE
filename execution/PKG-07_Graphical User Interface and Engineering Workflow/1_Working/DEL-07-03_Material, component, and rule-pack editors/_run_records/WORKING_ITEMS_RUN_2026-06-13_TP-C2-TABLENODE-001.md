---
run-id: WORKING_ITEMS_RUN_2026-06-13_TP-C2-TABLENODE-001
timestamp: 2026-06-13T14:20:00-0600
completed: 2026-06-13T15:10:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/apps/desktop/src
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C2-TABLENODE-001 — table-node structured sub-editor (Phase C2 slice 3)

## Tranche and authority basis

- Tranche: completion-plan Phase C item C2 remaining scope — the
  "table-node (interpolate/lookup) structured sub-editor", the earliest
  unblocked item on the R3/Phase C dependency spine after slice 2
  (`TP-C2-COMPOSER-001`) landed the structured AST composer with table nodes
  preserved read-only and explicitly routed the table sub-editor as the next
  slice (`plans/PLAN_2026-06-10_prd_completion.md` §3 C2; selected per the
  `_COORDINATION.md` Application Integration And Issuance Loop step 3.1).
- Authority: DEL-07-03 R-007/R-008/R-010/R-012; PRD §14.5 ("Expression
  editor"); DEC-022 (the typed AST is the sole canonical, checksum-bound
  grammar; AST-only authoring; D-02 §3 Q6 user tables). **D-02b
  AWAITING_RULING** — no writable expression text syntax and no text rendering
  of the AST may ship, so the sub-editor is purely structured form controls.
  Table-node JSON shape and semantics cross-checked against
  `core/rules/rule_pack_document` (`decode_table`/`table_value`),
  `core/rules/expression_evaluator` (`UserTable`/`LookupMode`), and the
  `UserTableValue` / interpolate / lookup `$defs` in
  `schemas/rule_pack.schema.yaml`. OPS-K-PRIV-1, OPS-K-RULE-2, OPS-K-RULE-3,
  OPS-K-AUTH-1.

## Changes (app-owned slice; all under apps/desktop/src and e2e)

1. **`features/rule-packs/ExpressionComposer.tsx`** — `interpolate` and
   `lookup` nodes, preserved read-only by slice 2, are now first-class
   authorable node types:
   - Both added to `EDITABLE_NODE_TYPES`, so they are reachable from every
     node-type selector and `defaultExpressionNode` seeds a **schema-valid
     default table** (`defaultUserTable`: two strictly-increasing rows,
     uppercase `"TBD"` dimension/unit placeholders, `table_id: "user_table_1"`)
     plus the recursive `argument` child; `lookup` additionally seeds
     `mode: "exact"`.
   - A structured table sub-editor renders `table_id` (text), argument/result
     `dimension` (shared `DimensionSelect`) and `unit_ref` (text), the
     `{argument, result}` rows (number inputs; **Add row** appends
     last-argument + 1 to keep strict monotonicity; **Remove row** is blocked
     at the single-row schema floor with a stated reason), the `lookup` `mode`
     selector (exact/step), and the recursive `ExpressionNodeEditor` for the
     table's `argument` expression. A guidance line states the
     evaluator-enforced rules (strictly-increasing arguments, interpolate
     needs ≥2 rows, out-of-range is blocking — never clamped/extrapolated).
   - **Lossless preservation:** row edits patch only the touched row object
     (`updateRow`), and the table object is structurally copied, so untouched
     rows and the sibling subtrees round-trip verbatim. The read-only branch
     now covers only the refusal markers (`unsupported_form`/
     `unsafe_host_access`) and unrecognized tags.
   - Factored a shared **`DimensionSelect`** used by the literal editor and
     both table dimension fields (removes a duplicated select; keeps the
     placeholder consistent and codec/schema-valid everywhere). Added the
     `LOOKUP_MODES` constant and `tableOf`/`rawTableRows`/`rowNumber` helpers.
2. **Regression repair (folded in):** slice 2 defaulted a literal's dimension
   to the **lowercase `"tbd"`** token and listed it in the shared `DIMENSIONS`
   vocabulary. That token is **not** in the document codec's vocabulary
   (`rule_pack_document::DIMENSION_TOKENS` → `decode_quantity` errors "unknown
   dimension token 'tbd'") nor the schema `DimensionId` enum — both use
   uppercase `"TBD"` (the same token the draft builder, `output_dimension`,
   and `value_slots` already use, and that the evaluator encodes via
   `Dimension::Tbd => "TBD"`). A composer-authored default literal therefore
   produced a document that fails backend validation/save. The slice-2 Vitest
   missed it because it asserted the produced *string*, never a Rust
   round-trip. Fixed `DIMENSIONS`, `defaultLiteral`, and the literal-dimension
   fallback to `"TBD"`; added a unit guard asserting the composer offers
   `"TBD"` and never `"tbd"`.
3. **`features/rule-packs/RulePackManagerPanel.tsx`** — header comment and the
   document-JSON textarea label updated: the composer now edits the formula's
   *full* expression AST including table-backed nodes; the raw JSON remains the
   canonical/fallback surface for the document-structure members whose form
   builders have not landed (required inputs, value slots, load combinations).
   No behavioral change to the panel.
4. **`styles.css`** — `.rule-pack-table-row` flex layout class.
5. **Tests** — `src/features/rule-packs` extended to **26** cases (was 21;
   +5 net): default encodings for `interpolate`/`lookup`; the
   uppercase-`"TBD"` vocabulary guard; an exact-shape (`toEqual`) default-
   literal assertion (pins the schema's value/dimension/unit_ref-only
   quantity); reclassified table nodes as editable; component drives for
   switching the root to an `interpolate` table (default seeded, no mode
   field), editing a `lookup` table's id/dimensions/units/mode/row, add/remove
   rows with the single-row block, editing the table's `argument` child while
   the table round-trips verbatim, and a refusal-marker preservation test (no
   node-type selector + byte-for-byte round-trip). `e2e/r2-smoke.spec.ts`
   "rule-pack manager" test extended to switch the root to `interpolate` and
   edit a row, asserting the canonical document JSON.

## Validation

- Targeted Vitest `src/features/rule-packs` **26/26** (was 21; +5 net).
- `tsc -b` clean.
- Full desktop Vitest (`npx vitest run`, 267 total): the changed rule-pack
  surface is green; the only failures observed were in `src/App.test.tsx` and
  were **load-induced timeouts, confirmed tranche-independent.** This host was
  heavily loaded during the session (concurrent test jobs); the failure set
  fell monotonically as load cleared — the documented `App.test.tsx` flake
  (slice-2 record `WORKING_ITEMS_RUN_2026-06-13_TP-C2-COMPOSER-001.md`;
  run-to-run failure-set variance 12→9→7→5→3→2→1). Evidence chain:
  - First full-suite run under peak load (357 s): 233 passed, **34 failed —
    every failure a per-test timeout in `App.test.tsx`** (the only failing
    file), zero in the rule-pack surface.
  - `App.test.tsx` in isolation under load: 20/52 failed (default timeouts) →
    **4/52** with `--testTimeout=60000 --hookTimeout=60000` (the 4 carry inline
    `it(…, 10000/15000)` timeouts the global flag cannot raise, plus one
    solve-window `findBy` miss — all in the `<App />` solve/diagnostics/report
    flow).
  - Once contention cleared, `App.test.tsx` (60 s flags) passed **52/52 on my
    working tree** and **52/52 on pristine HEAD** (my three runtime source
    files — `ExpressionComposer.tsx`, `RulePackManagerPanel.tsx`, `styles.css`
    — stashed out), identical flags, minutes apart. **My tree == pristine HEAD
    == 52/52 ⇒ no causation.**
  - Code-level confirmation: **no failing test exercises the changed code.**
    `ExpressionComposer` renders only when a draft exists; no `App.test.tsx`
    test creates a rule-pack draft, and the `RulePackManagerPanel` change is
    comment/label text only. The one rule-pack reference in `App.test.tsx`
    (the `editor-contract-rule-pack` status string) lives in a passing fixture
    test, not the changed surface.
  - The changed surface itself is fully green (rule-packs Vitest 26/26,
    Playwright 2/2, `tsc -b` clean). The gate-determinism follow-up (raise the
    inline `App.test.tsx` per-test timeouts / isolate it / reorder the sweep)
    remains the separately-recorded infra item, not this tranche.
- Playwright `-g "rule-pack manager"` **2/2** (chromium-desktop 1440×920 +
  chromium-compact 1280×800; the table sub-editor driven from blank through
  visible controls at both viewports).
- DEC-025 five-surface sweep recorded at the committed HEAD (summary committed
  alongside this tranche; see the sweep artifact referenced in the commit).
- Evidence posture: this is a homogeneous extension of an existing component
  with a Vitest unit suite and a Playwright journey (the slices-1/2 pattern);
  both were extended (Vitest + the rule-pack e2e), so the H4 default UI
  evidence posture is met without a manual-only exception.

## Pre-commit adversarial review and dispositions

A five-lens review (AST-encoding/codec-schema correctness, React
correctness/lossless editing, governance/D-02b boundary, evidence honesty,
regression completeness/dead-control audit) ran over the diff and the three
authoritative contracts (`rule_pack_document`, `expression_evaluator`,
`rule_pack.schema.yaml`) before commit. Four lenses returned zero findings and
affirmatively verified: exact interpolate/lookup/`UserTable` encoding against
all three contracts, the order-identical 29-token dimension vocabulary match,
lossless row/sibling/argument round-trip, the busy/disabled threading,
D-02b/DEC-022 (no text syntax or text rendering of the AST), no protected
content, and evidence-claim honesty (counts, the App.test.tsx
load-flake characterization). Dispositions:

- **Blocker fixed — schema-invalid default literal.** The slice-2
  `defaultLiteral()` emitted `unit_required: true` and
  `dimension_check_required: true`, but the schema's `ExpressionQuantity` is
  `additionalProperties:false` and those relaxation flags are deliberately not
  authorable in pack documents (absent ⇒ true; the schema description and the
  contract test `tests/test_rule_pack_schema.py` pin this). The Rust codec
  tolerated the keys (`optional_bool`), so the live validate/save path never
  surfaced it, but the default literal did not conform to the authoritative
  YAML schema — contradicting this tranche's "schema-valid" claim. Removed the
  two flags from `defaultLiteral()`; tightened the default-literal Vitest from
  `toMatchObject` to `toEqual` to pin the exact `{value, dimension, unit_ref}`
  shape and prevent recurrence. (An opened document's literal still preserves
  any flags it carries — only the authoring default omits them.)
- **Nit fixed — stale inner comment.** The read-only branch's comment still
  described table-backed nodes as the "next slice" / read-only; reworded to
  name only the refusal markers and unrecognized tags (tables are now
  editable). Flagged by two lenses.
- **Nit fixed — restored preservation coverage.** Replacing the old table
  read-only test dropped the only component assertion that a preserved node
  exposes *no* node-type selector. Added a refusal-marker
  (`unsafe_host_access`) component test asserting the read-only explanation, an
  empty `rule-pack-node-type` query, and a byte-for-byte (`toEqual`) round-trip.
- **Nits accepted (not blocking, recorded).** Number inputs coerce a cleared
  field to `0` (`Number("") === 0`), and `table_id`/`unit_ref` accept the empty
  string (caught downstream by schema `Id`/`minLength` and the evaluator's
  `TableMalformed`, never silently clamped). Both are pre-existing, consistent
  with the literal editor, and never corrupt the document; the lenses rated
  them acceptable. Left unchanged for editor consistency.

Re-validation after the fixes: rule-packs Vitest **26/26**, `tsc -b` clean,
Playwright `-g "rule-pack manager"` **2/2** (both viewports).

## Boundary review

- D-02b gate holds: purely structured form controls — no writable expression
  text syntax and no text rendering of the AST. The typed AST stays the sole
  edited and checksum-bound form (DEC-022).
- Table values are authored as invented placeholders only; private rule packs
  persist only in the local SQLite store; the GUI never transmits or commits
  them; no protected standards tables or constants are introduced.
- Validation/checksum statuses remain software findings; the panel's permanent
  notice still states no professional, certification, sealing, authentication,
  approval, or code-compliance claim. No lifecycle state changed; DEL-07-03
  remains CHECKING.

## Residuals and hand-offs (next C2 work)

- **Required-input / value-slot / load-combination form builders** — the last
  C2 authoring sub-surface still edited as raw JSON; the remaining structured
  PRD §14.5 sub-surfaces.
- **C4** engine-side rule evaluation on solved user models (USER_RULE_CHECKED /
  USER_RULE_FAILED end-to-end) — a separate Phase C item, not this slice.
