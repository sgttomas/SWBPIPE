# Unit check at rule-pack validation (W-17)

**Basis:** `origin/main` at `356a9be374` (2026-10-23). Design before
implementation. Standard claim fence applies (F-PIP-2; claims taxonomy per
DEC-081).

Requirement: FR-RULE-002 (Must), "Expressions reject incompatible units". The
map row says "Units are checked at run time, not when the pack is validated".
Also REQ-06-02-003 and -004. The grammar (DEC-022 v1.0.0) is unchanged.

## 1. Where things stand

- **Validation does not type-check.** `validate_rule_pack_document`
  (`core/rules/rule_pack_document/src/lib.rs:922`) decodes each declarative
  AST and rejects refusal markers (`:993-1060`). It never checks dimensions or
  units.
  - `validate_rule_pack` (`apps/desktop/src-tauri/src/lib.rs:2736-2762`) and the
    store's save and open (`:3474-3500,3513`) return this validation.
  - Saving never blocks: "drafts with findings remain saveable".
- **Run time is where units are checked.** `run_one_check`
  (`core/rules/rule_check_runner/src/lib.rs:408`) normalizes each bound value to
  its declaration's `quantity_intent.unit_ref`
  (`normalize_value_to_declared_unit`, `:1197-1263`). A failure is `UnitMismatch`
  (`:1286`).
  - **The evaluator does not convert.** It requires equal dimensions and
    identical `unit_ref` strings for add, subtract, compare, min, max and select
    (`expression_evaluator/src/lib.rs:1095-1117,1453,1589`). Products and
    quotients follow the enumerated table (`dimension_product`,
    `dimension_quotient`, `:1222-1270`), with composed unit strings
    (`:1272-1291`).
  - **The check's verdict** compares the formula's value with the value-slot
    limit through the same rule (`rule_check_runner/src/lib.rs:1098`).
- **Every bound variable therefore has a statically known type.** After
  normalization its unit is the declared `unit_ref` and its dimension the
  declared dimension (`:660-678`). Literals and tables carry their own. Unit
  findings that depend only on types can be computed without values.
- **The run blocks the whole pack on any blocking validation finding**
  (`src-tauri/src/lib.rs:2913-2915`). `has_blocking_findings` covers document,
  lifecycle and formula findings (`rule_pack_document/src/lib.rs:1178-1182`).
- **`output_dimension`** on formula declarations is read by no code.
- **The invented demo pack** declares `unit_ref: "demo_unit"`, which is not a
  catalog symbol (`fixtures/product_preview/invented_demo_rule_pack.json:28,56`).
  It runs because its supplied bindings carry the identical string.

## 2. Decisions

1. **A static type check in the evaluator crate.** `check_expression_types(expression,
   &TypeEnv) -> TypeCheck { value_type, findings }`, where `TypeEnv` maps
   variable ids to `(Dimension, unit_ref)`.
   - It applies the evaluator's own rules: the same helpers for dimension
     products and quotients, composed units and unit equality, and the same
     propagation, where a failed node yields no type and its parents add no
     finding.
   - It produces the evaluator's `FindingCode`s and subject ids.
   - It reads no values and changes no grammar, so no version bump or corpus
     extension is needed.
2. **What is known statically.** S is every finding the evaluator derives from
   the expression and the variables' types alone:
   - `type_mismatch`, `dimension_mismatch`, `unit_mismatch`;
   - `unsupported_expression_form`: an empty min or max, an unrepresentable
     product, an ambiguous or unrepresentable quotient, a refusal marker;
   - `unsafe_construct`;
   - `invalid_reference` (an empty variable id);
   - `missing_variable`;
   - `unit_metadata_missing`;
   - `table_malformed`, since tables are wholly pack literals.

   In pack validation, the type environment comes from the declarations. An
   authored refusal marker already blocks through `REFUSAL_MARKER_AUTHORED`,
   so the check reads `not_checkable` and the finding is not repeated.
   Validation adds these check-level findings:
     - `RULE_PACK_UNDECLARED_VARIABLE`: an AST variable missing from the
       formula's `input_refs`, or a reference to no declared required input;
     - `RULE_PACK_LIMIT_UNIT_MISMATCH`: the formula's type differs from the
       value slot's declared `(dimension, unit_ref)`, so the verdict comparison
       fails on every run;
     - `RULE_PACK_UNIT_DECLARATION_INCOMPLETE`: an input or slot declares `TBD`
       or an empty dimension or unit, so the check cannot be typed.
3. **What is known only at run time:**
   - the unit of each solver result, supplied value or library value, and
     whether it converts to the declared unit (`:1197-1263`);
   - missing or duplicate bindings;
   - value-dependent findings: out of range, key not found, division by zero,
     non-finite values and overflow;
   - interval indeterminacy;
   - which solver field the caller binds.
4. **Declared units outside the catalog** get a warning,
   `RULE_PACK_UNIT_NOT_IN_CATALOG`. The check is `unit_by_symbol` for the
   declared dimension, in `rule_pack_document` (which gains the
   `open_pipe_stress_units` dependency, as `rule_check_runner` has). It is not
   an error: a binding with the identical string still binds, which is how the
   demo pack runs. The message says a value in any other unit will be refused
   at run time.
   - `output_dimension` that differs from the inferred type also gets a warning
     (`RULE_PACK_OUTPUT_DIMENSION_DIFFERS`).
5. **Where the results go.** `RulePackDocumentValidation` gains `unit_check`:
   `{status: consistent | findings | not_checkable, checks: [{check_id,
   formula_id, status, findings}], declaration_findings}`.
   - The findings are blocking for their check and do not enter
     `has_blocking_findings`. The run therefore still evaluates the other checks,
     and an inconsistent check still blocks at run time with the same code, so
     run outcomes, the corpus and the runner tests are unchanged.
   - Making them pack-level blocking would refuse whole packs over one check.
     That change in run behaviour is not made here.
6. **Diagnostics never echo private content.** Each finding carries a code, a
   subject (an id), a JSON path into the pack, and dimension and unit tokens. It
   never carries a magnitude, table row, payload summary or rendered expression.
   - The findings appear only in validation responses to the local Rule Pack
     Manager.
   - They never enter rule-check outcomes, reports (R-08-01-008: check id,
     status and missing inputs only), issue bundles, logs or telemetry
     (DEL-12-02 CLM-029).

## 3. Desktop UX

- **`RulePackManagerPanel.tsx`** (`:492-515`) shows a "Unit consistency" group
  after the document findings. It has one row per check with its status, then
  the findings with their paths. A note reads: "Units of solver results and
  entered values are checked when the check runs."
- **The summary line** gains `unit_check=<status>`. Its `blocking_findings`
  keeps its meaning.
- **`rulePackService.ts`** types the new member. Validate, save and open all
  return it.

## 4. Tests

- **Corpus parity** (`expression_evaluator/tests/conformance_corpus.rs`, new
  test). For every corpus case, the static check over the case's binding types
  yields exactly the expected findings that fall in S, with the same codes and
  subject ids.
  - Cases 33-43, 49-57, 59-61, 64, 65 and 67-69 are flagged.
  - These produce no static finding: cases 01-32 (cases 30-32 test the grammar
    version, which validation checks elsewhere); the value-dependent cases 44-48
    and 58; and the binding and status cases 62, 63 and 66.
  - This keeps the static and dynamic rules equal; a rule change that breaks
    parity fails here.
- **Validation tests** (`rule_pack_document`):
  - an undeclared variable;
  - a limit mismatch;
  - an incomplete declaration;
  - a non-catalog unit as a warning, with the demo pack showing only that
    warning;
  - an `output_dimension` difference.
- **Unchanged outputs.** `corpus_parity.rs` and `invented_demo_document.rs` are
  extended only by the additive member. `has_blocking_findings` is unchanged for
  every existing fixture.
- **Privacy test.** A pack with a sentinel magnitude, table row and payload
  summary: the serialized `unit_check` contains none of them.
- **Run unchanged.** A pack with one inconsistent check validates with
  `unit_check.status = findings`. `run_rule_checks_core` still runs the other
  checks, and the inconsistent check blocks with the same code statically and at
  run time.
- **Vitest.** The panel renders the group and its statuses.

## 5. Files, overlap with W-13 and sequencing

| File | This item | W-13 S1b / S2 | Collision |
|---|---|---|---|
| `core/rules/expression_evaluator/src/lib.rs`, `tests/conformance_corpus.rs` | type check, parity test | none | no |
| `core/rules/rule_pack_document/src/lib.rs`, `Cargo.toml`, tests | `unit_check`; units dependency | none. W-13 §3.4 cites the checksum record (`:742-790`), read only | no |
| `Cargo.lock` | one dependency edge | S1b may touch it | low |
| `apps/desktop/src/services/rulePackService.ts` | type | none | no |
| `apps/desktop/src/features/rule-packs/RulePackManagerPanel.tsx` | findings group | S2 adds "Use with this model" and "Remove from model" | same file, different region; no |
| `apps/desktop/src-tauri/src/lib.rs` | none (the validation passes through) | S1b | no |
| D3-pinned files | none; `core/units/src` is only depended on, not edited | S1b's D3 chain | no |

**Sequencing:** this item touches no S1b or S2 file except the panel, and no
model-document member. It can be implemented before S1b, or in parallel.
